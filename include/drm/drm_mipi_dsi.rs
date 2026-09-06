//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_mipi_dsi.h
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
// MIPI DSI Bus
//
// Copyright (C) 2012-2013, Samsung Electronics, Co., Ltd.
// Andrzej Hajda <a.hajda@samsung.com>
//

// request ACK from peripheral

// use Low Power Mode to transmit message

//
// struct mipi_dsi_msg - read/write DSI buffer
// @channel: virtual channel id
// @type: payload data type
// @flags: flags controlling this message transmission
// @tx_len: length of @tx_buf
// @tx_buf: data to be written
// @rx_len: length of @rx_buf
// @rx_buf: data to be read, or NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_msg {
    pub channel: u8,
    pub type: u8,
    pub flags: u16,
    pub tx_len: usize,
    pub tx_buf: *const c_void,
    pub rx_len: usize,
    pub rx_buf: *mut c_void,
}

extern "C" {
    pub fn mipi_dsi_packet_format_is_short(type: u8) -> bool;
}
extern "C" {
    pub fn mipi_dsi_packet_format_is_long(type: u8) -> bool;
}
//
// struct mipi_dsi_packet - represents a MIPI DSI packet in protocol format
// @size: size (in bytes) of the packet
// @header: the four bytes that make up the header (Data ID, Word Count or
// Packet Data, and ECC)
// @payload_length: number of bytes in the payload
// @payload: a pointer to a buffer containing the payload, if any
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_packet {
    pub size: usize,
    pub header: [u8; 4],
    pub payload_length: usize,
    pub payload: *const u8,
}

//
// struct mipi_dsi_host_ops - DSI bus operations
// @attach: attach DSI device to DSI host
// @detach: detach DSI device from DSI host
// @transfer: transmit a DSI packet
//
// DSI packets transmitted by .transfer() are passed in as mipi_dsi_msg
// structures. This structure contains information about the type of packet
// being transmitted as well as the transmit and receive buffers. When an
// error is encountered during transmission, this function will return a
// negative error code. On success it shall return the number of bytes
// transmitted for write packets or the number of bytes received for read
// packets.
//
// Note that typically DSI packet transmission is atomic, so the .transfer()
// function will seldomly return anything other than the number of bytes
// contained in the transmit buffer on success.
//
// Also note that those callbacks can be called no matter the state the
// host is in. Drivers that need the underlying device to be powered to
// perform these operations will first need to make sure it's been
// properly enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_host_ops {
    pub dsi): *mut mipi_dsi_device,
    pub dsi): *mut mipi_dsi_device,
    pub msg): *const mipi_dsi_msg,
}

//
// struct mipi_dsi_host - DSI host device
// @dev: driver model device node for this DSI host
// @ops: DSI host operations
// @list: list management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_host {
    pub dev: *mut device,
    pub ops: *const mipi_dsi_host_ops,
    pub list: list_head,
}

extern "C" {
    pub fn mipi_dsi_host_register(host: *mut mipi_dsi_host) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_host_unregister(host: *mut mipi_dsi_host);
}
// DSI mode flags
// video mode

// video burst mode

// video pulse mode

// enable auto vertical count mode

// enable hsync-end packets in vsync-pulse and v-porch area

// disable hfront-porch area

// disable hback-porch area

// disable hsync-active area

// disable EoT packets in HS mode

// device supports non-continuous clock behavior (DSI spec 5.6.1)

// transmit data in low power

// transmit data ending at the same time for all lanes within one hsync

// pack all DSC slices for a line into a single packet

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_pixel_format {
    MIPI_DSI_FMT_RGB888,
    MIPI_DSI_FMT_RGB666,
    MIPI_DSI_FMT_RGB666_PACKED,
    MIPI_DSI_FMT_RGB565,
    MIPI_DSI_FMT_RGB101010,
}

pub const DSI_DEV_NAME_SIZE: c_int = 20;
//
// struct mipi_dsi_device_info - template for creating a mipi_dsi_device
// @type: DSI peripheral chip type
// @channel: DSI virtual channel assigned to peripheral
// @node: pointer to OF device node or NULL
//
// This is populated and passed to mipi_dsi_device_new to create a new
// DSI device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_device_info {
    pub type: [c_char; DSI_DEV_NAME_SIZE],
    pub channel: u32,
    pub node: *mut device_node,
}

