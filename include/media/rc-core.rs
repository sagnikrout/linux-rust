//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/rc-core.h
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
// Remote Controller core header
//
// Copyright (C) 2009-2010 by Mauro Carvalho Chehab
//

//
// enum rc_driver_type - type of the RC driver.
//
// @RC_DRIVER_SCANCODE:	 Driver or hardware generates a scancode.
// @RC_DRIVER_IR_RAW:	 Driver or hardware generates pulse/space sequences.
// It needs a Infra-Red pulse/space decoder
// @RC_DRIVER_IR_RAW_TX: Device transmitter only,
// driver requires pulse/space data sequence.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rc_driver_type {
    RC_DRIVER_SCANCODE = 0,
    RC_DRIVER_IR_RAW,
    RC_DRIVER_IR_RAW_TX,
}

//
// struct rc_scancode_filter - Filter scan codes.
// @data:	Scancode data to match.
// @mask:	Mask of bits of scancode to compare.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_scancode_filter {
    pub data: u32,
    pub mask: u32,
}

//
// enum rc_filter_type - Filter type constants.
// @RC_FILTER_NORMAL:	Filter for normal operation.
// @RC_FILTER_WAKEUP:	Filter for waking from suspend.
// @RC_FILTER_MAX:	Number of filter types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rc_filter_type {
    RC_FILTER_NORMAL = 0,
    RC_FILTER_WAKEUP,

    RC_FILTER_MAX
}

//
// struct lirc_fh - represents an open lirc file
// @list: list of open file handles
// @rc: rcdev for this lirc chardev
// @rawir: queue for incoming raw IR
// @scancodes: queue for incoming decoded scancodes
// @wait_poll: poll struct for lirc device
// @carrier_low: when setting the carrier range, first the low end must be
// set with an ioctl and then the high end with another ioctl
// @send_mode: lirc mode for sending, either LIRC_MODE_SCANCODE or
// LIRC_MODE_PULSE
// @rec_mode: lirc mode for receiving, either LIRC_MODE_SCANCODE or
// LIRC_MODE_MODE2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lirc_fh {
    pub list: list_head,
    pub rc: *mut rc_dev,
    pub int): DECLARE_KFIFO_PTR(rawir, unsigned,
    pub lirc_scancode): DECLARE_KFIFO_PTR(scancodes, struct,
    pub wait_poll: wait_queue_head_t,
    pub carrier_low: u32,
    pub send_mode: u8,
    pub rec_mode: u8,
}

