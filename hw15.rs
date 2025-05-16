use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() -> Vec<(u32, u32, u32)> {
    let digits: Vec<u32> = (1..=8).collect();
    let mut results = Vec::new();

    for perm in digits.iter().permutations(8).unique() {
        let m = *perm[0];
        let u = *perm[1];
        let x = *perm[2];
        let a = *perm[3];
        let s = *perm[4];
        let l = *perm[5];
        let o = *perm[6];
        let n = *perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            results.push((muxa, a, slon));
        }
    }

    results
}

pub fn run() {
    let solutions = solve();
    for (muxa, a, slon) in &solutions {
        println!("{:>5}\n  x {}\n------\n{:>5}\n", muxa, a, slon);
    }
    println!("Кількість рішень: {}", solutions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let results = solve();
        let unique: HashSet<_> = results.iter().collect();
        assert_eq!(results.len(), unique.len(), "Повторення у відповідях");
        assert!(results.len() > 0, "Немає ніякого рішення");
        for (muxa, a, slon) in results {
            assert_eq!(muxa * a, slon, "Помилка у виразі {} * {} != {}", muxa, a, slon);
        }
    }
}
