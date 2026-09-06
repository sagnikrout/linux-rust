//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/audio.h
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
// <linux/usb/audio.h> -- USB Audio definitions.
//
// Copyright (C) 2006 Thumtronics Pty Ltd.
// Developed for Thumtronics by Grey Innovation
// Ben Williamson <ben.williamson@greyinnovation.com>
//
// This software is distributed under the terms of the GNU General Public
// License ("GPL") version 2, as published by the Free Software Foundation.
//
// This file holds USB constants and structures defined
// by the USB Device Class Definition for Audio Devices.
// Comments below reference relevant sections of that document:
//
// http://www.usb.org/developers/devclass_docs/audio10.pdf
//
// Types and defines in this file are either specific to version 1.0 of
// this standard or common for newer versions.
//

// bInterfaceProtocol values to denote the version of the standard used
pub const UAC_VERSION_1: c_uint = 0x00;
pub const UAC_VERSION_2: c_uint = 0x20;
pub const UAC_VERSION_3: c_uint = 0x30;
// A.2 Audio Interface Subclass Codes
pub const USB_SUBCLASS_AUDIOCONTROL: c_uint = 0x01;
pub const USB_SUBCLASS_AUDIOSTREAMING: c_uint = 0x02;
pub const USB_SUBCLASS_MIDISTREAMING: c_uint = 0x03;
// A.5 Audio Class-Specific AC Interface Descriptor Subtypes
pub const UAC_HEADER: c_uint = 0x01;
pub const UAC_INPUT_TERMINAL: c_uint = 0x02;
pub const UAC_OUTPUT_TERMINAL: c_uint = 0x03;
pub const UAC_MIXER_UNIT: c_uint = 0x04;
pub const UAC_SELECTOR_UNIT: c_uint = 0x05;
pub const UAC_FEATURE_UNIT: c_uint = 0x06;
pub const UAC1_PROCESSING_UNIT: c_uint = 0x07;
pub const UAC1_EXTENSION_UNIT: c_uint = 0x08;
// A.6 Audio Class-Specific AS Interface Descriptor Subtypes
pub const UAC_AS_GENERAL: c_uint = 0x01;
pub const UAC_FORMAT_TYPE: c_uint = 0x02;
pub const UAC_FORMAT_SPECIFIC: c_uint = 0x03;
// A.7 Processing Unit Process Types
pub const UAC_PROCESS_UNDEFINED: c_uint = 0x00;
pub const UAC_PROCESS_UP_DOWNMIX: c_uint = 0x01;
pub const UAC_PROCESS_DOLBY_PROLOGIC: c_uint = 0x02;
pub const UAC_PROCESS_STEREO_EXTENDER: c_uint = 0x03;
pub const UAC_PROCESS_REVERB: c_uint = 0x04;
pub const UAC_PROCESS_CHORUS: c_uint = 0x05;
pub const UAC_PROCESS_DYN_RANGE_COMP: c_uint = 0x06;
// A.8 Audio Class-Specific Endpoint Descriptor Subtypes
pub const UAC_EP_GENERAL: c_uint = 0x01;
// A.9 Audio Class-Specific Request Codes
pub const UAC_SET_: c_uint = 0x00;
pub const UAC_GET_: c_uint = 0x80;
pub const UAC__CUR: c_uint = 0x1;
pub const UAC__MIN: c_uint = 0x2;
pub const UAC__MAX: c_uint = 0x3;
pub const UAC__RES: c_uint = 0x4;
pub const UAC__MEM: c_uint = 0x5;

pub const UAC_GET_STAT: c_uint = 0xff;
// A.10 Control Selector Codes
// A.10.1 Terminal Control Selectors
pub const UAC_TERM_COPY_PROTECT: c_uint = 0x01;
// A.10.2 Feature Unit Control Selectors
pub const UAC_FU_MUTE: c_uint = 0x01;
pub const UAC_FU_VOLUME: c_uint = 0x02;
pub const UAC_FU_BASS: c_uint = 0x03;
pub const UAC_FU_MID: c_uint = 0x04;
pub const UAC_FU_TREBLE: c_uint = 0x05;
pub const UAC_FU_GRAPHIC_EQUALIZER: c_uint = 0x06;
pub const UAC_FU_AUTOMATIC_GAIN: c_uint = 0x07;
pub const UAC_FU_DELAY: c_uint = 0x08;
pub const UAC_FU_BASS_BOOST: c_uint = 0x09;
pub const UAC_FU_LOUDNESS: c_uint = 0x0a;

