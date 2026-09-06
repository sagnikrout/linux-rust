//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_fs.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) 1995-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// SGI's XFS filesystem's major stuff (constants, structures)
// NOTE: This file must be compile-able with C++ compilers.
//
// Direct I/O attribute record used with XFS_IOC_DIOINFO
// d_miniosz is the min xfer size, xfer size multiple and file seek offset
// alignment.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dioattr {
    pub /: *mut *mut __u32 d_mem; / data buffer memory alignment,
    pub /: *mut *mut __u32 d_miniosz; / min xfer size,
    pub /: *mut *mut __u32 d_maxiosz; / max xfer size,
}

//
// Structure for XFS_IOC_GETBMAP.
// On input, fill in bmv_offset and bmv_length of the first structure
// to indicate the area of interest in the file, and bmv_entries with
// the number of array elements given back.  The first structure is
// updated on return to give the offset and length for the next call.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct getbmap {
    pub /: *mut *mut __s64 bmv_offset; / file offset of segment in blocks,
    pub /: *mut *mut __s64 bmv_block; / starting block (64-bit daddr_t),
    pub /: *mut *mut __s64 bmv_length; / length of segment, blocks,
    pub /: *mut *mut __s32 bmv_count; / # of entries in array incl. 1st,
    pub /: *mut *mut __s32 bmv_entries; / # of entries filled in (output),
}

//
// Structure for XFS_IOC_GETBMAPX.	 Fields bmv_offset through bmv_entries
// are used exactly as in the getbmap structure.  The getbmapx structure
// has additional bmv_iflags and bmv_oflags fields. The bmv_iflags field
// is only used for the first structure.  It contains input flags
// specifying XFS_IOC_GETBMAPX actions.  The bmv_oflags field is filled
// in by the XFS_IOC_GETBMAPX command for each returned structure after
// the first.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct getbmapx {
    pub /: *mut *mut __s64 bmv_offset; / file offset of segment in blocks,
    pub /: *mut *mut __s64 bmv_block; / starting block (64-bit daddr_t),
    pub /: *mut *mut __s64 bmv_length; / length of segment, blocks,
    pub /: *mut *mut __s32 bmv_count; / # of entries in array incl. 1st,
    pub /: *mut *mut __s32 bmv_entries; / # of entries filled in (output).,
    pub /: *mut *mut __s32 bmv_iflags; / input flags (1st structure),
    pub structure)*/: *mut *mut __s32 bmv_oflags; / output flags (after 1st,
    pub /: *mut *mut __s32 bmv_unused1; / future use,
    pub /: *mut *mut __s32 bmv_unused2; / future use,
}

// bmv_iflags values - set by XFS_IOC_GETBMAPX caller.
pub const BMV_IF_ATTRFORK: c_uint = 0x1	/* return attr fork rather than data */;
pub const BMV_IF_NO_DMAPI_READ: c_uint = 0x2	/* Deprecated */;
pub const BMV_IF_PREALLOC: c_uint = 0x4	/* rtn status BMV_OF_PREALLOC if req */;
pub const BMV_IF_DELALLOC: c_uint = 0x8	/* rtn status BMV_OF_DELALLOC if req */;
pub const BMV_IF_NO_HOLES: c_uint = 0x10	/* Do not return holes */;
pub const BMV_IF_COWFORK: c_uint = 0x20	/* return CoW fork rather than data */;

// bmv_oflags values - returned for each non-header segment
pub const BMV_OF_PREALLOC: c_uint = 0x1	/* segment = unwritten pre-allocation */;
pub const BMV_OF_DELALLOC: c_uint = 0x2	/* segment = delayed allocation */;
pub const BMV_OF_LAST: c_uint = 0x4	/* segment is the last in the file */;
pub const BMV_OF_SHARED: c_uint = 0x8	/* segment shared with another file */;
// fmr_owner special values for FS_IOC_GETFSMAP

