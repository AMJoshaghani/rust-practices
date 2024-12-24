use crate::vector::Vector3D;

const PI: f64 = std::f64::consts::PI; // π
const E_0: f64 = 8.854187817E-12; // Vacuum Permittivity
const K: f64 = 1f64 / (4f64 * PI * E_0); // Coulomb's Constant, approx. value: +9E+9

fn main() {
    // F = 1/(4 . pi . E_0) . (q q) . r / |r^3|
    let q: f64 = 1E-6; // Electric Charge in Coulomb
    let r: Vector3D = Vector3D::new(1f64,2f64,3f64); // Separation Vector
    let f = r.scalar_product(&(K * (q * q)))
        .scalar_product(&(1f64 / r.magnitude().powf(3f64))); // Coulomb's Force
    println!("{:?}", f); //: Vector3D { x: 0.0001715731561264385, y: 0.000343146312252877, z: 0.0005147194683793155 }
    println!("{}", f.magnitude()) //: 0.0006419679669926073
}
mod vector {
    #[derive(Debug)]
    pub struct Vector3D {
        x: f64,
        y: f64,
        z: f64
    }
    impl Vector3D {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Vector3D {
                x,
                y,
                z
            }
        }
        pub fn dot_product(&self, vec2: &Vector3D) -> f64
        {
            self.x * vec2.x + self.y * vec2.y + self.z * vec2.z
        }
        // pub fn cross_product(&self, vec2: &Vector3D) -> Vector3D
        // {
        //     Vector3D {
        //         x: self.y  * vec2.z - self.z * vec2.y,
        //         y: self.z * vec2.x - self.x * vec2.z,
        //         z: self.x * vec2.y - self.y * vec2.x
        //     }
        // }
        pub fn scalar_product(&self, s: &f64) -> Vector3D
        {
            Vector3D {
                x: s * self.x,
                y: s * self.y,
                z: s * self.z
            }
        }
        pub fn magnitude(&self) -> f64 {
            ((self.x.powf(2.)) + (self.y.powf(2.)) + (self.z.powf(2.))).powf(0.5)
        }
    }
}
