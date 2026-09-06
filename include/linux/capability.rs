//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/capability.h
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
// This is <linux/capability.h>
//
// Andrew G. Morgan <morgan@kernel.org>
// Alexander Kjeldaas <astor@guardian.no>
// with help from Aleph1, Roland Buresund and Andrew Main.
//
// See here for the libcap library ("POSIX draft" compliance):
//
// ftp://www.kernel.org/pub/linux/libs/security/linux-privs/kernel-2.6
//

// same as vfs_ns_cap_data but in cpu endian and always filled completely
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_vfs_cap_data {
    pub magic_etc: __u32,
    pub rootid: kuid_t,
    pub permitted: kernel_cap_t,
    pub inheritable: kernel_cap_t,
}

//
// CAP_FS_MASK and CAP_NFSD_MASKS:
//
// The fs mask is all the privileges that fsuid==0 historically meant.
// At one time in the past, that included CAP_MKNOD and CAP_LINUX_IMMUTABLE.
//
// It has never meant setting security.* and trusted.* xattrs.
//
// We could also define fsmask as follows:
// 1. CAP_FS_MASK is the privilege to bypass all fs-related DAC permissions
// 2. The security.* and trusted.* xattrs are fs-related MAC permissions
//

//
// Check if "a" is a subset of "set".
// return true if ALL of the capabilities in "a" are also in "set"
// cap_issubset(0101, 1111) will return true
// return false if ANY of the capabilities in "a" are not in "set"
// cap_issubset(1111, 0101) will return false
//
// Used to decide between falling back on the old suser() or fsuser().
extern "C" {
    pub fn cap_drop(_arg: a, _arg: CAP_FS_SET) -> return;
}
extern "C" {
    pub fn cap_combine(_arg: a, _arg: cap_intersect(permitted, _arg: CAP_FS_SET)) -> return;
}
extern "C" {
    pub fn cap_drop(_arg: a, _arg: CAP_NFSD_SET) -> return;
}
extern "C" {
    pub fn cap_combine(_arg: a, _arg: cap_intersect(permitted, _arg: CAP_NFSD_SET)) -> return;
}

extern "C" {
    pub fn has_capability_noaudit(t: *mut task_struct, cap: c_int) -> bool;
}
extern "C" {
    pub fn capable(cap: c_int) -> bool;
}
extern "C" {
    pub fn capable_noaudit(cap: c_int) -> bool;
}
extern "C" {
    pub fn ns_capable(ns: *mut user_namespace, cap: c_int) -> bool;
}
extern "C" {
    pub fn ns_capable_noaudit(ns: *mut user_namespace, cap: c_int) -> bool;
}
extern "C" {
    pub fn ns_capable_setid(ns: *mut user_namespace, cap: c_int) -> bool;
}

extern "C" {
    pub fn file_ns_capable(file: *const file, ns: *mut user_namespace, cap: c_int) -> bool;
}
extern "C" {
    pub fn ptracer_capable(tsk: *mut task_struct, ns: *mut user_namespace) -> bool;
}
extern "C" {
    pub fn capable(capable(CAP_SYS_ADMIN: CAP_PERFMON) ||) -> return;
}
extern "C" {
    pub fn capable(capable(CAP_SYS_ADMIN: CAP_BPF) ||) -> return;
}
// audit system wants to get cap info from files as well
