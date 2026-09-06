//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-protocol-v3.c
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
// motu-protocol-v3.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

pub const V3_CLOCK_STATUS_OFFSET: c_uint = 0x0b14;
pub const V3_FETCH_PCM_FRAMES: c_uint = 0x02000000;
pub const V3_CLOCK_RATE_MASK: c_uint = 0x0000ff00;
pub const V3_CLOCK_RATE_SHIFT: c_int = 8;
pub const V3_CLOCK_SOURCE_MASK: c_uint = 0x000000ff;
pub const V3_CLOCK_SRC_INTERNAL: c_uint = 0x00;
pub const V3_CLOCK_SRC_WORD_ON_BNC: c_uint = 0x01;
pub const V3_CLOCK_SRC_SPH: c_uint = 0x02;
pub const V3_CLOCK_SRC_AESEBU_ON_XLR: c_uint = 0x08;
pub const V3_CLOCK_SRC_SPDIF_ON_COAX: c_uint = 0x10;
pub const V3_CLOCK_SRC_OPT_IFACE_A: c_uint = 0x18;
pub const V3_CLOCK_SRC_OPT_IFACE_B: c_uint = 0x19;
pub const V3_OPT_IFACE_MODE_OFFSET: c_uint = 0x0c94;
pub const V3_ENABLE_OPT_IN_IFACE_A: c_uint = 0x00000001;
pub const V3_ENABLE_OPT_IN_IFACE_B: c_uint = 0x00000002;
pub const V3_ENABLE_OPT_OUT_IFACE_A: c_uint = 0x00000100;
pub const V3_ENABLE_OPT_OUT_IFACE_B: c_uint = 0x00000200;
pub const V3_NO_ADAT_OPT_IN_IFACE_A: c_uint = 0x00010000;
pub const V3_NO_ADAT_OPT_IN_IFACE_B: c_uint = 0x00100000;
pub const V3_NO_ADAT_OPT_OUT_IFACE_A: c_uint = 0x00040000;
pub const V3_NO_ADAT_OPT_OUT_IFACE_B: c_uint = 0x00400000;
pub const V3_MSG_FLAG_CLK_CHANGED: c_uint = 0x00000002;
pub const V3_CLK_WAIT_MSEC: c_int = 4000;
    int snd_motu_protocol_v3_get_clock_rate(struct snd_motu *motu,
    unsigned int *rate)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_transaction_read(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    data = (data & V3_CLOCK_RATE_MASK) >> V3_CLOCK_RATE_SHIFT;
    if (data >= ARRAY_SIZE(snd_motu_clock_rates))
    return -EIO;
// rate = snd_motu_clock_rates[data];
    return 0;
    }
    int snd_motu_protocol_v3_set_clock_rate(struct snd_motu *motu,
    unsigned int rate)
    {
    __be32 reg;
    u32 data;
    bool need_to_wait;
    int i, err;
    for (i = 0; i < ARRAY_SIZE(snd_motu_clock_rates); ++i) {
    if (snd_motu_clock_rates[i] == rate)
    break;
    }
    if (i == ARRAY_SIZE(snd_motu_clock_rates))
    return -EINVAL;
    err = snd_motu_transaction_read(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg);
    data &= ~(V3_CLOCK_RATE_MASK | V3_FETCH_PCM_FRAMES);
    data |= i << V3_CLOCK_RATE_SHIFT;
    need_to_wait = data != be32_to_cpu(reg);
    reg = cpu_to_be32(data);
    err = snd_motu_transaction_write(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    if (need_to_wait) {
    int result;
    motu.msg = 0;
    result = wait_event_interruptible_timeout(motu.hwdep_wait,
    motu.msg & V3_MSG_FLAG_CLK_CHANGED,
    msecs_to_jiffies(V3_CLK_WAIT_MSEC));
    if (result < 0)
    return result;
    if (result == 0)
    return -ETIMEDOUT;
    }
    return 0;
    }
    int snd_motu_protocol_v3_get_clock_source(struct snd_motu *motu,
    enum snd_motu_clock_source *src)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_transaction_read(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return err;
    data = be32_to_cpu(reg) & V3_CLOCK_SOURCE_MASK;
    switch (data) {
    case V3_CLOCK_SRC_INTERNAL:
// src = SND_MOTU_CLOCK_SOURCE_INTERNAL;
    break;
    case V3_CLOCK_SRC_WORD_ON_BNC:
// src = SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC;
    break;
    case V3_CLOCK_SRC_SPH:
// src = SND_MOTU_CLOCK_SOURCE_SPH;
    break;
    case V3_CLOCK_SRC_AESEBU_ON_XLR:
// src = SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
    break;
    case V3_CLOCK_SRC_SPDIF_ON_COAX:
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
    break;
    case V3_CLOCK_SRC_OPT_IFACE_A:
    case V3_CLOCK_SRC_OPT_IFACE_B:
    {
    __be32 reg;
    u32 options;
    err = snd_motu_transaction_read(motu,
    V3_OPT_IFACE_MODE_OFFSET, &reg, sizeof(reg));
    if (err < 0)
    return err;
    options = be32_to_cpu(reg);
    if (data == V3_CLOCK_SRC_OPT_IFACE_A) {
    if (options & V3_NO_ADAT_OPT_IN_IFACE_A)
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A;
    else
// src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A;
    } else {
    if (options & V3_NO_ADAT_OPT_IN_IFACE_B)
// src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B;
    else
// src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B;
    }
    break;
    }
    default:
// src = SND_MOTU_CLOCK_SOURCE_UNKNOWN;
    break;
    }
    return 0;
    }
    int snd_motu_protocol_v3_switch_fetching_mode(struct snd_motu *motu,
    bool enable)
    {
    __be32 reg;
    u32 data;
    int err;
    err = snd_motu_transaction_read(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    if (err < 0)
    return 0;
    data = be32_to_cpu(reg);
    if (enable)
    data |= V3_FETCH_PCM_FRAMES;
    else
    data &= ~V3_FETCH_PCM_FRAMES;
    reg = cpu_to_be32(data);
    return snd_motu_transaction_write(motu, V3_CLOCK_STATUS_OFFSET, &reg,
    sizeof(reg));
    }
#[no_mangle]
unsafe extern "C" fn detect_packet_formats_with_opt_ifaces(motu: *mut snd_motu, data: u32) -> c_int {
    static int detect_packet_formats_with_opt_ifaces(struct snd_motu *motu, u32 data)
    {
    if (data & V3_ENABLE_OPT_IN_IFACE_A) {
    if (data & V3_NO_ADAT_OPT_IN_IFACE_A) {
    motu.tx_packet_formats.pcm_chunks[0] += 4;
    motu.tx_packet_formats.pcm_chunks[1] += 4;
    } else {
    motu.tx_packet_formats.pcm_chunks[0] += 8;
    motu.tx_packet_formats.pcm_chunks[1] += 4;
    }
    }
    if (data & V3_ENABLE_OPT_IN_IFACE_B) {
    if (data & V3_NO_ADAT_OPT_IN_IFACE_B) {
    motu.tx_packet_formats.pcm_chunks[0] += 4;
    motu.tx_packet_formats.pcm_chunks[1] += 4;
    } else {
    motu.tx_packet_formats.pcm_chunks[0] += 8;
    motu.tx_packet_formats.pcm_chunks[1] += 4;
    }
    }
    if (data & V3_ENABLE_OPT_OUT_IFACE_A) {
    if (data & V3_NO_ADAT_OPT_OUT_IFACE_A) {
    motu.rx_packet_formats.pcm_chunks[0] += 4;
    motu.rx_packet_formats.pcm_chunks[1] += 4;
    } else {
    motu.rx_packet_formats.pcm_chunks[0] += 8;
    motu.rx_packet_formats.pcm_chunks[1] += 4;
    }
    }
    if (data & V3_ENABLE_OPT_OUT_IFACE_B) {
    if (data & V3_NO_ADAT_OPT_OUT_IFACE_B) {
    motu.rx_packet_formats.pcm_chunks[0] += 4;
    motu.rx_packet_formats.pcm_chunks[1] += 4;
    } else {
    motu.rx_packet_formats.pcm_chunks[0] += 8;
    motu.rx_packet_formats.pcm_chunks[1] += 4;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_cache_packet_formats(motu: *mut snd_motu) -> c_int {
    int snd_motu_protocol_v3_cache_packet_formats(struct snd_motu *motu)
    {
    __be32 reg;
    u32 data;
    int err;
    motu.tx_packet_formats.pcm_byte_offset = 10;
    motu.rx_packet_formats.pcm_byte_offset = 10;
    motu.tx_packet_formats.msg_chunks = 2;
    motu.rx_packet_formats.msg_chunks = 2;
    err = snd_motu_transaction_read(motu, V3_OPT_IFACE_MODE_OFFSET, &reg,
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
    if (motu.spec == &snd_motu_spec_828mk3_fw ||
    motu.spec == &snd_motu_spec_828mk3_hybrid ||
    motu.spec == &snd_motu_spec_896mk3 ||
    motu.spec == &snd_motu_spec_traveler_mk3 ||
    motu.spec == &snd_motu_spec_track16)
    return detect_packet_formats_with_opt_ifaces(motu, data);
    else
    return 0;
    }
    const struct snd_motu_spec snd_motu_spec_828mk3_fw = {
    .name = "828mk3",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_3RD_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {18, 18, 14},
    .rx_fixed_pcm_chunks = {14, 14, 10},
    };
    const struct snd_motu_spec snd_motu_spec_828mk3_hybrid = {
    .name = "828mk3",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_3RD_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {18, 18, 14},
    .rx_fixed_pcm_chunks = {14, 14, 14},	// Additional 4 dummy chunks at higher rate.
    };
    const struct snd_motu_spec snd_motu_spec_896mk3 = {
    .name = "896mk3",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {18, 14, 10},
    .rx_fixed_pcm_chunks = {18, 14, 10},
    };
    const struct snd_motu_spec snd_motu_spec_traveler_mk3 = {
    .name = "TravelerMk3",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_3RD_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {18, 14, 10},
    .rx_fixed_pcm_chunks = {14, 14, 10},
    };
    const struct snd_motu_spec snd_motu_spec_ultralite_mk3 = {
    .name = "UltraLiteMk3",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_3RD_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {18, 14, 10},
    .rx_fixed_pcm_chunks = {14, 14, 14},
    };
    const struct snd_motu_spec snd_motu_spec_audio_express = {
    .name = "AudioExpress",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_2ND_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {10, 10, 0},
    .rx_fixed_pcm_chunks = {10, 10, 0},
    };
    const struct snd_motu_spec snd_motu_spec_track16 = {
    .name = "Track16",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_RX_MIDI_3RD_Q |
    SND_MOTU_SPEC_TX_MIDI_3RD_Q |
    SND_MOTU_SPEC_COMMAND_DSP,
    .tx_fixed_pcm_chunks = {14, 14, 14},
    .rx_fixed_pcm_chunks = {6, 6, 6},
    };
    const struct snd_motu_spec snd_motu_spec_4pre = {
    .name = "4pre",
    .protocol_version = SND_MOTU_PROTOCOL_V3,
    .flags = SND_MOTU_SPEC_REGISTER_DSP,
    .tx_fixed_pcm_chunks = {10, 10, 0},
    .rx_fixed_pcm_chunks = {10, 10, 0},
    };