//
// File segment locking set data type for 64 bit access.
// Also used for all the RESV/FREE interfaces.
//
// Output for XFS_IOC_FSGEOMETRY_V1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fsop_geom_v1 {
    pub /: *mut *mut __u32 blocksize; / filesystem (data) block size,
    pub /: *mut *mut __u32 rtextsize; / realtime extent size,
    pub /: *mut *mut __u32 agblocks; / fsblocks in an AG,
    pub /: *mut *mut __u32 agcount; / number of allocation groups,
    pub /: *mut *mut __u32 logblocks; / fsblocks in the log,
    pub /: *mut *mut __u32 sectsize; / (data) sector size, bytes,
    pub /: *mut *mut __u32 inodesize; / inode size in bytes,
    pub /: *mut *mut __u32 imaxpct; / max allowed inode space(%),
    pub /: *mut *mut __u64 datablocks; / fsblocks in data subvolume,
    pub /: *mut *mut __u64 rtblocks; / fsblocks in realtime subvol,
    pub subvol*/: *mut *mut __u64 rtextents; / rt extents in realtime,
    pub /: *mut *mut __u64 logstart; / starting fsblock of the log,
    pub /: *mut *mut unsigned char uuid[16]; / unique id of the filesystem,
    pub /: *mut *mut __u32 sunit; / stripe unit, fsblocks,
    pub /: *mut *mut __u32 swidth; / stripe width, fsblocks,
    pub /: *mut *mut __s32 version; / structure version,
    pub /: *mut *mut __u32 flags; / superblock version flags,
    pub /: *mut *mut __u32 logsectsize; / log sector size, bytes,
    pub /: *mut *mut __u32 rtsectsize; / realtime sector size, bytes,
    pub /: *mut *mut __u32 dirblocksize; / directory block size, bytes,
}

//
// Output for XFS_IOC_FSGEOMETRY_V4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fsop_geom_v4 {
    pub /: *mut *mut __u32 blocksize; / filesystem (data) block size,
    pub /: *mut *mut __u32 rtextsize; / realtime extent size,
    pub /: *mut *mut __u32 agblocks; / fsblocks in an AG,
    pub /: *mut *mut __u32 agcount; / number of allocation groups,
    pub /: *mut *mut __u32 logblocks; / fsblocks in the log,
    pub /: *mut *mut __u32 sectsize; / (data) sector size, bytes,
    pub /: *mut *mut __u32 inodesize; / inode size in bytes,
    pub /: *mut *mut __u32 imaxpct; / max allowed inode space(%),
    pub /: *mut *mut __u64 datablocks; / fsblocks in data subvolume,
    pub /: *mut *mut __u64 rtblocks; / fsblocks in realtime subvol,
    pub subvol*/: *mut *mut __u64 rtextents; / rt extents in realtime,
    pub /: *mut *mut __u64 logstart; / starting fsblock of the log,
    pub /: *mut *mut unsigned char uuid[16]; / unique id of the filesystem,
    pub /: *mut *mut __u32 sunit; / stripe unit, fsblocks,
    pub /: *mut *mut __u32 swidth; / stripe width, fsblocks,
    pub /: *mut *mut __s32 version; / structure version,
    pub /: *mut *mut __u32 flags; / superblock version flags,
    pub /: *mut *mut __u32 logsectsize; / log sector size, bytes,
    pub /: *mut *mut __u32 rtsectsize; / realtime sector size, bytes,
    pub /: *mut *mut __u32 dirblocksize; / directory block size, bytes,
    pub /: *mut *mut __u32 logsunit; / log stripe unit, bytes,
}

//
// Output for XFS_IOC_FSGEOMETRY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fsop_geom {
    pub /: *mut *mut __u32 blocksize; / filesystem (data) block size,
    pub /: *mut *mut __u32 rtextsize; / realtime extent size,
    pub /: *mut *mut __u32 agblocks; / fsblocks in an AG,
    pub /: *mut *mut __u32 agcount; / number of allocation groups,
    pub /: *mut *mut __u32 logblocks; / fsblocks in the log,
    pub /: *mut *mut __u32 sectsize; / (data) sector size, bytes,
    pub /: *mut *mut __u32 inodesize; / inode size in bytes,
    pub /: *mut *mut __u32 imaxpct; / max allowed inode space(%),
    pub /: *mut *mut __u64 datablocks; / fsblocks in data subvolume,
    pub /: *mut *mut __u64 rtblocks; / fsblocks in realtime subvol,
    pub subvol*/: *mut *mut __u64 rtextents; / rt extents in realtime,
    pub /: *mut *mut __u64 logstart; / starting fsblock of the log,
    pub /: *mut *mut unsigned char uuid[16]; / unique id of the filesystem,
    pub /: *mut *mut __u32 sunit; / stripe unit, fsblocks,
    pub /: *mut *mut __u32 swidth; / stripe width, fsblocks,
    pub /: *mut *mut __s32 version; / structure version,
    pub /: *mut *mut __u32 flags; / superblock version flags,
    pub /: *mut *mut __u32 logsectsize; / log sector size, bytes,
    pub /: *mut *mut __u32 rtsectsize; / realtime sector size, bytes,
    pub /: *mut *mut __u32 dirblocksize; / directory block size, bytes,
    pub /: *mut *mut __u32 logsunit; / log stripe unit, bytes,
    pub /: *mut *mut uint32_t sick; / o: unhealthy fs & rt metadata,
    pub /: *mut *mut uint32_t checked; / o: checked fs & rt metadata,
    pub /: *mut *mut __u32 rgextents; / rt extents in a realtime group,
    pub /: *mut *mut __u32 rgcount; / number of realtime groups,
    pub /: *mut *mut __u64 rtstart; / start of internal rt section,
    pub /: *mut *mut __u64 rtreserved; / RT (zoned) reserved blocks,
    pub /: *mut *mut __u64 reserved[14]; / reserved space,
}

