//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-wiimote.h
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
// HID driver for Nintendo Wii / Wii U peripherals
// Copyright (c) 2011-2013 David Herrmann <dh.herrmann@gmail.com>
//

pub const WIIMOTE_BUFSIZE: c_int = 32;
pub const WIIPROTO_FLAG_LED1: c_uint = 0x01;
pub const WIIPROTO_FLAG_LED2: c_uint = 0x02;
pub const WIIPROTO_FLAG_LED3: c_uint = 0x04;
pub const WIIPROTO_FLAG_LED4: c_uint = 0x08;
pub const WIIPROTO_FLAG_RUMBLE: c_uint = 0x10;
pub const WIIPROTO_FLAG_ACCEL: c_uint = 0x20;
pub const WIIPROTO_FLAG_IR_BASIC: c_uint = 0x40;
pub const WIIPROTO_FLAG_IR_EXT: c_uint = 0x80;
pub const WIIPROTO_FLAG_IR_FULL: c_uint = 0xc0 /* IR_BASIC | IR_EXT */;
pub const WIIPROTO_FLAG_EXT_PLUGGED: c_uint = 0x0100;
pub const WIIPROTO_FLAG_EXT_USED: c_uint = 0x0200;
pub const WIIPROTO_FLAG_EXT_ACTIVE: c_uint = 0x0400;
pub const WIIPROTO_FLAG_MP_PLUGGED: c_uint = 0x0800;
pub const WIIPROTO_FLAG_MP_USED: c_uint = 0x1000;
pub const WIIPROTO_FLAG_MP_ACTIVE: c_uint = 0x2000;
pub const WIIPROTO_FLAG_EXITING: c_uint = 0x4000;
pub const WIIPROTO_FLAG_DRM_LOCKED: c_uint = 0x8000;
pub const WIIPROTO_FLAG_BUILTIN_MP: c_uint = 0x010000;
pub const WIIPROTO_FLAG_NO_MP: c_uint = 0x020000;
pub const WIIPROTO_FLAG_PRO_CALIB_DONE: c_uint = 0x040000;

