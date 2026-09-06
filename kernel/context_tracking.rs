//! Automatically rewritten from C to Rust
//! Source: kernel/context_tracking.c
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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Context tracking: Probe on high level context boundaries such as kernel,
// userspace, guest or idle.
//
// This is used by RCU to remove its dependency on the timer tick while a CPU
// runs in idle, userspace or guest mode.
//
// User/guest tracking started by Frederic Weisbecker:
//
// Copyright (C) 2012 Red Hat, Inc., Frederic Weisbecker
//
// Many thanks to Gilad Ben-Yossef, Paul McKenney, Ingo Molnar, Andrew Morton,
// Steven Rostedt, Peter Zijlstra for suggestions and improvements.
//
// RCU extended quiescent state bits imported from kernel/rcu/tree.c
// where the relevant authorship may be found.
//

    DEFINE_PER_CPU(struct context_tracking, context_tracking) = {

    .nesting = 1,
    .nmi_nesting = CT_NESTING_IRQ_NONIDLE,

    .state = ATOMIC_INIT(CT_RCU_WATCHING),
    };
// EXPORT_SYMBOL_GPL;

// Record the current task on exiting RCU-tasks (dyntick-idle entry).
#[no_mangle]
unsafe extern "C" fn rcu_task_exit() -> __always_inline void {

// WRITE_ONCE;

    }
// Record no current task on entering RCU-tasks (dyntick-idle exit).
#[no_mangle]
unsafe extern "C" fn rcu_task_enter() -> __always_inline void {

// WRITE_ONCE;

    }
//
// Record entry into an extended quiescent state.  This is only to be
// called when not already in an extended quiescent state, that is,
// RCU is watching prior to the call to this function and is no longer
// watching upon return.
//
#[no_mangle]
unsafe extern "C" fn ct_kernel_exit_state(offset: c_int) -> noinstr void {
//
// CPUs seeing atomic_add_return() must see prior RCU read-side
// critical sections, and we also must force ordering with the
// next idle sojourn.
//
// RCU is still watching.  Better not be in extended quiescent state!
// WARN_ON_ONCE;
    (void)ct_state_inc(offset);
// RCU is no longer watching.
    }
//
// Record exit from an extended quiescent state.  This is only to be
// called from an extended quiescent state, that is, RCU is not watching
// prior to the call to this function and is watching upon return.
//
#[no_mangle]
unsafe extern "C" fn ct_kernel_enter_state(offset: c_int) -> noinstr void {
    let mut seq = 0;
//
// CPUs seeing atomic_add_return() must see prior idle sojourns,
// and we also must force ordering with the next RCU read-side
// critical section.
//
    seq = ct_state_inc(offset);
// RCU is now watching.  Better not be in an extended quiescent state!
// WARN_ON_ONCE;
    }
//
// Enter an RCU extended quiescent state, which can be either the
// idle loop or adaptive-tickless usermode execution.
//
// We crowbar the ->nmi_nesting field to zero to allow for
// the possibility of usermode upcalls having messed up our count
// of interrupt nesting level during the prior busy period.
//
#[no_mangle]
unsafe extern "C" fn ct_kernel_exit(user: bool, offset: c_int) -> void noinstr {
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
// WARN_ON_ONCE;
// WRITE_ONCE;
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) &&
    ct_nesting() == 0);
    if (ct_nesting() != 1) {
// RCU will still be watching, so just do accounting and leave.
    ct.nesting--;
    return;
    }
    instrumentation_begin();
    lockdep_assert_irqs_disabled();
    trace_rcu_watching(TPS("End"), ct_nesting(), 0, ct_rcu_watching());
// WARN_ON_ONCE;
    rcu_preempt_deferred_qs(current);
// instrumentation for the noinstr ct_kernel_exit_state()
    instrument_atomic_write(&ct.state, sizeof(ct.state));
    instrumentation_end();
// WRITE_ONCE; /* Avoid irq-access tearing. */
// RCU is watching here ...
    ct_kernel_exit_state(offset);
// ... but is no longer watching here.
    rcu_task_exit();
    }
