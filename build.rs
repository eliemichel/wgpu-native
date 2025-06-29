use std::env;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions};
use std::io::{self, Write};
use std::process::Command;
use regex::Regex;

fn main() {
    println!("cargo:rerun-if-changed=ffi/webgpu-headers/webgpu.h");
    println!("cargo:rerun-if-changed=ffi/wgpu.h");

    let types_to_rename = vec![
        ("WGPUInstance", "WGPUInstanceImpl"),
        ("WGPUAdapter", "WGPUAdapterImpl"),
        ("WGPUSurface", "WGPUSurfaceImpl"),
        ("WGPUDevice", "WGPUDeviceImpl"),
        ("WGPUQueue", "WGPUQueueImpl"),
        ("WGPUBuffer", "WGPUBufferImpl"),
        ("WGPUTextureView", "WGPUTextureViewImpl"),
        ("WGPUTexture", "WGPUTextureImpl"),
        ("WGPUSampler", "WGPUSamplerImpl"),
        ("WGPUBindGroupLayout", "WGPUBindGroupLayoutImpl"),
        ("WGPUPipelineLayout", "WGPUPipelineLayoutImpl"),
        ("WGPUBindGroup", "WGPUBindGroupImpl"),
        ("WGPUShaderModule", "WGPUShaderModuleImpl"),
        ("WGPURenderPipeline", "WGPURenderPipelineImpl"),
        ("WGPUComputePipeline", "WGPUComputePipelineImpl"),
        ("WGPUCommandEncoder", "WGPUCommandEncoderImpl"),
        ("WGPUCommandBuffer", "WGPUCommandBufferImpl"),
        ("WGPURenderPassEncoder", "WGPURenderPassEncoderImpl"),
        ("WGPUComputePassEncoder", "WGPUComputePassEncoderImpl"),
        ("WGPURenderBundleEncoder", "WGPURenderBundleEncoderImpl"),
        ("WGPURenderBundle", "WGPURenderBundleImpl"),
        ("WGPUQuerySet", "WGPUQuerySetImpl"),
        ("WGPUSwapChain", "WGPUSwapChainImpl"),
    ];
    let mut builder = bindgen::Builder::default()
        .header("ffi/webgpu-headers/webgpu.h")
        .header("ffi/wgpu.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_function("wgpuGetProcAddress")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .ignore_functions()
        .layout_tests(true);

    for (old_name, new_name) in types_to_rename {
        let line = format!("pub type {old_name} = *mut {new_name};");
        builder = builder
            .blocklist_type(old_name)
            .blocklist_type(format!("{old_name}Impl"))
            .raw_line(line);
    }

    // See https://github.com/rust-lang/rust-bindgen/issues/1780
    if let Ok("ios") = env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        let output = Command::new("xcrun")
            .args(["--sdk", "iphoneos", "--show-sdk-path"])
            .output()
            .expect("xcrun failed")
            .stdout;
        let sdk = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
        builder = builder
            .clang_arg(format!("-isysroot {sdk}"))
            .clang_arg("--target=arm64-apple-ios");
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Extra builder, to extract constants that bindgen fails to parse
    let extra_builder = ExtraBuilder::default()
        .header("ffi/webgpu-headers/webgpu.h")
        .header("ffi/wgpu.h");
    let extra_bindings = extra_builder.generate().expect("Unable to generate bindings");
    extra_bindings
        .write_to_file(out_path.join("extra_bindings.rs"))
        .expect("Couldn't write extra bindings!");
}

/// A complement to bindgen::Builder that extracts #define constants that are
/// not correctly extracted by bindgen. 
#[derive(Debug, Default, Clone)]
struct ExtraBuilder {
    input_headers: Vec<String>
}

impl ExtraBuilder {
    pub fn header<T: Into<String>>(mut self, header: T) -> ExtraBuilder {
        self.input_headers.push(header.into());
        self
    }

    pub fn generate(self) -> Result<ExtraBindings, bindgen::BindgenError> {
        let mut bindings = ExtraBindings::default();

        for h in self.input_headers {
            // Check path
            let path = Path::new(&h);
            if let Ok(md) = std::fs::metadata(path) {
                if md.is_dir() {
                    return Err(bindgen::BindgenError::FolderAsHeader(
                        path.into(),
                    ));
                }
            } else {
                return Err(bindgen::BindgenError::NotExist(
                    path.into(),
                ));
            }

            Self::process_header(path, &mut bindings)
                .map_err(|_err| bindgen::BindgenError::InsufficientPermissions(
                    path.into(),
                ))?;
        }
        return Ok(bindings)
    }

    fn process_header(path: &Path, bindings: &mut ExtraBindings) -> std::io::Result<()> {
        let define_re = Regex::new(r"#define WGPU_([A-Z0-9_]+) \(([A-Z0-9_()]+)\)").unwrap();
        
        let file_contents = std::fs::read_to_string(path)?;
        for line in file_contents.lines() {
            if let Some(caps) = define_re.captures(line) {
                let const_name = caps.get(1).map_or("", |m| m.as_str());
                let const_raw_value = caps.get(2).map_or("", |m| m.as_str());
                //let const_type = "std::os::raw::c_uint";
                //let const_value = "std::os::raw::c_uint::MAX";
                if let Some((const_type, const_value)) = match const_raw_value {
                    "UINT32_MAX" => Some(("std::os::raw::c_uint", "std::os::raw::c_uint::MAX")),
                    "UINT64_MAX" => Some(("std::os::raw::c_ulonglong", "std::os::raw::c_ulonglong::MAX")),
                    "SIZE_MAX" => Some(("std::os::raw::c_ulong", "std::os::raw::c_ulong::MAX")),
                    "NAN" => Some(("f32", "f32::NAN")),
                    "UINT32_C(0)" => Some(("u32", "0")),
                    "UINT32_C(1)" => Some(("u32", "1")),
                    _ => None,
                } {
                    bindings.contents.push(format!("pub const WGPU_{}: {} = {};\n", const_name, const_type, const_value));
                }
            }
        }
        Ok(())
    }
}

/// Result of ExtraBuilder
#[derive(Debug, Default, Clone)]
struct ExtraBindings {
    contents: Vec<String>
}

impl ExtraBindings {
    /// Write these bindings as source text to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_ref())?;
        self.write(Box::new(file))?;
        Ok(())
    }

    /// Write these bindings as source text to the given `Write`able.
    pub fn write<'a>(&self, mut writer: Box<dyn Write + 'a>) -> io::Result<()> {
        for line in self.contents.iter().by_ref() {
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }
}
