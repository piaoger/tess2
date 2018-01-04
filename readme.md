# tess2

This is a wrapper around libtess2, written by
[memononen](https://github.com/memononen/libtess2). It exposes the unsafe c
api from bindgen and a safe wrapper for fill tessellations.

In general you should instead use [lyon](https://github.com/nical/lyon),
which is implemented in rust and faster in the majority of cases.

I created this because I have some very odd polygons with thousands of vertices
and self intersections which sometimes trips up lyon; check on
[issue 291](https://github.com/nical/lyon/issues/291) to see if this is still
the case before substituting libtess2.

# Building on Linux

Make sure you have `libglfw3-dev` and `libglew-dev`, then cargo should work as
expected.