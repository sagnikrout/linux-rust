//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/cirrus/cs421x.c
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
// Cirrus Logic CS421x HD-audio codec
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_spec {
    pub gen: hda_gen_spec,
    pub gpio_mask: c_uint,
    pub gpio_dir: c_uint,
    pub gpio_data: c_uint,
    pub /: *mut *mut unsigned int gpio_eapd_hp; / EAPD GPIO bit for headphones,
    pub /: *mut *mut unsigned int gpio_eapd_speaker; / EAPD GPIO bit for speakers,
// CS421x
    pub spdif_detect:1: c_uint,
    pub spdif_present:1: c_uint,
    pub sense_b:1: c_uint,
    pub vendor_nid: hda_nid_t,
// for MBP SPDIF control
    int (*spdif_sw_put)(struct snd_kcontrol *kcontrol,
    pub ucontrol): *mut snd_ctl_elem_value,
}

// CS421x boards
    enum {
    CS421X_CDB4210,
    CS421X_SENSE_B,
    CS421X_STUMPY,
    };
// Vendor-specific processing widget
pub const CS_DIG_OUT1_PIN_NID: c_uint = 0x10;
pub const CS_DIG_OUT2_PIN_NID: c_uint = 0x15;
pub const CS_DMIC1_PIN_NID: c_uint = 0x0e;
pub const CS_DMIC2_PIN_NID: c_uint = 0x12;
// coef indices
pub const IDX_SPDIF_STAT: c_uint = 0x0000;
pub const IDX_SPDIF_CTL: c_uint = 0x0001;
pub const IDX_ADC_CFG: c_uint = 0x0002;
// SZC bitmask, 4 modes below:
// 0 = immediate,
// 1 = digital immediate, analog zero-cross
// 2 = digtail & analog soft-ramp
// 3 = digital soft-ramp, analog zero-cross
//

// PGA mode: 0 = differential, 1 = signle-ended

pub const IDX_DAC_CFG: c_uint = 0x0003;
// SZC bitmask, 4 modes below:
// 0 = Immediate
// 1 = zero-cross
// 2 = soft-ramp
// 3 = soft-ramp on zero-cross
//

