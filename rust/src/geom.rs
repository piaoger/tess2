use libc;
extern "C" {
    pub type ActiveRegion;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
}
pub type TESSreal = libc::c_float;
pub type TESSindex = libc::c_int;
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
//#include "tesos.h"
#[no_mangle]
pub unsafe extern "C" fn tesvertLeq(mut u: *mut TESSvertex,
                                    mut v: *mut TESSvertex) -> libc::c_int {
    /* Returns TRUE if u is lexicographically <= v. */
    return ((*u).s < (*v).s || (*u).s == (*v).s && (*u).t <= (*v).t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tesedgeEval(mut u: *mut TESSvertex,
                                     mut v: *mut TESSvertex,
                                     mut w: *mut TESSvertex) -> TESSreal {
    /* Given three vertices u,v,w such that VertLeq(u,v) && VertLeq(v,w),
	* evaluates the t-coord of the edge uw at the s-coord of the vertex v.
	* Returns v->t - (uw)(v->s), ie. the signed distance from uw to v.
	* If uw is vertical (and thus passes thru v), the result is zero.
	*
	* The calculation is extremely accurate and stable, even when v
	* is very close to u or w.  In particular if we set v->t = 0 and
	* let r be the negated result (this evaluates (uw)(v->s)), then
	* r is guaranteed to satisfy MIN(u->t,w->t) <= r <= MAX(u->t,w->t).
	*/
    let mut gapL: TESSreal = 0.;
    let mut gapR: TESSreal = 0.;
    if !(((*u).s < (*v).s || (*u).s == (*v).s && (*u).t <= (*v).t) &&
             ((*v).s < (*w).s || (*v).s == (*w).s && (*v).t <= (*w).t)) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                               &[libc::c_char; 12]>(b"tesedgeEval\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/geom.c\x00"
                         as *const u8 as *const libc::c_char, 59i32,
                     b"VertLeq( u, v ) && VertLeq( v, w )\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    gapL = (*v).s - (*u).s;
    gapR = (*w).s - (*v).s;
    if gapL + gapR > 0i32 as libc::c_float {
        if gapL < gapR {
            return (*v).t - (*u).t +
                       ((*u).t - (*w).t) * (gapL / (gapL + gapR))
        } else {
            return (*v).t - (*w).t +
                       ((*w).t - (*u).t) * (gapR / (gapL + gapR))
        }
    }
    /* vertical line */
    return 0i32 as TESSreal;
}
#[no_mangle]
pub unsafe extern "C" fn tesedgeSign(mut u: *mut TESSvertex,
                                     mut v: *mut TESSvertex,
                                     mut w: *mut TESSvertex) -> TESSreal {
    /* Returns a number whose sign matches EdgeEval(u,v,w) but which
	* is cheaper to evaluate.  Returns > 0, == 0 , or < 0
	* as v is above, on, or below the edge uw.
	*/
    let mut gapL: TESSreal = 0.;
    let mut gapR: TESSreal = 0.;
    if !(((*u).s < (*v).s || (*u).s == (*v).s && (*u).t <= (*v).t) &&
             ((*v).s < (*w).s || (*v).s == (*w).s && (*v).t <= (*w).t)) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                               &[libc::c_char; 12]>(b"tesedgeSign\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/geom.c\x00"
                         as *const u8 as *const libc::c_char, 83i32,
                     b"VertLeq( u, v ) && VertLeq( v, w )\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    gapL = (*v).s - (*u).s;
    gapR = (*w).s - (*v).s;
    if gapL + gapR > 0i32 as libc::c_float {
        return ((*v).t - (*w).t) * gapL + ((*v).t - (*u).t) * gapR
    }
    /* vertical line */
    return 0i32 as TESSreal;
}
/* **********************************************************************
* Define versions of EdgeSign, EdgeEval with s and t transposed.
*/
#[no_mangle]
pub unsafe extern "C" fn testransEval(mut u: *mut TESSvertex,
                                      mut v: *mut TESSvertex,
                                      mut w: *mut TESSvertex) -> TESSreal {
    /* Given three vertices u,v,w such that TransLeq(u,v) && TransLeq(v,w),
	* evaluates the t-coord of the edge uw at the s-coord of the vertex v.
	* Returns v->s - (uw)(v->t), ie. the signed distance from uw to v.
	* If uw is vertical (and thus passes thru v), the result is zero.
	*
	* The calculation is extremely accurate and stable, even when v
	* is very close to u or w.  In particular if we set v->s = 0 and
	* let r be the negated result (this evaluates (uw)(v->t)), then
	* r is guaranteed to satisfy MIN(u->s,w->s) <= r <= MAX(u->s,w->s).
	*/
    let mut gapL: TESSreal = 0.;
    let mut gapR: TESSreal = 0.;
    if !(((*u).t < (*v).t || (*u).t == (*v).t && (*u).s <= (*v).s) &&
             ((*v).t < (*w).t || (*v).t == (*w).t && (*v).s <= (*w).s)) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"testransEval\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/geom.c\x00"
                         as *const u8 as *const libc::c_char, 114i32,
                     b"TransLeq( u, v ) && TransLeq( v, w )\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    gapL = (*v).t - (*u).t;
    gapR = (*w).t - (*v).t;
    if gapL + gapR > 0i32 as libc::c_float {
        if gapL < gapR {
            return (*v).s - (*u).s +
                       ((*u).s - (*w).s) * (gapL / (gapL + gapR))
        } else {
            return (*v).s - (*w).s +
                       ((*w).s - (*u).s) * (gapR / (gapL + gapR))
        }
    }
    /* vertical line */
    return 0i32 as TESSreal;
}
#[no_mangle]
pub unsafe extern "C" fn testransSign(mut u: *mut TESSvertex,
                                      mut v: *mut TESSvertex,
                                      mut w: *mut TESSvertex) -> TESSreal {
    /* Returns a number whose sign matches TransEval(u,v,w) but which
	* is cheaper to evaluate.  Returns > 0, == 0 , or < 0
	* as v is above, on, or below the edge uw.
	*/
    let mut gapL: TESSreal = 0.;
    let mut gapR: TESSreal = 0.;
    if !(((*u).t < (*v).t || (*u).t == (*v).t && (*u).s <= (*v).s) &&
             ((*v).t < (*w).t || (*v).t == (*w).t && (*v).s <= (*w).s)) as
           libc::c_int as libc::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                               &[libc::c_char; 13]>(b"testransSign\x00")).as_ptr(),
                     b"/Users/piaoger/w/repository/github.com/memononen/libtess2/source/geom.c\x00"
                         as *const u8 as *const libc::c_char, 138i32,
                     b"TransLeq( u, v ) && TransLeq( v, w )\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    gapL = (*v).t - (*u).t;
    gapR = (*w).t - (*v).t;
    if gapL + gapR > 0i32 as libc::c_float {
        return ((*v).s - (*w).s) * gapL + ((*v).s - (*u).s) * gapR
    }
    /* vertical line */
    return 0i32 as TESSreal;
}
#[no_mangle]
pub unsafe extern "C" fn tesvertCCW(mut u: *mut TESSvertex,
                                    mut v: *mut TESSvertex,
                                    mut w: *mut TESSvertex) -> libc::c_int {
    /* For almost-degenerate situations, the results are not reliable.
	* Unless the floating-point arithmetic can be performed without
	* rounding errors, *any* implementation will give incorrect results
	* on some degenerate inputs, so the client must have some way to
	* handle this situation.
	*/
    return ((*u).s * ((*v).t - (*w).t) + (*v).s * ((*w).t - (*u).t) +
                (*w).s * ((*u).t - (*v).t) >= 0i32 as libc::c_float) as
               libc::c_int;
}
/* Given parameters a,x,b,y returns the value (b*x+a*y)/(a+b),
* or (x+y)/2 if a==b==0.  It requires that a,b >= 0, and enforces
* this in the rare case that one argument is slightly negative.
* The implementation is extremely stable numerically.
* In particular it guarantees that the result r satisfies
* MIN(x,y) <= r <= MAX(x,y), and the results are very accurate
* even when a and b differ greatly in magnitude.
*/
#[no_mangle]
pub unsafe extern "C" fn tesedgeIntersect(mut o1: *mut TESSvertex,
                                          mut d1: *mut TESSvertex,
                                          mut o2: *mut TESSvertex,
                                          mut d2: *mut TESSvertex,
                                          mut v: *mut TESSvertex) 
 /* Given edges (o1,d1) and (o2,d2), compute their point of intersection.
					  * The computed point is guaranteed to lie in the intersection of the
					  * bounding rectangles defined by each edge.
					  */
 {
    let mut z1: TESSreal = 0.;
    let mut z2: TESSreal = 0.;
    /* This is certainly not the most efficient way to find the intersection
	* of two line segments, but it is very numerically stable.
	*
	* Strategy: find the two middle vertices in the VertLeq ordering,
	* and interpolate the intersection s-value from these.  Then repeat
	* using the TransLeq ordering to find the intersection t-value.
	*/
    if !((*o1).s < (*d1).s || (*o1).s == (*d1).s && (*o1).t <= (*d1).t) {
        let mut t: *mut TESSvertex = o1;
        o1 = d1;
        d1 = t
    }
    if !((*o2).s < (*d2).s || (*o2).s == (*d2).s && (*o2).t <= (*d2).t) {
        let mut t_0: *mut TESSvertex = o2;
        o2 = d2;
        d2 = t_0
    }
    if !((*o1).s < (*o2).s || (*o1).s == (*o2).s && (*o1).t <= (*o2).t) {
        let mut t_1: *mut TESSvertex = o1;
        o1 = o2;
        o2 = t_1;
        let mut t_2: *mut TESSvertex = d1;
        d1 = d2;
        d2 = t_2
    }
    if !((*o2).s < (*d1).s || (*o2).s == (*d1).s && (*o2).t <= (*d1).t) {
        /* Technically, no intersection -- do our best */
        (*v).s = ((*o2).s + (*d1).s) / 2i32 as libc::c_float
    } else if (*d1).s < (*d2).s || (*d1).s == (*d2).s && (*d1).t <= (*d2).t {
        /* Interpolate between o2 and d1 */
        z1 = tesedgeEval(o1, o2, d1);
        z2 = tesedgeEval(o2, d1, d2);
        if z1 + z2 < 0i32 as libc::c_float { z1 = -z1; z2 = -z2 }
        z1 =
            (if z1 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z1 });
        z2 =
            (if z2 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z2 });
        (*v).s =
            (if z1 <= z2 {
                 (if z2 == 0i32 as libc::c_float {
                      ((*o2).s + (*d1).s) / 2i32 as libc::c_float
                  } else { (*o2).s + ((*d1).s - (*o2).s) * (z1 / (z1 + z2)) })
             } else { (*d1).s + ((*o2).s - (*d1).s) * (z2 / (z1 + z2)) })
    } else {
        /* Interpolate between o2 and d2 */
        z1 = tesedgeSign(o1, o2, d1);
        z2 = -tesedgeSign(o1, d2, d1);
        if z1 + z2 < 0i32 as libc::c_float { z1 = -z1; z2 = -z2 }
        z1 =
            (if z1 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z1 });
        z2 =
            (if z2 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z2 });
        (*v).s =
            (if z1 <= z2 {
                 (if z2 == 0i32 as libc::c_float {
                      ((*o2).s + (*d2).s) / 2i32 as libc::c_float
                  } else { (*o2).s + ((*d2).s - (*o2).s) * (z1 / (z1 + z2)) })
             } else { (*d2).s + ((*o2).s - (*d2).s) * (z2 / (z1 + z2)) })
    }
    /* Now repeat the process for t */
    if !((*o1).t < (*d1).t || (*o1).t == (*d1).t && (*o1).s <= (*d1).s) {
        let mut t_3: *mut TESSvertex = o1;
        o1 = d1;
        d1 = t_3
    }
    if !((*o2).t < (*d2).t || (*o2).t == (*d2).t && (*o2).s <= (*d2).s) {
        let mut t_4: *mut TESSvertex = o2;
        o2 = d2;
        d2 = t_4
    }
    if !((*o1).t < (*o2).t || (*o1).t == (*o2).t && (*o1).s <= (*o2).s) {
        let mut t_5: *mut TESSvertex = o1;
        o1 = o2;
        o2 = t_5;
        let mut t_6: *mut TESSvertex = d1;
        d1 = d2;
        d2 = t_6
    }
    if !((*o2).t < (*d1).t || (*o2).t == (*d1).t && (*o2).s <= (*d1).s) {
        /* Technically, no intersection -- do our best */
        (*v).t = ((*o2).t + (*d1).t) / 2i32 as libc::c_float
    } else if (*d1).t < (*d2).t || (*d1).t == (*d2).t && (*d1).s <= (*d2).s {
        /* Interpolate between o2 and d1 */
        z1 = testransEval(o1, o2, d1);
        z2 = testransEval(o2, d1, d2);
        if z1 + z2 < 0i32 as libc::c_float { z1 = -z1; z2 = -z2 }
        z1 =
            (if z1 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z1 });
        z2 =
            (if z2 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z2 });
        (*v).t =
            (if z1 <= z2 {
                 (if z2 == 0i32 as libc::c_float {
                      ((*o2).t + (*d1).t) / 2i32 as libc::c_float
                  } else { (*o2).t + ((*d1).t - (*o2).t) * (z1 / (z1 + z2)) })
             } else { (*d1).t + ((*o2).t - (*d1).t) * (z2 / (z1 + z2)) })
    } else {
        /* Interpolate between o2 and d2 */
        z1 = testransSign(o1, o2, d1);
        z2 = -testransSign(o1, d2, d1);
        if z1 + z2 < 0i32 as libc::c_float { z1 = -z1; z2 = -z2 }
        z1 =
            (if z1 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z1 });
        z2 =
            (if z2 < 0i32 as libc::c_float {
                 0i32 as libc::c_float
             } else { z2 });
        (*v).t =
            (if z1 <= z2 {
                 (if z2 == 0i32 as libc::c_float {
                      ((*o2).t + (*d2).t) / 2i32 as libc::c_float
                  } else { (*o2).t + ((*d2).t - (*o2).t) * (z1 / (z1 + z2)) })
             } else { (*d2).t + ((*o2).t - (*d2).t) * (z2 / (z1 + z2)) })
    };
}
#[no_mangle]
pub unsafe extern "C" fn inCircle(mut v: *mut TESSvertex,
                                  mut v0: *mut TESSvertex,
                                  mut v1: *mut TESSvertex,
                                  mut v2: *mut TESSvertex) -> TESSreal {
    let mut adx: TESSreal = 0.;
    let mut ady: TESSreal = 0.;
    let mut bdx: TESSreal = 0.;
    let mut bdy: TESSreal = 0.;
    let mut cdx: TESSreal = 0.;
    let mut cdy: TESSreal = 0.;
    let mut abdet: TESSreal = 0.;
    let mut bcdet: TESSreal = 0.;
    let mut cadet: TESSreal = 0.;
    let mut alift: TESSreal = 0.;
    let mut blift: TESSreal = 0.;
    let mut clift: TESSreal = 0.;
    adx = (*v0).s - (*v).s;
    ady = (*v0).t - (*v).t;
    bdx = (*v1).s - (*v).s;
    bdy = (*v1).t - (*v).t;
    cdx = (*v2).s - (*v).s;
    cdy = (*v2).t - (*v).t;
    abdet = adx * bdy - bdx * ady;
    bcdet = bdx * cdy - cdx * bdy;
    cadet = cdx * ady - adx * cdy;
    alift = adx * adx + ady * ady;
    blift = bdx * bdx + bdy * bdy;
    clift = cdx * cdx + cdy * cdy;
    return alift * bcdet + blift * cadet + clift * abdet;
}
/*
	Returns 1 is edge is locally delaunay
 */
#[no_mangle]
pub unsafe extern "C" fn tesedgeIsLocallyDelaunay(mut e: *mut TESShalfEdge)
 -> libc::c_int {
    return (inCircle((*(*(*(*e).Sym).Lnext).Lnext).Org, (*(*e).Lnext).Org,
                     (*(*(*e).Lnext).Lnext).Org, (*e).Org) <
                0i32 as libc::c_float) as libc::c_int;
}
