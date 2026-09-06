//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_log_format.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// On-disk Log Format definitions.
//
// This file contains all the on-disk format definitions used within the log. It
// includes the physical log structure itself, as well as all the log item
// format structures that are written into the log and intepreted by log
// recovery. We start with the physical log format definitions, and then work
// through all the log items definitions and everything they encode into the
// log.
//
pub type xlog_tid_t = u32;
pub const XLOG_MIN_ICLOGS: c_int = 2;
pub const XLOG_MAX_ICLOGS: c_int = 8;
pub const XLOG_HEADER_MAGIC_NUM: c_uint = 0xFEEDbabe	/* Invalid cycle number */;
pub const XLOG_VERSION_1: c_int = 1;

pub const XLOG_HEADER_SIZE: c_int = 512;
// Minimum number of transactions that must fit in the log (defined by mkfs)
pub const XFS_MIN_LOG_FACTOR: c_int = 3;

// get lsn fields

//
// By comparing each component, we don't have to worry about extra endian issues
// in treating two 32 bit numbers as one 64 bit number
//
// this is used in a spot where we might otherwise double-endian-flip

extern "C" {
    pub fn be32_to_cpu(1): *mut *mut *mut ((__be32 )ptr +) -> return;
}
extern "C" {
    pub fn be32_to_cpu()ptr: *mut *mut (__be32) -> return;
}
// Log Clients
pub const XFS_TRANSACTION: c_uint = 0x69;
pub const XFS_LOG: c_uint = 0xaa;
pub const XLOG_UNMOUNT_TYPE: c_uint = 0x556e	/* Un for Unmount */;
//
// Log item for unmount records.
//
// The unmount record used to have a string "Unmount filesystem--" in the
// data section where the "Un" was really a magic number (XLOG_UNMOUNT_TYPE).
// We just write the magic number now; see xfs_log_unmount_write.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_unmount_log_format {
    pub /: *mut *mut uint16_t magic; / XLOG_UNMOUNT_TYPE,
    pub pad1: u16,
    pub /: *mut *mut uint32_t pad2; / may as well make it 64 bits,
}

//
// Flags to log operation header
//
// The first write of a new transaction will be preceded with a start
// record, XLOG_START_TRANS.  Once a transaction is committed, a commit
// record is written, XLOG_COMMIT_TRANS.  If a single region can not fit into
// the remainder of the current active in-core log, it is split up into
// multiple regions.  Each partial region will be marked with a
// XLOG_CONTINUE_TRANS until the last one, which gets marked with XLOG_END_TRANS.
//
pub const XLOG_START_TRANS: c_uint = 0x01	/* Start a new transaction */;
pub const XLOG_COMMIT_TRANS: c_uint = 0x02	/* Commit this transaction */;
pub const XLOG_CONTINUE_TRANS: c_uint = 0x04	/* Cont this trans into new region */;
pub const XLOG_WAS_CONT_TRANS: c_uint = 0x08	/* Cont this trans into new region */;
pub const XLOG_END_TRANS: c_uint = 0x10	/* End a continued transaction */;
pub const XLOG_UNMOUNT_TRANS: c_uint = 0x20	/* Unmount a filesystem transaction */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_op_header {
    pub /: *mut *mut __be32 oh_tid; / transaction id of operation : 4 b,
    pub /: *mut *mut __be32 oh_len; / bytes in data region : 4 b,
    pub /: *mut *mut __u8 oh_clientid; / who sent me this : 1 b,
    pub /: *mut *mut __u8 oh_flags; / : 1 b,
    pub /: *mut *mut __u16 oh_res2; / 32 bit align : 2 b,
}

// valid values for h_fmt
pub const XLOG_FMT_UNKNOWN: c_int = 0;
pub const XLOG_FMT_LINUX_LE: c_int = 1;
pub const XLOG_FMT_LINUX_BE: c_int = 2;
pub const XLOG_FMT_IRIX_BE: c_int = 3;
// our fmt

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_rec_ext_header {
    pub /: *mut *mut __be32 xh_cycle; / write cycle of log,
    pub xh_cycle_data: [__be32; XLOG_CYCLE_DATA_SIZE],
    pub xh_reserved: [__u8; 252],
}

