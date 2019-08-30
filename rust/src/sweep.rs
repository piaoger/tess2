use libc;
extern "C" {
    pub type BucketAlloc;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
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
    fn tessMeshMakeEdge(mesh: *mut TESSmesh) -> *mut TESShalfEdge;
    #[no_mangle]
    fn tessMeshSplice(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge,
                      eDst: *mut TESShalfEdge) -> libc::c_int;
    #[no_mangle]
    fn tessMeshDelete(mesh: *mut TESSmesh, eDel: *mut TESShalfEdge)
     -> libc::c_int;
    #[no_mangle]
    fn tessMeshCheckMesh(mesh: *mut TESSmesh);
    #[no_mangle]
    fn tessMeshConnect(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge,
                       eDst: *mut TESShalfEdge) -> *mut TESShalfEdge;
    #[no_mangle]
    fn tessMeshSplitEdge(mesh: *mut TESSmesh, eOrg: *mut TESShalfEdge)
     -> *mut TESShalfEdge;
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
** Author: Eric Veach, July 1994.
*/
    /* Versions of VertLeq, EdgeSign, EdgeEval with s and t transposed. */
    #[no_mangle]
    fn tesvertLeq(u: *mut TESSvertex, v: *mut TESSvertex) -> libc::c_int;
    #[no_mangle]
    fn tesedgeEval(u: *mut TESSvertex, v: *mut TESSvertex, w: *mut TESSvertex)
     -> TESSreal;
    #[no_mangle]
    fn tesedgeSign(u: *mut TESSvertex, v: *mut TESSvertex, w: *mut TESSvertex)
     -> TESSreal;
    #[no_mangle]
    fn tesedgeIntersect(o1: *mut TESSvertex, d1: *mut TESSvertex,
                        o2: *mut TESSvertex, d2: *mut TESSvertex,
                        v: *mut TESSvertex);
    #[no_mangle]
    fn dictNewDict(alloc: *mut TESSalloc, frame: *mut libc::c_void,
                   leq:
                       Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                   _: DictKey, _: DictKey)
                                  -> libc::c_int>) -> *mut Dict;
    #[no_mangle]
    fn dictDeleteDict(alloc: *mut TESSalloc, dict: *mut Dict);
    #[no_mangle]
    fn bucketAlloc(ba: *mut BucketAlloc) -> *mut libc::c_void;
    #[no_mangle]
    fn bucketFree(ba: *mut BucketAlloc, ptr: *mut libc::c_void);
    #[no_mangle]
    fn dictSearch(dict: *mut Dict, key: DictKey) -> *mut DictNode;
    #[no_mangle]
    fn dictInsertBefore(dict: *mut Dict, node: *mut DictNode, key: DictKey)
     -> *mut DictNode;
    #[no_mangle]
    fn dictDelete(dict: *mut Dict, node: *mut DictNode);
    #[no_mangle]
    fn pqNewPriorityQ(alloc: *mut TESSalloc, size: libc::c_int,
                      leq:
                          Option<unsafe extern "C" fn(_: PQkey, _: PQkey)
                                     -> libc::c_int>) -> *mut PriorityQ;
    #[no_mangle]
    fn pqDeletePriorityQ(alloc: *mut TESSalloc, pq: *mut PriorityQ);
    #[no_mangle]
    fn pqInit(alloc: *mut TESSalloc, pq: *mut PriorityQ) -> libc::c_int;
    #[no_mangle]
    fn pqInsert(alloc: *mut TESSalloc, pq: *mut PriorityQ, key: PQkey)
     -> PQhandle;
    #[no_mangle]
    fn pqExtractMin(pq: *mut PriorityQ) -> PQkey;
    #[no_mangle]
    fn pqDelete(pq: *mut PriorityQ, handle: PQhandle);
    #[no_mangle]
    fn pqMinimum(pq: *mut PriorityQ) -> PQkey;
}
pub type jmp_buf = [libc::c_int; 37];
pub type TessWindingRule = libc::c_uint;
pub const TESS_WINDING_ABS_GEQ_TWO: TessWindingRule = 4;
pub const TESS_WINDING_NEGATIVE: TessWindingRule = 3;
pub const TESS_WINDING_POSITIVE: TessWindingRule = 2;
pub const TESS_WINDING_NONZERO: TessWindingRule = 1;
pub const TESS_WINDING_ODD: TessWindingRule = 0;
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
** Author: Eric Veach, July 1994.
*/
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
** Author: Eric Veach, July 1994.
*/
/* The basic operations are insertion of a new key (pqInsert),
* and examination/extraction of a key whose value is minimum
* (pqMinimum/pqExtractMin).  Deletion is also allowed (pqDelete);
* for this purpose pqInsert returns a "handle" which is supplied
* as the argument.
*
* An initial heap may be created efficiently by calling pqInsert
* repeatedly, then calling pqInit.  In any case pqInit must be called
* before any operations other than pqInsert are used.
*
* If the heap is empty, pqMinimum/pqExtractMin will return a NULL key.
* This may also be tested with pqIsEmpty.
*/
/* Since we support deletion the data structure is a little more
* complicated than an ordinary heap.  "nodes" is the heap itself;
* active nodes are stored in the range 1..pq->size.  When the
* heap exceeds its allocated size (pq->max), its size doubles.
* The children of node i are nodes 2i and 2i+1.
*
* Each node stores an index into an array "handles".  Each handle
* stores a key, plus a pointer back to the node which currently
* represents that key (ie. nodes[handles[i].node].handle == i).
*/
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
unsafe extern "C" fn EdgeLeq(mut tess: *mut TESStesselator,
                             mut reg1: *mut ActiveRegion,
                             mut reg2: *mut ActiveRegion) -> libc::c_int 
 /*
* Both edges must be directed from right to left (this is the canonical
* direction for the upper edge of each region).
*
* The strategy is to evaluate a "t" value for each edge at the
* current sweep line position, given by tess->event.  The calculations
* are designed to be very stable, but of course they are not perfect.
*
* Special case: if both edge destinations are at the sweep event,
* we sort the edges by slope (they would otherwise compare equally).
*/
 {
    let mut event: *mut TESSvertex = (*tess).event;
    let mut e1: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut e2: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut t1: TESSreal = 0.;
    let mut t2: TESSreal = 0.;
    e1 = (*reg1).eUp;
    e2 = (*reg2).eUp;
    if (*(*e1).Sym).Org == event {
        if (*(*e2).Sym).Org == event {
            /* Two edges right of the sweep line which meet at the sweep event.
			* Sort them by slope.
			*/
            if (*(*e1).Org).s < (*(*e2).Org).s ||
                   (*(*e1).Org).s == (*(*e2).Org).s &&
                       (*(*e1).Org).t <= (*(*e2).Org).t {
                return (tesedgeSign((*(*e2).Sym).Org, (*e1).Org, (*e2).Org) <=
                            0i32 as libc::c_float) as libc::c_int
            }
            return (tesedgeSign((*(*e1).Sym).Org, (*e2).Org, (*e1).Org) >=
                        0i32 as libc::c_float) as libc::c_int
        }
        return (tesedgeSign((*(*e2).Sym).Org, event, (*e2).Org) <=
                    0i32 as libc::c_float) as libc::c_int
    }
    if (*(*e2).Sym).Org == event {
        return (tesedgeSign((*(*e1).Sym).Org, event, (*e1).Org) >=
                    0i32 as libc::c_float) as libc::c_int
    }
    /* General case - compute signed distance *from* e1, e2 to event */
    t1 = tesedgeEval((*(*e1).Sym).Org, event, (*e1).Org);
    t2 = tesedgeEval((*(*e2).Sym).Org, event, (*e2).Org);
    return (t1 >= t2) as libc::c_int;
}
unsafe extern "C" fn DeleteRegion(mut tess: *mut TESStesselator,
                                  mut reg: *mut ActiveRegion) {
    if (*reg).fixUpperEdge != 0 {
        /* It was created with zero winding number, so it better be
		* deleted with zero winding number (ie. it better not get merged
		* with a real edge).
		*/
        if !((*(*reg).eUp).winding == 0i32) as libc::c_int as libc::c_long !=
               0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                   &[libc::c_char; 13]>(b"DeleteRegion\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 146i32,
                         b"reg->eUp->winding == 0\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
    }
    (*(*reg).eUp).activeRegion = 0 as *mut ActiveRegion;
    dictDelete((*tess).dict, (*reg).nodeUp);
    bucketFree((*tess).regionPool, reg as *mut libc::c_void);
}
unsafe extern "C" fn FixUpperEdge(mut tess: *mut TESStesselator,
                                  mut reg: *mut ActiveRegion,
                                  mut newEdge: *mut TESShalfEdge)
 -> libc::c_int 
 /*
* Replace an upper edge which needs fixing (see ConnectRightVertex).
*/
 {
    if ((*reg).fixUpperEdge == 0) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"FixUpperEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 159i32,
                     b"reg->fixUpperEdge\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if tessMeshDelete((*tess).mesh, (*reg).eUp) == 0 { return 0i32 }
    (*reg).fixUpperEdge = 0i32;
    (*reg).eUp = newEdge;
    (*newEdge).activeRegion = reg;
    return 1i32;
}
unsafe extern "C" fn TopLeftRegion(mut tess: *mut TESStesselator,
                                   mut reg: *mut ActiveRegion)
 -> *mut ActiveRegion {
    let mut org: *mut TESSvertex = (*(*reg).eUp).Org;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    loop 
         /* Find the region above the uppermost edge with the same origin */
         {
        reg = (*(*(*reg).nodeUp).next).key as *mut ActiveRegion;
        if !((*(*reg).eUp).Org == org) { break ; }
    }
    /* If the edge above was a temporary edge introduced by ConnectRightVertex,
	* now is the time to fix it.
	*/
    if (*reg).fixUpperEdge != 0 {
        e =
            tessMeshConnect((*tess).mesh,
                            (*(*((*(*(*reg).nodeUp).prev).key as
                                     *mut ActiveRegion)).eUp).Sym,
                            (*(*reg).eUp).Lnext);
        if e.is_null() { return 0 as *mut ActiveRegion }
        if FixUpperEdge(tess, reg, e) == 0 { return 0 as *mut ActiveRegion }
        reg = (*(*(*reg).nodeUp).next).key as *mut ActiveRegion
    }
    return reg;
}
unsafe extern "C" fn TopRightRegion(mut reg: *mut ActiveRegion)
 -> *mut ActiveRegion {
    let mut dst: *mut TESSvertex = (*(*(*reg).eUp).Sym).Org;
    loop 
         /* Find the region above the uppermost edge with the same destination */
         {
        reg = (*(*(*reg).nodeUp).next).key as *mut ActiveRegion;
        if !((*(*(*reg).eUp).Sym).Org == dst) { break ; }
    }
    return reg;
}
unsafe extern "C" fn AddRegionBelow(mut tess: *mut TESStesselator,
                                    mut regAbove: *mut ActiveRegion,
                                    mut eNewUp: *mut TESShalfEdge)
 -> *mut ActiveRegion 
 /*
* Add a new active region to the sweep line, *somewhere* below "regAbove"
* (according to where the new edge belongs in the sweep-line dictionary).
* The upper edge of the new region will be "eNewUp".
* Winding number and "inside" flag are not updated.
*/
 {
    let mut regNew: *mut ActiveRegion =
        bucketAlloc((*tess).regionPool) as *mut ActiveRegion;
    if regNew.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    (*regNew).eUp = eNewUp;
    (*regNew).nodeUp =
        dictInsertBefore((*tess).dict, (*regAbove).nodeUp, regNew as DictKey);
    if (*regNew).nodeUp.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    (*regNew).fixUpperEdge = 0i32;
    (*regNew).sentinel = 0i32;
    (*regNew).dirty = 0i32;
    (*eNewUp).activeRegion = regNew;
    return regNew;
}
unsafe extern "C" fn IsWindingInside(mut tess: *mut TESStesselator,
                                     mut n: libc::c_int) -> libc::c_int {
    match (*tess).windingRule {
        0 => { return n & 1i32 }
        1 => { return (n != 0i32) as libc::c_int }
        2 => { return (n > 0i32) as libc::c_int }
        3 => { return (n < 0i32) as libc::c_int }
        4 => { return (n >= 2i32 || n <= -2i32) as libc::c_int }
        _ => { }
    }
    /*LINTED*/
    if (0i32 == 0) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 16],
                                               &[libc::c_char; 16]>(b"IsWindingInside\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 240i32,
                     b"FALSE\x00" as *const u8 as *const libc::c_char);
    } else { };
    /*NOTREACHED*/
    return 0i32;
}
unsafe extern "C" fn ComputeWinding(mut tess: *mut TESStesselator,
                                    mut reg: *mut ActiveRegion) {
    (*reg).windingNumber =
        (*((*(*(*reg).nodeUp).next).key as *mut ActiveRegion)).windingNumber +
            (*(*reg).eUp).winding;
    (*reg).inside = IsWindingInside(tess, (*reg).windingNumber);
}
unsafe extern "C" fn FinishRegion(mut tess: *mut TESStesselator,
                                  mut reg: *mut ActiveRegion) 
 /*
* Delete a region from the sweep line.  This happens when the upper
* and lower chains of a region meet (at a vertex on the sweep line).
* The "inside" flag is copied to the appropriate mesh face (we could
* not do this before -- since the structure of the mesh is always
* changing, this face may not have even existed until now).
*/
 {
    let mut e: *mut TESShalfEdge =
        (*reg).eUp; /* optimization for tessMeshTessellateMonoRegion() */
    let mut f: *mut TESSface = (*e).Lface;
    (*f).inside = (*reg).inside as libc::c_char;
    (*f).anEdge = e;
    DeleteRegion(tess, reg);
}
unsafe extern "C" fn FinishLeftRegions(mut tess: *mut TESStesselator,
                                       mut regFirst: *mut ActiveRegion,
                                       mut regLast: *mut ActiveRegion)
 -> *mut TESShalfEdge 
 /*
* We are given a vertex with one or more left-going edges.  All affected
* edges should be in the edge dictionary.  Starting at regFirst->eUp,
* we walk down deleting all regions where both edges have the same
* origin vOrg.  At the same time we copy the "inside" flag from the
* active region to the face, since at this point each face will belong
* to at most one region (this was not necessarily true until this point
* in the sweep).  The walk stops at the region above regLast; if regLast
* is NULL we walk as far as possible.  At the same time we relink the
* mesh if necessary, so that the ordering of edges around vOrg is the
* same as in the dictionary.
*/
 {
    let mut reg: *mut ActiveRegion =
        0 as *mut ActiveRegion; /* placement was OK */
    let mut regPrev: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut ePrev: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    regPrev = regFirst;
    ePrev = (*regFirst).eUp;
    while regPrev != regLast {
        (*regPrev).fixUpperEdge = 0i32;
        reg = (*(*(*regPrev).nodeUp).prev).key as *mut ActiveRegion;
        e = (*reg).eUp;
        if (*e).Org != (*ePrev).Org {
            if (*reg).fixUpperEdge == 0 {
                /* Remove the last left-going edge.  Even though there are no further
				* edges in the dictionary with this origin, there may be further
				* such edges in the mesh (if we are adding left edges to a vertex
				* that has already been processed).  Thus it is important to call
				* FinishRegion rather than just DeleteRegion.
				*/
                FinishRegion(tess, regPrev);
                break ;
            } else {
                /* If the edge below was a temporary edge introduced by
			* ConnectRightVertex, now is the time to fix it.
			*/
                e =
                    tessMeshConnect((*tess).mesh, (*(*ePrev).Onext).Sym,
                                    (*e).Sym);
                if e.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
                if FixUpperEdge(tess, reg, e) == 0 {
                    longjmp((*tess).env.as_mut_ptr(), 1i32);
                }
            }
        }
        /* Relink edges so that ePrev->Onext == e */
        if (*ePrev).Onext != e {
            if tessMeshSplice((*tess).mesh, (*(*e).Sym).Lnext, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(),
                        1i32); /* may change reg->eUp */
            }
            if tessMeshSplice((*tess).mesh, ePrev, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
        }
        FinishRegion(tess, regPrev);
        ePrev = (*reg).eUp;
        regPrev = reg
    }
    return ePrev;
}
unsafe extern "C" fn AddRightEdges(mut tess: *mut TESStesselator,
                                   mut regUp: *mut ActiveRegion,
                                   mut eFirst: *mut TESShalfEdge,
                                   mut eLast: *mut TESShalfEdge,
                                   mut eTopLeft: *mut TESShalfEdge,
                                   mut cleanUp: libc::c_int) 
 /*
* Purpose: insert right-going edges into the edge dictionary, and update
* winding numbers and mesh connectivity appropriately.  All right-going
* edges share a common origin vOrg.  Edges are inserted CCW starting at
* eFirst; the last edge inserted is eLast->Oprev.  If vOrg has any
* left-going edges already processed, then eTopLeft must be the edge
* such that an imaginary upward vertical segment from vOrg would be
* contained between eTopLeft->Oprev and eTopLeft; otherwise eTopLeft
* should be NULL.
*/
 {
    let mut reg: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut regPrev: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut ePrev: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut firstTime: libc::c_int = 1i32;
    /* Insert the new right-going edges in the dictionary */
    e = eFirst;
    loop  {
        if !((*(*e).Org).s < (*(*(*e).Sym).Org).s ||
                 (*(*e).Org).s == (*(*(*e).Sym).Org).s &&
                     (*(*e).Org).t <= (*(*(*e).Sym).Org).t) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                                   &[libc::c_char; 14]>(b"AddRightEdges\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 349i32,
                         b"VertLeq( e->Org, e->Dst )\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        AddRegionBelow(tess, regUp, (*e).Sym);
        e = (*e).Onext;
        if !(e != eLast) { break ; }
    }
    /* Walk *all* right-going edges from e->Org, in the dictionary order,
	* updating the winding numbers of each region, and re-linking the mesh
	* edges to match the dictionary ordering (if necessary).
	*/
    if eTopLeft.is_null() {
        eTopLeft =
            (*(*(*((*(*(*regUp).nodeUp).prev).key as
                       *mut ActiveRegion)).eUp).Sym).Onext
    }
    regPrev = regUp;
    ePrev = eTopLeft;
    loop  {
        reg = (*(*(*regPrev).nodeUp).prev).key as *mut ActiveRegion;
        e = (*(*reg).eUp).Sym;
        if (*e).Org != (*ePrev).Org { break ; }
        if (*e).Onext != ePrev {
            /* Unlink e from its current position, and relink below ePrev */
            if tessMeshSplice((*tess).mesh, (*(*e).Sym).Lnext, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            if tessMeshSplice((*tess).mesh, (*(*ePrev).Sym).Lnext, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
        }
        /* Compute the winding number and "inside" flag for the new regions */
        (*reg).windingNumber = (*regPrev).windingNumber - (*e).winding;
        (*reg).inside = IsWindingInside(tess, (*reg).windingNumber);
        /* Check for two outgoing edges with same slope -- process these
		* before any intersection tests (see example in tessComputeInterior).
		*/
        (*regPrev).dirty = 1i32;
        if firstTime == 0 && CheckForRightSplice(tess, regPrev) != 0 {
            (*e).winding += (*ePrev).winding;
            (*(*e).Sym).winding += (*(*ePrev).Sym).winding;
            DeleteRegion(tess, regPrev);
            if tessMeshDelete((*tess).mesh, ePrev) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
        }
        firstTime = 0i32;
        regPrev = reg;
        ePrev = e
    }
    (*regPrev).dirty = 1i32;
    if !((*regPrev).windingNumber - (*e).winding == (*reg).windingNumber) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                               &[libc::c_char; 14]>(b"AddRightEdges\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 391i32,
                     b"regPrev->windingNumber - e->winding == reg->windingNumber\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    if cleanUp != 0 {
        /* Check for intersections between newly adjacent edges. */
        WalkDirtyRegions(tess, regPrev);
    };
}
unsafe extern "C" fn SpliceMergeVertices(mut tess: *mut TESStesselator,
                                         mut e1: *mut TESShalfEdge,
                                         mut e2: *mut TESShalfEdge) 
 /*
* Two vertices with idential coordinates are combined into one.
* e1->Org is kept, while e2->Org is discarded.
*/
 {
    if tessMeshSplice((*tess).mesh, e1, e2) == 0 {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    };
}
unsafe extern "C" fn VertexWeights(mut isect: *mut TESSvertex,
                                   mut org: *mut TESSvertex,
                                   mut dst: *mut TESSvertex,
                                   mut weights: *mut TESSreal) 
 /*
* Find some weights which describe how the intersection vertex is
* a linear combination of "org" and "dest".  Each of the two edges
* which generated "isect" is allocated 50% of the weight; each edge
* splits the weight between its org and dst according to the
* relative distance to "isect".
*/
 {
    let mut t1: TESSreal =
        (if (*org).s - (*isect).s < 0i32 as libc::c_float {
             -((*org).s - (*isect).s)
         } else { (*org).s - (*isect).s }) +
            (if (*org).t - (*isect).t < 0i32 as libc::c_float {
                 -((*org).t - (*isect).t)
             } else { (*org).t - (*isect).t });
    let mut t2: TESSreal =
        (if (*dst).s - (*isect).s < 0i32 as libc::c_float {
             -((*dst).s - (*isect).s)
         } else { (*dst).s - (*isect).s }) +
            (if (*dst).t - (*isect).t < 0i32 as libc::c_float {
                 -((*dst).t - (*isect).t)
             } else { (*dst).t - (*isect).t });
    *weights.offset(0) = 0.5f64 as TESSreal * t2 / (t1 + t2);
    *weights.offset(1) = 0.5f64 as TESSreal * t1 / (t1 + t2);
    (*isect).coords[0] +=
        *weights.offset(0) * (*org).coords[0] +
            *weights.offset(1) * (*dst).coords[0];
    (*isect).coords[1] +=
        *weights.offset(0) * (*org).coords[1] +
            *weights.offset(1) * (*dst).coords[1];
    (*isect).coords[2] +=
        *weights.offset(0) * (*org).coords[2] +
            *weights.offset(1) * (*dst).coords[2];
}
unsafe extern "C" fn GetIntersectData(mut tess: *mut TESStesselator,
                                      mut isect: *mut TESSvertex,
                                      mut orgUp: *mut TESSvertex,
                                      mut dstUp: *mut TESSvertex,
                                      mut orgLo: *mut TESSvertex,
                                      mut dstLo: *mut TESSvertex) 
 /*
 * We've computed a new intersection point, now we need a "data" pointer
 * from the user so that we can refer to this new vertex in the
 * rendering callbacks.
 */
 {
    let mut weights: [TESSreal; 4] = [0.; 4];
    if 1i32 != 0 { } else { };
    (*isect).coords[2] = 0i32 as TESSreal;
    (*isect).coords[1] = (*isect).coords[2];
    (*isect).coords[0] = (*isect).coords[1];
    (*isect).idx = !0i32;
    VertexWeights(isect, orgUp, dstUp, &mut *weights.as_mut_ptr().offset(0));
    VertexWeights(isect, orgLo, dstLo, &mut *weights.as_mut_ptr().offset(2));
}
unsafe extern "C" fn CheckForRightSplice(mut tess: *mut TESStesselator,
                                         mut regUp: *mut ActiveRegion)
 -> libc::c_int 
 /*
* Check the upper and lower edge of "regUp", to make sure that the
* eUp->Org is above eLo, or eLo->Org is below eUp (depending on which
* origin is leftmost).
*
* The main purpose is to splice right-going edges with the same
* dest vertex and nearly identical slopes (ie. we can't distinguish
* the slopes numerically).  However the splicing can also help us
* to recover from numerical errors.  For example, suppose at one
* point we checked eUp and eLo, and decided that eUp->Org is barely
* above eLo.  Then later, we split eLo into two edges (eg. from
* a splice operation like this one).  This can change the result of
* our test so that now eUp->Org is incident to eLo, or barely below it.
* We must correct this condition to maintain the dictionary invariants.
*
* One possibility is to check these edges for intersection again
* (ie. CheckForIntersect).  This is what we do if possible.  However
* CheckForIntersect requires that tess->event lies between eUp and eLo,
* so that it has something to fall back on when the intersection
* calculation gives us an unusable answer.  So, for those cases where
* we can't check for intersection, this routine fixes the problem
* by just splicing the offending vertex into the other edge.
* This is a guaranteed solution, no matter how degenerate things get.
* Basically this is a combinatorial solution to a numerical problem.
*/
 {
    let mut regLo: *mut ActiveRegion =
        (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    let mut eUp: *mut TESShalfEdge = (*regUp).eUp;
    let mut eLo: *mut TESShalfEdge = (*regLo).eUp;
    if (*(*eUp).Org).s < (*(*eLo).Org).s ||
           (*(*eUp).Org).s == (*(*eLo).Org).s &&
               (*(*eUp).Org).t <= (*(*eLo).Org).t {
        if tesedgeSign((*(*eLo).Sym).Org, (*eUp).Org, (*eLo).Org) >
               0i32 as libc::c_float {
            return 0i32
        }
        /* eUp->Org appears to be below eLo */
        if !((*(*eUp).Org).s == (*(*eLo).Org).s &&
                 (*(*eUp).Org).t == (*(*eLo).Org).t) {
            /* Splice eUp->Org into eLo */
            if tessMeshSplitEdge((*tess).mesh, (*eLo).Sym).is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            if tessMeshSplice((*tess).mesh, eUp, (*(*eLo).Sym).Lnext) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            (*regLo).dirty = 1i32;
            (*regUp).dirty = (*regLo).dirty
        } else if (*eUp).Org != (*eLo).Org {
            /* merge the two vertices, discarding eUp->Org */
            pqDelete((*tess).pq, (*(*eUp).Org).pqHandle);
            SpliceMergeVertices(tess, (*(*eLo).Sym).Lnext, eUp);
        }
    } else {
        if tesedgeSign((*(*eUp).Sym).Org, (*eLo).Org, (*eUp).Org) <=
               0i32 as libc::c_float {
            return 0i32
        }
        /* eLo->Org appears to be above eUp, so splice eLo->Org into eUp */
        (*regUp).dirty = 1i32;
        (*((*(*(*regUp).nodeUp).next).key as *mut ActiveRegion)).dirty =
            (*regUp).dirty;
        if tessMeshSplitEdge((*tess).mesh, (*eUp).Sym).is_null() {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        if tessMeshSplice((*tess).mesh, (*(*eLo).Sym).Lnext, eUp) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
    }
    return 1i32;
}
unsafe extern "C" fn CheckForLeftSplice(mut tess: *mut TESStesselator,
                                        mut regUp: *mut ActiveRegion)
 -> libc::c_int 
 /*
* Check the upper and lower edge of "regUp", to make sure that the
* eUp->Dst is above eLo, or eLo->Dst is below eUp (depending on which
* destination is rightmost).
*
* Theoretically, this should always be true.  However, splitting an edge
* into two pieces can change the results of previous tests.  For example,
* suppose at one point we checked eUp and eLo, and decided that eUp->Dst
* is barely above eLo.  Then later, we split eLo into two edges (eg. from
* a splice operation like this one).  This can change the result of
* the test so that now eUp->Dst is incident to eLo, or barely below it.
* We must correct this condition to maintain the dictionary invariants
* (otherwise new edges might get inserted in the wrong place in the
* dictionary, and bad stuff will happen).
*
* We fix the problem by just splicing the offending vertex into the
* other edge.
*/
 {
    let mut regLo: *mut ActiveRegion =
        (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    let mut eUp: *mut TESShalfEdge = (*regUp).eUp;
    let mut eLo: *mut TESShalfEdge = (*regLo).eUp;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    if ((*(*(*eUp).Sym).Org).s == (*(*(*eLo).Sym).Org).s &&
            (*(*(*eUp).Sym).Org).t == (*(*(*eLo).Sym).Org).t) as libc::c_int
           as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 19],
                                               &[libc::c_char; 19]>(b"CheckForLeftSplice\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 531i32,
                     b"! VertEq( eUp->Dst, eLo->Dst )\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if (*(*(*eUp).Sym).Org).s < (*(*(*eLo).Sym).Org).s ||
           (*(*(*eUp).Sym).Org).s == (*(*(*eLo).Sym).Org).s &&
               (*(*(*eUp).Sym).Org).t <= (*(*(*eLo).Sym).Org).t {
        if tesedgeSign((*(*eUp).Sym).Org, (*(*eLo).Sym).Org, (*eUp).Org) <
               0i32 as libc::c_float {
            return 0i32
        }
        /* eLo->Dst is above eUp, so splice eLo->Dst into eUp */
        (*regUp).dirty = 1i32;
        (*((*(*(*regUp).nodeUp).next).key as *mut ActiveRegion)).dirty =
            (*regUp).dirty;
        e = tessMeshSplitEdge((*tess).mesh, eUp);
        if e.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
        if tessMeshSplice((*tess).mesh, (*eLo).Sym, e) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        (*(*e).Lface).inside = (*regUp).inside as libc::c_char
    } else {
        if tesedgeSign((*(*eLo).Sym).Org, (*(*eUp).Sym).Org, (*eLo).Org) >
               0i32 as libc::c_float {
            return 0i32
        }
        /* eUp->Dst is below eLo, so splice eUp->Dst into eLo */
        (*regLo).dirty = 1i32;
        (*regUp).dirty = (*regLo).dirty;
        e = tessMeshSplitEdge((*tess).mesh, eLo);
        if e.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
        if tessMeshSplice((*tess).mesh, (*eUp).Lnext, (*eLo).Sym) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        (*(*(*e).Sym).Lface).inside = (*regUp).inside as libc::c_char
    }
    return 1i32;
}
unsafe extern "C" fn CheckForIntersect(mut tess: *mut TESStesselator,
                                       mut regUp: *mut ActiveRegion)
 -> libc::c_int 
 /*
* Check the upper and lower edges of the given region to see if
* they intersect.  If so, create the intersection and add it
* to the data structures.
*
* Returns TRUE if adding the new intersection resulted in a recursive
* call to AddRightEdges(); in this case all "dirty" regions have been
* checked for intersections, and possibly regUp has been deleted.
*/
 {
    let mut regLo: *mut ActiveRegion =
        (*(*(*regUp).nodeUp).prev).key as
            *mut ActiveRegion; /* right endpoints are the same */
    let mut eUp: *mut TESShalfEdge =
        (*regUp).eUp; /* t ranges do not overlap */
    let mut eLo: *mut TESShalfEdge = (*regLo).eUp;
    let mut orgUp: *mut TESSvertex = (*eUp).Org;
    let mut orgLo: *mut TESSvertex = (*eLo).Org;
    let mut dstUp: *mut TESSvertex = (*(*eUp).Sym).Org;
    let mut dstLo: *mut TESSvertex = (*(*eLo).Sym).Org;
    let mut tMinUp: TESSreal = 0.;
    let mut tMaxLo: TESSreal = 0.;
    let mut isect: TESSvertex =
        TESSvertex{next: 0 as *mut TESSvertex,
                   prev: 0 as *mut TESSvertex,
                   anEdge: 0 as *mut TESShalfEdge,
                   coords: [0.; 3],
                   s: 0.,
                   t: 0.,
                   pqHandle: 0,
                   n: 0,
                   idx: 0,};
    let mut orgMin: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    if ((*dstLo).s == (*dstUp).s && (*dstLo).t == (*dstUp).t) as libc::c_int
           as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 578i32,
                     b"! VertEq( dstLo, dstUp )\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !(tesedgeSign(dstUp, (*tess).event, orgUp) <= 0i32 as libc::c_float) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 579i32,
                     b"EdgeSign( dstUp, tess->event, orgUp ) <= 0\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    if !(tesedgeSign(dstLo, (*tess).event, orgLo) >= 0i32 as libc::c_float) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 580i32,
                     b"EdgeSign( dstLo, tess->event, orgLo ) >= 0\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    if !(orgUp != (*tess).event && orgLo != (*tess).event) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 581i32,
                     b"orgUp != tess->event && orgLo != tess->event\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    if !((*regUp).fixUpperEdge == 0 && (*regLo).fixUpperEdge == 0) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 582i32,
                     b"! regUp->fixUpperEdge && ! regLo->fixUpperEdge\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    if orgUp == orgLo { return 0i32 }
    tMinUp = if (*orgUp).t <= (*dstUp).t { (*orgUp).t } else { (*dstUp).t };
    tMaxLo = if (*orgLo).t >= (*dstLo).t { (*orgLo).t } else { (*dstLo).t };
    if tMinUp > tMaxLo { return 0i32 }
    if (*orgUp).s < (*orgLo).s ||
           (*orgUp).s == (*orgLo).s && (*orgUp).t <= (*orgLo).t {
        if tesedgeSign(dstLo, orgUp, orgLo) > 0i32 as libc::c_float {
            return 0i32
        }
    } else if tesedgeSign(dstUp, orgLo, orgUp) < 0i32 as libc::c_float {
        return 0i32
    }
    /* At this point the edges intersect, at least marginally */
    tesedgeIntersect(dstUp, orgUp, dstLo, orgLo, &mut isect);
    /* The following properties are guaranteed: */
    if !((if (*orgUp).t <= (*dstUp).t { (*orgUp).t } else { (*dstUp).t }) <=
             isect.t) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 601i32,
                     b"MIN( orgUp->t, dstUp->t ) <= isect.t\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    if !(isect.t <=
             (if (*orgLo).t >= (*dstLo).t { (*orgLo).t } else { (*dstLo).t }))
           as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 602i32,
                     b"isect.t <= MAX( orgLo->t, dstLo->t )\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    if !((if (*dstLo).s <= (*dstUp).s { (*dstLo).s } else { (*dstUp).s }) <=
             isect.s) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 603i32,
                     b"MIN( dstLo->s, dstUp->s ) <= isect.s\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    if !(isect.s <=
             (if (*orgLo).s >= (*orgUp).s { (*orgLo).s } else { (*orgUp).s }))
           as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"CheckForIntersect\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 604i32,
                     b"isect.s <= MAX( orgLo->s, orgUp->s )\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    if isect.s < (*(*tess).event).s ||
           isect.s == (*(*tess).event).s && isect.t <= (*(*tess).event).t {
        /* The intersection point lies slightly to the left of the sweep line,
		* so move it until it''s slightly to the right of the sweep line.
		* (If we had perfect numerical precision, this would never happen
		* in the first place).  The easiest and safest thing to do is
		* replace the intersection by tess->event.
		*/
        isect.s = (*(*tess).event).s;
        isect.t = (*(*tess).event).t
    }
    /* Similarly, if the computed intersection lies to the right of the
	* rightmost origin (which should rarely happen), it can cause
	* unbelievable inefficiency on sufficiently degenerate inputs.
	* (If you have the test program, try running test54.d with the
	* "X zoom" option turned on).
	*/
    orgMin =
        if (*orgUp).s < (*orgLo).s ||
               (*orgUp).s == (*orgLo).s && (*orgUp).t <= (*orgLo).t {
            orgUp
        } else { orgLo };
    if (*orgMin).s < isect.s ||
           (*orgMin).s == isect.s && (*orgMin).t <= isect.t {
        isect.s = (*orgMin).s;
        isect.t = (*orgMin).t
    }
    if isect.s == (*orgUp).s && isect.t == (*orgUp).t ||
           isect.s == (*orgLo).s && isect.t == (*orgLo).t {
        /* Easy case -- intersection at one of the right endpoints */
        CheckForRightSplice(tess, regUp);
        return 0i32
    }
    if !((*dstUp).s == (*(*tess).event).s && (*dstUp).t == (*(*tess).event).t)
           &&
           tesedgeSign(dstUp, (*tess).event, &mut isect) >=
               0i32 as libc::c_float ||
           !((*dstLo).s == (*(*tess).event).s &&
                 (*dstLo).t == (*(*tess).event).t) &&
               tesedgeSign(dstLo, (*tess).event, &mut isect) <=
                   0i32 as libc::c_float {
        /* Very unusual -- the new upper or lower edge would pass on the
		* wrong side of the sweep event, or through it.  This can happen
		* due to very small numerical errors in the intersection calculation.
		*/
        if dstLo == (*tess).event {
            /* Splice dstLo into eUp, and process the new region(s) */
            if tessMeshSplitEdge((*tess).mesh, (*eUp).Sym).is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            if tessMeshSplice((*tess).mesh, (*eLo).Sym, eUp) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            regUp = TopLeftRegion(tess, regUp);
            if regUp.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
            eUp =
                (*((*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion)).eUp;
            FinishLeftRegions(tess,
                              (*(*(*regUp).nodeUp).prev).key as
                                  *mut ActiveRegion, regLo);
            AddRightEdges(tess, regUp, (*(*eUp).Sym).Lnext, eUp, eUp, 1i32);
            return 1i32
        }
        if dstUp == (*tess).event {
            /* Splice dstUp into eLo, and process the new region(s) */
            if tessMeshSplitEdge((*tess).mesh, (*eLo).Sym).is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            if tessMeshSplice((*tess).mesh, (*eUp).Lnext, (*(*eLo).Sym).Lnext)
                   == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            regLo = regUp;
            regUp = TopRightRegion(regUp);
            e =
                (*(*(*((*(*(*regUp).nodeUp).prev).key as
                           *mut ActiveRegion)).eUp).Sym).Onext;
            (*regLo).eUp = (*(*eLo).Sym).Lnext;
            eLo = FinishLeftRegions(tess, regLo, 0 as *mut ActiveRegion);
            AddRightEdges(tess, regUp, (*eLo).Onext, (*(*eUp).Sym).Onext, e,
                          1i32);
            return 1i32
        }
        /* Special case: called from ConnectRightVertex.  If either
		* edge passes on the wrong side of tess->event, split it
		* (and wait for ConnectRightVertex to splice it appropriately).
		*/
        if tesedgeSign(dstUp, (*tess).event, &mut isect) >=
               0i32 as libc::c_float {
            (*regUp).dirty = 1i32;
            (*((*(*(*regUp).nodeUp).next).key as *mut ActiveRegion)).dirty =
                (*regUp).dirty;
            if tessMeshSplitEdge((*tess).mesh, (*eUp).Sym).is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            (*(*eUp).Org).s = (*(*tess).event).s;
            (*(*eUp).Org).t = (*(*tess).event).t
        }
        if tesedgeSign(dstLo, (*tess).event, &mut isect) <=
               0i32 as libc::c_float {
            (*regLo).dirty = 1i32;
            (*regUp).dirty = (*regLo).dirty;
            if tessMeshSplitEdge((*tess).mesh, (*eLo).Sym).is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            (*(*eLo).Org).s = (*(*tess).event).s;
            (*(*eLo).Org).t = (*(*tess).event).t
        }
        /* leave the rest for ConnectRightVertex */
        return 0i32
    }
    /* General case -- split both edges, splice into new vertex.
	* When we do the splice operation, the order of the arguments is
	* arbitrary as far as correctness goes.  However, when the operation
	* creates a new face, the work done is proportional to the size of
	* the new face.  We expect the faces in the processed part of
	* the mesh (ie. eUp->Lface) to be smaller than the faces in the
	* unprocessed original contours (which will be eLo->Oprev->Lface).
	*/
    if tessMeshSplitEdge((*tess).mesh, (*eUp).Sym).is_null() {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    }
    if tessMeshSplitEdge((*tess).mesh, (*eLo).Sym).is_null() {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    }
    if tessMeshSplice((*tess).mesh, (*(*eLo).Sym).Lnext, eUp) == 0 {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    }
    (*(*eUp).Org).s = isect.s;
    (*(*eUp).Org).t = isect.t;
    (*(*eUp).Org).pqHandle =
        pqInsert(&mut (*tess).alloc, (*tess).pq, (*eUp).Org as PQkey);
    if (*(*eUp).Org).pqHandle == 0xfffffffi32 {
        pqDeletePriorityQ(&mut (*tess).alloc, (*tess).pq);
        (*tess).pq = 0 as *mut PriorityQ;
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    }
    GetIntersectData(tess, (*eUp).Org, orgUp, dstUp, orgLo, dstLo);
    (*regLo).dirty = 1i32;
    (*regUp).dirty = (*regLo).dirty;
    (*((*(*(*regUp).nodeUp).next).key as *mut ActiveRegion)).dirty =
        (*regUp).dirty;
    return 0i32;
}
unsafe extern "C" fn WalkDirtyRegions(mut tess: *mut TESStesselator,
                                      mut regUp: *mut ActiveRegion) 
 /*
* When the upper or lower edge of any region changes, the region is
* marked "dirty".  This routine walks through all the dirty regions
* and makes sure that the dictionary invariants are satisfied
* (see the comments at the beginning of this file).  Of course
* new dirty regions can be created as we make changes to restore
* the invariants.
*/
 {
    let mut regLo: *mut ActiveRegion =
        (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    let mut eUp: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eLo: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    loop  {
        /* Find the lowest dirty region (we walk from the bottom up). */
        while (*regLo).dirty != 0 {
            regUp = regLo;
            regLo = (*(*(*regLo).nodeUp).prev).key as *mut ActiveRegion
        }
        if (*regUp).dirty == 0 {
            regLo = regUp;
            regUp = (*(*(*regUp).nodeUp).next).key as *mut ActiveRegion;
            if regUp.is_null() || (*regUp).dirty == 0 {
                /* We've walked all the dirty regions */
                return
            }
        }
        (*regUp).dirty = 0i32;
        eUp = (*regUp).eUp;
        eLo = (*regLo).eUp;
        if (*(*eUp).Sym).Org != (*(*eLo).Sym).Org {
            /* Check that the edge ordering is obeyed at the Dst vertices. */
            if CheckForLeftSplice(tess, regUp) != 0 {
                /* If the upper or lower edge was marked fixUpperEdge, then
				* we no longer need it (since these edges are needed only for
				* vertices which otherwise have no right-going edges).
				*/
                if (*regLo).fixUpperEdge != 0 {
                    DeleteRegion(tess, regLo);
                    if tessMeshDelete((*tess).mesh, eLo) == 0 {
                        longjmp((*tess).env.as_mut_ptr(), 1i32);
                    }
                    regLo =
                        (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
                    eLo = (*regLo).eUp
                } else if (*regUp).fixUpperEdge != 0 {
                    DeleteRegion(tess, regUp);
                    if tessMeshDelete((*tess).mesh, eUp) == 0 {
                        longjmp((*tess).env.as_mut_ptr(), 1i32);
                    }
                    regUp =
                        (*(*(*regLo).nodeUp).next).key as *mut ActiveRegion;
                    eUp = (*regUp).eUp
                }
            }
        }
        if (*eUp).Org != (*eLo).Org {
            if (*(*eUp).Sym).Org != (*(*eLo).Sym).Org &&
                   (*regUp).fixUpperEdge == 0 && (*regLo).fixUpperEdge == 0 &&
                   ((*(*eUp).Sym).Org == (*tess).event ||
                        (*(*eLo).Sym).Org == (*tess).event) {
                /* When all else fails in CheckForIntersect(), it uses tess->event
				* as the intersection location.  To make this possible, it requires
				* that tess->event lie between the upper and lower edges, and also
				* that neither of these is marked fixUpperEdge (since in the worst
				* case it might splice one of these edges into tess->event, and
				* violate the invariant that fixable edges are the only right-going
				* edge from their associated vertex).
				*/
                if CheckForIntersect(tess, regUp) != 0 {
                    /* WalkDirtyRegions() was called recursively; we're done */
                    return
                }
            } else {
                /* Even though we can't use CheckForIntersect(), the Org vertices
				* may violate the dictionary edge ordering.  Check and correct this.
				*/
                CheckForRightSplice(tess, regUp);
            }
        }
        if (*eUp).Org == (*eLo).Org && (*(*eUp).Sym).Org == (*(*eLo).Sym).Org
           {
            /* A degenerate loop consisting of only two edges -- delete it. */
            (*eLo).winding += (*eUp).winding;
            (*(*eLo).Sym).winding += (*(*eUp).Sym).winding;
            DeleteRegion(tess, regUp);
            if tessMeshDelete((*tess).mesh, eUp) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            regUp = (*(*(*regLo).nodeUp).next).key as *mut ActiveRegion
        }
    };
}
unsafe extern "C" fn ConnectRightVertex(mut tess: *mut TESStesselator,
                                        mut regUp: *mut ActiveRegion,
                                        mut eBottomLeft: *mut TESShalfEdge) 
 /*
* Purpose: connect a "right" vertex vEvent (one where all edges go left)
* to the unprocessed portion of the mesh.  Since there are no right-going
* edges, two regions (one above vEvent and one below) are being merged
* into one.  "regUp" is the upper of these two regions.
*
* There are two reasons for doing this (adding a right-going edge):
*  - if the two regions being merged are "inside", we must add an edge
*    to keep them separated (the combined region would not be monotone).
*  - in any case, we must leave some record of vEvent in the dictionary,
*    so that we can merge vEvent with features that we have not seen yet.
*    For example, maybe there is a vertical edge which passes just to
*    the right of vEvent; we would like to splice vEvent into this edge.
*
* However, we don't want to connect vEvent to just any vertex.  We don''t
* want the new edge to cross any other edges; otherwise we will create
* intersection vertices even when the input data had no self-intersections.
* (This is a bad thing; if the user's input data has no intersections,
* we don't want to generate any false intersections ourselves.)
*
* Our eventual goal is to connect vEvent to the leftmost unprocessed
* vertex of the combined region (the union of regUp and regLo).
* But because of unseen vertices with all right-going edges, and also
* new vertices which may be created by edge intersections, we don''t
* know where that leftmost unprocessed vertex is.  In the meantime, we
* connect vEvent to the closest vertex of either chain, and mark the region
* as "fixUpperEdge".  This flag says to delete and reconnect this edge
* to the next processed vertex on the boundary of the combined region.
* Quite possibly the vertex we connected to will turn out to be the
* closest one, in which case we won''t need to make any changes.
*/
 {
    let mut eNew: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eTopLeft: *mut TESShalfEdge = (*eBottomLeft).Onext;
    let mut regLo: *mut ActiveRegion =
        (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    let mut eUp: *mut TESShalfEdge = (*regUp).eUp;
    let mut eLo: *mut TESShalfEdge = (*regLo).eUp;
    let mut degenerate: libc::c_int = 0i32;
    if (*(*eUp).Sym).Org != (*(*eLo).Sym).Org {
        CheckForIntersect(tess, regUp);
    }
    /* Possible new degeneracies: upper or lower edge of regUp may pass
	* through vEvent, or may coincide with new intersection vertex
	*/
    if (*(*eUp).Org).s == (*(*tess).event).s &&
           (*(*eUp).Org).t == (*(*tess).event).t {
        if tessMeshSplice((*tess).mesh, (*(*eTopLeft).Sym).Lnext, eUp) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        regUp = TopLeftRegion(tess, regUp);
        if regUp.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
        eTopLeft =
            (*((*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion)).eUp;
        FinishLeftRegions(tess,
                          (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion,
                          regLo);
        degenerate = 1i32
    }
    if (*(*eLo).Org).s == (*(*tess).event).s &&
           (*(*eLo).Org).t == (*(*tess).event).t {
        if tessMeshSplice((*tess).mesh, eBottomLeft, (*(*eLo).Sym).Lnext) == 0
           {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        eBottomLeft = FinishLeftRegions(tess, regLo, 0 as *mut ActiveRegion);
        degenerate = 1i32
    }
    if degenerate != 0 {
        AddRightEdges(tess, regUp, (*eBottomLeft).Onext, eTopLeft, eTopLeft,
                      1i32);
        return
    }
    /* Non-degenerate situation -- need to add a temporary, fixable edge.
	* Connect to the closer of eLo->Org, eUp->Org.
	*/
    if (*(*eLo).Org).s < (*(*eUp).Org).s ||
           (*(*eLo).Org).s == (*(*eUp).Org).s &&
               (*(*eLo).Org).t <= (*(*eUp).Org).t {
        eNew = (*(*eLo).Sym).Lnext
    } else { eNew = eUp }
    eNew = tessMeshConnect((*tess).mesh, (*(*eBottomLeft).Onext).Sym, eNew);
    if eNew.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    /* Prevent cleanup, otherwise eNew might disappear before we've even
	* had a chance to mark it as a temporary edge.
	*/
    AddRightEdges(tess, regUp, eNew, (*eNew).Onext, (*eNew).Onext, 0i32);
    (*(*(*eNew).Sym).activeRegion).fixUpperEdge = 1i32;
    WalkDirtyRegions(tess, regUp);
}
/* Because vertices at exactly the same location are merged together
* before we process the sweep event, some degenerate cases can't occur.
* However if someone eventually makes the modifications required to
* merge features which are close together, the cases below marked
* TOLERANCE_NONZERO will be useful.  They were debugged before the
* code to merge identical vertices in the main loop was added.
*/
unsafe extern "C" fn ConnectLeftDegenerate(mut tess: *mut TESStesselator,
                                           mut regUp: *mut ActiveRegion,
                                           mut vEvent: *mut TESSvertex) 
 /*
* The event vertex lies exacty on an already-processed edge or vertex.
* Adding the new vertex involves splicing it into the already-processed
* part of the mesh.
*/
 {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eTopLeft: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eTopRight: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eLast: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut reg: *mut ActiveRegion = 0 as *mut ActiveRegion;
    e = (*regUp).eUp;
    if (*(*e).Org).s == (*vEvent).s && (*(*e).Org).t == (*vEvent).t {
        /* e->Org is an unprocessed vertex - just combine them, and wait
		* for e->Org to be pulled from the queue
		*/
        if (0i32 == 0) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"ConnectLeftDegenerate\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 907i32,
                         b"TOLERANCE_NONZERO\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        SpliceMergeVertices(tess, e, (*vEvent).anEdge);
        return
    }
    if !((*(*(*e).Sym).Org).s == (*vEvent).s &&
             (*(*(*e).Sym).Org).t == (*vEvent).t) {
        /* General case -- splice vEvent into edge e which passes through it */
        if tessMeshSplitEdge((*tess).mesh, (*e).Sym).is_null() {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        if (*regUp).fixUpperEdge != 0 {
            /* This edge was fixable -- delete unused portion of original edge */
            if tessMeshDelete((*tess).mesh, (*e).Onext) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32); /* recurse */
            }
            (*regUp).fixUpperEdge = 0i32
        }
        if tessMeshSplice((*tess).mesh, (*vEvent).anEdge, e) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        SweepEvent(tess, vEvent);
        return
    }
    /* vEvent coincides with e->Dst, which has already been processed.
	* Splice in the additional right-going edges.
	*/
    if (0i32 == 0) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                               &[libc::c_char; 22]>(b"ConnectLeftDegenerate\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                         as *const u8 as *const libc::c_char, 928i32,
                     b"TOLERANCE_NONZERO\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    regUp = TopRightRegion(regUp);
    reg = (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    eTopRight = (*(*reg).eUp).Sym;
    eLast = (*eTopRight).Onext;
    eTopLeft = eLast;
    if (*reg).fixUpperEdge != 0 {
        /* Here e->Dst has only a single fixable edge going right.
		* We can delete it since now we have some real right-going edges.
		*/
        if !(eTopLeft != eTopRight) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"ConnectLeftDegenerate\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 937i32,
                         b"eTopLeft != eTopRight\x00" as *const u8 as
                             *const libc::c_char); /* there are some left edges too */
        } else { };
        DeleteRegion(tess, reg);
        if tessMeshDelete((*tess).mesh, eTopRight) == 0 {
            longjmp((*tess).env.as_mut_ptr(), 1i32);
        }
        eTopRight = (*(*eTopLeft).Sym).Lnext
    }
    if tessMeshSplice((*tess).mesh, (*vEvent).anEdge, eTopRight) == 0 {
        longjmp((*tess).env.as_mut_ptr(), 1i32);
    }
    if !((*(*(*eTopLeft).Sym).Org).s < (*(*eTopLeft).Org).s ||
             (*(*(*eTopLeft).Sym).Org).s == (*(*eTopLeft).Org).s &&
                 (*(*(*eTopLeft).Sym).Org).t <= (*(*eTopLeft).Org).t) {
        /* e->Dst had no left-going edges -- indicate this to AddRightEdges() */
        eTopLeft = 0 as *mut TESShalfEdge
    }
    AddRightEdges(tess, regUp, (*eTopRight).Onext, eLast, eTopLeft, 1i32);
}
unsafe extern "C" fn ConnectLeftVertex(mut tess: *mut TESStesselator,
                                       mut vEvent: *mut TESSvertex) 
 /*
* Purpose: connect a "left" vertex (one where both edges go right)
* to the processed portion of the mesh.  Let R be the active region
* containing vEvent, and let U and L be the upper and lower edge
* chains of R.  There are two possibilities:
*
* - the normal case: split R into two regions, by connecting vEvent to
*   the rightmost vertex of U or L lying to the left of the sweep line
*
* - the degenerate case: if vEvent is close enough to U or L, we
*   merge vEvent into that edge chain.  The subcases are:
*	- merging with the rightmost vertex of U or L
*	- merging with the active edge of U or L
*	- merging with an already-processed portion of U or L
*/
 {
    let mut regUp: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut regLo: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut reg: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut eUp: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eLo: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNew: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut tmp: ActiveRegion =
        ActiveRegion{eUp: 0 as *mut TESShalfEdge,
                     nodeUp: 0 as *mut DictNode,
                     windingNumber: 0,
                     inside: 0,
                     sentinel: 0,
                     dirty: 0,
                     fixUpperEdge: 0,};
    /* assert( vEvent->anEdge->Onext->Onext == vEvent->anEdge ); */
    /* Get a pointer to the active region containing vEvent */
    tmp.eUp = (*(*vEvent).anEdge).Sym;
    /* __GL_DICTLISTKEY */
    /* tessDictListSearch */
    regUp =
        (*dictSearch((*tess).dict,
                     &mut tmp as *mut ActiveRegion as DictKey)).key as
            *mut ActiveRegion;
    regLo = (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    if regLo.is_null() {
        // This may happen if the input polygon is coplanar.
        return
    }
    eUp = (*regUp).eUp;
    eLo = (*regLo).eUp;
    /* Try merging with U or L first */
    if tesedgeSign((*(*eUp).Sym).Org, vEvent, (*eUp).Org) ==
           0i32 as libc::c_float {
        ConnectLeftDegenerate(tess, regUp, vEvent);
        return
    }
    /* Connect vEvent to rightmost processed vertex of either chain.
	* e->Dst is the vertex that we will connect to vEvent.
	*/
    reg =
        if (*(*(*eLo).Sym).Org).s < (*(*(*eUp).Sym).Org).s ||
               (*(*(*eLo).Sym).Org).s == (*(*(*eUp).Sym).Org).s &&
                   (*(*(*eLo).Sym).Org).t <= (*(*(*eUp).Sym).Org).t {
            regUp
        } else { regLo };
    if (*regUp).inside != 0 || (*reg).fixUpperEdge != 0 {
        if reg == regUp {
            eNew =
                tessMeshConnect((*tess).mesh, (*(*vEvent).anEdge).Sym,
                                (*eUp).Lnext);
            if eNew.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
        } else {
            let mut tempHalfEdge: *mut TESShalfEdge =
                tessMeshConnect((*tess).mesh, (*(*(*eLo).Sym).Onext).Sym,
                                (*vEvent).anEdge);
            if tempHalfEdge.is_null() {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
            eNew = (*tempHalfEdge).Sym
        }
        if (*reg).fixUpperEdge != 0 {
            if FixUpperEdge(tess, reg, eNew) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
        } else { ComputeWinding(tess, AddRegionBelow(tess, regUp, eNew)); }
        SweepEvent(tess, vEvent);
    } else {
        /* The new vertex is in a region which does not belong to the polygon.
		* We don''t need to connect this vertex to the rest of the mesh.
		*/
        AddRightEdges(tess, regUp, (*vEvent).anEdge, (*vEvent).anEdge,
                      0 as *mut TESShalfEdge, 1i32);
    };
}
/*
* Invariants for the Edge Dictionary.
* - each pair of adjacent edges e2=Succ(e1) satisfies EdgeLeq(e1,e2)
*   at any valid location of the sweep event
* - if EdgeLeq(e2,e1) as well (at any valid sweep event), then e1 and e2
*   share a common endpoint
* - for each e, e->Dst has been processed, but not e->Org
* - each edge e satisfies VertLeq(e->Dst,event) && VertLeq(event,e->Org)
*   where "event" is the current sweep line event.
* - no edge e has zero length
*
* Invariants for the Mesh (the processed portion).
* - the portion of the mesh left of the sweep line is a planar graph,
*   ie. there is *some* way to embed it in the plane
* - no processed edge has zero length
* - no two processed vertices have identical coordinates
* - each "inside" region is monotone, ie. can be broken into two chains
*   of monotonically increasing vertices according to VertLeq(v1,v2)
*   - a non-invariant: these chains may intersect (very slightly)
*
* Invariants for the Sweep.
* - if none of the edges incident to the event vertex have an activeRegion
*   (ie. none of these edges are in the edge dictionary), then the vertex
*   has only right-going edges.
* - if an edge is marked "fixUpperEdge" (it is a temporary edge introduced
*   by ConnectRightVertex), then it is the only right-going edge from
*   its associated vertex.  (This says that these edges exist only
*   when it is necessary.)
*/
/* When we merge two edges into one, we need to compute the combined
* winding of the new edge.
*/
unsafe extern "C" fn SweepEvent(mut tess: *mut TESStesselator,
                                mut vEvent: *mut TESSvertex) 
 /*
* Does everything necessary when the sweep line crosses a vertex.
* Updates the mesh and the edge dictionary.
*/
 {
    let mut regUp: *mut ActiveRegion =
        0 as *mut ActiveRegion; /* for access in EdgeLeq() */
    let mut reg: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eTopLeft: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eBottomLeft: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    (*tess).event = vEvent;
    /* Check if this vertex is the right endpoint of an edge that is
	* already in the dictionary.  In this case we don't need to waste
	* time searching for the location to insert new edges.
	*/
    e = (*vEvent).anEdge;
    while (*e).activeRegion.is_null() {
        e = (*e).Onext;
        if e == (*vEvent).anEdge {
            /* All edges go right -- not incident to any processed edges */
            ConnectLeftVertex(tess, vEvent);
            return
        }
    }
    /* Processing consists of two phases: first we "finish" all the
	* active regions where both the upper and lower edges terminate
	* at vEvent (ie. vEvent is closing off these regions).
	* We mark these faces "inside" or "outside" the polygon according
	* to their winding number, and delete the edges from the dictionary.
	* This takes care of all the left-going edges from vEvent.
	*/
    regUp = TopLeftRegion(tess, (*e).activeRegion);
    if regUp.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    reg = (*(*(*regUp).nodeUp).prev).key as *mut ActiveRegion;
    eTopLeft = (*reg).eUp;
    eBottomLeft = FinishLeftRegions(tess, reg, 0 as *mut ActiveRegion);
    /* Next we process all the right-going edges from vEvent.  This
	* involves adding the edges to the dictionary, and creating the
	* associated "active regions" which record information about the
	* regions between adjacent dictionary edges.
	*/
    if (*eBottomLeft).Onext == eTopLeft {
        /* No right-going edges -- add a temporary "fixable" edge */
        ConnectRightVertex(tess, regUp, eBottomLeft);
    } else {
        AddRightEdges(tess, regUp, (*eBottomLeft).Onext, eTopLeft, eTopLeft,
                      1i32);
    };
}
/* Make the sentinel coordinates big enough that they will never be
* merged with real input features.
*/
unsafe extern "C" fn AddSentinel(mut tess: *mut TESStesselator,
                                 mut smin: TESSreal, mut smax: TESSreal,
                                 mut t: TESSreal) 
 /*
* We add two sentinel edges above and below all other edges,
* to avoid special cases at the top and bottom.
*/
 {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge; /* initialize it */
    let mut reg: *mut ActiveRegion =
        bucketAlloc((*tess).regionPool) as *mut ActiveRegion;
    if reg.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    e = tessMeshMakeEdge((*tess).mesh);
    if e.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    (*(*e).Org).s = smax;
    (*(*e).Org).t = t;
    (*(*(*e).Sym).Org).s = smin;
    (*(*(*e).Sym).Org).t = t;
    (*tess).event = (*(*e).Sym).Org;
    (*reg).eUp = e;
    (*reg).windingNumber = 0i32;
    (*reg).inside = 0i32;
    (*reg).fixUpperEdge = 0i32;
    (*reg).sentinel = 1i32;
    (*reg).dirty = 0i32;
    (*reg).nodeUp =
        dictInsertBefore((*tess).dict, &mut (*(*tess).dict).head,
                         reg as DictKey);
    if (*reg).nodeUp.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); };
}
unsafe extern "C" fn InitEdgeDict(mut tess: *mut TESStesselator) 
 /*
* We maintain an ordering of edge intersections with the sweep line.
* This order is maintained in a dynamic dictionary.
*/
 {
    let mut w: TESSreal = 0.;
    let mut h: TESSreal = 0.;
    let mut smin: TESSreal = 0.;
    let mut smax: TESSreal = 0.;
    let mut tmin: TESSreal = 0.;
    let mut tmax: TESSreal = 0.;
    (*tess).dict =
        dictNewDict(&mut (*tess).alloc, tess as *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut TESStesselator,
                                                                        _:
                                                                            *mut ActiveRegion,
                                                                        _:
                                                                            *mut ActiveRegion)
                                                       -> libc::c_int>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            DictKey,
                                                                        _:
                                                                            DictKey)
                                                       ->
                                                           libc::c_int>>(Some(EdgeLeq
                                                                                  as
                                                                                  unsafe extern "C" fn(_:
                                                                                                           *mut TESStesselator,
                                                                                                       _:
                                                                                                           *mut ActiveRegion,
                                                                                                       _:
                                                                                                           *mut ActiveRegion)
                                                                                      ->
                                                                                          libc::c_int)));
    if (*tess).dict.is_null() { longjmp((*tess).env.as_mut_ptr(), 1i32); }
    /* If the bbox is empty, ensure that sentinels are not coincident by slightly enlarging it. */
    w = (*tess).bmax[0] - (*tess).bmin[0] + 0.01f64 as TESSreal;
    h = (*tess).bmax[1] - (*tess).bmin[1] + 0.01f64 as TESSreal;
    smin = (*tess).bmin[0] - w;
    smax = (*tess).bmax[0] + w;
    tmin = (*tess).bmin[1] - h;
    tmax = (*tess).bmax[1] + h;
    AddSentinel(tess, smin, smax, tmin);
    AddSentinel(tess, smin, smax, tmax);
}
unsafe extern "C" fn DoneEdgeDict(mut tess: *mut TESStesselator) {
    let mut reg: *mut ActiveRegion = 0 as *mut ActiveRegion;
    let mut fixedEdges: libc::c_int = 0i32;
    loop  {
        reg = (*(*(*tess).dict).head.next).key as *mut ActiveRegion;
        if reg.is_null() { break ; }
        /*
		* At the end of all processing, the dictionary should contain
		* only the two sentinel edges, plus at most one "fixable" edge
		* created by ConnectRightVertex().
		*/
        if (*reg).sentinel == 0 {
            if ((*reg).fixUpperEdge == 0) as libc::c_int as libc::c_long != 0
               {
                __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                       &[libc::c_char; 13]>(b"DoneEdgeDict\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                                 as *const u8 as *const libc::c_char, 1147i32,
                             b"reg->fixUpperEdge\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            fixedEdges += 1;
            if !(fixedEdges == 1i32) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                       &[libc::c_char; 13]>(b"DoneEdgeDict\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                                 as *const u8 as *const libc::c_char, 1148i32,
                             b"++fixedEdges == 1\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
        }
        if !((*reg).windingNumber == 0i32) as libc::c_int as libc::c_long != 0
           {
            __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                   &[libc::c_char; 13]>(b"DoneEdgeDict\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 1150i32,
                         b"reg->windingNumber == 0\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        DeleteRegion(tess, reg);
        /*    tessMeshDelete( reg->eUp );*/
    }
    dictDeleteDict(&mut (*tess).alloc, (*tess).dict);
}
unsafe extern "C" fn RemoveDegenerateEdges(mut tess: *mut TESStesselator) 
 /*
* Remove zero-length edges, and contours with fewer than 3 vertices.
*/
 {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eLnext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eHead: *mut TESShalfEdge = &mut (*(*tess).mesh).eHead;
    /*LINTED*/
    e = (*eHead).next;
    while e != eHead {
        eNext = (*e).next;
        eLnext = (*e).Lnext;
        if (*(*e).Org).s == (*(*(*e).Sym).Org).s &&
               (*(*e).Org).t == (*(*(*e).Sym).Org).t &&
               (*(*e).Lnext).Lnext != e {
            /* Zero-length edge, contour has at least 3 edges */
            SpliceMergeVertices(tess, eLnext, e); /* deletes e->Org */
            if tessMeshDelete((*tess).mesh, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(),
                        1i32); /* e is a self-loop */
            }
            e = eLnext;
            eLnext = (*e).Lnext
        }
        if (*eLnext).Lnext == e {
            /* Degenerate contour (one or two edges) */
            if eLnext != e {
                if eLnext == eNext || eLnext == (*eNext).Sym {
                    eNext = (*eNext).next
                }
                if tessMeshDelete((*tess).mesh, eLnext) == 0 {
                    longjmp((*tess).env.as_mut_ptr(), 1i32);
                }
            }
            if e == eNext || e == (*eNext).Sym { eNext = (*eNext).next }
            if tessMeshDelete((*tess).mesh, e) == 0 {
                longjmp((*tess).env.as_mut_ptr(), 1i32);
            }
        }
        e = eNext
    };
}
unsafe extern "C" fn InitPriorityQ(mut tess: *mut TESStesselator)
 -> libc::c_int 
 /*
* Insert all vertices into the priority queue which determines the
* order in which vertices cross the sweep line.
*/
 {
    let mut pq: *mut PriorityQ = 0 as *mut PriorityQ;
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vHead: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vertexCount: libc::c_int = 0i32;
    vHead = &mut (*(*tess).mesh).vHead;
    v = (*vHead).next;
    while v != vHead { vertexCount += 1; v = (*v).next }
    /* Make sure there is enough space for sentinels. */
    vertexCount +=
        if 8i32 >= (*tess).alloc.extraVertices {
            8i32
        } else { (*tess).alloc.extraVertices };
    (*tess).pq =
        pqNewPriorityQ(&mut (*tess).alloc, vertexCount,
                       ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                               *mut TESSvertex,
                                                                           _:
                                                                               *mut TESSvertex)
                                                          -> libc::c_int>,
                                               Option<unsafe extern "C" fn(_:
                                                                               PQkey,
                                                                           _:
                                                                               PQkey)
                                                          ->
                                                              libc::c_int>>(Some(tesvertLeq
                                                                                     as
                                                                                     unsafe extern "C" fn(_:
                                                                                                              *mut TESSvertex,
                                                                                                          _:
                                                                                                              *mut TESSvertex)
                                                                                         ->
                                                                                             libc::c_int)));
    pq = (*tess).pq;
    if pq.is_null() { return 0i32 }
    vHead = &mut (*(*tess).mesh).vHead;
    v = (*vHead).next;
    while v != vHead {
        (*v).pqHandle = pqInsert(&mut (*tess).alloc, pq, v as PQkey);
        if (*v).pqHandle == 0xfffffffi32 { break ; }
        v = (*v).next
    }
    if v != vHead || pqInit(&mut (*tess).alloc, pq) == 0 {
        pqDeletePriorityQ(&mut (*tess).alloc, (*tess).pq);
        (*tess).pq = 0 as *mut PriorityQ;
        return 0i32
    }
    return 1i32;
}
unsafe extern "C" fn DonePriorityQ(mut tess: *mut TESStesselator) {
    pqDeletePriorityQ(&mut (*tess).alloc, (*tess).pq);
}
unsafe extern "C" fn RemoveDegenerateFaces(mut tess: *mut TESStesselator,
                                           mut mesh: *mut TESSmesh)
 -> libc::c_int 
 /*
* Delete any degenerate faces with only two edges.  WalkDirtyRegions()
* will catch almost all of these, but it won't catch degenerate faces
* produced by splice operations on already-processed edges.
* The two places this can happen are in FinishLeftRegions(), when
* we splice in a "temporary" edge produced by ConnectRightVertex(),
* and in CheckForLeftSplice(), where we splice already-processed
* edges to ensure that our dictionary invariants are not violated
* by numerical errors.
*
* In both these cases it is *very* dangerous to delete the offending
* edge at the time, since one of the routines further up the stack
* will sometimes be keeping a pointer to that edge.
*/
 {
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut fNext: *mut TESSface = 0 as *mut TESSface;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    /*LINTED*/
    f = (*mesh).fHead.next;
    while f != &mut (*mesh).fHead as *mut TESSface {
        fNext = (*f).next;
        e = (*f).anEdge;
        if !((*e).Lnext != e) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                                   &[libc::c_char; 22]>(b"RemoveDegenerateFaces\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/sweep.c\x00"
                             as *const u8 as *const libc::c_char, 1257i32,
                         b"e->Lnext != e\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if (*(*e).Lnext).Lnext == e {
            /* A face with only two edges */
            (*(*e).Onext).winding += (*e).winding;
            (*(*(*e).Onext).Sym).winding += (*(*e).Sym).winding;
            if tessMeshDelete((*tess).mesh, e) == 0 { return 0i32 }
        }
        f = fNext
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tessComputeInterior(mut tess: *mut TESStesselator)
 -> libc::c_int 
 /*
* tessComputeInterior( tess ) computes the planar arrangement specified
* by the given contours, and further subdivides this arrangement
* into regions.  Each region is marked "inside" if it belongs
* to the polygon, according to the rule given by tess->windingRule.
* Each interior region is guaranteed be monotone.
*/
 {
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vNext: *mut TESSvertex = 0 as *mut TESSvertex;
    /* Each vertex defines an event for our sweep line.  Start by inserting
	* all the vertices in a priority queue.  Events are processed in
	* lexicographic order, ie.
	*
	*	e1 < e2  iff  e1.x < e2.x || (e1.x == e2.x && e1.y < e2.y)
	*/
    RemoveDegenerateEdges(tess); /* if error */
    if InitPriorityQ(tess) == 0 { return 0i32 }
    InitEdgeDict(tess);
    loop  {
        v = pqExtractMin((*tess).pq) as *mut TESSvertex;
        if v.is_null() { break ; }
        loop  {
            vNext = pqMinimum((*tess).pq) as *mut TESSvertex;
            if vNext.is_null() ||
                   !((*vNext).s == (*v).s && (*vNext).t == (*v).t) {
                break ;
            }
            /* Merge together all vertices at exactly the same location.
			* This is more efficient than processing them one at a time,
			* simplifies the code (see ConnectLeftDegenerate), and is also
			* important for correct handling of certain degenerate cases.
			* For example, suppose there are two identical edges A and B
			* that belong to different contours (so without this code they would
			* be processed by separate sweep events).  Suppose another edge C
			* crosses A and B from above.  When A is processed, we split it
			* at its intersection point with C.  However this also splits C,
			* so when we insert B we may compute a slightly different
			* intersection point.  This might leave two edges with a small
			* gap between them.  This kind of error is especially obvious
			* when using boundary extraction (TESS_BOUNDARY_ONLY).
			*/
            vNext = pqExtractMin((*tess).pq) as *mut TESSvertex;
            SpliceMergeVertices(tess, (*v).anEdge, (*vNext).anEdge);
        }
        SweepEvent(tess, v);
    }
    /* Set tess->event for debugging purposes */
    (*tess).event =
        (*(*((*(*(*tess).dict).head.next).key as *mut ActiveRegion)).eUp).Org;
    DoneEdgeDict(tess);
    DonePriorityQ(tess);
    if RemoveDegenerateFaces(tess, (*tess).mesh) == 0 { return 0i32 }
    tessMeshCheckMesh((*tess).mesh);
    return 1i32;
}
