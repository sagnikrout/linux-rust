//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/css.h
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
// path grouping stuff
//
pub const SPID_FUNC_SINGLE_PATH: c_uint = 0x00;
pub const SPID_FUNC_MULTI_PATH: c_uint = 0x80;
pub const SPID_FUNC_ESTABLISH: c_uint = 0x00;
pub const SPID_FUNC_RESIGN: c_uint = 0x40;
pub const SPID_FUNC_DISBAND: c_uint = 0x20;
pub const SNID_STATE1_RESET: c_int = 0;
pub const SNID_STATE1_UNGROUPED: c_int = 2;
pub const SNID_STATE1_GROUPED: c_int = 3;
pub const SNID_STATE2_NOT_RESVD: c_int = 0;
pub const SNID_STATE2_RESVD_ELSE: c_int = 2;
pub const SNID_STATE2_RESVD_SELF: c_int = 3;
pub const SNID_STATE3_MULTI_PATH: c_int = 1;
pub const SNID_STATE3_SINGLE_PATH: c_int = 0;
//
// Miscellaneous constants
//
pub const CSS_NUM_CUB_PAGES: c_int = 2;
pub const CSS_CUES_PER_PAGE: c_int = 128;
pub const CSS_NUM_ECUB_PAGES: c_int = 4;
pub const CSS_ECUES_PER_PAGE: c_int = 64;
//
// Conditions used to specify which subchannels need evaluation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum css_eval_cond {
    CSS_EVAL_NO_PATH,		/* Subchannels with no operational paths */
    CSS_EVAL_NOT_ONLINE	/* sch without an online-device */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_state {
    pub /: *mut *mut __u8 state1 : 2; / path state value 1,
    pub /: *mut *mut __u8 state2 : 2; / path state value 2,
    pub /: *mut *mut __u8 state3 : 1; / path state value 3,
    pub /: *mut *mut __u8 resvd : 3; / reserved,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extended_cssid {
    pub version: u8,
    pub cssid: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgid {
    pub /: *mut *mut __u8 fc; / SPID function code,
    pub /: *mut *mut path_state ps; / SNID path state,
// C attribute field omitted
    pub /: *mut *mut __u32 cpu_addr : 16; / CPU address,
    pub ext_cssid: extended_cssid,
// C attribute field omitted
    pub /: *mut *mut __u32 cpu_id : 24; / CPU identification,
    pub /: *mut *mut __u32 cpu_model : 16; / CPU model,
    pub /: *mut *mut __u32 tod_high; / high word TOD clock,
// C attribute field omitted
    pub subchannel: struct,
    pub chp_link: struct,
//
// struct css_driver - device driver for subchannels
// @subchannel_type: subchannel type supported by this driver
// @drv: embedded device driver structure
// @irq: called on interrupts
// @chp_event: called for events affecting a channel path
// @sch_event: called for events affecting the subchannel
// @probe: function called on probe
// @remove: function called on remove
// @shutdown: called at device shutdown
// @settle: wait for asynchronous work to finish
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_driver {
    pub subchannel_type: *mut css_device_id,
    pub drv: device_driver,
    pub ): *mut *mut void (irq)(struct subchannel,
    pub int): *mut *mut *mut *mut int (chp_event)(struct subchannel , struct chp_link ,,
    pub int): *mut *mut *mut int (sch_event)(struct subchannel ,,
    pub ): *mut *mut int (probe)(struct subchannel,
    pub ): *mut *mut void (remove)(struct subchannel,
    pub ): *mut *mut void (shutdown)(struct subchannel,
    pub (*settle)(void): *mut c_int,
}

extern "C" {
    pub fn css_driver_register(: *mut css_driver) -> c_int;
}
extern "C" {
    pub fn css_driver_unregister(: *mut css_driver);
}
extern "C" {
    pub fn css_sch_device_unregister(: *mut subchannel);
}
extern "C" {
    pub fn css_register_subchannel(: *mut subchannel) -> c_int;
}
extern "C" {
    pub fn for_each_subchannel(subchannel_id: *mut *mut int(fn)(struct, ): *mut c_void, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn css_update_ssd_info(sch: *mut subchannel);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_subsystem {
    pub cssid: u8,
    pub iid: u8,
    pub /: *mut *mut bool id_valid; / cssid,iid,
    pub 1]: *mut *mut channel_path chps[__MAX_CHPID +,
    pub device: device,
    pub global_pgid: pgid,
    pub mutex: mutex,
// channel measurement related
    pub cm_enabled: c_int,
    pub cub: [*mut c_void; CSS_NUM_CUB_PAGES],
    pub ecub: [*mut c_void; CSS_NUM_ECUB_PAGES],
// for orphaned ccw devices
    pub pseudo_subchannel: *mut subchannel,
}

// Dummy helper which needs to change once we support more than one css.
// Dummy iterator which needs to change once we support more than one css.

// Helper functions to build lists for the slow path.
extern "C" {
    pub fn css_schedule_eval(schid: subchannel_id);
}
extern "C" {
    pub fn css_schedule_eval_all();
}
extern "C" {
    pub fn css_schedule_eval_cond(css_eval_cond: enum, delay: c_ulong);
}
extern "C" {
    pub fn css_complete_work() -> c_int;
}
extern "C" {
    pub fn sch_is_pseudo_sch(: *mut subchannel) -> c_int;
}
extern "C" {
    pub fn css_sch_is_valid(: *mut schib) -> c_int;
}
extern "C" {
    pub fn css_wait_for_slow_path();
}
extern "C" {
    pub fn css_sched_sch_todo(sch: *mut subchannel, todo: sch_todo);
}
