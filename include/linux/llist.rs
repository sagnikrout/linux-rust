//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/llist.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Lock-less NULL terminated single linked list
//
// Cases where locking is not needed:
// If there are multiple producers and multiple consumers, llist_add can be
// used in producers and llist_del_all can be used in consumers simultaneously
// without locking. Also a single consumer can use llist_del_first while
// multiple producers simultaneously use llist_add, without any locking.
//
// Cases where locking is needed:
// If we have multiple consumers with llist_del_first used in one consumer, and
// llist_del_first or llist_del_all used in other consumers, then a lock is
// needed.  This is because llist_del_first depends on list->first->next not
// changing, but without lock protection, there's no way to be sure about that
// if a preemption happens in the middle of the delete operation and on being
// preempted back, the list->first is the same as before causing the cmpxchg in
// llist_del_first to succeed. For example, while a llist_del_first operation
// is in progress in one consumer, then a llist_del_first, llist_add,
// llist_add (or llist_del_all, llist_add, llist_add) sequence in another
// consumer may cause violations.
//
// This can be summarized as follows:
//
// |   add    | del_first |  del_all
// add       |    -     |     -     |     -
// del_first |    -     |     L     |     L
// del_all   |    -     |     -     |     -
//
// Where, a particular row's operation can happen concurrently with a column's
// operation, with "-" being no lock needed, while "L" being lock is needed.
//
// The list entries deleted via llist_del_all can be traversed with
// traversing function such as llist_for_each etc.  But the list
// entries can not be traversed safely before deleted from the list.
// The order of deleted entries is from the newest to the oldest added
// one.  If you want to traverse from the oldest to the newest, you
// must reverse the order by yourself before traversing.
//
// The basic atomic operation of this list is cmpxchg on long.  On
// architectures that don't have NMI-safe cmpxchg implementation, the
// list can NOT be used in NMI handlers.  So code that uses the list in
// an NMI handler should depend on CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG.
//
// Copyright 2010,2011 Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head {
    pub first: *mut llist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_node {
    pub next: *mut llist_node,
}

//
// init_llist_head - initialize lock-less list head
// @list:	the head for your lock-less list
//
// init_llist_node - initialize lock-less list node
// @node:	the node to be initialised
//
// In cases where there is a need to test if a node is on
// a list or not, this initialises the node to clearly
// not be on any list.
//
// llist_on_list - test if a lock-list list node is on a list
// @node:	the node to test
//
// When a node is on a list the ->next pointer will be NULL or
// some other node.  It can never point to itself.  We use that
// in init_llist_node() to record that a node is not on any list,
// and here to test whether it is on any list.
//
// llist_entry - get the struct of this entry
// @ptr:	the &struct llist_node pointer.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the llist_node within the struct.
//

//
// member_address_is_nonnull - check whether the member address is not NULL
// @ptr:	the object pointer (struct type * that contains the llist_node)
// @member:	the name of the llist_node within the struct.
//
// This macro is conceptually the same as
// &ptr->member != NULL
// but it works around the fact that compilers can decide that taking a member
// address is never a NULL pointer.
//
// Real objects that start at a high address and have a member at NULL are
// unlikely to exist, but such pointers may be returned e.g. by the
// container_of() macro.
//

//
// llist_for_each - iterate over some deleted entries of a lock-less list
// @pos:	the &struct llist_node to use as a loop cursor
// @node:	the first entry of deleted list entries
//
// In general, some entries of the lock-less list can be traversed
// safely only after being deleted from list, so start with an entry
// instead of list head.
//
// If being used on entries deleted from lock-less list directly, the
// traverse order is from the newest to the oldest added entry.  If
// you want to traverse from the oldest to the newest, you must
// reverse the order by yourself before traversing.
//

//
// llist_for_each_safe - iterate over some deleted entries of a lock-less list
// safe against removal of list entry
// @pos:	the &struct llist_node to use as a loop cursor
// @n:		another &struct llist_node to use as temporary storage
// @node:	the first entry of deleted list entries
//
// In general, some entries of the lock-less list can be traversed
// safely only after being deleted from list, so start with an entry
// instead of list head.
//
// If being used on entries deleted from lock-less list directly, the
// traverse order is from the newest to the oldest added entry.  If
// you want to traverse from the oldest to the newest, you must
// reverse the order by yourself before traversing.
//

//
// llist_for_each_entry - iterate over some deleted entries of lock-less list of given type
// @pos:	the type * to use as a loop cursor.
// @node:	the fist entry of deleted list entries.
// @member:	the name of the llist_node with the struct.
//
// In general, some entries of the lock-less list can be traversed
// safely only after being removed from list, so start with an entry
// instead of list head.
//
// If being used on entries deleted from lock-less list directly, the
// traverse order is from the newest to the oldest added entry.  If
// you want to traverse from the oldest to the newest, you must
// reverse the order by yourself before traversing.
//

//
// llist_for_each_entry_safe - iterate over some deleted entries of lock-less list of given type
// safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @node:	the first entry of deleted list entries.
// @member:	the name of the llist_node with the struct.
//
// In general, some entries of the lock-less list can be traversed
// safely only after being removed from list, so start with an entry
// instead of list head.
//
// If being used on entries deleted from lock-less list directly, the
// traverse order is from the newest to the oldest added entry.  If
// you want to traverse from the oldest to the newest, you must
// reverse the order by yourself before traversing.
//

//
// llist_empty - tests whether a lock-less list is empty
// @head:	the list to test
//
// Not guaranteed to be accurate or up to date.  Just a quick way to
// test whether the list is empty without deleting something from the
// list.
//
extern "C" {
    pub fn READ_ONCE(_arg: node->next) -> return;
}
//
// llist_add_batch - add several linked entries in batch
// @new_first:	first entry in batch to be added
// @new_last:	last entry in batch to be added
// @head:	the head for your lock-less list
//
// Return whether list is empty before adding.
//
// llist_add - add a new entry
// @new:	new entry to be added
// @head:	the head for your lock-less list
//
// Returns true if the list was empty prior to adding this entry.
//
extern "C" {
    pub fn llist_add_batch(_arg: new, _arg: new, _arg: head) -> return;
}
extern "C" {
    pub fn __llist_add_batch(_arg: new, _arg: new, _arg: head) -> return;
}
//
// llist_del_all - delete all entries from lock-less list
// @head:	the head of lock-less list to delete all entries
//
// If list is empty, return NULL, otherwise, delete all entries and
// return the pointer to the first entry.  The order of entries
// deleted is from the newest to the oldest added one.
//
extern "C" {
    pub fn xchg(_arg: &head->first, _arg: NULL) -> return;
}
//
// llist_del_first_init - delete first entry from lock-list and mark is as being off-list
// @head:	the head of lock-less list to delete from.
//
// This behave the same as llist_del_first() except that llist_init_node() is called
// on the returned node so that llist_on_list() will report false for the node.
//
