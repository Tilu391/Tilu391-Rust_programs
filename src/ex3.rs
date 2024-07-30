//define 2 strings constants w1 and w2 each containing different words . use a comparision oertator to check if the length of word2 
//print the result;
fn main() {
    let word1:&str = "Tilak";
    let word2:&str = "Matagunde";
    if word1.len() > word2.len(){
        println!("{} > then {}",word1, word2);
    }
    else if word1.len() == word2.len(){
        println!("{} = {}",word1,word2);
    }
    else{
        println!("{} < {} ", word1,word2);
    }
}