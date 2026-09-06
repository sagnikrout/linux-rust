//! Automatically rewritten from C to Rust
//! Source: mm/hmm.c
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
// Copyright 2013 Red Hat Inc.
//
// Authors: Jérôme Glisse <jglisse@redhat.com>
//
// Refer to include/linux/hmm.h for information about heterogeneous memory
// management or HMM for short.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmm_vma_walk {
    pub range: *mut hmm_range,
    pub locked: *mut bool,
    pub last: c_ulong,
    pub end: c_ulong,
    pub required_fault: c_uint,
}

//
// Internal sentinel returned by walk callbacks when they need a page fault.
// The callback stores end/required_fault in hmm_vma_walk; the outer loop
// consumes the sentinel and never propagates it to the caller.
//

//
// Internal sentinel returned by hmm_do_fault() when handle_mm_fault()
// completes a page fault with the mmap lock dropped. hmm_do_fault() sets
// *locked = false; the outer loop consumes the sentinel and never propagates
// it to the caller.
//

    enum {
    HMM_NEED_FAULT = 1 << 0,
    HMM_NEED_WRITE_FAULT = 1 << 1,
    HMM_NEED_ALL_BITS = HMM_NEED_FAULT | HMM_NEED_WRITE_FAULT,
    };
    enum {
// These flags are carried from input-to-output
    HMM_PFN_INOUT_FLAGS = HMM_PFN_DMA_MAPPED | HMM_PFN_P2PDMA |
    HMM_PFN_P2PDMA_BUS,
    };
#[no_mangle]
pub unsafe extern "C" fn hmm_pfns_fill(addr: c_ulong, end: c_ulong, range: *mut hmm_range, cpu_flags: c_ulong) -> c_int {
pub static mut i: c_ulong = 0;
    while (addr < end) {
    range.hmm_pfns[i] &= HMM_PFN_INOUT_FLAGS;
    range.hmm_pfns[i] |= cpu_flags;
    }
    return 0;
    }
