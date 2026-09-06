//! Automatically rewritten from C to Rust
//! Source: mm/zswap.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// zswap.c - zswap driver file
//
// zswap is a cache that takes pages that are in the process
// of being swapped out and attempts to compress and store them in a
// RAM-based memory pool.  This can result in a significant I/O reduction on
// the swap device and, in the case where decompressing from RAM is faster
// than reading from the swap device, can also improve workload performance.
//
// Copyright (C) 2012  Seth Jennings <sjenning@linux.vnet.ibm.com>
//

//
// statistics
//
// The number of pages currently stored in zswap
pub static mut zswap_stored_pages: atomic_long_t = 0;
// The number of incompressible pages currently stored in zswap
pub static mut zswap_stored_incompressible_pages: atomic_long_t = 0;
//
// The statistics below are not protected from concurrent access for
// performance reasons so they may not be a 100% accurate.  However,
// they do provide useful information on roughly how many times a
// certain event is occurring.
//
// Pool limit was hit (see zswap_max_pool_percent)
    static u64 zswap_pool_limit_hit;
// Pages written back when pool limit was reached
    static u64 zswap_written_back_pages;
// Store failed due to a reclaim failure after pool limit was reached
    static u64 zswap_reject_reclaim_fail;
// Store failed due to compression algorithm failure
    static u64 zswap_reject_compress_fail;
// Compressed page was too big for the allocator to (optimally) store
    static u64 zswap_reject_compress_poor;
// Load or writeback failed due to decompression failure
    static u64 zswap_decompress_fail;
// Store failed because underlying allocator could not get memory
    static u64 zswap_reject_alloc_fail;
// Store failed because the entry metadata could not be allocated (rare)
    static u64 zswap_reject_kmemcache_fail;
// Shrinker work queue
pub static mut shrink_wq: *mut c_void = core::ptr::null_mut();
// Pool limit was hit, we need to calm down
    static bool zswap_pool_reached_full;
//
// tunables
//

// forward_decl: zswap_setup;
// Enable/disable zswap
pub static mut CONFIG_ZSWAP_DEFAULT_ON: usize = 0;
pub static mut zswap_enabled: bool = false;
// forward_decl: zswap_enabled_param_set;
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(enabled, &zswap_enabled_param_ops, &zswap_enabled, 0644);
// Crypto compressor to use
    static char *zswap_compressor = CONFIG_ZSWAP_COMPRESSOR_DEFAULT;
// forward_decl: zswap_compressor_param_set;
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(compressor, &zswap_compressor_param_ops,
    &zswap_compressor, 0644);
// The maximum percentage of memory that the compressed pool can occupy
pub static mut zswap_max_pool_percent: unsigned int = 20;
    module_param_named!(max_pool_percent, zswap_max_pool_percent, uint, 0644);
// The threshold for accepting new pages after the max_pool_percent was hit
    static unsigned int zswap_accept_thr_percent = 90; /* of max pool size */
    module_param_named!(accept_threshold_percent, zswap_accept_thr_percent,
    uint, 0644);
// Enable/disable memory pressure-based shrinker.
    static bool zswap_shrinker_enabled = IS_ENABLED!(
    CONFIG_ZSWAP_SHRINKER_DEFAULT_ON);
    module_param_named!(shrinker_enabled, zswap_shrinker_enabled, bool, 0644);
#[no_mangle]
pub unsafe extern "C" fn zswap_is_enabled() -> bool {
    return zswap_enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_never_enabled() -> bool {
    return !static_branch_maybe(CONFIG_ZSWAP_DEFAULT_ON, &zswap_ever_enabled);
    }
//
// data structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_acomp_ctx {
    pub acomp: *mut crypto_acomp,
    pub req: *mut acomp_req,
    pub wait: crypto_wait,
    pub buffer: *mut u8,
    pub mutex: mutex,
}

//
// The lock ordering is zswap_tree.lock -> zswap_pool.lru_lock.
// The only case where lru_lock is not acquired while holding tree.lock is
// when a zswap_entry is taken off the lru for writeback, in that case it
// needs to be verified that it's still valid in the tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zswap_pool {
    pub zs_pool: *mut zs_pool,
    pub acomp_ctx: *mut crypto_acomp_ctx ,
    pub ref: percpu_ref,
    pub list: list_head,
    pub release_work: work_struct,
    pub node: hlist_node,
    pub tfm_name: [c_char; CRYPTO_MAX_ALG_NAME],
}

// Global LRU lists shared by all zswap pools.
pub static mut zswap_list_lru: usize = 0;
// The lock protects zswap_next_shrink updates.
pub static mut zswap_shrink_lock: usize = 0;
pub static mut zswap_next_shrink: *mut c_void = core::ptr::null_mut();
pub static mut zswap_shrink_work: usize = 0;
pub static mut zswap_shrinker: *mut c_void = core::ptr::null_mut();
//
// struct zswap_entry
//
// This structure contains the metadata for tracking a single compressed
// page within zswap.
//
// swpentry - associated swap entry, the offset indexes into the xarray
// length - the length in bytes of the compressed page data.  Needed during
// decompression.
// referenced - true if the entry recently entered the zswap pool. Unset by the
// writeback logic. The entry is only reclaimed by the writeback
// logic if referenced is unset. See comments in the shrinker
// section for context.
// pool - the zswap_pool the entry's data is in
// handle - zsmalloc allocation handle that stores the compressed page data
// objcg - the obj_cgroup that the compressed memory is charged to
// lru - handle to the pool's lru used to evict pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zswap_entry {
    pub swpentry: swp_entry_t,
    pub length: c_uint,
    pub referenced: bool,
    pub pool: *mut zswap_pool,
    pub handle: c_ulong,
    pub objcg: *mut obj_cgroup,
    pub lru: list_head,
}

    static struct xarray *zswap_trees[MAX_SWAPFILES];
    static unsigned int nr_zswap_trees[MAX_SWAPFILES];
// RCU-protected iteration
pub static mut zswap_pools: usize = 0;
// protects zswap_pools list modification
pub static mut zswap_pools_lock: usize = 0;
// pool counter to provide unique names to zsmalloc
pub static mut zswap_pools_count: atomic_t = 0;
    enum zswap_init_type {
    ZSWAP_UNINIT,
    ZSWAP_INIT_SUCCEED,
    ZSWAP_INIT_FAILED
    };
    static enum zswap_init_type zswap_init_state;
// used to ensure the integrity of initialization
pub static mut zswap_init_lock: usize = 0;
// init completed, but couldn't create the initial pool
    static bool zswap_has_pool;
//
// helpers and fwd declarations
//
// One swap address space for each 64M swap space
pub const ZSWAP_ADDRESS_SPACE_SHIFT: c_int = 14;

#[no_mangle]
pub unsafe extern "C" fn swap_zswap_tree(swp: swp_entry_t) -> *mut c_void {
    return &zswap_trees[swp_type(swp)][swp_offset(swp)
    >> ZSWAP_ADDRESS_SPACE_SHIFT];
    }

    pr_debug!("%s pool %s\n", msg, (p).tfm_name)
