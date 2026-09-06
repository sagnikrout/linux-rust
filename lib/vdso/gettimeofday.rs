//! Automatically rewritten from C to Rust
//! Source: lib/vdso/gettimeofday.c
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
// Generic userspace implementations of gettimeofday() and similar.
//

//
// The generic vDSO implementation requires that gettimeofday.h
// provides:
// - __arch_get_hw_counter(): to get the hw counter based on the
// clock_mode.
// - gettimeofday_fallback(): fallback for gettimeofday.
// - clock_gettime_fallback(): fallback for clock_gettime.
// - clock_getres_fallback(): fallback for clock_getres.
//

// Bring in default accessors

#[no_mangle]
unsafe extern "C" fn vdso_delta_ok(vc: *const vdso_clock, delta: u64) -> __always_inline bool {
    static __always_inline bool vdso_delta_ok(const struct vdso_clock *vc, u64 delta)
    {
    return delta < vc.max_cycles;
    }

#[no_mangle]
unsafe extern "C" fn vdso_delta_ok(vc: *const vdso_clock, delta: u64) -> __always_inline bool {
    static __always_inline bool vdso_delta_ok(const struct vdso_clock *vc, u64 delta)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn vdso_shift_ns(ns: u64, shift: u32) -> __always_inline u64 {
    static __always_inline u64 vdso_shift_ns(u64 ns, u32 shift)
    {
    return ns >> shift;
    }

//
// Default implementation which works for all sane clocksources. That
// obviously excludes x86/TSC.
//
#[no_mangle]
unsafe extern "C" fn vdso_calc_ns(vc: *const vdso_clock, cycles: u64, base: u64) -> __always_inline u64 {
    static __always_inline u64 vdso_calc_ns(const struct vdso_clock *vc, u64 cycles, u64 base)
    {
    let mut delta: u64 = (cycles - vc.cycle_last) & VDSO_DELTA_MASK(vc);
    if (likely(vdso_delta_ok(vc, delta)))
    return vdso_shift_ns((delta * vc.mult) + base, vc.shift);
    return mul_u64_u32_add_u64_shr(delta, vc.mult, base, vc.shift);
    }

#[no_mangle]
pub unsafe extern "C" fn __arch_vdso_hres_capable() -> bool {
    static inline bool __arch_vdso_hres_capable(void)
    {
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn vdso_clocksource_ok(vc: *const vdso_clock) -> bool {
    static inline bool vdso_clocksource_ok(const struct vdso_clock *vc)
    {
    return vc.clock_mode != VDSO_CLOCKMODE_NONE;
    }

#[no_mangle]
pub unsafe extern "C" fn vdso_cycles_ok(cycles: u64) -> bool {
    static inline bool vdso_cycles_ok(u64 cycles)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn vdso_clockid_valid(clock: clockid_t) -> __always_inline bool {
    static __always_inline bool vdso_clockid_valid(clockid_t clock)
    {
// Check for negative values or invalid clocks
    return likely((u32) clock <= CLOCK_AUX_LAST);
    }
//
// Must not be invoked within the sequence read section as a race inside
// that loop could result in __iter_div_u64_rem() being extremely slow.
//
#[no_mangle]
unsafe extern "C" fn vdso_set_timespec(ts: *mut __kernel_timespec, sec: u64, ns: u64) -> __always_inline void {
    static __always_inline void vdso_set_timespec(struct __kernel_timespec *ts, u64 sec, u64 ns)
    {
    ts.tv_sec = sec + __iter_div_u64_rem(ns, NSEC_PER_SEC, &ns);
    ts.tv_nsec = ns;
    }
    static __always_inline
    bool vdso_get_timestamp(const struct vdso_time_data *vd, const struct vdso_clock *vc,
    unsigned int clkidx, u64 *sec, u64 *ns)
    {
    const struct vdso_timestamp *vdso_ts = &vc.basetime[clkidx];
    u64 cycles;
    if (unlikely(!vdso_clocksource_ok(vc)))
    return false;
    cycles = __arch_get_hw_counter(vc.clock_mode, vd);
    if (unlikely(!vdso_cycles_ok(cycles)))
    return false;
// ns = vdso_calc_ns(vc, cycles, vdso_ts->nsec);
// sec = vdso_ts->sec;
    return true;
    }
    static __always_inline
    const struct vdso_time_data *vdso_timens_data(const struct vdso_time_data *vd)
    {
    return (void *)vd + PAGE_SIZE;
    }
    static __always_inline
    bool do_hres_timens(const struct vdso_time_data *vdns, const struct vdso_clock *vcns,
    clockid_t clk, struct __kernel_timespec *ts)
    {
    const struct vdso_time_data *vd = vdso_timens_data(vdns);
    const struct timens_offset *offs = &vcns.offset[clk];
    const struct vdso_clock *vc = vd.clock_data;
    u32 seq;
    s64 sec;
    u64 ns;
    if (clk != CLOCK_MONOTONIC_RAW)
    vc = &vc[CS_HRES_COARSE];
    else
    vc = &vc[CS_RAW];
    do {
    seq = vdso_read_begin(vc);
    if (!vdso_get_timestamp(vd, vc, clk, &sec, &ns))
    return false;
    } while (vdso_read_retry(vc, seq));
// Add the namespace offset
    sec += offs.sec;
    ns += offs.nsec;
    vdso_set_timespec(ts, sec, ns);
    return true;
    }
    static __always_inline
    bool do_hres(const struct vdso_time_data *vd, const struct vdso_clock *vc,
    clockid_t clk, struct __kernel_timespec *ts)
    {
    u64 sec, ns;
    u32 seq;
// Allows to compile the high resolution parts out
    if (!__arch_vdso_hres_capable())
    return false;
    do {
    if (vdso_read_begin_timens(vc, &seq))
    return do_hres_timens(vd, vc, clk, ts);
    if (!vdso_get_timestamp(vd, vc, clk, &sec, &ns))
    return false;
    } while (vdso_read_retry(vc, seq));
    vdso_set_timespec(ts, sec, ns);
    return true;
    }
    static __always_inline
    bool do_coarse_timens(const struct vdso_time_data *vdns, const struct vdso_clock *vcns,
    clockid_t clk, struct __kernel_timespec *ts)
    {
    const struct vdso_time_data *vd = vdso_timens_data(vdns);
    const struct timens_offset *offs = &vcns.offset[clk];
    const struct vdso_clock *vc = vd.clock_data;
    const struct vdso_timestamp *vdso_ts;
    u64 nsec;
    s64 sec;
    s32 seq;
    vdso_ts = &vc.basetime[clk];
    do {
    seq = vdso_read_begin(vc);
    sec = vdso_ts.sec;
    nsec = vdso_ts.nsec;
    } while (vdso_read_retry(vc, seq));
// Add the namespace offset
    sec += offs.sec;
    nsec += offs.nsec;
    vdso_set_timespec(ts, sec, nsec);
    return true;
    }
    static __always_inline
    bool do_coarse(const struct vdso_time_data *vd, const struct vdso_clock *vc,
    clockid_t clk, struct __kernel_timespec *ts)
    {
    const struct vdso_timestamp *vdso_ts = &vc.basetime[clk];
    u32 seq;
    do {
    if (vdso_read_begin_timens(vc, &seq))
    return do_coarse_timens(vd, vc, clk, ts);
    ts.tv_sec = vdso_ts.sec;
    ts.tv_nsec = vdso_ts.nsec;
    } while (vdso_read_retry(vc, seq));
    return true;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn do_aux(vd: *const vdso_time_data, clock: clockid_t, ts: *mut __kernel_timespec) -> bool {
    bool do_aux(const struct vdso_time_data *vd, clockid_t clock, struct __kernel_timespec *ts)
    {
    const struct vdso_clock *vc;
    u32 seq, idx;
    u64 sec, ns;
    if (!IS_ENABLED(CONFIG_POSIX_AUX_CLOCKS))
    return false;
    idx = clock - CLOCK_AUX;
    vc = &vd.aux_clock_data[idx];
    do {
    while (vdso_read_begin_timens(vc, &seq)) {
// Re-read from the real time data page, reload seq by looping
    vd = vdso_timens_data(vd);
    vc = &vd.aux_clock_data[idx];
    }
// Auxclock disabled?
    if (vc.clock_mode == VDSO_CLOCKMODE_NONE)
    return false;
    if (!vdso_get_timestamp(vd, vc, VDSO_BASE_AUX, &sec, &ns))
    return false;
    } while (vdso_read_retry(vc, seq));
    vdso_set_timespec(ts, sec, ns);
    return true;
    }
    static __always_inline bool
    __cvdso_clock_gettime_common(const struct vdso_time_data *vd, clockid_t clock,
    struct __kernel_timespec *ts)
    {
    const struct vdso_clock *vc = vd.clock_data;
    u32 msk;
    if (!vdso_clockid_valid(clock))
    return false;
//
// Convert the clockid to a bitmask and use it to check which
// clocks are handled in the VDSO directly.
//
    msk = 1U << clock;
    if (likely(msk & VDSO_HRES))
    vc = &vc[CS_HRES_COARSE];
#[no_mangle]
pub unsafe extern "C" fn if(VDSO_COARSE: msk &) -> else {
    else if (msk & VDSO_COARSE)
    return do_coarse(vd, &vc[CS_HRES_COARSE], clock, ts);
#[no_mangle]
pub unsafe extern "C" fn if(VDSO_RAW: msk &) -> else {
    else if (msk & VDSO_RAW)
    vc = &vc[CS_RAW];
#[no_mangle]
pub unsafe extern "C" fn if(VDSO_AUX: msk &) -> else {
    else if (msk & VDSO_AUX)
    return do_aux(vd, clock, ts);
    else
    return false;
    return do_hres(vd, vc, clock, ts);
    }
    static int
    __cvdso_clock_gettime_data(const struct vdso_time_data *vd, clockid_t clock,
    struct __kernel_timespec *ts)
    {
    bool ok;
    ok = __cvdso_clock_gettime_common(vd, clock, ts);
    if (unlikely(!ok))
    return clock_gettime_fallback(clock, ts);
    return 0;
    }
    static __maybe_unused int
    __cvdso_clock_gettime(clockid_t clock, struct __kernel_timespec *ts)
    {
    return __cvdso_clock_gettime_data(__arch_get_vdso_u_time_data(), clock, ts);
    }

    static int
    __cvdso_clock_gettime32_data(const struct vdso_time_data *vd, clockid_t clock,
    struct old_timespec32 *res)
    {
    struct __kernel_timespec ts;
    bool ok;
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_COMPAT_32BIT_TIME));
    ok = __cvdso_clock_gettime_common(vd, clock, &ts);
    if (unlikely(!ok))
    return clock_gettime32_fallback(clock, res);
// For ok == true
    res.tv_sec = ts.tv_sec;
    res.tv_nsec = ts.tv_nsec;
    return 0;
    }
    static __maybe_unused int
    __cvdso_clock_gettime32(clockid_t clock, struct old_timespec32 *res)
    {
    return __cvdso_clock_gettime32_data(__arch_get_vdso_u_time_data(), clock, res);
    }

    static int
    __cvdso_gettimeofday_data(const struct vdso_time_data *vd,
    struct __kernel_old_timeval *tv, struct timezone *tz)
    {
    const struct vdso_clock *vc = vd.clock_data;

    BUILD_BUG();

    BUILD_BUG_ON(sizeof(tv.tv_sec) != 8 && !IS_ENABLED(CONFIG_COMPAT_32BIT_TIME));
    if (likely(tv != core::ptr::null_mut())) {
    struct __kernel_timespec ts;
    if (!do_hres(vd, &vc[CS_HRES_COARSE], CLOCK_REALTIME, &ts))
    return gettimeofday_fallback(tv, tz);
    tv.tv_sec = ts.tv_sec;
    tv.tv_usec = (u32)ts.tv_nsec / NSEC_PER_USEC;
    }
    if (unlikely(tz != core::ptr::null_mut())) {
    if (vdso_is_timens_clock(vc))
    vd = vdso_timens_data(vd);
    tz.tz_minuteswest = vd[CS_HRES_COARSE].tz_minuteswest;
    tz.tz_dsttime = vd[CS_HRES_COARSE].tz_dsttime;
    }
    return 0;
    }
    static __maybe_unused int
    __cvdso_gettimeofday(struct __kernel_old_timeval *tv, struct timezone *tz)
    {
    return __cvdso_gettimeofday_data(__arch_get_vdso_u_time_data(), tv, tz);
    }

    static __kernel_old_time_t
    __cvdso_time_data(const struct vdso_time_data *vd, __kernel_old_time_t *time)
    {
    const struct vdso_clock *vc = vd.clock_data;
    __kernel_old_time_t t;

    BUILD_BUG();

    BUILD_BUG_ON(sizeof(*time) != 8 && !IS_ENABLED(CONFIG_COMPAT_32BIT_TIME));
    if (vdso_is_timens_clock(vc)) {
    vd = vdso_timens_data(vd);
    vc = vd.clock_data;
    }
    t = READ_ONCE(vc[CS_HRES_COARSE].basetime[CLOCK_REALTIME].sec);
    if (time)
// time = t;
    return t;
    }
#[no_mangle]
unsafe extern "C" fn __cvdso_time(time: *mut __kernel_old_time_t) -> __maybe_unused __kernel_old_time_t {
    static __maybe_unused __kernel_old_time_t __cvdso_time(__kernel_old_time_t *time)
    {
    return __cvdso_time_data(__arch_get_vdso_u_time_data(), time);
    }

    static __always_inline
    bool __cvdso_clock_getres_common(const struct vdso_time_data *vd, clockid_t clock,
    struct __kernel_timespec *res)
    {
    const struct vdso_clock *vc = vd.clock_data;
    u32 msk;
    u64 ns;
    if (!vdso_clockid_valid(clock))
    return false;
    if (vdso_is_timens_clock(vc))
    vd = vdso_timens_data(vd);
//
// Convert the clockid to a bitmask and use it to check which
// clocks are handled in the VDSO directly.
//
    msk = 1U << clock;
    if (msk & (VDSO_HRES | VDSO_RAW)) {
//
// Preserves the behaviour of posix_get_hrtimer_res().
//
    ns = READ_ONCE(vd.hrtimer_res);
    } else if (msk & VDSO_COARSE) {
//
// Preserves the behaviour of posix_get_coarse_res().
//
    ns = LOW_RES_NSEC;
    } else if (msk & VDSO_AUX) {
    ns = aux_clock_resolution_ns();
    } else {
    return false;
    }
    if (likely(res)) {
    res.tv_sec = 0;
    res.tv_nsec = ns;
    }
    return true;
    }
    static
    int __cvdso_clock_getres_data(const struct vdso_time_data *vd, clockid_t clock,
    struct __kernel_timespec *res)
    {
    bool ok;
    ok =  __cvdso_clock_getres_common(vd, clock, res);
    if (unlikely(!ok))
    return clock_getres_fallback(clock, res);
    return 0;
    }
    static __maybe_unused
#[no_mangle]
pub unsafe extern "C" fn __cvdso_clock_getres(clock: clockid_t, res: *mut __kernel_timespec) -> c_int {
    int __cvdso_clock_getres(clockid_t clock, struct __kernel_timespec *res)
    {
    return __cvdso_clock_getres_data(__arch_get_vdso_u_time_data(), clock, res);
    }

    static int
    __cvdso_clock_getres_time32_data(const struct vdso_time_data *vd, clockid_t clock,
    struct old_timespec32 *res)
    {
    struct __kernel_timespec ts;
    bool ok;
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_COMPAT_32BIT_TIME));
    ok = __cvdso_clock_getres_common(vd, clock, &ts);
    if (unlikely(!ok))
    return clock_getres32_fallback(clock, res);
    if (likely(res)) {
    res.tv_sec = ts.tv_sec;
    res.tv_nsec = ts.tv_nsec;
    }
    return 0;
    }
    static __maybe_unused int
    __cvdso_clock_getres_time32(clockid_t clock, struct old_timespec32 *res)
    {
    return __cvdso_clock_getres_time32_data(__arch_get_vdso_u_time_data(),
    clock, res);
    }

