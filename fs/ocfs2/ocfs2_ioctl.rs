//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/ocfs2_ioctl.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ocfs2_ioctl.h
//
// Defines OCFS2 ioctls.
//
// Copyright (C) 2010 Oracle.  All rights reserved.
//
// Space reservation / allocation / free ioctls and argument structure
// are designed to be compatible with XFS.
//
// ALLOCSP* and FREESP* are not and will never be supported, but are
// included here for completeness.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_space_resv {
    pub l_type: __s16,
    pub l_whence: __s16,
    pub l_start: __s64,
    pub /: *mut *mut __s64 l_len; / len == 0 means until end of file,
    pub l_sysid: __s32,
    pub l_pid: __u32,
    pub /: *mut *mut __s32 l_pad[4]; / reserve area,
}

// Used to pass group descriptor data when online resize is done
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_new_group_input {
    pub /: *mut *mut __u64 group; / Group descriptor's blkno.,
    pub /: *mut *mut __u32 clusters; / Total number of clusters in this group,
    pub /: *mut *mut __u32 frees; / Total free clusters in this group,
    pub /: *mut *mut __u16 chain; / Chain for this group,
    pub reserved1: __u16,
    pub reserved2: __u32,
}

// Used to pass 2 file names to reflink.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reflink_arguments {
    pub old_path: __u64,
    pub new_path: __u64,
    pub preserve: __u64,
}

// Following definitions dedicated for ocfs2_info_request ioctls.

// Magic number of all requests

//
// Always try to separate info request into small pieces to
// guarantee the backward&forward compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info {
    pub /: *mut *mut __u64 oi_requests; / Array of __u64 pointers to requests,
    pub /: *mut *mut __u32 oi_count; / Number of requests in info_requests,
    pub oi_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_request {
// 00*/	__u32 ir_magic;	/* Magic number
    pub /: *mut *mut __u32 ir_code; / Info request code,
    pub /: *mut *mut __u32 ir_size; / Size of request,
    pub /: *mut *mut __u32 ir_flags; / Request flags,
// 10*/			/* Request specific fields
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_clustersize {
    pub ic_req: ocfs2_info_request,
    pub ic_clustersize: __u32,
    pub ic_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_blocksize {
    pub ib_req: ocfs2_info_request,
    pub ib_blocksize: __u32,
    pub ib_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_maxslots {
    pub im_req: ocfs2_info_request,
    pub im_max_slots: __u32,
    pub im_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_label {
    pub il_req: ocfs2_info_request,
    pub il_label: [__u8; OCFS2_MAX_VOL_LABEL_LEN],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_uuid {
    pub iu_req: ocfs2_info_request,
    pub 1]: __u8 iu_uuid_str[OCFS2_TEXT_UUID_LEN +,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_fs_features {
    pub if_req: ocfs2_info_request,
    pub if_compat_features: __u32,
    pub if_incompat_features: __u32,
    pub if_ro_compat_features: __u32,
    pub if_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_journal_size {
    pub ij_req: ocfs2_info_request,
    pub ij_journal_size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_freeinode {
    pub ifi_req: ocfs2_info_request,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_local_freeinode {
    pub lfi_total: __u64,
    pub lfi_free: __u64,
    pub ifi_stat: [}; OCFS2_MAX_SLOTS],
    pub /: *mut *mut __u32 ifi_slotnum; / out,
    pub ifi_pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_freefrag {
    pub iff_req: ocfs2_info_request,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_freefrag_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_info_free_chunk_list {
    pub fc_chunks: [__u32; OCFS2_INFO_MAX_HIST],
    pub fc_clusters: [__u32; OCFS2_INFO_MAX_HIST],
    pub ffs_fc_hist: },
    pub ffs_clusters: __u32,
    pub ffs_free_clusters: __u32,
    pub ffs_free_chunks: __u32,
    pub ffs_free_chunks_real: __u32,
    pub /: *mut *mut __u32 ffs_min; / Minimum free chunksize in clusters,
    pub ffs_max: __u32,
    pub ffs_avg: __u32,
    pub ffs_pad: __u32,
    pub iff_ffs: },
    pub /: *mut *mut __u32 iff_chunksize; / chunksize in clusters(in),
    pub iff_pad: __u32,
}

// Codes for ocfs2_info_request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_info_type {
    OCFS2_INFO_CLUSTERSIZE = 1,
    OCFS2_INFO_BLOCKSIZE,
    OCFS2_INFO_MAXSLOTS,
    OCFS2_INFO_LABEL,
    OCFS2_INFO_UUID,
    OCFS2_INFO_FS_FEATURES,
    OCFS2_INFO_JOURNAL_SIZE,
    OCFS2_INFO_FREEINODE,
    OCFS2_INFO_FREEFRAG,
    OCFS2_INFO_NUM_TYPES
}

// Flags for struct ocfs2_info_request
// Filled by the caller

// Filled by ocfs2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_move_extents {
// All values are in bytes
// in
    pub /: *mut *mut __u64 me_start; / Virtual start in the file to move,
    pub /: *mut *mut __u64 me_len; / Length of the extents to be moved,
    pub goal,: *mut *mut __u64 me_goal; / Physical offset of the,
    pub threshold: *mut *mut __u64 me_threshold; / Maximum distance from goal or,
    pub operation:: *mut *mut __u64 me_flags; / Flags for the,
// - auto defragmentation.
// - refcount,xattr cases.
//
// out
    pub /: *mut *mut __u64 me_moved_len; / Moved/defraged length,
    pub /: *mut *mut __u64 me_new_offset; / Resulting physical location,
    pub /: *mut *mut __u32 me_reserved[2]; / Reserved for futhure,
}

//

