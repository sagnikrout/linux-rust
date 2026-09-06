//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-ptn5150.c
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
// extcon-ptn5150.c - PTN5150 CC logic extcon driver to support USB detection
//
// Based on extcon-sm5502.c driver
// Copyright (c) 2018-2019 by Vijai Kumar K
// Author: Vijai Kumar K <vijaikumar.kanagarajan@gmail.com>
// Copyright (c) 2020 Krzysztof Kozlowski <krzk@kernel.org>

// PTN5150 registers
pub const PTN5150_REG_DEVICE_ID: c_uint = 0x01;
pub const PTN5150_REG_CONTROL: c_uint = 0x02;
pub const PTN5150_REG_INT_STATUS: c_uint = 0x03;
pub const PTN5150_REG_CC_STATUS: c_uint = 0x04;
pub const PTN5150_REG_CON_DET: c_uint = 0x09;
pub const PTN5150_REG_VCONN_STATUS: c_uint = 0x0a;
pub const PTN5150_REG_RESET: c_uint = 0x0b;
pub const PTN5150_REG_INT_MASK: c_uint = 0x18;
pub const PTN5150_REG_INT_REG_STATUS: c_uint = 0x19;

pub const PTN5150_DFP_ATTACHED: c_uint = 0x1;
pub const PTN5150_UFP_ATTACHED: c_uint = 0x2;
// Define PTN5150 MASK/SHIFT constant

pub const PTN5150_POLARITY_CC1: c_uint = 0x1;
pub const PTN5150_POLARITY_CC2: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptn5150_info {
    pub dev: *mut device,
    pub edev: *mut extcon_dev,
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub int_gpiod: *mut gpio_desc,
    pub vbus_gpiod: *mut gpio_desc,
    pub irq: c_int,
    pub irq_work: work_struct,
    pub mutex: mutex,
    pub orient_sw: *mut typec_switch,
    pub role_sw: *mut usb_role_switch,
}

// List of detectable cables
    static const unsigned int ptn5150_extcon_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_NONE,
    };
    static const struct regmap_config ptn5150_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= PTN5150_REG_END,
    };
