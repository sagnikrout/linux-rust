//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/fast_commit.h
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
// Note this file is present in e2fsprogs/lib/ext2fs/fast_commit.h and
// linux/fs/ext4/fast_commit.h. These file should always be byte identical.
//
// Fast commit tags
pub const EXT4_FC_TAG_ADD_RANGE: c_uint = 0x0001;
pub const EXT4_FC_TAG_DEL_RANGE: c_uint = 0x0002;
pub const EXT4_FC_TAG_CREAT: c_uint = 0x0003;
pub const EXT4_FC_TAG_LINK: c_uint = 0x0004;
pub const EXT4_FC_TAG_UNLINK: c_uint = 0x0005;
pub const EXT4_FC_TAG_INODE: c_uint = 0x0006;
pub const EXT4_FC_TAG_PAD: c_uint = 0x0007;
pub const EXT4_FC_TAG_TAIL: c_uint = 0x0008;
pub const EXT4_FC_TAG_HEAD: c_uint = 0x0009;
pub const EXT4_FC_SUPPORTED_FEATURES: c_uint = 0x0;
// On disk fast commit tlv value structures
// Fast commit on disk tag length structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_tl {
    pub fc_tag: __le16,
    pub fc_len: __le16,
}

// Value structure for tag EXT4_FC_TAG_HEAD.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_head {
    pub fc_features: __le32,
    pub fc_tid: __le32,
}

// Value structure for EXT4_FC_TAG_ADD_RANGE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_add_range {
    pub fc_ino: __le32,
    pub fc_ex: [__u8; 12],
}

// Value structure for tag EXT4_FC_TAG_DEL_RANGE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_del_range {
    pub fc_ino: __le32,
    pub fc_lblk: __le32,
    pub fc_len: __le32,
}

//
// This is the value structure for tags EXT4_FC_TAG_CREAT, EXT4_FC_TAG_LINK
// and EXT4_FC_TAG_UNLINK.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_dentry_info {
    pub fc_parent_ino: __le32,
    pub fc_ino: __le32,
    pub fc_dname: [__u8; ],
}

// Value structure for EXT4_FC_TAG_INODE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_inode {
    pub fc_ino: __le32,
    pub fc_raw_inode: [__u8; ],
}

// Value structure for tag EXT4_FC_TAG_TAIL.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_tail {
    pub fc_tid: __le32,
    pub fc_crc: __le32,
}

// Tag base length

//
// Fast commit status codes
//
// Fast commit ineligiblity reasons:
//

//
// In memory list of dentry updates that are performed on the file
// system used by fast commit code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_dentry_update {
    pub /: *mut *mut int fcd_op; / Type of update create / unlink / link,
    pub /: *mut *mut int fcd_parent; / Parent inode number,
    pub /: *mut *mut int fcd_ino; / Inode number,
    pub /: *mut *mut name_snapshot fcd_name; / Dirent name,
    pub fcd_list: list_head,
    pub fcd_dilist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_stats {
    pub fc_ineligible_reason_count: [c_uint; EXT4_FC_REASON_MAX],
    pub fc_num_commits: c_ulong,
    pub fc_ineligible_commits: c_ulong,
    pub fc_failed_commits: c_ulong,
    pub fc_skipped_commits: c_ulong,
    pub fc_numblks: c_ulong,
    pub s_fc_avg_commit_time: u64,
}

pub const EXT4_FC_REPLAY_REALLOC_INCREMENT: c_int = 4;
//
// Physical block regions added to different inodes due to fast commit
// recovery. These are set during the SCAN phase. During the replay phase,
// our allocator excludes these from its allocation. This ensures that
// we don't accidentally allocating a block that is going to be used by
// another inode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_alloc_region {
    pub lblk: ext4_lblk_t,
    pub pblk: ext4_fsblk_t,
    pub len: int ino,,
}

//
// Fast commit replay state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_fc_replay_state {
    pub fc_replay_num_tags: c_int,
    pub fc_replay_expected_off: c_int,
    pub fc_current_pass: c_int,
    pub fc_cur_tag: c_int,
    pub fc_crc: c_int,
    pub fc_regions: *mut ext4_fc_alloc_region,
    pub fc_regions_valid: int fc_regions_size, fc_regions_used,,
    pub fc_modified_inodes: *mut c_int,
    pub fc_modified_inodes_size: int fc_modified_inodes_used,,
}

