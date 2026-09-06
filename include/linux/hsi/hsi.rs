//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hsi/hsi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// HSI core header file.
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
//
// Contact: Carlos Chinea <carlos.chinea@nokia.com>
//

// HSI message ttype
pub const HSI_MSG_READ: c_int = 0;
pub const HSI_MSG_WRITE: c_int = 1;
// HSI configuration values
pub const HSI_MAX_CHANNELS: c_int = 16;
// HSI message status codes
// HSI port event codes
//
// struct hsi_channel - channel resource used by the hsi clients
// @id: Channel number
// @name: Channel name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_channel {
    pub id: c_uint,
    pub name: *const c_char,
}

//
// struct hsi_config - Configuration for RX/TX HSI modules
// @mode: Bit transmission mode (STREAM or FRAME)
// @channels: Channel resources used by the client
// @num_channels: Number of channel resources
// @num_hw_channels: Number of channels the transceiver is configured for [1..16]
// @speed: Max bit transmission speed (Kbit/s)
// @flow: RX flow type (SYNCHRONIZED or PIPELINE)
// @arb_mode: Arbitration mode for TX frame (Round robin, priority)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_config {
    pub mode: c_uint,
    pub channels: *mut hsi_channel,
    pub num_channels: c_uint,
    pub num_hw_channels: c_uint,
    pub speed: c_uint,
    pub /: *mut *mut unsigned int flow; / RX only,
    pub /: *mut *mut unsigned int arb_mode; / TX only,
}

//
// struct hsi_board_info - HSI client board info
// @name: Name for the HSI device
// @hsi_id: HSI controller id where the client sits
// @port: Port number in the controller where the client sits
// @tx_cfg: HSI TX configuration
// @rx_cfg: HSI RX configuration
// @platform_data: Platform related data
// @archdata: Architecture-dependent device data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_board_info {
    pub name: *const c_char,
    pub hsi_id: c_uint,
    pub port: c_uint,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
    pub platform_data: *mut c_void,
    pub archdata: *mut dev_archdata,
}

//
// struct hsi_client - HSI client attached to an HSI port
// @device: Driver model representation of the device
// @tx_cfg: HSI TX configuration
// @rx_cfg: HSI RX configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_client {
    pub device: device,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
// private:
    pub long): *mut *mut *mut void (ehandler)(struct hsi_client , unsigned,
    pub pclaimed:1: c_uint,
    pub nb: notifier_block,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &cl->device) -> return;
}
extern "C" {
    pub fn hsi_unregister_port_event(cl: *mut hsi_client) -> c_int;
}
//
// struct hsi_client_driver - Driver associated to an HSI client
// @driver: Driver model representation of the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_client_driver {
    pub driver: device_driver,
}

extern "C" {
    pub fn hsi_register_client_driver(drv: *mut hsi_client_driver) -> c_int;
}
//
// struct hsi_msg - HSI message descriptor
// @link: Free to use by the current descriptor owner
// @cl: HSI device client that issues the transfer
// @sgt: Head of the scatterlist array
// @context: Client context data associated to the transfer
// @complete: Transfer completion callback
// @destructor: Destructor to free resources when flushing
// @status: Status of the transfer when completed
// @actual_len: Actual length of data transferred on completion
// @channel: Channel were to TX/RX the message
// @ttype: Transfer type (TX if set, RX otherwise)
// @break_frame: if true HSI will send/receive a break frame. Data buffers are
// ignored in the request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_msg {
    pub link: list_head,
    pub cl: *mut hsi_client,
    pub sgt: sg_table,
    pub context: *mut c_void,
    pub msg): *mut *mut void (complete)(struct hsi_msg,
    pub msg): *mut *mut void (destructor)(struct hsi_msg,
    pub status: c_int,
    pub actual_len: c_uint,
    pub channel: c_uint,
    pub ttype:1: c_uint,
    pub break_frame:1: c_uint,
}

