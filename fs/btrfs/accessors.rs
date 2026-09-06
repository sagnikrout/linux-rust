//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/accessors.h
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
// Some macros to generate set/get functions for the struct fields.  This
// assumes there is a lefoo_to_cpu for every type, so lets make a simple one
// for u8:
//

// (u8 *)p = val;

extern "C" {
    pub fn btrfs_get_64(_arg: eb, _arg: s, btrfs_dev_item: offsetof(struct, _arg: total_bytes)) -> return;
}
extern "C" {
    pub fn btrfs_stripe_dev_uuid(_arg: btrfs_stripe_nr(c, _arg: nr)) -> return;
}
extern "C" {
    pub fn btrfs_stripe_offset(_arg: eb, _arg: btrfs_stripe_nr(c, _arg: nr)) -> return;
}
extern "C" {
    pub fn btrfs_stripe_devid(_arg: eb, _arg: btrfs_stripe_nr(c, _arg: nr)) -> return;
}
// struct btrfs_block_group_item
// struct btrfs_block_group_item_v2
// struct btrfs_free_space_info
// struct btrfs_inode_ref
// struct btrfs_inode_extref
// struct btrfs_inode_item
// struct btrfs_dev_extent
extern "C" {
    pub fn sizeof(btrfs_extent_inline_ref: struct) -> return;
}
extern "C" {
    pub fn sizeof(btrfs_extent_inline_ref: struct) -> return;
}
// struct btrfs_node
extern "C" {
    pub fn btrfs_key_blockptr(_arg: eb, )ptr: *mut (struct btrfs_key_ptr) -> return;
}
extern "C" {
    pub fn btrfs_key_generation(_arg: eb, )ptr: *mut (struct btrfs_key_ptr) -> return;
}
// struct btrfs_item

extern "C" {
    pub fn btrfs_item_offset(_arg: eb, btrfs_item_size(eb: nr) +, _arg: nr) -> return;
}
// struct btrfs_root_ref
// struct btrfs_dir_item
extern "C" {
    pub fn btrfs_dir_flags_to_ftype(_arg: btrfs_dir_flags(eb, _arg: item)) -> return;
}
extern "C" {
    pub fn btrfs_dir_flags_to_ftype(_arg: btrfs_stack_dir_flags(item)) -> return;
}
// struct btrfs_disk_key

//
// Optimized helpers for little-endian architectures where CPU and on-disk
// structures have the same endianness and we can skip conversions.
//

// struct btrfs_header
// struct btrfs_root_item
// struct btrfs_root_backup
// struct btrfs_balance_item
// struct btrfs_super_block
// struct btrfs_file_extent_item
// btrfs_qgroup_status_item
// btrfs_qgroup_info_item
// btrfs_qgroup_limit_item
// btrfs_dev_replace_item
// btrfs_verity_descriptor_item
// Cast into the data area of the leaf.

