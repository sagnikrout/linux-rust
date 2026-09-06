//! Automatically rewritten from C to Rust
//! Source: mm/slub.c
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
// SLUB: A slab allocator with low overhead percpu array caches and mostly
// lockless freeing of objects to slabs in the slowpath.
//
// The allocator synchronizes using spin_trylock for percpu arrays in the
// fastpath, and cmpxchg_double (or bit spinlock) for slowpath freeing.
// Uses a centralized lock to manage a pool of partial slabs.
//
// (C) 2007 SGI, Christoph Lameter
// (C) 2011 Linux Foundation, Christoph Lameter
// (C) 2025 SUSE, Vlastimil Babka
//

//
// Lock order:
// 0.  cpu_hotplug_lock
// 1.  slab_mutex (Global Mutex)
// 2a. kmem_cache->cpu_sheaves->lock (Local trylock)
// 2b. barn->lock (Spinlock)
// 2c. node->list_lock (Spinlock)
// 3.  slab_lock(slab) (Only on some arches)
// 4.  object_map_lock (Only for debugging)
//
// slab_mutex
//
// The role of the slab_mutex is to protect the list of all the slabs
// and to synchronize major metadata changes to slab cache structures.
// Also synchronizes memory hotplug callbacks.
//
// slab_lock
//
// The slab_lock is a wrapper around the page lock, thus it is a bit
// spinlock.
//
// The slab_lock is only used on arches that do not have the ability
// to do a cmpxchg_double. It only protects:
//
// A. slab->freelist	-> List of free objects in a slab
// B. slab->inuse		-> Number of objects in use
// C. slab->objects	-> Number of objects in slab
// D. slab->frozen		-> frozen state
//
// SL_partial slabs
//
// Slabs on node partial list have at least one free object. A limited number
// of slabs on the list can be fully free (slab->inuse == 0), until we start
// discarding them. These slabs are marked with SL_partial, and the flag is
// cleared while removing them, usually to grab their freelist afterwards.
// This clearing also exempts them from list management. Please see
// __slab_free() for more details.
//
// Full slabs
//
// For caches without debugging enabled, full slabs (slab->inuse ==
// slab->objects and slab->freelist == NULL) are not placed on any list.
// The __slab_free() freeing the first object from such a slab will place
// it on the partial list. Caches with debugging enabled place such slab
// on the full list and use different allocation and freeing paths.
//
// Frozen slabs
//
// If a slab is frozen then it is exempt from list management. It is used to
// indicate a slab that has failed consistency checks and thus cannot be
// allocated from anymore - it is also marked as full. Any previously
// allocated objects will be simply leaked upon freeing instead of attempting
// to modify the potentially corrupted freelist and metadata.
//
// To sum up, the current scheme is:
// - node partial slab:            SL_partial && !full && !frozen
// - taken off partial list:      !SL_partial && !full && !frozen
// - full slab, not on any list:  !SL_partial &&  full && !frozen
// - frozen due to inconsistency: !SL_partial &&  full &&  frozen
//
// node->list_lock (spinlock)
//
// The list_lock protects the partial and full list on each node and
// the partial slab counter. If taken then no new slabs may be added or
// removed from the lists nor make the number of partial slabs be modified.
// (Note that the total number of slabs is an atomic value that may be
// modified without taking the list lock).
//
// The list_lock is a centralized lock and thus we avoid taking it as
// much as possible. As long as SLUB does not have to handle partial
// slabs, operations can continue without any centralized lock.
//
// For debug caches, all allocations are forced to go through a list_lock
// protected region to serialize against concurrent validation.
//
// cpu_sheaves->lock (local_trylock)
//
// This lock protects fastpath operations on the percpu sheaves. On !RT it
// only disables preemption and does no atomic operations. As long as the main
// or spare sheaf can handle the allocation or free, there is no other
// overhead.
//
// barn->lock (spinlock)
//
// This lock protects the operations on per-NUMA-node barn. It can quickly
// serve an empty or full sheaf if available, and avoid more expensive refill
// or flush operation.
//
// Lockless freeing
//
// Objects may have to be freed to their slabs when they are from a remote
// node (where we want to avoid filling local sheaves with remote objects)
// or when there are too many full sheaves. On architectures supporting
// cmpxchg_double this is done by a lockless update of slab's freelist and
// counters, otherwise slab_lock is taken. This only needs to take the
// list_lock if it's a first free to a full slab, or when a slab becomes empty
// after the free.
//
// irq, preemption, migration considerations
//
// Interrupts are disabled as part of list_lock or barn lock operations, or
// around the slab_lock operation, in order to make the slab allocator safe
// to use in the context of an irq.
// Preemption is disabled as part of local_trylock operations.
// kmalloc_nolock() and kfree_nolock() are safe in NMI context but see
// their limitations.
//
// SLUB assigns two object arrays called sheaves for caching allocations and
// frees on each cpu, with a NUMA node shared barn for balancing between cpus.
// Allocations and frees are primarily served from these sheaves.
//
// Slabs with free elements are kept on a partial list and during regular
// operations no list for full slabs is used. If an object in a full slab is
// freed then the slab will show up again on the partial lists.
// We track full slabs for debugging purposes though because otherwise we
// cannot scan all objects.
//
// Slabs are freed when they become empty. Teardown and setup is minimal so we
// rely on the page allocators per cpu caches for fast frees and allocs.
//
// SLAB_DEBUG_FLAGS	Slab requires special handling due to debug
// options set. This moves	slab handling out of
// the fast path and disables lockless freelists.
//
// enum slab_flags - How the slab flags bits are used.
// @SL_locked: Is locked with slab_lock()
// @SL_partial: On the per-node partial list
// @SL_pfmemalloc: Was allocated from PF_MEMALLOC reserves
//
// The slab flags share space with the page flags but some bits have
// different interpretations.  The high bits are used for information
// like zone/node/section.
//
    enum slab_flags {
    SL_locked = PG_locked,
    SL_partial = PG_workingset,	/* Historical reasons for this bit */
    SL_pfmemalloc = PG_active,	/* Historical reasons for this bit */
    };

// Macro flag: #define __fastpath_inline

pub static mut slub_debug_enabled: usize = 0;

pub static mut slub_debug_enabled: usize = 0;

pub static mut strict_numa: usize = 0;

pub static mut CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT: usize = 0;

// Structure holding extra parameters for slab allocations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_alloc_context {
    pub caller_addr: c_ulong,
    pub orig_size: usize,
    pub alloc_flags: c_uint,
    pub lru: *mut list_lru,
}

// Structure holding parameters for get_partial_node_bulk()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partial_bulk_context {
    pub flags: gfp_t,
    pub min_objects: c_uint,
    pub max_objects: c_uint,
    pub slabs: list_head,
}

// Structure used to iterate over objects within a slab
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_obj_iter {
    pub pos: c_ulong,
    pub start: *mut c_void,

    pub freelist_count: c_ulong,
    pub page_limit: c_ulong,
    pub random: bool,

}

#[no_mangle]
pub unsafe extern "C" fn kmem_cache_debug(s: *mut kmem_cache) -> bool {
    return kmem_cache_debug_flags(s, SLAB_DEBUG_FLAGS);
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_red_left(s: *mut kmem_cache, p: *mut c_void) -> *mut c_void {
    if (kmem_cache_debug_flags(s, SLAB_RED_ZONE)) {
    p += s.red_left_pad;
    }
    return p;
    }
//
// Issues still to be resolved:
//
// - Support PAGE_ALLOC_DEBUG. Should be easy to do.
//
// - Variable sizing of the per node arrays
//
// Enable to log cmpxchg failures

//
// Minimum number of partial slabs. These will be left on the partial
// lists even if they are empty. kmem_cache_shrink may reclaim them.
//
pub const MIN_PARTIAL: c_int = 5;
//
// Maximum number of desirable partial slabs.
// The existence of more partial slabs makes kmem_cache_shrink
// sort the partial list by the number of objects in use.
//
pub const MAX_PARTIAL: c_int = 10;

pub const MIN_PARTIAL: c_int = 0;
pub const MAX_PARTIAL: c_int = 0;

    SLAB_POISON | SLAB_STORE_USER)
//
// These debug flags cannot use CMPXCHG because there might be consistency
// issues when checking or reading debug information
//

    SLAB_TRACE)
//
// Debugging flags that require metadata to be stored in the slab.  These get
// disabled when slab_debug=O is used and a cache's min order increases with
// metadata.
//

pub const OO_SHIFT: c_int = 16;

// Internal SLUB flags
// Poison object

// Use cmpxchg_double

//
// Tracking user of a slab.
//
pub const TRACK_ADDRS_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct track {
//     pub /: *mut *mut unsigned long addr; / Called from address,

    pub handle: depot_stack_handle_t,

//     pub /: *mut *mut int cpu; / Was running on cpu,
//     pub /: *mut *mut int pid; / Pid context,
//     pub /: *mut *mut unsigned long when; / When did the operation occur,
}

    enum track_item { TRACK_ALLOC, TRACK_FREE };

// forward_decl: sysfs_slab_add;
    static int __init slab_kset_init(void);
    static void __init slab_sysfs_process_aliases(void);

#[no_mangle]
pub unsafe extern "C" fn sysfs_slab_add(s: *mut kmem_cache) -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn slab_kset_init() -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn slab_sysfs_process_aliases() { }

// forward_decl: debugfs_slab_add;
    static void __init slab_debugfs_root_init(void);

