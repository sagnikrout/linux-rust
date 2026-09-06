//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/video.h
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
// USB Video Class definitions.
//
// Copyright (C) 2009 Laurent Pinchart <laurent.pinchart@skynet.be>
//
// This file holds USB constants and structures defined by the USB Device
// Class Definition for Video Devices. Unless otherwise stated, comments
// below reference relevant sections of the USB Video Class 1.1 specification
// available at
//
// http://www.usb.org/developers/devclass_docs/USB_Video_Class_1_1.zip
//

// --------------------------------------------------------------------------
// UVC constants
//
// A.2. Video Interface Subclass Codes
pub const UVC_SC_UNDEFINED: c_uint = 0x00;
pub const UVC_SC_VIDEOCONTROL: c_uint = 0x01;
pub const UVC_SC_VIDEOSTREAMING: c_uint = 0x02;
pub const UVC_SC_VIDEO_INTERFACE_COLLECTION: c_uint = 0x03;
// A.3. Video Interface Protocol Codes
pub const UVC_PC_PROTOCOL_UNDEFINED: c_uint = 0x00;
pub const UVC_PC_PROTOCOL_15: c_uint = 0x01;
// A.5. Video Class-Specific VC Interface Descriptor Subtypes
pub const UVC_VC_DESCRIPTOR_UNDEFINED: c_uint = 0x00;
pub const UVC_VC_HEADER: c_uint = 0x01;
pub const UVC_VC_INPUT_TERMINAL: c_uint = 0x02;
pub const UVC_VC_OUTPUT_TERMINAL: c_uint = 0x03;
pub const UVC_VC_SELECTOR_UNIT: c_uint = 0x04;
pub const UVC_VC_PROCESSING_UNIT: c_uint = 0x05;
pub const UVC_VC_EXTENSION_UNIT: c_uint = 0x06;
// A.6. Video Class-Specific VS Interface Descriptor Subtypes
pub const UVC_VS_UNDEFINED: c_uint = 0x00;
pub const UVC_VS_INPUT_HEADER: c_uint = 0x01;
pub const UVC_VS_OUTPUT_HEADER: c_uint = 0x02;
pub const UVC_VS_STILL_IMAGE_FRAME: c_uint = 0x03;
pub const UVC_VS_FORMAT_UNCOMPRESSED: c_uint = 0x04;
pub const UVC_VS_FRAME_UNCOMPRESSED: c_uint = 0x05;
pub const UVC_VS_FORMAT_MJPEG: c_uint = 0x06;
pub const UVC_VS_FRAME_MJPEG: c_uint = 0x07;
pub const UVC_VS_FORMAT_MPEG2TS: c_uint = 0x0a;
pub const UVC_VS_FORMAT_DV: c_uint = 0x0c;
pub const UVC_VS_COLORFORMAT: c_uint = 0x0d;
pub const UVC_VS_FORMAT_FRAME_BASED: c_uint = 0x10;
pub const UVC_VS_FRAME_FRAME_BASED: c_uint = 0x11;
pub const UVC_VS_FORMAT_STREAM_BASED: c_uint = 0x12;
// A.7. Video Class-Specific Endpoint Descriptor Subtypes
pub const UVC_EP_UNDEFINED: c_uint = 0x00;
pub const UVC_EP_GENERAL: c_uint = 0x01;
pub const UVC_EP_ENDPOINT: c_uint = 0x02;
pub const UVC_EP_INTERRUPT: c_uint = 0x03;
// A.8. Video Class-Specific Request Codes
pub const UVC_RC_UNDEFINED: c_uint = 0x00;
pub const UVC_SET_CUR: c_uint = 0x01;
pub const UVC_GET_CUR: c_uint = 0x81;
pub const UVC_GET_MIN: c_uint = 0x82;
pub const UVC_GET_MAX: c_uint = 0x83;
pub const UVC_GET_RES: c_uint = 0x84;
pub const UVC_GET_LEN: c_uint = 0x85;
pub const UVC_GET_INFO: c_uint = 0x86;
pub const UVC_GET_DEF: c_uint = 0x87;
// A.9.1. VideoControl Interface Control Selectors
pub const UVC_VC_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UVC_VC_VIDEO_POWER_MODE_CONTROL: c_uint = 0x01;
pub const UVC_VC_REQUEST_ERROR_CODE_CONTROL: c_uint = 0x02;
// A.9.2. Terminal Control Selectors
pub const UVC_TE_CONTROL_UNDEFINED: c_uint = 0x00;
// A.9.3. Selector Unit Control Selectors
pub const UVC_SU_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UVC_SU_INPUT_SELECT_CONTROL: c_uint = 0x01;
// A.9.4. Camera Terminal Control Selectors
pub const UVC_CT_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UVC_CT_SCANNING_MODE_CONTROL: c_uint = 0x01;
pub const UVC_CT_AE_MODE_CONTROL: c_uint = 0x02;
pub const UVC_CT_AE_PRIORITY_CONTROL: c_uint = 0x03;
pub const UVC_CT_EXPOSURE_TIME_ABSOLUTE_CONTROL: c_uint = 0x04;
pub const UVC_CT_EXPOSURE_TIME_RELATIVE_CONTROL: c_uint = 0x05;
pub const UVC_CT_FOCUS_ABSOLUTE_CONTROL: c_uint = 0x06;
pub const UVC_CT_FOCUS_RELATIVE_CONTROL: c_uint = 0x07;
pub const UVC_CT_FOCUS_AUTO_CONTROL: c_uint = 0x08;
pub const UVC_CT_IRIS_ABSOLUTE_CONTROL: c_uint = 0x09;
pub const UVC_CT_IRIS_RELATIVE_CONTROL: c_uint = 0x0a;
pub const UVC_CT_ZOOM_ABSOLUTE_CONTROL: c_uint = 0x0b;
pub const UVC_CT_ZOOM_RELATIVE_CONTROL: c_uint = 0x0c;
pub const UVC_CT_PANTILT_ABSOLUTE_CONTROL: c_uint = 0x0d;
pub const UVC_CT_PANTILT_RELATIVE_CONTROL: c_uint = 0x0e;
pub const UVC_CT_ROLL_ABSOLUTE_CONTROL: c_uint = 0x0f;
pub const UVC_CT_ROLL_RELATIVE_CONTROL: c_uint = 0x10;
pub const UVC_CT_PRIVACY_CONTROL: c_uint = 0x11;
pub const UVC_CT_REGION_OF_INTEREST_CONTROL: c_uint = 0x14;
// A.9.5. Processing Unit Control Selectors
pub const UVC_PU_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UVC_PU_BACKLIGHT_COMPENSATION_CONTROL: c_uint = 0x01;
pub const UVC_PU_BRIGHTNESS_CONTROL: c_uint = 0x02;
pub const UVC_PU_CONTRAST_CONTROL: c_uint = 0x03;
pub const UVC_PU_GAIN_CONTROL: c_uint = 0x04;
pub const UVC_PU_POWER_LINE_FREQUENCY_CONTROL: c_uint = 0x05;
pub const UVC_PU_HUE_CONTROL: c_uint = 0x06;
pub const UVC_PU_SATURATION_CONTROL: c_uint = 0x07;
pub const UVC_PU_SHARPNESS_CONTROL: c_uint = 0x08;
pub const UVC_PU_GAMMA_CONTROL: c_uint = 0x09;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_CONTROL: c_uint = 0x0a;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_AUTO_CONTROL: c_uint = 0x0b;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_CONTROL: c_uint = 0x0c;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_AUTO_CONTROL: c_uint = 0x0d;
pub const UVC_PU_DIGITAL_MULTIPLIER_CONTROL: c_uint = 0x0e;
pub const UVC_PU_DIGITAL_MULTIPLIER_LIMIT_CONTROL: c_uint = 0x0f;
pub const UVC_PU_HUE_AUTO_CONTROL: c_uint = 0x10;
pub const UVC_PU_ANALOG_VIDEO_STANDARD_CONTROL: c_uint = 0x11;
pub const UVC_PU_ANALOG_LOCK_STATUS_CONTROL: c_uint = 0x12;
// A.9.7. VideoStreaming Interface Control Selectors
pub const UVC_VS_CONTROL_UNDEFINED: c_uint = 0x00;
pub const UVC_VS_PROBE_CONTROL: c_uint = 0x01;
pub const UVC_VS_COMMIT_CONTROL: c_uint = 0x02;
pub const UVC_VS_STILL_PROBE_CONTROL: c_uint = 0x03;
pub const UVC_VS_STILL_COMMIT_CONTROL: c_uint = 0x04;
pub const UVC_VS_STILL_IMAGE_TRIGGER_CONTROL: c_uint = 0x05;
pub const UVC_VS_STREAM_ERROR_CODE_CONTROL: c_uint = 0x06;
pub const UVC_VS_GENERATE_KEY_FRAME_CONTROL: c_uint = 0x07;
pub const UVC_VS_UPDATE_FRAME_SEGMENT_CONTROL: c_uint = 0x08;
pub const UVC_VS_SYNC_DELAY_CONTROL: c_uint = 0x09;
// B.1. USB Terminal Types
pub const UVC_TT_VENDOR_SPECIFIC: c_uint = 0x0100;
pub const UVC_TT_STREAMING: c_uint = 0x0101;
// B.2. Input Terminal Types
pub const UVC_ITT_VENDOR_SPECIFIC: c_uint = 0x0200;
pub const UVC_ITT_CAMERA: c_uint = 0x0201;
pub const UVC_ITT_MEDIA_TRANSPORT_INPUT: c_uint = 0x0202;
// B.3. Output Terminal Types
pub const UVC_OTT_VENDOR_SPECIFIC: c_uint = 0x0300;
pub const UVC_OTT_DISPLAY: c_uint = 0x0301;
pub const UVC_OTT_MEDIA_TRANSPORT_OUTPUT: c_uint = 0x0302;
// B.4. External Terminal Types
pub const UVC_EXTERNAL_VENDOR_SPECIFIC: c_uint = 0x0400;
pub const UVC_COMPOSITE_CONNECTOR: c_uint = 0x0401;
pub const UVC_SVIDEO_CONNECTOR: c_uint = 0x0402;
pub const UVC_COMPONENT_CONNECTOR: c_uint = 0x0403;
// 2.4.2.2. Status Packet Type
pub const UVC_STATUS_TYPE_CONTROL: c_int = 1;
pub const UVC_STATUS_TYPE_STREAMING: c_int = 2;
// 2.4.3.3. Payload Header Information

