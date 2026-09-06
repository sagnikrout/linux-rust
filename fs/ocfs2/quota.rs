//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/quota.h
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
// quota.h for OCFS2
//
// On disk quota structures for local and global quota file, in-memory
// structures.
//

// Number of quota types we support
pub const OCFS2_MAXQUOTAS: c_int = 2;
//
// In-memory structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dquot {
    pub /: *mut *mut dquot dq_dquot; / Generic VFS dquot,
    pub /: *mut *mut loff_t dq_local_off; / Offset in the local quota file,
    pub /: *mut *mut u64 dq_local_phys_blk; / Physical block carrying quota structure,
    pub /: *mut *mut *mut ocfs2_quota_chunk dq_chunk; / Chunk dquot is in,
    pub /: *mut *mut unsigned int dq_use_count; / Number of nodes having reference to this entry in global quota file,
    pub /: *mut *mut s64 dq_origspace; / Last globally synced space usage,
    pub /: *mut *mut s64 dq_originodes; / Last globally synced inode usage,
    pub /: *mut *mut llist_node list; / Member of list of dquots to drop,
}

// Description of one chunk to recover in memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_recovery_chunk {
    pub /: *mut *mut list_head rc_list; / List of chunks,
    pub /: *mut *mut int rc_chunk; / Chunk number,
    pub /: *mut *mut *mut unsigned long rc_bitmap; / Bitmap of entries to recover,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_quota_recovery {
    pub /: *mut *mut list_head r_list[OCFS2_MAXQUOTAS]; / List of chunks to recover,
}

// In-memory structure with quota header information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_mem_dqinfo {
    pub /: *mut *mut unsigned int dqi_type; / Quota type this structure describes,
    pub /: *mut *mut *mut unsigned int dqi_flags; / Flags OLQF_,
    pub /: *mut *mut unsigned int dqi_chunks; / Number of chunks in local quota file,
    pub /: *mut *mut unsigned int dqi_blocks; / Number of blocks allocated for local quota file,
    pub /: *mut *mut unsigned int dqi_syncms; / How often should we sync with other nodes,
    pub /: *mut *mut list_head dqi_chunk; / List of chunks,
    pub /: *mut *mut *mut inode dqi_gqinode; / Global quota file inode,
    pub /: *mut *mut ocfs2_lock_res dqi_gqlock; / Lock protecting quota information structure,
    pub /: *mut *mut *mut buffer_head dqi_gqi_bh; / Buffer head with global quota file inode - set only if inode lock is obtained,
    pub /: *mut *mut int dqi_gqi_count; / Number of holders of dqi_gqi_bh,
    pub /: *mut *mut u64 dqi_giblk; / Number of block with global information header,
    pub /: *mut *mut *mut buffer_head dqi_lqi_bh; / Buffer head with local quota file inode,
    pub /: *mut *mut *mut buffer_head dqi_libh; / Buffer with local information header,
    pub /: *mut *mut qtree_mem_dqinfo dqi_gi; / Info about global file,
    pub /: *mut *mut delayed_work dqi_sync_work; / Work for syncing dquots,
    pub recovery: *mut *mut *mut ocfs2_quota_recovery dqi_rec; / Pointer to,
// information, in case we
// enable quotas on file
// needing it
}

extern "C" {
    pub fn container_of(_arg: dquot, ocfs2_dquot: struct, _arg: dq_dquot) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_quota_chunk {
    pub /: *mut *mut list_head qc_chunk; / List of quotafile chunks,
    pub /: *mut *mut int qc_num; / Number of quota chunk,
    pub /: *mut *mut *mut buffer_head qc_headerbh; / Buffer head with chunk header,
}

extern "C" {
    pub fn ocfs2_free_quota_recovery(rec: *mut ocfs2_quota_recovery);
}
extern "C" {
    pub fn ocfs2_global_read_info(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_global_write_info(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn __ocfs2_sync_dquot(dquot: *mut dquot, freeing: c_int) -> c_int;
}
extern "C" {
    pub fn __ocfs2_sync_dquot(_arg: dquot, _arg: 0) -> return;
}
extern "C" {
    pub fn __ocfs2_sync_dquot(_arg: dquot, _arg: 1) -> return;
}
extern "C" {
    pub fn ocfs2_lock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_unlock_global_qf(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int);
}
extern "C" {
    pub fn ocfs2_validate_quota_block(sb: *mut super_block, bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn ocfs2_create_local_dquot(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn ocfs2_local_release_dquot(handle: *mut handle_t, dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn ocfs2_local_write_dquot(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn ocfs2_drop_dquot_refs(work: *mut work_struct);
}
