//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nilfs2_ondisk.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// nilfs2_ondisk.h - NILFS2 on-disk structures
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation; either version 2.1 of the License, or
// (at your option) any later version.
//
// linux/include/linux/ext2_fs.h
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// from
//
// linux/include/linux/minix_fs.h
//
// Copyright (C) 1991, 1992  Linus Torvalds
//

pub const NILFS_INODE_BMAP_SIZE: c_int = 7;
//
// struct nilfs_inode - structure of an inode on disk
// @i_blocks: blocks count
// @i_size: size in bytes
// @i_ctime: creation time (seconds)
// @i_mtime: modification time (seconds)
// @i_ctime_nsec: creation time (nano seconds)
// @i_mtime_nsec: modification time (nano seconds)
// @i_uid: user id
// @i_gid: group id
// @i_mode: file mode
// @i_links_count: links count
// @i_flags: file flags
// @i_bmap: block mapping
// @i_xattr: extended attributes
// @i_generation: file generation (for NFS)
// @i_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_inode {
    pub i_blocks: __le64,
    pub i_size: __le64,
    pub i_ctime: __le64,
    pub i_mtime: __le64,
    pub i_ctime_nsec: __le32,
    pub i_mtime_nsec: __le32,
    pub i_uid: __le32,
    pub i_gid: __le32,
    pub i_mode: __le16,
    pub i_links_count: __le16,
    pub i_flags: __le32,
    pub i_bmap: [__le64; NILFS_INODE_BMAP_SIZE],
    pub i_xattr: __le64,
    pub i_generation: __le32,
    pub i_pad: __le32,
}

pub const NILFS_MIN_INODE_SIZE: c_int = 128;
//
// struct nilfs_super_root - structure of super root
// @sr_sum: check sum
// @sr_bytes: byte count of the structure
// @sr_flags: flags (reserved)
// @sr_nongc_ctime: write time of the last segment not for cleaner operation
// @sr_dat: DAT file inode
// @sr_cpfile: checkpoint file inode
// @sr_sufile: segment usage file inode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_super_root {
    pub sr_sum: __le32,
    pub sr_bytes: __le16,
    pub sr_flags: __le16,
    pub sr_nongc_ctime: __le64,
    pub sr_dat: nilfs_inode,
    pub sr_cpfile: nilfs_inode,
    pub sr_sufile: nilfs_inode,
}

//
// Maximal mount counts
//

