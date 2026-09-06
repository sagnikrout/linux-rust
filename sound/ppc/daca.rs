//! Automatically rewritten from C to Rust
//! Source: sound/ppc/daca.c
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
// PMac DACA lowlevel functions
//
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//

// i2c address
pub const DACA_I2C_ADDR: c_uint = 0x4d;
// registers
pub const DACA_REG_SR: c_uint = 0x01;
pub const DACA_REG_AVOL: c_uint = 0x02;
pub const DACA_REG_GCFG: c_uint = 0x03;
// maximum volume value
pub const DACA_VOL_MAX: c_uint = 0x38;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_daca {
    pub i2c: pmac_keywest,
    pub right_vol: int left_vol,,
    pub 1: unsigned int deemphasis :,
    pub 1: unsigned int amp_on :,
}

//
// initialize / detect DACA
//
#[no_mangle]
unsafe extern "C" fn daca_init_client(i2c: *mut pmac_keywest) -> c_int {
    static int daca_init_client(struct pmac_keywest *i2c)
    {
    let mut wdata: c_ushort = 0x00;
// SR: no swap, 1bit delay, 32-48kHz
// GCFG: power amp inverted, DAC on
    if (i2c_smbus_write_byte_data(i2c.client, DACA_REG_SR, 0x08) < 0 ||
    i2c_smbus_write_byte_data(i2c.client, DACA_REG_GCFG, 0x05) < 0)
    return -EINVAL;
    return i2c_smbus_write_block_data(i2c.client, DACA_REG_AVOL,
    2, (unsigned char*)&wdata);
    }
//
// update volume
//
#[no_mangle]
unsafe extern "C" fn daca_set_volume(mix: *mut pmac_daca) -> c_int {
    static int daca_set_volume(struct pmac_daca *mix)
    {
    unsigned char data[2];
    if (! mix.i2c.client)
    return -ENODEV;
    if (mix.left_vol > DACA_VOL_MAX)
    data[0] = DACA_VOL_MAX;
    else
    data[0] = mix.left_vol;
    if (mix.right_vol > DACA_VOL_MAX)
    data[1] = DACA_VOL_MAX;
    else
    data[1] = mix.right_vol;
    data[1] |= mix.deemphasis ? 0x40 : 0;
    if (i2c_smbus_write_block_data(mix.i2c.client, DACA_REG_AVOL,
    2, data) < 0) {
    dev_err(&mix.i2c.client.dev, "failed to set volume\n");
    return -EINVAL;
    }
    return 0;
    }
// deemphasis switch

    static int daca_get_deemphasis(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.deemphasis ? 1 : 0;
    return 0;
    }
    static int daca_put_deemphasis(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    change = mix.deemphasis != ucontrol.value.integer.value[0];
    if (change) {
    mix.deemphasis = !!ucontrol.value.integer.value[0];
    daca_set_volume(mix);
    }
    return change;
    }
// output volume
    static int daca_info_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 2;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = DACA_VOL_MAX;
    return 0;
    }
    static int daca_get_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.left_vol;
    ucontrol.value.integer.value[1] = mix.right_vol;
    return 0;
    }
    static int daca_put_volume(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    unsigned int vol[2];
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    vol[0] = ucontrol.value.integer.value[0];
    vol[1] = ucontrol.value.integer.value[1];
    if (vol[0] > DACA_VOL_MAX || vol[1] > DACA_VOL_MAX)
    return -EINVAL;
    change = mix.left_vol != vol[0] ||
    mix.right_vol != vol[1];
    if (change) {
    mix.left_vol = vol[0];
    mix.right_vol = vol[1];
    daca_set_volume(mix);
    }
    return change;
    }
// amplifier switch

    static int daca_get_amp(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    ucontrol.value.integer.value[0] = mix.amp_on ? 1 : 0;
    return 0;
    }
    static int daca_put_amp(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pmac *chip = snd_kcontrol_chip(kcontrol);
    struct pmac_daca *mix;
    int change;
    mix = chip.mixer_data;
    if (!mix)
    return -ENODEV;
    change = mix.amp_on != ucontrol.value.integer.value[0];
    if (change) {
    mix.amp_on = !!ucontrol.value.integer.value[0];
    i2c_smbus_write_byte_data(mix.i2c.client, DACA_REG_GCFG,
    mix.amp_on ? 0x05 : 0x04);
    }
    return change;
    }
    static const struct snd_kcontrol_new daca_mixers[] = {
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Deemphasis Switch",
    .info = daca_info_deemphasis,
    .get = daca_get_deemphasis,
    .put = daca_put_deemphasis
    },
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Master Playback Volume",
    .info = daca_info_volume,
    .get = daca_get_volume,
    .put = daca_put_volume
    },
    { .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Power Amplifier Switch",
    .info = daca_info_amp,
    .get = daca_get_amp,
    .put = daca_put_amp
    },
    };

#[no_mangle]
unsafe extern "C" fn daca_resume(chip: *mut snd_pmac) {
    static void daca_resume(struct snd_pmac *chip)
    {
    struct pmac_daca *mix = chip.mixer_data;
    i2c_smbus_write_byte_data(mix.i2c.client, DACA_REG_SR, 0x08);
    i2c_smbus_write_byte_data(mix.i2c.client, DACA_REG_GCFG,
    mix.amp_on ? 0x05 : 0x04);
    daca_set_volume(mix);
    }

#[no_mangle]
unsafe extern "C" fn daca_cleanup(chip: *mut snd_pmac) {
    static void daca_cleanup(struct snd_pmac *chip)
    {
    struct pmac_daca *mix = chip.mixer_data;
    if (! mix)
    return;
    snd_pmac_keywest_cleanup(&mix.i2c);
    kfree(mix);
    chip.mixer_data = core::ptr::null_mut();
    }
// exported
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_daca_init(chip: *mut snd_pmac) -> c_int {
    int snd_pmac_daca_init(struct snd_pmac *chip)
    {
    int i, err;
    struct pmac_daca *mix;
    request_module("i2c-powermac");
    mix = kzalloc_obj(*mix);
    if (! mix)
    return -ENOMEM;
    chip.mixer_data = mix;
    chip.mixer_free = daca_cleanup;
    mix.amp_on = 1; /* default on */
    mix.i2c.addr = DACA_I2C_ADDR;
    mix.i2c.init_client = daca_init_client;
    mix.i2c.name = "DACA";
    err = snd_pmac_keywest_init(&mix.i2c);
    if (err < 0)
    return err;
//
// build mixers
//
    strscpy(chip.card.mixername, "PowerMac DACA");
    for (i = 0; i < ARRAY_SIZE(daca_mixers); i++) {
    err = snd_ctl_add(chip.card, snd_ctl_new1(&daca_mixers[i], chip));
    if (err < 0)
    return err;
    }

    chip.resume = daca_resume;

    return 0;
    }
