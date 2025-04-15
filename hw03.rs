fn envelope() {
    const W: u32 = 40; 
    const H: u32 = 20; 

    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            let is_border = y == 0 || y == H - 1 || x == 0 || x == W - 1;
            let is_diag1 = x == (y * W) / H; 
            let is_diag2 = x == W - 1 - (y * W) / H; 

            let sym = if is_border || is_diag1 || is_diag2 {
                '*'
            } else {
                ' '
            };

            output.push(sym);
        }
        output.push('\n'); 
    }

    print!("{}", output);
}

fn main() {
    envelope();
}


