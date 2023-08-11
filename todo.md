App principles / Defense of Design
- Be fast to open apps.
- For slow processes, like finding apps, give clear feedback.
- Be concise in output.
- Be easy to use.
- Give clear error messages.

Todo list:
TODO: Exit codes when app fails to run or find an app to run it. Check Stream Deck can pick that up! (https://crates.io/crates/proc-exit/1.0.1)
TODO: Add method to update last run path periodically (or after each run?).
TODO: MacOS file version checking!
TODO: Add an --edit mode that can allow individual properties to be amended. Thinking of JetBrains tools changing their base path!
TODO: Refactor to use eyre / proper Rust error handling

FIXME: How to handle app update? Like WhatsApp
Before - C:\Program Files\WindowsApps\5319275A.WhatsAppDesktop_2.2330.7.0_x64__cv1g1gvanyjgm\whatsapp.exe
After - C:\Program Files\WindowsApps\5319275A.WhatsAppDesktop_2.2331.4.0_x64__cv1g1gvanyjgm\whatsapp.exe