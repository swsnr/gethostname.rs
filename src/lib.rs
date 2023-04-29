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
//! ```
//! use gethostname::gethostname;
//!
//! println!("Hostname: {:?}", gethostname());
//! ```
//!
//! [ghn]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html

#![deny(warnings, missing_docs, clippy::all)]

use std::ffi::OsString;

/// Get the standard host name for the current machine.
///
/// On Unix simply wrap POSIX [gethostname] in a safe interface.  On Windows
/// return the DNS host name of the local computer, as returned by
/// [GetComputerNameExW] with `ComputerNamePhysicalDnsHostname` as `NameType`.
///
/// This function panics if the buffer allocated for the hostname result of the
/// operating system is too small; however we take great care to allocate a
/// buffer of sufficient size:
///
/// * On Unix we allocate the buffer using the maximum permitted hostname size,
///     as returned by [sysconf] via `sysconf(_SC_HOST_NAME_MAX)`, plus an extra
///     byte for the trailing NUL byte.  A hostname cannot exceed this limit, so
///     this function can't realistically panic.
/// * On Windows we call `GetComputerNameExW` with a NULL buffer first, which
///     makes it return the length of the current host name.  We then use this
///     length to allocate a buffer for the actual result; this leaves a tiny
///     tiny race condition in case the hostname changes to a longer name right
///     in between those two calls but that's a risk we don't consider of any
///     practical relevance.
///
/// Hence _if_ this function does panic please [report an issue][new].
///
/// [gethostname]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
/// [sysconf]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html
/// [GetComputerNameExW]: https://docs.microsoft.com/en-us/windows/desktop/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
/// [new]: https://github.com/swsnr/gethostname.rs/issues/new
pub fn gethostname() -> OsString {
    gethostname_impl()
}

#[cfg(unix)]
#[inline]
fn gethostname_impl() -> OsString {
    use std::os::unix::ffi::OsStringExt;

    // libc::gethostname() is implemented in both musl and glibc by calling uname() and copying
    // the nodename field into the buffer.
    //
    // glibc:
    // https://github.com/lattera/glibc/blob/895ef79e04a953cac1493863bcae29ad85657ee1/sysdeps/posix/gethostname.c#L32-L36
    // musl:
    // https://github.com/cloudius-systems/musl/blob/00733dd1cf791d13ff6155509cf139a5f7b2eecb/src/unistd/gethostname.c#L4-L13
    //
    // On FreeBSD, libc::gethostname() is a sysctl call to KERN_HOSTNAME. uname() fetches the nodename
    // using the same strategy.
    //
    // freebsd gethostname:
    // https://github.com/FreeBSDDesktop/freebsd-base/blob/de1aa3dab23c06fec962a14da3e7b4755c5880cf/lib/libc/gen/gethostname.c#L47-L54
    // freebsd uname:
    // https://github.com/FreeBSDDesktop/freebsd-base/blob/de1aa3dab23c06fec962a14da3e7b4755c5880cf/lib/libc/gen/__xuname.c#L73-L83
    //
    // OpenBSD uses the same strategy, so I doubt that other implementations of uname() among the BSDs
    // stray too far from this pattern.
    let bytes = rustix::process::uname().nodename().to_bytes().to_vec();
    OsString::from_vec(bytes)
}

#[cfg(windows)]
#[inline]
fn gethostname_impl() -> OsString {
    use std::os::windows::ffi::OsStringExt;
    use windows::core::PWSTR;
    use windows::Win32::System::SystemInformation::{
        ComputerNamePhysicalDnsHostname, GetComputerNameExW,
    };

    let mut buffer_size: u32 = 0;

    unsafe {
        // This call always fails with ERROR_MORE_DATA, because we pass NULL to
        // get the required buffer size.  GetComputerNameExW then fills buffer_size with the size
        // of the host name string plus a trailing zero byte.
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::null(),
            &mut buffer_size,
        )
    };
    assert!(
        0 < buffer_size,
        "GetComputerNameExW did not provide buffer size"
    );

    let mut buffer = vec![0_u16; buffer_size as usize];
    unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            PWSTR::from_raw(buffer.as_mut_ptr()),
            &mut buffer_size,
        )
        .expect(
            "GetComputerNameExW failed to read hostname.
        Please report this issue to <https://github.com/swsnr/gethostname.rs/issues>!",
        )
    }
    assert!(
        // GetComputerNameExW returns the size _without_ the trailing zero byte on the second call
        buffer_size as usize == buffer.len() - 1,
        "GetComputerNameExW changed the buffer size unexpectedly"
    );

    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[0..end])
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn gethostname_matches_system_hostname() {
        let output = Command::new("hostname")
            .output()
            .expect("failed to get hostname");
        if output.status.success() {
            let hostname = String::from_utf8_lossy(&output.stdout);
            assert!(
                !hostname.is_empty(),
                "Failed to get hostname: hostname empty?"
            );
            // Convert both sides to lowercase; hostnames are case-insensitive
            // anyway.
            assert_eq!(
                super::gethostname().into_string().unwrap().to_lowercase(),
                hostname.trim_end().to_lowercase()
            );
        } else {
            panic!(
                "Failed to get hostname! {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[test]
    #[ignore]
    fn gethostname_matches_fixed_hostname() {
        assert_eq!(
            super::gethostname().into_string().unwrap().to_lowercase(),
            "hostname-for-testing"
        );
    }
}
