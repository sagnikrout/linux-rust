//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/stusb160x.c
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
// STMicroelectronics STUSB160x Type-C controller family driver
//
// Copyright (C) 2020, STMicroelectronics
// Author(s): Amelie Delaunay <amelie.delaunay@st.com>
//

pub const STUSB160X_ALERT_STATUS: c_uint = 0x0B /* RC */;
pub const STUSB160X_ALERT_STATUS_MASK_CTRL: c_uint = 0x0C /* RW */;
pub const STUSB160X_CC_CONNECTION_STATUS_TRANS: c_uint = 0x0D /* RC */;
pub const STUSB160X_CC_CONNECTION_STATUS: c_uint = 0x0E /* RO */;
pub const STUSB160X_MONITORING_STATUS_TRANS: c_uint = 0x0F /* RC */;
pub const STUSB160X_MONITORING_STATUS: c_uint = 0x10 /* RO */;
pub const STUSB160X_CC_OPERATION_STATUS: c_uint = 0x11 /* RO */;
pub const STUSB160X_HW_FAULT_STATUS_TRANS: c_uint = 0x12 /* RC */;
pub const STUSB160X_HW_FAULT_STATUS: c_uint = 0x13 /* RO */;
pub const STUSB160X_CC_CAPABILITY_CTRL: c_uint = 0x18 /* RW */;
pub const STUSB160X_CC_VCONN_SWITCH_CTRL: c_uint = 0x1E /* RW */;
pub const STUSB160X_VCONN_MONITORING_CTRL: c_uint = 0x20 /* RW */;
pub const STUSB160X_VBUS_MONITORING_RANGE_CTRL: c_uint = 0x22 /* RW */;
pub const STUSB160X_RESET_CTRL: c_uint = 0x23 /* RW */;
pub const STUSB160X_VBUS_DISCHARGE_TIME_CTRL: c_uint = 0x25 /* RW */;
pub const STUSB160X_VBUS_DISCHARGE_STATUS: c_uint = 0x26 /* RO */;
pub const STUSB160X_VBUS_ENABLE_STATUS: c_uint = 0x27 /* RO */;
pub const STUSB160X_CC_POWER_MODE_CTRL: c_uint = 0x28 /* RW */;
pub const STUSB160X_VBUS_MONITORING_CTRL: c_uint = 0x2E /* RW */;
pub const STUSB1600_REG_MAX: c_uint = 0x2F /* RO - Reserved */;
// STUSB160X_ALERT_STATUS/STUSB160X_ALERT_STATUS_MASK_CTRL bitfields

// STUSB160X_CC_CONNECTION_STATUS_TRANS bitfields

// STUSB160X_CC_CONNECTION_STATUS bitfields

// STUSB160X_MONITORING_STATUS_TRANS bitfields

// STUSB160X_MONITORING_STATUS bitfields

// STUSB160X_CC_OPERATION_STATUS bitfields

// STUSB160X_HW_FAULT_STATUS_TRANS bitfields

// STUSB160X_HW_FAULT_STATUS bitfields

// STUSB160X_CC_CAPABILITY_CTRL bitfields

// STUSB160X_VCONN_SWITCH_CTRL bitfields

// STUSB160X_VCONN_MONITORING_CTRL bitfields

// STUSB160X_VBUS_MONITORING_RANGE_CTRL bitfields

// STUSB160X_RESET_CTRL bitfields

// STUSB160X_VBUS_DISCHARGE_TIME_CTRL bitfields

// STUSB160X_VBUS_DISCHARGE_STATUS bitfields

// STUSB160X_VBUS_ENABLE_STATUS bitfields

// STUSB160X_CC_POWER_MODE_CTRL bitfields

