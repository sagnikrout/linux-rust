//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clocksource.h
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
// linux/include/linux/clocksource.h
//
// This file contains the structure definitions for clocksources.
//
// If you are not a clocksource, or timekeeping code, you should
// not be including this file!
//

//
// struct clocksource_hw_snapshot - Snapshot for the underlying hardware counter of derived
// clocksources like kvmclock or Hyper-V scaled TSC
// @hw_cycles:		The hardware counter value
// @hw_csid:		Clocksource ID of the hardware counter
//
// Such clocksources must implement the read_snapshot() callback and fill in the
// hardware counter value, the clocksource ID of the hardware counter and derive
// the actual clocksource cycles from @hw_cycles to provide an atomic snapshot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clocksource_hw_snapshot {
    pub hw_cycles: u64,
    pub hw_csid: clocksource_ids,
}

//
// struct clocksource - hardware abstraction for a free running counter
// Provides mostly state-free accessors to the underlying hardware.
// This is the structure used for system time.
//
// @read:		Returns a cycle value, passes clocksource as argument
// @mask:		Bitmask for two's complement
// subtraction of non 64 bit counters
// @mult:		Cycle to nanosecond multiplier
// @shift:		Cycle to nanosecond divisor (power of two)
// @max_idle_ns:	Maximum idle time permitted by the clocksource (nsecs)
// @maxadj:		Maximum adjustment value to mult (~11%)
// @archdata:		Optional arch-specific data
// @max_cycles:		Maximum safe cycle value which won't overflow on
// multiplication
// @max_raw_delta:	Maximum safe delta value for negative motion detection
// @name:		Pointer to clocksource name
// @list:		List head for registration (internal)
// @freq_khz:		Clocksource frequency in khz.
// @rating:		Rating value for selection (higher is better)
// To avoid rating inflation the following
// list should give you a guide as to how
// to assign your clocksource a rating
// 1-99: Unfit for real use
// Only available for bootup and testing purposes.
// 100-199: Base level usability.
// Functional for real use, but not desired.
// 200-299: Good.
// A correct and usable clocksource.
// 300-399: Desired.
// A reasonably fast and accurate clocksource.
// 400-499: Perfect
// The ideal clocksource. A must-use where
// available.
// @id:			Defaults to CSID_GENERIC. The id value is captured
// in certain snapshot functions to allow callers to
// validate the clocksource from which the snapshot was
// taken.
// @flags:		Flags describing special properties
// @base:		Hardware abstraction for clock on which a clocksource
// is based
// @read_snapshot:	Extended @read() function for clocksources such as
// kvmclock or the Hyper-V scaled TSC where the actual
// clocksource value for timekeeping is calculated from an
// underlying hardware counter. Returns the timekeeping
// relevant cycle value and stores the raw value of the
// underlying counter from which it was calculated
// including the clocksource ID of that counter in the
// clocksource hardware snapshot.
// @enable:		Optional function to enable the clocksource
// @disable:		Optional function to disable the clocksource
// @suspend:		Optional suspend function for the clocksource
// @resume:		Optional resume function for the clocksource
// @mark_unstable:	Optional function to inform the clocksource driver that
// the watchdog marked the clocksource unstable
// @tick_stable:        Optional function called periodically from the watchdog
// code to provide stable synchronization points
// @wd_list:		List head to enqueue into the watchdog list (internal)
// @cs_last:		Last clocksource value for clocksource watchdog
// @wd_last:		Last watchdog value corresponding to @cs_last
// @owner:		Module reference, must be set by clocksource in modules
//
// Note: This struct is not used in hotpathes of the timekeeping code
// because the timekeeper caches the hot path fields in its own data
// structure, so no cache line alignment is required,
//
// The pointer to the clocksource itself is handed to the read
// callback. If you need extra information there you can wrap struct
// clocksource into your own struct. Depending on the amount of
// information you need you should consider to cache line align that
// structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clocksource {
    pub cs): *mut *mut u64 (read)(struct clocksource,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
    pub max_idle_ns: u64,
    pub maxadj: u32,
    pub max_cycles: u64,
    pub max_raw_delta: u64,
    pub name: *const c_char,
    pub list: list_head,
    pub freq_khz: u32,
    pub rating: c_int,
    pub id: clocksource_ids,
    pub vdso_clock_mode: vdso_clock_mode,
    pub flags: c_ulong,
    pub base: *mut clocksource_base,
    pub chs): *mut *mut *mut u64 (read_snapshot)(struct clocksource cs, struct clocksource_hw_snapshot,
    pub cs): *mut *mut int (enable)(struct clocksource,
    pub cs): *mut *mut void (disable)(struct clocksource,
    pub cs): *mut *mut void (suspend)(struct clocksource,
    pub cs): *mut *mut void (resume)(struct clocksource,
    pub cs): *mut *mut void (mark_unstable)(struct clocksource,
    pub cs): *mut *mut void (tick_stable)(struct clocksource,
// private:

// Watchdog related data, used by the framework
    pub wd_list: list_head,
    pub cs_last: u64,
    pub wd_last: u64,
    pub wd_cpu: c_uint,

    pub owner: *mut module,
}

