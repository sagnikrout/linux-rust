//! Automatically rewritten from C to Rust
//! Source: kernel/locking/osq_lock.c
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
// An MCS like lock especially tailored for optimistic spinning for sleeping
// lock implementations (mutex, rwsem, etc).
//
// Using a single mcs node per CPU is safe because sleeping locks should not be
// called from interrupt context and we have preemption disabled while
// spinning.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optimistic_spin_node {
    pub prev: *mut *mut optimistic_spin_node next,,
//     pub /: *mut *mut int locked; / 1 if lock acquired,
//     pub /: *mut *mut int cpu; / encoded CPU # + 1 value,
}

pub static mut struct optimistic_spin_node: usize = 0;
//
// We use the value 0 to represent "no CPU", thus the encoded value
// will be the CPU number incremented by 1.
//
#[no_mangle]
pub unsafe extern "C" fn encode_cpu(cpu_nr: c_int) -> c_int {
    return cpu_nr + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn node_cpu(node: *mut optimistic_spin_node) -> c_int {
    return node.cpu - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn decode_cpu(encoded_cpu_val: c_int) -> *mut c_void {
pub static mut cpu_nr: c_int = 0;
    return per_cpu_ptr(&osq_node, cpu_nr);
    }
//
// Get a stable @node->next pointer, either for unlock() or unqueue() purposes.
// Can return NULL in case we were the last queued and we updated @lock instead.
//
// If osq_lock() is being cancelled there must be a previous node
// and 'old_cpu' is its CPU #.
// For osq_unlock() there is never a previous node and old_cpu is
// set to OSQ_UNLOCKED_VAL.
//
#[no_mangle]
pub unsafe extern "C" fn osq_wait_next(lock: *mut optimistic_spin_queue, node: *mut optimistic_spin_node, old_cpu: c_int) -> *mut c_void {
pub static mut curr: c_int = 0;
    for (;;) {
    if (atomic_read(&lock.tail) == curr &&
    atomic_cmpxchg_acquire(&lock.tail, curr, old_cpu) == curr) {
//
// We were the last queued, we moved @lock back. @prev
// will now observe @lock and will complete its
// unlock()/unqueue().
//
    return core::ptr::null_mut();
    }
//
// We must xchg() the @node->next value, because if we were to
// leave it in, a concurrent unlock()/unqueue() from
// @node->next might complete Step-A and think its @prev is
// still valid.
//
// If the concurrent unlock()/unqueue() wins the race, we'll
// wait for either @lock to point to us, through its Step-B, or
// wait for a new @node->next from its Step-C.
//
    if (node.next) {
pub static mut next: *mut c_void = core::ptr::null_mut();
    next = xchg(&node.next, core::ptr::null_mut());
    if (next) {
    return next;
    }
    }
    cpu_relax();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn osq_lock(lock: *mut optimistic_spin_queue) -> bool {
    let mut node = this_cpu_ptr(&osq_node);
    let mut prev = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut curr: c_int = 0;
    let mut old = 0;
    node.locked = 0;
    node.next = core::ptr::null_mut();
    node.cpu = curr;
//
// We need both ACQUIRE (pairs with corresponding RELEASE in
// unlock() uncontended, or fastpath) and RELEASE (to publish
// the node fields we just initialised) semantics when updating
// the lock tail.
//
    old = atomic_xchg(&lock.tail, curr);
    if (old == OSQ_UNLOCKED_VAL) {
    return true;
    }
    prev = decode_cpu(old);
    node.prev = prev;
//
// osq_lock()			unqueue
//
// node->prev = prev		osq_wait_next()
// WMB				MB
// prev->next = node		next->prev = prev // unqueue-C
//
// Here 'node->prev' and 'next->prev' are the same variable and we need
// to ensure these stores happen in-order to avoid corrupting the list.
//
    smp_wmb();
    WRITE_ONCE(prev.next, node);
//
// Normally @prev is untouchable after the above store; because at that
// moment unlock can proceed and wipe the node element from stack.
//
// However, since our nodes are static per-cpu storage, we're
// guaranteed their existence -- this allows us to apply
// cmpxchg in an attempt to undo our queueing.
//
// Wait to acquire the lock or cancellation. Note that need_resched()
// will come with an IPI, which will wake smp_cond_load_relaxed() if it
// is implemented with a monitor-wait. vcpu_is_preempted() relies on
// polling, be careful.
//
    if (smp_cond_load_relaxed(&node.locked, VAL || need_resched() ||
    vcpu_is_preempted(node_cpu(node.prev)))) {
    return true;
    }
// unqueue
//
// Step - A  -- stabilize @prev
//
// Undo our @prev->next assignment; this will make @prev's
// unlock()/unqueue() wait for a next pointer since @lock points to us
// (or later).
//
    for (;;) {
//
// cpu_relax() below implies a compiler barrier which would
// prevent this comparison being optimized away.
//
    if (data_race(prev.next) == node &&
    cmpxchg(&prev.next, node, core::ptr::null_mut()) == node) {
    break;
    }
//
// We can only fail the cmpxchg() racing against an unlock(),
// in which case we should observe @node->locked becoming
// true.
//
    if (smp_load_acquire(&node.locked)) {
    return true;
    }
    cpu_relax();
//
// Or we race against a concurrent unqueue()'s step-B, in which
// case its step-C will write us a new @node->prev pointer.
//
    prev = READ_ONCE(node.prev);
    }
//
// Step - B -- stabilize @next
//
// Similar to unlock(), wait for @node->next or move @lock from @node
// back to @prev.
//
    next = osq_wait_next(lock, node, prev.cpu);
    if (!next) {
    return false;
    }
//
// Step - C -- unlink
//
// @prev is stable because its still waiting for a new @prev->next
// pointer, @next is stable because our @node->next pointer is NULL and
// it will wait in Step-A.
//
    WRITE_ONCE(next.prev, prev);
    WRITE_ONCE(prev.next, next);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn osq_unlock(lock: *mut optimistic_spin_queue) {
    let mut node = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut curr: c_int = 0;
//
// Fast path for the uncontended case.
//
    if (atomic_try_cmpxchg_release(&lock.tail, &curr, OSQ_UNLOCKED_VAL)) {
    return;
    }
//
// Second most likely case.
//
    node = this_cpu_ptr(&osq_node);
    next = xchg(&node.next, core::ptr::null_mut());
    if (next) {
    WRITE_ONCE(next.locked, 1);
    return;
    }
    next = osq_wait_next(lock, node, OSQ_UNLOCKED_VAL);
    if (next) {
    WRITE_ONCE(next.locked, 1);
    }
    }