use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};
use nalgebra::{vector, Vector2};
use rapier2d::prelude::{RigidBodyHandle, RigidBodySet};

#[derive(Debug, Clone)]
pub struct ShapeRect {
    pub size: Vector2<f32>,
    pub position: Vector2<f32>,
    pub physics_scale: f32,
    pub speed: f32,
    pub jump: f32,
    pub physics_handle: Option<RigidBodyHandle>,
    pub draw_params: DrawRectangleParams,
}

impl ShapeRect {
    pub fn new(
        size: Vector2<f32>,
        position: Vector2<f32>,
        physics_scale: f32,
        speed: f32,
        jump: f32,
        physics_handle: Option<RigidBodyHandle>,
        draw_params: DrawRectangleParams,
    ) -> ShapeRect {        
        ShapeRect {
            size: size  / physics_scale, 
            position: position / physics_scale,
            physics_scale,
            speed, 
            jump,
            physics_handle,
            draw_params,
        }
    }

    pub fn physics_handle(
        &mut self,
        physics_handle: RigidBodyHandle
    ) -> &mut ShapeRect {
        self.physics_handle = Some(physics_handle);
        self
    }

    pub fn add_user_input(&mut self, user_input: Vector2<f32>, rigid_body_set: &mut RigidBodySet) {
        let body = rigid_body_set.get_mut(self.physics_handle.unwrap()).unwrap();
        body.apply_impulse(vector![user_input.x, user_input.y], true);
    }

    pub fn update(&mut self, rigid_body_set: &RigidBodySet) {
        if let Some(handle) = self.physics_handle {
            let body = &rigid_body_set[handle];
            self.position = *body.translation();
            self.draw_params.rotation = body.rotation().re.to_radians();
        }
    }

    pub fn draw(&self) {
        draw_rectangle_ex(
            self.physics_scale * self.position.x,
            -self.physics_scale * self.position.y,
            self.physics_scale * self.size.x,
            self.physics_scale * self.size.y,
            self.draw_params.clone());
    }

    // pub fn out(&self) {
    //     println!("size:{}, position:{}, physics_scale:{}, speed:{}, jump:{}", self.size,self.position, self.physics_scale, self.speed, self.jump);
    // }
}