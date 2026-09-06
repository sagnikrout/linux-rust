//! Automatically rewritten from C to Rust
//! Source: sound/core/pcm_misc.c
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


// SPDX-License-Identifier: LGPL-2.0+
//
// PCM Interface - misc routines
// Copyright (c) 1998 by Jaroslav Kysela <perex@perex.cz>
//

// NOTE: "signed" prefix must be given below since the default char is
// unsigned on some architectures!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_format_data {
    pub /: *mut *mut unsigned char width; / bit width,
    pub /: *mut *mut unsigned char phys; / physical bit width,
    pub /: *mut *mut signed char le; / 0 = big-endian, 1 = little-endian, -1 = others,
    pub /: *mut *mut signed char signd; / 0 = unsigned, 1 = signed, -1 = others,
    pub /: *mut *mut unsigned char silence[8]; / silence data to fill,
}

#[no_mangle]
unsafe extern "C" fn valid_format(format: snd_pcm_format_t) -> bool {
    static bool valid_format(snd_pcm_format_t format)
    {
    return format >= 0 && format <= SNDRV_PCM_FORMAT_LAST;
    }
    static const struct pcm_format_data pcm_formats[SNDRV_PCM_FORMAT_LAST+1] = {
    [SNDRV_PCM_FORMAT_S8] = {
    .width = 8, .phys = 8, .le = -1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U8] = {
    .width = 8, .phys = 8, .le = -1, .signd = 0,
    .silence = { 0x80 },
    },
    [SNDRV_PCM_FORMAT_S16_LE] = {
    .width = 16, .phys = 16, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S16_BE] = {
    .width = 16, .phys = 16, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U16_LE] = {
    .width = 16, .phys = 16, .le = 1, .signd = 0,
    .silence = { 0x00, 0x80 },
    },
    [SNDRV_PCM_FORMAT_U16_BE] = {
    .width = 16, .phys = 16, .le = 0, .signd = 0,
    .silence = { 0x80, 0x00 },
    },
    [SNDRV_PCM_FORMAT_S24_LE] = {
    .width = 24, .phys = 32, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S24_BE] = {
    .width = 24, .phys = 32, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U24_LE] = {
    .width = 24, .phys = 32, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x80 },
    },
    [SNDRV_PCM_FORMAT_U24_BE] = {
    .width = 24, .phys = 32, .le = 0, .signd = 0,
    .silence = { 0x00, 0x80, 0x00, 0x00 },
    },
    [SNDRV_PCM_FORMAT_S32_LE] = {
    .width = 32, .phys = 32, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S32_BE] = {
    .width = 32, .phys = 32, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U32_LE] = {
    .width = 32, .phys = 32, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x00, 0x80 },
    },
    [SNDRV_PCM_FORMAT_U32_BE] = {
    .width = 32, .phys = 32, .le = 0, .signd = 0,
    .silence = { 0x80, 0x00, 0x00, 0x00 },
    },
    [SNDRV_PCM_FORMAT_FLOAT_LE] = {
    .width = 32, .phys = 32, .le = 1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_FLOAT_BE] = {
    .width = 32, .phys = 32, .le = 0, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_FLOAT64_LE] = {
    .width = 64, .phys = 64, .le = 1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_FLOAT64_BE] = {
    .width = 64, .phys = 64, .le = 0, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE] = {
    .width = 32, .phys = 32, .le = 1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE] = {
    .width = 32, .phys = 32, .le = 0, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_MU_LAW] = {
    .width = 8, .phys = 8, .le = -1, .signd = -1,
    .silence = { 0x7f },
    },
    [SNDRV_PCM_FORMAT_A_LAW] = {
    .width = 8, .phys = 8, .le = -1, .signd = -1,
    .silence = { 0x55 },
    },
    [SNDRV_PCM_FORMAT_IMA_ADPCM] = {
    .width = 4, .phys = 4, .le = -1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_G723_24] = {
    .width = 3, .phys = 3, .le = -1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_G723_40] = {
    .width = 5, .phys = 5, .le = -1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_DSD_U8] = {
    .width = 8, .phys = 8, .le = 1, .signd = 0,
    .silence = { 0x69 },
    },
    [SNDRV_PCM_FORMAT_DSD_U16_LE] = {
    .width = 16, .phys = 16, .le = 1, .signd = 0,
    .silence = { 0x69, 0x69 },
    },
    [SNDRV_PCM_FORMAT_DSD_U32_LE] = {
    .width = 32, .phys = 32, .le = 1, .signd = 0,
    .silence = { 0x69, 0x69, 0x69, 0x69 },
    },
    [SNDRV_PCM_FORMAT_DSD_U16_BE] = {
    .width = 16, .phys = 16, .le = 0, .signd = 0,
    .silence = { 0x69, 0x69 },
    },
    [SNDRV_PCM_FORMAT_DSD_U32_BE] = {
    .width = 32, .phys = 32, .le = 0, .signd = 0,
    .silence = { 0x69, 0x69, 0x69, 0x69 },
    },
