//! Automatically rewritten from C to Rust
//! Source: kernel/sched/idle.c
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
// Generic entry points for the idle threads and
// implementation of the idle task scheduling class.
//
// (NOTE: these are not related to SCHED_IDLE batch scheduled
// tasks which are handled in sched/fair.c )
//

// Linker adds these: start and end of __cpuidle functions
    extern char __cpuidle_text_start[], __cpuidle_text_end[];
//
// sched_idle_set_state - Record idle state for the current CPU.
// @idle_state: State to record.
//
#[no_mangle]
pub unsafe extern "C" fn sched_idle_set_state(idle_state: *mut cpuidle_state) {
    idle_set_state(this_rq(), idle_state);
    }
    static int  cpu_idle_force_poll;
#[no_mangle]
pub unsafe extern "C" fn cpu_idle_poll_ctrl(enable: bool) {
    if (enable) {
    cpu_idle_force_poll += 1;
    } else {
    cpu_idle_force_poll -= 1;
    WARN_ON_ONCE!(cpu_idle_force_poll < 0);
    }
    }

#[no_mangle]
unsafe extern "C" fn cpu_idle_poll_setup(__unused: *mut c_char) -> c_int {
    cpu_idle_force_poll = 1;
    return 1;
    }
    __setup!("nohlt", cpu_idle_poll_setup);
#[no_mangle]
unsafe extern "C" fn cpu_idle_nopoll_setup(__unused: *mut c_char) -> c_int {
    cpu_idle_force_poll = 0;
    return 1;
    }
    __setup!("hlt", cpu_idle_nopoll_setup);

#[no_mangle]
unsafe extern "C" fn cpu_idle_poll() -> noinline int __cpuidle {
    instrumentation_begin();
    trace_cpu_idle(0, smp_processor_id());
    stop_critical_timings();
    ct_cpuidle_enter();
    raw_local_irq_enable();
    while (!tif_need_resched() &&
    (cpu_idle_force_poll || tick_check_broadcast_expired())) {
    cpu_relax();
    }
    raw_local_irq_disable();
    ct_cpuidle_exit();
    start_critical_timings();
    trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id());
    local_irq_enable();
    instrumentation_end();
    return 1;
    }
// Weak implementations for optional arch specific functions
    void __weak arch_cpu_idle_prepare(void) { }
    void __weak arch_cpu_idle_enter(void) { }
    void __weak arch_cpu_idle_exit(void) { }
    void __weak __noreturn arch_cpu_idle_dead(void) { while (1); }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() -> void __weak {
    cpu_idle_force_poll = 1;
    }

