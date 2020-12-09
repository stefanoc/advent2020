fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day3.txt")?;
    let rows = input
        .split('\n')
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = rows[0].len();
    let mut result = 1;
    let runs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for run in &runs {
        let mut trees = 0;
        let mut x_offset = 0usize;
        let mut y_offset = 0usize;
        loop {
            x_offset += run.0;
            y_offset += run.1;
            if y_offset >= rows.len() {
                break;
            }
            if rows[y_offset][x_offset % width] {
                trees += 1;
            }
        }
        println!("{}", trees);
        result *= trees;
    }
    println!("{}", result);
    Ok(())
}
