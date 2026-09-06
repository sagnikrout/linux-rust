//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/gina20_dsp.c
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
    static int set_professional_spdif(struct echoaudio *chip, char prof);
    static int update_flags(struct echoaudio *chip);
#[no_mangle]
unsafe extern "C" fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    static int init_hw(struct echoaudio *chip, u16 device_id, u16 subdevice_id)
    {
    int err;
    if (snd_BUG_ON((subdevice_id & 0xfff0) != GINA20))
    return -ENODEV;
    err = init_dsp_comm_page(chip);
    if (err) {
    dev_err(chip.card.dev,
    "init_hw - could not initialize DSP comm page\n");
    return err;
    }
    chip.device_id = device_id;
    chip.subdevice_id = subdevice_id;
    chip.bad_board = true;
    chip.dsp_code_to_load = FW_GINA20_DSP;
    chip.spdif_status = GD_SPDIF_STATUS_UNDEF;
    chip.clock_state = GD_CLOCK_UNDEF;
// Since this card has no ASIC, mark it as loaded so everything
    works OK */
    chip.asic_loaded = true;
    chip.input_clock_types = ECHO_CLOCK_BIT_INTERNAL |
    ECHO_CLOCK_BIT_SPDIF;
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
    chip.professional_spdif = false;
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
    if (clocks_from_dsp & GLDM_CLOCK_DETECT_BIT_SPDIF)
    clock_bits |= ECHO_CLOCK_BIT_SPDIF;
    return clock_bits;
    }
// The Gina20 has no ASIC. Just do nothing
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
    u8 clock_state, spdif_status;
    if (wait_handshake(chip))
    return -EIO;
    switch (rate) {
    case 44100:
    clock_state = GD_CLOCK_44;
    spdif_status = GD_SPDIF_STATUS_44;
    break;
    case 48000:
    clock_state = GD_CLOCK_48;
    spdif_status = GD_SPDIF_STATUS_48;
    break;
    default:
    clock_state = GD_CLOCK_NOCHANGE;
    spdif_status = GD_SPDIF_STATUS_NOCHANGE;
    break;
    }
    if (chip.clock_state == clock_state)
    clock_state = GD_CLOCK_NOCHANGE;
    if (spdif_status == chip.spdif_status)
    spdif_status = GD_SPDIF_STATUS_NOCHANGE;
    chip.comm_page.sample_rate = cpu_to_le32(rate);
    chip.comm_page.gd_clock_state = clock_state;
    chip.comm_page.gd_spdif_status = spdif_status;
    chip.comm_page.gd_resampler_state = 3;	/* magic number - should always be 3 */
// Save the new audio state if it changed
    if (clock_state != GD_CLOCK_NOCHANGE)
    chip.clock_state = clock_state;
    if (spdif_status != GD_SPDIF_STATUS_NOCHANGE)
    chip.spdif_status = spdif_status;
    chip.sample_rate = rate;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE);
    }
#[no_mangle]
unsafe extern "C" fn set_input_clock(chip: *mut echoaudio, clock: u16) -> c_int {
    static int set_input_clock(struct echoaudio *chip, u16 clock)
    {
    switch (clock) {
    case ECHO_CLOCK_INTERNAL:
// Reset the audio state to unknown (just in case)
    chip.clock_state = GD_CLOCK_UNDEF;
    chip.spdif_status = GD_SPDIF_STATUS_UNDEF;
    set_sample_rate(chip, chip.sample_rate);
    chip.input_clock = clock;
    break;
    case ECHO_CLOCK_SPDIF:
    chip.comm_page.gd_clock_state = GD_CLOCK_SPDIFIN;
    chip.comm_page.gd_spdif_status = GD_SPDIF_STATUS_NOCHANGE;
    clear_handshake(chip);
    send_vector(chip, DSP_VC_SET_GD_AUDIO_STATE);
    chip.clock_state = GD_CLOCK_SPDIFIN;
    chip.input_clock = clock;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
// Set input bus gain (one unit is 0.5dB !)
#[no_mangle]
unsafe extern "C" fn set_input_gain(chip: *mut echoaudio, input: u16, gain: c_int) -> c_int {
    static int set_input_gain(struct echoaudio *chip, u16 input, int gain)
    {
    if (snd_BUG_ON(input >= num_busses_in(chip)))
    return -EINVAL;
    if (wait_handshake(chip))
    return -EIO;
    chip.input_gain[input] = gain;
    gain += GL20_INPUT_GAIN_MAGIC_NUMBER;
    chip.comm_page.line_in_level[input] = gain;
    return 0;
    }
// Tell the DSP to reread the flags from the comm page
#[no_mangle]
unsafe extern "C" fn update_flags(chip: *mut echoaudio) -> c_int {
    static int update_flags(struct echoaudio *chip)
    {
    if (wait_handshake(chip))
    return -EIO;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_UPDATE_FLAGS);
    }
#[no_mangle]
unsafe extern "C" fn set_professional_spdif(chip: *mut echoaudio, prof: c_char) -> c_int {
    static int set_professional_spdif(struct echoaudio *chip, char prof)
    {
    if (prof)
    chip.comm_page.flags |=
    cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    else
    chip.comm_page.flags &=
    ~cpu_to_le32(DSP_FLAG_PROFESSIONAL_SPDIF);
    chip.professional_spdif = prof;
    return update_flags(chip);
    }
