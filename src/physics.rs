use crate::loader::NeedToLoad;
use bevy::{prelude::*, reflect::TypeUuid};

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);

pub struct Acceleration(pub Vec2);

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "fae44c41-c109-446a-a48f-0d7742ab877a"]
pub struct PhysicsSettings {
    pub normal_gravity: f32,
    pub hold_gravity: f32,
    pub initial_jump_velocity: f32,
    pub horizontal_a: f32,
    pub friction: f32,
    pub stopping_horizontal_speed: f32,
}

#[derive(Default)]
pub struct PhysicsSettingsHandle(pub Handle<PhysicsSettings>);

pub fn load_physics(
    mut need_to_load: ResMut<NeedToLoad>,
    server: Res<AssetServer>,
    mut physics_settings: ResMut<PhysicsSettingsHandle>,
) {
    physics_settings.0 = server.load("settings.physics.ron");
    need_to_load
        .handles
        .push(physics_settings.0.clone_untyped());
}

pub fn update_velocities(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut v, a) in query.iter_mut() {
        v.0 += a.0 * TIME_STEP;
    }
}

pub fn update_positions(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut p, v) in q.iter_mut() {
        p.0 += v.0 * TIME_STEP;
    }
}

pub fn update_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in q.iter_mut() {
        t.translation = p.0.extend(0.0);
    }
}

#[derive(Clone, Copy)]
pub enum CollisionShape {
    Rect(Vec2),
    Ray(Vec2),
}

pub struct CollisionData {
    pub entity: Entity,
    pub direction: Collision,
    pub collision_type: CollisionType,
}

pub enum CollisionType {
    PlayerHitsGround { ground_pos: Vec2, ground_size: Vec2 },
    PlayerRayHitsGround { ground_pos: Vec2, ground_size: Vec2 },
}

pub enum ColliderType {
    Player,
    PlayerRay,
    Ground,
}

pub struct Hitbox {
    pub shape: CollisionShape,
    pub col_type: ColliderType,
}

pub struct Hurtbox {
    pub shape: CollisionShape,
    pub col_type: ColliderType,
}

#[derive(Debug, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// taken from bevy source code
pub fn collide_aabb(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Some(Collision::Left), b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Some(Collision::Right), a_min.x - b_max.x)
        } else {
            (None, 0.0)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Some(Collision::Bottom), b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Some(Collision::Top), a_min.y - b_max.y)
        } else {
            (None, 0.0)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth.abs() < x_depth.abs() {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => None,
        }
    } else {
        None
    }
}

#[derive(Clone)]
struct RaySolution(Collision, f32);

fn c_min(a: &RaySolution, b: &RaySolution) -> RaySolution {
    if a.1 < b.1 {
        a.clone()
    } else {
        b.clone()
    }
}

fn c_max(a: &RaySolution, b: &RaySolution) -> RaySolution {
    if a.1 > b.1 {
        a.clone()
    } else {
        b.clone()
    }
}

// algorithm adapted from here https://tavianator.com/2011/ray_box.html
// may not handle collisions with corners correctly
fn raycast_to_box(ray_pos: Vec2, ray: Vec2, box_pos: Vec2, box_size: Vec2) -> Option<Collision> {
    // calculate vectors to corners of box from ray origin
    let bottom_left = box_pos - box_size / 2.0 - ray_pos; // bottom left
    let top_right = box_pos + box_size / 2.0 - ray_pos; // top right

    // calculate intersections with extended lines of sides of box
    // t is position along ray
    let n_inv = ray.normalize().recip();
    let t_tr = top_right * n_inv;
    let t_bl = bottom_left * n_inv;

    let left = RaySolution(Collision::Left, t_bl.x);
    let right = RaySolution(Collision::Right, t_tr.x);
    let top = RaySolution(Collision::Top, t_tr.y);
    let bottom = RaySolution(Collision::Bottom, t_bl.y);

    let tmin = c_max(&c_min(&left, &right), &c_min(&top, &bottom));
    let tmax = c_min(&c_max(&left, &right), &c_max(&top, &bottom));

    if (tmax.1 < tmin.1) // ray misses box completely
        || (tmin.1 < 0.0 && tmax.1 < 0.0) // points away from box
        || (tmin.1 < 0.0 && tmax.1 > ray.length()) // contained inside box
        || (tmin.1 > 0.0 && tmin.1 >= ray.length())
    // ends before box
    {
        None
    } else if tmin.1 >= 0.0 {
        // ray collides from outside box
        Some(tmin.0)
    } else {
        // ray collides from inside box
        Some(tmax.0)
    }
}

pub struct Collisions(pub Vec<CollisionData>);

