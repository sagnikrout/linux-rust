//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_format.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// XFS On Disk Format Definitions
//
// This header file defines all the on-disk format definitions for
// general XFS objects. Directory and attribute related objects are defined in
// xfs_da_format.h, which log and log item formats are defined in
// xfs_log_format.h. Everything else goes here.
//
// Super block
// Fits into a sector-sized buffer at address 0 of each allocation group.
// Only the first of these is ever updated except during growfs.
//
pub const XFS_SB_MAGIC: c_uint = 0x58465342	/* 'XFSB' */;

pub const XFS_SB_VERSION_NUMBITS: c_uint = 0x000f;
pub const XFS_SB_VERSION_ALLFBITS: c_uint = 0xfff0;
pub const XFS_SB_VERSION_ATTRBIT: c_uint = 0x0010;
pub const XFS_SB_VERSION_NLINKBIT: c_uint = 0x0020;
pub const XFS_SB_VERSION_QUOTABIT: c_uint = 0x0040;
pub const XFS_SB_VERSION_ALIGNBIT: c_uint = 0x0080;
pub const XFS_SB_VERSION_DALIGNBIT: c_uint = 0x0100;
pub const XFS_SB_VERSION_SHAREDBIT: c_uint = 0x0200;
pub const XFS_SB_VERSION_LOGV2BIT: c_uint = 0x0400;
pub const XFS_SB_VERSION_SECTORBIT: c_uint = 0x0800;
pub const XFS_SB_VERSION_EXTFLGBIT: c_uint = 0x1000;
pub const XFS_SB_VERSION_DIRV2BIT: c_uint = 0x2000;
pub const XFS_SB_VERSION_BORGBIT: c_uint = 0x4000	/* ASCII only case-insens. */;
pub const XFS_SB_VERSION_MOREBITSBIT: c_uint = 0x8000;
//
// The size of a single extended attribute on disk is limited by
// the size of index values within the attribute entries themselves.
// These are be16 fields, so we can only support attribute data
// sizes up to 2^16 bytes in length.
//

//
// Supported feature bit list is just all bits in the versionnum field because
// we've used them all up and understand them all. Except, of course, for the
// shared superblock bit, which nobody knows what it does and so is unsupported.
//

//
// There are two words to hold XFS "feature" bits: the original
// word, sb_versionnum, and sb_features2.  Whenever a bit is set in
// sb_features2, the feature bit XFS_SB_VERSION_MOREBITSBIT must be set.
//
// These defines represent bits in sb_features2.
//
pub const XFS_SB_VERSION2_RESERVED1BIT: c_uint = 0x00000001;
pub const XFS_SB_VERSION2_LAZYSBCOUNTBIT: c_uint = 0x00000002	/* Superblk counters */;
pub const XFS_SB_VERSION2_RESERVED4BIT: c_uint = 0x00000004;
pub const XFS_SB_VERSION2_ATTR2BIT: c_uint = 0x00000008	/* Inline attr rework */;
pub const XFS_SB_VERSION2_PARENTBIT: c_uint = 0x00000010	/* parent pointers */;
pub const XFS_SB_VERSION2_PROJID32BIT: c_uint = 0x00000080	/* 32 bit project id */;
pub const XFS_SB_VERSION2_CRCBIT: c_uint = 0x00000100	/* metadata CRCs */;
pub const XFS_SB_VERSION2_FTYPE: c_uint = 0x00000200	/* inode type in dir */;

// Maximum size of the xfs filesystem label, no terminating NULL
pub const XFSLABEL_MAX: c_int = 12;
//
// Superblock - in core version.  Must be padded to 64 bit alignment.
//
// statistics
//
// These fields must remain contiguous.  If you really
// want to change their layout, make sure you fix the
// code in xfs_trans_apply_sb_deltas().
//
// End contiguous fields.
//
// bad features2 field as a result of failing to pad the sb structure to
// 64 bits. Some machines will be using this field for features2 bits.
// Easiest just to mark it bad and not use it for anything else.
//
// This is not kept up to date in memory; it is always overwritten by
// the value in sb_features2 when formatting the incore superblock to
// the disk buffer.
//
// version 5 superblock fields start here
// feature masks
// must be padded to 64 bit alignment
//
// Superblock - on disk version.
// Must be padded to 64 bit alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dsb {
    pub /: *mut *mut __be32 sb_magicnum; / magic number == XFS_SB_MAGIC,
    pub /: *mut *mut __be32 sb_blocksize; / logical block size, bytes,
    pub /: *mut *mut __be64 sb_dblocks; / number of data blocks,
    pub /: *mut *mut __be64 sb_rblocks; / number of realtime blocks,
    pub /: *mut *mut __be64 sb_rextents; / number of realtime extents,
    pub /: *mut *mut uuid_t sb_uuid; / user-visible file system unique id,
    pub /: *mut *mut __be64 sb_logstart; / starting block of log if internal,
    pub /: *mut *mut __be64 sb_rootino; / root inode number,
    pub /: *mut *mut __be64 sb_rbmino; / bitmap inode for realtime extents,
    pub /: *mut *mut __be64 sb_rsumino; / summary inode for rt bitmap,
    pub /: *mut *mut __be32 sb_rextsize; / realtime extent size, blocks,
    pub /: *mut *mut __be32 sb_agblocks; / size of an allocation group,
    pub /: *mut *mut __be32 sb_agcount; / number of allocation groups,
    pub /: *mut *mut __be32 sb_rbmblocks; / number of rt bitmap blocks,
    pub /: *mut *mut __be32 sb_logblocks; / number of log blocks,
    pub /: *mut *mut __be16 sb_versionnum; / header version == XFS_SB_VERSION,
    pub /: *mut *mut __be16 sb_sectsize; / volume sector size, bytes,
    pub /: *mut *mut __be16 sb_inodesize; / inode size, bytes,
    pub /: *mut *mut __be16 sb_inopblock; / inodes per block,
    pub /: *mut *mut char sb_fname[XFSLABEL_MAX]; / file system name,
    pub /: *mut *mut __u8 sb_blocklog; / log2 of sb_blocksize,
    pub /: *mut *mut __u8 sb_sectlog; / log2 of sb_sectsize,
    pub /: *mut *mut __u8 sb_inodelog; / log2 of sb_inodesize,
    pub /: *mut *mut __u8 sb_inopblog; / log2 of sb_inopblock,
    pub /: *mut *mut __u8 sb_agblklog; / log2 of sb_agblocks (rounded up),
    pub /: *mut *mut __u8 sb_rextslog; / log2 of sb_rextents,
    pub /: *mut *mut __u8 sb_inprogress; / mkfs is in progress, don't mount,
    pub /: *mut *mut __u8 sb_imax_pct; / max % of fs for inode space,