// Output for XFS_FS_COUNTS
// Input/Output for XFS_GET_RESBLKS and XFS_SET_RESBLKS
pub const XFS_FSOP_GEOM_VERSION: c_int = 0;
pub const XFS_FSOP_GEOM_VERSION_V5: c_int = 5;

// -- Do not use --		(1 << 13)    SGI parent pointers

//
// Minimum and maximum sizes need for growth checks.
//
// Block counts are in units of filesystem blocks, not basic blocks.
//
pub const XFS_MIN_AG_BLOCKS: c_int = 64;

//
// Limits on sb_agblocks/sb_agblklog -- mkfs won't format AGs smaller than
// 16MB or larger than 1TB.
//

// keep the maximum size under 2^31 by a small amount

// Used for sanity checks on superblock

//
// Output for XFS_IOC_AG_GEOMETRY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ag_geometry {
    pub /: *mut *mut uint32_t ag_number; / i/o: AG number,
    pub /: *mut *mut uint32_t ag_length; / o: length in blocks,
    pub /: *mut *mut uint32_t ag_freeblks; / o: free space,
    pub /: *mut *mut uint32_t ag_icount; / o: inodes allocated,
    pub /: *mut *mut uint32_t ag_ifree; / o: inodes free,
    pub /: *mut *mut uint32_t ag_sick; / o: sick things in ag,
    pub /: *mut *mut uint32_t ag_checked; / o: checked metadata in ag,
    pub /: *mut *mut uint32_t ag_flags; / i/o: flags for this ag,
    pub /: *mut *mut uint64_t ag_reserved[12];/ o: zero,
}

//
// Structures for XFS_IOC_FSGROWFSDATA, XFS_IOC_FSGROWFSLOG & XFS_IOC_FSGROWFSRT
//
// Structures returned from ioctl XFS_IOC_FSBULKSTAT & XFS_IOC_FSBULKSTAT_SINGLE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bstat {
    pub /: *mut *mut __u64 bs_ino; / inode number,
    pub /: *mut *mut __u16 bs_mode; / type and mode,
    pub /: *mut *mut __u16 bs_nlink; / number of links,
    pub /: *mut *mut __u32 bs_uid; / user id,
    pub /: *mut *mut __u32 bs_gid; / group id,
    pub /: *mut *mut __u32 bs_rdev; / device value,
    pub /: *mut *mut __s32 bs_blksize; / block size,
    pub /: *mut *mut __s64 bs_size; / file size,
    pub /: *mut *mut xfs_bstime_t bs_atime; / access time,
    pub /: *mut *mut xfs_bstime_t bs_mtime; / modify time,
    pub /: *mut *mut xfs_bstime_t bs_ctime; / inode change time,
    pub /: *mut *mut int64_t bs_blocks; / number of blocks,
    pub /: *mut *mut __u32 bs_xflags; / extended flags,
    pub /: *mut *mut __s32 bs_extsize; / extent size,
    pub /: *mut *mut __s32 bs_extents; / number of extents,
    pub /: *mut *mut __u32 bs_gen; / generation count,
    pub /: *mut *mut __u16 bs_projid_lo; / lower part of project id,

    pub /: *mut *mut __u16 bs_forkoff; / inode fork offset in bytes,
    pub /: *mut *mut __u16 bs_projid_hi; / higher part of project id,
    pub /: *mut *mut uint16_t bs_sick; / sick inode metadata,
    pub /: *mut *mut uint16_t bs_checked; / checked inode metadata,
    pub /: *mut *mut unsigned char bs_pad[2]; / pad space, unused,
    pub /: *mut *mut __u32 bs_cowextsize; / cow extent size,
    pub /: *mut *mut __u32 bs_dmevmask; / DMIG event mask,
    pub /: *mut *mut __u16 bs_dmstate; / DMIG state info,
    pub /: *mut *mut __u16 bs_aextents; / attribute number of extents,
}

