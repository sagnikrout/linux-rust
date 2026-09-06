//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bmi160/bmi160_core.c
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
// BMI160 - Bosch IMU (accel, gyro plus external magnetometer)
//
// Copyright (c) 2016, Intel Corporation.
// Copyright (c) 2019, Martin Kelly.
//
// IIO core driver for BMI160, with support for I2C/SPI buses
//
// TODO: magnetometer, hardware FIFO
//

pub const BMI160_REG_CHIP_ID: c_uint = 0x00;
pub const BMI120_CHIP_ID_VAL: c_uint = 0xD3;
pub const BMI160_CHIP_ID_VAL: c_uint = 0xD1;
pub const BMI160_REG_PMU_STATUS: c_uint = 0x03;
// X axis data low byte address, the rest can be obtained using axis offset
pub const BMI160_REG_DATA_MAGN_XOUT_L: c_uint = 0x04;
pub const BMI160_REG_DATA_GYRO_XOUT_L: c_uint = 0x0C;
pub const BMI160_REG_DATA_ACCEL_XOUT_L: c_uint = 0x12;
pub const BMI160_REG_ACCEL_CONFIG: c_uint = 0x40;

pub const BMI160_REG_ACCEL_RANGE: c_uint = 0x41;
pub const BMI160_ACCEL_RANGE_2G: c_uint = 0x03;
pub const BMI160_ACCEL_RANGE_4G: c_uint = 0x05;
pub const BMI160_ACCEL_RANGE_8G: c_uint = 0x08;
pub const BMI160_ACCEL_RANGE_16G: c_uint = 0x0C;
pub const BMI160_REG_GYRO_CONFIG: c_uint = 0x42;

pub const BMI160_REG_GYRO_RANGE: c_uint = 0x43;
pub const BMI160_GYRO_RANGE_2000DPS: c_uint = 0x00;
pub const BMI160_GYRO_RANGE_1000DPS: c_uint = 0x01;
pub const BMI160_GYRO_RANGE_500DPS: c_uint = 0x02;
pub const BMI160_GYRO_RANGE_250DPS: c_uint = 0x03;
pub const BMI160_GYRO_RANGE_125DPS: c_uint = 0x04;
pub const BMI160_REG_CMD: c_uint = 0x7E;
pub const BMI160_CMD_ACCEL_PM_SUSPEND: c_uint = 0x10;
pub const BMI160_CMD_ACCEL_PM_NORMAL: c_uint = 0x11;
pub const BMI160_CMD_ACCEL_PM_LOW_POWER: c_uint = 0x12;
pub const BMI160_CMD_GYRO_PM_SUSPEND: c_uint = 0x14;
pub const BMI160_CMD_GYRO_PM_NORMAL: c_uint = 0x15;
pub const BMI160_CMD_GYRO_PM_FAST_STARTUP: c_uint = 0x17;
pub const BMI160_CMD_SOFTRESET: c_uint = 0xB6;
pub const BMI160_REG_INT_EN: c_uint = 0x51;

pub const BMI160_REG_INT_OUT_CTRL: c_uint = 0x53;
pub const BMI160_INT_OUT_CTRL_MASK: c_uint = 0x0f;
pub const BMI160_INT1_OUT_CTRL_SHIFT: c_int = 0;
pub const BMI160_INT2_OUT_CTRL_SHIFT: c_int = 4;

pub const BMI160_REG_INT_LATCH: c_uint = 0x54;