// actual ext header payload size for checksumming

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_rec_header {
    pub /: *mut *mut __be32 h_magicno; / log record (LR) identifier : 4,
    pub /: *mut *mut __be32 h_cycle; / write cycle of log : 4,
    pub /: *mut *mut __be32 h_version; / LR version : 4,
    pub /: *mut *mut __be32 h_len; / len in bytes; should be 64-bit aligned: 4,
    pub /: *mut *mut __be64 h_lsn; / lsn of this LR : 8,
    pub /: *mut *mut __be64 h_tail_lsn; / lsn of 1st LR w/ buffers not committed: 8,
    pub /: *mut *mut __le32 h_crc; / crc of log record : 4,
    pub /: *mut *mut __be32 h_prev_block; / block number to previous LR : 4,
    pub /: *mut *mut __be32 h_num_logops; / number of log operations in this LR : 4,
    pub h_cycle_data: [__be32; XLOG_CYCLE_DATA_SIZE],
// fields added by the Linux port:
    pub /: *mut *mut __be32 h_fmt; / format of log record : 4,
    pub /: *mut *mut uuid_t h_fs_uuid; / uuid of FS : 16,
// fields added for log v2:
    pub /: *mut *mut __be32 h_size; / iclog size : 4,
//
// When h_size added for log v2 support, it caused structure to have
// a different size on i386 vs all other architectures because the
// sum of the size ofthe  member is not aligned by that of the largest
// __be64-sized member, and i386 has really odd struct alignment rules.
//
// Due to the way the log headers are placed out on-disk that alone is
// not a problem becaue the xlog_rec_header always sits alone in a
// BBSIZEs area, and the rest of that area is padded with zeroes.
// But xlog_cksum used to calculate the checksum based on the structure
// size, and thus gives different checksums for i386 vs the rest.
// We now do two checksum validation passes for both sizes to allow
// moving v5 file systems with unclean logs between i386 and other
// (little-endian) architectures.
//
    pub h_pad0: __u32,
    pub h_reserved: [__u8; 184],
    pub h_ext: [xlog_rec_ext_header; ],
}

//
// Transaction Header definitions.
//
// This is the structure written in the log at the head of every transaction. It
// identifies the type and id of the transaction, and contains the number of
// items logged by the transaction so we know how many to expect during
// recovery.
//
// Do not change the below structure without redoing the code in
// xlog_recover_add_to_trans() and xlog_recover_add_to_cont_trans().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_trans_header {
    pub /: *mut *mut uint th_magic; / magic number,
    pub /: *mut *mut uint th_type; / transaction type,
    pub /: *mut *mut int32_t th_tid; / transaction id (unused),
    pub /: *mut *mut uint th_num_items; / num items logged by trans,
}

pub const XFS_TRANS_HEADER_MAGIC: c_uint = 0x5452414e	/* TRAN */;
//
// The only type valid for th_type in CIL-enabled file system logs:
//
pub const XFS_TRANS_CHECKPOINT: c_int = 40;
//
// Log item types.
//
pub const XFS_LI_EFI: c_uint = 0x1236;
pub const XFS_LI_EFD: c_uint = 0x1237;
pub const XFS_LI_IUNLINK: c_uint = 0x1238;
pub const XFS_LI_INODE: c_uint = 0x123b	/* aligned ino chunks, var-size ibufs */;
pub const XFS_LI_BUF: c_uint = 0x123c	/* v2 bufs, variable sized inode bufs */;
pub const XFS_LI_DQUOT: c_uint = 0x123d;
pub const XFS_LI_QUOTAOFF: c_uint = 0x123e;
pub const XFS_LI_ICREATE: c_uint = 0x123f;
pub const XFS_LI_RUI: c_uint = 0x1240	/* rmap update intent */;
pub const XFS_LI_RUD: c_uint = 0x1241;
pub const XFS_LI_CUI: c_uint = 0x1242	/* refcount update intent */;
pub const XFS_LI_CUD: c_uint = 0x1243;
pub const XFS_LI_BUI: c_uint = 0x1244	/* bmbt update intent */;
pub const XFS_LI_BUD: c_uint = 0x1245;
pub const XFS_LI_ATTRI: c_uint = 0x1246  /* attr set/remove intent*/;
pub const XFS_LI_ATTRD: c_uint = 0x1247  /* attr set/remove done */;
pub const XFS_LI_XMI: c_uint = 0x1248  /* mapping exchange intent */;
pub const XFS_LI_XMD: c_uint = 0x1249  /* mapping exchange done */;
pub const XFS_LI_EFI_RT: c_uint = 0x124a	/* realtime extent free intent */;
pub const XFS_LI_EFD_RT: c_uint = 0x124b	/* realtime extent free done */;
pub const XFS_LI_RUI_RT: c_uint = 0x124c	/* realtime rmap update intent */;
pub const XFS_LI_RUD_RT: c_uint = 0x124d	/* realtime rmap update done */;
pub const XFS_LI_CUI_RT: c_uint = 0x124e	/* realtime refcount update intent */;
pub const XFS_LI_CUD_RT: c_uint = 0x124f	/* realtime refcount update done */;

