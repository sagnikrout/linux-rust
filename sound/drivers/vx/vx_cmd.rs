//! Automatically rewritten from C Header to Rust Module
//! Source: sound/drivers/vx/vx_cmd.h
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
// Driver for Digigram VX soundcards
//
// Definitions of DSP commands
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//
// CMD_SET_STREAM_OUT_EFFECTS,
// CMD_GET_STREAM_OUT_EFFECTS,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vx_cmd_info {
    pub /: *mut *mut unsigned int opcode; / command word,
    pub /: *mut *mut int length; / command length (in words),
    pub /: *mut *mut int st_type; / status type (RMH_SSIZE_XXX),
    pub /: *mut *mut int st_length; / fixed length,
}

// Family and code op of some DSP requests.
pub const CODE_OP_PIPE_TIME: c_uint = 0x004e0000;
pub const CODE_OP_START_STREAM: c_uint = 0x00800000;
pub const CODE_OP_PAUSE_STREAM: c_uint = 0x00810000;
pub const CODE_OP_OUT_STREAM_LEVEL: c_uint = 0x00820000;
pub const CODE_OP_UPDATE_R_BUFFERS: c_uint = 0x00840000;
pub const CODE_OP_OUT_STREAM1_LEVEL_CURVE: c_uint = 0x00850000;
pub const CODE_OP_OUT_STREAM2_LEVEL_CURVE: c_uint = 0x00930000;
pub const CODE_OP_OUT_STREAM_FORMAT: c_uint = 0x00860000;
pub const CODE_OP_STREAM_TIME: c_uint = 0x008f0000;
pub const CODE_OP_OUT_STREAM_EXTRAPARAMETER: c_uint = 0x00910000;
pub const CODE_OP_OUT_AUDIO_LEVEL: c_uint = 0x00c20000;
pub const NOTIFY_LAST_COMMAND: c_uint = 0x00400000;
// Values for a user delay

