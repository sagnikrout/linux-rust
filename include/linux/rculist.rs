//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rculist.h
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
// RCU-protected list version
//

//
// INIT_LIST_HEAD_RCU - Initialize a list_head visible to RCU readers
// @list: list to be initialized
//
// You should instead use INIT_LIST_HEAD() for normal initialization and
// cleanup tasks, when readers have no access to the list being initialized.
// However, if the list being initialized is visible to readers, you
// need to keep the compiler from being too mischievous.
//
// return the ->next pointer of a list_head in an rcu safe
// way, we must not access it directly
//

//
// Return the ->prev pointer of a list_head in an rcu safe way. Don't
// access it directly.
//
// Any list traversed with list_bidir_prev_rcu() must never use
// list_del_rcu().  Doing so will poison the ->prev pointer that
// list_bidir_prev_rcu() relies on, which will result in segfaults.
// To prevent these segfaults, use list_bidir_del_rcu() instead
// of list_del_rcu().
//

//
// list_for_each_rcu - Iterate over a list in an RCU-safe fashion
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//

//
// list_tail_rcu - returns the prev pointer of the head of the list
// @head: the head of the list
//
// Note: This should only be used with the list header, and even then
// only if list_del() and similar primitives are not also used on the
// list header.
//

//
// Check during list traversal that we are within an RCU reader
//
// Macro flag: #define check_arg_count_one(dummy)

//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// list_add_rcu - add a new entry to rcu-protected list
// @new: new entry to be added
// @head: list head to add it after
//
// Insert a new entry after the specified head.
// This is good for implementing stacks.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as list_add_rcu()
// or list_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// list_for_each_entry_rcu().
//
// list_add_tail_rcu - add a new entry to rcu-protected list
// @new: new entry to be added
// @head: list head to add it before
//
// Insert a new entry before the specified head.
// This is useful for implementing queues.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as list_add_tail_rcu()
// or list_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// list_for_each_entry_rcu().
//
// list_del_rcu - deletes entry from list without re-initialization
// @entry: the element to delete from the list.
//
// Note: list_empty() on entry does not return true after this,
// the entry is in an undefined state. It is useful for RCU based
// lockfree traversal.
//
// In particular, it means that we can not poison the forward
// pointers that may still be used for walking the list.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as list_del_rcu()
// or list_add_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// list_for_each_entry_rcu().
//
// Note that the caller is not permitted to immediately free
// the newly deleted entry.  Instead, either synchronize_rcu()
// or call_rcu() must be used to defer freeing until an RCU
// grace period has elapsed.
//
// list_bidir_del_rcu - deletes entry from list without re-initialization
// @entry: the element to delete from the list.
//
// In contrast to list_del_rcu() doesn't poison the prev pointer thus
// allowing backwards traversal via list_bidir_prev_rcu().
//
// Note: list_empty() on entry does not return true after this because
// the entry is in a special undefined state that permits RCU-based
// lockfree reverse traversal. In particular this means that we can not
// poison the forward and backwards pointers that may still be used for
// walking the list.
//
// The caller must take whatever precautions are necessary (such as
// holding appropriate locks) to avoid racing with another list-mutation
// primitive, such as list_bidir_del_rcu() or list_add_rcu(), running on
// this same list. However, it is perfectly legal to run concurrently
// with the _rcu list-traversal primitives, such as
// list_for_each_entry_rcu().
//
// Note that list_del_rcu() and list_bidir_del_rcu() must not be used on
// the same list.
//
// Note that the caller is not permitted to immediately free
// the newly deleted entry.  Instead, either synchronize_rcu()
// or call_rcu() must be used to defer freeing until an RCU
// grace period has elapsed.
//
// hlist_del_init_rcu - deletes entry from hash list with re-initialization
// @n: the element to delete from the hash list.
//
// Note: list_unhashed() on the node return true after this. It is
// useful for RCU based read lockfree traversal if the writer side
// must know if the list entry is still hashed or already unhashed.
//
// In particular, it means that we can not poison the forward pointers
// that may still be used for walking the hash list and we can only
// zero the pprev pointer so list_unhashed() will return true after
// this.
//
// The caller must take whatever precautions are necessary (such as
// holding appropriate locks) to avoid racing with another
// list-mutation primitive, such as hlist_add_head_rcu() or
// hlist_del_rcu(), running on this same list.  However, it is
// perfectly legal to run concurrently with the _rcu list-traversal
// primitives, such as hlist_for_each_entry_rcu().
//
// list_replace_rcu - replace old entry by new one
// @old : the element to be replaced
// @new : the new element to insert
//
// The @old entry will be replaced with the @new entry atomically from
// the perspective of concurrent readers.  It is the caller's responsibility
// to synchronize with concurrent updaters, if any.
//
// Note: @old should not be empty.
//
// list_splice_rcu - splice a non-RCU list into an RCU-protected list,
// designed for stacks.
// @list:	the non RCU-protected list to splice
// @head:	the place in the existing RCU-protected list to splice
//
// The list pointed to by @head can be RCU-read traversed concurrently with
// this function.
//
// __list_splice_init_rcu - join an RCU-protected list into an existing list.
// @list:	the RCU-protected list to splice
// @prev:	points to the last element of the existing list
// @next:	points to the first element of the existing list
// @sync:	synchronize_rcu, synchronize_rcu_expedited, ...
//
// The list pointed to by @prev and @next can be RCU-read traversed
// concurrently with this function.
//
// Note that this function blocks.
//
// Important note: the caller must take whatever action is necessary to prevent
// any other updates to the existing list.  In principle, it is possible to
// modify the list as soon as sync() begins execution. If this sort of thing
// becomes necessary, an alternative version based on call_rcu() could be
// created.  But only if -really- needed -- there is no shortage of RCU API
// members.
//
// "first" and "last" tracking list, so initialize it.  RCU readers
// have access to this list, so we must use INIT_LIST_HEAD_RCU()
// instead of INIT_LIST_HEAD().
//
// At this point, the list body still points to the source list.
// Wait for any readers to finish using the list before splicing
// the list body into the new list.  Any new readers will see
// an empty list.
//
// Readers are finished with the source list, so perform splice.
// The order is important if the new list is global and accessible
// to concurrent RCU readers.  Note that RCU readers are not
// permitted to traverse the prev pointers without excluding
// this function.
//
// list_splice_init_rcu - splice an RCU-protected list into an existing list,
// designed for stacks.
// @list:	the RCU-protected list to splice
// @head:	the place in the existing list to splice the first list into
// @sync:	synchronize_rcu, synchronize_rcu_expedited, ...
//
// list_splice_tail_init_rcu - splice an RCU-protected list into an existing
// list, designed for queues.
// @list:	the RCU-protected list to splice
// @head:	the place in the existing list to splice the first list into
// @sync:	synchronize_rcu, synchronize_rcu_expedited, ...
//
// list_entry_rcu - get the struct for this entry
// @ptr:        the &struct list_head pointer.
// @type:       the type of the struct this is embedded in.
// @member:     the name of the list_head within the struct.
//
// This primitive may safely run concurrently with the _rcu list-mutation
// primitives such as list_add_rcu() as long as it's guarded by rcu_read_lock().
//

