//! Automatically rewritten from C to Rust
//! Source: kernel/unwind/deferred.c
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
// Deferred user space unwinding
//

//
// For requesting a deferred user space stack trace from NMI context
// the architecture must support a safe cmpxchg in NMI context.
// For those architectures that do not have that, then it cannot ask
// for a deferred user space stack trace from an NMI context. If it
// does, then it will get -EINVAL.
//

#[no_mangle]
pub unsafe extern "C" fn try_assign_cnt(info: *mut unwind_task_info, cnt: u32) -> bool {
pub static mut old: u32 = 0;
    return try_cmpxchg(&info.id.cnt, &old, cnt);
    }

// When NMIs are not allowed, this always succeeds
#[no_mangle]
#[no_mangle]
// duplicate fn: try_assign_cnt
pub unsafe extern "C" fn try_assign_cnt_dup(info: *mut unwind_task_info, cnt: u32) -> bool {
    info.id.cnt = cnt;
    return true;
    }

// Make the cache fit in a 4K page

    ((SZ_4K - sizeof!(unwind_cache)) / sizeof!(long))
// Guards adding to or removing from the list of callbacks
pub static mut callback_mutex: usize = 0;
pub static mut callbacks: usize = 0;

// Zero'd bits are available for assigning callback users
pub static mut unwind_mask: unsigned long = 0;
pub static mut unwind_srcu: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn unwind_pending(info: *mut unwind_task_info) -> bool {
    return atomic_long_read(&info.unwind_mask) & UNWIND_PENDING;
    }
//
// This is a unique percpu identifier for a given task entry context.
// Conceptually, it's incremented every time the CPU enters the kernel from
// user space, so that each "entry context" on the CPU gets a unique ID.  In
// reality, as an optimization, it's only incremented on demand for the first
// deferred unwind request after a given entry-from-user.
//
// It's combined with the CPU id to make a systemwide-unique "context cookie".
//
pub static mut u32: usize = 0;
//
// The context cookie is a unique identifier that is assigned to a user
// space stacktrace. As the user space stacktrace remains the same while
// the task is in the kernel, the cookie is an identifier for the stacktrace.
// Although it is possible for the stacktrace to get another cookie if another
// request is made after the cookie was cleared and before reentering user
// space.
//
#[no_mangle]
unsafe extern "C" fn get_cookie(info: *mut unwind_task_info) -> u64 {
pub static mut cnt: u32 = 1;
    lockdep_assert_irqs_disabled();
    if (info.id.cpu) {
    return info.id.id;
    }
// LSB is always set to ensure 0 is an invalid value
    cnt |= __this_cpu_read(unwind_ctx_ctr) + 2;
    if (try_assign_cnt(info, cnt)) {
// Update the per cpu counter
    __this_cpu_write(unwind_ctx_ctr, cnt);
    }
// Interrupts are disabled, the CPU will always be same
    info.id.cpu = smp_processor_id() + 1; /* Must be non zero */
    return info.id.id;
    }
