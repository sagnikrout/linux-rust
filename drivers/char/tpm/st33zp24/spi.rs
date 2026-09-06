//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/st33zp24/spi.c
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
// STMicroelectronics TPM SPI Linux driver for TPM ST33ZP24
// Copyright (C) 2009 - 2016 STMicroelectronics
//

pub const TPM_DATA_FIFO: c_uint = 0x24;
pub const TPM_INTF_CAPABILITY: c_uint = 0x14;
pub const TPM_DUMMY_BYTE: c_uint = 0x00;
pub const MAX_SPI_LATENCY: c_int = 15;
pub const LOCALITY0: c_int = 0;
pub const ST33ZP24_OK: c_uint = 0x5A;
pub const ST33ZP24_UNDEFINED_ERR: c_uint = 0x80;
pub const ST33ZP24_BADLOCALITY: c_uint = 0x81;
pub const ST33ZP24_TISREGISTER_UNKNOWN: c_uint = 0x82;
pub const ST33ZP24_LOCALITY_NOT_ACTIVATED: c_uint = 0x83;
pub const ST33ZP24_HASH_END_BEFORE_HASH_START: c_uint = 0x84;
pub const ST33ZP24_BAD_COMMAND_ORDER: c_uint = 0x85;
pub const ST33ZP24_INCORECT_RECEIVED_LENGTH: c_uint = 0x86;
pub const ST33ZP24_TPM_FIFO_OVERFLOW: c_uint = 0x89;
pub const ST33ZP24_UNEXPECTED_READ_FIFO: c_uint = 0x8A;
pub const ST33ZP24_UNEXPECTED_WRITE_FIFO: c_uint = 0x8B;
pub const ST33ZP24_CMDRDY_SET_WHEN_PROCESSING_HASH_END: c_uint = 0x90;
pub const ST33ZP24_DUMMY_BYTES: c_uint = 0x00;
//
// TPM command can be up to 2048 byte, A TPM response can be up to
// 1024 byte.
// Between command and response, there are latency byte (up to 15
// usually on st33zp24 2 are enough).
//
// Overall when sending a command and expecting an answer we need if
// worst case:
// 2048 (for the TPM command) + 1024 (for the TPM answer).  We need
// some latency byte before the answer is available (max 15).
// We have 2048 + 1024 + 15.
//

    MAX_SPI_LATENCY)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st33zp24_spi_phy {
    pub spi_device: *mut spi_device,
    pub tx_buf: [u8; ST33ZP24_SPI_BUFFER_SIZE],
    pub rx_buf: [u8; ST33ZP24_SPI_BUFFER_SIZE],
    pub latency: c_int,
}

