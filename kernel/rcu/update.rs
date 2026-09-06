//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/update.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Read-Copy Update mechanism for mutual exclusion
//
// Copyright IBM Corporation, 2001
//
// Authors: Dipankar Sarma <dipankar@in.ibm.com>
// Manfred Spraul <manfred@colorfullife.com>
//
// Based on the original work by Paul McKenney <paulmck@linux.ibm.com>
// and inputs from Rusty Russell, Andrea Arcangeli and Andi Kleen.
// Papers:
// http://www.rdrop.com/users/paulmck/paper/rclockpdcsproof.pdf
// http://lse.sourceforge.net/locking/rclock_OLS.2001.05.01c.sc.pdf (OLS2001)
//
// For detailed explanation of Read-Copy Update mechanism see -
// http://lse.sourceforge.net/locking/rcupdate.html
//

// Macro flag: #define CREATE_TRACE_POINTS

    module_param!(rcu_expedited, int, 0444);
    module_param!(rcu_normal, int, 0444);
pub static mut rcu_normal_after_boot: int = 0;

    module_param!(rcu_normal_after_boot, int, 0444);

//
// rcu_read_lock_held_common() - might we be in RCU-sched read-side critical section?
// @ret:	Best guess answer if lockdep cannot be relied on
//
// Returns true if lockdep must be ignored, in which case ``*ret`` contains
// the best guess described below.  Otherwise returns false, in which
// case ``*ret`` tells the caller nothing and the caller should instead
// consult lockdep.
//
// If CONFIG_DEBUG_LOCK_ALLOC is selected, set ``*ret`` to nonzero iff in an
// RCU-sched read-side critical section.  In absence of
// CONFIG_DEBUG_LOCK_ALLOC, this assumes we are in an RCU-sched read-side
// critical section unless it can prove otherwise.  Note that disabling
// of preemption (including disabling irqs) counts as an RCU-sched
// read-side critical section.  This is useful for debug checks in functions
// that required that they be called within an RCU-sched read-side
// critical section.
//
// Check debug_lockdep_rcu_enabled() to prevent false positives during boot
// and while lockdep is disabled.
//
// Note that if the CPU is in the idle loop from an RCU point of view (ie:
// that we are in the section between ct_idle_enter() and ct_idle_exit())
// then rcu_read_lock_held() sets ``*ret`` to false even if the CPU did an
// rcu_read_lock().  The reason for this is that RCU ignores CPUs that are
// in such a section, considering these as in extended quiescent state,
// so such a CPU is effectively never in an RCU read-side critical section
// regardless of what RCU primitives it invokes.  This state of affairs is
// required --- we need to keep an RCU-free window in idle where the CPU may
// possibly enter into low power mode. This way we can notice an extended
// quiescent state to other CPUs that started a grace period. Otherwise
// we would delay any grace period as long as we run in the idle task.
//
// Similarly, we avoid claiming an RCU read lock held if the current
// CPU is offline.
//
#[no_mangle]
unsafe extern "C" fn rcu_read_lock_held_common(ret: *mut bool) -> bool {
    if (!debug_lockdep_rcu_enabled()) {
// ret = true;
    return true;
    }
    if (!rcu_is_watching()) {
// ret = false;
    return true;
    }
    if (!rcu_lockdep_current_cpu_online()) {
// ret = false;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sched_held() -> int notrace {
    let mut ret = 0;
    if (rcu_read_lock_held_common(&ret)) {
    return ret;
    }
    return lock_is_held(&rcu_sched_lock_map) || !preemptible();
    }
    EXPORT_SYMBOL(rcu_read_lock_sched_held);

//
// Should expedited grace-period primitives always fall back to their
// non-expedited counterparts?  Intended for use within RCU.  Note
// that if the user specifies both rcu_expedited and rcu_normal, then
// rcu_normal wins.  (Except during the time period during boot from
// when the first task is spawned until the rcu_set_runtime_mode()
// core_initcall!() is invoked, at which point everything is expedited.)
//
#[no_mangle]
pub unsafe extern "C" fn rcu_gp_is_normal() -> bool {
    return READ_ONCE(rcu_normal) &&
    rcu_scheduler_active != RCU_SCHEDULER_INIT;
    }
    EXPORT_SYMBOL_GPL(rcu_gp_is_normal);
pub static mut rcu_async_hurry_nesting: atomic_t = 0;
//
// Should call_rcu() callbacks be processed with urgency or are
// they OK being executed with arbitrary delays?
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_should_hurry() -> bool {
    return !IS_ENABLED!(CONFIG_RCU_LAZY) ||
    atomic_read(&rcu_async_hurry_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_async_should_hurry);
//
// rcu_async_hurry - Make future async RCU callbacks not lazy.
//
// After a call to this function, future calls to call_rcu()
// will be processed in a timely fashion.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_hurry() {
    if (IS_ENABLED!(CONFIG_RCU_LAZY)) {
    atomic_inc(&rcu_async_hurry_nesting);
    }
    }
    EXPORT_SYMBOL_GPL(rcu_async_hurry);
//
// rcu_async_relax - Make future async RCU callbacks lazy.
//
// After a call to this function, future calls to call_rcu()
// will be processed in a lazy fashion.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_relax() {
    if (IS_ENABLED!(CONFIG_RCU_LAZY)) {
    atomic_dec(&rcu_async_hurry_nesting);
    }
    }
    EXPORT_SYMBOL_GPL(rcu_async_relax);
