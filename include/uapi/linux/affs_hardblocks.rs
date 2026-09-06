//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/affs_hardblocks.h
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

// Just the needed definitions for the RDB of an Amiga HD.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RigidDiskBlock {
    pub rdb_ID: __be32,
    pub rdb_SummedLongs: __be32,
    pub rdb_ChkSum: __be32,
    pub rdb_HostID: __be32,
    pub rdb_BlockBytes: __be32,
    pub rdb_Flags: __be32,
    pub rdb_BadBlockList: __be32,
    pub rdb_PartitionList: __be32,
    pub rdb_FileSysHeaderList: __be32,
    pub rdb_DriveInit: __be32,
    pub rdb_Reserved1: [__be32; 6],
    pub rdb_Cylinders: __be32,
    pub rdb_Sectors: __be32,
    pub rdb_Heads: __be32,
    pub rdb_Interleave: __be32,
    pub rdb_Park: __be32,
    pub rdb_Reserved2: [__be32; 3],
    pub rdb_WritePreComp: __be32,
    pub rdb_ReducedWrite: __be32,
    pub rdb_StepRate: __be32,
    pub rdb_Reserved3: [__be32; 5],
    pub rdb_RDBBlocksLo: __be32,
    pub rdb_RDBBlocksHi: __be32,
    pub rdb_LoCylinder: __be32,
    pub rdb_HiCylinder: __be32,
    pub rdb_CylBlocks: __be32,
    pub rdb_AutoParkSeconds: __be32,
    pub rdb_HighRDSKBlock: __be32,
    pub rdb_Reserved4: __be32,
    pub rdb_DiskVendor: [c_char; 8],
    pub rdb_DiskProduct: [c_char; 16],
    pub rdb_DiskRevision: [c_char; 4],
    pub rdb_ControllerVendor: [c_char; 8],
    pub rdb_ControllerProduct: [c_char; 16],
    pub rdb_ControllerRevision: [c_char; 4],
    pub rdb_Reserved5: [__be32; 10],
}

pub const IDNAME_RIGIDDISK: c_uint = 0x5244534B	/* "RDSK" */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionBlock {
    pub pb_ID: __be32,
    pub pb_SummedLongs: __be32,
    pub pb_ChkSum: __be32,
    pub pb_HostID: __be32,
    pub pb_Next: __be32,
    pub pb_Flags: __be32,
    pub pb_Reserved1: [__be32; 2],
    pub pb_DevFlags: __be32,
    pub pb_DriveName: [__u8; 32],
    pub pb_Reserved2: [__be32; 15],
    pub pb_Environment: [__be32; 17],
    pub pb_EReserved: [__be32; 15],
}

pub const IDNAME_PARTITION: c_uint = 0x50415254	/* "PART" */;
pub const RDB_ALLOCATION_LIMIT: c_int = 16;
