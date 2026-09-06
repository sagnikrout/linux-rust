//! Automatically rewritten from C to Rust
//! Source: drivers/net/pse-pd/realtek-pse-mcu-i2c.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// The core has already waited RTPSE_MCU_RESPONSE_MS before calling us, so
// the response is normally ready on the very first read. For commands the
// MCU produces more slowly, keep polling at the typical response cadence
// up to the worst-case ceiling.
//

#[no_mangle]
unsafe extern "C" fn rtpse_mcu_i2c_smbus_send(pse: *mut rtpse_mcu_ctrl, req: *const rtpse_mcu_msg) -> c_int {
    static int rtpse_mcu_i2c_smbus_send(struct rtpse_mcu_ctrl *pse, const struct rtpse_mcu_msg *req)
    {
    struct i2c_client *client = to_i2c_client(pse.dev);
// Send opcode as SMBus command byte; remaining 11 bytes as block data
    return i2c_smbus_write_i2c_block_data(client, req.opcode, RTPSE_MCU_MSG_SIZE - 1,
    (const u8 *)req + 1);
    }
    static int rtpse_mcu_i2c_smbus_recv(struct rtpse_mcu_ctrl *pse, const struct rtpse_mcu_msg *req,
    struct rtpse_mcu_msg *resp)
    {
    struct i2c_client *client = to_i2c_client(pse.dev);
    int tries, ret;
    for (tries = 0; tries < RTPSE_MCU_I2C_MAX_TRIES; tries++) {
    if (tries > 0)
    msleep(RTPSE_MCU_I2C_RETRY_MS);
// MCU needs 0x00 as command byte for read
    ret = i2c_smbus_read_i2c_block_data(client, 0x00,
    RTPSE_MCU_MSG_SIZE,
    (u8 *)resp);
    if (ret < 0)
    return ret;
    if (ret == RTPSE_MCU_MSG_SIZE && rtpse_mcu_resp_is_final(req, resp))
    return 0;
    }
    return -ETIMEDOUT;
    }
    static const struct rtpse_mcu_transport_ops rtpse_mcu_i2c_smbus_ops = {
    .send = rtpse_mcu_i2c_smbus_send,
    .recv = rtpse_mcu_i2c_smbus_recv,
    };
#[no_mangle]
unsafe extern "C" fn rtpse_mcu_i2c_native_send(pse: *mut rtpse_mcu_ctrl, req: *const rtpse_mcu_msg) -> c_int {
    static int rtpse_mcu_i2c_native_send(struct rtpse_mcu_ctrl *pse, const struct rtpse_mcu_msg *req)
    {
    struct i2c_client *client = to_i2c_client(pse.dev);
    int ret;
    ret = i2c_master_send(client, (const u8 *)req, RTPSE_MCU_MSG_SIZE);
    if (ret < 0)
    return ret;
    let mut ret: return = = RTPSE_MCU_MSG_SIZE ? 0 : -EIO;
    }
    static int rtpse_mcu_i2c_native_recv(struct rtpse_mcu_ctrl *pse, const struct rtpse_mcu_msg *req,
    struct rtpse_mcu_msg *resp)
    {
    struct i2c_client *client = to_i2c_client(pse.dev);
    int tries, ret;
    for (tries = 0; tries < RTPSE_MCU_I2C_MAX_TRIES; tries++) {
    if (tries > 0)
    msleep(RTPSE_MCU_I2C_RETRY_MS);
    ret = i2c_master_recv(client, (u8 *)resp, RTPSE_MCU_MSG_SIZE);
    if (ret < 0)
    return ret;
    if (ret == RTPSE_MCU_MSG_SIZE && rtpse_mcu_resp_is_final(req, resp))
    return 0;
    }
    return -ETIMEDOUT;
    }
    static const struct rtpse_mcu_transport_ops rtpse_mcu_i2c_native_ops = {
    .send = rtpse_mcu_i2c_native_send,
    .recv = rtpse_mcu_i2c_native_recv,
    };
#[no_mangle]
unsafe extern "C" fn rtpse_mcu_i2c_probe(client: *mut i2c_client) -> c_int {
    static int rtpse_mcu_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    const struct rtpse_mcu_match_data *match;
    struct rtpse_mcu_ctrl *pse;
    bool use_native;
    match = device_get_match_data(dev);
    if (!match)
    return dev_err_probe(dev, -ENODEV, "missing match data\n");
// The framing (raw I2C vs SMBus) is carried by the match data.
    use_native = match.native_i2c;
    if (use_native) {
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return dev_err_probe(dev, -EOPNOTSUPP,
    "plain-I2C MCU protocol requires I2C-capable adapter\n");
    } else {
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WRITE_I2C_BLOCK |
    I2C_FUNC_SMBUS_READ_I2C_BLOCK))
    return dev_err_probe(dev, -EOPNOTSUPP,
    "SMBus MCU protocol requires SMBus I2C-block support\n");
    }
    pse = devm_kzalloc(dev, sizeof(*pse), GFP_KERNEL);
    if (!pse)
    return -ENOMEM;
    pse.dev = dev;
    pse.pcdev.owner = THIS_MODULE;
    pse.transport = use_native ? &rtpse_mcu_i2c_native_ops : &rtpse_mcu_i2c_smbus_ops;
    return rtpse_mcu_register(pse);
    }
    static const struct of_device_id rtpse_mcu_i2c_of_match[] = {
    { .compatible = "realtek,pse-mcu-gen1-smbus", .data = &rtpse_mcu_gen1_data },
    { .compatible = "realtek,pse-mcu-gen2-smbus", .data = &rtpse_mcu_gen2_data },
    { .compatible = "realtek,pse-mcu-gen2-i2c", .data = &rtpse_mcu_gen2_i2c_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rtpse_mcu_i2c_of_match);
    static struct i2c_driver rtpse_mcu_i2c_driver = {
    .driver = {
    .name		= "realtek-pse-mcu-i2c",
    .of_match_table	= rtpse_mcu_i2c_of_match,
    },
    .probe		= rtpse_mcu_i2c_probe,
    };
    module_i2c_driver(rtpse_mcu_i2c_driver);
    MODULE_AUTHOR("Jonas Jelonek <jelonek.jonas@gmail.com>");
    MODULE_DESCRIPTION("Realtek PSE MCU driver (I2C transport)");
    MODULE_LICENSE("GPL");