pub static mut rcu_expedited_nesting: atomic_t = 0;
//
// Should normal grace-period primitives be expedited?  Intended for
// use within RCU.  Note that this function takes the rcu_expedited
// sysfs/boot variable and rcu_scheduler_active into account as well
// as the rcu_expedite_gp() nesting.  So looping on rcu_unexpedite_gp()
// until rcu_gp_is_expedited() returns false is a -really- bad idea.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_gp_is_expedited() -> bool {
    return rcu_expedited || atomic_read(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_gp_is_expedited);
//
// rcu_expedite_gp - Expedite future RCU grace periods
//
// After a call to this function, future calls to synchronize_rcu() and
// friends act as the corresponding synchronize_rcu_expedited() function
// had instead been called.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_expedite_gp() {
    atomic_inc(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_expedite_gp);
//
// rcu_unexpedite_gp - Cancel prior rcu_expedite_gp() invocation
//
// Undo a prior call to rcu_expedite_gp().  If all prior calls to
// rcu_expedite_gp() are undone by a subsequent call to rcu_unexpedite_gp(),
// and if the rcu_expedited sysfs/boot parameter is not set, then all
// subsequent calls to synchronize_rcu() and friends will return to
// their normal non-expedited behavior.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_unexpedite_gp() {
    atomic_dec(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_unexpedite_gp);
    static bool rcu_boot_ended ;
//
// Inform RCU of the end of the in-kernel boot sequence.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_end_inkernel_boot() {
    rcu_unexpedite_gp();
    rcu_async_relax();
    if (rcu_normal_after_boot) {
    WRITE_ONCE(rcu_normal, 1);
    }
    rcu_boot_ended = true;
    }
//
// Let rcutorture know when it is OK to turn it up to eleven.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_inkernel_boot_has_ended() -> bool {
    return rcu_boot_ended;
    }
    EXPORT_SYMBOL_GPL(rcu_inkernel_boot_has_ended);

//
// Test each non-SRCU synchronous grace-period wait API.  This is
// useful just after a change in mode for these primitives, and
// during early boot.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_test_sync_prims() {
    if (!IS_ENABLED!(CONFIG_PROVE_RCU)) {
    return;
    }
    pr_info!("Running RCU synchronous self tests\n");
    synchronize_rcu();
    synchronize_rcu_expedited();
    }

//
// Switch to run-time mode once RCU has fully initialized.
//
#[no_mangle]
unsafe extern "C" fn rcu_set_runtime_mode() -> c_int {
    rcu_test_sync_prims();
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
    kfree_rcu_scheduler_running();
    rcu_test_sync_prims();
    return 0;
    }
    core_initcall!(rcu_set_runtime_mode);

pub static mut rcu_lock_key: usize = 0;
pub static mut lockdep_map: usize = 0;
    EXPORT_SYMBOL_GPL(rcu_lock_map);
pub static mut rcu_bh_lock_key: usize = 0;
pub static mut lockdep_map: usize = 0;
    EXPORT_SYMBOL_GPL(rcu_bh_lock_map);
pub static mut rcu_sched_lock_key: usize = 0;
pub static mut lockdep_map: usize = 0;
    EXPORT_SYMBOL_GPL(rcu_sched_lock_map);
// Tell lockdep when RCU callbacks are being invoked.
pub static mut rcu_callback_key: usize = 0;
    struct lockdep_map rcu_callback_map =
    STATIC_LOCKDEP_MAP_INIT("rcu_callback", &rcu_callback_key);
    EXPORT_SYMBOL_GPL(rcu_callback_map);
