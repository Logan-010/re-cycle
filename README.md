# (Re)cycle

A simple 2d game about recycling

Built for Bevy gamejam 5.

# Building

If you wish to build yourself, instead of using the binaries on the itch.io page or releases, follow these steps.

clone and open repo

```sh
git clone https://github.com/Logan-010/re-cycle
cd re-cycle
```

install Just via

```sh
cargo install --locked just
```

build native & web release (takes a while)

```sh
just build && just build-web
```

# Crates

Using [Avian 2d](https://github.com/Jondolf/avian/) for collisions & [Bevy](https://github.com/bevyengine/bevy/) for the game engine.
Also using [Edges](https://github.com/shnewto/edges) crate to generate colliders for the trash.

Used [Sfxr](https://sfxr.me/) for most of the sound effects

# Attribution
You are not required to attribute me if you re-use any of my code or assets*

*assets meaning anything under the assets folder that DOES NOT include the following files (due to me not having copyright ownership):
 - nothingon.ogg**
 - trash_can.png
 - font.otf
 - title_font.TTF
 - cursor.png

**nothingon.ogg is royalty free and can be reused, however requires the following attribution:
```
Music by Bensound.com/royalty-free-music
License code: DFSWP3QQ3VIFDGYR
```