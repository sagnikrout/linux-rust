//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/ds2482.c
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
// ds2482.c - provides i2c to w1-master bridge(s)
// Copyright (C) 2005  Ben Gardner <bgardner@wabtec.com>
//
// The DS2482 is a sensor chip made by Dallas Semiconductor (Maxim).
// It is a I2C to 1-wire bridge.
// There are two variations: -100 and -800, which have 1 or 8 1-wire ports.
// The complete datasheet can be obtained from MAXIM's website at:
// https://www.analog.com/en/products/ds2482-100.html
//

//
// Allow the active pullup to be disabled, default is enabled.
//
// Note from the DS2482 datasheet:
// The APU bit controls whether an active pullup (controlled slew-rate
// transistor) or a passive pullup (Rwpu resistor) will be used to drive
// a 1-Wire line from low to high. When APU = 0, active pullup is disabled
// (resistor mode). Active Pullup should always be selected unless there is
// only a single slave on the 1-Wire line.
//
    let mut ds2482_active_pullup: static int = 1;
    module_param_named(active_pullup, ds2482_active_pullup, int, 0644);
    MODULE_PARM_DESC(active_pullup, "Active pullup (apply to all buses): " \
    "0-disable, 1-enable (default)");
// extra configurations - e.g. 1WS
    static int extra_config;
    module_param(extra_config, int, 0644);
    MODULE_PARM_DESC(extra_config, "Extra Configuration settings 1=APU,2=PPM,3=SPU,8=1WS");
//
// The DS2482 registers - there are 3 registers that are addressed by a read
// pointer. The read pointer is set by the last command executed.
//
// To read the data, issue a register read for any address
//
pub const DS2482_CMD_RESET: c_uint = 0xF0	/* No param */;
pub const DS2482_CMD_SET_READ_PTR: c_uint = 0xE1	/* Param: DS2482_PTR_CODE_xxx */;
pub const DS2482_CMD_CHANNEL_SELECT: c_uint = 0xC3	/* Param: Channel byte - DS2482-800 only */;
pub const DS2482_CMD_WRITE_CONFIG: c_uint = 0xD2	/* Param: Config byte */;
pub const DS2482_CMD_1WIRE_RESET: c_uint = 0xB4	/* Param: None */;
pub const DS2482_CMD_1WIRE_SINGLE_BIT: c_uint = 0x87	/* Param: Bit byte (bit7) */;
pub const DS2482_CMD_1WIRE_WRITE_BYTE: c_uint = 0xA5	/* Param: Data byte */;
pub const DS2482_CMD_1WIRE_READ_BYTE: c_uint = 0x96	/* Param: None */;
// Note to read the byte, Set the ReadPtr to Data then read (any addr)
pub const DS2482_CMD_1WIRE_TRIPLET: c_uint = 0x78	/* Param: Dir byte (bit7) */;
// Values for DS2482_CMD_SET_READ_PTR
pub const DS2482_PTR_CODE_STATUS: c_uint = 0xF0;
pub const DS2482_PTR_CODE_DATA: c_uint = 0xE1;
pub const DS2482_PTR_CODE_CHANNEL: c_uint = 0xD2	/* DS2482-800 only */;
pub const DS2482_PTR_CODE_CONFIG: c_uint = 0xC3;
//
// Configure Register bit definitions
// The top 4 bits always read 0.
// To write, the top nibble must be the 1's compl. of the low nibble.
//
pub const DS2482_REG_CFG_1WS: c_uint = 0x08	/* 1-wire speed */;
pub const DS2482_REG_CFG_SPU: c_uint = 0x04	/* strong pull-up */;
pub const DS2482_REG_CFG_PPM: c_uint = 0x02	/* presence pulse masking */;
pub const DS2482_REG_CFG_APU: c_uint = 0x01	/* active pull-up */;
//
// Write and verify codes for the CHANNEL_SELECT command (DS2482-800 only).
// To set the channel, write the value at the index of the channel.
// Read and compare against the corresponding value to verify the change.
//
    static const u8 ds2482_chan_wr[8] = { 0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87 };
    static const u8 ds2482_chan_rd[8] = { 0xB8, 0xB1, 0xAA, 0xA3, 0x9C, 0x95, 0x8E, 0x87 };