pub static mut arch_needs_tick_broadcast: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cond_tick_broadcast_enter() {
    if (static_branch_unlikely(&arch_needs_tick_broadcast)) {
    tick_broadcast_enter();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cond_tick_broadcast_exit() {
    if (static_branch_unlikely(&arch_needs_tick_broadcast)) {
    tick_broadcast_exit();
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: cond_tick_broadcast_enter
pub unsafe extern "C" fn cond_tick_broadcast_enter_dup() { }
#[no_mangle]
#[no_mangle]
// duplicate fn: cond_tick_broadcast_exit
pub unsafe extern "C" fn cond_tick_broadcast_exit_dup() { }

//
// default_idle_call - Default CPU idle routine.
//
// To use when the cpuidle framework cannot be used.
//
#[no_mangle]
pub unsafe extern "C" fn default_idle_call() -> void __cpuidle {
    instrumentation_begin();
    if (!current_clr_polling_and_test()) {
    cond_tick_broadcast_enter();
    trace_cpu_idle(1, smp_processor_id());
    stop_critical_timings();
    ct_cpuidle_enter();
    arch_cpu_idle();
    ct_cpuidle_exit();
    start_critical_timings();
    trace_cpu_idle(PWR_EVENT_EXIT, smp_processor_id());
    cond_tick_broadcast_exit();
    }
    local_irq_enable();
    instrumentation_end();
    }
#[no_mangle]
pub unsafe extern "C" fn call_cpuidle_s2idle(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, max_latency_ns: u64) -> c_int {
    if (current_clr_polling_and_test()) {
    return -EBUSY;
    }
    return cpuidle_enter_s2idle(drv, dev, max_latency_ns);
    }
#[no_mangle]
pub unsafe extern "C" fn call_cpuidle(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, next_state: c_int) -> c_int {
//
// The idle task must be scheduled, it is pointless to go to idle, just
// update no idle residency and return.
//
    if (current_clr_polling_and_test()) {
    dev.last_residency_ns = 0;
    local_irq_enable();
    return -EBUSY;
    }
//
// Enter the idle state previously returned by the governor decision.
// This function will block until an interrupt occurs and will take
// care of re-enabling the local interrupts
//
    return cpuidle_enter(drv, dev, next_state);
    }
#[no_mangle]
unsafe extern "C" fn idle_call_stop_or_retain_tick(stop_tick: bool) {
    if (stop_tick || tick_nohz_tick_stopped()) {
    tick_nohz_idle_stop_tick();
    }
    else {
    tick_nohz_idle_retain_tick();
    }
    }
//
// cpuidle_idle_call - the main idle function
//
// NOTE: no locks or semaphores should be used here
//
// On architectures that support TIF_POLLING_NRFLAG, is called with polling
// set, and it returns with polling set.  If it ever stops polling, it
// must clear the polling bit.
//
#[no_mangle]
unsafe extern "C" fn cpuidle_idle_call(stop_tick: bool) {
    let mut dev = cpuidle_get_device();
    let mut drv = cpuidle_get_cpu_driver(dev);
    let mut next_state = 0;
    let mut entered_state = 0;
//
// Check if the idle task must be rescheduled. If it is the
// case, exit the function after re-enabling the local IRQ.
//
    if (need_resched()) {
    local_irq_enable();
    return;
    }
    if (cpuidle_not_available(drv, dev)) {
    idle_call_stop_or_retain_tick(stop_tick);
    default_idle_call();
// goto;
    }
//
// Suspend-to-idle ("s2idle") is a system state in which all user space
// has been frozen, all I/O devices have been suspended and the only
// activity happens here and in interrupts (if any). In that case bypass
// the cpuidle governor and go straight for the deepest idle state
// available.  Possibly also suspend the local tick and the entire
// timekeeping to prevent timer interrupts from kicking us out of idle
// until a proper wakeup interrupt happens.
//
    if (idle_should_enter_s2idle() || dev.forced_idle_latency_limit_ns) {
    let mut max_latency_ns = 0;
    if (idle_should_enter_s2idle()) {
    max_latency_ns = cpu_wakeup_latency_qos_limit() *
    NSEC_PER_USEC;
    entered_state = call_cpuidle_s2idle(drv, dev,
    max_latency_ns);
    if (entered_state > 0) {
// goto;
    }
    } else {
    max_latency_ns = dev.forced_idle_latency_limit_ns;
    }
    tick_nohz_idle_stop_tick();
    next_state = cpuidle_find_deepest_state(drv, dev, max_latency_ns);
    call_cpuidle(drv, dev, next_state);
    } else if (drv.state_count > 1) {
//
// stop_tick is expected to be true by default by cpuidle
// governors, which allows them to select idle states with
// target residency above the tick period length.
//
    stop_tick = true;
//
// Ask the cpuidle framework to choose a convenient idle state.
//
    next_state = cpuidle_select(drv, dev, &stop_tick);
    idle_call_stop_or_retain_tick(stop_tick);
    entered_state = call_cpuidle(drv, dev, next_state);
//
// Give the governor an opportunity to reflect on the outcome
//
    cpuidle_reflect(dev, entered_state);
    } else {
    idle_call_stop_or_retain_tick(stop_tick);
//
// If there is only a single idle state (or none), there is
// nothing meaningful for the governor to choose.  Skip the
// governor and always use state 0.
//
    call_cpuidle(drv, dev, 0);
    }
// label;
    __current_set_polling();
//
// It is up to the idle functions to re-enable local interrupts
//
    if (WARN_ON_ONCE!(irqs_disabled())) {
    local_irq_enable();
    }
    }
//
// Generic idle loop implementation
//
// Called with polling cleared.
//
#[no_mangle]
unsafe extern "C" fn do_idle() {
pub static mut cpu: c_int = 0;
pub static mut got_tick: bool = false;
    if (cpu_is_offline(cpu)) {
    local_irq_disable();
// All per-CPU kernel threads should be done by now.
    WARN_ON_ONCE!(need_resched());
    cpuhp_report_idle_dead();
    arch_cpu_idle_dead();
    }
//
// Check if we need to update blocked load
//
    nohz_run_idle_balance(cpu);
//
// If the arch has a polling bit, we maintain an invariant:
//
// Our polling bit is clear if we're not scheduled (i.e. if rq->curr !=
// rq->idle). This means that, if rq->idle has the polling bit set,
// then setting need_resched is guaranteed to cause the CPU to
// reschedule.
//
    __current_set_polling();
    tick_nohz_idle_enter();
    while (!need_resched()) {
//
// Interrupts shouldn't be re-enabled from that point on until
// the CPU sleeping instruction is reached. Otherwise an interrupt
// may fire and queue a timer that would be ignored until the CPU
// wakes from the sleeping instruction. And testing need_resched()
// doesn't tell about pending needed timer reprogram.
//
// Several cases to consider:
//
// - SLEEP-UNTIL-PENDING-INTERRUPT based instructions such as
// "wfi" or "mwait" are fine because they can be entered with
// interrupt disabled.
//
// - sti;mwait() couple is fine because the interrupts are
// re-enabled only upon the execution of mwait, leaving no gap
// in-between.
//
// - ROLLBACK based idle handlers with the sleeping instruction
// called with interrupts enabled are NOT fine. In this scheme
// when the interrupt detects it has interrupted an idle handler,
// it rolls back to its beginning which performs the
// need_resched() check before re-executing the sleeping
// instruction. This can leak a pending needed timer reprogram.
// If such a scheme is really mandatory due to the lack of an
// appropriate CPU sleeping instruction, then a FAST-FORWARD
// must instead be applied: when the interrupt detects it has
// interrupted an idle handler, it must resume to the end of
// this idle handler so that the generic idle loop is iterated
// again to reprogram the tick.
//
    local_irq_disable();
    arch_cpu_idle_enter();
    rcu_nocb_flush_deferred_wakeup();
//
// In poll mode we re-enable interrupts and spin. Also if we
// detected in the wakeup from idle path that the tick
// broadcast device expired for us, we don't want to go deep
// idle as we know that the IPI is going to arrive right away.
//
    if (cpu_idle_force_poll || tick_check_broadcast_expired()) {
    tick_nohz_idle_restart_tick();
    cpu_idle_poll();
    } else {
    cpuidle_idle_call(got_tick);
    }
    got_tick = tick_nohz_idle_got_tick();
    arch_cpu_idle_exit();
    }
//
// Since we fell out of the loop above, we know TIF_NEED_RESCHED must
// be set, propagate it into PREEMPT_NEED_RESCHED.
//
// This is required because for polling idle loops we will not have had
// an IPI to fold the state for us.
//
    preempt_set_need_resched();
    tick_nohz_idle_exit();
    __current_clr_polling();
//
// We promise to call sched_ttwu_pending() and reschedule if
// need_resched() is set while polling is set. That means that clearing
// polling needs to be visible before doing these things.
//
    smp_mb__after_atomic();
//
// RCU relies on this call to be done outside of an RCU read-side
// critical section.
//
    flush_smp_call_function_queue();
    schedule_idle();
    if (unlikely(klp_patch_pending(current))) {
    klp_update_patch_state(current);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_in_idle(pc: c_ulong) -> bool {
    return pc >= (unsigned long)__cpuidle_text_start &&
    pc < (unsigned long)__cpuidle_text_end;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idle_timer {
    pub timer: hrtimer,
    pub done: c_int,
}

#[no_mangle]
unsafe extern "C" fn idle_inject_timer_fn(timer: *mut hrtimer) -> enum hrtimer_restart {
    let mut it = container_of!(timer, idle_timer, timer);
    WRITE_ONCE(it.done, 1);
    set_tsk_need_resched(current);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn play_idle_precise(duration_ns: u64, latency_ns: u64) {
pub static mut it: usize = 0;
//
// Only FIFO tasks can disable the tick since they don't need the forced
// preemption.
//
    WARN_ON_ONCE!(current.policy != SCHED_FIFO);
    WARN_ON_ONCE!(current.nr_cpus_allowed != 1);
    WARN_ON_ONCE!(!(current.flags & PF_KTHREAD));
    WARN_ON_ONCE!(!(current.flags & PF_NO_SETAFFINITY));
    WARN_ON_ONCE!(!duration_ns);
    WARN_ON_ONCE!(current.mm);
    rcu_sleep_check();
    preempt_disable();
    current.flags |= PF_IDLE;
    cpuidle_use_deepest_state(latency_ns);
    it.done = 0;
    hrtimer_setup_on_stack(&it.timer, idle_inject_timer_fn, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL_HARD);
    hrtimer_start(&it.timer, ns_to_ktime(duration_ns),
    HRTIMER_MODE_REL_PINNED_HARD);
    while (!READ_ONCE(it.done)) {
    do_idle();
    }
    cpuidle_use_deepest_state(0);
    current.flags &= ~PF_IDLE;
    preempt_fold_need_resched();
    preempt_enable();
    }
    EXPORT_SYMBOL_GPL(play_idle_precise);
#[no_mangle]
pub unsafe extern "C" fn cpu_startup_entry(state: cpuhp_state) {
    current.flags |= PF_IDLE;
    arch_cpu_idle_prepare();
    cpuhp_online_idle(state);
    while (1) {
    do_idle();
    }
    }
//
// idle-task scheduling class.
//
#[no_mangle]
pub unsafe extern "C" fn select_task_rq_idle(p: *mut task_struct, cpu: c_int, flags: c_int) -> c_int {
    return task_cpu(p); /* IDLE tasks as never migrated */
    }
#[no_mangle]
pub unsafe extern "C" fn balance_idle(rq: *mut rq, rf: *mut rq_flags) -> c_int {
    return WARN_ON_ONCE!(1);
    }
//
// Idle tasks are unconditionally rescheduled:
//
#[no_mangle]
unsafe extern "C" fn wakeup_preempt_idle(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    resched_curr(rq);
    }
// forward_decl: update_curr_idle;
#[no_mangle]
unsafe extern "C" fn put_prev_task_idle(rq: *mut rq, prev: *mut task_struct, next: *mut task_struct) {
    update_curr_idle(rq);
    scx_update_idle(rq, false, true);
    update_rq_avg_idle(rq);
    }
#[no_mangle]
unsafe extern "C" fn set_next_task_idle(rq: *mut rq, next: *mut task_struct, first: bool) {
    update_idle_core(rq);
    scx_update_idle(rq, true, true);
    schedstat_inc(rq.sched_goidle);
    next.se.exec_start = rq_clock_task(rq);
//
// rq is about to be idle, check if we need to update the
// lost_idle_time of clock_pelt
//
    update_idle_rq_clock_pelt(rq);
    }
#[no_mangle]
pub unsafe extern "C" fn pick_task_idle(rq: *mut rq, rf: *mut rq_flags) -> *mut c_void {
//
// Notify scx only on an idle-to-idle re-pick (the cpu was already idle).
// A real task->idle transition is delivered by set_next_task_idle(), so
// calling here too would duplicate it.
//
    if (scx_enabled() && is_idle_task(rq.curr)) {
    scx_update_idle(rq, true, false);
    }
    return rq.idle;
    }
//
// It is not legal to sleep in the idle task - print a warning
// message if some code attempts to do it:
//
#[no_mangle]
pub unsafe extern "C" fn dequeue_task_idle(rq: *mut rq, p: *mut task_struct, flags: c_int) -> bool {
    raw_spin_rq_unlock_irq(rq);
    printk("bad: scheduling from the idle thread!\n");
    dump_stack();
    raw_spin_rq_lock_irq(rq);
    return true;
    }
//
// scheduler tick hitting a task of our scheduling class.
//
// NOTE: This function can be called remotely by the tick offload that
// goes along full dynticks. Therefore no local assumption can be made
// and everything must be accessed through the @rq and @curr passed in
// parameters.
//
#[no_mangle]
unsafe extern "C" fn task_tick_idle(rq: *mut rq, curr: *mut task_struct, queued: c_int) {
    update_curr_idle(rq);
    }
#[no_mangle]
unsafe extern "C" fn switching_to_idle(rq: *mut rq, p: *mut task_struct) {
    BUG();
    }
#[no_mangle]
pub unsafe extern "C" fn prio_changed_idle(rq: *mut rq, p: *mut task_struct, oldprio: u64) {
    if (p.prio == oldprio) {
    return;
    }
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn update_curr_idle(rq: *mut rq) {
    let mut se = &rq.idle.se;
pub static mut now: u64 = 0;
    let mut delta_exec = 0;
    delta_exec = now - se.exec_start;
    if (unlikely(delta_exec <= 0)) {
    return;
    }
    se.exec_start = now;
    dl_server_update_idle(&rq.fair_server, delta_exec);

    dl_server_update_idle(&rq.ext_server, delta_exec);

    }
//
// Simple, special scheduling class for the per-CPU idle tasks:
//
    DEFINE_SCHED_CLASS(idle) = {
// no enqueue/yield_task for idle tasks
// dequeue is not valid, we print a debug message there:
    .dequeue_task		= dequeue_task_idle,
    .wakeup_preempt		= wakeup_preempt_idle,
    .pick_task		= pick_task_idle,
    .put_prev_task		= put_prev_task_idle,
    .set_next_task          = set_next_task_idle,
    .balance		= balance_idle,
    .select_task_rq		= select_task_rq_idle,
    .set_cpus_allowed	= set_cpus_allowed_common,
    .task_tick		= task_tick_idle,
    .prio_changed		= prio_changed_idle,
    .switching_to		= switching_to_idle,
    .update_curr		= update_curr_idle,
    };