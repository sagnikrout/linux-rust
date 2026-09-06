//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vmxfeatures.h
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
// Defines VMX CPU feature bits
//

//
// Note: If the comment begins with a quoted string, that string is used
// in /proc/cpuinfo instead of the macro name.  Otherwise, this feature bit
// is not displayed in /proc/cpuinfo at all.
//
// Pin-Based VM-Execution Controls, EPT/VPID, APIC and VM-Functions, word 0

// EPT/VPID features, scattered to bits 16-23

// Aggregated APIC features 24-27

// VM-Functions, shifted to bits 28-31

// Primary Processor-Based VM-Execution Controls, word 1

// Secondary Processor-Based VM-Execution Controls, word 2

// Tertiary Processor-Based VM-Execution Controls, word 3

