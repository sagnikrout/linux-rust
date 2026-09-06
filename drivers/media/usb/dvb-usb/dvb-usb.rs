//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/dvb-usb.h
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
// dvb-usb.h is part of the DVB USB library.
//
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// the headerfile, all dvb-usb-drivers have to include.
//
// TODO: clean-up the structures for unused fields and update the comments
//

// debug

// Macro flag: #define DVB_USB_DEBUG_STATUS

// generic log methods - taken from usb.h

//
// struct dvb_usb_device_description - name and its according USB IDs
// @name: real name of the box, regardless which DVB USB device class is in use
// @cold_ids: array of struct usb_device_id which describe the device in
// pre-firmware state
// @warm_ids: array of struct usb_device_id which describe the device in
// post-firmware state
//
// Each DVB USB device class can have one or more actual devices, this struct
// assigns a name to it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_device_description {
    pub name: *const c_char,
pub const DVB_USB_ID_MAX_NUM: c_int = 15;
    pub cold_ids: [*const usb_device_id; DVB_USB_ID_MAX_NUM],
    pub warm_ids: [*const usb_device_id; DVB_USB_ID_MAX_NUM],
}

//
// Properties of USB streaming - TODO this structure should be somewhere else
// describes the kind of USB transfer used for data-streaming.
// (BULK or ISOC)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_data_stream_properties {
pub const USB_BULK: c_int = 1;
pub const USB_ISOC: c_int = 2;
    pub type: c_int,
    pub count: c_int,
    pub endpoint: c_int,
    pub /: *mut *mut int buffersize; / per URB,
    pub bulk: },
    pub framesperurb: c_int,
    pub framesize: c_int,
    pub interval: c_int,
    pub isoc: },
    pub u: },
}

//
// struct dvb_usb_adapter_fe_properties - properties of a dvb-usb-adapter.
// A DVB-USB-Adapter is basically a dvb_adapter which is present on a USB-device.
// @caps: capabilities of the DVB USB device.
// @pid_filter_count: number of PID filter position in the optional hardware
// PID-filter.
// @streaming_ctrl: called to start and stop the MPEG2-TS streaming of the
// device (not URB submitting/killing).
// This callback will be called without data URBs being active - data URBs
// will be submitted only after streaming_ctrl(1) returns successfully and
// they will be killed before streaming_ctrl(0) gets called.
// @pid_filter_ctrl: called to en/disable the PID filter, if any.
// @pid_filter: called to set/unset a PID for filtering.
// @frontend_attach: called to attach the possible frontends (fill fe-field
// of struct dvb_usb_device).
// @tuner_attach: called to attach the correct tuner and to fill pll_addr,
// pll_desc and pll_init_buf of struct dvb_usb_device).
// @stream: configuration of the USB streaming
// @size_of_priv: size of the priv memory in struct dvb_usb_adapter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_adapter_fe_properties {
pub const DVB_USB_ADAP_HAS_PID_FILTER: c_uint = 0x01;
pub const DVB_USB_ADAP_PID_FILTER_CAN_BE_TURNED_OFF: c_uint = 0x02;
pub const DVB_USB_ADAP_NEED_PID_FILTERING: c_uint = 0x04;
pub const DVB_USB_ADAP_RECEIVES_204_BYTE_TS: c_uint = 0x08;
pub const DVB_USB_ADAP_RECEIVES_RAW_PAYLOAD: c_uint = 0x10;
    pub caps: c_int,
    pub pid_filter_count: c_int,
    pub int): *mut *mut *mut int (streaming_ctrl) (struct dvb_usb_adapter ,,
    pub int): *mut *mut *mut int (pid_filter_ctrl) (struct dvb_usb_adapter ,,
    pub int): *mut *mut *mut int (pid_filter) (struct dvb_usb_adapter , int, u16,,
    pub ): *mut *mut int (frontend_attach) (struct dvb_usb_adapter,
    pub ): *mut *mut int (tuner_attach) (struct dvb_usb_adapter,
    pub stream: usb_data_stream_properties,
    pub size_of_priv: c_int,
}

pub const MAX_NO_OF_FE_PER_ADAP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_adapter_properties {
    pub size_of_priv: c_int,
    pub int): *mut *mut *mut int (frontend_ctrl) (struct dvb_frontend ,,
    pub num_frontends: c_int,
    pub fe: [dvb_usb_adapter_fe_properties; MAX_NO_OF_FE_PER_ADAP],
}

