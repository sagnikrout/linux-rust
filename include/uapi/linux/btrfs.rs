//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/btrfs.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public
// License v2 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with this program; if not, write to the
// Free Software Foundation, Inc., 59 Temple Place - Suite 330,
// Boston, MA 021110-1307, USA.
//

pub const BTRFS_IOCTL_MAGIC: c_uint = 0x94;
pub const BTRFS_VOL_NAME_MAX: c_int = 255;
pub const BTRFS_LABEL_SIZE: c_int = 256;
// this should be 4k
pub const BTRFS_PATH_NAME_MAX: c_int = 4087;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_vol_args {
    pub fd: __s64,
    pub 1]: char name[BTRFS_PATH_NAME_MAX +,
}

pub const BTRFS_DEVICE_PATH_NAME_MAX: c_int = 1024;
pub const BTRFS_SUBVOL_NAME_MAX: c_int = 4039;
// Deprecated since 5.7

pub const BTRFS_FSID_SIZE: c_int = 16;
pub const BTRFS_UUID_SIZE: c_int = 16;
pub const BTRFS_UUID_UNPARSED_SIZE: c_int = 37;
//
// flags definition for qgroup limits
//
// Used by:
// struct btrfs_qgroup_limit.flags
// struct btrfs_qgroup_limit_item.flags
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_limit {
    pub flags: __u64,
    pub max_rfer: __u64,
    pub max_excl: __u64,
    pub rsv_rfer: __u64,
    pub rsv_excl: __u64,
}

//
// flags definition for qgroup inheritance
//
// Used by:
// struct btrfs_qgroup_inherit.flags
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_inherit {
    pub flags: __u64,
    pub num_qgroups: __u64,
    pub num_ref_copies: __u64,
    pub num_excl_copies: __u64,
    pub lim: btrfs_qgroup_limit,
    pub qgroups: [__u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_qgroup_limit_args {
    pub qgroupid: __u64,
    pub lim: btrfs_qgroup_limit,
}

//
// Arguments for specification of subvolumes or devices, supporting by-name or
// by-id and flags
//
// The set of supported flags depends on the ioctl
//
// BTRFS_SUBVOL_RDONLY is also provided/consumed by the following ioctls:
// - BTRFS_IOC_SUBVOL_GETFLAGS
// - BTRFS_IOC_SUBVOL_SETFLAGS
//
// Supported flags for BTRFS_IOC_RM_DEV_V2

// Supported flags for BTRFS_IOC_SNAP_CREATE_V2 and BTRFS_IOC_SUBVOL_CREATE_V2

// Supported flags for BTRFS_IOC_SNAP_DESTROY_V2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_vol_args_v2 {
    pub fd: __s64,
    pub transid: __u64,
    pub flags: __u64,
    pub size: __u64,
    pub qgroup_inherit: *mut btrfs_qgroup_inherit __user,
}

//
// structure to report errors and progress to userspace, either as a
// result of a finished scrub, a canceled scrub or a progress inquiry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_scrub_progress {
    pub /: *mut *mut __u64 data_extents_scrubbed; / # of data extents scrubbed,
    pub /: *mut *mut __u64 tree_extents_scrubbed; / # of tree extents scrubbed,
    pub /: *mut *mut __u64 data_bytes_scrubbed; / # of data bytes scrubbed,
    pub /: *mut *mut __u64 tree_bytes_scrubbed; / # of tree bytes scrubbed,
    pub /: *mut *mut __u64 read_errors; / # of read errors encountered (EIO),
    pub /: *mut *mut __u64 csum_errors; / # of failed csum checks,
    pub metadata: *mut *mut __u64 verify_errors; / # of occurrences, where the,
// of a tree block did not match the
// expected values, like generation or
// logical
    pub csum: *mut *mut __u64 no_csum; / # of 4k data block for which no,
// is present, probably the result of
// data written with nodatasum
    pub found: *mut *mut __u64 csum_discards; / # of csum for which no data was,
// in the extent tree.
    pub /: *mut *mut __u64 super_errors; / # of bad super blocks encountered,
    pub These: *mut *mut __u64 malloc_errors; / # of internal kmalloc errors.,
// will likely cause an incomplete
// scrub
    pub intact: *mut *mut __u64 uncorrectable_errors; / # of errors where either no,
// copy was found or the writeback
// failed
    pub /: *mut *mut __u64 corrected_errors; / # of errors corrected,
    pub In: *mut *mut __u64 last_physical; / last physical address scrubbed.,
// case a scrub was aborted, this can
// be used to restart the scrub
    pub a: *mut *mut __u64 unverified_errors; / # of occurrences where a read for,
// full (64k) bio failed, but the re-
// check succeeded for each 4k piece.
// Intermittent error.
}