// A.10.3.1 Up/Down-mix Processing Unit Controls Selectors
pub const UAC_UD_ENABLE: c_uint = 0x01;
pub const UAC_UD_MODE_SELECT: c_uint = 0x02;
// A.10.3.2 Dolby Prologic (tm) Processing Unit Controls Selectors
pub const UAC_DP_ENABLE: c_uint = 0x01;
pub const UAC_DP_MODE_SELECT: c_uint = 0x02;
// A.10.3.3 3D Stereo Extender Processing Unit Control Selectors
pub const UAC_3D_ENABLE: c_uint = 0x01;
pub const UAC_3D_SPACE: c_uint = 0x02;
// A.10.3.4 Reverberation Processing Unit Control Selectors
pub const UAC_REVERB_ENABLE: c_uint = 0x01;
pub const UAC_REVERB_LEVEL: c_uint = 0x02;
pub const UAC_REVERB_TIME: c_uint = 0x03;
pub const UAC_REVERB_FEEDBACK: c_uint = 0x04;
// A.10.3.5 Chorus Processing Unit Control Selectors
pub const UAC_CHORUS_ENABLE: c_uint = 0x01;
pub const UAC_CHORUS_LEVEL: c_uint = 0x02;
pub const UAC_CHORUS_RATE: c_uint = 0x03;
pub const UAC_CHORUS_DEPTH: c_uint = 0x04;
// A.10.3.6 Dynamic Range Compressor Unit Control Selectors
pub const UAC_DCR_ENABLE: c_uint = 0x01;
pub const UAC_DCR_RATE: c_uint = 0x02;
pub const UAC_DCR_MAXAMPL: c_uint = 0x03;
pub const UAC_DCR_THRESHOLD: c_uint = 0x04;
pub const UAC_DCR_ATTACK_TIME: c_uint = 0x05;
pub const UAC_DCR_RELEASE_TIME: c_uint = 0x06;
// A.10.4 Extension Unit Control Selectors
pub const UAC_XU_ENABLE: c_uint = 0x01;
// MIDI - A.1 MS Class-Specific Interface Descriptor Subtypes
pub const UAC_MS_HEADER: c_uint = 0x01;
pub const UAC_MIDI_IN_JACK: c_uint = 0x02;
pub const UAC_MIDI_OUT_JACK: c_uint = 0x03;
// MIDI - A.1 MS Class-Specific Endpoint Descriptor Subtypes
pub const UAC_MS_GENERAL: c_uint = 0x01;
// Terminals - 2.1 USB Terminal Types
pub const UAC_TERMINAL_UNDEFINED: c_uint = 0x100;
pub const UAC_TERMINAL_STREAMING: c_uint = 0x101;
pub const UAC_TERMINAL_VENDOR_SPEC: c_uint = 0x1FF;
// Terminal Control Selectors
// 4.3.2  Class-Specific AC Interface Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac1_ac_header_descriptor {
    pub /: *mut *mut __u8 bLength; / 8 + n,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / UAC_MS_HEADER,
    pub /: *mut *mut __le16 bcdADC; / 0x0100,
    pub /: *mut *mut __le16 wTotalLength; / includes Unit and Terminal desc.,
    pub /: *mut *mut __u8 bInCollection; / n,
    pub /: *mut *mut __u8 baInterfaceNr[]; / [n],
// C attribute field omitted

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __le16 bcdADC;,
    pub \: __le16 wTotalLength;,
    pub \: __u8 bInCollection;,
    pub \: __u8 baInterfaceNr[n];,
// 4.3.2.1 Input Terminal Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_input_terminal_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 12,
    pub /: *mut *mut __u8 bDescriptorType; / CS_INTERFACE descriptor type,
    pub /: *mut *mut __u8 bDescriptorSubtype; / INPUT_TERMINAL descriptor subtype,
    pub /: *mut *mut __u8 bTerminalID; / Constant uniquely terminal ID,
    pub /: *mut *mut __le16 wTerminalType; / USB Audio Terminal Types,
    pub /: *mut *mut __u8 bAssocTerminal; / ID of the Output Terminal associated,
    pub /: *mut *mut __u8 bNrChannels; / Number of logical output channels,
    pub wChannelConfig: __le16,
    pub iChannelNames: __u8,
    pub iTerminal: __u8,
