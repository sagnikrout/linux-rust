//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/xarray.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// eXtensible Arrays
// Copyright (c) 2017 Microsoft Corporation
// Author: Matthew Wilcox <willy@infradead.org>
//
// See Documentation/core-api/xarray.rst for how to use the XArray.
//

//
// The bottom two bits of the entry determine how the XArray interprets
// the contents:
//
// 00: Pointer entry
// 10: Internal entry
// x1: Value entry or tagged pointer
//
// Attempting to store internal entries in the XArray is a bug.
//
// Most internal entries are pointers to the next node in the tree.
// The following internal entries have a special meaning:
//
// 0-62: Sibling entries
// 256: Retry entry
// 257: Zero entry
//
// Errors are also represented as internal entries, but use the negative
// space (-4094 to -2).  They're never stored in the slots array; only
// returned by the normal API.
//

//
// xa_mk_value() - Create an XArray entry from an integer.
// @v: Value to store in XArray.
//
// Context: Any context.
// Return: An entry suitable for storing in the XArray.
//
// xa_to_value() - Get value stored in an XArray entry.
// @entry: XArray entry.
//
// Context: Any context.
// Return: The value stored in the XArray entry.
//
// xa_is_value() - Determine if an entry is a value.
// @entry: XArray entry.
//
// Context: Any context.
// Return: True if the entry is a value, false if it is a pointer.
//
// xa_tag_pointer() - Create an XArray entry for a tagged pointer.
// @p: Plain pointer.
// @tag: Tag value (0, 1 or 3).
//
// If the user of the XArray prefers, they can tag their pointers instead
// of storing value entries.  Three tags are available (0, 1 and 3).
// These are distinct from the xa_mark_t as they are not replicated up
// through the array and cannot be searched for.
//
// Context: Any context.
// Return: An XArray entry.
//
// xa_untag_pointer() - Turn an XArray entry into a plain pointer.
// @entry: XArray entry.
//
// If you have stored a tagged pointer in the XArray, call this function
// to get the untagged version of the pointer.
//
// Context: Any context.
// Return: A pointer.
//
// xa_pointer_tag() - Get the tag stored in an XArray entry.
// @entry: XArray entry.
//
// If you have stored a tagged pointer in the XArray, call this function
// to get the tag of that pointer.
//
// Context: Any context.
// Return: A tag.
//
// xa_mk_internal() - Create an internal entry.
// @v: Value to turn into an internal entry.
//
// Internal entries are used for a number of purposes.  Entries 0-255 are
// used for sibling entries (only 0-62 are used by the current code).  256
// is used for the retry entry.  257 is used for the reserved / zero entry.
// Negative internal entries are used to represent errnos.  Node pointers
// are also tagged as internal entries in some situations.
//
// Context: Any context.
// Return: An XArray internal entry corresponding to this value.
//
// xa_to_internal() - Extract the value from an internal entry.
// @entry: XArray entry.
//
// Context: Any context.
// Return: The value which was stored in the internal entry.
//
// xa_is_internal() - Is the entry an internal entry?
// @entry: XArray entry.
//
// Context: Any context.
// Return: %true if the entry is an internal entry.
//

//
// xa_is_zero() - Is the entry a zero entry?
// @entry: Entry retrieved from the XArray
//
// The normal API will return NULL as the contents of a slot containing
// a zero entry.  You can only see zero entries by using the advanced API.
//
// Return: %true if the entry is a zero entry.
//
extern "C" {
    pub fn unlikely(XA_ZERO_ENTRY: entry ==) -> return;
}
//
// xa_is_err() - Report whether an XArray operation returned an error
// @entry: Result from calling an XArray function
//
// If an XArray operation cannot complete an operation, it will return
// a special value indicating an error.  This function tells you
// whether an error occurred; xa_err() tells you which error occurred.
//
// Context: Any context.
// Return: %true if the entry indicates an error.
//
// xa_err() - Turn an XArray result into an errno.
// @entry: Result from calling an XArray function.
//
// If an XArray operation cannot complete an operation, it will return
// a special pointer value which encodes an errno.  This function extracts
// the errno from the pointer value, or returns 0 if the pointer does not
// represent an errno.
//
// Context: Any context.
// Return: A negative errno or 0.
//
// xa_to_internal() would not do sign extension.
//
// struct xa_limit - Represents a range of IDs.
// @min: The lowest ID to allocate (inclusive).
// @max: The maximum ID to allocate (inclusive).
//
// This structure is used either directly or via the XA_LIMIT() macro
// to communicate the range of IDs that are valid for allocation.
// Three common ranges are predefined for you:
// * xa_limit_32b	- [0 - UINT_MAX]
// * xa_limit_31b	- [0 - INT_MAX]
// * xa_limit_16b	- [0 - USHRT_MAX]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xa_limit {
    pub max: u32,
    pub min: u32,
}

