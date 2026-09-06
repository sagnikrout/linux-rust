//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiostat/lmp91000.c
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
// lmp91000.c - Support for Texas Instruments digital potentiostats
//
// Copyright (C) 2016, 2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//
// TODO: bias voltage + polarity control, and multiple chip support
//

pub const LMP91000_REG_LOCK: c_uint = 0x01;
pub const LMP91000_REG_TIACN: c_uint = 0x10;
pub const LMP91000_REG_TIACN_GAIN_SHIFT: c_int = 2;
pub const LMP91000_REG_REFCN: c_uint = 0x11;
pub const LMP91000_REG_REFCN_EXT_REF: c_uint = 0x20;
pub const LMP91000_REG_REFCN_50_ZERO: c_uint = 0x80;
pub const LMP91000_REG_MODECN: c_uint = 0x12;
pub const LMP91000_REG_MODECN_3LEAD: c_uint = 0x03;
pub const LMP91000_REG_MODECN_TEMP: c_uint = 0x07;

    static const int lmp91000_tia_gain[] = { 0, 2750, 3500, 7000, 14000, 35000,
    120000, 350000 };
    static const int lmp91000_rload[] = { 10, 33, 50, 100 };

    static const u16 lmp91000_temp_lut[] = {
    1875, 1867, 1860, 1852, 1844, 1836, 1828, 1821, 1813, 1805,
    1797, 1789, 1782, 1774, 1766, 1758, 1750, 1742, 1734, 1727,
    1719, 1711, 1703, 1695, 1687, 1679, 1671, 1663, 1656, 1648,
    1640, 1632, 1624, 1616, 1608, 1600, 1592, 1584, 1576, 1568,
    1560, 1552, 1544, 1536, 1528, 1520, 1512, 1504, 1496, 1488,
    1480, 1472, 1464, 1456, 1448, 1440, 1432, 1424, 1415, 1407,
    1399, 1391, 1383, 1375, 1367, 1359, 1351, 1342, 1334, 1326,
    1318, 1310, 1302, 1293, 1285, 1277, 1269, 1261, 1253, 1244,
    1236, 1228, 1220, 1212, 1203, 1195, 1187, 1179, 1170, 1162,
    1154, 1146, 1137, 1129, 1121, 1112, 1104, 1096, 1087, 1079,
    1071, 1063, 1054, 1046, 1038, 1029, 1021, 1012, 1004,  996,
    987,  979,  971,  962,  954,  945,  937,  929,  920,  912,
    903,  895,  886,  878,  870,  861 };
    static const struct regmap_config lmp91000_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lmp91000_data {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub trig: *mut iio_trigger,
    pub cb_buffer: *mut iio_cb_buffer,
    pub adc_chan: *mut iio_channel,
    pub completion: completion,
    pub chan_select: u8,
// 64-bit data + 64-bit naturally aligned timestamp
    pub __aligned(8): u32 buffer[4],
}

    static const struct iio_chan_spec lmp91000_channels[] = {
    { /* chemical channel mV */
    .type = IIO_VOLTAGE,
    .channel = 0,
    .address = LMP91000_REG_MODECN_3LEAD,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_OFFSET) |
    BIT(IIO_CHAN_INFO_SCALE),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 32,
    .storagebits = 32,
    },
    },
    IIO_CHAN_SOFT_TIMESTAMP(1),
    { /* temperature channel mV */
    .type = IIO_TEMP,
    .channel = 1,
    .address = LMP91000_REG_MODECN_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .scan_index = -1,
    },
    };
