//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ext4.h
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
// ext4-specific ioctl commands
//

// note ioctl 10 reserved for an early version of the FIEMAP ioctl
// note ioctl 11 reserved for filesystem-independent FIEMAP ioctl

// ioctl codes 19--39 are reserved for fscrypt

//
// ioctl commands in 32 bit emulation
//

//
// Flags returned by EXT4_IOC_GETSTATE
//
// We only expose to userspace a subset of the state flags in
// i_state_flags
//
pub const EXT4_STATE_FLAG_EXT_PRECACHED: c_uint = 0x00000001;
pub const EXT4_STATE_FLAG_NEW: c_uint = 0x00000002;
pub const EXT4_STATE_FLAG_NEWENTRY: c_uint = 0x00000004;
pub const EXT4_STATE_FLAG_DA_ALLOC_CLOSE: c_uint = 0x00000008;
//
// Flags for ioctl EXT4_IOC_CHECKPOINT
//
pub const EXT4_IOC_CHECKPOINT_FLAG_DISCARD: c_uint = 0x1;
pub const EXT4_IOC_CHECKPOINT_FLAG_ZEROOUT: c_uint = 0x2;
pub const EXT4_IOC_CHECKPOINT_FLAG_DRY_RUN: c_uint = 0x4;

//
// Structure for EXT4_IOC_GETFSUUID/EXT4_IOC_SETFSUUID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsuuid {
    pub fsu_len: __u32,
    pub fsu_flags: __u32,
    pub fsu_uuid: [__u8; ],
}

//
// Structure for EXT4_IOC_MOVE_EXT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct move_extent {
    pub /: *mut *mut __u32 reserved; / should be zero,
    pub /: *mut *mut __u32 donor_fd; / donor file descriptor,
    pub /: *mut *mut __u64 orig_start; / logical start offset in block for orig,
    pub /: *mut *mut __u64 donor_start; / logical start offset in block for donor,
    pub /: *mut *mut __u64 len; / block length to be moved,
    pub /: *mut *mut __u64 moved_len; / moved block length,
}

//
// Flags used by EXT4_IOC_SHUTDOWN
//
pub const EXT4_GOING_FLAGS_DEFAULT: c_uint = 0x0	/* going down */;
pub const EXT4_GOING_FLAGS_LOGFLUSH: c_uint = 0x1	/* flush log but not data */;
pub const EXT4_GOING_FLAGS_NOLOGFLUSH: c_uint = 0x2	/* don't flush log nor data */;
// Used to pass group descriptor data when online resize is done
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_new_group_input {
    pub /: *mut *mut __u32 group; / Group number for this data,
    pub /: *mut *mut __u64 block_bitmap; / Absolute block number of block bitmap,
    pub /: *mut *mut __u64 inode_bitmap; / Absolute block number of inode bitmap,
    pub /: *mut *mut __u64 inode_table; / Absolute block number of inode table start,
    pub /: *mut *mut __u32 blocks_count; / Total number of blocks in this group,
    pub /: *mut *mut __u16 reserved_blocks; / Number of reserved blocks in this group,
    pub unused: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_tune_sb_params {
    pub set_flags: __u32,
    pub checkinterval: __u32,
    pub errors_behavior: __u16,
    pub mnt_count: __u16,
    pub max_mnt_count: __u16,
    pub raid_stride: __u16,
    pub last_check_time: __u64,
    pub reserved_blocks: __u64,
    pub blocks_count: __u64,
    pub default_mnt_opts: __u32,
    pub reserved_uid: __u32,
    pub reserved_gid: __u32,
    pub raid_stripe_width: __u32,
    pub encoding: __u16,
    pub encoding_flags: __u16,
    pub def_hash_alg: __u8,
    pub pad_1: __u8,
    pub pad_2: __u16,
    pub feature_compat: __u32,
    pub feature_incompat: __u32,
    pub feature_ro_compat: __u32,
    pub set_feature_compat_mask: __u32,
    pub set_feature_incompat_mask: __u32,
    pub set_feature_ro_compat_mask: __u32,
    pub clear_feature_compat_mask: __u32,
    pub clear_feature_incompat_mask: __u32,
    pub clear_feature_ro_compat_mask: __u32,
    pub mount_opts: [__u8; 64],
    pub pad: [__u8; 68],
}

pub const EXT4_TUNE_FL_ERRORS_BEHAVIOR: c_uint = 0x00000001;
pub const EXT4_TUNE_FL_MNT_COUNT: c_uint = 0x00000002;
pub const EXT4_TUNE_FL_MAX_MNT_COUNT: c_uint = 0x00000004;
pub const EXT4_TUNE_FL_CHECKINTRVAL: c_uint = 0x00000008;
pub const EXT4_TUNE_FL_LAST_CHECK_TIME: c_uint = 0x00000010;
pub const EXT4_TUNE_FL_RESERVED_BLOCKS: c_uint = 0x00000020;
pub const EXT4_TUNE_FL_RESERVED_UID: c_uint = 0x00000040;
pub const EXT4_TUNE_FL_RESERVED_GID: c_uint = 0x00000080;
pub const EXT4_TUNE_FL_DEFAULT_MNT_OPTS: c_uint = 0x00000100;
pub const EXT4_TUNE_FL_DEF_HASH_ALG: c_uint = 0x00000200;
pub const EXT4_TUNE_FL_RAID_STRIDE: c_uint = 0x00000400;
pub const EXT4_TUNE_FL_RAID_STRIPE_WIDTH: c_uint = 0x00000800;
pub const EXT4_TUNE_FL_MOUNT_OPTS: c_uint = 0x00001000;
pub const EXT4_TUNE_FL_FEATURES: c_uint = 0x00002000;
pub const EXT4_TUNE_FL_EDIT_FEATURES: c_uint = 0x00004000;
pub const EXT4_TUNE_FL_FORCE_FSCK: c_uint = 0x00008000;
pub const EXT4_TUNE_FL_ENCODING: c_uint = 0x00010000;
pub const EXT4_TUNE_FL_ENCODING_FLAGS: c_uint = 0x00020000;
//
// Returned by EXT4_IOC_GET_ES_CACHE as an additional possible flag.
// It indicates that the entry in extent status cache is for a hole.
//
pub const EXT4_FIEMAP_EXTENT_HOLE: c_uint = 0x08000000;
