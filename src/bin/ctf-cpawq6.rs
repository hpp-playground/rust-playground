fn main() {
    let code = "fsdz{Fdhvdu_flskhu_lv_fodvvlfdo_flskhu}";
    let test = code
        .chars()
        .map(|ch| {
            if (ch as u8 > 67 && ch as u8 <= 90) || (ch as u8 >= 96 && ch as u8 <= 122) {
                (ch as u8 - 3) as char
            } else {
                ch
            }
        })
        .collect::<String>();
    println!("{}", test)
}
