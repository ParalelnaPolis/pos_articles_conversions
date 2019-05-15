extern crate configure_me_codegen;

fn main() {
    configure_me_codegen::build_script_with_man_written_to("gb2csv_config_spec.toml", "gbpos2csv.man").expect("Failed to generate config for gbpos2sv");
}
