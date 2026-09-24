use std::io::Error;

use run_script::ScriptOptions;
use uuid::Uuid;

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

        let _ = run_script::run(
            &format!(
                "
        mkdir -p {output};
        mkdir -p {data_extract};
        mkdir -p {control_extract};

        ar -x {archive_path} --output={output};
        tar -xf {data_archive} -C {data_extract};
        tar -xf {control_archive} -C {control_extract};
        "
            ),
            &vec![],
            &ScriptOptions::new(),
        );
    }

    Ok(output)
}
