# SpringKeys
Typing Tutor program using Hooks law for positioning chaos. 
(DEVELOPMENT PROTOTYPE) 

## Features
- Graphical Chaos Simulator.
- - Bad input leads to visual distractions and 'letter anger', where they get steamy mad and jumpy if you press them wrong.
- - Spring like behavior. For example a line return to scroll the window would make the top row have ballistic launch. A launch uses a long spring normal distance.
    

TODO A typing tutor experience unlike any other! This project aims to reinvent the terminal-based typing practice by introducing dynamic character movement, fluid animations, and engaging visuals.

Key Features
Characters with a Life of Their Own: Characters spring and drift within a network graph, creating a visually intriguing and interactive environment.
Physics-Driven Interactions: Explore a unique way to engage with text through physics-based character behavior.
Smooth Animations and Visual Feedback: Enjoy fluid transitions and subtle cues that enhance the typing experience.
Typing Test Essentials: Track your WPM (words per minute), accuracy, and other metrics to track your progress.
Rust at the Core: Built with Rust for performance, safety, and cross-platform compatibility.
Graphics Exploration: Experimenting with visual techniques (wasm3 or three.js) to push the boundaries of terminal-based interfaces.


## Sub arcade game
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

