use crate::vec::Vec2;

fn gravity((m1, r1): (f64, Vec2), (m2, r2): (f64, Vec2)) -> Vec2 {
    const G: f64 = 6.67430e-11;

    let r = r2 - r1;
    let f = G * m1 * m2 / r.mag_sq();
    r.norm() * f
}

pub struct System {
    pub ms: Vec<f64>,
    pub ps: Vec<Vec2>,
    pub vs: Vec<Vec2>,
}

impl System {
    pub fn add_body(&mut self, m: f64, p: Vec2, v: Vec2) {
        self.ms.push(m);
        self.ps.push(p);
        self.vs.push(v);
    }

    pub fn remove_body(&mut self, idx: usize) {
        self.ms.swap_remove(idx);
        self.ps.swap_remove(idx);
        self.vs.swap_remove(idx);
    }

    pub fn bodies(&self) -> impl Iterator<Item = (usize, f64, Vec2, Vec2)> {
        let n = self.ms.len();
        let bodies = (0..n).zip(&self.ms).zip(&self.ps).zip(&self.vs);
        bodies.map(|(((i, &m), &p), &v)| (i, m, p, v))
    }

    pub fn step(&mut self, h: f64) {
        let System { ms, ps, vs } = self;

        let k1 = vs;
        let k2 = increment(ms, ps, k1, h / 2.);
        let k3 = increment(ms, ps, &k2, h / 2.);
        let k4 = increment(ms, ps, &k3, h);

        let increments = k1.iter().zip(k2).zip(k3).zip(k4);
        let vs = increments
            .map(|(((&k1, k2), k3), k4)| (k1 + k2 * 2. + k3 * 2. + k4) / 6.)
            .collect::<Vec<_>>();

        let ps = apply_velos(ps, &vs, h);

        self.ps = ps;
        self.vs = vs;
    }
}

fn increment(ms: &[f64], ps: &[Vec2], vs: &[Vec2], h: f64) -> Vec<Vec2> {
    let ps = apply_velos(ps, vs, h);
    let system = ms.iter().zip(&ps).zip(vs);
    system
        .map(|((&m, &p), &v)| v + sum_forces(m, p, ms, &ps) / m * h)
        .collect()
}

fn apply_velos(ps: &[Vec2], vs: &[Vec2], h: f64) -> Vec<Vec2> {
    ps.iter().zip(vs).map(|(&p, &v)| p + v * h).collect()
}

fn sum_forces(m: f64, p: Vec2, ms: &[f64], ps: &[Vec2]) -> Vec2 {
    let others = ms.iter().zip(ps).filter(|(_, po)| **po != p);
    others.map(|(&mo, &po)| gravity((m, p), (mo, po))).sum()
}
