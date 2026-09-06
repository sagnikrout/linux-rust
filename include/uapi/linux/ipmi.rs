//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ipmi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// ipmi.h
//
// MontaVista IPMI interface
//
// Author: MontaVista Software, Inc.
// Corey Minyard <minyard@mvista.com>
// source@mvista.com
//
// Copyright 2002 MontaVista Software Inc.
//

//
// This file describes an interface to an IPMI driver.  You have to
// have a fairly good understanding of IPMI to use this, so go read
// the specs first before actually trying to do anything.
//
// With that said, this driver provides a multi-user interface to the
// IPMI driver, and it allows multiple IPMI physical interfaces below
// the driver.  The physical interfaces bind as a lower layer on the
// driver.  They appear as interfaces to the application using this
// interface.
//
// Multi-user means that multiple applications may use the driver,
// send commands, receive responses, etc.  The driver keeps track of
// commands the user sends and tracks the responses.  The responses
// will go back to the application that send the command.  If the
// response doesn't come back in time, the driver will return a
// timeout error response to the application.  Asynchronous events
// from the BMC event queue will go to all users bound to the driver.
// The incoming event queue in the BMC will automatically be flushed
// if it becomes full and it is queried once a second to see if
// anything is in it.  Incoming commands to the driver will get
// delivered as commands.
//
// This is an overlay for all the address types, so it's easy to
// determine the actual address type.  This is kind of like addresses
// work for sockets.
//
pub const IPMI_MAX_ADDR_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_addr {
// Try to take these from the "Channel Medium Type" table
    pub addr_type: c_int,
    pub channel: c_short,
    pub data: [c_char; IPMI_MAX_ADDR_SIZE],
}

//
// When the address is not used, the type will be set to this value.
// The channel is the BMC's channel number for the channel (usually
// 0), or IPMC_BMC_CHANNEL if communicating directly with the BMC.
//
pub const IPMI_SYSTEM_INTERFACE_ADDR_TYPE: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_system_interface_addr {
    pub addr_type: c_int,
    pub channel: c_short,
    pub lun: c_uchar,
}

// An IPMB Address.
pub const IPMI_IPMB_ADDR_TYPE: c_uint = 0x01;
// Used for broadcast get device id as described in section 17.9 of the
pub const IPMI_IPMB_BROADCAST_ADDR_TYPE: c_uint = 0x41;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_ipmb_addr {
    pub addr_type: c_int,
    pub channel: c_short,
    pub slave_addr: c_uchar,
    pub lun: c_uchar,
}

//
// Used for messages received directly from an IPMB that have not gone
// through a MC.  This is for systems that sit right on an IPMB so
// they can receive commands and respond to them.
//
pub const IPMI_IPMB_DIRECT_ADDR_TYPE: c_uint = 0x81;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_ipmb_direct_addr {
    pub addr_type: c_int,
    pub channel: c_short,
    pub slave_addr: c_uchar,
    pub rs_lun: c_uchar,
    pub rq_lun: c_uchar,
}

//
// A LAN Address.  This is an address to/from a LAN interface bridged
// by the BMC, not an address actually out on the LAN.
//
// A conscious decision was made here to deviate slightly from the IPMI
// spec.  We do not use rqSWID and rsSWID like it shows in the
// message.  Instead, we use remote_SWID and local_SWID.  This means
// that any message (a request or response) from another device will
// always have exactly the same address.  If you didn't do this,
// requests and responses from the same device would have different
// addresses, and that's not too cool.
//
// In this address, the remote_SWID is always the SWID the remote
// message came from, or the SWID we are sending the message to.
// local_SWID is always our SWID.  Note that having our SWID in the
// message is a little weird, but this is required.
//
pub const IPMI_LAN_ADDR_TYPE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_lan_addr {
    pub addr_type: c_int,
    pub channel: c_short,
    pub privilege: c_uchar,
    pub session_handle: c_uchar,
    pub remote_SWID: c_uchar,
    pub local_SWID: c_uchar,
    pub lun: c_uchar,
}