// New bulkstat structure that reports v5 features and fixes padding issues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bulkstat {
    pub /: *mut *mut uint64_t bs_ino; / inode number,
    pub /: *mut *mut uint64_t bs_size; / file size,
    pub /: *mut *mut uint64_t bs_blocks; / number of blocks,
    pub /: *mut *mut uint64_t bs_xflags; / extended flags,
    pub /: *mut *mut int64_t bs_atime; / access time, seconds,
    pub /: *mut *mut int64_t bs_mtime; / modify time, seconds,
    pub /: *mut *mut int64_t bs_ctime; / inode change time, seconds,
    pub /: *mut *mut int64_t bs_btime; / creation time, seconds,
    pub /: *mut *mut uint32_t bs_gen; / generation count,
    pub /: *mut *mut uint32_t bs_uid; / user id,
    pub /: *mut *mut uint32_t bs_gid; / group id,
    pub /: *mut *mut uint32_t bs_projectid; / project id,
    pub /: *mut *mut uint32_t bs_atime_nsec; / access time, nanoseconds,
    pub /: *mut *mut uint32_t bs_mtime_nsec; / modify time, nanoseconds,
    pub /: *mut *mut uint32_t bs_ctime_nsec; / inode change time, nanoseconds,
    pub /: *mut *mut uint32_t bs_btime_nsec; / creation time, nanoseconds,
    pub /: *mut *mut uint32_t bs_blksize; / block size,
    pub /: *mut *mut uint32_t bs_rdev; / device value,
    pub /: *mut *mut uint32_t bs_cowextsize_blks; / cow extent size hint, blocks,
    pub /: *mut *mut uint32_t bs_extsize_blks; / extent size hint, blocks,
    pub /: *mut *mut uint32_t bs_nlink; / number of links,
    pub /: *mut *mut uint32_t bs_extents; / 32-bit data fork extent counter,
    pub /: *mut *mut uint32_t bs_aextents; / attribute number of extents,
    pub /: *mut *mut uint16_t bs_version; / structure version,
    pub /: *mut *mut uint16_t bs_forkoff; / inode fork offset in bytes,
    pub /: *mut *mut uint16_t bs_sick; / sick inode metadata,
    pub /: *mut *mut uint16_t bs_checked; / checked inode metadata,
    pub /: *mut *mut uint16_t bs_mode; / type and mode,
    pub /: *mut *mut uint16_t bs_pad2; / zeroed,
    pub /: *mut *mut uint64_t bs_extents64; / 64-bit data fork extent counter,
    pub /: *mut *mut uint64_t bs_pad[6]; / zeroed,
}

// bs_sick flags

//
// Project quota id helpers (previously projid was 16bit only
// and using two 16bit values to hold new 32bit projid was chosen
// to retain compatibility with "old" filesystems).
//
// The user-level BulkStat Request interface structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fsop_bulkreq {
    pub /: *mut *mut *mut __u64 __user lastip; / last inode # pointer,
    pub /: *mut *mut __s32 icount; / count of entries in buffer,
    pub /: *mut *mut *mut void __user ubuffer;/ user buffer for inode desc.,
    pub /: *mut *mut *mut __s32 __user ocount; / output count pointer,
}

//
// Structures returned from xfs_inumbers routine (XFS_IOC_FSINUMBERS).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inogrp {
    pub /: *mut *mut __u64 xi_startino; / starting inode number,
    pub /: *mut *mut __s32 xi_alloccount; / # bits set in allocmask,
    pub /: *mut *mut __u64 xi_allocmask; / mask of allocated inodes,
}

