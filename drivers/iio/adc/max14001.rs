//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/max14001.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Analog Devices MAX14001/MAX14002 ADC driver
//
// Copyright (C) 2023-2025 Analog Devices Inc.
// Copyright (C) 2023 Kim Seer Paller <kimseer.paller@analog.com>
// Copyright (c) 2025 Marilene Andrade Garcia <marilene.agarcia@gmail.com>
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/MAX14001-MAX14002.pdf
//

// MAX14001 Registers Address
pub const MAX14001_REG_ADC: c_uint = 0x00;
pub const MAX14001_REG_FADC: c_uint = 0x01;
pub const MAX14001_REG_FLAGS: c_uint = 0x02;
pub const MAX14001_REG_FLTEN: c_uint = 0x03;
pub const MAX14001_REG_THL: c_uint = 0x04;
pub const MAX14001_REG_THU: c_uint = 0x05;
pub const MAX14001_REG_INRR: c_uint = 0x06;
pub const MAX14001_REG_INRT: c_uint = 0x07;
pub const MAX14001_REG_INRP: c_uint = 0x08;
pub const MAX14001_REG_CFG: c_uint = 0x09;
pub const MAX14001_REG_ENBL: c_uint = 0x0A;
pub const MAX14001_REG_ACT: c_uint = 0x0B;
pub const MAX14001_REG_WEN: c_uint = 0x0C;

