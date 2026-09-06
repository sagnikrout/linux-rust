//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/migrate_mode.h
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

// Macro flag: #define MIGRATE_MODE_H_INCLUDED
//
// MIGRATE_ASYNC means never block
// MIGRATE_SYNC_LIGHT in the current implementation means to allow blocking
// on most operations but not ->writepage as the potential stall time
// is too significant
// MIGRATE_SYNC will block when migrating pages
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum migrate_mode {
    MIGRATE_ASYNC,
    MIGRATE_SYNC_LIGHT,
    MIGRATE_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum migrate_reason {
    MR_COMPACTION,
    MR_MEMORY_FAILURE,
    MR_MEMORY_HOTPLUG,
    MR_SYSCALL,		/* also applies to cpusets */
    MR_MEMPOLICY_MBIND,
    MR_NUMA_MISPLACED,
    MR_CONTIG_RANGE,
    MR_LONGTERM_PIN,
    MR_DEMOTION,
    MR_DAMON,
    MR_NEVER,		/* page has never been migrated */
    MR_TYPES
}
