//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/helpers/ideapad_hotkey_led.c
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
// Ideapad helper functions for Lenovo Ideapad LED control,
// It should be included from codec driver.
//

#[no_mangle]
unsafe extern "C" fn is_ideapad(codec: *mut hda_codec) -> bool {
    static bool is_ideapad(struct hda_codec *codec)
    {
    return (codec.core.subsystem_id >> 16 == 0x17aa) &&
    (acpi_dev_found("LHK2019") || acpi_dev_found("VPC2004"));
    }
    static void hda_fixup_ideapad_acpi(struct hda_codec *codec,
    const struct hda_fixup *fix, int action)
    {
    if (action == HDA_FIXUP_ACT_PRE_PROBE) {
    if (!is_ideapad(codec))
    return;
    snd_hda_gen_add_mute_led_cdev(codec, core::ptr::null_mut());
    snd_hda_gen_add_micmute_led_cdev(codec, core::ptr::null_mut());
    }
    }

    static void hda_fixup_ideapad_acpi(struct hda_codec *codec,
    const struct hda_fixup *fix, int action)
    {
    }
