use crate::prelude::*;

pub struct SpriteBundleFactory;

impl SpriteBundleFactory {
    pub fn with_color(x: f32, y: f32, z: u8, width: f32, height: f32, color: Color) -> SpriteBundle  {
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, f32::from(z)),
                ..Default::default()

            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn with_texture(x: f32, y: f32, z: u8, width: f32, height: f32, texture: &Handle<Image>) -> SpriteBundle  {
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, f32::from(z)),
                ..Default::default()

            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            texture: (*texture).clone(),
            ..Default::default()
        }
    }
}