// 4.1.2. Control Capabilities

// 3.9.2.6 Color Matching Descriptor Values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_color_primaries_values {
    UVC_COLOR_PRIMARIES_UNSPECIFIED,
    UVC_COLOR_PRIMARIES_BT_709_SRGB,
    UVC_COLOR_PRIMARIES_BT_470_2_M,
    UVC_COLOR_PRIMARIES_BT_470_2_B_G,
    UVC_COLOR_PRIMARIES_SMPTE_170M,
    UVC_COLOR_PRIMARIES_SMPTE_240M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_transfer_characteristics_values {
    UVC_TRANSFER_CHARACTERISTICS_UNSPECIFIED,
    UVC_TRANSFER_CHARACTERISTICS_BT_709,
    UVC_TRANSFER_CHARACTERISTICS_BT_470_2_M,
    UVC_TRANSFER_CHARACTERISTICS_BT_470_2_B_G,
    UVC_TRANSFER_CHARACTERISTICS_SMPTE_170M,
    UVC_TRANSFER_CHARACTERISTICS_SMPTE_240M,
    UVC_TRANSFER_CHARACTERISTICS_LINEAR,
    UVC_TRANSFER_CHARACTERISTICS_SRGB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_matrix_coefficients {
    UVC_MATRIX_COEFFICIENTS_UNSPECIFIED,
    UVC_MATRIX_COEFFICIENTS_BT_709,
    UVC_MATRIX_COEFFICIENTS_FCC,
    UVC_MATRIX_COEFFICIENTS_BT_470_2_B_G,
    UVC_MATRIX_COEFFICIENTS_SMPTE_170M,
    UVC_MATRIX_COEFFICIENTS_SMPTE_240M,
}

// ------------------------------------------------------------------------
// UVC structures
//
// All UVC descriptors have these 3 fields at the beginning
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_descriptor_header {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub __attribute__((packed)): },
// 3.7.2. Video Control Interface Header Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdUVC: __le16,
    pub wTotalLength: __le16,
    pub dwClockFrequency: __le32,
    pub bInCollection: __u8,
    pub baInterfaceNr: [__u8; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __le16 bcdUVC;,
    pub \: __le16 wTotalLength;,
    pub \: __le32 dwClockFrequency;,
    pub \: __u8 bInCollection;,
    pub \: __u8 baInterfaceNr[n];,
// 3.7.2.1. Input Terminal Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_input_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub iTerminal: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_INPUT_TERMINAL_SIZE: c_int = 8;
// 3.7.2.2. Output Terminal Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_output_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub bSourceID: __u8,
    pub iTerminal: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_OUTPUT_TERMINAL_SIZE: c_int = 9;
// 3.7.2.3. Camera Terminal Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_camera_terminal_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bTerminalID: __u8,
    pub wTerminalType: __le16,
    pub bAssocTerminal: __u8,
    pub iTerminal: __u8,
    pub wObjectiveFocalLengthMin: __le16,
    pub wObjectiveFocalLengthMax: __le16,
    pub wOcularFocalLength: __le16,
    pub bControlSize: __u8,
    pub bmControls: [__u8; 3],
    pub __attribute__((__packed__)): },

// 3.7.2.4. Selector Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_selector_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bUnitID: __u8,
    pub bNrInPins: __u8,
    pub baSourceID: [__u8; 0],
    pub iSelector: __u8,
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bUnitID;,
    pub \: __u8 bNrInPins;,
    pub \: __u8 baSourceID[n];,
    pub \: __u8 iSelector;,
// 3.7.2.5. Processing Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_processing_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bUnitID: __u8,
    pub bSourceID: __u8,
    pub wMaxMultiplier: __le16,
    pub bControlSize: __u8,
    pub bmControls: [__u8; 2],
    pub iProcessing: __u8,
    pub bmVideoStandards: __u8,
    pub __attribute__((__packed__)): },

