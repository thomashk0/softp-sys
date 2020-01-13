pub fn main() {
    cc::Build::new()
        .files(&["softfp/softfp.c"])
        .compile("softfp");
}
