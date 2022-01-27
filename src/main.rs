use colored::{Color, Colorize};
use serde::Deserialize;
use serde_json::{Error, Map, Value};
use std::{collections::HashMap, io, process::exit};
use structopt::StructOpt;

/// Pretty print a stream of json logs.
#[derive(Debug, StructOpt)]
struct Opt {
    /// Color scheme to use [chalk, greyscale, ocean, solarized, mocha]
    #[structopt(long, default_value = "ocean")]
    color_scheme: String,
}

fn main() {
    let opt = Opt::from_args();
    let color_scheme_yaml = include_str!("color_schemes.yml");
    let colors_schemes: HashMap<String, ColorScheme> =
        serde_yaml::from_str(color_scheme_yaml).unwrap();
    if let Some(color_scheme) = colors_schemes.get(&opt.color_scheme) {
        let color_palette = ColorPalette::from_color_scheme(color_scheme);

        let format_service = FormatService {
            colors: color_palette,
        };
        let mut buffer = String::new();

        while let Ok(bytes_read) = io::stdin().read_line(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let output = format_service.format_input(&buffer);
            println!("{}", output.trim());
            buffer.clear();
        }
    } else {
        println!(
            "{}",
            format!("Unknown color scheme: {}", opt.color_scheme).red()
        );
        let available_schemes: Vec<&str> = colors_schemes.keys().map(|s| s.as_str()).collect();
        println!("Available color schemes: {}", available_schemes.join(", "));
        exit(1);
    }
}

#[derive(Debug)]
struct ColorPalette {
    null: Color,
    bool: Color,
    number: Color,
    string: Color,
    object_key: Color,
}

impl ColorPalette {
    fn from_color_scheme(color_scheme: &ColorScheme) -> Self {
        Self {
            null: color_scheme.number.to_true_color(),
            bool: color_scheme.bool.to_true_color(),
            number: color_scheme.number.to_true_color(),
            string: color_scheme.string.to_true_color(),
            object_key: color_scheme.object_key.to_true_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CsColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CsColor {
    fn to_true_color(&self) -> Color {
        Color::TrueColor {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ColorScheme {
    null: CsColor,
    bool: CsColor,
    number: CsColor,
    string: CsColor,
    object_key: CsColor,
}

struct FormatService {
    colors: ColorPalette,
}

impl FormatService {
    fn format_input(&self, line: &str) -> String {
        let parsed_json: Result<Value, Error> = serde_json::from_str(line);
        match parsed_json {
            Ok(j) => self.format_json(&j, 0),
            _ => line.to_string(),
        }
    }

    fn format_json(&self, value: &Value, depth: usize) -> String {
        match value {
            Value::Null => "null".color(self.colors.null).to_string(),
            Value::Bool(b) => b.to_string().color(self.colors.bool).to_string(),
            Value::Number(n) => n.to_string().color(self.colors.number).to_string(),
            Value::String(s) => format!("\"{}\"", s).color(self.colors.string).to_string(),
            Value::Array(a) => self.format_array(a, depth + 1),
            Value::Object(o) => self.format_object(o, depth + 1),
        }
    }

    fn format_array(&self, values: &[Value], depth: usize) -> String {
        let contents: Vec<String> = values
            .iter()
            .map(|v| format!("{}{}", indent(depth), self.format_json(v, depth)))
            .collect();

        format!("[\n{}\n{}]", contents.join(",\n"), indent(depth - 1))
    }

    fn format_object(&self, map: &Map<String, Value>, depth: usize) -> String {
        let contents: Vec<String> = map
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}{}: {}",
                    indent(depth),
                    k.color(self.colors.object_key),
                    self.format_json(v, depth)
                )
            })
            .collect();

        format!("{{\n{}\n{}}}", contents.join(",\n"), indent(depth - 1))
    }
}

fn indent(depth: usize) -> String {
    let spaces: Vec<&str> = (0..depth).map(|_| "  ").collect();

    spaces.join("")
}