pub type xa_mark_t = unsigned ;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xa_lock_type {
    XA_LOCK_IRQ = 1,
    XA_LOCK_BH = 2,
}

//
// Values for xa_flags.  The radix tree stores its GFP flags in the xa_flags,
// and we remain compatible with that.
//

// ALLOC is for a normal 0-based alloc.  ALLOC1 is for an 1-based alloc

//
// struct xarray - The anchor of the XArray.
// @xa_lock: Lock that protects the contents of the XArray.
//
// To use the xarray, define it statically or embed it in your data structure.
// It is a very small data structure, so it does not usually make sense to
// allocate it separately and keep a pointer to it in your data structure.
//
// You may use the xa_lock to protect your own data structures as well.
//
// If all of the entries in the array are NULL, @xa_head is a NULL pointer.
// If the only non-NULL entry in the array is at index 0, @xa_head is that
// entry.  If any other entry in the array is non-NULL, @xa_head points
// to an @xa_node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xarray {
    pub xa_lock: spinlock_t,
// private: The rest of the data structure is not to be used directly.
    pub xa_flags: gfp_t,
    pub xa_head: *mut *mut void __rcu,
}

//
// DEFINE_XARRAY_FLAGS() - Define an XArray with custom flags.
// @name: A string that names your XArray.
// @flags: XA_FLAG values.
//
// This is intended for file scope definitions of XArrays.  It declares
// and initialises an empty XArray with the chosen name and flags.  It is
// equivalent to calling xa_init_flags() on the array, but it does the
// initialisation at compiletime instead of runtime.
//

//
// DEFINE_XARRAY() - Define an XArray.
// @name: A string that names your XArray.
//
// This is intended for file scope definitions of XArrays.  It declares
// and initialises an empty XArray with the chosen name.  It is equivalent
// to calling xa_init() on the array, but it does the initialisation at
// compiletime instead of runtime.
//

//
// DEFINE_XARRAY_ALLOC() - Define an XArray which allocates IDs starting at 0.
// @name: A string that names your XArray.
//
// This is intended for file scope definitions of allocating XArrays.
// See also DEFINE_XARRAY().
//

//
// DEFINE_XARRAY_ALLOC1() - Define an XArray which allocates IDs starting at 1.
// @name: A string that names your XArray.
//
// This is intended for file scope definitions of allocating XArrays.
// See also DEFINE_XARRAY().
//

extern "C" {
    pub fn xa_get_mark(: *mut xarray, index: c_ulong, _arg: xa_mark_t) -> bool;
}
extern "C" {
    pub fn xa_set_mark(: *mut xarray, index: c_ulong, _arg: xa_mark_t);
}
extern "C" {
    pub fn xa_clear_mark(: *mut xarray, index: c_ulong, _arg: xa_mark_t);
}
extern "C" {
    pub fn xa_destroy(: *mut xarray);
}
//
// xa_init_flags() - Initialise an empty XArray with flags.
// @xa: XArray.
// @flags: XA_FLAG values.
//
// If you need to initialise an XArray with special flags (eg you need
// to take the lock from interrupt context), use this function instead
// of xa_init().
//
// Context: Any context.
//
// xa_init() - Initialise an empty XArray.
// @xa: XArray.
//
// An empty XArray is full of NULL entries.
//
// Context: Any context.
//
// xa_empty() - Determine if an array has any present entries.
// @xa: XArray.
//
// Context: Any context.
// Return: %true if the array contains only NULL pointers.
//
// xa_marked() - Inquire whether any entry in this array has a mark set
// @xa: Array
// @mark: Mark value
//
// Context: Any context.
// Return: %true if any entry has this mark set.
//
// xa_for_each_range() - Iterate over a portion of an XArray.
// @xa: XArray.
// @index: Index of @entry.
// @entry: Entry retrieved from array.
// @start: First index to retrieve from array.
// @last: Last index to retrieve from array.
//
// During the iteration, @entry will have the value of the entry stored
// in @xa at @index.  You may modify @index during the iteration if you
// want to skip or reprocess indices.  It is safe to modify the array
// during the iteration.  At the end of the iteration, @entry will be set
// to NULL and @index will have a value less than or equal to max.
//
// xa_for_each_range() is O(n.log(n)) while xas_for_each() is O(n).  You have
// to handle your own locking with xas_for_each(), and if you have to unlock
// after each iteration, it will also end up being O(n.log(n)).
// xa_for_each_range() will spin if it hits a retry entry; if you intend to
// see retry entries, you should use the xas_for_each() iterator instead.
// The xas_for_each() iterator will expand into more inline code than
// xa_for_each_range().
//
// Context: Any context.  Takes and releases the RCU lock.
//

