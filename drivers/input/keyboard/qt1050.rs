//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/qt1050.c
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
// Microchip AT42QT1050 QTouch Sensor Controller
//
// Copyright (C) 2019 Pengutronix, Marco Felsch <kernel@pengutronix.de>
//
// Base on AT42QT1070 driver by:
// Bo Shen <voice.shen@atmel.com>
// Copyright (C) 2011 Atmel
//

// Chip ID
pub const QT1050_CHIP_ID: c_uint = 0x00;
pub const QT1050_CHIP_ID_VER: c_uint = 0x46;
// Firmware version
pub const QT1050_FW_VERSION: c_uint = 0x01;
// Detection status
pub const QT1050_DET_STATUS: c_uint = 0x02;
// Key status
pub const QT1050_KEY_STATUS: c_uint = 0x03;
// Key Signals
pub const QT1050_KEY_SIGNAL_0_MSB: c_uint = 0x06;
pub const QT1050_KEY_SIGNAL_0_LSB: c_uint = 0x07;
pub const QT1050_KEY_SIGNAL_1_MSB: c_uint = 0x08;
pub const QT1050_KEY_SIGNAL_1_LSB: c_uint = 0x09;
pub const QT1050_KEY_SIGNAL_2_MSB: c_uint = 0x0c;
pub const QT1050_KEY_SIGNAL_2_LSB: c_uint = 0x0d;
pub const QT1050_KEY_SIGNAL_3_MSB: c_uint = 0x0e;
pub const QT1050_KEY_SIGNAL_3_LSB: c_uint = 0x0f;
pub const QT1050_KEY_SIGNAL_4_MSB: c_uint = 0x10;
pub const QT1050_KEY_SIGNAL_4_LSB: c_uint = 0x11;
// Reference data
pub const QT1050_REF_DATA_0_MSB: c_uint = 0x14;
pub const QT1050_REF_DATA_0_LSB: c_uint = 0x15;
pub const QT1050_REF_DATA_1_MSB: c_uint = 0x16;
pub const QT1050_REF_DATA_1_LSB: c_uint = 0x17;
pub const QT1050_REF_DATA_2_MSB: c_uint = 0x1a;
pub const QT1050_REF_DATA_2_LSB: c_uint = 0x1b;
pub const QT1050_REF_DATA_3_MSB: c_uint = 0x1c;
pub const QT1050_REF_DATA_3_LSB: c_uint = 0x1d;
pub const QT1050_REF_DATA_4_MSB: c_uint = 0x1e;
pub const QT1050_REF_DATA_4_LSB: c_uint = 0x1f;
// Negative threshold level
pub const QT1050_NTHR_0: c_uint = 0x21;
pub const QT1050_NTHR_1: c_uint = 0x22;
pub const QT1050_NTHR_2: c_uint = 0x24;
pub const QT1050_NTHR_3: c_uint = 0x25;
pub const QT1050_NTHR_4: c_uint = 0x26;
// Pulse / Scale
pub const QT1050_PULSE_SCALE_0: c_uint = 0x28;
pub const QT1050_PULSE_SCALE_1: c_uint = 0x29;
pub const QT1050_PULSE_SCALE_2: c_uint = 0x2b;
pub const QT1050_PULSE_SCALE_3: c_uint = 0x2c;
pub const QT1050_PULSE_SCALE_4: c_uint = 0x2d;
// Detection integrator counter / AKS
pub const QT1050_DI_AKS_0: c_uint = 0x2f;
pub const QT1050_DI_AKS_1: c_uint = 0x30;
pub const QT1050_DI_AKS_2: c_uint = 0x32;
pub const QT1050_DI_AKS_3: c_uint = 0x33;
pub const QT1050_DI_AKS_4: c_uint = 0x34;
// Charge Share Delay
pub const QT1050_CSD_0: c_uint = 0x36;
pub const QT1050_CSD_1: c_uint = 0x37;
pub const QT1050_CSD_2: c_uint = 0x39;
pub const QT1050_CSD_3: c_uint = 0x3a;
pub const QT1050_CSD_4: c_uint = 0x3b;
// Low Power Mode
pub const QT1050_LPMODE: c_uint = 0x3d;
// Calibration and Reset
pub const QT1050_RES_CAL: c_uint = 0x3f;

