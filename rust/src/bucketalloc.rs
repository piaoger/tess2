use libc;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
pub struct BucketAlloc {
    pub freelist: *mut libc::c_void,
    pub buckets: *mut Bucket,
    pub itemSize: libc::c_uint,
    pub bucketSize: libc::c_uint,
    pub name: *const libc::c_char,
    pub alloc: *mut TESSalloc,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Bucket {
    pub next: *mut Bucket,
}
unsafe extern "C" fn CreateBucket(mut ba: *mut BucketAlloc) -> libc::c_int {
    let mut size: size_t = 0;
    let mut bucket: *mut Bucket = 0 as *mut Bucket;
    let mut freelist: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut head: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut it: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    // Allocate memory for the bucket
    size =
        (::std::mem::size_of::<Bucket>() as
             libc::c_ulong).wrapping_add((*ba).itemSize.wrapping_mul((*ba).bucketSize)
                                             as libc::c_ulong);
    bucket =
        (*(*ba).alloc).memalloc.expect("non-null function pointer")((*(*ba).alloc).userData,
                                                                    size as
                                                                        libc::c_uint)
            as *mut Bucket;
    if bucket.is_null() { return 0i32 }
    (*bucket).next = 0 as *mut Bucket;
    // Add the bucket into the list of buckets.
    (*bucket).next = (*ba).buckets;
    (*ba).buckets = bucket;
    // Add new items to the free list.
    freelist = (*ba).freelist;
    head =
        (bucket as
             *mut libc::c_uchar).offset(::std::mem::size_of::<Bucket>() as
                                            libc::c_ulong as isize);
    it = head.offset((*ba).itemSize.wrapping_mul((*ba).bucketSize) as isize);
    loop  {
        it = it.offset(-((*ba).itemSize as isize));
        // Store pointer to next free item.
        let ref mut fresh0 = *(it as *mut *mut libc::c_void);
        *fresh0 = freelist;
        // Pointer to next location containing a free item.
        freelist = it as *mut libc::c_void;
        if !(it != head) { break ; }
    }
    // Update pointer to next location containing a free item.
    (*ba).freelist = it as *mut libc::c_void;
    return 1i32;
}
unsafe extern "C" fn NextFreeItem(mut ba: *mut BucketAlloc)
 -> *mut libc::c_void {
    return *((*ba).freelist as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn createBucketAlloc(mut alloc: *mut TESSalloc,
                                           mut name: *const libc::c_char,
                                           mut itemSize: libc::c_uint,
                                           mut bucketSize: libc::c_uint)
 -> *mut BucketAlloc {
    let mut ba: *mut BucketAlloc =
        (*alloc).memalloc.expect("non-null function pointer")((*alloc).userData,
                                                              ::std::mem::size_of::<BucketAlloc>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as
                                                                  libc::c_uint)
            as *mut BucketAlloc;
    (*ba).alloc = alloc;
    (*ba).name = name;
    (*ba).itemSize = itemSize;
    if ((*ba).itemSize as libc::c_ulong) <
           ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong {
        (*ba).itemSize =
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as
                libc::c_uint
    }
    (*ba).bucketSize = bucketSize;
    (*ba).freelist = 0 as *mut libc::c_void;
    (*ba).buckets = 0 as *mut Bucket;
    if CreateBucket(ba) == 0 {
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             ba as
                                                                 *mut libc::c_void);
        return 0 as *mut BucketAlloc
    }
    return ba;
}
#[no_mangle]
pub unsafe extern "C" fn bucketAlloc(mut ba: *mut BucketAlloc)
 -> *mut libc::c_void {
    let mut it: *mut libc::c_void = 0 as *mut libc::c_void;
    // If running out of memory, allocate new bucket and update the freelist.
    if (*ba).freelist.is_null() || NextFreeItem(ba).is_null() {
        if CreateBucket(ba) == 0 { return 0 as *mut libc::c_void }
    }
    // Pop item from in front of the free list.
    it = (*ba).freelist;
    (*ba).freelist = NextFreeItem(ba);
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn bucketFree(mut ba: *mut BucketAlloc,
                                    mut ptr: *mut libc::c_void) {
    // Add the node in front of the free list.
    let ref mut fresh1 = *(ptr as *mut *mut libc::c_void);
    *fresh1 = (*ba).freelist;
    (*ba).freelist = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn deleteBucketAlloc(mut ba: *mut BucketAlloc) {
    let mut alloc: *mut TESSalloc = (*ba).alloc;
    let mut bucket: *mut Bucket = (*ba).buckets;
    let mut next: *mut Bucket = 0 as *mut Bucket;
    while !bucket.is_null() {
        next = (*bucket).next;
        (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                             bucket as
                                                                 *mut libc::c_void);
        bucket = next
    }
    (*ba).freelist = 0 as *mut libc::c_void;
    (*ba).buckets = 0 as *mut Bucket;
    (*alloc).memfree.expect("non-null function pointer")((*alloc).userData,
                                                         ba as
                                                             *mut libc::c_void);
}
