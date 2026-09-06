//! Automatically rewritten from C to Rust
//! Source: drivers/phy/tegra/xusb-tegra186.c
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
// Copyright (c) 2016-2022, NVIDIA CORPORATION.  All rights reserved.
//

// FUSE USB_CALIB registers

pub const HS_CURR_LEVEL_PAD_MASK: c_uint = 0x3f;

pub const HS_TERM_RANGE_ADJ_PAD_MASK: c_uint = 0xf;
pub const HS_SQUELCH_SHIFT: c_int = 29;
pub const HS_SQUELCH_MASK: c_uint = 0x7;
pub const RPD_CTRL_SHIFT: c_int = 0;
pub const RPD_CTRL_MASK: c_uint = 0x1f;
// XUSB PADCTL registers
pub const XUSB_PADCTL_USB2_PAD_MUX: c_uint = 0x4;

pub const USB2_PORT_MASK: c_uint = 0x3;
pub const PORT_XUSB: c_int = 1;

pub const HSIC_PORT_MASK: c_uint = 0x1;
pub const PORT_HSIC: c_int = 0;
pub const XUSB_PADCTL_USB2_PORT_CAP: c_uint = 0x8;
pub const XUSB_PADCTL_SS_PORT_CAP: c_uint = 0xc;

pub const PORT_CAP_MASK: c_uint = 0x3;
pub const PORT_CAP_DISABLED: c_uint = 0x0;
pub const PORT_CAP_HOST: c_uint = 0x1;
pub const PORT_CAP_DEVICE: c_uint = 0x2;
pub const PORT_CAP_OTG: c_uint = 0x3;
pub const XUSB_PADCTL_ELPG_PROGRAM: c_uint = 0x20;

    (USB2_PORT_WAKEUP_EVENT(0) | USB2_PORT_WAKEUP_EVENT(1) |	\
    USB2_PORT_WAKEUP_EVENT(2) | SS_PORT_WAKEUP_EVENT(0) |		\
    SS_PORT_WAKEUP_EVENT(1) | SS_PORT_WAKEUP_EVENT(2) |		\
    USB2_HSIC_PORT_WAKEUP_EVENT(0))
pub const XUSB_PADCTL_ELPG_PROGRAM_1: c_uint = 0x24;

pub const XUSB_PADCTL_SS_PORT_CFG: c_uint = 0x2c;

pub const XUSB_PADCTL_USB2_BIAS_PAD_CTL0: c_uint = 0x284;

pub const XUSB_PADCTL_USB2_BIAS_PAD_CTL1: c_uint = 0x288;

pub const XUSB_PADCTL_USB2_BIAS_PAD_CTL2: c_uint = 0x28c;

pub const XUSB_PADCTL_HSIC_PAD_TRK_CTL0: c_uint = 0x340;

pub const USB2_VBUS_ID: c_uint = 0x360;

// XUSB AO registers

// phase A

// phase B

// phase C

// phase D

    (MASTER_ENABLE_B | MASTER_ENABLE_C | MASTER_ENABLE_D)

// phase A

// phase B

// phase C

// phase D

