//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/msp3400-driver.h
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

// ----------------------------------------------------------------------
// This macro is allowed for *constants* only, gcc must calculate it

pub const MSP_MODE_AM_DETECT: c_int = 0;
pub const MSP_MODE_FM_RADIO: c_int = 2;
pub const MSP_MODE_FM_TERRA: c_int = 3;
pub const MSP_MODE_FM_SAT: c_int = 4;
pub const MSP_MODE_FM_NICAM1: c_int = 5;
pub const MSP_MODE_FM_NICAM2: c_int = 6;
pub const MSP_MODE_AM_NICAM: c_int = 7;
pub const MSP_MODE_BTSC: c_int = 8;
pub const MSP_MODE_EXTERN: c_int = 9;
pub const SCART_IN1: c_int = 0;
pub const SCART_IN2: c_int = 1;
pub const SCART_IN3: c_int = 2;
pub const SCART_IN4: c_int = 3;
pub const SCART_IN1_DA: c_int = 4;
pub const SCART_IN2_DA: c_int = 5;
pub const SCART_MONO: c_int = 6;
pub const SCART_MUTE: c_int = 7;
pub const SCART_DSP_IN: c_int = 0;
pub const SCART1_OUT: c_int = 1;
pub const SCART2_OUT: c_int = 2;

pub const OPMODE_MANUAL: c_int = 0;

// module parameters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msp3400_pads {
    MSP3400_PAD_IF_INPUT,
    MSP3400_PAD_OUT,
    MSP3400_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msp_state {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub rev2: int rev1,,
    pub ident: c_int,
    pub has_nicam: u8,
    pub has_radio: u8,
    pub has_headphones: u8,
    pub has_ntsc_jp_d_k3: u8,
    pub has_scart2: u8,
    pub has_scart3: u8,
    pub has_scart4: u8,
    pub has_scart2_out: u8,
    pub has_scart2_out_volume: u8,
    pub has_i2s_conf: u8,
    pub has_subwoofer: u8,
    pub has_sound_processing: u8,
    pub has_virtual_dolby_surround: u8,
    pub has_dolby_pro_logic: u8,
    pub force_btsc: u8,
    pub radio: c_int,
    pub opmode: c_int,
    pub std: c_int,
    pub mode: c_int,
    pub detected_std: v4l2_std_id v4l2_std,,
    pub nicam_on: c_int,
    pub acb: c_int,
    pub in_scart: c_int,
    pub i2s_mode: c_int,
    pub /: *mut *mut int main, second; / sound carrier,
    pub input: c_int,
    pub route_in: u32,
    pub route_out: u32,
// v4l2
    pub audmode: c_int,
    pub rxsubchans: c_int,
// volume cluster
    pub volume: *mut v4l2_ctrl,
    pub muted: *mut v4l2_ctrl,
}

// thread

extern "C" {
    pub fn container_of(_arg: sd, msp_state: struct, _arg: sd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ctrl->handler, msp_state: struct, _arg: hdl) -> return;
}
// msp3400-driver.c
extern "C" {
    pub fn msp_write_dem(client: *mut i2c_client, addr: c_int, val: c_int) -> c_int;
}
extern "C" {
    pub fn msp_write_dsp(client: *mut i2c_client, addr: c_int, val: c_int) -> c_int;
}
extern "C" {
    pub fn msp_read_dem(client: *mut i2c_client, addr: c_int) -> c_int;
}
extern "C" {
    pub fn msp_read_dsp(client: *mut i2c_client, addr: c_int) -> c_int;
}
extern "C" {
    pub fn msp_reset(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn msp_set_scart(client: *mut i2c_client, in: c_int, out: c_int);
}
extern "C" {
    pub fn msp_update_volume(state: *mut msp_state);
}
extern "C" {
    pub fn msp_sleep(state: *mut msp_state, msec: c_int) -> c_int;
}
// msp3400-kthreads.c
extern "C" {
    pub fn msp_set_audmode(client: *mut i2c_client);
}
extern "C" {
    pub fn msp_detect_stereo(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn msp3400c_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn msp3410d_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn msp34xxg_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn msp3400c_set_mode(client: *mut i2c_client, mode: c_int);
}
extern "C" {
    pub fn msp3400c_set_carrier(client: *mut i2c_client, cdo1: c_int, cdo2: c_int);
}
