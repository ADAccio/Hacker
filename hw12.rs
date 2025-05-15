use rand::Rng;

pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1; 
    }

    let avg = total / n;
    let mut moves = 0;
    let mut diff = 0;

    for &weight in shipments {
        diff += weight as isize - avg as isize;
        moves += diff.abs();
    }

    moves / 2
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = 50;
    let mut rng = rand::thread_rng();
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(avg - 20..=avg + 20)).collect();
    let total: i32 = shipments.iter().map(|&x| x as i32).sum();
    let diff = total - (avg as i32 * n as i32);

    if let Some(last) = shipments.last_mut() {
        let corrected = (*last as i32 - diff) as u32;
        *last = corrected.max(10).min(99);
    }

    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        let data1 = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&data1), 4);

        let data2 = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&data2), 7);

        let data3 = vec![1, 1, 1, 6];
        assert_eq!(count_permutation(&data3), -1); 
    }

    #[test]
    fn test_gen_shipments() {
        let shipments = gen_shipments(10);
        let total: u32 = shipments.iter().sum();
        assert_eq!(total % shipments.len() as u32, 0);
    }
}
