# SpringKeys - Typing Performance Visualizer

A terminal-based typing tutor that provides real-time feedback on typing performance with visual keyboard heatmaps and finger performance metrics.

## Features

### Keyboard Heatmap
- Real-time visualization of typing speed per key
- Color-coded performance indicators
- Multi-line key display showing:
  - Key character (centered)
  - Average speed
  - Last keystroke delay

### Finger Performance Chart
Displays detailed metrics for each finger (excluding thumb) with color-coded speed indicators:

| Color  | Speed Range | Performance Level |
|--------|-------------|------------------|
| Green  | < 80ms      | Ultra fast       |
| Blue   | 80-120ms    | Very fast        |
| Yellow | 120-200ms   | Medium           |
| Red    | 200-350ms   | Slow            |
| Magenta| > 350ms     | Needs practice   |

Metrics shown per finger:
- Current speed
- 10-second average
- 60-second average
- Fastest time
- Slowest time

## Building from Source

### Prerequisites
- Rust toolchain (1.70.0 or newer)
- Cargo package manager
- Terminal with ANSI color support

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/spring-keys.git
   cd spring-keys
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run in development mode:
   ```bash
   cargo run
   ```

4. Build for release:
   ```bash
   cargo build --release
   ```

The release binary will be available in `target/release/spring-keys`

## Usage

1. Launch the application:
   ```bash
   ./target/release/spring-keys
   ```

2. Start typing to see real-time performance metrics
3. Watch the keyboard heatmap and finger performance chart update in real-time
4. Press `Ctrl+C` to exit

## Development Notes

- The keyboard visualization uses a 3-row height for each key
- Keys are 9 characters wide for better readability
- Performance metrics are updated in real-time
- Color coding is consistent across all visualizations

## Command Line Interface
```
Usage: spring-keys [OPTIONS] [COMMAND]

Options:
  -h, --help        Print help information
  -v, --version     Print version information
  -d, --difficulty  Set difficulty level (easy|medium|hard)
  -q, --quiet       Suppress non-error output

Commands:
  practice    Start typing practice (default)
  game       Start a typing mini-game
  stats      View typing statistics
  config     Edit configuration
  test       Run VGA test screen
```

## Mini Games
A series of mini games to give typing entertainment:
- Minesweeper: Type words to make choices
- Tetris: Type words for position
- Flappy Bird: Type letters at the correct rate
- RC Championship: Steer using words
- Fortune Teller: Zoltar-style dialogue choices
- Hockey Fighting: Type to dodge and counter

## Main Feature
Typing tutor program with heatmap and quickness statstics for trendlines and plotting.

### Fun Terminology classifications peronas and titles.
Helpful personas, type while being mindfull in the style of the hero...
  - Dr. Espacio Alivio, down to earth Space Relief and calming advise for a moment getaway
  - Sr. Fuego Dedos, arriba andale fasting typing
   - Dr. Clavia Enter, a holistic thought healer to help you write your story.
- Kiimaster): "Key master" in Japanese, but in English emphasizes expertise and control.
- Input Tornado - (Nyūryoku Tatsumaki): "Input tornado" in Japanese, but in English suggests whirlwind typing speed.
- Uchimagami): "Typing demon" in Japanese, but in English sounds like "Ouch! A Magical Mine," offering a humorous juxtaposition.
- Karauchi Boshi): "Air-typing star" in Japanese, but in English sounds like "Karaoke Boss," highlighting performance and rhythm.
- Coach Bounce-Back (Image: A coach rebounding a ball high into the air, symbolizing resilience)
- Coach Take-a-Breaker (Image: A coach holding up a "timeout" sign, promoting balance and rest)
- Coach Break-the-Mold (Image: A coach smashing a mold with a hammer, encouraging players to embrace their individuality)
- Sir Keysalot: (Image: A knight in shining armor, valiantly pounding out messages on a telegraph key) A play on Sir Lancelot but with "keys" replacing the lance, showcasing dexterity and precision.
-  Baron Dashwood: (Image: A distinguished gentleman with a handlebar mustache, dashing off messages with rapid-fire keystrokes) A play on "dash," a telegraph punctuation mark, and "wood" for the telegraph poles, highlighting speed and connection.
- The Dotty Duchess: (Image: A wise and witty matriarch with a monocle, deciphering messages with discerning dot-and-dash precision) Capitalizes on the "dot" and "dash" elements of Morse code in a regal context.
- Captain Quirk: (Image: A seasoned sailor with a peg leg and a salty tongue, adept at unraveling coded messages) Adds a nautical theme and emphasizes linguistic agility.
- Captain Typebeard: A classic pirate with a fearsome beard and even faster fingers.
- Commodore Keysmash: A fiery, impulsive leader who leaves a trail of typos in his wake.
- Admiral Autocorrect: A meticulous captain who ensures every message is ship-shape and error-free.
- Salty Fingers Flynn: A seasoned veteran with nimble fingers and a knack for telling tall tales (and typing them even faster).
 -Peg-Leg Punster: A resourceful pirate who weaves puns into every sentence, leaving their crew in stitches.
 -Bosun Backspace: A meticulous editor who polishes every sentence until it shines like gold.
 -Parrot Proofread: A loyal companion who squawks out typos and grammatical errors before they can set sail.
 -The Kraken of Caps Lock: A legendary creature who haunts the high seas, typing in all caps wherever it goes.