// statistics
//
// These fields must remain contiguous.  If you really
// want to change their layout, make sure you fix the
// code in xfs_trans_apply_sb_deltas().
//
    pub /: *mut *mut __be64 sb_icount; / allocated inodes,
    pub /: *mut *mut __be64 sb_ifree; / free inodes,
    pub /: *mut *mut __be64 sb_fdblocks; / free data blocks,
    pub /: *mut *mut __be64 sb_frextents; / free realtime extents,
//
// End contiguous fields.
//
    pub /: *mut *mut __be64 sb_uquotino; / user quota inode,
    pub /: *mut *mut __be64 sb_gquotino; / group quota inode,
    pub /: *mut *mut __be16 sb_qflags; / quota flags,
    pub /: *mut *mut __u8 sb_flags; / misc. flags,
    pub /: *mut *mut __u8 sb_shared_vn; / shared version number,
    pub /: *mut *mut __be32 sb_inoalignmt; / inode chunk alignment, fsblocks,
    pub /: *mut *mut __be32 sb_unit; / stripe or raid unit,
    pub /: *mut *mut __be32 sb_width; / stripe or raid width,
    pub /: *mut *mut __u8 sb_dirblklog; / log2 of dir block size (fsbs),
    pub /: *mut *mut __u8 sb_logsectlog; / log2 of the log sector size,
    pub /: *mut *mut __be16 sb_logsectsize; / sector size for the log, bytes,
    pub /: *mut *mut __be32 sb_logsunit; / stripe unit size for the log,
    pub /: *mut *mut __be32 sb_features2; / additional feature bits,
//
// bad features2 field as a result of failing to pad the sb
// structure to 64 bits. Some machines will be using this field
// for features2 bits. Easiest just to mark it bad and not use
// it for anything else.
//
    pub sb_bad_features2: __be32,
// version 5 superblock fields start here
// feature masks
    pub sb_features_compat: __be32,
    pub sb_features_ro_compat: __be32,
    pub sb_features_incompat: __be32,
    pub sb_features_log_incompat: __be32,
    pub /: *mut *mut __le32 sb_crc; / superblock crc,
    pub /: *mut *mut __be32 sb_spino_align; / sparse inode chunk alignment,
    pub /: *mut *mut __be64 sb_pquotino; / project quota inode,
    pub /: *mut *mut __be64 sb_lsn; / last write sequence,
    pub /: *mut *mut uuid_t sb_meta_uuid; / metadata file system unique id,
    pub /: *mut *mut __be64 sb_metadirino; / metadata directory tree root,
    pub /: *mut *mut __be32 sb_rgcount; / # of realtime groups,
    pub /: *mut *mut __be32 sb_rgextents; / size of rtgroup in rtx,
    pub /: *mut *mut __u8 sb_rgblklog; / rt group number shift,
    pub /: *mut *mut __u8 sb_pad[7]; / zeroes,
    pub /: *mut *mut __be64 sb_rtstart; / start of internal RT section (FSB),
    pub /: *mut *mut __be64 sb_rtreserved; / reserved (zoned) RT blocks,
//
// The size of this structure must be padded to 64 bit alignment.
//
// NOTE: Don't forget to update secondary_sb_whack in xfs_repair when
// adding new fields here.
//
}

//
// Misc. Flags - warning - these will be cleared by xfs_repair unless
// a feature bit is set when the flag is used.
//
pub const XFS_SBF_NOFLAGS: c_uint = 0x00	/* no flags set */;
pub const XFS_SBF_READONLY: c_uint = 0x01	/* only read-only mounts allowed */;
//
// define max. shared version we can interoperate with
//
pub const XFS_SB_MAX_SHARED_VN: c_int = 0;

//
// Detect a mismatched features2 field.  Older kernels read/wrote
// this into the wrong slot, so to be safe we keep them in sync.
//
// Extended v5 superblock feature masks. These are to be used for new v5
// superblock features only.
//
// Compat features are new features that old kernels will not notice or affect
// and so can mount read-write without issues.
//
// RO-Compat (read only) are features that old kernels can read but will break
// if they write. Hence only read-only mounts of such filesystems are allowed on
// kernels that don't support the feature bit.
//
// InCompat features are features which old kernels will not understand and so
// must not mount.
//
// Log-InCompat features are for changes to log formats or new transactions that
// can't be replayed on older kernels. The fields are set when the filesystem is
// mounted, and a clean unmount clears the fields.
//
pub const XFS_SB_FEAT_COMPAT_ALL: c_int = 0;

//
// File system sector to basic block conversions.
//

//
// File system block to basic block conversions.
//

//
// File system block to byte conversions.
//

