//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-taos-evm.c
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
// Driver for the TAOS evaluation modules
// These devices include an I2C controller which can be controlled over the
// serial port.
//
// Copyright (C) 2007 Jean Delvare <jdelvare@suse.de>
//

pub const TAOS_BUFFER_SIZE: c_int = 63;
pub const TAOS_STATE_INIT: c_int = 0;
pub const TAOS_STATE_IDLE: c_int = 1;
pub const TAOS_STATE_EOFF: c_int = 2;
pub const TAOS_STATE_RECV: c_int = 3;
pub const TAOS_CMD_RESET: c_uint = 0x12;

    static DECLARE_WAIT_QUEUE_HEAD(wq);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct taos_data {
    pub adapter: i2c_adapter,
    pub client: *mut i2c_client,
    pub state: c_int,
    pub /: *mut *mut u8 addr; / last used address,
    pub buffer: [c_uchar; TAOS_BUFFER_SIZE],
    pub /: *mut *mut unsigned int pos; / position inside the buffer,
}

// TAOS TSL2550 EVM
    static const struct i2c_board_info tsl2550_info = {
    I2C_BOARD_INFO("tsl2550", 0x39),
    };
// Instantiate i2c devices based on the adapter name
    static struct i2c_client *taos_instantiate_device(struct i2c_adapter *adapter)
    {
    if (!strncmp(adapter.name, "TAOS TSL2550 EVM", 16)) {
    dev_info(&adapter.dev, "Instantiating device %s at 0x%02x\n",
    tsl2550_info.type, tsl2550_info.addr);
    return i2c_new_client_device(adapter, &tsl2550_info);
    }
    return ERR_PTR(-ENODEV);
    }
    static int taos_smbus_xfer(struct i2c_adapter *adapter, u16 addr,
    unsigned short flags, char read_write, u8 command,
    int size, union i2c_smbus_data *data)
    {
    struct serio *serio = adapter.algo_data;
    struct taos_data *taos = serio_get_drvdata(serio);
    char *p;
// Encode our transaction. "@" is for the device address, "$" for the
    SMBus command and "#" for the data. */
    p = taos.buffer;
// The device remembers the last used address, no need to send it
    again if it's the same */
    if (addr != taos.addr)
    p += sprintf(p, "@%02X", addr);
    switch (size) {
    case I2C_SMBUS_BYTE:
    if (read_write == I2C_SMBUS_WRITE)
    sprintf(p, "$#%02X", command);
    else
    sprintf(p, "$");
    break;
    case I2C_SMBUS_BYTE_DATA:
    if (read_write == I2C_SMBUS_WRITE)
    sprintf(p, "$%02X#%02X", command, data.byte);
    else
    sprintf(p, "$%02X", command);
    break;
    default:
    dev_warn(&adapter.dev, "Unsupported transaction %d\n", size);
    return -EOPNOTSUPP;
    }
// Send the transaction to the TAOS EVM
    dev_dbg(&adapter.dev, "Command buffer: %s\n", taos.buffer);
    for (p = taos.buffer; *p; p++)
    serio_write(serio, *p);
    taos.addr = addr;
// Start the transaction and read the answer
    taos.pos = 0;
    taos.state = TAOS_STATE_RECV;
    serio_write(serio, read_write == I2C_SMBUS_WRITE ? '>' : '<');
    wait_event_interruptible_timeout(wq, taos.state == TAOS_STATE_IDLE,
    msecs_to_jiffies(150));
    if (taos.state != TAOS_STATE_IDLE
    || taos.pos != 5) {
    dev_err(&adapter.dev, "Transaction timeout (pos=%d)\n",
    taos.pos);
    return -EIO;
    }
    dev_dbg(&adapter.dev, "Answer buffer: %s\n", taos.buffer);
// Interpret the returned string
    p = taos.buffer + 1;
    p[3] = '\0';
    if (!strcmp(p, "NAK"))
    return -ENODEV;
    if (read_write == I2C_SMBUS_WRITE) {
    if (!strcmp(p, "ACK"))
    return 0;
    } else {
    if (p[0] == 'x') {
//
// Voluntarily dropping error code of kstrtou8 since all
// error code that it could return are invalid according
// to Documentation/i2c/fault-codes.rst.
//
    if (kstrtou8(p + 1, 16, &data.byte))
    return -EPROTO;
    return 0;
    }
    }
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn taos_smbus_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 taos_smbus_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_BYTE_DATA;
    }
    static const struct i2c_algorithm taos_algorithm = {
    .smbus_xfer	= taos_smbus_xfer,
    .functionality	= taos_smbus_func,
    };
    static irqreturn_t taos_interrupt(struct serio *serio, unsigned char data,
    unsigned int flags)
    {
    struct taos_data *taos = serio_get_drvdata(serio);
    switch (taos.state) {
    case TAOS_STATE_INIT:
    taos.buffer[taos.pos++] = data;
    if (data == ':'
    || taos.pos == TAOS_BUFFER_SIZE - 1) {
    taos.buffer[taos.pos] = '\0';
    taos.state = TAOS_STATE_IDLE;
    wake_up_interruptible(&wq);
    }
    break;
    case TAOS_STATE_EOFF:
    taos.state = TAOS_STATE_IDLE;
    wake_up_interruptible(&wq);
    break;
    case TAOS_STATE_RECV:
    taos.buffer[taos.pos++] = data;
    if (data == ']') {
    taos.buffer[taos.pos] = '\0';
    taos.state = TAOS_STATE_IDLE;
    wake_up_interruptible(&wq);
    }
    break;
    }
    return IRQ_HANDLED;
    }
