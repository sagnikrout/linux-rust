//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/dvb_usb.h
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
// DVB USB framework
//
// Copyright (C) 2004-6 Patrick Boettcher <patrick.boettcher@posteo.de>
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

//
// device file: /dev/dvb/adapter[0-1]/frontend[0-2]
//
// |-- device
// |   |-- adapter0
// |   |   |-- frontend0
// |   |   |-- frontend1
// |   |   `-- frontend2
// |   `-- adapter1
// |       |-- frontend0
// |       |-- frontend1
// |       `-- frontend2
//
// Commonly used variable names:
// d = pointer to device (struct dvb_usb_device *)
// adap = pointer to adapter (struct dvb_usb_adapter *)
// fe = pointer to frontend (struct dvb_frontend *)
//
// Use macros defined in that file to resolve needed pointers.
//
// helper macros for every DVB USB driver use

//
// struct dvb_usb_driver_info - structure for carrying all needed data from the
// device driver to the general
// dvb usb routines
// @name: device name
// @rc_map: name of rc codes table
// @props: structure containing all device properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_driver_info {
    pub name: *const c_char,
    pub rc_map: *const c_char,
    pub props: *const dvb_usb_device_properties,
}

//
// struct dvb_usb_rc - structure for remote controller configuration
// @map_name: name of rc codes table
// @allowed_protos: protocol(s) supported by the driver
// @change_protocol: callback to change protocol
// @query: called to query an event from the device
// @interval: time in ms between two queries
// @driver_type: used to point if a device supports raw mode
// @bulk_mode: device supports bulk mode for rc (disable polling mode)
// @timeout: set to length of last space before raw IR goes idle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_rc {
    pub map_name: *const c_char,
    pub allowed_protos: u64,
    pub rc_proto): *mut *mut *mut int (change_protocol)(struct rc_dev dev, u64,
    pub d): *mut *mut int (query) (struct dvb_usb_device,
    pub interval: c_uint,
    pub driver_type: rc_driver_type,
    pub bulk_mode: bool,
    pub timeout: c_int,
}

//
// struct usb_data_stream_properties - usb streaming configuration for adapter
// @type: urb type
// @count: count of used urbs
// @endpoint: stream usb endpoint number
// @u: union for @bulk and @isoc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_data_stream_properties {
pub const USB_BULK: c_int = 1;
pub const USB_ISOC: c_int = 2;
    pub type: u8,
    pub count: u8,
    pub endpoint: u8,
    pub /: *mut *mut unsigned int buffersize; / per URB,
    pub bulk: },
    pub framesperurb: c_int,
    pub framesize: c_int,
    pub interval: c_int,
    pub isoc: },
    pub u: },
}

//
// struct dvb_usb_adapter_properties - properties of dvb usb device adapter
// @caps: adapter capabilities
// @pid_filter_count: pid count of adapter pid-filter
// @pid_filter_ctrl: called to enable/disable pid-filter
// @pid_filter: called to set/unset pid for filtering
// @stream: adapter usb stream configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_adapter_properties {
pub const MAX_NO_OF_FE_PER_ADAP: c_int = 3;
pub const DVB_USB_ADAP_HAS_PID_FILTER: c_uint = 0x01;
pub const DVB_USB_ADAP_PID_FILTER_CAN_BE_TURNED_OFF: c_uint = 0x02;
pub const DVB_USB_ADAP_NEED_PID_FILTERING: c_uint = 0x04;
    pub caps: u8,
    pub pid_filter_count: u8,
    pub int): *mut *mut *mut int (pid_filter_ctrl) (struct dvb_usb_adapter ,,
    pub int): *mut *mut *mut int (pid_filter) (struct dvb_usb_adapter , int, u16,,
    pub stream: usb_data_stream_properties,
}