//
// unwind_user_faultable - Produce a user stacktrace in faultable context
// @trace: The descriptor that will store the user stacktrace
//
// This must be called in a known faultable context (usually when entering
// or exiting user space). Depending on the available implementations
// the @trace will be loaded with the addresses of the user space stacktrace
// if it can be found.
//
// Return: 0 on success and negative on error
// On success @trace will contain the user space stacktrace
//
#[no_mangle]
pub unsafe extern "C" fn unwind_user_faultable(trace: *mut unwind_stacktrace) -> c_int {
    let mut info = &current.unwind_info;
pub static mut cache: *mut c_void = core::ptr::null_mut();
// Should always be called from faultable context
    might_fault();
    if (!current.mm) {
    return -EINVAL;
    }
    if (!info.cache) {
    info.cache = kzalloc_flex(*cache, entries, UNWIND_MAX_ENTRIES);
    if (!info.cache) {
    return -ENOMEM;
    }
    }
    cache = info.cache;
    trace.entries = cache.entries;
    trace.nr = cache.nr_entries;
//
// The user stack has already been previously unwound in this
// entry context.  Skip the unwind and use the cache.
//
    if (trace.nr) {
    return 0;
    }
    unwind_user(trace, UNWIND_MAX_ENTRIES);
    cache.nr_entries = trace.nr;
// Clear nr_entries on way back to user space
    atomic_long_or(UNWIND_USED, &info.unwind_mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn process_unwind_deferred(task: *mut task_struct) {
    let mut info = &task.unwind_info;
pub static mut trace: usize = 0;
pub static mut work: *mut c_void = core::ptr::null_mut();
    let mut bits = 0;
    let mut cookie = 0;
    if (WARN_ON_ONCE!(!unwind_pending(info))) {
    return;
    }
// Clear pending bit but make sure to have the current bits
    bits = atomic_long_fetch_andnot(UNWIND_PENDING,
    &info.unwind_mask);
//
// From here on out, the callback must always be called, even if it's
// just an empty trace.
//
    trace.nr = 0;
    trace.entries = core::ptr::null_mut();
    unwind_user_faultable(&trace);
    if (info.cache) {
    bits &= ~(info.cache.unwind_completed);
    }
    cookie = info.id.id;
    guard(srcu)(&unwind_srcu);
    list_for_each_entry_srcu(work, &callbacks, list,
    srcu_read_lock_held(&unwind_srcu)) {
    if (test_bit(work.bit, &bits)) {
    work.func(work, &trace, cookie);
    if (info.cache) {
    info.cache.unwind_completed |= BIT(work.bit);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn unwind_deferred_task_work(head: *mut callback_head) {
    process_unwind_deferred(current);
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_deferred_task_exit(task: *mut task_struct) {
    let mut info = &current.unwind_info;
    if (!unwind_pending(info)) {
    return;
    }
    process_unwind_deferred(task);
    task_work_cancel(task, &info.work);
    }
//
// unwind_deferred_request - Request a user stacktrace on task kernel exit
// @work: Unwind descriptor requesting the trace
// @cookie: The cookie of the first request made for this task
//
// Schedule a user space unwind to be done in task work before exiting the
// kernel.
//
// The returned @cookie output is the generated cookie of the very first
// request for a user space stacktrace for this task since it entered the
// kernel. It can be from a request by any caller of this infrastructure.
// Its value will also be passed to the callback function.  It can be
// used to stitch kernel and user stack traces together in post-processing.
//
// It's valid to call this function multiple times for the same @work within
// the same task entry context.  Each call will return the same cookie
// while the task hasn't left the kernel. If the callback is not pending
// because it has already been previously called for the same entry context,
// it will be called again with the same stack trace and cookie.
//
// Return: 0 if the callback successfully was queued.
// 1 if the callback is pending or was already executed.
// Negative if there's an error.
// @cookie holds the cookie of the first request by any user
//
#[no_mangle]
pub unsafe extern "C" fn unwind_deferred_request(work: *mut unwind_work, cookie: *mut u64) -> c_int {
    let mut info = &current.unwind_info;
pub static mut twa_mode: c_int = 0;
    unsigned long old, bits;
    let mut bit = 0;
    let mut ret = 0;
// cookie = 0;
    if ((current.flags & (PF_KTHREAD | PF_EXITING)) ||
    !user_mode(task_pt_regs(current))) {
    return -EINVAL;
    }
//
// NMI requires having safe cmpxchg operations.
// Trigger a warning to make it obvious that an architecture
// is using this in NMI when it should not be.
//
    if (in_nmi()) {
    if (WARN_ON_ONCE!(!CAN_USE_IN_NMI)) {
    return -EINVAL;
    }
    twa_mode = TWA_NMI_CURRENT;
    }
// Do not allow cancelled works to request again
    bit = READ_ONCE(work.bit);
    if (WARN_ON_ONCE!(bit < 0)) {
    return -EINVAL;
    }
// Only need the mask now
    bit = BIT(bit);
    guard(irqsave)();
// cookie = get_cookie(info);
    old = atomic_long_read(&info.unwind_mask);
// Is this already queued or executed
    if (old & bit) {
    return 1;
    }
//
// This work's bit hasn't been set yet. Now set it with the PENDING
// bit and fetch the current value of unwind_mask. If ether the
// work's bit or PENDING was already set, then this is already queued
// to have a callback.
//
    bits = UNWIND_PENDING | bit;
    old = atomic_long_fetch_or(bits, &info.unwind_mask);
    if (old & bits) {
//
// If the work's bit was set, whatever set it had better
// have also set pending and queued a callback.
//
    WARN_ON_ONCE!(!(old & UNWIND_PENDING));
    return old & bit;
    }
// The work has been claimed, now schedule it.
    ret = task_work_add(current, &info.work, twa_mode);
    if (WARN_ON_ONCE!(ret)) {
    atomic_long_set(&info.unwind_mask, 0);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_deferred_cancel(work: *mut unwind_work) {
    let mut g = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut bit = 0;
    if (!work) {
    return;
    }
    bit = work.bit;
// No work should be using a reserved bit
    if (WARN_ON_ONCE!(BIT(bit) & RESERVED_BITS)) {
    return;
    }
    guard(mutex)(&callback_mutex);
    list_del_rcu(&work.list);
// Do not allow any more requests and prevent callbacks
    work.bit = -1;
    __clear_bit(bit, &unwind_mask);
    synchronize_srcu(&unwind_srcu);
    guard(rcu)();
// Clear this bit from all threads
    for_each_process_thread(g, t) {
    atomic_long_andnot(BIT(bit),
    &t.unwind_info.unwind_mask);
    if (t.unwind_info.cache) {
    clear_bit(bit, &t.unwind_info.cache.unwind_completed);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_deferred_init(work: *mut unwind_work, func: unwind_callback_t) -> c_int {
    memset(work, 0, sizeof!(*work));
    guard(mutex)(&callback_mutex);
// See if there's a bit in the mask available
    if (unwind_mask == ~0UL) {
    return -EBUSY;
    }
    work.bit = ffz(unwind_mask);
    __set_bit(work.bit, &unwind_mask);
    list_add_rcu(&work.list, &callbacks);
    work.func = func;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_task_init(task: *mut task_struct) {
    let mut info = &task.unwind_info;
    memset(info, 0, sizeof!(*info));
    init_task_work(&info.work, unwind_deferred_task_work);
    atomic_long_set(&info.unwind_mask, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn unwind_task_free(task: *mut task_struct) {
    let mut info = &task.unwind_info;
    kfree(info.cache);
    task_work_cancel(task, &info.work);
    }