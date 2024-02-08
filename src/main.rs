use std::fs;
use std::io;


fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <2 {
        println!("Usage: {} <filename>", args[0]); // if you forget to pass an argument you'll find this error
        return 1;
    }

    let fname = std::path::Path::new(&args[1]); //file name - fname, we got the name of the file
    let file = fs::File::open(&fname).unwrap(); // we open the file
    let mut archive = zip::ZipArchive::new(file).unwrap(); // create a mutable archive which will help us to process that file

    for i in 0..archive.len() { // the zip file will have many files. So we tell it go through all those files or folders
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment(); 
            if !comment.is_empty(){ //check for comments
                println!("File {} comment:{}", i, comment);
            }
        }

        // this is kinda optional. It helps to keep the structure of the folder as it was in the zip
        // check if it's a folder or file
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // here is where we are copying the file
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // set permissions to the files
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0 // the function is supposed to return something, that's why I'm using 0
}

//cargo run
// cargo run 1com.zip