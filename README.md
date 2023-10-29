# D-Bus DDC auto brightness

This software synchronizes laptop screen and external monitors brightness.

It detects brightness changes through D-Bus and applies a DDC command to set all connected monitors to this brightness level.

It currently only works on KDE, but could be adapted to work on any desktop environment that uses D-Bus.

## Launching automatically on login

```shell
cargo build --release
sudo cp target/release/auto_brightness /usr/bin/auto_brightness
cp auto_brightness.desktop ~/.config/autostart
```

Feel free to change the installation path, but make sure to reflect it in the .desktop file.