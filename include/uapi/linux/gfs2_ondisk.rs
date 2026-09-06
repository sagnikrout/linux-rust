//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gfs2_ondisk.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//
// This copyrighted material is made available to anyone wishing to use,
// modify, copy, or redistribute it subject to the terms and conditions
// of the GNU General Public License v.2.
//

pub const GFS2_MAGIC: c_uint = 0x01161970;
pub const GFS2_BASIC_BLOCK: c_int = 512;
pub const GFS2_BASIC_BLOCK_SHIFT: c_int = 9;
// Lock numbers of the LM_TYPE_NONDISK type
pub const GFS2_MOUNT_LOCK: c_int = 0;
pub const GFS2_LIVE_LOCK: c_int = 1;
pub const GFS2_FREEZE_LOCK: c_int = 2;
pub const GFS2_RENAME_LOCK: c_int = 3;
pub const GFS2_CONTROL_LOCK: c_int = 4;
pub const GFS2_MOUNTED_LOCK: c_int = 5;
// Format numbers for various metadata types
pub const GFS2_FORMAT_NONE: c_int = 0;
pub const GFS2_FORMAT_SB: c_int = 100;
pub const GFS2_FORMAT_RG: c_int = 200;
pub const GFS2_FORMAT_RB: c_int = 300;
pub const GFS2_FORMAT_DI: c_int = 400;
pub const GFS2_FORMAT_IN: c_int = 500;
pub const GFS2_FORMAT_LF: c_int = 600;
pub const GFS2_FORMAT_JD: c_int = 700;
pub const GFS2_FORMAT_LH: c_int = 800;
pub const GFS2_FORMAT_LD: c_int = 900;
pub const GFS2_FORMAT_LB: c_int = 1000;
pub const GFS2_FORMAT_EA: c_int = 1600;
pub const GFS2_FORMAT_ED: c_int = 1700;
pub const GFS2_FORMAT_QC: c_int = 1400;
// These are format numbers for entities contained in files
pub const GFS2_FORMAT_RI: c_int = 1100;
pub const GFS2_FORMAT_DE: c_int = 1200;
pub const GFS2_FORMAT_QU: c_int = 1500;
// These are part of the superblock
pub const GFS2_FORMAT_FS: c_int = 1802;
pub const GFS2_FORMAT_MULTI: c_int = 1900;
//
// An on-disk inode number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_inum {
    pub no_formal_ino: __be64,
    pub no_addr: __be64,
}

//
// Generic metadata head structure
// Every inplace buffer logged in the journal must start with this.
//
pub const GFS2_METATYPE_NONE: c_int = 0;
pub const GFS2_METATYPE_SB: c_int = 1;
pub const GFS2_METATYPE_RG: c_int = 2;
pub const GFS2_METATYPE_RB: c_int = 3;
pub const GFS2_METATYPE_DI: c_int = 4;
pub const GFS2_METATYPE_IN: c_int = 5;
pub const GFS2_METATYPE_LF: c_int = 6;
pub const GFS2_METATYPE_JD: c_int = 7;
pub const GFS2_METATYPE_LH: c_int = 8;
pub const GFS2_METATYPE_LD: c_int = 9;
pub const GFS2_METATYPE_LB: c_int = 12;
pub const GFS2_METATYPE_EA: c_int = 10;
pub const GFS2_METATYPE_ED: c_int = 11;
pub const GFS2_METATYPE_QC: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_meta_header {
    pub mh_magic: __be32,
    pub mh_type: __be32,
    pub /: *mut *mut __be64 __pad0; / Was generation number in gfs1,
    pub mh_format: __be32,
// This union is to keep userspace happy
    pub /: *mut *mut __be32 mh_jid; / Was incarnation number in gfs1,
    pub __pad1: __be32,
}