//
// Inode Log Item Format definitions.
//
// This is the structure used to lay out an inode log item in the
// log.  The size of the inline data/extents/b-tree root to be logged
// (if any) is indicated in the ilf_dsize field.  Changes to this structure
// must be added on to the end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inode_log_format {
    pub /: *mut *mut uint16_t ilf_type; / inode log item type,
    pub /: *mut *mut uint16_t ilf_size; / size of this item,
    pub /: *mut *mut uint32_t ilf_fields; / flags for fields logged,
    pub /: *mut *mut uint16_t ilf_asize; / size of attr d/ext/root,
    pub /: *mut *mut uint16_t ilf_dsize; / size of data/ext/root,
    pub /: *mut *mut uint32_t ilf_pad; / pad for 64 bit boundary,
    pub /: *mut *mut uint64_t ilf_ino; / inode number,
    pub inode*/: *mut *mut uint32_t ilfu_rdev; / rdev value for dev,
    pub /: *mut *mut uint8_t __pad[16]; / unused,
    pub ilf_u: },
    pub /: *mut *mut int64_t ilf_blkno; / blkno of inode buffer,
    pub /: *mut *mut int32_t ilf_len; / len of inode buffer,
    pub /: *mut *mut int32_t ilf_boffset; / off of inode in buffer,
}

//
// Old 32 bit systems will log in this format without the 64 bit
// alignment padding. Recovery will detect this and convert it to the
// correct format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inode_log_format_32 {
    pub /: *mut *mut uint16_t ilf_type; / inode log item type,
    pub /: *mut *mut uint16_t ilf_size; / size of this item,
    pub /: *mut *mut uint32_t ilf_fields; / flags for fields logged,
    pub /: *mut *mut uint16_t ilf_asize; / size of attr d/ext/root,
    pub /: *mut *mut uint16_t ilf_dsize; / size of data/ext/root,
    pub /: *mut *mut uint64_t ilf_ino; / inode number,
    pub inode*/: *mut *mut uint32_t ilfu_rdev; / rdev value for dev,
    pub /: *mut *mut uint8_t __pad[16]; / unused,
    pub ilf_u: },
    pub /: *mut *mut int64_t ilf_blkno; / blkno of inode buffer,
    pub /: *mut *mut int32_t ilf_len; / len of inode buffer,
    pub /: *mut *mut int32_t ilf_boffset; / off of inode in buffer,
    pub __attribute__((packed)): },
//
// Flags for xfs_trans_log_inode flags field.
//
pub const XFS_ILOG_CORE: c_uint = 0x001	/* log standard inode fields */;
pub const XFS_ILOG_DDATA: c_uint = 0x002	/* log i_df.if_data */;
pub const XFS_ILOG_DEXT: c_uint = 0x004	/* log i_df.if_extents */;
pub const XFS_ILOG_DBROOT: c_uint = 0x008	/* log i_df.i_broot */;
pub const XFS_ILOG_DEV: c_uint = 0x010	/* log the dev field */;
pub const XFS_ILOG_UUID: c_uint = 0x020	/* added long ago, but never used */;
pub const XFS_ILOG_ADATA: c_uint = 0x040	/* log i_af.if_data */;
pub const XFS_ILOG_AEXT: c_uint = 0x080	/* log i_af.if_extents */;
pub const XFS_ILOG_ABROOT: c_uint = 0x100	/* log i_af.i_broot */;
pub const XFS_ILOG_DOWNER: c_uint = 0x200	/* change the data fork owner on replay */;
pub const XFS_ILOG_AOWNER: c_uint = 0x400	/* change the attr fork owner on replay */;
//
// The timestamps are dirty, but not necessarily anything else in the inode
// core.  Unlike the other fields above this one must never make it to disk
// in the ilf_fields of the inode_log_format, but is purely store in-memory in
// ili_fields in the inode_log_item.
//
pub const XFS_ILOG_TIMESTAMP: c_uint = 0x4000;
//
// The version field has been changed, but not necessarily anything else of
// interest. This must never make it to disk - it is used purely to ensure that
// the inode item ->precommit operation can update the fsync flag triggers
// in the inode item correctly.
//
pub const XFS_ILOG_IVERSION: c_uint = 0x8000;

    pub XFS_ILOG_ABROOT): return (w == XFS_DATA_FORK ? XFS_ILOG_DBROOT :,
    pub XFS_ILOG_AEXT): return (w == XFS_DATA_FORK ? XFS_ILOG_DEXT :,
    pub XFS_ILOG_ADATA): return (w == XFS_DATA_FORK ? XFS_ILOG_DDATA :,
//
// Incore version of the on-disk inode core structures. We log this directly
// into the journal in host CPU format (for better or worse) and as such
// directly mirrors the xfs_dinode structure as it must contain all the same
// information.
//
pub type xfs_log_timestamp_t = u64;
// Legacy timestamp encoding format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_log_legacy_timestamp {
    pub /: *mut *mut int32_t t_sec; / timestamp seconds,
    pub /: *mut *mut int32_t t_nsec; / timestamp nanoseconds,
}

