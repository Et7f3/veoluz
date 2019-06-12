use crate::line;
use serde::{Deserialize, Serialize};
use crate::parabola::{Parabola, ray_parabola_collision};

use ncollide2d::query::Ray;
use ncollide2d::shape::Ball;

use ncollide2d::shape::Segment;
use ncollide2d::query::RayIntersection;
use ncollide2d::shape::FeatureId;
use std::f32::consts::PI;

use nalgebra::{Point2, Vector2};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum WallType {
    Line(Segment<line::float>),
    Circle(
        ncollide2d::shape::Ball<line::float>,
        Point2<line::float>,
        line::float,
        line::float,
    ),
    Parabola(Parabola),
}

impl WallType {
    pub fn rand_circle(width: usize, height: usize) -> WallType {
        WallType::Circle(
            Ball::new(rand::random::<f32>() * 50.0 + 50.0),
            Point2::new(
                rand::random::<f32>() * width as f32,
                rand::random::<f32>() * height as f32,
            ),
            -PI,
            PI,
        )
    }

    pub fn rand_line(width: usize, height: usize) -> WallType {
        let c = Point2::new(
            rand::random::<f32>() * (width as f32 - 200.0) + 100.0,
            rand::random::<f32>() * (height as f32 - 200.0) + 100.0,
        );
        let r = rand::random::<f32>() * std::f32::consts::PI;
        let len = rand::random::<f32>() * 70.0 + 30.0;
        let off = Vector2::new(r.cos() * len, r.sin() * len);
        WallType::Line(Segment::new(c + off, c - off))
    }

    pub fn rand_parabola(width: usize, height: usize) -> WallType {
        let c = Vector2::new(
            rand::random::<f32>() * (width as f32 - 200.0) + 100.0,
            rand::random::<f32>() * (height as f32 - 200.0) + 100.0,
        );
        let r = rand::random::<f32>() * std::f32::consts::PI;
        let a = rand::random::<f32>() * 100.0 + 5.0;
        WallType::Parabola(Parabola {
            a: 1.0 / (4.0 * a),
            left: -(rand::random::<f32>() * 50.0 + 10.0),
            right: (rand::random::<f32>() * 50.0 + 10.0),
            transform: nalgebra::Isometry2::from_parts(
                nalgebra::Translation2::from(c),
                nalgebra::UnitComplex::from_angle(r),
            ),
        })
    }

    pub fn rand_all(width: usize, height: usize) -> Vec<WallType> {
        vec![
            WallType::rand_circle(width, height),
            WallType::rand_line(width, height),
            WallType::rand_parabola(width, height),
        ]
    }

    pub fn scale(&mut self, by: usize) {
        match self {
            WallType::Line(wall) => {
                *wall = Segment::new(
                    wall.a() * by as f32,
                    wall.b() * by as f32,
                )
            },
            WallType::Circle(ball, center, _, _) => {
                *ball = Ball::new(ball.radius() * by as f32);
                *center = *center * by as f32;
            },
            WallType::Parabola(parabola) => {
                parabola.transform.translation.vector *= by as f32;
            }

        }
    }

    pub fn rotate_around(&mut self, center: &Point2<line::float>, angle: line::float) {
        let base = self.point_base();
        let diff = base - center;
        let current_angle = diff.y.atan2(diff.x);
        let dist = diff.norm_squared().sqrt();
        let new_angle = current_angle + angle;
        let new_base = Point2::new(center.x + new_angle.cos() * dist, center.y + new_angle.sin() * dist);
        match self {
            WallType::Line(wall) =>  {
                let diff = wall.b() - center;
                let current_angle = diff.y.atan2(diff.x);
                let dist = diff.norm_squared().sqrt();
                let new_angle = current_angle + angle;
                let new_b = Point2::new(center.x + new_angle.cos() * dist, center.y + new_angle.sin() * dist);

                *wall = Segment::new(
                    new_base,
                    new_b
                );
            },
            WallType::Circle(ball, center, t0, t1) => {
                *center = new_base;
                *t0 += angle;
                *t1 += angle;
            },
            WallType::Parabola(parabola) => {
                parabola.transform.translation = nalgebra::Translation2::from(new_base.coords);
                parabola.transform.rotation = nalgebra::UnitComplex::from_angle(parabola.transform.rotation.angle() + angle);
            }
        }
    }

    pub fn point_dist(&self, point: &Point2<line::float>) -> line::float {
      match self {
        WallType::Line(wall) => {
          // y = mx + b
          // m = wall.dy / wall.dx
          // b = wall.y0 - m * wall.x0
          // 
          // y = nx + c
          //
          // mx + b = nx + c
          // x = (c - b) / (m - n)
          let wd = wall.b() - wall.a();
          if wd.x == 0.0 {
            return std::f32::INFINITY;
          }
          let m = wd.y / wd.x;
          let b = wall.b().y - m * wall.b().x;
          let n = 1.0 / m;
          let c = point.y - n * point.x;
          let x = (c - b) / (m - n);
          let y = m * x + b;
          let x0 = wall.a().x.min(wall.b().x);
          let x1 = wall.a().x.max(wall.b().x);
          if x0 <= x && x <= x1 {
            (point - Point2::new(x, y)).norm_squared().sqrt()
          } else {
            std::f32::INFINITY
          }
        },
        WallType::Circle(circle, center, t0, t1) => crate::arc::point_dist(point, center, circle.radius(), *t0, *t1),
        WallType::Parabola(parabola) => crate::parabola::point_dist(point, parabola)
      }
    }

