//! Automatically rewritten from C to Rust
//! Source: mm/mempool.c
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
// memory buffer pool support. Such pools are mostly used
// for guaranteed, deadlock-free memory allocations during
// extreme VM load.
//
// started by Ingo Molnar, Copyright (C) 2001
// debugging by David Rientjes, Copyright (C) 2015
//

pub static mut fail_mempool_alloc: usize = 0;
pub static mut fail_mempool_alloc_bulk: usize = 0;
//
// Debugging support for mempool using static key.
//
// This allows enabling mempool debug at boot time via:
// mempool_debug
//
pub static mut mempool_debug_enabled: usize = 0;
#[no_mangle]
unsafe extern "C" fn mempool_debug_setup(str: *mut c_char) -> c_int {
    static_branch_enable(&mempool_debug_enabled);
    return 1;
    }
    __setup!("mempool_debug", mempool_debug_setup);
#[no_mangle]
unsafe extern "C" fn mempool_faul_inject_init() -> c_int {
    let mut error = 0;
    error = PTR_ERR_OR_ZERO(fault_create_debugfs_attr("fail_mempool_alloc",
    core::ptr::null_mut(), &fail_mempool_alloc));
    if (error) {
    return error;
    }
// booting will fail on error return here, don't bother to cleanup
    return PTR_ERR_OR_ZERO(
    fault_create_debugfs_attr("fail_mempool_alloc_bulk", core::ptr::null_mut(),
    &fail_mempool_alloc_bulk));
    }
    late_initcall!(mempool_faul_inject_init);
#[no_mangle]
pub unsafe extern "C" fn poison_error(pool: *mut mempool, element: *mut c_void, size: size_t, byte: size_t) {
pub static mut nr: c_int = 0;
pub static mut start: c_int = 0;
pub static mut end: c_int = 0;
    let mut i = 0;
    pr_err!("BUG: mempool element poison mismatch\n");
    pr_err!("Mempool %p size %zu\n", pool, size);
    pr_err!(" nr=%d @ %p: %s0x", nr, element, start > 0 ? "... " : "");
    for (i = start; i < end; i++) {
    pr_cont("%x ", *(element + i));
    }
    pr_cont("%s\n", end < size ? "..." : "");
    dump_stack();
    }
#[no_mangle]
unsafe extern "C" fn __check_element(pool: *mut mempool, element: *mut c_void, size: usize) {
    let mut obj = element;
    let mut i = 0;
    while (i < size) {
pub static mut exp: u8 = 0;
    if (obj[i] != exp) {
    poison_error(pool, element, size, i);
    return;
    }
    }
    memset(obj, POISON_INUSE, size);
    }
#[no_mangle]
unsafe extern "C" fn check_element(pool: *mut mempool, element: *mut c_void) {
// Skip checking: KASAN might save its metadata in the element.
    if (kasan_enabled()) {
    return;
    }
// Mempools backed by slab allocator
    if (pool.free == mempool_kfree) {
    __check_element(pool, element, (size_t)pool.pool_data);
    } else if (pool.free == mempool_free_slab) {
    __check_element(pool, element, kmem_cache_size(pool.pool_data));
    } else if (pool.free == mempool_free_pages) {
// Mempools backed by page allocator
pub static mut order: c_int = 0;

    while (i < (1 << order)) {
    let mut page = element;
    let mut addr = kmap_local_page(page + i);
    __check_element(pool, addr, PAGE_SIZE);
    kunmap_local(addr);
    }

    let mut addr = page_address(element);
    __check_element(pool, addr, PAGE_SIZE << order);

    }
    }
#[no_mangle]
unsafe extern "C" fn __poison_element(element: *mut c_void, size: usize) {
    let mut obj = element;
    memset(obj, POISON_FREE, size - 1);
    obj[size - 1] = POISON_END;
    }