// STUSB160X_VBUS_MONITORING_CTRL bitfields

    enum stusb160x_pwr_mode {
    SOURCE_WITH_ACCESSORY,
    SINK_WITH_ACCESSORY,
    SINK_WITHOUT_ACCESSORY,
    DUAL_WITH_ACCESSORY,
    DUAL_WITH_ACCESSORY_AND_TRY_SRC,
    DUAL_WITH_ACCESSORY_AND_TRY_SNK,
    };
    enum stusb160x_attached_mode {
    NO_DEVICE_ATTACHED,
    SINK_ATTACHED,
    SOURCE_ATTACHED,
    DEBUG_ACCESSORY_ATTACHED,
    AUDIO_ACCESSORY_ATTACHED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stusb160x {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub vdd_supply: *mut regulator,
    pub vsys_supply: *mut regulator,
    pub vconn_supply: *mut regulator,
    pub main_supply: *mut regulator,
    pub port: *mut typec_port,
    pub capability: typec_capability,
    pub partner: *mut typec_partner,
    pub port_type: enum typec_port_type,
    pub pwr_opmode: enum typec_pwr_opmode,
    pub vbus_on: bool,
    pub role_sw: *mut usb_role_switch,
}

#[no_mangle]
unsafe extern "C" fn stusb160x_reg_writeable(dev: *mut device, reg: c_uint) -> bool {
    static bool stusb160x_reg_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STUSB160X_ALERT_STATUS_MASK_CTRL:
    case STUSB160X_CC_CAPABILITY_CTRL:
    case STUSB160X_CC_VCONN_SWITCH_CTRL:
    case STUSB160X_VCONN_MONITORING_CTRL:
    case STUSB160X_VBUS_MONITORING_RANGE_CTRL:
    case STUSB160X_RESET_CTRL:
    case STUSB160X_VBUS_DISCHARGE_TIME_CTRL:
    case STUSB160X_CC_POWER_MODE_CTRL:
    case STUSB160X_VBUS_MONITORING_CTRL:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_reg_readable(dev: *mut device, reg: c_uint) -> bool {
    static bool stusb160x_reg_readable(struct device *dev, unsigned int reg)
    {
    if (reg <= 0x0A ||
    (reg >= 0x14 && reg <= 0x17) ||
    (reg >= 0x19 && reg <= 0x1D) ||
    (reg >= 0x29 && reg <= 0x2D) ||
    (reg == 0x1F || reg == 0x21 || reg == 0x24 || reg == 0x2F))
    return false;
    else
    return true;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_reg_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool stusb160x_reg_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STUSB160X_ALERT_STATUS:
    case STUSB160X_CC_CONNECTION_STATUS_TRANS:
    case STUSB160X_CC_CONNECTION_STATUS:
    case STUSB160X_MONITORING_STATUS_TRANS:
    case STUSB160X_MONITORING_STATUS:
    case STUSB160X_CC_OPERATION_STATUS:
    case STUSB160X_HW_FAULT_STATUS_TRANS:
    case STUSB160X_HW_FAULT_STATUS:
    case STUSB160X_VBUS_DISCHARGE_STATUS:
    case STUSB160X_VBUS_ENABLE_STATUS:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_reg_precious(dev: *mut device, reg: c_uint) -> bool {
    static bool stusb160x_reg_precious(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case STUSB160X_ALERT_STATUS:
    case STUSB160X_CC_CONNECTION_STATUS_TRANS:
    case STUSB160X_MONITORING_STATUS_TRANS:
    case STUSB160X_HW_FAULT_STATUS_TRANS:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config stusb1600_regmap_config = {
    .reg_bits	= 8,
    .reg_stride	= 1,
    .val_bits	= 8,
    .max_register	= STUSB1600_REG_MAX,
    .writeable_reg	= stusb160x_reg_writeable,
    .readable_reg	= stusb160x_reg_readable,
    .volatile_reg	= stusb160x_reg_volatile,
    .precious_reg	= stusb160x_reg_precious,
    .cache_type	= REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn stusb160x_get_vconn(chip: *mut stusb160x) -> bool {
    static bool stusb160x_get_vconn(struct stusb160x *chip)
    {
    u32 val;
    int ret;
    ret = regmap_read(chip.regmap, STUSB160X_CC_CAPABILITY_CTRL, &val);
    if (ret) {
    dev_err(chip.dev, "Unable to get Vconn status: %d\n", ret);
    return false;
    }
    return !!FIELD_GET(STUSB160X_CC_VCONN_SUPPLY_EN, val);
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_set_vconn(chip: *mut stusb160x, on: bool) -> c_int {
    static int stusb160x_set_vconn(struct stusb160x *chip, bool on)
    {
    int ret;
// Manage VCONN input supply
    if (chip.vconn_supply) {
    if (on) {
    ret = regulator_enable(chip.vconn_supply);
    if (ret) {
    dev_err(chip.dev,
    "failed to enable vconn supply: %d\n",
    ret);
    return ret;
    }
    } else {
    regulator_disable(chip.vconn_supply);
    }
    }
// Manage VCONN monitoring and power path
    ret = regmap_update_bits(chip.regmap, STUSB160X_VCONN_MONITORING_CTRL,
    STUSB160X_VCONN_MONITORING_EN,
    on ? STUSB160X_VCONN_MONITORING_EN : 0);
    if (ret)
    goto vconn_reg_disable;
    return 0;
    vconn_reg_disable:
    if (chip.vconn_supply && on)
    regulator_disable(chip.vconn_supply);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_get_pwr_opmode(chip: *mut stusb160x) -> enum typec_pwr_opmode {
    static enum typec_pwr_opmode stusb160x_get_pwr_opmode(struct stusb160x *chip)
    {
    u32 val;
    int ret;
    ret = regmap_read(chip.regmap, STUSB160X_CC_CAPABILITY_CTRL, &val);
    if (ret) {
    dev_err(chip.dev, "Unable to get pwr opmode: %d\n", ret);
    return TYPEC_PWR_MODE_USB;
    }
    return FIELD_GET(STUSB160X_CC_CURRENT_ADVERTISED, val);
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_get_accessory(status: u32) -> enum typec_accessory {
    static enum typec_accessory stusb160x_get_accessory(u32 status)
    {
    enum stusb160x_attached_mode mode;
    mode = FIELD_GET(STUSB160X_CC_ATTACHED_MODE, status);
    switch (mode) {
    case DEBUG_ACCESSORY_ATTACHED:
    return TYPEC_ACCESSORY_DEBUG;
    case AUDIO_ACCESSORY_ATTACHED:
    return TYPEC_ACCESSORY_AUDIO;
    default:
    return TYPEC_ACCESSORY_NONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_get_vconn_role(status: u32) -> enum typec_role {
    static enum typec_role stusb160x_get_vconn_role(u32 status)
    {
    if (FIELD_GET(STUSB160X_CC_VCONN_SUPPLY, status))
    return TYPEC_SOURCE;
    return TYPEC_SINK;
    }
    static void stusb160x_set_data_role(struct stusb160x *chip,
    enum typec_data_role data_role,
    bool attached)
    {
    let mut usb_role: enum usb_role = USB_ROLE_NONE;
    if (attached) {
    if (data_role == TYPEC_HOST)
    usb_role = USB_ROLE_HOST;
    else
    usb_role = USB_ROLE_DEVICE;
    }
    usb_role_switch_set_role(chip.role_sw, usb_role);
    typec_set_data_role(chip.port, data_role);
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_attach(chip: *mut stusb160x, status: u32) -> c_int {
    static int stusb160x_attach(struct stusb160x *chip, u32 status)
    {
    struct typec_partner_desc desc;
    int ret;
    if ((STUSB160X_CC_POWER_ROLE(status) == TYPEC_SOURCE) &&
    chip.vdd_supply) {
    ret = regulator_enable(chip.vdd_supply);
    if (ret) {
    dev_err(chip.dev,
    "Failed to enable Vbus supply: %d\n", ret);
    return ret;
    }
    chip.vbus_on = true;
    }
    desc.usb_pd = false;
    desc.accessory = stusb160x_get_accessory(status);
    desc.identity = core::ptr::null_mut();
    chip.partner = typec_register_partner(chip.port, &desc);
    if (IS_ERR(chip.partner)) {
    ret = PTR_ERR(chip.partner);
    goto vbus_disable;
    }
    typec_set_pwr_role(chip.port, STUSB160X_CC_POWER_ROLE(status));
    typec_set_pwr_opmode(chip.port, stusb160x_get_pwr_opmode(chip));
    typec_set_vconn_role(chip.port, stusb160x_get_vconn_role(status));
    stusb160x_set_data_role(chip, STUSB160X_CC_DATA_ROLE(status), true);
    return 0;
    vbus_disable:
    if (chip.vbus_on) {
    regulator_disable(chip.vdd_supply);
    chip.vbus_on = false;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_detach(chip: *mut stusb160x, status: u32) {
    static void stusb160x_detach(struct stusb160x *chip, u32 status)
    {
    typec_unregister_partner(chip.partner);
    chip.partner = core::ptr::null_mut();
    typec_set_pwr_role(chip.port, STUSB160X_CC_POWER_ROLE(status));
    typec_set_pwr_opmode(chip.port, TYPEC_PWR_MODE_USB);
    typec_set_vconn_role(chip.port, stusb160x_get_vconn_role(status));
    stusb160x_set_data_role(chip, STUSB160X_CC_DATA_ROLE(status), false);
    if (chip.vbus_on) {
    regulator_disable(chip.vdd_supply);
    chip.vbus_on = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t stusb160x_irq_handler(int irq, void *data)
    {
    struct stusb160x *chip = data;
    u32 pending, trans, status;
    int ret;
    ret = regmap_read(chip.regmap, STUSB160X_ALERT_STATUS, &pending);
    if (ret)
    goto err;
    if (pending & STUSB160X_CC_CONNECTION) {
    ret = regmap_read(chip.regmap,
    STUSB160X_CC_CONNECTION_STATUS_TRANS, &trans);
    if (ret)
    goto err;
    ret = regmap_read(chip.regmap,
    STUSB160X_CC_CONNECTION_STATUS, &status);
    if (ret)
    goto err;
    if (trans & STUSB160X_CC_ATTACH_TRANS) {
    if (status & STUSB160X_CC_ATTACH) {
    ret = stusb160x_attach(chip, status);
    if (ret)
    goto err;
    } else {
    stusb160x_detach(chip, status);
    }
    }
    }
    err:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_irq_init(chip: *mut stusb160x, irq: c_int) -> c_int {
    static int stusb160x_irq_init(struct stusb160x *chip, int irq)
    {
    u32 status;
    int ret;
    ret = regmap_read(chip.regmap,
    STUSB160X_CC_CONNECTION_STATUS, &status);
    if (ret)
    return ret;
    if (status & STUSB160X_CC_ATTACH) {
    ret = stusb160x_attach(chip, status);
    if (ret)
    dev_err(chip.dev, "attach failed: %d\n", ret);
    }
    ret = devm_request_threaded_irq(chip.dev, irq, core::ptr::null_mut(),
    stusb160x_irq_handler, IRQF_ONESHOT,
    dev_name(chip.dev), chip);
    if (ret)
    goto partner_unregister;
// Unmask CC_CONNECTION events
    ret = regmap_write_bits(chip.regmap, STUSB160X_ALERT_STATUS_MASK_CTRL,
    STUSB160X_CC_CONNECTION, 0);
    if (ret)
    goto partner_unregister;
    return 0;
    partner_unregister:
    if (chip.partner) {
    typec_unregister_partner(chip.partner);
    chip.partner = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_chip_init(chip: *mut stusb160x) -> c_int {
    static int stusb160x_chip_init(struct stusb160x *chip)
    {
    u32 val;
    int ret;
// Change the default Type-C power mode
    if (chip.port_type == TYPEC_PORT_SRC)
    ret = regmap_update_bits(chip.regmap,
    STUSB160X_CC_POWER_MODE_CTRL,
    STUSB160X_CC_POWER_MODE,
    SOURCE_WITH_ACCESSORY);
#[no_mangle]
pub unsafe extern "C" fn if(TYPEC_PORT_SNK: chip->port_type ==) -> else {
    else if (chip.port_type == TYPEC_PORT_SNK)
    ret = regmap_update_bits(chip.regmap,
    STUSB160X_CC_POWER_MODE_CTRL,
    STUSB160X_CC_POWER_MODE,
    SINK_WITH_ACCESSORY);
    else /* (chip.port_type == TYPEC_PORT_DRP) */
    ret = regmap_update_bits(chip.regmap,
    STUSB160X_CC_POWER_MODE_CTRL,
    STUSB160X_CC_POWER_MODE,
    DUAL_WITH_ACCESSORY);
    if (ret)
    return ret;
    if (chip.port_type == TYPEC_PORT_SNK)
    goto skip_src;
// Change the default Type-C Source power operation mode capability
    ret = regmap_update_bits(chip.regmap, STUSB160X_CC_CAPABILITY_CTRL,
    STUSB160X_CC_CURRENT_ADVERTISED,
    FIELD_PREP(STUSB160X_CC_CURRENT_ADVERTISED,
    chip.pwr_opmode));
    if (ret)
    return ret;
// Manage Type-C Source Vconn supply
    if (stusb160x_get_vconn(chip)) {
    ret = stusb160x_set_vconn(chip, true);
    if (ret)
    return ret;
    }
    skip_src:
// Mask all events interrupts - to be unmasked with interrupt support
    ret = regmap_update_bits(chip.regmap, STUSB160X_ALERT_STATUS_MASK_CTRL,
    STUSB160X_ALL_ALERTS, STUSB160X_ALL_ALERTS);
    if (ret)
    return ret;
// Read status at least once to clear any stale interrupts
    regmap_read(chip.regmap, STUSB160X_ALERT_STATUS, &val);
    regmap_read(chip.regmap, STUSB160X_CC_CONNECTION_STATUS_TRANS, &val);
    regmap_read(chip.regmap, STUSB160X_MONITORING_STATUS_TRANS, &val);
    regmap_read(chip.regmap, STUSB160X_HW_FAULT_STATUS_TRANS, &val);
    return 0;
    }
    static int stusb160x_get_fw_caps(struct stusb160x *chip,
    struct fwnode_handle *fwnode)
    {
    const char *cap_str;
    int ret;
    chip.capability.fwnode = fwnode;
//
// Supported port type can be configured through device tree
// else it is read from chip registers in stusb160x_get_caps.
//
    ret = fwnode_property_read_string(fwnode, "power-role", &cap_str);
    if (!ret) {
    ret = typec_find_port_power_role(cap_str);
    if (ret < 0)
    return ret;
    chip.port_type = ret;
    }
    chip.capability.type = chip.port_type;
// Skip DRP/Source capabilities in case of Sink only
    if (chip.port_type == TYPEC_PORT_SNK)
    return 0;
    if (chip.port_type == TYPEC_PORT_DRP)
    chip.capability.prefer_role = TYPEC_SINK;
//
// Supported power operation mode can be configured through device tree
// else it is read from chip registers in stusb160x_get_caps.
//
    ret = fwnode_property_read_string(fwnode, "typec-power-opmode", &cap_str);
    if (!ret) {
    ret = typec_find_pwr_opmode(cap_str);
// Power delivery not yet supported
    if (ret < 0 || ret == TYPEC_PWR_MODE_PD) {
    dev_err(chip.dev, "bad power operation mode: %d\n", ret);
    return -EINVAL;
    }
    chip.pwr_opmode = ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_get_caps(chip: *mut stusb160x) -> c_int {
    static int stusb160x_get_caps(struct stusb160x *chip)
    {
    enum typec_port_type *type = &chip.capability.type;
    enum typec_port_data *data = &chip.capability.data;
    enum typec_accessory *accessory = chip.capability.accessory;
    u32 val;
    int ret;
    chip.capability.revision = USB_TYPEC_REV_1_2;
    ret = regmap_read(chip.regmap, STUSB160X_CC_POWER_MODE_CTRL, &val);
    if (ret)
    return ret;
    switch (FIELD_GET(STUSB160X_CC_POWER_MODE, val)) {
    case SOURCE_WITH_ACCESSORY:
// type = TYPEC_PORT_SRC;
// data = TYPEC_PORT_DFP;
// accessory++ = TYPEC_ACCESSORY_AUDIO;
// accessory++ = TYPEC_ACCESSORY_DEBUG;
    break;
    case SINK_WITH_ACCESSORY:
// type = TYPEC_PORT_SNK;
// data = TYPEC_PORT_UFP;
// accessory++ = TYPEC_ACCESSORY_AUDIO;
// accessory++ = TYPEC_ACCESSORY_DEBUG;
    break;
    case SINK_WITHOUT_ACCESSORY:
// type = TYPEC_PORT_SNK;
// data = TYPEC_PORT_UFP;
    break;
    case DUAL_WITH_ACCESSORY:
    case DUAL_WITH_ACCESSORY_AND_TRY_SRC:
    case DUAL_WITH_ACCESSORY_AND_TRY_SNK:
// type = TYPEC_PORT_DRP;
// data = TYPEC_PORT_DRD;
// accessory++ = TYPEC_ACCESSORY_AUDIO;
// accessory++ = TYPEC_ACCESSORY_DEBUG;
    break;
    default:
    return -EINVAL;
    }
    chip.port_type = *type;
    chip.pwr_opmode = stusb160x_get_pwr_opmode(chip);
    return 0;
    }
    static const struct of_device_id stusb160x_of_match[] = {
    { .compatible = "st,stusb1600", .data = &stusb1600_regmap_config},
    {},
    };
    MODULE_DEVICE_TABLE(of, stusb160x_of_match);
#[no_mangle]
unsafe extern "C" fn stusb160x_probe(client: *mut i2c_client) -> c_int {
    static int stusb160x_probe(struct i2c_client *client)
    {
    const struct regmap_config *regmap_config;
    struct stusb160x *chip;
    struct fwnode_handle *fwnode;
    int ret;
    chip = devm_kzalloc(&client.dev, sizeof(struct stusb160x), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    i2c_set_clientdata(client, chip);
    regmap_config = i2c_get_match_data(client);
    chip.regmap = devm_regmap_init_i2c(client, regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    dev_err(&client.dev,
    "Failed to allocate register map:%d\n", ret);
    return ret;
    }
    chip.dev = &client.dev;
    chip.vsys_supply = devm_regulator_get_optional(chip.dev, "vsys");
    if (IS_ERR(chip.vsys_supply)) {
    ret = PTR_ERR(chip.vsys_supply);
    if (ret != -ENODEV)
    return ret;
    chip.vsys_supply = core::ptr::null_mut();
    }
    chip.vdd_supply = devm_regulator_get_optional(chip.dev, "vdd");
    if (IS_ERR(chip.vdd_supply)) {
    ret = PTR_ERR(chip.vdd_supply);
    if (ret != -ENODEV)
    return ret;
    chip.vdd_supply = core::ptr::null_mut();
    }
    chip.vconn_supply = devm_regulator_get_optional(chip.dev, "vconn");
    if (IS_ERR(chip.vconn_supply)) {
    ret = PTR_ERR(chip.vconn_supply);
    if (ret != -ENODEV)
    return ret;
    chip.vconn_supply = core::ptr::null_mut();
    }
    fwnode = device_get_named_child_node(chip.dev, "connector");
    if (!fwnode)
    return -ENODEV;
//
// This fwnode has a "compatible" property, but is never populated as a
// struct device. Instead we simply parse it to read the properties.
// This it breaks fw_devlink=on. To maintain backward compatibility
// with existing DT files, we work around this by deleting any
// fwnode_links to/from this fwnode.
//
    fw_devlink_purge_absent_suppliers(fwnode);
//
// When both VDD and VSYS power supplies are present, the low power
// supply VSYS is selected when VSYS voltage is above 3.1 V.
// Otherwise VDD is selected.
//
    if (chip.vdd_supply &&
    (!chip.vsys_supply ||
    (regulator_get_voltage(chip.vsys_supply) <= 3100000)))
    chip.main_supply = chip.vdd_supply;
    else
    chip.main_supply = chip.vsys_supply;
    if (chip.main_supply) {
    ret = regulator_enable(chip.main_supply);
    if (ret) {
    dev_err(chip.dev,
    "Failed to enable main supply: %d\n", ret);
    goto fwnode_put;
    }
    }
// Get configuration from chip
    ret = stusb160x_get_caps(chip);
    if (ret) {
    dev_err(chip.dev, "Failed to get port caps: %d\n", ret);
    goto main_reg_disable;
    }
// Get optional re-configuration from device tree
    ret = stusb160x_get_fw_caps(chip, fwnode);
    if (ret) {
    dev_err(chip.dev, "Failed to get connector caps: %d\n", ret);
    goto main_reg_disable;
    }
    ret = stusb160x_chip_init(chip);
    if (ret) {
    dev_err(chip.dev, "Failed to init port: %d\n", ret);
    goto main_reg_disable;
    }
    chip.port = typec_register_port(chip.dev, &chip.capability);
    if (IS_ERR(chip.port)) {
    ret = PTR_ERR(chip.port);
    goto all_reg_disable;
    }
//
// Default power operation mode initialization: will be updated upon
// attach/detach interrupt
//
    typec_set_pwr_opmode(chip.port, chip.pwr_opmode);
    if (client.irq) {
    chip.role_sw = fwnode_usb_role_switch_get(fwnode);
    if (IS_ERR(chip.role_sw)) {
    ret = dev_err_probe(chip.dev, PTR_ERR(chip.role_sw),
    "Failed to get usb role switch\n");
    goto port_unregister;
    }
    ret = stusb160x_irq_init(chip, client.irq);
    if (ret)
    goto role_sw_put;
    } else {
//
// If Source or Dual power role, need to enable VDD supply
// providing Vbus if present. In case of interrupt support,
// VDD supply will be dynamically managed upon attach/detach
// interrupt.
//
    if (chip.port_type != TYPEC_PORT_SNK && chip.vdd_supply) {
    ret = regulator_enable(chip.vdd_supply);
    if (ret) {
    dev_err(chip.dev,
    "Failed to enable VDD supply: %d\n",
    ret);
    goto port_unregister;
    }
    chip.vbus_on = true;
    }
    }
    fwnode_handle_put(fwnode);
    return 0;
    role_sw_put:
    if (chip.role_sw)
    usb_role_switch_put(chip.role_sw);
    port_unregister:
    typec_unregister_port(chip.port);
    all_reg_disable:
    if (stusb160x_get_vconn(chip))
    stusb160x_set_vconn(chip, false);
    main_reg_disable:
    if (chip.main_supply)
    regulator_disable(chip.main_supply);
    fwnode_put:
    fwnode_handle_put(fwnode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_remove(client: *mut i2c_client) {
    static void stusb160x_remove(struct i2c_client *client)
    {
    struct stusb160x *chip = i2c_get_clientdata(client);
    if (chip.partner) {
    typec_unregister_partner(chip.partner);
    chip.partner = core::ptr::null_mut();
    }
    if (chip.vbus_on)
    regulator_disable(chip.vdd_supply);
    if (chip.role_sw)
    usb_role_switch_put(chip.role_sw);
    typec_unregister_port(chip.port);
    if (stusb160x_get_vconn(chip))
    stusb160x_set_vconn(chip, false);
    if (chip.main_supply)
    regulator_disable(chip.main_supply);
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stusb160x_suspend(struct device *dev)
    {
    struct stusb160x *chip = dev_get_drvdata(dev);
// Mask interrupts
    return regmap_update_bits(chip.regmap,
    STUSB160X_ALERT_STATUS_MASK_CTRL,
    STUSB160X_ALL_ALERTS, STUSB160X_ALL_ALERTS);
    }
#[no_mangle]
unsafe extern "C" fn stusb160x_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stusb160x_resume(struct device *dev)
    {
    struct stusb160x *chip = dev_get_drvdata(dev);
    u32 status;
    int ret;
    ret = regcache_sync(chip.regmap);
    if (ret)
    return ret;
// Check if attach/detach occurred during low power
    ret = regmap_read(chip.regmap,
    STUSB160X_CC_CONNECTION_STATUS, &status);
    if (ret)
    return ret;
    if (chip.partner && !(status & STUSB160X_CC_ATTACH))
    stusb160x_detach(chip, status);
    if (!chip.partner && (status & STUSB160X_CC_ATTACH)) {
    ret = stusb160x_attach(chip, status);
    if (ret)
    dev_err(chip.dev, "attach failed: %d\n", ret);
    }
// Unmask interrupts
    return regmap_write_bits(chip.regmap, STUSB160X_ALERT_STATUS_MASK_CTRL,
    STUSB160X_CC_CONNECTION, 0);
    }
    static SIMPLE_DEV_PM_OPS(stusb160x_pm_ops, stusb160x_suspend, stusb160x_resume);
    static struct i2c_driver stusb160x_driver = {
    .driver = {
    .name = "stusb160x",
    .pm = &stusb160x_pm_ops,
    .of_match_table = stusb160x_of_match,
    },
    .probe = stusb160x_probe,
    .remove = stusb160x_remove,
    };
    module_i2c_driver(stusb160x_driver);
    MODULE_AUTHOR("Amelie Delaunay <amelie.delaunay@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STUSB160x Type-C controller driver");
    MODULE_LICENSE("GPL v2");
