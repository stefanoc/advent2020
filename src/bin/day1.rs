fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day1.txt")?;
    let mut input: Vec<u32> = input.split('\n').flat_map(str::parse).collect();
    input.sort_unstable();
    'outer: for m in &input {
        for n in &input {
            if m != n {
                match n + m {
                    2020 => {
                        println!("{}", n * m);
                        break 'outer;
                    }
                    x if x > 2020 => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day1.txt")?;
    let mut input: Vec<u32> = input.split('\n').flat_map(str::parse).collect();
    input.sort_unstable();
    'outer: for n in &input {
        for m in &input {
            if n != m && n + m > 2020 {
                break;
            }
            for p in &input {
                if n != m && m != p {
                    match n + m + p {
                        2020 => {
                            println!("{}", n * m * p);
                            break 'outer;
                        }
                        x if x > 2020 => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;
    part2()?;
    Ok(())
}
