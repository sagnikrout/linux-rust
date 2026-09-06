//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/libarena/include/libarena/rbtree.h
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbnode {
    pub parent: *mut rbnode __arena,
    pub left: *mut rbnode __arena,
    pub right: *mut rbnode __arena,
}

// Used as a linked list or to store KV pairs.
//
// Does the rbtree allocate its own nodes, or do they get
// allocated by the caller?
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rbtree_alloc {
    RB_ALLOC,
    RB_NOALLOC,
}

//
// Specify the behavior of rbtree insertions when the key is
// already present in the tree.
//
// RB_DEFAULT: Default behavior, reject the new insert.
//
// RB_UPDATE: Update the existing value in the rbtree.
// This updates the node itself, not just the value in
// the existing node.
//
// RB_DUPLICATE: Allow nodes with identical keys in the rbtree.
// Finding/popping/removing a key acts on any of the nodes
// with the appropriate key - there is no ordering by time
// of insertion.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rbtree_insert_mode {
    RB_DEFAULT,
    RB_UPDATE,
    RB_DUPLICATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbtree {
    pub root: *mut rbnode __arena,
    pub alloc: rbtree_alloc,
    pub insert: rbtree_insert_mode,
}

extern "C" {
    pub fn rb_destroy(rbtree: *mut rbtree __arena) -> c_int;
}
extern "C" {
    pub fn rb_insert(rbtree: *mut rbtree __arena, key: u64, value: u64) -> c_int;
}
extern "C" {
    pub fn rb_remove(rbtree: *mut rbtree __arena, key: u64) -> c_int;
}
extern "C" {
    pub fn rb_find(rbtree: *mut rbtree __arena, key: u64, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn rb_print(rbtree: *mut rbtree __arena) -> c_int;
}
extern "C" {
    pub fn rb_least(rbtree: *mut rbtree __arena, key: *mut u64, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn rb_pop(rbtree: *mut rbtree __arena, key: *mut u64, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn rb_insert_node(rbtree: *mut rbtree __arena, node: *mut rbnode __arena) -> c_int;
}
extern "C" {
    pub fn rb_remove_node(rbtree: *mut rbtree __arena, node: *mut rbnode __arena) -> c_int;
}
extern "C" {
    pub fn rb_node_free(rbnode: *mut rbnode __arena);
}
extern "C" {
    pub fn rb_integrity_check(rbtree: *mut rbtree __arena) -> c_int;
}