//
// File system states (sbp->s_state, nilfs->ns_mount_state)
//
pub const NILFS_VALID_FS: c_uint = 0x0001  /* Unmounted cleanly */;
pub const NILFS_ERROR_FS: c_uint = 0x0002  /* Errors detected */;
pub const NILFS_RESIZE_FS: c_uint = 0x0004	/* Resize required */;
//
// Mount flags (sbi->s_mount_opt)
//
pub const NILFS_MOUNT_ERROR_MODE: c_uint = 0x0070  /* Error mode mask */;
pub const NILFS_MOUNT_ERRORS_CONT: c_uint = 0x0010  /* Continue on errors */;
pub const NILFS_MOUNT_ERRORS_RO: c_uint = 0x0020  /* Remount fs ro on errors */;
pub const NILFS_MOUNT_ERRORS_PANIC: c_uint = 0x0040  /* Panic on errors */;
pub const NILFS_MOUNT_BARRIER: c_uint = 0x1000  /* Use block barriers */;
pub const NILFS_MOUNT_STRICT_ORDER: c_uint = 0x2000  /*;
// Apply strict in-order
// semantics also for data
//
pub const NILFS_MOUNT_NORECOVERY: c_uint = 0x4000  /*;
// Disable write access during
// mount-time recovery
//
pub const NILFS_MOUNT_DISCARD: c_uint = 0x8000  /* Issue DISCARD requests */;
//
// struct nilfs_super_block - structure of super block on disk
// @s_rev_level:		Revision level
// @s_minor_rev_level:		minor revision level
// @s_magic:			Magic signature
// @s_bytes:			Bytes count of CRC calculation for
// this structure.  s_reserved is excluded.
// @s_flags:			flags
// @s_crc_seed:			Seed value of CRC calculation
// @s_sum:			Check sum of super block
// @s_log_block_size:		Block size represented as follows:
// blocksize = 1 << (s_log_block_size + 10)
// @s_nsegments:		Number of segments in filesystem
// @s_dev_size:			block device size in bytes
// @s_first_data_block:		1st seg disk block number
// @s_blocks_per_segment:	number of blocks per full segment
// @s_r_segments_percentage:	Reserved segments percentage
// @s_last_cno:			Last checkpoint number
// @s_last_pseg:		disk block addr pseg written last
// @s_last_seq:			seq. number of seg written last
// @s_free_blocks_count:	Free blocks count
// @s_ctime:			Creation time (execution time of newfs)
// @s_mtime:			Mount time
// @s_wtime:			Write time
// @s_mnt_count:		Mount count
// @s_max_mnt_count:		Maximal mount count
// @s_state:			File system state
// @s_errors:			Behaviour when detecting errors
// @s_lastcheck:		time of last check
// @s_checkinterval:		max. time between checks
// @s_creator_os:		OS
// @s_def_resuid:		Default uid for reserved blocks
// @s_def_resgid:		Default gid for reserved blocks
// @s_first_ino:		First non-reserved inode
// @s_inode_size:		Size of an inode
// @s_dat_entry_size:		Size of a dat entry
// @s_checkpoint_size:		Size of a checkpoint
// @s_segment_usage_size:	Size of a segment usage
// @s_uuid:			128-bit uuid for volume
// @s_volume_name:		volume name
// @s_c_interval:		Commit interval of segment
// @s_c_block_max:		Threshold of data amount for the
// segment construction
// @s_feature_compat:		Compatible feature set
// @s_feature_compat_ro:	Read-only compatible feature set
// @s_feature_incompat:		Incompatible feature set
// @s_reserved:			padding to the end of the block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_super_block {
// 00*/	__le32	s_rev_level;
    pub s_minor_rev_level: __le16,
    pub s_magic: __le16,
    pub s_bytes: __le16,
    pub s_flags: __le16,
    pub s_crc_seed: __le32,
// 10*/	__le32	s_sum;
    pub s_log_block_size: __le32,
    pub s_nsegments: __le64,
// 20*/	__le64  s_dev_size;
    pub s_first_data_block: __le64,
// 30*/	__le32  s_blocks_per_segment;
    pub s_r_segments_percentage: __le32,
    pub s_last_cno: __le64,
// 40*/	__le64  s_last_pseg;
    pub s_last_seq: __le64,
// 50*/	__le64	s_free_blocks_count;
    pub s_ctime: __le64,
// 60*/	__le64	s_mtime;
    pub s_wtime: __le64,
// 70*/	__le16	s_mnt_count;
    pub s_max_mnt_count: __le16,
    pub s_state: __le16,
    pub s_errors: __le16,
    pub s_lastcheck: __le64,
// 80*/	__le32	s_checkinterval;
    pub s_creator_os: __le32,
    pub s_def_resuid: __le16,
    pub s_def_resgid: __le16,
    pub s_first_ino: __le32,
// 90*/	__le16  s_inode_size;
    pub s_dat_entry_size: __le16,
    pub s_checkpoint_size: __le16,
    pub s_segment_usage_size: __le16,
// 98*/	__u8	s_uuid[16];
// A8*/	char	s_volume_name[80]	__kernel_nonstring;
// F8*/	__le32  s_c_interval;
    pub s_c_block_max: __le32,
// 100*/	__le64  s_feature_compat;
    pub s_feature_compat_ro: __le64,
    pub s_feature_incompat: __le64,
    pub s_reserved: [__u32; 186],
}

//
// Codes for operating systems
//
pub const NILFS_OS_LINUX: c_int = 0;
// Codes from 1 to 4 are reserved to keep compatibility with ext2 creator-OS
//
// Revision levels
//

//
// Feature set definitions
//
// If there is a bit set in the incompatible feature set that the kernel
// doesn't know about, it should refuse to mount the filesystem.
//
pub const NILFS_FEATURE_COMPAT_RO_BLOCK_COUNT: c_uint = 0x00000001ULL;

//
// Bytes count of super_block for CRC-calculation
//

//
// Special inode number
//

// Minimum number of blocks in
// a full segment
//

