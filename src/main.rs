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


fn print_string(string: &str, x: u16, y: u16, direction: &str, height: u16, width: u16) {
    let direction = direction.parse::<Direction>().unwrap();
    let mut stdout = stdout();

    let steps = match direction {
        Direction::Down => height - y,
        Direction::Up => y,
        Direction::Left => x,
        Direction::Right => width - x,
    };

    for step in 0..steps {
        {
            let _lock = stdout.lock();

            for (i, ch) in string.chars().enumerate() {
                let step_updated = step + i as u16;
                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y - step_updated),
                    Direction::Left => (x - step_updated, y),
                    Direction::Right => (x + step_updated, y),
                };

                stdout.execute(cursor::MoveTo(x_new, y_new)).unwrap();
                print!("{ch}");
            }
            stdout.flush().unwrap();
        }

        sleep(Duration::from_millis(100));

        {
            let _lock = stdout.lock();

            for (i, ch) in string.chars().enumerate() {
                let step_updated = step + i as u16;
                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y - step_updated),
                    Direction::Left => (x - step_updated, y),
                    Direction::Right => (x + step_updated, y),
                };

                stdout.execute(cursor::MoveTo(x_new, y_new)).unwrap();
                print!(" ");
            }
            stdout.flush().unwrap();
        }
    }
}




fn main() 
{
    let (width, height) = terminal::size().unwrap(); // On récupère la hauteur et la largeur du terminal

    print_string("abcdef", 5, 5, "down", height, width);
    print_string("cccccc", 10, 5, "down", height, width);
    //print_char('t', 10, 10, "right", &mut stdout, height, width);

	//print_char_down('a', (0, height), 10, &mut stdout, height);    
	//print_char_up('b', (height, 10), 10, &mut stdout, height);    
	//print_char_to_right('c', (15, 30), 5, &mut stdout, width);  
	//print_char_to_left('d', (20, 0), 10, &mut stdout, width);  
}

// D'abord compiler : cargo run puis
// ./target/debug/Projet

