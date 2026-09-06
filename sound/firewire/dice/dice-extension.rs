//! Automatically rewritten from C to Rust
//! Source: sound/firewire/dice/dice-extension.c
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
// dice-extension.c - a part of driver for DICE based devices
//
// Copyright (c) 2018 Takashi Sakamoto
//

// For TCD2210/2220, TCAT defines extension of application protocol.
pub const DICE_EXT_APP_SPACE: c_uint = 0xffffe0200000uLL;
pub const DICE_EXT_APP_CAPS_OFFSET: c_uint = 0x00;
pub const DICE_EXT_APP_CAPS_SIZE: c_uint = 0x04;
pub const DICE_EXT_APP_CMD_OFFSET: c_uint = 0x08;
pub const DICE_EXT_APP_CMD_SIZE: c_uint = 0x0c;
pub const DICE_EXT_APP_MIXER_OFFSET: c_uint = 0x10;
pub const DICE_EXT_APP_MIXER_SIZE: c_uint = 0x14;
pub const DICE_EXT_APP_PEAK_OFFSET: c_uint = 0x18;
pub const DICE_EXT_APP_PEAK_SIZE: c_uint = 0x1c;
pub const DICE_EXT_APP_ROUTER_OFFSET: c_uint = 0x20;
pub const DICE_EXT_APP_ROUTER_SIZE: c_uint = 0x24;
pub const DICE_EXT_APP_STREAM_OFFSET: c_uint = 0x28;
pub const DICE_EXT_APP_STREAM_SIZE: c_uint = 0x2c;
pub const DICE_EXT_APP_CURRENT_OFFSET: c_uint = 0x30;
pub const DICE_EXT_APP_CURRENT_SIZE: c_uint = 0x34;
pub const DICE_EXT_APP_STANDALONE_OFFSET: c_uint = 0x38;
pub const DICE_EXT_APP_STANDALONE_SIZE: c_uint = 0x3c;
pub const DICE_EXT_APP_APPLICATION_OFFSET: c_uint = 0x40;
pub const DICE_EXT_APP_APPLICATION_SIZE: c_uint = 0x44;
pub const EXT_APP_STREAM_TX_NUMBER: c_uint = 0x0000;
pub const EXT_APP_STREAM_RX_NUMBER: c_uint = 0x0004;
pub const EXT_APP_STREAM_ENTRIES: c_uint = 0x0008;
pub const EXT_APP_STREAM_ENTRY_SIZE: c_uint = 0x010c;
pub const EXT_APP_NUMBER_AUDIO: c_uint = 0x0000;
pub const EXT_APP_NUMBER_MIDI: c_uint = 0x0004;
pub const EXT_APP_NAMES: c_uint = 0x0008;
pub const EXT_APP_NAMES_SIZE: c_int = 256;
pub const EXT_APP_AC3: c_uint = 0x0108;
pub const EXT_APP_CONFIG_LOW_ROUTER: c_uint = 0x0000;
pub const EXT_APP_CONFIG_LOW_STREAM: c_uint = 0x1000;
pub const EXT_APP_CONFIG_MIDDLE_ROUTER: c_uint = 0x2000;
pub const EXT_APP_CONFIG_MIDDLE_STREAM: c_uint = 0x3000;
pub const EXT_APP_CONFIG_HIGH_ROUTER: c_uint = 0x4000;
pub const EXT_APP_CONFIG_HIGH_STREAM: c_uint = 0x5000;
    static inline int read_transaction(struct snd_dice *dice, u64 section_addr,
    u32 offset, void *buf, size_t len)
    {
    return snd_fw_transaction(dice.unit,
    len == 4 ? TCODE_READ_QUADLET_REQUEST :
    TCODE_READ_BLOCK_REQUEST,
    section_addr + offset, buf, len, 0);
    }
    static int read_stream_entries(struct snd_dice *dice, u64 section_addr,
    u32 base_offset, unsigned int stream_count,
    unsigned int mode,
    unsigned int pcm_channels[MAX_STREAMS][3],
    unsigned int midi_ports[MAX_STREAMS])
    {
    u32 entry_offset;
    __be32 reg[2];
    int err;
    int i;
    for (i = 0; i < stream_count; ++i) {
    entry_offset = base_offset + i * EXT_APP_STREAM_ENTRY_SIZE;
    err = read_transaction(dice, section_addr,
    entry_offset + EXT_APP_NUMBER_AUDIO,
    reg, sizeof(reg));
    if (err < 0)
    return err;
    pcm_channels[i][mode] = be32_to_cpu(reg[0]);
    midi_ports[i] = max(midi_ports[i], be32_to_cpu(reg[1]));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn detect_stream_formats(dice: *mut snd_dice, section_addr: u64) -> c_int {
    static int detect_stream_formats(struct snd_dice *dice, u64 section_addr)
    {
    u32 base_offset;
    __be32 reg[2];
    unsigned int stream_count;
    int mode;
    let mut err: c_int = 0;
    for (mode = 0; mode < SND_DICE_RATE_MODE_COUNT; ++mode) {
    unsigned int cap;
//
// Some models report stream formats at highest mode, however
// they don't support the mode. Check clock capabilities.
//
    if (mode == 2) {
    cap = CLOCK_CAP_RATE_176400 | CLOCK_CAP_RATE_192000;
    } else if (mode == 1) {
    cap = CLOCK_CAP_RATE_88200 | CLOCK_CAP_RATE_96000;
    } else {
    cap = CLOCK_CAP_RATE_32000 | CLOCK_CAP_RATE_44100 |
    CLOCK_CAP_RATE_48000;
    }
    if (!(cap & dice.clock_caps))
    continue;
    base_offset = 0x2000 * mode + 0x1000;
    err = read_transaction(dice, section_addr,
    base_offset + EXT_APP_STREAM_TX_NUMBER,
    &reg, sizeof(reg));
    if (err < 0)
    break;
    base_offset += EXT_APP_STREAM_ENTRIES;
    stream_count = min_t(unsigned int, be32_to_cpu(reg[0]), MAX_STREAMS);
    err = read_stream_entries(dice, section_addr, base_offset,
    stream_count, mode,
    dice.tx_pcm_chs,
    dice.tx_midi_ports);
    if (err < 0)
    break;
    base_offset += stream_count * EXT_APP_STREAM_ENTRY_SIZE;
    stream_count = min_t(unsigned int, be32_to_cpu(reg[1]), MAX_STREAMS);
    err = read_stream_entries(dice, section_addr, base_offset,
    stream_count,
    mode, dice.rx_pcm_chs,
    dice.rx_midi_ports);
    if (err < 0)
    break;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_dice_detect_extension_formats(dice: *mut snd_dice) -> c_int {
    int snd_dice_detect_extension_formats(struct snd_dice *dice)
    {
    __be32 *pointers;
    unsigned int i;
    u64 section_addr;
    int err;
    pointers = kmalloc_array(9, sizeof(__be32) * 2, GFP_KERNEL);
    if (pointers == core::ptr::null_mut())
    return -ENOMEM;
    err = snd_fw_transaction(dice.unit, TCODE_READ_BLOCK_REQUEST,
    DICE_EXT_APP_SPACE, pointers,
    9 * sizeof(__be32) * 2, 0);
    if (err < 0)
    goto end;
// Check two of them for offset have the same value or not.
    for (i = 0; i < 9; ++i) {
    int j;
    for (j = i + 1; j < 9; ++j) {
    if (pointers[i * 2] == pointers[j * 2]) {
// Fallback to limited functionality.
    err = -ENXIO;
    goto end;
    }
    }
    }
    section_addr = DICE_EXT_APP_SPACE + be32_to_cpu(pointers[12]) * 4;
    err = detect_stream_formats(dice, section_addr);
    end:
    kfree(pointers);
    return err;
    }
