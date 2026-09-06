//! Automatically rewritten from C Header to Rust Module
//! Source: mm/slab.h
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

//
// Internal slab definitions
//
// slab's alloc_flags definitions
pub const SLAB_ALLOC_DEFAULT: c_uint = 0x00 /* no flags */;
pub const SLAB_ALLOC_NOLOCK: c_uint = 0x01 /* a kmalloc_nolock() allocation */;
pub const SLAB_ALLOC_NEW_SLAB: c_uint = 0x02 /* a flag for alloc_slab_obj_exts() */;
pub const SLAB_ALLOC_NO_RECURSE: c_uint = 0x04 /* prevent kmalloc() recursion */;
pub const SLAB_ALLOC_NO_OBJ_EXT: c_uint = 0x08 /* prevent obj_exts array allocation */;
pub const SLAB_FREE_DEFAULT: c_uint = 0x00 /* no flags */;
pub const SLAB_FREE_NOLOCK: c_uint = 0x01 /* spinning not allowed */;
extern "C" {
    pub fn __alloc_size(_arg: 1) -> __assume_kmalloc_alignment;
}
extern "C" {
    pub fn __kmalloc_flags_noprof(_arg: PASS_TOKEN_PARAMS(size, _arg: token), _arg: flags, _arg: alloc_flags, _arg: node) -> return;
}

pub type freelist_full_t = u128;

pub type freelist_full_t = u64;

//
// Freelist pointer and counter to cmpxchg together, avoids the typical ABA
// problems with cmpxchg of just a pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freelist_counters {
    pub freelist: *mut c_void,
    pub counters: c_ulong,
    pub inuse:16: unsigned,
    pub objects:15: unsigned,
//
// If slab debugging is enabled then the
// frozen bit can be reused to indicate
// that the slab was corrupted
//
    pub frozen:1: unsigned,

//
// Some optimizations use free bits in 'counters' field
// to save memory or CPU. If these free bits are not
// available, such optimizations are disabled.
//
    pub obj_exts_in_object:1: unsigned,
    pub obj_exts_needs_objcg:1: unsigned,

}

// Reuses the bits in struct page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab {
    pub flags: memdesc_flags_t,
    pub slab_cache: *mut kmem_cache,
    pub slab_list: list_head,
// Double-word boundary
    pub freelist_counters: struct,
}

//
// slab_folio - The folio allocated for a slab
// @s: The slab.
//
// Slabs are allocated as folios that contain the individual objects and are
// using some fields in the first struct page of the folio - those fields are
// now accessed by struct slab. It is occasionally necessary to convert back to
// a folio in order to communicate with the rest of the mm.  Please use this
// helper function instead of casting yourself, as the implementation may change
// in the future.
//

//
// page_slab - Converts from struct page to its slab.
// @page: A page which may or may not belong to a slab.
//
// Return: The slab which contains this page or NULL if the page does
// not belong to a slab.  This includes pages returned from large kmalloc.
//
// slab_page - The first struct page allocated for a slab
// @s: The slab.
//
// A convenience wrapper for converting slab to the first struct page of the
// underlying folio, to communicate with code not yet converted to folio or
// struct slab.
//

extern "C" {
    pub fn folio_address(_arg: slab_folio(slab)) -> return;
}
extern "C" {
    pub fn memdesc_nid(_arg: &slab->flags) -> return;
}
extern "C" {
    pub fn NODE_DATA(_arg: slab_nid(slab)) -> return;
}
extern "C" {
    pub fn page_slab(_arg: virt_to_page(addr)) -> return;
}
extern "C" {
    pub fn folio_order(_arg: slab_folio(slab)) -> return;
}
//
// Word size structure that can be atomically updated or read and that
// contains both the order and the number of objects that a slab of the
// given order would contain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_order_objects {
    pub x: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_per_node_ptrs {
    pub barn: *mut node_barn,
    pub node: *mut kmem_cache_node,
}

//
// Slab cache management.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache {
    pub cpu_sheaves: *mut slub_percpu_sheaves ,
// Used for retrieving partial slabs, etc.
    pub flags: slab_flags_t,
    pub min_partial: c_ulong,
//     pub /: *mut *mut unsigned int size; / Object size including metadata,
//     pub /: *mut *mut unsigned int object_size; / Object size without metadata,
    pub reciprocal_size: reciprocal_value,
//     pub /: *mut *mut unsigned int offset; / Free pointer offset,
    pub sheaf_capacity: c_uint,
    pub oo: kmem_cache_order_objects,
// Allocation and freeing of slabs
    pub min: kmem_cache_order_objects,
//     pub /: *mut *mut gfp_t allocflags; / gfp flags to use on each alloc,
//     pub /: *mut *mut int refcount; / Refcount for slab cache destroy,
//     pub /: *mut *mut *mut *mut c_void (ctor)(void object); / Object constructor,
//     pub /: *mut *mut unsigned int inuse; / Offset to metadata,
//     pub /: *mut *mut unsigned int align; / Alignment,
//     pub /: *mut *mut unsigned int red_left_pad; / Left redzone padding size,
//     pub /: *const *const *const char name; / Name (only for display!),
//     pub /: *mut *mut list_head list; / List of slab caches,

