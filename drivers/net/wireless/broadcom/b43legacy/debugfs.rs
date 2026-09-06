//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/debugfs.h
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

// Macro flag: #define B43legacy_DEBUGFS_H_
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43legacy_dyndbg {
    B43legacy_DBG_XMITPOWER,
    B43legacy_DBG_DMAOVERFLOW,
    B43legacy_DBG_DMAVERBOSE,
    B43legacy_DBG_PWORK_FAST,
    B43legacy_DBG_PWORK_STOP,
    __B43legacy_NR_DYNDBG,
}

pub const B43legacy_NR_LOGGED_TXSTATUS: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_txstatus_log {
    pub log: *mut b43legacy_txstatus,
    pub end: c_int,
    pub /: *mut *mut spinlock_t lock; / lock for debugging,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dfs_file {
    pub buffer: *mut c_char,
    pub data_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_dfsentry {
    pub dev: *mut b43legacy_wldev,
    pub subdir: *mut dentry,
    pub file_tsf: b43legacy_dfs_file,
    pub file_ucode_regs: b43legacy_dfs_file,
    pub file_shm: b43legacy_dfs_file,
    pub file_txstat: b43legacy_dfs_file,
    pub file_txpower_g: b43legacy_dfs_file,
    pub file_restart: b43legacy_dfs_file,
    pub file_loctls: b43legacy_dfs_file,
    pub txstatlog: b43legacy_txstatus_log,
// Enabled/Disabled list for the dynamic debugging features.
    pub dyn_debug: [bool; __B43legacy_NR_DYNDBG],
}

extern "C" {
    pub fn b43legacy_debugfs_init();
}
extern "C" {
    pub fn b43legacy_debugfs_exit();
}
extern "C" {
    pub fn b43legacy_debugfs_add_device(dev: *mut b43legacy_wldev);
}
extern "C" {
    pub fn b43legacy_debugfs_remove_device(dev: *mut b43legacy_wldev);
}

