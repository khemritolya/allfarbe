# allfarbe
![Rust](https://github.com/khemritolya/allfarbe/workflows/Rust/badge.svg) 

A Rust Desktop Background Project.

Currently, it creates a window which looks somewhat likes this, except animated.
![An image](https://github.com/khemritolya/allfarbe/blob/master/.github/previews/allfarbe_preview.png)

CPU usage is around 0-1% on my machine, which is pretty good. GPU usage is a bit pesky, however. More research is required, but at least the power draw is equivalent to firefox while scrolling. Should run on Windows/Mac/Linux. Open an issue if you run into a problem.

### A Roadmap for the future

- Better shaders
  - Integration with [shadertoy](https://www.shadertoy.com)
  - More colors! Better images!
- Make the window actually behave like the screen background
  - A la ![desktopgalaxy](https://github.com/khemritolya/desktopgalaxy) would require X11 badness.
  - Might (?) break cross-platform nature, which we would like to maintain.
