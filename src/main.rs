use std::io;
use rand::{thread_rng, Rng};
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

    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..10);
    println!("nombre aléatoirement generé : {}", n);
}
/*
 * cargo init pour lancer cargo
 * cargo run pour compiler et executer
 * git push pour modifié le git
 * 
 */