// New inumbers structure that reports v5 features and fixes padding issues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inumbers {
    pub /: *mut *mut uint64_t xi_startino; / starting inode number,
    pub /: *mut *mut uint64_t xi_allocmask; / mask of allocated inodes,
    pub /: *mut *mut uint8_t xi_alloccount; / # bits set in allocmask,
    pub /: *mut *mut uint8_t xi_version; / version,
    pub /: *mut *mut uint8_t xi_padding[6]; / zero,
}

// Header for bulk inode requests.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bulk_ireq {
    pub /: *mut *mut uint64_t ino; / I/O: start with this inode,
    pub /: *mut *mut uint32_t flags; / I/O: operation flags,
    pub /: *mut *mut uint32_t icount; / I: count of entries in buffer,
    pub /: *mut *mut uint32_t ocount; / O: count of entries filled out,
    pub /: *mut *mut uint32_t agno; / I: see comment for IREQ_AGNO,
    pub /: *mut *mut uint64_t reserved[5]; / must be zero,
}

//
// Only return results from the specified @agno.  If @ino is zero, start
// with the first inode of @agno.
//

//
// Return bulkstat information for a single inode, where @ino value is a
// special value, not a literal inode number.  See the XFS_BULK_IREQ_SPECIAL_
// values below.  Not compatible with XFS_BULK_IREQ_AGNO.
//

//
// Return data fork extent count via xfs_bulkstat->bs_extents64 field and assign
// 0 to xfs_bulkstat->bs_extents when the flag is set.  Otherwise, use
// xfs_bulkstat->bs_extents for returning data fork extent count and set
// xfs_bulkstat->bs_extents64 to 0. In the second case, return -EOVERFLOW and
// assign 0 to xfs_bulkstat->bs_extents if data fork extent count is larger than
// XFS_MAX_EXTCNT_DATA_FORK_OLD.
//

//
// Allow bulkstat to return information about metadata directories.  This
// enables xfs_scrub to find them for scanning, as they are otherwise ordinary
// directories.
//

// Operate on the root directory inode.

//
// ioctl structures for v5 bulkstat and inumbers requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_bulkstat_req {
    pub hdr: xfs_bulk_ireq,
    pub bulkstat: [xfs_bulkstat; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inumbers_req {
    pub hdr: xfs_bulk_ireq,
    pub inumbers: [xfs_inumbers; ],
}

//
// Error injection.
//
// Speculative preallocation trimming.
//
pub const XFS_EOFBLOCKS_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_fs_eofblocks {
    pub eof_version: __u32,
    pub eof_flags: __u32,
    pub eof_uid: uid_t,
    pub eof_gid: gid_t,
    pub eof_prid: prid_t,
    pub pad32: __u32,
    pub eof_min_file_size: __u64,
    pub pad64: [__u64; 12],
}

// eof_flags values

// kernel only, not included in
// valid mask

//
// The user-level Handle Request interface structure.
//
// Compound structures for passing args through Handle Request interfaces
// xfs_attrlist_by_handle, xfs_attrmulti_by_handle
// - ioctls: XFS_IOC_ATTRLIST_BY_HANDLE, and XFS_IOC_ATTRMULTI_BY_HANDLE
//
// Flags passed in xfs_attr_multiop.am_flags for the attr ioctl interface.
//
// NOTE: Must match the values declared in libattr without the XFS_IOC_ prefix.
//
pub const XFS_IOC_ATTR_ROOT: c_uint = 0x0002	/* use attrs in root namespace */;
pub const XFS_IOC_ATTR_SECURE: c_uint = 0x0008	/* use attrs in security namespace */;
pub const XFS_IOC_ATTR_CREATE: c_uint = 0x0010	/* fail if attr already exists */;
pub const XFS_IOC_ATTR_REPLACE: c_uint = 0x0020	/* fail if attr does not exist */;
//
// Define how lists of attribute names are returned to userspace from the
// XFS_IOC_ATTRLIST_BY_HANDLE ioctl.  struct xfs_attrlist is the header at the
// beginning of the returned buffer, and a each entry in al_offset contains the
// relative offset of an xfs_attrlist_ent containing the actual entry.
//
// NOTE: struct xfs_attrlist must match struct attrlist defined in libattr, and
// struct xfs_attrlist_ent must match struct attrlist_ent defined in libattr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attrlist {
    pub /: *mut *mut __s32 al_count; / number of entries in attrlist,
    pub /: *mut *mut __s32 al_more; / T/F: more attrs (do call again),
    pub /: *mut *mut __s32 al_offset[]; / byte offsets of attrs [var-sized],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_attrlist_ent {
    pub /: *mut *mut __u32 a_valuelen; / number bytes in value of attr,
    pub /: *mut *mut char a_name[]; / attr name (NULL terminated),
}

//
// per machine unique filesystem identifier types.
//

//
// Structure passed to XFS_IOC_SWAPEXT
//
pub const XFS_SX_VERSION: c_int = 0;
//
// Flags for going down operation
//
pub const XFS_FSOP_GOING_FLAGS_DEFAULT: c_uint = 0x0	/* going down */;
pub const XFS_FSOP_GOING_FLAGS_LOGFLUSH: c_uint = 0x1	/* flush log but not data */;
pub const XFS_FSOP_GOING_FLAGS_NOLOGFLUSH: c_uint = 0x2	/* don't flush log nor data */;
// metadata scrubbing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_scrub_metadata {
    pub /: *mut *mut __u32 sm_type; / What to check?,
    pub /: *mut *mut __u32 sm_flags; / flags; see below.,
    pub /: *mut *mut __u64 sm_ino; / inode number.,
    pub /: *mut *mut __u32 sm_gen; / inode generation.,
    pub /: *mut *mut __u32 sm_agno; / ag number.,
    pub /: *mut *mut __u64 sm_reserved[5]; / pad to 64 bytes,
}

