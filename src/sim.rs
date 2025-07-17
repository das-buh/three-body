use crate::vec::Vec2;

const G: f64 = 6.67430e-11;

fn gravity((m1, r1): (f64, Vec2), (m2, r2): (f64, Vec2)) -> Vec2 {
    let r = r2 - r1;
    let f = G * m1 * m2 / r.mag_sq();
    r.norm() * f
}

#[derive(Clone, Copy)]
pub struct Body {
    id: u64,
    pub m: f64,
    pub r: Vec2,
    pub v: Vec2,
    k: Vec2,
    r_aux: Vec2,
    v_acc: Vec2,
}

#[derive(Default)]
pub struct System {
    next_id: u64,
    bodies: Vec<Body>,
}

impl Body {
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl System {
    pub fn add_body(&mut self, m: f64, r: Vec2, v: Vec2) {
        let id = self.next_id;
        self.next_id += 1;

        self.bodies.push(Body {
            id,
            m,
            r,
            v,
            k: Vec2(0., 0.),
            r_aux: Vec2(0., 0.),
            v_acc: Vec2(0., 0.),
        });
    }

    pub fn remove_body(&mut self, id: u64) {
        let idx = self.bodies.iter().position(|a| a.id == id).unwrap();
        self.bodies.swap_remove(idx);
    }

    pub fn body(&self, id: u64) -> Body {
        *self.bodies().find(|a| a.id == id).unwrap()
    }

    pub fn body_mut(&mut self, id: u64) -> &mut Body {
        self.bodies_mut().find(|a| a.id == id).unwrap()
    }

    pub fn bodies(&self) -> impl Iterator<Item = &Body> {
        self.bodies.iter()
    }

    pub fn bodies_mut(&mut self) -> impl Iterator<Item = &mut Body> {
        self.bodies.iter_mut()
    }

    pub fn mass_total(&self) -> f64 {
        self.bodies().map(|b| b.m).sum()
    }

    pub fn center_mass(&self) -> Vec2 {
        let m = self.mass_total();
        let rm = self.bodies().map(|&Body { m, r, .. }| r * m).sum::<Vec2>();
        rm / m
    }

    pub fn energy(&self) -> f64 {
        let m_tot = self.mass_total();
        let cm = self.center_mass();
        self.bodies()
            .map(|&Body { m, r, v, .. }| {
                let k = 0.5 * m * v.mag_sq();
                let u = G * m_tot * m / (r - cm).mag();
                k + u
            })
            .sum()
    }

    pub fn momentum(&self) -> Vec2 {
        self.bodies().map(|&Body { m, v, .. }| v * m).sum()
    }

    pub fn step(&mut self, h: f64) {
        // Increment k1 with weight of 1.
        for a in self.bodies_mut() {
            a.k = a.v;
            a.v_acc = a.k;
        }

        // Increments k2 through k4
        for weight in [2., 2., 1.] {
            let h = h / weight;

            // Predict positions of bodies
            for a in self.bodies_mut() {
                a.r_aux = a.r + a.k * h;
            }

            for i in 0..self.bodies.len() {
                let a = self.bodies[i];
                let f = self.force_net(a.id, a.m, a.r);

                let a = &mut self.bodies[i];
                a.k = a.v + f / a.m * h;

                // Contribute to weighted sum
                a.v_acc += a.k * weight;
            }
        }

        // Derive new state from weighted increments
        for a in self.bodies_mut() {
            a.v = a.v_acc / 6.;
            a.r += a.v * h;
        }
    }

    fn force_net(&self, id: u64, m: f64, r: Vec2) -> Vec2 {
        self.bodies()
            .filter(|a| a.id != id)
            .map(|a| gravity((m, r), (a.m, a.r_aux)))
            .sum()
    }
}
