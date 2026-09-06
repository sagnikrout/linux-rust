//! Automatically rewritten from C to Rust
//! Source: mm/show_mem.c
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
// Generic show_mem() implementation
//
// Copyright (C) 2008 Johannes Weiner <hannes@saeurebad.de>
//

    let mut _totalram_pages;
    EXPORT_SYMBOL(_totalram_pages);
    let mut totalreserve_pages = 0;
    let mut totalcma_pages = 0;
#[no_mangle]
pub unsafe extern "C" fn show_node(zone: *mut zone) {
    if (IS_ENABLED!(CONFIG_NUMA)) {
    printk("Node %d ", zone_to_nid(zone));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn si_mem_available() -> c_long {
    let mut available = 0;
    let mut pagecache = 0;
pub static mut wmark_low: c_ulong = 0;
    let mut reclaimable = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_zone(zone) {
    wmark_low += low_wmark_pages(zone);
    }
//
// Estimate the amount of memory available for userspace allocations,
// without causing swapping or OOM.
//
    available = global_zone_page_state(NR_FREE_PAGES) - totalreserve_pages;
//
// Not all the page cache can be freed, otherwise the system will
// start swapping or thrashing. Assume at least half of the page
// cache, or the low watermark worth of cache, needs to stay.
//
    pagecache = global_node_page_state(NR_ACTIVE_FILE) +
    global_node_page_state(NR_INACTIVE_FILE);
    pagecache -= min(pagecache / 2, wmark_low);
    available += pagecache;
//
// Part of the reclaimable slab and other kernel memory consists of
// items that are in use, and cannot be freed. Cap this estimate at the
// low watermark.
//
    reclaimable = global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B) +
    global_node_page_state(NR_KERNEL_MISC_RECLAIMABLE);
    reclaimable -= min(reclaimable / 2, wmark_low);
    available += reclaimable;
    if (available < 0) {
    available = 0;
    }
    return available;
    }
    EXPORT_SYMBOL_GPL(si_mem_available);
#[no_mangle]
pub unsafe extern "C" fn si_meminfo(val: *mut sysinfo) {
    val.totalram = totalram_pages();
    val.sharedram = global_node_page_state(NR_SHMEM);
    val.freeram = global_zone_page_state(NR_FREE_PAGES);
    val.bufferram = nr_blockdev_pages();
    val.totalhigh = totalhigh_pages();
    val.freehigh = nr_free_highpages();
    val.mem_unit = PAGE_SIZE;
    }
    EXPORT_SYMBOL(si_meminfo);

#[no_mangle]
pub unsafe extern "C" fn si_meminfo_node(val: *mut sysinfo, nid: c_int) {
    let mut zone_type = 0;		/* needs to be signed */
pub static mut managed_pages: c_ulong = 0;
pub static mut managed_highpages: c_ulong = 0;
pub static mut free_highpages: c_ulong = 0;
    let mut pgdat = NODE_DATA(nid);
    while (zone_type < MAX_NR_ZONES) {
    let mut zone = &pgdat.node_zones[zone_type];
    managed_pages += zone_managed_pages(zone);
    if (is_highmem(zone)) {
    managed_highpages += zone_managed_pages(zone);
    free_highpages += zone_page_state(zone, NR_FREE_PAGES);
    }
    }
    val.totalram = managed_pages;
    val.sharedram = node_page_state(pgdat, NR_SHMEM);
    val.freeram = sum_zone_node_page_state(nid, NR_FREE_PAGES);
    val.totalhigh = managed_highpages;
    val.freehigh = free_highpages;
    val.mem_unit = PAGE_SIZE;
    }

//
// Determine whether the node should be displayed or not, depending on whether
// SHOW_MEM_FILTER_NODES was passed to show_free_areas().
//
#[no_mangle]
pub unsafe extern "C" fn show_mem_node_skip(flags: c_uint, nid: c_int, nodemask: *mut nodemask_t) -> bool {
    if (!(flags & SHOW_MEM_FILTER_NODES)) {
    return false;
    }
//
// no node mask - aka implicit memory numa policy. Do not bother with
// the synchronization - read_mems_allowed_begin - because we do not
// have to be precise here.
//
    if (!nodemask) {
    nodemask = &cpuset_current_mems_allowed;
    }
    return !node_isset(nid, *nodemask);
    }
#[no_mangle]
unsafe extern "C" fn show_migration_types(type: c_uchar) {
    static const char types[MIGRATE_TYPES] = {
    [MIGRATE_UNMOVABLE]	= 'U',
    [MIGRATE_MOVABLE]	= 'M',
    [MIGRATE_RECLAIMABLE]	= 'E',
    [MIGRATE_HIGHATOMIC]	= 'H',

    [MIGRATE_CMA]		= 'C',

    [MIGRATE_ISOLATE]	= 'I',

    };
    char tmp[MIGRATE_TYPES + 1];
    let mut p = tmp;
    let mut i = 0;
    while (i < MIGRATE_TYPES) {
    if (type & (1 << i)) {
// p++ = types[i];
    }
    }
// p = '\0';
    printk("(%s) ", tmp);
    }
#[no_mangle]
unsafe extern "C" fn node_has_managed_zones(pgdat: *mut pg_data_t, max_zone_idx: c_int) -> bool {
    let mut zone_idx = 0;
    for (zone_idx = 0; zone_idx <= max_zone_idx; zone_idx++) {
    if (zone_managed_pages(pgdat.node_zones + zone_idx))
    return true;
    }
    return false;
    }
//
// Show free area list (used inside shift_scroll-lock stuff)
// We also calculate the percentage fragmentation. We do this by counting the
// memory on each free list with the exception of the first item on the list.
//
// Bits in @filter:
// SHOW_MEM_FILTER_NODES: suppress nodes that are not allowed by current's
// cpuset.
//
#[no_mangle]
pub unsafe extern "C" fn show_free_areas(filter: c_uint, nodemask: *mut nodemask_t, max_zone_idx: c_int) {
pub static mut free_pcp: c_ulong = 0;
    let mut cpu = 0;
    let mut nid = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    if (zone_idx(zone) > max_zone_idx) {
    continue;
    }
    if (show_mem_node_skip(filter, zone_to_nid(zone), nodemask)) {
    continue;
    }
    for_each_online_cpu(cpu) {
    free_pcp += per_cpu_ptr(zone.per_cpu_pageset, cpu).count;
    }
    }
    printk("active_anon:%lu inactive_anon:%lu isolated_anon:%lu\n"
    " active_file:%lu inactive_file:%lu isolated_file:%lu\n"
    " unevictable:%lu dirty:%lu writeback:%lu\n"
    " slab_reclaimable:%lu slab_unreclaimable:%lu\n"
    " mapped:%lu shmem:%lu pagetables:%lu\n"
    " sec_pagetables:%lu bounce:%lu\n"
    " kernel_misc_reclaimable:%lu\n"
    " free:%lu free_pcp:%lu free_cma:%lu\n",
    global_node_page_state(NR_ACTIVE_ANON),
    global_node_page_state(NR_INACTIVE_ANON),
    global_node_page_state(NR_ISOLATED_ANON),
    global_node_page_state(NR_ACTIVE_FILE),
    global_node_page_state(NR_INACTIVE_FILE),
    global_node_page_state(NR_ISOLATED_FILE),
    global_node_page_state(NR_UNEVICTABLE),
    global_node_page_state(NR_FILE_DIRTY),
    global_node_page_state(NR_WRITEBACK),
    global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B),
    global_node_page_state_pages(NR_SLAB_UNRECLAIMABLE_B),
    global_node_page_state(NR_FILE_MAPPED),
    global_node_page_state(NR_SHMEM),
    global_node_page_state(NR_PAGETABLE),
    global_node_page_state(NR_SECONDARY_PAGETABLE),
    0UL,
    global_node_page_state(NR_KERNEL_MISC_RECLAIMABLE),
    global_zone_page_state(NR_FREE_PAGES),
    free_pcp,
    global_zone_page_state(NR_FREE_CMA_PAGES));
    for_each_online_pgdat(pgdat) {
    if (show_mem_node_skip(filter, pgdat.node_id, nodemask)) {
    continue;
    }
    if (!node_has_managed_zones(pgdat, max_zone_idx)) {
    continue;
    }
    printk("Node %d"
    " active_anon:%lukB"
    " inactive_anon:%lukB"
    " active_file:%lukB"
    " inactive_file:%lukB"
    " unevictable:%lukB"
    " isolated(anon):%lukB"
    " isolated(file):%lukB"
    " mapped:%lukB"
    " dirty:%lukB"
    " writeback:%lukB"
    " shmem:%lukB"

    " shmem_thp:%lukB"
    " shmem_pmdmapped:%lukB"
    " anon_thp:%lukB"

    " kernel_stack:%lukB"

    " shadow_call_stack:%lukB"

    " pagetables:%lukB"
    " sec_pagetables:%lukB"
    " all_unreclaimable? %s"
    " Balloon:%lukB"
    " gpu_active:%lukB"
    " gpu_reclaim:%lukB"
    "\n",
    pgdat.node_id,
    K(node_page_state(pgdat, NR_ACTIVE_ANON)),
    K(node_page_state(pgdat, NR_INACTIVE_ANON)),
    K(node_page_state(pgdat, NR_ACTIVE_FILE)),
    K(node_page_state(pgdat, NR_INACTIVE_FILE)),
    K(node_page_state(pgdat, NR_UNEVICTABLE)),
    K(node_page_state(pgdat, NR_ISOLATED_ANON)),
    K(node_page_state(pgdat, NR_ISOLATED_FILE)),
    K(node_page_state(pgdat, NR_FILE_MAPPED)),
    K(node_page_state(pgdat, NR_FILE_DIRTY)),
    K(node_page_state(pgdat, NR_WRITEBACK)),
    K(node_page_state(pgdat, NR_SHMEM)),

    K(node_page_state(pgdat, NR_SHMEM_THPS)),
    K(node_page_state(pgdat, NR_SHMEM_PMDMAPPED)),
    K(node_page_state(pgdat, NR_ANON_THPS)),

    node_page_state(pgdat, NR_KERNEL_STACK_KB),

    node_page_state(pgdat, NR_KERNEL_SCS_KB),

    K(node_page_state(pgdat, NR_PAGETABLE)),
    K(node_page_state(pgdat, NR_SECONDARY_PAGETABLE)),
    str_yes_no(kswapd_test_hopeless(pgdat)),
    K(node_page_state(pgdat, NR_BALLOON_PAGES)),
    K(node_page_state(pgdat, NR_GPU_ACTIVE)),
    K(node_page_state(pgdat, NR_GPU_RECLAIM)));
    }
    for_each_populated_zone(zone) {
    let mut i = 0;
    if (zone_idx(zone) > max_zone_idx) {
    continue;
    }
    if (show_mem_node_skip(filter, zone_to_nid(zone), nodemask)) {
    continue;
    }
    free_pcp = 0;
    for_each_online_cpu(cpu) {
    free_pcp += per_cpu_ptr(zone.per_cpu_pageset, cpu).count;
    }
    show_node(zone);
    printk("%s"
    " free:%lukB"
    " boost:%lukB"
    " min:%lukB"
    " low:%lukB"
    " high:%lukB"
    " reserved_highatomic:%lukB"
    " free_highatomic:%lukB"
    " active_anon:%lukB"
    " inactive_anon:%lukB"
    " active_file:%lukB"
    " inactive_file:%lukB"
    " unevictable:%lukB"
    " writepending:%lukB"
    " zspages:%lukB"
    " present:%lukB"
    " managed:%lukB"
    " mlocked:%lukB"
    " bounce:%lukB"
    " free_pcp:%lukB"
    " local_pcp:%lukB"
    " free_cma:%lukB"
    "\n",
    zone.name,
    K(zone_page_state(zone, NR_FREE_PAGES)),
    K(zone.watermark_boost),
    K(min_wmark_pages(zone)),
    K(low_wmark_pages(zone)),
    K(high_wmark_pages(zone)),
    K(zone.nr_reserved_highatomic),
    K(zone.nr_free_highatomic),
    K(zone_page_state(zone, NR_ZONE_ACTIVE_ANON)),
    K(zone_page_state(zone, NR_ZONE_INACTIVE_ANON)),
    K(zone_page_state(zone, NR_ZONE_ACTIVE_FILE)),
    K(zone_page_state(zone, NR_ZONE_INACTIVE_FILE)),
    K(zone_page_state(zone, NR_ZONE_UNEVICTABLE)),
    K(zone_page_state(zone, NR_ZONE_WRITE_PENDING)),

    K(zone_page_state(zone, NR_ZSPAGES)),

    0UL,

    K(zone.present_pages),
    K(zone_managed_pages(zone)),
    K(zone_page_state(zone, NR_MLOCK)),
    0UL,
    K(free_pcp),
    K((unsigned long)this_cpu_read(zone.per_cpu_pageset.count)),
    K(zone_page_state(zone, NR_FREE_CMA_PAGES)));
    printk("lowmem_reserve[]:");
    for (i = 0; i < MAX_NR_ZONES; i++) {
    printk(" %ld", zone.lowmem_reserve[i]);
    }
    printk("\n");
    }
    for_each_populated_zone(zone) {
    let mut order = 0;
    unsigned long nr[NR_PAGE_ORDERS], flags, total = 0;
    unsigned char types[NR_PAGE_ORDERS];
    if (zone_idx(zone) > max_zone_idx) {
    continue;
    }
    if (show_mem_node_skip(filter, zone_to_nid(zone), nodemask)) {
    continue;
    }
    show_node(zone);
    printk("%s: ", zone.name);
    spin_lock_irqsave(&zone.lock, flags);
    while (order < NR_PAGE_ORDERS) {
    let mut area = &zone.free_area[order];
    let mut type = 0;
    nr[order] = area.nr_free;
    total += nr[order] << order;
    types[order] = 0;
    while (type < MIGRATE_TYPES) {
    if (!free_area_empty(area, type)) {
    types[order] |= 1 << type;
    }
    }
    }
    spin_unlock_irqrestore(&zone.lock, flags);
    while (order < NR_PAGE_ORDERS) {
    printk("%lu*%lukB ",
    nr[order], K(1UL) << order);
    if (nr[order]) {
    show_migration_types(types[order]);
    }
    }
    printk("= %lukB\n", K(total));
    }
    for_each_online_node(nid) {
    if (show_mem_node_skip(filter, nid, nodemask)) {
    continue;
    }
    hugetlb_show_meminfo_node(nid);
    }
    printk("%lu total pagecache pages\n", global_node_page_state(NR_FILE_PAGES));
    show_swap_cache_info();
    }
