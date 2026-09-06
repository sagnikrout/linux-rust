//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/darla24_dsp.c
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
    Copyright Echo Digital Audio Corporation (c) 1998 - 2004
    All rights reserved
    www.echoaudio.com
    This file is part of Echo Digital Audio's generic driver library.
//
    Translation from C++ and adaptation for use in ALSA-Driver
    were made by Giuliano Pochini <pochini@shiny.it>
//
#[no_mangle]
unsafe extern "C" fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    static int init_hw(struct echoaudio *chip, u16 device_id, u16 subdevice_id)
    {
    int err;
    if (snd_BUG_ON((subdevice_id & 0xfff0) != DARLA24))
    return -ENODEV;
    err = init_dsp_comm_page(chip);
    if (err) {
    dev_err(chip.card.dev,
    "init_hw: could not initialize DSP comm page\n");
    return err;
    }
    chip.device_id = device_id;
    chip.subdevice_id = subdevice_id;
    chip.bad_board = true;
    chip.dsp_code_to_load = FW_DARLA24_DSP;
// Since this card has no ASIC, mark it as loaded so everything
    works OK */
    chip.asic_loaded = true;
    chip.input_clock_types = ECHO_CLOCK_BIT_INTERNAL |
    ECHO_CLOCK_BIT_ESYNC;
    err = load_firmware(chip);
    if (err < 0)
    return err;
    chip.bad_board = false;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn set_mixer_defaults(chip: *mut echoaudio) -> c_int {
    static int set_mixer_defaults(struct echoaudio *chip)
    {
    return init_line_levels(chip);
    }
#[no_mangle]
unsafe extern "C" fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    static u32 detect_input_clocks(const struct echoaudio *chip)
    {
    u32 clocks_from_dsp, clock_bits;
// Map the DSP clock detect bits to the generic driver clock
    detect bits */
    clocks_from_dsp = le32_to_cpu(chip.comm_page.status_clocks);
    clock_bits = ECHO_CLOCK_BIT_INTERNAL;
    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_ESYNC)
    clock_bits |= ECHO_CLOCK_BIT_ESYNC;
    return clock_bits;
    }
// The Darla24 has no ASIC. Just do nothing
#[no_mangle]
unsafe extern "C" fn load_asic(chip: *mut echoaudio) -> c_int {
    static int load_asic(struct echoaudio *chip)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> c_int {
    static int set_sample_rate(struct echoaudio *chip, u32 rate)
    {
    u8 clock;
    switch (rate) {
    case 96000:
    clock = GD24_96000;
    break;
    case 88200:
    clock = GD24_88200;
    break;
    case 48000:
    clock = GD24_48000;
    break;
    case 44100:
    clock = GD24_44100;
    break;
    case 32000:
    clock = GD24_32000;
    break;
    case 22050:
    clock = GD24_22050;
    break;
    case 16000:
    clock = GD24_16000;
    break;
    case 11025:
    clock = GD24_11025;
    break;
    case 8000:
    clock = GD24_8000;
    break;
    default:
    dev_err(chip.card.dev,
    "set_sample_rate: Error, invalid sample rate %d\n",
    rate);
    return -EINVAL;
    }
    if (wait_handshake(chip))
    return -EIO;
    dev_dbg(chip.card.dev,
    "set_sample_rate: %d clock %d\n", rate, clock);
    chip.sample_rate = rate;
// Override the sample rate if this card is set to Echo sync.
    if (chip.input_clock == ECHO_CLOCK_ESYNC)
    clock = GD24_EXT_SYNC;
    chip.comm_page.sample_rate = cpu_to_le32(rate);	/* ignored by the DSP ? */
    chip.comm_page.gd_clock_state = clock;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE);
    }
#[no_mangle]
unsafe extern "C" fn set_input_clock(chip: *mut echoaudio, clock: u16) -> c_int {
    static int set_input_clock(struct echoaudio *chip, u16 clock)
    {
    if (snd_BUG_ON(clock != ECHO_CLOCK_INTERNAL &&
    clock != ECHO_CLOCK_ESYNC))
    return -EINVAL;
    chip.input_clock = clock;
    return set_sample_rate(chip, chip.sample_rate);
    }