// FIXME: the following two formats are not defined properly yet
    [SNDRV_PCM_FORMAT_MPEG] = {
    .le = -1, .signd = -1,
    },
    [SNDRV_PCM_FORMAT_GSM] = {
    .le = -1, .signd = -1,
    },
    [SNDRV_PCM_FORMAT_S20_LE] = {
    .width = 20, .phys = 32, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S20_BE] = {
    .width = 20, .phys = 32, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U20_LE] = {
    .width = 20, .phys = 32, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x08, 0x00 },
    },
    [SNDRV_PCM_FORMAT_U20_BE] = {
    .width = 20, .phys = 32, .le = 0, .signd = 0,
    .silence = { 0x00, 0x08, 0x00, 0x00 },
    },
// FIXME: the following format is not defined properly yet
    [SNDRV_PCM_FORMAT_SPECIAL] = {
    .le = -1, .signd = -1,
    },
    [SNDRV_PCM_FORMAT_S24_3LE] = {
    .width = 24, .phys = 24, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S24_3BE] = {
    .width = 24, .phys = 24, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U24_3LE] = {
    .width = 24, .phys = 24, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x80 },
    },
    [SNDRV_PCM_FORMAT_U24_3BE] = {
    .width = 24, .phys = 24, .le = 0, .signd = 0,
    .silence = { 0x80, 0x00, 0x00 },
    },
    [SNDRV_PCM_FORMAT_S20_3LE] = {
    .width = 20, .phys = 24, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S20_3BE] = {
    .width = 20, .phys = 24, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U20_3LE] = {
    .width = 20, .phys = 24, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x08 },
    },
    [SNDRV_PCM_FORMAT_U20_3BE] = {
    .width = 20, .phys = 24, .le = 0, .signd = 0,
    .silence = { 0x08, 0x00, 0x00 },
    },
    [SNDRV_PCM_FORMAT_S18_3LE] = {
    .width = 18, .phys = 24, .le = 1, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_S18_3BE] = {
    .width = 18, .phys = 24, .le = 0, .signd = 1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_U18_3LE] = {
    .width = 18, .phys = 24, .le = 1, .signd = 0,
    .silence = { 0x00, 0x00, 0x02 },
    },
    [SNDRV_PCM_FORMAT_U18_3BE] = {
    .width = 18, .phys = 24, .le = 0, .signd = 0,
    .silence = { 0x02, 0x00, 0x00 },
    },
    [SNDRV_PCM_FORMAT_G723_24_1B] = {
    .width = 3, .phys = 8, .le = -1, .signd = -1,
    .silence = {},
    },
    [SNDRV_PCM_FORMAT_G723_40_1B] = {
    .width = 5, .phys = 8, .le = -1, .signd = -1,
    .silence = {},
    },
    };
//
// snd_pcm_format_signed - Check the PCM format is signed linear
// @format: the format to check
//
// Return: 1 if the given PCM format is signed linear, 0 if unsigned
// linear, and a negative error code for non-linear formats.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_signed(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_signed(snd_pcm_format_t format)
    {
    int val;
    if (!valid_format(format))
    return -EINVAL;
    val = pcm_formats[format].signd;
    if (val < 0)
    return -EINVAL;
    return val;
    }
    EXPORT_SYMBOL(snd_pcm_format_signed);
//
// snd_pcm_format_unsigned - Check the PCM format is unsigned linear
// @format: the format to check
//
// Return: 1 if the given PCM format is unsigned linear, 0 if signed
// linear, and a negative error code for non-linear formats.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_unsigned(snd_pcm_format_t format)
    {
    int val;
    val = snd_pcm_format_signed(format);
    if (val < 0)
    return val;
    return !val;
    }
    EXPORT_SYMBOL(snd_pcm_format_unsigned);
//
// snd_pcm_format_linear - Check the PCM format is linear
// @format: the format to check
//
// Return: 1 if the given PCM format is linear, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_linear(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_linear(snd_pcm_format_t format)
    {
    return snd_pcm_format_signed(format) >= 0;
    }
    EXPORT_SYMBOL(snd_pcm_format_linear);
