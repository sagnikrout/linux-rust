//! Automatically rewritten from C to Rust
//! Source: kernel/notifier.c
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



// SPDX-License-Identifier: GPL-2.0-only

// Macro flag: #define CREATE_TRACE_POINTS

//
// Notifier chain core routines.  The exported routines below
// are layered on top of these, with appropriate locking added.
//
#[no_mangle]
pub unsafe extern "C" fn notifier_chain_register(nl: *mut *mut notifier_block, n: *mut notifier_block, unique_priority: bool) -> c_int {
    while ((*nl) != core::ptr::null_mut()) {
    if (unlikely((*nl) == n)) {
    WARN(1, "notifier callback %ps already registered",
    n.notifier_call);
    return -EEXIST;
    }
    if (n.priority > (*nl).priority) {
    break;
    }
    if (n.priority == (*nl).priority && unique_priority) {
    return -EBUSY;
    }
    nl = &((*nl).next);
    }
    n.next = *nl;
    rcu_assign_pointer(*nl, n);
    trace_notifier_register(n.notifier_call);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn notifier_chain_unregister(nl: *mut *mut notifier_block, n: *mut notifier_block) -> c_int {
    while ((*nl) != core::ptr::null_mut()) {
    if ((*nl) == n) {
    rcu_assign_pointer(*nl, n.next);
    trace_notifier_unregister(n.notifier_call);
    return 0;
    }
    nl = &((*nl).next);
    }
    return -ENOENT;
    }
//
// notifier_call_chain - Informs the registered notifiers about an event.
// @nl:		Pointer to head of the blocking notifier chain
// @val:		Value passed unmodified to notifier function
// @v:		Pointer passed unmodified to notifier function
// @nr_to_call:	Number of notifier functions to be called. Don't care
// value of this parameter is -1.
// @nr_calls:	Records the number of notifications sent. Don't care
// value of this field is NULL.
// Return:		notifier_call_chain returns the value returned by the
// last notifier function called.
//
#[no_mangle]
pub unsafe extern "C" fn notifier_call_chain(nl: *mut *mut notifier_block, val: c_ulong, v: *mut c_void, nr_to_call: c_int, nr_calls: *mut c_int) -> c_int {
pub static mut ret: c_int = 0;
    let mut nb = core::ptr::null_mut();
    let mut next_nb = core::ptr::null_mut();
    nb = rcu_dereference_raw(*nl);
    while (nb && nr_to_call) {
    next_nb = rcu_dereference_raw(nb.next);

    if (unlikely(!func_ptr_is_kernel_text(nb.notifier_call))) {
    WARN(1, "Invalid notifier called!");
    nb = next_nb;
    continue;
    }

    trace_notifier_run(nb.notifier_call);
    ret = nb.notifier_call(nb, val, v);
    if (nr_calls) {
    (*nr_calls)++;
    }
    if (ret & NOTIFY_STOP_MASK) {
    break;
    }
    nb = next_nb;
    nr_to_call -= 1;
    }
    return ret;
    }
    NOKPROBE_SYMBOL(notifier_call_chain);
//
// notifier_call_chain_robust - Inform the registered notifiers about an event
// and rollback on error.
// @nl:		Pointer to head of the blocking notifier chain
// @val_up:	Value passed unmodified to the notifier function
// @val_down:	Value passed unmodified to the notifier function when recovering
// from an error on @val_up
// @v:		Pointer passed unmodified to the notifier function
//
// NOTE:	It is important the @nl chain doesn't change between the two
// invocations of notifier_call_chain() such that we visit the
// exact same notifier callbacks; this rules out any RCU usage.
//
// Return:	the return value of the @val_up call.
//
#[no_mangle]
pub unsafe extern "C" fn notifier_call_chain_robust(nl: *mut *mut notifier_block, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> c_int {
    int ret, nr = 0;
    ret = notifier_call_chain(nl, val_up, v, -1, &nr);
    if (ret & NOTIFY_STOP_MASK) {
    notifier_call_chain(nl, val_down, v, nr-1, core::ptr::null_mut());
    }
    return ret;
    }
//
// Atomic notifier chain routines.  Registration and unregistration
// use a spinlock, and call_chain is synchronized by RCU (no locks).
//
// atomic_notifier_chain_register - Add notifier to an atomic notifier chain
// @nh: Pointer to head of the atomic notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to an atomic notifier chain.
//
// Returns 0 on success, %-EEXIST on error.
//
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_register(nh: *mut atomic_notifier_head, n: *mut notifier_block) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    spin_lock_irqsave(&nh.lock, flags);
    ret = notifier_chain_register(&nh.head, n, false);
    spin_unlock_irqrestore(&nh.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(atomic_notifier_chain_register);
//
// atomic_notifier_chain_register_unique_prio - Add notifier to an atomic notifier chain
// @nh: Pointer to head of the atomic notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to an atomic notifier chain if there is no other
// notifier registered using the same priority.
//
// Returns 0 on success, %-EEXIST or %-EBUSY on error.
//
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_register_unique_prio(nh: *mut atomic_notifier_head, n: *mut notifier_block) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    spin_lock_irqsave(&nh.lock, flags);
    ret = notifier_chain_register(&nh.head, n, true);
    spin_unlock_irqrestore(&nh.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(atomic_notifier_chain_register_unique_prio);
//
// atomic_notifier_chain_unregister - Remove notifier from an atomic notifier chain
// @nh: Pointer to head of the atomic notifier chain
// @n: Entry to remove from notifier chain
//
// Removes a notifier from an atomic notifier chain.
//
// Returns zero on success or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_chain_unregister(nh: *mut atomic_notifier_head, n: *mut notifier_block) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    spin_lock_irqsave(&nh.lock, flags);
    ret = notifier_chain_unregister(&nh.head, n);
    spin_unlock_irqrestore(&nh.lock, flags);
    synchronize_rcu();
    return ret;
    }
    EXPORT_SYMBOL_GPL(atomic_notifier_chain_unregister);
//
// atomic_notifier_call_chain - Call functions in an atomic notifier chain
// @nh: Pointer to head of the atomic notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in a notifier chain in turn.  The functions
// run in an atomic context, so they must not block.
// This routine uses RCU to synchronize with changes to the chain.
//
// If the return value of the notifier can be and'ed
// with %NOTIFY_STOP_MASK then atomic_notifier_call_chain()
// will return immediately, with the return value of
// the notifier function which halted execution.
// Otherwise the return value is the return value
// of the last notifier function called.
//
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_call_chain(nh: *mut atomic_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    let mut ret = 0;
    rcu_read_lock();
    ret = notifier_call_chain(&nh.head, val, v, -1, core::ptr::null_mut());
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(atomic_notifier_call_chain);
    NOKPROBE_SYMBOL(atomic_notifier_call_chain);
//
// atomic_notifier_call_chain_is_empty - Check whether notifier chain is empty
// @nh: Pointer to head of the atomic notifier chain
//
// Checks whether notifier chain is empty.
//
// Returns true is notifier chain is empty, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn atomic_notifier_call_chain_is_empty(nh: *mut atomic_notifier_head) -> bool {
    return !rcu_access_pointer(nh.head);
    }
//
// Blocking notifier chain routines.  All access to the chain is
// synchronized by an rwsem.
//
#[no_mangle]
pub unsafe extern "C" fn __blocking_notifier_chain_register(nh: *mut blocking_notifier_head, n: *mut notifier_block, unique_priority: bool) -> c_int {
    let mut ret = 0;
//
// This code gets used during boot-up, when task switching is
// not yet working and interrupts must remain disabled.  At
// such times we must not call down_write().
//
    if (unlikely(system_state == SYSTEM_BOOTING)) {
    return notifier_chain_register(&nh.head, n, unique_priority);
    }
    down_write(&nh.rwsem);
    ret = notifier_chain_register(&nh.head, n, unique_priority);
    up_write(&nh.rwsem);
    return ret;
    }
//
// blocking_notifier_chain_register - Add notifier to a blocking notifier chain
// @nh: Pointer to head of the blocking notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to a blocking notifier chain.
// Must be called in process context.
//
// Returns 0 on success, %-EEXIST on error.
//
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_register(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    return __blocking_notifier_chain_register(nh, n, false);
    }
    EXPORT_SYMBOL_GPL(blocking_notifier_chain_register);
//
// blocking_notifier_chain_register_unique_prio - Add notifier to a blocking notifier chain
// @nh: Pointer to head of the blocking notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to an blocking notifier chain if there is no other
// notifier registered using the same priority.
//
// Returns 0 on success, %-EEXIST or %-EBUSY on error.
//
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_register_unique_prio(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    return __blocking_notifier_chain_register(nh, n, true);
    }
    EXPORT_SYMBOL_GPL(blocking_notifier_chain_register_unique_prio);
//
// blocking_notifier_chain_unregister - Remove notifier from a blocking notifier chain
// @nh: Pointer to head of the blocking notifier chain
// @n: Entry to remove from notifier chain
//
// Removes a notifier from a blocking notifier chain.
// Must be called from process context.
//
// Returns zero on success or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_chain_unregister(nh: *mut blocking_notifier_head, n: *mut notifier_block) -> c_int {
    let mut ret = 0;
//
// This code gets used during boot-up, when task switching is
// not yet working and interrupts must remain disabled.  At
// such times we must not call down_write().
//
    if (unlikely(system_state == SYSTEM_BOOTING)) {
    return notifier_chain_unregister(&nh.head, n);
    }
    down_write(&nh.rwsem);
    ret = notifier_chain_unregister(&nh.head, n);
    up_write(&nh.rwsem);
    return ret;
    }
    EXPORT_SYMBOL_GPL(blocking_notifier_chain_unregister);
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_call_chain_robust(nh: *mut blocking_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> c_int {
pub static mut ret: c_int = 0;
//
// We check the head outside the lock, but if this access is
// racy then it does not matter what the result of the test
// is, we re-check the list after having taken the lock anyway:
//
    if (rcu_access_pointer(nh.head)) {
    down_read(&nh.rwsem);
    ret = notifier_call_chain_robust(&nh.head, val_up, val_down, v);
    up_read(&nh.rwsem);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(blocking_notifier_call_chain_robust);
//
// blocking_notifier_call_chain - Call functions in a blocking notifier chain
// @nh: Pointer to head of the blocking notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in a notifier chain in turn.  The functions
// run in a process context, so they are allowed to block.
//
// If the return value of the notifier can be and'ed
// with %NOTIFY_STOP_MASK then blocking_notifier_call_chain()
// will return immediately, with the return value of
// the notifier function which halted execution.
// Otherwise the return value is the return value
// of the last notifier function called.
//
#[no_mangle]
pub unsafe extern "C" fn blocking_notifier_call_chain(nh: *mut blocking_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
pub static mut ret: c_int = 0;
//
// We check the head outside the lock, but if this access is
// racy then it does not matter what the result of the test
// is, we re-check the list after having taken the lock anyway:
//
    if (rcu_access_pointer(nh.head)) {
    down_read(&nh.rwsem);
    ret = notifier_call_chain(&nh.head, val, v, -1, core::ptr::null_mut());
    up_read(&nh.rwsem);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(blocking_notifier_call_chain);
//
// Raw notifier chain routines.  There is no protection;
// the caller must provide it.  Use at your own risk!
//
// raw_notifier_chain_register - Add notifier to a raw notifier chain
// @nh: Pointer to head of the raw notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to a raw notifier chain.
// All locking must be provided by the caller.
//
// Returns 0 on success, %-EEXIST on error.
//
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_chain_register(nh: *mut raw_notifier_head, n: *mut notifier_block) -> c_int {
    return notifier_chain_register(&nh.head, n, false);
    }
    EXPORT_SYMBOL_GPL(raw_notifier_chain_register);
//
// raw_notifier_chain_unregister - Remove notifier from a raw notifier chain
// @nh: Pointer to head of the raw notifier chain
// @n: Entry to remove from notifier chain
//
// Removes a notifier from a raw notifier chain.
// All locking must be provided by the caller.
//
// Returns zero on success or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_chain_unregister(nh: *mut raw_notifier_head, n: *mut notifier_block) -> c_int {
    return notifier_chain_unregister(&nh.head, n);
    }
    EXPORT_SYMBOL_GPL(raw_notifier_chain_unregister);
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_call_chain_robust(nh: *mut raw_notifier_head, val_up: c_ulong, val_down: c_ulong, v: *mut c_void) -> c_int {
    return notifier_call_chain_robust(&nh.head, val_up, val_down, v);
    }
    EXPORT_SYMBOL_GPL(raw_notifier_call_chain_robust);
//
// raw_notifier_call_chain - Call functions in a raw notifier chain
// @nh: Pointer to head of the raw notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in a notifier chain in turn.  The functions
// run in an undefined context.
// All locking must be provided by the caller.
//
// If the return value of the notifier can be and'ed
// with %NOTIFY_STOP_MASK then raw_notifier_call_chain()
// will return immediately, with the return value of
// the notifier function which halted execution.
// Otherwise the return value is the return value
// of the last notifier function called.
//
#[no_mangle]
pub unsafe extern "C" fn raw_notifier_call_chain(nh: *mut raw_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    return notifier_call_chain(&nh.head, val, v, -1, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(raw_notifier_call_chain);
//
// SRCU notifier chain routines.    Registration and unregistration
// use a mutex, and call_chain is synchronized by SRCU (no locks).
//
// srcu_notifier_chain_register - Add notifier to an SRCU notifier chain
// @nh: Pointer to head of the SRCU notifier chain
// @n: New entry in notifier chain
//
// Adds a notifier to an SRCU notifier chain.
// Must be called in process context.
//
// Returns 0 on success, %-EEXIST on error.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_chain_register(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> c_int {
    let mut ret = 0;
//
// This code gets used during boot-up, when task switching is
// not yet working and interrupts must remain disabled.  At
// such times we must not call mutex_lock().
//
    if (unlikely(system_state == SYSTEM_BOOTING)) {
    return notifier_chain_register(&nh.head, n, false);
    }
    mutex_lock(&nh.mutex);
    ret = notifier_chain_register(&nh.head, n, false);
    mutex_unlock(&nh.mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(srcu_notifier_chain_register);
//
// srcu_notifier_chain_unregister - Remove notifier from an SRCU notifier chain
// @nh: Pointer to head of the SRCU notifier chain
// @n: Entry to remove from notifier chain
//
// Removes a notifier from an SRCU notifier chain.
// Must be called from process context.
//
// Returns zero on success or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_chain_unregister(nh: *mut srcu_notifier_head, n: *mut notifier_block) -> c_int {
    let mut ret = 0;
//
// This code gets used during boot-up, when task switching is
// not yet working and interrupts must remain disabled.  At
// such times we must not call mutex_lock().
//
    if (unlikely(system_state == SYSTEM_BOOTING)) {
    return notifier_chain_unregister(&nh.head, n);
    }
    mutex_lock(&nh.mutex);
    ret = notifier_chain_unregister(&nh.head, n);
    mutex_unlock(&nh.mutex);
    synchronize_srcu(&nh.srcu);
    return ret;
    }
    EXPORT_SYMBOL_GPL(srcu_notifier_chain_unregister);
//
// srcu_notifier_call_chain - Call functions in an SRCU notifier chain
// @nh: Pointer to head of the SRCU notifier chain
// @val: Value passed unmodified to notifier function
// @v: Pointer passed unmodified to notifier function
//
// Calls each function in a notifier chain in turn.  The functions
// run in a process context, so they are allowed to block.
//
// If the return value of the notifier can be and'ed
// with %NOTIFY_STOP_MASK then srcu_notifier_call_chain()
// will return immediately, with the return value of
// the notifier function which halted execution.
// Otherwise the return value is the return value
// of the last notifier function called.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_notifier_call_chain(nh: *mut srcu_notifier_head, val: c_ulong, v: *mut c_void) -> c_int {
    let mut ret = 0;
    let mut idx = 0;
    idx = srcu_read_lock(&nh.srcu);
    ret = notifier_call_chain(&nh.head, val, v, -1, core::ptr::null_mut());
    srcu_read_unlock(&nh.srcu, idx);
    return ret;
    }
    EXPORT_SYMBOL_GPL(srcu_notifier_call_chain);
//
// srcu_init_notifier_head - Initialize an SRCU notifier head
// @nh: Pointer to head of the srcu notifier chain
//
// Unlike other sorts of notifier heads, SRCU notifier heads require
// dynamic initialization.  Be sure to call this routine before
// calling any of the other SRCU notifier routines for this head.
//
// If an SRCU notifier head is deallocated, it must first be cleaned
// up by calling srcu_cleanup_notifier_head().  Otherwise the head's
// per-cpu data (used by the SRCU mechanism) will leak.
//
#[no_mangle]
pub unsafe extern "C" fn srcu_init_notifier_head(nh: *mut srcu_notifier_head) {
    mutex_init(&nh.mutex);
    if (init_srcu_struct(&nh.srcu) < 0) {
    BUG();
    }
    nh.head = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(srcu_init_notifier_head);
// static ATOMIC_NOTIFIER_HEAD(die_chain);
    int notrace notify_die(enum die_val val, const char *str, pt_regs *regs, long err, int trap, int sig)
    {
pub static mut die_args: usize = 0;
    RCU_LOCKDEP_WARN(!rcu_is_watching(),
    "notify_die called but RCU thinks we're quiescent");
    return atomic_notifier_call_chain(&die_chain, val, &args);
    }
    NOKPROBE_SYMBOL(notify_die);
#[no_mangle]
pub unsafe extern "C" fn register_die_notifier(nb: *mut notifier_block) -> c_int {
    return atomic_notifier_chain_register(&die_chain, nb);
    }
    EXPORT_SYMBOL_GPL(register_die_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_die_notifier(nb: *mut notifier_block) -> c_int {
    return atomic_notifier_chain_unregister(&die_chain, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_die_notifier);