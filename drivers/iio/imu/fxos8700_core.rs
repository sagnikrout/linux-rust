//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/fxos8700_core.c
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
// FXOS8700 - NXP IMU (accelerometer plus magnetometer)
//
// IIO core driver for FXOS8700, with support for I2C/SPI buses
//
// TODO: Buffer, trigger, and IRQ support
//

// Register Definitions
pub const FXOS8700_STATUS: c_uint = 0x00;
pub const FXOS8700_OUT_X_MSB: c_uint = 0x01;
pub const FXOS8700_OUT_X_LSB: c_uint = 0x02;
pub const FXOS8700_OUT_Y_MSB: c_uint = 0x03;
pub const FXOS8700_OUT_Y_LSB: c_uint = 0x04;
pub const FXOS8700_OUT_Z_MSB: c_uint = 0x05;
pub const FXOS8700_OUT_Z_LSB: c_uint = 0x06;
pub const FXOS8700_F_SETUP: c_uint = 0x09;
pub const FXOS8700_TRIG_CFG: c_uint = 0x0a;
pub const FXOS8700_SYSMOD: c_uint = 0x0b;
pub const FXOS8700_INT_SOURCE: c_uint = 0x0c;
pub const FXOS8700_WHO_AM_I: c_uint = 0x0d;
pub const FXOS8700_XYZ_DATA_CFG: c_uint = 0x0e;
pub const FXOS8700_HP_FILTER_CUTOFF: c_uint = 0x0f;
pub const FXOS8700_PL_STATUS: c_uint = 0x10;
pub const FXOS8700_PL_CFG: c_uint = 0x11;
pub const FXOS8700_PL_COUNT: c_uint = 0x12;
pub const FXOS8700_PL_BF_ZCOMP: c_uint = 0x13;
pub const FXOS8700_PL_THS_REG: c_uint = 0x14;
pub const FXOS8700_A_FFMT_CFG: c_uint = 0x15;
pub const FXOS8700_A_FFMT_SRC: c_uint = 0x16;
pub const FXOS8700_A_FFMT_THS: c_uint = 0x17;
pub const FXOS8700_A_FFMT_COUNT: c_uint = 0x18;
pub const FXOS8700_TRANSIENT_CFG: c_uint = 0x1d;
pub const FXOS8700_TRANSIENT_SRC: c_uint = 0x1e;
pub const FXOS8700_TRANSIENT_THS: c_uint = 0x1f;
pub const FXOS8700_TRANSIENT_COUNT: c_uint = 0x20;
pub const FXOS8700_PULSE_CFG: c_uint = 0x21;
pub const FXOS8700_PULSE_SRC: c_uint = 0x22;
pub const FXOS8700_PULSE_THSX: c_uint = 0x23;
pub const FXOS8700_PULSE_THSY: c_uint = 0x24;
pub const FXOS8700_PULSE_THSZ: c_uint = 0x25;
pub const FXOS8700_PULSE_TMLT: c_uint = 0x26;
pub const FXOS8700_PULSE_LTCY: c_uint = 0x27;
pub const FXOS8700_PULSE_WIND: c_uint = 0x28;
pub const FXOS8700_ASLP_COUNT: c_uint = 0x29;
pub const FXOS8700_CTRL_REG1: c_uint = 0x2a;
pub const FXOS8700_CTRL_REG2: c_uint = 0x2b;
pub const FXOS8700_CTRL_REG3: c_uint = 0x2c;
pub const FXOS8700_CTRL_REG4: c_uint = 0x2d;
pub const FXOS8700_CTRL_REG5: c_uint = 0x2e;
pub const FXOS8700_OFF_X: c_uint = 0x2f;
pub const FXOS8700_OFF_Y: c_uint = 0x30;
pub const FXOS8700_OFF_Z: c_uint = 0x31;
pub const FXOS8700_M_DR_STATUS: c_uint = 0x32;
pub const FXOS8700_M_OUT_X_MSB: c_uint = 0x33;
pub const FXOS8700_M_OUT_X_LSB: c_uint = 0x34;
pub const FXOS8700_M_OUT_Y_MSB: c_uint = 0x35;
pub const FXOS8700_M_OUT_Y_LSB: c_uint = 0x36;
pub const FXOS8700_M_OUT_Z_MSB: c_uint = 0x37;
pub const FXOS8700_M_OUT_Z_LSB: c_uint = 0x38;
pub const FXOS8700_CMP_X_MSB: c_uint = 0x39;
pub const FXOS8700_CMP_X_LSB: c_uint = 0x3a;
pub const FXOS8700_CMP_Y_MSB: c_uint = 0x3b;
pub const FXOS8700_CMP_Y_LSB: c_uint = 0x3c;
pub const FXOS8700_CMP_Z_MSB: c_uint = 0x3d;
pub const FXOS8700_CMP_Z_LSB: c_uint = 0x3e;
pub const FXOS8700_M_OFF_X_MSB: c_uint = 0x3f;
pub const FXOS8700_M_OFF_X_LSB: c_uint = 0x40;
pub const FXOS8700_M_OFF_Y_MSB: c_uint = 0x41;
pub const FXOS8700_M_OFF_Y_LSB: c_uint = 0x42;
pub const FXOS8700_M_OFF_Z_MSB: c_uint = 0x43;
pub const FXOS8700_M_OFF_Z_LSB: c_uint = 0x44;
pub const FXOS8700_MAX_X_MSB: c_uint = 0x45;
pub const FXOS8700_MAX_X_LSB: c_uint = 0x46;
pub const FXOS8700_MAX_Y_MSB: c_uint = 0x47;
pub const FXOS8700_MAX_Y_LSB: c_uint = 0x48;
pub const FXOS8700_MAX_Z_MSB: c_uint = 0x49;
pub const FXOS8700_MAX_Z_LSB: c_uint = 0x4a;
pub const FXOS8700_MIN_X_MSB: c_uint = 0x4b;
pub const FXOS8700_MIN_X_LSB: c_uint = 0x4c;
pub const FXOS8700_MIN_Y_MSB: c_uint = 0x4d;
pub const FXOS8700_MIN_Y_LSB: c_uint = 0x4e;
pub const FXOS8700_MIN_Z_MSB: c_uint = 0x4f;
pub const FXOS8700_MIN_Z_LSB: c_uint = 0x50;
pub const FXOS8700_TEMP: c_uint = 0x51;
pub const FXOS8700_M_THS_CFG: c_uint = 0x52;
pub const FXOS8700_M_THS_SRC: c_uint = 0x53;
pub const FXOS8700_M_THS_X_MSB: c_uint = 0x54;
pub const FXOS8700_M_THS_X_LSB: c_uint = 0x55;
pub const FXOS8700_M_THS_Y_MSB: c_uint = 0x56;
pub const FXOS8700_M_THS_Y_LSB: c_uint = 0x57;
pub const FXOS8700_M_THS_Z_MSB: c_uint = 0x58;
pub const FXOS8700_M_THS_Z_LSB: c_uint = 0x59;
pub const FXOS8700_M_THS_COUNT: c_uint = 0x5a;
pub const FXOS8700_M_CTRL_REG1: c_uint = 0x5b;
pub const FXOS8700_M_CTRL_REG2: c_uint = 0x5c;
pub const FXOS8700_M_CTRL_REG3: c_uint = 0x5d;
pub const FXOS8700_M_INT_SRC: c_uint = 0x5e;
pub const FXOS8700_A_VECM_CFG: c_uint = 0x5f;
pub const FXOS8700_A_VECM_THS_MSB: c_uint = 0x60;
pub const FXOS8700_A_VECM_THS_LSB: c_uint = 0x61;
pub const FXOS8700_A_VECM_CNT: c_uint = 0x62;
pub const FXOS8700_A_VECM_INITX_MSB: c_uint = 0x63;
pub const FXOS8700_A_VECM_INITX_LSB: c_uint = 0x64;
pub const FXOS8700_A_VECM_INITY_MSB: c_uint = 0x65;
pub const FXOS8700_A_VECM_INITY_LSB: c_uint = 0x66;
pub const FXOS8700_A_VECM_INITZ_MSB: c_uint = 0x67;
pub const FXOS8700_A_VECM_INITZ_LSB: c_uint = 0x68;
pub const FXOS8700_M_VECM_CFG: c_uint = 0x69;
pub const FXOS8700_M_VECM_THS_MSB: c_uint = 0x6a;
pub const FXOS8700_M_VECM_THS_LSB: c_uint = 0x6b;
pub const FXOS8700_M_VECM_CNT: c_uint = 0x6c;
pub const FXOS8700_M_VECM_INITX_MSB: c_uint = 0x6d;
pub const FXOS8700_M_VECM_INITX_LSB: c_uint = 0x6e;
pub const FXOS8700_M_VECM_INITY_MSB: c_uint = 0x6f;
pub const FXOS8700_M_VECM_INITY_LSB: c_uint = 0x70;
pub const FXOS8700_M_VECM_INITZ_MSB: c_uint = 0x71;
pub const FXOS8700_M_VECM_INITZ_LSB: c_uint = 0x72;
pub const FXOS8700_A_FFMT_THS_X_MSB: c_uint = 0x73;
pub const FXOS8700_A_FFMT_THS_X_LSB: c_uint = 0x74;
pub const FXOS8700_A_FFMT_THS_Y_MSB: c_uint = 0x75;
pub const FXOS8700_A_FFMT_THS_Y_LSB: c_uint = 0x76;
pub const FXOS8700_A_FFMT_THS_Z_MSB: c_uint = 0x77;
pub const FXOS8700_A_FFMT_THS_Z_LSB: c_uint = 0x78;
pub const FXOS8700_A_TRAN_INIT_MSB: c_uint = 0x79;
pub const FXOS8700_A_TRAN_INIT_LSB_X: c_uint = 0x7a;
pub const FXOS8700_A_TRAN_INIT_LSB_Y: c_uint = 0x7b;
pub const FXOS8700_A_TRAN_INIT_LSB_Z: c_uint = 0x7d;
pub const FXOS8700_TM_NVM_LOCK: c_uint = 0x7e;
pub const FXOS8700_NVM_DATA0_35: c_uint = 0x80;
pub const FXOS8700_NVM_DATA_BNK3: c_uint = 0xa4;
pub const FXOS8700_NVM_DATA_BNK2: c_uint = 0xa5;
pub const FXOS8700_NVM_DATA_BNK1: c_uint = 0xa6;
pub const FXOS8700_NVM_DATA_BNK0: c_uint = 0xa7;
// Bit definitions for FXOS8700_CTRL_REG1
pub const FXOS8700_CTRL_ODR_MAX: c_uint = 0x00;

