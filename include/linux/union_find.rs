//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/union_find.h
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
// union_find.h - union-find data structure implementation
//
// This header provides functions and structures to implement the union-find
// data structure. The union-find data structure is used to manage disjoint
// sets and supports efficient union and find operations.
//
// See Documentation/core-api/union_find.rst for documentation and samples.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uf_node {
    pub parent: *mut uf_node,
    pub rank: c_uint,
}

// This macro is used for static initialization of a union-find node.

//
// uf_node_init - Initialize a union-find node
// @node: pointer to the union-find node to be initialized
//
// This function sets the parent of the node to itself and
// initializes its rank to 0.
//
// find the root of a node
// Merge two intersecting nodes
extern "C" {
    pub fn uf_union(node1: *mut uf_node, node2: *mut uf_node);
}