//
// xa_for_each_start() - Iterate over a portion of an XArray.
// @xa: XArray.
// @index: Index of @entry.
// @entry: Entry retrieved from array.
// @start: First index to retrieve from array.
//
// During the iteration, @entry will have the value of the entry stored
// in @xa at @index.  You may modify @index during the iteration if you
// want to skip or reprocess indices.  It is safe to modify the array
// during the iteration.  At the end of the iteration, @entry will be set
// to NULL and @index will have a value less than or equal to max.
//
// xa_for_each_start() is O(n.log(n)) while xas_for_each() is O(n).  You have
// to handle your own locking with xas_for_each(), and if you have to unlock
// after each iteration, it will also end up being O(n.log(n)).
// xa_for_each_start() will spin if it hits a retry entry; if you intend to
// see retry entries, you should use the xas_for_each() iterator instead.
// The xas_for_each() iterator will expand into more inline code than
// xa_for_each_start().
//
// Context: Any context.  Takes and releases the RCU lock.
//

//
// xa_for_each() - Iterate over present entries in an XArray.
// @xa: XArray.
// @index: Index of @entry.
// @entry: Entry retrieved from array.
//
// During the iteration, @entry will have the value of the entry stored
// in @xa at @index.  You may modify @index during the iteration if you want
// to skip or reprocess indices.  It is safe to modify the array during the
// iteration.  At the end of the iteration, @entry will be set to NULL and
// @index will have a value less than or equal to max.
//
// xa_for_each() is O(n.log(n)) while xas_for_each() is O(n).  You have
// to handle your own locking with xas_for_each(), and if you have to unlock
// after each iteration, it will also end up being O(n.log(n)).  xa_for_each()
// will spin if it hits a retry entry; if you intend to see retry entries,
// you should use the xas_for_each() iterator instead.  The xas_for_each()
// iterator will expand into more inline code than xa_for_each().
//
// Context: Any context.  Takes and releases the RCU lock.
//

//
// xa_for_each_marked() - Iterate over marked entries in an XArray.
// @xa: XArray.
// @index: Index of @entry.
// @entry: Entry retrieved from array.
// @filter: Selection criterion.
//
// During the iteration, @entry will have the value of the entry stored
// in @xa at @index.  The iteration will skip all entries in the array
// which do not match @filter.  You may modify @index during the iteration
// if you want to skip or reprocess indices.  It is safe to modify the array
// during the iteration.  At the end of the iteration, @entry will be set to
// NULL and @index will have a value less than or equal to max.
//
// xa_for_each_marked() is O(n.log(n)) while xas_for_each_marked() is O(n).
// You have to handle your own locking with xas_for_each(), and if you have
// to unlock after each iteration, it will also end up being O(n.log(n)).
// xa_for_each_marked() will spin if it hits a retry entry; if you intend to
// see retry entries, you should use the xas_for_each_marked() iterator
// instead.  The xas_for_each_marked() iterator will expand into more inline
// code than xa_for_each_marked().
//
// Context: Any context.  Takes and releases the RCU lock.
//

