fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day1.txt")?;
    let input: Vec<u32> = input.split('\n').flat_map(str::parse).collect();
    'outer: for n in &input {
        for m in &input {
            if n != m && n + m == 2020 {
                println!("{}", n * m);
                break 'outer;
            }
        }
    }
    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day1.txt")?;
    let input: Vec<u32> = input.split('\n').flat_map(str::parse).collect();
    'outer: for n in &input {
        for m in &input {
            for p in &input {
                if n != m && m != p && n + m + p == 2020 {
                    println!("{}", n * m * p);
                    break 'outer;
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