//
// Exit an RCU extended quiescent state, which can be either the
// idle loop or adaptive-tickless usermode execution.
//
// We crowbar the ->nmi_nesting field to CT_NESTING_IRQ_NONIDLE to
// allow for the possibility of usermode upcalls messing up our count of
// interrupt nesting level during the busy period that is just now starting.
//
#[no_mangle]
unsafe extern "C" fn ct_kernel_enter(user: bool, offset: c_int) -> void noinstr {
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
    let mut oldval = 0;
// WARN_ON_ONCE;
    oldval = ct_nesting();
// WARN_ON_ONCE;
    if (oldval) {
// RCU was already watching, so just do accounting and leave.
    ct.nesting++;
    return;
    }
    rcu_task_enter();
// RCU is not watching here ...
    ct_kernel_enter_state(offset);
// ... but is watching here.
    instrumentation_begin();
// instrumentation for the noinstr ct_kernel_enter_state()
    instrument_atomic_write(&ct.state, sizeof(ct.state));
    trace_rcu_watching(TPS("Start"), ct_nesting(), 1, ct_rcu_watching());
// WARN_ON_ONCE;
// WRITE_ONCE;
// WARN_ON_ONCE;
// WRITE_ONCE;
    instrumentation_end();
    }
//
// ct_nmi_exit - inform RCU of exit from NMI context
//
// If we are returning from the outermost NMI handler that interrupted an
// RCU-idle period, update ct->state and ct->nmi_nesting
// to let the RCU grace-period handling know that the CPU is back to
// being RCU-idle.
//
// If you add or remove a call to ct_nmi_exit(), be sure to test
// with CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_nmi_exit() -> void noinstr {
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
    instrumentation_begin();
//
// Check for ->nmi_nesting underflow and bad CT state.
// (We are exiting an NMI handler, so RCU better be paying attention
// to us!)
//
// WARN_ON_ONCE;
// WARN_ON_ONCE;
//
// If the nesting level is not 1, the CPU wasn't RCU-idle, so
// leave it in non-RCU-idle state.
//
    if (ct_nmi_nesting() != 1) {
    trace_rcu_watching(TPS("--="), ct_nmi_nesting(), ct_nmi_nesting() - 2,
    ct_rcu_watching());
    WRITE_ONCE(ct.nmi_nesting, /* No store tearing. */
    ct_nmi_nesting() - 2);
    instrumentation_end();
    return;
    }
// This NMI interrupted an RCU-idle CPU, restore RCU-idleness.
    trace_rcu_watching(TPS("Endirq"), ct_nmi_nesting(), 0, ct_rcu_watching());
// WRITE_ONCE; /* Avoid store tearing. */
// instrumentation for the noinstr ct_kernel_exit_state()
    instrument_atomic_write(&ct.state, sizeof(ct.state));
    instrumentation_end();
// RCU is watching here ...
    ct_kernel_exit_state(CT_RCU_WATCHING);
// ... but is no longer watching here.
    if (!in_nmi()) {
    rcu_task_exit();
    }
    }
//
// ct_nmi_enter - inform RCU of entry to NMI context
//
// If the CPU was idle from RCU's viewpoint, update ct->state and
// ct->nmi_nesting to let the RCU grace-period handling know
// that the CPU is active.  This implementation permits nested NMIs, as
// long as the nesting level does not overflow an int.  (You will probably
// run out of stack space first.)
//
// If you add or remove a call to ct_nmi_enter(), be sure to test
// with CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_nmi_enter() -> void noinstr {
pub static mut incby: c_long = 2;
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
// Complain about underflow.
// WARN_ON_ONCE;
//
// If idle from RCU viewpoint, atomically increment CT state
// to mark non-idle and increment ->nmi_nesting by one.
// Otherwise, increment ->nmi_nesting by two.  This means
// if ->nmi_nesting is equal to one, we are guaranteed
// to be in the outermost NMI handler that interrupted an RCU-idle
// period (observation due to Andy Lutomirski).
//
    if (!rcu_is_watching_curr_cpu()) {
    if (!in_nmi()) {
    rcu_task_enter();
    }
// RCU is not watching here ...
    ct_kernel_enter_state(CT_RCU_WATCHING);
// ... but is watching here.
    instrumentation_begin();
// instrumentation for the noinstr rcu_is_watching_curr_cpu()
    instrument_atomic_read(&ct.state, sizeof(ct.state));
// instrumentation for the noinstr ct_kernel_enter_state()
    instrument_atomic_write(&ct.state, sizeof(ct.state));
    incby = 1;
    } else if (!in_nmi()) {
    instrumentation_begin();
    rcu_irq_enter_check_tick();
    } else  {
    instrumentation_begin();
    }
    trace_rcu_watching(incby == 1 ? TPS("Startirq") : TPS("++="),
    ct_nmi_nesting(),
    ct_nmi_nesting() + incby, ct_rcu_watching());
    instrumentation_end();
    WRITE_ONCE(ct.nmi_nesting, /* Prevent store tearing. */
    ct_nmi_nesting() + incby);
    barrier();
    }
