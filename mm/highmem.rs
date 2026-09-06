//! Automatically rewritten from C to Rust
//! Source: mm/highmem.c
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
// High memory handling common code and variables.
//
// (C) 1999 Andrea Arcangeli, SuSE GmbH, andrea@suse.de
// Gerhard Wichert, Siemens AG, Gerhard.Wichert@pdb.siemens.de
//
// Redesigned the x86 32-bit VM architecture to deal with
// 64-bit physical space. With current x86 CPUs this
// means up to 64 Gigabytes physical RAM.
//
// Rewrote high memory support to move the page cache into
// high memory. Implemented permanent (schedulable) kmaps
// based on Linus' idea.
//
// Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
//

#[no_mangle]
pub unsafe extern "C" fn kmap_local_calc_idx(idx: c_int) -> c_int {
    return idx + KM_MAX_IDX * smp_processor_id();
    }

//
// Virtual_count is not a pure "count".
// 0 means that it is not mapped, and has not been mapped
// since a TLB flush - it is usable.
// 1 means that there are no users, but it has been mapped
// since the last TLB flush - so we can't use it.
// n means that there are (n-1) current users of it.
//

//
// Architecture with aliasing data cache may define the following family of
// helper functions in its asm/highmem.h to control cache color of virtual
// addresses where physical memory pages are mapped by kmap.
//

//
// Determine color of virtual address where the page should be mapped.
//
#[no_mangle]
pub unsafe extern "C" fn get_pkmap_color(page: *const page) -> c_uint {
    return 0;
    }

//
// Get next index for mapping inside PKMAP region for page with given color.
//
#[no_mangle]
pub unsafe extern "C" fn get_next_pkmap_nr(color: c_uint) -> c_uint {
    static unsigned int last_pkmap_nr;
    last_pkmap_nr = (last_pkmap_nr + 1) & LAST_PKMAP_MASK;
    return last_pkmap_nr;
    }
//
// Determine if page index inside PKMAP region (pkmap_nr) of given color
// has wrapped around PKMAP region end. When this happens an attempt to
// flush all unused PKMAP slots is made.
//
#[no_mangle]
pub unsafe extern "C" fn no_more_pkmaps(pkmap_nr: c_uint, color: c_uint) -> c_int {
pub static mut pkmap_nr: return = 0;
    }
//
// Get the number of PKMAP entries of the given color. If no free slot is
// found after checking that many entries, kmap will sleep waiting for
// someone to call kunmap and free PKMAP slot.
//
#[no_mangle]
pub unsafe extern "C" fn get_pkmap_entries_count(color: c_uint) -> c_int {
    return LAST_PKMAP;
    }
//
// Get head of a wait queue for PKMAP entries of the given color.
// Wait queues for different mapping colors should be independent to avoid
// unnecessary wakeups caused by freeing of slots of other colors.
//
    static inline wait_queue_head_t *get_pkmap_wait_queue_head(unsigned int color)
    {
pub static mut pkmap_map_wait: usize = 0;
    return &pkmap_map_wait;
    }

#[no_mangle]
pub unsafe extern "C" fn __nr_free_highpages() -> c_ulong {
pub static mut pages: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    if (is_highmem(zone)) {
    pages += zone_page_state(zone, NR_FREE_PAGES);
    }
    }
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn __totalhigh_pages() -> c_ulong {
pub static mut pages: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    for_each_populated_zone(zone) {
    if (is_highmem(zone)) {
    pages += zone_managed_pages(zone);
    }
    }
    return pages;
    }
    EXPORT_SYMBOL(__totalhigh_pages);
    static int pkmap_count[LAST_PKMAP];
    static  __cacheline_aligned_in_smp DEFINE_SPINLOCK(kmap_lock);
pub static mut pkmap_page_table: *mut c_void = core::ptr::null_mut();
//
// Most architectures have no use for kmap_high_get(), so let's abstract
// the disabling of IRQ out of the locking in that case to save on a
// potential useless overhead.
//

    do { spin_lock(&kmap_lock); (void)(flags); } while (0)

    do { spin_unlock(&kmap_lock); (void)(flags); } while (0)

