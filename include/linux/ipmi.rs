//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ipmi.h
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
// Opaque type for a IPMI message user.  One of these is needed to
// send and receive messages.
//
// Stuff coming from the receive interface comes as one of these.
// They are allocated, the receiver must free them with
// ipmi_free_recv_msg() when done with the message.  The link is not
// used after the message is delivered, so the upper layer may use the
// link to build a linked list, if it likes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_recv_msg {
    pub link: list_head,
//
// The type of message as defined in the "Receive Types"
// defines above.
//
    pub recv_type: c_int,
    pub user: *mut ipmi_user,
    pub addr: ipmi_addr,
    pub msgid: c_long,
    pub msg: kernel_ipmi_msg,
//
// The user_msg_data is the data supplied when a message was
// sent, if this is a response to a sent message.  If this is
// not a response to a sent message, then user_msg_data will
// be NULL.  If the user above is NULL, then this will be the
// intf.
//
    pub user_msg_data: *mut c_void,
//
// Call this when done with the message.  It will presumably free
// the message and do any other necessary cleanup.
//
    pub msg): *mut *mut void (done)(struct ipmi_recv_msg,
//
// Place-holder for the data, don't make any assumptions about
// the size or existence of this, since it may change.
//
    pub msg_data: [c_uchar; IPMI_MAX_MSG_LENGTH],
}

// Allocate and free the receive message.
extern "C" {
    pub fn ipmi_free_recv_msg(msg: *mut ipmi_recv_msg);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_user_hndl {
//
// Routine type to call when a message needs to be routed to
// the upper layer.  This will be called with some locks held,
// the only IPMI routines that can be called are ipmi_request
// and the alloc/free operations.  The handler_data is the
// variable supplied when the receive handler was registered.
//
    pub user_msg_data): *mut c_void,
//
// Called when the interface detects a watchdog pre-timeout.  If
// this is NULL, it will be ignored for the user.  Note that you
// can't do any IPMI calls from here, it's called with locks held.
//
    pub handler_data): *mut *mut void (ipmi_watchdog_pretimeout)(void,
//
// If not NULL, called at panic time after the interface has
// been set up to handle run to completion.
//
    pub handler_data): *mut *mut void (ipmi_panic_handler)(void,
//
// Called when the interface has been removed.  After this returns
// the user handle will be invalid.  The interface may or may
// not be usable when this is called, but it will return errors
// if it is not usable.
//
    pub handler_data): *mut *mut void (shutdown)(void,
}

// Create a new user of the IPMI layer on the given interface number.
//
// Destroy the given user of the IPMI layer.  Note that after this
// function returns, the system is guaranteed to not call any
// callbacks for the user.  Thus as long as you destroy all the users
// before you unload a module, you will be safe.  And if you destroy
// the users before you destroy the callback structures, it should be
// safe, too.
//
extern "C" {
    pub fn ipmi_destroy_user(user: *mut ipmi_user);
}
// Get the IPMI version of the BMC we are talking to.
//
// Set and get the slave address and LUN that we will use for our
// source messages.  Note that this affects the interface, not just
// this user, so it will affect all users of this interface.  This is
// so some initialization code can come in and do the OEM-specific
// things it takes to determine your address (if not the BMC) and set
// it for everyone else.  Note that each channel can have its own
// address.
//
// Like ipmi_request, but lets you specify the number of retries and
// the retry time.  The retries is the number of times the message
// will be resent if no reply is received.  If set to -1, the default
// value will be used.  The retry time is the time in milliseconds
// between retries.  If set to zero, the default value will be
// used.
//
// Don't use this unless you *really* have to.  It's primarily for the
// IPMI over LAN converter; since the LAN stuff does its own retries,
// it makes no sense to do it here.  However, this can be used if you
// have unusual requirements.
//
// Like ipmi_request, but with messages supplied.  This will not
// allocate any memory, and the messages may be statically allocated
// (just make sure to do the "done" handling on them).  Note that this
// is primarily for the watchdog timer, since it should be able to
// send messages even if no memory is available.  This is subject to
// change as the system changes, so don't use it unless you REALLY
// have to.
//
// Poll the IPMI interface for the user.  This causes the IPMI code to
// do an immediate check for information from the driver and handle
// anything that is immediately pending.  This will not block in any
// way.  This is useful if you need to spin waiting for something to
// happen in the IPMI driver.
//
extern "C" {
    pub fn ipmi_poll_interface(user: *mut ipmi_user);
}
//
// When commands come in to the SMS, the user can register to receive
// them.  Only one user can be listening on a specific netfn/cmd/chan tuple
// at a time, you will get an EBUSY error if the command is already
// registered.  If a command is received that does not have a user
// registered, the driver will automatically return the proper
// error.  Channels are specified as a bitfield, use IPMI_CHAN_ALL to
// mean all channels.
//
// Go into a mode where the driver will not autonomously attempt to do
// things with the interface.  It will still respond to attentions and
// interrupts, and it will expect that commands will complete.  It
// will not automatcially check for flags, events, or things of that
// nature.
//
// This is primarily used for firmware upgrades.  The idea is that
// when you go into firmware upgrade mode, you do this operation
// and the driver will not attempt to do anything but what you tell
// it or what the BMC asks for.
//
// Note that if you send a command that resets the BMC, the driver
// will still expect a response from that command.  So the BMC should
// reset itself *after* the response is sent.  Resetting before the
// response is just silly.
//
// If in auto maintenance mode, the driver will automatically go into
// maintenance mode for 30 seconds if it sees a cold reset, a warm
// reset, or a firmware NetFN.  This means that code that uses only
// firmware NetFN commands to do upgrades will work automatically
// without change, assuming it sends a message every 30 seconds or
// less.
//
// See the IPMI_MAINTENANCE_MODE_xxx defines for what the mode means.
//
extern "C" {
    pub fn ipmi_get_maintenance_mode(user: *mut ipmi_user) -> c_int;
}
extern "C" {
    pub fn ipmi_set_maintenance_mode(user: *mut ipmi_user, mode: c_int) -> c_int;
}
//
// When the user is created, it will not receive IPMI events by
// default.  The user must set this to TRUE to get incoming events.
// The first user that sets this to TRUE will receive all events that
// have been queued while no one was waiting for events.
//
extern "C" {
    pub fn ipmi_set_gets_events(user: *mut ipmi_user, val: bool) -> c_int;
}
//
// Called when a new SMI is registered.  This will also be called on
// every existing interface when a new watcher is registered with
// ipmi_smi_watcher_register().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_smi_watcher {
    pub link: list_head,
