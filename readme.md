# Autoxrandr

Autoxrandr is a small utility to automatically set xrandr settings based on saved monitor configurations.

## Installation

### Compile from source
> Prerequisites: You need to have cargo and rust installed
1. Clone this repository
2. `cargo install --path .`

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

## Roadmap
I might implement these things at some point

- [ ] Automatically detect which monitors are connected and apply profile
