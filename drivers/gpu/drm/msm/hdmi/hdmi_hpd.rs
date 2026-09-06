//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/hdmi/hdmi_hpd.c
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn msm_hdmi_phy_reset(hdmi: *mut hdmi) {
    static void msm_hdmi_phy_reset(struct hdmi *hdmi)
    {
    unsigned int val;
    val = hdmi_read(hdmi, REG_HDMI_PHY_CTRL);
    if (val & HDMI_PHY_CTRL_SW_RESET_LOW) {
// pull low
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val & ~HDMI_PHY_CTRL_SW_RESET);
    } else {
// pull high
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val | HDMI_PHY_CTRL_SW_RESET);
    }
    if (val & HDMI_PHY_CTRL_SW_RESET_PLL_LOW) {
// pull low
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val & ~HDMI_PHY_CTRL_SW_RESET_PLL);
    } else {
// pull high
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val | HDMI_PHY_CTRL_SW_RESET_PLL);
    }
    msleep(100);
    if (val & HDMI_PHY_CTRL_SW_RESET_LOW) {
// pull high
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val | HDMI_PHY_CTRL_SW_RESET);
    } else {
// pull low
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val & ~HDMI_PHY_CTRL_SW_RESET);
    }
    if (val & HDMI_PHY_CTRL_SW_RESET_PLL_LOW) {
// pull high
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val | HDMI_PHY_CTRL_SW_RESET_PLL);
    } else {
// pull low
    hdmi_write(hdmi, REG_HDMI_PHY_CTRL,
    val & ~HDMI_PHY_CTRL_SW_RESET_PLL);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_hpd_enable(bridge: *mut drm_bridge) {
    void msm_hdmi_hpd_enable(struct drm_bridge *bridge)
    {
    struct hdmi_bridge *hdmi_bridge = to_hdmi_bridge(bridge);
    struct hdmi *hdmi = hdmi_bridge.hdmi;
    struct device *dev = &hdmi.pdev.dev;
    u32 hpd_ctrl;
    int ret;
    unsigned long flags;
    if (hdmi.hpd_gpiod)
    gpiod_set_value_cansleep(hdmi.hpd_gpiod, 1);
    ret = pm_runtime_resume_and_get(dev);
    if (WARN_ON(ret))
    return;
    mutex_lock(&hdmi.state_mutex);
    msm_hdmi_set_mode(hdmi, false);
    msm_hdmi_phy_reset(hdmi);
    msm_hdmi_set_mode(hdmi, true);
    hdmi.hpd_enabled = true;
    mutex_unlock(&hdmi.state_mutex);
    hdmi_write(hdmi, REG_HDMI_USEC_REFTIMER, 0x0001001b);
// enable HPD events:
    hdmi_write(hdmi, REG_HDMI_HPD_INT_CTRL,
    HDMI_HPD_INT_CTRL_INT_CONNECT |
    HDMI_HPD_INT_CTRL_INT_EN);
// set timeout to 4.1ms (max) for hardware debounce
    spin_lock_irqsave(&hdmi.reg_lock, flags);
    hpd_ctrl = hdmi_read(hdmi, REG_HDMI_HPD_CTRL);
    hpd_ctrl |= HDMI_HPD_CTRL_TIMEOUT(0x1fff);
// Toggle HPD circuit to trigger HPD sense
    hdmi_write(hdmi, REG_HDMI_HPD_CTRL,
    ~HDMI_HPD_CTRL_ENABLE & hpd_ctrl);
    hdmi_write(hdmi, REG_HDMI_HPD_CTRL,
    HDMI_HPD_CTRL_ENABLE | hpd_ctrl);
    spin_unlock_irqrestore(&hdmi.reg_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_hpd_disable(bridge: *mut drm_bridge) {
    void msm_hdmi_hpd_disable(struct drm_bridge *bridge)
    {
    struct hdmi_bridge *hdmi_bridge = to_hdmi_bridge(bridge);
    struct hdmi *hdmi = hdmi_bridge.hdmi;
    struct device *dev = &hdmi.pdev.dev;
// Disable HPD interrupt
    hdmi_write(hdmi, REG_HDMI_HPD_INT_CTRL, 0);
    mutex_lock(&hdmi.state_mutex);
    hdmi.hpd_enabled = false;
    msm_hdmi_set_mode(hdmi, hdmi.power_on);
    mutex_unlock(&hdmi.state_mutex);
    pm_runtime_put(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_hpd_irq(bridge: *mut drm_bridge) {
    void msm_hdmi_hpd_irq(struct drm_bridge *bridge)
    {
    struct hdmi_bridge *hdmi_bridge = to_hdmi_bridge(bridge);
    struct hdmi *hdmi = hdmi_bridge.hdmi;
    u32 hpd_int_status, hpd_int_ctrl;
// Process HPD:
    hpd_int_status = hdmi_read(hdmi, REG_HDMI_HPD_INT_STATUS);
    hpd_int_ctrl   = hdmi_read(hdmi, REG_HDMI_HPD_INT_CTRL);
    if ((hpd_int_ctrl & HDMI_HPD_INT_CTRL_INT_EN) &&
    (hpd_int_status & HDMI_HPD_INT_STATUS_INT)) {
    let mut detected: bool = !!(hpd_int_status & HDMI_HPD_INT_STATUS_CABLE_DETECTED);
// ack & disable (temporarily) HPD events:
    hdmi_write(hdmi, REG_HDMI_HPD_INT_CTRL,
    HDMI_HPD_INT_CTRL_INT_ACK);
    DBG("status=%04x, ctrl=%04x", hpd_int_status, hpd_int_ctrl);
// detect disconnect if we are connected or visa versa:
    hpd_int_ctrl = HDMI_HPD_INT_CTRL_INT_EN;
    if (!detected)
    hpd_int_ctrl |= HDMI_HPD_INT_CTRL_INT_CONNECT;
    hdmi_write(hdmi, REG_HDMI_HPD_INT_CTRL, hpd_int_ctrl);
    queue_work(hdmi.workq, &hdmi_bridge.hpd_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn detect_reg(hdmi: *mut hdmi) -> enum drm_connector_status {
    static enum drm_connector_status detect_reg(struct hdmi *hdmi)
    {
    let mut hpd_int_status: u32 = 0;
    int ret;
    ret = pm_runtime_resume_and_get(&hdmi.pdev.dev);
    if (ret)
    goto out;
    hpd_int_status = hdmi_read(hdmi, REG_HDMI_HPD_INT_STATUS);
    out:
    pm_runtime_put(&hdmi.pdev.dev);
    return (hpd_int_status & HDMI_HPD_INT_STATUS_CABLE_DETECTED) ?
    connector_status_connected : connector_status_disconnected;
    }
pub const HPD_GPIO_INDEX: c_int = 2;
#[no_mangle]
unsafe extern "C" fn detect_gpio(hdmi: *mut hdmi) -> enum drm_connector_status {
    static enum drm_connector_status detect_gpio(struct hdmi *hdmi)
    {
    return gpiod_get_value(hdmi.hpd_gpiod) ?
    connector_status_connected :
    connector_status_disconnected;
    }
    enum drm_connector_status
    msm_hdmi_bridge_detect(struct drm_bridge *bridge, struct drm_connector *connector)
    {
    struct hdmi_bridge *hdmi_bridge = to_hdmi_bridge(bridge);
    struct hdmi *hdmi = hdmi_bridge.hdmi;
    enum drm_connector_status stat_gpio, stat_reg;
    let mut retry: c_int = 20;
//
// some platforms may not have hpd gpio. Rely only on the status
// provided by REG_HDMI_HPD_INT_STATUS in this case.
//
    if (!hdmi.hpd_gpiod)
    return detect_reg(hdmi);
    do {
    stat_gpio = detect_gpio(hdmi);
    stat_reg  = detect_reg(hdmi);
    if (stat_gpio == stat_reg)
    break;
    mdelay(10);
    } while (--retry);
// the status we get from reading gpio seems to be more reliable,
// so trust that one the most if we didn't manage to get hdmi and
// gpio status to agree:
//
    if (stat_gpio != stat_reg) {
    DBG("HDMI_HPD_INT_STATUS tells us: %d", stat_reg);
    DBG("hpd gpio tells us: %d", stat_gpio);
    }
    return stat_gpio;
    }