// Bit definitions for FXOS8700_M_CTRL_REG1

// Bit definitions for FXOS8700_M_CTRL_REG2

pub const FXOS8700_ACTIVE: c_uint = 0x01;

pub const FXOS8700_DEVICE_ID: c_uint = 0xC7;
pub const FXOS8700_PRE_DEVICE_ID: c_uint = 0xC4;
pub const FXOS8700_DATA_BUF_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fxos8700_data {
    pub regmap: *mut regmap,
    pub trig: *mut iio_trigger,
    pub __aligned(IIO_DMA_MINALIGN): __be16 buf[FXOS8700_DATA_BUF_SIZE],
}

// Regmap info
    static const struct regmap_range read_range[] = {
    {
    .range_min = FXOS8700_STATUS,
    .range_max = FXOS8700_A_FFMT_COUNT,
    }, {
    .range_min = FXOS8700_TRANSIENT_CFG,
    .range_max = FXOS8700_A_FFMT_THS_Z_LSB,
    },
    };
    static const struct regmap_range write_range[] = {
    {
    .range_min = FXOS8700_F_SETUP,
    .range_max = FXOS8700_TRIG_CFG,
    }, {
    .range_min = FXOS8700_XYZ_DATA_CFG,
    .range_max = FXOS8700_HP_FILTER_CUTOFF,
    }, {
    .range_min = FXOS8700_PL_CFG,
    .range_max = FXOS8700_A_FFMT_CFG,
    }, {
    .range_min = FXOS8700_A_FFMT_THS,
    .range_max = FXOS8700_TRANSIENT_CFG,
    }, {
    .range_min = FXOS8700_TRANSIENT_THS,
    .range_max = FXOS8700_PULSE_CFG,
    }, {
    .range_min = FXOS8700_PULSE_THSX,
    .range_max = FXOS8700_OFF_Z,
    }, {
    .range_min = FXOS8700_M_OFF_X_MSB,
    .range_max = FXOS8700_M_OFF_Z_LSB,
    }, {
    .range_min = FXOS8700_M_THS_CFG,
    .range_max = FXOS8700_M_THS_CFG,
    }, {
    .range_min = FXOS8700_M_THS_X_MSB,
    .range_max = FXOS8700_M_CTRL_REG3,
    }, {
    .range_min = FXOS8700_A_VECM_CFG,
    .range_max = FXOS8700_A_FFMT_THS_Z_LSB,
    },
    };
    static const struct regmap_access_table driver_read_table = {
    .yes_ranges =   read_range,
    .n_yes_ranges = ARRAY_SIZE(read_range),
    };
    static const struct regmap_access_table driver_write_table = {
    .yes_ranges =   write_range,
    .n_yes_ranges = ARRAY_SIZE(write_range),
    };
    const struct regmap_config fxos8700_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = FXOS8700_NVM_DATA_BNK0,
    .rd_table = &driver_read_table,
    .wr_table = &driver_write_table,
    };
    EXPORT_SYMBOL(fxos8700_regmap_config);

    .type = _type,						\
    .modified = 1,						\
    .channel2 = IIO_MOD_##_axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |  \
    BIT(IIO_CHAN_INFO_SAMP_FREQ),			\
    }
    enum fxos8700_accel_scale_bits {
    MODE_2G = 0,
    MODE_4G,
    MODE_8G,
    };
