//! Automatically rewritten from C to Rust
//! Source: kernel/time/vsyscall.c
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
// Copyright 2019 ARM Ltd.
//
// Generic implementation of update_vsyscall and update_vsyscall_tz.
//
// Based on the x86 specific implementation.
//

#[no_mangle]
pub unsafe extern "C" fn fill_clock_configuration(vc: *mut vdso_clock, base: *const tk_read_base) {
    vc.cycle_last	= base.cycle_last;

    vc.max_cycles	= base.clock.max_cycles;

    vc.mask	= base.mask;
    vc.mult	= base.mult;
    vc.shift	= base.shift;
    }
#[no_mangle]
pub unsafe extern "C" fn update_vdso_time_data(vdata: *mut vdso_time_data, tk: *mut timekeeper) {
    let mut vc = vdata.clock_data;
pub static mut vdso_ts: *mut c_void = core::ptr::null_mut();
    u64 nsec, sec;
    fill_clock_configuration(&vc[CS_HRES_COARSE],	&tk.tkr_mono);
    fill_clock_configuration(&vc[CS_RAW],		&tk.tkr_raw);
// CLOCK_MONOTONIC
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_MONOTONIC];
    vdso_ts.sec	= tk.xtime_sec + tk.wall_to_monotonic.tv_sec;
    nsec = tk.tkr_mono.xtime_nsec;
    nsec += ((u64)tk.wall_to_monotonic.tv_nsec << tk.tkr_mono.shift);
    while (nsec >= (((u64)NSEC_PER_SEC) << tk.tkr_mono.shift)) {
    nsec -= (((u64)NSEC_PER_SEC) << tk.tkr_mono.shift);
    vdso_ts.sec += 1;
    }
    vdso_ts.nsec	= nsec;
// Copy MONOTONIC time for BOOTTIME
    sec	= vdso_ts.sec;
// Add the boot offset
    sec	+= tk.monotonic_to_boot.tv_sec;
    nsec	+= (u64)tk.monotonic_to_boot.tv_nsec << tk.tkr_mono.shift;
// CLOCK_BOOTTIME
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_BOOTTIME];
    vdso_ts.sec	= sec;
    while (nsec >= (((u64)NSEC_PER_SEC) << tk.tkr_mono.shift)) {
    nsec -= (((u64)NSEC_PER_SEC) << tk.tkr_mono.shift);
    vdso_ts.sec += 1;
    }
    vdso_ts.nsec	= nsec;
// CLOCK_MONOTONIC_RAW
    vdso_ts		= &vc[CS_RAW].basetime[CLOCK_MONOTONIC_RAW];
    vdso_ts.sec	= tk.raw_sec;
    vdso_ts.nsec	= tk.tkr_raw.xtime_nsec;
// CLOCK_TAI
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_TAI];
    vdso_ts.sec	= tk.xtime_sec + (s64)tk.tai_offset;
    vdso_ts.nsec	= tk.tkr_mono.xtime_nsec;
    }
#[no_mangle]
pub unsafe extern "C" fn update_vsyscall(tk: *mut timekeeper) {
    let mut vdata = vdso_k_time_data;
    let mut vc = vdata.clock_data;
pub static mut vdso_ts: *mut c_void = core::ptr::null_mut();
    let mut clock_mode = 0;
    let mut nsec = 0;
// copy vsyscall data
    vdso_write_begin(vdata);
    clock_mode = tk.tkr_mono.clock.vdso_clock_mode;
    vc[CS_HRES_COARSE].clock_mode	= clock_mode;
    vc[CS_RAW].clock_mode		= clock_mode;
// CLOCK_REALTIME also required for time()
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_REALTIME];
    vdso_ts.sec	= tk.xtime_sec;
    vdso_ts.nsec	= tk.tkr_mono.xtime_nsec;
// CLOCK_REALTIME_COARSE
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_REALTIME_COARSE];
    vdso_ts.sec	= tk.xtime_sec;
    vdso_ts.nsec	= tk.coarse_nsec;