//     pub /: *mut *mut kobject kobj; / For sysfs,

    pub random: c_ulong,

//
// Defragmentation by allocating from a remote node.
//
    pub remote_node_defrag_ratio: c_uint,

    pub random_seq: *mut c_uint,

    pub kasan_info: kasan_cache,

//     pub /: *mut *mut unsigned int useroffset; / Usercopy region offset,
//     pub /: *mut *mut unsigned int usersize; / Usercopy region size,

    pub cpu_stats: *mut kmem_cache_stats ,
    pub per_node: [kmem_cache_per_node_ptrs; MAX_NUMNODES],
}

//
// Every cache has !NULL s->cpu_sheaves but they may point to the
// bootstrap_sheaf temporarily during init, or permanently for the boot caches
// and caches with debugging enabled, or all caches with CONFIG_SLUB_TINY. This
// helper distinguishes whether cache has real non-bootstrap sheaves.
//
// Test CONFIG_SLUB_TINY for code elimination purposes

pub const SLAB_SUPPORTS_SYSFS: c_int = 1;
extern "C" {
    pub fn sysfs_slab_unlink(s: *mut kmem_cache);
}
extern "C" {
    pub fn sysfs_slab_release(s: *mut kmem_cache);
}
extern "C" {
    pub fn sysfs_slab_alias(s: *mut kmem_cache, name: *const c_char) -> c_int;
}

// Determine object index from a given position
extern "C" {
    pub fn __obj_to_index(_arg: cache, _arg: slab_address(slab), _arg: obj) -> return;
}
//
// kvfree_rcu_head offset can be only less than page size.
// Calculate the start address while preserving the KASAN tag.
//
// State of the slab allocator.
//
// This is used to describe the states of the allocator during bootup.
// Allocators use this to gradually bootstrap themselves. Most allocators
// have the problem that the structures used for managing slab caches are
// allocated from slab caches themselves.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slab_state {
    DOWN,			/* No slab functionality yet */
    PARTIAL,		/* SLUB: kmem_cache_node available */
    UP,			/* Slab caches usable but not all extras yet */
    FULL			/* Everything is working */
}

// The slab cache mutex protects the management structures during changes
// The list of all slab caches on the system
// The slab cache that manages slab cache information
// A table of kmalloc cache names and sizes
// Kmalloc array related functions
extern "C" {
    pub fn setup_kmalloc_cache_index_table();
}
extern "C" {
    pub fn create_kmalloc_caches();
}
//
// Find the kmem_cache structure that serves a given size of
// allocation
//
// This assumes size is larger than zero and not larger than
// KMALLOC_MAX_CACHE_SIZE and the caller must check that.
//
extern "C" {
    pub fn kmalloc_fix_flags(flags: gfp_t) -> gfp_t;
}
// Functions provided by the slab allocators
extern "C" {
    pub fn kmem_cache_init() ;
}
extern "C" {
    pub fn slab_unmergeable(s: *mut kmem_cache) -> c_int;
}
extern "C" {
    pub fn slab_args_unmergeable(args: *mut kmem_cache_args, flags: slab_flags_t) -> bool;
}
extern "C" {
    pub fn kmem_cache_flags(flags: slab_flags_t, name: *const c_char) -> slab_flags_t;
}
extern "C" {
    pub fn __kfree_rcu_sheaf(s: *mut kmem_cache, obj: *mut c_void, free_flags: c_uint) -> bool;
}
extern "C" {
    pub fn flush_all_rcu_sheaves();
}
extern "C" {
    pub fn flush_rcu_sheaves_on_cache(s: *mut kmem_cache);
}

