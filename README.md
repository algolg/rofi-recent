# rofi-recent

rofi-recent is a script for rofi that provides a way for users to have quick access to recently-used files. The program works by parsing recently-used.xbel using a modified version of System76's [recently-used-xbel](https://github.com/pop-os/recently-used-xbel) Rust crate and outputing necessary information to rofi.

Once added as a rofi modi, rofi-recent can be used alongside drun for a convenient file-searching experience.

<details>
  <summary><b>Table of Contents</b></summary>
  <ol>
    <li><a href="#install">Install</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
  </ol>
</details>

<img src="https://user-images.githubusercontent.com/55261146/229308553-060b7f99-a667-49c8-b832-dde088e9e44f.png" width=75% height="auto">

## Install

1. Download the latest [release](https://github.com/algolg/rofi-recent/releases)

2. Extract the archive and make the program executable
   ```sh
   chmod +x rofi-recent
   ```

3. Install by copying the program to a location in `PATH` (run `echo $PATH` to list locations)
   
   Ex. this works if `~/.local/bin/` is in `PATH`:
   ```sh
   cp rofi-recent ~/.local/bin/
   ```
   Note: if you choose not to install rofi-recent, make sure to give rofi the path to rofi-recent in the next step

4. Add `recent:rofi-recent` to the modi section in `config.rasi`:
   ```css
   configuration {
       modi: "combi,drun,recent:rofi-recent";
       /* ... */
     }
   ```
   Note: if you did not install rofi-recent, add `recent:/path/to/rofi-recent` instead

   See [Arguments](#arguments) for options you can add to the command


5. By using rofi-recent with drun, searching for an application will also show a list of files recently opened in that application (ex. searching for GIMP will also show a list of files recently opened in GIMP, as in the image above)

   This can be done by adding `drun` and `recent` to the combi section in `config.rasi`:
   ```css
   configuration {
       /* ... */
       combi-modi: "drun,recent";
       /* ... */
     }
   ```

6. For a cleaner experience, I recommend adding this line to `config.rasi` to remove the prefix for rofi-recent:
   ```css
   display-recent: "";
   ```

## Usage

Now, rofi-recent can easily be called from the terminal:

```sh
rofi -show recent
```

If rofi-recent was added as a combi modi, the following command should also work:

```sh
rofi -show combi
```

### Arguments

You can optionally add any of these arguments in `config.rasi`

| Argument | Accepts a value? | Purpose |
| -------------------------- | --- | -------- |
| `-l` or `--limit`          | yes | Specify the max number of recent files to list per program. Set to `0` to disable the limit. Default limit is 5 files per program. |
| `-e` or `--exclude`        | yes | Specify programs to exclude from output. Take the program names word-for-word from rofi-recent's output. If excluding multiple programs, encase in quotes with a space between each program (ex. `rofi-recent -e "gimp firefox"`). Remember to escape quotes if needed. |
| `-s` or `--show-all-paths` | no  | Shows full path for all files |

Ex. `config.rasi`, using all arguments:
```css
configuration {
      modi: "combi,drun,recent:rofi-recent -l 0 -e \"gimp firefox\" -s";
      /* ... */
   }
```

## Contributing

If you want to help improve this program, feel free to fork the repo and make a pull request when you're ready. Contributions are highly appreciated ٩(◕‿◕｡)۶

