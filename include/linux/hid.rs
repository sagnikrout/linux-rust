//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid.h
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
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2001 Vojtech Pavlik
// Copyright (c) 2006-2007 Jiri Kosina
//
// Should you need to contact me, the author, you can do so either by
// e-mail - mail your message to <vojtech@ucw.cz>, or by paper mail:
// Vojtech Pavlik, Simunkova 1594, Prague 8, 182 00 Czech Republic
//

//
// We parse each description item into this structure. Short items data
// values are expanded to 32-bit signed int, long items contain a pointer
// into the data area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_item {
    pub format: unsigned,
    pub size: __u8,
    pub type: __u8,
    pub tag: __u8,
    pub u8: __u8,
    pub s8: __s8,
    pub u16: __u16,
    pub s16: __s16,
    pub u32: __u32,
    pub s32: __s32,
    pub longdata: *const __u8,
    pub data: },
}

//
// HID report item format
//
pub const HID_ITEM_FORMAT_SHORT: c_int = 0;
pub const HID_ITEM_FORMAT_LONG: c_int = 1;
//
// Special tag indicating long items
//
pub const HID_ITEM_TAG_LONG: c_int = 15;
//
// HID report descriptor item type (prefix bit 2,3)
//
pub const HID_ITEM_TYPE_MAIN: c_int = 0;
pub const HID_ITEM_TYPE_GLOBAL: c_int = 1;
pub const HID_ITEM_TYPE_LOCAL: c_int = 2;
pub const HID_ITEM_TYPE_RESERVED: c_int = 3;
//
// HID report descriptor main item tags
//
pub const HID_MAIN_ITEM_TAG_INPUT: c_int = 8;
pub const HID_MAIN_ITEM_TAG_OUTPUT: c_int = 9;
pub const HID_MAIN_ITEM_TAG_FEATURE: c_int = 11;
pub const HID_MAIN_ITEM_TAG_BEGIN_COLLECTION: c_int = 10;
pub const HID_MAIN_ITEM_TAG_END_COLLECTION: c_int = 12;
pub const HID_MAIN_ITEM_TAG_RESERVED_MIN: c_int = 13;
pub const HID_MAIN_ITEM_TAG_RESERVED_MAX: c_int = 15;
//
// HID report descriptor main item contents
//
pub const HID_MAIN_ITEM_CONSTANT: c_uint = 0x001;
pub const HID_MAIN_ITEM_VARIABLE: c_uint = 0x002;
pub const HID_MAIN_ITEM_RELATIVE: c_uint = 0x004;
pub const HID_MAIN_ITEM_WRAP: c_uint = 0x008;
pub const HID_MAIN_ITEM_NONLINEAR: c_uint = 0x010;
pub const HID_MAIN_ITEM_NO_PREFERRED: c_uint = 0x020;
pub const HID_MAIN_ITEM_NULL_STATE: c_uint = 0x040;
pub const HID_MAIN_ITEM_VOLATILE: c_uint = 0x080;
pub const HID_MAIN_ITEM_BUFFERED_BYTE: c_uint = 0x100;
//
// HID report descriptor collection item types
//
pub const HID_COLLECTION_PHYSICAL: c_int = 0;
pub const HID_COLLECTION_APPLICATION: c_int = 1;
pub const HID_COLLECTION_LOGICAL: c_int = 2;
pub const HID_COLLECTION_NAMED_ARRAY: c_int = 4;
//
// HID report descriptor global item tags
//
pub const HID_GLOBAL_ITEM_TAG_USAGE_PAGE: c_int = 0;
pub const HID_GLOBAL_ITEM_TAG_LOGICAL_MINIMUM: c_int = 1;
pub const HID_GLOBAL_ITEM_TAG_LOGICAL_MAXIMUM: c_int = 2;
pub const HID_GLOBAL_ITEM_TAG_PHYSICAL_MINIMUM: c_int = 3;
pub const HID_GLOBAL_ITEM_TAG_PHYSICAL_MAXIMUM: c_int = 4;
pub const HID_GLOBAL_ITEM_TAG_UNIT_EXPONENT: c_int = 5;
pub const HID_GLOBAL_ITEM_TAG_UNIT: c_int = 6;
pub const HID_GLOBAL_ITEM_TAG_REPORT_SIZE: c_int = 7;
pub const HID_GLOBAL_ITEM_TAG_REPORT_ID: c_int = 8;
pub const HID_GLOBAL_ITEM_TAG_REPORT_COUNT: c_int = 9;
pub const HID_GLOBAL_ITEM_TAG_PUSH: c_int = 10;
pub const HID_GLOBAL_ITEM_TAG_POP: c_int = 11;
//
// HID report descriptor local item tags
//
pub const HID_LOCAL_ITEM_TAG_USAGE: c_int = 0;
pub const HID_LOCAL_ITEM_TAG_USAGE_MINIMUM: c_int = 1;
pub const HID_LOCAL_ITEM_TAG_USAGE_MAXIMUM: c_int = 2;
pub const HID_LOCAL_ITEM_TAG_DESIGNATOR_INDEX: c_int = 3;
pub const HID_LOCAL_ITEM_TAG_DESIGNATOR_MINIMUM: c_int = 4;
pub const HID_LOCAL_ITEM_TAG_DESIGNATOR_MAXIMUM: c_int = 5;
pub const HID_LOCAL_ITEM_TAG_STRING_INDEX: c_int = 7;
pub const HID_LOCAL_ITEM_TAG_STRING_MINIMUM: c_int = 8;
pub const HID_LOCAL_ITEM_TAG_STRING_MAXIMUM: c_int = 9;
pub const HID_LOCAL_ITEM_TAG_DELIMITER: c_int = 10;
//
// HID usage tables
//
pub const HID_USAGE_PAGE: c_uint = 0xffff0000;
pub const HID_UP_UNDEFINED: c_uint = 0x00000000;
pub const HID_UP_GENDESK: c_uint = 0x00010000;
pub const HID_UP_SIMULATION: c_uint = 0x00020000;
pub const HID_UP_GENDEVCTRLS: c_uint = 0x00060000;
pub const HID_UP_KEYBOARD: c_uint = 0x00070000;
pub const HID_UP_LED: c_uint = 0x00080000;
pub const HID_UP_BUTTON: c_uint = 0x00090000;
pub const HID_UP_ORDINAL: c_uint = 0x000a0000;
pub const HID_UP_TELEPHONY: c_uint = 0x000b0000;
pub const HID_UP_CONSUMER: c_uint = 0x000c0000;
pub const HID_UP_DIGITIZER: c_uint = 0x000d0000;
pub const HID_UP_HAPTIC: c_uint = 0x000e0000;
pub const HID_UP_PID: c_uint = 0x000f0000;
pub const HID_UP_BATTERY: c_uint = 0x00850000;
pub const HID_UP_CAMERA: c_uint = 0x00900000;
pub const HID_UP_HPVENDOR: c_uint = 0xff7f0000;
pub const HID_UP_HPVENDOR2: c_uint = 0xff010000;
pub const HID_UP_MSVENDOR: c_uint = 0xff000000;
pub const HID_UP_CUSTOM: c_uint = 0x00ff0000;
pub const HID_UP_LOGIVENDOR: c_uint = 0xffbc0000;
pub const HID_UP_LOGIVENDOR2: c_uint = 0xff090000;
pub const HID_UP_LOGIVENDOR3: c_uint = 0xff430000;
pub const HID_UP_LNVENDOR: c_uint = 0xffa00000;
pub const HID_UP_SENSOR: c_uint = 0x00200000;
pub const HID_UP_ASUSVENDOR: c_uint = 0xff310000;
pub const HID_UP_GOOGLEVENDOR: c_uint = 0xffd10000;
pub const HID_USAGE: c_uint = 0x0000ffff;
pub const HID_GD_POINTER: c_uint = 0x00010001;
pub const HID_GD_MOUSE: c_uint = 0x00010002;
pub const HID_GD_JOYSTICK: c_uint = 0x00010004;
pub const HID_GD_GAMEPAD: c_uint = 0x00010005;
pub const HID_GD_KEYBOARD: c_uint = 0x00010006;
pub const HID_GD_KEYPAD: c_uint = 0x00010007;
pub const HID_GD_MULTIAXIS: c_uint = 0x00010008;
//
// Microsoft Win8 Wireless Radio Controls extensions CA, see:
// http://www.usb.org/developers/hidpage/HUTRR40RadioHIDUsagesFinal.pdf
//
pub const HID_GD_WIRELESS_RADIO_CTLS: c_uint = 0x0001000c;
//
// System Multi-Axis, see:
// http://www.usb.org/developers/hidpage/HUTRR62_-_Generic_Desktop_CA_for_System_Multi-Axis_Controllers.txt
//
pub const HID_GD_SYSTEM_MULTIAXIS: c_uint = 0x0001000e;
pub const HID_GD_X: c_uint = 0x00010030;
pub const HID_GD_Y: c_uint = 0x00010031;
pub const HID_GD_Z: c_uint = 0x00010032;
pub const HID_GD_RX: c_uint = 0x00010033;
pub const HID_GD_RY: c_uint = 0x00010034;
pub const HID_GD_RZ: c_uint = 0x00010035;
pub const HID_GD_SLIDER: c_uint = 0x00010036;
pub const HID_GD_DIAL: c_uint = 0x00010037;
pub const HID_GD_WHEEL: c_uint = 0x00010038;
pub const HID_GD_HATSWITCH: c_uint = 0x00010039;
pub const HID_GD_BUFFER: c_uint = 0x0001003a;
pub const HID_GD_BYTECOUNT: c_uint = 0x0001003b;
pub const HID_GD_MOTION: c_uint = 0x0001003c;
pub const HID_GD_START: c_uint = 0x0001003d;
pub const HID_GD_SELECT: c_uint = 0x0001003e;
pub const HID_GD_VX: c_uint = 0x00010040;
pub const HID_GD_VY: c_uint = 0x00010041;
pub const HID_GD_VZ: c_uint = 0x00010042;
pub const HID_GD_VBRX: c_uint = 0x00010043;
pub const HID_GD_VBRY: c_uint = 0x00010044;
pub const HID_GD_VBRZ: c_uint = 0x00010045;
pub const HID_GD_VNO: c_uint = 0x00010046;
pub const HID_GD_FEATURE: c_uint = 0x00010047;
pub const HID_GD_RESOLUTION_MULTIPLIER: c_uint = 0x00010048;
pub const HID_GD_SYSTEM_CONTROL: c_uint = 0x00010080;
pub const HID_GD_UP: c_uint = 0x00010090;
pub const HID_GD_DOWN: c_uint = 0x00010091;
pub const HID_GD_RIGHT: c_uint = 0x00010092;
pub const HID_GD_LEFT: c_uint = 0x00010093;
pub const HID_GD_DO_NOT_DISTURB: c_uint = 0x0001009b;
// Microsoft Win8 Wireless Radio Controls CA usage codes
pub const HID_GD_RFKILL_BTN: c_uint = 0x000100c6;
pub const HID_GD_RFKILL_LED: c_uint = 0x000100c7;
pub const HID_GD_RFKILL_SWITCH: c_uint = 0x000100c8;
pub const HID_DC_BATTERYSTRENGTH: c_uint = 0x00060020;
pub const HID_CP_CONSUMER_CONTROL: c_uint = 0x000c0001;
pub const HID_CP_AC_PAN: c_uint = 0x000c0238;
pub const HID_DG_DIGITIZER: c_uint = 0x000d0001;
pub const HID_DG_PEN: c_uint = 0x000d0002;
pub const HID_DG_LIGHTPEN: c_uint = 0x000d0003;
pub const HID_DG_TOUCHSCREEN: c_uint = 0x000d0004;
pub const HID_DG_TOUCHPAD: c_uint = 0x000d0005;
pub const HID_DG_WHITEBOARD: c_uint = 0x000d0006;
pub const HID_DG_STYLUS: c_uint = 0x000d0020;
pub const HID_DG_PUCK: c_uint = 0x000d0021;
pub const HID_DG_FINGER: c_uint = 0x000d0022;
pub const HID_DG_TIPPRESSURE: c_uint = 0x000d0030;
pub const HID_DG_BARRELPRESSURE: c_uint = 0x000d0031;
pub const HID_DG_INRANGE: c_uint = 0x000d0032;
pub const HID_DG_TOUCH: c_uint = 0x000d0033;
pub const HID_DG_UNTOUCH: c_uint = 0x000d0034;
pub const HID_DG_TAP: c_uint = 0x000d0035;
pub const HID_DG_TRANSDUCER_INDEX: c_uint = 0x000d0038;
pub const HID_DG_TABLETFUNCTIONKEY: c_uint = 0x000d0039;
pub const HID_DG_PROGRAMCHANGEKEY: c_uint = 0x000d003a;
pub const HID_DG_BATTERYSTRENGTH: c_uint = 0x000d003b;
pub const HID_DG_INVERT: c_uint = 0x000d003c;
pub const HID_DG_TILT_X: c_uint = 0x000d003d;
pub const HID_DG_TILT_Y: c_uint = 0x000d003e;
pub const HID_DG_TWIST: c_uint = 0x000d0041;
pub const HID_DG_TIPSWITCH: c_uint = 0x000d0042;
pub const HID_DG_TIPSWITCH2: c_uint = 0x000d0043;
pub const HID_DG_BARRELSWITCH: c_uint = 0x000d0044;
pub const HID_DG_ERASER: c_uint = 0x000d0045;
pub const HID_DG_TABLETPICK: c_uint = 0x000d0046;
pub const HID_DG_PEN_COLOR: c_uint = 0x000d005c;
pub const HID_DG_PEN_LINE_WIDTH: c_uint = 0x000d005e;
pub const HID_DG_PEN_LINE_STYLE: c_uint = 0x000d0070;
pub const HID_DG_PEN_LINE_STYLE_INK: c_uint = 0x000d0072;
pub const HID_DG_PEN_LINE_STYLE_PENCIL: c_uint = 0x000d0073;
pub const HID_DG_PEN_LINE_STYLE_HIGHLIGHTER: c_uint = 0x000d0074;
pub const HID_DG_PEN_LINE_STYLE_CHISEL_MARKER: c_uint = 0x000d0075;
pub const HID_DG_PEN_LINE_STYLE_BRUSH: c_uint = 0x000d0076;
pub const HID_DG_PEN_LINE_STYLE_NO_PREFERENCE: c_uint = 0x000d0077;
pub const HID_CP_CONSUMERCONTROL: c_uint = 0x000c0001;
pub const HID_CP_NUMERICKEYPAD: c_uint = 0x000c0002;
pub const HID_CP_PROGRAMMABLEBUTTONS: c_uint = 0x000c0003;
pub const HID_CP_MICROPHONE: c_uint = 0x000c0004;
pub const HID_CP_HEADPHONE: c_uint = 0x000c0005;
pub const HID_CP_GRAPHICEQUALIZER: c_uint = 0x000c0006;
pub const HID_CP_FUNCTIONBUTTONS: c_uint = 0x000c0036;
pub const HID_CP_SELECTION: c_uint = 0x000c0080;
pub const HID_CP_MEDIASELECTION: c_uint = 0x000c0087;
pub const HID_CP_SELECTDISC: c_uint = 0x000c00ba;
pub const HID_CP_VOLUMEUP: c_uint = 0x000c00e9;
pub const HID_CP_VOLUMEDOWN: c_uint = 0x000c00ea;
pub const HID_CP_PLAYBACKSPEED: c_uint = 0x000c00f1;
pub const HID_CP_PROXIMITY: c_uint = 0x000c0109;
pub const HID_CP_SPEAKERSYSTEM: c_uint = 0x000c0160;
pub const HID_CP_CHANNELLEFT: c_uint = 0x000c0161;
pub const HID_CP_CHANNELRIGHT: c_uint = 0x000c0162;
pub const HID_CP_CHANNELCENTER: c_uint = 0x000c0163;
pub const HID_CP_CHANNELFRONT: c_uint = 0x000c0164;
pub const HID_CP_CHANNELCENTERFRONT: c_uint = 0x000c0165;
pub const HID_CP_CHANNELSIDE: c_uint = 0x000c0166;
pub const HID_CP_CHANNELSURROUND: c_uint = 0x000c0167;
pub const HID_CP_CHANNELLOWFREQUENCYENHANCEMENT: c_uint = 0x000c0168;
pub const HID_CP_CHANNELTOP: c_uint = 0x000c0169;
pub const HID_CP_CHANNELUNKNOWN: c_uint = 0x000c016a;
pub const HID_CP_APPLICATIONLAUNCHBUTTONS: c_uint = 0x000c0180;
pub const HID_CP_GENERICGUIAPPLICATIONCONTROLS: c_uint = 0x000c0200;
pub const HID_DG_DEVICECONFIG: c_uint = 0x000d000e;
pub const HID_DG_DEVICESETTINGS: c_uint = 0x000d0023;
pub const HID_DG_AZIMUTH: c_uint = 0x000d003f;
pub const HID_DG_CONFIDENCE: c_uint = 0x000d0047;
pub const HID_DG_WIDTH: c_uint = 0x000d0048;
pub const HID_DG_HEIGHT: c_uint = 0x000d0049;
pub const HID_DG_CONTACTID: c_uint = 0x000d0051;
pub const HID_DG_INPUTMODE: c_uint = 0x000d0052;
pub const HID_DG_DEVICEINDEX: c_uint = 0x000d0053;
pub const HID_DG_CONTACTCOUNT: c_uint = 0x000d0054;
pub const HID_DG_CONTACTMAX: c_uint = 0x000d0055;
pub const HID_DG_SCANTIME: c_uint = 0x000d0056;
pub const HID_DG_SURFACESWITCH: c_uint = 0x000d0057;
pub const HID_DG_BUTTONSWITCH: c_uint = 0x000d0058;
pub const HID_DG_BUTTONTYPE: c_uint = 0x000d0059;
pub const HID_DG_BARRELSWITCH2: c_uint = 0x000d005a;
pub const HID_DG_TOOLSERIALNUMBER: c_uint = 0x000d005b;
pub const HID_DG_LATENCYMODE: c_uint = 0x000d0060;
pub const HID_HP_SIMPLECONTROLLER: c_uint = 0x000e0001;
pub const HID_HP_WAVEFORMLIST: c_uint = 0x000e0010;
pub const HID_HP_DURATIONLIST: c_uint = 0x000e0011;
pub const HID_HP_AUTOTRIGGER: c_uint = 0x000e0020;
pub const HID_HP_MANUALTRIGGER: c_uint = 0x000e0021;
pub const HID_HP_AUTOTRIGGERASSOCIATEDCONTROL: c_uint = 0x000e0022;
pub const HID_HP_INTENSITY: c_uint = 0x000e0023;
pub const HID_HP_REPEATCOUNT: c_uint = 0x000e0024;
pub const HID_HP_RETRIGGERPERIOD: c_uint = 0x000e0025;
pub const HID_HP_WAVEFORMVENDORPAGE: c_uint = 0x000e0026;
pub const HID_HP_WAVEFORMVENDORID: c_uint = 0x000e0027;
pub const HID_HP_WAVEFORMCUTOFFTIME: c_uint = 0x000e0028;
pub const HID_HP_WAVEFORMNONE: c_uint = 0x000e1001;
pub const HID_HP_WAVEFORMSTOP: c_uint = 0x000e1002;
pub const HID_HP_WAVEFORMCLICK: c_uint = 0x000e1003;
pub const HID_HP_WAVEFORMBUZZCONTINUOUS: c_uint = 0x000e1004;
pub const HID_HP_WAVEFORMRUMBLECONTINUOUS: c_uint = 0x000e1005;
pub const HID_HP_WAVEFORMPRESS: c_uint = 0x000e1006;
pub const HID_HP_WAVEFORMRELEASE: c_uint = 0x000e1007;
pub const HID_HP_VENDORWAVEFORMMIN: c_uint = 0x000e2001;
pub const HID_HP_VENDORWAVEFORMMAX: c_uint = 0x000e2fff;
pub const HID_BAT_ABSOLUTESTATEOFCHARGE: c_uint = 0x00850065;
pub const HID_BAT_CHARGING: c_uint = 0x00850044;
pub const HID_VD_ASUS_CUSTOM_MEDIA_KEYS: c_uint = 0xff310076;
//
// HID connect requests
//

