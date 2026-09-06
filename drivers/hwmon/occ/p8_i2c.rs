//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/occ/p8_i2c.c
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
// Copyright IBM Corp 2019

pub const OCC_TIMEOUT_MS: c_int = 1000;
pub const OCC_CMD_IN_PRG_WAIT_MS: c_int = 50;
// OCB (on-chip control bridge - interface to OCC) registers
pub const OCB_DATA1: c_uint = 0x6B035;
pub const OCB_ADDR: c_uint = 0x6B070;
pub const OCB_DATA3: c_uint = 0x6B075;
// OCC SRAM address space
pub const OCC_SRAM_ADDR_CMD: c_uint = 0xFFFF6000;
pub const OCC_SRAM_ADDR_RESP: c_uint = 0xFFFF7000;
pub const OCC_DATA_ATTN: c_uint = 0x20010000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p8_i2c_occ {
    pub occ: occ,
    pub client: *mut i2c_client,
}

#[no_mangle]
unsafe extern "C" fn p8_i2c_occ_getscom(client: *mut i2c_client, address: u32, data: *mut u8) -> c_int {
    static int p8_i2c_occ_getscom(struct i2c_client *client, u32 address, u8 *data)
    {
    ssize_t rc;
    __be64 buf;
    struct i2c_msg msgs[2];
// p8 i2c slave requires shift
    address <<= 1;
    msgs[0].addr = client.addr;
    msgs[0].flags = client.flags & I2C_M_TEN;
    msgs[0].len = sizeof(u32);
// address is a scom address; bus-endian
    msgs[0].buf = (char *)&address;
// data from OCC is big-endian
    msgs[1].addr = client.addr;
    msgs[1].flags = (client.flags & I2C_M_TEN) | I2C_M_RD;
    msgs[1].len = sizeof(u64);
    msgs[1].buf = (char *)&buf;
    rc = i2c_transfer(client.adapter, msgs, 2);
    if (rc < 0)
    return rc;
// (u64 *)data = be64_to_cpu(buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p8_i2c_occ_putscom(client: *mut i2c_client, address: u32, data: *mut u8) -> c_int {
    static int p8_i2c_occ_putscom(struct i2c_client *client, u32 address, u8 *data)
    {
    u32 buf[3];
    ssize_t rc;
// p8 i2c slave requires shift
    address <<= 1;
// address is bus-endian; data passed through from user as-is
    buf[0] = address;
    memcpy(&buf[1], &data[4], sizeof(u32));
    memcpy(&buf[2], data, sizeof(u32));
    rc = i2c_master_send(client, (const char *)buf, sizeof(buf));
    if (rc < 0)
    return rc;
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(buf): rc !=) -> else {
    else if (rc != sizeof(buf))
    return -EIO;
    return 0;
    }
    static int p8_i2c_occ_putscom_u32(struct i2c_client *client, u32 address,
    u32 data0, u32 data1)
    {
    u8 buf[8];
    memcpy(buf, &data0, 4);
    memcpy(buf + 4, &data1, 4);
    return p8_i2c_occ_putscom(client, address, buf);
    }
    static int p8_i2c_occ_putscom_be(struct i2c_client *client, u32 address,
    u8 *data, size_t len)
    {
    let mut data0: __be32 = 0, data1 = 0;
    memcpy(&data0, data, min_t(size_t, len, 4));
    if (len > 4) {
    len -= 4;
    memcpy(&data1, data + 4, min_t(size_t, len, 4));
    }
    return p8_i2c_occ_putscom_u32(client, address, be32_to_cpu(data0),
    be32_to_cpu(data1));
    }
    static int p8_i2c_occ_send_cmd(struct occ *occ, u8 *cmd, size_t len,
    void *resp, size_t resp_len)
    {
    int i, rc;
    unsigned long start;
    u16 data_length;
    let mut timeout: c_ulong = msecs_to_jiffies(OCC_TIMEOUT_MS);
    let mut wait_time: c_long = msecs_to_jiffies(OCC_CMD_IN_PRG_WAIT_MS);
    struct p8_i2c_occ *ctx = to_p8_i2c_occ(occ);
    struct i2c_client *client = ctx.client;
    struct occ_response *or = (struct occ_response *)resp;
    start = jiffies;
// set sram address for command
    rc = p8_i2c_occ_putscom_u32(client, OCB_ADDR, OCC_SRAM_ADDR_CMD, 0);
    if (rc)
    return rc;
// write command (expected to already be BE), we need bus-endian...
    rc = p8_i2c_occ_putscom_be(client, OCB_DATA3, cmd, len);
    if (rc)
    return rc;
// trigger OCC attention
    rc = p8_i2c_occ_putscom_u32(client, OCB_DATA1, OCC_DATA_ATTN, 0);
    if (rc)
    return rc;
    do {
// set sram address for response
    rc = p8_i2c_occ_putscom_u32(client, OCB_ADDR,
    OCC_SRAM_ADDR_RESP, 0);
    if (rc)
    return rc;
    rc = p8_i2c_occ_getscom(client, OCB_DATA3, (u8 *)resp);
    if (rc)
    return rc;
// wait for OCC
    if (or.return_status == OCC_RESP_CMD_IN_PRG) {
    rc = -EALREADY;
    if (time_after(jiffies, start + timeout))
    break;
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(wait_time);
    }
    } while (rc);
// check the OCC response
    switch (or.return_status) {
    case OCC_RESP_CMD_IN_PRG:
    rc = -ETIMEDOUT;
    break;
    case OCC_RESP_SUCCESS:
    rc = 0;
    break;
    case OCC_RESP_CMD_INVAL:
    case OCC_RESP_CMD_LEN_INVAL:
    case OCC_RESP_DATA_INVAL:
    case OCC_RESP_CHKSUM_ERR:
    rc = -EINVAL;
    break;
    case OCC_RESP_INT_ERR:
    case OCC_RESP_BAD_STATE:
    case OCC_RESP_CRIT_EXCEPT:
    case OCC_RESP_CRIT_INIT:
    case OCC_RESP_CRIT_WATCHDOG:
    case OCC_RESP_CRIT_OCB:
    case OCC_RESP_CRIT_HW:
    rc = -EREMOTEIO;
    break;
    default:
    rc = -EPROTO;
    }
    if (rc < 0)
    return rc;
    data_length = get_unaligned_be16(&or.data_length);
    if ((data_length + 7) > resp_len)
    return -EMSGSIZE;
// fetch the rest of the response data
    for (i = 8; i < data_length + 7; i += 8) {
    rc = p8_i2c_occ_getscom(client, OCB_DATA3, ((u8 *)resp) + i);
    if (rc)
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p8_i2c_occ_probe(client: *mut i2c_client) -> c_int {
    static int p8_i2c_occ_probe(struct i2c_client *client)
    {
    struct occ *occ;
    struct p8_i2c_occ *ctx = devm_kzalloc(&client.dev, sizeof(*ctx),
    GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.client = client;
    occ = &ctx.occ;
    occ.bus_dev = &client.dev;
    dev_set_drvdata(&client.dev, occ);
    occ.powr_sample_time_us = 250;
    occ.poll_cmd_data = 0x10;		/* P8 OCC poll data */
    occ.send_cmd = p8_i2c_occ_send_cmd;
    return occ_setup(occ);
    }
#[no_mangle]
unsafe extern "C" fn p8_i2c_occ_remove(client: *mut i2c_client) {
    static void p8_i2c_occ_remove(struct i2c_client *client)
    {
    struct occ *occ = dev_get_drvdata(&client.dev);
    occ_shutdown(occ);
    }
    static const struct of_device_id p8_i2c_occ_of_match[] = {
    { .compatible = "ibm,p8-occ-hwmon" },
    {}
    };
    MODULE_DEVICE_TABLE(of, p8_i2c_occ_of_match);
    static struct i2c_driver p8_i2c_occ_driver = {
    .driver = {
    .name = "occ-hwmon",
    .of_match_table = p8_i2c_occ_of_match,
    },
    .probe = p8_i2c_occ_probe,
    .remove = p8_i2c_occ_remove,
    };
    module_i2c_driver(p8_i2c_occ_driver);
    MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
    MODULE_DESCRIPTION("BMC P8 OCC hwmon driver");
    MODULE_LICENSE("GPL");