//
// struct rc_dev - represents a remote control device
// @dev: driver model's view of this device
// @registered: set to true by rc_register_device(), false by
// rc_unregister_device
// @idle: used to keep track of RX state
// @encode_wakeup: wakeup filtering uses IR encode API, therefore the allowed
// wakeup protocols is the set of all raw encoders
// @minor: unique minor remote control device number
// @sysfs_groups: sysfs attribute groups
// @device_name: name of the rc child device
// @input_phys: physical path to the input child device
// @input_id: id of the input child device (struct input_id)
// @driver_name: name of the hardware driver which registered this device
// @map_name: name of the default keymap
// @rc_map: current scan/key table
// @lock: used to ensure we've filled in all protocol details before
// anyone can call show_protocols or store_protocols
// @raw: additional data for raw pulse/space devices
// @input_dev: the input child device used to communicate events to userspace
// @driver_type: specifies if protocol decoding is done in hardware or software
// @users: number of current users of the device
// @allowed_protocols: bitmask with the supported RC_PROTO_BIT_* protocols
// @enabled_protocols: bitmask with the enabled RC_PROTO_BIT_* protocols
// @allowed_wakeup_protocols: bitmask with the supported RC_PROTO_BIT_* wakeup
// protocols
// @wakeup_protocol: the enabled RC_PROTO_* wakeup protocol or
// RC_PROTO_UNKNOWN if disabled.
// @scancode_filter: scancode filter
// @scancode_wakeup_filter: scancode wakeup filters
// @scancode_mask: some hardware decoders are not capable of providing the full
// scancode to the application. As this is a hardware limit, we can't do
// anything with it. Yet, as the same keycode table can be used with other
// devices, a mask is provided to allow its usage. Drivers should generally
// leave this field in blank
// @priv: driver-specific data
// @keylock: protects the remaining members of the struct
// @keypressed: whether a key is currently pressed
// @last_toggle: toggle value of last command
// @last_keycode: keycode of last keypress
// @last_protocol: protocol of last keypress
// @last_scancode: scancode of last keypress
// @keyup_jiffies: time (in jiffies) when the current keypress should be released
// @timer_keyup: timer for releasing a keypress
// @timer_repeat: timer for autorepeat events. This is needed for CEC, which
// has non-standard repeats.
// @timeout: optional time after which device stops sending data
// @min_timeout: minimum timeout supported by device
// @max_timeout: maximum timeout supported by device
// @rx_resolution : resolution (in us) of input sampler
// @lirc_dev: lirc device
// @lirc_cdev: lirc char cdev
// @gap_start: start time for gap after timeout if non-zero
// @lirc_fh_lock: protects lirc_fh list
// @lirc_fh: list of open files
// @change_protocol: allow changing the protocol used on hardware decoders
// @open: callback to allow drivers to enable polling/irq when IR input device
// is opened.
// @close: callback to allow drivers to disable polling/irq when IR input device
// is opened.
// @s_tx_mask: set transmitter mask (for devices with multiple tx outputs)
// @s_tx_carrier: set transmit carrier frequency
// @s_tx_duty_cycle: set transmit duty cycle (0% - 100%)
// @s_rx_carrier_range: inform driver about carrier it is expected to handle
// @tx_ir: transmit IR
// @s_idle: enable/disable hardware idle mode, upon which,
// device doesn't interrupt host until it sees IR pulses
// @s_wideband_receiver: enable wide band receiver used for learning
// @s_carrier_report: enable carrier reports
// @s_filter: set the scancode filter
// @s_wakeup_filter: set the wakeup scancode filter. If the mask is zero
// then wakeup should be disabled. wakeup_protocol will be set to
// a valid protocol if mask is nonzero.
// @s_timeout: set hardware timeout in us
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_dev {
    pub dev: device,
    pub registered: bool,
    pub idle: bool,
    pub encode_wakeup: bool,
    pub minor: c_uint,
    pub sysfs_groups: [*const attribute_group; 5],
    pub device_name: *const c_char,
    pub input_phys: *const c_char,
    pub input_id: input_id,
    pub driver_name: *const c_char,
    pub map_name: *const c_char,
    pub rc_map: rc_map,
    pub lock: mutex,
    pub raw: *mut ir_raw_event_ctrl,
    pub input_dev: *mut input_dev,
    pub driver_type: rc_driver_type,
    pub users: u32,
    pub allowed_protocols: u64,
    pub enabled_protocols: u64,
    pub allowed_wakeup_protocols: u64,
    pub wakeup_protocol: rc_proto,
    pub scancode_filter: rc_scancode_filter,
    pub scancode_wakeup_filter: rc_scancode_filter,
    pub scancode_mask: u32,
    pub priv: *mut c_void,
    pub keylock: spinlock_t,
    pub keypressed: bool,
    pub last_toggle: u8,
    pub last_keycode: u32,
    pub last_protocol: rc_proto,
    pub last_scancode: u64,
    pub keyup_jiffies: c_ulong,
    pub timer_keyup: timer_list,
    pub timer_repeat: timer_list,
    pub timeout: u32,
    pub min_timeout: u32,
    pub max_timeout: u32,
    pub rx_resolution: u32,

    pub lirc_dev: device,
    pub lirc_cdev: cdev,
    pub gap_start: ktime_t,
    pub lirc_fh_lock: spinlock_t,
    pub lirc_fh: list_head,

    pub rc_proto): *mut *mut *mut int (change_protocol)(struct rc_dev dev, u64,
    pub dev): *mut *mut int (open)(struct rc_dev,
    pub dev): *mut *mut void (close)(struct rc_dev,
    pub mask): *mut *mut *mut int (s_tx_mask)(struct rc_dev dev, u32,
    pub carrier): *mut *mut *mut int (s_tx_carrier)(struct rc_dev dev, u32,
    pub duty_cycle): *mut *mut *mut int (s_tx_duty_cycle)(struct rc_dev dev, u32,
    pub max): *mut *mut *mut int (s_rx_carrier_range)(struct rc_dev dev, u32 min, u32,
    pub n): *mut *mut *mut *mut int (tx_ir)(struct rc_dev dev, unsigned txbuf, unsigned,
    pub enable): *mut *mut *mut void (s_idle)(struct rc_dev dev, bool,
    pub enable): *mut *mut *mut int (s_wideband_receiver)(struct rc_dev dev, int,
    pub enable): *mut *mut *mut int (s_carrier_report) (struct rc_dev dev, int,
    pub filter): *mut rc_scancode_filter,
    pub filter): *mut rc_scancode_filter,
    pub timeout): c_uint,
}

