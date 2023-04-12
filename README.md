# A Rust CLI tool: Markdown to HTML Converter 
This is a CLI tool that converts Markdown files to HTML. I used `pulldown-cmark` crate to parse the Markdown and convert it to HTML.

## Project Setup
1. clone the repo:
```
git clone https://github.com/JuliaJHL/week11-rust-mini-proj.git
```
2. cd into the project:
```
cd week11-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run input.md output.html
```

## examples
Here is an example of `input.md`:
![input](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/week11input.png)
When you run the example with `cargo run input.md output.html`, you would get the following html file:
![output](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/week11output.png)
When you open the html file in your browser, it looks like this:
![page](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/week11page.png)


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
