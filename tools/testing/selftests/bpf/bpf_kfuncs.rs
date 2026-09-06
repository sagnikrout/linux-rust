//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/bpf_kfuncs.h
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


// Description
// Initializes an skb-type dynptr
// Returns
// Error code
//
// Description
// Initializes an xdp-type dynptr
// Returns
// Error code
//
// Description
// Obtain a read-only pointer to the dynptr's data
// Returns
// Either a direct pointer to the dynptr data or a pointer to the user-provided
// buffer if unable to obtain a direct pointer
//
// Description
// Obtain a read-write pointer to the dynptr's data
// Returns
// Either a direct pointer to the dynptr data or a pointer to the user-provided
// buffer if unable to obtain a direct pointer
//
// Description
// Modify the address of a AF_UNIX sockaddr.
// Returns
// -EINVAL if the address size is too big or, 0 if the sockaddr was successfully modified.
//
// Description
// Allocate and configure a reqsk and link it with a listener and skb.
// Returns
// Error code
//
// Description
// Returns xattr of a dentry
// Returns
// Error code
//
