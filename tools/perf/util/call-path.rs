//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/call-path.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// call-path.h: Manipulate a tree data structure containing function call paths
// Copyright (c) 2014, Intel Corporation.
//

//
// struct call_path - node in list of calls leading to a function call.
// @parent: call path to the parent function call
// @sym: symbol of function called
// @ip: only if sym is null, the ip of the function
// @db_id: id used for db-export
// @in_kernel: whether function is a in the kernel
// @rb_node: node in parent's tree of called functions
// @children: tree of call paths of functions called
//
// In combination with the call_return structure, the call_path structure
// defines a context-sensitive call-graph.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_path {
    pub parent: *mut call_path,
    pub sym: *mut symbol,
    pub ip: u64,
    pub db_id: u64,
    pub in_kernel: bool,
    pub rb_node: rb_node,
    pub children: rb_root,
}

pub const CALL_PATH_BLOCK_SHIFT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_path_block {
    pub cp: [call_path; CALL_PATH_BLOCK_SIZE],
    pub node: list_head,
}

//
// struct call_path_root - root of all call paths.
// @call_path: root call path
// @blocks: list of blocks to store call paths
// @next: next free space
// @sz: number of spaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_path_root {
    pub call_path: call_path,
    pub blocks: list_head,
    pub next: usize,
    pub sz: usize,
}

extern "C" {
    pub fn call_path_root__free(cpr: *mut call_path_root);
}
