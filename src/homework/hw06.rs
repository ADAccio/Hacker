fn draw_tree(layers: usize) {
    let total_height = (1..=layers).map(|n| n + 1).sum::<usize>();
    let max_width = 2 * total_height - 1;

    let mut current_line = 0;
    let output: String = (1..=layers)
        .flat_map(|triangle| {
            (0..triangle + 1).map(move |i| {
                current_line += 1;
                let stars = 2 * i + 1;
                let spaces = (max_width - stars) / 2;
                format!("{}{}\n", " ".repeat(spaces), "*".repeat(stars))
            })
        })
        .collect();

    print!("{}", output);
}

fn main() {
    const TRIANGLES: usize = 5;
    draw_tree(TRIANGLES);
}