#[no_mangle]
pub unsafe extern "C" fn __show_mem(filter: c_uint, nodemask: *mut nodemask_t, max_zone_idx: c_int) {
pub static mut total: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    printk("Mem-Info:\n");
    show_free_areas(filter, nodemask, max_zone_idx);
    for_each_populated_zone(zone) {
    total += zone.present_pages;
    reserved += zone.present_pages - zone_managed_pages(zone);
    if (is_highmem(zone)) {
    highmem += zone.present_pages;
    }
    }
    printk("%lu pages RAM\n", total);
    printk("%lu pages HighMem/MovableOnly\n", highmem);
    printk("%lu pages reserved\n", reserved);

    printk("%lu pages cma reserved\n", totalcma_pages);

    printk("%ld pages hwpoisoned\n", atomic_long_read(&num_poisoned_pages));

pub static mut mem_alloc_profiling_spinlock: usize = 0;
    if (spin_trylock(&mem_alloc_profiling_spinlock)) {
    struct codetag_bytes tags[10];
    size_t i, nr;
    nr = alloc_tag_top_users(tags, ARRAY_SIZE!(tags), false);
    if (nr) {
    pr_notice("Memory allocations (profiling is currently turned %s):\n",
    mem_alloc_profiling_enabled() ? "on" : "off");
    while (i < nr) {
    let mut ct = tags[i].ct;
    let mut tag = ct_to_alloc_tag(ct);
pub static mut counter: alloc_tag_counters = 0;
    char bytes[10];
    string_get_size(counter.bytes, 1, STRING_UNITS_2, bytes, sizeof!(bytes));
// Same as alloc_tag_to_text() but w/o intermediate buffer
    if (ct.modname) {
    pr_notice("%12s %8llu %s:%u [%s] func:%s\n",
    bytes, counter.calls, ct.filename,
    ct.lineno, ct.modname, ct.function);
    }
    else {
    pr_notice("%12s %8llu %s:%u func:%s\n",
    bytes, counter.calls, ct.filename,
    ct.lineno, ct.function);
    }
    }
    }
    spin_unlock(&mem_alloc_profiling_spinlock);
    }

    }