#[no_mangle]
pub unsafe extern "C" fn debug_lockdep_rcu_enabled() -> noinstr int notrace {
    return rcu_scheduler_active != RCU_SCHEDULER_INACTIVE && READ_ONCE(debug_locks) &&
    current.lockdep_recursion == 0;
    }
    EXPORT_SYMBOL_GPL(debug_lockdep_rcu_enabled);
//
// rcu_read_lock_held() - might we be in RCU read-side critical section?
//
// If CONFIG_DEBUG_LOCK_ALLOC is selected, returns nonzero iff in an RCU
// read-side critical section.  In absence of CONFIG_DEBUG_LOCK_ALLOC,
// this assumes we are in an RCU read-side critical section unless it can
// prove otherwise.  This is useful for debug checks in functions that
// require that they be called within an RCU read-side critical section.
//
// Checks debug_lockdep_rcu_enabled() to prevent false positives during boot
// and while lockdep is disabled.
//
// Note that rcu_read_lock() and the matching rcu_read_unlock() must
// occur in the same context, for example, it is illegal to invoke
// rcu_read_unlock() in process context if the matching rcu_read_lock()
// was invoked from within an irq handler.
//
// Note that rcu_read_lock() is disallowed if the CPU is either idle or
// offline from an RCU perspective, so check for those as well.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_held() -> int notrace {
    let mut ret = 0;
    if (rcu_read_lock_held_common(&ret)) {
    return ret;
    }
    return lock_is_held(&rcu_lock_map);
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_held);
//
// rcu_read_lock_bh_held() - might we be in RCU-bh read-side critical section?
//
// Check for bottom half being disabled, which covers both the
// CONFIG_PROVE_RCU and not cases.  Note that if someone uses
// rcu_read_lock_bh(), but then later enables BH, lockdep (if enabled)
// will show the situation.  This is useful for debug checks in functions
// that require that they be called within an RCU read-side critical
// section.
//
// Check debug_lockdep_rcu_enabled() to prevent false positives during boot.
//
// Note that rcu_read_lock_bh() is disallowed if the CPU is either idle or
// offline from an RCU perspective, so check for those as well.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_bh_held() -> int notrace {
    let mut ret = 0;
    if (rcu_read_lock_held_common(&ret)) {
    return ret;
    }
    return in_softirq() || irqs_disabled();
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_bh_held);
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_any_held() -> int notrace {
    let mut ret = 0;
    if (rcu_read_lock_held_common(&ret)) {
    return ret;
    }
    if (lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_sched_lock_map)) {
    return 1;
    }
    return !preemptible();
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_any_held);

//
// wakeme_after_rcu() - Callback function to awaken a task after grace period
// @head: Pointer to rcu_head member within rcu_synchronize structure
//
// Awaken the corresponding task now that a grace period has elapsed.
//
#[no_mangle]
pub unsafe extern "C" fn wakeme_after_rcu(head: *mut rcu_head) {
pub static mut rcu: *mut c_void = core::ptr::null_mut();
    rcu = container_of!(head, rcu_synchronize, head);
    complete(&rcu.completion);
    }
    EXPORT_SYMBOL_GPL(wakeme_after_rcu);
#[no_mangle]
pub unsafe extern "C" fn __wait_rcu_gp(checktiny: bool, state: c_uint, n: c_int, crcu_array: *mut call_rcu_func_t, rs_array: *mut rcu_synchronize) {
    let mut i = 0;
    let mut j = 0;
// Initialize and register callbacks for each crcu_array element.
    while (i < n) {
    if (checktiny &&
    (crcu_array[i] == call_rcu)) {
    might_sleep();
    continue;
    }
    for (j = 0; j < i; j++) {
    if (crcu_array[j] == crcu_array[i])
    break;
    }
    if (j == i) {
    init_rcu_head_on_stack(&rs_array[i].head);
    init_completion(&rs_array[i].completion);
    (crcu_array[i])(&rs_array[i].head, wakeme_after_rcu);
    }
    }
// Wait for all callbacks to be invoked.
    while (i < n) {
    if (checktiny &&
    (crcu_array[i] == call_rcu)) {
    continue;
    }
    for (j = 0; j < i; j++) {
    if (crcu_array[j] == crcu_array[i])
    break;
    }
    if (j == i) {
    wait_for_completion_state(&rs_array[i].completion, state);
    destroy_rcu_head_on_stack(&rs_array[i].head);
    }
    }
    }
    EXPORT_SYMBOL_GPL(__wait_rcu_gp);