//
// Allocation group header
//
// This is divided into three structures, placed in sequential 512-byte
// buffers after a copy of the superblock (also in a 512-byte buffer).
//
pub const XFS_AGF_MAGIC: c_uint = 0x58414746	/* 'XAGF' */;
pub const XFS_AGI_MAGIC: c_uint = 0x58414749	/* 'XAGI' */;
pub const XFS_AGFL_MAGIC: c_uint = 0x5841464c	/* 'XAFL' */;
pub const XFS_AGF_VERSION: c_int = 1;
pub const XFS_AGI_VERSION: c_int = 1;

//
// agf_cnt_level in the first AGF overlaps the EFS superblock's magic number.
// Since the magic numbers valid for EFS are > 64k, our value cannot be confused
// for an EFS superblock.
//
// Common allocation group header information
//
// Freespace and rmap information
//
// reserve some contiguous space for future logged fields before we add
// the unlogged fields. This makes the range logging via flags and
// structure offsets much simpler.
//
// unlogged fields, written during buffer writeback.
// structure must be padded to 64 bit alignment

pub const XFS_AGF_NUM_BITS: c_int = 18;

// disk block (xfs_daddr_t) in the AG

//
// Size of the unlinked inode hash table in the agi.
//
pub const XFS_AGI_UNLINKED_BUCKETS: c_int = 64;
//
// Common allocation group header information
//
// Inode information
// Inodes are mapped by interpreting the inode number, so no
// mapping data is needed here.
//
// Hash table of inodes which have been unlinked but are
// still being referenced.
//
// This marks the end of logging region 1 and start of logging region 2.
//
// structure must be padded to 64 bit alignment

pub const XFS_AGI_NUM_BITS_R2: c_int = 14;
// disk block (xfs_daddr_t) in the AG

//
// The third a.g. block contains the a.g. freelist, an array
// of block pointers to blocks owned by the allocation btree code.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_agfl {
    pub agfl_magicnum: __be32,
    pub agfl_seqno: __be32,
    pub agfl_uuid: uuid_t,
    pub agfl_lsn: __be64,
    pub agfl_crc: __be32,
    pub __attribute__((packed)): },

//
// For checking for bad ranges of xfs_daddr_t's, covering multiple
// allocation groups or a single xfs_daddr_t that's a superblock copy.
//

//
// Realtime bitmap information is accessed by the word, which is currently
// stored in host-endian format.  Starting with the realtime groups feature,
// the words are stored in be32 ondisk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_rtword_raw {
    pub old: __u32,
    pub rtg: __be32,
}

//
// Realtime summary counts are accessed by the word, which is currently
// stored in host-endian format.  Starting with the realtime groups feature,
// the words are stored in be32 ondisk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union xfs_suminfo_raw {
    pub old: __u32,
    pub rtg: __be32,
}

//
// Realtime allocation groups break the rt section into multiple pieces that
// could be locked independently.  Realtime block group numbers are 32-bit
// quantities.  Block numbers within a group are also 32-bit quantities, but
// the upper bit must never be set.  rtgroup 0 might have a superblock in it,
// so the minimum size of an rtgroup is 2 rtx.
//

pub const XFS_RTSB_MAGIC: c_uint = 0x46726F67	/* 'Frog' */;
//
// Realtime superblock - on disk version.  Must be padded to 64 bit alignment.
// The first block of the realtime volume contains this superblock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtsb {
    pub /: *mut *mut __be32 rsb_magicnum; / magic number == XFS_RTSB_MAGIC,
    pub /: *mut *mut __le32 rsb_crc; / superblock crc,
    pub /: *mut *mut __be32 rsb_pad; / zero,
    pub /: *mut *mut unsigned char rsb_fname[XFSLABEL_MAX]; / file system name,
    pub /: *mut *mut uuid_t rsb_uuid; / user-visible file system unique id,
    pub /: *mut *mut uuid_t rsb_meta_uuid; / metadata file system unique id,
// must be padded to 64 bit alignment
}

//
// XFS Timestamps
// ==============
//
// Traditional ondisk inode timestamps consist of signed 32-bit counters for
// seconds and nanoseconds; time zero is the Unix epoch, Jan  1 00:00:00 UTC
// 1970, which means that the timestamp epoch is the same as the Unix epoch.
// Therefore, the ondisk min and max defined here can be used directly to
// constrain the incore timestamps on a Unix system.  Note that we actually
// encode a __be64 value on disk.
//
// When the bigtime feature is enabled, ondisk inode timestamps become an
// unsigned 64-bit nanoseconds counter.  This means that the bigtime inode
// timestamp epoch is the start of the classic timestamp range, which is
// Dec 13 20:45:52 UTC 1901.  Because the epochs are not the same, callers
// /must/ use the bigtime conversion functions when encoding and decoding raw
// timestamps.
//
pub type xfs_timestamp_t = __be64;
// Legacy timestamp encoding format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_legacy_timestamp {
    pub /: *mut *mut __be32 t_sec; / timestamp seconds,
    pub /: *mut *mut __be32 t_nsec; / timestamp nanoseconds,
}

//
// Smallest possible ondisk seconds value with traditional timestamps.  This
// corresponds exactly with the incore timestamp Dec 13 20:45:52 UTC 1901.
//

//
// Largest possible ondisk seconds value with traditional timestamps.  This
// corresponds exactly with the incore timestamp Jan 19 03:14:07 UTC 2038.
//

//
// Smallest possible ondisk seconds value with bigtime timestamps.  This
// corresponds (after conversion to a Unix timestamp) with the traditional
// minimum timestamp of Dec 13 20:45:52 UTC 1901.
//