// Extract the adapter name from the buffer received after reset.
    The buffer is modified and a pointer inside the buffer is returned. */
    static char *taos_adapter_name(char *buffer)
    {
    char *start, *end;
    start = strstr(buffer, "TAOS ");
    if (!start)
    return core::ptr::null_mut();
    end = strchr(start, '\r');
    if (!end)
    return core::ptr::null_mut();
// end = '\0';
    return start;
    }
#[no_mangle]
unsafe extern "C" fn taos_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int taos_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct taos_data *taos;
    struct i2c_adapter *adapter;
    char *name;
    int err;
    taos = kzalloc_obj(struct taos_data);
    if (!taos) {
    err = -ENOMEM;
    goto exit;
    }
    taos.state = TAOS_STATE_INIT;
    serio_set_drvdata(serio, taos);
    err = serio_open(serio, drv);
    if (err)
    goto exit_kfree;
    adapter = &taos.adapter;
    adapter.owner = THIS_MODULE;
    adapter.algo = &taos_algorithm;
    adapter.algo_data = serio;
    adapter.dev.parent = &serio.dev;
// Reset the TAOS evaluation module to identify it
    serio_write(serio, TAOS_CMD_RESET);
    wait_event_interruptible_timeout(wq, taos.state == TAOS_STATE_IDLE,
    msecs_to_jiffies(2000));
    if (taos.state != TAOS_STATE_IDLE) {
    err = -ENODEV;
    dev_err(&serio.dev, "TAOS EVM reset failed (state=%d, "
    "pos=%d)\n", taos.state, taos.pos);
    goto exit_close;
    }
    name = taos_adapter_name(taos.buffer);
    if (!name) {
    err = -ENODEV;
    dev_err(&serio.dev, "TAOS EVM identification failed\n");
    goto exit_close;
    }
    strscpy(adapter.name, name, sizeof(adapter.name));
// Turn echo off for better performance
    taos.state = TAOS_STATE_EOFF;
    serio_write(serio, TAOS_CMD_ECHO_OFF);
    wait_event_interruptible_timeout(wq, taos.state == TAOS_STATE_IDLE,
    msecs_to_jiffies(250));
    if (taos.state != TAOS_STATE_IDLE) {
    err = -ENODEV;
    dev_err(&serio.dev, "TAOS EVM echo off failed "
    "(state=%d)\n", taos.state);
    goto exit_close;
    }
    err = i2c_add_adapter(adapter);
    if (err)
    goto exit_close;
    dev_info(&serio.dev, "Connected to TAOS EVM\n");
    taos.client = taos_instantiate_device(adapter);
    return 0;
    exit_close:
    serio_close(serio);
    exit_kfree:
    kfree(taos);
    exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn taos_disconnect(serio: *mut serio) {
    static void taos_disconnect(struct serio *serio)
    {
    struct taos_data *taos = serio_get_drvdata(serio);
    i2c_unregister_device(taos.client);
    i2c_del_adapter(&taos.adapter);
    serio_close(serio);
    kfree(taos);
    dev_info(&serio.dev, "Disconnected from TAOS EVM\n");
    }
    static const struct serio_device_id taos_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_TAOSEVM,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, taos_serio_ids);
    static struct serio_driver taos_drv = {
    .driver		= {
    .name	= "taos-evm",
    },
    .description	= "TAOS evaluation module driver",
    .id_table	= taos_serio_ids,
    .connect	= taos_connect,
    .disconnect	= taos_disconnect,
    .interrupt	= taos_interrupt,
    };
    module_serio_driver(taos_drv);
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("TAOS evaluation module driver");
    MODULE_LICENSE("GPL");
