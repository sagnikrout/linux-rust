//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable_32_areas.h
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


//
// Just any arbitrary offset to the start of the vmalloc VM area: the
// current 8MB value just means that there will be a 8MB "hole" after the
// physical memory until the kernel virtual memory starts.  That means that
// any out-of-bounds memory accesses will hopefully be caught.
// The vmalloc() routines leaves a hole of 4kB between each vmalloced
// area for the same reason. ;)
//

pub const LAST_PKMAP: c_int = 512;

pub const LAST_PKMAP: c_int = 1024;

// The +1 is for the readonly IDT page:

