//! Automatically rewritten from C to Rust
//! Source: mm/folio.c
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
// linux/mm/folio.c
//
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Folio LRU helpers: add/remove folios from LRU lists, batching,
// activation/deactivation, and page cache release paths.
//

// Macro flag: #define CREATE_TRACE_POINTS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_fbatches {
//
// The following folio batches are grouped together because they are protected
// by disabling preemption (and interrupts remain enabled).
//
    pub lock: local_lock_t,
    pub lru_add: folio_batch,
    pub lru_deactivate_file: folio_batch,
    pub lru_deactivate: folio_batch,
    pub lru_lazyfree: folio_batch,

    pub lru_activate: folio_batch,

// Protecting the following batches which require disabling interrupts
    pub lock_irq: local_lock_t,
    pub lru_move_tail: folio_batch,
}

    static DEFINE_PER_CPU(cpu_fbatches, cpu_fbatches) = {
    .lock = INIT_LOCAL_LOCK(lock),
    .lock_irq = INIT_LOCAL_LOCK(lock_irq),
    };
#[no_mangle]
pub unsafe extern "C" fn __page_cache_release(folio: *mut folio, lruvecp: *mut *mut lruvec, flagsp: *mut c_ulong) {
    if (folio_test_lru(folio)) {
    folio_lruvec_relock_irqsave(folio, lruvecp, flagsp);
    lruvec_del_folio(*lruvecp, folio);
    __folio_clear_lru_flags(folio);
    }
    }
