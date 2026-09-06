//! Automatically rewritten from C to Rust
//! Source: kernel/dma/pool.c
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
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2020 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_gen_pool {
    pub cc_shared: bool,
    pub pool: *mut gen_pool,
}

    static struct dma_gen_pool atomic_pool_dma __ro_after_init;
    static unsigned long pool_size_dma;
    static struct dma_gen_pool atomic_pool_dma32 __ro_after_init;
    static unsigned long pool_size_dma32;
    static struct dma_gen_pool atomic_pool_kernel __ro_after_init;
    static unsigned long pool_size_kernel;
// Size can be defined by the coherent_pool command line
    static size_t atomic_pool_size;
// Dynamic background expansion when the atomic pool is near capacity
pub static mut atomic_pool_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn early_coherent_pool(p: *mut c_char) -> c_int {
    atomic_pool_size = memparse(p, &p);
    return 0;
    }
    early_param!("coherent_pool", early_coherent_pool);
#[no_mangle]
unsafe extern "C" fn dma_atomic_pool_debugfs_init()  {
pub static mut root: *mut c_void = core::ptr::null_mut();
    root = debugfs_create_dir("dma_pools", core::ptr::null_mut());
    debugfs_create_ulong("pool_size_dma", 0400, root, &pool_size_dma);
    debugfs_create_ulong("pool_size_dma32", 0400, root, &pool_size_dma32);
    debugfs_create_ulong("pool_size_kernel", 0400, root, &pool_size_kernel);
    }
#[no_mangle]
unsafe extern "C" fn dma_atomic_pool_size_add(gfp: gfp_t, size: usize) {
    if (gfp & __GFP_DMA) {
    pool_size_dma += size;
    }

    else if (gfp & __GFP_DMA32) {
    pool_size_dma32 += size;
    }
    else {
    pool_size_kernel += size;
    }
    }
