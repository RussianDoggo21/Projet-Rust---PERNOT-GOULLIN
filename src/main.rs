// DISCLAIMER : While all the ideas behind this project stem from our brains, numerous implementations were done thanks to ChatGPT.
// See the file log.txt for more informations

use std::fs;
use std::io::{stdout, Write};
use std::process::{Command};
use std::str::FromStr;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::{cursor, execute, terminal, ExecutableCommand};
use crossterm::cursor::MoveTo;
use image::{GrayImage, ImageReader, Luma};
use rand::random_range;
use term_size;


// Enumeration to encapsulate the two possible argument types of the function parse_args (see below)
enum ParsedArgs {
    Mode1(Direction, Alphabet, u16),
    Mode2(String, char),
}

// Enumeration that contains the direction in which the digital rain may fall
#[derive(Debug, Copy, Clone)]
enum Direction 
{
    Down,
    Up,
    Left,
    Right,
}

// Enumeration containing the alphabets that will determine the characters of the digital rain
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

// Implementation of the trait FromSTr for the enum Direction
// Return a Result, allowing us to correctly tackle errors
impl FromStr for Direction 
{
    type Err = String; // The error type will be a String

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

// Implementation of the trait FromStr for the enumeration Alphabet
// It has the same purpose as the one for Direction
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

// Function used to entirely clear the terminal before the digital rain
fn clear_screen() {
    let status = Command::new("clear").status(); 
    if let Err(e) = status {
        eprintln!("Error during the clearance of the terminal: {}", e);
    }    
}


// Main function : print a string at the screen in the precised direction
// height and width are in practice the terminal's dimensions
// x and y are either the coordinates of start or of end of the string's movement
fn print_string(string: &str, x: u16, y: u16, direction: Direction, height: u16, width: u16) {
    
    let mut stdout = stdout(); // Output of the terminal

    // Determination of the movements's intervall of the string
    // Use of saturating_sub in order to prevent errors of negative result of soustraction (results minimized at 0)
    // Why : we can't print a character at the position (-3, 10) or (5, -7)
    let steps = match direction {
        Direction::Down => height.saturating_sub(y),
        Direction::Up => y.saturating_add(1),
        Direction::Left => x.saturating_add(1),
        Direction::Right => width.saturating_sub(x),
    };

    // Main loop to print then erase a string
    for step in 0..steps {
        {
            // Secured and exclusive use of the terminal
            let _lock = stdout.lock();
            
            // Print of the string in one go

            // Enumeration on the characters of the string
            for (i, ch) in string.chars().enumerate() { 
                let step_updated = step + i as u16;
                
                // Establishment of the position (x_new, y_new) to print the character
                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y.saturating_sub(step_updated)),
                    Direction::Left => (x.saturating_sub(step_updated), y),
                    Direction::Right => (x + step_updated, y),
                };

                // Checking if the character is supposed to still be visible on the screen
                let visible = x_new < width && y_new < height;
                
                // If that is the case, we can print it
                if visible {
                    match stdout.execute(cursor::MoveTo(x_new, y_new)){
                        Ok(_) => {
                            print!("{ch}");
                        },
                        Err(e) => eprintln!("Failed to move cursor: {}", e),
                    }
                    
                }
            }
        }
        
        // Pause for the animation
        sleep(Duration::from_millis(100));

        {
            // Secured and exclusive use of the terminal
            let _lock = stdout.lock();
            
            // Erasing of the string in one go
            // Similar process for the print of the string 
            // Only difference is that we are printing a " " character over the previous ch character

            for (i, _) in string.chars().enumerate() {
                let step_updated = step + i as u16;
                
                let (x_new, y_new) = match direction {
                    Direction::Down => (x, y + step_updated),
                    Direction::Up => (x, y.saturating_sub(step_updated)),
                    Direction::Left => (x.saturating_sub(step_updated), y),
                    Direction::Right => (x + step_updated, y),
                };

                let visible = x_new < width && y_new < height;
                    
                if visible {
                    match stdout.execute(cursor::MoveTo(x_new, y_new)){
                        Ok(_) => print!(" "),
                        Err(e) => eprintln!("Failed to move cursor: {}", e),
                    }
                }
            } 
        }
    }
}

