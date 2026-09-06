//! Automatically rewritten from C to Rust
//! Source: kernel/time/tick-common.c
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
// This file contains the base functions to manage periodic tick
// related events.
//
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
//

//
// Tick devices
//
pub static mut struct tick_device: usize = 0;
//
// Tick next event: keeps track of the tick time. It's updated by the
// CPU which handles the tick and protected by jiffies_lock. There is
// no requirement to write hold the jiffies seqcount for it.
//
    let mut tick_next_period;
//
// tick_do_timer_cpu is a timer core internal variable which holds the CPU NR
// which is responsible for calling do_timer(), i.e. the timekeeping stuff. This
// variable has two functions:
//
// 1) Prevent a thundering herd issue of a gazillion of CPUs trying to grab the
// timekeeping lock all at once. Only the CPU which is assigned to do the
// update is handling it.
//
// 2) Hand off the duty in the NOHZ idle case by setting the value to
// TICK_DO_TIMER_NONE, i.e. a non existing CPU. So the next cpu which looks
// at it will take over and keep the time keeping alive.  The handover
// procedure also covers cpu hotplug.
//
pub static mut : int tick_do_timer_cpu = 0;

//
// tick_do_timer_boot_cpu indicates the boot CPU temporarily owns
// tick_do_timer_cpu and it should be taken over by an eligible secondary
// when one comes online.
//
pub static mut : int tick_do_timer_boot_cpu = 0;

//
// Debugging: see timer_list.c
//
#[no_mangle]
pub unsafe extern "C" fn tick_get_device(cpu: c_int) -> *mut c_void {
    return &per_cpu(tick_cpu_device, cpu);
    }
//
// tick_is_oneshot_available - check for a oneshot capable event device
//
#[no_mangle]
pub unsafe extern "C" fn tick_is_oneshot_available() -> c_int {
    let mut dev = __this_cpu_read(tick_cpu_device.evtdev);
    if (!dev || !(dev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return 0;
    }
    if (!(dev.features & CLOCK_EVT_FEAT_C3STOP)) {
    return 1;
    }
    return tick_broadcast_oneshot_available();
    }
//
// Periodic tick
//
#[no_mangle]
unsafe extern "C" fn tick_periodic(cpu: c_int) {
    if (READ_ONCE(tick_do_timer_cpu) == cpu) {
    raw_spin_lock(&jiffies_lock);
    write_seqcount_begin(&jiffies_seq);
// Keep track of the next tick event
    tick_next_period = ktime_add_ns(tick_next_period, TICK_NSEC);
    do_timer(1);
    write_seqcount_end(&jiffies_seq);
    raw_spin_unlock(&jiffies_lock);
    update_wall_time();
    }
    update_process_times(user_mode(get_irq_regs()));
    profile_tick(CPU_PROFILING);
    }
//
// Event handler for periodic ticks
//
#[no_mangle]
pub unsafe extern "C" fn tick_handle_periodic(dev: *mut clock_event_device) {
pub static mut cpu: c_int = 0;
pub static mut next: ktime_t = 0;
    dev.next_event_forced = 0;
    tick_periodic(cpu);
//
// The cpu might have transitioned to HIGHRES or NOHZ mode via
// update_process_times() -> run_local_timers() ->
// hrtimer_run_queues().
//
    if (IS_ENABLED!(CONFIG_TICK_ONESHOT) && dev.event_handler != tick_handle_periodic) {
    return;
    }
    if (!clockevent_state_oneshot(dev)) {
    return;
    }
    for (;;) {
//
// Setup the next period for devices, which do not have
// periodic mode:
//
    next = ktime_add_ns(next, TICK_NSEC);
    if (!clockevents_program_event(dev, next, false)) {
    return;
    }
//
// Have to be careful here. If we're in oneshot mode,
// before we call tick_periodic() in a loop, we need
// to be sure we're using a real hardware clocksource.
// Otherwise we could get trapped in an infinite
// loop, as the tick_periodic() increments jiffies,
// which then will increment time, possibly causing
// the loop to trigger again and again.
//
    if (timekeeping_valid_for_hres()) {
    tick_periodic(cpu);
    }
    }
    }
//
// Setup the device for a periodic tick
//
#[no_mangle]
pub unsafe extern "C" fn tick_setup_periodic(dev: *mut clock_event_device, broadcast: c_int) {
    tick_set_periodic_handler(dev, broadcast);
// Broadcast setup ?
    if (!tick_device_is_functional(dev)) {
    return;
    }
    if ((dev.features & CLOCK_EVT_FEAT_PERIODIC) &&
    !tick_broadcast_oneshot_active()) {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_PERIODIC);
    } else {
    let mut seq = 0;
    let mut next;
    do {
    seq = read_seqcount_begin(&jiffies_seq);
    next = tick_next_period;
    } while (read_seqcount_retry(&jiffies_seq, seq));
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    for (;;) {
    if (!clockevents_program_event(dev, next, false)) {
    return;
    }
    next = ktime_add_ns(next, TICK_NSEC);
    }
    }
    }