//
// Define the format of the inode core that is logged. This structure must be
// kept identical to struct xfs_dinode except for the endianness annotations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_log_dinode {
    pub /: *mut *mut uint16_t di_magic; / inode magic # = XFS_DINODE_MAGIC,
    pub /: *mut *mut uint16_t di_mode; / mode and type of file,
    pub /: *mut *mut int8_t di_version; / inode version,
    pub /: *mut *mut int8_t di_format; / format of di_c data,
    pub /: *mut *mut uint16_t di_metatype; / metadata type, if DIFLAG2_METADATA,
    pub /: *mut *mut uint32_t di_uid; / owner's user id,
    pub /: *mut *mut uint32_t di_gid; / owner's group id,
    pub /: *mut *mut uint32_t di_nlink; / number of links to file,
    pub /: *mut *mut uint16_t di_projid_lo; / lower part of owner's project id,
    pub /: *mut *mut uint16_t di_projid_hi; / higher part of owner's project id,
// Number of data fork extents if NREXT64 is set
    pub di_big_nextents: u64,
// Padding for V3 inodes without NREXT64 set.
    pub di_v3_pad: u64,
// Padding and inode flush counter for V2 inodes.
    pub /: *mut *mut uint8_t di_v2_pad[6]; / V2 inode zeroed space,
    pub /: *mut *mut uint16_t di_flushiter; / V2 inode incremented on flush,
}

//
// For V2 inodes and V3 inodes without NREXT64 set, this
// is the number of data and attr fork extents.
//
// Number of attr fork extents if NREXT64 is set.
// di_next_unlinked is the only non-core field in the old dinode
// start of the extended dinode, writable fields
//
// The LSN we write to this field during formatting is not a reflection
// of the current on-disk LSN. It should never be used for recovery
// sequencing, nor should it be recovered into the on-disk inode at all.
// See xlog_recover_inode_commit_pass2() and xfs_log_dinode_to_disk()
// for details.
//
// basic cow extent size for (regular) file
// used blocks in RTG for (zoned) rtrmap inode
// fields only written to during inode creation
// structure must be padded to 64 bit alignment

//
// Buffer Log Format definitions
//
// These are the physical dirty bitmap definitions for the log format structure.
//
pub const XFS_BLF_CHUNK: c_int = 128;
pub const XFS_BLF_SHIFT: c_int = 7;
pub const BIT_TO_WORD_SHIFT: c_int = 5;

//
// This flag indicates that the buffer contains on disk inodes
// and requires special recovery handling.
//

//
// This flag indicates that the buffer should not be replayed
// during recovery because its blocks are being freed.
//

//
// This flag indicates that the buffer contains on disk
// user or group dquots and may require special recovery handling.
//

//
// This is the structure used to lay out a buf log item in the log.  The data
// map describes which 128 byte chunks of the buffer have been logged.
//
// The placement of blf_map_size causes blf_data_map to start at an odd
// multiple of sizeof(unsigned int) offset within the struct.  Because the data
// bitmap size will always be an even number, the end of the data_map (and
// therefore the structure) will also be at an odd multiple of sizeof(unsigned
// int).  Some 64-bit compilers will insert padding at the end of the struct to
// ensure 64-bit alignment of blf_blkno, but 32-bit ones will not.  Therefore,
// XFS_BLF_DATAMAP_SIZE must be an odd number to make the padding explicit and
// keep the structure size consistent between 32-bit and 64-bit platforms.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buf_log_format {
    pub /: *mut *mut unsigned short blf_type; / buf log item type indicator,
    pub /: *mut *mut unsigned short blf_size; / size of this item,
    pub /: *mut *mut unsigned short blf_flags; / misc state,
    pub /: *mut *mut unsigned short blf_len; / number of blocks in this buf,
    pub /: *mut *mut int64_t blf_blkno; / starting blkno of this buf,
    pub /: *mut *mut unsigned int blf_map_size; / used size of data bitmap in words,
    pub /: *mut *mut unsigned int blf_data_map[XFS_BLF_DATAMAP_SIZE]; / dirty bitmap,
}

