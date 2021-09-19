use std::io;
/**
 * unwap : permet d'ignorée les erreurs
 * let
 * mut
 * readline
 */
fn main(){
    println ! ("devine le nombre \n ");
    let mut input = String::new();
    println ! ("saississez votre proposition: ");
    io::stdin().read_line(&mut input).unwrap();
    println ! ("vous avez saisie {}",input);
}