//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/mixart/mixart_core.h
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
// Driver for Digigram miXart soundcards
//
// low level interface with interrupt handling and mail box implementation
//
// Copyright (c) 2003 by Digigram <alsa@digigram.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_message_id {
    MSG_CONNECTOR_GET_AUDIO_INFO         = 0x050008,
    MSG_CONNECTOR_GET_OUT_AUDIO_LEVEL    = 0x050009,
    MSG_CONNECTOR_SET_OUT_AUDIO_LEVEL    = 0x05000A,

    MSG_CONSOLE_MANAGER                  = 0x070000,
    MSG_CONSOLE_GET_CLOCK_UID            = 0x070003,

    MSG_PHYSICALIO_SET_LEVEL             = 0x0F0008,

    MSG_STREAM_ADD_INPUT_GROUP           = 0x130000,
    MSG_STREAM_ADD_OUTPUT_GROUP          = 0x130001,
    MSG_STREAM_DELETE_GROUP              = 0x130004,
    MSG_STREAM_START_STREAM_GRP_PACKET   = 0x130006,
    MSG_STREAM_START_INPUT_STAGE_PACKET  = 0x130007,
    MSG_STREAM_START_OUTPUT_STAGE_PACKET = 0x130008,
    MSG_STREAM_STOP_STREAM_GRP_PACKET    = 0x130009,
    MSG_STREAM_STOP_INPUT_STAGE_PACKET   = 0x13000A,
    MSG_STREAM_STOP_OUTPUT_STAGE_PACKET  = 0x13000B,
    MSG_STREAM_SET_INPUT_STAGE_PARAM     = 0x13000F,
    MSG_STREAM_SET_OUTPUT_STAGE_PARAM    = 0x130010,
    MSG_STREAM_SET_IN_AUDIO_LEVEL        = 0x130015,
    MSG_STREAM_SET_OUT_STREAM_LEVEL      = 0x130017,

    MSG_SYSTEM_FIRST_ID                  = 0x160000,
    MSG_SYSTEM_ENUM_PHYSICAL_IO          = 0x16000E,
    MSG_SYSTEM_ENUM_PLAY_CONNECTOR       = 0x160017,
    MSG_SYSTEM_ENUM_RECORD_CONNECTOR     = 0x160018,
    MSG_SYSTEM_WAIT_SYNCHRO_CMD          = 0x16002C,
    MSG_SYSTEM_SEND_SYNCHRO_CMD          = 0x16002D,

    MSG_SERVICES_TIMER_NOTIFY            = 0x1D0404,
    MSG_SERVICES_REPORT_TRACES           = 0x1D0700,

    MSG_CLOCK_CHECK_PROPERTIES           = 0x200001,
    MSG_CLOCK_SET_PROPERTIES             = 0x200002,
}

pub const MSG_DEFAULT_SIZE: c_int = 512;
// structs used to communicate with miXart
// used for following struct
pub const MIXART_FLOAT_P_22_0_TO_HEX: c_uint = 0x41b00000  /* 22.0f */;
pub const MIXART_FLOAT_M_20_0_TO_HEX: c_uint = 0xc1a00000  /* -20.0f */;
pub const MIXART_FLOAT____0_0_TO_HEX: c_uint = 0x00000000  /* 0.0f */;
// used for nb_bytes_max_per_sample
pub const MIXART_FLOAT_P__4_0_TO_HEX: c_uint = 0x40800000  /* +4.0f */;
pub const MIXART_FLOAT_P__8_0_TO_HEX: c_uint = 0x41000000  /* +8.0f */;
// MSG_STREAM_ADD_INPUT_GROUP
// MSG_STREAM_ADD_OUTPUT_GROUP
// MSG_STREAM_DELETE_GROUP
// request : mixart_uid_t group
// MSG_STREAM_START_INPUT_STAGE_PACKET  = 0x130000 + 7,
//
// MSG_STREAM_START_STREAM_GRP_PACKET   = 0x130000 + 6
//
// Structures used by the MSG_SERVICES_TIMER_NOTIFY command
//
// This structure is limited by the size of MSG_DEFAULT_SIZE. Instead of
// having MIXART_MAX_STREAM_PER_CARD * MIXART_MAX_CARDS many streams,
// this is capped to have a total size below MSG_DEFAULT_SIZE.
//

