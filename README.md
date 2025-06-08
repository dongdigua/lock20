# lock20
Lock your screen for 20s every 20min

## Usage
just run `lock20`

to skip once, send a SIGUSR1: `pkill -10 lock20`

### Caveats
When locked, killing the app may cause a "red screen"

## Package
s∞n

## Why
Inspired by [waycrate/twenty](https://github.com/waycrate/twenty), which uses tons of dependencies (~325)
and some [weird hack](https://github.com/waycrate/twenty/blob/7723178ce0d2e09c5c60d8d9e9d0869ba5889359/src/main.rs#L72)

So I want to build one with fewer deps (<110) and (probably) [some fun features](#TODO).

## TODO
- [ ] CraftMine-like text shaking
