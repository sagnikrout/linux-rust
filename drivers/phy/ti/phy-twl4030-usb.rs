//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ti/phy-twl4030-usb.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// twl4030_usb - TWL4030 USB transceiver, talking to OMAP OTG controller
//
// Copyright (C) 2004-2007 Texas Instruments
// Copyright (C) 2008 Nokia Corporation
// Contact: Felipe Balbi <felipe.balbi@nokia.com>
//
// Current status:
// - HS USB ULPI mode works.
// - 3-pin mode support may be added in future.
//

// Register defines
pub const MCPC_CTRL: c_uint = 0x30;

pub const MCPC_IO_CTRL: c_uint = 0x33;

pub const MCPC_CTRL2: c_uint = 0x36;

pub const OTHER_FUNC_CTRL: c_uint = 0x80;

pub const OTHER_IFC_CTRL: c_uint = 0x83;

pub const OTHER_INT_EN_RISE: c_uint = 0x86;
pub const OTHER_INT_EN_FALL: c_uint = 0x89;
pub const OTHER_INT_STS: c_uint = 0x8C;
pub const OTHER_INT_LATCH: c_uint = 0x8D;

pub const ID_STATUS: c_uint = 0x96;

pub const POWER_CTRL: c_uint = 0xAC;

pub const OTHER_IFC_CTRL2: c_uint = 0xAF;

pub const REG_CTRL_EN: c_uint = 0xB2;
pub const REG_CTRL_ERROR: c_uint = 0xB5;

pub const OTHER_FUNC_CTRL2: c_uint = 0xB8;

// following registers do not have separate _clr and _set registers
pub const VBUS_DEBOUNCE: c_uint = 0xC0;
pub const ID_DEBOUNCE: c_uint = 0xC1;
pub const VBAT_TIMER: c_uint = 0xD3;
pub const PHY_PWR_CTRL: c_uint = 0xFD;

pub const PHY_CLK_CTRL: c_uint = 0xFE;

pub const PHY_CLK_CTRL_STS: c_uint = 0xFF;

// In module TWL_MODULE_PM_MASTER
pub const STS_HW_CONDITIONS: c_uint = 0x0F;
// In module TWL_MODULE_PM_RECEIVER
pub const VUSB_DEDICATED1: c_uint = 0x7D;
pub const VUSB_DEDICATED2: c_uint = 0x7E;
pub const VUSB1V5_DEV_GRP: c_uint = 0x71;
pub const VUSB1V5_TYPE: c_uint = 0x72;
pub const VUSB1V5_REMAP: c_uint = 0x73;
pub const VUSB1V8_DEV_GRP: c_uint = 0x74;
pub const VUSB1V8_TYPE: c_uint = 0x75;
pub const VUSB1V8_REMAP: c_uint = 0x76;
pub const VUSB3V1_DEV_GRP: c_uint = 0x77;
pub const VUSB3V1_TYPE: c_uint = 0x78;
pub const VUSB3V1_REMAP: c_uint = 0x79;
// In module TWL4030_MODULE_INTBR
pub const PMBR1: c_uint = 0x0D;

    static irqreturn_t twl4030_usb_irq(int irq, void *_twl);
