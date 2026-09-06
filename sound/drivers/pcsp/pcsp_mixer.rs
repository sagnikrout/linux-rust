//! Automatically rewritten from C to Rust
//! Source: sound/drivers/pcsp/pcsp_mixer.c
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


// SPDX-License-Identifier: GPL-2.0
//
// PC-Speaker driver for Linux
//
// Mixer implementation.
// Copyright (C) 2001-2008  Stas Sergeev
//

    static int pcsp_enable_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 1;
    return 0;
    }
    static int pcsp_enable_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] = chip.enable;
    return 0;
    }
    static int pcsp_enable_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    let mut changed: c_int = 0;
    let mut enab: c_int = ucontrol.value.integer.value[0];
    if (enab != chip.enable) {
    chip.enable = enab;
    changed = 1;
    }
    return changed;
    }
    static int pcsp_treble_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    uinfo.type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    uinfo.count = 1;
    uinfo.value.enumerated.items = chip.max_treble + 1;
    if (uinfo.value.enumerated.item > chip.max_treble)
    uinfo.value.enumerated.item = chip.max_treble;
    sprintf(uinfo.value.enumerated.name, "%lu",
    (unsigned long)PCSP_CALC_RATE(uinfo.value.enumerated.item));
    return 0;
    }
    static int pcsp_treble_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    ucontrol.value.enumerated.item[0] = chip.treble;
    return 0;
    }
    static int pcsp_treble_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    let mut changed: c_int = 0;
    let mut treble: c_int = ucontrol.value.enumerated.item[0];
    if (treble != chip.treble) {
    chip.treble = treble;

    dev_dbg(chip.card.dev, "PCSP: rate set to %li\n", PCSP_RATE());

    changed = 1;
    }
    return changed;
    }
    static int pcsp_pcspkr_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 1;
    return 0;
    }
    static int pcsp_pcspkr_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] = chip.pcspkr;
    return 0;
    }
    static int pcsp_pcspkr_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_pcsp *chip = snd_kcontrol_chip(kcontrol);
    let mut changed: c_int = 0;
    let mut spkr: c_int = ucontrol.value.integer.value[0];
    if (spkr != chip.pcspkr) {
    chip.pcspkr = spkr;
    changed = 1;
    }
    return changed;
    }

    { \
    .iface =	SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name =		ctl_name, \
    .info =		pcsp_##ctl_type##_info, \
    .get =		pcsp_##ctl_type##_get, \
    .put =		pcsp_##ctl_type##_put, \
    }
    static const struct snd_kcontrol_new snd_pcsp_controls_pcm[] = {
    PCSP_MIXER_CONTROL(enable, "Master Playback Switch"),
    PCSP_MIXER_CONTROL(treble, "BaseFRQ Playback Volume"),
    };
    static const struct snd_kcontrol_new snd_pcsp_controls_spkr[] = {
    PCSP_MIXER_CONTROL(pcspkr, "Beep Playback Switch"),
    };
    static int snd_pcsp_ctls_add(struct snd_pcsp *chip,
    const struct snd_kcontrol_new *ctls, int num)
    {
    int i, err;
    struct snd_card *card = chip.card;
    for (i = 0; i < num; i++) {
    err = snd_ctl_add(card, snd_ctl_new1(ctls + i, chip));
    if (err < 0)
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_pcsp_new_mixer(chip: *mut snd_pcsp, nopcm: c_int) -> c_int {
    int snd_pcsp_new_mixer(struct snd_pcsp *chip, int nopcm)
    {
    int err;
    struct snd_card *card = chip.card;
    if (!nopcm) {
    err = snd_pcsp_ctls_add(chip, snd_pcsp_controls_pcm,
    ARRAY_SIZE(snd_pcsp_controls_pcm));
    if (err < 0)
    return err;
    }
    err = snd_pcsp_ctls_add(chip, snd_pcsp_controls_spkr,
    ARRAY_SIZE(snd_pcsp_controls_spkr));
    if (err < 0)
    return err;
    strscpy(card.mixername, "PC-Speaker");
    return 0;
    }
