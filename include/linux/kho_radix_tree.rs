//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho_radix_tree.h
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
// DOC: Kexec Handover Radix Tree
//
// This is a radix tree implementation for tracking numeric keys across kexec
// transitions. It was developed for the KHO preserved memory map but is
// designed for broader use by any subsystem that needs to track keys.
// Conceptually speaking, the data structure is similar to a set. It tracks the
// presence or absence of numeric keys.
//
// The radix tree is a multi-level tree where leaf nodes are bitmaps
// representing individual keys.
//
// For the KHO preserved memory map, to allow pages of different sizes (orders)
// to be stored efficiently in a single tree, it uses a unique key encoding
// scheme. Each key is an unsigned long that combines a page's physical address
// and its order.
//
// Client code is responsible for allocating the root node of the tree,
// initializing the mutex lock, and managing its lifecycle. It must use the
// tree data structures defined in the KHO ABI,
// `include/linux/kho/abi/kexec_handover.h`.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_radix_tree {
    pub root: *mut kho_radix_node,
    pub /: *mut *mut mutex lock; / protects the tree's structure and root pointer,
}

//
// struct kho_radix_walk_cb - Callbacks for KHO radix tree walk.
// @leaf:      Called on each present key in the radix tree.
// @node:      Called on each node of the radix tree itself. Receives the
// physical address of the page containing the node.
//
// For each callback, a return value of 0 continues the walk and a non-zero
// return value is directly returned to the caller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_radix_walk_cb {
    pub data): *mut *mut int (leaf)(unsigned long key, void,
    pub data): *mut *mut int (node)(phys_addr_t phys, void,
}

extern "C" {
    pub fn kho_radix_add_key(tree: *mut kho_radix_tree, key: c_ulong) -> c_int;
}
extern "C" {
    pub fn kho_radix_del_key(tree: *mut kho_radix_tree, key: c_ulong);
}
extern "C" {
    pub fn kho_radix_init_tree(tree: *mut kho_radix_tree, root: *mut kho_radix_node) -> c_int;
}
extern "C" {
    pub fn kho_radix_destroy_tree(tree: *mut kho_radix_tree);
}

