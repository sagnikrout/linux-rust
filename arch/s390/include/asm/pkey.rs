//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pkey.h
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
// Kernelspace interface to the pkey device driver
//
// Copyright IBM Corp. 2016, 2023
//
// Author: Harald Freudenberger <freude@de.ibm.com>
//

//
// In-kernel API: Transform an key blob (of any type) into a protected key.
// @param key pointer to a buffer containing the key blob
// @param keylen size of the key blob in bytes
// @param protkey pointer to buffer receiving the protected key
// @param xflags additional execution flags (see PKEY_XFLAG_* definitions below)
// As of now the only supported flags are PKEY_XFLAG_NOMEMALLOC
// and PKEY_XFLAG_NOCLEARKEY.
// @return 0 on success, negative errno value on failure
//
// If this flag is given in the xflags parameter, the pkey implementation
// is not allowed to allocate memory but instead should fall back to use
// preallocated memory or simple fail with -ENOMEM.
// This flag is for protected key derive within a cipher or similar
// which must not allocate memory which would cause io operations - see
// also the CRYPTO_ALG_ALLOCATES_MEMORY flag in crypto.h.
//
pub const PKEY_XFLAG_NOMEMALLOC: c_uint = 0x0001;
//
// Do not accept a clear key token as source for a protected key.
//
pub const PKEY_XFLAG_NOCLEARKEY: c_uint = 0x0002;
//
// Protected key expired due to relocation to another host. The long
// running re-wrap has no asynchronous completion notification, so
// polling is required. Trigger a re-schedule of this request by
// returning -ENOSPC ("hardware queue full") to the crypto engine.
// To avoid immediately re-invocation of this callback,
// tell the scheduler to voluntarily give up the CPU here.
//
