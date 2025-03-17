# Projet-Rust---PERNOT-GOULLIN
This is the repository for the Rust project (OUAP-4333E) of the pair PERNOT Thomas and GOULLIN Vianney

Here are the requisites before starting

# Setting Up Kitty with a Custom Matrix Theme

## Installation of Kitty

1. Install Kitty using your package manager:

   - **Debian/Ubuntu:** `sudo apt install kitty`
   - **Arch Linux:** `sudo pacman -S kitty`
   - **Fedora:** `sudo dnf install kitty`
   - **MacOS (Homebrew):** `brew install kitty`

2. Verify the installation:

   ```sh
   kitty --version
   ```

## Creating the Custom Matrix Theme

1. Navigate to the Kitty themes directory:

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

## Applying the Matrix Theme in Kitty

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

