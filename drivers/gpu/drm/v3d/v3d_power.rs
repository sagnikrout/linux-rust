//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/v3d/v3d_power.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2026 Raspberry Pi

    static int
    v3d_resume_sms(struct v3d_dev *v3d)
    {
    if (v3d.ver < V3D_GEN_71)
    return 0;
    V3D_SMS_WRITE(V3D_SMS_TEE_CS, V3D_SMS_CLEAR_POWER_OFF);
    if (wait_for((V3D_GET_FIELD(V3D_SMS_READ(V3D_SMS_TEE_CS),
    V3D_SMS_STATE) == V3D_SMS_IDLE), 100)) {
    drm_err(&v3d.drm, "Failed to power up SMS\n");
    return -ETIMEDOUT;
    }
    v3d_reset_sms(v3d);
    return 0;
    }
    static int
    v3d_suspend_sms(struct v3d_dev *v3d)
    {
    if (v3d.ver < V3D_GEN_71)
    return 0;
    V3D_SMS_WRITE(V3D_SMS_TEE_CS, V3D_SMS_POWER_OFF);
    if (wait_for((V3D_GET_FIELD(V3D_SMS_READ(V3D_SMS_TEE_CS),
    V3D_SMS_STATE) == V3D_SMS_POWER_OFF_STATE), 100)) {
    drm_err(&v3d.drm, "Failed to power off SMS\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_power_suspend(dev: *mut device) -> c_int {
    int v3d_power_suspend(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    struct v3d_dev *v3d = to_v3d_dev(drm);
    int ret;
    v3d_perfmon_suspend(v3d);
    v3d_irq_disable(v3d);
    v3d_clean_caches(v3d);
// Wait until V3D has no active or pending AXI transactions.
    v3d_idle_axi(v3d, 0);
    v3d_idle_gca(v3d);
    ret = v3d_suspend_sms(v3d);
    if (ret) {
// Staying active: undo the GMP STOP_REQ from v3d_idle_axi().
    V3D_WRITE(V3D_GMP_CFG(v3d.ver),
    V3D_READ(V3D_GMP_CFG(v3d.ver)) & ~V3D_GMP_CFG_STOP_REQ);
    v3d_irq_enable(v3d);
    return ret;
    }
    clk_disable_unprepare(v3d.clk);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_power_resume(dev: *mut device) -> c_int {
    int v3d_power_resume(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    struct v3d_dev *v3d = to_v3d_dev(drm);
    int ret;
    ret = clk_prepare_enable(v3d.clk);
    if (ret)
    return ret;
    ret = v3d_resume_sms(v3d);
    if (ret) {
    clk_disable_unprepare(v3d.clk);
    return ret;
    }
    v3d_init_hw_state(v3d);
    v3d_mmu_set_page_table(v3d);
    v3d_irq_enable(v3d);
    v3d_perfmon_resume(v3d);
    return 0;
    }