//
// Metadata types and flags for scrub operation.
//
// Scrub subcommands.

// Number of scrub subcommands.
pub const XFS_SCRUB_TYPE_NR: c_int = 33;
//
// This special type code only applies to the vectored scrub implementation.
//
// If any of the previous scrub vectors recorded runtime errors or have
// sv_flags bits set that match the OFLAG bits in the barrier vector's
// sv_flags, set the barrier's sv_ret to -ECANCELED and return to userspace.
//

// i: Repair this metadata.

// o: Metadata object needs repair.

//
// o: Metadata object could be optimized.  It's not corrupt, but
// we could improve on it somehow.
//

// o: Cross-referencing failed.

// o: Metadata object disagrees with cross-referenced metadata.

// o: Scan was not complete.

// o: Metadata object looked funny but isn't corrupt.

//
// o: IFLAG_REPAIR was set but metadata object did not need fixing or
// optimization and has therefore not been altered.
//

// i: Rebuild the data structure.

// Vectored scrub calls to reduce the number of kernel transitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_scrub_vec {
    pub /: *mut *mut *mut __u32 sv_type; / XFS_SCRUB_TYPE_,
    pub /: *mut *mut *mut __u32 sv_flags; / XFS_SCRUB_FLAGS_,
    pub /: *mut *mut __s32 sv_ret; / 0 or a negative error code,
    pub /: *mut *mut __u32 sv_reserved; / must be zero,
}

// Vectored metadata scrub control structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_scrub_vec_head {
    pub /: *mut *mut __u64 svh_ino; / inode number.,
    pub /: *mut *mut __u32 svh_gen; / inode generation.,
    pub /: *mut *mut __u32 svh_agno; / ag number.,
    pub /: *mut *mut *mut __u32 svh_flags; / XFS_SCRUB_VEC_FLAGS_,
    pub /: *mut *mut __u16 svh_rest_us; / wait this much time between vector items,
    pub /: *mut *mut __u16 svh_nr; / number of svh_vectors,
    pub /: *mut *mut __u64 svh_reserved; / must be zero,
    pub /: *mut *mut __u64 svh_vectors; / pointer to buffer of xfs_scrub_vec,
}

//
// i: sm_ino values for XFS_SCRUB_TYPE_METAPATH to select a metadata file for
// path checking.
//

// Number of metapath sm_ino values

//
// ioctl limits
//

//
// Exchange part of file1 with part of the file that this ioctl that is being
// called against (which we'll call file2).  Filesystems must be able to
// restart and complete the operation even after the system goes down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_exchange_range {
    pub file1_fd: __s32,
    pub /: *mut *mut __u32 pad; / must be zeroes,
    pub /: *mut *mut __u64 file1_offset; / file1 offset, bytes,
    pub /: *mut *mut __u64 file2_offset; / file2 offset, bytes,
    pub /: *mut *mut __u64 length; / bytes to exchange,
    pub /: *mut *mut *mut __u64 flags; / see XFS_EXCHANGE_RANGE_ below,
}