//
// Largest supported ondisk seconds value with bigtime timestamps.  This
// corresponds (after conversion to a Unix timestamp) with an incore timestamp
// of Jul  2 20:20:24 UTC 2486.
//
// We round down the ondisk limit so that the bigtime quota and inode max
// timestamps will be the same.
//

//
// Bigtime epoch is set exactly to the minimum time value that a traditional
// 32-bit timestamp can represent when using the Unix epoch as a reference.
// Hence the Unix epoch is at a fixed offset into the supported bigtime
// timestamp range.
//
// The bigtime epoch also matches the minimum value an on-disk 32-bit XFS
// timestamp can represent so we will not lose any fidelity in converting
// to/from unix and bigtime timestamps.
//
// The following conversion factor converts a seconds counter from the Unix
// epoch to the bigtime epoch.
//

// Convert a timestamp from the Unix epoch to the bigtime epoch.
// Convert a timestamp from the bigtime epoch to the Unix epoch.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_metafile_type {
    XFS_METAFILE_UNKNOWN,		/* unknown */
    XFS_METAFILE_DIR,		/* metadir directory */
    XFS_METAFILE_USRQUOTA,		/* user quota */
    XFS_METAFILE_GRPQUOTA,		/* group quota */
    XFS_METAFILE_PRJQUOTA,		/* project quota */
    XFS_METAFILE_RTBITMAP,		/* rt bitmap */
    XFS_METAFILE_RTSUMMARY,		/* rt summary */
    XFS_METAFILE_RTRMAP,		/* rt rmap */
    XFS_METAFILE_RTREFCOUNT,	/* rt refcount */

    XFS_METAFILE_MAX
    } __packed;

    { XFS_METAFILE_UNKNOWN,		"unknown" }, \
    { XFS_METAFILE_DIR,		"dir" }, \
    { XFS_METAFILE_USRQUOTA,	"usrquota" }, \
    { XFS_METAFILE_GRPQUOTA,	"grpquota" }, \
    { XFS_METAFILE_PRJQUOTA,	"prjquota" }, \
    { XFS_METAFILE_RTBITMAP,	"rtbitmap" }, \
    { XFS_METAFILE_RTSUMMARY,	"rtsummary" }, \
    { XFS_METAFILE_RTRMAP,		"rtrmap" }, \
    { XFS_METAFILE_RTREFCOUNT,	"rtrefcount" }

