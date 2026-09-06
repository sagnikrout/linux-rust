//! Automatically rewritten from C to Rust
//! Source: kernel/stop_machine.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// kernel/stop_machine.c
//
// Copyright (C) 2008, 2005	IBM Corporation.
// Copyright (C) 2008, 2005	Rusty Russell rusty@rustcorp.com.au
// Copyright (C) 2010		SUSE Linux Products GmbH
// Copyright (C) 2010		Tejun Heo <tj@kernel.org>
//

//
// Structure to determine completion condition and record errors.  May
// be shared by works on different cpus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_stop_done {
//     pub /: *mut *mut atomic_t nr_todo; / nr left to execute,
//     pub /: *mut *mut int ret; / collected return value,
//     pub /: *mut *mut completion completion; / fired if nr_todo reaches 0,
}

// the actual stopper, one per every possible cpu, enabled on online cpus
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_stopper {
    pub thread: *mut task_struct,
    pub lock: raw_spinlock_t,
//     pub /: *mut *mut bool enabled; / is this stopper enabled?,
//     pub /: *mut *mut list_head works; / list of pending works,
//     pub /: *mut *mut cpu_stop_work stop_work; / for stop_cpus,
    pub caller: c_ulong,
    pub fn: cpu_stop_fn_t,
}
// static DEFINE_PER_CPU(cpu_stopper, cpu_stopper);
pub static mut stop_machine_initialized: bool = false;
#[no_mangle]
pub unsafe extern "C" fn print_stop_info(log_lvl: *const c_char, task: *mut task_struct) {
//
// If @task is a stopper task, it cannot migrate and task_cpu() is
// stable.
//
    let mut stopper = per_cpu_ptr(&cpu_stopper, task_cpu(task));
    if (task != stopper.thread) {
    return;
    }
    printk("%sStopper: %pS <- %pS\n", log_lvl, stopper.fn, stopper.caller);
    }
// static data for stop_cpus
// static DEFINE_MUTEX(stop_cpus_mutex);
    static bool stop_cpus_in_progress;
#[no_mangle]
unsafe extern "C" fn cpu_stop_init_done(done: *mut cpu_stop_done, nr_todo: c_uint) {
    memset(done, 0, sizeof!(*done));
    atomic_set(&done.nr_todo, nr_todo);
    init_completion(&done.completion);
    }
