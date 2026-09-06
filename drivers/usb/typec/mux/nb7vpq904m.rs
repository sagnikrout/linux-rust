//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/nb7vpq904m.c
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
// OnSemi NB7VPQ904M Type-C driver
//
// Copyright (C) 2023 Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
//

pub const NB7_CHNA: c_int = 0;
pub const NB7_CHNB: c_int = 1;
pub const NB7_CHNC: c_int = 2;
pub const NB7_CHND: c_int = 3;

pub const GEN_DEV_SET_REG: c_uint = 0x00;

pub const GEN_DEV_SET_OP_MODE_DP_CC2: c_int = 0;
pub const GEN_DEV_SET_OP_MODE_DP_CC1: c_int = 1;
pub const GEN_DEV_SET_OP_MODE_DP_4LANE: c_int = 2;
pub const GEN_DEV_SET_OP_MODE_USB: c_int = 5;
pub const EQ_SETTING_REG_BASE: c_uint = 0x01;

pub const OUTPUT_COMPRESSION_AND_POL_REG_BASE: c_uint = 0x02;

pub const FLAT_GAIN_REG_BASE: c_uint = 0x18;

pub const LOSS_MATCH_REG_BASE: c_uint = 0x19;

pub const AUX_CC_REG: c_uint = 0x09;
pub const CHIP_VERSION_REG: c_uint = 0x17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nb7vpq904m {
    pub client: *mut i2c_client,
    pub enable_gpio: *mut gpio_desc,
    pub vcc_supply: *mut regulator,
    pub regmap: *mut regmap,
    pub sw: *mut typec_switch_dev,
    pub retimer: *mut typec_retimer,
    pub swap_data_lanes: bool,
    pub typec_switch: *mut typec_switch,
    pub typec_mux: *mut typec_mux,
    pub /: *mut *mut mutex lock; / protect non-concurrent retimer & switch,
    pub orientation: enum typec_orientation,
    pub mode: c_ulong,
    pub svid: c_uint,
}