#[no_mangle]
unsafe extern "C" fn poison_element(pool: *mut mempool, element: *mut c_void) {
// Skip poisoning: KASAN might save its metadata in the element.
    if (kasan_enabled()) {
    return;
    }
// Mempools backed by slab allocator
    if (pool.alloc == mempool_kmalloc) {
    __poison_element(element, (size_t)pool.pool_data);
    } else if (pool.alloc == mempool_alloc_slab) {
    __poison_element(element, kmem_cache_size(pool.pool_data));
    } else if (pool.alloc == mempool_alloc_pages) {
// Mempools backed by page allocator
pub static mut order: c_int = 0;

    while (i < (1 << order)) {
    let mut page = element;
    let mut addr = kmap_local_page(page + i);
    __poison_element(addr, PAGE_SIZE);
    kunmap_local(addr);
    }

    let mut addr = page_address(element);
    __poison_element(addr, PAGE_SIZE << order);

    }
    }
    static __always_inline bool kasan_poison_element(mempool *pool,
    void *element)
    {
    if (pool.alloc == mempool_alloc_slab || pool.alloc == mempool_kmalloc) {
    return kasan_mempool_poison_object(element);
    }

    else if (pool.alloc == mempool_alloc_pages) {
    return kasan_mempool_poison_pages(element,
    (unsigned long)pool.pool_data);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn kasan_unpoison_element(pool: *mut mempool, element: *mut c_void) {
    if (pool.alloc == mempool_kmalloc) {
    kasan_mempool_unpoison_object(element, (size_t)pool.pool_data);
    }

    else if (pool.alloc == mempool_alloc_slab) {
    kasan_mempool_unpoison_object(element,
    kmem_cache_size(pool.pool_data));
    }

    else if (pool.alloc == mempool_alloc_pages) {
    kasan_mempool_unpoison_pages(element,
    (unsigned long)pool.pool_data);
    }
    }
#[no_mangle]
unsafe extern "C" fn add_element(pool: *mut mempool, element: *mut c_void) -> __always_inline void {
    BUG_ON!(pool.min_nr != 0 && pool.curr_nr >= pool.min_nr);
    if (static_branch_unlikely(&mempool_debug_enabled)) {
    poison_element(pool, element);
    }
    if (kasan_poison_element(pool, element)) {
    pool.elements[pool.curr_nr++] = element;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn remove_element(pool: *mut mempool) -> *mut c_void {
    let mut element = pool.elements[--pool.curr_nr];
    BUG_ON!(pool.curr_nr < 0);
    kasan_unpoison_element(pool, element);
    if (static_branch_unlikely(&mempool_debug_enabled)) {
    check_element(pool, element);
    }
    return element;
    }
//
// mempool_exit - exit a mempool initialized with mempool_init()
// @pool:      pointer to the memory pool which was initialized with
// mempool_init().
//
// Free all reserved elements in @pool and @pool itself.  This function
// only sleeps if the free_fn() function sleeps.
//
// May be called on a zeroed but uninitialized mempool (i.e. allocated with
// kzalloc()).
//
#[no_mangle]
pub unsafe extern "C" fn mempool_exit(pool: *mut mempool) {
    while (pool.curr_nr) {
    let mut element = remove_element(pool);
    pool.free(element, pool.pool_data);
    }
    kfree(pool.elements);
    pool.elements = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(mempool_exit);
//
// mempool_destroy - deallocate a memory pool
// @pool:      pointer to the memory pool which was allocated via
// mempool_create().
//
// Free all reserved elements in @pool and @pool itself.  This function
// only sleeps if the free_fn() function sleeps.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_destroy(pool: *mut mempool) {
    if (unlikely(!pool)) {
    return;
    }
    mempool_exit(pool);
    kfree(pool);
    }
    EXPORT_SYMBOL(mempool_destroy);
#[no_mangle]
pub unsafe extern "C" fn mempool_init_node(pool: *mut mempool, min_nr: c_int, alloc_fn: *mut mempool_alloc_t, free_fn: *mut mempool_free_t, pool_data: *mut c_void, gfp_mask: gfp_t, node_id: c_int) -> c_int {
    spin_lock_init(&pool.lock);
    pool.min_nr	= min_nr;
    pool.pool_data = pool_data;
    pool.alloc	= alloc_fn;
    pool.free	= free_fn;
    init_waitqueue_head(&pool.wait);
//
// max() used here to ensure storage for at least 1 element to support
// zero minimum pool
//
    pool.elements = kmalloc_array_node(max(1, min_nr), sizeof!,
    gfp_mask, node_id);
    if (!pool.elements) {
    return -ENOMEM;
    }
//
// First pre-allocate the guaranteed number of buffers,
// also pre-allocate 1 element for zero minimum pool.
//
    while (pool.curr_nr < max(1, pool.min_nr)) {
pub static mut element: *mut c_void = core::ptr::null_mut();
    element = pool.alloc(gfp_mask, pool.pool_data);
    if (unlikely(!element)) {
    mempool_exit(pool);
    return -ENOMEM;
    }
    add_element(pool, element);
    }
    return 0;
    }
    EXPORT_SYMBOL(mempool_init_node);
//
// mempool_init - initialize a memory pool
// @pool:      pointer to the memory pool that should be initialized
// @min_nr:    the minimum number of elements guaranteed to be
// allocated for this pool.
// @alloc_fn:  user-defined element-allocation function.
// @free_fn:   user-defined element-freeing function.
// @pool_data: optional private data available to the user-defined functions.
//
// Like mempool_create(), but initializes the pool in (i.e. embedded in another
// structure).
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_init_noprof(pool: *mut mempool, min_nr: c_int, alloc_fn: *mut mempool_alloc_t, free_fn: *mut mempool_free_t, pool_data: *mut c_void) -> c_int {
    return mempool_init_node(pool, min_nr, alloc_fn, free_fn,
    pool_data, GFP_KERNEL, NUMA_NO_NODE);
    }
    EXPORT_SYMBOL(mempool_init_noprof);
//
// mempool_create_node - create a memory pool
// @min_nr:    the minimum number of elements guaranteed to be
// allocated for this pool.
// @alloc_fn:  user-defined element-allocation function.
// @free_fn:   user-defined element-freeing function.
// @pool_data: optional private data available to the user-defined functions.
// @gfp_mask:  memory allocation flags
// @node_id:   numa node to allocate on
//
// this function creates and allocates a guaranteed size, preallocated
// memory pool. The pool can be used from the mempool_alloc() and mempool_free()
// functions. This function might sleep. Both the alloc_fn() and the free_fn()
// functions might sleep - as long as the mempool_alloc() function is not called
// from IRQ contexts.
//
// Return: pointer to the created memory pool object or %NULL on error.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_create_node_noprof(min_nr: c_int, alloc_fn: *mut mempool_alloc_t, free_fn: *mut mempool_free_t, pool_data: *mut c_void, gfp_mask: gfp_t, node_id: c_int) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    pool = kmalloc_node_noprof(sizeof!(*pool), gfp_mask | __GFP_ZERO, node_id);
    if (!pool) {
    return core::ptr::null_mut();
    }
    if (mempool_init_node(pool, min_nr, alloc_fn, free_fn, pool_data,
    gfp_mask, node_id)) {
    kfree(pool);
    return core::ptr::null_mut();
    }
    return pool;
    }
    EXPORT_SYMBOL(mempool_create_node_noprof);
//
// mempool_resize - resize an existing memory pool
// @pool:       pointer to the memory pool which was allocated via
// mempool_create().
// @new_min_nr: the new minimum number of elements guaranteed to be
// allocated for this pool.
//
// This function shrinks/grows the pool. In the case of growing,
// it cannot be guaranteed that the pool will be grown to the new
// size immediately, but new mempool_free() calls will refill it.
// This function may sleep.
//
// Note, the caller must guarantee that no mempool_destroy is called
// while this function is running. mempool_alloc() & mempool_free()
// might be called (eg. from IRQ contexts) while this function executes.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_resize(pool: *mut mempool, new_min_nr: c_int) -> c_int {
pub static mut element: *mut c_void = core::ptr::null_mut();
pub static mut new_elements: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    BUG_ON!(new_min_nr <= 0);
    might_sleep();
    spin_lock_irqsave(&pool.lock, flags);
    if (new_min_nr <= pool.min_nr) {
    while (new_min_nr < pool.curr_nr) {
    element = remove_element(pool);
    spin_unlock_irqrestore(&pool.lock, flags);
    pool.free(element, pool.pool_data);
    spin_lock_irqsave(&pool.lock, flags);
    }
    pool.min_nr = new_min_nr;
// goto;
    }
    spin_unlock_irqrestore(&pool.lock, flags);
// Grow the pool
    new_elements = kmalloc_objs(*new_elements, new_min_nr);
    if (!new_elements) {
    return -ENOMEM;
    }
    spin_lock_irqsave(&pool.lock, flags);
    if (unlikely(new_min_nr <= pool.min_nr)) {
// Raced, other resize will do our work
    spin_unlock_irqrestore(&pool.lock, flags);
    kfree(new_elements);
// goto;
    }
    memcpy(new_elements, pool.elements,
    pool.curr_nr * sizeof!(*new_elements));
    kfree(pool.elements);
    pool.elements = new_elements;
    pool.min_nr = new_min_nr;
    while (pool.curr_nr < pool.min_nr) {
    spin_unlock_irqrestore(&pool.lock, flags);
    element = pool.alloc(GFP_KERNEL, pool.pool_data);
    if (!element) {
// goto;
    }
    spin_lock_irqsave(&pool.lock, flags);
    if (pool.curr_nr < pool.min_nr) {
    add_element(pool, element);
    } else {
    spin_unlock_irqrestore(&pool.lock, flags);
    pool.free(element, pool.pool_data);	/* Raced */
// goto;
    }
    }
// label;
    spin_unlock_irqrestore(&pool.lock, flags);
// label;
    return 0;
    }
    EXPORT_SYMBOL(mempool_resize);
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_from_pool(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint, allocated: c_uint, gfp_mask: gfp_t) -> c_uint {
    let mut flags = 0;
    let mut i = 0;
    spin_lock_irqsave(&pool.lock, flags);
    if (unlikely(pool.curr_nr < count - allocated)) {
// goto;
    }
    while (allocated < count) {
    elems[allocated++] = remove_element(pool);
    }
    spin_unlock_irqrestore(&pool.lock, flags);
// Paired with rmb in mempool_free(), read comment there.
    smp_wmb();
//
// Update the allocation stack trace as this is more useful for
// debugging.
//
    for (i = 0; i < count; i++) {
    kmemleak_update_trace(elems[i]);
    }
    return allocated;
// label;
    if (gfp_mask & __GFP_DIRECT_RECLAIM) {
pub static mut wait: usize = 0;
    prepare_to_wait(&pool.wait, &wait, TASK_UNINTERRUPTIBLE);
    spin_unlock_irqrestore(&pool.lock, flags);
//
// Wait for someone else to return an element to @pool, but wake
// up occasionally as memory pressure might have reduced even
// and the normal allocation in alloc_fn could succeed even if
// no element was returned.
//
    io_schedule_timeout(5 * HZ);
    finish_wait(&pool.wait, &wait);
    } else {
// We must not sleep if __GFP_DIRECT_RECLAIM is not set.
    spin_unlock_irqrestore(&pool.lock, flags);
    }
    return allocated;
    }
//
// Adjust the gfp flags for mempool allocations, as we never want to dip into
// the global emergency reserves or retry in the page allocator.
//
// The first pass also doesn't want to go reclaim, but the next passes do, so
// return a separate subset for that first iteration.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_adjust_gfp(gfp_mask: *mut gfp_t) -> gfp_t {
// gfp_mask |= __GFP_NOMEMALLOC | __GFP_NORETRY | __GFP_NOWARN;
    return *gfp_mask & ~(__GFP_DIRECT_RECLAIM | __GFP_IO);
    }
//
// mempool_alloc_bulk - allocate multiple elements from a memory pool
// @pool:	pointer to the memory pool
// @elems:	partially or fully populated elements array
// @count:	number of entries in @elem that need to be allocated
//
// Allocate @count elements into @elems.  This is done by first calling into the
// alloc_fn supplied at pool initialization time, and dipping into the reserved
// pool when alloc_fn fails to allocate an element.
//
// On return all @count elements in @elems will be populated.
//
// Return: Always 0.  If it wasn't for %$#^$ alloc tags, it would return void.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_bulk_noprof(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint) -> c_int {
pub static mut gfp_mask: gfp_t = 0;
pub static mut gfp_temp: gfp_t = 0;
pub static mut allocated: c_uint = 0;
    VM_WARN_ON_ONCE(count > pool.min_nr);
    might_alloc(gfp_mask);
//
// If an error is injected, fail all elements in a bulk allocation so
// that we stress the multiple elements missing path.
//
    if (should_fail_ex(&fail_mempool_alloc_bulk, 1, FAULT_NOWARN)) {
    pr_info!("forcing mempool usage for %pS\n",
    _RET_IP_);
// goto;
    }
// label;
//
// Try to allocate the elements using the allocation callback first as
// that might succeed even when the caller's bulk allocation did not.
//
    while (allocated < count) {
    elems[allocated] = pool.alloc(gfp_temp, pool.pool_data);
    if (unlikely(!elems[allocated])) {
// goto;
    }
    allocated += 1;
    }
    return 0;
// label;
    allocated = mempool_alloc_from_pool(pool, elems, count, allocated,
    gfp_temp);
    gfp_temp = gfp_mask;
// goto;
    }
    EXPORT_SYMBOL_GPL(mempool_alloc_bulk_noprof);
