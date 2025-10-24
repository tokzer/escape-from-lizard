use std::sync::mpsc;
// use std::{os::windows::prelude::OsStrExt, iter::once, ptr::null};
// use windows_sys::{core::*, Win32::{Foundation::*, Graphics::Gdi::ValidateRect, UI::WindowsAndMessaging::{FindWindowW, GetWindowRect}, Foundation::{HWND, RECT}}};

use macroquad::{input::{is_key_down, is_key_pressed, is_key_released, KeyCode}, math::Vec2, shapes::DrawRectangleParams, window::{clear_background, next_frame, screen_height, screen_width, Conf}};
use rapier2d::prelude::{nalgebra::{vector, Vector2}, ActiveEvents, CCDSolver, ChannelEventCollector, ColliderBuilder, ColliderSet, CollisionEvent, CollisionEventFlags, DefaultBroadPhase, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, RigidBodyBuilder, RigidBodySet};

mod colarc;

mod game_level;

mod game_resources;
use game_resources::ShapeRect;

use crate::game_level::game_level::GameLevel;

const WINDOW_WIDTH: f32 = 1366.0;
const WINDOW_HEIGHT: f32 = 768.0;
const CHARACTER_SIZE: f32 = 40.0;
// 1 meter is 50 pixels
const PHYSICS_SCALE: f32 = 50.0;
const GRAVITY: Vector2<f32> = vector![0.0, -9.82];

fn conf() -> Conf {
    #[allow(clippy::cast_possible_truncation)]
    Conf {
        window_title: String::from("Escape from the Lizard people!"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

// TODO: Fix this
// fn get_window() -> [i32; 2] {
//         let name_or_id = std::env::args_os().nth(1).unwrap();
//     let id: HWND = match name_or_id.to_str().and_then(|s| s.parse().ok()) {
//         Some(id) => id,
//         None => {
//             let name: Vec<u16> = name_or_id.as_os_str().encode_wide().chain(once(0)).collect();
//             unsafe { FindWindowW(null(), name.as_ptr()) }
//         }
//     };
//     let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
//     if id != 0 && unsafe { GetWindowRect(id, &mut rect) } != 0 {
//         println!("{} {} {}", id, rect.right - rect.left, rect.bottom - rect.top);
//     }

//     [rect.right - rect.left, rect.bottom - rect.top]
// }

// fn load_level() {
//         let mut high_score: u32 = fs::read_to_string("highscore.dat")
//         .map_or(Ok(0), |i| i.parse::<u32>())
//         .unwrap_or(0);
// }

// fn write_level() {
//     fs::write("highscore.dat", high_score.to_string()).ok();
// }

#[macroquad::main(conf)]
async fn main() {

    println!("width: {}", screen_width());
    println!("height: {}", screen_height());
 
    let game_levels = GameLevel::default();
    
    let mut cubicle = ShapeRect::new(
        vector![CHARACTER_SIZE, CHARACTER_SIZE], 
        vector![(WINDOW_WIDTH * 0.5) + 25.0, (-WINDOW_HEIGHT * 0.5) + 25.0], 
        PHYSICS_SCALE, 
        1.6, 
        4.2, 
        None, 
        DrawRectangleParams { offset: Vec2 { x: 0.5, y: 0.5 }, rotation: 0.0, color: colarc::ORANGE_CARROT }
    );

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    game_levels.add_to(&mut collider_set);
    let cubicle_body = RigidBodyBuilder::dynamic()
        .translation(cubicle.position)
        .lock_rotations()
        .build();
    let cubicle_collider = ColliderBuilder::cuboid(cubicle.size.x * 0.5, cubicle.size.y * 0.5)
        .restitution(0.7)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build();
    let cubicle_body_handle = rigid_body_set.insert(cubicle_body);
    cubicle.physics_handle(cubicle_body_handle);

    collider_set.insert_with_parent(cubicle_collider, cubicle_body_handle, &mut rigid_body_set);

    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let (collision_send, collision_recv) = mpsc::channel();
    let (contact_force_send, _contact_force_recv) = mpsc::channel();
    let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

    loop {
        if is_key_released(KeyCode::Escape) {
            break;
        }
        if is_key_released(KeyCode::Key1) {
            break;
        }
        if is_key_released(KeyCode::Key2) {
            break;
        }
        if is_key_pressed(KeyCode::Up) { // TODO: Double jump
            cubicle.add_user_input(vector![0.0, cubicle.jump], &mut rigid_body_set);
        }
        if is_key_down(KeyCode::Down) {
            // TODO: Couch for higher jump;
            println!("key_pressed(KeyCode::Down");
        }
        // TODO: Continous movement and time based force
        // TODO: Only move when still and on the ground
        // if is_key_down(KeyCode::Left) {
        if is_key_pressed(KeyCode::Left) {
            println!("key_pressed(KeyCode::Left");
            cubicle.add_user_input(vector![-cubicle.speed, cubicle.speed], &mut rigid_body_set);
        }
        if is_key_pressed(KeyCode::Right) {
            cubicle.add_user_input(vector![cubicle.speed, cubicle.speed], &mut rigid_body_set);
        }

        clear_background(colarc::GRAY_GUNMETAL);
        for iter in game_levels.lv_barriers.clone() {
            iter.draw();
        }

        cubicle.draw();

        physics_pipeline.step(
            &GRAVITY, 
            &integration_parameters, 
            &mut island_manager, 
            &mut broad_phase, 
            &mut narrow_phase, 
            &mut rigid_body_set, 
            &mut collider_set, 
            &mut impulse_joint_set, 
            &mut multibody_joint_set, 
            &mut ccd_solver, 
            &physics_hooks, 
            &event_handler
        );

        cubicle.update(&rigid_body_set);

        while let Ok(collision_event) = collision_recv.try_recv() {
            if let CollisionEvent::Started(
                _collider_handle_1, 
                _collider_handle_2, 
                CollisionEventFlags::SENSOR,
            ) = collision_event
            {
                println!("paused = true");
            }
        }

        // TODO: pause game on window resize
        next_frame().await;
    }
}