pub const BTRFS_SCRUB_READONLY: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_scrub_args {
    pub /: *mut *mut __u64 devid; / in,
    pub /: *mut *mut __u64 start; / in,
    pub /: *mut *mut __u64 end; / in,
    pub /: *mut *mut __u64 flags; / in,
    pub /: *mut *mut btrfs_scrub_progress progress; / out,
// pad to 1k
    pub btrfs_scrub_progress))/8]: __u64 unused[(1024-32-sizeof(struct,
}

pub const BTRFS_IOCTL_DEV_REPLACE_CONT_READING_FROM_SRCDEV_MODE_ALWAYS: c_int = 0;
pub const BTRFS_IOCTL_DEV_REPLACE_CONT_READING_FROM_SRCDEV_MODE_AVOID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_dev_replace_start_params {
    pub /: *mut *mut __u64 srcdevid; / in, if 0, use srcdev_name instead,
    pub #define: *mut *mut __u64 cont_reading_from_srcdev_mode; / in, see,
// above
    pub /: *mut *mut __u8 srcdev_name[BTRFS_DEVICE_PATH_NAME_MAX + 1]; / in,
    pub /: *mut *mut __u8 tgtdev_name[BTRFS_DEVICE_PATH_NAME_MAX + 1]; / in,
}

pub const BTRFS_IOCTL_DEV_REPLACE_STATE_NEVER_STARTED: c_int = 0;
pub const BTRFS_IOCTL_DEV_REPLACE_STATE_STARTED: c_int = 1;
pub const BTRFS_IOCTL_DEV_REPLACE_STATE_FINISHED: c_int = 2;
pub const BTRFS_IOCTL_DEV_REPLACE_STATE_CANCELED: c_int = 3;
pub const BTRFS_IOCTL_DEV_REPLACE_STATE_SUSPENDED: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_dev_replace_status_params {
    pub /: *mut *mut __u64 replace_state; / out, see #define above,
    pub /: *mut *mut __u64 progress_1000; / out, 0 <= x <= 1000,
    pub /: *mut *mut __u64 time_started; / out, seconds since 1-Jan-1970,
    pub /: *mut *mut __u64 time_stopped; / out, seconds since 1-Jan-1970,
    pub /: *mut *mut __u64 num_write_errors; / out,
    pub /: *mut *mut __u64 num_uncorrectable_read_errors; / out,
}

pub const BTRFS_IOCTL_DEV_REPLACE_CMD_START: c_int = 0;
pub const BTRFS_IOCTL_DEV_REPLACE_CMD_STATUS: c_int = 1;
pub const BTRFS_IOCTL_DEV_REPLACE_CMD_CANCEL: c_int = 2;
pub const BTRFS_IOCTL_DEV_REPLACE_RESULT_NO_ERROR: c_int = 0;
pub const BTRFS_IOCTL_DEV_REPLACE_RESULT_NOT_STARTED: c_int = 1;
pub const BTRFS_IOCTL_DEV_REPLACE_RESULT_ALREADY_STARTED: c_int = 2;
pub const BTRFS_IOCTL_DEV_REPLACE_RESULT_SCRUB_INPROGRESS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_dev_replace_args {
    pub /: *mut *mut __u64 cmd; / in,
    pub /: *mut *mut __u64 result; / out,
    pub start: btrfs_ioctl_dev_replace_start_params,
    pub status: btrfs_ioctl_dev_replace_status_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_dev_info_args {
    pub /: *mut *mut __u64 devid; / in/out,
    pub /: *mut *mut __u8 uuid[BTRFS_UUID_SIZE]; / in/out,
    pub /: *mut *mut __u64 bytes_used; / out,
    pub /: *mut *mut __u64 total_bytes; / out,
//
// Optional, out.
//
// Showing the fsid of the device, allowing user space to check if this
// device is a seeding one.
//
// Introduced in v6.3, thus user space still needs to check if kernel
// changed this value.  Older kernel will not touch the values here.
//
    pub fsid: [__u8; BTRFS_UUID_SIZE],
    pub /: *mut *mut __u64 unused[377]; / pad to 4k,
    pub /: *mut *mut __u8 path[BTRFS_DEVICE_PATH_NAME_MAX]; / out,
}

//
// Retrieve information about the filesystem
//
// Request information about checksum type and size

// Request information about filesystem generation

// Request information about filesystem metadata UUID

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_fs_info_args {
    pub /: *mut *mut __u64 max_id; / out,
    pub /: *mut *mut __u64 num_devices; / out,
    pub /: *mut *mut __u8 fsid[BTRFS_FSID_SIZE]; / out,
    pub /: *mut *mut __u32 nodesize; / out,
    pub /: *mut *mut __u32 sectorsize; / out,
    pub /: *mut *mut __u32 clone_alignment; / out,
// See BTRFS_FS_INFO_FLAG_*
    pub /: *mut *mut __u16 csum_type; / out,
    pub /: *mut *mut __u16 csum_size; / out,
    pub /: *mut *mut __u64 flags; / in/out,
    pub /: *mut *mut __u64 generation; / out,
    pub /: *mut *mut __u8 metadata_uuid[BTRFS_FSID_SIZE]; / out,
    pub /: *mut *mut __u8 reserved[944]; / pad to 1k,
}

//
// feature flags
//
// Used by:
// struct btrfs_ioctl_feature_flags
//

//
// Older kernels (< 4.9) on big-endian systems produced broken free space tree
// bitmaps, and btrfs-progs also used to corrupt the free space tree (versions
// < 4.7.3).  If this bit is clear, then the free space tree cannot be trusted.
// btrfs-progs can also intentionally clear this bit to ask the kernel to
// rebuild the free space tree, however this might not work on older kernels
// that do not know about this bit. If not sure, clear the cache manually on
// first mount when booting older kernel versions.
//

//
// Put all block group items into a dedicated block group tree, greatly
// reducing mount time for large filesystem due to better locality.
//

//
// older kernels tried to do bigger metadata blocks, but the
// code was pretty buggy.  Lets not let them try anymore.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_feature_flags {
    pub compat_flags: __u64,
    pub compat_ro_flags: __u64,
    pub incompat_flags: __u64,
}

// balance control ioctl modes
pub const BTRFS_BALANCE_CTL_PAUSE: c_int = 1;
pub const BTRFS_BALANCE_CTL_CANCEL: c_int = 2;
//
// this is packed, because it should be exactly the same as its disk
// byte order counterpart (struct btrfs_disk_balance_args)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_balance_args {
    pub profiles: __u64,
//
// usage filter
// BTRFS_BALANCE_ARGS_USAGE with a single value means '0..N'
// BTRFS_BALANCE_ARGS_USAGE_RANGE - range syntax, min..max
//
    pub usage: __u64,
    pub usage_min: __u32,
    pub usage_max: __u32,
}

//
// BTRFS_BALANCE_ARGS_LIMIT with value 'limit'
// BTRFS_BALANCE_ARGS_LIMIT_RANGE - the extend version can use minimum
// and maximum
//
// Process chunks that cross stripes_min..stripes_max devices,
// BTRFS_BALANCE_ARGS_STRIPES_RANGE
//
// report balance progress to userspace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_balance_progress {
    pub be: *mut *mut __u64 expected; / estimated # of chunks that will,
// relocated to fulfill the request
    pub /: *mut *mut __u64 considered; / # of chunks we have considered so far,
    pub /: *mut *mut __u64 completed; / # of chunks relocated so far,
}

