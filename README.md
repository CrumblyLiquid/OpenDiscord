# OpenDiscord
 Control your game through Discord chat

## Info
 Pretty fast Discord bot made in [Rust](https://www.rust-lang.org/) with [serenity](https://github.com/serenity-rs/serenity). OpenDiscord bot listens for messages in any guilds/servers it's in and when it recognizes a command it will send a keystroke to your PC as if it came from your keyboard.

 Original idea: https://github.com/c4r1sk1m/OpenTwitchPlays

## Limitations
 - Limited configuration options
 - Can't send keystrokes to specific window
 - Doesn't recognize special keys such as Ctrl, Shift, Alt, etc.

These limitations might get resolved later. If you need them now create an issue.

## Setup
You can either build the binary yourself or download the latest zip one from [releases](https://github.com/CrumblyLiquid/OpenDiscord/releases). We'll focus on the case with the zip file.

 #### 1) Create a Discord bot
 You can skip this step if you already know how to setup a Discord bot

 1) Log in on the [Discord website](https://discord.com/)
 2) Go to the [applications page](https://discord.com/developers/applications)
 3) Click `New Application`
 4) Give the application a name and click `Create`
 5) Go to the `Bot` tab and click `Add Bot`
 6) You can get the bot `token` later by going to `Bot` tab and clicking `Copy`

#### 2) Invite bot account to your server
 1) Go to the `OAuth2` tab (we're starting on the same page as in `step 1.6`)
 2) In the `scopes` part select `bot`, click `Copy`, paste the URL into new tab, select server and confirm

 #### 3) Setup OpenDiscord

 1) Extract files from the zip into your desired directory
 2) Open the config.json and replace "TOKEN" with your own Discord bot token (refer to `step 1.6`) (don't forget to save the file)
 3) Open terminal and navigate to the directory you extracted the zip to
 4) Run `./open_discord.exe`
 5) If you see `<name of your bot> is connected` then everything has worked. If you don't then check your config.json for any mistakes. If the issues still persist, please refer to the `Issues` section below
 6) You can stop the bot by clicking into the terminal window and pressing `Ctrl+C`

## Issues
 If you encounter any bugs create an issue and I'll try to resolve the porblem.

## Contribution
 If you want to improve the bot feel free to make a pull request!

## Lincense
 [GNU GPLv3](LICENSE)