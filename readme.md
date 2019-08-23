# tess2

This is a wrapper around libtess2, written by
[memononen](https://github.com/memononen/libtess2).

In general you should instead use [lyon](https://github.com/nical/lyon),
which is implemented in rust and faster in the majority of cases.

I created this because I have some very odd polygons with thousands of vertices
and self intersections which sometimes trips up lyon; check on
[issue 291](https://github.com/nical/lyon/issues/291) to see if this is still
the case before substituting libtess2.


## refs

- tess2.js

```
	// winding rule
	Tess2.WINDING_ODD = 0;
	Tess2.WINDING_NONZERO = 1;
	Tess2.WINDING_POSITIVE = 2;
	Tess2.WINDING_NEGATIVE = 3;
	Tess2.WINDING_ABS_GEQ_TWO = 4;

	// element type
	Tess2.POLYGONS = 0;
	Tess2.CONNECTED_POLYGONS = 1;
	Tess2.BOUNDARY_CONTOURS = 2;

	// tesselate
	Tess2.tesselate = function(opts) {
		var debug =  opts.debug || false;
		var tess = new Tesselator();
		for (var i = 0; i < opts.contours.length; i++) {
			tess.addContour(opts.vertexSize || 2, opts.contours[i]);
		}
		tess.tesselate(opts.windingRule || Tess2.WINDING_ODD,
					   opts.elementType || Tess2.POLYGONS,
					   opts.polySize || 3,
					   opts.vertexSize || 2,
					   opts.normal || [0,0,1]);
		return {
			vertices: tess.vertices,
			vertexIndices: tess.vertexIndices,
			vertexCount: tess.vertexCount,
			elements: tess.elements,
			elementCount: tess.elementCount,
			mesh: debug ? tess.mesh : undefined
		};
	};
```

- others

[math library aljabar](https://github.com/maplant/aljabar)