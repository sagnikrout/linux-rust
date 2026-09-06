//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/include/list.h
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

// Are two types/vars the same type (ignoring qualifiers)?

//
// container_of - cast a member of a structure out to the containing structure
// @ptr:	the pointer to the member.
// @type:	the type of the container struct this is embedded in.
// @member:	the name of the member within the struct.
//

//
// Circular doubly linked list implementation.
//
// Some of the internal functions ("__xxx") are useful when
// manipulating whole lists rather than single entries, as
// sometimes we already know the next/prev entries and we can
// generate better code by using them directly rather than
// using the generic single-entry routines.
//

//
// INIT_LIST_HEAD - Initialize a list_head structure
// @list: list_head structure to be initialized.
//
// Initializes the list_head to point to itself.  If it is a list header,
// the result is an empty list.
//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// list_add - add a new entry
// @new: new entry to be added
// @head: list head to add it after
//
// Insert a new entry after the specified head.
// This is good for implementing stacks.
//
// list_add_tail - add a new entry
// @new: new entry to be added
// @head: list head to add it before
//
// Insert a new entry before the specified head.
// This is useful for implementing queues.
//
// Delete a list entry by making the prev/next entries
// point to each other.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// list_del - deletes entry from list.
// @entry: the element to delete from the list.
// Note: list_empty() on entry does not return true after this, the entry is
// in an undefined state.
//
// list_replace - replace old entry by new one
// @old : the element to be replaced
// @new : the new element to insert
//
// If @old was empty, it will be overwritten.
//
// list_replace_init - replace old entry by new one and initialize the old one
// @old : the element to be replaced
// @new : the new element to insert
//
// If @old was empty, it will be overwritten.
//
// list_move - delete from one list and add as another's head
// @list: the entry to move
// @head: the head that will precede our entry
//
// list_move_tail - delete from one list and add as another's tail
// @list: the entry to move
// @head: the head that will follow our entry
//
// list_is_first -- tests whether @list is the first entry in list @head
// @list: the entry to test
// @head: the head of the list
//
// list_is_last - tests whether @list is the last entry in list @head
// @list: the entry to test
// @head: the head of the list
//
// list_is_head - tests whether @list is the list @head
// @list: the entry to test
// @head: the head of the list
//
// list_empty - tests whether a list is empty
// @head: the list to test.
//
// list_entry - get the struct for this entry
// @ptr:	the &struct list_head pointer.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//

//
// list_first_entry - get the first element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//
// Note, that list is expected to be not empty.
//

//
// list_last_entry - get the last element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//
// Note, that list is expected to be not empty.
//

//
// list_next_entry - get the next element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_prev_entry - get the prev element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_entry_is_head - test if the entry points to the head of the list
// @pos:	the type * to cursor
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_for_each_entry - iterate over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_for_each_entry_reverse - iterate backwards over list of given type.
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_for_each_entry_safe - iterate over list of given type. Safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// Double linked lists with a single pointer list head.
// Mostly useful for hash tables where the two pointer list head is
// too wasteful.
// You lose the ability to access the tail in O(1).
//

//
// hlist_unhashed - Has node been removed from list and reinitialized?
// @h: Node to be checked
//
// Not that not all removal functions will leave a node in unhashed
// state.  For example, hlist_nulls_del_init_rcu() does leave the
// node in unhashed state, but hlist_nulls_del() does not.
//
// pprev = next;
//
// hlist_del - Delete the specified hlist_node from its list
// @n: Node to delete.
//
// Note that this function leaves the node in hashed state.  Use
// hlist_del_init() or similar instead to unhash @n.
//
// hlist_del_init - Delete the specified hlist_node from its list and initialize
// @n: Node to delete.
//
// Note that this function leaves the node in unhashed state.
//
// hlist_add_head - add a new entry at the beginning of the hlist
// @n: new entry to be added
// @h: hlist head to add it after
//
// Insert a new entry after the specified head.
// This is good for implementing stacks.
//

//
// hlist_for_each_entry	- iterate over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_safe - iterate over list of given type safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		a &struct hlist_node to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

