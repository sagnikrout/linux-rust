//! Automatically rewritten from C to Rust
//! Source: kernel/dma/contiguous.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Contiguous Memory Allocator for DMA mapping framework
// Copyright (c) 2010-2011 by Samsung Electronics.
// Written by:
// Marek Szyprowski <m.szyprowski@samsung.com>
// Michal Nazarewicz <mina86@mina86.com>
//
// Contiguous Memory Allocator
//
// The Contiguous Memory Allocator (CMA) makes it possible to
// allocate big contiguous chunks of memory after the system has
// booted.
//
// Why is it needed?
//
// Various devices on embedded systems have no scatter-getter and/or
// IO map support and require contiguous blocks of memory to
// operate.  They include devices such as cameras, hardware video
// coders, etc.
//
// Such devices often require big memory buffers (a full HD frame
// is, for instance, more than 2 mega pixels large, i.e. more than 6
// MB of memory), which makes mechanisms such as kmalloc() or
// alloc_page() ineffective.
//
// At the same time, a solution where a big memory region is
// reserved for a device is suboptimal since often more memory is
// reserved then strictly required and, moreover, the memory is
// inaccessible to page system even if device drivers don't use it.
//
// CMA tries to solve this issue by operating on memory regions
// where only movable pages can be allocated from.  This way, kernel
// can use the memory for pagecache and when device driver requests
// it, allocated pages can be migrated.
//

pub const CMA_SIZE_MBYTES: c_int = 0;

    static struct cma *dma_contiguous_areas[MAX_CMA_AREAS];
    static unsigned int dma_contiguous_areas_num;
#[no_mangle]
unsafe extern "C" fn dma_contiguous_insert_area(cma: *mut cma) -> c_int {
    if (dma_contiguous_areas_num >= ARRAY_SIZE!(dma_contiguous_areas)) {
    return -EINVAL;
    }
    dma_contiguous_areas[dma_contiguous_areas_num++] = cma;
    return 0;
    }
//
// dma_contiguous_get_area_by_idx() - Get contiguous area at given index
// @idx: index of the area we query
//
// Queries for the contiguous area located at index @idx.
//
// Returns:
// A pointer to the requested contiguous area, or NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_get_area_by_idx(idx: c_uint) -> *mut c_void {
    if (idx >= dma_contiguous_areas_num) {
    return core::ptr::null_mut();
    }
    return dma_contiguous_areas[idx];
    }
    EXPORT_SYMBOL_GPL(dma_contiguous_get_area_by_idx);
pub static mut dma_contiguous_default_area: *mut c_void = core::ptr::null_mut();
//
// Default global CMA area size can be defined in kernel's .config.
// This is useful mainly for distro maintainers to create a kernel
// that works correctly for most supported systems.
// The size can be set in bytes or as a percentage of the total memory
// in the system.
//
// Users, who want to set the size of global CMA area for their system
// should use cma= kernel parameter.
//

pub static mut __initdata: phys_addr_t  size_cmdline = 0;
    static phys_addr_t base_cmdline __initdata;
    static phys_addr_t limit_cmdline __initdata;
#[no_mangle]
unsafe extern "C" fn early_cma(p: *mut c_char) -> c_int {
    if (!p) {
    pr_err!("Config string not provided\n");
    return -EINVAL;
    }
    size_cmdline = memparse(p, &p);
    if (*p != '@') {
    return 0;
    }
    base_cmdline = memparse(p + 1, &p);
    if (*p != '-') {
    limit_cmdline = base_cmdline + size_cmdline;
    return 0;
    }
    limit_cmdline = memparse(p + 1, &p);
    return 0;
    }
    early_param!("cma", early_cma);
#[no_mangle]
pub unsafe extern "C" fn dev_get_cma_area(dev: *mut device) -> *mut c_void {
    if (dev && dev.cma_area) {
    return dev.cma_area;
    }
    return dma_contiguous_default_area;
    }
    EXPORT_SYMBOL_GPL(dev_get_cma_area);

    static struct cma *dma_contiguous_numa_area[MAX_NUMNODES];
    static phys_addr_t numa_cma_size[MAX_NUMNODES] __initdata;
    static phys_addr_t pernuma_size_bytes __initdata;
    static bool numa_cma_configured __initdata;
