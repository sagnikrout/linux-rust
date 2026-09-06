//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/errata/abi-6.h
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
// DOC: erratum_2
//
// Erratum 2: Scoped signal handling
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// This fix addresses an issue where signal scoping was overly restrictive,
// preventing sandboxed threads from signaling other threads within the same
// process if they belonged to different domains.  Because threads are not
// security boundaries, user space might assume that all threads within the same
// process can send signals between themselves (see :manpage:`nptl(7)` and
// :manpage:`libpsx(3)`).  Consistent with :manpage:`ptrace(2)` behavior, direct
// interaction between threads of the same process should always be allowed.
// This change ensures that any thread is allowed to send signals to any other
// thread within the same process, regardless of their domain.
//
// Impact:
//
// This problem only manifests when the userspace process is itself using
// :manpage:`libpsx(3)` or an equivalent mechanism to enforce a Landlock policy
// on multiple already-running threads at once.  Programs which enforce a
// Landlock policy at startup time and only then become multithreaded are not
// affected.  Without this fix, signal scoping could break multi-threaded
// applications that expect threads within the same process to freely signal
// each other.
//
