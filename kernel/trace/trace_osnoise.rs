//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_osnoise.c
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
// OS Noise Tracer: computes the OS Noise suffered by a running thread.
// Timerlat Tracer: measures the wakeup latency of a timer triggered IRQ and thread.
//
// Based on "hwlat_detector" tracer by:
// Copyright (C) 2008-2009 Jon Masters, Red Hat, Inc. <jcm@redhat.com>
// Copyright (C) 2013-2016 Steven Rostedt, Red Hat, Inc. <srostedt@redhat.com>
// With feedback from Clark Williams <williams@redhat.com>
//
// And also based on the rtsl tracer presented on:
// DE OLIVEIRA, Daniel Bristot, et al. Demystifying the real-time linux
// scheduling latency. In: 32nd Euromicro Conference on Real-Time Systems
// (ECRTS 2020). Schloss Dagstuhl-Leibniz-Zentrum fur Informatik, 2020.
//
// Copyright (C) 2021 Daniel Bristot de Oliveira, Red Hat, Inc. <bristot@redhat.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// Default values.
//

//
// osnoise/options entries.
//
    enum osnoise_options_index {
    OSN_DEFAULTS = 0,
    OSN_WORKLOAD,
    OSN_PANIC_ON_STOP,
    OSN_PREEMPT_DISABLE,
    OSN_IRQ_DISABLE,
    OSN_TIMERLAT_ALIGN,
    OSN_MAX
    };
    static const char * const osnoise_options_str[OSN_MAX] = {
    "DEFAULTS",
    "OSNOISE_WORKLOAD",
    "PANIC_ON_STOP",
    "OSNOISE_PREEMPT_DISABLE",
    "OSNOISE_IRQ_DISABLE",
    "TIMERLAT_ALIGN" };
pub const OSN_DEFAULT_OPTIONS: c_uint = 0x2;
pub static mut osnoise_options: unsigned long = 0;
//
// trace_array of the enabled osnoise/timerlat instances.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_instance {
    pub list: list_head,
    pub tr: *mut trace_array,
}

pub static mut osnoise_instances: usize = 0;
#[no_mangle]
unsafe extern "C" fn osnoise_print(fmt: *const c_char, ...) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut ap;
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    tr = inst.tr;
    va_start(ap, fmt);
    trace_array_vprintk(tr, _RET_IP_, fmt, ap);
    va_end(ap);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn osnoise_has_registered_instances() -> bool {
    return !!list_first_or_null_rcu(&osnoise_instances, osnoise_instance,
    list);
    }
//
// osnoise_instance_registered - check if a tr is already registered
//
#[no_mangle]
unsafe extern "C" fn osnoise_instance_registered(tr: *mut trace_array) -> c_int {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut found: c_int = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    if (inst.tr == tr) {
    found = 1;
    }
    }
    rcu_read_unlock();
    return found;
    }
//
// osnoise_register_instance - register a new trace instance
//
// Register a trace_array *tr in the list of instances running
// osnoise/timerlat tracers.
//
#[no_mangle]
unsafe extern "C" fn osnoise_register_instance(tr: *mut trace_array) -> c_int {
pub static mut inst: *mut c_void = core::ptr::null_mut();
//
// register/unregister serialization is provided by trace's
// trace_types_lock.
//
    lockdep_assert_held(&trace_types_lock);
    trace_array_init_printk(tr);
    inst = kmalloc_obj(*inst);
    if (!inst) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD_RCU(&inst.list);
    inst.tr = tr;
    list_add_tail_rcu(&inst.list, &osnoise_instances);
    return 0;
    }
//
// osnoise_unregister_instance - unregister a registered trace instance
//
// Remove the trace_array *tr from the list of instances running
// osnoise/timerlat tracers.
//
#[no_mangle]
unsafe extern "C" fn osnoise_unregister_instance(tr: *mut trace_array) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut found: c_int = 0;
//
// register/unregister serialization is provided by trace's
// trace_types_lock.
//
    list_for_each_entry_rcu(inst, &osnoise_instances, list,
    lockdep_is_held(&trace_types_lock)) {
    if (inst.tr == tr) {
    list_del_rcu(&inst.list);
    found = 1;
    break;
    }
    }
    if (!found) {
    return;
    }
// Do a full sync to ensure that tr remains valid, not just inst
    synchronize_rcu();
    kvfree(inst);
    }
//
// NMI runtime info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osn_nmi {
    pub count: u64,
    pub delta_start: u64,
}

//
// IRQ runtime info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osn_irq {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

pub const IRQ_CONTEXT: c_int = 0;
pub const THREAD_CONTEXT: c_int = 1;
pub const THREAD_URET: c_int = 2;
//
// sofirq runtime info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osn_softirq {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

//
// thread runtime info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osn_thread {
    pub count: u64,
    pub arrival_time: u64,
    pub delta_start: u64,
}

//
// Runtime information: this structure saves the runtime information used by
// one sampling thread.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_variables {
    pub kthread: *mut task_struct,
    pub sampling: bool,
    pub pid: pid_t,
    pub nmi: osn_nmi,
    pub irq: osn_irq,
    pub softirq: osn_softirq,
    pub thread: osn_thread,
    pub int_counter: local_t,
}

//
// Per-cpu runtime information.
//
pub static mut struct osnoise_variables: usize = 0;
//
// this_cpu_osn_var - Return the per-cpu osnoise_variables on its relative CPU
//
#[no_mangle]
pub unsafe extern "C" fn this_cpu_osn_var() -> *mut c_void {
    return this_cpu_ptr(&per_cpu_osnoise_var);
    }
//
// Protect the interface.
//
pub static mut interface_lock: usize = 0;

//
// Runtime information for the timer mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timerlat_variables {
    pub kthread: *mut task_struct,
    pub timer: hrtimer,
    pub rel_period: u64,
    pub abs_period: u64,
    pub tracing_thread: bool,
    pub count: u64,
    pub uthread_migrate: bool,
}

pub static mut struct timerlat_variables: usize = 0;
//
// timerlat wake-up offset for next thread with TIMERLAT_ALIGN set.
//
    static atomic64_t align_next;
//
// this_cpu_tmr_var - Return the per-cpu timerlat_variables on its relative CPU
//
#[no_mangle]
pub unsafe extern "C" fn this_cpu_tmr_var() -> *mut c_void {
    return this_cpu_ptr(&per_cpu_timerlat_var);
    }
//
// tlat_var_reset - Reset the values of the given timerlat_variables
//
#[no_mangle]
pub unsafe extern "C" fn tlat_var_reset() {
pub static mut tlat_var: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// Synchronize with the timerlat interfaces
    mutex_lock(&interface_lock);
//
// So far, all the values are initialized as 0, so
// zeroing the structure is perfect.
//
    for_each_online_cpu(cpu) {
    tlat_var = per_cpu_ptr(&per_cpu_timerlat_var, cpu);
    if (tlat_var.kthread) {
    hrtimer_cancel(&tlat_var.timer);
    }
    memset(tlat_var, 0, sizeof!(*tlat_var));
    }
//
// Reset also align_next, to be filled by a new offset by the first timerlat
// thread that wakes up, if TIMERLAT_ALIGN is set.
//
    atomic64_set(&align_next, 0);
    mutex_unlock(&interface_lock);
    }

//
// osn_var_reset - Reset the values of the given osnoise_variables
//
#[no_mangle]
pub unsafe extern "C" fn osn_var_reset() {
pub static mut osn_var: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
//
// So far, all the values are initialized as 0, so
// zeroing the structure is perfect.
//
    for_each_online_cpu(cpu) {
    osn_var = per_cpu_ptr(&per_cpu_osnoise_var, cpu);
    memset(osn_var, 0, sizeof!(*osn_var));
    }
    }
//
// osn_var_reset_all - Reset the value of all per-cpu osnoise_variables
//
#[no_mangle]
pub unsafe extern "C" fn osn_var_reset_all() {
    osn_var_reset();
    tlat_var_reset();
    }
//
// Tells NMIs to call back to the osnoise tracer to record timestamps.
//
    let mut trace_osnoise_callback_enabled = 0;
//
// Tracer data.
//
    static struct osnoise_data {
    let mut sample_period = 0;		/* total sampling period */
    let mut sample_runtime = 0;		/* active sampling portion of period */
    let mut stop_tracing = 0;		/* stop trace in the internal operation (loop/irq) */
    let mut stop_tracing_total = 0;	/* stop trace in the final operation (report/thread) */

    let mut timerlat_period = 0;	/* timerlat period */
    let mut timerlat_align_us = 0;	/* timerlat alignment */
    let mut print_stack = 0;		/* print IRQ stack if total > */
    let mut timerlat_tracer = 0;	/* timerlat tracer */

    let mut tainted = 0;		/* info users and developers about a problem */
    } osnoise_data = {
    .sample_period			= DEFAULT_SAMPLE_PERIOD,
    .sample_runtime			= DEFAULT_SAMPLE_RUNTIME,
    .stop_tracing			= 0,
    .stop_tracing_total		= 0,

    .print_stack			= 0,
    .timerlat_period		= DEFAULT_TIMERLAT_PERIOD,
    .timerlat_align_us		= 0,
    .timerlat_tracer		= 0,

    };

#[no_mangle]
pub unsafe extern "C" fn timerlat_enabled() -> bool {
    return osnoise_data.timerlat_tracer;
    }