//
// Channel for talking directly with the BMC.  When using this
// channel, This is for the system interface address type only.  FIXME
// - is this right, or should we use -1?
//
pub const IPMI_BMC_CHANNEL: c_uint = 0xf;
pub const IPMI_NUM_CHANNELS: c_uint = 0x10;
//
// Used to signify an "all channel" bitmask.  This is more than the
// actual number of channels because this is used in userland and
// will cover us if the number of channels is extended.
//

//
// A raw IPMI message without any addressing.  This covers both
// commands and responses.  The completion code is always the first
// byte of data in the response (as the spec shows the messages laid
// out).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_msg {
    pub netfn: c_uchar,
    pub cmd: c_uchar,
    pub data_len: c_ushort,
    pub data: *mut unsigned char __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_ipmi_msg {
    pub netfn: c_uchar,
    pub cmd: c_uchar,
    pub data_len: c_ushort,
    pub data: *mut c_uchar,
}

//
// Various defines that are useful for IPMI applications.
//
pub const IPMI_INVALID_CMD_COMPLETION_CODE: c_uint = 0xC1;
pub const IPMI_TIMEOUT_COMPLETION_CODE: c_uint = 0xC3;
pub const IPMI_UNKNOWN_ERR_COMPLETION_CODE: c_uint = 0xff;
//
// Receive types for messages coming from the receive interface.  This
// is used for the receive in-kernel interface and in the receive
// IOCTL.
//
// The "IPMI_RESPONSE_RESPONSE_TYPE" is a little strange sounding, but
// it allows you to get the message results when you send a response
// message.
//

// Note that async events and received commands do not have a completion
//
// Modes for ipmi_set_maint_mode() and the userland IOCTL.  The AUTO
// setting is the default and means it will be set on certain
// commands.  Hard setting it on and off will override automatic
// operation.
//
pub const IPMI_MAINTENANCE_MODE_AUTO: c_int = 0;
pub const IPMI_MAINTENANCE_MODE_OFF: c_int = 1;
pub const IPMI_MAINTENANCE_MODE_ON: c_int = 2;
//
// The userland interface
//
// The userland interface for the IPMI driver is a standard character
// device, with each instance of an interface registered as a minor
// number under the major character device.
//
// The read and write calls do not work, to get messages in and out
// requires ioctl calls because of the complexity of the data.  select
// and poll do work, so you can wait for input using the file
// descriptor, you just can use read to get it.
//
// In general, you send a command down to the interface and receive
// responses back.  You can use the msgid value to correlate commands
// and responses, the driver will take care of figuring out which
// incoming messages are for which command and find the proper msgid
// value to report.  You will only receive reponses for commands you
// send.  Asynchronous events, however, go to all open users, so you
// must be ready to handle these (or ignore them if you don't care).
//
// The address type depends upon the channel type.  When talking
// directly to the BMC (IPMC_BMC_CHANNEL), the address is ignored
// (IPMI_UNUSED_ADDR_TYPE).  When talking to an IPMB channel, you must
// supply a valid IPMB address with the addr_type set properly.
//
// When talking to normal channels, the driver takes care of the
// details of formatting and sending messages on that channel.  You do
// not, for instance, have to format a send command, you just send
// whatever command you want to the channel, the driver will create
// the send command, automatically issue receive command and get even
// commands, and pass those up to the proper user.
//
// The magic IOCTL value for this interface.

// Messages sent to the interface are this format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_req {
    pub /: *mut *mut *mut unsigned char __user addr; / Address to send the message to.,
    pub addr_len: c_uint,
    pub This: *mut *mut long msgid; / The sequence number for the message.,
    pub msg: ipmi_msg,
}

