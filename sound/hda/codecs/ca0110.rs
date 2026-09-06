//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/ca0110.c
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
// HD audio codec driver for Creative X-Fi CA0110-IBG chip
//
// Copyright (c) 2008 Takashi Iwai <tiwai@suse.de>
//

#[no_mangle]
unsafe extern "C" fn ca0110_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static int ca0110_parse_auto_config(struct hda_codec *codec)
    {
    struct hda_gen_spec *spec = codec.spec;
    int err;
    err = snd_hda_parse_pin_defcfg(codec, &spec.autocfg, core::ptr::null_mut(), 0);
    if (err < 0)
    return err;
    err = snd_hda_gen_parse_auto_config(codec, &spec.autocfg);
    if (err < 0)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ca0110_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    static int ca0110_probe(struct hda_codec *codec, const struct hda_device_id *id)
    {
    struct hda_gen_spec *spec;
    int err;
    spec = kzalloc_obj(*spec);
    if (!spec)
    return -ENOMEM;
    snd_hda_gen_spec_init(spec);
    codec.spec = spec;
    spec.multi_cap_vol = 1;
    codec.bus.core.needs_damn_long_delay = 1;
    err = ca0110_parse_auto_config(codec);
    if (err < 0)
    goto error;
    return 0;
    error:
    snd_hda_gen_remove(codec);
    return err;
    }
    static const struct hda_codec_ops ca0110_codec_ops = {
    .probe = ca0110_probe,
    .remove = snd_hda_gen_remove,
    .build_controls = snd_hda_gen_build_controls,
    .build_pcms = snd_hda_gen_build_pcms,
    .init = snd_hda_gen_init,
    .unsol_event = snd_hda_jack_unsol_event,
    };
//
// driver entries
//
    static const struct hda_device_id snd_hda_id_ca0110[] = {
    HDA_CODEC_ID(0x1102000a, "CA0110-IBG"),
    HDA_CODEC_ID(0x1102000b, "CA0110-IBG"),
    HDA_CODEC_ID(0x1102000d, "SB0880 X-Fi"),
    {} /* terminator */
    };
    MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_ca0110);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Creative CA0110-IBG HD-audio codec");
    static struct hda_codec_driver ca0110_driver = {
    .id = snd_hda_id_ca0110,
    .ops = &ca0110_codec_ops,
    };
    module_hda_codec_driver(ca0110_driver);
