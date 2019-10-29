use dxplr::d3d;
use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};

#[cfg(feature = "d3dcompiler")]
struct Include<'a> {
    current_dir: &'a Path,
}
#[cfg(feature = "d3dcompiler")]
impl<'a> Include<'a> {
    fn new(current_dir: &'a Path) -> Self {
        Self {
            current_dir,
        }
    }
}
#[cfg(feature = "d3dcompiler")]
impl<'a> d3d::IInclude for Include<'a> {
    fn open(&self, include_type: d3d::IncludeType, filename: &str, _parent_data: Option<*const std::ffi::c_void>, data: &mut Vec<u8>) -> std::io::Result<()> {
        let file = match include_type {
            d3d::IncludeType::Local => {
                File::open(self.current_dir.join(filename)).unwrap()
            },
            d3d::IncludeType::System => {
                let dir = Path::new("tests/hlsl");
                File::open(dir.join(filename)).unwrap()
            },
        };
        let mut reader = BufReader::new(file);
        reader.read_to_end(data).unwrap();
        Ok(())
    }
}

#[cfg(feature = "d3dcompiler")]
#[test]
fn d3dcompile_custom_include() {
    let file = File::open("tests/hlsl/simple.hlsl").unwrap();
    let mut reader = BufReader::new(file);
    let mut data = Vec::new();
    reader.read_to_end(&mut data).unwrap();
    let include = Include::new(Path::new("tests/hlsl"));
    let res = d3d::compile(
        &data,
        Some("simple.hlsl"),
        None,
        Some(Box::new(include)),
        "vs_main",
        "vs_5_0",
        Some(d3d::CompileFlags::Debug),
        None
    );
    if let Err(e) = res {
        panic!("d3d::compile error = {}", e);
    }
}
