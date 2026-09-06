//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/tracing_map.h
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
pub const TRACING_MAP_BITS_DEFAULT: c_int = 11;
pub const TRACING_MAP_BITS_MAX: c_int = 17;
pub const TRACING_MAP_BITS_MIN: c_int = 7;
pub const TRACING_MAP_KEYS_MAX: c_int = 3;
pub const TRACING_MAP_VALS_MAX: c_int = 3;

pub const TRACING_MAP_VARS_MAX: c_int = 16;
pub const TRACING_MAP_SORT_KEYS_MAX: c_int = 2;
extern "C" {
    pub fn int(val_a: *mut *mut tracing_map_cmp_fn_t) (void, val_b: *mut c_void) -> typedef;
}
//
// This is an overview of the tracing_map data structures and how they
// relate to the tracing_map API.  The details of the algorithms
// aren't discussed here - this is just a general overview of the data
// structures and how they interact with the API.
//
// The central data structure of the tracing_map is an initially
// zeroed array of struct tracing_map_entry (stored in the map field
// of struct tracing_map).  tracing_map_entry is a very simple data
// structure containing only two fields: a 32-bit unsigned 'key'
// variable and a pointer named 'val'.  This array of struct
// tracing_map_entry is essentially a hash table which will be
// modified by a single function, tracing_map_insert(), but which can
// be traversed and read by a user at any time (though the user does
// this indirectly via an array of tracing_map_sort_entry - see the
// explanation of that data structure in the discussion of the
// sorting-related data structures below).
//
// The central function of the tracing_map API is
// tracing_map_insert().  tracing_map_insert() hashes the
// arbitrarily-sized key passed into it into a 32-bit unsigned key.
// It then uses this key, truncated to the array size, as an index
// into the array of tracing_map_entries.  If the value of the 'key'
// field of the tracing_map_entry found at that location is 0, then
// that entry is considered to be free and can be claimed, by
// replacing the 0 in the 'key' field of the tracing_map_entry with
// the new 32-bit hashed key.  Once claimed, that tracing_map_entry's
// 'val' field is then used to store a unique element which will be
// forever associated with that 32-bit hashed key in the
// tracing_map_entry.
//
// That unique element now in the tracing_map_entry's 'val' field is
// an instance of tracing_map_elt, where 'elt' in the latter part of
// that variable name is short for 'element'.  The purpose of a
// tracing_map_elt is to hold values specific to the particular
// 32-bit hashed key it's associated with.  Things such as the unique
// set of aggregated sums associated with the 32-bit hashed key, along
// with a copy of the full key associated with the entry, and which
// was used to produce the 32-bit hashed key.
//
// When tracing_map_create() is called to create the tracing map, the
// user specifies (indirectly via the map_bits param, the details are
// unimportant for this discussion) the maximum number of elements
// that the map can hold (stored in the max_elts field of struct
// tracing_map).  This is the maximum possible number of
// tracing_map_entries in the tracing_map_entry array which can be
// 'claimed' as described in the above discussion, and therefore is
// also the maximum number of tracing_map_elts that can be associated
// with the tracing_map_entry array in the tracing_map.  Because of
// the way the insertion algorithm works, the size of the allocated
// tracing_map_entry array is always twice the maximum number of
// elements (2 * max_elts).  This value is stored in the map_size
// field of struct tracing_map.
//
// Because tracing_map_insert() needs to work from any context,
// including from within the memory allocation functions themselves,
// both the tracing_map_entry array and a pool of max_elts
// tracing_map_elts are pre-allocated before any call is made to
// tracing_map_insert().
//
// The tracing_map_entry array is allocated as a single block by
// tracing_map_create().
//
// Because the tracing_map_elts are much larger objects and can't
// generally be allocated together as a single large array without
// failure, they're allocated individually, by tracing_map_init().
//
// The pool of tracing_map_elts are allocated by tracing_map_init()
// rather than by tracing_map_create() because at the time
// tracing_map_create() is called, there isn't enough information to
// create the tracing_map_elts.  Specifically,the user first needs to
// tell the tracing_map implementation how many fields the
// tracing_map_elts contain, and which types of fields they are (key
// or sum).  The user does this via the tracing_map_add_sum_field()
// and tracing_map_add_key_field() functions, following which the user
// calls tracing_map_init() to finish up the tracing map setup.  The
// array holding the pointers which make up the pre-allocated pool of
// tracing_map_elts is allocated as a single block and is stored in
// the elts field of struct tracing_map.
//
// There is also a set of structures used for sorting that might
// benefit from some minimal explanation.
//
// struct tracing_map_sort_key is used to drive the sort at any given
// time.  By 'any given time' we mean that a different
// tracing_map_sort_key will be used at different times depending on
// whether the sort currently being performed is a primary or a
// secondary sort.
//
// The sort key is very simple, consisting of the field index of the
// tracing_map_elt field to sort on (which the user saved when adding
// the field), and whether the sort should be done in an ascending or
// descending order.
//
// For the convenience of the sorting code, a tracing_map_sort_entry
// is created for each tracing_map_elt, again individually allocated
// to avoid failures that might be expected if allocated as a single
// large array of struct tracing_map_sort_entry.
// tracing_map_sort_entry instances are the objects expected by the
// various internal sorting functions, and are also what the user
// ultimately receives after calling tracing_map_sort_entries().
// Because it doesn't make sense for users to access an unordered and
// sparsely populated tracing_map directly, the
// tracing_map_sort_entries() function is provided so that users can
// retrieve a sorted list of all existing elements.  In addition to
// the associated tracing_map_elt 'elt' field contained within the
// tracing_map_sort_entry, which is the object of interest to the
// user, tracing_map_sort_entry objects contain a number of additional
// fields which are used for caching and internal purposes and can
// safely be ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_field {
    pub cmp_fn: tracing_map_cmp_fn_t,
    pub sum: core::sync::atomic::AtomicI64,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_elt {
    pub map: *mut tracing_map,
    pub fields: *mut tracing_map_field,
    pub vars: *mut core::sync::atomic::AtomicI64,
    pub var_set: *mut bool,
    pub key: *mut c_void,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_entry {
    pub key: u32,
    pub val: *mut tracing_map_elt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_sort_key {
    pub field_idx: c_uint,
    pub descending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_sort_entry {
    pub key: *mut c_void,
    pub elt: *mut tracing_map_elt,
    pub elt_copied: bool,
    pub dup: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_array {
    pub entries_per_page: c_uint,
    pub entry_size_shift: c_uint,
    pub entry_shift: c_uint,
    pub entry_mask: c_uint,
    pub n_pages: c_uint,
    pub __counted_by(n_pages): *mut *mut void pages[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map {
    pub key_size: c_uint,
    pub map_bits: c_uint,
    pub map_size: c_uint,
    pub max_elts: c_uint,
    pub next_elt: core::sync::atomic::AtomicI32,
    pub elts: *mut tracing_map_array,
    pub map: *mut tracing_map_array,
    pub ops: *const tracing_map_ops,
    pub private_data: *mut c_void,
    pub fields: [tracing_map_field; TRACING_MAP_FIELDS_MAX],
    pub n_fields: c_uint,
    pub key_idx: [c_int; TRACING_MAP_KEYS_MAX],
    pub n_keys: c_uint,
    pub sort_key: tracing_map_sort_key,
    pub n_vars: c_uint,
    pub hits: core::sync::atomic::AtomicI64,
    pub drops: core::sync::atomic::AtomicI64,
}

//
// struct tracing_map_ops - callbacks for tracing_map
//
// The methods in this structure define callback functions for various
// operations on a tracing_map or objects related to a tracing_map.
//
// For a detailed description of tracing_map_elt objects please see
// the overview of tracing_map data structures at the beginning of
// this file.
//
// All the methods below are optional.
//
// @elt_alloc: When a tracing_map_elt is allocated, this function, if
// defined, will be called and gives clients the opportunity to
// allocate additional data and attach it to the element
// (tracing_map_elt->private_data is meant for that purpose).
// Element allocation occurs before tracing begins, when the
// tracing_map_init() call is made by client code.
//
// @elt_free: When a tracing_map_elt is freed, this function is called
// and allows client-allocated per-element data to be freed.
//
// @elt_clear: This callback allows per-element client-defined data to
// be cleared, if applicable.
//
// @elt_init: This callback allows per-element client-defined data to
// be initialized when used i.e. when the element is actually
// claimed by tracing_map_insert() in the context of the map
// insertion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracing_map_ops {
    pub elt): *mut *mut int (elt_alloc)(struct tracing_map_elt,
    pub elt): *mut *mut void (elt_free)(struct tracing_map_elt,
    pub elt): *mut *mut void (elt_clear)(struct tracing_map_elt,
    pub elt): *mut *mut void (elt_init)(struct tracing_map_elt,
}

extern "C" {
    pub fn tracing_map_init(map: *mut tracing_map) -> c_int;
}
extern "C" {
    pub fn tracing_map_add_sum_field(map: *mut tracing_map) -> c_int;
}
extern "C" {
    pub fn tracing_map_add_var(map: *mut tracing_map) -> c_int;
}
extern "C" {
    pub fn tracing_map_destroy(map: *mut tracing_map);
}
extern "C" {
    pub fn tracing_map_clear(map: *mut tracing_map);
}
extern "C" {
    pub fn tracing_map_cmp_string(val_a: *mut c_void, val_b: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tracing_map_cmp_none(val_a: *mut c_void, val_b: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tracing_map_var_set(elt: *mut tracing_map_elt, i: c_uint) -> bool;
}
extern "C" {
    pub fn tracing_map_read_sum(elt: *mut tracing_map_elt, i: c_uint) -> u64;
}
extern "C" {
    pub fn tracing_map_read_var(elt: *mut tracing_map_elt, i: c_uint) -> u64;
}
extern "C" {
    pub fn tracing_map_read_var_once(elt: *mut tracing_map_elt, i: c_uint) -> u64;
}