// INT1 and INT2 are in the opposite order as in INT_OUT_CTRL!
pub const BMI160_REG_INT_MAP: c_uint = 0x56;
pub const BMI160_INT1_MAP_DRDY_EN: c_uint = 0x80;
pub const BMI160_INT2_MAP_DRDY_EN: c_uint = 0x08;
pub const BMI160_REG_DUMMY: c_uint = 0x7F;
pub const BMI160_NORMAL_WRITE_USLEEP: c_int = 2;
pub const BMI160_SUSPENDED_WRITE_USLEEP: c_int = 450;
pub const BMI160_ACCEL_PMU_MIN_USLEEP: c_int = 3800;
pub const BMI160_GYRO_PMU_MIN_USLEEP: c_int = 80000;
pub const BMI160_SOFTRESET_USLEEP: c_int = 1000;

    .type = _type,						\
    .modified = 1,						\
    .channel2 = IIO_MOD_##_axis,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |  \
    BIT(IIO_CHAN_INFO_SAMP_FREQ),			\
    .scan_index = _index,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = 16,					\
    .storagebits = 16,				\
    .endianness = IIO_LE,				\
    },							\
    .ext_info = bmi160_ext_info,				\
    }
    static const u8 bmi_chip_ids[] = {
    BMI120_CHIP_ID_VAL,
    BMI160_CHIP_ID_VAL,
    };
