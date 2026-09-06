//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/kbdif.h
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


// SPDX-License-Identifier: MIT
//
// kbdif.h -- Xen virtual keyboard/mouse
//
// Copyright (C) 2005 Anthony Liguori <aliguori@us.ibm.com>
// Copyright (C) 2006 Red Hat, Inc., Markus Armbruster <armbru@redhat.com>
//
// Feature and Parameter Negotiation
//
// The two halves of a para-virtual driver utilize nodes within
// XenStore to communicate capabilities and to negotiate operating parameters.
// This section enumerates these nodes which reside in the respective front and
// backend portions of XenStore, following XenBus convention.
//
// All data in XenStore is stored as strings.  Nodes specifying numeric
// values are encoded in decimal. Integer value ranges listed below are
// expressed as fixed sized integer types capable of storing the conversion
// of a properly formated node string, without loss of information.
//
// Backend XenBus Nodes
//
// ---------------------------- Features supported ----------------------------
//
// Capable backend advertises supported features by publishing
// corresponding entries in XenStore and puts 1 as the value of the entry.
// If a feature is not supported then 0 must be set or feature entry omitted.
//
// feature-disable-keyboard
// Values:         <uint>
//
// If there is no need to expose a virtual keyboard device by the
// frontend then this must be set to 1.
//
// feature-disable-pointer
// Values:         <uint>
//
// If there is no need to expose a virtual pointer device by the
// frontend then this must be set to 1.
//
// feature-abs-pointer
// Values:         <uint>
//
// Backends, which support reporting of absolute coordinates for pointer
// device should set this to 1.
//
// feature-multi-touch
// Values:         <uint>
//
// Backends, which support reporting of multi-touch events
// should set this to 1.
//
// feature-raw-pointer
// Values:        <uint>
//
// Backends, which support reporting raw (unscaled) absolute coordinates
// for pointer devices should set this to 1. Raw (unscaled) values have
// a range of [0, 0x7fff].
//
// -----------------------  Device Instance Parameters ------------------------
//
// unique-id
// Values:         <string>
//
// After device instance initialization it is assigned a unique ID,
// so every instance of the frontend can be identified by the backend
// by this ID. This can be UUID or such.
//
// ------------------------- Pointer Device Parameters ------------------------
//
// width
// Values:         <uint>
//
// Maximum X coordinate (width) to be used by the frontend
// while reporting input events, pixels, [0; UINT32_MAX].
//
// height
// Values:         <uint>
//
// Maximum Y coordinate (height) to be used by the frontend
// while reporting input events, pixels, [0; UINT32_MAX].
//
// ----------------------- Multi-touch Device Parameters ----------------------
//
// multi-touch-num-contacts
// Values:         <uint>
//
// Number of simultaneous touches reported.
//
// multi-touch-width
// Values:         <uint>
//
// Width of the touch area to be used by the frontend
// while reporting input events, pixels, [0; UINT32_MAX].
//
// multi-touch-height
// Values:         <uint>
//
// Height of the touch area to be used by the frontend
// while reporting input events, pixels, [0; UINT32_MAX].
//
// Frontend XenBus Nodes
//
// ------------------------------ Feature request -----------------------------
//
// Capable frontend requests features from backend via setting corresponding
// entries to 1 in XenStore. Requests for features not advertised as supported
// by the backend have no effect.
//
// request-abs-pointer
// Values:         <uint>
//
// Request backend to report absolute pointer coordinates
// (XENKBD_TYPE_POS) instead of relative ones (XENKBD_TYPE_MOTION).
//
// request-multi-touch
// Values:         <uint>
//
// Request backend to report multi-touch events.
//
// request-raw-pointer
// Values:         <uint>
//
// Request backend to report raw unscaled absolute pointer coordinates.
// This option is only valid if request-abs-pointer is also set.
// Raw unscaled coordinates have the range [0, 0x7fff]
//
// ----------------------- Request Transport Parameters -----------------------
//
// event-channel
// Values:         <uint>
//
// The identifier of the Xen event channel used to signal activity
// in the ring buffer.
//
// page-gref
// Values:         <uint>
//
// The Xen grant reference granting permission for the backend to map
// a sole page in a single page sized event ring buffer.
//
// page-ref
// Values:         <uint>
//
// OBSOLETE, not recommended for use.
// PFN of the shared page.
//
// EVENT CODES.
//
pub const XENKBD_TYPE_MOTION: c_int = 1;
pub const XENKBD_TYPE_RESERVED: c_int = 2;
pub const XENKBD_TYPE_KEY: c_int = 3;
pub const XENKBD_TYPE_POS: c_int = 4;
pub const XENKBD_TYPE_MTOUCH: c_int = 5;
// Multi-touch event sub-codes
pub const XENKBD_MT_EV_DOWN: c_int = 0;
pub const XENKBD_MT_EV_UP: c_int = 1;
pub const XENKBD_MT_EV_MOTION: c_int = 2;
pub const XENKBD_MT_EV_SYN: c_int = 3;
pub const XENKBD_MT_EV_SHAPE: c_int = 4;
pub const XENKBD_MT_EV_ORIENT: c_int = 5;
//
// CONSTANTS, XENSTORE FIELD AND PATH NAME STRINGS, HELPERS.
//