pub const TEGRA_UTMI_PAD_MAX: c_int = 4;

    {								\
    .name = _name,						\
    .offset = _offset,					\
    .shift = _shift,					\
    .mask = _mask,						\
    .num_funcs = ARRAY_SIZE(tegra186_##_type##_functions),	\
    .funcs = tegra186_##_type##_functions,			\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_xusb_fuse_calibration {
    pub hs_curr_level: *mut u32,
    pub hs_squelch: u32,
    pub hs_term_range_adj: *mut u32,
    pub rpd_ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_xusb_padctl_context {
    pub vbus_id: u32,
    pub usb2_pad_mux: u32,
    pub usb2_port_cap: u32,
    pub ss_port_cap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_xusb_padctl {
    pub base: tegra_xusb_padctl,
    pub ao_regs: *mut void __iomem,
    pub calib: tegra_xusb_fuse_calibration,
// UTMI bias and tracking
    pub usb2_trk_clk: *mut clk,
    pub TEGRA_UTMI_PAD_MAX): DECLARE_BITMAP(utmi_pad_enabled,,
// padctl context
    pub context: tegra186_xusb_padctl_context,
}

#[no_mangle]
pub unsafe extern "C" fn ao_writel(priv: *mut tegra186_xusb_padctl, value: u32, offset: c_uint) {
    static inline void ao_writel(struct tegra186_xusb_padctl *priv, u32 value, unsigned int offset)
    {
    writel(value, priv.ao_regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn ao_readl(priv: *mut tegra186_xusb_padctl, offset: c_uint) -> u32 {
    static inline u32 ao_readl(struct tegra186_xusb_padctl *priv, unsigned int offset)
    {
    return readl(priv.ao_regs + offset);
    }
    static inline struct tegra186_xusb_padctl *
    to_tegra186_xusb_padctl(struct tegra_xusb_padctl *padctl)
    {
    return container_of(padctl, struct tegra186_xusb_padctl, base);
    }
// USB 2.0 UTMI PHY support
    static struct tegra_xusb_lane *
    tegra186_usb2_lane_probe(struct tegra_xusb_pad *pad, struct device_node *np,
    unsigned int index)
    {
    struct tegra_xusb_usb2_lane *usb2;
    int err;
    usb2 = kzalloc_obj(*usb2);
    if (!usb2)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&usb2.base.list);
    usb2.base.soc = &pad.soc.lanes[index];
    usb2.base.index = index;
    usb2.base.pad = pad;
    usb2.base.np = np;
    err = tegra_xusb_lane_parse_dt(&usb2.base, np);
    if (err < 0) {
    kfree(usb2);
    return ERR_PTR(err);
    }
    return &usb2.base;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb2_lane_remove(lane: *mut tegra_xusb_lane) {
    static void tegra186_usb2_lane_remove(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_usb2_lane *usb2 = to_usb2_lane(lane);
    kfree(usb2);
    }
    static int tegra186_utmi_enable_phy_sleepwalk(struct tegra_xusb_lane *lane,
    enum usb_device_speed speed)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
// ensure sleepwalk logic is disabled
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~MASTER_ENABLE;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// ensure sleepwalk logics are in low power mode
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value |= MASTER_CFG_SEL;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// set debounce time
    value = ao_readl(priv, XUSB_AO_USB_DEBOUNCE_DEL);
    value &= ~UTMIP_LINE_DEB_CNT(~0);
    value |= UTMIP_LINE_DEB_CNT(1);
    ao_writel(priv, value, XUSB_AO_USB_DEBOUNCE_DEL);
// ensure fake events of sleepwalk logic are desiabled
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~(FAKE_USBOP_VAL | FAKE_USBON_VAL |
    FAKE_USBOP_EN | FAKE_USBON_EN);
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// ensure wake events of sleepwalk logic are not latched
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~LINE_WAKEUP_EN;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// disable wake event triggers of sleepwalk logic
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~WAKE_VAL(~0);
    value |= WAKE_VAL_NONE;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// power down the line state detectors of the pad
    value = ao_readl(priv, XUSB_AO_UTMIP_PAD_CFG(index));
    value |= (USBOP_VAL_PD | USBON_VAL_PD);
    ao_writel(priv, value, XUSB_AO_UTMIP_PAD_CFG(index));
// save state per speed
    value = ao_readl(priv, XUSB_AO_UTMIP_SAVED_STATE(index));
    value &= ~SPEED(~0);
    switch (speed) {
    case USB_SPEED_HIGH:
    value |= UTMI_HS;
    break;
    case USB_SPEED_FULL:
    value |= UTMI_FS;
    break;
    case USB_SPEED_LOW:
    value |= UTMI_LS;
    break;
    default:
    value |= UTMI_RST;
    break;
    }
    ao_writel(priv, value, XUSB_AO_UTMIP_SAVED_STATE(index));
// enable the trigger of the sleepwalk logic
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value |= LINEVAL_WALK_EN;
    value &= ~WAKE_WALK_EN;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// reset the walk pointer and clear the alarm of the sleepwalk logic,
// as well as capture the configuration of the USB2.0 pad
//
    value = ao_readl(priv, XUSB_AO_UTMIP_TRIGGERS(index));
    value |= (CLR_WALK_PTR | CLR_WAKE_ALARM | CAP_CFG);
    ao_writel(priv, value, XUSB_AO_UTMIP_TRIGGERS(index));
// setup the pull-ups and pull-downs of the signals during the four
// stages of sleepwalk.
// if device is connected, program sleepwalk logic to maintain a J and
// keep driving K upon seeing remote wake.
//
    value = USBOP_RPD_A | USBOP_RPD_B | USBOP_RPD_C | USBOP_RPD_D;
    value |= USBON_RPD_A | USBON_RPD_B | USBON_RPD_C | USBON_RPD_D;
    switch (speed) {
    case USB_SPEED_HIGH:
    case USB_SPEED_FULL:
// J state: D+/D- = high/low, K state: D+/D- = low/high
    value |= HIGHZ_A;
    value |= AP_A;
    value |= AN_B | AN_C | AN_D;
    if (padctl.soc.supports_lp_cfg_en)
    value |= MASTER_ENABLE_B_C_D;
    break;
    case USB_SPEED_LOW:
// J state: D+/D- = low/high, K state: D+/D- = high/low
    value |= HIGHZ_A;
    value |= AN_A;
    value |= AP_B | AP_C | AP_D;
    if (padctl.soc.supports_lp_cfg_en)
    value |= MASTER_ENABLE_B_C_D;
    break;
    default:
    value |= HIGHZ_A | HIGHZ_B | HIGHZ_C | HIGHZ_D;
    break;
    }
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK(index));
// power up the line state detectors of the pad
    value = ao_readl(priv, XUSB_AO_UTMIP_PAD_CFG(index));
    value &= ~(USBOP_VAL_PD | USBON_VAL_PD);
    ao_writel(priv, value, XUSB_AO_UTMIP_PAD_CFG(index));
    usleep_range(150, 200);
// switch the electric control of the USB2.0 pad to XUSB_AO
    value = ao_readl(priv, XUSB_AO_UTMIP_PAD_CFG(index));
    value |= FSLS_USE_XUSB_AO | TRK_CTRL_USE_XUSB_AO | RPD_CTRL_USE_XUSB_AO |
    RPU_USE_XUSB_AO | VREG_USE_XUSB_AO;
    ao_writel(priv, value, XUSB_AO_UTMIP_PAD_CFG(index));
// set the wake signaling trigger events
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~WAKE_VAL(~0);
    value |= WAKE_VAL_ANY;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// enable the wake detection
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value |= MASTER_ENABLE | LINE_WAKEUP_EN;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_disable_phy_sleepwalk(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_utmi_disable_phy_sleepwalk(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
// disable the wake detection
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~(MASTER_ENABLE | LINE_WAKEUP_EN);
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
// switch the electric control of the USB2.0 pad to XUSB vcore logic
    value = ao_readl(priv, XUSB_AO_UTMIP_PAD_CFG(index));
    value &= ~(FSLS_USE_XUSB_AO | TRK_CTRL_USE_XUSB_AO | RPD_CTRL_USE_XUSB_AO |
    RPU_USE_XUSB_AO | VREG_USE_XUSB_AO);
    ao_writel(priv, value, XUSB_AO_UTMIP_PAD_CFG(index));
// disable wake event triggers of sleepwalk logic
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    value &= ~WAKE_VAL(~0);
    value |= WAKE_VAL_NONE;
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK_CFG(index));
    if (padctl.soc.supports_lp_cfg_en) {
// disable the four stages of sleepwalk
    value = ao_readl(priv, XUSB_AO_UTMIP_SLEEPWALK(index));
    value &= ~(MASTER_ENABLE_A | MASTER_ENABLE_B_C_D);
    ao_writel(priv, value, XUSB_AO_UTMIP_SLEEPWALK(index));
    }
// power down the line state detectors of the port
    value = ao_readl(priv, XUSB_AO_UTMIP_PAD_CFG(index));
    value |= USBOP_VAL_PD | USBON_VAL_PD;
    ao_writel(priv, value, XUSB_AO_UTMIP_PAD_CFG(index));
// clear alarm of the sleepwalk logic
    value = ao_readl(priv, XUSB_AO_UTMIP_TRIGGERS(index));
    value |= CLR_WAKE_ALARM;
    ao_writel(priv, value, XUSB_AO_UTMIP_TRIGGERS(index));
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_enable_phy_wake(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_utmi_enable_phy_wake(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= USB2_PORT_WAKEUP_EVENT(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(10, 20);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= USB2_PORT_WAKE_INTERRUPT_ENABLE(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_disable_phy_wake(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_utmi_disable_phy_wake(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value &= ~USB2_PORT_WAKE_INTERRUPT_ENABLE(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(10, 20);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= USB2_PORT_WAKEUP_EVENT(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_phy_remote_wake_detected(lane: *mut tegra_xusb_lane) -> bool {
    static bool tegra186_utmi_phy_remote_wake_detected(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    if ((value & USB2_PORT_WAKE_INTERRUPT_ENABLE(index)) &&
    (value & USB2_PORT_WAKEUP_EVENT(index)))
    return true;
    return false;
    }
    static const struct tegra_xusb_lane_ops tegra186_usb2_lane_ops = {
    .probe = tegra186_usb2_lane_probe,
    .remove = tegra186_usb2_lane_remove,
    .enable_phy_sleepwalk = tegra186_utmi_enable_phy_sleepwalk,
    .disable_phy_sleepwalk = tegra186_utmi_disable_phy_sleepwalk,
    .enable_phy_wake = tegra186_utmi_enable_phy_wake,
    .disable_phy_wake = tegra186_utmi_disable_phy_wake,
    .remote_wake_detected = tegra186_utmi_phy_remote_wake_detected,
    };
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_bias_pad_power_on(padctl: *mut tegra_xusb_padctl) {
    static void tegra186_utmi_bias_pad_power_on(struct tegra_xusb_padctl *padctl)
    {
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    struct device *dev = padctl.dev;
    u32 value;
    int err;
    if (!bitmap_empty(priv.utmi_pad_enabled, TEGRA_UTMI_PAD_MAX))
    return;
    err = clk_prepare_enable(priv.usb2_trk_clk);
    if (err < 0)
    dev_warn(dev, "failed to enable USB2 trk clock: %d\n", err);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    value &= ~USB2_TRK_START_TIMER(~0);
    value |= USB2_TRK_START_TIMER(0x1e);
    value &= ~USB2_TRK_DONE_RESET_TIMER(~0);
    value |= USB2_TRK_DONE_RESET_TIMER(0xa);
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL0);
    value &= ~BIAS_PAD_PD;
    value &= ~HS_SQUELCH_LEVEL(~0);
    value |= HS_SQUELCH_LEVEL(priv.calib.hs_squelch);
    value &= ~HS_DISCON_LEVEL(~0);
    value |= HS_DISCON_LEVEL(0x7);
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL0);
    udelay(1);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    value &= ~USB2_PD_TRK;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    if (padctl.soc.poll_trk_completed) {
    err = padctl_readl_poll(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL1,
    USB2_TRK_COMPLETED, USB2_TRK_COMPLETED, 100);
    if (err) {
// The failure with polling on trk complete will not
// cause the failure of powering on the bias pad.
//
    dev_warn(dev, "failed to poll USB2 trk completed: %d\n", err);
    }
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    value |= USB2_TRK_COMPLETED;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    } else {
    udelay(100);
    }
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL2);
    if (padctl.soc.trk_update_on_idle)
    value &= ~CYA_TRK_CODE_UPDATE_ON_IDLE;
    if (padctl.soc.trk_hw_mode)
    value |= USB2_TRK_HW_MODE;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL2);
    if (!padctl.soc.trk_hw_mode)
    clk_disable_unprepare(priv.usb2_trk_clk);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_bias_pad_power_off(padctl: *mut tegra_xusb_padctl) {
    static void tegra186_utmi_bias_pad_power_off(struct tegra_xusb_padctl *padctl)
    {
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    u32 value;
    if (!bitmap_empty(priv.utmi_pad_enabled, TEGRA_UTMI_PAD_MAX))
    return;
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    value |= USB2_PD_TRK;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL1);
    if (padctl.soc.trk_hw_mode) {
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_BIAS_PAD_CTL2);
    value &= ~USB2_TRK_HW_MODE;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_BIAS_PAD_CTL2);
    clk_disable_unprepare(priv.usb2_trk_clk);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_pad_power_on(phy: *mut phy) {
    static void tegra186_utmi_pad_power_on(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    struct tegra_xusb_usb2_port *port;
    struct device *dev = padctl.dev;
    let mut index: c_uint = lane.index;
    u32 value;
    if (!phy)
    return;
    mutex_lock(&padctl.lock);
    if (test_bit(index, priv.utmi_pad_enabled)) {
    mutex_unlock(&padctl.lock);
    return;
    }
    port = tegra_xusb_find_usb2_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB2 lane %u\n", index);
    mutex_unlock(&padctl.lock);
    return;
    }
    dev_dbg(dev, "power on UTMI pad %u\n", index);
    tegra186_utmi_bias_pad_power_on(padctl);
    udelay(2);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value &= ~USB2_OTG_PD;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    value &= ~USB2_OTG_PD_DR;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    set_bit(index, priv.utmi_pad_enabled);
    mutex_unlock(&padctl.lock);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_pad_power_down(phy: *mut phy) {
    static void tegra186_utmi_pad_power_down(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    let mut index: c_uint = lane.index;
    u32 value;
    if (!phy)
    return;
    mutex_lock(&padctl.lock);
    if (!test_bit(index, priv.utmi_pad_enabled)) {
    mutex_unlock(&padctl.lock);
    return;
    }
    dev_dbg(padctl.dev, "power down UTMI pad %u\n", index);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value |= USB2_OTG_PD;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    value |= USB2_OTG_PD_DR;
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    udelay(2);
    clear_bit(index, priv.utmi_pad_enabled);
    tegra186_utmi_bias_pad_power_off(padctl);
    mutex_unlock(&padctl.lock);
    }
    static int tegra186_xusb_padctl_vbus_override(struct tegra_xusb_padctl *padctl,
    bool status)
    {
    u32 value;
    dev_dbg(padctl.dev, "%s vbus override\n", status ? "set" : "clear");
    value = padctl_readl(padctl, USB2_VBUS_ID);
    if (status) {
    value |= VBUS_OVERRIDE;
    value &= ~ID_OVERRIDE(~0);
    value |= ID_OVERRIDE_FLOATING;
    } else {
    value &= ~VBUS_OVERRIDE;
    }
    padctl_writel(padctl, value, USB2_VBUS_ID);
    return 0;
    }
    static int tegra186_xusb_padctl_id_override(struct tegra_xusb_padctl *padctl,
    struct tegra_xusb_usb2_port *port, bool status)
    {
    u32 value, id_override;
    let mut err: c_int = 0;
    dev_dbg(padctl.dev, "%s id override\n", status ? "set" : "clear");
    value = padctl_readl(padctl, USB2_VBUS_ID);
    id_override = value & ID_OVERRIDE(~0);
    if (status) {
    if (value & VBUS_OVERRIDE) {
    value &= ~VBUS_OVERRIDE;
    padctl_writel(padctl, value, USB2_VBUS_ID);
    usleep_range(1000, 2000);
    value = padctl_readl(padctl, USB2_VBUS_ID);
    }
    if (id_override != ID_OVERRIDE_GROUNDED) {
    value &= ~ID_OVERRIDE(~0);
    value |= ID_OVERRIDE_GROUNDED;
    padctl_writel(padctl, value, USB2_VBUS_ID);
    err = regulator_enable(port.supply);
    if (err) {
    dev_err(padctl.dev, "Failed to enable regulator: %d\n", err);
    return err;
    }
    }
    } else {
    if (id_override == ID_OVERRIDE_GROUNDED) {
//
// The regulator is disabled only when the role transitions
// from USB_ROLE_HOST to USB_ROLE_NONE.
//
    err = regulator_disable(port.supply);
    if (err) {
    dev_err(padctl.dev, "Failed to disable regulator: %d\n", err);
    return err;
    }
    value &= ~ID_OVERRIDE(~0);
    value |= ID_OVERRIDE_FLOATING;
    padctl_writel(padctl, value, USB2_VBUS_ID);
    }
    }
    return 0;
    }
    static int tegra186_utmi_phy_set_mode(struct phy *phy, enum phy_mode mode,
    int submode)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra_xusb_usb2_port *port = tegra_xusb_find_usb2_port(padctl,
    lane.index);
    let mut err: c_int = 0;
    mutex_lock(&padctl.lock);
    dev_dbg(&port.base.dev, "%s: mode %d", __func__, mode);
    if (mode == PHY_MODE_USB_OTG) {
    if (submode == USB_ROLE_HOST) {
    err = tegra186_xusb_padctl_id_override(padctl, port, true);
    if (err)
    goto out;
    } else if (submode == USB_ROLE_DEVICE) {
    tegra186_xusb_padctl_vbus_override(padctl, true);
    } else if (submode == USB_ROLE_NONE) {
    err = tegra186_xusb_padctl_id_override(padctl, port, false);
    if (err)
    goto out;
    tegra186_xusb_padctl_vbus_override(padctl, false);
    }
    }
    out:
    mutex_unlock(&padctl.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_phy_power_on(phy: *mut phy) -> c_int {
    static int tegra186_utmi_phy_power_on(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_usb2_lane *usb2 = to_usb2_lane(lane);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    struct tegra_xusb_usb2_port *port;
    let mut index: c_uint = lane.index;
    struct device *dev = padctl.dev;
    u32 value;
    port = tegra_xusb_find_usb2_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB2 lane %u\n", index);
    return -ENODEV;
    }
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_PAD_MUX);
    value &= ~(USB2_PORT_MASK << USB2_PORT_SHIFT(index));
    value |= (PORT_XUSB << USB2_PORT_SHIFT(index));
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_PAD_MUX);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_PORT_CAP);
    value &= ~(PORT_CAP_MASK << PORTX_CAP_SHIFT(index));
    if (port.mode == USB_DR_MODE_UNKNOWN)
    value |= (PORT_CAP_DISABLED << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_PERIPHERAL: port->mode ==) -> else {
    else if (port.mode == USB_DR_MODE_PERIPHERAL)
    value |= (PORT_CAP_DEVICE << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_HOST: port->mode ==) -> else {
    else if (port.mode == USB_DR_MODE_HOST)
    value |= (PORT_CAP_HOST << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_OTG: port->mode ==) -> else {
    else if (port.mode == USB_DR_MODE_OTG)
    value |= (PORT_CAP_OTG << PORTX_CAP_SHIFT(index));
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_PORT_CAP);
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value &= ~USB2_OTG_PD_ZI;
    value |= TERM_SEL;
    value &= ~HS_CURR_LEVEL(~0);
    if (usb2.hs_curr_level_offset) {
    int hs_current_level;
    hs_current_level = (int)priv.calib.hs_curr_level[index] +
    usb2.hs_curr_level_offset;
    if (hs_current_level < 0)
    hs_current_level = 0;
    if (hs_current_level > 0x3f)
    hs_current_level = 0x3f;
    value |= HS_CURR_LEVEL(hs_current_level);
    } else {
    value |= HS_CURR_LEVEL(priv.calib.hs_curr_level[index]);
    }
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL0(index));
    value = padctl_readl(padctl, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    value &= ~TERM_RANGE_ADJ(~0);
    value |= TERM_RANGE_ADJ(priv.calib.hs_term_range_adj[index]);
    value &= ~RPD_CTRL(~0);
    value |= RPD_CTRL(priv.calib.rpd_ctrl);
    padctl_writel(padctl, value, XUSB_PADCTL_USB2_OTG_PADX_CTL1(index));
    tegra186_utmi_pad_power_on(phy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_phy_power_off(phy: *mut phy) -> c_int {
    static int tegra186_utmi_phy_power_off(struct phy *phy)
    {
    tegra186_utmi_pad_power_down(phy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_phy_init(phy: *mut phy) -> c_int {
    static int tegra186_utmi_phy_init(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra_xusb_usb2_port *port;
    let mut index: c_uint = lane.index;
    struct device *dev = padctl.dev;
    int err;
    u32 reg;
    port = tegra_xusb_find_usb2_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB2 lane %u\n", index);
    return -ENODEV;
    }
    if (port.mode == USB_DR_MODE_OTG ||
    port.mode == USB_DR_MODE_PERIPHERAL) {
// reset VBUS&ID OVERRIDE
    reg = padctl_readl(padctl, USB2_VBUS_ID);
    reg &= ~VBUS_OVERRIDE;
    reg &= ~ID_OVERRIDE(~0);
    reg |= ID_OVERRIDE_FLOATING;
    padctl_writel(padctl, reg, USB2_VBUS_ID);
    }
    if (port.supply && port.mode == USB_DR_MODE_HOST) {
    err = regulator_enable(port.supply);
    if (err) {
    dev_err(dev, "failed to enable port %u VBUS: %d\n",
    index, err);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_utmi_phy_exit(phy: *mut phy) -> c_int {
    static int tegra186_utmi_phy_exit(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra_xusb_usb2_port *port;
    let mut index: c_uint = lane.index;
    struct device *dev = padctl.dev;
    int err;
    port = tegra_xusb_find_usb2_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB2 lane %u\n", index);
    return -ENODEV;
    }
    if (port.supply && port.mode == USB_DR_MODE_HOST) {
    err = regulator_disable(port.supply);
    if (err) {
    dev_err(dev, "failed to disable port %u VBUS: %d\n",
    index, err);
    return err;
    }
    }
    return 0;
    }
    static const struct phy_ops utmi_phy_ops = {
    .init = tegra186_utmi_phy_init,
    .exit = tegra186_utmi_phy_exit,
    .power_on = tegra186_utmi_phy_power_on,
    .power_off = tegra186_utmi_phy_power_off,
    .set_mode = tegra186_utmi_phy_set_mode,
    .owner = THIS_MODULE,
    };
    static struct tegra_xusb_pad *
    tegra186_usb2_pad_probe(struct tegra_xusb_padctl *padctl,
    const struct tegra_xusb_pad_soc *soc,
    struct device_node *np)
    {
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    struct tegra_xusb_usb2_pad *usb2;
    struct tegra_xusb_pad *pad;
    int err;
    usb2 = kzalloc_obj(*usb2);
    if (!usb2)
    return ERR_PTR(-ENOMEM);
    pad = &usb2.base;
    pad.ops = &tegra186_usb2_lane_ops;
    pad.soc = soc;
    err = tegra_xusb_pad_init(pad, padctl, np);
    if (err < 0) {
    kfree(usb2);
    goto out;
    }
    priv.usb2_trk_clk = devm_clk_get(&pad.dev, "trk");
    if (IS_ERR(priv.usb2_trk_clk)) {
    err = PTR_ERR(priv.usb2_trk_clk);
    dev_dbg(&pad.dev, "failed to get usb2 trk clock: %d\n", err);
    goto unregister;
    }
    err = tegra_xusb_pad_register(pad, &utmi_phy_ops);
    if (err < 0)
    goto unregister;
    dev_set_drvdata(&pad.dev, pad);
    return pad;
    unregister:
    device_unregister(&pad.dev);
    out:
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb2_pad_remove(pad: *mut tegra_xusb_pad) {
    static void tegra186_usb2_pad_remove(struct tegra_xusb_pad *pad)
    {
    struct tegra_xusb_usb2_pad *usb2 = to_usb2_pad(pad);
    kfree(usb2);
    }
    static const struct tegra_xusb_pad_ops tegra186_usb2_pad_ops = {
    .probe = tegra186_usb2_pad_probe,
    .remove = tegra186_usb2_pad_remove,
    };
    static const char * const tegra186_usb2_functions[] = {
    "xusb",
    };
#[no_mangle]
unsafe extern "C" fn tegra186_usb2_port_enable(port: *mut tegra_xusb_port) -> c_int {
    static int tegra186_usb2_port_enable(struct tegra_xusb_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb2_port_disable(port: *mut tegra_xusb_port) {
    static void tegra186_usb2_port_disable(struct tegra_xusb_port *port)
    {
    }
    static struct tegra_xusb_lane *
    tegra186_usb2_port_map(struct tegra_xusb_port *port)
    {
    return tegra_xusb_find_lane(port.padctl, "usb2", port.index);
    }
    static const struct tegra_xusb_port_ops tegra186_usb2_port_ops = {
    .release = tegra_xusb_usb2_port_release,
    .remove = tegra_xusb_usb2_port_remove,
    .enable = tegra186_usb2_port_enable,
    .disable = tegra186_usb2_port_disable,
    .map = tegra186_usb2_port_map,
    };
// SuperSpeed PHY support
    static struct tegra_xusb_lane *
    tegra186_usb3_lane_probe(struct tegra_xusb_pad *pad, struct device_node *np,
    unsigned int index)
    {
    struct tegra_xusb_usb3_lane *usb3;
    int err;
    usb3 = kzalloc_obj(*usb3);
    if (!usb3)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&usb3.base.list);
    usb3.base.soc = &pad.soc.lanes[index];
    usb3.base.index = index;
    usb3.base.pad = pad;
    usb3.base.np = np;
    err = tegra_xusb_lane_parse_dt(&usb3.base, np);
    if (err < 0) {
    kfree(usb3);
    return ERR_PTR(err);
    }
    return &usb3.base;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_lane_remove(lane: *mut tegra_xusb_lane) {
    static void tegra186_usb3_lane_remove(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_usb3_lane *usb3 = to_usb3_lane(lane);
    kfree(usb3);
    }
    static int tegra186_usb3_enable_phy_sleepwalk(struct tegra_xusb_lane *lane,
    enum usb_device_speed speed)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value |= SSPX_ELPG_CLAMP_EN_EARLY(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value |= SSPX_ELPG_CLAMP_EN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(250, 350);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_disable_phy_sleepwalk(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_usb3_disable_phy_sleepwalk(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value &= ~SSPX_ELPG_CLAMP_EN_EARLY(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value &= ~SSPX_ELPG_CLAMP_EN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_enable_phy_wake(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_usb3_enable_phy_wake(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= SS_PORT_WAKEUP_EVENT(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(10, 20);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= SS_PORT_WAKE_INTERRUPT_ENABLE(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_disable_phy_wake(lane: *mut tegra_xusb_lane) -> c_int {
    static int tegra186_usb3_disable_phy_wake(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value &= ~SS_PORT_WAKE_INTERRUPT_ENABLE(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    usleep_range(10, 20);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    value &= ~ALL_WAKE_EVENTS;
    value |= SS_PORT_WAKEUP_EVENT(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_phy_remote_wake_detected(lane: *mut tegra_xusb_lane) -> bool {
    static bool tegra186_usb3_phy_remote_wake_detected(struct tegra_xusb_lane *lane)
    {
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    let mut index: c_uint = lane.index;
    u32 value;
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM);
    if ((value & SS_PORT_WAKE_INTERRUPT_ENABLE(index)) && (value & SS_PORT_WAKEUP_EVENT(index)))
    return true;
    return false;
    }
    static const struct tegra_xusb_lane_ops tegra186_usb3_lane_ops = {
    .probe = tegra186_usb3_lane_probe,
    .remove = tegra186_usb3_lane_remove,
    .enable_phy_sleepwalk = tegra186_usb3_enable_phy_sleepwalk,
    .disable_phy_sleepwalk = tegra186_usb3_disable_phy_sleepwalk,
    .enable_phy_wake = tegra186_usb3_enable_phy_wake,
    .disable_phy_wake = tegra186_usb3_disable_phy_wake,
    .remote_wake_detected = tegra186_usb3_phy_remote_wake_detected,
    };
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_port_enable(port: *mut tegra_xusb_port) -> c_int {
    static int tegra186_usb3_port_enable(struct tegra_xusb_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_port_disable(port: *mut tegra_xusb_port) {
    static void tegra186_usb3_port_disable(struct tegra_xusb_port *port)
    {
    }
    static struct tegra_xusb_lane *
    tegra186_usb3_port_map(struct tegra_xusb_port *port)
    {
    return tegra_xusb_find_lane(port.padctl, "usb3", port.index);
    }
    static const struct tegra_xusb_port_ops tegra186_usb3_port_ops = {
    .release = tegra_xusb_usb3_port_release,
    .enable = tegra186_usb3_port_enable,
    .disable = tegra186_usb3_port_disable,
    .map = tegra186_usb3_port_map,
    };
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_phy_power_on(phy: *mut phy) -> c_int {
    static int tegra186_usb3_phy_power_on(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra_xusb_usb3_port *port;
    struct tegra_xusb_usb2_port *usb2;
    let mut index: c_uint = lane.index;
    struct device *dev = padctl.dev;
    u32 value;
    port = tegra_xusb_find_usb3_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB3 lane %u\n", index);
    return -ENODEV;
    }
    usb2 = tegra_xusb_find_usb2_port(padctl, port.port);
    if (!usb2) {
    dev_err(dev, "no companion port found for USB3 lane %u\n",
    index);
    return -ENODEV;
    }
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_SS_PORT_CAP);
    value &= ~(PORT_CAP_MASK << PORTX_CAP_SHIFT(index));
    if (usb2.mode == USB_DR_MODE_UNKNOWN)
    value |= (PORT_CAP_DISABLED << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_PERIPHERAL: usb2->mode ==) -> else {
    else if (usb2.mode == USB_DR_MODE_PERIPHERAL)
    value |= (PORT_CAP_DEVICE << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_HOST: usb2->mode ==) -> else {
    else if (usb2.mode == USB_DR_MODE_HOST)
    value |= (PORT_CAP_HOST << PORTX_CAP_SHIFT(index));
#[no_mangle]
pub unsafe extern "C" fn if(USB_DR_MODE_OTG: usb2->mode ==) -> else {
    else if (usb2.mode == USB_DR_MODE_OTG)
    value |= (PORT_CAP_OTG << PORTX_CAP_SHIFT(index));
    padctl_writel(padctl, value, XUSB_PADCTL_SS_PORT_CAP);
    if (padctl.soc.supports_gen2 && port.disable_gen2) {
    value = padctl_readl(padctl, XUSB_PADCTL_SS_PORT_CFG);
    value &= ~(PORTX_SPEED_SUPPORT_MASK <<
    PORTX_SPEED_SUPPORT_SHIFT(index));
    value |= (PORT_SPEED_SUPPORT_GEN1 <<
    PORTX_SPEED_SUPPORT_SHIFT(index));
    padctl_writel(padctl, value, XUSB_PADCTL_SS_PORT_CFG);
    }
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value &= ~SSPX_ELPG_VCORE_DOWN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value &= ~SSPX_ELPG_CLAMP_EN_EARLY(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value &= ~SSPX_ELPG_CLAMP_EN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_phy_power_off(phy: *mut phy) -> c_int {
    static int tegra186_usb3_phy_power_off(struct phy *phy)
    {
    struct tegra_xusb_lane *lane = phy_get_drvdata(phy);
    struct tegra_xusb_padctl *padctl = lane.pad.padctl;
    struct tegra_xusb_usb3_port *port;
    let mut index: c_uint = lane.index;
    struct device *dev = padctl.dev;
    u32 value;
    port = tegra_xusb_find_usb3_port(padctl, index);
    if (!port) {
    dev_err(dev, "no port found for USB3 lane %u\n", index);
    return -ENODEV;
    }
    mutex_lock(&padctl.lock);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value |= SSPX_ELPG_CLAMP_EN_EARLY(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(100, 200);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value |= SSPX_ELPG_CLAMP_EN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    usleep_range(250, 350);
    value = padctl_readl(padctl, XUSB_PADCTL_ELPG_PROGRAM_1);
    value |= SSPX_ELPG_VCORE_DOWN(index);
    padctl_writel(padctl, value, XUSB_PADCTL_ELPG_PROGRAM_1);
    mutex_unlock(&padctl.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_phy_init(phy: *mut phy) -> c_int {
    static int tegra186_usb3_phy_init(struct phy *phy)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_phy_exit(phy: *mut phy) -> c_int {
    static int tegra186_usb3_phy_exit(struct phy *phy)
    {
    return 0;
    }
    static const struct phy_ops usb3_phy_ops = {
    .init = tegra186_usb3_phy_init,
    .exit = tegra186_usb3_phy_exit,
    .power_on = tegra186_usb3_phy_power_on,
    .power_off = tegra186_usb3_phy_power_off,
    .owner = THIS_MODULE,
    };
    static struct tegra_xusb_pad *
    tegra186_usb3_pad_probe(struct tegra_xusb_padctl *padctl,
    const struct tegra_xusb_pad_soc *soc,
    struct device_node *np)
    {
    struct tegra_xusb_usb3_pad *usb3;
    struct tegra_xusb_pad *pad;
    int err;
    usb3 = kzalloc_obj(*usb3);
    if (!usb3)
    return ERR_PTR(-ENOMEM);
    pad = &usb3.base;
    pad.ops = &tegra186_usb3_lane_ops;
    pad.soc = soc;
    err = tegra_xusb_pad_init(pad, padctl, np);
    if (err < 0) {
    kfree(usb3);
    goto out;
    }
    err = tegra_xusb_pad_register(pad, &usb3_phy_ops);
    if (err < 0)
    goto unregister;
    dev_set_drvdata(&pad.dev, pad);
    return pad;
    unregister:
    device_unregister(&pad.dev);
    out:
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_usb3_pad_remove(pad: *mut tegra_xusb_pad) {
    static void tegra186_usb3_pad_remove(struct tegra_xusb_pad *pad)
    {
    struct tegra_xusb_usb2_pad *usb2 = to_usb2_pad(pad);
    kfree(usb2);
    }
    static const struct tegra_xusb_pad_ops tegra186_usb3_pad_ops = {
    .probe = tegra186_usb3_pad_probe,
    .remove = tegra186_usb3_pad_remove,
    };
    static const char * const tegra186_usb3_functions[] = {
    "xusb",
    };
    static int
    tegra186_xusb_read_fuse_calibration(struct tegra186_xusb_padctl *padctl)
    {
    const struct tegra_xusb_padctl_soc *soc = padctl.base.soc;
    struct device *dev = padctl.base.dev;
    unsigned int i, count;
    u32 value, *level;
    u32 *hs_term_range_adj;
    int err;
    count = soc.ports.usb2.count;
    level = devm_kcalloc(dev, count, sizeof(u32), GFP_KERNEL);
    if (!level)
    return -ENOMEM;
    hs_term_range_adj = devm_kcalloc(dev, count, sizeof(u32), GFP_KERNEL);
    if (!hs_term_range_adj)
    return -ENOMEM;
    err = tegra_fuse_readl(TEGRA_FUSE_SKU_CALIB_0, &value);
    if (err)
    return dev_err_probe(dev, err,
    "failed to read calibration fuse\n");
    dev_dbg(dev, "FUSE_USB_CALIB_0 %#x\n", value);
    for (i = 0; i < count; i++)
    level[i] = (value >> HS_CURR_LEVEL_PADX_SHIFT(i)) &
    HS_CURR_LEVEL_PAD_MASK;
    padctl.calib.hs_curr_level = level;
    padctl.calib.hs_squelch = (value >> HS_SQUELCH_SHIFT) &
    HS_SQUELCH_MASK;
    hs_term_range_adj[0] = (value >> HS_TERM_RANGE_ADJ_PADX_SHIFT(0)) &
    HS_TERM_RANGE_ADJ_PAD_MASK;
    err = tegra_fuse_readl(TEGRA_FUSE_USB_CALIB_EXT_0, &value);
    if (err) {
    dev_err(dev, "failed to read calibration fuse: %d\n", err);
    return err;
    }
    dev_dbg(dev, "FUSE_USB_CALIB_EXT_0 %#x\n", value);
    padctl.calib.rpd_ctrl = (value >> RPD_CTRL_SHIFT) & RPD_CTRL_MASK;
    for (i = 1; i < count; i++) {
    if (soc.has_per_pad_term)
    hs_term_range_adj[i] =
    (value >> HS_TERM_RANGE_ADJ_PADX_SHIFT(i)) &
    HS_TERM_RANGE_ADJ_PAD_MASK;
    else
    hs_term_range_adj[i] = hs_term_range_adj[0];
    }
    padctl.calib.hs_term_range_adj = hs_term_range_adj;
    return 0;
    }
    static struct tegra_xusb_padctl *
    tegra186_xusb_padctl_probe(struct device *dev,
    const struct tegra_xusb_padctl_soc *soc)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct tegra186_xusb_padctl *priv;
    struct resource *res;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return ERR_PTR(-ENOMEM);
    priv.base.dev = dev;
    priv.base.soc = soc;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "ao");
    priv.ao_regs = devm_ioremap_resource(dev, res);
    if (IS_ERR(priv.ao_regs))
    return ERR_CAST(priv.ao_regs);
    err = tegra186_xusb_read_fuse_calibration(priv);
    if (err < 0)
    return ERR_PTR(err);
    return &priv.base;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_xusb_padctl_save(padctl: *mut tegra_xusb_padctl) {
    static void tegra186_xusb_padctl_save(struct tegra_xusb_padctl *padctl)
    {
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    priv.context.vbus_id = padctl_readl(padctl, USB2_VBUS_ID);
    priv.context.usb2_pad_mux = padctl_readl(padctl, XUSB_PADCTL_USB2_PAD_MUX);
    priv.context.usb2_port_cap = padctl_readl(padctl, XUSB_PADCTL_USB2_PORT_CAP);
    priv.context.ss_port_cap = padctl_readl(padctl, XUSB_PADCTL_SS_PORT_CAP);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_xusb_padctl_restore(padctl: *mut tegra_xusb_padctl) {
    static void tegra186_xusb_padctl_restore(struct tegra_xusb_padctl *padctl)
    {
    struct tegra186_xusb_padctl *priv = to_tegra186_xusb_padctl(padctl);
    padctl_writel(padctl, priv.context.usb2_pad_mux, XUSB_PADCTL_USB2_PAD_MUX);
    padctl_writel(padctl, priv.context.usb2_port_cap, XUSB_PADCTL_USB2_PORT_CAP);
    padctl_writel(padctl, priv.context.ss_port_cap, XUSB_PADCTL_SS_PORT_CAP);
    padctl_writel(padctl, priv.context.vbus_id, USB2_VBUS_ID);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_xusb_padctl_suspend_noirq(padctl: *mut tegra_xusb_padctl) -> c_int {
    static int tegra186_xusb_padctl_suspend_noirq(struct tegra_xusb_padctl *padctl)
    {
    tegra186_xusb_padctl_save(padctl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_xusb_padctl_resume_noirq(padctl: *mut tegra_xusb_padctl) -> c_int {
    static int tegra186_xusb_padctl_resume_noirq(struct tegra_xusb_padctl *padctl)
    {
    tegra186_xusb_padctl_restore(padctl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_xusb_padctl_remove(padctl: *mut tegra_xusb_padctl) {
    static void tegra186_xusb_padctl_remove(struct tegra_xusb_padctl *padctl)
    {
    }
    static const struct tegra_xusb_padctl_ops tegra186_xusb_padctl_ops = {
    .probe = tegra186_xusb_padctl_probe,
    .remove = tegra186_xusb_padctl_remove,
    .suspend_noirq = tegra186_xusb_padctl_suspend_noirq,
    .resume_noirq = tegra186_xusb_padctl_resume_noirq,
    .vbus_override = tegra186_xusb_padctl_vbus_override,
    .utmi_pad_power_on = tegra186_utmi_pad_power_on,
    .utmi_pad_power_down = tegra186_utmi_pad_power_down,
    };

    static const char * const tegra186_xusb_padctl_supply_names[] = {
    "avdd-pll-erefeut",
    "avdd-usb",
    "vclamp-usb",
    "vddio-hsic",
    };
    static const struct tegra_xusb_lane_soc tegra186_usb2_lanes[] = {
    TEGRA186_LANE("usb2-0", 0,  0, 0, usb2),
    TEGRA186_LANE("usb2-1", 0,  0, 0, usb2),
    TEGRA186_LANE("usb2-2", 0,  0, 0, usb2),
    };
    static const struct tegra_xusb_pad_soc tegra186_usb2_pad = {
    .name = "usb2",
    .num_lanes = ARRAY_SIZE(tegra186_usb2_lanes),
    .lanes = tegra186_usb2_lanes,
    .ops = &tegra186_usb2_pad_ops,
    };
    static const struct tegra_xusb_lane_soc tegra186_usb3_lanes[] = {
    TEGRA186_LANE("usb3-0", 0,  0, 0, usb3),
    TEGRA186_LANE("usb3-1", 0,  0, 0, usb3),
    TEGRA186_LANE("usb3-2", 0,  0, 0, usb3),
    };
    static const struct tegra_xusb_pad_soc tegra186_usb3_pad = {
    .name = "usb3",
    .num_lanes = ARRAY_SIZE(tegra186_usb3_lanes),
    .lanes = tegra186_usb3_lanes,
    .ops = &tegra186_usb3_pad_ops,
    };
    static const struct tegra_xusb_pad_soc * const tegra186_pads[] = {
    &tegra186_usb2_pad,
    &tegra186_usb3_pad,

    &tegra186_hsic_pad,

    };
    const struct tegra_xusb_padctl_soc tegra186_xusb_padctl_soc = {
    .num_pads = ARRAY_SIZE(tegra186_pads),
    .pads = tegra186_pads,
    .ports = {
    .usb2 = {
    .ops = &tegra186_usb2_port_ops,
    .count = 3,
    },

    .hsic = {
    .ops = &tegra186_hsic_port_ops,
    .count = 1,
    },

    .usb3 = {
    .ops = &tegra186_usb3_port_ops,
    .count = 3,
    },
    },
    .ops = &tegra186_xusb_padctl_ops,
    .supply_names = tegra186_xusb_padctl_supply_names,
    .num_supplies = ARRAY_SIZE(tegra186_xusb_padctl_supply_names),
    };
    EXPORT_SYMBOL_GPL(tegra186_xusb_padctl_soc);

    IS_ENABLED(CONFIG_ARCH_TEGRA_234_SOC)
    static const char * const tegra194_xusb_padctl_supply_names[] = {
    "avdd-usb",
    "vclamp-usb",
    };
    static const struct tegra_xusb_lane_soc tegra194_usb2_lanes[] = {
    TEGRA186_LANE("usb2-0", 0,  0, 0, usb2),
    TEGRA186_LANE("usb2-1", 0,  0, 0, usb2),
    TEGRA186_LANE("usb2-2", 0,  0, 0, usb2),
    TEGRA186_LANE("usb2-3", 0,  0, 0, usb2),
    };
    static const struct tegra_xusb_pad_soc tegra194_usb2_pad = {
    .name = "usb2",
    .num_lanes = ARRAY_SIZE(tegra194_usb2_lanes),
    .lanes = tegra194_usb2_lanes,
    .ops = &tegra186_usb2_pad_ops,
    };
    static const struct tegra_xusb_lane_soc tegra194_usb3_lanes[] = {
    TEGRA186_LANE("usb3-0", 0,  0, 0, usb3),
    TEGRA186_LANE("usb3-1", 0,  0, 0, usb3),
    TEGRA186_LANE("usb3-2", 0,  0, 0, usb3),
    TEGRA186_LANE("usb3-3", 0,  0, 0, usb3),
    };
    static const struct tegra_xusb_pad_soc tegra194_usb3_pad = {
    .name = "usb3",
    .num_lanes = ARRAY_SIZE(tegra194_usb3_lanes),
    .lanes = tegra194_usb3_lanes,
    .ops = &tegra186_usb3_pad_ops,
    };
    static const struct tegra_xusb_pad_soc * const tegra194_pads[] = {
    &tegra194_usb2_pad,
    &tegra194_usb3_pad,
    };
    const struct tegra_xusb_padctl_soc tegra194_xusb_padctl_soc = {
    .num_pads = ARRAY_SIZE(tegra194_pads),
    .pads = tegra194_pads,
    .ports = {
    .usb2 = {
    .ops = &tegra186_usb2_port_ops,
    .count = 4,
    },
    .usb3 = {
    .ops = &tegra186_usb3_port_ops,
    .count = 4,
    },
    },
    .ops = &tegra186_xusb_padctl_ops,
    .supply_names = tegra194_xusb_padctl_supply_names,
    .num_supplies = ARRAY_SIZE(tegra194_xusb_padctl_supply_names),
    .supports_gen2 = true,
    .poll_trk_completed = true,
    .has_per_pad_term = true,
    };
    EXPORT_SYMBOL_GPL(tegra194_xusb_padctl_soc);
    const struct tegra_xusb_padctl_soc tegra234_xusb_padctl_soc = {
    .num_pads = ARRAY_SIZE(tegra194_pads),
    .pads = tegra194_pads,
    .ports = {
    .usb2 = {
    .ops = &tegra186_usb2_port_ops,
    .count = 4,
    },
    .usb3 = {
    .ops = &tegra186_usb3_port_ops,
    .count = 4,
    },
    },
    .ops = &tegra186_xusb_padctl_ops,
    .supply_names = tegra194_xusb_padctl_supply_names,
    .num_supplies = ARRAY_SIZE(tegra194_xusb_padctl_supply_names),
    .supports_gen2 = true,
    .poll_trk_completed = true,
    .trk_hw_mode = false,
    .trk_update_on_idle = true,
    .supports_lp_cfg_en = true,
    .has_per_pad_term = true,
    };
    EXPORT_SYMBOL_GPL(tegra234_xusb_padctl_soc);

    MODULE_AUTHOR("JC Kuo <jckuo@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra186 XUSB Pad Controller driver");
    MODULE_LICENSE("GPL v2");