#[no_mangle]
pub unsafe extern "C" fn debugfs_slab_add(s: *mut kmem_cache) { }
#[no_mangle]
pub unsafe extern "C" fn slab_debugfs_root_init() { }

    enum add_mode {
    ADD_TO_HEAD,
    ADD_TO_TAIL,
    };
    enum stat_item {
    ALLOC_FASTPATH,		/* Allocation from percpu sheaves */
    ALLOC_SLOWPATH,		/* Allocation from partial or new slab */
    FREE_RCU_SHEAF,		/* Free to rcu_free sheaf */
    FREE_RCU_SHEAF_FAIL,	/* Failed to free to a rcu_free sheaf */
    FREE_FASTPATH,		/* Free to percpu sheaves */
    FREE_SLOWPATH,		/* Free to a slab */
    FREE_ADD_PARTIAL,	/* Freeing moves slab to partial list */
    FREE_REMOVE_PARTIAL,	/* Freeing removes last object */
    ALLOC_SLAB,		/* New slab acquired from page allocator */
    ALLOC_NODE_MISMATCH,	/* Requested node different from cpu sheaf */
    FREE_SLAB,		/* Slab freed to the page allocator */
    ORDER_FALLBACK,		/* Number of times fallback was necessary */
    CMPXCHG_DOUBLE_FAIL,	/* Failures of slab freelist update */
    SHEAF_FLUSH,		/* Objects flushed from a sheaf */
    SHEAF_REFILL,		/* Objects refilled to a sheaf */
    SHEAF_ALLOC,		/* Allocation of an empty sheaf including oversized ones */
    SHEAF_FREE,		/* Freeing of an empty sheaf including oversized ones */
    BARN_GET,		/* Got full sheaf from barn */
    BARN_GET_FAIL,		/* Failed to get full sheaf from barn */
    BARN_PUT,		/* Put full sheaf to barn */
    BARN_PUT_FAIL,		/* Failed to put full sheaf to barn */
    SHEAF_PREFILL_FAST,	/* Sheaf prefill grabbed the spare sheaf */
    SHEAF_PREFILL_SLOW,	/* Sheaf prefill found no spare sheaf */
    SHEAF_PREFILL_OVERSIZE,	/* Allocation of oversize sheaf for prefill */
    SHEAF_RETURN_FAST,	/* Sheaf return reattached spare sheaf */
    SHEAF_RETURN_SLOW,	/* Sheaf return could not reattach spare */
    NR_SLUB_STAT_ITEMS
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_stats {
    pub stat: [c_uint; NR_SLUB_STAT_ITEMS],
}

#[no_mangle]
pub unsafe extern "C" fn stat(s: *const kmem_cache, si: stat_item) {

//
// The rmw is racy on a preemptible kernel but this is acceptable, so
// avoid this_cpu_add()'s irq-disable overhead.
//
    raw_cpu_inc(s.cpu_stats.stat[si]);

    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn stat_add(s: *const kmem_cache, si: stat_item, v: c_int) {

    raw_cpu_add(s.cpu_stats.stat[si], v);

    }
pub const MAX_FULL_SHEAVES: c_int = 10;
pub const MAX_EMPTY_SHEAVES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_barn {
    pub lock: spinlock_t,
    pub sheaves_full: list_head,
    pub sheaves_empty: list_head,
    pub nr_full: c_uint,
    pub nr_empty: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_sheaf {
    union {
    pub rcu_head: rcu_head,
    pub barn_list: list_head,
// only used to defer call_rcu() in unknown context
    pub llnode: llist_node,
// only used for prefilled sheafs
    struct {
    pub capacity: c_uint,
    pub pfmemalloc: bool,
}

    };
pub static mut cache: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut node = 0; /* only used for rcu_sheaf */
    void *objects[];
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slub_percpu_sheaves {
    pub lock: local_trylock_t,
//     pub /: *mut *mut *mut slab_sheaf main; / never NULL when unlocked,
//     pub /: *mut *mut *mut slab_sheaf spare; / empty or full, may be NULL,
//     pub /: *mut *mut *mut slab_sheaf rcu_free; / for batching kfree_rcu(),
}

//
// The slab lists for all objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_node {
    pub list_lock: spinlock_t,
    pub nr_partial: c_ulong,
    pub partial: list_head,

    pub nr_slabs: atomic_long_t,
    pub total_objects: atomic_long_t,
    pub full: list_head,

}

#[no_mangle]
pub unsafe extern "C" fn get_node(s: *mut kmem_cache, node: c_int) -> *mut c_void {
    return s.per_node[node].node;
    }
#[no_mangle]
pub unsafe extern "C" fn get_barn_node(s: *mut kmem_cache, node: c_int) -> *mut c_void {
    return s.per_node[node].barn;
    }
//
// Get the barn of the current cpu's NUMA node. It may be a memoryless node.
//
#[no_mangle]
pub unsafe extern "C" fn get_barn(s: *mut kmem_cache) -> *mut c_void {
    return get_barn_node(s, numa_node_id());
    }
//
// Iterator over all nodes. The body will be executed for each node that has
// a kmem_cache_node structure allocated (which is true for all online nodes)
//

    for (__node = 0; __node < nr_node_ids; __node++)  {
    if ((__n = get_node(__s, __node)))
//
// Tracks for which NUMA nodes we have kmem_cache_nodes allocated.
// Corresponds to node_state[N_MEMORY], but can temporarily
// differ during memory hotplug/hotremove operations.
// Protected by slab_mutex.
//
    static nodemask_t slab_nodes;
    }
//
// Similar to slab_nodes but for where we have node_barn allocated.
// Corresponds to N_ONLINE nodes.
//
    static nodemask_t slab_barn_nodes;
//
// Workqueue used for flushing cpu and kfree_rcu sheaves.
//
pub static mut flushwq: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slub_flush_work {
    pub work: work_struct,
    pub s: *mut kmem_cache,
    pub skip: bool,
}

pub static mut flush_lock: usize = 0;
pub static mut struct slub_flush_work: usize = 0;
//
// Core slab cache functions
//
// Returns freelist pointer (ptr). With hardening, this is obfuscated
// with an XOR of the address where the pointer is held and a per-cache
// random number.
//
    static inline freeptr_t freelist_ptr_encode(const struct kmem_cache *s,
    void *ptr, unsigned long ptr_addr)
    {
    let mut encoded = 0;

    encoded = (unsigned long)ptr ^ s.random ^ swab(ptr_addr);

    encoded = (unsigned long)ptr;

    return (freeptr_t){.v = encoded};
    }
#[no_mangle]
pub unsafe extern "C" fn freelist_ptr_decode(s: *mut kmem_cache, ptr: freeptr_t, ptr_addr: c_ulong) -> *mut c_void {
pub static mut decoded: *mut c_void = core::ptr::null_mut();

    decoded = (ptr.v ^ s.random ^ swab(ptr_addr));

    decoded = ptr.v;

    return decoded;
    }
#[no_mangle]
pub unsafe extern "C" fn get_freepointer(s: *mut kmem_cache, object: *mut c_void) -> *mut c_void {
    let mut ptr_addr = 0;
    let mut p;
    object = kasan_reset_tag(object);
    ptr_addr = (unsigned long)object + s.offset;
    p = *(ptr_addr);
    return freelist_ptr_decode(s, p, ptr_addr);
    }
#[no_mangle]
pub unsafe extern "C" fn set_freepointer(s: *mut kmem_cache, object: *mut c_void, fp: *mut c_void) {
pub static mut freeptr_addr: c_ulong = 0;

    BUG_ON!(object == fp); /* naive detection of double free or corruption */

    freeptr_addr = (unsigned long)kasan_reset_tag(freeptr_addr);
// freeptr_addr = freelist_ptr_encode(s, fp, freeptr_addr);
    }
//
// See comment in calculate_sizes().
//
#[no_mangle]
pub unsafe extern "C" fn freeptr_outside_object(s: *mut kmem_cache) -> bool {
    return s.offset >= s.inuse;
    }
//
// Return offset of the end of info block which is inuse + free pointer if
// not overlapping with object.
//
#[no_mangle]
pub unsafe extern "C" fn get_info_end(s: *mut kmem_cache) -> c_uint {
    if (freeptr_outside_object(s)) {
    return s.inuse + sizeof!;
    }
    else {
    return s.inuse;
    }
    }
// Loop over all objects in a slab

    for (__p = fixup_red_left(__s, __addr); 
    __p < (__addr) + (__objects) * (__s).size; 
    __p += (__s).size) {
#[no_mangle]
pub unsafe extern "C" fn order_objects(order: c_uint, size: c_uint) -> c_uint {
    }
    return ((unsigned int)PAGE_SIZE << order) / size;
    }
#[no_mangle]
pub unsafe extern "C" fn oo_make(order: c_uint, size: c_uint) {
pub static mut kmem_cache_order_objects: usize = 0;
    return x;
    }
#[no_mangle]
pub unsafe extern "C" fn oo_order(x: kmem_cache_order_objects) -> c_uint {
    return x.x >> OO_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn oo_objects(x: kmem_cache_order_objects) -> c_uint {
    return x.x & OO_MASK;
    }
//
// If network-based swap is enabled, slub must keep track of whether memory
// were allocated from pfmemalloc reserves.
//
#[no_mangle]
pub unsafe extern "C" fn slab_test_pfmemalloc(slab: *const slab) -> bool {
    return test_bit(SL_pfmemalloc, &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_set_pfmemalloc(slab: *mut slab) {
    set_bit(SL_pfmemalloc, &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn __slab_clear_pfmemalloc(slab: *mut slab) {
    __clear_bit(SL_pfmemalloc, &slab.flags.f);
    }
//
// Per slab locking using the pagelock
//
#[no_mangle]
unsafe extern "C" fn slab_lock(slab: *mut slab) -> __always_inline void {
    bit_spin_lock(SL_locked, &slab.flags.f);
    }
#[no_mangle]
unsafe extern "C" fn slab_unlock(slab: *mut slab) -> __always_inline void {
    bit_spin_unlock(SL_locked, &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn __update_freelist_fast(slab: *mut slab, old: *mut freelist_counters, new: *mut freelist_counters) -> bool {

    return try_cmpxchg_freelist(&slab.freelist_counters,
    &old.freelist_counters,
    new.freelist_counters);

    return false;

    }
#[no_mangle]
pub unsafe extern "C" fn __update_freelist_slow(slab: *mut slab, old: *mut freelist_counters, new: *mut freelist_counters) -> bool {
pub static mut ret: bool = false;
    slab_lock(slab);
    if (slab.freelist == old.freelist &&
    slab.counters == old.counters) {
    slab.freelist = new.freelist;
// prevent tearing for the read in get_partial_node_bulk()
    WRITE_ONCE(slab.counters, new.counters);
    ret = true;
    }
    slab_unlock(slab);
    return ret;
    }
//
// Interrupts must be disabled (for the fallback code to work right), typically
// by an _irqsave() lock variant. On PREEMPT_RT the preempt_disable(), which is
// part of bit_spin_lock(), is sufficient because the policy is not to allow any
// allocation/ free operation in hardirq context. Therefore nothing can
// interrupt the operation.
//
#[no_mangle]
pub unsafe extern "C" fn __slab_update_freelist(s: *mut kmem_cache, slab: *mut slab, old: *mut freelist_counters, new: *mut freelist_counters, n: *mut c_char) -> bool {
    let mut ret = 0;
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    lockdep_assert_irqs_disabled();
    }
    if (s.flags & __CMPXCHG_DOUBLE) {
    ret = __update_freelist_fast(slab, old, new);
    }
    else {
    ret = __update_freelist_slow(slab, old, new);
    }
    if (likely(ret)) {
    return true;
    }
    cpu_relax();
    stat(s, CMPXCHG_DOUBLE_FAIL);

    pr_info!("%s %s: cmpxchg double redo ", n, s.name);

    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn slab_update_freelist(s: *mut kmem_cache, slab: *mut slab, old: *mut freelist_counters, new: *mut freelist_counters, n: *mut c_char) -> bool {
    let mut ret = 0;
    if (s.flags & __CMPXCHG_DOUBLE) {
    ret = __update_freelist_fast(slab, old, new);
    } else {
    let mut flags = 0;
    local_irq_save(flags);
    ret = __update_freelist_slow(slab, old, new);
    local_irq_restore(flags);
    }
    if (likely(ret)) {
    return true;
    }
    cpu_relax();
    stat(s, CMPXCHG_DOUBLE_FAIL);

    pr_info!("%s %s: cmpxchg double redo ", n, s.name);

    return false;
    }
//
// kmalloc caches has fixed sizes (mostly power of 2), and kmalloc() API
// family will round up the real request size to these fixed ones, so
// there could be an extra area than what is requested. Save the original
// request size in the meta data area, for better debug and sanity check.
//
#[no_mangle]
pub unsafe extern "C" fn set_orig_size(s: *mut kmem_cache, object: *mut c_void, orig_size: c_ulong) {
    let mut p = kasan_reset_tag(object);
    if (!slub_debug_orig_size(s)) {
    return;
    }
    p += get_info_end(s);
    p += sizeof!(track) * 2;
// p = orig_size;
    }
#[no_mangle]
pub unsafe extern "C" fn get_orig_size(s: *mut kmem_cache, object: *mut c_void) -> c_ulong {
    let mut p = kasan_reset_tag(object);
    if (is_kfence_address(object)) {
    return kfence_ksize(object);
    }
    if (!slub_debug_orig_size(s)) {
    return s.object_size;
    }
    p += get_info_end(s);
    p += sizeof!(track) * 2;
    return *p;
    }

//
// Check if memory cgroup or memory allocation profiling is enabled.
// If enabled, SLUB tries to reduce memory overhead of accounting
// slab objects. If neither is enabled when this function is called,
// the optimization is simply skipped to avoid affecting caches that do not
// need slabobj_ext metadata.
//
// However, this may disable optimization when memory cgroup or memory
// allocation profiling is used, but slabs are created too early
// even before those subsystems are initialized.
//
#[no_mangle]
pub unsafe extern "C" fn need_slab_obj_exts(s: *mut kmem_cache) -> bool {
    if (s.flags & SLAB_NO_OBJ_EXT) {
    return false;
    }
    if (memcg_kmem_online() && (s.flags & SLAB_ACCOUNT)) {
    return true;
    }
    if (mem_alloc_profiling_enabled()) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn obj_exts_size_in_slab(slab: *mut slab) -> c_uint {
    return slab_obj_ext_size(slab) * slab.objects;
    }
#[no_mangle]
pub unsafe extern "C" fn obj_exts_offset_in_slab(s: *mut kmem_cache, slab: *mut slab) -> c_ulong {
    let mut objext_offset = 0;
    objext_offset = s.size * slab.objects;
    objext_offset = ALIGN(objext_offset, sizeof!(slabobj_ext));
    return objext_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn obj_exts_fit_within_slab_leftover(s: *mut kmem_cache, slab: *mut slab) -> bool {
pub static mut objext_offset: c_ulong = 0;
pub static mut objext_size: c_ulong = 0;
    return objext_offset + objext_size <= slab_size(slab);
    }
#[no_mangle]
pub unsafe extern "C" fn obj_exts_in_slab(s: *mut kmem_cache, slab: *mut slab) -> bool {
    let mut obj_exts = 0;
    let mut start = 0;
    let mut end = 0;
    obj_exts = slab_obj_exts(slab);
    if (!obj_exts) {
    return false;
    }
    start = (unsigned long)slab_address(slab);
    end = start + slab_size(slab);
    return (obj_exts >= start) && (obj_exts < end);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: need_slab_obj_exts
pub unsafe extern "C" fn need_slab_obj_exts_dup(s: *mut kmem_cache) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: obj_exts_size_in_slab
pub unsafe extern "C" fn obj_exts_size_in_slab_dup(slab: *mut slab) -> c_uint {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: obj_exts_offset_in_slab
pub unsafe extern "C" fn obj_exts_offset_in_slab_dup(s: *mut kmem_cache, slab: *mut slab) -> c_ulong {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: obj_exts_fit_within_slab_leftover
pub unsafe extern "C" fn obj_exts_fit_within_slab_leftover_dup(s: *mut kmem_cache, slab: *mut slab) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: obj_exts_in_slab
pub unsafe extern "C" fn obj_exts_in_slab_dup(s: *mut kmem_cache, slab: *mut slab) -> bool {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn obj_exts_offset_in_object(s: *mut kmem_cache) -> c_uint {
pub static mut offset: c_uint = 0;
    if (kmem_cache_debug_flags(s, SLAB_STORE_USER)) {
    offset += sizeof!(track) * 2;
    }
    if (slub_debug_orig_size(s)) {
    offset += sizeof!(unsigned long);
    }
    offset += kasan_metadata_size(s, false);
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn slab_set_obj_exts_in_object(slab: *mut slab) {
    slab.obj_exts_in_object = 1;
    }

#[no_mangle]
pub unsafe extern "C" fn obj_exts_offset_in_object(s: *mut kmem_cache) -> c_uint {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: slab_set_obj_exts_in_object
pub unsafe extern "C" fn slab_set_obj_exts_in_object_dup(slab: *mut slab) {
    }

//
// A no-op function used to attach kprobe handlers in slub_kunit tests.
// The barrier is needed to prevent the compiler from optimizing out callsites.
//

#[no_mangle]
unsafe extern "C" fn slab_attach_kprobe_locked() -> noinline void {
    barrier();
    }

#[no_mangle]
pub unsafe extern "C" fn slab_attach_kprobe_locked() { }

    lockdep_assert_held(lock);		
    slab_attach_kprobe_locked();	
    } while (0)

//
// For debugging context when we want to check if the struct slab pointer
// appears to be valid.
//
#[no_mangle]
pub unsafe extern "C" fn validate_slab_ptr(slab: *mut slab) -> bool {
    return PageSlab(slab_page(slab));
    }
    static unsigned long object_map[BITS_TO_LONGS(MAX_OBJS_PER_PAGE)];
pub static mut object_map_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __fill_map(obj_map: *mut c_ulong, s: *mut kmem_cache, slab: *mut slab) {
    let mut addr = slab_address(slab);
pub static mut p: *mut c_void = core::ptr::null_mut();
    bitmap_zero(obj_map, slab.objects);
    for (p = slab.freelist; p; p = get_freepointer(s, p)) {
    set_bit(__obj_to_index(s, addr, p), obj_map);
    }
    }

#[no_mangle]
unsafe extern "C" fn slab_add_kunit_errors() -> bool {
pub static mut resource: *mut c_void = core::ptr::null_mut();
    if (!kunit_get_current_test()) {
    return false;
    }
    resource = kunit_find_named_resource(current.kunit_test, "slab_errors");
    if (!resource) {
    return false;
    }
    (*resource.data)++;
    kunit_put_resource(resource);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn slab_in_kunit_test() -> bool {
pub static mut resource: *mut c_void = core::ptr::null_mut();
    if (!kunit_get_current_test()) {
    return false;
    }
    resource = kunit_find_named_resource(current.kunit_test, "slab_errors");
    if (!resource) {
    return false;
    }
    kunit_put_resource(resource);
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn slab_add_kunit_errors() -> bool { return false; }

#[no_mangle]
pub unsafe extern "C" fn size_from_object(s: *mut kmem_cache) -> c_uint {
    if (s.flags & SLAB_RED_ZONE) {
    return s.size - s.red_left_pad;
    }
    return s.size;
    }
#[no_mangle]
pub unsafe extern "C" fn restore_red_left(s: *mut kmem_cache, p: *mut c_void) -> *mut c_void {
    if (s.flags & SLAB_RED_ZONE) {
    p -= s.red_left_pad;
    }
    return p;
    }
//
// Debug settings:
//

pub static mut slub_debug: slab_flags_t = 0;

    static slab_flags_t slub_debug;

pub static mut slub_debug_string: *mut c_void = core::ptr::null_mut();
    static int disable_higher_order_debug;
//
// Object debugging
//
// Verify that a pointer has an address that is valid within a slab page
#[no_mangle]
pub unsafe extern "C" fn check_valid_pointer(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void) -> c_int {
pub static mut base: *mut c_void = core::ptr::null_mut();
    if (!object) {
    return 1;
    }
    base = slab_address(slab);
    object = kasan_reset_tag(object);
    object = restore_red_left(s, object);
    if (object < base || object >= base + slab.objects * s.size ||
    (object - base) % s.size) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn print_section(level: *mut c_char, text: *mut c_char, addr: *mut u8, length: c_uint) {
    metadata_access_enable();
    print_hex_dump(level, text, DUMP_PREFIX_ADDRESS,
    16, 1, kasan_reset_tag(addr), length, 1);
    metadata_access_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn get_track(s: *mut kmem_cache, object: *mut c_void, alloc: track_item) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = object + get_info_end(s);
    return kasan_reset_tag(p + alloc);
    }

#[no_mangle]
unsafe extern "C" fn set_track_prepare(gfp_flags: gfp_t) -> noinline depot_stack_handle_t {
    let mut handle;
    unsigned long entries[TRACK_ADDRS_COUNT];
    let mut nr_entries = 0;
    nr_entries = stack_trace_save(entries, ARRAY_SIZE!(entries), 3);
    handle = stack_depot_save(entries, nr_entries, gfp_flags);
    return handle;
    }

#[no_mangle]
pub unsafe extern "C" fn set_track_prepare(gfp_flags: gfp_t) -> depot_stack_handle_t {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn set_track_update(s: *mut kmem_cache, object: *mut c_void, alloc: track_item, addr: c_ulong, handle: depot_stack_handle_t) {
    let mut p = get_track(s, object, alloc);

    p.handle = handle;

    p.addr = addr;
    p.cpu = raw_smp_processor_id();
    p.pid = current.pid;
    p.when = jiffies;
    }
    static __always_inline void set_track(kmem_cache *s, void *object,
    enum track_item alloc, unsigned long addr, gfp_t gfp_flags)
    {
pub static mut handle: depot_stack_handle_t = 0;
    set_track_update(s, object, alloc, addr, handle);
    }
#[no_mangle]
unsafe extern "C" fn init_tracking(s: *mut kmem_cache, object: *mut c_void) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!(s.flags & SLAB_STORE_USER)) {
    return;
    }
    p = get_track(s, object, TRACK_ALLOC);
    memset(p, 0, 2*sizeof!(track));
    }
#[no_mangle]
unsafe extern "C" fn print_track(s: *const c_char, t: *mut track, pr_time: c_ulong) {
    depot_stack_handle_t handle __maybe_unused;
    if (!t.addr) {
    return;
    }
    pr_err!("%s in %pS age=%lu cpu=%u pid=%d\n",
    s, t.addr, pr_time - t.when, t.cpu, t.pid);

    handle = READ_ONCE(t.handle);
    if (handle) {
    stack_depot_print(handle);
    }
    else {
    pr_err!("object allocation/free stack trace missing\n");
    }

    }
#[no_mangle]
pub unsafe extern "C" fn print_tracking(s: *mut kmem_cache, object: *mut c_void) {
pub static mut pr_time: c_ulong = 0;
    if (!(s.flags & SLAB_STORE_USER)) {
    return;
    }
    print_track("Allocated", get_track(s, object, TRACK_ALLOC), pr_time);
    print_track("Freed", get_track(s, object, TRACK_FREE), pr_time);
    }
#[no_mangle]
unsafe extern "C" fn print_slab_info(slab: *const slab) {
    pr_err!("Slab 0x%p objects=%u used=%u fp=0x%p flags=%pGp\n",
    slab, slab.objects, slab.inuse, slab.freelist,
    &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn skip_orig_size_check(s: *mut kmem_cache, object: *const c_void) {
    set_orig_size(s, object, s.object_size);
    }
#[no_mangle]
unsafe extern "C" fn __slab_bug(s: *mut kmem_cache, fmt: *const c_char, argsp: va_list) {
pub static mut vaf: usize = 0;
    let mut args;
    va_copy(args, argsp);
    vaf.fmt = fmt;
    vaf.va = &args;
    pr_err!("=============================================================================\n");
    pr_err!("BUG %s (%s): %pV\n", s ? s.name : "<unknown>", print_tainted(), &vaf);
    pr_err!("-----------------------------------------------------------------------------\n\n");
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn slab_bug(s: *mut kmem_cache, fmt: *const c_char, ...) {
    let mut args;
    va_start(args, fmt);
    __slab_bug(s, fmt, args);
    va_end(args);
    }
    __printf(2, 3)
#[no_mangle]
unsafe extern "C" fn slab_fix(s: *mut kmem_cache, fmt: *const c_char, ...) {
pub static mut vaf: usize = 0;
    let mut args;
    if (slab_add_kunit_errors()) {
    return;
    }
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    pr_err!("FIX %s: %pV\n", s.name, &vaf);
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn print_trailer(s: *mut kmem_cache, slab: *mut slab, p: *mut u8) {
    let mut off = 0;	/* Offset of last byte */
    let mut addr = slab_address(slab);
    print_tracking(s, p);
    print_slab_info(slab);
    pr_err!("Object 0x%p @offset=%tu fp=0x%p\n\n",
    p, p - addr, get_freepointer(s, p));
    if (s.flags & SLAB_RED_ZONE) {
    print_section(KERN_ERR, "Redzone  ", p - s.red_left_pad,
    s.red_left_pad);
    }

    else if (p > addr + 16) {
    print_section(KERN_ERR, "Bytes b4 ", p - 16, 16);
    }
    print_section(KERN_ERR,         "Object   ", p,
    min_t(unsigned int, s.object_size, PAGE_SIZE));
    if (s.flags & SLAB_RED_ZONE) {
    print_section(KERN_ERR, "Redzone  ", p + s.object_size,
    s.inuse - s.object_size);
    }
    off = get_info_end(s);
    if (s.flags & SLAB_STORE_USER) {
    off += 2 * sizeof!(track);
    }
    if (slub_debug_orig_size(s)) {
    off += sizeof!(unsigned long);
    }
    off += kasan_metadata_size(s, false);
    if (obj_exts_in_object(slab)) {
    off += slab_obj_ext_size(slab);
    }
    if (off != size_from_object(s)) {
// Beginning of the filler is the free pointer
    print_section(KERN_ERR, "Padding  ", p + off,
    size_from_object(s) - off);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn object_err(s: *mut kmem_cache, slab: *mut slab, object: *mut u8, reason: *mut c_char) {
    if (slab_add_kunit_errors()) {
    return;
    }
    slab_bug(s, reason);
    if (!object || !check_valid_pointer(s, slab, object)) {
    print_slab_info(slab);
    pr_err!("Invalid pointer 0x%p\n", object);
    } else {
    print_trailer(s, slab, object);
    }
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    WARN_ON!(1);
    }
#[no_mangle]
unsafe extern "C" fn __slab_err(slab: *mut slab) {
    if (slab_in_kunit_test()) {
    return;
    }
    print_slab_info(slab);
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    WARN_ON!(1);
    }
    static __printf(3, 4) void slab_err(kmem_cache *s, slab *slab,
    const char *fmt, ...)
    {
    let mut args;
    if (slab_add_kunit_errors()) {
    return;
    }
    va_start(args, fmt);
    __slab_bug(s, fmt, args);
    va_end(args);
    __slab_err(slab);
    }
#[no_mangle]
unsafe extern "C" fn init_object(s: *mut kmem_cache, object: *mut c_void, val: u8) {
    let mut p = kasan_reset_tag(object);
pub static mut poison_size: c_uint = 0;
    if (s.flags & SLAB_RED_ZONE) {
//
// Here and below, avoid overwriting the KMSAN shadow. Keeping
// the shadow makes it possible to distinguish uninit-value
// from use-after-free.
//
    memset_no_sanitize_memory(p - s.red_left_pad, val,
    s.red_left_pad);
    if (slub_debug_orig_size(s) && val == SLUB_RED_ACTIVE) {
//
// Redzone the extra allocated space by kmalloc than
// requested, and the poison size will be limited to
// the original request size accordingly.
//
    poison_size = get_orig_size(s, object);
    }
    }
    if (s.flags & __OBJECT_POISON) {
    memset_no_sanitize_memory(p, POISON_FREE, poison_size - 1);
    memset_no_sanitize_memory(p + poison_size - 1, POISON_END, 1);
    }
    if (s.flags & SLAB_RED_ZONE) {
    memset_no_sanitize_memory(p + poison_size, val,
    s.inuse - poison_size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn restore_bytes(s: *mut kmem_cache, message: *mut c_char, data: u8, from: *mut c_void, to: *mut c_void) {
    slab_fix(s, "Restoring %s 0x%p-0x%p=0x%x", message, from, to - 1, data);
    memset(from, data, to - from);
    }

// Macro flag: #define pad_check_attributes

    static pad_check_attributes int
    check_bytes_and_report(kmem_cache *s, slab *slab,
    u8 *object, const char *what, u8 *start, unsigned int value,
    unsigned int bytes, bool slab_obj_print)
    {
pub static mut fault: *mut c_void = core::ptr::null_mut();
pub static mut end: *mut c_void = core::ptr::null_mut();
    let mut addr = slab_address(slab);
    metadata_access_enable();
    fault = memchr_inv(kasan_reset_tag(start), value, bytes);
    metadata_access_disable();
    if (!fault) {
    return 1;
    }
    end = start + bytes;
    while (end > fault && end[-1] == value) {
    end -= 1;
    }
    if (slab_add_kunit_errors()) {
// goto;
    }
    pr_err!("[%s overwritten] 0x%p-0x%p @offset=%tu. First byte 0x%x instead of 0x%x\n",
    what, fault, end - 1, fault - addr, fault[0], value);
    if (slab_obj_print) {
    object_err(s, slab, object, "Object corrupt");
    }
// label;
    restore_bytes(s, what, value, fault, end);
    return 0;
    }
//
// Object field layout:
//
// [Left redzone padding] (if SLAB_RED_ZONE)
// - Field size: s->red_left_pad
// - Immediately precedes each object when SLAB_RED_ZONE is set.
// - Filled with 0xbb (SLUB_RED_INACTIVE) for inactive objects and
// 0xcc (SLUB_RED_ACTIVE) for objects in use when SLAB_RED_ZONE.
//
// [Object bytes] (object address starts here)
// - Field size: s->object_size
// - Object payload bytes.
// - If the freepointer may overlap the object, it is stored inside
// the object (typically near the middle).
// - Poisoning uses 0x6b (POISON_FREE) and the last byte is
// 0xa5 (POISON_END) when __OBJECT_POISON is enabled.
//
// [Word-align padding] (right redzone when SLAB_RED_ZONE is set)
// - Field size: s->inuse - s->object_size
// - If redzoning is enabled and ALIGN(size, sizeof!) adds no
// padding, explicitly extend by one word so the right redzone is
// non-empty.
// - Filled with 0xbb (SLUB_RED_INACTIVE) for inactive objects and
// 0xcc (SLUB_RED_ACTIVE) for objects in use when SLAB_RED_ZONE.
//
// [Metadata starts at object + s->inuse]
// - A. freelist pointer (if freeptr_outside_object)
// - B. alloc tracking (SLAB_STORE_USER)
// - C. free tracking (SLAB_STORE_USER)
// - D. original request size (SLAB_KMALLOC && SLAB_STORE_USER)
// - E. KASAN metadata (if enabled)
//
// [Mandatory padding] (if CONFIG_SLUB_DEBUG && SLAB_RED_ZONE)
// - One mandatory debug word to guarantee a minimum poisoned gap
// between metadata and the next object, independent of alignment.
// - Filled with 0x5a (POISON_INUSE) when SLAB_POISON is set.
// [Final alignment padding]
// - Bytes added by ALIGN(size, s->align) to reach s->size.
// - When the padding is large enough, it can be used to store
// struct slabobj_ext for accounting metadata (obj_exts_in_object()).
// - The remaining bytes (if any) are filled with 0x5a (POISON_INUSE)
// when SLAB_POISON is set.
//
// Notes:
// - Redzones are filled by init_object() with SLUB_RED_ACTIVE/INACTIVE.
// - Object contents are poisoned with POISON_FREE/END when __OBJECT_POISON.
// - The trailing padding is pre-filled with POISON_INUSE by
// setup_slab_debug() when SLAB_POISON is set, and is validated by
// check_pad_bytes().
// - The first object pointer is slab_address(slab) +
// (s->red_left_pad if redzoning); subsequent objects are reached by
// adding s->size each time.
//
// If a slab cache flag relies on specific metadata to exist at a fixed
// offset, the flag must be included in SLAB_NEVER_MERGE to prevent merging.
// Otherwise, the cache would misbehave as s->object_size and s->inuse are
// adjusted during cache merging (see __kmem_cache_alias()).
//
#[no_mangle]
unsafe extern "C" fn check_pad_bytes(s: *mut kmem_cache, slab: *mut slab, p: *mut u8) -> c_int {
    let mut off = get_info_end(s);	/* The end of info */
    if (s.flags & SLAB_STORE_USER) {
// We also have user information there
    off += 2 * sizeof!(track);
    if (s.flags & SLAB_KMALLOC) {
    off += sizeof!(unsigned long);
    }
    }
    off += kasan_metadata_size(s, false);
    if (obj_exts_in_object(slab)) {
    off += slab_obj_ext_size(slab);
    }
    if (size_from_object(s) == off) {
    return 1;
    }
    return check_bytes_and_report(s, slab, p, "Object padding",
    p + off, POISON_INUSE, size_from_object(s) - off, true);
    }
// Check the pad bytes at the end of a slab page
    static pad_check_attributes void
    slab_pad_check(kmem_cache *s, slab *slab)
    {
pub static mut start: *mut c_void = core::ptr::null_mut();
pub static mut fault: *mut c_void = core::ptr::null_mut();
pub static mut end: *mut c_void = core::ptr::null_mut();
pub static mut pad: *mut c_void = core::ptr::null_mut();
    let mut length = 0;
    let mut remainder = 0;
    if (!(s.flags & SLAB_POISON)) {
    return;
    }
    start = slab_address(slab);
    length = slab_size(slab);
    end = start + length;
    if (obj_exts_in_slab(s, slab) && !obj_exts_in_object(slab)) {
    remainder = length;
    remainder -= obj_exts_offset_in_slab(s, slab);
    remainder -= obj_exts_size_in_slab(slab);
    } else {
    remainder = length % s.size;
    }
    if (!remainder) {
    return;
    }
    pad = end - remainder;
    metadata_access_enable();
    fault = memchr_inv(kasan_reset_tag(pad), POISON_INUSE, remainder);
    metadata_access_disable();
    if (!fault) {
    return;
    }
    while (end > fault && end[-1] == POISON_INUSE) {
    end -= 1;
    }
    slab_bug(s, "Padding overwritten. 0x%p-0x%p @offset=%tu",
    fault, end - 1, fault - start);
    print_section(KERN_ERR, "Padding ", pad, remainder);
    __slab_err(slab);
    restore_bytes(s, "slab padding", POISON_INUSE, fault, end);
    }
#[no_mangle]
pub unsafe extern "C" fn check_object(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, val: u8) -> c_int {
    let mut p = object;
    let mut endobject = object + s.object_size;
    let mut orig_size = 0;
    let mut kasan_meta_size = 0;
pub static mut ret: c_int = 1;
    if (s.flags & SLAB_RED_ZONE) {
    if (!check_bytes_and_report(s, slab, object, "Left Redzone",
    object - s.red_left_pad, val, s.red_left_pad, ret)) {
    ret = 0;
    }
    if (!check_bytes_and_report(s, slab, object, "Right Redzone",
    endobject, val, s.inuse - s.object_size, ret)) {
    ret = 0;
    }
    if (slub_debug_orig_size(s) && val == SLUB_RED_ACTIVE) {
    orig_size = get_orig_size(s, object);
    if (s.object_size > orig_size  &&
    !check_bytes_and_report(s, slab, object,
    "kmalloc Redzone", p + orig_size,
    val, s.object_size - orig_size, ret)) {
    ret = 0;
    }
    }
    } else {
    if ((s.flags & SLAB_POISON) && s.object_size < s.inuse) {
    if (!check_bytes_and_report(s, slab, p, "Alignment padding",
    endobject, POISON_INUSE,
    s.inuse - s.object_size, ret)) {
    ret = 0;
    }
    }
    }
    if (s.flags & SLAB_POISON) {
    if (val != SLUB_RED_ACTIVE && (s.flags & __OBJECT_POISON)) {
//
// KASAN can save its free meta data inside of the
// object at offset 0. Thus, skip checking the part of
// the redzone that overlaps with the meta data.
//
    kasan_meta_size = kasan_metadata_size(s, true);
    if (kasan_meta_size < s.object_size - 1 &&
    !check_bytes_and_report(s, slab, p, "Poison",
    p + kasan_meta_size, POISON_FREE,
    s.object_size - kasan_meta_size - 1, ret)) {
    ret = 0;
    }
    if (kasan_meta_size < s.object_size &&
    !check_bytes_and_report(s, slab, p, "End Poison",
    p + s.object_size - 1, POISON_END, 1, ret)) {
    ret = 0;
    }
    }
//
// check_pad_bytes cleans up on its own.
//
    if (!check_pad_bytes(s, slab, p)) {
    ret = 0;
    }
    }
//
// Cannot check freepointer while object is allocated if
// object and freepointer overlap.
//
    if ((freeptr_outside_object(s) || val != SLUB_RED_ACTIVE) &&
    !check_valid_pointer(s, slab, get_freepointer(s, p))) {
    object_err(s, slab, p, "Freepointer corrupt");
//
// No choice but to zap it and thus lose the remainder
// of the free objects in this slab. May cause
// another error because the object count is now wrong.
//
    set_freepointer(s, p, core::ptr::null_mut());
    ret = 0;
    }
    return ret;
    }
//
// Checks if the slab state looks sane. Assumes the struct slab pointer
// was either obtained in a way that ensures it's valid, or validated
// by validate_slab_ptr()
//
#[no_mangle]
unsafe extern "C" fn check_slab(s: *mut kmem_cache, slab: *mut slab) -> c_int {
    let mut maxobj = 0;
    maxobj = order_objects(slab_order(slab), s.size);
    if (slab.objects > maxobj) {
    slab_err(s, slab, "objects %u > max %u",
    slab.objects, maxobj);
    return 0;
    }
    if (slab.inuse > slab.objects) {
    slab_err(s, slab, "inuse %u > max %u",
    slab.inuse, slab.objects);
    return 0;
    }
    if (slab.frozen) {
    slab_err(s, slab, "Slab disabled since SLUB metadata consistency check failed");
    return 0;
    }
// Slab_pad_check fixes things up after itself
    slab_pad_check(s, slab);
    return 1;
    }
//
// Determine if a certain object in a slab is on the freelist. Must hold the
// slab lock to guarantee that the chains are in a consistent state.
//
#[no_mangle]
unsafe extern "C" fn on_freelist(s: *mut kmem_cache, slab: *mut slab, search: *mut c_void) -> bool {
pub static mut nr: c_int = 0;
pub static mut fp: *mut c_void = core::ptr::null_mut();
    let mut object = core::ptr::null_mut();
    let mut max_objects = 0;
    fp = slab.freelist;
    while (fp && nr <= slab.objects) {
    if (fp == search) {
    return true;
    }
    if (!check_valid_pointer(s, slab, fp)) {
    if (object) {
    object_err(s, slab, object,
    "Freechain corrupt");
    set_freepointer(s, object, core::ptr::null_mut());
    break;
    } else {
    slab_err(s, slab, "Freepointer corrupt");
    slab.freelist = core::ptr::null_mut();
    slab.inuse = slab.objects;
    slab_fix(s, "Freelist cleared");
    return false;
    }
    }
    object = fp;
    fp = get_freepointer(s, object);
    nr += 1;
    }
    if (nr > slab.objects) {
    slab_err(s, slab, "Freelist cycle detected");
    slab.freelist = core::ptr::null_mut();
    slab.inuse = slab.objects;
    slab_fix(s, "Freelist cleared");
    return false;
    }
    max_objects = order_objects(slab_order(slab), s.size);
    if (max_objects > MAX_OBJS_PER_PAGE) {
    max_objects = MAX_OBJS_PER_PAGE;
    }
    if (slab.objects != max_objects) {
    slab_err(s, slab, "Wrong number of objects. Found %d but should be %d",
    slab.objects, max_objects);
    slab.objects = max_objects;
    slab_fix(s, "Number of objects adjusted");
    }
    if (slab.inuse != slab.objects - nr) {
    slab_err(s, slab, "Wrong object count. Counter is %d but counted were %d",
    slab.inuse, slab.objects - nr);
    slab.inuse = slab.objects - nr;
    slab_fix(s, "Object count adjusted");
    }
pub static mut search: return = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, alloc: c_int) {
    if (s.flags & SLAB_TRACE) {
    pr_info!("TRACE %s %s 0x%p inuse=%d fp=0x%p\n",
    s.name,
    alloc ? "alloc" : "free",
    object, slab.inuse,
    slab.freelist);
    if (!alloc) {
    print_section(KERN_INFO, "Object ", object,
    s.object_size);
    }
    dump_stack();
    }
    }
//
// Tracking of fully allocated slabs for debugging purposes.
//
#[no_mangle]
pub unsafe extern "C" fn add_full(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {
    if (!(s.flags & SLAB_STORE_USER)) {
    return;
    }
    slab_lockdep_assert_held(&n.list_lock);
    list_add(&slab.slab_list, &n.full);
    }
#[no_mangle]
unsafe extern "C" fn remove_full(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {
    if (!(s.flags & SLAB_STORE_USER)) {
    return;
    }
    slab_lockdep_assert_held(&n.list_lock);
    list_del(&slab.slab_list);
    }
#[no_mangle]
pub unsafe extern "C" fn node_nr_slabs(n: *mut kmem_cache_node) -> c_ulong {
    return atomic_long_read(&n.nr_slabs);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_slabs_node(s: *mut kmem_cache, node: c_int, objects: c_int) {
    let mut n = get_node(s, node);
    atomic_long_inc(&n.nr_slabs);
    atomic_long_add(objects, &n.total_objects);
    }
#[no_mangle]
pub unsafe extern "C" fn dec_slabs_node(s: *mut kmem_cache, node: c_int, objects: c_int) {
    let mut n = get_node(s, node);
    atomic_long_dec(&n.nr_slabs);
    atomic_long_sub(objects, &n.total_objects);
    }
// Object debug checks for alloc/free paths
#[no_mangle]
unsafe extern "C" fn setup_object_debug(s: *mut kmem_cache, object: *mut c_void) {
    if (!kmem_cache_debug_flags(s, SLAB_STORE_USER|SLAB_RED_ZONE|__OBJECT_POISON)) {
    return;
    }
    init_object(s, object, SLUB_RED_INACTIVE);
    init_tracking(s, object);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn setup_slab_debug(s: *mut kmem_cache, slab: *mut slab, addr: *mut c_void) {
    if (!kmem_cache_debug_flags(s, SLAB_POISON)) {
    return;
    }
    metadata_access_enable();
    memset(kasan_reset_tag(addr), POISON_INUSE, slab_size(slab));
    metadata_access_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_consistency_checks(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void) -> c_int {
    if (!check_slab(s, slab)) {
    return 0;
    }
    if (!check_valid_pointer(s, slab, object)) {
    object_err(s, slab, object, "Freelist Pointer check fails");
    return 0;
    }
    if (!check_object(s, slab, object, SLUB_RED_INACTIVE)) {
    return 0;
    }
    return 1;
    }
    static noinline bool alloc_debug_processing(kmem_cache *s, slab *slab, void *object, int orig_size)
    {
    if (s.flags & SLAB_CONSISTENCY_CHECKS) {
    if (!alloc_consistency_checks(s, slab, object)) {
// goto;
    }
    }
// Success. Perform special debug activities for allocs
    trace(s, slab, object, 1);
    set_orig_size(s, object, orig_size);
    init_object(s, object, SLUB_RED_ACTIVE);
    return true;
// label;
//
// Let's do the best we can to avoid issues in the future. Marking all
// objects as used avoids touching the remaining objects.
//
    slab_fix(s, "Marking all objects used");
    slab.inuse = slab.objects;
    slab.freelist = core::ptr::null_mut();
    slab.frozen = 1; /* mark consistency-failed slab as frozen */
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn free_consistency_checks(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, addr: c_ulong) -> c_int {
    if (!check_valid_pointer(s, slab, object)) {
    slab_err(s, slab, "Invalid object pointer 0x%p", object);
    return 0;
    }
    if (on_freelist(s, slab, object)) {
    object_err(s, slab, object, "Object already free");
    return 0;
    }
    if (!check_object(s, slab, object, SLUB_RED_ACTIVE)) {
    return 0;
    }
    if (unlikely(s != slab.slab_cache)) {
    if (!slab.slab_cache) {
    slab_err(core::ptr::null_mut(), slab, "No slab cache for object 0x%p",
    object);
    } else {
    object_err(s, slab, object,
    "page slab pointer corrupt.");
    }
    return 0;
    }
    return 1;
    }
//
// Parse a block of slab_debug options. Blocks are delimited by ';'
//
// @str:    start of block
// @flags:  returns parsed flags, or DEBUG_DEFAULT_FLAGS if none specified
// @slabs:  return start of list of slabs, or NULL when there's no list
// @init:   assume this is initial parsing and not per-kmem-create parsing
//
// returns the start of next block if there's any, or NULL
//
    static const char *
    parse_slub_debug_flags(const char *str, slab_flags_t *flags, const char **slabs, bool init)
    {
pub static mut higher_order_disable: bool = false;
// Skip any completely empty blocks
    while (*str && *str == ';') {
    str += 1;
    }
    if (*str == ',') {
//
// No options but restriction on slabs. This means full
// debugging for slabs matching a pattern.
//
// flags = DEBUG_DEFAULT_FLAGS;
// goto;
    }
// flags = 0;
// Determine which debug features should be switched on
    while (*str && *str != ',' && *str != ') {
    switch (tolower(*str)) {
    case '-':
// flags = 0;
    break;
    case 'f':
// flags |= SLAB_CONSISTENCY_CHECKS;
    break;
    case 'z':
// flags |= SLAB_RED_ZONE;
    break;
    case 'p':
// flags |= SLAB_POISON;
    break;
    case 'u':
// flags |= SLAB_STORE_USER;
    break;
    case 't':
// flags |= SLAB_TRACE;
    break;
    case 'a':
// flags |= SLAB_FAILSLAB;
    break;
    case 'o':
//
// Avoid enabling debugging on caches if its minimum
// order would increase as a result.
//
    higher_order_disable = true;
    break;
// label;
    if (init) {
    pr_err!("slab_debug option '%c' unknown. skipped\n", *str);
    }
    }
    }
// label;
    if (*str == ',') {
// slabs = str += 1;
    }
    else {
// slabs = NULL;
    }
// Skip over the slab list
    while (*str && *str != ';') {
    str += 1;
    }
// Skip any completely empty blocks
    while (*str && *str == ';') {
    str += 1;
    }
    if (init && higher_order_disable) {
    disable_higher_order_debug = 1;
    }
    if (*str) {
    return str;
    }
    else {
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_slub_debug(str: *const c_char, kp: *const kernel_param) -> c_int {
    let mut flags;
    let mut global_flags;
pub static mut saved_str: *mut c_void = core::ptr::null_mut();
pub static mut slab_list: *mut c_void = core::ptr::null_mut();
pub static mut global_slub_debug_changed: bool = false;
pub static mut slab_list_specified: bool = false;
    global_flags = DEBUG_DEFAULT_FLAGS;
    if (!str || !*str) {
//
// No options specified. Switch on full debugging.
//
// goto;
    }
    saved_str = str;
    while (str) {
    str = parse_slub_debug_flags(str, &flags, &slab_list, true);
    if (!slab_list) {
    global_flags = flags;
    global_slub_debug_changed = true;
    } else {
    slab_list_specified = true;
    if (flags & SLAB_STORE_USER) {
    stack_depot_request_early_init();
    }
    }
    }
//
// For backwards compatibility, a single list of flags with list of
// slabs means debugging is only changed for those slabs, so the global
// slab_debug should be unchanged (0 or DEBUG_DEFAULT_FLAGS, depending
// on CONFIG_SLUB_DEBUG_ON). We can extended that to multiple lists as
// long as there is no option specifying flags without a slab list.
//
    if (slab_list_specified) {
    if (!global_slub_debug_changed) {
    global_flags = slub_debug;
    }
    slub_debug_string = saved_str;
    }
// label;
    slub_debug = global_flags;
    if (slub_debug & SLAB_STORE_USER) {
    stack_depot_request_early_init();
    }
    if (slub_debug != 0 || slub_debug_string) {
    static_branch_enable(&slub_debug_enabled);
    }
    else {
    static_branch_disable(&slub_debug_enabled);
    }
    if ((static_branch_unlikely(&init_on_alloc) ||
    static_branch_unlikely(&init_on_free)) &&
    (slub_debug & SLAB_POISON)) {
    pr_info!("mem auto-init: SLAB_POISON will take precedence over init_on_alloc/init_on_free\n");
    }
    return 0;
    }
    static const struct kernel_param_ops param_ops_slab_debug __initconst = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = setup_slub_debug,
    };
    __core_param_cb(slab_debug, &param_ops_slab_debug, core::ptr::null_mut(), 0);
    __core_param_cb(slub_debug, &param_ops_slab_debug, core::ptr::null_mut(), 0);
//
// kmem_cache_flags - apply debugging options to the cache
// @flags:		flags to set
// @name:		name of the cache
//
// Debug option(s) are applied to @flags. In addition to the debug
// option(s), if a slab name (or multiple) is specified i.e.
// slab_debug=<Debug-Options>,<slab name1>,<slab name2> ...
// then only the select slabs will receive the debug option(s).
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_flags(flags: slab_flags_t, name: *const c_char) -> slab_flags_t {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
pub static mut next_block: *mut c_void = core::ptr::null_mut();
    let mut block_flags;
pub static mut slub_debug_local: slab_flags_t = 0;
    if (flags & SLAB_NO_USER_FLAGS) {
    return flags;
    }
//
// If the slab cache is for debugging (e.g. kmemleak) then
// don't store user (stack trace) information by default,
// but let the user enable it via the command line below.
//
    if (flags & SLAB_NOLEAKTRACE) {
    slub_debug_local &= ~SLAB_STORE_USER;
    }
    len = strlen(name);
    next_block = slub_debug_string;
// Go through all blocks of debug options, see if any matches our slab's name
    while (next_block) {
    next_block = parse_slub_debug_flags(next_block, &block_flags, &iter, false);
    if (!iter) {
    continue;
    }
// Found a block that has a slab list, search it
    while (*iter) {
    let mut end = core::ptr::null_mut();
    let mut glob = core::ptr::null_mut();
    let mut cmplen = 0;
    end = strchrnul(iter, ',');
    if (next_block && next_block < end) {
    end = next_block - 1;
    }
    glob = strnchr(iter, end - iter, '*');
    if (glob) {
    cmplen = glob - iter;
    }
    else {
    cmplen = max_t(size_t, len, (end - iter));
    }
    if (!strncmp(name, iter, cmplen)) {
    flags |= block_flags;
    return flags;
    }
    if (!*end || *end == ';') {
    break;
    }
    iter = end + 1;
    }
    }
    return flags | slub_debug_local;
    }

#[no_mangle]
pub unsafe extern "C" fn setup_object_debug(s: *mut kmem_cache, object: *mut c_void) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: setup_slab_debug
pub unsafe extern "C" fn setup_slab_debug_dup(s: *mut kmem_cache, slab: *mut slab, addr: *mut c_void) {}
#[no_mangle]
pub unsafe extern "C" fn alloc_debug_processing(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, orig_size: c_int) -> bool { return true; }
#[no_mangle]
pub unsafe extern "C" fn free_debug_processing(s: *mut kmem_cache, slab: *mut slab, head: *mut c_void, tail: *mut c_void, bulk_cnt: *mut c_int, addr: c_ulong, handle: depot_stack_handle_t) -> bool { return true; }
#[no_mangle]
pub unsafe extern "C" fn slab_pad_check(s: *mut kmem_cache, slab: *mut slab) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: check_object
pub unsafe extern "C" fn check_object_dup(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, val: u8) -> c_int { return 1; }
    static inline depot_stack_handle_t set_track_prepare(gfp_t gfp_flags) { return 0; }
#[no_mangle]
pub unsafe extern "C" fn set_track(s: *mut kmem_cache, object: *mut c_void, alloc: track_item, addr: c_ulong, gfp_flags: gfp_t) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: add_full
pub unsafe extern "C" fn add_full_dup(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {}
#[no_mangle]
pub unsafe extern "C" fn remove_full(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: kmem_cache_flags
pub unsafe extern "C" fn kmem_cache_flags_dup(flags: slab_flags_t, name: *const c_char) -> slab_flags_t {
    return flags;
    }
pub const slub_debug: c_int = 0;
pub const disable_higher_order_debug: c_int = 0;
#[no_mangle]
#[no_mangle]
// duplicate fn: node_nr_slabs
pub unsafe extern "C" fn node_nr_slabs_dup(n: *mut kmem_cache_node) -> c_ulong { return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: inc_slabs_node
pub unsafe extern "C" fn inc_slabs_node_dup(s: *mut kmem_cache, node: c_int, objects: c_int) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: dec_slabs_node
pub unsafe extern "C" fn dec_slabs_node_dup(s: *mut kmem_cache, node: c_int, objects: c_int) {}

//
// The allocated objcg pointers array or sheaf is not accounted directly.
// Moreover, it should not come from DMA buffer and is not readily
// reclaimable. Node restriction for the parent allocation also should
// not apply to the slab's internal objects, as well as __GFP_COMP used
// for new slab allocations.
// So those GFP bits should be masked off.
//

    __GFP_ACCOUNT | __GFP_NOFAIL | 
    __GFP_THISNODE | __GFP_COMP)

#[no_mangle]
pub unsafe extern "C" fn mark_obj_codetag_empty(obj: *const c_void) {
pub static mut obj_slab: *mut c_void = core::ptr::null_mut();
    let mut slab_exts = 0;
    if (!slab_obj_ext_has_codetag()) {
    return;
    }
    obj_slab = virt_to_slab(obj);
    slab_exts = slab_obj_exts(obj_slab);
    if (slab_exts) {
pub static mut ext: *mut c_void = core::ptr::null_mut();
    union codetag_ref *ref;
    get_slab_obj_exts(slab_exts);
    ext = slab_obj_ext(obj_slab.slab_cache, obj_slab, slab_exts, obj);
    ref = slab_obj_ext_codetag_ref(obj_slab, ext);
    if (unlikely(is_codetag_empty(ref))) {
    put_slab_obj_exts(slab_exts);
    return;
    }
// codetag should be NULL here
    WARN_ON!(ref.ct);
    set_codetag_empty(ref);
    put_slab_obj_exts(slab_exts);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mark_failed_objexts_alloc(slab: *mut slab) -> bool {
    return cmpxchg(&slab.obj_exts, 0, OBJEXTS_ALLOC_FAIL) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_failed_objexts_alloc(slab: *mut slab, obj_exts: c_ulong, vec: *mut slabobj_ext) {
    let mut stride = 0;
    if (!slab_obj_ext_has_codetag()) {
    return;
    }
//
// If vector previously failed to allocate then we have live
// objects with no tag reference. Mark all references in this
// vector as empty to avoid warnings later on.
//
    if (obj_exts != OBJEXTS_ALLOC_FAIL) {
    return;
    }
    stride = slab_obj_ext_size(slab) / sizeof!(*vec);
    while (i < slab.objects) {
    union codetag_ref *ref = slab_obj_ext_codetag_ref(slab, vec);
    set_codetag_empty(ref);
    vec += stride;
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mark_obj_codetag_empty
pub unsafe extern "C" fn mark_obj_codetag_empty_dup(obj: *mut c_void) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: mark_failed_objexts_alloc
pub unsafe extern "C" fn mark_failed_objexts_alloc_dup(slab: *mut slab) -> bool { return false; }
#[no_mangle]
#[no_mangle]
// duplicate fn: handle_failed_objexts_alloc
pub unsafe extern "C" fn handle_failed_objexts_alloc_dup(slab: *mut slab, obj_exts: c_ulong, vec: *mut slabobj_ext) {}

#[no_mangle]
pub unsafe extern "C" fn init_slab_obj_exts(slab: *mut slab) {
    slab.obj_exts = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_slab_obj_exts(slab: *mut slab, s: *mut kmem_cache, gfp: gfp_t, alloc_flags: c_uint) -> c_int {
pub static mut allow_spin: bool = false;
pub static mut new_slab: bool = false;
    let mut new_exts = 0;
    let mut old_exts = 0;
pub static mut vec: *mut c_void = core::ptr::null_mut();
pub static mut sz: usize = 0;
    gfp &= ~OBJCGS_CLEAR_MASK;
//
// In most cases, obj_exts arrays are allocated from normal kmalloc.
// However, normal kmalloc caches must allocate them from
// KMALLOC_NO_OBJ_EXT caches to prevent recursion.
//
    if (is_kmalloc_normal(s)) {
    alloc_flags |= SLAB_ALLOC_NO_OBJ_EXT;
    }
    alloc_flags &= ~SLAB_ALLOC_NEW_SLAB;
// This will use kmalloc_nolock() if alloc_flags say so
    vec = kmalloc_flags(sz, gfp | __GFP_ZERO, alloc_flags, slab_nid(slab));
    if (!vec) {
//
// Try to mark vectors which failed to allocate.
// If this operation fails, there may be a racing process
// that has already completed the allocation.
//
    if (!mark_failed_objexts_alloc(slab) &&
    slab_obj_exts(slab)) {
    return 0;
    }
    return -ENOMEM;
    }
    if (IS_ENABLED!(CONFIG_DEBUG_VM)) {
pub static mut exts_cache: *mut c_void = core::ptr::null_mut();
pub static mut exts_slab: *mut c_void = core::ptr::null_mut();
    exts_slab = virt_to_slab(vec);
    if (exts_slab) {
//
// The vector must be allocated from either normal or
// KMALLOC_NO_OBJ_EXT kmalloc caches to avoid cycles.
//
    exts_cache = exts_slab.slab_cache;
    WARN_ON_ONCE!(!is_kmalloc_normal(exts_cache) &&
    !(exts_cache.flags & SLAB_NO_OBJ_EXT));
    }
    }
    new_exts = (unsigned long)vec;

    new_exts |= MEMCG_DATA_OBJEXTS;

// label;
    old_exts = READ_ONCE(slab.obj_exts);
    handle_failed_objexts_alloc(slab, old_exts, vec);
    if (new_slab) {
//
// If the slab is brand new and nobody can yet access its
// obj_exts, no synchronization is required and obj_exts can
// be simply assigned.
//
    slab.obj_exts = new_exts;
    } else if (old_exts & ~OBJEXTS_FLAGS_MASK) {
//
// If the slab is already in use, somebody can allocate and
// assign slabobj_exts in parallel. In this case the existing
// objcg vector should be reused.
//
    if (unlikely(!allow_spin)) {
    kfree_nolock(vec);
    }
    else {
    kfree(vec);
    }
    return 0;
    } else if (cmpxchg(&slab.obj_exts, old_exts, new_exts) != old_exts) {
// Retry if a racing thread changed slab->obj_exts from under us.
// goto;
    }
    if (allow_spin) {
    kmemleak_not_leak(vec);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_slab_obj_exts(slab: *mut slab, allow_spin: bool) {
pub static mut obj_exts: *mut c_void = core::ptr::null_mut();
    obj_exts = slab_obj_exts(slab);
    if (!obj_exts) {
//
// If obj_exts allocation failed, slab->obj_exts is set to
// OBJEXTS_ALLOC_FAIL. In this case, we end up here and should
// clear the flag.
//
    slab.obj_exts = 0;
    return;
    }
    if (obj_exts_in_slab(slab.slab_cache, slab)) {
    slab.obj_exts = 0;
    return;
    }
    if (allow_spin) {
    kfree(obj_exts);
    }
    else {
    kfree_nolock(obj_exts);
    }
    slab.obj_exts = 0;
    }
//
// Try to allocate slabobj_ext array from unused space.
// This function must be called on a freshly allocated slab to prevent
// concurrency problems.
//
#[no_mangle]
unsafe extern "C" fn alloc_slab_obj_exts_early(s: *mut kmem_cache, slab: *mut slab) {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut obj_exts = 0;
    if (!need_slab_obj_exts(s)) {
    return;
    }
    if (obj_exts_fit_within_slab_leftover(s, slab)) {
    addr = slab_address(slab) + obj_exts_offset_in_slab(s, slab);
    addr = kasan_reset_tag(addr);
    obj_exts = (unsigned long)addr;
    get_slab_obj_exts(obj_exts);
    memset(addr, 0, obj_exts_size_in_slab(slab));
    put_slab_obj_exts(obj_exts);

    obj_exts |= MEMCG_DATA_OBJEXTS;

    slab.obj_exts = obj_exts;
    } else if (s.flags & SLAB_OBJ_EXT_IN_OBJ) {
pub static mut offset: c_uint = 0;
    obj_exts = (unsigned long)slab_address(slab);
    obj_exts += s.red_left_pad;
    obj_exts += offset;
    get_slab_obj_exts(obj_exts);
    for_each_object(addr, s, slab_address(slab), slab.objects) {
    memset(kasan_reset_tag(addr) + offset, 0, slab_obj_ext_size(slab));
    }
    put_slab_obj_exts(obj_exts);

    obj_exts |= MEMCG_DATA_OBJEXTS;

    slab.obj_exts = obj_exts;
    slab_set_obj_exts_in_object(slab);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mark_obj_codetag_empty
pub unsafe extern "C" fn mark_obj_codetag_empty_dup(obj: *const c_void) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: init_slab_obj_exts
pub unsafe extern "C" fn init_slab_obj_exts_dup(slab: *mut slab) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_slab_obj_exts
pub unsafe extern "C" fn alloc_slab_obj_exts_dup(slab: *mut slab, s: *mut kmem_cache, gfp: gfp_t, alloc_flags: c_uint) -> c_int {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: free_slab_obj_exts
pub unsafe extern "C" fn free_slab_obj_exts_dup(slab: *mut slab, allow_spin: bool) {
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_slab_obj_exts_early(s: *mut kmem_cache, slab: *mut slab) {
    }

#[no_mangle]
pub unsafe extern "C" fn prepare_slab_obj_exts_hook(s: *mut kmem_cache, slab: *mut slab, flags: gfp_t, alloc_flags: c_uint, p: *mut c_void) -> c_ulong {
    if (!slab_obj_exts(slab)) {
    if (is_kfence_address(p)) {
    return 0;
    }
    if (alloc_slab_obj_exts(slab, s, flags, alloc_flags)) {
    pr_warn_once("%s, %s: Failed to create slab extension vector!\n",
    __func__, s.name);
    return 0;
    }
    }
    return slab_obj_exts(slab);
    }
// Should be called only if mem_alloc_profiling_enabled()
    static noinline void
    __alloc_tagging_slab_alloc_hook(kmem_cache *s, void *object, gfp_t flags,
    unsigned int alloc_flags)
    {
    let mut obj_exts = 0;
pub static mut obj_ext: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (!object) {
    return;
    }
    if (s.flags & (SLAB_NO_OBJ_EXT | SLAB_NOLEAKTRACE)) {
    return;
    }
    if (alloc_flags & SLAB_ALLOC_NO_RECURSE) {
    return;
    }
    slab = virt_to_slab(object);
    obj_exts = prepare_slab_obj_exts_hook(s, slab, flags, alloc_flags, object);
//
// Currently obj_exts is used only for allocation profiling.
// If other users appear then mem_alloc_profiling_enabled()
// check should be added before alloc_tag_add().
//
    if (obj_exts) {
    union codetag_ref *ref;
    get_slab_obj_exts(obj_exts);
    obj_ext = slab_obj_ext(s, slab, obj_exts, object);
    ref = slab_obj_ext_codetag_ref(slab, obj_ext);
    alloc_tag_add(ref, current.alloc_tag, s.size);
    put_slab_obj_exts(obj_exts);
    } else {
//
// KFENCE allocations are rare and the amount of outstanding
// ones is limited to a small number so it's not worth setting
// tags as inaccurate because of them.
//
    if (!is_kfence_address(object)) {
    alloc_tag_set_inaccurate(current.alloc_tag);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_tagging_slab_alloc_hook(s: *mut kmem_cache, object: *mut c_void, flags: gfp_t, alloc_flags: c_uint) {
    if (mem_alloc_profiling_enabled()) {
    __alloc_tagging_slab_alloc_hook(s, object, flags, alloc_flags);
    }
    }
// Should be called only if mem_alloc_profiling_enabled()
    static noinline void
    __alloc_tagging_slab_free_hook(kmem_cache *s, slab *slab, void **p,
    int objects)
    {
    let mut obj_exts = 0;
// slab->obj_exts might not be NULL if it was created for MEMCG accounting.
    if (s.flags & (SLAB_NO_OBJ_EXT | SLAB_NOLEAKTRACE)) {
    return;
    }
    obj_exts = slab_obj_exts(slab);
    if (!obj_exts) {
    return;
    }
    get_slab_obj_exts(obj_exts);
    while (i < objects) {
pub static mut ext: *mut c_void = core::ptr::null_mut();
    ext = slab_obj_ext(s, slab, obj_exts, p[i]);
    alloc_tag_sub(slab_obj_ext_codetag_ref(slab, ext), s.size);
    }
    put_slab_obj_exts(obj_exts);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_tagging_slab_free_hook(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut c_void, objects: c_int) {
    if (mem_alloc_profiling_enabled()) {
    __alloc_tagging_slab_free_hook(s, slab, p, objects);
    }
    }
//
// Make sure the static key used by slab_obj_ext_has_codetag() reflects the
// value of !mem_alloc_profiling_permanently_disabled()
//
// Any later mem alloc profiling shutdown won't be reflected in the static key
// because obj_exts with codetags might already exist.
//
#[no_mangle]
unsafe extern "C" fn slab_obj_ext_has_codetag_init()  {
pub static mut need_codetag: bool = false;
    if (need_codetag != static_key_enabled(&slab_obj_ext_has_codetag_key)) {
    if (need_codetag) {
    static_branch_enable(&slab_obj_ext_has_codetag_key);
    }
    else {
    static_branch_disable(&slab_obj_ext_has_codetag_key);
    }
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_tagging_slab_alloc_hook
pub unsafe extern "C" fn alloc_tagging_slab_alloc_hook_dup(s: *mut kmem_cache, object: *mut c_void, flags: gfp_t, alloc_flags: c_uint) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_tagging_slab_free_hook
pub unsafe extern "C" fn alloc_tagging_slab_free_hook_dup(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut c_void, objects: c_int) {
    }
#[no_mangle]
pub unsafe extern "C" fn slab_obj_ext_has_codetag_init() {
    }

// forward_decl: memcg_alloc_abort_single;
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn memcg_slab_post_alloc_hook(s: *mut kmem_cache, flags: gfp_t, size: size_t, p: *mut *mut c_void, ac: *mut slab_alloc_context) -> bool {
    if (likely(!memcg_kmem_online())) {
    return true;
    }
    if (likely(!(flags & __GFP_ACCOUNT) && !(s.flags & SLAB_ACCOUNT))) {
    return true;
    }
    if (likely(__memcg_slab_post_alloc_hook(s, ac.lru, flags,
    ac.alloc_flags, size, p))) {
    return true;
    }
    if (likely(size == 1)) {
    memcg_alloc_abort_single(s, *p);
// p = NULL;
    } else {
    kmem_cache_free_bulk(s, size, p);
    }
    return false;
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn memcg_slab_free_hook(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut c_void, objects: c_int) {
    let mut obj_exts = 0;
    if (!memcg_kmem_online()) {
    return;
    }
    obj_exts = slab_obj_exts(slab);
    if (likely(!obj_exts)) {
    return;
    }
    if (!slab_needs_objcg(slab)) {
    return;
    }
    get_slab_obj_exts(obj_exts);
    __memcg_slab_free_hook(s, slab, p, objects, obj_exts);
    put_slab_obj_exts(obj_exts);
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn memcg_slab_post_charge(p: *mut c_void, flags: gfp_t) -> bool {
    let mut obj_exts = 0;
pub static mut obj_ext: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    page = virt_to_page(p);
    if (PageLargeKmalloc(page)) {
    let mut order = 0;
    let mut size = 0;
    if (PageMemcgKmem(page)) {
    return true;
    }
    order = large_kmalloc_order(page);
    if (__memcg_kmem_charge_page(page, flags, order)) {
    return false;
    }
//
// This page has already been accounted in the global stats but
// not in the memcg stats. So, subtract from the global and use
// the interface which adds to both global and memcg stats.
//
    size = PAGE_SIZE << order;
    mod_node_page_state(page_pgdat(page), NR_SLAB_UNRECLAIMABLE_B, -size);
    mod_lruvec_page_state(page, NR_SLAB_UNRECLAIMABLE_B, size);
    return true;
    }
    slab = page_slab(page);
    s = slab.slab_cache;
//
// Ignore KMALLOC_NORMAL cache to avoid possible circular dependency
// of slab_obj_exts being allocated from the same slab and thus the slab
// becoming effectively unfreeable.
//
    if (!cache_needs_objcg(s)) {
    return true;
    }
// Ignore already charged objects.
    obj_exts = slab_obj_exts(slab);
    if (obj_exts) {
    get_slab_obj_exts(obj_exts);
    obj_ext = slab_obj_ext(s, slab, obj_exts, p);
    if (unlikely(slab_obj_ext_objcg(slab, obj_ext))) {
    put_slab_obj_exts(obj_exts);
    return true;
    }
    put_slab_obj_exts(obj_exts);
    }
    return __memcg_slab_post_alloc_hook(s, core::ptr::null_mut(), flags, SLAB_ALLOC_DEFAULT,
    1, &p);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: memcg_slab_post_alloc_hook
pub unsafe extern "C" fn memcg_slab_post_alloc_hook_dup(s: *mut kmem_cache, flags: gfp_t, size: size_t, p: *mut *mut c_void, ac: *mut slab_alloc_context) -> bool {
    return true;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: memcg_slab_free_hook
pub unsafe extern "C" fn memcg_slab_free_hook_dup(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut c_void, objects: c_int) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: memcg_slab_post_charge
pub unsafe extern "C" fn memcg_slab_post_charge_dup(p: *mut c_void, flags: gfp_t) -> bool {
    return true;
    }

// forward_decl: slab_free_after_rcu_debug;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_delayed_free {
    pub head: rcu_head,
    pub object: *mut c_void,
}

//
// Hooks for other subsystems that check memory allocations. In a typical
// production configuration these hooks all should produce no code at all.
//
// Returns true if freeing of the object can proceed, false if its reuse
// was delayed by CONFIG_SLUB_RCU_DEBUG or KASAN quarantine, or it was returned
// to KFENCE.
//
// For objects allocated via kmalloc_nolock(), only a subset of alloc hooks
// are invoked, so some free hooks must handle asymmetric hook calls.
//
// Alloc hooks called for kmalloc_nolock():
// - kmsan_slab_alloc()
// - kasan_slab_alloc()
// - memcg_slab_post_alloc_hook()
// - alloc_tagging_slab_alloc_hook()
//
// Free hooks that must handle missing corresponding alloc hooks:
// - kmemleak_free_recursive()
// - kfence_free()
//
// Free hooks that have no alloc hook counterpart, and thus safe to call:
// - debug_check_no_locks_freed()
// - debug_check_no_obj_freed()
// - __kcsan_check_access()
//
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn slab_free_hook(s: *mut kmem_cache, x: *mut c_void, init: bool, after_rcu_delay: bool) -> bool {
// Are the object contents still accessible?
pub static mut still_accessible: bool = false;
    kmemleak_free_recursive(x, s.flags);
    kmsan_slab_free(s, x);
    debug_check_no_locks_freed(x, s.object_size);
    if (!(s.flags & SLAB_DEBUG_OBJECTS)) {
    debug_check_no_obj_freed(x, s.object_size);
    }
// Use KCSAN to help debug racy use-after-free.
    if (!still_accessible) {
    __kcsan_check_access(x, s.object_size,
    KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ASSERT);
    }
    if (kfence_free(x)) {
    return false;
    }
//
// Give KASAN a chance to notice an invalid free operation before we
// modify the object.
//
    if (kasan_slab_pre_free(s, x)) {
    return false;
    }

    if (still_accessible) {
pub static mut delayed_free: *mut c_void = core::ptr::null_mut();
    delayed_free = kmalloc_obj(*delayed_free, GFP_NOWAIT);
    if (delayed_free) {
//
// Let KASAN track our call stack as a "related work
// creation", just like if the object had been freed
// normally via kfree_rcu().
// We have to do this manually because the rcu_head is
// not located inside the object.
//
    kasan_record_aux_stack(x);
    delayed_free.object = x;
    call_rcu(&delayed_free.head, slab_free_after_rcu_debug);
    return false;
    }
    }

//
// As memory initialization might be integrated into KASAN,
// kasan_slab_free and initialization memset's must be
// kept together to avoid discrepancies in behavior.
//
// The initialization memset's clear the object and the metadata,
// but don't touch the SLAB redzone.
//
// The object's freepointer is also avoided if stored outside the
// object.
//
    if (unlikely(init)) {
    let mut rsize = 0;
    let mut inuse = 0;
    let mut orig_size = 0;
    inuse = get_info_end(s);
    orig_size = get_orig_size(s, x);
    if (!kasan_has_integrated_init()) {
    memset(kasan_reset_tag(x), 0, orig_size);
    }
    rsize = (s.flags & SLAB_RED_ZONE) ? s.red_left_pad : 0;
    memset(kasan_reset_tag(x) + inuse, 0,
    s.size - inuse - rsize);
//
// Restore orig_size, otherwise kmalloc redzone overwritten
// would be reported
//
    set_orig_size(s, x, orig_size);
    }
// KASAN might put x into memory quarantine, delaying its reuse.
    return !kasan_slab_free(s, x, init, still_accessible, false);
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn slab_free_freelist_hook(s: *mut kmem_cache, head: *mut *mut c_void, tail: *mut *mut c_void, cnt: *mut c_int) -> bool {
pub static mut object: *mut c_void = core::ptr::null_mut();
    let mut next = *head;
    let mut old_tail = *tail;
    let mut init = 0;
    if (is_kfence_address(next)) {
    slab_free_hook(s, next, false, false);
    return false;
    }
// Head and tail of the reconstructed freelist
// head = NULL;
// tail = NULL;
    init = slab_want_init_on_free(s);
    do {
    object = next;
    next = get_freepointer(s, object);
// If object's reuse doesn't have to be delayed
    if (likely(slab_free_hook(s, object, init, false))) {
// Move object to the new freelist
    set_freepointer(s, object, *head);
// head = object;
    if (!*tail) {
// tail = object;
    }
    } else {
//
// Adjust the reconstructed freelist depth
// accordingly if object's reuse is delayed.
//
    --(*cnt);
    }
    } while (object != old_tail);
    return *head != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn setup_object(s: *mut kmem_cache, object: *mut c_void) -> *mut c_void {
    setup_object_debug(s, object);
    object = kasan_init_slab_obj(s, object);
    if (unlikely(s.ctor)) {
    kasan_unpoison_new_object(s, object);
    s.ctor(object);
    kasan_poison_new_object(s, object);
    }
    return object;
    }
#[no_mangle]
pub unsafe extern "C" fn __alloc_empty_sheaf(s: *mut kmem_cache, gfp: gfp_t, alloc_flags: c_uint, capacity: c_uint) -> *mut c_void {
pub static mut sheaf: *mut c_void = core::ptr::null_mut();
    let mut sheaf_size = 0;
//
// Prevent recursion to the same cache, or a deep stack of kmallocs of
// varying sizes (sheaf capacity might differ for each kmalloc size
// bucket)
//
    if (s.flags & SLAB_KMALLOC) {
    alloc_flags |= SLAB_ALLOC_NO_RECURSE;
    }
    sheaf_size = struct_size(sheaf, objects, capacity);
    sheaf = kmalloc_flags(sheaf_size, gfp | __GFP_ZERO, alloc_flags, NUMA_NO_NODE);
    if (unlikely(!sheaf)) {
    return core::ptr::null_mut();
    }
    sheaf.cache = s;
    stat(s, SHEAF_ALLOC);
    return sheaf;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_empty_sheaf(s: *mut kmem_cache, gfp: gfp_t, alloc_flags: c_uint) -> *mut c_void {
    if (alloc_flags & SLAB_ALLOC_NO_RECURSE) {
    return core::ptr::null_mut();
    }
    gfp &= ~OBJCGS_CLEAR_MASK;
    return __alloc_empty_sheaf(s, gfp, alloc_flags, s.sheaf_capacity);
    }
#[no_mangle]
pub unsafe extern "C" fn __free_empty_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf, free_flags: c_uint) {
//
// If the sheaf was created with SLAB_ALLOC_NO_RECURSE flag then its
// corresponding extension is NULL and alloc_tag_sub() will throw a
// warning, therefore replace NULL with CODETAG_EMPTY to indicate
// that the extension for this sheaf is expected to be NULL.
//
    if (s.flags & SLAB_KMALLOC) {
    mark_obj_codetag_empty(sheaf);
    }
    VM_WARN_ON_ONCE(sheaf.size > 0);
    if (unlikely(free_flags & SLAB_FREE_NOLOCK)) {
    kfree_nolock(sheaf);
    }
    else {
    kfree(sheaf);
    }
    stat(s, SHEAF_FREE);
    }
#[no_mangle]
unsafe extern "C" fn free_empty_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf) {
    __free_empty_sheaf(s, sheaf, SLAB_FREE_DEFAULT);
    }
// forward_decl: refill_objects;
#[no_mangle]
pub unsafe extern "C" fn refill_sheaf(s: *mut kmem_cache, sheaf: *mut slab_sheaf, gfp: gfp_t) -> c_int {
pub static mut to_fill: c_int = 0;
    let mut filled = 0;
    if (!to_fill) {
    return 0;
    }
    filled = refill_objects(s, &sheaf.objects[sheaf.size], gfp, to_fill,
    to_fill);
    sheaf.size += filled;
    stat_add(s, SHEAF_REFILL, filled);
    if (filled < to_fill) {
    return -ENOMEM;
    }
    return 0;
    }
//
// Maximum number of objects freed during a single flush of main pcs sheaf.
// Translates directly to an on-stack array size.
//

// forward_decl: __kmem_cache_free_bulk;
//
// Free all objects from the main sheaf. In order to perform
// __kmem_cache_free_bulk() outside of cpu_sheaves->lock, work in batches where
// object pointers are moved to a on-stack array under the lock. To bound the
// stack usage, limit each batch to PCS_BATCH_MAX.
//
// Must be called with s->cpu_sheaves->lock locked, returns with the lock
// unlocked.
//
// Returns how many objects are remaining to be flushed
//
#[no_mangle]
unsafe extern "C" fn __sheaf_flush_main_batch(s: *mut kmem_cache) -> c_uint {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    let mut batch = 0;
    let mut remaining = 0;
    void *objects[PCS_BATCH_MAX];
pub static mut sheaf: *mut c_void = core::ptr::null_mut();
    slab_lockdep_assert_held(this_cpu_ptr(&s.cpu_sheaves.lock));
    pcs = this_cpu_ptr(s.cpu_sheaves);
    sheaf = pcs.main;
    batch = min(PCS_BATCH_MAX, sheaf.size);
    sheaf.size -= batch;
    memcpy(objects, sheaf.objects + sheaf.size, batch * sizeof!);
    remaining = sheaf.size;
    local_unlock(&s.cpu_sheaves.lock);
    __kmem_cache_free_bulk(s, batch, &objects[0]);
    stat_add(s, SHEAF_FLUSH, batch);
    return remaining;
    }
#[no_mangle]
unsafe extern "C" fn sheaf_flush_main(s: *mut kmem_cache) {
    let mut remaining = 0;
    do {
    local_lock(&s.cpu_sheaves.lock);
    remaining = __sheaf_flush_main_batch(s);
    } while (remaining);
    }
//
// Returns true if the main sheaf was at least partially flushed.
//
#[no_mangle]
unsafe extern "C" fn sheaf_try_flush_main(s: *mut kmem_cache) -> bool {
    let mut remaining = 0;
pub static mut ret: bool = false;
    do {
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return ret;
    }
    ret = true;
    remaining = __sheaf_flush_main_batch(s);
    } while (remaining);
    return ret;
    }
//
// Free all objects from a sheaf that's unused, i.e. not linked to any
// cpu_sheaves, so we need no locking and batching. The locking is also not
// necessary when flushing cpu's sheaves (both spare and main) during cpu
// hotremove as the cpu is not executing anymore.
//
#[no_mangle]
unsafe extern "C" fn sheaf_flush_unused(s: *mut kmem_cache, sheaf: *mut slab_sheaf) {
    if (!sheaf.size) {
    return;
    }
    stat_add(s, SHEAF_FLUSH, sheaf.size);
    __kmem_cache_free_bulk(s, sheaf.size, &sheaf.objects[0]);
    sheaf.size = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __rcu_free_sheaf_prepare(s: *mut kmem_cache, sheaf: *mut slab_sheaf) -> bool {
pub static mut init: bool = false;
    let mut p = &sheaf.objects[0];
pub static mut i: c_uint = 0;
pub static mut pfmemalloc: bool = false;
    while (i < sheaf.size) {
    let mut slab = virt_to_slab(p[i]);
    memcg_slab_free_hook(s, slab, p + i, 1);
    alloc_tagging_slab_free_hook(s, slab, p + i, 1);
    if (unlikely(!slab_free_hook(s, p[i], init, true))) {
    p[i] = p[--sheaf.size];
    continue;
    }
    if (slab_test_pfmemalloc(slab)) {
    pfmemalloc = true;
    }
    i += 1;
    }
    return pfmemalloc;
    }
#[no_mangle]
unsafe extern "C" fn rcu_free_sheaf_nobarn(head: *mut rcu_head) {
pub static mut sheaf: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    sheaf = container_of!(head, slab_sheaf, rcu_head);
    s = sheaf.cache;
    __rcu_free_sheaf_prepare(s, sheaf);
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
    }
//
// Caller needs to make sure migration is disabled in order to fully flush
// single cpu's sheaves
//
// must not be called from an irq
//
// flushing operations are rare so let's keep it simple and flush to slabs
// directly, skipping the barn
//
#[no_mangle]
unsafe extern "C" fn pcs_flush_all(s: *mut kmem_cache) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    let mut spare = core::ptr::null_mut();
    let mut rcu_free = core::ptr::null_mut();
    local_lock(&s.cpu_sheaves.lock);
    pcs = this_cpu_ptr(s.cpu_sheaves);
    spare = pcs.spare;
    pcs.spare = core::ptr::null_mut();
    rcu_free = pcs.rcu_free;
    pcs.rcu_free = core::ptr::null_mut();
    local_unlock(&s.cpu_sheaves.lock);
    if (spare) {
    sheaf_flush_unused(s, spare);
    free_empty_sheaf(s, spare);
    }
    if (rcu_free) {
    call_rcu(&rcu_free.rcu_head, rcu_free_sheaf_nobarn);
    }
    sheaf_flush_main(s);
    }
#[no_mangle]
unsafe extern "C" fn __pcs_flush_all_cpu(s: *mut kmem_cache, cpu: c_uint) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    pcs = per_cpu_ptr(s.cpu_sheaves, cpu);
// The cpu is not executing anymore so we don't need pcs->lock
    sheaf_flush_unused(s, pcs.main);
    if (pcs.spare) {
    sheaf_flush_unused(s, pcs.spare);
    free_empty_sheaf(s, pcs.spare);
    pcs.spare = core::ptr::null_mut();
    }
    if (pcs.rcu_free) {
    call_rcu(&pcs.rcu_free.rcu_head, rcu_free_sheaf_nobarn);
    pcs.rcu_free = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn pcs_destroy(s: *mut kmem_cache) {
    let mut cpu = 0;
//
// We may be unwinding cache creation that failed before or during the
// allocation of this.
//
    if (!s.cpu_sheaves) {
    return;
    }
// pcs->main can only point to the bootstrap sheaf, nothing to free
    if (!cache_has_sheaves(s)) {
// goto;
    }
    for_each_possible_cpu(cpu) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    pcs = per_cpu_ptr(s.cpu_sheaves, cpu);
// This can happen when unwinding failed cache creation.
    if (!pcs.main) {
    continue;
    }
//
// We have already passed __kmem_cache_shutdown() so everything
// was flushed and there should be no objects allocated from
// slabs, otherwise kmem_cache_destroy() would have aborted.
// Therefore something would have to be really wrong if the
// warnings here trigger, and we should rather leave objects and
// sheaves to leak in that case.
//
    WARN_ON!(pcs.spare);
    WARN_ON!(pcs.rcu_free);
    if (!WARN_ON!(pcs.main.size)) {
    free_empty_sheaf(s, pcs.main);
    pcs.main = core::ptr::null_mut();
    }
    }
// label;
    free_percpu(s.cpu_sheaves);
    s.cpu_sheaves = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn barn_get_empty_sheaf(barn: *mut node_barn, allow_spin: bool) -> *mut c_void {
    let mut empty = core::ptr::null_mut();
    let mut flags = 0;
    if (!data_race(barn.nr_empty)) {
    return core::ptr::null_mut();
    }
    if (likely(allow_spin)) {
    spin_lock_irqsave(&barn.lock, flags);
    }

    else if (!spin_trylock_irqsave(&barn.lock, flags)) {
    return core::ptr::null_mut();
    }
    if (likely(barn.nr_empty)) {
    empty = list_first_entry(&barn.sheaves_empty, slab_sheaf, barn_list);
    list_del(&empty.barn_list);
    barn.nr_empty -= 1;
    }
    spin_unlock_irqrestore(&barn.lock, flags);
    return empty;
    }
//
// The following two functions are used mainly in cases where we have to undo an
// intended action due to a race or cpu migration. Thus they do not check the
// empty or full sheaf limits for simplicity.
//
#[no_mangle]
unsafe extern "C" fn barn_put_empty_sheaf(barn: *mut node_barn, sheaf: *mut slab_sheaf) {
    let mut flags = 0;
    spin_lock_irqsave(&barn.lock, flags);
    list_add(&sheaf.barn_list, &barn.sheaves_empty);
    barn.nr_empty += 1;
    spin_unlock_irqrestore(&barn.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn barn_put_full_sheaf(barn: *mut node_barn, sheaf: *mut slab_sheaf) {
    let mut flags = 0;
    spin_lock_irqsave(&barn.lock, flags);
    list_add(&sheaf.barn_list, &barn.sheaves_full);
    barn.nr_full += 1;
    spin_unlock_irqrestore(&barn.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn barn_get_full_or_empty_sheaf(barn: *mut node_barn) -> *mut c_void {
    let mut sheaf = core::ptr::null_mut();
    let mut flags = 0;
    if (!data_race(barn.nr_full) && !data_race(barn.nr_empty)) {
    return core::ptr::null_mut();
    }
    spin_lock_irqsave(&barn.lock, flags);
    if (barn.nr_full) {
    sheaf = list_first_entry(&barn.sheaves_full, slab_sheaf,
    barn_list);
    list_del(&sheaf.barn_list);
    barn.nr_full -= 1;
    } else if (barn.nr_empty) {
    sheaf = list_first_entry(&barn.sheaves_empty, slab_sheaf, barn_list);
    list_del(&sheaf.barn_list);
    barn.nr_empty -= 1;
    }
    spin_unlock_irqrestore(&barn.lock, flags);
    return sheaf;
    }
//
// If a full sheaf is available, return it and put the supplied empty one to
// barn. We ignore the limit on empty sheaves as the number of sheaves doesn't
// change.
//
#[no_mangle]
pub unsafe extern "C" fn barn_replace_empty_sheaf(barn: *mut node_barn, empty: *mut slab_sheaf, allow_spin: bool) -> *mut c_void {
    let mut full = core::ptr::null_mut();
    let mut flags = 0;
    if (!data_race(barn.nr_full)) {
    return core::ptr::null_mut();
    }
    if (likely(allow_spin)) {
    spin_lock_irqsave(&barn.lock, flags);
    }

    else if (!spin_trylock_irqsave(&barn.lock, flags)) {
    return core::ptr::null_mut();
    }
    if (likely(barn.nr_full)) {
    full = list_first_entry(&barn.sheaves_full, slab_sheaf,
    barn_list);
    list_del(&full.barn_list);
    list_add(&empty.barn_list, &barn.sheaves_empty);
    barn.nr_full -= 1;
    barn.nr_empty += 1;
    }
    spin_unlock_irqrestore(&barn.lock, flags);
    return full;
    }
//
// If an empty sheaf is available, return it and put the supplied full one to
// barn. But if there are too many full sheaves, reject this with -E2BIG.
//
#[no_mangle]
pub unsafe extern "C" fn barn_replace_full_sheaf(barn: *mut node_barn, full: *mut slab_sheaf, allow_spin: bool) -> *mut c_void {
pub static mut empty: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
// we don't repeat this check under barn->lock as it's not critical
    if (data_race(barn.nr_full) >= MAX_FULL_SHEAVES) {
    return ERR_PTR(-E2BIG);
    }
    if (!data_race(barn.nr_empty)) {
    return ERR_PTR(-ENOMEM);
    }
    if (likely(allow_spin)) {
    spin_lock_irqsave(&barn.lock, flags);
    }

    else if (!spin_trylock_irqsave(&barn.lock, flags)) {
    return ERR_PTR(-EBUSY);
    }
    if (likely(barn.nr_empty)) {
    empty = list_first_entry(&barn.sheaves_empty, slab_sheaf,
    barn_list);
    list_del(&empty.barn_list);
    list_add(&full.barn_list, &barn.sheaves_full);
    barn.nr_empty -= 1;
    barn.nr_full += 1;
    } else {
    empty = ERR_PTR(-ENOMEM);
    }
    spin_unlock_irqrestore(&barn.lock, flags);
    return empty;
    }
#[no_mangle]
unsafe extern "C" fn barn_init(barn: *mut node_barn) {
    spin_lock_init(&barn.lock);
    INIT_LIST_HEAD(&barn.sheaves_full);
    INIT_LIST_HEAD(&barn.sheaves_empty);
    barn.nr_full = 0;
    barn.nr_empty = 0;
    }
#[no_mangle]
unsafe extern "C" fn barn_shrink(s: *mut kmem_cache, barn: *mut node_barn) {
pub static mut empty_list: usize = 0;
pub static mut full_list: usize = 0;
    let mut sheaf = core::ptr::null_mut();
    let mut sheaf2 = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&barn.lock, flags);
    list_splice_init(&barn.sheaves_full, &full_list);
    barn.nr_full = 0;
    list_splice_init(&barn.sheaves_empty, &empty_list);
    barn.nr_empty = 0;
    spin_unlock_irqrestore(&barn.lock, flags);
    list_for_each_entry_safe(sheaf, sheaf2, &full_list, barn_list) {
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
    }
    list_for_each_entry_safe(sheaf, sheaf2, &empty_list, barn_list) {
    free_empty_sheaf(s, sheaf);
    }
    }
//
// Slab allocation and freeing
//
#[no_mangle]
pub unsafe extern "C" fn alloc_slab_page(flags: gfp_t, node: c_int, oo: kmem_cache_order_objects, allow_spin: bool) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut order: c_uint = 0;
    if (unlikely(!allow_spin)) {
    page = alloc_frozen_pages_nolock(0/* __GFP_COMP is implied */,
    node, order);
    }

    else if (node == NUMA_NO_NODE) {
    page = alloc_frozen_pages(flags, order);
    }
    else {
    page = __alloc_frozen_pages(flags, order, node, core::ptr::null_mut(),
    ALLOC_DEFAULT);
    }
    if (!page) {
    return core::ptr::null_mut();
    }
    __SetPageSlab(page);
    slab = page_slab(page);
    if (page_is_pfmemalloc(page)) {
    slab_set_pfmemalloc(slab);
    }
    return slab;
    }

// Pre-initialize the random sequence cache
#[no_mangle]
unsafe extern "C" fn init_cache_random_seq(s: *mut kmem_cache) -> c_int {
pub static mut count: c_uint = 0;
    let mut err = 0;
// Bailout if already initialised
    if (s.random_seq) {
    return 0;
    }
    err = cache_random_seq_create(s, count, GFP_KERNEL);
    if (err) {
    pr_err!("SLUB: Unable to initialize free list for %s\n",
    s.name);
    return err;
    }
// Transform to an offset on the set of pages
    if (s.random_seq) {
    let mut i = 0;
    for (i = 0; i < count; i++) {
    s.random_seq[i] *= s.size;
    }
    }
    return 0;
    }
// Initialize each random sequence freelist per cache
#[no_mangle]
unsafe extern "C" fn init_freelist_randomization()  {
pub static mut s: *mut c_void = core::ptr::null_mut();
    mutex_lock(&slab_mutex);
    list_for_each_entry(s, &slab_caches, list) {
    init_cache_random_seq(s);
    }
    mutex_unlock(&slab_mutex);
    }
pub static mut struct rnd_state: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn init_cache_random_seq(s: *mut kmem_cache) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn init_freelist_randomization() { }

    static __always_inline void account_slab(slab *slab, int order, kmem_cache *s, gfp_t gfp,
    unsigned int alloc_flags)
    {
    if (memcg_kmem_online() &&
    (s.flags & SLAB_ACCOUNT) &&
    !slab_obj_exts(slab)) {
    alloc_slab_obj_exts(slab, s, gfp,
    alloc_flags | SLAB_ALLOC_NEW_SLAB);
    }
    mod_node_page_state(slab_pgdat(slab), cache_vmstat_idx(s),
    PAGE_SIZE << order);
    }
    static __always_inline void unaccount_slab(slab *slab, int order, kmem_cache *s, bool allow_spin)
    {
//
// The slab object extensions should now be freed regardless of
// whether mem_alloc_profiling_enabled() or not because profiling
// might have been disabled after slab->obj_exts got allocated.
//
    free_slab_obj_exts(slab, allow_spin);
    mod_node_page_state(slab_pgdat(slab), cache_vmstat_idx(s),
    -(PAGE_SIZE << order));
    }
// Allocate and initialize a slab without building its freelist.
#[no_mangle]
pub unsafe extern "C" fn allocate_slab(s: *mut kmem_cache, flags: gfp_t, alloc_flags: c_uint, node: c_int) -> *mut c_void {
pub static mut allow_spin: bool = false;
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut oo: kmem_cache_order_objects = 0;
    let mut alloc_gfp;
pub static mut start: *mut c_void = core::ptr::null_mut();
    flags &= gfp_allowed_mask;
    flags |= s.allocflags;
//
// Let the initial higher-order allocation fail under memory pressure
// so we fall-back to the minimum order allocation.
//
    alloc_gfp = (flags | __GFP_NOWARN | __GFP_NORETRY) & ~__GFP_NOFAIL;
    if ((alloc_gfp & __GFP_DIRECT_RECLAIM) && oo_order(oo) > oo_order(s.min)) {
    alloc_gfp = (alloc_gfp | __GFP_NOMEMALLOC) & ~__GFP_RECLAIM;
    }
    slab = alloc_slab_page(alloc_gfp, node, oo, allow_spin);
    if (unlikely(!slab)) {
    oo = s.min;
    alloc_gfp = flags;
//
// Allocation may have failed due to fragmentation.
// Try a lower order alloc if possible
//
    slab = alloc_slab_page(alloc_gfp, node, oo, allow_spin);
    if (unlikely(!slab)) {
    return core::ptr::null_mut();
    }
    stat(s, ORDER_FALLBACK);
    }
// Initializes frozen, inuse, and any extra 64bit-only flags
    slab.counters = 0;
    slab.objects = oo_objects(oo);

    if (cache_needs_objcg(s)) {
    slab.obj_exts_needs_objcg = 1;
    }

    slab.slab_cache = s;
    kasan_poison_slab(slab);
    start = slab_address(slab);
    setup_slab_debug(s, slab, start);
    init_slab_obj_exts(slab);
//
// Poison the slab before initializing the slabobj_ext array
// to prevent the array from being overwritten.
//
    alloc_slab_obj_exts_early(s, slab);
    account_slab(slab, oo_order(oo), s, flags, alloc_flags);
    return slab;
    }
#[no_mangle]
pub unsafe extern "C" fn new_slab(s: *mut kmem_cache, flags: gfp_t, alloc_flags: c_uint, node: c_int) -> *mut c_void {
    if (unlikely(flags & GFP_SLAB_BUG_MASK)) {
    flags = kmalloc_fix_flags(flags);
    }
    WARN_ON_ONCE!(s.ctor && (flags & __GFP_ZERO));
    flags &= GFP_RECLAIM_MASK | GFP_CONSTRAINT_MASK;
    return allocate_slab(s, flags, alloc_flags, node);
    }
#[no_mangle]
unsafe extern "C" fn __free_slab(s: *mut kmem_cache, slab: *mut slab, allow_spin: bool) {
    let mut page = slab_page(slab);
pub static mut order: c_int = 0;
pub static mut pages: c_int = 0;
    __slab_clear_pfmemalloc(slab);
    page.mapping = core::ptr::null_mut();
    __ClearPageSlab(page);
    mm_account_reclaimed_pages(pages);
    unaccount_slab(slab, order, s, allow_spin);
    if (allow_spin) {
    free_frozen_pages(page, order);
    }
    else {
    free_frozen_pages_nolock(page, order);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_new_slab_nolock(s: *mut kmem_cache, slab: *mut slab) {
//
// Since it was just allocated, we can skip the actions in
// discard_slab() and free_slab().
//
    __free_slab(s, slab, false);
    }
#[no_mangle]
unsafe extern "C" fn rcu_free_slab(h: *mut rcu_head) {
    let mut slab = container_of!(h, slab, rcu_head);
    __free_slab(slab.slab_cache, slab, true);
    }
#[no_mangle]
unsafe extern "C" fn free_slab(s: *mut kmem_cache, slab: *mut slab) {
    if (kmem_cache_debug_flags(s, SLAB_CONSISTENCY_CHECKS)) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    slab_pad_check(s, slab);
    for_each_object(p, s, slab_address(slab), slab.objects) {
    check_object(s, slab, p, SLUB_RED_INACTIVE);
    }
    }
    if (unlikely(s.flags & SLAB_TYPESAFE_BY_RCU)) {
    call_rcu(&slab.rcu_head, rcu_free_slab);
    }
    else {
    __free_slab(s, slab, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn discard_slab(s: *mut kmem_cache, slab: *mut slab) {
    dec_slabs_node(s, slab_nid(slab), slab.objects);
    free_slab(s, slab);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_test_node_partial(slab: *const slab) -> bool {
    return test_bit(SL_partial, &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_set_node_partial(slab: *mut slab) {
    set_bit(SL_partial, &slab.flags.f);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_clear_node_partial(slab: *mut slab) {
    clear_bit(SL_partial, &slab.flags.f);
    }
//
// Management of partially allocated slabs.
//
#[no_mangle]
pub unsafe extern "C" fn set_node_partial_state(n: *mut kmem_cache_node, slab: *mut slab) {
    slab_set_node_partial(slab);
    n.nr_partial += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __add_partial(n: *mut kmem_cache_node, slab: *mut slab, mode: add_mode) {
    if (mode == ADD_TO_TAIL) {
    list_add_tail(&slab.slab_list, &n.partial);
    }
    else {
    list_add(&slab.slab_list, &n.partial);
    }
    set_node_partial_state(n, slab);
    }
#[no_mangle]
pub unsafe extern "C" fn add_partial(n: *mut kmem_cache_node, slab: *mut slab, mode: add_mode) {
    slab_lockdep_assert_held(&n.list_lock);
    __add_partial(n, slab, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn clear_node_partial_state(n: *mut kmem_cache_node, slab: *mut slab) {
    slab_clear_node_partial(slab);
    n.nr_partial -= 1;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_partial(n: *mut kmem_cache_node, slab: *mut slab) {
    slab_lockdep_assert_held(&n.list_lock);
    list_del(&slab.slab_list);
    clear_node_partial_state(n, slab);
    }
//
// Called only for kmem_cache_debug() caches instead of remove_partial(), with a
// slab from the n->partial list. Remove only a single object from the slab, do
// the alloc_debug_processing() checks and leave the slab on the list, or move
// it to full list if it was the last free object.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_single_from_partial(s: *mut kmem_cache, n: *mut kmem_cache_node, slab: *mut slab, orig_size: c_int) -> *mut c_void {
pub static mut object: *mut c_void = core::ptr::null_mut();
    slab_lockdep_assert_held(&n.list_lock);

    if (s.flags & SLAB_CONSISTENCY_CHECKS) {
    if (!validate_slab_ptr(slab)) {
    slab_err(s, slab, "Not a valid slab page");
    return core::ptr::null_mut();
    }
    }

    object = slab.freelist;
    slab.freelist = get_freepointer(s, object);
    slab.inuse += 1;
    if (!alloc_debug_processing(s, slab, object, orig_size)) {
    remove_partial(n, slab);
    return core::ptr::null_mut();
    }
    if (slab.inuse == slab.objects) {
    remove_partial(n, slab);
    add_full(s, n, slab);
    }
    return object;
    }
// Return the next free object in allocation order.
#[no_mangle]
pub unsafe extern "C" fn next_slab_obj(s: *mut kmem_cache, iter: *mut slab_obj_iter) -> *mut c_void {

    if (iter.random) {
    let mut idx = 0;
//
// If the target page allocation failed, the number of objects on the
// page might be smaller than the usual size defined by the cache.
//
    do {
    idx = s.random_seq[iter.pos];
    iter.pos += 1;
    if (iter.pos >= iter.freelist_count) {
    iter.pos = 0;
    }
    } while (unlikely(idx >= iter.page_limit));
    return setup_object(s, iter.start + idx);
    }

    return setup_object(s, iter.start + iter.pos++ * s.size);
    }
// Build a freelist from the objects not yet allocated from a fresh slab.
#[no_mangle]
pub unsafe extern "C" fn build_slab_freelist(s: *mut kmem_cache, slab: *mut slab, iter: *mut slab_obj_iter) {
pub static mut nr: c_uint = 0;
    let mut i = 0;
    let mut cur = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    if (!nr) {
    slab.freelist = core::ptr::null_mut();
    return;
    }
    cur = next_slab_obj(s, iter);
    slab.freelist = cur;
    while (i < nr) {
    next = next_slab_obj(s, iter);
    set_freepointer(s, cur, next);
    cur = next;
    }
    set_freepointer(s, cur, core::ptr::null_mut());
    }
// Initialize an iterator over free objects in allocation order.
#[no_mangle]
pub unsafe extern "C" fn init_slab_obj_iter(s: *mut kmem_cache, slab: *mut slab, iter: *mut slab_obj_iter, allow_spin: bool) {
    iter.pos = 0;
    iter.start = fixup_red_left(s, slab_address(slab));

    iter.random = (slab.objects >= 2 && s.random_seq);
    if (!iter.random) {
    return;
    }
    iter.freelist_count = oo_objects(s.oo);
    iter.page_limit = slab.objects * s.size;
    if (allow_spin) {
    iter.pos = get_random_u32_below(iter.freelist_count);
    } else {
pub static mut state: *mut c_void = core::ptr::null_mut();
//
// An interrupt or NMI handler might interrupt and change
// the state in the middle, but that's safe.
//
    state = &get_cpu_var(slab_rnd_state);
    iter.pos = prandom_u32_state(state) % iter.freelist_count;
    put_cpu_var(slab_rnd_state);
    }

    }
//
// Called only for kmem_cache_debug() caches to allocate from a freshly
// allocated slab. Allocate a single object instead of whole freelist
// and put the slab to the partial (or full) list.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_single_from_new_slab(s: *mut kmem_cache, slab: *mut slab, ac: *mut slab_alloc_context) -> *mut c_void {
pub static mut allow_spin: bool = false;
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
    let mut needs_add_partial = 0;
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    init_slab_obj_iter(s, slab, &iter, allow_spin);
    object = next_slab_obj(s, &iter);
    slab.inuse = 1;
    needs_add_partial = (slab.objects > 1);
    build_slab_freelist(s, slab, &iter);
// alloc_debug_processing() always expects a valid freepointer
    set_freepointer(s, object, slab.freelist);
    if (!alloc_debug_processing(s, slab, object, ac.orig_size)) {
//
// It's not really expected that this would fail on a
// freshly allocated slab, but a concurrent memory
// corruption in theory could cause that.
// Leak memory of allocated slab.
//
    return core::ptr::null_mut();
    }
    n = get_node(s, slab_nid(slab));
    if (allow_spin) {
    spin_lock_irqsave(&n.list_lock, flags);
    } else if (!spin_trylock_irqsave(&n.list_lock, flags)) {
//
// Unlucky, discard newly allocated slab.
// The slab is not fully free, but it's fine as
// objects are not allocated to users.
//
    free_new_slab_nolock(s, slab);
    return core::ptr::null_mut();
    }
    if (needs_add_partial) {
    add_partial(n, slab, ADD_TO_HEAD);
    }
    else {
    add_full(s, n, slab);
    }
//
// Debug caches require nr_slabs updates under n->list_lock so validation
// cannot race with slab (de)allocations and observe inconsistent state.
//
    inc_slabs_node(s, slab_nid(slab), slab.objects);
    spin_unlock_irqrestore(&n.list_lock, flags);
    return object;
    }
// forward_decl: pfmemalloc_match;
#[no_mangle]
pub unsafe extern "C" fn get_partial_node_bulk(s: *mut kmem_cache, n: *mut kmem_cache_node, pc: *mut partial_bulk_context, allow_spin: bool) -> bool {
    let mut slab = core::ptr::null_mut();
    let mut slab2 = core::ptr::null_mut();
    let mut first = core::ptr::null_mut(), *last = core::ptr::null_mut();
pub static mut total_free: c_uint = 0;
    let mut flags = 0;
// Racy check to avoid taking the lock unnecessarily.
    if (!n || data_race(!n.nr_partial)) {
    return false;
    }
    INIT_LIST_HEAD(&pc.slabs);
    if (allow_spin) {
    spin_lock_irqsave(&n.list_lock, flags);
    }

    else if (!spin_trylock_irqsave(&n.list_lock, flags)) {
    return false;
    }
    list_for_each_entry_safe(slab, slab2, &n.partial, slab_list) {
pub static mut flc: usize = 0;
    let mut slab_free = 0;
    if (!pfmemalloc_match(slab, pc.flags)) {
    if (first) {
    list_bulk_move_tail(&pc.slabs,
    &first.slab_list,
    &last.slab_list);
    first = core::ptr::null_mut();
    }
    continue;
    }
//
// determine the number of free objects in the slab racily
//
// slab_free is a lower bound due to possible subsequent
// concurrent freeing, so the caller may get more objects than
// requested and must handle that
//
    flc.counters = data_race(READ_ONCE(slab.counters));
    slab_free = flc.objects - flc.inuse;
// we have already min and this would get us over the max
    if (total_free >= pc.min_objects
    && total_free + slab_free > pc.max_objects) {
    break;
    }
    if (!first) {
    first = slab;
    }
    last = slab;
    clear_node_partial_state(n, slab);
    total_free += slab_free;
    if (total_free >= pc.max_objects) {
    break;
    }
    }
    if (first) {
    list_bulk_move_tail(&pc.slabs, &first.slab_list,
    &last.slab_list);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    return total_free > 0;
    }
//
// Try to allocate object from a partial slab on a specific node.
//
#[no_mangle]
pub unsafe extern "C" fn get_from_partial_node(s: *mut kmem_cache, n: *mut kmem_cache_node, gfp_flags: gfp_t, ac: *mut slab_alloc_context) -> *mut c_void {
    let mut slab = core::ptr::null_mut();
    let mut slab2 = core::ptr::null_mut();
    let mut flags = 0;
    let mut object = core::ptr::null_mut();
//
// Racy check. If we mistakenly see no partial slabs then we
// just allocate an empty slab. If we mistakenly try to get a
// partial slab and there is none available then get_from_partial()
// will return NULL.
//
    if (!n || !n.nr_partial) {
    return core::ptr::null_mut();
    }
    if (alloc_flags_allow_spinning(ac.alloc_flags)) {
    spin_lock_irqsave(&n.list_lock, flags);
    }

    else if (!spin_trylock_irqsave(&n.list_lock, flags)) {
    return core::ptr::null_mut();
    }
    list_for_each_entry_safe(slab, slab2, &n.partial, slab_list) {
    struct freelist_counters old, new;
    if (!pfmemalloc_match(slab, gfp_flags)) {
    continue;
    }
    if (IS_ENABLED!(CONFIG_SLUB_TINY) || kmem_cache_debug(s)) {
    object = alloc_single_from_partial(s, n, slab,
    ac.orig_size);
    if (object) {
    break;
    }
    continue;
    }
//
// get a single object from the slab. This might race against
// __slab_free(), which however has to take the list_lock if
// it's about to make the slab fully free.
//
    do {
    old.freelist = slab.freelist;
    old.counters = slab.counters;
    new.freelist = get_freepointer(s, old.freelist);
    new.counters = old.counters;
    new.inuse += 1;
    } while (!__slab_update_freelist(s, slab, &old, &new, "get_from_partial_node"));
    object = old.freelist;
    if (!new.freelist) {
    remove_partial(n, slab);
    }
    break;
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    return object;
    }
//
// Get an object from somewhere. Search in increasing NUMA distances.
//
#[no_mangle]
pub unsafe extern "C" fn get_from_any_partial(s: *mut kmem_cache, gfp_flags: gfp_t, ac: *mut slab_alloc_context) -> *mut c_void {

pub static mut zonelist: *mut c_void = core::ptr::null_mut();
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut highest_zoneidx: zone_type = 0;
    let mut cpuset_mems_cookie = 0;
pub static mut allow_spin: bool = false;
//
// The defrag ratio allows a configuration of the tradeoffs between
// inter node defragmentation and node local allocations. A lower
// defrag_ratio increases the tendency to do local allocations
// instead of attempting to obtain partial slabs from other nodes.
//
// If the defrag_ratio is set to 0 then kmalloc() always
// returns node local objects. If the ratio is higher then kmalloc()
// may return off node objects because partial slabs are obtained
// from other nodes and filled up.
//
// If /sys/kernel/slab/xx/remote_node_defrag_ratio is set to 100
// (which makes defrag_ratio = 1000) then every (well almost)
// allocation will first attempt to defrag slab caches on other nodes.
// This means scanning over all nodes to look for partial slabs which
// may be expensive if we do it every time we are trying to find a slab
// with available objects.
//
    if (!s.remote_node_defrag_ratio ||
    get_cycles() % 1024 > s.remote_node_defrag_ratio) {
    return core::ptr::null_mut();
    }
    do {
//
// read_mems_allowed_begin() accesses current->mems_allowed_seq,
// a seqcount_spinlock_t that is not NMI-safe. Do not access
// current->mems_allowed_seq and avoid retry when GFP flags
// indicate spinning is not allowed.
//
    if (allow_spin) {
    cpuset_mems_cookie = read_mems_allowed_begin();
    }
    zonelist = node_zonelist(mempolicy_slab_node(), gfp_flags);
    for_each_zone_zonelist(zone, z, zonelist, highest_zoneidx) {
pub static mut n: *mut c_void = core::ptr::null_mut();
    n = get_node(s, zone_to_nid(zone));
    if (n && cpuset_zone_allowed(zone, gfp_flags) &&
    n.nr_partial > s.min_partial) {
    let mut object = get_from_partial_node(s, n,
    gfp_flags, ac);
    if (object) {
//
// Don't check read_mems_allowed_retry()
// here - if mems_allowed was updated in
// parallel, that was a harmless race
// between allocation and the cpuset
// update
//
    return object;
    }
    }
    }
    } while (allow_spin && read_mems_allowed_retry(cpuset_mems_cookie));

    return core::ptr::null_mut();
    }
//
// Get an object from a partial slab
//
#[no_mangle]
pub unsafe extern "C" fn get_from_partial(s: *mut kmem_cache, node: c_int, flags: gfp_t, ac: *mut slab_alloc_context) -> *mut c_void {
pub static mut searchnode: c_int = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    if (node == NUMA_NO_NODE) {
    searchnode = numa_mem_id();
    }
    object = get_from_partial_node(s, get_node(s, searchnode), flags, ac);
    if (object || (node != NUMA_NO_NODE && (flags & __GFP_THISNODE))) {
    return object;
    }
    return get_from_any_partial(s, flags, ac);
    }
#[no_mangle]
unsafe extern "C" fn has_pcs_used(cpu: c_int, s: *mut kmem_cache) -> bool {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    if (!cache_has_sheaves(s)) {
    return false;
    }
    pcs = per_cpu_ptr(s.cpu_sheaves, cpu);
    return (pcs.spare || pcs.rcu_free || pcs.main.size);
    }
//
// Flush percpu sheaves
//
// Called from CPU work handler with migration disabled.
//
#[no_mangle]
unsafe extern "C" fn flush_cpu_sheaves(w: *mut work_struct) {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut sfw: *mut c_void = core::ptr::null_mut();
    sfw = container_of!(w, slub_flush_work, work);
    s = sfw.s;
    if (cache_has_sheaves(s)) {
    pcs_flush_all(s);
    }
    }
#[no_mangle]
unsafe extern "C" fn flush_all_cpus_locked(s: *mut kmem_cache) {
pub static mut sfw: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    lockdep_assert_cpus_held();
    mutex_lock(&flush_lock);
    for_each_online_cpu(cpu) {
    sfw = &per_cpu(slub_flush, cpu);
    if (!has_pcs_used(cpu, s)) {
    sfw.skip = true;
    continue;
    }
    INIT_WORK(&sfw.work, flush_cpu_sheaves);
    sfw.skip = false;
    sfw.s = s;
    queue_work_on(cpu, flushwq, &sfw.work);
    }
    for_each_online_cpu(cpu) {
    sfw = &per_cpu(slub_flush, cpu);
    if (sfw.skip) {
    continue;
    }
    flush_work(&sfw.work);
    }
    mutex_unlock(&flush_lock);
    }
#[no_mangle]
unsafe extern "C" fn flush_all(s: *mut kmem_cache) {
    cpus_read_lock();
    flush_all_cpus_locked(s);
    cpus_read_unlock();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deferred_percpu_work {
    pub objects: llist_head,
    pub objects_by_rcu: llist_head,
    pub rcu_sheaves: llist_head,
    pub work: irq_work,
}

// forward_decl: deferred_percpu_work_fn;
    static DEFINE_PER_CPU(deferred_percpu_work, deferred_percpu_work) = {
    .objects = LLIST_HEAD_INIT(objects),
    .objects_by_rcu = LLIST_HEAD_INIT(objects_by_rcu),
    .rcu_sheaves = LLIST_HEAD_INIT(rcu_sheaves),
    .work = IRQ_WORK_INIT(deferred_percpu_work_fn),
    };
#[no_mangle]
unsafe extern "C" fn flush_rcu_sheaf(w: *mut work_struct) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
pub static mut rcu_free: *mut c_void = core::ptr::null_mut();
pub static mut sfw: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    sfw = container_of!(w, slub_flush_work, work);
    s = sfw.s;
    local_lock(&s.cpu_sheaves.lock);
    pcs = this_cpu_ptr(s.cpu_sheaves);
    rcu_free = pcs.rcu_free;
    pcs.rcu_free = core::ptr::null_mut();
    local_unlock(&s.cpu_sheaves.lock);
    if (rcu_free) {
    call_rcu(&rcu_free.rcu_head, rcu_free_sheaf_nobarn);
    }
    }
// needed for kvfree_rcu_barrier()
#[no_mangle]
pub unsafe extern "C" fn flush_rcu_sheaves_on_cache(s: *mut kmem_cache) {
pub static mut sfw: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    lockdep_assert_cpus_held();
    mutex_lock(&flush_lock);
    for_each_online_cpu(cpu) {
    sfw = &per_cpu(slub_flush, cpu);
//
// we don't check if rcu_free sheaf exists - racing
// __kfree_rcu_sheaf() might have just removed it.
// by executing flush_rcu_sheaf() on the cpu we make
// sure the __kfree_rcu_sheaf() finished its call_rcu()
//
    INIT_WORK(&sfw.work, flush_rcu_sheaf);
    sfw.s = s;
    queue_work_on(cpu, flushwq, &sfw.work);
    }
    for_each_online_cpu(cpu) {
    sfw = &per_cpu(slub_flush, cpu);
    flush_work(&sfw.work);
    }
    mutex_unlock(&flush_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_all_rcu_sheaves() {
pub static mut s: *mut c_void = core::ptr::null_mut();
    deferred_work_barrier();
    cpus_read_lock();
    mutex_lock(&slab_mutex);
    list_for_each_entry(s, &slab_caches, list) {
    if (!cache_has_sheaves(s)) {
    continue;
    }
    flush_rcu_sheaves_on_cache(s);
    }
    mutex_unlock(&slab_mutex);
    cpus_read_unlock();
    rcu_barrier();
    }
#[no_mangle]
unsafe extern "C" fn slub_cpu_setup(cpu: c_uint) -> c_int {
pub static mut nid: c_int = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// we never clear a nid so it's safe to do a quick check before taking
// the mutex, and then recheck to handle parallel cpu hotplug safely
//
    if (node_isset(nid, slab_barn_nodes)) {
    return 0;
    }
    mutex_lock(&slab_mutex);
    if (node_isset(nid, slab_barn_nodes)) {
// goto;
    }
    list_for_each_entry(s, &slab_caches, list) {
pub static mut barn: *mut c_void = core::ptr::null_mut();
//
// barn might already exist if a previous callback failed midway
//
    if (!cache_has_sheaves(s) || get_barn_node(s, nid)) {
    continue;
    }
    barn = kmalloc_node(sizeof!(*barn), GFP_KERNEL, nid);
    if (!barn) {
    ret = -ENOMEM;
// goto;
    }
    barn_init(barn);
    s.per_node[nid].barn = barn;
    }
    node_set(nid, slab_barn_nodes);
// label;
    mutex_unlock(&slab_mutex);
    return ret;
    }
//
// Use the cpu notifier to insure that the cpu slabs are flushed when
// necessary.
//
#[no_mangle]
unsafe extern "C" fn slub_cpu_dead(cpu: c_uint) -> c_int {
pub static mut s: *mut c_void = core::ptr::null_mut();
    mutex_lock(&slab_mutex);
    list_for_each_entry(s, &slab_caches, list) {
    if (cache_has_sheaves(s)) {
    __pcs_flush_all_cpu(s, cpu);
    }
    }
    mutex_unlock(&slab_mutex);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn count_free(slab: *mut slab) -> c_int {
    return slab.objects - slab.inuse;
    }
#[no_mangle]
pub unsafe extern "C" fn node_nr_objs(n: *mut kmem_cache_node) -> c_ulong {
    return atomic_long_read(&n.total_objects);
    }
// Supports checking bulk free of a constructed freelist
#[no_mangle]
#[no_mangle]
// duplicate fn: free_debug_processing
pub unsafe extern "C" fn free_debug_processing_dup(s: *mut kmem_cache, slab: *mut slab, head: *mut c_void, tail: *mut c_void, bulk_cnt: *mut c_int, addr: c_ulong, handle: depot_stack_handle_t) -> bool {
pub static mut checks_ok: bool = false;
    let mut object = head;
pub static mut cnt: c_int = 0;
    if (s.flags & SLAB_CONSISTENCY_CHECKS) {
    if (!check_slab(s, slab)) {
// goto;
    }
    }
    if (slab.inuse < *bulk_cnt) {
    slab_err(s, slab, "Slab has %d allocated objects but %d are to be freed\n",
    slab.inuse, *bulk_cnt);
// goto;
    }
// label;
    if (++cnt > *bulk_cnt) {
// goto;
    }
    if (s.flags & SLAB_CONSISTENCY_CHECKS) {
    if (!free_consistency_checks(s, slab, object, addr)) {
// goto;
    }
    }
    if (s.flags & SLAB_STORE_USER) {
    set_track_update(s, object, TRACK_FREE, addr, handle);
    }
    trace(s, slab, object, 0);
// Freepointer not overwritten by init_object(), SLAB_POISON moved it
    init_object(s, object, SLUB_RED_INACTIVE);
// Reached end of constructed freelist yet?
    if (object != tail) {
    object = get_freepointer(s, object);
// goto;
    }
    checks_ok = true;
// label;
    if (cnt != *bulk_cnt) {
    slab_err(s, slab, "Bulk free expected %d objects but found %d\n",
// bulk_cnt, cnt);
// bulk_cnt = cnt;
    }
// label;
    if (!checks_ok) {
    slab_fix(s, "Object at 0x%p not freed", object);
    }
    return checks_ok;
    }

#[no_mangle]
pub unsafe extern "C" fn count_partial(n: *mut kmem_cache_node) -> c_ulong {
    let mut flags = 0;
pub static mut x: c_ulong = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
    spin_lock_irqsave(&n.list_lock, flags);
    list_for_each_entry(slab, &n.partial, slab_list) {
    x += get_count(slab);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    return x;
    }

pub const MAX_PARTIAL_TO_SCAN: c_int = 10000;
#[no_mangle]
unsafe extern "C" fn count_partial_free_approx(n: *mut kmem_cache_node) -> c_ulong {
    let mut flags = 0;
pub static mut x: c_ulong = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
    spin_lock_irqsave(&n.list_lock, flags);
    if (n.nr_partial <= MAX_PARTIAL_TO_SCAN) {
    list_for_each_entry(slab, &n.partial, slab_list) {
    x += slab.objects - slab.inuse;
    }
    } else {
//
// For a long list, approximate the total count of objects in
// it to meet the limit on the number of slabs to scan.
// Scan from both the list's head and tail for better accuracy.
//
pub static mut scanned: c_ulong = 0;
    list_for_each_entry(slab, &n.partial, slab_list) {
    x += slab.objects - slab.inuse;
    if (++scanned == MAX_PARTIAL_TO_SCAN / 2) {
    break;
    }
    }
    list_for_each_entry_reverse(slab, &n.partial, slab_list) {
    x += slab.objects - slab.inuse;
    if (++scanned == MAX_PARTIAL_TO_SCAN) {
    break;
    }
    }
    x = mult_frac(x, n.nr_partial, scanned);
    x = min(x, node_nr_objs(n));
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    return x;
    }
    static noinline void
    slab_out_of_memory(kmem_cache *s, gfp_t gfpflags, int nid)
    {
pub static mut slub_oom_rs: usize = 0;
pub static mut cpu: c_int = 0;
    let mut node = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    if ((gfpflags & __GFP_NOWARN) || !__ratelimit(&slub_oom_rs)) {
    return;
    }
    pr_warn!("SLUB: Unable to allocate memory on CPU %u (of node %d) on node %d, gfp=%#x(%pGg)\n",
    cpu, cpu_to_node(cpu), nid, gfpflags, &gfpflags);
    pr_warn!("  cache: %s, object size: %u, buffer size: %u, default order: %u, min order: %u\n",
    s.name, s.object_size, s.size, oo_order(s.oo),
    oo_order(s.min));
    if (oo_order(s.min) > get_order(s.object_size)) {
    pr_warn!("  %s debugging increased min order, use slab_debug=O to disable.\n",
    s.name);
    }
    for_each_kmem_cache_node(s, node, n) {
    let mut nr_slabs = 0;
    let mut nr_objs = 0;
    let mut nr_free = 0;
    nr_free  = count_partial_free_approx(n);
    nr_slabs = node_nr_slabs(n);
    nr_objs  = node_nr_objs(n);
    pr_warn!("  node %d: slabs: %ld, objs: %ld, free: %ld\n",
    node, nr_slabs, nr_objs, nr_free);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn slab_out_of_memory(s: *mut kmem_cache, gfpflags: gfp_t, nid: c_int) { }

#[no_mangle]
pub unsafe extern "C" fn pfmemalloc_match(slab: *mut slab, gfpflags: gfp_t) -> bool {
    if (unlikely(slab_test_pfmemalloc(slab))) {
    return gfp_pfmemalloc_allowed(gfpflags);
    }
    return true;
    }
//
// Get the slab's freelist and do not freeze it.
//
// Assumes the slab is isolated from node partial list and not frozen.
//
// Assumes this is performed only for caches without debugging so we
// don't need to worry about adding the slab to the full list.
//
#[no_mangle]
pub unsafe extern "C" fn get_freelist_nofreeze(s: *mut kmem_cache, slab: *mut slab, count: *mut c_uint) -> *mut c_void {
    struct freelist_counters old, new;
    do {
    old.freelist = slab.freelist;
    old.counters = slab.counters;
    new.freelist = core::ptr::null_mut();
    new.counters = old.counters;
    VM_WARN_ON_ONCE(new.frozen);
    new.inuse = old.objects;
    } while (!slab_update_freelist(s, slab, &old, &new, "get_freelist_nofreeze"));
// count = old.objects - old.inuse;
    return old.freelist;
    }
//
// If the object has been wiped upon free, make sure it's fully initialized by
// zeroing out freelist pointer.
//
// Note that we also wipe custom freelist pointers.
//
    static __always_inline void maybe_wipe_obj_freeptr(kmem_cache *s,
    void *obj)
    {
    if (unlikely(slab_want_init_on_free(s)) && obj &&
    !freeptr_outside_object(s)) {
    memset((kasan_reset_tag(obj) + s.offset),
    0, sizeof!);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_from_new_slab(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut c_void, count: c_uint, allow_spin: bool) -> c_uint {
pub static mut allocated: c_uint = 0;
pub static mut iter: usize = 0;
pub static mut needs_add_partial: bool = true;
    let mut flags = 0;
//
// Are we going to put the slab on the partial list?
// Note slab->inuse is 0 on a new slab.
//
    if (count >= slab.objects) {
    needs_add_partial = false;
    count = slab.objects;
    }
    init_slab_obj_iter(s, slab, &iter, allow_spin);
    while (allocated < count) {
    p[allocated] = next_slab_obj(s, &iter);
    allocated += 1;
    }
    slab.inuse = count;
    build_slab_freelist(s, slab, &iter);
    if (needs_add_partial) {
    let mut n = get_node(s, slab_nid(slab));
    if (allow_spin) {
    spin_lock_irqsave(&n.list_lock, flags);
    } else if (!spin_trylock_irqsave(&n.list_lock, flags)) {
//
// Unlucky, discard newly allocated slab.
// The slab is not fully free, but it's fine as
// objects are not allocated to users.
//
    free_new_slab_nolock(s, slab);
    return 0;
    }
    add_partial(n, slab, ADD_TO_HEAD);
    spin_unlock_irqrestore(&n.list_lock, flags);
    }
    inc_slabs_node(s, slab_nid(slab), slab.objects);
    return allocated;
    }
//
// Slow path. We failed to allocate via percpu sheaves or they are not available
// due to bootstrap or debugging enabled or SLUB_TINY.
//
// We try to allocate from partial slab lists and fall back to allocating a new
// slab.
//
#[no_mangle]
pub unsafe extern "C" fn ___slab_alloc(s: *mut kmem_cache, gfpflags: gfp_t, node: c_int, ac: *mut slab_alloc_context) -> *mut c_void {
pub static mut allow_spin: bool = false;
    let mut trynode_flags;
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut try_thisnode: bool = true;
    stat(s, ALLOC_SLOWPATH);
// label;
    trynode_flags = gfpflags;
//
// When a preferred node is indicated but no __GFP_THISNODE
//
// 1) try to get a partial slab from target node only by having
// __GFP_THISNODE in trynode_flags for get_from_partial()
// 2) if 1) failed, try to allocate a new slab from target node with
// (at most) GFP_NOWAIT | __GFP_THISNODE opportunistically
// 3) if 2) failed, retry with original gfpflags which will allow
// get_from_partial() try partial lists of other nodes before
// potentially allocating new page from other nodes
//
    if (unlikely(node != NUMA_NO_NODE && !(gfpflags & __GFP_THISNODE)
    && try_thisnode)) {
    trynode_flags &= GFP_NOWAIT | __GFP_NOMEMALLOC | __GFP_ACCOUNT;
    trynode_flags |= __GFP_NOWARN | __GFP_THISNODE;
    }
    object = get_from_partial(s, node, trynode_flags, ac);
    if (object) {
// goto;
    }
    slab = new_slab(s, trynode_flags, ac.alloc_flags, node);
    if (unlikely(!slab)) {
    if (node != NUMA_NO_NODE && !(gfpflags & __GFP_THISNODE)
    && try_thisnode) {
    try_thisnode = false;
// goto;
    }
    slab_out_of_memory(s, gfpflags, node);
    return core::ptr::null_mut();
    }
    stat(s, ALLOC_SLAB);
    if (IS_ENABLED!(CONFIG_SLUB_TINY) || kmem_cache_debug(s)) {
    object = alloc_single_from_new_slab(s, slab, ac);
    if (likely(object)) {
// goto;
    }
    } else {
// we don't need to check SLAB_STORE_USER here
    if (alloc_from_new_slab(s, slab, &object, 1, allow_spin)) {
    return object;
    }
    }
    if (allow_spin) {
// goto;
    }
// This could cause an endless loop. Fail instead.
    return core::ptr::null_mut();
// label;
    if (kmem_cache_debug_flags(s, SLAB_STORE_USER)) {
    set_track(s, object, TRACK_ALLOC, ac.caller_addr, gfpflags);
    }
    return object;
    }
#[no_mangle]
unsafe extern "C" fn apply_strict_numa_policy(node: c_int) -> __always_inline int {

    if (static_branch_unlikely(&strict_numa) &&
    node == NUMA_NO_NODE) {
    let mut mpol = current.mempolicy;
    if (mpol) {
//
// Special BIND rule support. If the local node
// is in permitted set then do not redirect
// to a particular node.
// Otherwise we apply the memory policy to get
// the node we need to allocate on.
//
    if (mpol.mode != MPOL_BIND ||
    !node_isset(numa_mem_id(), mpol.nodes)) {
    node = mempolicy_slab_node();
    }
    }
    }

    return node;
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn slab_pre_alloc_hook(s: *mut kmem_cache, flags: gfp_t) -> *mut c_void {
    flags &= gfp_allowed_mask;
    might_alloc(flags);
    if (unlikely(should_failslab(s, flags))) {
    return core::ptr::null_mut();
    }
    return s;
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn slab_post_alloc_hook(s: *mut kmem_cache, flags: gfp_t, size: size_t, p: *mut *mut c_void, ac: *mut slab_alloc_context) -> bool {
pub static mut init: bool = false;
pub static mut zero_size: c_uint = 0;
pub static mut init_flags: gfp_t = 0;
pub static mut kasan_init: bool = false;
//
// For kmalloc object, the allocated size (object_size) can be larger
// than the requested size (orig_size). We however need to zero the
// whole object_size to handle possible later krealloc() with
// __GFP_ZERO properly.
//
// But if we keep track of the requested size, krealloc() uses that
// information. Additionally if red zoning is enabled, the extra space
// is also red zone, so we should not overwrite it. So limit zeroing to
// orig_size if we track it.
//
    if (slub_debug_orig_size(s)) {
    zero_size = ac.orig_size;
    }
//
// ARM64 can set memory tags and zero the memory using a single
// instruction. Since HW_TAGS KASAN uses that while tagging the object,
// separate zeroing is unnecessary.
//
// However, KASAN never zeroes memory when slab_debug is enabled to
// avoid overwriting SLUB redzones. This does not lead to a performance
// penalty on production builds, as slab_debug is not intended to be
// enabled there.
//
    if (kasan_has_integrated_init() && !__slub_debug_enabled()) {
    kasan_init = init;
    init = false;
    }
    while (i < size) {
    p[i] = kasan_slab_alloc(s, p[i], init_flags, kasan_init);
//
// memset and hooks come after KASAN as p[i] might get tagged
//
// kfence zeroes the object instead of SLUB to avoid overwriting
// its own redzone starting at orig_size, which could happen
// with SLUB zeroing full s->object_size
//
    if (init && p[i] && !is_kfence_address(p[i])) {
    memset(p[i], 0, zero_size);
    }
    if (alloc_flags_allow_spinning(ac.alloc_flags)) {
    kmemleak_alloc_recursive(p[i], s.object_size, 1,
    s.flags, init_flags);
    }
    kmsan_slab_alloc(s, p[i], init_flags);
    alloc_tagging_slab_alloc_hook(s, p[i], flags, ac.alloc_flags);
    }
    return memcg_slab_post_alloc_hook(s, flags, size, p, ac);
    }
//
// Replace the empty main sheaf with a (at least partially) full sheaf.
//
// Must be called with the cpu_sheaves local lock locked. If successful, returns
// the pcs pointer and the local lock locked (possibly on a different cpu than
// initially called). If not successful, returns NULL and the local lock
// unlocked.
//
#[no_mangle]
pub unsafe extern "C" fn __pcs_replace_empty_main(s: *mut kmem_cache, pcs: *mut slub_percpu_sheaves, gfp: gfp_t, alloc_flags: c_uint) -> *mut c_void {
    let mut empty = core::ptr::null_mut();
pub static mut full: *mut c_void = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    let mut allow_spin = 0;
    slab_lockdep_assert_held(this_cpu_ptr(&s.cpu_sheaves.lock));
// Bootstrap or debug cache, back off
    if (unlikely(!cache_has_sheaves(s))) {
    local_unlock(&s.cpu_sheaves.lock);
    return core::ptr::null_mut();
    }
    if (pcs.spare && pcs.spare.size > 0) {
    swap(pcs.main, pcs.spare);
    return pcs;
    }
    barn = get_barn(s);
    if (!barn) {
    local_unlock(&s.cpu_sheaves.lock);
    return core::ptr::null_mut();
    }
    allow_spin = alloc_flags_allow_spinning(alloc_flags);
    full = barn_replace_empty_sheaf(barn, pcs.main, allow_spin);
    if (full) {
    stat(s, BARN_GET);
    pcs.main = full;
    return pcs;
    }
    stat(s, BARN_GET_FAIL);
    if (allow_spin) {
    if (pcs.spare) {
    empty = pcs.spare;
    pcs.spare = core::ptr::null_mut();
    } else {
    empty = barn_get_empty_sheaf(barn, true);
    }
    }
    local_unlock(&s.cpu_sheaves.lock);
    pcs = core::ptr::null_mut();
    if (!allow_spin) {
    return core::ptr::null_mut();
    }
    if (!empty) {
    empty = alloc_empty_sheaf(s, gfp, alloc_flags);
    if (!empty) {
    return core::ptr::null_mut();
    }
    }
    if (refill_sheaf(s, empty, gfp | __GFP_NOMEMALLOC | __GFP_NOWARN)) {
//
// we must be very low on memory so don't bother
// with the barn
//
    sheaf_flush_unused(s, empty);
    free_empty_sheaf(s, empty);
    return core::ptr::null_mut();
    }
    full = empty;
    empty = core::ptr::null_mut();
    if (!local_trylock(&s.cpu_sheaves.lock)) {
// goto;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
//
// If we put any empty or full sheaf to the barn below, it's due to
// racing or being migrated to a different cpu. Breaching the barn's
// sheaf limits should be thus rare enough so just ignore them to
// simplify the recovery.
//
    if (pcs.main.size == 0) {
    if (!pcs.spare) {
    pcs.spare = pcs.main;
    }
    else {
    barn_put_empty_sheaf(barn, pcs.main);
    }
    pcs.main = full;
    return pcs;
    }
    if (!pcs.spare) {
    pcs.spare = full;
    return pcs;
    }
    if (pcs.spare.size == 0) {
    barn_put_empty_sheaf(barn, pcs.spare);
    pcs.spare = full;
    return pcs;
    }
// label;
    barn_put_full_sheaf(barn, full);
    stat(s, BARN_PUT);
    return pcs;
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn alloc_from_pcs(s: *mut kmem_cache, gfp: gfp_t, alloc_flags: c_uint, node: c_int) -> *mut c_void {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    let mut node_requested = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    node_requested = IS_ENABLED!(CONFIG_NUMA) && node != NUMA_NO_NODE;
//
// We assume the percpu sheaves contain only local objects although it's
// not completely guaranteed, so we verify later.
//
    if (unlikely(node_requested && node != numa_mem_id())) {
    stat(s, ALLOC_NODE_MISMATCH);
    return core::ptr::null_mut();
    }
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return core::ptr::null_mut();
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (unlikely(pcs.main.size == 0)) {
    pcs = __pcs_replace_empty_main(s, pcs, gfp, alloc_flags);
    if (unlikely(!pcs)) {
    return core::ptr::null_mut();
    }
    }
    object = pcs.main.objects[pcs.main.size - 1];
    if (unlikely(node_requested)) {
//
// Verify that the object was from the node we want. This could
// be false because of cpu migration during an unlocked part of
// the current allocation or previous freeing process.
//
    if (page_to_nid(virt_to_page(object)) != node) {
    local_unlock(&s.cpu_sheaves.lock);
    stat(s, ALLOC_NODE_MISMATCH);
    return core::ptr::null_mut();
    }
    }
    pcs.main.size -= 1;
    local_unlock(&s.cpu_sheaves.lock);
    stat(s, ALLOC_FASTPATH);
    return object;
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn alloc_from_pcs_bulk(s: *mut kmem_cache, size: usize, p: *mut c_void) -> c_uint {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
pub static mut main: *mut c_void = core::ptr::null_mut();
pub static mut allocated: c_uint = 0;
    let mut batch = 0;
// label;
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return allocated;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (unlikely(pcs.main.size == 0)) {
pub static mut full: *mut c_void = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    if (unlikely(!cache_has_sheaves(s))) {
    local_unlock(&s.cpu_sheaves.lock);
    return allocated;
    }
    if (pcs.spare && pcs.spare.size > 0) {
    swap(pcs.main, pcs.spare);
// goto;
    }
    barn = get_barn(s);
    if (!barn) {
    local_unlock(&s.cpu_sheaves.lock);
    return allocated;
    }
    full = barn_replace_empty_sheaf(barn, pcs.main,
// allow_spin = */ true);
    if (full) {
    stat(s, BARN_GET);
    pcs.main = full;
// goto;
    }
    stat(s, BARN_GET_FAIL);
    local_unlock(&s.cpu_sheaves.lock);
//
// Once full sheaves in barn are depleted, let the bulk
// allocation continue from slab pages, otherwise we would just
// be copying arrays of pointers twice.
//
    return allocated;
    }
// label;
    main = pcs.main;
    batch = min(size, main.size);
    main.size -= batch;
    memcpy(p, main.objects + main.size, batch * sizeof!);
    local_unlock(&s.cpu_sheaves.lock);
    stat_add(s, ALLOC_FASTPATH, batch);
    allocated += batch;
    if (batch < size) {
    p += batch;
    size -= batch;
// goto;
    }
    return allocated;
    }
//
// Inlined fastpath so that allocation functions (kmalloc, kmem_cache_alloc)
// have the fastpath folded into their functions. So no function call
// overhead for requests that can be satisfied on the fastpath.
//
// The fastpath works by first checking if the lockless freelist can be used.
// If not then __slab_alloc is called for slow processing.
//
// Otherwise we can simply pick the next object from the lockless free list.
//
    static __fastpath_inline void *slab_alloc_node(kmem_cache *s,
    gfp_t gfpflags, int node, const struct slab_alloc_context *ac)
    {
pub static mut object: *mut c_void = core::ptr::null_mut();
    s = slab_pre_alloc_hook(s, gfpflags);
    if (unlikely(!s)) {
    return core::ptr::null_mut();
    }
    object = kfence_alloc(s, ac.orig_size, gfpflags);
    if (unlikely(object)) {
// goto;
    }
    node = apply_strict_numa_policy(node);
    object = alloc_from_pcs(s, gfpflags, ac.alloc_flags, node);
    if (unlikely(!object)) {
    object = ___slab_alloc(s, gfpflags, node, ac);
    }
    maybe_wipe_obj_freeptr(s, object);
// label;
//
// In case this fails due to memcg_slab_post_alloc_hook(),
// object is set to NULL
//
    slab_post_alloc_hook(s, gfpflags, 1, &object, ac);
    return object;
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_alloc_noprof(s: *mut kmem_cache, gfpflags: gfp_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    ret = slab_alloc_node(s, gfpflags, NUMA_NO_NODE, &ac);
    trace_kmem_cache_alloc(_RET_IP_, ret, s, gfpflags, NUMA_NO_NODE);
    return ret;
    }
    EXPORT_SYMBOL(kmem_cache_alloc_noprof);
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_alloc_lru_noprof(s: *mut kmem_cache, lru: *mut list_lru, gfpflags: gfp_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    ret = slab_alloc_node(s, gfpflags, NUMA_NO_NODE, &ac);
    trace_kmem_cache_alloc(_RET_IP_, ret, s, gfpflags, NUMA_NO_NODE);
    return ret;
    }
    EXPORT_SYMBOL(kmem_cache_alloc_lru_noprof);
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_charge(objp: *mut c_void, gfpflags: gfp_t) -> bool {
    if (!memcg_kmem_online()) {
    return true;
    }
    return memcg_slab_post_charge(objp, gfpflags);
    }
    EXPORT_SYMBOL(kmem_cache_charge);
//
// kmem_cache_alloc_node - Allocate an object on the specified node
// @s: The cache to allocate from.
// @gfpflags: See kmalloc().
// @node: node number of the target node.
//
// Identical to kmem_cache_alloc but it will allocate memory on the given
// node, which can improve the performance for cpu bound structures.
//
// Fallback to other node is possible if __GFP_THISNODE is not set.
//
// Return: pointer to the new object or %NULL in case of error
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_alloc_node_noprof(s: *mut kmem_cache, gfpflags: gfp_t, node: c_int) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    ret = slab_alloc_node(s, gfpflags, node, &ac);
    trace_kmem_cache_alloc(_RET_IP_, ret, s, gfpflags, node);
    return ret;
    }
    EXPORT_SYMBOL(kmem_cache_alloc_node_noprof);
#[no_mangle]
pub unsafe extern "C" fn __prefill_sheaf_pfmemalloc(s: *mut kmem_cache, sheaf: *mut slab_sheaf, gfp: gfp_t) -> c_int {
    let mut gfp_nomemalloc;
    let mut ret = 0;
    gfp_nomemalloc = gfp | __GFP_NOMEMALLOC;
    if (gfp_pfmemalloc_allowed(gfp)) {
    gfp_nomemalloc |= __GFP_NOWARN;
    }
    ret = refill_sheaf(s, sheaf, gfp_nomemalloc);
    if (likely(!ret || !gfp_pfmemalloc_allowed(gfp))) {
    return ret;
    }
//
// if we are allowed to, refill sheaf with pfmemalloc but then remember
// it for when it's returned
//
    ret = refill_sheaf(s, sheaf, gfp);
    sheaf.pfmemalloc = true;
    return ret;
    }
// forward_decl: __kmem_cache_alloc_bulk;
//
// returns a sheaf that has at least the requested size
// when prefilling is needed, do so with given gfp flags
//
// return NULL if sheaf allocation or prefilling failed
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_prefill_sheaf(s: *mut kmem_cache, gfp: gfp_t, size: c_uint) -> *mut c_void {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    let mut sheaf = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    if (unlikely(!size)) {
    return core::ptr::null_mut();
    }
    if (unlikely(size > s.sheaf_capacity)) {
    sheaf = __alloc_empty_sheaf(s, gfp, SLAB_ALLOC_DEFAULT, size);
    if (!sheaf) {
    return core::ptr::null_mut();
    }
    stat(s, SHEAF_PREFILL_OVERSIZE);
    sheaf.capacity = size;
//
// we do not need to care about pfmemalloc here because oversize
// sheaves are always flushed and freed when returned
//
    if (!__kmem_cache_alloc_bulk(s, gfp, size,
    &sheaf.objects[0])) {
    free_empty_sheaf(s, sheaf);
    return core::ptr::null_mut();
    }
    sheaf.size = size;
    return sheaf;
    }
    local_lock(&s.cpu_sheaves.lock);
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (pcs.spare) {
    sheaf = pcs.spare;
    pcs.spare = core::ptr::null_mut();
    stat(s, SHEAF_PREFILL_FAST);
    } else {
    barn = get_barn(s);
    stat(s, SHEAF_PREFILL_SLOW);
    if (barn) {
    sheaf = barn_get_full_or_empty_sheaf(barn);
    }
    if (sheaf && sheaf.size) {
    stat(s, BARN_GET);
    }
    else {
    stat(s, BARN_GET_FAIL);
    }
    }
    local_unlock(&s.cpu_sheaves.lock);
    if (!sheaf) {
    sheaf = alloc_empty_sheaf(s, gfp, SLAB_ALLOC_DEFAULT);
    }
    if (sheaf) {
    sheaf.capacity = s.sheaf_capacity;
    sheaf.pfmemalloc = false;
    if (sheaf.size < size &&
    __prefill_sheaf_pfmemalloc(s, sheaf, gfp)) {
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
    sheaf = core::ptr::null_mut();
    }
    }
    return sheaf;
    }
//
// Use this to return a sheaf obtained by kmem_cache_prefill_sheaf()
//
// If the sheaf cannot simply become the percpu spare sheaf, but there's space
// for a full sheaf in the barn, we try to refill the sheaf back to the cache's
// sheaf_capacity to avoid handling partially full sheaves.
//
// If the refill fails because gfp is e.g. GFP_NOWAIT, or the barn is full, the
// sheaf is instead flushed and freed.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_return_sheaf(s: *mut kmem_cache, gfp: gfp_t, sheaf: *mut slab_sheaf) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    if (unlikely((sheaf.capacity != s.sheaf_capacity)
    || sheaf.pfmemalloc)) {
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
    return;
    }
    local_lock(&s.cpu_sheaves.lock);
    pcs = this_cpu_ptr(s.cpu_sheaves);
    barn = get_barn(s);
    if (!pcs.spare) {
    pcs.spare = sheaf;
    sheaf = core::ptr::null_mut();
    stat(s, SHEAF_RETURN_FAST);
    }
    local_unlock(&s.cpu_sheaves.lock);
    if (!sheaf) {
    return;
    }
    stat(s, SHEAF_RETURN_SLOW);
//
// If the barn has too many full sheaves or we fail to refill the sheaf,
// simply flush and free it.
//
    if (!barn || data_race(barn.nr_full) >= MAX_FULL_SHEAVES ||
    refill_sheaf(s, sheaf, gfp | __GFP_NOMEMALLOC | __GFP_NOWARN)) {
    sheaf_flush_unused(s, sheaf);
    free_empty_sheaf(s, sheaf);
    return;
    }
    barn_put_full_sheaf(barn, sheaf);
    stat(s, BARN_PUT);
    }
//
// Refill a sheaf previously returned by kmem_cache_prefill_sheaf to at least
// the given size.
//
// Return: 0 on success. The sheaf will contain at least @size objects.
// The sheaf might have been replaced with a new one if more than
// sheaf->capacity objects are requested.
//
// Return: -ENOMEM on failure. Some objects might have been added to the sheaf
// but the sheaf will not be replaced.
//
// In practice we always refill to full sheaf's capacity.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_refill_sheaf(s: *mut kmem_cache, gfp: gfp_t, sheafp: *mut *mut slab_sheaf, size: c_uint) -> c_int {
pub static mut sheaf: *mut c_void = core::ptr::null_mut();
//
// TODO: do we want to support *sheaf == NULL to be equivalent of
// kmem_cache_prefill_sheaf() ?
//
    if (!sheafp || !(*sheafp)) {
    return -EINVAL;
    }
    sheaf = *sheafp;
    if (sheaf.size >= size) {
    return 0;
    }
    if (likely(sheaf.capacity >= size)) {
    if (likely(sheaf.capacity == s.sheaf_capacity)) {
    return __prefill_sheaf_pfmemalloc(s, sheaf, gfp);
    }
    if (!__kmem_cache_alloc_bulk(s, gfp, sheaf.capacity - sheaf.size,
    &sheaf.objects[sheaf.size])) {
    return -ENOMEM;
    }
    sheaf.size = sheaf.capacity;
    return 0;
    }
//
// We had a regular sized sheaf and need an oversize one, or we had an
// oversize one already but need a larger one now.
// This should be a very rare path so let's not complicate it.
//
    sheaf = kmem_cache_prefill_sheaf(s, gfp, size);
    if (!sheaf) {
    return -ENOMEM;
    }
    kmem_cache_return_sheaf(s, gfp, *sheafp);
// sheafp = sheaf;
    return 0;
    }
//
// Allocate from a sheaf obtained by kmem_cache_prefill_sheaf()
//
// Guaranteed not to fail as many allocations as was the requested size.
// After the sheaf is emptied, it fails - no fallback to the slab cache itself.
//
// The gfp parameter is meant only to specify __GFP_ZERO or __GFP_ACCOUNT
// memcg charging is forced over limit if necessary, to avoid failure.
//
// It is possible that the allocation comes from kfence and then the sheaf
// size is not decreased.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_alloc_from_sheaf_noprof(s: *mut kmem_cache, gfp: gfp_t, sheaf: *mut slab_sheaf) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    if (sheaf.size == 0) {
// goto;
    }
    ret = kfence_alloc(s, s.object_size, gfp);
    if (likely(!ret)) {
    ret = sheaf.objects[--sheaf.size];
    }
// add __GFP_NOFAIL to force successful memcg charging
    slab_post_alloc_hook(s, gfp | __GFP_NOFAIL, 1, &ret, &ac);
// label;
    trace_kmem_cache_alloc(_RET_IP_, ret, s, gfp, NUMA_NO_NODE);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_sheaf_size(sheaf: *mut slab_sheaf) -> c_uint {
    return sheaf.size;
    }
//
// To avoid unnecessary overhead, we pass through large allocation requests
// directly to the page allocator. We use __GFP_COMP, because we will need to
// know the allocation order to free the pages properly in kfree.
//
#[no_mangle]
pub unsafe extern "C" fn ___kmalloc_large_node(size: size_t, flags: gfp_t, node: c_int) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut ptr = core::ptr::null_mut();
pub static mut order: c_uint = 0;
    if (unlikely(flags & GFP_SLAB_BUG_MASK)) {
    flags = kmalloc_fix_flags(flags);
    }
    flags |= __GFP_COMP;
    if (node == NUMA_NO_NODE) {
    page = alloc_frozen_pages_noprof(flags, order);
    }
    else {
    page = __alloc_frozen_pages_noprof(flags, order, node, core::ptr::null_mut(),
    ALLOC_DEFAULT);
    }
    if (page) {
    ptr = page_address(page);
    mod_lruvec_page_state(page, NR_SLAB_UNRECLAIMABLE_B,
    PAGE_SIZE << order);
    __SetPageLargeKmalloc(page);
    }
    ptr = kasan_kmalloc_large(ptr, size, flags);
// As ptr might get tagged, call kmemleak hook after KASAN.
    kmemleak_alloc(ptr, size, 1, flags);
    kmsan_kmalloc_large(ptr, size, flags);
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_large_noprof(size: size_t, flags: gfp_t) -> *mut c_void {
    let mut ret = ___kmalloc_large_node(size, flags, NUMA_NO_NODE);
    trace_kmalloc(_RET_IP_, ret, size, PAGE_SIZE << get_order(size),
    flags, NUMA_NO_NODE);
    return ret;
    }
    EXPORT_SYMBOL(__kmalloc_large_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_large_node_noprof(size: size_t, flags: gfp_t, node: c_int) -> *mut c_void {
    let mut ret = ___kmalloc_large_node(size, flags, node);
    trace_kmalloc(_RET_IP_, ret, size, PAGE_SIZE << get_order(size),
    flags, node);
    return ret;
    }
    EXPORT_SYMBOL(__kmalloc_large_node_noprof);
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn __do_kmalloc_node(b: *mut kmem_buckets, flags: gfp_t, node: c_int, token: kmalloc_token_t, ac: *mut slab_alloc_context) -> *mut c_void {
pub static mut size: usize = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (unlikely(size > KMALLOC_MAX_CACHE_SIZE)) {
    ret = __kmalloc_large_node_noprof(size, flags, node);
    trace_kmalloc(ac.caller_addr, ret, size,
    PAGE_SIZE << get_order(size), flags, node);
    return ret;
    }
    if (unlikely(!size)) {
    return ZERO_SIZE_PTR;
    }
    s = kmalloc_slab(size, b, flags, token, ac.alloc_flags);
    ret = slab_alloc_node(s, flags, node, ac);
    ret = kasan_kmalloc(s, ret, size, flags);
    trace_kmalloc(ac.caller_addr, ret, size, s.size, flags, node);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_node_noprof(size: DECL_KMALLOC_PARAMS(, b: c_void, flags: gfp_t, node: c_int) -> *mut c_void {
pub static mut slab_alloc_context: usize = 0;
    return __do_kmalloc_node(PASS_BUCKET_PARAM(b), flags, node,
    PASS_TOKEN_PARAM(token), &ac);
    }
    EXPORT_SYMBOL(__kmalloc_node_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_noprof(size: DECL_TOKEN_PARAMS(, flags: gfp_t) -> *mut c_void {
pub static mut slab_alloc_context: usize = 0;
    return __do_kmalloc_node(core::ptr::null_mut(), flags,  NUMA_NO_NODE,
    PASS_TOKEN_PARAM(token), &ac);
    }
    EXPORT_SYMBOL(__kmalloc_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_nolock_noprof(size: DECL_TOKEN_PARAMS(, gfp_flags: gfp_t, node: c_int, ac: *mut slab_alloc_context) -> *mut c_void {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut can_retry: bool = true;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_ONCE(alloc_flags_allow_spinning(ac.alloc_flags));
    VM_WARN_ON_ONCE(gfp_flags & ~(__GFP_ACCOUNT | __GFP_ZERO |
    __GFP_NOWARN | __GFP_NOMEMALLOC));
    gfp_flags |= __GFP_NOWARN | __GFP_NOMEMALLOC;
    if (unlikely(!size)) {
    return ZERO_SIZE_PTR;
    }
    if (!can_spin_trylock()) {
    return core::ptr::null_mut();
    }
    node = apply_strict_numa_policy(node);
// label;
    if (unlikely(size > KMALLOC_MAX_CACHE_SIZE)) {
    return core::ptr::null_mut();
    }
    s = kmalloc_slab(size, core::ptr::null_mut(), gfp_flags, PASS_TOKEN_PARAM(token),
    ac.alloc_flags);
    if (!(s.flags & __CMPXCHG_DOUBLE) && !kmem_cache_debug(s)) {
//
// kmalloc_nolock() is not supported on architectures that
// don't implement cmpxchg16b and thus need slab_lock()
// which could be preempted by a nmi.
// But debug caches don't use that and only rely on
// kmem_cache_node->list_lock, so kmalloc_nolock() can attempt
// to allocate from debug caches by
// spin_trylock_irqsave(&n->list_lock, ...)
//
    return core::ptr::null_mut();
    }
    ret = alloc_from_pcs(s, gfp_flags, ac.alloc_flags, node);
    if (ret) {
// goto;
    }
//
// Do not call slab_alloc_node(), since trylock mode isn't
// compatible with slab_pre_alloc_hook/should_failslab and
// kfence_alloc. Hence call ___slab_alloc() (at most twice)
// and slab_post_alloc_hook() directly.
//
    ret = ___slab_alloc(s, gfp_flags, node, ac);
//
// It's possible we failed due to trylock as we preempted someone with
// the sheaves locked, and the list_lock is also held by another cpu.
// But it should be rare that multiple kmalloc buckets would have
// sheaves locked, so try a larger one.
//
    if (!ret && can_retry) {
// pick the next kmalloc bucket
    size = s.object_size + 1;
//
// Another alternative is to
// if (memcg) gfp_flags &= ~__GFP_ACCOUNT;
// else if (!memcg) gfp_flags |= __GFP_ACCOUNT; {
// to retry from bucket of the same size.
//
    can_retry = false;
}
// goto;
    }
// label;
    maybe_wipe_obj_freeptr(s, ret);
    slab_post_alloc_hook(s, gfp_flags, 1, &ret, ac);
    ret = kasan_kmalloc(s, ret, ac.orig_size, gfp_flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn _kmalloc_nolock_noprof(size: DECL_TOKEN_PARAMS(, gfp_flags: gfp_t, node: c_int) -> *mut c_void {
pub static mut slab_alloc_context: usize = 0;
    return __kmalloc_nolock_noprof(PASS_TOKEN_PARAMS(size, token),
    gfp_flags, node, &ac);
    }
    EXPORT_SYMBOL_GPL(_kmalloc_nolock_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_node_track_caller_noprof(size: DECL_KMALLOC_PARAMS(, b: c_void, flags: gfp_t, node: c_int, caller: c_ulong) -> *mut c_void {
pub static mut slab_alloc_context: usize = 0;
    return __do_kmalloc_node(PASS_BUCKET_PARAM(b), flags, node,
    PASS_TOKEN_PARAM(token), &ac);
    }
    EXPORT_SYMBOL(__kmalloc_node_track_caller_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_cache_noprof(s: *mut kmem_cache, gfpflags: gfp_t, size: size_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    ret = slab_alloc_node(s, gfpflags, NUMA_NO_NODE, &ac);
    trace_kmalloc(_RET_IP_, ret, size, s.size, gfpflags, NUMA_NO_NODE);
    ret = kasan_kmalloc(s, ret, size, gfpflags);
    return ret;
    }
    EXPORT_SYMBOL(__kmalloc_cache_noprof);
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_cache_node_noprof(s: *mut kmem_cache, gfpflags: gfp_t, node: c_int, size: size_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    ret = slab_alloc_node(s, gfpflags, node, &ac);
    trace_kmalloc(_RET_IP_, ret, size, s.size, gfpflags, node);
    ret = kasan_kmalloc(s, ret, size, gfpflags);
    return ret;
    }
    EXPORT_SYMBOL(__kmalloc_cache_node_noprof);
//
// The only version of kmalloc_node() that takes alloc_flags and thus can
// determine on its own whether to handle the allocation via kmalloc_nolock() or
// normally
//
#[no_mangle]
pub unsafe extern "C" fn __kmalloc_flags_noprof(size: DECL_TOKEN_PARAMS(, flags: gfp_t, alloc_flags: c_uint, node: c_int) -> *mut c_void {
pub static mut slab_alloc_context: usize = 0;
    if (alloc_flags_allow_spinning(alloc_flags)) {
    return __do_kmalloc_node(core::ptr::null_mut(), flags, node,
    PASS_TOKEN_PARAM(token), &ac);
    } else {
    return __kmalloc_nolock_noprof(PASS_TOKEN_PARAMS(size, token),
    flags, node, &ac);
    }
    }
    static noinline void free_to_partial_list(kmem_cache *s, slab *slab,
    void *head, void *tail, int bulk_cnt,
    unsigned long addr)
    {
    let mut n = get_node(s, slab_nid(slab));
    let mut slab_free = core::ptr::null_mut();
pub static mut cnt: c_int = 0;
    let mut flags = 0;
pub static mut handle: depot_stack_handle_t = 0;
//
// We cannot use GFP_NOWAIT as there are callsites where waking up
// kswapd could deadlock
//
    if (s.flags & SLAB_STORE_USER) {
    handle = set_track_prepare(__GFP_NOWARN);
    }
    spin_lock_irqsave(&n.list_lock, flags);
    if (free_debug_processing(s, slab, head, tail, &cnt, addr, handle)) {
    let mut prior = slab.freelist;
// Perform the actual freeing while we still hold the locks
    slab.inuse -= cnt;
    set_freepointer(s, tail, prior);
    slab.freelist = head;
//
// If the slab is empty, and node's partial list is full,
// it should be discarded anyway no matter it's on full or
// partial list.
//
    if (slab.inuse == 0 && n.nr_partial >= s.min_partial) {
    slab_free = slab;
    }
    if (!prior) {
// was on full list
    remove_full(s, n, slab);
    if (!slab_free) {
    add_partial(n, slab, ADD_TO_TAIL);
    stat(s, FREE_ADD_PARTIAL);
    }
    } else if (slab_free) {
    remove_partial(n, slab);
    stat(s, FREE_REMOVE_PARTIAL);
    }
    }
    if (slab_free) {
//
// Update the counters while still holding n->list_lock to
// prevent spurious validation warnings
//
    dec_slabs_node(s, slab_nid(slab_free), slab_free.objects);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    if (slab_free) {
    stat(s, FREE_SLAB);
    free_slab(s, slab_free);
    }
    }
//
// Try returning (remainder of) the freelist that we just detached from the
// slab.  Optimistically assume the slab is still full, so we don't need to find
// the tail of the detached freelist.
//
// Fail if the slab isn't full anymore due to a concurrent free.
//
#[no_mangle]
pub unsafe extern "C" fn __slab_try_return_freelist(s: *mut kmem_cache, slab: *mut slab, head: *mut c_void, cnt: c_int) -> bool {
    struct freelist_counters old, new;
    old.freelist = slab.freelist;
    old.counters = slab.counters;
    if (old.freelist) {
    return false;
    }
    new.freelist = head;
    new.counters = old.counters;
    new.inuse -= cnt;
    if (!slab_update_freelist(s, slab, &old, &new, "__slab_try_return_freelist")) {
    return false;
    }
    return true;
    }
//
// Slow path handling. This may still be called frequently since objects
// have a longer lifetime than the cpu slabs in most processing loads.
//
// So we still attempt to reduce cache line usage. Just take the slab
// lock and free the item. If there is no additional partial slab
// handling required then we can return immediately.
//
#[no_mangle]
pub unsafe extern "C" fn __slab_free(s: *mut kmem_cache, slab: *mut slab, head: *mut c_void, tail: *mut c_void, cnt: c_int, addr: c_ulong) {
    let mut was_full = 0;
    struct freelist_counters old, new;
    let mut n = core::ptr::null_mut();
    let mut flags = 0;
    let mut on_node_partial = 0;
    if (IS_ENABLED!(CONFIG_SLUB_TINY) || kmem_cache_debug(s)) {
    free_to_partial_list(s, slab, head, tail, cnt, addr);
    return;
    }
    do {
    if (unlikely(n)) {
    spin_unlock_irqrestore(&n.list_lock, flags);
    n = core::ptr::null_mut();
    }
    old.freelist = slab.freelist;
    old.counters = slab.counters;
    was_full = (old.freelist == core::ptr::null_mut());
    set_freepointer(s, tail, old.freelist);
    new.freelist = head;
    new.counters = old.counters;
    new.inuse -= cnt;
//
// Might need to be taken off (due to becoming empty) or added
// to (due to not being full anymore) the partial list.
// Unless it's frozen.
//
    if (!new.inuse || was_full) {
    n = get_node(s, slab_nid(slab));
//
// Speculatively acquire the list_lock.
// If the cmpxchg does not succeed then we may
// drop the list_lock without any processing.
//
// Otherwise the list_lock will synchronize with
// other processors updating the list of slabs.
//
    spin_lock_irqsave(&n.list_lock, flags);
    on_node_partial = slab_test_node_partial(slab);
    }
    } while (!slab_update_freelist(s, slab, &old, &new, "__slab_free"));
    if (likely(!n)) {
//
// We didn't take the list_lock because the slab was already on
// the partial list and will remain there.
//
    return;
    }
//
// This slab was partially empty but not on the per-node partial list,
// in which case we shouldn't manipulate its list, just return.
//
    if (!was_full && !on_node_partial) {
    spin_unlock_irqrestore(&n.list_lock, flags);
    return;
    }
//
// If slab became empty, should we add/keep it on the partial list or we
// have enough?
//
    if (unlikely(!new.inuse && n.nr_partial >= s.min_partial)) {
// goto;
    }
//
// Objects left in the slab. If it was not on the partial list before
// then add it.
//
    if (unlikely(was_full)) {
    add_partial(n, slab, ADD_TO_TAIL);
    stat(s, FREE_ADD_PARTIAL);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    return;
// label;
//
// The slab could have a single object and thus go from full to empty in
// a single free, but more likely it was on the partial list. Remove it.
//
    if (likely(!was_full)) {
    remove_partial(n, slab);
    stat(s, FREE_REMOVE_PARTIAL);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    stat(s, FREE_SLAB);
    discard_slab(s, slab);
    }
//
// pcs is locked. We should have get rid of the spare sheaf and obtained an
// empty sheaf, while the main sheaf is full. We want to install the empty sheaf
// as a main sheaf, and make the current main sheaf a spare sheaf.
//
// However due to having relinquished the cpu_sheaves lock when obtaining
// the empty sheaf, we need to handle some unlikely but possible cases.
//
// If we put any sheaf to barn here, it's because we were interrupted or have
// been migrated to a different cpu, which should be rare enough so just ignore
// the barn's limits to simplify the handling.
//
// An alternative scenario that gets us here is when we fail
// barn_replace_full_sheaf(), because there's no empty sheaf available in the
// barn, so we had to allocate it by alloc_empty_sheaf(). But because we saw the
// limit on full sheaves was not exceeded, we assume it didn't change and just
// put the full sheaf there.
//
#[no_mangle]
pub unsafe extern "C" fn __pcs_install_empty_sheaf(s: *mut kmem_cache, pcs: *mut slub_percpu_sheaves, empty: *mut slab_sheaf, barn: *mut node_barn) {
    slab_lockdep_assert_held(this_cpu_ptr(&s.cpu_sheaves.lock));
// This is what we expect to find if nobody interrupted us.
    if (likely(!pcs.spare)) {
    pcs.spare = pcs.main;
    pcs.main = empty;
    return;
    }
//
// Unlikely because if the main sheaf had space, we would have just
// freed to it. Get rid of our empty sheaf.
//
    if (pcs.main.size < s.sheaf_capacity) {
    barn_put_empty_sheaf(barn, empty);
    return;
    }
// Also unlikely for the same reason
    if (pcs.spare.size < s.sheaf_capacity) {
    swap(pcs.main, pcs.spare);
    barn_put_empty_sheaf(barn, empty);
    return;
    }
//
// We probably failed barn_replace_full_sheaf() due to no empty sheaf
// available there, but we allocated one, so finish the job.
//
    barn_put_full_sheaf(barn, pcs.main);
    stat(s, BARN_PUT);
    pcs.main = empty;
    }
//
// Replace the full main sheaf with a (at least partially) empty sheaf.
//
// Must be called with the cpu_sheaves local lock locked. If successful, returns
// the pcs pointer and the local lock locked (possibly on a different cpu than
// initially called). If not successful, returns NULL and the local lock
// unlocked.
//
#[no_mangle]
pub unsafe extern "C" fn __pcs_replace_full_main(s: *mut kmem_cache, pcs: *mut slub_percpu_sheaves, allow_spin: bool) -> *mut c_void {
pub static mut empty: *mut c_void = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    let mut put_fail = 0;
// label;
    slab_lockdep_assert_held(this_cpu_ptr(&s.cpu_sheaves.lock));
// Bootstrap or debug cache, back off
    if (unlikely(!cache_has_sheaves(s))) {
    local_unlock(&s.cpu_sheaves.lock);
    return core::ptr::null_mut();
    }
    barn = get_barn(s);
    if (!barn) {
    local_unlock(&s.cpu_sheaves.lock);
    return core::ptr::null_mut();
    }
    put_fail = false;
    if (!pcs.spare) {
    empty = barn_get_empty_sheaf(barn, allow_spin);
    if (empty) {
    pcs.spare = pcs.main;
    pcs.main = empty;
    return pcs;
    }
// goto;
    }
    if (pcs.spare.size < s.sheaf_capacity) {
    swap(pcs.main, pcs.spare);
    return pcs;
    }
    empty = barn_replace_full_sheaf(barn, pcs.main, allow_spin);
    if (!IS_ERR(empty)) {
    stat(s, BARN_PUT);
    pcs.main = empty;
    return pcs;
    }
// sheaf_flush_unused() doesn't support !allow_spin
    if (PTR_ERR(empty) == -E2BIG && allow_spin) {
// Since we got here, spare exists and is full
    let mut to_flush = pcs.spare;
    stat(s, BARN_PUT_FAIL);
    pcs.spare = core::ptr::null_mut();
    local_unlock(&s.cpu_sheaves.lock);
    sheaf_flush_unused(s, to_flush);
    empty = to_flush;
// goto;
    }
//
// We could not replace full sheaf because barn had no empty
// sheaves. We can still allocate it and put the full sheaf in
// __pcs_install_empty_sheaf(), but if we fail to allocate it,
// make sure to count the fail.
//
    put_fail = true;
// label;
    local_unlock(&s.cpu_sheaves.lock);
//
// alloc_empty_sheaf() doesn't support !allow_spin and it's
// easier to fall back to freeing directly without sheaves
// than add the support (and to sheaf_flush_unused() above)
//
    if (!allow_spin) {
    return core::ptr::null_mut();
    }
    empty = alloc_empty_sheaf(s, GFP_NOWAIT, SLAB_ALLOC_DEFAULT);
    if (empty) {
// goto;
    }
    if (put_fail) {
    stat(s, BARN_PUT_FAIL);
    }
    if (!sheaf_try_flush_main(s)) {
    return core::ptr::null_mut();
    }
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return core::ptr::null_mut();
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
//
// we flushed the main sheaf so it should be empty now,
// but in case we got preempted or migrated, we need to
// check again
//
    if (pcs.main.size == s.sheaf_capacity) {
// goto;
    }
    return pcs;
// label;
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    barn_put_empty_sheaf(barn, empty);
    return core::ptr::null_mut();
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    __pcs_install_empty_sheaf(s, pcs, empty, barn);
    return pcs;
    }
//
// Free an object to the percpu sheaves.
// The object is expected to have passed slab_free_hook() already.
//
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn free_to_pcs(s: *mut kmem_cache, object: *mut c_void, allow_spin: bool) -> bool {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return false;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (unlikely(pcs.main.size == s.sheaf_capacity)) {
    pcs = __pcs_replace_full_main(s, pcs, allow_spin);
    if (unlikely(!pcs)) {
    return false;
    }
    }
    pcs.main.objects[pcs.main.size++] = object;
    local_unlock(&s.cpu_sheaves.lock);
    stat(s, FREE_FASTPATH);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn rcu_free_sheaf(head: *mut rcu_head) {
pub static mut sheaf: *mut c_void = core::ptr::null_mut();
    let mut barn = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    sheaf = container_of!(head, slab_sheaf, rcu_head);
    s = sheaf.cache;
//
// This may remove some objects due to slab_free_hook() returning false,
// so that the sheaf might no longer be completely full. But it's easier
// to handle it as full (unless it became completely empty), as the code
// handles it fine. The only downside is that sheaf will serve fewer
// allocations when reused. It only happens due to debugging, which is a
// performance hit anyway.
//
// If it returns true, there was at least one object from pfmemalloc
// slab so simply flush everything.
//
    if (__rcu_free_sheaf_prepare(s, sheaf)) {
// goto;
    }
    barn = get_barn_node(s, sheaf.node);
    if (!barn) {
// goto;
    }
// due to slab_free_hook()
    if (unlikely(sheaf.size == 0)) {
// goto;
    }
//
// Checking nr_full/nr_empty outside lock avoids contention in case the
// barn is at the respective limit. Due to the race we might go over the
// limit but that should be rare and harmless.
//
    if (data_race(barn.nr_full) < MAX_FULL_SHEAVES) {
    stat(s, BARN_PUT);
    barn_put_full_sheaf(barn, sheaf);
    return;
    }
// label;
    stat(s, BARN_PUT_FAIL);
    sheaf_flush_unused(s, sheaf);
// label;
    if (barn && data_race(barn.nr_empty) < MAX_EMPTY_SHEAVES) {
    barn_put_empty_sheaf(barn, sheaf);
    return;
    }
    free_empty_sheaf(s, sheaf);
    }
//
// kvfree_call_rcu() can be called while holding a raw_spinlock_t. Since
// __kfree_rcu_sheaf() may acquire a spinlock_t (sleeping lock on PREEMPT_RT),
// this would violate lock nesting rules. Therefore, kvfree_call_rcu() avoids
// this problem by passing SLAB_FREE_NOLOCK on PREEMPT_RT.
//
// However, lockdep still complains that it is invalid to acquire spinlock_t
// while holding raw_spinlock_t, even on !PREEMPT_RT where spinlock_t is a
// spinning lock. Tell lockdep that acquiring spinlock_t is valid here
// by temporarily raising the wait-type to LD_WAIT_CONFIG. Skip the lockdep map
// on PREEMPT_RT to avoid suppressing valid lockdep warnings.
//
pub static mut kfree_rcu_sheaf_map: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __kfree_rcu_sheaf(s: *mut kmem_cache, obj: *mut c_void, free_flags: c_uint) -> bool {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
pub static mut rcu_sheaf: *mut c_void = core::ptr::null_mut();
pub static mut allow_spin: bool = false;
    VM_WARN_ON_ONCE(IS_ENABLED!(CONFIG_PREEMPT_RT) && allow_spin);
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    lock_map_acquire_try(&kfree_rcu_sheaf_map);
    }
    if (!local_trylock(&s.cpu_sheaves.lock)) {
// goto;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (unlikely(!pcs.rcu_free)) {
pub static mut empty: *mut c_void = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
pub static mut alloc_flags: c_uint = 0;
pub static mut gfp: gfp_t = 0;
// Bootstrap or debug cache, fall back
    if (unlikely(!cache_has_sheaves(s))) {
    local_unlock(&s.cpu_sheaves.lock);
// goto;
    }
    if (pcs.spare && pcs.spare.size == 0) {
    pcs.rcu_free = pcs.spare;
    pcs.spare = core::ptr::null_mut();
// goto;
    }
    barn = get_barn(s);
    if (!barn) {
    local_unlock(&s.cpu_sheaves.lock);
// goto;
    }
    empty = barn_get_empty_sheaf(barn, allow_spin);
    if (empty) {
    pcs.rcu_free = empty;
// goto;
    }
    local_unlock(&s.cpu_sheaves.lock);
    empty = alloc_empty_sheaf(s, gfp, alloc_flags);
    if (!empty) {
// goto;
    }
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    __free_empty_sheaf(s, empty, free_flags);
// goto;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (unlikely(pcs.rcu_free)) {
    __free_empty_sheaf(s, empty, free_flags);
    }
    else {
    pcs.rcu_free = empty;
    }
    }
// label;
    rcu_sheaf = pcs.rcu_free;
//
// Since we flush immediately when size reaches capacity, we never reach
// this with size already at capacity, so no OOB write is possible.
//
    rcu_sheaf.objects[rcu_sheaf.size++] = obj;
    if (likely(rcu_sheaf.size < s.sheaf_capacity)) {
    rcu_sheaf = core::ptr::null_mut();
    } else {
    pcs.rcu_free = core::ptr::null_mut();
    rcu_sheaf.node = numa_node_id();
    }
//
// we flush before local_unlock to make sure a racing
// flush_all_rcu_sheaves() doesn't miss this sheaf
//
    if (rcu_sheaf) {
//
// With !allow_spin, we might have interrupted call_rcu()'s
// IRQ-disabled critical section. If IRQs are not disabled,
// we know that's not the case.
//
    if (unlikely(!allow_spin && irqs_disabled())) {
pub static mut dpw: *mut c_void = core::ptr::null_mut();
    dpw = this_cpu_ptr(&deferred_percpu_work);
    if (llist_add(&rcu_sheaf.llnode, &dpw.rcu_sheaves)) {
    irq_work_queue(&dpw.work);
    }
    } else {
    call_rcu(&rcu_sheaf.rcu_head, rcu_free_sheaf);
    }
    }
    local_unlock(&s.cpu_sheaves.lock);
    stat(s, FREE_RCU_SHEAF);
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    lock_map_release(&kfree_rcu_sheaf_map);
    }
    return true;
// label;
    stat(s, FREE_RCU_SHEAF_FAIL);
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT)) {
    lock_map_release(&kfree_rcu_sheaf_map);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn can_free_to_pcs(slab: *mut slab) -> __always_inline bool {
    let mut slab_node = 0;
    let mut numa_node = 0;
    if (!IS_ENABLED!(CONFIG_NUMA)) {
// goto;
    }
    slab_node = slab_nid(slab);

//
// numa_mem_id() points to the closest node with memory so only allow
// objects from that node to the percpu sheaves
//
    numa_node = numa_mem_id();
    if (likely(slab_node == numa_node)) {
// goto;
    }

//
// numa_mem_id() is only a wrapper to numa_node_id() which is where this
// cpu belongs to, but it might be a memoryless node anyway. We don't
// know what the closest node is.
//
    numa_node = numa_node_id();
// freed object is from this cpu's node, proceed
    if (likely(slab_node == numa_node)) {
// goto;
    }
//
// Freed object isn't from this cpu's node, but that node is memoryless
// or only has ZONE_MOVABLE memory, which slab cannot allocate from.
// Proceed as it's better to cache remote objects than falling back to
// the slowpath for everything. The allocation side can never obtain
// a local object anyway, if none exist. We don't have numa_mem_id() to
// point to the closest node as we would on a proper memoryless node
// setup.
//
    if (unlikely(!node_state(numa_node, N_NORMAL_MEMORY))) {
// goto;
    }

    return false;
// label;
    return likely(!slab_test_pfmemalloc(slab));
    }
//
// Try to free as many objects (already processed by free hooks) as possible to
// a single per-cpu sheaf.
//
// Returns how many objects were freed. Zero means failure and the caller should
// fall back to __kmem_cache_free_bulk().
//
#[no_mangle]
unsafe extern "C" fn __free_to_pcs_batch(s: *mut kmem_cache, size: usize, p: *mut c_void) -> c_uint {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    let mut main = core::ptr::null_mut();
    let mut empty = core::ptr::null_mut();
pub static mut barn: *mut c_void = core::ptr::null_mut();
    let mut batch = 0;
    if (!local_trylock(&s.cpu_sheaves.lock)) {
    return 0;
    }
    pcs = this_cpu_ptr(s.cpu_sheaves);
    if (likely(pcs.main.size < s.sheaf_capacity)) {
// goto;
    }
    barn = get_barn(s);
    if (!barn) {
// goto;
    }
    if (!pcs.spare) {
    empty = barn_get_empty_sheaf(barn, true);
    if (!empty) {
// goto;
    }
    pcs.spare = pcs.main;
    pcs.main = empty;
// goto;
    }
    if (pcs.spare.size < s.sheaf_capacity) {
    swap(pcs.main, pcs.spare);
// goto;
    }
    empty = barn_replace_full_sheaf(barn, pcs.main, true);
    if (IS_ERR(empty)) {
    stat(s, BARN_PUT_FAIL);
// goto;
    }
    stat(s, BARN_PUT);
    pcs.main = empty;
// label;
    main = pcs.main;
    batch = min(size, s.sheaf_capacity - main.size);
    memcpy(main.objects + main.size, p, batch * sizeof!);
    main.size += batch;
    local_unlock(&s.cpu_sheaves.lock);
    stat_add(s, FREE_FASTPATH, batch);
    return batch;
// label;
    local_unlock(&s.cpu_sheaves.lock);
    return 0;
    }
//
// Bulk free objects to the percpu sheaves.
// Unlike free_to_pcs() this includes the calls to all necessary hooks
// and the fallback to freeing to slab pages.
//
#[no_mangle]
unsafe extern "C" fn free_to_pcs_bulk(s: *mut kmem_cache, size: usize, p: *mut c_void) {
pub static mut init: bool = false;
    let mut remote_objects = p;
pub static mut remote_nr: c_uint = 0;
//
// Process the free hooks and separate out remote objects by
// partitioning the 'p' array in place:
//
// [0, remote_nr) - processed remote objects
// [remote_nr, i) - processed local objects
// [i, size)      - unprocessed objects
//
    while (i < size) {
    let mut slab = virt_to_slab(p[i]);
    memcg_slab_free_hook(s, slab, p + i, 1);
    alloc_tagging_slab_free_hook(s, slab, p + i, 1);
    if (unlikely(!slab_free_hook(s, p[i], init, false))) {
    p[i] = p[--size];
    continue;
    }
    if (unlikely(!can_free_to_pcs(slab))) {
    if (i != remote_nr) {
    swap(remote_objects[remote_nr], p[i]);
    }
    remote_nr += 1;
    }
    i += 1;
    }
    p += remote_nr;
    size -= remote_nr;
    while (size) {
pub static mut batch_freed: c_uint = 0;
    if (!batch_freed) {
    __kmem_cache_free_bulk(s, size, p);
    stat_add(s, FREE_SLOWPATH, size);
    break;
    }
    p += batch_freed;
    size -= batch_freed;
    }
//
// Processing remote objects last decreases the chances of cpu migration
// while freeing to sheaves and compromising object locality
//
    if (remote_nr) {
    __kmem_cache_free_bulk(s, remote_nr, remote_objects);
    stat_add(s, FREE_SLOWPATH, remote_nr);
    }
    }
//
// In PREEMPT_RT irq_work runs in per-cpu kthread, so it's safe
// to take sleeping spin_locks from __slab_free().
// In !PREEMPT_RT irq_work will run after local_unlock_irqrestore().
//
#[no_mangle]
unsafe extern "C" fn deferred_percpu_work_fn(work: *mut irq_work) {
pub static mut dpw: *mut c_void = core::ptr::null_mut();
    let mut objs = core::ptr::null_mut();
    let mut objs_by_rcu = core::ptr::null_mut();
    let mut rcu_sheaves = core::ptr::null_mut();
    let mut llnode = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut sheaf = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    dpw = container_of!(work, deferred_percpu_work, work);
    rcu_sheaves = &dpw.rcu_sheaves;
    objs = &dpw.objects;
    objs_by_rcu = &dpw.objects_by_rcu;
    llnode = llist_del_all(objs);
    llist_for_each_safe(pos, t, llnode) {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    let mut x = pos;
    slab = virt_to_slab(x);
    s = slab.slab_cache;
// Point 'x' back to the beginning of allocated object
    x -= s.offset;
//
// We used freepointer in 'x' to link 'x' into df->objects.
// Clear it to NULL to avoid false positive detection
// of "Freepointer corruption".
//
    set_freepointer(s, x, core::ptr::null_mut());
    __slab_free(s, slab, x, x, 1, _THIS_IP_);
    stat(s, FREE_SLOWPATH);
    }
    llnode = llist_del_all(objs_by_rcu);
    llist_for_each_safe(pos, t, llnode) {
    let mut head = pos;
    let mut objp = kvmalloc_obj_start_addr(head);
    kvfree_call_rcu(head, objp);
    }
    llnode = llist_del_all(rcu_sheaves);
    llist_for_each_entry_safe(sheaf, next, llnode, llnode) {
    call_rcu(&sheaf.rcu_head, rcu_free_sheaf);
    }
    }
#[no_mangle]
unsafe extern "C" fn defer_free(s: *mut kmem_cache, head: *mut c_void) {
pub static mut dpw: *mut c_void = core::ptr::null_mut();
    guard(preempt)();
    head = kasan_reset_tag(head);
    dpw = this_cpu_ptr(&deferred_percpu_work);
    if (llist_add(head + s.offset, &dpw.objects)) {
    irq_work_queue(&dpw.work);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn defer_kfree_rcu(head: *mut kvfree_rcu_head) {
pub static mut dpw: *mut c_void = core::ptr::null_mut();
    guard(preempt)();
    dpw = this_cpu_ptr(&deferred_percpu_work);
    if (llist_add(head, &dpw.objects_by_rcu)) {
    irq_work_queue(&dpw.work);
    }
    }
// Must be called before flush_rcu_sheaves_on_cache()
#[no_mangle]
pub unsafe extern "C" fn deferred_work_barrier() {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    irq_work_sync(&per_cpu_ptr(&deferred_percpu_work, cpu).work);
    }
    }
    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn slab_free(s: *mut kmem_cache, slab: *mut slab, object: *mut c_void, addr: c_ulong) {
    memcg_slab_free_hook(s, slab, &object, 1);
    alloc_tagging_slab_free_hook(s, slab, &object, 1);
    if (unlikely(!slab_free_hook(s, object, slab_want_init_on_free(s), false))) {
    return;
    }
    if (likely(can_free_to_pcs(slab)) && likely(free_to_pcs(s, object, true))) {
    return;
    }
    __slab_free(s, slab, object, object, 1, addr);
    stat(s, FREE_SLOWPATH);
    }

// Do not inline the rare memcg charging failed path into the allocation path
    static noinline
#[no_mangle]
pub unsafe extern "C" fn memcg_alloc_abort_single(s: *mut kmem_cache, object: *mut c_void) {
    let mut slab = virt_to_slab(object);
    alloc_tagging_slab_free_hook(s, slab, &object, 1);
    if (likely(slab_free_hook(s, object, slab_want_init_on_free(s), false))) {
    __slab_free(s, slab, object, object, 1, _RET_IP_);
    }
    }

    static __fastpath_inline
#[no_mangle]
pub unsafe extern "C" fn slab_free_bulk(s: *mut kmem_cache, slab: *mut slab, head: *mut c_void, tail: *mut c_void, p: *mut *mut c_void, cnt: c_int, addr: c_ulong) {
    memcg_slab_free_hook(s, slab, p, cnt);
    alloc_tagging_slab_free_hook(s, slab, p, cnt);
//
// With KASAN enabled slab_free_freelist_hook modifies the freelist
// to remove objects, whose reuse must be delayed.
//
    if (likely(slab_free_freelist_hook(s, &head, &tail, &cnt))) {
    __slab_free(s, slab, head, tail, cnt, addr);
    stat_add(s, FREE_SLOWPATH, cnt);
    }
    }

#[no_mangle]
unsafe extern "C" fn slab_free_after_rcu_debug(rcu_head: *mut rcu_head) {
    let mut delayed_free = container_of!(rcu_head, rcu_delayed_free, head);
    let mut object = delayed_free.object;
    let mut slab = virt_to_slab(object);
pub static mut s: *mut c_void = core::ptr::null_mut();
    kfree(delayed_free);
    if (WARN_ON!(is_kfence_address(object))) {
    return;
    }
// find the object and the cache again
    if (WARN_ON!(!slab)) {
    return;
    }
    s = slab.slab_cache;
    if (WARN_ON!(!(s.flags & SLAB_TYPESAFE_BY_RCU))) {
    return;
    }
// resume freeing
    if (slab_free_hook(s, object, slab_want_init_on_free(s), true)) {
    __slab_free(s, slab, object, object, 1, _THIS_IP_);
    stat(s, FREE_SLOWPATH);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn ___cache_free(cache: *mut kmem_cache, x: *mut c_void, addr: c_ulong) {
    __slab_free(cache, virt_to_slab(x), x, x, 1, addr);
    stat(cache, FREE_SLOWPATH);
    }

#[no_mangle]
unsafe extern "C" fn warn_free_bad_obj(s: *mut kmem_cache, obj: *mut c_void) -> noinline void {
pub static mut cachep: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    slab = virt_to_slab(obj);
    if (WARN_ONCE(!slab,
    "kmem_cache_free(%s, %p): object is not in a slab page\n",
    s.name, obj)) {
    return;
    }
    cachep = slab.slab_cache;
    if (WARN_ONCE(cachep != s,
    "kmem_cache_free(%s, %p): object belongs to different cache %s\n",
    s.name, obj, cachep ? cachep.name : "(core::ptr::null_mut())")) {
    if (cachep) {
    print_tracking(cachep, obj);
    }
    return;
    }
    }
//
// kmem_cache_free - Deallocate an object
// @s: The cache the allocation was from.
// @x: The previously allocated object.
//
// Free an object which was previously allocated from this
// cache.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_free(s: *mut kmem_cache, x: *mut c_void) {
pub static mut slab: *mut c_void = core::ptr::null_mut();
    slab = virt_to_slab(x);
    if (IS_ENABLED!(CONFIG_SLAB_FREELIST_HARDENED) ||
    kmem_cache_debug_flags(s, SLAB_CONSISTENCY_CHECKS)) {
//
// Intentionally leak the object in these cases, because it
// would be too dangerous to continue.
//
    if (unlikely(!slab || (slab.slab_cache != s))) {
    warn_free_bad_obj(s, x);
    return;
    }
    }
    trace_kmem_cache_free(_RET_IP_, x, s);
    slab_free(s, slab, x, _RET_IP_);
    }
    EXPORT_SYMBOL(kmem_cache_free);
#[no_mangle]
pub unsafe extern "C" fn slab_ksize(slab: *mut slab) -> usize {
    let mut s = slab.slab_cache;

//
// Debugging requires use of the padding between object
// and whatever may come after it.
//
    if (s.flags & (SLAB_RED_ZONE | SLAB_POISON)) {
    return s.object_size;
    }

    if (s.flags & SLAB_KASAN) {
    return s.object_size;
    }
//
// If we have the need to store the freelist pointer
// or any other metadata back there then we can
// only use the space before that information.
//
    if (s.flags & (SLAB_TYPESAFE_BY_RCU | SLAB_STORE_USER)) {
    return s.inuse;
    }

    else if (obj_exts_in_object(slab)) {
    return s.inuse;
    }
//
// Else we can use all the padding etc for the allocation
//
    return s.size;
    }
#[no_mangle]
unsafe extern "C" fn __ksize(object: *const c_void) -> usize {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (unlikely(object == ZERO_SIZE_PTR)) {
    return 0;
    }
    page = virt_to_page(object);
    if (unlikely(PageLargeKmalloc(page))) {
    return large_kmalloc_size(page);
    }
    slab = page_slab(page);
// Delete this after we're sure there are no users
    if (WARN_ON!(!slab)) {
    return page_size(page);
    }

    skip_orig_size_check(slab.slab_cache, object);

    return slab_ksize(slab);
    }
//
// ksize -- Report full size of underlying allocation
// @objp: pointer to the object
//
// This should only be used internally to query the true size of allocations.
// It is not meant to be a way to discover the usable size of an allocation
// after the fact. Instead, use kmalloc_size_roundup(). Using memory beyond
// the originally requested allocation size may trigger KASAN, UBSAN_BOUNDS,
// and/or FORTIFY_SOURCE.
//
// Return: size of the actual memory used by @objp in bytes
//
#[no_mangle]
pub unsafe extern "C" fn ksize(objp: *const c_void) -> usize {
//
// We need to first check that the pointer to the object is valid.
// The KASAN report printed from ksize() is more useful, then when
// it's printed later when the behaviour could be undefined due to
// a potential use-after-free or double-free.
//
// We use kasan_check_byte(), which is supported for the hardware
// tag-based KASAN mode, unlike kasan_check_read/write().
//
// If the pointed to memory is invalid, we return 0 to avoid users of
// ksize() writing to and potentially corrupting the memory region.
//
// We want to perform the check before __ksize(), to avoid potentially
// crashing in __ksize() due to accessing invalid metadata.
//
    if (unlikely(ZERO_OR_NULL_PTR(objp)) || !kasan_check_byte(objp)) {
    return 0;
    }
    return kfence_ksize(objp) ?: __ksize(objp);
    }
    EXPORT_SYMBOL(ksize);
#[no_mangle]
unsafe extern "C" fn free_large_kmalloc(page: *mut page, object: *mut c_void) {
pub static mut order: c_uint = 0;
    if (WARN_ON_ONCE!(!PageLargeKmalloc(page))) {
    dump_page(page, "Not a kmalloc allocation");
    return;
    }
    if (WARN_ON_ONCE!(order == 0)) {
    pr_warn_once("object pointer: 0x%p\n", object);
    }
    kmemleak_free(object);
    kasan_kfree_large(object);
    kmsan_kfree_large(object);
    mod_lruvec_page_state(page, NR_SLAB_UNRECLAIMABLE_B,
    -(PAGE_SIZE << order));
    __ClearPageLargeKmalloc(page);
    free_frozen_pages(page, order);
    }
//
// Given an rcu_head embedded within an object obtained from kvmalloc at an
// offset < 4k, free the object in question.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_rcu_cb(head: *mut rcu_head) {
pub static mut obj: *mut c_void = core::ptr::null_mut();
    obj = kvmalloc_obj_start_addr(head);
    if (is_vmalloc_addr(obj)) {
    vfree(obj);
    } else {
    let mut page = virt_to_page(obj);
    let mut slab = page_slab(page);
    if (slab) {
    slab_free(slab.slab_cache, slab, obj, _RET_IP_);
    }
    else {
    free_large_kmalloc(page, obj);
    }
    }
    }
//
// kfree - free previously allocated memory
// @object: pointer returned by kmalloc(), kmalloc_nolock(), or kmem_cache_alloc()
//
// If @object is NULL, no operation is performed.
//
#[no_mangle]
pub unsafe extern "C" fn kfree(object: *const c_void) {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut x = object;
    trace_kfree(_RET_IP_, object);
    if (unlikely(ZERO_OR_NULL_PTR(object))) {
    return;
    }
    page = virt_to_page(object);
    slab = page_slab(page);
    if (!slab) {
// kmalloc_nolock() doesn't support large kmalloc
    free_large_kmalloc(page, object);
    return;
    }
    s = slab.slab_cache;
    slab_free(s, slab, x, _RET_IP_);
    }
    EXPORT_SYMBOL(kfree);
//
// Can be called while holding raw_spinlock_t or from IRQ and NMI,
// but ONLY for objects allocated by kmalloc_nolock().
// Debug checks (like kmemleak and kfence) were skipped on allocation,
// hence
// obj = kmalloc(); kfree_nolock(obj);
// will miss kmemleak/kfence book keeping and will cause false positives.
// large_kmalloc is not supported either.
//
#[no_mangle]
pub unsafe extern "C" fn kfree_nolock(object: *const c_void) {
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut x = object;
    if (unlikely(ZERO_OR_NULL_PTR(object))) {
    return;
    }
    slab = virt_to_slab(object);
    if (unlikely(!slab)) {
    WARN_ONCE(1, "large_kmalloc is not supported by kfree_nolock()");
    return;
    }
    s = slab.slab_cache;
    memcg_slab_free_hook(s, slab, &x, 1);
    alloc_tagging_slab_free_hook(s, slab, &x, 1);
//
// Unlike slab_free() do NOT call the following:
// kmemleak_free_recursive(x, s->flags);
// debug_check_no_locks_freed(x, s->object_size);
// debug_check_no_obj_freed(x, s->object_size);
// __kcsan_check_access(x, s->object_size, ..);
// kfence_free(x);
// since they take spinlocks or not safe from any context.
//
    kmsan_slab_free(s, x);
//
// If KASAN finds a kernel bug it will do kasan_report_invalid_free()
// which will call raw_spin_lock_irqsave() which is technically
// unsafe from NMI, but take chance and report kernel bug.
// The sequence of
// kasan_report_invalid_free() -> raw_spin_lock_irqsave() -> NMI
// -> kfree_nolock() -> kasan_report_invalid_free() on the same CPU
// is double buggy and deserves to deadlock.
//
    if (kasan_slab_pre_free(s, x)) {
    return;
    }
//
// memcg, kasan_slab_pre_free are done for 'x'.
// The only thing left is kasan_poison without quarantine,
// since kasan quarantine takes locks and not supported from NMI.
//
    kasan_slab_free(s, x, false, false, /* skip quarantine */true);
    if (likely(can_free_to_pcs(slab)) && likely(free_to_pcs(s, x, false))) {
    return;
    }
//
// __slab_free() can locklessly cmpxchg16 into a slab, but then it might
// need to take spin_lock for further processing.
// Avoid the complexity and simply add to a deferred list.
//
    defer_free(s, x);
    }
    EXPORT_SYMBOL_GPL(kfree_nolock);
    static __always_inline __realloc_size(2) void *
    __do_krealloc(const void *p, size_t new_size, unsigned long align, gfp_t flags, int nid, kmalloc_token_t token)
    {
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut ks: usize = 0;
pub static mut orig_size: c_int = 0;
    let mut s = core::ptr::null_mut();
    if (unlikely(ZERO_OR_NULL_PTR(p))) {
// goto;
    }
// Check for double-free.
    if (!kasan_check_byte(p)) {
    return core::ptr::null_mut();
    }
    if (is_kfence_address(p)) {
    ks = orig_size = kfence_ksize(p);
    } else {
    let mut page = virt_to_page(p);
    let mut slab = page_slab(page);
    if (!slab) {
// Big kmalloc object
    ks = page_size(page);
    WARN_ON!(ks <= KMALLOC_MAX_CACHE_SIZE);
    WARN_ON!(p != page_address(page));
    } else {
    s = slab.slab_cache;
    orig_size = get_orig_size(s, p);
    ks = s.object_size;
    }
    }
//
// If reallocation is not necessary (e. g. the new size is less
// than the current allocated size), the current allocation will be
// preserved unless __GFP_THISNODE is set. In the latter case a new
// allocation on the requested node will be attempted.
//
    if (unlikely(flags & __GFP_THISNODE) && nid != NUMA_NO_NODE &&
    nid != page_to_nid(virt_to_page(p))) {
// goto;
    }
// If the old object doesn't fit, allocate a bigger one
    if (new_size > ks) {
// goto;
    }
// If the old object doesn't satisfy the new alignment, allocate a new one
    if (!IS_ALIGNED((unsigned long)p, align)) {
// goto;
    }
// Zero out spare memory.
    if (want_init_on_alloc(flags)) {
    kasan_disable_current();
    if (orig_size && orig_size < new_size) {
    memset(kasan_reset_tag(p) + orig_size, 0, new_size - orig_size);
    }
    else {
    memset(kasan_reset_tag(p) + new_size, 0, ks - new_size);
    }
    kasan_enable_current();
    }
// Setup kmalloc redzone when needed
    if (s && slub_debug_orig_size(s)) {
    set_orig_size(s, p, new_size);
    if (s.flags & SLAB_RED_ZONE && new_size < ks) {
    memset_no_sanitize_memory(kasan_reset_tag(p) + new_size,
    SLUB_RED_ACTIVE, ks - new_size);
    }
    }
    p = kasan_krealloc(p, new_size, flags);
    return p;
// label;
    ret = __kmalloc_node_track_caller_noprof(PASS_KMALLOC_PARAMS(new_size, core::ptr::null_mut(), token), flags, nid, _RET_IP_);
    if (ret && p) {
// Disable KASAN checks as the object's redzone is accessed.
    kasan_disable_current();
    memcpy(ret, kasan_reset_tag(p), min(new_size, (size_t)(orig_size ?: ks)));
    kasan_enable_current();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn krealloc_node_align_noprof(p: *mut c_void, new_size: DECL_TOKEN_PARAMS(, align: c_ulong, flags: gfp_t, nid: c_int) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (unlikely(!new_size)) {
    kfree(p);
    return ZERO_SIZE_PTR;
    }
    ret = __do_krealloc(p, new_size, align, flags, nid, PASS_TOKEN_PARAM(token));
    if (ret && kasan_reset_tag(p) != kasan_reset_tag(ret)) {
    kfree(p);
    }
    return ret;
    }
    EXPORT_SYMBOL(krealloc_node_align_noprof);
#[no_mangle]
unsafe extern "C" fn kmalloc_gfp_adjust(flags: gfp_t, size: usize) -> gfp_t {
//
// We want to attempt a large physically contiguous block first because
// it is less likely to fragment multiple larger blocks and therefore
// contribute to a long term fragmentation less than vmalloc fallback.
// However make sure that larger requests are not too disruptive - i.e.
// do not direct reclaim unless physically continuous memory is preferred
// (__GFP_RETRY_MAYFAIL mode). We still kick in kswapd/kcompactd to
// start working in the background
//
    if (size > PAGE_SIZE) {
    flags |= __GFP_NOWARN;
    if (!(flags & __GFP_RETRY_MAYFAIL)) {
    flags &= ~__GFP_DIRECT_RECLAIM;
    }
// nofail semantic is implemented by the vmalloc fallback
    flags &= ~__GFP_NOFAIL;
    }
    return flags;
    }
#[no_mangle]
pub unsafe extern "C" fn __kvmalloc_node_noprof(size: DECL_KMALLOC_PARAMS(, b: c_void, align: c_ulong, flags: gfp_t, node: c_int) -> *mut c_void {
    let mut allow_block = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
//
// It doesn't really make sense to fallback to vmalloc for sub page
// requests
//
    ret = __do_kmalloc_node(PASS_BUCKET_PARAM(b),
    kmalloc_gfp_adjust(flags, size),
    node, PASS_TOKEN_PARAM(token), &ac);
    if (ret || size <= PAGE_SIZE) {
    return ret;
    }
// Don't even allow crazy sizes
    if (unlikely(size > INT_MAX)) {
    WARN_ON_ONCE!(!(flags & __GFP_NOWARN));
    return core::ptr::null_mut();
    }
//
// For non-blocking the VM_ALLOW_HUGE_VMAP is not used
// because the huge-mapping path in vmalloc contains at
// least one might_sleep() call.
//
// TODO: Revise huge-mapping path to support non-blocking
// flags.
//
    allow_block = gfpflags_allow_blocking(flags);
//
// kvmalloc() can always use VM_ALLOW_HUGE_VMAP,
// since the callers already cannot assume anything
// about the resulting pointer, and cannot play
// protection games.
//
    return __vmalloc_node_range_noprof(size, align, VMALLOC_START, VMALLOC_END,
    flags, PAGE_KERNEL, allow_block ? VM_ALLOW_HUGE_VMAP:0,
    node, __builtin_return_address(0));
    }
    EXPORT_SYMBOL(__kvmalloc_node_noprof);
//
// kvfree() - Free memory.
// @addr: Pointer to allocated memory.
//
// kvfree frees memory allocated by any of vmalloc(), kmalloc() or kvmalloc().
// It is slightly more efficient to use kfree() or vfree() if you are certain
// that you know which one to use.
//
// Context: Either preemptible task context or not-NMI interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree(addr: *const c_void) {
    if (is_vmalloc_addr(addr)) {
    vfree(addr);
    }
    else {
    kfree(addr);
    }
    }
    EXPORT_SYMBOL(kvfree);
//
// kvfree_atomic() - Free memory.
// @addr: Pointer to allocated memory.
//
// Same as kvfree(), but uses vfree_atomic() for vmalloc
// backed memory. Must not be called from NMI context.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_atomic(addr: *const c_void) {
    if (is_vmalloc_addr(addr)) {
    vfree_atomic(addr);
    }
    else {
    kfree(addr);
    }
    }
    EXPORT_SYMBOL(kvfree_atomic);
//
// kvfree_sensitive - Free a data object containing sensitive information.
// @addr: address of the data object to be freed.
// @len: length of the data object.
//
// Use the special memzero_explicit() function to clear the content of a
// kvmalloc'ed object containing sensitive data to make sure that the
// compiler won't optimize out the data clearing.
//
#[no_mangle]
pub unsafe extern "C" fn kvfree_sensitive(addr: *const c_void, len: usize) {
    if (likely(!ZERO_OR_NULL_PTR(addr))) {
    memzero_explicit(addr, len);
    kvfree(addr);
    }
    }
    EXPORT_SYMBOL(kvfree_sensitive);
#[no_mangle]
pub unsafe extern "C" fn kvrealloc_node_align_noprof(p: *mut c_void, size: DECL_TOKEN_PARAMS(, align: c_ulong, flags: gfp_t, nid: c_int) -> *mut c_void {
pub static mut n: *mut c_void = core::ptr::null_mut();
    if (is_vmalloc_addr(p)) {
    return vrealloc_node_align_noprof(p, size, align, flags, nid);
    }
    n = krealloc_node_align_noprof(p, PASS_TOKEN_PARAMS(size, token), align, kmalloc_gfp_adjust(flags, size), nid);
    if (!n) {
// We failed to krealloc(), fall back to kvmalloc().
    n = __kvmalloc_node_noprof(PASS_KMALLOC_PARAMS(size, core::ptr::null_mut(), token), align, flags, nid);
    if (!n) {
    return core::ptr::null_mut();
    }
    if (p) {
// We already know that `p` is not a vmalloc address.
    kasan_disable_current();
    memcpy(n, kasan_reset_tag(p), min(size, ksize(p)));
    kasan_enable_current();
    kfree(p);
    }
    }
    return n;
    }
    EXPORT_SYMBOL(kvrealloc_node_align_noprof);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detached_freelist {
    pub slab: *mut slab,
    pub tail: *mut c_void,
    pub freelist: *mut c_void,
    pub cnt: c_int,
    pub s: *mut kmem_cache,
}

//
// This function progressively scans the array with free objects (with
// a limited look ahead) and extract objects belonging to the same
// slab.  It builds a detached freelist directly within the given
// slab/objects.  This can happen without any need for
// synchronization, because the objects are owned by running process.
// The freelist is build up as a single linked list in the objects.
// The idea is, that this detached freelist can then be bulk
// transferred to the real freelist(s), but only requiring a single
// synchronization primitive.  Look ahead in the array is limited due
// to performance reasons.
//
#[no_mangle]
pub unsafe extern "C" fn build_detached_freelist(s: *mut kmem_cache, size: size_t, p: *mut *mut c_void, df: *mut detached_freelist) -> c_int {
pub static mut lookahead: c_int = 3;
pub static mut object: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    let mut same = 0;
    object = p[--size];
    page = virt_to_page(object);
    slab = page_slab(page);
    if (!s) {
// Handle kalloc'ed objects
    if (!slab) {
    free_large_kmalloc(page, object);
    df.slab = core::ptr::null_mut();
    return size;
    }
// Derive kmem_cache from object
    df.slab = slab;
    df.s = slab.slab_cache;
    } else {
    df.slab = slab;
    df.s = s;
    }
// Start new detached freelist
    df.tail = object;
    df.freelist = object;
    df.cnt = 1;
    if (is_kfence_address(object)) {
    return size;
    }
    set_freepointer(df.s, object, core::ptr::null_mut());
    same = size;
    while (size) {
    object = p[--size];
// df->slab is always set at this point
    if (df.slab == virt_to_slab(object)) {
// Opportunity build freelist
    set_freepointer(df.s, object, df.freelist);
    df.freelist = object;
    df.cnt += 1;
    same -= 1;
    if (size != same) {
    swap(p[size], p[same]);
    }
    continue;
    }
// Limit look ahead search
    if (!--lookahead) {
    break;
    }
    }
    return same;
    }
//
// Internal bulk free of objects that were not initialised by the post alloc
// hooks and thus should not be processed by the free hooks
//
#[no_mangle]
unsafe extern "C" fn __kmem_cache_free_bulk(s: *mut kmem_cache, size: usize, p: *mut c_void) {
    if (!size) {
    return;
    }
    do {
pub static mut df: usize = 0;
    size = build_detached_freelist(s, size, p, &df);
    if (!df.slab) {
    continue;
    }
    if (kfence_free(df.freelist)) {
    continue;
    }
    __slab_free(df.s, df.slab, df.freelist, df.tail, df.cnt,
    _RET_IP_);
    } while (likely(size));
    }
// Note that interrupts must be enabled when calling this function.
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_free_bulk(s: *mut kmem_cache, size: usize, p: *mut c_void) {
    if (!size) {
    return;
    }
//
// freeing to sheaves is so incompatible with the detached freelist so
// once we go that way, we have to do everything differently
//
    if (s && cache_has_sheaves(s)) {
    free_to_pcs_bulk(s, size, p);
    return;
    }
    do {
pub static mut df: usize = 0;
    size = build_detached_freelist(s, size, p, &df);
    if (!df.slab) {
    continue;
    }
    slab_free_bulk(df.s, df.slab, df.freelist, df.tail, &p[size],
    df.cnt, _RET_IP_);
    } while (likely(size));
    }
    EXPORT_SYMBOL(kmem_cache_free_bulk);
#[no_mangle]
pub unsafe extern "C" fn __refill_objects_node(s: *mut kmem_cache, p: *mut *mut c_void, gfp: gfp_t, min: c_uint, max: c_uint, n: *mut kmem_cache_node, allow_spin: bool) -> c_uint {
pub static mut pc: usize = 0;
    let mut slab = core::ptr::null_mut();
    let mut slab2 = core::ptr::null_mut();
pub static mut refilled: c_uint = 0;
    let mut flags = 0;
pub static mut object: *mut c_void = core::ptr::null_mut();
    pc.flags = gfp;
    pc.min_objects = min;
    pc.max_objects = max;
    if (!get_partial_node_bulk(s, n, &pc, allow_spin)) {
    return 0;
    }
    list_for_each_entry_safe(slab, slab2, &pc.slabs, slab_list) {
    let mut count = 0;
    list_del(&slab.slab_list);
    object = get_freelist_nofreeze(s, slab, &count);
    while (count && refilled < max) {
    p[refilled] = object;
    object = get_freepointer(s, object);
    maybe_wipe_obj_freeptr(s, p[refilled]);
    refilled += 1;
    count -= 1;
    }
//
// Freelist had more objects than we can accommodate, we need to
// free them back. First we try to be optimistic and assume the
// slab is still full since we just detached its freelist.
// Otherwise we must find the tail object.
//
    if (unlikely(count)) {
    let mut head = object;
pub static mut tail: *mut c_void = core::ptr::null_mut();
    if (__slab_try_return_freelist(s, slab, head, count)) {
    list_add(&slab.slab_list, &pc.slabs);
    break;
    }
    do {
    tail = object;
    object = get_freepointer(s, object);
    } while (object);
    __slab_free(s, slab, head, tail, count, _RET_IP_);
    }
    if (refilled >= max) {
    break;
    }
    }
    if (!list_empty(&pc.slabs)) {
    spin_lock_irqsave(&n.list_lock, flags);
    list_for_each_entry(slab, &pc.slabs, slab_list) {
    set_node_partial_state(n, slab);
    }
    list_splice_tail(&pc.slabs, &n.partial);
    spin_unlock_irqrestore(&n.list_lock, flags);
    }
    return refilled;
    }

#[no_mangle]
pub unsafe extern "C" fn __refill_objects_any(s: *mut kmem_cache, p: *mut *mut c_void, gfp: gfp_t, min: c_uint, max: c_uint) -> c_uint {
pub static mut zonelist: *mut c_void = core::ptr::null_mut();
pub static mut z: *mut c_void = core::ptr::null_mut();
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut highest_zoneidx: zone_type = 0;
    let mut cpuset_mems_cookie = 0;
pub static mut refilled: c_uint = 0;
// see get_from_any_partial() for the defrag ratio description
    if (!s.remote_node_defrag_ratio ||
    get_cycles() % 1024 > s.remote_node_defrag_ratio) {
    return 0;
    }
    do {
    cpuset_mems_cookie = read_mems_allowed_begin();
    zonelist = node_zonelist(mempolicy_slab_node(), gfp);
    for_each_zone_zonelist(zone, z, zonelist, highest_zoneidx) {
pub static mut n: *mut c_void = core::ptr::null_mut();
    let mut r = 0;
    n = get_node(s, zone_to_nid(zone));
    if (!n || !cpuset_zone_allowed(zone, gfp) ||
    n.nr_partial <= s.min_partial) {
    continue;
    }
    r = __refill_objects_node(s, p, gfp, min, max, n,
// allow_spin = */ false);
    refilled += r;
    if (r >= min) {
//
// Don't check read_mems_allowed_retry() here -
// if mems_allowed was updated in parallel, that
// was a harmless race between allocation and
// the cpuset update
//
    return refilled;
    }
    p += r;
    min -= r;
    max -= r;
    }
    } while (read_mems_allowed_retry(cpuset_mems_cookie));
    return refilled;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __refill_objects_any
pub unsafe extern "C" fn __refill_objects_any_dup(s: *mut kmem_cache, p: *mut *mut c_void, gfp: gfp_t, min: c_uint, max: c_uint) -> c_uint {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn refill_objects(s: *mut kmem_cache, p: *mut *mut c_void, gfp: gfp_t, min: c_uint, max: c_uint) -> c_uint {
pub static mut local_node: c_int = 0;
    let mut refilled = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
    refilled = __refill_objects_node(s, p, gfp, min, max,
    get_node(s, local_node),
// allow_spin = */ true);
    if (refilled >= min) {
    return refilled;
    }
    refilled += __refill_objects_any(s, p + refilled, gfp, min - refilled,
    max - refilled);
    if (refilled >= min) {
    return refilled;
    }
// label;
    slab = new_slab(s, gfp, SLAB_ALLOC_DEFAULT, local_node);
    if (!slab) {
// goto;
    }
    stat(s, ALLOC_SLAB);
    refilled += alloc_from_new_slab(s, slab, p + refilled, max - refilled,
// allow_spin = */ true);
    if (refilled < min) {
// goto;
    }
// label;
    return refilled;
    }
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_alloc_bulk(s: *mut kmem_cache, flags: gfp_t, size: size_t, p: *mut *mut c_void) -> bool {
    let mut i = 0;
    if (IS_ENABLED!(CONFIG_SLUB_TINY) || kmem_cache_debug(s)) {
pub static mut slab_alloc_context: usize = 0;
    while (i < size) {
    p[i] = ___slab_alloc(s, flags, NUMA_NO_NODE, &ac);
    if (unlikely(!p[i])) {
// goto;
    }
    maybe_wipe_obj_freeptr(s, p[i]);
    }
    } else {
    i = refill_objects(s, p, flags, size, size);
    if (i < size) {
// goto;
    }
    stat_add(s, ALLOC_SLOWPATH, i);
    }
    return true;
// label;
    __kmem_cache_free_bulk(s, i, p);
    return false;
    }
//
// kmem_cache_alloc_bulk - Allocate multiple objects
// @s:		The cache to allocate from
// @flags:	GFP_* flags. See kmalloc().
// @size:	Number of objects to allocate
// @p:		Array of allocated objects
//
// Allocate @size objects from @s and places them into @p.  @size must be larger
// than 0.
//
// Interrupts must be enabled when calling this function.
//
// Unlike alloc_pages_bulk(), this function does not check for already allocated
// objects in @p, and thus the caller does not need to zero it.
//
// Return: %true if the allocation succeeded, or %false if it failed.
//
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_alloc_bulk_noprof(s: *mut kmem_cache, flags: gfp_t, size: size_t, p: *mut *mut c_void) -> bool {
pub static mut i: c_uint = 0;
pub static mut kfence_obj: *mut c_void = core::ptr::null_mut();
pub static mut slab_alloc_context: usize = 0;
    if (!size) {
    return false;
    }
    s = slab_pre_alloc_hook(s, flags);
    if (unlikely(!s)) {
    return false;
    }
//
// to make things simpler, only assume at most once kfence allocated
// object per bulk allocation and choose its index randomly
//
    kfence_obj = kfence_alloc(s, s.object_size, flags);
    if (unlikely(kfence_obj)) {
    if (unlikely(size == 1)) {
    p[0] = kfence_obj;
// goto;
    }
    size -= 1;
    }
    i = alloc_from_pcs_bulk(s, size, p);
    if (i < size) {
//
// If we ran out of memory, don't bother with freeing back to
// the percpu sheaves, we have bigger problems.
//
    if (unlikely(!__kmem_cache_alloc_bulk(s, flags, size - i,
    p + i))) {
    if (i > 0) {
    __kmem_cache_free_bulk(s, i, p);
    }
    if (kfence_obj) {
    __kfence_free(kfence_obj);
    }
    return false;
    }
    }
    if (unlikely(kfence_obj)) {
pub static mut idx: c_int = 0;
    if (idx != size) {
    p[size] = p[idx];
    }
    p[idx] = kfence_obj;
    size += 1;
    }
// label;
// memcg and kmem_cache debug support and memory initialization
    return likely(slab_post_alloc_hook(s, flags, size, p, &ac));
    }
    EXPORT_SYMBOL(kmem_cache_alloc_bulk_noprof);
//
// Object placement in a slab is made very easy because we always start at
// offset 0. If we tune the size of the object to the alignment then we can
// get the required alignment by putting one properly sized object after
// another.
//
// Notice that the allocation order determines the sizes of the per cpu
// caches. Each processor has always one slab available for allocations.
// Increasing the allocation order reduces the number of times that slabs
// must be moved on and off the partial lists and is therefore a factor in
// locking overhead.
//
// Minimum / Maximum order of slab pages. This influences locking overhead
// and slab fragmentation. A higher order reduces the number of partial slabs
// and increases the number of allocations possible without having to
// take the list_lock.
//
    static unsigned int slub_min_order;
    static unsigned int slub_max_order =
    IS_ENABLED!(CONFIG_SLUB_TINY) ? 1 : PAGE_ALLOC_COSTLY_ORDER;
    static unsigned int slub_min_objects;
//
// Calculate the order of allocation given an slab object size.
//
// The order of allocation has significant impact on performance and other
// system components. Generally order 0 allocations should be preferred since
// order 0 does not cause fragmentation in the page allocator. Larger objects
// be problematic to put into order 0 slabs because there may be too much
// unused space left. We go to a higher order if more than 1/16th of the slab
// would be wasted.
//
// In order to reach satisfactory performance we must ensure that a minimum
// number of objects is in one slab. Otherwise we may generate too much
// activity on the partial lists which requires taking the list_lock. This is
// less a concern for large slabs though which are rarely used.
//
// slab_max_order specifies the order where we begin to stop considering the
// number of objects in a slab as critical. If we reach slab_max_order then
// we try to keep the page order as low as possible. So we accept more waste
// of space in favor of a small page order.
//
// Higher order allocations also allow the placement of more objects in a
// slab and thereby reduce object handling overhead. If the user has
// requested a higher minimum order then we start with that one instead of
// the smallest order which will fit the object.
//
#[no_mangle]
pub unsafe extern "C" fn calc_slab_order(size: c_uint, min_order: c_uint, max_order: c_uint, fract_leftover: c_uint) -> c_uint {
    let mut order = 0;
    while (order <= max_order) {
pub static mut slab_size: c_uint = 0;
    let mut rem = 0;
    rem = slab_size % size;
    if (rem <= slab_size / fract_leftover) {
    break;
    }
    }
    return order;
    }
#[no_mangle]
pub unsafe extern "C" fn calculate_order(size: c_uint) -> c_int {
    let mut order = 0;
    let mut min_objects = 0;
    let mut max_objects = 0;
    let mut min_order = 0;
    min_objects = slub_min_objects;
    if (!min_objects) {
//
// Some architectures will only update present cpus when
// onlining them, so don't trust the number if it's just 1. But
// we also don't want to use nr_cpu_ids always, as on some other
// architectures, there can be many possible cpus, but never
// onlined. Here we compromise between trying to avoid too high
// order on systems that appear larger than they are, and too
// low order on systems that appear smaller than they are.
//
pub static mut nr_cpus: c_uint = 0;
    if (nr_cpus <= 1) {
    nr_cpus = nr_cpu_ids;
    }
    min_objects = 4 * (fls(nr_cpus) + 1);
    }
// min_objects can't be 0 because get_order(0) is undefined
    max_objects = max(order_objects(slub_max_order, size), 1U);
    min_objects = min(min_objects, max_objects);
    min_order = max_t(unsigned int, slub_min_order,
    get_order(min_objects * size));
    if (order_objects(min_order, size) > MAX_OBJS_PER_PAGE) {
    return get_order(size * MAX_OBJS_PER_PAGE) - 1;
    }
//
// Attempt to find best configuration for a slab. This works by first
// attempting to generate a layout with the best possible configuration
// and backing off gradually.
//
// We start with accepting at most 1/16 waste and try to find the
// smallest order from min_objects-derived/slab_min_order up to
// slab_max_order that will satisfy the constraint. Note that increasing
// the order can only result in same or less fractional waste, not more.
//
// If that fails, we increase the acceptable fraction of waste and try
// again. The last iteration with fraction of 1/2 would effectively
// accept any waste and give us the order determined by min_objects, as
// long as at least single object fits within slab_max_order.
//
    while (fraction > 1) {
    order = calc_slab_order(size, min_order, slub_max_order,
    fraction);
    if (order <= slub_max_order) {
    return order;
    }
    }
//
// Doh this slab cannot be placed using slab_max_order.
//
    order = get_order(size);
    if (order <= MAX_PAGE_ORDER) {
    return order;
    }
    return -ENOSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn init_kmem_cache_node(n: *mut kmem_cache_node) {
    n.nr_partial = 0;
    spin_lock_init(&n.list_lock);
    INIT_LIST_HEAD(&n.partial);

    atomic_long_set(&n.nr_slabs, 0);
    atomic_long_set(&n.total_objects, 0);
    INIT_LIST_HEAD(&n.full);

    }

#[no_mangle]
pub unsafe extern "C" fn alloc_kmem_cache_stats(s: *mut kmem_cache) -> c_int {
    BUILD_BUG_ON!(PERCPU_DYNAMIC_EARLY_SIZE <
    NR_KMALLOC_TYPES * KMALLOC_SHIFT_HIGH *
    sizeof!(kmem_cache_stats));
    s.cpu_stats = alloc_percpu(kmem_cache_stats);
    if (!s.cpu_stats) {
    return 0;
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn init_percpu_sheaves(s: *mut kmem_cache) -> c_int {
pub static mut bootstrap_sheaf: slab_sheaf = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    pcs = per_cpu_ptr(s.cpu_sheaves, cpu);
    local_trylock_init(&pcs.lock);
//
// Bootstrap sheaf has zero size so fast-path allocation fails.
// It has also size == s->sheaf_capacity, so fast-path free
// fails. In the slow paths we recognize the situation by
// checking s->sheaf_capacity. This allows fast paths to assume
// s->cpu_sheaves and pcs->main always exists and are valid.
// It's also safe to share the single static bootstrap_sheaf
// with zero-sized objects array as it's never modified.
//
// Bootstrap_sheaf also has NULL pointer to kmem_cache so we
// recognize it and not attempt to free it when destroying the
// cache.
//
// We keep bootstrap_sheaf for kmem_cache and kmem_cache_node,
// caches with debug enabled, and all caches with SLUB_TINY.
// For kmalloc caches it's used temporarily during the initial
// bootstrap.
//
    if (!s.sheaf_capacity) {
    pcs.main = &bootstrap_sheaf;
    }
    else {
    pcs.main = alloc_empty_sheaf(s, GFP_KERNEL, SLAB_ALLOC_DEFAULT);
    }
    if (!pcs.main) {
    return -ENOMEM;
    }
    }
    return 0;
    }
pub static mut kmem_cache_node: *mut c_void = core::ptr::null_mut();
//
// No kmalloc_node yet so do it by hand. We know that this is the first
// slab on the node for this slabcache. There are no concurrent accesses
// possible.
//
// Note that this function only works on the kmem_cache_node
// when allocating for the kmem_cache_node. This is used for bootstrapping
// memory on a fresh node that has no slab structures yet.
//
#[no_mangle]
unsafe extern "C" fn early_kmem_cache_node_alloc(node: c_int) {
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
    BUG_ON!(kmem_cache_node.size < sizeof!(kmem_cache_node));
    slab = new_slab(kmem_cache_node, GFP_NOWAIT, SLAB_ALLOC_DEFAULT, node);
    BUG_ON!(!slab);
    if (slab_nid(slab) != node) {
    pr_err!("SLUB: Unable to allocate memory from node %d\n", node);
    pr_err!("SLUB: Allocating a useless per node structure in order to be able to continue\n");
    }
    init_slab_obj_iter(kmem_cache_node, slab, &iter, true);
    n = next_slab_obj(kmem_cache_node, &iter);
    BUG_ON!(!n);
    slab.inuse = 1;
    build_slab_freelist(kmem_cache_node, slab, &iter);

    init_object(kmem_cache_node, n, SLUB_RED_ACTIVE);

    n = kasan_slab_alloc(kmem_cache_node, n, GFP_KERNEL, false);
    kmem_cache_node.per_node[node].node = n;
    init_kmem_cache_node(n);
    inc_slabs_node(kmem_cache_node, node, slab.objects);
//
// No locks need to be taken here as it has just been
// initialized and there is no concurrent access.
//
    __add_partial(n, slab, ADD_TO_HEAD);
    }
#[no_mangle]
unsafe extern "C" fn free_kmem_cache_nodes(s: *mut kmem_cache) {
    let mut node = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    for_each_node(node) {
    let mut barn = get_barn_node(s, node);
    if (!barn) {
    continue;
    }
    WARN_ON!(barn.nr_full);
    WARN_ON!(barn.nr_empty);
    kfree(barn);
    s.per_node[node].barn = core::ptr::null_mut();
    }
    for_each_kmem_cache_node(s, node, n) {
    s.per_node[node].node = core::ptr::null_mut();
    kmem_cache_free(kmem_cache_node, n);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_release(s: *mut kmem_cache) {
    cache_random_seq_destroy(s);
    pcs_destroy(s);

    free_percpu(s.cpu_stats);

    free_kmem_cache_nodes(s);
    }
#[no_mangle]
unsafe extern "C" fn init_kmem_cache_nodes(s: *mut kmem_cache) -> c_int {
    let mut node = 0;
    for_each_node_mask(node, slab_nodes) {
pub static mut n: *mut c_void = core::ptr::null_mut();
    if (slab_state == DOWN) {
    early_kmem_cache_node_alloc(node);
    continue;
    }
    n = kmem_cache_alloc_node(kmem_cache_node,
    GFP_KERNEL, node);
    if (!n) {
    return 0;
    }
    init_kmem_cache_node(n);
    s.per_node[node].node = n;
    }
    if (slab_state == DOWN || !cache_has_sheaves(s)) {
    return 1;
    }
    for_each_node_mask(node, slab_barn_nodes) {
pub static mut barn: *mut c_void = core::ptr::null_mut();
    barn = kmalloc_node(sizeof!(*barn), GFP_KERNEL, node);
    if (!barn) {
    return 0;
    }
    barn_init(barn);
    s.per_node[node].barn = barn;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn calculate_sheaf_capacity(s: *mut kmem_cache, args: *mut kmem_cache_args) -> c_uint {
    let mut capacity = 0;
    let mut size = 0;
    if (IS_ENABLED!(CONFIG_SLUB_TINY) || s.flags & SLAB_DEBUG_FLAGS) {
    return 0;
    }
//
// Bootstrap caches can't have sheaves for now (SLAB_NO_SHEAVES).
// SLAB_NOLEAKTRACE caches (e.g., kmemleak's object_cache) must not
// have sheaves to avoid recursion when sheaf allocation triggers
// kmemleak tracking.
//
    if (s.flags & (SLAB_NO_SHEAVES | SLAB_NOLEAKTRACE)) {
    return 0;
    }
//
// For now we use roughly similar formula (divided by two as there are
// two percpu sheaves) as what was used for percpu partial slabs, which
// should result in similar lock contention (barn or list_lock)
//
    if (s.size >= PAGE_SIZE) {
    capacity = 4;
    }

    else if (s.size >= 1024) {
    capacity = 12;
    }

    else if (s.size >= 256) {
    capacity = 26;
    }
    else {
    capacity = 60;
    }
// Increment capacity to make sheaf exactly a kmalloc size bucket
    size = struct_size_t(slab_sheaf, objects, capacity);
    size = kmalloc_size_roundup(size);
    capacity = (size - struct_size_t(slab_sheaf, objects, 0)) / sizeof!;
//
// Respect an explicit request for capacity that's typically motivated by
// expected maximum size of kmem_cache_prefill_sheaf() to not end up
// using low-performance oversize sheaves
//
    return max(capacity, args.sheaf_capacity);
    }
//
// calculate_sizes() determines the order and the distribution of data within
// a slab object.
//
#[no_mangle]
unsafe extern "C" fn calculate_sizes(args: *mut kmem_cache_args, s: *mut kmem_cache) -> c_int {
pub static mut flags: slab_flags_t = 0;
pub static mut size: c_uint = 0;
    let mut aligned_size = 0;
    let mut order = 0;
//
// Round up object size to the next word boundary. We can only
// place the free pointer at word boundaries and this determines
// the possible location of the free pointer.
//
    size = ALIGN(size, sizeof!);

//
// Determine if we can poison the object itself. If the user of
// the slab may touch the object after free or before allocation
// then we should never poison the object itself.
//
    if ((flags & SLAB_POISON) && !(flags & SLAB_TYPESAFE_BY_RCU) &&
    !s.ctor) {
    s.flags |= __OBJECT_POISON;
    }
    else {
    s.flags &= ~__OBJECT_POISON;
    }
//
// If we are Redzoning and there is no space between the end of the
// object and the following fields, add one word so the right Redzone
// is non-empty.
//
    if ((flags & SLAB_RED_ZONE) && size == s.object_size) {
    size += sizeof!;
    }

//
// With that we have determined the number of bytes in actual use
// by the object and redzoning.
//
    s.inuse = size;
    if (((flags & SLAB_TYPESAFE_BY_RCU) && !args.use_freeptr_offset) ||
    (flags & SLAB_POISON) ||
    (s.ctor && !args.use_freeptr_offset) ||
    ((flags & SLAB_RED_ZONE) &&
    (s.object_size < sizeof! || slub_debug_orig_size(s)))) {
//
// Relocate free pointer after the object if it is not
// permitted to overwrite the first word of the object on
// kmem_cache_free.
//
// This is the case if we do RCU, have a constructor, are
// poisoning the objects, or are redzoning an object smaller
// than sizeof! or are redzoning an object with
// slub_debug_orig_size() enabled, in which case the right
// redzone may be extended.
//
// The assumption that s->offset >= s->inuse means free
// pointer is outside of the object is used in the
// freeptr_outside_object() function. If that is no
// longer true, the function needs to be modified.
//
    s.offset = size;
    size += sizeof!;
    } else if (((flags & SLAB_TYPESAFE_BY_RCU) || s.ctor) &&
    args.use_freeptr_offset) {
    s.offset = args.freeptr_offset;
    } else {
//
// Store freelist pointer near middle of object to keep
// it away from the edges of the object to avoid small
// sized over/underflows from neighboring allocations.
//
    s.offset = ALIGN_DOWN(s.object_size / 2, sizeof!);
    }

    if (flags & SLAB_STORE_USER) {
//
// Need to store information about allocs and frees after
// the object.
//
    size += 2 * sizeof!(track);
// Save the original kmalloc request size
    if (flags & SLAB_KMALLOC) {
    size += sizeof!(unsigned long);
    }
    }

    kasan_cache_create(s, &size, &s.flags);

    if (flags & SLAB_RED_ZONE) {
//
// Add some empty padding so that we can catch
// overwrites from earlier objects rather than let
// tracking information or the free pointer be
// corrupted if a user writes before the start
// of the object.
//
    size += sizeof!;
    s.red_left_pad = sizeof!;
    s.red_left_pad = ALIGN(s.red_left_pad, s.align);
    size += s.red_left_pad;
    }

//
// SLUB stores one object immediately after another beginning from
// offset 0. In order to align the objects we have to simply size
// each object to conform to the alignment.
//
    aligned_size = ALIGN(size, s.align);

    if (slab_args_unmergeable(args, s.flags) &&
    (aligned_size - size >= cache_obj_ext_size(s))) {
    s.flags |= SLAB_OBJ_EXT_IN_OBJ;
    }

    size = aligned_size;
    s.size = size;
    s.reciprocal_size = reciprocal_value(size);
    order = calculate_order(size);
    if ((int)order < 0) {
    return 0;
    }
    s.allocflags = __GFP_COMP;
    if (s.flags & SLAB_CACHE_DMA) {
    s.allocflags |= GFP_DMA;
    }
    if (s.flags & SLAB_CACHE_DMA32) {
    s.allocflags |= GFP_DMA32;
    }
    if (s.flags & SLAB_RECLAIM_ACCOUNT) {
    s.allocflags |= __GFP_RECLAIMABLE;
    }
//
// For kmalloc caches we enable sheaves later by
// bootstrap_kmalloc_sheaves() to avoid recursion.
//
    if (!is_kmalloc_cache(s)) {
    s.sheaf_capacity = calculate_sheaf_capacity(s, args);
    }
//
// Determine the number of objects per slab
//
    s.oo = oo_make(order, size);
    s.min = oo_make(get_order(size), size);
    return !!oo_objects(s.oo);
    }
#[no_mangle]
unsafe extern "C" fn list_slab_objects(s: *mut kmem_cache, slab: *mut slab) {

    let mut addr = slab_address(slab);
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!slab_add_kunit_errors()) {
    slab_bug(s, "Objects remaining on __kmem_cache_shutdown()");
    }
    spin_lock(&object_map_lock);
    __fill_map(object_map, s, slab);
    for_each_object(p, s, addr, slab.objects) {
    if (!test_bit(__obj_to_index(s, addr, p), object_map)) {
    if (slab_add_kunit_errors()) {
    continue;
    }
    pr_err!("Object 0x%p @offset=%tu\n", p, p - addr);
    print_tracking(s, p);
    }
    }
    spin_unlock(&object_map_lock);
    __slab_err(slab);

    }
//
// Attempt to free all partial slabs on a node.
// This is called from __kmem_cache_shutdown(). We must take list_lock
// because sysfs file might still access partial list after the shutdowning.
//
#[no_mangle]
unsafe extern "C" fn free_partial(s: *mut kmem_cache, n: *mut kmem_cache_node) {
pub static mut discard: usize = 0;
    let mut slab = core::ptr::null_mut();
    let mut h = core::ptr::null_mut();
    BUG_ON!(irqs_disabled());
    spin_lock_irq(&n.list_lock);
    list_for_each_entry_safe(slab, h, &n.partial, slab_list) {
    if (!slab.inuse) {
    remove_partial(n, slab);
    list_add(&slab.slab_list, &discard);
    } else {
    list_slab_objects(s, slab);
    }
    }
    spin_unlock_irq(&n.list_lock);
    list_for_each_entry_safe(slab, h, &discard, slab_list) {
    discard_slab(s, slab);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_empty(s: *mut kmem_cache) -> bool {
    let mut node = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    for_each_kmem_cache_node(s, node, n) {
    if (n.nr_partial || node_nr_slabs(n))
    return false;
    }
    return true;
    }
//
// Release all resources used by a slab cache.
//
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_shutdown(s: *mut kmem_cache) -> c_int {
    let mut node = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    flush_all_cpus_locked(s);
// we might have rcu sheaves in flight
    if (cache_has_sheaves(s)) {
    rcu_barrier();
    }
    for_each_node(node) {
    let mut barn = get_barn_node(s, node);
    if (barn) {
    barn_shrink(s, barn);
    }
    }
// Attempt to free all objects
    for_each_kmem_cache_node(s, node, n) {
    free_partial(s, n);
    if (n.nr_partial || node_nr_slabs(n)) {
    return 1;
    }
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn __kmem_obj_info(kpp: *mut kmem_obj_info, object: *mut c_void, slab: *mut slab) {
pub static mut base: *mut c_void = core::ptr::null_mut();
    int __maybe_unused i;
    let mut objnr = 0;
pub static mut objp: *mut c_void = core::ptr::null_mut();
pub static mut objp0: *mut c_void = core::ptr::null_mut();
    let mut s = slab.slab_cache;
    struct track __maybe_unused *trackp;
    kpp.kp_ptr = object;
    kpp.kp_slab = slab;
    kpp.kp_slab_cache = s;
    base = slab_address(slab);
    objp0 = kasan_reset_tag(object);

    objp = restore_red_left(s, objp0);

    objp = objp0;

    objnr = obj_to_index(s, slab, objp);
    kpp.kp_data_offset = (unsigned long)(objp0 - objp);
    objp = base + s.size * objnr;
    kpp.kp_objp = objp;
    if (WARN_ON_ONCE!(objp < base || objp >= base + slab.objects * s.size
    || (objp - base) % s.size) ||
    !(s.flags & SLAB_STORE_USER)) {
    return;
    }

    objp = fixup_red_left(s, objp);
    trackp = get_track(s, objp, TRACK_ALLOC);
    kpp.kp_ret = trackp.addr;

    {
    let mut handle;
pub static mut entries: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
    handle = READ_ONCE(trackp.handle);
    if (handle) {
    nr_entries = stack_depot_fetch(handle, &entries);
    for (i = 0; i < KS_ADDRS_COUNT && i < nr_entries; i++) {
    kpp.kp_stack[i] = entries[i];
    }
    }
    trackp = get_track(s, objp, TRACK_FREE);
    handle = READ_ONCE(trackp.handle);
    if (handle) {
    nr_entries = stack_depot_fetch(handle, &entries);
    for (i = 0; i < KS_ADDRS_COUNT && i < nr_entries; i++) {
    kpp.kp_free_stack[i] = entries[i];
    }
    }
    }

    }

//
// Kmalloc subsystem
//
#[no_mangle]
unsafe extern "C" fn setup_slub_min_order(str: *const c_char, kp: *const kernel_param) -> c_int {
    let mut ret = 0;
    ret = kstrtouint(str, 0, &slub_min_order);
    if (ret) {
    return ret;
    }
    if (slub_min_order > slub_max_order) {
    slub_max_order = slub_min_order;
    }
    return 0;
    }
    static const struct kernel_param_ops param_ops_slab_min_order __initconst = {
    .set = setup_slub_min_order,
    };
    __core_param_cb(slab_min_order, &param_ops_slab_min_order, &slub_min_order, 0);
    __core_param_cb(slub_min_order, &param_ops_slab_min_order, &slub_min_order, 0);
#[no_mangle]
unsafe extern "C" fn setup_slub_max_order(str: *const c_char, kp: *const kernel_param) -> c_int {
    let mut ret = 0;
    ret = kstrtouint(str, 0, &slub_max_order);
    if (ret) {
    return ret;
    }
    slub_max_order = min_t(unsigned int, slub_max_order, MAX_PAGE_ORDER);
    if (slub_min_order > slub_max_order) {
    slub_min_order = slub_max_order;
    }
    return 0;
    }
    static const struct kernel_param_ops param_ops_slab_max_order __initconst = {
    .set = setup_slub_max_order,
    };
    __core_param_cb(slab_max_order, &param_ops_slab_max_order, &slub_max_order, 0);
    __core_param_cb(slub_max_order, &param_ops_slab_max_order, &slub_max_order, 0);
    core_param!(slab_min_objects, slub_min_objects, uint, 0);
    core_param!(slub_min_objects, slub_min_objects, uint, 0);

#[no_mangle]
unsafe extern "C" fn setup_slab_strict_numa(str: *const c_char, kp: *const kernel_param) -> c_int {
    if (nr_node_ids > 1) {
    static_branch_enable(&strict_numa);
    pr_info!("SLUB: Strict NUMA enabled.\n");
    } else {
    pr_warn!("slab_strict_numa parameter set on non NUMA system.\n");
    }
    return 0;
    }
    static const struct kernel_param_ops param_ops_slab_strict_numa __initconst = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = setup_slab_strict_numa,
    };
    __core_param_cb(slab_strict_numa, &param_ops_slab_strict_numa, core::ptr::null_mut(), 0);

//
// Rejects incorrectly sized objects and objects that are to be copied
// to/from userspace but do not fall entirely within the containing slab
// cache's usercopy region.
//
// Returns NULL if check passes, otherwise const char * to name of cache
// to indicate an error.
//
#[no_mangle]
pub unsafe extern "C" fn __check_heap_object(ptr: *mut c_void, n: c_ulong, slab: *mut slab, to_user: bool) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;
pub static mut is_kfence: bool = false;
    ptr = kasan_reset_tag(ptr);
// Find object and usable object size.
    s = slab.slab_cache;
// Reject impossible pointers.
    if (ptr < slab_address(slab)) {
    usercopy_abort("SLUB object not in SLUB page?!", core::ptr::null_mut(),
    to_user, 0, n);
    }
// Find offset within object.
    if (is_kfence) {
    offset = ptr - kfence_object_start(ptr);
    }
    else {
    offset = (ptr - slab_address(slab)) % s.size;
    }
// Adjust for redzone and reject if within the redzone.
    if (!is_kfence && kmem_cache_debug_flags(s, SLAB_RED_ZONE)) {
    if (offset < s.red_left_pad) {
    usercopy_abort("SLUB object in left red zone",
    s.name, to_user, offset, n);
    }
    offset -= s.red_left_pad;
    }
// Allow address range falling entirely within usercopy region.
    if (offset >= s.useroffset &&
    offset - s.useroffset <= s.usersize &&
    n <= s.useroffset - offset + s.usersize) {
    return;
    }
    usercopy_abort("SLUB object", s.name, to_user, offset, n);
    }

pub const SHRINK_PROMOTE_MAX: c_int = 32;
//
// kmem_cache_shrink discards empty slabs and promotes the slabs filled
// up most to the head of the partial lists. New allocations will then
// fill those up and thus they can be removed from the partial lists.
//
// The slabs with the least items are placed last. This results in them
// being allocated from last increasing the chance that the last objects
// are freed in them.
//
#[no_mangle]
unsafe extern "C" fn __kmem_cache_do_shrink(s: *mut kmem_cache) -> c_int {
    let mut node = 0;
    let mut i = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut discard: usize = 0;
    struct list_head promote[SHRINK_PROMOTE_MAX];
    let mut flags = 0;
pub static mut ret: c_int = 0;
    for_each_node(node) {
    let mut barn = get_barn_node(s, node);
    if (barn) {
    barn_shrink(s, barn);
    }
    }
    for_each_kmem_cache_node(s, node, n) {
    INIT_LIST_HEAD(&discard);
    for (i = 0; i < SHRINK_PROMOTE_MAX; i++) {
    INIT_LIST_HEAD(promote + i);
    }
    spin_lock_irqsave(&n.list_lock, flags);
//
// Build lists of slabs to discard or promote.
//
// Note that concurrent frees may occur while we hold the
// list_lock. slab->inuse here is the upper limit.
//
    list_for_each_entry_safe(slab, t, &n.partial, slab_list) {
pub static mut free: c_int = 0;
// Do not reread slab->inuse
    barrier();
// We do not keep full slabs on the list
    BUG_ON!(free <= 0);
    if (free == slab.objects) {
    list_move(&slab.slab_list, &discard);
    clear_node_partial_state(n, slab);
    dec_slabs_node(s, node, slab.objects);
    } else if (free <= SHRINK_PROMOTE_MAX) {
    list_move(&slab.slab_list, promote + free - 1);
    }
    }
//
// Promote the slabs filled up most to the head of the
// partial list.
//
    for (i = SHRINK_PROMOTE_MAX - 1; i >= 0; i--) {
    list_splice(promote + i, &n.partial);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
// Release empty slabs
    list_for_each_entry_safe(slab, t, &discard, slab_list) {
    free_slab(s, slab);
    }
    if (node_nr_slabs(n)) {
    ret = 1;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __kmem_cache_shrink(s: *mut kmem_cache) -> c_int {
    flush_all(s);
    return __kmem_cache_do_shrink(s);
    }
#[no_mangle]
unsafe extern "C" fn slab_mem_going_offline_callback() -> c_int {
pub static mut s: *mut c_void = core::ptr::null_mut();
    mutex_lock(&slab_mutex);
    list_for_each_entry(s, &slab_caches, list) {
    flush_all_cpus_locked(s);
    __kmem_cache_do_shrink(s);
    }
    mutex_unlock(&slab_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_mem_going_online_callback(nid: c_int) -> c_int {
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// We are bringing a node online. No memory is available yet. We must
// allocate a kmem_cache_node structure in order to bring the node
// online.
//
    mutex_lock(&slab_mutex);
    list_for_each_entry(s, &slab_caches, list) {
    let mut barn = core::ptr::null_mut();
//
// The structure may already exist if the node was previously
// onlined and offlined.
//
    if (get_node(s, nid)) {
    continue;
    }
    if (cache_has_sheaves(s) && !get_barn_node(s, nid)) {
    barn = kmalloc_node(sizeof!(*barn), GFP_KERNEL, nid);
    if (!barn) {
    ret = -ENOMEM;
// goto;
    }
    }
//
// XXX: kmem_cache_alloc_node will fallback to other nodes
// since memory is not yet available from the node that
// is brought up.
//
    n = kmem_cache_alloc(kmem_cache_node, GFP_KERNEL);
    if (!n) {
    kfree(barn);
    ret = -ENOMEM;
// goto;
    }
    init_kmem_cache_node(n);
    s.per_node[nid].node = n;
    if (barn) {
    barn_init(barn);
    s.per_node[nid].barn = barn;
    }
    }
//
// Any cache created after this point will also have kmem_cache_node
// and barn initialized for the new node.
//
    node_set(nid, slab_nodes);
    node_set(nid, slab_barn_nodes);
// label;
    mutex_unlock(&slab_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn slab_memory_callback(self: *mut notifier_block, action: c_ulong, arg: *mut c_void) -> c_int {
    let mut nn = arg;
pub static mut nid: c_int = 0;
pub static mut ret: c_int = 0;
    match (action) {
    NODE_ADDING_FIRST_MEMORY => {
    ret = slab_mem_going_online_callback(nid);
    // break;
    }
    NODE_REMOVING_LAST_MEMORY => {
    ret = slab_mem_going_offline_callback();
    // break;
    }
    }
    if (ret) {
    ret = notifier_from_errno(ret);
    }
    else {
    ret = NOTIFY_OK;
    }
    return ret;
    }
//
// Basic setup of slabs
//
// Used for early kmem_cache structures that were allocated using
// the page allocator. Allocate them properly then fix up the pointers
// that may be pointing to the wrong kmem_cache structure.
//
#[no_mangle]
unsafe extern "C" fn bootstrap(static_cache: *mut kmem_cache) -> *mut kmem_cache  __init {
    let mut node = 0;
    let mut s = kmem_cache_zalloc(kmem_cache, GFP_NOWAIT);
pub static mut n: *mut c_void = core::ptr::null_mut();
    memcpy(s, static_cache, kmem_cache.object_size);
    for_each_kmem_cache_node(s, node, n) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(p, &n.partial, slab_list) {
    p.slab_cache = s;
    }

    list_for_each_entry(p, &n.full, slab_list) {
    p.slab_cache = s;
    }

    }
    list_add(&s.list, &slab_caches);
    return s;
    }
//
// Finish the sheaves initialization done normally by init_percpu_sheaves() and
// init_kmem_cache_nodes(). For normal kmalloc caches we have to bootstrap it
// since sheaves and barns are allocated by kmalloc.
//
#[no_mangle]
unsafe extern "C" fn bootstrap_cache_sheaves(s: *mut kmem_cache)  {
pub static mut empty_args: kmem_cache_args = 0;
    let mut capacity = 0;
pub static mut failed: bool = false;
    let mut node = 0;
    let mut cpu = 0;
    VM_WARN_ON_ONCE(cache_has_sheaves(s));
    capacity = calculate_sheaf_capacity(s, &empty_args);
// capacity can be 0 due to debugging or SLUB_TINY
    if (!capacity) {
    return;
    }
    for_each_node_mask(node, slab_barn_nodes) {
pub static mut barn: *mut c_void = core::ptr::null_mut();
    barn = kmalloc_node(sizeof!(*barn), GFP_KERNEL, node);
    if (!barn) {
    failed = true;
// goto;
    }
    barn_init(barn);
    s.per_node[node].barn = barn;
    }
    for_each_possible_cpu(cpu) {
pub static mut pcs: *mut c_void = core::ptr::null_mut();
    pcs = per_cpu_ptr(s.cpu_sheaves, cpu);
    pcs.main = __alloc_empty_sheaf(s, GFP_KERNEL,
    SLAB_ALLOC_DEFAULT, capacity);
    if (!pcs.main) {
    failed = true;
    break;
    }
    }
// label;
//
// It's still early in boot so treat this like same as a failure to
// create the kmalloc cache in the first place
//
    if (failed) {
    panic("Out of memory when creating kmem_cache %s\n", s.name);
    }
    s.sheaf_capacity = capacity;
    }
#[no_mangle]
unsafe extern "C" fn bootstrap_kmalloc_sheaves()  {
    enum kmalloc_cache_type type;
    while (type < NR_KMALLOC_TYPES) {
    while (idx < KMALLOC_SHIFT_HIGH + 1) {
    let mut s = kmalloc_caches[type][idx];
// Do not bootstrap twice when caches are aliased
    if (s && !cache_has_sheaves(s)) {
    bootstrap_cache_sheaves(s);
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_init()  {
    static __initdata struct kmem_cache boot_kmem_cache,
    boot_kmem_cache_node;
    let mut node = 0;
    slab_obj_ext_has_codetag_init();
    if (debug_guardpage_minorder()) {
    slub_max_order = 0;
    }
// Inform pointer hashing choice about slub debugging state.
    hash_pointers_finalize(__slub_debug_enabled());
    kmem_cache_node = &boot_kmem_cache_node;
    kmem_cache = &boot_kmem_cache;
//
// Initialize the nodemask for which we will allocate per node
// structures. Here we don't need taking slab_mutex yet.
//
    for_each_node_state(node, N_MEMORY) {
    node_set(node, slab_nodes);
    }
    for_each_online_node(node) {
    node_set(node, slab_barn_nodes);
    }
    create_boot_cache(kmem_cache_node, "kmem_cache_node",
    sizeof!(kmem_cache_node),
    SLAB_HWCACHE_ALIGN | SLAB_NO_SHEAVES | SLAB_NO_OBJ_EXT,
    0, 0);
    hotplug_node_notifier(slab_memory_callback, SLAB_CALLBACK_PRI);
// Able to allocate the per node structures
    slab_state = PARTIAL;
    create_boot_cache(kmem_cache, "kmem_cache",
    offsetof(kmem_cache, per_node) +
    nr_node_ids * sizeof!(kmem_cache_per_node_ptrs),
    SLAB_HWCACHE_ALIGN | SLAB_NO_SHEAVES | SLAB_NO_OBJ_EXT,
    0, 0);
    kmem_cache = bootstrap(&boot_kmem_cache);
    kmem_cache_node = bootstrap(&boot_kmem_cache_node);
// Now we can use the kmem_cache to allocate kmalloc slabs
    setup_kmalloc_cache_index_table();
    create_kmalloc_caches();
    bootstrap_kmalloc_sheaves();
// Setup random freelists for each cache
    init_freelist_randomization();
    cpuhp_setup_state_nocalls(CPUHP_SLUB_DEAD, "slub:dead", slub_cpu_setup,
    slub_cpu_dead);
    pr_info!("SLUB: HWalign=%d, Order=%u-%u, MinObjects=%u, CPUs=%u, Nodes=%u\n",
    cache_line_size(),
    slub_min_order, slub_max_order, slub_min_objects,
    nr_cpu_ids, nr_node_ids);
    }
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_init_late()  {
    flushwq = alloc_workqueue("slub_flushwq", WQ_MEM_RECLAIM | WQ_PERCPU,
    0);
    WARN_ON!(!flushwq);

    prandom_init_once(&slab_rnd_state);

    }
#[no_mangle]
pub unsafe extern "C" fn do_kmem_cache_create(s: *mut kmem_cache, name: *mut c_char, size: c_uint, args: *mut kmem_cache_args, flags: slab_flags_t) -> c_int {
pub static mut err: c_int = 0;
    s.name = name;
    s.size = s.object_size = size;
    s.flags = kmem_cache_flags(flags, s.name);

    s.random = get_random_long();

    s.align = args.align;
    s.ctor = args.ctor;

    s.useroffset = args.useroffset;
    s.usersize = args.usersize;

    if (!calculate_sizes(args, s)) {
// goto;
    }
    if (disable_higher_order_debug) {
//
// Disable debugging flags that store metadata if the min slab
// order increased.
//
    if (get_order(s.size) > get_order(s.object_size)) {
    s.flags &= ~DEBUG_METADATA_FLAGS;
    s.offset = 0;
    if (!calculate_sizes(args, s)) {
// goto;
    }
    }
    }

    if (system_has_freelist_aba() && !(s.flags & SLAB_NO_CMPXCHG)) {
// Enable fast mode
    s.flags |= __CMPXCHG_DOUBLE;
    }

//
// The larger the object size is, the more slabs we want on the partial
// list to avoid pounding the page allocator excessively.
//
    s.min_partial = min_t(unsigned long, MAX_PARTIAL, ilog2(s.size) / 2);
    s.min_partial = max_t(unsigned long, MIN_PARTIAL, s.min_partial);
    s.cpu_sheaves = alloc_percpu(slub_percpu_sheaves);
    if (!s.cpu_sheaves) {
    err = -ENOMEM;
// goto;
    }

    s.remote_node_defrag_ratio = 1000;

// Initialize the pre-computed randomized freelist if slab is up
    if (slab_state >= UP) {
    if (init_cache_random_seq(s)) {
// goto;
    }
    }
    if (!init_kmem_cache_nodes(s)) {
// goto;
    }

    if (!alloc_kmem_cache_stats(s)) {
// goto;
    }

    err = init_percpu_sheaves(s);
    if (err) {
// goto;
    }
    err = 0;
// Mutex is not taken during early boot
    if (slab_state <= UP) {
// goto;
    }
//
// Failing to create sysfs files is not critical to SLUB functionality.
// If it fails, proceed with cache creation without these files.
//
    if (sysfs_slab_add(s)) {
    pr_err!("SLUB: Unable to add cache %s to sysfs\n", s.name);
    }
    if (s.flags & SLAB_STORE_USER) {
    debugfs_slab_add(s);
    }
// label;
    if (err) {
    __kmem_cache_release(s);
    }
    return err;
    }

#[no_mangle]
unsafe extern "C" fn count_inuse(slab: *mut slab) -> c_int {
    return slab.inuse;
    }
#[no_mangle]
unsafe extern "C" fn count_total(slab: *mut slab) -> c_int {
    return slab.objects;
    }

#[no_mangle]
pub unsafe extern "C" fn validate_slab(s: *mut kmem_cache, slab: *mut slab, obj_map: *mut c_ulong) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut addr = slab_address(slab);
    if (!validate_slab_ptr(slab)) {
    slab_err(s, slab, "Not a valid slab page");
    return;
    }
    if (!check_slab(s, slab) || !on_freelist(s, slab, core::ptr::null_mut())) {
    return;
    }
// Now we know that a valid freelist exists
    __fill_map(obj_map, s, slab);
    for_each_object(p, s, addr, slab.objects) {
    u8 val = test_bit(__obj_to_index(s, addr, p), obj_map) ?
    SLUB_RED_INACTIVE : SLUB_RED_ACTIVE;
    if (!check_object(s, slab, p, val)) {
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn validate_slab_node(s: *mut kmem_cache, n: *mut kmem_cache_node, obj_map: *mut c_ulong) -> c_int {
pub static mut count: c_ulong = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&n.list_lock, flags);
    list_for_each_entry(slab, &n.partial, slab_list) {
    validate_slab(s, slab, obj_map);
    count += 1;
    }
    if (count != n.nr_partial) {
    pr_err!("SLUB %s: %ld partial slabs counted but counter=%ld\n",
    s.name, count, n.nr_partial);
    slab_add_kunit_errors();
    }
    if (!(s.flags & SLAB_STORE_USER)) {
// goto;
    }
    list_for_each_entry(slab, &n.full, slab_list) {
    validate_slab(s, slab, obj_map);
    count += 1;
    }
    if (count != node_nr_slabs(n)) {
    pr_err!("SLUB: %s %ld slabs counted but counter=%ld\n",
    s.name, count, node_nr_slabs(n));
    slab_add_kunit_errors();
    }
// label;
    spin_unlock_irqrestore(&n.list_lock, flags);
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn validate_slab_cache(s: *mut kmem_cache) -> c_long {
    let mut node = 0;
pub static mut count: c_ulong = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
pub static mut obj_map: *mut c_void = core::ptr::null_mut();
    obj_map = bitmap_alloc(oo_objects(s.oo), GFP_KERNEL);
    if (!obj_map) {
    return -ENOMEM;
    }
    flush_all(s);
    for_each_kmem_cache_node(s, node, n) {
    count += validate_slab_node(s, n, obj_map);
    }
    bitmap_free(obj_map);
    return count;
    }
    EXPORT_SYMBOL(validate_slab_cache);

//
// Generate lists of code addresses where slabcache objects are allocated
// and freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct location {
    pub handle: depot_stack_handle_t,
    pub count: c_ulong,
    pub addr: c_ulong,
    pub waste: c_ulong,
    pub sum_time: c_longlong,
    pub min_time: c_long,
    pub max_time: c_long,
    pub min_pid: c_long,
    pub max_pid: c_long,
    pub NR_CPUS): DECLARE_BITMAP(cpus,,
    pub nodes: nodemask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loc_track {
    pub max: c_ulong,
    pub count: c_ulong,
    pub loc: *mut location,
    pub idx: loff_t,
}

pub static mut slab_debugfs_root: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn free_loc_track(t: *mut loc_track) {
    if (t.max) {
    free_pages((unsigned long)t.loc,
    get_order(sizeof!(location) * t.max));
    }
    }
#[no_mangle]
unsafe extern "C" fn alloc_loc_track(t: *mut loc_track, max: c_ulong, flags: gfp_t) -> c_int {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut order = 0;
    order = get_order(sizeof!(location) * max);
    l = __get_free_pages(flags, order);
    if (!l) {
    return 0;
    }
    if (t.count) {
    memcpy(l, t.loc, sizeof!(location) * t.count);
    free_loc_track(t);
    }
    t.max = max;
    t.loc = l;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn add_location(t: *mut loc_track, s: *mut kmem_cache, track: *mut track, orig_size: c_uint) -> c_int {
    let mut start = 0;
    let mut end = 0;
    let mut pos = 0;
pub static mut l: *mut c_void = core::ptr::null_mut();
    unsigned long caddr, chandle, cwaste;
pub static mut age: c_ulong = 0;
pub static mut handle: depot_stack_handle_t = 0;
pub static mut waste: c_uint = 0;

    handle = READ_ONCE(track.handle);

    start = -1;
    end = t.count;
    while ( ) {
    pos = start + (end - start + 1) / 2;
//
// There is nothing at "end". If we end up there
// we need to add something to before end.
//
    if (pos == end) {
    break;
    }
    l = &t.loc[pos];
    caddr = l.addr;
    chandle = l.handle;
    cwaste = l.waste;
    if ((track.addr == caddr) && (handle == chandle) &&
    (waste == cwaste)) {
    l.count += 1;
    if (track.when) {
    l.sum_time += age;
    if (age < l.min_time) {
    l.min_time = age;
    }
    if (age > l.max_time) {
    l.max_time = age;
    }
    if (track.pid < l.min_pid) {
    l.min_pid = track.pid;
    }
    if (track.pid > l.max_pid) {
    l.max_pid = track.pid;
    }
    cpumask_set_cpu(track.cpu,
    to_cpumask(l.cpus));
    }
    node_set(page_to_nid(virt_to_page(track)), l.nodes);
    return 1;
    }
    if (track.addr < caddr) {
    end = pos;
    }

    else if (track.addr == caddr && handle < chandle) {
    end = pos;
    }
    else if (track.addr == caddr && handle == chandle &&
    waste < cwaste) {
    end = pos;
    }
    else {
    start = pos;
    }
    }
//
// Not found. Insert new tracking element.
//
    if (t.count >= t.max && !alloc_loc_track(t, 2 * t.max, GFP_ATOMIC)) {
    return 0;
    }
    l = t.loc + pos;
    if (pos < t.count) {
    memmove(l + 1, l,
    (t.count - pos) * sizeof!(location));
    }
    t.count += 1;
    l.count = 1;
    l.addr = track.addr;
    l.sum_time = age;
    l.min_time = age;
    l.max_time = age;
    l.min_pid = track.pid;
    l.max_pid = track.pid;
    l.handle = handle;
    l.waste = waste;
    cpumask_clear(to_cpumask(l.cpus));
    cpumask_set_cpu(track.cpu, to_cpumask(l.cpus));
    nodes_clear(l.nodes);
    node_set(page_to_nid(virt_to_page(track)), l.nodes);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn process_slab(t: *mut loc_track, s: *mut kmem_cache, slab: *mut slab, alloc: track_item, obj_map: *mut c_ulong) {
    let mut addr = slab_address(slab);
pub static mut is_alloc: bool = false;
pub static mut p: *mut c_void = core::ptr::null_mut();
    __fill_map(obj_map, s, slab);
    for_each_object(p, s, addr, slab.objects) {
    if (!test_bit(__obj_to_index(s, addr, p), obj_map))
    add_location(t, s, get_track(s, p, alloc),
    is_alloc ? get_orig_size(s, p) :
    s.object_size);
    }
    }

    enum slab_stat_type {
    SL_ALL,			/* All slabs */
    SL_PARTIAL,		/* Only partially allocated slabs */
    SL_OBJECTS,		/* Determine allocated objects not slabs */
    SL_TOTAL		/* Determine object capacity not slabs */
    };

#[no_mangle]
pub unsafe extern "C" fn show_slab_objects(s: *mut kmem_cache, buf: *mut c_char, flags: c_ulong) -> ssize_t {
pub static mut total: c_ulong = 0;
    let mut node = 0;
    let mut x = 0;
pub static mut nodes: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    nodes = kcalloc(nr_node_ids, sizeof!(unsigned long), GFP_KERNEL);
    if (!nodes) {
    return -ENOMEM;
    }
//
// It is impossible to take "mem_hotplug_lock" here with "kernfs_mutex"
// already held which will conflict with an existing lock order:
//
// mem_hotplug_lock->slab_mutex->kernfs_mutex
//
// We don't really need mem_hotplug_lock (to hold off
// slab_mem_going_offline_callback) here because slab's memory hot
// unplug code doesn't destroy the kmem_cache->node[] data.
//

    if (flags & SO_ALL) {
pub static mut n: *mut c_void = core::ptr::null_mut();
    for_each_kmem_cache_node(s, node, n) {
    if (flags & SO_TOTAL) {
    x = node_nr_objs(n);
    }

    else if (flags & SO_OBJECTS) {
    x = node_nr_objs(n) - count_partial(n, count_free);
    }
    else {
    x = node_nr_slabs(n);
    }
    total += x;
    nodes[node] += x;
    }
    } else {

    if (flags & SO_PARTIAL) {
    }
pub static mut n: *mut c_void = core::ptr::null_mut();
    for_each_kmem_cache_node(s, node, n) {
    if (flags & SO_TOTAL) {
    x = count_partial(n, count_total);
    }

    else if (flags & SO_OBJECTS) {
    x = count_partial(n, count_inuse);
    }
    else {
    x = n.nr_partial;
    }
    total += x;
    nodes[node] += x;
    }
    }
    len += sysfs_emit_at(buf, len, "%lu", total);

    while (node < nr_node_ids) {
    if (nodes[node]) {
    len += sysfs_emit_at(buf, len, " N%d=%lu",
    node, nodes[node]);
    }
    }

    len += sysfs_emit_at(buf, len, "\n");
    kfree(nodes);
    return len;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(kmem_cache s, char,
    pub count): *const *const *const *const ssize_t (store)(kmem_cache s, char x, size_t,
}

    static const struct slab_attribute _name##_attr = __ATTR_RO_MODE(_name, 0400)

    static const struct slab_attribute _name##_attr = __ATTR_RW_MODE(_name, 0600)
#[no_mangle]
unsafe extern "C" fn slab_size_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.size);
    }
    SLAB_ATTR_RO(slab_size);
#[no_mangle]
unsafe extern "C" fn align_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.align);
    }
    SLAB_ATTR_RO(align);
#[no_mangle]
unsafe extern "C" fn object_size_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.object_size);
    }
    SLAB_ATTR_RO(object_size);
#[no_mangle]
unsafe extern "C" fn objs_per_slab_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", oo_objects(s.oo));
    }
    SLAB_ATTR_RO(objs_per_slab);
#[no_mangle]
unsafe extern "C" fn order_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", oo_order(s.oo));
    }
    SLAB_ATTR_RO(order);
#[no_mangle]
unsafe extern "C" fn sheaf_capacity_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.sheaf_capacity);
    }
    SLAB_ATTR_RO(sheaf_capacity);
#[no_mangle]
unsafe extern "C" fn min_partial_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%lu\n", s.min_partial);
    }
#[no_mangle]
pub unsafe extern "C" fn min_partial_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
    let mut min = 0;
    let mut err = 0;
    err = kstrtoul(buf, 10, &min);
    if (err) {
    return err;
    }
    s.min_partial = min;
    return length;
    }
    SLAB_ATTR(min_partial);
#[no_mangle]
unsafe extern "C" fn cpu_partial_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "0\n");
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_partial_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
    let mut objects = 0;
    let mut err = 0;
    err = kstrtouint(buf, 10, &objects);
    if (err) {
    return err;
    }
    if (objects) {
    return -EINVAL;
    }
    return length;
    }
    SLAB_ATTR(cpu_partial);
#[no_mangle]
unsafe extern "C" fn ctor_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    if (!s.ctor) {
    return 0;
    }
    return sysfs_emit(buf, "%pS\n", s.ctor);
    }
    SLAB_ATTR_RO(ctor);
#[no_mangle]
unsafe extern "C" fn aliases_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", s.refcount < 0 ? 0 : s.refcount - 1);
    }
    SLAB_ATTR_RO(aliases);
#[no_mangle]
unsafe extern "C" fn partial_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return show_slab_objects(s, buf, SO_PARTIAL);
    }
    SLAB_ATTR_RO(partial);
#[no_mangle]
unsafe extern "C" fn cpu_slabs_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "0\n");
    }
    SLAB_ATTR_RO(cpu_slabs);
#[no_mangle]
unsafe extern "C" fn objects_partial_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return show_slab_objects(s, buf, SO_PARTIAL|SO_OBJECTS);
    }
    SLAB_ATTR_RO(objects_partial);
#[no_mangle]
unsafe extern "C" fn slabs_cpu_partial_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "0(0)\n");
    }
    SLAB_ATTR_RO(slabs_cpu_partial);
#[no_mangle]
unsafe extern "C" fn reclaim_account_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_RECLAIM_ACCOUNT));
    }
    SLAB_ATTR_RO(reclaim_account);
#[no_mangle]
unsafe extern "C" fn hwcache_align_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_HWCACHE_ALIGN));
    }
    SLAB_ATTR_RO(hwcache_align);

#[no_mangle]
unsafe extern "C" fn cache_dma_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_CACHE_DMA));
    }
    SLAB_ATTR_RO(cache_dma);

#[no_mangle]
unsafe extern "C" fn usersize_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.usersize);
    }
    SLAB_ATTR_RO(usersize);

#[no_mangle]
unsafe extern "C" fn destroy_by_rcu_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_TYPESAFE_BY_RCU));
    }
    SLAB_ATTR_RO(destroy_by_rcu);

#[no_mangle]
unsafe extern "C" fn slabs_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return show_slab_objects(s, buf, SO_ALL);
    }
    SLAB_ATTR_RO(slabs);
#[no_mangle]
unsafe extern "C" fn total_objects_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return show_slab_objects(s, buf, SO_ALL|SO_TOTAL);
    }
    SLAB_ATTR_RO(total_objects);
#[no_mangle]
unsafe extern "C" fn objects_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return show_slab_objects(s, buf, SO_ALL|SO_OBJECTS);
    }
    SLAB_ATTR_RO(objects);
#[no_mangle]
unsafe extern "C" fn sanity_checks_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_CONSISTENCY_CHECKS));
    }
    SLAB_ATTR_RO(sanity_checks);
#[no_mangle]
unsafe extern "C" fn trace_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_TRACE));
    }
    SLAB_ATTR_RO(trace);
#[no_mangle]
unsafe extern "C" fn red_zone_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_RED_ZONE));
    }
    SLAB_ATTR_RO(red_zone);
#[no_mangle]
unsafe extern "C" fn poison_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_POISON));
    }
    SLAB_ATTR_RO(poison);
#[no_mangle]
unsafe extern "C" fn store_user_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_STORE_USER));
    }
    SLAB_ATTR_RO(store_user);
#[no_mangle]
unsafe extern "C" fn validate_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn validate_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
pub static mut ret: c_int = 0;
    if (buf[0] == '1' && kmem_cache_debug(s)) {
    ret = validate_slab_cache(s);
    if (ret >= 0) {
    ret = length;
    }
    }
    return ret;
    }
    SLAB_ATTR(validate);

#[no_mangle]
unsafe extern "C" fn failslab_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_FAILSLAB));
    }
#[no_mangle]
pub unsafe extern "C" fn failslab_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
    if (s.refcount > 1) {
    return -EINVAL;
    }
    if (buf[0] == '1') {
    WRITE_ONCE(s.flags, s.flags | SLAB_FAILSLAB);
    }
    else {
    WRITE_ONCE(s.flags, s.flags & ~SLAB_FAILSLAB);
    }
    return length;
    }
    SLAB_ATTR(failslab);

#[no_mangle]
unsafe extern "C" fn shrink_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shrink_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
    if (buf[0] == '1') {
    kmem_cache_shrink(s);
    }
    else {
    return -EINVAL;
    }
    return length;
    }
    SLAB_ATTR(shrink);

#[no_mangle]
unsafe extern "C" fn remote_node_defrag_ratio_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%u\n", s.remote_node_defrag_ratio / 10);
    }
#[no_mangle]
pub unsafe extern "C" fn remote_node_defrag_ratio_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
    let mut ratio = 0;
    let mut err = 0;
    err = kstrtouint(buf, 10, &ratio);
    if (err) {
    return err;
    }
    if (ratio > 100) {
    return -ERANGE;
    }
    s.remote_node_defrag_ratio = ratio * 10;
    return length;
    }
    SLAB_ATTR(remote_node_defrag_ratio);

#[no_mangle]
unsafe extern "C" fn show_stat(s: *mut kmem_cache, buf: *mut c_char, si: stat_item) -> c_int {
pub static mut sum: c_ulong = 0;
    let mut cpu = 0;
pub static mut len: c_int = 0;
    let mut data = kmalloc_objs(int, nr_cpu_ids);
    if (!data) {
    return -ENOMEM;
    }
    for_each_online_cpu(cpu) {
pub static mut x: c_uint = 0;
    data[cpu] = x;
    sum += x;
    }
    len += sysfs_emit_at(buf, len, "%lu", sum);

    for_each_online_cpu(cpu) {
    if (data[cpu]) {
    len += sysfs_emit_at(buf, len, " C%d=%u",
    cpu, data[cpu]);
    }
    }

    kfree(data);
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }
#[no_mangle]
unsafe extern "C" fn clear_stat(s: *mut kmem_cache, si: stat_item) {
    let mut cpu = 0;
    for_each_online_cpu(cpu) {
    per_cpu_ptr(s.cpu_stats, cpu).stat[si] = 0;
    }
    }

    static ssize_t text##_show(kmem_cache *s, char *buf)	
    {								
    return show_stat(s, buf, si);				
    }								
    static ssize_t text##_store(kmem_cache *s,		
    const char *buf, size_t length)	
    {								
    if (buf[0] != '0')					 {
    return -EINVAL;					
    }
    clear_stat(s, si);					
    return length;						
    }								
    SLAB_ATTR(text);						
    STAT_ATTR(ALLOC_FASTPATH, alloc_fastpath);
    STAT_ATTR(ALLOC_SLOWPATH, alloc_slowpath);
    STAT_ATTR(FREE_RCU_SHEAF, free_rcu_sheaf);
    STAT_ATTR(FREE_RCU_SHEAF_FAIL, free_rcu_sheaf_fail);
    STAT_ATTR(FREE_FASTPATH, free_fastpath);
    STAT_ATTR(FREE_SLOWPATH, free_slowpath);
    STAT_ATTR(FREE_ADD_PARTIAL, free_add_partial);
    STAT_ATTR(FREE_REMOVE_PARTIAL, free_remove_partial);
    STAT_ATTR(ALLOC_SLAB, alloc_slab);
    STAT_ATTR(ALLOC_NODE_MISMATCH, alloc_node_mismatch);
    STAT_ATTR(FREE_SLAB, free_slab);
    STAT_ATTR(ORDER_FALLBACK, order_fallback);
    STAT_ATTR(CMPXCHG_DOUBLE_FAIL, cmpxchg_double_fail);
    STAT_ATTR(SHEAF_FLUSH, sheaf_flush);
    STAT_ATTR(SHEAF_REFILL, sheaf_refill);
    STAT_ATTR(SHEAF_ALLOC, sheaf_alloc);
    STAT_ATTR(SHEAF_FREE, sheaf_free);
    STAT_ATTR(BARN_GET, barn_get);
    STAT_ATTR(BARN_GET_FAIL, barn_get_fail);
    STAT_ATTR(BARN_PUT, barn_put);
    STAT_ATTR(BARN_PUT_FAIL, barn_put_fail);
    STAT_ATTR(SHEAF_PREFILL_FAST, sheaf_prefill_fast);
    STAT_ATTR(SHEAF_PREFILL_SLOW, sheaf_prefill_slow);
    STAT_ATTR(SHEAF_PREFILL_OVERSIZE, sheaf_prefill_oversize);
    STAT_ATTR(SHEAF_RETURN_FAST, sheaf_return_fast);
    STAT_ATTR(SHEAF_RETURN_SLOW, sheaf_return_slow);

#[no_mangle]
unsafe extern "C" fn skip_kfence_show(s: *mut kmem_cache, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", !!(s.flags & SLAB_SKIP_KFENCE));
    }
#[no_mangle]
pub unsafe extern "C" fn skip_kfence_store(s: *mut kmem_cache, buf: *mut c_char, length: size_t) -> ssize_t {
pub static mut ret: c_int = 0;
    if (buf[0] == '0') {
    s.flags &= ~SLAB_SKIP_KFENCE;
    }

    else if (buf[0] == '1') {
    s.flags |= SLAB_SKIP_KFENCE;
    }
    else {
    ret = -EINVAL;
    }
    return ret;
    }
    SLAB_ATTR(skip_kfence);

    static const struct attribute *const slab_attrs[] = {
    &slab_size_attr.attr,
    &object_size_attr.attr,
    &objs_per_slab_attr.attr,
    &order_attr.attr,
    &sheaf_capacity_attr.attr,
    &min_partial_attr.attr,
    &cpu_partial_attr.attr,
    &objects_partial_attr.attr,
    &partial_attr.attr,
    &cpu_slabs_attr.attr,
    &ctor_attr.attr,
    &aliases_attr.attr,
    &align_attr.attr,
    &hwcache_align_attr.attr,
    &reclaim_account_attr.attr,
    &destroy_by_rcu_attr.attr,
    &shrink_attr.attr,
    &slabs_cpu_partial_attr.attr,

    &total_objects_attr.attr,
    &objects_attr.attr,
    &slabs_attr.attr,
    &sanity_checks_attr.attr,
    &trace_attr.attr,
    &red_zone_attr.attr,
    &poison_attr.attr,
    &store_user_attr.attr,
    &validate_attr.attr,

    &cache_dma_attr.attr,

    &remote_node_defrag_ratio_attr.attr,

    &alloc_fastpath_attr.attr,
    &alloc_slowpath_attr.attr,
    &free_rcu_sheaf_attr.attr,
    &free_rcu_sheaf_fail_attr.attr,
    &free_fastpath_attr.attr,
    &free_slowpath_attr.attr,
    &free_add_partial_attr.attr,
    &free_remove_partial_attr.attr,
    &alloc_slab_attr.attr,
    &alloc_node_mismatch_attr.attr,
    &free_slab_attr.attr,
    &order_fallback_attr.attr,
    &cmpxchg_double_fail_attr.attr,
    &sheaf_flush_attr.attr,
    &sheaf_refill_attr.attr,
    &sheaf_alloc_attr.attr,
    &sheaf_free_attr.attr,
    &barn_get_attr.attr,
    &barn_get_fail_attr.attr,
    &barn_put_attr.attr,
    &barn_put_fail_attr.attr,
    &sheaf_prefill_fast_attr.attr,
    &sheaf_prefill_slow_attr.attr,
    &sheaf_prefill_oversize_attr.attr,
    &sheaf_return_fast_attr.attr,
    &sheaf_return_slow_attr.attr,

    &failslab_attr.attr,

    &usersize_attr.attr,

    &skip_kfence_attr.attr,

    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(slab);
#[no_mangle]
pub unsafe extern "C" fn slab_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
pub static mut attribute: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    attribute = to_slab_attr(attr);
    s = to_slab(kobj);
    if (!attribute.show) {
    return -EIO;
    }
    return attribute.show(s, buf);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char, len: size_t) -> ssize_t {
pub static mut attribute: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
    attribute = to_slab_attr(attr);
    s = to_slab(kobj);
    if (!attribute.store) {
    return -EIO;
    }
    return attribute.store(s, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn kmem_cache_release(k: *mut kobject) {
    slab_kmem_cache_release(to_slab(k));
    }
pub static mut sysfs_ops: usize = 0;
pub static mut kobj_type: usize = 0;
pub static mut slab_kset: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn cache_kset(s: *mut kmem_cache) -> *mut c_void {
    return slab_kset;
    }
pub const ID_STR_LENGTH: c_int = 32;
// Create a unique string id for a slab cache:
//
// Format	:[flags-]size
//
#[no_mangle]
pub unsafe extern "C" fn create_unique_id(s: *mut kmem_cache) -> *mut c_void {
    let mut name = kmalloc(ID_STR_LENGTH, GFP_KERNEL);
    let mut p = name;
    if (!name) {
    return ERR_PTR(-ENOMEM);
    }
// p++ = ':';
//
// First flags affecting slabcache operations. We will only
// get here for aliasable slabs so we do not need to support
// too many flags. The flags here must cover all flags that
// are matched during merging to guarantee that the id is
// unique.
//
    if (s.flags & SLAB_CACHE_DMA) {
// p++ = 'd';
    }
    if (s.flags & SLAB_CACHE_DMA32) {
// p++ = 'D';
    }
    if (s.flags & SLAB_RECLAIM_ACCOUNT) {
// p++ = 'a';
    }
    if (s.flags & SLAB_CONSISTENCY_CHECKS) {
// p++ = 'F';
    }
    if (s.flags & SLAB_ACCOUNT) {
// p++ = 'A';
    }
    if (p != name + 1) {
// p++ = '-';
    }
    p += snprintf(p, ID_STR_LENGTH - (p - name), "%07u", s.size);
    if (WARN_ON!(p > name + ID_STR_LENGTH - 1)) {
    kfree(name);
    return ERR_PTR(-EINVAL);
    }
    kmsan_unpoison_memory(name, p - name);
    return name;
    }
#[no_mangle]
unsafe extern "C" fn sysfs_slab_add(s: *mut kmem_cache) -> c_int {
    let mut err = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut kset = cache_kset(s);
pub static mut unmergeable: c_int = 0;
    if (!unmergeable && disable_higher_order_debug &&
    (slub_debug & DEBUG_METADATA_FLAGS)) {
    unmergeable = 1;
    }
    if (unmergeable) {
//
// Slabcache can never be merged so we can use the name proper.
// This is typically the case for debug situations. In that
// case we can catch duplicate names easily.
//
    sysfs_remove_link(&slab_kset.kobj, s.name);
    name = s.name;
    } else {
//
// Create a unique name for the slab as a target
// for the symlinks.
//
    name = create_unique_id(s);
    if (IS_ERR(name)) {
    return PTR_ERR(name);
    }
    }
    s.kobj.kset = kset;
    err = kobject_init_and_add(&s.kobj, &slab_ktype, core::ptr::null_mut(), "%s", name);
//
// Intentionally skip kobject_put(). See commit 2420baa8e046
// ("mm/slab: Allow cache creation to proceed even if sysfs
// registration fails")
//
    if (err) {
// goto;
    }
    if (!unmergeable) {
// Setup first alias
    sysfs_slab_alias(s, s.name);
    }
// label;
    if (!unmergeable) {
    kfree(name);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_slab_unlink(s: *mut kmem_cache) {
    if (s.kobj.state_in_sysfs) {
    kobject_del(&s.kobj);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_slab_release(s: *mut kmem_cache) {
    kobject_put(&s.kobj);
    }
//
// Need to buffer aliases during bootup until sysfs becomes
// available lest we lose that information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_alias {
    pub s: *mut kmem_cache,
    pub name: *const c_char,
    pub next: *mut saved_alias,
}

pub static mut alias_list: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn sysfs_slab_alias(s: *mut kmem_cache, name: *const c_char) -> c_int {
pub static mut al: *mut c_void = core::ptr::null_mut();
    if (slab_state == FULL) {
//
// If we have a leftover link then remove it.
//
    sysfs_remove_link(&slab_kset.kobj, name);
//
// The original cache may have failed to generate sysfs file.
// In that case, sysfs_create_link() returns -ENOENT and
// symbolic link creation is skipped.
//
    return sysfs_create_link(&slab_kset.kobj, &s.kobj, name);
    }
    al = kmalloc_obj(saved_alias);
    if (!al) {
    return -ENOMEM;
    }
    al.s = s;
    al.name = name;
    al.next = alias_list;
    alias_list = al;
    kmsan_unpoison_memory(al, sizeof!(*al));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_kset_init() -> c_int {
    slab_kset = kset_create_and_add("slab", core::ptr::null_mut(), kernel_kobj);
    if (!slab_kset) {
    pr_err!("Cannot register slab subsystem.\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_sysfs_process_aliases()  {
    let mut err = 0;
    while (alias_list) {
    let mut al = alias_list;
    alias_list = alias_list.next;
    err = sysfs_slab_alias(al.s, al.name);
    if (err) {
    pr_err!("SLUB: Unable to add boot slab alias %s to sysfs\n",
    al.name);
    }
    kfree(al);
    }
    }

    (defined(CONFIG_SLUB_DEBUG) && defined(CONFIG_DEBUG_FS))
#[no_mangle]
unsafe extern "C" fn slab_late_init() -> c_int {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    mutex_lock(&slab_mutex);
    err = slab_kset_init();
    if (err) {
// goto;
    }
    slab_debugfs_root_init();
    slab_state = FULL;
    list_for_each_entry(s, &slab_caches, list) {
    if (sysfs_slab_add(s)) {
    pr_err!("SLUB: Unable to add boot slab %s to sysfs\n",
    s.name);
    }
    if (s.flags & SLAB_STORE_USER) {
    debugfs_slab_add(s);
    }
    }
    slab_sysfs_process_aliases();
// label;
    mutex_unlock(&slab_mutex);
    return err;
    }
    late_initcall!(slab_late_init);

#[no_mangle]
unsafe extern "C" fn slab_debugfs_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let mut t = seq.private;
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    idx = (unsigned long) t.idx;
    if (idx < t.count) {
    l = &t.loc[idx];
    seq_printf(seq, "%7ld ", l.count);
    if (l.addr) {
    seq_printf(seq, "%pS", l.addr);
    }
    else {
    seq_puts(seq, "<not-available>");
    }
    if (l.waste) {
    seq_printf(seq, " waste=%lu/%lu",
    l.count * l.waste, l.waste);
    }
    if (l.sum_time != l.min_time) {
    seq_printf(seq, " age=%ld/%llu/%ld",
    l.min_time, div_u64(l.sum_time, l.count),
    l.max_time);
    } else {
    seq_printf(seq, " age=%ld", l.min_time);
    }
    if (l.min_pid != l.max_pid) {
    seq_printf(seq, " pid=%ld-%ld", l.min_pid, l.max_pid);
    }
    else {
    seq_printf(seq, " pid=%ld",
    l.min_pid);
    }
    if (num_online_cpus() > 1 && !cpumask_empty(to_cpumask(l.cpus))) {
    seq_printf(seq, " cpus=%*pbl",
    cpumask_pr_args(to_cpumask(l.cpus)));
    }
    if (nr_online_nodes > 1 && !nodes_empty(l.nodes)) {
    seq_printf(seq, " nodes=%*pbl",
    nodemask_pr_args(&l.nodes));
    }

    {
    let mut handle;
pub static mut entries: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
    let mut j = 0;
    handle = READ_ONCE(l.handle);
    if (handle) {
    nr_entries = stack_depot_fetch(handle, &entries);
    seq_puts(seq, "\n");
    for (j = 0; j < nr_entries; j++) {
    seq_printf(seq, "        %pS\n", entries[j]);
    }
    }
    }

    seq_puts(seq, "\n");
    }
    if (!idx && !t.count) {
    seq_puts(seq, "No data\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_debugfs_stop(seq: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
pub unsafe extern "C" fn slab_debugfs_next(seq: *mut seq_file, v: *mut c_void, ppos: *mut loff_t) -> *mut c_void {
    let mut t = seq.private;
    t.idx = ++(*ppos);
    if (*ppos <= t.count) {
    return ppos;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cmp_loc_by_count(a: *const c_void, b: *const c_void) -> c_int {
    let mut loc1 = a;
    let mut loc2 = b;
    return cmp_int(loc2.count, loc1.count);
    }
#[no_mangle]
pub unsafe extern "C" fn slab_debugfs_start(seq: *mut seq_file, ppos: *mut loff_t) -> *mut c_void {
    let mut t = seq.private;
    t.idx = *ppos;
    return ppos;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn slab_debug_trace_open(inode: *mut inode, filep: *mut file) -> c_int {
pub static mut n: *mut c_void = core::ptr::null_mut();
    enum track_item alloc;
    let mut node = 0;
    let mut t = __seq_open_private(filep, &slab_debugfs_sops,
    sizeof!(loc_track));
    let mut s = file_inode(filep).i_private;
pub static mut obj_map: *mut c_void = core::ptr::null_mut();
    if (!t) {
    return -ENOMEM;
    }
    obj_map = bitmap_alloc(oo_objects(s.oo), GFP_KERNEL);
    if (!obj_map) {
    seq_release_private(inode, filep);
    return -ENOMEM;
    }
    alloc = debugfs_get_aux_num(filep);
    if (!alloc_loc_track(t, PAGE_SIZE / sizeof!(location), GFP_KERNEL)) {
    bitmap_free(obj_map);
    seq_release_private(inode, filep);
    return -ENOMEM;
    }
    for_each_kmem_cache_node(s, node, n) {
    let mut flags = 0;
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (!node_nr_slabs(n)) {
    continue;
    }
    spin_lock_irqsave(&n.list_lock, flags);
    list_for_each_entry(slab, &n.partial, slab_list) {
    process_slab(t, s, slab, alloc, obj_map);
    }
    list_for_each_entry(slab, &n.full, slab_list) {
    process_slab(t, s, slab, alloc, obj_map);
    }
    spin_unlock_irqrestore(&n.list_lock, flags);
    }
// Sort locations by count
    sort(t.loc, t.count, sizeof!(location),
    cmp_loc_by_count, core::ptr::null_mut());
    bitmap_free(obj_map);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slab_debug_trace_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq = file.private_data;
    let mut t = seq.private;
    free_loc_track(t);
    return seq_release_private(inode, file);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn debugfs_slab_add(s: *mut kmem_cache) {
pub static mut slab_cache_dir: *mut c_void = core::ptr::null_mut();
    if (unlikely(!slab_debugfs_root)) {
    return;
    }
    slab_cache_dir = debugfs_create_dir(s.name, slab_debugfs_root);
    debugfs_create_file_aux_num("alloc_traces", 0400, slab_cache_dir, s,
    TRACK_ALLOC, &slab_debugfs_fops);
    debugfs_create_file_aux_num("free_traces", 0400, slab_cache_dir, s,
    TRACK_FREE, &slab_debugfs_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn debugfs_slab_release(s: *mut kmem_cache) {
    if (unlikely(!slab_debugfs_root)) {
    return;
    }
    debugfs_lookup_and_remove(s.name, slab_debugfs_root);
    }
#[no_mangle]
unsafe extern "C" fn slab_debugfs_root_init()  {
    slab_debugfs_root = debugfs_create_dir("slab", core::ptr::null_mut());
    }

//
// The /proc/slabinfo ABI
//

#[no_mangle]
pub unsafe extern "C" fn get_slabinfo(s: *mut kmem_cache, sinfo: *mut slabinfo) {
pub static mut nr_slabs: c_ulong = 0;
pub static mut nr_objs: c_ulong = 0;
pub static mut nr_free: c_ulong = 0;
    let mut node = 0;
pub static mut n: *mut c_void = core::ptr::null_mut();
    for_each_kmem_cache_node(s, node, n) {
    nr_slabs += node_nr_slabs(n);
    nr_objs += node_nr_objs(n);
    nr_free += count_partial_free_approx(n);
    }
    sinfo.active_objs = nr_objs - nr_free;
    sinfo.num_objs = nr_objs;
    sinfo.active_slabs = nr_slabs;
    sinfo.num_slabs = nr_slabs;
    sinfo.objects_per_slab = oo_objects(s.oo);
    sinfo.cache_order = oo_order(s.oo);