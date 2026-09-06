//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-protocol-v2.c
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
// motu-protocol-v2.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

pub const V2_CLOCK_STATUS_OFFSET: c_uint = 0x0b14;
pub const V2_CLOCK_RATE_MASK: c_uint = 0x00000038;
pub const V2_CLOCK_RATE_SHIFT: c_int = 3;
pub const V2_CLOCK_SRC_MASK: c_uint = 0x00000007;
pub const V2_CLOCK_SRC_SHIFT: c_int = 0;
pub const V2_CLOCK_SRC_AESEBU_ON_XLR: c_uint = 0x07	// In Traveler.;
pub const V2_CLOCK_SRC_ADAT_ON_DSUB: c_uint = 0x05;
pub const V2_CLOCK_SRC_WORD_ON_BNC: c_uint = 0x04;
pub const V2_CLOCK_SRC_SPH: c_uint = 0x03;
pub const V2_CLOCK_SRC_SPDIF: c_uint = 0x02	// on either coaxial or optical. AES/EBU in 896HD.;
pub const V2_CLOCK_SRC_ADAT_ON_OPT: c_uint = 0x01;
pub const V2_CLOCK_SRC_INTERNAL: c_uint = 0x00;
pub const V2_CLOCK_FETCH_ENABLE: c_uint = 0x02000000;
pub const V2_CLOCK_MODEL_SPECIFIC: c_uint = 0x04000000;
pub const V2_IN_OUT_CONF_OFFSET: c_uint = 0x0c04;
pub const V2_OPT_OUT_IFACE_MASK: c_uint = 0x00000c00;
pub const V2_OPT_OUT_IFACE_SHIFT: c_int = 10;
pub const V2_OPT_IN_IFACE_MASK: c_uint = 0x00000300;
pub const V2_OPT_IN_IFACE_SHIFT: c_int = 8;
pub const V2_OPT_IFACE_MODE_NONE: c_int = 0;
pub const V2_OPT_IFACE_MODE_ADAT: c_int = 1;
pub const V2_OPT_IFACE_MODE_SPDIF: c_int = 2;
#[no_mangle]
unsafe extern "C" fn get_clock_rate(data: u32, rate: *mut c_uint) -> c_int {
    static int get_clock_rate(u32 data, unsigned int *rate)
    {
    let mut index: c_uint = (data & V2_CLOCK_RATE_MASK) >> V2_CLOCK_RATE_SHIFT;
    if (index >= ARRAY_SIZE(snd_motu_clock_rates))
    return -EIO;
// rate = snd_motu_clock_rates[index];
    return 0;
    }
    int snd_motu_protocol_v2_get_clock_rate(struct snd_motu *motu,
    unsigned int *rate)
    {
    __be32 reg;
    int err;
    err = snd_motu_transaction_read(motu, V2_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    return get_clock_rate(be32_to_cpu(reg), rate);
    }
    int snd_motu_protocol_v2_set_clock_rate(struct snd_motu *motu,
    unsigned int rate)
    {
    __be32 reg;
    u32 data;
    int i;
    int err;
    for (i = 0; i < ARRAY_SIZE(snd_motu_clock_rates); ++i) {
    if (snd_motu_clock_rates[i] == rate)
    break;
    }
    if (i == ARRAY_SIZE(snd_motu_clock_rates))
    return -EINVAL;
    err = snd_motu_transaction_read(motu, V2_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    data &= ~V2_CLOCK_RATE_MASK;
    data |= i << V2_CLOCK_RATE_SHIFT;
    reg = cpu_to_be32(data);
    return snd_motu_transaction_write(motu, V2_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    }
    static int get_clock_source(struct snd_motu *motu, u32 data,
    enum snd_motu_clock_source *src)
    {
    switch (data & V2_CLOCK_SRC_MASK) {
    case V2_CLOCK_SRC_INTERNAL:
// src = SND_MOTU_CLOCK_SOURCE_INTERNAL;
    break;
    case V2_CLOCK_SRC_ADAT_ON_OPT:
// src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT;
    break;
    case V2_CLOCK_SRC_SPDIF:
    {
    bool support_iec60958_on_opt = (motu.spec == &snd_motu_spec_828mk2 ||
    motu.spec == &snd_motu_spec_traveler);
    if (motu.spec == &snd_motu_spec_896hd) {
// src = SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
    } else if (!support_iec60958_on_opt) {
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
    } else {
    __be32 reg;
// To check the configuration of optical interface.
    int err = snd_motu_transaction_read(motu, V2_IN_OUT_CONF_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    if (((data & V2_OPT_IN_IFACE_MASK) >> V2_OPT_IN_IFACE_SHIFT) ==
    V2_OPT_IFACE_MODE_SPDIF)
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT;
    else
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
    }
    break;
    }
    case V2_CLOCK_SRC_SPH:
// src = SND_MOTU_CLOCK_SOURCE_SPH;
    break;
    case V2_CLOCK_SRC_WORD_ON_BNC:
// src = SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC;
    break;
    case V2_CLOCK_SRC_ADAT_ON_DSUB:
// src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB;
    break;
    case V2_CLOCK_SRC_AESEBU_ON_XLR:
// For Traveler.
// src = SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
    break;
    default:
// src = SND_MOTU_CLOCK_SOURCE_UNKNOWN;
    break;
    }
    return 0;
    }
    int snd_motu_protocol_v2_get_clock_source(struct snd_motu *motu,
    enum snd_motu_clock_source *src)
    {
    __be32 reg;
    int err;
    err = snd_motu_transaction_read(motu, V2_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    return get_clock_source(motu, be32_to_cpu(reg), src);
    }
// Expected for Traveler, which implements Altera Cyclone EP1C3.
    static int switch_fetching_mode_cyclone(struct snd_motu *motu, u32 *data,
    bool enable)
    {
// data |= V2_CLOCK_MODEL_SPECIFIC;
    return 0;
    }
// For UltraLite and 8pre, which implements Xilinx Spartan XC3S200.
    static int switch_fetching_mode_spartan(struct snd_motu *motu, u32 *data,
    bool enable)
    {
    unsigned int rate;
    enum snd_motu_clock_source src;
    int err;
    err = get_clock_source(motu, *data, &src);
    if (err < 0)
    return err;
    err = get_clock_rate(*data, &rate);
    if (err < 0)
    return err;
    if (src == SND_MOTU_CLOCK_SOURCE_SPH && rate > 48000)
// data |= V2_CLOCK_MODEL_SPECIFIC;
    return 0;
    }
    int snd_motu_protocol_v2_switch_fetching_mode(struct snd_motu *motu,
    bool enable)
    {
    if (motu.spec == &snd_motu_spec_828mk2) {
// 828mkII implements Altera ACEX 1K EP1K30. Nothing to do.
    return 0;
    } else if (motu.spec == &snd_motu_spec_896hd) {
// 896HD implements Altera Cyclone EP1C3 but nothing to do.
    return 0;
    } else {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_transaction_read(motu, V2_CLOCK_STATUS_OFFSET,
    &reg, sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    data &= ~(V2_CLOCK_FETCH_ENABLE | V2_CLOCK_MODEL_SPECIFIC);
    if (enable)
    data |= V2_CLOCK_FETCH_ENABLE;
    if (motu.spec == &snd_motu_spec_traveler)
    err = switch_fetching_mode_cyclone(motu, &data, enable);
    else
    err = switch_fetching_mode_spartan(motu, &data, enable);
    if (err < 0)
    return err;
    reg = cpu_to_be32(data);
    return snd_motu_transaction_write(motu, V2_CLOCK_STATUS_OFFSET,
    &reg, sizeof(reg));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v2_cache_packet_formats(motu: *mut snd_motu) -> c_int {
    int snd_motu_protocol_v2_cache_packet_formats(struct snd_motu *motu)
    {
    let mut has_two_opt_ifaces: bool = (motu.spec == &snd_motu_spec_8pre);
    __be32 reg;
    u32 data;
    int err;
    motu.tx_packet_formats.pcm_byte_offset = 10;
    motu.rx_packet_formats.pcm_byte_offset = 10;
    motu.tx_packet_formats.msg_chunks = 2;
    motu.rx_packet_formats.msg_chunks = 2;
    err = snd_motu_transaction_read(motu, V2_IN_OUT_CONF_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    memcpy(motu.tx_packet_formats.pcm_chunks,
    motu.spec.tx_fixed_pcm_chunks,
    sizeof(motu.tx_packet_formats.pcm_chunks));
    memcpy(motu.rx_packet_formats.pcm_chunks,
    motu.spec.rx_fixed_pcm_chunks,
    sizeof(motu.rx_packet_formats.pcm_chunks));
    if (((data & V2_OPT_IN_IFACE_MASK) >> V2_OPT_IN_IFACE_SHIFT) == V2_OPT_IFACE_MODE_ADAT) {
    motu.tx_packet_formats.pcm_chunks[0] += 8;
    if (!has_two_opt_ifaces)
    motu.tx_packet_formats.pcm_chunks[1] += 4;
    else
    motu.tx_packet_formats.pcm_chunks[1] += 8;
    }
    if (((data & V2_OPT_OUT_IFACE_MASK) >> V2_OPT_OUT_IFACE_SHIFT) == V2_OPT_IFACE_MODE_ADAT) {
    motu.rx_packet_formats.pcm_chunks[0] += 8;
    if (!has_two_opt_ifaces)
    motu.rx_packet_formats.pcm_chunks[1] += 4;
    else
    motu.rx_packet_formats.pcm_chunks[1] += 8;
    }
    return 0;
    }
    const struct snd_motu_spec snd_motu_spec_828mk2 = {
    .name = "828mk2",
    .protocol_version = SND_MOTU_PROTOCOL_V2,
    .flags = SND_MOTU_SPEC_RX_MIDI_2ND_Q |
    SND_MOTU_SPEC_TX_MIDI_2ND_Q |
    SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {14, 14, 0},
    .rx_fixed_pcm_chunks = {14, 14, 0},
    };
    const struct snd_motu_spec snd_motu_spec_896hd = {
    .name = "896HD",
    .protocol_version = SND_MOTU_PROTOCOL_V2,
    .flags = SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {14, 14, 8},
    .rx_fixed_pcm_chunks = {14, 14, 8},
    };
    const struct snd_motu_spec snd_motu_spec_traveler = {
    .name = "Traveler",
    .protocol_version = SND_MOTU_PROTOCOL_V2,
    .flags = SND_MOTU_SPEC_RX_MIDI_2ND_Q |
    SND_MOTU_SPEC_TX_MIDI_2ND_Q |
    SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {14, 14, 8},
    .rx_fixed_pcm_chunks = {14, 14, 8},
    };
    const struct snd_motu_spec snd_motu_spec_ultralite = {
    .name = "UltraLite",
    .protocol_version = SND_MOTU_PROTOCOL_V2,
    .flags = SND_MOTU_SPEC_RX_MIDI_2ND_Q |
    SND_MOTU_SPEC_TX_MIDI_2ND_Q |
    SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {14, 14, 0},
    .rx_fixed_pcm_chunks = {14, 14, 0},
    };
    const struct snd_motu_spec snd_motu_spec_8pre = {
    .name = "8pre",
    .protocol_version = SND_MOTU_PROTOCOL_V2,
    .flags = SND_MOTU_SPEC_RX_MIDI_2ND_Q |
    SND_MOTU_SPEC_TX_MIDI_2ND_Q |
    SND_MOTU_SPEC_REGISTER_DSP,
// Two dummy chunks always in the end of data block.
    .tx_fixed_pcm_chunks = {10, 10, 0},
    .rx_fixed_pcm_chunks = {6, 6, 0},
    };
