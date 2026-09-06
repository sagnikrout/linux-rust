//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfsd/export.h
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
// include/linux/nfsd/export.h
//
// Public declarations for NFS exports. The definitions for the
// syscall interface are in nfsctl.h
//
// Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
//

//
// Important limits for the exports stuff.
//
pub const NFSCLNT_IDMAX: c_int = 1024;
pub const NFSCLNT_ADDRMAX: c_int = 16;
pub const NFSCLNT_KEYMAX: c_int = 32;
//
// Export flags.
//
// Please update the expflags[] array in fs/nfsd/export.c when adding
// a new flag.
//
pub const NFSEXP_READONLY: c_uint = 0x0001;
pub const NFSEXP_INSECURE_PORT: c_uint = 0x0002;
pub const NFSEXP_ROOTSQUASH: c_uint = 0x0004;
pub const NFSEXP_ALLSQUASH: c_uint = 0x0008;
pub const NFSEXP_ASYNC: c_uint = 0x0010;
pub const NFSEXP_GATHERED_WRITES: c_uint = 0x0020;
pub const NFSEXP_NOREADDIRPLUS: c_uint = 0x0040;
pub const NFSEXP_SECURITY_LABEL: c_uint = 0x0080;
pub const NFSEXP_SIGN_FH: c_uint = 0x0100;
pub const NFSEXP_NOHIDE: c_uint = 0x0200;
pub const NFSEXP_NOSUBTREECHECK: c_uint = 0x0400;
pub const NFSEXP_NOAUTHNLM: c_uint = 0x0800		/* Don't authenticate NLM requests - just trust */;
pub const NFSEXP_MSNFS: c_uint = 0x1000	/* do silly things that MS clients expect; no longer supported */;
pub const NFSEXP_FSID: c_uint = 0x2000;
pub const NFSEXP_CROSSMOUNT: c_uint = 0x4000;
pub const NFSEXP_NOACL: c_uint = 0x8000	/* reserved for possible ACL related use */;
//
// The NFSEXP_V4ROOT flag causes the kernel to give access only to NFSv4
// clients, and only to the single directory that is the root of the
// export; further lookup and readdir operations are treated as if every
// subdirectory was a mountpoint, and ignored if they are not themselves
// exported.  This is used by nfsd and mountd to construct the NFSv4
// pseudofilesystem, which provides access only to paths leading to each
// exported filesystem.
//
pub const NFSEXP_V4ROOT: c_uint = 0x10000;
pub const NFSEXP_PNFS: c_uint = 0x20000;
// All flags that we claim to support.  (Note we don't support NOACL.)
pub const NFSEXP_ALLFLAGS: c_uint = 0x3FFFF;
// The flags that may vary depending on security flavor:

//
// Transport layer security policies that are permitted to access
// an export
//
pub const NFSEXP_XPRTSEC_NONE: c_uint = 0x0001;
pub const NFSEXP_XPRTSEC_TLS: c_uint = 0x0002;
pub const NFSEXP_XPRTSEC_MTLS: c_uint = 0x0004;

