# Historique des commits

## First commit : "Version 0.1 : nothing is done yet"

The basic idea to begin is to be able to make a character fall down from the top of the terminal to its end. For that, various knowledge was needed: how to access a specific position in the terminal, print and erase the character in a loop at this specific position in order to create this effect of "falling down". 

To achieve this, ChatGPT gave us the first push, providing useful Rust libraries such as `crossterm` to manipulate the terminal and `std::io` to gain access to the cursor and print characters.  
Afterwards, we modified the example function to make a character fall not only down but also up, left, and right.

We also created a `README.md` to explain the necessary setup of the terminal *kitty* to render the matrix effect. The whole process of setting up *kitty* was explained by ChatGPT.

---

## Second commit : "Print string + thread + Mutex : ne fonctionne pas correctement"

We centralized the four functions `print_char_left`, `print_char_right`, `print_char_up`, and `print_char_down` into a single function `print_char`, taking into account a direction given by the user.  
To handle errors, we created an enumeration `Direction` and implemented the `FromStr` trait for it.

We then created the function `print_string`, which calls `print_char` using multiple threads to obtain our first "drop" of digital rain.  

However, this approach had a major flaw: we were already using multiple threads to display a single string, while a proper digital rain effect should display **dozens of strings simultaneously**.  
Using threads for each individual drop would be **too costly for the processor**. Furthermore, the terminal's output behavior was difficult to control, even when using `Mutex`.  
The final rendering was not satisfying.

---

## Third commit : "Print string OK, affichage saccadé"

Here, we tried to properly manage access to `stdout` across multiple functions using the `.lock()` method.  
We didn't modify the thread management in `print_string` yet. The function worked correctly, but the **display rate was jerky**.  
This issue was later resolved by adjusting the pause between each character's display and erasure.  

However, another problem arose: the displayed strings **did not exit** the terminal correctly.

---

## Fourth commit : "Print string fonctionne"

In practice, we were **calling `print_char` incorrectly** in the `print_string` function.  
We **removed** the sub-functions and directly integrated them into `print_string`, adding two `for` loops:  
- One for displaying each character  
- One for erasing each character  

The **position** of each character was manually calculated within `print_string`.  
With this, the function was completed.  
The implementation of this commit was mainly done with the help of ChatGPT.

---

## Fifth commit : "Effet sortie d'écran OK"

To make strings **fade out** of the terminal screen, we had to check if their next position was still **within the terminal dimensions**.  

For this, we created the boolean `visible`, which determines whether a character should be displayed/erased based on its position.  
While implementing this, we encountered **negative x or y coordinates**, which led to compilation errors.  
ChatGPT helped us resolve this issue using the `saturating_sub` function.

---

## Sixth commit : "Génération de random string implémentée avec différents alphabets"

We implemented different **alphabets** (`latin`, `greek`, `japanese`) for generating random strings.  

To do this, we:  
- Created an enumeration `Alphabet` to store these alphabets  
- Implemented the `FromStr` trait for `Alphabet` with the help of ChatGPT  
- Implemented the `random_string` function to generate a string of random length using a chosen alphabet

---

## Seventh commit : "Effet pluie digitale"

We now implemented the `print_string` function **inside a loop** to generate the full **digital rain effect**.  

To **increase** the number of falling strings, we added **four threads** to call `print_string` simultaneously.

---

## Eighth commit : "Arguments donnés en ligne de commande"

We added the ability for the user to provide parameters for the program:  
- `direction`  
- `alphabet`  
- (Later, **duration** of the digital rain)

To handle this, we used `std::env` (recommended by ChatGPT) to retrieve command-line arguments, then **validated them** using the `FromStr` trait previously implemented in both `Direction` and `Alphabet`.

---

## Ninth commit : "Suppression des unwrap()"

Since our professor kindly explained that he **did not** want to see **any `unwrap()` in the final code**,  
we **replaced all `unwrap()` calls** with proper **pattern matching** to handle potential errors gracefully.

The only `unwrap()` we kept was:
```rust
handle.join().unwrap();
```
because it followed the example given in our course.

---

## Tenth commit : "Added a binary image to terminal converter. Video is the next step"

To make the project more **interesting and challenging**, we had the idea to add an animation overlay on the digital rain. 
The objective was to be able to take any grayscale video in input and display it on the terminal (**in black and white only**), then to only keep the digital droplets where the white elements are, for every frame.
The first step was to create a program capable of displaying a single frame, a jpg  file in our case.

This part of the code was added as a comment below the program of the digital rain.

---

## Eleventh commit : Features a grayscale video display in terminal

While displaying a single image turned out quite easy, more things had to be taken into account for a whole video.
Since it does not appear to be possible to display videos straight from their MP4 files, they have to be broken down into multiple frames in a 'frames' folder.

Secondly, only videos natively black and white, with no gray values (unlike old  movies for example) turned out good. `spiral.mp4` is a good example.

Unfortunately, a few `unwrap()` calls were added again and were not deleted at this point.

This part of the code was added as a comment at the end of the program, replacing the previous version.

---

## Twelfth commit : Minor changes and failed fix of previous version of video display

The rendering from the previous version was quite satisfying.

A few changes were made:
- Two videos were added. All credits are given in the 'VideoCredits' file.
- The framerate was changed to 80ms, closer to the original videos.
- 'badapple.jpg' was deleted as this part of the code now only deals with video.

Now, with the idea of merging this terminal video display with the digital rain part, one major problem had to be addressed: the terminal displays each frame without deleting the previous one. This is not compatible with the current version of digital rain, which lies immediately below the command line. It turned out that the cursor had to be reset to the top-left corner after every frame, using
```rust
let mut stdout = stdout();
```
and
```rust
 stdout.flush().unwrap();
```
Even then, the first row of every frame was kept when the history of previous frames before interruption was now expected to not be visible.

A fix to this problem has not been found so far. As such, the project now consists of these two independent "mini-projects". As it is not possible to include another 'main.rs' file in the 'src' folder, the user has to manually comment/uncomment the corresponding parts (cf. README).

---

## First commit post-evaluation : Add of the argument to call either one of our 2 'main' functions

We added another argument called `type_main` to determine whether the user wanted to use the programm that displayed the digital rain or the video on the terminal.

---

## Second and third commits post-evaluation : Refactorisation of the code

Creation of two functions `main1` (digital rain) and `main2` (video) to be called by the "real" main function.
With the help of Chat GPT, creation of the function `parse_args` to check the arguments given by the user and deal with the possible errors that may arise.
Modification of some rough draft in the syntax of the code.

---

## Fourth commit post-evaluation : Update of the README.me and log.md

We also created a folder `Videos` for a clear structure of the GitHub deposit.

---

## Fifth commit post-evaluation : A few extra upgrades and updates

We got rid of all the `unwrap` calls and turned them into `expect` for an easier debugging.
Some extra crates that are now imported in the program were removed from the README. Also removed unused crates in the program.
The 'frames' folder that was mistakenly imported in the last commits was deleted, as it is automatically created/replaced at execution.