//
// All buffers now need to tell recovery where the magic number
// is so that it can verify and calculate the CRCs on the buffer correctly
// once the changes have been replayed into the buffer.
//
// The type value is held in the upper 5 bits of the blf_flags field, which is
// an unsigned 16 bit field. Hence we need to shift it 11 bits up and down.
//
pub const XFS_BLFT_BITS: c_int = 5;
pub const XFS_BLFT_SHIFT: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_blft {
    XFS_BLFT_UNKNOWN_BUF = 0,
    XFS_BLFT_UDQUOT_BUF,
    XFS_BLFT_PDQUOT_BUF,
    XFS_BLFT_GDQUOT_BUF,
    XFS_BLFT_BTREE_BUF,
    XFS_BLFT_AGF_BUF,
    XFS_BLFT_AGFL_BUF,
    XFS_BLFT_AGI_BUF,
    XFS_BLFT_DINO_BUF,
    XFS_BLFT_SYMLINK_BUF,
    XFS_BLFT_DIR_BLOCK_BUF,
    XFS_BLFT_DIR_DATA_BUF,
    XFS_BLFT_DIR_FREE_BUF,
    XFS_BLFT_DIR_LEAF1_BUF,
    XFS_BLFT_DIR_LEAFN_BUF,
    XFS_BLFT_DA_NODE_BUF,
    XFS_BLFT_ATTR_LEAF_BUF,
    XFS_BLFT_ATTR_RMT_BUF,
    XFS_BLFT_SB_BUF,
    XFS_BLFT_RTBITMAP_BUF,
    XFS_BLFT_RTSUMMARY_BUF,
    XFS_BLFT_MAX_BUF = (1 << XFS_BLFT_BITS),
}

//
// EFI/EFD log format definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_extent {
    pub ext_start: xfs_fsblock_t,
    pub ext_len: xfs_extlen_t,
}

//
// Since the structures in struct xfs_extent add up to 96 bytes, it has
// different alignments on i386 vs all other architectures, because i386
// does not pad structures to their natural alignment.
//
// Provide the different variants for use by a conversion routine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_extent_32 {
    pub ext_start: u64,
    pub ext_len: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_extent_64 {
    pub ext_start: u64,
    pub ext_len: u32,
    pub ext_pad: u32,
}

//
// This is the structure used to lay out an efi log item in the
// log.  The efi_extents field is a variable size array whose
// size is given by efi_nextents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efi_log_format {
    pub /: *mut *mut uint16_t efi_type; / efi log item type,
    pub /: *mut *mut uint16_t efi_size; / size of this item,
    pub /: *mut *mut uint32_t efi_nextents; / # extents to free,
    pub /: *mut *mut uint64_t efi_id; / efi identifier,
    pub /: *mut *mut xfs_extent efi_extents[]; / array of extents to free,
}

extern "C" {
    pub fn sizeof(xfs_extent: struct) -> *mut nr;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efi_log_format_32 {
    pub /: *mut *mut uint16_t efi_type; / efi log item type,
    pub /: *mut *mut uint16_t efi_size; / size of this item,
    pub /: *mut *mut uint32_t efi_nextents; / # extents to free,
    pub /: *mut *mut uint64_t efi_id; / efi identifier,
    pub /: *mut *mut xfs_extent_32 efi_extents[]; / array of extents to free,
    pub __attribute__((packed)): },
    pub xfs_extent_32): *mut *mut nr  sizeof(struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efi_log_format_64 {
    pub /: *mut *mut uint16_t efi_type; / efi log item type,
    pub /: *mut *mut uint16_t efi_size; / size of this item,
    pub /: *mut *mut uint32_t efi_nextents; / # extents to free,
    pub /: *mut *mut uint64_t efi_id; / efi identifier,
    pub /: *mut *mut xfs_extent_64 efi_extents[]; / array of extents to free,
}

extern "C" {
    pub fn sizeof(xfs_extent_64: struct) -> *mut nr;
}
//
// This is the structure used to lay out an efd log item in the
// log.  The efd_extents array is a variable size array whose
// size is given by efd_nextents;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efd_log_format {
    pub /: *mut *mut uint16_t efd_type; / efd log item type,
    pub /: *mut *mut uint16_t efd_size; / size of this item,
    pub /: *mut *mut uint32_t efd_nextents; / # of extents freed,
    pub /: *mut *mut uint64_t efd_efi_id; / id of corresponding efi,
    pub /: *mut *mut xfs_extent efd_extents[]; / array of extents freed,
}

