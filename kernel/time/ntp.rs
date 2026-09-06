//! Automatically rewritten from C to Rust
//! Source: kernel/time/ntp.c
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
// NTP state machine interfaces and logic.
//
// This code was mainly moved from kernel/timer.c and kernel/time.c
// Please see those files for relevant copyright info and historical
// changelogs.
//

//
// struct ntp_data - Structure holding all NTP related state
// @tick_usec:		USER_HZ period in microseconds
// @tick_length:	Tick length in ns << NTP_SCALE_SHIFT
// @time_state:		State of the clock synchronization
// @time_status:	Clock status bits
// @time_offset:	Time adjustment in nanoseconds
// @skew_delta:		Per-tick phase slew rate for the coming second, in
// @time_offset units (shifted-ns / HZ). Set by
// second_overflow().
// @time_constant:	PLL time constant
// @time_maxerror:	Maximum error in microseconds holding the NTP sync distance
// (NTP dispersion + delay / 2)
// @time_esterror:	Estimated error in microseconds holding NTP dispersion
// @time_freq:		Frequency offset scaled nsecs/secs
// @time_reftime:	Time at last adjustment in seconds
// @time_adjust:	Adjustment value
// @time_adjust_frac:	Sub-microsecond remainder of @time_adjust being
// delivered, in ns << NTP_SCALE_SHIFT (not divided by HZ).
// @ntp_tick_adj:	Constant boot-param configurable NTP tick adjustment (upscaled)
// @cs_tick_adj:	Fixed per-second adjustment compensating for the difference
// between the nominal NTP interval and the real time taken
// by the clocksource's integer @cycle_interval (upscaled).
// Set by the timekeeping core via ntp_clear().
// @ntp_next_leap_sec:	Second value of the next pending leapsecond, or TIME64_MAX if no leap
//
// @pps_valid:		PPS signal watchdog counter
// @pps_tf:		PPS phase median filter
// @pps_jitter:		PPS current jitter in nanoseconds
// @pps_fbase:		PPS beginning of the last freq interval
// @pps_shift:		PPS current interval duration in seconds (shift value)
// @pps_intcnt:		PPS interval counter
// @pps_freq:		PPS frequency offset in scaled ns/s
// @pps_stabil:		PPS current stability in scaled ns/s
// @pps_calcnt:		PPS monitor: calibration intervals
// @pps_jitcnt:		PPS monitor: jitter limit exceeded
// @pps_stbcnt:		PPS monitor: stability limit exceeded
// @pps_errcnt:		PPS monitor: calibration errors
//
// Protected by the timekeeping locks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntp_data {
    pub tick_usec: c_ulong,
    pub tick_length: u64,
    pub time_state: c_int,
    pub time_status: c_int,
    pub time_offset: i64,
    pub skew_delta: i64,
    pub time_constant: c_long,
    pub time_maxerror: c_long,
    pub time_esterror: c_long,
    pub time_freq: i64,
    pub time_reftime: time64_t,
    pub time_adjust: c_long,
    pub time_adjust_frac: i64,
    pub ntp_tick_adj: i64,
    pub cs_tick_adj: i64,
    pub ntp_next_leap_sec: time64_t,

    pub pps_valid: c_int,
    pub pps_tf: [c_long; 3],
    pub pps_jitter: c_long,
    pub pps_fbase: timespec64,
    pub pps_shift: c_int,
    pub pps_intcnt: c_int,
    pub pps_freq: i64,
    pub pps_stabil: c_long,
    pub pps_calcnt: c_long,
    pub pps_jitcnt: c_long,
    pub pps_stbcnt: c_long,
    pub pps_errcnt: c_long,

}

pub static mut ntp_data: usize = 0;
pub const SECS_PER_DAY: c_int = 86400;

// One microsecond of phase, in plain shifted-ns (ns << NTP_SCALE_SHIFT)

// Per-tick MAX_TICKADJ slew, in plain shifted-ns

    (((MAX_TICKADJ * NSEC_PER_USEC) << NTP_SCALE_SHIFT) / NTP_INTERVAL_FREQ)
pub const MAX_TAI_OFFSET: c_int = 100000;

//
// The following variables are used when a pulse-per-second (PPS) signal
// is available. They establish the engineering parameters of the clock
// discipline loop when controlled by the PPS signal.
//

    increase pps_shift or consecutive bad
    intervals to decrease it */

