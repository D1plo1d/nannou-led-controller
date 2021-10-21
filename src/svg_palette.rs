use std::fs;

use nannou::{color::{Gradient, Hsl, IntoColor, Srgb}};
use serde::{Deserialize};
use eyre::{Context, Result, eyre};

#[derive(Deserialize, Debug)]
pub struct Svg {
    g: SvgG,
}

#[derive(Deserialize, Debug)]
pub struct SvgG {
    defs: SvgDefs,
}

#[derive(Deserialize, Debug)]
pub struct SvgDefs {
    #[serde(rename="linearGradient")]
    linear_gradient: SvgLinearGradient,
}

#[derive(Deserialize, Debug)]
pub struct SvgLinearGradient {
    #[serde(rename="stop")]
    stops: Vec<SvgStop>,
}


#[derive(Deserialize, Debug)]
pub struct SvgStop {
    offset: String,
    #[serde(rename="stop-color")]
    color: String,
}

pub fn to_gradient(palette_name: &str) -> Result<Gradient<Hsl>> {
    let valid_name = palette_name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_');

    if !valid_name {
        return Err(eyre!("Invalid palette: {:?}", palette_name));
    }

    let mut root_dir = std::env::current_exe().unwrap();

    for _ in 0..3 {
        root_dir.pop();
    }

    let file_path = root_dir.join(format!("palettes/{}.svg", palette_name));

    if !file_path.exists() {
        return Err(eyre!("palette does not exist: {:?}", file_path));
    }

    let svg: Svg  = serde_xml_rs::from_str(&fs::read_to_string(file_path)?)?;

    let stops = svg.g.defs.linear_gradient.stops
        .into_iter()
        .map(|stop| {
            let offset = stop.offset.strip_suffix('%')
                .ok_or_else(|| eyre!("Invalid stop offset"))?
                .parse::<f32>()?;

            let color_values = stop.color
                .strip_prefix("rgb(")
                .ok_or_else(|| eyre!("Missing 'rgb' color string prefix"))?
                .strip_suffix(")")
                .ok_or_else(|| eyre!("Missing color string closing parentheses"))?
                .split(',')
                .map(|s| {
                    s
                        .trim()
                        .parse::<f32>()
                        .map(|v| v / 255.0)
                        .wrap_err("Invalid rgb value")
                })
                .collect::<Result<Vec<_>>>()?;

            let color = if let [r, g, b] = color_values[..] {
                Srgb::new(r, g, b).into_hsl()
            } else {
                return Err(eyre!("Missing rgb value(s)"));
            };

            Ok((
                offset,
                color,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let gradient = Gradient::with_domain(stops);

    Ok(gradient)
}
