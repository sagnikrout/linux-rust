//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/mailbox.c
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
// Mailbox interface for Wilco Embedded Controller
//
// Copyright 2018 Google LLC
//
// The Wilco EC is similar to a typical ChromeOS embedded controller.
// It uses the same MEC based low-level communication and a similar
// protocol, but with some important differences.  The EC firmware does
// not support the same mailbox commands so it is not registered as a
// cros_ec device type.
//
// Most messages follow a standard format, but there are some exceptions
// and an interface is provided to do direct/raw transactions that do not
// make assumptions about byte placement.
//

// Version of mailbox interface
pub const EC_MAILBOX_VERSION: c_int = 0;
// Command to start mailbox transaction
pub const EC_MAILBOX_START_COMMAND: c_uint = 0xda;
// Version of EC protocol
pub const EC_MAILBOX_PROTO_VERSION: c_int = 3;
// Number of header bytes to be counted as data bytes
pub const EC_MAILBOX_DATA_EXTRA: c_int = 2;
// Maximum timeout

// EC response flags

//
// wilco_ec_response_timed_out() - Wait for EC response.
// @ec: EC device.
//
// Return: true if EC timed out, false if EC did not time out.
//
#[no_mangle]
unsafe extern "C" fn wilco_ec_response_timed_out(ec: *mut wilco_ec_device) -> bool {
    static bool wilco_ec_response_timed_out(struct wilco_ec_device *ec)
    {
    let mut timeout: c_ulong = jiffies + EC_MAILBOX_TIMEOUT;
    do {
    if (!(inb(ec.io_command.start) &
    (EC_CMDR_PENDING | EC_CMDR_BUSY)))
    return false;
    usleep_range(100, 200);
    } while (time_before(jiffies, timeout));
    return true;
    }
//
// wilco_ec_checksum() - Compute 8-bit checksum over data range.
// @data: Data to checksum.
// @size: Number of bytes to checksum.
//
// Return: 8-bit checksum of provided data.
//
#[no_mangle]
unsafe extern "C" fn wilco_ec_checksum(data: *const c_void, size: usize) -> u8 {
    static u8 wilco_ec_checksum(const void *data, size_t size)
    {
    u8 *data_bytes = (u8 *)data;
    let mut checksum: u8 = 0;
    size_t i;
    for (i = 0; i < size; i++)
    checksum += data_bytes[i];
    return checksum;
    }
//
// wilco_ec_prepare() - Prepare the request structure for the EC.
// @msg: EC message with request information.
// @rq: EC request structure to fill.
//
    static void wilco_ec_prepare(struct wilco_ec_message *msg,
    struct wilco_ec_request *rq)
    {
    memset(rq, 0, sizeof(*rq));
    rq.struct_version = EC_MAILBOX_PROTO_VERSION;
    rq.mailbox_id = msg.type;
    rq.mailbox_version = EC_MAILBOX_VERSION;
    rq.data_size = msg.request_size;
// Checksum header and data
    rq.checksum = wilco_ec_checksum(rq, sizeof(*rq));
    rq.checksum += wilco_ec_checksum(msg.request_data, msg.request_size);
    rq.checksum = -rq.checksum;
    }
//
// wilco_ec_transfer() - Perform actual data transfer.
// @ec: EC device.
// @msg: EC message data for request and response.
// @rq: Filled in request structure
//
// Context: ec->mailbox_lock should be held while using this function.
// Return: number of bytes received or negative error code on failure.
//
    static int wilco_ec_transfer(struct wilco_ec_device *ec,
    struct wilco_ec_message *msg,
    struct wilco_ec_request *rq)
    {
    struct wilco_ec_response *rs;
    int ret;
    u8 flag;
// Write request header, then data
    ret = cros_ec_lpc_io_bytes_mec(MEC_IO_WRITE, 0, sizeof(*rq), (u8 *)rq);
    if (ret < 0)
    return ret;
    ret = cros_ec_lpc_io_bytes_mec(MEC_IO_WRITE, sizeof(*rq), msg.request_size,
    msg.request_data);
    if (ret < 0)
    return ret;
// Start the command
    outb(EC_MAILBOX_START_COMMAND, ec.io_command.start);
// For some commands (eg shutdown) the EC will not respond, that's OK
    if (msg.flags & WILCO_EC_FLAG_NO_RESPONSE) {
    dev_dbg(ec.dev, "EC does not respond to this command\n");
    return 0;
    }
// Wait for it to complete
    if (wilco_ec_response_timed_out(ec)) {
    dev_dbg(ec.dev, "response timed out\n");
    return -ETIMEDOUT;
    }
// Check result
    flag = inb(ec.io_data.start);
    if (flag) {
    dev_dbg(ec.dev, "bad response: 0x%02x\n", flag);
    return -EIO;
    }
// Read back response
    rs = ec.data_buffer;
    ret = cros_ec_lpc_io_bytes_mec(MEC_IO_READ, 0,
    sizeof(*rs) + EC_MAILBOX_DATA_SIZE,
    (u8 *)rs);
    if (ret < 0)
    return ret;
    if (ret) {
    dev_dbg(ec.dev, "bad packet checksum 0x%02x\n", rs.checksum);
    return -EBADMSG;
    }
    if (rs.result) {
    dev_dbg(ec.dev, "EC reported failure: 0x%02x\n", rs.result);
    return -EBADMSG;
    }
    if (rs.data_size != EC_MAILBOX_DATA_SIZE) {
    dev_dbg(ec.dev, "unexpected packet size (%u != %u)\n",
    rs.data_size, EC_MAILBOX_DATA_SIZE);
    return -EMSGSIZE;
    }
    if (rs.data_size < msg.response_size) {
    dev_dbg(ec.dev, "EC didn't return enough data (%u < %zu)\n",
    rs.data_size, msg.response_size);
    return -EMSGSIZE;
    }
    memcpy(msg.response_data, rs.data, msg.response_size);
    return rs.data_size;
    }
//
// wilco_ec_mailbox() - Send EC request and receive EC response.
// @ec: EC device.
// @msg: EC message data for request and response.
//
// On entry msg->type, msg->request_size, and msg->request_data should all be
// filled in. If desired, msg->flags can be set.
//
// If a response is expected, msg->response_size should be set, and
// msg->response_data should point to a buffer with enough space. On exit
// msg->response_data will be filled.
//
// Return: number of bytes received or negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn wilco_ec_mailbox(ec: *mut wilco_ec_device, msg: *mut wilco_ec_message) -> c_int {
    int wilco_ec_mailbox(struct wilco_ec_device *ec, struct wilco_ec_message *msg)
    {
    struct wilco_ec_request *rq;
    int ret;
    dev_dbg(ec.dev, "type=%04x flags=%02x rslen=%zu rqlen=%zu\n",
    msg.type, msg.flags, msg.response_size, msg.request_size);
    mutex_lock(&ec.mailbox_lock);
// Prepare request packet
    rq = ec.data_buffer;
    wilco_ec_prepare(msg, rq);
    ret = wilco_ec_transfer(ec, msg, rq);
    mutex_unlock(&ec.mailbox_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(wilco_ec_mailbox);