// C attribute field omitted
pub const UAC_DT_INPUT_TERMINAL_SIZE: c_int = 12;
// Terminals - 2.2 Input Terminal Types
pub const UAC_INPUT_TERMINAL_UNDEFINED: c_uint = 0x200;
pub const UAC_INPUT_TERMINAL_MICROPHONE: c_uint = 0x201;
pub const UAC_INPUT_TERMINAL_DESKTOP_MICROPHONE: c_uint = 0x202;
pub const UAC_INPUT_TERMINAL_PERSONAL_MICROPHONE: c_uint = 0x203;
pub const UAC_INPUT_TERMINAL_OMNI_DIR_MICROPHONE: c_uint = 0x204;
pub const UAC_INPUT_TERMINAL_MICROPHONE_ARRAY: c_uint = 0x205;
pub const UAC_INPUT_TERMINAL_PROC_MICROPHONE_ARRAY: c_uint = 0x206;
// Terminals - control selectors
pub const UAC_TERMINAL_CS_COPY_PROTECT_CONTROL: c_uint = 0x01;
// 4.3.2.2 Output Terminal Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac1_output_terminal_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 9,
    pub /: *mut *mut __u8 bDescriptorType; / CS_INTERFACE descriptor type,
    pub /: *mut *mut __u8 bDescriptorSubtype; / OUTPUT_TERMINAL descriptor subtype,
    pub /: *mut *mut __u8 bTerminalID; / Constant uniquely terminal ID,
    pub /: *mut *mut __le16 wTerminalType; / USB Audio Terminal Types,
    pub /: *mut *mut __u8 bAssocTerminal; / ID of the Input Terminal associated,
    pub Terminal*/: *mut *mut __u8 bSourceID; / ID of the connected Unit or,
    pub iTerminal: __u8,
// C attribute field omitted
pub const UAC_DT_OUTPUT_TERMINAL_SIZE: c_int = 9;
// Terminals - 2.3 Output Terminal Types
pub const UAC_OUTPUT_TERMINAL_UNDEFINED: c_uint = 0x300;
pub const UAC_OUTPUT_TERMINAL_SPEAKER: c_uint = 0x301;
pub const UAC_OUTPUT_TERMINAL_HEADPHONES: c_uint = 0x302;
pub const UAC_OUTPUT_TERMINAL_HEAD_MOUNTED_DISPLAY_AUDIO: c_uint = 0x303;
pub const UAC_OUTPUT_TERMINAL_DESKTOP_SPEAKER: c_uint = 0x304;
pub const UAC_OUTPUT_TERMINAL_ROOM_SPEAKER: c_uint = 0x305;
pub const UAC_OUTPUT_TERMINAL_COMMUNICATION_SPEAKER: c_uint = 0x306;
pub const UAC_OUTPUT_TERMINAL_LOW_FREQ_EFFECTS_SPEAKER: c_uint = 0x307;
// Terminals - 2.4 Bi-directional Terminal Types
pub const UAC_BIDIR_TERMINAL_UNDEFINED: c_uint = 0x400;
pub const UAC_BIDIR_TERMINAL_HANDSET: c_uint = 0x401;
pub const UAC_BIDIR_TERMINAL_HEADSET: c_uint = 0x402;
pub const UAC_BIDIR_TERMINAL_SPEAKER_PHONE: c_uint = 0x403;
pub const UAC_BIDIR_TERMINAL_ECHO_SUPPRESSING: c_uint = 0x404;
pub const UAC_BIDIR_TERMINAL_ECHO_CANCELING: c_uint = 0x405;
// Set bControlSize = 2 as default setting

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bUnitID;,
    pub \: __u8 bSourceID;,
    pub \: __u8 bControlSize;,
    pub \: __le16 bmaControls[ch + 1];,
    pub \: __u8 iFeature;,
// 4.3.2.3 Mixer Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_mixer_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub bNrInPins: __u8,
    pub baSourceID: [__u8; ],