extern "C" {
    pub fn sizeof(xfs_extent: struct) -> *mut nr;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efd_log_format_32 {
    pub /: *mut *mut uint16_t efd_type; / efd log item type,
    pub /: *mut *mut uint16_t efd_size; / size of this item,
    pub /: *mut *mut uint32_t efd_nextents; / # of extents freed,
    pub /: *mut *mut uint64_t efd_efi_id; / id of corresponding efi,
    pub /: *mut *mut xfs_extent_32 efd_extents[]; / array of extents freed,
    pub __attribute__((packed)): },
    pub xfs_extent_32): *mut *mut nr  sizeof(struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efd_log_format_64 {
    pub /: *mut *mut uint16_t efd_type; / efd log item type,
    pub /: *mut *mut uint16_t efd_size; / size of this item,
    pub /: *mut *mut uint32_t efd_nextents; / # of extents freed,
    pub /: *mut *mut uint64_t efd_efi_id; / id of corresponding efi,
    pub /: *mut *mut xfs_extent_64 efd_extents[]; / array of extents freed,
}

extern "C" {
    pub fn sizeof(xfs_extent_64: struct) -> *mut nr;
}
//
// RUI/RUD (reverse mapping) log format definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_map_extent {
    pub me_owner: u64,
    pub me_startblock: u64,
    pub me_startoff: u64,
    pub me_len: u32,
    pub me_flags: u32,
}

// rmap me_flags: upper bits are flags, lower byte is type code
pub const XFS_RMAP_EXTENT_MAP: c_int = 1;
pub const XFS_RMAP_EXTENT_MAP_SHARED: c_int = 2;
pub const XFS_RMAP_EXTENT_UNMAP: c_int = 3;
pub const XFS_RMAP_EXTENT_UNMAP_SHARED: c_int = 4;
pub const XFS_RMAP_EXTENT_CONVERT: c_int = 5;
pub const XFS_RMAP_EXTENT_CONVERT_SHARED: c_int = 6;
pub const XFS_RMAP_EXTENT_ALLOC: c_int = 7;
pub const XFS_RMAP_EXTENT_FREE: c_int = 8;
pub const XFS_RMAP_EXTENT_TYPE_MASK: c_uint = 0xFF;

//
// This is the structure used to lay out an rui log item in the
// log.  The rui_extents field is a variable size array whose
// size is given by rui_nextents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rui_log_format {
    pub /: *mut *mut uint16_t rui_type; / rui log item type,
    pub /: *mut *mut uint16_t rui_size; / size of this item,
    pub /: *mut *mut uint32_t rui_nextents; / # extents to free,
    pub /: *mut *mut uint64_t rui_id; / rui identifier,
    pub /: *mut *mut xfs_map_extent rui_extents[]; / array of extents to rmap,
}

extern "C" {
    pub fn sizeof(xfs_map_extent: struct) -> *mut nr;
}
//
// This is the structure used to lay out an rud log item in the
// log.  The rud_extents array is a variable size array whose
// size is given by rud_nextents;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rud_log_format {
    pub /: *mut *mut uint16_t rud_type; / rud log item type,
    pub /: *mut *mut uint16_t rud_size; / size of this item,
    pub __pad: u32,
    pub /: *mut *mut uint64_t rud_rui_id; / id of corresponding rui,
}

//
// CUI/CUD (refcount update) log format definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_phys_extent {
    pub pe_startblock: u64,
    pub pe_len: u32,
    pub pe_flags: u32,
}

// refcount pe_flags: upper bits are flags, lower byte is type code
// Type codes are taken directly from enum xfs_refcount_intent_type.
pub const XFS_REFCOUNT_EXTENT_TYPE_MASK: c_uint = 0xFF;

//
// This is the structure used to lay out a cui log item in the
// log.  The cui_extents field is a variable size array whose
// size is given by cui_nextents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_cui_log_format {
    pub /: *mut *mut uint16_t cui_type; / cui log item type,
    pub /: *mut *mut uint16_t cui_size; / size of this item,
    pub /: *mut *mut uint32_t cui_nextents; / # extents to free,
    pub /: *mut *mut uint64_t cui_id; / cui identifier,
    pub /: *mut *mut xfs_phys_extent cui_extents[]; / array of extents,
}

