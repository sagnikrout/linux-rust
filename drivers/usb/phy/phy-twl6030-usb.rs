//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/phy-twl6030-usb.c
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
// twl6030_usb - TWL6030 USB transceiver, talking to OMAP OTG driver.
//
// Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com
//
// Author: Hema HK <hemahk@ti.com>
//

// usb register definitions
pub const USB_VENDOR_ID_LSB: c_uint = 0x00;
pub const USB_VENDOR_ID_MSB: c_uint = 0x01;
pub const USB_PRODUCT_ID_LSB: c_uint = 0x02;
pub const USB_PRODUCT_ID_MSB: c_uint = 0x03;
pub const USB_VBUS_CTRL_SET: c_uint = 0x04;
pub const USB_VBUS_CTRL_CLR: c_uint = 0x05;
pub const USB_ID_CTRL_SET: c_uint = 0x06;
pub const USB_ID_CTRL_CLR: c_uint = 0x07;
pub const USB_VBUS_INT_SRC: c_uint = 0x08;
pub const USB_VBUS_INT_LATCH_SET: c_uint = 0x09;
pub const USB_VBUS_INT_LATCH_CLR: c_uint = 0x0A;
pub const USB_VBUS_INT_EN_LO_SET: c_uint = 0x0B;
pub const USB_VBUS_INT_EN_LO_CLR: c_uint = 0x0C;
pub const USB_VBUS_INT_EN_HI_SET: c_uint = 0x0D;
pub const USB_VBUS_INT_EN_HI_CLR: c_uint = 0x0E;
pub const USB_ID_INT_SRC: c_uint = 0x0F;
pub const USB_ID_INT_LATCH_SET: c_uint = 0x10;
pub const USB_ID_INT_LATCH_CLR: c_uint = 0x11;
pub const USB_ID_INT_EN_LO_SET: c_uint = 0x12;
pub const USB_ID_INT_EN_LO_CLR: c_uint = 0x13;
pub const USB_ID_INT_EN_HI_SET: c_uint = 0x14;
pub const USB_ID_INT_EN_HI_CLR: c_uint = 0x15;
pub const USB_OTG_ADP_CTRL: c_uint = 0x16;
pub const USB_OTG_ADP_HIGH: c_uint = 0x17;
pub const USB_OTG_ADP_LOW: c_uint = 0x18;
pub const USB_OTG_ADP_RISE: c_uint = 0x19;
pub const USB_OTG_REVISION: c_uint = 0x1A;
// to be moved to LDO
pub const TWL6030_MISC2: c_uint = 0xE5;
pub const TWL6030_CFG_LDO_PD2: c_uint = 0xF5;
pub const TWL6030_BACKUP_REG: c_uint = 0xFA;
pub const STS_HW_CONDITIONS: c_uint = 0x21;
// In module TWL6030_MODULE_PM_MASTER
pub const STS_HW_CONDITIONS: c_uint = 0x21;

// In module TWL6030_MODULE_PM_RECEIVER
pub const VUSB_CFG_TRANS: c_uint = 0x71;
pub const VUSB_CFG_STATE: c_uint = 0x72;
pub const VUSB_CFG_VOLTAGE: c_uint = 0x73;
// in module TWL6030_MODULE_MAIN_CHARGE
pub const CHARGERUSB_CTRL1: c_uint = 0x8;
pub const CONTROLLER_STAT1: c_uint = 0x03;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl6030_usb {
    pub comparator: phy_companion,
    pub dev: *mut device,
// for vbus reporting with irqs disabled
    pub lock: spinlock_t,
    pub usb3v3: *mut regulator,
// used to check initial cable status after probe
    pub get_status_work: delayed_work,
// used to set vbus, in atomic path
    pub set_vbus_work: work_struct,
    pub irq1: c_int,
    pub irq2: c_int,
    pub linkstat: enum musb_vbus_id_status,
    pub asleep: u8,
    pub vbus_enable: bool,
}

