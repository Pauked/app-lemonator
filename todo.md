# App principles

- Be fast to open apps.
- Be concise in output.
- Be easy to use.
- Give clear error messages.
- For slow processes, like finding apps, give clear feedback.

## TODO

[x] Push async code into db.rs
[/] Store File Version and File Description (can't read Windows file info easily)
[ ] Use saved File Version in search
[ ] Add method to update last run path periodically.
[/] MacOS file version checking!
[ ] Add abort to folder_search
[ ] Add abort to update "all"
[ ] Add GitHub action to build for Windows and MacOS and create releases
[x] Add arg for --always-update that checks app path, if not exists (like WhatsApp on a weekly basis), it attempts an update without asking
[ ] Show errors in a HTML page? Add arg --html-output.
[ ] Run a console app from a PowerShell script that keeps it open. New app type of console_pause (Windows only)
[ ] Add an app_settings table for various switches (like --check-app-path )
[ ] Add support for Dropbox folder on macOS
