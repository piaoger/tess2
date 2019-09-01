use tess2_sys::*;

/// Tessellates the union of the given simple polygon paths.
pub fn fill_union(contours: &[&[crate::math::Vector2]]) -> Result<crate::geom::Mesh2d, String> {
    let mut tess = crate::Tessellator::new();
    for contour in contours {
        tess = tess.add_contour_2d(contour, crate::Orientation::Clockwise);
    }
    tess.triangulate_2d(TESS_WINDING_NONZERO)
}

pub fn cleanup_contours(
    contours: &[&[crate::math::Vector2]],
) -> Result<crate::TessellateResult, String> {
    let mut tess = crate::Tessellator::new();
    for contour in contours {
        tess = tess.add_contour_2d(contour, crate::Orientation::Clockwise);
    }
    tess.tessellate_(TESS_WINDING_ODD, TESS_POLYGONS, 3, 2)
}

pub fn cleanup_contours_2(contours: &[&[f32]]) -> Result<crate::TessellateResult, String> {
    let mut tess = crate::Tessellator::new();
    for contour in contours {
        tess = tess.add_contour(contour, crate::Orientation::Clockwise);
    }
    tess.tessellate_(TESS_WINDING_ODD, TESS_POLYGONS, 3, 2)
}

/// Tessellates the intersection of the given simple polygon paths. To triangulate_2d
/// many, call this function again on the resulting triangles; may become expensive.
pub fn fill_intersection(
    a: &[crate::math::Vector2],
    b: &[crate::math::Vector2],
) -> Result<crate::geom::Mesh2d, String> {
    let mut tess = crate::Tessellator::new();
    tess = tess.add_contour_2d(a, crate::Orientation::Clockwise);
    tess = tess.add_contour_2d(b, crate::Orientation::Clockwise);
    tess.triangulate_2d(TESS_WINDING_ABS_GEQ_TWO)
}

pub fn fill_difference(
    base: &[crate::math::Vector2],
    subtract: &[&[crate::math::Vector2]],
) -> Result<crate::geom::Mesh2d, String> {
    let mut tess = crate::Tessellator::new();
    tess = tess.add_contour_2d(base, crate::Orientation::Clockwise);
    for sub in subtract {
        tess = tess.add_contour_2d(sub, crate::Orientation::CounterClockwise);
    }
    tess.triangulate_2d(TESS_WINDING_POSITIVE)
}

/// Fill triangulate_2d a simple polygon path.
pub fn fill(poly: &[crate::math::Vector2]) -> Result<crate::geom::Mesh2d, String> {
    fill_union(&[poly])
}
