use crate::prelude::*;

pub fn text_update_system(mut text_query: Query<&mut Text, With<DebugText>>,
    ball_query: Query<&Transform, With<Ball>>
) {
    if let Ok((mut text)) = text_query.get_single_mut() {
        if let Ok((ball_transform)) = ball_query.get_single() {
            text.sections[1].value = format!("{}, {}",
                                             ball_transform.translation.x,
                                             ball_transform.translation.y);
        }
    }
}
