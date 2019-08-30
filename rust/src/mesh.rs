use libc;
extern "C" {
    pub type BucketAlloc;
    pub type ActiveRegion;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn tesvertCCW(u: *mut TESSvertex, v: *mut TESSvertex, w: *mut TESSvertex)
     -> libc::c_int;
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
    fn createBucketAlloc(alloc: *mut TESSalloc, name: *const libc::c_char,
                         itemSize: libc::c_uint, bucketSize: libc::c_uint)
     -> *mut BucketAlloc;
    #[no_mangle]
    fn bucketAlloc(ba: *mut BucketAlloc) -> *mut libc::c_void;
    #[no_mangle]
    fn bucketFree(ba: *mut BucketAlloc, ptr: *mut libc::c_void);
    #[no_mangle]
    fn deleteBucketAlloc(ba: *mut BucketAlloc);
}
pub type TESSreal = libc::c_float;
pub type TESSindex = libc::c_int;
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
pub struct EdgePair {
    pub e: TESShalfEdge,
    pub eSym: TESShalfEdge,
}
/* MakeEdge creates a new pair of half-edges which form their own loop.
* No vertex or face structures are allocated, but these must be assigned
* before the current edge operation is completed.
*/
unsafe extern "C" fn MakeEdge(mut mesh: *mut TESSmesh,
                              mut eNext: *mut TESShalfEdge)
 -> *mut TESShalfEdge {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut ePrev: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut pair: *mut EdgePair =
        bucketAlloc((*mesh).edgeBucket) as *mut EdgePair;
    if pair.is_null() { return 0 as *mut TESShalfEdge }
    e = &mut (*pair).e;
    eSym = &mut (*pair).eSym;
    /* Make sure eNext points to the first edge of the edge pair */
    if (*eNext).Sym < eNext { eNext = (*eNext).Sym }
    /* Insert in circular doubly-linked list before eNext.
	* Note that the prev pointer is stored in Sym->next.
	*/
    ePrev = (*(*eNext).Sym).next;
    (*eSym).next = ePrev;
    (*(*ePrev).Sym).next = e;
    (*e).next = eNext;
    (*(*eNext).Sym).next = eSym;
    (*e).Sym = eSym;
    (*e).Onext = e;
    (*e).Lnext = eSym;
    (*e).Org = 0 as *mut TESSvertex;
    (*e).Lface = 0 as *mut TESSface;
    (*e).winding = 0i32;
    (*e).activeRegion = 0 as *mut ActiveRegion;
    (*e).mark = 0i32;
    (*eSym).Sym = e;
    (*eSym).Onext = eSym;
    (*eSym).Lnext = e;
    (*eSym).Org = 0 as *mut TESSvertex;
    (*eSym).Lface = 0 as *mut TESSface;
    (*eSym).winding = 0i32;
    (*eSym).activeRegion = 0 as *mut ActiveRegion;
    (*eSym).mark = 0i32;
    return e;
}
/* Splice( a, b ) is best described by the Guibas/Stolfi paper or the
* CS348a notes (see mesh.h).  Basically it modifies the mesh so that
* a->Onext and b->Onext are exchanged.  This can have various effects
* depending on whether a and b belong to different face or vertex rings.
* For more explanation see tessMeshSplice() below.
*/
unsafe extern "C" fn Splice(mut a: *mut TESShalfEdge,
                            mut b: *mut TESShalfEdge) {
    let mut aOnext: *mut TESShalfEdge = (*a).Onext;
    let mut bOnext: *mut TESShalfEdge = (*b).Onext;
    (*(*aOnext).Sym).Lnext = b;
    (*(*bOnext).Sym).Lnext = a;
    (*a).Onext = bOnext;
    (*b).Onext = aOnext;
}
/* MakeVertex( newVertex, eOrig, vNext ) attaches a new vertex and makes it the
* origin of all edges in the vertex loop to which eOrig belongs. "vNext" gives
* a place to insert the new vertex in the global vertex list.  We insert
* the new vertex *before* vNext so that algorithms which walk the vertex
* list will not see the newly created vertices.
*/
unsafe extern "C" fn MakeVertex(mut newVertex: *mut TESSvertex,
                                mut eOrig: *mut TESShalfEdge,
                                mut vNext: *mut TESSvertex) {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut vPrev: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vNew: *mut TESSvertex = newVertex;
    if vNew.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 11],
                                               &[libc::c_char; 11]>(b"MakeVertex\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 127i32,
                     b"vNew != NULL\x00" as *const u8 as *const libc::c_char);
    } else { };
    /* insert in circular doubly-linked list before vNext */
    vPrev = (*vNext).prev;
    (*vNew).prev = vPrev;
    (*vPrev).next = vNew;
    (*vNew).next = vNext;
    (*vNext).prev = vNew;
    (*vNew).anEdge = eOrig;
    /* leave coords, s, t undefined */
    /* fix other edges on this vertex loop */
    e = eOrig;
    loop  { (*e).Org = vNew; e = (*e).Onext; if !(e != eOrig) { break ; } };
}
/* MakeFace( newFace, eOrig, fNext ) attaches a new face and makes it the left
* face of all edges in the face loop to which eOrig belongs.  "fNext" gives
* a place to insert the new face in the global face list.  We insert
* the new face *before* fNext so that algorithms which walk the face
* list will not see the newly created faces.
*/
unsafe extern "C" fn MakeFace(mut newFace: *mut TESSface,
                              mut eOrig: *mut TESShalfEdge,
                              mut fNext: *mut TESSface) {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut fPrev: *mut TESSface = 0 as *mut TESSface;
    let mut fNew: *mut TESSface = newFace;
    if fNew.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 9],
                                               &[libc::c_char; 9]>(b"MakeFace\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 159i32,
                     b"fNew != NULL\x00" as *const u8 as *const libc::c_char);
    } else { };
    /* insert in circular doubly-linked list before fNext */
    fPrev = (*fNext).prev;
    (*fNew).prev = fPrev;
    (*fPrev).next = fNew;
    (*fNew).next = fNext;
    (*fNext).prev = fNew;
    (*fNew).anEdge = eOrig;
    (*fNew).trail = 0 as *mut TESSface;
    (*fNew).marked = 0i32 as libc::c_char;
    /* The new face is marked "inside" if the old one was.  This is a
	* convenience for the common case where a face has been split in two.
	*/
    (*fNew).inside = (*fNext).inside;
    /* fix other edges on this face loop */
    e = eOrig;
    loop  { (*e).Lface = fNew; e = (*e).Lnext; if !(e != eOrig) { break ; } };
}
/* KillEdge( eDel ) destroys an edge (the half-edges eDel and eDel->Sym),
* and removes from the global edge list.
*/
unsafe extern "C" fn KillEdge(mut mesh: *mut TESSmesh,
                              mut eDel: *mut TESShalfEdge) {
    let mut ePrev: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    /* Half-edges are allocated in pairs, see EdgePair above */
    if (*eDel).Sym < eDel { eDel = (*eDel).Sym }
    /* delete from circular doubly-linked list */
    eNext = (*eDel).next;
    ePrev = (*(*eDel).Sym).next;
    (*(*eNext).Sym).next = ePrev;
    (*(*ePrev).Sym).next = eNext;
    bucketFree((*mesh).edgeBucket, eDel as *mut libc::c_void);
}
/* KillVertex( vDel ) destroys a vertex and removes it from the global
* vertex list.  It updates the vertex loop to point to a given new vertex.
*/
unsafe extern "C" fn KillVertex(mut mesh: *mut TESSmesh,
                                mut vDel: *mut TESSvertex,
                                mut newOrg: *mut TESSvertex) {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eStart: *mut TESShalfEdge = (*vDel).anEdge;
    let mut vPrev: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vNext: *mut TESSvertex = 0 as *mut TESSvertex;
    /* change the origin of all affected edges */
    e = eStart;
    loop  { (*e).Org = newOrg; e = (*e).Onext; if !(e != eStart) { break ; } }
    /* delete from circular doubly-linked list */
    vPrev = (*vDel).prev;
    vNext = (*vDel).next;
    (*vNext).prev = vPrev;
    (*vPrev).next = vNext;
    bucketFree((*mesh).vertexBucket, vDel as *mut libc::c_void);
}
/* KillFace( fDel ) destroys a face and removes it from the global face
* list.  It updates the face loop to point to a given new face.
*/
unsafe extern "C" fn KillFace(mut mesh: *mut TESSmesh,
                              mut fDel: *mut TESSface,
                              mut newLface: *mut TESSface) {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eStart: *mut TESShalfEdge = (*fDel).anEdge;
    let mut fPrev: *mut TESSface = 0 as *mut TESSface;
    let mut fNext: *mut TESSface = 0 as *mut TESSface;
    /* change the left face of all affected edges */
    e = eStart;
    loop  {
        (*e).Lface = newLface;
        e = (*e).Lnext;
        if !(e != eStart) { break ; }
    }
    /* delete from circular doubly-linked list */
    fPrev = (*fDel).prev;
    fNext = (*fDel).next;
    (*fNext).prev = fPrev;
    (*fPrev).next = fNext;
    bucketFree((*mesh).faceBucket, fDel as *mut libc::c_void);
}
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
/* ***************** Basic Edge Operations **********************/
/* tessMeshMakeEdge creates one edge, two vertices, and a loop (face).
* The loop consists of the two new half-edges.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshMakeEdge(mut mesh: *mut TESSmesh)
 -> *mut TESShalfEdge {
    let mut newVertex1: *mut TESSvertex =
        bucketAlloc((*mesh).vertexBucket) as *mut TESSvertex;
    let mut newVertex2: *mut TESSvertex =
        bucketAlloc((*mesh).vertexBucket) as *mut TESSvertex;
    let mut newFace: *mut TESSface =
        bucketAlloc((*mesh).faceBucket) as *mut TESSface;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    /* if any one is null then all get freed */
    if newVertex1.is_null() || newVertex2.is_null() || newFace.is_null() {
        if !newVertex1.is_null() {
            bucketFree((*mesh).vertexBucket, newVertex1 as *mut libc::c_void);
        }
        if !newVertex2.is_null() {
            bucketFree((*mesh).vertexBucket, newVertex2 as *mut libc::c_void);
        }
        if !newFace.is_null() {
            bucketFree((*mesh).faceBucket, newFace as *mut libc::c_void);
        }
        return 0 as *mut TESShalfEdge
    }
    e = MakeEdge(mesh, &mut (*mesh).eHead);
    if e.is_null() { return 0 as *mut TESShalfEdge }
    MakeVertex(newVertex1, e, &mut (*mesh).vHead);
    MakeVertex(newVertex2, (*e).Sym, &mut (*mesh).vHead);
    MakeFace(newFace, e, &mut (*mesh).fHead);
    return e;
}
/* tessMeshSplice( eOrg, eDst ) is the basic operation for changing the
* mesh connectivity and topology.  It changes the mesh so that
*	eOrg->Onext <- OLD( eDst->Onext )
*	eDst->Onext <- OLD( eOrg->Onext )
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
* Some special cases:
* If eDst == eOrg, the operation has no effect.
* If eDst == eOrg->Lnext, the new face will have a single edge.
* If eDst == eOrg->Lprev, the old face will have a single edge.
* If eDst == eOrg->Onext, the new vertex will have a single edge.
* If eDst == eOrg->Oprev, the old vertex will have a single edge.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshSplice(mut mesh: *mut TESSmesh,
                                        mut eOrg: *mut TESShalfEdge,
                                        mut eDst: *mut TESShalfEdge)
 -> libc::c_int {
    let mut joiningLoops: libc::c_int = 0i32;
    let mut joiningVertices: libc::c_int = 0i32;
    if eOrg == eDst { return 1i32 }
    if (*eDst).Org != (*eOrg).Org {
        /* We are merging two disjoint vertices -- destroy eDst->Org */
        joiningVertices = 1i32;
        KillVertex(mesh, (*eDst).Org, (*eOrg).Org);
    }
    if (*eDst).Lface != (*eOrg).Lface {
        /* We are connecting two disjoint loops -- destroy eDst->Lface */
        joiningLoops = 1i32;
        KillFace(mesh, (*eDst).Lface, (*eOrg).Lface);
    }
    /* Change the edge structure */
    Splice(eDst, eOrg);
    if joiningVertices == 0 {
        let mut newVertex: *mut TESSvertex =
            bucketAlloc((*mesh).vertexBucket) as *mut TESSvertex;
        if newVertex.is_null() { return 0i32 }
        /* We split one vertex into two -- the new vertex is eDst->Org.
		* Make sure the old vertex points to a valid half-edge.
		*/
        MakeVertex(newVertex, eDst, (*eOrg).Org);
        (*(*eOrg).Org).anEdge = eOrg
    }
    if joiningLoops == 0 {
        let mut newFace: *mut TESSface =
            bucketAlloc((*mesh).faceBucket) as *mut TESSface;
        if newFace.is_null() { return 0i32 }
        /* We split one loop into two -- the new loop is eDst->Lface.
		* Make sure the old face points to a valid half-edge.
		*/
        MakeFace(newFace, eDst, (*eOrg).Lface);
        (*(*eOrg).Lface).anEdge = eOrg
    }
    return 1i32;
}
/* tessMeshDelete( eDel ) removes the edge eDel.  There are several cases:
* if (eDel->Lface != eDel->Rface), we join two loops into one; the loop
* eDel->Lface is deleted.  Otherwise, we are splitting one loop into two;
* the newly created loop will contain eDel->Dst.  If the deletion of eDel
* would create isolated vertices, those are deleted as well.
*
* This function could be implemented as two calls to tessMeshSplice
* plus a few calls to memFree, but this would allocate and delete
* unnecessary vertices and faces.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshDelete(mut mesh: *mut TESSmesh,
                                        mut eDel: *mut TESShalfEdge)
 -> libc::c_int {
    let mut eDelSym: *mut TESShalfEdge = (*eDel).Sym;
    let mut joiningLoops: libc::c_int = 0i32;
    /* First step: disconnect the origin vertex eDel->Org.  We make all
	* changes to get a consistent mesh in this "intermediate" state.
	*/
    if (*eDel).Lface != (*(*eDel).Sym).Lface {
        /* We are joining two loops into one -- remove the left face */
        joiningLoops = 1i32;
        KillFace(mesh, (*eDel).Lface, (*(*eDel).Sym).Lface);
    }
    if (*eDel).Onext == eDel {
        KillVertex(mesh, (*eDel).Org, 0 as *mut TESSvertex);
    } else {
        /* Make sure that eDel->Org and eDel->Rface point to valid half-edges */
        (*(*(*eDel).Sym).Lface).anEdge = (*(*eDel).Sym).Lnext;
        (*(*eDel).Org).anEdge = (*eDel).Onext;
        Splice(eDel, (*(*eDel).Sym).Lnext);
        if joiningLoops == 0 {
            let mut newFace: *mut TESSface =
                bucketAlloc((*mesh).faceBucket) as *mut TESSface;
            if newFace.is_null() { return 0i32 }
            /* We are splitting one loop into two -- create a new loop for eDel. */
            MakeFace(newFace, eDel, (*eDel).Lface);
        }
    }
    /* Claim: the mesh is now in a consistent state, except that eDel->Org
	* may have been deleted.  Now we disconnect eDel->Dst.
	*/
    if (*eDelSym).Onext == eDelSym {
        KillVertex(mesh, (*eDelSym).Org, 0 as *mut TESSvertex);
        KillFace(mesh, (*eDelSym).Lface, 0 as *mut TESSface);
    } else {
        /* Make sure that eDel->Dst and eDel->Lface point to valid half-edges */
        (*(*eDel).Lface).anEdge = (*(*eDelSym).Sym).Lnext;
        (*(*eDelSym).Org).anEdge = (*eDelSym).Onext;
        Splice(eDelSym, (*(*eDelSym).Sym).Lnext);
    }
    /* Any isolated vertices or faces have already been freed. */
    KillEdge(mesh, eDel);
    return 1i32;
}
/* ******************* Other Edge Operations **********************/
/* All these routines can be implemented with the basic edge
* operations above.  They are provided for convenience and efficiency.
*/
/* tessMeshAddEdgeVertex( eOrg ) creates a new edge eNew such that
* eNew == eOrg->Lnext, and eNew->Dst is a newly created vertex.
* eOrg and eNew will have the same left face.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshAddEdgeVertex(mut mesh: *mut TESSmesh,
                                               mut eOrg: *mut TESShalfEdge)
 -> *mut TESShalfEdge {
    let mut eNewSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNew: *mut TESShalfEdge = MakeEdge(mesh, eOrg);
    if eNew.is_null() { return 0 as *mut TESShalfEdge }
    eNewSym = (*eNew).Sym;
    /* Connect the new edge appropriately */
    Splice(eNew, (*eOrg).Lnext);
    /* Set the vertex and face information */
    (*eNew).Org = (*(*eOrg).Sym).Org;
    let mut newVertex: *mut TESSvertex =
        bucketAlloc((*mesh).vertexBucket) as *mut TESSvertex;
    if newVertex.is_null() { return 0 as *mut TESShalfEdge }
    MakeVertex(newVertex, eNewSym, (*eNew).Org);
    (*eNewSym).Lface = (*eOrg).Lface;
    (*eNew).Lface = (*eNewSym).Lface;
    return eNew;
}
/* tessMeshSplitEdge( eOrg ) splits eOrg into two edges eOrg and eNew,
* such that eNew == eOrg->Lnext.  The new vertex is eOrg->Dst == eNew->Org.
* eOrg and eNew will have the same left face.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshSplitEdge(mut mesh: *mut TESSmesh,
                                           mut eOrg: *mut TESShalfEdge)
 -> *mut TESShalfEdge {
    let mut eNew: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut tempHalfEdge: *mut TESShalfEdge =
        tessMeshAddEdgeVertex(mesh, eOrg);
    if tempHalfEdge.is_null() { return 0 as *mut TESShalfEdge }
    eNew = (*tempHalfEdge).Sym;
    /* Disconnect eOrg from eOrg->Dst and connect it to eNew->Org */
    Splice((*eOrg).Sym, (*(*(*eOrg).Sym).Sym).Lnext);
    Splice((*eOrg).Sym, eNew);
    /* Set the vertex and face information */
    (*(*eOrg).Sym).Org = (*eNew).Org; /* may have pointed to eOrg->Sym */
    (*(*(*eNew).Sym).Org).anEdge =
        (*eNew).Sym; /* copy old winding information */
    (*(*eNew).Sym).Lface = (*(*eOrg).Sym).Lface;
    (*eNew).winding = (*eOrg).winding;
    (*(*eNew).Sym).winding = (*(*eOrg).Sym).winding;
    return eNew;
}
/* tessMeshConnect( eOrg, eDst ) creates a new edge from eOrg->Dst
* to eDst->Org, and returns the corresponding half-edge eNew.
* If eOrg->Lface == eDst->Lface, this splits one loop into two,
* and the newly created loop is eNew->Lface.  Otherwise, two disjoint
* loops are merged into one, and the loop eDst->Lface is destroyed.
*
* If (eOrg == eDst), the new face will have only two edges.
* If (eOrg->Lnext == eDst), the old face is reduced to a single edge.
* If (eOrg->Lnext->Lnext == eDst), the old face is reduced to two edges.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshConnect(mut mesh: *mut TESSmesh,
                                         mut eOrg: *mut TESShalfEdge,
                                         mut eDst: *mut TESShalfEdge)
 -> *mut TESShalfEdge {
    let mut eNewSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut joiningLoops: libc::c_int = 0i32;
    let mut eNew: *mut TESShalfEdge = MakeEdge(mesh, eOrg);
    if eNew.is_null() { return 0 as *mut TESShalfEdge }
    eNewSym = (*eNew).Sym;
    if (*eDst).Lface != (*eOrg).Lface {
        /* We are connecting two disjoint loops -- destroy eDst->Lface */
        joiningLoops = 1i32;
        KillFace(mesh, (*eDst).Lface, (*eOrg).Lface);
    }
    /* Connect the new edge appropriately */
    Splice(eNew, (*eOrg).Lnext);
    Splice(eNewSym, eDst);
    /* Set the vertex and face information */
    (*eNew).Org = (*(*eOrg).Sym).Org;
    (*eNewSym).Org = (*eDst).Org;
    (*eNewSym).Lface = (*eOrg).Lface;
    (*eNew).Lface = (*eNewSym).Lface;
    /* Make sure the old face points to a valid half-edge */
    (*(*eOrg).Lface).anEdge = eNewSym;
    if joiningLoops == 0 {
        let mut newFace: *mut TESSface =
            bucketAlloc((*mesh).faceBucket) as *mut TESSface;
        if newFace.is_null() { return 0 as *mut TESShalfEdge }
        /* We split one loop into two -- the new loop is eNew->Lface */
        MakeFace(newFace, eNew, (*eOrg).Lface);
    }
    return eNew;
}
/* ******************* Other Operations **********************/
/* tessMeshZapFace( fZap ) destroys a face and removes it from the
* global face list.  All edges of fZap will have a NULL pointer as their
* left face.  Any edges which also have a NULL pointer as their right face
* are deleted entirely (along with any isolated vertices this produces).
* An entire mesh can be deleted by zapping its faces, one at a time,
* in any order.  Zapped faces cannot be used in further mesh operations!
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshZapFace(mut mesh: *mut TESSmesh,
                                         mut fZap: *mut TESSface) {
    let mut eStart: *mut TESShalfEdge = (*fZap).anEdge;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut fPrev: *mut TESSface = 0 as *mut TESSface;
    let mut fNext: *mut TESSface = 0 as *mut TESSface;
    /* walk around face, deleting edges whose right face is also NULL */
    eNext = (*eStart).Lnext;
    loop  {
        e = eNext;
        eNext = (*e).Lnext;
        (*e).Lface = 0 as *mut TESSface;
        if (*(*e).Sym).Lface.is_null() {
            /* delete the edge -- see TESSmeshDelete above */
            if (*e).Onext == e {
                KillVertex(mesh, (*e).Org, 0 as *mut TESSvertex);
            } else {
                /* Make sure that e->Org points to a valid half-edge */
                (*(*e).Org).anEdge = (*e).Onext;
                Splice(e, (*(*e).Sym).Lnext);
            }
            eSym = (*e).Sym;
            if (*eSym).Onext == eSym {
                KillVertex(mesh, (*eSym).Org, 0 as *mut TESSvertex);
            } else {
                /* Make sure that eSym->Org points to a valid half-edge */
                (*(*eSym).Org).anEdge = (*eSym).Onext;
                Splice(eSym, (*(*eSym).Sym).Lnext);
            }
            KillEdge(mesh, e);
        }
        if !(e != eStart) { break ; }
    }
    /* delete from circular doubly-linked list */
    fPrev = (*fZap).prev;
    fNext = (*fZap).next;
    (*fNext).prev = fPrev;
    (*fPrev).next = fNext;
    bucketFree((*mesh).faceBucket, fZap as *mut libc::c_void);
}
/* tessMeshNewMesh() creates a new mesh with no edges, no vertices,
* and no loops (what we usually call a "face").
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshNewMesh(mut alloc: *mut TESSalloc)
 -> *mut TESSmesh {
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut mesh: *mut TESSmesh =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<TESSmesh>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut TESSmesh;
    if mesh.is_null() { return 0 as *mut TESSmesh }
    if (*alloc).meshEdgeBucketSize < 16i32 {
        (*alloc).meshEdgeBucketSize = 16i32
    }
    if (*alloc).meshEdgeBucketSize > 4096i32 {
        (*alloc).meshEdgeBucketSize = 4096i32
    }
    if (*alloc).meshVertexBucketSize < 16i32 {
        (*alloc).meshVertexBucketSize = 16i32
    }
    if (*alloc).meshVertexBucketSize > 4096i32 {
        (*alloc).meshVertexBucketSize = 4096i32
    }
    if (*alloc).meshFaceBucketSize < 16i32 {
        (*alloc).meshFaceBucketSize = 16i32
    }
    if (*alloc).meshFaceBucketSize > 4096i32 {
        (*alloc).meshFaceBucketSize = 4096i32
    }
    (*mesh).edgeBucket =
        createBucketAlloc(alloc,
                          b"Mesh Edges\x00" as *const u8 as
                              *const libc::c_char,
                          ::std::mem::size_of::<EdgePair>() as libc::c_ulong
                              as libc::c_uint,
                          (*alloc).meshEdgeBucketSize as libc::c_uint);
    (*mesh).vertexBucket =
        createBucketAlloc(alloc,
                          b"Mesh Vertices\x00" as *const u8 as
                              *const libc::c_char,
                          ::std::mem::size_of::<TESSvertex>() as libc::c_ulong
                              as libc::c_uint,
                          (*alloc).meshVertexBucketSize as libc::c_uint);
    (*mesh).faceBucket =
        createBucketAlloc(alloc,
                          b"Mesh Faces\x00" as *const u8 as
                              *const libc::c_char,
                          ::std::mem::size_of::<TESSface>() as libc::c_ulong
                              as libc::c_uint,
                          (*alloc).meshFaceBucketSize as libc::c_uint);
    v = &mut (*mesh).vHead;
    f = &mut (*mesh).fHead;
    e = &mut (*mesh).eHead;
    eSym = &mut (*mesh).eHeadSym;
    (*v).prev = v;
    (*v).next = (*v).prev;
    (*v).anEdge = 0 as *mut TESShalfEdge;
    (*f).prev = f;
    (*f).next = (*f).prev;
    (*f).anEdge = 0 as *mut TESShalfEdge;
    (*f).trail = 0 as *mut TESSface;
    (*f).marked = 0i32 as libc::c_char;
    (*f).inside = 0i32 as libc::c_char;
    (*e).next = e;
    (*e).Sym = eSym;
    (*e).Onext = 0 as *mut TESShalfEdge;
    (*e).Lnext = 0 as *mut TESShalfEdge;
    (*e).Org = 0 as *mut TESSvertex;
    (*e).Lface = 0 as *mut TESSface;
    (*e).winding = 0i32;
    (*e).activeRegion = 0 as *mut ActiveRegion;
    (*eSym).next = eSym;
    (*eSym).Sym = e;
    (*eSym).Onext = 0 as *mut TESShalfEdge;
    (*eSym).Lnext = 0 as *mut TESShalfEdge;
    (*eSym).Org = 0 as *mut TESSvertex;
    (*eSym).Lface = 0 as *mut TESSface;
    (*eSym).winding = 0i32;
    (*eSym).activeRegion = 0 as *mut ActiveRegion;
    return mesh;
}
/* tessMeshUnion( mesh1, mesh2 ) forms the union of all structures in
* both meshes, and returns the new mesh (the old meshes are destroyed).
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshUnion(mut alloc: *mut TESSalloc,
                                       mut mesh1: *mut TESSmesh,
                                       mut mesh2: *mut TESSmesh)
 -> *mut TESSmesh {
    let mut f1: *mut TESSface = &mut (*mesh1).fHead;
    let mut v1: *mut TESSvertex = &mut (*mesh1).vHead;
    let mut e1: *mut TESShalfEdge = &mut (*mesh1).eHead;
    let mut f2: *mut TESSface = &mut (*mesh2).fHead;
    let mut v2: *mut TESSvertex = &mut (*mesh2).vHead;
    let mut e2: *mut TESShalfEdge = &mut (*mesh2).eHead;
    /* Add the faces, vertices, and edges of mesh2 to those of mesh1 */
    if (*f2).next != f2 {
        (*(*f1).prev).next = (*f2).next;
        (*(*f2).next).prev = (*f1).prev;
        (*(*f2).prev).next = f1;
        (*f1).prev = (*f2).prev
    }
    if (*v2).next != v2 {
        (*(*v1).prev).next = (*v2).next;
        (*(*v2).next).prev = (*v1).prev;
        (*(*v2).prev).next = v1;
        (*v1).prev = (*v2).prev
    }
    if (*e2).next != e2 {
        (*(*(*(*e1).Sym).next).Sym).next = (*e2).next;
        (*(*(*e2).next).Sym).next = (*(*e1).Sym).next;
        (*(*(*(*e2).Sym).next).Sym).next = e1;
        (*(*e1).Sym).next = (*(*e2).Sym).next
    }
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         mesh2 as
                                                             *mut libc::c_void);
    return mesh1;
}
unsafe extern "C" fn CountFaceVerts(mut f: *mut TESSface) -> libc::c_int {
    let mut eCur: *mut TESShalfEdge = (*f).anEdge;
    let mut n: libc::c_int = 0i32;
    loop  {
        n += 1;
        eCur = (*eCur).Lnext;
        if !(eCur != (*f).anEdge) { break ; }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn tessMeshMergeConvexFaces(mut mesh: *mut TESSmesh,
                                                  mut maxVertsPerFace:
                                                      libc::c_int)
 -> libc::c_int {
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eNext: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eSym: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut eHead: *mut TESShalfEdge = &mut (*mesh).eHead;
    let mut va: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vb: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vc: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vd: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut ve: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vf: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut leftNv: libc::c_int = 0;
    let mut rightNv: libc::c_int = 0;
    e = (*eHead).next;
    while e != eHead {
        eNext = (*e).next;
        eSym = (*e).Sym;
        if !eSym.is_null() {
            // Both faces must be inside
            if !((*e).Lface.is_null() || (*(*e).Lface).inside == 0) {
                if !((*eSym).Lface.is_null() || (*(*eSym).Lface).inside == 0)
                   {
                    leftNv = CountFaceVerts((*e).Lface);
                    rightNv = CountFaceVerts((*eSym).Lface);
                    if !(leftNv + rightNv - 2i32 > maxVertsPerFace) {
                        // Merge if the resulting poly is convex.
		//
		//      vf--ve--vd
		//          ^|
		// left   e ||   right
		//          |v
		//      va--vb--vc
                        va = (*(*(*e).Onext).Sym).Org;
                        vb = (*e).Org;
                        vc = (*(*(*(*e).Sym).Lnext).Sym).Org;
                        vd = (*(*(*(*e).Sym).Onext).Sym).Org;
                        ve = (*(*e).Sym).Org;
                        vf = (*(*(*e).Lnext).Sym).Org;
                        if tesvertCCW(va, vb, vc) != 0 &&
                               tesvertCCW(vd, ve, vf) != 0 {
                            if e == eNext || e == (*eNext).Sym {
                                eNext = (*eNext).next
                            }
                            if tessMeshDelete(mesh, e) == 0 { return 0i32 }
                        }
                    }
                }
            }
        }
        e = eNext
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tessMeshFlipEdge(mut mesh: *mut TESSmesh,
                                          mut edge: *mut TESShalfEdge) {
    let mut a0: *mut TESShalfEdge = edge;
    let mut a1: *mut TESShalfEdge = (*a0).Lnext;
    let mut a2: *mut TESShalfEdge = (*a1).Lnext;
    let mut b0: *mut TESShalfEdge = (*edge).Sym;
    let mut b1: *mut TESShalfEdge = (*b0).Lnext;
    let mut b2: *mut TESShalfEdge = (*b1).Lnext;
    let mut aOrg: *mut TESSvertex = (*a0).Org;
    let mut aOpp: *mut TESSvertex = (*a2).Org;
    let mut bOrg: *mut TESSvertex = (*b0).Org;
    let mut bOpp: *mut TESSvertex = (*b2).Org;
    let mut fa: *mut TESSface = (*a0).Lface;
    let mut fb: *mut TESSface = (*b0).Lface;
    if !(!(*(*edge).Sym).Lface.is_null() &&
             (*(*(*edge).Sym).Lface).inside as libc::c_int != 0) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 768i32,
                     b"EdgeIsInternal(edge)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*a2).Lnext == a0) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 769i32,
                     b"a2->Lnext == a0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*b2).Lnext == b0) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 770i32,
                     b"b2->Lnext == b0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    (*a0).Org = bOpp;
    (*a0).Onext = (*b1).Sym;
    (*b0).Org = aOpp;
    (*b0).Onext = (*a1).Sym;
    (*a2).Onext = b0;
    (*b2).Onext = a0;
    (*b1).Onext = (*a2).Sym;
    (*a1).Onext = (*b2).Sym;
    (*a0).Lnext = a2;
    (*a2).Lnext = b1;
    (*b1).Lnext = a0;
    (*b0).Lnext = b2;
    (*b2).Lnext = a1;
    (*a1).Lnext = b0;
    (*a1).Lface = fb;
    (*b1).Lface = fa;
    (*fa).anEdge = a0;
    (*fb).anEdge = b0;
    if (*aOrg).anEdge == a0 { (*aOrg).anEdge = b1 }
    if (*bOrg).anEdge == b0 { (*bOrg).anEdge = a1 }
    if !((*(*(*a0).Lnext).Onext).Sym == a0) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 798i32,
                     b"a0->Lnext->Onext->Sym == a0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a0).Onext).Sym).Lnext == a0) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 799i32,
                     b"a0->Onext->Sym->Lnext == a0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a0).Org).anEdge).Org == (*a0).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 800i32,
                     b"a0->Org->anEdge->Org == a0->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a1).Lnext).Onext).Sym == a1) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 803i32,
                     b"a1->Lnext->Onext->Sym == a1\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a1).Onext).Sym).Lnext == a1) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 804i32,
                     b"a1->Onext->Sym->Lnext == a1\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a1).Org).anEdge).Org == (*a1).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 805i32,
                     b"a1->Org->anEdge->Org == a1->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a2).Lnext).Onext).Sym == a2) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 807i32,
                     b"a2->Lnext->Onext->Sym == a2\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a2).Onext).Sym).Lnext == a2) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 808i32,
                     b"a2->Onext->Sym->Lnext == a2\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*a2).Org).anEdge).Org == (*a2).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 809i32,
                     b"a2->Org->anEdge->Org == a2->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b0).Lnext).Onext).Sym == b0) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 811i32,
                     b"b0->Lnext->Onext->Sym == b0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b0).Onext).Sym).Lnext == b0) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 812i32,
                     b"b0->Onext->Sym->Lnext == b0\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b0).Org).anEdge).Org == (*b0).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 813i32,
                     b"b0->Org->anEdge->Org == b0->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b1).Lnext).Onext).Sym == b1) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 815i32,
                     b"b1->Lnext->Onext->Sym == b1\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b1).Onext).Sym).Lnext == b1) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 816i32,
                     b"b1->Onext->Sym->Lnext == b1\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b1).Org).anEdge).Org == (*b1).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 817i32,
                     b"b1->Org->anEdge->Org == b1->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b2).Lnext).Onext).Sym == b2) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 819i32,
                     b"b2->Lnext->Onext->Sym == b2\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b2).Onext).Sym).Lnext == b2) as libc::c_int as libc::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 820i32,
                     b"b2->Onext->Sym->Lnext == b2\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*b2).Org).anEdge).Org == (*b2).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 821i32,
                     b"b2->Org->anEdge->Org == b2->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*aOrg).anEdge).Org == aOrg) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 823i32,
                     b"aOrg->anEdge->Org == aOrg\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*bOrg).anEdge).Org == bOrg) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 824i32,
                     b"bOrg->anEdge->Org == bOrg\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if !((*(*(*(*a0).Sym).Lnext).Onext).Org == (*a0).Org) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"tessMeshFlipEdge\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 826i32,
                     b"a0->Oprev->Onext->Org == a0->Org\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
}
/* tessMeshDeleteMesh( mesh ) will free all storage for any valid mesh.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshDeleteMesh(mut alloc: *mut TESSalloc,
                                            mut mesh: *mut TESSmesh) {
    deleteBucketAlloc((*mesh).edgeBucket);
    deleteBucketAlloc((*mesh).vertexBucket);
    deleteBucketAlloc((*mesh).faceBucket);
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         mesh as
                                                             *mut libc::c_void);
}
/* tessMeshCheckMesh( mesh ) checks a mesh for self-consistency.
*/
#[no_mangle]
pub unsafe extern "C" fn tessMeshCheckMesh(mut mesh: *mut TESSmesh) {
    let mut fHead: *mut TESSface = &mut (*mesh).fHead;
    let mut vHead: *mut TESSvertex = &mut (*mesh).vHead;
    let mut eHead: *mut TESShalfEdge = &mut (*mesh).eHead;
    let mut f: *mut TESSface = 0 as *mut TESSface;
    let mut fPrev: *mut TESSface = 0 as *mut TESSface;
    let mut v: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut vPrev: *mut TESSvertex = 0 as *mut TESSvertex;
    let mut e: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    let mut ePrev: *mut TESShalfEdge = 0 as *mut TESShalfEdge;
    fPrev = fHead;
    loop  {
        f = (*fPrev).next;
        if !(f != fHead) { break ; }
        if !((*f).prev == fPrev) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 874i32,
                         b"f->prev == fPrev\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        e = (*f).anEdge;
        loop  {
            if !((*e).Sym != e) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 877i32,
                             b"e->Sym != e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*e).Sym).Sym == e) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 878i32,
                             b"e->Sym->Sym == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*(*e).Lnext).Onext).Sym == e) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 879i32,
                             b"e->Lnext->Onext->Sym == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*(*e).Onext).Sym).Lnext == e) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 880i32,
                             b"e->Onext->Sym->Lnext == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*e).Lface == f) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 881i32,
                             b"e->Lface == f\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            e = (*e).Lnext;
            if !(e != (*f).anEdge) { break ; }
        }
        fPrev = f
    }
    if !((*f).prev == fPrev && (*f).anEdge.is_null()) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 885i32,
                     b"f->prev == fPrev && f->anEdge == NULL\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    vPrev = vHead;
    loop  {
        v = (*vPrev).next;
        if !(v != vHead) { break ; }
        if !((*v).prev == vPrev) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 888i32,
                         b"v->prev == vPrev\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        e = (*v).anEdge;
        loop  {
            if !((*e).Sym != e) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 891i32,
                             b"e->Sym != e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*e).Sym).Sym == e) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 892i32,
                             b"e->Sym->Sym == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*(*e).Lnext).Onext).Sym == e) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 893i32,
                             b"e->Lnext->Onext->Sym == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*(*(*e).Onext).Sym).Lnext == e) as libc::c_int as
                   libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 894i32,
                             b"e->Onext->Sym->Lnext == e\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            if !((*e).Org == v) as libc::c_int as libc::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                             b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                                 as *const u8 as *const libc::c_char, 895i32,
                             b"e->Org == v\x00" as *const u8 as
                                 *const libc::c_char);
            } else { };
            e = (*e).Onext;
            if !(e != (*v).anEdge) { break ; }
        }
        vPrev = v
    }
    if !((*v).prev == vPrev && (*v).anEdge.is_null()) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 899i32,
                     b"v->prev == vPrev && v->anEdge == NULL\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    ePrev = eHead;
    loop  {
        e = (*ePrev).next;
        if !(e != eHead) { break ; }
        if !((*(*e).Sym).next == (*ePrev).Sym) as libc::c_int as libc::c_long
               != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 902i32,
                         b"e->Sym->next == ePrev->Sym\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if !((*e).Sym != e) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 903i32,
                         b"e->Sym != e\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if !((*(*e).Sym).Sym == e) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 904i32,
                         b"e->Sym->Sym == e\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if (*e).Org.is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 905i32,
                         b"e->Org != NULL\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if (*(*e).Sym).Org.is_null() as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 906i32,
                         b"e->Dst != NULL\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if !((*(*(*e).Lnext).Onext).Sym == e) as libc::c_int as libc::c_long
               != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 907i32,
                         b"e->Lnext->Onext->Sym == e\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        if !((*(*(*e).Onext).Sym).Lnext == e) as libc::c_int as libc::c_long
               != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                             as *const u8 as *const libc::c_char, 908i32,
                         b"e->Onext->Sym->Lnext == e\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        ePrev = e
    }
    if !((*(*e).Sym).next == (*ePrev).Sym &&
             (*e).Sym == &mut (*mesh).eHeadSym as *mut TESShalfEdge &&
             (*(*e).Sym).Sym == e && (*e).Org.is_null() &&
             (*(*e).Sym).Org.is_null() && (*e).Lface.is_null() &&
             (*(*e).Sym).Lface.is_null()) as libc::c_int as libc::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"tessMeshCheckMesh\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/mesh.c\x00"
                         as *const u8 as *const libc::c_char, 914i32,
                     b"e->Sym->next == ePrev->Sym && e->Sym == &mesh->eHeadSym && e->Sym->Sym == e && e->Org == NULL && e->Dst == NULL && e->Lface == NULL && e->Rface == NULL\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
}