#[no_mangle]
pub unsafe extern "C" fn timerlat_softirq_exit(osn_var: *mut osnoise_variables) -> c_int {
    let mut tlat_var = this_cpu_tmr_var();
//
// If the timerlat is enabled, but the irq handler did
// not run yet enabling timerlat_tracer, do not trace.
//
    if (!tlat_var.tracing_thread) {
    osn_var.softirq.arrival_time = 0;
    osn_var.softirq.delta_start = 0;
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn timerlat_thread_exit(osn_var: *mut osnoise_variables) -> c_int {
    let mut tlat_var = this_cpu_tmr_var();
//
// If the timerlat is enabled, but the irq handler did
// not run yet enabling timerlat_tracer, do not trace.
//
    if (!tlat_var.tracing_thread) {
    osn_var.thread.delta_start = 0;
    osn_var.thread.arrival_time = 0;
    return 0;
    }
    return 1;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: timerlat_enabled
pub unsafe extern "C" fn timerlat_enabled_dup() -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: timerlat_softirq_exit
pub unsafe extern "C" fn timerlat_softirq_exit_dup(osn_var: *mut osnoise_variables) -> c_int {
    return 1;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: timerlat_thread_exit
pub unsafe extern "C" fn timerlat_thread_exit_dup(osn_var: *mut osnoise_variables) -> c_int {
    return 1;
    }

//
// Print the osnoise header info.
//
#[no_mangle]
unsafe extern "C" fn print_osnoise_headers(s: *mut seq_file) {
    if (osnoise_data.tainted) {
    seq_puts(s, "# osnoise is tainted!\n");
    }
    seq_puts(s, "#                                _-------=> irqs-off\n");
    seq_puts(s, "#                               / _------=> need-resched\n");
    seq_puts(s, "#                              | / _-----=> need-resched-lazy\n");
    seq_puts(s, "#                              || / _----=> hardirq/softirq\n");
    seq_puts(s, "#                              ||| / _---=> preempt-depth\n");
    seq_puts(s, "#                              |||| / _--=> preempt-lazy-depth\n");
    seq_puts(s, "#                              ||||| / _-=> migrate-disable\n");
    seq_puts(s, "#                              |||||| /          ");
    seq_puts(s, "                                     MAX\n");
    seq_puts(s, "#                              ||||| /                         ");
    seq_puts(s, "                    SINGLE      Interference counters:\n");
    seq_puts(s, "#                              |||||||               RUNTIME   ");
    seq_puts(s, "   NOISE  %% OF CPU  NOISE    +-----------------------------+\n");
    seq_puts(s, "#           TASK-PID      CPU# |||||||   TIMESTAMP    IN US    ");
    seq_puts(s, "   IN US  AVAILABLE  IN US     HW    NMI    IRQ   SIRQ THREAD\n");
    seq_puts(s, "#              | |         |   |||||||      |           |      ");
    seq_puts(s, "       |    |            |      |      |      |      |      |\n");
    }

#[no_mangle]
unsafe extern "C" fn print_osnoise_headers(s: *mut seq_file) {
    if (osnoise_data.tainted) {
    seq_puts(s, "# osnoise is tainted!\n");
    }
    seq_puts(s, "#                                _-----=> irqs-off\n");
    seq_puts(s, "#                               / _----=> need-resched\n");
    seq_puts(s, "#                              | / _---=> hardirq/softirq\n");
    seq_puts(s, "#                              || / _--=> preempt-depth\n");
    seq_puts(s, "#                              ||| / _-=> migrate-disable     ");
    seq_puts(s, "                    MAX\n");
    seq_puts(s, "#                              |||| /     delay               ");
    seq_puts(s, "                    SINGLE      Interference counters:\n");
    seq_puts(s, "#                              |||||               RUNTIME   ");
    seq_puts(s, "   NOISE  %% OF CPU  NOISE    +-----------------------------+\n");
    seq_puts(s, "#           TASK-PID      CPU# |||||   TIMESTAMP    IN US    ");
    seq_puts(s, "   IN US  AVAILABLE  IN US     HW    NMI    IRQ   SIRQ THREAD\n");
    seq_puts(s, "#              | |         |   |||||      |           |      ");
    seq_puts(s, "       |    |            |      |      |      |      |      |\n");
    }

//
// osnoise_taint - report an osnoise error.
//

    osnoise_print(msg);							
    osnoise_data.tainted = true;						
    })
//
// Record an osnoise_sample into the tracer buffer.
//
#[no_mangle]
pub unsafe extern "C" fn __record_osnoise_sample(sample: *mut osnoise_sample, buffer: *mut trace_buffer) {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_OSNOISE, sizeof!(*entry),
    tracing_gen_ctx());
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.runtime		= sample.runtime;
    entry.noise		= sample.noise;
    entry.max_sample	= sample.max_sample;
    entry.hw_count		= sample.hw_count;
    entry.nmi_count	= sample.nmi_count;
    entry.irq_count	= sample.irq_count;
    entry.softirq_count	= sample.softirq_count;
    entry.thread_count	= sample.thread_count;
    trace_buffer_unlock_commit_nostack(buffer, event);
    }
//
// Record an osnoise_sample on all osnoise instances and fire trace event.
//
#[no_mangle]
unsafe extern "C" fn record_osnoise_sample(sample: *mut osnoise_sample) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
    trace_osnoise_sample(sample);
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    buffer = inst.tr.array_buffer.buffer;
    __record_osnoise_sample(sample, buffer);
    }
    rcu_read_unlock();
    }

//
// Print the timerlat header info.
//

#[no_mangle]
unsafe extern "C" fn print_timerlat_headers(s: *mut seq_file) {
    seq_puts(s, "#                                _-------=> irqs-off\n");
    seq_puts(s, "#                               / _------=> need-resched\n");
    seq_puts(s, "#                              | / _-----=> need-resched-lazy\n");
    seq_puts(s, "#                              || / _----=> hardirq/softirq\n");
    seq_puts(s, "#                              ||| / _---=> preempt-depth\n");
    seq_puts(s, "#                              |||| / _--=> preempt-lazy-depth\n");
    seq_puts(s, "#                              ||||| / _-=> migrate-disable\n");
    seq_puts(s, "#                              |||||| /\n");
    seq_puts(s, "#                              |||||||             ACTIVATION\n");
    seq_puts(s, "#           TASK-PID      CPU# |||||||   TIMESTAMP    ID     ");
    seq_puts(s, "       CONTEXT                LATENCY\n");
    seq_puts(s, "#              | |         |   |||||||      |         |      ");
    seq_puts(s, "            |                       |\n");
    }

#[no_mangle]
unsafe extern "C" fn print_timerlat_headers(s: *mut seq_file) {
    seq_puts(s, "#                                _-----=> irqs-off\n");
    seq_puts(s, "#                               / _----=> need-resched\n");
    seq_puts(s, "#                              | / _---=> hardirq/softirq\n");
    seq_puts(s, "#                              || / _--=> preempt-depth\n");
    seq_puts(s, "#                              ||| / _-=> migrate-disable\n");
    seq_puts(s, "#                              |||| /     delay\n");
    seq_puts(s, "#                              |||||            ACTIVATION\n");
    seq_puts(s, "#           TASK-PID      CPU# |||||   TIMESTAMP   ID      ");
    seq_puts(s, "      CONTEXT                 LATENCY\n");
    seq_puts(s, "#              | |         |   |||||      |         |      ");
    seq_puts(s, "            |                       |\n");
    }

#[no_mangle]
pub unsafe extern "C" fn __record_timerlat_sample(sample: *mut timerlat_sample, buffer: *mut trace_buffer) {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_TIMERLAT, sizeof!(*entry),
    tracing_gen_ctx());
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.seqnum			= sample.seqnum;
    entry.context			= sample.context;
    entry.timer_latency		= sample.timer_latency;
    trace_buffer_unlock_commit_nostack(buffer, event);
    }
//
// Record an timerlat_sample into the tracer buffer.
//
#[no_mangle]
unsafe extern "C" fn record_timerlat_sample(sample: *mut timerlat_sample) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
    trace_timerlat_sample(sample);
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    buffer = inst.tr.array_buffer.buffer;
    __record_timerlat_sample(sample, buffer);
    }
    rcu_read_unlock();
    }

pub const MAX_CALLS: c_int = 256;
//
// Stack trace will take place only at IRQ level, so, no need
// to control nesting here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_stack {
    pub stack_size: c_int,
    pub nr_entries: c_int,
    pub calls: [c_ulong; MAX_CALLS],
}

pub static mut struct trace_stack: usize = 0;
//
// timerlat_save_stack - save a stack trace without printing
//
// Save the current stack trace without printing. The
// stack will be printed later, after the end of the measurement.
//
#[no_mangle]
unsafe extern "C" fn timerlat_save_stack(skip: c_int) {
    let mut size = 0;
    let mut nr_entries = 0;
pub static mut fstack: *mut c_void = core::ptr::null_mut();
    fstack = this_cpu_ptr(&trace_stack);
    size = ARRAY_SIZE!(fstack.calls);
    nr_entries = stack_trace_save(fstack.calls, size, skip);
    fstack.stack_size = nr_entries * sizeof!(unsigned long);
    fstack.nr_entries = nr_entries;
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn __timerlat_dump_stack(buffer: *mut trace_buffer, fstack: *mut trace_stack, size: c_uint) {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_STACK, sizeof!(*entry) + size,
    tracing_gen_ctx());
    if (!event) {
    return;
    }
    entry = ring_buffer_event_data(event);
    entry.size = fstack.nr_entries;
    memcpy(&entry.caller, fstack.calls, size);
    trace_buffer_unlock_commit_nostack(buffer, event);
    }
//
// timerlat_dump_stack - dump a stack trace previously saved
//
#[no_mangle]
unsafe extern "C" fn timerlat_dump_stack(latency: u64) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut fstack: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
//
// trace only if latency > print_stack config, if enabled.
//
    if (!osnoise_data.print_stack || osnoise_data.print_stack > latency) {
    return;
    }
    preempt_disable_notrace();
    fstack = this_cpu_ptr(&trace_stack);
    size = fstack.stack_size;
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    buffer = inst.tr.array_buffer.buffer;
    __timerlat_dump_stack(buffer, fstack, size);
    }
    rcu_read_unlock();
    preempt_enable_notrace();
    }

//
// Macros to encapsulate the time capturing infrastructure.
//

//
// cond_move_irq_delta_start - Forward the delta_start of a running IRQ
//
// If an IRQ is preempted by an NMI, its delta_start is pushed forward
// to discount the NMI interference.
//
// See get_int_safe_duration().
//
#[no_mangle]
pub unsafe extern "C" fn cond_move_irq_delta_start(osn_var: *mut osnoise_variables, duration: u64) {
    if (osn_var.irq.delta_start) {
    osn_var.irq.delta_start += duration;
    }
    }

//
// cond_move_softirq_delta_start - Forward the delta_start of a running softirq.
//
// If a softirq is preempted by an IRQ or NMI, its delta_start is pushed
// forward to discount the interference.
//
// See get_int_safe_duration().
//
#[no_mangle]
pub unsafe extern "C" fn cond_move_softirq_delta_start(osn_var: *mut osnoise_variables, duration: u64) {
    if (osn_var.softirq.delta_start) {
    osn_var.softirq.delta_start += duration;
    }
    }

//
// cond_move_thread_delta_start - Forward the delta_start of a running thread
//
// If a noisy thread is preempted by an softirq, IRQ or NMI, its delta_start
// is pushed forward to discount the interference.
//
// See get_int_safe_duration().
//
#[no_mangle]
pub unsafe extern "C" fn cond_move_thread_delta_start(osn_var: *mut osnoise_variables, duration: u64) {
    if (osn_var.thread.delta_start) {
    osn_var.thread.delta_start += duration;
    }
    }
