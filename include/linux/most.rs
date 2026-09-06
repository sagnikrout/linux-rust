//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/most.h
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
// most.h - API for component and adapter drivers
//
// Copyright (C) 2013-2015, Microchip Technology Germany II GmbH & Co. KG
//

//
// enum most_interface_type - Interface type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum most_interface_type {
    ITYPE_LOOPBACK = 1,
    ITYPE_I2C,
    ITYPE_I2S,
    ITYPE_TSI,
    ITYPE_HBI,
    ITYPE_MEDIALB_DIM,
    ITYPE_MEDIALB_DIM2,
    ITYPE_USB,
    ITYPE_PCIE
}

//
// enum most_channel_direction - Channel direction.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum most_channel_direction {
    MOST_CH_RX = 1 << 0,
    MOST_CH_TX = 1 << 1,
}

//
// enum most_channel_data_type - Channel data type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum most_channel_data_type {
    MOST_CH_CONTROL = 1 << 0,
    MOST_CH_ASYNC = 1 << 1,
    MOST_CH_ISOC = 1 << 2,
    MOST_CH_SYNC = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum most_status_flags {
// MBO was processed successfully (data was send or received )
    MBO_SUCCESS = 0,
// The MBO contains wrong or missing information.
    MBO_E_INVAL,
// MBO was completed as HDM Channel will be closed
    MBO_E_CLOSE,
}

//
// struct most_channel_capability - Channel capability
// @direction: Supported channel directions.
// The value is bitwise OR-combination of the values from the
// enumeration most_channel_direction. Zero is allowed value and means
// "channel may not be used".
// @data_type: Supported channel data types.
// The value is bitwise OR-combination of the values from the
// enumeration most_channel_data_type. Zero is allowed value and means
// "channel may not be used".
// @num_buffers_packet: Maximum number of buffers supported by this channel
// for packet data types (Async,Control,QoS)
// @buffer_size_packet: Maximum buffer size supported by this channel
// for packet data types (Async,Control,QoS)
// @num_buffers_streaming: Maximum number of buffers supported by this channel
// for streaming data types (Sync,AV Packetized)
// @buffer_size_streaming: Maximum buffer size supported by this channel
// for streaming data types (Sync,AV Packetized)
// @name_suffix: Optional suffix providean by an HDM that is attached to the
// regular channel name.
//
// Describes the capabilities of a MOST channel like supported Data Types
// and directions. This information is provided by an HDM for the MostCore.
//
// The Core creates read only sysfs attribute files in
// /sys/devices/most/mdev#/<channel>/ with the
// following attributes:
// -available_directions
// -available_datatypes
// -number_of_packet_buffers
// -number_of_stream_buffers
// -size_of_packet_buffer
// -size_of_stream_buffer
// where content of each file is a string with all supported properties of this
// very channel attribute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_channel_capability {
    pub direction: u16,
    pub data_type: u16,
    pub num_buffers_packet: u16,
    pub buffer_size_packet: u16,
    pub num_buffers_streaming: u16,
    pub buffer_size_streaming: u16,
    pub name_suffix: *const c_char,
}

//
// struct most_channel_config - stores channel configuration
// @direction: direction of the channel
// @data_type: data type travelling over this channel
// @num_buffers: number of buffers
// @buffer_size: size of a buffer for AIM.
// Buffer size may be cutted down by HDM in a configure callback
// to match to a given interface and channel type.
// @extra_len: additional buffer space for internal HDM purposes like padding.
// May be set by HDM in a configure callback if needed.
// @subbuffer_size: size of a subbuffer
// @packets_per_xact: number of MOST frames that are packet inside one USB
// packet. This is USB specific
// @dbr_size: DBR data buffer size (MediaLB communication only)
//
// Describes the configuration for a MOST channel. This information is
// provided from the MostCore to a HDM (like the Medusa PCIe Interface) as a
// parameter of the "configure" function call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_channel_config {
    pub direction: most_channel_direction,
    pub data_type: most_channel_data_type,
    pub num_buffers: u16,
    pub buffer_size: u16,
    pub extra_len: u16,
    pub subbuffer_size: u16,
    pub packets_per_xact: u16,
    pub dbr_size: u16,
}