// C attribute field omitted
    pub desc->baSourceID[desc->bNrInPins]: return,
    pub 1]: desc->baSourceID[desc->bNrInPins +,
    pub 1]): (desc->baSourceID[desc->bNrInPins +,
    pub 5]: desc->baSourceID[desc->bNrInPins +,
    pub 4]: return &desc->baSourceID[desc->bNrInPins +,
    pub 6]: return &desc->baSourceID[desc->bNrInPins +,
    pub 2]: return &desc->baSourceID[desc->bNrInPins +,
    pub NULL: return,
    pub desc: *mut *mut *mut __u8 raw = (__u8 ),
    pub 1]: return raw[desc->bLength -,
// 4.3.2.4 Selector Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_selector_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUintID: __u8,
    pub bNrInPins: __u8,
    pub baSourceID: [__u8; ],
// C attribute field omitted
    pub desc: *mut *mut *mut __u8 raw = (__u8 ),
    pub 1]: return raw[desc->bLength -,
// 4.3.2.5 Feature Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_feature_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub bSourceID: __u8,
    pub bControlSize: __u8,
    pub /: *mut *mut __u8 bmaControls[]; / variable length,
    pub __attribute__((packed)): },
    pub desc: *mut *mut *mut __u8 raw = (__u8 ),
    pub 1]: return raw[desc->bLength -,
// 4.3.2.6 Processing Unit Descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_processing_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub wProcessType: __le16,
    pub bNrInPins: __u8,
    pub baSourceID: [__u8; ],
// C attribute field omitted
    pub desc->baSourceID[desc->bNrInPins]: return,
    pub 1]: desc->baSourceID[desc->bNrInPins +,
    pub 1]): (desc->baSourceID[desc->bNrInPins +,
    pub 5]: desc->baSourceID[desc->bNrInPins +,
    pub 4]: return desc->baSourceID[desc->bNrInPins +,
    pub /: *mut *mut return 2; / in UAC2, this value is constant,
    pub /: *mut *mut return 4; / in UAC3, this value is constant,
    pub 1: return,
    pub 5]: return &desc->baSourceID[desc->bNrInPins +,
    pub 6]: return &desc->baSourceID[desc->bNrInPins +,
    pub 2]: return &desc->baSourceID[desc->bNrInPins +,
    pub NULL: return,
    pub protocol): __u8 control_size = uac_processing_unit_bControlSize(desc,,
    pub control_size): +,
    pub /: *mut *mut return 0; / UAC3 does not have this field,
    pub protocol): __u8 control_size = uac_processing_unit_bControlSize(desc,,
    pub 1: + control_size +,
    pub control_size: +,
//
// Extension Unit (XU) has almost compatible layout with Processing Unit, but
// on UAC2, it has a different bmControls size (bControlSize); it's 1 byte for
// XU while 2 bytes for PU.  The last iExtension field is a one-byte index as
// well as iProcessing field of PU.
//
    pub 4]: return desc->baSourceID[desc->bNrInPins +,
    pub /: *mut *mut return 1; / in UAC2, this value is constant,
    pub /: *mut *mut return 4; / in UAC3, this value is constant,
    pub 1: return,
    pub protocol): __u8 control_size = uac_extension_unit_bControlSize(desc,,
    pub control_size): +,
    pub /: *mut *mut return 0; / UAC3 does not have this field,
// 4.5.2 Class-Specific AS Interface Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac1_as_header_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 7,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / AS_GENERAL,
    pub /: *mut *mut __u8 bTerminalLink; / Terminal ID of connected Terminal,
    pub /: *mut *mut __u8 bDelay; / Delay introduced by the data path,
    pub /: *mut *mut __le16 wFormatTag; / The Audio Data Format,
