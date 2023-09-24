# Test script to initialise the database with some apps and run various commands.
# Written for Windows. Should run without errors.

# Set top level variables
$lemonator_path = ".\target\debug\app-lemonator.exe"
$jetbrains_path = "%localappdata%\Programs"
$export_file = ".\scripts\test-export.json"

# Delete the db so we have a clean slate
& $lemonator_path reset --force

# Add individual apps
& $lemonator_path add OneNote onenoteim.exe "*onenote*" ps-get-app
& $lemonator_path add WinTerm wt.exe "*WindowsTerminal*" ps-get-app
& $lemonator_path add WhatsApp whatsapp.exe "*whatsappdesktop*" ps-get-app
& $lemonator_path add Rider rider64.exe $jetbrains_path folder-search
& $lemonator_path add DataGrip datagrip64.exe $jetbrains_path folder-search
& $lemonator_path add PHPStorm phpstorm64.exe $jetbrains_path folder-search
& $lemonator_path add CLion clion64.exe $jetbrains_path folder-search
& $lemonator_path add ProcExp procexp.exe "%personaldropbox%\Utils\SysinternalsSuite" shortcut
& $lemonator_path add GitKraken gitkraken.exe "%localappdata%\GitKraken" folder-search
& $lemonator_path add ch-pauk chrome.exe "%programfiles%\Google\Chrome\Application" shortcut --params " --args --profile-directory=Default"
& $lemonator_path add ch-p1 chrome.exe "%programfiles%\Google\Chrome\Application" shortcut --params " --args --profile-directory='Profile 1'"
& $lemonator_path add ztree conhost.exe "%windir%\System32" shortcut --params " %personaldropbox%\Utils\Ztree\ZTW.EXE"

# Duff apps to test the error handling
& $lemonator_path add Bob bob.exe %temp% folder-search
& $lemonator_path add Jeff onenoteim.exe "*jeffyson*" ps-get-app
& $lemonator_path add simon simon.exe "C:\Simon Files" shortcut
& $lemonator_path add dave dave.exe "%homepath%\dave files" shortcut

# Edit an app with different args
& $lemonator_path add Jeff2 onenoteim.exe "*jeffyson*" ps-get-app
& $lemonator_path edit Jeff2 --exe-name notjeff.exe
& $lemonator_path edit Jeff2 --search-term *notjeff*
& $lemonator_path edit Jeff2 --search-method shortcut
& $lemonator_path edit Jeff2 --search-method folder-search
& $lemonator_path edit Jeff2 --params " --invalid-params"
& $lemonator_path edit Jeff2 --params " --invalid-params=jeff --poop"
& $lemonator_path edit Jeff2 --app-name NotJeff
& $lemonator_path list NotJeff
& $lemonator_path delete NotJeff

# Check export to JSON file, reset database, import from JSON file
& $lemonator_path export $export_file --force
& $lemonator_path reset --force
& $lemonator_path import $export_file
# Remove-Item $export_file

# Update the app path for all apps in the database
& $lemonator_path update --force

# List all apps in the database in the different list formats
& $lemonator_path list --full
& $lemonator_path list

# Final export to JSON file
& $lemonator_path export $export_file --force