//
// struct mbo - MOST Buffer Object.
// @context: context for core completion handler
// @priv: private data for HDM
//
// public: documented fields that are used for the communications
// between MostCore and HDMs
//
// @list: list head for use by the mbo's current owner
// @ifp: (in) associated interface instance
// @num_buffers_ptr: amount of pool buffers
// @hdm_channel_id: (in) HDM channel instance
// @virt_address: (in) kernel virtual address of the buffer
// @bus_address: (in) bus address of the buffer
// @buffer_length: (in) buffer payload length
// @processed_length: (out) processed length
// @status: (out) transfer status
// @complete: (in) completion routine
//
// The core allocates and initializes the MBO.
//
// The HDM receives MBO for transfer from the core with the call to enqueue().
// The HDM copies the data to- or from the buffer depending on configured
// channel direction, set "processed_length" and "status" and completes
// the transfer procedure by calling the completion routine.
//
// Finally, the MBO is being deallocated or recycled for further
// transfers of the same or a different HDM.
//
// Directions of usage:
// The core driver should never access any MBO fields (even if marked
// as "public") while the MBO is owned by an HDM. The ownership starts with
// the call of enqueue() and ends with the call of its complete() routine.
//
// II.
// Every HDM attached to the core driver _must_ ensure that it returns any MBO
// it owns (due to a previous call to enqueue() by the core driver) before it
// de-registers an interface or gets unloaded from the kernel. If this direction
// is violated memory leaks will occur, since the core driver does _not_ track
// MBOs it is currently not in control of.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbo {
    pub context: *mut c_void,
    pub priv: *mut c_void,
    pub list: list_head,
    pub ifp: *mut most_interface,
    pub num_buffers_ptr: *mut c_int,
    pub hdm_channel_id: u16,
    pub virt_address: *mut c_void,
    pub bus_address: dma_addr_t,
    pub buffer_length: u16,
    pub processed_length: u16,
    pub status: most_status_flags,
    pub mbo): *mut *mut void (complete)(struct mbo,
}

//
// struct most_interface - Interface instance description.
//
// Describes an interface of a MOST device the core driver is bound to.
// This structure is allocated and initialized in the HDM. MostCore may not
// modify this structure.
//
// @dev: the actual device
// @driver_dev: struct device for calling snd_card_new()
// @mod: module
// @interface: Interface type. \sa most_interface_type.
// @description: PRELIMINARY.
// Unique description of the device instance from point of view of the
// interface in free text form (ASCII).
// It may be a hexadecimal presentation of the memory address for the MediaLB
// IP or USB device ID with USB properties for USB interface, etc.
// @num_channels: Number of channels and size of the channel_vector.
// @channel_vector: Properties of the channels.
// Array index represents channel ID by the driver.
// @dma_alloc: Callback to interface driver to allocation DMA memory.
// @dma_free: Callback to interface driver to free a DMA memory allocation.
// @configure: Callback to change data type for the channel of the
// interface instance. May be zero if the instance of the interface is not
// configurable. Parameter channel_config describes direction and data
// type for the channel, configured by the higher level. The content of
// @enqueue: Delivers MBO to the HDM for processing.
// After HDM completes Rx- or Tx- operation the processed MBO shall
// be returned back to the MostCore using completion routine.
// The reason to get the MBO delivered from the MostCore after the channel
// is poisoned is the re-opening of the channel by the application.
// In this case the HDM shall hold MBOs and service the channel as usual.
// The HDM must be able to hold at least one MBO for each channel.
// The callback returns a negative value on error, otherwise 0.
// @poison_channel: Informs HDM about closing the channel. The HDM shall
// cancel all transfers and synchronously or asynchronously return
// all enqueued for this channel MBOs using the completion routine.
// The callback returns a negative value on error, otherwise 0.
// @request_netinfo: triggers retrieving of network info from the HDM by
// means of "Message exchange over MDP/MEP"
// The call of the function request_netinfo with the parameter on_netinfo as
// NULL prohibits use of the previously obtained function pointer.
// @priv: Private field used by mostcore to store context information.
// @p: &struct interface_private pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_interface {
    pub dev: *mut device,
    pub driver_dev: *mut device,
    pub mod: *mut module,
    pub interface: most_interface_type,
    pub description: *const c_char,
    pub num_channels: c_uint,
    pub channel_vector: *mut most_channel_capability,
    pub size): *mut *mut *mut *mut void (dma_alloc)(struct mbo mbo, u32,
    pub size): *mut *mut *mut void (dma_free)(struct mbo mbo, u32,
    pub channel_config): *mut most_channel_config,
    pub mbo): *mut mbo,
    pub channel_idx): *mut *mut *mut int (poison_channel)(struct most_interface iface, int,
    pub mac_addr)): *mut c_uchar,
    pub priv: *mut c_void,
    pub p: *mut interface_private,
}

