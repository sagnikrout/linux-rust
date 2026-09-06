//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pidfd.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Flags for pidfd_open().

// Flags for pidfd_send_signal().

// Flags for pidfd_info.

//
// Values for @coredump_mask in pidfd_info.
// Only valid if PIDFD_INFO_COREDUMP is set in @mask.
//
// Note, the @PIDFD_COREDUMP_ROOT flag indicates that the generated
// coredump should be treated as sensitive and access should only be
// granted to privileged users.
//

//
// ...and for userland we make life simpler - PIDFD_SELF refers to the current
// thread, PIDFD_SELF_PROCESS refers to the process thread group leader.
//
// For nearly all practical uses, a user will want to use PIDFD_SELF.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pidfd_info {
//
// This mask is similar to the request_mask in statx(2).
//
// Userspace indicates what extensions or expensive-to-calculate fields
// they want by setting the corresponding bits in mask. The kernel
// will ignore bits that it does not know about.
//
// When filling the structure, the kernel will only set bits
// corresponding to the fields that were actually filled by the kernel.
// This also includes any future extensions that might be automatically
// filled. If the structure size is too small to contain a field
// (requested or not), to avoid confusion the mask will not
// contain a bit for that field.
//
// As such, userspace MUST verify that mask contains the
// corresponding flags after the ioctl(2) returns to ensure that it is
// using valid data.
//
    pub mask: __u64,
//
// The information contained in the following fields might be stale at the
// time it is received, as the target process might have exited as soon as
// the IOCTL was processed, and there is no way to avoid that. However, it
// is guaranteed that if the call was successful, then the information was
// correct and referred to the intended process at the time the work was
// performed.
    pub cgroupid: __u64,
    pub pid: __u32,
    pub tgid: __u32,
    pub ppid: __u32,
    pub ruid: __u32,
    pub rgid: __u32,
    pub euid: __u32,
    pub egid: __u32,
    pub suid: __u32,
    pub sgid: __u32,
    pub fsuid: __u32,
    pub fsgid: __u32,
    pub exit_code: __s32,
    pub coredump_mask: __u32,
    pub coredump_signal: __u32,
    pub coredump_code: __u32,
    pub /: *mut *mut __u32 coredump_pad; / align supported_mask to 8 bytes,
}

pub const PIDFS_IOCTL_MAGIC: c_uint = 0xFF;

