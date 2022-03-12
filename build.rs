extern crate bindgen;

use std::path::{Path, PathBuf};

fn main() {
    let erlang_dir = get_erlang_directory().expect("Couldn't find erlang directory");

    fn get_path_str(base: &PathBuf, path_parts: Vec<&str>) -> String {
        let mut path = base.clone();
        path_parts.iter().for_each(|&path_part|path = path.join(path_part));
        if !path.exists() {
            panic!("Path {:?} does not exist", path);
        }
        return String::from(path.to_str().unwrap());
    }

    let erlang_clib_dir = get_path_str(&erlang_dir, ["usr", "lib"].into());
    let erlang_cinclude_dir = get_path_str(&erlang_dir, ["usr", "include"].into());

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    std::fs::read_dir(&erlang_cinclude_dir).unwrap()
        .map(|res|res.ok())
        .filter(|entry| {
            if entry.is_none() {
                return false;
            };
            let path = entry.as_ref().unwrap().path();
            let path_ext = 
                path.extension().unwrap_or("".as_ref())
                    .to_str().unwrap();
            let path_name = 
                path.file_name().unwrap_or("".as_ref())
                    .to_str().unwrap()
                    .strip_suffix(&format!(".{}", path_ext)).unwrap();
            let keep = path_ext.eq("h") && path_name != "erl_nif_api_funcs";
            return keep
        })
        .for_each(|entry| {
            let entry = entry.unwrap();
            let header_filename = {
                let os_file_name = entry.file_name();
                let file_name = os_file_name.to_str()
                    .expect(&format!("Couldn't convert file_name {:?} to string", os_file_name))
                    .strip_suffix(".h")
                    .unwrap();
                String::from(file_name)
            };

            let header_filepath = 
                Path::new(&erlang_cinclude_dir)
                    .join(format!("{}.h", &header_filename));

            let bindings = bindgen::Builder::default()
                .clang_arg(format!("-I{}", &erlang_cinclude_dir))
                .clang_arg(format!("-L{}", &erlang_clib_dir))
                .header(header_filepath.to_str().unwrap())
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate_comments(true)
                .generate_block(true)
                .rustfmt_bindings(true)
                .size_t_is_usize(true)
                .generate()
                .expect("Unable to generate bindings");
            
            let output_filepath = out_path.join(format!("{}.rs", &header_filename));

            bindings
                .write_to_file(&output_filepath)
                .expect("Couldn't write bindings!");

            if !(&output_filepath).exists() {
                panic!("FAILED TO WRITE TO {:?}!!", &output_filepath)
            }
        })
}

fn get_erlang_directory() -> Option<PathBuf> {
    let env = std::env::var("ERL_DIR").ok();
    return match env {
        Some(erl_dir) => {
            let path = Path::new(&erl_dir);
            match std::fs::canonicalize(path).ok() {
                None => None,
                Some(path) => {
                    let prefix = "\\\\?\\";
                    let path = 
                        path.to_str().unwrap()
                            .strip_prefix(prefix)
                            .expect(&format!("Failed to strip '{}' from {:?}", prefix, &path));
                    Some(PathBuf::from(path))
                }
            }
        },
        None => {
            let erl_path = which::which("erl");
            if erl_path.is_err() {
                return None
            }
            let path = erl_path.ok()?;
            let erl_dir =
                path        /* ../bin/erl  */
                .parent()?  /* ../bin  */
                .parent()?; /* ..  */
            std::fs::canonicalize(erl_dir).ok()
        }
    };
}