//
// Clock source flags bits::
//
pub const CLOCK_SOURCE_IS_CONTINUOUS: c_uint = 0x01;
pub const CLOCK_SOURCE_MUST_VERIFY: c_uint = 0x02;
pub const CLOCK_SOURCE_CALIBRATED: c_uint = 0x04;
pub const CLOCK_SOURCE_WATCHDOG: c_uint = 0x10;
pub const CLOCK_SOURCE_VALID_FOR_HRES: c_uint = 0x20;
pub const CLOCK_SOURCE_UNSTABLE: c_uint = 0x40;
pub const CLOCK_SOURCE_SUSPEND_NONSTOP: c_uint = 0x80;
pub const CLOCK_SOURCE_RESELECT: c_uint = 0x100;
pub const CLOCK_SOURCE_CAN_INLINE_READ: c_uint = 0x200;
pub const CLOCK_SOURCE_HAS_COUPLED_CLOCK_EVENT: c_uint = 0x400;
pub const CLOCK_SOURCE_WDTEST: c_uint = 0x800;
pub const CLOCK_SOURCE_WDTEST_PERCPU: c_uint = 0x1000;
// simplify initialization of mask field

// freq = cyc/from
// mult/2^shift  = ns/cyc
// mult = ns/cyc * 2^shift
// mult = from/freq * 2^shift
// mult = from * 2^shift / freq
// mult = (from<<shift) / freq
//
// clocksource_khz2mult - calculates mult from khz and shift
// @khz:		Clocksource frequency in KHz
// @shift_constant:	Clocksource shift factor
//
// Helper functions that converts a khz counter frequency to a timsource
// multiplier, given the clocksource shift value
//
extern "C" {
    pub fn clocksource_freq2mult(_arg: khz, _arg: shift_constant, _arg: NSEC_PER_MSEC) -> return;
}
//
// clocksource_hz2mult - calculates mult from hz and shift
// @hz:			Clocksource frequency in Hz
// @shift_constant:	Clocksource shift factor
//
// Helper functions that converts a hz counter
// frequency to a timsource multiplier, given the
// clocksource shift value
//
extern "C" {
    pub fn clocksource_freq2mult(_arg: hz, _arg: shift_constant, _arg: NSEC_PER_SEC) -> return;
}
//
// clocksource_cyc2ns - converts clocksource cycles to nanoseconds
// @cycles:	cycles
// @mult:	cycle to nanosecond multiplier
// @shift:	cycle to nanosecond divisor (power of two)
//
// Converts clocksource cycles to nanoseconds, using the given @mult and @shift.
// The code is optimized for performance and is not intended to work
// with absolute clocksource cycles (as those will easily overflow),
// but is only intended to be used with relative (delta) clocksource cycles.
//
// XXX - This could use some mult_lxl_ll() asm optimization
//
extern "C" {
    pub fn clocksource_unregister(clocksource*: *mut struct) -> c_int;
}
extern "C" {
    pub fn clocksource_touch_watchdog();
}
extern "C" {
    pub fn clocksource_suspend();
}
extern "C" {
    pub fn clocksource_resume();
}
extern "C" {
    pub fn clocksource_default_clock() -> *mut clocksource  __init;
}
extern "C" {
    pub fn clocksource_mark_unstable(cs: *mut clocksource);
}
extern "C" {
    pub fn clocksource_stop_suspend_timing(cs: *mut clocksource, now: u64) -> u64;
}
//
// Don't call __clocksource_register_scale directly, use
// clocksource_register_hz/khz
//
// Don't call this unless you are a default clocksource
// (AKA: jiffies) and absolutely have to.
//
extern "C" {
    pub fn __clocksource_register_scale(_arg: cs, _arg: 1, _arg: 0) -> return;
}
extern "C" {
    pub fn __clocksource_register_scale(_arg: cs, _arg: 1, _arg: hz) -> return;
}
extern "C" {
    pub fn __clocksource_register_scale(_arg: cs, _arg: 1000, _arg: khz) -> return;
}
extern "C" {
    pub fn __devm_clocksource_register_scale(_arg: dev, _arg: cs, _arg: 1, _arg: hz) -> return;
}
extern "C" {
    pub fn __devm_clocksource_register_scale(_arg: dev, _arg: cs, _arg: 1000, _arg: khz) -> return;
}

extern "C" {
    pub fn clocksource_arch_init(cs: *mut clocksource);
}

extern "C" {
    pub fn timekeeping_notify(clock: *mut clocksource) -> c_int;
}
extern "C" {
    pub fn clocksource_mmio_readl_up(: *mut clocksource) -> u64;
}
extern "C" {
    pub fn clocksource_mmio_readl_down(: *mut clocksource) -> u64;
}
extern "C" {
    pub fn clocksource_mmio_readw_up(: *mut clocksource) -> u64;
}
extern "C" {
    pub fn clocksource_mmio_readw_down(: *mut clocksource) -> u64;
}
extern "C" {
    pub fn clocksource_i8253_init() -> c_int;
}

extern "C" {
    pub fn timer_probe();
}

//
// struct clocksource_base - hardware abstraction for clock on which a clocksource
// is based
// @id:			Defaults to CSID_GENERIC. The id value is used for conversion
// functions which require that the current clocksource is based
// on a clocksource_base with a particular ID in certain snapshot
// functions to allow callers to validate the clocksource from
// which the snapshot was taken.
// @freq_khz:		Nominal frequency of the base clock in kHz
// @offset:		Offset between the base clock and the clocksource
// @numerator:		Numerator of the clock ratio between base clock and the clocksource
// @denominator:	Denominator of the clock ratio between base clock and the clocksource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clocksource_base {
    pub id: clocksource_ids,
    pub freq_khz: u32,
    pub offset: u64,
    pub numerator: u32,
    pub denominator: u32,
}
