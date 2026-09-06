//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/it5205.c
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
// ITE IT5205 Type-C USB alternate mode passive mux
//
// Copyright (c) 2020 MediaTek Inc.
// Copyright (c) 2024 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

pub const IT5205FN_CHIP_ID: c_uint = 0x35303235 /* "5025" -> "5205" */;
// MUX power down register
pub const IT5205_REG_MUXPDR: c_uint = 0x10;

// MUX control register
pub const IT5205_REG_MUXCR: c_uint = 0x11;

pub const IT5205_DP: c_uint = 0x0f;
pub const IT5205_DP_USB: c_uint = 0x03;
pub const IT5205_USB: c_uint = 0x07;
// Vref Select Register
pub const IT5205_REG_VSR: c_uint = 0x10;

pub const IT5205_VREF_SELECT_3_3V: c_uint = 0x00;
pub const IT5205_VREF_SELECT_OFF: c_uint = 0x20;
// CSBU Over Voltage Protection Register
pub const IT5205_REG_CSBUOVPSR: c_uint = 0x1e;

pub const IT5205_OVP_3_90V: c_uint = 0x00;
pub const IT5205_OVP_3_68V: c_uint = 0x10;
pub const IT5205_OVP_3_62V: c_uint = 0x20;
pub const IT5205_OVP_3_57V: c_uint = 0x30;
// CSBU Switch Register
pub const IT5205_REG_CSBUSR: c_uint = 0x22;

// Interrupt Switch Register
pub const IT5205_REG_ISR: c_uint = 0x25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct it5205 {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub sw: *mut typec_switch_dev,
    pub mux: *mut typec_mux_dev,
}