#[no_mangle]
pub unsafe extern "C" fn __kmap_to_page(vaddr: *mut c_void) -> *mut c_void {
pub static mut base: c_ulong = 0;
    let mut kctrl = &current.kmap_ctrl;
pub static mut addr: c_ulong = 0;
    let mut i = 0;
// kmap() mappings
    if (WARN_ON_ONCE!(addr >= PKMAP_ADDR(0) &&
    addr < PKMAP_ADDR(LAST_PKMAP))) {
    return pte_page(ptep_get(&pkmap_page_table[PKMAP_NR(addr)]));
    }
// kmap_local_page() mappings
    if (WARN_ON_ONCE!(base >= __fix_to_virt(FIX_KMAP_END) &&
    base < __fix_to_virt(FIX_KMAP_BEGIN))) {
    while (i < kctrl.idx) {
    let mut base_addr = 0;
    let mut idx = 0;
pub static mut pteval: pte_t = 0;
    idx = arch_kmap_local_map_idx(i, pte_pfn(pteval));
    base_addr = __fix_to_virt(FIX_KMAP_BEGIN + idx);
    if (base_addr == base) {
    return pte_page(pteval);
    }
    }
    }
    return virt_to_page(vaddr);
    }
    EXPORT_SYMBOL(__kmap_to_page);
