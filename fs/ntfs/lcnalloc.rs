//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/lcnalloc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Exports for NTFS kernel cluster (de)allocation.
//
// Copyright (c) 2004-2005 Anton Altaparmakov
//

//
// enum zone_type - Zone identifiers for cluster allocation policy
//
// FIRST_ZONE		For sanity checking.
// MFT_ZONE		Allocate from $MFT zone.
// DATA_ZONE		Allocate from $DATA zone.
// LAST_ZONE		For sanity checking.
//
// ntfs_cluster_free - free clusters on an ntfs volume
// @ni:		ntfs inode whose runlist describes the clusters to free
// @start_vcn:	vcn in the runlist of @ni at which to start freeing clusters
// @count:	number of clusters to free or -1 for all clusters
// @ctx:	active attribute search context if present or NULL if not
//
// Free @count clusters starting at the cluster @start_vcn in the runlist
// described by the ntfs inode @ni.
//
// If @count is -1, all clusters from @start_vcn to the end of the runlist are
// deallocated.  Thus, to completely free all clusters in a runlist, use
// @start_vcn = 0 and @count = -1.
//
// If @ctx is specified, it is an active search context of @ni and its base mft
// record.  This is needed when ntfs_cluster_free() encounters unmapped runlist
// fragments and allows their mapping.  If you do not have the mft record
// mapped, you can specify @ctx as NULL and ntfs_cluster_free() will perform
// the necessary mapping and unmapping.
//
// Note, ntfs_cluster_free() saves the state of @ctx on entry and restores it
// before returning.  Thus, @ctx will be left pointing to the same attribute on
// return as on entry.  However, the actual pointers in @ctx may point to
// different memory locations on return, so you must remember to reset any
// cached pointers from the @ctx, i.e. after the call to ntfs_cluster_free(),
// you will probably want to do:
// m = ctx->mrec;
// a = ctx->attr;
// Assuming you cache ctx->attr in a variable @a of type ATTR_RECORD * and that
// you cache ctx->mrec in a variable @m of type MFT_RECORD *.
//
// Note, ntfs_cluster_free() does not modify the runlist, so you have to remove
// from the runlist or mark sparse the freed runs later.
//
// Return the number of deallocated clusters (not counting sparse ones) on
// success and -errno on error.
//
// WARNING: If @ctx is supplied, regardless of whether success or failure is
// returned, you need to check IS_ERR(@ctx->mrec) and if 'true' the @ctx
// is no longer valid, i.e. you need to either call
// ntfs_attr_reinit_search_ctx() or ntfs_attr_put_search_ctx() on it.
// In that case PTR_ERR(@ctx->mrec) will give you the error code for
// why the mapping of the old inode failed.
//
// Locking: - The runlist described by @ni must be locked for writing on entry
// and is locked on return.  Note the runlist may be modified when
// needed runlist fragments need to be mapped.
// - The volume lcn bitmap must be unlocked on entry and is unlocked
// on return.
// - This function takes the volume lcn bitmap lock for writing and
// modifies the bitmap contents.
// - If @ctx is NULL, the base mft record of @ni must not be mapped on
// entry and it will be left unmapped on return.
// - If @ctx is not NULL, the base mft record must be mapped on entry
// and it will be left mapped on return.
//
extern "C" {
    pub fn __ntfs_cluster_free(_arg: ni, _arg: start_vcn, _arg: count, _arg: ctx, _arg: false) -> return;
}
//
// ntfs_cluster_free_from_rl - free clusters from runlist
// @vol:	mounted ntfs volume on which to free the clusters
// @rl:		runlist describing the clusters to free
//
// Free all the clusters described by the runlist @rl on the volume @vol.  In
// the case of an error being returned, at least some of the clusters were not
// freed.
//
// Return 0 on success and -errno on error.
//
// Locking: - This function takes the volume lcn bitmap lock for writing and
// modifies the bitmap contents.
// - The caller must have locked the runlist @rl for reading or
// writing.
//
