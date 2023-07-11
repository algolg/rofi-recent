# rofi-recent

rofi-recent is a script for rofi that provides a way for users to have quick access to recently-used files. The program works by parsing recently-used.xbel using a modified version of System76's [recently-used-xbel](https://github.com/pop-os/recently-used-xbel) Rust crate and outputing necessary information to rofi.

Once added as a rofi modi, rofi-recent can be used alongside drun for a convenient file-searching experience.

<details>
  <summary><b>Table of Contents</b></summary>
  <ol>
    <li><a href="#dependencies">Dependencies</a></li>
    <li><a href="#install">Install</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
  </ol>
</details>

<img src="https://user-images.githubusercontent.com/55261146/229308553-060b7f99-a667-49c8-b832-dde088e9e44f.png" width=75% height="auto">

## Dependencies

This project relies on:

- [rofi](https://github.com/davatorium/rofi)
- [rust](https://www.rust-lang.org/)
- [cargo](https://github.com/rust-lang/cargo)

## Install

1. Clone this repo
   ```sh
   git clone https://github.com/algolg/rofi-recent
   ```
   ```sh
   cd rofi-recent
   ```

2. Install
   ```sh
   cargo install --path .
   ```
   Now, all that's left is to add rofi-recent as a modi.

3. Add `recent:rofi-recent` to the modi section of your rofi config file:
   ```css
   configuration {
       modi: "combi,drun,recent:rofi-recent";
       /* ... */
     }
   ```
   By default, rofi-recent shows the 5 most recently-used files for each program. The optional `--limit <LIMIT>` (or `-l`) parameter can be used if you wish to change this limit. Setting the parameter to `0` will remove the limit entirely.

4. By adding rofi-recent as a combi modi, searching for a program will also show a list of files recently opened using the desired program (e.g. searching for GIMP will also show a list of files recently opened in GIMP), which may be more convenient for some users.

   This can be done by adding `recent` to the combi section of the rofi config file:
   ```css
   configuration {
       /* ... */
       combi-modi: "drun,recent";
       /* ... */
     }
   ```

5. (Optional) For a cleaner experience, I recommend adding this line to your rofi config file to remove the prefix for this modi:
   ```css
   display-recent: "";
   ```

## Usage

Now, rofi-recent can easily be called from the terminal:

```sh
rofi -show recent
```

If rofi-recent was added as a combi modi, the following command should also show rofi-recent:

```sh
rofi -show combi
```

## Contributing

If you want to help improve this program, feel free to fork the repo and make a pull request when you're ready. Contributions are highly appreciated ٩(◕‿◕｡)۶