### Sub arcade game
Consider a mine sweeper field, but not one mine per character, rather. One mine per word. Have a _minefield_ of words that are similar and easy typos of each other. Its a mine field, hard to type, muscle memory like tongue twisters help one articulate.


The below is a diagram of the mine of "doggy" being exposed with no mines at location of B3. The 5 indicates that 5 neighbor mines exist in this seed example. 
Minefield ascii art
```md
# Hidden Mine
~ Exposed
? unknown amount of mine.
####################################################
###       #####        ########        #############
###   ?   #####   ?    ########   ?    #############
### twerpy##### tappy  ######## flippy #############
###       #####        ########        #############
####################################################
####################################################
###       #####        ########        #############
###   ?   #####    ?   ########    ?   #############
### saggy ##### piggy  ########  boggy #############
###       #####        ########        #############
####################################################
##############~~~~~~~~~~############################
###       ####~        ~#######        #############
###       ####~   ?    ~#######    ?   #############
### goddy ####~ doggy  ~#######  tabby #############
###       ####~        ~#######        #############
##############~~~~~~~~~~############################
###      

```
Take a first guess, type `doggy`.

```md
# Hidden Mine
~ Exposed
? unkonwn amount of mine.
####################################################
###       #####        ########        #############
###   ?   #####   ?    ########   ?    #############
### twerpy##### tappy  ######## flippy #############
###       #####        ########        #############
####################################################
####################################################
###       #####        ########        #############
###   ?   #####    ?   ########    ?   #############
### saggy ##### piggy  ########  boggy #############
###       #####        ########        #############
####################################################
##############~~~~~~~~~~############################
###       ####~        ~#######        #############
###       ####~    5   ~#######    ?   #############
### goddy ####~        ~#######  tabby #############
###       ####~        ~#######        #############
##############~~~~~~~~~~############################
###      

```

At this point a 5 is revealed. Because 5 neighbors are mines.  
A smart play here is to make a random choice on the top row. 
But, we could earn points by flagging the words as mines.  because it is a typing program.... :)_  By default, hijack the caps-lock to not shift the register.  Not that you type them in all caps. But that the typing was done with Caps-lock as your toggle. Its a mine field, that is your trigger. The light on the LED means FLAG mode. (In vim, i hear legends of turning the caps into escape key).

User types tappy because of most data, smartest move. If you choose edge, you get only one mine data. middle would have both.


```md
# Hidden Mine
~ Exposed
? unknown amount of mine.
##############~~~~~~~~~~############################
###       ####~        ~#######        #############
###   ?   ####~   2    ~#######   ?    #############
### twerpy####~        ~####### flippy #############
###       ####~        ~#######        #############
##############~~~~~~~~~~############################
####################################################
###       #####        ########        #############
###   ?   #####    ?   ########    ?   #############
### saggy ##### piggy  ########  boggy #############
###       #####        ########        #############
####################################################
##############~~~~~~~~~~############################
###       ####~        ~#######        #############
###       ####~    5   ~#######    ?   #############
### goddy ####~        ~#######  tabby #############
###       ####~        ~#######        #############
##############~~~~~~~~~~############################
###      

```

Yay, unshown animation of mines being exposed. Because all others were mines. Good choices!!!

The points of correctly flagged mines are counted.

