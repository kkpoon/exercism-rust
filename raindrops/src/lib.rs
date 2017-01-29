pub fn raindrops(n: u64) -> String {
    let drops = (1..n + 1)
        .filter(|&x| n % x == 0)
        .map(|x| match x {
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
