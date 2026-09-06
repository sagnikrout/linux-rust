//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/ak8975.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// A sensor driver for the magnetometer AK8975.
//
// Magnetic compass sensor driver for monitoring magnetic flux information.
//
// Copyright (c) 2010, NVIDIA Corporation.
//

//
// Register definitions, as well as various shifts and masks to get at the
// individual fields of the registers.
//
pub const AK8975_REG_WIA: c_uint = 0x00;
pub const AK8975_DEVICE_ID: c_uint = 0x48;
pub const AK8975_REG_INFO: c_uint = 0x01;
pub const AK8975_REG_ST1: c_uint = 0x02;
pub const AK8975_REG_ST1_DRDY_SHIFT: c_int = 0;

pub const AK8975_REG_HXL: c_uint = 0x03;
pub const AK8975_REG_HXH: c_uint = 0x04;
pub const AK8975_REG_HYL: c_uint = 0x05;
pub const AK8975_REG_HYH: c_uint = 0x06;
pub const AK8975_REG_HZL: c_uint = 0x07;
pub const AK8975_REG_HZH: c_uint = 0x08;
pub const AK8975_REG_ST2: c_uint = 0x09;
pub const AK8975_REG_ST2_DERR_SHIFT: c_int = 2;

pub const AK8975_REG_ST2_HOFL_SHIFT: c_int = 3;

pub const AK8975_REG_CNTL: c_uint = 0x0A;
pub const AK8975_REG_CNTL_MODE_SHIFT: c_int = 0;

pub const AK8975_REG_CNTL_MODE_POWER_DOWN: c_uint = 0x00;
pub const AK8975_REG_CNTL_MODE_ONCE: c_uint = 0x01;
pub const AK8975_REG_CNTL_MODE_SELF_TEST: c_uint = 0x08;
pub const AK8975_REG_CNTL_MODE_FUSE_ROM: c_uint = 0x0F;
pub const AK8975_REG_RSVC: c_uint = 0x0B;
pub const AK8975_REG_ASTC: c_uint = 0x0C;
pub const AK8975_REG_TS1: c_uint = 0x0D;
pub const AK8975_REG_TS2: c_uint = 0x0E;
pub const AK8975_REG_I2CDIS: c_uint = 0x0F;
pub const AK8975_REG_ASAX: c_uint = 0x10;
pub const AK8975_REG_ASAY: c_uint = 0x11;
pub const AK8975_REG_ASAZ: c_uint = 0x12;

//
// AK09912 Register definitions
//
pub const AK09912_REG_WIA1: c_uint = 0x00;
pub const AK09912_REG_WIA2: c_uint = 0x01;
pub const AK09918_DEVICE_ID: c_uint = 0x0C;
pub const AK09916_DEVICE_ID: c_uint = 0x09;
pub const AK09912_DEVICE_ID: c_uint = 0x04;
pub const AK09911_DEVICE_ID: c_uint = 0x05;
pub const AK09911_REG_INFO1: c_uint = 0x02;
pub const AK09911_REG_INFO2: c_uint = 0x03;
pub const AK09912_REG_ST1: c_uint = 0x10;
pub const AK09912_REG_ST1_DRDY_SHIFT: c_int = 0;

pub const AK09912_REG_HXL: c_uint = 0x11;
pub const AK09912_REG_HXH: c_uint = 0x12;
pub const AK09912_REG_HYL: c_uint = 0x13;
pub const AK09912_REG_HYH: c_uint = 0x14;
pub const AK09912_REG_HZL: c_uint = 0x15;
pub const AK09912_REG_HZH: c_uint = 0x16;
pub const AK09912_REG_TMPS: c_uint = 0x17;
pub const AK09912_REG_ST2: c_uint = 0x18;
pub const AK09912_REG_ST2_HOFL_SHIFT: c_int = 3;

pub const AK09912_REG_CNTL1: c_uint = 0x30;
pub const AK09912_REG_CNTL2: c_uint = 0x31;
pub const AK09912_REG_CNTL_MODE_POWER_DOWN: c_uint = 0x00;
pub const AK09912_REG_CNTL_MODE_ONCE: c_uint = 0x01;
pub const AK09912_REG_CNTL_MODE_SELF_TEST: c_uint = 0x10;
pub const AK09912_REG_CNTL_MODE_FUSE_ROM: c_uint = 0x1F;
pub const AK09912_REG_CNTL2_MODE_SHIFT: c_int = 0;

pub const AK09912_REG_CNTL3: c_uint = 0x32;
pub const AK09912_REG_TS1: c_uint = 0x33;
pub const AK09912_REG_TS2: c_uint = 0x34;
pub const AK09912_REG_TS3: c_uint = 0x35;
pub const AK09912_REG_I2CDIS: c_uint = 0x36;
pub const AK09912_REG_TS4: c_uint = 0x37;
pub const AK09912_REG_ASAX: c_uint = 0x60;
pub const AK09912_REG_ASAY: c_uint = 0x61;
pub const AK09912_REG_ASAZ: c_uint = 0x62;

