//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/si3054.c
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
// Universal Interface for Intel High Definition Audio Codec
//
// HD audio codec driver for Silicon Labs 3054/5 modem codec
//
// Copyright (c) 2005 Sasha Khapyorsky <sashak@alsa-project.org>
// Takashi Iwai <tiwai@suse.de>
//

// si3054 verbs
pub const SI3054_VERB_READ_NODE: c_uint = 0x900;
pub const SI3054_VERB_WRITE_NODE: c_uint = 0x100;
// si3054 nodes (registers)
pub const SI3054_EXTENDED_MID: c_int = 2;
pub const SI3054_LINE_RATE: c_int = 3;
pub const SI3054_LINE_LEVEL: c_int = 4;
pub const SI3054_GPIO_CFG: c_int = 5;
pub const SI3054_GPIO_POLARITY: c_int = 6;
pub const SI3054_GPIO_STICKY: c_int = 7;
pub const SI3054_GPIO_WAKEUP: c_int = 8;
pub const SI3054_GPIO_STATUS: c_int = 9;
pub const SI3054_GPIO_CONTROL: c_int = 10;
pub const SI3054_MISC_AFE: c_int = 11;
pub const SI3054_CHIPID: c_int = 12;
pub const SI3054_LINE_CFG1: c_int = 13;
pub const SI3054_LINE_STATUS: c_int = 14;
pub const SI3054_DC_TERMINATION: c_int = 15;
pub const SI3054_LINE_CONFIG: c_int = 16;
pub const SI3054_CALLPROG_ATT: c_int = 17;
pub const SI3054_SQ_CONTROL: c_int = 18;
pub const SI3054_MISC_CONTROL: c_int = 19;
pub const SI3054_RING_CTRL1: c_int = 20;
pub const SI3054_RING_CTRL2: c_int = 21;
// extended MID
pub const SI3054_MEI_READY: c_uint = 0xf;
// line level
pub const SI3054_ATAG_MASK: c_uint = 0x00f0;
pub const SI3054_DTAG_MASK: c_uint = 0xf000;
// GPIO bits
pub const SI3054_GPIO_OH: c_uint = 0x0001;
pub const SI3054_GPIO_CID: c_uint = 0x0002;
// chipid and revisions
pub const SI3054_CHIPID_CODEC_REV_MASK: c_uint = 0x000f;
pub const SI3054_CHIPID_DAA_REV_MASK: c_uint = 0x00f0;
pub const SI3054_CHIPID_INTERNATIONAL: c_uint = 0x0100;
pub const SI3054_CHIPID_DAA_ID: c_uint = 0x0f00;

// si3054 codec registers (nodes) access macros

    snd_hda_codec_write_cache(codec,reg,0,SI3054_VERB_WRITE_NODE,val)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si3054_spec {
    pub international: unsigned,
}

//
// Modem mixer
//

    static int si3054_switch_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
    let mut reg: u16 = PRIVATE_REG(kcontrol.private_value);
    let mut mask: u16 = PRIVATE_MASK(kcontrol.private_value);
    uvalue.value.integer.value[0] = (GET_REG(codec, reg)) & mask ? 1 : 0 ;
    return 0;
    }
    static int si3054_switch_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *uvalue)
    {
    struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
    let mut reg: u16 = PRIVATE_REG(kcontrol.private_value);
    let mut mask: u16 = PRIVATE_MASK(kcontrol.private_value);
    if (uvalue.value.integer.value[0])
    SET_REG_CACHE(codec, reg, (GET_REG(codec, reg)) | mask);
    else
    SET_REG_CACHE(codec, reg, (GET_REG(codec, reg)) & ~mask);
    return 0;
    }

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name = kname, \
    .subdevice = HDA_SUBDEV_NID_FLAG | reg, \
    .info = si3054_switch_info, \
    .get  = si3054_switch_get, \
    .put  = si3054_switch_put, \
    .private_value = PRIVATE_VALUE(reg,mask), \
    }
    static const struct snd_kcontrol_new si3054_modem_mixer[] = {
    SI3054_KCONTROL("Off-hook Switch", SI3054_GPIO_CONTROL, SI3054_GPIO_OH),
    SI3054_KCONTROL("Caller ID Switch", SI3054_GPIO_CONTROL, SI3054_GPIO_CID),
    {}
    };
