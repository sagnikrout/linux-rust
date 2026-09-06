//! Automatically rewritten from C to Rust
//! Source: kernel/time/clockevents.c
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
// This file contains functions which manage clock event devices.
//
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
//

// The registered clock event devices
pub static mut clockevent_devices: usize = 0;
pub static mut clockevents_released: usize = 0;
// Protection for the above
pub static mut clockevents_lock: usize = 0;
// Protection for unbind operations
pub static mut clockevents_mutex: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_unbind {
    pub ce: *mut clock_event_device,
    pub res: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn cev_delta2ns(latch: c_ulong, evt: *mut clock_event_device, ismax: bool) -> u64 {
pub static mut clc: u64 = 0;
    let mut rnd = 0;
    if (WARN_ON!(!evt.mult)) {
    evt.mult = 1;
    }
    rnd = (u64) evt.mult - 1;
//
// Upper bound sanity check. If the backwards conversion is
// not equal latch, we know that the above shift overflowed.
//
    if ((clc >> evt.shift) != (u64)latch) {
    clc = ~0ULL;
    }
//
// Scaled math oddities:
//
// For mult <= (1 << shift) we can safely add mult - 1 to
// prevent integer rounding loss. So the backwards conversion
// from nsec to device ticks will be correct.
//
// For mult > (1 << shift), i.e. device frequency is > 1GHz we
// need to be careful. Adding mult - 1 will result in a value
// which when converted back to device ticks can be larger
// than latch by up to (mult - 1) >> shift. For the min_delta
// calculation we still want to apply this in order to stay
// above the minimum device ticks limit. For the upper limit
// we would end up with a latch value larger than the upper
// limit of the device, so we omit the add to stay below the
// device upper boundary.
//
// Also omit the add if it would overflow the u64 boundary.
//
    if ((~0ULL - clc > rnd) &&
    (!ismax || evt.mult <= (1ULL << evt.shift))) {
    clc += rnd;
    }
    do_div(clc, evt.mult);
// Deltas less than 1usec are pointless noise
    return clc > 1000 ? clc : 1000;
    }
//
// clockevent_delta2ns - Convert a latch value (device ticks) to nanoseconds
// @latch:	value to convert
// @evt:	pointer to clock event device descriptor
//
// Math helper, returns latch value converted to nanoseconds (bound checked)
//
#[no_mangle]
pub unsafe extern "C" fn clockevent_delta2ns(latch: c_ulong, evt: *mut clock_event_device) -> u64 {
    return cev_delta2ns(latch, evt, false);
    }
    EXPORT_SYMBOL_GPL(clockevent_delta2ns);
#[no_mangle]
pub unsafe extern "C" fn __clockevents_switch_state(dev: *mut clock_event_device, state: clock_event_state) -> c_int {
    if (dev.features & CLOCK_EVT_FEAT_DUMMY) {
    return 0;
    }
// On state transitions clear the forced flag unconditionally
    dev.next_event_forced = 0;
// Transition with new state-specific callbacks
    match (state) {
    CLOCK_EVT_STATE_DETACHED => {
// The clockevent device is getting replaced. Shut it down.
    }
    CLOCK_EVT_STATE_SHUTDOWN => {
    if (dev.set_state_shutdown) {
    return dev.set_state_shutdown(dev);
    }
    return 0;
    }
    CLOCK_EVT_STATE_PERIODIC => {
// Core internal bug
    if (!(dev.features & CLOCK_EVT_FEAT_PERIODIC)) {
    return -ENOSYS;
    }
    if (dev.set_state_periodic) {
    return dev.set_state_periodic(dev);
    }
    return 0;
    }
    CLOCK_EVT_STATE_ONESHOT => {
// Core internal bug
    if (!(dev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return -ENOSYS;
    }
    if (dev.set_state_oneshot) {
    return dev.set_state_oneshot(dev);
    }
    return 0;
    }
    CLOCK_EVT_STATE_ONESHOT_STOPPED => {
// Core internal bug
    if (WARN_ONCE(!clockevent_state_oneshot(dev),
    "Current state: %d\n",
    clockevent_get_state(dev))) {
    return -EINVAL;
    }
    if (dev.set_state_oneshot_stopped) {
    return dev.set_state_oneshot_stopped(dev);
    }
    else {
    return -ENOSYS;
    }
    }
    _ => {
    return -ENOSYS;
    }
    }
    }
//
// clockevents_switch_state - set the operating state of a clock event device
// @dev:	device to modify
// @state:	new state
//
// Must be called with interrupts disabled !
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_switch_state(dev: *mut clock_event_device, state: clock_event_state) {
    if (clockevent_get_state(dev) != state) {
    if (__clockevents_switch_state(dev, state)) {
    return;
    }
    clockevent_set_state(dev, state);
//
// A nsec2cyc multiplicator of 0 is invalid and we'd crash
// on it, so fix it up and emit a warning:
//
    if (clockevent_state_oneshot(dev)) {
    if (WARN_ON!(!dev.mult)) {
    dev.mult = 1;
    }
    }
    }
    }
//
// clockevents_shutdown - shutdown the device and clear next_event
// @dev:	device to shutdown
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_shutdown(dev: *mut clock_event_device) {
    clockevents_switch_state(dev, CLOCK_EVT_STATE_SHUTDOWN);
    dev.next_event = KTIME_MAX;
    dev.next_event_forced = 0;
    }
//
// clockevents_tick_resume -	Resume the tick device before using it again
// @dev:			device to resume
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_tick_resume(dev: *mut clock_event_device) -> c_int {
pub static mut ret: c_int = 0;
    if (dev.tick_resume) {
    ret = dev.tick_resume(dev);
    }
    return ret;
    }

// Limit min_delta to a jiffy

//
// clockevents_increase_min_delta - raise minimum delta of a clock event device
// @dev:       device to increase the minimum delta
//
// Returns 0 on success, -ETIME when the minimum delta reached the limit.
//
#[no_mangle]
unsafe extern "C" fn clockevents_increase_min_delta(dev: *mut clock_event_device) -> c_int {
// Nothing to do if we already reached the limit
    if (dev.min_delta_ns >= MIN_DELTA_LIMIT) {
    printk_deferred("CE: Reprogramming failure. Giving up\n");
    dev.next_event = KTIME_MAX;
    return -ETIME;
    }
    if (dev.min_delta_ns < 5000) {
    dev.min_delta_ns = 5000;
    }
    else {
    dev.min_delta_ns += dev.min_delta_ns >> 1;
    }
    if (dev.min_delta_ns > MIN_DELTA_LIMIT) {
    dev.min_delta_ns = MIN_DELTA_LIMIT;
    }
    printk_deferred("CE: %s increased min_delta_ns to %llu nsec\n",
    dev.name ? dev.name : "?",
    (unsigned long long) dev.min_delta_ns);
    return 0;
    }
//
// clockevents_program_min_delta - Set clock event device to the minimum delay.
// @dev:	device to program
//
// Returns 0 on success, -ETIME when the retry loop failed.
//
#[no_mangle]
unsafe extern "C" fn clockevents_program_min_delta(dev: *mut clock_event_device) -> c_int {
    unsigned long long clc;
    let mut delta;
    let mut i = 0;
    for (i = 0;;) {
    delta = dev.min_delta_ns;
    dev.next_event = ktime_add_ns(ktime_get(), delta);
    if (clockevent_state_shutdown(dev)) {
    return 0;
    }
    dev.retries += 1;
    clc = ((unsigned long long) delta * dev.mult) >> dev.shift;
    if (dev.set_next_event((unsigned long) clc, dev) == 0) {
    return 0;
    }
    if (++i > 2) {
//
// We tried 3 times to program the device with the
// given min_delta_ns. Try to increase the minimum
// delta, if that fails as well get out of here.
//
    if (clockevents_increase_min_delta(dev)) {
    return -ETIME;
    }
    i = 0;
    }
    }
    }

//
// clockevents_program_min_delta - Set clock event device to the minimum delay.
// @dev:	device to program
//
// Returns 0 on success, -ETIME when the retry loop failed.
//
#[no_mangle]
unsafe extern "C" fn clockevents_program_min_delta(dev: *mut clock_event_device) -> c_int {
    unsigned long long clc;
pub static mut delta: i64 = 0;
    let mut i = 0;
    while (i < 10) {
    delta += dev.min_delta_ns;
    dev.next_event = ktime_add_ns(ktime_get(), delta);
    if (clockevent_state_shutdown(dev)) {
    return 0;
    }
    dev.retries += 1;
    clc = ((unsigned long long) delta * dev.mult) >> dev.shift;
    if (dev.set_next_event((unsigned long) clc, dev) == 0) {
    return 0;
    }
    }
    return -ETIME;
    }

    static __always_inline void
    arch_inlined_clockevent_set_next_coupled(u64 cycles, clock_event_device *dev) { }

#[no_mangle]
pub unsafe extern "C" fn clockevent_set_next_coupled(dev: *mut clock_event_device, expires: ktime_t) -> bool {
    let mut cycles = 0;
    if (unlikely(!(dev.features & CLOCK_EVT_FEAT_CLOCKSOURCE_COUPLED))) {
    return false;
    }
    if (unlikely(!ktime_expiry_to_cycles(dev.cs_id, expires, &cycles))) {
    return false;
    }
    if (IS_ENABLED!(CONFIG_GENERIC_CLOCKEVENTS_COUPLED_INLINE)) {
    arch_inlined_clockevent_set_next_coupled(cycles, dev);
    }
    else {
    dev.set_next_coupled(cycles, dev);
    }
    return true;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: clockevent_set_next_coupled
pub unsafe extern "C" fn clockevent_set_next_coupled_dup(dev: *mut clock_event_device, expires: ktime_t) -> bool {
    return false;
    }

//
// clockevents_program_event - Reprogram the clock event device.
// @dev:	device to program
// @expires:	absolute expiry time (monotonic clock)
// @force:	program minimum delay if expires can not be set
//
// Returns 0 on success, -ETIME when the event is in the past.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_program_event(dev: *mut clock_event_device, expires: ktime_t, force: bool) -> c_int {
    let mut delta;
    let mut cycles = 0;
    if (WARN_ON_ONCE!(expires < 0)) {
    return -ETIME;
    }
    dev.next_event = expires;
    if (clockevent_state_shutdown(dev)) {
    return 0;
    }
// We must be in ONESHOT state here
    WARN_ONCE(!clockevent_state_oneshot(dev), "Current state: %d\n",
    clockevent_get_state(dev));
// ktime_t based reprogramming for the broadcast hrtimer device
    if (unlikely(dev.features & CLOCK_EVT_FEAT_HRTIMER)) {
    return dev.set_next_ktime(expires, dev);
    }
    if (likely(clockevent_set_next_coupled(dev, expires))) {
    return 0;
    }
    delta = ktime_to_ns(ktime_sub(expires, ktime_get()));
// Required for tick_periodic() during early boot
    if (delta <= 0 && !force) {
    return -ETIME;
    }
    if (delta > (int64_t)dev.min_delta_ns) {
    delta = min(delta, (int64_t) dev.max_delta_ns);
    cycles = ((u64)delta * dev.mult) >> dev.shift;
    if (!dev.set_next_event((unsigned long) cycles, dev)) {
    dev.next_event_forced = 0;
    return 0;
    }
    }
    if (dev.next_event_forced) {
    return 0;
    }
    if (dev.set_next_event(dev.min_delta_ticks, dev)) {
    if (!force || clockevents_program_min_delta(dev)) {
    return -ETIME;
    }
    }
    dev.next_event_forced = 1;
    return 0;
    }
//
// Called after a clockevent has been added which might
// have replaced a current regular or broadcast device. A
// released normal device might be a suitable replacement
// for the current broadcast device. Similarly a released
// broadcast device might be a suitable replacement for a
// normal device.
//
#[no_mangle]
unsafe extern "C" fn clockevents_notify_released() {
pub static mut dev: *mut c_void = core::ptr::null_mut();
//
// Keep iterating as long as tick_check_new_device()
// replaces a device.
//
    while (!list_empty(&clockevents_released)) {
    dev = list_entry(clockevents_released.next, clock_event_device, list);
    list_move(&dev.list, &clockevent_devices);
    tick_check_new_device(dev);
    }
    }
//
// Try to install a replacement clock event device
//
#[no_mangle]
unsafe extern "C" fn clockevents_replace(ced: *mut clock_event_device) -> c_int {
    struct clock_event_device *dev, *newdev = core::ptr::null_mut();
    list_for_each_entry(dev, &clockevent_devices, list) {
    if (dev == ced || !clockevent_state_detached(dev)) {
    continue;
    }
    if (!tick_check_replacement(newdev, dev)) {
    continue;
    }
    if (!try_module_get(dev.owner)) {
    continue;
    }
    if (newdev) {
    module_put!(newdev.owner);
    }
    newdev = dev;
    }
    if (newdev) {
    tick_install_replacement(newdev);
    list_del_init(&ced.list);
    }
    return newdev ? 0 : -EBUSY;
    }
//
// Called with clockevents_mutex and clockevents_lock held
//
#[no_mangle]
unsafe extern "C" fn __clockevents_try_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int {
// Fast track. Device is unused
    if (clockevent_state_detached(ced)) {
    list_del_init(&ced.list);
    return 0;
    }
pub static mut ced: return = 0;
    }
//
// SMP function call to unbind a device
//
#[no_mangle]
unsafe extern "C" fn __clockevents_unbind(arg: *mut c_void) {
    let mut cu = arg;
    let mut res = 0;
    raw_spin_lock(&clockevents_lock);
    res = __clockevents_try_unbind(cu.ce, smp_processor_id());
    if (res == -EAGAIN) {
    res = clockevents_replace(cu.ce);
    }
    cu.res = res;
    raw_spin_unlock(&clockevents_lock);
    }
//
// Issues smp function call to unbind a per cpu device. Called with
// clockevents_mutex held.
//
#[no_mangle]
unsafe extern "C" fn clockevents_unbind(ced: *mut clock_event_device, cpu: c_int) -> c_int {
pub static mut cu: ce_unbind = 0;
    smp_call_function_single(cpu, __clockevents_unbind, &cu, 1);
    return cu.res;
    }
//
// Unbind a clockevents device.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_unbind_device(ced: *mut clock_event_device, cpu: c_int) -> c_int {
    let mut ret = 0;
    mutex_lock(&clockevents_mutex);
    ret = clockevents_unbind(ced, cpu);
    mutex_unlock(&clockevents_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(clockevents_unbind_device);
//
// clockevents_register_device - register a clock event device
// @dev:	device to register
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_register_device(dev: *mut clock_event_device) {
    let mut flags = 0;
// Initialize state to DETACHED
    clockevent_set_state(dev, CLOCK_EVT_STATE_DETACHED);
    if (!dev.cpumask) {
    WARN_ON!(num_possible_cpus() > 1);
    dev.cpumask = cpumask_of(smp_processor_id());
    }
    if (dev.cpumask == cpu_all_mask) {
    WARN(1, "%s cpumask == cpu_all_mask, using cpu_possible_mask instead\n",
    dev.name);
    dev.cpumask = cpu_possible_mask;
    }
    raw_spin_lock_irqsave(&clockevents_lock, flags);
    list_add(&dev.list, &clockevent_devices);
    tick_check_new_device(dev);
    clockevents_notify_released();
    raw_spin_unlock_irqrestore(&clockevents_lock, flags);
    }
    EXPORT_SYMBOL_GPL(clockevents_register_device);
#[no_mangle]
unsafe extern "C" fn clockevents_config(dev: *mut clock_event_device, freq: u32) {
    let mut sec = 0;
    if (!(dev.features & CLOCK_EVT_FEAT_ONESHOT)) {
    return;
    }
//
// Calculate the maximum number of seconds we can sleep. Limit
// to 10 minutes for hardware which can program more than
// 32bit ticks so we still get reasonable conversion values.
//
    sec = dev.max_delta_ticks;
    do_div(sec, freq);
    if (!sec) {
    sec = 1;
    }

    else if (sec > 600 && dev.max_delta_ticks > UINT_MAX) {
    sec = 600;
    }
    clockevents_calc_mult_shift(dev, freq, sec);
    dev.min_delta_ns = cev_delta2ns(dev.min_delta_ticks, dev, false);
    dev.max_delta_ns = cev_delta2ns(dev.max_delta_ticks, dev, true);
    }
//
// clockevents_config_and_register - Configure and register a clock event device
// @dev:	device to register
// @freq:	The clock frequency
// @min_delta:	The minimum clock ticks to program in oneshot mode
// @max_delta:	The maximum clock ticks to program in oneshot mode
//
// min/max_delta can be 0 for devices which do not support oneshot mode.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_config_and_register(dev: *mut clock_event_device, freq: u32, min_delta: c_ulong, max_delta: c_ulong) {
    dev.min_delta_ticks = min_delta;
    dev.max_delta_ticks = max_delta;
    clockevents_config(dev, freq);
    clockevents_register_device(dev);
    }
    EXPORT_SYMBOL_GPL(clockevents_config_and_register);
#[no_mangle]
pub unsafe extern "C" fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int {
    clockevents_config(dev, freq);
    if (clockevent_state_oneshot(dev)) {
    return clockevents_program_event(dev, dev.next_event, false);
    }
    if (clockevent_state_periodic(dev)) {
    return __clockevents_switch_state(dev, CLOCK_EVT_STATE_PERIODIC);
    }
    return 0;
    }
//
// clockevents_update_freq - Update frequency and reprogram a clock event device.
// @dev:	device to modify
// @freq:	new device frequency
//
// Reconfigure and reprogram a clock event device in oneshot
// mode. Must be called on the cpu for which the device delivers per
// cpu timer events. If called for the broadcast device the core takes
// care of serialization.
//
// Returns 0 on success, -ETIME when the event is in the past.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    local_irq_save(flags);
    ret = tick_broadcast_update_freq(dev, freq);
    if (ret == -ENODEV) {
    ret = __clockevents_update_freq(dev, freq);
    }
    local_irq_restore(flags);
    return ret;
    }
//
// Noop handler when we shut down an event device
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_handle_noop(dev: *mut clock_event_device) {
    }
//
// clockevents_exchange_device - release and request clock devices
// @old:	device to release (can be NULL)
// @new:	device to request (can be NULL)
//
// Called from various tick functions with clockevents_lock held and
// interrupts disabled.
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_exchange_device(old: *mut clock_event_device, new: *mut clock_event_device) {
//
// Caller releases a clock event device. We queue it into the
// released list and do a notify add later.
//
    if (old) {
    module_put!(old.owner);
    clockevents_switch_state(old, CLOCK_EVT_STATE_DETACHED);
    list_move(&old.list, &clockevents_released);
    }
    if (new) {
    BUG_ON!(!clockevent_state_detached(new));
    clockevents_shutdown(new);
    }
    }
//
// clockevents_suspend - suspend clock devices
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_suspend() {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_reverse(dev, &clockevent_devices, list) {
    if (dev.suspend && !clockevent_state_detached(dev))
    dev.suspend(dev);
    }
    }
//
// clockevents_resume - resume clock devices
//
#[no_mangle]
pub unsafe extern "C" fn clockevents_resume() {
pub static mut dev: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(dev, &clockevent_devices, list) {
    if (dev.resume && !clockevent_state_detached(dev))
    dev.resume(dev);
    }
    }

//
// tick_offline_cpu - Shutdown all clock events related
// to this CPU and take it out of the
// broadcast mechanism.
// @cpu:	The outgoing CPU
//
// Called by the dying CPU during teardown.
//
#[no_mangle]
pub unsafe extern "C" fn tick_offline_cpu(cpu: c_uint) {
    let mut dev = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    raw_spin_lock(&clockevents_lock);
    tick_broadcast_offline(cpu);
    tick_shutdown();
//
// Unregister the clock event devices which were
// released above.
//
    list_for_each_entry_safe(dev, tmp, &clockevents_released, list) {
    list_del(&dev.list);
    }
//
// Now check whether the CPU has left unused per cpu devices
//
    list_for_each_entry_safe(dev, tmp, &clockevent_devices, list) {
    if (cpumask_test_cpu(cpu, dev.cpumask) &&
    cpumask_weight(dev.cpumask) == 1 &&
    !tick_is_broadcast_device(dev)) {
    BUG_ON!(!clockevent_state_detached(dev));
    list_del(&dev.list);
    }
    }
    raw_spin_unlock(&clockevents_lock);
    }

pub static mut bus_type: usize = 0;
pub static mut struct device: usize = 0;
// forward_decl: tick_get_tick_dev;
#[no_mangle]
pub unsafe extern "C" fn current_device_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
pub static mut td: *mut c_void = core::ptr::null_mut();
pub static mut count: isize = 0;
    raw_spin_lock_irq(&clockevents_lock);
    td = tick_get_tick_dev(dev);
    if (td && td.evtdev) {
    count = sysfs_emit(buf, "%s\n", td.evtdev.name);
    }
    raw_spin_unlock_irq(&clockevents_lock);
    return count;
    }
    static DEVICE_ATTR_RO(current_device);
