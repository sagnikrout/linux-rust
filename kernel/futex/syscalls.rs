//! Automatically rewritten from C to Rust
//! Source: kernel/futex/syscalls.c
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
// Support for robust futexes: the kernel cleans up held futexes at
// thread exit time.
//
// Implementation: user-space maintains a per-thread list of locks it
// is holding. Upon do_exit(), the kernel carefully walks this list,
// and marks all locks that are owned by this thread with the
// FUTEX_OWNER_DIED bit, and wakes up a waiter (if any). The list is
// always manipulated with the lock held, so the list is private and
// per-thread. Userspace also maintains a per-thread 'list_op_pending'
// field, to allow the kernel to clean up if the thread dies after
// acquiring the lock, but just before it could have added itself to
// the list. There can only be one such pending lock.
//
// sys_set_robust_list() - Set the robust-futex list head of a task
// @head:	pointer to the list-head
// @len:	length of the list-head, as userspace expects
//
#[no_mangle]
pub unsafe extern "C" fn sys_set_robust_list(head: usize, len: usize) -> c_long {
// The kernel knows only one size for now.
    if (unlikely(len != sizeof!(*head))) {
    return -EINVAL;
    }
    current.futex.robust_list = head;
    return 0;
    }
    static inline void  *futex_task_robust_list(task_struct *p, bool compat)
    {

    if (compat) {
    return p.futex.compat_robust_list;
    }

    return p.futex.robust_list;
    }
    static void  *futex_get_robust_list_common(int pid, bool compat)
    {
    let mut p = current;
    let mut head = core::ptr::null_mut();
    let mut ret = 0;
    scoped_guard(rcu) {
    if (pid) {
    p = find_task_by_vpid(pid);
    if (!p) {
    return ERR_PTR(-ESRCH);
    }
    }
    get_task_struct(p);
    }
//
// Hold exec_update_lock to serialize with concurrent exec()
// so ptrace_may_access() is checked against stable credentials
//
    ret = down_read_killable(&p.signal.exec_update_lock);
    if (ret) {
// goto;
    }
    ret = -EPERM;
    if (!ptrace_may_access(p, PTRACE_MODE_READ_REALCREDS)) {
// goto;
    }
    head = futex_task_robust_list(p, compat);
    up_read(&p.signal.exec_update_lock);
    put_task_struct(p);
    return head;
// label;
    up_read(&p.signal.exec_update_lock);
// label;
    put_task_struct(p);
    return ERR_PTR(ret);
    }
