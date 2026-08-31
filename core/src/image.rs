use crate::{Property, StyleError, StyleRule, Value};

pub(crate) fn parse(class: &str, offset: usize) -> Option<Result<StyleRule, StyleError>> {
    if let Some(path) = class.strip_prefix("bgi_url") {
        let path = path.trim_start();
        let Some(path) = path
            .strip_prefix('(')
            .and_then(|path| path.strip_suffix(')'))
        else {
            return Some(Err(crate::error(class, offset, "invalid image url")));
        };
        let Some(path) = path
            .strip_prefix('"')
            .and_then(|path| path.strip_suffix('"'))
        else {
            return Some(Err(crate::error(
                class,
                offset,
                "image url must be a string",
            )));
        };
        if path.is_empty() {
            return Some(Err(crate::error(
                class,
                offset,
                "image url cannot be empty",
            )));
        }
        return Some(Ok(StyleRule {
            property: Property::Image,
            value: Value::ImageUrl(path.to_owned()),
        }));
    }

    let (property, value) = if class == "bgi_auto" {
        (Property::ImageMode, Value::ImageModeAuto)
    } else if class == "bgi_stretch" {
        (Property::ImageMode, Value::ImageModeStretch)
    } else if class == "bgi_repeat" {
        (Property::ImageMode, Value::ImageModeRepeat)
    } else if class == "bgi_repeat_x" {
        (Property::ImageMode, Value::ImageModeRepeatX)
    } else if class == "bgi_repeat_y" {
        (Property::ImageMode, Value::ImageModeRepeatY)
    } else if class == "bgi_no_repeat" {
        (Property::ImageMode, Value::ImageModeNoRepeat)
    } else if class == "bgi_flip_x" {
        (Property::ImageFlipX, Value::ImageFlipX)
    } else if class == "bgi_flip_y" {
        (Property::ImageFlipY, Value::ImageFlipY)
    } else {
        return None;
    };

    Some(Ok(StyleRule { property, value }))
}
