//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda_verbs.h
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
// HD-audio codec verbs
//
// nodes
//
pub const AC_NODE_ROOT: c_uint = 0x00;
//
// function group types
//
// widget types
//
// GET verbs
//
pub const AC_VERB_GET_STREAM_FORMAT: c_uint = 0x0a00;
pub const AC_VERB_GET_AMP_GAIN_MUTE: c_uint = 0x0b00;
pub const AC_VERB_GET_PROC_COEF: c_uint = 0x0c00;
pub const AC_VERB_GET_COEF_INDEX: c_uint = 0x0d00;
pub const AC_VERB_PARAMETERS: c_uint = 0x0f00;
pub const AC_VERB_GET_CONNECT_SEL: c_uint = 0x0f01;
pub const AC_VERB_GET_CONNECT_LIST: c_uint = 0x0f02;
pub const AC_VERB_GET_PROC_STATE: c_uint = 0x0f03;
pub const AC_VERB_GET_SDI_SELECT: c_uint = 0x0f04;
pub const AC_VERB_GET_POWER_STATE: c_uint = 0x0f05;
pub const AC_VERB_GET_CONV: c_uint = 0x0f06;
pub const AC_VERB_GET_PIN_WIDGET_CONTROL: c_uint = 0x0f07;
pub const AC_VERB_GET_UNSOLICITED_RESPONSE: c_uint = 0x0f08;
pub const AC_VERB_GET_PIN_SENSE: c_uint = 0x0f09;
pub const AC_VERB_GET_BEEP_CONTROL: c_uint = 0x0f0a;
pub const AC_VERB_GET_EAPD_BTLENABLE: c_uint = 0x0f0c;
pub const AC_VERB_GET_DIGI_CONVERT_1: c_uint = 0x0f0d;
pub const AC_VERB_GET_DIGI_CONVERT_2: c_uint = 0x0f0e /* unused */;
pub const AC_VERB_GET_VOLUME_KNOB_CONTROL: c_uint = 0x0f0f;
// f10-f1a: GPI/GPO/GPIO
pub const AC_VERB_GET_GPI_DATA: c_uint = 0x0f10;
pub const AC_VERB_GET_GPI_WAKE_MASK: c_uint = 0x0f11;
pub const AC_VERB_GET_GPI_UNSOLICITED_RSP_MASK: c_uint = 0x0f12;
pub const AC_VERB_GET_GPI_STICKY_MASK: c_uint = 0x0f13;
pub const AC_VERB_GET_GPO_DATA: c_uint = 0x0f14;
pub const AC_VERB_GET_GPIO_DATA: c_uint = 0x0f15;
pub const AC_VERB_GET_GPIO_MASK: c_uint = 0x0f16;
pub const AC_VERB_GET_GPIO_DIRECTION: c_uint = 0x0f17;
pub const AC_VERB_GET_GPIO_WAKE_MASK: c_uint = 0x0f18;
pub const AC_VERB_GET_GPIO_UNSOLICITED_RSP_MASK: c_uint = 0x0f19;
pub const AC_VERB_GET_GPIO_STICKY_MASK: c_uint = 0x0f1a;
pub const AC_VERB_GET_CONFIG_DEFAULT: c_uint = 0x0f1c;
// f20: AFG/MFG
pub const AC_VERB_GET_SUBSYSTEM_ID: c_uint = 0x0f20;
pub const AC_VERB_GET_STRIPE_CONTROL: c_uint = 0x0f24;
pub const AC_VERB_GET_CVT_CHAN_COUNT: c_uint = 0x0f2d;
pub const AC_VERB_GET_HDMI_DIP_SIZE: c_uint = 0x0f2e;
pub const AC_VERB_GET_HDMI_ELDD: c_uint = 0x0f2f;
pub const AC_VERB_GET_HDMI_DIP_INDEX: c_uint = 0x0f30;
pub const AC_VERB_GET_HDMI_DIP_DATA: c_uint = 0x0f31;
pub const AC_VERB_GET_HDMI_DIP_XMIT: c_uint = 0x0f32;
pub const AC_VERB_GET_HDMI_CP_CTRL: c_uint = 0x0f33;
pub const AC_VERB_GET_HDMI_CHAN_SLOT: c_uint = 0x0f34;
pub const AC_VERB_GET_DEVICE_SEL: c_uint = 0xf35;
pub const AC_VERB_GET_DEVICE_LIST: c_uint = 0xf36;
//
// SET verbs
//
pub const AC_VERB_SET_STREAM_FORMAT: c_uint = 0x200;
pub const AC_VERB_SET_AMP_GAIN_MUTE: c_uint = 0x300;
pub const AC_VERB_SET_PROC_COEF: c_uint = 0x400;
pub const AC_VERB_SET_COEF_INDEX: c_uint = 0x500;
pub const AC_VERB_SET_CONNECT_SEL: c_uint = 0x701;
pub const AC_VERB_SET_PROC_STATE: c_uint = 0x703;
pub const AC_VERB_SET_SDI_SELECT: c_uint = 0x704;
pub const AC_VERB_SET_POWER_STATE: c_uint = 0x705;
pub const AC_VERB_SET_CHANNEL_STREAMID: c_uint = 0x706;
pub const AC_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x707;
pub const AC_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x708;
pub const AC_VERB_SET_PIN_SENSE: c_uint = 0x709;
pub const AC_VERB_SET_BEEP_CONTROL: c_uint = 0x70a;
pub const AC_VERB_SET_EAPD_BTLENABLE: c_uint = 0x70c;
pub const AC_VERB_SET_DIGI_CONVERT_1: c_uint = 0x70d;
pub const AC_VERB_SET_DIGI_CONVERT_2: c_uint = 0x70e;
pub const AC_VERB_SET_DIGI_CONVERT_3: c_uint = 0x73e;
pub const AC_VERB_SET_VOLUME_KNOB_CONTROL: c_uint = 0x70f;
pub const AC_VERB_SET_GPI_DATA: c_uint = 0x710;
pub const AC_VERB_SET_GPI_WAKE_MASK: c_uint = 0x711;
pub const AC_VERB_SET_SPI_UNSOLICITED_RSP_MASK: c_uint = 0x712;
pub const AC_VERB_SET_GPI_STICKY_MASK: c_uint = 0x713;
pub const AC_VERB_SET_GPO_DATA: c_uint = 0x714;
pub const AC_VERB_SET_GPIO_DATA: c_uint = 0x715;
pub const AC_VERB_SET_GPIO_MASK: c_uint = 0x716;
pub const AC_VERB_SET_GPIO_DIRECTION: c_uint = 0x717;
pub const AC_VERB_SET_GPIO_WAKE_MASK: c_uint = 0x718;
pub const AC_VERB_SET_GPIO_UNSOLICITED_RSP_MASK: c_uint = 0x719;
pub const AC_VERB_SET_GPIO_STICKY_MASK: c_uint = 0x71a;
pub const AC_VERB_SET_CONFIG_DEFAULT_BYTES_0: c_uint = 0x71c;
pub const AC_VERB_SET_CONFIG_DEFAULT_BYTES_1: c_uint = 0x71d;
pub const AC_VERB_SET_CONFIG_DEFAULT_BYTES_2: c_uint = 0x71e;
pub const AC_VERB_SET_CONFIG_DEFAULT_BYTES_3: c_uint = 0x71f;
pub const AC_VERB_SET_EAPD: c_uint = 0x788;
pub const AC_VERB_SET_CODEC_RESET: c_uint = 0x7ff;
pub const AC_VERB_SET_STRIPE_CONTROL: c_uint = 0x724;
pub const AC_VERB_SET_CVT_CHAN_COUNT: c_uint = 0x72d;
pub const AC_VERB_SET_HDMI_DIP_INDEX: c_uint = 0x730;
pub const AC_VERB_SET_HDMI_DIP_DATA: c_uint = 0x731;
pub const AC_VERB_SET_HDMI_DIP_XMIT: c_uint = 0x732;
pub const AC_VERB_SET_HDMI_CP_CTRL: c_uint = 0x733;
pub const AC_VERB_SET_HDMI_CHAN_SLOT: c_uint = 0x734;
pub const AC_VERB_SET_DEVICE_SEL: c_uint = 0x735;
//
// Parameter IDs
//
pub const AC_PAR_VENDOR_ID: c_uint = 0x00;
pub const AC_PAR_SUBSYSTEM_ID: c_uint = 0x01;
pub const AC_PAR_REV_ID: c_uint = 0x02;
pub const AC_PAR_NODE_COUNT: c_uint = 0x04;
pub const AC_PAR_FUNCTION_TYPE: c_uint = 0x05;
pub const AC_PAR_AUDIO_FG_CAP: c_uint = 0x08;
pub const AC_PAR_AUDIO_WIDGET_CAP: c_uint = 0x09;
pub const AC_PAR_PCM: c_uint = 0x0a;
pub const AC_PAR_STREAM: c_uint = 0x0b;
pub const AC_PAR_PIN_CAP: c_uint = 0x0c;
pub const AC_PAR_AMP_IN_CAP: c_uint = 0x0d;
pub const AC_PAR_CONNLIST_LEN: c_uint = 0x0e;
pub const AC_PAR_POWER_STATE: c_uint = 0x0f;
pub const AC_PAR_PROC_CAP: c_uint = 0x10;
pub const AC_PAR_GPIO_CAP: c_uint = 0x11;
pub const AC_PAR_AMP_OUT_CAP: c_uint = 0x12;
pub const AC_PAR_VOL_KNB_CAP: c_uint = 0x13;
pub const AC_PAR_DEVLIST_LEN: c_uint = 0x15;
pub const AC_PAR_HDMI_LPCM_CAP: c_uint = 0x20;
//
// AC_VERB_PARAMETERS results (32bit)
//
// Function Group Type

