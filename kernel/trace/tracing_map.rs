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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


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
    pub __counted_by(n_pages): *mut *mut c_void pages[],
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
    pub elt): *mut *mut int (elt_alloc)(tracing_map_elt,
    pub elt): *mut *mut c_void (elt_free)(tracing_map_elt,
    pub elt): *mut *mut c_void (elt_clear)(tracing_map_elt,
    pub elt): *mut *mut c_void (elt_init)(tracing_map_elt,
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