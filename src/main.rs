use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;
use std::str::FromStr;
use std::thread;

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
fn print_char_down(ch: char, (y_init, y_end) : (u16, u16), x: u16)
{
    let mut stdout = stdout(); // stdout exclusif à la fonction
	for y in y_init..y_end //Déplacement limité à la hauteur du terminal
	{
        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
            print!("{ch}"); // Affiche un caractère
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }
        
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
            print!(" "); // Remplace le caractère par "" : le caractère est effacé
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }  
    }
}

//Affiche un caractère qui monte de (x, y_init) à (x, y_end)
fn print_char_up(ch: char, (y_init, y_end) : (u16, u16), x: u16)
{
    let mut stdout = stdout(); // stdout exclusif à la fonction
	for y in (y_end..y_init).rev() //Déplacement limité à la hauteur du terminal
	{
        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
            print!("{ch}"); // Affiche un caractère
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }
        
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
            print!(" "); // Remplace le caractère par "" : le caractère est effacé
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }
        
    }
}

//Affiche un caractère qui part de (x_init, y) à (x_end, y)
fn print_char_to_right(ch: char, (x_init, x_end) : (u16, u16), y : u16)
{
    let mut stdout = stdout(); // stdout exclusif à la fonction
	for x in x_init..x_end //Déplacement limité à la largeur du terminal
	{
        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
            print!("{ch}"); // Affiche un caractère
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }

        sleep(Duration::from_millis(100)); // Pause pour l'animation

        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
            print!(" "); // Remplace le caractère par "" : le caractère est effacé
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }

    }
}

//Affiche un caractère qui part de (x_init, y) à (x_end, y)
fn print_char_to_left(ch: char, (x_init, x_end) : (u16, u16), y : u16)
{
    let mut stdout = stdout(); // stdout exclusif à la fonction
	for x in (x_end..x_init).rev() //Déplacement limité à la largeur du terminal
	{
        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Déplace le curseur
            print!("{ch}"); // Affiche un caractère
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }
 
        sleep(Duration::from_millis(100)); // Pause pour l'animation

        {
            let _lock = stdout.lock(); // Pour forcer l'ordre d'affichage
            stdout.execute(cursor::MoveTo(x, y)).unwrap(); // Place le curseur au même endroit que le caractère
            print!(" "); // Remplace le caractère par "" : le caractère est effacé
            stdout.flush().unwrap(); // Force l'affichage immédiat du caractère
        }

    }
}

// Fonction pour print un char dans la direction choisie : 
// "down" : de (x,y) à (x, height) selon l'axe x
// "up" : de (x, height) à (x, y) selon l'axe x
// "left" : de (width, y) à (x, y) selon l'axe y 
// "right" : de (x, y) à (width, y) selon l'axe y
fn print_char(ch: char, x : u16, y : u16, direction : &str, height : u16, width : u16)
{
    // Appel du trait FromStr de Direction 
    // direction.parse::<Direction>() = Direction::from_str(direction)
	match Direction::from_str(direction) // On match la valeur du type Result
    {
        Ok(Direction::Down) => print_char_down(ch, (y, height), x),
        Ok(Direction::Up) => print_char_up(ch, (height, y), x),
        Ok(Direction::Left) => print_char_to_left(ch, (width, x), y,),
        Ok(Direction::Right) => print_char_to_right(ch, (x, width), y),
        Err(err) => 
        {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn print_string(string: &str, x : u16, y : u16, direction : &str, height : u16, width : u16)
{
    let mut handles = vec![]; //Pour stocker les handles des thread

    for (i, ch) in string.chars().enumerate() {
        let i = i as u16;
        let direction_clone = direction.to_string();

        let handle = thread::spawn(move || {
            print_char(ch, x + i, y, &direction_clone, height, width);
            //println!("Thread lancé pour {} à ({}, {})", ch, x, y);
        });

        handles.push(handle); // Stocke le handle du thread en cours
        sleep(Duration::from_millis(10)); // Petit délai pour forcer l'ordre
    }

    // Attendre la fin de tous les threads
    for handle in handles {
        handle.join().unwrap();
    }

}

fn main() 
{
    let (width, height) = terminal::size().unwrap(); // On récupère la hauteur et la largeur du terminal

    print_string("abcdef", 5, 5, "down", height, width);
    //print_char('t', 10, 10, "right", &mut stdout, height, width);

	//print_char_down('a', (0, height), 10, &mut stdout, height);    
	//print_char_up('b', (height, 10), 10, &mut stdout, height);    
	//print_char_to_right('c', (15, 30), 5, &mut stdout, width);  
	//print_char_to_left('d', (20, 0), 10, &mut stdout, width);  
}

// D'abord compiler : cargo run puis
// ./target/debug/Projet

