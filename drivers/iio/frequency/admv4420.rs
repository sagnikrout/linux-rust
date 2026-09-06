//! Automatically rewritten from C to Rust
//! Source: drivers/iio/frequency/admv4420.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// ADMV4420
//
// Copyright 2021 Analog Devices Inc.
//

// ADMV4420 Register Map
pub const ADMV4420_SPI_CONFIG_1: c_uint = 0x00;
pub const ADMV4420_SPI_CONFIG_2: c_uint = 0x01;
pub const ADMV4420_CHIPTYPE: c_uint = 0x03;
pub const ADMV4420_PRODUCT_ID_L: c_uint = 0x04;
pub const ADMV4420_PRODUCT_ID_H: c_uint = 0x05;
pub const ADMV4420_SCRATCHPAD: c_uint = 0x0A;
pub const ADMV4420_SPI_REV: c_uint = 0x0B;
pub const ADMV4420_ENABLES: c_uint = 0x103;
pub const ADMV4420_SDO_LEVEL: c_uint = 0x108;
pub const ADMV4420_INT_L: c_uint = 0x200;
pub const ADMV4420_INT_H: c_uint = 0x201;
pub const ADMV4420_FRAC_L: c_uint = 0x202;
pub const ADMV4420_FRAC_M: c_uint = 0x203;
pub const ADMV4420_FRAC_H: c_uint = 0x204;
pub const ADMV4420_MOD_L: c_uint = 0x208;
pub const ADMV4420_MOD_M: c_uint = 0x209;
pub const ADMV4420_MOD_H: c_uint = 0x20A;
pub const ADMV4420_R_DIV_L: c_uint = 0x20C;
pub const ADMV4420_R_DIV_H: c_uint = 0x20D;
pub const ADMV4420_REFERENCE: c_uint = 0x20E;
pub const ADMV4420_VCO_DATA_READBACK1: c_uint = 0x211;
pub const ADMV4420_VCO_DATA_READBACK2: c_uint = 0x212;
pub const ADMV4420_PLL_MUX_SEL: c_uint = 0x213;
pub const ADMV4420_LOCK_DETECT: c_uint = 0x214;
pub const ADMV4420_BAND_SELECT: c_uint = 0x215;
pub const ADMV4420_VCO_ALC_TIMEOUT: c_uint = 0x216;
pub const ADMV4420_VCO_MANUAL: c_uint = 0x217;
pub const ADMV4420_ALC: c_uint = 0x219;
pub const ADMV4420_VCO_TIMEOUT1: c_uint = 0x21C;
pub const ADMV4420_VCO_TIMEOUT2: c_uint = 0x21D;
pub const ADMV4420_VCO_BAND_DIV: c_uint = 0x21E;
pub const ADMV4420_VCO_READBACK_SEL: c_uint = 0x21F;
pub const ADMV4420_AUTOCAL: c_uint = 0x226;
pub const ADMV4420_CP_STATE: c_uint = 0x22C;
pub const ADMV4420_CP_BLEED_EN: c_uint = 0x22D;
pub const ADMV4420_CP_CURRENT: c_uint = 0x22E;
pub const ADMV4420_CP_BLEED: c_uint = 0x22F;

pub const ADMV4420_SCRATCH_PAD_VAL_1: c_uint = 0xAD;
pub const ADMV4420_SCRATCH_PAD_VAL_2: c_uint = 0xEA;
pub const ADMV4420_REF_FREQ_HZ: c_int = 50000000;

