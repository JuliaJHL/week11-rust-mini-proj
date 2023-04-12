use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::{Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "md2html", about = "A simple Markdown to HTML converter.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let mut input_file = File::open(opt.input)?;
    let mut input_text = String::new();
    input_file.read_to_string(&mut input_text)?;

    let mut output_file = File::create(opt.output)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&input_text, options);
    html::write_html(&mut output_file, parser)?;

    Ok(())
}
