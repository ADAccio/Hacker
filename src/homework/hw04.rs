fn draw_rhombus() {
    const H: usize = 9; 

    let mut output = String::new();

    for y in 0..H {
        for x in 0..(2 * H - 1) {
            let is_star = x >= H - 1 - y && x <= H - 1 + y;
            output.push(if is_star { '*' } else { ' ' });
        }
        output.push('\n');
    }

    for y in (0..H - 1).rev() {
        for x in 0..(2 * H - 1) {
            let is_star = x >= H - 1 - y && x <= H - 1 + y;
            output.push(if is_star { '*' } else { ' ' });
        }
        output.push('\n');
    }

    print!("{}", output);
}

fn main() {
    draw_rhombus();
}
