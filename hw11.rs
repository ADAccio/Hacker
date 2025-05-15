use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    Some((min_index, data[min_index], data[min_index + 1]))
}

pub fn print_result(data: &[i32]) {
    let n = data.len();

    print!("indexes:");
    for i in 0..n {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:   [");
    for (i, val) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{:>2}", val);
    }
    println!("]");

    if let Some((i, a, b)) = min_adjacent_sum(data) {
        print!("indexes:");
        for j in 0..n {
            if j == i {
                print!("\\__");
            } else if j == i + 1 {
                print!(" __/");
            } else {
                print!("     ");
            }
        }
        println!();
        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            a,
            b,
            a + b,
            i,
            i + 1
        );
    } else {
        println!("Not enough elements");
    }
}
