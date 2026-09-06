//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/cmedia.c
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
// Universal codec driver for Intel High Definition Audio Codec
//
// HD audio codec driver for C-Media CMI9880
//
// Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
//

#[no_mangle]
unsafe extern "C" fn cmedia_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    static int cmedia_probe(struct hda_codec *codec, const struct hda_device_id *id)
    {
    struct hda_gen_spec *spec;
    struct auto_pin_cfg *cfg;
    let mut is_cmi8888: bool = id.vendor_id == 0x13f68888;
    int err;
    spec = kzalloc_obj(*spec);
    if (spec == core::ptr::null_mut())
    return -ENOMEM;
    codec.spec = spec;
    cfg = &spec.autocfg;
    snd_hda_gen_spec_init(spec);
    if (is_cmi8888) {
// mask NID 0x10 from the playback volume selection;
// it's a headphone boost volume handled manually below
//
    spec.out_vol_mask = (1ULL << 0x10);
    }
    err = snd_hda_parse_pin_defcfg(codec, cfg, core::ptr::null_mut(), 0);
    if (err < 0)
    goto error;
    err = snd_hda_gen_parse_auto_config(codec, cfg);
    if (err < 0)
    goto error;
    if (is_cmi8888) {
    if (get_defcfg_device(snd_hda_codec_get_pincfg(codec, 0x10)) ==
    AC_JACK_HP_OUT) {
    static const struct snd_kcontrol_new amp_kctl =
    HDA_CODEC_VOLUME("Headphone Amp Playback Volume",
    0x10, 0, HDA_OUTPUT);
    if (!snd_hda_gen_add_kctl(spec, core::ptr::null_mut(), &amp_kctl)) {
    err = -ENOMEM;
    goto error;
    }
    }
    }
    return 0;
    error:
    snd_hda_gen_remove(codec);
    return err;
    }
    static const struct hda_codec_ops cmedia_codec_ops = {
    .probe = cmedia_probe,
    .remove = snd_hda_gen_remove,
    .build_controls = snd_hda_gen_build_controls,
    .build_pcms = snd_hda_gen_build_pcms,
    .init = snd_hda_gen_init,
    .unsol_event = snd_hda_jack_unsol_event,
    .check_power_status = snd_hda_gen_check_power_status,
    .stream_pm = snd_hda_gen_stream_pm,
    };
//
// driver entries
//
    static const struct hda_device_id snd_hda_id_cmedia[] = {
    HDA_CODEC_ID(0x13f68888, "CMI8888"),
    HDA_CODEC_ID(0x13f69880, "CMI9880"),
    HDA_CODEC_ID(0x434d4980, "CMI9880"),
    {} /* terminator */
    };
    MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_cmedia);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("C-Media HD-audio codec");
    static struct hda_codec_driver cmedia_driver = {
    .id = snd_hda_id_cmedia,
    .ops = &cmedia_codec_ops,
    };
    module_hda_codec_driver(cmedia_driver);