//
// Versions of the normal API which require the caller to hold the
// xa_lock.  If the GFP flags allow it, they will drop the lock to
// allocate memory, then reacquire it afterwards.  These functions
// may also re-enable interrupts if the XArray flags indicate the
// locking should be interrupt safe.
//
extern "C" {
    pub fn __xa_set_mark(: *mut xarray, index: c_ulong, _arg: xa_mark_t);
}
extern "C" {
    pub fn __xa_clear_mark(: *mut xarray, index: c_ulong, _arg: xa_mark_t);
}
//
// xa_store_bh() - Store this entry in the XArray.
// @xa: XArray.
// @index: Index into array.
// @entry: New entry.
// @gfp: Memory allocation flags.
//
// This function is like calling xa_store() except it disables softirqs
// while holding the array lock.
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.
// Return: The old entry at this index or xa_err() if an error happened.
//
// xa_store_irq() - Store this entry in the XArray.
// @xa: XArray.
// @index: Index into array.
// @entry: New entry.
// @gfp: Memory allocation flags.
//
// This function is like calling xa_store() except it disables interrupts
// while holding the array lock.
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.
// Return: The old entry at this index or xa_err() if an error happened.
//
// xa_erase_bh() - Erase this entry from the XArray.
// @xa: XArray.
// @index: Index of entry.
//
// After this function returns, loading from @index will return %NULL.
// If the index is part of a multi-index entry, all indices will be erased
// and none of the entries will be part of a multi-index entry.
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.
// Return: The entry which used to be at this index.
//
// xa_erase_irq() - Erase this entry from the XArray.
// @xa: XArray.
// @index: Index of entry.
//
// After this function returns, loading from @index will return %NULL.
// If the index is part of a multi-index entry, all indices will be erased
// and none of the entries will be part of a multi-index entry.
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.
// Return: The entry which used to be at this index.
//
// xa_cmpxchg() - Conditionally replace an entry in the XArray.
// @xa: XArray.
// @index: Index into array.
// @old: Old value to test against.
// @entry: New value to place in array.
// @gfp: Memory allocation flags.
//
// If the entry at @index is the same as @old, replace it with @entry.
// If the return value is equal to @old, then the exchange was successful.
//
// Context: Any context.  Takes and releases the xa_lock.  May sleep
// if the @gfp flags permit.
// Return: The old value at this index or xa_err() if an error happened.
//
// xa_cmpxchg_bh() - Conditionally replace an entry in the XArray.
// @xa: XArray.
// @index: Index into array.
// @old: Old value to test against.
// @entry: New value to place in array.
// @gfp: Memory allocation flags.
//
// This function is like calling xa_cmpxchg() except it disables softirqs
// while holding the array lock.
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.  May sleep if the @gfp flags permit.
// Return: The old value at this index or xa_err() if an error happened.
//
// xa_cmpxchg_irq() - Conditionally replace an entry in the XArray.
// @xa: XArray.
// @index: Index into array.
// @old: Old value to test against.
// @entry: New value to place in array.
// @gfp: Memory allocation flags.
//
// This function is like calling xa_cmpxchg() except it disables interrupts
// while holding the array lock.
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.  May sleep if the @gfp flags permit.
// Return: The old value at this index or xa_err() if an error happened.
//
// xa_insert() - Store this entry in the XArray unless another entry is
// already present.
// @xa: XArray.
// @index: Index into array.
// @entry: New entry.
// @gfp: Memory allocation flags.
//
// Inserting a NULL entry will store a reserved entry (like xa_reserve())
// if no entry is present.  Inserting will fail if a reserved entry is
// present, even though loading from this index will return NULL.
//
// Context: Any context.  Takes and releases the xa_lock.  May sleep if
// the @gfp flags permit.
// Return: 0 if the store succeeded.  -EBUSY if another entry was present.
// -ENOMEM if memory could not be allocated.
//
// xa_insert_bh() - Store this entry in the XArray unless another entry is
// already present.
// @xa: XArray.
// @index: Index into array.
// @entry: New entry.
// @gfp: Memory allocation flags.
//
// Inserting a NULL entry will store a reserved entry (like xa_reserve())
// if no entry is present.  Inserting will fail if a reserved entry is
// present, even though loading from this index will return NULL.
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.  May sleep if the @gfp flags permit.
// Return: 0 if the store succeeded.  -EBUSY if another entry was present.
// -ENOMEM if memory could not be allocated.
//
// xa_insert_irq() - Store this entry in the XArray unless another entry is
// already present.
// @xa: XArray.
// @index: Index into array.
// @entry: New entry.
// @gfp: Memory allocation flags.
//
// Inserting a NULL entry will store a reserved entry (like xa_reserve())
// if no entry is present.  Inserting will fail if a reserved entry is
// present, even though loading from this index will return NULL.
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.  May sleep if the @gfp flags permit.
// Return: 0 if the store succeeded.  -EBUSY if another entry was present.
// -ENOMEM if memory could not be allocated.
//
// xa_alloc() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Context: Any context.  Takes and releases the xa_lock.  May sleep if
// the @gfp flags permit.
// Return: 0 on success, -ENOMEM if memory could not be allocated or
// -EBUSY if there are no free entries in @limit.
//
// xa_alloc_bh() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.  May sleep if the @gfp flags permit.
// Return: 0 on success, -ENOMEM if memory could not be allocated or
// -EBUSY if there are no free entries in @limit.
//
// xa_alloc_irq() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.  May sleep if the @gfp flags permit.
// Return: 0 on success, -ENOMEM if memory could not be allocated or
// -EBUSY if there are no free entries in @limit.
//
// xa_alloc_cyclic() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of allocated ID.
// @next: Pointer to next ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
// The search for an empty entry will start at @next and will wrap
// around if necessary.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Note that callers interested in whether wrapping has occurred should
// use __xa_alloc_cyclic() instead.
//
// Context: Any context.  Takes and releases the xa_lock.  May sleep if
// the @gfp flags permit.
// Return: 0 if the allocation succeeded, -ENOMEM if memory could not be
// allocated or -EBUSY if there are no free entries in @limit.
//
// xa_alloc_cyclic_bh() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of allocated ID.
// @next: Pointer to next ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
// The search for an empty entry will start at @next and will wrap
// around if necessary.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Note that callers interested in whether wrapping has occurred should
// use __xa_alloc_cyclic() instead.
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.  May sleep if the @gfp flags permit.
// Return: 0 if the allocation succeeded, -ENOMEM if memory could not be
// allocated or -EBUSY if there are no free entries in @limit.
//
// xa_alloc_cyclic_irq() - Find somewhere to store this entry in the XArray.
// @xa: XArray.
// @id: Pointer to ID.
// @entry: New entry.
// @limit: Range of allocated ID.
// @next: Pointer to next ID to allocate.
// @gfp: Memory allocation flags.
//
// Finds an empty entry in @xa between @limit.min and @limit.max,
// stores the index into the @id pointer, then stores the entry at
// that index.  A concurrent lookup will not see an uninitialised @id.
// The search for an empty entry will start at @next and will wrap
// around if necessary.
//
// Must only be operated on an xarray initialized with flag XA_FLAGS_ALLOC set
// in xa_init_flags().
//
// Note that callers interested in whether wrapping has occurred should
// use __xa_alloc_cyclic() instead.
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.  May sleep if the @gfp flags permit.
// Return: 0 if the allocation succeeded, -ENOMEM if memory could not be
// allocated or -EBUSY if there are no free entries in @limit.
//
// xa_reserve() - Reserve this index in the XArray.
// @xa: XArray.
// @index: Index into array.
// @gfp: Memory allocation flags.
//
// Ensures there is somewhere to store an entry at @index in the array.
// If there is already something stored at @index, this function does
// nothing.  If there was nothing there, the entry is marked as reserved.
// Loading from a reserved entry returns a %NULL pointer.
//
// If you do not use the entry that you have reserved, call xa_release()
// or xa_erase() to free any unnecessary memory.
//
// Context: Any context.  Takes and releases the xa_lock.
// May sleep if the @gfp flags permit.
// Return: 0 if the reservation succeeded or -ENOMEM if it failed.
//
extern "C" {
    pub fn xa_err(_arg: xa_cmpxchg(xa, _arg: index, _arg: NULL, _arg: XA_ZERO_ENTRY, _arg: gfp)) -> return;
}
//
// xa_reserve_bh() - Reserve this index in the XArray.
// @xa: XArray.
// @index: Index into array.
// @gfp: Memory allocation flags.
//
// A softirq-disabling version of xa_reserve().
//
// Context: Any context.  Takes and releases the xa_lock while
// disabling softirqs.
// Return: 0 if the reservation succeeded or -ENOMEM if it failed.
//
extern "C" {
    pub fn xa_err(_arg: xa_cmpxchg_bh(xa, _arg: index, _arg: NULL, _arg: XA_ZERO_ENTRY, _arg: gfp)) -> return;
}
//
// xa_reserve_irq() - Reserve this index in the XArray.
// @xa: XArray.
// @index: Index into array.
// @gfp: Memory allocation flags.
//
// An interrupt-disabling version of xa_reserve().
//
// Context: Process context.  Takes and releases the xa_lock while
// disabling interrupts.
// Return: 0 if the reservation succeeded or -ENOMEM if it failed.
//
extern "C" {
    pub fn xa_err(_arg: xa_cmpxchg_irq(xa, _arg: index, _arg: NULL, _arg: XA_ZERO_ENTRY, _arg: gfp)) -> return;
}
//
// xa_release() - Release a reserved entry.
// @xa: XArray.
// @index: Index of entry.
//
// After calling xa_reserve(), you can call this function to release the
// reservation.  If the entry at @index has been stored to, this function
// will do nothing.
//
// Everything below here is the Advanced API.  Proceed with caution.
//
// The xarray is constructed out of a set of 'chunks' of pointers.  Choosing
// the best chunk size requires some tradeoffs.  A power of two recommends
// itself so that we can walk the tree based purely on shifts and masks.
// Generally, the larger the better; as the number of slots per level of the
// tree increases, the less tall the tree needs to be.  But that needs to be
// balanced against the memory consumption of each node.  On a 64-bit system,
// xa_node is currently 576 bytes, and we get 7 of them per 4kB page.  If we
// doubled the number of slots per node, we'd get only 3 nodes per 4kB page.
//