//
// HID device quirks.
//
// Increase this if you need to configure more HID quirks at module load time
//
pub const MAX_USBHID_BOOT_QUIRKS: c_int = 4;
//
// DOC: HID quirks
// | @HID_QUIRK_NOTOUCH:
// | @HID_QUIRK_IGNORE: ignore this device
// | @HID_QUIRK_NOGET:
// | @HID_QUIRK_HIDDEV_FORCE:
// | @HID_QUIRK_BADPAD:
// | @HID_QUIRK_MULTI_INPUT:
// | @HID_QUIRK_HIDINPUT_FORCE:
// | @HID_QUIRK_ALWAYS_POLL:
// | @HID_QUIRK_INPUT_PER_APP:
// | @HID_QUIRK_X_INVERT:
// | @HID_QUIRK_Y_INVERT:
// | @HID_QUIRK_IGNORE_MOUSE:
// | @HID_QUIRK_SKIP_OUTPUT_REPORTS:
// | @HID_QUIRK_SKIP_OUTPUT_REPORT_ID:
// | @HID_QUIRK_NO_OUTPUT_REPORTS_ON_INTR_EP:
// | @HID_QUIRK_HAVE_SPECIAL_DRIVER:
// | @HID_QUIRK_INCREMENT_USAGE_ON_DUPLICATE:
// | @HID_QUIRK_IGNORE_SPECIAL_DRIVER
// | @HID_QUIRK_POWER_ON_AFTER_BACKLIGHT
// | @HID_QUIRK_FULLSPEED_INTERVAL:
// | @HID_QUIRK_NO_INIT_REPORTS:
// | @HID_QUIRK_NO_IGNORE:
// | @HID_QUIRK_NO_INPUT_SYNC:
//
// BIT(0) reserved for backward compatibility, was HID_QUIRK_INVERT

