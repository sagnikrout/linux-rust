//! Automatically rewritten from C to Rust
//! Source: kernel/time/tick-broadcast.c
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
// This file contains functions which emulate a local clock-event
// device via a broadcast event source.
//
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
//

//
// Broadcast support for broken x86 hardware, where the local apic
// timer stops in C3 state.
//
pub static mut tick_broadcast_device: usize = 0;
    static cpumask_var_t tick_broadcast_mask __cpumask_var_read_mostly;
    static cpumask_var_t tick_broadcast_on __cpumask_var_read_mostly;
    static cpumask_var_t tmpmask __cpumask_var_read_mostly;
    static int tick_broadcast_forced;
    static __cacheline_aligned_in_smp DEFINE_RAW_SPINLOCK(tick_broadcast_lock);

pub static mut struct clock_event_device *: usize = 0;
// forward_decl: tick_broadcast_setup_oneshot;
// forward_decl: tick_broadcast_clear_oneshot;
// forward_decl: tick_resume_broadcast_oneshot;

// forward_decl: tick_broadcast_oneshot_offline;

#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_setup_oneshot(bc: *mut clock_event_device, from_periodic: bool) { BUG(); }
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_clear_oneshot(cpu: c_int) { }
#[no_mangle]
pub unsafe extern "C" fn tick_resume_broadcast_oneshot(bc: *mut clock_event_device) { }

#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_oneshot_offline(cpu: c_uint) { }

//
// Debugging: see timer_list.c
//
#[no_mangle]
pub unsafe extern "C" fn tick_get_broadcast_device() -> *mut c_void {
    return &tick_broadcast_device;
    }
#[no_mangle]
pub unsafe extern "C" fn tick_get_broadcast_mask() -> *mut c_void {
    return tick_broadcast_mask;
    }
// forward_decl: tick_get_oneshot_wakeup_device;
    const struct clock_event_device *tick_get_wakeup_device(int cpu)
    {
    return tick_get_oneshot_wakeup_device(cpu);
    }
//
// Start the device in periodic mode
//
#[no_mangle]
unsafe extern "C" fn tick_broadcast_start_periodic(bc: *mut clock_event_device) {
    if (bc) {
    bc.next_event_forced = 0;
    tick_setup_periodic(bc, 1);
    }
    }