#[no_mangle]
unsafe extern "C" fn st33zp24_status_to_errno(code: u8) -> c_int {
    static int st33zp24_status_to_errno(u8 code)
    {
    switch (code) {
    case ST33ZP24_OK:
    return 0;
    case ST33ZP24_UNDEFINED_ERR:
    case ST33ZP24_BADLOCALITY:
    case ST33ZP24_TISREGISTER_UNKNOWN:
    case ST33ZP24_LOCALITY_NOT_ACTIVATED:
    case ST33ZP24_HASH_END_BEFORE_HASH_START:
    case ST33ZP24_BAD_COMMAND_ORDER:
    case ST33ZP24_UNEXPECTED_READ_FIFO:
    case ST33ZP24_UNEXPECTED_WRITE_FIFO:
    case ST33ZP24_CMDRDY_SET_WHEN_PROCESSING_HASH_END:
    return -EPROTO;
    case ST33ZP24_INCORECT_RECEIVED_LENGTH:
    case ST33ZP24_TPM_FIFO_OVERFLOW:
    return -EMSGSIZE;
    case ST33ZP24_DUMMY_BYTES:
    return -ENOSYS;
    }
    return code;
    }
//
// st33zp24_spi_send
// Send byte to the TIS register according to the ST33ZP24 SPI protocol.
// @param: phy_id, the phy description
// @param: tpm_register, the tpm tis register where the data should be written
// @param: tpm_data, the tpm_data to write inside the tpm_register
// @param: tpm_size, The length of the data
// @return: should be zero if success else a negative error code.
//
    static int st33zp24_spi_send(void *phy_id, u8 tpm_register, u8 *tpm_data,
    int tpm_size)
    {
    let mut total_length: c_int = 0, ret = 0;
    struct st33zp24_spi_phy *phy = phy_id;
    struct spi_device *dev = phy.spi_device;
    struct spi_transfer spi_xfer = {
    .tx_buf = phy.tx_buf,
    .rx_buf = phy.rx_buf,
    };
// Pre-Header
    phy.tx_buf[total_length++] = TPM_WRITE_DIRECTION | LOCALITY0;
    phy.tx_buf[total_length++] = tpm_register;
    if (tpm_size > 0 && tpm_register == TPM_DATA_FIFO) {
    phy.tx_buf[total_length++] = tpm_size >> 8;
    phy.tx_buf[total_length++] = tpm_size;
    }
    memcpy(&phy.tx_buf[total_length], tpm_data, tpm_size);
    total_length += tpm_size;
    memset(&phy.tx_buf[total_length], TPM_DUMMY_BYTE, phy.latency);
    spi_xfer.len = total_length + phy.latency;
    ret = spi_sync_transfer(dev, &spi_xfer, 1);
    if (ret == 0)
    ret = phy.rx_buf[total_length + phy.latency - 1];
    return st33zp24_status_to_errno(ret);
    } /* st33zp24_spi_send() */
//
// st33zp24_spi_read8_recv
// Recv byte from the TIS register according to the ST33ZP24 SPI protocol.
// @param: phy_id, the phy description
// @param: tpm_register, the tpm tis register where the data should be read
// @param: tpm_data, the TPM response
// @param: tpm_size, tpm TPM response size to read.
// @return: should be zero if success else a negative error code.
//
    static int st33zp24_spi_read8_reg(void *phy_id, u8 tpm_register, u8 *tpm_data,
    int tpm_size)
    {
    let mut total_length: c_int = 0, ret;
    struct st33zp24_spi_phy *phy = phy_id;
    struct spi_device *dev = phy.spi_device;
    struct spi_transfer spi_xfer = {
    .tx_buf = phy.tx_buf,
    .rx_buf = phy.rx_buf,
    };
// Pre-Header
    phy.tx_buf[total_length++] = LOCALITY0;
    phy.tx_buf[total_length++] = tpm_register;
    memset(&phy.tx_buf[total_length], TPM_DUMMY_BYTE,
    phy.latency + tpm_size);
    spi_xfer.len = total_length + phy.latency + tpm_size;
// header + status byte + size of the data + status byte
    ret = spi_sync_transfer(dev, &spi_xfer, 1);
    if (tpm_size > 0 && ret == 0) {
    ret = phy.rx_buf[total_length + phy.latency - 1];
    memcpy(tpm_data, phy.rx_buf + total_length + phy.latency,
    tpm_size);
    }
    return ret;
    } /* st33zp24_spi_read8_reg() */
//
// st33zp24_spi_recv
// Recv byte from the TIS register according to the ST33ZP24 SPI protocol.
// @param: phy_id, the phy description
// @param: tpm_register, the tpm tis register where the data should be read
// @param: tpm_data, the TPM response
// @param: tpm_size, tpm TPM response size to read.
// @return: number of byte read successfully: should be one if success.
//
    static int st33zp24_spi_recv(void *phy_id, u8 tpm_register, u8 *tpm_data,
    int tpm_size)
    {
    int ret;
    ret = st33zp24_spi_read8_reg(phy_id, tpm_register, tpm_data, tpm_size);
    if (!st33zp24_status_to_errno(ret))
    return tpm_size;
    return ret;
    } /* st33zp24_spi_recv() */
#[no_mangle]
unsafe extern "C" fn st33zp24_spi_evaluate_latency(phy_id: *mut c_void) -> c_int {
    static int st33zp24_spi_evaluate_latency(void *phy_id)
    {
    struct st33zp24_spi_phy *phy = phy_id;
    let mut latency: c_int = 1, status = 0;
    let mut data: u8 = 0;
    while (!status && latency < MAX_SPI_LATENCY) {
    phy.latency = latency;
    status = st33zp24_spi_read8_reg(phy_id, TPM_INTF_CAPABILITY,
    &data, 1);
    latency++;
    }
    if (status < 0)
    return status;
    if (latency == MAX_SPI_LATENCY)
    return -ENODEV;
    return latency - 1;
    } /* evaluate_latency() */
    static const struct st33zp24_phy_ops spi_phy_ops = {
    .send = st33zp24_spi_send,
    .recv = st33zp24_spi_recv,
    };
//
// st33zp24_spi_probe initialize the TPM device
// @param: dev, the spi_device description (TPM SPI description).
// @return: 0 in case of success.
// or a negative value describing the error.
//
#[no_mangle]
unsafe extern "C" fn st33zp24_spi_probe(dev: *mut spi_device) -> c_int {
    static int st33zp24_spi_probe(struct spi_device *dev)
    {
    struct st33zp24_spi_phy *phy;
    phy = devm_kzalloc(&dev.dev, sizeof(struct st33zp24_spi_phy),
    GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.spi_device = dev;
    phy.latency = st33zp24_spi_evaluate_latency(phy);
    if (phy.latency <= 0)
    return -ENODEV;
    return st33zp24_probe(phy, &spi_phy_ops, &dev.dev, dev.irq);
    }
//
// st33zp24_spi_remove remove the TPM device
// @param: client, the spi_device description (TPM SPI description).
// @return: 0 in case of success.
//
#[no_mangle]
unsafe extern "C" fn st33zp24_spi_remove(dev: *mut spi_device) {
    static void st33zp24_spi_remove(struct spi_device *dev)
    {
    struct tpm_chip *chip = spi_get_drvdata(dev);
    st33zp24_remove(chip);
    }
    static const struct spi_device_id st33zp24_spi_id[] = {
    {TPM_ST33_SPI, 0},
    {}
    };
    MODULE_DEVICE_TABLE(spi, st33zp24_spi_id);
    static const struct of_device_id of_st33zp24_spi_match[] __maybe_unused = {
    { .compatible = "st,st33zp24-spi", },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_st33zp24_spi_match);
    static const struct acpi_device_id st33zp24_spi_acpi_match[] __maybe_unused = {
    {"SMO3324"},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, st33zp24_spi_acpi_match);
    static SIMPLE_DEV_PM_OPS(st33zp24_spi_ops, st33zp24_pm_suspend,
    st33zp24_pm_resume);
    static struct spi_driver st33zp24_spi_driver = {
    .driver = {
    .name = "st33zp24-spi",
    .pm = &st33zp24_spi_ops,
    .of_match_table = of_match_ptr(of_st33zp24_spi_match),
    .acpi_match_table = ACPI_PTR(st33zp24_spi_acpi_match),
    },
    .probe = st33zp24_spi_probe,
    .remove = st33zp24_spi_remove,
    .id_table = st33zp24_spi_id,
    };
    module_spi_driver(st33zp24_spi_driver);
    MODULE_AUTHOR("TPM support <TPMsupport@list.st.com>");
    MODULE_DESCRIPTION("STM TPM 1.2 SPI ST33 Driver");
    MODULE_VERSION("1.3.0");
    MODULE_LICENSE("GPL");
