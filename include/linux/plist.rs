//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/plist.h
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
// Descending-priority-sorted double-linked list
//
// (C) 2002-2003 Intel Corp
// Inaky Perez-Gonzalez <inaky.perez-gonzalez@intel.com>.
//
// 2001-2005 (c) MontaVista Software, Inc.
// Daniel Walker <dwalker@mvista.com>
//
// (C) 2005 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
//
// Simplifications of the original code by
// Oleg Nesterov <oleg@tv-sign.ru>
//
// Based on simple lists (include/linux/list.h).
//
// This is a priority-sorted list of nodes; each node has a
// priority from INT_MIN (highest) to INT_MAX (lowest).
//
// Addition is O(K), removal is O(1), change of priority of a node is
// O(K) and K is the number of RT priority levels used in the system.
// (1 <= K <= 99)
//
// This list is really a list of lists:
//
// - The tier 1 list is the prio_list, different priority nodes.
//
// - The tier 2 list is the node_list, serialized nodes.
//
// Simple ASCII art explanation:
//
// pl:prio_list (only for plist_node)
// nl:node_list
// HEAD|             NODE(S)
// |
// ||------------------------------------|
// ||->|pl|<->|pl|<--------------->|pl|<-|
// |   |10|   |21|   |21|   |21|   |40|   (prio)
// |   |  |   |  |   |  |   |  |   |  |
// |->|nl|<->|nl|<->|nl|<->|nl|<->|nl|<->|nl|<-|
// |-------------------------------------------|
//
// The nodes on the prio_list list are sorted by priority to simplify
// the insertion of new nodes. There are no nodes with duplicate
// priorites on the list.
//
// The nodes on the node_list are ordered by priority and can contain
// entries which have the same priority. Those entries are ordered
// FIFO
//
// Addition means: look for the prio_list node in the prio_list
// for the priority of the node and insert it before the node_list
// entry of the next prio_list node. If it is the first node of
// that priority, add it to the prio_list in the right position and
// insert it into the serialized node_list list
//
// Removal means remove it from the node_list and remove it from
// the prio_list if the node_list list_head is non empty. In case
// of removal from the prio_list it must be checked whether other
// entries of the same priority are on the list or not. If there
// is another entry of the same priority then this entry has to
// replace the removed entry on the prio_list. If the entry which
// is removed is the only entry of this priority then a simple
// remove from both list is sufficient.
//
// INT_MIN is the highest priority, 0 is the medium highest, INT_MAX
// is lowest priority.
//
// No locking is done, up to the caller.
//

//
// PLIST_HEAD_INIT - static struct plist_head initializer
// @head:	struct plist_head variable name
//

//
// PLIST_HEAD - declare and init plist_head
// @head:	name for struct plist_head variable
//

//
// PLIST_NODE_INIT - static struct plist_node initializer
// @node:	struct plist_node variable name
// @__prio:	initial node priority
//

//
// plist_head_init - dynamic struct plist_head initializer
// @head:	&struct plist_head pointer
//
// plist_node_init - Dynamic struct plist_node initializer
// @node:	&struct plist_node pointer
// @prio:	initial node priority
//
extern "C" {
    pub fn plist_add(node: *mut plist_node, head: *mut plist_head);
}
extern "C" {
    pub fn plist_del(node: *mut plist_node, head: *mut plist_head);
}
extern "C" {
    pub fn plist_requeue(node: *mut plist_node, head: *mut plist_head);
}
//
// plist_for_each - iterate over the plist
// @pos:	the type * to use as a loop counter
// @head:	the head for your list
//

//
// plist_for_each_continue - continue iteration over the plist
// @pos:	the type * to use as a loop cursor
// @head:	the head for your list
//
// Continue to iterate over plist, continuing after the current position.
//

//
// plist_for_each_safe - iterate safely over a plist of given type
// @pos:	the type * to use as a loop counter
// @n:	another type * to use as temporary storage
// @head:	the head for your list
//
// Iterate over a plist of given type, safe against removal of list entry.
//

//
// plist_for_each_entry	- iterate over list of given type
// @pos:	the type * to use as a loop counter
// @head:	the head for your list
// @mem:	the name of the list_head within the struct
//

//
// plist_for_each_entry_continue - continue iteration over list of given type
// @pos:	the type * to use as a loop cursor
// @head:	the head for your list
// @m:		the name of the list_head within the struct
//
// Continue to iterate over list of given type, continuing after
// the current position.
//

//
// plist_for_each_entry_safe - iterate safely over list of given type
// @pos:	the type * to use as a loop counter
// @n:		another type * to use as temporary storage
// @head:	the head for your list
// @m:		the name of the list_head within the struct
//
// Iterate over list of given type, safe against removal of list entry.
//

//
// plist_head_empty - return !0 if a plist_head is empty
// @head:	&struct plist_head pointer
//
extern "C" {
    pub fn list_empty(_arg: &head->node_list) -> return;
}
//
// plist_node_empty - return !0 if plist_node is not on a list
// @node:	&struct plist_node pointer
//
extern "C" {
    pub fn list_empty(_arg: &node->node_list) -> return;
}
// All functions below assume the plist_head is not empty.
//
// plist_first_entry - get the struct for the first entry
// @head:	the &struct plist_head pointer
// @type:	the type of the struct this is embedded in
// @member:	the name of the list_head within the struct
//

//
// plist_last_entry - get the struct for the last entry
// @head:	the &struct plist_head pointer
// @type:	the type of the struct this is embedded in
// @member:	the name of the list_head within the struct
//

//
// plist_next - get the next entry in list
// @pos:	the type * to cursor
//

//
// plist_prev - get the prev entry in list
// @pos:	the type * to cursor
//

//
// plist_first - return the first node (and thus, highest priority)
// @head:	the &struct plist_head pointer
//
// Assumes the plist is _not_ empty.
//
// plist_last - return the last node (and thus, lowest priority)
// @head:	the &struct plist_head pointer
//
// Assumes the plist is _not_ empty.
//