//
// struct dvb_usb_device_properties - properties of a dvb-usb-device
// @driver_name: name of the owning driver module
// @owner: owner of the dvb_adapter
// @adapter_nr: values from the DVB_DEFINE_MOD_OPT_ADAPTER_NR() macro
// @bInterfaceNumber: usb interface number driver binds
// @size_of_priv: bytes allocated for the driver private data
// @generic_bulk_ctrl_endpoint: bulk control endpoint number for sent
// @generic_bulk_ctrl_endpoint_response: bulk control endpoint number for
// receive
// @generic_bulk_ctrl_delay: delay between bulk control sent and receive message
// @probe: like probe on driver model
// @disconnect: like disconnect on driver model
// @identify_state: called to determine the firmware state (cold or warm) and
// return possible firmware file name to be loaded
// @firmware: name of the firmware file to be loaded
// @download_firmware: called to download the firmware
// @i2c_algo: i2c_algorithm if the device has i2c-adapter
// @num_adapters: dvb usb device adapter count
// @get_adapter_count: called to resolve adapter count
// @adapter: array of all adapter properties of device
// @power_ctrl: called to enable/disable power of the device
// @read_config: called to resolve device configuration
// @read_mac_address: called to resolve adapter mac-address
// @frontend_attach: called to attach the possible frontends
// @frontend_detach: called to detach the possible frontends
// @tuner_attach: called to attach the possible tuners
// @tuner_detach: called to detach the possible tuners
// @frontend_ctrl: called to power on/off active frontend
// @streaming_ctrl: called to start/stop the usb streaming of adapter
// @init: called after adapters are created in order to finalize device
// configuration
// @exit: called when driver is unloaded
// @get_rc_config: called to resolve used remote controller configuration
// @get_stream_config: called to resolve input and output stream configuration
// of the adapter just before streaming is started. input stream is transport
// stream from the demodulator and output stream is usb stream to host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_device_properties {
pub const MAX_NO_OF_ADAPTER_PER_DEVICE: c_int = 2;
    pub driver_name: *const c_char,
    pub owner: *mut module,
    pub adapter_nr: *mut c_short,
    pub bInterfaceNumber: u8,
    pub size_of_priv: c_uint,
    pub generic_bulk_ctrl_endpoint: u8,
    pub generic_bulk_ctrl_endpoint_response: u8,
    pub generic_bulk_ctrl_delay: c_uint,
    pub ): *mut *mut int (probe)(struct dvb_usb_device,
    pub ): *mut *mut void (disconnect)(struct dvb_usb_device,
pub const WARM: c_int = 0;
pub const COLD: c_int = 1;
    pub ): *const *const *const int (identify_state) (struct dvb_usb_device , char,
    pub firmware: *const c_char,
pub const RECONNECTS_USB: c_int = 1;
    pub ): *const firmware,
    pub i2c_algo: *const i2c_algorithm,
    pub num_adapters: c_uint,
    pub ): *mut *mut int (get_adapter_count) (struct dvb_usb_device,
    pub adapter: [dvb_usb_adapter_properties; MAX_NO_OF_ADAPTER_PER_DEVICE],
    pub int): *mut *mut *mut int (power_ctrl) (struct dvb_usb_device ,,
    pub d): *mut *mut int (read_config) (struct dvb_usb_device,
    pub []): *mut *mut *mut int (read_mac_address) (struct dvb_usb_adapter , u8,
    pub ): *mut *mut int (frontend_attach) (struct dvb_usb_adapter,
    pub ): *mut *mut int (frontend_detach)(struct dvb_usb_adapter,
    pub ): *mut *mut int (tuner_attach) (struct dvb_usb_adapter,
    pub ): *mut *mut int (tuner_detach)(struct dvb_usb_adapter,
    pub int): *mut *mut *mut int (frontend_ctrl) (struct dvb_frontend ,,
    pub int): *mut *mut *mut int (streaming_ctrl) (struct dvb_frontend ,,
    pub ): *mut *mut int (init) (struct dvb_usb_device,
    pub ): *mut *mut void (exit) (struct dvb_usb_device,
    pub ): *mut *mut *mut int (get_rc_config) (struct dvb_usb_device , struct dvb_usb_rc,
pub const DVB_USB_FE_TS_TYPE_188: c_int = 0;
pub const DVB_USB_FE_TS_TYPE_204: c_int = 1;
pub const DVB_USB_FE_TS_TYPE_RAW: c_int = 2;
    pub ): *mut usb_data_stream_properties,
}

//
// struct usb_data_stream - generic object of an usb stream
// @udev: USB device
// @props: properties
// @state: state of the data stream
// @complete: complete callback
// @urb_list: list of URBs
// @buf_num: number of buffer allocated
// @buf_size: size of each buffer in buf_list
// @buf_list: array containing all allocate buffers for streaming
// @dma_addr: list of dma_addr_t for each buffer in buf_list
//
// @urbs_initialized: number of URBs initialized
// @urbs_submitted: number of URBs submitted
// @user_priv: private pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_data_stream {
pub const MAX_NO_URBS_FOR_DATA_STREAM: c_int = 10;
    pub udev: *mut usb_device,
    pub props: usb_data_stream_properties,
