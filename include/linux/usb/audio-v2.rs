//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/audio-v2.h
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
// Copyright (c) 2010 Daniel Mack <daniel@caiaq.de>
//
// This file holds USB constants and structures defined
// by the USB Device Class Definition for Audio Devices in version 2.0.
// Comments below reference relevant sections of the documents contained
// in http://www.usb.org/developers/devclass_docs/Audio2.0_final.zip
//

// v1.0 and v2.0 of this standard have many things in common. For the rest
// of the definitions, please refer to audio.h
//
// bmControl field decoders
//
// From the USB Audio spec v2.0:
//
// bmaControls() is a (ch+1)-element array of 4-byte bitmaps,
// each containing a set of bit pairs. If a Control is present,
// it must be Host readable. If a certain Control is not
// present then the bit pair must be set to 0b00.
// If a Control is present but read-only, the bit pair must be
// set to 0b01. If a Control is also Host programmable, the bit
// pair must be set to 0b11. The value 0b10 is not allowed.
//
// 4.7.2 Class-Specific AC Interface Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_ac_header_descriptor {
    pub /: *mut *mut __u8 bLength; / 9,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / UAC_MS_HEADER,
    pub /: *mut *mut __le16 bcdADC; / 0x0200,
    pub bCategory: __u8,
    pub /: *mut *mut __le16 wTotalLength; / includes Unit and Terminal desc.,
    pub bmControls: __u8,
    pub __packed: },
// 2.3.1.6 Type I Format Type Descriptor (Frmts20 final.pdf)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_format_type_i_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 6,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_INTERFACE,
    pub /: *mut *mut __u8 bDescriptorSubtype; / FORMAT_TYPE,
    pub /: *mut *mut __u8 bFormatType; / FORMAT_TYPE_1,
    pub /: *mut *mut __u8 bSubslotSize; / {1,2,3,4},
    pub bBitResolution: __u8,
    pub __packed: },
// 4.7.2.1 Clock Source Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_clock_source_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bmAttributes: __u8,
    pub bmControls: __u8,
    pub bAssocTerminal: __u8,
    pub iClockSource: __u8,
    pub __attribute__((packed)): },
// bmAttribute fields
pub const UAC_CLOCK_SOURCE_TYPE_EXT: c_uint = 0x0;
pub const UAC_CLOCK_SOURCE_TYPE_INT_FIXED: c_uint = 0x1;
pub const UAC_CLOCK_SOURCE_TYPE_INT_VAR: c_uint = 0x2;
pub const UAC_CLOCK_SOURCE_TYPE_INT_PROG: c_uint = 0x3;

// 4.7.2.2 Clock Selector Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_clock_selector_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bNrInPins: __u8,
    pub baCSourceID: [__u8; ],
// bmControls and iClockSelector omitted
    pub __attribute__((packed)): },
// 4.7.2.3 Clock Multiplier Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_clock_multiplier_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bClockID: __u8,
    pub bCSourceID: __u8,
    pub bmControls: __u8,
    pub iClockMultiplier: __u8,
    pub __attribute__((packed)): },
// 4.7.2.4 Input terminal descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_input_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub bCSourceID: __u8,
    pub bNrChannels: __u8,
    pub bmChannelConfig: __le32,
    pub iChannelNames: __u8,
    pub bmControls: __le16,
    pub iTerminal: __u8,
    pub __attribute__((packed)): },
// 4.7.2.5 Output terminal descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_output_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub bSourceID: __u8,
    pub bCSourceID: __u8,
    pub bmControls: __le16,
    pub iTerminal: __u8,
    pub __attribute__((packed)): },
// 4.7.2.8 Feature Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_feature_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub bSourceID: __u8,
// bmaControls is actually u32,
// but u8 is needed for the hybrid parser
    pub /: *mut *mut __u8 bmaControls[]; / variable length,
    pub __attribute__((packed)): },

// As above, but more useful for defining your own descriptors:

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubtype;,
    pub \: __u8 bUnitID;,
    pub \: __u8 bSourceID;,
    pub \: __le32 bmaControls[ch + 1];,
    pub \: __u8 iFeature;,
// 4.7.2.10 Effect Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_effect_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bUnitID: __u8,
    pub wEffectType: __le16,
    pub bSourceID: __u8,
    pub /: *mut *mut __u8 bmaControls[]; / variable length,
    pub __attribute__((packed)): },