//
// flags definition for balance
//
// Restriper's general type filter
//
// Used by:
// btrfs_ioctl_balance_args.flags
// btrfs_balance_control.flags (internal)
//

//
// flags definitions for per-type balance args
//
// Balance filters
//
// Used by:
// struct btrfs_balance_args
//

//
// Profile changing flags.  When SOFT is set we won't relocate chunk if
// it already has the target profile (even though it may be
// half-filled).
//

//
// flags definition for balance state
//
// Used by:
// struct btrfs_ioctl_balance_args.state
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_balance_args {
    pub /: *mut *mut __u64 flags; / in/out,
    pub /: *mut *mut __u64 state; / out,
    pub /: *mut *mut btrfs_balance_args data; / in/out,
    pub /: *mut *mut btrfs_balance_args meta; / in/out,
    pub /: *mut *mut btrfs_balance_args sys; / in/out,
    pub /: *mut *mut btrfs_balance_progress stat; / out,
    pub /: *mut *mut __u64 unused[72]; / pad to 1k,
}

pub const BTRFS_INO_LOOKUP_PATH_MAX: c_int = 4080;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_ino_lookup_args {
    pub treeid: __u64,
    pub objectid: __u64,
    pub name: [c_char; BTRFS_INO_LOOKUP_PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_ino_lookup_user_args {
// in, inode number containing the subvolume of 'subvolid'
    pub dirid: __u64,
// in
    pub treeid: __u64,
// out, name of the subvolume of 'treeid'
    pub 1]: char name[BTRFS_VOL_NAME_MAX +,
//
// out, constructed path from the directory with which the ioctl is
// called to dirid
//
    pub path: [c_char; BTRFS_INO_LOOKUP_USER_PATH_MAX],
}

// Search criteria for the btrfs SEARCH ioctl family.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_search_key {
//
// The tree we're searching in. 1 is the tree of tree roots, 2 is the
// extent tree, etc...
//
// A special tree_id value of 0 will cause a search in the subvolume
// tree that the inode which is passed to the ioctl is part of.
//
    pub /: *mut *mut __u64 tree_id; / in,
//
// When doing a tree search, we're actually taking a slice from a
// linear search space of 136-bit keys.
//
// A full 136-bit tree key is composed as:
// (objectid << 72) + (type << 64) + offset
//
// The individual min and max values for objectid, type and offset
// define the min_key and max_key values for the search range. All
// metadata items with a key in the interval [min_key, max_key] will be
// returned.
//
// Additionally, we can filter the items returned on transaction id of
// the metadata block they're stored in by specifying a transid range.
// Be aware that this transaction id only denotes when the metadata
// page that currently contains the item got written the last time as
// result of a COW operation.  The number does not have any meaning
// related to the transaction in which an individual item that is being
// returned was created or changed.
//
    pub /: *mut *mut __u64 min_objectid; / in,
    pub /: *mut *mut __u64 max_objectid; / in,
    pub /: *mut *mut __u64 min_offset; / in,
    pub /: *mut *mut __u64 max_offset; / in,
    pub /: *mut *mut __u64 min_transid; / in,
    pub /: *mut *mut __u64 max_transid; / in,
    pub /: *mut *mut __u32 min_type; / in,
    pub /: *mut *mut __u32 max_type; / in,
//
// input: The maximum amount of results desired.
// output: The actual amount of items returned, restricted by any of:
// - reaching the upper bound of the search range
// - reaching the input nr_items amount of items
// - completely filling the supplied memory buffer
//
    pub /: *mut *mut __u32 nr_items; / in/out,
// align to 64 bits
    pub unused: __u32,
// some extra for later
    pub unused1: __u64,
    pub unused2: __u64,
    pub unused3: __u64,
    pub unused4: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_search_header {
    pub transid: __u64,
    pub objectid: __u64,
    pub offset: __u64,
    pub type: __u32,
    pub len: __u32,
// C attribute field omitted

//
// the buf is an array of search headers where
// each header is followed by the actual item
// the type field is expanded to 32 bits for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_search_args {
    pub key: btrfs_ioctl_search_key,
    pub buf: [c_char; BTRFS_SEARCH_ARGS_BUFSIZE],
}

//
// Extended version of TREE_SEARCH ioctl that can return more than 4k of bytes.
// The allocated size of the buffer is set in buf_size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_search_args_v2 {
    pub /: *mut *mut btrfs_ioctl_search_key key; / in/out - search parameters,
    pub buffer: *mut *mut __u64 buf_size; / in - size of,
// out - on EOVERFLOW: needed size
// to store item
    pub /: *mut *mut __u8 buf[]; / out - found items,
}

// With a @src_length of zero, the range from @src_offset->EOF is cloned!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_clone_range_args {
    pub src_fd: __s64,
    pub src_length: __u64 src_offset,,
    pub dest_offset: __u64,
}

