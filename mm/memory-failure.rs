//! Automatically rewritten from C to Rust
//! Source: mm/memory-failure.c
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
// Copyright (C) 2008, 2009 Intel Corporation
// Authors: Andi Kleen, Fengguang Wu
//
// High level machine check handler. Handles pages reported by the
// hardware as being corrupted usually due to a multi-bit ECC memory or cache
// failure.
//
// In addition there is a "soft offline" entry point that allows stop using
// not-yet-corrupted-by-suspicious pages without killing anything.
//
// Handles page cache pages in various states.	The tricky part
// here is that we can access any page asynchronously in respect to
// other VM users, because memory failures could happen anytime and
// anywhere. This could violate some of their assumptions. This is why
// this code has to be extremely careful. Generally it tries to use
// normal locking rules, as in get the standard locks, even if that means
// the error handling takes potentially a long time.
//
// It can be very tempting to add handling for obscure cases here.
// In general any code for handling new cases should only be added iff:
// - You know how to test it.
// - You have a test that can be added to mce-test
// https://git.kernel.org/cgit/utils/cpu/mce/mce-test.git
// - The case actually shows up as a frequent (top 10) page state in
// tools/mm/page-types when running a real workload.
//
// There are several operations here with exponential complexity because
// of unsuitable VM data structures. For example the operation to map back
// from RMAP chains to processes has to walk the complete process list and
// has non linear complexity with the number. But since memory corruptions
// are rare we hope to get away with this. This avoids impacting the core
// VM.
//

// Macro flag: #define CREATE_TRACE_POINTS

    static int sysctl_memory_failure_early_kill ;
pub static mut : int sysctl_memory_failure_recovery = 1;
pub static mut : int sysctl_enable_soft_offline = 1;
    static int sysctl_panic_on_unrecoverable_mf ;
pub static mut : atomic_long_t num_poisoned_pages = 0;
    static bool hw_memory_failure ;
pub static mut mf_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn num_poisoned_pages_inc(pfn: c_ulong) {
    atomic_long_inc(&num_poisoned_pages);
    memblk_nr_poison_inc(pfn);
    }
#[no_mangle]
pub unsafe extern "C" fn num_poisoned_pages_sub(pfn: c_ulong, i: c_long) {
    atomic_long_sub(i, &num_poisoned_pages);
    if (pfn != -1UL) {
    memblk_nr_poison_sub(pfn, i);
    }
    }
