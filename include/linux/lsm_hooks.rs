//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lsm_hooks.h
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


//
// Linux Security Module interfaces
//
// Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
// Copyright (C) 2001 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
// Copyright (C) 2001 James Morris <jmorris@intercode.com.au>
// Copyright (C) 2001 Silicon Graphics, Inc. (Trust Technology Group)
// Copyright (C) 2015 Intel Corporation.
// Copyright (C) 2015 Casey Schaufler <casey@schaufler-ca.com>
// Copyright (C) 2016 Mellanox Techonologies
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// Due to this file being licensed under the GPL there is controversy over
// whether this permits you to write a module that #includes this file
// without placing your module under the GPL.  Please consult a lawyer for
// advice before doing this.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union security_list_options {

    pub lsm_func_addr: *mut c_void,
}

//
// @key: static call key as defined by STATIC_CALL_KEY
// @trampoline: static call trampoline as defined by STATIC_CALL_TRAMP
// @hl: The security_hook_list as initialized by the owning LSM.
// @active: Enabled when the static call has an LSM hook associated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_static_call {
    pub key: *mut static_call_key,
    pub trampoline: *mut c_void,
    pub hl: *mut security_hook_list,
// this needs to be true or false based on what the key defaults to
    pub active: *mut static_key_false,
    pub __randomize_layout: },
//
// Table of the static calls for each LSM hook.
// Once the LSMs are initialized, their callbacks will be copied to these
// tables such that the calls are filled backwards (from last to first).
// This way, we can jump directly to the first used static call, and execute
// all of them after. This essentially makes the entry point
// dynamic to adapt the number of static calls to the number of callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_static_calls_table {
    pub NAME: [lsm_static_call; MAX_LSM_COUNT],
    pub __randomize_layout: } __packed,
//
// struct lsm_id - Identify a Linux Security Module.
// @name: name of the LSM, must be approved by the LSM maintainers
// @id: LSM ID number from uapi/linux/lsm.h
//
// Contains the information that identifies the LSM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_id {
    pub name: *const c_char,
    pub id: u64,
}

//
// Security module hook list structure.
// For use with generic list macros for common operations.
//
// struct security_hook_list - Contents of a cacheable, mappable object.
// @scalls: The beginning of the array of static calls assigned to this hook.
// @hook: The callback for the hook.
// @lsm: The name of the lsm that owns this hook.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct security_hook_list {
    pub scalls: *mut lsm_static_call,
    pub hook: security_list_options,
    pub lsmid: *const lsm_id,
    pub __randomize_layout: },
//
// Security blob size or offset data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_blob_sizes {
    pub lbs_cred: c_uint,
    pub lbs_file: c_uint,
    pub lbs_backing_file: c_uint,
    pub lbs_ib: c_uint,
    pub lbs_inode: c_uint,
    pub lbs_sock: c_uint,
    pub lbs_superblock: c_uint,
    pub lbs_ipc: c_uint,
    pub lbs_key: c_uint,
    pub lbs_msg_msg: c_uint,
    pub lbs_perf_event: c_uint,
    pub lbs_task: c_uint,
    pub /: *mut *mut unsigned int lbs_xattr_count; / num xattr slots in new_xattrs array,
    pub lbs_tun_dev: c_uint,
    pub lbs_bdev: c_uint,
    pub lbs_bpf_map: c_uint,
    pub lbs_bpf_prog: c_uint,
    pub lbs_bpf_token: c_uint,
}

//
// LSM_RET_VOID is used as the default value in LSM_HOOK definitions for void
// LSM hooks (in include/linux/lsm_hook_defs.h).
//

//
// Initializing a security_hook_list structure takes
// up a lot of space in a source file. This macro takes
// care of the common case and reduces the amount of
// text involved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsm_order {
    LSM_ORDER_FIRST = -1,	/* This is only for capabilities. */
    LSM_ORDER_MUTABLE = 0,
    LSM_ORDER_LAST = 1,	/* This is only for integrity. */
}

//
// struct lsm_info - Define an individual LSM for the LSM framework.
// @id: LSM name/ID info
// @order: ordering with respect to other LSMs, optional
// @flags: descriptive flags, optional
// @blobs: LSM blob sharing, optional
// @enabled: controlled by CONFIG_LSM, optional
// @init: LSM specific initialization routine
// @initcall_pure: LSM callback for initcall_pure() setup, optional
// @initcall_early: LSM callback for early_initcall setup, optional
// @initcall_core: LSM callback for core_initcall() setup, optional
// @initcall_subsys: LSM callback for subsys_initcall() setup, optional
// @initcall_fs: LSM callback for fs_initcall setup, optional
// @initcall_device: LSM callback for device_initcall() setup, optional
// @initcall_late: LSM callback for late_initcall() setup, optional
// @initcall_late_sync: LSM callback for late_initcall_sync() setup, optional
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub order: lsm_order,
    pub flags: c_ulong,
    pub blobs: *mut lsm_blob_sizes,
    pub enabled: *mut c_int,
    pub (*init)(void): *mut c_int,
    pub (*initcall_pure)(void): *mut c_int,
    pub (*initcall_early)(void): *mut c_int,
    pub (*initcall_core)(void): *mut c_int,
    pub (*initcall_subsys)(void): *mut c_int,
    pub (*initcall_fs)(void): *mut c_int,
    pub (*initcall_device)(void): *mut c_int,
    pub (*initcall_late)(void): *mut c_int,
    pub (*initcall_late_sync)(void): *mut c_int,
}

// DO NOT tamper with these variables outside of the LSM framework
//
// lsm_get_xattr_slot - Return the next available slot and increment the index
// @xattrs: array storing LSM-provided xattrs
// @xattr_count: number of already stored xattrs (updated)
//
// Retrieve the first available slot in the @xattrs array to fill with an xattr,
// and increment @xattr_count.
//
// Return: The slot to fill in @xattrs if non-NULL, NULL otherwise.
//
