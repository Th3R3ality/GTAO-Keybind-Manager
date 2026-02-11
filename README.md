# A tool to help you make custom keybinds
> This was my first project written in rust!

<br>
<br>

## Usage

0. Open the program which *should* prompt you to pick a directory

1. Select the following folder on your system:

   ```
   Rockstar Games > GTA/Enhanced > Profiles
   ```
  > Usually located in your **My Documents** folder.

<br>
<br>

## Useful information
1. Currently i dont show the default keybinds so you will need to know how these keybinds will alter your ingame binds (tldr; if you bind a key it will have its default binds removed ((i think)), for default binds check /controls/default_binds.txt )

2. A profile refers to a rockstar games account so if you only have one there will probably only be one profile. <br>
Currently i don't know of a way to get the username from this folder so trial and error if you have multiple :)

3. Backups are automatically created every time you save a profile and are located at:
   ```
   Rockstar Games > GTA/Enhanced > Profiles > [profile] > Controls > user_[date].backup
   ```
   Currently the only way to restore it is to manually **rename** the backup to `user.xml`

4. Config files are stored at: <br>
   ### Windows
   ```
   C:\Users\<user>\AppData\Local\.keybindmanager\config.txt
   ```
   ### Linux
   ```
   /home/<user>/.config/.keybindmanager/config.txt
   ```
   ### macOS
   ```
   /Users/<user>/Library/Application Support/.keybindmanager/config.txt
   ```