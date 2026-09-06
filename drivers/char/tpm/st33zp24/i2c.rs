//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/st33zp24/i2c.c
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
// STMicroelectronics TPM I2C Linux driver for TPM ST33ZP24
// Copyright (C) 2009 - 2016 STMicroelectronics
//

pub const TPM_DUMMY_BYTE: c_uint = 0xAA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st33zp24_i2c_phy {
    pub client: *mut i2c_client,
    pub 1]: u8 buf[ST33ZP24_BUFSIZE +,
}

//
// write8_reg
// Send byte to the TIS register according to the ST33ZP24 I2C protocol.
// @param: tpm_register, the tpm tis register where the data should be written
// @param: tpm_data, the tpm_data to write inside the tpm_register
// @param: tpm_size, The length of the data
// @return: Returns negative errno, or else the number of bytes written.
//
#[no_mangle]
unsafe extern "C" fn write8_reg(phy_id: *mut c_void, tpm_register: u8, tpm_data: *mut u8, tpm_size: c_int) -> c_int {
    static int write8_reg(void *phy_id, u8 tpm_register, u8 *tpm_data, int tpm_size)
    {
    struct st33zp24_i2c_phy *phy = phy_id;
    phy.buf[0] = tpm_register;
    memcpy(phy.buf + 1, tpm_data, tpm_size);
    return i2c_master_send(phy.client, phy.buf, tpm_size + 1);
    } /* write8_reg() */
//
// read8_reg
// Recv byte from the TIS register according to the ST33ZP24 I2C protocol.
// @param: tpm_register, the tpm tis register where the data should be read
// @param: tpm_data, the TPM response
// @param: tpm_size, tpm TPM response size to read.
// @return: number of byte read successfully: should be one if success.
//
#[no_mangle]
unsafe extern "C" fn read8_reg(phy_id: *mut c_void, tpm_register: u8, tpm_data: *mut u8, tpm_size: c_int) -> c_int {
    static int read8_reg(void *phy_id, u8 tpm_register, u8 *tpm_data, int tpm_size)
    {
    struct st33zp24_i2c_phy *phy = phy_id;
    let mut status: u8 = 0;
    u8 data;
    data = TPM_DUMMY_BYTE;
    status = write8_reg(phy, tpm_register, &data, 1);
    if (status == 2)
    status = i2c_master_recv(phy.client, tpm_data, tpm_size);
    return status;
    } /* read8_reg() */
//
// st33zp24_i2c_send
// Send byte to the TIS register according to the ST33ZP24 I2C protocol.
// @param: phy_id, the phy description
// @param: tpm_register, the tpm tis register where the data should be written
// @param: tpm_data, the tpm_data to write inside the tpm_register
// @param: tpm_size, the length of the data
// @return: number of byte written successfully: should be one if success.
//
    static int st33zp24_i2c_send(void *phy_id, u8 tpm_register, u8 *tpm_data,
    int tpm_size)
    {
    return write8_reg(phy_id, tpm_register | TPM_WRITE_DIRECTION, tpm_data,
    tpm_size);
    }
//
// st33zp24_i2c_recv
// Recv byte from the TIS register according to the ST33ZP24 I2C protocol.
// @param: phy_id, the phy description
// @param: tpm_register, the tpm tis register where the data should be read
// @param: tpm_data, the TPM response
// @param: tpm_size, tpm TPM response size to read.
// @return: number of byte read successfully: should be one if success.
//
    static int st33zp24_i2c_recv(void *phy_id, u8 tpm_register, u8 *tpm_data,
    int tpm_size)
    {
    return read8_reg(phy_id, tpm_register, tpm_data, tpm_size);
    }
    static const struct st33zp24_phy_ops i2c_phy_ops = {
    .send = st33zp24_i2c_send,
    .recv = st33zp24_i2c_recv,
    };
//
// st33zp24_i2c_probe initialize the TPM device
// @param: client, the i2c_client description (TPM I2C description).
// @param: id, the i2c_device_id struct.
// @return: 0 in case of success.
// -1 in other case.
//
#[no_mangle]
unsafe extern "C" fn st33zp24_i2c_probe(client: *mut i2c_client) -> c_int {
    static int st33zp24_i2c_probe(struct i2c_client *client)
    {
    struct st33zp24_i2c_phy *phy;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_info(&client.dev, "client not i2c capable\n");
    return -ENODEV;
    }
    phy = devm_kzalloc(&client.dev, sizeof(struct st33zp24_i2c_phy),
    GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.client = client;
    return st33zp24_probe(phy, &i2c_phy_ops, &client.dev, client.irq);
    }
//
// st33zp24_i2c_remove remove the TPM device
// @param: client, the i2c_client description (TPM I2C description).
// @return: 0 in case of success.
//
#[no_mangle]
unsafe extern "C" fn st33zp24_i2c_remove(client: *mut i2c_client) {
    static void st33zp24_i2c_remove(struct i2c_client *client)
    {
    struct tpm_chip *chip = i2c_get_clientdata(client);
    st33zp24_remove(chip);
    }
    static const struct i2c_device_id st33zp24_i2c_id[] = {
    { TPM_ST33_I2C },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, st33zp24_i2c_id);
    static const struct of_device_id of_st33zp24_i2c_match[] __maybe_unused = {
    { .compatible = "st,st33zp24-i2c", },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_st33zp24_i2c_match);
    static const struct acpi_device_id st33zp24_i2c_acpi_match[] __maybe_unused = {
    {"SMO3324"},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, st33zp24_i2c_acpi_match);
    static SIMPLE_DEV_PM_OPS(st33zp24_i2c_ops, st33zp24_pm_suspend,
    st33zp24_pm_resume);
    static struct i2c_driver st33zp24_i2c_driver = {
    .driver = {
    .name = TPM_ST33_I2C,
    .pm = &st33zp24_i2c_ops,
    .of_match_table = of_match_ptr(of_st33zp24_i2c_match),
    .acpi_match_table = ACPI_PTR(st33zp24_i2c_acpi_match),
    },
    .probe = st33zp24_i2c_probe,
    .remove = st33zp24_i2c_remove,
    .id_table = st33zp24_i2c_id
    };
    module_i2c_driver(st33zp24_i2c_driver);
    MODULE_AUTHOR("TPM support <TPMsupport@list.st.com>");
    MODULE_DESCRIPTION("STM TPM 1.2 I2C ST33 Driver");
    MODULE_VERSION("1.3.0");
    MODULE_LICENSE("GPL");
