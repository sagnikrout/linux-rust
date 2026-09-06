//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/ptn36502.c
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
// NXP PTN36502 Type-C driver
//
// Copyright (C) 2023 Luca Weiss <luca.weiss@fairphone.com>
//
// Based on NB7VPQ904M driver:
// Copyright (C) 2023 Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
//

pub const PTN36502_CHIP_ID_REG: c_uint = 0x00;
pub const PTN36502_CHIP_ID: c_uint = 0x02;
pub const PTN36502_CHIP_REVISION_REG: c_uint = 0x01;

pub const PTN36502_DP_LINK_CTRL_REG: c_uint = 0x06;

// Registers for lane 0 (0x07) to lane 3 (0x0a) have the same layout

pub const PTN36502_MODE_CTRL1_REG: c_uint = 0x0b;

pub const PTN36502_DEVICE_CTRL_REG: c_uint = 0x0d;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptn36502 {
    pub client: *mut i2c_client,
    pub vdd18_supply: *mut regulator,
    pub regmap: *mut regmap,
    pub sw: *mut typec_switch_dev,
    pub retimer: *mut typec_retimer,
    pub typec_switch: *mut typec_switch,
    pub typec_mux: *mut typec_mux,
    pub /: *mut *mut mutex lock; / protect non-concurrent retimer & switch,
    pub orientation: enum typec_orientation,
    pub mode: c_ulong,
    pub svid: c_uint,
}