// CLOCK_MONOTONIC_COARSE
    vdso_ts		= &vc[CS_HRES_COARSE].basetime[CLOCK_MONOTONIC_COARSE];
    vdso_ts.sec	= tk.xtime_sec + tk.wall_to_monotonic.tv_sec;
    nsec		= tk.coarse_nsec;
    nsec		= nsec + tk.wall_to_monotonic.tv_nsec;
    vdso_ts.sec	+= __iter_div_u64_rem(nsec, NSEC_PER_SEC, &vdso_ts.nsec);
//
// Read without the seqlock held by clock_getres().
//
    WRITE_ONCE(vdata.hrtimer_res, hrtimer_resolution);
//
// If the current clocksource is not VDSO capable, then spare the
// update of the high resolution parts.
//
    if (clock_mode != VDSO_CLOCKMODE_NONE) {
    update_vdso_time_data(vdata, tk);
    }
    __arch_update_vdso_clock(&vc[CS_HRES_COARSE]);
    __arch_update_vdso_clock(&vc[CS_RAW]);
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
    }
#[no_mangle]
pub unsafe extern "C" fn update_vsyscall_tz() {
    let mut vdata = vdso_k_time_data;
    vdata.tz_minuteswest = sys_tz.tz_minuteswest;
    vdata.tz_dsttime = sys_tz.tz_dsttime;
    __arch_sync_vdso_time_data(vdata);
    }

#[no_mangle]
pub unsafe extern "C" fn vdso_time_update_aux(tk: *mut timekeeper) {
    let mut vdata = vdso_k_time_data;
pub static mut vdso_ts: *mut c_void = core::ptr::null_mut();
pub static mut vc: *mut c_void = core::ptr::null_mut();
    let mut clock_mode = 0;
    let mut nsec = 0;
    vc = &vdata.aux_clock_data[tk.id - TIMEKEEPER_AUX_FIRST];
    vdso_ts = &vc.basetime[VDSO_BASE_AUX];
    clock_mode = tk.tkr_mono.clock.vdso_clock_mode;
    if (!tk.clock_valid) {
    clock_mode = VDSO_CLOCKMODE_NONE;
    }
// copy vsyscall data
    vdso_write_begin_clock(vc);
    vc.clock_mode = clock_mode;
    if (clock_mode != VDSO_CLOCKMODE_NONE) {
    fill_clock_configuration(vc, &tk.tkr_mono);
    vdso_ts.sec = tk.xtime_sec + tk.monotonic_to_aux.tv_sec;
    nsec = tk.tkr_mono.xtime_nsec >> tk.tkr_mono.shift;
    nsec += tk.monotonic_to_aux.tv_nsec;
    vdso_ts.sec += __iter_div_u64_rem(nsec, NSEC_PER_SEC, &nsec);
    nsec = nsec << tk.tkr_mono.shift;
    vdso_ts.nsec = nsec;
    }
    __arch_update_vdso_clock(vc);
    vdso_write_end_clock(vc);
    __arch_sync_vdso_time_data(vdata);
    }

//
// vdso_update_begin - Start of a VDSO update section
//
// Allows architecture code to safely update the architecture specific VDSO
// data. Disables interrupts, acquires timekeeper lock to serialize against
// concurrent updates from timekeeping and invalidates the VDSO data
// sequence counter to prevent concurrent readers from accessing
// inconsistent data.
//
// Returns: Saved interrupt flags which need to be handed in to
// vdso_update_end().
//
#[no_mangle]
pub unsafe extern "C" fn vdso_update_begin() -> c_ulong {
    let mut vdata = vdso_k_time_data;
pub static mut flags: c_ulong = 0;
    vdso_write_begin(vdata);
    return flags;
    }
//
// vdso_update_end - End of a VDSO update section
// @flags:	Interrupt flags as returned from vdso_update_begin()
//
// Pairs with vdso_update_begin(). Marks vdso data consistent, invokes data
// synchronization if the architecture requires it, drops timekeeper lock
// and restores interrupt flags.
//
#[no_mangle]
pub unsafe extern "C" fn vdso_update_end(flags: c_ulong) {
    let mut vdata = vdso_k_time_data;
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
    timekeeper_unlock_irqrestore(flags);
    }