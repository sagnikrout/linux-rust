//! Automatically rewritten from C Header to Rust Module
//! Source: include/vdso/datapage.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_vdso_time_data {

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_arch_data {
// Needed for the generic code, never actually used at runtime
    pub __unused: c_char,
}

pub const VDSO_BASE_AUX: c_int = 0;

pub const CS_HRES_COARSE: c_int = 0;
pub const CS_RAW: c_int = 1;

//
// struct vdso_timestamp - basetime per clock_id
// @sec:	seconds
// @nsec:	nanoseconds
//
// There is one vdso_timestamp object in vvar for each vDSO-accelerated
// clock_id. For high-resolution clocks, this encodes the time
// corresponding to vdso_time_data.cycle_last. For coarse clocks this encodes
// the actual time.
//
// To be noticed that for highres clocks nsec is left-shifted by
// vdso_time_data[x].shift.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_timestamp {
    pub sec: u64,
    pub nsec: u64,
}

//
// struct vdso_clock - vdso per clocksource datapage representation
// @seq:		timebase sequence counter
// @clock_mode:		clock mode
// @cycle_last:		timebase at clocksource init
// @max_cycles:		maximum cycles which won't overflow 64bit multiplication
// @mask:		clocksource mask
// @mult:		clocksource multiplier
// @shift:		clocksource shift
// @basetime:		basetime per clock_id
// @offset:		time namespace offset per clock_id
//
// See also struct vdso_time_data for basic access and ordering information as
// struct vdso_clock is used there.
//
// @basetime is used to store the base time for the system wide time getter
// VVAR page.
//
// @offset is used by the special time namespace VVAR pages which are
// installed instead of the real VVAR page. These namespace pages must set
// @seq to 1 and @clock_mode to VDSO_CLOCKMODE_TIMENS to force the code into
// the time namespace slow path. The namespace aware functions retrieve the
// real system wide VVAR page, read host time and add the per clock offset.
// For clocks which are not affected by time namespace adjustment the
// offset must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_clock {
    pub seq: u32,
    pub clock_mode: i32,
    pub cycle_last: u64,

    pub max_cycles: u64,

    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
    pub basetime: [vdso_timestamp; VDSO_BASES],
    pub offset: [timens_offset; VDSO_BASES],
}

//
// struct vdso_time_data - vdso datapage representation
// @arch_data:		architecture specific data (optional, defaults
// to an empty struct)
// @clock_data:		clocksource related data (array)
// @aux_clock_data:	auxiliary clocksource related data (array)
// @tz_minuteswest:	minutes west of Greenwich
// @tz_dsttime:		type of DST correction
// @hrtimer_res:	hrtimer resolution
// @__unused:		unused
//
// vdso_time_data will be accessed by 64 bit and compat code at the same time
// so we should be careful before modifying this structure.
//
// The ordering of the struct members is optimized to have fast acces to the
// often required struct members which are related to CLOCK_REALTIME and
// CLOCK_MONOTONIC. This information is stored in the first cache lines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_time_data {
    pub arch_data: arch_vdso_time_data,
    pub clock_data: [vdso_clock; CS_BASES],
    pub aux_clock_data: [vdso_clock; MAX_AUX_CLOCKS],
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
    pub hrtimer_res: u32,
    pub __unused: u32,
    pub ____cacheline_aligned: },
//
// struct vdso_rng_data - vdso RNG state information
// @generation:	counter representing the number of RNG reseeds
// @is_ready:	boolean signaling whether the RNG is initialized
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_rng_data {
    pub generation: u64,
    pub is_ready: u8,
}

//
// We use the hidden visibility to prevent the compiler from generating a GOT
// relocation. Not only is going through a GOT useless (the entry couldn't and
// must not be overridden by another library), it does not even work: the linker
// cannot generate an absolute address to the data page.
//
// With the hidden visibility, the compiler simply generates a PC-relative
// relocation, and this is what we need.
//
extern "C" {
    pub fn __attribute__(_arg: (visibility("hidden"))) -> vdso_time_data vdso_u_time_data;
}
extern "C" {
    pub fn __attribute__(_arg: (visibility("hidden"))) -> vdso_rng_data vdso_u_rng_data;
}
extern "C" {
    pub fn __attribute__(_arg: (visibility("hidden"))) -> vdso_arch_data vdso_u_arch_data;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdso_pages {
    VDSO_TIME_PAGE_OFFSET,
    VDSO_TIMENS_PAGE_OFFSET,
    VDSO_RNG_PAGE_OFFSET,
    VDSO_ARCH_PAGES_START,
    VDSO_ARCH_PAGES_END = VDSO_ARCH_PAGES_START + VDSO_ARCH_DATA_PAGES - 1,
    VDSO_NR_PAGES
}

// Macro flag: #define __vdso_u_rng_data

// Macro flag: #define __vdso_u_arch_data

