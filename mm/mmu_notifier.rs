//! Automatically rewritten from C to Rust
//! Source: mm/mmu_notifier.c
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
// linux/mm/mmu_notifier.c
//
// Copyright (C) 2008  Qumranet, Inc.
// Copyright (C) 2008  SGI
// Christoph Lameter <cl@gentwo.org>
//

// global SRCU for all MMs
pub static mut srcu: usize = 0;

pub static mut lockdep_map: usize = 0;

//
// The mmu_notifier_subscriptions structure is allocated and installed in
// mm->notifier_subscriptions inside the mm_take_all_locks() protected
// critical section and it's released only when mm_count reaches zero
// in mmdrop().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_notifier_subscriptions {
// all mmu notifiers registered in this mm are queued in this list
    pub list: hlist_head,
    pub has_itree: bool,
// to serialize the list modifications and hlist_unhashed
    pub lock: spinlock_t,
    pub invalidate_seq: c_ulong,
    pub active_invalidate_ranges: c_ulong,
    pub itree: rb_root_cached,
    pub wq: wait_queue_head_t,
    pub deferred_list: hlist_head,
}

//
// This is a collision-retry read-side/write-side 'lock', a lot like a
// seqcount, however this allows multiple write-sides to hold it at
// once. Conceptually the write side is protecting the values of the PTEs in
// this mm, such that PTES cannot be read into SPTEs (shadow PTEs) while any
// writer exists.
//
// Note that the core mm creates nested invalidate_range_start()/end() regions
// within the same thread, and runs invalidate_range_start()/end() in parallel
// on multiple CPUs. This is designed to not reduce concurrency or block
// progress on the mm side.
//
// As a secondary function, holding the full write side also serves to prevent
// writers for the itree, this is an optimization to avoid extra locking
// during invalidate_range_start/end notifiers.
//
// The write side has two states, fully excluded:
// - mm->active_invalidate_ranges != 0
// - subscriptions->invalidate_seq & 1 == True (odd)
// - some range on the mm_struct is being invalidated
// - the itree is not allowed to change
//
// And partially excluded:
// - mm->active_invalidate_ranges != 0
// - subscriptions->invalidate_seq & 1 == False (even)
// - some range on the mm_struct is being invalidated
// - the itree is allowed to change
//
// Operations on notifier_subscriptions->invalidate_seq (under spinlock):
// seq |= 1  # Begin writing
// seq++     # Release the writing state
// seq & 1   # True if a writer exists
//
// The later state avoids some expensive work on inv_end in the common case of
// no mmu_interval_notifier monitoring the VA.
//
#[no_mangle]
pub unsafe extern "C" fn mn_itree_is_invalidating(subscriptions: *mut mmu_notifier_subscriptions) -> bool {
    lockdep_assert_held(&subscriptions.lock);
    return subscriptions.invalidate_seq & 1;
    }
#[no_mangle]
pub unsafe extern "C" fn mn_itree_inv_start_range(subscriptions: *mut mmu_notifier_subscriptions, range: *mut mmu_notifier_range, seq: *mut c_ulong) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut res = core::ptr::null_mut();
    spin_lock(&subscriptions.lock);
    subscriptions.active_invalidate_ranges += 1;
    node = interval_tree_iter_first(&subscriptions.itree, range.start,
    range.end - 1);
    if (node) {
    subscriptions.invalidate_seq |= 1;
    res = container_of!(node, mmu_interval_notifier,
    interval_tree);
    }
// seq = subscriptions->invalidate_seq;
    spin_unlock(&subscriptions.lock);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn mn_itree_inv_next(interval_sub: *mut mmu_interval_notifier, range: *mut mmu_notifier_range) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = interval_tree_iter_next(&interval_sub.interval_tree,
    range.start, range.end - 1);
    if (!node) {
    return core::ptr::null_mut();
    }
    return container_of!(node, mmu_interval_notifier, interval_tree);
    }
#[no_mangle]
unsafe extern "C" fn mn_itree_inv_end(subscriptions: *mut mmu_notifier_subscriptions) {
pub static mut interval_sub: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
    spin_lock(&subscriptions.lock);
    if (--subscriptions.active_invalidate_ranges ||
    !mn_itree_is_invalidating(subscriptions)) {
    spin_unlock(&subscriptions.lock);
    return;
    }
// Make invalidate_seq even
    subscriptions.invalidate_seq += 1;
//
// The inv_end incorporates a deferred mechanism like rtnl_unlock().
// Adds and removes are queued until the final inv_end happens then
// they are progressed. This arrangement for tree updates is used to
// avoid using a blocking lock during invalidate_range_start.
//
    hlist_for_each_entry_safe(interval_sub, next,
    &subscriptions.deferred_list,
    deferred_item) {
    if (RB_EMPTY_NODE(&interval_sub.interval_tree.rb)) {
// forward_decl: erval_tree_insert;
    }
    else {
// forward_decl: erval_tree_remove;
    }
    hlist_del(&interval_sub.deferred_item);
    }
    spin_unlock(&subscriptions.lock);
    wake_up_all(&subscriptions.wq);
    }