//
// get_int_safe_duration - Get the duration of a window
//
// The irq, softirq and thread variables need to have its duration without
// the interference from higher priority interrupts. Instead of keeping a
// variable to discount the interrupt interference from these variables, the
// starting time of these variables are pushed forward with the interrupt's
// duration. In this way, a single variable is used to:
//
// - Know if a given window is being measured.
// - Account its duration.
// - Discount the interference.
//
// To avoid getting inconsistent values, e.g.,:
//
// now = time_get()
// --->	interrupt!
// delta_start -= int duration;
// <---
// duration = now - delta_start;
//
// result: negative duration if the variable duration before the
// interrupt was smaller than the interrupt execution.
//
// A counter of interrupts is used. If the counter increased, try
// to capture an interference safe duration.
//
#[no_mangle]
pub unsafe extern "C" fn get_int_safe_duration(osn_var: *mut osnoise_variables, delta_start: *mut u64) -> s64 {
    u64 int_counter, now;
    let mut duration = 0;
    do {
    int_counter = local_read(&osn_var.int_counter);
// synchronize with interrupts
    barrier();
    now = time_get();
    duration = (now - *delta_start);
// synchronize with interrupts
    barrier();
    } while (int_counter != local_read(&osn_var.int_counter));
//
// This is an evidence of race conditions that cause
// a value to be "discounted" too much.
//
    if (duration < 0) {
    osnoise_taint("Negative duration!\n");
    }
// delta_start = 0;
    return duration;
    }
//
// set_int_safe_time - Save the current time on *time, aware of interference
//
// Get the time, taking into consideration a possible interference from
// higher priority interrupts.
//
// See get_int_safe_duration() for an explanation.
//
#[no_mangle]
pub unsafe extern "C" fn set_int_safe_time(osn_var: *mut osnoise_variables, time: *mut u64) -> u64 {
    let mut int_counter = 0;
    do {
    int_counter = local_read(&osn_var.int_counter);
// synchronize with interrupts
    barrier();
// time = time_get();
// synchronize with interrupts
    barrier();
    } while (int_counter != local_read(&osn_var.int_counter));
    return int_counter;
    }

//
// copy_int_safe_time - Copy *src into *desc aware of interference
//
#[no_mangle]
pub unsafe extern "C" fn copy_int_safe_time(osn_var: *mut osnoise_variables, dst: *mut u64, src: *mut u64) -> u64 {
    let mut int_counter = 0;
    do {
    int_counter = local_read(&osn_var.int_counter);
// synchronize with interrupts
    barrier();
// dst = *src;
// synchronize with interrupts
    barrier();
    } while (int_counter != local_read(&osn_var.int_counter));
    return int_counter;
    }

//
// trace_osnoise_callback - NMI entry/exit callback
//
// This function is called at the entry and exit NMI code. The bool enter
// distinguishes between either case. This function is used to note a NMI
// occurrence, compute the noise caused by the NMI, and to remove the noise
// it is potentially causing on other interference variables.
//
#[no_mangle]
pub unsafe extern "C" fn trace_osnoise_callback(enter: bool) {
    let mut osn_var = this_cpu_osn_var();
    let mut duration = 0;
    if (!osn_var.sampling) {
    return;
    }
//
// Currently trace_clock_local() calls sched_clock() and the
// generic version is not NMI safe.
//
    if (!IS_ENABLED!(CONFIG_GENERIC_SCHED_CLOCK)) {
    if (enter) {
    osn_var.nmi.delta_start = time_get();
    local_inc(&osn_var.int_counter);
    } else {
    duration = time_get() - osn_var.nmi.delta_start;
    trace_nmi_noise(osn_var.nmi.delta_start, duration);
    cond_move_irq_delta_start(osn_var, duration);
    cond_move_softirq_delta_start(osn_var, duration);
    cond_move_thread_delta_start(osn_var, duration);
    }
    }
    if (enter) {
    osn_var.nmi.count += 1;
    }
    }
//
// osnoise_trace_irq_entry - Note the starting of an IRQ
//
// Save the starting time of an IRQ. As IRQs are non-preemptive to other IRQs,
// it is safe to use a single variable (ons_var->irq) to save the statistics.
// The arrival_time is used to report... the arrival time. The delta_start
// is used to compute the duration at the IRQ exit handler. See
// cond_move_irq_delta_start().
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_trace_irq_entry(id: c_int) {
    let mut osn_var = this_cpu_osn_var();
    if (!osn_var.sampling) {
    return;
    }
//
// This value will be used in the report, but not to compute
// the execution time, so it is safe to get it unsafe.
//
    osn_var.irq.arrival_time = time_get();
    set_int_safe_time(osn_var, &osn_var.irq.delta_start);
    osn_var.irq.count += 1;
    local_inc(&osn_var.int_counter);
    }
//
// osnoise_irq_exit - Note the end of an IRQ, sava data and trace
//
// Computes the duration of the IRQ noise, and trace it. Also discounts the
// interference from other sources of noise could be currently being accounted.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_trace_irq_exit(id: c_int, desc: *const c_char) {
    let mut osn_var = this_cpu_osn_var();
    let mut duration = 0;
    if (!osn_var.sampling) {
    return;
    }
    duration = get_int_safe_duration(osn_var, &osn_var.irq.delta_start);
    trace_irq_noise(id, desc, osn_var.irq.arrival_time, duration);
    osn_var.irq.arrival_time = 0;
    cond_move_softirq_delta_start(osn_var, duration);
    cond_move_thread_delta_start(osn_var, duration);
    }
//
// trace_irqentry_callback - Callback to the irq:irq_entry traceevent
//
// Used to note the starting of an IRQ occurece.
//
#[no_mangle]
pub unsafe extern "C" fn trace_irqentry_callback(data: *mut c_void, irq: c_int, action: *mut irqaction) {
    osnoise_trace_irq_entry(irq);
    }
//
// trace_irqexit_callback - Callback to the irq:irq_exit traceevent
//
// Used to note the end of an IRQ occurece.
//
#[no_mangle]
pub unsafe extern "C" fn trace_irqexit_callback(data: *mut c_void, irq: c_int, action: *mut irqaction, ret: c_int) {
    osnoise_trace_irq_exit(irq, action.name);
    }
//
// arch specific register function.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_arch_register() -> int __weak {
    return 0;
    }
//
// arch specific unregister function.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_arch_unregister() -> void __weak {
    return;
    }