//
// super-block structure
//
// It's probably good if SIZEOF_SB <= GFS2_BASIC_BLOCK (512 bytes)
//
// Order is important, need to be able to read old superblocks to do on-disk
// version upgrades.
//
// Address of superblock in GFS2 basic blocks
pub const GFS2_SB_ADDR: c_int = 128;
// The lock number for the superblock (must be zero)
pub const GFS2_SB_LOCK: c_int = 0;
// Requirement:  GFS2_LOCKNAME_LEN % 8 == 0
pub const GFS2_LOCKNAME_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_sb {
    pub sb_header: gfs2_meta_header,
    pub sb_fs_format: __be32,
    pub sb_multihost_format: __be32,
    pub /: *mut *mut __u32 __pad0; / Was superblock flags in gfs1,
    pub sb_bsize: __be32,
    pub sb_bsize_shift: __be32,
    pub /: *mut *mut __u32 __pad1; / Was journal segment size in gfs1,
    pub /: *mut *mut gfs2_inum sb_master_dir; / Was jindex dinode in gfs1,
    pub /: *mut *mut gfs2_inum __pad2; / Was rindex dinode in gfs1,
    pub sb_root_dir: gfs2_inum,
    pub sb_lockproto: [c_char; GFS2_LOCKNAME_LEN],
    pub sb_locktable: [c_char; GFS2_LOCKNAME_LEN],
    pub /: *mut *mut gfs2_inum __pad3; / Was quota inode in gfs1,
    pub /: *mut *mut gfs2_inum __pad4; / Was licence inode in gfs1,
pub const GFS2_HAS_UUID: c_int = 1;
    pub /: *mut *mut __u8 sb_uuid[16]; / The UUID, maybe 0 for backwards compat,
}

//
// resource index structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_rindex {
    pub /: *mut *mut __be64 ri_addr; / grp block disk address,
    pub /: *mut *mut __be32 ri_length; / length of rgrp header in fs blocks,
    pub __pad: __u32,
    pub /: *mut *mut __be64 ri_data0; / first data location,
    pub /: *mut *mut __be32 ri_data; / num of data blocks in rgrp,
    pub /: *mut *mut __be32 ri_bitbytes; / number of bytes in data bitmaps,
    pub ri_reserved: [__u8; 64],
}

