# Projet-Rust---PERNOT-GOULLIN
This is the repository for the Rust project (OUAP-4333E) of the pair Thomas PERNOT and Vianney GOULLIN.

Here are some requisites before starting :
----------------------------------------------------------------------------------------------------------------------------------------------------------------------

## Setting Up Kitty with a Custom Matrix Theme

### Installation of Kitty

1. Install Kitty using your package manager:

   - **Debian/Ubuntu:** `sudo apt install kitty`
   - **Arch Linux:** `sudo pacman -S kitty`
   - **Fedora:** `sudo dnf install kitty`
   - **MacOS (Homebrew):** `brew install kitty`

2. Verify the installation:

   ```sh
   kitty --version
   ```

### Creating the Custom Matrix Theme

1. Navigate to the Kitty themes directory at the following path:

   ```sh
   ~/.config/kitty/themes
   ```
   
   If the directory does not exist, create it : 

   ```sh
   mkdir -p ~/.config/kitty/themes
   ```

2. Create a new file named `matrix.conf` in `~/.config/kitty/themes/`:

   ```sh
   nano ~/.config/kitty/themes/matrix.conf
   ```

3. Add the following color scheme and save the file:

   ```ini
   color0  #000000
   color1  #00AA00
   color2  #00FF00
   color3  #80FF80
   color4  #00CC00
   color5  #00AA00
   color6  #00DD00
   color7  #00FF00
   color8  #009900
   color9  #00BB00
   color10 #00FF00
   color11 #80FF80
   color12 #00DD00
   color13 #00AA00
   color14 #00FF00
   color15 #A0FFA0
   background #000000
   foreground #00FF00
   cursor #00FF00
   ```

### Applying the Matrix Theme in Kitty

1. Open the Kitty configuration file:

   ```sh
   nano ~/.config/kitty/kitty.conf
   ```

2. Add the following line at the end of the file:

   ```ini
   include themes/matrix.conf
   ```

3. Save and exit.

4. Restart Kitty to apply the changes:

   ```sh
   kitty
   ```

## Installing Necessary Dependencies for the Video Display

   ```sh
   sudo apt install ffmpeg
   cargo add image crossterm term_size
   ```

----------------------------------------------------------------------------------------------------------------------------------------------------------------------

### Rust and cargo should also be installed on your computer

## Running the Program

1. Clone the project:
   
   ```sh
   git clone <https://github.com/RussianDoggo21/Projet-Rust---PERNOT-GOULLIN>
   cd <projet>
   ```

2. Compile and run the project with Cargo:
   
   ```sh
   cargo run
   ```

3. Switch to Kitty:

   ```sh
   kitty
   ```

4. Execute the compiled binary with one of the 2 following commands:
   
   ```sh
   ./target/debug/Projet 1 down latin 3      
   ```

   ```sh
   ./target/debug/Projet 2 stickmen1.mp4 f      
   ```
