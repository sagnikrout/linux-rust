//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/list.h
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
// Circular doubly linked list implementation.
//
// Some of the internal functions ("__xxx") are useful when
// manipulating whole lists rather than single entries, as
// sometimes we already know the next/prev entries and we can
// generate better code by using them directly rather than
// using the generic single-entry routines.
//
// LIST_HEAD_INIT - initialize a &struct list_head's links to point to itself
// @name: name of the list_head
//

//
// LIST_HEAD - definition of a &struct list_head with initialization values
// @name: name of the list_head
//

//
// LIST_HEAD_GUARDED - define a &struct list_head annotated with __guarded_by()
// @name: name of the list_head
// @lock: lock protecting the list
//

//
// INIT_LIST_HEAD - Initialize a list_head structure
// @list: list_head structure to be initialized.
//
// Initializes the list_head to point to itself.  If it is a list header,
// the result is an empty list.
//

//
// Performs the full set of list corruption checks before __list_add().
// On list corruption reports a warning, and returns false.
//
// Performs list corruption checks before __list_add(). Returns false if a
// corruption is detected, true otherwise.
//
// With CONFIG_LIST_HARDENED only, performs minimal list integrity checking
// inline to catch non-faulting corruptions, and only if a corruption is
// detected calls the reporting function __list_add_valid_or_report().
//
// With the hardening version, elide checking if next and prev
// are NULL, since the immediate dereference of them below would
// result in a fault if NULL.
//
// With the reduced set of checks, we can afford to inline the
// checks, which also gives the compiler a chance to elide some
// of them completely if they can be proven at compile-time. If
// one of the pre-conditions does not hold, the slow-path will
// show a report which pre-condition failed.
//
// Performs the full set of list corruption checks before __list_del_entry().
// On list corruption reports a warning, and returns false.
//
extern "C" {
    pub fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool __list_valid_slowpath;
}
//
// Performs list corruption checks before __list_del_entry(). Returns false if a
// corruption is detected, true otherwise.
//
// With CONFIG_LIST_HARDENED only, performs minimal list integrity checking
// inline to catch non-faulting corruptions, and only if a corruption is
// detected calls the reporting function __list_del_entry_valid_or_report().
//
// With the hardening version, elide checking if next and prev
// are NULL, LIST_POISON1 or LIST_POISON2, since the immediate
// dereference of them below would result in a fault.
//

//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// Must be inlined to ensure it can be safely called
// with initdata arguments.
//
// list_add - add a new entry
// @new: new entry to be added
// @head: list head to add it after
//
// Insert a new entry after the specified head.
// This is good for implementing stacks.
//
// Must be inlined to ensure it can be safely called
// with initdata arguments.
//
// list_add_tail - add a new entry
// @new: new entry to be added
// @head: list head to add it before
//
// Insert a new entry before the specified head.
// This is useful for implementing queues.
//
// list_add_tail_release - add a new entry with release barrier
// @new: new entry to be added
// @head: list head to add it before
//
// Insert a new entry before the specified head, using a release barrier to set
// the ->next pointer that points to it.  This is useful for implementing
// queues, in particular one that the elements will be walked through forwards
// locklessly.
//
// Delete a list entry by making the prev/next entries
// point to each other.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// Delete a list entry and clear the 'prev' pointer.
//
// This is a special-purpose list clearing method used in the networking code
// for lists allocated as per-cpu, where we don't want to incur the extra
// WRITE_ONCE() overhead of a regular list_del_init(). The code that uses this
// needs to check the node 'prev' pointer instead of calling list_empty().
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
// list_swap - replace entry1 with entry2 and re-add entry1 at entry2's position
// @entry1: the location to place entry2
// @entry2: the location to place entry1
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
// list_bulk_move_tail - move a subsection of a list to its tail
// @head: the head that will follow our entry
// @first: first entry to move
// @last: last entry to move, can be the same as first
//
// Move all entries between @first and including @last before @head.
// All three entries must belong to the same linked list.
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
// list_del_init_careful - deletes entry from list and reinitialize it.
// @entry: the element to delete from the list.
//
// This is the same as list_del_init(), except designed to be used
// together with list_empty_careful() in a way to guarantee ordering
// of other memory operations.
//
// Any memory operations done before a list_del_init_careful() are
// guaranteed to be visible after a list_empty_careful() test.
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
extern "C" {
    pub fn list_is_head(_arg: next, READ_ONCE(head->prev): head) && (next ==) -> return;
}
//
// list_rotate_left - rotate the list to the left
// @head: the head of the list
//
// list_rotate_to_front() - Rotate list to specific item.
// @list: The desired new front of the list.
// @head: The head of the list.
//
// Rotates list so that @list becomes the new front of the list.
//
// Deletes the list head from the list denoted by @head and
// places it as the tail of @list, this effectively rotates the
// list so that @list is at the front.
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
// list_cut_before - cut a list into two, before given entry
// @list: a new list to add all removed entries
// @head: a list with entries
// @entry: an entry within head, could be the head itself
//
// This helper moves the initial part of @head, up to but
// excluding @entry, from @head to @list.  You should pass
// in @entry an element you know is on @head.  @list should
// be an empty list or a list you do not care about losing
// its data.
// If @entry == @head, all entries on @head are moved to
// @list.
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
// list_first_entry_or_null_acquire - get the first element from a list with barrier
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//
// Note that if the list is empty, it returns NULL.
//

