//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/wacom.h
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
// USB Wacom tablet support
//
// Copyright (c) 2000-2004 Vojtech Pavlik	<vojtech@ucw.cz>
// Copyright (c) 2000 Andreas Bach Aaen	<abach@stofanet.dk>
// Copyright (c) 2000 Clifford Wolf		<clifford@clifford.at>
// Copyright (c) 2000 Sam Mosel		<sam.mosel@computer.org>
// Copyright (c) 2000 James E. Blair		<corvus@gnu.org>
// Copyright (c) 2000 Daniel Egger		<egger@suse.de>
// Copyright (c) 2001 Frederic Lepied		<flepied@mandrakesoft.com>
// Copyright (c) 2004 Panagiotis Issaris	<panagiotis.issaris@mech.kuleuven.ac.be>
// Copyright (c) 2002-2011 Ping Cheng		<pingc@wacom.com>
// Copyright (c) 2014 Benjamin Tissoires	<benjamin.tissoires@redhat.com>
//
// ChangeLog:
// v0.1 (vp)  - Initial release
// v0.2 (aba) - Support for all buttons / combinations
// v0.3 (vp)  - Support for Intuos added
// v0.4 (sm)  - Support for more Intuos models, menustrip
// relative mode, proximity.
// v0.5 (vp)  - Big cleanup, nifty features removed,
// they belong in userspace
// v1.8 (vp)  - Submit URB only when operating, moved to CVS,
// use input_report_key instead of report_btn and
// other cleanups
// v1.11 (vp) - Add URB ->dev setting for new kernels
// v1.11 (jb) - Add support for the 4D Mouse & Lens
// v1.12 (de) - Add support for two more inking pen IDs
// v1.14 (vp) - Use new USB device id probing scheme.
// Fix Wacom Graphire mouse wheel
// v1.18 (vp) - Fix mouse wheel direction
// Make mouse relative
// v1.20 (fl) - Report tool id for Intuos devices
// - Multi tools support
// - Corrected Intuos protocol decoding (airbrush, 4D mouse, lens cursor...)
// - Add PL models support
// - Fix Wacom Graphire mouse wheel again
// v1.21 (vp) - Removed protocol descriptions
// - Added MISC_SERIAL for tool serial numbers
// (gb) - Identify version on module load.
// v1.21.1 (fl) - added Graphire2 support
// v1.21.2 (fl) - added Intuos2 support
// - added all the PL ids
// v1.21.3 (fl) - added another eraser id from Neil Okamoto
// - added smooth filter for Graphire from Peri Hankey
// - added PenPartner support from Olaf van Es
// - new tool ids from Ole Martin Bjoerndalen
// v1.29 (pc) - Add support for more tablets
// - Fix pressure reporting
// v1.30 (vp) - Merge 2.4 and 2.5 drivers
// - Since 2.5 now has input_sync(), remove MSC_SERIAL abuse
// - Cleanups here and there
// v1.30.1 (pi) - Added Graphire3 support
// v1.40 (pc) - Add support for several new devices, fix eraser reporting, ...
// v1.43 (pc) - Added support for Cintiq 21UX
// - Fixed a Graphire bug
// - Merged wacom_intuos3_irq into wacom_intuos_irq
// v1.44 (pc) - Added support for Graphire4, Cintiq 710, Intuos3 6x11, etc.
// - Report Device IDs
// v1.45 (pc) - Added support for DTF 521, Intuos3 12x12 and 12x19
// - Minor data report fix
// v1.46 (pc) - Split wacom.c into wacom_sys.c and wacom_wac.c,
// - where wacom_sys.c deals with system specific code,
// - and wacom_wac.c deals with Wacom specific code
// - Support Intuos3 4x6
// v1.47 (pc) - Added support for Bamboo
// v1.48 (pc) - Added support for Bamboo1, BambooFun, and Cintiq 12WX
// v1.49 (pc) - Added support for USB Tablet PC (0x90, 0x93, and 0x9A)
// v1.50 (pc) - Fixed a TabletPC touch bug in 2.6.28
// v1.51 (pc) - Added support for Intuos4
// v1.52 (pc) - Query Wacom data upon system resume
// - add defines for features->type
// - add new devices (0x9F, 0xE2, and 0XE3)
// v2.00 (bt) - conversion to a HID driver
// - integration of the Bluetooth devices
//