#[no_mangle]
unsafe extern "C" fn si3054_build_controls(codec: *mut hda_codec) -> c_int {
    static int si3054_build_controls(struct hda_codec *codec)
    {
    return snd_hda_add_new_ctls(codec, si3054_modem_mixer);
    }
//
// PCM callbacks
//
    static int si3054_pcm_prepare(struct hda_pcm_stream *hinfo,
    struct hda_codec *codec,
    unsigned int stream_tag,
    unsigned int format,
    struct snd_pcm_substream *substream)
    {
    u16 val;
    SET_REG(codec, SI3054_LINE_RATE, substream.runtime.rate);
    val = GET_REG(codec, SI3054_LINE_LEVEL);
    val &= 0xff << (8 * (substream.stream != SNDRV_PCM_STREAM_PLAYBACK));
    val |= ((stream_tag & 0xf) << 4) << (8 * (substream.stream == SNDRV_PCM_STREAM_PLAYBACK));
    SET_REG(codec, SI3054_LINE_LEVEL, val);
    snd_hda_codec_setup_stream(codec, hinfo.nid,
    stream_tag, 0, format);
    return 0;
    }
    static int si3054_pcm_open(struct hda_pcm_stream *hinfo,
    struct hda_codec *codec,
    struct snd_pcm_substream *substream)
    {
    static const unsigned int rates[] = { 8000, 9600, 16000 };
    static const struct snd_pcm_hw_constraint_list hw_constraints_rates = {
    .count = ARRAY_SIZE(rates),
    .list = rates,
    .mask = 0,
    };
    substream.runtime.hw.period_bytes_min = 80;
    return snd_pcm_hw_constraint_list(substream.runtime, 0,
    SNDRV_PCM_HW_PARAM_RATE, &hw_constraints_rates);
    }
    static const struct hda_pcm_stream si3054_pcm = {
    .substreams = 1,
    .channels_min = 1,
    .channels_max = 1,
    .nid = 0x1,
    .rates = SNDRV_PCM_RATE_8000|SNDRV_PCM_RATE_16000|SNDRV_PCM_RATE_KNOT,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    .maxbps = 16,
    .ops = {
    .open = si3054_pcm_open,
    .prepare = si3054_pcm_prepare,
    },
    };
#[no_mangle]
unsafe extern "C" fn si3054_build_pcms(codec: *mut hda_codec) -> c_int {
    static int si3054_build_pcms(struct hda_codec *codec)
    {
    struct hda_pcm *info;
    info = snd_hda_codec_pcm_new(codec, "Si3054 Modem");
    if (!info)
    return -ENOMEM;
    info.stream[SNDRV_PCM_STREAM_PLAYBACK] = si3054_pcm;
    info.stream[SNDRV_PCM_STREAM_CAPTURE]  = si3054_pcm;
    info.stream[SNDRV_PCM_STREAM_PLAYBACK].nid = codec.core.mfg;
    info.stream[SNDRV_PCM_STREAM_CAPTURE].nid = codec.core.mfg;
    info.pcm_type = HDA_PCM_TYPE_MODEM;
    return 0;
    }
//
// Init part
//
#[no_mangle]
unsafe extern "C" fn si3054_init(codec: *mut hda_codec) -> c_int {
    static int si3054_init(struct hda_codec *codec)
    {
    struct si3054_spec *spec = codec.spec;
    unsigned wait_count;
    u16 val;
    if (snd_hdac_regmap_add_vendor_verb(&codec.core,
    SI3054_VERB_WRITE_NODE))
    return -ENOMEM;
    snd_hda_codec_write(codec, AC_NODE_ROOT, 0, AC_VERB_SET_CODEC_RESET, 0);
    snd_hda_codec_write(codec, codec.core.mfg, 0, AC_VERB_SET_STREAM_FORMAT, 0);
    SET_REG(codec, SI3054_LINE_RATE, 9600);
    SET_REG(codec, SI3054_LINE_LEVEL, SI3054_DTAG_MASK|SI3054_ATAG_MASK);
    SET_REG(codec, SI3054_EXTENDED_MID, 0);
    wait_count = 10;
    do {
    msleep(2);
    val = GET_REG(codec, SI3054_EXTENDED_MID);
    } while ((val & SI3054_MEI_READY) != SI3054_MEI_READY && wait_count--);
    if((val&SI3054_MEI_READY) != SI3054_MEI_READY) {
    codec_err(codec, "si3054: cannot initialize. EXT MID = %04x\n", val);
// let's pray that this is no fatal error
// return -EACCES;
    }
    SET_REG(codec, SI3054_GPIO_POLARITY, 0xffff);
    SET_REG(codec, SI3054_GPIO_CFG, 0x0);
    SET_REG(codec, SI3054_MISC_AFE, 0);
    SET_REG(codec, SI3054_LINE_CFG1,0x200);
    if((GET_REG(codec,SI3054_LINE_STATUS) & (1<<6)) == 0) {
    codec_dbg(codec,
    "Link Frame Detect(FDT) is not ready (line status: %04x)\n",
    GET_REG(codec,SI3054_LINE_STATUS));
    }
    spec.international = GET_REG(codec, SI3054_CHIPID) & SI3054_CHIPID_INTERNATIONAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn si3054_remove(codec: *mut hda_codec) {
    static void si3054_remove(struct hda_codec *codec)
    {
    kfree(codec.spec);
    }
//
#[no_mangle]
unsafe extern "C" fn si3054_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    static int si3054_probe(struct hda_codec *codec, const struct hda_device_id *id)
    {
    codec.spec = kzalloc_obj(struct si3054_spec);
    if (!codec.spec)
    return -ENOMEM;
    return 0;
    }
    static const struct hda_codec_ops si3054_codec_ops = {
    .probe = si3054_probe,
    .remove = si3054_remove,
    .build_controls = si3054_build_controls,
    .build_pcms = si3054_build_pcms,
    .init = si3054_init,
    };
//
// driver entries
//
    static const struct hda_device_id snd_hda_id_si3054[] = {
    HDA_CODEC_ID(0x163c3055, "Si3054"),
    HDA_CODEC_ID(0x163c3155, "Si3054"),
    HDA_CODEC_ID(0x11c13026, "Si3054"),
    HDA_CODEC_ID(0x11c13055, "Si3054"),
    HDA_CODEC_ID(0x11c13155, "Si3054"),
    HDA_CODEC_ID(0x10573055, "Si3054"),
    HDA_CODEC_ID(0x10573057, "Si3054"),
    HDA_CODEC_ID(0x10573155, "Si3054"),
// VIA HDA on Clevo m540
    HDA_CODEC_ID(0x11063288, "Si3054"),
// Asus A8J Modem (SM56)
    HDA_CODEC_ID(0x15433155, "Si3054"),
// LG LW20 modem
    HDA_CODEC_ID(0x18540018, "Si3054"),
    {}
    };
    MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_si3054);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Si3054 HD-audio modem codec");
    static struct hda_codec_driver si3054_driver = {
    .id = snd_hda_id_si3054,
    .ops = &si3054_codec_ops,
    };
    module_hda_codec_driver(si3054_driver);
