# Linux-CPU-RAM-notifier
# About
The Gnome Desktop environment is one of the most popular Linux Desktop environments. This program uses the Gnome libnotify library to alert the users of high CPU/RAM usage.
This can be helpful for security reasons (detecting malicious crypto miners) or simply to monitor the system. Notifications will appear like a standard Gnome notification. 

# How to Build
First install Linux dependencies to get all the Gnome development tools
```
sudo apt-get intall gnome-dev libnotify-dev
```
Then clone the repository. You will need Rust installed
```
git clone https://github.com/C-Loftus/Linux-CPU-RAM-notifier.git
cd Linux-CPU-Ram-notified
```
# How to Run
```
cargo run -c -r
```
The -c and -r specify to track CPU and RAM

# Status / Known Bugs
Currently this program is in process of development due to my internship taking priority. There is a known bug with a string trait implementation that causes creation of a notification box to fail in main.rs. That will be fixed soon.
