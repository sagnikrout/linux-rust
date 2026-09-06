//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/f2fs.h
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

//
// f2fs-specific ioctl commands
//
pub const F2FS_IOCTL_MAGIC: c_uint = 0xf5;

//
// should be same as XFS_IOC_GOINGDOWN.
// Flags for going down operation used by FS_IOC_GOINGDOWN
//

pub const F2FS_GOING_DOWN_FULLSYNC: c_uint = 0x0	/* going down with full sync */;
pub const F2FS_GOING_DOWN_METASYNC: c_uint = 0x1	/* going down with metadata */;
pub const F2FS_GOING_DOWN_NOSYNC: c_uint = 0x2	/* going down */;
pub const F2FS_GOING_DOWN_METAFLUSH: c_uint = 0x3	/* going down with meta flush */;
pub const F2FS_GOING_DOWN_NEED_FSCK: c_uint = 0x4	/* going down to trigger fsck */;
//
// Flags used by F2FS_IOC_SEC_TRIM_FILE
//
pub const F2FS_TRIM_FILE_DISCARD: c_uint = 0x1	/* send discard command */;
pub const F2FS_TRIM_FILE_ZEROOUT: c_uint = 0x2	/* zero out */;
pub const F2FS_TRIM_FILE_MASK: c_uint = 0x3;
// for F2FS_IOC_IO_PRIO
// for F2FS_IOC_GET_DEV_ALIAS_STATUS
pub const F2FS_DEV_ALIAS_STATUS_RELEASED: c_int = 0;
pub const F2FS_DEV_ALIAS_STATUS_RESERVED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_gc_range {
    pub sync: __u32,
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_defragment {
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_move_range {
    pub /: *mut *mut __u32 dst_fd; / destination fd,
    pub /: *mut *mut __u64 pos_in; / start position in src_fd,
    pub /: *mut *mut __u64 pos_out; / start position in dst_fd,
    pub /: *mut *mut __u64 len; / size to move,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_flush_device {
    pub /: *mut *mut __u32 dev_num; / device number to flush,
    pub /: *mut *mut __u32 segments; / # of segments to flush,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sectrim_range {
    pub start: __u64,
    pub len: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_comp_option {
    pub algorithm: __u8,
    pub log_cluster_size: __u8,
}
