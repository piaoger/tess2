use libc;
extern "C" {
    pub type BucketAlloc;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn setjmp(_: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
    /*
** SGI FREE SOFTWARE LICENSE B (Version 2.0, Sept. 18, 2008) 
** Copyright (C) [dates of first publication] Silicon Graphics, Inc.
** All Rights Reserved.
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
** of the Software, and to permit persons to whom the Software is furnished to do so,
** subject to the following conditions:
** 
** The above copyright notice including the dates of first publication and either this
** permission notice or a reference to http://oss.sgi.com/projects/FreeB/ shall be
** included in all copies or substantial portions of the Software. 
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
** INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
** PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL SILICON GRAPHICS, INC.
** BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
** OR OTHER DEALINGS IN THE SOFTWARE.
** 
** Except as contained in this notice, the name of Silicon Graphics, Inc. shall not
** be used in advertising or otherwise to promote the sale, use or other dealings in
** this Software without prior written authorization from Silicon Graphics, Inc.
*/
/*
** Author: Mikko Mononen, July 2009.
*/
    #[no_mangle]
    fn deleteBucketAlloc(ba: *mut BucketAlloc);
    #[no_mangle]
    fn bucketFree(ba: *mut BucketAlloc, ptr: *mut libc::c_void);
    #[no_mangle]
    fn bucketAlloc(ba: *mut BucketAlloc) -> *mut libc::c_void;
    #[no_mangle]
    fn createBucketAlloc(alloc: *mut TESSalloc, name: *const libc::c_char,
                         itemSize: libc::c_uint, bucketSize: libc::c_uint)
     -> *mut BucketAlloc;
    /* The mesh operations below have three motivations: completeness,
* convenience, and efficiency.  The basic mesh operations are MakeEdge,
* Splice, and Delete.  All the other edge operations can be implemented
* in terms of these.  The other operations are provided for convenience
* and/or efficiency.
*
* When a face is split or a vertex is added, they are inserted into the
* global list *before* the existing vertex or face (ie. e->Org or e->Lface).
* This makes it easier to process all vertices or faces in the global lists
* without worrying about processing the same data twice.  As a convenience,
* when a face is split, the "inside" flag is copied from the old face.
* Other internal data (v->data, v->activeRegion, f->data, f->marked,
* f->trail, e->winding) is set to zero.
*
* ********************** Basic Edge Operations **************************
*
* tessMeshMakeEdge( mesh ) creates one edge, two vertices, and a loop.
* The loop (face) consists of the two new half-edges.
*
* tessMeshSplice( eOrg, eDst ) is the basic operation for changing the
* mesh connectivity and topology.  It changes the mesh so that
*  eOrg->Onext <- OLD( eDst->Onext )
*  eDst->Onext <- OLD( eOrg->Onext )
* where OLD(...) means the value before the meshSplice operation.
*
* This can have two effects on the vertex structure:
*  - if eOrg->Org != eDst->Org, the two vertices are merged together
*  - if eOrg->Org == eDst->Org, the origin is split into two vertices
* In both cases, eDst->Org is changed and eOrg->Org is untouched.
*
* Similarly (and independently) for the face structure,
*  - if eOrg->Lface == eDst->Lface, one loop is split into two
*  - if eOrg->Lface != eDst->Lface, two distinct loops are joined into one
* In both cases, eDst->Lface is changed and eOrg->Lface is unaffected.
*
* tessMeshDelete( eDel ) removes the edge eDel.  There are several cases:
* if (eDel->Lface != eDel->Rface), we join two loops into one; the loop
* eDel->Lface is deleted.  Otherwise, we are splitting one loop into two;
* the newly created loop will contain eDel->Dst.  If the deletion of eDel
* would create isolated vertices, those are deleted as well.
*
* ********************** Other Edge Operations **************************
*
* tessMeshAddEdgeVertex( eOrg ) creates a new edge eNew such that
* eNew == eOrg->Lnext, and eNew->Dst is a newly created vertex.
* eOrg and eNew will have the same left face.
*
* tessMeshSplitEdge( eOrg ) splits eOrg into two edges eOrg and eNew,
* such that eNew == eOrg->Lnext.  The new vertex is eOrg->Dst == eNew->Org.
* eOrg and eNew will have the same left face.
*
* tessMeshConnect( eOrg, eDst ) creates a new edge from eOrg->Dst
* to eDst->Org, and returns the corresponding half-edge eNew.
* If eOrg->Lface == eDst->Lface, this splits one loop into two,
* and the newly created loop is eNew->Lface.  Otherwise, two disjoint
* loops are merged into one, and the loop eDst->Lface is destroyed.
*
* ************************ Other Operations *****************************
*
* tessMeshNewMesh() creates a new mesh with no edges, no vertices,
* and no loops (what we usually call a "face").
*
* tessMeshUnion( mesh1, mesh2 ) forms the union of all structures in
* both meshes, and returns the new mesh (the old meshes are destroyed).
*
* tessMeshDeleteMesh( mesh ) will free all storage for any valid mesh.
*
* tessMeshZapFace( fZap ) destroys a face and removes it from the
* global face list.  All edges of fZap will have a NULL pointer as their
* left face.  Any edges which also have a NULL pointer as their right face
* are deleted entirely (along with any isolated vertices this produces).
* An entire mesh can be deleted by zapping its faces, one at a time,
* in any order.  Zapped faces cannot be used in further mesh operations!
*
* tessMeshCheckMesh( mesh ) checks a mesh for self-consistency.
*/
    #[no_mangle]
    fn tessMeshMergeConvexFaces(mesh: *mut TESSmesh,
                                maxVertsPerFace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tessMeshDeleteMesh(alloc: *mut TESSalloc, mesh: *mut TESSmesh);
    #[no_mangle]
    fn tessMeshCheckMesh(mesh: *mut TESSmesh);
    #[no_mangle]
    fn tessMeshSplice(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge,
                      eDst: *mut TESShalfEdge) -> libc::c_int;
    #[no_mangle]
    fn tessMeshMakeEdge(mesh: *mut TESSmesh) -> *mut TESShalfEdge;
    #[no_mangle]
    fn tessMeshNewMesh(alloc: *mut TESSalloc) -> *mut TESSmesh;
    #[no_mangle]
    fn tessMeshSplitEdge(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge)
     -> *mut TESShalfEdge;
    #[no_mangle]
    fn tessMeshFlipEdge(mesh: *mut TESSmesh, edge: *mut TESShalfEdge);
    #[no_mangle]
    fn tessMeshConnect(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge,
                       eDst: *mut TESShalfEdge) -> *mut TESShalfEdge;
    #[no_mangle]
    fn tessMeshDelete(mesh: *mut TESSmesh, eDel: *mut TESShalfEdge)
     -> libc::c_int;
    #[no_mangle]
    fn tessMeshZapFace(mesh: *mut TESSmesh, fZap: *mut TESSface);
    #[no_mangle]
    fn tessComputeInterior(tess: *mut TESStesselator) -> libc::c_int;
    #[no_mangle]
    fn tesedgeSign(u: *mut TESSvertex, v: *mut TESSvertex, w: *mut TESSvertex)
     -> TESSreal;
    #[no_mangle]
    fn tesedgeIsLocallyDelaunay(e: *mut TESShalfEdge) -> libc::c_int;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type jmp_buf = [libc::c_int; 37];
/*
** SGI FREE SOFTWARE LICENSE B (Version 2.0, Sept. 18, 2008)
** Copyright (C) [dates of first publication] Silicon Graphics, Inc.
** All Rights Reserved.
**
** Permission is hereby granted, free of charge, to any person obtaining a copy
** of this software and associated documentation files (the "Software"), to deal
** in the Software without restriction, including without limitation the rights
** to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
** of the Software, and to permit persons to whom the Software is furnished to do so,
** subject to the following conditions:
**
** The above copyright notice including the dates of first publication and either this
** permission notice or a reference to http://oss.sgi.com/projects/FreeB/ shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
** INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
** PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL SILICON GRAPHICS, INC.
** BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
** OR OTHER DEALINGS IN THE SOFTWARE.
**
** Except as contained in this notice, the name of Silicon Graphics, Inc. shall not
** be used in advertising or otherwise to promote the sale, use or other dealings in
** this Software without prior written authorization from Silicon Graphics, Inc.
*/
/*
** Author: Mikko Mononen, July 2009.
*/
// See OpenGL Red Book for description of the winding rules
// http://www.glprogramming.com/red/chapter11.html
pub type TessWindingRule = libc::c_uint;
pub const TESS_WINDING_ABS_GEQ_TWO: TessWindingRule = 4;
pub const TESS_WINDING_NEGATIVE: TessWindingRule = 3;
pub const TESS_WINDING_POSITIVE: TessWindingRule = 2;
pub const TESS_WINDING_NONZERO: TessWindingRule = 1;
pub const TESS_WINDING_ODD: TessWindingRule = 0;
// The contents of the tessGetElements() depends on element type being passed to tessTesselate().
// Tesselation result element types:
// TESS_POLYGONS
//   Each element in the element array is polygon defined as 'polySize' number of vertex indices.
//   If a polygon has than 'polySize' vertices, the remaining indices are stored as TESS_UNDEF.
//   Example, drawing a polygon:
//     const int nelems = tessGetElementCount(tess);
//     const TESSindex* elems = tessGetElements(tess);
//     for (int i = 0; i < nelems; i++) {
//         const TESSindex* poly = &elems[i * polySize];
//         glBegin(GL_POLYGON);
//         for (int j = 0; j < polySize; j++) {
//             if (poly[j] == TESS_UNDEF) break;
//             glVertex2fv(&verts[poly[j]*vertexSize]);
//         }
//         glEnd();
//     }
//
// TESS_CONNECTED_POLYGONS
//   Each element in the element array is polygon defined as 'polySize' number of vertex indices,
//   followed by 'polySize' indices to neighour polygons, that is each element is 'polySize' * 2 indices.
//   If a polygon has than 'polySize' vertices, the remaining indices are stored as TESS_UNDEF.
//   If a polygon edge is a boundary, that is, not connected to another polygon, the neighbour index is TESS_UNDEF.
//   Example, flood fill based on seed polygon:
//     const int nelems = tessGetElementCount(tess);
//     const TESSindex* elems = tessGetElements(tess);
//     unsigned char* visited = (unsigned char*)calloc(nelems);
//     TESSindex stack[50];
//     int nstack = 0;
//     stack[nstack++] = seedPoly;
//     visited[startPoly] = 1;
//     while (nstack > 0) {
//         TESSindex idx = stack[--nstack];
//			const TESSindex* poly = &elems[idx * polySize * 2];
//			const TESSindex* nei = &poly[polySize];
//          for (int i = 0; i < polySize; i++) {
//              if (poly[i] == TESS_UNDEF) break;
//              if (nei[i] != TESS_UNDEF && !visited[nei[i]])
//	                stack[nstack++] = nei[i];
//                  visited[nei[i]] = 1;
//              }
//          }
//     }
//
// TESS_BOUNDARY_CONTOURS
//   Each element in the element array is [base index, count] pair defining a range of vertices for a contour.
//   The first value is index to first vertex in contour and the second value is number of vertices in the contour.
//   Example, drawing contours:
//     const int nelems = tessGetElementCount(tess);
//     const TESSindex* elems = tessGetElements(tess);
//     for (int i = 0; i < nelems; i++) {
//         const TESSindex base = elems[i * 2];
//         const TESSindex count = elems[i * 2 + 1];
//         glBegin(GL_LINE_LOOP);
//         for (int j = 0; j < count; j++) {
//             glVertex2fv(&verts[(base+j) * vertexSize]);
//         }
//         glEnd();
//     }
pub type TessElementType = libc::c_uint;
pub const TESS_BOUNDARY_CONTOURS: TessElementType = 2;
pub const TESS_CONNECTED_POLYGONS: TessElementType = 1;
pub const TESS_POLYGONS: TessElementType = 0;
// TESS_CONSTRAINED_DELAUNAY_TRIANGULATION
//   If enabled, the initial triagulation is improved with non-robust Constrained Delayney triangulation.
//   Disable by default.
//
// TESS_REVERSE_CONTOURS
//   If enabled, tessAddContour() will treat CW contours as CCW and vice versa
//   Disabled by default.
pub type TessOption = libc::c_uint;
pub const TESS_REVERSE_CONTOURS: TessOption = 1;
pub const TESS_CONSTRAINED_DELAUNAY_TRIANGULATION: TessOption = 0;
pub type TESSreal = libc::c_float;
pub type TESSindex = libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESStesselator {
    pub mesh: *mut TESSmesh,
    pub outOfMemory: libc::c_int,
    pub normal: [TESSreal; 3],
    pub sUnit: [TESSreal; 3],
    pub tUnit: [TESSreal; 3],
    pub bmin: [TESSreal; 2],
    pub bmax: [TESSreal; 2],
    pub processCDT: libc::c_int,
    pub reverseContours: libc::c_int,
    pub windingRule: libc::c_int,
    pub dict: *mut Dict,
    pub pq: *mut PriorityQ,
    pub event: *mut TESSvertex,
    pub regionPool: *mut BucketAlloc,
    pub vertexIndexCounter: TESSindex,
    pub vertices: *mut TESSreal,
    pub vertexIndices: *mut TESSindex,
    pub vertexCount: libc::c_int,
    pub elements: *mut TESSindex,
    pub elementCount: libc::c_int,
    pub alloc: TESSalloc,
    pub env: jmp_buf,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESSalloc {
    pub memalloc: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: libc::c_uint)
                             -> *mut libc::c_void>,
    pub memrealloc: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *mut libc::c_void,
                                                _: libc::c_uint)
                               -> *mut libc::c_void>,
    pub memfree: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *mut libc::c_void) -> ()>,
    pub userData: *mut libc::c_void,
    pub meshEdgeBucketSize: libc::c_int,
    pub meshVertexBucketSize: libc::c_int,
    pub meshFaceBucketSize: libc::c_int,
    pub dictNodeBucketSize: libc::c_int,
    pub regionBucketSize: libc::c_int,
    pub extraVertices: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESSvertex {
    pub next: *mut TESSvertex,
    pub prev: *mut TESSvertex,
    pub anEdge: *mut TESShalfEdge,
    pub coords: [TESSreal; 3],
    pub s: TESSreal,
    pub t: TESSreal,
    pub pqHandle: libc::c_int,
    pub n: TESSindex,
    pub idx: TESSindex,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESShalfEdge {
    pub next: *mut TESShalfEdge,
    pub Sym: *mut TESShalfEdge,
    pub Onext: *mut TESShalfEdge,
    pub Lnext: *mut TESShalfEdge,
    pub Org: *mut TESSvertex,
    pub Lface: *mut TESSface,
    pub activeRegion: *mut ActiveRegion,
    pub winding: libc::c_int,
    pub mark: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ActiveRegion {
    pub eUp: *mut TESShalfEdge,
    pub nodeUp: *mut DictNode,
    pub windingNumber: libc::c_int,
    pub inside: libc::c_int,
    pub sentinel: libc::c_int,
    pub dirty: libc::c_int,
    pub fixUpperEdge: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct DictNode {
    pub key: DictKey,
    pub next: *mut DictNode,
    pub prev: *mut DictNode,
}
pub type DictKey = *mut libc::c_void;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESSface {
    pub next: *mut TESSface,
    pub prev: *mut TESSface,
    pub anEdge: *mut TESShalfEdge,
    pub trail: *mut TESSface,
    pub n: TESSindex,
    pub marked: libc::c_char,
    pub inside: libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PriorityQ {
    pub heap: *mut PriorityQHeap,
    pub keys: *mut PQkey,
    pub order: *mut *mut PQkey,
    pub size: PQhandle,
    pub max: PQhandle,
    pub initialized: libc::c_int,
    pub leq: Option<unsafe extern "C" fn(_: PQkey, _: PQkey) -> libc::c_int>,
}
pub type PQkey = *mut libc::c_void;
pub type PQhandle = libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PriorityQHeap {
    pub nodes: *mut PQnode,
    pub handles: *mut PQhandleElem,
    pub size: libc::c_int,
    pub max: libc::c_int,
    pub freeList: PQhandle,
    pub initialized: libc::c_int,
    pub leq: Option<unsafe extern "C" fn(_: PQkey, _: PQkey) -> libc::c_int>,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PQhandleElem {
    pub key: PQkey,
    pub node: PQhandle,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct PQnode {
    pub handle: PQhandle,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Dict {
    pub head: DictNode,
    pub frame: *mut libc::c_void,
    pub nodePool: *mut BucketAlloc,
    pub leq: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: DictKey,
                                         _: DictKey) -> libc::c_int>,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TESSmesh {
    pub vHead: TESSvertex,
    pub fHead: TESSface,
    pub eHead: TESShalfEdge,
    pub eHeadSym: TESShalfEdge,
    pub edgeBucket: *mut BucketAlloc,
    pub vertexBucket: *mut BucketAlloc,
    pub faceBucket: *mut BucketAlloc,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct EdgeStackNode {
    pub edge: *mut TESShalfEdge,
    pub next: *mut EdgeStackNode,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct EdgeStack {
    pub top: *mut EdgeStackNode,
    pub nodeBucket: *mut BucketAlloc,
}
unsafe extern "C" fn LongAxis(mut v: *mut TESSreal) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    if (if *v.offset(1) < 0i32 as libc::c_float {
            -*v.offset(1)
        } else { *v.offset(1) }) >
           (if *v.offset(0) < 0i32 as libc::c_float {
                -*v.offset(0)
            } else { *v.offset(0) }) {
        i = 1i32
    }
    if (if *v.offset(2) < 0i32 as libc::c_float {
            -*v.offset(2)
        } else { *v.offset(2) }) >
           (if *v.offset(i as isize) < 0i32 as libc::c_float {
                -*v.offset(i as isize)
            } else { *v.offset(i as isize) }) {
        i = 2i32
    }
    return i;
}
unsafe extern "C" fn ShortAxis(mut v: *mut TESSreal) -> libc::c_int {
    let mut i: libc::c_int = 0i32;
    if (if *v.offset(1) < 0i32 as libc::c_float {
            -*v.offset(1)
        } else { *v.offset(1) }) <
           (if *v.offset(0) < 0i32 as libc::c_float {
                -*v.offset(0)
            } else { *v.offset(0) }) {
        i = 1i32
    }
    if (if *v.offset(2) < 0i32 as libc::c_float {
            -*v.offset(2)
        } else { *v.offset(2) }) <
           (if *v.offset(i as isize) < 0i32 as libc::c_float {
                -*v.offset(i as isize)
            } else { *v.offset(i as isize) }) {
        i = 2i32
    }
    return i;
}
unsafe extern "C" fn ComputeNormal(mut tess: *mut TESStesselator,
                                   mut norm: *mut TESSreal) {
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut v1: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut v2: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut c: TESSreal = 0.;
    let mut tLen2: TESSreal = 0.;
    let mut maxLen2: TESSreal = 0.;
    let mut maxVal: [TESSreal; 3] = [0.; 3];
    let mut minVal: [TESSreal; 3] = [0.; 3];
    let mut d1: [TESSreal; 3] = [0.; 3];
    let mut d2: [TESSreal; 3] = [0.; 3];
    let mut tNorm: [TESSreal; 3] = [0.; 3];
    let mut maxVert: [*mut TESSvertex; 3] = [0 as *mut TESSvertex; 3];
    let mut minVert: [*mut TESSvertex; 3] = [0 as *mut TESSvertex; 3];
    let mut vHead: *mut TESSvertex = &mut (*(*tess).mesh).vHead;
    let mut i: libc::c_int = 0;
    v = (*vHead).next;
    i = 0i32;
    while i < 3i32 {
        c = (*v).coords[i as usize];
        minVal[i as usize] = c;
        minVert[i as usize] = v;
        maxVal[i as usize] = c;
        maxVert[i as usize] = v;
        i += 1
    }
    v = (*vHead).next;
    while v != vHead {
        i = 0i32;
        while i < 3i32 {
            c = (*v).coords[i as usize];
            if c < minVal[i as usize] {
                minVal[i as usize] = c;
                minVert[i as usize] = v
            }
            if c > maxVal[i as usize] {
                maxVal[i as usize] = c;
                maxVert[i as usize] = v
            }
            i += 1
        }
        v = (*v).next
    }
    /* Find two vertices separated by at least 1/sqrt(3) of the maximum
	* distance between any two vertices
	*/
    i = 0i32;
    if maxVal[1] - minVal[1] > maxVal[0] - minVal[0] { i = 1i32 }
    if maxVal[2] - minVal[2] > maxVal[i as usize] - minVal[i as usize] {
        i = 2i32
    }
    if minVal[i as usize] >= maxVal[i as usize] {
        /* All vertices are the same -- normal doesn't matter */
        *norm.offset(0) = 0i32 as TESSreal;
        *norm.offset(1) = 0i32 as TESSreal;
        *norm.offset(2) = 1i32 as TESSreal;
        return
    }
    /* Look for a third vertex which forms the triangle with maximum area
	* (Length of normal == twice the triangle area)
	*/
    maxLen2 = 0i32 as TESSreal;
    v1 = minVert[i as usize];
    v2 = maxVert[i as usize];
    d1[0] = (*v1).coords[0] - (*v2).coords[0];
    d1[1] = (*v1).coords[1] - (*v2).coords[1];
    d1[2] = (*v1).coords[2] - (*v2).coords[2];
    v = (*vHead).next;
    while v != vHead {
        d2[0] = (*v).coords[0] - (*v2).coords[0];
        d2[1] = (*v).coords[1] - (*v2).coords[1];
        d2[2] = (*v).coords[2] - (*v2).coords[2];
        tNorm[0] = d1[1] * d2[2] - d1[2] * d2[1];
        tNorm[1] = d1[2] * d2[0] - d1[0] * d2[2];
        tNorm[2] = d1[0] * d2[1] - d1[1] * d2[0];
        tLen2 =
            tNorm[0] * tNorm[0] + tNorm[1] * tNorm[1] + tNorm[2] * tNorm[2];
        if tLen2 > maxLen2 {
            maxLen2 = tLen2;
            *norm.offset(0) = tNorm[0];
            *norm.offset(1) = tNorm[1];
            *norm.offset(2) = tNorm[2]
        }
        v = (*v).next
    }
    if maxLen2 <= 0i32 as libc::c_float {
        /* All points lie on a single line -- any decent normal will do */
        let ref mut fresh1 = *norm.offset(1);
        let ref mut fresh0 = *norm.offset(2);
        *fresh0 = 0i32 as TESSreal;
        *fresh1 = *fresh0;
        *norm.offset(0) = *fresh1;
        *norm.offset(ShortAxis(d1.as_mut_ptr()) as isize) = 1i32 as TESSreal
    };
}
unsafe extern "C" fn CheckOrientation(mut tess: *mut TESStesselator) {
    let mut area: TESSreal = 0.;
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut fHead: *mut TESSface = &mut (*(*tess).mesh).fHead;
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vHead: *mut TESSvertex = &mut (*(*tess).mesh).vHead;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    /* When we compute the normal automatically, we choose the orientation
	* so that the the sum of the signed areas of all contours is non-negative.
	*/
    area = 0i32 as TESSreal;
    f = (*fHead).next;
    while f != fHead {
        e = (*f).anEdge;
        if !((*e).winding <= 0i32) {
            loop  {
                area +=
                    ((*(*e).Org).s - (*(*(*e).Sym).Org).s) *
                        ((*(*e).Org).t + (*(*(*e).Sym).Org).t);
                e = (*e).Lnext;
                if !(e != (*f).anEdge) { break ; }
            }
        }
        f = (*f).next
    }
    if area < 0i32 as libc::c_float {
        /* Reverse the orientation by flipping all the t-coordinates */
        v = (*vHead).next;
        while v != vHead { (*v).t = -(*v).t; v = (*v).next }
        (*tess).tUnit[0] = -(*tess).tUnit[0];
        (*tess).tUnit[1] = -(*tess).tUnit[1];
        (*tess).tUnit[2] = -(*tess).tUnit[2]
    };
}
/* Determine the polygon normal and project vertices onto the plane
* of the polygon.
*/
#[no_mangle]
pub unsafe extern "C" fn tessProjectPolygon(mut tess: *mut TESStesselator) {
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vHead: *mut TESSvertex = &mut (*(*tess).mesh).vHead;
    let mut norm: [TESSreal; 3] = [0.; 3];
    let mut sUnit: *mut TESSreal = 0 as *mut TESSreal;
    let mut tUnit: *mut TESSreal = 0 as *mut TESSreal;
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut computedNormal: libc::c_int = 0i32;
    norm[0] = (*tess).normal[0];
    norm[1] = (*tess).normal[1];
    norm[2] = (*tess).normal[2];
    if norm[0] == 0i32 as libc::c_float && norm[1] == 0i32 as libc::c_float &&
           norm[2] == 0i32 as libc::c_float {
        ComputeNormal(tess, norm.as_mut_ptr());
        computedNormal = 1i32
    }
    sUnit = (*tess).sUnit.as_mut_ptr();
    tUnit = (*tess).tUnit.as_mut_ptr();
    i = LongAxis(norm.as_mut_ptr());
    /* Project perpendicular to a coordinate axis -- better numerically */
    *sUnit.offset(i as isize) = 0i32 as TESSreal;
    *sUnit.offset(((i + 1i32) % 3i32) as isize) = 1.0f64 as TESSreal;
    *sUnit.offset(((i + 2i32) % 3i32) as isize) = 0.0f64 as TESSreal;
    *tUnit.offset(i as isize) = 0i32 as TESSreal;
    *tUnit.offset(((i + 1i32) % 3i32) as isize) =
        if norm[i as usize] > 0i32 as libc::c_float {
            -(0.0f64 as TESSreal)
        } else { 0.0f64 as TESSreal };
    *tUnit.offset(((i + 2i32) % 3i32) as isize) =
        if norm[i as usize] > 0i32 as libc::c_float {
            1.0f64 as TESSreal
        } else { -(1.0f64 as TESSreal) };
    /* Project the vertices onto the sweep plane */
    v = (*vHead).next;
    while v != vHead {
        (*v).s =
            (*v).coords[0] * *sUnit.offset(0) +
                (*v).coords[1] * *sUnit.offset(1) +
                (*v).coords[2] * *sUnit.offset(2);
        (*v).t =
            (*v).coords[0] * *tUnit.offset(0) +
                (*v).coords[1] * *tUnit.offset(1) +
                (*v).coords[2] * *tUnit.offset(2);
        v = (*v).next
    }
    if computedNormal != 0 { CheckOrientation(tess); }
    /* Compute ST bounds. */
    first = 1i32;
    v = (*vHead).next;
    while v != vHead {
        if first != 0 {
            (*tess).bmax[0] = (*v).s;
            (*tess).bmin[0] = (*tess).bmax[0];
            (*tess).bmax[1] = (*v).t;
            (*tess).bmin[1] = (*tess).bmax[1];
            first = 0i32
        } else {
            if (*v).s < (*tess).bmin[0] { (*tess).bmin[0] = (*v).s }
            if (*v).s > (*tess).bmax[0] { (*tess).bmax[0] = (*v).s }
            if (*v).t < (*tess).bmin[1] { (*tess).bmin[1] = (*v).t }
            if (*v).t > (*tess).bmax[1] { (*tess).bmax[1] = (*v).t }
        }
        v = (*v).next
    };
}
/* tessMeshTessellateMonoRegion( face ) tessellates a monotone region
* (what else would it do??)  The region must consist of a single
* loop of half-edges (see mesh.h) oriented CCW.  "Monotone" in this
* case means that any vertical line intersects the interior of the
* region in a single interval.
*
* Tessellation consists of adding interior edges (actually pairs of
* half-edges), to split the region into non-overlapping triangles.
*
* The basic idea is explained in Preparata and Shamos (which I don''t
* have handy right now), although their implementation is more
* complicated than this one.  The are two edge chains, an upper chain
* and a lower chain.  We process all vertices from both chains in order,
* from right to left.
*
* The algorithm ensures that the following invariant holds after each
* vertex is processed: the untessellated region consists of two
* chains, where one chain (say the upper) is a single edge, and
* the other chain is concave.  The left vertex of the single edge
* is always to the left of all vertices in the concave chain.
*
* Each step consists of adding the rightmost unprocessed vertex to one
* of the two chains, and forming a fan of triangles from the rightmost
* of two chain endpoints.  Determining whether we can add each triangle
* to the fan is a simple orientation test.  By making the fan as large
* as possible, we restore the invariant (check it yourself).
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshTessellateMonoRegion(mut mesh: *mut TESSmesh,
                                                      mut face: *mut TESSface)
 -> libc::c_int {
    let mut up: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut lo: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    /* All edges are oriented CCW around the boundary of the region.
	* First, find the half-edge whose origin vertex is rightmost.
	* Since the sweep goes from left to right, face->anEdge should
	* be close to the edge we want.
	*/
    up = (*face).anEdge;
    if !((*up).Lnext != up && (*(*up).Lnext).Lnext != up) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 29],
                                               &[libc::c_char; 29]>(b"tessMeshTessellateMonoRegion\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/tess.c\x00"
                         as *const u8 as *const libc::c_char, 331i32,
                     b"up->Lnext != up && up->Lnext->Lnext != up\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    while (*(*(*up).Sym).Org).s < (*(*up).Org).s ||
              (*(*(*up).Sym).Org).s == (*(*up).Org).s &&
                  (*(*(*up).Sym).Org).t <= (*(*up).Org).t {
        up = (*(*up).Onext).Sym
    }
    while (*(*up).Org).s < (*(*(*up).Sym).Org).s ||
              (*(*up).Org).s == (*(*(*up).Sym).Org).s &&
                  (*(*up).Org).t <= (*(*(*up).Sym).Org).t {
        up = (*up).Lnext
    }
    lo = (*(*up).Onext).Sym;
    while (*up).Lnext != lo {
        if (*(*(*up).Sym).Org).s < (*(*lo).Org).s ||
               (*(*(*up).Sym).Org).s == (*(*lo).Org).s &&
                   (*(*(*up).Sym).Org).t <= (*(*lo).Org).t {
            /* up->Dst is on the left.  It is safe to form triangles from lo->Org.
			* The EdgeGoesLeft test guarantees progress even when some triangles
			* are CW, given that the upper and lower chains are truly monotone.
			*/
            while (*lo).Lnext != up &&
                      ((*(*(*(*lo).Lnext).Sym).Org).s <
                           (*(*(*lo).Lnext).Org).s ||
                           (*(*(*(*lo).Lnext).Sym).Org).s ==
                               (*(*(*lo).Lnext).Org).s &&
                               (*(*(*(*lo).Lnext).Sym).Org).t <=
                                   (*(*(*lo).Lnext).Org).t ||
                           tesedgeSign((*lo).Org, (*(*lo).Sym).Org,
                                       (*(*(*lo).Lnext).Sym).Org) <=
                               0i32 as libc::c_float) {
                let mut tempHalfEdge: *mut TESShalfEdge =
                    tessMeshConnect(mesh, (*lo).Lnext, lo);
                if tempHalfEdge.is_null() { return 0i32 }
                lo = (*tempHalfEdge).Sym
            }
            lo = (*(*lo).Onext).Sym
        } else {
            /* lo->Org is on the left.  We can make CCW triangles from up->Dst. */
            while (*lo).Lnext != up &&
                      ((*(*(*(*up).Onext).Sym).Org).s <
                           (*(*(*(*(*up).Onext).Sym).Sym).Org).s ||
                           (*(*(*(*up).Onext).Sym).Org).s ==
                               (*(*(*(*(*up).Onext).Sym).Sym).Org).s &&
                               (*(*(*(*up).Onext).Sym).Org).t <=
                                   (*(*(*(*(*up).Onext).Sym).Sym).Org).t ||
                           tesedgeSign((*(*up).Sym).Org, (*up).Org,
                                       (*(*(*up).Onext).Sym).Org) >=
                               0i32 as libc::c_float) {
                let mut tempHalfEdge_0: *mut TESShalfEdge =
                    tessMeshConnect(mesh, up, (*(*up).Onext).Sym);
                if tempHalfEdge_0.is_null() { return 0i32 }
                up = (*tempHalfEdge_0).Sym
            }
            up = (*up).Lnext
        }
    }
    /* Now lo->Org == up->Dst == the leftmost vertex.  The remaining region
	* can be tessellated in a fan from this leftmost vertex.
	*/
    if !((*lo).Lnext != up) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 29],
                                               &[libc::c_char; 29]>(b"tessMeshTessellateMonoRegion\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/tess.c\x00"
                         as *const u8 as *const libc::c_char, 367i32,
                     b"lo->Lnext != up\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    while (*(*lo).Lnext).Lnext != up {
        let mut tempHalfEdge_1: *mut TESShalfEdge =
            tessMeshConnect(mesh, (*lo).Lnext, lo);
        if tempHalfEdge_1.is_null() { return 0i32 }
        lo = (*tempHalfEdge_1).Sym
    }
    return 1i32;
}
/* tessMeshTessellateInterior( mesh ) tessellates each region of
* the mesh which is marked "inside" the polygon.  Each such region
* must be monotone.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshTessellateInterior(mut mesh: *mut TESSmesh)
 -> libc::c_int {
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut next: *mut TESSface = 0 as *mut TESSface;
    /*LINTED*/
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        /* Make sure we don''t try to tessellate the new triangles. */
        next = (*f).next;
        if (*f).inside != 0 {
            if tessMeshTessellateMonoRegion(mesh, f) == 0 { return 0i32 }
        }
        f = next
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn stackInit(mut stack: *mut EdgeStack,
                                   mut alloc: *mut TESSalloc) -> libc::c_int {
    (*stack).top = 0 as *mut EdgeStackNode;
    (*stack).nodeBucket =
        createBucketAlloc(alloc,
                          b"CDT nodes\x00" as *const u8 as
                              *const libc::c_char,
                          ::std::mem::size_of::<EdgeStackNode>() as
                              libc::c_ulong as libc::c_uint,
                          512i32 as libc::c_uint);
    return ((*stack).nodeBucket != 0 as *mut libc::c_void as *mut BucketAlloc)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stackDelete(mut stack: *mut EdgeStack) {
    deleteBucketAlloc((*stack).nodeBucket);
}
#[no_mangle]
pub unsafe extern "C" fn stackEmpty(mut stack: *mut EdgeStack)
 -> libc::c_int {
    return ((*stack).top == 0 as *mut libc::c_void as *mut EdgeStackNode) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stackPush(mut stack: *mut EdgeStack,
                                   mut e: *mut TESShalfEdge) {
    let mut node: *mut EdgeStackNode =
        bucketAlloc((*stack).nodeBucket) as *mut EdgeStackNode;
    if node.is_null() { return }
    (*node).edge = e;
    (*node).next = (*stack).top;
    (*stack).top = node;
}
#[no_mangle]
pub unsafe extern "C" fn stackPop(mut stack: *mut EdgeStack)
 -> *mut TESShalfEdge {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut node: *mut EdgeStackNode = (*stack).top;
    if !node.is_null() {
        (*stack).top = (*node).next;
        e = (*node).edge;
        bucketFree((*stack).nodeBucket, node as *mut libc::c_void);
    }
    return e;
}
//	Starting with a valid triangulation, uses the Edge Flip algorithm to
//	refine the triangulation into a Constrained Delaunay Triangulation.
#[no_mangle]
pub unsafe extern "C" fn tessMeshRefineDelaunay(mut mesh: *mut TESSmesh,
                                                mut alloc: *mut TESSalloc) {
    // At this point, we have a valid, but not optimal, triangulation.
	// We refine the triangulation using the Edge Flip algorithm
	//
	//  1) Find all internal edges
	//	2) Mark all dual edges
	//	3) insert all dual edges into a queue
    let mut f: *mut TESSface = 0 as *mut TESSface; // Mark internal edges
    let mut stack: EdgeStack =
        EdgeStack{top: 0 as *mut EdgeStackNode,
                  nodeBucket: 0 as *mut BucketAlloc,}; // Insert into queue
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut maxFaces: libc::c_int = 0i32;
    let mut maxIter: libc::c_int = 0i32;
    let mut iter: libc::c_int = 0i32;
    stackInit(&mut stack, alloc);
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        if (*f).inside != 0 {
            e = (*f).anEdge;
            loop  {
                (*e).mark =
                    (!(*(*e).Sym).Lface.is_null() &&
                         (*(*(*e).Sym).Lface).inside as libc::c_int != 0) as
                        libc::c_int;
                if (*e).mark != 0 && (*(*e).Sym).mark == 0 {
                    stackPush(&mut stack, e);
                }
                e = (*e).Lnext;
                if !(e != (*f).anEdge) { break ; }
            }
            maxFaces += 1
        }
        f = (*f).next
    }
    // The algorithm should converge on O(n^2), since the predicate is not robust,
	// we'll save guard against infinite loop.
    maxIter = maxFaces * maxFaces;
    // Pop stack until we find a reversed edge
	// Flip the reversed edge, and insert any of the four opposite edges
	// which are internal and not already in the stack (!marked)
    while stackEmpty(&mut stack) == 0 && iter < maxIter {
        e = stackPop(&mut stack);
        (*(*e).Sym).mark = 0i32;
        (*e).mark = (*(*e).Sym).mark;
        if tesedgeIsLocallyDelaunay(e) == 0 {
            let mut edges: [*mut TESShalfEdge; 4] =
                [0 as *mut TESShalfEdge; 4];
            let mut i: libc::c_int = 0;
            tessMeshFlipEdge(mesh, e);
            // for each opposite edge
            edges[0] = (*e).Lnext;
            edges[1] = (*(*e).Onext).Sym;
            edges[2] = (*(*e).Sym).Lnext;
            edges[3] = (*(*(*e).Sym).Onext).Sym;
            i = 0i32;
            while i < 4i32 {
                if (*edges[i as usize]).mark == 0 &&
                       !(*(*edges[i as usize]).Sym).Lface.is_null() &&
                       (*(*(*edges[i as usize]).Sym).Lface).inside as
                           libc::c_int != 0 {
                    (*(*edges[i as usize]).Sym).mark = 1i32;
                    (*edges[i as usize]).mark =
                        (*(*edges[i as usize]).Sym).mark;
                    stackPush(&mut stack, edges[i as usize]);
                }
                i += 1
            }
        }
        iter += 1
    }
    stackDelete(&mut stack);
}
/* tessMeshDiscardExterior( mesh ) zaps (ie. sets to NULL) all faces
* which are not marked "inside" the polygon.  Since further mesh operations
* on NULL faces are not allowed, the main purpose is to clean up the
* mesh so that exterior loops are not represented in the data structure.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshDiscardExterior(mut mesh: *mut TESSmesh) {
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut next: *mut TESSface = 0 as *mut TESSface;
    /*LINTED*/
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        /* Since f will be destroyed, save its next pointer. */
        next = (*f).next;
        if (*f).inside == 0 { tessMeshZapFace(mesh, f); }
        f = next
    };
}
/* tessMeshSetWindingNumber( mesh, value, keepOnlyBoundary ) resets the
* winding numbers on all edges so that regions marked "inside" the
* polygon have a winding number of "value", and regions outside
* have a winding number of 0.
*
* If keepOnlyBoundary is TRUE, it also deletes all edges which do not
* separate an interior region from an exterior one.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshSetWindingNumber(mut mesh: *mut TESSmesh,
                                                  mut value: libc::c_int,
                                                  mut keepOnlyBoundary:
                                                      libc::c_int)
 -> libc::c_int {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    e = (*mesh).eHead.next;
    while e != &mut (*mesh).eHead as *mut TESShalfEdge {
        eNext = (*e).next;
        if (*(*(*e).Sym).Lface).inside as libc::c_int !=
               (*(*e).Lface).inside as libc::c_int {
            /* This is a boundary edge (one side is interior, one is exterior). */
            (*e).winding =
                if (*(*e).Lface).inside as libc::c_int != 0 {
                    value
                } else { -value }
        } else if keepOnlyBoundary == 0 {
            (*e).winding = 0i32
        } else if tessMeshDelete(mesh, e) == 0 { return 0i32 }
        e = eNext
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn heapAlloc(mut userData: *mut libc::c_void,
                                   mut size: libc::c_uint)
 -> *mut libc::c_void {
    if 1i32 != 0 { } else { };
    return malloc(size as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn heapRealloc(mut userData: *mut libc::c_void,
                                     mut ptr: *mut libc::c_void,
                                     mut size: libc::c_uint)
 -> *mut libc::c_void {
    if 1i32 != 0 { } else { };
    return realloc(ptr, size as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn heapFree(mut userData: *mut libc::c_void,
                                  mut ptr: *mut libc::c_void) {
    if 1i32 != 0 { } else { };
    free(ptr);
}
static mut defaulAlloc: TESSalloc =
    unsafe {
        {
            let mut init =
                TESSalloc{memalloc:
                              Some(heapAlloc as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: libc::c_uint)
                                           -> *mut libc::c_void),
                          memrealloc:
                              Some(heapRealloc as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_void,
                                                            _: libc::c_uint)
                                           -> *mut libc::c_void),
                          memfree:
                              Some(heapFree as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()),
                          userData:
                              0 as *const libc::c_void as *mut libc::c_void,
                          meshEdgeBucketSize: 0i32,
                          meshVertexBucketSize: 0i32,
                          meshFaceBucketSize: 0i32,
                          dictNodeBucketSize: 0i32,
                          regionBucketSize: 0i32,
                          extraVertices: 0i32,};
            init
        }
    };
/* Both regions are interior, or both are exterior. */
// Number of extra vertices allocated for the priority queue.
//
// Example use:
//
//
//
//
// tessNewTess() - Creates a new tesselator.
// Use tessDeleteTess() to delete the tesselator.
// Parameters:
//   alloc - pointer to a filled TESSalloc struct or NULL to use default malloc based allocator.
// Returns:
//   new tesselator object.
#[no_mangle]
pub unsafe extern "C" fn tessNewTess(mut alloc: *mut TESSalloc)
 -> *mut TESStesselator {
    let mut tess: *mut TESStesselator = 0 as *mut TESStesselator;
    if alloc.is_null() { alloc = &mut defaulAlloc }
    /* Only initialize fields which can be changed by the api.  Other fields
	* are initialized where they are used.
	*/
    tess =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<TESStesselator>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut TESStesselator;
    if tess.is_null() {
        return 0 as *mut TESStesselator
        /* out of memory */
    }
    (*tess).alloc = *alloc;
    /* Check and set defaults. */
    if (*tess).alloc.meshEdgeBucketSize == 0i32 {
        (*tess).alloc.meshEdgeBucketSize = 512i32
    }
    if (*tess).alloc.meshVertexBucketSize == 0i32 {
        (*tess).alloc.meshVertexBucketSize = 512i32
    }
    if (*tess).alloc.meshFaceBucketSize == 0i32 {
        (*tess).alloc.meshFaceBucketSize = 256i32
    }
    if (*tess).alloc.dictNodeBucketSize == 0i32 {
        (*tess).alloc.dictNodeBucketSize = 512i32
    }
    if (*tess).alloc.regionBucketSize == 0i32 {
        (*tess).alloc.regionBucketSize = 256i32
    }
    (*tess).normal[0] = 0i32 as TESSreal;
    (*tess).normal[1] = 0i32 as TESSreal;
    (*tess).normal[2] = 0i32 as TESSreal;
    (*tess).bmin[0] = 0i32 as TESSreal;
    (*tess).bmin[1] = 0i32 as TESSreal;
    (*tess).bmax[0] = 0i32 as TESSreal;
    (*tess).bmax[1] = 0i32 as TESSreal;
    (*tess).reverseContours = 0i32;
    (*tess).windingRule = TESS_WINDING_ODD as libc::c_int;
    (*tess).processCDT = 0i32;
    if (*tess).alloc.regionBucketSize < 16i32 {
        (*tess).alloc.regionBucketSize = 16i32
    }
    if (*tess).alloc.regionBucketSize > 4096i32 {
        (*tess).alloc.regionBucketSize = 4096i32
    }
    (*tess).regionPool =
        createBucketAlloc(&mut (*tess).alloc,
                          b"Regions\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<ActiveRegion>() as
                              libc::c_ulong as libc::c_uint,
                          (*tess).alloc.regionBucketSize as libc::c_uint);
    // Initialize to begin polygon.
    (*tess).mesh = 0 as *mut TESSmesh;
    (*tess).outOfMemory = 0i32;
    (*tess).vertexIndexCounter = 0i32;
    (*tess).vertices = 0 as *mut TESSreal;
    (*tess).vertexIndices = 0 as *mut TESSindex;
    (*tess).vertexCount = 0i32;
    (*tess).elements = 0 as *mut TESSindex;
    (*tess).elementCount = 0i32;
    return tess;
}
#[no_mangle]
pub unsafe extern "C" fn tessDeleteTess(mut tess: *mut TESStesselator) {
    let mut alloc: TESSalloc = (*tess).alloc;
    deleteBucketAlloc((*tess).regionPool);
    if !(*tess).mesh.is_null() {
        tessMeshDeleteMesh(&mut alloc, (*tess).mesh);
        (*tess).mesh = 0 as *mut TESSmesh
    }
    if !(*tess).vertices.is_null() {
        alloc.memfree.expect("non-null function pointer")(alloc.userData,
                                                          (*tess).vertices as
                                                              *mut libc::c_void);
        (*tess).vertices = 0 as *mut TESSreal
    }
    if !(*tess).vertexIndices.is_null() {
        alloc.memfree.expect("non-null function pointer")(alloc.userData,
                                                          (*tess).vertexIndices
                                                              as
                                                              *mut libc::c_void);
        (*tess).vertexIndices = 0 as *mut TESSindex
    }
    if !(*tess).elements.is_null() {
        alloc.memfree.expect("non-null function pointer")(alloc.userData,
                                                          (*tess).elements as
                                                              *mut libc::c_void);
        (*tess).elements = 0 as *mut TESSindex
    }
    alloc.memfree.expect("non-null function pointer")(alloc.userData,
                                                      tess as
                                                          *mut libc::c_void);
}
unsafe extern "C" fn GetNeighbourFace(mut edge: *mut TESShalfEdge)
 -> TESSindex {
    if (*(*edge).Sym).Lface.is_null() { return !0i32 }
    if (*(*(*edge).Sym).Lface).inside == 0 { return !0i32 }
    return (*(*(*edge).Sym).Lface).n;
}
#[no_mangle]
pub unsafe extern "C" fn OutputPolymesh(mut tess: *mut TESStesselator,
                                        mut mesh: *mut TESSmesh,
                                        mut elementType: libc::c_int,
                                        mut polySize: libc::c_int,
                                        mut vertexSize: libc::c_int) {
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut edge: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut maxFaceCount: libc::c_int = 0i32;
    let mut maxVertexCount: libc::c_int = 0i32;
    let mut faceVerts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut elements: *mut TESSindex = 0 as *mut TESSindex;
    let mut vert: *mut TESSreal = 0 as *mut TESSreal;
    // Assume that the input data is triangles now.
	// Try to merge as many polygons as possible
    if polySize > 3i32 {
        if tessMeshMergeConvexFaces(mesh, polySize) == 0 {
            (*tess).outOfMemory = 1i32;
            return
        }
    }
    // Mark unused
    v = (*mesh).vHead.next;
    while v != &mut (*mesh).vHead as *mut TESSvertex {
        (*v).n = !0i32;
        v = (*v).next
    }
    // Create unique IDs for all vertices and faces.
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        (*f).n = !0i32;
        if !((*f).inside == 0) {
            edge = (*f).anEdge;
            faceVerts = 0i32;
            loop  {
                v = (*edge).Org;
                if (*v).n == !0i32 {
                    (*v).n = maxVertexCount;
                    maxVertexCount += 1
                }
                faceVerts += 1;
                edge = (*edge).Lnext;
                if !(edge != (*f).anEdge) { break ; }
            }
            if !(faceVerts <= polySize) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 15],
                                                       &[libc::c_char; 15]>(b"OutputPolymesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/tess.c\x00"
                                 as *const u8 as *const libc::c_char, 743i32,
                             b"faceVerts <= polySize\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            (*f).n = maxFaceCount;
            maxFaceCount += 1
        }
        f = (*f).next
    }
    (*tess).elementCount = maxFaceCount;
    if elementType == TESS_CONNECTED_POLYGONS as libc::c_int {
        maxFaceCount *= 2i32
    }
    (*tess).elements =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSindex>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(maxFaceCount
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_mul(polySize
                                                                                                                                        as
                                                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSindex;
    if (*tess).elements.is_null() { (*tess).outOfMemory = 1i32; return }
    (*tess).vertexCount = maxVertexCount;
    (*tess).vertices =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSreal>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((*tess).vertexCount
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_mul(vertexSize
                                                                                                                                        as
                                                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSreal;
    if (*tess).vertices.is_null() { (*tess).outOfMemory = 1i32; return }
    (*tess).vertexIndices =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSindex>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((*tess).vertexCount
                                                                                                        as
                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSindex;
    if (*tess).vertexIndices.is_null() { (*tess).outOfMemory = 1i32; return }
    // Output vertices.
    v = (*mesh).vHead.next;
    while v != &mut (*mesh).vHead as *mut TESSvertex {
        if (*v).n != !0i32 {
            // Store coordinate
            vert =
                &mut *(*tess).vertices.offset(((*v).n * vertexSize) as isize)
                    as *mut TESSreal;
            *vert.offset(0) = (*v).coords[0];
            *vert.offset(1) = (*v).coords[1];
            if vertexSize > 2i32 { *vert.offset(2) = (*v).coords[2] }
            // Store vertex index.
            *(*tess).vertexIndices.offset((*v).n as isize) = (*v).idx
        }
        v = (*v).next
    }
    // Output indices.
    elements = (*tess).elements;
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        if !((*f).inside == 0) {
            // Store polygon
            edge = (*f).anEdge;
            faceVerts = 0i32;
            loop  {
                v = (*edge).Org;
                let fresh2 = elements;
                elements = elements.offset(1);
                *fresh2 = (*v).n;
                faceVerts += 1;
                edge = (*edge).Lnext;
                if !(edge != (*f).anEdge) { break ; }
            }
            // Fill unused.
            i = faceVerts;
            while i < polySize {
                let fresh3 = elements;
                elements = elements.offset(1);
                *fresh3 = !0i32;
                i += 1
            }
            // Store polygon connectivity
            if elementType == TESS_CONNECTED_POLYGONS as libc::c_int {
                edge = (*f).anEdge;
                loop  {
                    let fresh4 = elements;
                    elements = elements.offset(1);
                    *fresh4 = GetNeighbourFace(edge);
                    edge = (*edge).Lnext;
                    if !(edge != (*f).anEdge) { break ; }
                }
                // Fill unused.
                i = faceVerts;
                while i < polySize {
                    let fresh5 = elements;
                    elements = elements.offset(1);
                    *fresh5 = !0i32;
                    i += 1
                }
            }
        }
        f = (*f).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn OutputContours(mut tess: *mut TESStesselator,
                                        mut mesh: *mut TESSmesh,
                                        mut vertexSize: libc::c_int) {
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut edge: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut start: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut verts: *mut TESSreal = 0 as *mut TESSreal;
    let mut elements: *mut TESSindex = 0 as *mut TESSindex;
    let mut vertInds: *mut TESSindex = 0 as *mut TESSindex;
    let mut startVert: libc::c_int = 0i32;
    let mut vertCount: libc::c_int = 0i32;
    (*tess).vertexCount = 0i32;
    (*tess).elementCount = 0i32;
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        if !((*f).inside == 0) {
            edge = (*f).anEdge;
            start = edge;
            loop  {
                (*tess).vertexCount += 1;
                edge = (*edge).Lnext;
                if !(edge != start) { break ; }
            }
            (*tess).elementCount += 1
        }
        f = (*f).next
    }
    (*tess).elements =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSindex>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((*tess).elementCount
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_mul(2i32
                                                                                                                                        as
                                                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSindex;
    if (*tess).elements.is_null() { (*tess).outOfMemory = 1i32; return }
    (*tess).vertices =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSreal>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((*tess).vertexCount
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_mul(vertexSize
                                                                                                                                        as
                                                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSreal;
    if (*tess).vertices.is_null() { (*tess).outOfMemory = 1i32; return }
    (*tess).vertexIndices =
        (*tess).alloc.memalloc.expect("non-null function pointer")((*tess).alloc.userData,
                                                                   (::std::mem::size_of::<TESSindex>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul((*tess).vertexCount
                                                                                                        as
                                                                                                        libc::c_ulong)
                                                                       as
                                                                       libc::c_uint)
            as *mut TESSindex;
    if (*tess).vertexIndices.is_null() { (*tess).outOfMemory = 1i32; return }
    verts = (*tess).vertices;
    elements = (*tess).elements;
    vertInds = (*tess).vertexIndices;
    startVert = 0i32;
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        if !((*f).inside == 0) {
            vertCount = 0i32;
            edge = (*f).anEdge;
            start = edge;
            loop  {
                let fresh6 = verts;
                verts = verts.offset(1);
                *fresh6 = (*(*edge).Org).coords[0];
                let fresh7 = verts;
                verts = verts.offset(1);
                *fresh7 = (*(*edge).Org).coords[1];
                if vertexSize > 2i32 {
                    let fresh8 = verts;
                    verts = verts.offset(1);
                    *fresh8 = (*(*edge).Org).coords[2]
                }
                let fresh9 = vertInds;
                vertInds = vertInds.offset(1);
                *fresh9 = (*(*edge).Org).idx;
                vertCount += 1;
                edge = (*edge).Lnext;
                if !(edge != start) { break ; }
            }
            *elements.offset(0) = startVert;
            *elements.offset(1) = vertCount;
            elements = elements.offset(2);
            startVert += vertCount
        }
        f = (*f).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn tessAddContour(mut tess: *mut TESStesselator,
                                        mut size: libc::c_int,
                                        mut vertices: *const libc::c_void,
                                        mut stride: libc::c_int,
                                        mut numVertices: libc::c_int) {
    let mut src: *const libc::c_uchar = vertices as *const libc::c_uchar;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut i: libc::c_int = 0;
    if (*tess).mesh.is_null() {
        (*tess).mesh = tessMeshNewMesh(&mut (*tess).alloc)
    }
    if (*tess).mesh.is_null() { (*tess).outOfMemory = 1i32; return }
    if size < 2i32 { size = 2i32 }
    if size > 3i32 { size = 3i32 }
    e = 0 as *mut TESShalfEdge;
    i = 0i32;
    while i < numVertices {
        let mut coords: *const TESSreal = src as *const TESSreal;
        src = src.offset(stride as isize);
        if e.is_null() {
            /* Make a self-loop (one vertex, one edge). */
            e = tessMeshMakeEdge((*tess).mesh);
            if e.is_null() { (*tess).outOfMemory = 1i32; return }
            if tessMeshSplice((*tess).mesh, e, (*e).Sym) == 0 {
                (*tess).outOfMemory = 1i32;
                return
            }
        } else {
            /* Create a new vertex and edge which immediately follow e
			* in the ordering around the left face.
			*/
            if tessMeshSplitEdge((*tess).mesh, e).is_null() {
                (*tess).outOfMemory = 1i32;
                return
            }
            e = (*e).Lnext
        }
        /* The new vertex is now e->Org. */
        (*(*e).Org).coords[0] = *coords.offset(0);
        (*(*e).Org).coords[1] = *coords.offset(1);
        if size > 2i32 {
            (*(*e).Org).coords[2] = *coords.offset(2)
        } else { (*(*e).Org).coords[2] = 0i32 as TESSreal }
        /* Store the insertion number so that the vertex can be later recognized. */
        let fresh10 = (*tess).vertexIndexCounter;
        (*tess).vertexIndexCounter = (*tess).vertexIndexCounter + 1;
        (*(*e).Org).idx = fresh10;
        /* The winding of an edge says how the winding number changes as we
		* cross from the edge''s right face to its left face.  We add the
		* vertices in such an order that a CCW contour will add +1 to
		* the winding number of the region inside the contour.
		*/
        (*e).winding =
            if (*tess).reverseContours != 0 { -1i32 } else { 1i32 };
        (*(*e).Sym).winding =
            if (*tess).reverseContours != 0 { 1i32 } else { -1i32 };
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn tessSetOption(mut tess: *mut TESStesselator,
                                       mut option: libc::c_int,
                                       mut value: libc::c_int) {
    match option {
        0 => { (*tess).processCDT = if value > 0i32 { 1i32 } else { 0i32 } }
        1 => {
            (*tess).reverseContours = if value > 0i32 { 1i32 } else { 0i32 }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tessTesselate(mut tess: *mut TESStesselator,
                                       mut windingRule: libc::c_int,
                                       mut elementType: libc::c_int,
                                       mut polySize: libc::c_int,
                                       mut vertexSize: libc::c_int,
                                       mut normal: *const TESSreal)
 -> libc::c_int {
    let mut mesh: *mut TESSmesh = 0 as *mut TESSmesh;
    let mut rc: libc::c_int = 1i32;
    if !(*tess).vertices.is_null() {
        (*tess).alloc.memfree.expect("non-null function pointer")((*tess).alloc.userData,
                                                                  (*tess).vertices
                                                                      as
                                                                      *mut libc::c_void);
        (*tess).vertices = 0 as *mut TESSreal
    }
    if !(*tess).elements.is_null() {
        (*tess).alloc.memfree.expect("non-null function pointer")((*tess).alloc.userData,
                                                                  (*tess).elements
                                                                      as
                                                                      *mut libc::c_void);
        (*tess).elements = 0 as *mut TESSindex
    }
    if !(*tess).vertexIndices.is_null() {
        (*tess).alloc.memfree.expect("non-null function pointer")((*tess).alloc.userData,
                                                                  (*tess).vertexIndices
                                                                      as
                                                                      *mut libc::c_void);
        (*tess).vertexIndices = 0 as *mut TESSindex
    }
    (*tess).vertexIndexCounter = 0i32;
    if !normal.is_null() {
        (*tess).normal[0] = *normal.offset(0);
        (*tess).normal[1] = *normal.offset(1);
        (*tess).normal[2] = *normal.offset(2)
    }
    (*tess).windingRule = windingRule;
    if vertexSize < 2i32 { vertexSize = 2i32 }
    if vertexSize > 3i32 { vertexSize = 3i32 }
    if setjmp((*tess).env.as_mut_ptr()) != 0i32 {
        /* come back here if out of memory */
        return 0i32
    }
    if (*tess).mesh.is_null() { return 0i32 }
    /* Determine the polygon normal and project vertices onto the plane
	* of the polygon.
	*/
    tessProjectPolygon(tess);
    /* tessComputeInterior( tess ) computes the planar arrangement specified
	* by the given contours, and further subdivides this arrangement
	* into regions.  Each region is marked "inside" if it belongs
	* to the polygon, according to the rule given by tess->windingRule.
	* Each interior region is guaranteed be monotone.
	*/
    if tessComputeInterior(tess) == 0 {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
        /* could've used a label */
    }
    mesh = (*tess).mesh;
    /* If the user wants only the boundary contours, we throw away all edges
	* except those which separate the interior from the exterior.
	* Otherwise we tessellate all the regions marked "inside".
	*/
    if elementType == TESS_BOUNDARY_CONTOURS as libc::c_int {
        rc = tessMeshSetWindingNumber(mesh, 1i32, 1i32)
    } else {
        rc = tessMeshTessellateInterior(mesh); /* could've used a label */
        if rc != 0i32 && (*tess).processCDT != 0i32 {
            tessMeshRefineDelaunay(mesh, &mut (*tess).alloc);
        }
    }
    if rc == 0i32 { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    tessMeshCheckMesh(mesh);
    if elementType == TESS_BOUNDARY_CONTOURS as libc::c_int {
        OutputContours(tess, mesh, vertexSize);
        /* output contours */
    } else {
        OutputPolymesh(tess, mesh, elementType, polySize, vertexSize);
        /* output polygons */
    }
    tessMeshDeleteMesh(&mut (*tess).alloc, mesh);
    (*tess).mesh = 0 as *mut TESSmesh;
    if (*tess).outOfMemory != 0 { return 0i32 }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tessGetVertexCount(mut tess: *mut TESStesselator)
 -> libc::c_int {
    return (*tess).vertexCount;
}
#[no_mangle]
pub unsafe extern "C" fn tessGetVertices(mut tess: *mut TESStesselator)
 -> *const TESSreal {
    return (*tess).vertices;
}
#[no_mangle]
pub unsafe extern "C" fn tessGetVertexIndices(mut tess: *mut TESStesselator)
 -> *const TESSindex {
    return (*tess).vertexIndices;
}
#[no_mangle]
pub unsafe extern "C" fn tessGetElementCount(mut tess: *mut TESStesselator)
 -> libc::c_int {
    return (*tess).elementCount;
}
// tessDeleteTess() - Deletes a tesselator.
// Parameters:
//   tess - pointer to tesselator object to be deleted.
// tessAddContour() - Adds a contour to be tesselated.
// The type of the vertex coordinates is assumed to be TESSreal.
// Parameters:
//   tess - pointer to tesselator object.
//   size - number of coordinates per vertex. Must be 2 or 3.
//   pointer - pointer to the first coordinate of the first vertex in the array.
//   stride - defines offset in bytes between consecutive vertices.
//   count - number of vertices in contour.
// tessSetOption() - Toggles optional tessellation parameters
// Parameters:
//  option - one of TessOption
//  value - 1 if enabled, 0 if disabled.
// tessTesselate() - tesselate contours.
// Parameters:
//   tess - pointer to tesselator object.
//   windingRule - winding rules used for tesselation, must be one of TessWindingRule.
//   elementType - defines the tesselation result element type, must be one of TessElementType.
//   polySize - defines maximum vertices per polygons if output is polygons.
//   vertexSize - defines the number of coordinates in tesselation result vertex, must be 2 or 3.
//   normal - defines the normal of the input contours, of null the normal is calculated automatically.
// Returns:
//   1 if succeed, 0 if failed.
// tessGetVertexCount() - Returns number of vertices in the tesselated output.
// tessGetVertices() - Returns pointer to first coordinate of first vertex.
// tessGetVertexIndices() - Returns pointer to first vertex index.
// Vertex indices can be used to map the generated vertices to the original vertices.
// Every point added using tessAddContour() will get a new index starting at 0.
// New vertices generated at the intersections of segments are assigned value TESS_UNDEF.
// tessGetElementCount() - Returns number of elements in the the tesselated output.
// tessGetElements() - Returns pointer to the first element.
#[no_mangle]
pub unsafe extern "C" fn tessGetElements(mut tess: *mut TESStesselator)
 -> *const TESSindex {
    return (*tess).elements;
}
