//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/audio-v3.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2017 Ruslan Bilovol <ruslan.bilovol@gmail.com>
//
// This file holds USB constants and structures defined
// by the USB DEVICE CLASS DEFINITION FOR AUDIO DEVICES Release 3.0.
//

//
// v1.0, v2.0 and v3.0 of this standard have many things in common. For the rest
// of the definitions, please refer to audio.h and audio-v2.h
//
// All High Capability descriptors have these 2 fields at the beginning
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_hc_descriptor_header {
    pub wLength: __le16,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub wDescriptorID: __le16,
// C attribute field omitted
// 4.3.1 CLUSTER DESCRIPTOR HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_cluster_header_descriptor {
    pub wLength: __le16,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub wDescriptorID: __le16,
    pub bNrChannels: __u8,
// C attribute field omitted
// 4.3.2.1 SEGMENTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_cluster_segment_descriptor {
    pub wLength: __le16,
    pub bSegmentType: __u8,
// __u8[0]; segment-specific data
// C attribute field omitted
// 4.3.2.1.1 END SEGMENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_cluster_end_segment_descriptor {
    pub wLength: __le16,
    pub /: *mut *mut __u8 bSegmentType; / Constant END_SEGMENT,
// C attribute field omitted
// 4.3.2.1.3.1 INFORMATION SEGMENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_cluster_information_segment_descriptor {
    pub wLength: __le16,
    pub bSegmentType: __u8,
    pub bChPurpose: __u8,
    pub bChRelationship: __u8,
    pub bChGroupID: __u8,
// C attribute field omitted
// 4.5.2 CLASS-SPECIFIC AC INTERFACE DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_ac_header_descriptor {
    pub /: *mut *mut __u8 bLength; / 10,
    pub /: *mut *mut __u8 bDescriptorType; / CS_INTERFACE descriptor type,
    pub /: *mut *mut __u8 bDescriptorSubtype; / HEADER descriptor subtype,
    pub bCategory: __u8,
// includes Clock Source, Unit, Terminal, and Power Domain desc.
    pub wTotalLength: __le16,
    pub bmControls: __le32,
// C attribute field omitted
// 4.5.2.1 INPUT TERMINAL DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_input_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub bCSourceID: __u8,
    pub bmControls: __le32,
    pub wClusterDescrID: __le16,
    pub wExTerminalDescrID: __le16,
    pub wConnectorsDescrID: __le16,
    pub wTerminalDescrStr: __le16,
    pub __attribute__((packed)): },
// 4.5.2.2 OUTPUT TERMINAL DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_output_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub bSourceID: __u8,
    pub bCSourceID: __u8,
    pub bmControls: __le32,
    pub wExTerminalDescrID: __le16,
    pub wConnectorsDescrID: __le16,
    pub wTerminalDescrStr: __le16,
    pub __attribute__((packed)): },
// 4.5.2.7 FEATURE UNIT DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_feature_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub bSourceID: __u8,
// bmaControls is actually u32,
// but u8 is needed for the hybrid parser
    pub /: *mut *mut __u8 bmaControls[]; / variable length,
// wFeatureDescrStr omitted
    pub __attribute__((packed)): },

// As above, but more useful for defining your own descriptors

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bUnitID;,
    pub \: __u8 bSourceID;,
    pub \: __le32 bmaControls[ch + 1];,
    pub \: __le16 wFeatureDescrStr;,
// 4.5.2.12 CLOCK SOURCE DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_clock_source_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bmAttributes: __u8,
    pub bmControls: __le32,
    pub bReferenceTerminal: __u8,
    pub wClockSourceStr: __le16,
    pub __attribute__((packed)): },
// bmAttribute fields
pub const UAC3_CLOCK_SOURCE_TYPE_EXT: c_uint = 0x0;
pub const UAC3_CLOCK_SOURCE_TYPE_INT: c_uint = 0x1;

// 4.5.2.13 CLOCK SELECTOR DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_clock_selector_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bNrInPins: __u8,
    pub baCSourceID: [__u8; ],
// bmControls and wCSelectorDescrStr omitted
    pub __attribute__((packed)): },
// 4.5.2.14 CLOCK MULTIPLIER DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_clock_multiplier_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bCSourceID: __u8,
    pub bmControls: __le32,
    pub wCMultiplierDescrStr: __le16,
    pub __attribute__((packed)): },
