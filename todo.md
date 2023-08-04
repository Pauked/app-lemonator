App principles / Defense of Design
- Be fast to open apps.
- For slow processes, like finding apps, give clear feedback.
- Be concise in output.
- Be easy to use.
- Give clear error messages.

Todo list:
TODO: Exit codes when app fails to run or find an app to run it. Check Stream Deck can pick that up! (https://crates.io/crates/proc-exit/1.0.1)
TODO: Add method to update last run path periodically (or after each run?).
TODO: Add an admin drive search option
TODO: Add short and long args (i.e. --list -l)
TODO: MacOS file version checking!
TODO: Export to JSON
FIXME: Error handling on ps-get-apps and folder-search not finding anything.
TODO: Unit test Dropbox folder for empty folder sections for personal and business.
TODO: Improve add so it checks for app existing before attempting add
