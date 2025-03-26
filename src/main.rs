use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{Write, stdout};
use std::process::exit;
use std::thread;
use std::thread::sleep;
use std::str::FromStr;
use rand::random_range;
use std::time::{Duration, Instant};
use std::env;
use std::process::Command;

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

// Fonction pour effacer complètement le terminal
fn clear_screen() {
    let status = Command::new("clear").status(); 
    match status {
        Ok(_) => {}, // Rien à faire en cas de succès
        Err(e) => eprintln!("Erreur lors de l'effacement du terminal: {}", e),
    }
}


// Fonction permettant d'afficher une chaîne de caractère string à l'écran dans la direction précisée
// height et width sont en pratique les dimensions du terminal
// x et y sont les coordonnées de départ ou de fin de déplacement de la string
fn print_string(string: &str, x: u16, y: u16, direction: Direction, height: u16, width: u16) {
    
    let mut stdout = stdout(); // Sortie du terminal

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
                        Ok(_) => {
                            print!("{ch}");
                        },
                        Err(e) => eprintln!("Failed to move cursor: {}", e),
                    }
                    
                }
            }
            match stdout.flush() {
                Ok(_) => {}, 
                Err(e) => eprintln!("Erreur lors du flush de stdout: {}", e),
            }
            
        }
        
        // Pause pour l'animation
        sleep(Duration::from_millis(100));

        {
            // Utilisation de stdout de manière sécurisée
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

    // Calcul du nombre réel de caractères du charset en prenant en compte la taille de certains caractères spéciaux
    let charset_chars: Vec<char> = charset.chars().collect(); // Prise en compte de la taille des caractères non latins
    let charset_len = charset_chars.len(); // Nombre réel de caractères de charset

    for _ in 0..len {
        let random_index = random_range(0..charset_len); // Génère un index compris entre 0 et charset.len()
        if let Some(random_char) = charset.chars().nth(random_index) {
            result.push(random_char); // Ajoute le caractère à la chaîne
        } 
        else {
            eprintln!("Error : index {} out of range for charset '{}'", random_index, charset); // En cas d'erreur
        }
    }
    return result 
}

// Détermine le x et y de print_string selon la direction donnée par l'utilisateur
fn xy_by_direction(direction: Direction, height: u16, width: u16) -> (u16, u16){
    let (mut x, mut y) : (u16, u16) = (0, 0);

    match direction{
        Direction::Down => {
            x = random_range(0..width); 
            y = 0;
        },
        Direction::Up => {
            x = random_range(0..width);
            y = height;
        },
        Direction::Right => {
            x = 0;
            y = random_range(0..height);
        },
        Direction::Left => {
            x = width;
            y = random_range(0..height);
        },
    };

    return (x,y);
}


