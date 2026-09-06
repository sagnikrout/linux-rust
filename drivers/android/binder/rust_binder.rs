//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/android/binder/rust_binder.h
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
// Copyright (C) 2025 Google, Inc.
//

//
// These symbols are exposed by `rust_binderfs.c` and exist here so that Rust
// Binder can call them.
//
extern "C" {
    pub fn init_rust_binderfs() -> c_int;
}
extern "C" {
    pub fn rust_binderfs_remove_file(dentry: *mut dentry);
}
//
// The internal data types in the Rust Binder driver are opaque to C, so we use
// void pointer typedefs for these types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_process_layout {
    pub arc_offset: usize,
    pub task: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_transaction_layout {
    pub debug_id: usize,
    pub code: usize,
    pub flags: usize,
    pub from_thread: usize,
    pub to_proc: usize,
    pub target_node: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node_layout {
    pub arc_offset: usize,
    pub debug_id: usize,
    pub ptr: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rust_binder_layout {
    pub t: rb_transaction_layout,
    pub p: rb_process_layout,
    pub n: rb_node_layout,
}

// Nullable!
