fn main() {
    let config_max = Some(3u8);
    
    match config_max {
        Some(max) => println!("Using match, The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("Using if-let, The maximum is configured to be {max}");
    }

}
