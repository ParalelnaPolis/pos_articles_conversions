extern crate configure_me_codegen;

fn main() {
    configure_me_codegen::build_script_with_man_written_to("csv2btcpay_config_spec.toml", "csv2btcpay.man").expect("Failed to generate config for gbpos2sv");
}
