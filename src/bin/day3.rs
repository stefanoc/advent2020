fn is_tree(row: &str, offset: usize) -> bool {
    row.chars().nth(offset % row.len()) == Some('#')
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("data/day3.txt")?;
    let rows = input.split('\n').collect::<Vec<_>>();
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
            if is_tree(rows[y_offset], x_offset) {
                trees += 1;
            }
        }
        println!("{}", trees);
        result = result * trees;
    }
    println!("{}", result);
    Ok(())
}
