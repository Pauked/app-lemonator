#!/bin/bash
# To run the script in a terminal, type ./test_macos.sh
#
# If not runnable/permission denied, check permissions.
# Use "chmod 755 test_macos.sh" to make executable.
#
# Test script to initialise the database with some profiles and run various commands.
# Written for macOS. Should run without errors.

# Set top-level variables
lemonator_path="./target/debug/app-lemonator"
export_file="./scripts/test-export.json"

# Delete the db so we have a clean slate
"$lemonator_path" reset --force

# Add individual apps
"$lemonator_path" add dcli dcli "%personaldropbox%" folder-search
"$lemonator_path" add ch-pauk "Google Chrome.app" "~/Applications" shortcut --params " --args --profile-directory=Default"
"$lemonator_path" add ch-p1 "Google Chrome.app" "~/Applications" shortcut --params " --args --profile-directory='Profile 1'"

# Check export to JSON file, reset database, import from JSON file
"$lemonator_path" export "$export_file" --force
"$lemonator_path" reset --force
"$lemonator_path" import "$export_file"
# rm "$export_file"

# Update the app path for all apps in the database
"$lemonator_path" update --force

# List all apps in the database in different list formats
"$lemonator_path" list --full
"$lemonator_path" list

# Final export to JSON file
"$lemonator_path" export "$export_file" --force