// 4.9.2 Class-Specific AS Interface Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_as_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bTerminalLink: __u8,
    pub bmControls: __u8,
    pub bFormatType: __u8,
    pub bmFormats: __le32,
    pub bNrChannels: __u8,
    pub bmChannelConfig: __le32,
    pub iChannelNames: __u8,
    pub __attribute__((packed)): },

// 4.10.1.2 Class-Specific AS Isochronous Audio Data Endpoint Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_iso_endpoint_descriptor {
    pub /: *mut *mut __u8 bLength; / in bytes: 8,
    pub /: *mut *mut __u8 bDescriptorType; / USB_DT_CS_ENDPOINT,
    pub /: *mut *mut __u8 bDescriptorSubtype; / EP_GENERAL,
    pub bmAttributes: __u8,
    pub bmControls: __u8,
    pub bLockDelayUnits: __u8,
    pub wLockDelay: __le16,
    pub __attribute__((packed)): },

// 5.2.5.4.2 Connector Control Parameter Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_connectors_ctl_blk {
    pub bNrChannels: __u8,
    pub bmChannelConfig: __le32,
    pub iChannelNames: __u8,
    pub __attribute__((packed)): },
// 6.1 Interrupt Data Message

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac2_interrupt_data_msg {
    pub bInfo: __u8,
    pub bAttribute: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub __attribute__((packed)): },
