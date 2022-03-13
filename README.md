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
|      Ctrl + F | Enable or disable fps display.                                       |
|           Esc | Exits the game.                                                      |

## Screenshot
<p>
  <img src="https://github.com/gpietz/brickball/blob/master/docs/screenshots/brickball220111-01.png" 
       alt="Project Screenshot" />
</p>

## Credits 

* Sound Effects - [Juhani Junkala](https://juhanijunkala.com/)  
  [512 Sound Effects (8-bit style)](https://opengameart.org/content/512-sound-effects-8-bit-style)

## Planned changes (not yet done): 
* Enable fullscreen rendering
* Add textures for sprite elements
* Add sound effects
* Add title screen with menu

## Project update and notes

#### _2022-03-13_ Project Recap  
I haven't worked on the project for the last two weeks because, to be honest, I couldn't motivate myself. 
At the moment I see too many problems, many things take too long and the game engine seems more and more unsuitable.

I wanted to add the textures now but what I miss is the possibility to set properties for the texture repetition. 
This [discussion](https://github.com/bevyengine/bevy/issues/399) also deals with the problem, but it doesn't offer a good solution.

The realisation of a game menu also seemed too complex to me. The game engine certainly has many advantages and has 
many features in the 3D area, but for what I want to realise with it, it seems too complex to me.

At the moment I'm thinking about porting the game or starting something new, but in any case I'll use a simpler 
graphics engine.

#### _2022-02-19_
The project is progressing more slowly than was actually hoped. Last week I had no time to take care of the further 
development. Another problem is that I constantly have new ideas and from the original goal of a learning project the 
whole thing in my mind is developing more and more into a AAA title.

After I have reworked the collision detection last, now rather smaller extensions and improvements have been added. 
The FPS display I wanted to put in the window title, but had to realize that this has a negative impact on the 
performance. So changes to the window should not be done often.

#### _2022-01-29_
I've revised the main game routine. Now it's using <a href="https://bevy-cheatbook.github.io/programming/states.html">
bevy's states</a> which results into more options for the runtime "flow".<br/>  
But I dont feel certain about if it was a good idea to separate the ball movement from the collision detection.  
Maybe I've to combine this into one system.

#### _2022-01-26_
Finally I managed to get the audio playback working in the project. 
As suggested I'm using the <a href="https://github.com/niklasei/bevy_kira_audio">bevy_kira_audio</a> 
for it instead of the built-in audio replay routine. To make this working I had 
to change the features of the bevy library. For more information about it, you 
should take a look <a href="https://bevy-cheatbook.github.io/features/audio.html">here</a>.  
  
I'm not really happy with the current sounds, they doesn't seem to fit for the 
game. Maybe I'll find some better sounds in the next days. Actual I want spent
some time on the fullscreen display and there are still some silly bugs in 
the collision detection and ball movement system.
