use super::*;
use once_cell::sync::OnceCell;

fn six_sides_() -> Vec<Mat4> {
    let mut result = Vec::new();

    let mut trans = Mat4::identity();
    trans = glm::rotate(
        &trans,
        -std::f32::consts::PI * 0.5,
        &Vec3::new(0.0, 0.0, 1.0),
    );
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 0.5,
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.push(trans);

    trans = Mat4::identity();
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 0.5,
        &Vec3::new(0.0, 0.0, 1.0),
    );
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 0.5,
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.push(trans);

    trans = Mat4::identity();
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 1.0,
        &Vec3::new(0.0, 0.0, 1.0),
    );
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 0.5,
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.push(trans);

    trans = Mat4::identity();
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 0.5,
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.push(trans);

    trans = Mat4::identity();
    trans = glm::rotate(
        &trans,
        std::f32::consts::PI * 1.0,
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.push(trans);

    trans = Mat4::identity();
    result.push(trans);

    result
}

static POOL: OnceCell<Vec<Mat4>> = OnceCell::new();

pub type SixDir = i8;

/// rotation transform (right-left-back-front-top-bottom)
pub fn six_sides(i: SixDir) -> &'static Mat4 {
    &POOL.get_or_init(six_sides_)[i as usize]
}

/// X and Y
pub fn six_sides_dir(i: SixDir) -> (Vec3, Vec3) {
    match i {
        0 => (Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        1 => (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        2 => (Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        3 => (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        4 => (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        5 => (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        x => panic!("six_sides_dir failed: {}", x),
    }
}

pub fn rotation_between_shortest(a: Vec3, b: Vec3) -> Mat4 {
    let a = normalize(&a);
    let b = normalize(&b);
    let c = glm::cross(&a, &b);
    let sine = length(&c);
    let cosine = dot(&a, &b);
    if cosine > 0.9999 {
        return Mat4::identity();
    }

    unsafe {
        if cosine < -0.9999 {
            let mut m = Mat4::identity();
            *m.get_unchecked_mut((3, 3)) = 1.0;
            return m;
        }

        let mut v = Mat4::from_element(0.0);
        *v.get_unchecked_mut((0, 1)) = -c.z;
        *v.get_unchecked_mut((0, 2)) = c.y;
        *v.get_unchecked_mut((1, 2)) = -c.x;
        *v.get_unchecked_mut((1, 0)) = c.z;
        *v.get_unchecked_mut((2, 0)) = -c.y;
        *v.get_unchecked_mut((2, 1)) = c.x;
        Mat4::identity() + v + v * v * (1.0 - cosine) / (sine * sine)
    }
}

pub fn rotation_between(a1: &Vec3, b1: &Vec3, a2: &Vec3, b2: &Vec3) -> Mat4 {
    if a1 == a2 && b1 == b2 {
        return Mat4::identity();
    }

    let a1 = glm::normalize(a1);
    let b1 = glm::normalize(b1);
    let a2 = glm::normalize(a2);
    let b2 = glm::normalize(b2);

    let c1 = glm::cross(&a1, &b1);
    let c2 = glm::cross(&a2, &b2);

    let before = Mat3::from_columns(&[a1, b1, c1]);
    let after = Mat3::from_columns(&[a2, b2, c2]);

    let mut result = Mat4::from_element(0.0);
    // Assign to the upper-left corner of the result
    result
        .fixed_slice_mut::<3, 3>(0, 0)
        .copy_from(&(after * glm::inverse(&before)));
    unsafe {
        *result.get_unchecked_mut((3, 3)) = 1.0;
    }
    result
}
