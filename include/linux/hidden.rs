//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hidden.h
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
// When building position independent code with GCC using the -fPIC option,
// (or even the -fPIE one on older versions), it will assume that we are
// building a dynamic object (either a shared library or an executable) that
// may have symbol references that can only be resolved at load time. For a
// variety of reasons (ELF symbol preemption, the CoW footprint of the section
// that is modified by the loader), this results in all references to symbols
// with external linkage to go via entries in the Global Offset Table (GOT),
// which carries absolute addresses which need to be fixed up when the
// executable image is loaded at an offset which is different from its link
// time offset.
//
// Fortunately, there is a way to inform the compiler that such symbol
// references will be satisfied at link time rather than at load time, by
// giving them 'hidden' visibility.
//
