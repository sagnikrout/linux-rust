//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/indigo_express_dsp.c
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


// SPDX-License-Identifier: LGPL-2.1-or-later
//
    This file is part of Echo Digital Audio's generic driver library.
    Copyright Echo Digital Audio Corporation (c) 1998 - 2005
    All rights reserved
    www.echoaudio.com
    Translation from C++ and adaptation for use in ALSA-Driver
    were made by Giuliano Pochini <pochini@shiny.it>
//
#[no_mangle]
unsafe extern "C" fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> c_int {
    static int set_sample_rate(struct echoaudio *chip, u32 rate)
    {
    u32 clock, control_reg, old_control_reg;
    if (wait_handshake(chip))
    return -EIO;
    old_control_reg = le32_to_cpu(chip.comm_page.control_register);
    control_reg = old_control_reg & ~INDIGO_EXPRESS_CLOCK_MASK;
    switch (rate) {
    case 32000:
    clock = INDIGO_EXPRESS_32000;
    break;
    case 44100:
    clock = INDIGO_EXPRESS_44100;
    break;
    case 48000:
    clock = INDIGO_EXPRESS_48000;
    break;
    case 64000:
    clock = INDIGO_EXPRESS_32000|INDIGO_EXPRESS_DOUBLE_SPEED;
    break;
    case 88200:
    clock = INDIGO_EXPRESS_44100|INDIGO_EXPRESS_DOUBLE_SPEED;
    break;
    case 96000:
    clock = INDIGO_EXPRESS_48000|INDIGO_EXPRESS_DOUBLE_SPEED;
    break;
    default:
    return -EINVAL;
    }
    control_reg |= clock;
    if (control_reg != old_control_reg) {
    dev_dbg(chip.card.dev,
    "set_sample_rate: %d clock %d\n", rate, clock);
    chip.comm_page.control_register = cpu_to_le32(control_reg);
    chip.sample_rate = rate;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_UPDATE_CLOCKS);
    }
    return 0;
    }
// This function routes the sound from a virtual channel to a real output
    static int set_vmixer_gain(struct echoaudio *chip, u16 output, u16 pipe,
    int gain)
    {
    int index;
    if (snd_BUG_ON(pipe >= num_pipes_out(chip) ||
    output >= num_busses_out(chip)))
    return -EINVAL;
    if (wait_handshake(chip))
    return -EIO;
    chip.vmixer_gain[output][pipe] = gain;
    index = output * num_pipes_out(chip) + pipe;
    chip.comm_page.vmixer[index] = gain;
    dev_dbg(chip.card.dev,
    "set_vmixer_gain: pipe %d, out %d = %d\n", pipe, output, gain);
    return 0;
    }
// Tell the DSP to read and update virtual mixer levels in comm page.
#[no_mangle]
unsafe extern "C" fn update_vmixer_level(chip: *mut echoaudio) -> c_int {
    static int update_vmixer_level(struct echoaudio *chip)
    {
    if (wait_handshake(chip))
    return -EIO;
    clear_handshake(chip);
    return send_vector(chip, DSP_VC_SET_VMIXER_GAIN);
    }
#[no_mangle]
unsafe extern "C" fn detect_input_clocks(chip: *const echoaudio) -> u32 {
    static u32 detect_input_clocks(const struct echoaudio *chip)
    {
    return ECHO_CLOCK_BIT_INTERNAL;
    }
// The IndigoIO has no ASIC. Just do nothing
#[no_mangle]
unsafe extern "C" fn load_asic(chip: *mut echoaudio) -> c_int {
    static int load_asic(struct echoaudio *chip)
    {
    return 0;
    }