//
// struct mipi_dsi_device - DSI peripheral device
// @host: DSI host for this peripheral
// @dev: driver model device node for this peripheral
// @attached: the DSI device has been successfully attached
// @name: DSI peripheral chip type
// @channel: virtual channel assigned to the peripheral
// @format: pixel format for video mode
// @lanes: number of active data lanes
// @mode_flags: DSI operation mode related flags
// @hs_rate: maximum lane frequency for high speed mode in hertz, this should
// be set to the real limits of the hardware, zero is only accepted for
// legacy drivers
// @lp_rate: maximum lane frequency for low power mode in hertz, this should
// be set to the real limits of the hardware, zero is only accepted for
// legacy drivers
// @dsc: panel/bridge DSC pps payload to be sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_device {
    pub host: *mut mipi_dsi_host,
    pub dev: device,
    pub attached: bool,
    pub name: [c_char; DSI_DEV_NAME_SIZE],
    pub channel: c_uint,
    pub lanes: c_uint,
    pub format: mipi_dsi_pixel_format,
    pub mode_flags: c_ulong,
    pub hs_rate: c_ulong,
    pub lp_rate: c_ulong,
    pub dsc: *mut drm_dsc_config,
}

//
// struct mipi_dsi_multi_context - Context to call multiple MIPI DSI funcs in a row
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_multi_context {
//
// @dsi: Pointer to the MIPI DSI device
//
    pub dsi: *mut mipi_dsi_device,
//
// @accum_err: Storage for the accumulated error over the multiple calls
//
// Init to 0. If a function encounters an error then the error code
// will be stored here. If you call a function and this points to a
// non-zero value then the function will be a noop. This allows calling
// a function many times in a row and just checking the error at the
// end to see if any of them failed.
//
    pub accum_err: c_int,
}

//
// mipi_dsi_pixel_format_to_bpp - obtain the number of bits per pixel for any
// given pixel format defined by the MIPI DSI
// specification
// @fmt: MIPI DSI pixel format
//
// Returns: The number of bits per pixel of the given pixel format.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_compression_algo {
    MIPI_DSI_COMPRESSION_DSC = 0,
    MIPI_DSI_COMPRESSION_VENDOR = 3,
// other two values are reserved, DSI 1.3
}

