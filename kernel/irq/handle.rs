//! Automatically rewritten from C to Rust
//! Source: kernel/irq/handle.c
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
// Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
// Copyright (C) 2005-2006, Thomas Gleixner, Russell King
//
// This file contains the core interrupt handling code. Detailed
// information is available in Documentation/core-api/genericirq.rst
//

    void (*handle_arch_irq) __ro_after_init;

//
// handle_bad_irq - handle spurious and unhandled irqs
// @desc:      description of the interrupt
//
// Handles spurious and unhandled IRQ's. It also prints a debugmessage.
//
#[no_mangle]
pub unsafe extern "C" fn handle_bad_irq(desc: *mut irq_desc) {
pub static mut irq: c_uint = 0;
    print_irq_desc(irq, desc);
    kstat_incr_irqs_this_cpu(desc);
    ack_bad_irq(irq);
    }
    EXPORT_SYMBOL_GPL(handle_bad_irq);
//
// Special, empty irq handler:
//
#[no_mangle]
pub unsafe extern "C" fn no_action(cpl: c_int, dev_id: *mut c_void) -> irqreturn_t {
    return IRQ_NONE;
    }
    EXPORT_SYMBOL_GPL(no_action);
#[no_mangle]
unsafe extern "C" fn warn_no_thread(irq: c_uint, action: *mut irqaction) {
    if (test_and_set_bit(IRQTF_WARNED, &action.thread_flags)) {
    return;
    }
    printk("IRQ %d device %s returned IRQ_WAKE_THREAD "
    "but no thread function available.", irq, action.name);
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction) {
//
// In case the thread crashed and was killed we just pretend that
// we handled the interrupt. The hardirq handler has disabled the
// device interrupt, so no irq storm is lurking.
//
    if (action.thread.flags & PF_EXITING) {
    return;
    }
//
// Wake up the handler thread for this action. If the
// RUNTHREAD bit is already set, nothing to do.
//
    if (test_and_set_bit(IRQTF_RUNTHREAD, &action.thread_flags)) {
    return;
    }
//
// It's safe to OR the mask lockless here. We have only two
// places which write to threads_oneshot: This code and the
// irq thread.
//
// This code is the hard irq context and can never run on two
// cpus in parallel. If it ever does we have more serious
// problems than this bitmask.
//
// The irq threads of this irq which clear their "running" bit
// in threads_oneshot are serialized via desc->lock against
// each other and they are serialized against this code by
// IRQS_INPROGRESS.
//
// Hard irq handler:
//
// spin_lock(desc->lock);
// desc->state |= IRQS_INPROGRESS;
// spin_unlock(desc->lock);
// set_bit(IRQTF_RUNTHREAD, &action->thread_flags);
// desc->threads_oneshot |= mask;
// spin_lock(desc->lock);
// desc->state &= ~IRQS_INPROGRESS;
// spin_unlock(desc->lock);
//
// irq thread:
//
// again:
// spin_lock(desc->lock);
// if (desc->state & IRQS_INPROGRESS) {
// spin_unlock(desc->lock);
// while(desc->state & IRQS_INPROGRESS)
// cpu_relax();
// goto again;
// }
// if (!test_bit(IRQTF_RUNTHREAD, &action->thread_flags))
// desc->threads_oneshot &= ~mask;
// spin_unlock(desc->lock);
//
// So either the thread waits for us to clear IRQS_INPROGRESS
// or we are waiting in the flow handler for desc->lock to be
// released before we reach this point. The thread also checks
// IRQTF_RUNTHREAD under desc->lock. If set it leaves
// threads_oneshot untouched and runs the thread another time.
//
    desc.threads_oneshot |= action.thread_mask;
//
// We increment the threads_active counter in case we wake up
// the irq thread. The irq thread decrements the counter when
// it returns from the handler or in the exit path and wakes
// up waiters which are stuck in synchronize_irq() when the
// active count becomes zero. synchronize_irq() is serialized
// against this code (hard irq handler) via IRQS_INPROGRESS
// like the finalize_oneshot() code. See comment above.
//
    atomic_inc(&desc.threads_active);
//
// This might be a premature wakeup before the thread reached the
// thread function and set the IRQTF_READY bit. It's waiting in
// kthread code with state UNINTERRUPTIBLE. Once it reaches the
// thread function it waits with INTERRUPTIBLE. The wakeup is not
// lost in that case because the thread is guaranteed to observe
// the RUN flag before it goes to sleep in wait_for_interrupt().
//
    wake_up_state(action.thread, TASK_INTERRUPTIBLE);
    }