//
// On-disk inode structure.
//
// This is just the header or "dinode core", the inode is expanded to fill a
// variable size the leftover area split into a data and an attribute fork.
// The format of the data and attribute fork depends on the format of the
// inode as indicated by di_format and di_aformat.  To access the data and
// attribute use the XFS_DFORK_DPTR, XFS_DFORK_APTR, and XFS_DFORK_PTR macros
// below.
//
// There is a very similar struct xfs_log_dinode which matches the layout of
// this structure, but is kept in native format instead of big endian.
//
// Note: di_flushiter is only used by v1/2 inodes - it's effectively a zeroed
// padding field for v3 inodes.
//
pub const XFS_DINODE_MAGIC: c_uint = 0x494e	/* 'IN' */;
    struct xfs_dinode {
    __be16		di_magic;	/* inode magic # = XFS_DINODE_MAGIC */
    __be16		di_mode;	/* mode and type of file */
    __u8		di_version;	/* inode version */
    __u8		di_format;	/* format of di_c data */
    __be16		di_metatype;	/* XFS_METAFILE_*; was di_onlink */
    __be32		di_uid;		/* owner's user id */
    __be32		di_gid;		/* owner's group id */
    __be32		di_nlink;	/* number of links to file */
    __be16		di_projid_lo;	/* lower part of owner's project id */
    __be16		di_projid_hi;	/* higher part owner's project id */
    union {
// Number of data fork extents if NREXT64 is set
    __be64	di_big_nextents;

// Padding for V3 inodes without NREXT64 set.
    __be64	di_v3_pad;

// Padding and inode flush counter for V2 inodes.
    struct {
    __u8	di_v2_pad[6];
    __be16	di_flushiter;
}

//
// For V2 inodes and V3 inodes without NREXT64 set, this
// is the number of data and attr fork extents.
//
// Number of attr fork extents if NREXT64 is set.
// di_next_unlinked is the only non-core field in the old dinode
// start of the extended dinode, writable fields
// basic cow extent size for (regular) file
// used blocks in RTG for (zoned) rtrmap inode
// fields only written to during inode creation
// structure must be padded to 64 bit alignment

pub const DI_MAX_FLUSH: c_uint = 0xffff;
//
// Size of the core inode on disk.  Version 1 and 2 inodes have
// the same size, but version 3 has grown a few additional fields.
//
extern "C" {
    pub fn sizeof(xfs_dinode: struct) -> return;
}
extern "C" {
    pub fn offsetof(xfs_dinode: struct, _arg: di_crc) -> return;
}
//
// The 32 bit link count in the inode theoretically maxes out at UINT_MAX.
// Since the pathconf interface is signed, we use 2^31 - 1 instead.
//

//
// Any file that hits the maximum ondisk link count should be pinned to avoid
// a use-after-free situation.
//

//
// Values for di_format
//
// This enum is used in string mapping in xfs_trace.h; please keep the
// TRACE_DEFINE_ENUMs for it up to date.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_dinode_fmt {
    XFS_DINODE_FMT_DEV,		/* xfs_dev_t */
    XFS_DINODE_FMT_LOCAL,		/* bulk data */
    XFS_DINODE_FMT_EXTENTS,		/* struct xfs_bmbt_rec */
    XFS_DINODE_FMT_BTREE,		/* struct xfs_bmdr_block */
    XFS_DINODE_FMT_UUID,		/* added long ago, but never used */
    XFS_DINODE_FMT_META_BTREE,	/* metadata btree */
}

//
// Max values for extnum and aextnum.
//
// The original on-disk extent counts were held in signed fields, resulting in
// maximum extent counts of 2^31 and 2^15 for the data and attr forks
// respectively. Similarly the maximum extent length is limited to 2^21 blocks
// by the 21-bit wide blockcount field of a BMBT extent record.
//
// The newly introduced data fork extent counter can hold a 64-bit value,
// however the maximum number of extents in a file is also limited to 2^54
// extents by the 54-bit wide startoff field of a BMBT extent record.
//
// It is further limited by the maximum supported file size of 2^63
// *bytes*. This leads to a maximum extent count for maximally sized filesystem
// blocks (64kB) of:
//
// 2^63 bytes / 2^16 bytes per block = 2^47 blocks
//
// Rounding up 47 to the nearest multiple of bits-per-byte results in 48. Hence
// 2^48 was chosen as the maximum data fork extent count.
//
// The maximum file size that can be represented by the data fork extent counter
// in the worst case occurs when all extents are 1 block in length and each
// block is 1KB in size.
//
// With XFS_MAX_EXTCNT_DATA_FORK_SMALL representing maximum extent count and
// with 1KB sized blocks, a file can reach upto,
// 1KB * (2^31) = 2TB
//
// This is much larger than the theoretical maximum size of a directory
// i.e. XFS_DIR2_SPACE_SIZE * XFS_DIR2_MAX_SPACES = ~96GB.
//
// Hence, a directory inode can never overflow its data fork extent counter.
//

//
// When we upgrade an inode to the large extent counts, the maximum value by
// which the extent count can increase is bound by the change in size of the
// on-disk field. No upgrade operation should ever be adding more than a few
// tens of extents, so if we get a really large value it is a sign of a code bug
// or corruption.
//

//
// Inode minimum and maximum sizes.
//
pub const XFS_DINODE_MIN_LOG: c_int = 8;
pub const XFS_DINODE_MAX_LOG: c_int = 11;

//
// Inode size for given fs.
//

//
// Inode data & attribute fork sizes, per inode.
//

//
// Return pointers to the data or attribute forks.
//

//
// For block and character special files the 32bit dev_t is stored at the
// beginning of the data fork.
//
extern "C" {
    pub fn be32_to_cpu()XFS_DFORK_DPTR(dip): *mut *mut (__be32) -> return;
}
// (__be32 *)XFS_DFORK_DPTR(dip) = cpu_to_be32(rdev);
//
// Values for di_flags
//

// Do not use bit 15, di_flags is legacy and unchanging now

//
// Values for di_flags2 These start by being exposed to userspace in the upper
// 16 bits of the XFS_XFLAG_s range.
//
// use DAX for this inode
pub const XFS_DIFLAG2_DAX_BIT: c_int = 0;
// file's blocks may be shared
pub const XFS_DIFLAG2_REFLINK_BIT: c_int = 1;
// copy on write extent size hint
pub const XFS_DIFLAG2_COWEXTSIZE_BIT: c_int = 2;
// big timestamps
pub const XFS_DIFLAG2_BIGTIME_BIT: c_int = 3;
// large extent counters
pub const XFS_DIFLAG2_NREXT64_BIT: c_int = 4;
//
// The inode contains filesystem metadata and can be found through the metadata
// directory tree.  Metadata inodes must satisfy the following constraints:
//
// - V5 filesystem (and ftype) are enabled;
// - The only valid modes are regular files and directories;
// - The access bits must be zero;
// - DMAPI event and state masks are zero;
// - The user and group IDs must be zero;
// - The project ID can be used as a u32 annotation;
// - The immutable, sync, noatime, nodump, nodefrag flags must be set.
// - The dax flag must not be set.
// - Directories must have nosymlinks set.
//
// These requirements are chosen defensively to minimize the ability of
// userspace to read or modify the contents, should a metadata file ever
// escape to userspace.
//
// There are further constraints on the directory tree itself:
//
// - Metadata inodes must never be resolvable through the root directory;
// - They must never be accessed by userspace;
// - Metadata directory entries must have correct ftype.
//
// Superblock-rooted metadata files must have the METADATA iflag set even
// though they do not have a parent directory.
//
pub const XFS_DIFLAG2_METADATA_BIT: c_int = 5;

//
// Inode number format:
// low inopblog bits - offset in block
// next agblklog bits - block number in ag
// next agno_log bits - ag number
// high agno_log-agblklog-inopblog bits - 0
//

//
// RealTime Device format definitions
//
// Min and max rt extent sizes, specified in bytes

//
// RT bit manipulation macros.
//
pub const XFS_RTBITMAP_MAGIC: c_uint = 0x424D505A	/* BMPZ */;
pub const XFS_RTSUMMARY_MAGIC: c_uint = 0x53554D59	/* SUMY */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtbuf_blkinfo {
    pub /: *mut *mut __be32 rt_magic; / validity check on block,
    pub /: *mut *mut __be32 rt_crc; / CRC of block,
    pub /: *mut *mut __be64 rt_owner; / inode that owns the block,
    pub /: *mut *mut __be64 rt_blkno; / first block of the buffer,
    pub /: *mut *mut __be64 rt_lsn; / sequence number of last write,
    pub /: *mut *mut uuid_t rt_uuid; / filesystem we belong to,
}

//
// Dquot and dquot block format definitions
//
pub const XFS_DQUOT_MAGIC: c_uint = 0x4451		/* 'DQ' */;

// bitmask to determine if this is a user/group/project dquot

//
// XFS Quota Timers
// ================
//
// Traditional quota grace period expiration timers are an unsigned 32-bit
// seconds counter; time zero is the Unix epoch, Jan  1 00:00:01 UTC 1970.
// Note that an expiration value of zero means that the quota limit has not
// been reached, and therefore no expiration has been set.  Therefore, the
// ondisk min and max defined here can be used directly to constrain the incore
// quota expiration timestamps on a Unix system.
//
// When bigtime is enabled, we trade two bits of precision to expand the
// expiration timeout range to match that of big inode timestamps.  The min and
// max recorded here are the on-disk limits, not a Unix timestamp.
//
// The grace period for each quota type is stored in the root dquot (id = 0)
// and is applied to a non-root dquot when it exceeds the soft or hard limits.
// The length of quota grace periods are unsigned 32-bit quantities measured in
// units of seconds.  A value of zero means to use the default period.
//
// Smallest possible ondisk quota expiration value with traditional timestamps.
// This corresponds exactly with the incore expiration Jan  1 00:00:01 UTC 1970.
//

//
// Largest possible ondisk quota expiration value with traditional timestamps.
// This corresponds exactly with the incore expiration Feb  7 06:28:15 UTC 2106.
//

//
// Smallest possible ondisk quota expiration value with bigtime timestamps.
// This corresponds (after conversion to a Unix timestamp) with the incore
// expiration of Jan  1 00:00:04 UTC 1970.
//

//
// Largest supported ondisk quota expiration value with bigtime timestamps.
// This corresponds (after conversion to a Unix timestamp) with an incore
// expiration of Jul  2 20:20:24 UTC 2486.
//
// The ondisk field supports values up to -1U, which corresponds to an incore
// expiration in 2514.  This is beyond the maximum the bigtime inode timestamp,
// so we cap the maximum bigtime quota expiration to the max inode timestamp.
//

//
// The following conversion factors assist in converting a quota expiration
// timestamp between the incore and ondisk formats.
//

// Convert an incore quota expiration timestamp to an ondisk bigtime value.
//
// Round the expiration timestamp up to the nearest bigtime timestamp
// that we can store, to give users the most time to fix problems.
//
// Convert an ondisk bigtime quota expiration value to an incore timestamp.
//
// Default quota grace periods, ranging from zero (use the compiled defaults)
// to ~136 years.  These are applied to a non-root dquot that has exceeded
// either limit.
//

// Maximum id value for a quota record

//
// This is the main portion of the on-disk representation of quota information
// for a user.  We pad this with some more expansion room to construct the on
// disk structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_disk_dquot {
    pub /: *mut *mut __be16 d_magic; / dquot magic = XFS_DQUOT_MAGIC,
    pub /: *mut *mut __u8 d_version; / dquot version,
    pub /: *mut *mut __u8 d_type; / XFS_DQTYPE_USER/PROJ/GROUP,
    pub /: *mut *mut __be32 d_id; / user,project,group id,
    pub /: *mut *mut __be64 d_blk_hardlimit;/ absolute limit on disk blks,
    pub /: *mut *mut __be64 d_blk_softlimit;/ preferred limit on disk blks,
    pub /: *mut *mut __be64 d_ino_hardlimit;/ maximum # allocated inodes,
    pub /: *mut *mut __be64 d_ino_softlimit;/ preferred inode limit,
    pub /: *mut *mut __be64 d_bcount; / disk blocks owned by the user,
    pub /: *mut *mut __be64 d_icount; / inodes owned by the user,
    pub not,: *mut *mut __be32 d_itimer; / zero if within inode limits if,
    pub /: *mut *mut __be32 d_btimer; / similar to above; for disk blocks,
    pub /: *mut *mut __be16 d_iwarns; / warnings issued wrt num inodes,
    pub /: *mut *mut __be16 d_bwarns; / warnings issued wrt disk blocks,
    pub /: *mut *mut __be32 d_pad0; / 64 bit align,
    pub /: *mut *mut __be64 d_rtb_hardlimit;/ absolute limit on realtime blks,
    pub /: *mut *mut __be64 d_rtb_softlimit;/ preferred limit on RT disk blks,
    pub /: *mut *mut __be64 d_rtbcount; / realtime blocks owned,
    pub /: *mut *mut __be32 d_rtbtimer; / similar to above; for RT disk blocks,
    pub /: *mut *mut __be16 d_rtbwarns; / warnings issued wrt RT disk blocks,
    pub d_pad: __be16,
}