pub const XA_MAX_MARKS: c_int = 3;

//
// @count is the count of every non-NULL element in the ->slots array
// whether that is a value entry, a retry entry, a user pointer,
// a sibling entry or a pointer to the next level of the tree.
// @nr_values is the count of every element in ->slots which is
// either a value entry or a sibling of a value entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xa_node {
    pub /: *mut *mut unsigned char shift; / Bits remaining in each slot,
    pub /: *mut *mut unsigned char offset; / Slot offset in parent,
    pub /: *mut *mut unsigned char count; / Total entry count,
    pub /: *mut *mut unsigned char nr_values; / Value entry count,
    pub /: *mut *mut *mut xa_node __rcu parent; / NULL at top of tree,
    pub /: *mut *mut *mut xarray array; / The array we belong to,
    pub /: *mut *mut list_head private_list; / For tree user,
    pub /: *mut *mut rcu_head rcu_head; / Used when freeing node,
}

extern "C" {
    pub fn xa_dump(: *const xarray);
}
extern "C" {
    pub fn xa_dump_node(: *const xa_node);
}

// Private
extern "C" {
    pub fn xa_mk_internal(_arg: offset) -> return;
}
// Private
extern "C" {
    pub fn xa_to_internal(_arg: entry) -> return;
}
//
// xa_is_sibling() - Is the entry a sibling entry?
// @entry: Entry retrieved from the XArray
//
// Return: %true if the entry is a sibling entry.
//