#[no_mangle]
unsafe extern "C" fn lmp91000_read(data: *mut lmp91000_data, channel: c_int, val: *mut c_int) -> c_int {
    static int lmp91000_read(struct lmp91000_data *data, int channel, int *val)
    {
    int state, ret;
    ret = regmap_read(data.regmap, LMP91000_REG_MODECN, &state);
    if (ret)
    return -EINVAL;
    ret = regmap_write(data.regmap, LMP91000_REG_MODECN, channel);
    if (ret)
    return -EINVAL;
// delay till first temperature reading is complete
    if (state != channel && channel == LMP91000_REG_MODECN_TEMP)
    usleep_range(3000, 4000);
    data.chan_select = channel != LMP91000_REG_MODECN_3LEAD;
    iio_trigger_poll_nested(data.trig);
    ret = wait_for_completion_timeout(&data.completion, HZ);
    reinit_completion(&data.completion);
    if (!ret)
    return -ETIMEDOUT;
// val = data->buffer[data->chan_select];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lmp91000_buffer_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t lmp91000_buffer_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct lmp91000_data *data = iio_priv(indio_dev);
    int ret, val;
    memset(data.buffer, 0, sizeof(data.buffer));
    ret = lmp91000_read(data, LMP91000_REG_MODECN_3LEAD, &val);
    if (!ret) {
    data.buffer[0] = val;
    iio_push_to_buffers_with_timestamp(indio_dev, data.buffer,
    iio_get_time_ns(indio_dev));
    }
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int lmp91000_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct lmp91000_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    case IIO_CHAN_INFO_PROCESSED: {
    let mut ret: c_int = iio_channel_start_all_cb(data.cb_buffer);
    if (ret)
    return ret;
    ret = lmp91000_read(data, chan.address, val);
    iio_channel_stop_all_cb(data.cb_buffer);
    if (ret)
    return ret;
    if (mask == IIO_CHAN_INFO_PROCESSED) {
    int tmp, i;
    ret = iio_convert_raw_to_processed(data.adc_chan,
// val, &tmp, 1);
    if (ret)
    return ret;
    for (i = 0; i < ARRAY_SIZE(lmp91000_temp_lut); i++)
    if (lmp91000_temp_lut[i] < tmp)
    break;
// val = (LMP91000_TEMP_BASE + i) * 1000;
    }
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_OFFSET:
    return iio_read_channel_offset(data.adc_chan, val, val2);
    case IIO_CHAN_INFO_SCALE:
    return iio_read_channel_scale(data.adc_chan, val, val2);
    }
    return -EINVAL;
    }
    static const struct iio_info lmp91000_info = {
    .read_raw = lmp91000_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn lmp91000_read_config(data: *mut lmp91000_data) -> c_int {
    static int lmp91000_read_config(struct lmp91000_data *data)
    {
    struct device *dev = data.dev;
    unsigned int reg, val;
    int i, ret;
    ret = device_property_read_u32(dev, "ti,tia-gain-ohm", &val);
    if (ret) {
    if (!device_property_read_bool(dev, "ti,external-tia-resistor")) {
    dev_err(dev, "no ti,tia-gain-ohm defined and external resistor not specified\n");
    return ret;
    }
    val = 0;
    }
    ret = -EINVAL;
    for (i = 0; i < ARRAY_SIZE(lmp91000_tia_gain); i++) {
    if (lmp91000_tia_gain[i] == val) {
    reg = i << LMP91000_REG_TIACN_GAIN_SHIFT;
    ret = 0;
    break;
    }
    }
    if (ret) {
    dev_err(dev, "invalid ti,tia-gain-ohm %d\n", val);
    return ret;
    }
    ret = device_property_read_u32(dev, "ti,rload-ohm", &val);
    if (ret) {
    val = 100;
    dev_info(dev, "no ti,rload-ohm defined, default to %d\n", val);
    }
    ret = -EINVAL;
    for (i = 0; i < ARRAY_SIZE(lmp91000_rload); i++) {
    if (lmp91000_rload[i] == val) {
    reg |= i;
    ret = 0;
    break;
    }
    }
    if (ret) {
    dev_err(dev, "invalid ti,rload-ohm %d\n", val);
    return ret;
    }
    regmap_write(data.regmap, LMP91000_REG_LOCK, 0);
    regmap_write(data.regmap, LMP91000_REG_TIACN, reg);
    regmap_write(data.regmap, LMP91000_REG_REFCN,
    LMP91000_REG_REFCN_EXT_REF | LMP91000_REG_REFCN_50_ZERO);
    regmap_write(data.regmap, LMP91000_REG_LOCK, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lmp91000_buffer_cb(val: *const c_void, private: *mut c_void) -> c_int {
    static int lmp91000_buffer_cb(const void *val, void *private)
    {
    struct iio_dev *indio_dev = private;
    struct lmp91000_data *data = iio_priv(indio_dev);
    data.buffer[data.chan_select] = *((int *)val);
    complete_all(&data.completion);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lmp91000_buffer_postenable(indio_dev: *mut iio_dev) -> c_int {
    static int lmp91000_buffer_postenable(struct iio_dev *indio_dev)
    {
    struct lmp91000_data *data = iio_priv(indio_dev);
    return iio_channel_start_all_cb(data.cb_buffer);
    }
#[no_mangle]
unsafe extern "C" fn lmp91000_buffer_predisable(indio_dev: *mut iio_dev) -> c_int {
    static int lmp91000_buffer_predisable(struct iio_dev *indio_dev)
    {
    struct lmp91000_data *data = iio_priv(indio_dev);
    iio_channel_stop_all_cb(data.cb_buffer);
    return 0;
    }
    static const struct iio_buffer_setup_ops lmp91000_buffer_setup_ops = {
    .postenable = lmp91000_buffer_postenable,
    .predisable = lmp91000_buffer_predisable,
    };
#[no_mangle]
unsafe extern "C" fn lmp91000_probe(client: *mut i2c_client) -> c_int {
    static int lmp91000_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct lmp91000_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    indio_dev.info = &lmp91000_info;
    indio_dev.channels = lmp91000_channels;
    indio_dev.num_channels = ARRAY_SIZE(lmp91000_channels);
    indio_dev.name = LMP91000_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    i2c_set_clientdata(client, indio_dev);
    data = iio_priv(indio_dev);
    data.dev = dev;
    data.regmap = devm_regmap_init_i2c(client, &lmp91000_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(dev, "regmap initialization failed.\n");
    return PTR_ERR(data.regmap);
    }
    data.trig = devm_iio_trigger_alloc(dev, "%s-mux%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!data.trig)
    return -ENOMEM;
    init_completion(&data.completion);
    ret = lmp91000_read_config(data);
    if (ret)
    return ret;
    ret = iio_trigger_set_immutable(iio_channel_cb_get_iio_dev(data.cb_buffer),
    data.trig);
    if (ret) {
    dev_err(dev, "cannot set immutable trigger.\n");
    return ret;
    }
    ret = iio_trigger_register(data.trig);
    if (ret) {
    dev_err(dev, "cannot register iio trigger.\n");
    return ret;
    }
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(),
    &lmp91000_buffer_handler,
    &lmp91000_buffer_setup_ops);
    if (ret)
    goto error_unreg_trigger;
    data.cb_buffer = iio_channel_get_all_cb(dev, &lmp91000_buffer_cb,
    indio_dev);
    if (IS_ERR(data.cb_buffer)) {
    if (PTR_ERR(data.cb_buffer) == -ENODEV)
    ret = -EPROBE_DEFER;
    else
    ret = PTR_ERR(data.cb_buffer);
    goto error_unreg_buffer;
    }
    data.adc_chan = iio_channel_cb_get_channels(data.cb_buffer);
    ret = iio_device_register(indio_dev);
    if (ret)
    goto error_unreg_cb_buffer;
    return 0;
    error_unreg_cb_buffer:
    iio_channel_release_all_cb(data.cb_buffer);
    error_unreg_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    error_unreg_trigger:
    iio_trigger_unregister(data.trig);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lmp91000_remove(client: *mut i2c_client) {
    static void lmp91000_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct lmp91000_data *data = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    iio_channel_stop_all_cb(data.cb_buffer);
    iio_channel_release_all_cb(data.cb_buffer);
    iio_triggered_buffer_cleanup(indio_dev);
    iio_trigger_unregister(data.trig);
    }
    static const struct of_device_id lmp91000_of_match[] = {
    { .compatible = "ti,lmp91000", },
    { .compatible = "ti,lmp91002", },
    { }
    };
    MODULE_DEVICE_TABLE(of, lmp91000_of_match);
    static const struct i2c_device_id lmp91000_id[] = {
    { .name = "lmp91000" },
    { .name = "lmp91002" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lmp91000_id);
    static struct i2c_driver lmp91000_driver = {
    .driver = {
    .name = LMP91000_DRV_NAME,
    .of_match_table = lmp91000_of_match,
    },
    .probe = lmp91000_probe,
    .remove = lmp91000_remove,
    .id_table = lmp91000_id,
    };
    module_i2c_driver(lmp91000_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("LMP91000 digital potentiostat");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