//
// struct dvb_rc_legacy - old properties of remote controller
// @rc_map_table: a hard-wired array of struct rc_map_table (NULL to disable
// remote control handling).
// @rc_map_size: number of items in @rc_map_table.
// @rc_query: called to query an event event.
// @rc_interval: time in ms between two queries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_rc_legacy {
// remote control properties
pub const REMOTE_NO_KEY_PRESSED: c_uint = 0x00;
pub const REMOTE_KEY_PRESSED: c_uint = 0x01;
pub const REMOTE_KEY_REPEAT: c_uint = 0x02;
    pub rc_map_table: *mut rc_map_table,
    pub rc_map_size: c_int,
    pub ): *mut *mut *mut *mut int (rc_query) (struct dvb_usb_device , u32 , int,
    pub rc_interval: c_int,
}

//
// struct dvb_rc - properties of remote controller, using rc-core
// @rc_codes: name of rc codes table
// @protocol: type of protocol(s) currently used by the driver
// @allowed_protos: protocol(s) supported by the driver
// @driver_type: Used to point if a device supports raw mode
// @change_protocol: callback to change protocol
// @module_name: module name
// @rc_query: called to query an event event.
// @rc_interval: time in ms between two queries.
// @bulk_mode: device supports bulk mode for RC (disable polling mode)
// @scancode_mask: scancode mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_rc {
    pub rc_codes: *mut c_char,
    pub protocol: u64,
    pub allowed_protos: u64,
    pub driver_type: rc_driver_type,
    pub rc_proto): *mut *mut *mut int (change_protocol)(struct rc_dev dev, u64,
    pub module_name: *mut c_char,
    pub d): *mut *mut int (rc_query) (struct dvb_usb_device,
    pub rc_interval: c_int,
    pub /: *mut *mut bool bulk_mode; / uses bulk mode,
    pub scancode_mask: u32,
}

//
// enum dvb_usb_mode - Specifies if it is using a legacy driver or a new one
// based on rc-core
// This is initialized/used only inside dvb-usb-remote.c.
// It shouldn't be set by the drivers.
//
// @DVB_RC_LEGACY: legacy driver
// @DVB_RC_CORE: rc-core driver
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_usb_mode {
    DVB_RC_LEGACY,
    DVB_RC_CORE,
}

//
// struct dvb_usb_device_properties - properties of a dvb-usb-device
// @caps: capabilities
// @usb_ctrl: which USB device-side controller is in use. Needed for firmware
// download.
// @firmware: name of the firmware file.
// @download_firmware: called to download the firmware when the usb_ctrl is
// DEVICE_SPECIFIC.
// @no_reconnect: device doesn't do a reconnect after downloading the firmware,
// so do the warm initialization right after it
//
// @size_of_priv: how many bytes shall be allocated for the private field
// of struct dvb_usb_device.
// @priv_init: optional callback to initialize the variable that private field
// of struct dvb_usb_device has pointer to just after it had been allocated and
// zeroed.
// @priv_destroy: just like priv_init, only called before deallocating
// the memory pointed by private field of struct dvb_usb_device.
//
// @num_adapters: the number of adapters in @adapters
// @adapter: the adapters
// @power_ctrl: called to enable/disable power of the device.
// @read_mac_address: called to read the MAC address of the device.
// @identify_state: called to determine the state (cold or warm), when it
// is not distinguishable by the USB IDs.
//
// @rc: remote controller properties
//
// @i2c_algo: i2c_algorithm if the device has I2CoverUSB.
//
// @generic_bulk_ctrl_endpoint: most of the DVB USB devices have a generic
// endpoint which received control messages with bulk transfers. When this
// is non-zero, one can use dvb_usb_generic_rw and dvb_usb_generic_write-
// helper functions.
//
// @generic_bulk_ctrl_endpoint_response: some DVB USB devices use a separate
// endpoint for responses to control messages sent with bulk transfers via
// the generic_bulk_ctrl_endpoint. When this is non-zero, this will be used
// instead of the generic_bulk_ctrl_endpoint when reading usb responses in
// the dvb_usb_generic_rw helper function.
//
// @num_device_descs: number of struct dvb_usb_device_description in @devices
// @devices: array of struct dvb_usb_device_description compatibles with these
// properties.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_device_properties {
pub const MAX_NO_OF_ADAPTER_PER_DEVICE: c_int = 2;
pub const DVB_USB_IS_AN_I2C_ADAPTER: c_uint = 0x01;
    pub caps: c_int,
