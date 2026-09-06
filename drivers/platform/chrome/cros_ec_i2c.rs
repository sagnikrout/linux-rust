//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/cros_ec_i2c.c
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
// I2C interface for ChromeOS Embedded Controller
//
// Copyright (C) 2012 Google, Inc

//
// Request format for protocol v3
// byte 0	0xda (EC_COMMAND_PROTOCOL_3)
// byte 1-8	struct ec_host_request
// byte 10-	response data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_request_i2c {
// Always 0xda to backward compatible with v2 struct
    pub command_protocol: u8,
    pub ec_request: ec_host_request,
    pub __packed: },
//
// Response format for protocol v3
// byte 0	result code
// byte 1	packet_length
// byte 2-9	struct ec_host_response
// byte 10-	response data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_response_i2c {
    pub result: u8,
    pub packet_length: u8,
    pub ec_response: ec_host_response,
    pub __packed: },
    static inline struct cros_ec_device *to_ec_dev(struct device *dev)
    {
    pub to_i2c_client(dev): *mut *mut i2c_client client =,
    pub i2c_get_clientdata(client): return,
    }
    static int cros_ec_pkt_xfer_i2c(struct cros_ec_device *ec_dev,
    struct cros_ec_command *msg)
    {
    pub ec_dev->priv: *mut *mut i2c_client client =,
    pub -ENOMEM: int ret =,
    pub i: c_int,
    pub packet_len: c_int,
    pub NULL: *mut *mut u8 out_buf =,
    pub NULL: *mut *mut u8 in_buf =,
    pub sum: u8,
    pub i2c_msg: [i2c_msg; 2],
    pub ec_response: *mut ec_host_response,
    pub ec_request_i2c: *mut ec_host_request_i2c,
    pub ec_response_i2c: *mut ec_host_response_i2c,
    pub ec_host_request_i2c): int request_header_size = sizeof(struct,
    pub ec_host_response_i2c): int response_header_size = sizeof(struct,
    pub client->addr: i2c_msg[0].addr =,
    pub 0: i2c_msg[0].flags =,
    pub client->addr: i2c_msg[1].addr =,
    pub I2C_M_RD: i2c_msg[1].flags =,
    pub response_header_size: packet_len = msg->insize +,
    if (packet_len > ec_dev.din_size) {
    pub -EINVAL: ret =,
    pub done: goto,
    }
    pub ec_dev->din: in_buf =,
    pub packet_len: i2c_msg[1].len =,
    pub in_buf: *mut *mut i2c_msg[1].buf = (char ),
    pub request_header_size: packet_len = msg->outsize +,
    if (packet_len > ec_dev.dout_size) {
    pub -EINVAL: ret =,
    pub done: goto,
    }
    pub ec_dev->dout: out_buf =,
    pub packet_len: i2c_msg[0].len =,
    pub out_buf: *mut *mut i2c_msg[0].buf = (char ),
// create request data
    pub out_buf: *mut *mut ec_request_i2c = (struct ec_host_request_i2c ),
    pub EC_COMMAND_PROTOCOL_3: ec_request_i2c->command_protocol =,
    pub msg): ret = cros_ec_prepare_tx(ec_dev,,
    if (ret < 0)
    pub done: goto,
// send command to EC and read answer
    pub 2): ret = i2c_transfer(client->adapter, i2c_msg,,
    if (ret < 0) {
    pub ret): dev_dbg(ec_dev->dev, "i2c transfer failed: %d\n",,
    pub done: goto,
    } else if (ret != 2) {
    pub ret): dev_err(ec_dev->dev, "failed to get response: %d\n",,
    pub -EIO: ret =,
    pub done: goto,
    }
    pub in_buf: *mut *mut ec_response_i2c = (struct ec_host_response_i2c ),
    pub ec_response_i2c->result: msg->result =,
    pub &ec_response_i2c->ec_response: ec_response =,
    switch (msg.result) {
    case EC_RES_SUCCESS:
    case EC_RES_IN_PROGRESS:
    pub -EAGAIN: ret =,
    dev_dbg(ec_dev.dev, "command 0x%02x in progress\n",
    pub done: goto,
    default:
    dev_dbg(ec_dev.dev, "command 0x%02x returned %d\n",
    pub msg->result): msg->command,,
//
// When we send v3 request to v2 ec, ec won't recognize the
// 0xda (EC_COMMAND_PROTOCOL_3) and will return with status
// EC_RES_INVALID_COMMAND with zero data length.
//
// In case of invalid command for v3 protocol the data length
// will be at least sizeof(struct ec_host_response)
//
    if (ec_response_i2c.result == EC_RES_INVALID_COMMAND &&
    ec_response_i2c.packet_length == 0) {
    pub -EPROTONOSUPPORT: ret =,
    pub done: goto,
    }
    }
    if (ec_response_i2c.packet_length < sizeof(struct ec_host_response)) {
    dev_err(ec_dev.dev,
    pub header\n",: "response of %u bytes too short; not a full,
    pub -EBADMSG: ret =,
    pub done: goto,
    }
    if (msg.insize < ec_response.data_len) {
    dev_err(ec_dev.dev,
    "response data size is too large: expected %u, got %u\n",
    msg.insize,
    pub -EMSGSIZE: ret =,
    pub done: goto,
    }
// copy response packet payload and compute checksum
    pub 0: sum =,
    pub i++): for (i = 0; i < sizeof(struct ec_host_response);,
    pub )ec_response)[i]: *mut sum += ((u8,
    memcpy(msg.data,
    in_buf + response_header_size,
    pub i++): for (i = 0; i < ec_response->data_len;,
    pub msg->data[i]: sum +=,
// All bytes should sum to zero
    if (sum) {
    pub checksum\n"): dev_err(ec_dev->dev, "bad packet,
    pub -EBADMSG: ret =,
    pub done: goto,
    }
    pub ec_response->data_len: ret =,
    done:
    if (msg.command == EC_CMD_REBOOT_EC)
    pub ret: return,
    }
    static int cros_ec_cmd_xfer_i2c(struct cros_ec_device *ec_dev,
    struct cros_ec_command *msg)
    {
    pub ec_dev->priv: *mut *mut i2c_client client =,
    pub -ENOMEM: int ret =,
    pub i: c_int,
    pub len: c_int,
    pub packet_len: c_int,
    pub NULL: *mut *mut u8 out_buf =,
    pub NULL: *mut *mut u8 in_buf =,
    pub sum: u8,
    pub i2c_msg: [i2c_msg; 2],
    pub client->addr: i2c_msg[0].addr =,
    pub 0: i2c_msg[0].flags =,
    pub client->addr: i2c_msg[1].addr =,
    pub I2C_M_RD: i2c_msg[1].flags =,
//
// allocate larger packet (one byte for checksum, one byte for
// length, and one for result code)
//
    pub 3: packet_len = msg->insize +,
    pub GFP_KERNEL): in_buf = kzalloc(packet_len,,
    if (!in_buf)
    pub done: goto,
    pub packet_len: i2c_msg[1].len =,
    pub )in_buf: *mut i2c_msg[1].buf = (char,
//
// allocate larger packet (one byte for checksum, one for
// command code, one for length, and one for command version)
//
    pub 4: packet_len = msg->outsize +,
    pub GFP_KERNEL): out_buf = kzalloc(packet_len,,
    if (!out_buf)
    pub done: goto,
    pub packet_len: i2c_msg[0].len =,
    pub )out_buf: *mut i2c_msg[0].buf = (char,
    pub msg->version: out_buf[0] = EC_CMD_VERSION0 +,
    pub msg->command: out_buf[1] =,
    pub msg->outsize: out_buf[2] =,
// copy message payload and compute checksum
    pub out_buf: [sum = out_buf[0] + out_buf[1] +; 2],
    pub {: for (i = 0; i < msg->outsize; i++),
    pub msg->data[i]: out_buf[3 + i] =,
    pub i]: sum += out_buf[3 +,
    }
    pub sum: out_buf[3 + msg->outsize] =,
// send command to EC and read answer
    pub 2): ret = i2c_transfer(client->adapter, i2c_msg,,
    if (ret < 0) {
    pub ret): dev_err(ec_dev->dev, "i2c transfer failed: %d\n",,
    pub done: goto,
    } else if (ret != 2) {
    pub ret): dev_err(ec_dev->dev, "failed to get response: %d\n",,
    pub -EIO: ret =,
    pub done: goto,
    }
// check response error code
    pub i2c_msg: [msg->result =; 1].buf[0],
    pub msg): ret = cros_ec_check_result(ec_dev,,
    if (ret)
    pub done: goto,
    pub in_buf: [len =; 1],
    if (len > msg.insize) {
    dev_err(ec_dev.dev, "packet too long (%d bytes, expected %d)",
    pub msg->insize): len,,
    pub -ENOSPC: ret =,
    pub done: goto,
    }
// copy response packet payload and compute checksum
    pub in_buf: [sum = in_buf[0] +; 1],
    pub {: for (i = 0; i < len; i++),
    pub i]: msg->data[i] = in_buf[2 +,
    pub i]: sum += in_buf[2 +,
    }
    dev_dbg(ec_dev.dev, "packet: %*ph, sum = %02x\n",
    pub sum): i2c_msg[1].len, in_buf,,
    if (sum != in_buf[2 + len]) {
    pub checksum\n"): dev_err(ec_dev->dev, "bad packet,
    pub -EBADMSG: ret =,
    pub done: goto,
    }
    pub len: ret =,
    done:
    if (msg.command == EC_CMD_REBOOT_EC)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_i2c_probe(client: *mut i2c_client) -> c_int {
    static int cros_ec_i2c_probe(struct i2c_client *client)
    {
    pub &client->dev: *mut *mut device dev =,
    pub ec_dev: *mut cros_ec_device,
    pub err: c_int,
    pub cros_ec_device_alloc(dev): ec_dev =,
    if (!ec_dev)
    pub -ENOMEM: return,
    pub ec_dev): i2c_set_clientdata(client,,
    pub client: ec_dev->priv =,
    pub client->irq: ec_dev->irq =,
    pub cros_ec_cmd_xfer_i2c: ec_dev->cmd_xfer =,
    pub cros_ec_pkt_xfer_i2c: ec_dev->pkt_xfer =,
    pub client->adapter->name: ec_dev->phys_name =,
    pub cros_ec_register(ec_dev): err =,
    if (err) {
    pub EC\n"): dev_err(dev, "cannot register,
    pub err: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_i2c_remove(client: *mut i2c_client) {
    static void cros_ec_i2c_remove(struct i2c_client *client)
    {
    pub i2c_get_clientdata(client): *mut *mut cros_ec_device ec_dev =,
    }

#[no_mangle]
unsafe extern "C" fn cros_ec_i2c_suspend(dev: *mut device) -> c_int {
    static int cros_ec_i2c_suspend(struct device *dev)
    {
    pub to_ec_dev(dev): *mut *mut cros_ec_device ec_dev =,
    pub cros_ec_suspend(ec_dev): return,
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_i2c_resume(dev: *mut device) -> c_int {
    static int cros_ec_i2c_resume(struct device *dev)
    {
    pub to_ec_dev(dev): *mut *mut cros_ec_device ec_dev =,
    pub cros_ec_resume(ec_dev): return,
    }

    static const struct dev_pm_ops cros_ec_i2c_pm_ops = {
    SET_LATE_SYSTEM_SLEEP_PM_OPS(cros_ec_i2c_suspend, cros_ec_i2c_resume)
}

    static const struct of_device_id cros_ec_i2c_of_match[] = {
    { .compatible = "google,cros-ec-i2c", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, cros_ec_i2c_of_match);

    static const struct i2c_device_id cros_ec_i2c_id[] = {
    { .name = "cros-ec-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cros_ec_i2c_id);

    static const struct acpi_device_id cros_ec_i2c_acpi_id[] = {
    { "GOOG0008", 0 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, cros_ec_i2c_acpi_id);

    static struct i2c_driver cros_ec_driver = {
    .driver	= {
    .name	= "cros-ec-i2c",
    .acpi_match_table = ACPI_PTR(cros_ec_i2c_acpi_id),
    .of_match_table = of_match_ptr(cros_ec_i2c_of_match),
    .pm	= &cros_ec_i2c_pm_ops,
    },
    .probe		= cros_ec_i2c_probe,
    .remove		= cros_ec_i2c_remove,
    .id_table	= cros_ec_i2c_id,
    };
    module_i2c_driver(cros_ec_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("I2C interface for ChromeOS Embedded Controller");