//
// mmu_interval_read_begin - Begin a read side critical section against a VA
// range
// @interval_sub: The interval subscription
//
// mmu_iterval_read_begin()/mmu_iterval_read_retry() implement a
// collision-retry scheme similar to seqcount for the VA range under
// subscription. If the mm invokes invalidation during the critical section
// then mmu_interval_read_retry() will return true.
//
// This is useful to obtain shadow PTEs where teardown or setup of the SPTEs
// require a blocking context.  The critical region formed by this can sleep,
// and the required 'user_lock' can also be a sleeping lock.
//
// The caller is required to provide a 'user_lock' to serialize both teardown
// and setup.
//
// The return value should be passed to mmu_interval_read_retry().
//
#[no_mangle]
pub unsafe extern "C" fn mmu_interval_read_begin(interval_sub: *mut mmu_interval_notifier) -> c_ulong {
    let mut subscriptions = interval_sub.mm.notifier_subscriptions;
    let mut seq = 0;
    let mut is_invalidating = 0;
//
// If the subscription has a different seq value under the user_lock
// than we started with then it has collided.
//
// If the subscription currently has the same seq value as the
// subscriptions seq, then it is currently between
// invalidate_start/end and is colliding.
//
// The locking looks broadly like this:
// mn_itree_inv_start():                 mmu_interval_read_begin():
// spin_lock
// seq = READ_ONCE(interval_sub->invalidate_seq);
// seq == subs->invalidate_seq
// spin_unlock
// spin_lock
// seq = ++subscriptions->invalidate_seq
// spin_unlock
// op->invalidate():
// user_lock
// mmu_interval_set_seq()
// interval_sub->invalidate_seq = seq
// user_unlock
//
// [Required: mmu_interval_read_retry() == true]
//
// mn_itree_inv_end():
// spin_lock
// seq = ++subscriptions->invalidate_seq
// spin_unlock
//
// user_lock
// mmu_interval_read_retry():
// interval_sub->invalidate_seq != seq
// user_unlock
//
// Barriers are not needed here as any races here are closed by an
// eventual mmu_interval_read_retry(), which provides a barrier via the
// user_lock.
//
    spin_lock(&subscriptions.lock);
// Pairs with the WRITE_ONCE in mmu_interval_set_seq()
    seq = READ_ONCE(interval_sub.invalidate_seq);
    is_invalidating = seq == subscriptions.invalidate_seq;
    spin_unlock(&subscriptions.lock);
//
// interval_sub->invalidate_seq must always be set to an odd value via
// mmu_interval_set_seq() using the provided cur_seq from
// mn_itree_inv_start_range(). This ensures that if seq does wrap we
// will always clear the below sleep in some reasonable time as
// subscriptions->invalidate_seq is even in the idle state.
//
    lock_map_acquire(&__mmu_notifier_invalidate_range_start_map);
    lock_map_release(&__mmu_notifier_invalidate_range_start_map);
    if (is_invalidating) {
    wait_event(subscriptions.wq,
    READ_ONCE(subscriptions.invalidate_seq) != seq);
    }
//
// Notice that mmu_interval_read_retry() can already be true at this
// point, avoiding loops here allows the caller to provide a global
// time bound.
//
    return seq;
    }
    EXPORT_SYMBOL_GPL(mmu_interval_read_begin);