//
// resource group header structure
//
// Number of blocks per byte in rgrp
pub const GFS2_NBBY: c_int = 4;
pub const GFS2_BIT_SIZE: c_int = 2;
pub const GFS2_BIT_MASK: c_uint = 0x00000003;
pub const GFS2_BLKST_FREE: c_int = 0;
pub const GFS2_BLKST_USED: c_int = 1;
pub const GFS2_BLKST_UNLINKED: c_int = 2;
pub const GFS2_BLKST_DINODE: c_int = 3;
pub const GFS2_RGF_JOURNAL: c_uint = 0x00000001;
pub const GFS2_RGF_METAONLY: c_uint = 0x00000002;
pub const GFS2_RGF_DATAONLY: c_uint = 0x00000004;
pub const GFS2_RGF_NOALLOC: c_uint = 0x00000008;
pub const GFS2_RGF_TRIMMED: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_inode_lvb {
    pub ri_magic: __be32,
    pub __pad: __be32,
    pub ri_generation_deleted: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_rgrp_lvb {
    pub rl_magic: __be32,
    pub rl_flags: __be32,
    pub rl_free: __be32,
    pub rl_dinodes: __be32,
    pub rl_igeneration: __be64,
    pub rl_unlinked: __be32,
    pub __pad: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_rgrp {
    pub rg_header: gfs2_meta_header,
    pub rg_flags: __be32,
    pub rg_free: __be32,
    pub rg_dinodes: __be32,
    pub __pad: __be32,
    pub /: *mut *mut __be32 rg_skip; / Distance to the next rgrp in fs blocks,
}

// The following 3 fields are duplicated from gfs2_rindex to reduce
//
// quota structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_quota {
    pub qu_limit: __be64,
    pub qu_warn: __be64,
    pub qu_value: __be64,
    pub qu_reserved: [__u8; 64],
}

//
// dinode structure
//
pub const GFS2_MAX_META_HEIGHT: c_int = 10;
pub const GFS2_DIR_MAX_DEPTH: c_int = 17;

// Dinode flags
pub const GFS2_DIF_JDATA: c_uint = 0x00000001;
pub const GFS2_DIF_EXHASH: c_uint = 0x00000002;
pub const GFS2_DIF_UNUSED: c_uint = 0x00000004  /* only in gfs1 */;
pub const GFS2_DIF_EA_INDIRECT: c_uint = 0x00000008;
pub const GFS2_DIF_DIRECTIO: c_uint = 0x00000010;
pub const GFS2_DIF_IMMUTABLE: c_uint = 0x00000020;
pub const GFS2_DIF_APPENDONLY: c_uint = 0x00000040;
pub const GFS2_DIF_NOATIME: c_uint = 0x00000080;
pub const GFS2_DIF_SYNC: c_uint = 0x00000100;
pub const GFS2_DIF_SYSTEM: c_uint = 0x00000200 /* New in gfs2 */;
pub const GFS2_DIF_TOPDIR: c_uint = 0x00000400 /* New in gfs2 */;
pub const GFS2_DIF_TRUNC_IN_PROG: c_uint = 0x20000000 /* New in gfs2 */;
pub const GFS2_DIF_INHERIT_DIRECTIO: c_uint = 0x40000000 /* only in gfs1 */;
pub const GFS2_DIF_INHERIT_JDATA: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_dinode {
    pub di_header: gfs2_meta_header,
    pub di_num: gfs2_inum,
    pub /: *mut *mut __be32 di_mode; / mode of file,
    pub /: *mut *mut __be32 di_uid; / owner's user id,
    pub /: *mut *mut __be32 di_gid; / owner's group id,
    pub /: *mut *mut __be32 di_nlink; / number of links to this file,
    pub /: *mut *mut __be64 di_size; / number of bytes in file,
    pub /: *mut *mut __be64 di_blocks; / number of blocks in file,
    pub /: *mut *mut __be64 di_atime; / time last accessed,
    pub /: *mut *mut __be64 di_mtime; / time last modified,
    pub /: *mut *mut __be64 di_ctime; / time last changed,
    pub /: *mut *mut __be32 di_major; / device major number,
    pub /: *mut *mut __be32 di_minor; / device minor number,
// This section varies from gfs1. Padding added to align with
// remainder of dinode
//
    pub /: *mut *mut __be64 di_goal_meta; / rgrp to alloc from next,
    pub /: *mut *mut __be64 di_goal_data; / data block goal,
    pub /: *mut *mut __be64 di_generation; / generation number for NFS,
    pub /: *mut *mut __be32 di_flags; / GFS2_DIF_...,
    pub /: *mut *mut __be32 di_payload_format; / GFS2_FORMAT_...,
    pub /: *mut *mut __u16 __pad1; / Was ditype in gfs1,
    pub /: *mut *mut __be16 di_height; / height of metadata,
    pub /: *mut *mut __u32 __pad2; / Unused incarnation number from gfs1,
// These only apply to directories
    pub /: *mut *mut __u16 __pad3; / Padding,
    pub /: *mut *mut __be16 di_depth; / Number of bits in the table,
    pub /: *mut *mut __be32 di_entries; / The number of entries in the directory,
    pub /: *mut *mut gfs2_inum __pad4; / Unused even in current gfs1,
    pub /: *mut *mut __be64 di_eattr; / extended attribute block number,
    pub /: *mut *mut __be32 di_atime_nsec; / nsec portion of atime,
    pub /: *mut *mut __be32 di_mtime_nsec; / nsec portion of mtime,
    pub /: *mut *mut __be32 di_ctime_nsec; / nsec portion of ctime,
    pub di_reserved: [__u8; 44],
}

//
// directory structure - many of these per directory file
//
pub const GFS2_FNAMESIZE: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_dirent {
    pub de_inum: gfs2_inum,
    pub de_hash: __be32,
    pub de_rec_len: __be16,
    pub de_name_len: __be16,
    pub de_type: __be16,
    pub de_rahead: __be16,
    pub __pad: [__u8; 12],
    pub /: *mut *mut __u32 de_cookie; / ondisk value not used,
    pub pad3: [__u8; 8],
}

//
// Header of leaf directory nodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_leaf {
    pub lf_header: gfs2_meta_header,
    pub /: *mut *mut __be16 lf_depth; / Depth of leaf,
    pub /: *mut *mut __be16 lf_entries; / Number of dirents in leaf,
    pub /: *mut *mut __be32 lf_dirent_format; / Format of the dirents,
    pub /: *mut *mut __be64 lf_next; / Next leaf, if overflow,
    pub lf_reserved: [__u8; 64],
    pub /: *mut *mut __be64 lf_inode; / Dir inode number,
    pub /: *mut *mut __be32 lf_dist; / Dist from inode on chain,
    pub /: *mut *mut __be32 lf_nsec; / Last ins/del usecs,
    pub /: *mut *mut __be64 lf_sec; / Last ins/del in secs,
    pub lf_reserved2: [__u8; 40],
}

