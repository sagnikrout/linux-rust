//! Automatically rewritten from C to Rust
//! Source: kernel/sched/wait_bit.c
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
// The implementation of the wait_bit*() and related waiting APIs:
//
pub const WAIT_TABLE_BITS: c_int = 8;

    static wait_queue_head_t bit_wait_table[WAIT_TABLE_SIZE] __cacheline_aligned;
    wait_queue_head_t *bit_waitqueue(unsigned long *word, int bit)
    {
pub static mut shift: c_int = 0;
pub static mut val: c_ulong = 0;
    return bit_wait_table + hash_long(val, WAIT_TABLE_BITS);
    }
    EXPORT_SYMBOL(bit_waitqueue);
#[no_mangle]
pub unsafe extern "C" fn wake_bit_function(wq_entry: *mut wait_queue_entry, mode: unsigned, sync: c_int, arg: *mut c_void) -> c_int {
    let mut key = arg;
    let mut wait_bit = container_of!(wq_entry, wait_bit_queue_entry, wq_entry);
    if (wait_bit.key.flags != key.flags ||
    wait_bit.key.bit_nr != key.bit_nr ||
    test_bit(key.bit_nr, key.flags)) {
    return 0;
    }
    return autoremove_wake_function(wq_entry, mode, sync, key);
    }
    EXPORT_SYMBOL(wake_bit_function);
