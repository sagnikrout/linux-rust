//! Automatically rewritten from C to Rust
//! Source: drivers/iio/amplifiers/adl8113.c
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
// ADL8113 Low Noise Amplifier with integrated bypass switches
//
// Copyright 2025 Analog Devices Inc.
//

    enum adl8113_signal_path {
    ADL8113_INTERNAL_AMP,
    ADL8113_INTERNAL_BYPASS,
    ADL8113_EXTERNAL_A,
    ADL8113_EXTERNAL_B,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adl8113_gain_config {
    pub path: enum adl8113_signal_path,
    pub gain_db: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adl8113_state {
    pub gpios: *mut gpio_descs,
    pub gain_configs: *mut adl8113_gain_config,
    pub num_gain_configs: c_uint,
    pub current_path: enum adl8113_signal_path,
}

    static const char * const adl8113_supply_names[] = {
    "vdd1",
    "vss2",
    "vdd2",
    };
    static int adl8113_set_path(struct adl8113_state *st,
    enum adl8113_signal_path path)
    {
    DECLARE_BITMAP(values, 2);
    int ret;
//
// Determine GPIO values based on signal path.
// Va: bit 0, Vb: bit 1.
//
    switch (path) {
    case ADL8113_INTERNAL_AMP:
    bitmap_write(values, 0x00, 0, 2);
    break;
    case ADL8113_INTERNAL_BYPASS:
    bitmap_write(values, 0x03, 0, 2);
    break;
    case ADL8113_EXTERNAL_A:
    bitmap_write(values, 0x02, 0, 2);
    break;
    case ADL8113_EXTERNAL_B:
    bitmap_write(values, 0x01, 0, 2);
    break;
    default:
    return -EINVAL;
    }
    ret = gpiod_set_array_value_cansleep(st.gpios.ndescs, st.gpios.desc,
    st.gpios.info, values);
    if (ret)
    return ret;
    st.current_path = path;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adl8113_find_gain_config(st: *mut adl8113_state, gain_db: c_int) -> c_int {
    static int adl8113_find_gain_config(struct adl8113_state *st, int gain_db)
    {
    unsigned int i;
    for (i = 0; i < st.num_gain_configs; i++) {
    if (st.gain_configs[i].gain_db == gain_db)
    return i;
    }
    return -EINVAL;
    }
    static const struct iio_chan_spec adl8113_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_HARDWAREGAIN),
    },
    };
    static int adl8113_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct adl8113_state *st = iio_priv(indio_dev);
    unsigned int i;
    switch (mask) {
    case IIO_CHAN_INFO_HARDWAREGAIN:
// Find current gain configuration
    for (i = 0; i < st.num_gain_configs; i++) {
    if (st.gain_configs[i].path == st.current_path) {
// val = st->gain_configs[i].gain_db;
// val2 = 0;
    return IIO_VAL_INT_PLUS_MICRO_DB;
    }
    }
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static int adl8113_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct adl8113_state *st = iio_priv(indio_dev);
    int config_idx;
    switch (mask) {
    case IIO_CHAN_INFO_HARDWAREGAIN:
    if (val2 != 0)
    return -EINVAL;
    config_idx = adl8113_find_gain_config(st, val);
    if (config_idx < 0)
    return config_idx;
    return adl8113_set_path(st, st.gain_configs[config_idx].path);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info adl8113_info = {
    .read_raw = adl8113_read_raw,
    .write_raw = adl8113_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn adl8113_init_gain_configs(dev: *mut device, st: *mut adl8113_state) -> c_int {
    static int adl8113_init_gain_configs(struct device *dev, struct adl8113_state *st)
    {
    int external_a_gain, external_b_gain;
    unsigned int i;
//
// Allocate for all 4 possible paths:
// - Internal amp and bypass (always present)
// - External bypass A and B (optional if configured)
//
    st.gain_configs = devm_kcalloc(dev, 4, sizeof(*st.gain_configs),
    GFP_KERNEL);
    if (!st.gain_configs)
    return -ENOMEM;
// Start filling the gain configurations with data
    i = 0;
// Always include internal amplifier (14dB)
    st.gain_configs[i++] = (struct adl8113_gain_config) {
    .path = ADL8113_INTERNAL_AMP,
    .gain_db = 14,
    };
// Always include internal bypass (-2dB insertion loss)
    st.gain_configs[i++] = (struct adl8113_gain_config) {
    .path = ADL8113_INTERNAL_BYPASS,
    .gain_db = -2,
    };
// Add external bypass A if configured
    if (!device_property_read_u32(dev, "adi,external-bypass-a-gain-db",
    &external_a_gain)) {
    st.gain_configs[i++] = (struct adl8113_gain_config) {
    .path = ADL8113_EXTERNAL_A,
    .gain_db = external_a_gain,
    };
    }
// Add external bypass B if configured
    if (!device_property_read_u32(dev, "adi,external-bypass-b-gain-db",
    &external_b_gain)) {
    st.gain_configs[i++] = (struct adl8113_gain_config) {
    .path = ADL8113_EXTERNAL_B,
    .gain_db = external_b_gain,
    };
    }
    st.num_gain_configs = i;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adl8113_probe(pdev: *mut platform_device) -> c_int {
    static int adl8113_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct adl8113_state *st;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.gpios = devm_gpiod_get_array(dev, "ctrl", GPIOD_OUT_LOW);
    if (IS_ERR(st.gpios))
    return dev_err_probe(dev, PTR_ERR(st.gpios),
    "failed to get control GPIOs\n");
    if (st.gpios.ndescs != 2)
    return dev_err_probe(dev, -EINVAL,
    "expected 2 control GPIOs, got %u\n",
    st.gpios.ndescs);
    ret = devm_regulator_bulk_get_enable(dev,
    ARRAY_SIZE(adl8113_supply_names),
    adl8113_supply_names);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get and enable supplies\n");
// Initialize gain configurations from devicetree
    ret = adl8113_init_gain_configs(dev, st);
    if (ret)
    return ret;
// Initialize to internal amplifier path (14dB)
    ret = adl8113_set_path(st, ADL8113_INTERNAL_AMP);
    if (ret)
    return ret;
    indio_dev.info = &adl8113_info;
    indio_dev.name = "adl8113";
    indio_dev.channels = adl8113_channels;
    indio_dev.num_channels = ARRAY_SIZE(adl8113_channels);
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id adl8113_of_match[] = {
    { .compatible = "adi,adl8113" },
    { }
    };
    MODULE_DEVICE_TABLE(of, adl8113_of_match);
    static struct platform_driver adl8113_driver = {
    .driver = {
    .name = "adl8113",
    .of_match_table = adl8113_of_match,
    },
    .probe = adl8113_probe,
    };
    module_platform_driver(adl8113_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADL8113 Low Noise Amplifier");
    MODULE_LICENSE("GPL");