#[no_mangle]
unsafe extern "C" fn ptn36502_set(ptn: *mut ptn36502) -> c_int {
    static int ptn36502_set(struct ptn36502 *ptn)
    {
    let mut reverse: bool = (ptn.orientation == TYPEC_ORIENTATION_REVERSE);
    let mut ctrl1_val: c_uint = 0;
    let mut lane_ctrl_val: c_uint = 0;
    let mut link_ctrl_val: c_uint = 0;
    switch (ptn.mode) {
    case TYPEC_STATE_SAFE:
// Deep power saving state
    regmap_write(ptn.regmap, PTN36502_MODE_CTRL1_REG,
    FIELD_PREP(PTN36502_MODE_CTRL1_MODE_MASK,
    PTN36502_MODE_CTRL1_MODE_OFF));
    return 0;
    case TYPEC_STATE_USB:
//
// Normal Orientation (CC1)
// A -> USB RX
// B -> USB TX
// C -> X
// D -> X
// Flipped Orientation (CC2)
// A -> X
// B -> X
// C -> USB TX
// D -> USB RX
//
// USB 3.1 Gen 1 only
    ctrl1_val = FIELD_PREP(PTN36502_MODE_CTRL1_MODE_MASK,
    PTN36502_MODE_CTRL1_MODE_USB_ONLY);
    if (reverse)
    ctrl1_val |= FIELD_PREP(PTN36502_MODE_CTRL1_PLUG_ORIENT_MASK,
    PTN36502_MODE_CTRL1_PLUG_ORIENT_REVERSE);
    regmap_write(ptn.regmap, PTN36502_MODE_CTRL1_REG, ctrl1_val);
    return 0;
    default:
    if (ptn.svid != USB_TYPEC_DP_SID)
    return -EINVAL;
    break;
    }
// DP Altmode Setup
    switch (ptn.mode) {
    case TYPEC_DP_STATE_C:
    case TYPEC_DP_STATE_E:
//
// Normal Orientation (CC1)
// A -> DP3
// B -> DP2
// C -> DP1
// D -> DP0
// Flipped Orientation (CC2)
// A -> DP0
// B -> DP1
// C -> DP2
// D -> DP3
//
// 4-lane DP
    ctrl1_val |= FIELD_PREP(PTN36502_MODE_CTRL1_MODE_MASK,
    PTN36502_MODE_CTRL1_MODE_DP);
    link_ctrl_val |= FIELD_PREP(PTN36502_DP_LINK_CTRL_LANES_MASK,
    PTN36502_DP_LINK_CTRL_LANES_4);
    break;
    case TYPEC_DP_STATE_D:
    case TYPEC_DP_STATE_F: /* State F is deprecated */
//
// Normal Orientation (CC1)
// A -> USB RX
// B -> USB TX
// C -> DP1
// D -> DP0
// Flipped Orientation (CC2)
// A -> DP0
// B -> DP1
// C -> USB TX
// D -> USB RX
//
// USB 3.1 Gen 1 and 2-lane DP
    ctrl1_val |= FIELD_PREP(PTN36502_MODE_CTRL1_MODE_MASK,
    PTN36502_MODE_CTRL1_MODE_USB_DP);
    link_ctrl_val |= FIELD_PREP(PTN36502_DP_LINK_CTRL_LANES_MASK,
    PTN36502_DP_LINK_CTRL_LANES_2);
    break;
    default:
    return -EOPNOTSUPP;
    }
// Enable AUX monitoring
    regmap_write(ptn.regmap, PTN36502_DEVICE_CTRL_REG,
    FIELD_PREP(PTN36502_DEVICE_CTRL_AUX_MONITORING_MASK,
    PTN36502_DEVICE_CTRL_AUX_MONITORING_EN));
// Enable AUX switch path
    ctrl1_val |= FIELD_PREP(PTN36502_MODE_CTRL1_AUX_CROSSBAR_MASK,
    PTN36502_MODE_CTRL1_AUX_CROSSBAR_SW_ON);
    if (reverse)
    ctrl1_val |= FIELD_PREP(PTN36502_MODE_CTRL1_PLUG_ORIENT_MASK,
    PTN36502_MODE_CTRL1_PLUG_ORIENT_REVERSE);
    regmap_write(ptn.regmap, PTN36502_MODE_CTRL1_REG, ctrl1_val);
// DP Link rate: 5.4 Gbps (HBR2)
    link_ctrl_val |= FIELD_PREP(PTN36502_DP_LINK_CTRL_LINK_RATE_MASK,
    PTN36502_DP_LINK_CTRL_LINK_RATE_5_4GBPS);
    regmap_write(ptn.regmap, PTN36502_DP_LINK_CTRL_REG, link_ctrl_val);
//
// For all lanes:
// - Rx equivalization gain: 3 dB
// - TX output swing control: 800 mVppd
// - Pre-emphasis control: 3.5 dB
//
    lane_ctrl_val = FIELD_PREP(PTN36502_DP_LANE_CTRL_RX_GAIN_MASK,
    PTN36502_DP_LANE_CTRL_RX_GAIN_3DB) |
    FIELD_PREP(PTN36502_DP_LANE_CTRL_TX_SWING_MASK,
    PTN36502_DP_LANE_CTRL_TX_SWING_800MVPPD) |
    FIELD_PREP(PTN36502_DP_LANE_CTRL_PRE_EMPHASIS_MASK,
    PTN36502_DP_LANE_CTRL_PRE_EMPHASIS_3_5DB);
    regmap_write(ptn.regmap, PTN36502_DP_LANE_CTRL_REG(0), lane_ctrl_val);
    regmap_write(ptn.regmap, PTN36502_DP_LANE_CTRL_REG(1), lane_ctrl_val);
    regmap_write(ptn.regmap, PTN36502_DP_LANE_CTRL_REG(2), lane_ctrl_val);
    regmap_write(ptn.regmap, PTN36502_DP_LANE_CTRL_REG(3), lane_ctrl_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ptn36502_sw_set(sw: *mut typec_switch_dev, orientation: enum typec_orientation) -> c_int {
    static int ptn36502_sw_set(struct typec_switch_dev *sw, enum typec_orientation orientation)
    {
    struct ptn36502 *ptn = typec_switch_get_drvdata(sw);
    int ret;
    ret = typec_switch_set(ptn.typec_switch, orientation);
    if (ret)
    return ret;
    mutex_lock(&ptn.lock);
    if (ptn.orientation != orientation) {
    ptn.orientation = orientation;
    ret = ptn36502_set(ptn);
    }
    mutex_unlock(&ptn.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ptn36502_retimer_set(retimer: *mut typec_retimer, state: *mut typec_retimer_state) -> c_int {
    static int ptn36502_retimer_set(struct typec_retimer *retimer, struct typec_retimer_state *state)
    {
    struct ptn36502 *ptn = typec_retimer_get_drvdata(retimer);
    struct typec_mux_state mux_state;
    let mut ret: c_int = 0;
    mutex_lock(&ptn.lock);
    if (ptn.mode != state.mode) {
    ptn.mode = state.mode;
    if (state.alt)
    ptn.svid = state.alt.svid;
    else
    ptn.svid = 0; // No SVID
    ret = ptn36502_set(ptn);
    }
    mutex_unlock(&ptn.lock);
    if (ret)
    return ret;
    mux_state.alt = state.alt;
    mux_state.data = state.data;
    mux_state.mode = state.mode;
    return typec_mux_set(ptn.typec_mux, &mux_state);
    }
#[no_mangle]
unsafe extern "C" fn ptn36502_detect(ptn: *mut ptn36502) -> c_int {
    static int ptn36502_detect(struct ptn36502 *ptn)
    {
    struct device *dev = &ptn.client.dev;
    unsigned int reg_val;
    int ret;
    ret = regmap_read(ptn.regmap, PTN36502_CHIP_ID_REG,
    &reg_val);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read chip ID\n");
    if (reg_val != PTN36502_CHIP_ID)
    return dev_err_probe(dev, -ENODEV, "Unexpected chip ID: %x\n", reg_val);
    ret = regmap_read(ptn.regmap, PTN36502_CHIP_REVISION_REG,
    &reg_val);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read chip revision\n");
    dev_dbg(dev, "Chip revision: base layer version %lx, metal layer version %lx\n",
    FIELD_GET(PTN36502_CHIP_REVISION_BASE_MASK, reg_val),
    FIELD_GET(PTN36502_CHIP_REVISION_METAL_MASK, reg_val));
    return 0;
    }
    static const struct regmap_config ptn36502_regmap = {
    .max_register = 0x0d,
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn ptn36502_probe(client: *mut i2c_client) -> c_int {
    static int ptn36502_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    let mut sw_desc: typec_switch_desc = { };
    let mut retimer_desc: typec_retimer_desc = { };
    struct ptn36502 *ptn;
    int ret;
    ptn = devm_kzalloc(dev, sizeof(*ptn), GFP_KERNEL);
    if (!ptn)
    return -ENOMEM;
    ptn.client = client;
    ptn.regmap = devm_regmap_init_i2c(client, &ptn36502_regmap);
    if (IS_ERR(ptn.regmap)) {
    dev_err(&client.dev, "Failed to allocate register map\n");
    return PTR_ERR(ptn.regmap);
    }
    ptn.mode = TYPEC_STATE_SAFE;
    ptn.orientation = TYPEC_ORIENTATION_NONE;
    mutex_init(&ptn.lock);
    ptn.vdd18_supply = devm_regulator_get_optional(dev, "vdd18");
    if (IS_ERR(ptn.vdd18_supply))
    return PTR_ERR(ptn.vdd18_supply);
    ptn.typec_switch = fwnode_typec_switch_get(dev.fwnode);
    if (IS_ERR(ptn.typec_switch))
    return dev_err_probe(dev, PTR_ERR(ptn.typec_switch),
    "Failed to acquire orientation-switch\n");
    ptn.typec_mux = fwnode_typec_mux_get(dev.fwnode);
    if (IS_ERR(ptn.typec_mux)) {
    ret = dev_err_probe(dev, PTR_ERR(ptn.typec_mux),
    "Failed to acquire mode-switch\n");
    goto err_switch_put;
    }
    ret = regulator_enable(ptn.vdd18_supply);
    if (ret) {
    ret = dev_err_probe(dev, ret, "Failed to enable vdd18\n");
    goto err_mux_put;
    }
    ret = ptn36502_detect(ptn);
    if (ret)
    goto err_disable_regulator;
    ret = drm_aux_bridge_register(dev);
    if (ret)
    goto err_disable_regulator;
    sw_desc.drvdata = ptn;
    sw_desc.fwnode = dev.fwnode;
    sw_desc.set = ptn36502_sw_set;
    ptn.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(ptn.sw)) {
    ret = dev_err_probe(dev, PTR_ERR(ptn.sw),
    "Failed to register typec switch\n");
    goto err_disable_regulator;
    }
    retimer_desc.drvdata = ptn;
    retimer_desc.fwnode = dev.fwnode;
    retimer_desc.set = ptn36502_retimer_set;
    ptn.retimer = typec_retimer_register(dev, &retimer_desc);
    if (IS_ERR(ptn.retimer)) {
    ret = dev_err_probe(dev, PTR_ERR(ptn.retimer),
    "Failed to register typec retimer\n");
    goto err_switch_unregister;
    }
    return 0;
    err_switch_unregister:
    typec_switch_unregister(ptn.sw);
    err_disable_regulator:
    regulator_disable(ptn.vdd18_supply);
    err_mux_put:
    typec_mux_put(ptn.typec_mux);
    err_switch_put:
    typec_switch_put(ptn.typec_switch);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ptn36502_remove(client: *mut i2c_client) {
    static void ptn36502_remove(struct i2c_client *client)
    {
    struct ptn36502 *ptn = i2c_get_clientdata(client);
    typec_retimer_unregister(ptn.retimer);
    typec_switch_unregister(ptn.sw);
    regulator_disable(ptn.vdd18_supply);
    typec_mux_put(ptn.typec_mux);
    typec_switch_put(ptn.typec_switch);
    }
    static const struct i2c_device_id ptn36502_table[] = {
    { .name = "ptn36502" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ptn36502_table);
    static const struct of_device_id ptn36502_of_table[] = {
    { .compatible = "nxp,ptn36502" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ptn36502_of_table);
    static struct i2c_driver ptn36502_driver = {
    .driver = {
    .name = "ptn36502",
    .of_match_table = ptn36502_of_table,
    },
    .probe		= ptn36502_probe,
    .remove		= ptn36502_remove,
    .id_table	= ptn36502_table,
    };
    module_i2c_driver(ptn36502_driver);
    MODULE_AUTHOR("Luca Weiss <luca.weiss@fairphone.com>");
    MODULE_DESCRIPTION("NXP PTN36502 Type-C driver");
    MODULE_LICENSE("GPL");