//
// Using the same definition of file2 as struct xfs_exchange_range, commit the
// contents of file1 into file2 if file2 has the same inode number, mtime, and
// ctime as the arguments provided to the call.  The old contents of file2 will
// be moved to file1.
//
// Returns -EBUSY if there isn't an exact match for the file2 fields.
//
// Filesystems must be able to restart and complete the operation even after
// the system goes down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_commit_range {
    pub file1_fd: __s32,
    pub /: *mut *mut __u32 pad; / must be zeroes,
    pub /: *mut *mut __u64 file1_offset; / file1 offset, bytes,
    pub /: *mut *mut __u64 file2_offset; / file2 offset, bytes,
    pub /: *mut *mut __u64 length; / bytes to exchange,
    pub /: *mut *mut *mut __u64 flags; / see XFS_EXCHANGE_RANGE_ below,
// opaque file2 metadata for freshness checks
    pub file2_freshness: [__u64; 6],
}

//
// Exchange file data all the way to the ends of both files, and then exchange
// the file sizes.  This flag can be used to replace a file's contents with a
// different amount of data.  length will be ignored.
//

// Flush all changes in file data and file metadata to disk before returning.

// Dry run; do all the parameter verification but do not change anything.

//
// Exchange only the parts of the two files where the file allocation units
// mapped to file1's range have been written to.  This can accelerate
// scatter-gather atomic writes with a temp file if all writes are aligned to
// the file allocation unit.
//

// Iterating parent pointers of files.
// target was the root directory

// Cursor is done iterating pptrs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_getparents_rec {
    pub /: *mut *mut xfs_handle gpr_parent; / Handle to parent,
    pub /: *mut *mut __u32 gpr_reclen; / Length of entire record,
    pub /: *mut *mut __u32 gpr_reserved; / zero,
    pub /: *mut *mut char gpr_name[]; / Null-terminated filename,
}

// Iterate through this file's directory parent pointers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_getparents {
//
// Structure to track progress in iterating the parent pointers.
// Must be initialized to zeroes before the first ioctl call, and
// not touched by callers after that.
//
    pub gp_cursor: xfs_attrlist_cursor,
// Input flags: XFS_GETPARENTS_IFLAG*
    pub gp_iflags: __u16,
// Output flags: XFS_GETPARENTS_OFLAG*
    pub gp_oflags: __u16,
// Size of the gp_buffer in bytes
    pub gp_bufsize: __u32,
// Must be set to zero
    pub gp_reserved: __u64,
// Pointer to a buffer in which to place xfs_getparents_rec
    pub gp_buffer: __u64,
}

// Iterate through this file handle's directory parent pointers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_getparents_by_handle {
// Handle to file whose parents we want.
    pub gph_handle: xfs_handle,
    pub gph_request: xfs_getparents,
}

//
// Output for XFS_IOC_RTGROUP_GEOMETRY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rtgroup_geometry {
    pub /: *mut *mut __u32 rg_number; / i/o: rtgroup number,
    pub /: *mut *mut __u32 rg_length; / o: length in blocks,
    pub /: *mut *mut __u32 rg_sick; / o: sick things in ag,
    pub /: *mut *mut __u32 rg_checked; / o: checked metadata in ag,
    pub /: *mut *mut __u32 rg_flags; / i/o: flags for this ag,
    pub /: *mut *mut __u32 rg_writepointer; / o: write pointer block offset for zoned,
    pub /: *mut *mut __u32 rg_reserved[26]; / o: zero,
}

// Health monitor event domains
// affects the whole fs

// metadata health events

// disk events

// file range events

// Health monitor event types
// status of the monitor itself

// filesystem was unmounted

// metadata health events

// filesystem shutdown

// media errors

// pagecache I/O to a file range failed

// direct I/O to a file range failed

// out of band media error reported for a file range

// lost events
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_lost {
    pub count: __u64,
}

// fs/rt metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_fs {
// XFS_FSOP_GEOM_SICK_* flags
    pub mask: __u32,
}

// ag/rtgroup metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_group {
// XFS_{AG,RTGROUP}_SICK_* flags
    pub mask: __u32,
    pub gno: __u32,
}