// 3.7.2.6. Extension Unit Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_extension_unit_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bUnitID: __u8,
    pub guidExtensionCode: [__u8; 16],
    pub bNumControls: __u8,
    pub bNrInPins: __u8,
    pub baSourceID: [__u8; 0],
    pub bControlSize: __u8,
    pub bmControls: [__u8; 0],
    pub iExtension: __u8,
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bUnitID;,
    pub \: __u8 guidExtensionCode[16];,
    pub \: __u8 bNumControls;,
    pub \: __u8 bNrInPins;,
    pub \: __u8 baSourceID[p];,
    pub \: __u8 bControlSize;,
    pub \: __u8 bmControls[n];,
    pub \: __u8 iExtension;,
// 3.8.2.2. Video Control Interrupt Endpoint Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_control_endpoint_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub wMaxTransferSize: __le16,
    pub __attribute__((__packed__)): },
pub const UVC_DT_CONTROL_ENDPOINT_SIZE: c_int = 5;
// 3.9.2.1. Input Header Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_input_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bNumFormats: __u8,
    pub wTotalLength: __le16,
    pub bEndpointAddress: __u8,
    pub bmInfo: __u8,
    pub bTerminalLink: __u8,
    pub bStillCaptureMethod: __u8,
    pub bTriggerSupport: __u8,
    pub bTriggerUsage: __u8,
    pub bControlSize: __u8,
    pub bmaControls: [__u8; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bNumFormats;,
    pub \: __le16 wTotalLength;,
    pub \: __u8 bEndpointAddress;,
    pub \: __u8 bmInfo;,
    pub \: __u8 bTerminalLink;,
    pub \: __u8 bStillCaptureMethod;,
    pub \: __u8 bTriggerSupport;,
    pub \: __u8 bTriggerUsage;,
    pub \: __u8 bControlSize;,
    pub \: __u8 bmaControls[p][n];,
// 3.9.2.2. Output Header Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_output_header_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bNumFormats: __u8,
    pub wTotalLength: __le16,
    pub bEndpointAddress: __u8,
    pub bTerminalLink: __u8,
    pub bControlSize: __u8,
    pub bmaControls: [__u8; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bNumFormats;,
    pub \: __le16 wTotalLength;,
    pub \: __u8 bEndpointAddress;,
    pub \: __u8 bTerminalLink;,
    pub \: __u8 bControlSize;,
    pub \: __u8 bmaControls[p][n];,
// 3.9.2.6. Color matching descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_color_matching_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bColorPrimaries: __u8,
    pub bTransferCharacteristics: __u8,
    pub bMatrixCoefficients: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_COLOR_MATCHING_SIZE: c_int = 6;
// 4.3.1.1. Video Probe and Commit Controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_streaming_control {
    pub bmHint: __u16,
    pub bFormatIndex: __u8,
    pub bFrameIndex: __u8,
    pub dwFrameInterval: __u32,
    pub wKeyFrameRate: __u16,
    pub wPFrameRate: __u16,
    pub wCompQuality: __u16,
    pub wCompWindowSize: __u16,
    pub wDelay: __u16,
    pub dwMaxVideoFrameSize: __u32,
    pub dwMaxPayloadTransferSize: __u32,
    pub dwClockFrequency: __u32,
    pub bmFramingInfo: __u8,
    pub bPreferedVersion: __u8,
    pub bMinVersion: __u8,
    pub bMaxVersion: __u8,
    pub __attribute__((__packed__)): },
// Uncompressed Payload - 3.1.1. Uncompressed Video Format Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_format_uncompressed {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFormatIndex: __u8,
    pub bNumFrameDescriptors: __u8,
    pub guidFormat: [__u8; 16],
    pub bBitsPerPixel: __u8,
    pub bDefaultFrameIndex: __u8,
    pub bAspectRatioX: __u8,
    pub bAspectRatioY: __u8,
    pub bmInterlaceFlags: __u8,
    pub bCopyProtect: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_FORMAT_UNCOMPRESSED_SIZE: c_int = 27;
// Uncompressed Payload - 3.1.2. Uncompressed Video Frame Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_frame_uncompressed {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFrameIndex: __u8,
    pub bmCapabilities: __u8,
    pub wWidth: __le16,
    pub wHeight: __le16,
    pub dwMinBitRate: __le32,
    pub dwMaxBitRate: __le32,
    pub dwMaxVideoFrameBufferSize: __le32,
    pub dwDefaultFrameInterval: __le32,
    pub bFrameIntervalType: __u8,
    pub dwFrameInterval: [__le32; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bFrameIndex;,
    pub \: __u8 bmCapabilities;,
    pub \: __le16 wWidth;,
    pub \: __le16 wHeight;,
    pub \: __le32 dwMinBitRate;,
    pub \: __le32 dwMaxBitRate;,
    pub \: __le32 dwMaxVideoFrameBufferSize;,
    pub \: __le32 dwDefaultFrameInterval;,
    pub \: __u8 bFrameIntervalType;,
    pub \: __le32 dwFrameInterval[n];,
// MJPEG Payload - 3.1.1. MJPEG Video Format Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_format_mjpeg {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFormatIndex: __u8,
    pub bNumFrameDescriptors: __u8,
    pub bmFlags: __u8,
    pub bDefaultFrameIndex: __u8,
    pub bAspectRatioX: __u8,
    pub bAspectRatioY: __u8,
    pub bmInterlaceFlags: __u8,
    pub bCopyProtect: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_FORMAT_MJPEG_SIZE: c_int = 11;
// MJPEG Payload - 3.1.2. MJPEG Video Frame Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_frame_mjpeg {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFrameIndex: __u8,
    pub bmCapabilities: __u8,
    pub wWidth: __le16,
    pub wHeight: __le16,
    pub dwMinBitRate: __le32,
    pub dwMaxBitRate: __le32,
    pub dwMaxVideoFrameBufferSize: __le32,
    pub dwDefaultFrameInterval: __le32,
    pub bFrameIntervalType: __u8,
    pub dwFrameInterval: [__le32; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bFrameIndex;,
    pub \: __u8 bmCapabilities;,
    pub \: __le16 wWidth;,
    pub \: __le16 wHeight;,
    pub \: __le32 dwMinBitRate;,
    pub \: __le32 dwMaxBitRate;,
    pub \: __le32 dwMaxVideoFrameBufferSize;,
    pub \: __le32 dwDefaultFrameInterval;,
    pub \: __u8 bFrameIntervalType;,
    pub \: __le32 dwFrameInterval[n];,
// Frame Based Payload - 3.1.1. Frame Based Video Format Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_format_framebased {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFormatIndex: __u8,
    pub bNumFrameDescriptors: __u8,
    pub guidFormat: [__u8; 16],
    pub bBitsPerPixel: __u8,
    pub bDefaultFrameIndex: __u8,
    pub bAspectRatioX: __u8,
    pub bAspectRatioY: __u8,
    pub bmInterfaceFlags: __u8,
    pub bCopyProtect: __u8,
    pub bVariableSize: __u8,
    pub __attribute__((__packed__)): },
pub const UVC_DT_FORMAT_FRAMEBASED_SIZE: c_int = 28;
// Frame Based Payload - 3.1.2. Frame Based Video Frame Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_frame_framebased {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bFrameIndex: __u8,
    pub bmCapabilities: __u8,
    pub wWidth: __u16,
    pub wHeight: __u16,
    pub dwMinBitRate: __u32,
    pub dwMaxBitRate: __u32,
    pub dwDefaultFrameInterval: __u32,
    pub bFrameIntervalType: __u8,
    pub dwBytesPerLine: __u32,
    pub dwFrameInterval: [__u32; ],
    pub __attribute__((__packed__)): },

    pub \: __u8 bLength;,
    pub \: __u8 bDescriptorType;,
    pub \: __u8 bDescriptorSubType;,
    pub \: __u8 bFrameIndex;,
    pub \: __u8 bmCapabilities;,
    pub \: __u16 wWidth;,
    pub \: __u16 wHeight;,
    pub \: __u32 dwMinBitRate;,
    pub \: __u32 dwMaxBitRate;,
    pub \: __u32 dwDefaultFrameInterval;,
    pub \: __u8 bFrameIntervalType;,
    pub \: __u32 dwBytesPerLine;,
    pub \: __u32 dwFrameInterval[n];,