//
// Setup the tick device
//
#[no_mangle]
pub unsafe extern "C" fn tick_setup_device(td: *mut tick_device, newdev: *mut clock_event_device, cpu: c_int, cpumask: *mut cpumask) {
    void (*handler) = core::ptr::null_mut();
pub static mut next_event: ktime_t = 0;
//
// First device setup ?
//
    if (!td.evtdev) {
//
// If no cpu took the do_timer update, assign it to
// this cpu:
//
    if (READ_ONCE(tick_do_timer_cpu) == TICK_DO_TIMER_BOOT) {
    WRITE_ONCE(tick_do_timer_cpu, cpu);
    tick_next_period = ktime_get();

//
// The boot CPU may be nohz_full, in which case the
// first housekeeping secondary will take do_timer()
// from it.
//
    if (tick_nohz_full_cpu(cpu)) {
    tick_do_timer_boot_cpu = cpu;
    }
    } else if (tick_do_timer_boot_cpu != -1 && !tick_nohz_full_cpu(cpu)) {
    tick_do_timer_boot_cpu = -1;
//
// The boot CPU will stay in periodic (NOHZ disabled)
// mode until clocksource_done_booting() called after
// smp_init() selects a high resolution clocksource and
// timekeeping_notify() kicks the NOHZ stuff alive.
//
// So this WRITE_ONCE can only race with the READ_ONCE
// check in tick_periodic() but this race is harmless.
//
    WRITE_ONCE(tick_do_timer_cpu, cpu);

    }
//
// Startup in periodic mode first.
//
    td.mode = TICKDEV_MODE_PERIODIC;
    } else {
    handler = td.evtdev.event_handler;
    next_event = td.evtdev.next_event;
    td.evtdev.event_handler = clockevents_handle_noop;
    }
    td.evtdev = newdev;
//
// When the device is not per cpu, pin the interrupt to the
// current cpu:
//
    if (!cpumask_equal(newdev.cpumask, cpumask)) {
    irq_set_affinity(newdev.irq, cpumask);
    }
//
// When global broadcasting is active, check if the current
// device is registered as a placeholder for broadcast mode.
// This allows us to handle this x86 misfeature in a generic
// way. This function also returns !=0 when we keep the
// current active broadcast state for this CPU.
//
    if (tick_device_uses_broadcast(newdev, cpu)) {
    return;
    }
    if (td.mode == TICKDEV_MODE_PERIODIC) {
    tick_setup_periodic(newdev, 0);
    }
    else {
    tick_setup_oneshot(newdev, handler, next_event);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tick_install_replacement(newdev: *mut clock_event_device) {
    let mut td = this_cpu_ptr(&tick_cpu_device);
pub static mut cpu: c_int = 0;
    clockevents_exchange_device(td.evtdev, newdev);
    tick_setup_device(td, newdev, cpu, cpumask_of(cpu));
    if (newdev.features & CLOCK_EVT_FEAT_ONESHOT) {
    tick_oneshot_notify();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tick_check_percpu(curdev: *mut clock_event_device, newdev: *mut clock_event_device, cpu: c_int) -> bool {
    if (!cpumask_test_cpu(cpu, newdev.cpumask)) {
    return false;
    }
    if (cpumask_equal(newdev.cpumask, cpumask_of(cpu))) {
    return true;
    }
// Check if irq affinity can be set
    if (newdev.irq >= 0 && !irq_can_set_affinity(newdev.irq)) {
    return false;
    }
// Prefer an existing cpu local device
    if (curdev && cpumask_equal(curdev.cpumask, cpumask_of(cpu))) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn tick_check_preferred(curdev: *mut clock_event_device, newdev: *mut clock_event_device) -> bool {
// Prefer oneshot capable device
    if (!(newdev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    if (curdev && (curdev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return false;
    }
    if (tick_oneshot_mode_active()) {
    return false;
    }
    }
//
// Use the higher rated one, but prefer a CPU local device with a lower
// rating than a non-CPU local device
//
    return !curdev ||
    newdev.rating > curdev.rating ||
    !cpumask_equal(curdev.cpumask, newdev.cpumask);
    }
//
// Check whether the new device is a better fit than curdev. curdev
// can be NULL !
//
#[no_mangle]
pub unsafe extern "C" fn tick_check_replacement(curdev: *mut clock_event_device, newdev: *mut clock_event_device) -> bool {
    if (!tick_check_percpu(curdev, newdev, smp_processor_id())) {
    return false;
    }
    return tick_check_preferred(curdev, newdev);
    }
//
// Check, if the new registered device should be used. Called with
// clockevents_lock held and interrupts disabled.
//
#[no_mangle]
pub unsafe extern "C" fn tick_check_new_device(newdev: *mut clock_event_device) {
pub static mut curdev: *mut c_void = core::ptr::null_mut();
pub static mut td: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    cpu = smp_processor_id();
    td = &per_cpu(tick_cpu_device, cpu);
    curdev = td.evtdev;
    if (!tick_check_replacement(curdev, newdev)) {
// goto;
    }
    if (!try_module_get(newdev.owner)) {
    return;
    }
//
// Replace the eventually existing device by the new
// device. If the current device is the broadcast device, do
// not give it back to the clockevents layer !
//
    if (tick_is_broadcast_device(curdev)) {
    clockevents_shutdown(curdev);
    curdev = core::ptr::null_mut();
    }
    clockevents_exchange_device(curdev, newdev);
    tick_setup_device(td, newdev, cpu, cpumask_of(cpu));
    if (newdev.features & CLOCK_EVT_FEAT_ONESHOT) {
    tick_oneshot_notify();
    }
    return;
// label;
//
// Can the new device be used as a broadcast device ?
//
    tick_install_broadcast_device(newdev, cpu);
    }
//
// tick_broadcast_oneshot_control - Enter/exit broadcast oneshot mode
// @state:	The target state (enter/exit)
//
// The system enters/leaves a state, where affected devices might stop
// Returns 0 on success, -EBUSY if the cpu is used to broadcast wakeups.
//
// Called with interrupts disabled, so clockevents_lock is not
// required here because the local clock event device cannot go away
// under us.
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_oneshot_control(state: tick_broadcast_state) -> c_int {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    if (!(td.evtdev.features & CLOCK_EVT_FEAT_C3STOP)) {
    return 0;
    }
    return __tick_broadcast_oneshot_control(state);
    }
    EXPORT_SYMBOL_GPL(tick_broadcast_oneshot_control);

#[no_mangle]
pub unsafe extern "C" fn tick_assert_timekeeping_handover() {
    WARN_ON_ONCE!(tick_do_timer_cpu == smp_processor_id());
    }
//
// Stop the tick and transfer the timekeeping job away from a dying cpu.
//
#[no_mangle]
pub unsafe extern "C" fn tick_cpu_dying(dying_cpu: c_uint) -> c_int {
//
// If the current CPU is the timekeeper, it's the only one that can
// safely hand over its duty. Also all online CPUs are in stop
// machine, guaranteed not to be idle, therefore there is no
// concurrency and it's safe to pick any online successor.
//
    if (tick_do_timer_cpu == dying_cpu) {
    tick_do_timer_cpu = cpumask_first(cpu_online_mask);
    }
// Make sure the CPU won't try to retake the timekeeping duty
    tick_sched_timer_dying(dying_cpu);
// Remove CPU from timer broadcasting
    tick_offline_cpu(dying_cpu);
    return 0;
    }
//
// Shutdown an event device on the outgoing CPU:
//
// Called by the dying CPU during teardown, with clockevents_lock held
// and interrupts disabled.
//
#[no_mangle]
pub unsafe extern "C" fn tick_shutdown() {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    let mut dev = td.evtdev;
    td.mode = TICKDEV_MODE_PERIODIC;
    if (dev) {
    clockevents_exchange_device(dev, core::ptr::null_mut());
    dev.event_handler = clockevents_handle_noop;
    td.evtdev = core::ptr::null_mut();
    }
    }

//
// tick_suspend_local - Suspend the local tick device
//
// Called from the local cpu for freeze with interrupts disabled.
//
// No locks required. Nothing can change the per cpu device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_suspend_local() {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    clockevents_shutdown(td.evtdev);
    }
//
// tick_resume_local - Resume the local tick device
//
// Called from the local CPU for unfreeze or XEN resume magic.
//
// No locks required. Nothing can change the per cpu device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_resume_local() {
    let mut td = this_cpu_ptr(&tick_cpu_device);
pub static mut broadcast: bool = false;
    clockevents_tick_resume(td.evtdev);
    if (!broadcast) {
    if (td.mode == TICKDEV_MODE_PERIODIC) {
    tick_setup_periodic(td.evtdev, 0);
    }
    else {
    tick_resume_oneshot();
    }
    }
//
// Ensure that hrtimers are up to date and the clockevents device
// is reprogrammed correctly when high resolution timers are
// enabled.
//
    hrtimers_resume_local();
    }
//
// tick_suspend - Suspend the tick and the broadcast device
//
// Called from syscore_suspend() via timekeeping_suspend with only one
// CPU online and interrupts disabled or from tick_unfreeze() under
// tick_freeze_lock.
//
// No locks required. Nothing can change the per cpu device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_suspend() {
    tick_suspend_local();
    tick_suspend_broadcast();
    }
//
// tick_resume - Resume the tick and the broadcast device
//
// Called from syscore_resume() via timekeeping_resume with only one
// CPU online and interrupts disabled.
//
// No locks required. Nothing can change the per cpu device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_resume() {
    tick_resume_broadcast();
    tick_resume_local();
    }

pub static mut tick_freeze_lock: usize = 0;
pub static mut tick_freeze_map: usize = 0;
    static unsigned int tick_freeze_depth;
//
// tick_freeze - Suspend the local tick and (possibly) timekeeping.
//
// Check if this is the last online CPU executing the function and if so,
// suspend timekeeping.  Otherwise suspend the local tick.
//
// Call with interrupts disabled.  Must be balanced with %tick_unfreeze().
// Interrupts must not be enabled before the subsequent %tick_unfreeze().
//
#[no_mangle]
pub unsafe extern "C" fn tick_freeze() {
    raw_spin_lock(&tick_freeze_lock);
    tick_freeze_depth += 1;
    if (tick_freeze_depth == num_online_cpus()) {
    trace_suspend_resume(TPS("timekeeping_freeze"),
    smp_processor_id(), true);
//
// All other CPUs have their interrupts disabled and are
// suspended to idle. Other tasks have been frozen so there
// is no scheduling happening. This means that there is no
// concurrency in the system at this point. Therefore it is
// okay to acquire a sleeping lock on PREEMPT_RT, such as a
// spinlock, because the lock cannot be held by other CPUs
// or threads and acquiring it cannot block.
//
// Inform lockdep about the situation.
//
    lock_map_acquire_try(&tick_freeze_map);
    system_state = SYSTEM_SUSPEND;
    sched_clock_suspend();
    timekeeping_suspend();
    lock_map_release(&tick_freeze_map);
    } else {
    tick_suspend_local();
    }
    raw_spin_unlock(&tick_freeze_lock);
    }
//
// tick_unfreeze - Resume the local tick and (possibly) timekeeping.
//
// Check if this is the first CPU executing the function and if so, resume
// timekeeping.  Otherwise resume the local tick.
//
// Call with interrupts disabled.  Must be balanced with %tick_freeze().
// Interrupts must not be enabled after the preceding %tick_freeze().
//
#[no_mangle]
pub unsafe extern "C" fn tick_unfreeze() {
    raw_spin_lock(&tick_freeze_lock);
    if (tick_freeze_depth == num_online_cpus()) {
//
// Similar to tick_freeze(). On resumption the first CPU may
// acquire uncontended sleeping locks while other CPUs block on
// tick_freeze_lock.
//
    lock_map_acquire_try(&tick_freeze_map);
    timekeeping_resume();
    sched_clock_resume();
    lock_map_release(&tick_freeze_map);
    system_state = SYSTEM_RUNNING;
    trace_suspend_resume(TPS("timekeeping_freeze"),
    smp_processor_id(), false);
    } else {
    touch_softlockup_watchdog();
    tick_resume_local();
    }
    tick_freeze_depth -= 1;
    raw_spin_unlock(&tick_freeze_lock);
    }

//
// tick_init - initialize the tick control
//
#[no_mangle]
pub unsafe extern "C" fn tick_init()  {
    tick_broadcast_init();
    tick_nohz_init();
    }