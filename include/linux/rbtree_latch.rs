//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rbtree_latch.h
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
// Latched RB-trees
//
// Copyright (C) 2015 Intel Corp., Peter Zijlstra <peterz@infradead.org>
//
// Since RB-trees have non-atomic modifications they're not immediately suited
// for RCU/lockless queries. Even though we made RB-tree lookups non-fatal for
// lockless lookups; we cannot guarantee they return a correct result.
//
// The simplest solution is a seqlock + RB-tree, this will allow lockless
// lookups; but has the constraint (inherent to the seqlock) that read sides
// cannot nest in write sides.
//
// If we need to allow unconditional lookups (say as required for NMI context
// usage) we need a more complex setup; this data structure provides this by
// employing the latch technique -- see @write_seqcount_latch_begin -- to
// implement a latched RB-tree which does allow for unconditional lookups by
// virtue of always having (at least) one stable copy of the tree.
//
// However, while we have the guarantee that there is at all times one stable
// copy, this does not guarantee an iteration will not observe modifications.
// What might have been a stable copy at the start of the iteration, need not
// remain so for the duration of the iteration.
//
// Therefore, this does require a lockless RB-tree iteration to be non-fatal;
// see the comment in lib/rbtree.c. Note however that we only require the first
// condition -- not seeing partial stores -- because the latch thing isolates
// us from loops. If we were to interrupt a modification the lookup would be
// pointed at the stable tree and complete while the modification was halted.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct latch_tree_node {
    pub node: [rb_node; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct latch_tree_root {
    pub seq: seqcount_latch_t,
    pub tree: [rb_root; 2],
}

//
// struct latch_tree_ops - operators to define the tree order
// @less: used for insertion; provides the (partial) order between two elements.
// @comp: used for lookups; provides the order between the search key and an element.
//
// The operators are related like:
//
// comp(a->key,b) < 0  := less(a,b)
// comp(a->key,b) > 0  := less(b,a)
// comp(a->key,b) == 0 := !less(a,b) && !less(b,a)
//
// If these operators define a partial order on the elements we make no
// guarantee on which of the elements matching the key is found. See
// latch_tree_find().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct latch_tree_ops {
    pub b): *mut *mut *mut bool (less)(struct latch_tree_node a, struct latch_tree_node,
    pub b): *mut *mut *mut int (comp)(void key, struct latch_tree_node,
}

extern "C" {
    pub fn container_of(_arg: node, latch_tree_node: struct, _arg: node[idx]) -> return;
}
//
// latch_tree_insert() - insert @node into the trees @root
// @node: nodes to insert
// @root: trees to insert @node into
// @ops: operators defining the node order
//
// It inserts @node into @root in an ordered fashion such that we can always
// observe one complete tree. See the comment for write_seqcount_latch_begin().
//
// The inserts use rcu_assign_pointer() to publish the element such that the
// tree structure is stored before we can observe the new @node.
//
// All modifications (latch_tree_insert, latch_tree_remove) are assumed to be
// serialized.
//
// latch_tree_erase() - removes @node from the trees @root
// @node: nodes to remote
// @root: trees to remove @node from
// @ops: operators defining the node order
//
// Removes @node from the trees @root in an ordered fashion such that we can
// always observe one complete tree. See the comment for
// write_seqcount_latch_begin().
//
// It is assumed that @node will observe one RCU quiescent state before being
// reused of freed.
//
// All modifications (latch_tree_insert, latch_tree_remove) are assumed to be
// serialized.
//
// latch_tree_find() - find the node matching @key in the trees @root
// @key: search key
// @root: trees to search for @key
// @ops: operators defining the node order
//
// Does a lockless lookup in the trees @root for the node matching @key.
//
// It is assumed that this is called while holding the appropriate RCU read
// side lock.
//
// If the operators define a partial order on the elements (there are multiple
// elements which have the same key value) it is undefined which of these
// elements will be found. Nor is it possible to iterate the tree to find
// further elements with the same key value.
//
// Returns: a pointer to the node matching @key or NULL.
//
