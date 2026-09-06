//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/trace.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation

#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_init(sdev: *mut snd_sof_dev) -> c_int {
    int sof_fw_trace_init(struct snd_sof_dev *sdev)
    {
    const struct sof_ipc_fw_tracing_ops *fw_tracing = sof_ipc_get_ops(sdev, fw_tracing);
    if (!fw_tracing) {
    dev_info(sdev.dev, "Firmware tracing is not available\n");
    sdev.fw_trace_is_supported = false;
    return 0;
    }
    return fw_tracing.init(sdev);
    }
#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_free(sdev: *mut snd_sof_dev) {
    void sof_fw_trace_free(struct snd_sof_dev *sdev)
    {
    if (!sdev.fw_trace_is_supported)
    return;
    if (sdev.ipc.ops.fw_tracing.free)
    sdev.ipc.ops.fw_tracing.free(sdev);
    }
#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev) {
    void sof_fw_trace_fw_crashed(struct snd_sof_dev *sdev)
    {
    if (!sdev.fw_trace_is_supported)
    return;
    if (sdev.ipc.ops.fw_tracing.fw_crashed)
    sdev.ipc.ops.fw_tracing.fw_crashed(sdev);
    }
#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t) {
    void sof_fw_trace_suspend(struct snd_sof_dev *sdev, pm_message_t pm_state)
    {
    if (!sdev.fw_trace_is_supported)
    return;
    sdev.ipc.ops.fw_tracing.suspend(sdev, pm_state);
    }
#[no_mangle]
pub unsafe extern "C" fn sof_fw_trace_resume(sdev: *mut snd_sof_dev) -> c_int {
    int sof_fw_trace_resume(struct snd_sof_dev *sdev)
    {
    if (!sdev.fw_trace_is_supported)
    return 0;
    return sdev.ipc.ops.fw_tracing.resume(sdev);
    }
