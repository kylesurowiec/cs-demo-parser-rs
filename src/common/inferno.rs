use crate::sendtables::entity::Vector;
use crate::sendtables2::Entity;

/// Representation of an active inferno (molotov/incendiary flames).
#[derive(Default, Clone)]
pub struct Inferno {
    /// Underlying entity for the inferno if available.
    pub entity: Option<Entity>,
    /// Individual flame origins gathered from the entity properties.
    pub flames: Vec<Vector>,
    /// Cached convex hull around all flames in `flames`.
    pub hull: Vec<Vector>,
}

impl Inferno {
    /// Updates the convex hull from the currently stored flames.
    pub fn update_hull(&mut self) {
        self.hull = convex_hull(&self.flames);
    }

    /// Adds a new flame origin and recomputes the convex hull.
    pub fn add_flame(&mut self, pos: Vector) {
        self.flames.push(pos);
        self.update_hull();
    }

    /// Returns the precalculated convex hull points.
    pub fn hull(&self) -> &[Vector] {
        &self.hull
    }
}

/// Calculates the convex hull of a set of 2D points using the monotone chain
/// algorithm. `Vector::z` is ignored.
pub fn convex_hull(points: &[Vector]) -> Vec<Vector> {
    let mut pts = points.to_vec();
    if pts.len() <= 1 {
        return pts;
    }

    pts.sort_by(|a, b| match a.x.partial_cmp(&b.x) {
        | Some(core::cmp::Ordering::Equal) => {
            a.y.partial_cmp(&b.y).unwrap_or(core::cmp::Ordering::Equal)
        },
        | Some(o) => o,
        | None => core::cmp::Ordering::Equal,
    });

    let mut lower: Vec<Vector> = Vec::new();
    for p in &pts {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= 0.0
        {
            lower.pop();
        }
        lower.push(p.clone());
    }

    let mut upper: Vec<Vector> = Vec::new();
    for p in pts.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= 0.0
        {
            upper.pop();
        }
        upper.push(p.clone());
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn cross(o: &Vector, a: &Vector, b: &Vector) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}