//
// mempool_alloc - allocate an element from a memory pool
// @pool:	pointer to the memory pool
// @gfp_mask:	GFP_* flags.  %__GFP_ZERO is not supported.
//
// Allocate an element from @pool.  This is done by first calling into the
// alloc_fn supplied at pool initialization time, and dipping into the reserved
// pool when alloc_fn fails to allocate an element.
//
// This function only sleeps if the alloc_fn callback sleeps, or when waiting
// for elements to become available in the pool.
//
// Return: pointer to the allocated element or %NULL when failing to allocate
// an element.  Allocation failure can only happen when @gfp_mask does not
// include %__GFP_DIRECT_RECLAIM.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_noprof(pool: *mut mempool, gfp_mask: gfp_t) -> *mut c_void {
pub static mut gfp_temp: gfp_t = 0;
pub static mut element: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_ONCE(gfp_mask & __GFP_ZERO);
    might_alloc(gfp_mask);
// label;
    if (should_fail_ex(&fail_mempool_alloc, 1, FAULT_NOWARN)) {
    pr_info!("forcing mempool usage for %pS\n",
    _RET_IP_);
    element = core::ptr::null_mut();
    } else {
    element = pool.alloc(gfp_temp, pool.pool_data);
    }
    if (unlikely(!element)) {
//
// Try to allocate an element from the pool.
//
// The first pass won't have __GFP_DIRECT_RECLAIM and won't
// sleep in mempool_alloc_from_pool.  Retry the allocation
// with all flags set in that case.
//
    if (!mempool_alloc_from_pool(pool, &element, 1, 0, gfp_temp)) {
    if (gfp_temp != gfp_mask) {
    gfp_temp = gfp_mask;
// goto;
    }
    if (gfp_mask & __GFP_DIRECT_RECLAIM) {
// goto;
    }
    }
    }
    return element;
    }
    EXPORT_SYMBOL(mempool_alloc_noprof);
