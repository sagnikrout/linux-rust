//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-command-dsp-message-parser.c
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
// motu-command-dsp-message-parser.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2021 Takashi Sakamoto <o-takashi@sakamocchi.jp>
// Below models allow software to configure their DSP function by command transferred in
// asynchronous transaction:
// * 828 mk3 (FireWire only and Hybrid)
// * 896 mk3 (FireWire only and Hybrid)
// * Ultralite mk3 (FireWire only and Hybrid)
// * Traveler mk3
// * Track 16
//
// Isochronous packets from the above models includes messages to report state of hardware meter.

    enum msg_parser_state {
    INITIALIZED,
    FRAGMENT_DETECTED,
    AVAILABLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_parser {
    pub lock: spinlock_t,
    pub state: enum msg_parser_state,
    pub interval: c_uint,
    pub message_count: c_uint,
    pub fragment_pos: c_uint,
    pub value_index: c_uint,
    pub value: u64,
    pub meter: snd_firewire_motu_command_dsp_meter,
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_command_dsp_message_parser_new(motu: *mut snd_motu) -> c_int {
    int snd_motu_command_dsp_message_parser_new(struct snd_motu *motu)
    {
    struct msg_parser *parser;
    parser = devm_kzalloc(&motu.card.card_dev, sizeof(*parser), GFP_KERNEL);
    if (!parser)
    return -ENOMEM;
    spin_lock_init(&parser.lock);
    motu.message_parser = parser;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_command_dsp_message_parser_init(motu: *mut snd_motu, sfc: enum cip_sfc) -> c_int {
    int snd_motu_command_dsp_message_parser_init(struct snd_motu *motu, enum cip_sfc sfc)
    {
    struct msg_parser *parser = motu.message_parser;
    parser.state = INITIALIZED;
// All of data blocks don't have messages with meaningful information.
    switch (sfc) {
    case CIP_SFC_176400:
    case CIP_SFC_192000:
    parser.interval = 4;
    break;
    case CIP_SFC_88200:
    case CIP_SFC_96000:
    parser.interval = 2;
    break;
    case CIP_SFC_32000:
    case CIP_SFC_44100:
    case CIP_SFC_48000:
    default:
    parser.interval = 1;
    break;
    }
    return 0;
    }
pub const FRAGMENT_POS: c_int = 6;
pub const MIDI_BYTE_POS: c_int = 7;
pub const MIDI_FLAG_POS: c_int = 8;
// One value of hardware meter consists of 4 messages.
pub const FRAGMENTS_PER_VALUE: c_int = 4;
pub const VALUES_AT_IMAGE_END: c_uint = 0xffffffffffffffff;
    void snd_motu_command_dsp_message_parser_parse(const struct amdtp_stream *s,
    const struct pkt_desc *desc, unsigned int count)
    {
    struct snd_motu *motu = container_of(s, struct snd_motu, tx_stream);
    let mut data_block_quadlets: c_uint = s.data_block_quadlets;
    struct msg_parser *parser = motu.message_parser;
    let mut interval: c_uint = parser.interval;
    int i;
    guard(spinlock_irqsave)(&parser.lock);
    for (i = 0; i < count; ++i) {
    __be32 *buffer = desc.ctx_payload;
    let mut data_blocks: c_uint = desc.data_blocks;
    int j;
    desc = amdtp_stream_next_packet_desc(s, desc);
    for (j = 0; j < data_blocks; ++j) {
    u8 *b = (u8 *)buffer;
    buffer += data_block_quadlets;
    switch (parser.state) {
    case INITIALIZED:
    {
    let mut fragment: u8 = b[FRAGMENT_POS];
    if (fragment > 0) {
    parser.value = fragment;
    parser.message_count = 1;
    parser.state = FRAGMENT_DETECTED;
    }
    break;
    }
    case FRAGMENT_DETECTED:
    {
    if (parser.message_count % interval == 0) {
    let mut fragment: u8 = b[FRAGMENT_POS];
    parser.value >>= 8;
    parser.value |= (u64)fragment << 56;
    if (parser.value == VALUES_AT_IMAGE_END) {
    parser.state = AVAILABLE;
    parser.fragment_pos = 0;
    parser.value_index = 0;
    parser.message_count = 0;
    }
    }
    ++parser.message_count;
    break;
    }
    case AVAILABLE:
    default:
    {
    if (parser.message_count % interval == 0) {
    let mut fragment: u8 = b[FRAGMENT_POS];
    parser.value >>= 8;
    parser.value |= (u64)fragment << 56;
    ++parser.fragment_pos;
    if (parser.fragment_pos == 4) {
// Skip the last two quadlets since they could be
// invalid value (0xffffffff) as floating point
// number.
    if (parser.value_index <
    SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT - 2) {
    let mut val: u32 = (u32)(parser.value >> 32);
    parser.meter.data[parser.value_index] = val;
    }
    ++parser.value_index;
    parser.fragment_pos = 0;
    }
    if (parser.value == VALUES_AT_IMAGE_END) {
    parser.value_index = 0;
    parser.fragment_pos = 0;
    parser.message_count = 0;
    }
    }
    ++parser.message_count;
    break;
    }
    }
    }
    }
    }
    void snd_motu_command_dsp_message_parser_copy_meter(struct snd_motu *motu,
    struct snd_firewire_motu_command_dsp_meter *meter)
    {
    struct msg_parser *parser = motu.message_parser;
    guard(spinlock_irqsave)(&parser.lock);
    memcpy(meter, &parser.meter, sizeof(*meter));
    }
