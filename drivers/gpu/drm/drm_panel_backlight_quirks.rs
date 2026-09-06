//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_panel_backlight_quirks.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel_match {
    pub field: enum dmi_field,
    pub value: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_get_panel_backlight_quirk {
    pub dmi_match: drm_panel_match,
    pub dmi_match_other: drm_panel_match,
    pub ident: drm_edid_ident,
    pub quirk: drm_panel_backlight_quirk,
}

    static const struct drm_get_panel_backlight_quirk drm_panel_min_backlight_quirks[] = {
// Lenovo Legion 5 15ARH05, AUX backlight non-functional, force PWM
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "LENOVO",
    .dmi_match_other.field = DMI_PRODUCT_VERSION,
    .dmi_match_other.value = "Lenovo Legion 5 15ARH05",
    .ident.panel_id = drm_edid_encode_panel_id('B', 'O', 'E', 0x08df),
    .quirk = { .force_pwm = true, },
    },
// 13 inch matte panel
    {
    .dmi_match.field = DMI_BOARD_VENDOR,
    .dmi_match.value = "Framework",
    .ident.panel_id = drm_edid_encode_panel_id('B', 'O', 'E', 0x0bca),
    .ident.name = "NE135FBM-N41",
    .quirk = { .min_brightness = 1, },
    },
// 13 inch glossy panel
    {
    .dmi_match.field = DMI_BOARD_VENDOR,
    .dmi_match.value = "Framework",
    .ident.panel_id = drm_edid_encode_panel_id('B', 'O', 'E', 0x095f),
    .ident.name = "NE135FBM-N41",
    .quirk = { .min_brightness = 1, },
    },
// 13 inch 2.8k panel
    {
    .dmi_match.field = DMI_BOARD_VENDOR,
    .dmi_match.value = "Framework",
    .ident.panel_id = drm_edid_encode_panel_id('B', 'O', 'E', 0x0cb4),
    .ident.name = "NE135A1M-NY1",
    .quirk = { .min_brightness = 1, },
    },
// Steam Deck models
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "Valve",
    .dmi_match_other.field = DMI_PRODUCT_NAME,
    .dmi_match_other.value = "Jupiter",
    .quirk = { .min_brightness = 1, },
    },
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "Valve",
    .dmi_match_other.field = DMI_PRODUCT_NAME,
    .dmi_match_other.value = "Galileo",
    .quirk = { .min_brightness = 1, },
    },
// Have OLED Panels with brightness issue when last byte is 0/1
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "AYANEO",
    .dmi_match_other.field = DMI_PRODUCT_NAME,
    .dmi_match_other.value = "AYANEO 3",
    .quirk = { .brightness_mask = 3, },
    },
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "ZOTAC",
    .dmi_match_other.field = DMI_BOARD_NAME,
    .dmi_match_other.value = "G0A1W",
    .quirk = { .brightness_mask = 3, },
    },
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "ZOTAC",
    .dmi_match_other.field = DMI_BOARD_NAME,
    .dmi_match_other.value = "G1A1W",
    .quirk = { .brightness_mask = 3, },
    },
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "ONE-NETBOOK",
    .dmi_match_other.field = DMI_PRODUCT_NAME,
    .dmi_match_other.value = "ONEXPLAYER F1Pro",
    .quirk = { .brightness_mask = 3, },
    },
    {
    .dmi_match.field = DMI_SYS_VENDOR,
    .dmi_match.value = "ONE-NETBOOK",
    .dmi_match_other.field = DMI_PRODUCT_NAME,
    .dmi_match_other.value = "ONEXPLAYER F1 EVA-02",
    .quirk = { .brightness_mask = 3, },
    },
    };
    static bool drm_panel_min_backlight_quirk_matches(
    const struct drm_get_panel_backlight_quirk *quirk,
    const struct drm_edid *edid)
    {
    if (quirk.dmi_match.field &&
    !dmi_match(quirk.dmi_match.field, quirk.dmi_match.value))
    return false;
    if (quirk.dmi_match_other.field &&
    !dmi_match(quirk.dmi_match_other.field,
    quirk.dmi_match_other.value))
    return false;
    if (quirk.ident.panel_id && !drm_edid_match(edid, &quirk.ident))
    return false;
    return true;
    }
//
// drm_get_panel_backlight_quirk - Get backlight quirks for a panel
// @edid: EDID of the panel to check
//
// This function checks for platform specific (e.g. DMI based) quirks
// providing info on the minimum backlight brightness for systems where this
// cannot be probed correctly from the hard-/firm-ware and other sources.
//
// Returns:
// a drm_panel_backlight_quirk struct if a quirk was found, otherwise an
// error pointer.
//
    const struct drm_panel_backlight_quirk *
    drm_get_panel_backlight_quirk(const struct drm_edid *edid)
    {
    const struct drm_get_panel_backlight_quirk *quirk;
    size_t i;
    if (!IS_ENABLED(CONFIG_DMI))
    return ERR_PTR(-ENODATA);
    if (!edid)
    return ERR_PTR(-EINVAL);
    for (i = 0; i < ARRAY_SIZE(drm_panel_min_backlight_quirks); i++) {
    quirk = &drm_panel_min_backlight_quirks[i];
    if (drm_panel_min_backlight_quirk_matches(quirk, edid))
    return &quirk.quirk;
    }
    return ERR_PTR(-ENODATA);
    }
    EXPORT_SYMBOL(drm_get_panel_backlight_quirk);
    MODULE_DESCRIPTION("Quirks for panel backlight overrides");
    MODULE_LICENSE("GPL");