// BIT(8) reserved for backward compatibility, was HID_QUIRK_NO_EMPTY_INPUT
// BIT(9) reserved for backward compatibility, was NO_INIT_INPUT_REPORTS

//
// HID device groups
//
// Note: HID_GROUP_ANY is declared in linux/mod_devicetable.h
// and has a value of 0x0000
//
pub const HID_GROUP_GENERIC: c_uint = 0x0001;
pub const HID_GROUP_MULTITOUCH: c_uint = 0x0002;
pub const HID_GROUP_SENSOR_HUB: c_uint = 0x0003;
pub const HID_GROUP_MULTITOUCH_WIN_8: c_uint = 0x0004;
//
// Vendor specific HID device groups
//
pub const HID_GROUP_RMI: c_uint = 0x0100;
pub const HID_GROUP_WACOM: c_uint = 0x0101;
pub const HID_GROUP_LOGITECH_DJ_DEVICE: c_uint = 0x0102;
pub const HID_GROUP_STEAM: c_uint = 0x0103;
pub const HID_GROUP_LOGITECH_27MHZ_DEVICE: c_uint = 0x0104;
pub const HID_GROUP_VIVALDI: c_uint = 0x0105;
//
// HID protocol status
//
pub const HID_REPORT_PROTOCOL: c_int = 1;
pub const HID_BOOT_PROTOCOL: c_int = 0;
//
// HID units
//
pub const HID_UNIT_GRAM: c_uint = 0x0101;
pub const HID_UNIT_NEWTON: c_uint = 0xe111;
//
// This is the global environment of the parser. This information is
// persistent for main-items. The global environment can be saved and
// restored with PUSH/POP statements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_global {
    pub usage_page: unsigned,
    pub logical_minimum: __s32,
    pub logical_maximum: __s32,
    pub physical_minimum: __s32,
    pub physical_maximum: __s32,
    pub unit_exponent: __s32,
    pub unit: unsigned,
    pub report_id: unsigned,
    pub report_size: unsigned,
    pub report_count: unsigned,
}

