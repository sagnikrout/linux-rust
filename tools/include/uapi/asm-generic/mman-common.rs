//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/asm-generic/mman-common.h
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
pub const PROT_READ: c_uint = 0x1		/* page can be read */;
pub const PROT_WRITE: c_uint = 0x2		/* page can be written */;
pub const PROT_EXEC: c_uint = 0x4		/* page can be executed */;
pub const PROT_SEM: c_uint = 0x8		/* page may be used for atomic ops */;
// 0x10		   reserved for arch-specific use
// 0x20		   reserved for arch-specific use
pub const PROT_NONE: c_uint = 0x0		/* page can not be accessed */;
pub const PROT_GROWSDOWN: c_uint = 0x01000000	/* mprotect flag: extend change to start of growsdown vma */;
pub const PROT_GROWSUP: c_uint = 0x02000000	/* mprotect flag: extend change to end of growsup vma */;
// 0x01 - 0x03 are defined in linux/mman.h
pub const MAP_TYPE: c_uint = 0x0f		/* Mask for type of mapping */;
pub const MAP_FIXED: c_uint = 0x10		/* Interpret addr exactly */;
pub const MAP_ANONYMOUS: c_uint = 0x20		/* don't use a file */;
// 0x0100 - 0x4000 flags are defined in asm-generic/mman.h
pub const MAP_POPULATE: c_uint = 0x008000	/* populate (prefault) pagetables */;
pub const MAP_NONBLOCK: c_uint = 0x010000	/* do not block on IO */;
pub const MAP_STACK: c_uint = 0x020000	/* give out an address that is best suited for process/thread stacks */;
pub const MAP_HUGETLB: c_uint = 0x040000	/* create a huge page mapping */;
pub const MAP_SYNC: c_uint = 0x080000 /* perform synchronous page faults for the mapping */;
pub const MAP_FIXED_NOREPLACE: c_uint = 0x100000	/* MAP_FIXED which doesn't unmap underlying mapping */;
pub const MAP_UNINITIALIZED: c_uint = 0x4000000	/* For anonymous mmap, memory could be;
// uninitialized
//
// Flags for mlock
//
pub const MLOCK_ONFAULT: c_uint = 0x01		/* Lock pages in range after they are faulted in, do not prefault */;

// common parameters: try to keep these consistent across architectures

// compatibility flags
pub const MAP_FILE: c_int = 0;
pub const PKEY_UNRESTRICTED: c_uint = 0x0;
pub const PKEY_DISABLE_ACCESS: c_uint = 0x1;
pub const PKEY_DISABLE_WRITE: c_uint = 0x2;

