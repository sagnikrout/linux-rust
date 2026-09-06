//! Automatically rewritten from C to Rust
//! Source: kernel/locking/lockdep.c
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
// kernel/lockdep.c
//
// Runtime locking correctness validator
//
// Started by Ingo Molnar:
//
// Copyright (C) 2006,2007 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright (C) 2007 Red Hat, Inc., Peter Zijlstra
//
// this code maps all the lock dependencies as they occur in a live kernel
// and will warn about the following classes of locking bugs:
//
// - lock inversion scenarios
// - circular lock dependencies
// - hardirq/softirq safe/unsafe locking bugs
//
// Bugs are reported even if the current locking scenario does not cause
// any deadlock at this point.
//
// I.e. if anytime in the past two locks were taken in a different order,
// even if it happened for another task, even if those were different
// locks (but of the same class as this lock), this code will detect it.
//
// Thanks to Arjan van de Ven for coming up with the initial idea of
// mapping lock dependencies runtime.
//
// Macro flag: #define DISABLE_BRANCH_PROFILING

pub static mut prove_locking: int = 1;
    module_param!(prove_locking, int, 0644);

pub const prove_locking: c_int = 0;

pub static mut lock_stat: int = 1;
    module_param!(lock_stat, int, 0644);

pub const lock_stat: c_int = 0;

pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kernel_lockdep_sysctls_init() -> __init int {
    register_sysctl_init("kernel", kern_lockdep_table);
    return 0;
    }
    late_initcall!(kernel_lockdep_sysctls_init);

pub static mut unsigned int: usize = 0;
    EXPORT_PER_CPU_SYMBOL_GPL(lockdep_recursion);
#[no_mangle]
unsafe extern "C" fn lockdep_enabled() -> __always_inline bool {
    if (!debug_locks) {
    return false;
    }
    if (this_cpu_read(lockdep_recursion)) {
    return false;
    }
    if (current.lockdep_recursion) {
    return false;
    }
    return true;
    }
