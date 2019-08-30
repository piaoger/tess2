//use libc;
extern "C" {
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
    pub type BucketAlloc;
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
pub struct Dict {
    pub head: DictNode,
    pub frame: *mut libc::c_void,
    pub nodePool: *mut BucketAlloc,
    pub leq: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: DictKey,
                                         _: DictKey) -> libc::c_int>,
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
/* really tessDictListNewDict */
#[no_mangle]
pub unsafe extern "C" fn dictNewDict(mut alloc: *mut TESSalloc,
                                     mut frame: *mut libc::c_void,
                                     mut leq:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         DictKey,
                                                                     _:
                                                                         DictKey)
                                                    -> libc::c_int>)
 -> *mut Dict {
    let mut dict: *mut Dict =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<Dict>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut Dict;
    let mut head: *mut DictNode = 0 as *mut DictNode;
    if dict.is_null() { return 0 as *mut Dict }
    head = &mut (*dict).head;
    (*head).key = 0 as *mut libc::c_void;
    (*head).next = head;
    (*head).prev = head;
    (*dict).frame = frame;
    (*dict).leq = leq;
    if (*alloc).dictNodeBucketSize < 16i32 {
        (*alloc).dictNodeBucketSize = 16i32
    }
    if (*alloc).dictNodeBucketSize > 4096i32 {
        (*alloc).dictNodeBucketSize = 4096i32
    }
    (*dict).nodePool =
        createBucketAlloc(alloc,
                          b"Dict\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<DictNode>() as libc::c_ulong
                              as libc::c_uint,
                          (*alloc).dictNodeBucketSize as libc::c_uint);
    return dict;
}
/* really tessDictListDeleteDict */
#[no_mangle]
pub unsafe extern "C" fn dictDeleteDict(mut alloc: *mut TESSalloc,
                                        mut dict: *mut Dict) {
    deleteBucketAlloc((*dict).nodePool);
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         dict as
                                                             *mut libc::c_void);
}
/* really tessDictListInsertBefore */
#[no_mangle]
pub unsafe extern "C" fn dictInsertBefore(mut dict: *mut Dict,
                                          mut node: *mut DictNode,
                                          mut key: DictKey) -> *mut DictNode {
    let mut newNode: *mut DictNode = 0 as *mut DictNode;
    loop  {
        node = (*node).prev;
        if !(!(*node).key.is_null() &&
                 (*dict).leq.expect("non-null function pointer")((*dict).frame,
                                                                 (*node).key,
                                                                 key) == 0) {
            break ;
        }
    }
    newNode = bucketAlloc((*dict).nodePool) as *mut DictNode;
    if newNode.is_null() { return 0 as *mut DictNode }
    (*newNode).key = key;
    (*newNode).next = (*node).next;
    (*(*node).next).prev = newNode;
    (*newNode).prev = node;
    (*node).next = newNode;
    return newNode;
}
/* really tessDictListDelete */
#[no_mangle]
pub unsafe extern "C" fn dictDelete(mut dict: *mut Dict,
                                    mut node: *mut DictNode)
 /*ARGSUSED*/
 {
    (*(*node).next).prev = (*node).prev;
    (*(*node).prev).next = (*node).next;
    bucketFree((*dict).nodePool, node as *mut libc::c_void);
}
/* really tessDictListSearch */
#[no_mangle]
pub unsafe extern "C" fn dictSearch(mut dict: *mut Dict, mut key: DictKey)
 -> *mut DictNode {
    let mut node: *mut DictNode = &mut (*dict).head;
    loop  {
        node = (*node).next;
        if !(!(*node).key.is_null() &&
                 (*dict).leq.expect("non-null function pointer")((*dict).frame,
                                                                 key,
                                                                 (*node).key)
                     == 0) {
            break ;
        }
    }
    return node;
}