#[no_mangle]
unsafe extern "C" fn early_numa_cma(p: *mut c_char) -> c_int {
    int nid, count = 0;
    let mut node = 0;
    let mut size;
    let mut s = p;
    while (*s) {
    if (sscanf(s, "%lu%n", &node, &count) != 1) {
    break;
    }
    if (s[count] == ':') {
    if (node >= MAX_NUMNODES) {
    break;
    }
    nid = array_index_nospec(node, MAX_NUMNODES);
    s += count + 1;
    size = memparse(s, &s);
    numa_cma_size[nid] = size;
    if (*s == ',') {
    s += 1;
    }
    else {
    break;
    }
    } else {
    break;
    }
    }
    numa_cma_configured = true;
    return 0;
    }
    early_param!("numa_cma", early_numa_cma);
#[no_mangle]
unsafe extern "C" fn early_cma_pernuma(p: *mut c_char) -> c_int {
    pernuma_size_bytes = memparse(p, &p);
    numa_cma_configured = true;
    return 0;
    }
    early_param!("cma_pernuma", early_cma_pernuma);

#[no_mangle]
unsafe extern "C" fn cma_early_percent_memory() -> phys_addr_t __init __maybe_unused {
pub static mut total_pages: c_ulong = 0;
    return (total_pages * CONFIG_CMA_SIZE_PERCENTAGE / 100) << PAGE_SHIFT;
    }

