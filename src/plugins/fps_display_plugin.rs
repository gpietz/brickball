use crate::prelude::*;
use bevy::diagnostic::Diagnostics;
use bevy_prototype_lyon::prelude::tess::geom::utils::directed_angle;
use crate::FrameTimeDiagnosticsPlugin;

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_fps);
    }
}

#[derive(Component)]
struct FpsText;

fn update_fps(
    game_assets: Res<GameAssets>,
    game_settings: Res<GameSettings>,
    windows: Res<Windows>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<(Entity, &mut Text), With<FpsText>>,
    mut commands: Commands
) {
    let query_result = query.get_single_mut();

    if game_settings.fps_display_enabled && query_result.is_err() {
        let mut text = TextFactory::create_text_with_font("", game_assets.debug_font.clone(), 14.0, Color::WHITE);
        text.style.position_type = PositionType::Absolute;
        text.style.position = Rect {
            top: Val::Px(5.0),
            right: Val::Px(10.0),
            ..Default::default()
        };
        commands.spawn_bundle(text).insert(FpsText);
        return;
    } else if !game_settings.fps_display_enabled && query_result.is_ok() {
        let (entity, _) = query_result.unwrap();
        commands.entity(entity).despawn();
        return;
    }

    if game_settings.fps_display_enabled {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let (_, mut text) = query_result.unwrap();
                text.sections[0].value = format!("{:.2}", average);
            }
        }
    }
}
