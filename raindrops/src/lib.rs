pub fn raindrops(n: i32) -> String {
    let drops = vec![3, 5, 7]
        .iter()
        .filter(|&x| n % x == 0)
        .map(|&x| match x {
            3 => "Pling",
            5 => "Plang",
            7 => "Plong",
            _ => "",
        })
        .collect::<Vec<_>>()
        .join("");

    match drops.as_ref() {
        "" => n.to_string(),
        _ => drops,
    }
}
