pub fn resistor_value(ring1: &str, ring2: &str, ring3: &str) -> f64 {
    let mut value: f64 = get_ring_value(ring1) as f64 *10.0 + get_ring_value(ring2) as f64;
    let r3: i32 = get_ring_value(ring3);
    if r3 == -1 {
        value *= 0.1;
    } else if r3 == -2 {
        value *= 0.01;
    } else {
        for _ in 0..r3 {
            value *= 10.0;
    }
    }
    return value;
}

fn get_ring_value(ring: &str) -> i32 {
    match ring{
        "Black" => 0,
        "Brown" => 1,
        "Red" => 2,
        "Orange" => 3,
        "Yellow" => 4,
        "Green" => 5,
        "Blue" => 6,
        "Violet" => 7,
        "Grey" => 8,
        "White" => 9,
        "Silver" => -2,
        "Gold" => -1,
        &_ => panic!("Invalid color"),
    }
}
