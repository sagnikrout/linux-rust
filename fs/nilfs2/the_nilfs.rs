//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/the_nilfs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// the_nilfs shared structure.
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Ryusuke Konishi.
//

// the_nilfs struct
//
// struct the_nilfs - struct to supervise multiple nilfs mount points
// @ns_flags: flags
// @ns_flushed_device: flag indicating if all volatile data was flushed
// @ns_sb: back pointer to super block instance
// @ns_bdev: block device
// @ns_sem: semaphore for shared states
// @ns_snapshot_mount_mutex: mutex to protect snapshot mounts
// @ns_sbh: buffer heads of on-disk super blocks
// @ns_sbp: pointers to super block data
// @ns_sbwtime: previous write time of super block
// @ns_sbwcount: write count of super block
// @ns_sbsize: size of valid data in super block
// @ns_mount_state: file system state
// @ns_sb_update_freq: interval of periodical update of superblocks (in seconds)
// @ns_seg_seq: segment sequence counter
// @ns_segnum: index number of the latest full segment.
// @ns_nextnum: index number of the full segment index to be used next
// @ns_pseg_offset: offset of next partial segment in the current full segment
// @ns_cno: next checkpoint number
// @ns_ctime: write time of the last segment
// @ns_nongc_ctime: write time of the last segment not for cleaner operation
// @ns_ndirtyblks: Number of dirty data blocks
// @ns_last_segment_lock: lock protecting fields for the latest segment
// @ns_last_pseg: start block number of the latest segment
// @ns_last_seq: sequence value of the latest segment
// @ns_last_cno: checkpoint number of the latest segment
// @ns_prot_seq: least sequence number of segments which must not be reclaimed
// @ns_prev_seq: base sequence number used to decide if advance log cursor
// @ns_writer: log writer
// @ns_segctor_sem: semaphore protecting log write
// @ns_dat: DAT file inode
// @ns_cpfile: checkpoint file inode
// @ns_sufile: segusage file inode
// @ns_cptree: rb-tree of all mounted checkpoints (nilfs_root)
// @ns_cptree_lock: lock protecting @ns_cptree
// @ns_dirty_files: list of dirty files
// @ns_inode_lock: lock protecting @ns_dirty_files
// @ns_gc_inodes: dummy inodes to keep live blocks
// @ns_mount_opt: mount options
// @ns_resuid: uid for reserved blocks
// @ns_resgid: gid for reserved blocks
// @ns_interval: checkpoint creation interval
// @ns_watermark: watermark for the number of dirty buffers
// @ns_blocksize_bits: bit length of block size
// @ns_blocksize: block size
// @ns_nsegments: number of segments in filesystem
// @ns_blocks_per_segment: number of blocks per segment
// @ns_r_segments_percentage: reserved segments percentage
// @ns_nrsvsegs: number of reserved segments
// @ns_first_data_block: block number of first data block
// @ns_inode_size: size of on-disk inode
// @ns_first_ino: first not-special inode number
// @ns_crc_seed: seed value of CRC32 calculation
// @ns_dev_kobj: /sys/fs/<nilfs>/<device>
// @ns_dev_kobj_unregister: completion state
// @ns_dev_subgroups: <device> subgroups pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct the_nilfs {
    pub ns_flags: c_ulong,
    pub ns_flushed_device: c_int,
    pub ns_sb: *mut super_block,
    pub ns_bdev: *mut block_device,
    pub ns_sem: rw_semaphore,
    pub ns_snapshot_mount_mutex: mutex,
//
// used for
// - loading the latest checkpoint exclusively.
// - allocating a new full segment.
//
    pub ns_sbh: [*mut buffer_head; 2],
    pub ns_sbp: [*mut nilfs_super_block; 2],
    pub ns_sbwtime: time64_t,
    pub ns_sbwcount: c_uint,
    pub ns_sbsize: c_uint,
    pub ns_mount_state: c_uint,
    pub ns_sb_update_freq: c_uint,
//
// The following fields are updated by a writable FS-instance.
// These fields are protected by ns_segctor_sem outside load_nilfs().
//
    pub ns_seg_seq: u64,
    pub ns_segnum: __u64,
    pub ns_nextnum: __u64,
    pub ns_pseg_offset: c_ulong,
    pub ns_cno: __u64,
    pub ns_ctime: time64_t,
    pub ns_nongc_ctime: time64_t,
    pub ns_ndirtyblks: core::sync::atomic::AtomicI32,