//
// PPS kernel consumer compensates the whole phase error immediately.
// Otherwise, reduce the offset by a fixed factor times the time constant.
//
#[no_mangle]
pub unsafe extern "C" fn ntp_offset_chunk(ntpdata: *mut ntp_data, offset: i64) -> i64 {
    if (ntpdata.time_status & STA_PPSTIME && ntpdata.time_status & STA_PPSSIGNAL) {
    return offset;
    }
    else {
    return shift_right(offset, SHIFT_PLL + ntpdata.time_constant);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pps_reset_freq_interval(ntpdata: *mut ntp_data) {
// The PPS calibration interval may end surprisingly early
    ntpdata.pps_shift = PPS_INTMIN;
    ntpdata.pps_intcnt = 0;
    }
//
// pps_clear - Clears the PPS state variables
// @ntpdata:	Pointer to ntp data
//
#[no_mangle]
pub unsafe extern "C" fn pps_clear(ntpdata: *mut ntp_data) {
    pps_reset_freq_interval(ntpdata);
    ntpdata.pps_tf[0] = 0;
    ntpdata.pps_tf[1] = 0;
    ntpdata.pps_tf[2] = 0;
    ntpdata.pps_fbase.tv_sec = ntpdata.pps_fbase.tv_nsec = 0;
    ntpdata.pps_freq = 0;
    }
//
// Decrease pps_valid to indicate that another second has passed since the
// last PPS signal. When it reaches 0, indicate that PPS signal is missing.
//
#[no_mangle]
pub unsafe extern "C" fn pps_dec_valid(ntpdata: *mut ntp_data) {
    if (ntpdata.pps_valid > 0) {
    ntpdata.pps_valid -= 1;
    } else {
    ntpdata.time_status &= ~(STA_PPSSIGNAL | STA_PPSJITTER |
    STA_PPSWANDER | STA_PPSERROR);
    pps_clear(ntpdata);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pps_set_freq(ntpdata: *mut ntp_data) {
    ntpdata.pps_freq = ntpdata.time_freq;
    }
#[no_mangle]
pub unsafe extern "C" fn is_error_status(status: c_int) -> bool {
    return (status & (STA_UNSYNC|STA_CLOCKERR))
//
// PPS signal lost when either PPS time or PPS frequency
// synchronization requested
//
    || ((status & (STA_PPSFREQ|STA_PPSTIME))
    && !(status & STA_PPSSIGNAL))
//
// PPS jitter exceeded when PPS time synchronization
// requested
//
    || ((status & (STA_PPSTIME|STA_PPSJITTER))
    == (STA_PPSTIME|STA_PPSJITTER))
//
// PPS wander exceeded or calibration error when PPS
// frequency synchronization requested
//
    || ((status & STA_PPSFREQ)
    && (status & (STA_PPSWANDER|STA_PPSERROR)));
    }
#[no_mangle]
pub unsafe extern "C" fn pps_fill_timex(ntpdata: *mut ntp_data, txc: *mut __kernel_timex) {
    txc.ppsfreq	   = shift_right((ntpdata.pps_freq >> PPM_SCALE_INV_SHIFT) *
    PPM_SCALE_INV, NTP_SCALE_SHIFT);
    txc.jitter	   = ntpdata.pps_jitter;
    if (!(ntpdata.time_status & STA_NANO)) {
    txc.jitter = ntpdata.pps_jitter / NSEC_PER_USEC;
    }
    txc.shift	   = ntpdata.pps_shift;
    txc.stabil	   = ntpdata.pps_stabil;
    txc.jitcnt	   = ntpdata.pps_jitcnt;
    txc.calcnt	   = ntpdata.pps_calcnt;
    txc.errcnt	   = ntpdata.pps_errcnt;
    txc.stbcnt	   = ntpdata.pps_stbcnt;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: ntp_offset_chunk
pub unsafe extern "C" fn ntp_offset_chunk_dup(ntpdata: *mut ntp_data, offset: i64) -> i64 {
    return shift_right(offset, SHIFT_PLL + ntpdata.time_constant);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: pps_reset_freq_interval
pub unsafe extern "C" fn pps_reset_freq_interval_dup(ntpdata: *mut ntp_data) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: pps_clear
pub unsafe extern "C" fn pps_clear_dup(ntpdata: *mut ntp_data) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: pps_dec_valid
pub unsafe extern "C" fn pps_dec_valid_dup(ntpdata: *mut ntp_data) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: pps_set_freq
pub unsafe extern "C" fn pps_set_freq_dup(ntpdata: *mut ntp_data) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: is_error_status
pub unsafe extern "C" fn is_error_status_dup(status: c_int) -> bool {
    return status & (STA_UNSYNC|STA_CLOCKERR);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: pps_fill_timex
pub unsafe extern "C" fn pps_fill_timex_dup(ntpdata: *mut ntp_data, txc: *mut __kernel_timex) {
// PPS is not implemented, so these are zero
    txc.ppsfreq	   = 0;
    txc.jitter	   = 0;
    txc.shift	   = 0;
    txc.stabil	   = 0;
    txc.jitcnt	   = 0;
    txc.calcnt	   = 0;
    txc.errcnt	   = 0;
    txc.stbcnt	   = 0;
    }

//
// Update tick_length based on tick_usec, ntp_tick_adj and time_freq:
//
#[no_mangle]
unsafe extern "C" fn ntp_update_frequency(ntpdata: *mut ntp_data) {
    u64 second_length, new_base, tick_usec = (u64)ntpdata.tick_usec;
    second_length		 = (u64)(tick_usec * NSEC_PER_USEC * USER_HZ) << NTP_SCALE_SHIFT;
    second_length		+= ntpdata.ntp_tick_adj;
    second_length		+= ntpdata.cs_tick_adj;
    second_length		+= ntpdata.time_freq;
    new_base		 = div_u64(second_length, NTP_INTERVAL_FREQ);
//
// Don't wait for the next second_overflow, apply the change to the
// tick length immediately:
//
    ntpdata.tick_length	 = new_base;
    }
#[no_mangle]
pub unsafe extern "C" fn ntp_update_offset_fll(ntpdata: *mut ntp_data, offset64: i64, secs: c_long) -> i64 {
    ntpdata.time_status &= ~STA_MODE;
    if (secs < MINSEC) {
    return 0;
    }
    if (!(ntpdata.time_status & STA_FLL) && (secs <= MAXSEC)) {
    return 0;
    }
    ntpdata.time_status |= STA_MODE;
    return div64_long(offset64 << (NTP_SCALE_SHIFT - SHIFT_FLL), secs);
    }
#[no_mangle]
unsafe extern "C" fn ntp_update_offset(ntpdata: *mut ntp_data, offset: c_long) {
    s64 freq_adj, offset64;
    let mut secs = 0;
    let mut real_secs = 0;
    if (!(ntpdata.time_status & STA_PLL)) {
    return;
    }
    if (!(ntpdata.time_status & STA_NANO)) {
// Make sure the multiplication below won't overflow
    offset = clamp(offset, -USEC_PER_SEC, USEC_PER_SEC);
    offset *= NSEC_PER_USEC;
    }
// Scale the phase adjustment and clamp to the operating range.
    offset = clamp(offset, -MAXPHASE, MAXPHASE);
//
// Select how the frequency is to be controlled
// and in which mode (PLL or FLL).
//
    real_secs = ktime_get_ntp_seconds(ntpdata - tk_ntp_data);
    secs = (long)(real_secs - ntpdata.time_reftime);
    if (unlikely(ntpdata.time_status & STA_FREQHOLD)) {
    secs = 0;
    }
    ntpdata.time_reftime = real_secs;
    offset64    = offset;
    freq_adj    = ntp_update_offset_fll(ntpdata, offset64, secs);
//
// Clamp update interval to reduce PLL gain with low
// sampling rate (e.g. intermittent network connection)
// to avoid instability.
//
    if (unlikely(secs > 1 << (SHIFT_PLL + 1 + ntpdata.time_constant))) {
    secs = 1 << (SHIFT_PLL + 1 + ntpdata.time_constant);
    }
    freq_adj    += (offset64 * secs) <<
    (NTP_SCALE_SHIFT - 2 * (SHIFT_PLL + 2 + ntpdata.time_constant));
    freq_adj    = min(freq_adj + ntpdata.time_freq, MAXFREQ_SCALED);
    ntpdata.time_freq   = max(freq_adj, -MAXFREQ_SCALED);
    ntpdata.time_offset = div_s64(offset64 << NTP_SCALE_SHIFT, NTP_INTERVAL_FREQ);
    }
#[no_mangle]
unsafe extern "C" fn __ntp_clear(ntpdata: *mut ntp_data) {
// Stop active adjtime()
    ntpdata.time_adjust	= 0;
    ntpdata.time_adjust_frac = 0;
    ntpdata.time_status	|= STA_UNSYNC;
    ntpdata.time_maxerror	= NTP_PHASE_LIMIT;
    ntpdata.time_esterror	= NTP_PHASE_LIMIT;
    ntp_update_frequency(ntpdata);
    ntpdata.time_offset	= 0;
    ntpdata.skew_delta	= 0;
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
// Clear PPS state variables
    pps_clear(ntpdata);
    }
//
// ntp_clear - Clear NTP state and set the clocksource quantisation adjustment
// @tkid:		Timekeeper ID
// @cs_tick_adj:	Per-second adjustment in ns << NTP_SCALE_SHIFT
//
// The timekeeping core uses an integer number of cycles (@cycle_interval)
// per NTP interval, so the real time that interval represents differs from
// the nominal NTP_INTERVAL_LENGTH by up to half a counter period. Folding
// this fixed offset into @cs_tick_adj makes it an explicit part of the NTP
// tick_length computation in ntp.c, instead of being applied during
// timekeeping accumulation where the NTP code never saw it. Like
// @ntp_tick_adj it stays internal to the kernel; userspace still sees the
// nominal tick via adjtimex. NTP retains its full symmetric ±MAXFREQ range
// around the corrected base rate.
//
// Called whenever the clocksource is (re)configured, which is also when the
// rest of the NTP state must be cleared, so the two are done together.
//
#[no_mangle]
pub unsafe extern "C" fn ntp_clear(tkid: c_uint, cs_tick_adj: i64) {
    tk_ntp_data[tkid].cs_tick_adj = cs_tick_adj;
    __ntp_clear(&tk_ntp_data[tkid]);
    }
#[no_mangle]
pub unsafe extern "C" fn ntp_tick_length(tkid: c_uint) -> u64 {
    return tk_ntp_data[tkid].tick_length;
    }
#[no_mangle]
pub unsafe extern "C" fn ntp_get_skew_delta(tkid: c_uint) -> i64 {
    return tk_ntp_data[tkid].skew_delta;
    }
// Sign of @x as +1 or -1 (zero counts as positive; callers pass nonzero).
#[no_mangle]
pub unsafe extern "C" fn signof(x: i64) -> c_int {
    return x < 0 ? -1 : 1;
    }
#[no_mangle]
unsafe extern "C" fn ntp_drain_time_offset(tkid: c_uint, amount: i64) -> i64 {
    let mut ntpdata = &tk_ntp_data[tkid];
// Only drain if amount and time_offset have the same sign
    if (!amount || signof(amount) != signof(ntpdata.time_offset)) {
    return amount;
    }
// Clamp: don't overshoot zero
    if (abs(amount) > abs(ntpdata.time_offset)) {
pub static mut undrained: i64 = 0;
    ntpdata.time_offset = 0;
    return undrained;
    }
    ntpdata.time_offset -= amount;
    return 0;
    }
//
// Drain the legacy adjtime() correction (time_adjust) as it is delivered.
//
// @amount is the total intentional per-tick skew for this accumulation
// (skew_delta << shift), in time_offset units (shifted_ns / HZ); it covers
// both the exponential time_offset slew and the linear adjtime slew. This
// function claims only the adjtime share — capped at the MAX_TICKADJ rate —
// and returns the remainder for ntp_drain_time_offset().
//
// time_adjust is in whole µs. The sub-µs remainder being delivered lives in
// time_adjust_frac (plain shifted-ns, i.e. ns << NTP_SCALE_SHIFT -- unlike
// time_offset these are NOT pre-divided by HZ); we top it up by borrowing
// whole microseconds from time_adjust as the drain consumes it.
//
#[no_mangle]
unsafe extern "C" fn ntp_drain_time_adjust(tkid: c_uint, amount: i64, shift: c_uint) -> i64 {
    let mut ntpdata = &tk_ntp_data[tkid];
// Sign reference: time_adjust if any whole us remain, else the drawer
    s64 ref = ntpdata.time_adjust ? (s64)ntpdata.time_adjust
    : ntpdata.time_adjust_frac;
    s64 deliver, deficit, claimed;
    if (!amount || !ref || signof(amount) != signof(ref)) {
    return amount;
    }
//
// Phase to deliver this accumulation, in plain shifted-ns. The drain
// @amount is in ÷HZ units, so multiply by HZ first, then clamp to the
// MAX_TICKADJ rate (MAX_TICKADJ_SCALED is the per-tick slew in
// shifted-ns). Multiply-then-clamp avoids an s64 divide for the cap.
//
    deliver = min(abs(amount) * NTP_INTERVAL_FREQ,
    (s64)MAX_TICKADJ_SCALED << shift);
// Top up the sub-µs drawer from whole-µs time_adjust as needed
    deficit = deliver - abs(ntpdata.time_adjust_frac);
    if (deficit > 0 && ntpdata.time_adjust) {
pub static mut borrow: c_long = 0;
    if (ntpdata.time_adjust > 0) {
    borrow = min(borrow, ntpdata.time_adjust);
    ntpdata.time_adjust	  -= borrow;
    ntpdata.time_adjust_frac += (s64)borrow * ONE_US_NS;
    } else {
// Clamp without negating time_adjust (UB for LONG_MIN)
    if (ntpdata.time_adjust > -borrow) {
    borrow = -ntpdata.time_adjust;
    }
    ntpdata.time_adjust	  += borrow;
    ntpdata.time_adjust_frac -= (s64)borrow * ONE_US_NS;
    }
    }
// Never deliver more than the drawer holds
    deliver = min(deliver, abs(ntpdata.time_adjust_frac));
    if (ntpdata.time_adjust_frac > 0) {
    ntpdata.time_adjust_frac -= deliver;
    }
    else {
    ntpdata.time_adjust_frac += deliver;
    }
// Return the unclaimed remainder in ÷HZ drain units for time_offset
    claimed = div_s64(deliver, NTP_INTERVAL_FREQ);
    return amount - signof(amount) * claimed;
    }
//
// Drain one accumulation's worth of intentional skew as it is delivered.
//
// @amount is the total intentional per-tick skew for this accumulation
// (skew_delta << shift), in time_offset units (shifted_ns / HZ). The
// adjtime() linear share is taken from time_adjust first (capped at the
// MAX_TICKADJ rate, hence @shift), then the exponential remainder from
// time_offset. Returns the amount actually claimed (same ÷HZ units).
//
#[no_mangle]
pub unsafe extern "C" fn ntp_drain_skew(tkid: c_uint, amount: i64, shift: c_uint) -> i64 {
pub static mut unclaimed: i64 = 0;
    unclaimed = ntp_drain_time_offset(tkid, unclaimed);
//
// Return the amount actually drained from the intentional
// phase offset in time_offset and/or time_adjust.
//
    return amount - unclaimed;
    }
//
// time_offset (drained exponentially) and time_adjust (drained linearly at the
// MAX_TICKADJ rate) can be asked to slew the clock in opposite directions.
// second_overflow() only folds their *net* into skew_delta, so the cancelling
// part would never be drained from either tracker via the per-tick code -- and
// if they cancel exactly, skew_delta is zero and neither converges at all.
//
// Settle that cancelling phase directly between the two here. No clock motion
// results (the opposing slews annihilate), but both move toward zero so neither
// stalls. @amount is the phase to take off time_offset, in its (÷HZ) units and
// with its sign; the same real magnitude comes off time_adjust in the opposite
// direction. Clamped so neither tracker is driven past zero.
//
#[no_mangle]
unsafe extern "C" fn ntp_transfer_offset_adjust(ntpdata: *mut ntp_data, amount: i64) {
    s64 frac_delta, carry;
//
// Don't drain time_offset past zero. @amount shares its sign and is
// normally bounded below it by ntp_offset_chunk(), but the ±1 skew_delta
// floor for a tiny time_offset can exceed it, so clamp.
//
    if (abs(amount) > abs(ntpdata.time_offset)) {
    amount = ntpdata.time_offset;
    }
    if (!amount) {
    return;
    }
//
// Remove the matching phase from time_adjust, in plain shifted-ns. No
// clamp against time_adjust's zero is needed: @amount is bounded by the
// adjtime chunk, which second_overflow() never lets exceed time_adjust's
// own pending phase, so this cannot overshoot.
//
    frac_delta = amount * NTP_INTERVAL_FREQ;
    ntpdata.time_offset -= amount;
// Add the matching phase to time_adjust, carrying whole µs (O(1)).
    ntpdata.time_adjust_frac += frac_delta;
    if (ntpdata.time_adjust_frac >= ONE_US_NS ||
    ntpdata.time_adjust_frac <= -ONE_US_NS) {
    carry = div64_s64(ntpdata.time_adjust_frac, ONE_US_NS);
    ntpdata.time_adjust	  += carry;
    ntpdata.time_adjust_frac -= carry * ONE_US_NS;
    }
//
// Keep time_adjust and its sub-µs remainder the same sign. The
// truncating carry above can leave them opposed (e.g. +4 µs paired
// with -250 ns), and ntp_drain_time_adjust() treats abs(time_adjust_frac)
// as same-direction drawer capacity -- an opposing remainder there makes
// it over-deliver phase that was never removed from the pile. Borrow or
// repay a single whole µs to realign; the total phase is unchanged.
//
    if (ntpdata.time_adjust > 0 && ntpdata.time_adjust_frac < 0) {
    ntpdata.time_adjust -= 1;
    ntpdata.time_adjust_frac += ONE_US_NS;
    } else if (ntpdata.time_adjust < 0 && ntpdata.time_adjust_frac > 0) {
    ntpdata.time_adjust += 1;
    ntpdata.time_adjust_frac -= ONE_US_NS;
    }
    }
//
// ntp_get_next_leap - Returns the next leapsecond in CLOCK_REALTIME ktime_t
// @tkid:	Timekeeper ID
//
// Returns: For @tkid == TIMEKEEPER_CORE this provides the time of the next
// leap second against CLOCK_REALTIME in a ktime_t format if a
// leap second is pending. KTIME_MAX otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ntp_get_next_leap(tkid: c_uint) -> ktime_t {
    let mut ntpdata = &tk_ntp_data[TIMEKEEPER_CORE];
    if (tkid != TIMEKEEPER_CORE) {
    return KTIME_MAX;
    }
    if ((ntpdata.time_state == TIME_INS) && (ntpdata.time_status & STA_INS)) {
    return ktime_set(ntpdata.ntp_next_leap_sec, 0);
    }
    return KTIME_MAX;
    }
//
// This routine handles the overflow of the microsecond field
//
// The tricky bits of code to handle the accurate clock support
// were provided by Dave Mills (Mills@UDEL.EDU) of NTP fame.
// They were originally developed for SUN and DEC kernels.
// All the kudos should go to Dave for this stuff.
//
// Also handles leap second processing, and returns leap offset
//
#[no_mangle]
pub unsafe extern "C" fn second_overflow(tkid: c_uint, secs: time64_t) -> c_int {
    let mut ntpdata = &tk_ntp_data[tkid];
pub static mut leap: c_int = 0;
    let mut rem = 0;
//
// Leap second processing. If in leap-insert state at the end of the
// day, the system clock is set back one second; if in leap-delete
// state, the system clock is set ahead one second.
//
    match (ntpdata.time_state) {
    TIME_OK => {
    if (ntpdata.time_status & STA_INS) {
    ntpdata.time_state = TIME_INS;
    div_s64_rem(secs, SECS_PER_DAY, &rem);
    ntpdata.ntp_next_leap_sec = secs + SECS_PER_DAY - rem;
    } else if (ntpdata.time_status & STA_DEL) {
    ntpdata.time_state = TIME_DEL;
    div_s64_rem(secs + 1, SECS_PER_DAY, &rem);
    ntpdata.ntp_next_leap_sec = secs + SECS_PER_DAY - rem;
    }
    // break;
    }
    TIME_INS => {
    if (!(ntpdata.time_status & STA_INS)) {
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
    ntpdata.time_state = TIME_OK;
    } else if (secs == ntpdata.ntp_next_leap_sec) {
    leap = -1;
    ntpdata.time_state = TIME_OOP;
    pr_notice("Clock: inserting leap second 23:59:60 UTC\n");
    }
    // break;
    }
    TIME_DEL => {
    if (!(ntpdata.time_status & STA_DEL)) {
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
    ntpdata.time_state = TIME_OK;
    } else if (secs == ntpdata.ntp_next_leap_sec) {
    leap = 1;
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
    ntpdata.time_state = TIME_WAIT;
    pr_notice("Clock: deleting leap second 23:59:59 UTC\n");
    }
    // break;
    }
    TIME_OOP => {
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
    ntpdata.time_state = TIME_WAIT;
    // break;
    }
    TIME_WAIT => {
    if (!(ntpdata.time_status & (STA_INS | STA_DEL))) {
    ntpdata.time_state = TIME_OK;
    }
    // break;
    }
    }
// Bump the maxerror field
    ntpdata.time_maxerror += MAXFREQ / NSEC_PER_USEC;
    if (ntpdata.time_maxerror > NTP_PHASE_LIMIT) {
    ntpdata.time_maxerror = NTP_PHASE_LIMIT;
    ntpdata.time_status |= STA_UNSYNC;
    }
// Compute the phase adjustment for the next second
// Check PPS signal
    pps_dec_valid(ntpdata);
//
// Set the per-tick skew rate for the next second. This is in
// the same units as time_offset: (ns << NTP_SCALE_SHIFT) / HZ.
// If the result is so low that the skew imparted would round
// to zero, pass the bare minimum ±1 to ensure that it *does
// actually drain completely to zero. It won't overshoot because
// logarithmic_accumulation() only drains what it can from
// time_offset or time_adjust, and the rest ends up in ntp_error
// which drives the selection of 'mult' immediately each tick.
//
    if (ntpdata.time_offset || ntpdata.time_adjust ||
    ntpdata.time_adjust_frac) {
pub static mut off_chunk: i64 = 0;
pub static mut adj_chunk: i64 = 0;
//
// Once the exponential chunk rounds to zero, deliver the last
// remaining offset this second so it converges to zero instead
// of stalling just above it.
//
    if (!off_chunk) {
    off_chunk = ntpdata.time_offset;
    }
    if (ntpdata.time_adjust || ntpdata.time_adjust_frac) {
    let mut adj = 0;
    if (ntpdata.time_adjust >= MAX_TICKADJ) {
    adj = MAX_TICKADJ * ONE_US_NS;
    }

    else if (ntpdata.time_adjust <= -MAX_TICKADJ) {
    adj = -MAX_TICKADJ * ONE_US_NS;
    }
    else {
    adj = ntpdata.time_adjust * ONE_US_NS +
    ntpdata.time_adjust_frac;
    }
    adj_chunk = div_s64(adj, NTP_INTERVAL_FREQ);
    if (!adj_chunk) {
    adj_chunk = signof(ntpdata.time_adjust_frac);
    }
    }
//
// If the two slews oppose, only their net would drive the
// per-tick drain, so the cancelling part would never drain from
// either tracker and an exact cancellation would stall both.
// Settle that overlap directly between them (no clock motion).
//
    if (off_chunk && adj_chunk && signof(off_chunk) != signof(adj_chunk)) {
pub static mut conflict: i64 = 0;
    ntp_transfer_offset_adjust(ntpdata, signof(off_chunk) * conflict);
    }
// Net is what the clock delivers; reduce to per-tick, then floor.
    net = off_chunk + adj_chunk;
    ntpdata.skew_delta = div_s64(net, NTP_INTERVAL_FREQ);
    if (!ntpdata.skew_delta && net) {
    ntpdata.skew_delta = signof(net);
    }
    } else {
    ntpdata.skew_delta = 0;
    }
    return leap;
    }

// forward_decl: sync_hw_clock;
pub static mut sync_work: usize = 0;
pub static mut sync_hrtimer: usize = 0;

#[no_mangle]
unsafe extern "C" fn sync_timer_callback(timer: *mut hrtimer) -> enum hrtimer_restart {
    queue_work(system_freezable_power_efficient_wq, &sync_work);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn sched_sync_hw_clock(offset_nsec: c_ulong, retry: bool) {
pub static mut exp: ktime_t = 0;
    if (retry) {
    exp = ktime_add_ns(exp, 2ULL * NSEC_PER_SEC - offset_nsec);
    }
    else {
    exp = ktime_add_ns(exp, SYNC_PERIOD_NS - offset_nsec);
    }
    hrtimer_start(&sync_hrtimer, exp, HRTIMER_MODE_ABS);
    }
//
// Check whether @now is correct versus the required time to update the RTC
// and calculate the value which needs to be written to the RTC so that the
// next seconds increment of the RTC after the write is aligned with the next
// seconds increment of clock REALTIME.
//
// tsched     t1 write(t2.tv_sec - 1sec))	t2 RTC increments seconds
//
// t2.tv_nsec == 0
// tsched = t2 - set_offset_nsec
// newval = t2 - NSEC_PER_SEC
//
// ==> neval = tsched + set_offset_nsec - NSEC_PER_SEC
//
// As the execution of this code is not guaranteed to happen exactly at
// tsched this allows it to happen within a fuzzy region:
//
// abs(now - tsched) < FUZZ
//
// If @now is not inside the allowed window the function returns false.
//
#[no_mangle]
pub unsafe extern "C" fn rtc_tv_nsec_ok(set_offset_nsec: c_ulong, to_set: *mut timespec64, now: *mut timespec64) -> bool {
// Allowed error in tv_nsec, arbitrarily set to 5 jiffies in ns.
pub static mut TIME_SET_NSEC_FUZZ: c_ulong = 0;
pub static mut timespec64: usize = 0;
// to_set = timespec64_add(*now, delay);
    if (to_set.tv_nsec < TIME_SET_NSEC_FUZZ) {
    to_set.tv_nsec = 0;
    return true;
    }
    if (to_set.tv_nsec > NSEC_PER_SEC - TIME_SET_NSEC_FUZZ) {
    to_set.tv_sec += 1;
    to_set.tv_nsec = 0;
    return true;
    }
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn update_persistent_clock64(now64: timespec64) -> int __weak {
    return -ENODEV;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: update_persistent_clock64
pub unsafe extern "C" fn update_persistent_clock64_dup(now64: timespec64) -> c_int {
    return -ENODEV;
    }

// Save NTP synchronized time to the RTC
#[no_mangle]
unsafe extern "C" fn update_rtc(to_set: *mut timespec64, offset_nsec: *mut c_ulong) -> c_int {
pub static mut rtc: *mut c_void = core::ptr::null_mut();
pub static mut tm: usize = 0;
pub static mut err: c_int = 0;
    rtc = rtc_class_open(CONFIG_RTC_SYSTOHC_DEVICE);
    if (!rtc) {
    return -ENODEV;
    }
    if (!rtc.ops || !rtc.ops.set_time) {
// goto;
    }
// First call might not have the correct offset
    if (*offset_nsec == rtc.set_offset_nsec) {
    rtc_time64_to_tm(to_set.tv_sec, &tm);
    err = rtc_set_time(rtc, &tm);
    } else {
// Store the update offset and let the caller try again
// offset_nsec = rtc->set_offset_nsec;
    err = -EAGAIN;
    }
// label;
    rtc_class_close(rtc);
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn update_rtc(to_set: *mut timespec64, offset_nsec: *mut c_ulong) -> c_int {
    return -ENODEV;
    }

//
// ntp_synced - Tells whether the NTP status is not UNSYNC
// Returns:	true if not UNSYNC, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn ntp_synced() -> bool {
    return !(tk_ntp_data[TIMEKEEPER_CORE].time_status & STA_UNSYNC);
    }
//
// If we have an externally synchronized Linux clock, then update RTC clock
// accordingly every ~11 minutes. Generally RTCs can only store second
// precision, but many RTCs will adjust the phase of their second tick to
// match the moment of update. This infrastructure arranges to call to the RTC
// set at the correct moment to phase synchronize the RTC second tick over
// with the kernel clock.
//
#[no_mangle]
unsafe extern "C" fn sync_hw_clock(work: *mut work_struct) {
//
// The default synchronization offset is 500ms for the deprecated
// update_persistent_clock64() under the assumption that it uses
// the infamous CMOS clock (MC146818).
//
pub static mut offset_nsec: unsigned long = 0;
    struct timespec64 now, to_set;
pub static mut res: c_int = 0;
//
// Don't update if STA_UNSYNC is set and if ntp_notify_cmos_timer()
// managed to schedule the work between the timer firing and the
// work being able to rearm the timer. Wait for the timer to expire.
//
    if (!ntp_synced() || hrtimer_is_queued(&sync_hrtimer)) {
    return;
    }
    ktime_get_real_ts64(&now);
// If @now is not in the allowed window, try again
    if (!rtc_tv_nsec_ok(offset_nsec, &to_set, &now)) {
// goto;
    }
// Take timezone adjusted RTCs into account
    if (persistent_clock_is_local) {
    to_set.tv_sec -= (sys_tz.tz_minuteswest * 60);
    }
// Try the legacy RTC first.
    res = update_persistent_clock64(to_set);
    if (res != -ENODEV) {
// goto;
    }
// Try the RTC class
    res = update_rtc(&to_set, &offset_nsec);
    if (res == -ENODEV) {
    return;
    }
// label;
    sched_sync_hw_clock(offset_nsec, res != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn ntp_notify_cmos_timer(offset_set: bool) {
//
// If the time jumped (using ADJ_SETOFFSET) cancels sync timer,
// which may have been running if the time was synchronized
// prior to the ADJ_SETOFFSET call.
//
    if (offset_set) {
    hrtimer_cancel(&sync_hrtimer);
    }
//
// When the work is currently executed but has not yet the timer
// rearmed this queues the work immediately again. No big issue,
// just a pointless work scheduled.
//
    if (ntp_synced() && !hrtimer_is_queued(&sync_hrtimer)) {
    queue_work(system_freezable_power_efficient_wq, &sync_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn ntp_init_cmos_sync()  {
    hrtimer_setup(&sync_hrtimer, sync_timer_callback, CLOCK_REALTIME, HRTIMER_MODE_ABS);
    }

    static inline void __init ntp_init_cmos_sync(void) { }

//
// Propagate a new txc->status value into the NTP state:
//
#[no_mangle]
pub unsafe extern "C" fn process_adj_status(ntpdata: *mut ntp_data, txc: *const __kernel_timex) {
    if ((ntpdata.time_status & STA_PLL) && !(txc.status & STA_PLL)) {
    ntpdata.time_state = TIME_OK;
    ntpdata.time_status = STA_UNSYNC;
    ntpdata.ntp_next_leap_sec = TIME64_MAX;
// Restart PPS frequency calibration
    pps_reset_freq_interval(ntpdata);
    }
//
// If we turn on PLL adjustments then reset the
// reference time to current time.
//
    if (!(ntpdata.time_status & STA_PLL) && (txc.status & STA_PLL)) {
    ntpdata.time_reftime = ktime_get_ntp_seconds(ntpdata - tk_ntp_data);
    }
// only set allowed bits
    ntpdata.time_status &= STA_RONLY;
    ntpdata.time_status |= txc.status & ~STA_RONLY;
    }
#[no_mangle]
pub unsafe extern "C" fn process_adjtimex_modes(ntpdata: *mut ntp_data, txc: *mut __kernel_timex, time_tai: *mut s32) {
    if (txc.modes & ADJ_STATUS) {
    process_adj_status(ntpdata, txc);
    }
    if (txc.modes & ADJ_NANO) {
    ntpdata.time_status |= STA_NANO;
    }
    if (txc.modes & ADJ_MICRO) {
    ntpdata.time_status &= ~STA_NANO;
    }
    if (txc.modes & ADJ_FREQUENCY) {
    ntpdata.time_freq = txc.freq * PPM_SCALE;
    ntpdata.time_freq = min(ntpdata.time_freq, MAXFREQ_SCALED);
    ntpdata.time_freq = max(ntpdata.time_freq, -MAXFREQ_SCALED);
// Update pps_freq
    pps_set_freq(ntpdata);
    }
    if (txc.modes & ADJ_MAXERROR) {
    ntpdata.time_maxerror = clamp(txc.maxerror, 0, NTP_PHASE_LIMIT);
    }
    if (txc.modes & ADJ_ESTERROR) {
    ntpdata.time_esterror = clamp(txc.esterror, 0, NTP_PHASE_LIMIT);
    }
    if (txc.modes & ADJ_TIMECONST) {
    ntpdata.time_constant = clamp(txc.constant, 0, MAXTC);
    if (!(ntpdata.time_status & STA_NANO)) {
    ntpdata.time_constant += 4;
    }
    ntpdata.time_constant = clamp(ntpdata.time_constant, 0, MAXTC);
    }
    if (txc.modes & ADJ_TAI && txc.constant >= 0 && txc.constant <= MAX_TAI_OFFSET) {
// time_tai = txc->constant;
    }
    if (txc.modes & ADJ_OFFSET) {
    ntp_update_offset(ntpdata, txc.offset);
    }
    if (txc.modes & ADJ_TICK) {
    ntpdata.tick_usec = txc.tick;
    }
    if (txc.modes & (ADJ_TICK|ADJ_FREQUENCY|ADJ_OFFSET)) {
    ntp_update_frequency(ntpdata);
    }
    }
//
// adjtimex() mainly allows reading (and writing, if superuser) of
// kernel time-keeping variables. used by xntpd.
//
#[no_mangle]
pub unsafe extern "C" fn ntp_adjtimex(tkid: c_uint, txc: *mut __kernel_timex, ts: *mut timespec64, time_tai: *mut s32, ad: *mut audit_ntp_data) -> c_int {
    let mut ntpdata = &tk_ntp_data[tkid];
    let mut result = 0;
    if (txc.modes & ADJ_ADJTIME) {
pub static mut save_adjust: c_long = 0;
    if (!(txc.modes & ADJ_OFFSET_READONLY)) {
// adjtime() is independent from ntp_adjtime()
    ntpdata.time_adjust = txc.offset;
    ntpdata.time_adjust_frac = 0;
    ntp_update_frequency(ntpdata);
    audit_ntp_set_old(ad, AUDIT_NTP_ADJUST,	save_adjust);
    audit_ntp_set_new(ad, AUDIT_NTP_ADJUST,	ntpdata.time_adjust);
    }
    txc.offset = save_adjust;
    } else {
// If there are input parameters, then process them:
    if (txc.modes) {
    audit_ntp_set_old(ad, AUDIT_NTP_OFFSET,	ntpdata.time_offset);
    audit_ntp_set_old(ad, AUDIT_NTP_FREQ,	ntpdata.time_freq);
    audit_ntp_set_old(ad, AUDIT_NTP_STATUS,	ntpdata.time_status);
    audit_ntp_set_old(ad, AUDIT_NTP_TAI,	*time_tai);
    audit_ntp_set_old(ad, AUDIT_NTP_TICK,	ntpdata.tick_usec);
    process_adjtimex_modes(ntpdata, txc, time_tai);
    audit_ntp_set_new(ad, AUDIT_NTP_OFFSET,	ntpdata.time_offset);
    audit_ntp_set_new(ad, AUDIT_NTP_FREQ,	ntpdata.time_freq);
    audit_ntp_set_new(ad, AUDIT_NTP_STATUS,	ntpdata.time_status);
    audit_ntp_set_new(ad, AUDIT_NTP_TAI,	*time_tai);
    audit_ntp_set_new(ad, AUDIT_NTP_TICK,	ntpdata.tick_usec);
    }
    txc.offset = shift_right(ntpdata.time_offset * NTP_INTERVAL_FREQ, NTP_SCALE_SHIFT);
    if (!(ntpdata.time_status & STA_NANO)) {
    txc.offset = div_s64(txc.offset, NSEC_PER_USEC);
    }
    }
    result = ntpdata.time_state;
    if (is_error_status(ntpdata.time_status)) {
    result = TIME_ERROR;
    }
    txc.freq	   = shift_right((ntpdata.time_freq >> PPM_SCALE_INV_SHIFT) *
    PPM_SCALE_INV, NTP_SCALE_SHIFT);
    txc.maxerror	   = ntpdata.time_maxerror;
    txc.esterror	   = ntpdata.time_esterror;
    txc.status	   = ntpdata.time_status;
    txc.constant	   = ntpdata.time_constant;
    txc.precision	   = 1;
    txc.tolerance	   = MAXFREQ_SCALED / PPM_SCALE;
    txc.tick	   = ntpdata.tick_usec;
    txc.tai	   = *time_tai;
// Fill PPS status fields
    pps_fill_timex(ntpdata, txc);
    txc.time.tv_sec = ts.tv_sec;
    txc.time.tv_usec = ts.tv_nsec;
    if (!(ntpdata.time_status & STA_NANO)) {
    txc.time.tv_usec = ts.tv_nsec / NSEC_PER_USEC;
    }
// Handle leapsec adjustments
    if (unlikely(ts.tv_sec >= ntpdata.ntp_next_leap_sec)) {
    if ((ntpdata.time_state == TIME_INS) && (ntpdata.time_status & STA_INS)) {
    result = TIME_OOP;
    txc.tai += 1;
    txc.time.tv_sec -= 1;
    }
    if ((ntpdata.time_state == TIME_DEL) && (ntpdata.time_status & STA_DEL)) {
    result = TIME_WAIT;
    txc.tai -= 1;
    txc.time.tv_sec += 1;
    }
    if ((ntpdata.time_state == TIME_OOP) && (ts.tv_sec == ntpdata.ntp_next_leap_sec)) {
    result = TIME_WAIT;
    }
    }
    return result;
    }

//
// struct pps_normtime is basically a struct timespec, but it is
// semantically different (and it is the reason why it was invented):
// pps_normtime.nsec has a range of ( -NSEC_PER_SEC / 2, NSEC_PER_SEC / 2 ]
// while timespec.tv_nsec has a range of [0, NSEC_PER_SEC)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_normtime {
//     pub /: *mut *mut s64 sec; / seconds,
//     pub /: *mut *mut long nsec; / nanoseconds,
}

//
// Normalize the timestamp so that nsec is in the
// [ -NSEC_PER_SEC / 2, NSEC_PER_SEC / 2 ] interval
//
#[no_mangle]
pub unsafe extern "C" fn pps_normalize_ts(ts: timespec64) -> pps_normtime {
pub static mut pps_normtime: usize = 0;
    if (norm.nsec > (NSEC_PER_SEC >> 1)) {
    norm.nsec -= NSEC_PER_SEC;
    norm.sec += 1;
    }
    return norm;
    }
// Get current phase correction and jitter
#[no_mangle]
pub unsafe extern "C" fn pps_phase_filter_get(ntpdata: *mut ntp_data, jitter: *mut c_long) -> c_long {
// jitter = ntpdata->pps_tf[0] - ntpdata->pps_tf[1];
    if (*jitter < 0) {
// jitter = -*jitter;
    }
// TODO: test various filters
    return ntpdata.pps_tf[0];
    }
// Add the sample to the phase filter
#[no_mangle]
pub unsafe extern "C" fn pps_phase_filter_add(ntpdata: *mut ntp_data, err: c_long) {
    ntpdata.pps_tf[2] = ntpdata.pps_tf[1];
    ntpdata.pps_tf[1] = ntpdata.pps_tf[0];
    ntpdata.pps_tf[0] = err;
    }
//
// Decrease frequency calibration interval length. It is halved after four
// consecutive unstable intervals.
//
#[no_mangle]
pub unsafe extern "C" fn pps_dec_freq_interval(ntpdata: *mut ntp_data) {
    if (--ntpdata.pps_intcnt <= -PPS_INTCOUNT) {
    ntpdata.pps_intcnt = -PPS_INTCOUNT;
    if (ntpdata.pps_shift > PPS_INTMIN) {
    ntpdata.pps_shift -= 1;
    ntpdata.pps_intcnt = 0;
    }
    }
    }
//
// Increase frequency calibration interval length. It is doubled after
// four consecutive stable intervals.
//
#[no_mangle]
pub unsafe extern "C" fn pps_inc_freq_interval(ntpdata: *mut ntp_data) {
    if (++ntpdata.pps_intcnt >= PPS_INTCOUNT) {
    ntpdata.pps_intcnt = PPS_INTCOUNT;
    if (ntpdata.pps_shift < PPS_INTMAX) {
    ntpdata.pps_shift += 1;
    ntpdata.pps_intcnt = 0;
    }
    }
    }
//
// Update clock frequency based on MONOTONIC_RAW clock PPS signal
// timestamps
//
// At the end of the calibration interval the difference between the
// first and last MONOTONIC_RAW clock timestamps divided by the length
// of the interval becomes the frequency update. If the interval was
// too long, the data are discarded.
// Returns the difference between old and new frequency values.
//
#[no_mangle]
unsafe extern "C" fn hardpps_update_freq(ntpdata: *mut ntp_data, freq_norm: pps_normtime) -> c_long {
    let mut delta = 0;
    let mut delta_mod = 0;
    let mut ftemp = 0;
// Check if the frequency interval was too long
    if (freq_norm.sec > (2 << ntpdata.pps_shift)) {
    ntpdata.time_status |= STA_PPSERROR;
    ntpdata.pps_errcnt += 1;
    pps_dec_freq_interval(ntpdata);
    printk_deferred("hardpps: PPSERROR: interval too long - %lld s\n",
    freq_norm.sec);
    return 0;
    }
//
// Here the raw frequency offset and wander (stability) is
// calculated. If the wander is less than the wander threshold the
// interval is increased; otherwise it is decreased.
//
    ftemp = div_s64(((s64)(-freq_norm.nsec)) << NTP_SCALE_SHIFT,
    freq_norm.sec);
    delta = shift_right(ftemp - ntpdata.pps_freq, NTP_SCALE_SHIFT);
    ntpdata.pps_freq = ftemp;
    if (delta > PPS_MAXWANDER || delta < -PPS_MAXWANDER) {
    printk_deferred("hardpps: PPSWANDER: change=%ld\n", delta);
    ntpdata.time_status |= STA_PPSWANDER;
    ntpdata.pps_stbcnt += 1;
    pps_dec_freq_interval(ntpdata);
    } else {
// Good sample
    pps_inc_freq_interval(ntpdata);
    }
//
// The stability metric is calculated as the average of recent
// frequency changes, but is used only for performance monitoring
//
    delta_mod = delta;
    if (delta_mod < 0) {
    delta_mod = -delta_mod;
    }
    ntpdata.pps_stabil += (div_s64(((s64)delta_mod) << (NTP_SCALE_SHIFT - SHIFT_USEC),
    NSEC_PER_USEC) - ntpdata.pps_stabil) >> PPS_INTMIN;
// If enabled, the system clock frequency is updated
    if ((ntpdata.time_status & STA_PPSFREQ) && !(ntpdata.time_status & STA_FREQHOLD)) {
    ntpdata.time_freq = ntpdata.pps_freq;
    ntp_update_frequency(ntpdata);
    }
    return delta;
    }
// Correct REALTIME clock phase error against PPS signal
#[no_mangle]
unsafe extern "C" fn hardpps_update_phase(ntpdata: *mut ntp_data, error: c_long) {
pub static mut correction: c_long = 0;
    let mut jitter = 0;
// Add the sample to the median filter
    pps_phase_filter_add(ntpdata, correction);
    correction = pps_phase_filter_get(ntpdata, &jitter);
//
// Nominal jitter is due to PPS signal noise. If it exceeds the
// threshold, the sample is discarded; otherwise, if so enabled,
// the time offset is updated.
//
    if (jitter > (ntpdata.pps_jitter << PPS_POPCORN)) {
    printk_deferred("hardpps: PPSJITTER: jitter=%ld, limit=%ld\n",
    jitter, (ntpdata.pps_jitter << PPS_POPCORN));
    ntpdata.time_status |= STA_PPSJITTER;
    ntpdata.pps_jitcnt += 1;
    } else if (ntpdata.time_status & STA_PPSTIME) {
// Correct the time using the phase offset
    ntpdata.time_offset = div_s64(((s64)correction) << NTP_SCALE_SHIFT,
    NTP_INTERVAL_FREQ);
// Cancel running adjtime()
    ntpdata.time_adjust = 0;
    ntpdata.time_adjust_frac = 0;
    }
// Update jitter
    ntpdata.pps_jitter += (jitter - ntpdata.pps_jitter) >> PPS_INTMIN;
    }
//
// __hardpps() - discipline CPU clock oscillator to external PPS signal
//
// This routine is called at each PPS signal arrival in order to
// discipline the CPU clock oscillator to the PPS signal. It takes two
// parameters: REALTIME and MONOTONIC_RAW clock timestamps. The former
// is used to correct clock phase error and the latter is used to
// correct the frequency.
//
// This code is based on David Mills's reference nanokernel
// implementation. It was mostly rewritten but keeps the same idea.
//
#[no_mangle]
pub unsafe extern "C" fn __hardpps(phase_ts: *const timespec64, raw_ts: *const timespec64) {
    let mut ntpdata = &tk_ntp_data[TIMEKEEPER_CORE];
    struct pps_normtime pts_norm, freq_norm;
    pts_norm = pps_normalize_ts(*phase_ts);
// Clear the error bits, they will be set again if needed
    ntpdata.time_status &= ~(STA_PPSJITTER | STA_PPSWANDER | STA_PPSERROR);
// indicate signal presence
    ntpdata.time_status |= STA_PPSSIGNAL;
    ntpdata.pps_valid = PPS_VALID;
//
// When called for the first time, just start the frequency
// interval
//
    if (unlikely(ntpdata.pps_fbase.tv_sec == 0)) {
    ntpdata.pps_fbase = *raw_ts;
    return;
    }
// Ok, now we have a base for frequency calculation
    freq_norm = pps_normalize_ts(timespec64_sub(*raw_ts, ntpdata.pps_fbase));
//
// Check that the signal is in the range
// [1s - MAXFREQ us, 1s + MAXFREQ us], otherwise reject it
//
    if ((freq_norm.sec == 0) || (freq_norm.nsec > MAXFREQ * freq_norm.sec) ||
    (freq_norm.nsec < -MAXFREQ * freq_norm.sec)) {
    ntpdata.time_status |= STA_PPSJITTER;
// Restart the frequency calibration interval
    ntpdata.pps_fbase = *raw_ts;
    printk_deferred("hardpps: PPSJITTER: bad pulse\n");
    return;
    }
// Signal is ok. Check if the current frequency interval is finished
    if (freq_norm.sec >= (1 << ntpdata.pps_shift)) {
    ntpdata.pps_calcnt += 1;
// Restart the frequency calibration interval
    ntpdata.pps_fbase = *raw_ts;
    hardpps_update_freq(ntpdata, freq_norm);
    }
    hardpps_update_phase(ntpdata, pts_norm.nsec);
    }

#[no_mangle]
unsafe extern "C" fn ntp_tick_adj_setup(str: *mut c_char) -> c_int {
pub static mut rc: c_int = 0;
    if (rc) {
    return rc;
    }
    tk_ntp_data[TIMEKEEPER_CORE].ntp_tick_adj <<= NTP_SCALE_SHIFT;
    return 1;
    }
    __setup!("ntp_tick_adj=", ntp_tick_adj_setup);
#[no_mangle]
pub unsafe extern "C" fn ntp_init()  {
    for (int id = 0; id < TIMEKEEPERS_MAX; id++) {
    __ntp_clear(tk_ntp_data + id);
    }
    ntp_init_cmos_sync();
    }