//
// xa_is_retry() - Is the entry a retry entry?
// @entry: Entry retrieved from the XArray
//
// Return: %true if the entry is a retry entry.
//
extern "C" {
    pub fn unlikely(XA_RETRY_ENTRY: entry ==) -> return;
}
//
// xa_is_advanced() - Is the entry only permitted for the advanced API?
// @entry: Entry to be stored in the XArray.
//
// Return: %true if the entry cannot be stored by the normal API.
//
extern "C" {
    pub fn xa_is_internal(XA_RETRY_ENTRY: entry) && (entry <=) -> return;
}
//
// typedef xa_update_node_t - A callback function from the XArray.
// @node: The node which is being processed
//
// This function is called every time the XArray updates the count of
// present and value entries in a node.  It allows advanced users to
// maintain the private_list in the node.
//
// Context: The xa_lock is held and interrupts may be disabled.
// Implementations should not drop the xa_lock, nor re-enable
// interrupts.
//
extern "C" {
    pub fn void(node: *mut *mut xa_update_node_t)(struct xa_node) -> typedef;
}
extern "C" {
    pub fn xa_delete_node(: *mut xa_node, _arg: xa_update_node_t);
}
//
// The xa_state is opaque to its users.  It contains various different pieces
// of state involved in the current operation on the XArray.  It should be
// declared on the stack and passed between the various internal routines.
// The various elements in it should not be accessed directly, but only
// through the provided accessor functions.  The below documentation is for
// the benefit of those working on the code, not for users of the XArray.
//
// @xa_node usually points to the xa_node containing the slot we're operating
// on (and @xa_offset is the offset in the slots array).  If there is a
// single entry in the array at index 0, there are no allocated xa_nodes to
// point to, and so we store %NULL in @xa_node.  @xa_node is set to
// the value %XAS_RESTART if the xa_state is not walked to the correct
// position in the tree of nodes for this operation.  If an error occurs
// during an operation, it is set to an %XAS_ERROR value.  If we run off the
// end of the allocated nodes, it is set to %XAS_BOUNDS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xa_state {
    pub xa: *mut xarray,
    pub xa_index: c_ulong,
    pub xa_shift: c_uchar,
    pub xa_sibs: c_uchar,
    pub xa_offset: c_uchar,
    pub /: *mut *mut unsigned char xa_pad; / Helps gcc generate better code,
    pub xa_node: *mut xa_node,
    pub xa_alloc: *mut xa_node,
    pub xa_update: xa_update_node_t,
    pub xa_lru: *mut list_lru,
}

//
// We encode errnos in the xas->xa_node.  If an error has happened, we need to
// drop the lock to fix it, and once we've done so the xa_state is invalid.
//

//
// XA_STATE() - Declare an XArray operation state.
// @name: Name of this operation state (usually xas).
// @array: Array to operate on.
// @index: Initial index of interest.
//
// Declare and initialise an xa_state on the stack.
//

//
// XA_STATE_ORDER() - Declare an XArray operation state.
// @name: Name of this operation state (usually xas).
// @array: Array to operate on.
// @index: Initial index of interest.
// @order: Order of entry.
//
// Declare and initialise an xa_state on the stack.  This variant of
// XA_STATE() allows you to specify the 'order' of the element you
// want to operate on.`
//

