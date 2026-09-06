//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/psmouse.h
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
pub const PSMOUSE_OOB_NONE: c_uint = 0x00;
pub const PSMOUSE_OOB_EXTRA_BTNS: c_uint = 0x01;
pub const PSMOUSE_CMD_SETSCALE11: c_uint = 0x00e6;
pub const PSMOUSE_CMD_SETSCALE21: c_uint = 0x00e7;
pub const PSMOUSE_CMD_SETRES: c_uint = 0x10e8;
pub const PSMOUSE_CMD_GETINFO: c_uint = 0x03e9;
pub const PSMOUSE_CMD_SETSTREAM: c_uint = 0x00ea;
pub const PSMOUSE_CMD_SETPOLL: c_uint = 0x00f0;
pub const PSMOUSE_CMD_POLL: c_uint = 0x00eb	/* caller sets number of bytes to receive */;
pub const PSMOUSE_CMD_RESET_WRAP: c_uint = 0x00ec;
pub const PSMOUSE_CMD_GETID: c_uint = 0x02f2;
pub const PSMOUSE_CMD_SETRATE: c_uint = 0x10f3;
pub const PSMOUSE_CMD_ENABLE: c_uint = 0x00f4;
pub const PSMOUSE_CMD_DISABLE: c_uint = 0x00f5;
pub const PSMOUSE_CMD_RESET_DIS: c_uint = 0x00f6;
pub const PSMOUSE_CMD_RESET_BAT: c_uint = 0x02ff;
pub const PSMOUSE_RET_BAT: c_uint = 0xaa;
pub const PSMOUSE_RET_ID: c_uint = 0x00;
pub const PSMOUSE_RET_ACK: c_uint = 0xfa;
pub const PSMOUSE_RET_NAK: c_uint = 0xfe;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psmouse_state {
    PSMOUSE_IGNORE,
    PSMOUSE_INITIALIZING,
    PSMOUSE_RESYNCING,
    PSMOUSE_CMD_MODE,
    PSMOUSE_ACTIVATED,
}

// psmouse protocol handler return codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psmouse_scale {
    PSMOUSE_SCALE11,
    PSMOUSE_SCALE21
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psmouse_type {
    PSMOUSE_NONE,
    PSMOUSE_PS2,
    PSMOUSE_PS2PP,
    PSMOUSE_THINKPS,
    PSMOUSE_GENPS,
    PSMOUSE_IMPS,
    PSMOUSE_IMEX,
    PSMOUSE_SYNAPTICS,
    PSMOUSE_ALPS,
    PSMOUSE_LIFEBOOK,
    PSMOUSE_TRACKPOINT,
    PSMOUSE_TOUCHKIT_PS2,
    PSMOUSE_CORTRON,
    PSMOUSE_HGPK,		/* No longer used */
    PSMOUSE_ELANTECH,
    PSMOUSE_FSP,
    PSMOUSE_SYNAPTICS_RELATIVE,
    PSMOUSE_CYPRESS,
    PSMOUSE_FOCALTECH,
    PSMOUSE_VMMOUSE,
    PSMOUSE_BYD,
    PSMOUSE_SYNAPTICS_SMBUS,
    PSMOUSE_ELANTECH_SMBUS,
    PSMOUSE_AUTO		/* This one should always be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psmouse_protocol {
    pub type: psmouse_type,
    pub maxproto: bool,
    pub /: *mut *mut bool ignore_parity; / Protocol should ignore parity errors from KBC,
    pub /: *mut *mut bool try_passthru; / Try protocol also on passthrough ports,
    pub /: *mut *mut bool smbus_companion; / "Protocol" is a stub, device is on SMBus,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub bool): *mut *mut *mut int (detect)(struct psmouse ,,
    pub ): *mut *mut int (init)(struct psmouse,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psmouse {
    pub private: *mut c_void,
    pub dev: *mut input_dev,
    pub ps2dev: ps2dev,
    pub resync_work: work_struct,
    pub vendor: *const c_char,
    pub name: *const c_char,
    pub protocol: *const psmouse_protocol,
    pub packet: [c_uchar; 8],
    pub badbyte: c_uchar,
    pub pktcnt: c_uchar,
    pub pktsize: c_uchar,
    pub oob_data_type: c_uchar,
    pub extra_buttons: c_uchar,
    pub acks_disable_command: bool,
    pub model: c_uint,
    pub last: c_ulong,
    pub out_of_sync_cnt: c_ulong,
    pub num_resyncs: c_ulong,
    pub state: psmouse_state,
    pub devname: [c_char; 64],
    pub phys: [c_char; 32],
    pub rate: c_uint,
    pub resolution: c_uint,
    pub resetafter: c_uint,
    pub resync_time: c_uint,
    pub /: *mut *mut bool smartscroll; / Logitech only,
    pub psmouse): *mut *mut psmouse_ret_t (protocol_handler)(struct psmouse,
    pub rate): *mut *mut *mut void (set_rate)(struct psmouse psmouse, unsigned int,
    pub resolution): *mut *mut *mut void (set_resolution)(struct psmouse psmouse, unsigned int,
    pub scale): *mut *mut *mut void (set_scale)(struct psmouse psmouse, enum psmouse_scale,
    pub psmouse): *mut *mut int (reconnect)(struct psmouse,
    pub psmouse): *mut *mut int (fast_reconnect)(struct psmouse,
    pub psmouse): *mut *mut void (disconnect)(struct psmouse,
    pub psmouse): *mut *mut void (cleanup)(struct psmouse,
    pub psmouse): *mut *mut int (poll)(struct psmouse,
    pub psmouse): *mut *mut void (pt_activate)(struct psmouse,
    pub psmouse): *mut *mut void (pt_deactivate)(struct psmouse,
}

extern "C" {
    pub fn psmouse_reset(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn psmouse_set_state(psmouse: *mut psmouse, new_state: psmouse_state);
}
extern "C" {
    pub fn psmouse_set_resolution(psmouse: *mut psmouse, resolution: c_uint);
}
extern "C" {
    pub fn psmouse_process_byte(psmouse: *mut psmouse) -> psmouse_ret_t;
}
extern "C" {
    pub fn psmouse_activate(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn psmouse_deactivate(psmouse: *mut psmouse) -> c_int;
}
extern "C" {
    pub fn psmouse_matches_pnp_id(psmouse: *mut psmouse, ids[]: *const *const c_char) -> bool;
}
extern "C" {
    pub fn psmouse_report_standard_buttons(: *mut input_dev, buttons: u8);
}
extern "C" {
    pub fn psmouse_report_standard_motion(: *mut input_dev, packet: *mut u8);
}
extern "C" {
    pub fn psmouse_report_standard_packet(: *mut input_dev, packet: *mut u8);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psmouse_attribute {
    pub dattr: device_attribute,
    pub data: *mut c_void,
    pub buf): *mut *mut *mut *mut ssize_t (show)(struct psmouse psmouse, void data, char,
    pub count): *const *const char buf, size_t,
    pub protect: bool,
}

extern "C" {
    pub fn psmouse_smbus_module_init() -> c_int;
}
extern "C" {
    pub fn psmouse_smbus_module_exit();
}
extern "C" {
    pub fn psmouse_smbus_cleanup(psmouse: *mut psmouse);
}