// OBSOLETE, not recommended for use

//
// Description of the protocol between frontend and backend driver.
//
// The two halves of a Para-virtual driver communicate with
// each other using a shared page and an event channel.
// Shared page contains a ring with event structures.
//
// All reserved fields in the structures below must be 0.
//
// Backend to frontend events
//
// Frontends should ignore unknown in events.
// All event packets have the same length (40 octets)
// All event packets have common header:
//
// 0         octet
// +-----------------+
// |       type      |
// +-----------------+
// type - uint8_t, event code, XENKBD_TYPE_???
//
// Pointer relative movement event
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MOTION  |                     reserved                     | 4
// +----------------+----------------+----------------+----------------+
// |                               rel_x                               | 8
// +----------------+----------------+----------------+----------------+
// |                               rel_y                               | 12
// +----------------+----------------+----------------+----------------+
// |                               rel_z                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// rel_x - int32_t, relative X motion
// rel_y - int32_t, relative Y motion
// rel_z - int32_t, relative Z motion (wheel)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_motion {
    pub type: u8,
    pub rel_x: i32,
    pub rel_y: i32,
    pub rel_z: i32,
}

//
// Key event (includes pointer buttons)
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_KEY     |     pressed    |            reserved             | 4
// +----------------+----------------+----------------+----------------+
// |                              keycode                              | 8
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 12
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// pressed - uint8_t, 1 if pressed; 0 otherwise
// keycode - uint32_t, KEY_* from linux/input.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_key {
    pub type: u8,
    pub pressed: u8,
    pub keycode: u32,
}

//
// Pointer absolute position event
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_POS     |                     reserved                     | 4
// +----------------+----------------+----------------+----------------+
// |                               abs_x                               | 8
// +----------------+----------------+----------------+----------------+
// |                               abs_y                               | 12
// +----------------+----------------+----------------+----------------+
// |                               rel_z                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// abs_x - int32_t, absolute X position (in FB pixels)
// abs_y - int32_t, absolute Y position (in FB pixels)
// rel_z - int32_t, relative Z motion (wheel)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_position {
    pub type: u8,
    pub abs_x: i32,
    pub abs_y: i32,
    pub rel_z: i32,
}

