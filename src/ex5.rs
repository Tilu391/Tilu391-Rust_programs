fn main() {
    let m1 = "TILAK".to_string();
    let m2 = "Matagunde".to_string();
    let separator = "|".to_string();
    let num_separator = 4;

    let final_ = m1 + &separator.repeat(num_separator) + &m2;
    println!("{}",final_);
}