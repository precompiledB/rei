use crate::{maths::Vec3, light_transport::PBRMaterial};
use crate::ray::Ray;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};
use IntersectionResult::{Hit, Miss};

pub enum IntersectionResult {
    Hit {
        idx: usize, // Which object is hit?
        point: Vec3,
        normal: Vec3,
        t: f64,
        color: PBRMaterial,
    },
    Miss,
}

/*
#[derive(Clone)]
pub struct IntersectionCtx {
    //t_min: f64,
    //t_max: f64, TODO: Add
    pub hit_record: Vec<Box<dyn Intersect>>
}
*/

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> IntersectionResult;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub position: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub pbr_mat: PBRMaterial,
}

pub struct Geometry<'a> {
    pub objects: Vec<&'a dyn Intersect>,
}

pub struct TriGeometry {
    pub objects: Vec<Triangle>,
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> IntersectionResult {
        let a = ray.dir.dotp(ray.dir); // D^2
        let b = 2.0 * ray.dir.dotp(ray.pos - self.position); // 2D(O-C)

        let tmp = (ray.pos - self.position).abs();
        let c = (tmp.dotp(tmp)) - (self.radius * self.radius); // |O-C|^2 - R^2

        let delta = b * b - 4.0 * a * c;

        let t = f64::min((-b + delta.sqrt()) / 2.0 * a, (-b - delta.sqrt()) / 2.0 * a);

        const THRESHOLD: f64 = 0.03;
        match delta {
            // TODO: re-add functionality for edge detection
            /*x if x > -THRESHOLD && x < THRESHOLD && t > 0. => Hit(
                [156, 255, 120]
            ), // hit in one point;  green
            x if x > THRESHOLD && t > 0. => {
                let n = (ray.at(t) - self.position).normalize();
                let n = 255. * (0.5 * (Vec3([1.,1.,1.]) + n));
                Hit(n.0.map(|x| x as u8))
            }, // intersect in two point; normal colouring
            x if x < -THRESHOLD => Miss, // hit in no points*/
            x if x > 0. && t > 0.0 => {
                let point = ray.at(t);
                Hit {
                    idx: 0,
                    point,
                    normal: -(point - self.position).normalize(),
                    t,
                    color: [0xd3, 0x68, 0x7d].into(),
                }
            }
            _ => Miss,
        }
    }
}

impl Intersect for Triangle {
    /*fn intersect(&self, ray: Ray) -> IntersectionResult {




        let plane_normal = {
            let a = self.vertices[1] - self.vertices[0];
            let b = self.vertices[2] - self.vertices[0];
            let c = a.cross(b);
            c.normalize()
        };

        let distance = plane_normal.scalar_mul(self.vertices[0]);

        let t = - ((plane_normal.scalar_mul(ray.pos) + distance) /
            plane_normal.scalar_mul(ray.dir));

        let p = ray.pos + (ray.dir * t);


        let edges = [
            self.vertices[1] - self.vertices[0],
            self.vertices[2] - self.vertices[1],
            self.vertices[0] - self.vertices[2]
        ];

        let c = [
            p - self.vertices[0],
            p - self.vertices[1],
            p - self.vertices[2]
        ];

        let q = [
            edges[0].cross(c[0]),
            edges[1].cross(c[1]),
            edges[2].cross(c[2])
        ].map(|x| plane_normal.scalar_mul(x));

        if q.iter().all(|x: &f64| x > &0.0) {
            Hit {
                t,
                point: ray.at(t),
                normal: plane_normal
            }
        } else {
            Miss
        }
    }*/

    // https://graphicscodex.com/Sample2-RayTriangleIntersection.pdf
    fn intersect(&self, ray: &Ray) -> IntersectionResult {
        let eps = 1e-4;

        // edge vectors
        let e_1 = self.vertices[1] - self.vertices[0];
        let e_2 = self.vertices[2] - self.vertices[0];

        // face normal
        let n = e_1.cross(e_2).normalize();
        let q = ray.dir.cross(e_2);
        let a = e_1.dotp(q);

        // Backfacing or nearly parallel?
        if
        /*(n.scalar_mul(ray.dir) >= 0.) ||*/
        (a.abs() <= 1e-10) {
            //print!("█");
            return Miss;
        }

        // Barycentric coordinates
        let s = (ray.pos - self.vertices[0]) * (1. / a);
        let r = s.cross(e_1);

        let mut b = [0.; 3];
        b[0] = s.dotp(q);
        b[1] = r.dotp(ray.dir);
        b[2] = 1.0 - b[0] - b[1];

        // Intersected outside triangle?
        if b.iter().any(|x| *x < 0. || *x > 1.) {
            //print!("░");
            return Miss;
        }
        let t = e_2.dotp(r);
        match t >= 0. {
            // Hit
            true => Hit {
                idx: 0,
                point: ray.at(t),
                normal: n,
                t,
                color: self.pbr_mat,
            },
            // Miss
            false => Miss,
        }
    }
}
/* If ray P + tw hits triangle V[0] , V[1] , V[2] , then the
function returns true, stores the barycentric coordinates in
b[] , and stores the distance to the intersection in t .
Otherwise returns false and the other output parameters are
undefined.*/

/* impl<'a> Intersect for Geometry<'a> {
    fn intersect(&self, ray: &Ray) -> IntersectionResult {
        let res = self
            .objects
            .iter()
            .map(|obj| obj.intersect(&ray))
            .filter_map(|r| match r {
                Hit {
                    point,
                    normal,
                    t,
                    color,
                } => Some((point, normal, t, color)),
                Miss => None,
            })
            .filter(|(_p, _n, t, color)| *t >= ray.min && *t <= ray.max)
            .min_by(|a, b| {
                let t_1 = a.2;
                let t_2 = b.2;
                t_1.total_cmp(&t_2)
            });

        match res {
            Some((point, normal, t, color)) => Hit {
                point,
                normal,
                t,
                color,
            },
            None => Miss,
        }
    }
} */

impl Intersect for TriGeometry {
    fn intersect(&self, ray: &Ray) -> IntersectionResult {
        let res = self
            .objects
            .par_iter()
            .enumerate()
            .map(|(idx, obj)| (idx, obj.intersect(ray)))
            .filter_map(|(obj_idx, r)| match r {
                Hit {
                    idx: _, // this is always 0, cuz one obj is scanned; ignore this idx then 
                    point,
                    normal,
                    t,
                    color,
                } => Some((obj_idx, point, normal, t, color)),
                Miss => None,
            })
            .filter(|(_idx, _p, _n, t, _color)| *t >= ray.min && *t <= ray.max)
            .min_by(|a, b| {
                let t_1 = a.3;
                let t_2 = b.3;
                t_1.total_cmp(&t_2)
            });

        if let Some(_) = res {
            //print!("#");
        }

        match res {
            Some((idx, point, normal, t, color)) => Hit {
                idx,
                point,
                normal,
                t,
                color,
            },
            None => Miss,
        }
    }
}