pub const DEVICE_SPECIFIC: c_int = 0;
pub const CYPRESS_AN2135: c_int = 1;
pub const CYPRESS_AN2235: c_int = 2;
pub const CYPRESS_FX2: c_int = 3;
    pub usb_ctrl: c_int,
    pub ): *const *const *const int (download_firmware) (struct usb_device , struct firmware,
    pub firmware: *const c_char,
    pub no_reconnect: c_int,
    pub size_of_priv: c_int,
    pub ): *mut *mut int (priv_init)(struct dvb_usb_device,
    pub ): *mut *mut void (priv_destroy)(struct dvb_usb_device,
    pub num_adapters: c_int,
    pub adapter: [dvb_usb_adapter_properties; MAX_NO_OF_ADAPTER_PER_DEVICE],
    pub int): *mut *mut *mut int (power_ctrl) (struct dvb_usb_device ,,
    pub []): *mut *mut *mut int (read_mac_address) (struct dvb_usb_device , u8,
    pub cold): *mut c_int,
    pub /: *mut *mut dvb_usb_mode mode; / Drivers shouldn't touch on it,
    pub legacy: dvb_rc_legacy,
    pub core: dvb_rc,
    pub rc: },
    pub i2c_algo: *const i2c_algorithm,
    pub generic_bulk_ctrl_endpoint: c_int,
    pub generic_bulk_ctrl_endpoint_response: c_int,
    pub num_device_descs: c_int,
    pub devices: [dvb_usb_device_description; 12],
}

//
// struct usb_data_stream - generic object of an USB stream
// @udev: the USB device
// @props: data stream properties
// @state: state of the stream
// @complete: complete callback
// @urb_list: list of URBs
// @buf_num: number of buffer allocated.
// @buf_size: size of each buffer in buf_list.
// @buf_list: array containing all allocate buffers for streaming.
// @dma_addr: list of dma_addr_t for each buffer in buf_list.
//
// @urbs_initialized: number of URBs initialized.
// @urbs_submitted: number of URBs submitted.
// @user_priv: for private use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_data_stream {
pub const MAX_NO_URBS_FOR_DATA_STREAM: c_int = 10;
    pub udev: *mut usb_device,
    pub props: usb_data_stream_properties,
pub const USB_STATE_INIT: c_uint = 0x00;
pub const USB_STATE_URB_BUF: c_uint = 0x01;
    pub state: c_int,
    pub size_t): *mut *mut *mut *mut void (complete) (struct usb_data_stream , u8 ,,
    pub urb_list: [*mut urb; MAX_NO_URBS_FOR_DATA_STREAM],
    pub buf_num: c_int,
    pub buf_size: c_ulong,
    pub buf_list: [*mut u8; MAX_NO_URBS_FOR_DATA_STREAM],
    pub dma_addr: [dma_addr_t; MAX_NO_URBS_FOR_DATA_STREAM],
    pub urbs_initialized: c_int,
    pub urbs_submitted: c_int,
    pub user_priv: *mut c_void,
}

//
// struct dvb_usb_fe_adapter - a DVB adapter on a USB device
// @fe: frontend
// @fe_init:  rerouted frontend-init (wakeup) function.
// @fe_sleep: rerouted frontend-sleep function.
// @stream: the usb data stream.
// @pid_filtering: is hardware pid_filtering used or not.
// @max_feed_count: how many feeds can be handled simultaneously by this
// device
// @priv: private pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_fe_adapter {
    pub fe: *mut dvb_frontend,
    pub ): *mut *mut int (fe_init) (struct dvb_frontend,
    pub ): *mut *mut int (fe_sleep) (struct dvb_frontend,
    pub stream: usb_data_stream,
    pub pid_filtering: c_int,
    pub max_feed_count: c_int,
    pub priv: *mut c_void,
}

