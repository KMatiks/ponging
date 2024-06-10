use bevy::math::Vec2;

pub fn reflect_vec2(in_vec: Vec2, normal: Vec2) -> Vec2 {
    let normal = normal.normalize();
    let dot = in_vec.dot(normal);
    Vec2 {
        x: in_vec.x - 2.0 * dot * normal.x,
        y: in_vec.y - 2.0 * dot * normal.y,
    }
}

pub fn approx_eq_vec2(a: Vec2, b: Vec2) -> bool {
    let tolerance = 1e-6;
    (a.x - b.x).abs() < tolerance && (a.y - b.y).abs() < tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflect_across_x_axis() {
        let vec = Vec2 { x: 1.0, y: -1.0 };
        let normal = Vec2 { x: 0.0, y: 1.0 }.normalize();
        let reflected_vec = reflect_vec2(vec, normal);
        assert!(approx_eq_vec2(reflected_vec, Vec2 { x: 1.0, y: 1.0 }));
    }

    #[test]
    fn test_reflect_across_y_axis() {
        let vec = Vec2 { x: -1.0, y: 1.0 };
        let normal = Vec2 { x: 1.0, y: 0.0 }.normalize();
        let reflected_vec = reflect_vec2(vec, normal);
        assert!(approx_eq_vec2(reflected_vec, Vec2 { x: 1.0, y: 1.0 }));
    }

    #[test]
    fn test_reflect_across_diagonal() {
        let vec = Vec2 { x: 1.0, y: 1.0 };
        let normal = Vec2 { x: 1.0, y: 1.0 }.normalize();
        let reflected_vec = reflect_vec2(vec, normal);
        assert!(approx_eq_vec2(reflected_vec, Vec2 { x: -1.0, y: -1.0 }));
    }

    #[test]
    fn test_reflect_with_non_normalized_normal() {
        let vec = Vec2 { x: 1.0, y: 1.0 };
        let normal = Vec2 { x: 2.0, y: 2.0 }; // Not normalized
        let reflected_vec = reflect_vec2(vec, normal);
        assert!(approx_eq_vec2(reflected_vec, Vec2 { x: -1.0, y: -1.0 }));
    }

    #[test]
    fn test_reflect_with_zero_vector() {
        let vec = Vec2 { x: 0.0, y: 0.0 };
        let normal = Vec2 { x: 1.0, y: 0.0 }.normalize();
        let reflected_vec = reflect_vec2(vec, normal);
        assert!(approx_eq_vec2(reflected_vec, Vec2 { x: 0.0, y: 0.0 }));
    }
}
