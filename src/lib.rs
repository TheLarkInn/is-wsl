extern crate is_docker;
extern crate once_cell;
extern crate sys_info;

use once_cell::sync::OnceCell;

fn proc_version_includes_microsoft() -> bool {
    match std::fs::read_to_string("/proc/version") {
        Ok(file_contents) => file_contents.to_lowercase().contains("microsoft"),
        Err(_) => false,
    }
}

pub fn is_wsl() -> bool {
    static CACHED_RESULT: OnceCell<bool> = OnceCell::new();

    *CACHED_RESULT.get_or_init(|| {
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
    })
}
