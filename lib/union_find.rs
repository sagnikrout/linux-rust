//! Automatically rewritten from C to Rust
//! Source: lib/union_find.c
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
// uf_find - Find the root of a node and perform path compression
// @node: the node to find the root of
//
// This function returns the root of the node by following the parent
// pointers. It also performs path compression, making the tree shallower.
//
// Returns the root node of the set containing node.
//
    struct uf_node *uf_find(struct uf_node *node)
    {
    struct uf_node *parent;
    while (node.parent != node) {
    parent = node.parent;
    node.parent = parent.parent;
    node = parent;
    }
    return node;
    }
//
// uf_union - Merge two sets, using union by rank
// @node1: the first node
// @node2: the second node
//
// This function merges the sets containing node1 and node2, by comparing
// the ranks to keep the tree balanced.
//
#[no_mangle]
pub unsafe extern "C" fn uf_union(node1: *mut uf_node, node2: *mut uf_node) {
    void uf_union(struct uf_node *node1, struct uf_node *node2)
    {
    struct uf_node *root1 = uf_find(node1);
    struct uf_node *root2 = uf_find(node2);
    if (root1 == root2)
    return;
    if (root1.rank < root2.rank) {
    root1.parent = root2;
    } else if (root1.rank > root2.rank) {
    root2.parent = root1;
    } else {
    root2.parent = root1;
    root1.rank++;
    }
    }
