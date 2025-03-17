use crossterm::{
    cursor, execute, terminal,
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::io::Stdout;

//Affiche un caractère qui descend de (x, y_init) à (x, y_end)
fn print_char_down(ch: char, (y_init, y_end) : (u16, u16), x: u16, mut stdout : &mut Stdout, height : u16)
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
fn print_char_up(ch: char, (y_init, y_end) : (u16, u16), x: u16, mut stdout : &mut Stdout, height : u16)
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
fn print_char_to_right(ch: char, (x_init, x_end) : (u16, u16), y : u16, mut stdout : &mut Stdout, width : u16)
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
fn print_char_to_left(ch: char, (x_init, x_end) : (u16, u16), y : u16, mut stdout : &mut Stdout, width : u16)
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
// "down" : de (x,y) à (x, height)
// "up" : de (x, height) à (x, y)
// "left" : de (width, y) à (x, y)
// "right" : de (x, y) à (width, y)
fn print_char(ch: char, x : u16, y : u16, direction : &str, stdout : &mut Stdout, height : u16, width : u16)
{
	match direction
	{
		"down" => print_char_down(ch, (y, height), x, stdout, height),
		"up" => print_char_up(ch, (height, y), x, stdout, height),
		"left" => print_char_to_left(ch, (width, x), y, stdout, width),
		"right" => print_char_to_right(ch, (x, width), y, stdout, width),
		_ => 
		{
			eprintln!("Erreur : direction '{}' non reconnue.", direction);
			std::process::exit(1);
		}
		
	}
}

fn main() {
    let mut stdout = stdout(); // On récupère la sortie du terminal
    let (width, height) = terminal::size().unwrap(); // On récupère la hauteur et la largeur du terminal

    print_char('t', 10, 10, "right", &mut stdout, height, width);

	//print_char_down('a', (0, height), 10, &mut stdout, height);    
	//print_char_up('b', (height, 10), 10, &mut stdout, height);    
	//print_char_to_right('c', (15, 30), 5, &mut stdout, width);  
	//print_char_to_left('d', (20, 0), 10, &mut stdout, width);  
}

// D'abord compiler : cargo run puis
// ./target/debug/Projet

