//! Automatically rewritten from C to Rust
//! Source: mm/kasan/quarantine.c
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
// KASAN quarantine.
//
// Author: Alexander Potapenko <glider@google.com>
// Copyright (C) 2016 Google, Inc.
//
// Based on code by Dmitry Chernenkov.
//

// Data structure and operations for quarantine queues.
//
// Each queue is a single-linked list, which also stores the total size of
// objects inside of it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlist_head {
    pub head: *mut qlist_node,
    pub tail: *mut qlist_node,
    pub bytes: usize,
    pub offline: bool,
}

#[no_mangle]
unsafe extern "C" fn qlist_empty(q: *mut qlist_head) -> bool {
    return !q.head;
    }
#[no_mangle]
unsafe extern "C" fn qlist_init(q: *mut qlist_head) {
    q.head = q.tail = core::ptr::null_mut();
    q.bytes = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qlist_put(q: *mut qlist_head, qlink: *mut qlist_node, size: size_t) {
    if (unlikely(qlist_empty(q))) {
    q.head = qlink;
    }
    else {
    q.tail.next = qlink;
    }
    q.tail = qlink;
    qlink.next = core::ptr::null_mut();
    q.bytes += size;
    }
#[no_mangle]
unsafe extern "C" fn qlist_move_all(from: *mut qlist_head, to: *mut qlist_head) {
    if (unlikely(qlist_empty(from))) {
    return;
    }
    if (qlist_empty(to)) {
// to = *from;
    qlist_init(from);
    return;
    }
    to.tail.next = from.head;
    to.tail = from.tail;
    to.bytes += from.bytes;
    qlist_init(from);
    }

    (1024 > 4 * CONFIG_NR_CPUS ? 1024 : 4 * CONFIG_NR_CPUS)
//
// The object quarantine consists of per-cpu queues and a global queue,
// guarded by quarantine_lock.
//
pub static mut struct qlist_head: usize = 0;
// Round-robin FIFO array of batches.
    static struct qlist_head global_quarantine[QUARANTINE_BATCHES];
    static int quarantine_head;
    static int quarantine_tail;
// Total size of all objects in global_quarantine across all batches.
    static unsigned long quarantine_size;
pub static mut quarantine_lock: usize = 0;
pub static mut remove_cache_srcu: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_shrink_qlist {
    pub lock: raw_spinlock_t,
    pub qlist: qlist_head,
}

    static DEFINE_PER_CPU(cpu_shrink_qlist, shrink_qlist) = {
    .lock = __RAW_SPIN_LOCK_UNLOCKED(shrink_qlist.lock),
    };
// Maximum size of the global queue.
    static unsigned long quarantine_max_size;
//
// Target size of a batch in global_quarantine.
// Usually equal to QUARANTINE_PERCPU_SIZE unless we have too much RAM.
//
    static unsigned long quarantine_batch_size;
//
// The fraction of physical memory the quarantine is allowed to occupy.
// Quarantine doesn't support memory shrinker with SLAB allocator, so we keep
// the ratio low to avoid OOM.
//
pub const QUARANTINE_FRACTION: c_int = 32;
#[no_mangle]
pub unsafe extern "C" fn qlink_to_cache(qlink: *mut qlist_node) -> *mut c_void {
    return virt_to_slab(qlink).slab_cache;
    }
#[no_mangle]
pub unsafe extern "C" fn qlink_to_object(qlink: *mut qlist_node, cache: *mut kmem_cache) -> *mut c_void {
    let mut free_info = container_of!(qlink, kasan_free_meta,
    quarantine_link);
    return (free_info) - cache.kasan_info.free_meta_offset;
    }
#[no_mangle]
unsafe extern "C" fn qlink_free(qlink: *mut qlist_node, cache: *mut kmem_cache) {
    let mut object = qlink_to_object(qlink, cache);
    let mut free_meta = kasan_get_free_meta(cache, object);
//
// Note: Keep per-object metadata to allow KASAN print stack traces for
// use-after-free-before-realloc bugs.
//
// If init_on_free is enabled and KASAN's free metadata is stored in
// the object, zero the metadata. Otherwise, the object's memory will
// not be properly zeroed, as KASAN saves the metadata after the slab
// allocator zeroes the object.
//
    if (slab_want_init_on_free(cache) &&
    cache.kasan_info.free_meta_offset == 0) {
    memzero_explicit(free_meta, sizeof!(*free_meta));
    }
    ___cache_free(cache, object, _THIS_IP_);
    }
#[no_mangle]
unsafe extern "C" fn qlist_free_all(q: *mut qlist_head, cache: *mut kmem_cache) {
pub static mut qlink: *mut c_void = core::ptr::null_mut();
    if (unlikely(qlist_empty(q))) {
    return;
    }
    qlink = q.head;
    while (qlink) {
    let mut obj_cache = cache ? cache :	qlink_to_cache(qlink);
    let mut next = qlink.next;
    qlink_free(qlink, obj_cache);
    qlink = next;
    }
    qlist_init(q);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_quarantine_put(cache: *mut kmem_cache, object: *mut c_void) -> bool {
    let mut flags = 0;
pub static mut q: *mut c_void = core::ptr::null_mut();
pub static mut temp: qlist_head = 0;
    let mut meta = kasan_get_free_meta(cache, object);
//
// If there's no metadata for this object, don't put it into
// quarantine.
//
    if (!meta) {
    return false;
    }
//
// Note: irq must be disabled until after we move the batch to the
// global quarantine. Otherwise kasan_quarantine_remove_cache() can
// miss some objects belonging to the cache if they are in our local
// temp list. kasan_quarantine_remove_cache() executes on_each_cpu()
// at the beginning which ensures that it either sees the objects in
// per-cpu lists or in the global quarantine.
//
    local_irq_save(flags);
    q = this_cpu_ptr(&cpu_quarantine);
    if (q.offline) {
    local_irq_restore(flags);
    return false;
    }
    qlist_put(q, &meta.quarantine_link, cache.size);
    if (unlikely(q.bytes > QUARANTINE_PERCPU_SIZE)) {
    qlist_move_all(q, &temp);
    raw_spin_lock(&quarantine_lock);
    WRITE_ONCE(quarantine_size, quarantine_size + temp.bytes);
    qlist_move_all(&temp, &global_quarantine[quarantine_tail]);
    if (global_quarantine[quarantine_tail].bytes >=
    READ_ONCE(quarantine_batch_size)) {
    let mut new_tail = 0;
    new_tail = quarantine_tail + 1;
    if (new_tail == QUARANTINE_BATCHES) {
    new_tail = 0;
    }
    if (new_tail != quarantine_head) {
    quarantine_tail = new_tail;
    }
    }
    raw_spin_unlock(&quarantine_lock);
    }
    local_irq_restore(flags);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_quarantine_reduce() {
    size_t total_size, new_quarantine_size, percpu_quarantines;
    let mut flags = 0;
    let mut srcu_idx = 0;
pub static mut to_free: qlist_head = 0;
    if (likely(READ_ONCE(quarantine_size) <=
    READ_ONCE(quarantine_max_size))) {
    return;
    }
//
// srcu critical section ensures that kasan_quarantine_remove_cache()
// will not miss objects belonging to the cache while they are in our
// local to_free list. srcu is chosen because (1) it gives us private
// grace period domain that does not interfere with anything else,
// and (2) it allows synchronize_srcu() to return without waiting
// if there are no pending read critical sections (which is the
// expected case).
//
    srcu_idx = srcu_read_lock(&remove_cache_srcu);
    raw_spin_lock_irqsave(&quarantine_lock, flags);
//
// Update quarantine size in case of hotplug. Allocate a fraction of
// the installed memory to quarantine minus per-cpu queue limits.
//
    total_size = (totalram_pages() << PAGE_SHIFT) /
    QUARANTINE_FRACTION;
    percpu_quarantines = QUARANTINE_PERCPU_SIZE * num_online_cpus();
    new_quarantine_size = (total_size < percpu_quarantines) ?
    0 : total_size - percpu_quarantines;
    WRITE_ONCE(quarantine_max_size, new_quarantine_size);
// Aim at consuming at most 1/2 of slots in quarantine.
    WRITE_ONCE(quarantine_batch_size, max((size_t)QUARANTINE_PERCPU_SIZE,
    2 * total_size / QUARANTINE_BATCHES));
    if (likely(quarantine_size > quarantine_max_size)) {
    qlist_move_all(&global_quarantine[quarantine_head], &to_free);
    WRITE_ONCE(quarantine_size, quarantine_size - to_free.bytes);
    quarantine_head += 1;
    if (quarantine_head == QUARANTINE_BATCHES) {
    quarantine_head = 0;
    }
    }
    raw_spin_unlock_irqrestore(&quarantine_lock, flags);
    qlist_free_all(&to_free, core::ptr::null_mut());
    srcu_read_unlock(&remove_cache_srcu, srcu_idx);
    }
#[no_mangle]
pub unsafe extern "C" fn qlist_move_cache(from: *mut qlist_head, to: *mut qlist_head, cache: *mut kmem_cache) {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    if (unlikely(qlist_empty(from))) {
    return;
    }
    curr = from.head;
    qlist_init(from);
    while (curr) {
    let mut next = curr.next;
    let mut obj_cache = qlink_to_cache(curr);
    if (obj_cache == cache) {
    qlist_put(to, curr, obj_cache.size);
    }
    else {
    qlist_put(from, curr, obj_cache.size);
    }
    curr = next;
    }
    }
#[no_mangle]
unsafe extern "C" fn __per_cpu_remove_cache(q: *mut qlist_head, arg: *mut c_void) {
    let mut cache = arg;
    let mut flags = 0;
pub static mut sq: *mut c_void = core::ptr::null_mut();
    sq = this_cpu_ptr(&shrink_qlist);
    raw_spin_lock_irqsave(&sq.lock, flags);
    qlist_move_cache(q, &sq.qlist, cache);
    raw_spin_unlock_irqrestore(&sq.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn per_cpu_remove_cache(arg: *mut c_void) {
pub static mut q: *mut c_void = core::ptr::null_mut();
    q = this_cpu_ptr(&cpu_quarantine);
//
// Ensure the ordering between the writing to q->offline and
// per_cpu_remove_cache.  Prevent cpu_quarantine from being corrupted
// by interrupt.
//
    if (READ_ONCE(q.offline)) {
    return;
    }
    __per_cpu_remove_cache(q, arg);
    }
// Free all quarantined objects belonging to cache.
#[no_mangle]
pub unsafe extern "C" fn kasan_quarantine_remove_cache(cache: *mut kmem_cache) {
    unsigned long flags, i;
pub static mut to_free: qlist_head = 0;
    let mut cpu = 0;
pub static mut sq: *mut c_void = core::ptr::null_mut();
//
// Must be careful to not miss any objects that are being moved from
// per-cpu list to the global quarantine in kasan_quarantine_put(),
nor objects being freed in kasan_quarantine_reduce(). on_each_cpu()
// achieves the first goal, while synchronize_srcu() achieves the
// second.
//
    on_each_cpu(per_cpu_remove_cache, cache, 1);
//
// A CPU can go offline after on_each_cpu() returns, leaving cache
// objects on that CPU's shrink list. Scan all possible CPUs to
// drain those lists.
//
    for_each_possible_cpu(cpu) {
    sq = per_cpu_ptr(&shrink_qlist, cpu);
    raw_spin_lock_irqsave(&sq.lock, flags);
    qlist_move_cache(&sq.qlist, &to_free, cache);
    raw_spin_unlock_irqrestore(&sq.lock, flags);
    }
    qlist_free_all(&to_free, cache);
    raw_spin_lock_irqsave(&quarantine_lock, flags);
    while (i < QUARANTINE_BATCHES) {
    let mut old_bytes = 0;
    if (qlist_empty(&global_quarantine[i])) {
    continue;
    }
    old_bytes = global_quarantine[i].bytes;
    qlist_move_cache(&global_quarantine[i], &to_free, cache);
    WRITE_ONCE(quarantine_size, quarantine_size -
    (old_bytes - global_quarantine[i].bytes));
// Scanning whole quarantine can take a while.
    raw_spin_unlock_irqrestore(&quarantine_lock, flags);
    cond_resched();
    raw_spin_lock_irqsave(&quarantine_lock, flags);
    }
    raw_spin_unlock_irqrestore(&quarantine_lock, flags);
    qlist_free_all(&to_free, cache);
    synchronize_srcu(&remove_cache_srcu);
    }
#[no_mangle]
unsafe extern "C" fn kasan_cpu_online(cpu: c_uint) -> c_int {
    this_cpu_ptr(&cpu_quarantine).offline = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kasan_cpu_offline(cpu: c_uint) -> c_int {
pub static mut q: *mut c_void = core::ptr::null_mut();
    q = this_cpu_ptr(&cpu_quarantine);
// Ensure the ordering between the writing to q->offline and
// qlist_free_all. Otherwise, cpu_quarantine may be corrupted
// by interrupt.
//
    WRITE_ONCE(q.offline, true);
    barrier();
    qlist_free_all(q, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kasan_cpu_quarantine_init() -> c_int {
pub static mut ret: c_int = 0;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "mm/kasan:online",
    kasan_cpu_online, kasan_cpu_offline);
    if (ret < 0) {
    pr_err!("cpu quarantine register failed [%d]\n", ret);
    }
    return ret;
    }
    late_initcall!(kasan_cpu_quarantine_init);