pub mod resource {
    use std::{path, io};

    fn path(
        resource_root: path::PathBuf,
        request_path: path::PathBuf,
    ) -> Result<path::PathBuf, io::Error>
    {
        let mut p = resource_root;
        p.push(request_path);

        if p.is_dir() {
            p.push(path::PathBuf::from("index"));
            p.set_extension("html");
        }

        p.canonicalize()?;
        Ok(p)
    }
}


pub mod error {
    use std::{path, io};

    fn path(
        resource_root: path::PathBuf,
        error: &str
    ) -> path::PathBuf {

        let mut p = resource_root;
        p.push("error");
        p.with_file_name(error);
        p.set_extension("html");
        p
    }
}



