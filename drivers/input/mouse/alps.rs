//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/alps.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ALPS touchpad PS/2 mouse driver
//
// Copyright (c) 2003 Peter Osterlund <petero2@telia.com>
// Copyright (c) 2005 Vojtech Pavlik <vojtech@suse.cz>
//

pub const ALPS_PROTO_V1: c_uint = 0x100;
pub const ALPS_PROTO_V2: c_uint = 0x200;
pub const ALPS_PROTO_V3: c_uint = 0x300;
pub const ALPS_PROTO_V3_RUSHMORE: c_uint = 0x310;
pub const ALPS_PROTO_V4: c_uint = 0x400;
pub const ALPS_PROTO_V5: c_uint = 0x500;
pub const ALPS_PROTO_V6: c_uint = 0x600;
pub const ALPS_PROTO_V7: c_uint = 0x700	/* t3btl t4s */;
pub const ALPS_PROTO_V8: c_uint = 0x800	/* SS4btl SS4s */;
pub const ALPS_PROTO_V9: c_uint = 0x900	/* ss3btl */;
pub const MAX_TOUCHES: c_int = 4;
pub const DOLPHIN_COUNT_PER_ELECTRODE: c_int = 64;

//
// enum SS4_PACKET_ID - defines the packet type for V8
// SS4_PACKET_ID_IDLE: There's no finger and no button activity.
// SS4_PACKET_ID_ONE: There's one finger on touchpad
// or there's button activities.
// SS4_PACKET_ID_TWO: There's two or more fingers on touchpad
// SS4_PACKET_ID_MULTI: There's three or more fingers on touchpad
// SS4_PACKET_ID_STICK: A stick pointer packet
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SS4_PACKET_ID {
    SS4_PACKET_ID_IDLE = 0,
    SS4_PACKET_ID_ONE,
    SS4_PACKET_ID_TWO,
    SS4_PACKET_ID_MULTI,
    SS4_PACKET_ID_STICK,
}

pub const SS4_COUNT_PER_ELECTRODE: c_int = 256;
pub const SS4_NUMSENSOR_XOFFSET: c_int = 7;
pub const SS4_NUMSENSOR_YOFFSET: c_int = 7;
pub const SS4_MIN_PITCH_MM: c_int = 50;
pub const SS4_MASK_NORMAL_BUTTONS: c_uint = 0x07;
pub const SS4PLUS_COUNT_PER_ELECTRODE: c_int = 128;
pub const SS4PLUS_NUMSENSOR_XOFFSET: c_int = 16;
pub const SS4PLUS_NUMSENSOR_YOFFSET: c_int = 5;
pub const SS4PLUS_MIN_PITCH_MM: c_int = 37;

//
// enum V7_PACKET_ID - defines the packet type for V7
// V7_PACKET_ID_IDLE: There's no finger and no button activity.
// V7_PACKET_ID_TWO: There's one or two non-resting fingers on touchpad
// or there's button activities.
// V7_PACKET_ID_MULTI: There are at least three non-resting fingers.
// V7_PACKET_ID_NEW: The finger position in slot is not continues from
// previous packet.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum V7_PACKET_ID {
    V7_PACKET_ID_IDLE,
    V7_PACKET_ID_TWO,
    V7_PACKET_ID_MULTI,
    V7_PACKET_ID_NEW,
    V7_PACKET_ID_UNKNOWN,
}

//
// struct alps_protocol_info - information about protocol used by a device
// @version: Indicates V1/V2/V3/...
// @byte0: Helps figure out whether a position report packet matches the
// known format for this model.  The first byte of the report, ANDed with
// mask0, should match byte0.
// @mask0: The mask used to check the first byte of the report.
// @flags: Additional device capabilities (passthrough port, trackstick, etc.).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_protocol_info {
    pub version: u16,
    pub mask0: u8 byte0,,
    pub flags: c_uint,
}

//
// struct alps_model_info - touchpad ID table
// @signature: E7 response string to match.
// @protocol_info: information about protocol used by the device.
//
// Many (but not all) ALPS touchpads can be identified by looking at the
// values returned in the "E7 report" and/or the "EC report."  This table
// lists a number of such touchpads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_model_info {
    pub signature: [u8; 3],
    pub protocol_info: alps_protocol_info,
}

//
// struct alps_nibble_commands - encodings for register accesses
// @command: PS/2 command used for the nibble
// @data: Data supplied as an argument to the PS/2 command, if applicable
//
// The ALPS protocol uses magic sequences to transmit binary data to the
// touchpad, as it is generally not OK to send arbitrary bytes out the
// PS/2 port.  Each of the sequences in this table sends one nibble of the
// register address or (write) data.  Different versions of the ALPS protocol
// use slightly different encodings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_nibble_commands {
    pub command: c_int,
    pub data: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_bitmap_point {
    pub start_bit: c_int,
    pub num_bits: c_int,
}

