use libc;
extern "C" {
    pub type ActiveRegion;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
}
pub type size_t = libc::c_ulong;
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


pub fn wrapping_offset_from<T>(_self: *const T, origin: *const T) -> isize where T: Sized {
        let pointee_size = std::mem::size_of::<T>();
        assert!(0 < pointee_size && pointee_size <= isize::max_value() as usize);

        let d = isize::wrapping_sub(_self as _, origin as _);
        d.wrapping_div(pointee_size as _)
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
pub struct PriorityQ {
    pub heap: *mut PriorityQHeap,
    pub keys: *mut PQkey,
    pub order: *mut *mut PQkey,
    pub size: PQhandle,
    pub max: PQhandle,
    pub initialized: libc::c_int,
    pub leq: Option<unsafe extern "C" fn(_: PQkey, _: PQkey) -> libc::c_int>,
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
pub struct C2RustUnnamed {
    pub p: *mut *mut PQkey,
    pub r: *mut *mut PQkey,
}
/* Violates modularity, but a little faster */
/* Include all the code for the regular heap-based queue here. */
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
/* really pqHeapNewPriorityQHeap */
#[no_mangle]
pub unsafe extern "C" fn pqHeapNewPriorityQ(mut alloc: *mut TESSalloc,
                                            mut size: libc::c_int,
                                            mut leq:
                                                Option<unsafe extern "C" fn(_:
                                                                                PQkey,
                                                                            _:
                                                                                PQkey)
                                                           -> libc::c_int>)
 -> *mut PriorityQHeap {
    let mut pq: *mut PriorityQHeap =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<PriorityQHeap>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut PriorityQHeap; /* so that Minimum() returns NULL */
    if pq.is_null() { return 0 as *mut PriorityQHeap }
    (*pq).size = 0i32;
    (*pq).max = size;
    (*pq).nodes =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ((size + 1i32)
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQnode>()
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                  as
                                                                  libc::c_uint)
            as *mut PQnode;
    if (*pq).nodes.is_null() {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             pq as
                                                                 *mut libc::c_void);
        return 0 as *mut PriorityQHeap
    }
    (*pq).handles =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ((size + 1i32)
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQhandleElem>()
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                  as
                                                                  libc::c_uint)
            as *mut PQhandleElem;
    if (*pq).handles.is_null() {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             (*pq).nodes as
                                                                 *mut libc::c_void);
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             pq as
                                                                 *mut libc::c_void);
        return 0 as *mut PriorityQHeap
    }
    (*pq).initialized = 0i32;
    (*pq).freeList = 0i32;
    (*pq).leq = leq;
    (*(*pq).nodes.offset(1)).handle = 1i32;
    let ref mut fresh0 = (*(*pq).handles.offset(1)).key;
    *fresh0 = 0 as *mut libc::c_void;
    return pq;
}
/* really pqHeapDeletePriorityQHeap */
#[no_mangle]
pub unsafe extern "C" fn pqHeapDeletePriorityQ(mut alloc: *mut TESSalloc,
                                               mut pq: *mut PriorityQHeap) {
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         (*pq).handles as
                                                             *mut libc::c_void);
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         (*pq).nodes as
                                                             *mut libc::c_void);
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         pq as
                                                             *mut libc::c_void);
}
unsafe extern "C" fn FloatDown(mut pq: *mut PriorityQHeap,
                               mut curr: libc::c_int) {
    let mut n: *mut PQnode = (*pq).nodes;
    let mut h: *mut PQhandleElem = (*pq).handles;
    let mut hCurr: PQhandle = 0;
    let mut hChild: PQhandle = 0;
    let mut child: libc::c_int = 0;
    hCurr = (*n.offset(curr as isize)).handle;
    loop  {
        child = curr << 1i32;
        if child < (*pq).size &&
               ((*((*h.offset((*n.offset((child + 1i32) as isize)).handle as
                                  isize)).key as *mut TESSvertex)).s <
                    (*((*h.offset((*n.offset(child as isize)).handle as
                                      isize)).key as *mut TESSvertex)).s ||
                    (*((*h.offset((*n.offset((child + 1i32) as isize)).handle
                                      as isize)).key as *mut TESSvertex)).s ==
                        (*((*h.offset((*n.offset(child as isize)).handle as
                                          isize)).key as *mut TESSvertex)).s
                        &&
                        (*((*h.offset((*n.offset((child + 1i32) as
                                                     isize)).handle as
                                          isize)).key as *mut TESSvertex)).t
                            <=
                            (*((*h.offset((*n.offset(child as isize)).handle
                                              as isize)).key as
                                   *mut TESSvertex)).t) {
            child += 1
        }
        if !(child <= (*pq).max) as libc::c_int as libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                                   &[libc::c_char; 10]>(b"FloatDown\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                             as *const u8 as *const libc::c_char, 141i32,
                         b"child <= pq->max\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        hChild = (*n.offset(child as isize)).handle;
        if child > (*pq).size ||
               ((*((*h.offset(hCurr as isize)).key as *mut TESSvertex)).s <
                    (*((*h.offset(hChild as isize)).key as *mut TESSvertex)).s
                    ||
                    (*((*h.offset(hCurr as isize)).key as *mut TESSvertex)).s
                        ==
                        (*((*h.offset(hChild as isize)).key as
                               *mut TESSvertex)).s &&
                        (*((*h.offset(hCurr as isize)).key as
                               *mut TESSvertex)).t <=
                            (*((*h.offset(hChild as isize)).key as
                                   *mut TESSvertex)).t) {
            (*n.offset(curr as isize)).handle = hCurr;
            (*h.offset(hCurr as isize)).node = curr;
            break ;
        } else {
            (*n.offset(curr as isize)).handle = hChild;
            (*h.offset(hChild as isize)).node = curr;
            curr = child
        }
    };
}
unsafe extern "C" fn FloatUp(mut pq: *mut PriorityQHeap,
                             mut curr: libc::c_int) {
    let mut n: *mut PQnode = (*pq).nodes;
    let mut h: *mut PQhandleElem = (*pq).handles;
    let mut hCurr: PQhandle = 0;
    let mut hParent: PQhandle = 0;
    let mut parent: libc::c_int = 0;
    hCurr = (*n.offset(curr as isize)).handle;
    loop  {
        parent = curr >> 1i32;
        hParent = (*n.offset(parent as isize)).handle;
        if parent == 0i32 ||
               ((*((*h.offset(hParent as isize)).key as *mut TESSvertex)).s <
                    (*((*h.offset(hCurr as isize)).key as *mut TESSvertex)).s
                    ||
                    (*((*h.offset(hParent as isize)).key as
                           *mut TESSvertex)).s ==
                        (*((*h.offset(hCurr as isize)).key as
                               *mut TESSvertex)).s &&
                        (*((*h.offset(hParent as isize)).key as
                               *mut TESSvertex)).t <=
                            (*((*h.offset(hCurr as isize)).key as
                                   *mut TESSvertex)).t) {
            (*n.offset(curr as isize)).handle = hCurr;
            (*h.offset(hCurr as isize)).node = curr;
            break ;
        } else {
            (*n.offset(curr as isize)).handle = hParent;
            (*h.offset(hParent as isize)).node = curr;
            curr = parent
        }
    };
}
/* really pqHeapInit */
#[no_mangle]
pub unsafe extern "C" fn pqHeapInit(mut pq: *mut PriorityQHeap) {
    let mut i: libc::c_int = 0;
    /* This method of building a heap is O(n), rather than O(n lg n). */
    i = (*pq).size;
    while i >= 1i32 { FloatDown(pq, i); i -= 1 }
    (*pq).initialized = 1i32;
}
/* really pqHeapInsert */
/* returns INV_HANDLE iff out of memory */
#[no_mangle]
pub unsafe extern "C" fn pqHeapInsert(mut alloc: *mut TESSalloc,
                                      mut pq: *mut PriorityQHeap,
                                      mut keyNew: PQkey) -> PQhandle {
    let mut curr: libc::c_int = 0;
    let mut free: PQhandle = 0;
    (*pq).size += 1;
    curr = (*pq).size;
    if curr * 2i32 > (*pq).max {
        if (*alloc).memrealloc.is_none() {
            return 0xfffffffi32
        } else {
            let mut saveNodes: *mut PQnode = (*pq).nodes;
            let mut saveHandles: *mut PQhandleElem = (*pq).handles;
            // If the heap overflows, double its size.
            (*pq).max <<= 1i32; // restore ptr to free upon return
            (*pq).nodes =
                (*alloc).memrealloc.expect("non-null function pointer")((*alloc).userData,
                                                                        (*pq).nodes
                                                                            as
                                                                            *mut libc::c_void,
                                                                        (((*pq).max
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQnode>()
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            libc::c_uint)
                    as *mut PQnode; // restore ptr to free upon return
            if (*pq).nodes.is_null() {
                (*pq).nodes = saveNodes;
                return 0xfffffffi32
            }
            (*pq).handles =
                (*alloc).memrealloc.expect("non-null function pointer")((*alloc).userData,
                                                                        (*pq).handles
                                                                            as
                                                                            *mut libc::c_void,
                                                                        (((*pq).max
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQhandleElem>()
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            libc::c_uint)
                    as *mut PQhandleElem;
            if (*pq).handles.is_null() {
                (*pq).handles = saveHandles;
                return 0xfffffffi32
            }
        }
    }
    if (*pq).freeList == 0i32 {
        free = curr
    } else {
        free = (*pq).freeList;
        (*pq).freeList = (*(*pq).handles.offset(free as isize)).node
    }
    (*(*pq).nodes.offset(curr as isize)).handle = free;
    (*(*pq).handles.offset(free as isize)).node = curr;
    let ref mut fresh1 = (*(*pq).handles.offset(free as isize)).key;
    *fresh1 = keyNew;
    if (*pq).initialized != 0 { FloatUp(pq, curr); }
    if !(free != 0xfffffffi32) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"pqHeapInsert\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                         as *const u8 as *const libc::c_char, 240i32,
                     b"free != INV_HANDLE\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    return free;
}
/* really pqHeapExtractMin */
#[no_mangle]
pub unsafe extern "C" fn pqHeapExtractMin(mut pq: *mut PriorityQHeap)
 -> PQkey {
    let mut n: *mut PQnode = (*pq).nodes;
    let mut h: *mut PQhandleElem = (*pq).handles;
    let mut hMin: PQhandle = (*n.offset(1)).handle;
    let mut min: PQkey = (*h.offset(hMin as isize)).key;
    if (*pq).size > 0i32 {
        (*n.offset(1)).handle = (*n.offset((*pq).size as isize)).handle;
        (*h.offset((*n.offset(1)).handle as isize)).node = 1i32;
        let ref mut fresh2 = (*h.offset(hMin as isize)).key;
        *fresh2 = 0 as *mut libc::c_void;
        (*h.offset(hMin as isize)).node = (*pq).freeList;
        (*pq).freeList = hMin;
        (*pq).size -= 1;
        if (*pq).size > 0i32 { FloatDown(pq, 1i32); }
    }
    return min;
}
/* really pqHeapDelete */
#[no_mangle]
pub unsafe extern "C" fn pqHeapDelete(mut pq: *mut PriorityQHeap,
                                      mut hCurr: PQhandle) {
    let mut n: *mut PQnode = (*pq).nodes;
    let mut h: *mut PQhandleElem = (*pq).handles;
    let mut curr: libc::c_int = 0;
    if !(hCurr >= 1i32 && hCurr <= (*pq).max &&
             !(*h.offset(hCurr as isize)).key.is_null()) as libc::c_int as
           libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"pqHeapDelete\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                         as *const u8 as *const libc::c_char, 274i32,
                     b"hCurr >= 1 && hCurr <= pq->max && h[hCurr].key != NULL\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    curr = (*h.offset(hCurr as isize)).node;
    (*n.offset(curr as isize)).handle =
        (*n.offset((*pq).size as isize)).handle;
    (*h.offset((*n.offset(curr as isize)).handle as isize)).node = curr;
    (*pq).size -= 1;
    if curr <= (*pq).size {
        if curr <= 1i32 ||
               ((*((*h.offset((*n.offset((curr >> 1i32) as isize)).handle as
                                  isize)).key as *mut TESSvertex)).s <
                    (*((*h.offset((*n.offset(curr as isize)).handle as
                                      isize)).key as *mut TESSvertex)).s ||
                    (*((*h.offset((*n.offset((curr >> 1i32) as isize)).handle
                                      as isize)).key as *mut TESSvertex)).s ==
                        (*((*h.offset((*n.offset(curr as isize)).handle as
                                          isize)).key as *mut TESSvertex)).s
                        &&
                        (*((*h.offset((*n.offset((curr >> 1i32) as
                                                     isize)).handle as
                                          isize)).key as *mut TESSvertex)).t
                            <=
                            (*((*h.offset((*n.offset(curr as isize)).handle as
                                              isize)).key as
                                   *mut TESSvertex)).t) {
            FloatDown(pq, curr);
        } else { FloatUp(pq, curr); }
    }
    let ref mut fresh3 = (*h.offset(hCurr as isize)).key;
    *fresh3 = 0 as *mut libc::c_void;
    (*h.offset(hCurr as isize)).node = (*pq).freeList;
    (*pq).freeList = hCurr;
}
/* Now redefine all the function names to map to their "Sort" versions. */
/* really tessPqSortNewPriorityQ */
#[no_mangle]
pub unsafe extern "C" fn pqNewPriorityQ(mut alloc: *mut TESSalloc,
                                        mut size: libc::c_int,
                                        mut leq:
                                            Option<unsafe extern "C" fn(_:
                                                                            PQkey,
                                                                        _:
                                                                            PQkey)
                                                       -> libc::c_int>)
 -> *mut PriorityQ {
    let mut pq: *mut PriorityQ =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<PriorityQ>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut PriorityQ;
    if pq.is_null() { return 0 as *mut PriorityQ }
    (*pq).heap = pqHeapNewPriorityQ(alloc, size, leq);
    if (*pq).heap.is_null() {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             pq as
                                                                 *mut libc::c_void);
        return 0 as *mut PriorityQ
    }
    //	pq->keys = (PQkey *)memAlloc( INIT_SIZE * sizeof(pq->keys[0]) );
    (*pq).keys =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              (size as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQkey>()
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                  as
                                                                  libc::c_uint)
            as *mut PQkey; //INIT_SIZE;
    if (*pq).keys.is_null() {
        pqHeapDeletePriorityQ(alloc, (*pq).heap);
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             pq as
                                                                 *mut libc::c_void);
        return 0 as *mut PriorityQ
    }
    (*pq).size = 0i32;
    (*pq).max = size;
    (*pq).initialized = 0i32;
    (*pq).leq = leq;
    return pq;
}
/* really tessPqSortDeletePriorityQ */
#[no_mangle]
pub unsafe extern "C" fn pqDeletePriorityQ(mut alloc: *mut TESSalloc,
                                           mut pq: *mut PriorityQ) {
    if pq.is_null() as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"pqDeletePriorityQ\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                         as *const u8 as *const libc::c_char, 327i32,
                     b"pq != NULL\x00" as *const u8 as *const libc::c_char);
    } else { };
    if !(*pq).heap.is_null() { pqHeapDeletePriorityQ(alloc, (*pq).heap); }
    if !(*pq).order.is_null() {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             (*pq).order as
                                                                 *mut libc::c_void);
    }
    if !(*pq).keys.is_null() {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             (*pq).keys as
                                                                 *mut libc::c_void);
    }
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         pq as
                                                             *mut libc::c_void);
}
/* really tessPqSortInit */
#[no_mangle]
pub unsafe extern "C" fn pqInit(mut alloc: *mut TESSalloc,
                                mut pq: *mut PriorityQ) -> libc::c_int {
    let mut p: *mut *mut PQkey = 0 as *mut *mut PQkey;
    let mut r: *mut *mut PQkey = 0 as *mut *mut PQkey;
    let mut i: *mut *mut PQkey = 0 as *mut *mut PQkey;
    let mut j: *mut *mut PQkey = 0 as *mut *mut PQkey;
    let mut piv: *mut PQkey = 0 as *mut PQkey;
    let mut Stack: [C2RustUnnamed; 50] =
        [C2RustUnnamed{p: 0 as *mut *mut PQkey, r: 0 as *mut *mut PQkey,};
            50];
    let mut top: *mut C2RustUnnamed = Stack.as_mut_ptr();
    let mut seed: libc::c_uint = 2016473283i32 as libc::c_uint;
    /* Create an array of indirect pointers to the keys, so that we
	* the handles we have returned are still valid.
	*/
	/*
	pq->order = (PQkey **)memAlloc( (size_t)
	(pq->size * sizeof(pq->order[0])) );
	*/
    (*pq).order =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              (((*pq).size +
                                                                    1i32) as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut PQkey>()
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                  as
                                                                  libc::c_uint)
            as *mut *mut PQkey;
    /* the previous line is a patch to compensate for the fact that IBM */
	/* machines return a null on a malloc of zero bytes (unlike SGI),   */
	/* so we have to put in this defense to guard against a memory      */
	/* fault four lines down. from fossum@austin.ibm.com.               */
    if (*pq).order.is_null() { return 0i32 }
    p = (*pq).order;
    r = p.offset((*pq).size as isize).offset(-1);
    piv = (*pq).keys;
    i = p;
    while i <= r { *i = piv; piv = piv.offset(1); i = i.offset(1) }
    /* Sort the indirect pointers in descending order,
	* using randomized Quicksort
	*/
    (*top).p = p; /* Undo last swap */
    (*top).r = r;
    top = top.offset(1);
    loop  {
        top = top.offset(-1);
        if !(top >= Stack.as_mut_ptr()) { break ; }
        p = (*top).p;
        r = (*top).r;
        while r > p.offset(10) {
            seed =
                seed.wrapping_mul(1539415821i32 as
                                      libc::c_uint).wrapping_add(1i32 as
                                                                     libc::c_uint);
            i =
                p.offset((seed as libc::c_long %
                              (wrapping_offset_from(r,p) as libc::c_long +
                                   1i32 as libc::c_long)) as isize);
            piv = *i;
            *i = *p;
            *p = piv;
            i = p.offset(-1);
            j = r.offset(1);
            loop  {
                loop  {
                    i = i.offset(1);
                    if (*(**i as *mut TESSvertex)).s <
                           (*(*piv as *mut TESSvertex)).s ||
                           (*(**i as *mut TESSvertex)).s ==
                               (*(*piv as *mut TESSvertex)).s &&
                               (*(**i as *mut TESSvertex)).t <=
                                   (*(*piv as *mut TESSvertex)).t {
                        break ;
                    }
                }
                loop  {
                    j = j.offset(-1);
                    if (*(*piv as *mut TESSvertex)).s <
                           (*(**j as *mut TESSvertex)).s ||
                           (*(*piv as *mut TESSvertex)).s ==
                               (*(**j as *mut TESSvertex)).s &&
                               (*(*piv as *mut TESSvertex)).t <=
                                   (*(**j as *mut TESSvertex)).t {
                        break ;
                    }
                }
                let mut tmp: *mut PQkey = *i;
                *i = *j;
                *j = tmp;
                if !(i < j) { break ; }
            }
            let mut tmp_0: *mut PQkey = *i;
            *i = *j;
            *j = tmp_0;
            if (wrapping_offset_from(i,p) as libc::c_long) <
                   wrapping_offset_from(r,j) as libc::c_long {
                (*top).p = j.offset(1);
                (*top).r = r;
                top = top.offset(1);
                r = i.offset(-1)
            } else {
                (*top).p = p;
                (*top).r = i.offset(-1);
                top = top.offset(1);
                p = j.offset(1)
            }
        }
        /* Insertion sort small lists */
        i = p.offset(1); /* always succeeds */
        while i <= r {
            piv = *i;
            j = i;
            while j > p &&
                      !((*(*piv as *mut TESSvertex)).s <
                            (*(**j.offset(-1) as *mut TESSvertex)).s ||
                            (*(*piv as *mut TESSvertex)).s ==
                                (*(**j.offset(-1) as *mut TESSvertex)).s &&
                                (*(*piv as *mut TESSvertex)).t <=
                                    (*(**j.offset(-1) as *mut TESSvertex)).t)
                  {
                *j = *j.offset(-1);
                j = j.offset(-1)
            }
            *j = piv;
            i = i.offset(1)
        }
    }
    (*pq).max = (*pq).size;
    (*pq).initialized = 1i32;
    pqHeapInit((*pq).heap);
    p = (*pq).order;
    r = p.offset((*pq).size as isize).offset(-1);
    i = p;
    while i < r {
        if !((*(**i.offset(1) as *mut TESSvertex)).s <
                 (*(**i as *mut TESSvertex)).s ||
                 (*(**i.offset(1) as *mut TESSvertex)).s ==
                     (*(**i as *mut TESSvertex)).s &&
                     (*(**i.offset(1) as *mut TESSvertex)).t <=
                         (*(**i as *mut TESSvertex)).t) as libc::c_int as
               libc::c_long != 0 {
            __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                                   &[libc::c_char; 7]>(b"pqInit\x00")).as_ptr(),
                         b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                             as *const u8 as *const libc::c_char, 413i32,
                         b"LEQ( **(i+1), **i )\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        i = i.offset(1)
    }
    return 1i32;
}
/* really tessPqSortInsert */
/* returns INV_HANDLE iff out of memory */
#[no_mangle]
pub unsafe extern "C" fn pqInsert(mut alloc: *mut TESSalloc,
                                  mut pq: *mut PriorityQ, mut keyNew: PQkey)
 -> PQhandle {
    let mut curr: libc::c_int = 0;
    if (*pq).initialized != 0 {
        return pqHeapInsert(alloc, (*pq).heap, keyNew)
    }
    curr = (*pq).size;
    (*pq).size += 1;
    if (*pq).size >= (*pq).max {
        if (*alloc).memrealloc.is_none() {
            return 0xfffffffi32
        } else {
            let mut saveKey: *mut PQkey = (*pq).keys;
            // If the heap overflows, double its size.
            (*pq).max <<= 1i32; // restore ptr to free upon return
            (*pq).keys =
                (*alloc).memrealloc.expect("non-null function pointer")((*alloc).userData,
                                                                        (*pq).keys
                                                                            as
                                                                            *mut libc::c_void,
                                                                        ((*pq).max
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<PQkey>()
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            libc::c_uint)
                    as *mut PQkey;
            if (*pq).keys.is_null() {
                (*pq).keys = saveKey;
                return 0xfffffffi32
            }
        }
    }
    if !(curr != 0xfffffffi32) as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 9],
                                               &[libc::c_char; 9]>(b"pqInsert\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                         as *const u8 as *const libc::c_char, 448i32,
                     b"curr != INV_HANDLE\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    let ref mut fresh4 = *(*pq).keys.offset(curr as isize);
    *fresh4 = keyNew;
    /* Negative handles index the sorted array. */
    return -(curr + 1i32);
}
/* really tessPqSortExtractMin */
#[no_mangle]
pub unsafe extern "C" fn pqExtractMin(mut pq: *mut PriorityQ) -> PQkey {
    let mut sortMin: PQkey = 0 as *mut libc::c_void;
    let mut heapMin: PQkey = 0 as *mut libc::c_void;
    if (*pq).size == 0i32 { return pqHeapExtractMin((*pq).heap) }
    sortMin = **(*pq).order.offset(((*pq).size - 1i32) as isize);
    if !((*(*pq).heap).size == 0i32) {
        heapMin =
            (*(*(*pq).heap).handles.offset((*(*(*pq).heap).nodes.offset(1)).handle
                                               as isize)).key;
        if (*(heapMin as *mut TESSvertex)).s <
               (*(sortMin as *mut TESSvertex)).s ||
               (*(heapMin as *mut TESSvertex)).s ==
                   (*(sortMin as *mut TESSvertex)).s &&
                   (*(heapMin as *mut TESSvertex)).t <=
                       (*(sortMin as *mut TESSvertex)).t {
            return pqHeapExtractMin((*pq).heap)
        }
    }
    loop  {
        (*pq).size -= 1;
        if !((*pq).size > 0i32 &&
                 (**(*pq).order.offset(((*pq).size - 1i32) as
                                           isize)).is_null()) {
            break ;
        }
    }
    return sortMin;
}
/* really tessPqSortMinimum */
#[no_mangle]
pub unsafe extern "C" fn pqMinimum(mut pq: *mut PriorityQ) -> PQkey {
    let mut sortMin: PQkey = 0 as *mut libc::c_void;
    let mut heapMin: PQkey = 0 as *mut libc::c_void;
    if (*pq).size == 0i32 {
        return (*(*(*pq).heap).handles.offset((*(*(*pq).heap).nodes.offset(1)).handle
                                                  as isize)).key
    }
    sortMin = **(*pq).order.offset(((*pq).size - 1i32) as isize);
    if !((*(*pq).heap).size == 0i32) {
        heapMin =
            (*(*(*pq).heap).handles.offset((*(*(*pq).heap).nodes.offset(1)).handle
                                               as isize)).key;
        if (*(heapMin as *mut TESSvertex)).s <
               (*(sortMin as *mut TESSvertex)).s ||
               (*(heapMin as *mut TESSvertex)).s ==
                   (*(sortMin as *mut TESSvertex)).s &&
                   (*(heapMin as *mut TESSvertex)).t <=
                       (*(sortMin as *mut TESSvertex)).t {
            return heapMin
        }
    }
    return sortMin;
}
/* really tessPqSortIsEmpty */
#[no_mangle]
pub unsafe extern "C" fn pqIsEmpty(mut pq: *mut PriorityQ) -> libc::c_int {
    return ((*pq).size == 0i32 && (*(*pq).heap).size == 0i32) as libc::c_int;
}
/* really tessPqSortDelete */
#[no_mangle]
pub unsafe extern "C" fn pqDelete(mut pq: *mut PriorityQ,
                                  mut curr: PQhandle) {
    if curr >= 0i32 { pqHeapDelete((*pq).heap, curr); return }
    curr = -(curr + 1i32);
    if !(curr < (*pq).max && !(*(*pq).keys.offset(curr as isize)).is_null())
           as libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 9],
                                               &[libc::c_char; 9]>(b"pqDelete\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/priorityq.c\x00"
                         as *const u8 as *const libc::c_char, 508i32,
                     b"curr < pq->max && pq->keys[curr] != NULL\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    let ref mut fresh5 = *(*pq).keys.offset(curr as isize);
    *fresh5 = 0 as *mut libc::c_void;
    while (*pq).size > 0i32 &&
              (**(*pq).order.offset(((*pq).size - 1i32) as isize)).is_null() {
        (*pq).size -= 1
    };
}
