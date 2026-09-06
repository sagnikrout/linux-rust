//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/qcom_q6v5.h
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
pub struct qcom_q6v5 {
    pub dev: *mut device,
    pub rproc: *mut rproc,
    pub state: *mut qcom_smem_state,
    pub qmp: *mut qmp,
    pub path: *mut icc_path,
    pub stop_bit: unsigned,
    pub wdog_irq: c_int,
    pub fatal_irq: c_int,
    pub ready_irq: c_int,
    pub handover_irq: c_int,
    pub stop_irq: c_int,
// Protects handover_irq_enabled against stop/handover races.
    pub handover_lock: spinlock_t,
    pub handover_irq_enabled: bool,
    pub handover_issued: bool,
    pub start_done: completion,
    pub stop_done: completion,
    pub crash_reason: c_int,
    pub running: bool,
    pub load_state: *const c_char,
    pub q6v5): *mut *mut void (handover)(struct qcom_q6v5,
}

extern "C" {
    pub fn qcom_q6v5_deinit(q6v5: *mut qcom_q6v5);
}
extern "C" {
    pub fn qcom_q6v5_prepare(q6v5: *mut qcom_q6v5) -> c_int;
}
extern "C" {
    pub fn qcom_q6v5_unprepare(q6v5: *mut qcom_q6v5) -> c_int;
}
extern "C" {
    pub fn qcom_q6v5_request_stop(q6v5: *mut qcom_q6v5, sysmon: *mut qcom_sysmon) -> c_int;
}
extern "C" {
    pub fn qcom_q6v5_wait_for_start(q6v5: *mut qcom_q6v5, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn qcom_q6v5_panic(q6v5: *mut qcom_q6v5) -> c_ulong;
}
