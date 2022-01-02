use crate::prelude::*;

pub fn text_update(mut text_query: Query<&mut Text, With<DebugText>>,
                   ball_query: Query<(&Ball, &Transform)>) {
    if let Ok((mut text)) = text_query.single_mut() {
        if let Ok((ball, ball_transform)) = ball_query.single() {
            text.sections[1].value = format!("{}, {}",
                                             ball_transform.translation.x,
                                             ball_transform.translation.y);
        }
    }
}
