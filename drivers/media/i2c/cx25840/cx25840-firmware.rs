//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/cx25840/cx25840-firmware.c
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
// cx25840 firmware functions
//

//
// Mike Isely <isely@pobox.com> - The FWSEND parameter controls the
// size of the firmware chunks sent down the I2C bus to the chip.
// Previously this had been set to 1024 but unfortunately some I2C
// implementations can't transfer data in such big gulps.
// Specifically, the pvrusb2 driver has a hard limit of around 60
// bytes, due to the encapsulation there of I2C traffic into USB
// messages.  So we have to significantly reduce this parameter.
//
pub const FWSEND: c_int = 48;

    static char *firmware = "";
    module_param(firmware, charp, 0444);
    MODULE_PARM_DESC(firmware, "Firmware image to load");
#[no_mangle]
unsafe extern "C" fn start_fw_load(client: *mut i2c_client) {
    static void start_fw_load(struct i2c_client *client)
    {
// DL_ADDR_LB=0 DL_ADDR_HB=0
    cx25840_write(client, 0x800, 0x00);
    cx25840_write(client, 0x801, 0x00);
// DL_MAP=3 DL_AUTO_INC=0 DL_ENABLE=1
    cx25840_write(client, 0x803, 0x0b);
// AUTO_INC_DIS=1
    cx25840_write(client, 0x000, 0x20);
    }
#[no_mangle]
unsafe extern "C" fn end_fw_load(client: *mut i2c_client) {
    static void end_fw_load(struct i2c_client *client)
    {
// AUTO_INC_DIS=0
    cx25840_write(client, 0x000, 0x00);
// DL_ENABLE=0
    cx25840_write(client, 0x803, 0x03);
    }

    static const char *get_fw_name(struct i2c_client *client)
    {
    struct cx25840_state *state = to_state(i2c_get_clientdata(client));
    if (firmware[0])
    return firmware;
    if (is_cx2388x(state))
    return CX2388x_FIRMWARE;
    if (is_cx231xx(state))
    return CX231xx_FIRMWARE;
    return CX25840_FIRMWARE;
    }
#[no_mangle]
unsafe extern "C" fn check_fw_load(client: *mut i2c_client, size: c_int) -> c_int {
    static int check_fw_load(struct i2c_client *client, int size)
    {
// DL_ADDR_HB DL_ADDR_LB
    let mut s: c_int = cx25840_read(client, 0x801) << 8;
    s |= cx25840_read(client, 0x800);
    if (size != s) {
    v4l_err(client, "firmware %s load failed\n",
    get_fw_name(client));
    return -EINVAL;
    }
    v4l_info(client, "loaded %s firmware (%d bytes)\n",
    get_fw_name(client), size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fw_write(client: *mut i2c_client, data: *const u8, size: c_int) -> c_int {
    static int fw_write(struct i2c_client *client, const u8 *data, int size)
    {
    if (i2c_master_send(client, data, size) < size) {
    v4l_err(client, "firmware load i2c failure\n");
    return -ENOSYS;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cx25840_loadfw(client: *mut i2c_client) -> c_int {
    int cx25840_loadfw(struct i2c_client *client)
    {
    struct cx25840_state *state = to_state(i2c_get_clientdata(client));
    const struct firmware *fw = core::ptr::null_mut();
    u8 buffer[FWSEND];
    const u8 *ptr;
    const char *fwname = get_fw_name(client);
    int size, retval;
    let mut max_buf_size: c_int = FWSEND;
    let mut gpio_oe: u32 = 0, gpio_da = 0;
    if (is_cx2388x(state)) {
// Preserve the GPIO OE and output bits
    gpio_oe = cx25840_read(client, 0x160);
    gpio_da = cx25840_read(client, 0x164);
    }
// cx231xx cannot accept more than 16 bytes at a time
    if (is_cx231xx(state) && max_buf_size > 16)
    max_buf_size = 16;
    if (request_firmware(&fw, fwname, FWDEV(client)) != 0) {
    v4l_err(client, "unable to open firmware %s\n", fwname);
    return -EINVAL;
    }
    start_fw_load(client);
    buffer[0] = 0x08;
    buffer[1] = 0x02;
    size = fw.size;
    ptr = fw.data;
    while (size > 0) {
    let mut len: c_int = min(max_buf_size - 2, size);
    memcpy(buffer + 2, ptr, len);
    retval = fw_write(client, buffer, len + 2);
    if (retval < 0) {
    release_firmware(fw);
    return retval;
    }
    size -= len;
    ptr += len;
    }
    end_fw_load(client);
    size = fw.size;
    release_firmware(fw);
    if (is_cx2388x(state)) {
// Restore GPIO configuration after f/w load
    cx25840_write(client, 0x160, gpio_oe);
    cx25840_write(client, 0x164, gpio_da);
    }
    return check_fw_load(client, size);
    }
    MODULE_FIRMWARE(CX2388x_FIRMWARE);
    MODULE_FIRMWARE(CX231xx_FIRMWARE);
    MODULE_FIRMWARE(CX25840_FIRMWARE);