//
// hook_irq_events - Hook IRQ handling events
//
// This function hooks the IRQ related callbacks to the respective trace
// events.
//
#[no_mangle]
unsafe extern "C" fn hook_irq_events() -> c_int {
    let mut ret = 0;
    ret = register_trace_irq_handler_entry(trace_irqentry_callback, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = register_trace_irq_handler_exit(trace_irqexit_callback, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = osnoise_arch_register();
    if (ret) {
// goto;
    }
    return 0;
// label;
    unregister_trace_irq_handler_exit(trace_irqexit_callback, core::ptr::null_mut());
// label;
    unregister_trace_irq_handler_entry(trace_irqentry_callback, core::ptr::null_mut());
// label;
    return -EINVAL;
    }
//
// unhook_irq_events - Unhook IRQ handling events
//
// This function unhooks the IRQ related callbacks to the respective trace
// events.
//
#[no_mangle]
unsafe extern "C" fn unhook_irq_events() {
    osnoise_arch_unregister();
    unregister_trace_irq_handler_exit(trace_irqexit_callback, core::ptr::null_mut());
    unregister_trace_irq_handler_entry(trace_irqentry_callback, core::ptr::null_mut());
    }

//
// trace_softirq_entry_callback - Note the starting of a softirq
//
// Save the starting time of a softirq. As softirqs are non-preemptive to
// other softirqs, it is safe to use a single variable (ons_var->softirq)
// to save the statistics. The arrival_time is used to report... the
// arrival time. The delta_start is used to compute the duration at the
// softirq exit handler. See cond_move_softirq_delta_start().
//
#[no_mangle]
unsafe extern "C" fn trace_softirq_entry_callback(data: *mut c_void, vec_nr: c_uint) {
    let mut osn_var = this_cpu_osn_var();
    if (!osn_var.sampling) {
    return;
    }
//
// This value will be used in the report, but not to compute
// the execution time, so it is safe to get it unsafe.
//
    osn_var.softirq.arrival_time = time_get();
    set_int_safe_time(osn_var, &osn_var.softirq.delta_start);
    osn_var.softirq.count += 1;
    local_inc(&osn_var.int_counter);
    }
//
// trace_softirq_exit_callback - Note the end of an softirq
//
// Computes the duration of the softirq noise, and trace it. Also discounts the
// interference from other sources of noise could be currently being accounted.
//
#[no_mangle]
unsafe extern "C" fn trace_softirq_exit_callback(data: *mut c_void, vec_nr: c_uint) {
    let mut osn_var = this_cpu_osn_var();
    let mut duration = 0;
    if (!osn_var.sampling) {
    return;
    }
    if (unlikely(timerlat_enabled())) {
    if (!timerlat_softirq_exit(osn_var))
    return;
    }
    duration = get_int_safe_duration(osn_var, &osn_var.softirq.delta_start);
    trace_softirq_noise(vec_nr, osn_var.softirq.arrival_time, duration);
    cond_move_thread_delta_start(osn_var, duration);
    osn_var.softirq.arrival_time = 0;
    }
//
// hook_softirq_events - Hook softirq handling events
//
// This function hooks the softirq related callbacks to the respective trace
// events.
//
#[no_mangle]
unsafe extern "C" fn hook_softirq_events() -> c_int {
    let mut ret = 0;
    ret = register_trace_softirq_entry(trace_softirq_entry_callback, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = register_trace_softirq_exit(trace_softirq_exit_callback, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    return 0;
// label;
    unregister_trace_softirq_entry(trace_softirq_entry_callback, core::ptr::null_mut());
// label;
    return -EINVAL;
    }
//
// unhook_softirq_events - Unhook softirq handling events
//
// This function hooks the softirq related callbacks to the respective trace
// events.
//
#[no_mangle]
unsafe extern "C" fn unhook_softirq_events() {
    unregister_trace_softirq_entry(trace_softirq_entry_callback, core::ptr::null_mut());
    unregister_trace_softirq_exit(trace_softirq_exit_callback, core::ptr::null_mut());
    }

//
// softirq are threads on the PREEMPT_RT mode.
//
#[no_mangle]
unsafe extern "C" fn hook_softirq_events() -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unhook_softirq_events() {
    }

//
// thread_entry - Record the starting of a thread noise window
//
// It saves the context switch time for a noisy thread, and increments
// the interference counters.
//
#[no_mangle]
pub unsafe extern "C" fn thread_entry(osn_var: *mut osnoise_variables, t: *mut task_struct) {
    if (!osn_var.sampling) {
    return;
    }
//
// The arrival time will be used in the report, but not to compute
// the execution time, so it is safe to get it unsafe.
//
    osn_var.thread.arrival_time = time_get();
    set_int_safe_time(osn_var, &osn_var.thread.delta_start);
    osn_var.thread.count += 1;
    local_inc(&osn_var.int_counter);
    }
//
// thread_exit - Report the end of a thread noise window
//
// It computes the total noise from a thread, tracing if needed.
//
#[no_mangle]
pub unsafe extern "C" fn thread_exit(osn_var: *mut osnoise_variables, t: *mut task_struct) {
    let mut duration = 0;
    if (!osn_var.sampling) {
    return;
    }
    if (unlikely(timerlat_enabled())) {
    if (!timerlat_thread_exit(osn_var))
    return;
    }
    duration = get_int_safe_duration(osn_var, &osn_var.thread.delta_start);
    trace_thread_noise(t, osn_var.thread.arrival_time, duration);
    osn_var.thread.arrival_time = 0;
    }

//
// osnoise_stop_exception - Stop tracing and the tracer.
//
#[no_mangle]
unsafe extern "C" fn osnoise_stop_exception(msg: *mut c_char, cpu: c_int) -> __always_inline void {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    tr = inst.tr;
    trace_array_printk(tr, _THIS_IP_,
    "stop tracing hit on cpu %d due to exception: %s\n",
    smp_processor_id(),
    msg);
    if (test_bit(OSN_PANIC_ON_STOP, &osnoise_options)) {
    panic("tracer hit on cpu %d due to exception: %s\n",
    smp_processor_id(),
    msg);
    }
    tracer_tracing_off(tr);
    }
    rcu_read_unlock();
    }
//
// trace_sched_migrate_callback - sched:sched_migrate_task trace event handler
//
// his function is hooked to the sched:sched_migrate_task trace event, and monitors
// timerlat user-space thread migration.
//
#[no_mangle]
unsafe extern "C" fn trace_sched_migrate_callback(data: *mut c_void, p: *mut task_struct, dest_cpu: c_int) {
pub static mut osn_var: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_long = 0;
    osn_var = per_cpu_ptr(&per_cpu_osnoise_var, cpu);
    if (osn_var.pid == p.pid && dest_cpu != cpu) {
    per_cpu_ptr(&per_cpu_timerlat_var, cpu).uthread_migrate = 1;
    osnoise_taint("timerlat user-thread migrated\n");
    osnoise_stop_exception("timerlat user-thread migrated", cpu);
    }
    }
    static bool monitor_enabled;
#[no_mangle]
unsafe extern "C" fn register_migration_monitor() -> c_int {
pub static mut ret: c_int = 0;
//
// Timerlat thread migration check is only required when running timerlat in user-space.
// Thus, enable callback only if timerlat is set with no workload.
//
    if (timerlat_enabled() && !test_bit(OSN_WORKLOAD, &osnoise_options)) {
    if (WARN_ON_ONCE!(monitor_enabled)) {
    return 0;
    }
    ret = register_trace_sched_migrate_task(trace_sched_migrate_callback, core::ptr::null_mut());
    if (!ret) {
    monitor_enabled = true;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unregister_migration_monitor() {
    if (!monitor_enabled) {
    return;
    }
    unregister_trace_sched_migrate_task(trace_sched_migrate_callback, core::ptr::null_mut());
    monitor_enabled = false;
    }

#[no_mangle]
unsafe extern "C" fn register_migration_monitor() -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_migration_monitor() {}

//
// trace_sched_switch - sched:sched_switch trace event handler
//
// This function is hooked to the sched:sched_switch trace event, and it is
// used to record the beginning and to report the end of a thread noise window.
//
#[no_mangle]
pub unsafe extern "C" fn trace_sched_switch_callback(data: *mut c_void, preempt: bool, p: *mut task_struct, n: *mut task_struct, prev_state: c_uint) {
    let mut osn_var = this_cpu_osn_var();
pub static mut workload: c_int = 0;
    if ((p.pid != osn_var.pid) || !workload) {
    thread_exit(osn_var, p);
    }
    if ((n.pid != osn_var.pid) || !workload) {
    thread_entry(osn_var, n);
    }
    }
//
// hook_thread_events - Hook the instrumentation for thread noise
//
// Hook the osnoise tracer callbacks to handle the noise from other
// threads on the necessary kernel events.
//
#[no_mangle]
unsafe extern "C" fn hook_thread_events() -> c_int {
    let mut ret = 0;
    ret = register_trace_sched_switch(trace_sched_switch_callback, core::ptr::null_mut());
    if (ret) {
    return -EINVAL;
    }
    ret = register_migration_monitor();
    if (ret) {
// goto;
    }
    return 0;
// label;
    unregister_trace_sched_switch(trace_sched_switch_callback, core::ptr::null_mut());
    return -EINVAL;
    }
//
// unhook_thread_events - unhook the instrumentation for thread noise
//
// Unook the osnoise tracer callbacks to handle the noise from other
// threads on the necessary kernel events.
//
#[no_mangle]
unsafe extern "C" fn unhook_thread_events() {
    unregister_trace_sched_switch(trace_sched_switch_callback, core::ptr::null_mut());
    unregister_migration_monitor();
    }
//
// save_osn_sample_stats - Save the osnoise_sample statistics
//
// Save the osnoise_sample statistics before the sampling phase. These
// values will be used later to compute the diff betwneen the statistics
// before and after the osnoise sampling.
//
#[no_mangle]
pub unsafe extern "C" fn save_osn_sample_stats(osn_var: *mut osnoise_variables, s: *mut osnoise_sample) {
    s.nmi_count = osn_var.nmi.count;
    s.irq_count = osn_var.irq.count;
    s.softirq_count = osn_var.softirq.count;
    s.thread_count = osn_var.thread.count;
    }
//
// diff_osn_sample_stats - Compute the osnoise_sample statistics
//
// After a sample period, compute the difference on the osnoise_sample
// statistics. The struct osnoise_sample *s contains the statistics saved via
// save_osn_sample_stats() before the osnoise sampling.
//
#[no_mangle]
pub unsafe extern "C" fn diff_osn_sample_stats(osn_var: *mut osnoise_variables, s: *mut osnoise_sample) {
    s.nmi_count = osn_var.nmi.count - s.nmi_count;
    s.irq_count = osn_var.irq.count - s.irq_count;
    s.softirq_count = osn_var.softirq.count - s.softirq_count;
    s.thread_count = osn_var.thread.count - s.thread_count;
    }
//
// osnoise_stop_tracing - Stop tracing and the tracer.
//
#[no_mangle]
unsafe extern "C" fn osnoise_stop_tracing() -> __always_inline void {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    tr = inst.tr;
    trace_array_printk(tr, _THIS_IP_,
    "stop tracing hit on cpu %d\n", smp_processor_id());
    if (test_bit(OSN_PANIC_ON_STOP, &osnoise_options)) {
    panic("tracer hit stop condition on CPU %d\n", smp_processor_id());
    }
    tracer_tracing_off(tr);
    }
    rcu_read_unlock();
    }
//
// osnoise_has_tracing_on - Check if there is at least one instance on
//
#[no_mangle]
unsafe extern "C" fn osnoise_has_tracing_on() -> __always_inline int {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut trace_is_on: c_int = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    trace_is_on += tracer_tracing_is_on(inst.tr);
    }
    rcu_read_unlock();
    return trace_is_on;
    }
//
// notify_new_max_latency - Notify a new max latency via fsnotify interface.
//
#[no_mangle]
unsafe extern "C" fn notify_new_max_latency(latency: u64) {
pub static mut inst: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(inst, &osnoise_instances, list) {
    tr = inst.tr;
    if (tracer_tracing_is_on(tr) && tr.max_latency < latency) {
    tr.max_latency = latency;
    latency_fsnotify(tr);
    }
    }
    rcu_read_unlock();
    }
//
// run_osnoise - Sample the time and look for osnoise
//
// Used to capture the time, looking for potential osnoise latency repeatedly.
// Different from hwlat_detector, it is called with preemption and interrupts
// enabled. This allows irqs, softirqs and threads to run, interfering on the
// osnoise sampling thread, as they would do with a regular thread.
//
#[no_mangle]
unsafe extern "C" fn run_osnoise() -> c_int {
pub static mut disable_irq: bool = false;
    let mut osn_var = this_cpu_osn_var();
    u64 start, sample, last_sample;
    u64 last_int_count, int_count;
pub static mut noise: i64 = 0;
    s64 total, last_total = 0;
pub static mut s: usize = 0;
    let mut disable_preemption = 0;
    let mut threshold = 0;
    u64 runtime, stop_in;
pub static mut sum_noise: u64 = 0;
pub static mut hw_count: c_int = 0;
pub static mut ret: c_int = 0;
//
// Disabling preemption is only required if IRQs are enabled,
// and the options is set on.
//
    disable_preemption = !disable_irq && test_bit(OSN_PREEMPT_DISABLE, &osnoise_options);
//
// Considers the current thread as the workload.
//
    osn_var.pid = current.pid;
//
// Save the current stats for the diff
//
    save_osn_sample_stats(osn_var, &s);
//
// if threshold is 0, use the default value of 1 us.
//
    threshold = tracing_thresh ? : 1000;
//
// Apply PREEMPT and IRQ disabled options.
//
    if (disable_irq) {
    local_irq_disable();
    }
    if (disable_preemption) {
    preempt_disable();
    }
//
// Make sure NMIs see sampling first
//
    osn_var.sampling = true;
    barrier();
//
// Transform the *_us config to nanoseconds to avoid the
// division on the main loop.
//
    runtime = osnoise_data.sample_runtime * NSEC_PER_USEC;
    stop_in = osnoise_data.stop_tracing * NSEC_PER_USEC;
//
// Start timestamp
//
    start = time_get();
//
// "previous" loop.
//
    last_int_count = set_int_safe_time(osn_var, &last_sample);
    do {
//
// Get sample!
//
    int_count = set_int_safe_time(osn_var, &sample);
    noise = time_sub(sample, last_sample);
//
// This shouldn't happen.
//
    if (noise < 0) {
    osnoise_taint("negative noise!");
// goto;
    }
//
// Sample runtime.
//
    total = time_sub(sample, start);
//
// Check for possible overflows.
//
    if (total < last_total) {
    osnoise_taint("total overflow!");
    break;
    }
    last_total = total;
    if (noise >= threshold) {
pub static mut interference: c_int = 0;
    if (noise > max_noise) {
    max_noise = noise;
    }
    if (!interference) {
    hw_count += 1;
    }
    sum_noise += noise;
    trace_sample_threshold(last_sample, noise, interference);
    if (osnoise_data.stop_tracing) {
    if (noise > stop_in)
    osnoise_stop_tracing();
    }
    }
//
// In some cases, notably when running on a nohz_full CPU with
// a stopped tick PREEMPT_RCU or PREEMPT_LAZY have no way to
// account for QSs. This will eventually cause unwarranted
// noise as RCU forces preemption as the means of ending the
// current grace period.  We avoid this by calling
// rcu_momentary_eqs(), which performs a zero duration EQS
// allowing RCU to end the current grace period. This call
// shouldn't be wrapped inside an RCU critical section.
//
// Normally QSs for other cases are handled through cond_resched().
// For simplicity, however, we call rcu_momentary_eqs() for all
// configurations here.
//
    if (!disable_irq) {
    local_irq_disable();
    }
    rcu_momentary_eqs();
    if (!disable_irq) {
    local_irq_enable();
    }
//
// For the non-preemptive kernel config: let threads runs, if
// they so wish, unless set not do to so.
//
    if (!disable_irq && !disable_preemption) {
    cond_resched();
    }
    last_sample = sample;
    last_int_count = int_count;
    } while (total < runtime && !kthread_should_stop());
//
// Finish the above in the view for interrupts.
//
    barrier();
    osn_var.sampling = false;
//
// Make sure sampling data is no longer updated.
//
    barrier();
//
// Return to the preemptive state.
//
    if (disable_preemption) {
    preempt_enable();
    }
    if (disable_irq) {
    local_irq_enable();
    }
//
// Save noise info.
//
    s.noise = time_to_us(sum_noise);
    s.runtime = time_to_us(total);
    s.max_sample = time_to_us(max_noise);
    s.hw_count = hw_count;
// Save interference stats info
    diff_osn_sample_stats(osn_var, &s);
    record_osnoise_sample(&s);
    notify_new_max_latency(max_noise);
    if (osnoise_data.stop_tracing_total) {
    if (s.noise > osnoise_data.stop_tracing_total)
    osnoise_stop_tracing();
    }
    return 0;
// label;
    return ret;
    }
pub static mut osnoise_cpumask: usize = 0;
pub static mut save_cpumask: usize = 0;
pub static mut kthread_cpumask: usize = 0;
//
// osnoise_sleep - sleep until the next period
//
#[no_mangle]
unsafe extern "C" fn osnoise_sleep(skip_period: bool) {
    let mut interval = 0;
    let mut wake_time;
    mutex_lock(&interface_lock);
    if (skip_period) {
    interval = osnoise_data.sample_period;
    }
    else {
    interval = osnoise_data.sample_period - osnoise_data.sample_runtime;
    }
    mutex_unlock(&interface_lock);
//
// differently from hwlat_detector, the osnoise tracer can run
// without a pause because preemption is on.
//
    if (!interval) {
// Let synchronize_rcu_tasks() make progress
    cond_resched_tasks_rcu_qs();
    return;
    }
    wake_time = ktime_add_us(ktime_get(), interval);
    __set_current_state(TASK_INTERRUPTIBLE);
    while (schedule_hrtimeout(&wake_time, HRTIMER_MODE_ABS)) {
    if (kthread_should_stop()) {
    break;
    }
    }
    }
//
// osnoise_migration_pending - checks if the task needs to migrate
//
// osnoise/timerlat threads are per-cpu. If there is a pending request to
// migrate the thread away from the current CPU, something bad has happened.
// Play the good citizen and leave.
//
// Returns 0 if it is safe to continue, 1 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_migration_pending() -> c_int {
    if (!current.migration_pending) {
    return 0;
    }
//
// If migration is pending, there is a task waiting for the
// tracer to enable migration. The tracer does not allow migration,
// thus: taint and leave to unblock the blocked thread.
//
    osnoise_taint("migration requested to osnoise threads, leaving.");
//
// Unset this thread from the threads managed by the interface.
// The tracers are responsible for cleaning their env before
// exiting.
//
    mutex_lock(&interface_lock);
    this_cpu_osn_var().kthread = core::ptr::null_mut();
    cpumask_clear_cpu(smp_processor_id(), &kthread_cpumask);
    mutex_unlock(&interface_lock);
    return 1;
    }
//
// osnoise_main - The osnoise detection kernel thread
//
// Calls run_osnoise() function to measure the osnoise for the configured runtime,
// every period.
//
#[no_mangle]
unsafe extern "C" fn osnoise_main(data: *mut c_void) -> c_int {
    let mut flags = 0;
//
// This thread was created pinned to the CPU using PF_NO_SETAFFINITY.
// The problem is that cgroup does not allow PF_NO_SETAFFINITY thread.
//
// To work around this limitation, disable migration and remove the
// flag.
//
    migrate_disable();
    raw_spin_lock_irqsave(&current.pi_lock, flags);
    current.flags &= ~(PF_NO_SETAFFINITY);
    raw_spin_unlock_irqrestore(&current.pi_lock, flags);
    while (!kthread_should_stop()) {
    if (osnoise_migration_pending()) {
    break;
    }
// skip a period if tracing is off on all instances
    if (!osnoise_has_tracing_on()) {
    osnoise_sleep(true);
    continue;
    }
    run_osnoise();
    osnoise_sleep(false);
    }
    migrate_enable();
    return 0;
    }

//
// timerlat_irq - hrtimer handler for timerlat.
//
#[no_mangle]
unsafe extern "C" fn timerlat_irq(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut osn_var = this_cpu_osn_var();
pub static mut tlat: *mut c_void = core::ptr::null_mut();
pub static mut s: usize = 0;
    let mut now = 0;
    let mut diff = 0;
//
// I am not sure if the timer was armed for this CPU. So, get
// the timerlat struct from the timer itself, not from this
// CPU.
//
    tlat = container_of!(timer, timerlat_variables, timer);
    now = ktime_to_ns(hrtimer_cb_get_time(&tlat.timer));
//
// Enable the osnoise: events for thread an softirq.
//
    tlat.tracing_thread = true;
    osn_var.thread.arrival_time = time_get();
//
// A hardirq is running: the timer IRQ. It is for sure preempting
// a thread, and potentially preempting a softirq.
//
// At this point, it is not interesting to know the duration of the
// preempted thread (and maybe softirq), but how much time they will
// delay the beginning of the execution of the timer thread.
//
// To get the correct (net) delay added by the softirq, its delta_start
// is set as the IRQ one. In this way, at the return of the IRQ, the delta
// start of the sofitrq will be zeroed, accounting then only the time
// after that.
//
// The thread follows the same principle. However, if a softirq is
// running, the thread needs to receive the softirq delta_start. The
// reason being is that the softirq will be the last to be unfolded,
// resseting the thread delay to zero.
//
// The PREEMPT_RT is a special case, though. As softirqs run as threads
// on RT, moving the thread is enough.
//
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT) && osn_var.softirq.delta_start) {
    copy_int_safe_time(osn_var, &osn_var.thread.delta_start,
    &osn_var.softirq.delta_start);
    copy_int_safe_time(osn_var, &osn_var.softirq.delta_start,
    &osn_var.irq.delta_start);
    } else {
    copy_int_safe_time(osn_var, &osn_var.thread.delta_start,
    &osn_var.irq.delta_start);
    }
//
// Compute the current time with the expected time.
//
    diff = now - tlat.abs_period;
    tlat.count += 1;
    s.seqnum = tlat.count;
    s.timer_latency = diff;
    s.context = IRQ_CONTEXT;
    record_timerlat_sample(&s);
    if (osnoise_data.stop_tracing) {
    if (time_to_us(diff) >= osnoise_data.stop_tracing) {
//
// At this point, if stop_tracing is set and <= print_stack,
// print_stack is set and would be printed in the thread handler.
//
// Thus, print the stack trace as it is helpful to define the
// root cause of an IRQ latency.
//
    if (osnoise_data.stop_tracing <= osnoise_data.print_stack) {
    timerlat_save_stack(0);
    timerlat_dump_stack(time_to_us(diff));
    }
    osnoise_stop_tracing();
    notify_new_max_latency(diff);
    wake_up_process(tlat.kthread);
    return HRTIMER_NORESTART;
    }
    }
    wake_up_process(tlat.kthread);
    if (osnoise_data.print_stack) {
    timerlat_save_stack(0);
    }
    return HRTIMER_NORESTART;
    }
//
// wait_next_period - Wait for the next period for timerlat
//
#[no_mangle]
unsafe extern "C" fn wait_next_period(tlat: *mut timerlat_variables) -> c_int {
    ktime_t next_abs_period, now;
pub static mut rel_period: u64 = 0;
    now = hrtimer_cb_get_time(&tlat.timer);
    next_abs_period = ns_to_ktime(tlat.abs_period + rel_period);
//
// Save the next abs_period.
//
    tlat.abs_period = (u64) ktime_to_ns(next_abs_period);
//
// Align thread in the first cycle on each CPU to the set alignment
// if TIMERLAT_ALIGN is set.
//
// This is done by using an atomic64_t to store the next absolute period.
// The first thread that wakes up will set the atomic64_t to its
// absolute period, and the other threads will increment it by
// the alignment value.
//
    if (test_bit(OSN_TIMERLAT_ALIGN, &osnoise_options) && !tlat.count
    && atomic64_cmpxchg_relaxed(&align_next, 0, tlat.abs_period)) {
//
// A thread has already set align_next, use it and increment it
// to be used by the next thread that wakes up after this one.
//
    tlat.abs_period = atomic64_add_return_relaxed(
    osnoise_data.timerlat_align_us * 1000, &align_next);
    next_abs_period = ns_to_ktime(tlat.abs_period);
    }
//
// If the new abs_period is in the past, skip the activation.
//
    while (ktime_compare(now, next_abs_period) > 0) {
    next_abs_period = ns_to_ktime(tlat.abs_period + rel_period);
    tlat.abs_period = (u64) ktime_to_ns(next_abs_period);
    }
    set_current_state(TASK_INTERRUPTIBLE);
    hrtimer_start(&tlat.timer, next_abs_period, HRTIMER_MODE_ABS_PINNED_HARD);
    schedule();
    return 1;
    }
//
// timerlat_main- Timerlat main
//
#[no_mangle]
unsafe extern "C" fn timerlat_main(data: *mut c_void) -> c_int {
    let mut osn_var = this_cpu_osn_var();
    let mut tlat = this_cpu_tmr_var();
pub static mut s: usize = 0;
pub static mut sp: usize = 0;
    let mut flags = 0;
    u64 now, diff;
//
// Make the thread RT, that is how cyclictest is usually used.
//
    sp.sched_priority = DEFAULT_TIMERLAT_PRIO;
    sched_setscheduler_nocheck(current, SCHED_FIFO, &sp);
//
// This thread was created pinned to the CPU using PF_NO_SETAFFINITY.
// The problem is that cgroup does not allow PF_NO_SETAFFINITY thread.
//
// To work around this limitation, disable migration and remove the
// flag.
//
    migrate_disable();
    raw_spin_lock_irqsave(&current.pi_lock, flags);
    current.flags &= ~(PF_NO_SETAFFINITY);
    raw_spin_unlock_irqrestore(&current.pi_lock, flags);
    tlat.count = 0;
    tlat.tracing_thread = false;
    hrtimer_setup(&tlat.timer, timerlat_irq, CLOCK_MONOTONIC, HRTIMER_MODE_ABS_PINNED_HARD);
    tlat.kthread = current;
    osn_var.pid = current.pid;
//
// Annotate the arrival time.
//
    tlat.abs_period = hrtimer_cb_get_time(&tlat.timer);
    wait_next_period(tlat);
    osn_var.sampling = 1;
    while (!kthread_should_stop()) {
    now = ktime_to_ns(hrtimer_cb_get_time(&tlat.timer));
    diff = now - tlat.abs_period;
    s.seqnum = tlat.count;
    s.timer_latency = diff;
    s.context = THREAD_CONTEXT;
    record_timerlat_sample(&s);
    notify_new_max_latency(diff);
    timerlat_dump_stack(time_to_us(diff));
    tlat.tracing_thread = false;
    if (osnoise_data.stop_tracing_total) {
    if (time_to_us(diff) >= osnoise_data.stop_tracing_total)
    osnoise_stop_tracing();
    }
    if (osnoise_migration_pending()) {
    break;
    }
    wait_next_period(tlat);
    }
    hrtimer_cancel(&tlat.timer);
    migrate_enable();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn timerlat_main(data: *mut c_void) -> c_int {
    return 0;
    }

//
// stop_kthread - stop a workload thread
//
#[no_mangle]
unsafe extern "C" fn stop_kthread(cpu: c_uint) {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    kthread = xchg_relaxed(&(per_cpu(per_cpu_osnoise_var, cpu).kthread), core::ptr::null_mut());
    if (kthread) {
    if (cpumask_test_and_clear_cpu(cpu, &kthread_cpumask) &&
    !WARN_ON!(!test_bit(OSN_WORKLOAD, &osnoise_options))) {
    kthread_stop(kthread);
    } else if (!WARN_ON!(test_bit(OSN_WORKLOAD, &osnoise_options))) {
//
// This is a user thread waiting on the timerlat_fd. We need
// to close all users, and the best way to guarantee this is
// by killing the thread. NOTE: this is a purpose specific file.
//
    kill_pid(kthread.thread_pid, SIGKILL, 1);
    put_task_struct(kthread);
    }
    } else {
// if no workload, just return
    if (!test_bit(OSN_WORKLOAD, &osnoise_options)) {
//
// This is set in the osnoise tracer case.
//
    per_cpu(per_cpu_osnoise_var, cpu).sampling = false;
    barrier();
    }
    }
    }
//
// stop_per_cpu_kthread - Stop per-cpu threads
//
// Stop the osnoise sampling htread. Use this on unload and at system
// shutdown.
//
#[no_mangle]
unsafe extern "C" fn stop_per_cpu_kthreads() {
    let mut cpu = 0;
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    stop_kthread(cpu);
    }
    cpus_read_unlock();
    }
//
// start_kthread - Start a workload thread
//
#[no_mangle]
unsafe extern "C" fn start_kthread(cpu: c_uint) -> c_int {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    let mut main = osnoise_main;
    char comm[24];
// Do not start a new thread if it is already running
    if (per_cpu(per_cpu_osnoise_var, cpu).kthread) {
    return 0;
    }
    if (timerlat_enabled()) {
    snprintf(comm, 24, "timerlat/%d", cpu);
    main = timerlat_main;
    } else {
// if no workload, just return
    if (!test_bit(OSN_WORKLOAD, &osnoise_options)) {
    per_cpu(per_cpu_osnoise_var, cpu).sampling = true;
    barrier();
    return 0;
    }
    snprintf(comm, 24, "osnoise/%d", cpu);
    }
    kthread = kthread_run_on_cpu(main, core::ptr::null_mut(), cpu, comm);
    if (IS_ERR(kthread)) {
    pr_err!(BANNER "could not start sampling thread\n");
    return -ENOMEM;
    }
    per_cpu(per_cpu_osnoise_var, cpu).kthread = kthread;
    cpumask_set_cpu(cpu, &kthread_cpumask);
    return 0;
    }
//
// start_per_cpu_kthread - Kick off per-cpu osnoise sampling kthreads
//
// This starts the kernel thread that will look for osnoise on many
// cpus.
//
#[no_mangle]
unsafe extern "C" fn start_per_cpu_kthreads() -> c_int {
    let mut current_mask = &save_cpumask;
pub static mut retval: c_int = 0;
    let mut cpu = 0;
    if (!test_bit(OSN_WORKLOAD, &osnoise_options)) {
    if (timerlat_enabled()) {
    return 0;
    }
    }
    cpus_read_lock();
//
// Run only on online CPUs in which osnoise is allowed to run.
//
    cpumask_and(current_mask, cpu_online_mask, &osnoise_cpumask);
    for_each_possible_cpu(cpu) {
    if (cpumask_test_and_clear_cpu(cpu, &kthread_cpumask)) {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    kthread = xchg_relaxed(&(per_cpu(per_cpu_osnoise_var, cpu).kthread), core::ptr::null_mut());
    if (!WARN_ON!(!kthread)) {
    kthread_stop(kthread);
    }
    }
    }
    for_each_cpu(cpu, current_mask) {
    retval = start_kthread(cpu);
    if (retval) {
    cpus_read_unlock();
    stop_per_cpu_kthreads();
    return retval;
    }
    }
    cpus_read_unlock();
    return retval;
    }

#[no_mangle]
unsafe extern "C" fn osnoise_hotplug_workfn(dummy: *mut work_struct) {
pub static mut cpu: c_uint = 0;
    guard(mutex)(&trace_types_lock);
    if (!osnoise_has_registered_instances()) {
    return;
    }
    guard(cpus_read_lock)();
    guard(mutex)(&interface_lock);
    if (!cpu_online(cpu)) {
    return;
    }
    if (!cpumask_test_cpu(cpu, &osnoise_cpumask)) {
    return;
    }
    start_kthread(cpu);
    }
pub static mut osnoise_hotplug_work: usize = 0;
//
// osnoise_cpu_init - CPU hotplug online callback function
//
#[no_mangle]
unsafe extern "C" fn osnoise_cpu_init(cpu: c_uint) -> c_int {
    schedule_work_on(cpu, &osnoise_hotplug_work);
    return 0;
    }
//
// osnoise_cpu_die - CPU hotplug offline callback function
//
#[no_mangle]
unsafe extern "C" fn osnoise_cpu_die(cpu: c_uint) -> c_int {
    stop_kthread(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn osnoise_init_hotplug_support() {
    let mut ret = 0;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "trace/osnoise:online",
    osnoise_cpu_init, osnoise_cpu_die);
    if (ret < 0) {
    pr_warn!(BANNER "Error to init cpu hotplug support\n");
    }
    return;
    }

#[no_mangle]
unsafe extern "C" fn osnoise_init_hotplug_support() {
    return;
    }

//
// seq file functions for the osnoise/options file.
//
#[no_mangle]
pub unsafe extern "C" fn s_options_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut option: c_int = 0;
    mutex_lock(&interface_lock);
    if (option >= OSN_MAX) {
    return core::ptr::null_mut();
    }
    return pos;
    }
#[no_mangle]
pub unsafe extern "C" fn s_options_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut option: c_int = 0;
    if (option >= OSN_MAX) {
    return core::ptr::null_mut();
    }
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn s_options_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let mut pos = v;
pub static mut option: c_int = 0;
    if (option == OSN_DEFAULTS) {
    if (osnoise_options == OSN_DEFAULT_OPTIONS) {
    seq_printf(s, "%s", osnoise_options_str[option]);
    }
    else {
    seq_printf(s, "NO_%s", osnoise_options_str[option]);
    }
// goto;
    }
    if (test_bit(option, &osnoise_options)) {
    seq_printf(s, "%s", osnoise_options_str[option]);
    }
    else {
    seq_printf(s, "NO_%s", osnoise_options_str[option]);
    }
// label;
    if (option != OSN_MAX) {
    seq_puts(s, " ");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s_options_stop(s: *mut seq_file, v: *mut c_void) {
    seq_puts(s, "\n");
    mutex_unlock(&interface_lock);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn osnoise_options_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &osnoise_options_seq_ops);
    };
//
// osnoise_options_write - Write function for "options" entry
// @filp: The active open file structure
// @ubuf: The user buffer that contains the value to write
// @cnt: The maximum number of bytes to write to "file"
// @ppos: The current position in @file
//
// Writing the option name sets the option, writing the "NO_"
// prefix in front of the option name disables it.
//
// Writing "DEFAULTS" resets the option values to the default ones.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_options_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut running = 0;
    let mut option = 0;
    let mut enable = 0;
    let mut retval = 0;
    char buf[256], *option_str;
    if (cnt >= 256) {
    return -EINVAL;
    }
    if (copy_from_user(buf, ubuf, cnt)) {
    return -EFAULT;
    }
    buf[cnt] = 0;
    if (strncmp(buf, "NO_", 3)) {
    option_str = strstrip(buf);
    enable = true;
    } else {
    option_str = strstrip(&buf[3]);
    enable = false;
    }
    option = match_string(osnoise_options_str, OSN_MAX, option_str);
    if (option < 0) {
    return -EINVAL;
    }
//
// trace_types_lock is taken to avoid concurrency on start/stop.
//
    mutex_lock(&trace_types_lock);
    running = osnoise_has_registered_instances();
    if (running) {
    stop_per_cpu_kthreads();
    }
//
// avoid CPU hotplug operations that might read options.
//
    cpus_read_lock();
    mutex_lock(&interface_lock);
    retval = cnt;
    if (enable) {
    if (option == OSN_DEFAULTS) {
    osnoise_options = OSN_DEFAULT_OPTIONS;
    }
    else {
    set_bit(option, &osnoise_options);
    }
    } else {
    if (option == OSN_DEFAULTS) {
    retval = -EINVAL;
    }
    else {
    clear_bit(option, &osnoise_options);
    }
    }
    mutex_unlock(&interface_lock);
    cpus_read_unlock();
    if (running) {
    start_per_cpu_kthreads();
    }
    mutex_unlock(&trace_types_lock);
    return retval;
    }
