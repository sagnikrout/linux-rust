//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ns/nstree_types.h
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

//
// struct ns_tree_root - Root of a namespace tree
// @ns_rb: Red-black tree root for efficient lookups
// @ns_list_head: List head for sequential iteration
//
// Each namespace tree maintains both an rbtree (for O(log n) lookups)
// and a list (for efficient sequential iteration). The list is kept in
// the same sorted order as the rbtree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_tree_root {
    pub ns_rb: rb_root,
    pub ns_list_head: list_head,
}

//
// struct ns_tree_node - Node in a namespace tree
// @ns_node: Red-black tree node
// @ns_list_entry: List entry for sequential iteration
//
// Represents a namespace's position in a tree. Each namespace has
// multiple tree nodes for different trees (unified, per-type, owner).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_tree_node {
    pub ns_node: rb_node,
    pub ns_list_entry: list_head,
}

//
// struct ns_tree - Namespace tree nodes and active reference count
// @ns_id: Unique namespace identifier
// @__ns_ref_active: Active reference count (do not use directly)
// @ns_unified_node: Node in the global namespace tree
// @ns_tree_node: Node in the per-type namespace tree
// @ns_owner_node: Node in the owner namespace's tree of owned namespaces
// @ns_owner_root: Root of the tree of namespaces owned by this namespace
// (only used when this namespace is an owner)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_tree {
    pub ns_id: u64,
    pub __ns_ref_active: core::sync::atomic::AtomicI32,
    pub ns_unified_node: ns_tree_node,
    pub ns_tree_node: ns_tree_node,
    pub ns_owner_node: ns_tree_node,
    pub ns_owner_root: ns_tree_root,
}
