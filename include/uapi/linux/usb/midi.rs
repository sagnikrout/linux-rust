//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/midi.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// <linux/usb/midi.h> -- USB MIDI definitions.
//
// Copyright (C) 2006 Thumtronics Pty Ltd.
// Developed for Thumtronics by Grey Innovation
// Ben Williamson <ben.williamson@greyinnovation.com>
//
// This software is distributed under the terms of the GNU General Public
// License ("GPL") version 2, as published by the Free Software Foundation.
//
// This file holds USB constants and structures defined
// by the USB Device Class Definition for MIDI Devices.
// Comments below reference relevant sections of that document:
//
// http://www.usb.org/developers/devclass_docs/midi10.pdf
//

// A.1  MS Class-Specific Interface Descriptor Subtypes
pub const USB_MS_HEADER: c_uint = 0x01;
pub const USB_MS_MIDI_IN_JACK: c_uint = 0x02;
pub const USB_MS_MIDI_OUT_JACK: c_uint = 0x03;
pub const USB_MS_ELEMENT: c_uint = 0x04;
// A.2  MS Class-Specific Endpoint Descriptor Subtypes
pub const USB_MS_GENERAL: c_uint = 0x01;
// A.3  MS MIDI IN and OUT Jack Types
pub const USB_MS_EMBEDDED: c_uint = 0x01;
pub const USB_MS_EXTERNAL: c_uint = 0x02;
// 6.1.2.1  Class-Specific MS Interface Header Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ms_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bcdMSC: __le16,
    pub wTotalLength: __le16,
// C attribute field omitted
pub const USB_DT_MS_HEADER_SIZE: c_int = 7;
// 6.1.2.2  MIDI IN Jack Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_midi_in_jack_descriptor {
    pub bLength: __u8,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_MIDI_IN_JACK,
    pub /: *mut *mut __u8 bJackType; / USB_MS_EMBEDDED/EXTERNAL,
    pub bJackID: __u8,
    pub iJack: __u8,
// C attribute field omitted
pub const USB_DT_MIDI_IN_SIZE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_midi_source_pin {
    pub baSourceID: __u8,
    pub baSourcePin: __u8,
// C attribute field omitted
// 6.1.2.3  MIDI OUT Jack Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_midi_out_jack_descriptor {
    pub bLength: __u8,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_MIDI_OUT_JACK,
    pub /: *mut *mut __u8 bJackType; / USB_MS_EMBEDDED/EXTERNAL,
    pub bJackID: __u8,
    pub /: *mut *mut __u8 bNrInputPins; / p,
    pub /: *mut *mut usb_midi_source_pin pins[]; / [p],
// __u8  iJack;  -- omitted due to variable-sized pins[]
// C attribute field omitted

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bJackType;,
    pub \: __u8 bJackID;,
    pub \: __u8 bNrInputPins;,
    pub \: usb_midi_source_pin pins[p];,
    pub \: __u8 iJack;,
// 6.2.2  Class-Specific MS Bulk Data Endpoint Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ms_endpoint_descriptor {
    pub /: *mut *mut __u8 bLength; / 4+n,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_ENDPOINT,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_GENERAL,
    pub /: *mut *mut __u8 bNumEmbMIDIJack; / n,
    pub /: *mut *mut __u8 baAssocJackID[]; / [n],
// C attribute field omitted

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bNumEmbMIDIJack;,
    pub \: __u8 baAssocJackID[n];,
