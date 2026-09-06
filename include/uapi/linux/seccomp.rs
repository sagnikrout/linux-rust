//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/seccomp.h
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

// Valid values for seccomp.mode and prctl(PR_SET_SECCOMP, <mode>)

// Valid operations for seccomp syscall.
pub const SECCOMP_SET_MODE_STRICT: c_int = 0;
pub const SECCOMP_SET_MODE_FILTER: c_int = 1;
pub const SECCOMP_GET_ACTION_AVAIL: c_int = 2;
pub const SECCOMP_GET_NOTIF_SIZES: c_int = 3;
// Valid flags for SECCOMP_SET_MODE_FILTER

// Received notifications wait in killable state (only respond to fatal signals)

//
// All BPF programs must return a 32-bit value.
// The bottom 16-bits are for optional return data.
// The upper 16-bits are ordered from least permissive values to most,
// as a signed value (so 0x8000000 is negative).
//
// The ordering ensures that a min_t() over composed return values always
// selects the least permissive choice.
//
pub const SECCOMP_RET_KILL_PROCESS: c_uint = 0x80000000U /* kill the process */;
pub const SECCOMP_RET_KILL_THREAD: c_uint = 0x00000000U /* kill the thread */;

pub const SECCOMP_RET_TRAP: c_uint = 0x00030000U /* disallow and force a SIGSYS */;
pub const SECCOMP_RET_ERRNO: c_uint = 0x00050000U /* returns an errno */;
pub const SECCOMP_RET_USER_NOTIF: c_uint = 0x7fc00000U /* notifies userspace */;
pub const SECCOMP_RET_TRACE: c_uint = 0x7ff00000U /* pass to a tracer or disallow */;
pub const SECCOMP_RET_LOG: c_uint = 0x7ffc0000U /* allow after logging */;
pub const SECCOMP_RET_ALLOW: c_uint = 0x7fff0000U /* allow */;
// Masks for the return value sections.
pub const SECCOMP_RET_ACTION_FULL: c_uint = 0xffff0000U;
pub const SECCOMP_RET_ACTION: c_uint = 0x7fff0000U;
pub const SECCOMP_RET_DATA: c_uint = 0x0000ffffU;
//
// struct seccomp_data - the format the BPF program executes over.
// @nr: the system call number
// @arch: indicates system call convention as an AUDIT_ARCH_* value
// as defined in <linux/audit.h>.
// @instruction_pointer: at the time of the system call.
// @args: up to 6 system call arguments always stored as 64-bit values
// regardless of the architecture.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_data {
    pub nr: c_int,
    pub arch: __u32,
    pub instruction_pointer: __u64,
    pub args: [__u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif {
    pub id: __u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}

//
// Valid flags for struct seccomp_notif_resp
//
// Note, the SECCOMP_USER_NOTIF_FLAG_CONTINUE flag must be used with caution!
// If set by the process supervising the syscalls of another process the
// syscall will continue. This is problematic because of an inherent TOCTOU.
// An attacker can exploit the time while the supervised process is waiting on
// a response from the supervising process to rewrite syscall arguments which
// are passed as pointers of the intercepted syscall.
// It should be absolutely clear that this means that the seccomp notifier
// _cannot_ be used to implement a security policy! It should only ever be used
// in scenarios where a more privileged process supervises the syscalls of a
// lesser privileged process to get around kernel-enforced security
// restrictions when the privileged process deems this safe. In other words,
// in order to continue a syscall the supervising process should be sure that
// another security mechanism or the kernel itself will sufficiently block
// syscalls if arguments are rewritten to something unsafe.
//
// Similar precautions should be applied when stacking SECCOMP_RET_USER_NOTIF
// or SECCOMP_RET_TRACE. For SECCOMP_RET_USER_NOTIF filters acting on the
// same syscall, the most recently added filter takes precedence. This means
// that the new SECCOMP_RET_USER_NOTIF filter can override any
// SECCOMP_IOCTL_NOTIF_SEND from earlier filters, essentially allowing all
// such filtered syscalls to be executed by sending the response
// SECCOMP_USER_NOTIF_FLAG_CONTINUE. Note that SECCOMP_RET_TRACE can equally
// be overriden by SECCOMP_USER_NOTIF_FLAG_CONTINUE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_resp {
    pub id: __u64,
    pub val: __s64,
    pub error: __s32,
    pub flags: __u32,
}

// valid flags for seccomp_notif_addfd

//
// struct seccomp_notif_addfd
// @id: The ID of the seccomp notification
// @flags: SECCOMP_ADDFD_FLAG_
// @srcfd: The local fd number
// @newfd: Optional remote FD number if SETFD option is set, otherwise 0.
// @newfd_flags: The O_* flags the remote FD should have applied
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_notif_addfd {
    pub id: __u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}

// Flags for seccomp notification fd ioctl.

// On success, the return value is the remote process's added fd number

