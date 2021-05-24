use crate::run;
use crate::get_stdout;
use Result;

/// Returns the wallpaper of XFCE.
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    get_stdout(
        "xfconf-query",
        &[
            "-c",
            "xfce4-desktop",
            "-p",
            "/backdrop/screen0/monitor0/last-image",
        ],
    )
}

/// Sets the wallpaper for XFCE.
pub fn set(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    run(
        "xfconf-query",
        &[
            "-c",
            "xfce4-desktop",
            "-p",
            "/backdrop/screen0/monitor0/last-image",
            "-s",
            &path,
        ],
    )
}

#[test]
fn test_get_wallpaper(){
    dbg!(crate::linux::xfce::get().unwrap());
}
