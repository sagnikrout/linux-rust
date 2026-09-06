//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/bcm/bcm2835-power.c
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
//
// Power domain driver for Broadcom BCM2835
//
// Copyright (C) 2018 Broadcom
//

pub const PM_GNRIC: c_uint = 0x00;
pub const PM_AUDIO: c_uint = 0x04;
pub const PM_STATUS: c_uint = 0x18;
pub const PM_RSTC: c_uint = 0x1c;
pub const PM_RSTS: c_uint = 0x20;
pub const PM_WDOG: c_uint = 0x24;
pub const PM_PADS0: c_uint = 0x28;
pub const PM_PADS2: c_uint = 0x2c;
pub const PM_PADS3: c_uint = 0x30;
pub const PM_PADS4: c_uint = 0x34;
pub const PM_PADS5: c_uint = 0x38;
pub const PM_PADS6: c_uint = 0x3c;
pub const PM_CAM0: c_uint = 0x44;

pub const PM_CAM1: c_uint = 0x48;

pub const PM_CCP2TX: c_uint = 0x4c;

pub const PM_DSI0: c_uint = 0x50;

pub const PM_DSI1: c_uint = 0x54;

pub const PM_HDMI: c_uint = 0x58;

pub const PM_USB: c_uint = 0x5c;
// The power gates must be enabled with this bit before enabling the LDO in the
// USB block.
//

pub const PM_PXLDO: c_uint = 0x60;
pub const PM_PXBG: c_uint = 0x64;
pub const PM_DFT: c_uint = 0x68;
pub const PM_SMPS: c_uint = 0x6c;
pub const PM_XOSC: c_uint = 0x70;
pub const PM_SPAREW: c_uint = 0x74;
pub const PM_SPARER: c_uint = 0x78;
pub const PM_AVS_RSTDR: c_uint = 0x7c;
pub const PM_AVS_STAT: c_uint = 0x80;
pub const PM_AVS_EVENT: c_uint = 0x84;
pub const PM_AVS_INTEN: c_uint = 0x88;
pub const PM_DUMMY: c_uint = 0xfc;
pub const PM_IMAGE: c_uint = 0x108;
pub const PM_GRAFX: c_uint = 0x10c;
pub const PM_PROC: c_uint = 0x110;
pub const PM_GRAFX_2712: c_uint = 0x304;

pub const PM_INRUSH_SHIFT: c_int = 13;
pub const PM_INRUSH_3_5_MA: c_int = 0;
pub const PM_INRUSH_5_MA: c_int = 1;
pub const PM_INRUSH_10_MA: c_int = 2;
pub const PM_INRUSH_20_MA: c_int = 3;

pub const PM_PASSWORD: c_uint = 0x5a000000;
pub const PM_WDOG_TIME_SET: c_uint = 0x000fffff;
pub const PM_RSTC_WRCFG_CLR: c_uint = 0xffffffcf;
pub const PM_RSTS_HADWRH_SET: c_uint = 0x00000040;
pub const PM_RSTC_WRCFG_SET: c_uint = 0x00000030;
pub const PM_RSTC_WRCFG_FULL_RESET: c_uint = 0x00000020;
pub const PM_RSTC_RESET: c_uint = 0x00000102;

pub const ASB_BRDG_VERSION: c_uint = 0x00;
pub const ASB_CPR_CTRL: c_uint = 0x04;
pub const ASB_V3D_S_CTRL: c_uint = 0x08;
pub const ASB_V3D_M_CTRL: c_uint = 0x0c;
pub const ASB_ISP_S_CTRL: c_uint = 0x10;
pub const ASB_ISP_M_CTRL: c_uint = 0x14;
pub const ASB_H264_S_CTRL: c_uint = 0x18;
pub const ASB_H264_M_CTRL: c_uint = 0x1c;