#[no_mangle]
unsafe extern "C" fn it5205_switch_set(sw: *mut typec_switch_dev, orientation: enum typec_orientation) -> c_int {
    static int it5205_switch_set(struct typec_switch_dev *sw, enum typec_orientation orientation)
    {
    struct it5205 *it = typec_switch_get_drvdata(sw);
    switch (orientation) {
    case TYPEC_ORIENTATION_NORMAL:
    regmap_update_bits(it.regmap, IT5205_REG_MUXCR,
    IT5205_POLARITY_INVERTED, 0);
    break;
    case TYPEC_ORIENTATION_REVERSE:
    regmap_update_bits(it.regmap, IT5205_REG_MUXCR,
    IT5205_POLARITY_INVERTED, IT5205_POLARITY_INVERTED);
    break;
    case TYPEC_ORIENTATION_NONE:
    fallthrough;
    default:
    regmap_write(it.regmap, IT5205_REG_MUXCR, 0);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it5205_mux_set(mux: *mut typec_mux_dev, state: *mut typec_mux_state) -> c_int {
    static int it5205_mux_set(struct typec_mux_dev *mux, struct typec_mux_state *state)
    {
    struct it5205 *it = typec_mux_get_drvdata(mux);
    u8 val;
    if (state.mode >= TYPEC_STATE_MODAL &&
    state.alt.svid != USB_TYPEC_DP_SID)
    return -EINVAL;
    switch (state.mode) {
    case TYPEC_STATE_USB:
    val = IT5205_USB;
    break;
    case TYPEC_DP_STATE_C:
    fallthrough;
    case TYPEC_DP_STATE_E:
    val = IT5205_DP;
    break;
    case TYPEC_DP_STATE_D:
    val = IT5205_DP_USB;
    break;
    case TYPEC_STATE_SAFE:
    fallthrough;
    default:
    val = 0;
    break;
    }
    return regmap_update_bits(it.regmap, IT5205_REG_MUXCR,
    IT5205_DP_USB_CTRL_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn it5205_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t it5205_irq_handler(int irq, void *data)
    {
    struct it5205 *it = data;
    int ret;
    u32 val;
    ret = regmap_read(it.regmap, IT5205_REG_ISR, &val);
    if (ret)
    return IRQ_NONE;
    if (val & IT5205_ISR_CSBU_OVP) {
    dev_warn(&it.client.dev, "Overvoltage detected!\n");
// Reset CSBU
    regmap_update_bits(it.regmap, IT5205_REG_CSBUSR,
    IT5205_CSBUSR_SWITCH, 0);
    regmap_update_bits(it.regmap, IT5205_REG_CSBUSR,
    IT5205_CSBUSR_SWITCH, IT5205_CSBUSR_SWITCH);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn it5205_enable_ovp(it: *mut it5205) {
    static void it5205_enable_ovp(struct it5205 *it)
    {
// Select Vref 3.3v
    regmap_update_bits(it.regmap, IT5205_REG_VSR,
    IT5205_VREF_SELECT_MASK, IT5205_VREF_SELECT_3_3V);
// Trigger OVP at 3.68V
    regmap_update_bits(it.regmap, IT5205_REG_CSBUOVPSR,
    IT5205_OVP_SELECT_MASK, IT5205_OVP_3_68V);
// Unmask OVP interrupt
    regmap_update_bits(it.regmap, IT5205_REG_ISR,
    IT5205_ISR_CSBU_MASK, 0);
// Enable CSBU Interrupt
    regmap_update_bits(it.regmap, IT5205_REG_CSBUSR,
    IT5205_CSBUSR_SWITCH, IT5205_CSBUSR_SWITCH);
    }
    static const struct regmap_config it5205_regmap = {
    .max_register = 0x2f,
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn it5205_probe(client: *mut i2c_client) -> c_int {
    static int it5205_probe(struct i2c_client *client)
    {
    let mut sw_desc: typec_switch_desc = { };
    let mut mux_desc: typec_mux_desc = { };
    struct device *dev = &client.dev;
    struct it5205 *it;
    u32 val, chipid = 0;
    int i, ret;
    it = devm_kzalloc(dev, sizeof(*it), GFP_KERNEL);
    if (!it)
    return -ENOMEM;
    ret = devm_regulator_get_enable(dev, "vcc");
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get regulator\n");
    it.client = client;
    it.regmap = devm_regmap_init_i2c(client, &it5205_regmap);
    if (IS_ERR(it.regmap))
    return dev_err_probe(dev, PTR_ERR(it.regmap),
    "Failed to init regmap\n");
// IT5205 needs a long time to power up after enabling regulator
    msleep(50);
// Unset poweroff bit
    ret = regmap_write(it.regmap, IT5205_REG_MUXPDR, 0);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to set power on\n");
// Read the 32 bits ChipID
    for (i = 3; i >= 0; i--) {
    ret = regmap_read(it.regmap, IT5205_REG_CHIP_ID(i), &val);
    if (ret)
    return ret;
    chipid |= val << (i * 8);
    }
    if (chipid != IT5205FN_CHIP_ID)
    return dev_err_probe(dev, -EINVAL,
    "Unknown ChipID 0x%x\n", chipid);
// Initialize as USB mode with default (non-inverted) polarity
    ret = regmap_write(it.regmap, IT5205_REG_MUXCR, IT5205_USB);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot set mode to USB\n");
    sw_desc.drvdata = it;
    sw_desc.fwnode = dev_fwnode(dev);
    sw_desc.set = it5205_switch_set;
    it.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(it.sw))
    return dev_err_probe(dev, PTR_ERR(it.sw),
    "failed to register typec switch\n");
    mux_desc.drvdata = it;
    mux_desc.fwnode = dev_fwnode(dev);
    mux_desc.set = it5205_mux_set;
    it.mux = typec_mux_register(dev, &mux_desc);
    if (IS_ERR(it.mux)) {
    typec_switch_unregister(it.sw);
    return dev_err_probe(dev, PTR_ERR(it.mux),
    "failed to register typec mux\n");
    }
    i2c_set_clientdata(client, it);
    if (of_property_read_bool(dev.of_node, "ite,ovp-enable") && client.irq) {
    it5205_enable_ovp(it);
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    it5205_irq_handler,
    IRQF_ONESHOT, dev_name(dev), it);
    if (ret) {
    typec_mux_unregister(it.mux);
    typec_switch_unregister(it.sw);
    return dev_err_probe(dev, ret, "Failed to request irq\n");
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it5205_remove(client: *mut i2c_client) {
    static void it5205_remove(struct i2c_client *client)
    {
    struct it5205 *it = i2c_get_clientdata(client);
    typec_mux_unregister(it.mux);
    typec_switch_unregister(it.sw);
    }
    static const struct i2c_device_id it5205_table[] = {
    { .name = "it5205" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, it5205_table);
    static const struct of_device_id it5205_of_table[] = {
    { .compatible = "ite,it5205" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, it5205_of_table);
    static struct i2c_driver it5205_driver = {
    .driver = {
    .name = "it5205",
    .of_match_table = it5205_of_table,
    },
    .probe = it5205_probe,
    .remove = it5205_remove,
    .id_table = it5205_table,
    };
    module_i2c_driver(it5205_driver);
    MODULE_AUTHOR("Tianping Fang <tianping.fang@mediatek.com>");
    MODULE_AUTHOR("AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>");
    MODULE_DESCRIPTION("ITE IT5205 alternate mode passive MUX driver");
    MODULE_LICENSE("GPL");