//
// list_last_entry_or_null - get the last element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//
// Note that if the list is empty, it returns NULL.
//

//
// list_next_entry - get the next element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_next_entry_circular - get the next element in list
// @pos:	the type * to cursor.
// @head:	the list head to take the element from.
// @member:	the name of the list_head within the struct.
//
// Wraparound if pos is the last element (return the first element).
// Note, that list is expected to be not empty.
//

//
// list_prev_entry - get the prev element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_prev_entry_circular - get the prev element in list
// @pos:	the type * to cursor.
// @head:	the list head to take the element from.
// @member:	the name of the list_head within the struct.
//
// Wraparound if pos is the first element (return the last element).
// Note, that list is expected to be not empty.
//

//
// list_for_each	-	iterate over a list
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//

//
// list_for_each_continue - continue iteration over a list
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//
// Continue to iterate over a list, continuing after the current position.
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
// list_count_nodes - count nodes in the list
// @head:	the head for your list.
//
// list_entry_is_head - test if the entry points to the head of the list
// @pos:	the type * to cursor
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
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
// list_for_each_entry_from_reverse - iterate backwards over list of given type
// from the current point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate backwards over list of given type, continuing from current position.
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

//
// hlist_unhashed - Has node been removed from list and reinitialized?
// @h: Node to be checked
//
// Not that not all removal functions will leave a node in unhashed
// state.  For example, hlist_nulls_del_init_rcu() does leave the
// node in unhashed state, but hlist_nulls_del() does not.
//
// hlist_unhashed_lockless - Version of hlist_unhashed for lockless use
// @h: Node to be checked
//
// This variant of hlist_unhashed() must be used in lockless contexts
// to avoid potential load-tearing.  The READ_ONCE() is paired with the
// various WRITE_ONCE() in hlist helpers that are defined below.
//
// hlist_empty - Is the specified hlist_head structure an empty hlist?
// @h: Structure to check.
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
// hlist_add_before - add a new entry before the one specified
// @n: new entry to be added
// @next: hlist node to add it before, which must be non-NULL
//
// hlist_add_behind - add a new entry after the one specified
// @n: new entry to be added
// @prev: hlist node to add it after, which must be non-NULL
//
// hlist_add_fake - create a fake hlist consisting of a single headless node
// @n: Node to make a fake list out of
//
// This makes @n appear to be its own predecessor on a headless hlist.
// The point of this is to allow things like hlist_del() to work correctly
// in cases where there is no list.
//
// hlist_fake: Is this node a fake hlist?
// @h: Node to check for being a self-referential fake hlist.
//
// hlist_is_singular_node - is node the only element of the specified hlist?
// @n: Node to check for singularity.
// @h: Header for potentially singular list.
//
// Check whether the node is the only node of the head without
// accessing head, thus avoiding unnecessary cache misses.
//
// hlist_move_list - Move an hlist
// @old: hlist_head for old list.
// @new: hlist_head for new list.
//
// Move a list from one list head to another. Fixup the pprev
// reference of the first entry if it exists.
//
// hlist_splice_init() - move all entries from one list to another
// @from: hlist_head from which entries will be moved
// @last: last entry on the @from list
// @to:   hlist_head to which entries will be moved
//
// @to can be empty, @from must contain at least @last.
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
// @n:		a &struct hlist_node to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_count_nodes - count nodes in the hlist
// @head:	the head for your hlist.
//
