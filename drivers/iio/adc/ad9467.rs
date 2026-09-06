//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ad9467.c
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
// Analog Devices AD9467 SPI ADC driver
//
// Copyright 2012-2020 Analog Devices Inc.
//

//
// ADI High-Speed ADC common spi interface registers
// See Application-Note AN-877:
// https://www.analog.com/media/en/technical-documentation/application-notes/AN-877.pdf
//
pub const AN877_ADC_REG_CHIP_PORT_CONF: c_uint = 0x00;
pub const AN877_ADC_REG_CHIP_ID: c_uint = 0x01;
pub const AN877_ADC_REG_CHIP_GRADE: c_uint = 0x02;
pub const AN877_ADC_REG_CHAN_INDEX: c_uint = 0x05;
pub const AN877_ADC_REG_TRANSFER: c_uint = 0xFF;
pub const AN877_ADC_REG_MODES: c_uint = 0x08;
pub const AN877_ADC_REG_TEST_IO: c_uint = 0x0D;
pub const AN877_ADC_REG_ADC_INPUT: c_uint = 0x0F;
pub const AN877_ADC_REG_OFFSET: c_uint = 0x10;
pub const AN877_ADC_REG_OUTPUT_MODE: c_uint = 0x14;
pub const AN877_ADC_REG_OUTPUT_ADJUST: c_uint = 0x15;
pub const AN877_ADC_REG_OUTPUT_PHASE: c_uint = 0x16;
pub const AN877_ADC_REG_OUTPUT_DELAY: c_uint = 0x17;
pub const AN877_ADC_REG_VREF: c_uint = 0x18;
pub const AN877_ADC_REG_ANALOG_INPUT: c_uint = 0x2C;
// AN877_ADC_REG_TEST_IO
pub const AN877_ADC_TESTMODE_OFF: c_uint = 0x0;
pub const AN877_ADC_TESTMODE_MIDSCALE_SHORT: c_uint = 0x1;
pub const AN877_ADC_TESTMODE_POS_FULLSCALE: c_uint = 0x2;
pub const AN877_ADC_TESTMODE_NEG_FULLSCALE: c_uint = 0x3;
pub const AN877_ADC_TESTMODE_ALT_CHECKERBOARD: c_uint = 0x4;
pub const AN877_ADC_TESTMODE_PN23_SEQ: c_uint = 0x5;
pub const AN877_ADC_TESTMODE_PN9_SEQ: c_uint = 0x6;
pub const AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE: c_uint = 0x7;
pub const AN877_ADC_TESTMODE_USER: c_uint = 0x8;
pub const AN877_ADC_TESTMODE_BIT_TOGGLE: c_uint = 0x9;
pub const AN877_ADC_TESTMODE_SYNC: c_uint = 0xA;
pub const AN877_ADC_TESTMODE_ONE_BIT_HIGH: c_uint = 0xB;
pub const AN877_ADC_TESTMODE_MIXED_BIT_FREQUENCY: c_uint = 0xC;
pub const AN877_ADC_TESTMODE_RAMP: c_uint = 0xF;
// AN877_ADC_REG_TRANSFER
pub const AN877_ADC_TRANSFER_SYNC: c_uint = 0x1;
// AN877_ADC_REG_OUTPUT_MODE
pub const AN877_ADC_OUTPUT_MODE_OFFSET_BINARY: c_uint = 0x0;
pub const AN877_ADC_OUTPUT_MODE_TWOS_COMPLEMENT: c_uint = 0x1;
pub const AN877_ADC_OUTPUT_MODE_GRAY_CODE: c_uint = 0x2;

// AN877_ADC_REG_OUTPUT_PHASE
pub const AN877_ADC_OUTPUT_EVEN_ODD_MODE_EN: c_uint = 0x20;
pub const AN877_ADC_INVERT_DCO_CLK: c_uint = 0x80;
// AN877_ADC_REG_OUTPUT_DELAY
pub const AN877_ADC_DCO_DELAY_ENABLE: c_uint = 0x80;
//
// Analog Devices AD9211 10-Bit, 200/250/300 MSPS ADC
//
pub const CHIPID_AD9211: c_uint = 0x06;
pub const AD9211_DEF_OUTPUT_MODE: c_uint = 0x01;

//
// Analog Devices AD9265 16-Bit, 125/105/80 MSPS ADC
//
pub const CHIPID_AD9265: c_uint = 0x64;
pub const AD9265_DEF_OUTPUT_MODE: c_uint = 0x41;
pub const AD9265_REG_VREF_MASK: c_uint = 0xC0;
//
// Analog Devices AD9434 12-Bit, 370/500 MSPS ADC
//
pub const CHIPID_AD9434: c_uint = 0x6A;
pub const AD9434_DEF_OUTPUT_MODE: c_uint = 0x01;