// 4.5.2.15 POWER DOMAIN DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_power_domain_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bPowerDomainID: __u8,
    pub waRecoveryTime1: __le16,
    pub waRecoveryTime2: __le16,
    pub bNrEntities: __u8,
    pub baEntityID: [__u8; ],
// wPDomainDescrStr omitted
    pub __attribute__((packed)): },
// As above, but more useful for defining your own descriptors

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bPowerDomainID;,
    pub \: __le16 waRecoveryTime1;,
    pub \: __le16 waRecoveryTime2;,
    pub \: __u8 bNrEntities;,
    pub \: __u8 baEntityID[n];,
    pub \: __le16 wPDomainDescrStr;,
// 4.7.2 CLASS-SPECIFIC AS INTERFACE DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_as_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalLink: __u8,
    pub bmControls: __le32,
    pub wClusterDescrID: __le16,
    pub bmFormats: __le64,
    pub bSubslotSize: __u8,
    pub bBitResolution: __u8,
    pub bmAuxProtocols: __le16,
    pub bControlSize: __u8,
    pub __attribute__((packed)): },

// 4.8.1.2 CLASS-SPECIFIC AS ISOCHRONOUS AUDIO DATA ENDPOINT DESCRIPTOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_iso_endpoint_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bmControls: __le32,
    pub bLockDelayUnits: __u8,
    pub wLockDelay: __le16,
    pub __attribute__((packed)): },
// 5.2.1.6.1 INSERTION CONTROL PARAMETER BLOCK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_insertion_ctl_blk {
    pub bSize: __u8,
    pub bmConInserted: __u8,
// C attribute field omitted
// 6.1 INTERRUPT DATA MESSAGE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac3_interrupt_data_msg {
    pub bInfo: __u8,
    pub bSourceType: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub __attribute__((packed)): },
