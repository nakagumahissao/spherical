use std::f64::consts::PI;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Polar {
    r: f64,
    p: f64,
    theta: f64,
    phi: f64,
    theta_deg: f64,
    phi_deg: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangular {
    x: f64,
    y: f64,
    z: f64,
}

/* ---------- Polar ---------- */

impl Polar {
    pub fn new(r: f64, theta: f64, phi: f64) -> Self {
        let p = r / phi.sin();

        Self {
            r,
            p,
            theta,
            phi,
            theta_deg: to_deg(theta),
            phi_deg: to_deg(phi),
        }
    }

    pub fn to_rectangular(&self) -> Rectangular {
        Rectangular {
            x: self.p * self.phi.sin() * self.theta.cos(),
            y: self.p * self.phi.sin() * self.theta.sin(),
            z: self.p * self.phi.cos(),
        }
    }
}

impl fmt::Display for Polar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spherical Coordinates:\n\n\
             r = {}\n\
             p = {}\n\
             theta = {} rad ({}°)\n\
             phi   = {} rad ({}°)",
            self.r,
            self.p,
            self.theta,
            self.theta_deg,
            self.phi,
            self.phi_deg
        )
    }
}

/* ---------- Rectangular ---------- */

impl Rectangular {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn to_spherical(&self) -> Polar {
        let r = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let theta = self.y.atan2(self.x);
        let phi = (self.z / r).acos();

        Polar::new(r, theta, phi)
    }
}

impl fmt::Display for Rectangular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rectangular Coordinates:\n\n\
             x = {}\n\
             y = {}\n\
             z = {}",
            self.x, self.y, self.z
        )
    }
}

/* ---------- helpers ---------- */

fn to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}