//
// Extended attribute header format
//
// This works in a similar way to dirents. There is a fixed size header
// followed by a variable length section made up of the name and the
// associated data. In the case of a "stuffed" entry, the value is
// inline directly after the name, the ea_num_ptrs entry will be
// zero in that case. For non-"stuffed" entries, there will be
// a set of pointers (aligned to 8 byte boundary) to the block(s)
// containing the value.
//
// The blocks containing the values and the blocks containing the
// extended attribute headers themselves all start with the common
// metadata header. Each inode, if it has extended attributes, will
// have either a single block containing the extended attribute headers
// or a single indirect block pointing to blocks containing the
// extended attribute headers.
//
// The maximum size of the data part of an extended attribute is 64k
// so the number of blocks required depends upon block size. Since the
// block size also determines the number of pointers in an indirect
// block, its a fairly complicated calculation to work out the maximum
// number of blocks that an inode may have relating to extended attributes.
//
pub const GFS2_EA_MAX_NAME_LEN: c_int = 255;
pub const GFS2_EA_MAX_DATA_LEN: c_int = 65536;
pub const GFS2_EATYPE_UNUSED: c_int = 0;
pub const GFS2_EATYPE_USR: c_int = 1;
pub const GFS2_EATYPE_SYS: c_int = 2;
pub const GFS2_EATYPE_SECURITY: c_int = 3;
pub const GFS2_EATYPE_TRUSTED: c_int = 4;
pub const GFS2_EATYPE_LAST: c_int = 4;

pub const GFS2_EAFLAG_LAST: c_uint = 0x01	/* last ea in block */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_ea_header {
    pub ea_rec_len: __be32,
    pub ea_data_len: __be32,
    pub /: *mut *mut __u8 ea_name_len; / no NULL pointer after the string,
    pub /: *mut *mut __u8 ea_type; / GFS2_EATYPE_...,
    pub /: *mut *mut __u8 ea_flags; / GFS2_EAFLAG_...,
    pub ea_num_ptrs: __u8,
    pub __pad: __u32,
}

