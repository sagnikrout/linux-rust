//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/mcp4821.c
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
// Copyright (C) 2023 Anshul Dalal <anshulusr@gmail.com>
//
// Driver for Microchip MCP4801, MCP4802, MCP4811, MCP4812, MCP4821 and MCP4822
//
// Based on the work of:
// Michael Welling (MCP4922 Driver)
//
// Datasheet:
// MCP48x1: https://ww1.microchip.com/downloads/en/DeviceDoc/22244B.pdf
// MCP48x2: https://ww1.microchip.com/downloads/en/DeviceDoc/20002249B.pdf
//
// TODO:
// - Regulator control
//

// DAC uses an internal Voltage reference of 2.048V
pub const MCP4821_VREF_MV: c_int = 2048;
//
// MCP48xx DAC output:
//
// Vout = (Vref * D / 2^N) * G
//
// where:
// - Vref = 2.048V (internal reference)
// - N = DAC resolution (12 bits for MCP4821)
// - G = gain selection:
// 1x when GA bit = 1
// 2x when GA bit = 0 (default)
//
// Therefore full-scale voltage is:
// - 1x gain: 2.048V
// - 2x gain: 4.096V
//
// Scale = Vfull-scale / 2^N
//
    enum mcp4821_supported_device_ids {
    ID_MCP4801,
    ID_MCP4802,
    ID_MCP4811,
    ID_MCP4812,
    ID_MCP4821,
    ID_MCP4822,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp4821_state {
    pub spi: *mut spi_device,
    pub dac_value: [u16; 2],
    pub gain: c_int,
    pub scale_avail: [c_int; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp4821_chip_info {
    pub name: *const c_char,
    pub num_channels: c_int,
    pub channels: [iio_chan_spec; 2],
}

    {                                                             \
    .type = IIO_VOLTAGE, .output = 1, .indexed = 1,       \
    .channel = (channel_id),                              \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),         \
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE), \
    .info_mask_shared_by_type_available = BIT(IIO_CHAN_INFO_SCALE), \
    .scan_type = {                                        \
    .realbits = (resolution),                     \
    .shift = 12 - (resolution),                   \
    },                                                    \
    }
    static const struct mcp4821_chip_info mcp4821_chip_info_table[6] = {
    [ID_MCP4801] = {
    .name = "mcp4801",
    .num_channels = 1,
    .channels = {
    MCP4821_CHAN(0, 8),
    },
    },
    [ID_MCP4802] = {
    .name = "mcp4802",
    .num_channels = 2,
    .channels = {
    MCP4821_CHAN(0, 8),
    MCP4821_CHAN(1, 8),
    },
    },
    [ID_MCP4811] = {
    .name = "mcp4811",
    .num_channels = 1,
    .channels = {
    MCP4821_CHAN(0, 10),
    },
    },
    [ID_MCP4812] = {
    .name = "mcp4812",
    .num_channels = 2,
    .channels = {
    MCP4821_CHAN(0, 10),
    MCP4821_CHAN(1, 10),
    },
    },
    [ID_MCP4821] = {
    .name = "mcp4821",
    .num_channels = 1,
    .channels = {
    MCP4821_CHAN(0, 12),
    },
    },
    [ID_MCP4822] = {
    .name = "mcp4822",
    .num_channels = 2,
    .channels = {
    MCP4821_CHAN(0, 12),
    MCP4821_CHAN(1, 12),
    },
    },
    };
    static int mcp4821_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct mcp4821_state *state = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// val = state->dac_value[chan->channel];
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = MCP4821_VREF_MV * state->gain;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    }
    static void mcp4821_calc_scale(int vref_mv, int resolution,
    int *val, int *val2)
    {
    s64 tmp;
    int micro;
    tmp = (s64)vref_mv * MICRO >> resolution;
// val = div_s64_rem(tmp, MICRO, &micro);
// val2 = micro;
    }
    static int mcp4821_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int val,
    int val2, long mask)
    {
    struct mcp4821_state *state = iio_priv(indio_dev);
    u16 write_val;
    __be16 write_buffer;
    int ret;
    int v, v2;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (val2 != 0)
    return -EINVAL;
    if (val < 0 || val >= BIT(chan.scan_type.realbits))
    return -EINVAL;
    write_val = MCP4821_ACTIVE_MODE | val << chan.scan_type.shift;
    if (chan.channel)
    write_val |= MCP4802_SECOND_CHAN;
// GA bit = 1 -> 1x gain
    if (state.gain == 1)
    write_val |= MCP4821_GAIN_ENABLE;
    write_buffer = cpu_to_be16(write_val);
    ret = spi_write(state.spi, &write_buffer, sizeof(write_buffer));
    if (ret) {
    dev_err(&state.spi.dev, "Failed to write to device: %d", ret);
    return ret;
    }
    state.dac_value[chan.channel] = val;
    return 0;
    case IIO_CHAN_INFO_SCALE:
    mcp4821_calc_scale(MCP4821_VREF_MV, chan.scan_type.realbits, &v, &v2);
    if (val == v && val2 == v2) {
    state.gain = 1;
    return 0;
    }
    mcp4821_calc_scale(MCP4821_VREF_MV * 2,
    chan.scan_type.realbits, &v, &v2);
    if (val == v && val2 == v2) {
    state.gain = 2;
    return 0;
    }
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static inline void mcp4821_init_avail_gain(struct mcp4821_state *state,
    int resolution)
    {
    state.scale_avail[0] = MCP4821_VREF_MV;
    state.scale_avail[1] = resolution;
    state.scale_avail[2] = MCP4821_VREF_MV * 2;
    state.scale_avail[3] = resolution;
    }
    static int mcp4821_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long info)
    {
    struct mcp4821_state *state = iio_priv(indio_dev);
    switch (info) {
    case IIO_CHAN_INFO_SCALE:
// vals = state->scale_avail;
// type = IIO_VAL_FRACTIONAL_LOG2;
// length = ARRAY_SIZE(state->scale_avail);
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info mcp4821_info = {
    .read_raw = &mcp4821_read_raw,
    .write_raw = &mcp4821_write_raw,
    .read_avail = &mcp4821_read_avail,
    };
#[no_mangle]
unsafe extern "C" fn mcp4821_probe(spi: *mut spi_device) -> c_int {
    static int mcp4821_probe(struct spi_device *spi)
    {
    struct iio_dev *indio_dev;
    struct mcp4821_state *state;
    const struct mcp4821_chip_info *info;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*state));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    state = iio_priv(indio_dev);
    state.spi = spi;
// default gain is 2x
    state.gain = 2;
    info = spi_get_device_match_data(spi);
    indio_dev.name = info.name;
    indio_dev.info = &mcp4821_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = info.channels;
    indio_dev.num_channels = info.num_channels;
    mcp4821_init_avail_gain(state, info.channels[0].scan_type.realbits);
    return devm_iio_device_register(&spi.dev, indio_dev);
    }

    {                                            \
    .compatible = of_compatible,         \
    .data = &mcp4821_chip_info_table[id] \
    }
    static const struct of_device_id mcp4821_of_table[] = {
    MCP4821_COMPATIBLE("microchip,mcp4801", ID_MCP4801),
    MCP4821_COMPATIBLE("microchip,mcp4802", ID_MCP4802),
    MCP4821_COMPATIBLE("microchip,mcp4811", ID_MCP4811),
    MCP4821_COMPATIBLE("microchip,mcp4812", ID_MCP4812),
    MCP4821_COMPATIBLE("microchip,mcp4821", ID_MCP4821),
    MCP4821_COMPATIBLE("microchip,mcp4822", ID_MCP4822),
    { }
    };
    MODULE_DEVICE_TABLE(of, mcp4821_of_table);
    static const struct spi_device_id mcp4821_id_table[] = {
    { .name = "mcp4801", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4801] },
    { .name = "mcp4802", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4802] },
    { .name = "mcp4811", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4811] },
    { .name = "mcp4812", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4812] },
    { .name = "mcp4821", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4821] },
    { .name = "mcp4822", .driver_data = (kernel_ulong_t)&mcp4821_chip_info_table[ID_MCP4822] },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mcp4821_id_table);
    static struct spi_driver mcp4821_driver = {
    .driver = {
    .name = "mcp4821",
    .of_match_table = mcp4821_of_table,
    },
    .probe = mcp4821_probe,
    .id_table = mcp4821_id_table,
    };
    module_spi_driver(mcp4821_driver);
    MODULE_AUTHOR("Anshul Dalal <anshulusr@gmail.com>");
    MODULE_DESCRIPTION("Microchip MCP4821 DAC Driver");
    MODULE_LICENSE("GPL");
