//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nstree.h
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
// Copyright (c) 2025 Christian Brauner <brauner@kernel.org>

extern "C" {
    pub fn ns_tree_node_init(node: *mut ns_tree_node);
}
extern "C" {
    pub fn ns_tree_root_init(root: *mut ns_tree_root);
}
extern "C" {
    pub fn ns_tree_node_empty(node: *const ns_tree_node) -> bool;
}
extern "C" {
    pub fn ns_tree_node_del(node: *mut ns_tree_node, root: *mut ns_tree_root);
}

extern "C" {
    pub fn __ns_tree_gen_id(ns: *mut ns_common, id: u64) -> u64;
}
extern "C" {
    pub fn __ns_tree_add_raw(ns: *mut ns_common, ns_tree: *mut ns_tree_root);
}
extern "C" {
    pub fn __ns_tree_remove(ns: *mut ns_common, ns_tree: *mut ns_tree_root);
}
//
// ns_tree_add_raw - Add a namespace to a namespace
// @__ns: Namespace to add
//
// This function adds a namespace to the appropriate namespace tree
// without assigning a id.
//

//
// ns_tree_add - Add a namespace to a namespace tree
// @__ns: Namespace to add
//
// This function assigns a new id to the namespace and adds it to the
// appropriate namespace tree and list.
//

//
// ns_tree_remove - Remove a namespace from a namespace tree
// @__ns: Namespace to remove
//
// This function removes a namespace from the appropriate namespace
// tree and list.
//

