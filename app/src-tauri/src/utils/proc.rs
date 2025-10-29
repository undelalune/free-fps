// Apply "no console window" flags on Windows. No-op on other OS.
pub fn apply_no_window_std(cmd: &mut std::process::Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt as _;
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    {
        let _ = cmd;
    }
}

/// Apply "no console window" flags on Windows for tokio::process::Command. No-op on other OS.
pub fn apply_no_window_tokio(cmd: &mut tokio::process::Command) {
    #[cfg(windows)]
    {
        // tokio::process::Command has an inherent `creation_flags` method on Windows.
        use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    {
        let _ = cmd;
    }
}
