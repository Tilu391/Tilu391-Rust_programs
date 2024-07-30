fn main() {
    let secret_message = "Tilak".to_string();
    let encrypt_message = encrypt_mess(secret_message,6);
    println!("{}",encrypt_message);

}

fn encrypt_mess(secret_message:String,key:u8) -> String {
    let mut temp_sting = String::from("");
    for i in secret_message.chars(){
        temp_sting.push(((i as u8) + key) as char);
    }
    return temp_sting;


}