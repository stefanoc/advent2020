fn is_valid(passport: &&str) -> bool {
    let fields = passport
        .split(' ')
        .flat_map(|f| f.split(':').next())
        .collect::<Vec<_>>();
    fields.len() == 8 || (fields.len() == 7 && !fields.contains(&"cid"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::fs::read_to_string("data/day4.txt")?;
    input = input.replace("\n\n", "~~");
    input = input.replace('\n', " ");
    let passports = input.split("~~");
    println!("{}", passports.into_iter().filter(is_valid).count());
    Ok(())
}
