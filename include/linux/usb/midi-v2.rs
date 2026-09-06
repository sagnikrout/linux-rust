//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/midi-v2.h
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
// <linux/usb/midi-v2.h> -- USB MIDI 2.0 definitions.
//

// A.1 MS Class-Specific Interface Descriptor Types
pub const USB_DT_CS_GR_TRM_BLOCK: c_uint = 0x26;
// A.1 MS Class-Specific Interface Descriptor Subtypes
// same as MIDI 1.0
// A.2 MS Class-Specific Endpoint Descriptor Subtypes
pub const USB_MS_GENERAL_2_0: c_uint = 0x02;
// A.3 MS Class-Specific Group Terminal Block Descriptor Subtypes
pub const USB_MS_GR_TRM_BLOCK_UNDEFINED: c_uint = 0x00;
pub const USB_MS_GR_TRM_BLOCK_HEADER: c_uint = 0x01;
pub const USB_MS_GR_TRM_BLOCK: c_uint = 0x02;
// A.4 MS Interface Header MIDIStreaming Class Revision
pub const USB_MS_REV_MIDI_1_0: c_uint = 0x0100;
pub const USB_MS_REV_MIDI_2_0: c_uint = 0x0200;
// A.5 MS MIDI IN and OUT Jack Types
// same as MIDI 1.0
// A.6 Group Terminal Block Types
pub const USB_MS_GR_TRM_BLOCK_TYPE_BIDIRECTIONAL: c_uint = 0x00;
pub const USB_MS_GR_TRM_BLOCK_TYPE_INPUT_ONLY: c_uint = 0x01;
pub const USB_MS_GR_TRM_BLOCK_TYPE_OUTPUT_ONLY: c_uint = 0x02;
// A.7 Group Terminal Default MIDI Protocol
pub const USB_MS_MIDI_PROTO_UNKNOWN: c_uint = 0x00 /* Unknown (Use MIDI-CI) */;
pub const USB_MS_MIDI_PROTO_1_0_64: c_uint = 0x01 /* MIDI 1.0, UMP up to 64bits */;
pub const USB_MS_MIDI_PROTO_1_0_64_JRTS: c_uint = 0x02 /* MIDI 1.0, UMP up to 64bits, Jitter Reduction Timestamps */;
pub const USB_MS_MIDI_PROTO_1_0_128: c_uint = 0x03 /* MIDI 1.0, UMP up to 128bits */;
pub const USB_MS_MIDI_PROTO_1_0_128_JRTS: c_uint = 0x04 /* MIDI 1.0, UMP up to 128bits, Jitter Reduction Timestamps */;
pub const USB_MS_MIDI_PROTO_2_0: c_uint = 0x11 /* MIDI 2.0 */;
pub const USB_MS_MIDI_PROTO_2_0_JRTS: c_uint = 0x12 /* MIDI 2.0, Jitter Reduction Timestamps */;
// 5.2.2.1 Class-Specific MS Interface Header Descriptor
// Same as MIDI 1.0, use struct usb_ms_header_descriptor
// 5.3.2 Class-Specific MIDI Streaming Data Endpoint Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ms20_endpoint_descriptor {
    pub /: *mut *mut __u8 bLength; / 4+n,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_ENDPOINT,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_GENERAL_2_0,
    pub /: *mut *mut __u8 bNumGrpTrmBlock; / Number of Group Terminal Blocks: n,
    pub /: *mut *mut __u8 baAssoGrpTrmBlkID[]; / ID of the Group Terminal Blocks [n],
    pub __packed: },

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bNumGrpTrmBlock;,
    pub \: __u8 baAssoGrpTrmBlkID[n];,
// 5.4.1 Class-Specific Group Terminal Block Header Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ms20_gr_trm_block_header_descriptor {
    pub /: *mut *mut __u8 bLength; / 5,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_GR_TRM_BLOCK,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_GR_TRM_BLOCK_HEADER,
    pub /: *mut *mut __le16 wTotalLength; / Total number of bytes,
    pub __packed: },
// 5.4.2.1 Group Terminal Block Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ms20_gr_trm_block_descriptor {
    pub /: *mut *mut __u8 bLength; / 13,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_GR_TRM_BLOCK,
    pub /: *mut *mut __u8 bDescriptorSubtype; / USB_MS_GR_TRM_BLOCK,
    pub /: *mut *mut __u8 bGrpTrmBlkID; / ID of this Group Terminal Block,
    pub /: *mut *mut __u8 bGrpTrmBlkType; / Group Terminal Block Type,
    pub /: *mut *mut __u8 nGroupTrm; / The first member Group Terminal in this block,
    pub /: *mut *mut __u8 nNumGroupTrm; / Number of member Group Terminals spanned,
    pub /: *mut *mut __u8 iBlockItem; / String ID of Block item,
    pub /: *mut *mut __u8 bMIDIProtocol; / Default MIDI protocol,
    pub /: *mut *mut __le16 wMaxInputBandwidth; / Max input bandwidth capability in 4kB/s,
    pub /: *mut *mut __le16 wMaxOutputBandwidth; / Max output bandwidth capability in 4kB/s,
    pub __packed: },
