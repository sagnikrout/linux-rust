//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/tick-internal.h
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
// tick internal variable and functions used by low/high res code
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_events {
    pub local: u64,
    pub global: u64,
}

extern "C" {
    pub fn tick_setup_periodic(dev: *mut clock_event_device, broadcast: c_int);
}
extern "C" {
    pub fn tick_handle_periodic(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_check_new_device(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_offline_cpu(cpu: c_uint);
}
extern "C" {
    pub fn tick_shutdown();
}
extern "C" {
    pub fn tick_suspend();
}
extern "C" {
    pub fn tick_resume();
}
extern "C" {
    pub fn tick_install_replacement(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_is_oneshot_available() -> c_int;
}
extern "C" {
    pub fn clockevents_tick_resume(dev: *mut clock_event_device) -> c_int;
}
// Check, if the device is functional or a dummy for broadcast
extern "C" {
    pub fn clockevents_shutdown(dev: *mut clock_event_device);
}
extern "C" {
    pub fn clockevents_handle_noop(dev: *mut clock_event_device);
}
extern "C" {
    pub fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int;
}
// Broadcasting support

extern "C" {
    pub fn tick_device_uses_broadcast(dev: *mut clock_event_device, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn tick_install_broadcast_device(dev: *mut clock_event_device, cpu: c_int);
}
extern "C" {
    pub fn tick_is_broadcast_device(dev: *mut clock_event_device) -> c_int;
}
extern "C" {
    pub fn tick_suspend_broadcast();
}
extern "C" {
    pub fn tick_resume_broadcast();
}
extern "C" {
    pub fn tick_resume_check_broadcast() -> bool;
}
extern "C" {
    pub fn tick_broadcast_init();
}
extern "C" {
    pub fn tick_set_periodic_handler(dev: *mut clock_event_device, broadcast: c_int);
}
extern "C" {
    pub fn tick_broadcast_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int;
}

// Set the periodic handler in non broadcast mode

// Oneshot related functions

extern "C" {
    pub fn tick_program_event(expires: ktime_t, force: c_int) -> c_int;
}
extern "C" {
    pub fn tick_oneshot_notify();
}
extern "C" {
    pub fn tick_switch_to_oneshot(): *mut *mut c_void (handler)(clock_event_device) -> c_int;
}
extern "C" {
    pub fn tick_resume_oneshot();
}
extern "C" {
    pub fn tick_oneshot_mode_active() -> c_int;
}
extern "C" {
    pub fn tick_clock_notify();
}
extern "C" {
    pub fn tick_check_oneshot_change(allow_nohz: c_int) -> c_int;
}
extern "C" {
    pub fn tick_init_highres() -> c_int;
}

// Functions related to oneshot broadcasting

extern "C" {
    pub fn tick_broadcast_switch_to_oneshot();
}
extern "C" {
    pub fn tick_broadcast_oneshot_active() -> c_int;
}
extern "C" {
    pub fn tick_check_oneshot_broadcast_this_cpu();
}
extern "C" {
    pub fn tick_broadcast_oneshot_available() -> bool;
}

extern "C" {
    pub fn tick_broadcast_offline(cpu: c_uint);
}

// NO_HZ_FULL internal

extern "C" {
    pub fn tick_nohz_init();
}

extern "C" {
    pub fn timers_update_nohz();
}
extern "C" {
    pub fn get_jiffies_update(basej: *mut c_ulong) -> u64;
}

extern "C" {
    pub fn timer_lock_remote_bases(cpu: c_uint);
}
extern "C" {
    pub fn timer_unlock_remote_bases(cpu: c_uint);
}
extern "C" {
    pub fn timer_base_is_idle() -> bool;
}
extern "C" {
    pub fn timer_expire_remote(cpu: c_uint);
}

extern "C" {
    pub fn get_next_timer_interrupt(basej: c_ulong, basem: u64) -> u64;
}
extern "C" {
    pub fn timer_base_try_to_set_idle(basej: c_ulong, basem: u64, idle: *mut bool) -> u64;
}
extern "C" {
    pub fn timer_clear_idle();
}

extern "C" {
    pub fn clock_was_set(bases: c_uint);
}
extern "C" {
    pub fn clock_was_set_delayed();
}
extern "C" {
    pub fn hrtimers_resume_local();
}
// Since jiffies uses a simple TICK_NSEC multiplier
// conversion, the .shift value could be zero. However
// this would make NTP adjustments impossible as they are
// in units of 1/2^.shift. Thus we use JIFFIES_SHIFT to
// shift both the nominator and denominator the same
// amount, and give ntp adjustments in units of 1/2^8
//
// The value 8 is somewhat carefully chosen, as anything
// larger can result in overflows. TICK_NSEC grows as HZ
// shrinks, so values greater than 8 overflow 32bits when
// HZ=100.
//

pub const JIFFIES_SHIFT: c_int = 6;

pub const JIFFIES_SHIFT: c_int = 7;

pub const JIFFIES_SHIFT: c_int = 8;

extern "C" {
    pub fn sysfs_get_uname(buf: *const c_char, dst: *mut c_char, cnt: usize) -> isize;
}