# Brickball
A breakball variant to get me more into the Bevy framework.

This is a learning project to get more experience with the [bevy game engine](https://bevyengine.org/). 
It is neither feature complete or playbable until now.

I started by display and moving the player paddel. After that I added the ball and walls. 
Now I added extended keyboard control over the elements. So you can use the following keys to control the game: 


|          Keys | Description                                                          |
|--------------:|:---------------------------------------------------------------------|
|   Cursor keys | Controls the player paddle (or the ball).                            |
|     Space bar | Unlocks the ball from paddle (game starts).                          |
|            F1 | Toggles between paddle or ball control.                              |
|            F2 | Prints the ball and paddle coordinates.                              |
|            F3 | Toggles red test circle on / off.                                    |
|            F4 | Remove all bricks from current level.                                |
|            F5 | Reset all bricks in the current level.                               |
|            F6 | Toggle ball coordinates display on / off.                            |
 |            F7 | Toggles ball collision view on / off.                                |
|         - / + | Changes the ball speed.                                              | 
| Shift & - / + | Activates next or previous level.                                    |
|     Shift & C | Centers the ball on the field (only when ball control is activated). |
|             R | Resets the ball; locks it again on the paddle.                       |
|      Ctrl + S | Enable or disable sound.                                             |
|      Ctrl + M | Enable or disable music replay.                                      |
|           Esc | Exits the game.                                                      |

#@thr# Screenshot
<p>
  <img src="https://github.com/gpietz/brickball/blob/master/docs/screenshots/brickball220111-01.png" 
       alt="Project Screenshot" />
</p>

# Credits 

* Sound Effects - <a href="https://juhanijunkala.com/">Juhani Junkala</a>  
  <a href="https://opengameart.org/content/512-sound-effects-8-bit-style">512 Sound Effects (8-bit style)</a>

## Planned changes (not yet done): 
* Enable fullscreen rendering
* Add textures for sprite elements
* Add sound effects
* Add title screen with menu

## Project update and notes

#### _2021-01-29_
I've revised the main game routine. Now it's using <a href="https://bevy-cheatbook.github.io/programming/states.html">
bevy's states</a> which results into more options for the runtime "flow".<br/>  
But I dont feel certain about if it was a good idea to separate the ball movement from the collision detection.  
Maybe I've to combine this into one system.

#### _2021-01-26_
Finally I managed to get the audio playback working in the project. 
As suggested I'm using the <a href="https://github.com/niklasei/bevy_kira_audio">bevy_kira_audio</a> 
for it instead of the built-in audio replay routine. To make this working I had 
to change the features of the bevy library. For more information about it, you 
should take a look <a href="https://bevy-cheatbook.github.io/features/audio.html">here</a>.  
  
I'm not really happy with the current sounds, they doesn't seem to fit for the 
game. Maybe I'll find some better sounds in the next days. Actual I want spent
some time on the fullscreen display and there are still some silly bugs in 
the collision detection and ball movement system.
