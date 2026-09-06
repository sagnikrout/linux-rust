//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/blocklayoutxdr.h
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
pub const _NFSD_BLOCKLAYOUTXDR_H: c_int = 1;

// On the wire size of the layout4 struct with zero number of extents

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_extent {
    pub vol_id: nfsd4_deviceid,
    pub foff: u64,
    pub len: u64,
    pub soff: u64,
    pub es: pnfs_block_extent_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_range {
    pub foff: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_layout {
    pub nr_extents: u32,
    pub __counted_by(nr_extents): pnfs_block_extent extents[],
}

//
// Random upper cap for the uuid length to avoid unbounded allocation.
// Not actually limited by the protocol.
//
pub const PNFS_BLOCK_UUID_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_volume {
    pub type: pnfs_block_volume_type,
    pub offset: u64,
    pub sig_len: u32,
    pub sig: [u8; PNFS_BLOCK_UUID_LEN],
    pub simple: },
    pub code_set: scsi_code_set,
    pub designator_type: scsi_designator_type,
    pub designator_len: c_int,
    pub designator: [u8; 256],
    pub pr_key: u64,
    pub scsi: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_deviceaddr {
    pub nr_volumes: u32,
    pub __counted_by(nr_volumes): pnfs_block_volume volumes[],
}
