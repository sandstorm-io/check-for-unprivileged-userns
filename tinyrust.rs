#![crate_type="rlib"]
#![feature(core)]

#[macro_use] extern crate syscall;

use std::{mem, raw, intrinsics};

fn exit(n: usize) -> ! {
    unsafe {
        syscall!(EXIT, n);
        intrinsics::unreachable()
    }
}

fn write(fd: usize, buf: &[u8]) -> usize {
    unsafe {
        return syscall!(WRITE, fd, buf.as_ptr(), buf.len())
    }
}

fn getuid() -> usize {
    unsafe {
        return syscall!(GETUID)
    }
}

fn setuid(n: usize) -> usize {
    unsafe {
        return syscall!(SETUID, n)
    }
}

fn unshare(n: usize) -> usize {
    unsafe {
        return syscall!(UNSHARE, n)
    }
}

#[no_mangle]
pub fn main() {
    // The purpose of main() is to verify that UID namespaces work
    // when not root, so we make sure we're not root.

    // We will:
    //
    // - exit(0) if we confirm user namespaces work great as non-root.
    //
    // - exit non-zero if anything else happens.

    // Since I started this codebase by forking Keegan McAllister's
    // tinyrust project, it comes bundled with a string constant
    // ("Hello!\n"). We capture that message here, not because we use
    // it in this program, but because while modifying this code, it
    // can be helpful to be able to write() it for debugging's sake.

    // Make a Rust value representing the string constant we stashed
    // in the ELF file header.
    let message: &'static [u8] = unsafe {
        mem::transmute(raw::Slice {
            data: 0x00400008 as *const u8,
            len: 7,
        })
    };

    // First, check if we are root. If we are, we setuid to 65534
    // (which is just a dummy non-root user ID, which also happens to
    // usually be "nobody"), since we want to find out if
    // unshare(CLONE_NEWUSER) works when we are not root.

    let current_uid = getuid();
    if (current_uid == 0) {
        let setuid_result = setuid(65534);
        if (setuid_result != 0) {
            // Hmm, we failed to setuid properly. Abort now.
            exit(2);
        }
    }

    // Now that we're not root, we can try to unshare(CLONE_NEWUSER).
    // CLONE_NEWUSER is 0x10000000.
    //
    // We pass unshare()'s return code directly to exit(). It exits 0
    // in the case of success! And if the kernel gives us a non-zero
    // exit code, then we exit non-zero.
    //
    // Note that if we overflow, we'll be a little sad. Oh, well.
    exit(unshare(0x10000000));
}