//
// Check, if the device can be utilized as broadcast device:
//
#[no_mangle]
pub unsafe extern "C" fn tick_check_broadcast_device(curdev: *mut clock_event_device, newdev: *mut clock_event_device) -> bool {
    if ((newdev.features & CLOCK_EVT_FEAT_DUMMY) ||
    (newdev.features & CLOCK_EVT_FEAT_PERCPU) ||
    (newdev.features & CLOCK_EVT_FEAT_C3STOP)) {
    return false;
    }
    if (tick_broadcast_device.mode == TICKDEV_MODE_ONESHOT &&
    !(newdev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return false;
    }
    return !curdev || newdev.rating > curdev.rating;
    }

#[no_mangle]
pub unsafe extern "C" fn tick_get_oneshot_wakeup_device(cpu: c_int) -> *mut c_void {
    return per_cpu(tick_oneshot_wakeup_device, cpu);
    }
#[no_mangle]
unsafe extern "C" fn tick_oneshot_wakeup_handler(wd: *mut clock_event_device) {
    wd.next_event_forced = 0;
//
// If we woke up early and the tick was reprogrammed in the
// meantime then this may be spurious but harmless.
//
    tick_receive_broadcast();
    }
#[no_mangle]
pub unsafe extern "C" fn tick_set_oneshot_wakeup_device(newdev: *mut clock_event_device, cpu: c_int) -> bool {
    let mut curdev = tick_get_oneshot_wakeup_device(cpu);
    if (!newdev) {
// goto;
    }
    if ((newdev.features & CLOCK_EVT_FEAT_DUMMY) ||
    (newdev.features & CLOCK_EVT_FEAT_C3STOP)) {
    return false;
    }
    if (!(newdev.features & CLOCK_EVT_FEAT_PERCPU) ||
    !(newdev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return false;
    }
    if (!cpumask_equal(newdev.cpumask, cpumask_of(cpu))) {
    return false;
    }
    if (curdev && newdev.rating <= curdev.rating) {
    return false;
    }
    if (!try_module_get(newdev.owner)) {
    return false;
    }
    newdev.event_handler = tick_oneshot_wakeup_handler;
// label;
    clockevents_exchange_device(curdev, newdev);
    per_cpu(tick_oneshot_wakeup_device, cpu) = newdev;
    return true;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: tick_get_oneshot_wakeup_device
pub unsafe extern "C" fn tick_get_oneshot_wakeup_device_dup(cpu: c_int) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: tick_set_oneshot_wakeup_device
pub unsafe extern "C" fn tick_set_oneshot_wakeup_device_dup(newdev: *mut clock_event_device, cpu: c_int) -> bool {
    return false;
    }

//
// Conditionally install/replace broadcast device
//
#[no_mangle]
pub unsafe extern "C" fn tick_install_broadcast_device(dev: *mut clock_event_device, cpu: c_int) {
    let mut cur = tick_broadcast_device.evtdev;
    if (tick_set_oneshot_wakeup_device(dev, cpu)) {
    return;
    }
    if (!tick_check_broadcast_device(cur, dev)) {
    return;
    }
    if (!try_module_get(dev.owner)) {
    return;
    }
    clockevents_exchange_device(cur, dev);
    if (cur) {
    cur.event_handler = clockevents_handle_noop;
    }
    tick_broadcast_device.evtdev = dev;
    if (!cpumask_empty(tick_broadcast_mask)) {
    tick_broadcast_start_periodic(dev);
    }
    if (!(dev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return;
    }
//
// If the system already runs in oneshot mode, switch the newly
// registered broadcast device to oneshot mode explicitly.
//
    if (tick_broadcast_oneshot_active()) {
    tick_broadcast_switch_to_oneshot();
    return;
    }
//
// Inform all cpus about this. We might be in a situation
// where we did not switch to oneshot mode because the per cpu
// devices are affected by CLOCK_EVT_FEAT_C3STOP and the lack
// of a oneshot capable broadcast device. Without that
// notification the systems stays stuck in periodic mode
// forever.
//
    tick_clock_notify();
    }
//
// Check, if the device is the broadcast device
//
#[no_mangle]
pub unsafe extern "C" fn tick_is_broadcast_device(dev: *mut clock_event_device) -> c_int {
    return (dev && tick_broadcast_device.evtdev == dev);
    }
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int {
pub static mut ret: c_int = 0;
    if (tick_is_broadcast_device(dev)) {
    raw_spin_lock(&tick_broadcast_lock);
    ret = __clockevents_update_freq(dev, freq);
    raw_spin_unlock(&tick_broadcast_lock);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn err_broadcast(mask: *const cpumask) {
    pr_crit_once("Failed to broadcast timer tick. Some CPUs may be unresponsive.\n");
    }
#[no_mangle]
unsafe extern "C" fn tick_device_setup_broadcast_func(dev: *mut clock_event_device) {
    if (!dev.broadcast) {
    dev.broadcast = tick_broadcast;
    }
    if (!dev.broadcast) {
    pr_warn_once("%s depends on broadcast, but no broadcast function available\n",
    dev.name);
    dev.broadcast = err_broadcast;
    }
    }
//
// Check, if the device is dysfunctional and a placeholder, which
// needs to be handled by the broadcast device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_device_uses_broadcast(dev: *mut clock_event_device, cpu: c_int) -> c_int {
    let mut bc = tick_broadcast_device.evtdev;
    let mut flags = 0;
pub static mut ret: c_int = 0;
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
//
// Devices might be registered with both periodic and oneshot
// mode disabled. This signals, that the device needs to be
// operated from the broadcast device and is a placeholder for
// the cpu local device.
//
    if (!tick_device_is_functional(dev)) {
    dev.event_handler = tick_handle_periodic;
    tick_device_setup_broadcast_func(dev);
    cpumask_set_cpu(cpu, tick_broadcast_mask);
    if (tick_broadcast_device.mode == TICKDEV_MODE_PERIODIC) {
    tick_broadcast_start_periodic(bc);
    }
    else {
    tick_broadcast_setup_oneshot(bc, false);
    }
    ret = 1;
    } else {
//
// Clear the broadcast bit for this cpu if the
// device is not power state affected.
//
    if (!(dev.features & CLOCK_EVT_FEAT_C3STOP)) {
    cpumask_clear_cpu(cpu, tick_broadcast_mask);
    }
    else {
    tick_device_setup_broadcast_func(dev);
    }
//
// Clear the broadcast bit if the CPU is not in
// periodic broadcast on state.
//
    if (!cpumask_test_cpu(cpu, tick_broadcast_on)) {
    cpumask_clear_cpu(cpu, tick_broadcast_mask);
    }
    match (tick_broadcast_device.mode) {
    TICKDEV_MODE_ONESHOT => {
//
// If the system is in oneshot mode we can
// unconditionally clear the oneshot mask bit,
// because the CPU is running and therefore
// not in an idle state which causes the power
// state affected device to stop. Let the
// caller initialize the device.
//
    tick_broadcast_clear_oneshot(cpu);
    ret = 0;
    // break;
    }
    TICKDEV_MODE_PERIODIC => {
//
// If the system is in periodic mode, check
// whether the broadcast device can be
// switched off now.
//
    if (cpumask_empty(tick_broadcast_mask) && bc) {
    clockevents_shutdown(bc);
    }
//
// If we kept the cpu in the broadcast mask,
// tell the caller to leave the per cpu device
// in shutdown state. The periodic interrupt
// is delivered by the broadcast device, if
// the broadcast device exists and is not
// hrtimer based.
//
    if (bc && !(bc.features & CLOCK_EVT_FEAT_HRTIMER)) {
    ret = cpumask_test_cpu(cpu, tick_broadcast_mask);
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    }
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tick_receive_broadcast() -> c_int {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    let mut evt = td.evtdev;
    if (!evt) {
    return -ENODEV;
    }
    if (!evt.event_handler) {
    return -EINVAL;
    }
    evt.event_handler(evt);
    return 0;
    }
//
// Broadcast the event to the cpus, which are set in the mask (mangled).
//
#[no_mangle]
unsafe extern "C" fn tick_do_broadcast(mask: *mut cpumask) -> bool {
pub static mut cpu: c_int = 0;
pub static mut td: *mut c_void = core::ptr::null_mut();
pub static mut local: bool = false;
//
// Check, if the current cpu is in the mask
//
    if (cpumask_test_cpu(cpu, mask)) {
    let mut bc = tick_broadcast_device.evtdev;
    cpumask_clear_cpu(cpu, mask);
//
// We only run the local handler, if the broadcast
// device is not hrtimer based. Otherwise we run into
// a hrtimer recursion.
//
// local timer_interrupt()
// local_handler()
// expire_hrtimers()
// bc_handler()
// local_handler()
// expire_hrtimers()
//
    local = !(bc.features & CLOCK_EVT_FEAT_HRTIMER);
    }
    if (!cpumask_empty(mask)) {
//
// It might be necessary to actually check whether the devices
// have different broadcast functions. For now, just use the
// one of the first device. This works as long as we have this
// misfeature only on x86 (lapic)
//
    td = &per_cpu(tick_cpu_device, cpumask_first(mask));
    td.evtdev.broadcast(mask);
    }
    return local;
    }
//
// Periodic broadcast:
// - invoke the broadcast handlers
//
#[no_mangle]
unsafe extern "C" fn tick_do_periodic_broadcast() -> bool {
    cpumask_and(tmpmask, cpu_online_mask, tick_broadcast_mask);
    return tick_do_broadcast(tmpmask);
    }
//
// Event handler for periodic broadcast ticks
//
#[no_mangle]
unsafe extern "C" fn tick_handle_periodic_broadcast(dev: *mut clock_event_device) {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    let mut bc_local = 0;
    raw_spin_lock(&tick_broadcast_lock);
    tick_broadcast_device.evtdev.next_event_forced = 0;
// Handle spurious interrupts gracefully
    if (clockevent_state_shutdown(tick_broadcast_device.evtdev)) {
    raw_spin_unlock(&tick_broadcast_lock);
    return;
    }
    bc_local = tick_do_periodic_broadcast();
    if (clockevent_state_oneshot(dev)) {
pub static mut next: ktime_t = 0;
    clockevents_program_event(dev, next, true);
    }
    raw_spin_unlock(&tick_broadcast_lock);
//
// We run the handler of the local cpu after dropping
// tick_broadcast_lock because the handler might deadlock when
// trying to switch to oneshot mode.
//
    if (bc_local) {
    td.evtdev.event_handler(td.evtdev);
    }
    }
//
// tick_broadcast_control - Enable/disable or force broadcast mode
// @mode:	The selected broadcast mode
//
// Called when the system enters a state where affected tick devices
// might stop. Note: TICK_BROADCAST_FORCE cannot be undone.
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_control(mode: tick_broadcast_mode) {
    let mut bc = core::ptr::null_mut();
    let mut dev = core::ptr::null_mut();
pub static mut td: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    let mut bc_stopped = 0;
    let mut flags = 0;
// Protects also the local clockevent device.
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
    td = this_cpu_ptr(&tick_cpu_device);
    dev = td.evtdev;
//
// Is the device not affected by the powerstate ?
//
    if (!dev || !(dev.features & CLOCK_EVT_FEAT_C3STOP)) {
// goto;
    }
    if (!tick_device_is_functional(dev)) {
// goto;
    }
    cpu = smp_processor_id();
    bc = tick_broadcast_device.evtdev;
    bc_stopped = cpumask_empty(tick_broadcast_mask);
    match (mode) {
    TICK_BROADCAST_FORCE => {
    tick_broadcast_forced = 1;
    fallthrough;
    }
    TICK_BROADCAST_ON => {
    cpumask_set_cpu(cpu, tick_broadcast_on);
    if (!cpumask_test_and_set_cpu(cpu, tick_broadcast_mask)) {
//
// Only shutdown the cpu local device, if:
//
// - the broadcast device exists
// - the broadcast device is not a hrtimer based one
// - the broadcast device is in periodic mode to
// avoid a hiccup during switch to oneshot mode
//
    if (bc && !(bc.features & CLOCK_EVT_FEAT_HRTIMER) &&
    tick_broadcast_device.mode == TICKDEV_MODE_PERIODIC) {
    clockevents_shutdown(dev);
    }
    }
    // break;
    }
    TICK_BROADCAST_OFF => {
    if (tick_broadcast_forced) {
    // break;
    }
    cpumask_clear_cpu(cpu, tick_broadcast_on);
    if (cpumask_test_and_clear_cpu(cpu, tick_broadcast_mask)) {
    if (tick_broadcast_device.mode ==
    TICKDEV_MODE_PERIODIC) {
    tick_setup_periodic(dev, 0);
    }
    }
    // break;
    }
    }
    if (bc) {
    if (cpumask_empty(tick_broadcast_mask)) {
    if (!bc_stopped) {
    clockevents_shutdown(bc);
    }
    } else if (bc_stopped) {
    if (tick_broadcast_device.mode == TICKDEV_MODE_PERIODIC) {
    tick_broadcast_start_periodic(bc);
    }
    else {
    tick_broadcast_setup_oneshot(bc, false);
    }
    }
    }
// label;
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    }
    EXPORT_SYMBOL_GPL(tick_broadcast_control);
//
// Set the periodic handler depending on broadcast on/off
//
#[no_mangle]
pub unsafe extern "C" fn tick_set_periodic_handler(dev: *mut clock_event_device, broadcast: c_int) {
    if (!broadcast) {
    dev.event_handler = tick_handle_periodic;
    }
    else {
    dev.event_handler = tick_handle_periodic_broadcast;
    }
    }

#[no_mangle]
unsafe extern "C" fn tick_shutdown_broadcast() {
    let mut bc = tick_broadcast_device.evtdev;
    if (tick_broadcast_device.mode == TICKDEV_MODE_PERIODIC) {
    if (bc && cpumask_empty(tick_broadcast_mask)) {
    clockevents_shutdown(bc);
    }
    }
    }
//
// Remove a CPU from broadcasting
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_offline(cpu: c_uint) {
    raw_spin_lock(&tick_broadcast_lock);
    cpumask_clear_cpu(cpu, tick_broadcast_mask);
    cpumask_clear_cpu(cpu, tick_broadcast_on);
    tick_broadcast_oneshot_offline(cpu);
    tick_shutdown_broadcast();
    raw_spin_unlock(&tick_broadcast_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn tick_suspend_broadcast() {
pub static mut bc: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
    bc = tick_broadcast_device.evtdev;
    if (bc) {
    clockevents_shutdown(bc);
    }
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    }
//
// This is called from tick_resume_local() on a resuming CPU. That's
// called from the core resume function, tick_unfreeze() and the magic XEN
// resume hackery.
//
// In none of these cases the broadcast device mode can change and the
// bit of the resuming CPU in the broadcast mask is safe as well.
//
#[no_mangle]
pub unsafe extern "C" fn tick_resume_check_broadcast() -> bool {
    if (tick_broadcast_device.mode == TICKDEV_MODE_ONESHOT) {
    return false;
    }
    else {
    return cpumask_test_cpu(smp_processor_id(), tick_broadcast_mask);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tick_resume_broadcast() {
pub static mut bc: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
    bc = tick_broadcast_device.evtdev;
    if (bc) {
    clockevents_tick_resume(bc);
    match (tick_broadcast_device.mode) {
    TICKDEV_MODE_PERIODIC => {
    if (!cpumask_empty(tick_broadcast_mask)) {
    tick_broadcast_start_periodic(bc);
    }
    // break;
    }
    TICKDEV_MODE_ONESHOT => {
    if (!cpumask_empty(tick_broadcast_mask)) {
    tick_resume_broadcast_oneshot(bc);
    }
    // break;
    }
    }
    }
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    }

    static cpumask_var_t tick_broadcast_oneshot_mask __cpumask_var_read_mostly;
    static cpumask_var_t tick_broadcast_pending_mask __cpumask_var_read_mostly;
    static cpumask_var_t tick_broadcast_force_mask __cpumask_var_read_mostly;
//
// Exposed for debugging: see timer_list.c
//
#[no_mangle]
pub unsafe extern "C" fn tick_get_broadcast_oneshot_mask() -> *mut c_void {
    return tick_broadcast_oneshot_mask;
    }
//
// Called before going idle with interrupts disabled. Checks whether a
// broadcast event from the other core is about to happen. We detected
// that in tick_broadcast_oneshot_control(). The callsite can use this
// to avoid a deep idle transition as we are about to get the
// broadcast IPI right away.
//
#[no_mangle]
pub unsafe extern "C" fn tick_check_broadcast_expired() -> noinstr int {

    return arch_test_bit(smp_processor_id(), cpumask_bits(tick_broadcast_force_mask));

    return cpumask_test_cpu(smp_processor_id(), tick_broadcast_force_mask);

    }
//
// Set broadcast interrupt affinity
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_set_affinity(bc: *mut clock_event_device, cpumask: *mut cpumask) {
    if (!(bc.features & CLOCK_EVT_FEAT_DYNIRQ)) {
    return;
    }
    if (cpumask_equal(bc.cpumask, cpumask)) {
    return;
    }
    bc.cpumask = cpumask;
    irq_set_affinity(bc.irq, bc.cpumask);
    }
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_set_event(bc: *mut clock_event_device, cpu: c_int, expires: ktime_t) {
    if (!clockevent_state_oneshot(bc)) {
    clockevents_switch_state(bc, CLOCK_EVT_STATE_ONESHOT);
    }
    clockevents_program_event(bc, expires, 1);
    tick_broadcast_set_affinity(bc, cpumask_of(cpu));
    }
#[no_mangle]
unsafe extern "C" fn tick_resume_broadcast_oneshot(bc: *mut clock_event_device) {
    clockevents_switch_state(bc, CLOCK_EVT_STATE_ONESHOT);
    }
//
// Called from irq_enter() when idle was interrupted to reenable the
// per cpu device.
//
#[no_mangle]
pub unsafe extern "C" fn tick_check_oneshot_broadcast_this_cpu() {
    if (cpumask_test_cpu(smp_processor_id(), tick_broadcast_oneshot_mask)) {
    let mut td = this_cpu_ptr(&tick_cpu_device);
//
// We might be in the middle of switching over from
// periodic to oneshot. If the CPU has not yet
// switched over, leave the device alone.
//
    if (td.mode == TICKDEV_MODE_ONESHOT) {
    clockevents_switch_state(td.evtdev,
    CLOCK_EVT_STATE_ONESHOT);
    }
    }
    }
//
// Handle oneshot mode broadcasting
//
#[no_mangle]
unsafe extern "C" fn tick_handle_oneshot_broadcast(dev: *mut clock_event_device) {
pub static mut td: *mut c_void = core::ptr::null_mut();
    ktime_t now, next_event;
    int cpu, next_cpu = 0;
    let mut bc_local = 0;
    raw_spin_lock(&tick_broadcast_lock);
    dev.next_event = KTIME_MAX;
    tick_broadcast_device.evtdev.next_event_forced = 0;
    next_event = KTIME_MAX;
    cpumask_clear(tmpmask);
    now = ktime_get();
// Find all expired events
    for_each_cpu(cpu, tick_broadcast_oneshot_mask) {
//
// Required for !SMP because for_each_cpu() reports
// unconditionally CPU0 as set on UP kernels.
//
    if (!IS_ENABLED!(CONFIG_SMP) &&
    cpumask_empty(tick_broadcast_oneshot_mask)) {
    break;
    }
    td = &per_cpu(tick_cpu_device, cpu);
    if (td.evtdev.next_event <= now) {
    cpumask_set_cpu(cpu, tmpmask);
//
// Mark the remote cpu in the pending mask, so
// it can avoid reprogramming the cpu local
// timer in tick_broadcast_oneshot_control().
//
    cpumask_set_cpu(cpu, tick_broadcast_pending_mask);
    } else if (td.evtdev.next_event < next_event) {
    next_event = td.evtdev.next_event;
    next_cpu = cpu;
    }
    }
//
// Remove the current cpu from the pending mask. The event is
// delivered immediately in tick_do_broadcast() !
//
    cpumask_clear_cpu(smp_processor_id(), tick_broadcast_pending_mask);
// Take care of enforced broadcast requests
    cpumask_or(tmpmask, tmpmask, tick_broadcast_force_mask);
    cpumask_clear(tick_broadcast_force_mask);
//
// Sanity check. Catch the case where we try to broadcast to
// offline cpus.
//
    if (WARN_ON_ONCE!(!cpumask_subset(tmpmask, cpu_online_mask))) {
    cpumask_and(tmpmask, tmpmask, cpu_online_mask);
    }
//
// Wakeup the cpus which have an expired event.
//
    bc_local = tick_do_broadcast(tmpmask);
//
// Two reasons for reprogram:
//
// - The global event did not expire any CPU local
// events. This happens in dyntick mode, as the maximum PIT
// delta is quite small.
//
// - There are pending events on sleeping CPUs which were not
// in the event mask
//
    if (next_event != KTIME_MAX) {
    tick_broadcast_set_event(dev, next_cpu, next_event);
    }
    raw_spin_unlock(&tick_broadcast_lock);
    if (bc_local) {
    td = this_cpu_ptr(&tick_cpu_device);
    td.evtdev.event_handler(td.evtdev);
    }
    }
#[no_mangle]
unsafe extern "C" fn broadcast_needs_cpu(bc: *mut clock_event_device, cpu: c_int) -> c_int {
    if (!(bc.features & CLOCK_EVT_FEAT_HRTIMER)) {
    return 0;
    }
    if (bc.next_event == KTIME_MAX) {
    return 0;
    }
    return bc.bound_on == cpu ? -EBUSY : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn broadcast_shutdown_local(bc: *mut clock_event_device, dev: *mut clock_event_device) {
//
// For hrtimer based broadcasting we cannot shutdown the cpu
// local device if our own event is the first one to expire or
// if we own the broadcast timer.
//
    if (bc.features & CLOCK_EVT_FEAT_HRTIMER) {
    if (broadcast_needs_cpu(bc, smp_processor_id())) {
    return;
    }
    if (dev.next_event < bc.next_event) {
    return;
    }
    }
    clockevents_switch_state(dev, CLOCK_EVT_STATE_SHUTDOWN);
    }
#[no_mangle]
pub unsafe extern "C" fn ___tick_broadcast_oneshot_control(state: tick_broadcast_state, td: *mut tick_device, cpu: c_int) -> c_int {
    struct clock_event_device *bc, *dev = td.evtdev;
pub static mut ret: c_int = 0;
    let mut now;
    raw_spin_lock(&tick_broadcast_lock);
    bc = tick_broadcast_device.evtdev;
    if (state == TICK_BROADCAST_ENTER) {
//
// If the current CPU owns the hrtimer broadcast
// mechanism, it cannot go deep idle and we do not add
// the CPU to the broadcast mask. We don't have to go
// through the EXIT path as the local timer is not
// shutdown.
//
    ret = broadcast_needs_cpu(bc, cpu);
    if (ret) {
// goto;
    }
//
// If the broadcast device is in periodic mode, we
// return.
//
    if (tick_broadcast_device.mode == TICKDEV_MODE_PERIODIC) {
// If it is a hrtimer based broadcast, return busy
    if (bc.features & CLOCK_EVT_FEAT_HRTIMER) {
    ret = -EBUSY;
    }
// goto;
    }
    if (!cpumask_test_and_set_cpu(cpu, tick_broadcast_oneshot_mask)) {
    WARN_ON_ONCE!(cpumask_test_cpu(cpu, tick_broadcast_pending_mask));
// Conditionally shut down the local timer.
    broadcast_shutdown_local(bc, dev);
//
// We only reprogram the broadcast timer if we
// did not mark ourself in the force mask and
// if the cpu local event is earlier than the
// broadcast event. If the current CPU is in
// the force mask, then we are going to be
// woken by the IPI right away; we return
// busy, so the CPU does not try to go deep
// idle.
//
    if (cpumask_test_cpu(cpu, tick_broadcast_force_mask)) {
    ret = -EBUSY;
    } else if (dev.next_event < bc.next_event) {
    tick_broadcast_set_event(bc, cpu, dev.next_event);
//
// In case of hrtimer broadcasts the
// programming might have moved the
// timer to this cpu. If yes, remove
// us from the broadcast mask and
// return busy.
//
    ret = broadcast_needs_cpu(bc, cpu);
    if (ret) {
    cpumask_clear_cpu(cpu,
    tick_broadcast_oneshot_mask);
    }
    }
    }
    } else {
    if (cpumask_test_and_clear_cpu(cpu, tick_broadcast_oneshot_mask)) {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
//
// The cpu which was handling the broadcast
// timer marked this cpu in the broadcast
// pending mask and fired the broadcast
// IPI. So we are going to handle the expired
// event anyway via the broadcast IPI
// handler. No need to reprogram the timer
// with an already expired event.
//
    if (cpumask_test_and_clear_cpu(cpu,
    tick_broadcast_pending_mask)) {
// goto;
    }
//
// Bail out if there is no next event.
//
    if (dev.next_event == KTIME_MAX) {
// goto;
    }
//
// If the pending bit is not set, then we are
// either the CPU handling the broadcast
// interrupt or we got woken by something else.
//
// We are no longer in the broadcast mask, so
// if the cpu local expiry time is already
// reached, we would reprogram the cpu local
// timer with an already expired event.
//
// This can lead to a ping-pong when we return
// to idle and therefore rearm the broadcast
// timer before the cpu local timer was able
// to fire. This happens because the forced
// reprogramming makes sure that the event
// will happen in the future and depending on
// the min_delta setting this might be far
// enough out that the ping-pong starts.
//
// If the cpu local next_event has expired
// then we know that the broadcast timer
// next_event has expired as well and
// broadcast is about to be handled. So we
// avoid reprogramming and enforce that the
// broadcast handler, which did not run yet,
// will invoke the cpu local handler.
//
// We cannot call the handler directly from
// here, because we might be in a NOHZ phase
// and we did not go through the irq_enter()
// nohz fixups.
//
    now = ktime_get();
    if (dev.next_event <= now) {
    cpumask_set_cpu(cpu, tick_broadcast_force_mask);
// goto;
    }
//
// We got woken by something else. Reprogram
// the cpu local timer device.
//
    tick_program_event(dev.next_event, 1);
    }
    }
// label;
    raw_spin_unlock(&tick_broadcast_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tick_oneshot_wakeup_control(state: tick_broadcast_state, td: *mut tick_device, cpu: c_int) -> c_int {
    let mut dev = core::ptr::null_mut();
    let mut wd = core::ptr::null_mut();
    dev = td.evtdev;
    if (td.mode != TICKDEV_MODE_ONESHOT) {
    return -EINVAL;
    }
    wd = tick_get_oneshot_wakeup_device(cpu);
    if (!wd) {
    return -ENODEV;
    }
    match (state) {
    TICK_BROADCAST_ENTER => {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT_STOPPED);
    clockevents_switch_state(wd, CLOCK_EVT_STATE_ONESHOT);
    clockevents_program_event(wd, dev.next_event, 1);
    // break;
    }
    TICK_BROADCAST_EXIT => {
// We may have transitioned to oneshot mode while idle
    if (clockevent_get_state(wd) != CLOCK_EVT_STATE_ONESHOT) {
    return -ENODEV;
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __tick_broadcast_oneshot_control(state: tick_broadcast_state) -> c_int {
    let mut td = this_cpu_ptr(&tick_cpu_device);
pub static mut cpu: c_int = 0;
    if (!tick_oneshot_wakeup_control(state, td, cpu)) {
    return 0;
    }
    if (tick_broadcast_device.evtdev) {
    return ___tick_broadcast_oneshot_control(state, td, cpu);
    }
//
// If there is no broadcast or wakeup device, tell the caller not
// to go into deep idle.
//
    return -EBUSY;
    }
//
// Reset the one shot broadcast for a cpu
//
// Called with tick_broadcast_lock held
//
#[no_mangle]
unsafe extern "C" fn tick_broadcast_clear_oneshot(cpu: c_int) {
    cpumask_clear_cpu(cpu, tick_broadcast_oneshot_mask);
    cpumask_clear_cpu(cpu, tick_broadcast_pending_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_init_next_event(mask: *mut cpumask, expires: ktime_t) {
pub static mut td: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_cpu(cpu, mask) {
    td = &per_cpu(tick_cpu_device, cpu);
    if (td.evtdev) {
    td.evtdev.next_event = expires;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tick_get_next_period() -> ktime_t {
    let mut next;
//
// Protect against concurrent updates (store /load tearing on
// 32bit). It does not matter if the time is already in the
// past. The broadcast device which is about to be programmed will
// fire in any case.
//
    raw_spin_lock(&jiffies_lock);
    next = tick_next_period;
    raw_spin_unlock(&jiffies_lock);
    return next;
    }
//
// tick_broadcast_setup_oneshot - setup the broadcast device
// @bc: the broadcast device
// @from_periodic: true if called from periodic mode
//
#[no_mangle]
#[no_mangle]
// duplicate fn: tick_broadcast_setup_oneshot
pub unsafe extern "C" fn tick_broadcast_setup_oneshot_dup(bc: *mut clock_event_device, from_periodic: bool) {
pub static mut cpu: c_int = 0;
pub static mut nexttick: ktime_t = 0;
    if (!bc) {
    return;
    }
//
// When the broadcast device was switched to oneshot by the first
// CPU handling the NOHZ change, the other CPUs will reach this
// code via hrtimer_run_queues() -> tick_check_oneshot_change()
// too. Set up the broadcast device only once!
//
    if (bc.event_handler == tick_handle_oneshot_broadcast) {
//
// The CPU which switched from periodic to oneshot mode
// set the broadcast oneshot bit for all other CPUs which
// are in the general (periodic) broadcast mask to ensure
// that CPUs which wait for the periodic broadcast are
// woken up.
//
// Clear the bit for the local CPU as the set bit would
// prevent the first tick_broadcast_enter() after this CPU
// switched to oneshot state to program the broadcast
// device.
//
// This code can also be reached via tick_broadcast_control(),
but this cannot avoid the tick_broadcast_clear_oneshot()
// as that would break the periodic to oneshot transition of
// secondary CPUs. But that's harmless as the below only
// clears already cleared bits.
//
    tick_broadcast_clear_oneshot(cpu);
    return;
    }
    bc.event_handler = tick_handle_oneshot_broadcast;
    bc.next_event_forced = 0;
    bc.next_event = KTIME_MAX;
//
// When the tick mode is switched from periodic to oneshot it must
// be ensured that CPUs which are waiting for periodic broadcast
// get their wake-up at the next tick.  This is achieved by ORing
// tick_broadcast_mask into tick_broadcast_oneshot_mask.
//
// For other callers, e.g. broadcast device replacement,
// tick_broadcast_oneshot_mask must not be touched as this would
// set bits for CPUs which are already NOHZ, but not idle. Their
// next tick_broadcast_enter() would observe the bit set and fail
// to update the expiry time and the broadcast event device.
//
    if (from_periodic) {
    cpumask_copy(tmpmask, tick_broadcast_mask);
// Remove the local CPU as it is obviously not idle
    cpumask_clear_cpu(cpu, tmpmask);
    cpumask_or(tick_broadcast_oneshot_mask, tick_broadcast_oneshot_mask, tmpmask);
//
// Ensure that the oneshot broadcast handler will wake the
// CPUs which are still waiting for periodic broadcast.
//
    nexttick = tick_get_next_period();
    tick_broadcast_init_next_event(tmpmask, nexttick);
//
// If the underlying broadcast clock event device is
// already in oneshot state, then there is nothing to do.
// The device was already armed for the next tick
// in tick_handle_broadcast_periodic()
//
    if (clockevent_state_oneshot(bc)) {
    return;
    }
    }
//
// When switching from periodic to oneshot mode arm the broadcast
// device for the next tick.
//
// If the broadcast device has been replaced in oneshot mode and
// the oneshot broadcast mask is not empty, then arm it to expire
// immediately in order to reevaluate the next expiring timer.
// @nexttick is 0 and therefore in the past which will cause the
// clockevent code to force an event.
//
// For both cases the programming can be avoided when the oneshot
// broadcast mask is empty.
//
// tick_broadcast_set_event() implicitly switches the broadcast
// device to oneshot state.
//
    if (!cpumask_empty(tick_broadcast_oneshot_mask)) {
    tick_broadcast_set_event(bc, cpu, nexttick);
    }
    }
//
// Select oneshot operating mode for the broadcast device
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_switch_to_oneshot() {
pub static mut bc: *mut c_void = core::ptr::null_mut();
    enum tick_device_mode oldmode;
    let mut flags = 0;
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
    oldmode = tick_broadcast_device.mode;
    tick_broadcast_device.mode = TICKDEV_MODE_ONESHOT;
    bc = tick_broadcast_device.evtdev;
    if (bc) {
    tick_broadcast_setup_oneshot(bc, oldmode == TICKDEV_MODE_PERIODIC);
    }
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    }

#[no_mangle]
pub unsafe extern "C" fn hotplug_cpu__broadcast_tick_pull(deadcpu: c_int) {
pub static mut bc: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    raw_spin_lock_irqsave(&tick_broadcast_lock, flags);
    bc = tick_broadcast_device.evtdev;
    if (bc && broadcast_needs_cpu(bc, deadcpu)) {
//
// If the broadcast force bit of the current CPU is set,
// then the current CPU has not yet reprogrammed the local
// timer device to avoid a ping-pong race. See
// ___tick_broadcast_oneshot_control().
//
// If the broadcast device is hrtimer based then
// programming the broadcast event below does not have any
// effect because the local clockevent device is not
// running and not programmed because the broadcast event
// is not earlier than the pending event of the local clock
// event device. As a consequence all CPUs waiting for a
// broadcast event are stuck forever.
//
// Detect this condition and reprogram the cpu local timer
// device to avoid the starvation.
//
    if (tick_check_broadcast_expired()) {
    let mut td = this_cpu_ptr(&tick_cpu_device);
    cpumask_clear_cpu(smp_processor_id(), tick_broadcast_force_mask);
    tick_program_event(td.evtdev.next_event, 1);
    }
// This moves the broadcast assignment to this CPU:
    bc.next_event_forced = 0;
    clockevents_program_event(bc, bc.next_event, 1);
    }
    raw_spin_unlock_irqrestore(&tick_broadcast_lock, flags);
    }
//
// Remove a dying CPU from broadcasting
//
#[no_mangle]
unsafe extern "C" fn tick_broadcast_oneshot_offline(cpu: c_uint) {
    if (tick_get_oneshot_wakeup_device(cpu)) {
    tick_set_oneshot_wakeup_device(core::ptr::null_mut(), cpu);
    }
//
// Clear the broadcast masks for the dead cpu, but do not stop
// the broadcast device!
//
    cpumask_clear_cpu(cpu, tick_broadcast_oneshot_mask);
    cpumask_clear_cpu(cpu, tick_broadcast_pending_mask);
    cpumask_clear_cpu(cpu, tick_broadcast_force_mask);
    }

//
// Check, whether the broadcast device is in one shot mode
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_oneshot_active() -> c_int {
    return tick_broadcast_device.mode == TICKDEV_MODE_ONESHOT;
    }
//
// Check whether the broadcast device supports oneshot.
//
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_oneshot_available() -> bool {
    let mut bc = tick_broadcast_device.evtdev;
    return bc ? bc.features & CLOCK_EVT_FEAT_ONESHOT : false;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __tick_broadcast_oneshot_control
pub unsafe extern "C" fn __tick_broadcast_oneshot_control_dup(state: tick_broadcast_state) -> c_int {
    let mut bc = tick_broadcast_device.evtdev;
    if (!bc || (bc.features & CLOCK_EVT_FEAT_HRTIMER)) {
    return -EBUSY;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_init()  {
    zalloc_cpumask_var(&tick_broadcast_mask, GFP_NOWAIT);
    zalloc_cpumask_var(&tick_broadcast_on, GFP_NOWAIT);
    zalloc_cpumask_var(&tmpmask, GFP_NOWAIT);

    zalloc_cpumask_var(&tick_broadcast_oneshot_mask, GFP_NOWAIT);
    zalloc_cpumask_var(&tick_broadcast_pending_mask, GFP_NOWAIT);
    zalloc_cpumask_var(&tick_broadcast_force_mask, GFP_NOWAIT);

    }