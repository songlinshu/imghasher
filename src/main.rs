#[macro_use]
extern crate clap;
extern crate hex;
extern crate image;
extern crate img_hash;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use img_hash::{HashAlg, Hasher, HasherConfig};

// #[macro_use]
// mod macros;

static ALGORITHMS: [&str; 4] = ["ahash", "dct_ahash", "dhash", "dct_dhash"];

fn main() {
    let args: Vec<String> = env::args().collect();

    let matches = clap_app!(imghasher =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))

        // (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg FILE: +required +multiple "Sets the input files or directories to use")

        (@arg algorithm: -a --algo +takes_value
            default_value(&ALGORITHMS[2])
            possible_values(&ALGORITHMS)
            "Choose a hash algorithm")

        (@arg base64: -b --base64 "Output in base64")
        // (@arg debug: -d --debug ... "Sets the level of debugging information")
        (@arg force: -f --force "Do not prompt before overwriting")
        (@arg interactive: -i --interactive "Prompt before overwrite")
        (@arg quiet: -q --quiet "No output, suitable for rename mode")
        (@arg recursive: -R --recursive "Process directories recursively")
        (@arg rename: --rename "Rename the image file name to the corresponding hash")
        (@arg uppercase: -U --upper "Output in uppercase, ignored in base64 mode")
        // (@arg verbose: -v --verbose "Print test information verbosely")
    )
    .get_matches();

    let is_base64 = matches.is_present("base64");
    let is_force = matches.is_present("force");
    let is_interactive = matches.is_present("interactive");
    let is_quiet = matches.is_present("quiet");
    let is_recursive = matches.is_present("recursive");
    let is_rename = matches.is_present("rename");
    let is_uppercase = matches.is_present("uppercase");

    // let hasher = HasherConfig::new()
    //     .resize_filter(image::imageops::FilterType::Triangle)
    //     .hash_size(8, 8)
    //     .hash_alg(HashAlg::Gradient)
    //     .to_hasher();

    let config = HasherConfig::new();
    let algorithm = matches.value_of("algorithm").unwrap();

    let hasher = match &algorithm[..] {
        "ahash" => config.hash_alg(HashAlg::Mean).to_hasher(),
        "dct_ahash" => config.hash_alg(HashAlg::Mean).preproc_dct().to_hasher(),
        "dhash" => config.hash_alg(HashAlg::Gradient).to_hasher(),
        "dct_dhash" => config.hash_alg(HashAlg::Gradient).preproc_dct().to_hasher(),
        _ => config.to_hasher(),
    };

    let proc_img_file = |hasher: &Hasher, img_path: &Path, specified: bool| -> io::Result<()> {
        if !specified && image::ImageFormat::from_path(&img_path).is_err() {
            return Ok(());
        }

        let img = match image::open(&img_path) {
            Err(why) => {
                eprintln!(
                    "{}: couldn't open '{}': {}",
                    &args[0],
                    img_path.display(),
                    why
                );
                return Ok(());
            }
            Ok(data) => data,
        };

        let hash = hasher.hash_image(&img);

        let text = if is_base64 {
            hash.to_base64()
        } else if is_uppercase {
            hex::encode(hash.as_bytes()).to_uppercase()
        } else {
            hex::encode(hash.as_bytes())
        };

        if !is_quiet {
            // println!("{}  {}", &text, match path.to_str() {
            //     None => panic!("At least 1 path is not a valid UTF-8 sequence"),
            //     Some(s) => s
            // });
            println!("{}  {}", &text, img_path.display());
        }

        if is_rename {
            let parent: &Path = img_path.parent().unwrap();
            let prefix = img_path.file_stem();
            let suffix = img_path.extension();

            if let Some(prefix) = prefix {
                if OsString::from(&text) != prefix {
                    if let Some(suffix) = suffix {
                        let mut new_filename = OsString::from(text + ".");
                        new_filename.push(suffix);
                        let new_path = parent.join(Path::new(&new_filename));

                        if new_path.exists() {
                            if new_path.is_dir() {
                                eprintln!(
                                    "{}: cannot rename '{}': {}",
                                    &args[0],
                                    img_path.display(),
                                    "Dir exists"
                                );
                            } else if is_force {
                                fs::remove_file(&new_path)?;
                                fs::rename(img_path, new_path)?;
                            } else if is_interactive {
                                print!("{}: overwrite '{}'? ", &args[0], new_path.display());
                                io::stdout().flush()?;
                                let mut answer = String::new();
                                io::stdin().read_line(&mut answer)?;
                                let answer = answer.trim_end().chars();
                                if let Some(c) = answer.last() {
                                    if c == 'y' || c == 'Y' {
                                        fs::remove_file(&new_path)?;
                                        fs::rename(img_path, new_path)?;
                                    }
                                }
                            }
                        } else {
                            fs::rename(img_path, new_path)?;
                        }
                    }
                }
            }
        }

        Ok(())
    };

    fn dir_visitor(
        dir: &Path,
        recursive: bool,
        file_visitor: &dyn Fn(&fs::DirEntry),
    ) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if recursive {
                        dir_visitor(&path, recursive, file_visitor)?;
                    }
                } else {
                    file_visitor(&entry);
                }
            }
        }
        Ok(())
    }

    let proc = |hasher: &Hasher, img_path: &Path, being_chosen: bool| {
        match proc_img_file(&hasher, img_path, being_chosen) {
            Err(why) => {
                eprintln!(
                    "{}: cannot process '{}': {}",
                    &args[0],
                    img_path.display(),
                    why
                );
            }
            Ok(()) => {}
        };
    };

    let files = matches.values_of("FILE").unwrap();
    for file in files {
        let path = Path::new(file);
        if path.exists() {
            if path.is_file() {
                proc(&hasher, path, true);
            } else if path.is_dir() {
                let adapter = |entry: &fs::DirEntry| {
                    proc(&hasher, &entry.path(), false);
                };
                match dir_visitor(path, is_recursive, &adapter) {
                    Err(why) => {
                        eprintln!("{}: cannot process '{}': {}", &args[0], path.display(), why);
                    }
                    Ok(()) => {}
                };
            } else {
                eprintln!(
                    "{}: cannot process '{}': Not a file or directory",
                    &args[0],
                    path.display()
                );
            }
        } else {
            eprintln!(
                "{}: cannot access '{}': No such file or directory",
                &args[0],
                path.display()
            );
        }
    }
}