pub const IDX_BEEP_CFG: c_uint = 0x0004;
// 0x0008 - test reg key
// 0x0009 - 0x0014 -> 12 test regs
// 0x0015 - visibility reg
//
// Cirrus Logic CS4210
//
// 1 DAC => HP(sense) / Speakers,
// 1 ADC <= LineIn(sense) / MicIn / DMicIn,
// 1 SPDIF OUT => SPDIF Transmitter(sense)
//
pub const CS4210_DAC_NID: c_uint = 0x02;
pub const CS4210_ADC_NID: c_uint = 0x03;
pub const CS4210_VENDOR_NID: c_uint = 0x0B;
pub const CS421X_DMIC_PIN_NID: c_uint = 0x09 /* Port E */;
pub const CS421X_SPDIF_PIN_NID: c_uint = 0x0A /* Port H */;
pub const CS421X_IDX_DEV_CFG: c_uint = 0x01;
pub const CS421X_IDX_ADC_CFG: c_uint = 0x02;
pub const CS421X_IDX_DAC_CFG: c_uint = 0x03;
pub const CS421X_IDX_SPK_CTL: c_uint = 0x04;
// Cirrus Logic CS4213 is like CS4210 but does not have SPDIF input/output
pub const CS4213_VENDOR_NID: c_uint = 0x09;
#[no_mangle]
pub unsafe extern "C" fn cs_vendor_coef_get(codec: *mut hda_codec, idx: c_uint) -> c_int {
    static inline int cs_vendor_coef_get(struct hda_codec *codec, unsigned int idx)
    {
    struct cs_spec *spec = codec.spec;
    snd_hda_codec_write(codec, spec.vendor_nid, 0,
    AC_VERB_SET_COEF_INDEX, idx);
    return snd_hda_codec_read(codec, spec.vendor_nid, 0,
    AC_VERB_GET_PROC_COEF, 0);
    }
    static inline void cs_vendor_coef_set(struct hda_codec *codec, unsigned int idx,
    unsigned int coef)
    {
    struct cs_spec *spec = codec.spec;
    snd_hda_codec_write(codec, spec.vendor_nid, 0,
    AC_VERB_SET_COEF_INDEX, idx);
    snd_hda_codec_write(codec, spec.vendor_nid, 0,
    AC_VERB_SET_PROC_COEF, coef);
    }
//
// auto-mute and auto-mic switching
// CS421x auto-output redirecting
// HP/SPK/SPDIF
//
#[no_mangle]
unsafe extern "C" fn cs_automute(codec: *mut hda_codec) {
    static void cs_automute(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
// mute HPs if spdif jack (SENSE_B) is present
    spec.gen.master_mute = !!(spec.spdif_present && spec.sense_b);
    snd_hda_gen_update_outputs(codec);
    if (spec.gpio_eapd_hp || spec.gpio_eapd_speaker) {
    if (spec.gen.automute_speaker)
    spec.gpio_data = spec.gen.hp_jack_present ?
    spec.gpio_eapd_hp : spec.gpio_eapd_speaker;
    else
    spec.gpio_data =
    spec.gpio_eapd_hp | spec.gpio_eapd_speaker;
    snd_hda_codec_write(codec, 0x01, 0,
    AC_VERB_SET_GPIO_DATA, spec.gpio_data);
    }
    }
#[no_mangle]
unsafe extern "C" fn is_active_pin(codec: *mut hda_codec, nid: hda_nid_t) -> bool {
    static bool is_active_pin(struct hda_codec *codec, hda_nid_t nid)
    {
    unsigned int val;
    val = snd_hda_codec_get_pincfg(codec, nid);
    return (get_defcfg_connect(val) != AC_JACK_PORT_NONE);
    }
    static struct cs_spec *cs_alloc_spec(struct hda_codec *codec, int vendor_nid)
    {
    struct cs_spec *spec;
    spec = kzalloc_obj(*spec);
    if (!spec)
    return core::ptr::null_mut();
    codec.spec = spec;
    spec.vendor_nid = vendor_nid;
    codec.power_save_node = 1;
    snd_hda_gen_spec_init(&spec.gen);
    return spec;
    }
//
// Cirrus Logic CS4210
//
// 1 DAC => HP(sense) / Speakers,
// 1 ADC <= LineIn(sense) / MicIn / DMicIn,
// 1 SPDIF OUT => SPDIF Transmitter(sense)
//
// CS4210 board names
    static const struct hda_model_fixup cs421x_models[] = {
    { .id = CS421X_CDB4210, .name = "cdb4210" },
    { .id = CS421X_STUMPY, .name = "stumpy" },
    {}
    };
    static const struct hda_quirk cs421x_fixup_tbl[] = {
// Test Intel board + CDB2410
    SND_PCI_QUIRK(0x8086, 0x5001, "DP45SG/CDB4210", CS421X_CDB4210),
    {} /* terminator */
    };
// CS4210 board pinconfigs
// Default CS4210 (CDB4210)
    static const struct hda_pintbl cdb4210_pincfgs[] = {
    { 0x05, 0x0321401f },
    { 0x06, 0x90170010 },
    { 0x07, 0x03813031 },
    { 0x08, 0xb7a70037 },
    { 0x09, 0xb7a6003e },
    { 0x0a, 0x034510f0 },
    {} /* terminator */
    };
// Stumpy ChromeBox
    static const struct hda_pintbl stumpy_pincfgs[] = {
    { 0x05, 0x022120f0 },
    { 0x06, 0x901700f0 },
    { 0x07, 0x02a120f0 },
    { 0x08, 0x77a70037 },
    { 0x09, 0x77a6003e },
    { 0x0a, 0x434510f0 },
    {} /* terminator */
    };
// Setup GPIO/SENSE for each board (if used)
    static void cs421x_fixup_sense_b(struct hda_codec *codec,
    const struct hda_fixup *fix, int action)
    {
    struct cs_spec *spec = codec.spec;
    if (action == HDA_FIXUP_ACT_PRE_PROBE)
    spec.sense_b = 1;
    }
    static const struct hda_fixup cs421x_fixups[] = {
    [CS421X_CDB4210] = {
    .type = HDA_FIXUP_PINS,
    .v.pins = cdb4210_pincfgs,
    .chained = true,
    .chain_id = CS421X_SENSE_B,
    },
    [CS421X_SENSE_B] = {
    .type = HDA_FIXUP_FUNC,
    .v.func = cs421x_fixup_sense_b,
    },
    [CS421X_STUMPY] = {
    .type = HDA_FIXUP_PINS,
    .v.pins = stumpy_pincfgs,
    },
    };
    static const struct hda_verb cs421x_coef_init_verbs[] = {
    {0x0B, AC_VERB_SET_PROC_STATE, 1},
    {0x0B, AC_VERB_SET_COEF_INDEX, CS421X_IDX_DEV_CFG},
//
// Disable Coefficient Index Auto-Increment(DAI)=1,
// PDREF=0
//
    {0x0B, AC_VERB_SET_PROC_COEF, 0x0001 },
    {0x0B, AC_VERB_SET_COEF_INDEX, CS421X_IDX_ADC_CFG},
// ADC SZCMode = Digital Soft Ramp
    {0x0B, AC_VERB_SET_PROC_COEF, 0x0002 },
    {0x0B, AC_VERB_SET_COEF_INDEX, CS421X_IDX_DAC_CFG},
    {0x0B, AC_VERB_SET_PROC_COEF,
    (0x0002 /* DAC SZCMode = Digital Soft Ramp */
    | 0x0004 /* Mute DAC on FIFO error */
    | 0x0008 /* Enable DAC High Pass Filter */
    )},
    {} /* terminator */
    };
// Errata: CS4210 rev A1 Silicon
//
// http://www.cirrus.com/en/pubs/errata
//
// Description:
// 1. Performance degredation is present in the ADC.
// 2. Speaker output is not completely muted upon HP detect.
// 3. Noise is present when clipping occurs on the amplified
// speaker outputs.
//
// Workaround:
// The following verb sequence written to the registers during
// initialization will correct the issues listed above.
//
    static const struct hda_verb cs421x_coef_init_verbs_A1_silicon_fixes[] = {
    {0x0B, AC_VERB_SET_PROC_STATE, 0x01},  /* VPW: processing on */
    {0x0B, AC_VERB_SET_COEF_INDEX, 0x0006},
    {0x0B, AC_VERB_SET_PROC_COEF, 0x9999}, /* Test mode: on */
    {0x0B, AC_VERB_SET_COEF_INDEX, 0x000A},
    {0x0B, AC_VERB_SET_PROC_COEF, 0x14CB}, /* Chop double */
    {0x0B, AC_VERB_SET_COEF_INDEX, 0x0011},
    {0x0B, AC_VERB_SET_PROC_COEF, 0xA2D0}, /* Increase ADC current */
    {0x0B, AC_VERB_SET_COEF_INDEX, 0x001A},
    {0x0B, AC_VERB_SET_PROC_COEF, 0x02A9}, /* Mute speaker */
    {0x0B, AC_VERB_SET_COEF_INDEX, 0x001B},
    {0x0B, AC_VERB_SET_PROC_COEF, 0X1006}, /* Remove noise */
    {} /* terminator */
    };
// Speaker Amp Gain is controlled by the vendor widget's coef 4
    static const DECLARE_TLV_DB_SCALE(cs421x_speaker_boost_db_scale, 900, 300, 0);
    static int cs421x_boost_vol_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    uinfo.count = 1;
    uinfo.value.integer.min = 0;
    uinfo.value.integer.max = 3;
    return 0;
    }
    static int cs421x_boost_vol_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
    ucontrol.value.integer.value[0] =
    cs_vendor_coef_get(codec, CS421X_IDX_SPK_CTL) & 0x0003;
    return 0;
    }
    static int cs421x_boost_vol_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct hda_codec *codec = snd_kcontrol_chip(kcontrol);
    let mut vol: c_uint = ucontrol.value.integer.value[0];
    unsigned int coef =
    cs_vendor_coef_get(codec, CS421X_IDX_SPK_CTL);
    let mut original_coef: c_uint = coef;
    coef &= ~0x0003;
    coef |= (vol & 0x0003);
    if (original_coef != coef) {
    cs_vendor_coef_set(codec, CS421X_IDX_SPK_CTL, coef);
    return 1;
    }
    return 0;
    }
    static const struct snd_kcontrol_new cs421x_speaker_boost_ctl = {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .access = (SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ),
    .name = "Speaker Boost Playback Volume",
    .info = cs421x_boost_vol_info,
    .get = cs421x_boost_vol_get,
    .put = cs421x_boost_vol_put,
    .tlv = { .p = cs421x_speaker_boost_db_scale },
    };
