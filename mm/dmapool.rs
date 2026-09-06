//! Automatically rewritten from C to Rust
//! Source: mm/dmapool.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// DMA Pool allocator
//
// Copyright 2001 David Brownell
// Copyright 2007 Intel Corporation
// Author: Matthew Wilcox <willy@linux.intel.com>
//
// This allocator returns small blocks of a given size which are DMA-able by
// the given device.  It uses the dma_alloc_coherent page allocator to get
// new pages, then splits them up into blocks of the required size.
// Many older drivers still have their own code to do this.
//
// The current design of this allocator is fairly simple.  The pool is
// represented by the 'struct dma_pool' which keeps a doubly-linked list of
// allocated pages.  Each page in the page_list is split into blocks of at
// least 'size' bytes.  Free blocks are tracked in an unsorted singly-linked
// list of free blocks across all pages.  Used blocks aren't tracked, but we
// keep a count of how many are currently allocated from each page.
//

pub const DMAPOOL_DEBUG: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_block {
    pub next_block: *mut dma_block,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pool {
    pub page_list: list_head,
    pub lock: spinlock_t,
    pub next_block: *mut dma_block,
    pub nr_blocks: usize,
    pub nr_active: usize,
    pub nr_pages: usize,
    pub dev: *mut device,
    pub size: c_uint,
    pub allocation: c_uint,
    pub boundary: c_uint,
    pub node: c_int,
    pub name: [c_char; 32],
    pub pools: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_page {
    pub page_list: list_head,
    pub vaddr: *mut c_void,
    pub dma: dma_addr_t,
}

pub static mut pools_lock: usize = 0;
pub static mut pools_reg_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn pools_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut size: c_uint = 0;
    size = sysfs_emit(buf, "poolinfo - 0.1\n");
    mutex_lock(&pools_lock);
    list_for_each_entry(pool, &dev.dma_pools, pools) {
// per-pool info, no real statistics yet
    size += sysfs_emit_at(buf, size, "%-16s %4zu %4zu %4u %2zu\n",
    pool.name, pool.nr_active,
    pool.nr_blocks, pool.size,
    pool.nr_pages);
    }
    mutex_unlock(&pools_lock);
    return size;
    }
    static DEVICE_ATTR_RO(pools);

#[no_mangle]
pub unsafe extern "C" fn pool_check_block(pool: *mut dma_pool, block: *mut dma_block, mem_flags: gfp_t) {
    let mut data = block;
    let mut i = 0;
    while (i < pool.size) {
    if (data[i] == POOL_POISON_FREED) {
    continue;
    }
    dev_err(pool.dev, "%s %s, %p (corrupted)\n", __func__,
    pool.name, block);
//
// Dump the first 4 bytes even if they are not
// POOL_POISON_FREED
//
    print_hex_dump(KERN_ERR, "", DUMP_PREFIX_OFFSET, 16, 1,
    data, pool.size, 1);
    break;
    }
    if (!want_init_on_alloc(mem_flags)) {
    memset(block, POOL_POISON_ALLOCATED, pool.size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pool_find_page(pool: *mut dma_pool, dma: dma_addr_t) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(page, &pool.page_list, page_list) {
    if (dma < page.dma) {
    continue;
    }
    if ((dma - page.dma) < pool.allocation) {
    return page;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pool_block_err(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) -> bool {
    let mut block = pool.next_block;
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pool_find_page(pool, dma);
    if (!page) {
    dev_err(pool.dev, "%s %s, %p/%pad (bad dma)\n",
    __func__, pool.name, vaddr, &dma);
    return true;
    }
    while (block) {
    if (block != vaddr) {
    block = block.next_block;
    continue;
    }
    dev_err(pool.dev, "%s %s, dma %pad already free\n",
    __func__, pool.name, &dma);
    return true;
    }
    memset(vaddr, POOL_POISON_FREED, pool.size);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pool_init_page(pool: *mut dma_pool, page: *mut dma_page) {
    memset(page.vaddr, POOL_POISON_FREED, pool.allocation);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: pool_check_block
pub unsafe extern "C" fn pool_check_block_dup(pool: *mut dma_pool, block: *mut dma_block, mem_flags: gfp_t) {
    }
#[no_mangle]
unsafe extern "C" fn pool_block_err(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) -> bool {
    if (want_init_on_free()) {
    memset(vaddr, 0, pool.size);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pool_init_page(pool: *mut dma_pool, page: *mut dma_page) {
    }

#[no_mangle]
pub unsafe extern "C" fn pool_block_pop(pool: *mut dma_pool) -> *mut c_void {
    let mut block = pool.next_block;
    if (block) {
    pool.next_block = block.next_block;
    pool.nr_active += 1;
    }
    return block;
    }
#[no_mangle]
pub unsafe extern "C" fn pool_block_push(pool: *mut dma_pool, block: *mut dma_block, dma: dma_addr_t) {
    block.dma = dma;
    block.next_block = pool.next_block;
    pool.next_block = block;
    }
//
// dma_pool_create_node - Creates a pool of coherent DMA memory blocks.
// @name: name of pool, for diagnostics
// @dev: device that will be doing the DMA
// @size: size of the blocks in this pool.
// @align: alignment requirement for blocks; must be a power of two
// @boundary: returned blocks won't cross this power of two boundary
// @node: optional NUMA node to allocate structs 'dma_pool' and 'dma_page' on
// Context: not in_interrupt()
//
// Given one of these pools, dma_pool_alloc()
// may be used to allocate memory.  Such memory will all have coherent
// DMA mappings, accessible by the device and its driver without using
// cache flushing primitives.  The actual size of blocks allocated may be
// larger than requested because of alignment.
//
// If @boundary is nonzero, objects returned from dma_pool_alloc() won't
// cross that size boundary.  This is useful for devices which have
// addressing restrictions on individual DMA transfers, such as not crossing
// boundaries of 4KBytes.
//
// Return: a dma allocation pool with the requested characteristics, or
// %NULL if one can't be created.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_create_node(name: *mut c_char, dev: *mut device, size: size_t, align: size_t, boundary: size_t, node: c_int) -> *mut c_void {
pub static mut retval: *mut c_void = core::ptr::null_mut();
    let mut allocation = 0;
    let mut empty = 0;
    if (!dev) {
    return core::ptr::null_mut();
    }
    if (align == 0) {
    align = 1;
    }

    else if (align & (align - 1)) {
    return core::ptr::null_mut();
    }
    if (size == 0 || size > INT_MAX) {
    return core::ptr::null_mut();
    }
    if (size < sizeof!(dma_block)) {
    size = sizeof!(dma_block);
    }
    size = ALIGN(size, align);
    allocation = max_t(size_t, size, PAGE_SIZE);
    if (!boundary) {
    boundary = allocation;
    }

    else if ((boundary < size) || (boundary & (boundary - 1))) {
    return core::ptr::null_mut();
    }
    boundary = min(boundary, allocation);
    retval = kzalloc_node(sizeof!(*retval), GFP_KERNEL, node);
    if (!retval) {
    return retval;
    }
    strscpy(retval.name, name, sizeof!(retval.name));
    retval.dev = dev;
    INIT_LIST_HEAD(&retval.page_list);
    spin_lock_init(&retval.lock);
    retval.size = size;
    retval.boundary = boundary;
    retval.allocation = allocation;
    retval.node = node;
    INIT_LIST_HEAD(&retval.pools);
//
// pools_lock ensures that the ->dma_pools list does not get corrupted.
// pools_reg_lock ensures that there is not a race between
// dma_pool_create() and dma_pool_destroy() or within dma_pool_create()
// when the first invocation of dma_pool_create() failed on
// device_create_file() and the second assumes that it has been done (I
// know it is a short window).
//
    mutex_lock(&pools_reg_lock);
    mutex_lock(&pools_lock);
    empty = list_empty(&dev.dma_pools);
    list_add(&retval.pools, &dev.dma_pools);
    mutex_unlock(&pools_lock);
    if (empty) {
    let mut err = 0;
    err = device_create_file(dev, &dev_attr_pools);
    if (err) {
    mutex_lock(&pools_lock);
    list_del(&retval.pools);
    mutex_unlock(&pools_lock);
    mutex_unlock(&pools_reg_lock);
    kfree(retval);
    return core::ptr::null_mut();
    }
    }
    mutex_unlock(&pools_reg_lock);
    return retval;
    }
    EXPORT_SYMBOL(dma_pool_create_node);
#[no_mangle]
unsafe extern "C" fn pool_initialise_page(pool: *mut dma_pool, page: *mut dma_page) {
pub static mut next_boundary: c_uint = 0;
    struct dma_block *block, *first = core::ptr::null_mut(), *last = core::ptr::null_mut();
    pool_init_page(pool, page);
    while (offset + pool.size <= pool.allocation) {
    if (offset + pool.size > next_boundary) {
    offset = next_boundary;
    next_boundary += pool.boundary;
    continue;
    }
    block = page.vaddr + offset;
    block.dma = page.dma + offset;
    block.next_block = core::ptr::null_mut();
    if (last) {
    last.next_block = block;
    }
    else {
    first = block;
    }
    last = block;
    offset += pool.size;
    pool.nr_blocks += 1;
    }
    last.next_block = pool.next_block;
    pool.next_block = first;
    list_add(&page.page_list, &pool.page_list);
    pool.nr_pages += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pool_alloc_page(pool: *mut dma_pool, mem_flags: gfp_t) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = kmalloc_node(sizeof!(*page), mem_flags, pool.node);
    if (!page) {
    return core::ptr::null_mut();
    }
    page.vaddr = dma_alloc_coherent(pool.dev, pool.allocation,
    &page.dma, mem_flags);
    if (!page.vaddr) {
    kfree(page);
    return core::ptr::null_mut();
    }
    return page;
    }
//
// dma_pool_destroy - destroys a pool of dma memory blocks.
// @pool: dma pool that will be destroyed
// Context: !in_interrupt()
//
// Caller guarantees that no more memory from the pool is in use,
// and that nothing will try to use the pool after this call.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_destroy(pool: *mut dma_pool) {
    let mut page = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    bool empty, busy = false;
    if (unlikely(!pool)) {
    return;
    }
    mutex_lock(&pools_reg_lock);
    mutex_lock(&pools_lock);
    list_del(&pool.pools);
    empty = list_empty(&pool.dev.dma_pools);
    mutex_unlock(&pools_lock);
    if (empty) {
    device_remove_file(pool.dev, &dev_attr_pools);
    }
    mutex_unlock(&pools_reg_lock);
    if (pool.nr_active) {
    dev_err(pool.dev, "%s %s busy\n", __func__, pool.name);
    busy = true;
    }
    list_for_each_entry_safe(page, tmp, &pool.page_list, page_list) {
    if (!busy) {
    dma_free_coherent(pool.dev, pool.allocation,
    page.vaddr, page.dma);
    }
    list_del(&page.page_list);
    kfree(page);
    }
    kfree(pool);
    }
    EXPORT_SYMBOL(dma_pool_destroy);
//
// dma_pool_alloc - get a block of coherent memory
// @pool: dma pool that will produce the block
// @mem_flags: GFP_* bitmask
// @handle: pointer to dma address of block
//
// Return: the kernel virtual address of a currently unused block,
// and reports its dma address through the handle.
// If such a memory block can't be allocated, %NULL is returned.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_alloc(pool: *mut dma_pool, mem_flags: gfp_t, handle: *mut dma_addr_t) -> *mut c_void {
pub static mut block: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    might_alloc(mem_flags);
    spin_lock_irqsave(&pool.lock, flags);
    block = pool_block_pop(pool);
    if (!block) {
//
// pool_alloc_page() might sleep, so temporarily drop
// &pool->lock
//
    spin_unlock_irqrestore(&pool.lock, flags);
    page = pool_alloc_page(pool, mem_flags & (~__GFP_ZERO));
    if (!page) {
    return core::ptr::null_mut();
    }
    spin_lock_irqsave(&pool.lock, flags);
    pool_initialise_page(pool, page);
    block = pool_block_pop(pool);
    }
    spin_unlock_irqrestore(&pool.lock, flags);
// handle = block->dma;
    pool_check_block(pool, block, mem_flags);
    if (want_init_on_alloc(mem_flags)) {
    memset(block, 0, pool.size);
    }
    return block;
    }
    EXPORT_SYMBOL(dma_pool_alloc);
//
// dma_pool_free - put block back into dma pool
// @pool: the dma pool holding the block
// @vaddr: virtual address of block
// @dma: dma address of block
//
// Caller promises neither device nor driver will again touch this block
// unless it is first re-allocated.
//
#[no_mangle]
pub unsafe extern "C" fn dma_pool_free(pool: *mut dma_pool, vaddr: *mut c_void, dma: dma_addr_t) {
    let mut block = vaddr;
    let mut flags = 0;
    spin_lock_irqsave(&pool.lock, flags);
    if (!pool_block_err(pool, vaddr, dma)) {
    pool_block_push(pool, block, dma);
    pool.nr_active -= 1;
    }
    spin_unlock_irqrestore(&pool.lock, flags);
    }
    EXPORT_SYMBOL(dma_pool_free);
//
// Managed DMA pool
//
#[no_mangle]
unsafe extern "C" fn dmam_pool_release(dev: *mut device, res: *mut c_void) {
    let mut pool = *res;
    dma_pool_destroy(pool);
    }
#[no_mangle]
unsafe extern "C" fn dmam_pool_match(dev: *mut device, res: *mut c_void, match_data: *mut c_void) -> c_int {
    return *res == match_data;
    }
//
// dmam_pool_create - Managed dma_pool_create()
// @name: name of pool, for diagnostics
// @dev: device that will be doing the DMA
// @size: size of the blocks in this pool.
// @align: alignment requirement for blocks; must be a power of two
// @allocation: returned blocks won't cross this boundary (or zero)
//
// Managed dma_pool_create().  DMA pool created with this function is
// automatically destroyed on driver detach.
//
// Return: a managed dma allocation pool with the requested
// characteristics, or %NULL if one can't be created.
//
#[no_mangle]
pub unsafe extern "C" fn dmam_pool_create(name: *mut c_char, dev: *mut device, size: size_t, align: size_t, allocation: size_t) -> *mut c_void {
    let mut ptr = core::ptr::null_mut();
    let mut pool = core::ptr::null_mut();
    ptr = devres_alloc(dmam_pool_release, sizeof!(*ptr), GFP_KERNEL);
    if (!ptr) {
    return core::ptr::null_mut();
    }
    pool = *ptr = dma_pool_create(name, dev, size, align, allocation);
    if (pool) {
    devres_add(dev, ptr);
    }
    else {
    devres_free(ptr);
    }
    return pool;
    }
    EXPORT_SYMBOL(dmam_pool_create);
//
// dmam_pool_destroy - Managed dma_pool_destroy()
// @pool: dma pool that will be destroyed
//
// Managed dma_pool_destroy().
//
#[no_mangle]
pub unsafe extern "C" fn dmam_pool_destroy(pool: *mut dma_pool) {
    let mut dev = pool.dev;
    WARN_ON!(devres_release(dev, dmam_pool_release, dmam_pool_match, pool));
    }
    EXPORT_SYMBOL(dmam_pool_destroy);