//
// Where are list_empty_rcu() and list_first_entry_rcu()?
//
// They do not exist because they would lead to subtle race conditions:
//
// if (!list_empty_rcu(mylist)) {
// struct foo *bar = list_first_entry_rcu(mylist, struct foo, list_member);
// do_something(bar);
// }
//
// The list might be non-empty when list_empty_rcu() checks it, but it
// might have become empty by the time that list_first_entry_rcu() rereads
// the ->next pointer, which would result in a SEGV.
//
// When not using RCU, it is OK for list_first_entry() to re-read that
// pointer because both functions should be protected by some lock that
// blocks writers.
//
// When using RCU, list_empty() uses READ_ONCE() to fetch the
// RCU-protected ->next pointer and then compares it to the address of the
// list head.  However, it neither dereferences this pointer nor provides
// this pointer to its caller.  Thus, READ_ONCE() suffices (that is,
// rcu_dereference() is not needed), which means that list_empty() can be
// used anywhere you would want to use list_empty_rcu().  Just don't
// expect anything useful to happen if you do a subsequent lockless
// call to list_first_entry_rcu()!!!
//
// See list_first_or_null_rcu for an alternative.
//
// list_first_or_null_rcu - get the first element from a list
// @ptr:        the list head to take the element from.
// @type:       the type of the struct this is embedded in.
// @member:     the name of the list_head within the struct.
//
// Note that if the list is empty, it returns NULL.
//
// This primitive may safely run concurrently with the _rcu list-mutation
// primitives such as list_add_rcu() as long as it's guarded by rcu_read_lock().
//

//
// list_next_or_null_rcu - get the next element from a list
// @head:	the head for the list.
// @ptr:        the list head to take the next element from.
// @type:       the type of the struct this is embedded in.
// @member:     the name of the list_head within the struct.
//
// Note that if the ptr is at the end of the list, NULL is returned.
//
// This primitive may safely run concurrently with the _rcu list-mutation
// primitives such as list_add_rcu() as long as it's guarded by rcu_read_lock().
//

//
// list_for_each_entry_rcu	-	iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
// @cond:	optional lockdep expression if called from non-RCU protection.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as list_add_rcu()
// as long as the traversal is guarded by rcu_read_lock().
//

//
// list_for_each_entry_srcu	-	iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
// @cond:	lockdep expression for the lock required to traverse the list.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as list_add_rcu()
// as long as the traversal is guarded by srcu_read_lock().
// The lockdep expression srcu_read_lock_held() can be passed as the
// cond argument from read side.
//

//
// list_entry_lockless - get the struct for this entry
// @ptr:        the &struct list_head pointer.
// @type:       the type of the struct this is embedded in.
// @member:     the name of the list_head within the struct.
//
// This primitive may safely run concurrently with the _rcu
// list-mutation primitives such as list_add_rcu(), but requires some
// implicit RCU read-side guarding.  One example is running within a special
// exception-time environment where preemption is disabled and where lockdep
// cannot be invoked.  Another example is when items are added to the list,
// but never deleted.
//