//
// Analog Devices AD9467 16-Bit, 200/250 MSPS ADC
//
pub const CHIPID_AD9467: c_uint = 0x50;
pub const AD9467_DEF_OUTPUT_MODE: c_uint = 0x09;
pub const AD9467_REG_VREF_MASK: c_uint = 0x0F;
//
// Analog Devices AD9643 14-Bit, 170/210/250 MSPS ADC
//
pub const CHIPID_AD9643: c_uint = 0x82;
pub const AD9643_DEF_OUTPUT_MODE: c_uint = 0x01;
pub const AD9643_REG_VREF_MASK: c_uint = 0x1F;
//
// Analog Devices AD9652 16-bit 310 MSPS ADC
//
pub const CHIPID_AD9652: c_uint = 0xC1;
pub const AD9652_DEF_OUTPUT_MODE: c_uint = 0x01;
pub const AD9652_REG_VREF_MASK: c_uint = 0xC0;
//
// Analog Devices AD9649 14-bit 20/40/65/80 MSPS ADC
//
pub const CHIPID_AD9649: c_uint = 0x6F;
pub const AD9649_DEF_OUTPUT_MODE: c_uint = 0x01;
pub const AD9649_TEST_POINTS: c_int = 8;
pub const AD9647_MAX_TEST_POINTS: c_int = 32;

    (!(st).info.has_dco || (st).info.has_dco_invert)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad9467_chip_info {
    pub name: *const c_char,
    pub id: c_uint,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
    pub (*scale_table)[2]: *const c_uint,
    pub num_scales: c_int,
    pub test_mask: c_ulong,
    pub test_mask_len: c_uint,
    pub max_rate: c_ulong,
    pub default_output_mode: c_uint,
    pub vref_mask: c_uint,
    pub num_lanes: c_uint,
    pub dco_en: c_uint,
    pub test_points: c_uint,
    pub offset_range: *const c_int,
// data clock output
    pub has_dco: bool,
    pub has_dco_invert: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad9467_chan_test_mode {
    pub st: *mut ad9467_state,
    pub idx: c_uint,
    pub mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad9467_state {
    pub info: *const ad9467_chip_info,
    pub back: *mut iio_backend,
    pub spi: *mut spi_device,
    pub clk: *mut clk,
// used for debugfs
    pub chan_test: *mut ad9467_chan_test_mode,
    pub (*scales)[2]: *mut c_uint,
//
// Times 2 because we may also invert the signal polarity and run the
// calibration again. For some reference on the test points (ad9265) see:
// https://www.analog.com/media/en/technical-documentation/data-sheets/ad9265.pdf
// at page 38 for the dco output delay. On devices as ad9467, the
// calibration is done at the backend level. For the ADI axi-adc:
// https://wiki.analog.com/resources/fpga/docs/axi_adc_ip
// at the io delay control section.
//
    pub 2): *mut *mut DECLARE_BITMAP(calib_map, AD9647_MAX_TEST_POINTS,
// number of bits of the map
    pub calib_map_size: c_uint,
    pub pwrdown_gpio: *mut gpio_desc,
// ensure consistent state obtained on multiple related accesses
    pub lock: mutex,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[3],
}

#[no_mangle]
unsafe extern "C" fn ad9467_spi_read(st: *mut ad9467_state, reg: c_uint) -> c_int {
    static int ad9467_spi_read(struct ad9467_state *st, unsigned int reg)
    {
    unsigned char tbuf[2], rbuf[1];
    int ret;
    tbuf[0] = 0x80 | (reg >> 8);
    tbuf[1] = reg & 0xFF;
    ret = spi_write_then_read(st.spi,
    tbuf, ARRAY_SIZE(tbuf),
    rbuf, ARRAY_SIZE(rbuf));
    if (ret < 0)
    return ret;
    return rbuf[0];
    }
    static int ad9467_spi_write(struct ad9467_state *st, unsigned int reg,
    unsigned int val)
    {
    st.buf[0] = reg >> 8;
    st.buf[1] = reg & 0xFF;
    st.buf[2] = val;
    return spi_write(st.spi, st.buf, ARRAY_SIZE(st.buf));
    }
    static int ad9467_reg_access(struct iio_dev *indio_dev, unsigned int reg,
    unsigned int writeval, unsigned int *readval)
    {
    struct ad9467_state *st = iio_priv(indio_dev);
    int ret;
    if (!readval) {
    guard(mutex)(&st.lock);
    ret = ad9467_spi_write(st, reg, writeval);
    if (ret)
    return ret;
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
    ret = ad9467_spi_read(st, reg);
    if (ret < 0)
    return ret;
// readval = ret;
    return 0;
    }
    static const int ad9434_offset_range[] = {
    -128, 1, 127,
    };
    static const unsigned int ad9211_scale_table[][2] = {
    {980, 0x10}, {1000, 0x11}, {1020, 0x12}, {1040, 0x13},
    {1060, 0x14}, {1080, 0x15}, {1100, 0x16}, {1120, 0x17},
    {1140, 0x18}, {1160, 0x19}, {1180, 0x1A}, {1190, 0x1B},
    {1200, 0x1C}, {1210, 0x1D}, {1220, 0x1E}, {1230, 0x1F},
    {1250, 0x0}, {1270, 0x1}, {1290, 0x2}, {1310, 0x3},
    {1330, 0x4}, {1350, 0x5}, {1370, 0x6}, {1390, 0x7},
    {1410, 0x8}, {1430, 0x9}, {1450, 0xA}, {1460, 0xB},
    {1470, 0xC}, {1480, 0xD}, {1490, 0xE}, {1500, 0xF},
    };
    static const unsigned int ad9265_scale_table[][2] = {
    {1250, 0x00}, {1500, 0x40}, {1750, 0x80}, {2000, 0xC0},
    };
    static const unsigned int ad9434_scale_table[][2] = {
    {1600, 0x1C}, {1580, 0x1D}, {1550, 0x1E}, {1520, 0x1F}, {1500, 0x00},
    {1470, 0x01}, {1440, 0x02}, {1420, 0x03}, {1390, 0x04}, {1360, 0x05},
    {1340, 0x06}, {1310, 0x07}, {1280, 0x08}, {1260, 0x09}, {1230, 0x0A},
    {1200, 0x0B}, {1180, 0x0C},
    };
    static const unsigned int ad9467_scale_table[][2] = {
    {2000, 0}, {2100, 6}, {2200, 7},
    {2300, 8}, {2400, 9}, {2500, 10},
    };
    static const unsigned int ad9643_scale_table[][2] = {
    {2087, 0x0F}, {2065, 0x0E}, {2042, 0x0D}, {2020, 0x0C}, {1997, 0x0B},
    {1975, 0x0A}, {1952, 0x09}, {1930, 0x08}, {1907, 0x07}, {1885, 0x06},
    {1862, 0x05}, {1840, 0x04}, {1817, 0x03}, {1795, 0x02}, {1772, 0x01},
    {1750, 0x00}, {1727, 0x1F}, {1704, 0x1E}, {1681, 0x1D}, {1658, 0x1C},
    {1635, 0x1B}, {1612, 0x1A}, {1589, 0x19}, {1567, 0x18}, {1544, 0x17},
    {1521, 0x16}, {1498, 0x15}, {1475, 0x14}, {1452, 0x13}, {1429, 0x12},
    {1406, 0x11}, {1383, 0x10},
    };
    static const unsigned int ad9649_scale_table[][2] = {
    {2000, 0},
    };
    static const unsigned int ad9652_scale_table[][2] = {
    {1250, 0}, {1125, 1}, {1200, 2}, {1250, 3}, {1000, 5},
    };
    static void __ad9467_get_scale(struct ad9467_state *st, int index,
    unsigned int *val, unsigned int *val2)
    {
    const struct ad9467_chip_info *info = st.info;
    const struct iio_chan_spec *chan = &info.channels[0];
    unsigned int tmp;
    tmp = (info.scale_table[index][0] * 1000000ULL) >>
    chan.scan_type.realbits;
// val = tmp / 1000000;
// val2 = tmp % 1000000;
    }

    {									\
    .type = IIO_VOLTAGE,						\
    .indexed = 1,							\
    .channel = _chan,						\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_SAMP_FREQ),				\
    .info_mask_shared_by_type_available = avai_mask,		\
    .scan_index = _si,						\
    .scan_type = {							\
    .sign = _sign,						\
    .realbits = _bits,					\
    .storagebits = 16,					\
    },								\
    }
    static const struct iio_chan_spec ad9211_channels[] = {
    AD9467_CHAN(0, BIT(IIO_CHAN_INFO_SCALE), 0, 10, 's'),
    };
    static const struct iio_chan_spec ad9434_channels[] = {
    {
    .type = IIO_VOLTAGE,
    .indexed = 1,
    .channel = 0,
    .info_mask_shared_by_type =
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_SAMP_FREQ) |
    BIT(IIO_CHAN_INFO_CALIBBIAS),
    .info_mask_shared_by_type_available =
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_CALIBBIAS),
    .scan_index = 0,
    .scan_type = {
    .sign = 's',
    .realbits = 12,
    .storagebits = 16,
    },
    },
    };
    static const struct iio_chan_spec ad9467_channels[] = {
    AD9467_CHAN(0, BIT(IIO_CHAN_INFO_SCALE), 0, 16, 's'),
    };
    static const struct iio_chan_spec ad9643_channels[] = {
    AD9467_CHAN(0, BIT(IIO_CHAN_INFO_SCALE), 0, 14, 's'),
    AD9467_CHAN(1, BIT(IIO_CHAN_INFO_SCALE), 1, 14, 's'),
    };
    static const struct iio_chan_spec ad9649_channels[] = {
    AD9467_CHAN(0, 0, 0, 14, 's'),
    };
    static const struct iio_chan_spec ad9652_channels[] = {
    AD9467_CHAN(0, BIT(IIO_CHAN_INFO_SCALE), 0, 16, 's'),
    AD9467_CHAN(1, BIT(IIO_CHAN_INFO_SCALE), 1, 16, 's'),
    };
    static const char * const ad9467_test_modes[] = {
    [AN877_ADC_TESTMODE_OFF] = "off",
    [AN877_ADC_TESTMODE_MIDSCALE_SHORT] = "midscale_short",
    [AN877_ADC_TESTMODE_POS_FULLSCALE] = "pos_fullscale",
    [AN877_ADC_TESTMODE_NEG_FULLSCALE] = "neg_fullscale",
    [AN877_ADC_TESTMODE_ALT_CHECKERBOARD] = "checkerboard",
    [AN877_ADC_TESTMODE_PN23_SEQ] = "prbs23",
    [AN877_ADC_TESTMODE_PN9_SEQ] = "prbs9",
    [AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE] = "one_zero_toggle",
    [AN877_ADC_TESTMODE_USER] = "user",
    [AN877_ADC_TESTMODE_BIT_TOGGLE] = "bit_toggle",
    [AN877_ADC_TESTMODE_SYNC] = "sync",
    [AN877_ADC_TESTMODE_ONE_BIT_HIGH] = "one_bit_high",
    [AN877_ADC_TESTMODE_MIXED_BIT_FREQUENCY] = "mixed_bit_frequency",
    [AN877_ADC_TESTMODE_RAMP] = "ramp",
    };
    static const struct ad9467_chip_info ad9467_chip_tbl = {
    .name = "ad9467",
    .id = CHIPID_AD9467,
    .max_rate = 250000000UL,
    .scale_table = ad9467_scale_table,
    .num_scales = ARRAY_SIZE(ad9467_scale_table),
    .channels = ad9467_channels,
    .num_channels = ARRAY_SIZE(ad9467_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE,
    AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE + 1,
    .default_output_mode = AD9467_DEF_OUTPUT_MODE,
    .vref_mask = AD9467_REG_VREF_MASK,
    .num_lanes = 8,
    };
    static const struct ad9467_chip_info ad9434_chip_tbl = {
    .name = "ad9434",
    .id = CHIPID_AD9434,
    .max_rate = 500000000UL,
    .scale_table = ad9434_scale_table,
    .num_scales = ARRAY_SIZE(ad9434_scale_table),
    .channels = ad9434_channels,
    .num_channels = ARRAY_SIZE(ad9434_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_USER, AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_USER + 1,
    .default_output_mode = AD9434_DEF_OUTPUT_MODE,
    .vref_mask = AD9434_REG_VREF_MASK,
    .num_lanes = 6,
    .offset_range = ad9434_offset_range,
    };
    static const struct ad9467_chip_info ad9211_chip_tbl = {
    .name = "ad9211",
    .id = CHIPID_AD9211,
    .max_rate = 300 * HZ_PER_MHZ,
    .scale_table = ad9211_scale_table,
    .num_scales = ARRAY_SIZE(ad9211_scale_table),
    .channels = ad9211_channels,
    .num_channels = ARRAY_SIZE(ad9211_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE,
    AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE + 1,
    .default_output_mode = AD9211_DEF_OUTPUT_MODE,
    .vref_mask = AD9211_REG_VREF_MASK,
    .has_dco = true,
    };
    static const struct ad9467_chip_info ad9265_chip_tbl = {
    .name = "ad9265",
    .id = CHIPID_AD9265,
    .max_rate = 125000000UL,
    .scale_table = ad9265_scale_table,
    .num_scales = ARRAY_SIZE(ad9265_scale_table),
    .channels = ad9467_channels,
    .num_channels = ARRAY_SIZE(ad9467_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE,
    AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE + 1,
    .default_output_mode = AD9265_DEF_OUTPUT_MODE,
    .vref_mask = AD9265_REG_VREF_MASK,
    .has_dco = true,
    .has_dco_invert = true,
    };
    static const struct ad9467_chip_info ad9643_chip_tbl = {
    .name = "ad9643",
    .id = CHIPID_AD9643,
    .max_rate = 250000000UL,
    .scale_table = ad9643_scale_table,
    .num_scales = ARRAY_SIZE(ad9643_scale_table),
    .channels = ad9643_channels,
    .num_channels = ARRAY_SIZE(ad9643_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = BIT(AN877_ADC_TESTMODE_RAMP) |
    GENMASK(AN877_ADC_TESTMODE_MIXED_BIT_FREQUENCY, AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_RAMP + 1,
    .default_output_mode = AD9643_DEF_OUTPUT_MODE,
    .vref_mask = AD9643_REG_VREF_MASK,
    .has_dco = true,
    .has_dco_invert = true,
    .dco_en = AN877_ADC_DCO_DELAY_ENABLE,
    };
    static const struct ad9467_chip_info ad9649_chip_tbl = {
    .name = "ad9649",
    .id = CHIPID_AD9649,
    .max_rate = 80000000UL,
    .scale_table = ad9649_scale_table,
    .num_scales = ARRAY_SIZE(ad9649_scale_table),
    .channels = ad9649_channels,
    .num_channels = ARRAY_SIZE(ad9649_channels),
    .test_points = AD9649_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_MIXED_BIT_FREQUENCY,
    AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_MIXED_BIT_FREQUENCY + 1,
    .default_output_mode = AD9649_DEF_OUTPUT_MODE,
    .has_dco = true,
    .has_dco_invert = true,
    .dco_en = AN877_ADC_DCO_DELAY_ENABLE,
    };
    static const struct ad9467_chip_info ad9652_chip_tbl = {
    .name = "ad9652",
    .id = CHIPID_AD9652,
    .max_rate = 310000000UL,
    .scale_table = ad9652_scale_table,
    .num_scales = ARRAY_SIZE(ad9652_scale_table),
    .channels = ad9652_channels,
    .num_channels = ARRAY_SIZE(ad9652_channels),
    .test_points = AD9647_MAX_TEST_POINTS,
    .test_mask = GENMASK(AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE,
    AN877_ADC_TESTMODE_OFF),
    .test_mask_len = AN877_ADC_TESTMODE_ONE_ZERO_TOGGLE + 1,
    .default_output_mode = AD9652_DEF_OUTPUT_MODE,
    .vref_mask = AD9652_REG_VREF_MASK,
    .has_dco = true,
    };
#[no_mangle]
unsafe extern "C" fn ad9467_get_scale(st: *mut ad9467_state, val: *mut c_int, val2: *mut c_int) -> c_int {
    static int ad9467_get_scale(struct ad9467_state *st, int *val, int *val2)
    {
    const struct ad9467_chip_info *info = st.info;
    unsigned int vref_val;
    let mut i: c_uint = 0;
    int ret;
// nothing to read if we only have one possible scale
    if (info.num_scales == 1)
    goto out_get_scale;
    ret = ad9467_spi_read(st, AN877_ADC_REG_VREF);
    if (ret < 0)
    return ret;
    vref_val = ret & info.vref_mask;
    for (i = 0; i < info.num_scales; i++) {
    if (vref_val == info.scale_table[i][1])
    break;
    }
    if (i == info.num_scales)
    return -ERANGE;
    out_get_scale:
    __ad9467_get_scale(st, i, val, val2);
    return IIO_VAL_INT_PLUS_MICRO;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_set_scale(st: *mut ad9467_state, val: c_int, val2: c_int) -> c_int {
    static int ad9467_set_scale(struct ad9467_state *st, int val, int val2)
    {
    const struct ad9467_chip_info *info = st.info;
    unsigned int scale_val[2];
    unsigned int i;
    int ret;
    if (val != 0)
    return -EINVAL;
    if (info.num_scales == 1)
    return -EOPNOTSUPP;
    for (i = 0; i < info.num_scales; i++) {
    __ad9467_get_scale(st, i, &scale_val[0], &scale_val[1]);
    if (scale_val[0] != val || scale_val[1] != val2)
    continue;
    guard(mutex)(&st.lock);
    ret = ad9467_spi_write(st, AN877_ADC_REG_VREF,
    info.scale_table[i][1]);
    if (ret < 0)
    return ret;
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_get_offset(st: *mut ad9467_state, val: *mut c_int) -> c_int {
    static int ad9467_get_offset(struct ad9467_state *st, int *val)
    {
    int ret;
    ret = ad9467_spi_read(st, AN877_ADC_REG_OFFSET);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_set_offset(st: *mut ad9467_state, val: c_int) -> c_int {
    static int ad9467_set_offset(struct ad9467_state *st, int val)
    {
    int ret;
    if (val < st.info.offset_range[0] || val > st.info.offset_range[2])
    return -EINVAL;
    ret = ad9467_spi_write(st, AN877_ADC_REG_OFFSET, val);
    if (ret < 0)
    return ret;
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
#[no_mangle]
unsafe extern "C" fn ad9467_outputmode_set(st: *mut ad9467_state, mode: c_uint) -> c_int {
    static int ad9467_outputmode_set(struct ad9467_state *st, unsigned int mode)
    {
    int ret;
    ret = ad9467_spi_write(st, AN877_ADC_REG_OUTPUT_MODE, mode);
    if (ret < 0)
    return ret;
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
    static int ad9467_testmode_set(struct ad9467_state *st, unsigned int chan,
    unsigned int test_mode)
    {
    int ret;
    if (st.info.num_channels > 1) {
// so that the test mode is only applied to one channel
    ret = ad9467_spi_write(st, AN877_ADC_REG_CHAN_INDEX, BIT(chan));
    if (ret)
    return ret;
    }
    ret = ad9467_spi_write(st, AN877_ADC_REG_TEST_IO, test_mode);
    if (ret)
    return ret;
    if (st.info.num_channels > 1) {
// go to default state where all channels get write commands
    ret = ad9467_spi_write(st, AN877_ADC_REG_CHAN_INDEX,
    GENMASK(st.info.num_channels - 1, 0));
    if (ret)
    return ret;
    }
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
    static int ad9467_backend_testmode_on(struct ad9467_state *st,
    unsigned int chan,
    enum iio_backend_test_pattern pattern)
    {
    struct iio_backend_data_fmt data = {
    .enable = false,
    };
    int ret;
    ret = iio_backend_data_format_set(st.back, chan, &data);
    if (ret)
    return ret;
    ret = iio_backend_test_pattern_set(st.back, chan, pattern);
    if (ret)
    return ret;
    return iio_backend_chan_enable(st.back, chan);
    }
    static int ad9467_backend_testmode_off(struct ad9467_state *st,
    unsigned int chan)
    {
    struct iio_backend_data_fmt data = {
    .enable = true,
    .sign_extend = true,
    };
    int ret;
    ret = iio_backend_chan_disable(st.back, chan);
    if (ret)
    return ret;
    ret = iio_backend_test_pattern_set(st.back, chan,
    IIO_BACKEND_NO_TEST_PATTERN);
    if (ret)
    return ret;
    return iio_backend_data_format_set(st.back, chan, &data);
    }
#[no_mangle]
unsafe extern "C" fn ad9647_calibrate_prepare(st: *mut ad9467_state) -> c_int {
    static int ad9647_calibrate_prepare(struct ad9467_state *st)
    {
    unsigned int cmode;
    unsigned int c;
    int ret;
    cmode = st.info.default_output_mode;
    FIELD_MODIFY(AN877_ADC_OUTPUT_MODE_MASK, &cmode,
    AN877_ADC_OUTPUT_MODE_OFFSET_BINARY);
    ret = ad9467_outputmode_set(st, cmode);
    if (ret)
    return ret;
    for (c = 0; c < st.info.num_channels; c++) {
    ret = ad9467_testmode_set(st, c, AN877_ADC_TESTMODE_PN9_SEQ);
    if (ret)
    return ret;
    ret = ad9467_backend_testmode_on(st, c,
    IIO_BACKEND_ADI_PRBS_9A);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int ad9647_calibrate_polarity_set(struct ad9467_state *st,
    bool invert)
    {
    enum iio_backend_sample_trigger trigger;
    if (st.info.has_dco) {
    let mut phase: c_uint = AN877_ADC_OUTPUT_EVEN_ODD_MODE_EN;
    if (invert)
    phase |= AN877_ADC_INVERT_DCO_CLK;
    return ad9467_spi_write(st, AN877_ADC_REG_OUTPUT_PHASE,
    phase);
    }
    if (invert)
    trigger = IIO_BACKEND_SAMPLE_TRIGGER_EDGE_FALLING;
    else
    trigger = IIO_BACKEND_SAMPLE_TRIGGER_EDGE_RISING;
    return iio_backend_data_sample_trigger(st.back, trigger);
    }
//
// The idea is pretty simple. Find the max number of successful points in a row
// and get the one in the middle.
//
    static unsigned int ad9467_find_optimal_point(const unsigned long *calib_map,
    unsigned int start,
    unsigned int nbits,
    unsigned int *val)
    {
    let mut bit: c_uint = start, end, start_cnt, cnt = 0;
    for_each_clear_bitrange_from(bit, end, calib_map, nbits + start) {
    if (end - bit > cnt) {
    cnt = end - bit;
    start_cnt = bit;
    }
    }
    if (cnt)
// val = start_cnt + cnt / 2;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_calibrate_apply(st: *mut ad9467_state, val: c_uint) -> c_int {
    static int ad9467_calibrate_apply(struct ad9467_state *st, unsigned int val)
    {
    unsigned int lane;
    int ret;
    if (st.info.has_dco) {
    ret = ad9467_spi_write(st, AN877_ADC_REG_OUTPUT_DELAY,
    val | st.info.dco_en);
    if (ret)
    return ret;
    return ad9467_spi_write(st, AN877_ADC_REG_TRANSFER,
    AN877_ADC_TRANSFER_SYNC);
    }
    for (lane = 0; lane < st.info.num_lanes; lane++) {
    ret = iio_backend_iodelay_set(st.back, lane, val);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad9647_calibrate_stop(st: *mut ad9467_state) -> c_int {
    static int ad9647_calibrate_stop(struct ad9467_state *st)
    {
    unsigned int c, mode;
    int ret;
    for (c = 0; c < st.info.num_channels; c++) {
    ret = ad9467_backend_testmode_off(st, c);
    if (ret)
    return ret;
    ret = ad9467_testmode_set(st, c, AN877_ADC_TESTMODE_OFF);
    if (ret)
    return ret;
    }
    mode = st.info.default_output_mode;
    return ad9467_outputmode_set(st, mode);
    }
#[no_mangle]
unsafe extern "C" fn ad9467_calibrate(st: *mut ad9467_state) -> c_int {
    static int ad9467_calibrate(struct ad9467_state *st)
    {
    unsigned int point, val, inv_val, cnt, inv_cnt = 0, c;
//
// Half of the bitmap is for the inverted signal. The number of test
// points is the same though...
//
    let mut test_points: c_uint = st.info.test_points;
    let mut sample_rate: c_ulong = clk_get_rate(st.clk);
    struct device *dev = &st.spi.dev;
    let mut invert: bool = false, stat;
    int ret;
// all points invalid
    bitmap_fill(st.calib_map, st.calib_map_size);
    ret = ad9647_calibrate_prepare(st);
    if (ret)
    return ret;
    retune:
    ret = ad9647_calibrate_polarity_set(st, invert);
    if (ret)
    return ret;
    for (point = 0; point < st.info.test_points; point++) {
    ret = ad9467_calibrate_apply(st, point);
    if (ret)
    return ret;
    for (c = 0; c < st.info.num_channels; c++) {
    ret = iio_backend_chan_status(st.back, c, &stat);
    if (ret)
    return ret;
//
// A point is considered valid if all channels report no
// error. If one reports an error, then we consider the
// point as invalid and we can break the loop right away.
//
    if (stat) {
    dev_dbg(dev, "Invalid point(%u, inv:%u) for CH:%u\n",
    point, invert, c);
    break;
    }
    if (c == st.info.num_channels - 1)
    __clear_bit(point + invert * test_points,
    st.calib_map);
    }
    }
    if (!invert) {
    cnt = ad9467_find_optimal_point(st.calib_map, 0, test_points,
    &val);
//
// We're happy if we find, at least, three good test points in
// a row.
//
    if (cnt < 3) {
    if (AD9467_CAN_INVERT(st)) {
    invert = true;
    goto retune;
    }
    if (!cnt)
    return -EIO;
    }
    } else {
    inv_cnt = ad9467_find_optimal_point(st.calib_map, test_points,
    test_points, &inv_val);
    if (!inv_cnt && !cnt)
    return -EIO;
    }
    if (inv_cnt < cnt) {
    ret = ad9647_calibrate_polarity_set(st, false);
    if (ret)
    return ret;
    } else {
//
// polarity inverted is the last test to run. Hence, there's no
// need to re-do any configuration. We just need to "normalize"
// the selected value.
//
    val = inv_val - test_points;
    }
    if (st.info.has_dco)
    dev_dbg(dev, "%sDCO 0x%X CLK %lu Hz\n", inv_cnt >= cnt ? "INVERT " : "",
    val, sample_rate);
    else
    dev_dbg(dev, "%sIDELAY 0x%x\n", inv_cnt >= cnt ? "INVERT " : "",
    val);
    ret = ad9467_calibrate_apply(st, val);
    if (ret)
    return ret;
// finally apply the optimal value
    return ad9647_calibrate_stop(st);
    }
    static int ad9467_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long m)
    {
    struct ad9467_state *st = iio_priv(indio_dev);
    switch (m) {
    case IIO_CHAN_INFO_CALIBBIAS:
    return ad9467_get_offset(st, val);
    case IIO_CHAN_INFO_SCALE:
    return ad9467_get_scale(st, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = clk_get_rate(st->clk);
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn __ad9467_update_clock(st: *mut ad9467_state, r_clk: c_long) -> c_int {
    static int __ad9467_update_clock(struct ad9467_state *st, long r_clk)
    {
    int ret;
    ret = clk_set_rate(st.clk, r_clk);
    if (ret)
    return ret;
    guard(mutex)(&st.lock);
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION))
    return ad9467_calibrate(st);
    return 0;
    }
    static int ad9467_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct ad9467_state *st = iio_priv(indio_dev);
    const struct ad9467_chip_info *info = st.info;
    unsigned long sample_rate;
    long r_clk;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
    return ad9467_set_offset(st, val);
    case IIO_CHAN_INFO_SCALE:
    return ad9467_set_scale(st, val, val2);
    case IIO_CHAN_INFO_SAMP_FREQ:
    r_clk = clk_round_rate(st.clk, val);
    if (r_clk < 0 || r_clk > info.max_rate) {
    dev_warn(&st.spi.dev,
    "Error setting ADC sample rate %ld", r_clk);
    return -EINVAL;
    }
    sample_rate = clk_get_rate(st.clk);
//
// clk_set_rate() would also do this but since we would still
// need it for avoiding an unnecessary calibration, do it now.
//
    if (sample_rate == r_clk)
    return 0;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = __ad9467_update_clock(st, r_clk);
    iio_device_release_direct(indio_dev);
    return ret;
    default:
    return -EINVAL;
    }
    }
    static int ad9467_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct ad9467_state *st = iio_priv(indio_dev);
    const struct ad9467_chip_info *info = st.info;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
// type = IIO_VAL_INT;
// vals = info->offset_range;
    return IIO_AVAIL_RANGE;
    case IIO_CHAN_INFO_SCALE:
// vals = (const int *)st->scales;
// type = IIO_VAL_INT_PLUS_MICRO;
// Values are stored in a 2D matrix
// length = info->num_scales * 2;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static int ad9467_update_scan_mode(struct iio_dev *indio_dev,
    const unsigned long *scan_mask)
    {
    struct ad9467_state *st = iio_priv(indio_dev);
    unsigned int c;
    int ret;
    for (c = 0; c < st.info.num_channels; c++) {
    if (test_bit(c, scan_mask))
    ret = iio_backend_chan_enable(st.back, c);
    else
    ret = iio_backend_chan_disable(st.back, c);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct iio_info ad9467_info = {
    .read_raw = ad9467_read_raw,
    .write_raw = ad9467_write_raw,
    .update_scan_mode = ad9467_update_scan_mode,
    .debugfs_reg_access = ad9467_reg_access,
    .read_avail = ad9467_read_avail,
    };
// Same as above, but without .read_avail
    static const struct iio_info ad9467_info_no_read_avail = {
    .read_raw = ad9467_read_raw,
    .write_raw = ad9467_write_raw,
    .update_scan_mode = ad9467_update_scan_mode,
    .debugfs_reg_access = ad9467_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn ad9467_scale_fill(st: *mut ad9467_state) -> c_int {
    static int ad9467_scale_fill(struct ad9467_state *st)
    {
    const struct ad9467_chip_info *info = st.info;
    unsigned int i, val1, val2;
    st.scales = devm_kmalloc_array(&st.spi.dev, info.num_scales,
    sizeof(*st.scales), GFP_KERNEL);
    if (!st.scales)
    return -ENOMEM;
    for (i = 0; i < info.num_scales; i++) {
    __ad9467_get_scale(st, i, &val1, &val2);
    st.scales[i][0] = val1;
    st.scales[i][1] = val2;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_reset(dev: *mut device) -> c_int {
    static int ad9467_reset(struct device *dev)
    {
    struct gpio_desc *gpio;
    gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR_OR_NULL(gpio))
    return PTR_ERR_OR_ZERO(gpio);
    fsleep(1);
    gpiod_set_value_cansleep(gpio, 0);
    fsleep(10 * USEC_PER_MSEC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_iio_backend_get(st: *mut ad9467_state) -> c_int {
    static int ad9467_iio_backend_get(struct ad9467_state *st)
    {
    struct device *dev = &st.spi.dev;
    struct device_node *__back;
    st.back = devm_iio_backend_get(dev, core::ptr::null_mut());
    if (!IS_ERR(st.back))
    return 0;
// If not found, don't error out as we might have legacy DT property
    if (PTR_ERR(st.back) != -ENOENT)
    return PTR_ERR(st.back);
//
// if we don't get the backend using the normal API's, use the legacy
// 'adi,adc-dev' property. So we get all nodes with that property, and
// look for the one pointing at us. Then we directly lookup that fwnode
// on the backend list of registered devices. This is done so we don't
// make io-backends mandatory which would break DT ABI.
//
    for_each_node_with_property(__back, "adi,adc-dev") {
    struct device_node *__me;
    __me = of_parse_phandle(__back, "adi,adc-dev", 0);
    if (!__me)
    continue;
    if (!device_match_of_node(dev, __me)) {
    of_node_put(__me);
    continue;
    }
    of_node_put(__me);
    st.back = __devm_iio_backend_get_from_fwnode_lookup(dev,
    of_fwnode_handle(__back));
    of_node_put(__back);
    return PTR_ERR_OR_ZERO(st.back);
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn ad9467_test_mode_available_show(s: *mut seq_file, ignored: *mut c_void) -> c_int {
    static int ad9467_test_mode_available_show(struct seq_file *s, void *ignored)
    {
    struct ad9467_state *st = s.private;
    unsigned int bit;
    for_each_set_bit(bit, &st.info.test_mask, st.info.test_mask_len)
    seq_printf(s, "%s\n", ad9467_test_modes[bit]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ad9467_test_mode_available);
    static ssize_t ad9467_chan_test_mode_read(struct file *file,
    char __user *userbuf, size_t count,
    loff_t *ppos)
    {
    struct ad9467_chan_test_mode *chan = file.private_data;
    struct ad9467_state *st = chan.st;
    char buf[128] = {0};
    size_t len;
    int ret;
    if (chan.mode == AN877_ADC_TESTMODE_PN9_SEQ ||
    chan.mode == AN877_ADC_TESTMODE_PN23_SEQ) {
    len = scnprintf(buf, sizeof(buf), "Running \"%s\" Test:\n\t",
    ad9467_test_modes[chan.mode]);
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION)) {
    ret = iio_backend_debugfs_print_chan_status(st.back,
    chan.idx,
    buf + len,
    sizeof(buf) - len);
    if (ret < 0)
    return ret;
    len += ret;
    }
    } else if (chan.mode == AN877_ADC_TESTMODE_OFF) {
    len = scnprintf(buf, sizeof(buf), "No test Running...\n");
    } else {
    len = scnprintf(buf, sizeof(buf), "Running \"%s\" Test on CH:%u\n",
    ad9467_test_modes[chan.mode], chan.idx);
    }
    return simple_read_from_buffer(userbuf, count, ppos, buf, len);
    }
    static ssize_t ad9467_chan_test_mode_write(struct file *file,
    const char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct ad9467_chan_test_mode *chan = file.private_data;
    struct ad9467_state *st = chan.st;
    char test_mode[32] = {0};
    unsigned int mode;
    int ret;
    ret = simple_write_to_buffer(test_mode, sizeof(test_mode) - 1, ppos,
    userbuf, count);
    if (ret < 0)
    return ret;
    for_each_set_bit(mode, &st.info.test_mask, st.info.test_mask_len) {
    if (sysfs_streq(test_mode, ad9467_test_modes[mode]))
    break;
    }
    if (mode == st.info.test_mask_len)
    return -EINVAL;
    guard(mutex)(&st.lock);
    if (mode == AN877_ADC_TESTMODE_OFF) {
    unsigned int out_mode;
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION)) {
    if (chan.mode == AN877_ADC_TESTMODE_PN9_SEQ ||
    chan.mode == AN877_ADC_TESTMODE_PN23_SEQ) {
    ret = ad9467_backend_testmode_off(st, chan.idx);
    if (ret)
    return ret;
    }
    }
    ret = ad9467_testmode_set(st, chan.idx, mode);
    if (ret)
    return ret;
    out_mode = st.info.default_output_mode;
    ret = ad9467_outputmode_set(st, out_mode);
    if (ret)
    return ret;
    } else {
    unsigned int cmode;
    cmode = st.info.default_output_mode;
    FIELD_MODIFY(AN877_ADC_OUTPUT_MODE_MASK, &cmode,
    AN877_ADC_OUTPUT_MODE_OFFSET_BINARY);
    ret = ad9467_outputmode_set(st, cmode);
    if (ret)
    return ret;
    ret = ad9467_testmode_set(st, chan.idx, mode);
    if (ret)
    return ret;
// some patterns have a backend matching monitoring block
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION)) {
    if (mode == AN877_ADC_TESTMODE_PN9_SEQ) {
    ret = ad9467_backend_testmode_on(st, chan.idx,
    IIO_BACKEND_ADI_PRBS_9A);
    if (ret)
    return ret;
    } else if (mode == AN877_ADC_TESTMODE_PN23_SEQ) {
    ret = ad9467_backend_testmode_on(st, chan.idx,
    IIO_BACKEND_ADI_PRBS_23A);
    if (ret)
    return ret;
    }
    }
    }
    chan.mode = mode;
    return count;
    }
    static const struct file_operations ad9467_chan_test_mode_fops = {
    .open = simple_open,
    .read = ad9467_chan_test_mode_read,
    .write = ad9467_chan_test_mode_write,
    .llseek = default_llseek,
    .owner = THIS_MODULE,
    };
    static ssize_t ad9467_dump_calib_table(struct file *file,
    char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct ad9467_state *st = file.private_data;
    unsigned int bit;
// +2 for the newline and +1 for the string termination
    unsigned char map[AD9647_MAX_TEST_POINTS * 2 + 3];
    let mut len: isize = 0;
    guard(mutex)(&st.lock);
    if (*ppos)
    goto out_read;
    for (bit = 0; bit < st.calib_map_size; bit++) {
    if (AD9467_CAN_INVERT(st) && bit == st.calib_map_size / 2)
    len += scnprintf(map + len, sizeof(map) - len, "\n");
    len += scnprintf(map + len, sizeof(map) - len, "%c",
    test_bit(bit, st.calib_map) ? 'x' : 'o');
    }
    len += scnprintf(map + len, sizeof(map) - len, "\n");
    out_read:
    return simple_read_from_buffer(userbuf, count, ppos, map, len);
    }
    static const struct file_operations ad9467_calib_table_fops = {
    .open = simple_open,
    .read = ad9467_dump_calib_table,
    .llseek = default_llseek,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ad9467_debugfs_init(indio_dev: *mut iio_dev) {
    static void ad9467_debugfs_init(struct iio_dev *indio_dev)
    {
    struct dentry *d = iio_get_debugfs_dentry(indio_dev);
    struct ad9467_state *st = iio_priv(indio_dev);
    char attr_name[32];
    unsigned int chan;
    if (!IS_ENABLED(CONFIG_DEBUG_FS))
    return;
    st.chan_test = devm_kcalloc(&st.spi.dev, st.info.num_channels,
    sizeof(*st.chan_test), GFP_KERNEL);
    if (!st.chan_test)
    return;
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION))
    debugfs_create_file("calibration_table_dump", 0400, d, st,
    &ad9467_calib_table_fops);
    for (chan = 0; chan < st.info.num_channels; chan++) {
    snprintf(attr_name, sizeof(attr_name), "in_voltage%u_test_mode",
    chan);
    st.chan_test[chan].idx = chan;
    st.chan_test[chan].st = st;
    debugfs_create_file(attr_name, 0600, d, &st.chan_test[chan],
    &ad9467_chan_test_mode_fops);
    }
    debugfs_create_file("in_voltage_test_mode_available", 0400, d, st,
    &ad9467_test_mode_available_fops);
    iio_backend_debugfs_add(st.back, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn ad9467_probe(spi: *mut spi_device) -> c_int {
    static int ad9467_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct ad9467_state *st;
    unsigned int id;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.info = spi_get_device_match_data(spi);
    if (!st.info)
    return -ENODEV;
    st.calib_map_size = st.info.test_points;
    if (AD9467_CAN_INVERT(st))
    st.calib_map_size *= 2;
    st.clk = devm_clk_get_enabled(dev, "adc-clk");
    if (IS_ERR(st.clk))
    return PTR_ERR(st.clk);
    st.pwrdown_gpio = devm_gpiod_get_optional(dev, "powerdown", GPIOD_OUT_LOW);
    if (IS_ERR(st.pwrdown_gpio))
    return PTR_ERR(st.pwrdown_gpio);
    ret = ad9467_reset(dev);
    if (ret)
    return ret;
    ret = ad9467_scale_fill(st);
    if (ret)
    return ret;
    id = ad9467_spi_read(st, AN877_ADC_REG_CHIP_ID);
    if (id != st.info.id)
    return dev_err_probe(dev, -ENODEV,
    "Mismatch CHIP_ID, got 0x%X, expected 0x%X\n",
    id, st.info.id);
    if (st.info.num_scales > 1)
    indio_dev.info = &ad9467_info;
    else
    indio_dev.info = &ad9467_info_no_read_avail;
    indio_dev.name = st.info.name;
    indio_dev.channels = st.info.channels;
    indio_dev.num_channels = st.info.num_channels;
    ret = ad9467_iio_backend_get(st);
    if (ret)
    return ret;
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_BUFFER)) {
    ret = devm_iio_backend_request_buffer(dev, st.back, indio_dev);
    if (ret)
    return ret;
    }
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_ENABLE)) {
    ret = devm_iio_backend_enable(dev, st.back);
    if (ret)
    return ret;
    }
    if (iio_backend_has_caps(st.back, IIO_BACKEND_CAP_CALIBRATION)) {
    ret = ad9467_calibrate(st);
    if (ret)
    return ret;
    }
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret)
    return ret;
    ad9467_debugfs_init(indio_dev);
    return 0;
    }
    static const struct of_device_id ad9467_of_match[] = {
    { .compatible = "adi,ad9211", .data = &ad9211_chip_tbl, },
    { .compatible = "adi,ad9265", .data = &ad9265_chip_tbl, },
    { .compatible = "adi,ad9434", .data = &ad9434_chip_tbl, },
    { .compatible = "adi,ad9467", .data = &ad9467_chip_tbl, },
    { .compatible = "adi,ad9643", .data = &ad9643_chip_tbl, },
    { .compatible = "adi,ad9649", .data = &ad9649_chip_tbl, },
    { .compatible = "adi,ad9652", .data = &ad9652_chip_tbl, },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad9467_of_match);
    static const struct spi_device_id ad9467_ids[] = {
    { .name = "ad9211", .driver_data = (kernel_ulong_t)&ad9211_chip_tbl },
    { .name = "ad9265", .driver_data = (kernel_ulong_t)&ad9265_chip_tbl },
    { .name = "ad9434", .driver_data = (kernel_ulong_t)&ad9434_chip_tbl },
    { .name = "ad9467", .driver_data = (kernel_ulong_t)&ad9467_chip_tbl },
    { .name = "ad9643", .driver_data = (kernel_ulong_t)&ad9643_chip_tbl },
    { .name = "ad9649", .driver_data = (kernel_ulong_t)&ad9649_chip_tbl },
    { .name = "ad9652", .driver_data = (kernel_ulong_t)&ad9652_chip_tbl },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad9467_ids);
    static struct spi_driver ad9467_driver = {
    .driver = {
    .name = "ad9467",
    .of_match_table = ad9467_of_match,
    },
    .probe = ad9467_probe,
    .id_table = ad9467_ids,
    };
    module_spi_driver(ad9467_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD9467 ADC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BACKEND");
