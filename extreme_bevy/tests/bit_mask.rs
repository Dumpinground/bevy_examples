#[test]
fn mask_list() {
    let up: u8 = 1 << 0;
    let down: u8 = 1 << 1;
    let left: u8 = 1 << 2;
    let right: u8 = 1 << 3;
    let fire: u8 = 1 << 4;
    println!("{up} {down} {left} {right} {fire}");

    let mut input = 1u8;

    input |= fire;
    println!("{input}");
}