//
// The following fields hold information on the latest partial segment
// written to disk with a super root.  These fields are protected by
// ns_last_segment_lock.
//
    pub ns_last_segment_lock: spinlock_t,
    pub ns_last_pseg: sector_t,
    pub ns_last_seq: u64,
    pub ns_last_cno: __u64,
    pub ns_prot_seq: u64,
    pub ns_prev_seq: u64,
    pub ns_writer: *mut nilfs_sc_info,
    pub ns_segctor_sem: rw_semaphore,
//
// Following fields are lock free except for the period before
// the_nilfs is initialized.
//
    pub ns_dat: *mut inode,
    pub ns_cpfile: *mut inode,
    pub ns_sufile: *mut inode,
// Checkpoint tree
    pub ns_cptree: rb_root,
    pub ns_cptree_lock: spinlock_t,
// Dirty inode list
    pub ns_dirty_files: list_head,
    pub ns_inode_lock: spinlock_t,
// GC inode list
    pub ns_gc_inodes: list_head,
// Mount options
    pub ns_mount_opt: c_ulong,
    pub ns_resuid: uid_t,
    pub ns_resgid: gid_t,
    pub ns_interval: c_ulong,
    pub ns_watermark: c_ulong,
// Disk layout information (static)
    pub ns_blocksize_bits: c_uint,
    pub ns_blocksize: c_uint,
    pub ns_nsegments: c_ulong,
    pub ns_blocks_per_segment: c_ulong,
    pub ns_r_segments_percentage: c_ulong,
    pub ns_nrsvsegs: c_ulong,
    pub ns_first_data_block: c_ulong,
    pub ns_inode_size: c_int,
    pub ns_first_ino: c_uint,
    pub ns_crc_seed: u32,
// /sys/fs/<nilfs>/<device>
    pub ns_dev_kobj: kobject,
    pub ns_dev_kobj_unregister: completion,
    pub ns_dev_subgroups: *mut nilfs_sysfs_dev_subgroups,
}

//
// Mount option operations
//

//
// struct nilfs_root - nilfs root object
// @cno: checkpoint number
// @rb_node: red-black tree node
// @count: refcount of this structure
// @nilfs: nilfs object
// @ifile: inode file
// @inodes_count: number of inodes
// @blocks_count: number of blocks
// @snapshot_kobj: /sys/fs/<nilfs>/<device>/mounted_snapshots/<snapshot>
// @snapshot_kobj_unregister: completion state for kernel object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_root {
    pub cno: __u64,
    pub rb_node: rb_node,
    pub count: refcount_t,
    pub nilfs: *mut the_nilfs,
    pub ifile: *mut inode,
    pub inodes_count: core::sync::atomic::AtomicI64,
    pub blocks_count: core::sync::atomic::AtomicI64,
// /sys/fs/<nilfs>/<device>/mounted_snapshots/<snapshot>
    pub snapshot_kobj: kobject,
    pub snapshot_kobj_unregister: completion,
}

// Special checkpoint number
pub const NILFS_CPTREE_CURRENT_CNO: c_int = 0;
// Minimum interval of periodical update of superblocks (in seconds)
pub const NILFS_SB_FREQ: c_int = 10;
extern "C" {
    pub fn nilfs_set_last_segment(: *mut the_nilfs, _arg: sector_t, _arg: u64, _arg: __u64);
}
extern "C" {
    pub fn destroy_nilfs(nilfs: *mut the_nilfs);
}
extern "C" {
    pub fn init_nilfs(nilfs: *mut the_nilfs, sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn load_nilfs(nilfs: *mut the_nilfs, sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn nilfs_nrsvsegs(nilfs: *mut the_nilfs, nsegs: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn nilfs_set_nsegments(nilfs: *mut the_nilfs, nsegs: c_ulong);
}
extern "C" {
    pub fn nilfs_discard_segments(: *mut the_nilfs, : *mut __u64, _arg: usize) -> c_int;
}
extern "C" {
    pub fn nilfs_count_free_blocks(: *mut the_nilfs, : *mut sector_t) -> c_int;
}
extern "C" {
    pub fn nilfs_put_root(root: *mut nilfs_root);
}
extern "C" {
    pub fn nilfs_near_disk_full(: *mut the_nilfs) -> c_int;
}
extern "C" {
    pub fn nilfs_fall_back_super_block(: *mut the_nilfs);
}
extern "C" {
    pub fn nilfs_swap_super_block(: *mut the_nilfs);
}
// seg_start = (sector_t)nilfs->ns_blocks_per_segment * segnum;
// seg_end = *seg_start + nilfs->ns_blocks_per_segment - 1;
// seg_start = nilfs->ns_first_data_block;
// terminate the current full segment (used in case of I/O-error)
// move forward with a full segment
//
// the store to ns_flushed_device must not be reordered after
// blkdev_issue_flush().
//
