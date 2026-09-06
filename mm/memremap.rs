//! Automatically rewritten from C to Rust
//! Source: mm/memremap.c
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
// Copyright(c) 2015 Intel Corporation. All rights reserved.

pub static mut pgmap_array: usize = 0;
//
// The memremap() and memremap_pages() interfaces are alternately used
// to map persistent memory namespaces. These interfaces place different
// constraints on the alignment and size of the mapping (namespace).
// memremap() can map individual PAGE_SIZE pages. memremap_pages() can
// only map subsections (2MB), and at least one architecture (PowerPC)
// the minimum mapping granularity of memremap_pages() is 16MB.
//
// The role of memremap_compat_align() is to communicate the minimum
// arch supported alignment of a namespace such that it can freely
// switch modes without violating the arch constraint. Namely, do not
// allow a namespace to be PAGE_SIZE aligned since that namespace may be
// reconfigured into a mode that requires SUBSECTION_SIZE alignment.
//

#[no_mangle]
pub unsafe extern "C" fn memremap_compat_align() -> c_ulong {
    return SUBSECTION_SIZE;
    }
    EXPORT_SYMBOL_GPL(memremap_compat_align);

#[no_mangle]
unsafe extern "C" fn pgmap_array_delete(range: *mut range) {
    xa_store_range(&pgmap_array, PHYS_PFN(range.start), PHYS_PFN(range.end),
    core::ptr::null_mut(), GFP_KERNEL);
    synchronize_rcu();
    }
#[no_mangle]
unsafe extern "C" fn pfn_first(pgmap: *mut dev_pagemap, range_id: c_int) -> c_ulong {
    let mut range = &pgmap.ranges[range_id];
pub static mut pfn: c_ulong = 0;
    if (range_id) {
    return pfn;
    }
    return pfn + vmem_altmap_offset(pgmap_altmap(pgmap));
    }