extern "C" {
    pub fn __kmem_cache_empty(: *mut kmem_cache) -> bool;
}
extern "C" {
    pub fn __kmem_cache_shutdown(: *mut kmem_cache) -> c_int;
}
extern "C" {
    pub fn __kmem_cache_release(: *mut kmem_cache);
}
extern "C" {
    pub fn __kmem_cache_shrink(: *mut kmem_cache) -> c_int;
}
extern "C" {
    pub fn slab_kmem_cache_release(: *mut kmem_cache);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slabinfo {
    pub active_objs: c_ulong,
    pub num_objs: c_ulong,
    pub active_slabs: c_ulong,
    pub num_slabs: c_ulong,
    pub shared_avail: c_ulong,
    pub limit: c_uint,
    pub batchcount: c_uint,
    pub shared: c_uint,
    pub objects_per_slab: c_uint,
    pub cache_order: c_uint,
}

extern "C" {
    pub fn get_slabinfo(s: *mut kmem_cache, sinfo: *mut slabinfo);
}

extern "C" {
    pub fn print_tracking(s: *mut kmem_cache, object: *mut c_void);
}
extern "C" {
    pub fn validate_slab_cache(s: *mut kmem_cache) -> c_long;
}
extern "C" {
    pub fn static_branch_unlikely(_arg: &slub_debug_enabled) -> return;
}

//
// Returns true if any of the specified slab_debug flags is enabled for the
// cache. Use only for flags parsed by setup_slub_debug() as it also enables
// the static key.
//

extern "C" {
    pub fn slab_in_kunit_test() -> bool;
}

//
// slub is about to manipulate internal object metadata.  This memory lies
// outside the range of the allocated object, so accessing it would normally
// be reported by kasan as a bounds error.  metadata_access_enable() is used
// to tell kasan that these accesses are OK.
//
// Return true if KMALLOC_NORMAL caches may need obj_exts arrays.
//
// Memory allocation profiling requires obj_exts for all caches.
// Memcg usually doesn't need them for normal kmalloc caches, but kmalloc types
// with a priority higher than KMALLOC_CGROUP can be aliased with KMALLOC_NORMAL.
//
// Extended information for slab objects stored as a pointer to an array in
// slab->obj_exts (aliasing page->memcg_data) if MEMCG_DATA_OBJEXTS is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slabobj_ext {
//
// All elements of the union should be pointer-sized to avoid memory
// waste
//

    pub _objcg: *mut obj_cgroup,

    pub _ctref: codetag_ref,

}

extern "C" {
    pub fn cache_needs_objcg(_arg: slab->slab_cache) -> return;
}

//
// slab_obj_exts - get the pointer to the slab object extension vector
// associated with a slab.
// @slab: a pointer to the slab struct
//
// Returns the address of the object extension vector associated with the slab,
// or zero if no such vector has been associated yet.
// Do not dereference the return value directly; use get/put_slab_obj_exts()
// pair and slab_obj_ext() to access individual elements.
//
// Example usage:
//
// obj_exts = slab_obj_exts(slab);
// if (obj_exts) {
// get_slab_obj_exts(obj_exts);
// obj_ext = slab_obj_ext(s, slab, obj_exts, obj);
// // do something with obj_ext
// put_slab_obj_exts(obj_exts);
// }
//
// Note that the get/put semantics does not involve reference counting.
// Instead, it updates kasan/kmsan depth so that accesses to slabobj_ext
// won't be reported as access violations.
//

//
// obj_exts should be either NULL, a valid pointer with
// MEMCG_DATA_OBJEXTS bit set or be equal to OBJEXTS_ALLOC_FAIL.
//

//
// Note we cannot rely on the SLAB_OBJ_EXT_IN_OBJ flag here and need to
// check the per-slab bit. A cache can have SLAB_OBJ_EXT_IN_OBJ set, but
// allocations within_slab_leftover are preferred. And those may be
// possible or not depending on the particular slab's size.
//

//
// slab_obj_ext - get the pointer to the slab object extension metadata
// associated with an object in a slab.
// @s: cache that the slab belongs to
// @slab: a pointer to the slab struct
// @obj_exts: a pointer to the object extension vector
// @obj: a pointer to the object
//
// Returns a pointer to the object extension associated with the object.
// Must be called within a section covered by get/put_slab_obj_exts().
//
// KFENCE objects have NULL obj_exts and thus can't reach this
// and we don't need obj_to_index()
//
extern "C" {
    pub fn kasan_reset_tag(_arg: obj_ext) -> return;
}

// if objcg exists, it comes first, so we don't need to do anything

extern "C" {
    pub fn kvfree_rcu_cb(head: *mut rcu_head);
}

extern "C" {
    pub fn dump_unreclaimable_slab();
}

extern "C" {
    pub fn ___cache_free(cache: *mut kmem_cache, x: *mut c_void, addr: c_ulong);
}

extern "C" {
    pub fn cache_random_seq_destroy(cachep: *mut kmem_cache);
}

extern "C" {
    pub fn debugfs_slab_release(: *mut kmem_cache);
}

pub const KS_ADDRS_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_obj_info {
    pub kp_ptr: *mut c_void,
    pub kp_slab: *mut slab,
    pub kp_objp: *mut c_void,
    pub kp_data_offset: c_ulong,
    pub kp_slab_cache: *mut kmem_cache,
    pub kp_ret: *mut c_void,
    pub kp_stack: [*mut c_void; KS_ADDRS_COUNT],
    pub kp_free_stack: [*mut c_void; KS_ADDRS_COUNT],
}

extern "C" {
    pub fn __kmem_obj_info(kpp: *mut kmem_obj_info, object: *mut c_void, slab: *mut slab);
}

extern "C" {
    pub fn deferred_work_barrier();
}
extern "C" {
    pub fn defer_kfree_rcu(head: *mut kvfree_rcu_head);
}

extern "C" {
    pub fn skip_orig_size_check(s: *mut kmem_cache, object: *const c_void);
}