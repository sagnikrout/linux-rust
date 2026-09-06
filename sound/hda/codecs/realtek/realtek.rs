//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/realtek/realtek.h
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
// Realtek HD-audio codec support code
//

// extra amp-initialization sequence types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alc_customize_define {
    pub sku_cfg: c_uint,
    pub port_connectivity: c_uchar,
    pub check_sum: c_uchar,
    pub customization: c_uchar,
    pub external_amp: c_uchar,
    pub enable_pcbeep:1: c_uint,
    pub platform_type:1: c_uint,
    pub swap:1: c_uint,
    pub override:1: c_uint,
    pub /: *mut *mut unsigned int fixup:1; / Means that this sku is set by driver, not read from hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alc_coef_led {
    pub idx: c_uint,
    pub mask: c_uint,
    pub on: c_uint,
    pub off: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alc_spec {
    pub /: *mut *mut hda_gen_spec gen; / must be at head,
// codec parameterization
    pub cdefine: alc_customize_define,
    pub /: *mut *mut unsigned int parse_flags; / flag for snd_hda_parse_pin_defcfg(),
// GPIO bits
    pub gpio_mask: c_uint,
    pub gpio_dir: c_uint,
    pub gpio_data: c_uint,
    pub /: *mut *mut bool gpio_write_delay; / add a delay before writing gpio_data,
// mute LED for HP laptops, see vref_mute_led_set()
    pub mute_led_polarity: c_int,
    pub micmute_led_polarity: c_int,
    pub mute_led_nid: hda_nid_t,
    pub cap_mute_led_nid: hda_nid_t,
    pub gpio_mute_led_mask: c_uint,
    pub gpio_mic_led_mask: c_uint,
    pub mute_led_coef: alc_coef_led,
    pub mic_led_coef: alc_coef_led,
    pub coef_mutex: mutex,
    pub headset_mic_pin: hda_nid_t,
    pub headphone_mic_pin: hda_nid_t,
    pub current_headset_mode: c_int,
    pub current_headset_type: c_int,
// hooks
    pub codec): *mut *mut void (init_hook)(struct hda_codec,
    pub codec): *mut *mut void (power_hook)(struct hda_codec,
    pub codec): *mut *mut void (shutup)(struct hda_codec,
    pub init_amp: c_int,
    pub /: *mut *mut int codec_variant; / flag for other variants,
    pub has_alc5505_dsp:1: c_uint,
    pub no_depop_delay:1: c_uint,
    pub done_hp_init:1: c_uint,
    pub no_shutup_pins:1: c_uint,
    pub ultra_low_power:1: c_uint,
    pub has_hs_key:1: c_uint,
    pub no_internal_mic_pin:1: c_uint,
    pub en_3kpull_low:1: c_uint,
    pub num_speaker_amps: c_int,
// for PLL fix
    pub pll_nid: hda_nid_t,
    pub pll_coef_bit: unsigned int pll_coef_idx,,
    pub coef0: c_uint,
    pub kb_dev: *mut input_dev,
    pub alc_mute_keycode_map: [u8; 1],
// component binding
    pub comps: hda_component_parent,
}

extern "C" {
    pub fn alc_get_coef0(codec: *mut hda_codec) -> c_uint;
}
// coef writes/updates batch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coef_fw {
    pub nid: c_uchar,
    pub idx: c_uchar,
    pub mask: c_ushort,
    pub val: c_ushort,
}

extern "C" {
    pub fn alc_process_coef_fw(codec: *mut hda_codec, fw: *const coef_fw);
}
//
// GPIO helpers
//
extern "C" {
    pub fn alc_setup_gpio(codec: *mut hda_codec, mask: c_uint);
}
// common GPIO fixups
extern "C" {
    pub fn alc_fixup_gpio(codec: *mut hda_codec, action: c_int, mask: c_uint);
}
//
// Common init code, callbacks and helpers
//
extern "C" {
    pub fn alc_fix_pll(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_fill_eapd_coef(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_auto_setup_eapd(codec: *mut hda_codec, on: bool);
}
extern "C" {
    pub fn alc_find_ext_mic_pin(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn alc_headset_mic_no_shutup(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_shutup_pins(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_eapd_shutup(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_auto_init_amp(codec: *mut hda_codec, type: c_int);
}
extern "C" {
    pub fn alc_get_hp_pin(spec: *mut alc_spec) -> hda_nid_t;
}
extern "C" {
    pub fn alc_auto_parse_customize_define(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn alc_subsystem_id(codec: *mut hda_codec, ports: *const hda_nid_t) -> c_int;
}
extern "C" {
    pub fn alc_ssid_check(codec: *mut hda_codec, ports: *const hda_nid_t);
}
extern "C" {
    pub fn alc_build_controls(codec: *mut hda_codec) -> c_int;
}

extern "C" {
    pub fn alc_init(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn alc_shutup(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_power_eapd(codec: *mut hda_codec);
}
extern "C" {
    pub fn alc_suspend(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn alc_resume(codec: *mut hda_codec) -> c_int;
}
extern "C" {
    pub fn alc_alloc_spec(codec: *mut hda_codec, mixer_nid: hda_nid_t) -> c_int;
}

extern "C" {
    pub fn alc_set_beep_amp(spec: *mut alc_spec, nid: hda_nid_t, idx: c_int, dir: c_int) -> c_int;
}
extern "C" {
    pub fn alc_has_cdefine_beep(codec: *mut hda_codec) -> c_int;
}

pub const has_cdefine_beep(codec): c_int = 0;

// Common fixups
// device-specific, but used by multiple codec drivers
//
// COEF access helper functions
//
