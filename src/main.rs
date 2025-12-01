// Bitwise operations
fn main() {
    let mut value = 0b1111_0101u8;
    println!("Value is {}", value);
    println!("Value is {:08b}", value);

    // NOT
    value = !value;
    println!("Value is {:08b}", value);

    // AND
    value = value & 0b1111_0111;
    println!("Value is {:08b}", value);

    value = value | 0b0100_0000;
    println!("Value is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("Value is {:08b}", value);

    value = value >> 2;
    println!("Value is {:08b}", value);

    value = value << 4;
    println!("Value is {:08b}", value);
}
