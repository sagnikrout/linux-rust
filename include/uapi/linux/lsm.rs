//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/lsm.h
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
// Linux Security Modules (LSM) - User space API
//
// Copyright (C) 2022 Casey Schaufler <casey@schaufler-ca.com>
// Copyright (C) 2022 Intel Corporation
//

//
// struct lsm_ctx - LSM context information
// @id: the LSM id number, see LSM_ID_XXX
// @flags: LSM specific flags
// @len: length of the lsm_ctx struct, @ctx and any other data or padding
// @ctx_len: the size of @ctx
// @ctx: the LSM context value
//
// The @len field MUST be equal to the size of the lsm_ctx struct
// plus any additional padding and/or data placed after @ctx.
//
// In all cases @ctx_len MUST be equal to the length of @ctx.
// If @ctx is a string value it should be nul terminated with
// @ctx_len equal to `strlen(@ctx) + 1`.  Binary values are
// supported.
//
// The @flags and @ctx fields SHOULD only be interpreted by the
// LSM specified by @id; they MUST be set to zero/0 when not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_ctx {
    pub id: __u64,
    pub flags: __u64,
    pub len: __u64,
    pub ctx_len: __u64,
    pub __counted_by(ctx_len): __u8 ctx[],
}

//
// ID tokens to identify Linux Security Modules (LSMs)
//
// These token values are used to uniquely identify specific LSMs
// in the kernel as well as in the kernel's LSM userspace API.
//
// A value of zero/0 is considered undefined and should not be used
// outside the kernel. Values 1-99 are reserved for potential
// future use.
//
pub const LSM_ID_UNDEF: c_int = 0;
pub const LSM_ID_CAPABILITY: c_int = 100;
pub const LSM_ID_SELINUX: c_int = 101;
pub const LSM_ID_SMACK: c_int = 102;
pub const LSM_ID_TOMOYO: c_int = 103;
pub const LSM_ID_APPARMOR: c_int = 104;
pub const LSM_ID_YAMA: c_int = 105;
pub const LSM_ID_LOADPIN: c_int = 106;
pub const LSM_ID_SAFESETID: c_int = 107;
pub const LSM_ID_LOCKDOWN: c_int = 108;
pub const LSM_ID_BPF: c_int = 109;
pub const LSM_ID_LANDLOCK: c_int = 110;
pub const LSM_ID_IMA: c_int = 111;
pub const LSM_ID_EVM: c_int = 112;
pub const LSM_ID_IPE: c_int = 113;
//
// LSM_ATTR_XXX definitions identify different LSM attributes
// which are used in the kernel's LSM userspace API. Support
// for these attributes vary across the different LSMs. None
// are required.
//
// A value of zero/0 is considered undefined and should not be used
// outside the kernel. Values 1-99 are reserved for potential
// future use.
//
pub const LSM_ATTR_UNDEF: c_int = 0;
pub const LSM_ATTR_CURRENT: c_int = 100;
pub const LSM_ATTR_EXEC: c_int = 101;
pub const LSM_ATTR_FSCREATE: c_int = 102;
pub const LSM_ATTR_KEYCREATE: c_int = 103;
pub const LSM_ATTR_PREV: c_int = 104;
pub const LSM_ATTR_SOCKCREATE: c_int = 105;
//
// LSM_FLAG_XXX definitions identify special handling instructions
// for the API.
//
pub const LSM_FLAG_SINGLE: c_uint = 0x0001;
