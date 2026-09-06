//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/list.h
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
// Simple doubly linked list implementation.
//
// Some of the internal functions ("__xxx") are useful when
// manipulating whole lists rather than single entries, as
// sometimes we already know the next/prev entries and we can
// generate better code by using them directly rather than
// using the generic single-entry routines.
//

//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//

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

extern "C" {
    pub fn __list_del_entry(entry: *mut list_head);
}
extern "C" {
    pub fn list_del(entry: *mut list_head);
}

//
// list_replace - replace old entry by new one
// @old : the element to be replaced
// @new : the new element to insert
//
// If @old was empty, it will be overwritten.
//
// list_del_init - deletes entry from list and reinitialize it.
// @entry: the element to delete from the list.
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
// list_empty - tests whether a list is empty
// @head: the list to test.
//
// list_empty_careful - tests whether a list is empty and not being modified
// @head: the list to test
//
// Description:
// tests whether a list is empty _and_ checks that no other CPU might be
// in the process of modifying either member (next or prev)
//
// NOTE: using list_empty_careful() without synchronization
// can only be safe if the only activity that can happen
// to the list entry is list_del_init(). Eg. it cannot be used
// if another CPU could re-list_add() it.
//
// list_rotate_left - rotate the list to the left
// @head: the head of the list
//
// list_is_singular - tests whether a list has just one entry.
// @head: the list to test.
//
// list_cut_position - cut a list into two
// @list: a new list to add all removed entries
// @head: a list with entries
// @entry: an entry within head, could be the head itself
// and if so we won't cut the list
//
// This helper moves the initial part of @head, up to and
// including @entry, from @head to @list. You should
// pass on @entry an element you know is on @head. @list
// should be an empty list or a list you do not care about
// losing its data.
//
// list_splice - join two lists, this is designed for stacks
// @list: the new list to add.
// @head: the place to add it in the first list.
//
// list_splice_tail - join two lists, each list being a queue
// @list: the new list to add.
// @head: the place to add it in the first list.
//
// list_splice_init - join two lists and reinitialise the emptied list.
// @list: the new list to add.
// @head: the place to add it in the first list.
//
// The list at @list is reinitialised
//
// list_splice_tail_init - join two lists and reinitialise the emptied list
// @list: the new list to add.
// @head: the place to add it in the first list.
//
// Each of the lists is a queue.
// The list at @list is reinitialised
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
// list_first_entry_or_null - get the first element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//
// Note that if the list is empty, it returns NULL.
//

//
// list_last_entry_or_null - get the last element from a list
// @ptr:       the list head to take the element from.
// @type:      the type of the struct this is embedded in.
// @member:    the name of the list_head within the struct.
//
// Note that if the list is empty, it returns NULL.
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
// list_for_each	-	iterate over a list
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//

//
// list_for_each_prev	-	iterate over a list backwards
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//

//
// list_for_each_safe - iterate over a list safe against removal of list entry
// @pos:	the &struct list_head to use as a loop cursor.
// @n:		another &struct list_head to use as temporary storage
// @head:	the head for your list.
//

//
// list_for_each_prev_safe - iterate over a list backwards safe against removal of list entry
// @pos:	the &struct list_head to use as a loop cursor.
// @n:		another &struct list_head to use as temporary storage
// @head:	the head for your list.
//

//
// list_for_each_entry	-	iterate over list of given type
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
// list_prepare_entry - prepare a pos entry for use in list_for_each_entry_continue()
// @pos:	the type * to use as a start point
// @head:	the head of the list
// @member:	the name of the list_head within the struct.
//
// Prepares a pos entry for use as a start point in list_for_each_entry_continue().
//

//
// list_for_each_entry_continue - continue iteration over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Continue to iterate over list of given type, continuing after
// the current position.
//

//
// list_for_each_entry_continue_reverse - iterate backwards from the given point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Start to iterate over list of given type backwards, continuing after
// the current position.
//

//
// list_for_each_entry_from - iterate over list of given type from the current point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type, continuing from current position.
//

//
// list_for_each_entry_safe - iterate over list of given type safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_for_each_entry_safe_continue - continue list iteration safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type, continuing after current point,
// safe against removal of list entry.
//

//
// list_for_each_entry_safe_from - iterate over list from current point safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type from current point, safe against
// removal of list entry.
//

//
// list_for_each_entry_safe_reverse - iterate backwards over list safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate backwards over list of given type, safe against removal
// of list entry.
//

//
// list_safe_reset_next - reset a stale list_for_each_entry_safe loop
// @pos:	the loop cursor used in the list_for_each_entry_safe loop
// @n:		temporary storage used in list_for_each_entry_safe
// @member:	the name of the list_head within the struct.
//
// list_safe_reset_next is not safe to use in general if the list may be
// modified concurrently (eg. the lock is dropped in the loop body). An
// exception to this is if the cursor element (pos) is pinned in the list,
// and list_safe_reset_next is called after re-taking the lock and before
// completing the current iteration of the loop body.
//

//
// Double linked lists with a single pointer list head.
// Mostly useful for hash tables where the two pointer list head is
// too wasteful.
// You lose the ability to access the tail in O(1).
//

// next must be != NULL
// (n->pprev) = n;
// after that we'll appear to be on some hlist and hlist_del will work
//
// Move a list from one list head to another. Fixup the pprev
// reference of the first entry if it exists.
//

//
// hlist_for_each_entry	- iterate over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_continue - iterate over a hlist continuing after current point
// @pos:	the type * to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_from - iterate over a hlist continuing from current point
// @pos:	the type * to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_safe - iterate over list of given type safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		another &struct hlist_node to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// list_del_range - deletes range of entries from list.
// @begin: first element in the range to delete from the list.
// @end: last element in the range to delete from the list.
// Note: list_empty on the range of entries does not return true after this,
// the entries is in an undefined state.
//
// list_for_each_from	-	iterate over a list from one of its nodes
// @pos:  the &struct list_head to use as a loop cursor, from where to start
// @head: the head for your list.
//