extern "C" {
    pub fn sizeof(xfs_phys_extent: struct) -> *mut nr;
}
//
// This is the structure used to lay out a cud log item in the
// log.  The cud_extents array is a variable size array whose
// size is given by cud_nextents;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_cud_log_format {
    pub /: *mut *mut uint16_t cud_type; / cud log item type,
    pub /: *mut *mut uint16_t cud_size; / size of this item,
    pub __pad: u32,
    pub /: *mut *mut uint64_t cud_cui_id; / id of corresponding cui,
}

//
// BUI/BUD (inode block mapping) log format definitions
//
// bmbt me_flags: upper bits are flags, lower byte is type code
// Type codes are taken directly from enum xfs_bmap_intent_type.
pub const XFS_BMAP_EXTENT_TYPE_MASK: c_uint = 0xFF;

//
// This is the structure used to lay out an bui log item in the
// log.  The bui_extents field is a variable size array whose
// size is given by bui_nextents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bui_log_format {
    pub /: *mut *mut uint16_t bui_type; / bui log item type,
    pub /: *mut *mut uint16_t bui_size; / size of this item,
    pub /: *mut *mut uint32_t bui_nextents; / # extents to free,
    pub /: *mut *mut uint64_t bui_id; / bui identifier,
    pub /: *mut *mut xfs_map_extent bui_extents[]; / array of extents to bmap,
}

extern "C" {
    pub fn sizeof(xfs_map_extent: struct) -> *mut nr;
}
//
// This is the structure used to lay out an bud log item in the
// log.  The bud_extents array is a variable size array whose
// size is given by bud_nextents;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bud_log_format {
    pub /: *mut *mut uint16_t bud_type; / bud log item type,
    pub /: *mut *mut uint16_t bud_size; / size of this item,
    pub __pad: u32,
    pub /: *mut *mut uint64_t bud_bui_id; / id of corresponding bui,
}

//
// XMI/XMD (file mapping exchange) log format definitions
//
// This is the structure used to lay out an mapping exchange log item.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_xmi_log_format {
    pub /: *mut *mut uint16_t xmi_type; / xmi log item type,
    pub /: *mut *mut uint16_t xmi_size; / size of this item,
    pub /: *mut *mut uint32_t __pad; / must be zero,
    pub /: *mut *mut uint64_t xmi_id; / xmi identifier,
    pub /: *mut *mut uint64_t xmi_inode1; / inumber of first file,
    pub /: *mut *mut uint64_t xmi_inode2; / inumber of second file,
    pub /: *mut *mut uint32_t xmi_igen1; / generation of first file,
    pub /: *mut *mut uint32_t xmi_igen2; / generation of second file,
    pub /: *mut *mut uint64_t xmi_startoff1; / block offset into file1,
    pub /: *mut *mut uint64_t xmi_startoff2; / block offset into file2,
    pub /: *mut *mut uint64_t xmi_blockcount; / number of blocks,
    pub /: *mut *mut *mut uint64_t xmi_flags; / XFS_EXCHMAPS_,
    pub /: *mut *mut uint64_t xmi_isize1; / intended file1 size,
    pub /: *mut *mut uint64_t xmi_isize2; / intended file2 size,
}

// Exchange mappings between extended attribute forks instead of data forks.

// Set the file sizes when finished.

//
// Exchange the mappings of the two files only if the file allocation units
// mapped to file1's range have been written.
//

// Clear the reflink flag from inode1 after the operation.

// Clear the reflink flag from inode2 after the operation.

// This is the structure used to lay out an mapping exchange done log item.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_xmd_log_format {
    pub /: *mut *mut uint16_t xmd_type; / xmd log item type,
    pub /: *mut *mut uint16_t xmd_size; / size of this item,
    pub __pad: u32,
    pub /: *mut *mut uint64_t xmd_xmi_id; / id of corresponding xmi,
}

//
// Dquot Log format definitions.
//
// The first two fields must be the type and size fitting into
// 32 bits : log_recovery code assumes that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dq_logformat {
    pub /: *mut *mut uint16_t qlf_type; / dquot log item type,
    pub /: *mut *mut uint16_t qlf_size; / size of this item,
    pub /: *mut *mut xfs_dqid_t qlf_id; / usr/grp/proj id : 32 bits,
    pub /: *mut *mut int64_t qlf_blkno; / blkno of dquot buffer,
    pub /: *mut *mut int32_t qlf_len; / len of dquot buffer,
    pub /: *mut *mut uint32_t qlf_boffset; / off of dquot in buffer,
}

