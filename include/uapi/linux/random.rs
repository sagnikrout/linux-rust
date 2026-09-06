//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/random.h
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
// include/linux/random.h
//
// Include file for the random number generator.
//

// ioctl()'s for the random number generator
// Get the entropy count.

// Add to (or subtract from) the entropy count.  (Superuser only.)

// Get the contents of the entropy pool.  (Superuser only.) (Removed in 2.6.9-rc2.)

//
// Write bytes into the entropy pool and add to the entropy count.
// (Superuser only.)
//

// Clear entropy count to 0.  (Superuser only.)

// Clear the entropy pool and associated counters.  (Superuser only.)

// Reseed CRNG.  (Superuser only.)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rand_pool_info {
    pub entropy_count: c_int,
    pub buf_size: c_int,
    pub buf: [__u32; ],
}

//
// Flags for getrandom(2)
//
// GRND_NONBLOCK	Don't block and return EAGAIN instead
// GRND_RANDOM		No effect
// GRND_INSECURE	Return non-cryptographic random bytes
//
pub const GRND_NONBLOCK: c_uint = 0x0001;
pub const GRND_RANDOM: c_uint = 0x0002;
pub const GRND_INSECURE: c_uint = 0x0004;
//
// struct vgetrandom_opaque_params - arguments for allocating memory for vgetrandom
//
// @size_per_opaque_state:	Size of each state that is to be passed to vgetrandom().
// @mmap_prot:			Value of the prot argument in mmap(2).
// @mmap_flags:			Value of the flags argument in mmap(2).
// @reserved:			Reserved for future use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgetrandom_opaque_params {
    pub size_of_opaque_state: __u32,
    pub mmap_prot: __u32,
    pub mmap_flags: __u32,
    pub reserved: [__u32; 13],
}
