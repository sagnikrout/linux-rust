//! Automatically rewritten from C to Rust
//! Source: drivers/iio/frequency/adf4371.c
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
// Analog Devices ADF4371 SPI Wideband Synthesizer driver
//
// Copyright 2019 Analog Devices Inc.
//

// Registers address macro

// ADF4371_REG0

pub const ADF4371_RESET_CMD: c_uint = 0x81;
// ADF4371_REG17

// ADF4371_REG18

// ADF4371_REG1A

// ADF4371_REG22

// ADF4371_REG24

// ADF4371_REG25

// ADF4371_REG32

// ADF4371_REG34

// Specifications

// MOD1 is a 24-bit primary modulus with fixed value of 2^25

// MOD2 is the programmable, 14-bit auxiliary fractional modulus

    ((freq > ADF4371_MAX_ ## range) || (freq < ADF4371_MIN_ ## range))
    enum {
    ADF4371_FREQ,
    ADF4371_POWER_DOWN,
    ADF4371_CHANNEL_NAME
    };
    enum {
    ADF4371_CH_RF8,
    ADF4371_CH_RFAUX8,
    ADF4371_CH_RF16,
    ADF4371_CH_RF32
    };
    enum adf4371_variant {
    ADF4371,
    ADF4372
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4371_pwrdown {
    pub reg: c_uint,
    pub bit: c_uint,
}

    static const char * const adf4371_ch_names[] = {
    "RF8x", "RFAUX8x", "RF16x", "RF32x"
    };
    static const struct adf4371_pwrdown adf4371_pwrdown_ch[4] = {
    [ADF4371_CH_RF8] = { ADF4371_REG(0x25), 2 },
    [ADF4371_CH_RFAUX8] = { ADF4371_REG(0x72), 3 },
    [ADF4371_CH_RF16] = { ADF4371_REG(0x25), 3 },
    [ADF4371_CH_RF32] = { ADF4371_REG(0x25), 4 },
    };
    static const struct reg_sequence adf4371_reg_defaults[] = {
    { ADF4371_REG(0x0),  0x18 },
    { ADF4371_REG(0x12), 0x40 },
    { ADF4371_REG(0x1E), 0x48 },
    { ADF4371_REG(0x20), 0x14 },
    { ADF4371_REG(0x22), 0x00 },
    { ADF4371_REG(0x23), 0x00 },
    { ADF4371_REG(0x24), 0x80 },
    { ADF4371_REG(0x25), 0x07 },
    { ADF4371_REG(0x27), 0xC5 },
    { ADF4371_REG(0x28), 0x83 },
    { ADF4371_REG(0x2C), 0x44 },
    { ADF4371_REG(0x2D), 0x11 },
    { ADF4371_REG(0x2E), 0x12 },
    { ADF4371_REG(0x2F), 0x94 },
    { ADF4371_REG(0x32), 0x04 },
    { ADF4371_REG(0x35), 0xFA },
    { ADF4371_REG(0x36), 0x30 },
    { ADF4371_REG(0x39), 0x07 },
    { ADF4371_REG(0x3A), 0x55 },
    { ADF4371_REG(0x3E), 0x0C },
    { ADF4371_REG(0x3F), 0x80 },
    { ADF4371_REG(0x40), 0x50 },
    { ADF4371_REG(0x41), 0x28 },
    { ADF4371_REG(0x47), 0xC0 },
    { ADF4371_REG(0x52), 0xF4 },
    { ADF4371_REG(0x70), 0x03 },
    { ADF4371_REG(0x71), 0x60 },
    { ADF4371_REG(0x72), 0x32 },
    };
    static const struct regmap_config adf4371_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .read_flag_mask = BIT(7),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4371_chip_info {
    pub name: *const c_char,
    pub num_channels: c_uint,
    pub channels: *const iio_chan_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf4371_state {
    pub spi: *mut spi_device,
    pub regmap: *mut regmap,
//
// Lock for accessing device registers. Some operations require
// multiple consecutive R/W operations, during which the device
// shouldn't be interrupted. The buffers are also shared across
// all operations so need to be protected on stand alone reads and
// writes.
//
    pub lock: mutex,
    pub chip_info: *const adf4371_chip_info,
    pub clkin_freq: c_ulong,
    pub fpfd: c_ulong,
    pub integer: c_uint,
    pub fract1: c_uint,
    pub fract2: c_uint,
    pub mod2: c_uint,
    pub rf_div_sel: c_uint,
    pub ref_div_factor: c_uint,
    pub ref_diff_en: bool,
    pub __aligned(IIO_DMA_MINALIGN): u8 buf[10],
}

    static unsigned long long adf4371_pll_fract_n_get_rate(struct adf4371_state *st,
    u32 channel)
    {
    unsigned long long val, tmp;
    unsigned int ref_div_sel;
    val = (((u64)st.integer * ADF4371_MODULUS1) + st.fract1) * st.fpfd;
    tmp = (u64)st.fract2 * st.fpfd;
    do_div(tmp, st.mod2);
    val += tmp + ADF4371_MODULUS1 / 2;
    if (channel == ADF4371_CH_RF8 || channel == ADF4371_CH_RFAUX8)
    ref_div_sel = st.rf_div_sel;
    else
    ref_div_sel = 0;
    do_div(val, ADF4371_MODULUS1 * (1 << ref_div_sel));
    if (channel == ADF4371_CH_RF16)
    val <<= 1;
#[no_mangle]
pub unsafe extern "C" fn if(ADF4371_CH_RF32: channel ==) -> else {
    else if (channel == ADF4371_CH_RF32)
    val <<= 2;
    return val;
    }
    static void adf4371_pll_fract_n_compute(unsigned long long vco,
    unsigned long long pfd,
    unsigned int *integer,
    unsigned int *fract1,
    unsigned int *fract2,
    unsigned int *mod2)
    {
    unsigned long long tmp;
    u32 gcd_div;
    tmp = do_div(vco, pfd);
    tmp = tmp * ADF4371_MODULUS1;
// fract2 = do_div(tmp, pfd);
// integer = vco;
// fract1 = tmp;
// mod2 = pfd;
    while (*mod2 > ADF4371_MAX_MODULUS2) {
// mod2 >>= 1;
// fract2 >>= 1;
    }
    gcd_div = gcd(*fract2, *mod2);
// mod2 /= gcd_div;
// fract2 /= gcd_div;
    }
    static int adf4371_set_freq(struct adf4371_state *st, unsigned long long freq,
    unsigned int channel)
    {
    u32 cp_bleed;
    let mut int_mode: u8 = 0;
    int ret;
    switch (channel) {
    case ADF4371_CH_RF8:
    case ADF4371_CH_RFAUX8:
    if (ADF4371_CHECK_RANGE(freq, OUT_RF8_FREQ))
    return -EINVAL;
    st.rf_div_sel = 0;
    while (freq < ADF4371_MIN_VCO_FREQ) {
    freq <<= 1;
    st.rf_div_sel++;
    }
    break;
    case ADF4371_CH_RF16:
// ADF4371 RF16 8000...16000 MHz
    if (ADF4371_CHECK_RANGE(freq, OUT_RF16_FREQ))
    return -EINVAL;
    freq >>= 1;
    break;
    case ADF4371_CH_RF32:
// ADF4371 RF32 16000...32000 MHz
    if (ADF4371_CHECK_RANGE(freq, OUT_RF32_FREQ))
    return -EINVAL;
    freq >>= 2;
    break;
    default:
    return -EINVAL;
    }
    adf4371_pll_fract_n_compute(freq, st.fpfd, &st.integer, &st.fract1,
    &st.fract2, &st.mod2);
    st.buf[0] = st.integer >> 8;
    st.buf[1] = 0x40; /* REG12 default */
    st.buf[2] = 0x00;
    st.buf[3] = st.fract1 & 0xFF;
    st.buf[4] = st.fract1 >> 8;
    st.buf[5] = st.fract1 >> 16;
    st.buf[6] = ADF4371_FRAC2WORD_L(st.fract2 & 0x7F) |
    ADF4371_FRAC1WORD(st.fract1 >> 24);
    st.buf[7] = ADF4371_FRAC2WORD_H(st.fract2 >> 7);
    st.buf[8] = st.mod2 & 0xFF;
    st.buf[9] = ADF4371_MOD2WORD(st.mod2 >> 8);
    ret = regmap_bulk_write(st.regmap, ADF4371_REG(0x11), st.buf, 10);
    if (ret < 0)
    return ret;
//
// The R counter allows the input reference frequency to be
// divided down to produce the reference clock to the PFD
//
    ret = regmap_write(st.regmap, ADF4371_REG(0x1F), st.ref_div_factor);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(st.regmap, ADF4371_REG(0x24),
    ADF4371_RF_DIV_SEL_MSK,
    ADF4371_RF_DIV_SEL(st.rf_div_sel));
    if (ret < 0)
    return ret;
    cp_bleed = DIV_ROUND_UP(400 * 1750, st.integer * 375);
    cp_bleed = clamp(cp_bleed, 1U, 255U);
    ret = regmap_write(st.regmap, ADF4371_REG(0x26), cp_bleed);
    if (ret < 0)
    return ret;
//
// Set to 1 when in INT mode (when FRAC1 = FRAC2 = 0),
// and set to 0 when in FRAC mode.
//
    if (st.fract1 == 0 && st.fract2 == 0)
    int_mode = 0x01;
    ret = regmap_write(st.regmap, ADF4371_REG(0x2B), int_mode);
    if (ret < 0)
    return ret;
    return regmap_write(st.regmap, ADF4371_REG(0x10), st.integer & 0xFF);
    }
    static ssize_t adf4371_read(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    char *buf)
    {
    struct adf4371_state *st = iio_priv(indio_dev);
    let mut val: c_ulonglong = 0;
    unsigned int readval, reg, bit;
    int ret;
    switch ((u32)private) {
    case ADF4371_FREQ:
    val = adf4371_pll_fract_n_get_rate(st, chan.channel);
    ret = regmap_read(st.regmap, ADF4371_REG(0x7C), &readval);
    if (ret < 0)
    break;
    if (readval == 0x00) {
    dev_dbg(&st.spi.dev, "PLL un-locked\n");
    ret = -EBUSY;
    }
    break;
    case ADF4371_POWER_DOWN:
    reg = adf4371_pwrdown_ch[chan.channel].reg;
    bit = adf4371_pwrdown_ch[chan.channel].bit;
    ret = regmap_read(st.regmap, reg, &readval);
    if (ret < 0)
    break;
    val = !(readval & BIT(bit));
    break;
    case ADF4371_CHANNEL_NAME:
    return sprintf(buf, "%s\n", adf4371_ch_names[chan.channel]);
    default:
    ret = -EINVAL;
    val = 0;
    break;
    }
    return ret < 0 ? ret : sprintf(buf, "%llu\n", val);
    }
    static ssize_t adf4371_write(struct iio_dev *indio_dev,
    uintptr_t private,
    const struct iio_chan_spec *chan,
    const char *buf, size_t len)
    {
    struct adf4371_state *st = iio_priv(indio_dev);
    unsigned long long freq;
    bool power_down;
    unsigned int bit, readval, reg;
    int ret;
    mutex_lock(&st.lock);
    switch ((u32)private) {
    case ADF4371_FREQ:
    ret = kstrtoull(buf, 10, &freq);
    if (ret)
    break;
    ret = adf4371_set_freq(st, freq, chan.channel);
    break;
    case ADF4371_POWER_DOWN:
    ret = kstrtobool(buf, &power_down);
    if (ret)
    break;
    reg = adf4371_pwrdown_ch[chan.channel].reg;
    bit = adf4371_pwrdown_ch[chan.channel].bit;
    ret = regmap_read(st.regmap, reg, &readval);
    if (ret < 0)
    break;
    readval &= ~BIT(bit);
    readval |= (!power_down << bit);
    ret = regmap_write(st.regmap, reg, readval);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    mutex_unlock(&st.lock);
    return ret ? ret : len;
    }

    .name = _name, \
    .read = adf4371_read, \
    .write = adf4371_write, \
    .private = _ident, \
    .shared = IIO_SEPARATE, \
    }
    static const struct iio_chan_spec_ext_info adf4371_ext_info[] = {
//
// Ideally we use IIO_CHAN_INFO_FREQUENCY, but there are
// values > 2^32 in order to support the entire frequency range
// in Hz. Using scale is a bit ugly.
//
    _ADF4371_EXT_INFO("frequency", ADF4371_FREQ),
    _ADF4371_EXT_INFO("powerdown", ADF4371_POWER_DOWN),
    _ADF4371_EXT_INFO("name", ADF4371_CHANNEL_NAME),
    { }
    };

    .type = IIO_ALTVOLTAGE, \
    .output = 1, \
    .channel = index, \
    .ext_info = adf4371_ext_info, \
    .indexed = 1, \
    }
    static const struct iio_chan_spec adf4371_chan[] = {
    ADF4371_CHANNEL(ADF4371_CH_RF8),
    ADF4371_CHANNEL(ADF4371_CH_RFAUX8),
    ADF4371_CHANNEL(ADF4371_CH_RF16),
    ADF4371_CHANNEL(ADF4371_CH_RF32),
    };
    static const struct adf4371_chip_info adf4371_chip_info = {
    .name = "adf4371",
    .channels = adf4371_chan,
    .num_channels = 4,
    };
    static const struct adf4371_chip_info adf4372_chip_info = {
    .name = "adf4372",
    .channels = adf4371_chan,
    .num_channels = 3,
    };
    static int adf4371_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int writeval,
    unsigned int *readval)
    {
    struct adf4371_state *st = iio_priv(indio_dev);
    if (readval)
    return regmap_read(st.regmap, reg, readval);
    else
    return regmap_write(st.regmap, reg, writeval);
    }
    static const struct iio_info adf4371_info = {
    .debugfs_reg_access = &adf4371_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn adf4371_setup(st: *mut adf4371_state) -> c_int {
    static int adf4371_setup(struct adf4371_state *st)
    {
    let mut synth_timeout: c_uint = 2, timeout = 1, vco_alc_timeout = 1;
    unsigned int vco_band_div, tmp, ref_doubler_en = 0;
    int ret;
// Perform a software reset
    ret = regmap_write(st.regmap, ADF4371_REG(0x0), ADF4371_RESET_CMD);
    if (ret < 0)
    return ret;
    ret = regmap_multi_reg_write(st.regmap, adf4371_reg_defaults,
    ARRAY_SIZE(adf4371_reg_defaults));
    if (ret < 0)
    return ret;
// Mute to Lock Detect
    if (device_property_read_bool(&st.spi.dev, "adi,mute-till-lock-en")) {
    ret = regmap_update_bits(st.regmap, ADF4371_REG(0x25),
    ADF4371_MUTE_LD_MSK,
    ADF4371_MUTE_LD(1));
    if (ret < 0)
    return ret;
    }
// Set address in ascending order, so the bulk_write() will work
    ret = regmap_update_bits(st.regmap, ADF4371_REG(0x0),
    ADF4371_ADDR_ASC_MSK | ADF4371_ADDR_ASC_R_MSK,
    ADF4371_ADDR_ASC(1) | ADF4371_ADDR_ASC_R(1));
    if (ret < 0)
    return ret;
    if ((st.ref_diff_en && st.clkin_freq > ADF4371_MAX_FREQ_REFIN) ||
    (!st.ref_diff_en && st.clkin_freq > ADF4371_MAX_FREQ_REFIN_SE))
    return -EINVAL;
    if (st.clkin_freq < ADF4371_MAX_CLKIN_DOUB_FREQ &&
    st.clkin_freq > ADF4371_MIN_CLKIN_DOUB_FREQ)
    ref_doubler_en = 1;
    ret = regmap_update_bits(st.regmap,  ADF4371_REG(0x22),
    ADF4371_REF_DOUB_MASK |
    ADF4371_REFIN_MODE_MASK,
    ADF4371_REF_DOUB(ref_doubler_en) |
    ADF4371_REFIN_MODE(st.ref_diff_en));
    if (ret < 0)
    return ret;
//
// Calculate and maximize PFD frequency
// fPFD = REFIN × ((1 + D)/(R × (1 + T)))
// Where D is the REFIN doubler bit, T is the reference divide by 2,
// R is the reference division factor
// TODO: it is assumed D and T equal 0.
//
    do {
    st.ref_div_factor++;
    st.fpfd = st.clkin_freq * (1 + ref_doubler_en) /
    st.ref_div_factor;
    } while (st.fpfd > ADF4371_MAX_FREQ_PFD);
// Calculate Timeouts
    vco_band_div = DIV_ROUND_UP(st.fpfd, 2400000U);
    tmp = DIV_ROUND_CLOSEST(st.fpfd, 1000000U);
    do {
    timeout++;
    if (timeout > 1023) {
    timeout = 2;
    synth_timeout++;
    }
    } while (synth_timeout * 1024 + timeout <= 20 * tmp);
    do {
    vco_alc_timeout++;
    } while (vco_alc_timeout * 1024 - timeout <= 50 * tmp);
    st.buf[0] = vco_band_div;
    st.buf[1] = timeout & 0xFF;
    st.buf[2] = ADF4371_TIMEOUT(timeout >> 8) | 0x04;
    st.buf[3] = synth_timeout;
    st.buf[4] = ADF4371_VCO_ALC_TOUT(vco_alc_timeout);
    return regmap_bulk_write(st.regmap, ADF4371_REG(0x30), st.buf, 5);
    }
#[no_mangle]
unsafe extern "C" fn adf4371_probe(spi: *mut spi_device) -> c_int {
    static int adf4371_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct adf4371_state *st;
    struct regmap *regmap;
    struct clk *clkin;
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    regmap = devm_regmap_init_spi(spi, &adf4371_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&spi.dev, PTR_ERR(regmap),
    "Error initializing spi regmap\n");
    st = iio_priv(indio_dev);
    st.spi = spi;
    st.regmap = regmap;
    mutex_init(&st.lock);
    st.chip_info = spi_get_device_match_data(spi);
    if (!st.chip_info)
    return -ENODEV;
    indio_dev.name = st.chip_info.name;
    indio_dev.info = &adf4371_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = st.chip_info.channels;
    indio_dev.num_channels = st.chip_info.num_channels;
    st.ref_diff_en = false;
    clkin = devm_clk_get_enabled(&spi.dev, "clkin");
    if (IS_ERR(clkin)) {
    clkin = devm_clk_get_enabled(&spi.dev, "clkin-diff");
    if (IS_ERR(clkin))
    return dev_err_probe(&spi.dev, PTR_ERR(clkin),
    "Failed to get clkin/clkin-diff\n");
    st.ref_diff_en = true;
    }
    st.clkin_freq = clk_get_rate(clkin);
    ret = adf4371_setup(st);
    if (ret < 0)
    return dev_err_probe(&spi.dev, ret, "ADF4371 setup failed\n");
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id adf4371_id_table[] = {
    { .name = "adf4371", .driver_data = (kernel_ulong_t)&adf4371_chip_info },
    { .name = "adf4372", .driver_data = (kernel_ulong_t)&adf4372_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adf4371_id_table);
    static const struct of_device_id adf4371_of_match[] = {
    { .compatible = "adi,adf4371", .data = &adf4371_chip_info },
    { .compatible = "adi,adf4372", .data = &adf4372_chip_info},
    { }
    };
    MODULE_DEVICE_TABLE(of, adf4371_of_match);
    static struct spi_driver adf4371_driver = {
    .driver = {
    .name = "adf4371",
    .of_match_table = adf4371_of_match,
    },
    .probe = adf4371_probe,
    .id_table = adf4371_id_table,
    };
    module_spi_driver(adf4371_driver);
    MODULE_AUTHOR("Stefan Popa <stefan.popa@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADF4371 SPI PLL");
    MODULE_LICENSE("GPL");