//
// log format struct for QUOTAOFF records.
// The first two fields must be the type and size fitting into
// 32 bits : log_recovery code assumes that.
// We write two LI_QUOTAOFF logitems per quotaoff, the last one keeps a pointer
// to the first and ensures that the first logitem is taken out of the AIL
// only when the last one is securely committed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_qoff_logformat {
    pub /: *mut *mut unsigned short qf_type; / quotaoff log item type,
    pub /: *mut *mut unsigned short qf_size; / size of this item,
    pub /: *mut *mut unsigned int qf_flags; / USR and/or GRP,
    pub /: *mut *mut char qf_pad[12]; / padding for future,
}

//
// Disk quotas status in m_qflags, and also sb_qflags. 16 bits.
//
pub const XFS_UQUOTA_ACCT: c_uint = 0x0001  /* user quota accounting ON */;
pub const XFS_UQUOTA_ENFD: c_uint = 0x0002  /* user quota limits enforced */;
pub const XFS_UQUOTA_CHKD: c_uint = 0x0004  /* quotacheck run on usr quotas */;
pub const XFS_PQUOTA_ACCT: c_uint = 0x0008  /* project quota accounting ON */;
pub const XFS_OQUOTA_ENFD: c_uint = 0x0010  /* other (grp/prj) quota limits enforced */;
pub const XFS_OQUOTA_CHKD: c_uint = 0x0020  /* quotacheck run on other (grp/prj) quotas */;
pub const XFS_GQUOTA_ACCT: c_uint = 0x0040  /* group quota accounting ON */;
//
// Conversion to and from the combined OQUOTA flag (if necessary)
// is done only in xfs_sb_qflags_to_disk() and xfs_sb_qflags_from_disk()
//
pub const XFS_GQUOTA_ENFD: c_uint = 0x0080  /* group quota limits enforced */;
pub const XFS_GQUOTA_CHKD: c_uint = 0x0100  /* quotacheck run on group quotas */;
pub const XFS_PQUOTA_ENFD: c_uint = 0x0200  /* project quota limits enforced */;
pub const XFS_PQUOTA_CHKD: c_uint = 0x0400  /* quotacheck run on project quotas */;

//
// Inode create log item structure
//
// Log recovery assumes the first two entries are the type and size and they fit
// in 32 bits. Also in host order (ugh) so they have to be 32 bit aligned so
// decoding can be done correctly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_icreate_log {
    pub /: *mut *mut uint16_t icl_type; / type of log format structure,
    pub /: *mut *mut uint16_t icl_size; / size of log format structure,
    pub /: *mut *mut __be32 icl_ag; / ag being allocated in,
    pub /: *mut *mut __be32 icl_agbno; / start block of inode range,
    pub /: *mut *mut __be32 icl_count; / number of inodes to initialise,
    pub /: *mut *mut __be32 icl_isize; / size of inodes,
    pub /: *mut *mut __be32 icl_length; / length of extent to initialise,
    pub /: *mut *mut __be32 icl_gen; / inode generation number to use,
}

//
// Flags for deferred attribute operations.
// Upper bits are flags, lower byte is type code
//

pub const XFS_ATTRI_OP_FLAGS_TYPE_MASK: c_uint = 0xFF	/* Flags type mask */;
//
// alfi_attr_filter captures the state of xfs_da_args.attr_filter, so it should
// never have any other bits set.
//

//
// This is the structure used to lay out an attr log item in the
// log.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attri_log_format {
    pub /: *mut *mut uint16_t alfi_type; / attri log item type,
    pub /: *mut *mut uint16_t alfi_size; / size of this item,
    pub /: *mut *mut uint32_t alfi_igen; / generation of alfi_ino for pptr ops,
    pub /: *mut *mut uint64_t alfi_id; / attri identifier,
    pub /: *mut *mut uint64_t alfi_ino; / the inode for this attr operation,
    pub /: *mut *mut uint32_t alfi_op_flags; / marks the op as a set or remove,
    pub /: *mut *mut uint32_t alfi_name_len; / attr name length,
//
// For PPTR_REPLACE, these are the lengths of the old
// and new attr names.  The new and old values must
// have the same length.
//
    pub alfi_old_name_len: u16,
    pub alfi_new_name_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attrd_log_format {
    pub /: *mut *mut uint16_t alfd_type; / attrd log item type,
    pub /: *mut *mut uint16_t alfd_size; / size of this item,
    pub /: *mut *mut uint32_t __pad; / pad to 64 bit aligned,
    pub /: *mut *mut uint64_t alfd_alf_id; / id of corresponding attri,
}
