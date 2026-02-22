# click
this is a rust rewrite of tty-clock, because i wanted config files
i had 2 words on my mind, clock and tui, the best way i could fit "tui" in "clock" was just replacing o with ui, cluick didnt sound nice so i just made it click

## Features
- [x] - showing time, option for seconds
- [x] - centerize text
- [x] - 12h time format
- [x] - hide date
- [ ] - blinking colon
- [x] - configuration

## How to build
```
git clone https://github.com/segvyfault/click
cd click
cargo build -r
sudo cp target/release/click /usr/bin/click
```
