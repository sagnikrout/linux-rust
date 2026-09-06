//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/ucsi/ucsi_stm32g0.c
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// UCSI driver for STMicroelectronics STM32G0 Type-C PD controller
//
// Copyright (C) 2022, STMicroelectronics - All Rights Reserved
// Author: Fabrice Gasnier <fabrice.gasnier@foss.st.com>.
//

// STM32G0 I2C bootloader addr: 0b1010001x (See AN2606)

// STM32G0 I2C bootloader max data size
pub const STM32G0_I2C_BL_SZ: c_int = 256;
// STM32 I2C bootloader commands (See AN4221)
pub const STM32_CMD_GVR: c_uint = 0x01	/* Gets the bootloader version */;
pub const STM32_CMD_GVR_LEN: c_int = 1;
pub const STM32_CMD_RM: c_uint = 0x11	/* Reag memory */;
pub const STM32_CMD_WM: c_uint = 0x31	/* Write memory */;

pub const STM32_CMD_ERASE: c_uint = 0x44	/* Erase page, bank or all */;
pub const STM32_CMD_ERASE_SPECIAL_LEN: c_int = 3;
pub const STM32_CMD_GLOBAL_MASS_ERASE: c_uint = 0xffff /* All-bank erase */;
// STM32 I2C bootloader answer status
pub const STM32G0_I2C_BL_ACK: c_uint = 0x79;
pub const STM32G0_I2C_BL_NACK: c_uint = 0x1f;
pub const STM32G0_I2C_BL_BUSY: c_uint = 0x76;
// STM32G0 flash definitions
pub const STM32G0_USER_OPTION_BYTES: c_uint = 0x1fff7800;

pub const STM32G0_MAIN_MEM_ADDR: c_uint = 0x08000000;
// STM32 Firmware definitions: additional commands
pub const STM32G0_FW_GETVER: c_uint = 0x00	/* Gets the firmware version */;
pub const STM32G0_FW_GETVER_LEN: c_int = 4;
pub const STM32G0_FW_RSTGOBL: c_uint = 0x21	/* Reset and go to bootloader */;
pub const STM32G0_FW_KEYWORD: c_uint = 0xa56959a6;
// ucsi_stm32g0_fw_info located at the end of the firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_stm32g0_fw_info {
    pub version: u32,
    pub keyword: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_stm32g0 {
    pub client: *mut i2c_client,
    pub i2c_bl: *mut i2c_client,
    pub in_bootloader: bool,
    pub bl_version: u8,
    pub dev: *mut device,
    pub fw_name: *const c_char,
    pub ucsi: *mut ucsi,
    pub suspended: bool,
    pub wakeup_event: bool,
}

//
// Bootloader commands helpers:
// - send command (2 bytes)
// - check ack
// Then either:
// - receive data
// - receive data + check ack
// - send data + check ack
// These operations depends on the command and have various length.
//
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_check_ack(ucsi: *mut ucsi) -> c_int {
    static int ucsi_stm32g0_bl_check_ack(struct ucsi *ucsi)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.i2c_bl;
    unsigned char ack;
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = I2C_M_RD,
    .len	= 1,
    .buf	= &ack,
    },
    };
    int ret;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(g0.dev, "i2c bl ack (%02x), error: %d\n", client.addr, ret);
    return ret < 0 ? ret : -EIO;
    }
// The 'ack' byte should contain bootloader answer: ack/nack/busy
    switch (ack) {
    case STM32G0_I2C_BL_ACK:
    return 0;
    case STM32G0_I2C_BL_NACK:
    return -ENOENT;
    case STM32G0_I2C_BL_BUSY:
    return -EBUSY;
    default:
    dev_err(g0.dev, "i2c bl ack (%02x), invalid byte: %02x\n",
    client.addr, ack);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_cmd_check_ack(ucsi: *mut ucsi, cmd: c_uint, check_ack: bool) -> c_int {
    static int ucsi_stm32g0_bl_cmd_check_ack(struct ucsi *ucsi, unsigned int cmd, bool check_ack)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.i2c_bl;
    unsigned char buf[2];
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = 0,
    .len	= sizeof(buf),
    .buf	= buf,
    },
    };
    int ret;