//
// mempool_alloc_preallocated - allocate an element from preallocated elements
// belonging to a memory pool
// @pool:	pointer to the memory pool
//
// This function is similar to mempool_alloc(), but it only attempts allocating
// an element from the preallocated elements. It only takes a single spinlock_t
// and immediately returns if no preallocated elements are available.
//
// Return: pointer to the allocated element or %NULL if no elements are
// available.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_preallocated(pool: *mut mempool) -> *mut c_void {
    let mut element = core::ptr::null_mut();
    mempool_alloc_from_pool(pool, &element, 1, 0, GFP_NOWAIT);
    return element;
    }
    EXPORT_SYMBOL(mempool_alloc_preallocated);
//
// mempool_free_bulk - return elements to a mempool
// @pool:	pointer to the memory pool
// @elems:	elements to return
// @count:	number of elements to return
//
// Returns a number of elements from the start of @elem to @pool if @pool needs
// replenishing and sets their slots in @elem to NULL.  Other elements are left
// in @elem.
//
// Return: number of elements transferred to @pool.  Elements are always
// transferred from the beginning of @elem, so the return value can be used as
// an offset into @elem for the freeing the remaining elements in the caller.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_free_bulk(pool: *mut mempool, elems: *mut *mut c_void, count: c_uint) -> c_uint {
    let mut flags = 0;
