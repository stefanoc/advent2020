use regex::Regex;

type Passport<'a> = std::collections::HashMap<&'a str, &'a str>;

fn parse<'a, I>(fields: I) -> Passport<'a>
where
    I: Iterator<Item = &'a str>,
{
    let mut passport = Passport::new();
    for f in fields {
        let mut kv = f.split(':');
        passport.insert(kv.next().unwrap(), kv.next().unwrap());
    }
    passport
}

fn is_valid<'a>(passport: &Passport<'a>) -> bool {
    passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
}

fn is_valid_2<'a>(passport: &Passport<'a>) -> bool {
    passport.iter().all(validate)
}

fn validate(field: (&&str, &&str)) -> bool {
    match *field.0 {
        "byr" => {
            if let Ok(year) = field.1.parse::<i32>() {
                year >= 1920 && year <= 2002
            } else {
                false
            }
        }
        "iyr" => {
            if let Ok(year) = field.1.parse::<i32>() {
                year >= 2010 && year <= 2020
            } else {
                false
            }
        }
        "eyr" => {
            if let Ok(year) = field.1.parse::<i32>() {
                year >= 2020 && year <= 2030
            } else {
                false
            }
        }
        "hgt" => {
            let re = Regex::new(r"^(\d+)(in|cm)$").unwrap();
            if let Some(height) = re.captures(field.1) {
                let h = height[1].parse::<i32>().unwrap();
                if &height[2] == "cm" {
                    h >= 150 && h <= 193
                } else {
                    h >= 59 && h <= 76
                }
            } else {
                false
            }
        }
        "hcl" => {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(field.1)
        }
        "ecl" => {
            let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            re.is_match(field.1)
        }
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(field.1)
        }
        _ => true,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::fs::read_to_string("data/day4.txt")?;
    input = input.replace("\n\n", "~~");
    input = input.replace('\n', " ");
    let passports = input.split("~~").map(|p| p.split(' ')).map(parse);
    println!(
        "{}",
        passports.filter(|p| is_valid(p) && is_valid_2(p)).count()
    );
    Ok(())
}