//
// Send STM32 bootloader command format is two bytes:
// - command code
// - XOR'ed command code
//
    buf[0] = cmd;
    buf[1] = cmd ^ 0xff;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_dbg(g0.dev, "i2c bl cmd %d (%02x), error: %d\n", cmd, client.addr, ret);
    return ret < 0 ? ret : -EIO;
    }
    if (check_ack)
    return ucsi_stm32g0_bl_check_ack(ucsi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_cmd(ucsi: *mut ucsi, cmd: c_uint) -> c_int {
    static int ucsi_stm32g0_bl_cmd(struct ucsi *ucsi, unsigned int cmd)
    {
    return ucsi_stm32g0_bl_cmd_check_ack(ucsi, cmd, true);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_rcv_check_ack(ucsi: *mut ucsi, data: *mut c_void, len: usize, check_ack: bool) -> c_int {
    static int ucsi_stm32g0_bl_rcv_check_ack(struct ucsi *ucsi, void *data, size_t len, bool check_ack)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.i2c_bl;
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = I2C_M_RD,
    .len	= len,
    .buf	= data,
    },
    };
    int ret;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(g0.dev, "i2c bl rcv %02x, error: %d\n", client.addr, ret);
    return ret < 0 ? ret : -EIO;
    }
    if (check_ack)
    return ucsi_stm32g0_bl_check_ack(ucsi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_rcv(ucsi: *mut ucsi, data: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_bl_rcv(struct ucsi *ucsi, void *data, size_t len)
    {
    return ucsi_stm32g0_bl_rcv_check_ack(ucsi, data, len, true);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_rcv_woack(ucsi: *mut ucsi, data: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_bl_rcv_woack(struct ucsi *ucsi, void *data, size_t len)
    {
    return ucsi_stm32g0_bl_rcv_check_ack(ucsi, data, len, false);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_send(ucsi: *mut ucsi, data: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_bl_send(struct ucsi *ucsi, void *data, size_t len)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.i2c_bl;
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = 0,
    .len	= len,
    .buf	= data,
    },
    };
    int ret;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(g0.dev, "i2c bl send %02x, error: %d\n", client.addr, ret);
    return ret < 0 ? ret : -EIO;
    }
    return ucsi_stm32g0_bl_check_ack(ucsi);
    }
// Bootloader commands
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_get_version(ucsi: *mut ucsi, bl_version: *mut u8) -> c_int {
    static int ucsi_stm32g0_bl_get_version(struct ucsi *ucsi, u8 *bl_version)
    {
    int ret;
    ret = ucsi_stm32g0_bl_cmd(ucsi, STM32_CMD_GVR);
    if (ret)
    return ret;
    return ucsi_stm32g0_bl_rcv(ucsi, bl_version, STM32_CMD_GVR_LEN);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_send_addr(ucsi: *mut ucsi, addr: u32) -> c_int {
    static int ucsi_stm32g0_bl_send_addr(struct ucsi *ucsi, u32 addr)
    {
    u8 data8[STM32_CMD_ADDR_LEN];
// Address format: 4 bytes addr (MSB first) + XOR'ed addr bytes
    put_unaligned_be32(addr, data8);
    data8[4] = data8[0] ^ data8[1] ^ data8[2] ^ data8[3];
    return ucsi_stm32g0_bl_send(ucsi, data8, STM32_CMD_ADDR_LEN);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_global_mass_erase(ucsi: *mut ucsi) -> c_int {
    static int ucsi_stm32g0_bl_global_mass_erase(struct ucsi *ucsi)
    {
    u8 data8[4];
    u16 *data16 = (u16 *)&data8[0];
    int ret;
    data16[0] = STM32_CMD_GLOBAL_MASS_ERASE;
    data8[2] = data8[0] ^ data8[1];
    ret = ucsi_stm32g0_bl_cmd(ucsi, STM32_CMD_ERASE);
    if (ret)
    return ret;
    return ucsi_stm32g0_bl_send(ucsi, data8, STM32_CMD_ERASE_SPECIAL_LEN);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_write(ucsi: *mut ucsi, addr: u32, data: *const c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_bl_write(struct ucsi *ucsi, u32 addr, const void *data, size_t len)
    {
    u8 *data8;
    int i, ret;
    if (!len || len > STM32G0_I2C_BL_SZ)
    return -EINVAL;
// Write memory: len bytes -1, data up to 256 bytes + XOR'ed bytes
    data8 = kmalloc(STM32G0_I2C_BL_SZ + 2, GFP_KERNEL);
    if (!data8)
    return -ENOMEM;
    ret = ucsi_stm32g0_bl_cmd(ucsi, STM32_CMD_WM);
    if (ret)
    goto free;
    ret = ucsi_stm32g0_bl_send_addr(ucsi, addr);
    if (ret)
    goto free;
    data8[0] = len - 1;
    memcpy(data8 + 1, data, len);
    data8[len + 1] = data8[0];
    for (i = 1; i <= len; i++)
    data8[len + 1] ^= data8[i];
    ret = ucsi_stm32g0_bl_send(ucsi, data8, len + 2);
    free:
    kfree(data8);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_bl_read(ucsi: *mut ucsi, addr: u32, data: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_bl_read(struct ucsi *ucsi, u32 addr, void *data, size_t len)
    {
    int ret;
    if (!len || len > STM32G0_I2C_BL_SZ)
    return -EINVAL;
    ret = ucsi_stm32g0_bl_cmd(ucsi, STM32_CMD_RM);
    if (ret)
    return ret;
    ret = ucsi_stm32g0_bl_send_addr(ucsi, addr);
    if (ret)
    return ret;
    ret = ucsi_stm32g0_bl_cmd(ucsi, len - 1);
    if (ret)
    return ret;
    return ucsi_stm32g0_bl_rcv_woack(ucsi, data, len);
    }
// Firmware commands (the same address as the bootloader)
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_fw_cmd(ucsi: *mut ucsi, cmd: c_uint) -> c_int {
    static int ucsi_stm32g0_fw_cmd(struct ucsi *ucsi, unsigned int cmd)
    {
    return ucsi_stm32g0_bl_cmd_check_ack(ucsi, cmd, false);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_fw_rcv(ucsi: *mut ucsi, data: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_fw_rcv(struct ucsi *ucsi, void *data, size_t len)
    {
    return ucsi_stm32g0_bl_rcv_woack(ucsi, data, len);
    }
// UCSI ops
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_read(ucsi: *mut ucsi, offset: c_uint, val: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_read(struct ucsi *ucsi, unsigned int offset, void *val, size_t len)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.client;
    let mut reg: u8 = offset;
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = 0,
    .len	= 1,
    .buf	= &reg,
    },
    {
    .addr	= client.addr,
    .flags  = I2C_M_RD,
    .len	= len,
    .buf	= val,
    },
    };
    int ret;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(g0.dev, "i2c read %02x, %02x error: %d\n", client.addr, reg, ret);
    return ret < 0 ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_read_version(ucsi: *mut ucsi, version: *mut u16) -> c_int {
    static int ucsi_stm32g0_read_version(struct ucsi *ucsi, u16 *version)
    {
    return ucsi_stm32g0_read(ucsi, UCSI_VERSION, version, sizeof(*version));
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_read_cci(ucsi: *mut ucsi, cci: *mut u32) -> c_int {
    static int ucsi_stm32g0_read_cci(struct ucsi *ucsi, u32 *cci)
    {
    return ucsi_stm32g0_read(ucsi, UCSI_CCI, cci, sizeof(*cci));
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_read_message_in(ucsi: *mut ucsi, val: *mut c_void, len: usize) -> c_int {
    static int ucsi_stm32g0_read_message_in(struct ucsi *ucsi, void *val, size_t len)
    {
    return ucsi_stm32g0_read(ucsi, UCSI_MESSAGE_IN, val, len);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_async_control(ucsi: *mut ucsi, command: u64) -> c_int {
    static int ucsi_stm32g0_async_control(struct ucsi *ucsi, u64 command)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.client;
    struct i2c_msg msg[] = {
    {
    .addr	= client.addr,
    .flags  = 0,
    }
    };
    unsigned char *buf;
    int ret;
    buf = kmalloc(sizeof(command) + 1, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    buf[0] = UCSI_CONTROL;
    memcpy(&buf[1], &command, sizeof(command));
    msg[0].len = sizeof(command) + 1;
    msg[0].buf = buf;
    ret = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    kfree(buf);
    if (ret != ARRAY_SIZE(msg)) {
    dev_err(g0.dev, "i2c write %02x, %02x error: %d\n", client.addr, UCSI_CONTROL, ret);
    return ret < 0 ? ret : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ucsi_stm32g0_irq_handler(int irq, void *data)
    {
    struct ucsi_stm32g0 *g0 = data;
    u32 cci;
    int ret;
    if (g0.suspended)
    g0.wakeup_event = true;
    ret = ucsi_stm32g0_read(g0.ucsi, UCSI_CCI, &cci, sizeof(cci));
    if (ret)
    return IRQ_NONE;
    ucsi_notify_common(g0.ucsi, cci);
    return IRQ_HANDLED;
    }
    static const struct ucsi_operations ucsi_stm32g0_ops = {
    .read_version = ucsi_stm32g0_read_version,
    .read_cci = ucsi_stm32g0_read_cci,
    .poll_cci = ucsi_stm32g0_read_cci,
    .read_message_in = ucsi_stm32g0_read_message_in,
    .sync_control = ucsi_sync_control_common,
    .async_control = ucsi_stm32g0_async_control,
    };
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_register(ucsi: *mut ucsi) -> c_int {
    static int ucsi_stm32g0_register(struct ucsi *ucsi)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.client;
    int ret;
// Request alert interrupt
    ret = request_threaded_irq(client.irq, core::ptr::null_mut(), ucsi_stm32g0_irq_handler, IRQF_ONESHOT,
    dev_name(g0.dev), g0);
    if (ret) {
    dev_err(g0.dev, "request IRQ failed: %d\n", ret);
    return ret;
    }
    ret = ucsi_register(ucsi);
    if (ret) {
    dev_err_probe(g0.dev, ret, "ucsi_register failed\n");
    free_irq(client.irq, g0);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_unregister(ucsi: *mut ucsi) {
    static void ucsi_stm32g0_unregister(struct ucsi *ucsi)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    struct i2c_client *client = g0.client;
    ucsi_unregister(ucsi);
    free_irq(client.irq, g0);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_fw_cb(fw: *const firmware, context: *mut c_void) {
    static void ucsi_stm32g0_fw_cb(const struct firmware *fw, void *context)
    {
    struct ucsi_stm32g0 *g0;
    const u8 *data, *end;
    const struct ucsi_stm32g0_fw_info *fw_info;
    let mut addr: u32 = STM32G0_MAIN_MEM_ADDR, ob, fw_version;
    int ret, size;
    if (!context)
    return;
    g0 = ucsi_get_drvdata(context);
    if (!fw)
    goto fw_release;
    fw_info = (struct ucsi_stm32g0_fw_info *)(fw.data + fw.size - sizeof(*fw_info));
    if (!g0.in_bootloader) {
// Read running firmware version
    ret = ucsi_stm32g0_fw_cmd(g0.ucsi, STM32G0_FW_GETVER);
    if (ret) {
    dev_err(g0.dev, "Get version cmd failed %d\n", ret);
    goto fw_release;
    }
    ret = ucsi_stm32g0_fw_rcv(g0.ucsi, &fw_version,
    STM32G0_FW_GETVER_LEN);
    if (ret) {
    dev_err(g0.dev, "Get version failed %d\n", ret);
    goto fw_release;
    }
// Sanity check on keyword and firmware version
    if (fw_info.keyword != STM32G0_FW_KEYWORD || fw_info.version == fw_version)
    goto fw_release;
    dev_info(g0.dev, "Flashing FW: %08x (%08x cur)\n", fw_info.version, fw_version);
// Switch to bootloader mode
    ucsi_stm32g0_unregister(g0.ucsi);
    ret = ucsi_stm32g0_fw_cmd(g0.ucsi, STM32G0_FW_RSTGOBL);
    if (ret) {
    dev_err(g0.dev, "bootloader cmd failed %d\n", ret);
    goto fw_release;
    }
    g0.in_bootloader = true;
// STM32G0 reboot delay
    msleep(100);
    }
    ret = ucsi_stm32g0_bl_global_mass_erase(g0.ucsi);
    if (ret) {
    dev_err(g0.dev, "Erase failed %d\n", ret);
    goto fw_release;
    }
    data = fw.data;
    end = fw.data + fw.size;
    while (data < end) {
    size = min(end - data, STM32G0_I2C_BL_SZ);
    ret = ucsi_stm32g0_bl_write(g0.ucsi, addr, data, size);
    if (ret) {
    dev_err(g0.dev, "Write failed %d\n", ret);
    goto fw_release;
    }
    addr += size;
    data += size;
    }
    dev_dbg(g0.dev, "Configure to boot from main flash\n");
    ret = ucsi_stm32g0_bl_read(g0.ucsi, STM32G0_USER_OPTION_BYTES, &ob, sizeof(ob));
    if (ret) {
    dev_err(g0.dev, "read user option bytes failed %d\n", ret);
    goto fw_release;
    }
    dev_dbg(g0.dev, "STM32G0_USER_OPTION_BYTES 0x%08x\n", ob);
// Configure user option bytes to boot from main flash next time
    ob |= STM32G0_USER_OB_BOOT_MAIN;
// Writing option bytes will also reset G0 for updates to be loaded
    ret = ucsi_stm32g0_bl_write(g0.ucsi, STM32G0_USER_OPTION_BYTES, &ob, sizeof(ob));
    if (ret) {
    dev_err(g0.dev, "write user option bytes failed %d\n", ret);
    goto fw_release;
    }
    dev_info(g0.dev, "Starting, option bytes:0x%08x\n", ob);
// STM32G0 FW boot delay
    msleep(500);
// Register UCSI interface
    if (!ucsi_stm32g0_register(g0.ucsi))
    g0.in_bootloader = false;
    fw_release:
    release_firmware(fw);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_probe_bootloader(ucsi: *mut ucsi) -> c_int {
    static int ucsi_stm32g0_probe_bootloader(struct ucsi *ucsi)
    {
    struct ucsi_stm32g0 *g0 = ucsi_get_drvdata(ucsi);
    int ret;
    u16 ucsi_version;
// firmware-name is optional
    if (device_property_present(g0.dev, "firmware-name")) {
    ret = device_property_read_string(g0.dev, "firmware-name", &g0.fw_name);
    if (ret < 0)
    return dev_err_probe(g0.dev, ret, "Error reading firmware-name\n");
    }
    if (g0.fw_name) {
// STM32G0 in bootloader mode communicates at reserved address 0x51
    g0.i2c_bl = i2c_new_dummy_device(g0.client.adapter, STM32G0_I2C_BL_ADDR);
    if (IS_ERR(g0.i2c_bl)) {
    ret = dev_err_probe(g0.dev, PTR_ERR(g0.i2c_bl),
    "Failed to register bootloader I2C address\n");
    return ret;
    }
    }
//
// Try to guess if the STM32G0 is running a UCSI firmware. First probe the UCSI FW at its
// i2c address. Fallback to bootloader i2c address only if firmware-name is specified.
//
    ret = ucsi_stm32g0_read(ucsi, UCSI_VERSION, &ucsi_version, sizeof(ucsi_version));
    if (!ret || !g0.fw_name)
    return ret;
// Speculatively read the bootloader version that has a known length.
    ret = ucsi_stm32g0_bl_get_version(ucsi, &g0.bl_version);
    if (ret < 0) {
    i2c_unregister_device(g0.i2c_bl);
    return ret;
    }
// Device in bootloader mode
    g0.in_bootloader = true;
    dev_info(g0.dev, "Bootloader Version 0x%02x\n", g0.bl_version);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_probe(client: *mut i2c_client) -> c_int {
    static int ucsi_stm32g0_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct ucsi_stm32g0 *g0;
    int ret;
    g0 = devm_kzalloc(dev, sizeof(*g0), GFP_KERNEL);
    if (!g0)
    return -ENOMEM;
    g0.dev = dev;
    g0.client = client;
    i2c_set_clientdata(client, g0);
    g0.ucsi = ucsi_create(dev, &ucsi_stm32g0_ops);
    if (IS_ERR(g0.ucsi))
    return PTR_ERR(g0.ucsi);
    ucsi_set_drvdata(g0.ucsi, g0);
    ret = ucsi_stm32g0_probe_bootloader(g0.ucsi);
    if (ret < 0)
    goto destroy;
//
// Don't register in bootloader mode: wait for the firmware to be loaded and started before
// registering UCSI device.
//
    if (!g0.in_bootloader) {
    ret = ucsi_stm32g0_register(g0.ucsi);
    if (ret < 0)
    goto freei2c;
    }
    if (g0.fw_name) {
//
// Asynchronously flash (e.g. bootloader mode) or update the running firmware,
// not to hang the boot process
//
    ret = request_firmware_nowait(THIS_MODULE, FW_ACTION_UEVENT, g0.fw_name, g0.dev,
    GFP_KERNEL, g0.ucsi, ucsi_stm32g0_fw_cb);
    if (ret < 0) {
    dev_err_probe(dev, ret, "firmware request failed\n");
    goto unregister;
    }
    }
    return 0;
    unregister:
    if (!g0.in_bootloader)
    ucsi_stm32g0_unregister(g0.ucsi);
    freei2c:
    if (g0.fw_name)
    i2c_unregister_device(g0.i2c_bl);
    destroy:
    ucsi_destroy(g0.ucsi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_remove(client: *mut i2c_client) {
    static void ucsi_stm32g0_remove(struct i2c_client *client)
    {
    struct ucsi_stm32g0 *g0 = i2c_get_clientdata(client);
    if (!g0.in_bootloader)
    ucsi_stm32g0_unregister(g0.ucsi);
    if (g0.fw_name)
    i2c_unregister_device(g0.i2c_bl);
    ucsi_destroy(g0.ucsi);
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_suspend(dev: *mut device) -> c_int {
    static int ucsi_stm32g0_suspend(struct device *dev)
    {
    struct ucsi_stm32g0 *g0 = dev_get_drvdata(dev);
    struct i2c_client *client = g0.client;
    if (g0.in_bootloader)
    return 0;
// Keep the interrupt disabled until the i2c bus has been resumed
    disable_irq(client.irq);
    g0.suspended = true;
    g0.wakeup_event = false;
    if (device_may_wakeup(dev) || device_wakeup_path(dev))
    enable_irq_wake(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ucsi_stm32g0_resume(dev: *mut device) -> c_int {
    static int ucsi_stm32g0_resume(struct device *dev)
    {
    struct ucsi_stm32g0 *g0 = dev_get_drvdata(dev);
    struct i2c_client *client = g0.client;
    if (g0.in_bootloader)
    return 0;
    if (device_may_wakeup(dev) || device_wakeup_path(dev))
    disable_irq_wake(client.irq);
    enable_irq(client.irq);
// Enforce any pending handler gets called to signal a wakeup_event
    synchronize_irq(client.irq);
    if (g0.wakeup_event)
    pm_wakeup_event(g0.dev, 0);
    g0.suspended = false;
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ucsi_stm32g0_pm_ops, ucsi_stm32g0_suspend, ucsi_stm32g0_resume);
    static const struct of_device_id __maybe_unused ucsi_stm32g0_typec_of_match[] = {
    { .compatible = "st,stm32g0-typec" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ucsi_stm32g0_typec_of_match);
    static const struct i2c_device_id ucsi_stm32g0_typec_i2c_devid[] = {
    { .name = "stm32g0-typec" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ucsi_stm32g0_typec_i2c_devid);
    static struct i2c_driver ucsi_stm32g0_i2c_driver = {
    .driver = {
    .name = "ucsi-stm32g0-i2c",
    .of_match_table = of_match_ptr(ucsi_stm32g0_typec_of_match),
    .pm = pm_sleep_ptr(&ucsi_stm32g0_pm_ops),
    },
    .probe = ucsi_stm32g0_probe,
    .remove = ucsi_stm32g0_remove,
    .id_table = ucsi_stm32g0_typec_i2c_devid
    };
    module_i2c_driver(ucsi_stm32g0_i2c_driver);
    MODULE_AUTHOR("Fabrice Gasnier <fabrice.gasnier@foss.st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32G0 Type-C controller");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_ALIAS("platform:ucsi-stm32g0");