//
// flags definition for the defrag range ioctl
//
// Used by:
// struct btrfs_ioctl_defrag_range_args.flags
//
pub const BTRFS_DEFRAG_RANGE_COMPRESS: c_int = 1;
pub const BTRFS_DEFRAG_RANGE_START_IO: c_int = 2;
pub const BTRFS_DEFRAG_RANGE_COMPRESS_LEVEL: c_int = 4;
// Request no compression on the range (uncompress if necessary).
pub const BTRFS_DEFRAG_RANGE_NOCOMPRESS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_defrag_range_args {
// start of the defrag operation
    pub start: __u64,
// number of bytes to defrag, use (u64)-1 to say all
    pub len: __u64,
//
// flags for the operation, which can include turning
// on compression for this one defrag
//
    pub flags: __u64,
//
// any extent bigger than this will be considered
// already defragged.  Use 0 to take the kernel default
// Use 1 to say every single extent must be rewritten
//
    pub extent_thresh: __u32,
//
// which compression method to use if turning on compression
// for this defrag operation. If unspecified, zlib will be
// used. If compression level is also being specified, set the
// BTRFS_DEFRAG_RANGE_COMPRESS_LEVEL flag and fill the compress
// member structure instead of the compress_type field.
//
    pub compress_type: __u32,
    pub type: __u8,
    pub level: __s8,
    pub compress: },
}