#[no_mangle]
unsafe extern "C" fn cma_in_zone(gfp: gfp_t) -> bool {
    let mut size = 0;
    let mut end;
pub static mut cma: *mut c_void = core::ptr::null_mut();
    cma = dev_get_cma_area(core::ptr::null_mut());
    if (!cma) {
    return false;
    }
    size = cma_get_size(cma);
    if (!size) {
    return false;
    }
// CMA can't cross zone boundaries, see cma_activate_area()
    end = cma_get_base(cma) + size - 1;
    if (IS_ENABLED!(CONFIG_ZONE_DMA) && (gfp & GFP_DMA)) {
    return end <= zone_dma_limit;
    }
    if (IS_ENABLED!(CONFIG_ZONE_DMA32) && (gfp & GFP_DMA32)) {
    return end <= max(DMA_BIT_MASK(32), zone_dma_limit);
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn atomic_pool_expand(dma_pool: *mut dma_gen_pool, pool_size: size_t, gfp: gfp_t) -> c_int {
    let mut order = 0;
    let mut page = core::ptr::null_mut();
pub static mut leak_pages: bool = false;
pub static mut addr: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    pgprot_t prot __maybe_unused;
// Cannot allocate larger than MAX_PAGE_ORDER
    order = min(get_order(pool_size), MAX_PAGE_ORDER);
    do {
    pool_size = 1 << (PAGE_SHIFT + order);
    if (cma_in_zone(gfp)) {
    page = dma_alloc_from_contiguous(core::ptr::null_mut(), 1 << order,
    order, false);
    }
    if (!page) {
    page = alloc_pages(gfp | __GFP_NOWARN, order);
    }
    } while (!page && order-- > 0);
    if (!page) {
// goto;
    }
    arch_dma_prep_coherent(page, pool_size);

    if (dma_pool.cc_shared) {
    prot = pgprot_decrypted(pgprot_dmacoherent(PAGE_KERNEL));
    }
    else {
    prot = pgprot_dmacoherent(PAGE_KERNEL);
    }
    addr = dma_common_contiguous_remap(page, pool_size, prot,
    __builtin_return_address(0));
    if (!addr) {
// goto;
    }

    addr = page_to_virt(page);

//
// Memory in the atomic DMA pools must be unencrypted, the pools do not
// shrink so no re-encryption occurs in dma_direct_free().
//
    if (dma_pool.cc_shared) {
    ret = set_memory_decrypted((unsigned long)page_to_virt(page),
    1 << order);
    if (ret) {
    leak_pages = true;
// goto;
    }
    }
    ret = gen_pool_add_virt(dma_pool.pool, (unsigned long)addr,
    page_to_phys(page), pool_size, NUMA_NO_NODE);
    if (ret) {
// goto;
    }
    dma_atomic_pool_size_add(gfp, pool_size);
    return 0;
// label;
    if (dma_pool.cc_shared &&
    set_memory_encrypted((unsigned long)page_to_virt(page), 1 << order)) {
    leak_pages = true;
    }
// label;
    dma_common_free_remap(addr, pool_size);
// label;
    if (!leak_pages) {
    __free_pages(page, order);
    }
// label;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atomic_pool_resize(dma_pool: *mut dma_gen_pool, gfp: gfp_t) {
    if (dma_pool.pool && gen_pool_avail(dma_pool.pool) < atomic_pool_size) {
    atomic_pool_expand(dma_pool, gen_pool_size(dma_pool.pool), gfp);
    }
    }
#[no_mangle]
unsafe extern "C" fn atomic_pool_work_fn(work: *mut work_struct) {
    if (IS_ENABLED!(CONFIG_ZONE_DMA)) {
    atomic_pool_resize(&atomic_pool_dma,
    GFP_KERNEL | GFP_DMA);
    }
    if (IS_ENABLED!(CONFIG_ZONE_DMA32)) {
    atomic_pool_resize(&atomic_pool_dma32,
    GFP_KERNEL | GFP_DMA32);
    }
    atomic_pool_resize(&atomic_pool_kernel, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn __dma_atomic_pool_init(dma_pool: *mut dma_gen_pool, pool_size: size_t, gfp: gfp_t) -> *mut c_void {
    let mut ret = 0;
    dma_pool.pool = gen_pool_create(PAGE_SHIFT, NUMA_NO_NODE);
    if (!dma_pool.pool) {
    return core::ptr::null_mut();
    }
    gen_pool_set_algo(dma_pool.pool, gen_pool_first_fit_order_align, core::ptr::null_mut());
// if platform is using memory encryption atomic pools are by default shared.
    if (cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    dma_pool.cc_shared = true;
    }
    else {
    dma_pool.cc_shared = false;
    }
    ret = atomic_pool_expand(dma_pool, pool_size, gfp);
    if (ret) {
    gen_pool_destroy(dma_pool.pool);
    dma_pool.pool = core::ptr::null_mut();
    pr_err!("DMA: failed to allocate %zu KiB %pGg pool for atomic allocation\n",
    pool_size >> 10, &gfp);
    return core::ptr::null_mut();
    }
    pr_info!("DMA: preallocated %zu KiB %pGg pool for atomic allocations\n",
    gen_pool_size(dma_pool.pool) >> 10, &gfp);
    return dma_pool;
    }

#[no_mangle]
unsafe extern "C" fn dma_atomic_pool_init() -> c_int {
pub static mut ret: c_int = 0;
//
// If coherent_pool was not used on the command line, default the pool
// sizes to 128KB per 1GB of memory, min 128KB, max MAX_PAGE_ORDER.
//
    if (!atomic_pool_size) {
pub static mut pages: c_ulong = 0;
    pages = min_t(unsigned long, pages, MAX_ORDER_NR_PAGES);
    atomic_pool_size = max_t(size_t, pages << PAGE_SHIFT, SZ_128K);
    }
    INIT_WORK(&atomic_pool_work, atomic_pool_work_fn);
// All memory might be in the DMA zone(s) to begin with
    if (has_managed_zone(ZONE_NORMAL)) {
    __dma_atomic_pool_init(&atomic_pool_kernel, atomic_pool_size, GFP_KERNEL);
    if (!atomic_pool_kernel.pool) {
    ret = -ENOMEM;
    }
    }
    if (has_managed_dma()) {
    __dma_atomic_pool_init(&atomic_pool_dma, atomic_pool_size,
    GFP_KERNEL | GFP_DMA);
    if (!atomic_pool_dma.pool) {
    ret = -ENOMEM;
    }
    }
    if (has_managed_dma32) {
    __dma_atomic_pool_init(&atomic_pool_dma32, atomic_pool_size,
    GFP_KERNEL | GFP_DMA32);
    if (!atomic_pool_dma32.pool) {
    ret = -ENOMEM;
    }
    }
    dma_atomic_pool_debugfs_init();
    return ret;
    }
    postcore_initcall!(dma_atomic_pool_init);
#[no_mangle]
pub unsafe extern "C" fn __dma_guess_pool(first: *mut dma_gen_pool, second: *mut dma_gen_pool, third: *mut dma_gen_pool) -> *mut c_void {
    if (first.pool) {
    return first;
    }
    if (second && second.pool) {
    return second;
    }
    if (third && third.pool) {
    return third;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dma_guess_pool(prev: *mut dma_gen_pool, gfp: gfp_t) -> *mut c_void {
    if (!prev) {
    if (gfp & GFP_DMA) {
    return __dma_guess_pool(&atomic_pool_dma,
    &atomic_pool_dma32,
    &atomic_pool_kernel);
    }
    if (gfp & GFP_DMA32) {
    return __dma_guess_pool(&atomic_pool_dma32,
    &atomic_pool_dma,
    &atomic_pool_kernel);
    }
    return __dma_guess_pool(&atomic_pool_kernel,
    &atomic_pool_dma32,
    &atomic_pool_dma);
    }
    if (prev == &atomic_pool_kernel) {
    return __dma_guess_pool(&atomic_pool_dma32,
    &atomic_pool_dma, core::ptr::null_mut());
    }
    if (prev == &atomic_pool_dma32) {
    return __dma_guess_pool(&atomic_pool_dma, core::ptr::null_mut(), core::ptr::null_mut());
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __dma_alloc_from_pool(dev: *mut device, size: size_t, pool: *mut gen_pool, cpu_addr: *mut *mut c_void, phys_addr_t: c_void) -> *mut c_void {
    let mut addr = 0;
    let mut phys;
    addr = gen_pool_alloc(pool, size);
    if (!addr) {
    return core::ptr::null_mut();
    }
    phys = gen_pool_virt_to_phys(pool, addr);
    if (phys_addr_ok && !phys_addr_ok(dev, phys, size)) {
    gen_pool_free(pool, addr, size);
    return core::ptr::null_mut();
    }
    if (gen_pool_avail(pool) < atomic_pool_size) {
    schedule_work(&atomic_pool_work);
    }
// cpu_addr = addr;
    memset(*cpu_addr, 0, size);
    return pfn_to_page(__phys_to_pfn(phys));
    }
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_from_pool(dev: *mut device, size: size_t, cpu_addr: *mut *mut c_void, gfp: gfp_t, attrs: c_ulong, phys_addr_t: c_void) -> *mut c_void {
    let mut dma_pool = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut pool_found: bool = false;
    while ((dma_pool = dma_guess_pool(dma_pool, gfp))) {
    if (dma_pool.cc_shared != !!(attrs & __DMA_ATTR_ALLOC_CC_SHARED)) {
    continue;
    }
    pool_found = true;
    page = __dma_alloc_from_pool(dev, size, dma_pool.pool, cpu_addr,
    phys_addr_ok);
    if (page) {
    return page;
    }
    }
    if (pool_found) {
    WARN(!(gfp & __GFP_NOWARN), "DMA pool exhausted for %s\n", dev_name(dev));
    }
    else {
    WARN(1, "Failed to get suitable pool for %s\n", dev_name(dev));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dma_free_from_pool(dev: *mut device, start: *mut c_void, size: usize) -> bool {
    let mut dma_pool = core::ptr::null_mut();
    while ((dma_pool = dma_guess_pool(dma_pool, 0))) {
    if (!gen_pool_has_addr(dma_pool.pool, (unsigned long)start, size)) {
    continue;
    }
    gen_pool_free(dma_pool.pool, (unsigned long)start, size);
    return true;
    }
    return false;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pool_phys_match {
    pub phys: phys_addr_t,
    pub size: usize,
    pub addr: c_ulong,
    pub found: bool,
}

#[no_mangle]
pub unsafe extern "C" fn dma_pool_find_phys(pool: *mut gen_pool, chunk: *mut gen_pool_chunk, data: *mut c_void) {
    let mut match = data;
pub static mut end: phys_addr_t = 0;
    let mut chunk_end;
    if (match.found) {
    return;
    }
    chunk_end = chunk.phys_addr + (chunk.end_addr - chunk.start_addr);
    if (match.phys < chunk.phys_addr || end > chunk_end) {
    return;
    }
    match.addr = chunk.start_addr + (match.phys - chunk.phys_addr);
    match.found = true;
    }
#[no_mangle]
pub unsafe extern "C" fn dma_free_from_pool_phys(dma_pool: *mut dma_gen_pool, phys: phys_addr_t, size: size_t) -> bool {
pub static mut dma_pool_phys_match: usize = 0;
    gen_pool_for_each_chunk(dma_pool.pool, dma_pool_find_phys, &match);
    if (!match.found) {
    return false;
    }
    gen_pool_free(dma_pool.pool, match.addr, size);
    return true;
    }
//
// FIXME: We could avoid this by storing the remapped virtual address in
// struct page and using that for lookup.
//
#[no_mangle]
pub unsafe extern "C" fn dma_free_from_pool_page(dev: *mut device, page: *mut page, size: usize) -> bool {
    let mut dma_pool = core::ptr::null_mut();
pub static mut phys: phys_addr_t = 0;
    if (!IS_ENABLED!(CONFIG_DMA_DIRECT_REMAP)) {
    return dma_free_from_pool(dev, page_address(page), size);
    }
    while ((dma_pool = dma_guess_pool(dma_pool, 0))) {
    if (dma_free_from_pool_phys(dma_pool, phys, size)) {
    return true;
    }
    }
    return false;
    }