extern "C" {
    pub fn mipi_dsi_device_unregister(dsi: *mut mipi_dsi_device);
}
extern "C" {
    pub fn mipi_dsi_attach(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_detach(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn devm_mipi_dsi_attach(dev: *mut device, dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_shutdown_peripheral(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_turn_on_peripheral(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_compression_mode(dsi: *mut mipi_dsi_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn drm_mipi_dsi_get_input_bus_fmt(dsi_format: mipi_dsi_pixel_format) -> u32;
}

//
// enum mipi_dsi_dcs_tear_mode - Tearing Effect Output Line mode
// @MIPI_DSI_DCS_TEAR_MODE_VBLANK: the TE output line consists of V-Blanking
// information only
// @MIPI_DSI_DCS_TEAR_MODE_VHBLANK : the TE output line consists of both
// V-Blanking and H-Blanking information
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_dcs_tear_mode {
    MIPI_DSI_DCS_TEAR_MODE_VBLANK,
    MIPI_DSI_DCS_TEAR_MODE_VHBLANK,
}

extern "C" {
    pub fn mipi_dsi_dcs_nop(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_soft_reset(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_get_power_mode(dsi: *mut mipi_dsi_device, mode: *mut u8) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_get_pixel_format(dsi: *mut mipi_dsi_device, format: *mut u8) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_enter_sleep_mode(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_exit_sleep_mode(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_set_display_off(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_set_display_on(dsi: *mut mipi_dsi_device) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_set_pixel_format(dsi: *mut mipi_dsi_device, format: u8) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_set_tear_scanline(dsi: *mut mipi_dsi_device, scanline: u16) -> c_int;
}
extern "C" {
    pub fn mipi_dsi_dcs_nop_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_enter_sleep_mode_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_exit_sleep_mode_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_set_display_off_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_set_display_on_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_turn_on_peripheral_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_soft_reset_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_dcs_set_tear_off_multi(ctx: *mut mipi_dsi_multi_context);
}
extern "C" {
    pub fn mipi_dsi_shutdown_peripheral_multi(ctx: *mut mipi_dsi_multi_context);
}
//
// mipi_dsi_generic_write_seq_multi - transmit data using a generic write packet
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @ctx: Context for multiple DSI transactions
// @seq: buffer containing the payload
//

//
// mipi_dsi_generic_write_var_seq_multi - transmit non-constant data using a
// generic write packet
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @ctx: Context for multiple DSI transactions
// @seq: buffer containing the payload
//

//
// mipi_dsi_dcs_write_seq_multi - transmit a DCS command with payload
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @ctx: Context for multiple DSI transactions
// @cmd: Command
// @seq: buffer containing data to be transmitted
//

//
// mipi_dsi_dcs_write_var_seq_multi - transmit a DCS command with non-constant
// payload
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @ctx: Context for multiple DSI transactions
// @cmd: Command
// @seq: buffer containing data to be transmitted
//

//
// mipi_dsi_dual - send the same MIPI DSI command to two interfaces
//
// This macro will send the specified MIPI DSI command twice, once per each of
// the two interfaces supplied. This is useful for reducing duplication of code
// in panel drivers which use two parallel serial interfaces.
//
// Note that the _func parameter cannot accept a macro such as
// mipi_dsi_generic_write_multi() or mipi_dsi_dcs_write_buffer_multi(). See
// mipi_dsi_dual_generic_write_multi() and
// mipi_dsi_dual_dcs_write_buffer_multi() instead.
//
// WARNING: This macro reuses the _func argument and the optional trailing
// arguments twice each, which may cause unintended side effects. For example,
// adding the postfix increment ++ operator to one of the arguments to be
// passed to _func will cause the variable to be incremented twice instead of
// once and the variable will be its original value + 1 when sent to _dsi2.
//
// @_func: MIPI DSI function to pass context and arguments into
// @_ctx: Context for multiple DSI transactions
// @_dsi1: First DSI interface to act as recipient of the MIPI DSI command
// @_dsi2: Second DSI interface to act as recipient of the MIPI DSI command
// @...: Arguments to pass to MIPI DSI function or macro
//

//
// mipi_dsi_dual_generic_write_seq_multi - transmit data using a generic write
// packet to two dsi interfaces, one after the other
//
// This macro will send the specified generic packet twice, once per each of
// the two interfaces supplied. This is useful for reducing duplication of code
// in panel drivers which use two parallel serial interfaces.
//
// Note that if an error occurs while transmitting the packet to the first DSI
// interface, the packet will not be sent to the second DSI interface.
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @_ctx: Context for multiple DSI transactions
// @_dsi1: First DSI interface to act as recipient of packet
// @_dsi2: Second DSI interface to act as recipient of packet
// @_seq: buffer containing the payload
//

//
// mipi_dsi_dual_dcs_write_seq_multi - transmit a DCS command with payload to
// two dsi interfaces, one after the other
//
// This macro will send the specified DCS command with payload twice, once per
// each of the two interfaces supplied. This is useful for reducing duplication
// of code in panel drivers which use two parallel serial interfaces.
//
// Note that if an error occurs while transmitting the payload to the first DSI
// interface, the payload will not be sent to the second DSI interface.
//
// This macro will print errors for you and error handling is optimized for
// callers that call this multiple times in a row.
//
// @_ctx: Context for multiple DSI transactions
// @_dsi1: First DSI interface to act as recipient of packet
// @_dsi2: Second DSI interface to act as recipient of packet
// @_cmd: Command
// @_seq: buffer containing the payload
//

//
// struct mipi_dsi_driver - DSI driver
// @driver: device driver model driver
// @probe: callback for device binding
// @remove: callback for device unbinding
// @shutdown: called at shutdown time to quiesce the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dsi_driver {
    pub driver: device_driver,
    pub dsi): *mut *mut int(probe)(struct mipi_dsi_device,
    pub dsi): *mut *mut void (remove)(struct mipi_dsi_device,
    pub dsi): *mut *mut void (shutdown)(struct mipi_dsi_device,
}

extern "C" {
    pub fn container_of(_arg: driver, mipi_dsi_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &dsi->dev) -> return;
}
extern "C" {
    pub fn mipi_dsi_driver_unregister(driver: *mut mipi_dsi_driver);
}

