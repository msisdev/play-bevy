# sprite_char


## Goal
Implement two states for player:
- Idle/Walk
- Facing Up/Left/Down/Right

Apply different sprite animation on each state.


## How it works
When arrow key is pressed,
- set Walk state
- set facing state

When arrow key is released,
- set Idle state

And map each state to the corresponding sprite index.