//
// pool functions
//
// forward_decl: __zswap_pool_empty;
#[no_mangle]
unsafe extern "C" fn acomp_ctx_free(acomp_ctx: *mut crypto_acomp_ctx) {
    if (!acomp_ctx) {
    return;
    }
//
// If there was an error in allocating @acomp_ctx->req, it
// would be set to NULL.
//
    if (acomp_ctx.req) {
    acomp_request_free(acomp_ctx.req);
    }
    acomp_ctx.req = core::ptr::null_mut();
//
// We have to handle both cases here: an error pointer return from
// crypto_alloc_acomp_node(); and a) NULL initialization by zswap, or
// b) NULL assignment done in a previous call to acomp_ctx_free().
//
    if (!IS_ERR_OR_NULL(acomp_ctx.acomp)) {
    crypto_free_acomp(acomp_ctx.acomp);
    }
    acomp_ctx.acomp = core::ptr::null_mut();
    kfree(acomp_ctx.buffer);
    acomp_ctx.buffer = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_pool_create(compressor: *mut c_char) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    char name[38]; /* 'zswap' + 32 char (max) num + \0 */
    let mut ret = 0;
    let mut cpu = 0;
    if (!zswap_has_pool && !strcmp(compressor, ZSWAP_PARAM_UNSET)) {
    return core::ptr::null_mut();
    }
    pool = kzalloc_obj(*pool);
    if (!pool) {
    return core::ptr::null_mut();
    }
// unique name for each pool specifically required by zsmalloc
    snprintf(name, 38, "zswap%x", atomic_inc_return(&zswap_pools_count));
    pool.zs_pool = zs_create_pool(name);
    if (!pool.zs_pool) {
// goto;
    }
    strscpy(pool.tfm_name, compressor, sizeof!(pool.tfm_name));
// Many things rely on the zero-initialization.
    pool.acomp_ctx = alloc_percpu_gfp(*pool.acomp_ctx,
    GFP_KERNEL | __GFP_ZERO);
    if (!pool.acomp_ctx) {
    pr_err!("percpu alloc failed\n");
// goto;
    }
//
// This is serialized against CPU hotplug operations. Hence, cores
// cannot be offlined until this finishes.
//
    ret = cpuhp_state_add_instance(CPUHP_MM_ZSWP_POOL_PREPARE,
    &pool.node);
//
// cpuhp_state_add_instance() will not cleanup on failure since
// we don't register a hotunplug callback.
//
    if (ret) {
// goto;
    }
// being the current pool takes 1 ref; this func expects the
// caller to always add the new pool as the current pool
//
    ret = percpu_ref_init(&pool.ref, __zswap_pool_empty,
    PERCPU_REF_ALLOW_REINIT, GFP_KERNEL);
    if (ret) {
// goto;
    }
    INIT_LIST_HEAD(&pool.list);
    zswap_pool_debug("created", pool);
    return pool;
// label;
    cpuhp_state_remove_instance(CPUHP_MM_ZSWP_POOL_PREPARE, &pool.node);
// label;
    for_each_possible_cpu(cpu) {
    acomp_ctx_free(per_cpu_ptr(pool.acomp_ctx, cpu));
    }
// label;
    if (pool.acomp_ctx) {
    free_percpu(pool.acomp_ctx);
    }
    if (pool.zs_pool) {
    zs_destroy_pool(pool.zs_pool);
    }
    kfree(pool);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __zswap_pool_create_fallback() -> *mut c_void {
    if (!crypto_has_acomp(zswap_compressor, 0, 0) &&
    strcmp(zswap_compressor, CONFIG_ZSWAP_COMPRESSOR_DEFAULT)) {
    pr_err!("compressor %s not available, using default %s\n",
    zswap_compressor, CONFIG_ZSWAP_COMPRESSOR_DEFAULT);
    param_free_charp(&zswap_compressor);
    zswap_compressor = CONFIG_ZSWAP_COMPRESSOR_DEFAULT;
    }
// Default compressor should be available. Kconfig bug?
    if (WARN_ON_ONCE!(!crypto_has_acomp(zswap_compressor, 0, 0))) {
    zswap_compressor = ZSWAP_PARAM_UNSET;
    return core::ptr::null_mut();
    }
    return zswap_pool_create(zswap_compressor);
    }
#[no_mangle]
unsafe extern "C" fn zswap_pool_destroy(pool: *mut zswap_pool) {
    let mut cpu = 0;
    zswap_pool_debug("destroying", pool);
    cpuhp_state_remove_instance(CPUHP_MM_ZSWP_POOL_PREPARE, &pool.node);
    for_each_possible_cpu(cpu) {
    acomp_ctx_free(per_cpu_ptr(pool.acomp_ctx, cpu));
    }
    free_percpu(pool.acomp_ctx);
    zs_destroy_pool(pool.zs_pool);
    kfree(pool);
    }
#[no_mangle]
unsafe extern "C" fn __zswap_pool_release(work: *mut work_struct) {
    let mut pool = container_of!(work, typeof(*pool),
    release_work);
    synchronize_rcu();
// nobody should have been able to get a ref...
    WARN_ON!(!percpu_ref_is_zero(&pool.ref));
    percpu_ref_exit(&pool.ref);
// pool is now off zswap_pools list and has no references.
    zswap_pool_destroy(pool);
    }
// forward_decl: zswap_pool_current;
#[no_mangle]
unsafe extern "C" fn __zswap_pool_empty(ref: *mut percpu_ref) {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    pool = container_of!(ref, typeof(*pool), ref);
    spin_lock_bh(&zswap_pools_lock);
    WARN_ON!(pool == zswap_pool_current());
    list_del_rcu(&pool.list);
    INIT_WORK(&pool.release_work, __zswap_pool_release);
    schedule_work(&pool.release_work);
    spin_unlock_bh(&zswap_pools_lock);
    }
#[no_mangle]
unsafe extern "C" fn zswap_pool_tryget(pool: *mut zswap_pool) -> int __must_check {
    if (!pool) {
    return 0;
    }
    return percpu_ref_tryget(&pool.ref);
    }
// The caller must already have a reference.
#[no_mangle]
unsafe extern "C" fn zswap_pool_get(pool: *mut zswap_pool) {
    percpu_ref_get(&pool.ref);
    }
#[no_mangle]
unsafe extern "C" fn zswap_pool_put(pool: *mut zswap_pool) {
    percpu_ref_put(&pool.ref);
    }
#[no_mangle]
pub unsafe extern "C" fn __zswap_pool_current() -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    pool = list_first_or_null_rcu(&zswap_pools, typeof(*pool), list);
    WARN_ONCE(!pool && zswap_has_pool,
    "%s: no page storage pool!\n", __func__);
    return pool;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_pool_current() -> *mut c_void {
    assert_spin_locked(&zswap_pools_lock);
    return __zswap_pool_current();
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_pool_current_get() -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    pool = __zswap_pool_current();
    if (!zswap_pool_tryget(pool)) {
    pool = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return pool;
    }
// type and compressor must be null-terminated
#[no_mangle]
pub unsafe extern "C" fn zswap_pool_find_get(compressor: *mut c_char) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    assert_spin_locked(&zswap_pools_lock);
    list_for_each_entry_rcu(pool, &zswap_pools, list) {
    if (strcmp(pool.tfm_name, compressor)) {
    continue;
    }
// if we can't get it, it's about to be destroyed
    if (!zswap_pool_tryget(pool)) {
    continue;
    }
    return pool;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn zswap_max_pages() -> c_ulong {
    return totalram_pages() * zswap_max_pool_percent / 100;
    }
#[no_mangle]
unsafe extern "C" fn zswap_accept_thr_pages() -> c_ulong {
    return zswap_max_pages() * zswap_accept_thr_percent / 100;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_total_pages() -> c_ulong {
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut total: c_ulong = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(pool, &zswap_pools, list) {
    total += zs_get_total_pages(pool.zs_pool);
    }
    rcu_read_unlock();
    return total;
    }
#[no_mangle]
unsafe extern "C" fn zswap_check_limits() -> bool {
pub static mut cur_pages: c_ulong = 0;
pub static mut max_pages: c_ulong = 0;
    if (cur_pages >= max_pages) {
    zswap_pool_limit_hit += 1;
    zswap_pool_reached_full = true;
    } else if (zswap_pool_reached_full &&
    cur_pages <= zswap_accept_thr_pages()) {
    zswap_pool_reached_full = false;
    }
    return zswap_pool_reached_full;
    }
//
// param callbacks
//
#[no_mangle]
unsafe extern "C" fn zswap_compressor_param_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    struct zswap_pool *pool, *put_pool = core::ptr::null_mut();
    let mut s = strstrip(val);
pub static mut create_pool: bool = false;
pub static mut ret: c_int = 0;
    mutex_lock(&zswap_init_lock);
    match (zswap_init_state) {
    ZSWAP_UNINIT => {
// Handled in zswap_setup()
    ret = param_set_charp(s, kp);
    // break;
    }
    ZSWAP_INIT_SUCCEED => {
    if (!zswap_has_pool || strcmp(s, *kp.arg)) {
    create_pool = true;
    }
    // break;
    }
    ZSWAP_INIT_FAILED => {
    pr_err!("can't set param, initialization failed\n");
    ret = -ENODEV;
    }
    }
    mutex_unlock(&zswap_init_lock);
    if (!create_pool) {
    return ret;
    }
    if (!crypto_has_acomp(s, 0, 0)) {
    pr_err!("compressor %s not available\n", s);
    return -ENOENT;
    }
    spin_lock_bh(&zswap_pools_lock);
    pool = zswap_pool_find_get(s);
    if (pool) {
    zswap_pool_debug("using existing", pool);
    WARN_ON!(pool == zswap_pool_current());
    list_del_rcu(&pool.list);
    }
    spin_unlock_bh(&zswap_pools_lock);
    if (!pool) {
    pool = zswap_pool_create(s);
    }
    else {
//
// Restore the initial ref dropped by percpu_ref_kill()
// when the pool was decommissioned and switch it again
// to percpu mode.
//
    percpu_ref_resurrect(&pool.ref);
// Drop the ref from zswap_pool_find_get().
    zswap_pool_put(pool);
    }
    if (pool) {
    ret = param_set_charp(s, kp);
    }
    else {
    ret = -EINVAL;
    }
    spin_lock_bh(&zswap_pools_lock);
    if (!ret) {
    put_pool = zswap_pool_current();
    list_add_rcu(&pool.list, &zswap_pools);
    zswap_has_pool = true;
    } else if (pool) {
//
// Add the possibly pre-existing pool to the end of the pools
// list; if it's new (and empty) then it'll be removed and
// destroyed by the put after we drop the lock
//
    list_add_tail_rcu(&pool.list, &zswap_pools);
    put_pool = pool;
    }
    spin_unlock_bh(&zswap_pools_lock);
//
// Drop the ref from either the old current pool,
// or the new pool we failed to add
//
    if (put_pool) {
    percpu_ref_kill(&put_pool.ref);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_enabled_param_set(val: *mut c_char, kp: *mut kernel_param) -> c_int {
pub static mut ret: c_int = 0;
// if this is load-time (pre-init) param setting, only set param.
    if (system_state != SYSTEM_RUNNING) {
    return param_set_bool(val, kp);
    }
    mutex_lock(&zswap_init_lock);
    match (zswap_init_state) {
    ZSWAP_UNINIT => {
    if (zswap_setup()) {
    // break;
    }
    fallthrough;
    }
    ZSWAP_INIT_SUCCEED => {
    if (!zswap_has_pool) {
    pr_err!("can't enable, no pool configured\n");
    }
    else {
    ret = param_set_bool(val, kp);
    }
    // break;
    }
    ZSWAP_INIT_FAILED => {
    pr_err!("can't enable, initialization failed\n");
    }
    }
    mutex_unlock(&zswap_init_lock);
    return ret;
    }
//
// lru functions
//
// should be called under RCU

#[no_mangle]
pub unsafe extern "C" fn mem_cgroup_from_entry(entry: *mut zswap_entry) -> *mut c_void {
    return entry.objcg ? obj_cgroup_memcg(entry.objcg) : core::ptr::null_mut();
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mem_cgroup_from_entry
pub unsafe extern "C" fn mem_cgroup_from_entry_dup(entry: *mut zswap_entry) -> *mut c_void {
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn entry_to_nid(entry: *mut zswap_entry) -> c_int {
    return page_to_nid(virt_to_page(entry));
    }
#[no_mangle]
unsafe extern "C" fn zswap_lru_add(entry: *mut zswap_entry) {
pub static mut nid: c_int = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
//
// Note that it is safe to use rcu_read_lock() here, even in the face of
// concurrent memcg offlining:
//
// 1. list_lru_add() is called before list_lru_one is dead. The
// new entry will be reparented to memcg's parent's list_lru.
// 2. list_lru_add() is called after list_lru_one is dead. The
// new entry will be added directly to memcg's parent's list_lru.
//
// Similar reasoning holds for list_lru_del().
//
    rcu_read_lock();
    memcg = mem_cgroup_from_entry(entry);
// will always succeed
    list_lru_add(&zswap_list_lru, &entry.lru, nid, memcg);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn zswap_lru_del(entry: *mut zswap_entry) {
pub static mut nid: c_int = 0;
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    memcg = mem_cgroup_from_entry(entry);
// will always succeed
    list_lru_del(&zswap_list_lru, &entry.lru, nid, memcg);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_lruvec_state_init(lruvec: *mut lruvec) {
    atomic_long_set(&lruvec.zswap_lruvec_state.nr_disk_swapins, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_folio_swapin(folio: *mut folio) {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    if (folio) {
    rcu_read_lock();
    lruvec = folio_lruvec(folio);
    atomic_long_inc(&lruvec.zswap_lruvec_state.nr_disk_swapins);
    rcu_read_unlock();
    }
    }
//
// This function should be called when a memcg is being offlined.
//
// Since the global shrinker shrink_worker() may hold a reference
// of the memcg, we must check and release the reference in
// zswap_next_shrink.
//
// shrink_worker() must handle the case where this function releases
// the reference of memcg being shrunk.
//
#[no_mangle]
pub unsafe extern "C" fn zswap_memcg_offline_cleanup(memcg: *mut mem_cgroup) {
// lock out zswap shrinker walking memcg tree
    spin_lock(&zswap_shrink_lock);
    if (zswap_next_shrink == memcg) {
    do {
    zswap_next_shrink = mem_cgroup_iter(core::ptr::null_mut(), zswap_next_shrink, core::ptr::null_mut());
    } while (zswap_next_shrink && !mem_cgroup_online(zswap_next_shrink));
    }
    spin_unlock(&zswap_shrink_lock);
    }
//
// zswap entry functions
//
pub static mut zswap_entry_cache: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn zswap_entry_cache_alloc(gfp: gfp_t, nid: c_int) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    entry = kmem_cache_alloc_node(zswap_entry_cache, gfp, nid);
    if (!entry) {
    return core::ptr::null_mut();
    }
    return entry;
    }
#[no_mangle]
unsafe extern "C" fn zswap_entry_cache_free(entry: *mut zswap_entry) {
    kmem_cache_free(zswap_entry_cache, entry);
    }
//
// Carries out the common pattern of freeing an entry's zsmalloc allocation,
// freeing the entry itself, and decrementing the number of stored pages.
//
#[no_mangle]
unsafe extern "C" fn zswap_entry_free(entry: *mut zswap_entry) {
    zswap_lru_del(entry);
    zs_free(entry.pool.zs_pool, entry.handle);
    zswap_pool_put(entry.pool);
    if (entry.objcg) {
    obj_cgroup_uncharge_zswap(entry.objcg, entry.length);
    obj_cgroup_put(entry.objcg);
    }
    if (entry.length == PAGE_SIZE) {
    atomic_long_dec(&zswap_stored_incompressible_pages);
    }
    zswap_entry_cache_free(entry);
    atomic_long_dec(&zswap_stored_pages);
    }
//
// compressed storage functions
//
#[no_mangle]
unsafe extern "C" fn zswap_cpu_comp_prepare(cpu: c_uint, node: *mut hlist_node) -> c_int {
    let mut pool = hlist_entry(node, zswap_pool, node);
    let mut acomp_ctx = per_cpu_ptr(pool.acomp_ctx, cpu);
pub static mut ret: c_int = 0;
//
// To handle cases where the CPU goes through online-offline-online
// transitions, we return if the acomp_ctx has already been initialized.
//
    if (acomp_ctx.acomp) {
    WARN_ON_ONCE!(IS_ERR(acomp_ctx.acomp));
    return 0;
    }
    acomp_ctx.buffer = kmalloc_node(PAGE_SIZE, GFP_KERNEL, cpu_to_node(cpu));
    if (!acomp_ctx.buffer) {
    return ret;
    }
//
// In case of an error, crypto_alloc_acomp_node() returns an
// error pointer, never NULL.
//
    acomp_ctx.acomp = crypto_alloc_acomp_node(pool.tfm_name, 0, 0, cpu_to_node(cpu));
    if (IS_ERR(acomp_ctx.acomp)) {
    pr_err!("could not alloc crypto acomp %s : %pe\n",
    pool.tfm_name, acomp_ctx.acomp);
    ret = PTR_ERR(acomp_ctx.acomp);
// goto;
    }
// acomp_request_alloc() returns NULL in case of an error.
    acomp_ctx.req = acomp_request_alloc(acomp_ctx.acomp);
    if (!acomp_ctx.req) {
    pr_err!("could not alloc crypto acomp_request %s\n",
    pool.tfm_name);
// goto;
    }
    crypto_init_wait(&acomp_ctx.wait);
//
// if the backend of acomp is async zip, crypto_req_done() will wakeup
// crypto_wait_req(); if the backend of acomp is scomp, the callback
// won't be called, crypto_wait_req() will return without blocking.
//
    acomp_request_set_callback(acomp_ctx.req, CRYPTO_TFM_REQ_MAY_BACKLOG,
    crypto_req_done, &acomp_ctx.wait);
    mutex_init(&acomp_ctx.mutex);
    return 0;
// label;
    acomp_ctx_free(acomp_ctx);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_compress(page: *mut page, entry: *mut zswap_entry, pool: *mut zswap_pool) -> bool {
pub static mut acomp_ctx: *mut c_void = core::ptr::null_mut();
    struct scatterlist input, output;
pub static mut comp_ret: c_int = 0;
pub static mut dlen: c_uint = 0;
    let mut handle = 0;
    let mut gfp;
pub static mut dst: *mut c_void = core::ptr::null_mut();
pub static mut mapped: bool = false;
    acomp_ctx = raw_cpu_ptr(pool.acomp_ctx);
    mutex_lock(&acomp_ctx.mutex);
    dst = acomp_ctx.buffer;
    sg_init_table(&input, 1);
    sg_set_page(&input, page, PAGE_SIZE, 0);
    sg_init_one(&output, dst, PAGE_SIZE);
    acomp_request_set_params(acomp_ctx.req, &input, &output, PAGE_SIZE, dlen);
//
// it maybe looks a little bit silly that we send an asynchronous request,
// then wait for its completion synchronously. This makes the process look
// synchronous in fact.
// Theoretically, acomp supports users send multiple acomp requests in one
// acomp instance, then get those requests done simultaneously. but in this
// case, zswap actually does store and load page by page, there is no
// existing method to send the second page before the first page is done
// in one thread doing zswap.
// but in different threads running on different cpu, we have different
// acomp instance, so multiple threads can do (de)compression in parallel.
//
    comp_ret = crypto_wait_req(crypto_acomp_compress(acomp_ctx.req), &acomp_ctx.wait);
    dlen = acomp_ctx.req.dlen;
//
// If a page cannot be compressed into a size smaller than PAGE_SIZE,
// save the content as is without a compression, to keep the LRU order
// of writebacks.  If writeback is disabled, reject the page since it
// only adds metadata overhead.  swap_writeout() will put the page back
// to the active LRU list in the case.
//
    if (comp_ret || !dlen || dlen >= PAGE_SIZE) {
    rcu_read_lock();
    if (!mem_cgroup_zswap_writeback_enabled(
    folio_memcg(page_folio(page)))) {
    rcu_read_unlock();
    comp_ret = comp_ret ? comp_ret : -EINVAL;
// goto;
    }
    rcu_read_unlock();
    comp_ret = 0;
    dlen = PAGE_SIZE;
    dst = kmap_local_page(page);
    mapped = true;
    }
    gfp = GFP_NOWAIT | __GFP_NORETRY | __GFP_HIGHMEM | __GFP_MOVABLE;
    handle = zs_malloc(pool.zs_pool, dlen, gfp, page_to_nid(page));
    if (IS_ERR_VALUE(handle)) {
    alloc_ret = PTR_ERR(handle);
// goto;
    }
    zs_obj_write(pool.zs_pool, handle, dst, dlen);
    entry.handle = handle;
    entry.length = dlen;
// label;
    if (mapped) {
    kunmap_local(dst);
    }
    if (comp_ret == -ENOSPC || alloc_ret == -ENOSPC) {
    zswap_reject_compress_poor += 1;
    }

    else if (comp_ret) {
    zswap_reject_compress_fail += 1;
    }

    else if (alloc_ret) {
    zswap_reject_alloc_fail += 1;
    }
    mutex_unlock(&acomp_ctx.mutex);
pub static mut comp_ret: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn zswap_decompress(entry: *mut zswap_entry, folio: *mut folio) -> bool {
    let mut pool = entry.pool;
    struct scatterlist input[2]; /* zsmalloc returns an SG list 1-2 entries */
pub static mut output: usize = 0;
pub static mut acomp_ctx: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    acomp_ctx = raw_cpu_ptr(pool.acomp_ctx);
    mutex_lock(&acomp_ctx.mutex);
    zs_obj_read_sg_begin(pool.zs_pool, entry.handle, input, entry.length);
// zswap entries of length PAGE_SIZE are not compressed.
    if (entry.length == PAGE_SIZE) {
pub static mut dst: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(input.length != PAGE_SIZE);
    dst = kmap_local_folio(folio, 0);
    memcpy_from_sglist(dst, input, 0, PAGE_SIZE);
    dlen = PAGE_SIZE;
    kunmap_local(dst);
    flush_dcache_folio(folio);
    } else {
    sg_init_table(&output, 1);
    sg_set_folio(&output, folio, PAGE_SIZE, 0);
    acomp_request_set_params(acomp_ctx.req, input, &output,
    entry.length, PAGE_SIZE);
    ret = crypto_acomp_decompress(acomp_ctx.req);
    ret = crypto_wait_req(ret, &acomp_ctx.wait);
    dlen = acomp_ctx.req.dlen;
    }
    zs_obj_read_sg_end(pool.zs_pool, entry.handle);
    mutex_unlock(&acomp_ctx.mutex);
    if (!ret && dlen == PAGE_SIZE) {
    return true;
    }
    zswap_decompress_fail += 1;
    pr_alert_ratelimited("Decompression error from zswap (%d:%lu %s %u.%d)\n",
    swp_type(entry.swpentry),
    swp_offset(entry.swpentry),
    entry.pool.tfm_name,
    entry.length, dlen);
    return false;
    }
//
// writeback code
//
// Attempts to free an entry by adding a folio to the swap cache,
// decompressing the entry data into the folio, and issuing a
// bio write to write the folio back to the swap device.
//
// This can be thought of as a "resumed writeback" of the folio
// to the swap device.  We are basically resuming the same swap
// writeback path that was intercepted with the zswap_store()
// in the first place.  After the folio has been decompressed into
// the swap cache, the compressed version stored by zswap can be
// freed.
//
#[no_mangle]
pub unsafe extern "C" fn zswap_writeback_entry(entry: *mut zswap_entry, swpentry: swp_entry_t) -> c_int {
pub static mut tree: *mut c_void = core::ptr::null_mut();
pub static mut offset: pgoff_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut mpol: *mut c_void = core::ptr::null_mut();
pub static mut si: *mut c_void = core::ptr::null_mut();
pub static mut ctx: swap_io_ctx = 0;
pub static mut ret: c_int = 0;
// try to allocate swap cache folio
    si = get_swap_device(swpentry);
    if (!si) {
    return -EEXIST;
    }
    mpol = get_task_policy(current);
    folio = swap_cache_alloc_folio(swpentry, GFP_KERNEL, BIT(0), core::ptr::null_mut(), mpol,
    NO_INTERLEAVE_INDEX);
    put_swap_device(si);
//
// Swap cache allocation might fail due to OOM, or the entry
// may already be cached due to concurrent swapin or have been
// freed. If already cached, a concurrent swapin made the folio
// hot, so skip it. For the unlikely concurrent shrinker case,
// it will be unlinked and freed when invalidated anyway.
//
    if (IS_ERR(folio)) {
    return PTR_ERR(folio);
    }
//
// folio is locked, and the swapcache is now secured against
// concurrent swapping to and from the slot, and concurrent
// swapoff so we can safely dereference the zswap tree here.
// Verify that the swap entry hasn't been invalidated and recycled
// behind our backs, to avoid overwriting a new swap folio with
// old compressed data. Only when this is successful can the entry
// be dereferenced.
//
    tree = swap_zswap_tree(swpentry);
    if (entry != xa_load(tree, offset)) {
    ret = -ENOMEM;
// goto;
    }
    if (!zswap_decompress(entry, folio)) {
    ret = -EIO;
// goto;
    }
    xa_erase(tree, offset);
    count_vm_event(ZSWPWB);
    if (entry.objcg) {
    count_objcg_events(entry.objcg, ZSWPWB, 1);
    }
    zswap_entry_free(entry);
// folio is up to date
    folio_mark_uptodate(folio);
// move it to the tail of the inactive list after end_writeback
    folio_set_reclaim(folio);
// start writeback
    __swap_writepage(&ctx, folio);
    swap_write_submit(&ctx);
// label;
    if (ret) {
    swap_cache_del_folio(folio);
    folio_unlock(folio);
    }
    folio_put(folio);
    return ret;
    }
//
// shrinker functions
//
// The dynamic shrinker is modulated by the following factors:
//
// 1. Each zswap entry has a referenced bit, which the shrinker unsets (giving
// the entry a second chance) before rotating it in the LRU list. If the
// entry is considered again by the shrinker, with its referenced bit unset,
// it is written back. The writeback rate as a result is dynamically
// adjusted by the pool activities - if the pool is dominated by new entries
// (i.e lots of recent zswapouts), these entries will be protected and
// the writeback rate will slow down. On the other hand, if the pool has a
// lot of stagnant entries, these entries will be reclaimed immediately,
// effectively increasing the writeback rate.
//
// 2. Swapins counter: If we observe swapins, it is a sign that we are
// overshrinking and should slow down. We maintain a swapins counter, which
// is consumed and subtract from the number of eligible objects on the LRU
// in zswap_shrinker_count().
//
// 3. Compression ratio. The better the workload compresses, the less gains we
// can expect from writeback. We scale down the number of objects available
// for reclaim by this ratio.
//
    static enum lru_status shrink_memcg_cb(list_head *item, list_lru_one *l,
    void *arg)
    {
    let mut entry = container_of!(item, zswap_entry, lru);
    let mut encountered_page_in_swapcache = arg;
    let mut swpentry;
pub static mut ret: lru_status = 0;
    let mut writeback_result = 0;
//
// Second chance algorithm: if the entry has its referenced bit set, give it
// a second chance. Only clear the referenced bit and rotate it in the
// zswap's LRU list.
//
    if (entry.referenced) {
    entry.referenced = false;
    return LRU_ROTATE;
    }
//
// As soon as we drop the LRU lock, the entry can be freed by
// a concurrent invalidation. This means the following:
//
// 1. We extract the swp_entry_t to the stack, allowing
// zswap_writeback_entry() to pin the swap entry and
// then validate the zswap entry against that swap entry's
// tree using pointer value comparison. Only when that
// is successful can the entry be dereferenced.
//
// 2. Usually, objects are taken off the LRU for reclaim. In
// this case this isn't possible, because if reclaim fails
// for whatever reason, we have no means of knowing if the
// entry is alive to put it back on the LRU.
//
// So rotate it before dropping the lock. If the entry is
// written back or invalidated, the free path will unlink
// it. For failures, rotation is the right thing as well.
//
// Temporary failures, where the same entry should be tried
// again immediately, almost never happen for this shrinker.
// We don't do any trylocking; -ENOMEM comes closest,
// but that's extremely rare and doesn't happen spuriously
// either. Don't bother distinguishing this case.
//
    list_move_tail(item, &l.list);
//
// Once the lru lock is dropped, the entry might get freed. The
// swpentry is copied to the stack, and entry isn't deref'd again
// until the entry is verified to still be alive in the tree.
//
    swpentry = entry.swpentry;
//
// It's safe to drop the lock here because we return either
// LRU_REMOVED_RETRY, LRU_RETRY or LRU_STOP.
//
    spin_unlock(&l.lock);
    writeback_result = zswap_writeback_entry(entry, swpentry);
    if (writeback_result) {
    zswap_reject_reclaim_fail += 1;
    ret = LRU_RETRY;
//
// Encountering a page already in swap cache is a sign that we are shrinking
// into the warmer region. We should terminate shrinking (if we're in the dynamic
// shrinker context).
//
    if (writeback_result == -EEXIST && encountered_page_in_swapcache) {
    ret = LRU_STOP;
// encountered_page_in_swapcache = true;
    }
    } else {
    zswap_written_back_pages += 1;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_shrinker_scan(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut shrink_ret = 0;
pub static mut encountered_page_in_swapcache: bool = false;
    if (!zswap_shrinker_enabled ||
    !mem_cgroup_zswap_writeback_enabled(sc.memcg)) {
    sc.nr_scanned = 0;
    return SHRINK_STOP;
    }
    shrink_ret = list_lru_shrink_walk(&zswap_list_lru, sc, &shrink_memcg_cb,
    &encountered_page_in_swapcache);
    if (encountered_page_in_swapcache) {
    return SHRINK_STOP;
    }
    return shrink_ret ? shrink_ret : SHRINK_STOP;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_shrinker_count(shrinker: *mut shrinker, sc: *mut shrink_control) -> c_ulong {
    let mut memcg = sc.memcg;
    let mut lruvec = mem_cgroup_lruvec(memcg, NODE_DATA(sc.nid));
    let mut nr_disk_swapins = &lruvec.zswap_lruvec_state.nr_disk_swapins;
    unsigned long nr_backing, nr_stored, nr_freeable, nr_disk_swapins_cur,
    nr_remain;
    if (!zswap_shrinker_enabled || !mem_cgroup_zswap_writeback_enabled(memcg)) {
    return 0;
    }
//
// The shrinker resumes swap writeback, which will enter block
// and may enter fs. XXX: Harmonize with vmscan.c __GFP_FS
// rules (may_enter_fs()), which apply on a per-folio basis.
//
    if (!gfp_has_io_fs(sc.gfp_mask)) {
    return 0;
    }
//
// For memcg, use the cgroup-wide ZSWAP stats since we don't
// have them per-node and thus per-lruvec. Careful if memcg is
// runtime-disabled: we can get sc->memcg == NULL, which is ok
// for the lruvec, but not for memcg_page_state().
//
// Without memcg, use the zswap pool-wide metrics.
//
    if (!mem_cgroup_disabled()) {
    mem_cgroup_flush_stats_ratelimited(memcg);
    nr_backing = memcg_page_state(memcg, MEMCG_ZSWAP_B) >> PAGE_SHIFT;
    nr_stored = memcg_page_state(memcg, MEMCG_ZSWAPPED);
    } else {
    nr_backing = zswap_total_pages();
    nr_stored = atomic_long_read(&zswap_stored_pages);
    }
    if (!nr_stored) {
    return 0;
    }
    nr_freeable = list_lru_shrink_count(&zswap_list_lru, sc);
    if (!nr_freeable) {
    return 0;
    }
//
// Subtract from the lru size the number of pages that are recently swapped
// in from disk. The idea is that had we protect the zswap's LRU by this
// amount of pages, these disk swapins would not have happened.
//
    nr_disk_swapins_cur = atomic_long_read(nr_disk_swapins);
    do {
    if (nr_freeable >= nr_disk_swapins_cur) {
    nr_remain = 0;
    }
    else {
    nr_remain = nr_disk_swapins_cur - nr_freeable;
    }
    } while (!atomic_long_try_cmpxchg(
    nr_disk_swapins, &nr_disk_swapins_cur, nr_remain));
    nr_freeable -= nr_disk_swapins_cur - nr_remain;
    if (!nr_freeable) {
    return 0;
    }
//
// Scale the number of freeable pages by the memory saving factor.
// This ensures that the better zswap compresses memory, the fewer
// pages we will evict to swap (as it will otherwise incur IO for
// relatively small memory saving).
//
    return mult_frac(nr_freeable, nr_backing, nr_stored);
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_alloc_shrinker() -> *mut c_void {
pub static mut shrinker: *mut c_void = core::ptr::null_mut();
    shrinker =
    shrinker_alloc(SHRINKER_NUMA_AWARE | SHRINKER_MEMCG_AWARE, "mm-zswap");
    if (!shrinker) {
    return core::ptr::null_mut();
    }
    shrinker.scan_objects = zswap_shrinker_scan;
    shrinker.count_objects = zswap_shrinker_count;
    shrinker.batch = 0;
    shrinker.seeks = DEFAULT_SEEKS;
    return shrinker;
    }
//
// Scan up to SWAP_CLUSTER_MAX pages on each per-node zswap LRU of @memcg
// and write back the reclaimable ones.
//
// Return: 0 if at least one entry was written back, -EAGAIN if entries
// were scanned but none could be written back, or -ENOENT if @memcg has
// writeback disabled, is a zombie cgroup, or has empty zswap LRUs.
//
#[no_mangle]
unsafe extern "C" fn shrink_memcg(memcg: *mut mem_cgroup) -> c_int {
    int nid, shrunk = 0, scanned = 0;
    if (!mem_cgroup_zswap_writeback_enabled(memcg)) {
    return -ENOENT;
    }
//
// Skip zombies because their LRUs are reparented and we would be
// reclaiming from the parent instead of the dead memcg.
//
    if (memcg && !mem_cgroup_online(memcg)) {
    return -ENOENT;
    }
    for_each_node_state(nid, N_NORMAL_MEMORY) {
pub static mut nr_to_walk: c_ulong = 0;
    shrunk += list_lru_walk_one(&zswap_list_lru, nid, memcg,
    &shrink_memcg_cb, core::ptr::null_mut(), &nr_to_walk);
    scanned += SWAP_CLUSTER_MAX - nr_to_walk;
    }
// Nothing was scanned: every LRU under @memcg was empty.
    if (!scanned) {
    return -ENOENT;
    }
    return shrunk ? 0 : -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn shrink_worker(w: *mut work_struct) {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    int ret, failures = 0, attempts = 0;
    let mut thr = 0;
// Reclaim down to the accept threshold
    thr = zswap_accept_thr_pages();
//
// Global reclaim will select cgroup in a round-robin fashion from all
// online memcgs, but memcgs that have no pages in zswap and
// writeback-disabled memcgs (memory.zswap.writeback=0) are not
// candidates for shrinking.
//
// Shrinking will be aborted if we encounter the following
// MAX_RECLAIM_RETRIES times:
// - No writeback-candidate memcgs found in a memcg tree walk.
// - Shrinking a writeback-candidate memcg failed.
//
// We save iteration cursor memcg into zswap_next_shrink,
// which can be modified by the offline memcg cleaner
// zswap_memcg_offline_cleanup().
//
// Since the offline cleaner is called only once, we cannot leave an
// offline memcg reference in zswap_next_shrink.
// We can rely on the cleaner only if we get online memcg under lock.
//
// If we get an offline memcg, we cannot determine if the cleaner has
// already been called or will be called later. We must put back the
// reference before returning from this function. Otherwise, the
// offline memcg left in zswap_next_shrink will hold the reference
// until the next run of shrink_worker().
//
    do {
//
// Start shrinking from the next memcg after zswap_next_shrink.
// When the offline cleaner has already advanced the cursor,
// advancing the cursor here overlooks one memcg, but this
// should be negligibly rare.
//
// If we get an online memcg, keep the extra reference in case
// the original one obtained by mem_cgroup_iter() is dropped by
// zswap_memcg_offline_cleanup() while we are shrinking the
// memcg.
//
    spin_lock(&zswap_shrink_lock);
    do {
    memcg = mem_cgroup_iter(core::ptr::null_mut(), zswap_next_shrink, core::ptr::null_mut());
    zswap_next_shrink = memcg;
    } while (memcg && !mem_cgroup_tryget_online(memcg));
    spin_unlock(&zswap_shrink_lock);
//
// A NULL memcg ends a full hierarchy pass (except when memcg is
// disabled, where it is always NULL: fall through to the root LRU).
// Count a failure only if the last pass found no candidates.
//
    if (!memcg && !mem_cgroup_disabled()) {
    if (!attempts && ++failures == MAX_RECLAIM_RETRIES) {
    break;
    }
    attempts = 0;
// goto;
    }
    ret = shrink_memcg(memcg);
// drop the extra reference
    mem_cgroup_put(memcg);
//
// There are no writeback-candidate pages in the memcg.
// This is not an issue as long as we can find another memcg
// with pages in zswap. Skip this without incrementing attempts
// and failures.
//
    if (ret == -ENOENT) {
// goto;
    }
    attempts += 1;
    if (ret && ++failures == MAX_RECLAIM_RETRIES) {
    break;
    }
// label;
    cond_resched();
    } while (zswap_total_pages() > thr);
    }
//
// main API
//
#[no_mangle]
pub unsafe extern "C" fn zswap_store_page(page: *mut page, objcg: *mut obj_cgroup, pool: *mut zswap_pool) -> bool {
pub static mut page_swpentry: swp_entry_t = 0;
    let mut entry = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
// allocate entry
    entry = zswap_entry_cache_alloc(GFP_KERNEL, page_to_nid(page));
    if (!entry) {
    zswap_reject_kmemcache_fail += 1;
    return false;
    }
    if (!zswap_compress(page, entry, pool)) {
// goto;
    }
    old = xa_store(swap_zswap_tree(page_swpentry),
    swp_offset(page_swpentry),
    entry, GFP_KERNEL);
    if (xa_is_err(old)) {
pub static mut err: c_int = 0;
    WARN_ONCE(err != -ENOMEM, "unexpected xarray error: %d\n", err);
    zswap_reject_alloc_fail += 1;
// goto;
    }
//
// We may have had an existing entry that became stale when
// the folio was redirtied and now the new version is being
// swapped out. Get rid of the old.
//
    if (old) {
    zswap_entry_free(old);
    }
//
// The entry is successfully compressed and stored in the tree, there is
// no further possibility of failure. Grab refs to the pool and objcg,
// charge zswap memory, and increment zswap_stored_pages.
// The opposite actions will be performed by zswap_entry_free()
// when the entry is removed from the tree.
//
    zswap_pool_get(pool);
    if (objcg) {
    obj_cgroup_get(objcg);
    obj_cgroup_charge_zswap(objcg, entry.length);
    }
    atomic_long_inc(&zswap_stored_pages);
    if (entry.length == PAGE_SIZE) {
    atomic_long_inc(&zswap_stored_incompressible_pages);
    }
//
// We finish initializing the entry while it's already in xarray.
// This is safe because:
//
// 1. Concurrent stores and invalidations are excluded by folio lock.
//
// 2. Writeback is excluded by the entry not being on the LRU yet.
// The publishing order matters to prevent writeback from seeing
// an incoherent entry.
//
    entry.pool = pool;
    entry.swpentry = page_swpentry;
    entry.objcg = objcg;
    entry.referenced = true;
    if (entry.length) {
    INIT_LIST_HEAD(&entry.lru);
    zswap_lru_add(entry);
    }
    return true;
// label;
    zs_free(pool.zs_pool, entry.handle);
// label;
    zswap_entry_cache_free(entry);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_store(folio: *mut folio) -> bool {
pub static mut nr_pages: c_long = 0;
pub static mut swp: swp_entry_t = 0;
    let mut objcg = core::ptr::null_mut();
    let mut memcg = core::ptr::null_mut();
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = false;
    let mut index = 0;
    VM_WARN_ON_ONCE(!folio_test_locked(folio));
    VM_WARN_ON_ONCE(!folio_test_swapcache(folio));
    if (!zswap_enabled) {
// goto;
    }
    objcg = get_obj_cgroup_from_folio(folio);
    if (objcg && !obj_cgroup_may_zswap(objcg)) {
    memcg = get_mem_cgroup_from_objcg(objcg);
    if (shrink_memcg(memcg)) {
    mem_cgroup_put(memcg);
// goto;
    }
    mem_cgroup_put(memcg);
    }
    if (zswap_check_limits()) {
// goto;
    }
    pool = zswap_pool_current_get();
    if (!pool) {
// goto;
    }
    if (objcg) {
    memcg = get_mem_cgroup_from_objcg(objcg);
    if (memcg_list_lru_alloc(memcg, &zswap_list_lru, GFP_KERNEL)) {
    mem_cgroup_put(memcg);
// goto;
    }
    mem_cgroup_put(memcg);
    }
    while (index < nr_pages) {
    let mut page = folio_page(folio, index);
    if (!zswap_store_page(page, objcg, pool)) {
// goto;
    }
    }
    if (objcg) {
    count_objcg_events(objcg, ZSWPOUT, nr_pages);
    }
    count_vm_events(ZSWPOUT, nr_pages);
    ret = true;
// label;
    zswap_pool_put(pool);
// label;
    obj_cgroup_put(objcg);
    if (!ret && zswap_pool_reached_full) {
    queue_work(shrink_wq, &zswap_shrink_work);
    }
// label;
//
// If the zswap store fails or zswap is disabled, we must invalidate
// the possibly stale entries which were previously stored at the
// offsets corresponding to each page of the folio. Otherwise,
// writeback could overwrite the new data in the swapfile.
//
    if (!ret) {
pub static mut type: unsigned = 0;
pub static mut offset: pgoff_t = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut tree: *mut c_void = core::ptr::null_mut();
    while (index < nr_pages) {
    tree = swap_zswap_tree(swp_entry(type, offset + index));
    entry = xa_erase(tree, offset + index);
    if (entry) {
    zswap_entry_free(entry);
    }
    }
    }
    return ret;
    }
//
// zswap_load() - load a folio from zswap
// @folio: folio to load
//
// Return: 0 on success, with the folio unlocked and marked up-to-date, or one
// of the following error codes:
//
// -EIO: if the swapped out content was in zswap, but could not be loaded
// into the page due to a decompression failure. The folio is unlocked, but
// NOT marked up-to-date, so that an IO error is emitted (e.g. do_swap_page()
// will SIGBUS).
//
// -EINVAL: if the swapped out content was in zswap, but the page belongs
// to a large folio, which is not supported by zswap. The folio is unlocked,
// but NOT marked up-to-date, so that an IO error is emitted (e.g.
// do_swap_page() will SIGBUS).
//
// -ENOENT: if the swapped out content was not in zswap. The folio remains
// locked on return.
//
#[no_mangle]
pub unsafe extern "C" fn zswap_load(folio: *mut folio) -> c_int {
pub static mut swp: swp_entry_t = 0;
pub static mut offset: pgoff_t = 0;
    let mut tree = swap_zswap_tree(swp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_ONCE(!folio_test_locked(folio));
    VM_WARN_ON_ONCE(!folio_test_swapcache(folio));
    if (zswap_never_enabled()) {
    return -ENOENT;
    }
//
// Large folios should not be swapped in while zswap is being used, as
// they are not properly handled. Zswap does not properly load large
// folios, and a large folio may only be partially in zswap.
//
    if (WARN_ON_ONCE!(folio_test_large(folio))) {
    folio_unlock(folio);
    return -EINVAL;
    }
    entry = xa_load(tree, offset);
    if (!entry) {
    return -ENOENT;
    }
    if (!zswap_decompress(entry, folio)) {
    folio_unlock(folio);
    return -EIO;
    }
    folio_mark_uptodate(folio);
    count_vm_event(ZSWPIN);
    if (entry.objcg) {
    count_objcg_events(entry.objcg, ZSWPIN, 1);
    }
//
// We are reading into the swapcache, invalidate zswap entry.
// The swapcache is the authoritative owner of the page and
// its mappings, and the pressure that results from having two
// in-memory copies outweighs any benefits of caching the
// compression work.
//
    folio_mark_dirty(folio);
    xa_erase(tree, offset);
    zswap_entry_free(entry);
    folio_unlock(folio);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_invalidate(swp: swp_entry_t) {
pub static mut offset: pgoff_t = 0;
    let mut tree = swap_zswap_tree(swp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
    if (xa_empty(tree)) {
    return;
    }
    entry = xa_erase(tree, offset);
    if (entry) {
    zswap_entry_free(entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_swapon(type: c_int, nr_pages: c_ulong) -> c_int {
    let mut trees = core::ptr::null_mut();
    let mut tree = core::ptr::null_mut();
    let mut nr = 0;
    let mut i = 0;
    nr = DIV_ROUND_UP(nr_pages, ZSWAP_ADDRESS_SPACE_PAGES);
    trees = kvzalloc_objs(*tree, nr);
    if (!trees) {
    pr_err!("alloc failed, zswap disabled for swap type %d\n", type);
    return -ENOMEM;
    }
    for (i = 0; i < nr; i++) {
    xa_init(trees + i);
    }
    nr_zswap_trees[type] = nr;
    zswap_trees[type] = trees;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zswap_swapoff(type: c_int) {
    let mut trees = zswap_trees[type];
    let mut i = 0;
    if (!trees) {
    return;
    }
// try_to_unuse() invalidated all the entries already
    for (i = 0; i < nr_zswap_trees[type]; i++) {
    WARN_ON_ONCE!(!xa_empty(trees + i));
    }
    kvfree(trees);
    nr_zswap_trees[type] = 0;
    zswap_trees[type] = core::ptr::null_mut();
    }
//
// debugfs functions
//

pub static mut zswap_debugfs_root: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn debugfs_get_total_size(data: *mut c_void, val: *mut u64) -> c_int {
// val = zswap_total_pages() * PAGE_SIZE;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(total_size_fops, debugfs_get_total_size, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn debugfs_get_stored_pages(data: *mut c_void, val: *mut u64) -> c_int {
// val = atomic_long_read(&zswap_stored_pages);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(stored_pages_fops, debugfs_get_stored_pages, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn debugfs_get_stored_incompressible_pages(data: *mut c_void, val: *mut u64) -> c_int {
// val = atomic_long_read(&zswap_stored_incompressible_pages);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(stored_incompressible_pages_fops,
    debugfs_get_stored_incompressible_pages, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn zswap_debugfs_init() -> c_int {
    if (!debugfs_initialized()) {
    return -ENODEV;
    }
    zswap_debugfs_root = debugfs_create_dir("zswap", core::ptr::null_mut());
    debugfs_create_u64("pool_limit_hit", 0444,
    zswap_debugfs_root, &zswap_pool_limit_hit);
    debugfs_create_u64("reject_reclaim_fail", 0444,
    zswap_debugfs_root, &zswap_reject_reclaim_fail);
    debugfs_create_u64("reject_alloc_fail", 0444,
    zswap_debugfs_root, &zswap_reject_alloc_fail);
    debugfs_create_u64("reject_kmemcache_fail", 0444,
    zswap_debugfs_root, &zswap_reject_kmemcache_fail);
    debugfs_create_u64("reject_compress_fail", 0444,
    zswap_debugfs_root, &zswap_reject_compress_fail);
    debugfs_create_u64("reject_compress_poor", 0444,
    zswap_debugfs_root, &zswap_reject_compress_poor);
    debugfs_create_u64("decompress_fail", 0444,
    zswap_debugfs_root, &zswap_decompress_fail);
    debugfs_create_u64("written_back_pages", 0444,
    zswap_debugfs_root, &zswap_written_back_pages);
    debugfs_create_file("pool_total_size", 0444,
    zswap_debugfs_root, core::ptr::null_mut(), &total_size_fops);
    debugfs_create_file("stored_pages", 0444,
    zswap_debugfs_root, core::ptr::null_mut(), &stored_pages_fops);
    debugfs_create_file("stored_incompressible_pages", 0444,
    zswap_debugfs_root, core::ptr::null_mut(),
    &stored_incompressible_pages_fops);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn zswap_debugfs_init() -> c_int {
    return 0;
    }

//
// module init and exit
//
#[no_mangle]
unsafe extern "C" fn zswap_setup() -> c_int {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    zswap_entry_cache = KMEM_CACHE(zswap_entry, 0);
    if (!zswap_entry_cache) {
    pr_err!("entry cache creation failed\n");
// goto;
    }
    ret = cpuhp_setup_state_multi(CPUHP_MM_ZSWP_POOL_PREPARE,
    "mm/zswap_pool:prepare",
    zswap_cpu_comp_prepare,
    core::ptr::null_mut());
    if (ret) {
// goto;
    }
    shrink_wq = alloc_workqueue("zswap-shrink",
    WQ_UNBOUND|WQ_MEM_RECLAIM, 1);
    if (!shrink_wq) {
// goto;
    }
    zswap_shrinker = zswap_alloc_shrinker();
    if (!zswap_shrinker) {
// goto;
    }
    if (list_lru_init_memcg(&zswap_list_lru, zswap_shrinker)) {
// goto;
    }
    shrinker_register(zswap_shrinker);
    INIT_WORK(&zswap_shrink_work, shrink_worker);
    pool = __zswap_pool_create_fallback();
    if (pool) {
    pr_info!("loaded using pool %s\n", pool.tfm_name);
    list_add(&pool.list, &zswap_pools);
    zswap_has_pool = true;
    static_branch_enable(&zswap_ever_enabled);
    } else {
    pr_err!("pool creation failed\n");
    zswap_enabled = false;
    }
    if (zswap_debugfs_init()) {
    pr_warn!("debugfs initialization failed\n");
    }
    zswap_init_state = ZSWAP_INIT_SUCCEED;
    return 0;
// label;
    shrinker_free(zswap_shrinker);
// label;
    destroy_workqueue(shrink_wq);
// label;
    cpuhp_remove_multi_state(CPUHP_MM_ZSWP_POOL_PREPARE);
// label;
    kmem_cache_destroy(zswap_entry_cache);
// label;
// if built-in, we aren't unloaded on failure; don't allow use
    zswap_init_state = ZSWAP_INIT_FAILED;
    zswap_enabled = false;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn zswap_init() -> c_int {
    if (!zswap_enabled) {
    return 0;
    }
    return zswap_setup();
    }
// must be late so crypto has time to come up
    late_initcall!(zswap_init);
    MODULE_AUTHOR("Seth Jennings <sjennings@variantweb.net>");
    MODULE_DESCRIPTION("Compressed cache for swap pages");