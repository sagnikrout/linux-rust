//! Automatically rewritten from C to Rust
//! Source: drivers/tee/qcomtee/async.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const QCOMTEE_ASYNC_VERSION_1_0: c_uint = 0x00010000U /* Maj: 0x0001, Min: 0x0000. */;
pub const QCOMTEE_ASYNC_VERSION_1_1: c_uint = 0x00010001U /* Maj: 0x0001, Min: 0x0001. */;
pub const QCOMTEE_ASYNC_VERSION_1_2: c_uint = 0x00010002U /* Maj: 0x0001, Min: 0x0002. */;

    QCOMTEE_ASYNC_VERSION_MAJOR(QCOMTEE_ASYNC_VERSION_CURRENT)

    QCOMTEE_ASYNC_VERSION_MINOR(QCOMTEE_ASYNC_VERSION_CURRENT)
//
// struct qcomtee_async_msg_hdr - Asynchronous message header format.
// @version: current async protocol version of the remote endpoint.
// @op: async operation.
//
// @version specifies the endpoint's (QTEE or driver) supported async protocol.
// For example, if QTEE sets @version to %QCOMTEE_ASYNC_VERSION_1_1, QTEE
// handles operations supported in %QCOMTEE_ASYNC_VERSION_1_1 or
// %QCOMTEE_ASYNC_VERSION_1_0. @op determines the message format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_async_msg_hdr {
    pub version: u32,
    pub op: u32,
}

// Size of an empty async message.

//
// struct qcomtee_async_release_msg - Release asynchronous message.
// @hdr: message header as &struct qcomtee_async_msg_hdr.
// @counts: number of objects in @object_ids.
// @object_ids: array of object IDs that should be released.
//
// Available in Maj = 0x0001, Min >= 0x0000.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_async_release_msg {
    pub hdr: qcomtee_async_msg_hdr,
    pub counts: u32,
    pub __counted_by(counts): u32 object_ids[],
}

//
// qcomtee_get_async_buffer() - Get the start of the asynchronous message.
// @oic: context used for the current invocation.
// @async_buffer: return buffer to extract from or fill in async messages.
//
// If @oic is used for direct object invocation, the whole outbound buffer
// is available for the async message. If @oic is used for a callback request,
// the tail of the outbound buffer (after the callback request message) is
// available for the async message.
//
// The start of the async buffer is aligned, see qcomtee_msg_offset_align().
//
    static void qcomtee_get_async_buffer(struct qcomtee_object_invoke_ctx *oic,
    struct qcomtee_buffer *async_buffer)
    {
    struct qcomtee_msg_callback *msg;
    unsigned int offset;
    int i;
    if (!(oic.flags & QCOMTEE_OIC_FLAG_BUSY)) {
// The outbound buffer is empty. Using the whole buffer.
    offset = 0;
    } else {
    msg = (struct qcomtee_msg_callback *)oic.out_msg.addr;
// Start offset in a message for buffer arguments.
    offset = qcomtee_msg_buffer_args(struct qcomtee_msg_callback,
    qcomtee_msg_args(msg));
// Add size of IB arguments.
    qcomtee_msg_for_each_input_buffer(i, msg)
    offset += qcomtee_msg_offset_align(msg.args[i].b.size);
// Add size of OB arguments.
    qcomtee_msg_for_each_output_buffer(i, msg)
    offset += qcomtee_msg_offset_align(msg.args[i].b.size);
    }
    async_buffer.addr = oic.out_msg.addr + offset;
    async_buffer.size = oic.out_msg.size - offset;
    }
//
// async_release() - Process QTEE async release requests.
// @oic: context used for the current invocation.
// @msg: async message for object release.
// @size: size of the async buffer available.
//
// Return: Size of the outbound buffer used when processing @msg.
//
    static size_t async_release(struct qcomtee_object_invoke_ctx *oic,
    struct qcomtee_async_msg_hdr *async_msg,
    size_t size)
    {
    struct qcomtee_async_release_msg *msg;
    struct qcomtee_object *object;
    int i;
    msg = (struct qcomtee_async_release_msg *)async_msg;
    for (i = 0; i < msg.counts; i++) {
    object = qcomtee_idx_erase(oic, msg.object_ids[i]);
    qcomtee_object_put(object);
    }
    return struct_size(msg, object_ids, msg.counts);
    }
//
// qcomtee_fetch_async_reqs() - Fetch and process asynchronous messages.
// @oic: context used for the current invocation.
//
// Calls handlers to process the requested operations in the async message.
// Currently, only supports async release requests.
//
#[no_mangle]
pub unsafe extern "C" fn qcomtee_fetch_async_reqs(oic: *mut qcomtee_object_invoke_ctx) {
    void qcomtee_fetch_async_reqs(struct qcomtee_object_invoke_ctx *oic)
    {
    struct qcomtee_async_msg_hdr *async_msg;
    struct qcomtee_buffer async_buffer;
    size_t consumed, used = 0;
    u16 major_ver;
    qcomtee_get_async_buffer(oic, &async_buffer);
    while (async_buffer.size - used > QCOMTEE_ASYNC_MSG_ZERO) {
    async_msg = (struct qcomtee_async_msg_hdr *)(async_buffer.addr +
    used);
//
// QTEE assumes that the unused space of the async buffer is
// zeroed; so if version is zero, the buffer is unused.
//
    if (async_msg.version == 0)
    goto out;
    major_ver = QCOMTEE_ASYNC_VERSION_MAJOR(async_msg.version);
// Major version mismatch is a compatibility break.
    if (major_ver != QCOMTEE_ASYNC_VERSION_CURRENT_MAJOR) {
    pr_err("Async message version mismatch (%u != %u)\n",
    major_ver, QCOMTEE_ASYNC_VERSION_CURRENT_MAJOR);
    goto out;
    }
    switch (async_msg.op) {
    case QCOMTEE_MSG_OBJECT_OP_RELEASE:
    consumed = async_release(oic, async_msg,
    async_buffer.size - used);
    break;
    default:
    pr_err("Unsupported async message %u\n", async_msg.op);
    goto out;
    }
// Supported operation but unable to parse the message.
    if (!consumed) {
    pr_err("Unable to parse async message for op %u\n",
    async_msg.op);
    goto out;
    }
// Next async message.
    used += qcomtee_msg_offset_align(consumed);
    }
    out:
// Reset the async buffer so async requests do not loop to QTEE.
    memzero_explicit(async_buffer.addr, async_buffer.size);
    }