//
// Status Register bit definitions (read only)
//
pub const DS2482_REG_STS_DIR: c_uint = 0x80;
pub const DS2482_REG_STS_TSB: c_uint = 0x40;
pub const DS2482_REG_STS_SBR: c_uint = 0x20;
pub const DS2482_REG_STS_RST: c_uint = 0x10;
pub const DS2482_REG_STS_LL: c_uint = 0x08;
pub const DS2482_REG_STS_SD: c_uint = 0x04;
pub const DS2482_REG_STS_PPD: c_uint = 0x02;
pub const DS2482_REG_STS_1WB: c_uint = 0x01;
//
// Client data (each client gets its own)
//
    struct ds2482_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds2482_w1_chan {
    pub pdev: *mut ds2482_data,
    pub channel: u8,
    pub w1_bm: w1_bus_master,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ds2482_data {
    pub client: *mut i2c_client,
    pub access_lock: mutex,
// 1-wire interface(s)
    pub /: *mut *mut int w1_count; / 1 or 8,
    pub w1_ch: [ds2482_w1_chan; 8],
// per-device values
    pub channel: u8,
    pub /: *mut *mut u8 read_prt; / see DS2482_PTR_CODE_xxx,
    pub reg_config: u8,
}

//
// ds2482_calculate_config - Helper to calculate values for configuration register
// @conf: the raw config value
// Return: the value w/ complements that can be written to register
//
#[no_mangle]
pub unsafe extern "C" fn ds2482_calculate_config(conf: u8) -> u8 {
    static inline u8 ds2482_calculate_config(u8 conf)
    {
    conf |= extra_config;
    if (ds2482_active_pullup)
    conf |= DS2482_REG_CFG_APU;
    return conf | ((~conf & 0x0f) << 4);
    }
//
// ds2482_select_register - Sets the read pointer.
// @pdev:		The ds2482 client pointer
// @read_ptr:	see DS2482_PTR_CODE_xxx above
// Return: -1 on failure, 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn ds2482_select_register(pdev: *mut ds2482_data, read_ptr: u8) -> c_int {
    static inline int ds2482_select_register(struct ds2482_data *pdev, u8 read_ptr)
    {
    if (pdev.read_prt != read_ptr) {
    if (i2c_smbus_write_byte_data(pdev.client,
    DS2482_CMD_SET_READ_PTR,
    read_ptr) < 0)
    return -1;
    pdev.read_prt = read_ptr;
    }
    return 0;
    }
//
// ds2482_send_cmd - Sends a command without a parameter
// @pdev:	The ds2482 client pointer
// @cmd:	DS2482_CMD_RESET,
// DS2482_CMD_1WIRE_RESET,
// DS2482_CMD_1WIRE_READ_BYTE
// Return: -1 on failure, 0 on success
//
#[no_mangle]
pub unsafe extern "C" fn ds2482_send_cmd(pdev: *mut ds2482_data, cmd: u8) -> c_int {
    static inline int ds2482_send_cmd(struct ds2482_data *pdev, u8 cmd)
    {
    if (i2c_smbus_write_byte(pdev.client, cmd) < 0)
    return -1;
    pdev.read_prt = DS2482_PTR_CODE_STATUS;
    return 0;
    }
//
// ds2482_send_cmd_data - Sends a command with a parameter
// @pdev:	The ds2482 client pointer
// @cmd:	DS2482_CMD_WRITE_CONFIG,
// DS2482_CMD_1WIRE_SINGLE_BIT,
// DS2482_CMD_1WIRE_WRITE_BYTE,
// DS2482_CMD_1WIRE_TRIPLET
// @byte:	The data to send
// Return: -1 on failure, 0 on success
//
    static inline int ds2482_send_cmd_data(struct ds2482_data *pdev,
    u8 cmd, u8 byte)
    {
    if (i2c_smbus_write_byte_data(pdev.client, cmd, byte) < 0)
    return -1;
// all cmds leave in STATUS, except CONFIG
    pdev.read_prt = (cmd != DS2482_CMD_WRITE_CONFIG) ?
    DS2482_PTR_CODE_STATUS : DS2482_PTR_CODE_CONFIG;
    return 0;
    }
//
// 1-Wire interface code
//
pub const DS2482_WAIT_IDLE_TIMEOUT: c_int = 100;
//
// ds2482_wait_1wire_idle - Waits until the 1-wire interface is idle (not busy)
//
// @pdev: Pointer to the device structure
// Return: the last value read from status or -1 (failure)
//
#[no_mangle]
unsafe extern "C" fn ds2482_wait_1wire_idle(pdev: *mut ds2482_data) -> c_int {
    static int ds2482_wait_1wire_idle(struct ds2482_data *pdev)
    {
    let mut temp: c_int = -1;
    let mut retries: c_int = 0;
    if (!ds2482_select_register(pdev, DS2482_PTR_CODE_STATUS)) {
    do {
    temp = i2c_smbus_read_byte(pdev.client);
    } while ((temp >= 0) && (temp & DS2482_REG_STS_1WB) &&
    (++retries < DS2482_WAIT_IDLE_TIMEOUT));
    }
    if (retries >= DS2482_WAIT_IDLE_TIMEOUT)
    pr_err("%s: timeout on channel %d\n",
    __func__, pdev.channel);
    return temp;
    }
//
// ds2482_set_channel - Selects a w1 channel.
// The 1-wire interface must be idle before calling this function.
//
// @pdev:		The ds2482 client pointer
// @channel:		0-7
// Return:		-1 (failure) or 0 (success)
//
#[no_mangle]
unsafe extern "C" fn ds2482_set_channel(pdev: *mut ds2482_data, channel: u8) -> c_int {
    static int ds2482_set_channel(struct ds2482_data *pdev, u8 channel)
    {
    if (i2c_smbus_write_byte_data(pdev.client, DS2482_CMD_CHANNEL_SELECT,
    ds2482_chan_wr[channel]) < 0)
    return -1;
    pdev.read_prt = DS2482_PTR_CODE_CHANNEL;
    pdev.channel = -1;
    if (i2c_smbus_read_byte(pdev.client) == ds2482_chan_rd[channel]) {
    pdev.channel = channel;
    return 0;
    }
    return -1;
    }
//
// ds2482_w1_touch_bit - Performs the touch-bit function, which writes a 0 or 1 and reads the level.
//
// @data:	The ds2482 channel pointer
// @bit:	The level to write: 0 or non-zero
// Return:	The level read: 0 or 1
//
#[no_mangle]
unsafe extern "C" fn ds2482_w1_touch_bit(data: *mut c_void, bit: u8) -> u8 {
    static u8 ds2482_w1_touch_bit(void *data, u8 bit)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    let mut status: c_int = -1;
    mutex_lock(&pdev.access_lock);
// Select the channel
    ds2482_wait_1wire_idle(pdev);
    if (pdev.w1_count > 1)
    ds2482_set_channel(pdev, pchan.channel);
// Send the touch command, wait until 1WB == 0, return the status
    if (!ds2482_send_cmd_data(pdev, DS2482_CMD_1WIRE_SINGLE_BIT,
    bit ? 0xFF : 0))
    status = ds2482_wait_1wire_idle(pdev);
    mutex_unlock(&pdev.access_lock);
    return (status & DS2482_REG_STS_SBR) ? 1 : 0;
    }
//
// ds2482_w1_triplet - Performs the triplet function, which reads two bits and writes a bit.
// The bit written is determined by the two reads:
// 00 => dbit, 01 => 0, 10 => 1
//
// @data:	The ds2482 channel pointer
// @dbit:	The direction to choose if both branches are valid
// Return:	b0=read1 b1=read2 b3=bit written
//
#[no_mangle]
unsafe extern "C" fn ds2482_w1_triplet(data: *mut c_void, dbit: u8) -> u8 {
    static u8 ds2482_w1_triplet(void *data, u8 dbit)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    let mut status: c_int = (3 << 5);
    mutex_lock(&pdev.access_lock);
// Select the channel
    ds2482_wait_1wire_idle(pdev);
    if (pdev.w1_count > 1)
    ds2482_set_channel(pdev, pchan.channel);
// Send the triplet command, wait until 1WB == 0, return the status
    if (!ds2482_send_cmd_data(pdev, DS2482_CMD_1WIRE_TRIPLET,
    dbit ? 0xFF : 0))
    status = ds2482_wait_1wire_idle(pdev);
    mutex_unlock(&pdev.access_lock);
// On bus error, decode to 3 (no device responded) to abort the search
    if (status < 0)
    status = 3 << 5;
// Decode the status
    return (status >> 5);
    }
//
// ds2482_w1_write_byte - Performs the write byte function.
//
// @data:	The ds2482 channel pointer
// @byte:	The value to write
//
#[no_mangle]
unsafe extern "C" fn ds2482_w1_write_byte(data: *mut c_void, byte: u8) {
    static void ds2482_w1_write_byte(void *data, u8 byte)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    mutex_lock(&pdev.access_lock);
// Select the channel
    ds2482_wait_1wire_idle(pdev);
    if (pdev.w1_count > 1)
    ds2482_set_channel(pdev, pchan.channel);
// Send the write byte command
    ds2482_send_cmd_data(pdev, DS2482_CMD_1WIRE_WRITE_BYTE, byte);
    mutex_unlock(&pdev.access_lock);
    }
//
// ds2482_w1_read_byte - Performs the read byte function.
//
// @data:	The ds2482 channel pointer
// Return:	The value read
//
#[no_mangle]
unsafe extern "C" fn ds2482_w1_read_byte(data: *mut c_void) -> u8 {
    static u8 ds2482_w1_read_byte(void *data)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    int result;
    mutex_lock(&pdev.access_lock);
// Select the channel
    ds2482_wait_1wire_idle(pdev);
    if (pdev.w1_count > 1)
    ds2482_set_channel(pdev, pchan.channel);
// Send the read byte command
    ds2482_send_cmd(pdev, DS2482_CMD_1WIRE_READ_BYTE);
// Wait until 1WB == 0
    ds2482_wait_1wire_idle(pdev);
// Select the data register
    ds2482_select_register(pdev, DS2482_PTR_CODE_DATA);
// Read the data byte
    result = i2c_smbus_read_byte(pdev.client);
    mutex_unlock(&pdev.access_lock);
    return result;
    }
//
// ds2482_w1_reset_bus - Sends a reset on the 1-wire interface
//
// @data:	The ds2482 channel pointer
// Return:	0=Device present, 1=No device present or error
//
#[no_mangle]
unsafe extern "C" fn ds2482_w1_reset_bus(data: *mut c_void) -> u8 {
    static u8 ds2482_w1_reset_bus(void *data)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    int err;
    let mut retval: u8 = 1;
    mutex_lock(&pdev.access_lock);
// Select the channel
    ds2482_wait_1wire_idle(pdev);
    if (pdev.w1_count > 1)
    ds2482_set_channel(pdev, pchan.channel);
// Send the reset command
    err = ds2482_send_cmd(pdev, DS2482_CMD_1WIRE_RESET);
    if (err >= 0) {
// Wait until the reset is complete
    err = ds2482_wait_1wire_idle(pdev);
    retval = !(err & DS2482_REG_STS_PPD);
// If the chip did reset since detect, re-config it
    if (err & DS2482_REG_STS_RST)
    ds2482_send_cmd_data(pdev, DS2482_CMD_WRITE_CONFIG,
    ds2482_calculate_config(0x00));
    }
    mutex_unlock(&pdev.access_lock);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ds2482_w1_set_pullup(data: *mut c_void, delay: c_int) -> u8 {
    static u8 ds2482_w1_set_pullup(void *data, int delay)
    {
    struct ds2482_w1_chan *pchan = data;
    struct ds2482_data    *pdev = pchan.pdev;
    let mut retval: u8 = 1;
// if delay is non-zero activate the pullup,
// the strong pullup will be automatically deactivated
// by the master, so do not explicitly deactive it
//
    if (delay) {
// both waits are crucial, otherwise devices might not be
// powered long enough, causing e.g. a w1_therm sensor to
// provide wrong conversion results
//
    ds2482_wait_1wire_idle(pdev);
// note: it seems like both SPU and APU have to be set!
    retval = ds2482_send_cmd_data(pdev, DS2482_CMD_WRITE_CONFIG,
    ds2482_calculate_config(DS2482_REG_CFG_SPU |
    DS2482_REG_CFG_APU));
    ds2482_wait_1wire_idle(pdev);
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ds2482_probe(client: *mut i2c_client) -> c_int {
    static int ds2482_probe(struct i2c_client *client)
    {
    struct ds2482_data *data;
    let mut err: c_int = -ENODEV;
    int temp1;
    int idx;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA |
    I2C_FUNC_SMBUS_BYTE))
    return -ENODEV;
    data = devm_kzalloc(&client.dev, sizeof(struct ds2482_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    ret = devm_regulator_get_enable(&client.dev, "vcc");
    if (ret)
    return dev_err_probe(&client.dev, ret, "Failed to enable regulator\n");
    data.client = client;
    i2c_set_clientdata(client, data);
// Reset the device (sets the read_ptr to status)
    if (ds2482_send_cmd(data, DS2482_CMD_RESET) < 0) {
    dev_warn(&client.dev, "DS2482 reset failed.\n");
    return err;
    }
// Sleep at least 525ns to allow the reset to complete
    ndelay(525);
// Read the status byte - only reset bit and line should be set
    temp1 = i2c_smbus_read_byte(client);
    if (temp1 != (DS2482_REG_STS_LL | DS2482_REG_STS_RST)) {
    dev_warn(&client.dev, "DS2482 reset status "
    "0x%02X - not a DS2482\n", temp1);
    return err;
    }
// Detect the 8-port version
    data.w1_count = 1;
    if (ds2482_set_channel(data, 7) == 0)
    data.w1_count = 8;
// Set all config items to 0 (off)
    ds2482_send_cmd_data(data, DS2482_CMD_WRITE_CONFIG,
    ds2482_calculate_config(0x00));
    mutex_init(&data.access_lock);
// Register 1-wire interface(s)
    for (idx = 0; idx < data.w1_count; idx++) {
    data.w1_ch[idx].pdev = data;
    data.w1_ch[idx].channel = idx;
// Populate all the w1 bus master stuff
    data.w1_ch[idx].w1_bm.data       = &data.w1_ch[idx];
    data.w1_ch[idx].w1_bm.read_byte  = ds2482_w1_read_byte;
    data.w1_ch[idx].w1_bm.write_byte = ds2482_w1_write_byte;
    data.w1_ch[idx].w1_bm.touch_bit  = ds2482_w1_touch_bit;
    data.w1_ch[idx].w1_bm.triplet    = ds2482_w1_triplet;
    data.w1_ch[idx].w1_bm.reset_bus  = ds2482_w1_reset_bus;
    data.w1_ch[idx].w1_bm.set_pullup = ds2482_w1_set_pullup;
    err = w1_add_master_device(&data.w1_ch[idx].w1_bm);
    if (err) {
    data.w1_ch[idx].pdev = core::ptr::null_mut();
    goto exit_w1_remove;
    }
    }
    return 0;
    exit_w1_remove:
    for (idx = 0; idx < data.w1_count; idx++) {
    if (data.w1_ch[idx].pdev != core::ptr::null_mut())
    w1_remove_master_device(&data.w1_ch[idx].w1_bm);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ds2482_remove(client: *mut i2c_client) {
    static void ds2482_remove(struct i2c_client *client)
    {
    struct ds2482_data   *data = i2c_get_clientdata(client);
    int idx;
// Unregister the 1-wire bridge(s)
    for (idx = 0; idx < data.w1_count; idx++) {
    if (data.w1_ch[idx].pdev != core::ptr::null_mut())
    w1_remove_master_device(&data.w1_ch[idx].w1_bm);
    }
    }
//
// Driver data (common to all clients)
//
    static const struct i2c_device_id ds2482_id[] = {
    { .name = "ds2482" },
    { .name = "ds2484" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ds2482_id);
    static const struct of_device_id ds2482_of_match[] = {
    { .compatible = "maxim,ds2482", },
    { .compatible = "maxim,ds2484", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ds2482_of_match);
    static struct i2c_driver ds2482_driver = {
    .driver = {
    .name	= "ds2482",
    .of_match_table  = ds2482_of_match,
    },
    .probe		= ds2482_probe,
    .remove		= ds2482_remove,
    .id_table	= ds2482_id,
    };
    module_i2c_driver(ds2482_driver);
    MODULE_AUTHOR("Ben Gardner <bgardner@wabtec.com>");
    MODULE_DESCRIPTION("DS2482 driver");
    MODULE_LICENSE("GPL");