//
// Version Information
//

pub const USB_VENDOR_ID_WACOM: c_uint = 0x056a;
pub const USB_VENDOR_ID_LENOVO: c_uint = 0x17ef;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wacom_worker {
    WACOM_WORKER_WIRELESS,
    WACOM_WORKER_BATTERY,
    WACOM_WORKER_REMOTE,
    WACOM_WORKER_MODE_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_led {
    pub cdev: led_classdev,
    pub trigger: led_trigger,
    pub wacom: *mut wacom,
    pub group: c_uint,
    pub id: c_uint,
    pub llv: u8,
    pub hlv: u8,
    pub held: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_group_leds {
    pub /: *mut *mut u8 select; / status led selector (0..3),
    pub leds: *mut wacom_led,
    pub count: c_uint,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_battery {
    pub wacom: *mut wacom,
    pub bat_desc: power_supply_desc,
    pub battery: *mut power_supply,
    pub bat_name: [c_char; WACOM_NAME_MAX],
    pub bat_status: c_int,
    pub battery_capacity: c_int,
    pub bat_charging: c_int,
    pub bat_connected: c_int,
    pub ps_connected: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_remote {
    pub remote_lock: spinlock_t,
    pub remote_fifo: kfifo,
    pub remote_dir: *mut kobject,
    pub group: attribute_group,
    pub serial: u32,
    pub input: *mut input_dev,
    pub registered: bool,
    pub battery: wacom_battery,
    pub active_time: ktime_t,
    pub remotes: [}; WACOM_MAX_REMOTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom {
    pub usbdev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub wacom_wac: wacom_wac,
    pub hdev: *mut hid_device,
    pub lock: mutex,
    pub wireless_work: work_struct,
    pub battery_work: work_struct,
    pub remote_work: work_struct,
    pub init_work: delayed_work,
    pub aes_battery_work: delayed_work,
    pub remote: *mut wacom_remote,
    pub mode_change_work: work_struct,
    pub idleprox_timer: timer_list,
    pub generic_has_leds: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_leds {
    pub groups: *mut wacom_group_leds,
    pub count: c_uint,
    pub /: *mut *mut u8 llv; / status led brightness no button (1..127),
    pub /: *mut *mut u8 hlv; / status led brightness button pressed (1..127),
    pub /: *mut *mut u8 img_lum; / OLED matrix display brightness,
    pub /: *mut *mut u8 max_llv; / maximum brightness of LED (llv),
    pub /: *mut *mut u8 max_hlv; / maximum brightness of LED (hlv),
    pub led: },
    pub battery: wacom_battery,
    pub resources: bool,
}

//
// Convert a signed 32-bit integer to an unsigned n-bit integer. Undoes
// the normally-helpful work of 'hid_snto32' for fields that use signed
// ranges for questionable reasons.
//
extern "C" {
    pub fn DIV_ROUND_CLOSEST(out_max: *mut *mut value, _arg: in_max) -> return;
}
extern "C" {
    pub fn wacom_wac_irq(wacom_wac: *mut wacom_wac, len: usize);
}
extern "C" {
    pub fn wacom_setup_device_quirks(wacom: *mut wacom);
}
extern "C" {
    pub fn wacom_wac_report(hdev: *mut hid_device, report: *mut hid_report);
}
extern "C" {
    pub fn wacom_battery_work(work: *mut work_struct);
}
extern "C" {
    pub fn wacom_leds_brightness_get(led: *mut wacom_led) -> led_brightness;
}
extern "C" {
    pub fn wacom_equivalent_usage(usage: c_int) -> c_int;
}
extern "C" {
    pub fn wacom_initialize_leds(wacom: *mut wacom) -> c_int;
}
extern "C" {
    pub fn wacom_idleprox_timeout(list: *mut timer_list);
}
