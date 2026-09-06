//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/filesystems/wrappers.h
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

pub const STATX_MNT_ID_UNIQUE: c_uint = 0x00004000U /* Want/got extended stx_mount_id */;

extern "C" {
    pub fn syscall(_arg: __NR_fsopen, _arg: fsname, _arg: flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_fsconfig, _arg: fd, _arg: cmd, _arg: key, _arg: value, _arg: aux) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_fsmount, _arg: fd, _arg: flags, _arg: attr_flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_mount, _arg: src, _arg: tgt, _arg: fst, _arg: flags, _arg: data) -> return;
}

pub const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004 /* Empty from path permitted */;

pub const MOVE_MOUNT_T_EMPTY_PATH: c_uint = 0x00000040 /* Empty to path permitted */;

pub const __NR_move_mount: c_int = 539;

pub const __NR_move_mount: c_int = 4429;

pub const __NR_move_mount: c_int = 6429;

pub const __NR_move_mount: c_int = 5429;

pub const __NR_move_mount: c_int = 429;

pub const OPEN_TREE_CLONE: c_int = 1;

pub const AT_RECURSIVE: c_uint = 0x8000 /* Apply to the entire subtree */;

pub const __NR_open_tree: c_int = 538;

pub const __NR_open_tree: c_int = 4428;

pub const __NR_open_tree: c_int = 6428;

pub const __NR_open_tree: c_int = 5428;

pub const __NR_open_tree: c_int = 428;

extern "C" {
    pub fn syscall(_arg: __NR_open_tree, _arg: dfd, _arg: filename, _arg: flags) -> return;
}
