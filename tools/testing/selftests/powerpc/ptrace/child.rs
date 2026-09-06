//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/ptrace/child.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Helper functions to sync execution between parent and child processes.
//
// Copyright 2018, Thiago Jung Bauermann, IBM Corporation.
//

//
// Information in a shared memory location for synchronization between child and
// parent.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_sync {
// The parent waits on this semaphore.
    pub sem_parent: sem_t,
// If true, the child should give up as well.
    pub parent_gave_up: bool,
// The child waits on this semaphore.
    pub sem_child: sem_t,
// If true, the parent should give up as well.
    pub child_gave_up: bool,
}

// Wait until the child prods us.
// Unblock the child now.
// Wait until the parent prods us.
// Unblock the parent now.
