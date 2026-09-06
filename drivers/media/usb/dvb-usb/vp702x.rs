//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/vp702x.h
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

// commands are read and written with USB control messages
// consecutive read/write operation
pub const REQUEST_OUT: c_uint = 0xB2;
pub const REQUEST_IN: c_uint = 0xB3;
// the out-buffer of these consecutive operations contain sub-commands when b[0] = 0
// request: 0xB2; i: 0; v: 0; b[0] = 0, b[1] = subcmd, additional buffer
// the returning buffer looks as follows
// request: 0xB3; i: 0; v: 0; b[0] = 0xB3, additional buffer
pub const GET_TUNER_STATUS: c_uint = 0x05;
// additional in buffer:
// 0   1   2    3              4   5   6               7       8
// N/A N/A 0x05 signal-quality N/A N/A signal-strength lock==0 N/A
pub const GET_SYSTEM_STRING: c_uint = 0x06;
// additional in buffer:
// 0   1   2   3   4   5   6   7   8
// N/A 'U' 'S' 'B' '7' '0' '2' 'X' N/A
pub const SET_DISEQC_CMD: c_uint = 0x08;
// additional out buffer:
// 0    1  2  3  4
// len  X1 X2 X3 X4
// additional in buffer:
// 0   1 2
// N/A 0 0   b[1] == b[2] == 0 -> success, failure otherwise
pub const SET_LNB_POWER: c_uint = 0x09;
// additional out buffer:
// 0    1    2
// 0x00 0xff 1 = on, 0 = off
// additional in buffer:
// 0   1 2
// N/A 0 0   b[1] == b[2] == 0 -> success failure otherwise
pub const GET_MAC_ADDRESS: c_uint = 0x0A;
// #define GET_MAC_ADDRESS   0x0B
// additional in buffer:
// 0   1   2            3    4    5    6    7    8
// N/A N/A 0x0A or 0x0B MAC0 MAC1 MAC2 MAC3 MAC4 MAC5
pub const SET_PID_FILTER: c_uint = 0x11;
// additional in buffer:
// 0        1        ... 14       15       16
// PID0_MSB PID0_LSB ... PID7_MSB PID7_LSB PID_active (bits)
// request: 0xB2; i: 0; v: 0;
// b[0] != 0 -> tune and lock a channel
// 0     1     2       3      4      5      6    7
// freq0 freq1 divstep srate0 srate1 srate2 flag chksum
//
// one direction requests
pub const READ_REMOTE_REQ: c_uint = 0xB4;
// IN  i: 0; v: 0; b[0] == request, b[1] == key
pub const READ_PID_NUMBER_REQ: c_uint = 0xB5;
// IN  i: 0; v: 0; b[0] == request, b[1] == 0, b[2] = pid number
pub const WRITE_EEPROM_REQ: c_uint = 0xB6;
// OUT i: offset; v: value to write; no extra buffer
pub const READ_EEPROM_REQ: c_uint = 0xB7;
// IN  i: bufferlen; v: offset; buffer with bufferlen bytes
pub const READ_STATUS: c_uint = 0xB8;
// IN  i: 0; v: 0; bufferlen 10
pub const READ_TUNER_REG_REQ: c_uint = 0xB9;
// IN  i: 0; v: register; b[0] = value
pub const READ_FX2_REG_REQ: c_uint = 0xBA;
// IN  i: offset; v: 0; b[0] = value
pub const WRITE_FX2_REG_REQ: c_uint = 0xBB;
// OUT i: offset; v: value to write; 1 byte extra buffer
pub const SET_TUNER_POWER_REQ: c_uint = 0xBC;
// IN  i: 0 = power off, 1 = power on
pub const WRITE_TUNER_REG_REQ: c_uint = 0xBD;
// IN  i: register, v: value to write, no extra buffer
pub const RESET_TUNER: c_uint = 0xBE;
// IN  i: 0, v: 0, no extra buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp702x_device_state {
    pub buf_mutex: mutex,
    pub buf_len: c_int,
    pub buf: *mut u8,
}

extern "C" {
    pub fn vp702x_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend;
}
extern "C" {
    pub fn vp702x_usb_inout_op(d: *mut dvb_usb_device, o: *mut u8, olen: c_int, i: *mut u8, ilen: c_int, msec: c_int) -> c_int;
}
extern "C" {
    pub fn vp702x_usb_in_op(d: *mut dvb_usb_device, req: u8, value: u16, index: u16, b: *mut u8, blen: c_int) -> c_int;
}