//
// struct alps_fields - decoded version of the report packet
// @x_map: Bitmap of active X positions for MT.
// @y_map: Bitmap of active Y positions for MT.
// @fingers: Number of fingers for MT.
// @pressure: Pressure.
// @st: position for ST.
// @mt: position for MT.
// @first_mp: Packet is the first of a multi-packet report.
// @is_mp: Packet is part of a multi-packet report.
// @left: Left touchpad button is active.
// @right: Right touchpad button is active.
// @middle: Middle touchpad button is active.
// @ts_left: Left trackstick button is active.
// @ts_right: Right trackstick button is active.
// @ts_middle: Middle trackstick button is active.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_fields {
    pub x_map: c_uint,
    pub y_map: c_uint,
    pub fingers: c_uint,
    pub pressure: c_int,
    pub st: input_mt_pos,
    pub mt: [input_mt_pos; MAX_TOUCHES],
    pub first_mp:1: c_uint,
    pub is_mp:1: c_uint,
    pub left:1: c_uint,
    pub right:1: c_uint,
    pub middle:1: c_uint,
    pub ts_left:1: c_uint,
    pub ts_right:1: c_uint,
    pub ts_middle:1: c_uint,
}

//
// struct alps_data - private data structure for the ALPS driver
// @psmouse: Pointer to parent psmouse device
// @dev2: Trackstick device (can be NULL).
// @dev3: Generic PS/2 mouse (can be NULL, delayed registering).
// @phys2: Physical path for the trackstick device.
// @phys3: Physical path for the generic PS/2 mouse.
// @dev3_register_work: A work instance for registering PS/2 mouse.
// @nibble_commands: Command mapping used for touchpad register accesses.
// @addr_command: Command used to tell the touchpad that a register address
// follows.
// @proto_version: Indicates V1/V2/V3/...
// @byte0: Helps figure out whether a position report packet matches the
// known format for this model.  The first byte of the report, ANDed with
// mask0, should match byte0.
// @mask0: The mask used to check the first byte of the report.
// @fw_ver: cached copy of firmware version (EC report)
// @flags: Additional device capabilities (passthrough port, trackstick, etc.).
// @x_max: Largest possible X position value.
// @y_max: Largest possible Y position value.
// @x_bits: Number of X bits in the MT bitmap.
// @y_bits: Number of Y bits in the MT bitmap.
// @hw_init: Protocol-specific hardware init function.
// @process_packet: Protocol-specific function to process a report packet.
// @decode_fields: Protocol-specific function to read packet bitfields.
// @set_abs_params: Protocol-specific function to configure the input_dev.
// @prev_fin: Finger bit from previous packet.
// @multi_packet: Multi-packet data in progress.
// @multi_data: Saved multi-packet data.
// @f: Decoded packet data fields.
// @quirks: Bitmap of ALPS_QUIRK_*.
// @timer: Timer for flushing out the final report packet in the stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_data {
    pub psmouse: *mut psmouse,
    pub dev2: *mut input_dev,
    pub dev3: *mut input_dev,
    pub phys2: [c_char; 32],
    pub phys3: [c_char; 32],
    pub dev3_register_work: work_struct,
// these are autodetected when the device is identified
    pub nibble_commands: *const alps_nibble_commands,
    pub addr_command: c_int,
    pub proto_version: u16,
    pub mask0: u8 byte0,,
    pub dev_id: [u8; 3],
    pub fw_ver: [u8; 3],
    pub flags: c_int,
    pub x_max: c_int,
    pub y_max: c_int,
    pub x_bits: c_int,
    pub y_bits: c_int,
    pub x_res: c_uint,
    pub y_res: c_uint,
    pub psmouse): *mut *mut int (hw_init)(struct psmouse,
    pub psmouse): *mut *mut void (process_packet)(struct psmouse,
    pub psmouse): *mut psmouse,
    pub dev1): *mut *mut *mut void (set_abs_params)(struct alps_data priv, struct input_dev,
    pub prev_fin: c_int,
    pub multi_packet: c_int,
    pub second_touch: c_int,
    pub multi_data: [c_uchar; 6],
    pub f: alps_fields,
    pub quirks: u8,
    pub timer: timer_list,
}

extern "C" {
    pub fn alps_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
extern "C" {
    pub fn alps_init(psmouse: *mut psmouse) -> c_int;
}