// MSG_CONSOLE_GET_CLOCK_UID            = 0x070003,
//
// request is a uid with desc = MSG_CONSOLE_MANAGER | cardindex
// MSG_CLOCK_CHECK_PROPERTIES           = 0x200001,
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_clock_generic_type {
    CGT_NO_CLOCK,
    CGT_INTERNAL_CLOCK,
    CGT_PROGRAMMABLE_CLOCK,
    CGT_INTERNAL_ENSLAVED_CLOCK,
    CGT_EXTERNAL_CLOCK,
    CGT_CURRENT_CLOCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_clock_mode {
    CM_UNDEFINED,
    CM_MASTER,
    CM_SLAVE,
    CM_STANDALONE,
    CM_NOT_CONCERNED
}

// MSG_STREAM_SET_INPUT_STAGE_PARAM     = 0x13000F
// MSG_STREAM_SET_OUTPUT_STAGE_PARAM    = 0x130010
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_coding_type {
    CT_NOT_DEFINED,
    CT_LINEAR,
    CT_MPEG_L1,
    CT_MPEG_L2,
    CT_MPEG_L3,
    CT_MPEG_L3_LSF,
    CT_GSM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_sample_type {
    ST_NOT_DEFINED,
    ST_FLOATING_POINT_32BE,
    ST_FLOATING_POINT_32LE,
    ST_FLOATING_POINT_64BE,
    ST_FLOATING_POINT_64LE,
    ST_FIXED_POINT_8,
    ST_FIXED_POINT_16BE,
    ST_FIXED_POINT_16LE,
    ST_FIXED_POINT_24BE,
    ST_FIXED_POINT_24LE,
    ST_FIXED_POINT_32BE,
    ST_FIXED_POINT_32LE,
    ST_INTEGER_8,
    ST_INTEGER_16BE,
    ST_INTEGER_16LE,
    ST_INTEGER_24BE,
    ST_INTEGER_24LE,
    ST_INTEGER_32BE,
    ST_INTEGER_32LE
}

// MSG_CONNECTOR_GET_OUT_AUDIO_LEVEL    = 0x050009,
//
// MSG_CONNECTOR_SET_OUT_AUDIO_LEVEL    = 0x05000A,
//
// used for valid_mask below
pub const MIXART_AUDIO_LEVEL_ANALOG_MASK: c_uint = 0x01;
pub const MIXART_AUDIO_LEVEL_DIGITAL_MASK: c_uint = 0x02;
pub const MIXART_AUDIO_LEVEL_MONITOR_MASK: c_uint = 0x04;
pub const MIXART_AUDIO_LEVEL_MUTE_MASK: c_uint = 0x08;
pub const MIXART_AUDIO_LEVEL_MUTE_M1_MASK: c_uint = 0x10;
pub const MIXART_AUDIO_LEVEL_MUTE_M2_MASK: c_uint = 0x20;
// MSG_SYSTEM_ENUM_PHYSICAL_IO          = 0x16000E,
//

// MSG_PHYSICALIO_SET_LEVEL             = 0x0F0008,
//
// MSG_STREAM_SET_IN_AUDIO_LEVEL        = 0x130015,
//
// response is a 32 bit status
// MSG_STREAM_SET_OUT_STREAM_LEVEL      = 0x130017,
//
// defines used for valid_mask1
pub const MIXART_OUT_STREAM_SET_LEVEL_LEFT_AUDIO1: c_uint = 0x01;
pub const MIXART_OUT_STREAM_SET_LEVEL_LEFT_AUDIO2: c_uint = 0x02;
pub const MIXART_OUT_STREAM_SET_LEVEL_RIGHT_AUDIO1: c_uint = 0x04;
pub const MIXART_OUT_STREAM_SET_LEVEL_RIGHT_AUDIO2: c_uint = 0x08;
pub const MIXART_OUT_STREAM_SET_LEVEL_STREAM_1: c_uint = 0x10;
pub const MIXART_OUT_STREAM_SET_LEVEL_STREAM_2: c_uint = 0x20;
pub const MIXART_OUT_STREAM_SET_LEVEL_MUTE_1: c_uint = 0x40;
pub const MIXART_OUT_STREAM_SET_LEVEL_MUTE_2: c_uint = 0x80;
// response to this request is a u32 status value
// exported
extern "C" {
    pub fn snd_mixart_init_mailbox(mgr: *mut mixart_mgr);
}
extern "C" {
    pub fn snd_mixart_exit_mailbox(mgr: *mut mixart_mgr);
}
extern "C" {
    pub fn snd_mixart_send_msg(mgr: *mut mixart_mgr, request: *mut mixart_msg, max_resp_size: c_int, resp_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn snd_mixart_send_msg_wait_notif(mgr: *mut mixart_mgr, request: *mut mixart_msg, notif_event: u32) -> c_int;
}
extern "C" {
    pub fn snd_mixart_send_msg_nonblock(mgr: *mut mixart_mgr, request: *mut mixart_msg) -> c_int;
}
extern "C" {
    pub fn snd_mixart_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_mixart_threaded_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_mixart_reset_board(mgr: *mut mixart_mgr);
}