//
// To allow interruptible waiting and asynchronous (i.e. non-blocking)
// waiting, the actions of __wait_on_bit() and __wait_on_bit_lock() are
// permitted return codes. Nonzero return codes halt waiting and return.
//
    int __sched
    __wait_on_bit(wait_queue_head *wq_head, wait_bit_queue_entry *wbq_entry,
    wait_bit_action_f *action, unsigned mode)
    {
pub static mut ret: c_int = 0;
    do {
    prepare_to_wait(wq_head, &wbq_entry.wq_entry, mode);
    if (test_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags)) {
    ret = (*action)(&wbq_entry.key, mode);
    }
    } while (test_bit_acquire(wbq_entry.key.bit_nr, wbq_entry.key.flags) && !ret);
    finish_wait(wq_head, &wbq_entry.wq_entry);
    return ret;
    }
    EXPORT_SYMBOL(__wait_on_bit);
    int __sched out_of_line_wait_on_bit(unsigned long *word, int bit,
    wait_bit_action_f *action, unsigned mode)
    {
    let mut wq_head = bit_waitqueue(word, bit);
pub static mut wq_entry: usize = 0;
    return __wait_on_bit(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL(out_of_line_wait_on_bit);
    int __sched out_of_line_wait_on_bit_timeout(
    unsigned long *word, int bit, wait_bit_action_f *action,
    unsigned mode, unsigned long timeout)
    {
    let mut wq_head = bit_waitqueue(word, bit);
pub static mut wq_entry: usize = 0;
    wq_entry.key.timeout = jiffies + timeout;
    return __wait_on_bit(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL_GPL(out_of_line_wait_on_bit_timeout);
    int __sched
    __wait_on_bit_lock(wait_queue_head *wq_head, wait_bit_queue_entry *wbq_entry,
    wait_bit_action_f *action, unsigned mode)
    {
pub static mut ret: c_int = 0;
    for (;;) {
    prepare_to_wait_exclusive(wq_head, &wbq_entry.wq_entry, mode);
    if (test_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags)) {
    ret = action(&wbq_entry.key, mode);
//
// See the comment in prepare_to_wait_event().
// finish_wait() does not necessarily takes wwq_head->lock,
// but test_and_set_bit() implies mb() which pairs with
// smp_mb__after_atomic() before wake_up_page().
//
    if (ret) {
    finish_wait(wq_head, &wbq_entry.wq_entry);
    }
    }
    if (!test_and_set_bit(wbq_entry.key.bit_nr, wbq_entry.key.flags)) {
    if (!ret) {
    finish_wait(wq_head, &wbq_entry.wq_entry);
    }
    return 0;
    } else if (ret) {
    return ret;
    }
    }
    }
    EXPORT_SYMBOL(__wait_on_bit_lock);
    int __sched out_of_line_wait_on_bit_lock(unsigned long *word, int bit,
    wait_bit_action_f *action, unsigned mode)
    {
    let mut wq_head = bit_waitqueue(word, bit);
pub static mut wq_entry: usize = 0;
    return __wait_on_bit_lock(wq_head, &wq_entry, action, mode);
    }
    EXPORT_SYMBOL(out_of_line_wait_on_bit_lock);
#[no_mangle]
pub unsafe extern "C" fn __wake_up_bit(wq_head: *mut wait_queue_head, word: *mut c_ulong, bit: c_int) {
pub static mut key: wait_bit_key = 0;
    if (waitqueue_active(wq_head)) {
    __wake_up(wq_head, TASK_NORMAL, 1, &key);
    }
    }
    EXPORT_SYMBOL(__wake_up_bit);
//
// wake_up_bit - wake up waiters on a bit
// @word: the address containing the bit being waited on
// @bit: the bit at that address being waited on
//
// Wake up any process waiting in wait_on_bit() or similar for the
// given bit to be cleared.
//
// The wake-up is sent to tasks in a waitqueue selected by hash from a
// shared pool.  Only those tasks on that queue which have requested
// wake_up on this specific address and bit will be woken, and only if the
// bit is clear.
//
// In order for this to function properly there must be a full memory
// barrier after the bit is cleared and before this function is called.
// If the bit was cleared atomically, such as a by clear_bit() then
// smb_mb__after_atomic() can be used, othwewise smb_mb() is needed.
// If the bit was cleared with a fully-ordered operation, no further
// barrier is required.
//
// Normally the bit should be cleared by an operation with RELEASE
// semantics so that any changes to memory made before the bit is
// cleared are guaranteed to be visible after the matching wait_on_bit()
// completes.
//
#[no_mangle]
pub unsafe extern "C" fn wake_up_bit(word: *mut c_ulong, bit: c_int) {
    __wake_up_bit(bit_waitqueue(word, bit), word, bit);
    }
    EXPORT_SYMBOL(wake_up_bit);
    wait_queue_head_t *__var_waitqueue(void *p)
    {
    return bit_wait_table + hash_ptr(p, WAIT_TABLE_BITS);
    }
    EXPORT_SYMBOL(__var_waitqueue);
#[no_mangle]
pub unsafe extern "C" fn __var_wake_key(wq_entry: *mut wait_queue_entry, arg: *mut c_void) -> *mut c_void {
    let mut key = arg;
    let mut wbq_entry = container_of!(wq_entry, wait_bit_queue_entry, wq_entry);
    if (wbq_entry.key.flags != key.flags ||
    wbq_entry.key.bit_nr != key.bit_nr) {
    return core::ptr::null_mut();
    }
    return key;
    }
#[no_mangle]
pub unsafe extern "C" fn var_wake_function(wq_entry: *mut wait_queue_entry, mode: c_uint, sync: c_int, arg: *mut c_void) -> c_int {
    let mut key = __var_wake_key(wq_entry, arg);
    if (!key) {
    return 0;
    }
    return autoremove_wake_function(wq_entry, mode, sync, key);
    }
#[no_mangle]
pub unsafe extern "C" fn init_wait_var_entry(wbq_entry: *mut wait_bit_queue_entry, var: *mut c_void, flags: c_int) {
// wbq_entry = (wait_bit_queue_entry){
    .key = {
    .flags	= (var),
    .bit_nr = -1,
    },
    .wq_entry = {
    .flags	 = flags,
    .private = current,
    .func	 = var_wake_function,
    .entry	 = LIST_HEAD_INIT(wbq_entry.wq_entry.entry),
    },
    };
    }
    EXPORT_SYMBOL(init_wait_var_entry);
//
// wake_up_var - wake up waiters on a variable (kernel address)
// @var: the address of the variable being waited on
//
// Wake up any process waiting in wait_var_event() or similar for the
// given variable to change.  wait_var_event() can be waiting for an
// arbitrary condition to be true and associates that condition with an
// address.  Calling wake_up_var() suggests that the condition has been
// made true, but does not strictly require the condtion to use the
// address given.
//
// The wake-up is sent to tasks in a waitqueue selected by hash from a
// shared pool.  Only those tasks on that queue which have requested
// wake_up on this specific address will be woken.
//
// In order for this to function properly there must be a full memory
// barrier after the variable is updated (or more accurately, after the
// condition waited on has been made to be true) and before this function
// is called.  If the variable was updated atomically, such as a by
// atomic_dec() then smb_mb__after_atomic() can be used.  If the
// variable was updated by a fully ordered operation such as
// atomic_dec_and_test() then no extra barrier is required.  Otherwise
// smb_mb() is needed.
//
// Normally the variable should be updated (the condition should be made
// to be true) by an operation with RELEASE semantics such as
// smp_store_release() so that any changes to memory made before the
// variable was updated are guaranteed to be visible after the matching
// wait_var_event() completes.
//
#[no_mangle]
pub unsafe extern "C" fn wake_up_var(var: *mut c_void) {
    __wake_up_bit(__var_waitqueue(var), var, -1);
    }
    EXPORT_SYMBOL(wake_up_var);
#[no_mangle]
pub unsafe extern "C" fn bit_wait(word: *mut wait_bit_key, mode: c_int) -> __sched int {
    schedule();
    if (signal_pending_state(mode, current)) {
    return -EINTR;
    }
    return 0;
    }
    EXPORT_SYMBOL(bit_wait);
#[no_mangle]
pub unsafe extern "C" fn bit_wait_io(word: *mut wait_bit_key, mode: c_int) -> __sched int {
    io_schedule();
    if (signal_pending_state(mode, current)) {
    return -EINTR;
    }
    return 0;
    }
    EXPORT_SYMBOL(bit_wait_io);
#[no_mangle]
pub unsafe extern "C" fn bit_wait_timeout(word: *mut wait_bit_key, mode: c_int) -> __sched int {
pub static mut now: c_ulong = 0;
    if (time_after_eq(now, word.timeout)) {
    return -EAGAIN;
    }
    schedule_timeout(word.timeout - now);
    if (signal_pending_state(mode, current)) {
    return -EINTR;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(bit_wait_timeout);
#[no_mangle]
pub unsafe extern "C" fn wait_bit_init()  {
    let mut i = 0;
    for (i = 0; i < WAIT_TABLE_SIZE; i++) {
    init_waitqueue_head(bit_wait_table + i);
    }
    }