// A.7 Audio Function Category Codes
pub const UAC2_FUNCTION_SUBCLASS_UNDEFINED: c_uint = 0x00;
pub const UAC2_FUNCTION_DESKTOP_SPEAKER: c_uint = 0x01;
pub const UAC2_FUNCTION_HOME_THEATER: c_uint = 0x02;
pub const UAC2_FUNCTION_MICROPHONE: c_uint = 0x03;
pub const UAC2_FUNCTION_HEADSET: c_uint = 0x04;
pub const UAC2_FUNCTION_TELEPHONE: c_uint = 0x05;
pub const UAC2_FUNCTION_CONVERTER: c_uint = 0x06;
pub const UAC2_FUNCTION_SOUND_RECORDER: c_uint = 0x07;
pub const UAC2_FUNCTION_IO_BOX: c_uint = 0x08;
pub const UAC2_FUNCTION_MUSICAL_INSTRUMENT: c_uint = 0x09;
pub const UAC2_FUNCTION_PRO_AUDIO: c_uint = 0x0a;
pub const UAC2_FUNCTION_AUDIO_VIDEO: c_uint = 0x0b;
pub const UAC2_FUNCTION_CONTROL_PANEL: c_uint = 0x0c;
pub const UAC2_FUNCTION_OTHER: c_uint = 0xff;
// A.9 Audio Class-Specific AC Interface Descriptor Subtypes
// see audio.h for the rest, which is identical to v1
pub const UAC2_EFFECT_UNIT: c_uint = 0x07;
pub const UAC2_PROCESSING_UNIT_V2: c_uint = 0x08;
pub const UAC2_EXTENSION_UNIT_V2: c_uint = 0x09;
pub const UAC2_CLOCK_SOURCE: c_uint = 0x0a;
pub const UAC2_CLOCK_SELECTOR: c_uint = 0x0b;
pub const UAC2_CLOCK_MULTIPLIER: c_uint = 0x0c;
pub const UAC2_SAMPLE_RATE_CONVERTER: c_uint = 0x0d;
// A.10 Audio Class-Specific AS Interface Descriptor Subtypes
// see audio.h for the rest, which is identical to v1
pub const UAC2_ENCODER: c_uint = 0x03;
pub const UAC2_DECODER: c_uint = 0x04;
// A.11 Effect Unit Effect Types
pub const UAC2_EFFECT_UNDEFINED: c_uint = 0x00;
pub const UAC2_EFFECT_PARAM_EQ: c_uint = 0x01;
pub const UAC2_EFFECT_REVERB: c_uint = 0x02;
pub const UAC2_EFFECT_MOD_DELAY: c_uint = 0x03;
pub const UAC2_EFFECT_DYN_RANGE_COMP: c_uint = 0x04;
// A.12 Processing Unit Process Types
pub const UAC2_PROCESS_UNDEFINED: c_uint = 0x00;
pub const UAC2_PROCESS_UP_DOWNMIX: c_uint = 0x01;
pub const UAC2_PROCESS_DOLBY_PROLOCIC: c_uint = 0x02;
pub const UAC2_PROCESS_STEREO_EXTENDER: c_uint = 0x03;
// A.14 Audio Class-Specific Request Codes
pub const UAC2_CS_CUR: c_uint = 0x01;
pub const UAC2_CS_RANGE: c_uint = 0x02;
pub const UAC2_CS_MEM: c_uint = 0x03;
// A.15 Encoder Type Codes
pub const UAC2_ENCODER_UNDEFINED: c_uint = 0x00;
pub const UAC2_ENCODER_OTHER: c_uint = 0x01;
pub const UAC2_ENCODER_MPEG: c_uint = 0x02;
pub const UAC2_ENCODER_AC3: c_uint = 0x03;
pub const UAC2_ENCODER_WMA: c_uint = 0x04;
pub const UAC2_ENCODER_DTS: c_uint = 0x05;
// A.16 Decoder Type Codes
pub const UAC2_DECODER_UNDEFINED: c_uint = 0x00;
pub const UAC2_DECODER_OTHER: c_uint = 0x01;
pub const UAC2_DECODER_MPEG: c_uint = 0x02;
pub const UAC2_DECODER_AC3: c_uint = 0x03;
pub const UAC2_DECODER_WMA: c_uint = 0x04;
pub const UAC2_DECODER_DTS: c_uint = 0x05;
// A.17.1 Clock Source Control Selectors
pub const UAC2_CS_UNDEFINED: c_uint = 0x00;
pub const UAC2_CS_CONTROL_SAM_FREQ: c_uint = 0x01;
pub const UAC2_CS_CONTROL_CLOCK_VALID: c_uint = 0x02;
// A.17.2 Clock Selector Control Selectors
pub const UAC2_CX_UNDEFINED: c_uint = 0x00;
pub const UAC2_CX_CLOCK_SELECTOR: c_uint = 0x01;
// A.17.3 Clock Multiplier Control Selectors
pub const UAC2_CM_UNDEFINED: c_uint = 0x00;
pub const UAC2_CM_NUMERATOR: c_uint = 0x01;
pub const UAC2_CM_DENOMINTATOR: c_uint = 0x02;
// A.17.4 Terminal Control Selectors
pub const UAC2_TE_UNDEFINED: c_uint = 0x00;
pub const UAC2_TE_COPY_PROTECT: c_uint = 0x01;
pub const UAC2_TE_CONNECTOR: c_uint = 0x02;
pub const UAC2_TE_OVERLOAD: c_uint = 0x03;
pub const UAC2_TE_CLUSTER: c_uint = 0x04;
pub const UAC2_TE_UNDERFLOW: c_uint = 0x05;
pub const UAC2_TE_OVERFLOW: c_uint = 0x06;
pub const UAC2_TE_LATENCY: c_uint = 0x07;
// A.17.5 Mixer Control Selectors
pub const UAC2_MU_UNDEFINED: c_uint = 0x00;
pub const UAC2_MU_MIXER: c_uint = 0x01;
pub const UAC2_MU_CLUSTER: c_uint = 0x02;
pub const UAC2_MU_UNDERFLOW: c_uint = 0x03;
pub const UAC2_MU_OVERFLOW: c_uint = 0x04;
pub const UAC2_MU_LATENCY: c_uint = 0x05;
// A.17.6 Selector Control Selectors
pub const UAC2_SU_UNDEFINED: c_uint = 0x00;
pub const UAC2_SU_SELECTOR: c_uint = 0x01;
pub const UAC2_SU_LATENCY: c_uint = 0x02;
// A.17.7 Feature Unit Control Selectors
// see audio.h for the rest, which is identical to v1
pub const UAC2_FU_INPUT_GAIN: c_uint = 0x0b;
pub const UAC2_FU_INPUT_GAIN_PAD: c_uint = 0x0c;
pub const UAC2_FU_PHASE_INVERTER: c_uint = 0x0d;
pub const UAC2_FU_UNDERFLOW: c_uint = 0x0e;
pub const UAC2_FU_OVERFLOW: c_uint = 0x0f;
pub const UAC2_FU_LATENCY: c_uint = 0x10;
// A.17.8.1 Parametric Equalizer Section Effect Unit Control Selectors
pub const UAC2_PE_UNDEFINED: c_uint = 0x00;
pub const UAC2_PE_ENABLE: c_uint = 0x01;
pub const UAC2_PE_CENTERFREQ: c_uint = 0x02;
pub const UAC2_PE_QFACTOR: c_uint = 0x03;
pub const UAC2_PE_GAIN: c_uint = 0x04;
pub const UAC2_PE_UNDERFLOW: c_uint = 0x05;
pub const UAC2_PE_OVERFLOW: c_uint = 0x06;
pub const UAC2_PE_LATENCY: c_uint = 0x07;
// A.17.8.2 Reverberation Effect Unit Control Selectors
pub const UAC2_RV_UNDEFINED: c_uint = 0x00;
pub const UAC2_RV_ENABLE: c_uint = 0x01;
pub const UAC2_RV_TYPE: c_uint = 0x02;
pub const UAC2_RV_LEVEL: c_uint = 0x03;
pub const UAC2_RV_TIME: c_uint = 0x04;
pub const UAC2_RV_FEEDBACK: c_uint = 0x05;
pub const UAC2_RV_PREDELAY: c_uint = 0x06;
pub const UAC2_RV_DENSITY: c_uint = 0x07;
pub const UAC2_RV_HIFREQ_ROLLOFF: c_uint = 0x08;
pub const UAC2_RV_UNDERFLOW: c_uint = 0x09;
pub const UAC2_RV_OVERFLOW: c_uint = 0x0a;
pub const UAC2_RV_LATENCY: c_uint = 0x0b;
// A.17.8.3 Modulation Delay Effect Control Selectors
pub const UAC2_MD_UNDEFINED: c_uint = 0x00;
pub const UAC2_MD_ENABLE: c_uint = 0x01;
pub const UAC2_MD_BALANCE: c_uint = 0x02;
pub const UAC2_MD_RATE: c_uint = 0x03;
pub const UAC2_MD_DEPTH: c_uint = 0x04;
pub const UAC2_MD_TIME: c_uint = 0x05;
pub const UAC2_MD_FEEDBACK: c_uint = 0x06;
pub const UAC2_MD_UNDERFLOW: c_uint = 0x07;
pub const UAC2_MD_OVERFLOW: c_uint = 0x08;
pub const UAC2_MD_LATENCY: c_uint = 0x09;
// A.17.8.4 Dynamic Range Compressor Effect Unit Control Selectors
pub const UAC2_DR_UNDEFINED: c_uint = 0x00;
pub const UAC2_DR_ENABLE: c_uint = 0x01;
pub const UAC2_DR_COMPRESSION_RATE: c_uint = 0x02;
pub const UAC2_DR_MAXAMPL: c_uint = 0x03;
pub const UAC2_DR_THRESHOLD: c_uint = 0x04;
pub const UAC2_DR_ATTACK_TIME: c_uint = 0x05;
pub const UAC2_DR_RELEASE_TIME: c_uint = 0x06;
pub const UAC2_DR_UNDEFLOW: c_uint = 0x07;
pub const UAC2_DR_OVERFLOW: c_uint = 0x08;
pub const UAC2_DR_LATENCY: c_uint = 0x09;
// A.17.9.1 Up/Down-mix Processing Unit Control Selectors
pub const UAC2_UD_UNDEFINED: c_uint = 0x00;
pub const UAC2_UD_ENABLE: c_uint = 0x01;
pub const UAC2_UD_MODE_SELECT: c_uint = 0x02;
pub const UAC2_UD_CLUSTER: c_uint = 0x03;
pub const UAC2_UD_UNDERFLOW: c_uint = 0x04;
pub const UAC2_UD_OVERFLOW: c_uint = 0x05;
pub const UAC2_UD_LATENCY: c_uint = 0x06;
// A.17.9.2 Dolby Prologic[tm] Processing Unit Control Selectors
pub const UAC2_DP_UNDEFINED: c_uint = 0x00;
pub const UAC2_DP_ENABLE: c_uint = 0x01;
pub const UAC2_DP_MODE_SELECT: c_uint = 0x02;
pub const UAC2_DP_CLUSTER: c_uint = 0x03;
pub const UAC2_DP_UNDERFFLOW: c_uint = 0x04;
pub const UAC2_DP_OVERFLOW: c_uint = 0x05;
pub const UAC2_DP_LATENCY: c_uint = 0x06;
// A.17.9.3 Stereo Expander Processing Unit Control Selectors
pub const UAC2_ST_EXT_UNDEFINED: c_uint = 0x00;
pub const UAC2_ST_EXT_ENABLE: c_uint = 0x01;
pub const UAC2_ST_EXT_WIDTH: c_uint = 0x02;
pub const UAC2_ST_EXT_UNDEFLOW: c_uint = 0x03;
pub const UAC2_ST_EXT_OVERFLOW: c_uint = 0x04;
pub const UAC2_ST_EXT_LATENCY: c_uint = 0x05;
// A.17.10 Extension Unit Control Selectors
pub const UAC2_XU_UNDEFINED: c_uint = 0x00;
pub const UAC2_XU_ENABLE: c_uint = 0x01;
pub const UAC2_XU_CLUSTER: c_uint = 0x02;
pub const UAC2_XU_UNDERFLOW: c_uint = 0x03;
pub const UAC2_XU_OVERFLOW: c_uint = 0x04;
pub const UAC2_XU_LATENCY: c_uint = 0x05;
// A.17.11 AudioStreaming Interface Control Selectors
pub const UAC2_AS_UNDEFINED: c_uint = 0x00;
pub const UAC2_AS_ACT_ALT_SETTING: c_uint = 0x01;
pub const UAC2_AS_VAL_ALT_SETTINGS: c_uint = 0x02;
pub const UAC2_AS_AUDIO_DATA_FORMAT: c_uint = 0x03;
// A.17.12 Encoder Control Selectors
pub const UAC2_EN_UNDEFINED: c_uint = 0x00;
pub const UAC2_EN_BIT_RATE: c_uint = 0x01;
pub const UAC2_EN_QUALITY: c_uint = 0x02;
pub const UAC2_EN_VBR: c_uint = 0x03;
pub const UAC2_EN_TYPE: c_uint = 0x04;
pub const UAC2_EN_UNDERFLOW: c_uint = 0x05;
pub const UAC2_EN_OVERFLOW: c_uint = 0x06;
pub const UAC2_EN_ENCODER_ERROR: c_uint = 0x07;
pub const UAC2_EN_PARAM1: c_uint = 0x08;
pub const UAC2_EN_PARAM2: c_uint = 0x09;
pub const UAC2_EN_PARAM3: c_uint = 0x0a;
pub const UAC2_EN_PARAM4: c_uint = 0x0b;
pub const UAC2_EN_PARAM5: c_uint = 0x0c;
pub const UAC2_EN_PARAM6: c_uint = 0x0d;
pub const UAC2_EN_PARAM7: c_uint = 0x0e;
pub const UAC2_EN_PARAM8: c_uint = 0x0f;
// A.17.13.1 MPEG Decoder Control Selectors
pub const UAC2_MPEG_UNDEFINED: c_uint = 0x00;
pub const UAC2_MPEG_DUAL_CHANNEL: c_uint = 0x01;
pub const UAC2_MPEG_SECOND_STEREO: c_uint = 0x02;
pub const UAC2_MPEG_MULTILINGUAL: c_uint = 0x03;
pub const UAC2_MPEG_DYN_RANGE: c_uint = 0x04;
pub const UAC2_MPEG_SCALING: c_uint = 0x05;
pub const UAC2_MPEG_HILO_SCALING: c_uint = 0x06;
pub const UAC2_MPEG_UNDERFLOW: c_uint = 0x07;
pub const UAC2_MPEG_OVERFLOW: c_uint = 0x08;
pub const UAC2_MPEG_DECODER_ERROR: c_uint = 0x09;
// A17.13.2 AC3 Decoder Control Selectors
pub const UAC2_AC3_UNDEFINED: c_uint = 0x00;
pub const UAC2_AC3_MODE: c_uint = 0x01;
pub const UAC2_AC3_DYN_RANGE: c_uint = 0x02;
pub const UAC2_AC3_SCALING: c_uint = 0x03;
pub const UAC2_AC3_HILO_SCALING: c_uint = 0x04;
pub const UAC2_AC3_UNDERFLOW: c_uint = 0x05;
pub const UAC2_AC3_OVERFLOW: c_uint = 0x06;
pub const UAC2_AC3_DECODER_ERROR: c_uint = 0x07;
// A17.13.3 WMA Decoder Control Selectors
pub const UAC2_WMA_UNDEFINED: c_uint = 0x00;
pub const UAC2_WMA_UNDERFLOW: c_uint = 0x01;
pub const UAC2_WMA_OVERFLOW: c_uint = 0x02;
pub const UAC2_WMA_DECODER_ERROR: c_uint = 0x03;
// A17.13.4 DTS Decoder Control Selectors
pub const UAC2_DTS_UNDEFINED: c_uint = 0x00;
pub const UAC2_DTS_UNDERFLOW: c_uint = 0x01;
pub const UAC2_DTS_OVERFLOW: c_uint = 0x02;
pub const UAC2_DTS_DECODER_ERROR: c_uint = 0x03;
// A17.14 Endpoint Control Selectors
pub const UAC2_EP_CS_UNDEFINED: c_uint = 0x00;
pub const UAC2_EP_CS_PITCH: c_uint = 0x01;
pub const UAC2_EP_CS_DATA_OVERRUN: c_uint = 0x02;
pub const UAC2_EP_CS_DATA_UNDERRUN: c_uint = 0x03;