//
// ct_idle_enter - inform RCU that current CPU is entering idle
//
// Enter idle mode, in other words, -leave- the mode in which RCU
// read-side critical sections can occur.  (Though RCU read-side
// critical sections can occur in irq handlers in idle, a possibility
// handled by irq_enter() and irq_exit().)
//
// If you add or remove a call to ct_idle_enter(), be sure to test with
// CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_idle_enter() -> void noinstr {
// WARN_ON_ONCE;
    ct_kernel_exit(false, CT_RCU_WATCHING + CT_STATE_IDLE);
    }
// EXPORT_SYMBOL_GPL;
//
// ct_idle_exit - inform RCU that current CPU is leaving idle
//
// Exit idle mode, in other words, -enter- the mode in which RCU
// read-side critical sections can occur.
//
// If you add or remove a call to ct_idle_exit(), be sure to test with
// CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_idle_exit() -> void noinstr {
    let mut flags = 0;
    raw_local_irq_save(flags);
    ct_kernel_enter(false, CT_RCU_WATCHING - CT_STATE_IDLE);
    raw_local_irq_restore(flags);
    }
// EXPORT_SYMBOL_GPL;
//
// ct_irq_enter - inform RCU that current CPU is entering irq away from idle
//
// Enter an interrupt handler, which might possibly result in exiting
// idle mode, in other words, entering the mode in which read-side critical
// sections can occur.  The caller must have disabled interrupts.
//
// Note that the Linux kernel is fully capable of entering an interrupt
// handler that it never exits, for example when doing upcalls to user mode!
// This code assumes that the idle loop never does upcalls to user mode.
// If your architecture's idle loop does do upcalls to user mode (or does
// anything else that results in unbalanced calls to the irq_enter() and
// irq_exit() functions), RCU will give you what you deserve, good and hard.
// But very infrequently and irreproducibly.
//
// Use things like work queues to work around this limitation.
//
// You have been warned.
//
// If you add or remove a call to ct_irq_enter(), be sure to test with
// CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_irq_enter() -> noinstr void {
    lockdep_assert_irqs_disabled();
    ct_nmi_enter();
    }
//
// ct_irq_exit - inform RCU that current CPU is exiting irq towards idle
//
// Exit from an interrupt handler, which might possibly result in entering
// idle mode, in other words, leaving the mode in which read-side critical
// sections can occur.  The caller must have disabled interrupts.
//
// This code assumes that the idle loop never does anything that might
// result in unbalanced calls to irq_enter() and irq_exit().  If your
// architecture's idle loop violates this assumption, RCU will give you what
// you deserve, good and hard.  But very infrequently and irreproducibly.
//
// Use things like work queues to work around this limitation.
//
// You have been warned.
//
// If you add or remove a call to ct_irq_exit(), be sure to test with
// CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_irq_exit() -> noinstr void {
    lockdep_assert_irqs_disabled();
    ct_nmi_exit();
    }
//
// Wrapper for ct_irq_enter() where interrupts are enabled.
//
// If you add or remove a call to ct_irq_enter_irqson(), be sure to test
// with CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_irq_enter_irqson() {
    let mut flags = 0;
    local_irq_save(flags);
    ct_irq_enter();
    local_irq_restore(flags);
    }
//
// Wrapper for ct_irq_exit() where interrupts are enabled.
//
// If you add or remove a call to ct_irq_exit_irqson(), be sure to test
// with CONFIG_RCU_EQS_DEBUG=y.
//
#[no_mangle]
pub unsafe extern "C" fn ct_irq_exit_irqson() {
    let mut flags = 0;
    local_irq_save(flags);
    ct_irq_exit();
    local_irq_restore(flags);
    }

    static __always_inline void ct_kernel_exit(bool user, int offset) { }
    static __always_inline void ct_kernel_enter(bool user, int offset) { }

// Macro flag: #define CREATE_TRACE_POINTS

