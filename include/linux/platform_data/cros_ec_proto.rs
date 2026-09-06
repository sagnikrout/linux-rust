//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/cros_ec_proto.h
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
// ChromeOS Embedded Controller protocol interface.
//
// Copyright (C) 2012 Google, Inc
//

pub const CROS_EC_DEV_EC_INDEX: c_int = 0;
pub const CROS_EC_DEV_PD_INDEX: c_int = 1;
//
// The EC is unresponsive for a time after a reboot command.  Add a
// simple delay to make sure that the bus stays locked.
//
pub const EC_REBOOT_DELAY_MS: c_int = 50;
//
// Max bus-specific overhead incurred by request/responses.
//
// Request:
// - I2C requires 1 byte (see struct ec_host_request_i2c).
// - ISHTP requires 4 bytes (see struct cros_ish_out_msg).
//
// Response:
// - I2C requires 2 bytes (see struct ec_host_response_i2c).
// - ISHTP requires 4 bytes (see struct cros_ish_in_msg).
// - SPI requires 32 bytes (see EC_MSG_PREAMBLE_COUNT).
//
pub const EC_PROTO_VERSION_UNKNOWN: c_int = 0;
pub const EC_MAX_REQUEST_OVERHEAD: c_int = 4;
pub const EC_MAX_RESPONSE_OVERHEAD: c_int = 32;
//
// ACPI notify value for MKBP host event.
//
pub const ACPI_NOTIFY_CROS_EC_MKBP: c_uint = 0x80;
//
// EC panic is not covered by the standard (0-F) ACPI notify values.
// Arbitrarily choosing B0 to notify ec panic, which is in the 84-BF
// device specific ACPI notify range.
//
pub const ACPI_NOTIFY_CROS_EC_PANIC: c_uint = 0xB0;
//
// Command interface between EC and AP, for LPC, I2C and SPI interfaces.
//
// Max length of messages for proto 2
//
// struct cros_ec_command - Information about a ChromeOS EC command.
// @version: Command version number (often 0).
// @command: Command to send (EC_CMD_...).
// @outsize: Outgoing length in bytes.
// @insize: Max number of bytes to accept from the EC.
// @result: EC's response to the command (separate from communication failure).
// @data: Where to put the incoming data from EC and outgoing data to EC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_command {
    pub version: u32,
    pub command: u32,
    pub outsize: u32,
    pub insize: u32,
    pub result: u32,
    pub data: [u8; ],
}

//
// struct cros_ec_device - Information about a ChromeOS EC device.
// @phys_name: Name of physical comms layer (e.g. 'i2c-4').
// @dev: Device pointer for physical comms device
// @cros_class: The class structure for this device.
// @cmd_readmem: Direct read of the EC memory-mapped region, if supported.
// @offset: Is within EC_LPC_ADDR_MEMMAP region.
// @bytes: Number of bytes to read. zero means "read a string" (including
// the trailing '\0'). At most only EC_MEMMAP_SIZE bytes can be
// read. Caller must ensure that the buffer is large enough for the
// result when reading a string.
// @max_request: Max size of message requested.
// @max_response: Max size of message response.
// @max_passthru: Max sice of passthru message.
// @proto_version: The protocol version used for this device.
// @priv: Private data.
// @irq: Interrupt to use.
// @id: Device id.
// @din: Input buffer (for data from EC). This buffer will always be
// dword-aligned and include enough space for up to 7 word-alignment
// bytes also, so we can ensure that the body of the message is always
// dword-aligned (64-bit). We use this alignment to keep ARM and x86
// happy. Probably word alignment would be OK, there might be a small
// performance advantage to using dword.
// @dout: Output buffer (for data to EC). This buffer will always be
// dword-aligned and include enough space for up to 7 word-alignment
// bytes also, so we can ensure that the body of the message is always
// dword-aligned (64-bit). We use this alignment to keep ARM and x86
// happy. Probably word alignment would be OK, there might be a small
// performance advantage to using dword.
// @din_size: Size of din buffer to allocate (zero to use static din).
// @dout_size: Size of dout buffer to allocate (zero to use static dout).
// @wake_enabled: True if this device can wake the system from sleep.
// @suspended: True if this device had been suspended.
// @registered: True if this device had been registered.
// @cmd_xfer: Send command to EC and get response.
// Returns the number of bytes received if the communication
// succeeded, but that doesn't mean the EC was happy with the
// command. The caller should check msg.result for the EC's result
// code.
// @pkt_xfer: Send packet to EC and get response.
// @lockdep_key: Lockdep class for each instance. Unused if CONFIG_LOCKDEP is
// not enabled.
// @lock: One transaction at a time.
// @mkbp_event_supported: 0 if MKBP not supported. Otherwise its value is
// the maximum supported version of the MKBP host event
// command + 1.
// @host_sleep_v1: True if this EC supports the sleep v1 command.
// @event_notifier: Interrupt event notifier for transport devices.
// @event_data: Raw payload transferred with the MKBP event.
// @event_size: Size in bytes of the event data.
// @host_event_wake_mask: Mask of host events that cause wake from suspend.
// @suspend_timeout_ms: The timeout in milliseconds between when sleep event
// is received and when the EC will declare sleep
// transition failure if the sleep signal is not
// asserted.  See also struct
// ec_params_host_sleep_event_v1 in cros_ec_commands.h.
// @last_resume_result: The number of sleep power signal transitions that
// occurred since the suspend message. The high bit
// indicates a timeout occurred.  See also struct
// ec_response_host_sleep_event_v1 in cros_ec_commands.h.
// @last_event_time: exact time from the hard irq when we got notified of
// a new event.
// @notifier_ready: The notifier_block to let the kernel re-query EC
// communication protocol when the EC sends
// EC_HOST_EVENT_INTERFACE_READY.
// @ec: The platform_device used by the mfd driver to interface with the
// main EC.
// @pd: The platform_device used by the mfd driver to interface with the
// PD behind an EC.
// @panic_notifier: EC panic notifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_device {
// These are used by other drivers that want to talk to the EC
    pub phys_name: *const c_char,
    pub dev: *mut device,
    pub cros_class: *mut class,
    pub dest): *mut unsigned int bytes, void,
