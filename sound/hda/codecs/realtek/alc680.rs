//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/realtek/alc680.c
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
// Realtek ALC680 codec
//

#[no_mangle]
unsafe extern "C" fn alc680_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static int alc680_parse_auto_config(struct hda_codec *codec)
    {
    return alc_parse_auto_config(codec, core::ptr::null_mut(), core::ptr::null_mut());
    }
//
#[no_mangle]
unsafe extern "C" fn alc680_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    static int alc680_probe(struct hda_codec *codec, const struct hda_device_id *id)
    {
    int err;
// ALC680 has no aa-loopback mixer
    err = alc_alloc_spec(codec, 0);
    if (err < 0)
    return err;
// automatic parse from the BIOS config
    err = alc680_parse_auto_config(codec);
    if (err < 0) {
    snd_hda_gen_remove(codec);
    return err;
    }
    return 0;
    }
    static const struct hda_codec_ops alc680_codec_ops = {
    .probe = alc680_probe,
    .remove = snd_hda_gen_remove,
    .build_controls = alc_build_controls,
    .build_pcms = snd_hda_gen_build_pcms,
    .init = alc_init,
    .unsol_event = snd_hda_jack_unsol_event,
    .resume = alc_resume,
    .suspend = alc_suspend,
    .check_power_status = snd_hda_gen_check_power_status,
    .stream_pm = snd_hda_gen_stream_pm,
    };
//
// driver entries
//
    static const struct hda_device_id snd_hda_id_alc680[] = {
    HDA_CODEC_ID(0x10ec0680, "ALC680"),
    {} /* terminator */
    };
    MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc680);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Realtek ALC680 HD-audio codec");
    MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK");
    static struct hda_codec_driver alc680_driver = {
    .id = snd_hda_id_alc680,
    .ops = &alc680_codec_ops,
    };
    module_hda_codec_driver(alc680_driver);
