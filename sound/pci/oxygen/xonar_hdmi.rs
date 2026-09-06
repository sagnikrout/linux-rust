//! Automatically rewritten from C to Rust
//! Source: sound/pci/oxygen/xonar_hdmi.c
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
// helper functions for HDMI models (Xonar HDAV1.3/HDAV1.3 Slim)
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
//

    static void hdmi_write_command(struct oxygen *chip, u8 command,
    unsigned int count, const u8 *params)
    {
    unsigned int i;
    u8 checksum;
    oxygen_write_uart(chip, 0xfb);
    oxygen_write_uart(chip, 0xef);
    oxygen_write_uart(chip, command);
    oxygen_write_uart(chip, count);
    for (i = 0; i < count; ++i)
    oxygen_write_uart(chip, params[i]);
    checksum = 0xfb + 0xef + command + count;
    for (i = 0; i < count; ++i)
    checksum += params[i];
    oxygen_write_uart(chip, checksum);
    }
    static void xonar_hdmi_init_commands(struct oxygen *chip,
    struct xonar_hdmi *hdmi)
    {
    u8 param;
    oxygen_reset_uart(chip);
    param = 0;
    hdmi_write_command(chip, 0x61, 1, &param);
    param = 1;
    hdmi_write_command(chip, 0x74, 1, &param);
    hdmi_write_command(chip, 0x54, 5, hdmi.params);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_hdmi_init(chip: *mut oxygen, hdmi: *mut xonar_hdmi) {
    void xonar_hdmi_init(struct oxygen *chip, struct xonar_hdmi *hdmi)
    {
    hdmi.params[1] = IEC958_AES3_CON_FS_48000;
    hdmi.params[4] = 1;
    xonar_hdmi_init_commands(chip, hdmi);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_hdmi_cleanup(chip: *mut oxygen) {
    void xonar_hdmi_cleanup(struct oxygen *chip)
    {
    let mut param: u8 = 0;
    hdmi_write_command(chip, 0x74, 1, &param);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_hdmi_resume(chip: *mut oxygen, hdmi: *mut xonar_hdmi) {
    void xonar_hdmi_resume(struct oxygen *chip, struct xonar_hdmi *hdmi)
    {
    xonar_hdmi_init_commands(chip, hdmi);
    }
    void xonar_hdmi_pcm_hardware_filter(unsigned int channel,
    struct snd_pcm_hardware *hardware)
    {
    if (channel == PCM_MULTICH) {
    hardware.rates = SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_192000;
    hardware.rate_min = 44100;
    }
    }
    void xonar_set_hdmi_params(struct oxygen *chip, struct xonar_hdmi *hdmi,
    struct snd_pcm_hw_params *params)
    {
    hdmi.params[0] = 0; /* 1 = non-audio */
    switch (params_rate(params)) {
    case 44100:
    hdmi.params[1] = IEC958_AES3_CON_FS_44100;
    break;
    case 48000:
    hdmi.params[1] = IEC958_AES3_CON_FS_48000;
    break;
    default: /* 96000 */
    hdmi.params[1] = IEC958_AES3_CON_FS_96000;
    break;
    case 192000:
    hdmi.params[1] = IEC958_AES3_CON_FS_192000;
    break;
    }
    hdmi.params[2] = params_channels(params) / 2 - 1;
    if (params_format(params) == SNDRV_PCM_FORMAT_S16_LE)
    hdmi.params[3] = 0;
    else
    hdmi.params[3] = 0xc0;
    hdmi.params[4] = 1; /* ? */
    hdmi_write_command(chip, 0x54, 5, hdmi.params);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_hdmi_uart_input(chip: *mut oxygen) {
    void xonar_hdmi_uart_input(struct oxygen *chip)
    {
    if (chip.uart_input_count >= 2 &&
    chip.uart_input[chip.uart_input_count - 2] == 'O' &&
    chip.uart_input[chip.uart_input_count - 1] == 'K') {
    dev_dbg(chip.card.dev, "message from HDMI chip received:\n");
    print_hex_dump_bytes("", DUMP_PREFIX_OFFSET,
    chip.uart_input, chip.uart_input_count);
    chip.uart_input_count = 0;
    }
    }