//
// From rc-main.c
// Those functions can be used on any type of Remote Controller. They
// basically creates an input_dev and properly reports the device as a
// Remote Controller, at sys/class/rc.
//
// rc_allocate_device - Allocates a RC device
//
// @rc_driver_type: specifies the type of the RC output to be allocated
// returns a pointer to struct rc_dev.
//
// devm_rc_allocate_device - Managed RC device allocation
//
// @dev: pointer to struct device
// @rc_driver_type: specifies the type of the RC output to be allocated
// returns a pointer to struct rc_dev.
//
// rc_free_device - Frees a RC device
//
// @dev: pointer to struct rc_dev.
//
extern "C" {
    pub fn rc_free_device(dev: *mut rc_dev);
}
//
// rc_register_device - Registers a RC device
//
// @dev: pointer to struct rc_dev.
//
extern "C" {
    pub fn rc_register_device(dev: *mut rc_dev) -> c_int;
}
//
// devm_rc_register_device - Manageded registering of a RC device
//
// @parent: pointer to struct device.
// @dev: pointer to struct rc_dev.
//
extern "C" {
    pub fn devm_rc_register_device(parent: *mut device, dev: *mut rc_dev) -> c_int;
}
//
// rc_unregister_device - Unregisters a RC device
//
// @dev: pointer to struct rc_dev.
//
extern "C" {
    pub fn rc_unregister_device(dev: *mut rc_dev);
}
extern "C" {
    pub fn rc_repeat(dev: *mut rc_dev);
}
extern "C" {
    pub fn rc_keyup(dev: *mut rc_dev);
}
extern "C" {
    pub fn rc_g_keycode_from_table(dev: *mut rc_dev, scancode: u64) -> u32;
}
//
// From rc-raw.c
// The Raw interface is specific to InfraRed. It may be a good idea to
// split it later into a separate header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_raw_event {
    pub duration: u32,
    pub carrier: u32,
}

extern "C" {
    pub fn ir_raw_event_handle(dev: *mut rc_dev);
}
extern "C" {
    pub fn ir_raw_event_store(dev: *mut rc_dev, ev: *mut ir_raw_event) -> c_int;
}
extern "C" {
    pub fn ir_raw_event_store_edge(dev: *mut rc_dev, pulse: bool) -> c_int;
}
extern "C" {
    pub fn ir_raw_event_set_idle(dev: *mut rc_dev, idle: bool);
}
extern "C" {
    pub fn ir_raw_encode_carrier(protocol: rc_proto) -> c_int;
}
// extract mask bits out of data and pack them into the result
// Get NEC scancode and protocol type from address and command bytes
// NEC transport, but modified protocol, used by at
// least Apple and TiVo remotes
//
// protocol = RC_PROTO_NEC32;
// Extended NEC
// protocol = RC_PROTO_NECX;
// Normal NEC
// protocol = RC_PROTO_NEC;
