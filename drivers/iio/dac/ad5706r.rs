//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5706r.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AD5706R 16-bit Current Output Digital to Analog Converter
//
// Copyright 2026 Analog Devices Inc.
//

// SPI frame layout

// Registers

pub const AD5706R_DAC_RESOLUTION: c_int = 16;

pub const AD5706R_MULTIBYTE_REG_START: c_uint = 0x14;
pub const AD5706R_MULTIBYTE_REG_END: c_uint = 0x71;
pub const AD5706R_MAX_REG: c_uint = 0x77;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5706r_state {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub __aligned(IIO_DMA_MINALIGN): u8 tx_buf[4],
    pub rx_buf: [u8; 4],
}

#[no_mangle]
unsafe extern "C" fn ad5706r_reg_len(reg: c_uint) -> c_int {
    static int ad5706r_reg_len(unsigned int reg)
    {
    if (reg >= AD5706R_MULTIBYTE_REG_START && reg <= AD5706R_MULTIBYTE_REG_END)
    return 2;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ad5706r_regmap_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int ad5706r_regmap_write(void *context, const void *data, size_t count)
    {
    struct ad5706r_state *st = context;
    unsigned int num_bytes;
    u16 reg, val;
    if (count != 4)
    return -EINVAL;
    reg = get_unaligned_be16(data);
    val = get_unaligned_be16(data + 2);
    num_bytes = ad5706r_reg_len(reg);
    struct spi_transfer xfer = {
    .tx_buf = st.tx_buf,
    .len = num_bytes + 2,
    };
    put_unaligned_be16(reg, &st.tx_buf[0]);
    if (num_bytes == 1)
    st.tx_buf[2] = (u8)val;
#[no_mangle]
pub unsafe extern "C" fn if(2: num_bytes ==) -> else {
    else if (num_bytes == 2)
    put_unaligned_be16(val, &st.tx_buf[2]);
    else
    return -EINVAL;
    return spi_sync_transfer(st.spi, &xfer, 1);
    }
    static int ad5706r_regmap_read(void *context, const void *reg_buf,
    size_t reg_size, void *val_buf, size_t val_size)
    {
    struct ad5706r_state *st = context;
    unsigned int num_bytes;
    u16 reg, cmd, val;
    int ret;
    if (reg_size != 2 || val_size != 2)
    return -EINVAL;
    reg = get_unaligned_be16(reg_buf);
    num_bytes = ad5706r_reg_len(reg);
// Full duplex, device responds immediately after command
    struct spi_transfer xfer = {
    .tx_buf = st.tx_buf,
    .rx_buf = st.rx_buf,
    .len = 2 + num_bytes,
    };
    cmd = AD5706R_RD_MASK | (reg & AD5706R_ADDR_MASK);
    put_unaligned_be16(cmd, &st.tx_buf[0]);
    put_unaligned_be16(0, &st.tx_buf[2]);
    ret = spi_sync_transfer(st.spi, &xfer, 1);
    if (ret)
    return ret;
// Extract value from response (skip 2-byte command echo)
    if (num_bytes == 1)
    val = st.rx_buf[2];
#[no_mangle]
pub unsafe extern "C" fn if(2: num_bytes ==) -> else {
    else if (num_bytes == 2)
    val = get_unaligned_be16(&st.rx_buf[2]);
    else
    return -EINVAL;
    put_unaligned_be16(val, val_buf);
    return 0;
    }
    static int ad5706r_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ad5706r_state *st = iio_priv(indio_dev);
    unsigned int reg, reg_val;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    reg = AD5706R_REG_DAC_DATA_READBACK_CH(chan.channel);
    ret = regmap_read(st.regmap, reg, &reg_val);
    if (ret)
    return ret;
// val = reg_val;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 50;
// val2 = AD5706R_DAC_RESOLUTION;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static int ad5706r_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct ad5706r_state *st = iio_priv(indio_dev);
    unsigned int reg;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (val < 0 || val > AD5706R_DAC_MAX_CODE)
    return -EINVAL;
    reg = AD5706R_REG_DAC_INPUT_A_CH(chan.channel);
    return regmap_write(st.regmap, reg, val);
    default:
    return -EINVAL;
    }
    }
    static const struct regmap_bus ad5706r_regmap_bus = {
    .write = ad5706r_regmap_write,
    .read = ad5706r_regmap_read,
    .reg_format_endian_default = REGMAP_ENDIAN_BIG,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    };
    static const struct regmap_config ad5706r_regmap_config = {
    .reg_bits = 16,
    .val_bits = 16,
    .max_register = AD5706R_MAX_REG,
    };
    static const struct iio_info ad5706r_info = {
    .read_raw = ad5706r_read_raw,
    .write_raw = ad5706r_write_raw,
    };

    .type = IIO_CURRENT,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |		\
    BIT(IIO_CHAN_INFO_SCALE),		\
    .output = 1,						\
    .indexed = 1,						\
    .channel = _channel,					\
    }
    static const struct iio_chan_spec ad5706r_channels[] = {
    AD5706R_CHAN(0),
    AD5706R_CHAN(1),
    AD5706R_CHAN(2),
    AD5706R_CHAN(3),
    };
#[no_mangle]
unsafe extern "C" fn ad5706r_probe(spi: *mut spi_device) -> c_int {
    static int ad5706r_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct ad5706r_state *st;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.regmap = devm_regmap_init(dev, &ad5706r_regmap_bus,
    st, &ad5706r_regmap_config);
    if (IS_ERR(st.regmap))
    return dev_err_probe(dev, PTR_ERR(st.regmap),
    "Failed to init regmap\n");
    indio_dev.name = "ad5706r";
    indio_dev.info = &ad5706r_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ad5706r_channels;
    indio_dev.num_channels = ARRAY_SIZE(ad5706r_channels);
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id ad5706r_of_match[] = {
    { .compatible = "adi,ad5706r" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5706r_of_match);
    static const struct spi_device_id ad5706r_id[] = {
    { .name = "ad5706r" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad5706r_id);
    static struct spi_driver ad5706r_driver = {
    .driver = {
    .name = "ad5706r",
    .of_match_table = ad5706r_of_match,
    },
    .probe = ad5706r_probe,
    .id_table = ad5706r_id,
    };
    module_spi_driver(ad5706r_driver);
    MODULE_AUTHOR("Alexis Czezar Torreno <alexisczezar.torreno@analog.com>");
    MODULE_DESCRIPTION("AD5706R 16-bit Current Output DAC driver");
    MODULE_LICENSE("GPL");
