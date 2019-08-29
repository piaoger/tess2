use std::mem;
use tess2_sys::*;

pub mod geom;
pub mod math;
pub mod ops;

pub use tess2_sys::TessElementType as ElementType;
pub use tess2_sys::TessOption as OptionType;
pub use tess2_sys::TessWindingRule as WindingRule;

pub enum Orientation {
    Clockwise,
    CounterClockwise,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct TessellateResult {
    /// generated vertex buffer
    pub vertices: Vec<f32>,

    /// vertx index mapped to origin one
    pub vertex_indices: Vec<isize>,

    /// elements: polygons, connected polygons, boundary
    pub elements: Vec<isize>,

    pub element_count: usize,
}


impl Default for TessellateResult {
    fn default() -> Self {
        TessellateResult{
            vertices:vec![],
            vertex_indices:vec![],
            elements:vec![],
        }
    }
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

    // TODO: generic support
    pub fn add_contour(self, contour: &[f32], orientation: Orientation) -> Self {
        type T = math::Vector2;

        // TODO: try to use .into()
        let format = |v: &T| vec![v.x, v.y];

        unsafe {
            use std::os::raw::c_void;
            tessAddContour(
                self.tess,
                T::dim(),
                (&contour[0] as *const f32) as *const c_void,
                mem::size_of_val(&contour[0]) as i32 * T::dim(),
                contour.len() as i32 / 2,
            );
        }

        self
    }

    // triangulation
    pub fn tessellate_(
        &mut self,
        rule: WindingRule,
        elem_type: TessElementType,
        poly_size: u32,
        vert_size: u32,
    ) -> Result<TessellateResult, String> {
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
                return Err(String::from("Tessellate failed."));
            }

            // element
            let raw_element_count = tessGetElementCount(self.tess);
            if raw_element_count < 1 {
                return Err(String::from("Tessellate failed to yield elements."));
            };

            let element_count = raw_element_count as usize;
            let raw_elements = tessGetElements(self.tess);
            let elem_buf_len = match elem_type {
                TessElementType::TESS_POLYGONS => element_count * poly_size as usize,
                TessElementType::TESS_CONNECTED_POLYGONS => element_count * poly_size as usize * 2,
                TessElementType::TESS_BOUNDARY_CONTOURS => element_count * 2,
            };

            println!("elments array len{:?}", elem_buf_len);
            let element_buffer = slice::from_raw_parts(raw_elements, elem_buf_len);

            // vertex
            let vert_count = tessGetVertexCount(self.tess) as usize;
            let vert_stride = vert_size as usize;
            let vertex_buffer =
                slice::from_raw_parts(tessGetVertices(self.tess), vert_count * vert_stride);
            let vert_indices_buffer =
                slice::from_raw_parts(tessGetVertexIndices(self.tess), vert_count);

            // support Mesh3d or Mesh2d in the future
            Ok(TessellateResult {
                vertices: vertex_buffer.to_vec(), //.iter().map(|i| *i).collect(),
                vertex_indices: vert_indices_buffer.iter().map(|i| *i as isize).collect(),
                elements: element_buffer.iter().map(|i| *i as isize).collect(),
                element_count:element_count,
            })
        }
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
                return Err(String::from("Tessellate failed."));
            }

            let raw_element_count = tessGetElementCount(self.tess);
            if raw_element_count < 1 {
                return Err(String::from("Tessellate failed to yield elements."));
            };

            let vert_count = tessGetVertexCount(self.tess) as usize;
            let vert_stride = vert_size as usize;
            let vertex_buffer =
                slice::from_raw_parts(tessGetVertices(self.tess), vert_count * vert_stride);

            let element_count = raw_element_count as usize;
            let elem_buf_len = match elem_type {
                TessElementType::TESS_POLYGONS => element_count * poly_size as usize,
                TessElementType::TESS_CONNECTED_POLYGONS => element_count * poly_size as usize * 2,
                TessElementType::TESS_BOUNDARY_CONTOURS => element_count * poly_size as usize * 2,
            };

            let element_buffer = slice::from_raw_parts(tessGetElements(self.tess), elem_buf_len);

            let xs = vertex_buffer.iter().step_by(vert_stride);
            let ys = vertex_buffer.iter().skip(1).step_by(vert_stride);
            let verts = xs.zip(ys);

            // support Mesh3d or Mesh2d in the future
            Ok(geom::Mesh2d {
                vertices: verts.map(|(x, y)| math::Vector2 { x: *x, y: *y }).collect(),
                // vertex_indices:
                indices: element_buffer.iter().map(|i| *i as u32).collect(),
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