//
// MF_ATTR_RO - Create sysfs entry for each memory failure statistics.
// @_name: name of the file in the per NUMA sysfs directory.
//

    static ssize_t _name##_show(device *dev, device_attribute *attr,	
    char *buf)				
    {								
    let mut mf_stats = &NODE_DATA(dev.id).mf_stats;			
    return sysfs_emit(buf, "%lu\n", mf_stats._name);	
    }								
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: _name) -> static {
    static DEVICE_ATTR_RO(_name)
    MF_ATTR_RO(total);
    MF_ATTR_RO(ignored);
    MF_ATTR_RO(failed);
    MF_ATTR_RO(delayed);
    MF_ATTR_RO(recovered);
    static struct attribute *memory_failure_attr[] = {
    &dev_attr_total.attr,
    &dev_attr_ignored.attr,
    &dev_attr_failed.attr,
    &dev_attr_delayed.attr,
    &dev_attr_recovered.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
pub static mut ctl_table: usize = 0;
pub static mut pfn_space_itree: rb_root_cached = 0;
pub static mut pfn_space_lock: usize = 0;
//
// Return values:
// 1:   the page is dissolved (if needed) and taken off from buddy,
// 0:   the page is dissolved (if needed) and not taken off from buddy,
// < 0: failed to dissolve.
//
#[no_mangle]
unsafe extern "C" fn __page_handle_poison(page: *mut page) -> c_int {
    let mut ret = 0;
    zone_pcp_disable(page_zone(page));
    ret = dissolve_free_hugetlb_folio(page_folio(page));
    if (!ret) {
    ret = take_page_off_buddy(page);
    }
    zone_pcp_enable(page_zone(page));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn page_handle_poison(page: *mut page, hugepage_or_freepage: bool, release: bool) -> bool {
    if (hugepage_or_freepage) {
//
// Doing this check for free pages is also fine since
// dissolve_free_hugetlb_folio() returns 0 for non-hugetlb folios as well.
//
    if (__page_handle_poison(page) <= 0) {
//
// We could fail to take off the target page from buddy
// for example due to racy page allocation, but that's
// acceptable because soft-offlined page is not broken
// and if someone really want to use it, they should
// take it.
//
    return false;
    }
    }
    SetPageHWPoison(page);
    if (release) {
    put_page(page);
    }
    page_ref_inc(page);
    num_poisoned_pages_inc(page_to_pfn(page));
    return true;
    }
    static hwpoison_filter_func_t  *hwpoison_filter_func ;
#[no_mangle]
pub unsafe extern "C" fn hwpoison_filter_register(filter: *mut hwpoison_filter_func_t) {
    rcu_assign_pointer(hwpoison_filter_func, filter);
    }
    EXPORT_SYMBOL_GPL(hwpoison_filter_register);
#[no_mangle]
pub unsafe extern "C" fn hwpoison_filter_unregister() {
    RCU_INIT_POINTER(hwpoison_filter_func, core::ptr::null_mut());
    synchronize_rcu();
    }
    EXPORT_SYMBOL_GPL(hwpoison_filter_unregister);
#[no_mangle]
unsafe extern "C" fn hwpoison_filter(p: *mut page) -> c_int {
pub static mut ret: c_int = 0;
pub static mut filter: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    filter = rcu_dereference(hwpoison_filter_func);
    if (filter) {
    ret = filter(p);
    }
    rcu_read_unlock();
    return ret;
    }
//
// Kill all processes that have a poisoned page mapped and then isolate
// the page.
//
// General strategy:
// Find all processes having the page mapped and kill them.
// But we keep a page reference around so that the page is not
// actually freed yet.
// Then stash the page away
//
// There's no convenient way to get back to mapped processes
// from the VMAs. So do a brute-force search over all
// running processes.
//
// Remember that machine checks are not common (or rather
// if they are common you have other problems), so this shouldn't
// be a performance issue.
//
// Also there are some races possible while we get from the
// error detection to actually handle it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct to_kill {
    pub nd: list_head,
    pub tsk: *mut task_struct,
    pub addr: c_ulong,
    pub size_shift: c_short,
}

//
// Send all the processes who have the page mapped a signal.
// ``action optional'' if they are not immediately affected by the error
// ``action required'' if error happened in current execution context
//
#[no_mangle]
unsafe extern "C" fn kill_proc(tk: *mut to_kill, pfn: c_ulong, flags: c_int) -> c_int {
    let mut t = tk.tsk;
pub static mut addr_lsb: c_short = 0;
pub static mut ret: c_int = 0;
    pr_err!("%#lx: Sending SIGBUS to %s:%d due to hardware memory corruption\n",
    pfn, t.comm, task_pid_nr(t));
    if ((flags & MF_ACTION_REQUIRED) && (t == current)) {
    ret = force_sig_mceerr(BUS_MCEERR_AR,
    tk.addr, addr_lsb);
    }
    else {
//
// Signal other processes sharing the page if they have
// PF_MCE_EARLY set.
// Don't use force here, it's convenient if the signal
// can be temporarily blocked.
//
    ret = send_sig_mceerr(BUS_MCEERR_AO, tk.addr,
    addr_lsb, t);
    }
    if (ret < 0) {
    pr_info!("Error sending signal to %s:%d: %d\n",
    t.comm, task_pid_nr(t), ret);
    }
    return ret;
    }
//
// Unknown page type encountered. Try to check whether it can turn PageLRU by
// lru_add_drain_all.
//
#[no_mangle]
pub unsafe extern "C" fn shake_folio(folio: *mut folio) {
    if (folio_test_hugetlb(folio)) {
    return;
    }
//
// TODO: Could shrink slab caches here if a lightweight range-based
// shrinker will be available.
//
    if (folio_test_slab(folio)) {
    return;
    }
    lru_add_drain_all();
    }
    EXPORT_SYMBOL_GPL(shake_folio);
#[no_mangle]
unsafe extern "C" fn shake_page(page: *mut page) {
    shake_folio(page_folio(page));
    }
#[no_mangle]
pub unsafe extern "C" fn dev_pagemap_mapping_shift(vma: *mut vm_area_struct, address: c_ulong) -> c_ulong {
pub static mut ret: c_ulong = 0;
pub static mut pgd: *mut c_void = core::ptr::null_mut();
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
pub static mut pmd: *mut c_void = core::ptr::null_mut();
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut ptent;
    VM_BUG_ON_VMA(address == -EFAULT, vma);
    pgd = pgd_offset(vma.vm_mm, address);
    if (!pgd_present(*pgd)) {
    return 0;
    }
    p4d = p4d_offset(pgd, address);
    if (!p4d_present(*p4d)) {
    return 0;
    }
    pud = pud_offset(p4d, address);
    if (!pud_present(*pud)) {
    return 0;
    }
    if (pud_trans_huge(*pud)) {
    return PUD_SHIFT;
    }
    pmd = pmd_offset(pud, address);
    if (!pmd_present(*pmd)) {
    return 0;
    }
    if (pmd_trans_huge(*pmd)) {
    return PMD_SHIFT;
    }
    pte = pte_offset_map(pmd, address);
    if (!pte) {
    return 0;
    }
    ptent = ptep_get(pte);
    if (pte_present(ptent)) {
    ret = PAGE_SHIFT;
    }
    pte_unmap(pte);
    return ret;
    }
//
// Failure handling: if we can't find or can't kill a process there's
// not much we can do.	We just print a message and ignore otherwise.
//
// Schedule a process for later kill.
// Uses GFP_ATOMIC allocations to avoid potential recursions in the VM.
//
#[no_mangle]
pub unsafe extern "C" fn __add_to_kill(tsk: *mut task_struct, p: *mut page, vma: *mut vm_area_struct, to_kill: *mut list_head, addr: c_ulong) {
pub static mut tk: *mut c_void = core::ptr::null_mut();
    tk = kmalloc_obj(to_kill, GFP_ATOMIC);
    if (!tk) {
    pr_err!("Out of memory while machine check handling\n");
    return;
    }
    tk.addr = addr;
    if (is_zone_device_page(p)) {
    tk.size_shift = dev_pagemap_mapping_shift(vma, tk.addr);
    }
    else {
    tk.size_shift = folio_shift(page_folio(p));
    }
//
// Send SIGKILL if "tk->addr == -EFAULT". Also, as
// "tk->size_shift" is always non-zero for !is_zone_device_page(),
// so "tk->size_shift == 0" effectively checks no mapping on
// ZONE_DEVICE. Indeed, when a devdax page is mmapped N times
// to a process' address space, it's possible not all N VMAs
// contain mappings for the page, but at least one VMA does.
// Only deliver SIGBUS with payload derived from the VMA that
// has a mapping for the page.
//
    if (tk.addr == -EFAULT) {
    pr_info!("Unable to find user space address %lx in %s\n",
    page_to_pfn(p), tsk.comm);
    } else if (tk.size_shift == 0) {
    kfree(tk);
    return;
    }
    get_task_struct(tsk);
    tk.tsk = tsk;
    list_add_tail(&tk.nd, to_kill);
    }
#[no_mangle]
pub unsafe extern "C" fn add_to_kill_anon_file(tsk: *mut task_struct, p: *mut page, vma: *mut vm_area_struct, to_kill: *mut list_head, addr: c_ulong) {
    if (addr == -EFAULT) {
    return;
    }
    __add_to_kill(tsk, p, vma, to_kill, addr);
    }

#[no_mangle]
pub unsafe extern "C" fn task_in_to_kill_list(to_kill: *mut list_head, tsk: *mut task_struct) -> bool {
    let mut tk = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(tk, next, to_kill, nd) {
    if (tk.tsk == tsk) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn add_to_kill_ksm(tsk: *mut task_struct, p: *mut page, vma: *mut vm_area_struct, to_kill: *mut list_head, addr: c_ulong) {
    if (!task_in_to_kill_list(to_kill, tsk)) {
    __add_to_kill(tsk, p, vma, to_kill, addr);
    }
    }

//
// Kill the processes that have been collected earlier.
//
// Only do anything when FORCEKILL is set, otherwise just free the
// list (this is used for clean pages which do not need killing)
//
#[no_mangle]
pub unsafe extern "C" fn kill_procs(to_kill: *mut list_head, forcekill: bool, pfn: c_ulong, flags: c_int) {
    let mut tk = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(tk, next, to_kill, nd) {
    if (forcekill) {
    if (tk.addr == -EFAULT) {
    pr_err!("%#lx: forcibly killing %s:%d because of failure to unmap corrupted page\n",
    pfn, tk.tsk.comm, task_pid_nr(tk.tsk));
    do_send_sig_info(SIGKILL, SEND_SIG_PRIV,
    tk.tsk, PIDTYPE_PID);
    }
//
// In theory the process could have mapped
// something else on the address in-between. We could
// check for that, but we need to tell the
// process anyways.
//

    else if (kill_proc(tk, pfn, flags) < 0) {
    pr_err!("%#lx: Cannot send advisory machine check signal to %s:%d\n",
    pfn, tk.tsk.comm, task_pid_nr(tk.tsk));
    }
    }
    list_del(&tk.nd);
    put_task_struct(tk.tsk);
    kfree(tk);
    }
    }
//
// Find a dedicated thread which is supposed to handle SIGBUS(BUS_MCEERR_AO)
// on behalf of the thread group. Return task_struct of the (first found)
// dedicated thread if found, and return NULL otherwise.
//
// We already hold rcu lock in the caller, so we don't have to call
// rcu_read_lock/unlock() in this function.
//
#[no_mangle]
pub unsafe extern "C" fn find_early_kill_thread(tsk: *mut task_struct) -> *mut c_void {
pub static mut t: *mut c_void = core::ptr::null_mut();
    for_each_thread(tsk, t) {
    if (t.flags & PF_MCE_PROCESS) {
    if (t.flags & PF_MCE_EARLY) {
    return t;
    }
    } else {
    if (sysctl_memory_failure_early_kill) {
    return t;
    }
    }
    }
    return core::ptr::null_mut();
    }
//
// Determine whether a given process is "early kill" process which expects
// to be signaled when some page under the process is hwpoisoned.
// Return task_struct of the dedicated thread (main thread unless explicitly
// specified) if the process is "early kill" and otherwise returns NULL.
//
// Note that the above is true for Action Optional case. For Action Required
// case, it's only meaningful to the current thread which need to be signaled
// with SIGBUS, this error is Action Optional for other non current
// processes sharing the same error page,if the process is "early kill", the
// task_struct of the dedicated thread will also be returned.
//
#[no_mangle]
pub unsafe extern "C" fn task_early_kill(tsk: *mut task_struct, force_early: c_int) -> *mut c_void {
    if (!tsk.mm) {
    return core::ptr::null_mut();
    }
//
// Comparing ->mm here because current task might represent
// a subthread, while tsk always points to the main thread.
//
    if (force_early && tsk.mm == current.mm) {
    return current;
    }
    return find_early_kill_thread(tsk);
    }
//
// Collect processes when the error hit an anonymous page.
//
#[no_mangle]
pub unsafe extern "C" fn collect_procs_anon(folio: *mut folio, page: *mut page, to_kill: *mut list_head, force_early: c_int) {
pub static mut tsk: *mut c_void = core::ptr::null_mut();
pub static mut av: *mut c_void = core::ptr::null_mut();
    let mut pgoff;
    av = folio_lock_anon_vma_read(folio, core::ptr::null_mut());
    if (av == core::ptr::null_mut())	/* Not actually mapped anymore */ {
    return;
    }
    pgoff = page_pgoff(folio, page);
    rcu_read_lock();
    for_each_process(tsk) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut vmac: *mut c_void = core::ptr::null_mut();
    let mut t = task_early_kill(tsk, force_early);
    let mut addr = 0;
    if (!t) {
    continue;
    }
    anon_rmap_tree_foreach(vmac, av, pgoff, pgoff) {
    vma = vmac.vma;
    if (vma.vm_mm != t.mm) {
    continue;
    }
    addr = page_mapped_in_vma(page, vma);
    add_to_kill_anon_file(t, page, vma, to_kill, addr);
    }
    }
    rcu_read_unlock();
    anon_vma_unlock_read(av);
    }
//
// Collect processes when the error hit a file mapped page.
//
#[no_mangle]
pub unsafe extern "C" fn collect_procs_file(folio: *mut folio, page: *mut page, to_kill: *mut list_head, force_early: c_int) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    let mut mapping = folio.mapping;
    let mut pgoff;
    i_mmap_lock_read(mapping);
    rcu_read_lock();
    pgoff = page_pgoff(folio, page);
    for_each_process(tsk) {
    let mut t = task_early_kill(tsk, force_early);
    let mut addr = 0;
    if (!t) {
    continue;
    }
    mapping_rmap_tree_foreach(vma, mapping, pgoff, pgoff) {
//
// Send early kill signal to tasks where a vma covers
// the page but the corrupted page is not necessarily
// mapped in its pte.
// Assume applications who requested early kill want
// to be informed of all such data corruptions.
//
    if (vma.vm_mm != t.mm) {
    continue;
    }
    addr = page_address_in_vma(folio, page, vma);
    add_to_kill_anon_file(t, page, vma, to_kill, addr);
    }
    }
    rcu_read_unlock();
    i_mmap_unlock_read(mapping);
    }

#[no_mangle]
pub unsafe extern "C" fn add_to_kill_fsdax(tsk: *mut task_struct, p: *mut page, vma: *mut vm_area_struct, to_kill: *mut list_head, pgoff: pgoff_t) {
pub static mut addr: c_ulong = 0;
    __add_to_kill(tsk, p, vma, to_kill, addr);
    }
//
// Collect processes when the error hit a fsdax page.
//
#[no_mangle]
pub unsafe extern "C" fn collect_procs_fsdax(page: *mut page, mapping: *mut address_space, pgoff: pgoff_t, to_kill: *mut list_head, pre_remove: bool) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    i_mmap_lock_read(mapping);
    rcu_read_lock();
    for_each_process(tsk) {
    let mut t = tsk;
//
// Search for all tasks while MF_MEM_PRE_REMOVE is set, because
// the current may not be the one accessing the fsdax page.
// Otherwise, search for the current task.
//
    if (!pre_remove) {
    t = task_early_kill(tsk, true);
    }
    if (!t) {
    continue;
    }
    mapping_rmap_tree_foreach(vma, mapping, pgoff, pgoff) {
    if (vma.vm_mm == t.mm) {
    add_to_kill_fsdax(t, page, vma, to_kill, pgoff);
    }
    }
    }
    rcu_read_unlock();
    i_mmap_unlock_read(mapping);
    }

//
// Collect the processes who have the corrupted page mapped to kill.
//
#[no_mangle]
pub unsafe extern "C" fn collect_procs(folio: *mut folio, page: *mut page, tokill: *mut list_head, force_early: c_int) {
    if (!folio.mapping) {
    return;
    }
    if (unlikely(folio_test_ksm(folio))) {
    collect_procs_ksm(folio, page, tokill, force_early);
    }

    else if (folio_test_anon(folio)) {
    collect_procs_anon(folio, page, tokill, force_early);
    }
    else {
    collect_procs_file(folio, page, tokill, force_early);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwpoison_walk {
    pub tk: to_kill,
    pub pfn: c_ulong,
    pub flags: c_int,
}

#[no_mangle]
unsafe extern "C" fn set_to_kill(tk: *mut to_kill, addr: c_ulong, shift: c_short) {
    tk.addr = addr;
    tk.size_shift = shift;
    }
#[no_mangle]
pub unsafe extern "C" fn check_hwpoisoned_entry(pte: pte_t, addr: c_ulong, shift: c_short, poisoned_pfn: c_ulong, tk: *mut to_kill) -> c_int {
pub static mut pfn: c_ulong = 0;
    let mut hwpoison_vaddr = 0;
    let mut mask = 0;
    if (pte_present(pte)) {
    pfn = pte_pfn(pte);
    } else {
pub static mut entry: softleaf_t = 0;
    if (softleaf_is_hwpoison(entry)) {
    pfn = softleaf_to_pfn(entry);
    }
    }
    mask = ~((1UL << (shift - PAGE_SHIFT)) - 1);
    if (!pfn || pfn != (poisoned_pfn & mask)) {
    return 0;
    }
    hwpoison_vaddr = addr + ((poisoned_pfn - pfn) << PAGE_SHIFT);
    set_to_kill(tk, hwpoison_vaddr, shift);
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn check_hwpoisoned_pmd_entry(pmdp: *mut pmd_t, addr: c_ulong, hwp: *mut hwpoison_walk) -> c_int {
pub static mut pmd: pmd_t = 0;
    let mut pfn = 0;
    let mut hwpoison_vaddr = 0;
    if (!pmd_present(pmd)) {
    return 0;
    }
    pfn = pmd_pfn(pmd);
    if (pfn <= hwp.pfn && hwp.pfn < pfn + HPAGE_PMD_NR) {
    hwpoison_vaddr = addr + ((hwp.pfn - pfn) << PAGE_SHIFT);
    set_to_kill(&hwp.tk, hwpoison_vaddr, PAGE_SHIFT);
    return 1;
    }
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: check_hwpoisoned_pmd_entry
pub unsafe extern "C" fn check_hwpoisoned_pmd_entry_dup(pmdp: *mut pmd_t, addr: c_ulong, hwp: *mut hwpoison_walk) -> c_int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hwpoison_pte_range(pmdp: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut hwp = walk.private;
pub static mut ret: c_int = 0;
    let mut ptep = core::ptr::null_mut();
    let mut mapped_pte = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    ptl = pmd_trans_huge_lock(pmdp, walk.vma);
    if (ptl) {
    ret = check_hwpoisoned_pmd_entry(pmdp, addr, hwp);
    spin_unlock(ptl);
// goto;
    }
    mapped_pte = ptep = pte_offset_map_lock(walk.vma.vm_mm, pmdp,
    addr, &ptl);
    if (!ptep) {
// goto;
    }
    while (addr != end) {
    ret = check_hwpoisoned_entry(ptep_get(ptep), addr, PAGE_SHIFT,
    hwp.pfn, &hwp.tk);
    if (ret == 1) {
    break;
    }
    }
    pte_unmap_unlock(mapped_pte, ptl);
// label;
    cond_resched();
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn hwpoison_hugetlb_range(ptep: *mut pte_t, hmask: c_ulong, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut hwp = walk.private;
    let mut h = hstate_vma(walk.vma);
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut pte;
    let mut ret = 0;
    ptl = huge_pte_lock(h, walk.mm, ptep);
    pte = huge_ptep_get(walk.mm, addr, ptep);
    ret = check_hwpoisoned_entry(pte, addr, huge_page_shift(h),
    hwp.pfn, &hwp.tk);
    spin_unlock(ptl);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn hwpoison_test_walk(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
// We also want to consider pages mapped into VM_PFNMAP.
    return 0;
    }
pub static mut mm_walk_ops: usize = 0;
//
// Sends SIGBUS to the current process with error info.
//
// This function is intended to handle "Action Required" MCEs on already
// hardware poisoned pages. They could happen, for example, when
// memory_failure() failed to unmap the error page at the first call, or
// when multiple local machine checks happened on different CPUs.
//
// MCE handler currently has no easy access to the error virtual address,
// so this function walks page table to find it. The returned virtual address
// is proper in most cases, but it could be wrong when the application
// process has multiple entries mapping the error page.
//
#[no_mangle]
pub unsafe extern "C" fn kill_accessing_process(p: *mut task_struct, pfn: c_ulong, flags: c_int) -> c_int {
    let mut ret = 0;
pub static mut hwpoison_walk: usize = 0;
    priv.tk.tsk = p;
    if (!p.mm) {
    return -EFAULT;
    }
    mmap_read_lock(p.mm);
    ret = walk_page_range(p.mm, 0, TASK_SIZE, &hwpoison_walk_ops,
    &priv);
//
// ret = 1 when CMCI wins, regardless of whether try_to_unmap()
// succeeds or fails, then kill the process with SIGBUS.
// ret = 0 when poison page is a clean page and it's dropped, no
// SIGBUS is needed.
//
    if (ret == 1 && priv.tk.addr) {
    kill_proc(&priv.tk, pfn, flags);
    }
    mmap_read_unlock(p.mm);
    return ret > 0 ? -EHWPOISON : 0;
    }
//
// MF_IGNORED - The m-f() handler marks the page as PG_hwpoisoned'ed.
// But it could not do more to isolate the page from being accessed again,
// nor does it kill the process. This is extremely rare and one of the
// potential causes is that the page state has been changed due to
// underlying race condition. This is the most severe outcomes.
//
// MF_FAILED - The m-f() handler marks the page as PG_hwpoisoned'ed.
// It should have killed the process, but it can't isolate the page,
// due to conditions such as extra pin, unmap failure, etc. Accessing
// the page again may trigger another MCE and the process will be killed
// by the m-f() handler immediately.
//
// MF_DELAYED - The m-f() handler marks the page as PG_hwpoisoned'ed.
// The page is unmapped, and is removed from the LRU or file mapping.
// An attempt to access the page again will trigger page fault and the
// PF handler will kill the process.
//
// MF_RECOVERED - The m-f() handler marks the page as PG_hwpoisoned'ed.
// The page has been completely isolated, that is, unmapped, taken out of
// the buddy system, or hole-punched out of the file mapping.
//
    static const char *action_name[] = {
    [MF_IGNORED] = "Ignored",
    [MF_FAILED] = "Failed",
    [MF_DELAYED] = "Delayed",
    [MF_RECOVERED] = "Recovered",
    };
    static const char * const action_page_types[] = {
    [MF_MSG_KERNEL]			= "reserved kernel page",
    [MF_MSG_KERNEL_HIGH_ORDER]	= "high-order kernel page",
    [MF_MSG_HUGE]			= "huge page",
    [MF_MSG_FREE_HUGE]		= "free huge page",
    [MF_MSG_GET_HWPOISON]		= "get hwpoison page",
    [MF_MSG_UNMAP_FAILED]		= "unmapping failed page",
    [MF_MSG_DIRTY_SWAPCACHE]	= "dirty swapcache page",
    [MF_MSG_CLEAN_SWAPCACHE]	= "clean swapcache page",
    [MF_MSG_DIRTY_MLOCKED_LRU]	= "dirty mlocked LRU page",
    [MF_MSG_CLEAN_MLOCKED_LRU]	= "clean mlocked LRU page",
    [MF_MSG_DIRTY_UNEVICTABLE_LRU]	= "dirty unevictable LRU page",
    [MF_MSG_CLEAN_UNEVICTABLE_LRU]	= "clean unevictable LRU page",
    [MF_MSG_DIRTY_LRU]		= "dirty LRU page",
    [MF_MSG_CLEAN_LRU]		= "clean LRU page",
    [MF_MSG_TRUNCATED_LRU]		= "already truncated LRU page",
    [MF_MSG_BUDDY]			= "free buddy page",
    [MF_MSG_DAX]			= "dax page",
    [MF_MSG_UNSPLIT_THP]		= "unsplit thp",
    [MF_MSG_ALREADY_POISONED]	= "already poisoned page",
    [MF_MSG_PFN_MAP]                = "non struct page pfn",
    [MF_MSG_UNKNOWN]		= "unknown page",
    };
//
// XXX: It is possible that a page is isolated from LRU cache,
// and then kept in swap cache or failed to remove from page cache.
// The page count will stop it from being freed by unpoison.
// Stress tests should be aware of this memory leak problem.
//
#[no_mangle]
unsafe extern "C" fn delete_from_lru_cache(folio: *mut folio) -> c_int {
    if (folio_isolate_lru(folio)) {
//
// Clear sensible page flags, so that the buddy system won't
// complain when the folio is unpoison-and-freed.
//
    folio_clear_active(folio);
    folio_clear_unevictable(folio);
//
// Poisoned page might never drop its ref count to 0 so we have
// to uncharge it manually from its memcg.
//
    mem_cgroup_uncharge(folio);
//
// drop the refcount elevated by folio_isolate_lru()
//
    folio_put(folio);
    return 0;
    }
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn truncate_error_folio(folio: *mut folio, pfn: c_ulong, mapping: *mut address_space) -> c_int {
pub static mut ret: c_int = 0;
    if (mapping.a_ops.error_remove_folio) {
pub static mut err: c_int = 0;
    if (err != 0) {
    pr_info!("%#lx: Failed to punch page: %d\n", pfn, err);
    }

    else if (!filemap_release_folio(folio, GFP_NOIO)) {
    pr_info!("%#lx: failed to release buffers\n", pfn);
    }
    else {
    ret = MF_RECOVERED;
    }
    } else {
//
// If the file system doesn't support it just invalidate
// This fails on dirty or anything with private pages
//
    if (mapping_evict_folio(mapping, folio)) {
    ret = MF_RECOVERED;
    }
    else {
    pr_info!("%#lx: Failed to invalidate\n",	pfn);
    }
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_state {
    pub mask: c_ulong,
    pub res: c_ulong,
    pub type: mf_action_page_type,
// Callback ->action() has to unlock the relevant page inside it.
    pub p): *mut *mut *mut int (action)(page_state ps, page,
}

//
// Return true if page is still referenced by others, otherwise return
// false.
//
// The extra_pins is true when one extra refcount is expected.
//
#[no_mangle]
pub unsafe extern "C" fn has_extra_refcount(ps: *mut page_state, p: *mut page, extra_pins: bool) -> bool {
pub static mut count: c_int = 0;
    if (extra_pins) {
    count -= folio_nr_pages(page_folio(p));
    }
    if (count > 0) {
    pr_err!("%#lx: %s still referenced by %d users\n",
    page_to_pfn(p), action_page_types[ps.type], count);
    return true;
    }
    return false;
    }
//
// Page in unknown state. Do nothing.
// This is a catch-all in case we fail to make sense of the page state.
//
#[no_mangle]
unsafe extern "C" fn me_unknown(ps: *mut page_state, p: *mut page) -> c_int {
    pr_err!("%#lx: Unknown page state\n", page_to_pfn(p));
    unlock_page(p);
    return MF_IGNORED;
    }
//
// Clean (or cleaned) page cache page.
//
#[no_mangle]
unsafe extern "C" fn me_pagecache_clean(ps: *mut page_state, p: *mut page) -> c_int {
    let mut folio = page_folio(p);
    let mut ret = 0;
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    let mut extra_pins = 0;
    delete_from_lru_cache(folio);
//
// For anonymous folios the only reference left
// should be the one m_f() holds.
//
    if (folio_test_anon(folio)) {
    ret = MF_RECOVERED;
// goto;
    }
//
// Now truncate the page in the page cache. This is really
// more like a "temporary hole punch"
// Don't do this for block devices when someone else
// has a reference, because it could be file system metadata
// and that's not safe to truncate.
//
    mapping = folio_mapping(folio);
    if (!mapping) {
// Folio has been torn down in the meantime
    ret = MF_FAILED;
// goto;
    }
//
// The shmem page is kept in page cache instead of truncating
// so is expected to have an extra refcount after error-handling.
//
    extra_pins = shmem_mapping(mapping);
//
// Truncation is a bit tricky. Enable it per file system for now.
//
// Open: to take i_rwsem or not for this? Right now we don't.
//
    ret = truncate_error_folio(folio, page_to_pfn(p), mapping);
    if (has_extra_refcount(ps, p, extra_pins)) {
    ret = MF_FAILED;
    }
// label;
    folio_unlock(folio);
    return ret;
    }
//
// Dirty pagecache page
// Issues: when the error hit a hole page the error is not properly
// propagated.
//
#[no_mangle]
unsafe extern "C" fn me_pagecache_dirty(ps: *mut page_state, p: *mut page) -> c_int {
    let mut folio = page_folio(p);
    let mut mapping = folio_mapping(folio);
// TBD: print more information about the file.
    if (mapping) {
//
// IO error will be reported by write(), fsync(), etc.
// who check the mapping.
// This way the application knows that something went
// wrong with its dirty file data.
//
    mapping_set_error(mapping, -EIO);
    }
    return me_pagecache_clean(ps, p);
    }
//
// Clean and dirty swap cache.
//
// Dirty swap cache page is tricky to handle. The page could live both in page
// table and swap cache(ie. page is freshly swapped in). So it could be
// referenced concurrently by 2 types of PTEs:
// normal PTEs and swap PTEs. We try to handle them consistently by calling
// try_to_unmap(!TTU_HWPOISON) to convert the normal PTEs to swap PTEs,
// and then
// - clear dirty bit to prevent IO
// - remove from LRU
// - but keep in the swap cache, so that when we return to it on
// a later page fault, we know the application is accessing
// corrupted data and shall be killed (we installed simple
// interception code in do_swap_page to catch it).
//
// Clean swap cache pages can be directly isolated. A later page fault will
// bring in the known good data from disk.
//
#[no_mangle]
unsafe extern "C" fn me_swapcache_dirty(ps: *mut page_state, p: *mut page) -> c_int {
    let mut folio = page_folio(p);
    let mut ret = 0;
pub static mut extra_pins: bool = false;
    folio_clear_dirty(folio);
// Trigger EIO in shmem:
    folio_clear_uptodate(folio);
    ret = delete_from_lru_cache(folio) ? MF_FAILED : MF_DELAYED;
    folio_unlock(folio);
    if (ret == MF_DELAYED) {
    extra_pins = true;
    }
    if (has_extra_refcount(ps, p, extra_pins)) {
    ret = MF_FAILED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn me_swapcache_clean(ps: *mut page_state, p: *mut page) -> c_int {
    let mut folio = page_folio(p);
    let mut ret = 0;
    swap_cache_del_folio(folio);
    ret = delete_from_lru_cache(folio) ? MF_FAILED : MF_RECOVERED;
    folio_unlock(folio);
    if (has_extra_refcount(ps, p, false)) {
    ret = MF_FAILED;
    }
    return ret;
    }
//
// Huge pages. Needs work.
// Issues:
// - Error on hugepage is contained in hugepage unit (not in raw page unit.)
// To narrow down kill region to one page, we need to break up pmd.
//
#[no_mangle]
unsafe extern "C" fn me_huge_page(ps: *mut page_state, p: *mut page) -> c_int {
    let mut folio = page_folio(p);
    let mut res = 0;
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut extra_pins: bool = false;
    mapping = folio_mapping(folio);
    if (mapping) {
    res = truncate_error_folio(folio, page_to_pfn(p), mapping);
// The page is kept in page cache.
    extra_pins = true;
    folio_unlock(folio);
    } else {
    folio_unlock(folio);
//
// migration entry prevents later access on error hugepage,
// so we can free and dissolve it into buddy to save healthy
// subpages.
//
    folio_put(folio);
    if (__page_handle_poison(p) > 0) {
    page_ref_inc(p);
    res = MF_RECOVERED;
    } else {
    res = MF_FAILED;
    }
    }
    if (has_extra_refcount(ps, p, extra_pins)) {
    res = MF_FAILED;
    }
    return res;
    }
//
// Various page states we can handle.
//
// A page state is defined by its current page->flags bits.
// The table matches them in order and calls the right handler.
//
// This is quite tricky because we can access page at any time
// in its live cycle, so all accesses have to be extremely careful.
//
// This is not complete. More states could be added.
// For any missing state don't attempt recovery.
//

pub static mut page_state: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn update_per_node_mf_stats(pfn: c_ulong, result: mf_result) {
pub static mut nid: c_int = 0;
    let mut mf_stats = core::ptr::null_mut();
    nid = pfn_to_nid(pfn);
    if (unlikely(nid < 0 || nid >= MAX_NUMNODES)) {
    WARN_ONCE(1, "Memory failure: pfn=%#lx, invalid nid=%d", pfn, nid);
    return;
    }
    mf_stats = &NODE_DATA(nid).mf_stats;
    match (result) {
    MF_IGNORED => {
    ++mf_stats.ignored;
    // break;
    }
    MF_FAILED => {
    ++mf_stats.failed;
    // break;
    }
    MF_DELAYED => {
    ++mf_stats.delayed;
    // break;
    }
    MF_RECOVERED => {
    ++mf_stats.recovered;
    // break;
    }
    _ => {
    WARN_ONCE(1, "Memory failure: mf_result=%d is not properly handled", result);
    // break;
    }
    }
    ++mf_stats.total;
    }
#[no_mangle]
pub unsafe extern "C" fn panic_on_unrecoverable_mf(type: mf_action_page_type, result: mf_result) -> bool {
    if (!sysctl_panic_on_unrecoverable_mf) {
    return false;
    }
pub static mut type: return = 0;
    }
//
// "Dirty/Clean" indication is not 100% accurate due to the possibility of
// setting PG_dirty outside page lock. See also comment above set_page_dirty().
//
#[no_mangle]
pub unsafe extern "C" fn action_result(pfn: c_ulong, type: mf_action_page_type, result: mf_result) -> c_int {
    trace_memory_failure_event(pfn, type, result);
    if (type != MF_MSG_ALREADY_POISONED && type != MF_MSG_PFN_MAP) {
    num_poisoned_pages_inc(pfn);
    update_per_node_mf_stats(pfn, result);
    }
    pr_err!("%#lx: recovery action for %s: %s\n",
    pfn, action_page_types[type], action_name[result]);
    if (panic_on_unrecoverable_mf(type, result)) {
    panic("Memory failure: %#lx: unrecoverable page", pfn);
    }
    return (result == MF_RECOVERED || result == MF_DELAYED) ? 0 : -EBUSY;
    }
#[no_mangle]
pub unsafe extern "C" fn page_action(ps: *mut page_state, p: *mut page, pfn: c_ulong) -> c_int {
    let mut result = 0;
// page p should be unlocked after returning from ps->action().
    result = ps.action(ps, p);
// Could do more checks here if page looks ok
//
// Could adjust zone counters here to correct for the missing page.
//
    return action_result(pfn, ps.type, result);
    }
#[no_mangle]
pub unsafe extern "C" fn PageHWPoisonTakenOff(page: *mut page) -> bool {
    return PageHWPoison(page) && page_private(page) == MAGIC_HWPOISON;
    }
#[no_mangle]
pub unsafe extern "C" fn SetPageHWPoisonTakenOff(page: *mut page) {
    set_page_private(page, MAGIC_HWPOISON);
    }
#[no_mangle]
pub unsafe extern "C" fn ClearPageHWPoisonTakenOff(page: *mut page) {
    if (PageHWPoison(page)) {
    set_page_private(page, 0);
    }
    }
//
// Return true if a page type of a given page is supported by hwpoison
// mechanism (while handling could fail), otherwise false.  This function
// does not return true for hugetlb or device memory pages, so it's assumed
// to be called only in the context where we never have such pages.
//
#[no_mangle]
pub unsafe extern "C" fn HWPoisonHandlable(page: *mut page, flags: c_ulong) -> bool {
    if (PageSlab(page)) {
    return false;
    }
// Soft offline could migrate movable_ops pages
    if ((flags & MF_SOFT_OFFLINE) && page_has_movable_ops(page)) {
    return true;
    }
    return PageLRU(page) || is_free_buddy_page(page);
    }
//
// Positive identification of pages the hwpoison handler cannot recover:
// pages owned by kernel internals with no userspace mapping to unmap, no
// file mapping to invalidate, and no migration target.
//
#[no_mangle]
pub unsafe extern "C" fn is_kernel_owned_page(page: *mut page) -> bool {
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut kernel_owned = 0;
// PG_reserved is a per-page flag, never set on a compound page.
    if (PageReserved(page)) {
    return true;
    }
//
// Page-type bits live only on the head page, so resolve any tail
// first.  The check takes no refcount; recheck the head afterwards
// so a concurrent split or compound free cannot leave us trusting
// a stale view.  A residual free->alloc->free cannot be closed here
// (frozen slab and large-kmalloc pages cannot be pinned), but is
// harmless: where a wrong verdict could panic, memory_failure() has
// already set PageHWPoison, which bars the page from the allocator.
//
// label;
    head = compound_head(page);
    kernel_owned = PageSlab(head) || PageTable(head) ||
    PageLargeKmalloc(head);
    if (head != compound_head(page)) {
// goto;
    }
    return kernel_owned;
    }
#[no_mangle]
unsafe extern "C" fn __get_hwpoison_page(page: *mut page, flags: c_ulong) -> c_int {
    let mut folio = page_folio(page);
pub static mut ret: c_int = 0;
pub static mut hugetlb: bool = false;
    ret = get_hwpoison_hugetlb_folio(folio, &hugetlb, false);
    if (hugetlb) {
// Make sure hugetlb demotion did not happen from under us.
    if (folio == page_folio(page)) {
    return ret;
    }
    if (ret > 0) {
    folio_put(folio);
    folio = page_folio(page);
    }
    }
//
// This check prevents from calling folio_try_get() for any
// unsupported type of folio in order to reduce the risk of unexpected
// races caused by taking a folio refcount.
//
    if (!HWPoisonHandlable(&folio.page, flags)) {
    return -EBUSY;
    }
    if (folio_try_get(folio)) {
    if (folio == page_folio(page)) {
    return 1;
    }
    pr_info!("%#lx cannot catch tail\n", page_to_pfn(page));
    folio_put(folio);
    }
    return 0;
    }
pub const GET_PAGE_MAX_RETRY_NUM: c_int = 3;
#[no_mangle]
unsafe extern "C" fn get_any_page(p: *mut page, flags: c_ulong) -> c_int {
pub static mut ret: c_int = 0;
pub static mut count_increased: bool = false;
    if (flags & MF_COUNT_INCREASED) {
    count_increased = true;
    }
//
// Page types we know are kernel-owned and cannot be recovered.
// Short-circuit before the shake_page() / retry loop, which
// cannot turn any of these into something HWPoisonHandlable().
// Drop the caller's reference if MF_COUNT_INCREASED took one.
//
    if (is_kernel_owned_page(p)) {
    if (count_increased) {
    put_page(p);
    }
    ret = -ENOTRECOVERABLE;
// goto;
    }
// label;
    if (!count_increased) {
    ret = __get_hwpoison_page(p, flags);
    if (!ret) {
    if (page_count(p)) {
// We raced with an allocation, retry.
    if (pass++ < GET_PAGE_MAX_RETRY_NUM) {
// goto;
    }
    ret = -EBUSY;
    } else if (!PageHuge(p) && !is_free_buddy_page(p)) {
// We raced with put_page, retry.
    if (pass++ < GET_PAGE_MAX_RETRY_NUM) {
// goto;
    }
    ret = -EIO;
    }
// goto;
    } else if (ret == -EBUSY) {
//
// We raced with (possibly temporary) unhandlable
// page, retry.
//
    if (pass++ < GET_PAGE_MAX_RETRY_NUM) {
    shake_page(p);
// goto;
    }
    ret = -EIO;
// goto;
    }
    }
    if (PageHuge(p) || HWPoisonHandlable(p, flags)) {
    ret = 1;
    } else {
//
// A page we cannot handle. Check whether we can turn
// it into something we can handle.
//
    if (pass++ < GET_PAGE_MAX_RETRY_NUM) {
    put_page(p);
    shake_page(p);
    count_increased = false;
// goto;
    }
    put_page(p);
    ret = -EIO;
    }
// label;
    if (ret == -EIO || ret == -ENOTRECOVERABLE) {
    pr_err!("%#lx: unhandlable page.\n", page_to_pfn(p));
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __get_unpoison_page(page: *mut page) -> c_int {
    let mut folio = page_folio(page);
pub static mut ret: c_int = 0;
pub static mut hugetlb: bool = false;
    ret = get_hwpoison_hugetlb_folio(folio, &hugetlb, true);
    if (hugetlb) {
// Make sure hugetlb demotion did not happen from under us.
    if (folio == page_folio(page)) {
    return ret;
    }
    if (ret > 0) {
    folio_put(folio);
    }
    }
//
// PageHWPoisonTakenOff pages are not only marked as PG_hwpoison,
// but also isolated from buddy freelist, so need to identify the
// state and have to cancel both operations to unpoison.
//
    if (PageHWPoisonTakenOff(page)) {
    return -EHWPOISON;
    }
    return get_page_unless_zero(page) ? 1 : 0;
    }
//
// get_hwpoison_page() - Get refcount for memory error handling
// @p:		Raw error page (hit by memory error)
// @flags:	Flags controlling behavior of error handling
//
// get_hwpoison_page() takes a page refcount of an error page to handle memory
// error on it, after checking that the error page is in a well-defined state
// (defined as a page-type we can successfully handle the memory error on it,
// such as LRU page and hugetlb page).
//
// Memory error handling could be triggered at any time on any type of page,
// so it's prone to race with typical memory management lifecycle (like
// allocation and free).  So to avoid such races, get_hwpoison_page() takes
// extra care for the error page's state (as done in __get_hwpoison_page()),
// and has some retry logic in get_any_page().
//
// When called from unpoison_memory(), the caller should already ensure that
// the given page has PG_hwpoison. So it's never reused for other page
// allocations, and __get_unpoison_page() never races with them.
//
// Return: 0 on failure or free buddy (hugetlb) page,
// 1 on success for in-use pages in a well-defined state,
// -EIO for pages on which we can not handle memory errors,
// -EBUSY when get_hwpoison_page() has raced with page lifecycle
// operations like allocation and free,
// -EHWPOISON when the page is hwpoisoned and taken off from buddy,
// -ENOTRECOVERABLE for kernel-owned pages identified by
// is_kernel_owned_page() (PG_reserved, slab,
// page-table, large-kmalloc) that the handler cannot recover.
//
#[no_mangle]
unsafe extern "C" fn get_hwpoison_page(p: *mut page, flags: c_ulong) -> c_int {
    let mut ret = 0;
    zone_pcp_disable(page_zone(p));
    if (flags & MF_UNPOISON) {
    ret = __get_unpoison_page(p);
    }
    else {
    ret = get_any_page(p, flags);
    }
    zone_pcp_enable(page_zone(p));
    return ret;
    }
//
// The caller must guarantee the folio isn't large folio, except hugetlb.
// try_to_unmap() can't handle it.
//
#[no_mangle]
pub unsafe extern "C" fn unmap_poisoned_folio(folio: *mut folio, pfn: c_ulong, must_kill: bool) -> c_int {
pub static mut ttu: ttu_flags = 0;
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    if (folio_test_swapcache(folio)) {
    pr_err!("%#lx: keeping poisoned page in swap cache\n", pfn);
    ttu &= ~TTU_HWPOISON;
    }
//
// Propagate the dirty bit from PTEs to struct page first, because we
// need this to decide if we should kill or just drop the page.
// XXX: the dirty test could be racy: set_page_dirty() may not always
// be called inside page lock (it's recommended but not enforced).
//
    mapping = folio_mapping(folio);
    if (!must_kill && !folio_test_dirty(folio) && mapping &&
    mapping_can_writeback(mapping)) {
    if (folio_mkclean(folio)) {
    folio_set_dirty(folio);
    } else {
    ttu &= ~TTU_HWPOISON;
    pr_info!("%#lx: corrupted page was clean: dropped without side effects\n",
    pfn);
    }
    }
    if (folio_test_hugetlb(folio) && !folio_test_anon(folio)) {
//
// For hugetlb folios in shared mappings, try_to_unmap
// could potentially call huge_pmd_unshare.  Because of
// this, take semaphore in write mode here and set
// TTU_RMAP_LOCKED to indicate we have taken the lock
// at this higher level.
//
    mapping = hugetlb_folio_mapping_lock_write(folio);
    if (!mapping) {
    pr_info!("%#lx: could not lock mapping for mapped hugetlb folio\n",
    folio_pfn(folio));
    return -EBUSY;
    }
    try_to_unmap(folio, ttu|TTU_RMAP_LOCKED);
    i_mmap_unlock_write(mapping);
    } else {
    try_to_unmap(folio, ttu);
    }
    return folio_mapped(folio) ? -EBUSY : 0;
    }
//
// Do all that is necessary to remove user space mappings. Unmap
// the pages and send SIGBUS to the processes if the data was dirty.
//
#[no_mangle]
pub unsafe extern "C" fn hwpoison_user_mappings(folio: *mut folio, p: *mut page, pfn: c_ulong, flags: c_int) -> bool {
pub static mut tokill: usize = 0;
    let mut unmap_success = 0;
    let mut forcekill = 0;
pub static mut mlocked: bool = false;
//
// Here we are interested only in user-mapped pages, so skip any
// other types of pages.
//
    if (folio_test_reserved(folio) || folio_test_slab(folio) ||
    folio_test_pgtable(folio) || folio_test_offline(folio)) {
    return true;
    }
    if (!(folio_test_lru(folio) || folio_test_hugetlb(folio))) {
    return true;
    }
//
// This check implies we don't kill processes if their pages
// are in the swap cache early. Those are always late kills.
//
    if (!folio_mapped(folio)) {
    return true;
    }
//
// First collect all the processes that have the page
// mapped in dirty form.  This has to be done before try_to_unmap,
// because ttu takes the rmap data structures down.
//
    collect_procs(folio, p, &tokill, flags & MF_ACTION_REQUIRED);
    unmap_success = !unmap_poisoned_folio(folio, pfn, flags & MF_MUST_KILL);
    if (!unmap_success) {
    pr_err!("%#lx: failed to unmap page (folio mapcount=%d)\n",
    pfn, folio_mapcount(folio));
    }
//
// try_to_unmap() might put mlocked page in lru cache, so call
// shake_page() again to ensure that it's flushed.
//
    if (mlocked) {
    shake_folio(folio);
    }
//
// Now that the dirty bit has been propagated to the
// struct page and all unmaps done we can decide if
// killing is needed or not.  Only kill when the page
// was dirty or the process is not restartable,
// otherwise the tokill list is merely
// freed.  When there was a problem unmapping earlier
// use a more force-full uncatchable kill to prevent
// any accesses to the poisoned memory.
//
    forcekill = folio_test_dirty(folio) || (flags & MF_MUST_KILL) ||
    !unmap_success;
    kill_procs(&tokill, forcekill, pfn, flags);
    return unmap_success;
    }
#[no_mangle]
pub unsafe extern "C" fn identify_page_state(pfn: c_ulong, p: *mut page, page_flags: c_ulong) -> c_int {
pub static mut ps: *mut c_void = core::ptr::null_mut();
//
// The first check uses the current page flags which may not have any
// relevant information. The second check with the saved page flags is
// carried out only if the first check can't determine the page status.
//
    for (ps = error_states;; ps++) {
    if ((p.flags.f & ps.mask) == ps.res)
    break;
    }
    page_flags |= (p.flags.f & (1UL << PG_dirty));
    if (!ps.mask) {
    for (ps = error_states;; ps++)
    }
    if ((page_flags & ps.mask) == ps.res) {
    break;
    }
    return page_action(ps, p, pfn);
    }
//
// When 'release' is 'false', it means that if thp split has failed,
// there is still more to do, hence the page refcount we took earlier
// is still needed.
//
#[no_mangle]
pub unsafe extern "C" fn try_to_split_thp_page(page: *mut page, new_order: c_uint, release: bool) -> c_int {
    let mut ret = 0;
    lock_page(page);
    ret = split_huge_page_to_order(page, new_order);
    unlock_page(page);
    if (ret && release) {
    put_page(page);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unmap_and_kill(to_kill: *mut list_head, pfn: c_ulong, mapping: *mut address_space, index: pgoff_t, flags: c_int) {
pub static mut tk: *mut c_void = core::ptr::null_mut();
pub static mut size: c_ulong = 0;
    list_for_each_entry(tk, to_kill, nd) {
    if (tk.size_shift)
    size = max(size, 1UL << tk.size_shift);
    }
    if (size) {
//
// Unmap the largest mapping to avoid breaking up device-dax
// mappings which are constant size. The actual size of the
// mapping being torn down is communicated in siginfo, see
// kill_proc()
//
pub static mut start: loff_t = 0;
    unmap_mapping_range(mapping, start, size, 0);
    }
    kill_procs(to_kill, !!(flags & MF_MUST_KILL), pfn, flags);
    }
//
// Only dev_pagemap pages get here, such as fsdax when the filesystem
// either do not claim or fails to claim a hwpoison event, or devdax.
// The fsdax pages are initialized per base page, and the devdax pages
// could be initialized either as base pages, or as compound pages with
// vmemmap optimization enabled. Devdax is simplistic in its dealing with
// hwpoison, such that, if a subpage of a compound page is poisoned,
// simply mark the compound head page is by far sufficient.
//
#[no_mangle]
pub unsafe extern "C" fn mf_generic_kill_procs(pfn: unsigned long long, flags: c_int, pgmap: *mut dev_pagemap) -> c_int {
    let mut folio = pfn_folio(pfn);
pub static mut to_kill: usize = 0;
    let mut cookie;
pub static mut rc: c_int = 0;
//
// Prevent the inode from being freed while we are interrogating
// the address_space, typically this would be handled by
// lock_page(), but dax pages do not use the page lock. This
// also prevents changes to the mapping of this pfn until
// poison signaling is complete.
//
    cookie = dax_lock_folio(folio);
    if (!cookie) {
    return -EBUSY;
    }
    if (hwpoison_filter(&folio.page)) {
    rc = -EOPNOTSUPP;
// goto;
    }
    match (pgmap.type) {
    MEMORY_DEVICE_PRIVATE => {
    }
    MEMORY_DEVICE_COHERENT => {
//
// TODO: Handle device pages which may need coordination
// with device-side memory.
//
    rc = -ENXIO;
// goto;
    }
    _ => {
    // break;
    }
    }
//
// Use this flag as an indication that the dax page has been
// remapped UC to prevent speculative consumption of poison.
//
    SetPageHWPoison(&folio.page);
//
// Unlike System-RAM there is no possibility to swap in a
// different physical page at a given virtual address, so all
// userspace consumption of ZONE_DEVICE memory necessitates
// SIGBUS (i.e. MF_MUST_KILL)
//
    flags |= MF_ACTION_REQUIRED | MF_MUST_KILL;
    collect_procs(folio, &folio.page, &to_kill, true);
    unmap_and_kill(&to_kill, pfn, folio.mapping, folio.index, flags);
// label;
    dax_unlock_folio(folio, cookie);
    return rc;
    }

//
// mf_dax_kill_procs - Collect and kill processes who are using this file range
// @mapping:	address_space of the file in use
// @index:	start pgoff of the range within the file
// @count:	length of the range, in unit of PAGE_SIZE
// @mf_flags:	memory failure flags
//
#[no_mangle]
pub unsafe extern "C" fn mf_dax_kill_procs(mapping: *mut address_space, index: pgoff_t, count: c_ulong, mf_flags: c_int) -> c_int {
pub static mut to_kill: usize = 0;
    let mut cookie;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut end: usize = 0;
pub static mut pre_remove: bool = false;
    mf_flags |= MF_ACTION_REQUIRED | MF_MUST_KILL;
    while (index < end) {
    page = core::ptr::null_mut();
    cookie = dax_lock_mapping_entry(mapping, index, &page);
    if (!cookie) {
    return -EBUSY;
    }
    if (!page) {
// goto;
    }
    if (!pre_remove) {
    SetPageHWPoison(page);
    }
//
// The pre_remove case is revoking access, the memory is still
// good and could theoretically be put back into service.
//
    collect_procs_fsdax(page, mapping, index, &to_kill, pre_remove);
    unmap_and_kill(&to_kill, page_to_pfn(page), mapping,
    index, mf_flags);
// label;
    dax_unlock_mapping_entry(mapping, index, cookie);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(mf_dax_kill_procs);

//
// Struct raw_hwp_page represents information about "raw error page",
// constructing singly linked list from ->_hugetlb_hwpoison field of folio.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_hwp_page {
    pub node: llist_node,
    pub page: *mut page,
}

#[no_mangle]
pub unsafe extern "C" fn raw_hwp_list_head(folio: *mut folio) -> *mut c_void {
    return &folio._hugetlb_hwpoison;
    }
#[no_mangle]
pub unsafe extern "C" fn is_raw_hwpoison_page_in_hugepage(page: *mut page) -> bool {
pub static mut raw_hwp_head: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut folio = page_folio(page);
pub static mut ret: bool = false;
    if (!folio_test_hwpoison(folio)) {
    return false;
    }
    if (!folio_test_hugetlb(folio)) {
    return PageHWPoison(page);
    }
//
// When RawHwpUnreliable is set, kernel lost track of which subpages
// are HWPOISON. So return as if ALL subpages are HWPOISONed.
//
    if (folio_test_hugetlb_raw_hwp_unreliable(folio)) {
    return true;
    }
    mutex_lock(&mf_mutex);
    raw_hwp_head = raw_hwp_list_head(folio);
    llist_for_each_entry(p, raw_hwp_head.first, node) {
    if (page == p.page) {
    ret = true;
    break;
    }
    }
    mutex_unlock(&mf_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __folio_free_raw_hwp(folio: *mut folio, move_flag: bool) -> c_ulong {
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut count: c_ulong = 0;
    head = llist_del_all(raw_hwp_list_head(folio));
    llist_for_each_entry_safe(p, next, head, node) {
    if (move_flag) {
    SetPageHWPoison(p.page);
    }
    else {
    num_poisoned_pages_sub(page_to_pfn(p.page), 1);
    }
    kfree(p);
    count += 1;
    }
    return count;
    }

//
// Set hugetlb folio as hwpoisoned, update folio private raw hwpoison list
// to keep track of the poisoned pages.
//
#[no_mangle]
unsafe extern "C" fn hugetlb_update_hwpoison(folio: *mut folio, page: *mut page) -> c_int {
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut raw_hwp: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// Once the hwpoison hugepage has lost reliable raw error info,
// there is little meaning to keep additional error info precisely,
// so skip to add additional raw error info.
//
    if (folio_test_hugetlb_raw_hwp_unreliable(folio)) {
    return MF_HUGETLB_FOLIO_PRE_POISONED;
    }
    head = raw_hwp_list_head(folio);
    llist_for_each_entry(p, head.first, node) {
    if (p.page == page) {
    return MF_HUGETLB_PAGE_PRE_POISONED;
    }
    }
    raw_hwp = kmalloc_obj(raw_hwp_page, GFP_ATOMIC);
    if (raw_hwp) {
    raw_hwp.page = page;
    llist_add(&raw_hwp.node, head);
    } else {
//
// Failed to save raw error info.  We no longer trace all
// hwpoisoned subpages, and we need refuse to free/dissolve
// this hwpoisoned hugepage.
//
    folio_set_hugetlb_raw_hwp_unreliable(folio);
//
// Once hugetlb_raw_hwp_unreliable is set, raw_hwp_page is not
// used any more, so free it.
//
    __folio_free_raw_hwp(folio, false);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn folio_free_raw_hwp(folio: *mut folio, move_flag: bool) -> c_ulong {
//
// hugetlb_vmemmap_optimized hugepages can't be freed because struct
// pages for tail pages are required but they don't exist.
//
    if (move_flag && folio_test_hugetlb_vmemmap_optimized(folio)) {
    return 0;
    }
//
// hugetlb_raw_hwp_unreliable hugepages shouldn't be unpoisoned by
// definition.
//
    if (folio_test_hugetlb_raw_hwp_unreliable(folio)) {
    return 0;
    }
    return __folio_free_raw_hwp(folio, move_flag);
    }
#[no_mangle]
pub unsafe extern "C" fn folio_clear_hugetlb_hwpoison(folio: *mut folio) {
    if (folio_test_hugetlb_raw_hwp_unreliable(folio)) {
    return;
    }
    if (folio_test_hugetlb_vmemmap_optimized(folio)) {
    return;
    }
    folio_clear_hwpoison(folio);
    folio_free_raw_hwp(folio, true);
    }
#[no_mangle]
pub unsafe extern "C" fn get_huge_page_for_hwpoison(pfn: c_ulong, flags: c_int, migratable_cleared: *mut bool) -> c_int {
    let mut page = pfn_to_page(pfn);
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut count_increased: bool = false;
    let mut ret = 0;
    let mut rc = 0;
    spin_lock_irq(&hugetlb_lock);
    folio = page_folio(page);
    if (!folio_test_hugetlb(folio)) {
    ret = MF_HUGETLB_NON_HUGEPAGE;
// goto;
    } else if (flags & MF_COUNT_INCREASED) {
    ret = MF_HUGETLB_IN_USED;
    count_increased = true;
    } else if (folio_test_hugetlb_freed(folio)) {
    ret = MF_HUGETLB_FREED;
    } else if (folio_test_hugetlb_migratable(folio)) {
    if (folio_try_get(folio)) {
    ret = MF_HUGETLB_IN_USED;
    count_increased = true;
    } else {
    ret = MF_HUGETLB_FREED;
    }
    } else {
    ret = MF_HUGETLB_RETRY;
    if (!(flags & MF_NO_RETRY)) {
// goto;
    }
    }
    rc = hugetlb_update_hwpoison(folio, page);
    if (rc >= MF_HUGETLB_FOLIO_PRE_POISONED) {
    ret = rc;
// goto;
    }
//
// Clearing hugetlb_migratable for hwpoisoned hugepages to prevent them
// from being migrated by memory hotremove.
//
    if (count_increased && folio_test_hugetlb_migratable(folio)) {
    folio_clear_hugetlb_migratable(folio);
// migratable_cleared = true;
    }
    spin_unlock_irq(&hugetlb_lock);
    return ret;
// label;
    spin_unlock_irq(&hugetlb_lock);
    if (count_increased) {
    folio_put(folio);
    }
    return ret;
    }
//
// Taking refcount of hugetlb pages needs extra care about race conditions
// with basic operations like hugepage allocation/free/demotion.
// So some of prechecks for hwpoison (pinning, and testing/setting
// PageHWPoison) should be done in single hugetlb_lock range.
// Returns:
// 0		- recovered
// -ENOENT		- no hugetlb page
// -EBUSY		- not recovered
// -EOPNOTSUPP	- hwpoison_filter'ed
// -EHWPOISON	- folio or exact page already poisoned
// -EFAULT		- kill_accessing_process finds current->mm null
//
#[no_mangle]
unsafe extern "C" fn try_memory_failure_hugetlb(pfn: c_ulong, flags: c_int) -> c_int {
    let mut res = 0;
    let mut rv = 0;
    let mut p = pfn_to_page(pfn);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut page_flags = 0;
pub static mut migratable_cleared: bool = false;
// label;
    res = get_huge_page_for_hwpoison(pfn, flags, &migratable_cleared);
    match (res) {
    MF_HUGETLB_NON_HUGEPAGE => {
    return -ENOENT;
    }
    MF_HUGETLB_RETRY => {
    if (!(flags & MF_NO_RETRY)) {
    flags |= MF_NO_RETRY;
// goto;
    }
    return action_result(pfn, MF_MSG_GET_HWPOISON, MF_IGNORED);
    }
    MF_HUGETLB_FOLIO_PRE_POISONED => {
    }
    MF_HUGETLB_PAGE_PRE_POISONED => {
    rv = -EHWPOISON;
    if (flags & MF_ACTION_REQUIRED) {
    rv = kill_accessing_process(current, pfn, flags);
    }
    if (res == MF_HUGETLB_PAGE_PRE_POISONED) {
    action_result(pfn, MF_MSG_ALREADY_POISONED, MF_FAILED);
    }
    else {
    action_result(pfn, MF_MSG_HUGE, MF_FAILED);
    }
    return rv;
    }
    _ => {
    WARN_ON!((res != MF_HUGETLB_FREED) && (res != MF_HUGETLB_IN_USED));
    // break;
    }
    }
    folio = page_folio(p);
    folio_lock(folio);
    if (hwpoison_filter(p)) {
    folio_clear_hugetlb_hwpoison(folio);
    if (migratable_cleared) {
    folio_set_hugetlb_migratable(folio);
    }
    folio_unlock(folio);
    if (res == MF_HUGETLB_IN_USED) {
    folio_put(folio);
    }
    return -EOPNOTSUPP;
    }
//
// Handling free hugepage.  The possible race with hugepage allocation
// or demotion can be prevented by PageHWPoison flag.
//
    if (res == MF_HUGETLB_FREED) {
    folio_unlock(folio);
    if (__page_handle_poison(p) > 0) {
    page_ref_inc(p);
    res = MF_RECOVERED;
    } else {
    res = MF_FAILED;
    }
    return action_result(pfn, MF_MSG_FREE_HUGE, res);
    }
    page_flags = folio.flags.f;
    if (!hwpoison_user_mappings(folio, p, pfn, flags)) {
    folio_unlock(folio);
    return action_result(pfn, MF_MSG_UNMAP_FAILED, MF_FAILED);
    }
    return identify_page_state(pfn, p, page_flags);
    }

#[no_mangle]
pub unsafe extern "C" fn try_memory_failure_hugetlb(pfn: c_ulong, flags: c_int) -> c_int {
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_free_raw_hwp(folio: *mut folio, flag: bool) -> c_ulong {
    return 0;
    }

// Drop the extra refcount in case we come from madvise()
#[no_mangle]
unsafe extern "C" fn put_ref_page(pfn: c_ulong, flags: c_int) {
    if (!(flags & MF_COUNT_INCREASED)) {
    return;
    }
    put_page(pfn_to_page(pfn));
    }
#[no_mangle]
pub unsafe extern "C" fn memory_failure_dev_pagemap(pfn: c_ulong, flags: c_int, pgmap: *mut dev_pagemap) -> c_int {
pub static mut rc: c_int = 0;
// device metadata space is not recoverable
    if (!pgmap_pfn_valid(pgmap, pfn)) {
// goto;
    }
//
// Call driver's implementation to handle the memory failure, otherwise
// fall back to generic handler.
//
    if (pgmap_has_memory_failure(pgmap)) {
    rc = pgmap.ops.memory_failure(pgmap, pfn, 1, flags);
//
// Fall back to generic handler too if operation is not
// supported inside the driver/device/filesystem.
//
    if (rc != -EOPNOTSUPP) {
// goto;
    }
    }
    rc = mf_generic_kill_procs(pfn, flags, pgmap);
// label;
// drop pgmap ref acquired in caller
    put_dev_pagemap(pgmap);
    if (rc != -EOPNOTSUPP) {
    action_result(pfn, MF_MSG_DAX, rc ? MF_FAILED : MF_RECOVERED);
    }
    return rc;
    }
//
// The calling condition is as such: thp split failed, page might have
// been RDMA pinned, not much can be done for recovery.
// But a SIGBUS should be delivered with vaddr provided so that the user
// application has a chance to recover. Also, application processes'
// election for MCE early killed will be honored.
//
#[no_mangle]
pub unsafe extern "C" fn kill_procs_now(p: *mut page, pfn: c_ulong, flags: c_int, folio: *mut folio) {
pub static mut tokill: usize = 0;
    folio_lock(folio);
    collect_procs(folio, p, &tokill, flags & MF_ACTION_REQUIRED);
    folio_unlock(folio);
    kill_procs(&tokill, true, pfn, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn register_pfn_address_space(pfn_space: *mut pfn_address_space) -> c_int {
    guard(mutex)(&pfn_space_lock);
    if (!pfn_space.pfn_to_vma_pgoff) {
    return -EINVAL;
    }
    if (interval_tree_iter_first(&pfn_space_itree,
    pfn_space.node.start,
    pfn_space.node.last)) {
    return -EBUSY;
    }
// forward_decl: erval_tree_insert;
    return 0;
    }
    EXPORT_SYMBOL_GPL(register_pfn_address_space);
#[no_mangle]
pub unsafe extern "C" fn unregister_pfn_address_space(pfn_space: *mut pfn_address_space) {
    guard(mutex)(&pfn_space_lock);
    if (interval_tree_iter_first(&pfn_space_itree,
    pfn_space.node.start,
    pfn_space.node.last)) {
// forward_decl: erval_tree_remove;
    }
    }
    EXPORT_SYMBOL_GPL(unregister_pfn_address_space);
#[no_mangle]
pub unsafe extern "C" fn add_to_kill_pgoff(tsk: *mut task_struct, vma: *mut vm_area_struct, to_kill: *mut list_head, pgoff: pgoff_t) {
pub static mut tk: *mut c_void = core::ptr::null_mut();
    tk = kmalloc_obj(*tk, GFP_ATOMIC);
    if (!tk) {
    pr_info!("Unable to kill proc %d\n", tsk.pid);
    return;
    }
// Check for pgoff not backed by struct page
    tk.addr = vma_filebacked_address(vma, pgoff, 1);
    tk.size_shift = PAGE_SHIFT;
    if (tk.addr == -EFAULT) {
    pr_info!("Unable to find address %lx in %s\n",
    pgoff, tsk.comm);
    }
    get_task_struct(tsk);
    tk.tsk = tsk;
    list_add_tail(&tk.nd, to_kill);
    }
//
// Collect processes when the error hit a PFN not backed by struct page.
//
#[no_mangle]
pub unsafe extern "C" fn collect_procs_pfn(pfn_space: *mut pfn_address_space, pfn: c_ulong, to_kill: *mut list_head) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut tsk: *mut c_void = core::ptr::null_mut();
    let mut mapping = pfn_space.mapping;
    i_mmap_lock_read(mapping);
    rcu_read_lock();
    for_each_process(tsk) {
    let mut t = tsk;
    t = task_early_kill(tsk, true);
    if (!t) {
    continue;
    }
    mapping_rmap_tree_foreach(vma, mapping, 0, ULONG_MAX) {
    let mut pgoff;
    if (vma.vm_mm == t.mm &&
    !pfn_space.pfn_to_vma_pgoff(vma, pfn, &pgoff)) {
    add_to_kill_pgoff(t, vma, to_kill, pgoff);
    }
    }
    }
    rcu_read_unlock();
    i_mmap_unlock_read(mapping);
    }
//
// memory_failure_pfn - Handle memory failure on a page not backed by
// struct page.
// @pfn: Page Number of the corrupted page
// @flags: fine tune action taken
//
// Return:
// 0             - success,
// -EBUSY        - Page PFN does not belong to any address space mapping.
//
#[no_mangle]
unsafe extern "C" fn memory_failure_pfn(pfn: c_ulong, flags: c_int) -> c_int {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut tokill: usize = 0;
    scoped_guard(mutex, &pfn_space_lock) {
pub static mut mf_handled: bool = false;
//
// Modules registers with MM the address space mapping to
// the device memory they manage. Iterate to identify
// exactly which address space has mapped to this failing
// PFN.
//
    for (node = interval_tree_iter_first(&pfn_space_itree, pfn, pfn); node;
    node = interval_tree_iter_next(node, pfn, pfn)) {
    let mut pfn_space = container_of!(node, pfn_address_space, node);
    collect_procs_pfn(pfn_space, pfn, &tokill);
    mf_handled = true;
    }
    if (!mf_handled) {
    return action_result(pfn, MF_MSG_PFN_MAP, MF_IGNORED);
    }
    }
//
// Unlike System-RAM there is no possibility to swap in a different
// physical page at a given virtual address, so all userspace
// consumption of direct PFN memory necessitates SIGBUS (i.e.
// MF_MUST_KILL)
//
    flags |= MF_ACTION_REQUIRED | MF_MUST_KILL;
    kill_procs(&tokill, true, pfn, flags);
    return action_result(pfn, MF_MSG_PFN_MAP, MF_RECOVERED);
    }
//
// memory_failure - Handle memory failure of a page.
// @pfn: Page Number of the corrupted page
// @flags: fine tune action taken
//
// This function is called by the low level machine check code
// of an architecture when it detects hardware memory corruption
// of a page. It tries its best to recover, which includes
// dropping pages, killing processes etc.
//
// The function is primarily of use for corruptions that
// happen outside the current execution context (e.g. when
// detected by a background scrubber)
//
// Must run in process context (e.g. a work queue) with interrupts
// enabled and no spinlocks held.
//
// Return:
// 0             - success,
// -ENXIO        - memory not managed by the kernel
// -EOPNOTSUPP   - hwpoison_filter() filtered the error event,
// -EHWPOISON    - the page was already poisoned, potentially
// kill process,
// other negative values - failure.
//
#[no_mangle]
pub unsafe extern "C" fn memory_failure(pfn: c_ulong, flags: c_int) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut pgmap: *mut c_void = core::ptr::null_mut();
pub static mut res: c_int = 0;
    let mut page_flags = 0;
pub static mut retry: bool = true;
    if (!sysctl_memory_failure_recovery) {
    panic("Memory failure on page %lx", pfn);
    }
    mutex_lock(&mf_mutex);
    if (!(flags & MF_SW_SIMULATED)) {
    hw_memory_failure = true;
    }
    p = pfn_to_online_page(pfn);
    if (!p) {
    res = arch_memory_failure(pfn, flags);
    if (res == 0) {
// goto;
    }
    if (!pfn_valid(pfn) && !arch_is_platform_page(PFN_PHYS(pfn))) {
//
// The PFN is not backed by struct page.
//
    res = memory_failure_pfn(pfn, flags);
// goto;
    }
    if (pfn_valid(pfn)) {
    pgmap = get_dev_pagemap(pfn);
    put_ref_page(pfn, flags);
    if (pgmap) {
    res = memory_failure_dev_pagemap(pfn, flags,
    pgmap);
// goto;
    }
    }
    pr_err!("%#lx: memory outside kernel control\n", pfn);
    res = -ENXIO;
// goto;
    }
// label;
    res = try_memory_failure_hugetlb(pfn, flags);
//
// -ENOENT means the page we found is not hugetlb, so proceed with normal page handling
//
    if (res != -ENOENT) {
// goto;
    }
    if (TestSetPageHWPoison(p)) {
    res = -EHWPOISON;
    if (flags & MF_ACTION_REQUIRED) {
    res = kill_accessing_process(current, pfn, flags);
    }
    if (flags & MF_COUNT_INCREASED) {
    put_page(p);
    }
    action_result(pfn, MF_MSG_ALREADY_POISONED, MF_FAILED);
// goto;
    }
//
// We need/can do nothing about count=0 pages.
// 1) it's a free page, and therefore in safe hand:
// check_new_page() will be the gate keeper.
// 2) it's part of a non-compound high order page.
// Implies some kernel user: cannot stop them from
// R/W the page; let's pray that the page has been
// used and will be freed some time later.
// In fact it's dangerous to directly bump up page count from 0,
// that may make page_ref_freeze()/page_ref_unfreeze() mismatch.
//
    res = get_hwpoison_page(p, flags);
    match (res) {
    0 => {
    if (is_free_buddy_page(p)) {
    if (take_page_off_buddy(p)) {
    page_ref_inc(p);
    res = MF_RECOVERED;
    } else {
// We lost the race, try again
    if (retry) {
    ClearPageHWPoison(p);
    retry = false;
// goto;
    }
    res = MF_FAILED;
    }
    res = action_result(pfn, MF_MSG_BUDDY, res);
    } else {
    res = action_result(pfn, MF_MSG_KERNEL_HIGH_ORDER, MF_IGNORED);
    }
// goto;
    }
    1 => {
// Got a refcount on a handlable page.
    // break;
    }
    -ENOTRECOVERABLE => {
//
// Stable unhandlable kernel-owned page (PG_reserved,
// slab, page tables, large-kmalloc).
// No recovery possible.
//
    res = action_result(pfn, MF_MSG_KERNEL, MF_IGNORED);
// goto;
    }
    _ => {
// Transient lifecycle race with the page allocator.
    res = action_result(pfn, MF_MSG_GET_HWPOISON, MF_IGNORED);
// goto;
    }
    }
    folio = page_folio(p);
// filter pages that are protected from hwpoison test by users
    folio_lock(folio);
    if (hwpoison_filter(p)) {
    ClearPageHWPoison(p);
    folio_unlock(folio);
    folio_put(folio);
    res = -EOPNOTSUPP;
// goto;
    }
    folio_unlock(folio);
    if (folio_test_large(folio)) {
pub static mut new_order: c_int = 0;
    let mut err = 0;
//
// The flag must be set after the refcount is bumped
// otherwise it may race with THP split.
// And the flag can't be set in get_hwpoison_page() since
// it is called by soft offline too and it is just called
// for !MF_COUNT_INCREASED.  So here seems to be the best
// place.
//
// Don't need care about the above error handling paths for
// get_hwpoison_page() since they handle either free page
// or unhandlable page.  The refcount is bumped iff the
// page is a valid handlable page.
//
    folio_set_has_hwpoisoned(folio);
    err = try_to_split_thp_page(p, new_order, /* release= */ false);
//
// If splitting a folio to order-0 fails, kill the process.
// Split the folio regardless to minimize unusable pages.
// Because the memory failure code cannot handle large
// folios, this split is always treated as if it failed.
//
    if (err || new_order) {
// get folio again in case the original one is split
    folio = page_folio(p);
    res = -EHWPOISON;
    kill_procs_now(p, pfn, flags, folio);
    put_page(p);
    action_result(pfn, MF_MSG_UNSPLIT_THP, MF_FAILED);
// goto;
    }
    VM_BUG_ON_PAGE(!page_count(p), p);
    folio = page_folio(p);
    }
//
// We ignore non-LRU pages for good reasons.
// - PG_locked is only well defined for LRU pages and a few others
// - to avoid races with __SetPageLocked()
// - to avoid races with __SetPageSlab*() (and more non-atomic ops)
// The check (unnecessarily) ignores LRU pages being isolated and
// walked by the page reclaim code, however that's not a big loss.
//
    shake_folio(folio);
    folio_lock(folio);
//
// We're only intended to deal with the non-Compound page here.
// The page cannot become compound pages again as folio has been
// splited and extra refcnt is held.
//
    WARN_ON!(folio_test_large(folio));
//
// We use page flags to determine what action should be taken, but
// the flags can be modified by the error containment action.  One
// example is an mlocked page, where PG_mlocked is cleared by
// folio_remove_rmap_*() in try_to_unmap_one(). So to determine page
// status correctly, we save a copy of the page flags at this time.
//
    page_flags = folio.flags.f;
//
// __munlock_folio() may clear a writeback folio's LRU flag without
// the folio lock. We need to wait for writeback completion for this
// folio or it may trigger a vfs BUG while evicting inode.
//
    if (!folio_test_lru(folio) && !folio_test_writeback(folio)) {
// goto;
    }
//
// It's very difficult to mess with pages currently under IO
// and in many cases impossible, so we just avoid it here.
//
    folio_wait_writeback(folio);
//
// Now take care of user space mappings.
// Abort on fail: __filemap_remove_folio() assumes unmapped page.
//
    if (!hwpoison_user_mappings(folio, p, pfn, flags)) {
    res = action_result(pfn, MF_MSG_UNMAP_FAILED, MF_FAILED);
// goto;
    }
//
// Torn down by someone else?
//
    if (folio_test_lru(folio) && !folio_test_swapcache(folio) &&
    folio.mapping == core::ptr::null_mut()) {
    res = action_result(pfn, MF_MSG_TRUNCATED_LRU, MF_IGNORED);
// goto;
    }
// label;
    res = identify_page_state(pfn, p, page_flags);
    mutex_unlock(&mf_mutex);
    return res;
// label;
    folio_unlock(folio);
// label;
    mutex_unlock(&mf_mutex);
    return res;
    }
    EXPORT_SYMBOL_GPL(memory_failure);
pub const MEMORY_FAILURE_FIFO_ORDER: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_failure_entry {
    pub pfn: c_ulong,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_failure_cpu {
pub static mut fifo: usize = 0;
//
// memory_failure_queue - Schedule handling memory failure of a page.
// @pfn: Page Number of the corrupted page
// @flags: Flags for memory failure handling
//
// This function is called by the low level hardware error handler
// when it detects hardware memory corruption of a page. It schedules
// the recovering of error page, including dropping pages, killing
// processes etc.
//
// The function is primarily of use for corruptions that
// happen outside the current execution context (e.g. when
// detected by a background scrubber)
//
// Can run in IRQ context.
//
#[no_mangle]
pub unsafe extern "C" fn memory_failure_queue(pfn: c_ulong, flags: c_int) {
pub static mut mf_cpu: *mut c_void = core::ptr::null_mut();
    let mut proc_flags = 0;
    let mut buffer_overflow = 0;
pub static mut memory_failure_entry: usize = 0;
    mf_cpu = &get_cpu_var(memory_failure_cpu);
    raw_spin_lock_irqsave(&mf_cpu.lock, proc_flags);
    buffer_overflow = !kfifo_put(&mf_cpu.fifo, entry);
    if (!buffer_overflow) {
    schedule_work_on(smp_processor_id(), &mf_cpu.work);
    }
    raw_spin_unlock_irqrestore(&mf_cpu.lock, proc_flags);
    put_cpu_var(memory_failure_cpu);
    if (buffer_overflow) {
    pr_err!("buffer overflow when queuing memory failure at %#lx\n",
    pfn);
    }
    }
    EXPORT_SYMBOL_GPL(memory_failure_queue);
#[no_mangle]
unsafe extern "C" fn memory_failure_work_func(work: *mut work_struct) {
pub static mut mf_cpu: *mut c_void = core::ptr::null_mut();
pub static mut entry: memory_failure_entry = 0;
    let mut proc_flags = 0;
    let mut gotten = 0;
    mf_cpu = container_of!(work, memory_failure_cpu, work);
    for (;;) {
    raw_spin_lock_irqsave(&mf_cpu.lock, proc_flags);
    gotten = kfifo_get(&mf_cpu.fifo, &entry);
    raw_spin_unlock_irqrestore(&mf_cpu.lock, proc_flags);
    if (!gotten) {
    break;
    }
    if (entry.flags & MF_SOFT_OFFLINE) {
    soft_offline_page(entry.pfn, entry.flags);
    }
    else {
    memory_failure(entry.pfn, entry.flags);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn memory_failure_init() -> c_int {
pub static mut mf_cpu: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    mf_cpu = &per_cpu(memory_failure_cpu, cpu);
    raw_spin_lock_init(&mf_cpu.lock);
    INIT_KFIFO(mf_cpu.fifo);
    INIT_WORK(&mf_cpu.work, memory_failure_work_func);
    }
    register_sysctl_init("vm", memory_failure_table);
    return 0;
    }
    core_initcall!(memory_failure_init);

    ({							
    if (__ratelimit(rs))				 {
    pr_info!(fmt, pfn);			
    }
    })
//
// unpoison_memory - Unpoison a previously poisoned page
// @pfn: Page number of the to be unpoisoned page
//
// Software-unpoison a page that has been poisoned by
// memory_failure() earlier.
//
// This is only done on the software-level, so it only works
// for linux injected failures, not real hardware failures
//
// Returns 0 for success, otherwise -errno.
//
#[no_mangle]
pub unsafe extern "C" fn unpoison_memory(pfn: c_ulong) -> c_int {
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut count = 0;
pub static mut huge: bool = false;
pub static mut unpoison_rs: usize = 0;
    p = pfn_to_online_page(pfn);
    if (!p) {
    return -EIO;
    }
    folio = page_folio(p);
    mutex_lock(&mf_mutex);
    if (hw_memory_failure) {
    unpoison_pr_info("%#lx: disabled after HW memory failure\n",
    pfn, &unpoison_rs);
    ret = -EOPNOTSUPP;
// goto;
    }
    if (is_huge_zero_folio(folio)) {
    unpoison_pr_info("%#lx: huge zero page is not supported\n",
    pfn, &unpoison_rs);
    ret = -EOPNOTSUPP;
// goto;
    }
    if (!PageHWPoison(p)) {
    unpoison_pr_info("%#lx: page was already unpoisoned\n",
    pfn, &unpoison_rs);
// goto;
    }
    if (folio_ref_count(folio) > 1) {
    unpoison_pr_info("%#lx: someone grabs the hwpoison page\n",
    pfn, &unpoison_rs);
// goto;
    }
    if (folio_test_slab(folio) || folio_test_pgtable(folio) ||
    folio_test_reserved(folio) || folio_test_offline(folio)) {
// goto;
    }
    if (folio_mapped(folio)) {
    unpoison_pr_info("%#lx: someone maps the hwpoison page\n",
    pfn, &unpoison_rs);
// goto;
    }
    if (folio_mapping(folio)) {
    unpoison_pr_info("%#lx: the hwpoison page has non-core::ptr::null_mut() mapping\n",
    pfn, &unpoison_rs);
// goto;
    }
    ghp = get_hwpoison_page(p, MF_UNPOISON);
    if (!ghp) {
    if (folio_test_hugetlb(folio)) {
    huge = true;
    count = folio_free_raw_hwp(folio, false);
    if (count == 0) {
// goto;
    }
    }
    ret = folio_test_clear_hwpoison(folio) ? 0 : -EBUSY;
    } else if (ghp < 0) {
    if (ghp == -EHWPOISON) {
    ret = put_page_back_buddy(p) ? 0 : -EBUSY;
    } else {
    ret = ghp;
    unpoison_pr_info("%#lx: failed to grab page\n",
    pfn, &unpoison_rs);
    }
    } else {
    if (folio_test_hugetlb(folio)) {
    huge = true;
    count = folio_free_raw_hwp(folio, false);
    if (count == 0) {
    folio_put(folio);
// goto;
    }
    }
    folio_put(folio);
    if (TestClearPageHWPoison(p)) {
    folio_put(folio);
    ret = 0;
    }
    }
// label;
    mutex_unlock(&mf_mutex);
    if (!ret) {
    if (!huge) {
    num_poisoned_pages_sub(pfn, 1);
    }
    unpoison_pr_info("%#lx: software-unpoisoned page\n",
    page_to_pfn(p), &unpoison_rs);
    }
    return ret;
    }
    EXPORT_SYMBOL(unpoison_memory);

//
// soft_offline_in_use_page handles hugetlb-pages and non-hugetlb pages.
// If the page is a non-dirty unmapped page-cache page, it simply invalidates.
// If the page is mapped, it migrates the contents over.
//
#[no_mangle]
unsafe extern "C" fn soft_offline_in_use_page(page: *mut page) -> c_int {
pub static mut ret: c_long = 0;
pub static mut pfn: c_ulong = 0;
    let mut folio = page_folio(page);
    char const *msg_page[] = {"page", "hugepage"};
pub static mut huge: bool = false;
    let mut isolated = 0;
pub static mut pagelist: usize = 0;
pub static mut migration_target_control: usize = 0;
    if (!huge && folio_test_large(folio)) {
pub static mut new_order: c_int = 0;
//
// If new_order (target split order) is not 0, do not split the
// folio at all to retain the still accessible large folio.
// NOTE: if minimizing the number of soft offline pages is
// preferred, split it to non-zero new_order like it is done in
// memory_failure().
//
    if (new_order || try_to_split_thp_page(page, /* new_order= */ 0,
release= */ true)) {
    pr_info!("%#lx: thp split failed\n", pfn);
    return -EBUSY;
    }
    folio = page_folio(page);
    }
    folio_lock(folio);
    if (!huge) {
    folio_wait_writeback(folio);
    }
    if (PageHWPoison(page)) {
    folio_unlock(folio);
    folio_put(folio);
    pr_info!("%#lx: page already poisoned\n", pfn);
    return 0;
    }
    if (!huge && folio_test_lru(folio) && !folio_test_swapcache(folio)) {
//
// Try to invalidate first. This should work for
// non dirty unmapped page cache pages.
//
    ret = mapping_evict_folio(folio_mapping(folio), folio);
    }
    folio_unlock(folio);
    if (ret) {
    pr_info!("%#lx: invalidated\n", pfn);
    page_handle_poison(page, false, true);
    return 0;
    }
    isolated = isolate_folio_to_list(folio, &pagelist);
//
// If we succeed to isolate the folio, we grabbed another refcount on
// the folio, so we can safely drop the one we got from get_any_page().
// If we failed to isolate the folio, it means that we cannot go further
// and we will return an error, so drop the reference we got from
// get_any_page() as well.
//
    folio_put(folio);
    if (isolated) {
    ret = migrate_pages(&pagelist, alloc_migration_target, core::ptr::null_mut(),
    (unsigned long)&mtc, MIGRATE_SYNC, MR_MEMORY_FAILURE, core::ptr::null_mut());
    if (!ret) {
pub static mut release: bool = false;
    if (!page_handle_poison(page, huge, release)) {
    ret = -EBUSY;
    }
    } else {
    if (!list_empty(&pagelist)) {
    putback_movable_pages(&pagelist);
    }
    pr_info!("%#lx: %s migration failed %ld, type %pGp\n",
    pfn, msg_page[huge], ret, &page.flags.f);
    if (ret > 0) {
    ret = -EBUSY;
    }
    }
    } else {
    pr_info!("%#lx: %s isolation failed, page count %d, type %pGp\n",
    pfn, msg_page[huge], page_count(page), &page.flags.f);
    ret = -EBUSY;
    }
    return ret;
    }
//
// soft_offline_page - Soft offline a page.
// @pfn: pfn to soft-offline
// @flags: flags. Same as memory_failure().
//
// Returns 0 on success,
// -EOPNOTSUPP for hwpoison_filter() filtered the error event, or
// disabled by /proc/sys/vm/enable_soft_offline,
// < 0 otherwise negated errno.
//
// Soft offline a page, by migration or invalidation,
// without killing anything. This is for the case when
// a page is not corrupted yet (so it's still valid to access),
// but has had a number of corrected errors and is better taken
// out.
//
// The actual policy on when to do that is maintained by
// user space.
//
// This should never impact any application or cause data loss,
// however it might take some time.
//
// This is not a 100% solution for all memory, but tries to be
// ``good enough'' for the majority of memory.
//
#[no_mangle]
pub unsafe extern "C" fn soft_offline_page(pfn: c_ulong, flags: c_int) -> c_int {
    let mut ret = 0;
pub static mut try_again: bool = true;
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!pfn_valid(pfn)) {
    WARN_ON_ONCE!(flags & MF_COUNT_INCREASED);
    return -ENXIO;
    }
// Only online pages can be soft-offlined (esp., not ZONE_DEVICE).
    page = pfn_to_online_page(pfn);
    if (!page) {
    put_ref_page(pfn, flags);
    return -EIO;
    }
    if (!sysctl_enable_soft_offline) {
    pr_info_once!("disabled by /proc/sys/vm/enable_soft_offline\n");
    put_ref_page(pfn, flags);
    return -EOPNOTSUPP;
    }
    mutex_lock(&mf_mutex);
    if (PageHWPoison(page)) {
    pr_info!("%#lx: page already poisoned\n", pfn);
    put_ref_page(pfn, flags);
    mutex_unlock(&mf_mutex);
    return 0;
    }
// label;
    get_online_mems();
    ret = get_hwpoison_page(page, flags | MF_SOFT_OFFLINE);
    put_online_mems();
    if (hwpoison_filter(page)) {
    if (ret > 0) {
    put_page(page);
    }
    mutex_unlock(&mf_mutex);
    return -EOPNOTSUPP;
    }
    if (ret > 0) {
    ret = soft_offline_in_use_page(page);
    } else if (ret == 0) {
    if (!page_handle_poison(page, true, false)) {
    if (try_again) {
    try_again = false;
    flags &= ~MF_COUNT_INCREASED;
// goto;
    }
    ret = -EBUSY;
    }
    }
    mutex_unlock(&mf_mutex);
    return ret;
    }
}
}