//
// This is what goes on disk. This is separated from the xfs_disk_dquot because
// carrying the unnecessary padding would be a waste of memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dqblk {
    pub /: *mut *mut xfs_disk_dquot dd_diskdq; / portion living incore as well,
    pub /: *mut *mut char dd_fill[4];/ filling for posterity,
//
// These two are only present on filesystems with the CRC bits set.
//
    pub /: *mut *mut __be32 dd_crc; / checksum,
    pub /: *mut *mut __be64 dd_lsn; / last modification in log,
    pub /: *mut *mut uuid_t dd_uuid; / location information,
}

//
// This defines the unit of allocation of dquots.
//
// Currently, it is just one file system block, and a 4K blk contains 30
// (136 * 30 = 4080) dquots. It's probably not worth trying to make
// this more dynamic.
//
// However, if this number is changed, we have to make sure that we don't
// implicitly assume that we do allocations in chunks of a single filesystem
// block in the dquot/xqm code.
//
// This is part of the ondisk format because the structure size is not a power
// of two, which leaves slack at the end of the disk block.
//

//
// Remote symlink format and access functions.
//
pub const XFS_SYMLINK_MAGIC: c_uint = 0x58534c4d	/* XSLM */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dsymlink_hdr {
    pub sl_magic: __be32,
    pub sl_offset: __be32,
    pub sl_bytes: __be32,
    pub sl_crc: __be32,
    pub sl_uuid: uuid_t,
    pub sl_owner: __be64,
    pub sl_blkno: __be64,
    pub sl_lsn: __be64,
}