pub const ASB_AXI_BRDG_ID: c_uint = 0x20;
pub const BCM2835_BRDG_ID: c_uint = 0x62726467;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_power_domain {
    pub base: generic_pm_domain,
    pub power: *mut bcm2835_power,
    pub domain: u32,
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_power {
    pub dev: *mut device,
// PM registers.
    pub base: *mut void __iomem,
// AXI Async bridge registers.
    pub asb: *mut void __iomem,
// RPiVid bridge registers.
    pub rpivid_asb: *mut void __iomem,
    pub pd_xlate: genpd_onecell_data,
    pub domains: [bcm2835_power_domain; BCM2835_POWER_DOMAIN_COUNT],
    pub reset: reset_controller_dev,
}

#[no_mangle]
unsafe extern "C" fn bcm2835_asb_control(power: *mut bcm2835_power, reg: u32, enable: bool) -> c_int {
    static int bcm2835_asb_control(struct bcm2835_power *power, u32 reg, bool enable)
    {
    void __iomem *base = power.asb;
    u32 val;
    switch (reg) {
    case 0:
    return 0;
    case ASB_V3D_S_CTRL:
    case ASB_V3D_M_CTRL:
    if (power.rpivid_asb)
    base = power.rpivid_asb;
    break;
    }
// Enable the module's async AXI bridges.
    if (enable) {
    val = readl(base + reg) & ~ASB_REQ_STOP;
    } else {
    val = readl(base + reg) | ASB_REQ_STOP;
    }
    writel(PM_PASSWORD | val, base + reg);
    if (readl_poll_timeout_atomic(base + reg, val,
    !!(val & ASB_ACK) != enable, 0, 100))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_asb_enable(power: *mut bcm2835_power, reg: u32) -> c_int {
    static int bcm2835_asb_enable(struct bcm2835_power *power, u32 reg)
    {
    return bcm2835_asb_control(power, reg, true);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_asb_disable(power: *mut bcm2835_power, reg: u32) -> c_int {
    static int bcm2835_asb_disable(struct bcm2835_power *power, u32 reg)
    {
    return bcm2835_asb_control(power, reg, false);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_power_power_off(pd: *mut bcm2835_power_domain, pm_reg: u32) -> c_int {
    static int bcm2835_power_power_off(struct bcm2835_power_domain *pd, u32 pm_reg)
    {
    struct bcm2835_power *power = pd.power;
// We don't run this on BCM2711
    if (power.rpivid_asb)
    return 0;
// Enable functional isolation
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~PM_ISFUNC);
// Enable electrical isolation
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~PM_ISPOW);
// Open the power switches.
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~PM_POWUP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_power_power_on(pd: *mut bcm2835_power_domain, pm_reg: u32) -> c_int {
    static int bcm2835_power_power_on(struct bcm2835_power_domain *pd, u32 pm_reg)
    {
    struct bcm2835_power *power = pd.power;
    struct device *dev = power.dev;
    int ret;
    int inrush;
    bool powok;
    u32 val;
// We don't run this on BCM2711
    if (power.rpivid_asb)
    return 0;
// If it was already powered on by the fw, leave it that way.
    if (PM_READ(pm_reg) & PM_POWUP)
    return 0;
// Enable power.  Allowing too much current at once may result
// in POWOK never getting set, so start low and ramp it up as
// necessary to succeed.
//
    powok = false;
    for (inrush = PM_INRUSH_3_5_MA; inrush <= PM_INRUSH_20_MA; inrush++) {
    PM_WRITE(pm_reg,
    (PM_READ(pm_reg) & ~PM_INRUSH_MASK) |
    (inrush << PM_INRUSH_SHIFT) |
    PM_POWUP);
    powok = !readl_poll_timeout_atomic(power.base + pm_reg,
    val, val & PM_POWOK, 0, 3);
    }
    if (!powok) {
    dev_err(dev, "Timeout waiting for %s power OK\n",
    pd.base.name);
    ret = -ETIMEDOUT;
    goto err_disable_powup;
    }
// Disable electrical isolation
    PM_WRITE(pm_reg, PM_READ(pm_reg) | PM_ISPOW);
// Repair memory
    PM_WRITE(pm_reg, PM_READ(pm_reg) | PM_MEMREP);
    if (readl_poll_timeout_atomic(power.base + pm_reg, val,
    val & PM_MRDONE, 0, 1)) {
    dev_err(dev, "Timeout waiting for %s memory repair\n",
    pd.base.name);
    ret = -ETIMEDOUT;
    goto err_disable_ispow;
    }
// Disable functional isolation
    PM_WRITE(pm_reg, PM_READ(pm_reg) | PM_ISFUNC);
    return 0;
    err_disable_ispow:
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~PM_ISPOW);
    err_disable_powup:
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~(PM_POWUP | PM_INRUSH_MASK));
    return ret;
    }
    static int bcm2835_asb_power_on(struct bcm2835_power_domain *pd,
    u32 pm_reg,
    u32 asb_m_reg,
    u32 asb_s_reg,
    u32 reset_flags)
    {
    struct bcm2835_power *power = pd.power;
    int ret;
    ret = clk_prepare_enable(pd.clk);
    if (ret) {
    dev_err(power.dev, "Failed to enable clock for %s\n",
    pd.base.name);
    return ret;
    }
// Wait 32 clocks for reset to propagate, 1 us will be enough
    udelay(1);
    clk_disable_unprepare(pd.clk);
// Deassert the resets.
    PM_WRITE(pm_reg, PM_READ(pm_reg) | reset_flags);
    ret = clk_prepare_enable(pd.clk);
    if (ret) {
    dev_err(power.dev, "Failed to enable clock for %s\n",
    pd.base.name);
    goto err_enable_resets;
    }
    ret = bcm2835_asb_enable(power, asb_m_reg);
    if (ret) {
    dev_err(power.dev, "Failed to enable ASB master for %s\n",
    pd.base.name);
    goto err_disable_clk;
    }
    ret = bcm2835_asb_enable(power, asb_s_reg);
    if (ret) {
    dev_err(power.dev, "Failed to enable ASB slave for %s\n",
    pd.base.name);
    goto err_disable_asb_master;
    }
    return 0;
    err_disable_asb_master:
    bcm2835_asb_disable(power, asb_m_reg);
    err_disable_clk:
    clk_disable_unprepare(pd.clk);
    err_enable_resets:
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~reset_flags);
    return ret;
    }
    static int bcm2835_asb_power_off(struct bcm2835_power_domain *pd,
    u32 pm_reg,
    u32 asb_m_reg,
    u32 asb_s_reg,
    u32 reset_flags)
    {
    struct bcm2835_power *power = pd.power;
    int ret;
    ret = bcm2835_asb_disable(power, asb_s_reg);
    if (ret) {
    dev_warn(power.dev, "Failed to disable ASB slave for %s\n",
    pd.base.name);
    return ret;
    }
    ret = bcm2835_asb_disable(power, asb_m_reg);
    if (ret) {
    dev_warn(power.dev, "Failed to disable ASB master for %s\n",
    pd.base.name);
    bcm2835_asb_enable(power, asb_s_reg);
    return ret;
    }
    clk_disable_unprepare(pd.clk);
// Assert the resets.
    PM_WRITE(pm_reg, PM_READ(pm_reg) & ~reset_flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_power_pd_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int bcm2835_power_pd_power_on(struct generic_pm_domain *domain)
    {
    struct bcm2835_power_domain *pd =
    container_of(domain, struct bcm2835_power_domain, base);
    struct bcm2835_power *power = pd.power;
    switch (pd.domain) {
    case BCM2835_POWER_DOMAIN_GRAFX:
    return bcm2835_power_power_on(pd, PM_GRAFX);
    case BCM2835_POWER_DOMAIN_GRAFX_V3D:
    if (!power.asb)
    return bcm2835_asb_power_on(pd, PM_GRAFX_2712,
    0, 0, PM_V3DRSTN);
    return bcm2835_asb_power_on(pd, PM_GRAFX,
    ASB_V3D_M_CTRL, ASB_V3D_S_CTRL,
    PM_V3DRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE:
    return bcm2835_power_power_on(pd, PM_IMAGE);
    case BCM2835_POWER_DOMAIN_IMAGE_PERI:
    return bcm2835_asb_power_on(pd, PM_IMAGE,
    0, 0,
    PM_PERIRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE_ISP:
    return bcm2835_asb_power_on(pd, PM_IMAGE,
    ASB_ISP_M_CTRL, ASB_ISP_S_CTRL,
    PM_ISPRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE_H264:
    return bcm2835_asb_power_on(pd, PM_IMAGE,
    ASB_H264_M_CTRL, ASB_H264_S_CTRL,
    PM_H264RSTN);
    case BCM2835_POWER_DOMAIN_USB:
    PM_WRITE(PM_USB, PM_USB_CTRLEN);
    return 0;
    case BCM2835_POWER_DOMAIN_DSI0:
    PM_WRITE(PM_DSI0, PM_DSI0_CTRLEN);
    PM_WRITE(PM_DSI0, PM_DSI0_CTRLEN | PM_DSI0_LDOHPEN);
    return 0;
    case BCM2835_POWER_DOMAIN_DSI1:
    PM_WRITE(PM_DSI1, PM_DSI1_CTRLEN);
    PM_WRITE(PM_DSI1, PM_DSI1_CTRLEN | PM_DSI1_LDOHPEN);
    return 0;
    case BCM2835_POWER_DOMAIN_CCP2TX:
    PM_WRITE(PM_CCP2TX, PM_CCP2TX_CTRLEN);
    PM_WRITE(PM_CCP2TX, PM_CCP2TX_CTRLEN | PM_CCP2TX_LDOEN);
    return 0;
    case BCM2835_POWER_DOMAIN_HDMI:
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) | PM_HDMI_RSTDR);
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) | PM_HDMI_CTRLEN);
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) & ~PM_HDMI_LDOPD);
    usleep_range(100, 200);
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) & ~PM_HDMI_RSTDR);
    return 0;
    default:
    dev_err(power.dev, "Invalid domain %d\n", pd.domain);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_power_pd_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int bcm2835_power_pd_power_off(struct generic_pm_domain *domain)
    {
    struct bcm2835_power_domain *pd =
    container_of(domain, struct bcm2835_power_domain, base);
    struct bcm2835_power *power = pd.power;
    switch (pd.domain) {
    case BCM2835_POWER_DOMAIN_GRAFX:
    return bcm2835_power_power_off(pd, PM_GRAFX);
    case BCM2835_POWER_DOMAIN_GRAFX_V3D:
    if (!power.asb)
    return bcm2835_asb_power_off(pd, PM_GRAFX_2712,
    0, 0, PM_V3DRSTN);
    return bcm2835_asb_power_off(pd, PM_GRAFX,
    ASB_V3D_M_CTRL, ASB_V3D_S_CTRL,
    PM_V3DRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE:
    return bcm2835_power_power_off(pd, PM_IMAGE);
    case BCM2835_POWER_DOMAIN_IMAGE_PERI:
    return bcm2835_asb_power_off(pd, PM_IMAGE,
    0, 0,
    PM_PERIRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE_ISP:
    return bcm2835_asb_power_off(pd, PM_IMAGE,
    ASB_ISP_M_CTRL, ASB_ISP_S_CTRL,
    PM_ISPRSTN);
    case BCM2835_POWER_DOMAIN_IMAGE_H264:
    return bcm2835_asb_power_off(pd, PM_IMAGE,
    ASB_H264_M_CTRL, ASB_H264_S_CTRL,
    PM_H264RSTN);
    case BCM2835_POWER_DOMAIN_USB:
    PM_WRITE(PM_USB, 0);
    return 0;
    case BCM2835_POWER_DOMAIN_DSI0:
    PM_WRITE(PM_DSI0, PM_DSI0_CTRLEN);
    PM_WRITE(PM_DSI0, 0);
    return 0;
    case BCM2835_POWER_DOMAIN_DSI1:
    PM_WRITE(PM_DSI1, PM_DSI1_CTRLEN);
    PM_WRITE(PM_DSI1, 0);
    return 0;
    case BCM2835_POWER_DOMAIN_CCP2TX:
    PM_WRITE(PM_CCP2TX, PM_CCP2TX_CTRLEN);
    PM_WRITE(PM_CCP2TX, 0);
    return 0;
    case BCM2835_POWER_DOMAIN_HDMI:
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) | PM_HDMI_LDOPD);
    PM_WRITE(PM_HDMI, PM_READ(PM_HDMI) & ~PM_HDMI_CTRLEN);
    return 0;
    default:
    dev_err(power.dev, "Invalid domain %d\n", pd.domain);
    return -EINVAL;
    }
    }
    static int
    bcm2835_init_power_domain(struct bcm2835_power *power,
    int pd_xlate_index, const char *name)
    {
    struct device *dev = power.dev;
    struct bcm2835_power_domain *dom = &power.domains[pd_xlate_index];
    dom.clk = devm_clk_get_optional(dev.parent, name);
    if (IS_ERR(dom.clk))
    return dev_err_probe(dev, PTR_ERR(dom.clk), "Failed to get clock %s\n",
    name);
    dom.base.name = name;
    dom.base.flags = GENPD_FLAG_ACTIVE_WAKEUP;
    dom.base.power_on = bcm2835_power_pd_power_on;
    dom.base.power_off = bcm2835_power_pd_power_off;
    dom.domain = pd_xlate_index;
    dom.power = power;
// XXX: on/off at boot?
    pm_genpd_init(&dom.base, core::ptr::null_mut(), true);
    power.pd_xlate.domains[pd_xlate_index] = &dom.base;
    return 0;
    }
// bcm2835_reset_reset - Resets a block that has a reset line in the
// PM block.
//
// The consumer of the reset controller must have the power domain up
// -- there's no reset ability with the power domain down.  To reset
// the sub-block, we just disable its access to memory through the
// ASB, reset, and re-enable.
//
    static int bcm2835_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct bcm2835_power *power = container_of(rcdev, struct bcm2835_power,
    reset);
    struct bcm2835_power_domain *pd;
    int ret;
    switch (id) {
    case BCM2835_RESET_V3D:
    pd = &power.domains[BCM2835_POWER_DOMAIN_GRAFX_V3D];
    break;
    case BCM2835_RESET_H264:
    pd = &power.domains[BCM2835_POWER_DOMAIN_IMAGE_H264];
    break;
    case BCM2835_RESET_ISP:
    pd = &power.domains[BCM2835_POWER_DOMAIN_IMAGE_ISP];
    break;
    default:
    dev_err(power.dev, "Bad reset id %ld\n", id);
    return -EINVAL;
    }
    ret = bcm2835_power_pd_power_off(&pd.base);
    if (ret)
    return ret;
    return bcm2835_power_pd_power_on(&pd.base);
    }
    static int bcm2835_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct bcm2835_power *power = container_of(rcdev, struct bcm2835_power,
    reset);
    switch (id) {
    case BCM2835_RESET_V3D:
    return !(PM_READ(PM_GRAFX) & PM_V3DRSTN);
    case BCM2835_RESET_H264:
    return !(PM_READ(PM_IMAGE) & PM_H264RSTN);
    case BCM2835_RESET_ISP:
    return !(PM_READ(PM_IMAGE) & PM_ISPRSTN);
    default:
    return -EINVAL;
    }
    }
    static const struct reset_control_ops bcm2835_reset_ops = {
    .reset = bcm2835_reset_reset,
    .status = bcm2835_reset_status,
    };
    static const char *const power_domain_names[] = {
    [BCM2835_POWER_DOMAIN_GRAFX] = "grafx",
    [BCM2835_POWER_DOMAIN_GRAFX_V3D] = "v3d",
    [BCM2835_POWER_DOMAIN_IMAGE] = "image",
    [BCM2835_POWER_DOMAIN_IMAGE_PERI] = "peri_image",
    [BCM2835_POWER_DOMAIN_IMAGE_H264] = "h264",
    [BCM2835_POWER_DOMAIN_IMAGE_ISP] = "isp",
    [BCM2835_POWER_DOMAIN_USB] = "usb",
    [BCM2835_POWER_DOMAIN_DSI0] = "dsi0",
    [BCM2835_POWER_DOMAIN_DSI1] = "dsi1",
    [BCM2835_POWER_DOMAIN_CAM0] = "cam0",
    [BCM2835_POWER_DOMAIN_CAM1] = "cam1",
    [BCM2835_POWER_DOMAIN_CCP2TX] = "ccp2tx",
    [BCM2835_POWER_DOMAIN_HDMI] = "hdmi",
    };
#[no_mangle]
unsafe extern "C" fn bcm2835_power_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_power_probe(struct platform_device *pdev)
    {
    struct bcm2835_pm *pm = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct bcm2835_power *power;
    static const struct {
    int parent, child;
    } domain_deps[] = {
    { BCM2835_POWER_DOMAIN_GRAFX, BCM2835_POWER_DOMAIN_GRAFX_V3D },
    { BCM2835_POWER_DOMAIN_IMAGE, BCM2835_POWER_DOMAIN_IMAGE_PERI },
    { BCM2835_POWER_DOMAIN_IMAGE, BCM2835_POWER_DOMAIN_IMAGE_H264 },
    { BCM2835_POWER_DOMAIN_IMAGE, BCM2835_POWER_DOMAIN_IMAGE_ISP },
    { BCM2835_POWER_DOMAIN_IMAGE_PERI, BCM2835_POWER_DOMAIN_USB },
    { BCM2835_POWER_DOMAIN_IMAGE_PERI, BCM2835_POWER_DOMAIN_CAM0 },
    { BCM2835_POWER_DOMAIN_IMAGE_PERI, BCM2835_POWER_DOMAIN_CAM1 },
    };
    let mut ret: c_int = 0, i;
    u32 id;
    power = devm_kzalloc(dev, sizeof(*power), GFP_KERNEL);
    if (!power)
    return -ENOMEM;
    platform_set_drvdata(pdev, power);
    power.dev = dev;
    power.base = pm.base;
    power.asb = pm.asb;
    power.rpivid_asb = pm.rpivid_asb;
    if (power.asb) {
    id = readl(power.asb + ASB_AXI_BRDG_ID);
    if (id != BCM2835_BRDG_ID /* "BRDG" */) {
    dev_err(dev, "ASB register ID returned 0x%08x\n", id);
    return -ENODEV;
    }
    }
    if (power.rpivid_asb) {
    id = readl(power.rpivid_asb + ASB_AXI_BRDG_ID);
    if (id != BCM2835_BRDG_ID /* "BRDG" */) {
    dev_err(dev, "RPiVid ASB register ID returned 0x%08x\n",
    id);
    return -ENODEV;
    }
    }
    power.pd_xlate.domains = devm_kcalloc(dev,
    ARRAY_SIZE(power_domain_names),
    sizeof(*power.pd_xlate.domains),
    GFP_KERNEL);
    if (!power.pd_xlate.domains)
    return -ENOMEM;
    power.pd_xlate.num_domains = ARRAY_SIZE(power_domain_names);
    for (i = 0; i < ARRAY_SIZE(power_domain_names); i++) {
    ret = bcm2835_init_power_domain(power, i, power_domain_names[i]);
    if (ret)
    goto fail;
    }
    for (i = 0; i < ARRAY_SIZE(domain_deps); i++) {
    pm_genpd_add_subdomain(&power.domains[domain_deps[i].parent].base,
    &power.domains[domain_deps[i].child].base);
    }
    power.reset.owner = THIS_MODULE;
    power.reset.nr_resets = BCM2835_RESET_COUNT;
    power.reset.ops = &bcm2835_reset_ops;
    power.reset.of_node = dev.parent.of_node;
    ret = devm_reset_controller_register(dev, &power.reset);
    if (ret)
    goto fail;
    ret = of_genpd_add_provider_onecell(dev.parent.of_node,
    &power.pd_xlate);
    if (ret) {
    dev_err_probe(dev, ret, "failed to add genpd provider\n");
    goto fail;
    }
    dev_info(dev, "Broadcom BCM2835 power domains driver");
    return 0;
    fail:
    for (i = 0; i < ARRAY_SIZE(power_domain_names); i++) {
    struct generic_pm_domain *dom = &power.domains[i].base;
    if (dom.name)
    pm_genpd_remove(dom);
    }
    return ret;
    }
    static struct platform_driver bcm2835_power_driver = {
    .probe		= bcm2835_power_probe,
    .driver = {
    .name =	"bcm2835-power",
    },
    };
    module_platform_driver(bcm2835_power_driver);
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("Driver for Broadcom BCM2835 PM power domains and reset");