//
// xas_error() - Return an errno stored in the xa_state.
// @xas: XArray operation state.
//
// Return: 0 if no error has been noted.  A negative errno if one has.
//
extern "C" {
    pub fn xa_err(_arg: xas->xa_node) -> return;
}
//
// xas_set_err() - Note an error in the xa_state.
// @xas: XArray operation state.
// @err: Negative error number.
//
// Only call this function with a negative @err; zero or positive errors
// will probably not behave the way you think they should.  If you want
// to clear the error from an xa_state, use xas_reset().
//
// xas_invalid() - Is the xas in a retry or error state?
// @xas: XArray operation state.
//
// Return: %true if the xas cannot be used for operations.
//
// xas_valid() - Is the xas a valid cursor into the array?
// @xas: XArray operation state.
//
// Return: %true if the xas can be used for operations.
//
// xas_is_node() - Does the xas point to a node?
// @xas: XArray operation state.
//
// Return: %true if the xas currently references a node.
//
// True if the pointer is something other than a node
// True if the node represents RESTART or an error
// True if the node represents head-of-tree, RESTART or BOUNDS
//
// xas_reset() - Reset an XArray operation state.
// @xas: XArray operation state.
//
// Resets the error or walk state of the @xas so future walks of the
// array will start from the root.  Use this if you have dropped the
// xarray lock and want to reuse the xa_state.
//
// Context: Any context.
//
// xas_retry() - Retry the operation if appropriate.
// @xas: XArray operation state.
// @entry: Entry from xarray.
//
// The advanced functions may sometimes return an internal entry, such as
// a retry entry or a zero entry.  This function sets up the @xas to restart
// the walk from the head of the array if needed.
//
// Context: Any context.
// Return: true if the operation needs to be retried.
//
extern "C" {
    pub fn xas_get_mark(: *const xa_state, _arg: xa_mark_t) -> bool;
}
extern "C" {
    pub fn xas_set_mark(: *const xa_state, _arg: xa_mark_t);
}
extern "C" {
    pub fn xas_clear_mark(: *const xa_state, _arg: xa_mark_t);
}
extern "C" {
    pub fn xas_init_marks(: *const xa_state);
}
extern "C" {
    pub fn xas_nomem(: *mut xa_state, _arg: gfp_t) -> bool;
}
extern "C" {
    pub fn xas_destroy(: *mut xa_state);
}
extern "C" {
    pub fn xas_pause(: *mut xa_state);
}
extern "C" {
    pub fn xas_create_range(: *mut xa_state);
}

extern "C" {
    pub fn xa_get_order(: *mut xarray, index: c_ulong) -> c_int;
}
extern "C" {
    pub fn xas_get_order(xas: *mut xa_state) -> c_int;
}
extern "C" {
    pub fn xas_split(: *mut xa_state, entry: *mut c_void, order: c_uint);
}
extern "C" {
    pub fn xas_split_alloc(: *mut xa_state, entry: *mut c_void, order: c_uint, _arg: gfp_t);
}
extern "C" {
    pub fn xas_try_split(xas: *mut xa_state, entry: *mut c_void, order: c_uint);
}
extern "C" {
    pub fn xas_try_split_min_order(order: c_uint) -> c_uint;
}

//
// xas_reload() - Refetch an entry from the xarray.
// @xas: XArray operation state.
//
// Use this function to check that a previously loaded entry still has
// the same value.  This is useful for the lockless pagecache lookup where
// we walk the array with only the RCU lock to protect us, lock the page,
// then check that the page hasn't moved since we looked it up.
//
// The caller guarantees that @xas is still valid.  If it may be in an
// error or restart state, call xas_load() instead.
//
// Return: The entry at this location in the xarray.
//
extern "C" {
    pub fn xa_head(_arg: xas->xa) -> return;
}
extern "C" {
    pub fn xa_entry(_arg: xas->xa, _arg: node, _arg: offset) -> return;
}
//
// xas_set() - Set up XArray operation state for a different index.
// @xas: XArray operation state.
// @index: New index into the XArray.
//
// Move the operation state to refer to a different index.  This will
// have the effect of starting a walk from the top; see xas_next()
// to move to an adjacent index.
//
// xas_advance() - Skip over sibling entries.
// @xas: XArray operation state.
// @index: Index of last sibling entry.
//
// Move the operation state to refer to the last sibling entry.
// This is useful for loops that normally want to see sibling
// entries but sometimes want to skip them.  Use xas_set() if you
// want to move to an index which is not part of this entry.
//
// xas_set_order() - Set up XArray operation state for a multislot entry.
// @xas: XArray operation state.
// @index: Target of the operation.
// @order: Entry occupies 2^@order indices.
//

