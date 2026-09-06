//! Automatically rewritten from C to Rust
//! Source: kernel/entry/common.c
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

// Workaround to allow gradual conversion of architecture code
    void __weak arch_do_signal_or_restart(pt_regs *regs) { }

// TIF bits, which prevent a time slice extension.

//
// Since rseq slice ext has a direct correlation to the worst case
// scheduling latency (schedule is delayed after all), only have it affect
// LAZY reschedules on PREEMPT_RT for now.
//
// However, since this delay is only applicable to userspace, a value
// for rseq_slice_extension_nsec that is strictly less than the worst case
// kernel space preempt_disable() region, should mean the scheduling latency
// is not affected, even for !LAZY.
//
// However, since this value depends on the hardware at hand, it cannot be
// pre-determined in any sensible way. Hence punt on this problem for now.
//

    static __always_inline unsigned long __exit_to_user_mode_loop(pt_regs *regs,
    unsigned long ti_work)
    {
//
// Before returning to user space ensure that all pending work
// items have been completed.
//
    while (ti_work & EXIT_TO_USER_MODE_WORK_LOOP) {
    local_irq_enable();
    if (ti_work & (_TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY)) {
    if (!rseq_grant_slice_extension(ti_work, TIF_SLICE_EXT_DENY)) {
    schedule();
    }
    }
    if (ti_work & _TIF_UPROBE) {
    uprobe_notify_resume(regs);
    }
    if (ti_work & _TIF_PATCH_PENDING) {
    klp_update_patch_state(current);
    }
    if (ti_work & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL)) {
    futex_fixup_robust_unlock(regs);
    arch_do_signal_or_restart(regs);
    }
    if (ti_work & _TIF_NOTIFY_RESUME) {
    resume_user_mode_work(regs);
    }
// Architecture specific TIF work
    arch_exit_to_user_mode_work(regs, ti_work);
//
// Disable interrupts and reevaluate the work flags as they
// might have changed while interrupts and preemption was
// enabled above.
//
    local_irq_disable();
// Check if any of the above work has queued a deferred wakeup
    tick_nohz_user_enter_prepare();
    ti_work = read_thread_flags();
    }
// Return the latest work state for arch_exit_to_user_mode()
    return ti_work;
    }
//
// exit_to_user_mode_loop - do any pending work before leaving to user space
// @regs:	Pointer to pt_regs on entry stack
// @ti_work:	TIF work flags as read by the caller
//
    __always_inline unsigned long exit_to_user_mode_loop(pt_regs *regs,
    unsigned long ti_work)
    {
    for (;;) {
    ti_work = __exit_to_user_mode_loop(regs, ti_work);
    if (likely(!rseq_exit_to_user_mode_restart(regs, ti_work))) {
    return ti_work;
    }
    ti_work = read_thread_flags();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irqentry_enter(regs: *mut pt_regs) -> noinstr irqentry_state_t {
    if (user_mode(regs)) {
    irqentry_state_t ret = {
    .exit_rcu = false,
    };
    irqentry_enter_from_user_mode(regs);
    return ret;
    }
    return irqentry_enter_from_kernel_mode(regs);
    }
//
// arch_irqentry_exit_need_resched - Architecture specific need resched function
//
// Invoked from raw_irqentry_exit_cond_resched() to check if resched is needed.
// Defaults return true.
//
// The main purpose is to permit arch to avoid preemption of a task from an IRQ.
//
// forward_decl: arch_irqentry_exit_need_resched;

#[no_mangle]
pub unsafe extern "C" fn arch_irqentry_exit_need_resched() -> bool { return true; }

#[no_mangle]
pub unsafe extern "C" fn raw_irqentry_exit_cond_resched() {
    if (!preempt_count()) {
// Sanity check RCU and thread stack
    rcu_irq_exit_check_preempt();
    if (IS_ENABLED!(CONFIG_DEBUG_ENTRY)) {
    WARN_ON_ONCE!(!on_thread_stack());
    }
    if (need_resched() && arch_irqentry_exit_need_resched()) {
    preempt_schedule_irq();
    }
    }
    }

pub static mut irqentry_exit_cond_resched: usize = 0;

pub static mut sk_dynamic_irqentry_exit_cond_resched: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn dynamic_irqentry_exit_cond_resched() {
    if (!static_branch_unlikely(&sk_dynamic_irqentry_exit_cond_resched)) {
    return;
    }
    raw_irqentry_exit_cond_resched();
    }

#[no_mangle]
pub unsafe extern "C" fn irqentry_exit(regs: *mut pt_regs, state: irqentry_state_t) -> noinstr void {
    if (user_mode(regs)) {
    irqentry_exit_to_user_mode(regs);
    }
    else {
    irqentry_exit_to_kernel_mode(regs, state);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irqentry_nmi_enter(regs: *mut pt_regs) -> irqentry_state_t noinstr {
    let mut irq_state;
    irq_state.lockdep = lockdep_hardirqs_enabled();
    __nmi_enter();
    lockdep_hardirqs_off(CALLER_ADDR0);
    lockdep_hardirq_enter();
    ct_nmi_enter();
    instrumentation_begin();
    kmsan_unpoison_entry_regs(regs);
    trace_hardirqs_off_finish();
    ftrace_nmi_enter();
    instrumentation_end();
    return irq_state;
    }
#[no_mangle]
pub unsafe extern "C" fn irqentry_nmi_exit(regs: *mut pt_regs, irq_state: irqentry_state_t) -> void noinstr {
    instrumentation_begin();
    ftrace_nmi_exit();
    if (irq_state.lockdep) {
    trace_hardirqs_on_prepare();
    lockdep_hardirqs_on_prepare();
    }
    instrumentation_end();
    ct_nmi_exit();
    lockdep_hardirq_exit();
    if (irq_state.lockdep) {
    lockdep_hardirqs_on(CALLER_ADDR0);
    }
    __nmi_exit();
    }