#[no_mangle]
unsafe extern "C" fn ptn5150_check_state(info: *mut ptn5150_info) {
    static void ptn5150_check_state(struct ptn5150_info *info)
    {
    let mut orient: enum typec_orientation = TYPEC_ORIENTATION_NONE;
    unsigned int port_status, reg_data, vbus;
    let mut usb_role: enum usb_role = USB_ROLE_NONE;
    int ret;
    ret = regmap_read(info.regmap, PTN5150_REG_CC_STATUS, &reg_data);
    if (ret) {
    dev_err(info.dev, "failed to read CC STATUS %d\n", ret);
    return;
    }
    orient = FIELD_GET(PTN5150_REG_CC_POLARITY, reg_data);
    switch (orient) {
    case PTN5150_POLARITY_CC1:
    orient = TYPEC_ORIENTATION_NORMAL;
    break;
    case PTN5150_POLARITY_CC2:
    orient = TYPEC_ORIENTATION_REVERSE;
    break;
    default:
    orient = TYPEC_ORIENTATION_NONE;
    break;
    }
    ret = typec_switch_set(info.orient_sw, orient);
    if (ret)
    dev_err(info.dev, "failed to set orientation: %d\n", ret);
    port_status = FIELD_GET(PTN5150_REG_CC_PORT_ATTACHMENT, reg_data);
    switch (port_status) {
    case PTN5150_DFP_ATTACHED:
    extcon_set_state_sync(info.edev, EXTCON_USB_HOST, false);
    gpiod_set_value_cansleep(info.vbus_gpiod, 0);
    extcon_set_state_sync(info.edev, EXTCON_USB, true);
    usb_role = USB_ROLE_DEVICE;
    break;
    case PTN5150_UFP_ATTACHED:
    extcon_set_state_sync(info.edev, EXTCON_USB, false);
    vbus = FIELD_GET(PTN5150_REG_CC_VBUS_DETECTION, reg_data);
    if (vbus)
    gpiod_set_value_cansleep(info.vbus_gpiod, 0);
    else
    gpiod_set_value_cansleep(info.vbus_gpiod, 1);
    extcon_set_state_sync(info.edev, EXTCON_USB_HOST, true);
    usb_role = USB_ROLE_HOST;
    break;
    default:
    break;
    }
    if (usb_role) {
    ret = usb_role_switch_set_role(info.role_sw, usb_role);
    if (ret)
    dev_err(info.dev, "failed to set %s role: %d\n",
    usb_role_string(usb_role), ret);
    }
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_irq_work(work: *mut work_struct) {
    static void ptn5150_irq_work(struct work_struct *work)
    {
    struct ptn5150_info *info = container_of(work,
    struct ptn5150_info, irq_work);
    let mut ret: c_int = 0;
    unsigned int int_status;
    if (!info.edev)
    return;
    mutex_lock(&info.mutex);
// Clear interrupt. Read would clear the register
    ret = regmap_read(info.regmap, PTN5150_REG_INT_STATUS, &int_status);
    if (ret) {
    dev_err(info.dev, "failed to read INT STATUS %d\n", ret);
    mutex_unlock(&info.mutex);
    return;
    }
    if (int_status) {
    unsigned int cable_attach;
    cable_attach = int_status & PTN5150_REG_INT_CABLE_ATTACH_MASK;
    if (cable_attach) {
    ptn5150_check_state(info);
    } else {
    extcon_set_state_sync(info.edev,
    EXTCON_USB_HOST, false);
    extcon_set_state_sync(info.edev,
    EXTCON_USB, false);
    gpiod_set_value_cansleep(info.vbus_gpiod, 0);
    ret = usb_role_switch_set_role(info.role_sw,
    USB_ROLE_NONE);
    if (ret)
    dev_err(info.dev,
    "failed to set none role: %d\n",
    ret);
    ret = typec_switch_set(info.orient_sw,
    TYPEC_ORIENTATION_NONE);
    if (ret)
    dev_err(info.dev,
    "failed to set orientation: %d\n", ret);
    }
    }
// Clear interrupt. Read would clear the register
    ret = regmap_read(info.regmap, PTN5150_REG_INT_REG_STATUS,
    &int_status);
    if (ret) {
    dev_err(info.dev,
    "failed to read INT REG STATUS %d\n", ret);
    mutex_unlock(&info.mutex);
    return;
    }
    mutex_unlock(&info.mutex);
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ptn5150_irq_handler(int irq, void *data)
    {
    struct ptn5150_info *info = data;
    schedule_work(&info.irq_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_init_dev_type(info: *mut ptn5150_info) -> c_int {
    static int ptn5150_init_dev_type(struct ptn5150_info *info)
    {
    unsigned int reg_data, vendor_id, version_id;
    int ret;
    ret = regmap_read(info.regmap, PTN5150_REG_DEVICE_ID, &reg_data);
    if (ret) {
    dev_err(info.dev, "failed to read DEVICE_ID %d\n", ret);
    return -EINVAL;
    }
    vendor_id = FIELD_GET(PTN5150_REG_DEVICE_ID_VENDOR, reg_data);
    version_id = FIELD_GET(PTN5150_REG_DEVICE_ID_VERSION, reg_data);
    dev_dbg(info.dev, "Device type: version: 0x%x, vendor: 0x%x\n",
    version_id, vendor_id);
// Clear any existing interrupts
    ret = regmap_read(info.regmap, PTN5150_REG_INT_STATUS, &reg_data);
    if (ret) {
    dev_err(info.dev,
    "failed to read PTN5150_REG_INT_STATUS %d\n",
    ret);
    return -EINVAL;
    }
    ret = regmap_read(info.regmap, PTN5150_REG_INT_REG_STATUS, &reg_data);
    if (ret) {
    dev_err(info.dev,
    "failed to read PTN5150_REG_INT_REG_STATUS %d\n", ret);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_work_sync_and_put(data: *mut c_void) {
    static void ptn5150_work_sync_and_put(void *data)
    {
    struct ptn5150_info *info = data;
    cancel_work_sync(&info.irq_work);
    usb_role_switch_put(info.role_sw);
    typec_switch_put(info.orient_sw);
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ptn5150_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct device_node *np = i2c.dev.of_node;
    struct fwnode_handle *connector;
    struct ptn5150_info *info;
    int ret;
    if (!np)
    return -EINVAL;
    info = devm_kzalloc(&i2c.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    i2c_set_clientdata(i2c, info);
    info.dev = &i2c.dev;
    info.i2c = i2c;
    info.vbus_gpiod = devm_gpiod_get(&i2c.dev, "vbus", GPIOD_OUT_LOW);
    if (IS_ERR(info.vbus_gpiod)) {
    ret = PTR_ERR(info.vbus_gpiod);
    if (ret == -ENOENT) {
    dev_info(dev, "No VBUS GPIO, ignoring VBUS control\n");
    info.vbus_gpiod = core::ptr::null_mut();
    } else {
    return dev_err_probe(dev, ret, "failed to get VBUS GPIO\n");
    }
    }
    mutex_init(&info.mutex);
    INIT_WORK(&info.irq_work, ptn5150_irq_work);
    info.regmap = devm_regmap_init_i2c(i2c, &ptn5150_regmap_config);
    if (IS_ERR(info.regmap)) {
    return dev_err_probe(info.dev, PTR_ERR(info.regmap),
    "failed to allocate register map\n");
    }
    if (i2c.irq > 0) {
    info.irq = i2c.irq;
    } else {
    info.int_gpiod = devm_gpiod_get(&i2c.dev, "int", GPIOD_IN);
    if (IS_ERR(info.int_gpiod)) {
    return dev_err_probe(dev, PTR_ERR(info.int_gpiod),
    "failed to get INT GPIO\n");
    }
    info.irq = gpiod_to_irq(info.int_gpiod);
    if (info.irq < 0) {
    dev_err(dev, "failed to get INTB IRQ\n");
    return info.irq;
    }
    }
    ret = devm_request_threaded_irq(dev, info.irq, core::ptr::null_mut(),
    ptn5150_irq_handler,
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    i2c.name, info);
    if (ret < 0) {
    dev_err(dev, "failed to request handler for INTB IRQ\n");
    return ret;
    }
// Allocate extcon device
    info.edev = devm_extcon_dev_allocate(info.dev, ptn5150_extcon_cable);
    if (IS_ERR(info.edev)) {
    dev_err(info.dev, "failed to allocate memory for extcon\n");
    return -ENOMEM;
    }
// Register extcon device
    ret = devm_extcon_dev_register(info.dev, info.edev);
    if (ret) {
    dev_err(info.dev, "failed to register extcon device\n");
    return ret;
    }
    extcon_set_property_capability(info.edev, EXTCON_USB,
    EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_VBUS);
    extcon_set_property_capability(info.edev, EXTCON_USB_HOST,
    EXTCON_PROP_USB_TYPEC_POLARITY);
// Initialize PTN5150 device and print vendor id and version id
    ret = ptn5150_init_dev_type(info);
    if (ret)
    return -EINVAL;
    connector = device_get_named_child_node(dev, "connector");
    if (connector) {
    info.orient_sw = fwnode_typec_switch_get(connector);
    if (IS_ERR(info.orient_sw))
    return dev_err_probe(info.dev, PTR_ERR(info.orient_sw),
    "failed to get orientation switch\n");
    }
    info.role_sw = usb_role_switch_get(info.dev);
    if (!info.role_sw && connector)
    info.role_sw = fwnode_usb_role_switch_get(connector);
    if (IS_ERR(info.role_sw))
    return dev_err_probe(info.dev, PTR_ERR(info.role_sw),
    "failed to get role switch\n");
    ret = devm_add_action_or_reset(dev, ptn5150_work_sync_and_put, info);
    if (ret)
    return ret;
//
// Update current extcon state if for example OTG connection was there
// before the probe
//
    mutex_lock(&info.mutex);
    ptn5150_check_state(info);
    mutex_unlock(&info.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptn5150_resume(dev: *mut device) -> c_int {
    static int ptn5150_resume(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    struct ptn5150_info *info = i2c_get_clientdata(i2c);
// Need to check possible pending interrupt events
    schedule_work(&info.irq_work);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ptn5150_pm_ops, core::ptr::null_mut(), ptn5150_resume);
    static const struct of_device_id ptn5150_dt_match[] = {
    { .compatible = "nxp,ptn5150" },
    { },
    };
    MODULE_DEVICE_TABLE(of, ptn5150_dt_match);
    static const struct i2c_device_id ptn5150_i2c_id[] = {
    { "ptn5150" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ptn5150_i2c_id);
    static struct i2c_driver ptn5150_i2c_driver = {
    .driver		= {
    .name	= "ptn5150",
    .pm = pm_sleep_ptr(&ptn5150_pm_ops),
    .of_match_table = ptn5150_dt_match,
    },
    .probe		= ptn5150_i2c_probe,
    .id_table = ptn5150_i2c_id,
    };
    module_i2c_driver(ptn5150_i2c_driver);
    MODULE_DESCRIPTION("NXP PTN5150 CC logic Extcon driver");
    MODULE_AUTHOR("Vijai Kumar K <vijaikumar.kanagarajan@gmail.com>");
    MODULE_AUTHOR("Krzysztof Kozlowski <krzk@kernel.org>");
    MODULE_LICENSE("GPL v2");
