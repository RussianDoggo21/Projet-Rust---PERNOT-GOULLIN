use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{Write, stdout};
use std::process::exit;
use std::thread;
use std::thread::sleep;
use std::str::FromStr;
use rand::random_range;
use std::time::{Duration, Instant};
//use std::process::Command;
use std::env;

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
            _ => Err(format!("Error : unknown direction '{}'. Valid directions : 'up', 'down', 'left', 'right' ", s)),
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
             _ => Err(format!("Error : unknown alphabet '{}'. Valid alphabets : 'numbers', 'latin', 'cyrillic', 'japanese', 'chinese', 'greek' ", s)),
        }
    }
}

/* Abandonné pour l'instant */
/* 
// Fonction pour modifier la taille de la police dans Kitty
fn set_kitty_font_size(size: u16) {
    Command::new("kitty")
        .args(&["@set-font-size", &size.to_string()])
        .spawn()
        .expect("Failed to change font size");
}

// Fonction pour réinitialiser la taille de la police dans Kitty
fn reset_kitty_font_size() {
    Command::new("kitty")
        .args(&["@set-font-size", "14"]) // Remettre à la taille par défaut
        .spawn()
        .expect("Failed to reset font size");
}
*/

// Fonction permettant d'afficher une chaîne de caractère string à l'écran dans la direction précisée
// height et width sont en pratique les dimensions du terminal
// x et y sont les coordonnées de départ ou de fin de déplacement de la string
fn print_string(string: &str, x: u16, y: u16, direction: Direction, height: u16, width: u16) {
    
    let mut stdout = stdout(); // Sortie du terminal

    // Modifier la taille de la police avec Kitty
    //set_kitty_font_size(font_size);

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
                    match stdout.execute(cursor::MoveTo(x_new, y_new)){
                        Ok(_) => print!("{ch}"),
                        Err(e) => eprintln!("Failed to move cursor: {}", e),
                    }
                    
                }
            }
            match stdout.flush() {
                Ok(_) => {}, // Rien à faire en cas de succès
                Err(e) => eprintln!("Erreur lors du flush de stdout: {}", e),
            }
            
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
                    match stdout.execute(cursor::MoveTo(x_new, y_new)){
                        Ok(_) => print!(" "),
                        Err(e) => eprintln!("Failed to move cursor: {}", e),
                    }
                }
            }
            match stdout.flush() {
                Ok(_) => {}, // Rien à faire en cas de succès
                Err(e) => eprintln!("Erreur lors du flush de stdout: {}", e),
            }
            
        }
    }

    // Réinitialisation de la police de Kitty après affichage
    //reset_kitty_font_size();

}

// Fonction pour générer une string de manière aléatoire en fonction de l'alphabet choisi
fn random_string(len: usize, alphabet : Alphabet) -> String {

    let charset = match alphabet{
        Alphabet::Chinese => "的一是在不了有和人这中大为上个国我以要他时来用们生到作地于出就分对成会可主发年动同工也能下过子说产种面而方后多定行学法所民得经十三之进着等部度家电力里如水化高自二理起小物现实加量都两体制机当",
        Alphabet::Japanese => "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン",
        Alphabet::Cyrillic => "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя",
        Alphabet::Latin => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        Alphabet::Numbers => "0123456789",
        Alphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩαβγδεζηθικλμνξοπρστυφχψω",
    };

    let mut result = String::new(); // String à retourner à la fin
    let charset_chars: Vec<char> = charset.chars().collect(); // Afin d'éviter les erreurs dûes aux tailles des caractères non latins
    let charset_len = charset_chars.len(); // Nombre réel de caractères

    for _ in 0..len {
        let random_index = random_range(0..charset_len); // Génère un index compris entre 0 et charset.len()
        if let Some(random_char) = charset.chars().nth(random_index) {
            result.push(random_char); // Ajoute le caractère à la chaîne
        } else {
            eprintln!("Error : index {} out of range for charset '{}'", random_index, charset); // En cas d'erreur
        }
    }

    return result 
}

// Modifier la vitesse de chute des lettres en fonction de leur taille (abandonné)
// Afficher différentes tailles des lettres (abandonné)
// Arrière-plan / premier plan (abandonné)

// Affichage de plusieurs alphabets choisi au hasard
fn main() {

    let args: Vec<String> = env::args().collect(); // Récupérer les arguments fournis par l'utilisateur
    // args[0] : nom de l'exécutable (./target/debug/Projet)
    // args[1] : direction de la digital rain
    // args[2] : alphabet choisi

    // Affichage d'un message d'erreur + explication d'utilisation si il n'y a pas le nombre d'argument requis
    if args.len() != 3 {
        println!("Number of arguments incorrect \nCorrect use : executable direction alphabet");
        exit(1);
    }    
    
    // Test des strings données par l'utilisateur
    let direction: Direction = match args[1].parse() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    
    let alphabet: Alphabet = match args[2].parse() {
        Ok(alph) => alph,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    //let (width, height) = terminal::size().unwrap(); // Dimensions du terminal
    if let Ok((width, height)) = terminal::size() {
        let mut stdout = stdout(); // Sortie du terminal
    
        // Masquer le curseur pour un rendu plus propre
        match stdout.execute(cursor::Hide) {
            Ok(_) => (),
            Err(e) => eprintln!("Erreur lors du masquage du curseur : {}", e),
        }
    
        let start_time = Instant::now(); // Démarrer le chronomètre
        let duration = Duration::from_secs(10); // Durée totale de la génération
        let mut handles = vec![]; // Stocke les threads pour les attendre
    
        for _ in 0..duration.as_millis() { // Vérifier si 10 secondes se sont écoulées
            for _ in 0..4{
                let col = random_range(0..width); // Choisir une colonne aléatoire
                let length = random_range(5..20); // Longueur aléatoire de la chute
                //let font_size = random_range(5..100); // Taille aléatoire de la string
                let generated_string = random_string(length, alphabet); // Générer une chaîne
                
                // Lancer un thread pour faire tomber la chaîne
                let width = width;
                let height = height;
                let handle = thread::spawn(move || {
                    print_string(&generated_string, col, 0, direction, height, width);
                });
                handles.push(handle);
            }
            
            // Attendre avant de lancer une autre chute pour éviter la surcharge
            thread::sleep(Duration::from_millis(150));
    
            // Arrêter si le temps est écoulé
            if start_time.elapsed() >= duration {
                break;
            }
        }
    
        // Attendre que tous les threads en cours se terminent
        for handle in handles {
            handle.join().unwrap();
        }
    
        // Réafficher le curseur avant de quitter
        match stdout.execute(cursor::Show) {
            Ok(_) => (),
            Err(e) => eprintln!("Erreur lors du masquage du curseur : {}", e),
        }

    } 
    
    else {
        eprintln!("Erreur : impossible d'obtenir la taille du terminal.");
        return;
    }
        
}
    
// D'abord compiler : cargo run puis lancer la commande ./target/debug/Projet
    
    
    
    