//
// lockdep_lock: protects the lockdep graph, the hashes and the
// class/list/hash allocators.
//
// This is one of the rare exceptions where it's justified
// to use a raw spinlock - we really dont want the spinlock
// code to recurse back into the lockdep code...
//
pub static mut __lock: arch_spinlock_t = 0;
pub static mut __owner: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn lockdep_lock() {
    DEBUG_LOCKS_WARN_ON(!irqs_disabled());
    __this_cpu_inc(lockdep_recursion);
    arch_spin_lock(&__lock);
    __owner = current;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_unlock() {
    DEBUG_LOCKS_WARN_ON(!irqs_disabled());
    if (debug_locks && DEBUG_LOCKS_WARN_ON(__owner != current)) {
    return;
    }
    __owner = core::ptr::null_mut();
    arch_spin_unlock(&__lock);
    __this_cpu_dec(lockdep_recursion);
    }

#[no_mangle]
pub unsafe extern "C" fn lockdep_assert_locked() -> bool {
    return DEBUG_LOCKS_WARN_ON(__owner != current);
    }

pub static mut lockdep_selftest_task_struct: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn graph_lock() -> c_int {
    lockdep_lock();
    lockevent_inc(lockdep_lock);
//
// Make sure that if another CPU detected a bug while
// walking the graph we dont change it (while the other
// CPU is busy printing out stuff with the graph lock
// dropped already)
//
    if (!debug_locks) {
    lockdep_unlock();
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn graph_unlock() {
    lockdep_unlock();
    }
//
// Turn lock debugging off and return with 0 if it was off already,
// and also release the graph lock:
//
#[no_mangle]
pub unsafe extern "C" fn debug_locks_off_graph_unlock() -> c_int {
pub static mut ret: c_int = 0;
    lockdep_unlock();
    return ret;
    }
    let mut nr_list_entries = 0;
    static struct lock_list list_entries[MAX_LOCKDEP_ENTRIES];
pub static mut list_entries_in_use: usize = 0;
//
// All data structures here are protected by the global debug_lock.
//
// nr_lock_classes is the number of elements of lock_classes[] that is
// in use.
//

    static struct hlist_head lock_keys_hash[KEYHASH_SIZE];
    let mut nr_lock_classes = 0;
    let mut nr_zapped_classes = 0;
    let mut nr_dynamic_keys = 0;
    let mut max_lock_class_idx = 0;
    struct lock_class lock_classes[MAX_LOCKDEP_KEYS];
pub static mut lock_classes_in_use: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn hlock_class(hlock: *mut held_lock) -> *mut c_void {
pub static mut class_idx: c_uint = 0;
// Don't re-read hlock->class_idx, can't use READ_ONCE() on bitfield
    barrier();
    if (!test_bit(class_idx, lock_classes_in_use)) {
//
// Someone passed in garbage, we give up.
//
    DEBUG_LOCKS_WARN_ON(1);
    return core::ptr::null_mut();
    }
//
// At this point, if the passed hlock->class_idx is still garbage,
// we just have to live with it
//
    return lock_classes + class_idx;
    }

pub static mut struct lock_class_stats[MAX_LOCKDEP_KEYS]: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn lockstat_clock() -> u64 {
    return local_clock();
    }
#[no_mangle]
unsafe extern "C" fn lock_point(points[]: c_ulong, ip: c_ulong) -> c_int {
    let mut i = 0;
    while (i < LOCKSTAT_POINTS) {
    if (points[i] == 0) {
    points[i] = ip;
    break;
    }
    if (points[i] == ip) {
    break;
    }
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn lock_time_inc(lt: *mut lock_time, time: u64) {
    if (time > lt.max) {
    lt.max = time;
    }
    if (time < lt.min || !lt.nr) {
    lt.min = time;
    }
    lt.total += time;
    lt.nr += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_time_add(src: *mut lock_time, dst: *mut lock_time) {
    if (!src.nr) {
    return;
    }
    if (src.max > dst.max) {
    dst.max = src.max;
    }
    if (src.min < dst.min || !dst.nr) {
    dst.min = src.min;
    }
    dst.total += src.total;
    dst.nr += src.nr;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_stats(class: *mut lock_class, stats: *mut lock_class_stats) {
    let mut cpu = 0;
    let mut i = 0;
    memset(stats, 0, sizeof!(lock_class_stats));
    for_each_possible_cpu(cpu) {
    let mut pcs = &per_cpu(cpu_lock_stats, cpu)[class - lock_classes];
    for (i = 0; i < ARRAY_SIZE!(stats.contention_point); i++) {
    stats.contention_point[i] += pcs.contention_point[i];
    }
    for (i = 0; i < ARRAY_SIZE!(stats.contending_point); i++) {
    stats.contending_point[i] += pcs.contending_point[i];
    }
    lock_time_add(&pcs.read_waittime, &stats.read_waittime);
    lock_time_add(&pcs.write_waittime, &stats.write_waittime);
    lock_time_add(&pcs.read_holdtime, &stats.read_holdtime);
    lock_time_add(&pcs.write_holdtime, &stats.write_holdtime);
    for (i = 0; i < ARRAY_SIZE!(stats.bounces); i++) {
    stats.bounces[i] += pcs.bounces[i];
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn clear_lock_stats(class: *mut lock_class) {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    let mut cpu_stats = &per_cpu(cpu_lock_stats, cpu)[class - lock_classes];
    memset(cpu_stats, 0, sizeof!(lock_class_stats));
    }
    memset(class.contention_point, 0, sizeof!(class.contention_point));
    memset(class.contending_point, 0, sizeof!(class.contending_point));
    }
#[no_mangle]
pub unsafe extern "C" fn get_lock_stats(class: *mut lock_class) -> *mut c_void {
    return &this_cpu_ptr(cpu_lock_stats)[class - lock_classes];
    }
#[no_mangle]
unsafe extern "C" fn lock_release_holdtime(hlock: *mut held_lock) {
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut holdtime = 0;
    if (!lock_stat) {
    return;
    }
    holdtime = lockstat_clock() - hlock.holdtime_stamp;
    stats = get_lock_stats(hlock_class(hlock));
    if (hlock.read) {
    lock_time_inc(&stats.read_holdtime, holdtime);
    }
    else {
    lock_time_inc(&stats.write_holdtime, holdtime);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn lock_release_holdtime(hlock: *mut held_lock) {
    }

//
// We keep a global list of all lock classes. The list is only accessed with
// the lockdep spinlock lock held. free_lock_classes is a list with free
// elements. These elements are linked together by the lock_entry member in
// struct lock_class.
//
pub static mut all_lock_classes: usize = 0;
pub static mut free_lock_classes: usize = 0;
//
// struct pending_free - information about data structures about to be freed
// @zapped: Head of a list with struct lock_class elements.
// @lock_chains_being_freed: Bitmap that indicates which lock_chains[] elements
// are about to be freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_free {
    pub zapped: list_head,
    pub MAX_LOCKDEP_CHAINS): DECLARE_BITMAP(lock_chains_being_freed,,
}

//
// struct delayed_free - data structures used for delayed freeing
//
// A data structure for delayed freeing of data structures that may be
// accessed by RCU readers at the time these were freed.
//
// @rcu_head:  Used to schedule an RCU callback for freeing data structures.
// @index:     Index of @pf to which freed data structures are added.
// @scheduled: Whether or not an RCU callback has been scheduled.
// @pf:        Array with information about data structures about to be freed.
//
    static struct delayed_free {
pub static mut rcu_head: usize = 0;
    let mut index = 0;
    let mut scheduled = 0;
    struct pending_free	pf[2];
    } delayed_free;
//
// The lockdep classes are in a hash-table as well, for fast lookup:
//

    static struct hlist_head classhash_table[CLASSHASH_SIZE];
//
// We put the lock dependency chains into a hash-table as well, to cache
// their existence:
//

    static struct hlist_head chainhash_table[CHAINHASH_SIZE];
//
// the id of held_lock
//
#[no_mangle]
pub unsafe extern "C" fn hlock_id(hlock: *mut held_lock) -> u16 {
    BUILD_BUG_ON!(MAX_LOCKDEP_KEYS_BITS + 2 > 16);
    return (hlock.class_idx | (hlock.read << MAX_LOCKDEP_KEYS_BITS));
    }
#[no_mangle]
pub unsafe extern "C" fn chain_hlock_class_idx(hlock_id: u16) -> __maybe_unused unsigned int {
    return hlock_id & (MAX_LOCKDEP_KEYS - 1);
    }
//
// The hash key of the lock dependency chains is a hash itself too:
// it's a hash of all locks taken up to that lock, including that lock.
// It's a 64-bit hash, because it's important for the keys to be
// unique.
//
#[no_mangle]
pub unsafe extern "C" fn iterate_chain_key(key: u64, idx: u32) -> u64 {
pub static mut k0: u32 = 0;
    __jhash_mix(idx, k0, k1); /* Macro that modifies arguments! */
    return k0 | (u64)k1 << 32;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_init_task(task: *mut task_struct) {
    task.lockdep_depth = 0; /* no locks held yet */
    task.curr_chain_key = INITIAL_CHAIN_KEY;
    task.lockdep_recursion = 0;
    }
#[no_mangle]
unsafe extern "C" fn lockdep_recursion_inc() -> __always_inline void {
    __this_cpu_inc(lockdep_recursion);
    }
#[no_mangle]
unsafe extern "C" fn lockdep_recursion_finish() -> __always_inline void {
    if (WARN_ON_ONCE!(__this_cpu_dec_return(lockdep_recursion))) {
    __this_cpu_write(lockdep_recursion, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_set_selftest_task(task: *mut task_struct) {
    lockdep_selftest_task_struct = task;
    }
//
// Debugging switches:
//
pub const VERBOSE: c_int = 0;
pub const VERY_VERBOSE: c_int = 0;

//
// Quick filtering for interesting events:
//
#[no_mangle]
unsafe extern "C" fn class_filter(class: *mut lock_class) -> c_int {

// Example
    if (class.name_version == 1 &&
    !strcmp(class.name, "lockname")) {
    return 1;
    }
    if (class.name_version == 1 &&
    !strcmp(class.name, "&struct.lockfield")) {
    return 1;
    }

// Filter everything else. 1 would be to allow everything else
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn verbose(class: *mut lock_class) -> c_int {

    return class_filter(class);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_lockdep_off(bug_msg: *const c_char) {
    printk("%s\n", bug_msg);
    printk("turning off the locking correctness validator.\n");

    printk("Please attach the output of /proc/lock_stat to the bug report\n");

    }
    let mut nr_stack_trace_entries = 0;

//
// struct lock_trace - single stack backtrace
// @hash_entry:	Entry in a stack_trace_hash[] list.
// @hash:	jhash() of @entries.
// @nr_entries:	Number of entries in @entries.
// @entries:	Actual stack backtrace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_trace {
    pub hash_entry: hlist_node,
    pub hash: u32,
    pub nr_entries: u32,
    pub long)): unsigned long entries[] __aligned(sizeof!(unsigned,
}

    (sizeof!(lock_trace) / sizeof!(unsigned long))
//
// Stack-trace: sequence of lock_trace structures. Protected by the graph_lock.
//
    static unsigned long stack_trace[MAX_STACK_TRACE_ENTRIES];
    static struct hlist_head stack_trace_hash[STACK_TRACE_HASH_SIZE];
#[no_mangle]
unsafe extern "C" fn traces_identical(t1: *mut lock_trace, t2: *mut lock_trace) -> bool {
    return t1.hash == t2.hash && t1.nr_entries == t2.nr_entries &&
    memcmp(t1.entries, t2.entries,
    t1.nr_entries * sizeof!(t1.entries[0])) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn save_trace() -> *mut c_void {
    let mut trace = core::ptr::null_mut();
    let mut t2 = core::ptr::null_mut();
pub static mut hash_head: *mut c_void = core::ptr::null_mut();
    let mut hash = 0;
    let mut max_entries = 0;
    BUILD_BUG_ON_NOT_POWER_OF_2(STACK_TRACE_HASH_SIZE);
    BUILD_BUG_ON!(LOCK_TRACE_SIZE_IN_LONGS >= MAX_STACK_TRACE_ENTRIES);
    trace = (stack_trace + nr_stack_trace_entries);
    max_entries = MAX_STACK_TRACE_ENTRIES - nr_stack_trace_entries -
    LOCK_TRACE_SIZE_IN_LONGS;
    if (max_entries <= 0) {
    if (!debug_locks_off_graph_unlock()) {
    return core::ptr::null_mut();
    }
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_STACK_TRACE_ENTRIES too low!");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return core::ptr::null_mut();
    }
    trace.nr_entries = stack_trace_save(trace.entries, max_entries, 3);
    hash = jhash(trace.entries, trace.nr_entries *
    sizeof!(trace.entries[0]), 0);
    trace.hash = hash;
    hash_head = stack_trace_hash + (hash & (STACK_TRACE_HASH_SIZE - 1));
    hlist_for_each_entry(t2, hash_head, hash_entry) {
    if (traces_identical(trace, t2)) {
    return t2;
    }
    }
    nr_stack_trace_entries += LOCK_TRACE_SIZE_IN_LONGS + trace.nr_entries;
    hlist_add_head(&trace.hash_entry, hash_head);
    return trace;
    }
// Return the number of stack traces in the stack_trace[] array.
#[no_mangle]
pub unsafe extern "C" fn lockdep_stack_trace_count() -> u64 {
pub static mut trace: *mut c_void = core::ptr::null_mut();
pub static mut c: u64 = 0;
    let mut i = 0;
    while (i < ARRAY_SIZE!(stack_trace_hash)) {
    hlist_for_each_entry(trace, &stack_trace_hash[i], hash_entry) {
    c += 1;
    }
    }
    return c;
    }
// Return the number of stack hash chains that have at least one stack trace.
#[no_mangle]
pub unsafe extern "C" fn lockdep_stack_hash_count() -> u64 {
pub static mut c: u64 = 0;
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE!(stack_trace_hash); i++) {
    if (!hlist_empty(&stack_trace_hash[i]))
    c += 1;
    }
    return c;
    }

    let mut nr_hardirq_chains = 0;
    let mut nr_softirq_chains = 0;
    let mut nr_process_chains = 0;
    let mut max_lockdep_depth = 0;

//
// Various lockdep statistics:
//
pub static mut struct lockdep_stats: usize = 0;

//
// Locking printouts:
//

    [LOCK_USED_IN_##__STATE] = "IN-"__stringify(__STATE)"-W",	
    [LOCK_ENABLED_##__STATE] = __stringify(__STATE)"-ON-W",		
    [LOCK_USED_IN_##__STATE##_READ] = "IN-"__stringify(__STATE)"-R",
    [LOCK_ENABLED_##__STATE##_READ] = __stringify(__STATE)"-ON-R",
    static const char *usage_str[] =
    {

    [LOCK_USED] = "INITIAL USE",
    [LOCK_USED_READ] = "INITIAL READ USE",
abused as string storage for verify_lock_unused()
    [LOCK_USAGE_STATES] = "IN-NMI",
    };

    const char *__get_key_name(const struct lockdep_subclass_key *key, char *str)
    {
    return kallsyms_lookup((unsigned long)key, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), str);
    }
#[no_mangle]
pub unsafe extern "C" fn lock_flag(bit: lock_usage_bit) -> c_ulong {
    return 1UL << bit;
    }
#[no_mangle]
unsafe extern "C" fn get_usage_char(class: *mut lock_class, bit: lock_usage_bit) -> c_char {
//
// The usage character defaults to '.' (i.e., irqs disabled and not in
// irq context), which is the safest usage category.
//
pub static mut c: c_char = '.';
//
// The order of the following usage checks matters, which will
// result in the outcome character as follows:
//
// - '+': irq is enabled and not in irq context
// - '-': in irq context and irq is disabled
// - '?': in irq context and irq is enabled
//
    if (class.usage_mask & lock_flag(bit + LOCK_USAGE_DIR_MASK)) {
    c = '+';
    if (class.usage_mask & lock_flag(bit)) {
    c = '?';
    }
    } else if (class.usage_mask & lock_flag(bit)) {
    c = '-';
    }
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn get_usage_chars(class: *mut lock_class, usage[LOCK_USAGE_CHARS]: c_char) {
pub static mut i: c_int = 0;

    usage[i++] = get_usage_char(class, LOCK_USED_IN_##__STATE);	
    usage[i++] = get_usage_char(class, LOCK_USED_IN_##__STATE##_READ);

    usage[i] = '\0';
    }
#[no_mangle]
unsafe extern "C" fn __print_lock_name(hlock: *mut held_lock, class: *mut lock_class) {
    char str[KSYM_NAME_LEN];
pub static mut name: *mut c_void = core::ptr::null_mut();
    name = class.name;
    if (!name) {
    name = __get_key_name(class.key, str);
    printk("%s", name);
    } else {
    printk("%s", name);
    if (class.name_version > 1) {
    printk("#%d", class.name_version);
    }
    if (class.subclass) {
    printk("/%d", class.subclass);
    }
    if (hlock && class.print_fn) {
    class.print_fn(hlock.instance);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn print_lock_name(hlock: *mut held_lock, class: *mut lock_class) {
    char usage[LOCK_USAGE_CHARS];
    get_usage_chars(class, usage);
    printk(" (");
    __print_lock_name(hlock, class);
    printk("){%s}-{%d:%d}", usage,
    class.wait_type_outer ?: class.wait_type_inner,
    class.wait_type_inner);
    }
#[no_mangle]
unsafe extern "C" fn print_lockdep_cache(lock: *mut lockdep_map) {
pub static mut name: *mut c_void = core::ptr::null_mut();
    char str[KSYM_NAME_LEN];
    name = lock.name;
    if (!name) {
    name = __get_key_name(lock.key.subkeys, str);
    }
    printk("%s", name);
    }
#[no_mangle]
unsafe extern "C" fn print_lock(hlock: *mut held_lock) {
//
// We can be called locklessly through debug_show_all_locks() so be
// extra careful, the hlock might have been released and cleared.
//
// If this indeed happens, lets pretend it does not hurt to continue
// to print the lock unless the hlock class_idx does not point to a
// registered class. The rationale here is: since we don't attempt
// to distinguish whether we are in this situation, if it just
// happened we can't count on class_idx to tell either.
//
    let mut lock = hlock_class(hlock);
    if (!lock) {
    printk("<RELEASED>\n");
    return;
    }
    printk("%px", hlock.instance);
    print_lock_name(hlock, lock);
    printk(", at: %pS\n", hlock.acquire_ip);
    }
#[no_mangle]
unsafe extern "C" fn lockdep_print_held_locks(p: *mut task_struct) {
    int i, depth = READ_ONCE(p.lockdep_depth);
//
// Note that it's always somewhat unreliable to print held locks
// of a task that is running on another CPU, but we cannot guarantee
// the stability of ->held_locks without actually stopping all active
// remote CPUs, which we absolutely do not want to do because it's
// very intrusive and thus slow.
//
// So we do the next best thing here: we print out the held lock
// array on a best-effort basis, without crashing even if the
// fields are being modified on another CPU. Note the careful
// construction of print_lock() so that it never crashes.
//
// We also print out the CPU the task is or was last running on, with
// the message saying 'on CPU...' if the task is running, and
// 'last CPU' if it's not.
//
// Also note that the task_is_running(p) information is fundamentally
// racy: even if the message says the task is 'on CPU', the task may
// have scheduled out already, or if it says 'last CPU', it may just
// have scheduled in on another CPU. But even with these limitations
// it's still useful debuggining information.
//
    printk("locks held by %s/%d: %d, %s CPU#%d%s\n",
    p.comm, task_pid_nr(p), depth,
    task_is_running(p) ? "last" : "on", task_cpu(p),
    depth > 0 ? ":" : "");
    while (i < depth) {
    printk(" #%d: ", i);
    print_lock(p.held_locks + i);
    }
    }
#[no_mangle]
unsafe extern "C" fn print_kernel_ident() {
    printk("%s %.*s %s\n", init_utsname().release,
    (int)strcspn(init_utsname().version, " "),
    init_utsname().version,
    print_tainted());
    }
#[no_mangle]
unsafe extern "C" fn very_verbose(class: *mut lock_class) -> c_int {

    return class_filter(class);

    return 0;
    }
//
// Is this the address of a static object:
//

#[no_mangle]
unsafe extern "C" fn static_obj(obj: *const c_void) -> c_int {
pub static mut addr: c_ulong = 0;
    if (is_kernel_core_data(addr)) {
    return 1;
    }
//
// keys are allowed in the __ro_after_init section.
//
    if (is_kernel_rodata(addr)) {
    return 1;
    }
//
// in initdata section and used during bootup only?
// NOTE: On some platforms the initdata section is
// outside of the _stext ... _end range.
//
    if (system_state < SYSTEM_FREEING_INITMEM &&
    init_section_contains(addr, 1)) {
    return 1;
    }
//
// in-kernel percpu var?
//
    if (is_kernel_percpu_address(addr)) {
    return 1;
    }
//
// module static or percpu var?
//
    return is_module_address(addr) || is_module_percpu_address(addr);
    }

//
// To make lock name printouts unique, we calculate a unique
// class->name_version generation counter. The caller must hold the graph
// lock.
//
#[no_mangle]
unsafe extern "C" fn count_matching_names(new_class: *mut lock_class) -> c_int {
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut count: c_int = 0;
    if (!new_class.name) {
    return 0;
    }
    list_for_each_entry(class, &all_lock_classes, lock_entry) {
    if (new_class.key - new_class.subclass == class.key) {
    return class.name_version;
    }
    if (class.name && !strcmp(class.name, new_class.name)) {
    count = max(count, class.name_version);
    }
    }
    return count + 1;
    }
// used from NMI context -- must be lockless
    static noinstr struct lock_class *
    look_up_lock_class(const struct lockdep_map *lock, unsigned int subclass)
    {
pub static mut key: *mut c_void = core::ptr::null_mut();
pub static mut hash_head: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
    if (unlikely(subclass >= MAX_LOCKDEP_SUBCLASSES)) {
    instrumentation_begin();
    debug_locks_off();
    nbcon_cpu_emergency_enter();
    printk("BUG: looking up invalid subclass: %u\n", subclass);
    printk("turning off the locking correctness validator.\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    instrumentation_end();
    return core::ptr::null_mut();
    }
//
// If it is not initialised then it has never been locked,
// so it won't be present in the hash table.
//
    if (unlikely(!lock.key)) {
    return core::ptr::null_mut();
    }
//
// NOTE: the class-key must be unique. For dynamic locks, a static
// lock_class_key variable is passed in through the mutex_init()
// (or spin_lock_init()) call - which acts as the key. For static
// locks we use the lock object itself as the key.
//
    BUILD_BUG_ON!(sizeof!(lock_class_key) >
    sizeof!(lockdep_map));
    key = lock.key.subkeys + subclass;
    hash_head = classhashentry(key);
//
// We do an RCU walk of the hash, see lockdep_free_key_range().
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return core::ptr::null_mut();
    }
    hlist_for_each_entry_rcu_notrace(class, hash_head, hash_entry) {
    if (class.key == key) {
//
// Huh! same key, different name? Did someone trample
// on some memory? We're most confused.
//
    WARN_ONCE(class.name != lock.name &&
    lock.key != &__lockdep_no_validate__,
    "Looking for class \"%s\" with key %ps, but found a different class \"%s\" with the same key\n",
    lock.name, lock.key, class.name);
    return class;
    }
    }
    return core::ptr::null_mut();
    }
//
// Static locks do not have their class-keys yet - for them the key is
// the lock object itself. If the lock is in the per cpu area, the
// canonical address of the lock (per cpu offset removed) is used.
//
#[no_mangle]
unsafe extern "C" fn assign_lock_key(lock: *mut lockdep_map) -> bool {
    unsigned long can_addr, addr = (unsigned long)lock;

//
// lockdep_free_key_range() assumes that struct lock_class_key
// objects do not overlap. Since we use the address of lock
// objects as class key for static objects, check whether the
// size of lock_class_key objects does not exceed the size of
// the smallest lock object.
//
    BUILD_BUG_ON!(sizeof!(lock_class_key) > sizeof!(raw_spinlock_t));

    if (__is_kernel_percpu_address(addr, &can_addr)) {
    lock.key = can_addr;
    }

    else if (__is_module_percpu_address(addr, &can_addr)) {
    lock.key = can_addr;
    }

    else if (static_obj(lock)) {
    lock.key = lock;
    }
    else {
// Debug-check: all keys must be persistent!
    debug_locks_off();
    nbcon_cpu_emergency_enter();
    pr_err!("INFO: trying to register non-static key.\n");
    pr_err!("The code is fine but needs lockdep annotation, or maybe\n");
    pr_err!("you didn't initialize this object before use?\n");
    pr_err!("turning off the locking correctness validator.\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return false;
    }
    return true;
    }

// Check whether element @e occurs in list @h
#[no_mangle]
unsafe extern "C" fn in_list(e: *mut list_head, h: *mut list_head) -> bool {
pub static mut f: *mut c_void = core::ptr::null_mut();
    list_for_each(f, h) {
    if (e == f) {
    return true;
    }
    }
    return false;
    }
//
// Check whether entry @e occurs in any of the locks_after or locks_before
// lists.
//
#[no_mangle]
unsafe extern "C" fn in_any_class_list(e: *mut list_head) -> bool {
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < ARRAY_SIZE!(lock_classes)) {
    class = &lock_classes[i];
    if (in_list(e, &class.locks_after) ||
    in_list(e, &class.locks_before)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn class_lock_list_valid(c: *mut lock_class, h: *mut list_head) -> bool {
pub static mut e: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(e, h, entry) {
    if (e.links_to != c) {
    printk("class %s: mismatch for lock entry %ld; class %s <> %s",
    c.name ? : "(?)",
    (unsigned long)(e - list_entries),
    e.links_to && e.links_to.name ?
    e.links_to.name : "(?)",
    e.class && e.class.name ? e.class.name :
    "(?)");
    return false;
    }
    }
    return true;
    }

    static u16 chain_hlocks[MAX_LOCKDEP_CHAIN_HLOCKS];

#[no_mangle]
unsafe extern "C" fn check_lock_chain_key(chain: *mut lock_chain) -> bool {

pub static mut chain_key: u64 = 0;
    let mut i = 0;
    for (i = chain.base; i < chain.base + chain.depth; i++) {
    chain_key = iterate_chain_key(chain_key, chain_hlocks[i]);
    }
//
// The 'unsigned long long' casts avoid that a compiler warning
// is reported when building tools/lib/lockdep.
//
    if (chain.chain_key != chain_key) {
    printk("chain %lld: key %#llx <> %#llx\n",
    (unsigned long long)(chain - lock_chains),
    (unsigned long long)chain.chain_key,
    (unsigned long long)chain_key);
    return false;
    }

    return true;
    }
#[no_mangle]
unsafe extern "C" fn in_any_zapped_class_list(class: *mut lock_class) -> bool {
pub static mut pf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < ARRAY_SIZE!(delayed_free.pf)) {
    if (in_list(&class.lock_entry, &pf.zapped)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn __check_data_structures() -> bool {
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut chain: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut e: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Check whether all classes occur in a lock list.
    while (i < ARRAY_SIZE!(lock_classes)) {
    class = &lock_classes[i];
    if (!in_list(&class.lock_entry, &all_lock_classes) &&
    !in_list(&class.lock_entry, &free_lock_classes) &&
    !in_any_zapped_class_list(class)) {
    printk("class %px/%s is not in any class list\n",
    class, class.name ? : "(?)");
    return false;
    }
    }
// Check whether all classes have valid lock lists.
    while (i < ARRAY_SIZE!(lock_classes)) {
    class = &lock_classes[i];
    if (!class_lock_list_valid(class, &class.locks_before)) {
    return false;
    }
    if (!class_lock_list_valid(class, &class.locks_after)) {
    return false;
    }
    }
// Check the chain_key of all lock chains.
    while (i < ARRAY_SIZE!(chainhash_table)) {
    head = chainhash_table + i;
    hlist_for_each_entry_rcu(chain, head, entry) {
    if (!check_lock_chain_key(chain)) {
    return false;
    }
    }
    }
//
// Check whether all list entries that are in use occur in a class
// lock list.
//
    for_each_set_bit(i, list_entries_in_use, ARRAY_SIZE!(list_entries)) {
    e = list_entries + i;
    if (!in_any_class_list(&e.entry)) {
    printk("list entry %d is not in any class list; class %s <> %s\n",
    (unsigned int)(e - list_entries),
    e.class.name ? : "(?)",
    e.links_to.name ? : "(?)");
    return false;
    }
    }
//
// Check whether all list entries that are not in use do not occur in
// a class lock list.
//
    for_each_clear_bit(i, list_entries_in_use, ARRAY_SIZE!(list_entries)) {
    e = list_entries + i;
    if (in_any_class_list(&e.entry)) {
    printk("list entry %d occurs in a class list; class %s <> %s\n",
    (unsigned int)(e - list_entries),
    e.class && e.class.name ? e.class.name :
    "(?)",
    e.links_to && e.links_to.name ?
    e.links_to.name : "(?)");
    return false;
    }
    }
    return true;
    }
pub static mut check_consistency: c_int = 0;
    module_param!(check_consistency, int, 0644);
#[no_mangle]
unsafe extern "C" fn check_data_structures() {
pub static mut once: bool = false;
    if (check_consistency && !once) {
    if (!__check_data_structures()) {
    once = true;
    WARN_ON!(once);
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn check_data_structures() { }

// forward_decl: init_chain_block_buckets;
//
// Initialize the lock_classes[] array elements, the free_lock_classes list
// and also the delayed_free structure.
//
#[no_mangle]
unsafe extern "C" fn init_data_structures_once() {
    static bool  ds_initialized, rcu_head_initialized;
    let mut i = 0;
    if (likely(rcu_head_initialized)) {
    return;
    }
    if (system_state >= SYSTEM_SCHEDULING) {
    init_rcu_head(&delayed_free.rcu_head);
    rcu_head_initialized = true;
    }
    if (ds_initialized) {
    return;
    }
    ds_initialized = true;
    INIT_LIST_HEAD(&delayed_free.pf[0].zapped);
    INIT_LIST_HEAD(&delayed_free.pf[1].zapped);
    while (i < ARRAY_SIZE!(lock_classes)) {
    list_add_tail(&lock_classes[i].lock_entry, &free_lock_classes);
    INIT_LIST_HEAD(&lock_classes[i].locks_after);
    INIT_LIST_HEAD(&lock_classes[i].locks_before);
    }
    init_chain_block_buckets();
    }
#[no_mangle]
pub unsafe extern "C" fn keyhashentry(key: *mut lock_class_key) -> *mut c_void {
pub static mut hash: c_ulong = 0;
    return lock_keys_hash + hash;
    }
// Register a dynamically allocated key.
#[no_mangle]
pub unsafe extern "C" fn lockdep_register_key(key: *mut lock_class_key) {
pub static mut hash_head: *mut c_void = core::ptr::null_mut();
pub static mut k: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    if (WARN_ON_ONCE!(static_obj(key))) {
    return;
    }
    hash_head = keyhashentry(key);
    raw_local_irq_save(flags);
    if (!graph_lock()) {
// goto;
    }
    hlist_for_each_entry_rcu(k, hash_head, hash_entry) {
    if (WARN_ON_ONCE!(k == key)) {
// goto;
    }
    }
    hlist_add_head_rcu(&key.hash_entry, hash_head);
    nr_dynamic_keys += 1;
// label;
    graph_unlock();
// label;
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lockdep_register_key);
// Check whether a key has been registered as a dynamic key.
#[no_mangle]
unsafe extern "C" fn is_dynamic_key(key: *const lock_class_key) -> bool {
pub static mut hash_head: *mut c_void = core::ptr::null_mut();
pub static mut k: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
    if (WARN_ON_ONCE!(static_obj(key))) {
    return false;
    }
//
// If lock debugging is disabled lock_keys_hash[] may contain
// pointers to memory that has already been freed. Avoid triggering
// a use-after-free in that case by returning early.
//
    if (!debug_locks) {
    return true;
    }
    hash_head = keyhashentry(key);
    rcu_read_lock();
    hlist_for_each_entry_rcu(k, hash_head, hash_entry) {
    if (k == key) {
    found = true;
    break;
    }
    }
    rcu_read_unlock();
    return found;
    }
//
// Register a lock's class in the hash-table, if the class is not present
// yet. Otherwise we look it up. We cache the result in the lock object
// itself, so actual lookup of the hash should be once per lock object.
//
#[no_mangle]
pub unsafe extern "C" fn register_lock_class(lock: *mut lockdep_map, subclass: c_uint, force: c_int) -> *mut c_void {
pub static mut key: *mut c_void = core::ptr::null_mut();
pub static mut hash_head: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    DEBUG_LOCKS_WARN_ON(!irqs_disabled());
    class = look_up_lock_class(lock, subclass);
    if (likely(class)) {
// goto;
    }
    if (!lock.key) {
    if (!assign_lock_key(lock)) {
    return core::ptr::null_mut();
    }
    } else if (!static_obj(lock.key) && !is_dynamic_key(lock.key)) {
    return core::ptr::null_mut();
    }
    key = lock.key.subkeys + subclass;
    hash_head = classhashentry(key);
    if (!graph_lock()) {
    return core::ptr::null_mut();
    }
//
// We have to do the hash-walk again, to avoid races
// with another CPU:
//
    hlist_for_each_entry_rcu(class, hash_head, hash_entry) {
    if (class.key == key) {
// goto;
    }
    }
    init_data_structures_once();
// Allocate a new lock class and add it to the hash.
    class = list_first_entry_or_null(&free_lock_classes, typeof(*class),
    lock_entry);
    if (!class) {
    if (!debug_locks_off_graph_unlock()) {
    return core::ptr::null_mut();
    }
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_LOCKDEP_KEYS too low!");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return core::ptr::null_mut();
    }
    nr_lock_classes += 1;
    __set_bit(class - lock_classes, lock_classes_in_use);
    debug_atomic_inc(nr_unused_locks);
    class.key = key;
    class.name = lock.name;
    class.subclass = subclass;
    WARN_ON_ONCE!(!list_empty(&class.locks_before));
    WARN_ON_ONCE!(!list_empty(&class.locks_after));
    class.name_version = count_matching_names(class);
    class.wait_type_inner = lock.wait_type_inner;
    class.wait_type_outer = lock.wait_type_outer;
    class.lock_type = lock.lock_type;
//
// We use RCU's safe list-add method to make
// parallel walking of the hash-list safe:
//
    hlist_add_head_rcu(&class.hash_entry, hash_head);
//
// Remove the class from the free list and add it to the global list
// of classes.
//
    list_move_tail(&class.lock_entry, &all_lock_classes);
    idx = class - lock_classes;
    if (idx > max_lock_class_idx) {
    max_lock_class_idx = idx;
    }
    if (verbose(class)) {
    graph_unlock();
    nbcon_cpu_emergency_enter();
    printk("\nnew class %px: %s", class.key, class.name);
    if (class.name_version > 1) {
    printk("#%d", class.name_version);
    }
    printk("\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    if (!graph_lock()) {
    return core::ptr::null_mut();
    }
    }
// label;
    graph_unlock();
// label;
    if (!subclass || force) {
    lock.class_cache[0] = class;
    }

    else if (subclass < NR_LOCKDEP_CACHING_CLASSES) {
    lock.class_cache[subclass] = class;
    }
//
// Hash collision, did we smoke some? We found a class with a matching
// hash but the subclass -- which is hashed in -- didn't match.
//
    if (DEBUG_LOCKS_WARN_ON(class.subclass != subclass)) {
    return core::ptr::null_mut();
    }
    return class;
    }

//
// Allocate a lockdep entry. (assumes the graph_lock held, returns
// with NULL on failure)
//
#[no_mangle]
pub unsafe extern "C" fn alloc_list_entry() -> *mut c_void {
    let mut idx = find_first_zero_bit(list_entries_in_use,
    ARRAY_SIZE!(list_entries));
    if (idx >= ARRAY_SIZE!(list_entries)) {
    if (!debug_locks_off_graph_unlock()) {
    return core::ptr::null_mut();
    }
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_LOCKDEP_ENTRIES too low!");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return core::ptr::null_mut();
    }
    nr_list_entries += 1;
    __set_bit(idx, list_entries_in_use);
    return list_entries + idx;
    }
//
// Add a new dependency to the head of the list:
//
#[no_mangle]
pub unsafe extern "C" fn add_lock_to_list(this: *mut lock_class, links_to: *mut lock_class, head: *mut list_head, distance: u16, dep: u8, trace: *mut lock_trace) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
//
// Lock not present yet - get a new dependency struct and
// add it to the list:
//
    entry = alloc_list_entry();
    if (!entry) {
    return 0;
    }
    entry.class = this;
    entry.links_to = links_to;
    entry.dep = dep;
    entry.distance = distance;
    entry.trace = trace;
//
// Both allocation and removal are done under the graph lock; but
// iteration is under RCU-sched; see look_up_lock_class() and
// lockdep_free_key_range().
//
    list_add_tail_rcu(&entry.entry, head);
    return 1;
    }
//
// For good efficiency of modular, we use power of 2
//

//
// The circular_queue and helpers are used to implement graph
// breadth-first search (BFS) algorithm, by which we can determine
// whether there is a path from a lock to another. In deadlock checks,
// a path from the next lock to be acquired to a previous held lock
// indicates that adding the <prev> -> <next> lock dependency will
// produce a circle in the graph. Breadth-first search instead of
// depth-first search is used in order to find the shortest (circular)
// path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct circular_queue {
    pub element: [*mut lock_list; MAX_CIRCULAR_QUEUE_SIZE],
    pub rear: unsigned int front,,
}

pub static mut lock_cq: usize = 0;
    let mut max_bfs_queue_depth = 0;
    static unsigned int lockdep_dependency_gen_id;
#[no_mangle]
pub unsafe extern "C" fn __cq_init(cq: *mut circular_queue) {
    cq.front = cq.rear = 0;
    lockdep_dependency_gen_id += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __cq_empty(cq: *mut circular_queue) -> c_int {
    return (cq.front == cq.rear);
    }
#[no_mangle]
pub unsafe extern "C" fn __cq_full(cq: *mut circular_queue) -> c_int {
    return ((cq.rear + 1) & CQ_MASK) == cq.front;
    }
#[no_mangle]
pub unsafe extern "C" fn __cq_enqueue(cq: *mut circular_queue, elem: *mut lock_list) -> c_int {
    if (__cq_full(cq)) {
    return -1;
    }
    cq.element[cq.rear] = elem;
    cq.rear = (cq.rear + 1) & CQ_MASK;
    return 0;
    }
//
// Dequeue an element from the circular_queue, return a lock_list if
// the queue is not empty, or NULL if otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __cq_dequeue(cq: *mut circular_queue) -> *mut lock_list {
    struct lock_list * lock;
    if (__cq_empty(cq)) {
    return core::ptr::null_mut();
    }
    lock = cq.element[cq.front];
    cq.front = (cq.front + 1) & CQ_MASK;
    return lock;
    }
#[no_mangle]
pub unsafe extern "C" fn __cq_get_elem_count(cq: *mut circular_queue) -> c_uint {
    return (cq.rear - cq.front) & CQ_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn mark_lock_accessed(lock: *mut lock_list) {
    lock.class.dep_gen_id = lockdep_dependency_gen_id;
    }
#[no_mangle]
pub unsafe extern "C" fn visit_lock_entry(lock: *mut lock_list, parent: *mut lock_list) {
    lock.parent = parent;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_accessed(lock: *mut lock_list) -> c_ulong {
    return lock.class.dep_gen_id == lockdep_dependency_gen_id;
    }
#[no_mangle]
pub unsafe extern "C" fn get_lock_parent(child: *mut lock_list) -> *mut c_void {
    return child.parent;
    }
#[no_mangle]
pub unsafe extern "C" fn get_lock_depth(child: *mut lock_list) -> c_int {
pub static mut depth: c_int = 0;
pub static mut parent: *mut c_void = core::ptr::null_mut();
    while ((parent = get_lock_parent(child))) {
    child = parent;
    depth += 1;
    }
    return depth;
    }
//
// Return the forward or backward dependency list.
//
// @lock:   the lock_list to get its class's dependency list
// @offset: the offset to struct lock_class to determine whether it is
// locks_after or locks_before
//
#[no_mangle]
pub unsafe extern "C" fn get_dep_list(lock: *mut lock_list, offset: c_int) -> *mut c_void {
    let mut lock_class = lock.class;
    return lock_class + offset;
    }
//
// Return values of a bfs search:
//
// BFS_E* indicates an error
// BFS_R* indicates a result (match or not)
//
// BFS_EINVALIDNODE: Find a invalid node in the graph.
//
// BFS_EQUEUEFULL: The queue is full while doing the bfs.
//
// BFS_RMATCH: Find the matched node in the graph, and put that node into
// *@target_entry.
//
// BFS_RNOMATCH: Haven't found the matched node and keep *@target_entry
// _unchanged_.
//
    enum bfs_result {
    BFS_EINVALIDNODE = -2,
    BFS_EQUEUEFULL = -1,
    BFS_RMATCH = 0,
    BFS_RNOMATCH = 1,
    };
//
// bfs_result < 0 means error
//
#[no_mangle]
pub unsafe extern "C" fn bfs_error(res: bfs_result) -> bool {
    return res < 0;
    }
//
// DEP_*_BIT in lock_list::dep
//
// For dependency @prev -> @next:
//
// SR: @prev is shared reader (->read != 0) and @next is recursive reader
// (->read == 2)
// ER: @prev is exclusive locker (->read == 0) and @next is recursive reader
// SN: @prev is shared reader and @next is non-recursive locker (->read != 2)
// EN: @prev is exclusive locker and @next is non-recursive locker
//
// Note that we define the value of DEP_*_BITs so that:
// bit0 is prev->read == 0
// bit1 is next->read != 2
//

#[no_mangle]
pub unsafe extern "C" fn __calc_dep_bit(prev: *mut held_lock, next: *mut held_lock) -> c_uint {
    return (prev.read == 0) + ((next.read != 2) << 1);
    }
#[no_mangle]
pub unsafe extern "C" fn calc_dep(prev: *mut held_lock, next: *mut held_lock) -> u8 {
    return 1U << __calc_dep_bit(prev, next);
    }
//
// calculate the dep_bit for backwards edges. We care about whether @prev is
// shared and whether @next is recursive.
//
#[no_mangle]
pub unsafe extern "C" fn __calc_dep_bitb(prev: *mut held_lock, next: *mut held_lock) -> c_uint {
    return (next.read != 2) + ((prev.read == 0) << 1);
    }
#[no_mangle]
pub unsafe extern "C" fn calc_depb(prev: *mut held_lock, next: *mut held_lock) -> u8 {
    return 1U << __calc_dep_bitb(prev, next);
    }
//
// Initialize a lock_list entry @lock belonging to @class as the root for a BFS
// search.
//
#[no_mangle]
pub unsafe extern "C" fn __bfs_init_root(lock: *mut lock_list, class: *mut lock_class) {
    lock.class = class;
    lock.parent = core::ptr::null_mut();
    lock.only_xr = 0;
    }
//
// Initialize a lock_list entry @lock based on a lock acquisition @hlock as the
// root for a BFS search.
//
// ->only_xr of the initial lock node is set to @hlock->read == 2, to make sure
// that <prev> -> @hlock and @hlock -> <whatever __bfs() found> is not -(*R)->
// and -(S*)->.
//
#[no_mangle]
pub unsafe extern "C" fn bfs_init_root(lock: *mut lock_list, hlock: *mut held_lock) {
    __bfs_init_root(lock, hlock_class(hlock));
    lock.only_xr = (hlock.read == 2);
    }
//
// Similar to bfs_init_root() but initialize the root for backwards BFS.
//
// ->only_xr of the initial lock node is set to @hlock->read != 0, to make sure
// that <next> -> @hlock and @hlock -> <whatever backwards BFS found> is not
// -(*S)-> and -(R*)-> (reverse order of -(*R)-> and -(S*)->).
//
#[no_mangle]
pub unsafe extern "C" fn bfs_init_rootb(lock: *mut lock_list, hlock: *mut held_lock) {
    __bfs_init_root(lock, hlock_class(hlock));
    lock.only_xr = (hlock.read != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn __bfs_next(lock: *mut lock_list, offset: c_int) -> *mut c_void {
    if (!lock || !lock.parent) {
    return core::ptr::null_mut();
    }
    return list_next_or_null_rcu(get_dep_list(lock.parent, offset),
    &lock.entry, lock_list, entry);
    }
//
// Breadth-First Search to find a strong path in the dependency graph.
//
// @source_entry: the source of the path we are searching for.
// @data: data used for the second parameter of @match function
// @match: match function for the search
// @target_entry: pointer to the target of a matched path
// @offset: the offset to struct lock_class to determine whether it is
// locks_after or locks_before
//
// We may have multiple edges (considering different kinds of dependencies,
// e.g. ER and SN) between two nodes in the dependency graph. But
// only the strong dependency path in the graph is relevant to deadlocks. A
// strong dependency path is a dependency path that doesn't have two adjacent
// dependencies as -(*R)-> -(S*)->, please see:
//
// Documentation/locking/lockdep-design.rst
//
// for more explanation of the definition of strong dependency paths
//
// In __bfs(), we only traverse in the strong dependency path:
//
// In lock_list::only_xr, we record whether the previous dependency only
// has -(*R)-> in the search, and if it does (prev only has -(*R)->), we
// filter out any -(S*)-> in the current dependency and after that, the
// ->only_xr is set according to whether we only have -(*R)-> left.
//
    static enum bfs_result __bfs(lock_list *source_entry,
    void *data,
    bool (*match)(lock_list *entry, void *data),
    bool (*skip)(lock_list *entry, void *data), lock_list **target_entry,
    int offset)
    {
    let mut cq = &lock_cq;
    let mut lock = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut cq_depth = 0;
    let mut first = 0;
    lockdep_assert_locked();
    __cq_init(cq);
    __cq_enqueue(cq, source_entry);
    while ((lock = __bfs_next(lock, offset)) || (lock = __cq_dequeue(cq))) {
    if (!lock.class) {
    return BFS_EINVALIDNODE;
    }
//
// Step 1: check whether we already finish on this one.
//
// If we have visited all the dependencies from this @lock to
// others (iow, if we have visited all lock_list entries in
// @lock->class->locks_{after,before}) we skip, otherwise go
// and visit all the dependencies in the list and mark this
// list accessed.
//
    if (lock_accessed(lock)) {
    continue;
    }
    else {
    mark_lock_accessed(lock);
    }
//
// Step 2: check whether prev dependency and this form a strong
// dependency path.
//
    if (lock.parent) { /* Parent exists, check prev dependency */ {
pub static mut dep: u8 = 0;
    }
pub static mut prev_only_xr: bool = false;
//
// Mask out all -(S*)-> if we only have *R in previous
// step, because -(*R)-> -(S*)-> don't make up a strong
// dependency.
//
    if (prev_only_xr) {
    dep &= ~(DEP_SR_MASK | DEP_SN_MASK);
    }
// If nothing left, we skip
    if (!dep) {
    continue;
    }
// If there are only -(*R)-> left, set that for the next step
    lock.only_xr = !(dep & (DEP_SN_MASK | DEP_EN_MASK));
    }
//
// Step 3: we haven't visited this and there is a strong
// dependency path to this, so check with @match.
// If @skip is provide and returns true, we skip this
// lock (and any path this lock is in).
//
    if (skip && skip(lock, data)) {
    continue;
    }
    if (match(lock, data)) {
// target_entry = lock;
    return BFS_RMATCH;
    }
//
// Step 4: if not match, expand the path by adding the
// forward or backwards dependencies in the search
//
    first = true;
    head = get_dep_list(lock, offset);
    list_for_each_entry_rcu(entry, head, entry) {
    visit_lock_entry(entry, lock);
//
// Note we only enqueue the first of the list into the
// queue, because we can always find a sibling
// dependency from one (see __bfs_next()), as a result
// the space of queue is saved.
//
    if (!first) {
    continue;
    }
    first = false;
    if (__cq_enqueue(cq, entry)) {
    return BFS_EQUEUEFULL;
    }
    cq_depth = __cq_get_elem_count(cq);
    if (max_bfs_queue_depth < cq_depth) {
    max_bfs_queue_depth = cq_depth;
    }
    }
    }
    return BFS_RNOMATCH;
    }
    static inline enum bfs_result
    __bfs_forwards(lock_list *src_entry,
    void *data,
    bool (*match)(lock_list *entry, void *data),
    bool (*skip)(lock_list *entry, void *data), lock_list **target_entry)
    {
    return __bfs(src_entry, data, match, skip, target_entry,
    offsetof(lock_class, locks_after));
    }
    static inline enum bfs_result
    __bfs_backwards(lock_list *src_entry,
    void *data,
    bool (*match)(lock_list *entry, void *data),
    bool (*skip)(lock_list *entry, void *data), lock_list **target_entry)
    {
    return __bfs(src_entry, data, match, skip, target_entry,
    offsetof(lock_class, locks_before));
    }
#[no_mangle]
pub unsafe extern "C" fn print_lock_trace(trace: *mut lock_trace, spaces: c_uint) {
    stack_trace_print(trace.entries, trace.nr_entries, spaces);
    }
//
// Print a dependency chain entry (this is only done when a deadlock
// has been detected):
//
    static noinline void
    print_circular_bug_entry(lock_list *target, int depth)
    {
    if (debug_locks_silent) {
    return;
    }
    printk("\n. #%u", depth);
    print_lock_name(core::ptr::null_mut(), target.class);
    printk(":\n");
    print_lock_trace(target.trace, 6);
    }
#[no_mangle]
pub unsafe extern "C" fn print_circular_lock_scenario(src: *mut held_lock, tgt: *mut held_lock, prt: *mut lock_list) {
    let mut source = hlock_class(src);
    let mut target = hlock_class(tgt);
    let mut parent = prt.class;
pub static mut src_read: c_int = 0;
pub static mut tgt_read: c_int = 0;
//
// A direct locking problem where unsafe_class lock is taken
// directly by safe_class lock, then all we need to show
// is the deadlock scenario, as it is obvious that the
// unsafe lock is taken under the safe lock.
//
// But if there is a chain instead, where the safe lock takes
// an intermediate lock (middle_class) where this lock is
// not the same as the safe lock, then the lock chain is
// used to describe the problem. Otherwise we would need
// to show a different CPU case for each link in the chain
// from the safe_class lock to the unsafe_class lock.
//
    if (parent != source) {
    printk("Chain exists of:\n  ");
    __print_lock_name(src, source);
    printk(" -. ");
    __print_lock_name(core::ptr::null_mut(), parent);
    printk(" -. ");
    __print_lock_name(tgt, target);
    printk("\n\n");
    }
    printk(" Possible unsafe locking scenario:\n\n");
    printk("       CPU0                    CPU1\n");
    printk("       ----                    ----\n");
    if (tgt_read != 0) {
    printk("  rlock(");
    }
    else {
    printk("  lock(");
    }
    __print_lock_name(tgt, target);
    printk(");\n");
    printk("                               lock(");
    __print_lock_name(core::ptr::null_mut(), parent);
    printk(");\n");
    printk("                               lock(");
    __print_lock_name(tgt, target);
    printk(");\n");
    if (src_read != 0) {
    printk("  rlock(");
    }

    else if (src.sync) {
    printk("  sync(");
    }
    else {
    printk("  lock(");
    }
    __print_lock_name(src, source);
    printk(");\n");
    printk("\n *** DEADLOCK ***\n\n");
    }
//
// When a circular dependency is detected, print the
// header first:
//
    static noinline void
    print_circular_bug_header(lock_list *entry, unsigned int depth, held_lock *check_src, held_lock *check_tgt)
    {
    let mut curr = current;
    if (debug_locks_silent) {
    return;
    }
    pr_warn!("\n");
    pr_warn!("======================================================\n");
    pr_warn!("WARNING: possible circular locking dependency detected\n");
    print_kernel_ident();
    pr_warn!("------------------------------------------------------\n");
    pr_warn!("%s/%d is trying to acquire lock:\n",
    curr.comm, task_pid_nr(curr));
    print_lock(check_src);
    pr_warn!("\nbut task is already holding lock:\n");
    print_lock(check_tgt);
    pr_warn!("\nwhich lock already depends on the new lock.\n\n");
    pr_warn!("\nthe existing dependency chain (in reverse order) is:\n");
    print_circular_bug_entry(entry, depth);
    }
//
// We are about to add B -> A into the dependency graph, and in __bfs() a
// strong dependency path A -> .. -> B is found: hlock_class equals
// entry->class.
//
// We will have a deadlock case (conflict) if A -> .. -> B -> A is a strong
// dependency cycle, that means:
//
// Either
//
// a) B -> A is -(E*)->
//
// or
//
// b) A -> .. -> B is -(*N)-> (i.e. A -> .. -(*N)-> B)
//
// as then we don't have -(*R)-> -(S*)-> in the cycle.
//
#[no_mangle]
pub unsafe extern "C" fn hlock_conflict(entry: *mut lock_list, data: *mut c_void) -> bool {
    let mut hlock = data;
    return hlock_class(hlock) == entry.class && /* Found A . .. . B */
    (hlock.read == 0 || /* B . A is -(E*). */
    !entry.only_xr); /* A . .. . B is -(*N). */
    }
    static noinline void print_circular_bug(lock_list *this, lock_list *target, held_lock *check_src, held_lock *check_tgt)
    {
    let mut curr = current;
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut first_parent: *mut c_void = core::ptr::null_mut();
    let mut depth = 0;
    if (!debug_locks_off_graph_unlock() || debug_locks_silent) {
    return;
    }
    this.trace = save_trace();
    if (!this.trace) {
    return;
    }
    depth = get_lock_depth(target);
    nbcon_cpu_emergency_enter();
    print_circular_bug_header(target, depth, check_src, check_tgt);
    parent = get_lock_parent(target);
    first_parent = parent;
    while (parent) {
    print_circular_bug_entry(parent, --depth);
    parent = get_lock_parent(parent);
    }
    printk("\nother info that might help us debug this:\n\n");
    print_circular_lock_scenario(check_src, check_tgt,
    first_parent);
    lockdep_print_held_locks(curr);
    printk("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
#[no_mangle]
unsafe extern "C" fn print_bfs_bug(ret: c_int) -> noinline void {
    if (!debug_locks_off_graph_unlock()) {
    return;
    }
//
// Breadth-first-search failed, graph got corrupted?
//
    if (ret == BFS_EQUEUEFULL) {
    pr_warn!("Increase LOCKDEP_CIRCULAR_QUEUE_BITS to avoid this warning:\n");
    }
    WARN(1, "lockdep bfs error:%d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn noop_count(entry: *mut lock_list, data: *mut c_void) -> bool {
    (*data)++;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn __lockdep_count_forward_deps(this: *mut lock_list) -> c_ulong {
pub static mut count: c_ulong = 0;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
    __bfs_forwards(this, &count, noop_count, core::ptr::null_mut(), &target_entry);
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_count_forward_deps(class: *mut lock_class) -> c_ulong {
    unsigned long ret, flags;
pub static mut this: usize = 0;
    __bfs_init_root(&this, class);
    raw_local_irq_save(flags);
    lockdep_lock();
    ret = __lockdep_count_forward_deps(&this);
    lockdep_unlock();
    raw_local_irq_restore(flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __lockdep_count_backward_deps(this: *mut lock_list) -> c_ulong {
pub static mut count: c_ulong = 0;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
    __bfs_backwards(this, &count, noop_count, core::ptr::null_mut(), &target_entry);
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_count_backward_deps(class: *mut lock_class) -> c_ulong {
    unsigned long ret, flags;
pub static mut this: usize = 0;
    __bfs_init_root(&this, class);
    raw_local_irq_save(flags);
    lockdep_lock();
    ret = __lockdep_count_backward_deps(&this);
    lockdep_unlock();
    raw_local_irq_restore(flags);
    return ret;
    }
//
// Check that the dependency graph starting at <src> can lead to
// <target> or not.
//
    static noinline enum bfs_result
    check_path(held_lock *target, lock_list *src_entry,
    bool (*match)(lock_list *entry, void *data),
    bool (*skip)(lock_list *entry, void *data), lock_list **target_entry)
    {
    enum bfs_result ret;
    ret = __bfs_forwards(src_entry, target, match, skip, target_entry);
    if (unlikely(bfs_error(ret))) {
    print_bfs_bug(ret);
    }
    return ret;
    }
// forward_decl: print_deadlock_bug;
//
// Prove that the dependency graph starting at <src> can not
// lead to <target>. If it can, there is a circle when adding
// <target> -> <src> dependency.
//
// Print an error and return BFS_RMATCH if it does.
//
    static noinline enum bfs_result
    check_noncircular(held_lock *src, held_lock *target, lock_trace **const trace)
    {
    enum bfs_result ret;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
pub static mut src_entry: usize = 0;
    bfs_init_root(&src_entry, src);
    debug_atomic_inc(nr_cyclic_checks);
    ret = check_path(target, &src_entry, hlock_conflict, core::ptr::null_mut(), &target_entry);
    if (unlikely(ret == BFS_RMATCH)) {
    if (!*trace) {
//
// If save_trace fails here, the printing might
// trigger a WARN but because of the !nr_entries it
// should not do bad things.
//
// trace = save_trace();
    }
    if (src.class_idx == target.class_idx) {
    print_deadlock_bug(current, src, target);
    }
    else {
    print_circular_bug(&src_entry, target_entry, src, target);
    }
    }
    return ret;
    }

//
// Forwards and backwards subgraph searching, for the purposes of
// proving that two subgraphs can be connected by a new dependency
// without creating any illegal irq-safe -> irq-unsafe lock dependency.
//
// A irq safe->unsafe deadlock happens with the following conditions:
//
// 1) We have a strong dependency path A -> ... -> B
//
// 2) and we have ENABLED_IRQ usage of B and USED_IN_IRQ usage of A, therefore
// irq can create a new dependency B -> A (consider the case that a holder
// of B gets interrupted by an irq whose handler will try to acquire A).
//
// 3) the dependency circle A -> ... -> B -> A we get from 1) and 2) is a
// strong circle:
//
// For the usage bits of B:
// a) if A -> B is -(*N)->, then B -> A could be any type, so any
// ENABLED_IRQ usage suffices.
// b) if A -> B is -(*R)->, then B -> A must be -(E*)->, so only
// ENABLED_IRQ_*_READ usage suffices.
//
// For the usage bits of A:
// c) if A -> B is -(E*)->, then B -> A could be any type, so any
// USED_IN_IRQ usage suffices.
// d) if A -> B is -(S*)->, then B -> A must be -(*N)->, so only
// USED_IN_IRQ_*_READ usage suffices.
//
// There is a strong dependency path in the dependency graph: A -> B, and now
// we need to decide which usage bit of A should be accumulated to detect
// safe->unsafe bugs.
//
// Note that usage_accumulate() is used in backwards search, so ->only_xr
// stands for whether A -> B only has -(S*)-> (in this case ->only_xr is true).
//
// As above, if only_xr is false, which means A -> B has -(E*)-> dependency
// path, any usage of A should be considered. Otherwise, we should only
// consider _READ usage.
//
#[no_mangle]
pub unsafe extern "C" fn usage_accumulate(entry: *mut lock_list, mask: *mut c_void) -> bool {
    if (!entry.only_xr) {
// mask |= entry->class->usage_mask;
    }
    else /* Mask out _READ usage bits */
// mask |= (entry->class->usage_mask & LOCKF_IRQ);
    return false;
    }
//
// There is a strong dependency path in the dependency graph: A -> B, and now
// we need to decide which usage bit of B conflicts with the usage bits of A,
// i.e. which usage bit of B may introduce safe->unsafe deadlocks.
//
// As above, if only_xr is false, which means A -> B has -(*N)-> dependency
// path, any usage of B should be considered. Otherwise, we should only
// consider _READ usage.
//
#[no_mangle]
pub unsafe extern "C" fn usage_match(entry: *mut lock_list, mask: *mut c_void) -> bool {
    if (!entry.only_xr) {
    return !!(entry.class.usage_mask & *mask);
    }
    else /* Mask out _READ usage bits */
    return !!((entry.class.usage_mask & LOCKF_IRQ) & *mask);
    }
#[no_mangle]
pub unsafe extern "C" fn usage_skip(entry: *mut lock_list, mask: *mut c_void) -> bool {
    if (entry.class.lock_type == LD_LOCK_NORMAL) {
    return false;
    }
//
// Skip local_lock() for irq inversion detection.
//
// For !RT, local_lock() is not a real lock, so it won't carry any
// dependency.
//
// For RT, an irq inversion happens when we have lock A and B, and on
// some CPU we can have:
//
// lock(A);
// <interrupted>
// lock(B);
//
// where lock(B) cannot sleep, and we have a dependency B -> ... -> A.
//
// Now we prove local_lock() cannot exist in that dependency. First we
// have the observation for any lock chain L1 -> ... -> Ln, for any
// 1 <= i <= n, Li.inner_wait_type <= L1.inner_wait_type, otherwise
// wait context check will complain. And since B is not a sleep lock,
// therefore B.inner_wait_type >= 2, and since the inner_wait_type of
// local_lock() is 3, which is greater than 2, therefore there is no
// way the local_lock() exists in the dependency B -> ... -> A.
//
// As a result, we will skip local_lock(), when we search for irq
// inversion bugs.
//
    if (entry.class.lock_type == LD_LOCK_PERCPU &&
    DEBUG_LOCKS_WARN_ON(entry.class.wait_type_inner < LD_WAIT_CONFIG)) {
    return false;
    }
//
// Skip WAIT_OVERRIDE for irq inversion detection -- it's not actually
// a lock and only used to override the wait_type.
//
    return true;
    }
//
// Find a node in the forwards-direction dependency sub-graph starting
// at @root->class that matches @bit.
//
// Return BFS_MATCH if such a node exists in the subgraph, and put that node
// into *@target_entry.
//
    static enum bfs_result
    find_usage_forwards(lock_list *root, unsigned long usage_mask, lock_list **target_entry)
    {
    enum bfs_result result;
    debug_atomic_inc(nr_find_usage_forwards_checks);
    result = __bfs_forwards(root, &usage_mask, usage_match, usage_skip, target_entry);
    return result;
    }
//
// Find a node in the backwards-direction dependency sub-graph starting
// at @root->class that matches @bit.
//
    static enum bfs_result
    find_usage_backwards(lock_list *root, unsigned long usage_mask, lock_list **target_entry)
    {
    enum bfs_result result;
    debug_atomic_inc(nr_find_usage_backwards_checks);
    result = __bfs_backwards(root, &usage_mask, usage_match, usage_skip, target_entry);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn print_lock_class_header(class: *mut lock_class, depth: c_int) {
    let mut bit = 0;
    printk("%*s.", depth, "");
    print_lock_name(core::ptr::null_mut(), class);

    printk(" ops: %lu", debug_class_ops_read(class));

    printk(" {\n");
    while (bit < LOCK_TRACE_STATES) {
    if (class.usage_mask & (1 << bit)) {
pub static mut len: c_int = 0;
    len += printk("%*s   %s", depth, "", usage_str[bit]);
    len += printk(" at:\n");
    print_lock_trace(class.usage_traces[bit], len);
    }
    }
    printk("%*s }\n", depth, "");
    printk("%*s ... key      at: [<%px>] %pS\n",
    depth, "", class.key, class.key);
    }
//
// Dependency path printing:
//
// After BFS we get a lock dependency path (linked via ->parent of lock_list),
// printing out each lock in the dependency path will help on understanding how
// the deadlock could happen. Here are some details about dependency path
// printing:
//
// 1)	A lock_list can be either forwards or backwards for a lock dependency,
// for a lock dependency A -> B, there are two lock_lists:
//
// a)	lock_list in the ->locks_after list of A, whose ->class is B and
// ->links_to is A. In this case, we can say the lock_list is
// "A -> B" (forwards case).
//
// b)	lock_list in the ->locks_before list of B, whose ->class is A
// and ->links_to is B. In this case, we can say the lock_list is
// "B <- A" (bacwards case).
//
// The ->trace of both a) and b) point to the call trace where B was
// acquired with A held.
//
// 2)	A "helper" lock_list is introduced during BFS, this lock_list doesn't
// represent a certain lock dependency, it only provides an initial entry
// for BFS. For example, BFS may introduce a "helper" lock_list whose
// ->class is A, as a result BFS will search all dependencies starting with
// A, e.g. A -> B or A -> C.
//
// The notation of a forwards helper lock_list is like "-> A", which means
// we should search the forwards dependencies starting with "A", e.g A -> B
// or A -> C.
//
// The notation of a bacwards helper lock_list is like "<- B", which means
// we should search the backwards dependencies ending with "B", e.g.
// B <- A or B <- C.
//
// printk the shortest lock dependencies from @root to @leaf in reverse order.
//
// We have a lock dependency path as follow:
//
// @root                                                                 @leaf
// |                                                                     |
// V                                                                     V
// ->parent                                   ->parent
// | lock_list | <--------- | lock_list | ... | lock_list  | <--------- | lock_list |
// |    -> L1  |            | L1 -> L2  | ... |Ln-2 -> Ln-1|            | Ln-1 -> Ln|
//
// , so it's natural that we start from @leaf and print every ->class and
// ->trace until we reach the @root.
//
    static void __used
    print_shortest_lock_dependencies(lock_list *leaf, lock_list *root)
    {
    let mut entry = leaf;
    let mut depth = 0;
// compute depth from generated tree by BFS
    depth = get_lock_depth(leaf);
    do {
    print_lock_class_header(entry.class, depth);
    printk("%*s ... acquired at:\n", depth, "");
    print_lock_trace(entry.trace, 2);
    printk("\n");
    if (depth == 0 && (entry != root)) {
    printk("lockdep:%s bad path found in chain graph\n", __func__);
    break;
    }
    entry = get_lock_parent(entry);
    depth -= 1;
    } while (entry && (depth >= 0));
    }
//
// printk the shortest lock dependencies from @leaf to @root.
//
// We have a lock dependency path (from a backwards search) as follow:
//
// @leaf                                                                 @root
// |                                                                     |
// V                                                                     V
// ->parent                                   ->parent
// | lock_list | ---------> | lock_list | ... | lock_list  | ---------> | lock_list |
// | L2 <- L1  |            | L3 <- L2  | ... | Ln <- Ln-1 |            |    <- Ln  |
//
// , so when we iterate from @leaf to @root, we actually print the lock
// dependency path L1 -> L2 -> .. -> Ln in the non-reverse order.
//
// Another thing to notice here is that ->class of L2 <- L1 is L1, while the
// ->trace of L2 <- L1 is the call trace of L2, in fact we don't have the call
// trace of L1 in the dependency path, which is alright, because most of the
// time we can figure out where L1 is held from the call trace of L2.
//
    static void __used
    print_shortest_lock_dependencies_backwards(lock_list *leaf, lock_list *root)
    {
    let mut entry = leaf;
    let mut trace = core::ptr::null_mut();
    let mut depth = 0;
// compute depth from generated tree by BFS
    depth = get_lock_depth(leaf);
    do {
    print_lock_class_header(entry.class, depth);
    if (trace) {
    printk("%*s ... acquired at:\n", depth, "");
    print_lock_trace(trace, 2);
    printk("\n");
    }
//
// Record the pointer to the trace for the next lock_list
// entry, see the comments for the function.
//
    trace = entry.trace;
    if (depth == 0 && (entry != root)) {
    printk("lockdep:%s bad path found in chain graph\n", __func__);
    break;
    }
    entry = get_lock_parent(entry);
    depth -= 1;
    } while (entry && (depth >= 0));
    }
#[no_mangle]
pub unsafe extern "C" fn print_irq_lock_scenario(safe_entry: *mut lock_list, unsafe_entry: *mut lock_list, prev_class: *mut lock_class, next_class: *mut lock_class) {
    let mut safe_class = safe_entry.class;
    let mut unsafe_class = unsafe_entry.class;
    let mut middle_class = prev_class;
    if (middle_class == safe_class) {
    middle_class = next_class;
    }
//
// A direct locking problem where unsafe_class lock is taken
// directly by safe_class lock, then all we need to show
// is the deadlock scenario, as it is obvious that the
// unsafe lock is taken under the safe lock.
//
// But if there is a chain instead, where the safe lock takes
// an intermediate lock (middle_class) where this lock is
// not the same as the safe lock, then the lock chain is
// used to describe the problem. Otherwise we would need
// to show a different CPU case for each link in the chain
// from the safe_class lock to the unsafe_class lock.
//
    if (middle_class != unsafe_class) {
    printk("Chain exists of:\n  ");
    __print_lock_name(core::ptr::null_mut(), safe_class);
    printk(" -. ");
    __print_lock_name(core::ptr::null_mut(), middle_class);
    printk(" -. ");
    __print_lock_name(core::ptr::null_mut(), unsafe_class);
    printk("\n\n");
    }
    printk(" Possible interrupt unsafe locking scenario:\n\n");
    printk("       CPU0                    CPU1\n");
    printk("       ----                    ----\n");
    printk("  lock(");
    __print_lock_name(core::ptr::null_mut(), unsafe_class);
    printk(");\n");
    printk("                               local_irq_disable();\n");
    printk("                               lock(");
    __print_lock_name(core::ptr::null_mut(), safe_class);
    printk(");\n");
    printk("                               lock(");
    __print_lock_name(core::ptr::null_mut(), middle_class);
    printk(");\n");
    printk("  <Interrupt>\n");
    printk("    lock(");
    __print_lock_name(core::ptr::null_mut(), safe_class);
    printk(");\n");
    printk("\n *** DEADLOCK ***\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn print_bad_irq_dependency(curr: *mut task_struct, prev_root: *mut lock_list, next_root: *mut lock_list, backwards_entry: *mut lock_list, forwards_entry: *mut lock_list, prev: *mut held_lock, next: *mut held_lock, bit1: lock_usage_bit, bit2: lock_usage_bit, irqclass: *mut c_char) {
    if (!debug_locks_off_graph_unlock() || debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=====================================================\n");
    pr_warn!("WARNING: %s-safe . %s-unsafe lock order detected\n",
    irqclass, irqclass);
    print_kernel_ident();
    pr_warn!("-----------------------------------------------------\n");
    pr_warn!("%s/%d [HC%u[%lu]:SC%u[%lu]:HE%u:SE%u] is trying to acquire:\n",
    curr.comm, task_pid_nr(curr),
    lockdep_hardirq_context(), hardirq_count() >> HARDIRQ_SHIFT,
    curr.softirq_context, softirq_count() >> SOFTIRQ_SHIFT,
    lockdep_hardirqs_enabled(),
    curr.softirqs_enabled);
    print_lock(next);
    pr_warn!("\nand this task is already holding:\n");
    print_lock(prev);
    pr_warn!("which would create a new lock dependency:\n");
    print_lock_name(prev, hlock_class(prev));
    pr_cont(" .");
    print_lock_name(next, hlock_class(next));
    pr_cont("\n");
    pr_warn!("\nbut this new dependency connects a %s-irq-safe lock:\n",
    irqclass);
    print_lock_name(core::ptr::null_mut(), backwards_entry.class);
    pr_warn!("\n... which became %s-irq-safe at:\n", irqclass);
    print_lock_trace(backwards_entry.class.usage_traces[bit1], 1);
    pr_warn!("\nto a %s-irq-unsafe lock:\n", irqclass);
    print_lock_name(core::ptr::null_mut(), forwards_entry.class);
    pr_warn!("\n... which became %s-irq-unsafe at:\n", irqclass);
    pr_warn!("...");
    print_lock_trace(forwards_entry.class.usage_traces[bit2], 1);
    pr_warn!("\nother info that might help us debug this:\n\n");
    print_irq_lock_scenario(backwards_entry, forwards_entry,
    hlock_class(prev), hlock_class(next));
    lockdep_print_held_locks(curr);
    pr_warn!("\nthe dependencies between %s-irq-safe lock and the holding lock:\n", irqclass);
    print_shortest_lock_dependencies_backwards(backwards_entry, prev_root);
    pr_warn!("\nthe dependencies between the lock to be acquired");
    pr_warn!(" and %s-irq-unsafe lock:\n", irqclass);
    next_root.trace = save_trace();
    if (!next_root.trace) {
// goto;
    }
    print_shortest_lock_dependencies(forwards_entry, next_root);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
// label;
    nbcon_cpu_emergency_exit();
    }
    static const char *state_names[] = {

    __stringify(__STATE),

    };
    static const char *state_rnames[] = {

    __stringify(__STATE)"-READ",

    };
    static inline const char *state_name(enum lock_usage_bit bit)
    {
    if (bit & LOCK_USAGE_READ_MASK) {
    return state_rnames[bit >> LOCK_USAGE_DIR_MASK];
    }
    else {
    return state_names[bit >> LOCK_USAGE_DIR_MASK];
    }
    }
//
// The bit number is encoded like:
//
// bit0: 0 exclusive, 1 read lock
// bit1: 0 used in irq, 1 irq enabled
// bit2-n: state
//
#[no_mangle]
unsafe extern "C" fn exclusive_bit(new_bit: c_int) -> c_int {
pub static mut state: c_int = 0;
pub static mut dir: c_int = 0;
//
// keep state, bit flip the direction and strip read.
//
    return state | (dir ^ LOCK_USAGE_DIR_MASK);
    }
//
// Observe that when given a bitmask where each bitnr is encoded as above, a
// right shift of the mask transforms the individual bitnrs as -1 and
// conversely, a left shift transforms into +1 for the individual bitnrs.
//
// So for all bits whose number have LOCK_ENABLED_* set (bitnr1 == 1), we can
// create the mask with those bit numbers using LOCK_USED_IN_* (bitnr1 == 0)
// instead by subtracting the bit number by 2, or shifting the mask right by 2.
//
// Similarly, bitnr1 == 0 becomes bitnr1 == 1 by adding 2, or shifting left 2.
//
// So split the mask (note that LOCKF_ENABLED_IRQ_ALL|LOCKF_USED_IN_IRQ_ALL is
// all bits set) and recompose with bitnr1 flipped.
//
#[no_mangle]
unsafe extern "C" fn invert_dir_mask(mask: c_ulong) -> c_ulong {
pub static mut excl: c_ulong = 0;
// Invert dir
    excl |= (mask & LOCKF_ENABLED_IRQ_ALL) >> LOCK_USAGE_DIR_MASK;
    excl |= (mask & LOCKF_USED_IN_IRQ_ALL) << LOCK_USAGE_DIR_MASK;
    return excl;
    }
//
// Note that a LOCK_ENABLED_IRQ_*_READ usage and a LOCK_USED_IN_IRQ_*_READ
// usage may cause deadlock too, for example:
//
// P1				P2
// <irq disabled>
// write_lock(l1);		<irq enabled>
// read_lock(l2);
// write_lock(l2);
// <in irq>
// read_lock(l1);
//
// , in above case, l1 will be marked as LOCK_USED_IN_IRQ_HARDIRQ_READ and l2
// will marked as LOCK_ENABLE_IRQ_HARDIRQ_READ, and this is a possible
// deadlock.
//
// In fact, all of the following cases may cause deadlocks:
//
// LOCK_USED_IN_IRQ_* -> LOCK_ENABLED_IRQ_
// LOCK_USED_IN_IRQ_*_READ -> LOCK_ENABLED_IRQ_
// LOCK_USED_IN_IRQ_* -> LOCK_ENABLED_IRQ_*_READ
// LOCK_USED_IN_IRQ_*_READ -> LOCK_ENABLED_IRQ_*_READ
//
// As a result, to calculate the "exclusive mask", first we invert the
// direction (USED_IN/ENABLED) of the original mask, and 1) for all bits with
// bitnr0 set (LOCK_*_READ), add those with bitnr0 cleared (LOCK_*). 2) for all
// bits with bitnr0 cleared (LOCK_*_READ), add those with bitnr0 set (LOCK_*).
//
#[no_mangle]
unsafe extern "C" fn exclusive_mask(mask: c_ulong) -> c_ulong {
pub static mut excl: c_ulong = 0;
    excl |= (excl & LOCKF_IRQ_READ) >> LOCK_USAGE_READ_MASK;
    excl |= (excl & LOCKF_IRQ) << LOCK_USAGE_READ_MASK;
    return excl;
    }
//
// Retrieve the _possible_ original mask to which @mask is
// exclusive. Ie: this is the opposite of exclusive_mask().
// Note that 2 possible original bits can match an exclusive
// bit: one has LOCK_USAGE_READ_MASK set, the other has it
// cleared. So both are returned for each exclusive bit.
//
#[no_mangle]
unsafe extern "C" fn original_mask(mask: c_ulong) -> c_ulong {
pub static mut excl: c_ulong = 0;
// Include read in existing usages
    excl |= (excl & LOCKF_IRQ_READ) >> LOCK_USAGE_READ_MASK;
    excl |= (excl & LOCKF_IRQ) << LOCK_USAGE_READ_MASK;
    return excl;
    }
//
// Find the first pair of bit match between an original
// usage mask and an exclusive usage mask.
//
#[no_mangle]
pub unsafe extern "C" fn find_exclusive_match(mask: c_ulong, excl_mask: c_ulong, bitp: *mut lock_usage_bit, excl_bitp: *mut lock_usage_bit) -> c_int {
    let mut bit = 0;
    let mut excl = 0;
    let mut excl_read = 0;
    for_each_set_bit(bit, &mask, LOCK_USED) {
//
// exclusive_bit() strips the read bit, however,
// LOCK_ENABLED_IRQ_*_READ may cause deadlocks too, so we need
// to search excl | LOCK_USAGE_READ_MASK as well.
//
    excl = exclusive_bit(bit);
    excl_read = excl | LOCK_USAGE_READ_MASK;
    if (excl_mask & lock_flag(excl)) {
// bitp = bit;
// excl_bitp = excl;
    return 0;
    } else if (excl_mask & lock_flag(excl_read)) {
// bitp = bit;
// excl_bitp = excl_read;
    return 0;
    }
    }
    return -1;
    }
//
// Prove that the new dependency does not connect a hardirq-safe(-read)
// lock with a hardirq-unsafe lock - to achieve this we search
// the backwards-subgraph starting at <prev>, and the
// forwards-subgraph starting at <next>:
//
#[no_mangle]
pub unsafe extern "C" fn check_irq_usage(curr: *mut task_struct, prev: *mut held_lock, next: *mut held_lock) -> c_int {
pub static mut usage_mask: c_ulong = 0;
pub static mut forward_bit: lock_usage_bit = 0;
pub static mut target_entry1: *mut c_void = core::ptr::null_mut();
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
    struct lock_list this, that;
    enum bfs_result ret;
//
// Step 1: gather all hard/soft IRQs usages backward in an
// accumulated usage mask.
//
    bfs_init_rootb(&this, prev);
    ret = __bfs_backwards(&this, &usage_mask, usage_accumulate, usage_skip, core::ptr::null_mut());
    if (bfs_error(ret)) {
    print_bfs_bug(ret);
    return 0;
    }
    usage_mask &= LOCKF_USED_IN_IRQ_ALL;
    if (!usage_mask) {
    return 1;
    }
//
// Step 2: find exclusive uses forward that match the previous
// backward accumulated mask.
//
    forward_mask = exclusive_mask(usage_mask);
    bfs_init_root(&that, next);
    ret = find_usage_forwards(&that, forward_mask, &target_entry1);
    if (bfs_error(ret)) {
    print_bfs_bug(ret);
    return 0;
    }
    if (ret == BFS_RNOMATCH) {
    return 1;
    }
//
// Step 3: we found a bad match! Now retrieve a lock from the backward
// list whose usage mask matches the exclusive usage mask from the
// lock found on the forward list.
//
// Note, we should only keep the LOCKF_ENABLED_IRQ_ALL bits, considering
// the follow case:
//
// When trying to add A -> B to the graph, we find that there is a
// hardirq-safe L, that L -> ... -> A, and another hardirq-unsafe M,
// that B -> ... -> M. However M is **softirq-safe**, if we use exact
// invert bits of M's usage_mask, we will find another lock N that is
// **softirq-unsafe** and N -> ... -> A, however N -> .. -> M will not
// cause a inversion deadlock.
//
    backward_mask = original_mask(target_entry1.class.usage_mask & LOCKF_ENABLED_IRQ_ALL);
    ret = find_usage_backwards(&this, backward_mask, &target_entry);
    if (bfs_error(ret)) {
    print_bfs_bug(ret);
    return 0;
    }
    if (DEBUG_LOCKS_WARN_ON(ret == BFS_RNOMATCH)) {
    return 1;
    }
//
// Step 4: narrow down to a pair of incompatible usage bits
// and report it.
//
    ret = find_exclusive_match(target_entry.class.usage_mask,
    target_entry1.class.usage_mask,
    &backward_bit, &forward_bit);
    if (DEBUG_LOCKS_WARN_ON(ret == -1)) {
    return 1;
    }
    print_bad_irq_dependency(curr, &this, &that,
    target_entry, target_entry1,
    prev, next,
    backward_bit, forward_bit,
    state_name(backward_bit));
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: check_irq_usage
pub unsafe extern "C" fn check_irq_usage_dup(curr: *mut task_struct, prev: *mut held_lock, next: *mut held_lock) -> c_int {
    return 1;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: usage_skip
pub unsafe extern "C" fn usage_skip_dup(entry: *mut lock_list, mask: *mut c_void) -> bool {
    return false;
    }

//
// We are about to add A -> B into the dependency graph, and in __bfs() a
// strong dependency path A -> .. -> B is found: hlock_class equals
// entry->class.
//
// If A -> .. -> B can replace A -> B in any __bfs() search (means the former
// is _stronger_ than or equal to the latter), we consider A -> B as redundant.
// For example if A -> .. -> B is -(EN)-> (i.e. A -(E*)-> .. -(*N)-> B), and A
// -> B is -(ER)-> or -(EN)->, then we don't need to add A -> B into the
// dependency graph, as any strong path ..-> A -> B ->.. we can get with
// having dependency A -> B, we could already get a equivalent path ..-> A ->
// .. -> B -> .. with A -> .. -> B. Therefore A -> B is redundant.
//
// We need to make sure both the start and the end of A -> .. -> B is not
// weaker than A -> B. For the start part, please see the comment in
// check_redundant(). For the end part, we need:
//
// Either
//
// a) A -> B is -(*R)-> (everything is not weaker than that)
//
// or
//
// b) A -> .. -> B is -(*N)-> (nothing is stronger than this)
//
#[no_mangle]
pub unsafe extern "C" fn hlock_equal(entry: *mut lock_list, data: *mut c_void) -> bool {
    let mut hlock = data;
    return hlock_class(hlock) == entry.class && /* Found A . .. . B */
    (hlock.read == 2 ||  /* A . B is -(*R). */
    !entry.only_xr); /* A . .. . B is -(*N). */
    }
//
// Check that the dependency graph starting at <src> can lead to
// <target> or not. If it can, <src> -> <target> dependency is already
// in the graph.
//
// Return BFS_RMATCH if it does, or BFS_RNOMATCH if it does not, return BFS_E* if
// any error appears in the bfs search.
//
    static noinline enum bfs_result
    check_redundant(held_lock *src, held_lock *target)
    {
    enum bfs_result ret;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
pub static mut src_entry: usize = 0;
    bfs_init_root(&src_entry, src);
//
// Special setup for check_redundant().
//
// To report redundant, we need to find a strong dependency path that
// is equal to or stronger than <src> -> <target>. So if <src> is E,
// we need to let __bfs() only search for a path starting at a -(E*)->,
// we achieve this by setting the initial node's ->only_xr to true in
// that case. And if <prev> is S, we set initial ->only_xr to false
// because both -(S*)-> (equal) and -(E*)-> (stronger) are redundant.
//
    src_entry.only_xr = src.read == 0;
    debug_atomic_inc(nr_redundant_checks);
//
// Note: we skip local_lock() for redundant check, because as the
// comment in usage_skip(), A -> local_lock() -> B and A -> B are not
// the same.
//
    ret = check_path(target, &src_entry, hlock_equal, usage_skip, &target_entry);
    if (ret == BFS_RMATCH) {
    debug_atomic_inc(nr_redundant);
    }
    return ret;
    }

    static inline enum bfs_result
    check_redundant(held_lock *src, held_lock *target)
    {
    return BFS_RNOMATCH;
    }

#[no_mangle]
unsafe extern "C" fn inc_chains(irq_context: c_int) {
    if (irq_context & LOCK_CHAIN_HARDIRQ_CONTEXT) {
    nr_hardirq_chains += 1;
    }

    else if (irq_context & LOCK_CHAIN_SOFTIRQ_CONTEXT) {
    nr_softirq_chains += 1;
    }
    else {
    nr_process_chains += 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn dec_chains(irq_context: c_int) {
    if (irq_context & LOCK_CHAIN_HARDIRQ_CONTEXT) {
    nr_hardirq_chains -= 1;
    }

    else if (irq_context & LOCK_CHAIN_SOFTIRQ_CONTEXT) {
    nr_softirq_chains -= 1;
    }
    else {
    nr_process_chains -= 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_deadlock_scenario(nxt: *mut held_lock, prv: *mut held_lock) {
    let mut next = hlock_class(nxt);
    let mut prev = hlock_class(prv);
    printk(" Possible unsafe locking scenario:\n\n");
    printk("       CPU0\n");
    printk("       ----\n");
    printk("  lock(");
    __print_lock_name(prv, prev);
    printk(");\n");
    printk("  lock(");
    __print_lock_name(nxt, next);
    printk(");\n");
    printk("\n *** DEADLOCK ***\n\n");
    printk(" May be due to missing lock nesting notation\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn print_deadlock_bug(curr: *mut task_struct, prev: *mut held_lock, next: *mut held_lock) {
    let mut class = hlock_class(prev);
    if (!debug_locks_off_graph_unlock() || debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("============================================\n");
    pr_warn!("WARNING: possible recursive locking detected\n");
    print_kernel_ident();
    pr_warn!("--------------------------------------------\n");
    pr_warn!("%s/%d is trying to acquire lock:\n",
    curr.comm, task_pid_nr(curr));
    print_lock(next);
    pr_warn!("\nbut task is already holding lock:\n");
    print_lock(prev);
    if (class.cmp_fn) {
    pr_warn!("and the lock comparison function returns %i:\n",
    class.cmp_fn(prev.instance, next.instance));
    }
    pr_warn!("\nother info that might help us debug this:\n");
    print_deadlock_scenario(next, prev);
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
//
// Check whether we are holding such a class already.
//
// (Note that this has to be done separately, because the graph cannot
// detect such classes of deadlocks.)
//
// Returns: 0 on deadlock detected, 1 on OK, 2 if another lock with the same
// lock class is held but nest_lock is also held, i.e. we rely on the
// nest_lock to avoid the deadlock.
//
#[no_mangle]
pub unsafe extern "C" fn check_deadlock(curr: *mut task_struct, next: *mut held_lock) -> c_int {
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut prev: *mut c_void = core::ptr::null_mut();
    let mut nest = core::ptr::null_mut();
    let mut i = 0;
    while (i < curr.lockdep_depth) {
    prev = curr.held_locks + i;
    if (prev.instance == next.nest_lock) {
    nest = prev;
    }
    if (hlock_class(prev) != hlock_class(next)) {
    continue;
    }
//
// Allow read-after-read recursion of the same
// lock class (i.e. read_lock(lock)+read_lock(lock)):
//
    if ((next.read == 2) && prev.read) {
    continue;
    }
    class = hlock_class(prev);
    if (class.cmp_fn &&
    class.cmp_fn(prev.instance, next.instance) < 0) {
    continue;
    }
//
// We're holding the nest_lock, which serializes this lock's
// nesting behaviour.
//
    if (nest) {
    return 2;
    }
    print_deadlock_bug(curr, prev, next);
    return 0;
    }
    return 1;
    }
//
// There was a chain-cache miss, and we are about to add a new dependency
// to a previous lock. We validate the following rules:
//
// - would the adding of the <prev> -> <next> dependency create a
// circular dependency in the graph? [== circular deadlock]
//
// - does the new prev->next dependency connect any hardirq-safe lock
// (in the full backwards-subgraph starting at <prev>) with any
// hardirq-unsafe lock (in the full forwards-subgraph starting at
// <next>)? [== illegal lock inversion with hardirq contexts]
//
// - does the new prev->next dependency connect any softirq-safe lock
// (in the full backwards-subgraph starting at <prev>) with any
// softirq-unsafe lock (in the full forwards-subgraph starting at
// <next>)? [== illegal lock inversion with softirq contexts]
//
// any of these scenarios could lead to a deadlock.
//
// Then if all the validations pass, we add the forwards and backwards
// dependency.
//
#[no_mangle]
pub unsafe extern "C" fn check_prev_add(curr: *mut task_struct, prev: *mut held_lock, next: *mut held_lock, distance: u16, trace: *mut *mut lock_trace) -> c_int {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    enum bfs_result ret;
    if (!hlock_class(prev).key || !hlock_class(next).key) {
//
// The warning statements below may trigger a use-after-free
// of the class name. It is better to trigger a use-after free
// and to have the class name most of the time instead of not
// having the class name available.
//
    WARN_ONCE(!debug_locks_silent && !hlock_class(prev).key,
    "Detected use-after-free of lock class %px/%s\n",
    hlock_class(prev),
    hlock_class(prev).name);
    WARN_ONCE(!debug_locks_silent && !hlock_class(next).key,
    "Detected use-after-free of lock class %px/%s\n",
    hlock_class(next),
    hlock_class(next).name);
    return 2;
    }
    if (prev.class_idx == next.class_idx) {
    let mut class = hlock_class(prev);
    if (class.cmp_fn &&
    class.cmp_fn(prev.instance, next.instance) < 0) {
    return 2;
    }
    }
//
// Prove that the new <prev> -> <next> dependency would not
// create a circular dependency in the graph. (We do this by
// a breadth-first search into the graph starting at <next>,
and check whether we can reach <prev>.)
//
// The search is limited by the size of the circular queue (i.e.,
// MAX_CIRCULAR_QUEUE_SIZE) which keeps track of a breadth of nodes
// in the graph whose neighbours are to be checked.
//
    ret = check_noncircular(next, prev, trace);
    if (unlikely(bfs_error(ret) || ret == BFS_RMATCH)) {
    return 0;
    }
    if (!check_irq_usage(curr, prev, next)) {
    return 0;
    }
//
// Is the <prev> -> <next> dependency already present?
//
// (this may occur even though this is a new chain: consider
// e.g. the L1 -> L2 -> L3 -> L4 and the L5 -> L1 -> L2 -> L3
// chains - the second one will be new, but L1 already has
// L2 added to its dependency list, due to the first chain.)
//
    list_for_each_entry(entry, &hlock_class(prev).locks_after, entry) {
    if (entry.class == hlock_class(next)) {
    if (distance == 1) {
    entry.distance = 1;
    }
    entry.dep |= calc_dep(prev, next);
//
// Also, update the reverse dependency in @next's
// ->locks_before list.
//
// Here we reuse @entry as the cursor, which is fine
// because we won't go to the next iteration of the
// outer loop:
//
// For normal cases, we return in the inner loop.
//
// If we fail to return, we have inconsistency, i.e.
// <prev>::locks_after contains <next> while
// <next>::locks_before doesn't contain <prev>. In
// that case, we return after the inner and indicate
// something is wrong.
//
    list_for_each_entry(entry, &hlock_class(next).locks_before, entry) {
    if (entry.class == hlock_class(prev)) {
    if (distance == 1) {
    entry.distance = 1;
    }
    entry.dep |= calc_depb(prev, next);
    return 1;
    }
    }
// <prev> is not found in <next>::locks_before
    return 0;
    }
    }
//
// Is the <prev> -> <next> link redundant?
//
    ret = check_redundant(prev, next);
    if (bfs_error(ret)) {
    return 0;
    }

    else if (ret == BFS_RMATCH) {
    return 2;
    }
    if (!*trace) {
// trace = save_trace();
    if (!*trace) {
    return 0;
    }
    }
//
// Ok, all validations passed, add the new lock
// to the previous lock's dependency list:
//
    ret = add_lock_to_list(hlock_class(next), hlock_class(prev),
    &hlock_class(prev).locks_after, distance,
    calc_dep(prev, next), *trace);
    if (!ret) {
    return 0;
    }
    ret = add_lock_to_list(hlock_class(prev), hlock_class(next),
    &hlock_class(next).locks_before, distance,
    calc_depb(prev, next), *trace);
    if (!ret) {
    return 0;
    }
    return 2;
    }
//
// Add the dependency to all directly-previous locks that are 'relevant'.
// The ones that are relevant are (in increasing distance from curr):
// all consecutive trylock entries and the final non-trylock entry - or
// the end of this context's lock-chain - whichever comes first.
//
#[no_mangle]
pub unsafe extern "C" fn check_prevs_add(curr: *mut task_struct, next: *mut held_lock) -> c_int {
    let mut trace = core::ptr::null_mut();
pub static mut depth: c_int = 0;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
//
// Debugging checks.
//
// Depth must not be zero for a non-head lock:
//
    if (!depth) {
// goto;
    }
//
// At least two relevant locks must exist for this
// to be a head:
//
    if (curr.held_locks[depth].irq_context !=
    curr.held_locks[depth-1].irq_context) {
// goto;
    }
    for (;;) {
pub static mut distance: u16 = 0;
    hlock = curr.held_locks + depth - 1;
    if (hlock.check) {
pub static mut ret: c_int = 0;
    if (!ret) {
    return 0;
    }
//
// Stop after the first non-trylock entry,
// as non-trylock entries have added their
// own direct dependencies already, so this
// lock is connected to them indirectly:
//
    if (!hlock.trylock) {
    break;
    }
    }
    depth -= 1;
//
// End of lock-stack?
//
    if (!depth) {
    break;
    }
//
// Stop the search if we cross into another context:
//
    if (curr.held_locks[depth].irq_context !=
    curr.held_locks[depth-1].irq_context) {
    break;
    }
    }
    return 1;
// label;
    if (!debug_locks_off_graph_unlock()) {
    return 0;
    }
//
// Clearly we all shouldn't be here, but since we made it we
// can reliable say we messed up our state. See the above two
// gotos for reasons why we could possibly end up here.
//
    WARN_ON!(1);
    return 0;
    }
    struct lock_chain lock_chains[MAX_LOCKDEP_CHAINS];
pub static mut lock_chains_in_use: usize = 0;
    static u16 chain_hlocks[MAX_LOCKDEP_CHAIN_HLOCKS];
    let mut nr_zapped_lock_chains = 0;
    let mut nr_free_chain_hlocks = 0;	/* Free chain_hlocks in buckets */
    let mut nr_lost_chain_hlocks = 0;	/* Lost chain_hlocks */
    let mut nr_large_chain_blocks = 0;	/* size > MAX_CHAIN_BUCKETS */
//
// The first 2 chain_hlocks entries in the chain block in the bucket
// list contains the following meta data:
//
// entry[0]:
// Bit    15 - always set to 1 (it is not a class index)
// Bits 0-14 - upper 15 bits of the next block index
// entry[1]    - lower 16 bits of next block index
//
// A next block index of all 1 bits means it is the end of the list.
//
// On the unsized bucket (bucket-0), the 3rd and 4th entries contain
// the chain block size:
//
// entry[2] - upper 16 bits of the chain block size
// entry[3] - lower 16 bits of the chain block size
//
pub const MAX_CHAIN_BUCKETS: c_int = 16;

pub const CHAIN_BLK_LIST_END: c_uint = 0xFFFFU;
    static int chain_block_buckets[MAX_CHAIN_BUCKETS];
#[no_mangle]
pub unsafe extern "C" fn size_to_bucket(size: c_int) -> c_int {
    if (size > MAX_CHAIN_BUCKETS) {
    return 0;
    }
    return size - 1;
    }
//
// Iterate all the chain blocks in a bucket.
//

    for ((prev) = -1, (curr) = chain_block_buckets[bucket];	
    (curr) >= 0;					
    (prev) = (curr), (curr) = chain_block_next(curr)) {
//
// next block or -1
//
#[no_mangle]
pub unsafe extern "C" fn chain_block_next(offset: c_int) -> c_int {
    }
pub static mut next: c_int = 0;
    WARN_ON_ONCE!(!(next & CHAIN_BLK_FLAG));
    if (next == CHAIN_BLK_LIST_END) {
    return -1;
    }
    next &= ~CHAIN_BLK_FLAG;
    next <<= 16;
    next |= chain_hlocks[offset + 1];
    return next;
    }
//
// bucket-0 only
//
#[no_mangle]
pub unsafe extern "C" fn chain_block_size(offset: c_int) -> c_int {
    return (chain_hlocks[offset + 2] << 16) | chain_hlocks[offset + 3];
    }
#[no_mangle]
pub unsafe extern "C" fn init_chain_block(offset: c_int, next: c_int, bucket: c_int, size: c_int) {
    chain_hlocks[offset] = (next >> 16) | CHAIN_BLK_FLAG;
    chain_hlocks[offset + 1] = (u16)next;
    if (size && !bucket) {
    chain_hlocks[offset + 2] = size >> 16;
    chain_hlocks[offset + 3] = (u16)size;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn add_chain_block(offset: c_int, size: c_int) {
pub static mut bucket: c_int = 0;
pub static mut next: c_int = 0;
    let mut prev = 0;
    let mut curr = 0;
    if (unlikely(size < 2)) {
//
// We can't store single entries on the freelist. Leak them.
//
// One possible way out would be to uniquely mark them, other
// than with CHAIN_BLK_FLAG, such that we can recover them when
// the block before it is re-added.
//
    if (size) {
    nr_lost_chain_hlocks += 1;
    }
    return;
    }
    nr_free_chain_hlocks += size;
    if (!bucket) {
    nr_large_chain_blocks += 1;
//
// Variable sized, sort large to small.
//
    for_each_chain_block(0, prev, curr) {
    if (size >= chain_block_size(curr)) {
    break;
    }
    }
    init_chain_block(offset, curr, 0, size);
    if (prev < 0) {
    chain_block_buckets[0] = offset;
    }
    else {
    init_chain_block(prev, offset, 0, 0);
    }
    return;
    }
//
// Fixed size, add to head.
//
    init_chain_block(offset, next, bucket, size);
    chain_block_buckets[bucket] = offset;
    }
//
// Only the first block in the list can be deleted.
//
// For the variable size bucket[0], the first block (the largest one) is
// returned, broken up and put back into the pool. So if a chain block of
// length > MAX_CHAIN_BUCKETS is ever used and zapped, it will just be
// queued up after the primordial chain block and never be used until the
// hlock entries in the primordial chain block is almost used up. That
// causes fragmentation and reduce allocation efficiency. That can be
// monitored by looking at the "large chain blocks" number in lockdep_stats.
//
#[no_mangle]
pub unsafe extern "C" fn del_chain_block(bucket: c_int, size: c_int, next: c_int) {
    nr_free_chain_hlocks -= size;
    chain_block_buckets[bucket] = next;
    if (!bucket) {
    nr_large_chain_blocks -= 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn init_chain_block_buckets() {
    let mut i = 0;
    for (i = 0; i < MAX_CHAIN_BUCKETS; i++) {
    chain_block_buckets[i] = -1;
    }
    add_chain_block(0, ARRAY_SIZE!(chain_hlocks));
    }
//
// Return offset of a chain block of the right size or -1 if not found.
//
// Fairly simple worst-fit allocator with the addition of a number of size
// specific free lists.
//
#[no_mangle]
unsafe extern "C" fn alloc_chain_hlocks(req: c_int) -> c_int {
    let mut bucket = 0;
    let mut curr = 0;
    let mut size = 0;
//
// We rely on the MSB to act as an escape bit to denote freelist
// pointers. Make sure this bit isn't set in 'normal' class_idx usage.
//
    BUILD_BUG_ON!((MAX_LOCKDEP_KEYS-1) & CHAIN_BLK_FLAG);
    init_data_structures_once();
    if (nr_free_chain_hlocks < req) {
    return -1;
    }
//
// We require a minimum of 2 (u16) entries to encode a freelist
// 'pointer'.
//
    req = max(req, 2);
    bucket = size_to_bucket(req);
    curr = chain_block_buckets[bucket];
    if (bucket) {
    if (curr >= 0) {
    del_chain_block(bucket, req, chain_block_next(curr));
    return curr;
    }
// Try bucket 0
    curr = chain_block_buckets[0];
    }
//
// The variable sized freelist is sorted by size; the first entry is
// the largest. Use it if it fits.
//
    if (curr >= 0) {
    size = chain_block_size(curr);
    if (likely(size >= req)) {
    del_chain_block(0, size, chain_block_next(curr));
    if (size > req) {
    add_chain_block(curr + req, size - req);
    }
    return curr;
    }
    }
//
// Last resort, split a block in a larger sized bucket.
//
    while (size > req) {
    bucket = size_to_bucket(size);
    curr = chain_block_buckets[bucket];
    if (curr < 0) {
    continue;
    }
    del_chain_block(bucket, size, chain_block_next(curr));
    add_chain_block(curr + req, size - req);
    return curr;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn free_chain_hlocks(base: c_int, size: c_int) {
    add_chain_block(base, max(size, 2));
    }
#[no_mangle]
pub unsafe extern "C" fn lock_chain_get_class(chain: *mut lock_chain, i: c_int) -> *mut c_void {
pub static mut chain_hlock: u16 = 0;
pub static mut class_idx: c_uint = 0;
    return lock_classes + class_idx;
    }
//
// Returns the index of the first held_lock of the current chain
//
#[no_mangle]
pub unsafe extern "C" fn get_first_held_lock(curr: *mut task_struct, hlock: *mut held_lock) -> c_int {
    let mut i = 0;
pub static mut hlock_curr: *mut c_void = core::ptr::null_mut();
    while (i >= 0) {
    hlock_curr = curr.held_locks + i;
    if (hlock_curr.irq_context != hlock.irq_context) {
    break;
    }
    }
    return i += 1;
    }

//
// Returns the next chain_key iteration
//
#[no_mangle]
unsafe extern "C" fn print_chain_key_iteration(hlock_id: u16, chain_key: u64) -> u64 {
pub static mut new_chain_key: u64 = 0;
    printk(" hlock_id:%d . chain_key:%016Lx",
    (unsigned int)hlock_id,
    (unsigned long long)new_chain_key);
    return new_chain_key;
    }
#[no_mangle]
pub unsafe extern "C" fn print_chain_keys_held_locks(curr: *mut task_struct, hlock_next: *mut held_lock) {
pub static mut hlock: *mut c_void = core::ptr::null_mut();
pub static mut chain_key: u64 = 0;
pub static mut depth: c_int = 0;
pub static mut i: c_int = 0;
    printk("depth: %u (irq_context %u)\n", depth - i + 1,
    hlock_next.irq_context);
    while (i < depth) {
    hlock = curr.held_locks + i;
    chain_key = print_chain_key_iteration(hlock_id(hlock), chain_key);
    print_lock(hlock);
    }
    print_chain_key_iteration(hlock_id(hlock_next), chain_key);
    print_lock(hlock_next);
    }
#[no_mangle]
unsafe extern "C" fn print_chain_keys_chain(chain: *mut lock_chain) {
    let mut i = 0;
pub static mut chain_key: u64 = 0;
    let mut hlock_id = 0;
    printk("depth: %u\n", chain.depth);
    while (i < chain.depth) {
    hlock_id = chain_hlocks[chain.base + i];
    chain_key = print_chain_key_iteration(hlock_id, chain_key);
    print_lock_name(core::ptr::null_mut(), lock_classes + chain_hlock_class_idx(hlock_id));
    printk("\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_collision(curr: *mut task_struct, hlock_next: *mut held_lock, chain: *mut lock_chain) {
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("============================\n");
    pr_warn!("WARNING: chain_key collision\n");
    print_kernel_ident();
    pr_warn!("----------------------------\n");
    pr_warn!("%s/%d: ", current.comm, task_pid_nr(current));
    pr_warn!("Hash chain already cached but the contents don't match!\n");
    pr_warn!("Held locks:");
    print_chain_keys_held_locks(curr, hlock_next);
    pr_warn!("Locks in cached chain:");
    print_chain_keys_chain(chain);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }

//
// Checks whether the chain and the current held locks are consistent
// in depth and also in content. If they are not it most likely means
// that there was a collision during the calculation of the chain_key.
// Returns: 0 not passed, 1 passed
//
#[no_mangle]
pub unsafe extern "C" fn check_no_collision(curr: *mut task_struct, hlock: *mut held_lock, chain: *mut lock_chain) -> c_int {

    let mut i = 0;
    let mut j = 0;
    let mut id = 0;
    i = get_first_held_lock(curr, hlock);
    if (DEBUG_LOCKS_WARN_ON(chain.depth != curr.lockdep_depth - (i - 1))) {
    print_collision(curr, hlock, chain);
    return 0;
    }
    while (j < chain.depth - 1) {
    id = hlock_id(&curr.held_locks[i]);
    if (DEBUG_LOCKS_WARN_ON(chain_hlocks[chain.base + j] != id)) {
    print_collision(curr, hlock, chain);
    return 0;
    }
    }

    return 1;
    }
//
// Given an index that is >= -1, return the index of the next lock chain.
// Return -2 if there is no next lock chain.
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_next_lockchain(i: c_long) -> c_long {
    i = find_next_bit(lock_chains_in_use, ARRAY_SIZE!(lock_chains), i + 1);
    return i < ARRAY_SIZE!(lock_chains) ? i : -2;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_chain_count() -> c_ulong {
    return bitmap_weight(lock_chains_in_use, ARRAY_SIZE!(lock_chains));
    }
// Must be called with the graph lock held.
#[no_mangle]
pub unsafe extern "C" fn alloc_lock_chain() -> *mut c_void {
    let mut idx = find_first_zero_bit(lock_chains_in_use,
    ARRAY_SIZE!(lock_chains));
    if (unlikely(idx >= ARRAY_SIZE!(lock_chains))) {
    return core::ptr::null_mut();
    }
    __set_bit(idx, lock_chains_in_use);
    return lock_chains + idx;
    }
//
// Adds a dependency chain into chain hashtable. And must be called with
// graph_lock held.
//
// Return 0 if fail, and graph_lock is released.
// Return 1 if succeed, with graph_lock held.
//
#[no_mangle]
pub unsafe extern "C" fn add_chain_cache(curr: *mut task_struct, hlock: *mut held_lock, chain_key: u64) -> c_int {
    let mut hash_head = chainhashentry(chain_key);
pub static mut chain: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
//
// The caller must hold the graph lock, ensure we've got IRQs
// disabled to make this an IRQ-safe lock.. for recursion reasons
// lockdep won't complain about its own locking errors.
//
    if (lockdep_assert_locked()) {
    return 0;
    }
    chain = alloc_lock_chain();
    if (!chain) {
    if (!debug_locks_off_graph_unlock()) {
    return 0;
    }
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_LOCKDEP_CHAINS too low!");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return 0;
    }
    chain.chain_key = chain_key;
    chain.irq_context = hlock.irq_context;
    i = get_first_held_lock(curr, hlock);
    chain.depth = curr.lockdep_depth + 1 - i;
    BUILD_BUG_ON!((1UL << 24) <= ARRAY_SIZE!(chain_hlocks));
    BUILD_BUG_ON!((1UL << 6)  <= ARRAY_SIZE!(curr.held_locks));
    BUILD_BUG_ON!((1UL << 8*sizeof!(chain_hlocks[0])) <= ARRAY_SIZE!(lock_classes));
    j = alloc_chain_hlocks(chain.depth);
    if (j < 0) {
    if (!debug_locks_off_graph_unlock()) {
    return 0;
    }
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_LOCKDEP_CHAIN_HLOCKS too low!");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return 0;
    }
    chain.base = j;
    while (j < chain.depth - 1) {
pub static mut lock_id: c_int = 0;
    chain_hlocks[chain.base + j] = lock_id;
    }
    chain_hlocks[chain.base + j] = hlock_id(hlock);
    hlist_add_head_rcu(&chain.entry, hash_head);
    debug_atomic_inc(chain_lookup_misses);
    inc_chains(chain.irq_context);
    return 1;
    }
//
// Look up a dependency chain. Must be called with either the graph lock or
// the RCU read lock held.
//
#[no_mangle]
pub unsafe extern "C" fn lookup_chain_cache(chain_key: u64) -> *mut c_void {
    let mut hash_head = chainhashentry(chain_key);
pub static mut chain: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry_rcu(chain, hash_head, entry) {
    if (READ_ONCE(chain.chain_key) == chain_key) {
    debug_atomic_inc(chain_lookup_hits);
    return chain;
    }
    }
    return core::ptr::null_mut();
    }
//
// If the key is not present yet in dependency chain cache then
// add it and return 1 - in this case the new dependency chain is
// validated. If the key is already hashed, return 0.
// (On return with 1 graph_lock is held.)
//
#[no_mangle]
pub unsafe extern "C" fn lookup_chain_cache_add(curr: *mut task_struct, hlock: *mut held_lock, chain_key: u64) -> c_int {
    let mut class = hlock_class(hlock);
    let mut chain = lookup_chain_cache(chain_key);
    if (chain) {
// label;
    if (!check_no_collision(curr, hlock, chain)) {
    return 0;
    }
    if (very_verbose(class)) {
    printk("\nhash chain already cached, key: "
    "%016Lx tail class: [%px] %s\n",
    (unsigned long long)chain_key,
    class.key, class.name);
    }
    return 0;
    }
    if (very_verbose(class)) {
    printk("\nnew hash chain, key: %016Lx tail class: [%px] %s\n",
    (unsigned long long)chain_key, class.key, class.name);
    }
    if (!graph_lock()) {
    return 0;
    }
//
// We have to walk the chain again locked - to avoid duplicates:
//
    chain = lookup_chain_cache(chain_key);
    if (chain) {
    graph_unlock();
// goto;
    }
    if (!add_chain_cache(curr, hlock, chain_key)) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn validate_chain(curr: *mut task_struct, hlock: *mut held_lock, chain_head: c_int, chain_key: u64) -> c_int {
//
// Trylock needs to maintain the stack of held locks, but it
// does not add new dependencies, because trylock can be done
// in any order.
//
// We look up the chain_key and do the O(N^2) check and update of
// the dependencies only if this is a new dependency chain.
// (If lookup_chain_cache_add() return with 1 it acquires
// graph_lock for us)
//
    if (!hlock.trylock && hlock.check &&
    lookup_chain_cache_add(curr, hlock, chain_key)) {
//
// Check whether last held lock:
//
// - is irq-safe, if this lock is irq-unsafe
// - is softirq-safe, if this lock is hardirq-unsafe
//
// And check whether the new lock's dependency graph
// could lead back to the previous lock:
//
// - within the current held-lock stack
// - across our accumulated lock dependency records
//
// any of these scenarios could lead to a deadlock.
//
// The simple case: does the current hold the same lock
// already?
//
pub static mut ret: c_int = 0;
    if (!ret) {
    return 0;
    }
//
// Add dependency only if this lock is not the head
// of the chain, and if the new lock introduces no more
// lock dependency (because we already hold a lock with the
// same lock class) nor deadlock (because the nest_lock
// serializes nesting locks), see the comments for
// check_deadlock().
//
    if (!chain_head && ret != 2) {
    if (!check_prevs_add(curr, hlock)) {
    return 0;
    }
    }
    graph_unlock();
    } else {
// after lookup_chain_cache_add():
    if (unlikely(!debug_locks)) {
    return 0;
    }
    }
    return 1;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: validate_chain
pub unsafe extern "C" fn validate_chain_dup(curr: *mut task_struct, hlock: *mut held_lock, chain_head: c_int, chain_key: u64) -> c_int {
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn init_chain_block_buckets() { }

//
// We are building curr_chain_key incrementally, so double-check
// it from scratch, to make sure that it's done correctly:
//
#[no_mangle]
unsafe extern "C" fn check_chain_key(curr: *mut task_struct) {

    struct held_lock *hlock, *prev_hlock = core::ptr::null_mut();
    let mut i = 0;
pub static mut chain_key: u64 = 0;
    while (i < curr.lockdep_depth) {
    hlock = curr.held_locks + i;
    if (chain_key != hlock.prev_chain_key) {
    debug_locks_off();
//
// We got mighty confused, our chain keys don't match
// with what we expect, someone trample on our task state?
//
    WARN(1, "hm#1, depth: %u [%u], %016Lx != %016Lx\n",
    curr.lockdep_depth, i,
    (unsigned long long)chain_key,
    (unsigned long long)hlock.prev_chain_key);
    return;
    }
//
// hlock->class_idx can't go beyond MAX_LOCKDEP_KEYS, but is
// it registered lock class index?
//
    if (DEBUG_LOCKS_WARN_ON(!test_bit(hlock.class_idx, lock_classes_in_use))) {
    return;
    }
    if (prev_hlock && (prev_hlock.irq_context !=
    hlock.irq_context)) {
    chain_key = INITIAL_CHAIN_KEY;
    }
    chain_key = iterate_chain_key(chain_key, hlock_id(hlock));
    prev_hlock = hlock;
    }
    if (chain_key != curr.curr_chain_key) {
    debug_locks_off();
//
// More smoking hash instead of calculating it, damn see these
// numbers float.. I bet that a pink elephant stepped on my memory.
//
    WARN(1, "hm#2, depth: %u [%u], %016Lx != %016Lx\n",
    curr.lockdep_depth, i,
    (unsigned long long)chain_key,
    (unsigned long long)curr.curr_chain_key);
    }

    }

// forward_decl: mark_lock;
#[no_mangle]
unsafe extern "C" fn print_usage_bug_scenario(lock: *mut held_lock) {
    let mut class = hlock_class(lock);
    printk(" Possible unsafe locking scenario:\n\n");
    printk("       CPU0\n");
    printk("       ----\n");
    printk("  lock(");
    __print_lock_name(lock, class);
    printk(");\n");
    printk("  <Interrupt>\n");
    printk("    lock(");
    __print_lock_name(lock, class);
    printk(");\n");
    printk("\n *** DEADLOCK ***\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn print_usage_bug(curr: *mut task_struct, this: *mut held_lock, prev_bit: lock_usage_bit, new_bit: lock_usage_bit) {
    if (!debug_locks_off() || debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("================================\n");
    pr_warn!("WARNING: inconsistent lock state\n");
    print_kernel_ident();
    pr_warn!("--------------------------------\n");
    pr_warn!("inconsistent {%s} . {%s} usage.\n",
    usage_str[prev_bit], usage_str[new_bit]);
    pr_warn!("%s/%d [HC%u[%lu]:SC%u[%lu]:HE%u:SE%u] takes:\n",
    curr.comm, task_pid_nr(curr),
    lockdep_hardirq_context(), hardirq_count() >> HARDIRQ_SHIFT,
    lockdep_softirq_context(curr), softirq_count() >> SOFTIRQ_SHIFT,
    lockdep_hardirqs_enabled(),
    lockdep_softirqs_enabled(curr));
    print_lock(this);
    pr_warn!("{%s} state was registered at:\n", usage_str[prev_bit]);
    print_lock_trace(hlock_class(this).usage_traces[prev_bit], 1);
    print_irqtrace_events(curr);
    pr_warn!("\nother info that might help us debug this:\n");
    print_usage_bug_scenario(this);
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
//
// Print out an error if an invalid bit is set:
//
#[no_mangle]
pub unsafe extern "C" fn valid_state(curr: *mut task_struct, this: *mut held_lock, new_bit: lock_usage_bit, bad_bit: lock_usage_bit) -> c_int {
    if (unlikely(hlock_class(this).usage_mask & (1 << bad_bit))) {
    graph_unlock();
    print_usage_bug(curr, this, bad_bit, new_bit);
    return 0;
    }
    return 1;
    }
//
// print irq inversion bug:
//
#[no_mangle]
pub unsafe extern "C" fn print_irq_inversion_bug(curr: *mut task_struct, root: *mut lock_list, other: *mut lock_list, this: *mut held_lock, forwards: c_int, irqclass: *mut c_char) {
    let mut entry = other;
    let mut middle = core::ptr::null_mut();
    let mut depth = 0;
    if (!debug_locks_off_graph_unlock() || debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("========================================================\n");
    pr_warn!("WARNING: possible irq lock inversion dependency detected\n");
    print_kernel_ident();
    pr_warn!("--------------------------------------------------------\n");
    pr_warn!("%s/%d just changed the state of lock:\n",
    curr.comm, task_pid_nr(curr));
    print_lock(this);
    if (forwards) {
    pr_warn!("but this lock took another, %s-unsafe lock in the past:\n", irqclass);
    }
    else {
    pr_warn!("but this lock was taken by another, %s-safe lock in the past:\n", irqclass);
    }
    print_lock_name(core::ptr::null_mut(), other.class);
    pr_warn!("\n\nand interrupts could create inverse lock ordering between them.\n\n");
    pr_warn!("\nother info that might help us debug this:\n");
// Find a middle lock (if one exists)
    depth = get_lock_depth(other);
    do {
    if (depth == 0 && (entry != root)) {
    pr_warn!("lockdep:%s bad path found in chain graph\n", __func__);
    break;
    }
    middle = entry;
    entry = get_lock_parent(entry);
    depth -= 1;
    } while (entry && entry != root && (depth >= 0));
    if (forwards) {
    print_irq_lock_scenario(root, other,
    middle ? middle.class : root.class, other.class);
    }
    else {
    print_irq_lock_scenario(other, root,
    middle ? middle.class : other.class, root.class);
    }
    lockdep_print_held_locks(curr);
    pr_warn!("\nthe shortest dependencies between 2nd lock and 1st lock:\n");
    root.trace = save_trace();
    if (!root.trace) {
// goto;
    }
    print_shortest_lock_dependencies(other, root);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
// label;
    nbcon_cpu_emergency_exit();
    }
//
// Prove that in the forwards-direction subgraph starting at <this>
// there is no lock matching <mask>:
//
#[no_mangle]
pub unsafe extern "C" fn check_usage_forwards(curr: *mut task_struct, this: *mut held_lock, bit: lock_usage_bit) -> c_int {
    enum bfs_result ret;
pub static mut root: usize = 0;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
pub static mut read_bit: lock_usage_bit = 0;
pub static mut usage_mask: unsigned = 0;
    bfs_init_root(&root, this);
    ret = find_usage_forwards(&root, usage_mask, &target_entry);
    if (bfs_error(ret)) {
    print_bfs_bug(ret);
    return 0;
    }
    if (ret == BFS_RNOMATCH) {
    return 1;
    }
// Check whether write or read usage is the match
    if (target_entry.class.usage_mask & lock_flag(bit)) {
    print_irq_inversion_bug(curr, &root, target_entry,
    this, 1, state_name(bit));
    } else {
    print_irq_inversion_bug(curr, &root, target_entry,
    this, 1, state_name(read_bit));
    }
    return 0;
    }
//
// Prove that in the backwards-direction subgraph starting at <this>
// there is no lock matching <mask>:
//
#[no_mangle]
pub unsafe extern "C" fn check_usage_backwards(curr: *mut task_struct, this: *mut held_lock, bit: lock_usage_bit) -> c_int {
    enum bfs_result ret;
pub static mut root: usize = 0;
pub static mut target_entry: *mut c_void = core::ptr::null_mut();
pub static mut read_bit: lock_usage_bit = 0;
pub static mut usage_mask: unsigned = 0;
    bfs_init_rootb(&root, this);
    ret = find_usage_backwards(&root, usage_mask, &target_entry);
    if (bfs_error(ret)) {
    print_bfs_bug(ret);
    return 0;
    }
    if (ret == BFS_RNOMATCH) {
    return 1;
    }
// Check whether write or read usage is the match
    if (target_entry.class.usage_mask & lock_flag(bit)) {
    print_irq_inversion_bug(curr, &root, target_entry,
    this, 0, state_name(bit));
    } else {
    print_irq_inversion_bug(curr, &root, target_entry,
    this, 0, state_name(read_bit));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn print_irqtrace_events(curr: *mut task_struct) {
    let mut trace = &curr.irqtrace;
    nbcon_cpu_emergency_enter();
    printk("irq event stamp: %u\n", trace.irq_events);
    printk("hardirqs last  enabled at (%u): [<%px>] %pS\n",
    trace.hardirq_enable_event, trace.hardirq_enable_ip,
    trace.hardirq_enable_ip);
    printk("hardirqs last disabled at (%u): [<%px>] %pS\n",
    trace.hardirq_disable_event, trace.hardirq_disable_ip,
    trace.hardirq_disable_ip);
    printk("softirqs last  enabled at (%u): [<%px>] %pS\n",
    trace.softirq_enable_event, trace.softirq_enable_ip,
    trace.softirq_enable_ip);
    printk("softirqs last disabled at (%u): [<%px>] %pS\n",
    trace.softirq_disable_event, trace.softirq_disable_ip,
    trace.softirq_disable_ip);
    nbcon_cpu_emergency_exit();
    }
#[no_mangle]
unsafe extern "C" fn HARDIRQ_verbose(class: *mut lock_class) -> c_int {

    return class_filter(class);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn SOFTIRQ_verbose(class: *mut lock_class) -> c_int {

    return class_filter(class);

    return 0;
    }
    static int (*state_verbose_f[])(lock_class *class) = {

    __STATE##_verbose,

    };
#[no_mangle]
pub unsafe extern "C" fn state_verbose(bit: lock_usage_bit, class: *mut lock_class) -> c_int {
    return state_verbose_f[bit >> LOCK_USAGE_DIR_MASK](class);
    }
    typedef int (*check_usage_f)(task_struct *, held_lock *,
    enum lock_usage_bit bit, const char *name);
#[no_mangle]
pub unsafe extern "C" fn mark_lock_irq(curr: *mut task_struct, this: *mut held_lock, new_bit: lock_usage_bit) -> c_int {
pub static mut excl_bit: c_int = 0;
pub static mut read: c_int = 0;
pub static mut dir: c_int = 0;
//
// Validate that this particular lock does not have conflicting
// usage states.
//
    if (!valid_state(curr, this, new_bit, excl_bit)) {
    return 0;
    }
//
// Check for read in write conflicts
//
    if (!read && !valid_state(curr, this, new_bit,
    excl_bit + LOCK_USAGE_READ_MASK)) {
    return 0;
    }
//
// Validate that the lock dependencies don't have conflicting usage
// states.
//
    if (dir) {
//
// mark ENABLED has to look backwards -- to ensure no dependee
// has USED_IN state, which, again, would allow  recursion deadlocks.
//
    if (!check_usage_backwards(curr, this, excl_bit)) {
    return 0;
    }
    } else {
//
// mark USED_IN has to look forwards -- to ensure no dependency
// has ENABLED state, which would allow recursion deadlocks.
//
    if (!check_usage_forwards(curr, this, excl_bit)) {
    return 0;
    }
    }
    if (state_verbose(new_bit, hlock_class(this))) {
    return 2;
    }
    return 1;
    }
//
// Mark all held locks with a usage bit:
//
#[no_mangle]
pub unsafe extern "C" fn mark_held_locks(curr: *mut task_struct, base_bit: lock_usage_bit) -> c_int {
pub static mut hlock: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < curr.lockdep_depth) {
pub static mut hlock_bit: lock_usage_bit = 0;
    hlock = curr.held_locks + i;
    if (hlock.read) {
    hlock_bit += LOCK_USAGE_READ_MASK;
    }
    BUG_ON!(hlock_bit >= LOCK_USAGE_STATES);
    if (!hlock.check) {
    continue;
    }
    if (!mark_lock(curr, hlock, hlock_bit)) {
    return 0;
    }
    }
    return 1;
    }
//
// Hardirqs will be enabled:
//
#[no_mangle]
unsafe extern "C" fn __trace_hardirqs_on_caller() {
    let mut curr = current;
//
// We are going to turn hardirqs on, so set the
// usage bit for all held locks:
//
    if (!mark_held_locks(curr, LOCK_ENABLED_HARDIRQ)) {
    return;
    }
//
// If we have softirqs enabled, then set the usage
// bit for all held locks. (disabled hardirqs prevented
// this bit from being set before)
//
    if (curr.softirqs_enabled) {
    mark_held_locks(curr, LOCK_ENABLED_SOFTIRQ);
    }
    }
//
// lockdep_hardirqs_on_prepare - Prepare for enabling interrupts
//
// Invoked before a possible transition to RCU idle from exit to user or
// guest mode. This ensures that all RCU operations are done before RCU
// stops watching. After the RCU transition lockdep_hardirqs_on() has to be
// invoked to set the final state.
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_hardirqs_on_prepare() {
    if (unlikely(!debug_locks)) {
    return;
    }
//
// NMIs do not (and cannot) track lock dependencies, nothing to do.
//
    if (unlikely(in_nmi())) {
    return;
    }
    if (unlikely(this_cpu_read(lockdep_recursion))) {
    return;
    }
    if (unlikely(lockdep_hardirqs_enabled())) {
//
// Neither irq nor preemption are disabled here
// so this is racy by nature but losing one hit
// in a stat is not a big deal.
//
    __debug_atomic_inc(redundant_hardirqs_on);
    return;
    }
//
// We're enabling irqs and according to our state above irqs weren't
// already enabled, yet we find the hardware thinks they are in fact
// enabled.. someone messed up their IRQ state tracing.
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return;
    }
//
// See the fine text that goes along with this variable definition.
//
    if (DEBUG_LOCKS_WARN_ON(early_boot_irqs_disabled)) {
    return;
    }
//
// Can't allow enabling interrupts while in an interrupt handler,
// that's general bad form and such. Recursion, limited stack etc..
//
    if (DEBUG_LOCKS_WARN_ON(lockdep_hardirq_context())) {
    return;
    }
    current.hardirq_chain_key = current.curr_chain_key;
    lockdep_recursion_inc();
    __trace_hardirqs_on_caller();
    lockdep_recursion_finish();
    }
    EXPORT_SYMBOL_GPL(lockdep_hardirqs_on_prepare);
#[no_mangle]
pub unsafe extern "C" fn lockdep_hardirqs_on(ip: c_ulong) -> void noinstr {
    let mut trace = &current.irqtrace;
    if (unlikely(!debug_locks)) {
    return;
    }
//
// NMIs can happen in the middle of local_irq_{en,dis}able() where the
// tracking state and hardware state are out of sync.
//
// NMIs must save lockdep_hardirqs_enabled() to restore IRQ state from,
// and not rely on hardware state like normal interrupts.
//
    if (unlikely(in_nmi())) {
    if (!IS_ENABLED!(CONFIG_TRACE_IRQFLAGS_NMI)) {
    return;
    }
//
// Skip:
// - recursion check, because NMI can hit lockdep;
// - hardware state check, because above;
// - chain_key check, see lockdep_hardirqs_on_prepare().
//
// goto;
    }
    if (unlikely(this_cpu_read(lockdep_recursion))) {
    return;
    }
    if (lockdep_hardirqs_enabled()) {
//
// Neither irq nor preemption are disabled here
// so this is racy by nature but losing one hit
// in a stat is not a big deal.
//
    __debug_atomic_inc(redundant_hardirqs_on);
    return;
    }
//
// We're enabling irqs and according to our state above irqs weren't
// already enabled, yet we find the hardware thinks they are in fact
// enabled.. someone messed up their IRQ state tracing.
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return;
    }
//
// Ensure the lock stack remained unchanged between
// lockdep_hardirqs_on_prepare() and lockdep_hardirqs_on().
//
    DEBUG_LOCKS_WARN_ON(current.hardirq_chain_key !=
    current.curr_chain_key);
// label;
// we'll do an OFF -> ON transition:
    __this_cpu_write(hardirqs_enabled, 1);
    trace.hardirq_enable_ip = ip;
    trace.hardirq_enable_event = ++trace.irq_events;
    debug_atomic_inc(hardirqs_on_events);
    }
    EXPORT_SYMBOL_GPL(lockdep_hardirqs_on);
//
// Hardirqs were disabled:
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_hardirqs_off(ip: c_ulong) -> void noinstr {
    if (unlikely(!debug_locks)) {
    return;
    }
//
// Matching lockdep_hardirqs_on(), allow NMIs in the middle of lockdep;
// they will restore the software state. This ensures the software
// state is consistent inside NMIs as well.
//
    if (in_nmi()) {
    if (!IS_ENABLED!(CONFIG_TRACE_IRQFLAGS_NMI)) {
    return;
    }
    } else if (__this_cpu_read(lockdep_recursion)) {
    return;
    }
//
// So we're supposed to get called after you mask local IRQs, but for
// some reason the hardware doesn't quite think you did a proper job.
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return;
    }
    if (lockdep_hardirqs_enabled()) {
    let mut trace = &current.irqtrace;
//
// We have done an ON -> OFF transition:
//
    __this_cpu_write(hardirqs_enabled, 0);
    trace.hardirq_disable_ip = ip;
    trace.hardirq_disable_event = ++trace.irq_events;
    debug_atomic_inc(hardirqs_off_events);
    } else {
    debug_atomic_inc(redundant_hardirqs_off);
    }
    }
    EXPORT_SYMBOL_GPL(lockdep_hardirqs_off);
//
// Softirqs will be enabled:
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_softirqs_on(ip: c_ulong) {
    let mut trace = &current.irqtrace;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
//
// We fancy IRQs being disabled here, see softirq.c, avoids
// funny state and nesting things.
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return;
    }
    if (current.softirqs_enabled) {
    debug_atomic_inc(redundant_softirqs_on);
    return;
    }
    lockdep_recursion_inc();
//
// We'll do an OFF -> ON transition:
//
    current.softirqs_enabled = 1;
    trace.softirq_enable_ip = ip;
    trace.softirq_enable_event = ++trace.irq_events;
    debug_atomic_inc(softirqs_on_events);
//
// We are going to turn softirqs on, so set the
// usage bit for all held locks, if hardirqs are
// enabled too:
//
    if (lockdep_hardirqs_enabled()) {
    mark_held_locks(current, LOCK_ENABLED_SOFTIRQ);
    }
    lockdep_recursion_finish();
    }
//
// Softirqs were disabled:
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_softirqs_off(ip: c_ulong) {
    if (unlikely(!lockdep_enabled())) {
    return;
    }
//
// We fancy IRQs being disabled here, see softirq.c
//
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return;
    }
    if (current.softirqs_enabled) {
    let mut trace = &current.irqtrace;
//
// We have done an ON -> OFF transition:
//
    current.softirqs_enabled = 0;
    trace.softirq_disable_ip = ip;
    trace.softirq_disable_event = ++trace.irq_events;
    debug_atomic_inc(softirqs_off_events);
//
// Whoops, we wanted softirqs off, so why aren't they?
//
    DEBUG_LOCKS_WARN_ON(!softirq_count());
    } else {
    debug_atomic_inc(redundant_softirqs_off);
    }
    }
//
// lockdep_cleanup_dead_cpu - Ensure CPU lockdep state is cleanly stopped
//
// @cpu: index of offlined CPU
// @idle: task pointer for offlined CPU's idle thread
//
// Invoked after the CPU is dead. Ensures that the tracing infrastructure
// is left in a suitable state for the CPU to be subsequently brought
// online again.
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_cleanup_dead_cpu(cpu: c_uint, idle: *mut task_struct) {
    if (unlikely(!debug_locks)) {
    return;
    }
    if (unlikely(per_cpu(hardirqs_enabled, cpu))) {
    pr_warn!("CPU %u left hardirqs enabled!", cpu);
    if (idle) {
    print_irqtrace_events(idle);
    }
// Clean it up for when the CPU comes online again.
    per_cpu(hardirqs_enabled, cpu) = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mark_usage(curr: *mut task_struct, hlock: *mut held_lock, check: c_int) -> c_int {
    if (!check) {
// goto;
    }
//
// If non-trylock use in a hardirq or softirq context, then
// mark the lock as used in these contexts:
//
    if (!hlock.trylock) {
    if (hlock.read) {
    if (lockdep_hardirq_context()) {
    if (!mark_lock(curr, hlock,
    LOCK_USED_IN_HARDIRQ_READ))
    return 0;
    }
    if (curr.softirq_context) {
    if (!mark_lock(curr, hlock,
    LOCK_USED_IN_SOFTIRQ_READ))
    return 0;
    }
    } else {
    if (lockdep_hardirq_context()) {
    if (!mark_lock(curr, hlock, LOCK_USED_IN_HARDIRQ))
    return 0;
    }
    if (curr.softirq_context) {
    if (!mark_lock(curr, hlock, LOCK_USED_IN_SOFTIRQ))
    return 0;
    }
    }
    }
//
// For lock_sync(), don't mark the ENABLED usage, since lock_sync()
// creates no critical section and no extra dependency can be introduced
// by interrupts
//
    if (!hlock.hardirqs_off && !hlock.sync) {
    if (hlock.read) {
    if (!mark_lock(curr, hlock,
    LOCK_ENABLED_HARDIRQ_READ)) {
    return 0;
    }
    if (curr.softirqs_enabled) {
    if (!mark_lock(curr, hlock,
    LOCK_ENABLED_SOFTIRQ_READ))
    return 0;
    }
    } else {
    if (!mark_lock(curr, hlock,
    LOCK_ENABLED_HARDIRQ)) {
    return 0;
    }
    if (curr.softirqs_enabled) {
    if (!mark_lock(curr, hlock,
    LOCK_ENABLED_SOFTIRQ))
    return 0;
    }
    }
    }
// label;
// mark it as used:
    if (!mark_lock(curr, hlock, LOCK_USED)) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn task_irq_context(task: *mut task_struct) -> c_uint {
    return LOCK_CHAIN_HARDIRQ_CONTEXT * !!lockdep_hardirq_context() +
    LOCK_CHAIN_SOFTIRQ_CONTEXT * !!task.softirq_context;
    }
#[no_mangle]
pub unsafe extern "C" fn separate_irq_context(curr: *mut task_struct, hlock: *mut held_lock) -> c_int {
pub static mut depth: c_uint = 0;
//
// Keep track of points where we cross into an interrupt context:
//
    if (depth) {
pub static mut prev_hlock: *mut c_void = core::ptr::null_mut();
    prev_hlock = curr.held_locks + depth-1;
//
// If we cross into another context, reset the
// hash key (this also prevents the checking and the
// adding of the dependency to 'prev'):
//
    if (prev_hlock.irq_context != hlock.irq_context) {
    return 1;
    }
    }
    return 0;
    }
//
// Mark a lock with a usage bit, and validate the state transition:
//
#[no_mangle]
pub unsafe extern "C" fn mark_lock(curr: *mut task_struct, this: *mut held_lock, new_bit: lock_usage_bit) -> c_int {
    unsigned int new_mask, ret = 1;
    if (new_bit >= LOCK_USAGE_STATES) {
    DEBUG_LOCKS_WARN_ON(1);
    return 0;
    }
    if (new_bit == LOCK_USED && this.read) {
    new_bit = LOCK_USED_READ;
    }
    new_mask = 1 << new_bit;
//
// If already set then do not dirty the cacheline,
// nor do any checks:
//
    if (likely(hlock_class(this).usage_mask & new_mask)) {
    return 1;
    }
    if (!graph_lock()) {
    return 0;
    }
//
// Make sure we didn't race:
//
    if (unlikely(hlock_class(this).usage_mask & new_mask)) {
// goto;
    }
    if (!hlock_class(this).usage_mask) {
    debug_atomic_dec(nr_unused_locks);
    }
    hlock_class(this).usage_mask |= new_mask;
    if (new_bit < LOCK_TRACE_STATES) {
    if (!(hlock_class(this).usage_traces[new_bit] = save_trace())) {
    return 0;
    }
    }
    if (new_bit < LOCK_USED) {
    ret = mark_lock_irq(curr, this, new_bit);
    if (!ret) {
    return 0;
    }
    }
// label;
    graph_unlock();
//
// We must printk outside of the graph_lock:
//
    if (ret == 2) {
    nbcon_cpu_emergency_enter();
    printk("\nmarked lock as {%s}:\n", usage_str[new_bit]);
    print_lock(this);
    print_irqtrace_events(curr);
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn task_wait_context(curr: *mut task_struct) -> c_short {
//
// Set appropriate wait type for the context; for IRQs we have to take
// into account force_irqthread as that is implied by PREEMPT_RT.
//
    if (lockdep_hardirq_context()) {
//
// Check if force_irqthreads will run us threaded.
//
    if (curr.hardirq_threaded || curr.irq_config) {
    return LD_WAIT_CONFIG;
    }
    return LD_WAIT_SPIN;
    } else if (curr.softirq_context) {
//
// Softirqs are always threaded.
//
    return LD_WAIT_CONFIG;
    }
    return LD_WAIT_MAX;
    }
#[no_mangle]
pub unsafe extern "C" fn print_lock_invalid_wait_context(curr: *mut task_struct, hlock: *mut held_lock) -> c_int {
    let mut curr_inner = 0;
    if (!debug_locks_off()) {
    return 0;
    }
    if (debug_locks_silent) {
    return 0;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=============================\n");
    pr_warn!("[ BUG: Invalid wait context ]\n");
    print_kernel_ident();
    pr_warn!("-----------------------------\n");
    pr_warn!("%s/%d is trying to lock:\n", curr.comm, task_pid_nr(curr));
    print_lock(hlock);
    pr_warn!("other info that might help us debug this:\n");
    curr_inner = task_wait_context(curr);
    pr_warn!("context-{%d:%d}\n", curr_inner, curr_inner);
    lockdep_print_held_locks(curr);
    pr_warn!("stack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    return 0;
    }
//
// Verify the wait_type context.
//
// This check validates we take locks in the right wait-type order; that is it
// ensures that we do not take mutexes inside spinlocks and do not attempt to
// acquire spinlocks inside raw_spinlocks and the sort.
//
// The entire thing is slightly more complex because of RCU, RCU is a lock that
// can be taken from (pretty much) any context but also has constraints.
// However when taken in a stricter environment the RCU lock does not loosen
// the constraints.
//
// Therefore we must look for the strictest environment in the lock stack and
// compare that to the lock we're trying to acquire.
//
#[no_mangle]
unsafe extern "C" fn check_wait_context(curr: *mut task_struct, next: *mut held_lock) -> c_int {
pub static mut next_inner: u8 = 0;
pub static mut next_outer: u8 = 0;
    let mut curr_inner = 0;
    let mut depth = 0;
    if (!next_inner || next.trylock) {
    return 0;
    }
    if (!next_outer) {
    next_outer = next_inner;
    }
//
// Find start of current irq_context..
//
    while (depth >= 0) {
    let mut prev = curr.held_locks + depth;
    if (prev.irq_context != next.irq_context) {
    break;
    }
    }
    depth += 1;
    curr_inner = task_wait_context(curr);
    while (depth < curr.lockdep_depth) {
    let mut prev = curr.held_locks + depth;
    let mut class = hlock_class(prev);
pub static mut prev_inner: u8 = 0;
    if (prev_inner) {
//
// We can have a bigger inner than a previous one
// when outer is smaller than inner, as with RCU.
//
// Also due to trylocks.
//
    curr_inner = min(curr_inner, prev_inner);
//
// Allow override for annotations -- this is typically
// only valid/needed for code that only exists when
// CONFIG_PREEMPT_RT=n.
//
    if (unlikely(class.lock_type == LD_LOCK_WAIT_OVERRIDE)) {
    curr_inner = prev_inner;
    }
    }
    }
    if (next_outer > curr_inner) {
    return print_lock_invalid_wait_context(curr, next);
    }
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: mark_usage
pub unsafe extern "C" fn mark_usage_dup(curr: *mut task_struct, hlock: *mut held_lock, check: c_int) -> c_int {
    return 1;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: task_irq_context
pub unsafe extern "C" fn task_irq_context_dup(task: *mut task_struct) -> c_uint {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: separate_irq_context
pub unsafe extern "C" fn separate_irq_context_dup(curr: *mut task_struct, hlock: *mut held_lock) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn check_wait_context(curr: *mut task_struct, next: *mut held_lock) -> c_int {
    return 0;
    }

//
// Initialize a lock instance's lock-class mapping info:
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_init_map_type(lock: *mut lockdep_map, name: *mut c_char, key: *mut lock_class_key, subclass: c_int, inner: u8, outer: u8, lock_type: u8) {
    let mut i = 0;
    for (i = 0; i < NR_LOCKDEP_CACHING_CLASSES; i++) {
    lock.class_cache[i] = core::ptr::null_mut();
    }

    lock.cpu = raw_smp_processor_id();

//
// Can't be having no nameless bastards around this place!
//
    if (DEBUG_LOCKS_WARN_ON(!name)) {
    lock.name = "core::ptr::null_mut()";
    return;
    }
    lock.name = name;
    lock.wait_type_outer = outer;
    lock.wait_type_inner = inner;
    lock.lock_type = lock_type;
//
// No key, no joy, we need to hash something.
//
    if (DEBUG_LOCKS_WARN_ON(!key)) {
    return;
    }
//
// Sanity check, the lock-class key must either have been allocated
// statically or must have been registered as a dynamic key.
//
    if (!static_obj(key) && !is_dynamic_key(key)) {
    if (debug_locks) {
    printk("BUG: key %px has not been registered!\n", key);
    }
    DEBUG_LOCKS_WARN_ON(1);
    return;
    }
    lock.key = key;
    if (unlikely(!debug_locks)) {
    return;
    }
    if (subclass) {
    let mut flags = 0;
    if (DEBUG_LOCKS_WARN_ON(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    lockdep_recursion_inc();
    register_lock_class(lock, subclass, 1);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    }
    EXPORT_SYMBOL_GPL(lockdep_init_map_type);
pub static mut __lockdep_no_validate__: usize = 0;
    EXPORT_SYMBOL_GPL(__lockdep_no_validate__);
pub static mut __lockdep_no_track__: usize = 0;
    EXPORT_SYMBOL_GPL(__lockdep_no_track__);

#[no_mangle]
pub unsafe extern "C" fn lockdep_set_lock_cmp_fn(lock: *mut lockdep_map, cmp_fn: lock_cmp_fn, print_fn: lock_print_fn) {
    let mut class = lock.class_cache[0];
    let mut flags = 0;
    raw_local_irq_save(flags);
    lockdep_recursion_inc();
    if (!class) {
    class = register_lock_class(lock, 0, 0);
    }
    if (class) {
    WARN_ON!(class.cmp_fn	&& class.cmp_fn != cmp_fn);
    WARN_ON!(class.print_fn && class.print_fn != print_fn);
    class.cmp_fn	= cmp_fn;
    class.print_fn = print_fn;
    }
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lockdep_set_lock_cmp_fn);

#[no_mangle]
pub unsafe extern "C" fn print_lock_nested_lock_not_held(curr: *mut task_struct, hlock: *mut held_lock) {
    if (!debug_locks_off()) {
    return;
    }
    if (debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("==================================\n");
    pr_warn!("WARNING: Nested lock was not taken\n");
    print_kernel_ident();
    pr_warn!("----------------------------------\n");
    pr_warn!("%s/%d is trying to lock:\n", curr.comm, task_pid_nr(curr));
    print_lock(hlock);
    pr_warn!("\nbut this task is not holding:\n");
    pr_warn!("%s\n", hlock.nest_lock.name);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    pr_warn!("\nother info that might help us debug this:\n");
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
// forward_decl: __lock_is_held;
//
// This gets called for every mutex_lock*()/spin_lock*() operation.
// We maintain the dependency maps and validate the locking attempt:
//
// The callers must make sure that IRQs are disabled before calling it,
// otherwise we could get an interrupt which would want to take locks,
// which would end up in lockdep again.
//
#[no_mangle]
pub unsafe extern "C" fn __lock_acquire(lock: *mut lockdep_map, subclass: c_uint, trylock: c_int, read: c_int, check: c_int, hardirqs_off: c_int, nest_lock: *mut lockdep_map, ip: c_ulong, references: c_int, pin_count: c_int, sync: c_int, seq: c_int) -> c_int {
    let mut curr = current;
    let mut class = core::ptr::null_mut();
pub static mut hlock: *mut c_void = core::ptr::null_mut();
    let mut depth = 0;
pub static mut chain_head: c_int = 0;
    let mut class_idx = 0;
    let mut chain_key = 0;
    if (unlikely(!debug_locks)) {
    return 0;
    }
    if (unlikely(lock.key == &__lockdep_no_track__)) {
    return 0;
    }
    lockevent_inc(lockdep_acquire);
    if (!prove_locking || lock.key == &__lockdep_no_validate__) {
    check = 0;
    lockevent_inc(lockdep_nocheck);
    }
    if (DEBUG_LOCKS_WARN_ON(subclass >= MAX_LOCKDEP_SUBCLASSES)) {
    return 0;
    }
    if (subclass < NR_LOCKDEP_CACHING_CLASSES) {
    class = lock.class_cache[subclass];
    }
//
// Not cached?
//
    if (unlikely(!class)) {
    class = register_lock_class(lock, subclass, 0);
    if (!class) {
    return 0;
    }
    }
    debug_class_ops_inc(class);
    if (very_verbose(class)) {
    nbcon_cpu_emergency_enter();
    printk("\nacquire class [%px] %s", class.key, class.name);
    if (class.name_version > 1) {
    printk("#%d", class.name_version);
    }
    printk("\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
//
// Add the lock to the list of currently held locks.
// (we dont increase the depth just yet, up until the
// dependency checks are done)
//
    depth = curr.lockdep_depth;
//
// Ran out of static storage for our per-task lock stack again have we?
//
    if (DEBUG_LOCKS_WARN_ON(depth >= MAX_LOCK_DEPTH)) {
    return 0;
    }
    class_idx = class - lock_classes;
    if (depth && !sync) {
// we're holding locks and the new held lock is not a sync
    hlock = curr.held_locks + depth - 1;
    if (hlock.class_idx == class_idx && nest_lock) {
    if (!references) {
    references += 1;
    }
    if (!hlock.references) {
    hlock.references += 1;
    }
    hlock.references += references;
// Overflow
    if (DEBUG_LOCKS_WARN_ON(hlock.references < references)) {
    return 0;
    }
    return 2;
    }
    }
    hlock = curr.held_locks + depth;
//
// Plain impossible, we just registered it and checked it weren't no
// NULL like.. I bet this mushroom I ate was good!
//
    if (DEBUG_LOCKS_WARN_ON(!class)) {
    return 0;
    }
    hlock.class_idx = class_idx;
    hlock.acquire_ip = ip;
    hlock.instance = lock;
    hlock.nest_lock = nest_lock;
    hlock.irq_context = task_irq_context(curr);
    hlock.trylock = trylock;
    hlock.read = read;
    hlock.check = check;
    hlock.sync = !!sync;
    hlock.hardirqs_off = !!hardirqs_off;
    hlock.references = references;

    hlock.waittime_stamp = 0;
    hlock.holdtime_stamp = lockstat_clock();

    hlock.pin_count = pin_count;
    hlock.seq_count = seq;
    if (check_wait_context(curr, hlock)) {
    return 0;
    }
// Initialize the lock usage bit
    if (!mark_usage(curr, hlock, check)) {
    return 0;
    }
//
// Calculate the chain hash: it's the combined hash of all the
// lock keys along the dependency chain. We save the hash value
// at every step so that we can get the current hash easily
// after unlock. The chain hash is then used to cache dependency
// results.
//
// The 'key ID' is what is the most compact key value to drive
// the hash, not class->key.
//
// Whoops, we did it again.. class_idx is invalid.
//
    if (DEBUG_LOCKS_WARN_ON(!test_bit(class_idx, lock_classes_in_use))) {
    return 0;
    }
    chain_key = curr.curr_chain_key;
    if (!depth) {
//
// How can we have a chain hash when we ain't got no keys?!
//
    if (DEBUG_LOCKS_WARN_ON(chain_key != INITIAL_CHAIN_KEY)) {
    return 0;
    }
    chain_head = 1;
    }
    hlock.prev_chain_key = chain_key;
    if (separate_irq_context(curr, hlock)) {
    chain_key = INITIAL_CHAIN_KEY;
    chain_head = 1;
    }
    chain_key = iterate_chain_key(chain_key, hlock_id(hlock));
    if (nest_lock && !__lock_is_held(nest_lock, -1)) {
    print_lock_nested_lock_not_held(curr, hlock);
    return 0;
    }
    if (!debug_locks_silent) {
    WARN_ON_ONCE!(depth && !hlock_class(hlock - 1).key);
    WARN_ON_ONCE!(!hlock_class(hlock).key);
    }
    if (!validate_chain(curr, hlock, chain_head, chain_key)) {
    return 0;
    }
// For lock_sync(), we are done here since no actual critical section
    if (hlock.sync) {
    return 1;
    }
    curr.curr_chain_key = chain_key;
    curr.lockdep_depth += 1;
    check_chain_key(curr);

    if (unlikely(!debug_locks)) {
    return 0;
    }

    if (unlikely(curr.lockdep_depth >= MAX_LOCK_DEPTH)) {
    debug_locks_off();
    nbcon_cpu_emergency_enter();
    print_lockdep_off("BUG: MAX_LOCK_DEPTH too low!");
    printk("depth: %i  max: %lu!\n",
    curr.lockdep_depth, MAX_LOCK_DEPTH);
    lockdep_print_held_locks(current);
    debug_show_all_locks();
    dump_stack();
    nbcon_cpu_emergency_exit();
    return 0;
    }
    if (unlikely(curr.lockdep_depth > max_lockdep_depth)) {
    max_lockdep_depth = curr.lockdep_depth;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn print_unlock_imbalance_bug(curr: *mut task_struct, lock: *mut lockdep_map, ip: c_ulong) {
    if (!debug_locks_off()) {
    return;
    }
    if (debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=====================================\n");
    pr_warn!("WARNING: bad unlock balance detected!\n");
    print_kernel_ident();
    pr_warn!("-------------------------------------\n");
    pr_warn!("%s/%d is trying to release lock (",
    curr.comm, task_pid_nr(curr));
    print_lockdep_cache(lock);
    pr_cont(") at:\n");
    print_ip_sym(KERN_WARNING, ip);
    pr_warn!("but there are no more locks to release!\n");
    pr_warn!("\nother info that might help us debug this:\n");
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
    static noinstr int match_held_lock(const struct held_lock *hlock,
    const struct lockdep_map *lock)
    {
    if (hlock.instance == lock) {
    return 1;
    }
    if (hlock.references) {
    let mut class = lock.class_cache[0];
    if (!class) {
    class = look_up_lock_class(lock, 0);
    }
//
// If look_up_lock_class() failed to find a class, we're trying
// to test if we hold a lock that has never yet been acquired.
// Clearly if the lock hasn't been acquired _ever_, we're not
// holding it either, so report failure.
//
    if (!class) {
    return 0;
    }
//
// References, but not a lock we're actually ref-counting?
// State got messed up, follow the sites that change ->references
// and try to make sense of it.
//
    if (DEBUG_LOCKS_WARN_ON(!hlock.nest_lock)) {
    return 0;
    }
    if (hlock.class_idx == class - lock_classes) {
    return 1;
    }
    }
    return 0;
    }
// @depth must not be zero
#[no_mangle]
pub unsafe extern "C" fn find_held_lock(curr: *mut task_struct, lock: *mut lockdep_map, depth: c_uint, idx: *mut c_int) -> *mut c_void {
    let mut ret = core::ptr::null_mut();
    let mut hlock = core::ptr::null_mut();
    let mut prev_hlock = core::ptr::null_mut();
    let mut i = 0;
    i = depth - 1;
    hlock = curr.held_locks + i;
    ret = hlock;
    if (match_held_lock(hlock, lock)) {
// goto;
    }
    ret = core::ptr::null_mut();
    while (i >= 0) {
//
// We must not cross into another context:
//
    if (prev_hlock.irq_context != hlock.irq_context) {
    ret = core::ptr::null_mut();
    break;
    }
    if (match_held_lock(hlock, lock)) {
    ret = hlock;
    break;
    }
    }
// label;
// idx = i;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn reacquire_held_locks(curr: *mut task_struct, depth: c_uint, idx: c_int, merged: *mut c_uint) -> c_int {
pub static mut hlock: *mut c_void = core::ptr::null_mut();
pub static mut first_idx: c_int = 0;
    if (DEBUG_LOCKS_WARN_ON(!irqs_disabled())) {
    return 0;
    }
    while (idx < depth) {
    switch (__lock_acquire(hlock.instance,
    hlock_class(hlock).subclass,
    hlock.trylock,
    hlock.read, hlock.check,
    hlock.hardirqs_off,
    hlock.nest_lock, hlock.acquire_ip,
    hlock.references, hlock.pin_count, 0, hlock.seq_count)) {
    case 0:
    return 1;
    case 1:
    break;
    case 2:
// merged += (idx == first_idx);
    break;
// label;
    WARN_ON!(1);
    return 0;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __lock_set_class(lock: *mut lockdep_map, name: *mut c_char, key: *mut lock_class_key, subclass: c_uint, ip: c_ulong) -> c_int {
    let mut curr = current;
    unsigned int depth, merged = 0;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return 0;
    }
    depth = curr.lockdep_depth;
//
// This function is about (re)setting the class of a held lock,
// yet we're not actually holding any locks. Naughty user!
//
    if (DEBUG_LOCKS_WARN_ON(!depth)) {
    return 0;
    }
    hlock = find_held_lock(curr, lock, depth, &i);
    if (!hlock) {
    print_unlock_imbalance_bug(curr, lock, ip);
    return 0;
    }
    lockdep_init_map_type(lock, name, key, 0,
    lock.wait_type_inner,
    lock.wait_type_outer,
    lock.lock_type);
    class = register_lock_class(lock, subclass, 0);
    if (!class) {
    return 0;
    }
    hlock.class_idx = class - lock_classes;
    curr.lockdep_depth = i;
    curr.curr_chain_key = hlock.prev_chain_key;
    if (reacquire_held_locks(curr, depth, i, &merged)) {
    return 0;
    }
//
// I took it apart and put it back together again, except now I have
// these 'spare' parts.. where shall I put them.
//
    if (DEBUG_LOCKS_WARN_ON(curr.lockdep_depth != depth - merged)) {
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn __lock_downgrade(lock: *mut lockdep_map, ip: c_ulong) -> c_int {
    let mut curr = current;
    unsigned int depth, merged = 0;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return 0;
    }
    depth = curr.lockdep_depth;
//
// This function is about (re)setting the class of a held lock,
// yet we're not actually holding any locks. Naughty user!
//
    if (DEBUG_LOCKS_WARN_ON(!depth)) {
    return 0;
    }
    hlock = find_held_lock(curr, lock, depth, &i);
    if (!hlock) {
    print_unlock_imbalance_bug(curr, lock, ip);
    return 0;
    }
    curr.lockdep_depth = i;
    curr.curr_chain_key = hlock.prev_chain_key;
    WARN(hlock.read, "downgrading a read lock");
    hlock.read = 1;
    hlock.acquire_ip = ip;
    if (reacquire_held_locks(curr, depth, i, &merged)) {
    return 0;
    }
// Merging can't happen with unchanged classes..
    if (DEBUG_LOCKS_WARN_ON(merged)) {
    return 0;
    }
//
// I took it apart and put it back together again, except now I have
// these 'spare' parts.. where shall I put them.
//
    if (DEBUG_LOCKS_WARN_ON(curr.lockdep_depth != depth)) {
    return 0;
    }
    return 1;
    }
//
// Remove the lock from the list of currently held locks - this gets
// called on mutex_unlock()/spin_unlock*() (or on a failed
// mutex_lock_interruptible()).
//
#[no_mangle]
pub unsafe extern "C" fn __lock_release(lock: *mut lockdep_map, ip: c_ulong) -> c_int {
    let mut curr = current;
    unsigned int depth, merged = 1;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return 0;
    }
    depth = curr.lockdep_depth;
//
// So we're all set to release this lock.. wait what lock? We don't
// own any locks, you've been drinking again?
//
    if (depth <= 0) {
    print_unlock_imbalance_bug(curr, lock, ip);
    return 0;
    }
//
// Check whether the lock exists in the current stack
// of held locks:
//
    hlock = find_held_lock(curr, lock, depth, &i);
    if (!hlock) {
    print_unlock_imbalance_bug(curr, lock, ip);
    return 0;
    }
    if (hlock.instance == lock) {
    lock_release_holdtime(hlock);
    }
    WARN(hlock.pin_count, "releasing a pinned lock\n");
    if (hlock.references) {
    hlock.references -= 1;
    if (hlock.references) {
//
// We had, and after removing one, still have
// references, the current lock stack is still
// valid. We're done!
//
    return 1;
    }
    }
//
// We have the right lock to unlock, 'hlock' points to it.
// Now we remove it from the stack, and add back the other
// entries (if any), recalculating the hash along the way:
//
    curr.lockdep_depth = i;
    curr.curr_chain_key = hlock.prev_chain_key;
//
// The most likely case is when the unlock is on the innermost
// lock. In this case, we are done!
//
    if (i == depth-1) {
    return 1;
    }
    if (reacquire_held_locks(curr, depth, i + 1, &merged)) {
    return 0;
    }
//
// We had N bottles of beer on the wall, we drank one, but now
// there's not N-1 bottles of beer left on the wall...
// Pouring two of the bottles together is acceptable.
//
    DEBUG_LOCKS_WARN_ON(curr.lockdep_depth != depth - merged);
//
// Since reacquire_held_locks() would have called check_chain_key()
// indirectly via __lock_acquire(), we don't need to do it again
// on return.
//
    return 0;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn __lock_is_held(lock: *const lockdep_map, read: c_int) -> c_int {
    let mut curr = current;
    let mut i = 0;
    while (i < curr.lockdep_depth) {
    let mut hlock = curr.held_locks + i;
    if (match_held_lock(hlock, lock)) {
    if (read == -1 || !!hlock.read == read) {
    return LOCK_STATE_HELD;
    }
    return LOCK_STATE_NOT_HELD;
    }
    }
    return LOCK_STATE_NOT_HELD;
    }
#[no_mangle]
unsafe extern "C" fn __lock_pin_lock(lock: *mut lockdep_map) -> pin_cookie {
pub static mut cookie: pin_cookie = 0;
    let mut curr = current;
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return cookie;
    }
    while (i < curr.lockdep_depth) {
    let mut hlock = curr.held_locks + i;
    if (match_held_lock(hlock, lock)) {
//
// Grab 16bits of randomness; this is sufficient to not
// be guessable and still allows some pin nesting in
// our u32 pin_count.
//
    cookie.val = 1 + (sched_clock() & 0xffff);
    hlock.pin_count += cookie.val;
    return cookie;
    }
    }
    WARN(1, "pinning an unheld lock\n");
    return cookie;
    }
#[no_mangle]
unsafe extern "C" fn __lock_repin_lock(lock: *mut lockdep_map, cookie: pin_cookie) {
    let mut curr = current;
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return;
    }
    while (i < curr.lockdep_depth) {
    let mut hlock = curr.held_locks + i;
    if (match_held_lock(hlock, lock)) {
    hlock.pin_count += cookie.val;
    return;
    }
    }
    WARN(1, "pinning an unheld lock\n");
    }
#[no_mangle]
unsafe extern "C" fn __lock_unpin_lock(lock: *mut lockdep_map, cookie: pin_cookie) {
    let mut curr = current;
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return;
    }
    while (i < curr.lockdep_depth) {
    let mut hlock = curr.held_locks + i;
    if (match_held_lock(hlock, lock)) {
    let mut pin_count = 0;
    if (WARN(!hlock.pin_count, "unpinning an unpinned lock\n")) {
    return;
    }
    pin_count = hlock.pin_count - cookie.val;
    if (WARN(pin_count < 0, "pin count corrupted\n")) {
    pin_count = 0;
    }
    hlock.pin_count = pin_count;
    return;
    }
    }
    WARN(1, "unpinning an unheld lock\n");
    }
#[no_mangle]
unsafe extern "C" fn __lock_sequence(lock: *mut lockdep_map) -> u32 {
    let mut curr = current;
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return ~0;
    }
    while (i < curr.lockdep_depth) {
    let mut hlock = curr.held_locks + i;
    if (match_held_lock(hlock, lock)) {
    return hlock.seq_count;
    }
    }
    return ~0;
    }
//
// Check whether we follow the irq-flags state precisely:
//
#[no_mangle]
unsafe extern "C" fn check_flags(flags: c_ulong) -> noinstr void {

    if (!debug_locks) {
    return;
    }
// Get the warning out..
    instrumentation_begin();
    if (irqs_disabled_flags(flags)) {
    if (DEBUG_LOCKS_WARN_ON(lockdep_hardirqs_enabled())) {
    printk("possible reason: unannotated irqs-off.\n");
    }
    } else {
    if (DEBUG_LOCKS_WARN_ON(!lockdep_hardirqs_enabled())) {
    printk("possible reason: unannotated irqs-on.\n");
    }
    }

//
// We dont accurately track softirq state in e.g.
// hardirq contexts (such as on 4KSTACKS), so only
// check if not in hardirq contexts:
//
    if (!hardirq_count()) {
    if (softirq_count()) {
// like the above, but with softirqs
    DEBUG_LOCKS_WARN_ON(current.softirqs_enabled);
    } else {
// lick the above, does it taste good?
    DEBUG_LOCKS_WARN_ON(!current.softirqs_enabled);
    }
    }

    if (!debug_locks) {
    print_irqtrace_events(current);
    }
    instrumentation_end();

    }
#[no_mangle]
pub unsafe extern "C" fn lock_set_class(lock: *mut lockdep_map, name: *mut c_char, key: *mut lock_class_key, subclass: c_uint, ip: c_ulong) {
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    lockdep_recursion_inc();
    check_flags(flags);
    if (__lock_set_class(lock, name, key, subclass, ip)) {
    check_chain_key(current);
    }
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_set_class);
#[no_mangle]
pub unsafe extern "C" fn lock_downgrade(lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    lockdep_recursion_inc();
    check_flags(flags);
    if (__lock_downgrade(lock, ip)) {
    check_chain_key(current);
    }
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_downgrade);
// NMI context !!!
#[no_mangle]
unsafe extern "C" fn verify_lock_unused(lock: *mut lockdep_map, hlock: *mut held_lock, subclass: c_int) {

    let mut class = look_up_lock_class(lock, subclass);
pub static mut mask: c_ulong = 0;
// if it doesn't have a class (yet), it certainly hasn't been used yet
    if (!class) {
    return;
    }
//
// READ locks only conflict with USED, such that if we only ever use
// READ locks, there is no deadlock possible -- RCU.
//
    if (!hlock.read) {
    mask |= LOCKF_USED_READ;
    }
    if (!(class.usage_mask & mask)) {
    return;
    }
    hlock.class_idx = class - lock_classes;
    print_usage_bug(current, hlock, LOCK_USED, LOCK_USAGE_STATES);

    }
#[no_mangle]
unsafe extern "C" fn lockdep_nmi() -> bool {
    if (raw_cpu_read(lockdep_recursion)) {
    return false;
    }
    if (!in_nmi()) {
    return false;
    }
    return true;
    }
//
// read_lock() is recursive if:
// 1. We force lockdep think this way in selftests or
// 2. The implementation is not queued read/write lock or
// 3. The locker is at an in_interrupt() context.
//
#[no_mangle]
pub unsafe extern "C" fn read_lock_is_recursive() -> bool {
    return force_read_lock_recursive ||
    !IS_ENABLED!(CONFIG_QUEUED_RWLOCKS) ||
    in_interrupt();
    }
    EXPORT_SYMBOL_GPL(read_lock_is_recursive);
//
// We are not always called with irqs disabled - do that here,
// and also avoid lockdep recursion:
//
#[no_mangle]
pub unsafe extern "C" fn lock_acquire(lock: *mut lockdep_map, subclass: c_uint, trylock: c_int, read: c_int, check: c_int, nest_lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    trace_lock_acquire(lock, subclass, trylock, read, check, nest_lock, ip);
    if (!debug_locks) {
    return;
    }
//
// As KASAN instrumentation is disabled and lock_acquire() is usually
// the first lockdep call when a task tries to acquire a lock, add
// kasan_check_byte() here to check for use-after-free and other
// memory errors.
//
    kasan_check_byte(lock);
    if (unlikely(!lockdep_enabled())) {
// XXX allow trylock from NMI ?!?
    if (lockdep_nmi() && !trylock) {
pub static mut hlock: usize = 0;
    hlock.acquire_ip = ip;
    hlock.instance = lock;
    hlock.nest_lock = nest_lock;
    hlock.irq_context = 2; // XXX
    hlock.trylock = trylock;
    hlock.read = read;
    hlock.check = check;
    hlock.hardirqs_off = true;
    hlock.references = 0;
    verify_lock_unused(lock, &hlock, subclass);
    }
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_acquire(lock, subclass, trylock, read, check,
    irqs_disabled_flags(flags), nest_lock, ip, 0, 0, 0,
    ++current.lockdep_seq);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_acquire);
#[no_mangle]
pub unsafe extern "C" fn lock_release(lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    trace_lock_release(lock, ip);
    if (unlikely(!lockdep_enabled() ||
    lock.key == &__lockdep_no_track__)) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    if (__lock_release(lock, ip)) {
    check_chain_key(current);
    }
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_release);
//
// lock_sync() - A special annotation for synchronize_{s,}rcu()-like API.
//
// No actual critical section is created by the APIs annotated with this: these
// APIs are used to wait for one or multiple critical sections (on other CPUs
// or threads), and it means that calling these APIs inside these critical
// sections is potential deadlock.
//
#[no_mangle]
pub unsafe extern "C" fn lock_sync(lock: *mut lockdep_map, subclass: c_uint, read: c_int, check: c_int, nest_lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_acquire(lock, subclass, 0, read, check,
    irqs_disabled_flags(flags), nest_lock, ip, 0, 0, 1,
    ++current.lockdep_seq);
    check_chain_key(current);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_sync);
#[no_mangle]
pub unsafe extern "C" fn lock_is_held_type(lock: *const lockdep_map, read: c_int) -> noinstr int {
    let mut flags = 0;
pub static mut ret: c_int = 0;
//
// Avoid false negative lockdep_assert_held() and
// lockdep_assert_not_held().
//
    if (unlikely(!lockdep_enabled())) {
    return LOCK_STATE_UNKNOWN;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    ret = __lock_is_held(lock, read);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(lock_is_held_type);
    NOKPROBE_SYMBOL(lock_is_held_type);
#[no_mangle]
pub unsafe extern "C" fn lock_pin_lock(lock: *mut lockdep_map) -> pin_cookie {
pub static mut cookie: pin_cookie = 0;
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return cookie;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    cookie = __lock_pin_lock(lock);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    return cookie;
    }
    EXPORT_SYMBOL_GPL(lock_pin_lock);
#[no_mangle]
pub unsafe extern "C" fn lock_repin_lock(lock: *mut lockdep_map, cookie: pin_cookie) {
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_repin_lock(lock, cookie);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_repin_lock);
#[no_mangle]
pub unsafe extern "C" fn lock_unpin_lock(lock: *mut lockdep_map, cookie: pin_cookie) {
    let mut flags = 0;
    if (unlikely(!lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_unpin_lock(lock, cookie);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_unpin_lock);
#[no_mangle]
pub unsafe extern "C" fn lock_sequence(lock: *mut lockdep_map) -> u32 {
    let mut flags = 0;
pub static mut seq: u32 = 0;
    if (unlikely(!lockdep_enabled())) {
    return seq;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    seq = __lock_sequence(lock);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    return seq;
    }
    EXPORT_SYMBOL_GPL(lock_sequence);

#[no_mangle]
pub unsafe extern "C" fn print_lock_contention_bug(curr: *mut task_struct, lock: *mut lockdep_map, ip: c_ulong) {
    if (!debug_locks_off()) {
    return;
    }
    if (debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=================================\n");
    pr_warn!("WARNING: bad contention detected!\n");
    print_kernel_ident();
    pr_warn!("---------------------------------\n");
    pr_warn!("%s/%d is trying to contend lock (",
    curr.comm, task_pid_nr(curr));
    print_lockdep_cache(lock);
    pr_cont(") at:\n");
    print_ip_sym(KERN_WARNING, ip);
    pr_warn!("but there are no locks held!\n");
    pr_warn!("\nother info that might help us debug this:\n");
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn __lock_contended(lock: *mut lockdep_map, ip: c_ulong) {
    let mut curr = current;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut depth = 0;
    let mut i = 0;
    let mut contention_point = 0;
    let mut contending_point = 0;
    depth = curr.lockdep_depth;
//
// Whee, we contended on this lock, except it seems we're not
// actually trying to acquire anything much at all..
//
    if (DEBUG_LOCKS_WARN_ON(!depth)) {
    return;
    }
    if (unlikely(lock.key == &__lockdep_no_track__)) {
    return;
    }
    hlock = find_held_lock(curr, lock, depth, &i);
    if (!hlock) {
    print_lock_contention_bug(curr, lock, ip);
    return;
    }
    if (hlock.instance != lock) {
    return;
    }
    hlock.waittime_stamp = lockstat_clock();
    contention_point = lock_point(hlock_class(hlock).contention_point, ip);
    contending_point = lock_point(hlock_class(hlock).contending_point,
    lock.ip);
    stats = get_lock_stats(hlock_class(hlock));
    if (contention_point < LOCKSTAT_POINTS) {
    stats.contention_point[contention_point]++;
    }
    if (contending_point < LOCKSTAT_POINTS) {
    stats.contending_point[contending_point]++;
    }
    if (lock.cpu != smp_processor_id()) {
    stats.bounces[bounce_contended + !!hlock.read]++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __lock_acquired(lock: *mut lockdep_map, ip: c_ulong) {
    let mut curr = current;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
pub static mut stats: *mut c_void = core::ptr::null_mut();
    let mut depth = 0;
    u64 now, waittime = 0;
    let mut i = 0;
    let mut cpu = 0;
    depth = curr.lockdep_depth;
//
// Yay, we acquired ownership of this lock we didn't try to
// acquire, how the heck did that happen?
//
    if (DEBUG_LOCKS_WARN_ON(!depth)) {
    return;
    }
    if (unlikely(lock.key == &__lockdep_no_track__)) {
    return;
    }
    hlock = find_held_lock(curr, lock, depth, &i);
    if (!hlock) {
    print_lock_contention_bug(curr, lock, _RET_IP_);
    return;
    }
    if (hlock.instance != lock) {
    return;
    }
    cpu = smp_processor_id();
    if (hlock.waittime_stamp) {
    now = lockstat_clock();
    waittime = now - hlock.waittime_stamp;
    hlock.holdtime_stamp = now;
    }
    stats = get_lock_stats(hlock_class(hlock));
    if (waittime) {
    if (hlock.read) {
    lock_time_inc(&stats.read_waittime, waittime);
    }
    else {
    lock_time_inc(&stats.write_waittime, waittime);
    }
    }
    if (lock.cpu != cpu) {
    stats.bounces[bounce_acquired + !!hlock.read]++;
    }
    lock.cpu = cpu;
    lock.ip = ip;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_contended(lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    trace_lock_contended(lock, ip);
    if (unlikely(!lock_stat || !lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_contended(lock, ip);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_contended);
#[no_mangle]
pub unsafe extern "C" fn lock_acquired(lock: *mut lockdep_map, ip: c_ulong) {
    let mut flags = 0;
    trace_lock_acquired(lock, ip);
    if (unlikely(!lock_stat || !lockdep_enabled())) {
    return;
    }
    raw_local_irq_save(flags);
    check_flags(flags);
    lockdep_recursion_inc();
    __lock_acquired(lock, ip);
    lockdep_recursion_finish();
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(lock_acquired);

//
// Used by the testsuite, sanitize the validator state
// after a simulated failure:
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_reset() {
    let mut flags = 0;
    let mut i = 0;
    raw_local_irq_save(flags);
    lockdep_init_task(current);
    memset(current.held_locks, 0, MAX_LOCK_DEPTH*sizeof!(held_lock));
    nr_hardirq_chains = 0;
    nr_softirq_chains = 0;
    nr_process_chains = 0;
    debug_locks = 1;
    for (i = 0; i < CHAINHASH_SIZE; i++) {
    INIT_HLIST_HEAD(chainhash_table + i);
    }
    raw_local_irq_restore(flags);
    }
// Remove a class from a lock chain. Must be called with the graph lock held.
#[no_mangle]
pub unsafe extern "C" fn remove_class_from_lock_chain(pf: *mut pending_free, chain: *mut lock_chain, class: *mut lock_class) {

    let mut i = 0;
    while (i < chain.base + chain.depth) {
    if (chain_hlock_class_idx(chain_hlocks[i]) != class - lock_classes) {
    continue;
    }
//
// Each lock class occurs at most once in a lock chain so once
// we found a match we can break out of this loop.
//
// goto;
    }
// Since the chain has not been modified, return.
    return;
// label;
    free_chain_hlocks(chain.base, chain.depth);
// Overwrite the chain key for concurrent RCU readers.
    WRITE_ONCE(chain.chain_key, INITIAL_CHAIN_KEY);
    dec_chains(chain.irq_context);
//
// Note: calling hlist_del_rcu() from inside a
// hlist_for_each_entry_rcu() loop is safe.
//
    hlist_del_rcu(&chain.entry);
    __set_bit(chain - lock_chains, pf.lock_chains_being_freed);
    nr_zapped_lock_chains += 1;

    }
// Must be called with the graph lock held.
#[no_mangle]
pub unsafe extern "C" fn remove_class_from_lock_chains(pf: *mut pending_free, class: *mut lock_class) {
pub static mut chain: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < ARRAY_SIZE!(chainhash_table)) {
    head = chainhash_table + i;
    hlist_for_each_entry_rcu(chain, head, entry) {
    remove_class_from_lock_chain(pf, chain, class);
    }
    }
    }
//
// Remove all references to a lock class. The caller must hold the graph lock.
//
#[no_mangle]
unsafe extern "C" fn zap_class(pf: *mut pending_free, class: *mut lock_class) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    WARN_ON_ONCE!(!class.key);
//
// Remove all dependencies this lock is
// involved in:
//
    for_each_set_bit(i, list_entries_in_use, ARRAY_SIZE!(list_entries)) {
    entry = list_entries + i;
    if (entry.class != class && entry.links_to != class) {
    continue;
    }
    __clear_bit(i, list_entries_in_use);
    nr_list_entries -= 1;
    list_del_rcu(&entry.entry);
    }
    if (list_empty(&class.locks_after) &&
    list_empty(&class.locks_before)) {
    list_move_tail(&class.lock_entry, &pf.zapped);
    hlist_del_rcu(&class.hash_entry);
    WRITE_ONCE(class.key, core::ptr::null_mut());
    WRITE_ONCE(class.name, core::ptr::null_mut());
// Class allocated but not used, -1 in nr_unused_locks
    if (class.usage_mask == 0) {
    debug_atomic_dec(nr_unused_locks);
    }
    nr_lock_classes -= 1;
    __clear_bit(class - lock_classes, lock_classes_in_use);
    if (class - lock_classes == max_lock_class_idx) {
    max_lock_class_idx -= 1;
    }
    } else {
    WARN_ONCE(true, "%s() failed for class %s\n", __func__,
    class.name);
    }
    remove_class_from_lock_chains(pf, class);
    nr_zapped_classes += 1;
    }
#[no_mangle]
unsafe extern "C" fn reinit_class(class: *mut lock_class) {
    WARN_ON_ONCE!(!class.lock_entry.next);
    WARN_ON_ONCE!(!list_empty(&class.locks_after));
    WARN_ON_ONCE!(!list_empty(&class.locks_before));
    memset_startat(class, 0, key);
    WARN_ON_ONCE!(!class.lock_entry.next);
    WARN_ON_ONCE!(!list_empty(&class.locks_after));
    WARN_ON_ONCE!(!list_empty(&class.locks_before));
    }
#[no_mangle]
pub unsafe extern "C" fn within(addr: *const c_void, start: *mut c_void, size: c_ulong) -> c_int {
    return addr >= start && addr < start + size;
    }
#[no_mangle]
unsafe extern "C" fn inside_selftest() -> bool {
pub static mut current: return = 0;
    }
// The caller must hold the graph lock.
#[no_mangle]
pub unsafe extern "C" fn get_pending_free() -> *mut c_void {
    return delayed_free.pf + delayed_free.index;
    }
// forward_decl: free_zapped_rcu;
//
// See if we need to queue an RCU callback, must called with
// the lockdep lock held, returns false if either we don't have
// any pending free or the callback is already scheduled.
// Otherwise, a call_rcu() must follow this function call.
//
#[no_mangle]
unsafe extern "C" fn prepare_call_rcu_zapped(pf: *mut pending_free) -> bool {
    WARN_ON_ONCE!(inside_selftest());
    if (list_empty(&pf.zapped)) {
    return false;
    }
    if (delayed_free.scheduled) {
    return false;
    }
    delayed_free.scheduled = true;
    WARN_ON_ONCE!(delayed_free.pf + delayed_free.index != pf);
    delayed_free.index ^= 1;
    return true;
    }
// The caller must hold the graph lock. May be called from RCU context.
#[no_mangle]
unsafe extern "C" fn __free_zapped_classes(pf: *mut pending_free) {
pub static mut class: *mut c_void = core::ptr::null_mut();
    check_data_structures();
    list_for_each_entry(class, &pf.zapped, lock_entry) {
    reinit_class(class);
    }
    list_splice_init(&pf.zapped, &free_lock_classes);

    bitmap_andnot(lock_chains_in_use, lock_chains_in_use,
    pf.lock_chains_being_freed, ARRAY_SIZE!(lock_chains));
    bitmap_clear(pf.lock_chains_being_freed, 0, ARRAY_SIZE!(lock_chains));

    }
#[no_mangle]
unsafe extern "C" fn free_zapped_rcu(ch: *mut rcu_head) {
pub static mut pf: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut need_callback = 0;
    if (WARN_ON_ONCE!(ch != &delayed_free.rcu_head)) {
    return;
    }
    raw_local_irq_save(flags);
    lockdep_lock();
// closed head
    pf = delayed_free.pf + (delayed_free.index ^ 1);
    __free_zapped_classes(pf);
    delayed_free.scheduled = false;
    need_callback =
    prepare_call_rcu_zapped(delayed_free.pf + delayed_free.index);
    lockdep_unlock();
    raw_local_irq_restore(flags);
//
// If there's pending free and its callback has not been scheduled,
// queue an RCU callback.
//
    if (need_callback) {
    call_rcu(&delayed_free.rcu_head, free_zapped_rcu);
    }
    }
//
// Remove all lock classes from the class hash table and from the
// all_lock_classes list whose key or name is in the address range [start,
// start + size). Move these lock classes to the zapped_classes list. Must
// be called with the graph lock held.
//
#[no_mangle]
pub unsafe extern "C" fn __lockdep_free_key_range(pf: *mut pending_free, start: *mut c_void, size: c_ulong) {
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Unhash all classes that were created by a module.
    while (i < CLASSHASH_SIZE) {
    head = classhash_table + i;
    hlist_for_each_entry_rcu(class, head, hash_entry) {
    if (!within(class.key, start, size) &&
    !within(class.name, start, size)) {
    continue;
    }
    zap_class(pf, class);
    }
    }
    }
//
// Used in module.c to remove lock classes from memory that is going to be
// freed; and possibly re-used by other modules.
//
// We will have had one synchronize_rcu() before getting here, so we're
// guaranteed nobody will look up these exact classes -- they're properly dead
// but still allocated.
//
#[no_mangle]
unsafe extern "C" fn lockdep_free_key_range_reg(start: *mut c_void, size: c_ulong) {
pub static mut pf: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut need_callback = 0;
    init_data_structures_once();
    raw_local_irq_save(flags);
    lockdep_lock();
    pf = get_pending_free();
    __lockdep_free_key_range(pf, start, size);
    need_callback = prepare_call_rcu_zapped(pf);
    lockdep_unlock();
    raw_local_irq_restore(flags);
    if (need_callback) {
    call_rcu(&delayed_free.rcu_head, free_zapped_rcu);
    }
//
// Wait for any possible iterators from look_up_lock_class() to pass
// before continuing to free the memory they refer to.
//
    synchronize_rcu();
    }
//
// Free all lockdep keys in the range [start, start+size). Does not sleep.
// Ignores debug_locks. Must only be used by the lockdep selftests.
//
#[no_mangle]
unsafe extern "C" fn lockdep_free_key_range_imm(start: *mut c_void, size: c_ulong) {
    let mut pf = delayed_free.pf;
    let mut flags = 0;
    init_data_structures_once();
    raw_local_irq_save(flags);
    lockdep_lock();
    __lockdep_free_key_range(pf, start, size);
    __free_zapped_classes(pf);
    lockdep_unlock();
    raw_local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_free_key_range(start: *mut c_void, size: c_ulong) {
    init_data_structures_once();
    if (inside_selftest()) {
    lockdep_free_key_range_imm(start, size);
    }
    else {
    lockdep_free_key_range_reg(start, size);
    }
    }
//
// Check whether any element of the @lock->class_cache[] array refers to a
// registered lock class. The caller must hold either the graph lock or the
// RCU read lock.
//
#[no_mangle]
unsafe extern "C" fn lock_class_cache_is_registered(lock: *mut lockdep_map) -> bool {
pub static mut class: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    while (i < CLASSHASH_SIZE) {
    head = classhash_table + i;
    hlist_for_each_entry_rcu(class, head, hash_entry) {
    for (j = 0; j < NR_LOCKDEP_CACHING_CLASSES; j++) {
    if (lock.class_cache[j] == class)
    return true;
    }
    }
    }
    return false;
    }
// The caller must hold the graph lock. Does not sleep.
#[no_mangle]
pub unsafe extern "C" fn __lockdep_reset_lock(pf: *mut pending_free, lock: *mut lockdep_map) {
pub static mut class: *mut c_void = core::ptr::null_mut();
    let mut j = 0;
//
// Remove all classes this lock might have:
//
    while (j < MAX_LOCKDEP_SUBCLASSES) {
//
// If the class exists we look it up and zap it:
//
    class = look_up_lock_class(lock, j);
    if (class) {
    zap_class(pf, class);
    }
    }
//
// Debug check: in the end all mapped classes should
// be gone.
//
    if (WARN_ON_ONCE!(lock_class_cache_is_registered(lock))) {
    debug_locks_off();
    }
    }
//
// Remove all information lockdep has about a lock if debug_locks == 1. Free
// released data structures from RCU context.
//
#[no_mangle]
unsafe extern "C" fn lockdep_reset_lock_reg(lock: *mut lockdep_map) {
pub static mut pf: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut locked = 0;
pub static mut need_callback: bool = false;
    raw_local_irq_save(flags);
    locked = graph_lock();
    if (!locked) {
// goto;
    }
    pf = get_pending_free();
    __lockdep_reset_lock(pf, lock);
    need_callback = prepare_call_rcu_zapped(pf);
    graph_unlock();
// label;
    raw_local_irq_restore(flags);
    if (need_callback) {
    call_rcu(&delayed_free.rcu_head, free_zapped_rcu);
    }
    }
//
// Reset a lock. Does not sleep. Ignores debug_locks. Must only be used by the
// lockdep selftests.
//
#[no_mangle]
unsafe extern "C" fn lockdep_reset_lock_imm(lock: *mut lockdep_map) {
    let mut pf = delayed_free.pf;
    let mut flags = 0;
    raw_local_irq_save(flags);
    lockdep_lock();
    __lockdep_reset_lock(pf, lock);
    __free_zapped_classes(pf);
    lockdep_unlock();
    raw_local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_reset_lock(lock: *mut lockdep_map) {
    init_data_structures_once();
    if (inside_selftest()) {
    lockdep_reset_lock_imm(lock);
    }
    else {
    lockdep_reset_lock_reg(lock);
    }
    }
//
// Unregister a dynamically allocated key.
//
// Unlike lockdep_register_key(), a search is always done to find a matching
// key irrespective of debug_locks to avoid potential invalid access to freed
// memory in lock_class entry.
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_unregister_key(key: *mut lock_class_key) {
    let mut hash_head = keyhashentry(key);
pub static mut k: *mut c_void = core::ptr::null_mut();
pub static mut pf: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut found: bool = false;
pub static mut need_callback: bool = false;
    might_sleep();
    if (WARN_ON_ONCE!(static_obj(key))) {
    return;
    }
    raw_local_irq_save(flags);
    lockdep_lock();
    hlist_for_each_entry_rcu(k, hash_head, hash_entry) {
    if (k == key) {
    hlist_del_rcu(&k.hash_entry);
    found = true;
    break;
    }
    }
    WARN_ON_ONCE!(!found && debug_locks);
    if (found) {
    pf = get_pending_free();
    __lockdep_free_key_range(pf, key, 1);
    need_callback = prepare_call_rcu_zapped(pf);
    nr_dynamic_keys -= 1;
    }
    lockdep_unlock();
    raw_local_irq_restore(flags);
    if (need_callback) {
    call_rcu(&delayed_free.rcu_head, free_zapped_rcu);
    }
//
// Wait until is_dynamic_key() has finished accessing k->hash_entry.
//
// Some operations like __qdisc_destroy() will call this in a debug
// kernel, and the network traffic is disabled while waiting, hence
// the delay of the wait matters in debugging cases. Currently use a
// synchronize_rcu_expedited() to speed up the wait at the cost of
// system IPIs. TODO: Replace RCU with hazptr for this.
//
    synchronize_rcu_expedited();
    }
    EXPORT_SYMBOL_GPL(lockdep_unregister_key);
#[no_mangle]
pub unsafe extern "C" fn lockdep_init()  {
    pr_info!("Lock dependency validator: Copyright (c) 2006 Red Hat, Inc., Ingo Molnar\n");
    pr_info!("... MAX_LOCKDEP_SUBCLASSES:  %lu\n", MAX_LOCKDEP_SUBCLASSES);
    pr_info!("... MAX_LOCK_DEPTH:          %lu\n", MAX_LOCK_DEPTH);
    pr_info!("... MAX_LOCKDEP_KEYS:        %lu\n", MAX_LOCKDEP_KEYS);
    pr_info!("... CLASSHASH_SIZE:          %lu\n", CLASSHASH_SIZE);
    pr_info!("... MAX_LOCKDEP_ENTRIES:     %lu\n", MAX_LOCKDEP_ENTRIES);
    pr_info!("... MAX_LOCKDEP_CHAINS:      %lu\n", MAX_LOCKDEP_CHAINS);
    pr_info!("... CHAINHASH_SIZE:          %lu\n", CHAINHASH_SIZE);
    pr_info!(" memory used by lock dependency info: %zu kB\n",
    (sizeof!(lock_classes) +
    sizeof!(lock_classes_in_use) +
    sizeof!(classhash_table) +
    sizeof!(list_entries) +
    sizeof!(list_entries_in_use) +
    sizeof!(chainhash_table) +
    sizeof!(delayed_free)

    + sizeof!(lock_cq)
    + sizeof!(lock_chains)
    + sizeof!(lock_chains_in_use)
    + sizeof!(chain_hlocks)

    ) / 1024
    );

    pr_info!(" memory used for stack traces: %zu kB\n",
    (sizeof!(stack_trace) + sizeof!(stack_trace_hash)) / 1024
    );

    pr_info!(" per task-struct memory footprint: %zu bytes\n",
    sizeof!((core::ptr::null_mut()).held_locks));
    }
#[no_mangle]
pub unsafe extern "C" fn print_freed_lock_bug(curr: *mut task_struct, mem_from: *mut c_void, mem_to: *mut c_void, hlock: *mut held_lock) {
    if (!debug_locks_off()) {
    return;
    }
    if (debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=========================\n");
    pr_warn!("WARNING: held lock freed!\n");
    print_kernel_ident();
    pr_warn!("-------------------------\n");
    pr_warn!("%s/%d is freeing memory %px-%px, with a lock still held there!\n",
    curr.comm, task_pid_nr(curr), mem_from, mem_to-1);
    print_lock(hlock);
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn not_in_range(mem_from: *mut c_void, mem_len: c_ulong, lock_from: *mut c_void, lock_len: c_ulong) -> c_int {
    return lock_from + lock_len <= mem_from ||
    mem_from + mem_len <= lock_from;
    }
//
// Called when kernel memory is freed (or unmapped), or if a lock
// is destroyed or reinitialized - this code checks whether there is
// any held lock in the memory range of <from> to <to>:
//
#[no_mangle]
pub unsafe extern "C" fn debug_check_no_locks_freed(mem_from: *const c_void, mem_len: c_ulong) {
    let mut curr = current;
pub static mut hlock: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut i = 0;
    if (unlikely(!debug_locks)) {
    return;
    }
    raw_local_irq_save(flags);
    while (i < curr.lockdep_depth) {
    hlock = curr.held_locks + i;
    if (not_in_range(mem_from, mem_len, hlock.instance,
    sizeof!(*hlock.instance))) {
    continue;
    }
    print_freed_lock_bug(curr, mem_from, mem_from + mem_len, hlock);
    break;
    }
    raw_local_irq_restore(flags);
    }
    EXPORT_SYMBOL_GPL(debug_check_no_locks_freed);
#[no_mangle]
unsafe extern "C" fn print_held_locks_bug() {
    if (!debug_locks_off()) {
    return;
    }
    if (debug_locks_silent) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("====================================\n");
    pr_warn!("WARNING: %s/%d still has locks held!\n",
    current.comm, task_pid_nr(current));
    print_kernel_ident();
    pr_warn!("------------------------------------\n");
    lockdep_print_held_locks(current);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn debug_check_no_locks_held() {
    if (unlikely(current.lockdep_depth > 0)) {
    print_held_locks_bug();
    }
    }
    EXPORT_SYMBOL_GPL(debug_check_no_locks_held);

#[no_mangle]
pub unsafe extern "C" fn debug_show_all_locks() {
    let mut g = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    if (unlikely(!debug_locks)) {
    pr_warn!("INFO: lockdep is turned off.\n");
    return;
    }
    pr_warn!("\nShowing all locks held in the system:\n");
    rcu_read_lock();
    for_each_process_thread(g, p) {
    if (!p.lockdep_depth) {
    continue;
    }
    lockdep_print_held_locks(p);
    touch_nmi_watchdog();
    touch_all_softlockup_watchdogs();
    }
    rcu_read_unlock();
    pr_warn!("\n");
    pr_warn!("=============================================\n\n");
    }
    EXPORT_SYMBOL_GPL(debug_show_all_locks);

//
// Careful: only use this function if you are sure that
// the task cannot run in parallel!
//
#[no_mangle]
pub unsafe extern "C" fn debug_show_held_locks(task: *mut task_struct) {
    if (unlikely(!debug_locks)) {
    printk("INFO: lockdep is turned off.\n");
    return;
    }
    lockdep_print_held_locks(task);
    }
    EXPORT_SYMBOL_GPL(debug_show_held_locks);
#[no_mangle]
pub unsafe extern "C" fn lockdep_sys_exit() -> asmlinkage __visible void {
    let mut curr = current;
    if (unlikely(curr.lockdep_depth)) {
    if (!debug_locks_off()) {
    return;
    }
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("================================================\n");
    pr_warn!("WARNING: lock held when returning to user space!\n");
    print_kernel_ident();
    pr_warn!("------------------------------------------------\n");
    pr_warn!("%s/%d is leaving the kernel with locks still held!\n",
    curr.comm, curr.pid);
    lockdep_print_held_locks(curr);
    nbcon_cpu_emergency_exit();
    }
//
// The lock history for each syscall should be independent. So wipe the
// slate clean on return to userspace.
//
    lockdep_invariant_state(false);
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_rcu_suspicious(file: *const c_char, line: c_int, s: *const c_char) {
    let mut curr = current;
pub static mut dl: c_int = 0;
pub static mut rcu: bool = false;
// Note: the following can be executed concurrently, so be careful.
    nbcon_cpu_emergency_enter();
    pr_warn!("\n");
    pr_warn!("=============================\n");
    pr_warn!("WARNING: suspicious RCU usage\n");
    print_kernel_ident();
    pr_warn!("-----------------------------\n");
    pr_warn!("%s:%d %s!\n", file, line, s);
    pr_warn!("\nother info that might help us debug this:\n\n");
    pr_warn!("\n%srcu_scheduler_active = %d, debug_locks = %d\n%s",
    !rcu_lockdep_current_cpu_online()
    ? "RCU used illegally from offline CPU!\n"
    : "",
    rcu_scheduler_active, dl,
    dl ? "" : "Possible false positive due to lockdep disabling via debug_locks = 0\n");
//
// If a CPU is in the RCU-free window in idle (ie: in the section
// between ct_idle_enter() and ct_idle_exit(), then RCU
// considers that CPU to be in an "extended quiescent state",
// which means that RCU will be completely ignoring that CPU.
// Therefore, rcu_read_lock() and friends have absolutely no
// effect on a CPU running in that state. In other words, even if
// such an RCU-idle CPU has called rcu_read_lock(), RCU might well
// delete data structures out from under it.  RCU really has no
// choice here: we need to keep an RCU-free window in idle where
// the CPU may possibly enter into low power mode. This way we can
// notice an extended quiescent state to other CPUs that started a grace
// period. Otherwise we would delay any grace period as long as we run
// in the idle task.
//
// So complain bitterly if someone does call rcu_read_lock(),
// rcu_read_lock_bh() and so on from extended quiescent states.
//
    if (!rcu_is_watching()) {
    pr_warn!("RCU used illegally from extended quiescent state!\n");
    }
    lockdep_print_held_locks(curr);
    pr_warn!("\nstack backtrace:\n");
    dump_stack();
    nbcon_cpu_emergency_exit();
    warn_rcu_exit(rcu);
    }
    EXPORT_SYMBOL_GPL(lockdep_rcu_suspicious);