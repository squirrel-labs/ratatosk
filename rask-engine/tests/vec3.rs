use rask_engine::math::{EPSILON, Vec3};

#[test]
fn test_add_vec3() {
    let a = Vec3::new(1.0, 60.0, 7.5);
    let b = Vec3::new(-3.0, -4.0, 2.5);
    let c = Vec3::new(-2.0, 56.0, 10.0);

    assert_eq!(a + b, c);
}

#[test]
fn test_add_assign_vec3() {
    let mut a = Vec3::new(1.0, 60.0, 7.5);
    let b = Vec3::new(-3.0, -4.0, 2.5);
    let c = Vec3::new(-2.0, 56.0, 10.0);
    a += b;

    assert_eq!(a, c);
}

#[test]
fn test_sub_vec3() {
    let a = Vec3::new(1.0, 60.0, 7.5);
    let b = Vec3::new(-3.0, -4.0, 2.5);
    let c = Vec3::new(4.0, 64.0, 5.0);

    assert_eq!(a - b, c);
}

#[test]
fn test_sub_assign_vec3() {
    let mut a = Vec3::new(1.0, 60.0, 7.5);
    let b = Vec3::new(-3.0, -4.0, 2.5);
    let c = Vec3::new(4.0, 64.0, 5.0);
    a -= b;

    assert_eq!(a, c);
}

#[test]
fn test_neg_vec3() {
    let a = Vec3::new(1.0, 4.0, 7.5);
    let b = Vec3::new(-1.0, -4f32, -7.5);

    assert_eq!(-a, b);
}

#[test]
fn test_mul_f32() {
    let a = Vec3::new(3.9, 2.3, -4.2);

    assert_eq!(a * 2.0, Vec3::new(7.8, 4.6, -8.4));
}

#[test]
fn test_mul_vec3() {
    let a = Vec3::new(1.0, 7.5, 4.0);
    let b = Vec3::new(-4.2, 2.0, 0.25);

    assert_eq!(a * b, Vec3::new(-4.2, 15.0, 1.0));
}

#[test]
fn test_div_f32() {
    let a = Vec3::new(3.0, 4.2, -6.2);

    assert_eq!(a / 2.0, Vec3::new(1.5, 2.1, -3.1));
}

#[test]
fn test_div_vec3() {
    let a = Vec3::new(-4.2, 7.5, 4.0);
    let b = Vec3::new(1.0, 2.0, 0.25);

    assert_eq!(a / b, Vec3::new(-4.2, 3.75, 16.0));
}

#[test]
fn test_less_vec3() {
    let a = Vec3::new(1.0, 50.0, 7.5);
    let b = Vec3::new(-3.0, 1.0, 2.5);

    assert!(b < a);
}

#[test]
fn test_less_vec3_fail() {
    let a = Vec3::new(1.0, 67.0, 7.5);
    let b = Vec3::new(3.0, -2.0, 2.5);

    assert!(!(a < b));
}

#[test]
fn test_greater_vec3() {
    let a = Vec3::new(1.0, 4.0, 7.5);
    let b = Vec3::new(-3.0, 1.0, 2.5);

    assert!(a > b);
}

#[test]
fn test_greater_vec3_fail() {
    let a = Vec3::new(1.0, 54.0, 7.5);
    let b = Vec3::new(3.0, 5.0, 2.5);

    assert!(!(a > b));
}

#[test]
fn test_norm_vec3() {
    let a = Vec3::new(13.0, 4.0, 16.0);

    assert!(f32::abs(a.norm() - 21.0) < EPSILON);
}

#[test]
fn test_norm2_vec3() {
    let a = Vec3::new(1.0, 3.0, 2.0);

    assert!(f32::abs(a.norm2() - 14.0) < EPSILON);
}

#[test]
fn test_dot() {
    let a = Vec3::new(3.0, 1.0, -6.0);
    let b = Vec3::new(1.0, 1.0, 7.0);

    assert!(f32::abs(a.dot(b) - -38.0) < EPSILON);
}

#[test]
fn test_normalized_vec3() {
    let a = Vec3::new(-1.0, 1.0, std::f32::consts::SQRT_2);
    let b = Vec3::new(-0.5, 0.5, std::f32::consts::FRAC_1_SQRT_2);

    assert_eq!(a.normalized(), b);
}