pub const MAX14001_REG_WEN_VALUE_WRITE: c_uint = 0x294;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14001_state {
    pub chip_info: *const max14001_chip_info,
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub vref_mV: c_int,
    pub spi_hw_has_lsb_first: bool,
//
// The following buffers will be bit-reversed during device
// communication, because the device transmits and receives data
// LSB-first.
// DMA (thus cache coherency maintenance) requires the transfer
// buffers to live in their own cache lines.
//
    union {
    pub be: __be16,
    pub le: __le16,
    pub __aligned(IIO_DMA_MINALIGN): } spi_tx_buffer,
    union {
    pub be: __be16,
    pub le: __le16,
    pub spi_rx_buffer: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14001_chip_info {
    pub name: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn max14001_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int max14001_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct max14001_state *st = context;
    struct spi_transfer xfers[] = {
    {
    .tx_buf = &st.spi_tx_buffer,
    .len = sizeof(st.spi_tx_buffer),
    .cs_change = 1,
    }, {
    .rx_buf = &st.spi_rx_buffer,
    .len = sizeof(st.spi_rx_buffer),
    },
    };
    int ret;
    unsigned int addr, data;
//
// Prepare SPI transmit buffer 16 bit-value and reverse bit order
// to align with the LSB-first input on SDI port in order to meet
// the device communication requirements. If the controller supports
// SPI_LSB_FIRST, this step will be handled by the SPI controller.
//
    addr = FIELD_PREP(MAX14001_MASK_ADDR, reg);
    if (st.spi_hw_has_lsb_first)
    st.spi_tx_buffer.le = cpu_to_le16(addr);
    else
    st.spi_tx_buffer.be = cpu_to_be16(bitrev16(addr));
    ret = spi_sync_transfer(st.spi, xfers, ARRAY_SIZE(xfers));
    if (ret)
    return ret;
//
// Convert received 16-bit value to cpu-endian format and reverse
// bit order. If the controller supports SPI_LSB_FIRST, this step
// will be handled by the SPI controller.
//
    if (st.spi_hw_has_lsb_first)
    data = le16_to_cpu(st.spi_rx_buffer.le);
    else
    data = bitrev16(be16_to_cpu(st.spi_rx_buffer.be));
// val = FIELD_GET(MAX14001_MASK_DATA, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max14001_write(st: *mut max14001_state, reg: c_uint, val: c_uint) -> c_int {
    static int max14001_write(struct max14001_state *st, unsigned int reg, unsigned int val)
    {
    unsigned int addr;
//
// Prepare SPI transmit buffer 16 bit-value and reverse bit order
// to align with the LSB-first input on SDI port in order to meet
// the device communication requirements. If the controller supports
// SPI_LSB_FIRST, this step will be handled by the SPI controller.
//
    addr = FIELD_PREP(MAX14001_MASK_ADDR, reg) |
    FIELD_PREP(MAX14001_MASK_WR, 1) |
    FIELD_PREP(MAX14001_MASK_DATA, val);
    if (st.spi_hw_has_lsb_first)
    st.spi_tx_buffer.le = cpu_to_le16(addr);
    else
    st.spi_tx_buffer.be = cpu_to_be16(bitrev16(addr));
    return spi_write(st.spi, &st.spi_tx_buffer, sizeof(st.spi_tx_buffer));
    }
#[no_mangle]
unsafe extern "C" fn max14001_write_single_reg(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int max14001_write_single_reg(void *context, unsigned int reg, unsigned int val)
    {
    struct max14001_state *st = context;
    int ret;
// Enable writing to the SPI register.
    ret = max14001_write(st, MAX14001_REG_WEN, MAX14001_REG_WEN_VALUE_WRITE);
    if (ret)
    return ret;
// Writing data into SPI register.
    ret = max14001_write(st, reg, val);
    if (ret)
    return ret;
// Disable writing to the SPI register.
    return max14001_write(st, MAX14001_REG_WEN, 0);
    }
#[no_mangle]
unsafe extern "C" fn max14001_write_verification_reg(st: *mut max14001_state, reg: c_uint) -> c_int {
    static int max14001_write_verification_reg(struct max14001_state *st, unsigned int reg)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(st.regmap, reg, &val);
    if (ret)
    return ret;
    return max14001_write(st, MAX14001_REG_VERIFICATION(reg), val);
    }
#[no_mangle]
unsafe extern "C" fn max14001_disable_mv_fault(st: *mut max14001_state) -> c_int {
    static int max14001_disable_mv_fault(struct max14001_state *st)
    {
    unsigned int reg;
    int ret;
// Enable writing to the SPI registers.
    ret = max14001_write(st, MAX14001_REG_WEN, MAX14001_REG_WEN_VALUE_WRITE);
    if (ret)
    return ret;
//
// Reads all registers and writes the values to their appropriate
// verification registers to clear the Memory Validation fault.
//
    for (reg = MAX14001_REG_FLTEN; reg <= MAX14001_REG_ENBL; reg++) {
    ret = max14001_write_verification_reg(st, reg);
    if (ret)
    return ret;
    }
// Disable writing to the SPI registers.
    return max14001_write(st, MAX14001_REG_WEN, 0);
    }
    static int max14001_debugfs_reg_access(struct iio_dev *indio_dev,
    unsigned int reg, unsigned int writeval,
    unsigned int *readval)
    {
    struct max14001_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    return regmap_write(st.regmap, reg, writeval);
    }
    static int max14001_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max14001_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = regmap_read(st.regmap, MAX14001_REG_ADC, val);
    if (ret)
    return ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = st->vref_mV;
// val2 = 10;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static const struct regmap_range max14001_regmap_rd_range[] = {
    regmap_reg_range(MAX14001_REG_ADC, MAX14001_REG_ENBL),
    regmap_reg_range(MAX14001_REG_WEN, MAX14001_REG_WEN),
    regmap_reg_range(MAX14001_REG_VERIFICATION(MAX14001_REG_FLTEN),
    MAX14001_REG_VERIFICATION(MAX14001_REG_ENBL)),
    };
    static const struct regmap_access_table max14001_regmap_rd_table = {
    .yes_ranges = max14001_regmap_rd_range,
    .n_yes_ranges = ARRAY_SIZE(max14001_regmap_rd_range),
    };
    static const struct regmap_range max14001_regmap_wr_range[] = {
    regmap_reg_range(MAX14001_REG_FLTEN, MAX14001_REG_WEN),
    regmap_reg_range(MAX14001_REG_VERIFICATION(MAX14001_REG_FLTEN),
    MAX14001_REG_VERIFICATION(MAX14001_REG_ENBL)),
    };
    static const struct regmap_access_table max14001_regmap_wr_table = {
    .yes_ranges = max14001_regmap_wr_range,
    .n_yes_ranges = ARRAY_SIZE(max14001_regmap_wr_range),
    };
    static const struct regmap_config max14001_regmap_config = {
    .reg_read = max14001_read,
    .reg_write = max14001_write_single_reg,
    .max_register = MAX14001_REG_VERIFICATION(MAX14001_REG_ENBL),
    .rd_table = &max14001_regmap_rd_table,
    .wr_table = &max14001_regmap_wr_table,
    };
    static const struct iio_info max14001_info = {
    .read_raw = max14001_read_raw,
    .debugfs_reg_access = max14001_debugfs_reg_access,
    };
    static const struct iio_chan_spec max14001_channel[] = {
    {
    .type = IIO_VOLTAGE,
    .indexed = 1,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    };
#[no_mangle]
unsafe extern "C" fn max14001_probe(spi: *mut spi_device) -> c_int {
    static int max14001_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct max14001_state *st;
    int ret;
    let mut use_ext_vrefin: bool = false;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.spi_hw_has_lsb_first = spi.mode & SPI_LSB_FIRST;
    st.chip_info = spi_get_device_match_data(spi);
    if (!st.chip_info)
    return -EINVAL;
    indio_dev.name = st.chip_info.name;
    indio_dev.info = &max14001_info;
    indio_dev.channels = max14001_channel;
    indio_dev.num_channels = ARRAY_SIZE(max14001_channel);
    indio_dev.modes = INDIO_DIRECT_MODE;
    st.regmap = devm_regmap_init(dev, core::ptr::null_mut(), st, &max14001_regmap_config);
    if (IS_ERR(st.regmap))
    return dev_err_probe(dev, PTR_ERR(st.regmap), "Failed to initialize regmap\n");
    ret = devm_regulator_get_enable(dev, "vdd");
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable Vdd supply\n");
    ret = devm_regulator_get_enable(dev, "vddl");
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable Vddl supply\n");
    ret = devm_regulator_get_enable_read_voltage(dev, "refin");
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(dev, ret, "Failed to get REFIN voltage\n");
    if (ret == -ENODEV)
    ret = 1250000;
    else
    use_ext_vrefin = true;
    st.vref_mV = ret / (MICRO / MILLI);
    if (use_ext_vrefin) {
//
// Configure the MAX14001/MAX14002 to use an external voltage
// reference source by setting the bit 5 of the configuration register.
//
    ret = regmap_set_bits(st.regmap, MAX14001_REG_CFG,
    MAX14001_REG_CFG_BIT_EXRF);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to set External REFIN in Configuration Register\n");
    }
    ret = max14001_disable_mv_fault(st);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to disable MV Fault\n");
    return devm_iio_device_register(dev, indio_dev);
    }
    static struct max14001_chip_info max14001_chip_info = {
    .name = "max14001",
    };
    static struct max14001_chip_info max14002_chip_info = {
    .name = "max14002",
    };
    static const struct spi_device_id max14001_id_table[] = {
    { .name = "max14001", .driver_data = (kernel_ulong_t)&max14001_chip_info },
    { .name = "max14002", .driver_data = (kernel_ulong_t)&max14002_chip_info },
    { }
    };
    static const struct of_device_id max14001_of_match[] = {
    { .compatible = "adi,max14001", .data = &max14001_chip_info },
    { .compatible = "adi,max14002", .data = &max14002_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, max14001_of_match);
    static struct spi_driver max14001_driver = {
    .driver = {
    .name = "max14001",
    .of_match_table = max14001_of_match,
    },
    .probe = max14001_probe,
    .id_table = max14001_id_table,
    };
    module_spi_driver(max14001_driver);
    MODULE_AUTHOR("Kim Seer Paller <kimseer.paller@analog.com>");
    MODULE_AUTHOR("Marilene Andrade Garcia <marilene.agarcia@gmail.com>");
    MODULE_DESCRIPTION("Analog Devices MAX14001/MAX14002 ADCs driver");
    MODULE_LICENSE("GPL");