//
// This path almost never happens for VM activity - pages are normally freed
// in batches.  But it gets used by networking - and for compound pages.
//
#[no_mangle]
unsafe extern "C" fn page_cache_release(folio: *mut folio) {
    let mut lruvec = core::ptr::null_mut();
    let mut flags = 0;
    __page_cache_release(folio, &lruvec, &flags);
    if (lruvec) {
    lruvec_unlock_irqrestore(lruvec, flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __folio_put(folio: *mut folio) {
    if (unlikely(folio_is_zone_device(folio))) {
    free_zone_device_folio(folio);
    return;
    }
    if (folio_test_hugetlb(folio)) {
    free_huge_folio(folio);
    return;
    }
    page_cache_release(folio);
    folio_unqueue_deferred_split(folio);
    mem_cgroup_uncharge(folio);
    free_frozen_pages(&folio.page, folio_order(folio));
    }
    EXPORT_SYMBOL(__folio_put);
    typedef void (*move_fn_t)(lruvec *lruvec, folio *folio);
#[no_mangle]
unsafe extern "C" fn lru_add(lruvec: *mut lruvec, folio: *mut folio) {
pub static mut was_unevictable: c_int = 0;
pub static mut nr_pages: c_long = 0;
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
//
// Is an smp_mb__after_atomic() still required here, before
// folio_evictable() tests the mlocked flag, to rule out the possibility
// of stranding an evictable folio on an unevictable LRU?  I think
// not, because __munlock_folio() only clears the mlocked flag
// while the LRU lock is held.
//
// (That is not true of __page_cache_release(), and not necessarily
// true of folios_put(): but those only clear the mlocked flag after
// folio_put_testzero() has excluded any other users of the folio.)
//
    if (folio_evictable(folio)) {
    if (was_unevictable) {
    __count_vm_events(UNEVICTABLE_PGRESCUED, nr_pages);
    }
    } else {
    folio_clear_active(folio);
    folio_set_unevictable(folio);
//
// folio->mlock_count = !!folio_test_mlocked(folio)?
// But that leaves __mlock_folio() in doubt whether another
// actor has already counted the mlock or not.  Err on the
// safe side, underestimate, let page reclaim fix it, rather
// than leaving a page on the unevictable LRU indefinitely.
//
    folio.mlock_count = 0;
    if (!was_unevictable) {
    __count_vm_events(UNEVICTABLE_PGCULLED, nr_pages);
    }
    }
    lruvec_add_folio(lruvec, folio);
    trace_mm_lru_insertion(folio);
    }
#[no_mangle]
unsafe extern "C" fn folio_batch_move_lru(fbatch: *mut folio_batch, move_fn: move_fn_t) {
    let mut i = 0;
    let mut lruvec = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
pub static mut free_fbatch: usize = 0;
pub static mut is_lru_add: bool = false;
//
// If we're adding to the LRU, preemptively filter dead folios. Use
// this dedicated folio batch for temp storage and deferred cleanup.
//
    if (is_lru_add) {
    folio_batch_init(&free_fbatch);
    }
    while (i < folio_batch_count(fbatch)) {
    let mut folio = fbatch.folios[i];
// block memcg migration while the folio moves between lru
    if (!is_lru_add && !folio_test_clear_lru(folio)) {
    continue;
    }
//
// Filter dead folios by moving them from the add batch to the temp
// batch for freeing after this loop.
//
// We're bypassing normal cleanup. Clear flags that are not
// applicable to dead folios.
//
// Since the folio may be part of a huge page, unqueue from
// deferred split list to avoid a dangling list entry.
//
    if (is_lru_add && folio_ref_freeze(folio, 1)) {
    __folio_clear_active(folio);
    __folio_clear_unevictable(folio);
    folio_unqueue_deferred_split(folio);
    fbatch.folios[i] = core::ptr::null_mut();
    folio_batch_add(&free_fbatch, folio);
    continue;
    }
    folio_lruvec_relock_irqsave(folio, &lruvec, &flags);
    move_fn(lruvec, folio);
    folio_set_lru(folio);
    }
    if (lruvec) {
    lruvec_unlock_irqrestore(lruvec, flags);
    }
// Cleanup filtered dead folios.
    if (is_lru_add) {
    mem_cgroup_uncharge_folios(&free_fbatch);
    free_unref_folios(&free_fbatch);
    }
    folios_put(fbatch);
    }
#[no_mangle]
pub unsafe extern "C" fn __folio_batch_add_and_move(fbatch: *mut folio_batch, folio: *mut folio, move_fn: move_fn_t, disable_irq: bool) {
    let mut flags = 0;
    folio_get(folio);
    if (disable_irq) {
    local_lock_irqsave(&cpu_fbatches.lock_irq, flags);
    }
    else {
    local_lock(&cpu_fbatches.lock);
    }
    if (!folio_batch_add(this_cpu_ptr(fbatch), folio) ||
    !folio_may_be_lru_cached(folio) || lru_cache_disabled()) {
    folio_batch_move_lru(this_cpu_ptr(fbatch), move_fn);
    }
    if (disable_irq) {
    local_unlock_irqrestore(&cpu_fbatches.lock_irq, flags);
    }
    else {
    local_unlock(&cpu_fbatches.lock);
    }
    }

    __folio_batch_add_and_move(			
    &cpu_fbatches.op,			
    folio,					
    op,					
    offsetof(cpu_fbatches, op) >=	
    offsetof(cpu_fbatches, lock_irq)	
    )
#[no_mangle]
unsafe extern "C" fn lru_move_tail(lruvec: *mut lruvec, folio: *mut folio) {
    if (folio_test_unevictable(folio)) {
    return;
    }
    lruvec_del_folio(lruvec, folio);
    folio_clear_active(folio);
    lruvec_add_folio_tail(lruvec, folio);
    __count_vm_events(PGROTATED, folio_nr_pages(folio));
    }
//
// Writeback is about to end against a folio which has been marked for
// immediate reclaim.  If it still appears to be reclaimable, move it
// to the tail of the inactive list.
//
// folio_rotate_reclaimable() must disable IRQs, to prevent nasty races.
//
#[no_mangle]
pub unsafe extern "C" fn folio_rotate_reclaimable(folio: *mut folio) {
    if (folio_test_locked(folio) || folio_test_dirty(folio) ||
    folio_test_unevictable(folio) || !folio_test_lru(folio)) {
    return;
    }
    folio_batch_add_and_move(folio, lru_move_tail);
    }
#[no_mangle]
unsafe extern "C" fn lru_activate(lruvec: *mut lruvec, folio: *mut folio) {
pub static mut nr_pages: c_long = 0;
    if (folio_test_active(folio) || folio_test_unevictable(folio)) {
    return;
    }
    lruvec_del_folio(lruvec, folio);
    folio_set_active(folio);
    lruvec_add_folio(lruvec, folio);
    trace_mm_lru_activate(folio);
    __count_vm_events(PGACTIVATE, nr_pages);
    count_memcg_events(lruvec_memcg(lruvec), PGACTIVATE, nr_pages);
    }

#[no_mangle]
unsafe extern "C" fn folio_activate_drain(cpu: c_int) {
    let mut fbatch = &per_cpu(cpu_fbatches.lru_activate, cpu);
    if (folio_batch_count(fbatch)) {
    folio_batch_move_lru(fbatch, lru_activate);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn folio_activate(folio: *mut folio) {
    if (folio_test_active(folio) || folio_test_unevictable(folio) ||
    !folio_test_lru(folio)) {
    return;
    }
    folio_batch_add_and_move(folio, lru_activate);
    }

#[no_mangle]
pub unsafe extern "C" fn folio_activate_drain(cpu: c_int) {
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: folio_activate
pub unsafe extern "C" fn folio_activate_dup(folio: *mut folio) {
pub static mut lruvec: *mut c_void = core::ptr::null_mut();
    if (!folio_test_clear_lru(folio)) {
    return;
    }
    lruvec = folio_lruvec_lock_irq(folio);
    lru_activate(lruvec, folio);
    lruvec_unlock_irq(lruvec);
    folio_set_lru(folio);
    }

#[no_mangle]
unsafe extern "C" fn __lru_cache_activate_folio(folio: *mut folio) {
pub static mut fbatch: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    local_lock(&cpu_fbatches.lock);
    fbatch = this_cpu_ptr(&cpu_fbatches.lru_add);
//
// Search backwards on the optimistic assumption that the folio being
// activated has just been added to this batch. Note that only
// the local batch is examined as a !LRU folio could be in the
// process of being released, reclaimed, migrated or on a remote
// batch that is currently being drained. Furthermore, marking
// a remote batch's folio active potentially hits a race where
// a folio is marked active just after it is added to the inactive
// list causing accounting errors and BUG_ON checks to trigger.
//
    while (i >= 0) {
    let mut batch_folio = fbatch.folios[i];
    if (batch_folio == folio) {
    folio_set_active(folio);
    break;
    }
    }
    local_unlock(&cpu_fbatches.lock);
    }

#[no_mangle]
unsafe extern "C" fn lru_gen_inc_refs(folio: *mut folio) {
    unsigned long new_flags, old_flags = READ_ONCE(folio.flags.f);
    if (folio_test_unevictable(folio)) {
    return;
    }
// see the comment on LRU_REFS_FLAGS
    if (!folio_test_referenced(folio)) {
    set_mask_bits(&folio.flags.f, LRU_REFS_MASK, BIT(PG_referenced));
    return;
    }
    do {
    if ((old_flags & LRU_REFS_MASK) == LRU_REFS_MASK) {
    if (!folio_test_workingset(folio)) {
    folio_set_workingset(folio);
    }
    return;
    }
    new_flags = old_flags + BIT(LRU_REFS_PGOFF);
    } while (!try_cmpxchg(&folio.flags.f, &old_flags, new_flags));
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_clear_refs(folio: *mut folio) -> bool {
pub static mut gen: c_int = 0;
pub static mut type: c_int = 0;
    let mut seq = 0;
    if (gen < 0) {
    return true;
    }
    set_mask_bits(&folio.flags.f, LRU_REFS_FLAGS | BIT(PG_workingset), 0);
    rcu_read_lock();
    seq = READ_ONCE(folio_lruvec(folio).lrugen.min_seq[type]);
    rcu_read_unlock();
// whether can do without shuffling under the LRU lock
pub static mut gen: return = 0;
    }

#[no_mangle]
unsafe extern "C" fn lru_gen_inc_refs(folio: *mut folio) {
    }
#[no_mangle]
unsafe extern "C" fn lru_gen_clear_refs(folio: *mut folio) -> bool {
    return false;
    }

//
// folio_mark_accessed - Mark a folio as having seen activity.
// @folio: The folio to mark.
//
// This function will perform one of the following transitions:
//
// * inactive,unreferenced	->	inactive,referenced
// * inactive,referenced	->	active,unreferenced
// * active,unreferenced	->	active,referenced
//
// When a newly allocated folio is not yet visible, so safe for non-atomic ops,
// __folio_set_referenced() may be substituted for folio_mark_accessed().
//
#[no_mangle]
pub unsafe extern "C" fn folio_mark_accessed(folio: *mut folio) {
    if (folio_test_dropbehind(folio)) {
    return;
    }
    if (lru_gen_enabled()) {
    lru_gen_inc_refs(folio);
    return;
    }
    if (!folio_test_referenced(folio)) {
    folio_set_referenced(folio);
    } else if (folio_test_unevictable(folio)) {
//
// Unevictable pages are on the "LRU_UNEVICTABLE" list. But,
// this list is never rotated or maintained, so marking an
// unevictable page accessed has no effect.
//
    } else if (!folio_test_active(folio)) {
//
// If the folio is on the LRU, queue it for activation via
// cpu_fbatches.lru_activate. Otherwise, assume the folio is in a
// folio_batch, mark it active and it'll be moved to the active
// LRU on the next drain.
//
    if (folio_test_lru(folio)) {
    folio_activate(folio);
    }
    else {
    __lru_cache_activate_folio(folio);
    }
    folio_clear_referenced(folio);
    workingset_activation(folio);
    }
    if (folio_test_idle(folio)) {
    folio_clear_idle(folio);
    }
    }
    EXPORT_SYMBOL(folio_mark_accessed);
//
// folio_add_lru - Add a folio to an LRU list.
// @folio: The folio to be added to the LRU.
//
// Queue the folio for addition to the LRU. The decision on whether
// to add the page to the [in]active [file|anon] list is deferred until the
// folio_batch is drained. This gives a chance for the caller of folio_add_lru()
// have the folio added to the active list using folio_mark_accessed().
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_lru(folio: *mut folio) {
    VM_BUG_ON_FOLIO(folio_test_active(folio) &&
    folio_test_unevictable(folio), folio);
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
//
// For refaulted workingset folios, set PG_active so they
// can be added to active generations.
// For prefaulted file folios, folio_mark_accessed() sets
// PG_referenced so lru_gen_folio_seq() places them into
// the second oldest generation.
//
    if (lru_gen_enabled() && !folio_test_unevictable(folio) &&
    lru_gen_in_fault() && !(current.flags & PF_MEMALLOC)) {
    if (folio_test_workingset(folio)) {
    folio_set_active(folio);
    }

    else if (!folio_test_referenced(folio)) {
    folio_mark_accessed(folio);
    }
    }
    folio_batch_add_and_move(folio, lru_add);
    }
    EXPORT_SYMBOL(folio_add_lru);
//
// folio_add_lru_vma() - Add a folio to the appropriate LRU list for this VMA.
// @folio: The folio to be added to the LRU.
// @vma: VMA in which the folio is mapped.
//
// If the VMA is mlocked, @folio is added to the unevictable list.
// Otherwise, it is treated the same way as folio_add_lru().
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_lru_vma(folio: *mut folio, vma: *mut vm_area_struct) {
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    if (unlikely((vma.vm_flags & (VM_LOCKED | VM_SPECIAL)) == VM_LOCKED)) {
    mlock_new_folio(folio);
    }
    else {
    folio_add_lru(folio);
    }
    }
//
// If the folio cannot be invalidated, it is moved to the
// inactive list to speed up its reclaim.  It is moved to the
// head of the list, rather than the tail, to give the flusher
// threads some time to write it out, as this is much more
// effective than the single-page writeout from reclaim.
//
// If the folio isn't mapped and dirty/writeback, the folio
// could be reclaimed asap using the reclaim flag.
//
// 1. active, mapped folio -> none
// 2. active, dirty/writeback folio -> inactive, head, reclaim
// 3. inactive, mapped folio -> none
// 4. inactive, dirty/writeback folio -> inactive, head, reclaim
// 5. inactive, clean -> inactive, tail
// 6. Others -> none
//
// In 4, it moves to the head of the inactive list so the folio is
// written out by flusher threads as this is much more efficient
// than the single-page writeout from reclaim.
//
#[no_mangle]
unsafe extern "C" fn lru_deactivate_file(lruvec: *mut lruvec, folio: *mut folio) {
pub static mut active: bool = false;
pub static mut nr_pages: c_long = 0;
    if (folio_test_unevictable(folio)) {
    return;
    }
// Some processes are using the folio
    if (folio_mapped(folio)) {
    return;
    }
    lruvec_del_folio(lruvec, folio);
    folio_clear_active(folio);
    folio_clear_referenced(folio);
    if (folio_test_writeback(folio) || folio_test_dirty(folio)) {
//
// Setting the reclaim flag could race with
// folio_end_writeback() and confuse readahead.  But the
// race window is _really_ small and  it's not a critical
// problem.
//
    lruvec_add_folio(lruvec, folio);
    folio_set_reclaim(folio);
    } else {
//
// The folio's writeback ended while it was in the batch.
// We move that folio to the tail of the inactive list.
//
    lruvec_add_folio_tail(lruvec, folio);
    __count_vm_events(PGROTATED, nr_pages);
    }
    if (active) {
    __count_vm_events(PGDEACTIVATE, nr_pages);
    count_memcg_events(lruvec_memcg(lruvec), PGDEACTIVATE,
    nr_pages);
    }
    }
#[no_mangle]
unsafe extern "C" fn lru_deactivate(lruvec: *mut lruvec, folio: *mut folio) {
pub static mut nr_pages: c_long = 0;
    if (folio_test_unevictable(folio) || !(folio_test_active(folio) || lru_gen_enabled())) {
    return;
    }
    lruvec_del_folio(lruvec, folio);
    folio_clear_active(folio);
    folio_clear_referenced(folio);
    lruvec_add_folio(lruvec, folio);
    __count_vm_events(PGDEACTIVATE, nr_pages);
    count_memcg_events(lruvec_memcg(lruvec), PGDEACTIVATE, nr_pages);
    }
#[no_mangle]
unsafe extern "C" fn lru_lazyfree(lruvec: *mut lruvec, folio: *mut folio) {
pub static mut nr_pages: c_long = 0;
    if (!folio_test_anon(folio) || !folio_test_swapbacked(folio) ||
    folio_test_swapcache(folio) || folio_test_unevictable(folio)) {
    return;
    }
    lruvec_del_folio(lruvec, folio);
    folio_clear_active(folio);
    if (lru_gen_enabled()) {
    lru_gen_clear_refs(folio);
    }
    else {
    folio_clear_referenced(folio);
    }
//
// Lazyfree folios are clean anonymous folios.  They have
// the swapbacked flag cleared, to distinguish them from normal
// anonymous folios
//
    folio_clear_swapbacked(folio);
    lruvec_add_folio(lruvec, folio);
    __count_vm_events(PGLAZYFREE, nr_pages);
    count_memcg_events(lruvec_memcg(lruvec), PGLAZYFREE, nr_pages);
    }
//
// Drain pages out of the cpu's folio_batch.
// Either "cpu" is the current CPU, and preemption has already been
// disabled; or "cpu" is being hot-unplugged, and is already dead.
//
#[no_mangle]
pub unsafe extern "C" fn lru_add_drain_cpu(cpu: c_int) {
    let mut fbatches = &per_cpu(cpu_fbatches, cpu);
    let mut fbatch = &fbatches.lru_add;
pub static mut nr_folios: c_uint = 0;
    if (nr_folios) {
    folio_batch_move_lru(fbatch, lru_add);
    trace_mm_lru_add_drain_tp(cpu, nr_folios);
    }
    fbatch = &fbatches.lru_move_tail;
// Disabling interrupts below acts as a compiler barrier.
    if (data_race(folio_batch_count(fbatch))) {
    let mut flags = 0;
// No harm done if a racing interrupt already did this
    local_lock_irqsave(&cpu_fbatches.lock_irq, flags);
    folio_batch_move_lru(fbatch, lru_move_tail);
    local_unlock_irqrestore(&cpu_fbatches.lock_irq, flags);
    }
    fbatch = &fbatches.lru_deactivate_file;
    if (folio_batch_count(fbatch)) {
    folio_batch_move_lru(fbatch, lru_deactivate_file);
    }
    fbatch = &fbatches.lru_deactivate;
    if (folio_batch_count(fbatch)) {
    folio_batch_move_lru(fbatch, lru_deactivate);
    }
    fbatch = &fbatches.lru_lazyfree;
    if (folio_batch_count(fbatch)) {
    folio_batch_move_lru(fbatch, lru_lazyfree);
    }
    folio_activate_drain(cpu);
    }
//
// deactivate_file_folio() - Deactivate a file folio.
// @folio: Folio to deactivate.
//
// This function hints to the VM that @folio is a good reclaim candidate,
// for example if its invalidation fails due to the folio being dirty
// or under writeback.
//
// Context: Caller holds a reference on the folio.
//
#[no_mangle]
pub unsafe extern "C" fn deactivate_file_folio(folio: *mut folio) {
// Deactivating an unevictable folio will not accelerate reclaim
    if (folio_test_unevictable(folio) || !folio_test_lru(folio)) {
    return;
    }
    if (lru_gen_enabled() && lru_gen_clear_refs(folio)) {
    return;
    }
    folio_batch_add_and_move(folio, lru_deactivate_file);
    }
//
// folio_deactivate - deactivate a folio
// @folio: folio to deactivate
//
// folio_deactivate() moves @folio to the inactive list if @folio was on the
// active list and was not unevictable. This is done to accelerate the
// reclaim of @folio.
//
#[no_mangle]
pub unsafe extern "C" fn folio_deactivate(folio: *mut folio) {
    if (folio_test_unevictable(folio) || !folio_test_lru(folio)) {
    return;
    }
    if (lru_gen_enabled() ? lru_gen_clear_refs(folio) : !folio_test_active(folio)) {
    return;
    }
    folio_batch_add_and_move(folio, lru_deactivate);
    }
//
// folio_mark_lazyfree - make an anon folio lazyfree
// @folio: folio to deactivate
//
// folio_mark_lazyfree() moves @folio to the inactive file list.
// This is done to accelerate the reclaim of @folio.
//
#[no_mangle]
pub unsafe extern "C" fn folio_mark_lazyfree(folio: *mut folio) {
    if (!folio_test_anon(folio) || !folio_test_swapbacked(folio) ||
    !folio_test_lru(folio) ||
    folio_test_swapcache(folio) || folio_test_unevictable(folio)) {
    return;
    }
    folio_batch_add_and_move(folio, lru_lazyfree);
    }
#[no_mangle]
pub unsafe extern "C" fn lru_add_drain() {
    local_lock(&cpu_fbatches.lock);
    lru_add_drain_cpu(smp_processor_id());
    local_unlock(&cpu_fbatches.lock);
    mlock_drain_local();
    }
//
// It's called from per-cpu workqueue context in SMP case so
// lru_add_drain_cpu and invalidate_bh_lrus_cpu should run on
// the same cpu. It shouldn't be a problem in !SMP case since
// the core is only one and the locks will disable preemption.
//
#[no_mangle]
unsafe extern "C" fn lru_add_and_bh_lrus_drain() {
    local_lock(&cpu_fbatches.lock);
    lru_add_drain_cpu(smp_processor_id());
    local_unlock(&cpu_fbatches.lock);
    invalidate_bh_lrus_cpu();
    mlock_drain_local();
    }
#[no_mangle]
pub unsafe extern "C" fn lru_add_drain_cpu_zone(zone: *mut zone) {
    local_lock(&cpu_fbatches.lock);
    lru_add_drain_cpu(smp_processor_id());
    drain_local_pages(zone);
    local_unlock(&cpu_fbatches.lock);
    mlock_drain_local();
    }

pub static mut struct work_struct: usize = 0;
#[no_mangle]
unsafe extern "C" fn lru_add_drain_per_cpu(dummy: *mut work_struct) {
    lru_add_and_bh_lrus_drain();
    }
#[no_mangle]
unsafe extern "C" fn cpu_needs_drain(cpu: c_uint) -> bool {
    let mut fbatches = &per_cpu(cpu_fbatches, cpu);
// Check these in order of likelihood that they're not zero
    return data_race(folio_batch_count(&fbatches.lru_add) ||
    folio_batch_count(&fbatches.lru_move_tail) ||
    folio_batch_count(&fbatches.lru_deactivate_file) ||
    folio_batch_count(&fbatches.lru_deactivate) ||
    folio_batch_count(&fbatches.lru_lazyfree) ||
    folio_batch_count(&fbatches.lru_activate) ||
    need_mlock_drain(cpu)) ||
    has_bh_in_lru(cpu, core::ptr::null_mut());
    }
//
// Doesn't need any cpu hotplug locking because we do rely on per-cpu
// kworkers being shut down before our page_alloc_cpu_dead callback is
// executed on the offlined cpu.
// Calling this function with cpu hotplug locks held can actually lead
// to obscure indirect dependencies via WQ context.
//
#[no_mangle]
pub unsafe extern "C" fn __lru_add_drain_all(force_all_cpus: bool) {
//
// lru_drain_gen - Global pages generation number
//
// (A) Definition: global lru_drain_gen = x implies that all generations
// 0 < n <= x are already *scheduled* for draining.
//
// This is an optimization for the highly-contended use case where a
// user space workload keeps constantly generating a flow of pages for
// each CPU.
//
    static unsigned int lru_drain_gen;
pub static mut has_work: usize = 0;
pub static mut lock: usize = 0;
    let mut cpu = 0;
    let mut this_gen = 0;
//
// Make sure nobody triggers this path before mm_percpu_wq is fully
// initialized.
//
    if (WARN_ON!(!mm_percpu_wq)) {
    return;
    }
    trace_mm_lru_add_drain_all_tp(force_all_cpus);
//
// Guarantee folio_batch counter stores visible by this CPU
// are visible to other CPUs before loading the current drain
// generation.
//
    smp_mb();
//
// (B) Locally cache global LRU draining generation number
//
// The read barrier ensures that the counter is loaded before the mutex
// is taken. It pairs with smp_mb() inside the mutex critical section
// at (D).
//
    this_gen = smp_load_acquire(&lru_drain_gen);
// It helps everyone if we do our own local drain immediately.
    lru_add_drain();
    mutex_lock(&lock);
//
// (C) Exit the draining operation if a newer generation, from another
// lru_add_drain_all(), was already scheduled for draining. Check (A).
//
    if (unlikely(this_gen != lru_drain_gen && !force_all_cpus)) {
// goto;
    }
//
// (D) Increment global generation number
//
// Pairs with smp_load_acquire() at (B), outside of the critical
// section. Use a full memory barrier to guarantee that the
// new global drain generation number is stored before loading
// folio_batch counters.
//
// This pairing must be done here, before the for_each_online_cpu loop
// below which drains the page vectors.
//
// Let x, y, and z represent some system CPU numbers, where x < y < z.
// Assume CPU #z is in the middle of the for_each_online_cpu loop
// below and has already reached CPU #y's per-cpu data. CPU #x comes
// along, adds some pages to its per-cpu vectors, then calls
// lru_add_drain_all().
//
// If the paired barrier is done at any later step, e.g. after the
// loop, CPU #x will just exit at (C) and miss flushing out all of its
// added pages.
//
    WRITE_ONCE(lru_drain_gen, lru_drain_gen + 1);
    smp_mb();
    cpumask_clear(&has_work);
    for_each_online_cpu(cpu) {
    let mut work = &per_cpu(lru_add_drain_work, cpu);
    if (cpu_needs_drain(cpu)) {
    INIT_WORK(work, lru_add_drain_per_cpu);
    queue_work_on(cpu, mm_percpu_wq, work);
    __cpumask_set_cpu(cpu, &has_work);
    }
    }
    for_each_cpu(cpu, &has_work) {
    flush_work(&per_cpu(lru_add_drain_work, cpu));
    }
// label;
    mutex_unlock(&lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lru_add_drain_all() {
    __lru_add_drain_all(false);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: lru_add_drain_all
pub unsafe extern "C" fn lru_add_drain_all_dup() {
    lru_add_drain();
    }

//
// lru_cache_drain_for_folio() - drain LRU caches if the caches might hold
// folio references
// @folio: The folio.
// @extra_refs: Extra folio references held by the caller.
// @drained: Drain status for batch folio processing.
//
// Drain LRU caches if the caches might hold folio references. Start
// with a local LRU cache drain, to then drain LRU caches on all CPUs if
// local draining was insufficient.
//
// This function detects LRU cache references by comparing the folio refcount
// with the sum of the expected folio refcount + extra references held by the
// caller. Note that we cannot rely on PG_lru to reliably detect all LRU
// cache references, and there are rare scenarios (concurrent folio (un)mapping)
// where this function might miss detecting LRU cache references.
//
// If @drained is not NULL, the function will avoid re-draining LRU caches
// when processing multiple folios in a row. In that case, the variable
// @drained points at must be initialized to LRU_CACHE_NOT_DRAINED before
// the first invocation by the caller.
//
#[no_mangle]
pub unsafe extern "C" fn lru_cache_drain_for_folio(folio: *mut folio, extra_refs: c_uint, drained: *mut lru_cache_drained) {
    if (!folio_may_be_lru_cached(folio)) {
    return;
    }
    if (!drained || *drained == LRU_CACHE_NOT_DRAINED) {
    if (folio_ref_count(folio) ==
    folio_expected_ref_count(folio) + extra_refs) {
    return;
    }
    lru_add_drain();
    if (drained) {
// drained = LRU_CACHE_DRAINED;
    }
    }
    if (!drained || *drained == LRU_CACHE_DRAINED) {
    if (folio_ref_count(folio) ==
    folio_expected_ref_count(folio) + extra_refs) {
    return;
    }
    lru_add_drain_all();
    if (drained) {
// drained = LRU_CACHE_DRAINED_ALL;
    }
    }
    }
pub static mut lru_disable_count: core::sync::atomic::AtomicI32 = 0;
//
// lru_cache_disable() needs to be called before we start compiling
// a list of folios to be migrated using folio_isolate_lru().
// It drains folios on LRU cache and then disable on all cpus until
// lru_cache_enable is called.
//
// Must be paired with a call to lru_cache_enable().
//
#[no_mangle]
pub unsafe extern "C" fn lru_cache_disable() {
    atomic_inc(&lru_disable_count);
//
// Readers of lru_disable_count are protected by either disabling
// preemption or rcu_read_lock:
//
// preempt_disable, local_irq_disable  [bh_lru_lock()]
// rcu_read_lock		       [rt_spin_lock CONFIG_PREEMPT_RT]
// preempt_disable		       [local_lock !CONFIG_PREEMPT_RT]
//
// Since v5.1 kernel, synchronize_rcu() is guaranteed to wait on
// preempt_disable() regions of code. So any CPU which sees
// lru_disable_count = 0 will have exited the critical
// section when synchronize_rcu() returns.
//
    synchronize_rcu_expedited();

    __lru_add_drain_all(true);

    lru_add_and_bh_lrus_drain();

    }
//
// folios_put_refs - Reduce the reference count on a batch of folios.
// @folios: The folios.
// @refs: The number of refs to subtract from each folio.
//
// Like folio_put(), but for a batch of folios.  This is more efficient
// than writing the loop yourself as it will optimise the locks which need
// to be taken if the folios are freed.  The folios batch is returned
// empty and ready to be reused for another batch; there is no need
// to reinitialise it.  If @refs is NULL, we subtract one from each
// folio refcount.
//
// Context: May be called in process or interrupt context, but not in NMI
// context.  May be called while holding a spinlock.
//
#[no_mangle]
pub unsafe extern "C" fn folios_put_refs(folios: *mut folio_batch, refs: *mut c_uint) {
    let mut i = 0;
    let mut j = 0;
    let mut lruvec = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    while (i < folios.nr) {
    let mut folio = folios.folios[i];
pub static mut nr_refs: c_uint = 0;
// Folio batch entry may have been preemptively removed during drain.
    if (!folio) {
    continue;
    }
    if (is_huge_zero_folio(folio)) {
    continue;
    }
    if (folio_is_zone_device(folio)) {
    if (lruvec) {
    lruvec_unlock_irqrestore(lruvec, flags);
    lruvec = core::ptr::null_mut();
    }
    if (folio_ref_sub_and_test(folio, nr_refs)) {
    free_zone_device_folio(folio);
    }
    continue;
    }
    if (!folio_ref_sub_and_test(folio, nr_refs)) {
    continue;
    }
// hugetlb has its own memcg
    if (folio_test_hugetlb(folio)) {
    if (lruvec) {
    lruvec_unlock_irqrestore(lruvec, flags);
    lruvec = core::ptr::null_mut();
    }
    free_huge_folio(folio);
    continue;
    }
    folio_unqueue_deferred_split(folio);
    __page_cache_release(folio, &lruvec, &flags);
    if (j != i) {
    folios.folios[j] = folio;
    }
    j += 1;
    }
    if (lruvec) {
    lruvec_unlock_irqrestore(lruvec, flags);
    }
    if (!j) {
    folio_batch_reinit(folios);
    return;
    }
    folios.nr = j;
    mem_cgroup_uncharge_folios(folios);
    free_unref_folios(folios);
    }
    EXPORT_SYMBOL(folios_put_refs);
//
// release_pages - batched put_page()
// @arg: array of pages to release
// @nr: number of pages
//
// Decrement the reference count on all the pages in @arg.  If it
// fell to zero, remove the page from the LRU and free it.
//
// Note that the argument can be an array of pages, encoded pages,
// or folio pointers. We ignore any encoded bits, and turn any of
// them into just a folio that gets free'd.
//
#[no_mangle]
pub unsafe extern "C" fn release_pages(arg: release_pages_arg, nr: c_int) {
pub static mut fbatch: usize = 0;
    int refs[FOLIO_BATCH_SIZE];
    let mut encoded = arg.encoded_pages;
    let mut i = 0;
    folio_batch_init(&fbatch);
    while (i < nr) {
// Turn any of the argument types into a folio
    let mut folio = page_folio(encoded_page_ptr(encoded[i]));
// Is our next entry actually "nr_pages" -> "nr_refs" ?
    refs[fbatch.nr] = 1;
    if (unlikely(encoded_page_flags(encoded[i]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT)) {
    refs[fbatch.nr] = encoded_nr_pages(encoded[++i]);
    }
    if (folio_batch_add(&fbatch, folio) > 0) {
    continue;
    }
    folios_put_refs(&fbatch, refs);
    }
    if (fbatch.nr) {
    folios_put_refs(&fbatch, refs);
    }
    }
    EXPORT_SYMBOL(release_pages);
//
// The folios which we're about to release may be in the deferred lru-addition
// queues.  That would prevent them from really being freed right now.  That's
// OK from a correctness point of view but is inefficient - those folios may be
// cache-warm and we want to give them back to the page allocator ASAP.
//
// So __folio_batch_release() will drain those queues here.
// folio_batch_move_lru() calls folios_put() directly to avoid
// mutual recursion.
//
#[no_mangle]
pub unsafe extern "C" fn __folio_batch_release(fbatch: *mut folio_batch) {
    if (!fbatch.percpu_pvec_drained) {
    lru_add_drain();
    fbatch.percpu_pvec_drained = true;
    }
    folios_put(fbatch);
    }
    EXPORT_SYMBOL(__folio_batch_release);
//
// folio_batch_remove_exceptionals() - Prune non-folios from a batch.
// @fbatch: The batch to prune
//
// find_get_entries() fills a batch with both folios and shadow/swap/DAX
// entries.  This function prunes all the non-folio entries from @fbatch
// without leaving holes, so that it can be passed on to folio-only batch
// operations.
//
#[no_mangle]
pub unsafe extern "C" fn folio_batch_remove_exceptionals(fbatch: *mut folio_batch) {
    let mut i = 0;
    let mut j = 0;
    while (i < folio_batch_count(fbatch)) {
    let mut folio = fbatch.folios[i];
    if (!xa_is_value(folio)) {
    fbatch.folios[j++] = folio;
    }
    }
    fbatch.nr = j;
    }

#[no_mangle]
pub unsafe extern "C" fn lruvec_reparent_lru(child_lruvec: *mut lruvec, parent_lruvec: *mut lruvec, lru: lru_list, nid: c_int) {
    let mut zid = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    if (lru != LRU_UNEVICTABLE) {
    list_splice_tail_init(&child_lruvec.lists[lru], &parent_lruvec.lists[lru]);
    }
    for_each_managed_zone_pgdat(zone, NODE_DATA(nid), zid, MAX_NR_ZONES - 1) {
pub static mut size: c_ulong = 0;
    if (!size) {
    continue;
    }
//
// The folios are accounted to the parent from now on, so the
// size has to be moved, not just copied. Leaving it behind
// makes the dying child describe folios it no longer owns.
//
    mem_cgroup_update_lru_size(parent_lruvec, lru, zid, size);
    mem_cgroup_update_lru_size(child_lruvec, lru, zid, -(long)size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lru_reparent_memcg(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, nid: c_int) {
    enum lru_list lru;
    let mut child_lruvec = core::ptr::null_mut();
    let mut parent_lruvec = core::ptr::null_mut();
    child_lruvec = mem_cgroup_lruvec(memcg, NODE_DATA(nid));
    parent_lruvec = mem_cgroup_lruvec(parent, NODE_DATA(nid));
    for_each_lru(lru) {
    lruvec_reparent_lru(child_lruvec, parent_lruvec, lru, nid);
    }
    }