pub const XFS_SYMLINK_MAXLEN: c_int = 1024;
//
// The maximum pathlen is 1024 bytes. Since the minimum file system
// blocksize is 512 bytes, we can get a max of 3 extents back from
// bmapi when crc headers are taken into account.
//
pub const XFS_SYMLINK_MAPS: c_int = 3;

//
// Allocation Btree format definitions
//
// There are two on-disk btrees, one sorted by blockno and one sorted
// by blockcount and blockno.  All blocks look the same to make the code
// simpler; if we have time later, we'll make the optimizations.
//
pub const XFS_ABTB_MAGIC: c_uint = 0x41425442	/* 'ABTB' for bno tree */;
pub const XFS_ABTB_CRC_MAGIC: c_uint = 0x41423342	/* 'AB3B' */;
pub const XFS_ABTC_MAGIC: c_uint = 0x41425443	/* 'ABTC' for cnt tree */;
pub const XFS_ABTC_CRC_MAGIC: c_uint = 0x41423343	/* 'AB3C' */;
//
// Data record/key structure
//
// btree pointer type
pub type xfs_alloc_ptr_t = __be32;
//
// Block numbers in the AG:
// SB is sector 0, AGF is sector 1, AGI is sector 2, AGFL is sector 3.
//

//
// Inode Allocation Btree format definitions
//
// There is a btree for the inode map per allocation group.
//
pub const XFS_IBT_MAGIC: c_uint = 0x49414254	/* 'IABT' */;
pub const XFS_IBT_CRC_MAGIC: c_uint = 0x49414233	/* 'IAB3' */;
pub const XFS_FIBT_MAGIC: c_uint = 0x46494254	/* 'FIBT' */;
pub const XFS_FIBT_CRC_MAGIC: c_uint = 0x46494233	/* 'FIB3' */;
pub type xfs_inofree_t = u64;

//
// The on-disk inode record structure has two formats. The original "full"
// format uses a 4-byte freecount. The "sparse" format uses a 1-byte freecount
// and replaces the 3 high-order freecount bytes wth the holemask and inode
// count.
//
// The holemask of the sparse record format allows an inode chunk to have holes
// that refer to blocks not owned by the inode record. This facilitates inode
// allocation in the event of severe free space fragmentation.
//
// non-zero holemask represents a sparse rec.
//
// Key structure
//
// btree pointer type
pub type xfs_inobt_ptr_t = __be32;
//
// block numbers in the AG.
//

//
// Reverse mapping btree format definitions
//
// There is a btree for the reverse map per allocation group
//
pub const XFS_RMAP_CRC_MAGIC: c_uint = 0x524d4233	/* 'RMB3' */;
//
// Ownership info for an extent.  This is used to create reverse-mapping
// entries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_owner_info {
    pub oi_owner: u64,
    pub oi_offset: xfs_fileoff_t,
    pub oi_flags: c_uint,
}

//
// Special owner types.
//
// Seeing as we only support up to 8EB, we have the upper bit of the owner field
// to tell us we have a special owner value. We use these for static metadata
// allocated at mkfs/growfs time, as well as for freespace management metadata.
//

//
// Data record structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_rec {
    pub /: *mut *mut __be32 rm_startblock; / extent start block,
    pub /: *mut *mut __be32 rm_blockcount; / extent length,
    pub /: *mut *mut __be64 rm_owner; / extent owner,
    pub /: *mut *mut __be64 rm_offset; / offset within the owner,
}

//
// rmap btree record
// rm_offset:63 is the attribute fork flag
// rm_offset:62 is the bmbt block flag
// rm_offset:61 is the unwritten extent flag (same as l0:63 in bmbt)
// rm_offset:54-60 aren't used and should be zero
// rm_offset:0-53 is the block offset within the inode
//

pub const RMAPBT_STARTBLOCK_BITLEN: c_int = 32;
pub const RMAPBT_BLOCKCOUNT_BITLEN: c_int = 32;
pub const RMAPBT_OWNER_BITLEN: c_int = 64;
pub const RMAPBT_ATTRFLAG_BITLEN: c_int = 1;
pub const RMAPBT_BMBTFLAG_BITLEN: c_int = 1;
pub const RMAPBT_EXNTFLAG_BITLEN: c_int = 1;
pub const RMAPBT_UNUSED_OFFSET_BITLEN: c_int = 7;
pub const RMAPBT_OFFSET_BITLEN: c_int = 54;
//
// Key structure
//
// We don't use the length for lookups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rmap_key {
    pub /: *mut *mut __be32 rm_startblock; / extent start block,
    pub /: *mut *mut __be64 rm_owner; / extent owner,
    pub /: *mut *mut __be64 rm_offset; / offset within the owner,
    pub __attribute__((packed)): },
// btree pointer type
pub type xfs_rmap_ptr_t = __be32;

//
// Realtime Reverse mapping btree format definitions
//
// This is a btree for reverse mapping records for realtime volumes
//
pub const XFS_RTRMAP_CRC_MAGIC: c_uint = 0x4d415052	/* 'MAPR' */;
//
// rtrmap root header, on-disk form only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtrmap_root {
    pub /: *mut *mut __be16 bb_level; / 0 is a leaf,
    pub /: *mut *mut __be16 bb_numrecs; / current # of data records,
}

