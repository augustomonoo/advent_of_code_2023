use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    let data = std::fs::read_to_string(file_path).unwrap();
    let mut total = 0;

    let mut replacements = HashMap::new();
    replacements.insert("one", 1);
    replacements.insert("two", 2);
    replacements.insert("three", 3);
    replacements.insert("four", 4);
    replacements.insert("five", 5);
    replacements.insert("six", 6);
    replacements.insert("seven", 7);
    replacements.insert("eight", 8);
    replacements.insert("nine", 9);
    replacements.insert("1", 1);
    replacements.insert("2", 2);
    replacements.insert("3", 3);
    replacements.insert("4", 4);
    replacements.insert("5", 5);
    replacements.insert("6", 6);
    replacements.insert("7", 7);
    replacements.insert("8", 8);
    replacements.insert("9", 9);

    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut min: (usize, &str) = (std::usize::MAX, "");
        let mut max: (usize, &str) = (0, "");
        for replacement in replacements.keys() {
            for m in line.match_indices(replacement) {
                if m.0 <= min.0 {
                    min = m;
                }
                if m.0 >= max.0 {
                    max = m;
                }
            }
        }
        let number = [min, max]
            .iter()
            .map(|n| replacements.get(n.1).unwrap().to_owned())
            .reduce(|a, b| (a * 10) + b)
            .unwrap();
        total += number;
    }
    println!("{}", total);
}