// scan indexes follow DATA register order
    enum bmi160_scan_axis {
    BMI160_SCAN_EXT_MAGN_X = 0,
    BMI160_SCAN_EXT_MAGN_Y,
    BMI160_SCAN_EXT_MAGN_Z,
    BMI160_SCAN_RHALL,
    BMI160_SCAN_GYRO_X,
    BMI160_SCAN_GYRO_Y,
    BMI160_SCAN_GYRO_Z,
    BMI160_SCAN_ACCEL_X,
    BMI160_SCAN_ACCEL_Y,
    BMI160_SCAN_ACCEL_Z,
    BMI160_SCAN_TIMESTAMP,
    };
    enum bmi160_sensor_type {
    BMI160_ACCEL	= 0,
    BMI160_GYRO,
    BMI160_EXT_MAGN,
    BMI160_NUM_SENSORS /* must be last */
    };
    enum bmi160_int_pin {
    BMI160_PIN_INT1,
    BMI160_PIN_INT2
    };
    const struct regmap_config bmi160_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    EXPORT_SYMBOL_NS(bmi160_regmap_config, "IIO_BMI160");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi160_regs {
    pub /: *mut *mut u8 data; / LSB byte register for X-axis,
    pub config: u8,
    pub config_odr_mask: u8,
    pub config_bwp_mask: u8,
    pub range: u8,
    pub pmu_cmd_normal: u8,
    pub pmu_cmd_suspend: u8,
}

    static const struct bmi160_regs bmi160_regs[] = {
    [BMI160_ACCEL] = {
    .data	= BMI160_REG_DATA_ACCEL_XOUT_L,
    .config	= BMI160_REG_ACCEL_CONFIG,
    .config_odr_mask = BMI160_ACCEL_CONFIG_ODR_MASK,
    .config_bwp_mask = BMI160_ACCEL_CONFIG_BWP_MASK,
    .range	= BMI160_REG_ACCEL_RANGE,
    .pmu_cmd_normal = BMI160_CMD_ACCEL_PM_NORMAL,
    .pmu_cmd_suspend = BMI160_CMD_ACCEL_PM_SUSPEND,
    },
    [BMI160_GYRO] = {
    .data	= BMI160_REG_DATA_GYRO_XOUT_L,
    .config	= BMI160_REG_GYRO_CONFIG,
    .config_odr_mask = BMI160_GYRO_CONFIG_ODR_MASK,
    .config_bwp_mask = BMI160_GYRO_CONFIG_BWP_MASK,
    .range	= BMI160_REG_GYRO_RANGE,
    .pmu_cmd_normal = BMI160_CMD_GYRO_PM_NORMAL,
    .pmu_cmd_suspend = BMI160_CMD_GYRO_PM_SUSPEND,
    },
    };
    static unsigned long bmi160_pmu_time[] = {
    [BMI160_ACCEL] = BMI160_ACCEL_PMU_MIN_USLEEP,
    [BMI160_GYRO] = BMI160_GYRO_PMU_MIN_USLEEP,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi160_scale {
    pub bits: u8,
    pub uscale: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi160_odr {
    pub bits: u8,
    pub odr: c_int,
    pub uodr: c_int,
}

    static const struct bmi160_scale bmi160_accel_scale[] = {
    { BMI160_ACCEL_RANGE_2G, 598},
    { BMI160_ACCEL_RANGE_4G, 1197},
    { BMI160_ACCEL_RANGE_8G, 2394},
    { BMI160_ACCEL_RANGE_16G, 4788},
    };
    static const struct bmi160_scale bmi160_gyro_scale[] = {
    { BMI160_GYRO_RANGE_2000DPS, 1065},
    { BMI160_GYRO_RANGE_1000DPS, 532},
    { BMI160_GYRO_RANGE_500DPS, 266},
    { BMI160_GYRO_RANGE_250DPS, 133},
    { BMI160_GYRO_RANGE_125DPS, 66},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi160_scale_item {
    pub tbl: *const bmi160_scale,
    pub num: c_int,
}

    static const struct  bmi160_scale_item bmi160_scale_table[] = {
    [BMI160_ACCEL] = {
    .tbl	= bmi160_accel_scale,
    .num	= ARRAY_SIZE(bmi160_accel_scale),
    },
    [BMI160_GYRO] = {
    .tbl	= bmi160_gyro_scale,
    .num	= ARRAY_SIZE(bmi160_gyro_scale),
    },
    };
    static const struct bmi160_odr bmi160_accel_odr[] = {
    {0x01, 0, 781250},
    {0x02, 1, 562500},
    {0x03, 3, 125000},
    {0x04, 6, 250000},
    {0x05, 12, 500000},
    {0x06, 25, 0},
    {0x07, 50, 0},
    {0x08, 100, 0},
    {0x09, 200, 0},
    {0x0A, 400, 0},
    {0x0B, 800, 0},
    {0x0C, 1600, 0},
    };
    static const struct bmi160_odr bmi160_gyro_odr[] = {
    {0x06, 25, 0},
    {0x07, 50, 0},
    {0x08, 100, 0},
    {0x09, 200, 0},
    {0x0A, 400, 0},
    {0x0B, 800, 0},
    {0x0C, 1600, 0},
    {0x0D, 3200, 0},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi160_odr_item {
    pub tbl: *const bmi160_odr,
    pub num: c_int,
}

    static const struct  bmi160_odr_item bmi160_odr_table[] = {
    [BMI160_ACCEL] = {
    .tbl	= bmi160_accel_odr,
    .num	= ARRAY_SIZE(bmi160_accel_odr),
    },
    [BMI160_GYRO] = {
    .tbl	= bmi160_gyro_odr,
    .num	= ARRAY_SIZE(bmi160_gyro_odr),
    },
    };
    static const struct iio_mount_matrix *
    bmi160_get_mount_matrix(const struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct bmi160_data *data = iio_priv(indio_dev);
    return &data.orientation;
    }
    static const struct iio_chan_spec_ext_info bmi160_ext_info[] = {
    IIO_MOUNT_MATRIX(IIO_SHARED_BY_DIR, bmi160_get_mount_matrix),
    { }
    };
    static const struct iio_chan_spec bmi160_channels[] = {
    BMI160_CHANNEL(IIO_ACCEL, X, BMI160_SCAN_ACCEL_X),
    BMI160_CHANNEL(IIO_ACCEL, Y, BMI160_SCAN_ACCEL_Y),
    BMI160_CHANNEL(IIO_ACCEL, Z, BMI160_SCAN_ACCEL_Z),
    BMI160_CHANNEL(IIO_ANGL_VEL, X, BMI160_SCAN_GYRO_X),
    BMI160_CHANNEL(IIO_ANGL_VEL, Y, BMI160_SCAN_GYRO_Y),
    BMI160_CHANNEL(IIO_ANGL_VEL, Z, BMI160_SCAN_GYRO_Z),
    IIO_CHAN_SOFT_TIMESTAMP(BMI160_SCAN_TIMESTAMP),
    };
#[no_mangle]
unsafe extern "C" fn bmi160_to_sensor(iio_type: enum iio_chan_type) -> enum bmi160_sensor_type {
    static enum bmi160_sensor_type bmi160_to_sensor(enum iio_chan_type iio_type)
    {
    switch (iio_type) {
    case IIO_ACCEL:
    return BMI160_ACCEL;
    case IIO_ANGL_VEL:
    return BMI160_GYRO;
    default:
    return -EINVAL;
    }
    }
    static
    int bmi160_set_mode(struct bmi160_data *data, enum bmi160_sensor_type t,
    bool mode)
    {
    int ret;
    u8 cmd;
    if (mode)
    cmd = bmi160_regs[t].pmu_cmd_normal;
    else
    cmd = bmi160_regs[t].pmu_cmd_suspend;
    ret = regmap_write(data.regmap, BMI160_REG_CMD, cmd);
    if (ret)
    return ret;
    usleep_range(bmi160_pmu_time[t], bmi160_pmu_time[t] + 1000);
    return 0;
    }
    static
    int bmi160_set_scale(struct bmi160_data *data, enum bmi160_sensor_type t,
    int uscale)
    {
    int i;
    for (i = 0; i < bmi160_scale_table[t].num; i++)
    if (bmi160_scale_table[t].tbl[i].uscale == uscale)
    break;
    if (i == bmi160_scale_table[t].num)
    return -EINVAL;
    return regmap_write(data.regmap, bmi160_regs[t].range,
    bmi160_scale_table[t].tbl[i].bits);
    }
    static
    int bmi160_get_scale(struct bmi160_data *data, enum bmi160_sensor_type t,
    int *uscale)
    {
    int i, ret, val;
    ret = regmap_read(data.regmap, bmi160_regs[t].range, &val);
    if (ret)
    return ret;
    for (i = 0; i < bmi160_scale_table[t].num; i++)
    if (bmi160_scale_table[t].tbl[i].bits == val) {
// uscale = bmi160_scale_table[t].tbl[i].uscale;
    return 0;
    }
    return -EINVAL;
    }
    static int bmi160_get_data(struct bmi160_data *data, int chan_type,
    int axis, int *val)
    {
    u8 reg;
    int ret;
    __le16 sample;
    let mut t: enum bmi160_sensor_type = bmi160_to_sensor(chan_type);
    reg = bmi160_regs[t].data + (axis - IIO_MOD_X) * sizeof(sample);
    ret = regmap_bulk_read(data.regmap, reg, &sample, sizeof(sample));
    if (ret)
    return ret;
// val = sign_extend32(le16_to_cpu(sample), 15);
    return 0;
    }
    static
    int bmi160_set_odr(struct bmi160_data *data, enum bmi160_sensor_type t,
    int odr, int uodr)
    {
    int i;
    for (i = 0; i < bmi160_odr_table[t].num; i++)
    if (bmi160_odr_table[t].tbl[i].odr == odr &&
    bmi160_odr_table[t].tbl[i].uodr == uodr)
    break;
    if (i >= bmi160_odr_table[t].num)
    return -EINVAL;
    return regmap_update_bits(data.regmap,
    bmi160_regs[t].config,
    bmi160_regs[t].config_odr_mask,
    bmi160_odr_table[t].tbl[i].bits);
    }
    static int bmi160_get_odr(struct bmi160_data *data, enum bmi160_sensor_type t,
    int *odr, int *uodr)
    {
    int i, val, ret;
    ret = regmap_read(data.regmap, bmi160_regs[t].config, &val);
    if (ret)
    return ret;
    val &= bmi160_regs[t].config_odr_mask;
    for (i = 0; i < bmi160_odr_table[t].num; i++)
    if (val == bmi160_odr_table[t].tbl[i].bits)
    break;
    if (i >= bmi160_odr_table[t].num)
    return -EINVAL;
// odr = bmi160_odr_table[t].tbl[i].odr;
// uodr = bmi160_odr_table[t].tbl[i].uodr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bmi160_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t bmi160_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct bmi160_data *data = iio_priv(indio_dev);
    int i, ret, j = 0, base = BMI160_REG_DATA_MAGN_XOUT_L;
    __le16 sample;
    iio_for_each_active_channel(indio_dev, i) {
    ret = regmap_bulk_read(data.regmap, base + i * sizeof(sample),
    &sample, sizeof(sample));
    if (ret)
    goto done;
    data.buf[j++] = sample;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, data.buf, pf.timestamp);
    done:
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static int bmi160_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    int ret;
    struct bmi160_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = bmi160_get_data(data, chan.type, chan.channel2, val);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
    ret = bmi160_get_scale(data,
    bmi160_to_sensor(chan.type), val2);
    return ret ? ret : IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_SAMP_FREQ:
    ret = bmi160_get_odr(data, bmi160_to_sensor(chan.type),
    val, val2);
    return ret ? ret : IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int bmi160_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct bmi160_data *data = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_SCALE:
    return bmi160_set_scale(data,
    bmi160_to_sensor(chan.type), val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    return bmi160_set_odr(data, bmi160_to_sensor(chan.type),
    val, val2);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static
    IIO_CONST_ATTR(in_accel_sampling_frequency_available,
    "0.78125 1.5625 3.125 6.25 12.5 25 50 100 200 400 800 1600");
    static
    IIO_CONST_ATTR(in_anglvel_sampling_frequency_available,
    "25 50 100 200 400 800 1600 3200");
    static
    IIO_CONST_ATTR(in_accel_scale_available,
    "0.000598 0.001197 0.002394 0.004788");
    static
    IIO_CONST_ATTR(in_anglvel_scale_available,
    "0.001065 0.000532 0.000266 0.000133 0.000066");
    static struct attribute *bmi160_attrs[] = {
    &iio_const_attr_in_accel_sampling_frequency_available.dev_attr.attr,
    &iio_const_attr_in_anglvel_sampling_frequency_available.dev_attr.attr,
    &iio_const_attr_in_accel_scale_available.dev_attr.attr,
    &iio_const_attr_in_anglvel_scale_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group bmi160_attrs_group = {
    .attrs = bmi160_attrs,
    };
    static const struct iio_info bmi160_info = {
    .read_raw = bmi160_read_raw,
    .write_raw = bmi160_write_raw,
    .attrs = &bmi160_attrs_group,
    };
    static int bmi160_write_conf_reg(struct regmap *regmap, unsigned int reg,
    unsigned int mask, unsigned int bits,
    unsigned int write_usleep)
    {
    int ret;
    unsigned int val;
    ret = regmap_read(regmap, reg, &val);
    if (ret)
    return ret;
    val = (val & ~mask) | bits;
    ret = regmap_write(regmap, reg, val);
    if (ret)
    return ret;
//
// We need to wait after writing before we can write again. See the
// datasheet, page 93.
//
    usleep_range(write_usleep, write_usleep + 1000);
    return 0;
    }
    static int bmi160_config_pin(struct regmap *regmap, enum bmi160_int_pin pin,
    bool open_drain, u8 irq_mask,
    unsigned long write_usleep)
    {
    int ret;
    struct device *dev = regmap_get_device(regmap);
    u8 int_out_ctrl_shift;
    u8 int_latch_mask;
    u8 int_map_mask;
    u8 int_out_ctrl_mask;
    u8 int_out_ctrl_bits;
    const char *pin_name;
    switch (pin) {
    case BMI160_PIN_INT1:
    int_out_ctrl_shift = BMI160_INT1_OUT_CTRL_SHIFT;
    int_latch_mask = BMI160_INT1_LATCH_MASK;
    int_map_mask = BMI160_INT1_MAP_DRDY_EN;
    pin_name = "INT1";
    break;
    case BMI160_PIN_INT2:
    int_out_ctrl_shift = BMI160_INT2_OUT_CTRL_SHIFT;
    int_latch_mask = BMI160_INT2_LATCH_MASK;
    int_map_mask = BMI160_INT2_MAP_DRDY_EN;
    pin_name = "INT2";
    break;
    default:
    return -EINVAL;
    }
    int_out_ctrl_mask = BMI160_INT_OUT_CTRL_MASK << int_out_ctrl_shift;
//
// Enable the requested pin with the right settings:
// - Push-pull/open-drain
// - Active low/high
// - Edge/level triggered
//
    int_out_ctrl_bits = BMI160_OUTPUT_EN;
    if (open_drain)
// Default is push-pull.
    int_out_ctrl_bits |= BMI160_OPEN_DRAIN;
    int_out_ctrl_bits |= irq_mask;
    int_out_ctrl_bits <<= int_out_ctrl_shift;
    ret = bmi160_write_conf_reg(regmap, BMI160_REG_INT_OUT_CTRL,
    int_out_ctrl_mask, int_out_ctrl_bits,
    write_usleep);
    if (ret)
    return ret;
// Set the pin to input mode with no latching.
    ret = bmi160_write_conf_reg(regmap, BMI160_REG_INT_LATCH,
    int_latch_mask, int_latch_mask,
    write_usleep);
    if (ret)
    return ret;
// Map interrupts to the requested pin.
    ret = bmi160_write_conf_reg(regmap, BMI160_REG_INT_MAP,
    int_map_mask, int_map_mask,
    write_usleep);
    if (ret)
    dev_err(dev, "Failed to configure %s IRQ pin", pin_name);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bmi160_enable_irq(regmap: *mut regmap, enable: bool) -> c_int {
    int bmi160_enable_irq(struct regmap *regmap, bool enable)
    {
    let mut enable_bit: c_uint = 0;
    if (enable)
    enable_bit = BMI160_DRDY_INT_EN;
    return bmi160_write_conf_reg(regmap, BMI160_REG_INT_EN,
    BMI160_DRDY_INT_EN, enable_bit,
    BMI160_NORMAL_WRITE_USLEEP);
    }
    EXPORT_SYMBOL_NS(bmi160_enable_irq, "IIO_BMI160");
#[no_mangle]
unsafe extern "C" fn bmi160_get_irq(fwnode: *mut fwnode_handle, pin: *mut enum bmi160_int_pin) -> c_int {
    static int bmi160_get_irq(struct fwnode_handle *fwnode, enum bmi160_int_pin *pin)
    {
    int irq;
// Use INT1 if possible, otherwise fall back to INT2.
    irq = fwnode_irq_get_byname(fwnode, "INT1");
    if (irq > 0) {
// pin = BMI160_PIN_INT1;
    return irq;
    }
    irq = fwnode_irq_get_byname(fwnode, "INT2");
    if (irq > 0)
// pin = BMI160_PIN_INT2;
    return irq;
    }
    static int bmi160_config_device_irq(struct iio_dev *indio_dev, int irq_type,
    enum bmi160_int_pin pin)
    {
    bool open_drain;
    u8 irq_mask;
    struct bmi160_data *data = iio_priv(indio_dev);
    struct device *dev = regmap_get_device(data.regmap);
// Level-triggered, active-low is the default if we set all zeroes.
    if (irq_type == IRQF_TRIGGER_RISING)
    irq_mask = BMI160_ACTIVE_HIGH | BMI160_EDGE_TRIGGERED;
#[no_mangle]
pub unsafe extern "C" fn if(IRQF_TRIGGER_FALLING: irq_type ==) -> else {
    else if (irq_type == IRQF_TRIGGER_FALLING)
    irq_mask = BMI160_EDGE_TRIGGERED;
#[no_mangle]
pub unsafe extern "C" fn if(IRQF_TRIGGER_HIGH: irq_type ==) -> else {
    else if (irq_type == IRQF_TRIGGER_HIGH)
    irq_mask = BMI160_ACTIVE_HIGH;
#[no_mangle]
pub unsafe extern "C" fn if(IRQF_TRIGGER_LOW: irq_type ==) -> else {
    else if (irq_type == IRQF_TRIGGER_LOW)
    irq_mask = 0;
    else {
    dev_err(&indio_dev.dev,
    "Invalid interrupt type 0x%x specified\n", irq_type);
    return -EINVAL;
    }
    open_drain = device_property_read_bool(dev, "drive-open-drain");
    return bmi160_config_pin(data.regmap, pin, open_drain, irq_mask,
    BMI160_NORMAL_WRITE_USLEEP);
    }
    static int bmi160_setup_irq(struct iio_dev *indio_dev, int irq,
    enum bmi160_int_pin pin)
    {
    let mut irq_type: u32 = irq_get_trigger_type(irq);
    int ret;
    ret = bmi160_config_device_irq(indio_dev, irq_type, pin);
    if (ret)
    return ret;
    return bmi160_probe_trigger(indio_dev, irq, irq_type);
    }
#[no_mangle]
unsafe extern "C" fn bmi160_check_chip_id(chip_id: u8) -> c_int {
    static int bmi160_check_chip_id(const u8 chip_id)
    {
    for (int i = 0; i < ARRAY_SIZE(bmi_chip_ids); i++) {
    if (chip_id == bmi_chip_ids[i])
    return 0;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn bmi160_chip_init(data: *mut bmi160_data, use_spi: bool) -> c_int {
    static int bmi160_chip_init(struct bmi160_data *data, bool use_spi)
    {
    int ret;
    unsigned int val;
    struct device *dev = regmap_get_device(data.regmap);
    ret = regulator_bulk_enable(ARRAY_SIZE(data.supplies), data.supplies);
    if (ret) {
    dev_err(dev, "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    ret = regmap_write(data.regmap, BMI160_REG_CMD, BMI160_CMD_SOFTRESET);
    if (ret)
    goto disable_regulator;
    usleep_range(BMI160_SOFTRESET_USLEEP, BMI160_SOFTRESET_USLEEP + 1);
//
// CS rising edge is needed before starting SPI, so do a dummy read
// See Section 3.2.1, page 86 of the datasheet
//
    if (use_spi) {
    ret = regmap_read(data.regmap, BMI160_REG_DUMMY, &val);
    if (ret)
    goto disable_regulator;
    }
    ret = regmap_read(data.regmap, BMI160_REG_CHIP_ID, &val);
    if (ret) {
    dev_err(dev, "Error reading chip id\n");
    goto disable_regulator;
    }
    ret = bmi160_check_chip_id(val);
    if (ret)
    dev_warn(dev, "Chip id not found: %x\n", val);
    ret = bmi160_set_mode(data, BMI160_ACCEL, true);
    if (ret)
    goto disable_regulator;
    ret = bmi160_set_mode(data, BMI160_GYRO, true);
    if (ret)
    goto disable_accel;
    return 0;
    disable_accel:
    bmi160_set_mode(data, BMI160_ACCEL, false);
    disable_regulator:
    regulator_bulk_disable(ARRAY_SIZE(data.supplies), data.supplies);
    return ret;
    }
    static int bmi160_data_rdy_trigger_set_state(struct iio_trigger *trig,
    bool enable)
    {
    struct iio_dev *indio_dev = iio_trigger_get_drvdata(trig);
    struct bmi160_data *data = iio_priv(indio_dev);
    return bmi160_enable_irq(data.regmap, enable);
    }
    static const struct iio_trigger_ops bmi160_trigger_ops = {
    .set_trigger_state = &bmi160_data_rdy_trigger_set_state,
    };
#[no_mangle]
pub unsafe extern "C" fn bmi160_probe_trigger(indio_dev: *mut iio_dev, irq: c_int, irq_type: u32) -> c_int {
    int bmi160_probe_trigger(struct iio_dev *indio_dev, int irq, u32 irq_type)
    {
    struct bmi160_data *data = iio_priv(indio_dev);
    int ret;
    data.trig = devm_iio_trigger_alloc(&indio_dev.dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (data.trig == core::ptr::null_mut())
    return -ENOMEM;
    ret = devm_request_irq(&indio_dev.dev, irq,
    &iio_trigger_generic_data_rdy_poll,
    irq_type | IRQF_NO_THREAD,
    "bmi160", data.trig);
    if (ret)
    return ret;
    data.trig.dev.parent = regmap_get_device(data.regmap);
    data.trig.ops = &bmi160_trigger_ops;
    iio_trigger_set_drvdata(data.trig, indio_dev);
    ret = devm_iio_trigger_register(&indio_dev.dev, data.trig);
    if (ret)
    return ret;
    indio_dev.trig = iio_trigger_get(data.trig);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bmi160_chip_uninit(data: *mut c_void) {
    static void bmi160_chip_uninit(void *data)
    {
    struct bmi160_data *bmi_data = data;
    struct device *dev = regmap_get_device(bmi_data.regmap);
    int ret;
    bmi160_set_mode(bmi_data, BMI160_GYRO, false);
    bmi160_set_mode(bmi_data, BMI160_ACCEL, false);
    ret = regulator_bulk_disable(ARRAY_SIZE(bmi_data.supplies),
    bmi_data.supplies);
    if (ret)
    dev_err(dev, "Failed to disable regulators: %d\n", ret);
    }
    int bmi160_core_probe(struct device *dev, struct regmap *regmap,
    const char *name, bool use_spi)
    {
    struct iio_dev *indio_dev;
    struct bmi160_data *data;
    int irq;
    enum bmi160_int_pin int_pin;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    dev_set_drvdata(dev, indio_dev);
    data.regmap = regmap;
    data.supplies[0].supply = "vdd";
    data.supplies[1].supply = "vddio";
    ret = devm_regulator_bulk_get(dev,
    ARRAY_SIZE(data.supplies),
    data.supplies);
    if (ret) {
    dev_err(dev, "Failed to get regulators: %d\n", ret);
    return ret;
    }
    ret = iio_read_mount_matrix(dev, &data.orientation);
    if (ret)
    return ret;
    ret = bmi160_chip_init(data, use_spi);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, bmi160_chip_uninit, data);
    if (ret)
    return ret;
    indio_dev.channels = bmi160_channels;
    indio_dev.num_channels = ARRAY_SIZE(bmi160_channels);
    indio_dev.name = name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &bmi160_info;
    ret = devm_iio_triggered_buffer_setup(dev, indio_dev,
    iio_pollfunc_store_time,
    bmi160_trigger_handler, core::ptr::null_mut());
    if (ret)
    return ret;
    irq = bmi160_get_irq(dev_fwnode(dev), &int_pin);
    if (irq > 0) {
    ret = bmi160_setup_irq(indio_dev, irq, int_pin);
    if (ret)
    dev_err(&indio_dev.dev, "Failed to setup IRQ %d\n",
    irq);
    } else {
    dev_info(&indio_dev.dev, "Not setting up IRQ trigger\n");
    }
    return devm_iio_device_register(dev, indio_dev);
    }
    EXPORT_SYMBOL_NS_GPL(bmi160_core_probe, "IIO_BMI160");
#[no_mangle]
unsafe extern "C" fn bmi160_core_runtime_suspend(dev: *mut device) -> c_int {
    static int bmi160_core_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    return iio_device_suspend_triggering(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn bmi160_core_runtime_resume(dev: *mut device) -> c_int {
    static int bmi160_core_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    return iio_device_resume_triggering(indio_dev);
    }
    const struct dev_pm_ops bmi160_core_pm_ops = {
    RUNTIME_PM_OPS(bmi160_core_runtime_suspend, bmi160_core_runtime_resume, core::ptr::null_mut())
    };
    EXPORT_SYMBOL_NS_GPL(bmi160_core_pm_ops, "IIO_BMI160");
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com>");
    MODULE_DESCRIPTION("Bosch BMI160 driver");
    MODULE_LICENSE("GPL v2");
