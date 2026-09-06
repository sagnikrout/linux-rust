//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvme/host/fabrics.h
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
// NVMe over Fabrics common host code.
// Copyright (c) 2015-2016 HGST, a Western Digital Company.
//
pub const _NVME_FABRICS_H: c_int = 1;

pub const NVMF_MIN_QUEUE_SIZE: c_int = 16;
pub const NVMF_MAX_QUEUE_SIZE: c_int = 1024;
pub const NVMF_DEF_QUEUE_SIZE: c_int = 128;
pub const NVMF_DEF_RECONNECT_DELAY: c_int = 10;
// default to 600 seconds of reconnect attempts before giving up
pub const NVMF_DEF_CTRL_LOSS_TMO: c_int = 600;
// default is -1: the fail fast mechanism is disabled

//
// Define a host as seen by the target.  We allocate one at boot, but also
// allow the override it when creating controllers.  This is both to provide
// persistence of the Host NQN over multiple boots, and to allow using
// multiple ones, for example in a container scenario.  Because we must not
// use different Host NQNs with the same Host ID we generate a Host ID and
// use this structure to keep track of the relation between the two.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_host {
    pub ref: kref,
    pub list: list_head,
    pub nqn: [c_char; NVMF_NQN_SIZE],
    pub id: uuid_t,
}

//
// enum nvmf_parsing_opts - used to define the sysfs parsing options used.
//
// struct nvmf_ctrl_options - Used to hold the options specified
// with the parsing opts enum.
// @mask:	Used by the fabrics library to parse through sysfs options
// on adding a NVMe controller.
// @max_reconnects: maximum number of allowed reconnect attempts before removing
// the controller, (-1) means reconnect forever, zero means remove
// immediately;
// @transport:	Holds the fabric transport "technology name" (for a lack of
// better description) that will be used by an NVMe controller
// being added.
// @subsysnqn:	Hold the fully qualified NQN subsystem name (format defined
// in the NVMe specification, "NVMe Qualified Names").
// @traddr:	The transport-specific TRADDR field for a port on the
// subsystem which is adding a controller.
// @trsvcid:	The transport-specific TRSVCID field for a port on the
// subsystem which is adding a controller.
// @host_traddr: A transport-specific field identifying the NVME host port
// to use for the connection to the controller.
// @host_iface: A transport-specific field identifying the NVME host
// interface to use for the connection to the controller.
// @queue_size: Number of IO queue elements.
// @nr_io_queues: Number of controller IO queues that will be established.
// @reconnect_delay: Time between two consecutive reconnect attempts.
// @discovery_nqn: indicates if the subsysnqn is the well-known discovery NQN.
// @kato:	Keep-alive timeout.
// @host:	Virtual NVMe host, contains the NQN and Host ID.
// @dhchap_secret: DH-HMAC-CHAP secret
// @dhchap_ctrl_secret: DH-HMAC-CHAP controller secret for bi-directional
// authentication
// @keyring:    Keyring to use for key lookups
// @tls_key:    TLS key for encrypted connections (TCP)
// @tls:        Start TLS encrypted connections (TCP)
// @concat:     Enabled Secure channel concatenation (TCP)
// @disable_sqflow: disable controller sq flow control
// @hdr_digest: generate/verify header digest (TCP)
// @data_digest: generate/verify data digest (TCP)
// @nr_write_queues: number of queues for write I/O
// @nr_poll_queues: number of queues for polling I/O
// @tos: type of service
// @fast_io_fail_tmo: Fast I/O fail timeout in seconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_ctrl_options {
    pub mask: unsigned,
    pub max_reconnects: c_int,
    pub transport: *mut c_char,
    pub subsysnqn: *mut c_char,
    pub traddr: *mut c_char,
    pub trsvcid: *mut c_char,
    pub host_traddr: *mut c_char,
    pub host_iface: *mut c_char,
    pub queue_size: usize,
    pub nr_io_queues: c_uint,
    pub reconnect_delay: c_uint,
    pub discovery_nqn: bool,
    pub duplicate_connect: bool,
    pub kato: c_uint,
    pub host: *mut nvmf_host,
    pub dhchap_secret: *mut c_char,
    pub dhchap_ctrl_secret: *mut c_char,
    pub keyring: *mut key,
    pub tls_key: *mut key,
    pub tls: bool,
    pub concat: bool,
    pub disable_sqflow: bool,
    pub hdr_digest: bool,
    pub data_digest: bool,
    pub nr_write_queues: c_uint,
    pub nr_poll_queues: c_uint,
    pub tos: c_int,
    pub fast_io_fail_tmo: c_int,
}

//
// struct nvmf_transport_ops - used to register a specific
// fabric implementation of NVMe fabrics.
// @entry:		Used by the fabrics library to add the new
// registration entry to its linked-list internal tree.
// @module:             Transport module reference
// @name:		Name of the NVMe fabric driver implementation.
// @required_opts:	sysfs command-line options that must be specified
// when adding a new NVMe controller.
// @allowed_opts:	sysfs command-line options that can be specified
// when adding a new NVMe controller.
// @create_ctrl():	function pointer that points to a non-NVMe
// implementation-specific fabric technology
// that would go into starting up that fabric
// for the purpose of connection to an NVMe controller
// using that fabric technology.
//
// Notes:
// 1. At minimum, 'required_opts' and 'allowed_opts' should
// be set to the same enum parsing options defined earlier.
// 2. create_ctrl() must be defined (even if it does nothing)
// 3. struct nvmf_transport_ops must be statically allocated in the
// modules .bss section so that a pure module_get on @module
// prevents the memory from being freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmf_transport_ops {
    pub entry: list_head,
    pub module: *mut module,
    pub name: *const c_char,
    pub required_opts: c_int,
    pub allowed_opts: c_int,
    pub opts): *mut nvmf_ctrl_options,
}

extern "C" {
    pub fn nvmf_reg_read32(ctrl: *mut nvme_ctrl, off: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn nvmf_reg_read64(ctrl: *mut nvme_ctrl, off: u32, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn nvmf_reg_write32(ctrl: *mut nvme_ctrl, off: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn nvmf_subsystem_reset(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvmf_connect_admin_queue(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvmf_connect_io_queue(ctrl: *mut nvme_ctrl, qid: u16) -> c_int;
}
extern "C" {
    pub fn nvmf_register_transport(ops: *mut nvmf_transport_ops) -> c_int;
}
extern "C" {
    pub fn nvmf_unregister_transport(ops: *mut nvmf_transport_ops);
}
extern "C" {
    pub fn nvmf_free_options(opts: *mut nvmf_ctrl_options);
}
extern "C" {
    pub fn nvmf_get_address(ctrl: *mut nvme_ctrl, buf: *mut c_char, size: c_int) -> c_int;
}
extern "C" {
    pub fn nvmf_should_reconnect(ctrl: *mut nvme_ctrl, status: c_int) -> bool;
}
