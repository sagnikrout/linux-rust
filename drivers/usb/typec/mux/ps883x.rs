//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/mux/ps883x.c
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
// Parade ps883x usb retimer driver
//
// Copyright (C) 2024 Linaro Ltd.
//

pub const REG_USB_PORT_CONN_STATUS_0: c_uint = 0x00;

pub const REG_USB_PORT_CONN_STATUS_1: c_uint = 0x01;

pub const REG_USB_PORT_CONN_STATUS_2: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps883x_retimer {
    pub client: *mut i2c_client,
    pub reset_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub sw: *mut typec_switch_dev,
    pub retimer: *mut typec_retimer,
    pub xo_clk: *mut clk,
    pub vdd_supply: *mut regulator,
    pub vdd33_supply: *mut regulator,
    pub vdd33_cap_supply: *mut regulator,
    pub vddat_supply: *mut regulator,
    pub vddar_supply: *mut regulator,
    pub vddio_supply: *mut regulator,
    pub typec_switch: *mut typec_switch,
    pub typec_mux: *mut typec_mux,
    pub /: *mut *mut mutex lock; / protect non-concurrent retimer & switch,
    pub orientation: enum typec_orientation,
    pub in_reset: bool,
}

#[no_mangle]
unsafe extern "C" fn ps883x_enable_vregs(retimer: *mut ps883x_retimer) -> c_int {
    static int ps883x_enable_vregs(struct ps883x_retimer *retimer)
    {
    struct device *dev = &retimer.client.dev;
    int ret;
    ret = regulator_enable(retimer.vdd33_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD 3.3V regulator: %d\n", ret);
    return ret;
    }
    ret = regulator_enable(retimer.vdd33_cap_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD 3.3V CAP regulator: %d\n", ret);
    goto err_vdd33_disable;
    }
    usleep_range(4000, 10000);
    ret = regulator_enable(retimer.vdd_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD regulator: %d\n", ret);
    goto err_vdd33_cap_disable;
    }
    ret = regulator_enable(retimer.vddar_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD AR regulator: %d\n", ret);
    goto err_vdd_disable;
    }
    ret = regulator_enable(retimer.vddat_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD AT regulator: %d\n", ret);
    goto err_vddar_disable;
    }
    ret = regulator_enable(retimer.vddio_supply);
    if (ret) {
    dev_err(dev, "cannot enable VDD IO regulator: %d\n", ret);
    goto err_vddat_disable;
    }
    return 0;
    err_vddat_disable:
    regulator_disable(retimer.vddat_supply);
    err_vddar_disable:
    regulator_disable(retimer.vddar_supply);
    err_vdd_disable:
    regulator_disable(retimer.vdd_supply);
    err_vdd33_cap_disable:
    regulator_disable(retimer.vdd33_cap_supply);
    err_vdd33_disable:
    regulator_disable(retimer.vdd33_supply);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ps883x_disable_vregs(retimer: *mut ps883x_retimer) {
    static void ps883x_disable_vregs(struct ps883x_retimer *retimer)
    {
    regulator_disable(retimer.vddio_supply);
    regulator_disable(retimer.vddat_supply);
    regulator_disable(retimer.vddar_supply);
    regulator_disable(retimer.vdd_supply);
    regulator_disable(retimer.vdd33_cap_supply);
    regulator_disable(retimer.vdd33_supply);
    }
#[no_mangle]
unsafe extern "C" fn ps883x_reset(retimer: *mut ps883x_retimer) {
    static void ps883x_reset(struct ps883x_retimer *retimer)
    {
    if (retimer.in_reset)
    return;
    gpiod_set_value(retimer.reset_gpio, 1);
    ps883x_disable_vregs(retimer);
    retimer.in_reset = true;
    }
    static int ps883x_configure(struct ps883x_retimer *retimer, int cfg0,
    int cfg1, int cfg2, bool reset)
    {
    struct device *dev = &retimer.client.dev;
    int ret;
    if (reset) {
    ps883x_reset(retimer);
    return 0;
    } else if (retimer.in_reset) {
    ret = ps883x_enable_vregs(retimer);
    if (ret)
    return ret;
    gpiod_set_value(retimer.reset_gpio, 0);
// firmware initialization delay
    msleep(60);
    retimer.in_reset = false;
    }
    ret = regmap_write(retimer.regmap, REG_USB_PORT_CONN_STATUS_0, cfg0);
    if (ret) {
    dev_err(dev, "failed to write conn_status_0: %d\n", ret);
    return ret;
    }
    ret = regmap_write(retimer.regmap, REG_USB_PORT_CONN_STATUS_1, cfg1);
    if (ret) {
    dev_err(dev, "failed to write conn_status_1: %d\n", ret);
    return ret;
    }
    ret = regmap_write(retimer.regmap, REG_USB_PORT_CONN_STATUS_2, cfg2);
    if (ret) {
    dev_err(dev, "failed to write conn_status_2: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps883x_set(retimer: *mut ps883x_retimer, state: *mut typec_retimer_state) -> c_int {
    static int ps883x_set(struct ps883x_retimer *retimer, struct typec_retimer_state *state)
    {
    struct typec_thunderbolt_data *tb_data;
    const struct enter_usb_data *eudo_data;
    let mut cfg0: c_int = CONN_STATUS_0_CONNECTION_PRESENT;
    let mut cfg1: c_int = 0x00;
    let mut cfg2: c_int = 0x00;
    let mut reset: bool = false;
    if (retimer.orientation == TYPEC_ORIENTATION_REVERSE)
    cfg0 |= CONN_STATUS_0_ORIENTATION_REVERSED;
    if (state.alt) {
    switch (state.alt.svid) {
    case USB_TYPEC_DP_SID:
    cfg1 |= CONN_STATUS_1_DP_CONNECTED |
    CONN_STATUS_1_DP_HPD_LEVEL;
    switch (state.mode)  {
    case TYPEC_DP_STATE_D:
    cfg0 |= CONN_STATUS_0_USB_3_1_CONNECTED;
    fallthrough;
    case TYPEC_DP_STATE_C:
    cfg1 |= CONN_STATUS_1_DP_SINK_REQUESTED |
    CONN_STATUS_1_DP_PIN_ASSIGNMENT_C_D;
    break;
    default: /* MODE_E */
    break;
    }
    break;
    case USB_TYPEC_TBT_SID:
    tb_data = state.data;
// Unconditional
    cfg2 |= CONN_STATUS_2_TBT_CONNECTED;
    if (tb_data.cable_mode & TBT_CABLE_ACTIVE_PASSIVE)
    cfg0 |= CONN_STATUS_0_ACTIVE_CABLE;
    if (tb_data.enter_vdo & TBT_ENTER_MODE_UNI_DIR_LSRX)
    cfg2 |= CONN_STATUS_2_TBT_UNIDIR_LSRX_ACT_LT;
    break;
    default:
    dev_err(&retimer.client.dev, "Got unsupported SID: 0x%x\n",
    state.alt.svid);
    return -EOPNOTSUPP;
    }
    } else {
    switch (state.mode) {
// SAFE can be transient or point to an actual disconnect
    case TYPEC_STATE_SAFE:
    reset = retimer.orientation == TYPEC_ORIENTATION_NONE;
    break;
// USB2 pins don't even go through this chip
    case TYPEC_MODE_USB2:
    reset = true;
    break;
    case TYPEC_STATE_USB:
    case TYPEC_MODE_USB3:
    cfg0 |= CONN_STATUS_0_USB_3_1_CONNECTED;
    break;
    case TYPEC_MODE_USB4:
    eudo_data = state.data;
    cfg2 |= CONN_STATUS_2_USB4_CONNECTED;
    if (FIELD_GET(EUDO_CABLE_TYPE_MASK, eudo_data.eudo) != EUDO_CABLE_TYPE_PASSIVE)
    cfg0 |= CONN_STATUS_0_ACTIVE_CABLE;
    break;
    default:
    dev_err(&retimer.client.dev, "Got unsupported mode: %lu\n",
    state.mode);
    return -EOPNOTSUPP;
    }
    }
    return ps883x_configure(retimer, cfg0, cfg1, cfg2, reset);
    }
    static int ps883x_sw_set(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
    struct ps883x_retimer *retimer = typec_switch_get_drvdata(sw);
    let mut ret: c_int = 0;
    ret = typec_switch_set(retimer.typec_switch, orientation);
    if (ret)
    return ret;
    guard(mutex)(&retimer.lock);
    if (retimer.orientation != orientation) {
    retimer.orientation = orientation;
//
// Orientation notifications usually come prior to mode switch
// events. If the retimer is already in reset, we still want to
// cache the new orientation value for the subsequent ps883x_set().
//
    if (retimer.in_reset)
    return 0;
    ret = regmap_assign_bits(retimer.regmap, REG_USB_PORT_CONN_STATUS_0,
    CONN_STATUS_0_ORIENTATION_REVERSED,
    orientation == TYPEC_ORIENTATION_REVERSE);
    if (ret)
    dev_err(&retimer.client.dev, "failed to set orientation: %d\n", ret);
    }
    return ret;
    }
    static int ps883x_retimer_set(struct typec_retimer *rtmr,
    struct typec_retimer_state *state)
    {
    struct ps883x_retimer *retimer = typec_retimer_get_drvdata(rtmr);
    struct typec_mux_state mux_state;
    let mut ret: c_int = 0;
    mutex_lock(&retimer.lock);
    ret = ps883x_set(retimer, state);
    mutex_unlock(&retimer.lock);
    if (ret)
    return ret;
    mux_state.alt = state.alt;
    mux_state.data = state.data;
    mux_state.mode = state.mode;
    return typec_mux_set(retimer.typec_mux, &mux_state);
    }
#[no_mangle]
unsafe extern "C" fn ps883x_get_vregs(retimer: *mut ps883x_retimer) -> c_int {
    static int ps883x_get_vregs(struct ps883x_retimer *retimer)
    {
    struct device *dev = &retimer.client.dev;
    retimer.vdd_supply = devm_regulator_get(dev, "vdd");
    if (IS_ERR(retimer.vdd_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vdd_supply),
    "failed to get VDD\n");
    retimer.vdd33_supply = devm_regulator_get(dev, "vdd33");
    if (IS_ERR(retimer.vdd33_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vdd33_supply),
    "failed to get VDD 3.3V\n");
    retimer.vdd33_cap_supply = devm_regulator_get(dev, "vdd33-cap");
    if (IS_ERR(retimer.vdd33_cap_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vdd33_cap_supply),
    "failed to get VDD CAP 3.3V\n");
    retimer.vddat_supply = devm_regulator_get(dev, "vddat");
    if (IS_ERR(retimer.vddat_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vddat_supply),
    "failed to get VDD AT\n");
    retimer.vddar_supply = devm_regulator_get(dev, "vddar");
    if (IS_ERR(retimer.vddar_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vddar_supply),
    "failed to get VDD AR\n");
    retimer.vddio_supply = devm_regulator_get(dev, "vddio");
    if (IS_ERR(retimer.vddio_supply))
    return dev_err_probe(dev, PTR_ERR(retimer.vddio_supply),
    "failed to get VDD IO\n");
    return 0;
    }
    static const struct regmap_config ps883x_retimer_regmap = {
    .max_register = 0x1f,
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn ps883x_retimer_probe(client: *mut i2c_client) -> c_int {
    static int ps883x_retimer_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    let mut sw_desc: typec_switch_desc = { };
    let mut rtmr_desc: typec_retimer_desc = { };
    struct ps883x_retimer *retimer;
    unsigned int val;
    int ret;
    retimer = devm_kzalloc(dev, sizeof(*retimer), GFP_KERNEL);
    if (!retimer)
    return -ENOMEM;
    retimer.client = client;
    mutex_init(&retimer.lock);
    retimer.regmap = devm_regmap_init_i2c(client, &ps883x_retimer_regmap);
    if (IS_ERR(retimer.regmap))
    return dev_err_probe(dev, PTR_ERR(retimer.regmap),
    "failed to allocate register map\n");
    ret = ps883x_get_vregs(retimer);
    if (ret)
    return ret;
    retimer.xo_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(retimer.xo_clk))
    return dev_err_probe(dev, PTR_ERR(retimer.xo_clk),
    "failed to get xo clock\n");
    retimer.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_ASIS);
    if (IS_ERR(retimer.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(retimer.reset_gpio),
    "failed to get reset gpio\n");
    retimer.typec_switch = typec_switch_get(dev);
    if (IS_ERR(retimer.typec_switch))
    return dev_err_probe(dev, PTR_ERR(retimer.typec_switch),
    "failed to acquire orientation-switch\n");
    retimer.typec_mux = typec_mux_get(dev);
    if (IS_ERR(retimer.typec_mux)) {
    ret = dev_err_probe(dev, PTR_ERR(retimer.typec_mux),
    "failed to acquire mode-mux\n");
    goto err_switch_put;
    }
    ret = drm_aux_bridge_register(dev);
    if (ret)
    goto err_mux_put;
    ret = ps883x_enable_vregs(retimer);
    if (ret)
    goto err_mux_put;
    ret = clk_prepare_enable(retimer.xo_clk);
    if (ret) {
    dev_err(dev, "failed to enable XO: %d\n", ret);
    goto err_vregs_disable;
    }
// skip resetting if already configured
    if (regmap_test_bits(retimer.regmap, REG_USB_PORT_CONN_STATUS_0,
    CONN_STATUS_0_CONNECTION_PRESENT) == 1) {
    gpiod_direction_output(retimer.reset_gpio, 0);
    } else {
    gpiod_direction_output(retimer.reset_gpio, 1);
// VDD IO supply enable to reset release delay
    usleep_range(4000, 14000);
    gpiod_set_value(retimer.reset_gpio, 0);
// firmware initialization delay
    msleep(60);
// make sure device is accessible
    ret = regmap_read(retimer.regmap, REG_USB_PORT_CONN_STATUS_0,
    &val);
    if (ret) {
    dev_err(dev, "failed to read conn_status_0: %d\n", ret);
    if (ret == -ENXIO)
    ret = -EIO;
    goto err_clk_disable;
    }
    }
// Keep the retimer in reset until a Type-C notification comes
    ps883x_reset(retimer);
    sw_desc.drvdata = retimer;
    sw_desc.fwnode = dev_fwnode(dev);
    sw_desc.set = ps883x_sw_set;
    retimer.sw = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(retimer.sw)) {
    ret = PTR_ERR(retimer.sw);
    dev_err(dev, "failed to register typec switch: %d\n", ret);
    goto err_clk_disable;
    }
    rtmr_desc.drvdata = retimer;
    rtmr_desc.fwnode = dev_fwnode(dev);
    rtmr_desc.set = ps883x_retimer_set;
    retimer.retimer = typec_retimer_register(dev, &rtmr_desc);
    if (IS_ERR(retimer.retimer)) {
    ret = PTR_ERR(retimer.retimer);
    dev_err(dev, "failed to register typec retimer: %d\n", ret);
    goto err_switch_unregister;
    }
    i2c_set_clientdata(client, retimer);
    return 0;
    err_switch_unregister:
    typec_switch_unregister(retimer.sw);
    err_clk_disable:
    clk_disable_unprepare(retimer.xo_clk);
    err_vregs_disable:
    gpiod_set_value(retimer.reset_gpio, 1);
    ps883x_disable_vregs(retimer);
    err_mux_put:
    typec_mux_put(retimer.typec_mux);
    err_switch_put:
    typec_switch_put(retimer.typec_switch);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ps883x_retimer_remove(client: *mut i2c_client) {
    static void ps883x_retimer_remove(struct i2c_client *client)
    {
    struct ps883x_retimer *retimer = i2c_get_clientdata(client);
    typec_retimer_unregister(retimer.retimer);
    typec_switch_unregister(retimer.sw);
    gpiod_set_value(retimer.reset_gpio, 1);
    clk_disable_unprepare(retimer.xo_clk);
    ps883x_disable_vregs(retimer);
    typec_mux_put(retimer.typec_mux);
    typec_switch_put(retimer.typec_switch);
    }
    static const struct of_device_id ps883x_retimer_of_table[] = {
    { .compatible = "parade,ps8830" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ps883x_retimer_of_table);
    static struct i2c_driver ps883x_retimer_driver = {
    .driver = {
    .name = "ps883x_retimer",
    .of_match_table = ps883x_retimer_of_table,
    },
    .probe		= ps883x_retimer_probe,
    .remove		= ps883x_retimer_remove,
    };
    module_i2c_driver(ps883x_retimer_driver);
    MODULE_DESCRIPTION("Parade ps883x Type-C Retimer driver");
    MODULE_LICENSE("GPL");