#[no_mangle]
unsafe extern "C" fn cs4210_pinmux_init(codec: *mut hda_codec) {
    static void cs4210_pinmux_init(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
    unsigned int def_conf, coef;
// GPIO, DMIC_SCL, DMIC_SDA and SENSE_B are multiplexed
    coef = cs_vendor_coef_get(codec, CS421X_IDX_DEV_CFG);
    if (spec.gpio_mask)
    coef |= 0x0008; /* B1,B2 are GPIOs */
    else
    coef &= ~0x0008;
    if (spec.sense_b)
    coef |= 0x0010; /* B2 is SENSE_B, not inverted  */
    else
    coef &= ~0x0010;
    cs_vendor_coef_set(codec, CS421X_IDX_DEV_CFG, coef);
    if ((spec.gpio_mask || spec.sense_b) &&
    is_active_pin(codec, CS421X_DMIC_PIN_NID)) {
//
// GPIO or SENSE_B forced - disconnect the DMIC pin.
//
    def_conf = snd_hda_codec_get_pincfg(codec, CS421X_DMIC_PIN_NID);
    def_conf &= ~AC_DEFCFG_PORT_CONN;
    def_conf |= (AC_JACK_PORT_NONE << AC_DEFCFG_PORT_CONN_SHIFT);
    snd_hda_codec_set_pincfg(codec, CS421X_DMIC_PIN_NID, def_conf);
    }
    }
    static void cs4210_spdif_automute(struct hda_codec *codec,
    struct hda_jack_callback *tbl)
    {
    struct cs_spec *spec = codec.spec;
    let mut spdif_present: bool = false;
    let mut spdif_pin: hda_nid_t = spec.gen.autocfg.dig_out_pins[0];
// detect on spdif is specific to CS4210
    if (!spec.spdif_detect ||
    spec.vendor_nid != CS4210_VENDOR_NID)
    return;
    spdif_present = snd_hda_jack_detect(codec, spdif_pin);
    if (spdif_present == spec.spdif_present)
    return;
    spec.spdif_present = spdif_present;
// SPDIF TX on/off
    snd_hda_set_pin_ctl(codec, spdif_pin, spdif_present ? PIN_OUT : 0);
    cs_automute(codec);
    }
#[no_mangle]
unsafe extern "C" fn parse_cs421x_digital(codec: *mut hda_codec) {
    static void parse_cs421x_digital(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
    struct auto_pin_cfg *cfg = &spec.gen.autocfg;
    int i;
    for (i = 0; i < cfg.dig_outs; i++) {
    let mut nid: hda_nid_t = cfg.dig_out_pins[i];
    if (get_wcaps(codec, nid) & AC_WCAP_UNSOL_CAP) {
    spec.spdif_detect = 1;
    snd_hda_jack_detect_enable_callback(codec, nid,
    cs4210_spdif_automute);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cs421x_init(codec: *mut hda_codec) -> c_int {
    static int cs421x_init(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
    if (spec.vendor_nid == CS4210_VENDOR_NID) {
    snd_hda_sequence_write(codec, cs421x_coef_init_verbs);
    snd_hda_sequence_write(codec, cs421x_coef_init_verbs_A1_silicon_fixes);
    cs4210_pinmux_init(codec);
    }
    snd_hda_gen_init(codec);
    if (spec.gpio_mask)
    snd_hda_codec_set_gpio(codec, spec.gpio_mask, spec.gpio_dir,
    spec.gpio_data, 0);
    cs4210_spdif_automute(codec, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fix_volume_caps(codec: *mut hda_codec, dac: hda_nid_t) {
    static void fix_volume_caps(struct hda_codec *codec, hda_nid_t dac)
    {
    unsigned int caps;
// set the upper-limit for mixer amp to 0dB
    caps = query_amp_caps(codec, dac, HDA_OUTPUT);
    caps &= ~(0x7f << AC_AMPCAP_NUM_STEPS_SHIFT);
    caps |= ((caps >> AC_AMPCAP_OFFSET_SHIFT) & 0x7f)
    << AC_AMPCAP_NUM_STEPS_SHIFT;
    snd_hda_override_amp_caps(codec, dac, HDA_OUTPUT, caps);
    }
#[no_mangle]
unsafe extern "C" fn cs421x_parse_auto_config(codec: *mut hda_codec) -> c_int {
    static int cs421x_parse_auto_config(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
    let mut dac: hda_nid_t = CS4210_DAC_NID;
    int err;
    fix_volume_caps(codec, dac);
    err = snd_hda_parse_pin_defcfg(codec, &spec.gen.autocfg, core::ptr::null_mut(), 0);
    if (err < 0)
    return err;
    err = snd_hda_gen_parse_auto_config(codec, &spec.gen.autocfg);
    if (err < 0)
    return err;
    parse_cs421x_digital(codec);
    if (spec.gen.autocfg.speaker_outs &&
    spec.vendor_nid == CS4210_VENDOR_NID) {
    if (!snd_hda_gen_add_kctl(&spec.gen, core::ptr::null_mut(),
    &cs421x_speaker_boost_ctl))
    return -ENOMEM;
    }
    return 0;
    }
//
// Manage PDREF, when transitioning to D3hot
// (DAC,ADC) -> D3, PDREF=1, AFG->D3
//
#[no_mangle]
unsafe extern "C" fn cs421x_suspend(codec: *mut hda_codec) -> c_int {
    static int cs421x_suspend(struct hda_codec *codec)
    {
    struct cs_spec *spec = codec.spec;
    unsigned int coef;
    snd_hda_shutup_pins(codec);
    snd_hda_codec_write(codec, CS4210_DAC_NID, 0,
    AC_VERB_SET_POWER_STATE,  AC_PWRST_D3);
    snd_hda_codec_write(codec, CS4210_ADC_NID, 0,
    AC_VERB_SET_POWER_STATE,  AC_PWRST_D3);
    if (spec.vendor_nid == CS4210_VENDOR_NID) {
    coef = cs_vendor_coef_get(codec, CS421X_IDX_DEV_CFG);
    coef |= 0x0004; /* PDREF */
    cs_vendor_coef_set(codec, CS421X_IDX_DEV_CFG, coef);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs421x_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    static int cs421x_probe(struct hda_codec *codec, const struct hda_device_id *id)
    {
    struct cs_spec *spec;
    int err;
    spec = cs_alloc_spec(codec, id.driver_data);
    if (!spec)
    return -ENOMEM;
    spec.gen.automute_hook = cs_automute;
    if (spec.vendor_nid == CS4210_VENDOR_NID) {
    snd_hda_pick_fixup(codec, cs421x_models, cs421x_fixup_tbl,
    cs421x_fixups);
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);
//
// Update the GPIO/DMIC/SENSE_B pinmux before the configuration
// is auto-parsed. If GPIO or SENSE_B is forced, DMIC input
// is disabled.
//
    cs4210_pinmux_init(codec);
    }
    err = cs421x_parse_auto_config(codec);
    if (err < 0)
    goto error;
    snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);
    return 0;
    error:
    snd_hda_gen_remove(codec);
    return err;
    }
    static const struct hda_codec_ops cs421x_codec_ops = {
    .probe = cs421x_probe,
    .remove = snd_hda_gen_remove,
    .build_controls = snd_hda_gen_build_controls,
    .build_pcms = snd_hda_gen_build_pcms,
    .init = cs421x_init,
    .unsol_event = snd_hda_jack_unsol_event,
    .suspend = cs421x_suspend,
    .stream_pm = snd_hda_gen_stream_pm,
    };
//
// driver entries
//
    static const struct hda_device_id snd_hda_id_cs421x[] = {
    HDA_CODEC_ID_MODEL(0x10134210, "CS4210", CS4210_VENDOR_NID),
    HDA_CODEC_ID_MODEL(0x10134213, "CS4213", CS4213_VENDOR_NID),
    {} /* terminator */
    };
    MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_cs421x);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Cirrus Logic CS421x HD-audio codec");
    static struct hda_codec_driver cs421x_driver = {
    .id = snd_hda_id_cs421x,
    .ops = &cs421x_codec_ops,
    };
    module_hda_codec_driver(cs421x_driver);