    pub fn point_base(&self) -> Point2<line::float> {
      match self {
        WallType::Line(wall) => wall.a().clone(),
        WallType::Circle(_, center, _, _) => center.clone(),
        WallType::Parabola(parabola) => parabola.transform.translation.vector.into()
      }
    }

    pub fn set_point_base(&mut self, point: Point2<line::float>) {
      match self {
        WallType::Line(wall) => {*wall = Segment::new(point, point + (wall.b() - wall.a()))},
        WallType::Circle(_, center, _, _) => {*center = point},
        WallType::Parabola(parabola) => {
          parabola.transform.translation = nalgebra::Translation2::from(point.coords);
        }
      }
    }

    pub fn toi_and_normal_with_ray(
        &self,
        ray: &Ray<line::float>,
    ) -> Option<ncollide2d::query::RayIntersection<line::float>> {
        use ncollide2d::query::ray_internal::ray::RayCast;
        match self {
            WallType::Line(wall) => {
                match wall.toi_and_normal_with_ray(
                    &nalgebra::geometry::Isometry::identity(),
                    ray,
                    true,
                ) {
                    None => None,
                    Some(mut intersection) => {
                        let delta = wall.b() - wall.a();
                        let wall_theta = delta.y.atan2(delta.x);
                        let normal_theta = intersection.normal.y.atan2(intersection.normal.x);
                        let left_side = if wall_theta >= 0.0 {
                            normal_theta < wall_theta && normal_theta > wall_theta - PI
                        } else {
                            normal_theta < wall_theta || normal_theta > wall_theta + PI
                        };
                        intersection.feature =
                            ncollide2d::shape::FeatureId::Face(if left_side { 0 } else { 1 });
                        Some(intersection)
                    }
                }
            }
            WallType::Circle(circle, center, t0, t1) => {
                crate::arc::ray_arc_collision(&ray, (circle, center, *t0, *t1))
            }
            WallType::Parabola(parabola) => ray_parabola_collision(&ray, &parabola),
        }
    }

    pub fn check_handle(&self, pos: &Point2<line::float>, size: line::float) -> Option<usize> {
        let dist = size * size;
        for (i, handle) in self.handles().iter().enumerate() {
            if (handle - pos).norm_squared() < dist {
                return Some(i);
            }
        }
        None
    }

    pub fn move_handle(&mut self, id: usize, pos: &Point2<line::float>) {
        match self {
            WallType::Line(wall) => match id {
                0 => *wall = Segment::new(*pos, wall.b().clone()),
                1 => *wall = Segment::new(wall.a().clone(), *pos),
                _ => (),
            },
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => match id {
                // 0 => transform.translation = nalgebra::Translation2::from(pos.coords),
                0 => {
                    let dist = transform.translation.vector - pos.coords;
                    *a = -1.0 / (4.0 * dist.norm_squared().sqrt());
                    transform.rotation = nalgebra::UnitComplex::from_angle(
                        dist.y.atan2(dist.x) - std::f32::consts::PI / 2.0,
                    );
                }
                1 => {
                    let pos = transform.inverse_transform_point(pos);
                    *left = pos.x;
                    if *right < *left {
                        *right = *left + 10.0;
                    }
                }
                2 => {
                    let pos = transform.inverse_transform_point(pos);
                    *right = pos.x;
                    if *left > *right {
                        *left = *right - 10.0;
                    }
                }
                _ => (),
            },
            WallType::Circle(circle, center, t0, t1) => match id {
                // 0 => *center = *pos,
                0 => {
                    let d = pos - *center;
                    *t0 = d.y.atan2(d.x);
                    *circle = Ball::new(d.norm_squared().sqrt());
                }
                1 => {
                    let d = pos - *center;
                    *t1 = d.y.atan2(d.x);
                    *circle = Ball::new(d.norm_squared().sqrt());
                }
                _ => (),
            },
        }
    }

    pub fn handles(&self) -> Vec<Point2<line::float>> {
        match self {
            WallType::Line(wall) => vec![wall.a().clone(), wall.b().clone()],
            WallType::Parabola(Parabola {
                a,
                left,
                right,
                transform,
            }) => vec![
                // transform.translation.vector.into(),
                Point2::from(transform.translation.vector)
                    + transform
                        .rotation
                        .transform_vector(&Vector2::new(0.0, 1.0 / (*a * 4.0))),
                transform.transform_point(&Point2::new(*left, 0.0)),
                transform.transform_point(&Point2::new(*right, 0.0)),
            ], // TODO left & right
            WallType::Circle(circle, center, t0, t1) => vec![
                // center.clone(),
                Point2::new(
                    center.x + t0.cos() * circle.radius(),
                    center.y + t0.sin() * circle.radius(),
                ),
                Point2::new(
                    center.x + t1.cos() * circle.radius(),
                    center.y + t1.sin() * circle.radius(),
                ),
            ],
        }
    }
}