//
// Send a message to the interfaces.  error values are:
// - EFAULT - an address supplied was invalid.
// - EINVAL - The address supplied was not valid, or the command
// was not allowed.
// - EMSGSIZE - The message to was too large.
// - ENOMEM - Buffers could not be allocated for the command.
//

// Messages sent to the interface with timing parameters are this
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_req_settime {
    pub req: ipmi_req,
// See ipmi_request_settime() above for details on these
    pub retries: c_int,
    pub retry_time_ms: c_uint,
}

//
// Send a message to the interfaces with timing parameters.  error values
// are:
// - EFAULT - an address supplied was invalid.
// - EINVAL - The address supplied was not valid, or the command
// was not allowed.
// - EMSGSIZE - The message to was too large.
// - ENOMEM - Buffers could not be allocated for the command.
//

// Messages received from the interface are this format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_recv {
    pub an: *mut *mut int recv_type; / Is this a command, response or,
    pub put: *mut *mut *mut unsigned char __user addr; / Address the message was from is,
    pub buffer.: *mut *mut unsigned int addr_len; / The size of the address,
    pub request: *mut *mut long msgid; / The sequence number specified in the,
    pub buffer.: *mut *mut ipmi_msg msg; / The data field must point to a,
}

//
// Receive a message.  error values:
// - EAGAIN - no messages in the queue.
// - EFAULT - an address supplied was invalid.
// - EINVAL - The address supplied was not valid.
// - EMSGSIZE - The message to was too large to fit into the message buffer,
// the message will be left in the buffer.

//
// Like RECEIVE_MSG, but if the message won't fit in the buffer, it
// will truncate the contents instead of leaving the data in the
// buffer.
//

// Register to get commands from other entities on this interface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_cmdspec {
    pub netfn: c_uchar,
    pub cmd: c_uchar,
}

//
// Register to receive a specific command.  error values:
// - EFAULT - an address supplied was invalid.
// - EBUSY - The netfn/cmd supplied was already in use.
// - ENOMEM - could not allocate memory for the entry.
//

//
// Unregister a registered command.  error values:
// - EFAULT - an address supplied was invalid.
// - ENOENT - The netfn/cmd was not found registered for this user.
//

//
// Register to get commands from other entities on specific channels.
// This way, you can only listen on specific channels, or have messages
// from some channels go to one place and other channels to someplace
// else.  The chans field is a bitmask, (1 << channel) for each channel.
// It may be IPMI_CHAN_ALL for all channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_cmdspec_chans {
    pub netfn: c_uint,
    pub cmd: c_uint,
    pub chans: c_uint,
}

//
// Register to receive a specific command on specific channels.  error values:
// - EFAULT - an address supplied was invalid.
// - EBUSY - One of the netfn/cmd/chans supplied was already in use.
// - ENOMEM - could not allocate memory for the entry.
//

//
// Unregister some netfn/cmd/chans.  error values:
// - EFAULT - an address supplied was invalid.
// - ENOENT - None of the netfn/cmd/chans were found registered for this user.
//

//
// Set whether this interface receives events.  Note that the first
// user registered for events will get all pending events for the
// interface.  error values:
// - EFAULT - an address supplied was invalid.
//

//
// Set and get the slave address and LUN that we will use for our
// source messages.  Note that this affects the interface, not just
// this user, so it will affect all users of this interface.  This is
// so some initialization code can come in and do the OEM-specific
// things it takes to determine your address (if not the BMC) and set
// it for everyone else.  You should probably leave the LUN alone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_channel_lun_address_set {
    pub channel: c_ushort,
    pub value: c_uchar,
}

// Legacy interfaces, these only set IPMB 0.

//
// Get/set the default timing values for an interface.  You shouldn't
// generally mess with these.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_timing_parms {
    pub retries: c_int,
    pub retry_time_ms: c_uint,
}

//
// Set the maintenance mode.  See ipmi_set_maintenance_mode() above
// for a description of what this does.
//

