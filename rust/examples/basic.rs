use tess2x::*;

fn main() {
    let mut data: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];

    unsafe {
        let nvp = 3usize;
        let fpv = 2;
        let bpv = 2 * std::mem::size_of::<f32>();
        let mut tess = tessNewTess(0 as *mut _);

        tessAddContour(
            tess,
            2,
            data.as_ptr() as _,
            2 * std::mem::size_of::<f32>() as i32,
            (data.len() / 2) as _,
        );

        println!(
            "STATUS:{:?}",
            tessTesselate(
                tess,
                TESS_WINDING_POSITIVE as i32,
                TESS_POLYGONS as i32,
                nvp as i32,
                2,
                0 as *const _
            )
        );

        println!(
            "Elems:{:?} {:?}",
            tessGetElements(tess),
            tessGetElementCount(tess)
        );
        println!(
            "Elems:{:?} {:?}",
            tessGetVertices(tess),
            tessGetVertexCount(tess)
        );
        println!(
            "Elems:{:?} {:?}",
            tessGetVertexIndices(tess),
            tessGetVertexCount(tess)
        );
        println!(
            "Elems:{:?}",
            std::slice::from_raw_parts(
                tessGetElements(tess),
                ((tessGetElementCount(tess) as usize) * nvp)
            )
        );
        println!(
            "Verts:{:?}",
            std::slice::from_raw_parts(
                tessGetVertices(tess) as *const f32,
                tessGetVertexCount(tess) as usize * fpv
            )
        );
        println!(
            "Indis:{:?}",
            std::slice::from_raw_parts(tessGetVertexIndices(tess), tessGetVertexCount(tess) as _)
        );
    }
}