pub const USB_STATE_INIT: c_uint = 0x00;
pub const USB_STATE_URB_BUF: c_uint = 0x01;
    pub state: u8,
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
// struct dvb_usb_adapter - dvb adapter object on dvb usb device
// @props: pointer to adapter properties
// @stream: adapter the usb data stream
// @id: index of this adapter (starting with 0)
// @ts_type: transport stream, input stream, type
// @suspend_resume_active: set when there is ongoing suspend / resume
// @pid_filtering: is hardware pid_filtering used or not
// @feed_count: current feed count
// @max_feed_count: maimum feed count device can handle
// @active_fe: active frontend
// @state_bits: status bits
// @dvb_adap: adapter dvb_adapter
// @dmxdev: adapter dmxdev
// @demux: adapter software demuxer
// @dvb_net: adapter dvb_net interfaces
// @fe: adapter frontends
// @fe_init: rerouted frontend-init function
// @fe_sleep: rerouted frontend-sleep function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_adapter {
    pub props: *const dvb_usb_adapter_properties,
    pub stream: usb_data_stream,
    pub id: u8,
    pub ts_type: u8,
    pub suspend_resume_active: bool,
    pub pid_filtering: bool,
    pub feed_count: u8,
    pub max_feed_count: u8,
    pub active_fe: i8,
pub const ADAP_INIT: c_int = 0;
pub const ADAP_SLEEP: c_int = 1;
pub const ADAP_STREAMING: c_int = 2;
    pub state_bits: c_ulong,
// dvb
    pub dvb_adap: dvb_adapter,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub dvb_net: dvb_net,
    pub fe: [*mut dvb_frontend; MAX_NO_OF_FE_PER_ADAP],
    pub ): *mut *mut int (fe_init[MAX_NO_OF_FE_PER_ADAP]) (struct dvb_frontend,
    pub ): *mut *mut int (fe_sleep[MAX_NO_OF_FE_PER_ADAP]) (struct dvb_frontend,
}

//
// struct dvb_usb_device - dvb usb device object
// @props: device properties
// @name: device name
// @rc_map: name of rc codes table
// @rc_polling_active: set when RC polling is active
// @intf: pointer to the device's struct usb_interface
// @udev: pointer to the device's struct usb_device
// @rc: remote controller configuration
// @powered: indicated whether the device is power or not
// @usb_mutex: mutex for usb control messages
// @i2c_mutex: mutex for i2c-transfers
// @i2c_adap: device's i2c-adapter
// @adapter: adapters
// @rc_dev: rc device for the remote control
// @rc_phys: rc path
// @rc_query_work: work for polling remote
// @priv: private data of the actual driver (allocate by dvb usb, size defined
// in size_of_priv of dvb_usb_properties).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_usb_device {
    pub props: *const dvb_usb_device_properties,
    pub name: *const c_char,
    pub rc_map: *const c_char,
    pub rc_polling_active: bool,
    pub intf: *mut usb_interface,
    pub udev: *mut usb_device,
    pub rc: dvb_usb_rc,
    pub powered: c_int,
// locking
    pub usb_mutex: mutex,
// i2c
    pub i2c_mutex: mutex,
    pub i2c_adap: i2c_adapter,
    pub adapter: [dvb_usb_adapter; MAX_NO_OF_ADAPTER_PER_DEVICE],
// remote control
    pub rc_dev: *mut rc_dev,
    pub rc_phys: [c_char; 64],
    pub rc_query_work: delayed_work,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn dvb_usbv2_disconnect(: *mut usb_interface);
}
extern "C" {
    pub fn dvb_usbv2_suspend(: *mut usb_interface, _arg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn dvb_usbv2_resume(: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn dvb_usbv2_reset_resume(: *mut usb_interface) -> c_int;
}
// the generic read/write method for device control
extern "C" {
    pub fn dvb_usbv2_generic_rw(: *mut dvb_usb_device, : *mut u8, _arg: u16, : *mut u8, _arg: u16) -> c_int;
}
extern "C" {
    pub fn dvb_usbv2_generic_write(: *mut dvb_usb_device, : *mut u8, _arg: u16) -> c_int;
}
// caller must hold lock when locked versions are called
extern "C" {
    pub fn dvb_usbv2_generic_write_locked(: *mut dvb_usb_device, : *mut u8, _arg: u16) -> c_int;
}