// DEFINE_STATIC_KEY_FALSE_RO;
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn context_tracking_recursion_enter() -> noinstr bool {
    let mut recursion = 0;
    recursion = __this_cpu_inc_return(context_tracking.recursion);
    if (recursion == 1) {
    return true;
    }
// WARN_ONCE;
    __this_cpu_dec(context_tracking.recursion);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn context_tracking_recursion_exit() -> __always_inline void {
    __this_cpu_dec(context_tracking.recursion);
    }
//
// __ct_user_enter - Inform the context tracking that the CPU is going
// to enter user or guest space mode.
//
// @state: userspace context-tracking state to enter.
//
// This function must be called right before we switch from the kernel
// to user or guest space, when it's guaranteed the remaining kernel
// instructions to execute won't use any RCU read side critical section
// because this function sets RCU in extended quiescent state.
//
#[no_mangle]
pub unsafe extern "C" fn __ct_user_enter(state: ctx_state) -> void noinstr {
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
    lockdep_assert_irqs_disabled();
// Kernel threads aren't supposed to go to userspace
// WARN_ON_ONCE;
    if (!context_tracking_recursion_enter()) {
    return;
    }
    if (__ct_state() != state) {
    if (ct.active) {
//
// At this stage, only low level arch entry code remains and
// then we'll run in userspace. We can assume there won't be
// any RCU read-side critical section until the next call to
// user_exit() or ct_irq_enter(). Let's remove RCU's dependency
// on the tick.
//
    if (state == CT_STATE_USER) {
    instrumentation_begin();
    trace_user_enter(0);
    vtime_user_enter(current);
    instrumentation_end();
    }
//
// Other than generic entry implementation, we may be past the last
// rescheduling opportunity in the entry code. Trigger a self IPI
// that will fire and reschedule once we resume in user/guest mode.
//
    rcu_irq_work_resched();
//
// Enter RCU idle mode right before resuming userspace.  No use of RCU
// is permitted between this call and rcu_eqs_exit(). This way the
// CPU doesn't need to maintain the tick for RCU maintenance purposes
// when the CPU runs in userspace.
//
    ct_kernel_exit(true, CT_RCU_WATCHING + state);
//
// Special case if we only track user <-> kernel transitions for tickless
// cputime accounting but we don't support RCU extended quiescent state.
// In this we case we don't care about any concurrency/ordering.
//
    if (!IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE)) {
    raw_atomic_set(&ct.state, state);
    }
    } else {
//
// Even if context tracking is disabled on this CPU, because it's outside
// the full dynticks mask for example, we still have to keep track of the
// context transitions and states to prevent inconsistency on those of
// other CPUs.
// If a task triggers an exception in userspace, sleep on the exception
// handler and then migrate to another CPU, that new CPU must know where
// the exception returns by the time we call exception_exit().
// This information can only be provided by the previous CPU when it called
// exception_enter().
// OTOH we can spare the calls to vtime and RCU when context_tracking.active
// is false because we know that CPU is not tickless.
//
    if (!IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE)) {
// Tracking for vtime only, no concurrent RCU EQS accounting
    raw_atomic_set(&ct.state, state);
    } else {
//
// Tracking for vtime and RCU EQS. Make sure we don't race
// with NMIs. OTOH we don't care about ordering here since
// RCU only requires CT_RCU_WATCHING increments to be fully
// ordered.
//
    raw_atomic_add(state, &ct.state);
    }
    }
    }
    context_tracking_recursion_exit();
    }
// EXPORT_SYMBOL_GPL;
//
// OBSOLETE:
// This function should be noinstr but the below local_irq_restore() is
// unsafe because it involves illegal RCU uses through tracing and lockdep.
// This is unlikely to be fixed as this function is obsolete. The preferred
// way is to call __context_tracking_enter() through user_enter_irqoff()
// or context_tracking_guest_enter(). It should be the arch entry code
// responsibility to call into context tracking with IRQs disabled.
//
#[no_mangle]
pub unsafe extern "C" fn ct_user_enter(state: ctx_state) {
    let mut flags = 0;
//
// Some contexts may involve an exception occuring in an irq,
// leading to that nesting:
// ct_irq_enter() rcu_eqs_exit(true) rcu_eqs_enter(true) ct_irq_exit()
// This would mess up the dyntick_nesting count though. And rcu_irq_*()
// helpers are enough to protect RCU uses inside the exception. So
// just return immediately if we detect we are in an IRQ.
//
    if (in_interrupt()) {
    return;
    }
    local_irq_save(flags);
    __ct_user_enter(state);
    local_irq_restore(flags);
    }
// NOKPROBE_SYMBOL;
// EXPORT_SYMBOL_GPL;
//
// user_enter_callable() - Unfortunate ASM callable version of user_enter() for
// archs that didn't manage to check the context tracking
// static key from low level code.
//
// This OBSOLETE function should be noinstr but it unsafely calls
// local_irq_restore(), involving illegal RCU uses through tracing and lockdep.
// This is unlikely to be fixed as this function is obsolete. The preferred
// way is to call user_enter_irqoff(). It should be the arch entry code
// responsibility to call into context tracking with IRQs disabled.
//
#[no_mangle]
pub unsafe extern "C" fn user_enter_callable() {
    user_enter();
    }
