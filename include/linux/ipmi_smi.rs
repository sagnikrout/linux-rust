//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ipmi_smi.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// ipmi_smi.h
//
// MontaVista IPMI system management interface
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//

//
// This files describes the interface for IPMI system management interface
// drivers to bind into the IPMI message handler.
//
// Structure for the low-level drivers.
//
// Flags for set_check_watch() below.  Tells if the SMI should be
// waiting for watchdog timeouts, commands and/or messages.
//

//
// SMI messages
//
// When communicating with an SMI, messages come in two formats:
//
// * Normal (to a BMC over a BMC interface)
//
// * IPMB (over a IPMB to another MC)
//
// When normal, commands are sent using the format defined by a
// standard message over KCS (NetFn must be even):
//
// +-----------+-----+------+
// | NetFn/LUN | Cmd | Data |
// +-----------+-----+------+
//
// And responses, similarly, with an completion code added (NetFn must
// be odd):
//
// +-----------+-----+------+------+
// | NetFn/LUN | Cmd | CC   | Data |
// +-----------+-----+------+------+
//
// With normal messages, only commands are sent and only responses are
// received.
//
// In IPMB mode, we are acting as an IPMB device. Commands will be in
// the following format (NetFn must be even):
//
// +-------------+------+-------------+-----+------+
// | NetFn/rsLUN | Addr | rqSeq/rqLUN | Cmd | Data |
// +-------------+------+-------------+-----+------+
//
// Responses will using the following format:
//
// +-------------+------+-------------+-----+------+------+
// | NetFn/rqLUN | Addr | rqSeq/rsLUN | Cmd | CC   | Data |
// +-------------+------+-------------+-----+------+------+
//
// This is similar to the format defined in the IPMB manual section
// 2.11.1 with the checksums and the first address removed.  Also, the
// address is always the remote address.
//
// IPMB messages can be commands and responses in both directions.
// Received commands are handled as received commands from the message
// queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipmi_smi_msg_type {
    IPMI_SMI_MSG_TYPE_NORMAL = 0,
    IPMI_SMI_MSG_TYPE_IPMB_DIRECT
}

//
// Messages to/from the lower layer.  The smi interface will take one
// of these to send. After the send has occurred and a response has
// been received, it will report this same data structure back up to
// the upper layer.  If an error occurs, it should fill in the
// response with an error code in the completion code location. When
// asynchronous data is received, one of these is allocated, the
// data_size is set to zero and the response holds the data from the
// get message or get event command that the interface initiated.
// Note that it is the interfaces responsibility to detect
// asynchronous data and messages and request them from the
// interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_smi_msg {
    pub link: list_head,
    pub type: ipmi_smi_msg_type,
    pub msgid: c_long,
// Response to this message, will be NULL if not from a user request.
    pub recv_msg: *mut ipmi_recv_msg,
    pub data_size: c_int,
    pub data: [c_uchar; IPMI_MAX_MSG_LENGTH],
    pub rsp_size: c_int,
    pub rsp: [c_uchar; IPMI_MAX_MSG_LENGTH],
//
// Will be called when the system is done with the message
// (presumably to free it).
//
    pub msg): *mut *mut void (done)(struct ipmi_smi_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_smi_handlers {
    pub owner: *mut module,
// Capabilities of the SMI.

    pub flags: c_uint,
//
// The low-level interface cannot start sending messages to
// the upper layer until this function is called.  This may
// not be NULL, the lower layer must take the interface from
// this call.
//
    pub new_intf): *mut ipmi_smi,
//
// When called, the low-level interface should disable all
// processing, it should be complete shut down when it returns.
//
    pub send_info): *mut *mut void (shutdown)(void,
//
// Get the detailed private info of the low level interface and store
// it into the structure of ipmi_smi_data. For example: the
// ACPI device handle will be returned for the pnp_acpi IPMI device.
//
    pub data): *mut *mut *mut int (get_smi_info)(void send_info, struct ipmi_smi_info,
//
// Called to enqueue an SMI message to be sent.  This
// operation is not allowed to fail.  If an error occurs, it
// should report back the error in a received message.  It may
// do this in the current call context, since no write locks
// are held when this is run.  Message are delivered one at
// a time by the message handler, a new message will not be
// delivered until the previous message is returned.
//
// This can return an error if the SMI is not in a state where it
// can send a message.
//
    pub msg): *mut *mut *mut int (sender)(void send_info, struct ipmi_smi_msg,
//
// Called by the upper layer to request that we try to get
// events from the BMC we are attached to.
//
    pub send_info): *mut *mut void (request_events)(void,
//
// Called by the upper layer when some user requires that the
// interface watch for received messages and watchdog
// pretimeouts (basically do a "Get Flags", or not.  Used by
// the SMI to know if it should watch for these.  This may be
// NULL if the SMI does not implement it.  watch_mask is from
// IPMI_WATCH_MASK_xxx above.  The interface should run slower
// timeouts for just watchdog checking or faster timeouts when
// waiting for the message queue.
//
    pub watch_mask): *mut *mut *mut void (set_need_watch)(void send_info, unsigned int,
//
// Called when flushing all pending messages.
//
    pub send_info): *mut *mut void (flush_messages)(void,
//
// Called when the interface should go into "run to
// completion" mode.  If this call sets the value to true, the
// interface should make sure that all messages are flushed
// out and that none are pending, and any new requests are run
// to completion immediately.
//
    pub run_to_completion): *mut *mut *mut void (set_run_to_completion)(void send_info, bool,
//
// Called to poll for work to do.  This is so upper layers can
// poll for operations during things like crash dumps.
//
    pub send_info): *mut *mut void (poll)(void,
//
// Enable/disable firmware maintenance mode.  Note that this
// is *not* the modes defined, this is simply an on/off
// setting.  The message handler does the mode handling.  Note
// that this is called from interrupt context, so it cannot
// block.
//
    pub enable): *mut *mut *mut void (set_maintenance_mode)(void send_info, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_device_id {
    pub device_id: c_uchar,
    pub device_revision: c_uchar,
    pub firmware_revision_1: c_uchar,
    pub firmware_revision_2: c_uchar,
    pub ipmi_version: c_uchar,
    pub additional_device_support: c_uchar,
    pub manufacturer_id: c_uint,
    pub product_id: c_uint,
    pub aux_firmware_revision: [c_uchar; 4],
    pub 1: unsigned int aux_firmware_revision_set :,
}

//
// Take a pointer to an IPMI response and extract device id information from
// it. @netfn is in the IPMI_NETFN_ format, so may need to be shifted from
// a SI response.
//
// Strange, didn't get the response we expected.
// That's odd, it shouldn't be able to fail.
//
// Add a low-level interface to the IPMI driver.  Note that if the
// interface doesn't know its slave address, it should pass in zero.
// The low-level interface should not deliver any messages to the
// upper layer until the start_processing() function in the handlers
// is called, and the lower layer must get the interface from that
// call.
//

//
// Remove a low-level interface from the IPMI driver.  This will
// return an error if the interface is still in use by a user.
//
extern "C" {
    pub fn ipmi_unregister_smi(intf: *mut ipmi_smi);
}
//
// The lower layer reports received messages through this interface.
// The data_size should be zero if this is an asynchronous message.  If
// the lower layer gets an error sending a message, it should format
// an error response in the message response.
//
// The lower layer received a watchdog pre-timeout on interface.
extern "C" {
    pub fn ipmi_smi_watchdog_pretimeout(intf: *mut ipmi_smi);
}