// signal completion unless @done is NULL
#[no_mangle]
unsafe extern "C" fn cpu_stop_signal_done(done: *mut cpu_stop_done) {
    if (atomic_dec_and_test(&done.nr_todo)) {
    complete(&done.completion);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_stop_queue_work(stopper: *mut cpu_stopper, work: *mut cpu_stop_work) {
    list_add_tail(&work.list, &stopper.works);
    }
// queue @work to @stopper.  if offline, @work is completed immediately
#[no_mangle]
unsafe extern "C" fn cpu_stop_queue_work(cpu: c_uint, work: *mut cpu_stop_work) -> bool {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
    let mut flags = 0;
    let mut enabled = 0;
    preempt_disable();
    raw_spin_lock_irqsave(&stopper.lock, flags);
    enabled = stopper.enabled;
    if (enabled) {
    __cpu_stop_queue_work(stopper, work);
    }

    else if (work.done) {
    cpu_stop_signal_done(work.done);
    }
    raw_spin_unlock_irqrestore(&stopper.lock, flags);
    if (enabled) {
    wake_up_process(stopper.thread);
    }
    preempt_enable();
    return enabled;
    }
//
// stop_one_cpu - stop a cpu
// @cpu: cpu to stop
// @fn: function to execute
// @arg: argument to @fn
//
// Execute @fn(@arg) on @cpu.  @fn is run in a process context with
// the highest priority preempting any task on the cpu and
// monopolizing it.  This function returns after the execution is
// complete.
//
// This function doesn't guarantee @cpu stays online till @fn
// completes.  If @cpu goes down in the middle, execution may happen
// partially or fully on different cpus.  @fn should either be ready
// for that or the caller should ensure that @cpu stays online until
// this function completes.
//
// CONTEXT:
// Might sleep.
//
// RETURNS:
// -ENOENT if @fn(@arg) was not executed because @cpu was offline;
// otherwise, the return value of @fn.
//
#[no_mangle]
pub unsafe extern "C" fn stop_one_cpu(cpu: c_uint, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int {
pub static mut done: usize = 0;
pub static mut work: cpu_stop_work = 0;
    cpu_stop_init_done(&done, 1);
    if (!cpu_stop_queue_work(cpu, &work)) {
    return -ENOENT;
    }
//
// In case @cpu == smp_proccessor_id() we can avoid a sleep+wakeup
// cycle by doing a preemption:
//
    cond_resched();
    wait_for_completion(&done.completion);
    return done.ret;
    }
// This controls the threads on each CPU.
    enum multi_stop_state {
// Dummy starting state for thread.
    MULTI_STOP_NONE,
// Awaiting everyone to be scheduled.
    MULTI_STOP_PREPARE,
// Disable interrupts.
    MULTI_STOP_DISABLE_IRQ,
// Run the function
    MULTI_STOP_RUN,
// Exit
    MULTI_STOP_EXIT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_stop_data {
    pub fn: cpu_stop_fn_t,
    pub data: *mut c_void,
// Like num_online_cpus(), but hotplug cpu uses us, so we need this.
    pub num_threads: c_uint,
    pub active_cpus: *const cpumask,
    pub state: multi_stop_state,
    pub thread_ack: core::sync::atomic::AtomicI32,
}

#[no_mangle]
pub unsafe extern "C" fn set_state(msdata: *mut multi_stop_data, newstate: multi_stop_state) {
// Reset ack counter.
    atomic_set(&msdata.thread_ack, msdata.num_threads);
    smp_wmb();
    WRITE_ONCE(msdata.state, newstate);
    }
// Last one to ack a state moves to the next state.
#[no_mangle]
unsafe extern "C" fn ack_state(msdata: *mut multi_stop_data) {
    if (atomic_dec_and_test(&msdata.thread_ack)) {
    set_state(msdata, msdata.state + 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn stop_machine_yield(cpumask: *const cpumask) -> notrace void __weak {
    cpu_relax();
    }
// This is the cpu_stop function which stops the CPU.
#[no_mangle]
unsafe extern "C" fn multi_cpu_stop(data: *mut c_void) -> c_int {
    let mut msdata = data;
    enum multi_stop_state newstate, curstate = MULTI_STOP_NONE;
pub static mut cpu: c_int = 0;
pub static mut cpumask: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut is_active = 0;
//
// When called from stop_machine_from_inactive_cpu(), irq might
// already be disabled.  Save the state and restore it on exit.
//
    local_save_flags(flags);
    if (!msdata.active_cpus) {
    cpumask = cpu_online_mask;
    is_active = cpu == cpumask_first(cpumask);
    } else {
    cpumask = msdata.active_cpus;
    is_active = cpumask_test_cpu(cpu, cpumask);
    }
// Simple state machine
    do {
// Chill out and ensure we re-read multi_stop_state.
    stop_machine_yield(cpumask);
    newstate = READ_ONCE(msdata.state);
    if (newstate != curstate) {
    curstate = newstate;
    match (curstate) {
    MULTI_STOP_DISABLE_IRQ => {
    local_irq_disable();
    hard_irq_disable();
    // break;
    }
    MULTI_STOP_RUN => {
    if (is_active) {
    err = msdata.fn(msdata.data);
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    ack_state(msdata);
    } else if (curstate > MULTI_STOP_PREPARE) {
//
// At this stage all other CPUs we depend on must spin
// in the same loop. Any reason for hard-lockup should
// be detected and reported on their side.
//
    touch_nmi_watchdog();
// Also suppress RCU CPU stall warnings.
    rcu_momentary_eqs();
    }
    } while (curstate != MULTI_STOP_EXIT);
    local_irq_restore(flags);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_stop_queue_two_works(cpu1: c_int, work1: *mut cpu_stop_work, cpu2: c_int, work2: *mut cpu_stop_work) -> c_int {
    let mut stopper1 = per_cpu_ptr(&cpu_stopper, cpu1);
    let mut stopper2 = per_cpu_ptr(&cpu_stopper, cpu2);
    let mut err = 0;
// label;
//
// The waking up of stopper threads has to happen in the same
// scheduling context as the queueing.  Otherwise, there is a
// possibility of one of the above stoppers being woken up by another
// CPU, and preempting us. This will cause us to not wake up the other
// stopper forever.
//
    preempt_disable();
    raw_spin_lock_irq(&stopper1.lock);
    raw_spin_lock_nested(&stopper2.lock, SINGLE_DEPTH_NESTING);
    if (!stopper1.enabled || !stopper2.enabled) {
    err = -ENOENT;
// goto;
    }
//
// Ensure that if we race with __stop_cpus() the stoppers won't get
// queued up in reverse order leading to system deadlock.
//
// We can't miss stop_cpus_in_progress if queue_stop_cpus_work() has
// queued a work on cpu1 but not on cpu2, we hold both locks.
//
// It can be falsely true but it is safe to spin until it is cleared,
// queue_stop_cpus_work() does everything under preempt_disable().
//
    if (unlikely(stop_cpus_in_progress)) {
    err = -EDEADLK;
// goto;
    }
    err = 0;
    __cpu_stop_queue_work(stopper1, work1);
    __cpu_stop_queue_work(stopper2, work2);
// label;
    raw_spin_unlock(&stopper2.lock);
    raw_spin_unlock_irq(&stopper1.lock);
    if (unlikely(err == -EDEADLK)) {
    preempt_enable();
    while (stop_cpus_in_progress) {
    cpu_relax();
    }
// goto;
    }
    if (!err) {
    wake_up_process(stopper1.thread);
    wake_up_process(stopper2.thread);
    }
    preempt_enable();
    return err;
    }
//
// stop_two_cpus - stops two cpus
// @cpu1: the cpu to stop
// @cpu2: the other cpu to stop
// @fn: function to execute
// @arg: argument to @fn
//
// Stops both the current and specified CPU and runs @fn on one of them.
//
// returns when both are completed.
//
#[no_mangle]
pub unsafe extern "C" fn stop_two_cpus(cpu1: c_uint, cpu2: c_uint, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int {
pub static mut done: usize = 0;
    struct cpu_stop_work work1, work2;
pub static mut msdata: usize = 0;
    msdata = (multi_stop_data){
    .fn = fn,
    .data = arg,
    .num_threads = 2,
    .active_cpus = cpumask_of(cpu1),
    };
    work1 = work2 = (cpu_stop_work){
    .fn = multi_cpu_stop,
    .arg = &msdata,
    .done = &done,
    .caller = _RET_IP_,
    };
    cpu_stop_init_done(&done, 2);
    set_state(&msdata, MULTI_STOP_PREPARE);
    if (cpu1 > cpu2) {
    swap(cpu1, cpu2);
    }
    if (cpu_stop_queue_two_works(cpu1, &work1, cpu2, &work2)) {
    return -ENOENT;
    }
    wait_for_completion(&done.completion);
    return done.ret;
    }
//
// stop_one_cpu_nowait - stop a cpu but don't wait for completion
// @cpu: cpu to stop
// @fn: function to execute
// @arg: argument to @fn
// @work_buf: pointer to cpu_stop_work structure
//
// Similar to stop_one_cpu() but doesn't wait for completion.  The
// caller is responsible for ensuring @work_buf is currently unused
// and will remain untouched until stopper starts executing @fn.
//
// CONTEXT:
// Don't care, but the caller must ensure @cpu's stopper stays enabled
// until the work is queued, e.g. by preempt_disable().
//
#[no_mangle]
pub unsafe extern "C" fn stop_one_cpu_nowait(cpu: c_uint, fn: cpu_stop_fn_t, arg: *mut c_void, work_buf: *mut cpu_stop_work) {
// work_buf = (cpu_stop_work){ .fn = fn, .arg = arg, .caller = _RET_IP_, };
    WARN_ON_ONCE!(!cpu_stop_queue_work(cpu, work_buf));
    }
#[no_mangle]
pub unsafe extern "C" fn queue_stop_cpus_work(cpumask: *mut cpumask, fn: cpu_stop_fn_t, arg: *mut c_void, done: *mut cpu_stop_done) -> bool {
pub static mut work: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
pub static mut queued: bool = false;
//
// Disable preemption while queueing to avoid getting
// preempted by a stopper which might wait for other stoppers
// to enter @fn which can lead to deadlock.
//
    preempt_disable();
    stop_cpus_in_progress = true;
    barrier();
    for_each_cpu(cpu, cpumask) {
    work = &per_cpu(cpu_stopper.stop_work, cpu);
    work.fn = fn;
    work.arg = arg;
    work.done = done;
    work.caller = _RET_IP_;
    if (cpu_stop_queue_work(cpu, work)) {
    queued = true;
    }
    }
    barrier();
    stop_cpus_in_progress = false;
    preempt_enable();
    return queued;
    }
#[no_mangle]
pub unsafe extern "C" fn __stop_cpus(cpumask: *mut cpumask, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int {
pub static mut done: usize = 0;
    cpu_stop_init_done(&done, cpumask_weight(cpumask));
    if (!queue_stop_cpus_work(cpumask, fn, arg, &done)) {
    return -ENOENT;
    }
    wait_for_completion(&done.completion);
    return done.ret;
    }
//
// stop_cpus - stop multiple cpus
// @cpumask: cpus to stop
// @fn: function to execute
// @arg: argument to @fn
//
// Execute @fn(@arg) on online cpus in @cpumask.  On each target cpu,
// @fn is run in a process context with the highest priority
// preempting any task on the cpu and monopolizing it.  This function
// returns after all executions are complete.
//
// This function doesn't guarantee the cpus in @cpumask stay online
// till @fn completes.  If some cpus go down in the middle, execution
// on the cpu may happen partially or fully on different cpus.  @fn
// should either be ready for that or the caller should ensure that
// the cpus stay online until this function completes.
//
// All stop_cpus() calls are serialized making it safe for @fn to wait
// for all cpus to start executing it.
//
// CONTEXT:
// Might sleep.
//
// RETURNS:
// -ENOENT if @fn(@arg) was not executed at all because all cpus in
// @cpumask were offline; otherwise, 0 if all executions of @fn
// returned 0, any non zero return value if any returned non zero.
//
#[no_mangle]
unsafe extern "C" fn stop_cpus(cpumask: *const cpumask, fn: cpu_stop_fn_t, arg: *mut c_void) -> c_int {
    let mut ret = 0;
// static works are used, process one request at a time
    mutex_lock(&stop_cpus_mutex);
    ret = __stop_cpus(cpumask, fn, arg);
    mutex_unlock(&stop_cpus_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_stop_should_run(cpu: c_uint) -> c_int {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
    let mut flags = 0;
    let mut run = 0;
    raw_spin_lock_irqsave(&stopper.lock, flags);
    run = !list_empty(&stopper.works);
    raw_spin_unlock_irqrestore(&stopper.lock, flags);
    return run;
    }
#[no_mangle]
unsafe extern "C" fn cpu_stopper_thread(cpu: c_uint) {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
pub static mut work: *mut c_void = core::ptr::null_mut();
// label;
    work = core::ptr::null_mut();
    raw_spin_lock_irq(&stopper.lock);
    if (!list_empty(&stopper.works)) {
    work = list_first_entry(&stopper.works, cpu_stop_work, list);
    list_del_init(&work.list);
    }
    raw_spin_unlock_irq(&stopper.lock);
    if (work) {
pub static mut fn: cpu_stop_fn_t = 0;
    let mut arg = work.arg;
    let mut done = work.done;
    let mut ret = 0;
// cpu stop callbacks must not sleep, make in_atomic() == T
    stopper.caller = work.caller;
    stopper.fn = fn;
    preempt_count_inc();
    ret = fn(arg);
    if (done) {
    if (ret) {
    done.ret = ret;
    }
    cpu_stop_signal_done(done);
    }
    preempt_count_dec();
    stopper.fn = core::ptr::null_mut();
    stopper.caller = 0;
    WARN_ONCE(preempt_count(),
    "cpu_stop: %ps(%p) leaked preempt count\n", fn, arg);
// goto;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn stop_machine_park(cpu: c_int) {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
//
// Lockless. cpu_stopper_thread() will take stopper->lock and flush
// the pending works before it parks, until then it is fine to queue
// the new works.
//
    stopper.enabled = false;
    kthread_park(stopper.thread);
    }
#[no_mangle]
unsafe extern "C" fn cpu_stop_create(cpu: c_uint) {
    sched_set_stop_task(cpu, per_cpu(cpu_stopper.thread, cpu));
    }
#[no_mangle]
unsafe extern "C" fn cpu_stop_park(cpu: c_uint) {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
    WARN_ON!(!list_empty(&stopper.works));
    }
#[no_mangle]
pub unsafe extern "C" fn stop_machine_unpark(cpu: c_int) {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
    stopper.enabled = true;
    kthread_unpark(stopper.thread);
    }
pub static mut smp_hotplug_thread: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpu_stop_init() -> c_int {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    let mut stopper = &per_cpu(cpu_stopper, cpu);
    raw_spin_lock_init(&stopper.lock);
    INIT_LIST_HEAD(&stopper.works);
    }
    BUG_ON!(smpboot_register_percpu_thread(&cpu_stop_threads));
    stop_machine_unpark(raw_smp_processor_id());
    stop_machine_initialized = true;
    return 0;
    }
    early_initcall!(cpu_stop_init);
#[no_mangle]
pub unsafe extern "C" fn stop_machine_cpuslocked(fn: cpu_stop_fn_t, data: *mut c_void, cpus: *mut cpumask) -> c_int {
pub static mut multi_stop_data: usize = 0;
    lockdep_assert_cpus_held();
    if (!stop_machine_initialized) {
//
// Handle the case where stop_machine() is called
// early in boot before stop_machine() has been
// initialized.
//
    let mut flags = 0;
    let mut ret = 0;
    WARN_ON_ONCE!(msdata.num_threads != 1);
    local_irq_save(flags);
    hard_irq_disable();
    ret = (*fn)(data);
    local_irq_restore(flags);
    return ret;
    }
// Set the initial state and stop all online cpus.
    set_state(&msdata, MULTI_STOP_PREPARE);
    return stop_cpus(cpu_online_mask, multi_cpu_stop, &msdata);
    }
#[no_mangle]
pub unsafe extern "C" fn stop_machine(fn: cpu_stop_fn_t, data: *mut c_void, cpus: *const cpumask) -> c_int {
    let mut ret = 0;
// No CPUs can come up or down during this.
    cpus_read_lock();
    ret = stop_machine_cpuslocked(fn, data, cpus);
    cpus_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(stop_machine);

//
// INTEL_IFS is the only user of this API. That selftest can
// only be compiled if SMP=y. On x86 it selects SCHED_SMT.
// Keep the ifdefs for now.
//
#[no_mangle]
pub unsafe extern "C" fn stop_core_cpuslocked(cpu: c_uint, fn: cpu_stop_fn_t, data: *mut c_void) -> c_int {
    let mut smt_mask = cpu_smt_mask(cpu);
pub static mut multi_stop_data: usize = 0;
    lockdep_assert_cpus_held();
// Set the initial state and stop all online cpus.
    set_state(&msdata, MULTI_STOP_PREPARE);
    return stop_cpus(smt_mask, multi_cpu_stop, &msdata);
    }
    EXPORT_SYMBOL_GPL(stop_core_cpuslocked);

//
// stop_machine_from_inactive_cpu - stop_machine() from inactive CPU
// @fn: the function to run
// @data: the data ptr for the @fn()
// @cpus: the cpus to run the @fn() on (NULL = any online cpu)
//
// This is identical to stop_machine() but can be called from a CPU which
// is not active.  The local CPU is in the process of hotplug (so no other
// CPU hotplug can start) and not marked active and doesn't have enough
// context to sleep.
//
// This function provides stop_machine() functionality for such state by
// using busy-wait for synchronization and executing @fn directly for local
// CPU.
//
// CONTEXT:
// Local CPU is inactive.  Temporarily stops all active CPUs.
//
// RETURNS:
// 0 if all executions of @fn returned 0, any non zero return value if any
// returned non zero.
//
#[no_mangle]
pub unsafe extern "C" fn stop_machine_from_inactive_cpu(fn: cpu_stop_fn_t, data: *mut c_void, cpus: *mut cpumask) -> c_int {
pub static mut multi_stop_data: usize = 0;
pub static mut done: usize = 0;
    let mut ret = 0;
// Local CPU must be inactive and CPU hotplug in progress.
    BUG_ON!(cpu_active(raw_smp_processor_id()));
    msdata.num_threads = num_active_cpus() + 1;	/* +1 for local */
// No proper task established and can't sleep - busy wait for lock.
    while (!mutex_trylock(&stop_cpus_mutex)) {
    cpu_relax();
    }
// Schedule work on other CPUs and execute directly for local CPU
    set_state(&msdata, MULTI_STOP_PREPARE);
    cpu_stop_init_done(&done, num_active_cpus());
    queue_stop_cpus_work(cpu_active_mask, multi_cpu_stop, &msdata,
    &done);
    ret = multi_cpu_stop(&msdata);
// Busy wait for completion.
    while (!completion_done(&done.completion)) {
    cpu_relax();
    }
    mutex_unlock(&stop_cpus_mutex);
    return ret ?: done.ret;
    }