impl Hurtbox {
    pub fn check_collision(
        self: &Self,
        hurt_position: &Position,
        hitbox: &Hitbox,
        hitbox_position: &Position,
        hit_entity: Entity,
    ) -> Option<CollisionData> {
        match (&self.shape, &hitbox.shape) {
            (&CollisionShape::Rect(hurt_size), &CollisionShape::Rect(hit_size)) => {
                if let Some(direction) = collide_aabb(
                    hurt_position.0.extend(0.0),
                    hurt_size,
                    hitbox_position.0.extend(0.0),
                    hit_size,
                ) {
                    return match (&self.col_type, &hitbox.col_type) {
                        (&ColliderType::Player, &ColliderType::Ground) => Some(CollisionData {
                            entity: hit_entity,
                            direction,
                            collision_type: CollisionType::PlayerHitsGround {
                                ground_pos: hitbox_position.0.clone(),
                                ground_size: hit_size.clone(),
                            },
                        }),
                        _ => None,
                    };
                }
                return None;
            }
            (&CollisionShape::Ray(ray), &CollisionShape::Rect(hit_size)) => {
                if let Some(direction) =
                    raycast_to_box(hurt_position.0, ray, hitbox_position.0, hit_size)
                {
                    return match (&self.col_type, &hitbox.col_type) {
                        (&ColliderType::PlayerRay, &ColliderType::Ground) => Some(CollisionData {
                            entity: hit_entity,
                            direction,
                            collision_type: CollisionType::PlayerRayHitsGround {
                                ground_pos: hitbox_position.0.clone(),
                                ground_size: hit_size.clone(),
                            },
                        }),
                        _ => None,
                    };
                }
                return None;
            }
            // being lazy and not supporting all collision types
            _ => return None,
        }
    }
}

pub fn check_collisions(
    mut hurtboxes: Query<(&Hurtbox, &Position, &mut Collisions)>,
    hitboxes: Query<(Entity, &Hitbox, &Position)>,
) {
    for (hurtbox, hurt_position, mut collisions) in hurtboxes.iter_mut() {
        for (hit_entity, hitbox, hitbox_position) in hitboxes.iter() {
            if let Some(collision) =
                hurtbox.check_collision(hurt_position, hitbox, hitbox_position, hit_entity)
            {
                collisions.0.push(collision);
            }
        }
    }
}

pub fn clean_up_collisions(mut q: Query<&mut Collisions>) {
    for mut c in q.iter_mut() {
        c.0.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    *---------->
        ____
        |  |
        |  |
        |__|
    */
    #[test]
    fn ray_misses_box() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(10.0, 10.0);
        let hitbox_size = Vec2::new(1.0, 1.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        assert!(result.is_none());
    }

    /*
            ____
    *---->  |  |
            |__|
    */

    #[test]
    fn ray_ends_before_box() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(6.0, 0.0);
        let hitbox_size = Vec2::new(1.0, 1.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        assert!(result.is_none());
    }

    /*
    ____
    |  |  *---->
    |__|
    */

    #[test]
    fn ray_starts_after_box() {
        let ray_pos = Vec2::new(7.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(6.0, 0.0);
        let hitbox_size = Vec2::new(1.0, 1.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        assert!(result.is_none());
    }

    // checks case where ray pierces box
    /*
        ______
        |    |
     *--X----X-->
        |    |
        |____|
    */
    #[test]
    fn it_detects_collision_from_left() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(3.0, 0.0);
        let hitbox_size = Vec2::new(2.0, 4.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Left => assert!(true),
            _ => assert!(false),
        }
    }

    // checks case where ray pierces box
    /*
        ______
        |    |
    <---X----X--*
        |    |
        |____|
    */
    #[test]
    fn it_detects_collision_from_right() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(-5.0, 0.0);
        let hitbox_pos = Vec2::new(-3.0, 0.0);
        let hitbox_size = Vec2::new(2.0, 4.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Right => assert!(true),
            _ => assert!(false),
        }
    }

    /*
        _____
        |   |
    *---X-> |
        |   |
        -----
     */
    #[test]
    fn it_detects_end_point_in_box() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(6.0, 0.0);
        let hitbox_size = Vec2::new(3.0, 4.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Left => assert!(true),
            _ => assert!(false),
        }
    }

    /*
        _____
        |   |
        | *-X--->
        |   |
        -----
    */
    #[test]
    fn it_detects_origin_in_box() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(5.0, 0.0);
        let hitbox_pos = Vec2::new(0.0, 0.0);
        let hitbox_size = Vec2::new(3.0, 4.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Right => assert!(true),
            _ => assert!(false),
        }
    }

    /*
    _______
    |     |
    | *-> |
    |     |
    |_____|
    */
    #[test]
    fn it_doesnt_detect_ray_fully_enclosed_in_box() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(1.0, 0.0);
        let hitbox_pos = Vec2::new(0.0, 0.0);
        let hitbox_size = Vec2::new(3.0, 4.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        assert!(result.is_none());
    }

    #[test]
    fn it_detects_collision_from_top() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(0.0, -5.0);
        let hitbox_pos = Vec2::new(0.0, -2.0);
        let hitbox_size = Vec2::new(4.0, 2.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Top => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn it_detects_collision_from_bottom() {
        let ray_pos = Vec2::new(0.0, 0.0);
        let ray = Vec2::new(0.0, 5.0);
        let hitbox_pos = Vec2::new(0.0, 2.0);
        let hitbox_size = Vec2::new(4.0, 2.0);

        let result = raycast_to_box(ray_pos, ray, hitbox_pos, hitbox_size);
        let collision = result.unwrap();

        match collision {
            Collision::Bottom => assert!(true),
            _ => assert!(false),
        }
    }
}
