//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/saa7164/saa7164-reg.h
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
// Driver for the NXP SAA7164 PCIe bridge
//
// Copyright (c) 2010-2015 Steven Toth <stoth@kernellabs.com>
//
// TODO: Retest the driver with errors expressed as negatives
// Result codes
pub const SAA_OK: c_int = 0;
pub const SAA_ERR_BAD_PARAMETER: c_uint = 0x09;
pub const SAA_ERR_NO_RESOURCES: c_uint = 0x0c;
pub const SAA_ERR_NOT_SUPPORTED: c_uint = 0x13;
pub const SAA_ERR_BUSY: c_uint = 0x15;
pub const SAA_ERR_READ: c_uint = 0x17;
pub const SAA_ERR_TIMEOUT: c_uint = 0x1f;
pub const SAA_ERR_OVERFLOW: c_uint = 0x20;
pub const SAA_ERR_EMPTY: c_uint = 0x22;
pub const SAA_ERR_NOT_STARTED: c_uint = 0x23;
pub const SAA_ERR_ALREADY_STARTED: c_uint = 0x24;
pub const SAA_ERR_NOT_STOPPED: c_uint = 0x25;
pub const SAA_ERR_ALREADY_STOPPED: c_uint = 0x26;
pub const SAA_ERR_INVALID_COMMAND: c_uint = 0x3e;
pub const SAA_ERR_NULL_PACKET: c_uint = 0x59;
// Errors and flags from the silicon
pub const PVC_ERRORCODE_UNKNOWN: c_uint = 0x00;
pub const PVC_ERRORCODE_INVALID_COMMAND: c_uint = 0x01;
pub const PVC_ERRORCODE_INVALID_CONTROL: c_uint = 0x02;
pub const PVC_ERRORCODE_INVALID_DATA: c_uint = 0x03;
pub const PVC_ERRORCODE_TIMEOUT: c_uint = 0x04;
pub const PVC_ERRORCODE_NAK: c_uint = 0x05;
pub const PVC_RESPONSEFLAG_ERROR: c_uint = 0x01;
pub const PVC_RESPONSEFLAG_OVERFLOW: c_uint = 0x02;
pub const PVC_RESPONSEFLAG_RESET: c_uint = 0x04;
pub const PVC_RESPONSEFLAG_INTERFACE: c_uint = 0x08;
pub const PVC_RESPONSEFLAG_CONTINUED: c_uint = 0x10;
pub const PVC_CMDFLAG_INTERRUPT: c_uint = 0x02;
pub const PVC_CMDFLAG_INTERFACE: c_uint = 0x04;
pub const PVC_CMDFLAG_SERIALIZE: c_uint = 0x08;
pub const PVC_CMDFLAG_CONTINUE: c_uint = 0x10;
// Silicon Commands
pub const GET_DESCRIPTORS_CONTROL: c_uint = 0x01;
pub const GET_STRING_CONTROL: c_uint = 0x03;
pub const GET_LANGUAGE_CONTROL: c_uint = 0x05;
pub const SET_POWER_CONTROL: c_uint = 0x07;
pub const GET_FW_STATUS_CONTROL: c_uint = 0x08;
pub const GET_FW_VERSION_CONTROL: c_uint = 0x09;
pub const SET_DEBUG_LEVEL_CONTROL: c_uint = 0x0B;
pub const GET_DEBUG_DATA_CONTROL: c_uint = 0x0C;
pub const GET_PRODUCTION_INFO_CONTROL: c_uint = 0x0D;
// cmd defines
pub const SAA_CMDFLAG_CONTINUE: c_uint = 0x10;
pub const SAA_CMD_MAX_MSG_UNITS: c_int = 256;
// Some defines
pub const SAA_BUS_TIMEOUT: c_int = 50;
pub const SAA_DEVICE_TIMEOUT: c_int = 5000;
pub const SAA_DEVICE_MAXREQUESTSIZE: c_int = 256;
// Register addresses
pub const SAA_DEVICE_VERSION: c_uint = 0x30;
pub const SAA_DOWNLOAD_FLAGS: c_uint = 0x34;
pub const SAA_DOWNLOAD_FLAG: c_uint = 0x34;
pub const SAA_DOWNLOAD_FLAG_ACK: c_uint = 0x38;
pub const SAA_DATAREADY_FLAG: c_uint = 0x3C;
pub const SAA_DATAREADY_FLAG_ACK: c_uint = 0x40;
// Boot loader register and bit definitions
pub const SAA_BOOTLOADERERROR_FLAGS: c_uint = 0x44;
pub const SAA_DEVICE_IMAGE_SEARCHING: c_uint = 0x01;
pub const SAA_DEVICE_IMAGE_LOADING: c_uint = 0x02;
pub const SAA_DEVICE_IMAGE_BOOTING: c_uint = 0x03;
pub const SAA_DEVICE_IMAGE_CORRUPT: c_uint = 0x04;
pub const SAA_DEVICE_MEMORY_CORRUPT: c_uint = 0x08;
pub const SAA_DEVICE_NO_IMAGE: c_uint = 0x10;
// Register addresses
pub const SAA_DEVICE_2ND_VERSION: c_uint = 0x50;
pub const SAA_DEVICE_2ND_DOWNLOADFLAG_OFFSET: c_uint = 0x54;
// Register addresses
pub const SAA_SECONDSTAGEERROR_FLAGS: c_uint = 0x64;
// Bootloader regs and flags
pub const SAA_DEVICE_DEADLOCK_DETECTED_OFFSET: c_uint = 0x6C;
pub const SAA_DEVICE_DEADLOCK_DETECTED: c_uint = 0xDEADDEAD;
// Basic firmware status registers
pub const SAA_DEVICE_SYSINIT_STATUS_OFFSET: c_uint = 0x70;
pub const SAA_DEVICE_SYSINIT_STATUS: c_uint = 0x70;
pub const SAA_DEVICE_SYSINIT_MODE: c_uint = 0x74;
pub const SAA_DEVICE_SYSINIT_SPEC: c_uint = 0x78;
pub const SAA_DEVICE_SYSINIT_INST: c_uint = 0x7C;
pub const SAA_DEVICE_SYSINIT_CPULOAD: c_uint = 0x80;
pub const SAA_DEVICE_SYSINIT_REMAINHEAP: c_uint = 0x84;
pub const SAA_DEVICE_DOWNLOAD_OFFSET: c_uint = 0x1000;
pub const SAA_DEVICE_BUFFERBLOCKSIZE: c_uint = 0x1000;
pub const SAA_DEVICE_2ND_BUFFERBLOCKSIZE: c_uint = 0x100000;
pub const SAA_DEVICE_2ND_DOWNLOAD_OFFSET: c_uint = 0x200000;
// Descriptors
pub const CS_INTERFACE: c_uint = 0x24;
// Descriptor subtypes
pub const VC_INPUT_TERMINAL: c_uint = 0x02;
pub const VC_OUTPUT_TERMINAL: c_uint = 0x03;
pub const VC_SELECTOR_UNIT: c_uint = 0x04;
pub const VC_PROCESSING_UNIT: c_uint = 0x05;
pub const FEATURE_UNIT: c_uint = 0x06;
pub const TUNER_UNIT: c_uint = 0x09;
pub const ENCODER_UNIT: c_uint = 0x0A;
pub const EXTENSION_UNIT: c_uint = 0x0B;
pub const VC_TUNER_PATH: c_uint = 0xF0;
pub const PVC_HARDWARE_DESCRIPTOR: c_uint = 0xF1;
pub const PVC_INTERFACE_DESCRIPTOR: c_uint = 0xF2;
pub const PVC_INFRARED_UNIT: c_uint = 0xF3;
pub const DRM_UNIT: c_uint = 0xF4;
pub const GENERAL_REQUEST: c_uint = 0xF5;
// Format Types
pub const VS_FORMAT_TYPE: c_uint = 0x02;
pub const VS_FORMAT_TYPE_I: c_uint = 0x01;
pub const VS_FORMAT_UNCOMPRESSED: c_uint = 0x04;
pub const VS_FRAME_UNCOMPRESSED: c_uint = 0x05;
pub const VS_FORMAT_MPEG2PS: c_uint = 0x09;
pub const VS_FORMAT_MPEG2TS: c_uint = 0x0A;
pub const VS_FORMAT_MPEG4SL: c_uint = 0x0B;
pub const VS_FORMAT_WM9: c_uint = 0x0C;
pub const VS_FORMAT_DIVX: c_uint = 0x0D;
pub const VS_FORMAT_VBI: c_uint = 0x0E;
pub const VS_FORMAT_RDS: c_uint = 0x0F;
// Device extension commands
pub const EXU_REGISTER_ACCESS_CONTROL: c_uint = 0x00;
pub const EXU_GPIO_CONTROL: c_uint = 0x01;
pub const EXU_GPIO_GROUP_CONTROL: c_uint = 0x02;
pub const EXU_INTERRUPT_CONTROL: c_uint = 0x03;
// State Transition and args
pub const SAA_PROBE_CONTROL: c_uint = 0x01;
pub const SAA_COMMIT_CONTROL: c_uint = 0x02;
pub const SAA_STATE_CONTROL: c_uint = 0x03;
pub const SAA_DMASTATE_STOP: c_uint = 0x00;
pub const SAA_DMASTATE_ACQUIRE: c_uint = 0x01;
pub const SAA_DMASTATE_PAUSE: c_uint = 0x02;
pub const SAA_DMASTATE_RUN: c_uint = 0x03;
// A/V Mux Input Selector
pub const SU_INPUT_SELECT_CONTROL: c_uint = 0x01;
// Encoder Profiles
pub const EU_PROFILE_PS_DVD: c_uint = 0x06;
pub const EU_PROFILE_TS_HQ: c_uint = 0x09;
pub const EU_VIDEO_FORMAT_MPEG_2: c_uint = 0x02;
// Tuner
pub const TU_AUDIO_MODE_CONTROL: c_uint = 0x17;
// Video Formats
pub const TU_STANDARD_CONTROL: c_uint = 0x00;
pub const TU_STANDARD_AUTO_CONTROL: c_uint = 0x01;
pub const TU_STANDARD_NONE: c_uint = 0x00;
pub const TU_STANDARD_NTSC_M: c_uint = 0x01;
pub const TU_STANDARD_PAL_I: c_uint = 0x08;
pub const TU_STANDARD_MANUAL: c_uint = 0x00;
pub const TU_STANDARD_AUTO: c_uint = 0x01;
// Video Controls
pub const PU_BRIGHTNESS_CONTROL: c_uint = 0x02;
pub const PU_CONTRAST_CONTROL: c_uint = 0x03;
pub const PU_HUE_CONTROL: c_uint = 0x06;
pub const PU_SATURATION_CONTROL: c_uint = 0x07;
pub const PU_SHARPNESS_CONTROL: c_uint = 0x08;
// Audio Controls
pub const MUTE_CONTROL: c_uint = 0x01;
pub const VOLUME_CONTROL: c_uint = 0x02;
pub const AUDIO_DEFAULT_CONTROL: c_uint = 0x0D;
// Default Volume Levels
pub const TMHW_LEV_ADJ_DECLEV_DEFAULT: c_uint = 0x00;
pub const TMHW_LEV_ADJ_MONOLEV_DEFAULT: c_uint = 0x00;
pub const TMHW_LEV_ADJ_NICLEV_DEFAULT: c_uint = 0x00;
pub const TMHW_LEV_ADJ_SAPLEV_DEFAULT: c_uint = 0x00;
pub const TMHW_LEV_ADJ_ADCLEV_DEFAULT: c_uint = 0x00;
// Encoder Related Commands
pub const EU_PROFILE_CONTROL: c_uint = 0x00;
pub const EU_VIDEO_FORMAT_CONTROL: c_uint = 0x01;
pub const EU_VIDEO_BIT_RATE_CONTROL: c_uint = 0x02;
pub const EU_VIDEO_RESOLUTION_CONTROL: c_uint = 0x03;
pub const EU_VIDEO_GOP_STRUCTURE_CONTROL: c_uint = 0x04;
pub const EU_VIDEO_INPUT_ASPECT_CONTROL: c_uint = 0x0A;
pub const EU_AUDIO_FORMAT_CONTROL: c_uint = 0x0C;
pub const EU_AUDIO_BIT_RATE_CONTROL: c_uint = 0x0D;
// Firmware Debugging
pub const SET_DEBUG_LEVEL_CONTROL: c_uint = 0x0B;
pub const GET_DEBUG_DATA_CONTROL: c_uint = 0x0C;