//
// struct dvb_usb_adapter - a DVB adapter on a USB device
// @dev: DVB USB device pointer
// @props: properties
// @state: status
// @id: index of this adapter (starting with 0).
//
// @feedcount: number of requested feeds (used for streaming-activation)
//
// @dvb_adap: device's dvb_adapter.
// @dmxdev: device's dmxdev.
// @demux: device's software demuxer.
// @dvb_net: device's dvb_net interfaces.
//
// @fe_adap: frontend adapters
// @active_fe: active frontend
// @num_frontends_initialized: number of initialized frontends
// @priv: private pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_adapter {
    pub dev: *mut dvb_usb_device,
    pub props: dvb_usb_adapter_properties,
pub const DVB_USB_ADAP_STATE_INIT: c_uint = 0x000;
pub const DVB_USB_ADAP_STATE_DVB: c_uint = 0x001;
    pub state: c_int,
    pub id: u8,
    pub feedcount: c_int,
// dvb
    pub dvb_adap: dvb_adapter,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub dvb_net: dvb_net,
    pub fe_adap: [dvb_usb_fe_adapter; MAX_NO_OF_FE_PER_ADAP],
    pub active_fe: c_int,
    pub num_frontends_initialized: c_int,
    pub priv: *mut c_void,
}

//
// struct dvb_usb_device - object of a DVB USB device
// @props: copy of the struct dvb_usb_properties this device belongs to.
// @desc: pointer to the device's struct dvb_usb_device_description.
// @state: initialization and runtime state of the device.
//
// @powered: indicated whether the device is power or not.
// Powered is in/decremented for each call to modify the state.
// @udev: pointer to the device's struct usb_device.
//
// @data_mutex: mutex to protect the data structure used to store URB data
// @usb_mutex: mutex of USB control messages (reading needs two messages).
// Please notice that this mutex is used internally at the generic
// URB control functions. So, drivers using dvb_usb_generic_rw() and
// derivated functions should not lock it internally.
// @i2c_mutex: mutex for i2c-transfers
//
// @i2c_adap: device's i2c_adapter if it uses I2CoverUSB
//
// @num_adapters_initialized: number of initialized adapters
// @adapter: adapters
//
// @rc_dev: rc device for the remote control (rc-core mode)
// @input_dev: input device for the remote control (legacy mode)
// @rc_phys: rc device path
// @rc_query_work: struct work_struct frequent rc queries
// @last_event: last triggered event
// @last_state: last state (no, pressed, repeat)
// @owner: owner of the dvb_adapter
// @priv: private data of the actual driver (allocate by dvb-usb, size defined
// in size_of_priv of dvb_usb_properties).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_device {
    pub props: dvb_usb_device_properties,
    pub desc: *const dvb_usb_device_description,
    pub udev: *mut usb_device,
pub const DVB_USB_STATE_INIT: c_uint = 0x000;
pub const DVB_USB_STATE_I2C: c_uint = 0x001;
pub const DVB_USB_STATE_DVB: c_uint = 0x002;
pub const DVB_USB_STATE_REMOTE: c_uint = 0x004;
    pub state: c_int,
    pub powered: c_int,
// locking
    pub data_mutex: mutex,
    pub usb_mutex: mutex,
// i2c
    pub i2c_mutex: mutex,
    pub i2c_adap: i2c_adapter,
    pub num_adapters_initialized: c_int,
    pub adapter: [dvb_usb_adapter; MAX_NO_OF_ADAPTER_PER_DEVICE],
// remote control
    pub rc_dev: *mut rc_dev,
    pub input_dev: *mut input_dev,
    pub rc_phys: [c_char; 64],
    pub rc_query_work: delayed_work,
    pub last_event: u32,
    pub last_state: c_int,
    pub owner: *mut module,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn dvb_usb_device_exit(: *mut usb_interface);
}
// the generic read/write method for device control
// commonly used remote control parsing
// commonly used firmware download types and function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hexline {
    pub len: u8,
    pub addr: u32,
    pub type: u8,
    pub data: [u8; 255],
    pub chk: u8,
}

extern "C" {
    pub fn usb_cypress_load_firmware(udev: *mut usb_device, fw: *const firmware, type: c_int) -> c_int;
}
extern "C" {
    pub fn dvb_usb_get_hexline(fw: *const firmware, hx: *mut hexline, pos: *mut c_int) -> c_int;
}
