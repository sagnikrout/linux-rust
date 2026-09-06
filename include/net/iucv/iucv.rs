//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/iucv/iucv.h
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
// drivers/s390/net/iucv.h
// IUCV base support.
//
// S390 version
// Copyright 2000, 2006 IBM Corporation
// Author(s):Alan Altmark (Alan_Altmark@us.ibm.com)
// Xenia Tkatschow (xenia@us.ibm.com)
// Rewritten for af_iucv:
// Martin Schwidefsky <schwidefsky@de.ibm.com>
//
// Functionality:
// To explore any of the IUCV functions, one must first register their
// program using iucv_register(). Once your program has successfully
// completed a register, it can exploit the other functions.
// For further reference on all IUCV functionality, refer to the
// CP Programming Services book, also available on the web thru
// www.vm.ibm.com/pubs, manual # SC24-6084
//
// Definition of Return Codes
// - All positive return codes including zero are reflected back
// from CP. The definition of each return code can be found in
// CP Programming Services book.
// - Return Code of:
// -EINVAL: Invalid value
// -ENOMEM: storage allocation failed
//

//
// IUCV option flags usable by device drivers:
//
// IUCV_IPRMDATA  Indicates that your program can handle a message in the
// parameter list / a message is sent in the parameter list.
// Used for iucv_path_accept, iucv_path_connect,
// iucv_message_reply, iucv_message_send, iucv_message_send2way.
// IUCV_IPQUSCE	  Indicates that you do not want to receive messages on this
// path until an iucv_path_resume is issued.
// Used for iucv_path_accept, iucv_path_connect.
// IUCV_IPBUFLST  Indicates that an address list is used for the message data.
// Used for iucv_message_receive, iucv_message_send,
// iucv_message_send2way.
// IUCV_IPPRTY	  Specifies that you want to send priority messages.
// Used for iucv_path_accept, iucv_path_connect,
// iucv_message_reply, iucv_message_send, iucv_message_send2way.
// IUCV_IPSYNC	  Indicates a synchronous send request.
// Used for iucv_message_send, iucv_message_send2way.
// IUCV_IPANSLST  Indicates that an address list is used for the reply data.
// Used for iucv_message_reply, iucv_message_send2way.
// IUCV_IPLOCAL	  Specifies that the communication partner has to be on the
// local system. If local is specified no target class can be
// specified.
// Used for iucv_path_connect.
//
// All flags are defined in the input field IPFLAGS1 of each function
// and can be found in CP Programming Services.
//
pub const IUCV_IPRMDATA: c_uint = 0x80;
pub const IUCV_IPQUSCE: c_uint = 0x40;
pub const IUCV_IPBUFLST: c_uint = 0x40;
pub const IUCV_IPPRTY: c_uint = 0x20;
pub const IUCV_IPANSLST: c_uint = 0x08;
pub const IUCV_IPSYNC: c_uint = 0x04;
pub const IUCV_IPLOCAL: c_uint = 0x01;
//
// iucv_array : Defines buffer array.
// Inside the array may be 31- bit addresses and 31-bit lengths.
// Use a pointer to an iucv_array as the buffer, reply or answer
// parameter on iucv_message_send, iucv_message_send2way, iucv_message_receive
// and iucv_message_reply if IUCV_IPBUFLST or IUCV_IPANSLST are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_array {
    pub address: dma32_t,
    pub length: u32,
// C attribute field omitted
    pub iucv_bus: extern struct bus_type,
    pub device_driver: struct,
    pub 5): *const *const char fmt, ...) __printf(4,,
//
// struct iucv_path
// pathid: 16 bit path identification
// msglim: 16 bit message limit
// flags: properties of the path: IPRMDATA, IPQUSCE, IPPRTY
// handler:  address of iucv handler structure
// private: private information of the handler associated with the path
// list: list_head for the iucv_handler path list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_path {
    pub pathid: u16,
    pub msglim: u16,
    pub flags: u8,
    pub private: *mut c_void,
    pub handler: *mut iucv_handler,
    pub list: list_head,
}

//
// struct iucv_message
// id: 32 bit message id
// audit: 32 bit error information of purged or replied messages
// class: 32 bit target class of a message (source class for replies)
// tag: 32 bit tag to be associated with the message
// length: 32 bit length of the message / reply
// reply_size: 32 bit maximum allowed length of the reply
// rmmsg: 8 byte inline message
// flags: message properties (IUCV_IPPRTY)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_message {
    pub id: u32,
    pub audit: u32,
    pub class: u32,
    pub tag: u32,
    pub length: u32,
    pub reply_size: u32,
    pub rmmsg: [u8; 8],
    pub flags: u8,
    pub __packed: },
