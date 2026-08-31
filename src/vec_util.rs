use glam::Vec3A;

pub fn random(min: f32, max: f32) -> Vec3A {
    Vec3A::new(
        min + (max - min) * fastrand::f32(),
        min + (max - min) * fastrand::f32(),
        min + (max - min) * fastrand::f32(),
    )
}

pub fn random_in_unit_disk() -> Vec3A {
    loop {
        let p = Vec3A::new(
            fastrand::f32() * 2.0 - 1.0,
            fastrand::f32() * 2.0 - 1.0,
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3A {
    loop {
        let p = Vec3A::new(
            fastrand::f32() * 2.0 - 1.0,
            fastrand::f32() * 2.0 - 1.0,
            fastrand::f32() * 2.0 - 1.0,
        );
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn near_zero(v: Vec3A) -> bool {
    const S: f32 = 1e-8;
    v.x.abs() < S && v.y.abs() < S && v.z.abs() < S
}
