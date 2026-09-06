//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/dln2.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dln2_platform_data {
    pub /: *mut *mut u16 handle; / sub-driver handle (internally used only),
    pub /: *mut *mut u8 port; / I2C/SPI port,
}

//
// dln2_event_cb_t - event callback function signature
//
// @pdev - the sub-device that registered this callback
// @echo - the echo header field received in the message
// @data - the data payload
// @len  - the data payload length
//
// The callback function is called in interrupt context and the data payload is
// only valid during the call. If the user needs later access of the data, it
// must copy it.
//
// dl2n_register_event_cb - register a callback function for an event
//
// @pdev - the sub-device that registers the callback
// @event - the event for which to register a callback
// @event_cb - the callback function
//
// @return 0 in case of success, negative value in case of error
//
// dln2_unregister_event_cb - unregister the callback function for an event
//
// @pdev - the sub-device that registered the callback
// @event - the event for which to register a callback
//
extern "C" {
    pub fn dln2_unregister_event_cb(pdev: *mut platform_device, event: u16);
}
//
// dln2_transfer - issue a DLN2 command and wait for a response and the
// associated data
//
// @pdev - the sub-device which is issuing this transfer
// @cmd - the command to be sent to the device
// @obuf - the buffer to be sent to the device; it can be NULL if the user
// doesn't need to transmit data with this command
// @obuf_len - the size of the buffer to be sent to the device
// @ibuf - any data associated with the response will be copied here; it can be
// NULL if the user doesn't need the response data
// @ibuf_len - must be initialized to the input buffer size; it will be modified
// to indicate the actual data transferred;
//
// @return 0 for success, negative value for errors
//
// dln2_transfer_rx - variant of @dln2_transfer() where TX buffer is not needed
//
// @pdev - the sub-device which is issuing this transfer
// @cmd - the command to be sent to the device
// @ibuf - any data associated with the response will be copied here; it can be
// NULL if the user doesn't need the response data
// @ibuf_len - must be initialized to the input buffer size; it will be modified
// to indicate the actual data transferred;
//
// @return 0 for success, negative value for errors
//
extern "C" {
    pub fn dln2_transfer(_arg: pdev, _arg: cmd, _arg: NULL, _arg: 0, _arg: ibuf, _arg: ibuf_len) -> return;
}
//
// dln2_transfer_tx - variant of @dln2_transfer() where RX buffer is not needed
//
// @pdev - the sub-device which is issuing this transfer
// @cmd - the command to be sent to the device
// @obuf - the buffer to be sent to the device; it can be NULL if the
// user doesn't need to transmit data with this command
// @obuf_len - the size of the buffer to be sent to the device
//
// @return 0 for success, negative value for errors
//
extern "C" {
    pub fn dln2_transfer(_arg: pdev, _arg: cmd, _arg: obuf, _arg: obuf_len, _arg: NULL, _arg: NULL) -> return;
}