// A.2 AUDIO AUDIO FUNCTION SUBCLASS CODES
pub const UAC3_FUNCTION_SUBCLASS_UNDEFINED: c_uint = 0x00;
pub const UAC3_FUNCTION_SUBCLASS_FULL_ADC_3_0: c_uint = 0x01;
// BADD profiles
pub const UAC3_FUNCTION_SUBCLASS_GENERIC_IO: c_uint = 0x20;
pub const UAC3_FUNCTION_SUBCLASS_HEADPHONE: c_uint = 0x21;
pub const UAC3_FUNCTION_SUBCLASS_SPEAKER: c_uint = 0x22;
pub const UAC3_FUNCTION_SUBCLASS_MICROPHONE: c_uint = 0x23;
pub const UAC3_FUNCTION_SUBCLASS_HEADSET: c_uint = 0x24;
pub const UAC3_FUNCTION_SUBCLASS_HEADSET_ADAPTER: c_uint = 0x25;
pub const UAC3_FUNCTION_SUBCLASS_SPEAKERPHONE: c_uint = 0x26;
// A.7 AUDIO FUNCTION CATEGORY CODES
pub const UAC3_FUNCTION_SUBCLASS_UNDEFINED: c_uint = 0x00;
pub const UAC3_FUNCTION_DESKTOP_SPEAKER: c_uint = 0x01;
pub const UAC3_FUNCTION_HOME_THEATER: c_uint = 0x02;
pub const UAC3_FUNCTION_MICROPHONE: c_uint = 0x03;
pub const UAC3_FUNCTION_HEADSET: c_uint = 0x04;
pub const UAC3_FUNCTION_TELEPHONE: c_uint = 0x05;
pub const UAC3_FUNCTION_CONVERTER: c_uint = 0x06;
pub const UAC3_FUNCTION_SOUND_RECORDER: c_uint = 0x07;
pub const UAC3_FUNCTION_IO_BOX: c_uint = 0x08;
pub const UAC3_FUNCTION_MUSICAL_INSTRUMENT: c_uint = 0x09;
pub const UAC3_FUNCTION_PRO_AUDIO: c_uint = 0x0a;
pub const UAC3_FUNCTION_AUDIO_VIDEO: c_uint = 0x0b;
pub const UAC3_FUNCTION_CONTROL_PANEL: c_uint = 0x0c;
pub const UAC3_FUNCTION_HEADPHONE: c_uint = 0x0d;
pub const UAC3_FUNCTION_GENERIC_SPEAKER: c_uint = 0x0e;
pub const UAC3_FUNCTION_HEADSET_ADAPTER: c_uint = 0x0f;
pub const UAC3_FUNCTION_SPEAKERPHONE: c_uint = 0x10;
pub const UAC3_FUNCTION_OTHER: c_uint = 0xff;
// A.8 AUDIO CLASS-SPECIFIC DESCRIPTOR TYPES
pub const UAC3_CS_UNDEFINED: c_uint = 0x20;
pub const UAC3_CS_DEVICE: c_uint = 0x21;
pub const UAC3_CS_CONFIGURATION: c_uint = 0x22;
pub const UAC3_CS_STRING: c_uint = 0x23;
pub const UAC3_CS_INTERFACE: c_uint = 0x24;
pub const UAC3_CS_ENDPOINT: c_uint = 0x25;
pub const UAC3_CS_CLUSTER: c_uint = 0x26;
// A.10 CLUSTER DESCRIPTOR SEGMENT TYPES
pub const UAC3_SEGMENT_UNDEFINED: c_uint = 0x00;
pub const UAC3_CLUSTER_DESCRIPTION: c_uint = 0x01;
pub const UAC3_CLUSTER_VENDOR_DEFINED: c_uint = 0x1F;
pub const UAC3_CHANNEL_INFORMATION: c_uint = 0x20;
pub const UAC3_CHANNEL_AMBISONIC: c_uint = 0x21;
pub const UAC3_CHANNEL_DESCRIPTION: c_uint = 0x22;
pub const UAC3_CHANNEL_VENDOR_DEFINED: c_uint = 0xFE;
pub const UAC3_END_SEGMENT: c_uint = 0xFF;
// A.11 CHANNEL PURPOSE DEFINITIONS
pub const UAC3_PURPOSE_UNDEFINED: c_uint = 0x00;
pub const UAC3_PURPOSE_GENERIC_AUDIO: c_uint = 0x01;
pub const UAC3_PURPOSE_VOICE: c_uint = 0x02;
pub const UAC3_PURPOSE_SPEECH: c_uint = 0x03;
pub const UAC3_PURPOSE_AMBIENT: c_uint = 0x04;
pub const UAC3_PURPOSE_REFERENCE: c_uint = 0x05;
pub const UAC3_PURPOSE_ULTRASONIC: c_uint = 0x06;
pub const UAC3_PURPOSE_VIBROKINETIC: c_uint = 0x07;
pub const UAC3_PURPOSE_NON_AUDIO: c_uint = 0xFF;
// A.12 CHANNEL RELATIONSHIP DEFINITIONS
pub const UAC3_CH_RELATIONSHIP_UNDEFINED: c_uint = 0x00;
pub const UAC3_CH_MONO: c_uint = 0x01;
pub const UAC3_CH_LEFT: c_uint = 0x02;
pub const UAC3_CH_RIGHT: c_uint = 0x03;
pub const UAC3_CH_ARRAY: c_uint = 0x04;
pub const UAC3_CH_PATTERN_X: c_uint = 0x20;
pub const UAC3_CH_PATTERN_Y: c_uint = 0x21;
pub const UAC3_CH_PATTERN_A: c_uint = 0x22;
pub const UAC3_CH_PATTERN_B: c_uint = 0x23;
pub const UAC3_CH_PATTERN_M: c_uint = 0x24;
pub const UAC3_CH_PATTERN_S: c_uint = 0x25;
pub const UAC3_CH_FRONT_LEFT: c_uint = 0x80;
pub const UAC3_CH_FRONT_RIGHT: c_uint = 0x81;
pub const UAC3_CH_FRONT_CENTER: c_uint = 0x82;
pub const UAC3_CH_FRONT_LEFT_OF_CENTER: c_uint = 0x83;
pub const UAC3_CH_FRONT_RIGHT_OF_CENTER: c_uint = 0x84;
pub const UAC3_CH_FRONT_WIDE_LEFT: c_uint = 0x85;
pub const UAC3_CH_FRONT_WIDE_RIGHT: c_uint = 0x86;
pub const UAC3_CH_SIDE_LEFT: c_uint = 0x87;
pub const UAC3_CH_SIDE_RIGHT: c_uint = 0x88;
pub const UAC3_CH_SURROUND_ARRAY_LEFT: c_uint = 0x89;
pub const UAC3_CH_SURROUND_ARRAY_RIGHT: c_uint = 0x8A;
pub const UAC3_CH_BACK_LEFT: c_uint = 0x8B;
pub const UAC3_CH_BACK_RIGHT: c_uint = 0x8C;
pub const UAC3_CH_BACK_CENTER: c_uint = 0x8D;
pub const UAC3_CH_BACK_LEFT_OF_CENTER: c_uint = 0x8E;
pub const UAC3_CH_BACK_RIGHT_OF_CENTER: c_uint = 0x8F;
pub const UAC3_CH_BACK_WIDE_LEFT: c_uint = 0x90;
pub const UAC3_CH_BACK_WIDE_RIGHT: c_uint = 0x91;
pub const UAC3_CH_TOP_CENTER: c_uint = 0x92;
pub const UAC3_CH_TOP_FRONT_LEFT: c_uint = 0x93;
pub const UAC3_CH_TOP_FRONT_RIGHT: c_uint = 0x94;
pub const UAC3_CH_TOP_FRONT_CENTER: c_uint = 0x95;
pub const UAC3_CH_TOP_FRONT_LOC: c_uint = 0x96;
pub const UAC3_CH_TOP_FRONT_ROC: c_uint = 0x97;
pub const UAC3_CH_TOP_FRONT_WIDE_LEFT: c_uint = 0x98;
pub const UAC3_CH_TOP_FRONT_WIDE_RIGHT: c_uint = 0x99;
pub const UAC3_CH_TOP_SIDE_LEFT: c_uint = 0x9A;
pub const UAC3_CH_TOP_SIDE_RIGHT: c_uint = 0x9B;
pub const UAC3_CH_TOP_SURR_ARRAY_LEFT: c_uint = 0x9C;
pub const UAC3_CH_TOP_SURR_ARRAY_RIGHT: c_uint = 0x9D;
pub const UAC3_CH_TOP_BACK_LEFT: c_uint = 0x9E;
pub const UAC3_CH_TOP_BACK_RIGHT: c_uint = 0x9F;
pub const UAC3_CH_TOP_BACK_CENTER: c_uint = 0xA0;
pub const UAC3_CH_TOP_BACK_LOC: c_uint = 0xA1;
pub const UAC3_CH_TOP_BACK_ROC: c_uint = 0xA2;
pub const UAC3_CH_TOP_BACK_WIDE_LEFT: c_uint = 0xA3;
pub const UAC3_CH_TOP_BACK_WIDE_RIGHT: c_uint = 0xA4;
pub const UAC3_CH_BOTTOM_CENTER: c_uint = 0xA5;
pub const UAC3_CH_BOTTOM_FRONT_LEFT: c_uint = 0xA6;
pub const UAC3_CH_BOTTOM_FRONT_RIGHT: c_uint = 0xA7;
pub const UAC3_CH_BOTTOM_FRONT_CENTER: c_uint = 0xA8;
pub const UAC3_CH_BOTTOM_FRONT_LOC: c_uint = 0xA9;
pub const UAC3_CH_BOTTOM_FRONT_ROC: c_uint = 0xAA;
pub const UAC3_CH_BOTTOM_FRONT_WIDE_LEFT: c_uint = 0xAB;
pub const UAC3_CH_BOTTOM_FRONT_WIDE_RIGHT: c_uint = 0xAC;
pub const UAC3_CH_BOTTOM_SIDE_LEFT: c_uint = 0xAD;
pub const UAC3_CH_BOTTOM_SIDE_RIGHT: c_uint = 0xAE;
pub const UAC3_CH_BOTTOM_SURR_ARRAY_LEFT: c_uint = 0xAF;
pub const UAC3_CH_BOTTOM_SURR_ARRAY_RIGHT: c_uint = 0xB0;
pub const UAC3_CH_BOTTOM_BACK_LEFT: c_uint = 0xB1;
pub const UAC3_CH_BOTTOM_BACK_RIGHT: c_uint = 0xB2;
pub const UAC3_CH_BOTTOM_BACK_CENTER: c_uint = 0xB3;
pub const UAC3_CH_BOTTOM_BACK_LOC: c_uint = 0xB4;
pub const UAC3_CH_BOTTOM_BACK_ROC: c_uint = 0xB5;
pub const UAC3_CH_BOTTOM_BACK_WIDE_LEFT: c_uint = 0xB6;
pub const UAC3_CH_BOTTOM_BACK_WIDE_RIGHT: c_uint = 0xB7;
pub const UAC3_CH_LOW_FREQUENCY_EFFECTS: c_uint = 0xB8;
pub const UAC3_CH_LFE_LEFT: c_uint = 0xB9;
pub const UAC3_CH_LFE_RIGHT: c_uint = 0xBA;
pub const UAC3_CH_HEADPHONE_LEFT: c_uint = 0xBB;
pub const UAC3_CH_HEADPHONE_RIGHT: c_uint = 0xBC;
// A.15 AUDIO CLASS-SPECIFIC AC INTERFACE DESCRIPTOR SUBTYPES
// see audio.h for the rest, which is identical to v1
pub const UAC3_EXTENDED_TERMINAL: c_uint = 0x04;
pub const UAC3_MIXER_UNIT: c_uint = 0x05;
pub const UAC3_SELECTOR_UNIT: c_uint = 0x06;
pub const UAC3_FEATURE_UNIT: c_uint = 0x07;
pub const UAC3_EFFECT_UNIT: c_uint = 0x08;
pub const UAC3_PROCESSING_UNIT: c_uint = 0x09;
pub const UAC3_EXTENSION_UNIT: c_uint = 0x0a;
pub const UAC3_CLOCK_SOURCE: c_uint = 0x0b;
pub const UAC3_CLOCK_SELECTOR: c_uint = 0x0c;
pub const UAC3_CLOCK_MULTIPLIER: c_uint = 0x0d;
pub const UAC3_SAMPLE_RATE_CONVERTER: c_uint = 0x0e;
pub const UAC3_CONNECTORS: c_uint = 0x0f;
pub const UAC3_POWER_DOMAIN: c_uint = 0x10;
// A.20 PROCESSING UNIT PROCESS TYPES
pub const UAC3_PROCESS_UNDEFINED: c_uint = 0x00;
pub const UAC3_PROCESS_UP_DOWNMIX: c_uint = 0x01;
pub const UAC3_PROCESS_STEREO_EXTENDER: c_uint = 0x02;
pub const UAC3_PROCESS_MULTI_FUNCTION: c_uint = 0x03;
// A.22 AUDIO CLASS-SPECIFIC REQUEST CODES
// see audio-v2.h for the rest, which is identical to v2
pub const UAC3_CS_REQ_INTEN: c_uint = 0x04;
pub const UAC3_CS_REQ_STRING: c_uint = 0x05;
pub const UAC3_CS_REQ_HIGH_CAPABILITY_DESCRIPTOR: c_uint = 0x06;
// A.23.1 AUDIOCONTROL INTERFACE CONTROL SELECTORS
pub const UAC3_AC_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UAC3_AC_ACTIVE_INTERFACE_CONTROL: c_uint = 0x01;
pub const UAC3_AC_POWER_DOMAIN_CONTROL: c_uint = 0x02;
// A.23.5 TERMINAL CONTROL SELECTORS
pub const UAC3_TE_UNDEFINED: c_uint = 0x00;
pub const UAC3_TE_INSERTION: c_uint = 0x01;
pub const UAC3_TE_OVERLOAD: c_uint = 0x02;
pub const UAC3_TE_UNDERFLOW: c_uint = 0x03;
pub const UAC3_TE_OVERFLOW: c_uint = 0x04;
pub const UAC3_TE_LATENCY: c_uint = 0x05;
// A.23.10 PROCESSING UNITS CONTROL SELECTROS
// Up/Down Mixer
pub const UAC3_UD_MODE_SELECT: c_uint = 0x01;
// Stereo Extender
pub const UAC3_EXT_WIDTH_CONTROL: c_uint = 0x01;
// BADD predefined Unit/Terminal values

// BADD wMaxPacketSize of AS endpoints
pub const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_16: c_uint = 0x0060;
pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_16: c_uint = 0x0062;
pub const UAC3_BADD_EP_MAXPSIZE_SYNC_MONO_24: c_uint = 0x0090;
pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_MONO_24: c_uint = 0x0093;
pub const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_16: c_uint = 0x00C0;
pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_16: c_uint = 0x00C4;
pub const UAC3_BADD_EP_MAXPSIZE_SYNC_STEREO_24: c_uint = 0x0120;
pub const UAC3_BADD_EP_MAXPSIZE_ASYNC_STEREO_24: c_uint = 0x0126;
// BADD sample rate is always fixed to 48kHz
pub const UAC3_BADD_SAMPLING_RATE: c_int = 48000;
// BADD power domains recovery times in 50us increments
pub const UAC3_BADD_PD_RECOVER_D1D0: c_uint = 0x0258	/* 30ms */;
pub const UAC3_BADD_PD_RECOVER_D2D0: c_uint = 0x1770	/* 300ms */;
