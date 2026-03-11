# /pixel_grid_snap

https://github.com/bevyengine/bevy/blob/latest/examples/2d/pixel_grid_snap.rs

## How it works

### PIXEL_PERFECT_LAYERS
has two objects:
- bevy_pixel_dark.png
- `Capsule2d` mesh

### InnerCamera
Renders
- `PIXEL_PERFECT_LAYERS` objects
- onto a tiny canvas (320x180)

### HIGH_RES_LAYERS
has two objects:
- a canvas
- bevy_pixel_light.png

### OuterCamera
Renders
- `HIGH_RES_LAYERS` object
- onto screen.
