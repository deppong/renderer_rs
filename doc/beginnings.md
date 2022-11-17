# First Impressions

## Why Rust?
I would consider myself pretty new to rust, but overall I have been really enjoying the language.
I think that it's fast and the tooling around it is just exceptional to getting something up and running compared 
to wrangling CMake and getting all of the libraries working properly with C++.

I've made plenty of projects in the past with C++, and I'm just dipping my toes in the Rust world. I find it very cozy to program
in Rust.

## sdlrs_template

I am using a little template that I made to just get things going in sdl. If you want to follow along with any of the code feel free to look
at any of the commits as I document my experience. Effectively what's happening is I have a 2d texture stored in memory called framebuffer which every
frame is then drawn to the screen with a single draw call. All actual editing of that framebuffer is done with a vector of u8 bytes containing the rgb values.
That framedata is then copied to the framebuffer once per frame. I chose this only because I think it will probably be faster than calling a draw call for every
pixel I would like to place on the frame.

To start I need some basic 3d primitives and will start with some basic structs containing that information.
