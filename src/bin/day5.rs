// FFBBBFFRRL

fn lower_half(r: (u8, u8)) -> (u8, u8) {
    (r.0, r.0 + (r.1 - r.0 + 1) / 2)
}

fn upper_half(r: (u8, u8)) -> (u8, u8) {
    (r.0 + ((r.1 - r.0 + 1) / 2), r.1)
}

fn seek(i: char, r: (u8, u8)) -> (u8, u8) {
    match i {
        'F' | 'L' => lower_half(r),
        'B' | 'R' => upper_half(r),
        _ => panic!("Illegal instruction"),
    }
}

fn row(i: &str) -> u8 {
    let mut r = (0, 127);
    for ch in i.chars() {
        r = seek(ch, r);
    }
    (r.1 + r.0) / 2
}

fn col(i: &str) -> u8 {
    let mut r = (0, 7);
    for ch in i.chars() {
        r = seek(ch, r)
    }
    (r.1 + r.0) / 2
}

fn ids<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.split('\n').map(|line| {
        let row = row(&line[0..7]);
        let col = col(&line[7..]);
        row as u32 * 8 + col as u32
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day5.txt")?;
    let max_id = ids(&input).max();
    println!("{:?}", max_id);
    let mut ids = ids(&input).collect::<Vec<_>>();
    ids.sort_unstable();
    let my_id = ids
        .windows(2)
        .find(|g| g[1] == g[0] + 2)
        .map(|g| (g[1] + g[0]) / 2);
    println!("{:?}", my_id);
    Ok(())
}