// NOKPROBE_SYMBOL;
//
// __ct_user_exit - Inform the context tracking that the CPU is
// exiting user or guest mode and entering the kernel.
//
// @state: userspace context-tracking state being exited from.
//
// This function must be called after we entered the kernel from user or
// guest space before any use of RCU read side critical section. This
// potentially include any high level kernel code like syscalls, exceptions,
// signal handling, etc...
//
// This call supports re-entrancy. This way it can be called from any exception
// handler without needing to know if we came from userspace or not.
//
#[no_mangle]
pub unsafe extern "C" fn __ct_user_exit(state: ctx_state) -> void noinstr {
    struct context_tracking *ct = this_cpu_ptr(&context_tracking);
    if (!context_tracking_recursion_enter()) {
    return;
    }
    if (__ct_state() == state) {
    if (ct.active) {
//
// Exit RCU idle mode while entering the kernel because it can
// run a RCU read side critical section anytime.
//
    ct_kernel_enter(true, CT_RCU_WATCHING - state);
    if (state == CT_STATE_USER) {
    instrumentation_begin();
    vtime_user_exit(current);
    trace_user_exit(0);
    instrumentation_end();
    }
//
// Special case if we only track user <-> kernel transitions for tickless
// cputime accounting but we don't support RCU extended quiescent state.
// In this we case we don't care about any concurrency/ordering.
//
    if (!IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE)) {
    raw_atomic_set(&ct.state, CT_STATE_KERNEL);
    }
    } else {
    if (!IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE)) {
// Tracking for vtime only, no concurrent RCU EQS accounting
    raw_atomic_set(&ct.state, CT_STATE_KERNEL);
    } else {
//
// Tracking for vtime and RCU EQS. Make sure we don't race
// with NMIs. OTOH we don't care about ordering here since
// RCU only requires CT_RCU_WATCHING increments to be fully
// ordered.
//
    raw_atomic_sub(state, &ct.state);
    }
    }
    }
    context_tracking_recursion_exit();
    }
// EXPORT_SYMBOL_GPL;
//
// OBSOLETE:
// This function should be noinstr but the below local_irq_save() is
// unsafe because it involves illegal RCU uses through tracing and lockdep.
// This is unlikely to be fixed as this function is obsolete. The preferred
// way is to call __context_tracking_exit() through user_exit_irqoff()
// or context_tracking_guest_exit(). It should be the arch entry code
// responsibility to call into context tracking with IRQs disabled.
//
#[no_mangle]
pub unsafe extern "C" fn ct_user_exit(state: ctx_state) {
    let mut flags = 0;
    if (in_interrupt()) {
    return;
    }
    local_irq_save(flags);
    __ct_user_exit(state);
    local_irq_restore(flags);
    }
// NOKPROBE_SYMBOL;
// EXPORT_SYMBOL_GPL;
//
// user_exit_callable() - Unfortunate ASM callable version of user_exit() for
// archs that didn't manage to check the context tracking
// static key from low level code.
//
// This OBSOLETE function should be noinstr but it unsafely calls local_irq_save(),
// involving illegal RCU uses through tracing and lockdep. This is unlikely
// to be fixed as this function is obsolete. The preferred way is to call
// user_exit_irqoff(). It should be the arch entry code responsibility to
// call into context tracking with IRQs disabled.
//
#[no_mangle]
pub unsafe extern "C" fn user_exit_callable() {
    user_exit();
    }
// NOKPROBE_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn ct_cpu_track_user(cpu: c_int) -> c_int {
pub static mut initialized: __initdata bool = false;
    if (!per_cpu(context_tracking.active, cpu)) {
    per_cpu(context_tracking.active, cpu) = true;
    static_branch_inc(&context_tracking_key);
    }
    if (initialized) {
    return;
    }

//
// Set TIF_NOHZ to init/0 and let it propagate to all tasks through fork
// This assumes that init is the only task at this early boot stage.
//
    set_tsk_thread_flag(&init_task, TIF_NOHZ);

// WARN_ON_ONCE;
    initialized = true;
    }

#[no_mangle]
pub unsafe extern "C" fn context_tracking_init() -> c_int {
    let mut cpu = 0;
    for_each_possible_cpu(cpu)
    ct_cpu_track_user(cpu);
    }