// C attribute field omitted
pub const UAC_DT_AS_HEADER_SIZE: c_int = 7;
// Formats - A.1.1 Audio Data Format Type I Codes
pub const UAC_FORMAT_TYPE_I_UNDEFINED: c_uint = 0x0;
pub const UAC_FORMAT_TYPE_I_PCM: c_uint = 0x1;
pub const UAC_FORMAT_TYPE_I_PCM8: c_uint = 0x2;
pub const UAC_FORMAT_TYPE_I_IEEE_FLOAT: c_uint = 0x3;
pub const UAC_FORMAT_TYPE_I_ALAW: c_uint = 0x4;
pub const UAC_FORMAT_TYPE_I_MULAW: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_format_type_i_continuous_descriptor {
    pub /: *mut *mut *mut __u8 bLength; / in bytes: 8 + (ns  3),
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / FORMAT_TYPE,
    pub /: *mut *mut __u8 bFormatType; / FORMAT_TYPE_1,
    pub /: *mut *mut __u8 bNrChannels; / physical channels in the stream,
    pub /: *mut *mut __u8 bSubframeSize; /,
    pub bBitResolution: __u8,
    pub bSamFreqType: __u8,
    pub tLowerSamFreq: [__u8; 3],
    pub tUpperSamFreq: [__u8; 3],
// C attribute field omitted
pub const UAC_FORMAT_TYPE_I_CONTINUOUS_DESC_SIZE: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_format_type_i_discrete_descriptor {
    pub /: *mut *mut *mut __u8 bLength; / in bytes: 8 + (ns  3),
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / FORMAT_TYPE,
    pub /: *mut *mut __u8 bFormatType; / FORMAT_TYPE_1,
    pub /: *mut *mut __u8 bNrChannels; / physical channels in the stream,
    pub /: *mut *mut __u8 bSubframeSize; /,
    pub bBitResolution: __u8,
    pub bSamFreqType: __u8,
    pub tSamFreq: [__u8; ][3],
// C attribute field omitted

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bFormatType;,
    pub \: __u8 bNrChannels;,
    pub \: __u8 bSubframeSize;,
    pub \: __u8 bBitResolution;,
    pub \: __u8 bSamFreqType;,
    pub \: __u8 tSamFreq[n][3];,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_format_type_i_ext_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bFormatType: __u8,
    pub bSubslotSize: __u8,
    pub bBitResolution: __u8,
    pub bHeaderLength: __u8,
    pub bControlSize: __u8,
    pub bSideBandProtocol: __u8,
    pub __attribute__((packed)): },
// Formats - Audio Data Format Type I Codes
pub const UAC_FORMAT_TYPE_II_MPEG: c_uint = 0x1001;
pub const UAC_FORMAT_TYPE_II_AC3: c_uint = 0x1002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_format_type_ii_discrete_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bFormatType: __u8,
    pub wMaxBitRate: __le16,
    pub wSamplesPerFrame: __le16,
    pub bSamFreqType: __u8,
    pub tSamFreq: [__u8; ][3],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_format_type_ii_ext_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bFormatType: __u8,
    pub wMaxBitRate: __le16,
    pub wSamplesPerFrame: __le16,
    pub bHeaderLength: __u8,
    pub bSideBandProtocol: __u8,
    pub __attribute__((packed)): },
// type III
pub const UAC_FORMAT_TYPE_III_IEC1937_AC3: c_uint = 0x2001;
pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG1_LAYER1: c_uint = 0x2002;
pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_NOEXT: c_uint = 0x2003;
pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_EXT: c_uint = 0x2004;
pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_LAYER1_LS: c_uint = 0x2005;
pub const UAC_FORMAT_TYPE_III_IEC1937_MPEG2_LAYER23_LS: c_uint = 0x2006;
// Formats - A.2 Format Type Codes
pub const UAC_FORMAT_TYPE_UNDEFINED: c_uint = 0x0;
pub const UAC_FORMAT_TYPE_I: c_uint = 0x1;
pub const UAC_FORMAT_TYPE_II: c_uint = 0x2;
pub const UAC_FORMAT_TYPE_III: c_uint = 0x3;
pub const UAC_EXT_FORMAT_TYPE_I: c_uint = 0x81;
pub const UAC_EXT_FORMAT_TYPE_II: c_uint = 0x82;
pub const UAC_EXT_FORMAT_TYPE_III: c_uint = 0x83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_iso_endpoint_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 7,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_ENDPOINT,
    pub /: *mut *mut __u8 bDescriptorSubtype; / EP_GENERAL,
    pub bmAttributes: __u8,
    pub bLockDelayUnits: __u8,
    pub wLockDelay: __le16,
    pub __attribute__((packed)): },
pub const UAC_ISO_ENDPOINT_DESC_SIZE: c_int = 7;
pub const UAC_EP_CS_ATTR_SAMPLE_RATE: c_uint = 0x01;
pub const UAC_EP_CS_ATTR_PITCH_CONTROL: c_uint = 0x02;
pub const UAC_EP_CS_ATTR_FILL_MAX: c_uint = 0x80;
// status word format (3.7.1.1)
pub const UAC1_STATUS_TYPE_ORIG_MASK: c_uint = 0x0f;
pub const UAC1_STATUS_TYPE_ORIG_AUDIO_CONTROL_IF: c_uint = 0x0;
pub const UAC1_STATUS_TYPE_ORIG_AUDIO_STREAM_IF: c_uint = 0x1;
pub const UAC1_STATUS_TYPE_ORIG_AUDIO_STREAM_EP: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac1_status_word {
    pub bStatusType: __u8,
    pub bOriginator: __u8,
    pub __attribute__((packed)): },
