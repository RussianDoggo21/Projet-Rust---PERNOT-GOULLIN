//Test de Vianney, j'espere que ca marche cette fois bachi-bouzouk de mille sabords

use crossterm::{cursor, execute, terminal, ExecutableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::io::Stdout;
use std::str::FromStr;

// Enumération permettant de savoir dans quelle direction les caractères de la digital rain "tomberont" 
enum Direction 
{
    Down,
    Up,
    Left,
    Right,
}


// Implémentation du trait FromStr pour l'enum Direction
// Retourne un Result, ce qui permet de gérer correctement les erreurs par la suite
impl FromStr for Direction 
{
    type Err = String; // Le type de l'erreur sera une String

    // On remarque que from_str retourne bien un type Result
    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        match s {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err(format!("Erreur : direction '{}' non reconnue.", s)),
        }
    }
}


//Affiche un caractère qui descend de (x, y_init) à (x, y_end)
fn print_char_down(ch: char, (y_init, y_end) : (u16, u16), x: u16, mut stdout : &mut Stdout)
{
	for y in y_init..y_end //Déplacement limité à la hauteur du terminal
	{
        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
        print!("{ch}"); // Affiche un caractère
        stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
        print!(" "); // Remplace le caractère par "" : le caractère est effacé
    }
}

//Affiche un caractère qui monte de (x, y_init) à (x, y_end)
fn print_char_up(ch: char, (y_init, y_end) : (u16, u16), x: u16, mut stdout : &mut Stdout)
{
	for y in (y_end..y_init).rev() //Déplacement limité à la hauteur du terminal
	{
        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
        print!("{ch}"); // Affiche un caractère
        stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
        print!(" "); // Remplace le caractère par "" : le caractère est effacé
    }
}

//Affiche un caractère qui part de (x_init, y) à (x_end, y)
fn print_char_to_right(ch: char, (x_init, x_end) : (u16, u16), y : u16, mut stdout : &mut Stdout)
{
	for x in x_init..x_end //Déplacement limité à la largeur du terminal
	{
        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
        print!("{ch}"); // Affiche un caractère
        stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
        print!(" "); // Remplace le caractère par "" : le caractère est effacé
    }
}

//Affiche un caractère qui part de (x_init, y) à (x_end, y)
fn print_char_to_left(ch: char, (x_init, x_end) : (u16, u16), y : u16, mut stdout : &mut Stdout)
{
	for x in (x_end..x_init).rev() //Déplacement limité à la largeur du terminal
	{
        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
        print!("{ch}"); // Affiche un caractère
        stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
        print!(" "); // Remplace le caractère par "" : le caractère est effacé
    }
}

// Fonction pour print un char dans la direction choisie : 
// "down" : de (x,y) à (x, height) selon l'axe x
// "up" : de (x, height) à (x, y) selon l'axe x
// "left" : de (width, y) à (x, y) selon l'axe y 
// "right" : de (x, y) à (width, y) selon l'axe y
fn print_char(ch: char, x : u16, y : u16, direction : &str, stdout : &mut Stdout, height : u16, width : u16)
{
    // Appel du trait FromStr de Direction 
    // direction.parse::<Direction>() = Direction::from_str(direction)
	match Direction::from_str(direction) // On match la valeur du type Result
    {
        Ok(Direction::Down) => print_char_down(ch, (y, height), x, stdout),
        Ok(Direction::Up) => print_char_up(ch, (height, y), x, stdout),
        Ok(Direction::Left) => print_char_to_left(ch, (width, x), y, stdout),
        Ok(Direction::Right) => print_char_to_right(ch, (x, width), y, stdout),
        Err(err) => 
        {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn print_string(string: &str, x : u16, y : u16, direction : &str, stdout : &mut Stdout, height : u16, width : u16)
{
    for ch in string.chars()
    {
        print_char(ch, x, y, direction,stdout, height, width);
    }
}

fn main() 
{
    let mut stdout = stdout(); // On récupère la sortie du terminal
    let (width, height) = terminal::size().unwrap(); // On récupère la hauteur et la largeur du terminal

    print_string("abcdef", 5, 5, "down", &mut stdout, height, width);
    //print_char('t', 10, 10, "right", &mut stdout, height, width);

	//print_char_down('a', (0, height), 10, &mut stdout, height);    
	//print_char_up('b', (height, 10), 10, &mut stdout, height);    
	//print_char_to_right('c', (15, 30), 5, &mut stdout, width);  
	//print_char_to_left('d', (20, 0), 10, &mut stdout, width);  
}

// D'abord compiler : cargo run puis
// ./target/debug/Projet

