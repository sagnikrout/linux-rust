//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/apr.h
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
// HEADER field
// version:0:3
// header_size : 4:7
// message_type : 8:9
// reserved: 10:15
//

// Version
pub const APR_PKT_VER: c_uint = 0x0;
// Command and Response Types
pub const APR_MSG_TYPE_EVENT: c_uint = 0x0;
pub const APR_MSG_TYPE_CMD_RSP: c_uint = 0x1;
pub const APR_MSG_TYPE_SEQ_CMD: c_uint = 0x2;
pub const APR_MSG_TYPE_NSEQ_CMD: c_uint = 0x3;
pub const APR_MSG_TYPE_MAX: c_uint = 0x04;
// APR Basic Response Message
pub const APR_BASIC_RSP_RESULT: c_uint = 0x000110E8;
pub const APR_RSP_ACCEPTED: c_uint = 0x000100BE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aprv2_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

// hdr field Ver [0:3], Size [4:7], Message type [8:10]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_hdr {
    pub hdr_field: u16,
    pub pkt_size: u16,
    pub src_svc: u8,
    pub src_domain: u8,
    pub src_port: u16,
    pub dest_svc: u8,
    pub dest_domain: u8,
    pub dest_port: u16,
    pub token: u32,
    pub opcode: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_pkt {
    pub hdr: apr_hdr,
    pub payload: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_resp_pkt {
    pub hdr: apr_hdr,
    pub payload: *mut c_void,
    pub payload_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpr_hdr {
    pub version:4: u32,
    pub hdr_size:4: u32,
    pub pkt_size:24: u32,
    pub dest_domain:8: u32,
    pub src_domain:8: u32,
    pub reserved:16: u32,
    pub src_port: u32,
    pub dest_port: u32,
    pub token: u32,
    pub opcode: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpr_pkt {
    pub hdr: gpr_hdr,
    pub payload: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpr_resp_pkt {
    pub hdr: gpr_hdr,
    pub payload: *mut c_void,
    pub payload_size: c_int,
}

pub const GPR_PKT_VER: c_uint = 0x0;

pub const GPR_BASIC_RSP_RESULT: c_uint = 0x02001005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpr_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

pub const GPR_BASIC_EVT_ACCEPTED: c_uint = 0x02001006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpr_ibasic_rsp_accepted_t {
    pub opcode: u32,
}

// Bits 0 to 15 -- Minor version,  Bits 16 to 31 -- Major version

extern "C" {
    pub fn int(d: *const *const gpr_port_cb) (struct gpr_resp_pkt, priv: *mut c_void, op: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_router_svc {
    pub dev: *mut device,
    pub callback: gpr_port_cb,
    pub pr: *mut packet_router,
    pub lock: spinlock_t,
    pub id: c_int,
    pub priv: *mut c_void,
}

pub type gpr_port_t = pkt_router_svc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_device {
    pub dev: device,
    pub svc_id: u16,
    pub domain_id: u16,
    pub version: u32,
    pub name: [c_char; APR_NAME_SIZE],
    pub service_path: *const c_char,
    pub svc: pkt_router_svc,
    pub node: list_head,
}

pub type gpr_device_t = apr_device;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apr_driver {
    pub sl): *mut *mut int (probe)(struct apr_device,
    pub sl): *mut *mut void (remove)(struct apr_device,
    pub d): *const apr_resp_pkt,
    pub gpr_callback: gpr_port_cb,
    pub driver: device_driver,
    pub id_table: *const apr_device_id,
}

pub type gpr_driver_t = apr_driver;

//
// use a macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn __apr_driver_register(drv: *mut apr_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn apr_driver_unregister(drv: *mut apr_driver);
}
//
// module_apr_driver() - Helper macro for registering a aprbus driver
// @__apr_driver: apr_driver struct
//
// Helper macro for aprbus drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module
// may only use this macro once, and calling it replaces module_init()
// and module_exit()
//

extern "C" {
    pub fn apr_send_pkt(adev: *mut apr_device, pkt: *mut apr_pkt) -> c_int;
}
extern "C" {
    pub fn gpr_free_port(port: *mut gpr_port_t);
}
extern "C" {
    pub fn gpr_send_port_pkt(port: *mut gpr_port_t, pkt: *const gpr_pkt) -> c_int;
}
extern "C" {
    pub fn gpr_send_pkt(gdev: *mut gpr_device_t, pkt: *const gpr_pkt) -> c_int;
}
