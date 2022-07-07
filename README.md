# rofi-recent

rofi-recent is a script for rofi that provides a way for users to have quick access to recently-used files. The program works by parsing recently-used.xbel using a modified version of System76's [recently-used-xbel](https://github.com/pop-os/recently-used-xbel) Rust package and outputing necessary information to rofi.

Once added as a rofi modi, rofi-recent can be used alongside drun for a convenient file-searching experience.

## Dependencies

This project relies on:

- [rofi](https://github.com/davatorium/rofi)
- [rust](https://www.rust-lang.org/)
- [cargo](https://github.com/rust-lang/cargo)
- [make](https://www.gnu.org/software/make/)

## Install

1. Clone this repo
```
git clone https://github.com/algolg/rofi-recent
```
```
cd rofi-recent
```

2. Build and Install
```
make all
```
This will build and install the program to ~/.local/bin/rofi-recent. Now, all that's left is to direct rofi to this file.

3. Add `recent:~/.local/bin/rofi-recent` to the modi section of your rofi config file, e.g.
```css
configuration {
    modi: "combi,drun,recent:~/.local/bin/rofi-recent";
    /* ... */
  }
```

4. (Optional) By adding rofi-recent as a combi modi, searching for a program will also show a list of files recently opened using the desired program (e.g. searching for GIMP will also show a list of files recently opened in GIMP), which may be more convenient for some users. This can be done by adding `recent` to the combi section of the rofi config file, e.g.
```css
configuration {
    /* ... */
    combi-modi: "drun,recent";
    /* ... */
  }
```

5. (Optional) For a cleaner experience, it is recommended that users add the following line to their rofi config files in order to remove the prefix for this modi:
```css
display-recent: "";
```

## Usage

Now, rofi-recent can easily be called from the terminal:

```
rofi -show recent
```

If rofi-recent was added as a combi modi, the following command should also show rofi-recent:

```
rofi -show combi
```

## Contributing

If you want to help improve this program, feel free to fork the repo and make a pull request when you're ready. Contributions are highly appreciated ٩(◕‿◕｡)۶

