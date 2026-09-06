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
    static inline void fill_clock_configuration(struct vdso_clock *vc, const struct tk_read_base *base)
    {
    vc.cycle_last	= base.cycle_last;

    vc.max_cycles	= base.clock.max_cycles;

    vc.mask	= base.mask;
    vc.mult	= base.mult;
    vc.shift	= base.shift;
    }
#[no_mangle]
pub unsafe extern "C" fn update_vdso_time_data(vdata: *mut vdso_time_data, tk: *mut timekeeper) {
    static inline void update_vdso_time_data(struct vdso_time_data *vdata, struct timekeeper *tk)
    {
    struct vdso_clock *vc = vdata.clock_data;
    struct vdso_timestamp *vdso_ts;
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
    vdso_ts.sec++;
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
    vdso_ts.sec++;
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
    void update_vsyscall(struct timekeeper *tk)
    {
    struct vdso_time_data *vdata = vdso_k_time_data;
    struct vdso_clock *vc = vdata.clock_data;
    struct vdso_timestamp *vdso_ts;
    s32 clock_mode;
    u64 nsec;
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
    if (clock_mode != VDSO_CLOCKMODE_NONE)
    update_vdso_time_data(vdata, tk);
    __arch_update_vdso_clock(&vc[CS_HRES_COARSE]);
    __arch_update_vdso_clock(&vc[CS_RAW]);
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
    }
#[no_mangle]
pub unsafe extern "C" fn update_vsyscall_tz() {
    void update_vsyscall_tz(void)
    {
    struct vdso_time_data *vdata = vdso_k_time_data;
    vdata.tz_minuteswest = sys_tz.tz_minuteswest;
    vdata.tz_dsttime = sys_tz.tz_dsttime;
    __arch_sync_vdso_time_data(vdata);
    }

#[no_mangle]
pub unsafe extern "C" fn vdso_time_update_aux(tk: *mut timekeeper) {
    void vdso_time_update_aux(struct timekeeper *tk)
    {
    struct vdso_time_data *vdata = vdso_k_time_data;
    struct vdso_timestamp *vdso_ts;
    struct vdso_clock *vc;
    s32 clock_mode;
    u64 nsec;
    vc = &vdata.aux_clock_data[tk.id - TIMEKEEPER_AUX_FIRST];
    vdso_ts = &vc.basetime[VDSO_BASE_AUX];
    clock_mode = tk.tkr_mono.clock.vdso_clock_mode;
    if (!tk.clock_valid)
    clock_mode = VDSO_CLOCKMODE_NONE;
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
    unsigned long vdso_update_begin(void)
    {
    struct vdso_time_data *vdata = vdso_k_time_data;
    let mut flags: c_ulong = timekeeper_lock_irqsave();
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
    void vdso_update_end(unsigned long flags)
    {
    struct vdso_time_data *vdata = vdso_k_time_data;
    vdso_write_end(vdata);
    __arch_sync_vdso_time_data(vdata);
    timekeeper_unlock_irqrestore(flags);
    }
