# /sprite

## How it works
`Sprite` ...
- has optional `TextureAtlas`.

`TextureAtlas` ...
- holds sprite sheet
- from the original sprite image.

So make `animate_sprite` system ...
- measure some time and
- modify the index of sprite in `TextureAtlas` of `Sprite`.