//
// list_for_each_entry_lockless - iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_struct within the struct.
//
// This primitive may safely run concurrently with the _rcu
// list-mutation primitives such as list_add_rcu(), but requires some
// implicit RCU read-side guarding.  One example is running within a special
// exception-time environment where preemption is disabled and where lockdep
// cannot be invoked.  Another example is when items are added to the list,
// but never deleted.
//

//
// list_for_each_entry_continue_rcu - continue iteration over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Continue to iterate over list of given type, continuing after
// the current position which must have been in the list when the RCU read
// lock was taken.
// This would typically require either that you obtained the node from a
// previous walk of the list in the same RCU read-side critical section, or
// that you held some sort of non-RCU reference (such as a reference count)
// to keep the node alive *and* in the list.
//
// This iterator is similar to list_for_each_entry_from_rcu() except
// this starts after the given position and that one starts at the given
// position.
//

//
// list_for_each_entry_from_rcu - iterate over a list from current point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_node within the struct.
//
// Iterate over the tail of a list starting from a given position,
// which must have been in the list when the RCU read lock was taken.
// This would typically require either that you obtained the node from a
// previous walk of the list in the same RCU read-side critical section, or
// that you held some sort of non-RCU reference (such as a reference count)
// to keep the node alive *and* in the list.
//
// This iterator is similar to list_for_each_entry_continue_rcu() except
// this starts from the given position and that one starts from the position
// after the given position.
//

//
// hlist_del_rcu - deletes entry from hash list without re-initialization
// @n: the element to delete from the hash list.
//
// Note: list_unhashed() on entry does not return true after this,
// the entry is in an undefined state. It is useful for RCU based
// lockfree traversal.
//
// In particular, it means that we can not poison the forward
// pointers that may still be used for walking the hash list.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_add_head_rcu()
// or hlist_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_for_each_entry().
//
// hlist_replace_rcu - replace old entry by new one
// @old : the element to be replaced
// @new : the new element to insert
//
// The @old entry will be replaced with the @new entry atomically from
// the perspective of concurrent readers.  It is the caller's responsibility
// to synchronize with concurrent updaters, if any.
//
// hlists_swap_heads_rcu - swap the lists the hlist heads point to
// @left:  The hlist head on the left
// @right: The hlist head on the right
//
// The lists start out as [@left  ][node1 ... ] and
// [@right ][node2 ... ]
// The lists end up as    [@left  ][node2 ... ]
// [@right ][node1 ... ]
//
// return the first or the next element in an RCU protected hlist
//

//
// hlist_add_head_rcu
// @n: the element to add to the hash list.
// @h: the list to add to.
//
// Description:
// Adds the specified element to the specified hlist,
// while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_add_head_rcu()
// or hlist_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.  Regardless of the type of CPU, the
// list-traversal primitive must be guarded by rcu_read_lock().
//
// hlist_add_tail_rcu
// @n: the element to add to the hash list.
// @h: the list to add to.
//
// Description:
// Adds the specified element to the specified hlist,
// while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_add_head_rcu()
// or hlist_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.  Regardless of the type of CPU, the
// list-traversal primitive must be guarded by rcu_read_lock().
//
// Note: write side code, so rcu accessors are not needed.
//
// hlist_add_before_rcu
// @n: the new element to add to the hash list.
// @next: the existing element to add the new element before.
//
// Description:
// Adds the specified element to the specified hlist
// before the specified node while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_add_head_rcu()
// or hlist_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.
//
// hlist_add_behind_rcu
// @n: the new element to add to the hash list.
// @prev: the existing element to add the new element after.
//
// Description:
// Adds the specified element to the specified hlist
// after the specified node while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_add_head_rcu()
// or hlist_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.
//

//
// hlist_for_each_entry_rcu - iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
// @cond:	optional lockdep expression if called from non-RCU protection.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as hlist_add_head_rcu()
// as long as the traversal is guarded by rcu_read_lock().
//

//
// hlist_for_each_entry_srcu - iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
// @cond:	lockdep expression for the lock required to traverse the list.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as hlist_add_head_rcu()
// as long as the traversal is guarded by srcu_read_lock().
// The lockdep expression srcu_read_lock_held() can be passed as the
// cond argument from read side.
//

//
// hlist_for_each_entry_rcu_notrace - iterate over rcu list of given type (for tracing)
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as hlist_add_head_rcu()
// as long as the traversal is guarded by rcu_read_lock().
//
// This is the same as hlist_for_each_entry_rcu() except that it does
// not do any RCU debugging or tracing.
//

//
// hlist_for_each_entry_rcu_bh - iterate over rcu list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//
// This list-traversal primitive may safely run concurrently with
// the _rcu list-mutation primitives such as hlist_add_head_rcu()
// as long as the traversal is guarded by rcu_read_lock().
//

//
// hlist_for_each_entry_continue_rcu - iterate over a hlist continuing after current point
// @pos:	the type * to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_continue_rcu_bh - iterate over a hlist continuing after current point
// @pos:	the type * to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_for_each_entry_from_rcu - iterate over a hlist continuing from current point
// @pos:	the type * to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