#[no_mangle]
unsafe extern "C" fn mn_itree_finish_pass(finish_passes: *mut llist_head) {
    let mut first = llist_reverse_order(__llist_del_all(finish_passes));
    let mut f = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    llist_for_each_entry_safe(f, next, first, link) {
    f.notifier.ops.invalidate_finish(f);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mn_itree_release(subscriptions: *mut mmu_notifier_subscriptions, mm: *mut mm_struct) {
pub static mut mmu_notifier_range: usize = 0;
pub static mut interval_sub: *mut c_void = core::ptr::null_mut();
pub static mut finish_passes: usize = 0;
    let mut cur_seq = 0;
    let mut ret = 0;
    for (interval_sub =
    mn_itree_inv_start_range(subscriptions, &range, &cur_seq);
    interval_sub;
    interval_sub = mn_itree_inv_next(interval_sub, &range)) {
    if (interval_sub.ops.invalidate_start) {
    let mut finish = core::ptr::null_mut();
    ret = interval_sub.ops.invalidate_start(interval_sub,
    &range,
    cur_seq,
    &finish);
    if (ret && finish) {
    finish.notifier = interval_sub;
    __llist_add(&finish.link, &finish_passes);
    }
    } else {
    ret = interval_sub.ops.invalidate(interval_sub,
    &range,
    cur_seq);
    }
    WARN_ON!(!ret);
    }
    mn_itree_finish_pass(&finish_passes);
    mn_itree_inv_end(subscriptions);
    }
//
// This function can't run concurrently against mmu_notifier_register
// because mm->mm_users > 0 during mmu_notifier_register and exit_mmap
// runs with mm_users == 0. Other tasks may still invoke mmu notifiers
// in parallel despite there being no task using this mm any more,
// through the vmas outside of the exit_mmap context, such as with
// vmtruncate. This serializes against mmu_notifier_unregister with
// the notifier_subscriptions->lock in addition to SRCU and it serializes
// against the other mmu notifiers with SRCU. struct mmu_notifier_subscriptions
// can't go away from under us as exit_mmap holds an mm_count pin
// itself.
//
#[no_mangle]
pub unsafe extern "C" fn mn_hlist_release(subscriptions: *mut mmu_notifier_subscriptions, mm: *mut mm_struct) {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
//
// SRCU here will block mmu_notifier_unregister until
// ->release returns.
//
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription, &subscriptions.list, hlist,
    srcu_read_lock_held(&srcu))
//
// If ->release runs before mmu_notifier_unregister it must be
// handled, as it's the only way for the driver to flush all
// existing sptes and stop the driver from establishing any more
// sptes before all the pages in the mm are freed.
//
    if (subscription.ops.release) {
    subscription.ops.release(subscription, mm);
    }
    spin_lock(&subscriptions.lock);
    while (unlikely(!hlist_empty(&subscriptions.list))) {
    subscription = hlist_entry(subscriptions.list.first, mmu_notifier, hlist);
//
// We arrived before mmu_notifier_unregister so
// mmu_notifier_unregister will do nothing other than to wait
// for ->release to finish and for mmu_notifier_unregister to
// return.
//
    hlist_del_init_rcu(&subscription.hlist);
    }
    spin_unlock(&subscriptions.lock);
    srcu_read_unlock(&srcu, id);
//
// synchronize_srcu here prevents mmu_notifier_release from returning to
// exit_mmap (which would proceed with freeing all pages in the mm)
// until the ->release method returns, if it was invoked by
// mmu_notifier_unregister.
//
// The notifier_subscriptions can't go away from under us because
// one mm_count is held by exit_mmap.
//
    synchronize_srcu(&srcu);
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_release(mm: *mut mm_struct) {
    let mut subscriptions = mm.notifier_subscriptions;
    if (subscriptions.has_itree) {
    mn_itree_release(subscriptions, mm);
    }
    if (!hlist_empty(&subscriptions.list)) {
    mn_hlist_release(subscriptions, mm);
    }
    }
//
// If no young bitflag is supported by the hardware, ->clear_flush_young can
// unmap the address and return 1 or 0 depending if the mapping previously
// existed or not.
//
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_clear_flush_young(mm: *mut mm_struct, start: c_ulong, end: c_ulong) -> bool {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
pub static mut young: bool = false;
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription,
    &mm.notifier_subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    if (subscription.ops.clear_flush_young) {
    young |= subscription.ops.clear_flush_young(
    subscription, mm, start, end);
    }
    }
    srcu_read_unlock(&srcu, id);
    return young;
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_clear_young(mm: *mut mm_struct, start: c_ulong, end: c_ulong) -> bool {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
pub static mut young: bool = false;
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription,
    &mm.notifier_subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    if (subscription.ops.clear_young) {
    young |= subscription.ops.clear_young(subscription,
    mm, start, end);
    }
    }
    srcu_read_unlock(&srcu, id);
    return young;
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_test_young(mm: *mut mm_struct, address: c_ulong) -> bool {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
pub static mut young: bool = false;
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription,
    &mm.notifier_subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    if (subscription.ops.test_young) {
    young = subscription.ops.test_young(subscription, mm,
    address);
    if (young) {
    break;
    }
    }
    }
    srcu_read_unlock(&srcu, id);
    return young;
    }
#[no_mangle]
pub unsafe extern "C" fn mn_itree_invalidate(subscriptions: *mut mmu_notifier_subscriptions, range: *mut mmu_notifier_range) -> c_int {
pub static mut interval_sub: *mut c_void = core::ptr::null_mut();
pub static mut finish_passes: usize = 0;
    let mut cur_seq = 0;
pub static mut err: c_int = 0;
    for (interval_sub =
    mn_itree_inv_start_range(subscriptions, range, &cur_seq);
    interval_sub;
    interval_sub = mn_itree_inv_next(interval_sub, range)) {
    let mut ret = 0;
    if (interval_sub.ops.invalidate_start) {
    let mut finish = core::ptr::null_mut();
    ret = interval_sub.ops.invalidate_start(interval_sub,
    range,
    cur_seq,
    &finish);
    if (ret && finish) {
    finish.notifier = interval_sub;
    __llist_add(&finish.link, &finish_passes);
    }
    } else {
    ret = interval_sub.ops.invalidate(interval_sub,
    range,
    cur_seq);
    }
    if (!ret) {
    if (WARN_ON!(mmu_notifier_range_blockable(range))) {
    continue;
    }
    err = -EAGAIN;
    break;
    }
    }
    mn_itree_finish_pass(&finish_passes);
//
// On -EAGAIN the non-blocking caller is not allowed to call
// invalidate_range_end()
//
    if (err) {
    mn_itree_inv_end(subscriptions);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mn_hlist_invalidate_range_start(subscriptions: *mut mmu_notifier_subscriptions, range: *mut mmu_notifier_range) -> c_int {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription, &subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    let mut ops = subscription.ops;
    if (ops.invalidate_range_start) {
    let mut _ret = 0;
    if (!mmu_notifier_range_blockable(range)) {
    non_block_start();
    }
    _ret = ops.invalidate_range_start(subscription, range);
    if (!mmu_notifier_range_blockable(range)) {
    non_block_end();
    }
    if (_ret) {
    pr_info!("%pS callback failed with %d in %sblockable context.\n",
    ops.invalidate_range_start, _ret,
    !mmu_notifier_range_blockable(range) ?
    "non-" :
    "");
    WARN_ON!(mmu_notifier_range_blockable(range) ||
    _ret != -EAGAIN);
//
// We call all the notifiers on any EAGAIN,
// there is no way for a notifier to know if
// its start method failed, thus a start that
// does EAGAIN can't also do end.
//
    WARN_ON!(ops.invalidate_range_end);
    ret = _ret;
    }
    }
    }
    if (ret) {
//
// Must be non-blocking to get here.  If there are multiple
// notifiers and one or more failed start, any that succeeded
// start are expecting their end to be called.  Do so now.
//
    hlist_for_each_entry_srcu(subscription, &subscriptions.list,
    hlist, srcu_read_lock_held(&srcu)) {
    if (!subscription.ops.invalidate_range_end) {
    continue;
    }
    subscription.ops.invalidate_range_end(subscription,
    range);
    }
    }
    srcu_read_unlock(&srcu, id);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_invalidate_range_start(range: *mut mmu_notifier_range) -> c_int {
    let mut subscriptions = range.mm.notifier_subscriptions;
    let mut ret = 0;
    if (subscriptions.has_itree) {
    ret = mn_itree_invalidate(subscriptions, range);
    if (ret) {
    return ret;
    }
    }
    if (!hlist_empty(&subscriptions.list)) {
    return mn_hlist_invalidate_range_start(subscriptions, range);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mn_hlist_invalidate_end(subscriptions: *mut mmu_notifier_subscriptions, range: *mut mmu_notifier_range) {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription, &subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    if (subscription.ops.invalidate_range_end) {
    if (!mmu_notifier_range_blockable(range)) {
    non_block_start();
    }
    subscription.ops.invalidate_range_end(subscription,
    range);
    if (!mmu_notifier_range_blockable(range)) {
    non_block_end();
    }
    }
    }
    srcu_read_unlock(&srcu, id);
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_invalidate_range_end(range: *mut mmu_notifier_range) {
    let mut subscriptions = range.mm.notifier_subscriptions;
    lock_map_acquire(&__mmu_notifier_invalidate_range_start_map);
    if (subscriptions.has_itree) {
    mn_itree_inv_end(subscriptions);
    }
    if (!hlist_empty(&subscriptions.list)) {
    mn_hlist_invalidate_end(subscriptions, range);
    }
    lock_map_release(&__mmu_notifier_invalidate_range_start_map);
    }
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_arch_invalidate_secondary_tlbs(mm: *mut mm_struct, start: c_ulong, end: c_ulong) {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
    let mut id = 0;
    id = srcu_read_lock(&srcu);
    hlist_for_each_entry_srcu(subscription,
    &mm.notifier_subscriptions.list, hlist,
    srcu_read_lock_held(&srcu)) {
    if (subscription.ops.arch_invalidate_secondary_tlbs) {
    subscription.ops.arch_invalidate_secondary_tlbs(
    subscription, mm,
    start, end);
    }
    }
    srcu_read_unlock(&srcu, id);
    }
//
// Same as mmu_notifier_register but here the caller must hold the mmap_lock in
// write mode. A NULL mn signals the notifier is being registered for itree
// mode.
//
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> c_int {
    let mut subscriptions = core::ptr::null_mut();
    let mut ret = 0;
    mmap_assert_write_locked(mm);
    BUG_ON!(atomic_read(&mm.mm_users) <= 0);
//
// Subsystems should only register for invalidate_secondary_tlbs() or
// invalidate_range_start()/end() callbacks, not both.
//
    if (WARN_ON_ONCE!(subscription &&
    (subscription.ops.arch_invalidate_secondary_tlbs &&
    (subscription.ops.invalidate_range_start ||
    subscription.ops.invalidate_range_end)))) {
    return -EINVAL;
    }
    if (!mm.notifier_subscriptions) {
//
// kmalloc cannot be called under mm_take_all_locks(), but we
// know that mm->notifier_subscriptions can't change while we
// hold the write side of the mmap_lock.
//
    subscriptions = kzalloc_obj(mmu_notifier_subscriptions);
    if (!subscriptions) {
    return -ENOMEM;
    }
    INIT_HLIST_HEAD(&subscriptions.list);
    spin_lock_init(&subscriptions.lock);
    subscriptions.invalidate_seq = 2;
    subscriptions.itree = RB_ROOT_CACHED;
    init_waitqueue_head(&subscriptions.wq);
    INIT_HLIST_HEAD(&subscriptions.deferred_list);
    }
    ret = mm_take_all_locks(mm);
    if (unlikely(ret)) {
// goto;
    }
//
// Serialize the update against mmu_notifier_unregister. A
// side note: mmu_notifier_release can't run concurrently with
// us because we hold the mm_users pin (either implicitly as
// current->mm or explicitly with get_task_mm() or similar).
// We can't race against any other mmu notifier method either
// thanks to mm_take_all_locks().
//
// release semantics on the initialization of the
// mmu_notifier_subscriptions's contents are provided for unlocked
// readers.  acquire can only be used while holding the mmgrab or
// mmget, and is safe because once created the
// mmu_notifier_subscriptions is not freed until the mm is destroyed.
// As above, users holding the mmap_lock or one of the
// mm_take_all_locks() do not need to use acquire semantics.
//
    if (subscriptions) {
    smp_store_release(&mm.notifier_subscriptions, subscriptions);
    }
    if (subscription) {
// Pairs with the mmdrop in mmu_notifier_unregister_*
    mmgrab(mm);
    subscription.mm = mm;
    subscription.users = 1;
    spin_lock(&mm.notifier_subscriptions.lock);
    hlist_add_head_rcu(&subscription.hlist,
    &mm.notifier_subscriptions.list);
    spin_unlock(&mm.notifier_subscriptions.lock);
    } else {
    mm.notifier_subscriptions.has_itree = true;
    }
    mm_drop_all_locks(mm);
    BUG_ON!(atomic_read(&mm.mm_users) <= 0);
    return 0;
// label;
    kfree(subscriptions);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__mmu_notifier_register);
//
// mmu_notifier_register - Register a notifier on a mm
// @subscription: The notifier to attach
// @mm: The mm to attach the notifier to
//
// Must not hold mmap_lock nor any other VM related lock when calling
// this registration function. Must also ensure mm_users can't go down
// to zero while this runs to avoid races with mmu_notifier_release,
// so mm has to be current->mm or the mm should be pinned safely such
// as with get_task_mm(). If the mm is not current->mm, the mm_users
// pin should be released by calling mmput after mmu_notifier_register
// returns.
//
// mmu_notifier_unregister() or mmu_notifier_put() must be always called to
// unregister the notifier.
//
// While the caller has a mmu_notifier get the subscription->mm pointer will remain
// valid, and can be converted to an active mm pointer via mmget_not_zero().
//
#[no_mangle]
pub unsafe extern "C" fn mmu_notifier_register(subscription: *mut mmu_notifier, mm: *mut mm_struct) -> c_int {
    let mut ret = 0;
    mmap_write_lock(mm);
    ret = __mmu_notifier_register(subscription, mm);
    mmap_write_unlock(mm);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mmu_notifier_register);
#[no_mangle]
pub unsafe extern "C" fn find_get_mmu_notifier(mm: *mut mm_struct, ops: *mut mmu_notifier_ops) -> *mut c_void {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
    spin_lock(&mm.notifier_subscriptions.lock);
    hlist_for_each_entry_srcu(subscription,
    &mm.notifier_subscriptions.list, hlist,
    lockdep_is_held(&mm.notifier_subscriptions.lock)) {
    if (subscription.ops != ops) {
    continue;
    }
    if (likely(subscription.users != UINT_MAX)) {
    subscription.users += 1;
    }
    else {
    subscription = ERR_PTR(-EOVERFLOW);
    }
    spin_unlock(&mm.notifier_subscriptions.lock);
    return subscription;
    }
    spin_unlock(&mm.notifier_subscriptions.lock);
    return core::ptr::null_mut();
    }
//
// mmu_notifier_get_locked - Return the single struct mmu_notifier for
// the mm & ops
// @ops: The operations struct being subscribe with
// @mm : The mm to attach notifiers too
//
// This function either allocates a new mmu_notifier via
// ops->alloc_notifier(), or returns an already existing notifier on the
// list. The value of the ops pointer is used to determine when two notifiers
// are the same.
//
// Each call to mmu_notifier_get() must be paired with a call to
// mmu_notifier_put(). The caller must hold the write side of mm->mmap_lock.
//
// While the caller has a mmu_notifier get the mm pointer will remain valid,
// and can be converted to an active mm pointer via mmget_not_zero().
//
#[no_mangle]
pub unsafe extern "C" fn mmu_notifier_get_locked(ops: *mut mmu_notifier_ops, mm: *mut mm_struct) -> *mut c_void {
pub static mut subscription: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    mmap_assert_write_locked(mm);
    if (mm.notifier_subscriptions) {
    subscription = find_get_mmu_notifier(mm, ops);
    if (subscription) {
    return subscription;
    }
    }
    subscription = ops.alloc_notifier(mm);
    if (IS_ERR(subscription)) {
    return subscription;
    }
    subscription.ops = ops;
    ret = __mmu_notifier_register(subscription, mm);
    if (ret) {
// goto;
    }
    return subscription;
// label;
    subscription.ops.free_notifier(subscription);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(mmu_notifier_get_locked);
// this is called after the last mmu_notifier_unregister() returned
#[no_mangle]
pub unsafe extern "C" fn __mmu_notifier_subscriptions_destroy(mm: *mut mm_struct) {
    BUG_ON!(!hlist_empty(&mm.notifier_subscriptions.list));
    kfree(mm.notifier_subscriptions);
    mm.notifier_subscriptions = LIST_POISON1; /* debug */
    }
//
// This releases the mm_count pin automatically and frees the mm
// structure if it was the last user of it. It serializes against
// running mmu notifiers with SRCU and against mmu_notifier_unregister
// with the unregister lock + SRCU. All sptes must be dropped before
// calling mmu_notifier_unregister. ->release or any other notifier
// method may be invoked concurrently with mmu_notifier_unregister,
// and only after mmu_notifier_unregister returned we're guaranteed
// that ->release or any other method can't run anymore.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_notifier_unregister(subscription: *mut mmu_notifier, mm: *mut mm_struct) {
    BUG_ON!(atomic_read(&mm.mm_count) <= 0);
    if (!hlist_unhashed(&subscription.hlist)) {
//
// SRCU here will force exit_mmap to wait for ->release to
// finish before freeing the pages.
//
    let mut id = 0;
    id = srcu_read_lock(&srcu);
//
// exit_mmap will block in mmu_notifier_release to guarantee
// that ->release is called before freeing the pages.
//
    if (subscription.ops.release) {
    subscription.ops.release(subscription, mm);
    }
    srcu_read_unlock(&srcu, id);
    spin_lock(&mm.notifier_subscriptions.lock);
//
// Can not use list_del_rcu() since __mmu_notifier_release
// can delete it before we hold the lock.
//
    hlist_del_init_rcu(&subscription.hlist);
    spin_unlock(&mm.notifier_subscriptions.lock);
    }
//
// Wait for any running method to finish, of course including
// ->release if it was run by mmu_notifier_release instead of us.
//
    synchronize_srcu(&srcu);
    BUG_ON!(atomic_read(&mm.mm_count) <= 0);
    mmdrop(mm);
    }
    EXPORT_SYMBOL_GPL(mmu_notifier_unregister);
#[no_mangle]
unsafe extern "C" fn mmu_notifier_free_rcu(rcu: *mut rcu_head) {
    let mut subscription = container_of!(rcu, mmu_notifier, rcu);
    let mut mm = subscription.mm;
    subscription.ops.free_notifier(subscription);
// Pairs with the get in __mmu_notifier_register()
    mmdrop(mm);
    }
//
// mmu_notifier_put - Release the reference on the notifier
// @subscription: The notifier to act on
//
// This function must be paired with each mmu_notifier_get(), it releases the
// reference obtained by the get. If this is the last reference then process
// to free the notifier will be run asynchronously.
//
// Unlike mmu_notifier_unregister() the get/put flow only calls ops->release
// when the mm_struct is destroyed. Instead free_notifier is always called to
// release any resources held by the user.
//
// As ops->release is not guaranteed to be called, the user must ensure that
// all sptes are dropped, and no new sptes can be established before
// mmu_notifier_put() is called.
//
// This function can be called from the ops->release callback, however the
// caller must still ensure it is called pairwise with mmu_notifier_get().
//
// Modules calling this function must call mmu_notifier_synchronize() in
// their __exit functions to ensure the async work is completed.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_notifier_put(subscription: *mut mmu_notifier) {
    let mut mm = subscription.mm;
    spin_lock(&mm.notifier_subscriptions.lock);
    if (WARN_ON!(!subscription.users) || --subscription.users) {
// goto;
    }
    hlist_del_init_rcu(&subscription.hlist);
    spin_unlock(&mm.notifier_subscriptions.lock);
    call_srcu(&srcu, &subscription.rcu, mmu_notifier_free_rcu);
    return;
// label;
    spin_unlock(&mm.notifier_subscriptions.lock);
    }
    EXPORT_SYMBOL_GPL(mmu_notifier_put);
#[no_mangle]
pub unsafe extern "C" fn __mmu_interval_notifier_insert(interval_sub: *mut mmu_interval_notifier, mm: *mut mm_struct, subscriptions: *mut mmu_notifier_subscriptions, start: c_ulong, length: c_ulong, ops: *mut mmu_interval_notifier_ops) -> c_int {
    interval_sub.mm = mm;
    interval_sub.ops = ops;
    RB_CLEAR_NODE(&interval_sub.interval_tree.rb);
    interval_sub.interval_tree.start = start;
//
// Note that the representation of the intervals in the interval tree
// considers the ending point as contained in the interval.
//
    if (length == 0 ||
    check_add_overflow(start, length - 1,
    &interval_sub.interval_tree.last)) {
    return -EOVERFLOW;
    }
// Must call with a mmget() held
    if (WARN_ON!(atomic_read(&mm.mm_users) <= 0)) {
    return -EINVAL;
    }
// pairs with mmdrop in mmu_interval_notifier_remove()
    mmgrab(mm);
//
// If some invalidate_range_start/end region is going on in parallel
// we don't know what VA ranges are affected, so we must assume this
// new range is included.
//
// If the itree is invalidating then we are not allowed to change
// it. Retrying until invalidation is done is tricky due to the
// possibility for live lock, instead defer the add to
// mn_itree_inv_end() so this algorithm is deterministic.
//
// In all cases the value for the interval_sub->invalidate_seq should be
// odd, see mmu_interval_read_begin()
//
    spin_lock(&subscriptions.lock);
    if (subscriptions.active_invalidate_ranges) {
    if (mn_itree_is_invalidating(subscriptions)) {
    hlist_add_head(&interval_sub.deferred_item,
    &subscriptions.deferred_list);
    }
    else {
    subscriptions.invalidate_seq |= 1;
// forward_decl: erval_tree_insert;
    }
    interval_sub.invalidate_seq = subscriptions.invalidate_seq;
    } else {
    WARN_ON!(mn_itree_is_invalidating(subscriptions));
//
// The starting seq for a subscription not under invalidation
// should be odd, not equal to the current invalidate_seq and
// invalidate_seq should not 'wrap' to the new seq any time
// soon.
//
    interval_sub.invalidate_seq =
    subscriptions.invalidate_seq - 1;
// forward_decl: erval_tree_insert;
    }
    spin_unlock(&subscriptions.lock);
    return 0;
    }
//
// mmu_interval_notifier_insert - Insert an interval notifier
// @interval_sub: Interval subscription to register
// @start: Starting virtual address to monitor
// @length: Length of the range to monitor
// @mm: mm_struct to attach to
// @ops: Interval notifier operations to be called on matching events
//
// This function subscribes the interval notifier for notifications from the
// mm.  Upon return the ops related to mmu_interval_notifier will be called
// whenever an event that intersects with the given range occurs.
//
// Upon return the range_notifier may not be present in the interval tree yet.
// The caller must use the normal interval notifier read flow via
// mmu_interval_read_begin() to establish SPTEs for this range.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_interval_notifier_insert(interval_sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: c_ulong, length: c_ulong, ops: *mut mmu_interval_notifier_ops) -> c_int {
pub static mut subscriptions: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    WARN_ON_ONCE!(ops.invalidate_start && !ops.invalidate_finish);
    might_lock(&mm.mmap_lock);
    subscriptions = smp_load_acquire(&mm.notifier_subscriptions);
    if (!subscriptions || !subscriptions.has_itree) {
    ret = mmu_notifier_register(core::ptr::null_mut(), mm);
    if (ret) {
    return ret;
    }
    subscriptions = mm.notifier_subscriptions;
    }
    return __mmu_interval_notifier_insert(interval_sub, mm, subscriptions,
    start, length, ops);
    }
    EXPORT_SYMBOL_GPL(mmu_interval_notifier_insert);
#[no_mangle]
pub unsafe extern "C" fn mmu_interval_notifier_insert_locked(interval_sub: *mut mmu_interval_notifier, mm: *mut mm_struct, start: c_ulong, length: c_ulong, ops: *mut mmu_interval_notifier_ops) -> c_int {
    let mut subscriptions = mm.notifier_subscriptions;
    let mut ret = 0;
    mmap_assert_write_locked(mm);
    if (!subscriptions || !subscriptions.has_itree) {
    ret = __mmu_notifier_register(core::ptr::null_mut(), mm);
    if (ret) {
    return ret;
    }
    subscriptions = mm.notifier_subscriptions;
    }
    return __mmu_interval_notifier_insert(interval_sub, mm, subscriptions,
    start, length, ops);
    }
    EXPORT_SYMBOL_GPL(mmu_interval_notifier_insert_locked);
#[no_mangle]
pub unsafe extern "C" fn mmu_interval_seq_released(subscriptions: *mut mmu_notifier_subscriptions, seq: c_ulong) -> bool {
    let mut ret = 0;
    spin_lock(&subscriptions.lock);
    ret = subscriptions.invalidate_seq != seq;
    spin_unlock(&subscriptions.lock);
    return ret;
    }
//
// mmu_interval_notifier_remove - Remove a interval notifier
// @interval_sub: Interval subscription to unregister
//
// This function must be paired with mmu_interval_notifier_insert(). It cannot
// be called from any ops callback.
//
// Once this returns ops callbacks are no longer running on other CPUs and
// will not be called in future.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_interval_notifier_remove(interval_sub: *mut mmu_interval_notifier) {
    let mut mm = interval_sub.mm;
    let mut subscriptions = mm.notifier_subscriptions;
pub static mut seq: c_ulong = 0;
    might_sleep();
    spin_lock(&subscriptions.lock);
    if (mn_itree_is_invalidating(subscriptions)) {
//
// remove is being called after insert put this on the
// deferred list, but before the deferred list was processed.
//
    if (RB_EMPTY_NODE(&interval_sub.interval_tree.rb)) {
    hlist_del(&interval_sub.deferred_item);
    } else {
    hlist_add_head(&interval_sub.deferred_item,
    &subscriptions.deferred_list);
    seq = subscriptions.invalidate_seq;
    }
    } else {
    WARN_ON!(RB_EMPTY_NODE(&interval_sub.interval_tree.rb));
// forward_decl: erval_tree_remove;
    }
    spin_unlock(&subscriptions.lock);
//
// The possible sleep on progress in the invalidation requires the
// caller not hold any locks held by invalidation callbacks.
//
    lock_map_acquire(&__mmu_notifier_invalidate_range_start_map);
    lock_map_release(&__mmu_notifier_invalidate_range_start_map);
    if (seq) {
    wait_event(subscriptions.wq,
    mmu_interval_seq_released(subscriptions, seq));
    }
// pairs with mmgrab in mmu_interval_notifier_insert()
    mmdrop(mm);
    }
    EXPORT_SYMBOL_GPL(mmu_interval_notifier_remove);
//
// mmu_notifier_synchronize - Ensure all mmu_notifiers are freed
//
// This function ensures that all outstanding async SRU work from
// mmu_notifier_put() is completed. After it returns any mmu_notifier_ops
// associated with an unused mmu_notifier will no longer be called.
//
// Before using the caller must ensure that all of its mmu_notifiers have been
// fully released via mmu_notifier_put().
//
// Modules using the mmu_notifier_put() API should call this in their __exit
// function to avoid module unloading races.
//
#[no_mangle]
pub unsafe extern "C" fn mmu_notifier_synchronize() {
    synchronize_srcu(&srcu);
    }
    EXPORT_SYMBOL_GPL(mmu_notifier_synchronize);