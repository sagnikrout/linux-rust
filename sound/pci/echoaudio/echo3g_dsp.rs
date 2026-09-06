//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/echo3g_dsp.c
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
    static int load_asic(struct echoaudio *chip);
    static int dsp_set_digital_mode(struct echoaudio *chip, u8 mode);
    static int set_digital_mode(struct echoaudio *chip, u8 mode);
    static int check_asic_status(struct echoaudio *chip);
    static int set_sample_rate(struct echoaudio *chip, u32 rate);
    static int set_input_clock(struct echoaudio *chip, u16 clock);
    static int set_professional_spdif(struct echoaudio *chip, char prof);
    static int set_phantom_power(struct echoaudio *chip, char on);
    static int write_control_reg(struct echoaudio *chip, u32 ctl, u32 frq,
    char force);

#[no_mangle]
unsafe extern "C" fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    static int init_hw(struct echoaudio *chip, u16 device_id, u16 subdevice_id)
    {
    int err;
    local_irq_enable();
    if (snd_BUG_ON((subdevice_id & 0xfff0) != ECHO3G))
    return -ENODEV;
    err = init_dsp_comm_page(chip);
    if (err) {
    dev_err(chip.card.dev,
    "init_hw - could not initialize DSP comm page\n");
    return err;
    }
    chip.comm_page.e3g_frq_register =
    cpu_to_le32((E3G_MAGIC_NUMBER / 48000) - 2);
    chip.device_id = device_id;
    chip.subdevice_id = subdevice_id;
    chip.bad_board = true;
    chip.has_midi = true;
    chip.dsp_code_to_load = FW_ECHO3G_DSP;
// Load the DSP code and the ASIC on the PCI card and get
    what type of external box is attached */
    err = load_firmware(chip);
    if (err < 0) {
    return err;
    } else if (err == E3G_GINA3G_BOX_TYPE) {
    chip.input_clock_types =	ECHO_CLOCK_BIT_INTERNAL |
    ECHO_CLOCK_BIT_SPDIF |
    ECHO_CLOCK_BIT_ADAT;
    chip.card_name = "Gina3G";
    chip.px_digital_out = chip.bx_digital_out = 6;
    chip.px_analog_in = chip.bx_analog_in = 14;
    chip.px_digital_in = chip.bx_digital_in = 16;
    chip.px_num = chip.bx_num = 24;
    chip.has_phantom_power = true;
    chip.hasnt_input_nominal_level = true;
    } else if (err == E3G_LAYLA3G_BOX_TYPE) {
    chip.input_clock_types =	ECHO_CLOCK_BIT_INTERNAL |
    ECHO_CLOCK_BIT_SPDIF |
    ECHO_CLOCK_BIT_ADAT |
    ECHO_CLOCK_BIT_WORD;
    chip.card_name = "Layla3G";
    chip.px_digital_out = chip.bx_digital_out = 8;
    chip.px_analog_in = chip.bx_analog_in = 16;
    chip.px_digital_in = chip.bx_digital_in = 24;
    chip.px_num = chip.bx_num = 32;
    } else {
    return -ENODEV;
    }
    chip.digital_modes =	ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA |
    ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL |
    ECHOCAPS_HAS_DIGITAL_MODE_ADAT;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn set_mixer_defaults(chip: *mut echoaudio) -> c_int {
    static int set_mixer_defaults(struct echoaudio *chip)
    {
    chip.digital_mode = DIGITAL_MODE_SPDIF_RCA;
    chip.professional_spdif = false;
    chip.non_audio_spdif = false;
    chip.bad_board = false;
    chip.phantom_power = false;
    return init_line_levels(chip);
    }
#[no_mangle]
unsafe extern "C" fn set_phantom_power(chip: *mut echoaudio, on: c_char) -> c_int {
    static int set_phantom_power(struct echoaudio *chip, char on)
    {
    let mut control_reg: u32 = le32_to_cpu(chip.comm_page.control_register);
    if (on)
    control_reg |= E3G_PHANTOM_POWER;
    else
    control_reg &= ~E3G_PHANTOM_POWER;
    chip.phantom_power = on;
    return write_control_reg(chip, control_reg,
    le32_to_cpu(chip.comm_page.e3g_frq_register),
    0);
    }