pub static mut irqhandler_duration_check_enabled: usize = 0;
    static u64 irqhandler_duration_threshold_ns __ro_after_init;
#[no_mangle]
unsafe extern "C" fn irqhandler_duration_check_setup(arg: *mut c_char) -> c_int {
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul(arg, 0, &val);
    if (ret) {
    pr_err!("Unable to parse irqhandler.duration_warn_us setting: ret=%d\n", ret);
    return 0;
    }
    if (!val) {
    pr_err!("Invalid irqhandler.duration_warn_us setting, must be > 0\n");
    return 0;
    }
    irqhandler_duration_threshold_ns = val * 1000;
    static_branch_enable(&irqhandler_duration_check_enabled);
    return 1;
    }
    __setup!("irqhandler.duration_warn_us=", irqhandler_duration_check_setup);
#[no_mangle]
pub unsafe extern "C" fn irqhandler_duration_check(ts_start: u64, irq: c_uint, action: *mut irqaction) {
pub static mut delta_ns: u64 = 0;
    if (unlikely(delta_ns > irqhandler_duration_threshold_ns)) {
    pr_warn_ratelimited("[CPU%u] long duration of IRQ[%u:%ps], took: %llu us\n",
    smp_processor_id(), irq, action.handler,
    div_u64(delta_ns, NSEC_PER_USEC));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t {
pub static mut retval: irqreturn_t = 0;
pub static mut irq: c_uint = 0;
pub static mut action: *mut c_void = core::ptr::null_mut();
    for_each_action_of_desc(desc, action) {
    let mut res;
//
// If this IRQ would be threaded under force_irqthreads, mark it so.
//
    if (irq_settings_can_thread(desc) &&
    !(action.flags & (IRQF_NO_THREAD | IRQF_PERCPU | IRQF_ONESHOT))) {
    lockdep_hardirq_threaded();
    }
    trace_irq_handler_entry(irq, action);
    if (static_branch_unlikely(&irqhandler_duration_check_enabled)) {
pub static mut ts_start: u64 = 0;
    res = action.handler(irq, action.dev_id);
    irqhandler_duration_check(ts_start, irq, action);
    } else {
    res = action.handler(irq, action.dev_id);
    }
    trace_irq_handler_exit(irq, action, res);
    if (WARN_ONCE(!irqs_disabled(),"irq %u handler %pS enabled interrupts\n",
    irq, action.handler)) {
    local_irq_disable();
    }
    match (res) {
    IRQ_WAKE_THREAD => {
//
// Catch drivers which return WAKE_THREAD but
// did not set up a thread function
//
    if (unlikely(!action.thread_fn)) {
    warn_no_thread(irq, action);
    // break;
    }
    __irq_wake_thread(desc, action);
    // break;
    }
    _ => {
    // break;
    }
    }
    retval |= res;
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t {
    let mut retval;
    retval = __handle_irq_event_percpu(desc);
    add_interrupt_randomness(desc.irq_data.irq);
    if (!irq_settings_no_debug(desc)) {
    note_interrupt(desc, retval);
    }
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_irq_event(desc: *mut irq_desc) -> irqreturn_t {
    let mut ret;
    desc.istate &= ~IRQS_PENDING;
    irqd_set(&desc.irq_data, IRQD_IRQ_INPROGRESS);
    raw_spin_unlock(&desc.lock);
    ret = handle_irq_event_percpu(desc);
    raw_spin_lock(&desc.lock);
    irqd_clear(&desc.irq_data, IRQD_IRQ_INPROGRESS);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn set_handle_irq(): *mut *mut c_void (handle_irq)(pt_regs) -> c_int {
    int __init set_handle_irq(void (*handle_irq))
    {
    if (handle_arch_irq) {
    return -EBUSY;
    }
    handle_arch_irq = handle_irq;
    return 0;
    }
//
// generic_handle_arch_irq - root irq handler for architectures which do no
// entry accounting themselves
// @regs:	Register file coming from the low-level handling code
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_arch_irq(regs: *mut pt_regs) -> asmlinkage void noinstr {
pub static mut old_regs: *mut c_void = core::ptr::null_mut();
    irq_enter();
    old_regs = set_irq_regs(regs);
    handle_arch_irq(regs);
    set_irq_regs(old_regs);
    irq_exit();
    }
}
