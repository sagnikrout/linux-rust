//! Automatically rewritten from C to Rust
//! Source: sound/i2c/other/pt2258.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA Driver for the PT2258 volume controller.
//
// Copyright (c) 2006  Jochen Voss <voss@seehuhn.de>
//

    MODULE_AUTHOR("Jochen Voss <voss@seehuhn.de>");
    MODULE_DESCRIPTION("PT2258 volume controller (Princeton Technology Corp.)");
    MODULE_LICENSE("GPL");
pub const PT2258_CMD_RESET: c_uint = 0xc0;
pub const PT2258_CMD_UNMUTE: c_uint = 0xf8;
pub const PT2258_CMD_MUTE: c_uint = 0xf9;
    static const unsigned char pt2258_channel_code[12] = {
    0x80, 0x90,		/* channel 1: -10dB, -1dB */
    0x40, 0x50,		/* channel 2: -10dB, -1dB */
    0x00, 0x10,		/* channel 3: -10dB, -1dB */
    0x20, 0x30,		/* channel 4: -10dB, -1dB */
    0x60, 0x70,		/* channel 5: -10dB, -1dB */
    0xa0, 0xb0		/* channel 6: -10dB, -1dB */
    };
#[no_mangle]
pub unsafe extern "C" fn snd_pt2258_reset(pt: *mut snd_pt2258) -> c_int {
    int snd_pt2258_reset(struct snd_pt2258 *pt)
    {
    unsigned char bytes[2];
    int i;
// reset chip
    bytes[0] = PT2258_CMD_RESET;
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 1) != 1)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
// mute all channels
    pt.mute = 1;
    bytes[0] = PT2258_CMD_MUTE;
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 1) != 1)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
// set all channels to 0dB
    for (i = 0; i < 6; ++i)
    pt.volume[i] = 0;
    bytes[0] = 0xd0;
    bytes[1] = 0xe0;
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 2) != 2)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
    return 0;
    __error:
    snd_i2c_unlock(pt.i2c_bus);
    dev_err(pt.card.dev, "PT2258 reset failed\n");
    return -EIO;
    }
    static int pt2258_stereo_volume_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 79;
    return 0;
    }
    static int pt2258_stereo_volume_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pt2258 *pt = snd_kcontrol_chip(kcontrol);
    let mut base: c_int = kcontrol.private_value;
// chip does not support register reads
    ucontrol.value.integer.value[0] = 79 - pt.volume[base];
    ucontrol.value.integer.value[1] = 79 - pt.volume[base + 1];
    return 0;
    }
    static int pt2258_stereo_volume_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pt2258 *pt = snd_kcontrol_chip(kcontrol);
    let mut base: c_int = kcontrol.private_value;
    unsigned char bytes[2];
    int val0, val1;
    val0 = 79 - ucontrol.value.integer.value[0];
    val1 = 79 - ucontrol.value.integer.value[1];
    if (val0 < 0 || val0 > 79 || val1 < 0 || val1 > 79)
    return -EINVAL;
    if (val0 == pt.volume[base] && val1 == pt.volume[base + 1])
    return 0;
    pt.volume[base] = val0;
    bytes[0] = pt2258_channel_code[2 * base] | (val0 / 10);
    bytes[1] = pt2258_channel_code[2 * base + 1] | (val0 % 10);
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 2) != 2)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
    pt.volume[base + 1] = val1;
    bytes[0] = pt2258_channel_code[2 * base + 2] | (val1 / 10);
    bytes[1] = pt2258_channel_code[2 * base + 3] | (val1 % 10);
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 2) != 2)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
    return 1;
    __error:
    snd_i2c_unlock(pt.i2c_bus);
    dev_err(pt.card.dev, "PT2258 access failed\n");
    return -EIO;
    }

    static int pt2258_switch_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pt2258 *pt = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] = !pt.mute;
    return 0;
    }
    static int pt2258_switch_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pt2258 *pt = snd_kcontrol_chip(kcontrol);
    unsigned char bytes[2];
    int val;
    val = !ucontrol.value.integer.value[0];
    if (pt.mute == val)
    return 0;
    pt.mute = val;
    bytes[0] = val ? PT2258_CMD_MUTE : PT2258_CMD_UNMUTE;
    snd_i2c_lock(pt.i2c_bus);
    if (snd_i2c_sendbytes(pt.i2c_dev, bytes, 1) != 1)
    goto __error;
    snd_i2c_unlock(pt.i2c_bus);
    return 1;
    __error:
    snd_i2c_unlock(pt.i2c_bus);
    dev_err(pt.card.dev, "PT2258 access failed 2\n");
    return -EIO;
    }
    static const DECLARE_TLV_DB_SCALE(pt2258_db_scale, -7900, 100, 0);
#[no_mangle]
pub unsafe extern "C" fn snd_pt2258_build_controls(pt: *mut snd_pt2258) -> c_int {
    int snd_pt2258_build_controls(struct snd_pt2258 *pt)
    {
    struct snd_kcontrol_new knew;
    char *names[3] = {
    "Mic Loopback Playback Volume",
    "Line Loopback Playback Volume",
    "CD Loopback Playback Volume"
    };
    int i, err;
    for (i = 0; i < 3; ++i) {
    memset(&knew, 0, sizeof(knew));
    knew.name = names[i];
    knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    knew.count = 1;
    knew.access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ;
    knew.private_value = 2 * i;
    knew.info = pt2258_stereo_volume_info;
    knew.get = pt2258_stereo_volume_get;
    knew.put = pt2258_stereo_volume_put;
    knew.tlv.p = pt2258_db_scale;
    err = snd_ctl_add(pt.card, snd_ctl_new1(&knew, pt));
    if (err < 0)
    return err;
    }
    memset(&knew, 0, sizeof(knew));
    knew.name = "Loopback Switch";
    knew.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    knew.info = pt2258_switch_info;
    knew.get = pt2258_switch_get;
    knew.put = pt2258_switch_put;
    knew.access = 0;
    err = snd_ctl_add(pt.card, snd_ctl_new1(&knew, pt));
    if (err < 0)
    return err;
    return 0;
    }
    EXPORT_SYMBOL(snd_pt2258_reset);
    EXPORT_SYMBOL(snd_pt2258_build_controls);
