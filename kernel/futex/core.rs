//! Automatically rewritten from C to Rust
//! Source: kernel/futex/core.c
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
// Fast Userspace Mutexes (which I call "Futexes!").
// (C) Rusty Russell, IBM 2002
//
// Generalized futexes, futex requeueing, misc fixes by Ingo Molnar
// (C) Copyright 2003 Red Hat Inc, All Rights Reserved
//
// Removed page pinning, fix privately mapped COW pages and other cleanups
// (C) Copyright 2003, 2004 Jamie Lokier
//
// Robust futex support started by Ingo Molnar
// (C) Copyright 2006 Red Hat Inc, All Rights Reserved
// Thanks to Thomas Gleixner for suggestions, analysis and fixes.
//
// PI-futex support started by Ingo Molnar and Thomas Gleixner
// Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright (C) 2006 Timesys Corp., Thomas Gleixner <tglx@timesys.com>
//
// PRIVATE futexes by Eric Dumazet
// Copyright (C) 2007 Eric Dumazet <dada1@cosmosbay.com>
//
// Requeue-PI support by Darren Hart <dvhltc@us.ibm.com>
// Copyright (C) IBM Corporation, 2009
// Thanks to Thomas Gleixner for conceptual design and careful reviews.
//
// Thanks to Ben LaHaise for yelling "hashed waitqueues" loudly
// enough at me, Linus for the original (flawed) idea, Matthew
// Kirkwood for proof-of-concept implementation.
//
// "The futexes are also cursed."
// "But they come in a choice of three flavours!"
//

    static u32 __futex_mask __ro_after_init;
    static u32 __futex_shift __ro_after_init;