//
// If VBUS is valid or ID is ground, then we know a
// cable is present and we need to be runtime-enabled
//
#[no_mangle]
pub unsafe extern "C" fn cable_present(stat: enum musb_vbus_id_status) -> bool {
    static inline bool cable_present(enum musb_vbus_id_status stat)
    {
    return stat == MUSB_VBUS_VALID ||
    stat == MUSB_ID_GROUND;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_usb {
    pub phy: usb_phy,
    pub dev: *mut device,
// TWL4030 internal USB regulator supplies
    pub usb1v5: *mut regulator,
    pub usb1v8: *mut regulator,
    pub usb3v1: *mut regulator,
// for vbus reporting with irqs disabled
    pub lock: mutex,
// pin configuration
    pub usb_mode: enum twl4030_usb_mode,
    pub irq: c_int,
    pub linkstat: enum musb_vbus_id_status,
    pub connected: core::sync::atomic::AtomicI32,
    pub vbus_supplied: bool,
    pub musb_mailbox_pending: bool,
    pub runtime_suspended:1: c_ulong,
    pub needs_resume:1: c_ulong,
    pub id_workaround_work: delayed_work,
}

// internal define on top of container_of

// -------------------------------------------------------------------------
    static int twl4030_i2c_write_u8_verify(struct twl4030_usb *twl,
    u8 module, u8 data, u8 address)
    {
    let mut check: u8 = 0xFF;
    if ((twl_i2c_write_u8(module, data, address) >= 0) &&
    (twl_i2c_read_u8(module, &check, address) >= 0) &&
    (check == data))
    return 0;
    dev_dbg(twl.dev, "Write%d[%d,0x%x] wrote %02x but read %02x\n",
    1, module, address, check, data);
// Failed once: Try again
    if ((twl_i2c_write_u8(module, data, address) >= 0) &&
    (twl_i2c_read_u8(module, &check, address) >= 0) &&
    (check == data))
    return 0;
    dev_dbg(twl.dev, "Write%d[%d,0x%x] wrote %02x but read %02x\n",
    2, module, address, check, data);
// Failed again: Return error
    return -EBUSY;
    }

    twl4030_i2c_write_u8_verify(twl, TWL_MODULE_USB, (data), (address))
    static inline int twl4030_usb_write(struct twl4030_usb *twl,
    u8 address, u8 data)
    {
    let mut ret: c_int = 0;
    ret = twl_i2c_write_u8(TWL_MODULE_USB, data, address);
    if (ret < 0)
    dev_dbg(twl.dev,
    "TWL4030:USB:Write[0x%x] Error %d\n", address, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn twl4030_readb(twl: *mut twl4030_usb, module: u8, address: u8) -> c_int {
    static inline int twl4030_readb(struct twl4030_usb *twl, u8 module, u8 address)
    {
    u8 data;
    let mut ret: c_int = 0;
    ret = twl_i2c_read_u8(module, &data, address);
    if (ret >= 0)
    ret = data;
    else
    dev_dbg(twl.dev,
    "TWL4030:readb[0x%x,0x%x] Error %d\n",
    module, address, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn twl4030_usb_read(twl: *mut twl4030_usb, address: u8) -> c_int {
    static inline int twl4030_usb_read(struct twl4030_usb *twl, u8 address)
    {
    return twl4030_readb(twl, TWL_MODULE_USB, address);
    }
// -------------------------------------------------------------------------
    static inline int
    twl4030_usb_set_bits(struct twl4030_usb *twl, u8 reg, u8 bits)
    {
    return twl4030_usb_write(twl, ULPI_SET(reg), bits);
    }
    static inline int
    twl4030_usb_clear_bits(struct twl4030_usb *twl, u8 reg, u8 bits)
    {
    return twl4030_usb_write(twl, ULPI_CLR(reg), bits);
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn twl4030_is_driving_vbus(twl: *mut twl4030_usb) -> bool {
    static bool twl4030_is_driving_vbus(struct twl4030_usb *twl)
    {
    int ret;
    ret = twl4030_usb_read(twl, PHY_CLK_CTRL_STS);
    if (ret < 0 || !(ret & PHY_DPLL_CLK))
//
// if clocks are off, registers are not updated,
// but we can assume we don't drive VBUS in this case
//
    return false;
    ret = twl4030_usb_read(twl, ULPI_OTG_CTRL);
    if (ret < 0)
    return false;
    return (ret & (ULPI_OTG_DRVVBUS | ULPI_OTG_CHRGVBUS)) ? true : false;
    }
    static enum musb_vbus_id_status
    twl4030_usb_linkstat(struct twl4030_usb *twl)
    {
    int	status;
    let mut linkstat: enum musb_vbus_id_status = MUSB_UNKNOWN;
    twl.vbus_supplied = false;
//
// For ID/VBUS sensing, see manual section 15.4.8 ...
// except when using only battery backup power, two
// comparators produce VBUS_PRES and ID_PRES signals,
// which don't match docs elsewhere.  But ... BIT(7)
// and BIT(2) of STS_HW_CONDITIONS, respectively, do
// seem to match up.  If either is true the USB_PRES
// signal is active, the OTG module is activated, and
// its interrupt may be raised (may wake the system).
//
    status = twl4030_readb(twl, TWL_MODULE_PM_MASTER, STS_HW_CONDITIONS);
    if (status < 0)
    dev_err(twl.dev, "USB link status err %d\n", status);
#[no_mangle]
pub unsafe extern "C" fn if(BIT(2)): status & (BIT(7) |) -> else {
    if (status & BIT(7)) {
    if (twl4030_is_driving_vbus(twl))
    status &= ~BIT(7);
    else
    twl.vbus_supplied = true;
    }
    if (status & BIT(2))
    linkstat = MUSB_ID_GROUND;
#[no_mangle]
pub unsafe extern "C" fn if(BIT(7): status &) -> else {
    else if (status & BIT(7))
    linkstat = MUSB_VBUS_VALID;
    else
    linkstat = MUSB_VBUS_OFF;
    } else {
    if (twl.linkstat != MUSB_UNKNOWN)
    linkstat = MUSB_VBUS_OFF;
    }
    kobject_uevent(&twl.dev.kobj, linkstat == MUSB_VBUS_VALID
    ? KOBJ_ONLINE : KOBJ_OFFLINE);
    dev_dbg(twl.dev, "HW_CONDITIONS 0x%02x/%d; link %d\n",
    status, status, linkstat);
// REVISIT this assumes host and peripheral controllers
// are registered, and that both are active...
//
    return linkstat;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_set_mode(twl: *mut twl4030_usb, mode: c_int) {
    static void twl4030_usb_set_mode(struct twl4030_usb *twl, int mode)
    {
    twl.usb_mode = mode;
    switch (mode) {
    case T2_USB_MODE_ULPI:
    twl4030_usb_clear_bits(twl, ULPI_IFC_CTRL,
    ULPI_IFC_CTRL_CARKITMODE);
    twl4030_usb_set_bits(twl, POWER_CTRL, POWER_CTRL_OTG_ENAB);
    twl4030_usb_clear_bits(twl, ULPI_FUNC_CTRL,
    ULPI_FUNC_CTRL_XCVRSEL_MASK |
    ULPI_FUNC_CTRL_OPMODE_MASK);
    break;
    case -1:
// FIXME: power on defaults
    break;
    default:
    dev_err(twl.dev, "unsupported T2 transceiver mode %d\n",
    mode);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn twl4030_i2c_access(twl: *mut twl4030_usb, on: c_int) {
    static void twl4030_i2c_access(struct twl4030_usb *twl, int on)
    {
    unsigned long timeout;
    let mut val: c_int = twl4030_usb_read(twl, PHY_CLK_CTRL);
    if (val >= 0) {
    if (on) {
// enable DPLL to access PHY registers over I2C
    val |= REQ_PHY_DPLL_CLK;
    WARN_ON(twl4030_usb_write_verify(twl, PHY_CLK_CTRL,
    (u8)val) < 0);
    timeout = jiffies + HZ;
    while (!(twl4030_usb_read(twl, PHY_CLK_CTRL_STS) &
    PHY_DPLL_CLK)
    && time_before(jiffies, timeout))
    udelay(10);
    if (!(twl4030_usb_read(twl, PHY_CLK_CTRL_STS) &
    PHY_DPLL_CLK))
    dev_err(twl.dev, "Timeout setting T2 HSUSB "
    "PHY DPLL clock\n");
    } else {
// let ULPI control the DPLL clock
    val &= ~REQ_PHY_DPLL_CLK;
    WARN_ON(twl4030_usb_write_verify(twl, PHY_CLK_CTRL,
    (u8)val) < 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __twl4030_phy_power(twl: *mut twl4030_usb, on: c_int) {
    static void __twl4030_phy_power(struct twl4030_usb *twl, int on)
    {
    let mut pwr: u8 = twl4030_usb_read(twl, PHY_PWR_CTRL);
    if (on)
    pwr &= ~PHY_PWR_PHYPWD;
    else
    pwr |= PHY_PWR_PHYPWD;
    WARN_ON(twl4030_usb_write_verify(twl, PHY_PWR_CTRL, pwr) < 0);
    }
    static int twl4030_usb_runtime_suspend(struct device *dev);
    static int twl4030_usb_runtime_resume(struct device *dev);
#[no_mangle]
unsafe extern "C" fn twl4030_usb_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused twl4030_usb_suspend(struct device *dev)
    {
    struct twl4030_usb *twl = dev_get_drvdata(dev);
//
// we need enabled runtime on resume,
// so turn irq off here, so we do not get it early
// note: wakeup on usb plug works independently of this
//
    dev_dbg(twl.dev, "%s\n", __func__);
    disable_irq(twl.irq);
    if (!twl.runtime_suspended && !atomic_read(&twl.connected)) {
    twl4030_usb_runtime_suspend(dev);
    twl.needs_resume = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused twl4030_usb_resume(struct device *dev)
    {
    struct twl4030_usb *twl = dev_get_drvdata(dev);
    dev_dbg(twl.dev, "%s\n", __func__);
    enable_irq(twl.irq);
    if (twl.needs_resume)
    twl4030_usb_runtime_resume(dev);
// check whether cable status changed
    twl4030_usb_irq(0, twl);
    twl.runtime_suspended = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused twl4030_usb_runtime_suspend(struct device *dev)
    {
    struct twl4030_usb *twl = dev_get_drvdata(dev);
    dev_dbg(twl.dev, "%s\n", __func__);
    __twl4030_phy_power(twl, 0);
    regulator_disable(twl.usb1v5);
    regulator_disable(twl.usb1v8);
    regulator_disable(twl.usb3v1);
    twl.runtime_suspended = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused twl4030_usb_runtime_resume(struct device *dev)
    {
    struct twl4030_usb *twl = dev_get_drvdata(dev);
    int res;
    dev_dbg(twl.dev, "%s\n", __func__);
    res = regulator_enable(twl.usb3v1);
    if (res)
    dev_err(twl.dev, "Failed to enable usb3v1\n");
    res = regulator_enable(twl.usb1v8);
    if (res)
    dev_err(twl.dev, "Failed to enable usb1v8\n");
//
// Disabling usb3v1 regulator (= writing 0 to VUSB3V1_DEV_GRP
// in twl4030) resets the VUSB_DEDICATED2 register. This reset
// enables VUSB3V1_SLEEP bit that remaps usb3v1 ACTIVE state to
// SLEEP. We work around this by clearing the bit after usv3v1
// is re-activated. This ensures that VUSB3V1 is really active.
//
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB_DEDICATED2);
    res = regulator_enable(twl.usb1v5);
    if (res)
    dev_err(twl.dev, "Failed to enable usb1v5\n");
    __twl4030_phy_power(twl, 1);
    twl4030_usb_write(twl, PHY_CLK_CTRL,
    twl4030_usb_read(twl, PHY_CLK_CTRL) |
    (PHY_CLK_CTRL_CLOCKGATING_EN |
    PHY_CLK_CTRL_CLK32K_EN));
    twl4030_i2c_access(twl, 1);
    twl4030_usb_set_mode(twl, twl.usb_mode);
    if (twl.usb_mode == T2_USB_MODE_ULPI)
    twl4030_i2c_access(twl, 0);
//
// According to the TPS65950 TRM, there has to be at least 50ms
// delay between setting POWER_CTRL_OTG_ENAB and enabling charging
// so wait here so that a fully enabled phy can be expected after
// resume
//
    msleep(50);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_phy_power_off(phy: *mut phy) -> c_int {
    static int twl4030_phy_power_off(struct phy *phy)
    {
    struct twl4030_usb *twl = phy_get_drvdata(phy);
    dev_dbg(twl.dev, "%s\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_phy_power_on(phy: *mut phy) -> c_int {
    static int twl4030_phy_power_on(struct phy *phy)
    {
    struct twl4030_usb *twl = phy_get_drvdata(phy);
    dev_dbg(twl.dev, "%s\n", __func__);
    pm_runtime_get_sync(twl.dev);
    schedule_delayed_work(&twl.id_workaround_work, HZ);
    pm_runtime_mark_last_busy(twl.dev);
    pm_runtime_put_autosuspend(twl.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_ldo_init(twl: *mut twl4030_usb) -> c_int {
    static int twl4030_usb_ldo_init(struct twl4030_usb *twl)
    {
// Enable writing to power configuration registers
    twl_i2c_write_u8(TWL_MODULE_PM_MASTER, TWL4030_PM_MASTER_KEY_CFG1,
    TWL4030_PM_MASTER_PROTECT_KEY);
    twl_i2c_write_u8(TWL_MODULE_PM_MASTER, TWL4030_PM_MASTER_KEY_CFG2,
    TWL4030_PM_MASTER_PROTECT_KEY);
// Keep VUSB3V1 LDO in sleep state until VBUS/ID change detected
// twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB_DEDICATED2);
// input to VUSB3V1 LDO is from VBAT, not VBUS
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0x14, VUSB_DEDICATED1);
// Initialize 3.1V regulator
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB3V1_DEV_GRP);
    twl.usb3v1 = devm_regulator_get(twl.dev, "usb3v1");
    if (IS_ERR(twl.usb3v1))
    return -ENODEV;
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB3V1_TYPE);
// Initialize 1.5V regulator
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB1V5_DEV_GRP);
    twl.usb1v5 = devm_regulator_get(twl.dev, "usb1v5");
    if (IS_ERR(twl.usb1v5))
    return -ENODEV;
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB1V5_TYPE);
// Initialize 1.8V regulator
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB1V8_DEV_GRP);
    twl.usb1v8 = devm_regulator_get(twl.dev, "usb1v8");
    if (IS_ERR(twl.usb1v8))
    return -ENODEV;
    twl_i2c_write_u8(TWL_MODULE_PM_RECEIVER, 0, VUSB1V8_TYPE);
// disable access to power configuration registers
    twl_i2c_write_u8(TWL_MODULE_PM_MASTER, 0,
    TWL4030_PM_MASTER_PROTECT_KEY);
    return 0;
    }
    static ssize_t vbus_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct twl4030_usb *twl = dev_get_drvdata(dev);
    let mut ret: c_int = -EINVAL;
    mutex_lock(&twl.lock);
    ret = sprintf(buf, "%s\n",
    twl.vbus_supplied ? "on" : "off");
    mutex_unlock(&twl.lock);
    return ret;
    }
    static DEVICE_ATTR_RO(vbus);
#[no_mangle]
unsafe extern "C" fn twl4030_usb_irq(irq: c_int, _twl: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl4030_usb_irq(int irq, void *_twl)
    {
    struct twl4030_usb *twl = _twl;
    enum musb_vbus_id_status status;
    int err;
    status = twl4030_usb_linkstat(twl);
    mutex_lock(&twl.lock);
    twl.linkstat = status;
    mutex_unlock(&twl.lock);
    if (cable_present(status)) {
    if (atomic_add_unless(&twl.connected, 1, 1)) {
    dev_dbg(twl.dev, "%s: cable connected %i\n",
    __func__, status);
    pm_runtime_get_sync(twl.dev);
    twl.musb_mailbox_pending = true;
    }
    } else {
    if (atomic_add_unless(&twl.connected, -1, 0)) {
    dev_dbg(twl.dev, "%s: cable disconnected %i\n",
    __func__, status);
    pm_runtime_mark_last_busy(twl.dev);
    pm_runtime_put_autosuspend(twl.dev);
    twl.musb_mailbox_pending = true;
    }
    }
    if (twl.musb_mailbox_pending) {
    err = musb_mailbox(status);
    if (!err)
    twl.musb_mailbox_pending = false;
    }
// don't schedule during sleep - irq works right then
    if (status == MUSB_ID_GROUND && pm_runtime_active(twl.dev)) {
    cancel_delayed_work(&twl.id_workaround_work);
    schedule_delayed_work(&twl.id_workaround_work, HZ);
    }
    if (irq)
    sysfs_notify(&twl.dev.kobj, core::ptr::null_mut(), "vbus");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_id_workaround_work(work: *mut work_struct) {
    static void twl4030_id_workaround_work(struct work_struct *work)
    {
    struct twl4030_usb *twl = container_of(work, struct twl4030_usb,
    id_workaround_work.work);
    twl4030_usb_irq(0, twl);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_phy_init(phy: *mut phy) -> c_int {
    static int twl4030_phy_init(struct phy *phy)
    {
    struct twl4030_usb *twl = phy_get_drvdata(phy);
    pm_runtime_get_sync(twl.dev);
    twl.linkstat = MUSB_UNKNOWN;
    schedule_delayed_work(&twl.id_workaround_work, HZ);
    pm_runtime_mark_last_busy(twl.dev);
    pm_runtime_put_autosuspend(twl.dev);
    return 0;
    }
    static int twl4030_set_peripheral(struct usb_otg *otg,
    struct usb_gadget *gadget)
    {
    if (!otg)
    return -ENODEV;
    otg.gadget = gadget;
    if (!gadget)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_set_host(otg: *mut usb_otg, host: *mut usb_bus) -> c_int {
    static int twl4030_set_host(struct usb_otg *otg, struct usb_bus *host)
    {
    if (!otg)
    return -ENODEV;
    otg.host = host;
    if (!host)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
    static const struct phy_ops ops = {
    .init		= twl4030_phy_init,
    .power_on	= twl4030_phy_power_on,
    .power_off	= twl4030_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct dev_pm_ops twl4030_usb_pm_ops = {
    SET_RUNTIME_PM_OPS(twl4030_usb_runtime_suspend,
    twl4030_usb_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(twl4030_usb_suspend, twl4030_usb_resume)
    };
#[no_mangle]
unsafe extern "C" fn twl4030_usb_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_usb_probe(struct platform_device *pdev)
    {
    struct twl4030_usb_data *pdata = dev_get_platdata(&pdev.dev);
    struct twl4030_usb	*twl;
    struct phy		*phy;
    int			status, err;
    struct usb_otg		*otg;
    struct device_node	*np = pdev.dev.of_node;
    struct phy_provider	*phy_provider;
    twl = devm_kzalloc(&pdev.dev, sizeof(*twl), GFP_KERNEL);
    if (!twl)
    return -ENOMEM;
    if (np)
    of_property_read_u32(np, "usb_mode",
    (enum twl4030_usb_mode *)&twl.usb_mode);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pdata) -> else {
    twl.usb_mode = pdata.usb_mode;
    } else {
    dev_err(&pdev.dev, "twl4030 initialized without pdata\n");
    return -EINVAL;
    }
    otg = devm_kzalloc(&pdev.dev, sizeof(*otg), GFP_KERNEL);
    if (!otg)
    return -ENOMEM;
    twl.dev		= &pdev.dev;
    twl.irq		= platform_get_irq(pdev, 0);
    twl.vbus_supplied	= false;
    twl.linkstat		= MUSB_UNKNOWN;
    twl.musb_mailbox_pending = false;
    twl.phy.dev		= twl.dev;
    twl.phy.label		= "twl4030";
    twl.phy.otg		= otg;
    twl.phy.type		= USB_PHY_TYPE_USB2;
    otg.usb_phy		= &twl.phy;
    otg.set_host		= twl4030_set_host;
    otg.set_peripheral	= twl4030_set_peripheral;
    phy = devm_phy_create(twl.dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(phy)) {
    dev_dbg(&pdev.dev, "Failed to create PHY\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, twl);
    phy_provider = devm_of_phy_provider_register(twl.dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
// init mutex for workqueue
    mutex_init(&twl.lock);
    INIT_DELAYED_WORK(&twl.id_workaround_work, twl4030_id_workaround_work);
    err = twl4030_usb_ldo_init(twl);
    if (err) {
    dev_err(&pdev.dev, "ldo init failed\n");
    return err;
    }
    usb_add_phy_dev(&twl.phy);
    platform_set_drvdata(pdev, twl);
    if (device_create_file(&pdev.dev, &dev_attr_vbus))
    dev_warn(&pdev.dev, "could not create sysfs file\n");
    ATOMIC_INIT_NOTIFIER_HEAD(&twl.phy.notifier);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 2000);
    pm_runtime_enable(&pdev.dev);
    pm_runtime_get_sync(&pdev.dev);
// Our job is to use irqs and status from the power module
// to keep the transceiver disabled when nothing's connected.
//
// FIXME we actually shouldn't start enabling it until the
// USB controller drivers have said they're ready, by calling
// set_host() and/or set_peripheral() ... OTG_capable boards
// need both handles, otherwise just one suffices.
//
    status = devm_request_threaded_irq(twl.dev, twl.irq, core::ptr::null_mut(),
    twl4030_usb_irq, IRQF_TRIGGER_FALLING |
    IRQF_TRIGGER_RISING | IRQF_ONESHOT, "twl4030_usb", twl);
    if (status < 0) {
    dev_dbg(&pdev.dev, "can't get IRQ %d, err %d\n",
    twl.irq, status);
    return status;
    }
    if (pdata)
    err = phy_create_lookup(phy, "usb", "musb-hdrc.0");
    if (err)
    return err;
    pm_runtime_mark_last_busy(&pdev.dev);
    pm_runtime_put_autosuspend(twl.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_usb_remove(pdev: *mut platform_device) {
    static void twl4030_usb_remove(struct platform_device *pdev)
    {
    struct twl4030_usb *twl = platform_get_drvdata(pdev);
    int val;
    usb_remove_phy(&twl.phy);
    pm_runtime_get_sync(twl.dev);
    cancel_delayed_work_sync(&twl.id_workaround_work);
    device_remove_file(twl.dev, &dev_attr_vbus);
// set transceiver mode to power on defaults
    twl4030_usb_set_mode(twl, -1);
// idle ulpi before powering off
    if (cable_present(twl.linkstat))
    pm_runtime_put_noidle(twl.dev);
    pm_runtime_mark_last_busy(twl.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    pm_runtime_put_sync(twl.dev);
    pm_runtime_disable(twl.dev);
// autogate 60MHz ULPI clock,
// clear dpll clock request for i2c access,
// disable 32KHz
//
    val = twl4030_usb_read(twl, PHY_CLK_CTRL);
    if (val >= 0) {
    val |= PHY_CLK_CTRL_CLOCKGATING_EN;
    val &= ~(PHY_CLK_CTRL_CLK32K_EN | REQ_PHY_DPLL_CLK);
    twl4030_usb_write(twl, PHY_CLK_CTRL, (u8)val);
    }
// disable complete OTG block
    twl4030_usb_clear_bits(twl, POWER_CTRL, POWER_CTRL_OTG_ENAB);
    }

    static const struct of_device_id twl4030_usb_id_table[] = {
    { .compatible = "ti,twl4030-usb" },
    {}
    };
    MODULE_DEVICE_TABLE(of, twl4030_usb_id_table);

    static struct platform_driver twl4030_usb_driver = {
    .probe		= twl4030_usb_probe,
    .remove		= twl4030_usb_remove,
    .driver		= {
    .name	= "twl4030_usb",
    .pm	= &twl4030_usb_pm_ops,
    .of_match_table = of_match_ptr(twl4030_usb_id_table),
    },
    };
#[no_mangle]
unsafe extern "C" fn twl4030_usb_init() -> int __init {
    static int __init twl4030_usb_init(void)
    {
    return platform_driver_register(&twl4030_usb_driver);
    }
    subsys_initcall(twl4030_usb_init);
#[no_mangle]
unsafe extern "C" fn twl4030_usb_exit() -> void __exit {
    static void __exit twl4030_usb_exit(void)
    {
    platform_driver_unregister(&twl4030_usb_driver);
    }
    module_exit(twl4030_usb_exit);
    MODULE_ALIAS("platform:twl4030_usb");
    MODULE_AUTHOR("Texas Instruments, Inc, Nokia Corporation");
    MODULE_DESCRIPTION("TWL4030 USB transceiver driver");
    MODULE_LICENSE("GPL");
