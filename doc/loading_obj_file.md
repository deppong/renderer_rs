# The Wavefront .obj file
- [wikipedia page](https://en.wikipedia.org/wiki/Wavefront_.obj_file)

In the obj_loader.rs file contains a struct that will load an obj file into memory using our Vec3f structures.

In the .obj format v indicates that the line contains vertex data
```
v -3.000000 1.800000 0.000000
v -2.991600 1.800000 -0.081000
v -2.991600 1.800000 0.081000
.
.
.
```

and the f contains data relating to a face, where the values are the index of the verticies that
have been loaded in.
```
f 104 94 76
f 55 65 84
f 84 76 55
f 40 44 65
f 65 55 40
f 22 30 44
.
.
.
```