//
// hmm_record_fault() - record a range that needs to be faulted in
//
// Called by the walk callbacks when they discover that part of the range
// needs a page fault.  The callback records what to fault and returns
// HMM_FAULT_PENDING; the outer loop in hmm_range_fault_locked() drops
// back out of walk_page_range() and invokes handle_mm_fault() from a context
// where no page-table or hugetlb_vma_lock is held.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_record_fault(addr: c_ulong, end: c_ulong, required_fault: c_uint, walk: *mut mm_walk) -> c_int {
    let mut hmm_vma_walk = walk.private;
    WARN_ON_ONCE!(!required_fault);
    hmm_vma_walk.last = addr;
    hmm_vma_walk.end = end;
    hmm_vma_walk.required_fault = required_fault;
    return HMM_FAULT_PENDING;
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_pte_need_fault(hmm_vma_walk: *mut hmm_vma_walk, pfn_req_flags: c_ulong, cpu_flags: c_ulong) -> c_uint {
    let mut range = hmm_vma_walk.range;
//
// So we not only consider the individual per page request we also
// consider the default flags requested for the range. The API can
// be used 2 ways. The first one where the HMM user coalesces
// multiple page faults into one request and sets flags per pfn for
// those faults. The second one where the HMM user wants to pre-
// fault a range with specific flags. For the latter one it is a
// waste to have the user pre-fill the pfn arrays with a default
// flags value.
//
    pfn_req_flags &= range.pfn_flags_mask;
    pfn_req_flags |= range.default_flags;
// We aren't ask to do anything ...
    if (!(pfn_req_flags & HMM_PFN_REQ_FAULT)) {
    return 0;
    }
// Need to write fault ?
    if ((pfn_req_flags & HMM_PFN_REQ_WRITE) &&
    !(cpu_flags & HMM_PFN_WRITE)) {
    return HMM_NEED_FAULT | HMM_NEED_WRITE_FAULT;
    }
// If CPU page table is not valid then we need to fault
    if (!(cpu_flags & HMM_PFN_VALID)) {
    return HMM_NEED_FAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_range_need_fault(hmm_vma_walk: *mut hmm_vma_walk, npages: c_ulong, cpu_flags: c_ulong) -> c_uint {
    let mut range = hmm_vma_walk.range;
pub static mut required_fault: c_uint = 0;
    let mut i = 0;
//
// If the default flags do not request to fault pages, and the mask does
// not allow for individual pages to be faulted, then
// hmm_pte_need_fault() will always return 0.
//
    if (!((range.default_flags | range.pfn_flags_mask) &
    HMM_PFN_REQ_FAULT)) {
    return 0;
    }
    while (i < npages) {
    required_fault |= hmm_pte_need_fault(hmm_vma_walk, hmm_pfns[i],
    cpu_flags);
    if (required_fault == HMM_NEED_ALL_BITS) {
    return required_fault;
    }
    }
    return required_fault;
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_vma_walk_hole(addr: c_ulong, end: c_ulong, depth: __always_unused int, walk: *mut mm_walk) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    let mut required_fault = 0;
    unsigned long i, npages;
pub static mut hmm_pfns: *mut c_void = core::ptr::null_mut();
    i = (addr - range.start) >> PAGE_SHIFT;
    npages = (end - addr) >> PAGE_SHIFT;
    hmm_pfns = &range.hmm_pfns[i];
    required_fault =
    hmm_range_need_fault(hmm_vma_walk, hmm_pfns, npages, 0);
    if (!walk.vma) {
    if (required_fault) {
    return -EFAULT;
    }
    return hmm_pfns_fill(addr, end, range, HMM_PFN_ERROR);
    }
    if (required_fault) {
    return hmm_record_fault(addr, end, required_fault, walk);
    }
    return hmm_pfns_fill(addr, end, range, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_pfn_flags_order(order: c_ulong) -> c_ulong {
    return order << HMM_PFN_ORDER_SHIFT;
    }

#[no_mangle]
pub unsafe extern "C" fn pmd_to_hmm_pfn_flags(range: *mut hmm_range, pmd: pmd_t) -> c_ulong {
    if (pmd_protnone(pmd)) {
    return 0;
    }
    return (pmd_write(pmd) ? (HMM_PFN_VALID | HMM_PFN_WRITE) :
    HMM_PFN_VALID) |
    hmm_pfn_flags_order(PMD_SHIFT - PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_vma_handle_pmd(walk: *mut mm_walk, addr: c_ulong, end: c_ulong, pmd: pmd_t) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    unsigned long pfn, npages, i;
    let mut required_fault = 0;
    let mut cpu_flags = 0;
    npages = (end - addr) >> PAGE_SHIFT;
    cpu_flags = pmd_to_hmm_pfn_flags(range, pmd);
    required_fault =
    hmm_range_need_fault(hmm_vma_walk, hmm_pfns, npages, cpu_flags);
    if (required_fault) {
    return hmm_record_fault(addr, end, required_fault, walk);
    }
    pfn = pmd_pfn(pmd) + ((addr & ~PMD_MASK) >> PAGE_SHIFT);
    while (addr < end) {
    hmm_pfns[i] &= HMM_PFN_INOUT_FLAGS;
    hmm_pfns[i] |= pfn | cpu_flags;
    }
    return 0;
    }

// stub to allow the code below to compile
// forward_decl: hmm_vma_handle_pmd;

#[no_mangle]
pub unsafe extern "C" fn pte_to_hmm_pfn_flags(range: *mut hmm_range, pte: pte_t) -> c_ulong {
    if (pte_none(pte) || !pte_present(pte) || pte_protnone(pte)) {
    return 0;
    }
    return pte_write(pte) ? (HMM_PFN_VALID | HMM_PFN_WRITE) : HMM_PFN_VALID;
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_vma_handle_pte(walk: *mut mm_walk, addr: c_ulong, end: c_ulong, pmdp: *mut pmd_t, ptep: *mut pte_t, hmm_pfn: *mut c_ulong) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    let mut required_fault = 0;
    let mut cpu_flags = 0;
pub static mut pte: pte_t = 0;
pub static mut pfn_req_flags: u64 = 0;
pub static mut new_pfn_flags: u64 = 0;
//
// Any other marker than a UFFD WP marker will result in a fault error
// that will be correctly handled, so we need only check for UFFD WP
// here.
//
    if (pte_none(pte) || pte_is_uffd_wp_marker(pte)) {
    required_fault =
    hmm_pte_need_fault(hmm_vma_walk, pfn_req_flags, 0);
    if (required_fault) {
// goto;
    }
// goto;
    }
    if (!pte_present(pte)) {
pub static mut entry: softleaf_t = 0;
//
// Don't fault in device private pages owned by the caller,
// just report the PFN.
//
    if (softleaf_is_device_private(entry) &&
    page_pgmap(softleaf_to_page(entry)).owner ==
    range.dev_private_owner) {
    cpu_flags = HMM_PFN_VALID;
    if (softleaf_is_device_private_write(entry)) {
    cpu_flags |= HMM_PFN_WRITE;
    }
    new_pfn_flags = softleaf_to_pfn(entry) | cpu_flags;
// goto;
    }
    required_fault =
    hmm_pte_need_fault(hmm_vma_walk, pfn_req_flags, 0);
    if (!required_fault) {
// goto;
    }
    if (softleaf_is_swap(entry)) {
// goto;
    }
    if (softleaf_is_device_private(entry)) {
// goto;
    }
    if (softleaf_is_device_exclusive(entry)) {
// goto;
    }
    if (softleaf_is_migration(entry)) {
    pte_unmap(ptep);
    hmm_vma_walk.last = addr;
    migration_entry_wait(walk.mm, pmdp, addr);
    return -EBUSY;
    }
// Report error for everything else
    pte_unmap(ptep);
    return -EFAULT;
    }
    cpu_flags = pte_to_hmm_pfn_flags(range, pte);
    required_fault =
    hmm_pte_need_fault(hmm_vma_walk, pfn_req_flags, cpu_flags);
    if (required_fault) {
// goto;
    }
//
// Since each architecture defines a struct page for the zero page, just
// fall through and treat it like a normal page.
//
    if (!vm_normal_page(walk.vma, addr, pte) &&
    !is_zero_pfn(pte_pfn(pte))) {
    if (hmm_pte_need_fault(hmm_vma_walk, pfn_req_flags, 0)) {
    pte_unmap(ptep);
    return -EFAULT;
    }
    new_pfn_flags = HMM_PFN_ERROR;
// goto;
    }
    new_pfn_flags = pte_pfn(pte) | cpu_flags;
// label;
// hmm_pfn = (*hmm_pfn & HMM_PFN_INOUT_FLAGS) | new_pfn_flags;
    return 0;
// label;
    pte_unmap(ptep);
// Fault any virtual address we were asked to fault
    return hmm_record_fault(addr, end, required_fault, walk);
    }

#[no_mangle]
pub unsafe extern "C" fn hmm_vma_handle_absent_pmd(walk: *mut mm_walk, start: c_ulong, end: c_ulong, hmm_pfns: *mut c_ulong, pmd: pmd_t) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
pub static mut npages: c_ulong = 0;
pub static mut entry: softleaf_t = 0;
pub static mut addr: c_ulong = 0;
    let mut required_fault = 0;
    if (softleaf_is_device_private(entry) &&
    softleaf_to_folio(entry).pgmap.owner ==
    range.dev_private_owner) {
    let mut cpu_flags = HMM_PFN_VALID |
    hmm_pfn_flags_order(PMD_SHIFT - PAGE_SHIFT);
pub static mut pfn: c_ulong = 0;
    let mut i = 0;
    if (softleaf_is_device_private_write(entry)) {
    cpu_flags |= HMM_PFN_WRITE;
    }
//
// Fully populate the PFN list though subsequent PFNs could be
// inferred, because drivers which are not yet aware of large
// folios probably do not support sparsely populated PFN lists.
//
    while (addr < end) {
    hmm_pfns[i] &= HMM_PFN_INOUT_FLAGS;
    hmm_pfns[i] |= pfn | cpu_flags;
    }
    return 0;
    }
    required_fault = hmm_range_need_fault(hmm_vma_walk, hmm_pfns,
    npages, 0);
    if (required_fault) {
    if (softleaf_is_device_private(entry)) {
    return hmm_record_fault(addr, end, required_fault, walk);
    }
    else {
    return -EFAULT;
    }
    }
    return hmm_pfns_fill(start, end, range, HMM_PFN_ERROR);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: hmm_vma_handle_absent_pmd
pub unsafe extern "C" fn hmm_vma_handle_absent_pmd_dup(walk: *mut mm_walk, start: c_ulong, end: c_ulong, hmm_pfns: *mut c_ulong, pmd: pmd_t) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
pub static mut npages: c_ulong = 0;
    if (hmm_range_need_fault(hmm_vma_walk, hmm_pfns, npages, 0)) {
    return -EFAULT;
    }
    return hmm_pfns_fill(start, end, range, HMM_PFN_ERROR);
    }

#[no_mangle]
pub unsafe extern "C" fn hmm_vma_walk_pmd(pmdp: *mut pmd_t, start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    let mut hmm_pfns = &range.hmm_pfns[(start - range.start) >> PAGE_SHIFT];
pub static mut npages: c_ulong = 0;
pub static mut addr: c_ulong = 0;
pub static mut ptep: *mut c_void = core::ptr::null_mut();
    let mut pmd;
// label;
    pmd = pmdp_get_lockless(pmdp);
    if (pmd_none(pmd)) {
    return hmm_vma_walk_hole(start, end, -1, walk);
    }
    if (thp_migration_supported() && pmd_is_migration_entry(pmd)) {
    if (hmm_range_need_fault(hmm_vma_walk, hmm_pfns, npages, 0)) {
    hmm_vma_walk.last = addr;
    pmd_migration_entry_wait(walk.mm, pmdp);
    return -EBUSY;
    }
    return hmm_pfns_fill(start, end, range, 0);
    }
    if (!pmd_present(pmd)) {
    return hmm_vma_handle_absent_pmd(walk, start, end, hmm_pfns,
    pmd);
    }
    if (pmd_trans_huge(pmd)) {
//
// No need to take pmd_lock here, even if some other thread
// is splitting the huge pmd we will get that event through
// mmu_notifier callback.
//
// So just read pmd value and check again it's a transparent
// huge or device mapping one and compute corresponding pfn
// values.
//
    pmd = pmdp_get_lockless(pmdp);
    if (!pmd_trans_huge(pmd)) {
// goto;
    }
    return hmm_vma_handle_pmd(walk, addr, end, hmm_pfns, pmd);
    }
//
// We have handled all the valid cases above ie either none, migration,
// huge or transparent huge. At this point either it is a valid pmd
// entry pointing to pte directory or it is a bad pmd that will not
// recover.
//
    if (pmd_bad(pmd)) {
    if (hmm_range_need_fault(hmm_vma_walk, hmm_pfns, npages, 0)) {
    return -EFAULT;
    }
    return hmm_pfns_fill(start, end, range, HMM_PFN_ERROR);
    }
    ptep = pte_offset_map(pmdp, addr);
    if (!ptep) {
// goto;
    }
    while (addr < end) {
    let mut r = 0;
    r = hmm_vma_handle_pte(walk, addr, end, pmdp, ptep, hmm_pfns);
    if (r) {
// hmm_vma_handle_pte() did pte_unmap()
    return r;
    }
    }
    pte_unmap(ptep - 1);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn pud_to_hmm_pfn_flags(range: *mut hmm_range, pud: pud_t) -> c_ulong {
    if (!pud_present(pud)) {
    return 0;
    }
    return (pud_write(pud) ? (HMM_PFN_VALID | HMM_PFN_WRITE) :
    HMM_PFN_VALID) |
    hmm_pfn_flags_order(PUD_SHIFT - PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn hmm_vma_walk_pud(pudp: *mut pud_t, start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
pub static mut addr: c_ulong = 0;
    let mut pud;
    let mut ptl = pud_trans_huge_lock(pudp, walk.vma);
    if (!ptl) {
    return 0;
    }
// Normally we don't want to split the huge page
    walk.action = ACTION_CONTINUE;
    pud = pudp_get(pudp);
    if (!pud_present(pud)) {
    spin_unlock(ptl);
    return hmm_vma_walk_hole(start, end, -1, walk);
    }
    if (pud_leaf(pud)) {
    unsigned long i, npages, pfn;
    let mut required_fault = 0;
pub static mut hmm_pfns: *mut c_void = core::ptr::null_mut();
    let mut cpu_flags = 0;
    i = (addr - range.start) >> PAGE_SHIFT;
    npages = (end - addr) >> PAGE_SHIFT;
    hmm_pfns = &range.hmm_pfns[i];
    cpu_flags = pud_to_hmm_pfn_flags(range, pud);
    required_fault = hmm_range_need_fault(hmm_vma_walk, hmm_pfns,
    npages, cpu_flags);
    if (required_fault) {
    spin_unlock(ptl);
    return hmm_record_fault(addr, end, required_fault, walk);
    }
    pfn = pud_pfn(pud) + ((addr & ~PUD_MASK) >> PAGE_SHIFT);
    while (i < npages) {
    hmm_pfns[i] &= HMM_PFN_INOUT_FLAGS;
    hmm_pfns[i] |= pfn | cpu_flags;
    }
// goto;
    }
// Ask for the PUD to be split
    walk.action = ACTION_SUBTREE;
// label;
    spin_unlock(ptl);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hmm_vma_walk_hugetlb_entry(pte: *mut pte_t, hmask: c_ulong, start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut addr: c_ulong = 0;
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    let mut vma = walk.vma;
    let mut required_fault = 0;
    let mut pfn_req_flags = 0;
    let mut cpu_flags = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
    ptl = huge_pte_lock(hstate_vma(vma), walk.mm, pte);
    entry = huge_ptep_get(walk.mm, addr, pte);
    i = (start - range.start) >> PAGE_SHIFT;
    pfn_req_flags = range.hmm_pfns[i];
    cpu_flags = pte_to_hmm_pfn_flags(range, entry) |
    hmm_pfn_flags_order(huge_page_order(hstate_vma(vma)));
    required_fault =
    hmm_pte_need_fault(hmm_vma_walk, pfn_req_flags, cpu_flags);
    if (required_fault) {
    spin_unlock(ptl);
    return hmm_record_fault(addr, end, required_fault, walk);
    }
    pfn = pte_pfn(entry) + ((start & ~hmask) >> PAGE_SHIFT);
    while (addr < end) {
    range.hmm_pfns[i] &= HMM_PFN_INOUT_FLAGS;
    range.hmm_pfns[i] |= pfn | cpu_flags;
    }
    spin_unlock(ptl);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hmm_vma_walk_test(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut hmm_vma_walk = walk.private;
    let mut range = hmm_vma_walk.range;
    let mut vma = walk.vma;
    if (!(vma.vm_flags & (VM_IO | VM_PFNMAP)) &&
    vma.vm_flags & VM_READ) {
    return 0;
    }
//
// vma ranges that don't have struct page backing them or map I/O
// devices directly cannot be handled by hmm_range_fault().
//
// If the vma does not allow read access, then assume that it does not
// allow write access either. HMM does not support architectures that
// allow write without read.
//
// If a fault is requested for an unsupported range then it is a hard
// failure.
//
    if (hmm_range_need_fault(hmm_vma_walk,
    range.hmm_pfns +
    ((start - range.start) >> PAGE_SHIFT),
    (end - start) >> PAGE_SHIFT, 0)) {
    return -EFAULT;
    }
    hmm_pfns_fill(start, end, range, HMM_PFN_ERROR);
// Skip this vma and continue processing the next vma.
    return 1;
    }
pub static mut mm_walk_ops: usize = 0;
//
// hmm_do_fault - fault in a range recorded by a walk callback
//
// Called from the outer loop in hmm_range_fault_locked() after a callback
// returned HMM_FAULT_PENDING.  At this point we hold only mmap_lock;
// the page-table spinlock and any hugetlb_vma_lock acquired by the walk
// framework have already been released by the unwind.
//
// Returns -EBUSY on success (all pages faulted, caller should re-walk).
// Returns a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_do_fault(mm: *mut mm_struct, hmm_vma_walk: *mut hmm_vma_walk) -> c_int {
pub static mut addr: c_ulong = 0;
pub static mut end: c_ulong = 0;
pub static mut required_fault: c_uint = 0;
pub static mut fault_flags: c_uint = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    if (hmm_vma_walk.locked) {
    fault_flags |= FAULT_FLAG_ALLOW_RETRY | FAULT_FLAG_KILLABLE;
    }
    vma = vma_lookup(mm, addr);
    if (!vma) {
    return -EFAULT;
    }
    if (required_fault & HMM_NEED_WRITE_FAULT) {
    if (!(vma.vm_flags & VM_WRITE)) {
    return -EPERM;
    }
    fault_flags |= FAULT_FLAG_WRITE;
    }
    while (addr < end) {
    let mut ret;
    ret = handle_mm_fault(vma, addr, fault_flags, core::ptr::null_mut());
    if (ret & (VM_FAULT_COMPLETED | VM_FAULT_RETRY)) {
    if (hmm_vma_walk.locked)    /* needed by sparse */ {
// hmm_vma_walk->locked = false;
    }
    else {
    WARN_ON_ONCE!(1);    /* broken fault handler */
    }
    return HMM_FAULT_UNLOCKED;
    }
    if (ret & VM_FAULT_ERROR) {
pub static mut err: c_int = 0;
    if (WARN_ON!(!err)) {
    err = -EINVAL;
    }
    return err;
    }
    }
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn hmm_range_fault_locked(range: *mut hmm_range, locked: *mut bool) -> c_int {
pub static mut hmm_vma_walk: usize = 0;
    let mut mm = range.notifier.mm;
    let mut ret = 0;
    mmap_assert_locked(mm);
    do {
// If range is no longer valid force retry.
    if (mmu_interval_check_retry(range.notifier,
    range.notifier_seq)) {
    return -EBUSY;
    }
    ret = walk_page_range(mm, hmm_vma_walk.last, range.end,
    &hmm_walk_ops, &hmm_vma_walk);
//
// When HMM_FAULT_PENDING is returned a walk callback
// recorded a range that needs handle_mm_fault();
// hmm_do_fault() runs the fault outside walk_page_range()
// (so no page-table or hugetlb_vma_lock is held) and
// returns -EBUSY so the loop re-walks and picks up the
// now-present entries.
//
    if (ret == HMM_FAULT_PENDING) {
    ret = hmm_do_fault(mm, &hmm_vma_walk);
    if (ret == HMM_FAULT_UNLOCKED) {
    if (fatal_signal_pending(current)) {
    return -EINTR;
    }
    return -EBUSY;
    }
    }
//
// When -EBUSY is returned the loop restarts with
// hmm_vma_walk.last set to an address that has not been stored
// in pfns. All entries < last in the pfn array are set to their
// output, and all >= are still at their input values.
//
    } while (ret == -EBUSY);
    return ret;
    }
//
// hmm_range_fault - try to fault some address in a virtual address range
// @range:	argument structure
//
// Returns 0 on success or one of the following error codes:
//
// -EINVAL:	Invalid arguments or mm or virtual address is in an invalid vma
// (e.g., device file vma).
// -ENOMEM:	Out of memory.
// -EPERM:	Invalid permission (e.g., asking for write and range is read
// only).
// -EBUSY:	The range has been invalidated and the caller needs to wait for
// the invalidation to finish.
// -EFAULT:     A page was requested to be valid and could not be made valid
// ie it has no backing VMA or it is illegal to access
//
// This is similar to get_user_pages(), except that it can read the page tables
// without mutating them (ie causing faults).
//
// The mmap lock must be held by the caller and will remain held on return.
// New users should prefer hmm_range_fault_unlocked_timeout() unless they
// specifically need to keep the mmap lock held across the call. This helper
// cannot support VMAs whose fault handlers need to drop the mmap lock.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_range_fault(range: *mut hmm_range) -> c_int {
    return hmm_range_fault_locked(range, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(hmm_range_fault);
//
// hmm_range_fault_unlocked_timeout - fault in a range with a retry timeout
// @range:	argument structure
// @timeout:	timeout in jiffies for internal -EBUSY retries, or 0 to retry
// indefinitely
//
// The caller must not hold the mmap lock. The function takes the mmap read
// lock internally and allows handle_mm_fault() to drop it during faults. If
// the mmap lock is dropped or the range is invalidated, the function refreshes
// range->notifier_seq and restarts the walk internally.
//
// Passing 0 for @timeout retries indefinitely. A non-zero @timeout is a caller
// policy limit for repeated mmu-notifier invalidation retries. HMM does not
// interrupt page fault handling when the timeout expires, but returns -EBUSY
// if the retry budget is exhausted before a stable range is obtained.
//
// Returns 0 on success or one of the error codes documented for
// hmm_range_fault(). -EINTR is returned if mmap_lock acquisition is
// interrupted or a fatal signal is pending during retry handling.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_range_fault_unlocked_timeout(range: *mut hmm_range, timeout: c_ulong) -> c_int {
    let mut mm = range.notifier.mm;
pub static mut deadline: c_ulong = 0;
pub static mut locked: bool = false;
    let mut ret = 0;
    do {
//
// If the previous fault dropped mmap_lock, then the fault
// handler made progress. Restart the retry timeout in that
// case, but keep the existing deadline for ordinary -EBUSY
// retries.
//
    if (timeout && !locked) {
    deadline = jiffies + timeout;
    }
    range.notifier_seq =
    mmu_interval_read_begin(range.notifier);
    ret = mmap_read_lock_killable(mm);
    if (ret) {
    return ret;
    }
    if (check_stable_address_space(mm)) {
    mmap_read_unlock(mm);
    return -EFAULT;
    }
    if (timeout && time_after(jiffies, deadline)) {
    mmap_read_unlock(mm);
    return -EBUSY;
    }
    locked = true;
    ret = hmm_range_fault_locked(range, &locked);
    if (locked) {
    mmap_read_unlock(mm);
    }
    } while (ret == -EBUSY);
    return ret;
    }
    EXPORT_SYMBOL(hmm_range_fault_unlocked_timeout);
//
// hmm_dma_map_alloc - Allocate HMM map structure
// @dev: device to allocate structure for
// @map: HMM map to allocate
// @nr_entries: number of entries in the map
// @dma_entry_size: size of the DMA entry in the map
//
// Allocate the HMM map structure and all the lists it contains.
// Return 0 on success, -ENOMEM on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_dma_map_alloc(dev: *mut device, map: *mut hmm_dma_map, nr_entries: size_t, dma_entry_size: size_t) -> c_int {
pub static mut dma_need_sync: bool = false;
    let mut use_iova = 0;
    WARN_ON_ONCE!(!(nr_entries * PAGE_SIZE / dma_entry_size));
//
// The HMM API violates our normal DMA buffer ownership rules and can't
// transfer buffer ownership.  The dma_addressing_limited() check is a
// best approximation to ensure no swiotlb buffering happens.
//

    dma_need_sync = !dev_dma_skip_sync(dev);

    if (dma_need_sync || dma_addressing_limited(dev)) {
    return -EOPNOTSUPP;
    }
    map.dma_entry_size = dma_entry_size;
    map.pfn_list = kvcalloc(nr_entries, sizeof!(*map.pfn_list),
    GFP_KERNEL | __GFP_NOWARN);
    if (!map.pfn_list) {
    return -ENOMEM;
    }
    use_iova = dma_iova_try_alloc(dev, &map.state, 0,
    nr_entries * PAGE_SIZE);
    if (!use_iova && dma_need_unmap(dev)) {
    map.dma_list = kvzalloc_objs(*map.dma_list, nr_entries,
    GFP_KERNEL | __GFP_NOWARN);
    if (!map.dma_list) {
// goto;
    }
    }
    return 0;
// label;
    kvfree(map.pfn_list);
    return -ENOMEM;
    }
    EXPORT_SYMBOL_GPL(hmm_dma_map_alloc);
//
// hmm_dma_map_free - iFree HMM map structure
// @dev: device to free structure from
// @map: HMM map containing the various lists and state
//
// Free the HMM map structure and all the lists it contains.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_dma_map_free(dev: *mut device, map: *mut hmm_dma_map) {
    if (dma_use_iova(&map.state)) {
    dma_iova_free(dev, &map.state);
    }
    kvfree(map.pfn_list);
    kvfree(map.dma_list);
    }
    EXPORT_SYMBOL_GPL(hmm_dma_map_free);
//
// hmm_dma_map_pfn - Map a physical HMM page to DMA address
// @dev: Device to map the page for
// @map: HMM map
// @idx: Index into the PFN and dma address arrays
// @p2pdma_state: PCI P2P state.
//
// dma_alloc_iova() allocates IOVA based on the size specified by their use in
// iova->size. Call this function after IOVA allocation to link whole @page
// to get the DMA address. Note that very first call to this function
// will have @offset set to 0 in the IOVA space allocated from
// dma_alloc_iova(). For subsequent calls to this function on same @iova,
// @offset needs to be advanced by the caller with the size of previous
// page that was linked + DMA address returned for the previous page that was
// linked by this function.
//
    dma_addr_t hmm_dma_map_pfn(device *dev, hmm_dma_map *map,
    size_t idx, pci_p2pdma_map_state *p2pdma_state)
    {
    let mut state = &map.state;
    let mut dma_addrs = map.dma_list;
    let mut pfns = map.pfn_list;
    let mut page = hmm_pfn_to_page(pfns[idx]);
pub static mut paddr: phys_addr_t = 0;
pub static mut offset: usize = 0;
pub static mut attrs: c_ulong = 0;
    let mut dma_addr;
    let mut ret = 0;
    if ((pfns[idx] & HMM_PFN_DMA_MAPPED) &&
    !(pfns[idx] & HMM_PFN_P2PDMA_BUS)) {
//
// We are in this flow when there is a need to resync flags,
// for example when page was already linked in prefetch call
// with READ flag and now we need to add WRITE flag
//
// This page was already programmed to HW and we don't want/need
// to unlink and link it again just to resync flags.
//
    if (dma_use_iova(state)) {
    return state.addr + offset;
    }
//
// Without dma_need_unmap, the dma_addrs array is NULL, thus we
// need to regenerate the address below even if there already
// was a mapping. But !dma_need_unmap implies that the
// mapping stateless, so this is fine.
//
    if (dma_need_unmap(dev)) {
    return dma_addrs[idx];
    }
// Continue to remapping
    }
    switch (pci_p2pdma_state(p2pdma_state, dev, page)) {
    case PCI_P2PDMA_MAP_NONE:
    break;
    case PCI_P2PDMA_MAP_THRU_HOST_BRIDGE:
    attrs |= DMA_ATTR_MMIO;
    pfns[idx] |= HMM_PFN_P2PDMA;
    break;
    case PCI_P2PDMA_MAP_BUS_ADDR:
    pfns[idx] |= HMM_PFN_P2PDMA_BUS | HMM_PFN_DMA_MAPPED;
    return pci_p2pdma_bus_addr_map(p2pdma_state.mem, paddr);
// label;
    return DMA_MAPPING_ERROR;
    }
    if (dma_use_iova(state)) {
    ret = dma_iova_link(dev, state, paddr, offset,
    map.dma_entry_size, DMA_BIDIRECTIONAL,
    attrs);
    if (ret) {
// goto;
    }
    ret = dma_iova_sync(dev, state, offset, map.dma_entry_size);
    if (ret) {
    dma_iova_unlink(dev, state, offset, map.dma_entry_size,
    DMA_BIDIRECTIONAL, attrs);
// goto;
    }
    dma_addr = state.addr + offset;
    } else {
    if (WARN_ON_ONCE!(dma_need_unmap(dev) && !dma_addrs)) {
// goto;
    }
    dma_addr = dma_map_phys(dev, paddr, map.dma_entry_size,
    DMA_BIDIRECTIONAL, attrs);
    if (dma_mapping_error(dev, dma_addr)) {
// goto;
    }
    if (dma_need_unmap(dev)) {
    dma_addrs[idx] = dma_addr;
    }
    }
    pfns[idx] |= HMM_PFN_DMA_MAPPED;
    return dma_addr;
// label;
    pfns[idx] &= ~HMM_PFN_P2PDMA;
    return DMA_MAPPING_ERROR;
    }
    EXPORT_SYMBOL_GPL(hmm_dma_map_pfn);
//
// hmm_dma_unmap_pfn - Unmap a physical HMM page from DMA address
// @dev: Device to unmap the page from
// @map: HMM map
// @idx: Index of the PFN to unmap
//
// Returns true if the PFN was mapped and has been unmapped, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn hmm_dma_unmap_pfn(dev: *mut device, map: *mut hmm_dma_map, idx: usize) -> bool {
pub static mut valid_dma: c_ulong = 0;
    let mut state = &map.state;
    let mut dma_addrs = map.dma_list;
    let mut pfns = map.pfn_list;
pub static mut attrs: c_ulong = 0;
    if ((pfns[idx] & valid_dma) != valid_dma) {
    return false;
    }
    if (pfns[idx] & HMM_PFN_P2PDMA) {
    attrs |= DMA_ATTR_MMIO;
    }
    if (pfns[idx] & HMM_PFN_P2PDMA_BUS) {
    ; /* no need to unmap bus address P2P mappings */
    }

    else if (dma_use_iova(state)) {
    dma_iova_unlink(dev, state, idx * map.dma_entry_size,
    map.dma_entry_size, DMA_BIDIRECTIONAL, attrs);
    }

    else if (dma_need_unmap(dev)) {
    dma_unmap_phys(dev, dma_addrs[idx], map.dma_entry_size,
    DMA_BIDIRECTIONAL, attrs);
    }
    pfns[idx] &=
    ~(HMM_PFN_DMA_MAPPED | HMM_PFN_P2PDMA | HMM_PFN_P2PDMA_BUS);
    return true;
    }
    EXPORT_SYMBOL_GPL(hmm_dma_unmap_pfn);