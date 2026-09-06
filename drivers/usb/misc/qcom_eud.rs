//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/qcom_eud.c
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
// Copyright (c) 2015-2021, The Linux Foundation. All rights reserved.
//

pub const EUD_REG_INT1_EN_MASK: c_uint = 0x0024;
pub const EUD_REG_INT_STATUS_1: c_uint = 0x0044;
pub const EUD_REG_CTL_OUT_1: c_uint = 0x0074;
pub const EUD_REG_VBUS_INT_CLR: c_uint = 0x0080;
pub const EUD_REG_CSR_EUD_EN: c_uint = 0x1014;
pub const EUD_REG_SW_ATTACH_DET: c_uint = 0x1018;
pub const EUD_REG_EUD_EN2: c_uint = 0x0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eud_chip {
    pub dev: *mut device,
    pub role_sw: *mut usb_role_switch,
    pub base: *mut void __iomem,
    pub mode_mgr: phys_addr_t,
    pub int_status: c_uint,
    pub irq: c_int,
    pub enabled: bool,
    pub usb_attached: bool,
}

#[no_mangle]
unsafe extern "C" fn enable_eud(priv: *mut eud_chip) -> c_int {
    static int enable_eud(struct eud_chip *priv)
    {
    int ret;
    ret = qcom_scm_io_writel(priv.mode_mgr + EUD_REG_EUD_EN2, 1);
    if (ret)
    return ret;
    writel(EUD_ENABLE, priv.base + EUD_REG_CSR_EUD_EN);
    writel(EUD_INT_VBUS | EUD_INT_SAFE_MODE,
    priv.base + EUD_REG_INT1_EN_MASK);
    return usb_role_switch_set_role(priv.role_sw, USB_ROLE_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn disable_eud(priv: *mut eud_chip) -> c_int {
    static int disable_eud(struct eud_chip *priv)
    {
    int ret;
    ret = qcom_scm_io_writel(priv.mode_mgr + EUD_REG_EUD_EN2, 0);
    if (ret)
    return ret;
    writel(0, priv.base + EUD_REG_CSR_EUD_EN);
    return 0;
    }
    static ssize_t enable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct eud_chip *chip = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", chip.enabled);
    }
    static ssize_t enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct eud_chip *chip = dev_get_drvdata(dev);
    bool enable;
    int ret;
    if (kstrtobool(buf, &enable))
    return -EINVAL;
    if (enable) {
    ret = enable_eud(chip);
    if (!ret)
    chip.enabled = enable;
    else
    disable_eud(chip);
    } else {
    ret = disable_eud(chip);
    }
    return ret < 0 ? ret : count;
    }
    static DEVICE_ATTR_RW(enable);
    static struct attribute *eud_attrs[] = {
    &dev_attr_enable.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(eud);
#[no_mangle]
unsafe extern "C" fn usb_attach_detach(chip: *mut eud_chip) {
    static void usb_attach_detach(struct eud_chip *chip)
    {
    u32 reg;
// read ctl_out_1[4] to find USB attach or detach event
    reg = readl(chip.base + EUD_REG_CTL_OUT_1);
    chip.usb_attached = reg & EUD_INT_SAFE_MODE;
    }
#[no_mangle]
unsafe extern "C" fn pet_eud(chip: *mut eud_chip) {
    static void pet_eud(struct eud_chip *chip)
    {
    u32 reg;
    int ret;
// When the EUD_INT_PET_EUD in SW_ATTACH_DET is set, the cable has been
// disconnected and we need to detach the pet to check if EUD is in safe
// mode before attaching again.
//
    reg = readl(chip.base + EUD_REG_SW_ATTACH_DET);
    if (reg & EUD_INT_PET_EUD) {
// Detach & Attach pet for EUD
    writel(0, chip.base + EUD_REG_SW_ATTACH_DET);
// Delay to make sure detach pet is done before attach pet
    ret = readl_poll_timeout(chip.base + EUD_REG_SW_ATTACH_DET,
    reg, (reg == 0), 1, 100);
    if (ret) {
    dev_err(chip.dev, "Detach pet failed\n");
    return;
    }
    }
// Attach pet for EUD
    writel(EUD_INT_PET_EUD, chip.base + EUD_REG_SW_ATTACH_DET);
    }
#[no_mangle]
unsafe extern "C" fn handle_eud_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t handle_eud_irq(int irq, void *data)
    {
    struct eud_chip *chip = data;
    u32 reg;
    reg = readl(chip.base + EUD_REG_INT_STATUS_1);
    switch (reg & EUD_INT_ALL) {
    case EUD_INT_VBUS:
    usb_attach_detach(chip);
    return IRQ_WAKE_THREAD;
    case EUD_INT_SAFE_MODE:
    pet_eud(chip);
    return IRQ_HANDLED;
    default:
    return IRQ_NONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_eud_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t handle_eud_irq_thread(int irq, void *data)
    {
    struct eud_chip *chip = data;
    int ret;
    if (chip.usb_attached)
    ret = usb_role_switch_set_role(chip.role_sw, USB_ROLE_DEVICE);
    else
    ret = usb_role_switch_set_role(chip.role_sw, USB_ROLE_HOST);
    if (ret)
    dev_err(chip.dev, "failed to set role switch\n");
// set and clear vbus_int_clr[0] to clear interrupt
    writel(BIT(0), chip.base + EUD_REG_VBUS_INT_CLR);
    writel(0, chip.base + EUD_REG_VBUS_INT_CLR);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn eud_role_switch_release(data: *mut c_void) {
    static void eud_role_switch_release(void *data)
    {
    struct eud_chip *chip = data;
    usb_role_switch_put(chip.role_sw);
    }
#[no_mangle]
unsafe extern "C" fn eud_probe(pdev: *mut platform_device) -> c_int {
    static int eud_probe(struct platform_device *pdev)
    {
    struct eud_chip *chip;
    struct resource *res;
    int ret;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &pdev.dev;
    chip.role_sw = usb_role_switch_get(&pdev.dev);
    if (IS_ERR(chip.role_sw))
    return dev_err_probe(chip.dev, PTR_ERR(chip.role_sw),
    "failed to get role switch\n");
    ret = devm_add_action_or_reset(chip.dev, eud_role_switch_release, chip);
    if (ret)
    return ret;
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base))
    return PTR_ERR(chip.base);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res)
    return -ENODEV;
    chip.mode_mgr = res.start;
    chip.irq = platform_get_irq(pdev, 0);
    if (chip.irq < 0)
    return chip.irq;
    ret = devm_request_threaded_irq(&pdev.dev, chip.irq, handle_eud_irq,
    handle_eud_irq_thread, IRQF_ONESHOT, core::ptr::null_mut(), chip);
    if (ret)
    return dev_err_probe(chip.dev, ret, "failed to allocate irq\n");
    enable_irq_wake(chip.irq);
    platform_set_drvdata(pdev, chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eud_remove(pdev: *mut platform_device) {
    static void eud_remove(struct platform_device *pdev)
    {
    struct eud_chip *chip = platform_get_drvdata(pdev);
    if (chip.enabled)
    disable_eud(chip);
    device_init_wakeup(&pdev.dev, false);
    disable_irq_wake(chip.irq);
    }
    static const struct of_device_id eud_dt_match[] = {
    { .compatible = "qcom,eud" },
    { }
    };
    MODULE_DEVICE_TABLE(of, eud_dt_match);
    static struct platform_driver eud_driver = {
    .probe	= eud_probe,
    .remove = eud_remove,
    .driver	= {
    .name = "qcom_eud",
    .dev_groups = eud_groups,
    .of_match_table = eud_dt_match,
    },
    };
    module_platform_driver(eud_driver);
    MODULE_DESCRIPTION("QTI EUD driver");
    MODULE_LICENSE("GPL v2");
