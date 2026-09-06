//! Automatically rewritten from C to Rust
//! Source: mm/memory_hotplug.c
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
// linux/mm/memory_hotplug.c
//
// Copyright (C)
//

    enum {
    MEMMAP_ON_MEMORY_DISABLE = 0,
    MEMMAP_ON_MEMORY_ENABLE,
    MEMMAP_ON_MEMORY_FORCE,
    };
pub static mut : int memmap_mode = 0;
#[no_mangle]
pub unsafe extern "C" fn memory_block_memmap_size() -> c_ulong {
    return PHYS_PFN(memory_block_size_bytes()) * sizeof!(page);
    }
#[no_mangle]
pub unsafe extern "C" fn memory_block_memmap_on_memory_pages() -> c_ulong {
pub static mut nr_pages: c_ulong = 0;
//
// In "forced" memmap_on_memory mode, we add extra pages to align the
// vmemmap size to cover full pageblocks. That way, we can add memory
// even if the vmemmap size is not properly aligned, however, we might waste
// memory.
//
    if (memmap_mode == MEMMAP_ON_MEMORY_FORCE) {
    return pageblock_align(nr_pages);
    }
    return nr_pages;
    }

//
// memory_hotplug.memmap_on_memory parameter
//
#[no_mangle]
unsafe extern "C" fn set_memmap_mode(val: *const c_char, kp: *const kernel_param) -> c_int {
    let mut ret = 0;
    let mut mode = 0;
    let mut enabled = 0;
    if (sysfs_streq(val, "force") ||  sysfs_streq(val, "FORCE")) {
    mode = MEMMAP_ON_MEMORY_FORCE;
    } else {
    ret = kstrtobool(val, &enabled);
    if (ret < 0) {
    return ret;
    }
    if (enabled) {
    mode = MEMMAP_ON_MEMORY_ENABLE;
    }
    else {
    mode = MEMMAP_ON_MEMORY_DISABLE;
    }
    }
// (kp->arg) = mode;
    if (mode == MEMMAP_ON_MEMORY_FORCE) {
pub static mut memmap_pages: c_ulong = 0;
    pr_info_once!("Memory hotplug will waste %ld pages in each memory block\n",
    memmap_pages - PFN_UP(memory_block_memmap_size()));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_memmap_mode(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
pub static mut mode: c_int = 0;
    if (mode == MEMMAP_ON_MEMORY_FORCE) {
    return sprintf(buffer, "force\n");
    }
    return sprintf(buffer, "%c\n", mode ? 'Y' : 'N');
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(memmap_on_memory, &memmap_mode_ops, &memmap_mode, 0444);
    MODULE_PARM_DESC(memmap_on_memory, "Enable memmap on memory for memory hotplug\n"
    "With value \"force\" it could result in memory wastage due "
    "to memmap size limitations (Y/N/force)");
#[no_mangle]
pub unsafe extern "C" fn mhp_memmap_on_memory() -> bool {
    return memmap_mode != MEMMAP_ON_MEMORY_DISABLE;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mhp_memmap_on_memory
pub unsafe extern "C" fn mhp_memmap_on_memory_dup() -> bool {
    return false;
    }

    enum {
    ONLINE_POLICY_CONTIG_ZONES = 0,
    ONLINE_POLICY_AUTO_MOVABLE,
    };
    static const char * const online_policy_to_str[] = {
    [ONLINE_POLICY_CONTIG_ZONES] = "contig-zones",
    [ONLINE_POLICY_AUTO_MOVABLE] = "auto-movable",
    };
#[no_mangle]
unsafe extern "C" fn set_online_policy(val: *const c_char, kp: *const kernel_param) -> c_int {
pub static mut ret: c_int = 0;
    if (ret < 0) {
    return ret;
    }
// (kp->arg) = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_online_policy(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    return sprintf(buffer, "%s\n", online_policy_to_str[*(kp.arg)]);
    }
//
// memory_hotplug.online_policy: configure online behavior when onlining without
// specifying a zone (MMOP_ONLINE)
//
// "contig-zones": keep zone contiguous
// "auto-movable": online memory to ZONE_MOVABLE if the configuration
// (auto_movable_ratio, auto_movable_numa_aware) allows for it
//
pub static mut : int online_policy = 0;
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(online_policy, &online_policy_ops, &online_policy, 0644);
    MODULE_PARM_DESC(online_policy,
    "Set the online policy (\"contig-zones\", \"auto-movable\") "
    "Default: \"contig-zones\"");
//
// memory_hotplug.auto_movable_ratio: specify maximum MOVABLE:KERNEL ratio
//
// The ratio represent an upper limit and the kernel might decide to not
// online some memory to ZONE_MOVABLE -- e.g., because hotplugged KERNEL memory
// doesn't allow for more MOVABLE memory.
//
pub static mut : unsigned int auto_movable_ratio = 301;
    module_param!(auto_movable_ratio, uint, 0644);
    MODULE_PARM_DESC(auto_movable_ratio,
    "Set the maximum ratio of MOVABLE:KERNEL memory in the system "
    "in percent for \"auto-movable\" online policy. Default: 301");
//
// memory_hotplug.auto_movable_numa_aware: consider numa node stats
//

pub static mut : bool auto_movable_numa_aware = true;
    module_param!(auto_movable_numa_aware, bool, 0644);
    MODULE_PARM_DESC(auto_movable_numa_aware,
    "Consider numa node stats in addition to global stats in "
    "\"auto-movable\" online policy. Default: true");

//
// online_page_callback contains pointer to current page onlining function.
// Initially it is generic_online_page(). If it is required it could be
// changed by calling set_online_page_callback() for callback registration
// and restore_online_page_callback() for generic callback restore.
//
pub static mut online_page_callback: online_page_callback_t = 0;
pub static mut online_page_callback_lock: usize = 0;
pub static mut mem_hotplug_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn get_online_mems() {
    percpu_down_read(&mem_hotplug_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn put_online_mems() {
    percpu_up_read(&mem_hotplug_lock);
    }
pub static mut movable_node_enabled: bool = false;
pub static mut mhp_default_online_type: int = 0;
#[no_mangle]
pub unsafe extern "C" fn mhp_get_default_online_type() -> enum mmop {
    if (mhp_default_online_type >= 0) {
    return mhp_default_online_type;
    }
    if (IS_ENABLED!(CONFIG_MHP_DEFAULT_ONLINE_TYPE_OFFLINE)) {
    mhp_default_online_type = MMOP_OFFLINE;
    }

    else if (IS_ENABLED!(CONFIG_MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO)) {
    mhp_default_online_type = MMOP_ONLINE;
    }

    else if (IS_ENABLED!(CONFIG_MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL)) {
    mhp_default_online_type = MMOP_ONLINE_KERNEL;
    }

    else if (IS_ENABLED!(CONFIG_MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE)) {
    mhp_default_online_type = MMOP_ONLINE_MOVABLE;
    }
    else {
    mhp_default_online_type = MMOP_OFFLINE;
    }
    return mhp_default_online_type;
    }
    EXPORT_SYMBOL_GPL(mhp_get_default_online_type);
#[no_mangle]
pub unsafe extern "C" fn mhp_set_default_online_type(online_type: mmop) {
    mhp_default_online_type = online_type;
    }
#[no_mangle]
unsafe extern "C" fn setup_memhp_default_state(str: *mut c_char) -> c_int {
pub static mut online_type: c_int = 0;
    if (online_type >= 0) {
    mhp_default_online_type = online_type;
    }
    return 1;
    }
    __setup!("memhp_default_state=", setup_memhp_default_state);
#[no_mangle]
pub unsafe extern "C" fn mem_hotplug_begin() {
    cpus_read_lock();
    percpu_down_write(&mem_hotplug_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn mem_hotplug_done() {
    percpu_up_write(&mem_hotplug_lock);
    cpus_read_unlock();
    }
pub static mut max_mem_size: u64 = 0;
// add this memory to iomem resource
#[no_mangle]
pub unsafe extern "C" fn register_memory_resource(start: u64, size: u64, resource_name: *mut c_char) -> *mut c_void {
pub static mut res: *mut c_void = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    if (strcmp(resource_name, "System RAM")) {
    flags |= IORESOURCE_SYSRAM_DRIVER_MANAGED;
    }
    if (!mhp_range_allowed(start, size, true)) {
    return ERR_PTR(-E2BIG);
    }
//
// Make sure value parsed from 'mem=' only restricts memory adding
// while booting, so that memory hotplug won't be impacted. Please
// refer to document of 'mem=' in kernel-parameters.txt for more
// details.
//
    if (start + size > max_mem_size && system_state < SYSTEM_RUNNING) {
    return ERR_PTR(-E2BIG);
    }
//
// Request ownership of the new memory range.  This might be
// a child of an existing resource that was present but
// not marked as busy.
//
    res = __request_region(&iomem_resource, start, size,
    resource_name, flags);
    if (!res) {
    pr_debug!("Unable to reserve System RAM region: %016llx.%016llx\n",
    start, start + size);
    return ERR_PTR(-EEXIST);
    }
    return res;
    }
#[no_mangle]
unsafe extern "C" fn release_memory_resource(res: *mut resource) {
    if (!res) {
    return;
    }
    release_resource(res);
    kfree(res);
    }
#[no_mangle]
unsafe extern "C" fn check_pfn_span(pfn: c_ulong, nr_pages: c_ulong) -> c_int {
//
// Disallow all operations smaller than a sub-section.
// Note that check_hotplug_memory_range() enforces a larger
// memory_block_size_bytes() granularity for memory that will be marked
// online, so this check should only fire for direct
// arch_{add,remove}_memory() users outside of add_memory_resource().
//
    if (!IS_ALIGNED(pfn | nr_pages, PAGES_PER_SUBSECTION)) {
    return -EINVAL;
    }
    return 0;
    }
//
// Return page for the valid pfn only if the page is online. All pfn
// walkers which rely on the fully initialized page->flags and others
// should use this rather than pfn_valid && pfn_to_page
//
#[no_mangle]
pub unsafe extern "C" fn pfn_to_online_page(pfn: c_ulong) -> *mut c_void {
pub static mut nr: c_ulong = 0;
pub static mut pgmap: *mut c_void = core::ptr::null_mut();
pub static mut ms: *mut c_void = core::ptr::null_mut();
    if (nr >= NR_MEM_SECTIONS) {
    return core::ptr::null_mut();
    }
    ms = __nr_to_section(nr);
    if (!online_section(ms)) {
    return core::ptr::null_mut();
    }
//
// Save some code text when online_section() +
// pfn_section_valid() are sufficient.
//
    if (IS_ENABLED!(CONFIG_HAVE_ARCH_PFN_VALID) && !pfn_valid(pfn)) {
    return core::ptr::null_mut();
    }
    if (!pfn_section_valid(ms, pfn)) {
    return core::ptr::null_mut();
    }
    if (!online_device_section(ms)) {
    return pfn_to_page(pfn);
    }
//
// Slowpath: when ZONE_DEVICE collides with
// ZONE_{NORMAL,MOVABLE} within the same section some pfns in
// the section may be 'offline' but 'valid'. Only
// get_dev_pagemap() can determine sub-section online status.
//
    pgmap = get_dev_pagemap(pfn);
    put_dev_pagemap(pgmap);
// The presence of a pgmap indicates ZONE_DEVICE offline pfn
    if (pgmap) {
    return core::ptr::null_mut();
    }
    return pfn_to_page(pfn);
    }
    EXPORT_SYMBOL_GPL(pfn_to_online_page);
#[no_mangle]
pub unsafe extern "C" fn __add_pages(nid: c_int, pfn: c_ulong, nr_pages: c_ulong, params: *mut mhp_params) -> c_int {
pub static mut end_pfn: c_ulong = 0;
    let mut cur_nr_pages = 0;
    let mut err = 0;
    let mut altmap = params.altmap;
    if (WARN_ON_ONCE!(!pgprot_val(params.pgprot))) {
    return -EINVAL;
    }
    VM_BUG_ON(!mhp_range_allowed(PFN_PHYS(pfn), nr_pages * PAGE_SIZE, false));
    if (altmap) {
//
// Validate altmap is within bounds of the total request
//
    if (altmap.base_pfn != pfn
    || vmem_altmap_offset(altmap) > nr_pages) {
    pr_warn_once("memory add fail, invalid altmap\n");
    return -EINVAL;
    }
    altmap.alloc = 0;
    }
    if (check_pfn_span(pfn, nr_pages)) {
    WARN(1, "Misaligned %s start: %#lx end: %#lx\n", __func__, pfn, pfn + nr_pages - 1);
    return -EINVAL;
    }
    while (pfn < end_pfn) {
// Select all remaining pages up to the next section boundary
    cur_nr_pages = min(end_pfn - pfn,
    SECTION_ALIGN_UP(pfn + 1) - pfn);
    err = sparse_add_section(nid, pfn, cur_nr_pages, altmap,
    params.pgmap);
    if (err) {
    break;
    }
    cond_resched();
    }
    vmemmap_populate_print_last();
    return err;
    }
// find the smallest valid pfn in the range [start_pfn, end_pfn)
#[no_mangle]
pub unsafe extern "C" fn find_smallest_section_pfn(nid: c_int, zone: *mut zone, start_pfn: c_ulong, end_pfn: c_ulong) -> c_ulong {
    while (start_pfn < end_pfn) {
    if (unlikely(!pfn_to_online_page(start_pfn))) {
    continue;
    }
    if (unlikely(pfn_to_nid(start_pfn) != nid)) {
    continue;
    }
    if (zone != page_zone(pfn_to_page(start_pfn))) {
    continue;
    }
    return start_pfn;
    }
    return 0;
    }
// find the biggest valid pfn in the range [start_pfn, end_pfn).
#[no_mangle]
pub unsafe extern "C" fn find_biggest_section_pfn(nid: c_int, zone: *mut zone, start_pfn: c_ulong, end_pfn: c_ulong) -> c_ulong {
    let mut pfn = 0;
// pfn is the end pfn of a memory section.
    pfn = end_pfn - 1;
    while (pfn >= start_pfn) {
    if (unlikely(!pfn_to_online_page(pfn))) {
    continue;
    }
    if (unlikely(pfn_to_nid(pfn) != nid)) {
    continue;
    }
    if (zone != page_zone(pfn_to_page(pfn))) {
    continue;
    }
    return pfn;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn shrink_zone_span(zone: *mut zone, start_pfn: c_ulong, end_pfn: c_ulong) {
    let mut pfn = 0;
pub static mut nid: c_int = 0;
    if (zone.zone_start_pfn == start_pfn) {
//
// If the section is smallest section in the zone, it need
// shrink zone->zone_start_pfn and zone->zone_spanned_pages.
// In this case, we find second smallest valid mem_section
// for shrinking zone.
//
    pfn = find_smallest_section_pfn(nid, zone, end_pfn,
    zone_end_pfn(zone));
    if (pfn) {
    zone.spanned_pages = zone_end_pfn(zone) - pfn;
    zone.zone_start_pfn = pfn;
    } else {
    zone.zone_start_pfn = 0;
    zone.spanned_pages = 0;
    }
    } else if (zone_end_pfn(zone) == end_pfn) {
//
// If the section is biggest section in the zone, it need
// shrink zone->spanned_pages.
// In this case, we find second biggest valid mem_section for
// shrinking zone.
//
    pfn = find_biggest_section_pfn(nid, zone, zone.zone_start_pfn,
    start_pfn);
    if (pfn) {
    zone.spanned_pages = pfn - zone.zone_start_pfn + 1;
    }
    else {
    zone.zone_start_pfn = 0;
    zone.spanned_pages = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn update_pgdat_span(pgdat: *mut pglist_data) {
pub static mut node_start_pfn: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    while (zone < pgdat.node_zones + MAX_NR_ZONES) {
pub static mut end_pfn: c_ulong = 0;
// No need to lock the zones, they can't change.
    if (!zone.spanned_pages) {
    continue;
    }
    if (!node_end_pfn) {
    node_start_pfn = zone.zone_start_pfn;
    node_end_pfn = end_pfn;
    continue;
    }
    if (end_pfn > node_end_pfn) {
    node_end_pfn = end_pfn;
    }
    if (zone.zone_start_pfn < node_start_pfn) {
    node_start_pfn = zone.zone_start_pfn;
    }
    }
    pgdat.node_start_pfn = node_start_pfn;
    pgdat.node_spanned_pages = node_end_pfn - node_start_pfn;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_pfn_range_from_zone(zone: *mut zone, start_pfn: c_ulong, nr_pages: c_ulong) {
pub static mut end_pfn: c_ulong = 0;
    let mut pgdat = zone.zone_pgdat;
    unsigned long pfn, cur_nr_pages;
// Poison struct pages because they are now uninitialized again.
    while (pfn < end_pfn) {
    cond_resched();
// Select all remaining pages up to the next section boundary
    cur_nr_pages =
    min(end_pfn - pfn, SECTION_ALIGN_UP(pfn + 1) - pfn);
    page_init_poison(pfn_to_page(pfn),
    sizeof!(page) * cur_nr_pages);
    }
//
// Zone shrinking code cannot properly deal with ZONE_DEVICE. So
// we will not try to shrink the zones - which is okay as
// set_zone_contiguous() cannot deal with ZONE_DEVICE either way.
//
    if (zone_is_zone_device(zone)) {
    return;
    }
    clear_zone_contiguous(zone);
    shrink_zone_span(zone, start_pfn, start_pfn + nr_pages);
    update_pgdat_span(pgdat);
    set_zone_contiguous(zone);
    }
//
// __remove_pages() - remove sections of pages
// @pfn: starting pageframe (must be aligned to start of a section)
// @nr_pages: number of pages to remove (must be multiple of section size)
// @altmap: alternative device page map or %NULL if default memmap is used
// @pgmap: device page map or %NULL if not ZONE_DEVICE
//
// Generic helper function to remove section mappings and sysfs entries
// for the section of the memory we are removing. Caller needs to make
// sure that pages are marked reserved and zones are adjust properly by
// calling offline_pages().
//
#[no_mangle]
pub unsafe extern "C" fn __remove_pages(pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) {
pub static mut end_pfn: c_ulong = 0;
    let mut cur_nr_pages = 0;
    if (check_pfn_span(pfn, nr_pages)) {
    WARN(1, "Misaligned %s start: %#lx end: %#lx\n", __func__, pfn, pfn + nr_pages - 1);
    return;
    }
    while (pfn < end_pfn) {
    cond_resched();
// Select all remaining pages up to the next section boundary
    cur_nr_pages = min(end_pfn - pfn,
    SECTION_ALIGN_UP(pfn + 1) - pfn);
    sparse_remove_section(pfn, cur_nr_pages, altmap, pgmap);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_online_page_callback(callback: online_page_callback_t) -> c_int {
pub static mut rc: c_int = 0;
    get_online_mems();
    mutex_lock(&online_page_callback_lock);
    if (online_page_callback == generic_online_page) {
    online_page_callback = callback;
    rc = 0;
    }
    mutex_unlock(&online_page_callback_lock);
    put_online_mems();
    return rc;
    }
    EXPORT_SYMBOL_GPL(set_online_page_callback);
#[no_mangle]
pub unsafe extern "C" fn restore_online_page_callback(callback: online_page_callback_t) -> c_int {
pub static mut rc: c_int = 0;
    get_online_mems();
    mutex_lock(&online_page_callback_lock);
    if (online_page_callback == callback) {
    online_page_callback = generic_online_page;
    rc = 0;
    }
    mutex_unlock(&online_page_callback_lock);
    put_online_mems();
    return rc;
    }
    EXPORT_SYMBOL_GPL(restore_online_page_callback);
// we are OK calling __meminit stuff here - we have CONFIG_MEMORY_HOTPLUG
#[no_mangle]
pub unsafe extern "C" fn generic_online_page(page: *mut page, order: c_uint) {
    __free_pages_core(page, order, MEMINIT_HOTPLUG);
    }
    EXPORT_SYMBOL_GPL(generic_online_page);
#[no_mangle]
unsafe extern "C" fn online_pages_range(start_pfn: c_ulong, nr_pages: c_ulong) {
pub static mut end_pfn: c_ulong = 0;
    let mut pfn = 0;
//
// Online the pages in MAX_PAGE_ORDER aligned chunks. The callback might
// decide to not expose all pages to the buddy (e.g., expose them
// later). We account all pages as being online and belonging to this
// zone ("present").
// When using memmap_on_memory, the range might not be aligned to
// MAX_ORDER_NR_PAGES - 1, but pageblock aligned. __ffs() will detect
// this and the first chunk to online will be pageblock_nr_pages.
//
    while (pfn < end_pfn) {
    let mut page = pfn_to_page(pfn);
    let mut order = 0;
//
// Free to online pages in the largest chunks alignment allows.
//
// __ffs() behaviour is undefined for 0. start == 0 is
// MAX_PAGE_ORDER-aligned, Set order to MAX_PAGE_ORDER for
// the case.
//
    if (pfn) {
    order = min_t(int, MAX_PAGE_ORDER, __ffs(pfn));
    }
    else {
    order = MAX_PAGE_ORDER;
    }
//
// Exposing the page to the buddy by freeing can cause
// issues with debug_pagealloc enabled: some archs don't
// like double-unmappings. So treat them like any pages that
// were allocated from the buddy.
//
    debug_pagealloc_map_pages(page, 1 << order);
    (*online_page_callback)(page, order);
    pfn += (1UL << order);
    }
// mark all involved sections as online
    online_mem_sections(start_pfn, end_pfn);
    }
    static void __meminit resize_zone_range(zone *zone, unsigned long start_pfn,
    unsigned long nr_pages)
    {
pub static mut old_end_pfn: c_ulong = 0;
    if (zone_is_empty(zone) || start_pfn < zone.zone_start_pfn) {
    zone.zone_start_pfn = start_pfn;
    }
    zone.spanned_pages = max(start_pfn + nr_pages, old_end_pfn) - zone.zone_start_pfn;
    }
    static void __meminit resize_pgdat_range(pglist_data *pgdat, unsigned long start_pfn,
    unsigned long nr_pages)
    {
pub static mut old_end_pfn: c_ulong = 0;
    if (!pgdat.node_spanned_pages || start_pfn < pgdat.node_start_pfn) {
    pgdat.node_start_pfn = start_pfn;
    }
    pgdat.node_spanned_pages = max(start_pfn + nr_pages, old_end_pfn) - pgdat.node_start_pfn;
    }

#[no_mangle]
unsafe extern "C" fn section_taint_zone_device(pfn: c_ulong) {
    let mut ms = __pfn_to_section(pfn);
    ms.section_mem_map |= SECTION_TAINT_ZONE_DEVICE;
    }

#[no_mangle]
pub unsafe extern "C" fn section_taint_zone_device(pfn: c_ulong) {
    }

//
// Associate the pfn range with the given zone, initializing the memmaps
// and resizing the pgdat/zone data to span the added pages. After this
// call, all affected pages are PageOffline().
//
// All aligned pageblocks are initialized to the specified migratetype
// (usually MIGRATE_MOVABLE). Besides setting the migratetype, no related
// zone stats (e.g., nr_isolate_pageblock) are touched.
//
#[no_mangle]
pub unsafe extern "C" fn move_pfn_range_to_zone(zone: *mut zone, start_pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, migratetype: c_int, isolate_pageblock: bool) {
    let mut pgdat = zone.zone_pgdat;
pub static mut nid: c_int = 0;
    clear_zone_contiguous(zone);
    if (zone_is_empty(zone)) {
    init_currently_empty_zone(zone, start_pfn, nr_pages);
    }
    resize_zone_range(zone, start_pfn, nr_pages);
    resize_pgdat_range(pgdat, start_pfn, nr_pages);
//
// Subsection population requires care in pfn_to_online_page().
// Set the taint to enable the slow path detection of
// ZONE_DEVICE pages in an otherwise  ZONE_{NORMAL,MOVABLE}
// section.
//
    if (zone_is_zone_device(zone)) {
    if (!IS_ALIGNED(start_pfn, PAGES_PER_SECTION)) {
    section_taint_zone_device(start_pfn);
    }
    if (!IS_ALIGNED(start_pfn + nr_pages, PAGES_PER_SECTION)) {
    section_taint_zone_device(start_pfn + nr_pages);
    }
    }
//
// TODO now we have a visible range of pages which are not associated
// with their zone properly. Not nice but set_pfnblock_migratetype()
// expects the zone spans the pfn range. All the pages in the range
// are reserved so nobody should be touching them so we should be safe
//
    memmap_init_range(nr_pages, nid, zone_idx(zone), start_pfn, 0,
    MEMINIT_HOTPLUG, altmap, migratetype,
    isolate_pageblock);
    set_zone_contiguous(zone);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_movable_stats {
    pub kernel_early_pages: c_ulong,
    pub movable_pages: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn auto_movable_stats_account_zone(stats: *mut auto_movable_stats, zone: *mut zone) {
    if (zone_idx(zone) == ZONE_MOVABLE) {
    stats.movable_pages += zone.present_pages;
    } else {
    stats.kernel_early_pages += zone.present_early_pages;

//
// CMA pages (never on hotplugged memory) behave like
// ZONE_MOVABLE.
//
    stats.movable_pages += zone.cma_pages;
    stats.kernel_early_pages -= zone.cma_pages;

    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_movable_group_stats {
    pub movable_pages: c_ulong,
    pub req_kernel_early_pages: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn auto_movable_stats_account_group(group: *mut memory_group, arg: *mut c_void) -> c_int {
pub static mut ratio: c_int = 0;
    let mut stats = arg;
    let mut pages = 0;
//
// We don't support modifying the config while the auto-movable online
// policy is already enabled. Just avoid the division by zero below.
//
    if (!ratio) {
    return 0;
    }
//
// Calculate how many early kernel pages this group requires to
// satisfy the configured zone ratio.
//
    pages = group.present_movable_pages * 100 / ratio;
    pages -= group.present_kernel_pages;
    if (pages > 0) {
    stats.req_kernel_early_pages += pages;
    }
    stats.movable_pages += group.present_movable_pages;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn auto_movable_can_online_movable(nid: c_int, group: *mut memory_group, nr_pages: c_ulong) -> bool {
    unsigned long kernel_early_pages, movable_pages;
pub static mut group_stats: auto_movable_group_stats = 0;
pub static mut stats: auto_movable_stats = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Walk all relevant zones and collect MOVABLE vs. KERNEL stats.
    if (nid == NUMA_NO_NODE) {
// TODO: cache values
    for_each_populated_zone(zone) {
    auto_movable_stats_account_zone(&stats, zone);
    }
    } else {
    while (i < MAX_NR_ZONES) {
    let mut pgdat = NODE_DATA(nid);
    zone = pgdat.node_zones + i;
    if (populated_zone(zone)) {
    auto_movable_stats_account_zone(&stats, zone);
    }
    }
    }
    kernel_early_pages = stats.kernel_early_pages;
    movable_pages = stats.movable_pages;
//
// Kernel memory inside dynamic memory group allows for more MOVABLE
// memory within the same group. Remove the effect of all but the
// current group from the stats.
//
    walk_dynamic_memory_groups(nid, auto_movable_stats_account_group,
    group, &group_stats);
    if (kernel_early_pages <= group_stats.req_kernel_early_pages) {
    return false;
    }
    kernel_early_pages -= group_stats.req_kernel_early_pages;
    movable_pages -= group_stats.movable_pages;
    if (group && group.is_dynamic) {
    kernel_early_pages += group.present_kernel_pages;
    }
//
// Test if we could online the given number of pages to ZONE_MOVABLE
// and still stay in the configured ratio.
//
    movable_pages += nr_pages;
    return movable_pages <= (auto_movable_ratio * kernel_early_pages) / 100;
    }
//
// Returns a default kernel memory zone for the given pfn range.
// If no kernel zone covers this pfn range it will automatically go
// to the ZONE_NORMAL.
//
#[no_mangle]
pub unsafe extern "C" fn default_kernel_zone_for_pfn(nid: c_int, start_pfn: c_ulong, nr_pages: c_ulong) -> *mut c_void {
    let mut pgdat = NODE_DATA(nid);
    let mut zid = 0;
    while (zid < ZONE_NORMAL) {
    let mut zone = &pgdat.node_zones[zid];
    if (zone_intersects(zone, start_pfn, nr_pages)) {
    return zone;
    }
    }
    return &pgdat.node_zones[ZONE_NORMAL];
    }
//
// Determine to which zone to online memory dynamically based on user
// configuration and system stats. We care about the following ratio:
//
// MOVABLE : KERNEL
//
// Whereby MOVABLE is memory in ZONE_MOVABLE and KERNEL is memory in
// one of the kernel zones. CMA pages inside one of the kernel zones really
// behaves like ZONE_MOVABLE, so we treat them accordingly.
//
// We don't allow for hotplugged memory in a KERNEL zone to increase the
// amount of MOVABLE memory we can have, so we end up with:
//
// MOVABLE : KERNEL_EARLY
//
// Whereby KERNEL_EARLY is memory in one of the kernel zones, available since
// boot. We base our calculation on KERNEL_EARLY internally, because:
//
// a) Hotplugged memory in one of the kernel zones can sometimes still get
// hotunplugged, especially when hot(un)plugging individual memory blocks.
// There is no coordination across memory devices, therefore "automatic"
// hotunplugging, as implemented in hypervisors, could result in zone
// imbalances.
// b) Early/boot memory in one of the kernel zones can usually not get
// hotunplugged again (e.g., no firmware interface to unplug, fragmented
// with unmovable allocations). While there are corner cases where it might
// still work, it is barely relevant in practice.
//
// Exceptions are dynamic memory groups, which allow for more MOVABLE
// memory within the same memory group -- because in that case, there is
// coordination within the single memory device managed by a single driver.
//
// We rely on "present pages" instead of "managed pages", as the latter is
// highly unreliable and dynamic in virtualized environments, and does not
// consider boot time allocations. For example, memory ballooning adjusts the
// managed pages when inflating/deflating the balloon, and balloon page
// migration can even migrate inflated pages between zones.
//
// Using "present pages" is better but some things to keep in mind are:
//
// a) Some memblock allocations, such as for the crashkernel area, are
// effectively unused by the kernel, yet they account to "present pages".
// Fortunately, these allocations are comparatively small in relevant setups
// (e.g., fraction of system memory).
// b) Some hotplugged memory blocks in virtualized environments, especially
// hotplugged by virtio-mem, look like they are completely present, however,
// only parts of the memory block are actually currently usable.
// "present pages" is an upper limit that can get reached at runtime. As
// we base our calculations on KERNEL_EARLY, this is not an issue.
//
#[no_mangle]
pub unsafe extern "C" fn auto_movable_zone_for_pfn(nid: c_int, group: *mut memory_group, pfn: c_ulong, nr_pages: c_ulong) -> *mut c_void {
pub static mut online_pages: c_ulong = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!auto_movable_ratio) {
// goto;
    }
    if (group && !group.is_dynamic) {
    max_pages = group.s.max_pages;
    online_pages = group.present_movable_pages;
// If anything is !MOVABLE online the rest !MOVABLE.
    if (group.present_kernel_pages) {
// goto;
    }
    } else if (!group || group.d.unit_pages == nr_pages) {
    max_pages = nr_pages;
    } else {
    max_pages = group.d.unit_pages;
//
// Take a look at all online sections in the current unit.
// We can safely assume that all pages within a section belong
// to the same zone, because dynamic memory groups only deal
// with hotplugged memory.
//
    pfn = ALIGN_DOWN(pfn, group.d.unit_pages);
    end_pfn = pfn + group.d.unit_pages;
    while (pfn < end_pfn) {
    page = pfn_to_online_page(pfn);
    if (!page) {
    continue;
    }
// If anything is !MOVABLE online the rest !MOVABLE.
    if (!is_zone_movable_page(page)) {
// goto;
    }
    online_pages += PAGES_PER_SECTION;
    }
    }
//
// Online MOVABLE if we could *currently* online all remaining parts
// MOVABLE. We expect to (add+) online them immediately next, so if
// nobody interferes, all will be MOVABLE if possible.
//
    nr_pages = max_pages - online_pages;
    if (!auto_movable_can_online_movable(NUMA_NO_NODE, group, nr_pages)) {
// goto;
    }

    if (auto_movable_numa_aware &&
    !auto_movable_can_online_movable(nid, group, nr_pages)) {
// goto;
    }

    return &NODE_DATA(nid).node_zones[ZONE_MOVABLE];
// label;
    return default_kernel_zone_for_pfn(nid, pfn, nr_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn default_zone_for_pfn(nid: c_int, start_pfn: c_ulong, nr_pages: c_ulong) -> *mut c_void {
    let mut kernel_zone = default_kernel_zone_for_pfn(nid, start_pfn,
    nr_pages);
    let mut movable_zone = &NODE_DATA(nid).node_zones[ZONE_MOVABLE];
pub static mut in_kernel: bool = false;
pub static mut in_movable: bool = false;
//
// We inherit the existing zone in a simple case where zones do not
// overlap in the given range
//
    if (in_kernel ^ in_movable) {
    return (in_kernel) ? kernel_zone : movable_zone;
    }
//
// If the range doesn't belong to any zone or two zones overlap in the
// given range then we use movable zone only if movable_node is
// enabled because we always online to a kernel zone by default.
//
    return movable_node_enabled ? movable_zone : kernel_zone;
    }
#[no_mangle]
pub unsafe extern "C" fn zone_for_pfn_range(online_type: mmop, nid: c_int, group: *mut memory_group, start_pfn: c_ulong, nr_pages: c_ulong) -> *mut c_void {
    if (online_type == MMOP_ONLINE_KERNEL) {
    return default_kernel_zone_for_pfn(nid, start_pfn, nr_pages);
    }
    if (online_type == MMOP_ONLINE_MOVABLE) {
    return &NODE_DATA(nid).node_zones[ZONE_MOVABLE];
    }
    if (online_policy == ONLINE_POLICY_AUTO_MOVABLE) {
    return auto_movable_zone_for_pfn(nid, group, start_pfn, nr_pages);
    }
    return default_zone_for_pfn(nid, start_pfn, nr_pages);
    }
//
// This function should only be called by memory_block_{online,offline},
// and {online,offline}_pages.
//
#[no_mangle]
pub unsafe extern "C" fn adjust_present_page_count(page: *mut page, group: *mut memory_group, nr_pages: c_long) {
    let mut zone = page_zone(page);
pub static mut movable: bool = false;
//
// We only support onlining/offlining/adding/removing of complete
// memory blocks; therefore, either all is either early or hotplugged.
//
    if (early_section(__pfn_to_section(page_to_pfn(page)))) {
    zone.present_early_pages += nr_pages;
    }
    zone.present_pages += nr_pages;
    zone.zone_pgdat.node_present_pages += nr_pages;
    if (group && movable) {
    group.present_movable_pages += nr_pages;
    }

    else if (group && !movable) {
    group.present_kernel_pages += nr_pages;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mhp_init_memmap_on_memory(pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone) -> c_int {
pub static mut end_pfn: c_ulong = 0;
    let mut ret = 0;
    let mut i = 0;
    ret = kasan_add_zero_shadow(__va(PFN_PHYS(pfn)), PFN_PHYS(nr_pages));
    if (ret) {
    return ret;
    }
    move_pfn_range_to_zone(zone, pfn, nr_pages, core::ptr::null_mut(), MIGRATE_UNMOVABLE,
    false);
    while (i < nr_pages) {
    let mut page = pfn_to_page(pfn + i);
    __ClearPageOffline(page);
    SetPageVmemmapSelfHosted(page);
    }
//
// It might be that the vmemmap_pages fully span sections. If that is
// the case, mark those sections online here as otherwise they will be
// left offline.
//
    if (nr_pages >= PAGES_PER_SECTION) {
    online_mem_sections(pfn, ALIGN_DOWN(end_pfn, PAGES_PER_SECTION));
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mhp_deinit_memmap_on_memory(pfn: c_ulong, nr_pages: c_ulong) {
pub static mut end_pfn: c_ulong = 0;
//
// It might be that the vmemmap_pages fully span sections. If that is
// the case, mark those sections offline here as otherwise they will be
// left online.
//
    if (nr_pages >= PAGES_PER_SECTION) {
    offline_mem_sections(pfn, ALIGN_DOWN(end_pfn, PAGES_PER_SECTION));
    }
//
// The pages associated with this vmemmap have been offlined, so
// we can reset its state here.
//
    remove_pfn_range_from_zone(page_zone(pfn_to_page(pfn)), pfn, nr_pages);
    kasan_remove_zero_shadow(__va(PFN_PHYS(pfn)), PFN_PHYS(nr_pages));
    }
//
// Must be called with mem_hotplug_lock in write mode.
//
#[no_mangle]
pub unsafe extern "C" fn online_pages(pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone, group: *mut memory_group) -> c_int {
pub static mut memory_notify: usize = 0;
pub static mut node_notify: usize = 0;
pub static mut nid: c_int = 0;
pub static mut need_zonelists_rebuild: c_int = 0;
    let mut flags = 0;
    let mut ret = 0;
//
// {on,off}lining is constrained to full memory sections (or more
// precisely to memory blocks from the user space POV).
// memmap_on_memory is an exception because it reserves initial part
// of the physical memory space for vmemmaps. That space is pageblock
// aligned.
//
    if (WARN_ON_ONCE!(!nr_pages || !pageblock_aligned(pfn) ||
    !IS_ALIGNED(pfn + nr_pages, PAGES_PER_SECTION))) {
    return -EINVAL;
    }
// associate pfn range with the zone
    move_pfn_range_to_zone(zone, pfn, nr_pages, core::ptr::null_mut(), MIGRATE_MOVABLE,
    true);
    if (!node_state(nid, N_MEMORY)) {
// Adding memory to the node for the first time
    node_arg.nid = nid;
    ret = node_notify(NODE_ADDING_FIRST_MEMORY, &node_arg);
    ret = notifier_to_errno(ret);
    if (ret) {
// goto;
    }
    }
    ret = memory_notify(MEM_GOING_ONLINE, &mem_arg);
    ret = notifier_to_errno(ret);
    if (ret) {
// goto;
    }
//
// Fixup the number of isolated pageblocks before marking the sections
// onlining, such that undo_isolate_page_range() works correctly.
//
    spin_lock_irqsave(&zone.lock, flags);
    zone.nr_isolate_pageblock += nr_pages / pageblock_nr_pages;
    spin_unlock_irqrestore(&zone.lock, flags);
//
// If this zone is not populated, then it is not in zonelist.
// This means the page allocator ignores this zone.
// So, zonelist must be updated after online.
//
    if (!populated_zone(zone)) {
    need_zonelists_rebuild = 1;
    setup_zone_pageset(zone);
    }
    online_pages_range(pfn, nr_pages);
    adjust_present_page_count(pfn_to_page(pfn), group, nr_pages);
    if (node_arg.nid >= 0) {
    node_set_state(nid, N_MEMORY);
    }
//
// Check whether we are adding normal memory to the node for the first
// time.
//
    if (!node_state(nid, N_NORMAL_MEMORY) && zone_idx(zone) <= ZONE_NORMAL) {
    node_set_state(nid, N_NORMAL_MEMORY);
    }
    if (need_zonelists_rebuild) {
    build_all_zonelists(core::ptr::null_mut());
    }
// Basic onlining is complete, allow allocation of onlined pages.
    undo_isolate_page_range(pfn, pfn + nr_pages);
//
// Freshly onlined pages aren't shuffled (e.g., all pages are placed to
// the tail of the freelist when undoing isolation). Shuffle the whole
// zone to make sure the just onlined pages are properly distributed
// across the whole freelist - to create an initial shuffle.
//
    shuffle_zone(zone);
// reinitialise watermarks and update pcp limits
    init_per_zone_wmark_min();
    kswapd_run(nid);
    kcompactd_run(nid);
    if (node_arg.nid >= 0) {
// First memory added successfully. Notify consumers.
    node_notify(NODE_ADDED_FIRST_MEMORY, &node_arg);
    }
    writeback_set_ratelimit();
    memory_notify(MEM_ONLINE, &mem_arg);
    return 0;
// label;
    pr_debug!("online_pages [mem %#010llx-%#010llx] failed\n",
    (unsigned long long) pfn << PAGE_SHIFT,
    (((unsigned long long) pfn + nr_pages) << PAGE_SHIFT) - 1);
    memory_notify(MEM_CANCEL_ONLINE, &mem_arg);
    if (node_arg.nid != NUMA_NO_NODE) {
    node_notify(NODE_CANCEL_ADDING_FIRST_MEMORY, &node_arg);
    }
    remove_pfn_range_from_zone(zone, pfn, nr_pages);
    return ret;
    }
// we are OK calling __meminit stuff here - we have CONFIG_MEMORY_HOTPLUG
    static pg_data_t *hotadd_init_pgdat(int nid)
    {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
//
// NODE_DATA is preallocated (free_area_init) but its internal
// state is not allocated completely. Add missing pieces.
// Completely offline nodes stay around and they just need
// reinitialization.
//
    pgdat = NODE_DATA(nid);
// init node's zones as empty zones, we don't have any present pages.
    if (free_area_init_core_hotplug(pgdat)) {
    return core::ptr::null_mut();
    }
//
// The node we allocated has no zone fallback lists. For avoiding
// to access not-initialized zonelist, build here.
//
    build_all_zonelists(pgdat);
    return pgdat;
    }
//
// __try_online_node - online a node if offlined
// @nid: the node ID
// @set_node_online: Whether we want to online the node
// called by cpu_up() to online a node without onlined memory.
//
// Returns:
// 1 -> a new node has been allocated
// 0 -> the node is already online
// -ENOMEM -> the node could not be allocated
//
#[no_mangle]
unsafe extern "C" fn __try_online_node(nid: c_int, set_node_online: bool) -> c_int {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 1;
    if (node_online(nid)) {
    return 0;
    }
    pgdat = hotadd_init_pgdat(nid);
    if (!pgdat) {
    pr_err!("Cannot online node %d due to core::ptr::null_mut() pgdat\n", nid);
    ret = -ENOMEM;
// goto;
    }
    if (set_node_online) {
    node_set_online(nid);
    ret = register_node(nid);
    BUG_ON!(ret);
    }
// label;
    return ret;
    }
//
// Users of this function always want to online/register the node
//
#[no_mangle]
pub unsafe extern "C" fn try_online_node(nid: c_int) -> c_int {
    let mut ret = 0;
    mem_hotplug_begin();
    ret =  __try_online_node(nid, true);
    mem_hotplug_done();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_hotplug_memory_range(start: u64, size: u64) -> c_int {
// memory range must be block size aligned
    if (!size || !IS_ALIGNED(start, memory_block_size_bytes()) ||
    !IS_ALIGNED(size, memory_block_size_bytes())) {
    pr_err!("Block size [%#lx] unaligned hotplug range: start %#llx, size %#llx",
    memory_block_size_bytes(), start, size);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn online_memory_block(mem: *mut memory_block, arg: *mut c_void) -> c_int {
    enum mmop *online_type = arg;
    mem.online_type = *online_type;
    return device_online(&mem.dev);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_supports_memmap_on_memory(vmemmap_size: c_ulong) -> bool {
//
// As default, we want the vmemmap to span a complete PMD such that we
// can map the vmemmap using a single PMD if supported by the
// architecture.
//
    return IS_ALIGNED(vmemmap_size, PMD_SIZE);
    }

#[no_mangle]
pub unsafe extern "C" fn mhp_supports_memmap_on_memory() -> bool {
pub static mut vmemmap_size: c_ulong = 0;
pub static mut memmap_pages: c_ulong = 0;
//
// Besides having arch support and the feature enabled at runtime, we
// need a few more assumptions to hold true:
//
// a) The vmemmap pages span complete PMDs: We don't want vmemmap code
// to populate memory from the altmap for unrelated parts (i.e.,
other memory blocks)
//
// b) The vmemmap pages (and thereby the pages that will be exposed to
// the buddy) have to cover full pageblocks: memory onlining/offlining
// code requires applicable ranges to be page-aligned, for example, to
// set the migratetypes properly.
//
// TODO: Although we have a check here to make sure that vmemmap pages
// fully populate a PMD, it is not the right place to check for
// this. A much better solution involves improving vmemmap code
// to fallback to base pages when trying to populate vmemmap using
// altmap as an alternative source of memory, and we do not exactly
// populate a single PMD.
//
    if (!mhp_memmap_on_memory()) {
    return false;
    }
//
// Make sure the vmemmap allocation is fully contained
// so that we always allocate vmemmap memory from altmap area.
//
    if (!IS_ALIGNED(vmemmap_size, PAGE_SIZE)) {
    return false;
    }
//
// start pfn should be pageblock_nr_pages aligned for correctly
// setting migrate types
//
    if (!pageblock_aligned(memmap_pages)) {
    return false;
    }
    if (memmap_pages == PHYS_PFN(memory_block_size_bytes())) {
// No effective hotplugged memory doesn't make sense.
    return false;
    }
    return arch_supports_memmap_on_memory(vmemmap_size);
    }
    EXPORT_SYMBOL_GPL(mhp_supports_memmap_on_memory);
#[no_mangle]
unsafe extern "C" fn altmap_free(altmap: *mut vmem_altmap) {
    WARN_ONCE(altmap.alloc, "Altmap not fully unmapped");
    kfree(altmap);
    }
#[no_mangle]
unsafe extern "C" fn remove_memory_blocks_and_altmaps(start: u64, size: u64) {
pub static mut memblock_size: c_ulong = 0;
    let mut cur_start = 0;
//
// For memmap_on_memory, the altmaps were added on a per-memblock
// basis; we have to process each individual memory block.
//
    while (cur_start < start + size) {
    let mut altmap = core::ptr::null_mut();
pub static mut mem: *mut c_void = core::ptr::null_mut();
    mem = memory_block_get(phys_to_block_id(cur_start));
    if (WARN_ON_ONCE!(!mem)) {
    continue;
    }
    altmap = mem.altmap;
    mem.altmap = core::ptr::null_mut();
    memory_block_put(mem);
    remove_memory_block_devices(cur_start, memblock_size);
    arch_remove_memory(cur_start, memblock_size, altmap, core::ptr::null_mut());
    altmap_free(altmap);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn create_altmaps_and_memory_blocks(nid: c_int, group: *mut memory_group, start: u64, size: u64) -> c_int {
pub static mut memblock_size: c_ulong = 0;
    let mut cur_start = 0;
    let mut ret = 0;
    while (cur_start < start + size) {
pub static mut mhp_params: usize = 0;
pub static mut vmem_altmap: usize = 0;
    mhp_altmap.free = memory_block_memmap_on_memory_pages();
    params.altmap = kmemdup(&mhp_altmap, sizeof!(vmem_altmap),
    GFP_KERNEL);
    if (!params.altmap) {
    ret = -ENOMEM;
// goto;
    }
// call arch's memory hotadd
    ret = arch_add_memory(nid, cur_start, memblock_size, &params);
    if (ret < 0) {
    altmap_free(params.altmap);
// goto;
    }
// create memory block devices after memory was added
    ret = create_memory_block_devices(cur_start, memblock_size, nid,
    params.altmap, group);
    if (ret) {
    arch_remove_memory(cur_start, memblock_size, params.altmap, core::ptr::null_mut());
    altmap_free(params.altmap);
// goto;
    }
    }
    return 0;
// label;
    if (ret && cur_start != start) {
    remove_memory_blocks_and_altmaps(start, cur_start - start);
    }
    return ret;
    }
//
// NOTE: The caller must call lock_device_hotplug() to serialize hotplug
// and online/offline operations (triggered e.g. by sysfs).
//
// we are OK calling __meminit stuff here - we have CONFIG_MEMORY_HOTPLUG
//
#[no_mangle]
pub unsafe extern "C" fn __add_memory_resource(nid: c_int, res: *mut resource, mhp_flags: mhp_t, online_type: mmop) -> c_int {
pub static mut params: mhp_params = 0;
pub static mut memblock_flags: memblock_flags = 0;
    let mut group = core::ptr::null_mut();
    u64 start, size;
pub static mut new_node: bool = false;
    let mut ret = 0;
    start = res.start;
    size = resource_size(res);
    ret = check_hotplug_memory_range(start, size);
    if (ret) {
    return ret;
    }
    if (mhp_flags & MHP_NID_IS_MGID) {
    group = memory_group_find_by_id(nid);
    if (!group) {
    return -EINVAL;
    }
    nid = group.nid;
    }
    if (!node_possible(nid)) {
    WARN(1, "node %d was absent from the node_possible_map\n", nid);
    return -EINVAL;
    }
    mem_hotplug_begin();
    if (IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    if (res.flags & IORESOURCE_SYSRAM_DRIVER_MANAGED) {
    memblock_flags = MEMBLOCK_DRIVER_MANAGED;
    }
    ret = memblock_add_node(start, size, nid, memblock_flags);
    if (ret) {
// goto;
    }
    }
    ret = __try_online_node(nid, false);
    if (ret < 0) {
// goto;
    }
    if (ret) {
    node_set_online(nid);
    ret = register_node(nid);
    if (WARN_ON!(ret)) {
    node_set_offline(nid);
// goto;
    }
    new_node = true;
    }
//
// Self hosted memmap array
//
    if ((mhp_flags & MHP_MEMMAP_ON_MEMORY) &&
    mhp_supports_memmap_on_memory()) {
    ret = create_altmaps_and_memory_blocks(nid, group, start, size);
    if (ret) {
// goto;
    }
    } else {
    ret = arch_add_memory(nid, start, size, &params);
    if (ret < 0) {
// goto;
    }
// create memory block devices after memory was added
    ret = create_memory_block_devices(start, size, nid, core::ptr::null_mut(), group);
    if (ret) {
    arch_remove_memory(start, size, params.altmap, core::ptr::null_mut());
// goto;
    }
    }
    register_memory_blocks_under_node_hotplug(nid, PFN_DOWN(start),
    PFN_UP(start + size - 1));
// create new memmap entry
    if (!strcmp(res.name, "System RAM")) {
    firmware_map_add_hotplug(start, start + size, "System RAM");
    }
// device_online() will take the lock when calling online_pages()
    mem_hotplug_done();
//
// In case we're allowed to merge the resource, flag it and trigger
// merging now that adding succeeded.
//
    if (mhp_flags & MHP_MERGE_RESOURCE) {
    merge_system_ram_resource(res);
    }
// online pages if requested
    if (online_type != MMOP_OFFLINE) {
    walk_memory_blocks(start, size, &online_type,
    online_memory_block);
    }
    return ret;
// label;
    if (new_node) {
    node_set_offline(nid);
    unregister_node(nid);
    }
// label;
    if (IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    memblock_remove(start, size);
    }
// label;
    mem_hotplug_done();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn add_memory_resource(nid: c_int, res: *mut resource, mhp_flags: mhp_t) -> c_int {
    return __add_memory_resource(nid, res, mhp_flags,
    mhp_get_default_online_type());
    }
// requires device_hotplug_lock, see __add_memory_resource()
#[no_mangle]
pub unsafe extern "C" fn __add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int {
pub static mut res: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    res = register_memory_resource(start, size, "System RAM");
    if (IS_ERR(res)) {
    return PTR_ERR(res);
    }
    ret = add_memory_resource(nid, res, mhp_flags);
    if (ret < 0) {
    release_memory_resource(res);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int {
    let mut rc = 0;
    lock_device_hotplug();
    rc = __add_memory(nid, start, size, mhp_flags);
    unlock_device_hotplug();
    return rc;
    }
    EXPORT_SYMBOL_GPL(add_memory);
//
// __add_memory_driver_managed - add driver-managed memory with explicit online_type
// @nid: NUMA node ID where the memory will be added
// @start: Start physical address of the memory range
// @size: Size of the memory range in bytes
// @resource_name: Resource name in format "System RAM ($DRIVER)"
// @mhp_flags: Memory hotplug flags
// @online_type: Auto-Online behavior (offline, online, kernel, movable)
//
// Add special, driver-managed memory to the system as system RAM. Such
// memory is not exposed via the raw firmware-provided memmap as system
// RAM, instead, it is detected and added by a driver - during cold boot,
// after a reboot, and after kexec.
//
// Reasons why this memory should not be used for the initial memmap of a
// kexec kernel or for placing kexec images:
//
// - The booting kernel is in charge of determining how this memory will be
// used (e.g., use persistent memory as system RAM)
// - Coordination with a hypervisor is required before this memory
// can be used (e.g., inaccessible parts).
//
// For this memory, no entries in /sys/firmware/memmap ("raw firmware-provided
// memory map") are created. Also, the created memory resource is flagged
// with IORESOURCE_SYSRAM_DRIVER_MANAGED, so in-kernel users can special-case
// this memory as well (esp., not place kexec images onto it).
//
// The resource_name (visible via /proc/iomem) has to have the format
// "System RAM ($DRIVER)".
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn __add_memory_driver_managed(nid: c_int, start: u64, size: u64, resource_name: *mut c_char, mhp_flags: mhp_t, online_type: mmop) -> c_int {
pub static mut res: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    if (!resource_name ||
    strstr(resource_name, "System RAM (") != resource_name ||
    resource_name[strlen(resource_name) - 1] != ')') {
    return -EINVAL;
    }
    if (online_type < MMOP_OFFLINE || online_type > MMOP_ONLINE_MOVABLE) {
    return -EINVAL;
    }
    lock_device_hotplug();
    res = register_memory_resource(start, size, resource_name);
    if (IS_ERR(res)) {
    rc = PTR_ERR(res);
// goto;
    }
    rc = __add_memory_resource(nid, res, mhp_flags, online_type);
    if (rc < 0) {
    release_memory_resource(res);
    }
// label;
    unlock_device_hotplug();
    return rc;
    }
    EXPORT_SYMBOL_FOR_MODULES(__add_memory_driver_managed, "kmem");
//
// add_memory_driver_managed - add driver-managed memory
// @nid: NUMA node ID where the memory will be added
// @start: Start physical address of the memory range
// @size: Size of the memory range in bytes
// @resource_name: Resource name in format "System RAM ($DRIVER)"
// @mhp_flags: Memory hotplug flags
//
// Add driver-managed memory with the system default online type set by
// build config or kernel boot parameter.
//
// See __add_memory_driver_managed for more details.
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn add_memory_driver_managed(nid: c_int, start: u64, size: u64, resource_name: *mut c_char, mhp_flags: mhp_t) -> c_int {
    return __add_memory_driver_managed(nid, start, size, resource_name,
    mhp_flags,
    mhp_get_default_online_type());
    }
    EXPORT_SYMBOL_GPL(add_memory_driver_managed);
//
// Platforms should define arch_get_mappable_range() that provides
// maximum possible addressable physical memory range for which the
// linear mapping could be created. The platform returned address
// range must adhere to these following semantics.
//
// - range.start <= range.end
// - Range includes both end points [range.start..range.end]
//
// There is also a fallback definition provided here, allowing the
// entire possible physical address range in case any platform does
// not define arch_get_mappable_range().
//
#[no_mangle]
pub unsafe extern "C" fn arch_get_mappable_range() -> range __weak {
pub static mut range: usize = 0;
    return mhp_range;
    }
#[no_mangle]
pub unsafe extern "C" fn mhp_get_pluggable_range(need_mapping: bool) -> range {
pub static mut max_phys: u64 = 0;
pub static mut mhp_range: usize = 0;
    if (need_mapping) {
    mhp_range = arch_get_mappable_range();
    if (mhp_range.start > max_phys) {
    mhp_range.start = 0;
    mhp_range.end = 0;
    }
    mhp_range.end = min_t(u64, mhp_range.end, max_phys);
    } else {
    mhp_range.start = 0;
    mhp_range.end = max_phys;
    }
    return mhp_range;
    }
    EXPORT_SYMBOL_GPL(mhp_get_pluggable_range);
#[no_mangle]
pub unsafe extern "C" fn mhp_range_allowed(start: u64, size: u64, need_mapping: bool) -> bool {
pub static mut mhp_range: range = 0;
pub static mut end: u64 = 0;
    if (start < end && start >= mhp_range.start && (end - 1) <= mhp_range.end) {
    return true;
    }
    pr_warn!("Hotplug memory [%#llx-%#llx] exceeds maximum addressable range [%#llx-%#llx]\n",
    start, end, mhp_range.start, mhp_range.end);
    return false;
    }

//
// Scan pfn range [start,end) to find movable/migratable pages (LRU and
// hugetlb folio, movable_ops pages). Will skip over most unmovable
// pages (esp., pages that can be skipped when offlining), but bail out on
// definitely unmovable pages.
//
// Returns:
// 0 in case a movable page is found and movable_pfn was updated.
// -ENOENT in case no movable page was found.
// -EBUSY in case a definitely unmovable page was found.
//
#[no_mangle]
pub unsafe extern "C" fn scan_movable_pages(start: c_ulong, end: c_ulong, movable_pfn: *mut c_ulong) -> c_int {
    let mut pfn = 0;
    while (pfn < end) {
    let mut nr_pages = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    page = pfn_to_page(pfn);
    if (PageLRU(page) || page_has_movable_ops(page)) {
// goto;
    }
//
// PageOffline() pages that do not have movable_ops and
// have a reference count > 0 (after MEM_GOING_OFFLINE) are
// definitely unmovable. If their reference count would be 0,
// they could at least be skipped when offlining memory.
//
    if (PageOffline(page) && page_count(page)) {
    return -EBUSY;
    }
    folio = page_folio(page);
    if (!folio_test_hugetlb(folio)) {
    continue;
    }
//
// This test is racy as we hold no reference or lock.  The
// hugetlb page could have been free'ed and head is no longer
// a hugetlb page before the following check.  In such unlikely
// cases false positives and negatives are possible.  Calling
// code must deal with these scenarios.
//
    if (folio_test_hugetlb_migratable(folio)) {
// goto;
    }
    nr_pages = folio_nr_pages(folio);
    if (unlikely(nr_pages < 1 || nr_pages > MAX_FOLIO_NR_PAGES ||
    !is_power_of_2(nr_pages))) {
    continue;
    }
    pfn |= nr_pages - 1;
    }
    return -ENOENT;
// label;
// movable_pfn = pfn;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_migrate_range(start_pfn: c_ulong, end_pfn: c_ulong) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut pfn = 0;
pub static mut source: usize = 0;
pub static mut migrate_rs: usize = 0;
    while (pfn < end_pfn) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pfn_to_page(pfn);
    folio = page_folio(page);
    if (!folio_try_get(folio)) {
    continue;
    }
    if (unlikely(page_folio(page) != folio)) {
// goto;
    }
    if (folio_test_large(folio)) {
    pfn = folio_pfn(folio) + folio_nr_pages(folio) - 1;
    }
    if (folio_contain_hwpoisoned_page(folio)) {
//
// unmap_poisoned_folio() cannot handle large folios
// in all cases yet.
//
    if (folio_test_large(folio) && !folio_test_hugetlb(folio)) {
// goto;
    }
    if (folio_test_lru(folio) && !folio_isolate_lru(folio)) {
// goto;
    }
    if (folio_mapped(folio)) {
    folio_lock(folio);
    unmap_poisoned_folio(folio, pfn, false);
    folio_unlock(folio);
    }
// goto;
    }
    if (!isolate_folio_to_list(folio, &source)) {
    if (__ratelimit(&migrate_rs)) {
    pr_warn!("failed to isolate pfn %lx\n",
    page_to_pfn(page));
    dump_page(page, "isolation failed");
    }
    }
// label;
    folio_put(folio);
    }
    if (!list_empty(&source)) {
pub static mut nmask: nodemask_t = 0;
pub static mut migration_target_control: usize = 0;
    let mut ret = 0;
//
// We have checked that migration range is on a single zone so
// we can use the nid of the first page to all the others.
//
    mtc.nid = folio_nid(list_first_entry(&source, folio, lru));
//
// try to allocate from a different node but reuse this node
// if there are no other online nodes to be used (e.g. we are
// offlining a part of the only existing node)
//
    node_clear(mtc.nid, nmask);
    if (nodes_empty(nmask)) {
    node_set(mtc.nid, nmask);
    }
    ret = migrate_pages(&source, alloc_migration_target, core::ptr::null_mut(),
    (unsigned long)&mtc, MIGRATE_SYNC, MR_MEMORY_HOTPLUG, core::ptr::null_mut());
    if (ret) {
    list_for_each_entry(folio, &source, lru) {
    if (__ratelimit(&migrate_rs)) {
    pr_warn!("migrating pfn %lx failed ret:%d\n",
    folio_pfn(folio), ret);
    dump_page(&folio.page,
    "migration failure");
    }
    }
    putback_movable_pages(&source);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parse_movable_node(p: *mut c_char) -> c_int {
    movable_node_enabled = true;
    return 0;
    }
    early_param!("movable_node", cmdline_parse_movable_node);
#[no_mangle]
pub unsafe extern "C" fn count_system_ram_pages_cb(start_pfn: c_ulong, nr_pages: c_ulong, data: *mut c_void) -> c_int {
    let mut nr_system_ram_pages = data;
// nr_system_ram_pages += nr_pages;
    return 0;
    }
//
// Must be called with mem_hotplug_lock in write mode.
//
#[no_mangle]
pub unsafe extern "C" fn offline_pages(start_pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone, group: *mut memory_group) -> c_int {
    unsigned long pfn, managed_pages, system_ram_pages = 0;
pub static mut end_pfn: c_ulong = 0;
    let mut pgdat = zone.zone_pgdat;
pub static mut node: c_int = 0;
pub static mut memory_notify: usize = 0;
pub static mut node_notify: usize = 0;
    let mut flags = 0;
pub static mut reason: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
pub static mut normal_pages: c_ulong = 0;
    enum zone_type zt;
//
// {on,off}lining is constrained to full memory sections (or more
// precisely to memory blocks from the user space POV).
// memmap_on_memory is an exception because it reserves initial part
// of the physical memory space for vmemmaps. That space is pageblock
// aligned.
//
    if (WARN_ON_ONCE!(!nr_pages || !pageblock_aligned(start_pfn) ||
    !IS_ALIGNED(start_pfn + nr_pages, PAGES_PER_SECTION))) {
    return -EINVAL;
    }
//
// Don't allow to offline memory blocks that contain holes.
// Consequently, memory blocks with holes can never get onlined
// via the hotplug path - online_pages() - as hotplugged memory has
// no holes. This way, we don't have to worry about memory holes,
// don't need pfn_valid() checks, and can avoid using
// walk_system_ram_range() later.
//
    walk_system_ram_range(start_pfn, nr_pages, &system_ram_pages,
    count_system_ram_pages_cb);
    if (system_ram_pages != nr_pages) {
    ret = -EINVAL;
    reason = "memory holes";
// goto;
    }
//
// We only support offlining of memory blocks managed by a single zone,
// checked by calling code. This is just a sanity check that we might
// want to remove in the future.
//
    if (WARN_ON_ONCE!(page_zone(pfn_to_page(start_pfn)) != zone ||
    page_zone(pfn_to_page(end_pfn - 1)) != zone)) {
    ret = -EINVAL;
    reason = "multizone range";
// goto;
    }
//
// Disable pcplists so that page isolation cannot race with freeing
// in a way that pages from isolated pageblock are left on pcplists.
//
    zone_pcp_disable(zone);
    lru_cache_disable();
// set above range as isolated
    ret = start_isolate_page_range(start_pfn, end_pfn,
    PB_ISOLATE_MODE_MEM_OFFLINE);
    if (ret) {
    reason = "failure to isolate range";
// goto;
    }
//
// Check whether the node will have no present pages after we offline
// 'nr_pages' more. If so, we know that the node will become empty, and
// so we will clear N_MEMORY for it.
//
    if (nr_pages >= pgdat.node_present_pages) {
    node_arg.nid = node;
    ret = node_notify(NODE_REMOVING_LAST_MEMORY, &node_arg);
    ret = notifier_to_errno(ret);
    if (ret) {
    reason = "node notifier failure";
// goto;
    }
    }
    ret = memory_notify(MEM_GOING_OFFLINE, &mem_arg);
    ret = notifier_to_errno(ret);
    if (ret) {
    reason = "notifier failure";
// goto;
    }
    do {
    pfn = start_pfn;
    do {
//
// Historically we always checked for any signal and
// can't limit it to fatal signals without eventually
// breaking user space.
//
    if (signal_pending(current)) {
    ret = -EINTR;
    reason = "signal backoff";
// goto;
    }
    cond_resched();
    ret = scan_movable_pages(pfn, end_pfn, &pfn);
    if (!ret) {
//
// TODO: fatal migration failures should bail
// out
//
    do_migrate_range(pfn, end_pfn);
    }
    } while (!ret);
    if (ret != -ENOENT) {
    reason = "unmovable page";
// goto;
    }
//
// Dissolve free hugetlb folios in the memory block before doing
// offlining actually in order to make hugetlbfs's object
// counting consistent.
//
    ret = dissolve_free_hugetlb_folios(start_pfn, end_pfn);
    if (ret) {
    reason = "failure to dissolve huge pages";
// goto;
    }
    ret = test_pages_isolated(start_pfn, end_pfn,
    PB_ISOLATE_MODE_MEM_OFFLINE);
    } while (ret);
// Mark all sections offline and remove free pages from the buddy.
    managed_pages = __offline_isolated_pages(start_pfn, end_pfn);
    pr_debug!("Offlined Pages %ld\n", nr_pages);
//
// The memory sections are marked offline, and the pageblock flags
// effectively stale; nobody should be touching them. Fixup the number
// of isolated pageblocks, memory onlining will properly revert this.
//
    spin_lock_irqsave(&zone.lock, flags);
    zone.nr_isolate_pageblock -= nr_pages / pageblock_nr_pages;
    spin_unlock_irqrestore(&zone.lock, flags);
    lru_cache_enable();
    zone_pcp_enable(zone);
// removal success
    adjust_managed_page_count(pfn_to_page(start_pfn), -managed_pages);
    adjust_present_page_count(pfn_to_page(start_pfn), group, -nr_pages);
// reinitialise watermarks and update pcp limits
    init_per_zone_wmark_min();
//
// Check whether this operation removes the last normal memory from
// the node. We do this before clearing N_MEMORY to avoid the possible
// transient "!N_MEMORY && N_NORMAL_MEMORY" state.
//
    if (zone_idx(zone) <= ZONE_NORMAL) {
    for (zt = 0; zt <= ZONE_NORMAL; zt++) {
    normal_pages += pgdat.node_zones[zt].present_pages;
    }
    if (!normal_pages) {
    node_clear_state(node, N_NORMAL_MEMORY);
    }
    }
//
// Make sure to mark the node as memory-less before rebuilding the zone
// list. Otherwise this node would still appear in the fallback lists.
//
    if (node_arg.nid >= 0) {
    node_clear_state(node, N_MEMORY);
    }
    if (!populated_zone(zone)) {
    zone_pcp_reset(zone);
    build_all_zonelists(core::ptr::null_mut());
    }
    if (node_arg.nid >= 0) {
    kcompactd_stop(node);
    kswapd_stop(node);
// Node went memoryless. Notify consumers
    node_notify(NODE_REMOVED_LAST_MEMORY, &node_arg);
    }
    writeback_set_ratelimit();
    memory_notify(MEM_OFFLINE, &mem_arg);
    remove_pfn_range_from_zone(zone, start_pfn, nr_pages);
    return 0;
// label;
// pushback to free area
    undo_isolate_page_range(start_pfn, end_pfn);
    memory_notify(MEM_CANCEL_OFFLINE, &mem_arg);
    if (node_arg.nid != NUMA_NO_NODE) {
    node_notify(NODE_CANCEL_REMOVING_LAST_MEMORY, &node_arg);
    }
// label;
    lru_cache_enable();
    zone_pcp_enable(zone);
// label;
    pr_debug!("memory offlining [mem %#010llx-%#010llx] failed due to %s\n",
    (unsigned long long) start_pfn << PAGE_SHIFT,
    ((unsigned long long) end_pfn << PAGE_SHIFT) - 1,
    reason);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_memblock_offlined_cb(mem: *mut memory_block, arg: *mut c_void) -> c_int {
    let mut nid = arg;
// nid = mem->nid;
    if (unlikely(mem.state != MEM_OFFLINE)) {
    phys_addr_t beginpa, endpa;
    beginpa = PFN_PHYS(section_nr_to_pfn(mem.start_section_nr));
    endpa = beginpa + memory_block_size_bytes() - 1;
    pr_warn!("removing memory fails, because memory [%pa-%pa] is onlined\n",
    &beginpa, &endpa);
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn count_memory_range_altmaps_cb(mem: *mut memory_block, arg: *mut c_void) -> c_int {
    let mut num_altmaps = arg;
    if (mem.altmap) {
// num_altmaps += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_cpu_on_node(nid: c_int) -> c_int {
    let mut cpu = 0;
    for_each_present_cpu(cpu) {
    if (cpu_to_node(cpu) == nid) {
//
// the cpu on this node isn't removed, and we can't
// offline this node.
//
    return -EBUSY;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_no_memblock_for_node_cb(mem: *mut memory_block, arg: *mut c_void) -> c_int {
pub static mut nid: c_int = 0;
//
// If a memory block belongs to multiple nodes, the stored nid is not
// reliable. However, such blocks are always online (e.g., cannot get
// offlined) and, therefore, are still spanned by the node.
//
    return mem.nid == nid ? -EEXIST : 0;
    }
//
// try_offline_node
// @nid: the node ID
//
// Offline a node if all memory sections and cpus of the node are removed.
//
// NOTE: The caller must call lock_device_hotplug() to serialize hotplug
// and online/offline operations before this call.
//
#[no_mangle]
pub unsafe extern "C" fn try_offline_node(nid: c_int) {
    let mut rc = 0;
//
// If the node still spans pages (especially ZONE_DEVICE), don't
// offline it. A node spans memory after move_pfn_range_to_zone(),
// e.g., after the memory block was onlined.
//
    if (node_spanned_pages(nid)) {
    return;
    }
//
// Especially offline memory blocks might not be spanned by the
// node. They will get spanned by the node once they get onlined.
// However, they link to the node in sysfs and can get onlined later.
//
    rc = for_each_memory_block(&nid, check_no_memblock_for_node_cb);
    if (rc) {
    return;
    }
    if (check_cpu_on_node(nid)) {
    return;
    }
//
// all memory/cpu of this node are removed, we can offline this
// node now.
//
    node_set_offline(nid);
    unregister_node(nid);
    }
    EXPORT_SYMBOL(try_offline_node);
#[no_mangle]
unsafe extern "C" fn memory_blocks_have_altmaps(start: u64, size: u64) -> c_int {
pub static mut num_memblocks: u64 = 0;
pub static mut num_altmaps: u64 = 0;
    if (!mhp_memmap_on_memory()) {
    return 0;
    }
    walk_memory_blocks(start, size, &num_altmaps,
    count_memory_range_altmaps_cb);
    if (num_altmaps == 0) {
    return 0;
    }
    if (WARN_ON_ONCE!(num_memblocks != num_altmaps)) {
    return -EINVAL;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn try_remove_memory(start: u64, size: u64) -> c_int {
    int rc, nid = NUMA_NO_NODE;
    BUG_ON!(check_hotplug_memory_range(start, size));
//
// All memory blocks must be offlined before removing memory.  Check
// whether all memory blocks in question are offline and return error
// if this is not the case.
//
// While at it, determine the nid. Note that if we'd have mixed nodes,
// we'd only try to offline the last determined one -- which is good
// enough for the cases we care about.
//
    rc = walk_memory_blocks(start, size, &nid, check_memblock_offlined_cb);
    if (rc) {
    return rc;
    }
// remove memmap entry
    firmware_map_remove(start, start + size, "System RAM");
    mem_hotplug_begin();
    rc = memory_blocks_have_altmaps(start, size);
    if (rc < 0) {
    mem_hotplug_done();
    return rc;
    } else if (!rc) {
//
// Memory block device removal under the device_hotplug_lock is
// a barrier against racing online attempts.
// No altmaps present, do the removal directly
//
    remove_memory_block_devices(start, size);
    arch_remove_memory(start, size, core::ptr::null_mut(), core::ptr::null_mut());
    } else {
// all memblocks in the range have altmaps
    remove_memory_blocks_and_altmaps(start, size);
    }
    if (IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    memblock_remove(start, size);
    }
    release_mem_region_adjustable(start, size);
    if (nid != NUMA_NO_NODE) {
    try_offline_node(nid);
    }
    mem_hotplug_done();
    return 0;
    }
//
// __remove_memory - Remove memory if every memory block is offline
// @start: physical address of the region to remove
// @size: size of the region to remove
//
// NOTE: The caller must call lock_device_hotplug() to serialize hotplug
// and online/offline operations before this call, as required by
// try_offline_node().
//
#[no_mangle]
pub unsafe extern "C" fn __remove_memory(start: u64, size: u64) {
//
// trigger BUG() if some memory is not offlined prior to calling this
// function
//
    if (try_remove_memory(start, size)) {
    BUG();
    }
    }
//
// Remove memory if every memory block is offline, otherwise return -EBUSY is
// some memory is not offline
//
#[no_mangle]
pub unsafe extern "C" fn remove_memory(start: u64, size: u64) -> c_int {
    let mut rc = 0;
    lock_device_hotplug();
    rc = try_remove_memory(start, size);
    unlock_device_hotplug();
    return rc;
    }
    EXPORT_SYMBOL_GPL(remove_memory);
#[no_mangle]
unsafe extern "C" fn try_offline_memory_block(mem: *mut memory_block, arg: *mut c_void) -> c_int {
pub static mut online_type: mmop = 0;
    let mut online_types = arg;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
//
// Sense the online_type via the zone of the memory block. Offlining
// with multiple zones within one memory block will be rejected
// by offlining code ... so we don't care about that.
//
    page = pfn_to_online_page(section_nr_to_pfn(mem.start_section_nr));
    if (page && page_zonenum(page) == ZONE_MOVABLE) {
    online_type = MMOP_ONLINE_MOVABLE;
    }
    rc = device_offline(&mem.dev);
//
// Default is MMOP_OFFLINE - change it only if offlining succeeded,
// so try_reonline_memory_block() can do the right thing.
//
    if (!rc) {
// online_types = online_type;
    }
    (*online_types)++;
// Ignore if already offline.
    return rc < 0 ? rc : 0;
    }
#[no_mangle]
unsafe extern "C" fn try_reonline_memory_block(mem: *mut memory_block, arg: *mut c_void) -> c_int {
    let mut online_types = arg;
    let mut rc = 0;
    if (**online_types != MMOP_OFFLINE) {
    mem.online_type = (enum mmop)**online_types;
    rc = device_online(&mem.dev);
    if (rc < 0) {
    pr_warn!("%s: Failed to re-online memory: %d",
    __func__, rc);
    }
    }
// Continue processing all remaining memory blocks.
    (*online_types)++;
    return 0;
    }
//
// Try to offline and remove memory. Might take a long time to finish in case
// memory is still in use. Primarily useful for memory devices that logically
// unplugged all memory (so it's no longer in use) and want to offline + remove
// that memory.
//
#[no_mangle]
pub unsafe extern "C" fn offline_and_remove_memory(start: u64, size: u64) -> c_int {
pub static mut range: usize = 0;
    return offline_and_remove_memory_ranges(&range, 1);
    }
    EXPORT_SYMBOL_GPL(offline_and_remove_memory);
//
// offline_and_remove_memory_ranges - offline and remove multiple memory ranges
// @ranges: array of physical address ranges to offline and remove
// @nr_ranges: number of entries in @ranges
//
// Offline and remove several memory ranges as one operation, serialized
// against other hotplug operations by a single lock_device_hotplug().
//
// This offlines all ranges before removing any of them.  If offlining any
// range fails, the entire process is reverted and nothing is removed.
// This provides a fully atomic semantic for unplugging an entire device.
//
// Each range must be memory-block aligned in start and size.
//
// Return: 0 on success, negative errno on failure (never positive).  On
// failure no range has been removed.
//
#[no_mangle]
pub unsafe extern "C" fn offline_and_remove_memory_ranges(ranges: *mut range, nr_ranges: c_uint) -> c_int {
pub static mut mb_count: c_ulong = 0;
    let mut online_types = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut i = 0;
pub static mut rc: c_int = 0;
    if (!ranges || !nr_ranges) {
    return -EINVAL;
    }
    while (i < nr_ranges) {
pub static mut start: u64 = 0;
pub static mut size: u64 = 0;
    if (!IS_ALIGNED(start, memory_block_size_bytes()) ||
    !IS_ALIGNED(size, memory_block_size_bytes()) || !size) {
    return -EINVAL;
    }
    mb_count += size / memory_block_size_bytes();
    }
//
// Remember the old online type of every memory block across all ranges,
// so we can revert if offlining a later block fails.  All entries start
// as MMOP_OFFLINE so blocks we never touched are skipped on rollback.
//
    online_types = kmalloc_array(mb_count, sizeof!(*online_types),
    GFP_KERNEL);
    if (!online_types) {
    return -ENOMEM;
    }
    memset(online_types, MMOP_OFFLINE, mb_count);
    lock_device_hotplug();
//
// Phase 1: offline every block in every range.  An already-offline
// block folds to success, so out-of-band offlining never blocks unplug.
//
    tmp = online_types;
    while (i < nr_ranges) {
    rc = walk_memory_blocks(ranges[i].start, range_len(&ranges[i]),
    &tmp, try_offline_memory_block);
    if (rc) {
    break;
    }
    }
// If any failure occurred at all, rollback any changes and bail
    if (rc) {
    tmp = online_types;
    for (i = 0; i < nr_ranges; i++) {
    walk_memory_blocks(ranges[i].start,
    range_len(&ranges[i]), &tmp,
    try_reonline_memory_block);
    }
// goto;
    }
// Phase 2: Remove. This should never fail holding the hotplug lock
    for (i = 0; i < nr_ranges; i++) {
    WARN_ON_ONCE!(try_remove_memory(ranges[i].start,
    range_len(&ranges[i])));
    }
// label;
    unlock_device_hotplug();
    kfree(online_types);
    return rc;
    }
    EXPORT_SYMBOL_GPL(offline_and_remove_memory_ranges);