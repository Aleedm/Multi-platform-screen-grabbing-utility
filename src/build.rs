fn main() {
    // actions
    glib_build_tools::compile_resources(
        &["sresources"],
        "resources/resources.gresource.xml",
        "compiled.gresource",
    );
}