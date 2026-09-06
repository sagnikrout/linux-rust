//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/kcsan/permissive.h
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
// Special rules for ignoring entire classes of data-racy memory accesses. None
// of the rules here imply that such data races are generally safe!
//
// All rules in this file can be configured via CONFIG_KCSAN_PERMISSIVE. Keep
// them separate from core code to make it easier to audit.
//
// Copyright (C) 2019, Google LLC.
//

//
// Access ignore rules based on address.
//
// Data-racy bitops on current->flags are too common, ignore completely
// for now.
//
// Data race ignore rules based on access type and value change patterns.
//
// Rules here are only for plain read accesses, so that we still report
// data races between plain read-write accesses.
//
// A common pattern is checking/setting just 1 bit in a variable; for
// example:
//
// if (flags & SOME_FLAG) { ... }
//
// and elsewhere flags is updated concurrently:
//
// flags |= SOME_OTHER_FLAG; // just 1 bit
//
// While it is still recommended that such accesses be marked
// appropriately, in many cases these types of data races are so common
// that marking them all is often unrealistic and left to maintainer
// preference.
//
// The assumption in all cases is that with all known compiler
// optimizations (including those that tear accesses), because no more
// than 1 bit changed, the plain accesses are safe despite the presence
// of data races.
//
// The rules here will ignore the data races if we observe no more than
// 1 bit changed.
//
// Of course many operations can effecively change just 1 bit, but the
// general assuption that data races involving 1-bit changes can be
// tolerated still applies.
//
// And in case a true bug is missed, the bug likely manifests as a
// reportable data race elsewhere.
//
// Exception: Report data races where the values look like
// ordinary booleans (one of them was 0 and the 0th bit was
// changed) More often than not, they come with interesting
// memory ordering requirements, so let's report them.
//