// inode-based btree pointer type
pub type xfs_rtrmap_ptr_t = __be64;
//
// Reference Count Btree format definitions
//
pub const XFS_REFC_CRC_MAGIC: c_uint = 0x52334643	/* 'R3FC' */;
extern "C" {
    pub fn xfs_refc_block(mp: *mut xfs_mount) -> c_uint;
}
//
// Data record/key structure
//
// Each record associates a range of physical blocks (starting at
// rc_startblock and ending rc_blockcount blocks later) with a reference
// count (rc_refcount).  Extents that are being used to stage a copy on
// write (CoW) operation are recorded in the refcount btree with a
// refcount of 1.  All other records must have a refcount > 1 and must
// track an extent mapped only by file data forks.
//
// Extents with a single owner (attributes, metadata, non-shared file
// data) are not tracked here.  Free space is also not tracked here.
// This is consistent with pre-reflink XFS.
//
// Extents that are being used to stage a copy on write are stored
// in the refcount btree with a refcount of 1 and the upper bit set
// on the startblock.  This speeds up mount time deletion of stale
// staging extents because they're all at the right side of the tree.
//

pub const REFCNTBT_COWFLAG_BITLEN: c_int = 1;
pub const REFCNTBT_AGBLOCK_BITLEN: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_refcount_rec {
    pub /: *mut *mut __be32 rc_startblock; / starting block number,
    pub /: *mut *mut __be32 rc_blockcount; / count of blocks,
    pub /: *mut *mut __be32 rc_refcount; / number of inodes linked here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_refcount_key {
    pub /: *mut *mut __be32 rc_startblock; / starting block number,
}

// btree pointer type
pub type xfs_refcount_ptr_t = __be32;
//
// Realtime Reference Count btree format definitions
//
// This is a btree for reference count records for realtime volumes
//
pub const XFS_RTREFC_CRC_MAGIC: c_uint = 0x52434e54	/* 'RCNT' */;
//
// rt refcount root header, on-disk form only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtrefcount_root {
    pub /: *mut *mut __be16 bb_level; / 0 is a leaf,
    pub /: *mut *mut __be16 bb_numrecs; / current # of data records,
}

// inode-rooted btree pointer type
pub type xfs_rtrefcount_ptr_t = __be64;
//
// BMAP Btree format definitions
//
// This includes both the root block definition that sits inside an inode fork
// and the record/pointer formats for the leaf/node in the blocks.
//
pub const XFS_BMAP_MAGIC: c_uint = 0x424d4150	/* 'BMAP' */;
pub const XFS_BMAP_CRC_MAGIC: c_uint = 0x424d4133	/* 'BMA3' */;
//
// Bmap root header, on-disk form only.
//
// Bmap btree record and extent descriptor.
// l0:63 is an extent flag (value 1 indicates non-normal).
// l0:9-62 are startoff.
// l0:0-8 and l1:21-63 are startblock.
// l1:0-20 are blockcount.
//
pub const BMBT_EXNTFLAG_BITLEN: c_int = 1;
pub const BMBT_STARTOFF_BITLEN: c_int = 54;
pub const BMBT_STARTBLOCK_BITLEN: c_int = 52;
pub const BMBT_BLOCKCOUNT_BITLEN: c_int = 21;

//
// bmbt records have a file offset (block) field that is 54 bits wide, so this
// is the largest xfs_fileoff_t that we ever expect to see.
//

pub type xfs_bmdr_rec_t = xfs_bmbt_rec_t;
//
// Values and macros for delayed-allocation startblock fields.
//
pub const STARTBLOCKVALBITS: c_int = 17;

//
// Key structure for non-leaf levels of the tree.
//
// btree pointer type
//
// Generic Btree block format definitions
//
// This is a combination of the actual format used on disk for short and long
// format btrees.  The first three fields are shared by both format, but the
// pointers are different and should be used with care.
//
// To get the size of the actual short or long form headers please use the size
// macros below.  Never use sizeof(xfs_btree_block).
//
// The blkno, crc, lsn, owner and uuid fields are only available in filesystems
// with the crc feature bit, and all accesses to them must be conditional on
// that flag.
//
// short form block header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_block_shdr {
    pub bb_leftsib: __be32,
    pub bb_rightsib: __be32,
    pub bb_blkno: __be64,
    pub bb_lsn: __be64,
    pub bb_uuid: uuid_t,
    pub bb_owner: __be32,
    pub bb_crc: __le32,
}

// long form block header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_block_lhdr {
    pub bb_leftsib: __be64,
    pub bb_rightsib: __be64,
    pub bb_blkno: __be64,
    pub bb_lsn: __be64,
    pub bb_uuid: uuid_t,
    pub bb_owner: __be64,
    pub bb_crc: __le32,
    pub /: *mut *mut __be32 bb_pad; / padding for alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_btree_block {
    pub /: *mut *mut __be32 bb_magic; / magic number for block type,
    pub /: *mut *mut __be16 bb_level; / 0 is a leaf,
    pub /: *mut *mut __be16 bb_numrecs; / current # of data records,
    pub s: xfs_btree_block_shdr,
    pub l: xfs_btree_block_lhdr,
    pub /: *mut *mut } bb_u; / rest,
}

// size of a short form block

// size of a long form block

// sizes of CRC enabled btree blocks

//
// On-disk XFS access control list structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_acl_entry {
    pub ae_tag: __be32,
    pub ae_id: __be32,
    pub ae_perm: __be16,
    pub /: *mut *mut __be16 ae_pad; / fill the implicit hole in the structure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_acl {
    pub acl_cnt: __be32,
    pub acl_entry: [xfs_acl_entry; ],
}

//
// The number of ACL entries allowed is defined by the on-disk format.
// For v4 superblocks, that is limited to 25 entries. For v5 superblocks, it is
// limited only by the maximum size of the xattr that stores the information.
//

// On-disk XFS extended attribute names

