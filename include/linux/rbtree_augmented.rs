//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rbtree_augmented.h
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
// Please note - only struct rb_augment_callbacks and the prototypes for
// rb_insert_augmented() and rb_erase_augmented() are intended to be public.
// The rest are implementation details you are not expected to depend on.
//
// See Documentation/core-api/rbtree.rst for documentation and samples.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_augment_callbacks {
    pub stop): *mut *mut *mut void (propagate)(struct rb_node node, struct rb_node,
    pub new): *mut *mut *mut void (copy)(struct rb_node old, struct rb_node,
    pub new): *mut *mut *mut void (rotate)(struct rb_node old, struct rb_node,
}

//
// Fixup the rbtree and update the augmented information when rebalancing.
//
// On insertion, the user must update the augmented information on the path
// leading to the inserted node, then call rb_link_node() as usual and
// rb_insert_augmented() instead of the usual rb_insert_color() call.
// If rb_insert_augmented() rebalances the rbtree, it will callback into
// a user provided function to update the augmented information on the
// affected subtrees.
//
// Template for declaring augmented rbtree callbacks (generic case)
//
// RBSTATIC:    'static' or empty
// RBNAME:      name of the rb_augment_callbacks structure
// RBSTRUCT:    struct type of the tree nodes
// RBFIELD:     name of struct rb_node field within RBSTRUCT
// RBAUGMENTED: name of field within RBSTRUCT holding data for subtree
// RBCOMPUTE:   name of function that recomputes the RBAUGMENTED data
//

//
// Template for declaring augmented rbtree callbacks,
// computing RBAUGMENTED scalar as max(RBCOMPUTE(node)) for all subtree nodes.
//
// RBSTATIC:    'static' or empty
// RBNAME:      name of the rb_augment_callbacks structure
// RBSTRUCT:    struct type of the tree nodes
// RBFIELD:     name of struct rb_node field within RBSTRUCT
// RBTYPE:      type of the RBAUGMENTED field
// RBAUGMENTED: name of RBTYPE field within RBSTRUCT holding data for subtree
// RBCOMPUTE:   name of function that returns the per-node RBTYPE scalar
//

pub const RB_RED: c_int = 0;
pub const RB_BLACK: c_int = 1;

//
// Case 1: node to erase has no more than 1 child (easy!)
//
// Note that if there is one child it must be red due to 5)
// and node must be black due to 4). We adjust colors locally
// so as to bypass __rb_erase_color() later on.
//
// Still case 1, but this time the child is node->rb_left
//
// Case 2: node's successor is its right child
//
// (n)          (s)
// / \          / \
// (x) (s)  ->  (x) (c)
// \
// (c)
//
// Case 3: node's successor is leftmost under
// node's right child subtree
//
// (n)          (s)
// / \          / \
// (x) (y)  ->  (x) (y)
// /
// (p)          (p)
// /
// (s)          (c)
// \
// (c)
//
