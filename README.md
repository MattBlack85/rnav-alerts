# rnav-alerts
Set custom alerts for rnav spotting if you use an ADS-B receiver


# Install
run the following command

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/rnav-alerts/main/install.sh | sh
```

sudo will be needed as last step to move `rnav-alerts` to `/usr/local/bin`

# How to run
Simply type `rnav-alerts`, on the very first run (or every time you delete the config) the program will ask some questions to generate
a configuration

# TROUBLESHOOTING

- I get `command not found` error => try to open a new terminal and run the command, if that doesn't work make sure `/usr/local/bin` is in your `$PATH`
