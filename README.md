# Brickball
A breakball variant to get me more into the Bevy framework.

This is a learning project to get more aknowledge with the [bevy game engine](https://bevyengine.org/). It is neither feature complete or playbable until now.

I started by display and moving the player paddel. After that I added the ball and walls. Now I added extended keyboard control over the elements. So you can use the following keys to control the game: 

<table>
<tr>
<td>Cursor keys</td>
<td>Controls the player paddle (or the ball).</td>
</tr>
<tr>
<td>Space bar</td>
<td>Unlocks the ball from paddle (game starts).</td>
</tr>
<tr>
<td>F1</td>
<td>Toggles between paddle or ball control.</td>
</tr>
<tr>
<td>F2</td>
<td>Prints the ball and paddle coordinates.</td>
</tr>
<tr>
<td>- / +</td>
<td>changes the ball speed.</td>
</tr>
<tr>
<td>Shift & - / +</td>
<td>Activated next or previous level.
<tr>
<td>Shift & C</td>
<td>Centers the ball on the field (only when ball control is activated).</td>
</tr>
<tr>
<td>R</td>
<td>Resets the ball; locks it again on the paddle.</td>
</tr>
<tr>
<td>Esc</td>
<td>Exits the game.</td>
</tr>
</table>

## Screenshot
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

<span style="text-decoration: underline">
2021-01-26
</span><br/>
Finally I managed to get the audio playback working in the project. 
As suggested I'm using the <a href="https://github.com/niklasei/bevy_kira_audio">bevy_kira_audio</a> 
for it instead of the built-in audio replay routine. To make this working I had 
to change the features of the bevy library. For more information about it, you 
should take a look <a href="https://bevy-cheatbook.github.io/features/audio.html">here</a>.

I'm not really happy with the current sounds, they doesn't seem to fit for the 
game. Maybe I'll find some better sounds in the next days. Actual I want spent
some time on the fullscreen display and there are still some silly bugs in 
the collision detection and ball movement system.
