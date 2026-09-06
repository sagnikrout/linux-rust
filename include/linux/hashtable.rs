//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hashtable.h
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
// Statically sized hash table implementation
// (C) 2012  Sasha Levin <levinsasha928@gmail.com>
//

// Use hash_32 when possible to allow for fast 32bit hashing in 64bit kernels.

//
// hash_init - initialize a hash table
// @hashtable: hashtable to be initialized
//
// Calculates the size of the hashtable from the given parameter, otherwise
// same as hash_init_size.
//
// This has to be a macro since HASH_BITS() will not work on pointers since
// it calculates the size during preprocessing.
//

//
// hash_add - add an object to a hashtable
// @hashtable: hashtable to add to
// @node: the &struct hlist_node of the object to be added
// @key: the key of the object to be added
//

//
// hash_add_rcu - add an object to a rcu enabled hashtable
// @hashtable: hashtable to add to
// @node: the &struct hlist_node of the object to be added
// @key: the key of the object to be added
//

//
// hash_hashed - check whether an object is in any hashtable
// @node: the &struct hlist_node of the object to be checked
//
// hash_empty - check whether a hashtable is empty
// @hashtable: hashtable to check
//
// This has to be a macro since HASH_BITS() will not work on pointers since
// it calculates the size during preprocessing.
//

//
// hash_del - remove an object from a hashtable
// @node: &struct hlist_node of the object to remove
//
// hash_del_rcu - remove an object from a rcu enabled hashtable
// @node: &struct hlist_node of the object to remove
//
// hash_for_each - iterate over a hashtable
// @name: hashtable to iterate
// @bkt: integer to use as bucket loop cursor
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
//

//
// hash_for_each_rcu - iterate over a rcu enabled hashtable
// @name: hashtable to iterate
// @bkt: integer to use as bucket loop cursor
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
//

//
// hash_for_each_safe - iterate over a hashtable safe against removal of
// hash entry
// @name: hashtable to iterate
// @bkt: integer to use as bucket loop cursor
// @tmp: a &struct hlist_node used for temporary storage
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
//

//
// hash_for_each_possible - iterate over all possible objects hashing to the
// same bucket
// @name: hashtable to iterate
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
// @key: the key of the objects to iterate over
//

//
// hash_for_each_possible_rcu - iterate over all possible objects hashing to the
// same bucket in an rcu enabled hashtable
// @name: hashtable to iterate
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
// @key: the key of the objects to iterate over
//

//
// hash_for_each_possible_rcu_notrace - iterate over all possible objects hashing
// to the same bucket in an rcu enabled hashtable in a rcu enabled hashtable
// @name: hashtable to iterate
// @obj: the type * to use as a loop cursor for each entry
// @member: the name of the hlist_node within the struct
// @key: the key of the objects to iterate over
//
// This is the same as hash_for_each_possible_rcu() except that it does
// not do any RCU debugging or tracing.
//

//
// hash_for_each_possible_safe - iterate over all possible objects hashing to the
// same bucket safe against removals
// @name: hashtable to iterate
// @obj: the type * to use as a loop cursor for each entry
// @tmp: a &struct hlist_node used for temporary storage
// @member: the name of the hlist_node within the struct
// @key: the key of the objects to iterate over
//

