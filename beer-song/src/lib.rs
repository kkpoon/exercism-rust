fn bottle(n: u32) -> String {
    match n {
        x if x == 0 => format!("No more bottles of beer on the wall, no more bottles of beer"),
        x if x == 1 => format!("{} bottle of beer on the wall, {} bottle of beer", n, n),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer", n, n),
    }
}

fn action(n: u32) -> String {
    match n {
        x if x == 0 => "Go to the store and buy some more".to_string(),
        x if x == 1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string(),
    }
}

fn after(n: u32) -> String {
    match n {
        x if x == 0 => "99 bottles of beer on the wall".to_string(),
        x if x == 1 => "no more bottles of beer on the wall".to_string(),
        x if x == 2 => "1 bottle of beer on the wall".to_string(),
        _ => format!("{} bottles of beer on the wall", n - 1),
    }
}

pub fn verse(n: u32) -> String {
    format!("{}.\n{}, {}.\n", bottle(n), action(n), after(n))

}

pub fn sing(from: u32, to: u32) -> String {
    if from > to {
        format!("{}\n{}", verse(from), sing(from - 1, to))
    } else {
        verse(from)
    }
}
