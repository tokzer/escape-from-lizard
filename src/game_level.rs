pub mod game_level {
    use macroquad::{math::Vec2, shapes::DrawRectangleParams};
    use nalgebra::vector;
    use rapier2d::prelude::{ColliderBuilder, ColliderSet};

    use crate::{colarc, game_resources::ShapeRect};

    const WINDOW_WIDTH: f32 = 1366.0;
    const WINDOW_HEIGHT: f32 = 768.0;
    const CHARACTER_SIZE: f32 = 40.0;
    const BARRIER_SIZE: f32 = CHARACTER_SIZE * 0.5;
    const BARRIER_OFFSET: f32 = BARRIER_SIZE * 0.5;
    // 1 meter is 50 pixels
    const PHYSICS_SCALE: f32 = 50.0;
    
    pub struct GameLevel {
        // TODO: pub lv_name: String,
        pub lv_barriers: Vec<ShapeRect>,
    }

    impl Default for GameLevel {
        fn default() -> GameLevel {
            GameLevel {
                lv_barriers: vec![ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.1), (-WINDOW_HEIGHT * 0.1)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.2), (-WINDOW_HEIGHT * 0.2)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.3), (-WINDOW_HEIGHT * 0.3)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.4), (-WINDOW_HEIGHT * 0.4)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.5), (-WINDOW_HEIGHT * 0.5)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.6), (-WINDOW_HEIGHT * 0.6)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.7), (-WINDOW_HEIGHT * 0.7)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.8), (-WINDOW_HEIGHT * 0.8)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new(
                    vector![200.0, BARRIER_SIZE], 
                    vector![(WINDOW_WIDTH * 0.9), (-WINDOW_HEIGHT * 0.9)], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                ),ShapeRect::new( // Ceiling
                    vector![WINDOW_WIDTH, BARRIER_SIZE], 
                    vector![WINDOW_WIDTH * 0.5, -BARRIER_OFFSET], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::BLUE_CELESTIAL }
                ),ShapeRect::new( // Left wall
                    vector![BARRIER_SIZE, WINDOW_HEIGHT - BARRIER_SIZE * 2.0], 
                    vector![BARRIER_OFFSET, -WINDOW_HEIGHT * 0.5], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::YELLOW_SUNGLOW }
                ),ShapeRect::new( // Right wall
                    vector![BARRIER_SIZE, WINDOW_HEIGHT - BARRIER_SIZE * 2.0], 
                    vector![WINDOW_WIDTH - BARRIER_OFFSET, -WINDOW_HEIGHT * 0.5], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::YELLOW_SUNGLOW }
                ),ShapeRect::new( // Floor
                    vector![WINDOW_WIDTH, BARRIER_SIZE], 
                    vector![WINDOW_WIDTH * 0.5, -WINDOW_HEIGHT + BARRIER_OFFSET], 
                    PHYSICS_SCALE, 
                    0.0, 
                    0.0, 
                    None, 
                    DrawRectangleParams { offset: Vec2::new(0.5, 0.5), rotation: 0.0, color: colarc::GREEN_GRASS }
                )],
            }
        }
    }

    impl GameLevel {
        // TODO: pub new(BARRIER_SIZE: f32, WINDOW_WIDTH: f32, WINDOW_HEIGHT: f32) -> GameLevel {}
        pub fn add_to(&self, collider_set: &mut ColliderSet) {
            for b in &self.lv_barriers {
                let collider = ColliderBuilder::cuboid(b.size.x * 0.5, b.size.y * 0.5)
                    .translation(b.position)
                    .build();
                collider_set.insert(collider);
            }
        }

        // TODO: pub fn load_level(level_path: String) -> GameLevel {}
        // TODO: pub fn save_level(level_path: String) -> bool {}
    }
}