// These are used to implement the platform-specific interface
    pub max_request: u16,
    pub max_response: u16,
    pub max_passthru: u16,
    pub proto_version: u16,
    pub priv: *mut c_void,
    pub irq: c_int,
    pub din: *mut u8,
    pub dout: *mut u8,
    pub din_size: c_int,
    pub dout_size: c_int,
    pub wake_enabled: bool,
    pub suspended: bool,
    pub registered: bool,
    pub msg): *mut cros_ec_command,
    pub msg): *mut cros_ec_command,
    pub lockdep_key: lock_class_key,
    pub lock: mutex,
    pub mkbp_event_supported: u8,
    pub host_sleep_v1: bool,
    pub event_notifier: blocking_notifier_head,
    pub event_data: ec_response_get_next_event_v3,
    pub event_size: c_int,
    pub host_event_wake_mask: u32,
    pub last_resume_result: u32,
    pub suspend_timeout_ms: u16,
    pub last_event_time: ktime_t,
    pub notifier_ready: notifier_block,
// The platform devices used by the mfd driver
    pub ec: *mut platform_device,
    pub pd: *mut platform_device,
    pub panic_notifier: blocking_notifier_head,
}

//
// struct cros_ec_platform - ChromeOS EC platform information.
// @ec_name: Name of EC device (e.g. 'cros-ec', 'cros-pd', ...)
// used in /dev/ and sysfs.
// @cmd_offset: Offset to apply for each command. Set when
// registering a device behind another one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_platform {
    pub ec_name: *const c_char,
    pub cmd_offset: u16,
}

//
// struct cros_ec_dev - ChromeOS EC device entry point.
// @class_dev: Device structure used in sysfs.
// @group: sysfs attributes groups for this EC.
// @ec_dev: cros_ec_device structure to talk to the physical device.
// @dev: Pointer to the platform device.
// @debug_info: cros_ec_debugfs structure for debugging information.
// @has_kb_wake_angle: True if at least 2 accelerometer are connected to the EC.
// @cmd_offset: Offset to apply for each command.
// @features: Features supported by the EC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_dev {
    pub class_dev: device,
    pub group: *const attribute_group,
    pub ec_dev: *mut cros_ec_device,
    pub dev: *mut device,
    pub debug_info: *mut cros_ec_debugfs,
    pub has_kb_wake_angle: bool,
    pub cmd_offset: u16,
    pub features: ec_response_get_features,
}

extern "C" {
    pub fn cros_ec_rwsig_continue(ec_dev: *mut cros_ec_device) -> c_int;
}
extern "C" {
    pub fn cros_ec_query_all(ec_dev: *mut cros_ec_device) -> c_int;
}
extern "C" {
    pub fn cros_ec_get_host_event(ec_dev: *mut cros_ec_device) -> u32;
}
extern "C" {
    pub fn cros_ec_read_features(ec: *mut cros_ec_dev) -> c_int;
}
extern "C" {
    pub fn cros_ec_check_features(ec: *mut cros_ec_dev, feature: c_int) -> bool;
}
extern "C" {
    pub fn cros_ec_get_sensor_count(ec: *mut cros_ec_dev) -> c_int;
}
extern "C" {
    pub fn cros_ec_cmd_readmem(ec_dev: *mut cros_ec_device, offset: u8, size: u8, dest: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cros_ec_get_cmd_versions(ec_dev: *mut cros_ec_device, cmd: u16) -> c_int;
}
extern "C" {
    pub fn cros_ec_device_registered(ec_dev: *mut cros_ec_device) -> bool;
}
//
// cros_ec_get_time_ns() - Return time in ns.
//
// This is the function used to record the time for last_event_time in struct
// cros_ec_device during the hard irq.
//
// Return: ktime_t format since boot.
//
extern "C" {
    pub fn ktime_get_boottime_ns() -> return;
}
