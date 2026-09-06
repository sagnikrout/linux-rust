//! Automatically rewritten from C to Rust
//! Source: sound/pci/echoaudio/indigoiox_dsp.c
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
    static int update_vmixer_level(struct echoaudio *chip);
    static int set_vmixer_gain(struct echoaudio *chip, u16 output,
    u16 pipe, int gain);
#[no_mangle]
unsafe extern "C" fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
    static int init_hw(struct echoaudio *chip, u16 device_id, u16 subdevice_id)
    {
    int err;
    if (snd_BUG_ON((subdevice_id & 0xfff0) != INDIGO_IOX))
    return -ENODEV;
    err = init_dsp_comm_page(chip);
    if (err < 0) {
    dev_err(chip.card.dev,
    "init_hw - could not initialize DSP comm page\n");
    return err;
    }
    chip.device_id = device_id;
    chip.subdevice_id = subdevice_id;
    chip.bad_board = true;
    chip.dsp_code_to_load = FW_INDIGO_IOX_DSP;
// Since this card has no ASIC, mark it as loaded so everything
    works OK */
    chip.asic_loaded = true;
    chip.input_clock_types = ECHO_CLOCK_BIT_INTERNAL;
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
