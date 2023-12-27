pub const BASE_DIRECTORY: &str = "./";
pub const DRAFT_DIRECTORY: &str = "draft";
pub const CONTEXT_PATH: &str = "Context.toml";
pub const CONFIG_PATH: &str = "Config.toml";
pub const OUTPUT_PATH: &str = "./out/manuscript.docx";

pub const EXAMPLE_SCENE: &str = r#"Hi, my name is {{ names.mc }}.
"#;

pub const EXAMPLE_CONTEXT: &str = r#"[names]
mc = "Core"
"#;

pub const EXAMPLE_CONFIG: &str = r#"[project]
base_directory = "./mock"
draft_directory = "draft"
output_path = "./out/manuscript.docx"
context_path = "Context.toml"

[story]
title = "Untitled"
pen_name = "Brutus Ellis"

[author]
name = "Brutus Ellis"
address_1 = "2688 South Avenue"
address_2 = "Barangay Olympia, Makati City"
mobile_number = "+63 895 053 4757"
email_address = "brutusellis@email.com"

[agent]
name = "Cymone Sabina"
address_1 = "755 Maria Clara Street"
address_2 = "Mandaluyong City"
mobile_number = "+63 908 524 4125"
email_address = "cymonesabina.@email.com"
"#;
