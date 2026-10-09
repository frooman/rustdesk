fn main() {
    #[cfg(windows)]
    {
        use std::io::Write;
        // SCTG: версия EvmDesk в метаданных SFX: из окружения CI (VERSION=1.5.0.9),
        // иначе — версия пакета сборщика (для локальных сборок).
        let ver = std::env::var("VERSION")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());
        let num = {
            let mut p = ver.split('.').map(|s| s.parse::<u64>().unwrap_or(0));
            let a = p.next().unwrap_or(0) & 0xFFFF;
            let b = p.next().unwrap_or(0) & 0xFFFF;
            let c = p.next().unwrap_or(0) & 0xFFFF;
            let d = p.next().unwrap_or(0) & 0xFFFF;
            (a << 48) | (b << 32) | (c << 16) | d
        };
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("../../res/manifest.xml")
            .set_version_info(winres::VersionInfo::FILEVERSION, num)
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, num)
            .set("FileVersion", &ver)
            .set("ProductVersion", &ver);
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}
