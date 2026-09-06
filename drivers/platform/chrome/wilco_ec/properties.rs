//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/properties.c
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
// Copyright 2019 Google LLC
//

// Operation code; what the EC should do with the property
    enum ec_property_op {
    EC_OP_GET = 0,
    EC_OP_SET = 1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_property_request {
    pub /: *mut *mut u8 op; / One of enum ec_property_op,
    pub /: *mut *mut u8 property_id[4]; / The 32 bit PID is stored Little Endian,
    pub length: u8,
    pub data: [u8; WILCO_EC_PROPERTY_MAX_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_property_response {
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 op; / One of enum ec_property_op,
    pub /: *mut *mut u8 property_id[4]; / The 32 bit PID is stored Little Endian,
    pub length: u8,
    pub data: [u8; WILCO_EC_PROPERTY_MAX_SIZE],
    pub __packed: },
    static int send_property_msg(struct wilco_ec_device *ec,
    struct ec_property_request *rq,
    struct ec_property_response *rs)
    {
    pub ec_msg: wilco_ec_message,
    pub ret: c_int,
    pub sizeof(ec_msg)): memset(&ec_msg, 0,,
    pub WILCO_EC_MSG_PROPERTY: ec_msg.type =,
    pub rq: ec_msg.request_data =,
    pub sizeof(*rq): *mut ec_msg.request_size =,
    pub rs: ec_msg.response_data =,
    pub sizeof(*rs): *mut ec_msg.response_size =,
    pub &ec_msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0)
    pub ret: return,
    if (rs.op != rq.op)
    pub -EBADMSG: return,
    if (memcmp(rq.property_id, rs.property_id, sizeof(rs.property_id)))
    pub -EBADMSG: return,
    pub 0: return,
    }
    int wilco_ec_get_property(struct wilco_ec_device *ec,
    struct wilco_ec_property_msg *prop_msg)
    {
    pub rq: ec_property_request,
    pub rs: ec_property_response,
    pub ret: c_int,
    pub sizeof(rq)): memset(&rq, 0,,
    pub EC_OP_GET: rq.op =,
    pub rq.property_id): put_unaligned_le32(prop_msg->property_id,,
    pub &rs): ret = send_property_msg(ec, &rq,,
    if (ret < 0)
    pub ret: return,
    pub rs.length: prop_msg->length =,
    pub rs.length): memcpy(prop_msg->data, rs.data,,
    pub 0: return,
    }
    int wilco_ec_set_property(struct wilco_ec_device *ec,
    struct wilco_ec_property_msg *prop_msg)
    {
    pub rq: ec_property_request,
    pub rs: ec_property_response,
    pub ret: c_int,
    pub sizeof(rq)): memset(&rq, 0,,
    pub EC_OP_SET: rq.op =,
    pub rq.property_id): put_unaligned_le32(prop_msg->property_id,,
    pub prop_msg->length: rq.length =,
    pub prop_msg->length): memcpy(rq.data, prop_msg->data,,
    pub &rs): ret = send_property_msg(ec, &rq,,
    if (ret < 0)
    pub ret: return,
    if (rs.length != prop_msg.length)
    pub -EBADMSG: return,
    pub 0: return,
    }
    int wilco_ec_get_byte_property(struct wilco_ec_device *ec, u32 property_id,
    u8 *val)
    {
    pub msg: wilco_ec_property_msg,
    pub ret: c_int,
    pub property_id: msg.property_id =,
    pub &msg): ret = wilco_ec_get_property(ec,,
    if (ret < 0)
    pub ret: return,
    if (msg.length != 1)
    pub -EBADMSG: return,
// val = msg.data[0];
    pub 0: return,
    }
    int wilco_ec_set_byte_property(struct wilco_ec_device *ec, u32 property_id,
    u8 val)
    {
    pub msg: wilco_ec_property_msg,
    pub property_id: msg.property_id =,
    pub val: msg.data[0] =,
    pub 1: msg.length =,
    pub &msg): return wilco_ec_set_property(ec,,
    }
