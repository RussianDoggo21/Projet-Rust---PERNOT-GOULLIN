use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;
use std::str::FromStr;
//use std::thread;
use rand::Rng;

// Enumération permettant de savoir dans quelle direction les caractères de la digital rain "tomberont" 
#[derive(Debug, Copy, Clone)]
enum Direction 
{
    Down,
    Up,
    Left,
    Right,
}

// Enumération contenant l'alphabet qui déterminera les caractères de la digital rain
#[derive(Debug, Copy, Clone)]
enum Alphabet
{
    Numbers,
    Latin,
    Cyrillic,
    Japanese,
    Chinese,
    Greek,
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
            _ => Err(format!("Error : unknown direction '{}'. \nValid directions : 'up', 'down', 'left', 'right' ", s)),
        }
    }
}

// Implémentation du trait FromStr pour l'enum Alphabet
// Retourne un Result, ce qui permet de gérer correctement les erreurs par la suite
impl FromStr for Alphabet{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "numbers" => Ok(Alphabet::Numbers),
            "latin" => Ok(Alphabet::Latin),
            "cyrillic" => Ok(Alphabet::Cyrillic),
            "japanese" => Ok(Alphabet::Japanese),
            "chinese" => Ok(Alphabet::Chinese),
            "greek" => Ok(Alphabet::Greek),
             _ => Err(format!("Error : unknown alphabet {}. \nValid alphabets : 'numbers', 'latin', 'cyrillic', 'japanese', 'chinese', 'greek' ", s))
        }
    }
}

// Fonction permettant d'afficher une chaîne de caractère string à l'écran dans la direction précisée
// height et width sont en pratique les dimensions du terminal
// x et y sont les coordonnées de départ ou de fin de déplacement de la string
fn print_string(string: &str, x: u16, y: u16, direction: &str, height: u16, width: u16) {
    let direction = direction.parse::<Direction>().unwrap();
    let mut stdout = stdout();

    // Utilisation de saturating_sub pour éviter les erreurs de résultats de soustraction négatifs (Résultats capés à 0)
    // Détermination de l'intervalle de déplacement de la chaîne de caractère
    let steps = match direction {
        Direction::Down => height.saturating_sub(y),
        Direction::Up => y.saturating_add(1),
        Direction::Left => x.saturating_add(1),
        Direction::Right => width.saturating_sub(x),
    };

    // Boucle principale pour afficher puis effacer la chaîne de caractère
    for step in 0..steps {
        {
            // Utilisation sécurisée du terminal
            let _lock = stdout.lock();
            
            //Affichage de la chaîne de caractère d'un coup
            for (i, ch) in string.chars().enumerate() {
                let step_updated = step + i as u16;

                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y.saturating_sub(step_updated)),
                    Direction::Left => (x.saturating_sub(step_updated), y),
                    Direction::Right => (x + step_updated, y),
                };

                // Vérifier si le caractère est supposé être encore visible à l'écran
                let visible = match direction {
                    Direction::Down => y_new < height,
                    Direction::Up => y_new < height,
                    Direction::Left => x_new < width,
                    Direction::Right => x_new < width,
                };
                
                // Si c'est le cas, on peut l'afficher
                if visible {
                    stdout.execute(cursor::MoveTo(x_new, y_new)).unwrap();
                    print!("{ch}");
                }
            }
            stdout.flush().unwrap();
        }
        
        // Pause pour l'animation
        sleep(Duration::from_millis(100));

        {
            let _lock = stdout.lock();
            
            //Effacement de la chaîne de caractère d'un coup
            for (i, _) in string.chars().enumerate() {
                let step_updated = step + i as u16;
                
                // Utilisation de saturating_sub à nouveau
                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y.saturating_sub(step_updated)),
                    Direction::Left => (x.saturating_sub(step_updated), y),
                    Direction::Right => (x + step_updated, y),
                };

                // Vérifier si le caractère est encore visible avant de l'effacer
                let visible = match direction {
                    Direction::Down => y_new < height,
                    Direction::Up => y_new < height,
                    Direction::Left => x_new < width,
                    Direction::Right => x_new < width,
                };

                if visible {
                    stdout.execute(cursor::MoveTo(x_new, y_new)).unwrap();
                    print!(" ");
                }
            }
            stdout.flush().unwrap();
        }
    }
}

// Fonction pour générer une string de manière aléatoire en fonction de l'alphabet choisi
fn random_string(len: usize, alphabet : &str) -> String {

    //Rajouter une enum sur les alphabets ? Gérer les erreurs type Result
    let chosen_alphabet = alphabet.parse::<Alphabet>().unwrap();

    let charset = match chosen_alphabet{
        Alphabet::Chinese => "的一是在不了有和人这中大为上个国我以要他时来用们生到作地于出就分对成会可主发年动同工也能下过子说产种面而方后多定行学法所民得经十三之进着等部度家电力里如水化高自二理起小物现实加量都两体制机当",
        Alphabet::Japanese => "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン",
        Alphabet::Cyrillic => "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя",
        Alphabet::Latin => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        Alphabet::Numbers => "0123456789",
        Alphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩαβγδεζηθικλμνξοπρστυφχψω",
    };

    
    let mut rng = rand::rng(); // Générateur de nombre pseudo-aléatoire
    let mut result = String::new(); // String à retourner à la fin
    let charset_chars: Vec<char> = charset.chars().collect(); // Convertir en Vec<char> afin d'éviter les erreurs dûes aux tailles des caractères non latins
    let charset_len = charset_chars.len(); // Nombre réel de caractères

    for _ in 0..len {
        let random_index = rng.random_range(0..charset_len); // Génère un index compris entre 0 et charset.len()
        let random_char = charset.chars().nth(random_index).unwrap(); // Sélectionne le caractère de charset à l'index random_index
        result.push(random_char); // Ajoute le caractère à la chaîne
    }

    return result 
}


fn main() 
{
    let (width, height) = terminal::size().unwrap(); // On récupère la hauteur et la largeur du terminal
    let cyrillic = random_string(20, "cyrillic");
    let str_cy: &str = &cyrillic; 

    let japanese = random_string(20, "japanese");
    let str_j: &str = &japanese; 

    let chinese = random_string(20, "chinese");
    let str_ch: &str = &chinese; 

    let greek = random_string(20, "greek");
    let str_g: &str = &greek; 

    print_string(str_cy, 5, 0, "down", height, width);
    print_string(str_j, 10, 0, "down", height, width);
    print_string(str_ch, 15, 0, "down", height, width);
    print_string(str_g, 20, 0, "down", height, width);
    //OK 
    //print_string("abcdef", 5, 5, "right", height, width);
    //print_string("abcdef", 5, 5, "down", height, width);

    //Erreur de soustraction
    //print_string("abcdef", 5, height, "up", height, width);
    //print_string("cccccc", 100, 10, "left", height, width);

}

// D'abord compiler : cargo run puis
// ./target/debug/Projet

