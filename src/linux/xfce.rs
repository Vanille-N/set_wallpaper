use crate::get_stdout;
use crate::run;
use Result;

/// Returns the wallpaper of XFCE.
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    let mut ret = String::new();
    for xfce_path in get_last_image_paths()? {
        ret = format!(
            "{},{}",
            ret,
            get_stdout("xfconf-query", &["-c", "xfce4-desktop", "-p", &xfce_path])?
        )
    }
    Ok(ret)
}

/// Sets the wallpaper for XFCE.
pub fn set(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    for xfce_path in dbg!(get_last_image_paths()?) {
        run(
            "xfconf-query",
            &["-c", "xfce4-desktop", "-p", &xfce_path, "-s", &path],
        )?
    }
    Ok(())
}

fn get_last_image_paths() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let str = get_stdout("xfconf-query", &["-l", "-c", "xfce4-desktop"])?;
    Ok(str
        .split("\n")
        .filter(|s| s.ends_with("/last-image"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}

#[test]
fn test_get_wallpaper() {
    dbg!(crate::linux::xfce::get().unwrap());
}