extern "C" {
    pub fn hsi_free_msg(msg: *mut hsi_msg);
}
//
// struct hsi_port - HSI port device
// @device: Driver model representation of the device
// @tx_cfg: Current TX path configuration
// @rx_cfg: Current RX path configuration
// @num: Port number
// @shared: Set when port can be shared by different clients
// @claimed: Reference count of clients which claimed the port
// @lock: Serialize port claim
// @async: Asynchronous transfer callback
// @setup: Callback to set the HSI client configuration
// @flush: Callback to clean the HW state and destroy all pending transfers
// @start_tx: Callback to inform that a client wants to TX data
// @stop_tx: Callback to inform that a client no longer wishes to TX data
// @release: Callback to inform that a client no longer uses the port
// @n_head: Notifier chain for signaling port events to the clients.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_port {
    pub device: device,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
    pub num: c_uint,
    pub shared:1: c_uint,
    pub claimed: c_int,
    pub lock: mutex,
    pub msg): *mut *mut int (async)(struct hsi_msg,
    pub cl): *mut *mut int (setup)(struct hsi_client,
    pub cl): *mut *mut int (flush)(struct hsi_client,
    pub cl): *mut *mut int (start_tx)(struct hsi_client,
    pub cl): *mut *mut int (stop_tx)(struct hsi_client,
    pub cl): *mut *mut int (release)(struct hsi_client,
// private
    pub n_head: blocking_notifier_head,
}

extern "C" {
    pub fn hsi_event(port: *mut hsi_port, event: c_ulong) -> c_int;
}
extern "C" {
    pub fn hsi_claim_port(cl: *mut hsi_client, share: c_uint) -> c_int;
}
extern "C" {
    pub fn hsi_release_port(cl: *mut hsi_client);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &port->device) -> return;
}
//
// struct hsi_controller - HSI controller device
// @device: Driver model representation of the device
// @owner: Pointer to the module owning the controller
// @id: HSI controller ID
// @num_ports: Number of ports in the HSI controller
// @port: Array of HSI ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_controller {
    pub device: device,
    pub owner: *mut module,
    pub id: c_uint,
    pub num_ports: c_uint,
    pub __counted_by(num_ports): *mut *mut hsi_port port[],
}

extern "C" {
    pub fn hsi_put_controller(hsi: *mut hsi_controller);
}
extern "C" {
    pub fn hsi_register_controller(hsi: *mut hsi_controller) -> c_int;
}
extern "C" {
    pub fn hsi_unregister_controller(hsi: *mut hsi_controller);
}
extern "C" {
    pub fn hsi_remove_client(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hsi_port_unregister_clients(port: *mut hsi_port);
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &hsi->device) -> return;
}
//
// API for HSI clients
//
extern "C" {
    pub fn hsi_async(cl: *mut hsi_client, msg: *mut hsi_msg) -> c_int;
}
extern "C" {
    pub fn hsi_get_channel_id_by_name(cl: *mut hsi_client, name: *mut c_char) -> c_int;
}
//
// hsi_id - Get HSI controller ID associated to a client
// @cl: Pointer to a HSI client
//
// Return the controller id where the client is attached to
//
// hsi_port_id - Gets the port number a client is attached to
// @cl: Pointer to HSI client
//
// Return the port number associated to the client
//
// hsi_setup - Configure the client's port
// @cl: Pointer to the HSI client
//
// When sharing ports, clients should either relay on a single
// client setup or have the same setup for all of them.
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_get_port(_arg: cl)->setup(cl) -> return;
}
//
// hsi_flush - Flush all pending transactions on the client's port
// @cl: Pointer to the HSI client
//
// This function will destroy all pending hsi_msg in the port and reset
// the HW port so it is ready to receive and transmit from a clean state.
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_get_port(_arg: cl)->flush(cl) -> return;
}
//
// hsi_async_read - Submit a read transfer
// @cl: Pointer to the HSI client
// @msg: HSI message descriptor of the transfer
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_async(_arg: cl, _arg: msg) -> return;
}
//
// hsi_async_write - Submit a write transfer
// @cl: Pointer to the HSI client
// @msg: HSI message descriptor of the transfer
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_async(_arg: cl, _arg: msg) -> return;
}
//
// hsi_start_tx - Signal the port that the client wants to start a TX
// @cl: Pointer to the HSI client
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_get_port(_arg: cl)->start_tx(cl) -> return;
}
//
// hsi_stop_tx - Signal the port that the client no longer wants to transmit
// @cl: Pointer to the HSI client
//
// Return -errno on failure, 0 on success
//
extern "C" {
    pub fn hsi_get_port(_arg: cl)->stop_tx(cl) -> return;
}
