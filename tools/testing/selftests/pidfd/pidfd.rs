//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/pidfd/pidfd.h
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


// SPDX-License-Identifier: GPL-2.0

//
// Remove the userspace definitions of the following preprocessor symbols
// to avoid duplicate-definition warnings from the subsequent in-kernel
// definitions.
//

pub const P_PIDFD: c_int = 3;

pub const CLONE_NEWTIME: c_uint = 0x00000080;

pub const CLONE_PIDFD: c_uint = 0x00001000;

pub const __NR_pidfd_open: c_int = 434;

pub const __NR_pidfd_send_signal: c_int = 424;

pub const __NR_clone3: c_int = 435;

pub const __NR_pidfd_getfd: c_int = 438;

pub const PIDFS_IOCTL_MAGIC: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pidfd_info {
    pub mask: __u64,
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
}

//
// The kernel reserves 300 pids via RESERVED_PIDS in kernel/pid.c
// That means, when it wraps around any pid < 300 will be skipped.
// So we need to use a pid > 300 in order to test recycling.
//
pub const PID_RECYCLE: c_int = 1000;
//
// Define a few custom error codes for the child process to clearly indicate
// what is happening. This way we can tell the difference between a system
// error, a test error, etc.
//
pub const PIDFD_PASS: c_int = 0;
pub const PIDFD_FAIL: c_int = 1;
pub const PIDFD_ERROR: c_int = 2;
pub const PIDFD_SKIP: c_int = 3;
pub const PIDFD_XFAIL: c_int = 4;
extern "C" {
    pub fn syscall(_arg: __NR_waitid, _arg: which, _arg: pid, _arg: info, _arg: options, _arg: NULL) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pidfd_open, _arg: pid, _arg: flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pidfd_send_signal, _arg: pidfd, _arg: sig, _arg: info, _arg: flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_pidfd_getfd, _arg: pidfd, _arg: fd, _arg: flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_memfd_create, _arg: name, _arg: flags) -> return;
}
extern "C" {
    pub fn sys_clone3(_arg: &args, __clone_args): sizeof(struct) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_execveat, _arg: dirfd, _arg: pathname, _arg: argv, _arg: envp, _arg: flags) -> return;
}