//
// xas_set_update() - Set up XArray operation state for a callback.
// @xas: XArray operation state.
// @update: Function to call when updating a node.
//
// The XArray can notify a caller after it has updated an xa_node.
// This is advanced functionality and is only needed by the page
// cache and swap cache.
//
// xas_next_entry() - Advance iterator to next present entry.
// @xas: XArray operation state.
// @max: Highest index to return.
//
// xas_next_entry() is an inline function to optimise xarray traversal for
// speed.  It is equivalent to calling xas_find(), and will call xas_find()
// for all the hard cases.
//
// Return: The next present entry after the one currently referred to by @xas.
//
extern "C" {
    pub fn xas_find(_arg: xas, _arg: max) -> return;
}
extern "C" {
    pub fn xas_find(_arg: xas, _arg: max) -> return;
}
extern "C" {
    pub fn xas_find(_arg: xas, _arg: max) -> return;
}
extern "C" {
    pub fn xas_find(_arg: xas, _arg: max) -> return;
}
// Private
extern "C" {
    pub fn __ffs(_arg: data) -> return;
}
extern "C" {
    pub fn find_next_bit(_arg: addr, _arg: XA_CHUNK_SIZE, _arg: offset) -> return;
}
//
// xas_next_marked() - Advance iterator to next marked entry.
// @xas: XArray operation state.
// @max: Highest index to return.
// @mark: Mark to search for.
//
// xas_next_marked() is an inline function to optimise xarray traversal for
// speed.  It is equivalent to calling xas_find_marked(), and will call
// xas_find_marked() for all the hard cases.
//
// Return: The next marked entry after the one currently referred to by @xas.
//
extern "C" {
    pub fn xas_find_marked(_arg: xas, _arg: max, _arg: mark) -> return;
}
extern "C" {
    pub fn xas_find_marked(_arg: xas, _arg: max, _arg: mark) -> return;
}
extern "C" {
    pub fn xas_find_marked(_arg: xas, _arg: max, _arg: mark) -> return;
}
//
// If iterating while holding a lock, drop the lock and reschedule
// every %XA_CHECK_SCHED loops.
//
// xas_for_each() - Iterate over a range of an XArray.
// @xas: XArray operation state.
// @entry: Entry retrieved from the array.
// @max: Maximum index to retrieve from array.
//
// The loop body will be executed for each entry present in the xarray
// between the current xas position and @max.  @entry will be set to
// the entry retrieved from the xarray.  It is safe to delete entries
// from the array in the loop body.  You should hold either the RCU lock
// or the xa_lock while iterating.  If you need to drop the lock, call
// xas_pause() first.
//

//
// xas_for_each_marked() - Iterate over a range of an XArray.
// @xas: XArray operation state.
// @entry: Entry retrieved from the array.
// @max: Maximum index to retrieve from array.
// @mark: Mark to search for.
//
// The loop body will be executed for each marked entry in the xarray
// between the current xas position and @max.  @entry will be set to
// the entry retrieved from the xarray.  It is safe to delete entries
// from the array in the loop body.  You should hold either the RCU lock
// or the xa_lock while iterating.  If you need to drop the lock, call
// xas_pause() first.
//

//
// xas_for_each_conflict() - Iterate over a range of an XArray.
// @xas: XArray operation state.
// @entry: Entry retrieved from the array.
//
// The loop body will be executed for each entry in the XArray that
// lies within the range specified by @xas.  If the loop terminates
// normally, @entry will be %NULL.  The user may break out of the loop,
// which will leave @entry set to the conflicting entry.  The caller
// may also call xa_set_err() to exit the loop while setting an error
// to record the reason.
//

//
// xas_prev() - Move iterator to previous index.
// @xas: XArray operation state.
//
// If the @xas was in an error state, it will remain in an error state
// and this function will return %NULL.  If the @xas has never been walked,
// it will have the effect of calling xas_load().  Otherwise one will be
// subtracted from the index and the state will be walked to the correct
// location in the array for the next operation.
//
// If the iterator was referencing index 0, this function wraps
// around to %ULONG_MAX.
//
// Return: The entry at the new index.  This may be %NULL or an internal
// entry.
//
extern "C" {
    pub fn __xas_prev(_arg: xas) -> return;
}
extern "C" {
    pub fn xa_entry(_arg: xas->xa, _arg: node, _arg: xas->xa_offset) -> return;
}
//
// xas_next() - Move state to next index.
// @xas: XArray operation state.
//
// If the @xas was in an error state, it will remain in an error state
// and this function will return %NULL.  If the @xas has never been walked,
// it will have the effect of calling xas_load().  Otherwise one will be
// added to the index and the state will be walked to the correct
// location in the array for the next operation.
//
// If the iterator was referencing index %ULONG_MAX, this function wraps
// around to 0.
//
// Return: The entry at the new index.  This may be %NULL or an internal
// entry.
//
extern "C" {
    pub fn __xas_next(_arg: xas) -> return;
}
extern "C" {
    pub fn xa_entry(_arg: xas->xa, _arg: node, _arg: xas->xa_offset) -> return;
}