//
// You must set the owner to the current module, if you are in
// a module (generally just set it to "THIS_MODULE").
//
    pub owner: *mut module,
//
// These two are called with read locks held for the interface
// the watcher list.  So you can add and remove users from the
// IPMI interface, send messages, etc., but you cannot add
// or remove SMI watchers or SMI interfaces.
//
    pub dev): *mut *mut void (new_smi)(int if_num, struct device,
    pub if_num): *mut *mut void (smi_gone)(int,
}

extern "C" {
    pub fn ipmi_smi_watcher_register(watcher: *mut ipmi_smi_watcher) -> c_int;
}
extern "C" {
    pub fn ipmi_smi_watcher_unregister(watcher: *mut ipmi_smi_watcher) -> c_int;
}
//
// The following are various helper functions for dealing with IPMI
// addresses.
//
// Return the maximum length of an IPMI address given it's type.
extern "C" {
    pub fn ipmi_addr_length(addr_type: c_int) -> c_uint;
}
// Validate that the given IPMI address is valid.
extern "C" {
    pub fn ipmi_validate_addr(addr: *mut ipmi_addr, len: c_int) -> c_int;
}
//
// How did the IPMI driver find out about the device?
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipmi_addr_src {
    SI_INVALID = 0, SI_HOTMOD, SI_HARDCODED, SI_SPMI, SI_ACPI, SI_SMBIOS,
    SI_PCI,	SI_DEVICETREE, SI_PLATFORM, SI_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ipmi_smi_info_union {

//
// the acpi_info element is defined for the SI_ACPI
// address type
//
    pub acpi_handle: acpi_handle,
    pub acpi_info: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_smi_info {
    pub addr_src: ipmi_addr_src,
//
// Base device for the interface.  Don't forget to put this when
// you are done.
//
    pub dev: *mut device,
//
// The addr_info provides more detailed info for some IPMI
// devices, depending on the addr_src.  Currently only SI_ACPI
// info is provided.
//
    pub addr_info: ipmi_smi_info_union,
}

// This is to get the private info of struct ipmi_smi
extern "C" {
    pub fn ipmi_get_smi_info(if_num: c_int, data: *mut ipmi_smi_info) -> c_int;
}
pub const GET_DEVICE_ID_MAX_RETRY: c_int = 5;
// Helper function for computing the IPMB checksum of some data.
extern "C" {
    pub fn ipmb_checksum(data: *mut c_uchar, size: c_int) -> c_uchar;
}
//
// For things that must send messages at panic time, like the IPMI watchdog
// driver that extends the reset time on a panic, use this to send messages
// from panic context.  Note that this puts the driver into a mode that
// only works at panic time, so only use it then.
//