// inode metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_inode {
// XFS_BS_SICK_* flags
    pub mask: __u32,
    pub gen: __u32,
    pub ino: __u64,
}

// shutdown reasons

// shutdown
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_shutdown {
// XFS_HEALTH_SHUTDOWN_* flags
    pub reasons: __u32,
}

// file range events
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_filerange {
    pub pos: __u64,
    pub len: __u64,
    pub ino: __u64,
    pub gen: __u32,
    pub error: __u32,
}

// disk media errors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_media {
    pub daddr: __u64,
    pub bbcount: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor_event {
// XFS_HEALTH_MONITOR_DOMAIN_*
    pub domain: __u32,
// XFS_HEALTH_MONITOR_TYPE_*
    pub type: __u32,
// Timestamp of the event, in nanoseconds since the Unix epoch
    pub time_ns: __u64,
//
// Details of the event.  The primary clients are written in python
// and rust, so break this up because bindgen hates anonymous structs
// and unions.
//
    pub lost: xfs_health_monitor_lost,
    pub fs: xfs_health_monitor_fs,
    pub group: xfs_health_monitor_group,
    pub inode: xfs_health_monitor_inode,
    pub shutdown: xfs_health_monitor_shutdown,
    pub media: xfs_health_monitor_media,
    pub filerange: xfs_health_monitor_filerange,
    pub e: },
// zeroes
    pub pad: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_monitor {
    pub /: *mut *mut __u64 flags; / flags,
    pub /: *mut *mut __u8 format; / output format,
    pub /: *mut *mut __u8 pad[23]; / zeroes,
}

// Return all health status events, not just deltas

// Initial return format version

//
// Check that a given fd points to the same filesystem that the health monitor
// is monitoring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_health_file_on_monitored_fs {
    pub fd: __s32,
    pub /: *mut *mut __u32 flags; / zero for now,
}

// Verify the media of the underlying devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_verify_media {
    pub /: *mut *mut __u32 me_dev; / I: XFS_DEV_{DATA,LOG,RT},
    pub /: *mut *mut *mut __u32 me_flags; / I: XFS_VERIFY_MEDIA_,
//
// IO: inclusive start of disk range to verify, in 512b blocks.
// Will be adjusted upwards as media verification succeeds.
//
    pub me_start_daddr: __u64,
//
// IO: exclusive end of the disk range to verify, in 512b blocks.
// Can be adjusted downwards to match device size.
//
    pub me_end_daddr: __u64,
    pub /: *mut *mut __u32 me_ioerror; / O: I/O error (positive),
    pub /: *mut *mut __u32 me_max_io_size; / I: maximum IO size in bytes,
    pub /: *mut *mut __u32 me_rest_us; / I: rest time between IOs, usecs,
    pub /: *mut *mut __u32 me_pad; / zero,
}

//
// ioctl commands that are used by Linux filesystems
//

//
// ioctl commands that replace IRIX fcntl()'s
// For 'documentation' purposed more than anything else,
// the "cmd #" field reflects the IRIX fcntl number.
//
// XFS_IOC_ALLOCSP ------- deprecated 10
// XFS_IOC_FREESP -------- deprecated 11

// XFS_IOC_ALLOCSP64 ----- deprecated 36
// XFS_IOC_FREESP64 ------ deprecated 37

// XFS_IOC_FSSETDM ------- deprecated 39

// XFS_IOC_SETBIOSIZE ---- deprecated 46
// XFS_IOC_GETBIOSIZE ---- deprecated 47

// XFS_IOC_GETFSMAP ------ hoisted 59

//
// ioctl commands that replace IRIX syssgi()'s
//

// XFS_IOC_ATTRCTL_BY_HANDLE -- deprecated 118

// XFS_IOC_FSSETDM_BY_HANDLE -- deprecated 121

// XFS_IOC_GETFSUUID ---------- deprecated 140
//
// Devices supported by a single XFS file system.  Reported in fsmaps fmr_device
// when using internal RT devices.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_device {
    XFS_DEV_DATA	= 1,
    XFS_DEV_LOG	= 2,
    XFS_DEV_RT	= 3,
}

//
// Block I/O parameterization.	A basic block (BB) is the lowest size of
// filesystem allocation, and must equal 512.  Length units given to bio
// routines are in BB's.
//
pub const BBSHIFT: c_int = 9;

