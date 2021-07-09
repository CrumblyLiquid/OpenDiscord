# OpenDiscord
## Setup
You can either [build the binary yourself](BUILD.md) or download the latest open_discord.zip (includes open_discord.exe and config.json) from [releases](https://github.com/CrumblyLiquid/OpenDiscord/releases). Now that we have the binary let's set it up!

 ### 1) Create a Discord bot
 You can skip this step if you already know how to setup a Discord bot
 1) Log in on the [Discord website](https://discord.com/)
 2) Go to the [applications page](https://discord.com/developers/applications)
 3) Click `New Application`
 4) Give the application a name and click `Create`
 5) Go to the `Bot` tab and click `Add Bot`
 6) You can get the bot `token` later by going to `Bot` tab and clicking `Copy`

### 2) Invite bot account to your server
 1) Go to the `OAuth2` tab (we're starting on the same page as in `step 1.6`)
 2) In the `scopes` part select `bot`, click `Copy`, paste the URL into new tab, select server and confirm

 ### 3) Setup OpenDiscord
 1) Extract files from the zip into your desired directory
 2) Open the config.json and replace `TOKEN` with your own Discord bot token (refer to `step 1.6`) (don't forget to save the file). For further configuration read the [Configuration](#configuration) section.
 3) Open terminal and navigate to the directory you extracted the zip to
 4) Run `./open_discord.exe`
 5) If you see `<name of your bot> is connected` then everything has worked. If you don't then check your config.json for any mistakes. If the issues persist, feel free to create Issue here on GitHub.
 6) You can stop the bot by clicking into the terminal window and pressing `Ctrl+C`

## Configuration
 For now configuration is stored in `config.json` file structured as shown below and saved in directory we're executing from (usually the same directory as the binary).

 Notes:
 1) OpenDiscord doesn't support special keys like Shift, Ctrl, Alt, Esc, etc. (refer to issue #7)
 2) Setting roles will make the command processing slower. Not by much but when handling a lot of commands it might be noticeable. _Try it and see :D_
 ```
 {
    "token": "TOKEN", <- Your Discord bot token
    // Optional
    "guilds": [], <- List of server IDs
    "channels": [], <- List of channel IDs
    "roles": [], <- List of role IDs
    "duration": 100, <- How long to press the key in milliseconds
    "commands": { <- Dictionary of commands
        "walk": "w", <- Command is in format "command": "key"
                        Type `walk` into Discord and the bot will type `w` on your PC
        "forward": "w",
        "back": "s"
        "left": "a",
        "right": "d",
        "w": "w",
        "s": "s",
        "a": "a",
        "d": "d",
    }
 }
 ```