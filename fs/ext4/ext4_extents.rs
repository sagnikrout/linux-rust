//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/ext4_extents.h
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
// Copyright (c) 2003-2006, Cluster File Systems, Inc, info@clusterfs.com
// Written by Alex Tomas <alex@clusterfs.com>
//

//
// With AGGRESSIVE_TEST defined, the capacity of index/leaf blocks
// becomes very small, so index split, in-depth growing and
// other hard changes happen much more often.
// This is for debug purposes only.
//
// Macro flag: #define AGGRESSIVE_TEST_
//
// With EXTENTS_STATS defined, the number of blocks and extents
// are collected in the truncate path. They'll be shown at
// umount time.
//
// Macro flag: #define EXTENTS_STATS__
//
// If CHECK_BINSEARCH is defined, then the results of the binary search
// will also be checked by linear search.
//
// Macro flag: #define CHECK_BINSEARCH__
//
// ext4_inode has i_block array (60 bytes total).
// The first 12 bytes store ext4_extent_header;
// the remainder stores an array of ext4_extent.
// For non-inode extent blocks, ext4_extent_tail
// follows the array.
//
// This is the extent tail on-disk structure.
// All other extent structures are 12 bytes long.  It turns out that
// block_size % 12 >= 4 for at least all powers of 2 greater than 512, which
// covers all valid ext4 block sizes.  Therefore, this tail structure can be
// crammed into the end of the block without having to rebalance the tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_extent_tail {
    pub /: *mut *mut __le32 et_checksum; / crc32c(uuid+inum+extent_block),
}

//
// This is the extent on-disk structure.
// It's used at the bottom of the tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_extent {
    pub /: *mut *mut __le32 ee_block; / first logical block extent covers,
    pub /: *mut *mut __le16 ee_len; / number of blocks covered by extent,
    pub /: *mut *mut __le16 ee_start_hi; / high 16 bits of physical block,
    pub /: *mut *mut __le32 ee_start_lo; / low 32 bits of physical block,
}

//
// This is index on-disk structure.
// It's used at all the levels except the bottom.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_extent_idx {
    pub /: *mut *mut __le32 ei_block; / index covers logical blocks from 'block',
    pub : *mut *mut __le32 ei_leaf_lo; / pointer to the physical block of the next,
// level. leaf or next index could be there
    pub /: *mut *mut __le16 ei_leaf_hi; / high 16 bits of physical block,
    pub ei_unused: __u16,
}

//
// Each block (leaves and indexes), even inode-stored has header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_extent_header {
    pub /: *mut *mut __le16 eh_magic; / probably will support different formats,
    pub /: *mut *mut __le16 eh_entries; / number of valid entries,
    pub /: *mut *mut __le16 eh_max; / capacity of store in entries,
    pub /: *mut *mut __le16 eh_depth; / has tree real underlying blocks?,
    pub /: *mut *mut __le32 eh_generation; / generation of the tree,
}

pub const EXT4_MAX_EXTENT_DEPTH: c_int = 5;

//
// Array of ext4_ext_path contains path to some extent.
// Creation/lookup routines use it for traversal/splitting/etc.
// Truncate uses it to simulate recursive walking.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_ext_path {
    pub p_block: ext4_fsblk_t,
    pub p_depth: __u16,
    pub p_maxdepth: __u16,
    pub p_ext: *mut ext4_extent,
    pub p_idx: *mut ext4_extent_idx,
    pub p_hdr: *mut ext4_extent_header,
    pub p_bh: *mut buffer_head,
}

//
// Used to record a portion of a cluster found at the beginning or end
// of an extent while traversing the extent tree during space removal.
// A partial cluster may be removed if it does not contain blocks shared
// with extents that aren't being deleted (tofree state).  Otherwise,
// it cannot be removed (nofree state).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partial_cluster {
    pub /: *mut *mut ext4_fsblk_t pclu; / physical cluster number,
    pub /: *mut *mut ext4_lblk_t lblk; / logical block number within logical cluster,
    pub state: {initial, tofree, nofree},
}

//
// structure for external API
//
// EXT_INIT_MAX_LEN is the maximum number of blocks we can have in an
// initialized extent. This is 2^15 and not (2^16 - 1), since we use the
// MSB of ee_len field in the extent datastructure to signify if this
// particular extent is an initialized extent or an unwritten (i.e.
// preallocated).
// EXT_UNWRITTEN_MAX_LEN is the maximum number of blocks we can have in an
// unwritten extent.
// If ee_len is <= 0x8000, it is an initialized extent. Otherwise, it is an
// unwritten one. In other words, if MSB of ee_len is set, it is an
// unwritten extent with only one special scenario when ee_len = 0x8000.
// In this case we can not have an unwritten extent of zero length and
// thus we make it as a special case of initialized extent with 0x8000 length.
// This way we get better extent-to-group alignment for initialized extents.
// Hence, the maximum number of blocks we can have in an *initialized
// extent is 2^15 (32768) and in an *unwritten* extent is 2^15-1 (32767).
//

extern "C" {
    pub fn le16_to_cpu(_arg: ext_inode_hdr(inode)->eh_depth) -> return;
}
// We can not have an unwritten extent of zero length!
// Extent with ee_len of 0x8000 is treated as an initialized extent
//
// ext4_ext_pblock:
// combine low and high parts of physical block number into ext4_fsblk_t
//
// ext4_idx_pblock:
// combine low and high parts of a leaf physical block number into ext4_fsblk_t
//
// ext4_ext_store_pblock:
// stores a large physical block number into an extent struct,
// breaking it into parts
//
// ext4_idx_store_pblock:
// stores a large physical block number into an index struct,
// breaking it into parts
//
extern "C" {
    pub fn ext4_ext_zeroout(inode: *mut inode, ex: *mut ext4_extent) -> c_int;
}

extern "C" {
    pub fn ext4_ext_space_root_idx_test(inode: *mut inode, check: c_int) -> c_int;
}

