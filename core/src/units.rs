use crate::{StyleError, Value, error};

pub(crate) fn parse(value: &str, class: &str, offset: usize) -> Result<Value, StyleError> {
    if value == "full" {
        return Ok(Value::Percent(100));
    }
    let (number, constructor): (&str, fn(u16) -> Value) =
        if let Some(number) = value.strip_suffix("px") {
            (number, Value::Pixels)
        } else if let Some(number) = value.strip_suffix("per") {
            (number, Value::Percent)
        } else if let Some(number) = value.strip_suffix('w') {
            (number, Value::ViewportWidth)
        } else if let Some(number) = value.strip_suffix('h') {
            (number, Value::ViewportHeight)
        } else {
            return Err(error(class, offset, "invalid dimension value"));
        };
    Ok(constructor(parse_number(
        number,
        class,
        offset,
        "dimension",
    )?))
}

pub(crate) fn parse_number(
    number: &str,
    class: &str,
    offset: usize,
    kind: &str,
) -> Result<u16, StyleError> {
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(error(
            class,
            offset,
            &format!("{kind} value must be a non-negative integer"),
        ));
    }
    number
        .parse::<u16>()
        .map_err(|_| error(class, offset, &format!("{kind} value does not fit in u16")))
}
