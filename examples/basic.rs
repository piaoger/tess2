use tess2::geom::*;
use tess2::math::*;
use tess2::*;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            ops::fill(&[
                Vector2 { x: 0.0, y: 0.0 },
                Vector2 { x: 1.0, y: 0.0 },
                Vector2 { x: 1.0, y: 1.0 },
                Vector2 { x: 0.0, y: 1.0 }
            ])
            .expect("triangulation"),
            Mesh2d {
                vertices: vec![
                    Vector2 { x: 0.0, y: 1.0 },
                    Vector2 { x: 1.0, y: 0.0 },
                    Vector2 { x: 1.0, y: 1.0 },
                    Vector2 { x: 0.0, y: 0.0 }
                ],
                indices: vec![0, 1, 2, 1, 0, 3],
            }
        );
    }

    #[test]
    fn intersection() {
        assert_eq!(
            ops::fill_intersection(
                &[
                    Vector2 { x: 0.0, y: 0.0 },
                    Vector2 { x: 1.0, y: 0.0 },
                    Vector2 { x: 1.0, y: 1.0 },
                    Vector2 { x: 0.0, y: 1.0 }
                ],
                &[
                    Vector2 { x: 0.25, y: 0.25 },
                    Vector2 { x: 0.75, y: 0.25 },
                    Vector2 { x: 0.75, y: 0.75 },
                    Vector2 { x: 0.25, y: 0.75 }
                ]
            )
            .expect("triangulation"),
            Mesh2d {
                vertices: vec![
                    Vector2 { x: 0.25, y: 0.75 },
                    Vector2 { x: 0.75, y: 0.25 },
                    Vector2 { x: 0.75, y: 0.75 },
                    Vector2 { x: 0.25, y: 0.25 }
                ],
                indices: vec![0, 1, 2, 1, 0, 3],
            }
        );
    }

    #[test]
    fn union() {
        assert_eq!(
            ops::fill_union(&[
                &[
                    Vector2 { x: 0.0, y: 0.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                    Vector2 { x: 4.0, y: 0.0 }
                ],
                &[
                    Vector2 { x: 0.5, y: 0.0 },
                    Vector2 { x: 2.0, y: 2.0 },
                    Vector2 { x: 3.5, y: 0.0 }
                ]
            ])
            .expect("triangulation"),
            Mesh2d {
                vertices: vec![
                    Vector2 { x: 2.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 0.0 },
                    Vector2 { x: 3.5, y: 0.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                    Vector2 { x: 0.5, y: 0.0 },
                    Vector2 { x: 0.0, y: 0.0 }
                ],
                indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5, 2, 4, 0],
            }
        );
    }

    #[test]
    fn difference() {
        assert_eq!(
            ops::fill_difference(
                &[
                    Vector2 { x: 0.0, y: 0.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                    Vector2 { x: 4.0, y: 0.0 }
                ],
                &[&[
                    Vector2 { x: 0.5, y: 0.0 },
                    Vector2 { x: 2.0, y: 2.0 },
                    Vector2 { x: 3.5, y: 0.0 }
                ]]
            )
            .expect("triangulation"),
            Mesh2d {
                vertices: vec![
                    Vector2 { x: 2.0, y: 2.0 },
                    Vector2 { x: 4.0, y: 0.0 },
                    Vector2 { x: 3.5, y: 0.0 },
                    Vector2 { x: 2.0, y: 4.0 },
                    Vector2 { x: 0.5, y: 0.0 },
                    Vector2 { x: 0.0, y: 0.0 }
                ],
                indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5],
            }
        );
    }
}
