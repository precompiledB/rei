use IntersectionResult::{Hit, Miss};
use crate::maths::Vec3;
use crate::ray::Ray;

pub enum IntersectionResult {
    Hit {
        point: Vec3,
        normal: Vec3,
        t: f64,
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
    fn intersect(&self, ray: Ray) -> IntersectionResult;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub position: Vec3,
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}

pub struct Geometry<'a> {
    pub objects: Vec<&'a dyn Intersect>
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> IntersectionResult {
        let a = ray.dir.scalar_mul(ray.dir); // D^2
        let b = 2.0 * ray.dir.scalar_mul(ray.pos - self.position); // 2D(O-C)

        let tmp = (ray.pos - self.position).abs();
        let c = (tmp.scalar_mul(tmp)) - (self.radius * self.radius); // |O-C|^2 - R^2

        let delta = b * b - 4.0 * a * c;

        let t = f64::min((-b + delta.sqrt())/2.0*a, (-b - delta.sqrt())/2.0*a);

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
                    point,
                    normal: (point - self.position).normalize(),
                    t,
                }
            }
            _ => Miss
        }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: Ray) -> IntersectionResult {
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
    }
}

impl<'a> Intersect for Geometry<'a> {
    fn intersect(&self, ray: Ray) -> IntersectionResult {
        let res = self.objects.iter()
            .map(|obj| obj.intersect(ray.clone()))
            .filter_map(|r| match r {
                Hit { point, normal, t } => Some((point, normal, t)),
                Miss => None,
            })
            .filter(|(_p, _n, t)|
                    *t >= ray.min && *t <= ray.max
            )
            .min_by(|a, b| {
                let t_1 = a.2;
                let t_2 = b.2;
                t_1.total_cmp(&t_2)
            });

        match res {
            Some((point, normal, t)) => Hit { point, normal, t },
            None => Miss
        }
    }
}
