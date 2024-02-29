# Autoxrandr

Autoxrandr is a small utility to automatically set xrandr settings based on saved monitor configurations.

## Usage

### Save profile
This command saves your current xrandr configuration (resolution, offset, refresh-rate and active monitors) to a profile.
```
autoxrandr save <profile_name>
```
### Apply profile
This command applies a previously saved profile.
```
autoxrandr apply <profile_name>
```

### List profiles
This command lists all saved profiles.
```
autoxrandr list
```

### Delete profile
This command deletes a previously saved profile.
```
autoxrandr remove <profile_name>
```