// Values for tiDelayed field in TIME_INFO structure,
// and for pbPause field in PLAY_BUFFER_INFO structure
//
pub const BIT_DIFFERED_COMMAND: c_int = 0;
pub const BIT_NOTIFIED_COMMAND: c_int = 1;
pub const BIT_TIME_RELATIVE_TO_BUFFER: c_int = 2;
pub const BIT_RESERVED: c_int = 3;
pub const BIT_STREAM_TIME: c_int = 4;
pub const BIT_CANCELLED_COMMAND: c_int = 5;
// Access to the "Size" field of the response of the CMD_GET_NOTIFY_EVENT request.
pub const GET_NOTIFY_EVENT_SIZE_FIELD_MASK: c_uint = 0x000000ff;
// DSP commands general masks
pub const OPCODE_MASK: c_uint = 0x00ff0000;
pub const DSP_DIFFERED_COMMAND_MASK: c_uint = 0x0000C000;
// Notifications (NOTIFY_INFO)
pub const ALL_CMDS_NOTIFIED: c_uint = 0x0000  // reserved;
pub const START_STREAM_NOTIFIED: c_uint = 0x0001;
pub const PAUSE_STREAM_NOTIFIED: c_uint = 0x0002;
pub const OUT_STREAM_LEVEL_NOTIFIED: c_uint = 0x0003;
pub const OUT_STREAM_PARAMETER_NOTIFIED: c_uint = 0x0004  // left for backward compatibility;
pub const OUT_STREAM_FORMAT_NOTIFIED: c_uint = 0x0004;
pub const PIPE_TIME_NOTIFIED: c_uint = 0x0005;
pub const OUT_AUDIO_LEVEL_NOTIFIED: c_uint = 0x0006;
pub const OUT_STREAM_LEVEL_CURVE_NOTIFIED: c_uint = 0x0007;
pub const STREAM_TIME_NOTIFIED: c_uint = 0x0008;
pub const OUT_STREAM_EXTRAPARAMETER_NOTIFIED: c_uint = 0x0009;
pub const UNKNOWN_COMMAND_NOTIFIED: c_uint = 0xffff;
// Output pipe parameters setting
pub const MASK_VALID_PIPE_MPEG_PARAM: c_uint = 0x000040;
pub const MASK_VALID_PIPE_BACKWARD_PARAM: c_uint = 0x000020;
pub const MASK_SET_PIPE_MPEG_PARAM: c_uint = 0x000002;
pub const MASK_SET_PIPE_BACKWARD_PARAM: c_uint = 0x000001;
pub const MASK_DSP_WORD: c_uint = 0x00FFFFFF;
pub const MASK_ALL_STREAM: c_uint = 0x00FFFFFF;
pub const MASK_DSP_WORD_LEVEL: c_uint = 0x000001FF;
pub const MASK_FIRST_FIELD: c_uint = 0x0000001F;
pub const FIELD_SIZE: c_int = 5;
pub const COMMAND_RECORD_MASK: c_uint = 0x000800;
// PipeManagement definition bits (PIPE_DECL_INFO)
pub const P_UNDERRUN_SKIP_SOUND_MASK: c_uint = 0x01;
pub const P_PREPARE_FOR_MPEG3_MASK: c_uint = 0x02;
pub const P_DO_NOT_RESET_ANALOG_LEVELS: c_uint = 0x04;
pub const P_ALLOW_UNDER_ALLOCATION_MASK: c_uint = 0x08;
pub const P_DATA_MODE_MASK: c_uint = 0x10;
pub const P_ASIO_BUFFER_MANAGEMENT_MASK: c_uint = 0x20;
pub const BIT_SKIP_SOUND: c_uint = 0x08	// bit 3;
pub const BIT_DATA_MODE: c_uint = 0x10	// bit 4;
// Bits in the CMD_MODIFY_CLOCK request.
pub const CMD_MODIFY_CLOCK_FD_BIT: c_uint = 0x00000001;
pub const CMD_MODIFY_CLOCK_T_BIT: c_uint = 0x00000002;
pub const CMD_MODIFY_CLOCK_S_BIT: c_uint = 0x00000004;
// Access to the results of the CMD_GET_TIME_CODE RMH.
pub const TIME_CODE_V_MASK: c_uint = 0x00800000;
pub const TIME_CODE_N_MASK: c_uint = 0x00400000;
pub const TIME_CODE_B_MASK: c_uint = 0x00200000;
pub const TIME_CODE_W_MASK: c_uint = 0x00100000;
// Values for the CMD_MANAGE_SIGNAL RMH.
pub const MANAGE_SIGNAL_TIME_CODE: c_uint = 0x01;
pub const MANAGE_SIGNAL_MIDI: c_uint = 0x02;
// Values for the CMD_CONFIG_TIME_CODE RMH.
pub const CONFIG_TIME_CODE_CANCEL: c_uint = 0x00001000;
// Mask to get only the effective time from the
// high word out of the 2 returned by the DSP
//
pub const PCX_TIME_HI_MASK: c_uint = 0x000fffff;
// Values for setting a H-Buffer time
pub const HBUFFER_TIME_HIGH: c_uint = 0x00200000;
pub const HBUFFER_TIME_LOW: c_uint = 0x00000000;
pub const NOTIFY_MASK_TIME_HIGH: c_uint = 0x00400000;
pub const MULTIPLE_MASK_TIME_HIGH: c_uint = 0x00100000;
pub const STREAM_MASK_TIME_HIGH: c_uint = 0x00800000;
//
extern "C" {
    pub fn vx_init_rmh(rmh: *mut vx_rmh, cmd: c_uint);
}
//
// vx_set_pipe_cmd_params - fill first command word for pipe commands
// @rmh: the rmh to be modified
// @is_capture: 0 = playback, 1 = capture operation
// @param1: first pipe-parameter
// @param2: second pipe-parameter
//
// vx_set_stream_cmd_params - fill first command word for stream commands
// @rmh: the rmh to be modified
// @is_capture: 0 = playback, 1 = capture operation
// @pipe: the pipe index (zero-based)
//