#[no_mangle]
unsafe extern "C" fn nb7vpq904m_set_channel(nb7: *mut nb7vpq904m, channel: c_uint, dp: bool) {
    static void nb7vpq904m_set_channel(struct nb7vpq904m *nb7, unsigned int channel, bool dp)
    {
    u8 eq, out_comp, flat_gain, loss_match;
    if (dp) {
    eq = NB7_IS_CHAN_AD(channel) ? 0x6 : 0x4;
    out_comp = 0x3;
    flat_gain = NB7_IS_CHAN_AD(channel) ? 0x2 : 0x1;
    loss_match = 0x3;
    } else {
    eq = 0x4;
    out_comp = 0x3;
    flat_gain = NB7_IS_CHAN_AD(channel) ? 0x3 : 0x1;
    loss_match = NB7_IS_CHAN_AD(channel) ? 0x1 : 0x3;
    }
    regmap_update_bits(nb7.regmap, EQ_SETTING_REG(channel),
    EQ_SETTING_MASK, FIELD_PREP(EQ_SETTING_MASK, eq));
    regmap_update_bits(nb7.regmap, OUTPUT_COMPRESSION_AND_POL_REG(channel),
    OUTPUT_COMPRESSION_MASK, FIELD_PREP(OUTPUT_COMPRESSION_MASK, out_comp));
    regmap_update_bits(nb7.regmap, FLAT_GAIN_REG(channel),
    FLAT_GAIN_MASK, FIELD_PREP(FLAT_GAIN_MASK, flat_gain));
    regmap_update_bits(nb7.regmap, LOSS_MATCH_REG(channel),
    LOSS_MATCH_MASK, FIELD_PREP(LOSS_MATCH_MASK, loss_match));
    }
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_set(nb7: *mut nb7vpq904m) -> c_int {
    static int nb7vpq904m_set(struct nb7vpq904m *nb7)
    {
    let mut reverse: bool = (nb7.orientation == TYPEC_ORIENTATION_REVERSE);
    switch (nb7.mode) {
    case TYPEC_STATE_SAFE:
    regmap_write(nb7.regmap, GEN_DEV_SET_REG,
    GEN_DEV_SET_CHIP_EN |
    GEN_DEV_SET_CHNA_EN |
    GEN_DEV_SET_CHNB_EN |
    GEN_DEV_SET_CHNC_EN |
    GEN_DEV_SET_CHND_EN |
    FIELD_PREP(GEN_DEV_SET_OP_MODE_MASK,
    GEN_DEV_SET_OP_MODE_USB));
    nb7vpq904m_set_channel(nb7, NB7_CHNA, false);
    nb7vpq904m_set_channel(nb7, NB7_CHNB, false);
    nb7vpq904m_set_channel(nb7, NB7_CHNC, false);
    nb7vpq904m_set_channel(nb7, NB7_CHND, false);
    regmap_write(nb7.regmap, AUX_CC_REG, 0x2);
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
// Reversed if data lanes are swapped
//
    if (reverse ^ nb7.swap_data_lanes) {
    regmap_write(nb7.regmap, GEN_DEV_SET_REG,
    GEN_DEV_SET_CHIP_EN |
    GEN_DEV_SET_CHNA_EN |
    GEN_DEV_SET_CHNB_EN |
    FIELD_PREP(GEN_DEV_SET_OP_MODE_MASK,
    GEN_DEV_SET_OP_MODE_USB));
    nb7vpq904m_set_channel(nb7, NB7_CHNA, false);
    nb7vpq904m_set_channel(nb7, NB7_CHNB, false);
    } else {
    regmap_write(nb7.regmap, GEN_DEV_SET_REG,
    GEN_DEV_SET_CHIP_EN |
    GEN_DEV_SET_CHNC_EN |
    GEN_DEV_SET_CHND_EN |
    FIELD_PREP(GEN_DEV_SET_OP_MODE_MASK,
    GEN_DEV_SET_OP_MODE_USB));
    nb7vpq904m_set_channel(nb7, NB7_CHNC, false);
    nb7vpq904m_set_channel(nb7, NB7_CHND, false);
    }
    regmap_write(nb7.regmap, AUX_CC_REG, 0x2);
    return 0;
    default:
    if (nb7.svid != USB_TYPEC_DP_SID)
    return -EINVAL;
    break;
    }
// DP Altmode Setup
    regmap_write(nb7.regmap, AUX_CC_REG, reverse ? 0x1 : 0x0);
    switch (nb7.mode) {
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
    regmap_write(nb7.regmap, GEN_DEV_SET_REG,
    GEN_DEV_SET_CHIP_EN |
    GEN_DEV_SET_CHNA_EN |
    GEN_DEV_SET_CHNB_EN |
    GEN_DEV_SET_CHNC_EN |
    GEN_DEV_SET_CHND_EN |
    FIELD_PREP(GEN_DEV_SET_OP_MODE_MASK,
    GEN_DEV_SET_OP_MODE_DP_4LANE));
    nb7vpq904m_set_channel(nb7, NB7_CHNA, true);
    nb7vpq904m_set_channel(nb7, NB7_CHNB, true);
    nb7vpq904m_set_channel(nb7, NB7_CHNC, true);
    nb7vpq904m_set_channel(nb7, NB7_CHND, true);
    break;
    case TYPEC_DP_STATE_D:
    case TYPEC_DP_STATE_F:
    regmap_write(nb7.regmap, GEN_DEV_SET_REG,
    GEN_DEV_SET_CHIP_EN |
    GEN_DEV_SET_CHNA_EN |
    GEN_DEV_SET_CHNB_EN |
    GEN_DEV_SET_CHNC_EN |
    GEN_DEV_SET_CHND_EN |
    FIELD_PREP(GEN_DEV_SET_OP_MODE_MASK,
    reverse ^ nb7.swap_data_lanes ?
    GEN_DEV_SET_OP_MODE_DP_CC2
    : GEN_DEV_SET_OP_MODE_DP_CC1));
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
// Reversed if data lanes are swapped
//
    if (nb7.swap_data_lanes) {
    nb7vpq904m_set_channel(nb7, NB7_CHNA, !reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHNB, !reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHNC, reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHND, reverse);
    } else {
    nb7vpq904m_set_channel(nb7, NB7_CHNA, reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHNB, reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHNC, !reverse);
    nb7vpq904m_set_channel(nb7, NB7_CHND, !reverse);
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_sw_set(sw: *mut typec_switch_dev, orientation: enum typec_orientation) -> c_int {
    static int nb7vpq904m_sw_set(struct typec_switch_dev *sw, enum typec_orientation orientation)
    {
    struct nb7vpq904m *nb7 = typec_switch_get_drvdata(sw);
    int ret;
    ret = typec_switch_set(nb7.typec_switch, orientation);
    if (ret)
    return ret;
    mutex_lock(&nb7.lock);
    if (nb7.orientation != orientation) {
    nb7.orientation = orientation;
    ret = nb7vpq904m_set(nb7);
    }
    mutex_unlock(&nb7.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_retimer_set(retimer: *mut typec_retimer, state: *mut typec_retimer_state) -> c_int {
    static int nb7vpq904m_retimer_set(struct typec_retimer *retimer, struct typec_retimer_state *state)
    {
    struct nb7vpq904m *nb7 = typec_retimer_get_drvdata(retimer);
    struct typec_mux_state mux_state;
    let mut ret: c_int = 0;
    mutex_lock(&nb7.lock);
    if (nb7.mode != state.mode) {
    nb7.mode = state.mode;
    if (state.alt)
    nb7.svid = state.alt.svid;
    else
    nb7.svid = 0; // No SVID
    ret = nb7vpq904m_set(nb7);
    }
    mutex_unlock(&nb7.lock);
    if (ret)
    return ret;
    mux_state.alt = state.alt;
    mux_state.data = state.data;
    mux_state.mode = state.mode;
    return typec_mux_set(nb7.typec_mux, &mux_state);
    }
    static const struct regmap_config nb7_regmap = {
    .max_register = 0x1f,
    .reg_bits = 8,
    .val_bits = 8,
    };
    enum {
    NORMAL_LANE_MAPPING,
    INVERT_LANE_MAPPING,
    };
pub const DATA_LANES_COUNT: c_int = 4;
    static const int supported_data_lane_mapping[][DATA_LANES_COUNT] = {
    [NORMAL_LANE_MAPPING] = { 0, 1, 2, 3 },
    [INVERT_LANE_MAPPING] = { 3, 2, 1, 0 },
    };
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_parse_data_lanes_mapping(nb7: *mut nb7vpq904m) -> c_int {
    static int nb7vpq904m_parse_data_lanes_mapping(struct nb7vpq904m *nb7)
    {
    struct device_node *ep;
    u32 data_lanes[4];
    int ret, i, j;
    ep = of_graph_get_endpoint_by_regs(nb7.client.dev.of_node, 1, 0);
    if (!ep)
    return 0;
    ret = of_property_count_u32_elems(ep, "data-lanes");
    if (ret == -EINVAL)
// Property isn't here, consider default mapping
    goto out_done;
    if (ret < 0)
    goto out_error;
    if (ret != DATA_LANES_COUNT) {
    dev_err(&nb7.client.dev, "expected 4 data lanes\n");
    ret = -EINVAL;
    goto out_error;
    }
    ret = of_property_read_u32_array(ep, "data-lanes", data_lanes, DATA_LANES_COUNT);
    if (ret)
    goto out_error;
    for (i = 0; i < ARRAY_SIZE(supported_data_lane_mapping); i++) {
    for (j = 0; j < DATA_LANES_COUNT; j++) {
    if (data_lanes[j] != supported_data_lane_mapping[i][j])
    break;
    }
    if (j == DATA_LANES_COUNT)
    break;
    }
    switch (i) {
    case NORMAL_LANE_MAPPING:
    break;
    case INVERT_LANE_MAPPING:
    nb7.swap_data_lanes = true;
    dev_info(&nb7.client.dev, "using inverted data lanes mapping\n");
    break;
    default:
    dev_err(&nb7.client.dev, "invalid data lanes mapping\n");
    ret = -EINVAL;
    goto out_error;
    }
    out_done:
    ret = 0;
    out_error:
    of_node_put(ep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_probe(client: *mut i2c_client) -> c_int {
    static int nb7vpq904m_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    let mut sw_desc: typec_switch_desc = { };
    let mut retimer_desc: typec_retimer_desc = { };
    struct nb7vpq904m *nb7;
    int ret;
    nb7 = devm_kzalloc(dev, sizeof(*nb7), GFP_KERNEL);
    if (!nb7)
    return -ENOMEM;
    nb7.client = client;
    nb7.regmap = devm_regmap_init_i2c(client, &nb7_regmap);
    if (IS_ERR(nb7.regmap)) {
    dev_err(&client.dev, "Failed to allocate register map\n");
    return PTR_ERR(nb7.regmap);
    }
    nb7.mode = TYPEC_STATE_SAFE;
    nb7.orientation = TYPEC_ORIENTATION_NONE;
    mutex_init(&nb7.lock);
    nb7.enable_gpio = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(nb7.enable_gpio))
    return dev_err_probe(dev, PTR_ERR(nb7.enable_gpio),
    "unable to acquire enable gpio\n");
    nb7.vcc_supply = devm_regulator_get_optional(dev, "vcc");
    if (IS_ERR(nb7.vcc_supply))
    return PTR_ERR(nb7.vcc_supply);
    nb7.typec_switch = fwnode_typec_switch_get(dev.fwnode);
    if (IS_ERR(nb7.typec_switch))
    return dev_err_probe(dev, PTR_ERR(nb7.typec_switch),
    "failed to acquire orientation-switch\n");
    nb7.typec_mux = fwnode_typec_mux_get(dev.fwnode);
    if (IS_ERR(nb7.typec_mux)) {
    ret = dev_err_probe(dev, PTR_ERR(nb7.typec_mux),
    "Failed to acquire mode-switch\n");
    goto err_switch_put;
    }
    ret = nb7vpq904m_parse_data_lanes_mapping(nb7);
    if (ret)
    goto err_mux_put;
    ret = regulator_enable(nb7.vcc_supply);
    if (ret)
    dev_warn(dev, "Failed to enable vcc: %d\n", ret);
    gpiod_set_value(nb7.enable_gpio, 1);
    ret = drm_aux_bridge_register(dev);
    if (ret)
    goto err_disable_gpio;
    sw_desc.drvdata = nb7;
    sw_desc.fwnode = dev.fwnode;
    sw_desc.set = nb7vpq904m_sw_set;
    nb7.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(nb7.sw)) {
    ret = dev_err_probe(dev, PTR_ERR(nb7.sw),
    "Error registering typec switch\n");
    goto err_disable_gpio;
    }
    retimer_desc.drvdata = nb7;
    retimer_desc.fwnode = dev.fwnode;
    retimer_desc.set = nb7vpq904m_retimer_set;
    nb7.retimer = typec_retimer_register(dev, &retimer_desc);
    if (IS_ERR(nb7.retimer)) {
    ret = dev_err_probe(dev, PTR_ERR(nb7.retimer),
    "Error registering typec retimer\n");
    goto err_switch_unregister;
    }
    return 0;
    err_switch_unregister:
    typec_switch_unregister(nb7.sw);
    err_disable_gpio:
    gpiod_set_value(nb7.enable_gpio, 0);
    regulator_disable(nb7.vcc_supply);
    err_mux_put:
    typec_mux_put(nb7.typec_mux);
    err_switch_put:
    typec_switch_put(nb7.typec_switch);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nb7vpq904m_remove(client: *mut i2c_client) {
    static void nb7vpq904m_remove(struct i2c_client *client)
    {
    struct nb7vpq904m *nb7 = i2c_get_clientdata(client);
    typec_retimer_unregister(nb7.retimer);
    typec_switch_unregister(nb7.sw);
    gpiod_set_value(nb7.enable_gpio, 0);
    regulator_disable(nb7.vcc_supply);
    typec_mux_put(nb7.typec_mux);
    typec_switch_put(nb7.typec_switch);
    }
    static const struct i2c_device_id nb7vpq904m_table[] = {
    { .name = "nb7vpq904m" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, nb7vpq904m_table);
    static const struct of_device_id nb7vpq904m_of_table[] = {
    { .compatible = "onnn,nb7vpq904m" },
    { }
    };
    MODULE_DEVICE_TABLE(of, nb7vpq904m_of_table);
    static struct i2c_driver nb7vpq904m_driver = {
    .driver = {
    .name = "nb7vpq904m",
    .of_match_table = nb7vpq904m_of_table,
    },
    .probe		= nb7vpq904m_probe,
    .remove		= nb7vpq904m_remove,
    .id_table	= nb7vpq904m_table,
    };
    module_i2c_driver(nb7vpq904m_driver);
    MODULE_AUTHOR("Dmitry Baryshkov <dmitry.baryshkov@linaro.org>");
    MODULE_DESCRIPTION("OnSemi NB7VPQ904M Type-C driver");
    MODULE_LICENSE("GPL");