//
// This is the local environment. It is persistent up the next main-item.
//
pub const HID_MAX_USAGES: c_int = 12288;
pub const HID_DEFAULT_NUM_COLLECTIONS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_local {
    pub /: *mut *mut unsigned usage[HID_MAX_USAGES]; / usage array,
    pub /: *mut *mut u8 usage_size[HID_MAX_USAGES]; / usage size array,
    pub /: *mut *mut unsigned collection_index[HID_MAX_USAGES]; / collection index array,
    pub usage_index: unsigned,
    pub usage_minimum: unsigned,
    pub delimiter_depth: unsigned,
    pub delimiter_branch: unsigned,
}

//
// This is the collection stack. We climb up the stack to determine
// application and function of each field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_collection {
    pub /: *mut *mut int parent_idx; / device->collection,
    pub type: unsigned,
    pub usage: unsigned,
    pub level: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_usage {
    pub /: *mut *mut unsigned hid; / hid usage code,
    pub /: *mut *mut unsigned collection_index; / index into collection array,
    pub /: *mut *mut unsigned usage_index; / index into usage array,
    pub Multiplier: *mut *mut __s8 resolution_multiplier;/ Effective Resolution,
// hidinput data
    pub /: *mut *mut __s8 wheel_factor; / 120/resolution_multiplier,
    pub /: *mut *mut __u16 code; / input driver code,
    pub /: *mut *mut __u8 type; / input driver type,
    pub /: *mut *mut __s16 hat_min; / hat switch fun,
    pub /: *mut *mut __s16 hat_max; / ditto,
    pub /: *mut *mut __s16 hat_dir; / ditto,
    pub /: *mut *mut __s16 wheel_accumulated; / hi-res wheel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_field {
    pub /: *mut *mut unsigned physical; / physical usage for this field,
    pub /: *mut *mut unsigned logical; / logical usage for this field,
    pub /: *mut *mut unsigned application; / application usage for this field,
    pub /: *mut *mut *mut hid_usage usage; / usage table for this function,
    pub /: *mut *mut unsigned maxusage; / maximum usage index,
    pub /: *mut *mut unsigned flags; / main-item flags (i.e. volatile,array,constant),
    pub /: *mut *mut unsigned report_offset; / bit offset in the report,
    pub /: *mut *mut unsigned report_size; / size of this field in the report,
    pub /: *mut *mut unsigned report_count; / number of this field in the report,
    pub /: *mut *mut unsigned report_type; / (input,output,feature),
    pub /: *mut *mut *mut __s32 value; / last known value(s),
    pub /: *mut *mut *mut __s32 new_value; / newly read value(s),
    pub report: *mut *mut *mut __s32 usages_priorities; / priority of each usage when reading the,
// bits 8-16 are reserved for hid-input usage
//
    pub logical_minimum: __s32,
    pub logical_maximum: __s32,
    pub physical_minimum: __s32,
    pub physical_maximum: __s32,
    pub unit_exponent: __s32,
    pub unit: unsigned,
    pub /: *mut *mut bool ignored; / this field is ignored in this event,
    pub /: *mut *mut *mut hid_report report; / associated report,
    pub /: *mut *mut unsigned index; / index into report->field[],
// hidinput data
    pub /: *mut *mut *mut hid_input hidinput; / associated input structure,
    pub /: *mut *mut __u16 dpad; / dpad input code,
    pub /: *mut *mut unsigned int slot_idx; / slot index in a report,
}

pub const HID_MAX_FIELDS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_field_entry {
    pub list: list_head,
    pub field: *mut hid_field,
    pub index: c_uint,
    pub priority: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_report {
    pub list: list_head,
    pub hidinput_list: list_head,
    pub /: *mut *mut list_head field_entry_list; / ordered list of input fields,
    pub /: *mut *mut unsigned int id; / id of this report,
    pub /: *mut *mut hid_report_type type; / report type,
    pub /: *mut *mut unsigned int application; / application usage for this report,
    pub /: *mut *mut *mut hid_field field[HID_MAX_FIELDS]; / fields of the report,
    pub /: *mut *mut *mut hid_field_entry field_entries; / allocated memory of input field_entry,
    pub /: *mut *mut unsigned maxfield; / maximum valid field index,
    pub /: *mut *mut unsigned size; / size of the report (bits),
    pub /: *mut *mut *mut hid_device device; / associated device,
// tool related state
    pub /: *mut *mut bool tool_active; / whether the current tool is active,
    pub /: *mut *mut *mut unsigned int tool; / BTN_TOOL_,
}

pub const HID_MAX_IDS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_report_enum {
    pub numbered: unsigned,
    pub report_list: list_head,
    pub report_id_hash: [*mut hid_report; HID_MAX_IDS],
}

pub const HID_OUTPUT_FIFO_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_control_fifo {
    pub dir: c_uchar,
    pub report: *mut hid_report,
    pub raw_report: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_output_fifo {
    pub report: *mut hid_report,
    pub raw_report: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_input {
    pub list: list_head,
    pub report: *mut hid_report,
    pub input: *mut input_dev,
    pub name: *const c_char,
    pub /: *mut *mut list_head reports; / the list of reports,
    pub /: *mut *mut unsigned int application; / application usage for this input,
    pub registered: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_type {
    HID_TYPE_OTHER = 0,
    HID_TYPE_USBMOUSE,
    HID_TYPE_USBNONE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hid_battery_status {
    HID_BATTERY_UNKNOWN = 0,
    HID_BATTERY_QUERIED,		/* Kernel explicitly queried battery strength */
    HID_BATTERY_REPORTED,		/* Device sent unsolicited battery strength report */
}

//
// struct hid_battery - represents a single battery power supply
// @dev: pointer to the parent hid_device
// @ps: the power supply instance
// @min: minimum battery value from HID descriptor
// @max: maximum battery value from HID descriptor
// @report_type: HID report type (input/feature)
// @report_id: HID report ID for this battery
// @report_offset: bit offset of the capacity field within its report
// @charge_status: current charging status
// @status: battery reporting status
// @capacity: current battery capacity (0-100)
// @avoid_query: if true, avoid querying battery (e.g., for stylus)
// @present: if true, battery is present (may be dynamic)
// @ratelimit_time: rate limiting for battery reports
// @list: list node for linking into hid_device's battery list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_battery {
    pub dev: *mut hid_device,
    pub ps: *mut power_supply,
    pub min: __s32,
    pub max: __s32,
    pub report_type: __s32,
    pub report_id: __s32,
    pub report_offset: __s32,
    pub charge_status: __s32,
    pub status: hid_battery_status,
    pub capacity: __s32,
    pub avoid_query: bool,
    pub present: bool,
    pub ratelimit_time: ktime_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_device {
    pub /: *const *const *const __u8 dev_rdesc; / device report descriptor,
    pub /: *const *const *const __u8 bpf_rdesc; / bpf modified report descriptor, if any,
    pub /: *const *const *const __u8 rdesc; / currently used report descriptor,
    pub dev_rsize: c_uint,
    pub bpf_rsize: c_uint,
    pub rsize: c_uint,
    pub /: *mut *mut unsigned int collection_size; / Number of allocated hid_collections,
    pub /: *mut *mut *mut hid_collection collection; / List of HID collections,
    pub /: *mut *mut unsigned int maxcollection; / Number of parsed collections,
    pub /: *mut *mut unsigned int maxapplication; / Number of applications,
    pub /: *mut *mut __u16 bus; / BUS ID,
    pub /: *mut *mut __u16 group; / Report group,
    pub /: *mut *mut __u32 vendor; / Vendor ID,
    pub /: *mut *mut __u32 product; / Product ID,
    pub /: *mut *mut __u32 version; / HID version,
    pub /: *mut *mut hid_type type; / device type (mouse, kbd, ...),
    pub /: *mut *mut unsigned country; / HID country,
    pub report_enum: [hid_report_enum; HID_REPORT_TYPES],
    pub /: *mut *mut work_led_work; / delayed LED worker,
    pub /: *mut *mut semaphore driver_input_lock; / protects the current driver,
    pub /: *mut *mut device dev; / device,
    pub driver: *mut hid_driver,
    pub /: *mut *mut *mut void devres_group_id; / ID of probe devres group,
    pub ll_driver: *const hid_ll_driver,
    pub ll_open_lock: mutex,
    pub ll_open_count: c_uint,

//
// Power supply information for HID devices which report
// battery strength. Each battery is tracked separately in the
// batteries list.
//
    pub batteries: list_head,

    pub /: *mut *mut unsigned long status; / see STAT flags above,
    pub /: *mut *mut unsigned claimed; / Claimed by hidinput, hiddev?,
    pub /: *mut *mut unsigned quirks; / Various quirks the device can pull on us,
    pub /: *mut *mut unsigned initial_quirks; / Initial set of quirks supplied when creating device,
    pub /: *mut *mut bool io_started; / If IO has started,
    pub /: *mut *mut list_head inputs; / The list of inputs,
    pub /: *mut *mut *mut void hiddev; / The hiddev structure,
    pub hidraw: *mut c_void,
    pub /: *mut *mut char name[128]; / Device name,
    pub /: *mut *mut char phys[64]; / Device physical location,
    pub /: *mut *mut char uniq[64]; / Device unique identifier (serial #),
    pub /: *mut *mut u64 firmware_version; / Firmware version,
    pub driver_data: *mut c_void,
// temporary hid_ff handling (until moved to the drivers)
    pub ): *mut *mut int (ff_init)(struct hid_device,
// hiddev event handler
    pub int): *mut *mut *mut int (hiddev_connect)(struct hid_device , unsigned,
    pub ): *mut *mut void (hiddev_disconnect)(struct hid_device,
    pub __s32): *mut *mut hid_usage ,,
    pub ): *mut *mut *mut void (hiddev_report_event) (struct hid_device , struct hid_report,
// debugging support via debugfs
    pub debug: c_ushort,
    pub debug_dir: *mut dentry,
    pub debug_rdesc: *mut dentry,
    pub debug_events: *mut dentry,
    pub debug_list: list_head,
    pub debug_list_lock: spinlock_t,
    pub debug_wait: wait_queue_head_t,
    pub ref: kref,
    pub /: *mut *mut unsigned int id; / system unique id,

    pub /: *mut *mut hid_bpf bpf; / hid-bpf data,

}

extern "C" {
    pub fn hiddev_free(ref: *mut kref);
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &hdev->dev) -> return;
}

extern "C" {
    pub fn list_first_entry(_arg: &hdev->batteries, hid_battery: struct, _arg: list) -> return;
}

pub const HID_GLOBAL_STACK_SIZE: c_int = 4;
pub const HID_COLLECTION_STACK_SIZE: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_parser {
    pub global: hid_global,
    pub global_stack: [hid_global; HID_GLOBAL_STACK_SIZE],
    pub global_stack_ptr: c_uint,
    pub local: hid_local,
    pub collection_stack: *mut c_uint,
    pub collection_stack_ptr: c_uint,
    pub collection_stack_size: c_uint,
    pub device: *mut hid_device,
    pub scan_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_class_descriptor {
    pub bDescriptorType: __u8,
    pub wDescriptorLength: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdHID: __le16,
    pub bCountryCode: __u8,
    pub bNumDescriptors: __u8,
    pub rpt_desc: hid_class_descriptor,
    pub opt_descs: [hid_class_descriptor; ],
// C attribute field omitted

// we don't want to catch types and codes equal to 0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_report_id {
    pub report_type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_usage_id {
    pub usage_hid: __u32,
    pub usage_type: __u32,
    pub usage_code: __u32,
}

//
// struct hid_driver
// @name: driver name (e.g. "Footech_bar-wheel")
// @id_table: which devices is this driver for (must be non-NULL for probe
// to be called)
// @dyn_list: list of dynamically added device ids
// @dyn_lock: lock protecting @dyn_list
// @match: check if the given device is handled by this driver
// @probe: new device inserted
// @remove: device removed (NULL if not a hot-plug capable driver)
// @report_table: on which reports to call raw_event (NULL means all)
// @raw_event: if report in report_table, this hook is called (NULL means nop)
// @usage_table: on which events to call event (NULL means all)
// @event: if usage in usage_table, this hook is called (NULL means nop)
// @report: this hook is called after parsing a report (NULL means nop)
// @report_fixup: called before report descriptor parsing (NULL means nop)
// @input_mapping: invoked on input registering before mapping an usage
// @input_mapped: invoked on input registering after mapping an usage
// @input_configured: invoked just before the device is registered
// @feature_mapping: invoked on feature registering
// @suspend: invoked on suspend (NULL means nop)
// @resume: invoked on resume if device was not reset (NULL means nop)
// @reset_resume: invoked on resume if device was reset (NULL means nop)
// @on_hid_hw_open: invoked when hid core opens first instance (NULL means nop)
// @on_hid_hw_close: invoked when hid core closes last instance (NULL means nop)
//
// probe should return -errno on error, or 0 on success. During probe,
// input will not be passed to raw_event unless hid_device_io_start is
// called.
//
// raw_event and event should return negative on error, any other value will
// pass the event on to .event() typically return 0 for success.
//
// report_fixup must return a report descriptor pointer whose lifetime is at
// least that of the input rdesc.  This is usually done by mutating the input
// rdesc and returning it or a sub-portion of it.  In case a new buffer is
// allocated and returned, the implementation of report_fixup is responsible for
// freeing it later.
//
// input_mapping shall return a negative value to completely ignore this usage
// (e.g. doubled or invalid usage), zero to continue with parsing of this
// usage by generic code (no special handling needed) or positive to skip
// generic parsing (needed special handling which was done in the hook already)
// input_mapped shall return negative to inform the layer that this usage
// should not be considered for further processing or zero to notify that
// no processing was performed and should be done in a generic manner
// Both these functions may be NULL which means the same behavior as returning
// zero from them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_driver {
    pub name: *const c_char,
    pub id_table: *const hid_device_id,
    pub dyn_list: list_head,
    pub dyn_lock: spinlock_t,
    pub ignore_special_driver): *mut *mut *mut bool (match)(struct hid_device dev, bool,
    pub id): *const *const *const int (probe)(struct hid_device dev, struct hid_device_id,
    pub dev): *mut *mut void (remove)(struct hid_device,
    pub report_table: *const hid_report_id,
    pub size): *mut *mut u8 data, int,
    pub usage_table: *const hid_usage_id,
    pub value): *mut *mut hid_usage usage, __s32,
    pub report): *mut *mut *mut void (report)(struct hid_device hdev, struct hid_report,
    pub size): *mut c_uint,
    pub max): *mut *mut *mut *mut hid_usage usage, unsigned long bit, int,
    pub max): *mut *mut *mut *mut hid_usage usage, unsigned long bit, int,
    pub hidinput): *mut hid_input,
    pub usage): *mut hid_usage,
    pub message): *mut *mut *mut int (suspend)(struct hid_device hdev, pm_message_t,
    pub hdev): *mut *mut int (resume)(struct hid_device,
    pub hdev): *mut *mut int (reset_resume)(struct hid_device,
    pub hdev): *mut *mut void (on_hid_hw_open)(struct hid_device,
    pub hdev): *mut *mut void (on_hid_hw_close)(struct hid_device,
// private:
    pub driver: device_driver,
}