pub const QT1050_MAX_KEYS: c_int = 5;
pub const QT1050_RESET_TIME: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt1050_key_regs {
    pub nthr: c_uint,
    pub pulse_scale: c_uint,
    pub di_aks: c_uint,
    pub csd: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt1050_key {
    pub num: u32,
    pub charge_delay: u32,
    pub thr_cnt: u32,
    pub samples: u32,
    pub scale: u32,
    pub keycode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt1050_priv {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub regmap: *mut regmap,
    pub keys: [qt1050_key; QT1050_MAX_KEYS],
    pub keycodes: [c_ushort; QT1050_MAX_KEYS],
    pub reg_keys: u8,
    pub last_keys: u8,
}

    static const struct qt1050_key_regs qt1050_key_regs_data[] = {
    {
    .nthr = QT1050_NTHR_0,
    .pulse_scale = QT1050_PULSE_SCALE_0,
    .di_aks = QT1050_DI_AKS_0,
    .csd = QT1050_CSD_0,
    }, {
    .nthr = QT1050_NTHR_1,
    .pulse_scale = QT1050_PULSE_SCALE_1,
    .di_aks = QT1050_DI_AKS_1,
    .csd = QT1050_CSD_1,
    }, {
    .nthr = QT1050_NTHR_2,
    .pulse_scale = QT1050_PULSE_SCALE_2,
    .di_aks = QT1050_DI_AKS_2,
    .csd = QT1050_CSD_2,
    }, {
    .nthr = QT1050_NTHR_3,
    .pulse_scale = QT1050_PULSE_SCALE_3,
    .di_aks = QT1050_DI_AKS_3,
    .csd = QT1050_CSD_3,
    }, {
    .nthr = QT1050_NTHR_4,
    .pulse_scale = QT1050_PULSE_SCALE_4,
    .di_aks = QT1050_DI_AKS_4,
    .csd = QT1050_CSD_4,
    }
    };
#[no_mangle]
unsafe extern "C" fn qt1050_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool qt1050_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case QT1050_DET_STATUS:
    case QT1050_KEY_STATUS:
    case QT1050_KEY_SIGNAL_0_MSB:
    case QT1050_KEY_SIGNAL_0_LSB:
    case QT1050_KEY_SIGNAL_1_MSB:
    case QT1050_KEY_SIGNAL_1_LSB:
    case QT1050_KEY_SIGNAL_2_MSB:
    case QT1050_KEY_SIGNAL_2_LSB:
    case QT1050_KEY_SIGNAL_3_MSB:
    case QT1050_KEY_SIGNAL_3_LSB:
    case QT1050_KEY_SIGNAL_4_MSB:
    case QT1050_KEY_SIGNAL_4_LSB:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_range qt1050_readable_ranges[] = {
    regmap_reg_range(QT1050_CHIP_ID, QT1050_KEY_STATUS),
    regmap_reg_range(QT1050_KEY_SIGNAL_0_MSB, QT1050_KEY_SIGNAL_1_LSB),
    regmap_reg_range(QT1050_KEY_SIGNAL_2_MSB, QT1050_KEY_SIGNAL_4_LSB),
    regmap_reg_range(QT1050_REF_DATA_0_MSB, QT1050_REF_DATA_1_LSB),
    regmap_reg_range(QT1050_REF_DATA_2_MSB, QT1050_REF_DATA_4_LSB),
    regmap_reg_range(QT1050_NTHR_0, QT1050_NTHR_1),
    regmap_reg_range(QT1050_NTHR_2, QT1050_NTHR_4),
    regmap_reg_range(QT1050_PULSE_SCALE_0, QT1050_PULSE_SCALE_1),
    regmap_reg_range(QT1050_PULSE_SCALE_2, QT1050_PULSE_SCALE_4),
    regmap_reg_range(QT1050_DI_AKS_0, QT1050_DI_AKS_1),
    regmap_reg_range(QT1050_DI_AKS_2, QT1050_DI_AKS_4),
    regmap_reg_range(QT1050_CSD_0, QT1050_CSD_1),
    regmap_reg_range(QT1050_CSD_2, QT1050_RES_CAL),
    };
    static const struct regmap_access_table qt1050_readable_table = {
    .yes_ranges = qt1050_readable_ranges,
    .n_yes_ranges = ARRAY_SIZE(qt1050_readable_ranges),
    };
    static const struct regmap_range qt1050_writeable_ranges[] = {
    regmap_reg_range(QT1050_NTHR_0, QT1050_NTHR_1),
    regmap_reg_range(QT1050_NTHR_2, QT1050_NTHR_4),
    regmap_reg_range(QT1050_PULSE_SCALE_0, QT1050_PULSE_SCALE_1),
    regmap_reg_range(QT1050_PULSE_SCALE_2, QT1050_PULSE_SCALE_4),
    regmap_reg_range(QT1050_DI_AKS_0, QT1050_DI_AKS_1),
    regmap_reg_range(QT1050_DI_AKS_2, QT1050_DI_AKS_4),
    regmap_reg_range(QT1050_CSD_0, QT1050_CSD_1),
    regmap_reg_range(QT1050_CSD_2, QT1050_RES_CAL),
    };
    static const struct regmap_access_table qt1050_writeable_table = {
    .yes_ranges = qt1050_writeable_ranges,
    .n_yes_ranges = ARRAY_SIZE(qt1050_writeable_ranges),
    };
    static const struct regmap_config qt1050_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = QT1050_RES_CAL,
    .cache_type = REGCACHE_MAPLE,
    .wr_table = &qt1050_writeable_table,
    .rd_table = &qt1050_readable_table,
    .volatile_reg = qt1050_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn qt1050_identify(ts: *mut qt1050_priv) -> bool {
    static bool qt1050_identify(struct qt1050_priv *ts)
    {
    unsigned int val;
    int err;
// Read Chip ID
    err = regmap_read(ts.regmap, QT1050_CHIP_ID, &val);
    if (err) {
    dev_err(&ts.client.dev, "Failed to read chip ID: %d\n", err);
    return false;
    }
    if (val != QT1050_CHIP_ID_VER) {
    dev_err(&ts.client.dev, "ID %d not supported\n", val);
    return false;
    }
// Read firmware version
    err = regmap_read(ts.regmap, QT1050_FW_VERSION, &val);
    if (err) {
    dev_err(&ts.client.dev, "could not read the firmware version\n");
    return false;
    }
    dev_info(&ts.client.dev, "AT42QT1050 firmware version %1d.%1d\n",
    val >> 4, val & 0xf);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn qt1050_irq_threaded(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t qt1050_irq_threaded(int irq, void *dev_id)
    {
    struct qt1050_priv *ts = dev_id;
    struct input_dev *input = ts.input;
    unsigned long new_keys, changed;
    unsigned int val;
    int i, err;
// Read the detected status register, thus clearing interrupt
    err = regmap_read(ts.regmap, QT1050_DET_STATUS, &val);
    if (err) {
    dev_err(&ts.client.dev, "Fail to read detection status: %d\n",
    err);
    return IRQ_NONE;
    }
// Read which key changed, keys are not continuous
    err = regmap_read(ts.regmap, QT1050_KEY_STATUS, &val);
    if (err) {
    dev_err(&ts.client.dev,
    "Fail to determine the key status: %d\n", err);
    return IRQ_NONE;
    }
    new_keys = (val & 0x70) >> 2 | (val & 0x6) >> 1;
    changed = ts.last_keys ^ new_keys;
// Report registered keys only
    changed &= ts.reg_keys;
    for_each_set_bit(i, &changed, QT1050_MAX_KEYS)
    input_report_key(input, ts.keys[i].keycode,
    test_bit(i, &new_keys));
    ts.last_keys = new_keys;
    input_sync(input);
    return IRQ_HANDLED;
    }
    static const struct qt1050_key_regs *qt1050_get_key_regs(int key_num)
    {
    return &qt1050_key_regs_data[key_num];
    }
#[no_mangle]
unsafe extern "C" fn qt1050_set_key(map: *mut regmap, number: c_int, on: c_int) -> c_int {
    static int qt1050_set_key(struct regmap *map, int number, int on)
    {
    const struct qt1050_key_regs *key_regs;
    key_regs = qt1050_get_key_regs(number);
    return regmap_update_bits(map, key_regs.di_aks, 0xfc,
    on ? BIT(4) : 0x00);
    }
#[no_mangle]
unsafe extern "C" fn qt1050_apply_fw_data(ts: *mut qt1050_priv) -> c_int {
    static int qt1050_apply_fw_data(struct qt1050_priv *ts)
    {
    struct regmap *map = ts.regmap;
    struct qt1050_key *button = &ts.keys[0];
    const struct qt1050_key_regs *key_regs;
    int i, err;
// Disable all keys and enable only the specified ones
    for (i = 0; i < QT1050_MAX_KEYS; i++) {
    err = qt1050_set_key(map, i, 0);
    if (err)
    return err;
    }
    for (i = 0; i < QT1050_MAX_KEYS; i++, button++) {
// Keep KEY_RESERVED keys off
    if (button.keycode == KEY_RESERVED)
    continue;
    err = qt1050_set_key(map, button.num, 1);
    if (err)
    return err;
    key_regs = qt1050_get_key_regs(button.num);
    err = regmap_write(map, key_regs.pulse_scale,
    (button.samples << 4) | (button.scale));
    if (err)
    return err;
    err = regmap_write(map, key_regs.csd, button.charge_delay);
    if (err)
    return err;
    err = regmap_write(map, key_regs.nthr, button.thr_cnt);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt1050_parse_fw(ts: *mut qt1050_priv) -> c_int {
    static int qt1050_parse_fw(struct qt1050_priv *ts)
    {
    struct device *dev = &ts.client.dev;
    int nbuttons;
    nbuttons = device_get_child_node_count(dev);
    if (nbuttons == 0 || nbuttons > QT1050_MAX_KEYS)
    return -ENODEV;
    device_for_each_child_node_scoped(dev, child) {
    struct qt1050_key button;
// Required properties
    if (fwnode_property_read_u32(child, "linux,code",
    &button.keycode)) {
    dev_err(dev, "Button without keycode\n");
    return -EINVAL;
    }
    if (button.keycode >= KEY_MAX) {
    dev_err(dev, "Invalid keycode 0x%x\n",
    button.keycode);
    return -EINVAL;
    }
    if (fwnode_property_read_u32(child, "reg",
    &button.num)) {
    dev_err(dev, "Button without pad number\n");
    return -EINVAL;
    }
    if (button.num < 0 || button.num > QT1050_MAX_KEYS - 1)
    return -EINVAL;
    ts.reg_keys |= BIT(button.num);
// Optional properties
    if (fwnode_property_read_u32(child,
    "microchip,pre-charge-time-ns",
    &button.charge_delay)) {
    button.charge_delay = 0;
    } else {
    if (button.charge_delay % 2500 == 0)
    button.charge_delay =
    button.charge_delay / 2500;
    else
    button.charge_delay = 0;
    }
    if (fwnode_property_read_u32(child, "microchip,average-samples",
    &button.samples)) {
    button.samples = 0;
    } else {
    if (is_power_of_2(button.samples))
    button.samples = ilog2(button.samples);
    else
    button.samples = 0;
    }
    if (fwnode_property_read_u32(child, "microchip,average-scaling",
    &button.scale)) {
    button.scale = 0;
    } else {
    if (is_power_of_2(button.scale))
    button.scale = ilog2(button.scale);
    else
    button.scale = 0;
    }
    if (fwnode_property_read_u32(child, "microchip,threshold",
    &button.thr_cnt)) {
    button.thr_cnt = 20;
    } else {
    if (button.thr_cnt > 255)
    button.thr_cnt = 20;
    }
    ts.keys[button.num] = button;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt1050_probe(client: *mut i2c_client) -> c_int {
    static int qt1050_probe(struct i2c_client *client)
    {
    struct qt1050_priv *ts;
    struct input_dev *input;
    struct device *dev = &client.dev;
    struct regmap *map;
    unsigned int status, i;
    int err;
// Check basic functionality
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE)) {
    dev_err(&client.dev, "%s adapter not supported\n",
    dev_driver_string(&client.adapter.dev));
    return -ENODEV;
    }
    if (!client.irq) {
    dev_err(dev, "assign a irq line to this device\n");
    return -EINVAL;
    }
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    map = devm_regmap_init_i2c(client, &qt1050_regmap_config);
    if (IS_ERR(map))
    return PTR_ERR(map);
    ts.client = client;
    ts.input = input;
    ts.regmap = map;
    i2c_set_clientdata(client, ts);
// Identify the qt1050 chip
    if (!qt1050_identify(ts))
    return -ENODEV;
// Get pdata
    err = qt1050_parse_fw(ts);
    if (err) {
    dev_err(dev, "Failed to parse firmware: %d\n", err);
    return err;
    }
    input.name = "AT42QT1050 QTouch Sensor";
    input.dev.parent = &client.dev;
    input.id.bustype = BUS_I2C;
// Add the keycode
    input.keycode = ts.keycodes;
    input.keycodesize = sizeof(ts.keycodes[0]);
    input.keycodemax = QT1050_MAX_KEYS;
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < QT1050_MAX_KEYS; i++) {
    ts.keycodes[i] = ts.keys[i].keycode;
    __set_bit(ts.keycodes[i], input.keybit);
    }
// Trigger re-calibration
    err = regmap_update_bits(ts.regmap, QT1050_RES_CAL, 0x7f,
    QT1050_RES_CAL_CALIBRATE);
    if (err) {
    dev_err(dev, "Trigger calibration failed: %d\n", err);
    return err;
    }
    err = regmap_read_poll_timeout(ts.regmap, QT1050_DET_STATUS, status,
    status >> 7 == 1, 10000, 200000);
    if (err) {
    dev_err(dev, "Calibration failed: %d\n", err);
    return err;
    }
// Soft reset to set defaults
    err = regmap_update_bits(ts.regmap, QT1050_RES_CAL,
    QT1050_RES_CAL_RESET, QT1050_RES_CAL_RESET);
    if (err) {
    dev_err(dev, "Trigger soft reset failed: %d\n", err);
    return err;
    }
    msleep(QT1050_RESET_TIME);
// Set pdata
    err = qt1050_apply_fw_data(ts);
    if (err) {
    dev_err(dev, "Failed to set firmware data: %d\n", err);
    return err;
    }
    err = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    qt1050_irq_threaded, IRQF_ONESHOT,
    "qt1050", ts);
    if (err) {
    dev_err(&client.dev, "Failed to request irq: %d\n", err);
    return err;
    }
// Clear #CHANGE line
    err = regmap_read(ts.regmap, QT1050_DET_STATUS, &status);
    if (err) {
    dev_err(dev, "Failed to clear #CHANGE line level: %d\n", err);
    return err;
    }
// Register the input device
    err = input_register_device(ts.input);
    if (err) {
    dev_err(&client.dev, "Failed to register input device: %d\n",
    err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt1050_suspend(dev: *mut device) -> c_int {
    static int qt1050_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct qt1050_priv *ts = i2c_get_clientdata(client);
    disable_irq(client.irq);
//
// Set measurement interval to 1s (125 x 8ms) if wakeup is allowed
// else turn off. The 1s interval seems to be a good compromise between
// low power and response time.
//
    return regmap_write(ts.regmap, QT1050_LPMODE,
    device_may_wakeup(dev) ? 125 : 0);
    }
#[no_mangle]
unsafe extern "C" fn qt1050_resume(dev: *mut device) -> c_int {
    static int qt1050_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct qt1050_priv *ts = i2c_get_clientdata(client);
    enable_irq(client.irq);
// Set measurement interval back to 16ms (2 x 8ms)
    return regmap_write(ts.regmap, QT1050_LPMODE, 2);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(qt1050_pm_ops, qt1050_suspend, qt1050_resume);
    static const struct of_device_id __maybe_unused qt1050_of_match[] = {
    { .compatible = "microchip,qt1050", },
    { },
    };
    MODULE_DEVICE_TABLE(of, qt1050_of_match);
    static struct i2c_driver qt1050_driver = {
    .driver	= {
    .name = "qt1050",
    .of_match_table = of_match_ptr(qt1050_of_match),
    .pm = pm_sleep_ptr(&qt1050_pm_ops),
    },
    .probe = qt1050_probe,
    };
    module_i2c_driver(qt1050_driver);
    MODULE_AUTHOR("Marco Felsch <kernel@pengutronix.de");
    MODULE_DESCRIPTION("Driver for AT42QT1050 QTouch sensor");
    MODULE_LICENSE("GPL v2");
