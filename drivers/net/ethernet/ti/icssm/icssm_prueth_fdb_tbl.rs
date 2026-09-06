//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssm/icssm_prueth_fdb_tbl.h
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
// Copyright (C) 2019-2021 Texas Instruments Incorporated - https://www.ti.com

// 4 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_index_tbl_entry {
// Bucket Table index of first Bucket with this MAC address
    pub bucket_idx: u16,
    pub /: *mut *mut u16 bucket_entries; / Number of entries in this bucket,
}

// 4 * 256 = 1024 = 0x200 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_index_array {
    pub index_tbl_entry: [fdb_index_tbl_entry; FDB_INDEX_TBL_MAX_ENTRIES],
}

// 10 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_mac_tbl_entry {
    pub mac: [u8; ETH_ALEN],
    pub age: u16,
    pub /: *mut *mut u8 port; / 0 based: 0=port1, 1=port2,
    pub is_static:1: u8,
    pub active:1: u8,
}

// 10 * 256 = 2560 = 0xa00 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_mac_tbl_array {
    pub mac_tbl_entry: [fdb_mac_tbl_entry; FDB_MAC_TBL_MAX_ENTRIES],
}

// 1 byte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_stp_config {
    pub /: *mut *mut u8 state; / per-port STP state (defined in FW header),
}

// 1 byte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_flood_config {
    pub host_flood_enable:1: u8,
    pub port1_flood_enable:1: u8,
    pub port2_flood_enable:1: u8,
}

// 2 byte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_arbitration {
    pub host_lock: u8,
    pub pru_locks: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_tbl {
// fdb index table
    pub index_a: *mut fdb_index_array __iomem,
// fdb MAC table
    pub mac_tbl_a: *mut fdb_mac_tbl_array __iomem,
// port 1 stp config
    pub port1_stp_cfg: *mut fdb_stp_config __iomem,
// port 2 stp config
    pub port2_stp_cfg: *mut fdb_stp_config __iomem,
// per-port flood enable
    pub flood_enable_flags: *mut fdb_flood_config __iomem,
// fdb locking mechanism
    pub locks: *mut fdb_arbitration __iomem,
// total number of entries in hash table
    pub total_entries: u16,
}
