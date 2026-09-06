//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/mmflags.h
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
// The order of these masks is important. Matching masks will be seen
// first and the left over flags will end up showing by themselves.
//
// For example, if we have GFP_KERNEL before GFP_USER we wil get:
//
// GFP_KERNEL|GFP_HARDWALL
//
// Thus most bits set go first.
//
// These define the values that are enums (the bits)

// Just in case these are ever used

//
// For the values that match the bits, use the TRACE_GFP_FLAGS
// which will allow any updates to be included automatically.
//

// Macro flag: #define IF_HAVE_PG_MLOCK(_name)

// Macro flag: #define IF_HAVE_PG_HWPOISON(_name)

// Macro flag: #define IF_HAVE_PG_IDLE(_name)

// Macro flag: #define IF_HAVE_PG_ARCH_2(_name)

// Macro flag: #define IF_HAVE_PG_ARCH_3(_name)

// Macro flag: #define IF_HAVE_VM_SOFTDIRTY(flag,name)

// High-level compaction status feedback
pub const COMPACTION_FAILED: c_int = 1;
pub const COMPACTION_WITHDRAWN: c_int = 2;
pub const COMPACTION_PROGRESS: c_int = 3;

// Macro flag: #define COMPACTION_STATUS
// Macro flag: #define COMPACTION_PRIORITY
// Macro flag: #define COMPACTION_FEEDBACK

// Macro flag: #define IFDEF_ZONE_DMA(X)

// Macro flag: #define IFDEF_ZONE_DMA32(X)

// Macro flag: #define IFDEF_ZONE_HIGHMEM(X)

//
// First define the enums in the above macros to be exported to userspace
// via TRACE_DEFINE_ENUM().
//

// COMPACTION_FEEDBACK are defines not enums. Not needed here.
//
// Now redefine the EM() and EMe() macros to map the enums to the strings
// that will be printed in the output.
//