// Minimum number of blocks in
// a partial segment
//

// Minimum number of reserved
// segments
//
// We call DAT, cpfile, and sufile root metadata files.  Inodes of
// these files are written in super root block instead of ifile, and
// garbage collector doesn't keep any past versions of these files.
//

//
// bytes offset of secondary super block
//

//
// Maximal count of links to a file
//
pub const NILFS_LINK_MAX: c_int = 32000;
//
// Structure of a directory entry
// (Same as ext2)
//
pub const NILFS_NAME_LEN: c_int = 255;
//
// Block size limitations
//
pub const NILFS_MIN_BLOCK_SIZE: c_int = 1024;
pub const NILFS_MAX_BLOCK_SIZE: c_int = 65536;
//
// The new version of the directory entry.  Since V0 structures are
// stored in intel byte order, and the name_len field could never be
// bigger than 255 chars, it's safe to reclaim the extra byte for the
// file_type field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_dir_entry {
    pub /: *mut *mut __le64 inode; / Inode number,
    pub /: *mut *mut __le16 rec_len; / Directory entry length,
    pub /: *mut *mut __u8 name_len; / Name length,
    pub /: *mut *mut __u8 file_type; / Dir entry type (file, dir, etc),
    pub /: *mut *mut char name[NILFS_NAME_LEN]; / File name,
    pub pad: c_char,
}

//
// NILFS directory file types.  Only the low 3 bits are used.  The
// other bits are reserved for now.
//
// NILFS_DIR_PAD defines the directory entries boundaries
//
// NOTE: It must be a multiple of 8
//
pub const NILFS_DIR_PAD: c_int = 8;

//
// struct nilfs_finfo - file information
// @fi_ino: inode number
// @fi_cno: checkpoint number
// @fi_nblocks: number of blocks (including intermediate blocks)
// @fi_ndatablk: number of file data blocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_finfo {
    pub fi_ino: __le64,
    pub fi_cno: __le64,
    pub fi_nblocks: __le32,
    pub fi_ndatablk: __le32,
}

//
// struct nilfs_binfo_v - information on a data block (except DAT)
// @bi_vblocknr: virtual block number
// @bi_blkoff: block offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_binfo_v {
    pub bi_vblocknr: __le64,
    pub bi_blkoff: __le64,
}

//
// struct nilfs_binfo_dat - information on a DAT node block
// @bi_blkoff: block offset
// @bi_level: level
// @bi_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_binfo_dat {
    pub bi_blkoff: __le64,
    pub bi_level: __u8,
    pub bi_pad: [__u8; 7],
}

//
// union nilfs_binfo: block information
// @bi_v: nilfs_binfo_v structure
// @bi_dat: nilfs_binfo_dat structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nilfs_binfo {
    pub bi_v: nilfs_binfo_v,
    pub bi_dat: nilfs_binfo_dat,
}

//
// struct nilfs_segment_summary - segment summary header
// @ss_datasum: checksum of data
// @ss_sumsum: checksum of segment summary
// @ss_magic: magic number
// @ss_bytes: size of this structure in bytes
// @ss_flags: flags
// @ss_seq: sequence number
// @ss_create: creation timestamp
// @ss_next: next segment
// @ss_nblocks: number of blocks
// @ss_nfinfo: number of finfo structures
// @ss_sumbytes: total size of segment summary in bytes
// @ss_pad: padding
// @ss_cno: checkpoint number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_segment_summary {
    pub ss_datasum: __le32,
    pub ss_sumsum: __le32,
    pub ss_magic: __le32,
    pub ss_bytes: __le16,
    pub ss_flags: __le16,
    pub ss_seq: __le64,
    pub ss_create: __le64,
    pub ss_next: __le64,
    pub ss_nblocks: __le32,
    pub ss_nfinfo: __le32,
    pub ss_sumbytes: __le32,
    pub ss_pad: __le32,
    pub ss_cno: __le64,
// array of finfo structures
}

