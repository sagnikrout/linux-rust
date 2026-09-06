//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/cachefiles.h
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
// CacheFiles tracepoints
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Define enums for tracing information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_obj_ref_trace {
    cachefiles_obj_get_ioreq,
    cachefiles_obj_new,
    cachefiles_obj_put_alloc_fail,
    cachefiles_obj_put_detach,
    cachefiles_obj_put_ioreq,
    cachefiles_obj_see_clean_commit,
    cachefiles_obj_see_clean_delete,
    cachefiles_obj_see_clean_drop_tmp,
    cachefiles_obj_see_lookup_cookie,
    cachefiles_obj_see_lookup_failed,
    cachefiles_obj_see_withdraw_cookie,
    cachefiles_obj_see_withdrawal,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_why_object_killed {
    FSCACHE_OBJECT_IS_STALE,
    FSCACHE_OBJECT_IS_WEIRD,
    FSCACHE_OBJECT_INVALIDATED,
    FSCACHE_OBJECT_NO_SPACE,
    FSCACHE_OBJECT_WAS_RETIRED,
    FSCACHE_OBJECT_WAS_CULLED,
    FSCACHE_VOLUME_IS_WEIRD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_coherency_trace {
    cachefiles_coherency_check_aux,
    cachefiles_coherency_check_content,
    cachefiles_coherency_check_dirty,
    cachefiles_coherency_check_len,
    cachefiles_coherency_check_objsize,
    cachefiles_coherency_check_ok,
    cachefiles_coherency_check_type,
    cachefiles_coherency_check_xattr,
    cachefiles_coherency_set_fail,
    cachefiles_coherency_set_ok,
    cachefiles_coherency_vol_check_cmp,
    cachefiles_coherency_vol_check_ok,
    cachefiles_coherency_vol_check_resv,
    cachefiles_coherency_vol_check_xattr,
    cachefiles_coherency_vol_set_fail,
    cachefiles_coherency_vol_set_ok,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_trunc_trace {
    cachefiles_trunc_dio_adjust,
    cachefiles_trunc_expand_tmpfile,
    cachefiles_trunc_shrink,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_prepare_read_trace {
    cachefiles_trace_read_after_eof,
    cachefiles_trace_read_found_hole,
    cachefiles_trace_read_found_part,
    cachefiles_trace_read_have_data,
    cachefiles_trace_read_no_data,
    cachefiles_trace_read_no_file,
    cachefiles_trace_read_seek_error,
    cachefiles_trace_read_seek_nxio,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_error_trace {
    cachefiles_trace_fallocate_error,
    cachefiles_trace_getxattr_error,
    cachefiles_trace_link_error,
    cachefiles_trace_lookup_error,
    cachefiles_trace_mkdir_error,
    cachefiles_trace_notify_change_error,
    cachefiles_trace_open_error,
    cachefiles_trace_read_error,
    cachefiles_trace_remxattr_error,
    cachefiles_trace_rename_error,
    cachefiles_trace_seek_error,
    cachefiles_trace_setxattr_error,
    cachefiles_trace_statfs_error,
    cachefiles_trace_tmpfile_error,
    cachefiles_trace_trunc_error,
    cachefiles_trace_unlink_error,
    cachefiles_trace_write_error,
}

//
// Define enum -> string mappings for display.
//

//
// Export enum symbols via userspace.
//

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

// Note that obj may be NULL

// This part must be outside protection
