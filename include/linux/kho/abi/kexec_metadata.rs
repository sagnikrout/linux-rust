//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho/abi/kexec_metadata.h
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
// DOC: Kexec Metadata ABI
//
// The "kexec-metadata" subtree stores optional metadata about the kexec chain.
// It is registered via kho_add_subtree(), keeping it independent from the core
// KHO ABI. This allows the metadata format to evolve without affecting other
// KHO consumers.
//
// The metadata is stored as a plain C struct rather than FDT format for
// simplicity and direct field access.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Breno Leitao <leitao@debian.org>
//

pub const KHO_KEXEC_METADATA_VERSION: c_int = 1;
//
// struct kho_kexec_metadata - Kexec metadata passed between kernels
// @version: ABI version of this struct (must be first field)
// @previous_release: Kernel version string that initiated the kexec
// @kexec_count: Number of kexec boots since last cold boot
//
// This structure is preserved across kexec and allows the new kernel to
// identify which kernel it was booted from and how many kexec reboots
// have occurred.
//
// __NEW_UTS_LEN is part of uABI, so it safe to use it in here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_kexec_metadata {
    pub version: u32,
    pub 1]: char previous_release[__NEW_UTS_LEN +,
    pub kexec_count: u32,
    pub __packed: },