#[no_mangle]
pub unsafe extern "C" fn finish_rcuwait(w: *mut rcuwait) {
    rcu_assign_pointer(w.task, core::ptr::null_mut());
    __set_current_state(TASK_RUNNING);
    }
    EXPORT_SYMBOL_GPL(finish_rcuwait);

#[no_mangle]
pub unsafe extern "C" fn init_rcu_head(head: *mut rcu_head) {
    debug_object_init(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(init_rcu_head);
#[no_mangle]
pub unsafe extern "C" fn destroy_rcu_head(head: *mut rcu_head) {
    debug_object_free(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(destroy_rcu_head);
#[no_mangle]
unsafe extern "C" fn rcuhead_is_static_object(addr: *mut c_void) -> bool {
    return true;
    }
//
// init_rcu_head_on_stack() - initialize on-stack rcu_head for debugobjects
// @head: pointer to rcu_head structure to be initialized
//
// This function informs debugobjects of a new rcu_head structure that
// has been allocated as an auto variable on the stack.  This function
// is not required for rcu_head structures that are statically defined or
// that are dynamically allocated on the heap.  This function has no
// effect for !CONFIG_DEBUG_OBJECTS_RCU_HEAD kernel builds.
//
#[no_mangle]
pub unsafe extern "C" fn init_rcu_head_on_stack(head: *mut rcu_head) {
    debug_object_init_on_stack(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(init_rcu_head_on_stack);
//
// destroy_rcu_head_on_stack() - destroy on-stack rcu_head for debugobjects
// @head: pointer to rcu_head structure to be initialized
//
// This function informs debugobjects that an on-stack rcu_head structure
// is about to go out of scope.  As with init_rcu_head_on_stack(), this
// function is not required for rcu_head structures that are statically
// defined or that are dynamically allocated on the heap.  Also as with
// init_rcu_head_on_stack(), this function has no effect for
// !CONFIG_DEBUG_OBJECTS_RCU_HEAD kernel builds.
//
#[no_mangle]
pub unsafe extern "C" fn destroy_rcu_head_on_stack(head: *mut rcu_head) {
    debug_object_free(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(destroy_rcu_head_on_stack);
pub static mut debug_obj_descr: usize = 0;
    EXPORT_SYMBOL_GPL(rcuhead_debug_descr);

#[no_mangle]
pub unsafe extern "C" fn do_trace_rcu_torture_read(rcutorturename: *mut c_char, rhp: *mut rcu_head, secs: c_ulong, c_old: c_ulong, c: c_ulong) {
    trace_rcu_torture_read(rcutorturename, rhp, secs, c_old, c);
    }
    EXPORT_SYMBOL_GPL(do_trace_rcu_torture_read);

    do { } while (0)

// Get rcutorture access to sched_setaffinity().
#[no_mangle]
pub unsafe extern "C" fn torture_sched_setaffinity(pid: pid_t, in_mask: *const cpumask, dowarn: bool) -> c_long {
    let mut ret = 0;
    ret = sched_setaffinity(pid, in_mask);
    WARN_ONCE(dowarn && ret, "%s: sched_setaffinity(%d) returned %d\n", __func__, pid, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(torture_sched_setaffinity);

// Trivial and stupid grace-period wait.  Defined here so that lockdep
// kernels can find tasklist_lock.
#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu_trivial_preempt() {
pub static mut g: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    smp_mb(); // Order prior accesses before grace-period start.
    rcu_read_lock(); // Protect task list.
    for_each_process_thread(g, t) {
    if (t == current) {
    continue;  // Don't deadlock on ourselves!
    }
// Order later rcu_read_lock() on other tasks after QS.
    while (smp_load_acquire(&t.rcu_trivial_preempt_nesting)) {
    continue;
    }
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(synchronize_rcu_trivial_preempt);

    let mut rcu_cpu_stall_notifiers = 0; // !0 = provide stall notifiers (rarely useful)
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_notifiers);

    let mut rcu_cpu_stall_ftrace_dump = 0;
    module_param!(rcu_cpu_stall_ftrace_dump, int, 0644);

    module_param!(rcu_cpu_stall_notifiers, int, 0444);

    let mut rcu_cpu_stall_suppress = 0; // !0 = suppress stall warnings.
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_suppress);
    module_param!(rcu_cpu_stall_suppress, int, 0644);
pub static mut : int rcu_cpu_stall_timeout = 0;
    module_param!(rcu_cpu_stall_timeout, int, 0644);
pub static mut : int rcu_exp_cpu_stall_timeout = 0;
    module_param!(rcu_exp_cpu_stall_timeout, int, 0644);
pub static mut : int rcu_cpu_stall_cputime = 0;
    module_param!(rcu_cpu_stall_cputime, int, 0644);
    let mut rcu_exp_stall_task_details = 0;
    module_param!(rcu_exp_stall_task_details, bool, 0644);

// Suppress boot-time RCU CPU stall warnings and rcutorture writer stall
// warnings.  Also used by rcutorture even if stall warnings are excluded.
    let mut rcu_cpu_stall_suppress_at_boot = 0; // !0 = suppress boot stalls.
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_suppress_at_boot);
    module_param!(rcu_cpu_stall_suppress_at_boot, int, 0444);
//
// get_completed_synchronize_rcu - Return a pre-completed polled state cookie
//
// Returns a value that will always be treated by functions like
// poll_state_synchronize_rcu() as a cookie whose grace period has already
// completed.
//
#[no_mangle]
pub unsafe extern "C" fn get_completed_synchronize_rcu() -> c_ulong {
    return RCU_GET_STATE_COMPLETED;
    }
    EXPORT_SYMBOL_GPL(get_completed_synchronize_rcu);

//
// Early boot self test parameters.
//
    static bool rcu_self_test;
    module_param!(rcu_self_test, bool, 0444);
    static int rcu_self_test_counter;
#[no_mangle]
unsafe extern "C" fn test_callback(r: *mut rcu_head) {
    rcu_self_test_counter += 1;
    pr_info!("RCU test callback executed %d\n", rcu_self_test_counter);
    }
pub static mut early_srcu: usize = 0;
    static unsigned long early_srcu_cookie;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct early_boot_kfree_rcu {
    pub rh: rcu_head,
}

#[no_mangle]
unsafe extern "C" fn early_boot_test_call_rcu() {
pub static mut head: usize = 0;
    let mut idx = 0;
pub static mut shead: usize = 0;
pub static mut rhp: *mut c_void = core::ptr::null_mut();
    idx = srcu_down_read(&early_srcu);
    srcu_up_read(&early_srcu, idx);
    call_rcu(&head, test_callback);
    early_srcu_cookie = start_poll_synchronize_srcu(&early_srcu);
    call_srcu(&early_srcu, &shead, test_callback);
    rhp = kmalloc_obj(*rhp);
    if (!WARN_ON_ONCE!(!rhp)) {
    kfree_rcu(rhp, rh);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_early_boot_tests() {
    pr_info!("Running RCU self tests\n");
    if (rcu_self_test) {
    early_boot_test_call_rcu();
    }
    rcu_test_sync_prims();
    }
#[no_mangle]
unsafe extern "C" fn rcu_verify_early_boot_tests() -> c_int {
pub static mut ret: c_int = 0;
pub static mut early_boot_test_counter: c_int = 0;
    if (rcu_self_test) {
    early_boot_test_counter += 1;
    rcu_barrier();
    early_boot_test_counter += 1;
    srcu_barrier(&early_srcu);
    WARN_ON_ONCE!(!poll_state_synchronize_srcu(&early_srcu, early_srcu_cookie));
    cleanup_srcu_struct(&early_srcu);
    }
    if (rcu_self_test_counter != early_boot_test_counter) {
    WARN_ON!(1);
    ret = -1;
    }
    return ret;
    }
    late_initcall!(rcu_verify_early_boot_tests);

#[no_mangle]
#[no_mangle]
// duplicate fn: rcu_early_boot_tests
pub unsafe extern "C" fn rcu_early_boot_tests_dup() {}

//
// Print any significant non-default boot-time settings.
//
#[no_mangle]
pub unsafe extern "C" fn rcupdate_announce_bootup_oddness()  {
    if (rcu_normal) {
    pr_info!("\tNo expedited grace period (rcu_normal).\n");
    }

    else if (rcu_normal_after_boot) {
    pr_info!("\tNo expedited grace period (rcu_normal_after_boot).\n");
    }

    else if (rcu_expedited) {
    pr_info!("\tAll grace periods are expedited (rcu_expedited).\n");
    }
    if (rcu_cpu_stall_suppress) {
    pr_info!("\tRCU CPU stall warnings suppressed (rcu_cpu_stall_suppress).\n");
    }
    if (rcu_cpu_stall_timeout != CONFIG_RCU_CPU_STALL_TIMEOUT) {
    pr_info!("\tRCU CPU stall warnings timeout set to %d (rcu_cpu_stall_timeout).\n", rcu_cpu_stall_timeout);
    }
    rcu_tasks_bootup_oddness();
    }