//
// sys_get_robust_list() - Get the robust-futex list head of a task
// @pid:	pid of the process [zero for current task]
// @head_ptr:	pointer to a list-head pointer, the kernel fills it in
// @len_ptr:	pointer to a length field, the kernel fills in the header size
//
#[no_mangle]
pub unsafe extern "C" fn sys_get_robust_list(pid: usize, head_ptr: usize, len_ptr: usize) -> c_long {
    let mut head = futex_get_robust_list_common(pid, false);
    if (IS_ERR(head)) {
    return PTR_ERR(head);
    }
    if (put_user(sizeof!(*head), len_ptr)) {
    return -EFAULT;
    }
    return put_user(head, head_ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn do_futex(uaddr: *mut u32, op: c_int, val: u32, timeout: *mut ktime_t, uaddr2: *mut u32, val2: u32, val3: u32) -> c_long {
pub static mut flags: c_uint = 0;
pub static mut cmd: c_int = 0;
    if (flags & FLAGS_CLOCKRT) {
    if (cmd != FUTEX_WAIT_BITSET &&
    cmd != FUTEX_WAIT_REQUEUE_PI &&
    cmd != FUTEX_LOCK_PI2) {
    return -ENOSYS;
    }
    }
    if (flags & FLAGS_ROBUST_UNLOCK) {
    if (cmd != FUTEX_WAKE &&
    cmd != FUTEX_WAKE_BITSET &&
    cmd != FUTEX_UNLOCK_PI) {
    return -ENOSYS;
    }
    }
    match (cmd) {
    FUTEX_WAIT => {
    val3 = FUTEX_BITSET_MATCH_ANY;
    fallthrough;
    }
    FUTEX_WAIT_BITSET => {
    return futex_wait(uaddr, flags, val, timeout, val3);
    }
    FUTEX_WAKE => {
    val3 = FUTEX_BITSET_MATCH_ANY;
    fallthrough;
    }
    FUTEX_WAKE_BITSET => {
    return futex_wake(uaddr, flags, uaddr2, val, val3);
    }
    FUTEX_REQUEUE => {
    return futex_requeue(uaddr, flags, uaddr2, flags, val, val2, core::ptr::null_mut(), 0);
    }
    FUTEX_CMP_REQUEUE => {
    return futex_requeue(uaddr, flags, uaddr2, flags, val, val2, &val3, 0);
    }
    FUTEX_WAKE_OP => {
    return futex_wake_op(uaddr, flags, uaddr2, val, val2, val3);
    }
    FUTEX_LOCK_PI => {
    flags |= FLAGS_CLOCKRT;
    fallthrough;
    }
    FUTEX_LOCK_PI2 => {
    return futex_lock_pi(uaddr, flags, timeout, 0);
    }
    FUTEX_UNLOCK_PI => {
    return futex_unlock_pi(uaddr, flags, uaddr2);
    }
    FUTEX_TRYLOCK_PI => {
    return futex_lock_pi(uaddr, flags, core::ptr::null_mut(), 1);
    }
    FUTEX_WAIT_REQUEUE_PI => {
    val3 = FUTEX_BITSET_MATCH_ANY;
    return futex_wait_requeue_pi(uaddr, flags, val, timeout, val3,
    uaddr2);
    }
    FUTEX_CMP_REQUEUE_PI => {
    return futex_requeue(uaddr, flags, uaddr2, flags, val, val2, &val3, 1);
    }
    }
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn futex_cmd_has_timeout(cmd: u32) -> __always_inline bool {
    match (cmd) {
    FUTEX_WAIT => {
    }
    FUTEX_LOCK_PI => {
    }
    FUTEX_LOCK_PI2 => {
    }
    FUTEX_WAIT_BITSET => {
    }
    FUTEX_WAIT_REQUEUE_PI => {
    return true;
    }
    }
    return false;
    }
    static __always_inline int
    futex_init_timeout(u32 cmd, u32 op, timespec64 *ts, ktime_t *t)
    {
    if (!timespec64_valid(ts)) {
    return -EINVAL;
    }
// t = timespec64_to_ktime(*ts);
    if (cmd == FUTEX_WAIT) {
// t = ktime_add_safe(ktime_get(), *t);
    }

    else if (cmd != FUTEX_LOCK_PI && !(op & FUTEX_CLOCK_REALTIME)) {
// t = timens_ktime_to_host(CLOCK_MONOTONIC, *t);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_futex(uaddr: usize, op: usize, val: usize, utime: usize, uaddr2: usize, val3: usize) -> c_long {
    int ret, cmd = op & FUTEX_CMD_MASK;
    ktime_t t, *tp = core::ptr::null_mut();
pub static mut ts: usize = 0;
    if (utime && futex_cmd_has_timeout(cmd)) {
    if (unlikely(should_fail_futex(!(op & FUTEX_PRIVATE_FLAG)))) {
    return -EFAULT;
    }
    if (get_timespec64(&ts, utime)) {
    return -EFAULT;
    }
    ret = futex_init_timeout(cmd, op, &ts, &t);
    if (ret) {
    return ret;
    }
    tp = &t;
    }
    return do_futex(uaddr, op, val, tp, uaddr2, (unsigned long)utime, val3);
    }
//
// futex_parse_waitv - Parse a waitv array from userspace
// @futexv:	Kernel side list of waiters to be filled
// @uwaitv:     Userspace list to be parsed
// @nr_futexes: Length of futexv
// @wake:	Wake to call when futex is woken
// @wake_data:	Data for the wake handler
//
// Return: Error code on failure, 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn futex_parse_waitv(futexv: *mut futex_vector, uwaitv: *mut futex_waitv, nr_futexes: c_uint, wake: *mut futex_wake_fn, wake_data: *mut c_void) -> c_int {
pub static mut aux: usize = 0;
    let mut i = 0;
    while (i < nr_futexes) {
    let mut flags = 0;
    if (copy_from_user(&aux, &uwaitv[i], sizeof!(aux))) {
    return -EFAULT;
    }
    if ((aux.flags & ~FUTEX2_VALID_MASK) || aux.__reserved) {
    return -EINVAL;
    }
    flags = futex2_to_flags(aux.flags);
    if (!futex_flags_valid(flags)) {
    return -EINVAL;
    }
    if (!futex_validate_input(flags, aux.val)) {
    return -EINVAL;
    }
    futexv[i].w.flags = flags;
    futexv[i].w.val = aux.val;
    futexv[i].w.uaddr = aux.uaddr;
    futexv[i].q = futex_q_init;
    futexv[i].q.wake = wake;
    futexv[i].q.wake_data = wake_data;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn futex2_setup_timeout(timeout: *mut __kernel_timespec, clockid: clockid_t, to: *mut hrtimer_sleeper) -> c_int {
pub static mut flag_clkid: c_int = 0;
pub static mut ts: usize = 0;
    let mut time;
    let mut ret = 0;
    if (!timeout) {
    return 0;
    }
    if (clockid == CLOCK_REALTIME) {
    flag_clkid = FLAGS_CLOCKRT;
    flag_init = FUTEX_CLOCK_REALTIME;
    }
    if (clockid != CLOCK_REALTIME && clockid != CLOCK_MONOTONIC) {
    return -EINVAL;
    }
    if (get_timespec64(&ts, timeout)) {
    return -EFAULT;
    }
//
// Since there's no opcode for futex_waitv, use
// FUTEX_WAIT_BITSET that uses absolute timeout as well
//
    ret = futex_init_timeout(FUTEX_WAIT_BITSET, flag_init, &ts, &time);
    if (ret) {
    return ret;
    }
    futex_setup_timer(&time, to, flag_clkid, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn futex2_destroy_timeout(to: *mut hrtimer_sleeper) {
    hrtimer_cancel(&to.timer);
    destroy_hrtimer_on_stack(&to.timer);
    }
//
// sys_futex_waitv - Wait on a list of futexes
// @waiters:    List of futexes to wait on
// @nr_futexes: Length of futexv
// @flags:      Flag for timeout (monotonic/realtime)
// @timeout:	Optional absolute timeout.
// @clockid:	Clock to be used for the timeout, realtime or monotonic.
//
// Given an array of `struct futex_waitv`, wait on each uaddr. The thread wakes
// if a futex_wake() is performed at any uaddr. The syscall returns immediately
// if any waiter has *uaddr != val. *timeout is an optional timeout value for
// the operation. Each waiter has individual flags. The `flags` argument for
// the syscall should be used solely for specifying the timeout as realtime, if
// needed. Flags for private futexes, sizes, etc. should be used on the
// individual flags of each waiter.
//
// Returns the array index of one of the woken futexes. No further information
// is provided: any number of other futexes may also have been woken by the
// same event, and if more than one futex was woken, the retrned index may
// refer to any one of them. (It is not necessaryily the futex with the
// smallest index, nor the one most recently woken, nor...)
//
#[no_mangle]
pub unsafe extern "C" fn sys_futex_waitv(waiters: usize, nr_futexes: usize, flags: usize, timeout: usize, clockid: usize) -> c_long {
pub static mut to: usize = 0;
pub static mut futexv: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// This syscall supports no flags for now
    if (flags) {
    return -EINVAL;
    }
    if (!nr_futexes || nr_futexes > FUTEX_WAITV_MAX || !waiters) {
    return -EINVAL;
    }
    if (timeout && (ret = futex2_setup_timeout(timeout, clockid, &to))) {
    return ret;
    }
    futexv = kzalloc_objs(*futexv, nr_futexes);
    if (!futexv) {
    ret = -ENOMEM;
// goto;
    }
    ret = futex_parse_waitv(futexv, waiters, nr_futexes, futex_wake_mark,
    core::ptr::null_mut());
    if (!ret) {
    ret = futex_wait_multiple(futexv, nr_futexes, timeout ? &to : core::ptr::null_mut());
    }
    kfree(futexv);
// label;
    if (timeout) {
    futex2_destroy_timeout(&to);
    }
    return ret;
    }
//
// sys_futex_wake - Wake a number of futexes
// @uaddr:	Address of the futex(es) to wake
// @mask:	bitmask
// @nr:		Number of the futexes to wake
// @flags:	FUTEX2 flags
//
// Identical to the traditional FUTEX_WAKE_BITSET op, except it is part of the
// futex2 family of calls.
//
#[no_mangle]
pub unsafe extern "C" fn sys_futex_wake(uaddr: usize, mask: usize, nr: usize, flags: usize) -> c_long {
    if (flags & ~FUTEX2_VALID_MASK) {
    return -EINVAL;
    }
    flags = futex2_to_flags(flags);
    if (!futex_flags_valid(flags)) {
    return -EINVAL;
    }
    if (!futex_validate_input(flags, mask)) {
    return -EINVAL;
    }
    return futex_wake(uaddr, FLAGS_STRICT | flags, core::ptr::null_mut(), nr, mask);
    }
//
// sys_futex_wait - Wait on a futex
// @uaddr:	Address of the futex to wait on
// @val:	Value of @uaddr
// @mask:	bitmask
// @flags:	FUTEX2 flags
// @timeout:	Optional absolute timeout
// @clockid:	Clock to be used for the timeout, realtime or monotonic
//
// Identical to the traditional FUTEX_WAIT_BITSET op, except it is part of the
// futex2 familiy of calls.
//
#[no_mangle]
pub unsafe extern "C" fn sys_futex_wait(uaddr: usize, val: usize, mask: usize, flags: usize, timeout: usize, clockid: usize) -> c_long {
pub static mut to: usize = 0;
    let mut ret = 0;
    if (flags & ~FUTEX2_VALID_MASK) {
    return -EINVAL;
    }
    flags = futex2_to_flags(flags);
    if (!futex_flags_valid(flags)) {
    return -EINVAL;
    }
    if (!futex_validate_input(flags, val) ||
    !futex_validate_input(flags, mask)) {
    return -EINVAL;
    }
    if (timeout && (ret = futex2_setup_timeout(timeout, clockid, &to))) {
    return ret;
    }
    ret = __futex_wait(uaddr, flags, val, timeout ? &to : core::ptr::null_mut(), mask);
    if (timeout) {
    futex2_destroy_timeout(&to);
    }
    return ret;
    }
//
// sys_futex_requeue - Requeue a waiter from one futex to another
// @waiters:	array describing the source and destination futex
// @flags:	unused
// @nr_wake:	number of futexes to wake
// @nr_requeue:	number of futexes to requeue
//
// Identical to the traditional FUTEX_CMP_REQUEUE op, except it is part of the
// futex2 family of calls.
//
#[no_mangle]
pub unsafe extern "C" fn sys_futex_requeue(waiters: usize, flags: usize, nr_wake: usize, nr_requeue: usize) -> c_long {
    struct futex_vector futexes[2];
    let mut cmpval = 0;
    let mut ret = 0;
    if (flags) {
    return -EINVAL;
    }
    if (!waiters) {
    return -EINVAL;
    }
    ret = futex_parse_waitv(futexes, waiters, 2, futex_wake_mark, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
//
// For now mandate both flags are identical, like the sys_futex()
// interface has. If/when we merge the variable sized futex support,
// that patch can modify this test to allow a difference in size.
//
    if (futexes[0].w.flags != futexes[1].w.flags) {
    return -EINVAL;
    }
    cmpval = futexes[0].w.val;
    return futex_requeue(u64_to_user_ptr(futexes[0].w.uaddr), futexes[0].w.flags,
// forward_decl: _to_user_ptr;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: sys_set_robust_list
pub unsafe extern "C" fn sys_set_robust_list_dup(head: usize, len: usize) -> c_long {
    if (unlikely(len != sizeof!(*head))) {
    return -EINVAL;
    }
    current.futex.compat_robust_list = head;
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_get_robust_list
pub unsafe extern "C" fn sys_get_robust_list_dup(pid: usize, head_ptr: usize, len_ptr: usize) -> c_long {
    let mut head = futex_get_robust_list_common(pid, true);
    if (IS_ERR(head)) {
    return PTR_ERR(head);
    }
    if (put_user(sizeof!(*head), len_ptr)) {
    return -EFAULT;
    }
    return put_user(ptr_to_compat(head), head_ptr);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_futex_time32(uaddr: usize, op: usize, val: usize, utime: usize, uaddr2: usize, val3: usize) -> c_long {
    int ret, cmd = op & FUTEX_CMD_MASK;
    ktime_t t, *tp = core::ptr::null_mut();
pub static mut ts: usize = 0;
    if (utime && futex_cmd_has_timeout(cmd)) {
    if (get_old_timespec32(&ts, utime)) {
    return -EFAULT;
    }
    ret = futex_init_timeout(cmd, op, &ts, &t);
    if (ret) {
    return ret;
    }
    tp = &t;
    }
    return do_futex(uaddr, op, val, tp, uaddr2, (unsigned long)utime, val3);
    }