# pincel [WIP]

![](demo/demo.gif)

## Running
```bash
cargo run
```

## Installing
```bash
cargo install --path .
```

## Using

```bash
pincel
```
## Controls

| Control                                              | Description                                                   |
|------------------------------------------------------|---------------------------------------------------------------|
| <kbd>q</kbd> or <kbd>ESC</kbd> or <kbd>Casplock</kbd>| Quit the application.                                         |
| Drag with left mouse button                          | Draw lines.                                                   |
| Click with right mouse button                        | Undo last drawing.                                            |
| Click with middle mouse button                       | Clear all draws.                                            |
| <kbd>p</kbd>                                         | PrintScreen.                                                  |
| <kbd>1</kbd>                                         | Change brush color to red.                                    |
| <kbd>2</kbd>                                         | Change brush color to blue.                                   |
| <kbd>3</kbd>                                         | Change brush color to yellow.                                 |
| <kbd>4</kbd>                                         | Change brush color to green.                                  |
| <kbd>5</kbd>                                         | Change brush color to orange.                                 |
| <kbd>6</kbd>                                         | Change brush color to black.                                  |

## Notes

To run this program it is necessary to have a composite manager like ![picom](https://github.com/yshui/picom) or xcompmgr running to make the window transparent, if you are using a desktop like Gnome or KDE you are probably already using it, but if you are using a window manager like i3, you have to run it before you start using pincel.