// We don't support the abomination of removable broadcast devices
#[no_mangle]
pub unsafe extern "C" fn unbind_device_store(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    char name[CS_NAME_LEN];
pub static mut ret: isize = 0;
    let mut ce = core::ptr::null_mut(), *iter;
    if (ret < 0) {
    return ret;
    }
    ret = -ENODEV;
    mutex_lock(&clockevents_mutex);
    raw_spin_lock_irq(&clockevents_lock);
    list_for_each_entry(iter, &clockevent_devices, list) {
    if (!strcmp(iter.name, name)) {
    ret = __clockevents_try_unbind(iter, dev.id);
    ce = iter;
    break;
    }
    }
    raw_spin_unlock_irq(&clockevents_lock);
//
// We hold clockevents_mutex, so ce can't go away
//
    if (ret == -EAGAIN) {
    ret = clockevents_unbind(ce, dev.id);
    }
    mutex_unlock(&clockevents_mutex);
    return ret ? ret : count;
    }
    static DEVICE_ATTR_WO(unbind_device);

pub static mut device: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn tick_get_tick_dev(dev: *mut device) -> *mut c_void {
    return dev == &tick_bc_dev ? tick_get_broadcast_device() :
    &per_cpu(tick_cpu_device, dev.id);
    }
#[no_mangle]
unsafe extern "C" fn tick_broadcast_init_sysfs() -> __init int {
pub static mut err: c_int = 0;
    if (!err) {
    err = device_create_file(&tick_bc_dev, &dev_attr_current_device);
    }
    return err;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: tick_get_tick_dev
pub unsafe extern "C" fn tick_get_tick_dev_dup(dev: *mut device) -> *mut c_void {
    return &per_cpu(tick_cpu_device, dev.id);
    }
#[no_mangle]
pub unsafe extern "C" fn tick_broadcast_init_sysfs() -> c_int { return 0; }

#[no_mangle]
unsafe extern "C" fn tick_init_sysfs() -> c_int {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    let mut dev = &per_cpu(tick_percpu_dev, cpu);
    let mut err = 0;
    dev.id = cpu;
    dev.bus = &clockevents_subsys;
    err = device_register(dev);
    if (!err) {
    err = device_create_file(dev, &dev_attr_current_device);
    }
    if (!err) {
    err = device_create_file(dev, &dev_attr_unbind_device);
    }
    if (err) {
    return err;
    }
    }
    return tick_broadcast_init_sysfs();
    }
#[no_mangle]
unsafe extern "C" fn clockevents_init_sysfs() -> c_int {
pub static mut err: c_int = 0;
    if (!err) {
    err = tick_init_sysfs();
    }
    return err;
    }
    device_initcall!(clockevents_init_sysfs);