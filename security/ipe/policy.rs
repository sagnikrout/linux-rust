//! Automatically rewritten from C Header to Rust Module
//! Source: security/ipe/policy.h
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
// Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_op_type {
    IPE_OP_EXEC = 0,
    IPE_OP_FIRMWARE,
    IPE_OP_KERNEL_MODULE,
    IPE_OP_KEXEC_IMAGE,
    IPE_OP_KEXEC_INITRAMFS,
    IPE_OP_POLICY,
    IPE_OP_X509,
    __IPE_OP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_action_type {
    IPE_ACTION_ALLOW = 0,
    IPE_ACTION_DENY,
    __IPE_ACTION_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_prop_type {
    IPE_PROP_BOOT_VERIFIED_FALSE,
    IPE_PROP_BOOT_VERIFIED_TRUE,
    IPE_PROP_DMV_ROOTHASH,
    IPE_PROP_DMV_SIG_FALSE,
    IPE_PROP_DMV_SIG_TRUE,
    IPE_PROP_FSV_DIGEST,
    IPE_PROP_FSV_SIG_FALSE,
    IPE_PROP_FSV_SIG_TRUE,
    __IPE_PROP_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_prop {
    pub next: list_head,
    pub type: ipe_prop_type,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_rule {
    pub op: ipe_op_type,
    pub action: ipe_action_type,
    pub props: list_head,
    pub next: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_op_table {
    pub rules: list_head,
    pub default_action: ipe_action_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_parsed_policy {
    pub name: *const c_char,
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
    pub version: },
    pub global_default_action: ipe_action_type,
    pub rules: [ipe_op_table; __IPE_OP_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_policy {
    pub pkcs7: *const c_char,
    pub pkcs7len: usize,
    pub text: *const c_char,
    pub textlen: usize,
    pub parsed: *mut ipe_parsed_policy,
    pub policyfs: *mut dentry,
}

extern "C" {
    pub fn ipe_free_policy(pol: *mut ipe_policy);
}
extern "C" {
    pub fn ipe_set_active_pol(p: *const ipe_policy) -> c_int;
}