#[no_mangle]
pub unsafe extern "C" fn pgmap_pfn_valid(pgmap: *mut dev_pagemap, pfn: c_ulong) -> bool {
    let mut i = 0;
    while (i < pgmap.nr_range) {
    let mut range = &pgmap.ranges[i];
    if (pfn >= PHYS_PFN(range.start) &&
    pfn <= PHYS_PFN(range.end)) {
    return pfn >= pfn_first(pgmap, i);
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pfn_end(pgmap: *mut dev_pagemap, range_id: c_int) -> c_ulong {
    let mut range = &pgmap.ranges[range_id];
    return (range.start + range_len(range)) >> PAGE_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn pfn_len(pgmap: *mut dev_pagemap, range_id: c_ulong) -> c_ulong {
    return (pfn_end(pgmap, range_id) -
    pfn_first(pgmap, range_id)) >> pgmap.vmemmap_shift;
    }
#[no_mangle]
unsafe extern "C" fn pageunmap_range(pgmap: *mut dev_pagemap, range_id: c_int) {
    let mut range = &pgmap.ranges[range_id];
pub static mut first_page: *mut c_void = core::ptr::null_mut();
// make sure to access a memmap that was actually initialized
    first_page = pfn_to_page(pfn_first(pgmap, range_id));
// pages are dead and unused, undo the arch mapping
    mem_hotplug_begin();
    remove_pfn_range_from_zone(page_zone(first_page), PHYS_PFN(range.start),
    PHYS_PFN(range_len(range)));
    if (pgmap.type == MEMORY_DEVICE_PRIVATE) {
    __remove_pages(PHYS_PFN(range.start),
    PHYS_PFN(range_len(range)), core::ptr::null_mut(), pgmap);
    } else {
    arch_remove_memory(range.start, range_len(range),
    pgmap_altmap(pgmap), pgmap);
    kasan_remove_zero_shadow(__va(range.start), range_len(range));
    }
    mem_hotplug_done();
    pfnmap_untrack(PHYS_PFN(range.start), range_len(range));
    pgmap_array_delete(range);
    }
#[no_mangle]
pub unsafe extern "C" fn memunmap_pages(pgmap: *mut dev_pagemap) {
    let mut i = 0;
    percpu_ref_kill(&pgmap.ref);
    if (pgmap.type != MEMORY_DEVICE_PRIVATE &&
    pgmap.type != MEMORY_DEVICE_COHERENT) {
    for (i = 0; i < pgmap.nr_range; i++)
    }
    percpu_ref_put_many(&pgmap.ref, pfn_len(pgmap, i));
    wait_for_completion(&pgmap.done);
    for (i = 0; i < pgmap.nr_range; i++) {
    pageunmap_range(pgmap, i);
    }
    percpu_ref_exit(&pgmap.ref);
    WARN_ONCE(pgmap.altmap.alloc, "failed to free all reserved pages\n");
    }
    EXPORT_SYMBOL_GPL(memunmap_pages);
#[no_mangle]
unsafe extern "C" fn devm_memremap_pages_release(data: *mut c_void) {
    memunmap_pages(data);
    }
#[no_mangle]
unsafe extern "C" fn dev_pagemap_percpu_release(ref: *mut percpu_ref) {
    let mut pgmap = container_of!(ref, dev_pagemap, ref);
    complete(&pgmap.done);
    }
#[no_mangle]
pub unsafe extern "C" fn pagemap_range(pgmap: *mut dev_pagemap, params: *mut mhp_params, range_id: c_int, nid: c_int) -> c_int {
pub static mut is_private: bool = false;
    let mut range = &pgmap.ranges[range_id];
pub static mut conflict_pgmap: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    let mut is_ram = 0;
    if (WARN_ONCE(pgmap_altmap(pgmap) && range_id > 0,
    "altmap not supported for multiple ranges\n")) {
    return -EINVAL;
    }
    conflict_pgmap = get_dev_pagemap(PHYS_PFN(range.start));
    if (conflict_pgmap) {
    WARN(1, "Conflicting mapping in same section\n");
    put_dev_pagemap(conflict_pgmap);
    return -ENOMEM;
    }
    conflict_pgmap = get_dev_pagemap(PHYS_PFN(range.end));
    if (conflict_pgmap) {
    WARN(1, "Conflicting mapping in same section\n");
    put_dev_pagemap(conflict_pgmap);
    return -ENOMEM;
    }
    is_ram = region_intersects(range.start, range_len(range),
    IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE);
    if (is_ram != REGION_DISJOINT) {
    WARN_ONCE(1, "attempted on %s region %#llx-%#llx\n",
    is_ram == REGION_MIXED ? "mixed" : "ram",
    range.start, range.end);
    return -ENXIO;
    }
    error = xa_err(xa_store_range(&pgmap_array, PHYS_PFN(range.start),
    PHYS_PFN(range.end), pgmap, GFP_KERNEL));
    if (error) {
    return error;
    }
    if (nid < 0) {
    nid = numa_mem_id();
    }
    error = pfnmap_track(PHYS_PFN(range.start), range_len(range),
    &params.pgprot);
    if (error) {
// goto;
    }
    if (!mhp_range_allowed(range.start, range_len(range), !is_private)) {
    error = -EINVAL;
// goto;
    }
    mem_hotplug_begin();
//
// For device private memory we call add_pages() as we only need to
// allocate and initialize struct page for the device memory. More-
// over the device memory is un-accessible thus we do not want to
// create a linear mapping for the memory like arch_add_memory()
// would do.
//
// For all other device memory types, which are accessible by
// the CPU, we do want the linear mapping and thus use
// arch_add_memory().
//
    if (is_private) {
    error = add_pages(nid, PHYS_PFN(range.start),
    PHYS_PFN(range_len(range)), params);
    } else {
    error = kasan_add_zero_shadow(__va(range.start), range_len(range));
    if (error) {
    mem_hotplug_done();
// goto;
    }
    error = arch_add_memory(nid, range.start, range_len(range),
    params);
    }
    if (!error) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    zone = &NODE_DATA(nid).node_zones[ZONE_DEVICE];
    move_pfn_range_to_zone(zone, PHYS_PFN(range.start),
    PHYS_PFN(range_len(range)), params.altmap,
    MIGRATE_MOVABLE, false);
    }
    mem_hotplug_done();
    if (error) {
// goto;
    }
//
// Initialization of the pages has been deferred until now in order
// to allow us to do the work while not holding the hotplug lock.
//
    memmap_init_zone_device(&NODE_DATA(nid).node_zones[ZONE_DEVICE],
    PHYS_PFN(range.start),
    PHYS_PFN(range_len(range)), pgmap);
    if (pgmap.type != MEMORY_DEVICE_PRIVATE &&
    pgmap.type != MEMORY_DEVICE_COHERENT) {
    percpu_ref_get_many(&pgmap.ref, pfn_len(pgmap, range_id));
    }
    return 0;
// label;
    if (!is_private) {
    kasan_remove_zero_shadow(__va(range.start), range_len(range));
    }
// label;
    pfnmap_untrack(PHYS_PFN(range.start), range_len(range));
// label;
    pgmap_array_delete(range);
    return error;
    }
//
// Not device managed version of devm_memremap_pages, undone by
// memunmap_pages().  Please use devm_memremap_pages if you have a struct
// device available.
//
#[no_mangle]
pub unsafe extern "C" fn memremap_pages(pgmap: *mut dev_pagemap, nid: c_int) -> *mut c_void {
pub static mut mhp_params: usize = 0;
pub static mut nr_range: c_int = 0;
    let mut error = 0;
    let mut i = 0;
    if (WARN_ONCE(!nr_range, "nr_range must be specified\n")) {
    return ERR_PTR(-EINVAL);
    }
    if (WARN_ONCE(pgmap.vmemmap_shift > MAX_FOLIO_ORDER,
    "requested folio size unsupported\n")) {
    return ERR_PTR(-EINVAL);
    }
    match (pgmap.type) {
    MEMORY_DEVICE_PRIVATE => {
    if (!IS_ENABLED!(CONFIG_DEVICE_PRIVATE)) {
    WARN(1, "Device private memory not supported\n");
    return ERR_PTR(-EINVAL);
    }
    if (!pgmap.ops || !pgmap.ops.migrate_to_ram) {
    WARN(1, "Missing migrate_to_ram method\n");
    return ERR_PTR(-EINVAL);
    }
    if (!pgmap.ops.folio_free) {
    WARN(1, "Missing folio_free method\n");
    return ERR_PTR(-EINVAL);
    }
    if (!pgmap.owner) {
    WARN(1, "Missing owner\n");
    return ERR_PTR(-EINVAL);
    }
    // break;
    }
    MEMORY_DEVICE_COHERENT => {
    if (!pgmap.ops.folio_free) {
    WARN(1, "Missing folio_free method\n");
    return ERR_PTR(-EINVAL);
    }
    if (!pgmap.owner) {
    WARN(1, "Missing owner\n");
    return ERR_PTR(-EINVAL);
    }
    // break;
    }
    MEMORY_DEVICE_FS_DAX => {
    params.pgprot = pgprot_decrypted(params.pgprot);
    // break;
    }
    MEMORY_DEVICE_GENERIC => {
    // break;
    }
    MEMORY_DEVICE_PCI_P2PDMA => {
    params.pgprot = pgprot_noncached(params.pgprot);
    // break;
    }
    _ => {
    WARN(1, "Invalid pgmap type %d\n", pgmap.type);
    // break;
    }
    }
    init_completion(&pgmap.done);
    error = percpu_ref_init(&pgmap.ref, dev_pagemap_percpu_release, 0,
    GFP_KERNEL);
    if (error) {
    return ERR_PTR(error);
    }
//
// Clear the pgmap nr_range as it will be incremented for each
// successfully processed range. This communicates how many
// regions to unwind in the abort case.
//
    pgmap.nr_range = 0;
    error = 0;
    while (i < nr_range) {
    error = pagemap_range(pgmap, &params, i, nid);
    if (error) {
    break;
    }
    pgmap.nr_range += 1;
    }
    if (i < nr_range) {
    memunmap_pages(pgmap);
    pgmap.nr_range = nr_range;
    return ERR_PTR(error);
    }
    return __va(pgmap.ranges[0].start);
    }
    EXPORT_SYMBOL_GPL(memremap_pages);
//
// devm_memremap_pages - remap and provide memmap backing for the given resource
// @dev: hosting device for @res
// @pgmap: pointer to a struct dev_pagemap
//
// Notes:
// 1/ At a minimum the range and type members of @pgmap must be initialized
// by the caller before passing it to this function
//
// 2/ The altmap field may optionally be initialized, in which case
// PGMAP_ALTMAP_VALID must be set in pgmap->flags.
//
// 3/ The ref field may optionally be provided, in which pgmap->ref must be
// 'live' on entry and will be killed and reaped at
// devm_memremap_pages_release() time, or if this routine fails.
//
// 4/ range is expected to be a host memory range that could feasibly be
// treated as a "System RAM" range, i.e. not a device mmio range, but
// this is not enforced.
//
#[no_mangle]
pub unsafe extern "C" fn devm_memremap_pages(dev: *mut device, pgmap: *mut dev_pagemap) -> *mut c_void {
    let mut error = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = memremap_pages(pgmap, dev_to_node(dev));
    if (IS_ERR(ret)) {
    return ret;
    }
    error = devm_add_action_or_reset(dev, devm_memremap_pages_release,
    pgmap);
    if (error) {
    return ERR_PTR(error);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_memremap_pages);
#[no_mangle]
pub unsafe extern "C" fn devm_memunmap_pages(dev: *mut device, pgmap: *mut dev_pagemap) {
    devm_release_action(dev, devm_memremap_pages_release, pgmap);
    }
    EXPORT_SYMBOL_GPL(devm_memunmap_pages);
//
// get_dev_pagemap() - take a new live reference on the dev_pagemap for @pfn
// @pfn: page frame number to lookup page_map
//
#[no_mangle]
pub unsafe extern "C" fn get_dev_pagemap(pfn: c_ulong) -> *mut c_void {
pub static mut pgmap: *mut c_void = core::ptr::null_mut();
pub static mut phys: resource_size_t = 0;
    rcu_read_lock();
    pgmap = xa_load(&pgmap_array, PHYS_PFN(phys));
    if (pgmap && !percpu_ref_tryget_live_rcu(&pgmap.ref)) {
    pgmap = core::ptr::null_mut();
    }
    rcu_read_unlock();
    return pgmap;
    }
    EXPORT_SYMBOL_GPL(get_dev_pagemap);
#[no_mangle]
pub unsafe extern "C" fn free_zone_device_folio(folio: *mut folio) {
    let mut pgmap = folio.pgmap;
pub static mut nr: c_ulong = 0;
    let mut i = 0;
    if (WARN_ON_ONCE!(!pgmap)) {
    return;
    }
    mem_cgroup_uncharge(folio);
    if (folio_test_anon(folio)) {
    mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON, -1);
    for (i = 0; i < nr; i++) {
    __ClearPageAnonExclusive(folio_page(folio, i));
    }
    }
//
// When a device managed page is freed, the folio->mapping field
// may still contain a (stale) mapping value. For example, the
// lower bits of folio->mapping may still identify the folio as an
// anonymous folio. Ultimately, this entire field is just stale
// and wrong, and it will cause errors if not cleared.
//
// For other types of ZONE_DEVICE pages, migration is either
// handled differently or not done at all, so there is no need
// to clear folio->mapping.
//
// FS DAX pages clear the mapping when the folio->share count hits
// zero which indicating the page has been removed from the file
// system mapping.
//
    if (pgmap.type != MEMORY_DEVICE_FS_DAX &&
    pgmap.type != MEMORY_DEVICE_GENERIC) {
    folio.mapping = core::ptr::null_mut();
    }
    match (pgmap.type) {
    MEMORY_DEVICE_PRIVATE => {
    }
    MEMORY_DEVICE_COHERENT => {
    if (WARN_ON_ONCE!(!pgmap.ops || !pgmap.ops.folio_free)) {
    // break;
    }
    pgmap.ops.folio_free(folio);
    percpu_ref_put_many(&pgmap.ref, nr);
    // break;
    }
    MEMORY_DEVICE_GENERIC => {
//
// Reset the refcount to 1 to prepare for handing out the page
// again.
//
    folio_set_count(folio, 1);
    // break;
    }
    MEMORY_DEVICE_FS_DAX => {
    wake_up_var(&folio.page);
    // break;
    }
    MEMORY_DEVICE_PCI_P2PDMA => {
    if (WARN_ON_ONCE!(!pgmap.ops || !pgmap.ops.folio_free)) {
    // break;
    }
    pgmap.ops.folio_free(folio);
    // break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn zone_device_page_init(page: *mut page, pgmap: *mut dev_pagemap, order: c_uint) {
    let mut new_page = page;
    let mut i = 0;
    VM_WARN_ON_ONCE(order > MAX_ORDER_NR_PAGES);
    while (i < (1UL << order)) {
    let mut new_folio = new_page;
//
// new_page could have been part of previous higher order folio
// which encodes the order, in page + 1, in the flags bits. We
// blindly clear bits which could have set my order field here,
// including page head.
//
    new_page.flags.f &= ~0xffUL;	/* Clear possible order, page head */

//
// This pointer math looks odd, but new_page could have been
// part of a previous higher order folio, which sets _nr_pages
// in page + 1 (new_page). Therefore, we use pointer casting to
// correctly locate the _nr_pages bits within new_page which
// could have modified by previous higher order folio.
//
    ((new_page - 1))._nr_pages = 0;

    new_folio.mapping = core::ptr::null_mut();
    new_folio.pgmap = pgmap;	/* Also clear compound head */
    new_folio.share = 0;   /* fsdax only, unused for device private */
    VM_WARN_ON_FOLIO(folio_ref_count(new_folio), new_folio);
    VM_WARN_ON_FOLIO(!folio_is_zone_device(new_folio), new_folio);
    }
//
// Drivers shouldn't be allocating pages after calling
// memunmap_pages().
//
    WARN_ON_ONCE!(!percpu_ref_tryget_many(&page_pgmap(page).ref, 1 << order));
    set_page_count(page, 1);
    lock_page(page);
    if (order) {
    prep_compound_page(page, order);
    }
    }
    EXPORT_SYMBOL_GPL(zone_device_page_init);