//
// osnoise_cpus_read - Read function for reading the "cpus" file
// @filp: The active open file structure
// @ubuf: The userspace provided buffer to read value into
// @cnt: The maximum number of bytes to read
// @ppos: The current "file" position
//
// Prints the "cpus" output into the user-provided buffer.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_cpus_read(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    char *mask_str __free(kfree) = core::ptr::null_mut();
    let mut len = 0;
    guard(mutex)(&interface_lock);
    len = snprintf(core::ptr::null_mut(), 0, "%*pbl\n", cpumask_pr_args(&osnoise_cpumask)) + 1;
    mask_str = kmalloc(len, GFP_KERNEL);
    if (!mask_str) {
    return -ENOMEM;
    }
    len = snprintf(mask_str, len, "%*pbl\n", cpumask_pr_args(&osnoise_cpumask));
    if (len >= count) {
    return -EINVAL;
    }
    count = simple_read_from_buffer(ubuf, count, ppos, mask_str, len);
    return count;
    }
//
// osnoise_cpus_write - Write function for "cpus" entry
// @filp: The active open file structure
// @ubuf: The user buffer that contains the value to write
// @count: The maximum number of bytes to write to "file"
// @ppos: The current position in @file
//
// This function provides a write implementation for the "cpus"
// interface to the osnoise trace. By default, it lists all  CPUs,
// in this way, allowing osnoise threads to run on any online CPU
// of the system. It serves to restrict the execution of osnoise to the
// set of CPUs writing via this interface. Why not use "tracing_cpumask"?
// Because the user might be interested in tracing what is running on
// other CPUs. For instance, one might run osnoise in one HT CPU
// while observing what is running on the sibling HT CPU.
//
#[no_mangle]
pub unsafe extern "C" fn osnoise_cpus_write(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut osnoise_cpumask_new;
    let mut running = 0;
    let mut err = 0;
    char *buf __free(kfree) = core::ptr::null_mut();
    if (count < 1) {
    return 0;
    }
    buf = memdup_user_nul(ubuf, count);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    if (!zalloc_cpumask_var(&osnoise_cpumask_new, GFP_KERNEL)) {
    return -ENOMEM;
    }
    err = cpulist_parse(buf, osnoise_cpumask_new);
    if (err) {
// goto;
    }
//
// trace_types_lock is taken to avoid concurrency on start/stop.
//
    mutex_lock(&trace_types_lock);
    running = osnoise_has_registered_instances();
    if (running) {
    stop_per_cpu_kthreads();
    }
//
// osnoise_cpumask is read by CPU hotplug operations.
//
    cpus_read_lock();
    mutex_lock(&interface_lock);
    cpumask_copy(&osnoise_cpumask, osnoise_cpumask_new);
    mutex_unlock(&interface_lock);
    cpus_read_unlock();
    if (running) {
    start_per_cpu_kthreads();
    }
    mutex_unlock(&trace_types_lock);
    free_cpumask_var(osnoise_cpumask_new);
    return count;
// label;
    free_cpumask_var(osnoise_cpumask_new);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn timerlat_fd_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut osn_var: *mut c_void = core::ptr::null_mut();
pub static mut tlat: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_long = 0;
    mutex_lock(&interface_lock);
//
// This file is accessible only if timerlat is enabled, and
// NO_OSNOISE_WORKLOAD is set.
//
    if (!timerlat_enabled() || test_bit(OSN_WORKLOAD, &osnoise_options)) {
    mutex_unlock(&interface_lock);
    return -EINVAL;
    }
    migrate_disable();
    osn_var = this_cpu_osn_var();
//
// The osn_var->pid holds the single access to this file.
//
    if (osn_var.pid) {
    mutex_unlock(&interface_lock);
    migrate_enable();
    return -EBUSY;
    }
//
// timerlat tracer is a per-cpu tracer. Check if the user-space too
// is pinned to a single CPU. The tracer laters monitor if the task
// migrates and then disables tracer if it does. However, it is
// worth doing this basic acceptance test to avoid obviusly wrong
// setup.
//
    if (current.nr_cpus_allowed > 1 ||  cpu != smp_processor_id()) {
    mutex_unlock(&interface_lock);
    migrate_enable();
    return -EPERM;
    }
//
// From now on, it is good to go.
//
    file.private_data = inode.i_cdev;
    get_task_struct(current);
    osn_var.kthread = current;
    osn_var.pid = current.pid;
//
// Setup is done.
//
    mutex_unlock(&interface_lock);
    tlat = this_cpu_tmr_var();
    tlat.count = 0;
    hrtimer_setup(&tlat.timer, timerlat_irq, CLOCK_MONOTONIC, HRTIMER_MODE_ABS_PINNED_HARD);
    migrate_enable();
    return 0;
    };
