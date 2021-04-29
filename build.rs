fn main() {
    cc::Build::new()
        .file("src/smooth.c")
        .compile("smooth");
}