//
// struct hid_ll_driver - low level driver callbacks
// @start: called on probe to start the device
// @stop: called on remove
// @open: called by input layer on open
// @close: called by input layer on close
// @power: request underlying hardware to enter requested power mode
// @parse: this method is called only once to parse the device data,
// shouldn't allocate anything to not leak memory
// @request: send report request to device (e.g. feature report)
// @wait: wait for buffered io to complete (send/recv reports)
// @raw_request: send raw report request to device (e.g. feature report)
// @output_report: send output report to device
// @idle: send idle request to device
// @may_wakeup: return if device may act as a wakeup source during system-suspend
// @max_buffer_size: over-ride maximum data buffer size (default: HID_MAX_BUFFER_SIZE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_ll_driver {
    pub hdev): *mut *mut int (start)(struct hid_device,
    pub hdev): *mut *mut void (stop)(struct hid_device,
    pub hdev): *mut *mut int (open)(struct hid_device,
    pub hdev): *mut *mut void (close)(struct hid_device,
    pub level): *mut *mut *mut int (power)(struct hid_device hdev, int,
    pub hdev): *mut *mut int (parse)(struct hid_device,
    pub reqtype): *mut *mut hid_report report, int,
    pub hdev): *mut *mut int (wait)(struct hid_device,
    pub reqtype): c_int,
    pub len): *mut *mut *mut *mut int (output_report) (struct hid_device hdev, __u8 buf, size_t,
    pub reqtype): *mut *mut *mut int (idle)(struct hid_device hdev, int report, int idle, int,
    pub hdev): *mut *mut bool (may_wakeup)(struct hid_device,
    pub max_buffer_size: c_uint,
}