// Function generating a random string based on the chosen alphabet
fn random_string(string_len: usize, alphabet : Alphabet) -> String {

    // All the possible alphabets available
    let charset = match alphabet{
        Alphabet::Chinese => "的一是在不了有和人这中大为上个国我以要他时来用们生到作地于出就分对成会可主发年动同工也能下过子说产种面而方后多定行学法所民得经十三之进着等部度家电力里如水化高自二理起小物现实加量都两体制机当",
        Alphabet::Japanese => "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン",
        Alphabet::Cyrillic => "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюя",
        Alphabet::Latin => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        Alphabet::Numbers => "0123456789",
        Alphabet::Greek => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩαβγδεζηθικλμνξοπρστυφχψω",
    };

    let mut result = String::new(); // String to be returned

    // Computation of the true number of the characters in the charset while taking in account the size of some special character
    let charset_chars: Vec<char> = charset.chars().collect(); // Taking account of the size of non-latin characters
    let charset_len = charset_chars.len(); // True number of characters in the charset

    for _ in 0..string_len {
        let random_index = random_range(0..charset_len); // Generate an index between 0 and charset.len() - 1
        if let Some(random_char) = charset.chars().nth(random_index) { // Select the character of charset at the index random_index
            result.push(random_char); // Add the character to result
        } 
        else {
            eprintln!("Error : index {} out of range for charset '{}'", random_index, charset); 
        }
    }
    return result 
}

