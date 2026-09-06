//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/asm-generic/hugetlb_encode.h
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
// Several system calls take a flag to request "hugetlb" huge pages.
// Without further specification, these system calls will use the
// system's default huge page size.  If a system supports multiple
// huge page sizes, the desired huge page size can be specified in
// bits [26:31] of the flag arguments.  The value in these 6 bits
// will encode the log2 of the huge page size.
//
// The following definitions are associated with this huge page size
// encoding in flag arguments.  System call specific header files
// that use this encoding should include this file.  They can then
// provide definitions based on these with their own specific prefix.
// for example:
// #define MAP_HUGE_SHIFT HUGETLB_FLAG_ENCODE_SHIFT
//
pub const HUGETLB_FLAG_ENCODE_SHIFT: c_int = 26;
pub const HUGETLB_FLAG_ENCODE_MASK: c_uint = 0x3f;

