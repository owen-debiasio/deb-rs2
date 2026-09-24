use std::{fs::create_dir_all, io::Error};

use uuid::Uuid;

use crate::file::fs::{extract_ar_archive, extract_tar_archive};

pub fn extract(archive_path: &str) -> Result<String, Error> {
    // Generates a unique id to stop this from conflicting
    let output = format!("/tmp/libdeb/{}/", Uuid::new_v4());

    if cfg!(target_os = "windows") {
        // We don't support windows
        return Err(Error::other(
            "The target_os windows is not supported. Please only use on linux",
        ));
    } else {
        let data_archive = format!("{output}data.tar.xz");
        let data_extract = format!("{output}data/");

        let control_archive = format!("{output}control.tar.xz");
        let control_extract = format!("{output}control/");

        create_dir_all(&output)?;
        create_dir_all(&data_extract)?;
        create_dir_all(&control_extract)?;

        extract_ar_archive(archive_path, &output)?;

        extract_tar_archive(&data_archive, &data_extract, "xz")?;
        extract_tar_archive(&control_archive, &control_extract, "xz")?;
    }

    Ok(output)
}