pub static mut freed: c_uint = 0;
pub static mut added: bool = false;
//
// Paired with the wmb in mempool_alloc().  The preceding read is
// for @element and the following @pool->curr_nr.  This ensures
// that the visible value of @pool->curr_nr is from after the
// allocation of @element.  This is necessary for fringe cases
// where @element was passed to this task without going through
// barriers.
//
// For example, assume @p is %NULL at the beginning and one task
// performs "p = mempool_alloc(...);" while another task is doing
// "while (!p) cpu_relax(); mempool_free(p, ...);".  This function
// may end up using curr_nr value which is from before allocation
// of @p without the following rmb.
//
    smp_rmb();
//
// For correctness, we need a test which is guaranteed to trigger
// if curr_nr + #allocated == min_nr.  Testing curr_nr < min_nr
// without locking achieves that and refilling as soon as possible
// is desirable.
//
// Because curr_nr visible here is always a value after the
// allocation of @element, any task which decremented curr_nr below
// min_nr is guaranteed to see curr_nr < min_nr unless curr_nr gets
// incremented to min_nr afterwards.  If curr_nr gets incremented
// to min_nr after the allocation of @element, the elements
// allocated after that are subject to the same guarantee.
//
// Waiters happen iff curr_nr is 0 and the above guarantee also
// ensures that there will be frees which return elements to the
// pool waking up the waiters.
//
// For zero-minimum pools, curr_nr < min_nr (0 < 0) never succeeds,
// so waiters sleeping on pool->wait would never be woken by the
// wake-up path of previous test. This explicit check ensures the
// allocation of element when both min_nr and curr_nr are 0, and
// any active waiters are properly awakened.
//
    if (unlikely(READ_ONCE(pool.curr_nr) < pool.min_nr)) {
    spin_lock_irqsave(&pool.lock, flags);
    while (pool.curr_nr < pool.min_nr && freed < count) {
    add_element(pool, elems[freed++]);
    added = true;
    }
    spin_unlock_irqrestore(&pool.lock, flags);
    } else if (unlikely(pool.min_nr == 0 &&
    READ_ONCE(pool.curr_nr) == 0)) {
// Handle the min_nr = 0 edge case:
    spin_lock_irqsave(&pool.lock, flags);
    if (likely(pool.curr_nr == 0)) {
    add_element(pool, elems[freed++]);
    added = true;
    }
    spin_unlock_irqrestore(&pool.lock, flags);
    }
    if (unlikely(added) && wq_has_sleeper(&pool.wait)) {
    wake_up(&pool.wait);
    }
    return freed;
    }
    EXPORT_SYMBOL_GPL(mempool_free_bulk);
