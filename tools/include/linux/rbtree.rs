//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/rbtree.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
    pub __attribute__((aligned(sizeof(long)))): },
// The alignment might seem pointless, but allegedly CRIS needs it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

// 'empty' nodes are nodes that are known not to be inserted in an rbtree

extern "C" {
    pub fn rb_insert_color(: *mut rb_node, : *mut rb_root);
}
extern "C" {
    pub fn rb_erase(: *mut rb_node, : *mut rb_root);
}
// Find logical next and previous nodes in a tree
// Postorder iteration - always visit the parent after its children
// Fast replacement of a single node without remove/rebalance/add/rebalance
// rb_link = node;

//
// rbtree_postorder_for_each_entry_safe - iterate in post-order over rb_root of
// given type allowing the backing memory of @pos to be invalidated
//
// @pos:	the 'type *' to use as a loop cursor.
// @n:		another 'type *' to use as temporary storage
// @root:	'rb_root *' of the rbtree.
// @field:	the name of the rb_node field within 'type'.
//
// rbtree_postorder_for_each_entry_safe() provides a similar guarantee as
// list_for_each_entry_safe() and allows the iteration to continue independent
// of changes to @pos by the body of the loop.
//
// Note, however, that it cannot handle other modifications that re-order the
// rbtree it is iterating over. This includes calling rb_erase() on @pos, as
// rb_erase() may rebalance the tree, causing us to miss some nodes.
//

//
// Leftmost-cached rbtrees.
//
// We do not cache the rightmost node based on footprint
// size vs number of potential users that could benefit
// from O(1) rb_last(). Just not worth it, users that want
// this feature can always implement the logic explicitly.
// Furthermore, users that want to cache both pointers may
// find it a bit asymmetric, but that's ok.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

// Same as rb_first(), but O(1)

//
// The below helper functions use 2 operators with 3 different
// calling conventions. The operators are related like:
//
// comp(a->key,b) < 0  := less(a,b)
// comp(a->key,b) > 0  := less(b,a)
// comp(a->key,b) == 0 := !less(a,b) && !less(b,a)
//
// If these operators define a partial order on the elements we make no
// guarantee on which of the elements matching the key is found. See
// rb_find().
//
// The reason for this is to allow the find() interface without requiring an
// on-stack dummy object, which might not be feasible due to object size.
//
// rb_add_cached() - insert @node into the leftmost cached tree @tree
// @node: node to insert
// @tree: leftmost cached tree to insert @node into
// @less: operator defining the (partial) node order
//
// rb_add() - insert @node into @tree
// @node: node to insert
// @tree: tree to insert @node into
// @less: operator defining the (partial) node order
//
// rb_find_add() - find equivalent @node in @tree, or add @node
// @node: node to look-for / insert
// @tree: tree to search / modify
// @cmp: operator defining the node order
//
// Returns the rb_node matching @node, or NULL when no match is found and @node
// is inserted.
//
// rb_find() - find @key in tree @tree
// @key: key to match
// @tree: tree to search
// @cmp: operator defining the node order
//
// Returns the rb_node matching @key or NULL.
//
// rb_find_first() - find the first @key in @tree
// @key: key to match
// @tree: tree to search
// @cmp: operator defining node order
//
// Returns the leftmost node matching @key, or NULL.
//
// rb_next_match() - find the next @key in @tree
// @key: key to match
// @tree: tree to search
// @cmp: operator defining node order
//
// Returns the next node matching @key, or NULL.
//
// rb_for_each() - iterates a subtree matching @key
// @node: iterator
// @key: key to match
// @tree: tree to search
// @cmp: operator defining node order
//

