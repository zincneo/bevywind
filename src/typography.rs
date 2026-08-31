use bevy::ecs::template::FnTemplate;
use bevy::scene::ResolvedScene;
use bevy::text::{FontStyle, FontWeight, Justify, LineBreak, LineHeight};
use bevy::text::{TextColor, TextFont, TextLayout};
use bevywind_core::{Property, StyleRule, Value};

pub(crate) fn apply(scene: &mut ResolvedScene, rules: &[StyleRule]) {
    if let Some(rule) = rules
        .iter()
        .find(|rule| rule.property == Property::TextColor)
    {
        let Value::TextColor(red, green, blue, alpha) = rule.value else {
            unreachable!();
        };
        scene.push_template(FnTemplate(move |_| {
            Ok(TextColor(bevy::color::Color::srgba(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
                alpha as f32 / 255.0,
            )))
        }));
    }

    let mut font = None;
    for rule in rules {
        match rule.value {
            Value::FontSize(value) => {
                font.get_or_insert_with(TextFont::default).font_size =
                    bevy::text::FontSize::Px(value as f32);
            }
            Value::FontWeight(value) => {
                font.get_or_insert_with(TextFont::default).weight = FontWeight(value);
            }
            Value::FontStyleNormal => {
                font.get_or_insert_with(TextFont::default).style = FontStyle::Normal;
            }
            Value::FontStyleItalic => {
                font.get_or_insert_with(TextFont::default).style = FontStyle::Italic;
            }
            _ => {}
        }
    }
    if let Some(font) = font {
        scene.push_template(FnTemplate(move |_| Ok(font.clone())));
    }

    let mut layout = TextLayout::default();
    let mut has_layout = false;
    for rule in rules {
        match rule.value {
            Value::TextJustifyLeft => {
                layout.justify = Justify::Left;
                has_layout = true;
            }
            Value::TextJustifyCenter => {
                layout.justify = Justify::Center;
                has_layout = true;
            }
            Value::TextJustifyRight => {
                layout.justify = Justify::Right;
                has_layout = true;
            }
            Value::TextJustify => {
                layout.justify = Justify::Justified;
                has_layout = true;
            }
            Value::TextJustifyStart => {
                layout.justify = Justify::Start;
                has_layout = true;
            }
            Value::TextJustifyEnd => {
                layout.justify = Justify::End;
                has_layout = true;
            }
            Value::LineBreakWordBoundary => {
                layout.linebreak = LineBreak::WordBoundary;
                has_layout = true;
            }
            Value::LineBreakNoWrap => {
                layout.linebreak = LineBreak::NoWrap;
                has_layout = true;
            }
            Value::LineBreakAnyCharacter => {
                layout.linebreak = LineBreak::AnyCharacter;
                has_layout = true;
            }
            Value::LineBreakWordOrCharacter => {
                layout.linebreak = LineBreak::WordOrCharacter;
                has_layout = true;
            }
            _ => {}
        }
    }
    if has_layout {
        scene.push_template(FnTemplate(move |_| Ok(layout)));
    }

    if let Some(rule) = rules
        .iter()
        .find(|rule| rule.property == Property::LineHeight)
    {
        let line_height = match rule.value {
            Value::LineHeightRelative(value) => LineHeight::RelativeToFont(value as f32 / 1000.0),
            Value::LineHeightPixels(value) => LineHeight::Px(value as f32),
            _ => unreachable!(),
        };
        scene.push_template(FnTemplate(move |_| Ok(line_height)));
    }
}
