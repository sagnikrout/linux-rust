//! Automatically rewritten from C Header to Rust Module
//! Source: block/partitions/mac.h
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
// fs/partitions/mac.h
//
pub const MAC_PARTITION_MAGIC: c_uint = 0x504d;
// type field value for A/UX or other Unix partitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_partition {
    pub /: *mut *mut __be16 signature; / expected to be MAC_PARTITION_MAGIC,
    pub res1: __be16,
    pub /: *mut *mut __be32 map_count; / # blocks in partition map,
    pub /: *mut *mut __be32 start_block; / absolute starting block # of partition,
    pub /: *mut *mut __be32 block_count; / number of blocks in partition,
    pub /: *mut *mut char name[32]; / partition name,
    pub /: *mut *mut char type[32]; / string type description,
    pub /: *mut *mut __be32 data_start; / rel block # of first data block,
    pub /: *mut *mut __be32 data_count; / number of data blocks,
    pub /: *mut *mut __be32 status; / partition status bits,
    pub boot_start: __be32,
    pub boot_size: __be32,
    pub boot_load: __be32,
    pub boot_load2: __be32,
    pub boot_entry: __be32,
    pub boot_entry2: __be32,
    pub boot_cksum: __be32,
    pub /: *mut *mut char processor[16]; / identifies ISA of boot,
// there is more stuff after this that we don't need
}

pub const MAC_DRIVER_MAGIC: c_uint = 0x4552;
// Driver descriptor structure, in block 0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_driver_desc {
    pub /: *mut *mut __be16 signature; / expected to be MAC_DRIVER_MAGIC,
    pub block_size: __be16,
    pub block_count: __be32,
// ... more stuff
}