pub const NILFS_SEGSUM_MAGIC: c_uint = 0x1eaffa11  /* segment summary magic number */;
//
// Segment summary flags
//
pub const NILFS_SS_LOGBGN: c_uint = 0x0001  /* begins a logical segment */;
pub const NILFS_SS_LOGEND: c_uint = 0x0002  /* ends a logical segment */;
pub const NILFS_SS_SR: c_uint = 0x0004  /* has super root */;
pub const NILFS_SS_SYNDT: c_uint = 0x0008  /* includes data only updates */;
pub const NILFS_SS_GC: c_uint = 0x0010  /* segment written for cleaner operation */;
//
// struct nilfs_btree_node - header of B-tree node block
// @bn_flags: flags
// @bn_level: level
// @bn_nchildren: number of children
// @bn_pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_btree_node {
    pub bn_flags: __u8,
    pub bn_level: __u8,
    pub bn_nchildren: __le16,
    pub bn_pad: __le32,
}

// flags
pub const NILFS_BTREE_NODE_ROOT: c_uint = 0x01;
// level
pub const NILFS_BTREE_LEVEL_DATA: c_int = 0;

//
// struct nilfs_direct_node - header of built-in bmap array
// @dn_flags: flags
// @pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_direct_node {
    pub dn_flags: __u8,
    pub pad: [__u8; 7],
}

//
// struct nilfs_palloc_group_desc - block group descriptor
// @pg_nfrees: number of free entries in block group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_palloc_group_desc {
    pub pg_nfrees: __le32,
}

//
// struct nilfs_dat_entry - disk address translation entry
// @de_blocknr: block number
// @de_start: start checkpoint number
// @de_end: end checkpoint number
// @de_rsv: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_dat_entry {
    pub de_blocknr: __le64,
    pub de_start: __le64,
    pub de_end: __le64,
    pub de_rsv: __le64,
}

pub const NILFS_MIN_DAT_ENTRY_SIZE: c_int = 32;
//
// struct nilfs_snapshot_list - snapshot list
// @ssl_next: next checkpoint number on snapshot list
// @ssl_prev: previous checkpoint number on snapshot list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_snapshot_list {
    pub ssl_next: __le64,
    pub ssl_prev: __le64,
}

//
// struct nilfs_checkpoint - checkpoint structure
// @cp_flags: flags
// @cp_checkpoints_count: checkpoints count in a block
// @cp_snapshot_list: snapshot list
// @cp_cno: checkpoint number
// @cp_create: creation timestamp
// @cp_nblk_inc: number of blocks incremented by this checkpoint
// @cp_inodes_count: inodes count
// @cp_blocks_count: blocks count
// @cp_ifile_inode: inode of ifile
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_checkpoint {
    pub cp_flags: __le32,
    pub cp_checkpoints_count: __le32,
    pub cp_snapshot_list: nilfs_snapshot_list,
    pub cp_cno: __le64,
    pub cp_create: __le64,
    pub cp_nblk_inc: __le64,
    pub cp_inodes_count: __le64,
    pub cp_blocks_count: __le64,
//
// Do not change the byte offset of ifile inode.
// To keep the compatibility of the disk format,
// additional fields should be added behind cp_ifile_inode.
//
    pub cp_ifile_inode: nilfs_inode,
}

// checkpoint flags

//
// struct nilfs_cpfile_header - checkpoint file header
// @ch_ncheckpoints: number of checkpoints
// @ch_nsnapshots: number of snapshots
// @ch_snapshot_list: snapshot list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_cpfile_header {
    pub ch_ncheckpoints: __le64,
    pub ch_nsnapshots: __le64,
    pub ch_snapshot_list: nilfs_snapshot_list,
}

//
// struct nilfs_segment_usage - segment usage
// @su_lastmod: last modified timestamp
// @su_nblocks: number of blocks in segment
// @su_flags: flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_segment_usage {
    pub su_lastmod: __le64,
    pub su_nblocks: __le32,
    pub su_flags: __le32,
}

pub const NILFS_MIN_SEGMENT_USAGE_SIZE: c_int = 16;
// segment usage flag

//
// struct nilfs_sufile_header - segment usage file header
// @sh_ncleansegs: number of clean segments
// @sh_ndirtysegs: number of dirty segments
// @sh_last_alloc: last allocated segment number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_sufile_header {
    pub sh_ncleansegs: __le64,
    pub sh_ndirtysegs: __le64,
    pub sh_last_alloc: __le64,
// ...
}

