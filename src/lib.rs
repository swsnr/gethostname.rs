// Copyright 2019  Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

//! [gethostname()][ghn] for all platforms.
//!
//! [ghn]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html

#![deny(warnings, missing_docs, clippy::all)]

use std::ffi::OsString;

/// Get the standard host name for the current machine.
///
/// Wraps POSIX [gethostname] in a safe interface. The function doesn’t fail but
/// it may `panic!` if the internal buffer for the hostname is too small, but we
/// use a buffer large enough to hold the maximum hostname, so we consider any
/// panics from this function as bug which you should report.
///
/// [gethostname]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
#[cfg(not(windows))]
pub fn gethostname() -> OsString {
    use libc::{c_char, sysconf, _SC_HOST_NAME_MAX};
    use std::os::unix::ffi::OsStringExt;
    // Get the maximum size of host names on this system, and account for the
    // trailing NUL byte.
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
    let mut buffer = vec![0 as u8; (hostname_max as usize) + 1];
    let returncode = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };
    if returncode != 0 {
        // There are no reasonable failures, so lets panic
        panic!("gethostname failed!  Please report an issue to <https://github.com/lunaryorn/gethostname.rs/issues>!");
    }
    // We explicitly search for the trailing NUL byte and cap at the buffer
    // length: If the buffer's too small (which shouldn't happen since we
    // explicitly use the max hostname size above but just in case) POSIX
    // doesn't specify whether there's a NUL byte at the end, so if we didn't
    // check we might read from memory that's not ours.
    let end = buffer
        .iter()
        .position(|&b| b == 0)
        .unwrap_or_else(|| buffer.len());
    OsString::from_vec(buffer[0..end].to_vec())
}

#[cfg(windows)]
pub fn gethostname() -> OsString {
    use std::os::windows::ffi::OsStringExt;
    use winapi::ctypes::{c_ulong, wchar_t};
    use winapi::um::sysinfoapi::{ComputerNamePhysicalDnsHostname, GetComputerNameExW};

    let mut buffer_size: c_ulong = 0;

    unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            std::ptr::null(),
            &mut buffer_size,
        )
    };

    let mut buffer = vec![0 as wchar_t; buffer_size as usize];

    unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            buffer.as_mut_ptr() as *mut wchar_t,
            &mut buffer_size,
        )
    };

    OsString::from_wide(buffer)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::process::Command;

    #[test]
    fn gethostname_matches_system_hostname() {
        let output = Command::new("hostname")
            .output()
            .expect("failed to get hostname");
        let hostname = String::from_utf8_lossy(&output.stdout);
        // Convert both sides to lowercase; hostnames are case-insensitive
        // anyway.
        assert_eq!(
            super::gethostname().into_string().unwrap().to_lowercase(),
            hostname.trim_end().to_lowercase()
        );
    }
}
