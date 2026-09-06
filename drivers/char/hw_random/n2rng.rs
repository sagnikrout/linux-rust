//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/hw_random/n2rng.h
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
// n2rng.h: Niagara2 RNG defines.
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//
// ver1 devices - n2-rng, vf-rng, kt-rng
pub const RNG_v1_CTL_WAIT: c_uint = 0x0000000001fffe00ULL /* Minimum wait time    */;
pub const RNG_v1_CTL_WAIT_SHIFT: c_int = 9;
pub const RNG_v1_CTL_BYPASS: c_uint = 0x0000000000000100ULL /* VCO voltage source   */;
pub const RNG_v1_CTL_VCO: c_uint = 0x00000000000000c0ULL /* VCO rate control     */;
pub const RNG_v1_CTL_VCO_SHIFT: c_int = 6;
pub const RNG_v1_CTL_ASEL: c_uint = 0x0000000000000030ULL /* Analog MUX select    */;
pub const RNG_v1_CTL_ASEL_SHIFT: c_int = 4;
pub const RNG_v1_CTL_ASEL_NOOUT: c_int = 2;
// these are the same in v2 as in v1
pub const RNG_CTL_LFSR: c_uint = 0x0000000000000008ULL /* Use LFSR or plain shift */;
pub const RNG_CTL_ES3: c_uint = 0x0000000000000004ULL /* Enable entropy source 3 */;
pub const RNG_CTL_ES2: c_uint = 0x0000000000000002ULL /* Enable entropy source 2 */;
pub const RNG_CTL_ES1: c_uint = 0x0000000000000001ULL /* Enable entropy source 1 */;
// ver2 devices - m4-rng, m7-rng
pub const RNG_v2_CTL_WAIT: c_uint = 0x0000000007fff800ULL /* Minimum wait time    */;
pub const RNG_v2_CTL_WAIT_SHIFT: c_int = 12;
pub const RNG_v2_CTL_BYPASS: c_uint = 0x0000000000000400ULL /* VCO voltage source   */;
pub const RNG_v2_CTL_VCO: c_uint = 0x0000000000000300ULL /* VCO rate control     */;
pub const RNG_v2_CTL_VCO_SHIFT: c_int = 9;
pub const RNG_v2_CTL_PERF: c_uint = 0x0000000000000180ULL /* Perf */;
pub const RNG_v2_CTL_ASEL: c_uint = 0x0000000000000070ULL /* Analog MUX select    */;
pub const RNG_v2_CTL_ASEL_SHIFT: c_int = 4;
pub const RNG_v2_CTL_ASEL_NOOUT: c_int = 7;
pub const HV_FAST_RNG_GET_DIAG_CTL: c_uint = 0x130;
pub const HV_FAST_RNG_CTL_READ: c_uint = 0x131;
pub const HV_FAST_RNG_CTL_WRITE: c_uint = 0x132;
pub const HV_FAST_RNG_DATA_READ_DIAG: c_uint = 0x133;
pub const HV_FAST_RNG_DATA_READ: c_uint = 0x134;
pub const HV_RNG_STATE_UNCONFIGURED: c_int = 0;
pub const HV_RNG_STATE_CONFIGURED: c_int = 1;
pub const HV_RNG_STATE_HEALTHCHECK: c_int = 2;
pub const HV_RNG_STATE_ERROR: c_int = 3;
pub const HV_RNG_NUM_CONTROL: c_int = 4;
extern "C" {
    pub fn sun4v_rng_get_diag_ctl() -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum n2rng_compat_id {
    N2_n2_rng,
    N2_vf_rng,
    N2_kt_rng,
    N2_m4_rng,
    N2_m7_rng,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct n2rng_template {
    pub id: n2rng_compat_id,
    pub multi_capable: c_int,
    pub chip_version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct n2rng_unit {
    pub control: [u64; HV_RNG_NUM_CONTROL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct n2rng {
    pub op: *mut platform_device,
    pub flags: c_ulong,
pub const N2RNG_FLAG_MULTI: c_uint = 0x00000001 /* Multi-unit capable RNG */;
pub const N2RNG_FLAG_CONTROL: c_uint = 0x00000002 /* Operating in control domain */;
pub const N2RNG_FLAG_READY: c_uint = 0x00000008 /* Ready for hw-rng layer      */;
pub const N2RNG_FLAG_SHUTDOWN: c_uint = 0x00000010 /* Driver unregistering        */;
pub const N2RNG_FLAG_BUFFER_VALID: c_uint = 0x00000020 /* u32 buffer holds valid data */;
    pub data: *mut n2rng_template,
    pub num_units: c_int,
    pub units: *mut n2rng_unit,
    pub hwrng: hwrng,
    pub buffer: u32,
// Registered hypervisor group API major and minor version.
    pub hvapi_major: c_ulong,
    pub hvapi_minor: c_ulong,
    pub work: delayed_work,
    pub /: *mut *mut unsigned long hv_state; / HV_RNG_STATE_foo,
    pub health_check_sec: c_ulong,
    pub accum_cycles: c_ulong,
    pub wd_timeo: c_ulong,
pub const N2RNG_HEALTH_CHECK_SEC_DEFAULT: c_int = 0;
pub const N2RNG_ACCUM_CYCLES_DEFAULT: c_int = 2048;
pub const N2RNG_WD_TIMEO_DEFAULT: c_int = 0;
    pub scratch_control: [u64; HV_RNG_NUM_CONTROL],
pub const RNG_v1_SELFTEST_TICKS: c_int = 38859;

pub const RNG_v2_SELFTEST_TICKS: c_int = 64;

pub const SELFTEST_MATCH_GOAL: c_int = 6;
pub const SELFTEST_LOOPS_MAX: c_int = 40000;
pub const SELFTEST_BUFFER_WORDS: c_int = 8;
    pub test_data: u64,
    pub test_control: [u64; HV_RNG_NUM_CONTROL],
    pub test_buffer: [u64; SELFTEST_BUFFER_WORDS],
}

pub const N2RNG_BLOCK_LIMIT: c_int = 60000;
pub const N2RNG_BUSY_LIMIT: c_int = 100;
pub const N2RNG_HCHECK_LIMIT: c_int = 100;

