use colored::{Color, Colorize};
use serde_json::{Error, Map, Value};
use structopt::StructOpt;
use std::{io, process::exit};

/// Pretty print a stream of json logs.
#[derive(Debug, StructOpt)]
struct Opt {

    /// Color scheme to use [chalk, greyscale, ocean, solarized]
    #[structopt(long, default_value = "chalk")]
    color_scheme: String,

}

fn main() {
    let opt = Opt::from_args();
    if let Some(color_palette) = ColorPalette::from_str(&opt.color_scheme) {
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
        println!("{}", format!("Unknown color scheme: {}", opt.color_scheme).red());
        exit(1);
    }
}

struct ColorPalette {
    null: Color,
    bool: Color,
    number: Color,
    string: Color,
    object_key: Color,
}

impl ColorPalette {
    fn from_str(palette: &str) -> Option<Self> {
        match palette {
            "ocean" => Some(Self::ocean()),
            "chalk" => Some(Self::chalk()),
            "greyscale" => Some(Self::greyscale()),
            "solarized" => Some(Self::solarized()),
            _ => None,
        }
    }

    fn chalk() -> Self {
        Self {
            null: Color::TrueColor {r: 0xdd, g: 0xb2, b: 0x6f},
            bool: Color::TrueColor {r: 0xe1, g: 0xa3, b: 0xee},
            number: Color::TrueColor {r: 0x6f, g: 0xc2, b: 0xef},
            string: Color::TrueColor {r: 0x12, g: 0xcf, b: 0xc0},
            object_key: Color::TrueColor {r: 0xfb, g: 0x9f, b: 0xb1},
        }
    }

    fn ocean() -> Self {
        Self {
            null: Color::TrueColor {r: 0xbf, g: 0x61, b: 0x6a},
            bool: Color::TrueColor {r: 0xeb, g: 0xec, b: 0x8b},
            number: Color::TrueColor {r: 0xa3, g: 0xbe, b: 0x8c},
            string: Color::TrueColor {r: 0x8f, g: 0xa1, b: 0xb3},
            object_key: Color::TrueColor {r: 0xb4, g: 0x8e, b: 0xad},
        }
    }

    fn greyscale() -> Self {
        Self {
            null: Color::TrueColor {r: 0xb9, g: 0xb9, b: 0xb9},
            bool: Color::TrueColor {r: 0xf7, g: 0xf7, b: 0xf7},
            number: Color::TrueColor {r: 0x86, g: 0x86, b: 0x86},
            string: Color::TrueColor {r: 0xa0, g: 0xa0, b: 0xa0},
            object_key: Color::TrueColor {r: 0x68, g: 0x68, b: 0x68},
        }
    }

    fn solarized() -> Self {
        Self {
            null: Color::TrueColor {r: 0xdc, g: 0x32, b: 0x2f},
            bool: Color::TrueColor {r: 0xb5, g: 0x89, b: 0x00},
            number: Color::TrueColor {r: 0x6c, g: 0x71, b: 0xc4},
            string: Color::TrueColor {r: 0x26, g: 0x8b, b: 0xd2},
            object_key: Color::TrueColor {r: 0x2a, g: 0xa1, b: 0x98},
        }

    }
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

    fn format_array(&self, values: &Vec<Value>, depth: usize) -> String {
        let contents: Vec<String> = values
            .iter()
            .map(|v| {
                format!(
                    "{}{}",
                    indent(depth),
                    self.format_json(v, depth)
                )
            })
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
