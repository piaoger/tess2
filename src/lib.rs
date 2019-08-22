
use tess2_sys::*;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangles {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

struct Tessellator {
    tess: *mut TESStesselator,
}

enum Orientation {
    Clockwise,
    CounterClockwise,
}

impl Tessellator {
    fn new() -> Self {
        Self { tess: unsafe { tessNewTess(0 as *mut TESSalloc) } }
    }

    fn add_poly(&mut self, poly: &[Vertex], orientation: Orientation) -> Result<(), String> {
        use std::os::raw::c_void;

        if poly.len() < 3 {
            return Err(String::from("A polygon must have at least 3 vertices."));
        }

        let format = |v: &Vertex| vec![v.x, v.y];
        let formatted_vertices: Vec<f32> = match orientation {
            Orientation::Clockwise => poly.iter().flat_map(format).collect(),
            Orientation::CounterClockwise => poly.iter().rev().flat_map(format).collect(),
        };

        unsafe {
            tessAddContour(self.tess,
                           2,
                           (&formatted_vertices[0] as *const f32) as *const c_void,
                           mem::size_of_val(&formatted_vertices[0]) as i32 * 2,
                           poly.len() as i32);
        }

        Ok(())
    }

    fn tessellate(&mut self, rule: TessWindingRule) -> Result<Triangles, String> {
        unsafe {
            use std::slice;
            if tessTesselate(self.tess,
                             rule,
                             TessElementType::TESS_POLYGONS,
                             3,
                             2,
                             0 as *mut TESSreal) != 1 {
                return Err(String::from("Triangulation failed."));
            }

            let raw_triangle_count = tessGetElementCount(self.tess);
            if raw_triangle_count < 1 {
                return Err(String::from("Triangulation failed to yield triangles."));
            };
            let triangle_count = raw_triangle_count as usize;

            let vertex_buffer = slice::from_raw_parts(tessGetVertices(self.tess),
                                                      tessGetVertexCount(self.tess) as usize * 2);
            let triangle_buffer = slice::from_raw_parts(tessGetElements(self.tess),
                                                        triangle_count * 3);

            let xs = vertex_buffer.iter().step_by(2);
            let ys = vertex_buffer.iter().skip(1).step_by(2);
            let verts = xs.zip(ys);

            Ok(Triangles {
                   vertices: verts.map(|(x, y)| Vertex { x: *x, y: *y }).collect(),
                   indices: triangle_buffer.iter().map(|i| *i as u32).collect(),
               })
        }
    }
}

impl Drop for Tessellator {
    fn drop(&mut self) {
        unsafe { tessDeleteTess(self.tess) }
    }
}

/// Tessellates the union of the given simple polygon paths.
pub fn fill_union(polies: &[&[Vertex]]) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    for poly in polies {
        tess.add_poly(poly, Orientation::Clockwise)?;
    }
    tess.tessellate(TessWindingRule::TESS_WINDING_NONZERO)
}

/// Tessellates the intersection of the given simple polygon paths. To tessellate
/// many, call this function again on the resulting triangles; may become expensive.
pub fn fill_intersection(a: &[Vertex], b: &[Vertex]) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    tess.add_poly(a, Orientation::Clockwise)?;
    tess.add_poly(b, Orientation::Clockwise)?;
    tess.tessellate(TessWindingRule::TESS_WINDING_ABS_GEQ_TWO)
}

pub fn fill_difference(base: &[Vertex], subtract: &[&[Vertex]]) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    tess.add_poly(base, Orientation::Clockwise)?;
    for sub in subtract {
        tess.add_poly(sub, Orientation::CounterClockwise)?;
    }
    tess.tessellate(TessWindingRule::TESS_WINDING_POSITIVE)
}

/// Fill tessellate a simple polygon path.
pub fn fill(poly: &[Vertex]) -> Result<Triangles, String> {
    fill_union(&[poly])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fill(&[Vertex { x: 0.0, y: 0.0 },
                          Vertex { x: 1.0, y: 0.0 },
                          Vertex { x: 1.0, y: 1.0 },
                          Vertex { x: 0.0, y: 1.0 }])
                           .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 0.0, y: 1.0 },
                                      Vertex { x: 1.0, y: 0.0 },
                                      Vertex { x: 1.0, y: 1.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 1, 0, 3],
                   });
    }

    #[test]
    fn intersection() {
        assert_eq!(fill_intersection(&[Vertex { x: 0.0, y: 0.0 },
                                       Vertex { x: 1.0, y: 0.0 },
                                       Vertex { x: 1.0, y: 1.0 },
                                       Vertex { x: 0.0, y: 1.0 }],
                                     &[Vertex { x: 0.25, y: 0.25 },
                                       Vertex { x: 0.75, y: 0.25 },
                                       Vertex { x: 0.75, y: 0.75 },
                                       Vertex { x: 0.25, y: 0.75 }])
                           .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 0.25, y: 0.75 },
                                      Vertex { x: 0.75, y: 0.25 },
                                      Vertex { x: 0.75, y: 0.75 },
                                      Vertex { x: 0.25, y: 0.25 }],
                       indices: vec![0, 1, 2, 1, 0, 3],
                   });
    }

    #[test]
    fn union() {
        assert_eq!(fill_union(&[&[Vertex { x: 0.0, y: 0.0 },
                                  Vertex { x: 2.0, y: 4.0 },
                                  Vertex { x: 4.0, y: 0.0 }],
                                &[Vertex { x: 0.5, y: 0.0 },
                                  Vertex { x: 2.0, y: 2.0 },
                                  Vertex { x: 3.5, y: 0.0 }]])
                           .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 2.0, y: 2.0 },
                                      Vertex { x: 4.0, y: 0.0 },
                                      Vertex { x: 3.5, y: 0.0 },
                                      Vertex { x: 2.0, y: 4.0 },
                                      Vertex { x: 0.5, y: 0.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5, 2, 4, 0],
                   });
    }

    #[test]
    fn difference() {
        assert_eq!(fill_difference(&[Vertex { x: 0.0, y: 0.0 },
                                     Vertex { x: 2.0, y: 4.0 },
                                     Vertex { x: 4.0, y: 0.0 }],
                                   &[&[Vertex { x: 0.5, y: 0.0 },
                                       Vertex { x: 2.0, y: 2.0 },
                                       Vertex { x: 3.5, y: 0.0 }]])
                           .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 2.0, y: 2.0 },
                                      Vertex { x: 4.0, y: 0.0 },
                                      Vertex { x: 3.5, y: 0.0 },
                                      Vertex { x: 2.0, y: 4.0 },
                                      Vertex { x: 0.5, y: 0.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5],
                   });
    }
}
