use crate::prelude::*;

pub struct TextFactory;

impl TextFactory {
    pub fn create_text_with_font(
        text: &str,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
    ) -> TextBundle {
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: text.to_owned(),
                    style: TextStyle {
                        font,
                        font_size,
                        color,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
