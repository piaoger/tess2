use std::mem;
use tess2_sys::*;

pub mod geom;
pub mod math;
pub mod ops;

pub use tess2_sys::TessWindingRule as WindingRule;
pub use tess2_sys::TessElementType as ElementType;
pub use tess2_sys::TessOption as OptionType;

pub enum Orientation {
    Clockwise,
    CounterClockwise,
}

pub struct Tessellator {
    tess: *mut TESStesselator,
}

impl Tessellator {

    pub fn new() -> Self {
        unsafe {
            Tessellator {
                tess: tessNewTess(std::ptr::null_mut()),
            }
        }
    }

    fn set_option(self, opt: TessOption, enable: bool) -> Self {
        unsafe {
            tessSetOption(self.tess, opt, if enable { 1 } else { 0 });
        }
        self
    }


    // TODO: generic support
    pub fn add_contour_2d(self, contour: &[math::Vector2], orientation: Orientation) -> Self {
        type T = math::Vector2;

        // TODO: try to use .into()
        let format = |v: &T| vec![v.x, v.y];

        let formatted_vertices: Vec<f32> = match orientation {
            Orientation::Clockwise => contour.iter().flat_map(format).collect(),
            Orientation::CounterClockwise => contour.iter().rev().flat_map(format).collect(),
        };

        unsafe {
            use std::os::raw::c_void;
            tessAddContour(
                self.tess,
                T::dim(),
                (&formatted_vertices[0] as *const f32) as *const c_void,
                mem::size_of_val(&formatted_vertices[0]) as i32 * T::dim(),
                contour.len() as i32,
            );
        }

        self
    }

    // triangulation
    pub fn tessellate(
        &mut self,
        rule: WindingRule,
        elem_type: TessElementType,
        poly_size: u32,
        vert_size: u32,
    ) -> Result<geom::Mesh2d, String> {
        unsafe {
            use std::slice;

            if tessTesselate(
                self.tess,
                rule,
                elem_type,
                poly_size as i32,
                vert_size as i32,
                0 as *mut TESSreal,
            ) != 1
            {
                return Err(String::from("Triangulation failed."));
            }

            let raw_triangle_count = tessGetElementCount(self.tess);
            if raw_triangle_count < 1 {
                return Err(String::from("Triangulation failed to yield triangles."));
            };

            let triangle_count = raw_triangle_count as usize;
            let stride = vert_size as usize;
            let vertex_buffer = slice::from_raw_parts(
                tessGetVertices(self.tess),
                tessGetVertexCount(self.tess) as usize * stride,
            );
            let triangle_buffer =
                slice::from_raw_parts(tessGetElements(self.tess), triangle_count * poly_size as usize);

            let xs = vertex_buffer.iter().step_by(stride);
            let ys = vertex_buffer.iter().skip(1).step_by(stride);
            let verts = xs.zip(ys);

            // support Mesh3d or Mesh2d in the future
            Ok(geom::Mesh2d {
                vertices: verts.map(|(x, y)| math::Vector2 { x: *x, y: *y }).collect(),
                indices: triangle_buffer.iter().map(|i| *i as u32).collect(),
            })
        }
    }

    pub fn triangulate_2d(&mut self, rule: TessWindingRule) -> Result<geom::Mesh2d, String> {
        let elem_type = TessElementType::TESS_POLYGONS;
        let poly_size = 3;
        let vert_size = 2;

        self.tessellate(rule, elem_type, poly_size, vert_size)
    }
}

impl Drop for Tessellator {
    fn drop(&mut self) {
        unsafe { tessDeleteTess(self.tess) }
    }
}

impl Default for Tessellator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TessellatorBuilder {
    tess: *mut TESStesselator,
}