pub const MAX_R_DIVIDER: c_int = 1024;

    enum admv4420_mux_sel {
    ADMV4420_LOW = 0,
    ADMV4420_LOCK_DTCT = 1,
    ADMV4420_R_COUNTER_PER_2 = 4,
    ADMV4420_N_CONUTER_PER_2 = 5,
    ADMV4420_HIGH = 8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct admv4420_reference_block {
    pub doubler_en: bool,
    pub divide_by_2_en: bool,
    pub ref_single_ended: bool,
    pub divider: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct admv4420_n_counter {
    pub int_val: u32,
    pub frac_val: u32,
    pub mod_val: u32,
    pub n_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct admv4420_state {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
    pub vco_freq_hz: u64,
    pub lo_freq_hz: u64,
    pub ref_block: admv4420_reference_block,
    pub n_counter: admv4420_n_counter,
    pub mux_sel: enum admv4420_mux_sel,
    pub lock: mutex,
    pub __aligned(IIO_DMA_MINALIGN): u8 transf_buf[4],
}

    static const struct regmap_config admv4420_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    };
    static int admv4420_reg_access(struct iio_dev *indio_dev,
    u32 reg, u32 writeval,
    u32 *readval)
    {
    struct admv4420_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    else
    return regmap_write(st.regmap, reg, writeval);
    }
    static int admv4420_set_n_counter(struct admv4420_state *st, u32 int_val,
    u32 frac_val, u32 mod_val)
    {
    int ret;
    put_unaligned_le32(frac_val, st.transf_buf);
    ret = regmap_bulk_write(st.regmap, ADMV4420_FRAC_L, st.transf_buf, 3);
    if (ret)
    return ret;
    put_unaligned_le32(mod_val, st.transf_buf);
    ret = regmap_bulk_write(st.regmap, ADMV4420_MOD_L, st.transf_buf, 3);
    if (ret)
    return ret;
    put_unaligned_le32(int_val, st.transf_buf);
    return regmap_bulk_write(st.regmap, ADMV4420_INT_L, st.transf_buf, 2);
    }
    static int admv4420_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long info)
    {
    struct admv4420_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_FREQUENCY:
// val = div_u64_rem(st->lo_freq_hz, MICRO, val2);
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info admv4420_info = {
    .read_raw = admv4420_read_raw,
    .debugfs_reg_access = &admv4420_reg_access,
    };
    static const struct iio_chan_spec admv4420_channels[] = {
    {
    .type = IIO_ALTVOLTAGE,
    .output = 0,
    .indexed = 1,
    .channel = 0,
    .info_mask_separate = BIT(IIO_CHAN_INFO_FREQUENCY),
    },
    };
#[no_mangle]
unsafe extern "C" fn admv4420_fw_parse(st: *mut admv4420_state) {
    static void admv4420_fw_parse(struct admv4420_state *st)
    {
    struct device *dev = &st.spi.dev;
    u32 tmp;
    int ret;
    ret = device_property_read_u32(dev, "adi,lo-freq-khz", &tmp);
    if (!ret)
    st.lo_freq_hz = (u64)tmp * KILO;
    st.ref_block.ref_single_ended = device_property_read_bool(dev,
    "adi,ref-ext-single-ended-en");
    }
#[no_mangle]
pub unsafe extern "C" fn admv4420_calc_pfd_vco(st: *mut admv4420_state) -> u64 {
    static inline uint64_t admv4420_calc_pfd_vco(struct admv4420_state *st)
    {
    return div_u64(st.vco_freq_hz * 10, st.n_counter.n_counter);
    }
#[no_mangle]
pub unsafe extern "C" fn admv4420_calc_pfd_ref(st: *mut admv4420_state) -> u32 {
    static inline uint32_t admv4420_calc_pfd_ref(struct admv4420_state *st)
    {
    uint32_t tmp;
    u8 doubler, divide_by_2;
    doubler = st.ref_block.doubler_en ? 2 : 1;
    divide_by_2 = st.ref_block.divide_by_2_en ? 2 : 1;
    tmp = ADMV4420_REF_FREQ_HZ * doubler;
    return (tmp / (st.ref_block.divider * divide_by_2));
    }
#[no_mangle]
unsafe extern "C" fn admv4420_calc_parameters(st: *mut admv4420_state) -> c_int {
    static int admv4420_calc_parameters(struct admv4420_state *st)
    {
    u64 pfd_ref, pfd_vco;
    let mut sol_found: bool = false;
    st.ref_block.doubler_en = false;
    st.ref_block.divide_by_2_en = false;
    st.vco_freq_hz = div_u64(st.lo_freq_hz, 2);
    for (st.ref_block.divider = 1; st.ref_block.divider < MAX_R_DIVIDER;
    st.ref_block.divider++) {
    pfd_ref = admv4420_calc_pfd_ref(st);
    for (st.n_counter.n_counter = 1; st.n_counter.n_counter < MAX_N_COUNTER;
    st.n_counter.n_counter++) {
    pfd_vco = admv4420_calc_pfd_vco(st);
    if (pfd_ref == pfd_vco) {
    sol_found = true;
    break;
    }
    }
    if (sol_found)
    break;
    st.n_counter.n_counter = 1;
    }
    if (!sol_found)
    return -EINVAL;
    st.n_counter.int_val = div_u64_rem(st.n_counter.n_counter, 10, &st.n_counter.frac_val);
    st.n_counter.mod_val = 10;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn admv4420_setup(indio_dev: *mut iio_dev) -> c_int {
    static int admv4420_setup(struct iio_dev *indio_dev)
    {
    struct admv4420_state *st = iio_priv(indio_dev);
    struct device *dev = indio_dev.dev.parent;
    u32 val;
    int ret;
    ret = regmap_write(st.regmap, ADMV4420_SPI_CONFIG_1,
    ADMV4420_SPI_CONFIG_1_SOFTRESET);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, ADMV4420_SPI_CONFIG_1,
    ADMV4420_SPI_CONFIG_1_SDOACTIVE |
    ADMV4420_SPI_CONFIG_1_ENDIAN);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap,
    ADMV4420_SCRATCHPAD,
    ADMV4420_SCRATCH_PAD_VAL_1);
    if (ret)
    return ret;
    ret = regmap_read(st.regmap, ADMV4420_SCRATCHPAD, &val);
    if (ret)
    return ret;
    if (val != ADMV4420_SCRATCH_PAD_VAL_1)
    return dev_err_probe(dev, -EIO,
    "Failed ADMV4420 to read/write scratchpad %x\n", val);
    ret = regmap_write(st.regmap,
    ADMV4420_SCRATCHPAD,
    ADMV4420_SCRATCH_PAD_VAL_2);
    if (ret)
    return ret;
    ret = regmap_read(st.regmap, ADMV4420_SCRATCHPAD, &val);
    if (ret)
    return ret;
    if (val != ADMV4420_SCRATCH_PAD_VAL_2)
    return dev_err_probe(dev, -EIO,
    "Failed to read/write scratchpad %x\n", val);
    st.mux_sel = ADMV4420_LOCK_DTCT;
    st.lo_freq_hz = ADMV4420_DEFAULT_LO_FREQ_HZ;
    admv4420_fw_parse(st);
    ret = admv4420_calc_parameters(st);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed calc parameters for %llu\n",
    st.vco_freq_hz);
    ret = regmap_write(st.regmap, ADMV4420_R_DIV_L,
    FIELD_GET(0xFF, st.ref_block.divider));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, ADMV4420_R_DIV_H,
    FIELD_GET(0xFF00, st.ref_block.divider));
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, ADMV4420_REFERENCE,
    st.ref_block.divide_by_2_en |
    FIELD_PREP(ADMV4420_REFERENCE_MODE_MASK, st.ref_block.ref_single_ended) |
    FIELD_PREP(ADMV4420_REFERENCE_DOUBLER_MASK, st.ref_block.doubler_en));
    if (ret)
    return ret;
    ret = admv4420_set_n_counter(st, st.n_counter.int_val,
    st.n_counter.frac_val,
    st.n_counter.mod_val);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, ADMV4420_PLL_MUX_SEL, st.mux_sel);
    if (ret)
    return ret;
    return regmap_write(st.regmap, ADMV4420_ENABLES,
    ENABLE_PLL | ENABLE_LO | ENABLE_VCO |
    ENABLE_IFAMP | ENABLE_MIXER | ENABLE_LNA);
    }
#[no_mangle]
unsafe extern "C" fn admv4420_probe(spi: *mut spi_device) -> c_int {
    static int admv4420_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct admv4420_state *st;
    struct regmap *regmap;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    regmap = devm_regmap_init_spi(spi, &admv4420_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to initializing spi regmap\n");
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.regmap = regmap;
    indio_dev.name = "admv4420";
    indio_dev.info = &admv4420_info;
    indio_dev.channels = admv4420_channels;
    indio_dev.num_channels = ARRAY_SIZE(admv4420_channels);
    ret = admv4420_setup(indio_dev);
    if (ret)
    return dev_err_probe(dev, ret, "Setup ADMV4420 failed\n");
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id admv4420_of_match[] = {
    { .compatible = "adi,admv4420" },
    { }
    };
    MODULE_DEVICE_TABLE(of, admv4420_of_match);
    static struct spi_driver admv4420_driver = {
    .driver = {
    .name = "admv4420",
    .of_match_table = admv4420_of_match,
    },
    .probe = admv4420_probe,
    };
    module_spi_driver(admv4420_driver);
    MODULE_AUTHOR("Cristian Pop <cristian.pop@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADMV44200 K Band Downconverter");
    MODULE_LICENSE("Dual BSD/GPL");
