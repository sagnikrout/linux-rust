//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/remoteproc/qcom_rproc.h
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


//
// enum qcom_ssr_notify_type - Startup/Shutdown events related to a remoteproc
// processor.
//
// @QCOM_SSR_BEFORE_POWERUP:	Remoteproc about to start (prepare stage)
// @QCOM_SSR_AFTER_POWERUP:	Remoteproc is running (start stage)
// @QCOM_SSR_BEFORE_SHUTDOWN:	Remoteproc crashed or shutting down (stop stage)
// @QCOM_SSR_AFTER_SHUTDOWN:	Remoteproc is down (unprepare stage)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_ssr_notify_type {
    QCOM_SSR_BEFORE_POWERUP,
    QCOM_SSR_AFTER_POWERUP,
    QCOM_SSR_BEFORE_SHUTDOWN,
    QCOM_SSR_AFTER_SHUTDOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_ssr_notify_data {
    pub name: *const c_char,
    pub crashed: bool,
}

extern "C" {
    pub fn qcom_unregister_ssr_notifier(notify: *mut c_void, nb: *mut notifier_block) -> c_int;
}