//
// snd_pcm_format_little_endian - Check the PCM format is little-endian
// @format: the format to check
//
// Return: 1 if the given PCM format is little-endian, 0 if
// big-endian, or a negative error code if endian not specified.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_little_endian(snd_pcm_format_t format)
    {
    int val;
    if (!valid_format(format))
    return -EINVAL;
    val = pcm_formats[format].le;
    if (val < 0)
    return -EINVAL;
    return val;
    }
    EXPORT_SYMBOL(snd_pcm_format_little_endian);
//
// snd_pcm_format_big_endian - Check the PCM format is big-endian
// @format: the format to check
//
// Return: 1 if the given PCM format is big-endian, 0 if
// little-endian, or a negative error code if endian not specified.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_big_endian(snd_pcm_format_t format)
    {
    int val;
    val = snd_pcm_format_little_endian(format);
    if (val < 0)
    return val;
    return !val;
    }
    EXPORT_SYMBOL(snd_pcm_format_big_endian);
//
// snd_pcm_format_width - return the bit-width of the format
// @format: the format to check
//
// Return: The bit-width of the format, or a negative error code
// if unknown format.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_width(snd_pcm_format_t format)
    {
    int val;
    if (!valid_format(format))
    return -EINVAL;
    val = pcm_formats[format].width;
    if (!val)
    return -EINVAL;
    return val;
    }
    EXPORT_SYMBOL(snd_pcm_format_width);
//
// snd_pcm_format_physical_width - return the physical bit-width of the format
// @format: the format to check
//
// Return: The physical bit-width of the format, or a negative error code
// if unknown format.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int {
    int snd_pcm_format_physical_width(snd_pcm_format_t format)
    {
    int val;
    if (!valid_format(format))
    return -EINVAL;
    val = pcm_formats[format].phys;
    if (!val)
    return -EINVAL;
    return val;
    }
    EXPORT_SYMBOL(snd_pcm_format_physical_width);
//
// snd_pcm_format_size - return the byte size of samples on the given format
// @format: the format to check
// @samples: sampling rate
//
// Return: The byte size of the given samples for the format, or a
// negative error code if unknown format.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_size(format: snd_pcm_format_t, samples: usize) -> isize {
    ssize_t snd_pcm_format_size(snd_pcm_format_t format, size_t samples)
    {
    let mut phys_width: c_int = snd_pcm_format_physical_width(format);
    if (phys_width < 0)
    return -EINVAL;
    return samples * phys_width / 8;
    }
    EXPORT_SYMBOL(snd_pcm_format_size);
//
// snd_pcm_format_silence_64 - return the silent data in 8 bytes array
// @format: the format to check
//
// Return: The format pattern to fill or %NULL if error.
//
    const unsigned char *snd_pcm_format_silence_64(snd_pcm_format_t format)
    {
    if (!valid_format(format))
    return core::ptr::null_mut();
    if (! pcm_formats[format].phys)
    return core::ptr::null_mut();
    return pcm_formats[format].silence;
    }
    EXPORT_SYMBOL(snd_pcm_format_silence_64);
//
// snd_pcm_format_set_silence - set the silence data on the buffer
// @format: the PCM format
// @data: the buffer pointer
// @samples: the number of samples to set silence
//
// Sets the silence data on the buffer for the given samples.
//
// Return: Zero if successful, or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: c_uint) -> c_int {
    int snd_pcm_format_set_silence(snd_pcm_format_t format, void *data, unsigned int samples)
    {
    int width;
    unsigned char *dst;
    const unsigned char *pat;
    if (!valid_format(format))
    return -EINVAL;
    if (samples == 0)
    return 0;
    width = pcm_formats[format].phys; /* physical width */
    if (!width)
    return -EINVAL;
    pat = pcm_formats[format].silence;
// signed or 1 byte data
    if (pcm_formats[format].signd == 1 || width <= 8) {
    let mut bytes: c_uint = samples * width / 8;
    memset(data, *pat, bytes);
    return 0;
    }
// non-zero samples, fill using a loop
    width /= 8;
    dst = data;

    while (samples--) {
    memcpy(dst, pat, width);
    dst += width;
    }

// a bit optimization for constant width
    switch (width) {
    case 2:
    while (samples--) {
    memcpy(dst, pat, 2);
    dst += 2;
    }
    break;
    case 3:
    while (samples--) {
    memcpy(dst, pat, 3);
    dst += 3;
    }
    break;
    case 4:
    while (samples--) {
    memcpy(dst, pat, 4);
    dst += 4;
    }
    break;
    case 8:
    while (samples--) {
    memcpy(dst, pat, 8);
    dst += 8;
    }
    break;
    }

    return 0;
    }
    EXPORT_SYMBOL(snd_pcm_format_set_silence);