//
// Precalculate scale factor (in Gauss units) for each axis and
// store in the device data.
//
// This scale factor is axis-dependent, and is derived from 3 calibration
// factors ASA(x), ASA(y), and ASA(z).
//
// These ASA values are read from the sensor device at start of day, and
// cached in the device context struct.
//
// Adjusting the flux value with the sensitivity adjustment value should be
// done via the following formula:
//
// Hadj = H * ( ( ( (ASA-128)*0.5 ) / 128 ) + 1 )
// where H is the raw value, ASA is the sensitivity adjustment, and Hadj
// is the resultant adjusted value.
//
// We reduce the formula to:
//
// Hadj = H * (ASA + 128) / 256
//
// H is in the range of -4096 to 4095.  The magnetometer has a range of
// +-1229uT.  To go from the raw value to uT is:
//
// HuT = H * 1229/4096, or roughly, 3/10.
//
// Since 1uT = 0.01 gauss, our final scale factor becomes:
//
// Hadj = H * ((ASA + 128) / 256) * 3/10 * 1/100
// Hadj = H * ((ASA + 128) * 0.003) / 256
//
// Since ASA doesn't change, we cache the resultant scale factor into the
// device context in ak8975_setup().
//
// Given we use IIO_VAL_INT_PLUS_MICRO bit when displaying the scale, we
// multiply the stored scale value by 1e6.
//
#[no_mangle]
unsafe extern "C" fn ak8975_raw_to_gauss(data: u16) -> c_long {
    static long ak8975_raw_to_gauss(u16 data)
    {
    return (((long)data + 128) * 3000) / 256;
    }
//
// For AK8963 and AK09911, same calculation, but the device is less sensitive:
//
// H is in the range of +-8190.  The magnetometer has a range of
// +-4912uT.  To go from the raw value to uT is:
//
// HuT = H * 4912/8190, or roughly, 6/10, instead of 3/10.
//
#[no_mangle]
unsafe extern "C" fn ak8963_09911_raw_to_gauss(data: u16) -> c_long {
    static long ak8963_09911_raw_to_gauss(u16 data)
    {
    return (((long)data + 128) * 6000) / 256;
    }
//
// For AK09912, same calculation, except the device is more sensitive:
//
// H is in the range of -32752 to 32752.  The magnetometer has a range of
// +-4912uT.  To go from the raw value to uT is:
//
// HuT = H * 4912/32752, or roughly, 3/20, instead of 3/10.
//
#[no_mangle]
unsafe extern "C" fn ak09912_raw_to_gauss(data: u16) -> c_long {
    static long ak09912_raw_to_gauss(u16 data)
    {
    return (((long)data + 128) * 1500) / 256;
    }
// Compatible Asahi Kasei Compass parts
    enum asahi_compass_chipset {
    AK8975,
    AK8963,
    AK09911,
    AK09912,
    AK09916,
    AK09918,
    };
    enum ak_ctrl_reg_addr {
    ST1,
    ST2,
    CNTL,
    ASA_BASE,
    MAX_REGS,
    REGS_END,
    };
    enum ak_ctrl_reg_mask {
    ST1_DRDY,
    ST2_HOFL,
    ST2_DERR,
    CNTL_MODE,
    MASK_END,
    };
    enum ak_ctrl_mode {
    POWER_DOWN,
    MODE_ONCE,
    SELF_TEST,
    FUSE_ROM,
    MODE_END,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak_def {
    pub type: enum asahi_compass_chipset,
    pub data): *mut *mut long (raw_to_gauss)(u16,
    pub range: u16,
    pub ctrl_regs: [u8; REGS_END],
    pub ctrl_masks: [u8; MASK_END],
    pub ctrl_modes: [u8; MODE_END],
    pub data_regs: [u8; 3],
}

    static const struct ak_def ak_def_array[] = {
    [AK8975] = {
    .type = AK8975,
    .raw_to_gauss = ak8975_raw_to_gauss,
    .range = 4096,
    .ctrl_regs = {
    AK8975_REG_ST1,
    AK8975_REG_ST2,
    AK8975_REG_CNTL,
    AK8975_REG_ASAX,
    AK8975_MAX_REGS},
    .ctrl_masks = {
    AK8975_REG_ST1_DRDY_MASK,
    AK8975_REG_ST2_HOFL_MASK,
    AK8975_REG_ST2_DERR_MASK,
    AK8975_REG_CNTL_MODE_MASK},
    .ctrl_modes = {
    AK8975_REG_CNTL_MODE_POWER_DOWN,
    AK8975_REG_CNTL_MODE_ONCE,
    AK8975_REG_CNTL_MODE_SELF_TEST,
    AK8975_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK8975_REG_HXL,
    AK8975_REG_HYL,
    AK8975_REG_HZL},
    },
    [AK8963] = {
    .type = AK8963,
    .raw_to_gauss = ak8963_09911_raw_to_gauss,
    .range = 8190,
    .ctrl_regs = {
    AK8975_REG_ST1,
    AK8975_REG_ST2,
    AK8975_REG_CNTL,
    AK8975_REG_ASAX,
    AK8975_MAX_REGS},
    .ctrl_masks = {
    AK8975_REG_ST1_DRDY_MASK,
    AK8975_REG_ST2_HOFL_MASK,
    0,
    AK8975_REG_CNTL_MODE_MASK},
    .ctrl_modes = {
    AK8975_REG_CNTL_MODE_POWER_DOWN,
    AK8975_REG_CNTL_MODE_ONCE,
    AK8975_REG_CNTL_MODE_SELF_TEST,
    AK8975_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK8975_REG_HXL,
    AK8975_REG_HYL,
    AK8975_REG_HZL},
    },
    [AK09911] = {
    .type = AK09911,
    .raw_to_gauss = ak8963_09911_raw_to_gauss,
    .range = 8192,
    .ctrl_regs = {
    AK09912_REG_ST1,
    AK09912_REG_ST2,
    AK09912_REG_CNTL2,
    AK09912_REG_ASAX,
    AK09912_MAX_REGS},
    .ctrl_masks = {
    AK09912_REG_ST1_DRDY_MASK,
    AK09912_REG_ST2_HOFL_MASK,
    0,
    AK09912_REG_CNTL2_MODE_MASK},
    .ctrl_modes = {
    AK09912_REG_CNTL_MODE_POWER_DOWN,
    AK09912_REG_CNTL_MODE_ONCE,
    AK09912_REG_CNTL_MODE_SELF_TEST,
    AK09912_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK09912_REG_HXL,
    AK09912_REG_HYL,
    AK09912_REG_HZL},
    },
    [AK09912] = {
    .type = AK09912,
    .raw_to_gauss = ak09912_raw_to_gauss,
    .range = 32752,
    .ctrl_regs = {
    AK09912_REG_ST1,
    AK09912_REG_ST2,
    AK09912_REG_CNTL2,
    AK09912_REG_ASAX,
    AK09912_MAX_REGS},
    .ctrl_masks = {
    AK09912_REG_ST1_DRDY_MASK,
    AK09912_REG_ST2_HOFL_MASK,
    0,
    AK09912_REG_CNTL2_MODE_MASK},
    .ctrl_modes = {
    AK09912_REG_CNTL_MODE_POWER_DOWN,
    AK09912_REG_CNTL_MODE_ONCE,
    AK09912_REG_CNTL_MODE_SELF_TEST,
    AK09912_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK09912_REG_HXL,
    AK09912_REG_HYL,
    AK09912_REG_HZL},
    },
    [AK09916] = {
    .type = AK09916,
    .raw_to_gauss = ak09912_raw_to_gauss,
    .range = 32752,
    .ctrl_regs = {
    AK09912_REG_ST1,
    AK09912_REG_ST2,
    AK09912_REG_CNTL2,
    AK09912_REG_ASAX,
    AK09912_MAX_REGS},
    .ctrl_masks = {
    AK09912_REG_ST1_DRDY_MASK,
    AK09912_REG_ST2_HOFL_MASK,
    0,
    AK09912_REG_CNTL2_MODE_MASK},
    .ctrl_modes = {
    AK09912_REG_CNTL_MODE_POWER_DOWN,
    AK09912_REG_CNTL_MODE_ONCE,
    AK09912_REG_CNTL_MODE_SELF_TEST,
    AK09912_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK09912_REG_HXL,
    AK09912_REG_HYL,
    AK09912_REG_HZL},
    },
    [AK09918] = {
// ak09918 is register compatible with ak09912 this is for avoid
// unknown id messages.
//
    .type = AK09918,
    .raw_to_gauss = ak09912_raw_to_gauss,
    .range = 32752,
    .ctrl_regs = {
    AK09912_REG_ST1,
    AK09912_REG_ST2,
    AK09912_REG_CNTL2,
    AK09912_REG_ASAX,
    AK09912_MAX_REGS},
    .ctrl_masks = {
    AK09912_REG_ST1_DRDY_MASK,
    AK09912_REG_ST2_HOFL_MASK,
    0,
    AK09912_REG_CNTL2_MODE_MASK},
    .ctrl_modes = {
    AK09912_REG_CNTL_MODE_POWER_DOWN,
    AK09912_REG_CNTL_MODE_ONCE,
    AK09912_REG_CNTL_MODE_SELF_TEST,
    AK09912_REG_CNTL_MODE_FUSE_ROM},
    .data_regs = {
    AK09912_REG_HXL,
    AK09912_REG_HYL,
    AK09912_REG_HZL},
    }
    };
//
// Per-instance context data for the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak8975_data {
    pub client: *mut i2c_client,
    pub def: *const ak_def,
    pub lock: mutex,
    pub asa: [u8; 3],
    pub raw_to_gauss: [c_long; 3],
    pub eoc_gpiod: *mut gpio_desc,
    pub reset_gpiod: *mut gpio_desc,
    pub eoc_irq: c_int,
    pub data_ready_queue: wait_queue_head_t,
    pub flags: c_ulong,
    pub cntl_cache: u8,
    pub orientation: iio_mount_matrix,
    pub vdd: *mut regulator,
    pub vid: *mut regulator,
// Ensure natural alignment of timestamp
    struct {
    pub channels: [i16; 3],
    pub ts: aligned_s64,
    pub scan: },
}

// Enable attached power regulator if any.
#[no_mangle]
unsafe extern "C" fn ak8975_power_on(data: *const ak8975_data) -> c_int {
    static int ak8975_power_on(const struct ak8975_data *data)
    {
    int ret;
    ret = regulator_enable(data.vdd);
    if (ret) {
    dev_warn(&data.client.dev,
    "Failed to enable specified Vdd supply\n");
    return ret;
    }
    ret = regulator_enable(data.vid);
    if (ret) {
    dev_warn(&data.client.dev,
    "Failed to enable specified Vid supply\n");
    regulator_disable(data.vdd);
    return ret;
    }
    gpiod_set_value_cansleep(data.reset_gpiod, 0);
//
// According to the datasheet the power supply rise time is 200us
// and the minimum wait time before mode setting is 100us, in
// total 300us. Add some margin and say minimum 500us here.
//
    fsleep(500);
    return 0;
    }
// Disable attached power regulator if any.
#[no_mangle]
unsafe extern "C" fn ak8975_power_off(data: *const ak8975_data) {
    static void ak8975_power_off(const struct ak8975_data *data)
    {
    gpiod_set_value_cansleep(data.reset_gpiod, 1);
    regulator_disable(data.vid);
    regulator_disable(data.vdd);
    }
//
// Return 0 if the i2c device is the one we expect.
// return a negative error number otherwise
//
    static int ak8975_who_i_am(const struct ak8975_data *data,
    enum asahi_compass_chipset type)
    {
    struct i2c_client *client = data.client;
    u8 wia_val[2];
    int ret;
//
// Signature for each device:
// Device   |  WIA1      |  WIA2
// AK09918  |  DEVICE_ID_|  AK09918_DEVICE_ID
// AK09916  |  DEVICE_ID_|  AK09916_DEVICE_ID
// AK09912  |  DEVICE_ID |  AK09912_DEVICE_ID
// AK09911  |  DEVICE_ID |  AK09911_DEVICE_ID
// AK8975   |  DEVICE_ID |  NA
// AK8963   |  DEVICE_ID |  NA
//
    ret = i2c_smbus_read_i2c_block_data_or_emulated(client,
    AK09912_REG_WIA1,
    sizeof(wia_val),
    wia_val);
    if (ret < 0) {
    dev_err(&client.dev, "Error reading WIA\n");
    return ret;
    }
    if (ret != sizeof(wia_val)) {
    dev_err(&client.dev, "Error reading WIA\n");
    return -EIO;
    }
    if (wia_val[0] != AK8975_DEVICE_ID)
    return -ENODEV;
    switch (type) {
    case AK8975:
    case AK8963:
    return 0;
    case AK09911:
    if (wia_val[1] == AK09911_DEVICE_ID)
    return 0;
    break;
    case AK09912:
    if (wia_val[1] == AK09912_DEVICE_ID)
    return 0;
    break;
    case AK09916:
    if (wia_val[1] == AK09916_DEVICE_ID)
    return 0;
    break;
    case AK09918:
    if (wia_val[1] == AK09918_DEVICE_ID)
    return 0;
    break;
    }
    dev_info(&client.dev, "Device ID %x is unknown.\n", wia_val[1]);
//
// Let driver to probe on unknown id for support more register
// compatible variants.
//
    return 0;
    }
//
// Helper function to write to CNTL register.
//
#[no_mangle]
unsafe extern "C" fn ak8975_set_mode(data: *mut ak8975_data, mode: enum ak_ctrl_mode) -> c_int {
    static int ak8975_set_mode(struct ak8975_data *data, enum ak_ctrl_mode mode)
    {
    u8 regval;
    int ret;
    regval = (data.cntl_cache & ~data.def.ctrl_masks[CNTL_MODE]) |
    data.def.ctrl_modes[mode];
    ret = i2c_smbus_write_byte_data(data.client,
    data.def.ctrl_regs[CNTL], regval);
    if (ret < 0)
    return ret;
    data.cntl_cache = regval;
// After mode change wait at least 100us
    fsleep(100);
    return 0;
    }
//
// Handle data ready irq
//
#[no_mangle]
unsafe extern "C" fn ak8975_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ak8975_irq_handler(int irq, void *data)
    {
    struct ak8975_data *ak8975 = data;
    set_bit(0, &ak8975.flags);
    wake_up(&ak8975.data_ready_queue);
    return IRQ_HANDLED;
    }
//
// Install data ready interrupt handler
//
#[no_mangle]
unsafe extern "C" fn ak8975_setup_irq(data: *mut ak8975_data) -> c_int {
    static int ak8975_setup_irq(struct ak8975_data *data)
    {
    struct i2c_client *client = data.client;
    int irq;
    int ret;
    init_waitqueue_head(&data.data_ready_queue);
    clear_bit(0, &data.flags);
    if (client.irq)
    irq = client.irq;
    else
    irq = gpiod_to_irq(data.eoc_gpiod);
    ret = devm_request_irq(&client.dev, irq, ak8975_irq_handler,
    IRQF_TRIGGER_RISING,
    dev_name(&client.dev), data);
    if (ret)
    return ret;
    data.eoc_irq = irq;
    return 0;
    }
//
// Perform some start-of-day setup, including reading the asa calibration
// values and caching them.
//
#[no_mangle]
unsafe extern "C" fn ak8975_setup(data: *mut ak8975_data) -> c_int {
    static int ak8975_setup(struct ak8975_data *data)
    {
    struct i2c_client *client = data.client;
    int ret;
// Write the fused rom access mode.
    ret = ak8975_set_mode(data, FUSE_ROM);
    if (ret < 0) {
    dev_err(&client.dev, "Error in setting fuse access mode\n");
    return ret;
    }
// Get asa data and store in the device data.
    ret = i2c_smbus_read_i2c_block_data_or_emulated(client,
    data.def.ctrl_regs[ASA_BASE],
    sizeof(data.asa),
    data.asa);
    if (ret < 0) {
    dev_err(&client.dev, "Not able to read asa data\n");
    return ret;
    }
    if (ret != sizeof(data.asa)) {
    dev_err(&client.dev, "Error reading asa data\n");
    return -EIO;
    }
// After reading fuse ROM data set power-down mode
    ret = ak8975_set_mode(data, POWER_DOWN);
    if (ret < 0) {
    dev_err(&client.dev, "Error in setting power-down mode\n");
    return ret;
    }
    if (data.eoc_gpiod || client.irq > 0) {
    ret = ak8975_setup_irq(data);
    if (ret < 0) {
    dev_err(&client.dev,
    "Error setting data ready interrupt\n");
    return ret;
    }
    }
    data.raw_to_gauss[0] = data.def.raw_to_gauss(data.asa[0]);
    data.raw_to_gauss[1] = data.def.raw_to_gauss(data.asa[1]);
    data.raw_to_gauss[2] = data.def.raw_to_gauss(data.asa[2]);
    return 0;
    }
    static int wait_conversion_complete_gpio(struct ak8975_data *data,
    unsigned int poll_ms,
    unsigned int timeout_ms)
    {
    struct i2c_client *client = data.client;
    int ret;
    int val;
// Wait for the conversion to complete.
    ret = readx_poll_timeout(gpiod_get_value, data.eoc_gpiod, val, val != 0,
    poll_ms * USEC_PER_MSEC,
    timeout_ms * USEC_PER_MSEC);
    if (ret)
    return ret;
    if (val < 0) {
    dev_err(&client.dev, "Error in reading GPIOD\n");
    return val;
    }
    ret = i2c_smbus_read_byte_data(client, data.def.ctrl_regs[ST1]);
    if (ret < 0)
    dev_err(&client.dev, "Error in reading ST1\n");
    return ret;
    }
    static int wait_conversion_complete_polled(struct ak8975_data *data,
    unsigned int poll_ms,
    unsigned int timeout_ms)
    {
    struct i2c_client *client = data.client;
    int ret;
    int val;
// Wait for the conversion to complete.
    ret = read_poll_timeout(i2c_smbus_read_byte_data, val, val != 0,
    poll_ms * USEC_PER_MSEC,
    timeout_ms * USEC_PER_MSEC,
    true,
    client, data.def.ctrl_regs[ST1]);
    if (ret)
    return ret;
    if (val < 0)
    dev_err(&client.dev, "Error in reading ST1\n");
    return val;
    }
// Returns 0 if the end of conversion interrupt occurred or -ETIMEDOUT otherwise
    static int wait_conversion_complete_interrupt(struct ak8975_data *data,
    unsigned int timeout_ms)
    {
    int ret;
    ret = wait_event_timeout(data.data_ready_queue,
    test_bit(0, &data.flags),
    msecs_to_jiffies(timeout_ms));
    clear_bit(0, &data.flags);
    return ret > 0 ? 0 : -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ak8975_start_read_axis(data: *mut ak8975_data) -> c_int {
    static int ak8975_start_read_axis(struct ak8975_data *data)
    {
    struct i2c_client *client = data.client;
    int ret;
// Set up the device for taking a sample.
    ret = ak8975_set_mode(data, MODE_ONCE);
    if (ret < 0) {
    dev_err(&client.dev, "Error in setting operating mode\n");
    return ret;
    }
// Wait for the conversion to complete.
    if (data.eoc_irq)
    ret = wait_conversion_complete_interrupt(data, 100);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: data->eoc_gpiod) -> else {
    else if (data.eoc_gpiod)
    ret = wait_conversion_complete_gpio(data, 10, 500);
    else
    ret = wait_conversion_complete_polled(data, 10, 500);
    if (ret < 0)
    return ret;
// Return with zero if the data is ready.
    return !data.def.ctrl_regs[ST1_DRDY];
    }
// Retrieve raw flux value for one of the x, y, or z axis.
#[no_mangle]
unsafe extern "C" fn ak8975_read_axis(indio_dev: *mut iio_dev, index: c_int, val: *mut c_int) -> c_int {
    static int ak8975_read_axis(struct iio_dev *indio_dev, int index, int *val)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    const struct i2c_client *client = data.client;
    const struct ak_def *def = data.def;
    __le16 rval;
    int ret;
    pm_runtime_get_sync(&data.client.dev);
    mutex_lock(&data.lock);
    ret = ak8975_start_read_axis(data);
    if (ret)
    goto exit;
    ret = i2c_smbus_read_i2c_block_data_or_emulated(client,
    def.data_regs[index],
    sizeof(rval),
    (u8 *)&rval);
    if (ret < 0)
    goto exit;
    if (ret != sizeof(rval)) {
    ret = -EIO;
    goto exit;
    }
// Read out ST2 for release lock on measurement data.
    ret = i2c_smbus_read_byte_data(client, data.def.ctrl_regs[ST2]);
    if (ret < 0) {
    dev_err(&client.dev, "Error in reading ST2\n");
    goto exit;
    }
    if (ret & (data.def.ctrl_masks[ST2_DERR] |
    data.def.ctrl_masks[ST2_HOFL])) {
    dev_err(&client.dev, "ST2 status error 0x%x\n", ret);
    ret = -EINVAL;
    goto exit;
    }
    mutex_unlock(&data.lock);
    pm_runtime_put_autosuspend(&data.client.dev);
// Swap bytes and convert to valid range.
// val = clamp_t(s16, le16_to_cpu(rval), -def->range, def->range);
    return IIO_VAL_INT;
    exit:
    mutex_unlock(&data.lock);
    pm_runtime_put_autosuspend(&data.client.dev);
    dev_err(&client.dev, "Error in reading axis\n");
    return ret;
    }
    static int ak8975_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2,
    long mask)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    return ak8975_read_axis(indio_dev, chan.address, val);
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// val2 = data->raw_to_gauss[chan->address];
    return IIO_VAL_INT_PLUS_MICRO;
    }
    return -EINVAL;
    }
    static const struct iio_mount_matrix *
    ak8975_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    return &data.orientation;
    }
    static const struct iio_chan_spec_ext_info ak8975_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_DIR, ak8975_get_mount_matrix),
    { }
    };

    {								\
    .type = IIO_MAGN,					\
    .modified = 1,						\
    .channel2 = IIO_MOD_##axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .address = index,					\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = 16,					\
    .storagebits = 16,				\
    .endianness = IIO_CPU				\
    },							\
    .ext_info = ak8975_ext_info,				\
    }
    static const struct iio_chan_spec ak8975_channels[] = {
    AK8975_CHANNEL(X, 0), AK8975_CHANNEL(Y, 1), AK8975_CHANNEL(Z, 2),
    IIO_CHAN_SOFT_TIMESTAMP(3),
    };
    static const unsigned long ak8975_scan_masks[] = { 0x7, 0 };
    static const struct iio_info ak8975_info = {
    .read_raw = &ak8975_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ak8975_fill_buffer(indio_dev: *mut iio_dev) {
    static void ak8975_fill_buffer(struct iio_dev *indio_dev)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    const struct i2c_client *client = data.client;
    const struct ak_def *def = data.def;
    int ret;
    __le16 fval[3];
    mutex_lock(&data.lock);
    ret = ak8975_start_read_axis(data);
    if (ret)
    goto unlock;
//
// For each axis, read the flux value from the appropriate register
// (the register is specified in the iio device attributes).
//
    ret = i2c_smbus_read_i2c_block_data_or_emulated(client,
    def.data_regs[0],
    sizeof(fval),
    (u8 *)fval);
    if (ret < 0)
    goto unlock;
    if (ret != sizeof(fval))
    goto unlock;
    mutex_unlock(&data.lock);
// Clamp to valid range.
    data.scan.channels[0] = clamp_t(s16, le16_to_cpu(fval[0]), -def.range, def.range);
    data.scan.channels[1] = clamp_t(s16, le16_to_cpu(fval[1]), -def.range, def.range);
    data.scan.channels[2] = clamp_t(s16, le16_to_cpu(fval[2]), -def.range, def.range);
    iio_push_to_buffers_with_ts(indio_dev, &data.scan, sizeof(data.scan),
    iio_get_time_ns(indio_dev));
    return;
    unlock:
    mutex_unlock(&data.lock);
    dev_err(&client.dev, "Error in reading axes block\n");
    }
#[no_mangle]
unsafe extern "C" fn ak8975_handle_trigger(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ak8975_handle_trigger(int irq, void *p)
    {
    const struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    ak8975_fill_buffer(indio_dev);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ak8975_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int ak8975_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    struct device *dev = &data.client.dev;
    return pm_runtime_resume_and_get(dev);
    }
#[no_mangle]
unsafe extern "C" fn ak8975_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int ak8975_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct ak8975_data *data = iio_priv(indio_dev);
    struct device *dev = &data.client.dev;
    pm_runtime_put_autosuspend(dev);
    return 0;
    }
    static const struct iio_buffer_setup_ops ak8975_buffer_setup_ops = {
    .preenable = ak8975_buffer_preenable,
    .postdisable = ak8975_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn ak8975_probe(client: *mut i2c_client) -> c_int {
    static int ak8975_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct ak8975_data *data;
    struct iio_dev *indio_dev;
    struct gpio_desc *eoc_gpiod;
    struct gpio_desc *reset_gpiod;
    const char *name = core::ptr::null_mut();
    int ret;
//
// Grab and set up the supplied GPIO.
// We may not have a GPIO based IRQ to scan, that is fine, we will
// poll if so.
//
    eoc_gpiod = devm_gpiod_get_optional(&client.dev, core::ptr::null_mut(), GPIOD_IN);
    if (IS_ERR(eoc_gpiod))
    return PTR_ERR(eoc_gpiod);
    gpiod_set_consumer_name(eoc_gpiod, "ak_8975");
//
// According to AK09911 datasheet, if reset GPIO is provided then
// deassert reset on ak8975_power_on() and assert reset on
// ak8975_power_off().
//
    reset_gpiod = devm_gpiod_get_optional(&client.dev,
    "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(reset_gpiod))
    return PTR_ERR(reset_gpiod);
// Register with IIO
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    data.eoc_gpiod = eoc_gpiod;
    data.reset_gpiod = reset_gpiod;
    data.eoc_irq = 0;
    ret = iio_read_mount_matrix(&client.dev, &data.orientation);
    if (ret)
    return ret;
// id will be NULL when enumerated via ACPI
    data.def = i2c_get_match_data(client);
    if (!data.def)
    return -ENODEV;
// If enumerated via firmware node, fix the ABI
    if (dev_fwnode(&client.dev))
    name = dev_name(&client.dev);
    else
    name = id.name;
// Fetch the regulators
    data.vdd = devm_regulator_get(&client.dev, "vdd");
    if (IS_ERR(data.vdd))
    return PTR_ERR(data.vdd);
    data.vid = devm_regulator_get(&client.dev, "vid");
    if (IS_ERR(data.vid))
    return PTR_ERR(data.vid);
    ret = ak8975_power_on(data);
    if (ret)
    return ret;
    ret = ak8975_who_i_am(data, data.def.type);
    if (ret) {
    dev_err(&client.dev, "Unexpected device\n");
    goto power_off;
    }
    dev_dbg(&client.dev, "Asahi compass chip %s\n", name);
// Perform some basic start-of-day setup of the device.
    ret = ak8975_setup(data);
    if (ret) {
    dev_err(&client.dev, "%s initialization fails\n", name);
    goto power_off;
    }
    mutex_init(&data.lock);
    indio_dev.channels = ak8975_channels;
    indio_dev.num_channels = ARRAY_SIZE(ak8975_channels);
    indio_dev.info = &ak8975_info;
    indio_dev.available_scan_masks = ak8975_scan_masks;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.name = name;
    ret = iio_triggered_buffer_setup(indio_dev, core::ptr::null_mut(), ak8975_handle_trigger,
    &ak8975_buffer_setup_ops);
    if (ret) {
    dev_err(&client.dev, "triggered buffer setup failed\n");
    goto power_off;
    }
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&client.dev, "device register failed\n");
    goto cleanup_buffer;
    }
// Enable runtime PM
    pm_runtime_get_noresume(&client.dev);
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
//
// The device comes online in 500us, so add two orders of magnitude
// of delay before autosuspending: 50 ms.
//
    pm_runtime_set_autosuspend_delay(&client.dev, 50);
    pm_runtime_use_autosuspend(&client.dev);
    pm_runtime_put(&client.dev);
    return 0;
    cleanup_buffer:
    iio_triggered_buffer_cleanup(indio_dev);
    power_off:
    ak8975_power_off(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ak8975_remove(client: *mut i2c_client) {
    static void ak8975_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct ak8975_data *data = iio_priv(indio_dev);
    pm_runtime_get_sync(&client.dev);
    pm_runtime_put_noidle(&client.dev);
    pm_runtime_disable(&client.dev);
    iio_device_unregister(indio_dev);
    iio_triggered_buffer_cleanup(indio_dev);
    ak8975_set_mode(data, POWER_DOWN);
    ak8975_power_off(data);
    }
#[no_mangle]
unsafe extern "C" fn ak8975_runtime_suspend(dev: *mut device) -> c_int {
    static int ak8975_runtime_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct ak8975_data *data = iio_priv(indio_dev);
    int ret;
// Set the device in power down if it wasn't already
    ret = ak8975_set_mode(data, POWER_DOWN);
    if (ret < 0) {
    dev_err(&client.dev, "Error in setting power-down mode\n");
    return ret;
    }
// Next cut the regulators
    ak8975_power_off(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak8975_runtime_resume(dev: *mut device) -> c_int {
    static int ak8975_runtime_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct ak8975_data *data = iio_priv(indio_dev);
    int ret;
// Take up the regulators
    ak8975_power_on(data);
//
// We come up in powered down mode, the reading routines will
// put us in the mode to read values later.
//
    ret = ak8975_set_mode(data, POWER_DOWN);
    if (ret < 0) {
    dev_err(&client.dev, "Error in setting power-down mode\n");
    return ret;
    }
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(ak8975_dev_pm_ops, ak8975_runtime_suspend,
    ak8975_runtime_resume, core::ptr::null_mut());
    static const struct acpi_device_id ak_acpi_match[] = {
    {"AK8963", (kernel_ulong_t)&ak_def_array[AK8963] },
    {"AK8975", (kernel_ulong_t)&ak_def_array[AK8975] },
    {"AK009911", (kernel_ulong_t)&ak_def_array[AK09911] },
    {"AK09911", (kernel_ulong_t)&ak_def_array[AK09911] },
    {"AK09912", (kernel_ulong_t)&ak_def_array[AK09912] },
    {"AKM9911", (kernel_ulong_t)&ak_def_array[AK09911] },
    {"INVN6500", (kernel_ulong_t)&ak_def_array[AK8963] },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ak_acpi_match);
    static const struct i2c_device_id ak8975_id[] = {
    { .name = "AK8963", .driver_data = (kernel_ulong_t)&ak_def_array[AK8963] },
    { .name = "ak8963", .driver_data = (kernel_ulong_t)&ak_def_array[AK8963] },
    { .name = "ak8975", .driver_data = (kernel_ulong_t)&ak_def_array[AK8975] },
    { .name = "ak09911", .driver_data = (kernel_ulong_t)&ak_def_array[AK09911] },
    { .name = "ak09912", .driver_data = (kernel_ulong_t)&ak_def_array[AK09912] },
    { .name = "ak09916", .driver_data = (kernel_ulong_t)&ak_def_array[AK09916] },
    { .name = "ak09918", .driver_data = (kernel_ulong_t)&ak_def_array[AK09918] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak8975_id);
    static const struct of_device_id ak8975_of_match[] = {
    { .compatible = "asahi-kasei,ak8975", .data = &ak_def_array[AK8975] },
    { .compatible = "ak8975", .data = &ak_def_array[AK8975] },
    { .compatible = "asahi-kasei,ak8963", .data = &ak_def_array[AK8963] },
    { .compatible = "ak8963", .data = &ak_def_array[AK8963] },
    { .compatible = "asahi-kasei,ak09911", .data = &ak_def_array[AK09911] },
    { .compatible = "ak09911", .data = &ak_def_array[AK09911] },
    { .compatible = "asahi-kasei,ak09912", .data = &ak_def_array[AK09912] },
    { .compatible = "ak09912", .data = &ak_def_array[AK09912] },
    { .compatible = "asahi-kasei,ak09916", .data = &ak_def_array[AK09916] },
    { .compatible = "asahi-kasei,ak09918", .data = &ak_def_array[AK09918] },
    { }
    };
    MODULE_DEVICE_TABLE(of, ak8975_of_match);
    static struct i2c_driver ak8975_driver = {
    .driver = {
    .name	= "ak8975",
    .pm = pm_ptr(&ak8975_dev_pm_ops),
    .of_match_table = ak8975_of_match,
    .acpi_match_table = ak_acpi_match,
    },
    .probe		= ak8975_probe,
    .remove		= ak8975_remove,
    .id_table	= ak8975_id,
    };
    module_i2c_driver(ak8975_driver);
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_DESCRIPTION("AK8975 magnetometer driver");
    MODULE_LICENSE("GPL");