// -------------------------------------------------------------------------
    static inline int twl6030_writeb(struct twl6030_usb *twl, u8 module,
    u8 data, u8 address)
    {
    let mut ret: c_int = 0;
    ret = twl_i2c_write_u8(module, data, address);
    if (ret < 0)
    dev_err(twl.dev,
    "Write[0x%x] Error %d\n", address, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn twl6030_readb(twl: *mut twl6030_usb, module: u8, address: u8) -> u8 {
    static inline u8 twl6030_readb(struct twl6030_usb *twl, u8 module, u8 address)
    {
    u8 data;
    int ret;
    ret = twl_i2c_read_u8(module, &data, address);
    if (ret >= 0)
    ret = data;
    else
    dev_err(twl.dev,
    "readb[0x%x,0x%x] Error %d\n",
    module, address, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_start_srp(comparator: *mut phy_companion) -> c_int {
    static int twl6030_start_srp(struct phy_companion *comparator)
    {
    struct twl6030_usb *twl = comparator_to_twl(comparator);
    twl6030_writeb(twl, TWL_MODULE_USB, 0x24, USB_VBUS_CTRL_SET);
    twl6030_writeb(twl, TWL_MODULE_USB, 0x84, USB_VBUS_CTRL_SET);
    mdelay(100);
    twl6030_writeb(twl, TWL_MODULE_USB, 0xa0, USB_VBUS_CTRL_CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_usb_ldo_init(twl: *mut twl6030_usb) -> c_int {
    static int twl6030_usb_ldo_init(struct twl6030_usb *twl)
    {
// Set to OTG_REV 1.3 and turn on the ID_WAKEUP_COMP
    twl6030_writeb(twl, TWL6030_MODULE_ID0, 0x1, TWL6030_BACKUP_REG);
// Program CFG_LDO_PD2 register and set VUSB bit
    twl6030_writeb(twl, TWL6030_MODULE_ID0, 0x1, TWL6030_CFG_LDO_PD2);
// Program MISC2 register and set bit VUSB_IN_VBAT
    twl6030_writeb(twl, TWL6030_MODULE_ID0, 0x10, TWL6030_MISC2);
    twl.usb3v3 = regulator_get(twl.dev, "usb");
    if (IS_ERR(twl.usb3v3))
    return -ENODEV;
// Program the USB_VBUS_CTRL_SET and set VBUS_ACT_COMP bit
    twl6030_writeb(twl, TWL_MODULE_USB, 0x4, USB_VBUS_CTRL_SET);
//
// Program the USB_ID_CTRL_SET register to enable GND drive
// and the ID comparators
//
    twl6030_writeb(twl, TWL_MODULE_USB, 0x14, USB_ID_CTRL_SET);
    return 0;
    }
    static ssize_t vbus_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct twl6030_usb *twl = dev_get_drvdata(dev);
    unsigned long flags;
    let mut ret: c_int = -EINVAL;
    spin_lock_irqsave(&twl.lock, flags);
    switch (twl.linkstat) {
    case MUSB_VBUS_VALID:
    ret = sysfs_emit(buf, "vbus\n");
    break;
    case MUSB_ID_GROUND:
    ret = sysfs_emit(buf, "id\n");
    break;
    case MUSB_VBUS_OFF:
    ret = sysfs_emit(buf, "none\n");
    break;
    default:
    ret = sysfs_emit(buf, "UNKNOWN\n");
    }
    spin_unlock_irqrestore(&twl.lock, flags);
    return ret;
    }
    static DEVICE_ATTR_RO(vbus);
    static struct attribute *twl6030_attrs[] = {
    &dev_attr_vbus.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(twl6030);
#[no_mangle]
unsafe extern "C" fn twl6030_usb_irq(irq: c_int, _twl: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl6030_usb_irq(int irq, void *_twl)
    {
    struct twl6030_usb *twl = _twl;
    let mut status: enum musb_vbus_id_status = MUSB_UNKNOWN;
    u8 vbus_state, hw_state;
    int ret;
    hw_state = twl6030_readb(twl, TWL6030_MODULE_ID0, STS_HW_CONDITIONS);
    vbus_state = twl6030_readb(twl, TWL_MODULE_MAIN_CHARGE,
    CONTROLLER_STAT1);
    if (!(hw_state & STS_USB_ID)) {
    if (vbus_state & VBUS_DET) {
    ret = regulator_enable(twl.usb3v3);
    if (ret)
    dev_err(twl.dev, "Failed to enable usb3v3\n");
    twl.asleep = 1;
    status = MUSB_VBUS_VALID;
    twl.linkstat = status;
    ret = musb_mailbox(status);
    if (ret)
    twl.linkstat = MUSB_UNKNOWN;
    } else {
    if (twl.linkstat != MUSB_UNKNOWN) {
    status = MUSB_VBUS_OFF;
    twl.linkstat = status;
    ret = musb_mailbox(status);
    if (ret)
    twl.linkstat = MUSB_UNKNOWN;
    if (twl.asleep) {
    regulator_disable(twl.usb3v3);
    twl.asleep = 0;
    }
    }
    }
    }
    sysfs_notify(&twl.dev.kobj, core::ptr::null_mut(), "vbus");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_usbotg_irq(irq: c_int, _twl: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl6030_usbotg_irq(int irq, void *_twl)
    {
    struct twl6030_usb *twl = _twl;
    let mut status: enum musb_vbus_id_status = MUSB_UNKNOWN;
    u8 hw_state;
    int ret;
    hw_state = twl6030_readb(twl, TWL6030_MODULE_ID0, STS_HW_CONDITIONS);
    if (hw_state & STS_USB_ID) {
    ret = regulator_enable(twl.usb3v3);
    if (ret)
    dev_err(twl.dev, "Failed to enable usb3v3\n");
    twl.asleep = 1;
    twl6030_writeb(twl, TWL_MODULE_USB, 0x1, USB_ID_INT_EN_HI_CLR);
    twl6030_writeb(twl, TWL_MODULE_USB, 0x10, USB_ID_INT_EN_HI_SET);
    status = MUSB_ID_GROUND;
    twl.linkstat = status;
    ret = musb_mailbox(status);
    if (ret)
    twl.linkstat = MUSB_UNKNOWN;
    } else  {
    twl6030_writeb(twl, TWL_MODULE_USB, 0x10, USB_ID_INT_EN_HI_CLR);
    twl6030_writeb(twl, TWL_MODULE_USB, 0x1, USB_ID_INT_EN_HI_SET);
    }
    twl6030_writeb(twl, TWL_MODULE_USB, status, USB_ID_INT_LATCH_CLR);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_status_work(work: *mut work_struct) {
    static void twl6030_status_work(struct work_struct *work)
    {
    struct twl6030_usb *twl = container_of(work, struct twl6030_usb,
    get_status_work.work);
    twl6030_usb_irq(twl.irq2, twl);
    twl6030_usbotg_irq(twl.irq1, twl);
    }
#[no_mangle]
unsafe extern "C" fn twl6030_enable_irq(twl: *mut twl6030_usb) -> c_int {
    static int twl6030_enable_irq(struct twl6030_usb *twl)
    {
    twl6030_writeb(twl, TWL_MODULE_USB, 0x1, USB_ID_INT_EN_HI_SET);
    twl6030_interrupt_unmask(0x05, REG_INT_MSK_LINE_C);
    twl6030_interrupt_unmask(0x05, REG_INT_MSK_STS_C);
    twl6030_interrupt_unmask(TWL6030_CHARGER_CTRL_INT_MASK,
    REG_INT_MSK_LINE_C);
    twl6030_interrupt_unmask(TWL6030_CHARGER_CTRL_INT_MASK,
    REG_INT_MSK_STS_C);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn otg_set_vbus_work(data: *mut work_struct) {
    static void otg_set_vbus_work(struct work_struct *data)
    {
    struct twl6030_usb *twl = container_of(data, struct twl6030_usb,
    set_vbus_work);
//
// Start driving VBUS. Set OPA_MODE bit in CHARGERUSB_CTRL1
// register. This enables boost mode.
//
    if (twl.vbus_enable)
    twl6030_writeb(twl, TWL_MODULE_MAIN_CHARGE, 0x40,
    CHARGERUSB_CTRL1);
    else
    twl6030_writeb(twl, TWL_MODULE_MAIN_CHARGE, 0x00,
    CHARGERUSB_CTRL1);
    }
#[no_mangle]
unsafe extern "C" fn twl6030_set_vbus(comparator: *mut phy_companion, enabled: bool) -> c_int {
    static int twl6030_set_vbus(struct phy_companion *comparator, bool enabled)
    {
    struct twl6030_usb *twl = comparator_to_twl(comparator);
    twl.vbus_enable = enabled;
    schedule_work(&twl.set_vbus_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_usb_probe(pdev: *mut platform_device) -> c_int {
    static int twl6030_usb_probe(struct platform_device *pdev)
    {
    struct twl6030_usb	*twl;
    int			status, err, ret;
    struct device_node	*np = pdev.dev.of_node;
    struct device		*dev = &pdev.dev;
    if (!np) {
    dev_err(dev, "no DT info\n");
    return -EINVAL;
    }
    twl = devm_kzalloc(dev, sizeof(*twl), GFP_KERNEL);
    if (!twl)
    return -ENOMEM;
    twl.dev		= &pdev.dev;
    twl.irq1		= platform_get_irq(pdev, 0);
    twl.irq2		= platform_get_irq(pdev, 1);
    twl.linkstat		= MUSB_UNKNOWN;
    if (twl.irq1 < 0)
    return twl.irq1;
    if (twl.irq2 < 0)
    return twl.irq2;
    twl.comparator.set_vbus	= twl6030_set_vbus;
    twl.comparator.start_srp	= twl6030_start_srp;
    ret = omap_usb2_set_comparator(&twl.comparator);
    if (ret == -ENODEV) {
    dev_info(&pdev.dev, "phy not ready, deferring probe");
    return -EPROBE_DEFER;
    }
// init spinlock for workqueue
    spin_lock_init(&twl.lock);
    err = twl6030_usb_ldo_init(twl);
    if (err) {
    dev_err(&pdev.dev, "ldo init failed\n");
    return err;
    }
    platform_set_drvdata(pdev, twl);
    INIT_WORK(&twl.set_vbus_work, otg_set_vbus_work);
    INIT_DELAYED_WORK(&twl.get_status_work, twl6030_status_work);
    status = request_threaded_irq(twl.irq1, core::ptr::null_mut(), twl6030_usbotg_irq,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    "twl6030_usb", twl);
    if (status < 0) {
    dev_err(&pdev.dev, "can't get IRQ %d, err %d\n",
    twl.irq1, status);
    goto err_put_regulator;
    }
    status = request_threaded_irq(twl.irq2, core::ptr::null_mut(), twl6030_usb_irq,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    "twl6030_usb", twl);
    if (status < 0) {
    dev_err(&pdev.dev, "can't get IRQ %d, err %d\n",
    twl.irq2, status);
    goto err_free_irq1;
    }
    twl.asleep = 0;
    twl6030_enable_irq(twl);
    schedule_delayed_work(&twl.get_status_work, HZ);
    dev_info(&pdev.dev, "Initialized TWL6030 USB module\n");
    return 0;
    err_free_irq1:
    free_irq(twl.irq1, twl);
    err_put_regulator:
    regulator_put(twl.usb3v3);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn twl6030_usb_remove(pdev: *mut platform_device) {
    static void twl6030_usb_remove(struct platform_device *pdev)
    {
    struct twl6030_usb *twl = platform_get_drvdata(pdev);
    cancel_delayed_work_sync(&twl.get_status_work);
    twl6030_interrupt_mask(TWL6030_USBOTG_INT_MASK,
    REG_INT_MSK_LINE_C);
    twl6030_interrupt_mask(TWL6030_USBOTG_INT_MASK,
    REG_INT_MSK_STS_C);
    free_irq(twl.irq1, twl);
    free_irq(twl.irq2, twl);
    regulator_put(twl.usb3v3);
    cancel_work_sync(&twl.set_vbus_work);
    }
    static const struct of_device_id twl6030_usb_id_table[] = {
    { .compatible = "ti,twl6030-usb" },
    {}
    };
    MODULE_DEVICE_TABLE(of, twl6030_usb_id_table);
    static struct platform_driver twl6030_usb_driver = {
    .probe		= twl6030_usb_probe,
    .remove		= twl6030_usb_remove,
    .driver		= {
    .name	= "twl6030_usb",
    .of_match_table = of_match_ptr(twl6030_usb_id_table),
    .dev_groups = twl6030_groups,
    },
    };
#[no_mangle]
unsafe extern "C" fn twl6030_usb_init() -> int __init {
    static int __init twl6030_usb_init(void)
    {
    return platform_driver_register(&twl6030_usb_driver);
    }
    subsys_initcall(twl6030_usb_init);
#[no_mangle]
unsafe extern "C" fn twl6030_usb_exit() -> void __exit {
    static void __exit twl6030_usb_exit(void)
    {
    platform_driver_unregister(&twl6030_usb_driver);
    }
    module_exit(twl6030_usb_exit);
    MODULE_ALIAS("platform:twl6030_usb");
    MODULE_AUTHOR("Hema HK <hemahk@ti.com>");
    MODULE_DESCRIPTION("TWL6030 USB transceiver driver");
    MODULE_LICENSE("GPL");
