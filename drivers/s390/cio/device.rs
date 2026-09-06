//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/device.h
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
// states of the device statemachine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_state {
    DEV_STATE_NOT_OPER,
    DEV_STATE_SENSE_ID,
    DEV_STATE_OFFLINE,
    DEV_STATE_VERIFY,
    DEV_STATE_ONLINE,
    DEV_STATE_W4SENSE,
    DEV_STATE_DISBAND_PGID,
    DEV_STATE_BOXED,
// states to wait for i/o completion before doing something
    DEV_STATE_TIMEOUT_KILL,
    DEV_STATE_QUIESCE,
// special states for devices gone not operational
    DEV_STATE_DISCONNECTED,
    DEV_STATE_DISCONNECTED_SENSE_ID,
    DEV_STATE_CMFCHANGE,
    DEV_STATE_CMFUPDATE,
    DEV_STATE_STEAL_LOCK,
// last element!
    NR_DEV_STATES
}

//
// asynchronous events of the device statemachine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_event {
    DEV_EVENT_NOTOPER,
    DEV_EVENT_INTERRUPT,
    DEV_EVENT_TIMEOUT,
    DEV_EVENT_VERIFY,
// last element!
    NR_DEV_EVENTS
}

//
// action called through jumptable
//
extern "C" {
    pub fn void(: *mut fsm_func_t)(struct ccw_device, dev_event: enum) -> typedef;
}
//
// Delivers 1 if the device state is final.
//
extern "C" {
    pub fn io_subchannel_init() -> int __init;
}
extern "C" {
    pub fn io_subchannel_recog_done(cdev: *mut ccw_device);
}
extern "C" {
    pub fn io_subchannel_init_config(sch: *mut subchannel);
}
extern "C" {
    pub fn ccw_device_cancel_halt_clear(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_is_orphan(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_recognition(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_online(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_offline(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_update_sense_data(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_test_sense_data(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_purge_blacklisted() -> c_int;
}
extern "C" {
    pub fn ccw_device_sched_todo(cdev: *mut ccw_device, todo: cdev_todo);
}
// Function prototypes for device status and basic sense stuff.
extern "C" {
    pub fn ccw_device_accumulate_irb(: *mut ccw_device, : *mut irb);
}
extern "C" {
    pub fn ccw_device_accumulate_basic_sense(: *mut ccw_device, : *mut irb);
}
extern "C" {
    pub fn ccw_device_accumulate_and_sense(: *mut ccw_device, : *mut irb) -> c_int;
}
extern "C" {
    pub fn ccw_device_do_sense(: *mut ccw_device, : *mut irb) -> c_int;
}
// Function prototype for internal request handling.
extern "C" {
    pub fn lpm_adjust(lpm: c_int, mask: c_int) -> c_int;
}
extern "C" {
    pub fn ccw_request_start(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_request_cancel(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_request_handler(cdev: *mut ccw_device);
}
extern "C" {
    pub fn ccw_request_timeout(cdev: *mut ccw_device);
}
extern "C" {
    pub fn ccw_request_notoper(cdev: *mut ccw_device);
}
// Function prototypes for sense id stuff.
extern "C" {
    pub fn ccw_device_sense_id_start(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_sense_id_done(: *mut ccw_device, _arg: c_int);
}
// Function prototypes for path grouping stuff.
extern "C" {
    pub fn ccw_device_verify_start(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_verify_done(: *mut ccw_device, _arg: c_int);
}
extern "C" {
    pub fn ccw_device_disband_start(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_disband_done(: *mut ccw_device, _arg: c_int);
}
extern "C" {
    pub fn ccw_device_stlck(: *mut ccw_device) -> c_int;
}
// Helper function for machine check handling.
extern "C" {
    pub fn ccw_device_trigger_reprobe(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_kill_io(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_notify(: *mut ccw_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ccw_device_set_disconnected(cdev: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_set_notoper(cdev: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn ccw_device_set_timeout(: *mut ccw_device, _arg: c_int);
}
extern "C" {
    pub fn ccw_device_schedule_recovery();
}
// Channel measurement facility related
extern "C" {
    pub fn retry_set_schib(cdev: *mut ccw_device);
}
extern "C" {
    pub fn cmf_retry_copy_block(: *mut ccw_device);
}
extern "C" {
    pub fn cmf_reenable(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn cmf_reactivate();
}