//
// snd_pcm_hw_limit_rates - determine rate_min/rate_max fields
// @hw: the pcm hw instance
//
// Determines the rate_min and rate_max fields from the rates bits of
// the given hw.
//
// Return: Zero if successful.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_hw_limit_rates(hw: *mut snd_pcm_hardware) -> c_int {
    int snd_pcm_hw_limit_rates(struct snd_pcm_hardware *hw)
    {
    int i;
    unsigned int rmin, rmax;
    rmin = UINT_MAX;
    rmax = 0;
    for (i = 0; i < (int)snd_pcm_known_rates.count; i++) {
    if (hw.rates & (1 << i)) {
    rmin = min(rmin, snd_pcm_known_rates.list[i]);
    rmax = max(rmax, snd_pcm_known_rates.list[i]);
    }
    }
    if (rmin > rmax)
    return -EINVAL;
    hw.rate_min = rmin;
    hw.rate_max = rmax;
    return 0;
    }
    EXPORT_SYMBOL(snd_pcm_hw_limit_rates);
//
// snd_pcm_rate_to_rate_bit - converts sample rate to SNDRV_PCM_RATE_xxx bit
// @rate: the sample rate to convert
//
// Return: The SNDRV_PCM_RATE_xxx flag that corresponds to the given rate, or
// SNDRV_PCM_RATE_KNOT for an unknown rate.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint {
    unsigned int snd_pcm_rate_to_rate_bit(unsigned int rate)
    {
    unsigned int i;
    for (i = 0; i < snd_pcm_known_rates.count; i++)
    if (snd_pcm_known_rates.list[i] == rate)
    return 1u << i;
    return SNDRV_PCM_RATE_KNOT;
    }
    EXPORT_SYMBOL(snd_pcm_rate_to_rate_bit);
//
// snd_pcm_rate_bit_to_rate - converts SNDRV_PCM_RATE_xxx bit to sample rate
// @rate_bit: the rate bit to convert
//
// Return: The sample rate that corresponds to the given SNDRV_PCM_RATE_xxx flag
// or 0 for an unknown rate bit.
//
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_rate_bit_to_rate(rate_bit: c_uint) -> c_uint {
    unsigned int snd_pcm_rate_bit_to_rate(unsigned int rate_bit)
    {
    unsigned int i;
    for (i = 0; i < snd_pcm_known_rates.count; i++)
    if ((1u << i) == rate_bit)
    return snd_pcm_known_rates.list[i];
    return 0;
    }
    EXPORT_SYMBOL(snd_pcm_rate_bit_to_rate);
#[no_mangle]
unsafe extern "C" fn snd_pcm_rate_mask_sanitize(rates: c_uint) -> c_uint {
    static unsigned int snd_pcm_rate_mask_sanitize(unsigned int rates)
    {
    if (rates & SNDRV_PCM_RATE_CONTINUOUS)
    return SNDRV_PCM_RATE_CONTINUOUS;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_RATE_KNOT: rates &) -> else {
    else if (rates & SNDRV_PCM_RATE_KNOT)
    return SNDRV_PCM_RATE_KNOT;
    return rates;
    }
//
// snd_pcm_rate_mask_intersect - computes the intersection between two rate masks
// @rates_a: The first rate mask
// @rates_b: The second rate mask
//
// This function computes the rates that are supported by both rate masks passed
// to the function. It will take care of the special handling of
// SNDRV_PCM_RATE_CONTINUOUS and SNDRV_PCM_RATE_KNOT.
//
// Return: A rate mask containing the rates that are supported by both rates_a
// and rates_b.
//
    unsigned int snd_pcm_rate_mask_intersect(unsigned int rates_a,
    unsigned int rates_b)
    {
    rates_a = snd_pcm_rate_mask_sanitize(rates_a);
    rates_b = snd_pcm_rate_mask_sanitize(rates_b);
    if (rates_a & SNDRV_PCM_RATE_CONTINUOUS)
    return rates_b;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_RATE_CONTINUOUS: rates_b &) -> else {
    else if (rates_b & SNDRV_PCM_RATE_CONTINUOUS)
    return rates_a;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_RATE_KNOT: rates_a &) -> else {
    else if (rates_a & SNDRV_PCM_RATE_KNOT)
    return rates_b;
#[no_mangle]
pub unsafe extern "C" fn if(SNDRV_PCM_RATE_KNOT: rates_b &) -> else {
    else if (rates_b & SNDRV_PCM_RATE_KNOT)
    return rates_a;
    return rates_a & rates_b;
    }
    EXPORT_SYMBOL_GPL(snd_pcm_rate_mask_intersect);
