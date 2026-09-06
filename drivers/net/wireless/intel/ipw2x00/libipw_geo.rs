//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/ipw2x00/libipw_geo.c
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
    Copyright(c) 2005 Intel Corporation. All rights reserved.
    Contact Information:
    Intel Linux Wireless <ilw@linux.intel.com>
    Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//

#[no_mangle]
pub unsafe extern "C" fn libipw_is_valid_channel(ieee: *mut libipw_device, channel: u8) -> c_int {
    int libipw_is_valid_channel(struct libipw_device *ieee, u8 channel)
    {
    int i;
// Driver needs to initialize the geography map before using
// these helper functions
    if (ieee.geo.bg_channels == 0 && ieee.geo.a_channels == 0)
    return 0;
    if (ieee.freq_band & LIBIPW_24GHZ_BAND)
    for (i = 0; i < ieee.geo.bg_channels; i++)
// NOTE: If G mode is currently supported but
// this is a B only channel, we don't see it
// as valid.
    if ((ieee.geo.bg[i].channel == channel) &&
    !(ieee.geo.bg[i].flags & LIBIPW_CH_INVALID) &&
    (!(ieee.mode & IEEE_G) ||
    !(ieee.geo.bg[i].flags & LIBIPW_CH_B_ONLY)))
    return LIBIPW_24GHZ_BAND;
    if (ieee.freq_band & LIBIPW_52GHZ_BAND)
    for (i = 0; i < ieee.geo.a_channels; i++)
    if ((ieee.geo.a[i].channel == channel) &&
    !(ieee.geo.a[i].flags & LIBIPW_CH_INVALID))
    return LIBIPW_52GHZ_BAND;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_channel_to_index(ieee: *mut libipw_device, channel: u8) -> c_int {
    int libipw_channel_to_index(struct libipw_device *ieee, u8 channel)
    {
    int i;
// Driver needs to initialize the geography map before using
// these helper functions
    if (ieee.geo.bg_channels == 0 && ieee.geo.a_channels == 0)
    return -1;
    if (ieee.freq_band & LIBIPW_24GHZ_BAND)
    for (i = 0; i < ieee.geo.bg_channels; i++)
    if (ieee.geo.bg[i].channel == channel)
    return i;
    if (ieee.freq_band & LIBIPW_52GHZ_BAND)
    for (i = 0; i < ieee.geo.a_channels; i++)
    if (ieee.geo.a[i].channel == channel)
    return i;
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_channel_to_freq(ieee: *mut *mut libipw_device, channel: u8) -> u32 {
    u32 libipw_channel_to_freq(struct libipw_device * ieee, u8 channel)
    {
    const struct libipw_channel * ch;
// Driver needs to initialize the geography map before using
// these helper functions
    if (ieee.geo.bg_channels == 0 && ieee.geo.a_channels == 0)
    return 0;
    ch = libipw_get_channel(ieee, channel);
    if (!ch.channel)
    return 0;
    return ch.freq;
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_freq_to_channel(ieee: *mut *mut libipw_device, freq: u32) -> u8 {
    u8 libipw_freq_to_channel(struct libipw_device * ieee, u32 freq)
    {
    int i;
// Driver needs to initialize the geography map before using
// these helper functions
    if (ieee.geo.bg_channels == 0 && ieee.geo.a_channels == 0)
    return 0;
    freq /= 100000;
    if (ieee.freq_band & LIBIPW_24GHZ_BAND)
    for (i = 0; i < ieee.geo.bg_channels; i++)
    if (ieee.geo.bg[i].freq == freq)
    return ieee.geo.bg[i].channel;
    if (ieee.freq_band & LIBIPW_52GHZ_BAND)
    for (i = 0; i < ieee.geo.a_channels; i++)
    if (ieee.geo.a[i].freq == freq)
    return ieee.geo.a[i].channel;
    return 0;
    }
    void libipw_set_geo(struct libipw_device *ieee,
    const struct libipw_geo *geo)
    {
    memcpy(ieee.geo.name, geo.name, 3);
    ieee.geo.name[3] = '\0';
    ieee.geo.bg_channels = geo.bg_channels;
    ieee.geo.a_channels = geo.a_channels;
    memcpy(ieee.geo.bg, geo.bg, geo.bg_channels *
    sizeof(struct libipw_channel));
    memcpy(ieee.geo.a, geo.a, ieee.geo.a_channels *
    sizeof(struct libipw_channel));
    }
    const struct libipw_geo *libipw_get_geo(struct libipw_device *ieee)
    {
    return &ieee.geo;
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_get_channel_flags(ieee: *mut *mut libipw_device, channel: u8) -> u8 {
    u8 libipw_get_channel_flags(struct libipw_device * ieee, u8 channel)
    {
    let mut index: c_int = libipw_channel_to_index(ieee, channel);
    if (index == -1)
    return LIBIPW_CH_INVALID;
    if (channel <= LIBIPW_24GHZ_CHANNELS)
    return ieee.geo.bg[index].flags;
    return ieee.geo.a[index].flags;
    }
    static const struct libipw_channel bad_channel = {
    .channel = 0,
    .flags = LIBIPW_CH_INVALID,
    .max_power = 0,
    };
    const struct libipw_channel *libipw_get_channel(struct libipw_device
// ieee, u8 channel)
    {
    let mut index: c_int = libipw_channel_to_index(ieee, channel);
    if (index == -1)
    return &bad_channel;
    if (channel <= LIBIPW_24GHZ_CHANNELS)
    return &ieee.geo.bg[index];
    return &ieee.geo.a[index];
    }
    EXPORT_SYMBOL(libipw_get_channel);
    EXPORT_SYMBOL(libipw_get_channel_flags);
    EXPORT_SYMBOL(libipw_is_valid_channel);
    EXPORT_SYMBOL(libipw_freq_to_channel);
    EXPORT_SYMBOL(libipw_channel_to_freq);
    EXPORT_SYMBOL(libipw_channel_to_index);
    EXPORT_SYMBOL(libipw_set_geo);
    EXPORT_SYMBOL(libipw_get_geo);