//
// struct iucv_handler
//
// A vector of functions that handle IUCV interrupts. Each functions gets
// a parameter area as defined by the CP Programming Services and private
// pointer that is provided by the user of the interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_handler {
//
// The path_pending function is called after an iucv interrupt
// type 0x01 has been received. The base code allocates a path
// structure and "asks" the handler if this path belongs to the
// handler. To accept the path the path_pending function needs
// to call iucv_path_accept and return 0. If the callback returns
// a value != 0 the iucv base code will continue with the next
// handler. The order in which the path_pending functions are
// called is the order of the registration of the iucv handlers
// to the base code.
//
    pub ipuser): *mut *mut *mut *mut int (path_pending)(struct iucv_path , u8 ipvmid, u8,
//
// The path_complete function is called after an iucv interrupt
// type 0x02 has been received for a path that has been established
// for this handler with iucv_path_connect and got accepted by the
// peer with iucv_path_accept.
//
    pub ipuser): *mut *mut *mut void (path_complete)(struct iucv_path , u8,
//
// The path_severed function is called after an iucv interrupt
// type 0x03 has been received. The communication peer shutdown
// his end of the communication path. The path still exists and
// remaining messages can be received until a iucv_path_sever
// shuts down the other end of the path as well.
//
    pub ipuser): *mut *mut *mut void (path_severed)(struct iucv_path , u8,
//
// The path_quiesced function is called after an icuv interrupt
// type 0x04 has been received. The communication peer has quiesced
// the path. Delivery of messages is stopped until iucv_path_resume
// has been called.
//
    pub ipuser): *mut *mut *mut void (path_quiesced)(struct iucv_path , u8,
//
// The path_resumed function is called after an icuv interrupt
// type 0x05 has been received. The communication peer has resumed
// the path.
//
    pub ipuser): *mut *mut *mut void (path_resumed)(struct iucv_path , u8,
//
// The message_pending function is called after an icuv interrupt
// type 0x06 or type 0x07 has been received. A new message is
// available and can be received with iucv_message_receive.
//
    pub ): *mut *mut *mut void (message_pending)(struct iucv_path , struct iucv_message,
//
// The message_complete function is called after an icuv interrupt
// type 0x08 or type 0x09 has been received. A message send with
// iucv_message_send2way has been replied to. The reply can be
// received with iucv_message_receive.
//
    pub ): *mut *mut *mut void (message_complete)(struct iucv_path , struct iucv_message,
    pub list: list_head,
    pub paths: list_head,
}

extern "C" {
    pub fn iucv_register(handler: *mut iucv_handler, smp: c_int) -> c_int;
}
extern "C" {
    pub fn iucv_unregister(handler: *mut iucv_handler, smp: c_int);
}
//
// iucv_path_alloc - Allocate a new path structure for use with iucv_connect.
// @msglim: initial message limit
// @flags: initial flags
// @gfp: kmalloc allocation flag
//
// Returns: NULL if the memory allocation failed or a pointer to the
// path structure.
//
// iucv_path_free - Frees a path structure.
// @path: address of iucv path structure
//
extern "C" {
    pub fn iucv_path_quiesce(path: *mut iucv_path, userdata: *mut u8) -> c_int;
}
extern "C" {
    pub fn iucv_path_resume(path: *mut iucv_path, userdata: *mut u8) -> c_int;
}
extern "C" {
    pub fn iucv_path_sever(path: *mut iucv_path, userdata: *mut u8) -> c_int;
}
extern "C" {
    pub fn iucv_message_reject(path: *mut iucv_path, msg: *mut iucv_message) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iucv_interface {
    pub residual): *mut *mut u8 flags, void buffer, size_t size, size_t,
    pub residual): *mut usize,
    pub size): *mut *mut u8 flags, void reply, size_t,
    pub msg): *mut *mut *mut int (message_reject)(struct iucv_path path, struct iucv_message,
    pub size): *mut *mut u8 flags, u32 srccls, void buffer, size_t,
    pub size): *mut *mut u8 flags, u32 srccls, void buffer, size_t,
    pub residual): *mut *mut size_t size, void answer, size_t asize, size_t,
    pub srccls): u32,
    pub private): *mut u8 userdata[16], void,
    pub private): *mut u8 userid[8], u8 system[8], u8 userdata[16], void,
    pub userdata[16]): *mut *mut *mut int (path_quiesce)(struct iucv_path path, u8,
    pub userdata[16]): *mut *mut *mut int (path_resume)(struct iucv_path path, u8,
    pub userdata[16]): *mut *mut *mut int (path_sever)(struct iucv_path path, u8,
    pub smp): *mut *mut *mut int (iucv_register)(struct iucv_handler handler, int,
    pub smp): *mut *mut *mut void (iucv_unregister)(struct iucv_handler handler, int,
    pub bus: *const bus_type,
    pub root: *mut device,
}
