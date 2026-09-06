//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/debugfs.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_dyndbg {
    B43_DBG_XMITPOWER,
    B43_DBG_DMAOVERFLOW,
    B43_DBG_DMAVERBOSE,
    B43_DBG_PWORK_FAST,
    B43_DBG_PWORK_STOP,
    B43_DBG_LO,
    B43_DBG_FIRMWARE,
    B43_DBG_KEYS,
    B43_DBG_VERBOSESTATS,
    __B43_NR_DYNDBG,
}

pub const B43_NR_LOGGED_TXSTATUS: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_txstatus_log {
// This structure is protected by wl->mutex
    pub log: *mut b43_txstatus,
    pub end: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dfs_file {
    pub buffer: *mut c_char,
    pub data_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_dfsentry {
    pub dev: *mut b43_wldev,
    pub subdir: *mut dentry,
    pub file_shm16read: b43_dfs_file,
    pub file_shm16write: b43_dfs_file,
    pub file_shm32read: b43_dfs_file,
    pub file_shm32write: b43_dfs_file,
    pub file_mmio16read: b43_dfs_file,
    pub file_mmio16write: b43_dfs_file,
    pub file_mmio32read: b43_dfs_file,
    pub file_mmio32write: b43_dfs_file,
    pub file_txstat: b43_dfs_file,
    pub file_txpower_g: b43_dfs_file,
    pub file_restart: b43_dfs_file,
    pub file_loctls: b43_dfs_file,
    pub txstatlog: b43_txstatus_log,
// The cached address for the next mmio16read file read
    pub mmio16read_next: u16,
// The cached address for the next mmio32read file read
    pub mmio32read_next: u16,
// The cached address for the next shm16read file read
    pub shm16read_routing_next: u32,
    pub shm16read_addr_next: u32,
// The cached address for the next shm32read file read
    pub shm32read_routing_next: u32,
    pub shm32read_addr_next: u32,
// Enabled/Disabled list for the dynamic debugging features.
    pub dyn_debug: [bool; __B43_NR_DYNDBG],
}

extern "C" {
    pub fn b43_debug(dev: *mut b43_wldev, feature: b43_dyndbg) -> bool;
}
extern "C" {
    pub fn b43_debugfs_init();
}
extern "C" {
    pub fn b43_debugfs_exit();
}
extern "C" {
    pub fn b43_debugfs_add_device(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_debugfs_remove_device(dev: *mut b43_wldev);
}