//
// struct most_component - identifies a loadable component for the mostcore
// @list: list_head
// @name: component name
// @mod: owning module
// @probe_channel: function for core to notify driver about channel connection
// @disconnect_channel: callback function to disconnect a certain channel
// @rx_completion: completion handler for received packets
// @tx_completion: completion handler for transmitted packets
// @cfg_complete: setup completion function called after the device is matched
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct most_component {
    pub list: list_head,
    pub name: *const c_char,
    pub mod: *mut module,
    pub param): *mut c_char,
    pub channel_idx): c_int,
    pub mbo): *mut *mut int (rx_completion)(struct mbo,
    pub channel_idx): *mut *mut *mut int (tx_completion)(struct most_interface iface, int,
    pub (*cfg_complete)(void): *mut c_int,
}

//
// most_register_interface - Registers instance of the interface.
// @iface: Pointer to the interface instance description.
//
// Returns: a pointer to the kobject of the generated instance.
//
// Note: HDM has to ensure that any reference held on the kobj is
// released before deregistering the interface.
//
extern "C" {
    pub fn most_register_interface(iface: *mut most_interface) -> c_int;
}
//
// most_deregister_interface - Deregisters instance of the interface.
// @iface: Pointer to the interface instance description.
//
extern "C" {
    pub fn most_deregister_interface(iface: *mut most_interface);
}
extern "C" {
    pub fn most_submit_mbo(mbo: *mut mbo);
}
//
// most_stop_enqueue - prevents core from enqueing MBOs
// @iface: pointer to interface
// @channel_idx: channel index
//
extern "C" {
    pub fn most_stop_enqueue(iface: *mut most_interface, channel_idx: c_int);
}
//
// most_resume_enqueue - allow core to enqueue MBOs again
// @iface: pointer to interface
// @channel_idx: channel index
//
// This clears the enqueue halt flag and enqueues all MBOs currently
// in wait fifo.
//
extern "C" {
    pub fn most_resume_enqueue(iface: *mut most_interface, channel_idx: c_int);
}
extern "C" {
    pub fn most_register_component(comp: *mut most_component) -> c_int;
}
extern "C" {
    pub fn most_deregister_component(comp: *mut most_component) -> c_int;
}
extern "C" {
    pub fn most_put_mbo(mbo: *mut mbo);
}
extern "C" {
    pub fn configfs_init() -> int __init;
}
extern "C" {
    pub fn most_register_configfs_subsys(comp: *mut most_component) -> c_int;
}
extern "C" {
    pub fn most_deregister_configfs_subsys(comp: *mut most_component);
}
extern "C" {
    pub fn most_remove_link(mdev: *mut c_char, mdev_ch: *mut c_char, comp_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_buffer_size(mdev: *mut c_char, mdev_ch: *mut c_char, val: u16) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_subbuffer_size(mdev: *mut c_char, mdev_ch: *mut c_char, val: u16) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_dbr_size(mdev: *mut c_char, mdev_ch: *mut c_char, val: u16) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_num_buffers(mdev: *mut c_char, mdev_ch: *mut c_char, val: u16) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_datatype(mdev: *mut c_char, mdev_ch: *mut c_char, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_direction(mdev: *mut c_char, mdev_ch: *mut c_char, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn most_set_cfg_packets_xact(mdev: *mut c_char, mdev_ch: *mut c_char, val: u16) -> c_int;
}
extern "C" {
    pub fn most_cfg_complete(comp_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn most_interface_register_notify(mdev_name: *const c_char);
}
