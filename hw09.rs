fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let shift = ((n % len) + len) % len;

    let bytes = s.as_bytes();
    let rotated = [&bytes[(len - shift) as usize..], &bytes[..(len - shift) as usize]].concat();

    String::from_utf8(rotated).unwrap()
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.to_string(), *n), exp.to_string());
    });
}
    