fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("{}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("the max value is {}", max);
    }
}