#[no_mangle]
unsafe extern "C" fn flush_all_zero_pkmaps() {
    let mut i = 0;
pub static mut need_flush: c_int = 0;
    flush_cache_kmaps();
    while (i < LAST_PKMAP) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut ptent;
//
// zero means we don't have anything to do,
// >1 means that it is still in use. Only
// a count of 1 means that it is free but
// needs to be unmapped
//
    if (pkmap_count[i] != 1) {
    continue;
    }
    pkmap_count[i] = 0;
// sanity check
    ptent = ptep_get(&pkmap_page_table[i]);
    BUG_ON!(pte_none(ptent));
//
// Don't need an atomic fetch-and-clear op here;
// no-one has the page mapped, and cannot get at
// its virtual address (and hence PTE) without first
// getting the kmap_lock (which is held here).
// So no dangers, even with speculative execution.
//
    page = pte_page(ptent);
    pte_clear(&init_mm, PKMAP_ADDR(i), &pkmap_page_table[i]);
    set_page_address(page, core::ptr::null_mut());
    need_flush = 1;
    }
    if (need_flush) {
    flush_tlb_kernel_range(PKMAP_ADDR(0), PKMAP_ADDR(LAST_PKMAP));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kmap_flush_unused() {
    lock_kmap();
    flush_all_zero_pkmaps();
    unlock_kmap();
    }
#[no_mangle]
pub unsafe extern "C" fn map_new_virtual(page: *mut page) -> c_ulong {
    let mut vaddr = 0;
    let mut count = 0;
    let mut last_pkmap_nr = 0;
pub static mut color: c_uint = 0;
// label;
    count = get_pkmap_entries_count(color);
// Find an empty entry
    for (;;) {
    last_pkmap_nr = get_next_pkmap_nr(color);
    if (no_more_pkmaps(last_pkmap_nr, color)) {
    flush_all_zero_pkmaps();
    count = get_pkmap_entries_count(color);
    }
    if (!pkmap_count[last_pkmap_nr]) {
    break;	/* Found a usable entry */
    }
    if (--count) {
    continue;
    }
//
// Sleep for somebody else to unmap their entries
//
    {
pub static mut wait: usize = 0;
    let mut pkmap_map_wait = get_pkmap_wait_queue_head(color);
    __set_current_state(TASK_UNINTERRUPTIBLE);
    add_wait_queue(pkmap_map_wait, &wait);
    unlock_kmap();
    schedule();
    remove_wait_queue(pkmap_map_wait, &wait);
    lock_kmap();
// Somebody else might have mapped it while we slept
    if (page_address(page)) {
    return (unsigned long)page_address(page);
    }
// Re-start
// goto;
    }
    }
    vaddr = PKMAP_ADDR(last_pkmap_nr);
    set_pte_at(&init_mm, vaddr,
    &(pkmap_page_table[last_pkmap_nr]), mk_pte(page, kmap_prot));
    pkmap_count[last_pkmap_nr] = 1;
    set_page_address(page, vaddr);
    return vaddr;
    }
//
// kmap_high - map a highmem page into memory
// @page: &struct page to map
//
// Returns the page's virtual memory address.
//
// We cannot call this from interrupts, as it may block.
//
#[no_mangle]
pub unsafe extern "C" fn kmap_high(page: *mut page) -> *mut c_void {
    let mut vaddr = 0;
//
// For highmem pages, we can't trust "virtual" until
// after we have the lock.
//
    lock_kmap();
    vaddr = (unsigned long)page_address(page);
    if (!vaddr) {
    vaddr = map_new_virtual(page);
    }
    pkmap_count[PKMAP_NR(vaddr)]++;
    BUG_ON!(pkmap_count[PKMAP_NR(vaddr)] < 2);
    unlock_kmap();
    return  vaddr;
    }
    EXPORT_SYMBOL(kmap_high);

//
// kmap_high_get - pin a highmem page into memory
// @page: &struct page to pin
//
// Returns the page's current virtual memory address, or NULL if no mapping
// exists.  If and only if a non null address is returned then a
// matching call to kunmap_high() is necessary.
//
// This can be called from any context.
//
#[no_mangle]
pub unsafe extern "C" fn kmap_high_get(page: *mut page) -> *mut c_void {
    unsigned long vaddr, flags;
    lock_kmap_any(flags);
    vaddr = (unsigned long)page_address(page);
    if (vaddr) {
    BUG_ON!(pkmap_count[PKMAP_NR(vaddr)] < 1);
    pkmap_count[PKMAP_NR(vaddr)]++;
    }
    unlock_kmap_any(flags);
    return  vaddr;
    }

//
// kunmap_high - unmap a highmem page into memory
// @page: &struct page to unmap
//
// If ARCH_NEEDS_KMAP_HIGH_GET is not defined then this may be called
// only from user context.
//
#[no_mangle]
pub unsafe extern "C" fn kunmap_high(page: *const page) {
    let mut vaddr = 0;
    let mut nr = 0;
    let mut flags = 0;
    let mut need_wakeup = 0;
pub static mut color: c_uint = 0;
pub static mut pkmap_map_wait: *mut c_void = core::ptr::null_mut();
    lock_kmap_any(flags);
    vaddr = (unsigned long)page_address(page);
    BUG_ON!(!vaddr);
    nr = PKMAP_NR(vaddr);
//
// A count must never go down to zero
// without a TLB flush!
//
    need_wakeup = 0;
    match (--pkmap_count[nr]) {
    0 => {
    BUG();
    }
    1 => {
//
// Avoid an unnecessary wake_up() function call.
// The common case is pkmap_count[] == 1, but
// no waiters.
// The tasks queued in the wait-queue are guarded
// by both the lock in the wait-queue-head and by
// the kmap_lock.  As the kmap_lock is held here,
// no need for the wait-queue-head's lock.  Simply
// test if the queue is empty.
//
    pkmap_map_wait = get_pkmap_wait_queue_head(color);
    need_wakeup = waitqueue_active(pkmap_map_wait);
    }
    }
    unlock_kmap_any(flags);
// do wake-up, if needed, race-free outside of the spin lock
    if (need_wakeup) {
    wake_up(pkmap_map_wait);
    }
    }
    EXPORT_SYMBOL(kunmap_high);
#[no_mangle]
pub unsafe extern "C" fn zero_user_segments(page: *mut page, start1: c_uint, end1: c_uint, start2: c_uint, end2: c_uint) {
    let mut i = 0;
    BUG_ON!(end1 > page_size(page) || end2 > page_size(page));
    if (start1 >= end1) {
    start1 = end1 = 0;
    }
    if (start2 >= end2) {
    start2 = end2 = 0;
    }
    while (i < compound_nr(page)) {
    let mut kaddr = core::ptr::null_mut();
    if (start1 >= PAGE_SIZE) {
    start1 -= PAGE_SIZE;
    end1 -= PAGE_SIZE;
    } else {
pub static mut this_end: unsigned = 0;
    if (end1 > start1) {
    kaddr = kmap_local_page(page + i);
    memset(kaddr + start1, 0, this_end - start1);
    }
    end1 -= this_end;
    start1 = 0;
    }
    if (start2 >= PAGE_SIZE) {
    start2 -= PAGE_SIZE;
    end2 -= PAGE_SIZE;
    } else {
pub static mut this_end: unsigned = 0;
    if (end2 > start2) {
    if (!kaddr) {
    kaddr = kmap_local_page(page + i);
    }
    memset(kaddr + start2, 0, this_end - start2);
    }
    end2 -= this_end;
    start2 = 0;
    }
    if (kaddr) {
    kunmap_local(kaddr);
    flush_dcache_page(page + i);
    }
    if (!end1 && !end2) {
    break;
    }
    }
    BUG_ON!((start1 | start2 | end1 | end2) != 0);
    }
    EXPORT_SYMBOL(zero_user_segments);

//
// With DEBUG_KMAP_LOCAL the stack depth is doubled and every second
// slot is unused which acts as a guard page
//

#[no_mangle]
pub unsafe extern "C" fn kmap_local_idx_push() -> c_int {
    WARN_ON_ONCE!(in_hardirq() && !irqs_disabled());
    current.kmap_ctrl.idx += KM_INCR;
    BUG_ON!(current.kmap_ctrl.idx >= KM_MAX_IDX);
    return current.kmap_ctrl.idx - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kmap_local_idx() -> c_int {
    return current.kmap_ctrl.idx - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kmap_local_idx_pop() {
    current.kmap_ctrl.idx -= KM_INCR;
    BUG_ON!(current.kmap_ctrl.idx < 0);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_kmap_local_high_get(page: *mut page) -> *mut c_void {
    return core::ptr::null_mut();
    }

    set_pte_at(mm, vaddr, ptep, ptev)

// Unmap a local mapping which was obtained by kmap_high_get()
#[no_mangle]
pub unsafe extern "C" fn kmap_high_unmap_local(vaddr: c_ulong) -> bool {

    if (vaddr >= PKMAP_ADDR(0) && vaddr < PKMAP_ADDR(LAST_PKMAP)) {
    kunmap_high(pte_page(ptep_get(&pkmap_page_table[PKMAP_NR(vaddr)])));
    return true;
    }

    return false;
    }
pub static mut __kmap_pte: *mut c_void = core::ptr::null_mut();
    static pte_t *kmap_get_pte(unsigned long vaddr, int idx)
    {
    if (IS_ENABLED!(CONFIG_KMAP_LOCAL_NON_LINEAR_PTE_ARRAY)) {
//
// Set by the arch if __kmap_pte[-idx] does not produce
// the correct entry.
//
    return virt_to_kpte(vaddr);
    }
    if (!__kmap_pte) {
    __kmap_pte = virt_to_kpte(__fix_to_virt(FIX_KMAP_BEGIN));
    }
    return &__kmap_pte[-idx];
    }
#[no_mangle]
pub unsafe extern "C" fn __kmap_local_pfn_prot(pfn: c_ulong, prot: pgprot_t) -> *mut c_void {
    pte_t pteval, *kmap_pte;
    let mut vaddr = 0;
    let mut idx = 0;
//
// Disable migration so resulting virtual address is stable
// across preemption.
//
    migrate_disable();
    preempt_disable();
    idx = arch_kmap_local_map_idx(kmap_local_idx_push(), pfn);
    vaddr = __fix_to_virt(FIX_KMAP_BEGIN + idx);
    kmap_pte = kmap_get_pte(vaddr, idx);
    BUG_ON!(!pte_none(ptep_get(kmap_pte)));
    pteval = pfn_pte(pfn, prot);
    arch_kmap_local_set_pte(&init_mm, vaddr, kmap_pte, pteval);
    arch_kmap_local_post_map(vaddr, pteval);
    current.kmap_ctrl.pteval[kmap_local_idx()] = pteval;
    preempt_enable();
    return vaddr;
    }
    EXPORT_SYMBOL_GPL(__kmap_local_pfn_prot);
#[no_mangle]
pub unsafe extern "C" fn __kmap_local_page_prot(page: *mut page, prot: pgprot_t) -> *mut c_void {
pub static mut kmap: *mut c_void = core::ptr::null_mut();
//
// To broaden the usage of the actual kmap_local() machinery always map
// pages when debugging is enabled and the architecture has no problems
// with alias mappings.
//
    if (!IS_ENABLED!(CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP) && !PageHighMem(page)) {
    return page_address(page);
    }
// Try kmap_high_get() if architecture has it enabled
    kmap = arch_kmap_local_high_get(page);
    if (kmap) {
    return kmap;
    }
    return __kmap_local_pfn_prot(page_to_pfn(page), prot);
    }
    EXPORT_SYMBOL(__kmap_local_page_prot);
#[no_mangle]
pub unsafe extern "C" fn kunmap_local_indexed(vaddr: *const c_void) {
pub static mut addr: c_ulong = 0;
pub static mut kmap_pte: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (addr < __fix_to_virt(FIX_KMAP_END) ||
    addr > __fix_to_virt(FIX_KMAP_BEGIN)) {
    if (IS_ENABLED!(CONFIG_DEBUG_KMAP_LOCAL_FORCE_MAP)) {
// This _should_ never happen! See above.
    WARN_ON_ONCE!(1);
    return;
    }
//
// Handle mappings which were obtained by kmap_high_get()
// first as the virtual address of such mappings is below
// PAGE_OFFSET. Warn for all other addresses which are in
// the user space part of the virtual address space.
//
    if (!kmap_high_unmap_local(addr)) {
    WARN_ON_ONCE!(addr < PAGE_OFFSET);
    }
    return;
    }
    preempt_disable();
    idx = arch_kmap_local_unmap_idx(kmap_local_idx(), addr);
    WARN_ON_ONCE!(addr != __fix_to_virt(FIX_KMAP_BEGIN + idx));
    kmap_pte = kmap_get_pte(addr, idx);
    arch_kmap_local_pre_unmap(addr);
    pte_clear(&init_mm, addr, kmap_pte);
    arch_kmap_local_post_unmap(addr);
    current.kmap_ctrl.pteval[kmap_local_idx()] = __pte(0);
    kmap_local_idx_pop();
    preempt_enable();
    migrate_enable();
    }
    EXPORT_SYMBOL(kunmap_local_indexed);
//
// Invoked before switch_to(). This is safe even when during or after
// clearing the maps an interrupt which needs a kmap_local happens because
// the task::kmap_ctrl.idx is not modified by the unmapping code so a
// nested kmap_local will use the next unused index and restore the index
// on unmap. The already cleared kmaps of the outgoing task are irrelevant
// because the interrupt context does not know about them. The same applies
// when scheduling back in for an interrupt which happens before the
// restore is complete.
//
#[no_mangle]
pub unsafe extern "C" fn __kmap_local_sched_out() {
    let mut tsk = current;
pub static mut kmap_pte: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Clear kmaps
    while (i < tsk.kmap_ctrl.idx) {
pub static mut pteval: pte_t = 0;
    let mut addr = 0;
    let mut idx = 0;
// With debug all even slots are unmapped and act as guard
    if (IS_ENABLED!(CONFIG_DEBUG_KMAP_LOCAL) && !(i & 0x01)) {
    WARN_ON_ONCE!(pte_val(pteval) != 0);
    continue;
    }
    if (WARN_ON_ONCE!(pte_none(pteval))) {
    continue;
    }
//
// This is a horrible hack for XTENSA to calculate the
// coloured PTE index. Uses the PFN encoded into the pteval
// and the map index calculation because the actual mapped
// virtual address is not stored in task::kmap_ctrl.
// For any sane architecture this is optimized out.
//
    idx = arch_kmap_local_map_idx(i, pte_pfn(pteval));
    addr = __fix_to_virt(FIX_KMAP_BEGIN + idx);
    kmap_pte = kmap_get_pte(addr, idx);
    arch_kmap_local_pre_unmap(addr);
    pte_clear(&init_mm, addr, kmap_pte);
    arch_kmap_local_post_unmap(addr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kmap_local_sched_in() {
    let mut tsk = current;
pub static mut kmap_pte: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Restore kmaps
    while (i < tsk.kmap_ctrl.idx) {
pub static mut pteval: pte_t = 0;
    let mut addr = 0;
    let mut idx = 0;
// With debug all even slots are unmapped and act as guard
    if (IS_ENABLED!(CONFIG_DEBUG_KMAP_LOCAL) && !(i & 0x01)) {
    WARN_ON_ONCE!(pte_val(pteval) != 0);
    continue;
    }
    if (WARN_ON_ONCE!(pte_none(pteval))) {
    continue;
    }
// See comment in __kmap_local_sched_out()
    idx = arch_kmap_local_map_idx(i, pte_pfn(pteval));
    addr = __fix_to_virt(FIX_KMAP_BEGIN + idx);
    kmap_pte = kmap_get_pte(addr, idx);
    set_pte_at(&init_mm, addr, kmap_pte, pteval);
    arch_kmap_local_post_map(addr, pteval);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmap_local_fork(tsk: *mut task_struct) {
    if (WARN_ON_ONCE!(tsk.kmap_ctrl.idx)) {
    memset(&tsk.kmap_ctrl, 0, sizeof!(tsk.kmap_ctrl));
    }
    }

pub const PA_HASH_ORDER: c_int = 7;
//
// Describes one page->virtual association
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_address_map {
    pub page: *mut page,
    pub virtual: *mut c_void,
    pub list: list_head,
}

    static struct page_address_map page_address_maps[LAST_PKMAP];
//
// Hash table bucket
//
    static struct page_address_slot {
pub static mut lh: usize = 0;			/* List of page_address_maps */
    let mut lock;			/* Protect this bucket's list */
    } ____cacheline_aligned_in_smp page_address_htable[1<<PA_HASH_ORDER];
#[no_mangle]
pub unsafe extern "C" fn page_slot(page: *mut page) -> *mut c_void {
    return &page_address_htable[hash_ptr(page, PA_HASH_ORDER)];
    }
//
// page_address - get the mapped virtual address of a page
// @page: &struct page to get the virtual address of
//
// Returns the page's virtual address.
//
#[no_mangle]
pub unsafe extern "C" fn page_address(page: *mut page) -> *mut c_void {
    let mut flags = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut pas: *mut c_void = core::ptr::null_mut();
    if (!PageHighMem(page)) {
    return lowmem_page_address(page);
    }
    pas = page_slot(page);
    ret = core::ptr::null_mut();
    spin_lock_irqsave(&pas.lock, flags);
    if (!list_empty(&pas.lh)) {
pub static mut pam: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(pam, &pas.lh, list) {
    if (pam.page == page) {
    ret = pam.virtual;
    break;
    }
    }
    }
    spin_unlock_irqrestore(&pas.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(page_address);
//
// set_page_address - set a page's virtual address
// @page: &struct page to set
// @virtual: virtual address to use
//
#[no_mangle]
pub unsafe extern "C" fn set_page_address(page: *mut page, virtual: *mut c_void) {
    let mut flags = 0;
pub static mut pas: *mut c_void = core::ptr::null_mut();
pub static mut pam: *mut c_void = core::ptr::null_mut();
    BUG_ON!(!PageHighMem(page));
    pas = page_slot(page);
    if (virtual) {		/* Add */ {
    pam = &page_address_maps[PKMAP_NR((unsigned long)virtual)];
    }
    pam.page = page;
    pam.virtual = virtual;
    spin_lock_irqsave(&pas.lock, flags);
    list_add_tail(&pam.list, &pas.lh);
    spin_unlock_irqrestore(&pas.lock, flags);
    } else {		/* Remove */
    spin_lock_irqsave(&pas.lock, flags);
    list_for_each_entry(pam, &pas.lh, list) {
    if (pam.page == page) {
    list_del(&pam.list);
    break;
    }
    }
    spin_unlock_irqrestore(&pas.lock, flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn page_address_init()  {
    let mut i = 0;
    while (i < ARRAY_SIZE!(page_address_htable)) {
    INIT_LIST_HEAD(&page_address_htable[i].lh);
    spin_lock_init(&page_address_htable[i].lock);
    }
    }