#[no_mangle]
pub unsafe extern "C" fn cma_early_percent_memory() -> __maybe_unused phys_addr_t {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dma_numa_cma_reserve()  {
    let mut nid = 0;
    if (IS_ENABLED!(CONFIG_CMA_SIZE_PERNUMA) &&
    !numa_cma_configured && dma_contiguous_default_area &&
    nr_online_nodes > 1) {
    pernuma_size_bytes = cma_get_size(dma_contiguous_default_area);
    }
    for_each_node(nid) {
    let mut size;
    char name[CMA_MAX_NAME];
pub static mut cma: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!node_online(nid)) {
    if (pernuma_size_bytes || numa_cma_size[nid]) {
    pr_warn!("invalid node %d specified\n", nid);
    }
    continue;
    }
// per-node numa setting has the priority
    size = numa_cma_size[nid] ?: pernuma_size_bytes;
    if (!size) {
    continue;
    }
    cma = &dma_contiguous_numa_area[nid];
    snprintf(name, sizeof!(name), "numa%d", nid);
    ret = cma_declare_contiguous_nid(0, size, 0, 0, 0, false, name, cma, nid);
    if (ret) {
    pr_warn!("%s: reservation failed: err %d, node %d", __func__,
    ret, nid);
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn dma_numa_cma_reserve()  {
    }

//
// dma_contiguous_reserve() - reserve area(s) for contiguous memory handling
// @limit: End address of the reserved memory (optional, 0 for any).
//
// This function reserves memory from early allocator. It should be
// called by arch specific code once the early allocator (memblock or bootmem)
// has been activated and all other subsystems have already allocated/reserved
// memory.
//
#[no_mangle]
pub unsafe extern "C" fn dma_contiguous_reserve(limit: phys_addr_t)  {
pub static mut selected_size: phys_addr_t = 0;
pub static mut selected_base: phys_addr_t = 0;
pub static mut selected_limit: phys_addr_t = 0;
pub static mut fixed: bool = false;
    pr_debug!("%s(limit %08lx)\n", __func__, (unsigned long)limit);
    if (size_cmdline != -1) {
    selected_size = size_cmdline;
    selected_base = base_cmdline;
// Hornor the user setup dma address limit
    selected_limit = limit_cmdline ?: limit;
    if (base_cmdline + size_cmdline == limit_cmdline) {
    fixed = true;
    }
    } else {

    selected_size = size_bytes;

    selected_size = cma_early_percent_memory();

    selected_size = min(size_bytes, cma_early_percent_memory());

    selected_size = max(size_bytes, cma_early_percent_memory());

    }
    if (selected_size && !dma_contiguous_default_area) {
    let mut ret = 0;
    pr_debug!("%s: reserving %ld MiB for global area\n", __func__,
    (unsigned long)selected_size / SZ_1M);
    ret = dma_contiguous_reserve_area(selected_size, selected_base,
    selected_limit,
    &dma_contiguous_default_area,
    fixed);
    if (ret) {
    return;
    }
//
// We need to insert the new area in our list to avoid
// any inconsistencies between having the default area
// listed in the DT or not.
//
// The DT case is handled by rmem_cma_setup() and will
// always insert all its areas in our list. However, if
// it didn't run (because OF_RESERVED_MEM isn't set, or
// there's no DT region specified), then we don't have a
// default area yet, and no area in our list.
//
// This block creates the default area in such a case,
// but we also need to insert it in our list to avoid
// having a default area but an empty list.
//
    ret = dma_contiguous_insert_area(dma_contiguous_default_area);
    if (ret) {
    pr_warn!("Couldn't queue default CMA region for heap creation.");
    }
    }
    dma_numa_cma_reserve();
    }
    void __weak
    dma_contiguous_early_fixup(phys_addr_t base, unsigned long size)
    {
    }
//
// dma_contiguous_reserve_area() - reserve custom contiguous area
// @size: Size of the reserved area (in bytes),
// @base: Base address of the reserved area optional, use 0 for any
// @limit: End address of the reserved memory (optional, 0 for any).
// @res_cma: Pointer to store the created cma region.
// @fixed: hint about where to place the reserved area
//
// This function reserves memory from early allocator. It should be
// called by arch specific code once the early allocator (memblock or bootmem)
// has been activated and all other subsystems have already allocated/reserved
// memory. This function allows to create custom reserved areas for specific
// devices.
//
// If @fixed is true, reserve contiguous area at exactly @base.  If false,
// reserve in range from @base to @limit.
//
    int __init dma_contiguous_reserve_area(phys_addr_t size, phys_addr_t base,
    phys_addr_t limit, cma **res_cma,
    bool fixed)
    {
    let mut ret = 0;
    ret = cma_declare_contiguous(base, size, limit, 0, 0, fixed,
    "reserved", res_cma);
    if (ret) {
    return ret;
    }
// Architecture specific contiguous memory fixup.
    dma_contiguous_early_fixup(cma_get_base(*res_cma),
    cma_get_size(*res_cma));
    return 0;
    }
//
// dma_alloc_from_contiguous() - allocate pages from contiguous area
// @dev:   Pointer to device for which the allocation is performed.
// @count: Requested number of pages.
// @align: Requested alignment of pages (in PAGE_SIZE order).
// @no_warn: Avoid printing message about failed allocation.
//
// This function allocates memory buffer for specified device. It uses
// device specific contiguous memory area if available or the default
// global one. Requires architecture specific dev_get_cma_area() helper
// function.
//
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_from_contiguous(dev: *mut device, count: size_t, align: c_uint, no_warn: bool) -> *mut c_void {
    if (align > CONFIG_CMA_ALIGNMENT) {
    align = CONFIG_CMA_ALIGNMENT;
    }
    return cma_alloc(dev_get_cma_area(dev), count, align, no_warn);
    }
//
// dma_release_from_contiguous() - release allocated pages
// @dev:   Pointer to device for which the pages were allocated.
// @pages: Allocated pages.
// @count: Number of allocated pages.
//
// This function releases memory allocated by dma_alloc_from_contiguous().
// It returns false when provided pages do not belong to contiguous area and
// true otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn dma_release_from_contiguous(dev: *mut device, pages: *mut page, count: c_int) -> bool {
    return cma_release(dev_get_cma_area(dev), pages, count);
    }
#[no_mangle]
pub unsafe extern "C" fn cma_alloc_aligned(cma: *mut cma, size: size_t, gfp: gfp_t) -> *mut c_void {
pub static mut align: c_uint = 0;
    return cma_alloc(cma, size >> PAGE_SHIFT, align, gfp & __GFP_NOWARN);
    }
//
// dma_alloc_contiguous() - allocate contiguous pages
// @dev:   Pointer to device for which the allocation is performed.
// @size:  Requested allocation size.
// @gfp:   Allocation flags.
//
// tries to use device specific contiguous memory area if available, or it
// tries to use per-numa cma, if the allocation fails, it will fallback to
// try default global one.
//
// Note that it bypass one-page size of allocations from the per-numa and
// global area as the addresses within one page are always contiguous, so
// there is no need to waste CMA pages for that kind; it also helps reduce
// fragmentations.
//
#[no_mangle]
pub unsafe extern "C" fn dma_alloc_contiguous(dev: *mut device, size: size_t, gfp: gfp_t) -> *mut c_void {

pub static mut nid: c_int = 0;

// CMA can be used only in the context which permits sleeping
    if (!gfpflags_allow_blocking(gfp)) {
    return core::ptr::null_mut();
    }
    if (dev.cma_area) {
    return cma_alloc_aligned(dev.cma_area, size, gfp);
    }
    if (size <= PAGE_SIZE) {
    return core::ptr::null_mut();
    }

    if (nid != NUMA_NO_NODE && !(gfp & (GFP_DMA | GFP_DMA32))) {
    let mut cma = dma_contiguous_numa_area[nid];
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (cma) {
    page = cma_alloc_aligned(cma, size, gfp);
    if (page) {
    return page;
    }
    }
    }

    if (!dma_contiguous_default_area) {
    return core::ptr::null_mut();
    }
    return cma_alloc_aligned(dma_contiguous_default_area, size, gfp);
    }
//
// dma_free_contiguous() - release allocated pages
// @dev:   Pointer to device for which the pages were allocated.
// @page:  Pointer to the allocated pages.
// @size:  Size of allocated pages.
//
// This function releases memory allocated by dma_alloc_contiguous(). As the
// cma_release returns false when provided pages do not belong to contiguous
// area and true otherwise, this function then does a fallback __free_pages()
// upon a false-return.
//
#[no_mangle]
pub unsafe extern "C" fn dma_free_contiguous(dev: *mut device, page: *mut page, size: usize) {
pub static mut count: c_uint = 0;
// if dev has its own cma, free page from there
    if (dev.cma_area) {
    if (cma_release(dev.cma_area, page, count)) {
    return;
    }
    } else {
//
// otherwise, page is from either per-numa cma or default cma
//

    if (cma_release(dma_contiguous_numa_area[page_to_nid(page)],
    page, count)) {
    return;
    }

    if (cma_release(dma_contiguous_default_area, page, count)) {
    return;
    }
    }
// not in any cma, free from buddy
    __free_pages(page, get_order(size));
    }
//
// Support for reserved memory regions defined in device tree
//

#[no_mangle]
unsafe extern "C" fn rmem_cma_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int {
    dev.cma_area = rmem.priv;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rmem_cma_device_release(rmem: *mut reserved_mem, dev: *mut device) {
    dev.cma_area = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __rmem_cma_verify_node(node: c_ulong) -> c_int {
    if (!of_get_flat_dt_prop(node, "reusable", core::ptr::null_mut()) ||
    of_get_flat_dt_prop(node, "no-map", core::ptr::null_mut())) {
    return -ENODEV;
    }
    if (size_cmdline != -1 &&
    of_get_flat_dt_prop(node, "linux,cma-default", core::ptr::null_mut())) {
    pr_err!("Skipping dt linux,cma-default node in favor for \"cma=\" kernel param.\n");
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmem_cma_validate(node: c_ulong, align: *mut phys_addr_t) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    return ret;
    }
    if (align) {
// align = max_t(phys_addr_t, *align, CMA_MIN_ALIGNMENT_BYTES);
    }
    return 0;
    }
    static int __init rmem_cma_fixup(unsigned long node, phys_addr_t base,
    phys_addr_t size)
    {
pub static mut ret: c_int = 0;
    if (ret) {
    return ret;
    }
// Architecture specific contiguous memory fixup.
    dma_contiguous_early_fixup(base, size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmem_cma_setup(node: c_ulong, rmem: *mut reserved_mem) -> c_int {
pub static mut default_cma: bool = false;
pub static mut cma: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = __rmem_cma_verify_node(node);
    if (ret) {
    return ret;
    }
    if (!IS_ALIGNED(rmem.base | rmem.size, CMA_MIN_ALIGNMENT_BYTES)) {
    pr_err!("Reserved memory: incorrect alignment of CMA region\n");
    return -EINVAL;
    }
    ret = cma_init_reserved_mem(rmem.base, rmem.size, 0, rmem.name, &cma);
    if (ret) {
    pr_err!("Reserved memory: unable to setup CMA region\n");
    return ret;
    }
    if (default_cma) {
    dma_contiguous_default_area = cma;
    }
    rmem.priv = cma;
    pr_info!("Reserved memory: created CMA memory pool at %pa, size %ld MiB\n",
    &rmem.base, (unsigned long)rmem.size / SZ_1M);
    ret = dma_contiguous_insert_area(cma);
    if (ret) {
    pr_warn!("Couldn't store CMA reserved area.");
    }
    return 0;
    }
pub static mut reserved_mem_ops: usize = 0;
    RESERVEDMEM_OF_DECLARE(cma, "shared-dma-pool", &rmem_cma_ops);