pub static mut __futex_queues: *mut c_void = core::ptr::null_mut();
    static __always_inline struct futex_hash_bucket **futex_queues(void)
    {
    return runtime_const_ptr(__futex_queues);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_private_hash {
    pub state: c_int,
    pub hash_mask: c_uint,
    pub rcu: rcu_head,
    pub mm: *mut c_void,
    pub custom: bool,
    pub queues: [futex_hash_bucket; 0],
}

//
// Fault injections for futexes.
//

pub static mut fail_futex: usize = 0;
#[no_mangle]
unsafe extern "C" fn setup_fail_futex(str: *mut c_char) -> c_int {
    return setup_fault_attr(&fail_futex.attr, str);
    }
    __setup!("fail_futex=", setup_fail_futex);
#[no_mangle]
pub unsafe extern "C" fn should_fail_futex(fshared: bool) -> bool {
    if (fail_futex.ignore_private && !fshared) {
    return false;
    }
    return should_fail(&fail_futex.attr, 1);
    }

#[no_mangle]
unsafe extern "C" fn fail_futex_debugfs() -> c_int {
pub static mut mode: umode_t = 0;
pub static mut dir: *mut c_void = core::ptr::null_mut();
    dir = fault_create_debugfs_attr("fail_futex", core::ptr::null_mut(),
    &fail_futex.attr);
    if (IS_ERR(dir)) {
    return PTR_ERR(dir);
    }
    debugfs_create_bool("ignore-private", mode, dir,
    &fail_futex.ignore_private);
    return 0;
    }
    late_initcall!(fail_futex_debugfs);

// forward_decl: __futex_hash;

// forward_decl: futex_ref_get;
// forward_decl: futex_ref_put;
// forward_decl: futex_ref_is_dead;
    enum { FR_PERCPU = 0, FR_ATOMIC };
#[no_mangle]
unsafe extern "C" fn futex_private_hash_get(fph: *mut futex_private_hash) -> bool {
    return futex_ref_get(fph);
    }
#[no_mangle]
pub unsafe extern "C" fn futex_private_hash_put(fph: *mut futex_private_hash) {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    if (!fph) {
    return;
    }
    mm = fph.mm;
    if (futex_ref_put(fph)) {
    wake_up_var(mm);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __futex_hash_private(key: *mut union futex_key, fph: *mut futex_private_hash) -> *mut c_void {
    let mut hash = 0;
    hash = jhash2(&key.private.address, sizeof!(key.private.address) / 4,
    key.both.offset);
    return &fph.queues[hash & fph.hash_mask];
    }
#[no_mangle]
pub unsafe extern "C" fn futex_rehash_private(old: *mut futex_private_hash, new: *mut futex_private_hash) {
    let mut hb_old = core::ptr::null_mut();
    let mut hb_new = core::ptr::null_mut();
pub static mut slots: c_uint = 0;
    let mut i = 0;
    while (i < slots) {
    let mut this = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    hb_old = &old.queues[i];
    spin_lock(&hb_old.lock);
    plist_for_each_entry_safe(this, tmp, &hb_old.chain, list) {
    plist_del(&this.list, &hb_old.chain);
    futex_hb_waiters_dec(hb_old);
    WARN_ON_ONCE!(this.lock_ptr != &hb_old.lock);
    hb_new = __futex_hash(&this.key, new, core::ptr::null_mut());
    futex_hb_waiters_inc(hb_new);
//
// The new pointer isn't published yet but an already
// moved user can be unqueued due to timeout or signal.
//
    spin_lock_nested(&hb_new.lock, SINGLE_DEPTH_NESTING);
    plist_add(&this.list, &hb_new.chain);
    this.lock_ptr = &hb_new.lock;
    spin_unlock(&hb_new.lock);
    }
    spin_unlock(&hb_old.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn __futex_pivot_hash(mm: *mut mm_struct, new: *mut futex_private_hash) -> bool {
    let mut mmph = &mm.futex.phash;
pub static mut fph: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(mmph.hash_new);
    fph = rcu_dereference_protected(mmph.hash, lockdep_is_held(&mmph.lock));
    if (fph) {
    if (!futex_ref_is_dead(fph)) {
    mmph.hash_new = new;
    return false;
    }
    futex_rehash_private(fph, new);
    }
    new.state = FR_PERCPU;
    scoped_guard(rcu) {
    mmph.batches = get_state_synchronize_rcu();
    rcu_assign_pointer(mmph.hash, new);
    }
    kvfree_rcu(fph, rcu);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn futex_pivot_hash(mm: *mut mm_struct) {
    scoped_guard(mutex, &mm.futex.phash.lock) {
pub static mut fph: *mut c_void = core::ptr::null_mut();
    fph = mm.futex.phash.hash_new;
    if (fph) {
    mm.futex.phash.hash_new = core::ptr::null_mut();
    __futex_pivot_hash(mm, fph);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn futex_private_hash(mm: *mut mm_struct) -> *mut c_void {
//
// Ideally we don't loop. If there is a replacement in progress
// then a new private hash is already prepared and a reference can't be
// obtained once the last user dropped it's.
// In that case we block on mm_struct::futex_hash_lock and either have
// to perform the replacement or wait while someone else is doing the
// job. Eitherway, on the second iteration we acquire a reference on the
// new private hash or loop again because a new replacement has been
// requested.
//
// label;
    scoped_guard(rcu) {
pub static mut fph: *mut c_void = core::ptr::null_mut();
    fph = rcu_dereference(mm.futex.phash.hash);
    if (!fph) {
    return core::ptr::null_mut();
    }
    if (futex_private_hash_get(fph)) {
    return fph;
    }
    }
    futex_pivot_hash(mm);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_hash(key: *mut union futex_key) -> futex_bucket_ref {
// label;
    scoped_guard(rcu) {
    let mut fph = core::ptr::null_mut();
pub static mut hb: *mut c_void = core::ptr::null_mut();
    hb = __futex_hash(key, core::ptr::null_mut(), &fph);
    if (!fph || futex_private_hash_get(fph)) {
    return (futex_bucket_ref){ .hb = hb, .fph = fph };
    }
    }
    futex_pivot_hash(key.private.mm);
// goto;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: futex_hash
pub unsafe extern "C" fn futex_hash_dup(key: *mut union futex_key) -> futex_bucket_ref {
    return (futex_bucket_ref){ .hb = __futex_hash(key, core::ptr::null_mut(), core::ptr::null_mut()), .fph = core::ptr::null_mut() };
    }

#[no_mangle]
unsafe extern "C" fn __futex_key_to_node(mm: *mut mm_struct, addr: c_ulong) -> c_int {
    let mut vma = vma_lookup(mm, addr);
pub static mut mpol: *mut c_void = core::ptr::null_mut();
pub static mut node: c_int = 0;
    if (!vma) {
    return FUTEX_NO_NODE;
    }
    mpol = READ_ONCE(vma.vm_policy);
    if (!mpol) {
    return FUTEX_NO_NODE;
    }
    match (mpol.mode) {
    MPOL_PREFERRED => {
    node = first_node(mpol.nodes);
    // break;
    }
    MPOL_PREFERRED_MANY => {
    }
    MPOL_BIND => {
    if (mpol.home_node != NUMA_NO_NODE) {
    node = mpol.home_node;
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn futex_key_to_node_opt(mm: *mut mm_struct, addr: c_ulong) -> c_int {
    let mut seq = 0;
    let mut node = 0;
    guard(rcu)();
    if (!mmap_lock_speculate_try_begin(mm, &seq)) {
    return -EBUSY;
    }
    node = __futex_key_to_node(mm, addr);
    if (mmap_lock_speculate_retry(mm, seq)) {
    return -EAGAIN;
    }
    return node;
    }
#[no_mangle]
unsafe extern "C" fn futex_mpol(mm: *mut mm_struct, addr: c_ulong) -> c_int {
    let mut node = 0;
    node = futex_key_to_node_opt(mm, addr);
    if (node >= FUTEX_NO_NODE) {
    return node;
    }
    guard(mmap_read_lock)(mm);
    return __futex_key_to_node(mm, addr);
    }

#[no_mangle]
unsafe extern "C" fn futex_mpol(mm: *mut mm_struct, addr: c_ulong) -> c_int {
    return FUTEX_NO_NODE;
    }

//
// __futex_hash - Return the hash bucket
// @key:	Pointer to the futex key for which the hash is calculated
// @fph:	Pointer to private hash if known
// @fph_p:	Pointer to a private hash pointer; output for the private hash
// used when set.
//
// We hash on the keys returned from get_futex_key (see below) and return the
// corresponding hash bucket.
// If the FUTEX is PROCESS_PRIVATE then a per-process hash bucket (from the
// private hash) is returned if existing. Otherwise a hash bucket from the
// global hash is returned.
//
#[no_mangle]
pub unsafe extern "C" fn __futex_hash(key: *mut union futex_key, fph: *mut futex_private_hash, fph_p: *mut *mut futex_private_hash) -> *mut c_void {
pub static mut node: c_int = 0;
    let mut hash = 0;

    if (node == FUTEX_NO_NODE && futex_key_is_private(key)) {
    if (!fph) {
    fph = rcu_dereference(key.private.mm.futex.phash.hash);
    }
    if (fph && fph.hash_mask) {
    if (fph_p) {
// fph_p = fph;
    }
    return __futex_hash_private(key, fph);
    }
    }

    hash = jhash2(key, offsetof(typeof(*key), both.offset) / sizeof!(u32),
    key.both.offset);
    if (node == FUTEX_NO_NODE) {
//
// In case of !FLAGS_NUMA, use some unused hash bits to pick a
// node -- this ensures regular futexes are interleaved across
// the nodes and avoids having to allocate multiple
// hash-tables.
//
// NOTE: this isn't perfectly uniform, but it is fast and
// handles sparse node masks.
//
    node = runtime_const_shift_right_32(hash, __futex_shift) % nr_node_ids;
    if (!node_possible(node)) {
    node = find_next_bit_wrap(node_possible_map.bits, nr_node_ids, node);
    }
    }
    return &futex_queues()[node][runtime_const_mask_32(hash, __futex_mask)];
    }
//
// futex_setup_timer - set up the sleeping hrtimer.
// @time:	ptr to the given timeout value
// @timeout:	the hrtimer_sleeper structure to be set up
// @flags:	futex flags
// @range_ns:	optional range in ns
//
// Return: Initialized hrtimer_sleeper structure or NULL if no timeout
// value given
//
#[no_mangle]
pub unsafe extern "C" fn futex_setup_timer(time: *mut ktime_t, timeout: *mut hrtimer_sleeper, flags: c_int, range_ns: u64) -> *mut c_void {
    if (!time) {
    return core::ptr::null_mut();
    }
    hrtimer_setup_sleeper_on_stack(timeout,
    (flags & FLAGS_CLOCKRT) ? CLOCK_REALTIME : CLOCK_MONOTONIC,
    HRTIMER_MODE_ABS);
//
// If range_ns is 0, calling hrtimer_set_expires_range_ns() is
// effectively the same as calling hrtimer_set_expires().
//
    hrtimer_set_expires_range_ns(&timeout.timer, *time, range_ns);
    return timeout;
    }
//
// Generate a machine wide unique identifier for this inode.
//
// This relies on u64 not wrapping in the life-time of the machine; which with
// 1ns resolution means almost 585 years.
//
// This further relies on the fact that a well formed program will not unmap
// the file while it has a (shared) futex waiting on it. This mapping will have
// a file reference which pins the mount and inode.
//
// If for some reason an inode gets evicted and read back in again, it will get
// a new sequence number and will _NOT_ match, even though it is the exact same
// file.
//
// It is important that futex_match() will never have a false-positive, esp.
// for PI futexes that can mess up the state. The above argues that false-negatives
// are only possible for malformed programs.
//
#[no_mangle]
unsafe extern "C" fn get_inode_sequence_number(inode: *mut inode) -> u64 {
    static atomic64_t i_seq;
    let mut old = 0;
// Does the inode already have a sequence number?
    old = atomic64_read(&inode.i_sequence);
    if (likely(old)) {
    return old;
    }
    for (;;) {
pub static mut new: u64 = 0;
    if (WARN_ON_ONCE!(!new)) {
    continue;
    }
    old = 0;
    if (!atomic64_try_cmpxchg_relaxed(&inode.i_sequence, &old, new)) {
    return old;
    }
    return new;
    }
    }
//
// get_futex_key() - Get parameters which are the keys for a futex
// @uaddr:	virtual address of the futex
// @flags:	FLAGS_
// @key:	address where result is stored.
// @rw:		mapping needs to be read/write (values: FUTEX_READ,
FUTEX_WRITE)
//
// Return: a negative error code or 0
//
// The key words are stored in @key on success.
//
// For shared mappings (when @fshared), the key is:
//
// ( inode->i_sequence, page offset within mapping, offset_within_page )
//
// [ also see get_inode_sequence_number() ]
//
// For private mappings (or when !@fshared), the key is:
//
// ( current->mm, address, 0 )
//
// This allows (cross process, where applicable) identification of the futex
// without keeping the page pinned for the duration of the FUTEX_WAIT.
//
// lock_page() might sleep, the caller should not hold a spinlock.
//
#[no_mangle]
pub unsafe extern "C" fn get_futex_key(uaddr: *mut u32, flags: c_uint, key: *mut union futex_key, rw: futex_access) -> c_int {
pub static mut address: c_ulong = 0;
    let mut mm = current.mm;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    int node, err, size, ro = 0;
pub static mut node_updated: bool = false;
    let mut fshared = 0;
    fshared = flags & FLAGS_SHARED;
    size = futex_size(flags);
    if (flags & FLAGS_NUMA) {
    size *= 2;
    }
//
// The futex address must be "naturally" aligned.
//
    key.both.offset = address % PAGE_SIZE;
    if (unlikely((address & (size-1)) != 0)) {
    return -EINVAL;
    }
    address -= key.both.offset;
    if (unlikely(!access_ok(uaddr, size))) {
    return -EFAULT;
    }
    if (unlikely(should_fail_futex(fshared))) {
    return -EFAULT;
    }
    node = FUTEX_NO_NODE;
    if (flags & FLAGS_NUMA) {
    let mut naddr = uaddr + size / 2;
    if (get_user_inline(node, naddr)) {
    return -EFAULT;
    }
    if ((node != FUTEX_NO_NODE) &&
    ((unsigned int)node >= MAX_NUMNODES || !node_possible(node))) {
    return -EINVAL;
    }
    }
    if (node == FUTEX_NO_NODE && (flags & FLAGS_MPOL)) {
    node = futex_mpol(mm, address);
    node_updated = true;
    }
    if (flags & FLAGS_NUMA) {
    let mut naddr = uaddr + size / 2;
    if (node == FUTEX_NO_NODE) {
    node = numa_node_id();
    node_updated = true;
    }
    if (node_updated && put_user_inline(node, naddr)) {
    return -EFAULT;
    }
    }
    key.both.node = node;
//
// PROCESS_PRIVATE futexes are fast.
// As the mm cannot disappear under us and the 'key' only needs
// virtual address, we dont even have to find the underlying vma.
// Note : We do have to check 'uaddr' is a valid user address,
but access_ok() should be faster than find_vma()
//
    if (!fshared) {
//
// On no-MMU, shared futexes are treated as private, therefore
// we must not include the current process in the key. Since
// there is only one address space, the address is a unique key
// on its own.
//
    if (IS_ENABLED!(CONFIG_MMU)) {
    key.private.mm = mm;
    }
    else {
    key.private.mm = core::ptr::null_mut();
    }
    key.private.address = address;
    return 0;
    }
// label;
// Ignore any VERIFY_READ mapping (futex common case)
    if (unlikely(should_fail_futex(true))) {
    return -EFAULT;
    }
    err = get_user_pages_fast(address, 1, FOLL_WRITE, &page);
//
// If write access is not required (eg. FUTEX_WAIT), try
// and get read-only access.
//
    if (err == -EFAULT && rw == FUTEX_READ) {
    err = get_user_pages_fast(address, 1, 0, &page);
    ro = 1;
    }
    if (err < 0) {
    return err;
    }
    else {
    err = 0;
    }
//
// The treatment of mapping from this point on is critical. The folio
// lock protects many things but in this context the folio lock
// stabilizes mapping, prevents inode freeing in the shared
// file-backed region case and guards against movement to swap cache.
//
// Strictly speaking the folio lock is not needed in all cases being
// considered here and folio lock forces unnecessarily serialization.
// From this point on, mapping will be re-verified if necessary and
// folio lock will be acquired only if it is unavoidable
//
// Mapping checks require the folio so it is looked up now. For
// anonymous pages, it does not matter if the folio is split
// in the future as the key is based on the address. For
// filesystem-backed pages, the precise page is required as the
// index of the page determines the key.
//
    folio = page_folio(page);
    mapping = READ_ONCE(folio.mapping);
//
// If folio->mapping is NULL, then it cannot be an anonymous
// page; but it might be the ZERO_PAGE or in the gate area or
// in a special mapping (all cases which we are happy to fail);
// or it may have been a good file page when get_user_pages_fast
// found it, but truncated or holepunched or subjected to
// invalidate_complete_page2 before we got the folio lock (also
// cases which we are happy to fail).  And we hold a reference,
// so refcount care in invalidate_inode_page's remove_mapping
// prevents drop_caches from setting mapping to NULL beneath us.
//
// The case we do have to guard against is when memory pressure made
// shmem_writepage move it from filecache to swapcache beneath us:
// an unlikely race, but we do need to retry for folio->mapping.
//
    if (unlikely(!mapping)) {
    let mut shmem_swizzled = 0;
//
// Folio lock is required to identify which special case above
// applies. If this is really a shmem page then the folio lock
// will prevent unexpected transitions.
//
    folio_lock(folio);
    shmem_swizzled = folio_test_swapcache(folio) || folio.mapping;
    folio_unlock(folio);
    folio_put(folio);
    if (shmem_swizzled) {
// goto;
    }
    return -EFAULT;
    }
//
// Private mappings are handled in a simple way.
//
// If the futex key is stored in anonymous memory, then the associated
// object is the mm which is implicitly pinned by the calling process.
//
// NOTE: When userspace waits on a MAP_SHARED mapping, even if
// it's a read-only handle, it's expected that futexes attach to
// the object not the particular process.
//
    if (folio_test_anon(folio)) {
//
// A RO anonymous page will never change and thus doesn't make
// sense for futex operations.
//
    if (unlikely(should_fail_futex(true)) || ro) {
    err = -EFAULT;
// goto;
    }
    key.both.offset |= FUT_OFF_MMSHARED; /* ref taken on mm */
    key.private.mm = mm;
    key.private.address = address;
    } else {
pub static mut inode: *mut c_void = core::ptr::null_mut();
//
// The associated futex object in this case is the inode and
// the folio->mapping must be traversed. Ordinarily this should
// be stabilised under folio lock but it's not strictly
// necessary in this case as we just want to pin the inode, not
// update i_pages or anything like that.
//
// The RCU read lock is taken as the inode is finally freed
// under RCU. If the mapping still matches expectations then the
// mapping->host can be safely accessed as being a valid inode.
//
    rcu_read_lock();
    if (READ_ONCE(folio.mapping) != mapping) {
    rcu_read_unlock();
    folio_put(folio);
// goto;
    }
    inode = READ_ONCE(mapping.host);
    if (!inode) {
    rcu_read_unlock();
    folio_put(folio);
// goto;
    }
    key.both.offset |= FUT_OFF_INODE; /* inode-based key */
    key.shared.i_seq = get_inode_sequence_number(inode);
    key.shared.pgoff = page_pgoff(folio, page);
    rcu_read_unlock();
    }
// label;
    folio_put(folio);
    return err;
    }
//
// fault_in_user_writeable() - Fault in user address and verify RW access
// @uaddr:	pointer to faulting user space address
//
// Slow path to fixup the fault we just took in the atomic write
// access to @uaddr.
//
// We have no generic implementation of a non-destructive write to the
// user address. We know that we faulted in the atomic pagefault
// disabled section so we can as well avoid the #PF overhead by
// calling get_user_pages() right away.
//
#[no_mangle]
pub unsafe extern "C" fn fault_in_user_writeable(uaddr: *mut u32 ) -> c_int {
    let mut mm = current.mm;
    let mut ret = 0;
    mmap_read_lock(mm);
    ret = fixup_user_fault(mm, (unsigned long)uaddr,
    FAULT_FLAG_WRITE, core::ptr::null_mut());
    mmap_read_unlock(mm);
    return ret < 0 ? ret : 0;
    }
//
// futex_top_waiter() - Return the highest priority waiter on a futex
// @hb:		the hash bucket the futex_q's reside in
// @key:	the futex key (to distinguish it from other futex futex_q's)
//
// Must be called with the hb lock held.
//
#[no_mangle]
pub unsafe extern "C" fn futex_top_waiter(hb: *mut futex_hash_bucket, key: *mut union futex_key) -> *mut c_void {
pub static mut this: *mut c_void = core::ptr::null_mut();
    plist_for_each_entry(this, &hb.chain, list) {
    if (futex_match(&this.key, key)) {
    return this;
    }
    }
    return core::ptr::null_mut();
    }
//
// wait_for_owner_exiting - Block until the owner has exited
// @ret: owner's current futex lock status
// @exiting:	Pointer to the exiting task
//
// Caller must hold a refcount on @exiting.
//
#[no_mangle]
pub unsafe extern "C" fn wait_for_owner_exiting(ret: c_int, exiting: *mut task_struct) {
    if (ret != -EBUSY) {
    WARN_ON_ONCE!(exiting);
    return;
    }
    if (WARN_ON_ONCE!(ret == -EBUSY && !exiting)) {
    return;
    }
    mutex_lock(&exiting.futex.exit_mutex);
//
// No point in doing state checking here. If the waiter got here
// while the task was in exec()->exec_futex_release() then it can
// have any FUTEX_STATE_* value when the waiter has acquired the
// mutex. OK, if running, EXITING or DEAD if it reached exit()
// already. Highly unlikely and not a problem. Just one more round
// through the futex maze.
//
    mutex_unlock(&exiting.futex.exit_mutex);
    put_task_struct(exiting);
    }
//
// __futex_unqueue() - Remove the futex_q from its futex_hash_bucket
// @q:	The futex_q to unqueue
//
// The q->lock_ptr must not be NULL and must be held by the caller.
//
#[no_mangle]
pub unsafe extern "C" fn __futex_unqueue(q: *mut futex_q) {
pub static mut hb: *mut c_void = core::ptr::null_mut();
    if (WARN_ON_SMP(!q.lock_ptr) || WARN_ON!(plist_node_empty(&q.list))) {
    return;
    }
    lockdep_assert_held(q.lock_ptr);
    hb = container_of!(q.lock_ptr, futex_hash_bucket, lock);
    plist_del(&q.list, &hb.chain);
    futex_hb_waiters_dec(hb);
    }
// The key must be already stored in q->key.
#[no_mangle]
pub unsafe extern "C" fn futex_q_lock(q: *mut futex_q, hb: *mut futex_hash_bucket) {
//
// Increment the counter before taking the lock so that
// a potential waker won't miss a to-be-slept task that is
// waiting for the spinlock. This is safe as all futex_q_lock()
// users end up calling futex_queue(). Similarly, for housekeeping,
// decrement the counter at futex_q_unlock() when some error has
// occurred and we don't end up adding the task to the list.
//
    futex_hb_waiters_inc(hb); /* implies smp_mb(); (A) */
    q.lock_ptr = &hb.lock;
    spin_lock(&hb.lock);
    __acquire(q.lock_ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn futex_q_unlock(hb: *mut futex_hash_bucket) {
    futex_hb_waiters_dec(hb);
    spin_unlock(&hb.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __futex_queue(q: *mut futex_q, hb: *mut futex_hash_bucket, task: *mut task_struct) {
    let mut prio = 0;
//
// The priority used to register this element is
// - either the real thread-priority for the real-time threads
// (i.e. threads with a priority lower than MAX_RT_PRIO)
// - or MAX_RT_PRIO for non-RT threads.
// Thus, all RT-threads are woken first in priority order, and
// the others are woken last, in FIFO order.
//
    prio = min(current.normal_prio, MAX_RT_PRIO);
    plist_node_init(&q.list, prio);
    plist_add(&q.list, &hb.chain);
    q.task = task;
    }
//
// futex_unqueue() - Remove the futex_q from its futex_hash_bucket
// @q:	The futex_q to unqueue
//
// The q->lock_ptr must not be held by the caller. A call to futex_unqueue() must
// be paired with exactly one earlier call to futex_queue().
//
// Return:
// - 1 - if the futex_q was still queued (and we removed unqueued it);
// - 0 - if the futex_q was already removed by the waking thread
//
#[no_mangle]
pub unsafe extern "C" fn futex_unqueue(q: *mut futex_q) -> c_int {
pub static mut lock_ptr: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// RCU so lock_ptr is not going away during locking.
    guard(rcu)();
// In the common case we don't take the spinlock, which is nice.
// label;
//
// q->lock_ptr can change between this read and the following spin_lock.
// Use READ_ONCE to forbid the compiler from reloading q->lock_ptr and
// optimizing lock_ptr out of the logic below.
//
    lock_ptr = READ_ONCE(q.lock_ptr);
    if (lock_ptr != core::ptr::null_mut()) {
    spin_lock(lock_ptr);
//
// q->lock_ptr can change between reading it and
// spin_lock(), causing us to take the wrong lock.  This
// corrects the race condition.
//
// Reasoning goes like this: if we have the wrong lock,
q.lock_ptr must have changed (maybe several times)
// between reading it and the spin_lock().  It can
// change again after the spin_lock() but only if it was
// already changed before the spin_lock().  It cannot,
// however, change back to the original value.  Therefore
// we can detect whether we acquired the correct lock.
//
    if (unlikely(lock_ptr != q.lock_ptr)) {
    spin_unlock(lock_ptr);
// goto;
    }
    __futex_unqueue(q);
    BUG_ON!(q.pi_state);
    spin_unlock(lock_ptr);
    ret = 1;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_q_lockptr_lock(q: *mut futex_q) {
pub static mut lock_ptr: *mut c_void = core::ptr::null_mut();
//
// See futex_unqueue() why lock_ptr can change.
//
    guard(rcu)();
// label;
    lock_ptr = READ_ONCE(q.lock_ptr);
    spin_lock(lock_ptr);
    if (unlikely(lock_ptr != q.lock_ptr)) {
    spin_unlock(lock_ptr);
// goto;
    }
    }
//
// PI futexes can not be requeued and must remove themselves from the hash
// bucket. The hash bucket lock (i.e. lock_ptr) is held.
//
#[no_mangle]
pub unsafe extern "C" fn futex_unqueue_pi(q: *mut futex_q) {
//
// If the lock was not acquired (due to timeout or signal) then the
// rt_waiter is removed before futex_q is. If this is observed by
// an unlocker after dropping the rtmutex wait lock and before
// acquiring the hash bucket lock, then the unlocker dequeues the
// futex_q from the hash bucket list to guarantee consistent state
// vs. userspace. Therefore the dequeue here must be conditional.
//
    if (!plist_node_empty(&q.list)) {
    __futex_unqueue(q);
    }
    BUG_ON!(!q.pi_state);
    put_pi_state(q.pi_state);
    q.pi_state = core::ptr::null_mut();
    }
// Constants for the pending_op argument of handle_futex_death

//
// Process a futex-list entry, check whether it's owned by the
// dying task, and do notification if so:
//
#[no_mangle]
pub unsafe extern "C" fn handle_futex_death(uaddr: *mut u32, curr: *mut task_struct, mod: c_uint, pending_op: bool) -> c_int {
pub static mut pi: bool = false;
    u32 uval, nval, mval;
    let mut owner = 0;
    let mut err = 0;
// Futex address must be 32bit aligned
    if ((((unsigned long)uaddr) % sizeof!(*uaddr)) != 0) {
    return -1;
    }
// label;
    if (get_user(uval, uaddr)) {
    return -1;
    }
//
// Special case for regular (non PI) futexes. Ordinarily, we do
// not perform any processing here unless the current thread was
// the owner of the futex (by the TID check below).
//
// However, the unlock path has three race scenarios:
//
// 1. The unlock path releases the user space futex value and
// before it can execute the futex() syscall to wake up
// waiters it is killed.
//
// 2. A woken up waiter is killed before it can acquire the
// futex in user space.
//
// 3. A woken up waiter is killed in user space after another
// thread has acquired the futex, but before it can set
// FUTEX_WAITERS.
//
// Note that, if userspace uses the FUTEX_ROBUST_UNLOCK flag, we
// will not see case 1 here.
//
// In the second and third case, the wake up notification could
// be generated from any of:
//
// i.   An ordinary futex wakeup after unlock (with or
// without FUTEX_ROBUST_UNLOCK)
// ii.  A robust wakeup from another thread's death
// iii. A previous round through this special case
//
// As a result, the futex world will be in one of four states:
//
// A. The futex word is 0 (unlocked)
// B. The futex word is owned by another thread
// (FUTEX_WAITERS is not set)
// C. The futex word is owned by another thread
// (FUTEX_WAITERS set)
// D. The futex's owner died and OWNER_DIED is set
// (the owner part of the word is 0)
//
// The key issue is that the kernel usually (at least from
// sources ii. and iii. or when so requested by userspace from
// source i.) only ever wakes *one* waiter at a time. If this
// waiter dies before acquiring the futex (or setting the
// FUTEX_WAITERS bit), the kernel *must* still wake the next
// waiter down the line to uphold the futex invariants and
// avoid lost wakeups. Note we do not need to handle state C,
// as it does not matter to us whether *we* successfully set
// the bit or a third thread did so in the meantime.
//
// Therefore, in these cases we must issue an additional
// futex_wake(). Note however that we *must not* set OWNER_DIED
// here. Our thread is *not* the owner of the futex.
//
// Thus to summarize, the conditions for needing the additional
// futex_wake() are:
//
// 1) @pending_op == true (the thread has not finished the
// mutex operation)
// 2) The futex word is in one of the states A, B or D
// 3) Regular futex: @pi == false
//
// Note in particular that in all of the states A-D the owner
// portion of the futex word differs from our thread's TID
// (unless the actual owner has the same TID in another PID
// namespace, but we cannot currently distinguish that
// scenario), so this can be a special-case wakeup in the bail
// path of the ordinary TID check.
//
    owner = uval & FUTEX_TID_MASK;
    if (owner != task_pid_vnr(curr)) {
    if (pending_op && !pi && (!owner || !(uval & FUTEX_WAITERS))) {
    futex_wake(uaddr, FLAGS_SIZE_32 | FLAGS_SHARED, core::ptr::null_mut(), 1,
    FUTEX_BITSET_MATCH_ANY);
    }
    return 0;
    }
//
// Ok, this dying thread is truly holding a futex
// of interest. Set the OWNER_DIED bit atomically
// via cmpxchg, and if the value had FUTEX_WAITERS
// set, wake up a waiter (if any). (We have to do a
// futex_wake() even if OWNER_DIED is already set -
// to handle the rare but possible case of recursive
// thread-death.) The rest of the cleanup is done in
// userspace.
//
    mval = (uval & FUTEX_WAITERS) | FUTEX_OWNER_DIED;
//
// We are not holding a lock here, but we want to have
// the pagefault_disable/enable() protection because
// we want to handle the fault gracefully. If the
// access fails we try to fault in the futex with R/W
// verification via get_user_pages. get_user() above
// does not guarantee R/W access. If that fails we
// give up and leave the futex locked.
//
    if ((err = futex_cmpxchg_value_locked(&nval, uaddr, uval, mval))) {
    match (err) {
    -EFAULT => {
    if (fault_in_user_writeable(uaddr)) {
    return -1;
    }
// goto;
    }
    -EAGAIN => {
    cond_resched();
// goto;
    }
    _ => {
    WARN_ON_ONCE!(1);
    return err;
    }
    }
    }
    if (nval != uval) {
// goto;
    }
//
// Wake robust non-PI futexes here. The wakeup of
// PI futexes happens in exit_pi_state():
//
    if (!pi && (uval & FUTEX_WAITERS)) {
    futex_wake(uaddr, FLAGS_SIZE_32 | FLAGS_SHARED, core::ptr::null_mut(), 1,
    FUTEX_BITSET_MATCH_ANY);
    }
    return 0;
    }
//
// Fetch a robust-list pointer. Bit 0 signals PI futexes:
//
#[no_mangle]
pub unsafe extern "C" fn fetch_robust_entry(entry: *mut *mut robust_list, head: *mut *mut robust_list, mod: *mut c_uint) -> c_int {
    let mut uentry = 0;
    if (get_user(uentry, head)) {
    return -EFAULT;
    }
// entry = (uentry & ~FUTEX_ROBUST_MOD_MASK);
// mod = uentry & FUTEX_ROBUST_MOD_MASK;
    return 0;
    }
//
// Walk curr->futex.robust_list (very carefully, it's a userspace list!)
// and mark any locks found there dead, and notify any waiters.
//
// We silently return on any sign of list-walking problem.
//
#[no_mangle]
unsafe extern "C" fn exit_robust_list(curr: *mut task_struct) {
    let mut head = curr.futex.robust_list;
pub static mut limit: c_uint = 0;
    let mut entry = core::ptr::null_mut();
    let mut next_entry = core::ptr::null_mut();
    let mut pending = core::ptr::null_mut();
    let mut futex_offset = 0;
    let mut rc = 0;
//
// Fetch the list head (which was registered earlier, via
// sys_set_robust_list()):
//
    if (fetch_robust_entry(&entry, &head.list.next, &cur_mod)) {
    return;
    }
//
// Fetch the relative futex offset:
//
    if (get_user(futex_offset, &head.futex_offset)) {
    return;
    }
//
// Fetch any possibly pending lock-add first, and handle it
// if it exists:
//
    if (fetch_robust_entry(&pending, &head.list_op_pending, &pend_mod)) {
    return;
    }
    next_entry = core::ptr::null_mut();	/* avoid warning with gcc */
    while (entry != &head.list) {
//
// Fetch the next entry in the list before calling
// handle_futex_death:
//
    rc = fetch_robust_entry(&next_entry, &entry.next, &next_mod);
//
// A pending lock might already be on the list, so
// don't process it twice:
//
    if (entry != pending) {
    if (handle_futex_death(entry + futex_offset,
    curr, cur_mod, HANDLE_DEATH_LIST)) {
    return;
    }
    }
    if (rc) {
    return;
    }
    entry = next_entry;
    cur_mod = next_mod;
//
// Avoid excessively long or circular lists:
//
    if (!--limit) {
    break;
    }
    cond_resched();
    }
    if (pending) {
    handle_futex_death(pending + futex_offset,
    curr, pend_mod, HANDLE_DEATH_PENDING);
    }
    }
#[no_mangle]
unsafe extern "C" fn robust_list_clear_pending(pop: *mut unsigned long ) -> bool {
    let mut head = current.futex.robust_list;
    if (!put_user(0UL, pop)) {
    return true;
    }
//
// Just give up. The robust list head is usually part of TLS, so the
// chance that this gets resolved is close to zero.
//
// If @pop_addr is the robust_list_head::list_op_pending pointer then
// clear the robust list head pointer to prevent further damage when the
// task exits.  Better a few stale futexes than corrupted memory. But
// that's mostly an academic exercise.
//
    if (pop == &head.list_op_pending) {
    current.futex.robust_list = core::ptr::null_mut();
    }
    return false;
    }

    static void  *futex_uaddr(robust_list  *entry,
    compat_long_t futex_offset)
    {
pub static mut base: compat_uptr_t = 0;
    let mut uaddr = compat_ptr(base + futex_offset);
    return uaddr;
    }
//
// Fetch a robust-list pointer. Bit 0 signals PI futexes:
//
#[no_mangle]
pub unsafe extern "C" fn compat_fetch_robust_entry(uentry: *mut compat_uptr_t, entry: *mut *mut robust_list, head: *mut compat_uptr_t, pflags: *mut c_uint) -> c_int {
    if (get_user(*uentry, head)) {
    return -EFAULT;
    }
// entry = compat_ptr((*uentry) & ~FUTEX_ROBUST_MOD_MASK);
// pflags = (unsigned int)(*uentry) & FUTEX_ROBUST_MOD_MASK;
    return 0;
    }
//
// Walk curr->futex.robust_list (very carefully, it's a userspace list!)
// and mark any locks found there dead, and notify any waiters.
//
// We silently return on any sign of list-walking problem.
//
#[no_mangle]
unsafe extern "C" fn compat_exit_robust_list(curr: *mut task_struct) {
    let mut head = current.futex.compat_robust_list;
pub static mut limit: c_uint = 0;
    let mut entry = core::ptr::null_mut();
    let mut next_entry = core::ptr::null_mut();
    let mut pending = core::ptr::null_mut();
    compat_uptr_t uentry, next_uentry, upending;
    let mut futex_offset;
    let mut rc = 0;
//
// Fetch the list head (which was registered earlier, via
// sys_set_robust_list()):
//
    if (compat_fetch_robust_entry(&uentry, &entry, &head.list.next, &cur_mod)) {
    return;
    }
//
// Fetch the relative futex offset:
//
    if (get_user(futex_offset, &head.futex_offset)) {
    return;
    }
//
// Fetch any possibly pending lock-add first, and handle it
// if it exists:
//
    if (compat_fetch_robust_entry(&upending, &pending, &head.list_op_pending, &pend_mod)) {
    return;
    }
    next_entry = core::ptr::null_mut();	/* avoid warning with gcc */
    while (entry !=  &head.list) {
//
// Fetch the next entry in the list before calling
// handle_futex_death:
//
    rc = compat_fetch_robust_entry(&next_uentry, &next_entry,
    &entry.next, &next_mod);
//
// A pending lock might already be on the list, so
// dont process it twice:
//
    if (entry != pending) {
    let mut uaddr = futex_uaddr(entry, futex_offset);
    if (handle_futex_death(uaddr, curr, cur_mod, HANDLE_DEATH_LIST)) {
    return;
    }
    }
    if (rc) {
    return;
    }
    uentry = next_uentry;
    entry = next_entry;
    cur_mod = next_mod;
//
// Avoid excessively long or circular lists:
//
    if (!--limit) {
    break;
    }
    cond_resched();
    }
    if (pending) {
    let mut uaddr = futex_uaddr(pending, futex_offset);
    handle_futex_death(uaddr, curr, pend_mod, HANDLE_DEATH_PENDING);
    }
    }
#[no_mangle]
unsafe extern "C" fn compat_robust_list_clear_pending(pop: *mut u32 ) -> bool {
    let mut head = current.futex.compat_robust_list;
    if (!put_user(0U, pop)) {
    return true;
    }
// See comment in robust_list_clear_pending().
    if (pop == &head.list_op_pending) {
    current.futex.compat_robust_list = core::ptr::null_mut();
    }
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn compat_robust_list_clear_pending(pop_addr: *mut u32) -> bool { return false; }

//
// This task is holding PI mutexes at exit time => bad.
// Kernel cleans up PI-state, but userspace is likely hosed.
// (Robust-futex cleanup is separate and might save the day for userspace.)
//
#[no_mangle]
unsafe extern "C" fn exit_pi_state_list(curr: *mut task_struct) {
    struct list_head *next, *head = &curr.futex.pi_state_list;
pub static mut pi_state: *mut c_void = core::ptr::null_mut();
pub static mut key: union futex_key = 0;
//
// The mutex mm_struct::futex_hash_lock might be acquired.
//
    might_sleep();
//
// Ensure the hash remains stable (no resize) during the while loop
// below. The hb pointer is acquired under the pi_lock so we can't block
// on the mutex.
//
    WARN_ON!(curr != current);
    guard(private_hash)(current.mm);
//
// We are a ZOMBIE and nobody can enqueue itself on
// pi_state_list anymore, but we have to be careful
// versus waiters unqueueing themselves:
//
    raw_spin_lock_irq(&curr.pi_lock);
    while (!list_empty(head)) {
    next = head.next;
    pi_state = list_entry(next, futex_pi_state, list);
    key = pi_state.key;
    if (1) {
    CLASS(hbr, hbr)(&key);
pub static mut hb: auto = 0;
//
// We can race against put_pi_state() removing itself from the
// list (a waiter going away). put_pi_state() will first
// decrement the reference count and then modify the list, so
// its possible to see the list entry but fail this reference
// acquire.
//
// In that case; drop the locks to let put_pi_state() make
// progress and retry the loop.
//
    if (!refcount_inc_not_zero(&pi_state.refcount)) {
    raw_spin_unlock_irq(&curr.pi_lock);
    cpu_relax();
    raw_spin_lock_irq(&curr.pi_lock);
    continue;
    }
    raw_spin_unlock_irq(&curr.pi_lock);
    spin_lock(&hb.lock);
    raw_spin_lock_irq(&pi_state.pi_mutex.wait_lock);
    raw_spin_lock(&curr.pi_lock);
//
// We dropped the pi-lock, so re-check whether this
// task still owns the PI-state:
//
    if (head.next != next) {
// retain curr->pi_lock for the loop invariant
    raw_spin_unlock(&pi_state.pi_mutex.wait_lock);
    spin_unlock(&hb.lock);
    put_pi_state(pi_state);
    continue;
    }
    WARN_ON!(pi_state.owner != curr);
    WARN_ON!(list_empty(&pi_state.list));
    list_del_init(&pi_state.list);
    pi_state.owner = core::ptr::null_mut();
    raw_spin_unlock(&curr.pi_lock);
    raw_spin_unlock_irq(&pi_state.pi_mutex.wait_lock);
    spin_unlock(&hb.lock);
    }
    rt_mutex_futex_unlock(&pi_state.pi_mutex);
    put_pi_state(pi_state);
    raw_spin_lock_irq(&curr.pi_lock);
    }
    raw_spin_unlock_irq(&curr.pi_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn exit_pi_state_list(curr: *mut task_struct) { }

#[no_mangle]
pub unsafe extern "C" fn futex_robust_list_clear_pending(pop: *mut c_void , flags: c_uint) -> bool {
pub static mut size32bit: bool = false;
    if (!IS_ENABLED!(CONFIG_64BIT) && !size32bit) {
    return false;
    }
    if (IS_ENABLED!(CONFIG_64BIT) && size32bit) {
    return compat_robust_list_clear_pending(pop);
    }
    return robust_list_clear_pending(pop);
    }

#[no_mangle]
pub unsafe extern "C" fn __futex_fixup_robust_unlock(regs: *mut pt_regs, csr: *mut futex_unlock_cs_range) {
//
// arch_futex_robust_unlock_get_pop() returns the list pending op pointer from
// @regs if the try_cmpxchg() succeeded.
//
    let mut pop = arch_futex_robust_unlock_get_pop(regs);
    if (!pop) {
    return;
    }
    futex_robust_list_clear_pending(pop, csr.pop_size32 ? FLAGS_ROBUST_LIST32 : 0);
    }

#[no_mangle]
unsafe extern "C" fn futex_cleanup(tsk: *mut task_struct) {
    if (unlikely(tsk.futex.robust_list)) {
    exit_robust_list(tsk);
    tsk.futex.robust_list = core::ptr::null_mut();
    }

    if (unlikely(tsk.futex.compat_robust_list)) {
    compat_exit_robust_list(tsk);
    tsk.futex.compat_robust_list = core::ptr::null_mut();
    }

    if (unlikely(!list_empty(&tsk.futex.pi_state_list))) {
    exit_pi_state_list(tsk);
    }
    }
//
// futex_exit_recursive - Set the tasks futex state to FUTEX_STATE_DEAD
// @tsk:	task to set the state on
//
// Set the futex exit state of the task lockless. The futex waiter code
// observes that state when a task is exiting and loops until the task has
// actually finished the futex cleanup. The worst case for this is that the
// waiter runs through the wait loop until the state becomes visible.
//
// This is called from the recursive fault handling path in make_task_dead().
//
// This is best effort. Either the futex exit code has run already or
// not. If the OWNER_DIED bit has been set on the futex then the waiter can
// take it over. If not, the problem is pushed back to user space. If the
// futex exit code did not run yet, then an already queued waiter might
// block forever, but there is nothing which can be done about that.
//
#[no_mangle]
pub unsafe extern "C" fn futex_exit_recursive(tsk: *mut task_struct) {
// If the state is FUTEX_STATE_EXITING then futex_exit_mutex is held
    if (tsk.futex.state == FUTEX_STATE_EXITING) {
    __assume_ctx_lock(&tsk.futex.exit_mutex);
    mutex_unlock(&tsk.futex.exit_mutex);
    }
    tsk.futex.state = FUTEX_STATE_DEAD;
    }
#[no_mangle]
unsafe extern "C" fn futex_cleanup_begin(tsk: *mut task_struct) {
//
// Prevent various race issues against a concurrent incoming waiter
// including live locks by forcing the waiter to block on
// tsk->futex.exit_mutex when it observes FUTEX_STATE_EXITING in
// attach_to_pi_owner().
//
    mutex_lock(&tsk.futex.exit_mutex);
//
// Switch the state to FUTEX_STATE_EXITING under tsk->pi_lock.
//
// This ensures that all subsequent checks of tsk->futex_state in
// attach_to_pi_owner() must observe FUTEX_STATE_EXITING with
// tsk->pi_lock held.
//
// It guarantees also that a pi_state which was queued right before
// the state change under tsk->pi_lock by a concurrent waiter must
// be observed in exit_pi_state_list().
//
    raw_spin_lock_irq(&tsk.pi_lock);
    tsk.futex.state = FUTEX_STATE_EXITING;
    raw_spin_unlock_irq(&tsk.pi_lock);
    }
#[no_mangle]
unsafe extern "C" fn futex_cleanup_end(tsk: *mut task_struct) {
    scoped_guard(raw_spinlock_irq, &tsk.pi_lock)
    tsk.futex.state = FUTEX_STATE_DEAD;
//
// Drop the exit protection. This unblocks waiters which observed
// FUTEX_STATE_EXITING to reevaluate the state.
//
    mutex_unlock(&tsk.futex.exit_mutex);
    }
//
// Invoked from mm_exit_exec_release() to cleanup the robust lists and pi state
// of the outgoing task.
//
// exec() makes it interesting for futexes because the TID of the task stays the
// same, but from a futex perspective the task has to be treated like an exiting
// task. This is especially important for the sanity check for private futexes
// in attach_to_pi_owner() which compares the owner's mm with the waiter's mm.
//
// That check would give the wrong answer if futex_cleanup_end() would
// set the state to FUTEX_STATE_OK as long as the task still has the old
// mm.
//
// After the task has switched to the new mm it sets it to
// FUTEX_STATE_OK again in futex_exec_done().
//
#[no_mangle]
pub unsafe extern "C" fn futex_exit_exec_release(tsk: *mut task_struct) {
    futex_cleanup_begin(tsk);
    futex_cleanup(tsk);
    futex_cleanup_end(tsk);
    }
//
// exec() has switched to the new mm. Futex operations are safe again.
//
#[no_mangle]
pub unsafe extern "C" fn futex_exec_done(tsk: *mut task_struct) {
//
// This store does not have to take tsk::futex::exit_mutex because the
// phase where waiters block on it during state FUTEX_STATE_EXITING has
// been finished when futex_cleanup_end() set the state to
// FUTEX_STATE_DEAD.
//
// This transitions back from FUTEX_STATE_DEAD to FUTEX_STATE_OK. The
// ordering guarantee required here is that the previous store to
// tsk::mm in the calling code cannot be reordered against this store.
//
    guard(raw_spinlock_irq)(&tsk.pi_lock);
    tsk.futex.state = FUTEX_STATE_OK;
    }
#[no_mangle]
unsafe extern "C" fn futex_hash_bucket_init(fhb: *mut futex_hash_bucket) {
    atomic_set(&fhb.waiters, 0);
    plist_head_init(&fhb.chain);
    spin_lock_init(&fhb.lock);
    }
pub const FH_CUSTOM: c_uint = 0x01;

//
// futex-ref
//
// Heavily inspired by percpu-rwsem/percpu-refcount; not reusing any of that
// code because it just doesn't fit right.
//
// Dual counter, per-cpu / atomic approach like percpu-refcount, except it
// re-initializes the state automatically, such that the fph swizzle is also a
// transition back to per-cpu.
//
// forward_decl: futex_ref_rcu;
#[no_mangle]
unsafe extern "C" fn __futex_ref_atomic_begin(fph: *mut futex_private_hash) {
    let mut mm = fph.mm;
//
// The counter we're about to switch to must have fully switched;
// otherwise it would be impossible for it to have reported success
// from futex_ref_is_dead().
//
    WARN_ON_ONCE!(atomic_long_read(&mm.futex.phash.atomic) != 0);
//
// Set the atomic to the bias value such that futex_ref_{get,put}()
// will never observe 0. Will be fixed up in __futex_ref_atomic_end()
// when folding in the percpu count.
//
    atomic_long_set(&mm.futex.phash.atomic, LONG_MAX);
    smp_store_release(&fph.state, FR_ATOMIC);
    call_rcu_hurry(&mm.futex.phash.rcu, futex_ref_rcu);
    }
#[no_mangle]
unsafe extern "C" fn __futex_ref_atomic_end(fph: *mut futex_private_hash) {
    let mut mm = fph.mm;
pub static mut count: c_uint = 0;
    let mut ret = 0;
    let mut cpu = 0;
//
// Per __futex_ref_atomic_begin() the state of the fph must be ATOMIC
// and per this RCU callback, everybody must now observe this state and
// use the atomic variable.
//
    WARN_ON_ONCE!(fph.state != FR_ATOMIC);
//
// Therefore the per-cpu counter is now stable, sum and reset.
//
    for_each_possible_cpu(cpu) {
    let mut ptr = per_cpu_ptr(mm.futex.phash.ref, cpu);
    count += *ptr;
// ptr = 0;
    }
//
// Re-init for the next cycle.
//
    this_cpu_inc(*mm.futex.phash.ref); /* 0 . 1 */
//
// Add actual count, subtract bias and initial refcount.
//
// The moment this atomic operation happens, futex_ref_is_dead() can
// become true.
//
    ret = atomic_long_add_return(count - LONG_MAX - 1, &mm.futex.phash.atomic);
    if (!ret) {
    wake_up_var(mm);
    }
    WARN_ON_ONCE!(ret < 0);
    mmput_async(mm);
    }
#[no_mangle]
unsafe extern "C" fn futex_ref_rcu(head: *mut rcu_head) {
    let mut mm = container_of!(head, mm_struct, futex.phash.rcu);
    let mut fph = rcu_dereference_raw(mm.futex.phash.hash);
    if (fph.state == FR_PERCPU) {
//
// Per this extra grace-period, everybody must now observe
// fph as the current fph and no previously observed fph's
// are in-flight.
//
// Notably, nobody will now rely on the atomic
// futex_ref_is_dead() state anymore so we can begin the
// migration of the per-cpu counter into the atomic.
//
    __futex_ref_atomic_begin(fph);
    return;
    }
    __futex_ref_atomic_end(fph);
    }
//
// Drop the initial refcount and transition to atomics.
//
#[no_mangle]
unsafe extern "C" fn futex_ref_drop(fph: *mut futex_private_hash) {
    let mut mm = fph.mm;
//
// Can only transition the current fph;
//
    WARN_ON_ONCE!(rcu_dereference_raw(mm.futex.phash.hash) != fph);
//
// We enqueue at least one RCU callback. Ensure mm stays if the task
// exits before the transition is completed.
//
    mmget(mm);
//
// In order to avoid the following scenario:
//
// futex_hash()			__futex_pivot_hash()
// guard(rcu);		  guard(mm->futex.phash.lock);
// fph = mm->futex.phash.hash;
// rcu_assign_pointer(&mm->futex.phash.hash, new);
// futex_hash_allocate()
// futex_ref_drop()
// fph->state = FR_ATOMIC;
// atomic_set(, BIAS);
//
// futex_private_hash_get(fph); // OOPS
//
// Where an old fph (which is FR_ATOMIC) and should fail on
// inc_not_zero, will succeed because a new transition is started and
// the atomic is bias'ed away from 0.
//
// There must be at least one full grace-period between publishing a
// new fph and trying to replace it.
//
    if (poll_state_synchronize_rcu(mm.futex.phash.batches)) {
//
// There was a grace-period, we can begin now.
//
    __futex_ref_atomic_begin(fph);
    return;
    }
    call_rcu_hurry(&mm.futex.phash.rcu, futex_ref_rcu);
    }
#[no_mangle]
unsafe extern "C" fn futex_ref_get(fph: *mut futex_private_hash) -> bool {
    let mut mm = fph.mm;
    guard(preempt)();
    if (READ_ONCE(fph.state) == FR_PERCPU) {
    __this_cpu_inc(*mm.futex.phash.ref);
    return true;
    }
    return atomic_long_inc_not_zero(&mm.futex.phash.atomic);
    }
#[no_mangle]
unsafe extern "C" fn futex_ref_put(fph: *mut futex_private_hash) -> bool {
    let mut mm = fph.mm;
    guard(preempt)();
    if (READ_ONCE(fph.state) == FR_PERCPU) {
    __this_cpu_dec(*mm.futex.phash.ref);
    return false;
    }
    return atomic_long_dec_and_test(&mm.futex.phash.atomic);
    }
#[no_mangle]
unsafe extern "C" fn futex_ref_is_dead(fph: *mut futex_private_hash) -> bool {
    let mut mm = fph.mm;
    guard(rcu)();
    if (smp_load_acquire(&fph.state) == FR_PERCPU) {
    return false;
    }
    return atomic_long_read(&mm.futex.phash.atomic) == 0;
    }
#[no_mangle]
unsafe extern "C" fn futex_hash_init_mm(fd: *mut futex_mm_data) {
    memset(&fd.phash, 0, sizeof!(fd.phash));
    mutex_init(&fd.phash.lock);
    fd.phash.batches = get_state_synchronize_rcu();
    }
#[no_mangle]
pub unsafe extern "C" fn futex_hash_free(mm: *mut mm_struct) {
pub static mut fph: *mut c_void = core::ptr::null_mut();
    free_percpu(mm.futex.phash.ref);
    kvfree(mm.futex.phash.hash_new);
    fph = rcu_dereference_raw(mm.futex.phash.hash);
    kvfree(fph);
    }
#[no_mangle]
unsafe extern "C" fn futex_pivot_pending(mm: *mut mm_struct) -> bool {
    let mut mmph = &mm.futex.phash;
pub static mut fph: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&mmph.lock);
    if (!mmph.hash_new) {
    return true;
    }
    fph = rcu_dereference_raw(mmph.hash);
    return futex_ref_is_dead(fph);
    }
#[no_mangle]
pub unsafe extern "C" fn futex_hash_less(a: *mut futex_private_hash, b: *mut futex_private_hash) -> bool {
// user provided always wins
    if (!a.custom && b.custom) {
    return true;
    }
    if (a.custom && !b.custom) {
    return false;
    }
// zero-sized hash wins
    if (!b.hash_mask) {
    return true;
    }
    if (!a.hash_mask) {
    return false;
    }
// keep the biggest
    if (a.hash_mask < b.hash_mask) {
    return true;
    }
    if (a.hash_mask > b.hash_mask) {
    return false;
    }
    return false; /* equal */
    }
#[no_mangle]
unsafe extern "C" fn futex_hash_allocate(hash_slots: c_uint, flags: c_uint) -> c_int {
    let mut mm = current.mm;
pub static mut fph: *mut c_void = core::ptr::null_mut();
pub static mut custom: bool = false;
    let mut i = 0;
    if (hash_slots && (hash_slots == 1 || !is_power_of_2(hash_slots))) {
    return -EINVAL;
    }
//
// Once we've disabled the global hash there is no way back.
//
    scoped_guard(rcu) {
    fph = rcu_dereference(mm.futex.phash.hash);
    if (fph && !fph.hash_mask) {
    if (custom) {
    return -EBUSY;
    }
    return 0;
    }
    }
    if (!mm.futex.phash.ref) {
    let mut ref = alloc_percpu(unsigned int);
    if (!ref) {
    return -ENOMEM;
    }
//
// Tasks sharing the mm can run this concurrently, so take the
// initial reference before publishing the counter.
//
    this_cpu_inc(*ref); /* 0 . 1 */
    if (cmpxchg(&mm.futex.phash.ref, core::ptr::null_mut(), ref)) {
    free_percpu(ref);
    }
    }
    fph = kvzalloc_flex(*fph, queues, hash_slots,
    GFP_KERNEL_ACCOUNT | __GFP_NOWARN);
    if (!fph) {
    return -ENOMEM;
    }
    fph.hash_mask = hash_slots ? hash_slots - 1 : 0;
    fph.custom = custom;
    fph.mm = mm;
    for (i = 0; i < hash_slots; i++) {
    futex_hash_bucket_init(&fph.queues[i]);
    }
    if (custom) {
pub static mut __wbq_entry: usize = 0;
pub static mut __wq_head: *mut c_void = core::ptr::null_mut();
//
// Only let prctl() wait / retry; don't unduly delay clone().
//
// label;
    __wq_head = __var_waitqueue(mm);
    init_wait_var_entry(&__wbq_entry, mm, 0);
    __wbq_entry.wq_entry.func = woken_wake_bit_function;
    add_wait_queue(__wq_head, &__wbq_entry.wq_entry);
//
// add_wait_queue()		futex_ref_put()
// MB (this)			MB (implied)
// futex_pivot_pending()	wake_up_var()
// waitqueue_active()
//
// Notably, it must not be possible to see
// !futex_pivot_pending() && !waitqueue_active().
//
    smp_mb();
    while (!futex_pivot_pending(mm) &&
    wait_woken(&__wbq_entry.wq_entry, TASK_UNINTERRUPTIBLE,
    MAX_SCHEDULE_TIMEOUT)) {
// empty */;
    }
    remove_wait_queue(__wq_head, &__wbq_entry.wq_entry);
    }
    scoped_guard(mutex, &mm.futex.phash.lock) {
    struct futex_private_hash *free __free(kvfree) = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    cur = rcu_dereference_protected(mm.futex.phash.hash,
    lockdep_is_held(&mm.futex.phash.lock));
    new = mm.futex.phash.hash_new;
    mm.futex.phash.hash_new = core::ptr::null_mut();
    if (fph) {
    if (cur && !cur.hash_mask) {
//
// If two threads simultaneously request the global
// hash then the first one performs the switch,
// the second one returns here.
//
    free = fph;
    mm.futex.phash.hash_new = new;
    return -EBUSY;
    }
    if (cur && !new) {
//
// If we have an existing hash, but do not yet have
// allocated a replacement hash, drop the initial
// reference on the existing hash.
//
    futex_ref_drop(cur);
    }
    if (new) {
//
// Two updates raced; throw out the lesser one.
//
    if (futex_hash_less(new, fph)) {
    free = new;
    new = fph;
    } else {
    free = fph;
    }
    } else {
    new = fph;
    }
    fph = core::ptr::null_mut();
    }
    if (new) {
//
// Will set mm->futex.phash.new_hash on failure;
// futex_private_hash_get() will try again.
//
    if (!__futex_pivot_hash(mm, new) && custom) {
// goto;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn futex_hash_allocate_default() -> c_int {
    unsigned int threads, buckets, current_buckets = 0;
pub static mut fph: *mut c_void = core::ptr::null_mut();
    if (!current.mm) {
    return 0;
    }
    scoped_guard(rcu) {
    threads = min_t(unsigned int, get_nr_threads(current), num_online_cpus());
    fph = rcu_dereference(current.mm.futex.phash.hash);
    if (fph) {
    if (fph.custom) {
    return 0;
    }
    current_buckets = fph.hash_mask + 1;
    }
    }
//
// The default allocation will remain within
// 16 <= threads * 4 <= global hash size
//
    buckets = roundup_pow_of_two(4 * threads);
    buckets = clamp(buckets, 16, __futex_mask + 1);
    if (current_buckets >= buckets) {
    return 0;
    }
    return futex_hash_allocate(buckets, 0);
    }
#[no_mangle]
unsafe extern "C" fn futex_hash_get_slots() -> c_int {
pub static mut fph: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    fph = rcu_dereference(current.mm.futex.phash.hash);
    if (fph && fph.hash_mask) {
    return fph.hash_mask + 1;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn futex_hash_allocate(hslots: c_uint, flags: c_uint) -> c_int { return -EINVAL; }
#[no_mangle]
pub unsafe extern "C" fn futex_hash_get_slots() -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn futex_hash_init_mm(fd: *mut futex_mm_data) { }

#[no_mangle]
unsafe extern "C" fn futex_invalidate_cs_ranges(fd: *mut futex_mm_data) {
//
// Invalidate start_ip so that the quick check fails for ip >= start_ip
// if VDSO is not mapped or the second slot is not available for compat
// tasks as they use VDSO32 which does not provide the 64-bit pointer
// variant.
//
    for (int i = 0; i < FUTEX_ROBUST_MAX_CS_RANGES; i++) {
    fd.unlock.cs_ranges[i].start_ip = ~0UL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn futex_reset_cs_ranges(fd: *mut futex_mm_data) {
    memset(fd.unlock.cs_ranges, 0, sizeof!(fd.unlock.cs_ranges));
    futex_invalidate_cs_ranges(fd);
    }
#[no_mangle]
unsafe extern "C" fn futex_robust_unlock_init_mm(fd: *mut futex_mm_data) {
// mm_dup() preserves the range, mm_alloc() clears it
    if (!fd.unlock.cs_ranges[0].start_ip) {
    futex_invalidate_cs_ranges(fd);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn futex_robust_unlock_init_mm(fd: *mut futex_mm_data) { }

#[no_mangle]
pub unsafe extern "C" fn futex_mm_init(mm: *mut mm_struct) {
    futex_hash_init_mm(&mm.futex);
    futex_robust_unlock_init_mm(&mm.futex);
    }

#[no_mangle]
pub unsafe extern "C" fn futex_hash_prctl(arg2: c_ulong, arg3: c_ulong, arg4: c_ulong) -> c_int {
pub static mut flags: c_uint = 0;
    let mut ret = 0;
    match (arg2) {
    PR_FUTEX_HASH_SET_SLOTS => {
    if (arg4) {
    return -EINVAL;
    }
    ret = futex_hash_allocate(arg3, flags);
    // break;
    }
    PR_FUTEX_HASH_GET_SLOTS => {
    ret = futex_hash_get_slots();
    // break;
    }
    _ => {
    ret = -EINVAL;
    // break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn futex_init() -> c_int {
    unsigned long hashsize, i;
    let mut order = 0;
    let mut n = 0;
    let mut size = 0;

    hashsize = 16;

    hashsize = 256 * num_possible_cpus();
    hashsize /= num_possible_nodes();
    hashsize = max(4, hashsize);
    hashsize = roundup_pow_of_two(hashsize);

    __futex_mask = hashsize - 1;
    __futex_shift = ilog2(hashsize);
    size = sizeof!(futex_hash_bucket) * hashsize;
    order = get_order(size);
    __futex_queues = kzalloc_objs(*__futex_queues, nr_node_ids);
    kmemleak_not_leak(__futex_queues);
    runtime_const_init(shift, __futex_shift);
    runtime_const_init(mask,  __futex_mask);
    runtime_const_init(ptr,   __futex_queues);
    barrier();
    BUG_ON!(!futex_queues());
    for_each_node(n) {
pub static mut table: *mut c_void = core::ptr::null_mut();
    if (order > MAX_PAGE_ORDER) {
    table = vmalloc_huge_node(size, GFP_KERNEL, n);
    }
    else {
    table = alloc_pages_exact_nid(n, size, GFP_KERNEL);
    }
    BUG_ON!(!table);
    for (i = 0; i < hashsize; i++) {
    futex_hash_bucket_init(&table[i]);
    }
    futex_queues()[n] = table;
    }
    pr_info!("futex hash table entries: %lu (%lu bytes on %d NUMA nodes, total %lu KiB, %s).\n",
    hashsize, size, num_possible_nodes(), size * num_possible_nodes() / 1024,
    order > MAX_PAGE_ORDER ? "vmalloc" : "linear");
    return 0;
    }
    core_initcall!(futex_init);