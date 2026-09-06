//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-axp288.c
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
// extcon-axp288.c - X-Power AXP288 PMIC extcon cable detection driver
//
// Copyright (c) 2017-2018 Hans de Goede <hdegoede@redhat.com>
// Copyright (C) 2015 Intel Corporation
// Author: Ramakrishna Pallala <ramakrishna.pallala@intel.com>
//

// Power source status register

// BC module global register

pub const BC_GLOBAL_DCD_TOUT_300MS: c_int = 0;
pub const BC_GLOBAL_DCD_TOUT_100MS: c_int = 1;
pub const BC_GLOBAL_DCD_TOUT_500MS: c_int = 2;
pub const BC_GLOBAL_DCD_TOUT_900MS: c_int = 3;

// BC module vbus control and status register

// BC USB status register

pub const USB_STAT_BUS_STAT_SHIFT: c_int = 0;
pub const USB_STAT_BUS_STAT_ATHD: c_int = 0;
pub const USB_STAT_BUS_STAT_CONN: c_int = 1;
pub const USB_STAT_BUS_STAT_SUSP: c_int = 2;
pub const USB_STAT_BUS_STAT_CONF: c_int = 3;

// BC detect status register

pub const DET_STAT_SHIFT: c_int = 5;
pub const DET_STAT_SDP: c_int = 1;
pub const DET_STAT_CDP: c_int = 2;
pub const DET_STAT_DCP: c_int = 3;
    enum axp288_extcon_reg {
    AXP288_PS_STAT_REG		= 0x00,
    AXP288_PS_BOOT_REASON_REG	= 0x02,
    AXP288_BC_GLOBAL_REG		= 0x2c,
    AXP288_BC_VBUS_CNTL_REG		= 0x2d,
    AXP288_BC_USB_STAT_REG		= 0x2e,
    AXP288_BC_DET_STAT_REG		= 0x2f,
    };
    enum axp288_extcon_irq {
    VBUS_FALLING_IRQ = 0,
    VBUS_RISING_IRQ,
    MV_CHNG_IRQ,
    BC_USB_CHNG_IRQ,
    EXTCON_IRQ_END,
    };
    static const unsigned int axp288_extcon_cables[] = {
    EXTCON_CHG_USB_SDP,
    EXTCON_CHG_USB_CDP,
    EXTCON_CHG_USB_DCP,
    EXTCON_USB,
    EXTCON_NONE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp288_extcon_info {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irqc: *mut regmap_irq_chip_data,
    pub role_sw: *mut usb_role_switch,
    pub role_work: work_struct,
    pub irq: [c_int; EXTCON_IRQ_END],
    pub edev: *mut extcon_dev,
    pub id_extcon: *mut extcon_dev,
    pub id_nb: notifier_block,
    pub previous_cable: c_uint,
    pub vbus_attach: bool,
}

    static const struct x86_cpu_id cherry_trail_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_ATOM_AIRMONT,	core::ptr::null_mut()),
    {}
    };
// Power up/down reason string array
    static const char * const axp288_pwr_up_down_info[] = {
    "Last wake caused by user pressing the power button",
    "Last wake caused by a charger insertion",
    "Last wake caused by a battery insertion",
    "Last wake caused by SOC initiated global reset",
    "Last wake caused by cold reset",
    "Last shutdown caused by PMIC UVLO threshold",
    "Last shutdown caused by SOC initiated cold off",
    "Last shutdown caused by user pressing the power button",
    };
