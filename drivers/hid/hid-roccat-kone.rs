//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-kone.h
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
// Copyright (c) 2010 Stefan Achatz <erazor_de@users.sourceforge.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_keystroke {
    pub key: u8,
    pub action: u8,
    pub /: *mut *mut uint16_t period; / in milliseconds,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_keystroke_buttons {
    kone_keystroke_button_1 = 0xf0, /* left mouse button */
    kone_keystroke_button_2 = 0xf1, /* right mouse button */
    kone_keystroke_button_3 = 0xf2, /* wheel */
    kone_keystroke_button_9 = 0xf3, /* side button up */
    kone_keystroke_button_8 = 0xf4 /* side button down */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_keystroke_actions {
    kone_keystroke_action_press = 0,
    kone_keystroke_action_release = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_button_info {
    pub /: *mut *mut uint8_t number; / range 1-8,
    pub type: u8,
    pub /: *mut *mut uint8_t macro_type; / 0 = short, 1 = overlong,
    pub /: *mut *mut uint8_t macro_set_name[16]; / can be max 15 chars long,
    pub /: *mut *mut uint8_t macro_name[16]; / can be max 15 chars long,
    pub count: u8,
    pub keystrokes: [kone_keystroke; 20],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_button_info_types {
// valid button types until firmware 1.32
    kone_button_info_type_button_1 = 0x1, /* click (left mouse button) */
    kone_button_info_type_button_2 = 0x2, /* menu (right mouse button)*/
    kone_button_info_type_button_3 = 0x3, /* scroll (wheel) */
    kone_button_info_type_double_click = 0x4,
    kone_button_info_type_key = 0x5,
    kone_button_info_type_macro = 0x6,
    kone_button_info_type_off = 0x7,
// TODO clarify function and rename
    kone_button_info_type_osd_xy_prescaling = 0x8,
    kone_button_info_type_osd_dpi = 0x9,
    kone_button_info_type_osd_profile = 0xa,
    kone_button_info_type_button_9 = 0xb, /* ie forward */
    kone_button_info_type_button_8 = 0xc, /* ie backward */
    kone_button_info_type_dpi_up = 0xd, /* internal */
    kone_button_info_type_dpi_down = 0xe, /* internal */
    kone_button_info_type_button_7 = 0xf, /* tilt left */
    kone_button_info_type_button_6 = 0x10, /* tilt right */
    kone_button_info_type_profile_up = 0x11, /* internal */
    kone_button_info_type_profile_down = 0x12, /* internal */
// additional valid button types since firmware 1.38
    kone_button_info_type_multimedia_open_player = 0x20,
    kone_button_info_type_multimedia_next_track = 0x21,
    kone_button_info_type_multimedia_prev_track = 0x22,
    kone_button_info_type_multimedia_play_pause = 0x23,
    kone_button_info_type_multimedia_stop = 0x24,
    kone_button_info_type_multimedia_mute = 0x25,
    kone_button_info_type_multimedia_volume_up = 0x26,
    kone_button_info_type_multimedia_volume_down = 0x27
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_button_info_numbers {
    kone_button_top = 1,
    kone_button_wheel_tilt_left = 2,
    kone_button_wheel_tilt_right = 3,
    kone_button_forward = 4,
    kone_button_backward = 5,
    kone_button_middle = 6,
    kone_button_plus = 7,
    kone_button_minus = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_light_info {
    pub /: *mut *mut uint8_t number; / number of light 1-5,
    pub /: *mut *mut uint8_t mod; / 1 = on, 2 = off,
    pub /: *mut *mut uint8_t red; / range 0x00-0xff,
    pub /: *mut *mut uint8_t green; / range 0x00-0xff,
    pub /: *mut *mut uint8_t blue; / range 0x00-0xff,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_profile {
    pub /: *mut *mut uint16_t size; / always 975,
    pub /: *mut *mut uint16_t unused; / always 0,
//
// range 1-5
// This number does not need to correspond with location where profile
// saved
//
    pub /: *mut *mut uint8_t profile; / range 1-5,
    pub /: *mut *mut uint16_t main_sensitivity; / range 100-1000,
    pub /: *mut *mut uint8_t xy_sensitivity_enabled; / 1 = on, 2 = off,
    pub /: *mut *mut uint16_t x_sensitivity; / range 100-1000,
    pub /: *mut *mut uint16_t y_sensitivity; / range 100-1000,
    pub /: *mut *mut uint8_t dpi_rate; / bit 1 = 800, ...,
    pub /: *mut *mut uint8_t startup_dpi; / range 1-6,
    pub /: *mut *mut uint8_t polling_rate; / 1 = 125Hz, 2 = 500Hz, 3 = 1000Hz,
// kone has no dcu
// value is always 2 in firmwares <= 1.32 and
// 1 in firmwares > 1.32
//
    pub dcu_flag: u8,
    pub /: *mut *mut uint8_t light_effect_1; / range 1-3,
    pub /: *mut *mut uint8_t light_effect_2; / range 1-5,
    pub /: *mut *mut uint8_t light_effect_3; / range 1-4,
    pub /: *mut *mut uint8_t light_effect_speed; / range 0-255,
    pub light_infos: [kone_light_info; 5],
// offset is kone_button_info_numbers - 1
    pub button_infos: [kone_button_info; 8],
    pub /: *mut *mut uint16_t checksum; / \brief holds checksum of struct,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_polling_rates {
    kone_polling_rate_125 = 1,
    kone_polling_rate_500 = 2,
    kone_polling_rate_1000 = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_settings {
    pub /: *mut *mut uint16_t size; / always 36,
    pub /: *mut *mut uint8_t startup_profile; / 1-5,
    pub unknown1: u8,
    pub /: *mut *mut uint8_t tcu; / 0 = off, 1 = on,
    pub unknown2: [u8; 23],
    pub calibration_data: [u8; 4],
    pub unknown3: [u8; 2],
    pub checksum: u16,
// C attribute field omitted
//
// 12 byte mouse event read by interrupt_read
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_mouse_event {
    pub /: *mut *mut uint8_t report_number; / always 1,
    pub button: u8,
    pub x: u16,
    pub y: u16,
    pub /: *mut *mut uint8_t wheel; / up = 1, down = -1,
    pub /: *mut *mut uint8_t tilt; / right = 1, left = -1,
    pub unknown: u8,
    pub event: u8,
    pub /: *mut *mut uint8_t value; / press = 0, release = 1,
    pub /: *mut *mut uint8_t macro_key; / 0 to 8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_mouse_events {
// osd events are thought to be display on screen
    kone_mouse_event_osd_dpi = 0xa0,
    kone_mouse_event_osd_profile = 0xb0,
// TODO clarify meaning and occurence of kone_mouse_event_calibration
    kone_mouse_event_calibration = 0xc0,
    kone_mouse_event_call_overlong_macro = 0xe0,
    kone_mouse_event_multimedia = 0xe1,
// switch events notify if user changed values with mousebutton click
    kone_mouse_event_switch_dpi = 0xf0,
    kone_mouse_event_switch_profile = 0xf1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kone_commands {
    kone_command_profile = 0x5a,
    kone_command_settings = 0x15a,
    kone_command_firmware_version = 0x25a,
    kone_command_weight = 0x45a,
    kone_command_calibrate = 0x55a,
    kone_command_confirm_write = 0x65a,
    kone_command_firmware = 0xe5a
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_roccat_report {
    pub event: u8,
    pub /: *mut *mut uint8_t value; / holds dpi or profile value,
    pub /: *mut *mut uint8_t key; / macro key on overlong macro execution,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kone_device {
//
// Storing actual values when we get informed about changes since there
// is no way of getting this information from the device on demand
//
    pub actual_dpi: int actual_profile,,
// Used for neutralizing abnormal button behaviour
    pub last_mouse_event: kone_mouse_event,
//
// It's unlikely that multiple sysfs attributes are accessed at a time,
// so only one mutex is used to secure hardware access and profiles and
// settings of this struct.
//
    pub kone_lock: mutex,
//
// Storing the data here reduces IO and ensures that data is available
// when its needed (E.g. interrupt handler).
//
    pub profiles: [kone_profile; 5],
    pub settings: kone_settings,
//
// firmware doesn't change unless firmware update is implemented,
// so it's read only once
//
    pub firmware_version: c_int,
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
}