fn main() {

    let args: Vec<String> = env::args().collect();  // Récupérer les arguments fournis par l'utilisateur
                                                    // args[0] : nom de l'exécutable (./target/debug/Projet)
                                                    // args[1] : direction de la digital rain
                                                    // args[2] : alphabet choisi

    // Affichage d'un message d'erreur + explication d'utilisation si il n'y a pas le nombre d'argument requis
    if args.len() != 4 {
        println!("Number of arguments incorrect \nCorrect use : executable direction alphabet duration_in_seconds");
        exit(1);
    }    
    
    // Test de l'argument 1 donné par l'utilisateur
    let direction: Direction = match args[1].parse() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    
    // Test de l'argument 2 donné par l'utilisateur
    let alphabet: Alphabet = match args[2].parse() {
        Ok(alph) => alph,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    // Test de l'argument 3 donné par l'utilisateur
    let duration: u16 = match args[3].parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erreur : '{}' n'est pas une durée valide (donner un entier positif)", args[3]);
            exit(1);
        }
    };

    if let Ok((width, height)) = terminal::size() { // Dimensions du terminal
        clear_screen(); // "Vide" l'écran du terminal
        let mut stdout = stdout(); // Sortie du terminal
    
        // Masquer le curseur pour un rendu plus propre
        match stdout.execute(cursor::Hide) {
            Ok(_) => (),
            Err(e) => eprintln!("Erreur lors du masquage du curseur : {}", e),
        }
    
        let start_time = Instant::now(); // Démarrer le chronomètre
        let mut handles = vec![]; 
    
        while start_time.elapsed().as_secs() < duration as u64 { // Vérifier si durations secondes se sont écoulées
            
            // Génération de 4 threads pour afficher plus de chaînes de caractère 
            for _ in 0..4{
                let random_length = random_range(5..20); // Longueur aléatoire de la chaîne
                let generated_string = random_string(random_length, alphabet); // Générer une chaîne de random_length
                let width = width;
                let height = height;
                
                // Lancer un thread pour faire tomber la chaîne
                let handle = thread::spawn(move || {
                    let (x, y) = xy_by_direction(direction, height, width);
                    print_string(&generated_string, x, y, direction, height, width);
                });

                handles.push(handle); // Stocker la thread
            }
            
            // Attendre avant de lancer une autre chute pour éviter la surcharge
            thread::sleep(Duration::from_millis(150));
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
    
// D'abord compiler : cargo run puis lancer la commande ./target/debug/Projet down numbers
// Les caractères chinois et japonais ne s'effacent pas tous
// Rajouter en paramètre la durée de l'affichageg







/*______________________________________________________________________________________________________________________*/

//Prints a video on the terminal with a decent framerate (actually breaks down the video into multiple frames).
//OK appearance, cannot do better anyway due to inherent terminal resolution.

// DISCLAIMER : The code was generated using ChatGPT, but most of its logic and mechanics have been personally understood.


// IMPORTANT - Before executing : 
// sudo apt install ffmpeg
// cargo add image term_size crossterm



/*


use image::{GrayImage, io::Reader as ImageReader, Luma};
use std::{env, fs, thread, time::Duration, process::Command};
use term_size;
use crossterm::{execute, terminal::{Clear, ClearType}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <video_path> <character>", args[0]);
        return;
    }

    let video_path = &args[1];
    let display_char = args[2].chars().next().unwrap_or('#');

    // Extract frames using FFmpeg
    fs::create_dir_all("frames").expect("Failed to create frames directory");
    Command::new("ffmpeg")
        .args(["-i", video_path, "-vf", "fps=10,scale=80:-1", "frames/frame%04d.png"])
        .output()
        .expect("Failed to extract frames");

    // Get terminal size
    let (term_width, term_height) = term_size::dimensions().unwrap_or((80, 24));

    // Read and display frames in sequence
    let frame_paths: Vec<_> = fs::read_dir("frames")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    for frame_path in frame_paths {
        let img = ImageReader::open(&frame_path)
            .expect("Failed to open frame")
            .decode()
            .expect("Failed to decode frame")
            .into_luma8();

        let img_resized = resize_image(&img, term_width as u32, (term_height * 2) as u32);

        // Clear the terminal before printing the next frame
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

        for y in (0..img_resized.height()).step_by(2) {
            for x in 0..img_resized.width() {
                let pixel = img_resized.get_pixel(x, y)[0];
                let char_to_print = if pixel < 128 { ' ' } else { display_char };
                print!("{}", char_to_print);
            }
            println!();
        }

        // Adjust the framerate (10 FPS)
        thread::sleep(Duration::from_millis(100));
    }

    // Cleanup: Delete extracted frames
    fs::remove_dir_all("frames").expect("Failed to clean up frames");
}

fn resize_image(img: &GrayImage, width: u32, height: u32) -> GrayImage {
    let resized = image::imageops::resize(img, width, height, image::imageops::FilterType::Nearest);

    let mut binary = GrayImage::new(resized.width(), resized.height());
    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y)[0];
            let bw_pixel = if pixel < 128 { Luma([0]) } else { Luma([255]) };
            binary.put_pixel(x, y, bw_pixel);
        }
    }
    binary
}


*/