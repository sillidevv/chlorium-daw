# + CHLORIUM +

> [!IMPORTANT]
> Chlorium is work in progress, there is no "stable" branch/version
> and we work directly on the main branch. As of right now, Chlorium
> isn't even a DAW, I'm yet to start working on the DAW things :P
> 
> Separate branches will and should be made but it's just that
> we dont have any form of stable branch.


`+ chlorium +` is a very simple tracker-ish DAW *(Digital Audio Workstation)* 
built fully in rust with the help of [egui](https://github.com/emilk/egui).

Since I started using [Renoise](https://www.renoise.com/), 
I've been really interested in trackers, and was inspired to try to
build my own. Trackers being (in concept) relatively simple than other 
types of DAWs, I decided to start making `+ chlorium +`

Don't expect anything good or serious... this started mostly out of
boredom.

### Contributing
While this is mostly a solo project, making a daw is really hard so 
contributions are heavily appreciated.

I wouldn't recommend contributing yet tho... this codebase is
genuinely awful and I haven't bothered to document it even in the
slightest

### TODO
**Ordererd after priority. This is mostly for myself.*
- Readme.

- Change chlorium_theme to fully use colors from 
  chlorium_theme::color_palette and remove all the Catppuccin 
  Mocha artifacts

- Make a `Window` trait with `draw(ctx: &egui::Context)`
  and `draw_content(ui: &mut egui::Ui)`

- Finish the "About Chlorium" window.

- Remake tracker channel system into a more grid based type of 
  a tracker interface

### Contact
The best way to contact me about `+ chlorium +` is through sending 
me a dm on Discord,

Discord username: `sillidev`

## MORE WILL BE ADDED TO THIS README AS THE PROJECT EXPANDS