//
// mempool_free - return an element to the pool.
// @element:	element to return
// @pool:	pointer to the memory pool
//
// Returns @element to @pool if it needs replenishing, else frees it using
// the free_fn callback in @pool.
//
// This function only sleeps if the free_fn callback sleeps.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_free(element: *mut c_void, pool: *mut mempool) {
    if (likely(element) && !mempool_free_bulk(pool, &element, 1)) {
    pool.free(element, pool.pool_data);
    }
    }
    EXPORT_SYMBOL(mempool_free);
//
// A commonly used alloc and free fn.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_slab(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void {
    let mut mem = pool_data;
    VM_BUG_ON(mem.ctor);
    return kmem_cache_alloc_noprof(mem, gfp_mask);
    }
    EXPORT_SYMBOL(mempool_alloc_slab);
#[no_mangle]
pub unsafe extern "C" fn mempool_free_slab(element: *mut c_void, pool_data: *mut c_void) {
    let mut mem = pool_data;
    kmem_cache_free(mem, element);
    }
    EXPORT_SYMBOL(mempool_free_slab);
//
// A commonly used alloc and free fn that kmalloc/kfrees the amount of memory
// specified by pool_data
//
#[no_mangle]
pub unsafe extern "C" fn mempool_kmalloc(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void {
pub static mut size: usize = 0;
    return kmalloc_noprof(size, gfp_mask);
    }
    EXPORT_SYMBOL(mempool_kmalloc);
#[no_mangle]
pub unsafe extern "C" fn mempool_kfree(element: *mut c_void, pool_data: *mut c_void) {
    kfree(element);
    }
    EXPORT_SYMBOL(mempool_kfree);
//
// A simple mempool-backed page allocator that allocates pages
// of the order specified by pool_data.
//
#[no_mangle]
pub unsafe extern "C" fn mempool_alloc_pages(gfp_mask: gfp_t, pool_data: *mut c_void) -> *mut c_void {
pub static mut order: c_int = 0;
    return alloc_pages_noprof(gfp_mask, order);
    }
    EXPORT_SYMBOL(mempool_alloc_pages);
#[no_mangle]
pub unsafe extern "C" fn mempool_free_pages(element: *mut c_void, pool_data: *mut c_void) {
pub static mut order: c_int = 0;
    __free_pages(element, order);
    }
    EXPORT_SYMBOL(mempool_free_pages);