//
// Decode and log the given "reset source indicator" (rsi)
// register and then clear it.
//
#[no_mangle]
unsafe extern "C" fn axp288_extcon_log_rsi(info: *mut axp288_extcon_info) {
    static void axp288_extcon_log_rsi(struct axp288_extcon_info *info)
    {
    unsigned int val, i, clear_mask = 0;
    unsigned long bits;
    int ret;
    ret = regmap_read(info.regmap, AXP288_PS_BOOT_REASON_REG, &val);
    if (ret < 0) {
    dev_err(info.dev, "failed to read reset source indicator\n");
    return;
    }
    bits = val & GENMASK(ARRAY_SIZE(axp288_pwr_up_down_info) - 1, 0);
    for_each_set_bit(i, &bits, ARRAY_SIZE(axp288_pwr_up_down_info))
    dev_dbg(info.dev, "%s\n", axp288_pwr_up_down_info[i]);
    clear_mask = bits;
// Clear the register value for next reboot (write 1 to clear bit)
    regmap_write(info.regmap, AXP288_PS_BOOT_REASON_REG, clear_mask);
    }
//
// The below code to control the USB role-switch on devices with an AXP288
// may seem out of place, but there are 2 reasons why this is the best place
// to control the USB role-switch on such devices:
// 1) On many devices the USB role is controlled by AML code, but the AML code
// only switches between the host and none roles, because of Windows not
// really using device mode. To make device mode work we need to toggle
// between the none/device roles based on Vbus presence, and this driver
// gets interrupts on Vbus insertion / removal.
// 2) In order for our BC1.2 charger detection to work properly the role
// mux must be properly set to device mode before we do the detection.
//
// Returns the id-pin value, note pulled low / false == host-mode
#[no_mangle]
unsafe extern "C" fn axp288_get_id_pin(info: *mut axp288_extcon_info) -> bool {
    static bool axp288_get_id_pin(struct axp288_extcon_info *info)
    {
    enum usb_role role;
    if (info.id_extcon)
    return extcon_get_state(info.id_extcon, EXTCON_USB_HOST) <= 0;
// We cannot access the id-pin, see what mode the AML code has set
    role = usb_role_switch_get_role(info.role_sw);
    return role != USB_ROLE_HOST;
    }
#[no_mangle]
unsafe extern "C" fn axp288_usb_role_work(work: *mut work_struct) {
    static void axp288_usb_role_work(struct work_struct *work)
    {
    struct axp288_extcon_info *info =
    container_of(work, struct axp288_extcon_info, role_work);
    enum usb_role role;
    bool id_pin;
    int ret;
    id_pin = axp288_get_id_pin(info);
    if (!id_pin)
    role = USB_ROLE_HOST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info->vbus_attach) -> else {
    else if (info.vbus_attach)
    role = USB_ROLE_DEVICE;
    else
    role = USB_ROLE_NONE;
    ret = usb_role_switch_set_role(info.role_sw, role);
    if (ret)
    dev_err(info.dev, "failed to set role: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn axp288_get_vbus_attach(info: *mut axp288_extcon_info) -> bool {
    static bool axp288_get_vbus_attach(struct axp288_extcon_info *info)
    {
    int ret, pwr_stat;
    ret = regmap_read(info.regmap, AXP288_PS_STAT_REG, &pwr_stat);
    if (ret < 0) {
    dev_err(info.dev, "failed to read vbus status\n");
    return false;
    }
    return !!(pwr_stat & PS_STAT_VBUS_VALID);
    }
#[no_mangle]
unsafe extern "C" fn axp288_handle_chrg_det_event(info: *mut axp288_extcon_info) -> c_int {
    static int axp288_handle_chrg_det_event(struct axp288_extcon_info *info)
    {
    int ret, stat, cfg;
    u8 chrg_type;
    let mut cable: c_uint = info.previous_cable;
    let mut vbus_attach: bool = false;
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    vbus_attach = axp288_get_vbus_attach(info);
    if (!vbus_attach)
    goto no_vbus;
// Check charger detection completion status
    ret = regmap_read(info.regmap, AXP288_BC_GLOBAL_REG, &cfg);
    if (ret < 0)
    goto dev_det_ret;
    if (cfg & BC_GLOBAL_DET_STAT) {
    dev_dbg(info.dev, "can't complete the charger detection\n");
    goto dev_det_ret;
    }
    ret = regmap_read(info.regmap, AXP288_BC_DET_STAT_REG, &stat);
    if (ret < 0)
    goto dev_det_ret;
    chrg_type = (stat & DET_STAT_MASK) >> DET_STAT_SHIFT;
    switch (chrg_type) {
    case DET_STAT_SDP:
    dev_dbg(info.dev, "sdp cable is connected\n");
    cable = EXTCON_CHG_USB_SDP;
    break;
    case DET_STAT_CDP:
    dev_dbg(info.dev, "cdp cable is connected\n");
    cable = EXTCON_CHG_USB_CDP;
    break;
    case DET_STAT_DCP:
    dev_dbg(info.dev, "dcp cable is connected\n");
    cable = EXTCON_CHG_USB_DCP;
    break;
    default:
    dev_warn(info.dev, "unknown (reserved) bc detect result\n");
    cable = EXTCON_CHG_USB_SDP;
    }
    no_vbus:
    iosf_mbi_unblock_punit_i2c_access();
    extcon_set_state_sync(info.edev, info.previous_cable, false);
    if (info.previous_cable == EXTCON_CHG_USB_SDP)
    extcon_set_state_sync(info.edev, EXTCON_USB, false);
    if (vbus_attach) {
    extcon_set_state_sync(info.edev, cable, vbus_attach);
    if (cable == EXTCON_CHG_USB_SDP)
    extcon_set_state_sync(info.edev, EXTCON_USB,
    vbus_attach);
    info.previous_cable = cable;
    }
    if (info.role_sw && info.vbus_attach != vbus_attach) {
    info.vbus_attach = vbus_attach;
// Setting the role can take a while
    queue_work(system_long_wq, &info.role_work);
    }
    return 0;
    dev_det_ret:
    iosf_mbi_unblock_punit_i2c_access();
    if (ret < 0)
    dev_err(info.dev, "failed to detect BC Mod\n");
    return ret;
    }
    static int axp288_extcon_id_evt(struct notifier_block *nb,
    unsigned long event, void *param)
    {
    struct axp288_extcon_info *info =
    container_of(nb, struct axp288_extcon_info, id_nb);
// We may not sleep and setting the role can take a while
    queue_work(system_long_wq, &info.role_work);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t axp288_extcon_isr(int irq, void *data)
    {
    struct axp288_extcon_info *info = data;
    int ret;
    ret = axp288_handle_chrg_det_event(info);
    if (ret < 0)
    dev_err(info.dev, "failed to handle the interrupt\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_enable(info: *mut axp288_extcon_info) -> c_int {
    static int axp288_extcon_enable(struct axp288_extcon_info *info)
    {
    let mut ret: c_int = 0;
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    regmap_update_bits(info.regmap, AXP288_BC_GLOBAL_REG,
    BC_GLOBAL_RUN, 0);
// Enable the charger detection logic
    regmap_update_bits(info.regmap, AXP288_BC_GLOBAL_REG,
    BC_GLOBAL_RUN, BC_GLOBAL_RUN);
    iosf_mbi_unblock_punit_i2c_access();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axp288_put_role_sw(data: *mut c_void) {
    static void axp288_put_role_sw(void *data)
    {
    struct axp288_extcon_info *info = data;
    cancel_work_sync(&info.role_work);
    usb_role_switch_put(info.role_sw);
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_find_role_sw(info: *mut axp288_extcon_info) -> c_int {
    static int axp288_extcon_find_role_sw(struct axp288_extcon_info *info)
    {
    const struct software_node *swnode;
    struct fwnode_handle *fwnode;
    if (!x86_match_cpu(cherry_trail_cpu_ids))
    return 0;
    swnode = software_node_find_by_name(core::ptr::null_mut(), "intel-xhci-usb-sw");
    if (!swnode)
    return -EPROBE_DEFER;
    fwnode = software_node_fwnode(swnode);
    info.role_sw = usb_role_switch_find_by_fwnode(fwnode);
    fwnode_handle_put(fwnode);
    return info.role_sw ? 0 : -EPROBE_DEFER;
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_probe(pdev: *mut platform_device) -> c_int {
    static int axp288_extcon_probe(struct platform_device *pdev)
    {
    struct axp288_extcon_info *info;
    struct axp20x_dev *axp20x = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct acpi_device *adev;
    int ret, i, pirq;
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = &pdev.dev;
    info.regmap = axp20x.regmap;
    info.regmap_irqc = axp20x.regmap_irqc;
    info.previous_cable = EXTCON_NONE;
    INIT_WORK(&info.role_work, axp288_usb_role_work);
    info.id_nb.notifier_call = axp288_extcon_id_evt;
    platform_set_drvdata(pdev, info);
    ret = axp288_extcon_find_role_sw(info);
    if (ret)
    return ret;
    if (info.role_sw) {
    ret = devm_add_action_or_reset(dev, axp288_put_role_sw, info);
    if (ret)
    return ret;
    adev = acpi_dev_get_first_match_dev("INT3496", core::ptr::null_mut(), -1);
    if (adev) {
    info.id_extcon = extcon_get_extcon_dev(acpi_dev_name(adev));
    acpi_dev_put(adev);
    if (IS_ERR(info.id_extcon))
    return PTR_ERR(info.id_extcon);
    dev_info(dev, "controlling USB role\n");
    } else {
    dev_info(dev, "controlling USB role based on Vbus presence\n");
    }
    }
    ret = iosf_mbi_block_punit_i2c_access();
    if (ret < 0)
    return ret;
    info.vbus_attach = axp288_get_vbus_attach(info);
    axp288_extcon_log_rsi(info);
    iosf_mbi_unblock_punit_i2c_access();
// Initialize extcon device
    info.edev = devm_extcon_dev_allocate(&pdev.dev,
    axp288_extcon_cables);
    if (IS_ERR(info.edev)) {
    dev_err(&pdev.dev, "failed to allocate memory for extcon\n");
    return PTR_ERR(info.edev);
    }
// Register extcon device
    ret = devm_extcon_dev_register(&pdev.dev, info.edev);
    if (ret) {
    dev_err(&pdev.dev, "failed to register extcon device\n");
    return ret;
    }
    for (i = 0; i < EXTCON_IRQ_END; i++) {
    pirq = platform_get_irq(pdev, i);
    if (pirq < 0)
    return pirq;
    info.irq[i] = regmap_irq_get_virq(info.regmap_irqc, pirq);
    if (info.irq[i] < 0) {
    dev_err(&pdev.dev,
    "failed to get virtual interrupt=%d\n", pirq);
    ret = info.irq[i];
    return ret;
    }
    ret = devm_request_threaded_irq(&pdev.dev, info.irq[i],
    core::ptr::null_mut(), axp288_extcon_isr,
    IRQF_ONESHOT | IRQF_NO_SUSPEND,
    pdev.name, info);
    if (ret) {
    dev_err(&pdev.dev, "failed to request interrupt=%d\n",
    info.irq[i]);
    return ret;
    }
    }
    if (info.id_extcon) {
    ret = devm_extcon_register_notifier_all(dev, info.id_extcon,
    &info.id_nb);
    if (ret)
    return ret;
    }
// Make sure the role-sw is set correctly before doing BC detection
    if (info.role_sw) {
    queue_work(system_long_wq, &info.role_work);
    flush_work(&info.role_work);
    }
// Start charger cable type detection
    ret = axp288_extcon_enable(info);
    if (ret < 0)
    return ret;
    devm_device_init_wakeup(dev);
    platform_set_drvdata(pdev, info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused axp288_extcon_suspend(struct device *dev)
    {
    struct axp288_extcon_info *info = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(info.irq[VBUS_RISING_IRQ]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp288_extcon_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused axp288_extcon_resume(struct device *dev)
    {
    struct axp288_extcon_info *info = dev_get_drvdata(dev);
//
// Wakeup when a charger is connected to do charger-type
// connection and generate an extcon event which makes the
// axp288 charger driver set the input current limit.
//
    if (device_may_wakeup(dev))
    disable_irq_wake(info.irq[VBUS_RISING_IRQ]);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(axp288_extcon_pm_ops, axp288_extcon_suspend,
    axp288_extcon_resume);
    static const struct platform_device_id axp288_extcon_table[] = {
    { .name = "axp288_extcon" },
    {},
    };
    MODULE_DEVICE_TABLE(platform, axp288_extcon_table);
    static struct platform_driver axp288_extcon_driver = {
    .probe = axp288_extcon_probe,
    .id_table = axp288_extcon_table,
    .driver = {
    .name = "axp288_extcon",
    .pm = &axp288_extcon_pm_ops,
    },
    };
    module_platform_driver(axp288_extcon_driver);
    MODULE_AUTHOR("Ramakrishna Pallala <ramakrishna.pallala@intel.com>");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("X-Powers AXP288 extcon driver");
    MODULE_LICENSE("GPL v2");
