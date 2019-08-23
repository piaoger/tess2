/// Tessellates the union of the given simple polygon paths.
pub fn fill_union(polies: &[&[crate::math::Vector2]]) -> Result<crate::geom::Mesh2d, String> {
    let mut tess = crate::Tessellator::new();
    for poly in polies {
        tess = tess.add_contour_2d(poly, crate::Orientation::Clockwise);
    }
    tess.tessellate(crate::WindingRule::TESS_WINDING_NONZERO)
}

/// Tessellates the intersection of the given simple polygon paths. To tessellate
/// many, call this function again on the resulting triangles; may become expensive.
pub fn fill_intersection(
    a: &[crate::math::Vector2],
    b: &[crate::math::Vector2],
) -> Result<crate::geom::Mesh2d, String> {
    let mut tess = crate::Tessellator::new();
    tess = tess.add_contour_2d(a, crate::Orientation::Clockwise);
    tess = tess.add_contour_2d(b, crate::Orientation::Clockwise);
    tess.tessellate(crate::WindingRule::TESS_WINDING_ABS_GEQ_TWO)
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
    tess.tessellate(crate::WindingRule::TESS_WINDING_POSITIVE)
}

/// Fill tessellate a simple polygon path.
pub fn fill(poly: &[crate::math::Vector2]) -> Result<crate::geom::Mesh2d, String> {
    fill_union(&[poly])
}