//
// Log header structure
//
pub const GFS2_LOG_HEAD_UNMOUNT: c_uint = 0x00000001 /* log is clean */;
pub const GFS2_LOG_HEAD_FLUSH_NORMAL: c_uint = 0x00000002 /* normal log flush */;
pub const GFS2_LOG_HEAD_FLUSH_SYNC: c_uint = 0x00000004 /* Sync log flush */;
pub const GFS2_LOG_HEAD_FLUSH_SHUTDOWN: c_uint = 0x00000008 /* Shutdown log flush */;
pub const GFS2_LOG_HEAD_FLUSH_FREEZE: c_uint = 0x00000010 /* Freeze flush */;
pub const GFS2_LOG_HEAD_RECOVERY: c_uint = 0x00000020 /* Journal recovery */;
pub const GFS2_LOG_HEAD_USERSPACE: c_uint = 0x80000000 /* Written by gfs2-utils */;
// Log flush callers
pub const GFS2_LFC_SHUTDOWN: c_uint = 0x00000100;
pub const GFS2_LFC_JDATA_WPAGES: c_uint = 0x00000200;
pub const GFS2_LFC_SET_FLAGS: c_uint = 0x00000400;
pub const GFS2_LFC_AIL_EMPTY_GL: c_uint = 0x00000800;
pub const GFS2_LFC_AIL_FLUSH: c_uint = 0x00001000;
pub const GFS2_LFC_RGRP_GO_SYNC: c_uint = 0x00002000;
pub const GFS2_LFC_INODE_GO_SYNC: c_uint = 0x00004000;
pub const GFS2_LFC_INODE_GO_INVAL: c_uint = 0x00008000;
pub const GFS2_LFC_FREEZE_GO_SYNC: c_uint = 0x00010000;
pub const GFS2_LFC_KILL_SB: c_uint = 0x00020000;
pub const GFS2_LFC_DO_SYNC: c_uint = 0x00040000;
pub const GFS2_LFC_INPLACE_RESERVE: c_uint = 0x00080000;
pub const GFS2_LFC_WRITE_INODE: c_uint = 0x00100000;
pub const GFS2_LFC_MAKE_FS_RO: c_uint = 0x00200000;
pub const GFS2_LFC_SYNC_FS: c_uint = 0x00400000;
pub const GFS2_LFC_EVICT_INODE: c_uint = 0x00800000;
pub const GFS2_LFC_TRANS_END: c_uint = 0x01000000;
pub const GFS2_LFC_LOGD_JFLUSH_REQD: c_uint = 0x02000000;
pub const GFS2_LFC_LOGD_AIL_FLUSH_REQD: c_uint = 0x04000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_log_header {
    pub lh_header: gfs2_meta_header,
    pub /: *mut *mut __be64 lh_sequence; / Sequence number of this transaction,
    pub /: *mut *mut __be32 lh_flags; / GFS2_LOG_HEAD_...,
    pub /: *mut *mut __be32 lh_tail; / Block number of log tail,
    pub lh_blkno: __be32,
    pub /: *mut *mut __be32 lh_hash; / crc up to here with this field 0,
// Version 2 additional fields start here
    pub /: *mut *mut __be32 lh_crc; / crc32c from lh_nsec to end of block,
    pub /: *mut *mut __be32 lh_nsec; / Nanoseconds of timestamp,
    pub /: *mut *mut __be64 lh_sec; / Seconds of timestamp,
    pub /: *mut *mut __be64 lh_addr; / Block addr of this log header (absolute),
    pub /: *mut *mut __be64 lh_jinode; / Journal inode number,
    pub /: *mut *mut __be64 lh_statfs_addr; / Local statfs inode number,
    pub /: *mut *mut __be64 lh_quota_addr; / Local quota change inode number,
// Statfs local changes (i.e. diff from global statfs)
    pub lh_local_total: __be64,
    pub lh_local_free: __be64,
    pub lh_local_dinodes: __be64,
}

//
// Log type descriptor
//
pub const GFS2_LOG_DESC_METADATA: c_int = 300;
// ld_data1 is the number of metadata blocks in the descriptor.
pub const GFS2_LOG_DESC_REVOKE: c_int = 301;
// ld_data1 is the number of revoke blocks in the descriptor.
pub const GFS2_LOG_DESC_JDATA: c_int = 302;
// ld_data1 is the number of data blocks in the descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_log_descriptor {
    pub ld_header: gfs2_meta_header,
    pub /: *mut *mut __be32 ld_type; / GFS2_LOG_DESC_...,
    pub /: *mut *mut __be32 ld_length; / Number of buffers in this chunk,
    pub /: *mut *mut __be32 ld_data1; / descriptor-specific field,
    pub /: *mut *mut __be32 ld_data2; / descriptor-specific field,
    pub ld_reserved: [__u8; 32],
}

//
// Inum Range
// Describe a range of formal inode numbers allocated to
// one machine to assign to inodes.
//
pub const GFS2_INUM_QUANTUM: c_int = 1048576;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_inum_range {
    pub ir_start: __be64,
    pub ir_length: __be64,
}

//
// Statfs change
// Describes an change to the pool of free and allocated
// blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_statfs_change {
    pub sc_total: __be64,
    pub sc_free: __be64,
    pub sc_dinodes: __be64,
}

//
// Quota change
// Describes an allocation change for a particular
// user or group.
//
pub const GFS2_QCF_USER: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_quota_change {
    pub qc_change: __be64,
    pub /: *mut *mut __be32 qc_flags; / GFS2_QCF_...,
    pub qc_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfs2_quota_lvb {
    pub qb_magic: __be32,
    pub __pad: __u32,
    pub /: *mut *mut __be64 qb_limit; / Hard limit of # blocks to alloc,
    pub /: *mut *mut __be64 qb_warn; / Warn user when alloc is above this #,
    pub /: *mut *mut __be64 qb_value; / Current # blocks allocated,
}
