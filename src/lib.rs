extern crate is_docker;
extern crate sys_info;

fn proc_version_includes_microsoft() -> bool {
    match std::fs::read_to_string("/proc/version") {
        Ok(file_contents) => file_contents.to_lowercase().contains("microsoft"),
        Err(_) => false,
    }
}

pub fn is_wsl() -> bool {
    if std::env::consts::OS != "linux" {
        return false;
    }

    if sys_info::os_release().is_ok()
        && sys_info::os_release()
            .unwrap()
            .to_lowercase()
            .contains("microsoft")
    {
        if is_docker::is_docker() {
            return false;
        }

        return true;
    }

    return if proc_version_includes_microsoft() {
        !is_docker::is_docker()
    } else {
        false
    };
}