// spare for later
pub const BTRFS_SAME_DATA_DIFFERS: c_int = 1;
// For extent-same ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_same_extent_info {
    pub /: *mut *mut __s64 fd; / in - destination file,
    pub /: *mut *mut __u64 logical_offset; / in - start of extent in destination,
    pub able: *mut *mut __u64 bytes_deduped; / out - total # of bytes we were,
// to dedupe from this file
// status of this dedupe operation:
// 0 if dedup succeeds
// < 0 for error
// == BTRFS_SAME_DATA_DIFFERS if data differs
//
    pub /: *mut *mut __s32 status; / out - see above description,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_same_args {
    pub /: *mut *mut __u64 logical_offset; / in - start of extent in source,
    pub /: *mut *mut __u64 length; / in - length of extent,
    pub /: *mut *mut __u16 dest_count; / in - total elements in info array,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub info: [btrfs_ioctl_same_extent_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_space_info {
    pub flags: __u64,
    pub total_bytes: __u64,
    pub used_bytes: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_space_args {
    pub space_slots: __u64,
    pub total_spaces: __u64,
    pub spaces: [btrfs_ioctl_space_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_data_container {
    pub /: *mut *mut __u32 bytes_left; / out -- bytes not needed to deliver output,
    pub /: *mut *mut __u32 bytes_missing; / out -- additional bytes needed for result,
    pub /: *mut *mut __u32 elem_cnt; / out,
    pub /: *mut *mut __u32 elem_missed; / out,
    pub /: *mut *mut __u64 val[]; / out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_ino_path_args {
    pub /: *mut *mut __u64 inum; / in,
    pub /: *mut *mut __u64 size; / in,
    pub reserved: [__u64; 4],
// struct btrfs_data_container	*fspath;	   out
    pub /: *mut *mut __u64 fspath; / out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_logical_ino_args {
    pub /: *mut *mut __u64 logical; / in,
    pub /: *mut *mut __u64 size; / in,
    pub /: *mut *mut __u64 reserved[3]; / must be 0 for now,
    pub /: *mut *mut __u64 flags; / in, v2 only,
// struct btrfs_data_container	*inodes;	out
    pub inodes: __u64,
}

//
// Return every ref to the extent, not just those containing logical block.
// Requires logical == extent bytenr.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_dev_stat_values {
// disk I/O failure stats
    BTRFS_DEV_STAT_WRITE_ERRS, /* EIO or EREMOTEIO from lower layers */
    BTRFS_DEV_STAT_READ_ERRS, /* EIO or EREMOTEIO from lower layers */
    BTRFS_DEV_STAT_FLUSH_ERRS, /* EIO or EREMOTEIO from lower layers */

// stats for indirect indications for I/O failures
    BTRFS_DEV_STAT_CORRUPTION_ERRS, /* checksum error, bytenr error or
// contents is illegal: this is an
// indication that the block was damaged
// during read or write, or written to
// wrong location or read from wrong
// location
    BTRFS_DEV_STAT_GENERATION_ERRS, /* an indication that blocks have not
// been written

    BTRFS_DEV_STAT_VALUES_MAX
}

// Reset statistics after reading; needs SYS_ADMIN capability

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_get_dev_stats {
    pub /: *mut *mut __u64 devid; / in,
    pub /: *mut *mut __u64 nr_items; / in/out,
    pub /: *mut *mut __u64 flags; / in/out,
// out values:
    pub values: [__u64; BTRFS_DEV_STAT_VALUES_MAX],
//
// This pads the struct to 1032 bytes. It was originally meant to pad to
// 1024 bytes, but when adding the flags field, the padding calculation
// was not adjusted.
//
    pub BTRFS_DEV_STAT_VALUES_MAX]: __u64 unused[128 - 2 -,
}

pub const BTRFS_QUOTA_CTL_ENABLE: c_int = 1;
pub const BTRFS_QUOTA_CTL_DISABLE: c_int = 2;
pub const BTRFS_QUOTA_CTL_RESCAN__NOTUSED: c_int = 3;
pub const BTRFS_QUOTA_CTL_ENABLE_SIMPLE_QUOTA: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_quota_ctl_args {
    pub cmd: __u64,
    pub status: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_quota_rescan_args {
    pub flags: __u64,
    pub progress: __u64,
    pub reserved: [__u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_qgroup_assign_args {
    pub assign: __u64,
    pub src: __u64,
    pub dst: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_qgroup_create_args {
    pub create: __u64,
    pub qgroupid: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_timespec {
    pub sec: __u64,
    pub nsec: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_received_subvol_args {
    pub /: *mut *mut char uuid[BTRFS_UUID_SIZE]; / in,
    pub /: *mut *mut __u64 stransid; / in,
    pub /: *mut *mut __u64 rtransid; / out,
    pub /: *mut *mut btrfs_ioctl_timespec stime; / in,
    pub /: *mut *mut btrfs_ioctl_timespec rtime; / out,
    pub /: *mut *mut __u64 flags; / in,
    pub /: *mut *mut __u64 reserved[16]; / in,
}

//
// Caller doesn't want file data in the send stream, even if the
// search of clone sources doesn't find an extent. UPDATE_EXTENT
// commands will be sent instead of WRITE commands.
//
pub const BTRFS_SEND_FLAG_NO_FILE_DATA: c_uint = 0x1;
//
// Do not add the leading stream header. Used when multiple snapshots
// are sent back to back.
//
pub const BTRFS_SEND_FLAG_OMIT_STREAM_HEADER: c_uint = 0x2;
//
// Omit the command at the end of the stream that indicated the end
// of the stream. This option is used when multiple snapshots are
// sent back to back.
//
pub const BTRFS_SEND_FLAG_OMIT_END_CMD: c_uint = 0x4;
//
// Read the protocol version in the structure
//
pub const BTRFS_SEND_FLAG_VERSION: c_uint = 0x8;
//
// Send compressed data using the ENCODED_WRITE command instead of decompressing
// the data and sending it with the WRITE command. This requires protocol
// version >= 2.
//
pub const BTRFS_SEND_FLAG_COMPRESSED: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_send_args {
    pub /: *mut *mut __s64 send_fd; / in,
    pub /: *mut *mut __u64 clone_sources_count; / in,
    pub /: *mut *mut *mut __u64 __user clone_sources; / in,
    pub /: *mut *mut __u64 parent_root; / in,
    pub /: *mut *mut __u64 flags; / in,
    pub /: *mut *mut __u32 version; / in,
    pub /: *mut *mut __u8 reserved[28]; / in,
}

//
// Information about a fs tree root.
//
// All items are filled by the ioctl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_get_subvol_info_args {
// Id of this subvolume
    pub treeid: __u64,
// Name of this subvolume, used to get the real name at mount point
    pub 1]: char name[BTRFS_VOL_NAME_MAX +,
//
// Id of the subvolume which contains this subvolume.
// Zero for top-level subvolume or a deleted subvolume.
//
    pub parent_id: __u64,
//
// Inode number of the directory which contains this subvolume.
// Zero for top-level subvolume or a deleted subvolume
//
    pub dirid: __u64,
// Latest transaction id of this subvolume
    pub generation: __u64,
// Flags of this subvolume
    pub flags: __u64,
// UUID of this subvolume
    pub uuid: [__u8; BTRFS_UUID_SIZE],
//
// UUID of the subvolume of which this subvolume is a snapshot.
// All zero for a non-snapshot subvolume.
//
    pub parent_uuid: [__u8; BTRFS_UUID_SIZE],
//
// UUID of the subvolume from which this subvolume was received.
// All zero for non-received subvolume.
//
    pub received_uuid: [__u8; BTRFS_UUID_SIZE],
// Transaction id indicating when change/create/send/receive happened
    pub ctransid: __u64,
    pub otransid: __u64,
    pub stransid: __u64,
    pub rtransid: __u64,
// Time corresponding to c/o/s/rtransid
    pub ctime: btrfs_ioctl_timespec,
    pub otime: btrfs_ioctl_timespec,
    pub stime: btrfs_ioctl_timespec,
    pub rtime: btrfs_ioctl_timespec,
// Must be zero
    pub reserved: [__u64; 8],
}

pub const BTRFS_MAX_ROOTREF_BUFFER_NUM: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_get_subvol_rootref_args {
// in/out, minimum id of rootref's treeid to be searched
    pub min_treeid: __u64,
// out
    pub treeid: __u64,
    pub dirid: __u64,
    pub rootref: [}; BTRFS_MAX_ROOTREF_BUFFER_NUM],
// out, number of found items
    pub num_items: __u8,
    pub align: [__u8; 7],
}

//
// Data and metadata for an encoded read or write.
//
// Encoded I/O bypasses any encoding automatically done by the filesystem (e.g.,
// compression). This can be used to read the compressed contents of a file or
// write pre-compressed data directly to a file.
//
// BTRFS_IOC_ENCODED_READ and BTRFS_IOC_ENCODED_WRITE are essentially
// preadv/pwritev with additional metadata about how the data is encoded and the
// size of the unencoded data.
//
// BTRFS_IOC_ENCODED_READ fills the given iovecs with the encoded data, fills
// the metadata fields, and returns the size of the encoded data. It reads one
// extent per call. It can also read data which is not encoded.
//
// BTRFS_IOC_ENCODED_WRITE uses the metadata fields, writes the encoded data
// from the iovecs, and returns the size of the encoded data. Note that the
// encoded data is not validated when it is written; if it is not valid (e.g.,
// it cannot be decompressed), then a subsequent read may return an error.
//
// Since the filesystem page cache contains decoded data, encoded I/O bypasses
// the page cache. Encoded I/O requires CAP_SYS_ADMIN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_encoded_io_args {
// Input parameters for both reads and writes.
//
// iovecs containing encoded data.
//
// For reads, if the size of the encoded data is larger than the sum of
// iov[n].iov_len for 0 <= n < iovcnt, then the ioctl fails with
// ENOBUFS.
//
// For writes, the size of the encoded data is the sum of iov[n].iov_len
// for 0 <= n < iovcnt. This must be less than 128 KiB (this limit may
// increase in the future). This must also be less than or equal to
// unencoded_len.
//
    pub iov: *const iovec __user,
// Number of iovecs.
    pub iovcnt: c_ulong,
//
// Offset in file.
//
// For writes, must be aligned to the sector size of the filesystem.
//
    pub offset: __s64,
// Currently must be zero.
    pub flags: __u64,
//
// For reads, the following members are output parameters that will
// contain the returned metadata for the encoded data.
// For writes, the following members must be set to the metadata for the
// encoded data.
//
// Length of the data in the file.
//
// Must be less than or equal to unencoded_len - unencoded_offset. For
// writes, must be aligned to the sector size of the filesystem unless
// the data ends at or beyond the current end of the file.
//
    pub len: __u64,
//
// Length of the unencoded (i.e., decrypted and decompressed) data.
//
// For writes, must be no more than 128 KiB (this limit may increase in
// the future). If the unencoded data is actually longer than
// unencoded_len, then it is truncated; if it is shorter, then it is
// extended with zeroes.
//
    pub unencoded_len: __u64,
//
// Offset from the first byte of the unencoded data to the first byte of
// logical data in the file.
//
// Must be less than unencoded_len.
//
    pub unencoded_offset: __u64,
//
// BTRFS_ENCODED_IO_COMPRESSION_* type.
//
// For writes, must not be BTRFS_ENCODED_IO_COMPRESSION_NONE.
//
    pub compression: __u32,
// Currently always BTRFS_ENCODED_IO_ENCRYPTION_NONE.
    pub encryption: __u32,
//
// Reserved for future expansion.
//
// For reads, always returned as zero. Users should check for non-zero
// bytes. If there are any, then the kernel has a newer version of this
// structure with additional information that the user definition is
// missing.
//
// For writes, must be zeroed.
//
    pub reserved: [__u8; 64],
}

// Data is not compressed.
pub const BTRFS_ENCODED_IO_COMPRESSION_NONE: c_int = 0;
// Data is compressed as a single zlib stream.
pub const BTRFS_ENCODED_IO_COMPRESSION_ZLIB: c_int = 1;
//
// Data is compressed as a single zstd frame with the windowLog compression
// parameter set to no more than 17.
//
pub const BTRFS_ENCODED_IO_COMPRESSION_ZSTD: c_int = 2;
//
// Data is compressed sector by sector (using the sector size indicated by the
// name of the constant) with LZO1X and wrapped in the format documented in
// fs/btrfs/lzo.c. For writes, the compression sector size must match the
// filesystem sector size.
//
pub const BTRFS_ENCODED_IO_COMPRESSION_LZO_4K: c_int = 3;
pub const BTRFS_ENCODED_IO_COMPRESSION_LZO_8K: c_int = 4;
pub const BTRFS_ENCODED_IO_COMPRESSION_LZO_16K: c_int = 5;
pub const BTRFS_ENCODED_IO_COMPRESSION_LZO_32K: c_int = 6;
pub const BTRFS_ENCODED_IO_COMPRESSION_LZO_64K: c_int = 7;
pub const BTRFS_ENCODED_IO_COMPRESSION_TYPES: c_int = 8;
// Data is not encrypted.
pub const BTRFS_ENCODED_IO_ENCRYPTION_NONE: c_int = 0;
pub const BTRFS_ENCODED_IO_ENCRYPTION_TYPES: c_int = 1;
//
// Wait for subvolume cleaning process. This queries the kernel queue and it
// can change between the calls.
//
// - FOR_ONE	- specify the subvolid
// - FOR_QUEUED - wait for all currently queued
// - COUNT	- count number of queued
// - PEEK_FIRST - read which is the first in the queue (to be cleaned or being
// cleaned already), or 0 if the queue is empty
// - PEEK_LAST  - read the last subvolid in the queue, or 0 if the queue is empty
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_subvol_wait {
    pub subvolid: __u64,
    pub mode: __u32,
    pub count: __u32,
}

// Error codes as returned by the kernel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_err_code {
    BTRFS_ERROR_DEV_RAID1_MIN_NOT_MET = 1,
    BTRFS_ERROR_DEV_RAID10_MIN_NOT_MET,
    BTRFS_ERROR_DEV_RAID5_MIN_NOT_MET,
    BTRFS_ERROR_DEV_RAID6_MIN_NOT_MET,
    BTRFS_ERROR_DEV_TGT_REPLACE,
    BTRFS_ERROR_DEV_MISSING_NOT_FOUND,
    BTRFS_ERROR_DEV_ONLY_WRITABLE,
    BTRFS_ERROR_DEV_EXCL_RUN_IN_PROGRESS,
    BTRFS_ERROR_DEV_RAID1C3_MIN_NOT_MET,
    BTRFS_ERROR_DEV_RAID1C4_MIN_NOT_MET,
}

// Flags for struct btrfs_ioctl_get_csums_entry::type.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_get_csums_entry {
// File offset of this range.
    pub offset: __u64,
// Length in bytes.
    pub length: __u64,
// One of BTRFS_GET_CSUMS_* types.
    pub type: __u32,
// Padding, must be 0.
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ioctl_get_csums_args {
// In/out: file offset in bytes.
    pub offset: __u64,
// In/out: range length in bytes.
    pub length: __u64,
// In/out: buffer capacity / bytes written.
    pub buf_size: __u64,
// In: flags, must be 0 for now.
    pub flags: __u64,
// Out: entries of type btrfs_ioctl_get_csums_entry + csum data
    pub buf: [__u8; ],
}

// Flags for IOC_SHUTDOWN, must match XFS_FSOP_GOING_FLAGS_* flags.
pub const BTRFS_SHUTDOWN_FLAGS_DEFAULT: c_uint = 0x0;
pub const BTRFS_SHUTDOWN_FLAGS_LOGFLUSH: c_uint = 0x1;
pub const BTRFS_SHUTDOWN_FLAGS_NOLOGFLUSH: c_uint = 0x2;
pub const BTRFS_SHUTDOWN_FLAGS_LAST: c_uint = 0x3;

// trans start and trans end are dangerous, and only for
// use by applications that know how to avoid the
// resulting deadlocks
//

// Shutdown ioctl should follow XFS's interfaces, thus not using btrfs magic.