// return flag for led \num

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiiproto_keys {
    WIIPROTO_KEY_LEFT,
    WIIPROTO_KEY_RIGHT,
    WIIPROTO_KEY_UP,
    WIIPROTO_KEY_DOWN,
    WIIPROTO_KEY_PLUS,
    WIIPROTO_KEY_MINUS,
    WIIPROTO_KEY_ONE,
    WIIPROTO_KEY_TWO,
    WIIPROTO_KEY_A,
    WIIPROTO_KEY_B,
    WIIPROTO_KEY_HOME,
    WIIPROTO_KEY_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiimote_devtype {
    WIIMOTE_DEV_PENDING,
    WIIMOTE_DEV_UNKNOWN,
    WIIMOTE_DEV_GENERIC,
    WIIMOTE_DEV_GEN10,
    WIIMOTE_DEV_GEN20,
    WIIMOTE_DEV_BALANCE_BOARD,
    WIIMOTE_DEV_PRO_CONTROLLER,
    WIIMOTE_DEV_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiimote_exttype {
    WIIMOTE_EXT_NONE,
    WIIMOTE_EXT_UNKNOWN,
    WIIMOTE_EXT_NUNCHUK,
    WIIMOTE_EXT_CLASSIC_CONTROLLER,
    WIIMOTE_EXT_BALANCE_BOARD,
    WIIMOTE_EXT_PRO_CONTROLLER,
    WIIMOTE_EXT_DRUMS,
    WIIMOTE_EXT_GUITAR,
    WIIMOTE_EXT_TURNTABLE,
    WIIMOTE_EXT_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiimote_mptype {
    WIIMOTE_MP_NONE,
    WIIMOTE_MP_UNKNOWN,
    WIIMOTE_MP_SINGLE,
    WIIMOTE_MP_PASSTHROUGH_NUNCHUK,
    WIIMOTE_MP_PASSTHROUGH_CLASSIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimote_buf {
    pub data: [__u8; HID_MAX_BUFFER_SIZE],
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimote_queue {
    pub lock: spinlock_t,
    pub worker: work_struct,
    pub head: __u8,
    pub tail: __u8,
    pub outq: [wiimote_buf; WIIMOTE_BUFSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimote_state {
    pub lock: spinlock_t,
    pub flags: __u32,
    pub accel_split: [__u8; 2],
    pub drm: __u8,
    pub devtype: __u8,
    pub exttype: __u8,
    pub mp: __u8,
// synchronous cmd requests
    pub sync: mutex,
    pub ready: completion,
    pub cmd: c_int,
    pub opt: __u32,
// results of synchronous requests
    pub cmd_battery: __u8,
    pub cmd_err: __u8,
    pub cmd_read_buf: *mut __u8,
    pub cmd_read_size: __u8,
// calibration/cache data
    pub calib_bboard: [__u16; 4][3],
    pub calib_pro_sticks: [__s16; 4],
    pub pressure_drums: [__u8; 7],
    pub cache_rumble: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimote_data {
    pub hdev: *mut hid_device,
    pub input: *mut input_dev,
    pub rumble_worker: work_struct,
    pub leds: [*mut led_classdev; 4],
    pub accel: *mut input_dev,
    pub ir: *mut input_dev,
    pub battery: *mut power_supply,
    pub battery_desc: power_supply_desc,
    pub mp: *mut input_dev,
    pub timer: timer_list,
    pub debug: *mut wiimote_debug,
    pub input: *mut input_dev,
    pub extension: },
    pub queue: wiimote_queue,
    pub state: wiimote_state,
    pub init_worker: work_struct,
}

// wiimote modules
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiimod_module {
    WIIMOD_KEYS,
    WIIMOD_RUMBLE,
    WIIMOD_BATTERY,
    WIIMOD_LED1,
    WIIMOD_LED2,
    WIIMOD_LED3,
    WIIMOD_LED4,
    WIIMOD_ACCEL,
    WIIMOD_IR,
    WIIMOD_BUILTIN_MP,
    WIIMOD_NO_MP,
    WIIMOD_NUM,
    WIIMOD_NULL = WIIMOD_NUM,
}

pub const WIIMOD_FLAG_INPUT: c_uint = 0x0001;
pub const WIIMOD_FLAG_EXT8: c_uint = 0x0002;
pub const WIIMOD_FLAG_EXT16: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wiimod_ops {
    pub flags: __u16,
    pub arg: c_ulong,
    pub wdata): *mut wiimote_data,
    pub wdata): *mut wiimote_data,
    pub keys): *const *const *const void (in_keys) (struct wiimote_data wdata, __u8,
    pub accel): *const *const *const void (in_accel) (struct wiimote_data wdata, __u8,
    pub id): c_uint,
    pub mp): *const *const *const void (in_mp) (struct wiimote_data wdata, __u8,
    pub ext): *const *const *const void (in_ext) (struct wiimote_data wdata, __u8,
}

// wiimote requests
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wiiproto_reqs {
    WIIPROTO_REQ_NULL = 0x0,
    WIIPROTO_REQ_RUMBLE = 0x10,
    WIIPROTO_REQ_LED = 0x11,
    WIIPROTO_REQ_DRM = 0x12,
    WIIPROTO_REQ_IR1 = 0x13,
    WIIPROTO_REQ_SREQ = 0x15,
    WIIPROTO_REQ_WMEM = 0x16,
    WIIPROTO_REQ_RMEM = 0x17,
    WIIPROTO_REQ_IR2 = 0x1a,
    WIIPROTO_REQ_STATUS = 0x20,
    WIIPROTO_REQ_DATA = 0x21,
    WIIPROTO_REQ_RETURN = 0x22,

// DRM_K: BB*2
    WIIPROTO_REQ_DRM_K = 0x30,

// DRM_KA: BB*2 AA*3
    WIIPROTO_REQ_DRM_KA = 0x31,

// DRM_KE: BB*2 EE*8
    WIIPROTO_REQ_DRM_KE = 0x32,

// DRM_KAI: BB*2 AA*3 II*12
    WIIPROTO_REQ_DRM_KAI = 0x33,

// DRM_KEE: BB*2 EE*19
    WIIPROTO_REQ_DRM_KEE = 0x34,

// DRM_KAE: BB*2 AA*3 EE*16
    WIIPROTO_REQ_DRM_KAE = 0x35,

// DRM_KIE: BB*2 II*10 EE*9
    WIIPROTO_REQ_DRM_KIE = 0x36,

// DRM_KAIE: BB*2 AA*3 II*10 EE*6
    WIIPROTO_REQ_DRM_KAIE = 0x37,

// DRM_E: EE*21
    WIIPROTO_REQ_DRM_E = 0x3d,

// DRM_SKAI1: BB*2 AA*1 II*18
    WIIPROTO_REQ_DRM_SKAI1 = 0x3e,

// DRM_SKAI2: BB*2 AA*1 II*18
    WIIPROTO_REQ_DRM_SKAI2 = 0x3f,

    WIIPROTO_REQ_MAX
}

extern "C" {
    pub fn __wiimote_schedule(wdata: *mut wiimote_data);
}
extern "C" {
    pub fn wiiproto_req_drm(wdata: *mut wiimote_data, drm: __u8);
}
extern "C" {
    pub fn wiiproto_req_rumble(wdata: *mut wiimote_data, rumble: __u8);
}
extern "C" {
    pub fn wiiproto_req_leds(wdata: *mut wiimote_data, leds: c_int);
}
extern "C" {
    pub fn wiiproto_req_status(wdata: *mut wiimote_data);
}
extern "C" {
    pub fn wiiproto_req_accel(wdata: *mut wiimote_data, accel: __u8);
}
extern "C" {
    pub fn wiiproto_req_ir1(wdata: *mut wiimote_data, flags: __u8);
}
extern "C" {
    pub fn wiiproto_req_ir2(wdata: *mut wiimote_data, flags: __u8);
}

extern "C" {
    pub fn wiidebug_init(wdata: *mut wiimote_data) -> c_int;
}
extern "C" {
    pub fn wiidebug_deinit(wdata: *mut wiimote_data);
}

// requires the state.lock spinlock to be held
// Abort synchronous request by waking up the sleeping caller. But
// reset the state.cmd field to an invalid value so no further event
// handlers will work with it.
// requires the state.lock spinlock to be held
// The completion acts as implicit memory barrier so we can safely
// assume that state.cmd is set on success/failure and isn't accessed
// by any other thread, anymore.
// no locking needed; see wiimote_cmd_wait()