// Determine the x and y needed in print_string based ont the direction given by the user
fn xy_by_direction(direction: Direction, height: u16, width: u16) -> (u16, u16){
    let (x, y) : (u16, u16);

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

// Used to resize an image (what can I say)
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

// Function to parse user-provided arguments
fn parse_args() -> Result<ParsedArgs, String> {
    let args: Vec<String> = std::env::args().collect(); // Retrieve command-line arguments
    
    // Check the number of arguments
    if args.len() != 5 && args.len() != 4 {
        return Err(format!(
            "Incorrect number of arguments\n\
             Correct usage 1: {} 1 <direction> <alphabet> <duration_in_seconds>\n\
             Correct usage 2: {} 2 <video_path> <character>",
            args[0], args[0]
        ));    
    }    

    // Parse the mode type (1 or 2)
    let type_main: u16 = args[1].parse::<u16>()
        .map_err(|_| format!("Error: '{}' is not a valid number", args[1]))?;
    
    if type_main != 1 && type_main != 2 {
        return Err(format!("Error: '{}' is not a valid option (choose between 1 and 2)", args[1]));
    }

    // If `type_main == 1`, validate <direction>, <alphabet>, and <duration_in_seconds>
    if type_main == 1 {
        let direction: Direction = args[2].parse()
            .map_err(|_| format!("Error: '{}' is not a valid direction", args[2]))?;
        let alphabet: Alphabet = args[3].parse()
            .map_err(|_| format!("Error: '{}' is not a valid alphabet", args[3]))?;
        let duration: u16 = args[4].parse()
            .map_err(|_| format!("Error: '{}' is not a valid duration (expected a positive integer)", args[4]))?;
        
        return Ok(ParsedArgs::Mode1(direction, alphabet, duration));
    }

    // If `type_main == 2`, validate <video_path> and <character>
    let video_path = args[2].clone();
    let character = args[3].chars().next()
        .ok_or_else(|| format!("Error: '{}' is not a valid character", args[3]))?;

    Ok(ParsedArgs::Mode2(video_path, character))
}

// Function to call the digital rain
fn main1(direction : Direction, alphabet : Alphabet, duration : u16){
    if let Ok((width, height)) = terminal::size() { // Dimensions of the terminal
        clear_screen(); // Clear the terminal's screen
        
        let mut stdout = stdout(); // Recovery of the terminal's output to hide its cursor for a better rendering
        if let Err(e) = stdout.execute(cursor::Hide) {
            eprintln!("Error during the camouflage of the cursor: {}", e);
        }
    
        let start_time = Instant::now(); // Start the timer (to determine when to stop the digital rain)
        let mut handles = vec![]; // Vector to store the handles of the threads used later on
    
        while start_time.elapsed().as_secs() < duration as u64 { // Checking if duration seconds have been elapsed
            
            // Generation of 4 threads to parallelise print_string() 
            for _ in 0..4{
                let random_length = random_range(5..20); 
                let generated_string = random_string(random_length, alphabet); // Generation of a string of random_length
                let width = width; // Copy of the dimensions of the terminal
                let height = height; // Copy of the dimensions of the terminal
                
                // Launch a thread to make the string "fall" in the screen
                let handle = thread::spawn(move || {
                    let (x, y) = xy_by_direction(direction, height, width);
                    print_string(&generated_string, x, y, direction, height, width);
                });

                handles.push(handle); // Store the thread
            }
            
            // Wait before launching another "fall" to prevent overloading
            thread::sleep(Duration::from_millis(150));
        }
    
        // Waiting that all threads are done
        for handle in handles {
            handle.join().unwrap();
        }
    
        // Display the cursor again before ending the programme
        if let Err(e) = stdout.execute(cursor::Show){
            eprintln!("Erreur lors du masquage du curseur : {}", e)
        }
    } 
    
    else {
        eprintln!("Erreur : impossible d'obtenir la taille du terminal.");
        return;
    }
}

// Function to display a video on the terminal screen
fn main2(video_path : String, display_char: char){

    let video_path_str = video_path.as_str();
    // Extract frames using FFmpeg
    fs::create_dir_all("frames").expect("Failed to create frames directory");
    Command::new("ffmpeg")
        .args(["-i", video_path_str, "-vf", "fps=10,scale=80:-1", "frames/frame%04d.png"])
        .output()
        .expect("Failed to extract frames");

    // Get terminal size
    let (term_width, term_height) = term_size::dimensions().unwrap_or((80, 24));

    // Read and display frames in sequence
    let mut frame_paths: Vec<_> = fs::read_dir("frames")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    frame_paths.sort(); // Ensure frames are displayed in correct order

    let mut stdout = stdout(); // Get a handle to stdout

    for frame_path in frame_paths {
        let img = ImageReader::open(&frame_path)
            .expect("Failed to open frame")
            .decode()
            .expect("Failed to decode frame")
            .into_luma8();

        let img_resized = resize_image(&img, term_width as u32, (term_height * 2) as u32);

        // Move cursor to the top before printing the new frame
        execute!(stdout, MoveTo(0, 0)).unwrap();

        for y in (0..img_resized.height()).step_by(2) {
            for x in 0..img_resized.width() {
                let pixel = img_resized.get_pixel(x, y)[0];
                let char_to_print = if pixel < 128 { ' ' } else { display_char };
                print!("{}", char_to_print);
            }
            println!();
        }

        stdout.flush().unwrap(); // Force immediate print to prevent lag

        // Adjust the framerate (10 FPS)
        thread::sleep(Duration::from_millis(80));
    }

    // Cleanup: Delete extracted frames
    fs::remove_dir_all("frames").expect("Failed to clean up frames");
}

fn main() {
    // Test of the arguments with parse_args then call of the correct function
    match parse_args() {
        Ok(ParsedArgs::Mode1(direction, alphabet, duration)) => {
            main1(direction, alphabet, duration);
        }
        Ok(ParsedArgs::Mode2(video_path, character)) => {
            main2(video_path, character);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
     
// First compile : cargo run 
// Then launch the command : ./target/debug/Projet 1 down numbers 5
//                      OR : ./target/debug/Projet 2 Videos/stickmen1.mp4 f

// Les caractères chinois et japonais ne s'effacent pas tous
