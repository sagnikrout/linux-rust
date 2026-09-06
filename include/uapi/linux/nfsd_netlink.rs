//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfsd_netlink.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/nfsd.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const NFSD_FAMILY_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd_cache_type {
    NFSD_CACHE_TYPE_SVC_EXPORT = 1,
    NFSD_CACHE_TYPE_EXPKEY = 2,
}

//
// These flags are ordered to match the NFSEXP_* flags in
// include/linux/nfsd/export.h
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd_export_flags {
    NFSD_EXPORT_FLAGS_READONLY = 1,
    NFSD_EXPORT_FLAGS_INSECURE_PORT = 2,
    NFSD_EXPORT_FLAGS_ROOTSQUASH = 4,
    NFSD_EXPORT_FLAGS_ALLSQUASH = 8,
    NFSD_EXPORT_FLAGS_ASYNC = 16,
    NFSD_EXPORT_FLAGS_GATHERED_WRITES = 32,
    NFSD_EXPORT_FLAGS_NOREADDIRPLUS = 64,
    NFSD_EXPORT_FLAGS_SECURITY_LABEL = 128,
    NFSD_EXPORT_FLAGS_SIGN_FH = 256,
    NFSD_EXPORT_FLAGS_NOHIDE = 512,
    NFSD_EXPORT_FLAGS_NOSUBTREECHECK = 1024,
    NFSD_EXPORT_FLAGS_NOAUTHNLM = 2048,
    NFSD_EXPORT_FLAGS_MSNFS = 4096,
    NFSD_EXPORT_FLAGS_FSID = 8192,
    NFSD_EXPORT_FLAGS_CROSSMOUNT = 16384,
    NFSD_EXPORT_FLAGS_NOACL = 32768,
    NFSD_EXPORT_FLAGS_V4ROOT = 65536,
    NFSD_EXPORT_FLAGS_PNFS = 131072,
}

//
// These flags are ordered to match the NFSEXP_XPRTSEC_* flags in
// include/linux/nfsd/export.h
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd_xprtsec_mode {
    NFSD_XPRTSEC_MODE_NONE = 1,
    NFSD_XPRTSEC_MODE_TLS = 2,
    NFSD_XPRTSEC_MODE_MTLS = 4,
}