extern "C" {
    pub fn hid_is_usb(hdev: *const hid_device) -> bool;
}

// Applications from HID Usage Tables 4/8/99 Version 1.1
// We ignore a few input applications that are not widely used

// HID core API
extern "C" {
    pub fn hid_ignore(: *mut hid_device) -> bool;
}
extern "C" {
    pub fn hid_add_device(: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_destroy_device(: *mut hid_device);
}
// use a define to avoid include chaining to get THIS_MODULE & friends

extern "C" {
    pub fn hid_unregister_driver(: *mut hid_driver);
}
//
// module_hid_driver() - Helper macro for registering a HID driver
// @__hid_driver: hid_driver struct
//
// Helper macro for HID drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn hidinput_hid_event(: *mut hid_device, : *mut hid_field, : *mut hid_usage, _arg: __s32);
}
extern "C" {
    pub fn hidinput_report_event(hid: *mut hid_device, report: *mut hid_report);
}
extern "C" {
    pub fn hidinput_connect(hid: *mut hid_device, connect_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn hidinput_disconnect(: *mut hid_device);
}
extern "C" {
    pub fn hidinput_reset_resume(hid: *mut hid_device);
}
extern "C" {
    pub fn hid_set_field(: *mut hid_field, _arg: unsigned, _arg: __s32) -> c_int;
}
extern "C" {
    pub fn hidinput_count_leds(hid: *mut hid_device) -> c_uint;
}
extern "C" {
    pub fn hidinput_calc_abs_res(field: *const hid_field, code: __u16) -> __s32;
}
extern "C" {
    pub fn hid_output_report(report: *mut hid_report, data: *mut __u8);
}
extern "C" {
    pub fn __hid_request(hid: *mut hid_device, rep: *mut hid_report, reqtype: hid_class_request) -> c_int;
}
extern "C" {
    pub fn hid_parse_report(hid: *mut hid_device, start: *const __u8, size: unsigned) -> c_int;
}
extern "C" {
    pub fn hid_setup_resolution_multiplier(hid: *mut hid_device);
}
extern "C" {
    pub fn hid_open_report(device: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_check_keys_pressed(hid: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_connect(hid: *mut hid_device, connect_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn hid_disconnect(hid: *mut hid_device);
}

extern "C" {
    pub fn hid_driver_suspend(hdev: *mut hid_device, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn hid_driver_reset_resume(hdev: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_driver_resume(hdev: *mut hid_device) -> c_int;
}

//
// hid_device_io_start - enable HID input during probe, remove
//
// @hid: the device
//
// This should only be called during probe or remove and only be
// called by the thread calling probe or remove. It will allow
// incoming packets to be delivered to the driver.
//
// hid_device_io_stop - disable HID input during probe, remove
//
// @hid: the device
//
// Should only be called after hid_device_io_start. It will prevent
// incoming packets from going to the driver for the duration of
// probe, remove. If called during probe, packets will still go to the
// driver after probe is complete. This function should only be called
// by the thread calling probe or remove.
//
// hid_map_usage - map usage input bits
//
// @hidinput: hidinput which we are interested in
// @usage: usage to fill in
// @bit: pointer to input->{}bit (out parameter)
// @max: maximal valid usage->code to consider later (out parameter)
// @type: input event type (EV_KEY, EV_REL, ...)
// @c: code which corresponds to this usage and type
//
// The value pointed to by @bit will be set to NULL if either @type is
// an unhandled event type, or if @c is out of range for @type. This
// can be used as an error condition.
//
// bit = NULL;
// max = limit;
// bit = bmap;
//
// hid_map_usage_clear - map usage input bits and clear the input bit
//
// @hidinput: hidinput which we are interested in
// @usage: usage to fill in
// @bit: pointer to input->{}bit (out parameter)
// @max: maximal valid usage->code to consider later (out parameter)
// @type: input event type (EV_KEY, EV_REL, ...)
// @c: code which corresponds to this usage and type
//
// The same as hid_map_usage, except the @c bit is also cleared in supported
// bits (@bit).
//
// hid_parse - parse HW reports
//
// @hdev: hid device
//
// Call this from probe after you set up the device (if needed). Your
// report_fixup will be called (if non-NULL) after reading raw report from
// device before passing it to hid layer for real parsing.
//
extern "C" {
    pub fn hid_open_report(_arg: hdev) -> return;
}
extern "C" {
    pub fn hid_hw_stop(hdev: *mut hid_device);
}
extern "C" {
    pub fn hid_hw_open(hdev: *mut hid_device) -> int __must_check;
}
extern "C" {
    pub fn hid_hw_close(hdev: *mut hid_device);
}
extern "C" {
    pub fn hid_hw_output_report(hdev: *mut hid_device, buf: *mut __u8, len: usize) -> c_int;
}
//
// hid_hw_power - requests underlying HW to go into given power mode
//
// @hdev: hid device
// @level: requested power level (one of %PM_HINT_* defines)
//
// This function requests underlying hardware to enter requested power
// mode.
//
// hid_hw_idle - send idle request to device
//
// @hdev: hid device
// @report: report to control
// @idle: idle state
// @reqtype: hid request type
//
// hid_hw_may_wakeup - return if the hid device may act as a wakeup source during system-suspend
//
// @hdev: hid device
//
extern "C" {
    pub fn device_may_wakeup(_arg: hdev->dev.parent) -> return;
}
//
// hid_hw_wait - wait for buffered io to complete
//
// @hdev: hid device
//
// hid_report_len - calculate the report length
//
// @report: the report whose length we want to know
//
// The length counts the report ID byte, but only if the ID is nonzero
// and therefore is included in the report.  Reports whose ID is zero
// never include an ID byte.
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: report->size, 0: 8) + (report->id >) -> return;
}
// HID quirks API
extern "C" {
    pub fn hid_lookup_quirk(hdev: *const hid_device) -> c_ulong;
}
extern "C" {
    pub fn hid_quirks_init(quirks_param: *mut c_char, bus: __u16, count: c_int) -> c_int;
}
extern "C" {
    pub fn hid_quirks_exit(bus: __u16);
}

