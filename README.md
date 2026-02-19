# A tool to help you make custom keybinds
> This was my first project written in rust!

# Usage

0. Open the program which *should* prompt you to pick a directory

1. Select the following folder on your system:

   ```
   Rockstar Games > GTA/Enhanced > Profiles
   ```
  > Usually located in your **My Documents** folder.

# Screenshots
<details>
<summary>
   Click me!
</summary>
   
<br>
   
Start Screen <br>
![](/assets/ss/landing.png)

Keybinding Page <br>
![](/assets/ss/keybindings.png)

New Keybind <br>
![](/assets/ss/new_keybind.png)

Modify Keybind <br>
![](/assets/ss/modify_keybind.png)

Unsaved Changes <br>
![](/assets/ss/unsaved_changes.png)

Mismatch <br>
![](/assets/ss/mismatch.png)

Info Screen <br>
![](/assets/ss/info.png)

</details>

# Useful Information

0. be sure to check out the wiki
   
1. To reload keybinds without restarting your game open go to
    ``` Pause > Settings > Keybinds ```
   open the keybinds menu and back out of it
> if you have "illegal" binds ( like unbinding all-chat ) accept the alert when backing out of the menu, it doesnt actually revert changes

2. A neat trick you can do if you are wondering what a certain ingame keybinds input value is, is to go into the keybinds menu ingame and restore to default and only edit the thing you are trying to get, then exit the settings menu and open user.xml in your profiles folder ```( Rockstar Games > GTA/Enhanced > Profiles > [profile] > Controls > user.xml )```, it will only contain one keybind, remember or copy the input code and go into the keyboard manager, press restore, then add a new keybind with that value and save

3. Currently i dont show the default keybinds so you will need to know how these keybinds will alter your ingame binds `(tldr; if you bind a key it will have its default binds removed ((i think)), for default binds check /controls/default_binds.txt)`

4. A profile refers to a rockstar games account so if you only have one there will probably only be one profile. <br>
Currently i don't know of a way to get the username from this folder so trial and error if you have multiple :)

5. Backups are automatically created every time you save a profile and are located at:
   ```
   Rockstar Games > GTA/Enhanced > Profiles > [profile] > Controls > user_[date].backup
   ```
   Currently the only way to restore it is to manually **rename** the backup to `user.xml`

6. Config files are stored at: <br>
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

# For the Future (in no particular order)
1. add **ctrl+z** *(undo)* and **ctrl+shift+z** *(redo)*
2. on screen keyboard with tooltips showing what each key is bound to, recolor non-default bound keys
3. implement localization using localization mappings (/controls/keyboard layout/)
4. on windows, make the icon show on the executable
5. make a button to easily pick and load from a .backup file
6. add keyboard navigation
7. ???
8. profit!

# Credits

[Google Material Icons](https://fonts.google.com/icons?icon.set=Material+Icons)
