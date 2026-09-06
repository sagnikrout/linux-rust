//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/interval_tree_generic.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

//
// Template for implementing interval trees
//
// ITSTRUCT:   struct type of the interval tree nodes
// ITRB:       name of struct rb_node field within ITSTRUCT
// ITTYPE:     type of the interval endpoints
// ITSUBTREE:  name of ITTYPE field within ITSTRUCT holding last-in-subtree
// ITSTART(n): start endpoint of ITSTRUCT node n
// ITLAST(n):  last endpoint of ITSTRUCT node n
// ITSTATIC:   'static' or empty
// ITPREFIX:   prefix to use for the inline tree definitions
//
// Note - before using this, please consider if generic version
// (interval_tree.h) would work for you...
//

// Callbacks for augmented rbtree insert and remove */			      \
// Insert / remove interval nodes from the tree */			      \
// \
// Iterate over intervals intersecting [start;last]			      \
// \
// Note that a node's interval intersects [start;last] iff:		      \
// Cond1: ITSTART(node) <= last					      \
// and									      \
// Cond2: start <= ITLAST(node)					      \
// \
// Loop invariant: start <= node->ITSUBTREE		      \
// (Cond2 is satisfied by one of the subtree nodes)	      \
// \
// Some nodes in left subtree satisfy Cond2.  \
// Iterate to find the leftmost such node N.  \
// If it also satisfies Cond1, that's the     \
// match we are looking for. Otherwise, there \
// is no matching interval as nodes to the    \
// right of N can't satisfy Cond1 either.     \
// \
// Fastpath range intersection/overlap between A: [a0, a1] and	      \
// B: [b0, b1] is given by:					      \
// \
// a0 <= b1 && b0 <= a1					      \
// \
// ... where A holds the lock range and B holds the smallest	      \
// 'start' and largest 'last' in the tree. For the later, we	      \
// rely on the root node, which by augmented interval tree	      \
// property, holds the largest value in its last-in-subtree.	      \
// This allows mitigating some of the tree walk overhead for	      \
// for non-intersecting ranges, maintained and consulted in O(1).     \
// \
// Loop invariants:					      \
// Cond1: ITSTART(node) <= last			      \
// rb == node->ITRB.rb_right				      \
// \
// First, search right subtree if suitable		      \
// \
// Move up the tree until we come from a node's left child */ \
// Check if the node intersects [start;last] */		      \