//
// timerlat_fd_read - Read function for "timerlat_fd" file
// @file: The active open file structure
// @ubuf: The userspace provided buffer to read value into
// @cnt: The maximum number of bytes to read
// @ppos: The current "file" position
//
// Prints 1 on timerlat, the number of interferences on osnoise, -1 on error.
//
#[no_mangle]
pub unsafe extern "C" fn timerlat_fd_read(file: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut cpu: c_long = 0;
pub static mut osn_var: *mut c_void = core::ptr::null_mut();
pub static mut tlat: *mut c_void = core::ptr::null_mut();
pub static mut s: usize = 0;
    let mut diff = 0;
    let mut now = 0;
    migrate_disable();
    tlat = this_cpu_tmr_var();
//
// While in user-space, the thread is migratable. There is nothing
// we can do about it.
// So, if the thread is running on another CPU, stop the machinery.
//
    if (cpu == smp_processor_id()) {
    if (tlat.uthread_migrate) {
    migrate_enable();
    return -EINVAL;
    }
    } else {
    per_cpu_ptr(&per_cpu_timerlat_var, cpu).uthread_migrate = 1;
    osnoise_taint("timerlat user thread migrate\n");
    osnoise_stop_tracing();
    migrate_enable();
    return -EINVAL;
    }
    osn_var = this_cpu_osn_var();
//
// The timerlat in user-space runs in a different order:
// the read() starts from the execution of the previous occurrence,
// sleeping for the next occurrence.
//
// So, skip if we are entering on read() before the first wakeup
// from timerlat IRQ:
//
    if (likely(osn_var.sampling)) {
    now = ktime_to_ns(hrtimer_cb_get_time(&tlat.timer));
    diff = now - tlat.abs_period;
//
// it was not a timer firing, but some other signal?
//
    if (diff < 0) {
// goto;
    }
    s.seqnum = tlat.count;
    s.timer_latency = diff;
    s.context = THREAD_URET;
    record_timerlat_sample(&s);
    notify_new_max_latency(diff);
    tlat.tracing_thread = false;
    if (osnoise_data.stop_tracing_total) {
    if (time_to_us(diff) >= osnoise_data.stop_tracing_total) {
    timerlat_dump_stack(time_to_us(diff));
    osnoise_stop_tracing();
    }
    }
    } else {
    tlat.tracing_thread = false;
    tlat.kthread = current;
// Annotate now to drift new period
    tlat.abs_period = hrtimer_cb_get_time(&tlat.timer);
    osn_var.sampling = 1;
    }
// wait for the next period
    wait_next_period(tlat);
// This is the wakeup from this cycle
    now = ktime_to_ns(hrtimer_cb_get_time(&tlat.timer));
    diff = now - tlat.abs_period;
//
// it was not a timer firing, but some other signal?
//
    if (diff < 0) {
// goto;
    }
    s.seqnum = tlat.count;
    s.timer_latency = diff;
    s.context = THREAD_CONTEXT;
    record_timerlat_sample(&s);
    if (osnoise_data.stop_tracing_total) {
    if (time_to_us(diff) >= osnoise_data.stop_tracing_total) {
    timerlat_dump_stack(time_to_us(diff));
    notify_new_max_latency(diff);
    osnoise_stop_tracing();
    }
    }
// label;
    migrate_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timerlat_fd_release(inode: *mut inode, file: *mut file) -> c_int {
pub static mut osn_var: *mut c_void = core::ptr::null_mut();
pub static mut tlat_var: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_long = 0;
    migrate_disable();
    mutex_lock(&interface_lock);
    osn_var = per_cpu_ptr(&per_cpu_osnoise_var, cpu);
    tlat_var = per_cpu_ptr(&per_cpu_timerlat_var, cpu);
    if (tlat_var.kthread) {
    hrtimer_cancel(&tlat_var.timer);
    }
    memset(tlat_var, 0, sizeof!(*tlat_var));
    osn_var.sampling = 0;
    osn_var.pid = 0;
//
// We are leaving, not being stopped... see stop_kthread();
//
    if (osn_var.kthread) {
    put_task_struct(osn_var.kthread);
    osn_var.kthread = core::ptr::null_mut();
    }
    mutex_unlock(&interface_lock);
    migrate_enable();
    return 0;
    }

//
// osnoise/runtime_us: cannot be greater than the period.
//
pub static mut trace_min_max_param: usize = 0;
//
// osnoise/period_us: cannot be smaller than the runtime.
//
pub static mut trace_min_max_param: usize = 0;
//
// osnoise/stop_tracing_us: no limit.
//
pub static mut trace_min_max_param: usize = 0;
//
// osnoise/stop_tracing_total_us: no limit.
//
pub static mut trace_min_max_param: usize = 0;

//
// osnoise/print_stack: print the stacktrace of the IRQ handler if the total
// latency is higher than val.
//
pub static mut trace_min_max_param: usize = 0;
//
// osnoise/timerlat_period: min 100 us, max 1 s
//
pub static mut timerlat_min_period: u64 = 100;
pub static mut timerlat_max_period: u64 = 1000000;
pub static mut trace_min_max_param: usize = 0;
//
// osnoise/timerlat_align_us: align the first wakeup of all timerlat
// threads to a common boundary (in us). 0 means disabled.
//
pub static mut trace_min_max_param: usize = 0;
pub static mut file_operations: usize = 0;

pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;

#[no_mangle]
unsafe extern "C" fn init_timerlat_stack_tracefs(top_dir: *mut dentry) -> c_int {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    tmp = tracefs_create_file("print_stack", TRACE_MODE_WRITE, top_dir,
    &osnoise_print_stack, &trace_min_max_fops);
    if (!tmp) {
    return -ENOMEM;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn init_timerlat_stack_tracefs(top_dir: *mut dentry) -> c_int {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn osnoise_create_cpu_timerlat_fd(top_dir: *mut dentry) -> c_int {
pub static mut timerlat_fd: *mut c_void = core::ptr::null_mut();
pub static mut per_cpu: *mut c_void = core::ptr::null_mut();
pub static mut cpu_dir: *mut c_void = core::ptr::null_mut();
    char cpu_str[30]; /* see trace.c: tracing_init_tracefs_percpu() */
    let mut cpu = 0;
//
// Why not using tracing instance per_cpu/ dir?
//
// Because osnoise/timerlat have a single workload, having
// multiple files like these are waste of memory.
//
    per_cpu = tracefs_create_dir("per_cpu", top_dir);
    if (!per_cpu) {
    return -ENOMEM;
    }
    for_each_possible_cpu(cpu) {
    snprintf(cpu_str, 30, "cpu%ld", cpu);
    cpu_dir = tracefs_create_dir(cpu_str, per_cpu);
    if (!cpu_dir) {
// goto;
    }
    timerlat_fd = trace_create_file("timerlat_fd", TRACE_MODE_READ,
    cpu_dir, core::ptr::null_mut(), &timerlat_fd_fops);
    if (!timerlat_fd) {
// goto;
    }
// Record the CPU
    d_inode(timerlat_fd).i_cdev = (cpu);
    }
    return 0;
// label;
    tracefs_remove(per_cpu);
    return -ENOMEM;
    }
//
// init_timerlat_tracefs - A function to initialize the timerlat interface files
//
#[no_mangle]
unsafe extern "C" fn init_timerlat_tracefs(top_dir: *mut dentry) -> c_int {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    tmp = tracefs_create_file("timerlat_period_us", TRACE_MODE_WRITE, top_dir,
    &timerlat_period, &trace_min_max_fops);
    if (!tmp) {
    return -ENOMEM;
    }
    tmp = tracefs_create_file("timerlat_align_us", TRACE_MODE_WRITE, top_dir,
    &timerlat_align_us, &trace_min_max_fops);
    if (!tmp) {
    return -ENOMEM;
    }
    retval = osnoise_create_cpu_timerlat_fd(top_dir);
    if (retval) {
    return retval;
    }
    return init_timerlat_stack_tracefs(top_dir);
    }

#[no_mangle]
unsafe extern "C" fn init_timerlat_tracefs(top_dir: *mut dentry) -> c_int {
    return 0;
    }

//
// init_tracefs - A function to initialize the tracefs interface files
//
// This function creates entries in tracefs for "osnoise" and "timerlat".
// It creates these directories in the tracing directory, and within that
// directory the use can change and view the configs.
//
#[no_mangle]
unsafe extern "C" fn init_tracefs() -> c_int {
pub static mut top_dir: *mut c_void = core::ptr::null_mut();
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = tracing_init_dentry();
    if (ret) {
    return -ENOMEM;
    }
    top_dir = tracefs_create_dir("osnoise", core::ptr::null_mut());
    if (!top_dir) {
    return 0;
    }
    tmp = tracefs_create_file("period_us", TRACE_MODE_WRITE, top_dir,
    &osnoise_period, &trace_min_max_fops);
    if (!tmp) {
// goto;
    }
    tmp = tracefs_create_file("runtime_us", TRACE_MODE_WRITE, top_dir,
    &osnoise_runtime, &trace_min_max_fops);
    if (!tmp) {
// goto;
    }
    tmp = tracefs_create_file("stop_tracing_us", TRACE_MODE_WRITE, top_dir,
    &osnoise_stop_tracing_in, &trace_min_max_fops);
    if (!tmp) {
// goto;
    }
    tmp = tracefs_create_file("stop_tracing_total_us", TRACE_MODE_WRITE, top_dir,
    &osnoise_stop_tracing_total, &trace_min_max_fops);
    if (!tmp) {
// goto;
    }
    tmp = trace_create_file("cpus", TRACE_MODE_WRITE, top_dir, core::ptr::null_mut(), &cpus_fops);
    if (!tmp) {
// goto;
    }
    tmp = trace_create_file("options", TRACE_MODE_WRITE, top_dir, core::ptr::null_mut(),
    &osnoise_options_fops);
    if (!tmp) {
// goto;
    }
    ret = init_timerlat_tracefs(top_dir);
    if (ret) {
// goto;
    }
    return 0;
// label;
    tracefs_remove(top_dir);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn osnoise_hook_events() -> c_int {
    let mut retval = 0;
//
// Trace is already hooked, we are re-enabling from
// a stop_tracing_*.
//
    if (trace_osnoise_callback_enabled) {
    return 0;
    }
    retval = hook_irq_events();
    if (retval) {
    return -EINVAL;
    }
    retval = hook_softirq_events();
    if (retval) {
// goto;
    }
    retval = hook_thread_events();
//
// All fine!
//
    if (!retval) {
    return 0;
    }
    unhook_softirq_events();
// label;
    unhook_irq_events();
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn osnoise_unhook_events() {
    unhook_thread_events();
    unhook_softirq_events();
    unhook_irq_events();
    }
//
// osnoise_workload_start - start the workload and hook to events
//
#[no_mangle]
unsafe extern "C" fn osnoise_workload_start() -> c_int {
    let mut retval = 0;
//
// Instances need to be registered after calling workload
// start. Hence, if there is already an instance, the
// workload was already registered. Otherwise, this
// code is on the way to register the first instance,
// and the workload will start.
//
    if (osnoise_has_registered_instances()) {
    return 0;
    }
    osn_var_reset_all();
    retval = osnoise_hook_events();
    if (retval) {
    return retval;
    }
//
// Make sure that ftrace_nmi_enter/exit() see reset values
// before enabling trace_osnoise_callback_enabled.
//
    barrier();
    trace_osnoise_callback_enabled = true;
    retval = start_per_cpu_kthreads();
    if (retval) {
    trace_osnoise_callback_enabled = false;
//
// Make sure that ftrace_nmi_enter/exit() see
// trace_osnoise_callback_enabled as false before continuing.
//
    barrier();
    osnoise_unhook_events();
    return retval;
    }
    return 0;
    }
//
// osnoise_workload_stop - stop the workload and unhook the events
//
#[no_mangle]
unsafe extern "C" fn osnoise_workload_stop() {
//
// Instances need to be unregistered before calling
// stop. Hence, if there is a registered instance, more
// than one instance is running, and the workload will not
// yet stop. Otherwise, this code is on the way to disable
// the last instance, and the workload can stop.
//
    if (osnoise_has_registered_instances()) {
    return;
    }
//
// If callbacks were already disabled in a previous stop
// call, there is no need to disable then again.
//
// For instance, this happens when tracing is stopped via:
// echo 0 > tracing_on
// echo nop > current_tracer.
//
    if (!trace_osnoise_callback_enabled) {
    return;
    }
    trace_osnoise_callback_enabled = false;
//
// Make sure that ftrace_nmi_enter/exit() see
// trace_osnoise_callback_enabled as false before continuing.
//
    barrier();
    stop_per_cpu_kthreads();
    osnoise_unhook_events();
    }
#[no_mangle]
unsafe extern "C" fn osnoise_tracer_start(tr: *mut trace_array) {
    let mut retval = 0;
//
// If the instance is already registered, there is no need to
// register it again.
//
    if (osnoise_instance_registered(tr)) {
    return;
    }
    retval = osnoise_workload_start();
    if (retval) {
    pr_err!(BANNER "Error starting osnoise tracer\n");
    }
    osnoise_register_instance(tr);
    }
#[no_mangle]
unsafe extern "C" fn osnoise_tracer_stop(tr: *mut trace_array) {
    osnoise_unregister_instance(tr);
    osnoise_workload_stop();
    }
#[no_mangle]
unsafe extern "C" fn osnoise_tracer_init(tr: *mut trace_array) -> c_int {
//
// Only allow osnoise tracer if timerlat tracer is not running
// already.
//
    if (timerlat_enabled()) {
    return -EBUSY;
    }
    tr.max_latency = 0;
    osnoise_tracer_start(tr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn osnoise_tracer_reset(tr: *mut trace_array) {
    osnoise_tracer_stop(tr);
    }
    static struct tracer osnoise_tracer  = {
    .name		= "osnoise",
    .init		= osnoise_tracer_init,
    .reset		= osnoise_tracer_reset,
    .start		= osnoise_tracer_start,
    .stop		= osnoise_tracer_stop,
    .print_header	= print_osnoise_headers,
    .allow_instances = true,
    };

#[no_mangle]
unsafe extern "C" fn timerlat_tracer_start(tr: *mut trace_array) {
    let mut retval = 0;
//
// If the instance is already registered, there is no need to
// register it again.
//
    if (osnoise_instance_registered(tr)) {
    return;
    }
    retval = osnoise_workload_start();
    if (retval) {
    pr_err!(BANNER "Error starting timerlat tracer\n");
    }
    osnoise_register_instance(tr);
    return;
    }
#[no_mangle]
unsafe extern "C" fn timerlat_tracer_stop(tr: *mut trace_array) {
    let mut cpu = 0;
    osnoise_unregister_instance(tr);
//
// Instruct the threads to stop only if this is the last instance.
//
    if (!osnoise_has_registered_instances()) {
    for_each_online_cpu(cpu) {
    per_cpu(per_cpu_osnoise_var, cpu).sampling = 0;
    }
    }
    osnoise_workload_stop();
    }
#[no_mangle]
unsafe extern "C" fn timerlat_tracer_init(tr: *mut trace_array) -> c_int {
//
// Only allow timerlat tracer if osnoise tracer is not running already.
//
    if (osnoise_has_registered_instances() && !osnoise_data.timerlat_tracer) {
    return -EBUSY;
    }
//
// If this is the first instance, set timerlat_tracer to block
// osnoise tracer start.
//
    if (!osnoise_has_registered_instances()) {
    osnoise_data.timerlat_tracer = 1;
    }
    tr.max_latency = 0;
    timerlat_tracer_start(tr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timerlat_tracer_reset(tr: *mut trace_array) {
    timerlat_tracer_stop(tr);
//
// If this is the last instance, reset timerlat_tracer allowing
// osnoise to be started.
//
    if (!osnoise_has_registered_instances()) {
    osnoise_data.timerlat_tracer = 0;
    }
    }
    static struct tracer timerlat_tracer  = {
    .name		= "timerlat",
    .init		= timerlat_tracer_init,
    .reset		= timerlat_tracer_reset,
    .start		= timerlat_tracer_start,
    .stop		= timerlat_tracer_stop,
    .print_header	= print_timerlat_headers,
    .allow_instances = true,
    };
#[no_mangle]
pub unsafe extern "C" fn init_timerlat_tracer() -> __init static int {
    return register_tracer(&timerlat_tracer);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: init_timerlat_tracer
pub unsafe extern "C" fn init_timerlat_tracer_dup() -> __init static int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn init_osnoise_tracer() -> __init static int {
    let mut ret = 0;
    mutex_init(&interface_lock);
    cpumask_copy(&osnoise_cpumask, cpu_all_mask);
    ret = register_tracer(&osnoise_tracer);
    if (ret) {
    pr_err!(BANNER "Error registering osnoise!\n");
    return ret;
    }
    ret = init_timerlat_tracer();
    if (ret) {
    pr_err!(BANNER "Error registering timerlat!\n");
    return ret;
    }
    osnoise_init_hotplug_support();
    INIT_LIST_HEAD_RCU(&osnoise_instances);
    init_tracefs();
    return 0;
    }
    late_initcall!(init_osnoise_tracer);