pub const AC_FGT_TYPE_SHIFT: c_int = 0;

// Audio Function Group Capabilities

// Audio Widget Capabilities

pub const AC_WCAP_DELAY_SHIFT: c_int = 16;

pub const AC_WCAP_TYPE_SHIFT: c_int = 20;
// supported PCM rates and bits

// supported PCM stream format

// GP I/O count

pub const AC_GPIO_O_COUNT_SHIFT: c_int = 8;

pub const AC_GPIO_I_COUNT_SHIFT: c_int = 16;

// Converter stream, channel

pub const AC_CONV_STREAM_SHIFT: c_int = 4;
// Input converter SDI select

// stream format id
pub const AC_FMT_CHAN_SHIFT: c_int = 0;

pub const AC_FMT_BITS_SHIFT: c_int = 4;

pub const AC_FMT_DIV_SHIFT: c_int = 8;

pub const AC_FMT_MULT_SHIFT: c_int = 11;

pub const AC_FMT_BASE_SHIFT: c_int = 14;

pub const AC_FMT_TYPE_SHIFT: c_int = 15;

// Unsolicited response control

// Unsolicited responses

pub const AC_UNSOL_RES_TAG_SHIFT: c_int = 26;

pub const AC_UNSOL_RES_SUBTAG_SHIFT: c_int = 21;