// scan indexes follow DATA register order
    enum fxos8700_scan_axis {
    FXOS8700_SCAN_ACCEL_X = 0,
    FXOS8700_SCAN_ACCEL_Y,
    FXOS8700_SCAN_ACCEL_Z,
    FXOS8700_SCAN_MAGN_X,
    FXOS8700_SCAN_MAGN_Y,
    FXOS8700_SCAN_MAGN_Z,
    FXOS8700_SCAN_RHALL,
    FXOS8700_SCAN_TIMESTAMP,
    };
    enum fxos8700_sensor {
    FXOS8700_ACCEL	= 0,
    FXOS8700_MAGN,
    FXOS8700_NUM_SENSORS /* must be last */
    };
    enum fxos8700_int_pin {
    FXOS8700_PIN_INT1,
    FXOS8700_PIN_INT2
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fxos8700_scale {
    pub bits: u8,
    pub uscale: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fxos8700_odr {
    pub bits: u8,
    pub odr: c_int,
    pub uodr: c_int,
}

    static const struct fxos8700_scale fxos8700_accel_scale[] = {
    { MODE_2G, 244},
    { MODE_4G, 488},
    { MODE_8G, 976},
    };
//
// Accellerometer and magnetometer have the same ODR options, set in the
// CTRL_REG1 register. ODR is halved when using both sensors at once in
// hybrid mode.
//
    static const struct fxos8700_odr fxos8700_odr[] = {
    {0x00, 800, 0},
    {0x01, 400, 0},
    {0x02, 200, 0},
    {0x03, 100, 0},
    {0x04, 50, 0},
    {0x05, 12, 500000},
    {0x06, 6, 250000},
    {0x07, 1, 562500},
    };
    static const struct iio_chan_spec fxos8700_channels[] = {
    FXOS8700_CHANNEL(IIO_ACCEL, X),
    FXOS8700_CHANNEL(IIO_ACCEL, Y),
    FXOS8700_CHANNEL(IIO_ACCEL, Z),
    FXOS8700_CHANNEL(IIO_MAGN, X),
    FXOS8700_CHANNEL(IIO_MAGN, Y),
    FXOS8700_CHANNEL(IIO_MAGN, Z),
    IIO_CHAN_SOFT_TIMESTAMP(FXOS8700_SCAN_TIMESTAMP),
    };
#[no_mangle]
unsafe extern "C" fn fxos8700_to_sensor(iio_type: enum iio_chan_type) -> enum fxos8700_sensor {
    static enum fxos8700_sensor fxos8700_to_sensor(enum iio_chan_type iio_type)
    {
    switch (iio_type) {
    case IIO_ACCEL:
    return FXOS8700_ACCEL;
    case IIO_MAGN:
    return FXOS8700_MAGN;
    default:
    return -EINVAL;
    }
    }
    static int fxos8700_set_active_mode(struct fxos8700_data *data,
    enum fxos8700_sensor t, bool mode)
    {
    int ret;
    ret = regmap_write(data.regmap, FXOS8700_CTRL_REG1, mode);
    if (ret)
    return ret;
    usleep_range(FXOS8700_ACTIVE_MIN_USLEEP,
    FXOS8700_ACTIVE_MIN_USLEEP + 1000);
    return 0;
    }
    static int fxos8700_set_scale(struct fxos8700_data *data,
    enum fxos8700_sensor t, int uscale)
    {
    int i, ret, val;
    bool active_mode;
    let mut scale_num: static int = ARRAY_SIZE(fxos8700_accel_scale);
    struct device *dev = regmap_get_device(data.regmap);
    if (t == FXOS8700_MAGN) {
    dev_err(dev, "Magnetometer scale is locked at 0.001Gs\n");
    return -EINVAL;
    }
//
// When device is in active mode, it failed to set an ACCEL
// full-scale range(2g/4g/8g) in FXOS8700_XYZ_DATA_CFG.
// This is not align with the datasheet, but it is a fxos8700
// chip behavier. Set the device in standby mode before setting
// an ACCEL full-scale range.
//
    ret = regmap_read(data.regmap, FXOS8700_CTRL_REG1, &val);
    if (ret)
    return ret;
    active_mode = val & FXOS8700_ACTIVE;
    if (active_mode) {
    ret = regmap_write(data.regmap, FXOS8700_CTRL_REG1,
    val & ~FXOS8700_ACTIVE);
    if (ret)
    return ret;
    }
    for (i = 0; i < scale_num; i++)
    if (fxos8700_accel_scale[i].uscale == uscale)
    break;
    if (i == scale_num)
    return -EINVAL;
    ret = regmap_write(data.regmap, FXOS8700_XYZ_DATA_CFG,
    fxos8700_accel_scale[i].bits);
    if (ret)
    return ret;
    return regmap_write(data.regmap, FXOS8700_CTRL_REG1,
    active_mode);
    }
    static int fxos8700_get_scale(struct fxos8700_data *data,
    enum fxos8700_sensor t, int *uscale)
    {
    int i, ret, val;
    let mut scale_num: static int = ARRAY_SIZE(fxos8700_accel_scale);
    if (t == FXOS8700_MAGN) {
// uscale = 1000; /* Magnetometer is locked at 0.001Gs
    return 0;
    }
    ret = regmap_read(data.regmap, FXOS8700_XYZ_DATA_CFG, &val);
    if (ret)
    return ret;
    for (i = 0; i < scale_num; i++) {
    if (fxos8700_accel_scale[i].bits == (val & 0x3)) {
// uscale = fxos8700_accel_scale[i].uscale;
    return 0;
    }
    }
    return -EINVAL;
    }
    static int fxos8700_get_data(struct fxos8700_data *data, int chan_type,
    int axis, int *val)
    {
    u8 base, reg;
    s16 tmp;
    int ret;
//
// Different register base addresses varies with channel types.
// This bug hasn't been noticed before because using an enum is
// really hard to read. Use an a switch statement to take over that.
//
    switch (chan_type) {
    case IIO_ACCEL:
    base = FXOS8700_OUT_X_MSB;
    break;
    case IIO_MAGN:
    base = FXOS8700_M_OUT_X_MSB;
    break;
    default:
    return -EINVAL;
    }
// Block read 6 bytes of device output registers to avoid data loss
    ret = regmap_bulk_read(data.regmap, base, data.buf,
    sizeof(data.buf));
    if (ret)
    return ret;
// Convert axis to buffer index
    reg = axis - IIO_MOD_X;
//
// Convert to native endianness. The accel data and magn data
// are signed, so a forced type conversion is needed.
//
    tmp = be16_to_cpu(data.buf[reg]);
//
// ACCEL output data registers contain the X-axis, Y-axis, and Z-axis
// 14-bit left-justified sample data and MAGN output data registers
// contain the X-axis, Y-axis, and Z-axis 16-bit sample data. Apply
// a signed 2 bits right shift to the readback raw data from ACCEL
// output data register and keep that from MAGN sensor as the origin.
// Value should be extended to 32 bit.
//
    switch (chan_type) {
    case IIO_ACCEL:
    tmp = tmp >> 2;
    break;
    case IIO_MAGN:
// Nothing to do
    break;
    default:
    return -EINVAL;
    }
// Convert to native endianness
// val = sign_extend32(tmp, 15);
    return 0;
    }
    static int fxos8700_set_odr(struct fxos8700_data *data, enum fxos8700_sensor t,
    int odr, int uodr)
    {
    int i, ret, val;
    bool active_mode;
    let mut odr_num: static int = ARRAY_SIZE(fxos8700_odr);
    ret = regmap_read(data.regmap, FXOS8700_CTRL_REG1, &val);
    if (ret)
    return ret;
    active_mode = val & FXOS8700_ACTIVE;
    if (active_mode) {
//
// The device must be in standby mode to change any of the
// other fields within CTRL_REG1
//
    ret = regmap_write(data.regmap, FXOS8700_CTRL_REG1,
    val & ~FXOS8700_ACTIVE);
    if (ret)
    return ret;
    }
    for (i = 0; i < odr_num; i++)
    if (fxos8700_odr[i].odr == odr && fxos8700_odr[i].uodr == uodr)
    break;
    if (i >= odr_num)
    return -EINVAL;
    val &= ~FXOS8700_CTRL_ODR_MSK;
    val |= FIELD_PREP(FXOS8700_CTRL_ODR_MSK, fxos8700_odr[i].bits) | FXOS8700_ACTIVE;
    return regmap_write(data.regmap, FXOS8700_CTRL_REG1, val);
    }
    static int fxos8700_get_odr(struct fxos8700_data *data, enum fxos8700_sensor t,
    int *odr, int *uodr)
    {
    int i, val, ret;
    let mut odr_num: static int = ARRAY_SIZE(fxos8700_odr);
    ret = regmap_read(data.regmap, FXOS8700_CTRL_REG1, &val);
    if (ret)
    return ret;
    val = FIELD_GET(FXOS8700_CTRL_ODR_MSK, val);
    for (i = 0; i < odr_num; i++)
    if (val == fxos8700_odr[i].bits)
    break;
    if (i >= odr_num)
    return -EINVAL;
// odr = fxos8700_odr[i].odr;
// uodr = fxos8700_odr[i].uodr;
    return 0;
    }
    static int fxos8700_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    struct fxos8700_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = fxos8700_get_data(data, chan.type, chan.channel2, val);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
    ret = fxos8700_get_scale(data, fxos8700_to_sensor(chan.type),
    val2);
    return ret ? ret : IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = fxos8700_get_odr(data, fxos8700_to_sensor(chan.type),
    val, val2);
    return ret ? ret : IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int fxos8700_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct fxos8700_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    return fxos8700_set_scale(data, fxos8700_to_sensor(chan.type),
    val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return fxos8700_set_odr(data, fxos8700_to_sensor(chan.type),
    val, val2);
    default:
    return -EINVAL;
    }
    }
    static IIO_CONST_ATTR(in_accel_sampling_frequency_available,
    "1.5625 6.25 12.5 50 100 200 400 800");
    static IIO_CONST_ATTR(in_magn_sampling_frequency_available,
    "1.5625 6.25 12.5 50 100 200 400 800");
    static IIO_CONST_ATTR(in_accel_scale_available, "0.000244 0.000488 0.000976");
    static IIO_CONST_ATTR(in_magn_scale_available, "0.001000");
    static struct attribute *fxos8700_attrs[] = {
    &iio_const_attr_in_accel_sampling_frequency_available.dev_attr.attr,
    &iio_const_attr_in_magn_sampling_frequency_available.dev_attr.attr,
    &iio_const_attr_in_accel_scale_available.dev_attr.attr,
    &iio_const_attr_in_magn_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fxos8700_attrs_group = {
    .attrs = fxos8700_attrs,
    };
    static const struct iio_info fxos8700_info = {
    .read_raw = fxos8700_read_raw,
    .write_raw = fxos8700_write_raw,
    .attrs = &fxos8700_attrs_group,
    };
#[no_mangle]
unsafe extern "C" fn fxos8700_chip_init(data: *mut fxos8700_data, use_spi: bool) -> c_int {
    static int fxos8700_chip_init(struct fxos8700_data *data, bool use_spi)
    {
    int ret;
    unsigned int val;
    struct device *dev = regmap_get_device(data.regmap);
    ret = regmap_read(data.regmap, FXOS8700_WHO_AM_I, &val);
    if (ret) {
    dev_err(dev, "Error reading chip id\n");
    return ret;
    }
    if (val != FXOS8700_DEVICE_ID && val != FXOS8700_PRE_DEVICE_ID) {
    dev_err(dev, "Wrong chip id, got %x expected %x or %x\n",
    val, FXOS8700_DEVICE_ID, FXOS8700_PRE_DEVICE_ID);
    return -ENODEV;
    }
    ret = fxos8700_set_active_mode(data, FXOS8700_ACCEL, true);
    if (ret)
    return ret;
    ret = fxos8700_set_active_mode(data, FXOS8700_MAGN, true);
    if (ret)
    return ret;
//
// The device must be in standby mode to change any of the other fields
// within CTRL_REG1
//
    ret = regmap_write(data.regmap, FXOS8700_CTRL_REG1, 0x00);
    if (ret)
    return ret;
// Set max oversample ratio (OSR) and both devices active
    ret = regmap_write(data.regmap, FXOS8700_M_CTRL_REG1,
    FXOS8700_HMS_MASK | FXOS8700_OS_MASK);
    if (ret)
    return ret;
// Disable and rst min/max measurements & threshold
    ret = regmap_write(data.regmap, FXOS8700_M_CTRL_REG2,
    FXOS8700_MAXMIN_RST | FXOS8700_MAXMIN_DIS_THS |
    FXOS8700_MAXMIN_DIS);
    if (ret)
    return ret;
//
// Set max full-scale range (+/-8G) for ACCEL sensor in chip
// initialization then activate the device.
//
    ret = regmap_write(data.regmap, FXOS8700_XYZ_DATA_CFG, MODE_8G);
    if (ret)
    return ret;
// Max ODR (800Hz individual or 400Hz hybrid), active mode
    return regmap_update_bits(data.regmap, FXOS8700_CTRL_REG1,
    FXOS8700_CTRL_ODR_MSK | FXOS8700_ACTIVE,
    FIELD_PREP(FXOS8700_CTRL_ODR_MSK, FXOS8700_CTRL_ODR_MAX) |
    FXOS8700_ACTIVE);
    }
#[no_mangle]
unsafe extern "C" fn fxos8700_chip_uninit(data: *mut c_void) {
    static void fxos8700_chip_uninit(void *data)
    {
    struct fxos8700_data *fxos8700_data = data;
    fxos8700_set_active_mode(fxos8700_data, FXOS8700_ACCEL, false);
    fxos8700_set_active_mode(fxos8700_data, FXOS8700_MAGN, false);
    }
    int fxos8700_core_probe(struct device *dev, struct regmap *regmap,
    const char *name, bool use_spi)
    {
    struct iio_dev *indio_dev;
    struct fxos8700_data *data;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    dev_set_drvdata(dev, indio_dev);
    data.regmap = regmap;
    ret = fxos8700_chip_init(data, use_spi);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, fxos8700_chip_uninit, data);
    if (ret)
    return ret;
    indio_dev.channels = fxos8700_channels;
    indio_dev.num_channels = ARRAY_SIZE(fxos8700_channels);
    indio_dev.name = name ? name : "fxos8700";
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &fxos8700_info;
    return devm_iio_device_register(dev, indio_dev);
    }
    EXPORT_SYMBOL_GPL(fxos8700_core_probe);
    MODULE_AUTHOR("Robert Jones <rjones@gateworks.com>");
    MODULE_DESCRIPTION("FXOS8700 6-Axis Acc and Mag Combo Sensor driver");
    MODULE_LICENSE("GPL v2");
