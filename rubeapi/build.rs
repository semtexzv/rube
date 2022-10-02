fn main() -> protokit_codegen::Result<()> {
    protokit_codegen::Codegen::new()
        .include("../proto/")
        .out_dir("src/gen")
        .compile("rube/api.proto")?
        .compile("rube/util.proto")?
        .compile("rube/systemd/unit.proto")?
        .generate()
}