// (for DP1.2 MST)
//
pub const AC_UNSOL_RES_DE_SHIFT: c_int = 15;

// Pin widget capabilies

// Note: This LR_SWAP pincap is defined in the Realtek ALC883 specification,
// but is marked reserved in the Intel HDA specification.
//

// Note: The same bit as LR_SWAP is newly defined as HDMI capability
// in HD-audio specification
//

// coexist with AC_PINCAP_HDMI
//

pub const AC_PINCAP_VREF_SHIFT: c_int = 8;

// Vref status (used in pin cap)

// Amplifier capabilities

pub const AC_AMPCAP_OFFSET_SHIFT: c_int = 0;

pub const AC_AMPCAP_NUM_STEPS_SHIFT: c_int = 8;

// in 0.25dB
//
pub const AC_AMPCAP_STEP_SIZE_SHIFT: c_int = 16;

pub const AC_AMPCAP_MUTE_SHIFT: c_int = 31;
// driver-specific amp-caps: using bits 24-30

// Connection list

// Supported power status

// Power state values

pub const AC_PWRST_ACTUAL_SHIFT: c_int = 4;
pub const AC_PWRST_D0: c_uint = 0x00;
pub const AC_PWRST_D1: c_uint = 0x01;
pub const AC_PWRST_D2: c_uint = 0x02;
pub const AC_PWRST_D3: c_uint = 0x03;

// Processing capabilies

pub const AC_PCAP_NUM_COEF_SHIFT: c_int = 8;
// Volume knobs capabilities

// HDMI LPCM capabilities

// Display pin's device list length
pub const AC_DEV_LIST_LEN_MASK: c_uint = 0x3f;
pub const AC_MAX_DEV_LIST_LEN: c_int = 64;
//
// Control Parameters
//
// Amp gain/mute

pub const AC_AMP_SET_INDEX_SHIFT: c_int = 8;

// DIGITAL1 bits

// DIGITAL2 bits

// DIGITAL3 bits

// Pin widget control - 8bit

pub const AC_PINCTL_EPT_NATIVE: c_int = 0;
pub const AC_PINCTL_EPT_HBR: c_int = 3;

// Pin sense - 32bit

// EAPD/BTL enable - 32bit

// HDMI ELD data

pub const AC_ELDD_ELD_DATA: c_uint = 0xff;
// HDMI DIP size

// HDMI DIP index

// HDMI DIP xmit (transmit) control

// HDMI content protection (CP) control

// Converter channel <-> HDMI slot mapping

// configuration default - 32bit

pub const AC_DEFCFG_ASSOC_SHIFT: c_int = 4;

pub const AC_DEFCFG_MISC_SHIFT: c_int = 8;

pub const AC_DEFCFG_COLOR_SHIFT: c_int = 12;

pub const AC_DEFCFG_CONN_TYPE_SHIFT: c_int = 16;

pub const AC_DEFCFG_DEVICE_SHIFT: c_int = 20;

pub const AC_DEFCFG_LOCATION_SHIFT: c_int = 24;

pub const AC_DEFCFG_PORT_CONN_SHIFT: c_int = 30;
// Display pin's device list entry

// device types (0x0-0xf)
// jack connection types (0x0-0xf)
// jack colors (0x0-0xf)
// Jack location (0x0-0x3f)
// common case
// bits 4-5
// external on primary chasis
// internal
// others
// Port connectivity (0-3)
// max. codec address
pub const HDA_MAX_CODEC_ADDRESS: c_uint = 0x0f;