//
// Multi-touch event and its sub-types
//
// All multi-touch event packets have common header:
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |   event_type   |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
//
// event_type - unt8_t, multi-touch event sub-type, XENKBD_MT_EV_???
// contact_id - unt8_t, ID of the contact
//
// Touch interactions can consist of one or more contacts.
// For each contact, a series of events is generated, starting
// with a down event, followed by zero or more motion events,
// and ending with an up event. Events relating to the same
// contact point can be identified by the ID of the sequence: contact ID.
// Contact ID may be reused after XENKBD_MT_EV_UP event and
// is in the [0; XENKBD_FIELD_NUM_CONTACTS - 1] range.
//
// For further information please refer to documentation on Wayland [1],
// Linux [2] and Windows [3] multi-touch support.
//
// [1] https://cgit.freedesktop.org/wayland/wayland/tree/protocol/wayland.xml
// [2] https://www.kernel.org/doc/Documentation/input/multi-touch-protocol.rst
// [3] https://msdn.microsoft.com/en-us/library/jj151564(v=vs.85).aspx
//
// Multi-touch down event - sent when a new touch is made: touch is assigned
// a unique contact ID, sent with this and consequent events related
// to this touch.
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |   _MT_EV_DOWN  |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                               abs_x                               | 12
// +----------------+----------------+----------------+----------------+
// |                               abs_y                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// abs_x - int32_t, absolute X position, in pixels
// abs_y - int32_t, absolute Y position, in pixels
//
// Multi-touch contact release event
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |  _MT_EV_UP     |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// Multi-touch motion event
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |  _MT_EV_MOTION |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                               abs_x                               | 12
// +----------------+----------------+----------------+----------------+
// |                               abs_y                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// abs_x - int32_t, absolute X position, in pixels,
// abs_y - int32_t, absolute Y position, in pixels,
//
// Multi-touch input synchronization event - shows end of a set of events
// which logically belong together.
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |  _MT_EV_SYN    |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// Multi-touch shape event - touch point's shape has changed its shape.
// Shape is approximated by an ellipse through the major and minor axis
// lengths: major is the longer diameter of the ellipse and minor is the
// shorter one. Center of the ellipse is reported via
// XENKBD_MT_EV_DOWN/XENKBD_MT_EV_MOTION events.
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |  _MT_EV_SHAPE  |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                               major                               | 12
// +----------------+----------------+----------------+----------------+
// |                               minor                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// major - unt32_t, length of the major axis, pixels
// minor - unt32_t, length of the minor axis, pixels
//
// Multi-touch orientation event - touch point's shape has changed
// its orientation: calculated as a clockwise angle between the major axis
// of the ellipse and positive Y axis in degrees, [-180; +180].
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |  _TYPE_MTOUCH  |  _MT_EV_ORIENT |   contact_id   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |           orientation           |            reserved             | 12
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 16
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 40
// +----------------+----------------+----------------+----------------+
//
// orientation - int16_t, clockwise angle of the major axis
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_mtouch {
    pub /: *mut *mut uint8_t type; / XENKBD_TYPE_MTOUCH,
    pub /: *mut *mut uint8_t event_type; / XENKBD_MT_EV_???,
    pub contact_id: u8,
    pub /: *mut *mut uint8_t reserved[5]; / reserved for the future use,
    pub /: *mut *mut int32_t abs_x; / absolute X position, pixels,
    pub /: *mut *mut int32_t abs_y; / absolute Y position, pixels,
    pub pos: },
    pub /: *mut *mut uint32_t major; / length of the major axis, pixels,
    pub /: *mut *mut uint32_t minor; / length of the minor axis, pixels,
    pub shape: },
    pub /: *mut *mut int16_t orientation; / clockwise angle of the major axis,
    pub u: },
}

pub const XENKBD_IN_EVENT_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub union xenkbd_in_event {
    pub type: u8,
    pub motion: xenkbd_motion,
    pub key: xenkbd_key,
    pub pos: xenkbd_position,
    pub mtouch: xenkbd_mtouch,
    pub pad: [c_char; XENKBD_IN_EVENT_SIZE],
}

//
// Frontend to backend events
//
// Out events may be sent only when requested by backend, and receipt
// of an unknown out event is an error.
// No out events currently defined.
// All event packets have the same length (40 octets)
// All event packets have common header:
// 0         octet
// +-----------------+
// |       type      |
// +-----------------+
// type - uint8_t, event code
//
pub const XENKBD_OUT_EVENT_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub union xenkbd_out_event {
    pub type: u8,
    pub pad: [c_char; XENKBD_OUT_EVENT_SIZE],
}

//
// Shared page
//
pub const XENKBD_IN_RING_SIZE: c_int = 2048;

pub const XENKBD_IN_RING_OFFS: c_int = 1024;

pub const XENKBD_OUT_RING_SIZE: c_int = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenkbd_page {
    pub in_prod: uint32_t in_cons,,
    pub out_prod: uint32_t out_cons,,
}
