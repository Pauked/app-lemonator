# Welcome to App Lemonator

App Lemonator is a console app for quickly launching and managing your application shortcuts. Originally written to handle the hidden locations of Windows App Store apps and the difficulties of consistently opening them from an Elgato Stream Deck, it has since grown to be a general purpose app launcher.

**Think of it as a mini shortcut database. You give it details on an app and it will try its darndest to keep its app path up to date and launch it when you ask it to.**

It has some smarts around finding your apps and there are path shortcuts you can use to make it easier to define your apps.

## Installation

App Lemonator is a self contained executable that can be run from anywhere on Windows and macOS. It does not require any installation. On first run it will create a local Sqlite database to store any settings. It is entirely written in Rust because it's a fun thing to do in 2023!

You can download the latest release from the [releases page](https://github.com/Pauked/app-lemonator/releases) and then extract the ZIP or DMG file to a folder of your choice.

## Quick Start

It is a console app so you will need to open a Command Prompt, PowerShell Prompt or Terminal window to use it. You can then run it from the folder you extracted it to. It will create a local Sqlite database in the same folder to store any settings.

### Adding apps

To add apps, use the ``add`` command. For example:

```powershell
.\app-lemonator.exe add WinTerm wt.exe "*WindowsTerminal*" ps-get-app
.\app-lemonator.exe add Rider rider64.exe "%localappdata%\Programs" folder-search
.\app-lemonator.exe add ch-p1 chrome.exe "%programfiles%\Google\Chrome\Application" shortcut --params " --args --profile-directory='Profile 1'"
```

You can use the ``--params`` argument to pass additional parameters to the app when it is launched.

### Opening apps

To open an app, use the ``open`` command. For example:

```powershell
.\app-lemonator.exe open WinTerm
.\app-lemonator.exe open Rider
.\app-lemonator.exe open ch-p1
```

If you're using an [Elgato Stream Deck](https://www.elgato.com/uk/en/s/welcome-to-stream-deck), this is what you would set as your "App / File" within the Stream Deck software. You will need to set the icon yourself and [NirSoft IconsExtract](https://www.nirsoft.net/utils/iconsext.html) is good tool for extracting icons from executables.

_Note: On first open of an app it will go to find and set the app path. This can take a few seconds, especially if it doing a folder search across a large number of sub folders. On next run it will use the saved app path. If an app path no longer exists prior to opening, it will attempt to find the app again._

### Updating apps

To manually update the app path of an individual app, use the ``update`` command with the app name. For example:

```powershell
.\app-lemonator.exe update WinTerm
```

To update all apps in your database, don't pass any arguments to the ``update`` command. For example:

```powershell
.\app-lemonator.exe update
```

### Deleting apps

To delete an individual app, use the ``delete`` command with the app name. For example:

```powershell
.\app-lemonator.exe delete WinTerm
```

### Listing apps

If you want to see what apps are configured, use the ``list`` command. You can use ``--full`` to get a complete output. For example:

```powershell
.\app-lemonator.exe list
```

You would get output like this:

```powershell
┌──────────┬────────────────────────────────────────────────────┬─────────────────────┬─────────────────────┐
│ App Name │ App Path                                           │ Last Opened         │ Last Updated        │
├──────────┼────────────────────────────────────────────────────┼─────────────────────┼─────────────────────┤
│ Rider    │ C:\Users\Jeff\AppData\Local\Programs\Rider\bin\rid │ 2023-10-18 14:05:56 │ 2023-09-08 12:15:20 │
│          │ er64.exe                                           │                     │                     │
├──────────┼────────────────────────────────────────────────────┼─────────────────────┼─────────────────────┤
│ WinTerm  │ C:\Program Files\WindowsApps\Microsoft.WindowsTerm │ 2023-10-15 10:12:32 │ 2023-09-05 09:05:47 │
│          │ inal_1.18.2822.0_x64__8wekyb3d8bbwe\wt.exe         │                     │                     │
├──────────┼────────────────────────────────────────────────────┼─────────────────────┼─────────────────────┤
│ ch-p1    │ C:\Program                                         │ 2023-10-16 11:23:48 │ 2023-08-29 13:24:49 │
│          │ Files\Google\Chrome\Application\chrome.exe         │                     │                     │
└──────────┴────────────────────────────────────────────────────┴─────────────────────┴─────────────────────┘
```

### Further Help

For further details, use ``--help`` to get a list of the available agruments. A more detailed example is listed in the [Windows PowerShell test script](scripts/test_windows.ps1).

## Search Methods

App Lemonator allows you to select different search methods for your apps:

- ``ps-get-app`` - Uses PowerShell to search for apps installed from the Windows App Store.
- ``folder-search`` - Searches a folder for an app. You give it a base folder and it will recursively search and use the highest version number of the executable.
- ``shortcut`` - Uses a shortcut to launch an app. You give it the folder the app is in and it will launch it.

## Path Shortcuts

App Lemonator supports various path shortcuts:

- ``%appdata%`` - The current user's (Roaming) AppData folder
- ``%localappdata%`` - The current user's Local AppData folder
- ``%homepath%`` - The current user's Home folder
- ``%programfiles%`` - The Program Files folder
- ``%programfilesx86%`` - The Program Files (x86) folder
- ``%temp%`` - The current user's Temp folder
- ``%windir%`` - The Windows folder

_The above shortcuts are only available on Windows. You can type them into Explorer to check what there are._

App Lemonator also has the ability to work out where your Dropbox folder is and use that as a shortcut. This is useful for apps that store their settings in Dropbox.

- ``%personaldropbox%`` - The current user's Personal Dropbox folder
- ``%businessdropbox%`` - The current user's Business Dropbox folder

_The above shortcuts are only available on Windows. It will look for your Dropbox\info.json file and parse out the correct folder._

## Logging

If you want to see what App Lemonator is doing under the hood, open ``%temp%\app-lemonator.log`` in a text editor of your choice on Windows. The same log file will be in your temp folder on macOS (you can use ``echo $TMPDIR`` in a Terminal window to find it).

## macOS support

Whilst the app was primarily written for Windows it also works on macOS. The app acts more as a simple shortcut manager on macOS support the ``folder-search`` and ``shortcut`` options. Features related to grepping PowerShell output from the Windows App Store and Windows environment path shortcuts are not relevant on macOS.

If there is a need for more macOS specific features, please raise an issue and I'll see what I can do.