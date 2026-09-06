//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-ali1563.c
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
// i2c-ali1563.c - i2c driver for the ALi 1563 Southbridge
//
// Copyright (C) 2004 Patrick Mochel
// 2005 Rudolf Marek <r.marek@assembler.cz>
//
// The 1563 southbridge is deceptively similar to the 1533, with a
// few notable exceptions. One of those happens to be the fact they
// upgraded the i2c core to be 2.0 compliant, and happens to be almost
// identical to the i2c controller found in the Intel 801 south
// bridges.
//
// This driver is based on a mix of the 15x3, 1535, and i801 drivers,
// with a little help from the ALi 1563 spec.
//

pub const ALI1563_MAX_TIMEOUT: c_int = 500;
pub const ALI1563_SMBBA: c_uint = 0x80;
pub const ALI1563_SMB_IOEN: c_int = 1;
pub const ALI1563_SMB_HOSTEN: c_int = 2;
pub const ALI1563_SMB_IOSIZE: c_int = 16;

pub const HST_STS_BUSY: c_uint = 0x01;
pub const HST_STS_INTR: c_uint = 0x02;
pub const HST_STS_DEVERR: c_uint = 0x04;
pub const HST_STS_BUSERR: c_uint = 0x08;
pub const HST_STS_FAIL: c_uint = 0x10;
pub const HST_STS_DONE: c_uint = 0x80;
pub const HST_STS_BAD: c_uint = 0x1c;
pub const HST_CNTL1_TIMEOUT: c_uint = 0x80;
pub const HST_CNTL1_LAST: c_uint = 0x40;
pub const HST_CNTL2_KILL: c_uint = 0x04;
pub const HST_CNTL2_START: c_uint = 0x40;
pub const HST_CNTL2_QUICK: c_uint = 0x00;
pub const HST_CNTL2_BYTE: c_uint = 0x01;
pub const HST_CNTL2_BYTE_DATA: c_uint = 0x02;
pub const HST_CNTL2_WORD_DATA: c_uint = 0x03;
pub const HST_CNTL2_BLOCK: c_uint = 0x05;
pub const HST_CNTL2_SIZEMASK: c_uint = 0x38;
    static struct pci_driver ali1563_pci_driver;
    static unsigned short ali1563_smba;
#[no_mangle]
unsafe extern "C" fn ali1563_transaction(a: *mut i2c_adapter, size: c_int) -> c_int {
    static int ali1563_transaction(struct i2c_adapter *a, int size)
    {
    u32 data;
    int timeout;
    let mut status: c_int = -EIO;
    dev_dbg(&a.dev, "Transaction (pre): STS=%02x, CNTL1=%02x, "
    "CNTL2=%02x, CMD=%02x, ADD=%02x, DAT0=%02x, DAT1=%02x\n",
    inb_p(SMB_HST_STS), inb_p(SMB_HST_CNTL1), inb_p(SMB_HST_CNTL2),
    inb_p(SMB_HST_CMD), inb_p(SMB_HST_ADD), inb_p(SMB_HST_DAT0),
    inb_p(SMB_HST_DAT1));
    data = inb_p(SMB_HST_STS);
    if (data & HST_STS_BAD) {
    dev_err(&a.dev, "ali1563: Trying to reset busy device\n");
    outb_p(data | HST_STS_BAD, SMB_HST_STS);
    data = inb_p(SMB_HST_STS);
    if (data & HST_STS_BAD)
    return -EBUSY;
    }
    outb_p(inb_p(SMB_HST_CNTL2) | HST_CNTL2_START, SMB_HST_CNTL2);
    timeout = ALI1563_MAX_TIMEOUT;
    do {
    msleep(1);
    } while (((data = inb_p(SMB_HST_STS)) & HST_STS_BUSY) && --timeout);
    dev_dbg(&a.dev, "Transaction (post): STS=%02x, CNTL1=%02x, "
    "CNTL2=%02x, CMD=%02x, ADD=%02x, DAT0=%02x, DAT1=%02x\n",
    inb_p(SMB_HST_STS), inb_p(SMB_HST_CNTL1), inb_p(SMB_HST_CNTL2),
    inb_p(SMB_HST_CMD), inb_p(SMB_HST_ADD), inb_p(SMB_HST_DAT0),
    inb_p(SMB_HST_DAT1));
    if (timeout && !(data & HST_STS_BAD))
    return 0;
    if (!timeout) {
// Issue 'kill' to host controller
    outb_p(HST_CNTL2_KILL, SMB_HST_CNTL2);
    data = inb_p(SMB_HST_STS);
    status = -ETIMEDOUT;
    }
// device error - no response, ignore the autodetection case
    if (data & HST_STS_DEVERR) {
    if (size != HST_CNTL2_QUICK)
    dev_err(&a.dev, "Device error!\n");
    status = -ENXIO;
    }
// bus collision
    if (data & HST_STS_BUSERR) {
    dev_err(&a.dev, "Bus collision!\n");
// Issue timeout, hoping it helps
    outb_p(HST_CNTL1_TIMEOUT, SMB_HST_CNTL1);
    }
    if (data & HST_STS_FAIL) {
    dev_err(&a.dev, "Cleaning fail after KILL!\n");
    outb_p(0x0, SMB_HST_CNTL2);
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ali1563_block_start(a: *mut i2c_adapter) -> c_int {
    static int ali1563_block_start(struct i2c_adapter *a)
    {
    u32 data;
    int timeout;
    let mut status: c_int = -EIO;
    dev_dbg(&a.dev, "Block (pre): STS=%02x, CNTL1=%02x, "
    "CNTL2=%02x, CMD=%02x, ADD=%02x, DAT0=%02x, DAT1=%02x\n",
    inb_p(SMB_HST_STS), inb_p(SMB_HST_CNTL1), inb_p(SMB_HST_CNTL2),
    inb_p(SMB_HST_CMD), inb_p(SMB_HST_ADD), inb_p(SMB_HST_DAT0),
    inb_p(SMB_HST_DAT1));
    data = inb_p(SMB_HST_STS);
    if (data & HST_STS_BAD) {
    dev_warn(&a.dev, "ali1563: Trying to reset busy device\n");
    outb_p(data | HST_STS_BAD, SMB_HST_STS);
    data = inb_p(SMB_HST_STS);
    if (data & HST_STS_BAD)
    return -EBUSY;
    }
// Clear byte-ready bit
    outb_p(data | HST_STS_DONE, SMB_HST_STS);
// Start transaction and wait for byte-ready bit to be set
    outb_p(inb_p(SMB_HST_CNTL2) | HST_CNTL2_START, SMB_HST_CNTL2);
    timeout = ALI1563_MAX_TIMEOUT;
    do {
    msleep(1);
    } while (!((data = inb_p(SMB_HST_STS)) & HST_STS_DONE) && --timeout);
    dev_dbg(&a.dev, "Block (post): STS=%02x, CNTL1=%02x, "
    "CNTL2=%02x, CMD=%02x, ADD=%02x, DAT0=%02x, DAT1=%02x\n",
    inb_p(SMB_HST_STS), inb_p(SMB_HST_CNTL1), inb_p(SMB_HST_CNTL2),
    inb_p(SMB_HST_CMD), inb_p(SMB_HST_ADD), inb_p(SMB_HST_DAT0),
    inb_p(SMB_HST_DAT1));
    if (timeout && !(data & HST_STS_BAD))
    return 0;
    if (timeout == 0)
    status = -ETIMEDOUT;
    if (data & HST_STS_DEVERR)
    status = -ENXIO;
    dev_err(&a.dev, "SMBus Error: %s%s%s%s%s\n",
    timeout ? "" : "Timeout ",
    data & HST_STS_FAIL ? "Transaction Failed " : "",
    data & HST_STS_BUSERR ? "No response or Bus Collision " : "",
    data & HST_STS_DEVERR ? "Device Error " : "",
    !(data & HST_STS_DONE) ? "Transaction Never Finished " : "");
    return status;
    }
    static int ali1563_block(struct i2c_adapter *a,
    union i2c_smbus_data *data, u8 rw)
    {
    int i, len;
    let mut error: c_int = 0;
// Do we need this?
    outb_p(HST_CNTL1_LAST, SMB_HST_CNTL1);
    if (rw == I2C_SMBUS_WRITE) {
    len = data.block[0];
    if (len < 1)
    len = 1;
#[no_mangle]
pub unsafe extern "C" fn if(32: len >) -> else {
    else if (len > 32)
    len = 32;
    outb_p(len, SMB_HST_DAT0);
    outb_p(data.block[1], SMB_BLK_DAT);
    } else
    len = 32;
    outb_p(inb_p(SMB_HST_CNTL2) | HST_CNTL2_BLOCK, SMB_HST_CNTL2);
    for (i = 0; i < len; i++) {
    if (rw == I2C_SMBUS_WRITE) {
    outb_p(data.block[i + 1], SMB_BLK_DAT);
    error = ali1563_block_start(a);
    if (error)
    break;
    } else {
    error = ali1563_block_start(a);
    if (error)
    break;
    if (i == 0) {
    len = inb_p(SMB_HST_DAT0);
    if (len < 1)
    len = 1;
#[no_mangle]
pub unsafe extern "C" fn if(32: len >) -> else {
    else if (len > 32)
    len = 32;
    }
    data.block[i+1] = inb_p(SMB_BLK_DAT);
    }
    }
// Do we need this?
    outb_p(HST_CNTL1_LAST, SMB_HST_CNTL1);
    return error;
    }
    static s32 ali1563_access(struct i2c_adapter *a, u16 addr,
    unsigned short flags, char rw, u8 cmd,
    int size, union i2c_smbus_data *data)
    {
    let mut error: c_int = 0;
    int timeout;
    u32 reg;
    for (timeout = ALI1563_MAX_TIMEOUT; timeout; timeout--) {
    reg = inb_p(SMB_HST_STS);
    if (!(reg & HST_STS_BUSY))
    break;
    }
    if (!timeout)
    dev_warn(&a.dev, "SMBus not idle. HST_STS = %02x\n", reg);
    outb_p(0xff, SMB_HST_STS);
// Map the size to what the chip understands
    switch (size) {
    case I2C_SMBUS_QUICK:
    size = HST_CNTL2_QUICK;
    break;
    case I2C_SMBUS_BYTE:
    size = HST_CNTL2_BYTE;
    break;
    case I2C_SMBUS_BYTE_DATA:
    size = HST_CNTL2_BYTE_DATA;
    break;
    case I2C_SMBUS_WORD_DATA:
    size = HST_CNTL2_WORD_DATA;
    break;
    case I2C_SMBUS_BLOCK_DATA:
    size = HST_CNTL2_BLOCK;
    break;
    default:
    dev_warn(&a.dev, "Unsupported transaction %d\n", size);
    error = -EOPNOTSUPP;
    goto Done;
    }
    outb_p(((addr & 0x7f) << 1) | (rw & 0x01), SMB_HST_ADD);
    outb_p((inb_p(SMB_HST_CNTL2) & ~HST_CNTL2_SIZEMASK) |
    (size << 3), SMB_HST_CNTL2);
// Write the command register
    switch (size) {
    case HST_CNTL2_BYTE:
    if (rw == I2C_SMBUS_WRITE)
// Beware it uses DAT0 register and not CMD!
    outb_p(cmd, SMB_HST_DAT0);
    break;
    case HST_CNTL2_BYTE_DATA:
    outb_p(cmd, SMB_HST_CMD);
    if (rw == I2C_SMBUS_WRITE)
    outb_p(data.byte, SMB_HST_DAT0);
    break;
    case HST_CNTL2_WORD_DATA:
    outb_p(cmd, SMB_HST_CMD);
    if (rw == I2C_SMBUS_WRITE) {
    outb_p(data.word & 0xff, SMB_HST_DAT0);
    outb_p((data.word & 0xff00) >> 8, SMB_HST_DAT1);
    }
    break;
    case HST_CNTL2_BLOCK:
    outb_p(cmd, SMB_HST_CMD);
    error = ali1563_block(a, data, rw);
    goto Done;
    }
    error = ali1563_transaction(a, size);
    if (error)
    goto Done;
    if ((rw == I2C_SMBUS_WRITE) || (size == HST_CNTL2_QUICK))
    goto Done;
    switch (size) {
    case HST_CNTL2_BYTE:	/* Result put in SMBHSTDAT0 */
    data.byte = inb_p(SMB_HST_DAT0);
    break;
    case HST_CNTL2_BYTE_DATA:
    data.byte = inb_p(SMB_HST_DAT0);
    break;
    case HST_CNTL2_WORD_DATA:
    data.word = inb_p(SMB_HST_DAT0) + (inb_p(SMB_HST_DAT1) << 8);
    break;
    }
    Done:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ali1563_func(a: *mut i2c_adapter) -> u32 {
    static u32 ali1563_func(struct i2c_adapter *a)
    {
    return I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA;
    }
#[no_mangle]
unsafe extern "C" fn ali1563_setup(dev: *mut pci_dev) -> c_int {
    static int ali1563_setup(struct pci_dev *dev)
    {
    u16 ctrl;
    pci_read_config_word(dev, ALI1563_SMBBA, &ctrl);
// SMB I/O Base in high 12 bits and must be aligned with the
// size of the I/O space.
    ali1563_smba = ctrl & ~(ALI1563_SMB_IOSIZE - 1);
    if (!ali1563_smba) {
    dev_warn(&dev.dev, "ali1563_smba Uninitialized\n");
    goto Err;
    }
// Check if device is enabled
    if (!(ctrl & ALI1563_SMB_HOSTEN)) {
    dev_warn(&dev.dev, "Host Controller not enabled\n");
    goto Err;
    }
    if (!(ctrl & ALI1563_SMB_IOEN)) {
    dev_warn(&dev.dev, "I/O space not enabled, trying manually\n");
    pci_write_config_word(dev, ALI1563_SMBBA,
    ctrl | ALI1563_SMB_IOEN);
    pci_read_config_word(dev, ALI1563_SMBBA, &ctrl);
    if (!(ctrl & ALI1563_SMB_IOEN)) {
    dev_err(&dev.dev,
    "I/O space still not enabled, giving up\n");
    goto Err;
    }
    }
    if (acpi_check_region(ali1563_smba, ALI1563_SMB_IOSIZE,
    ali1563_pci_driver.name))
    goto Err;
    if (!request_region(ali1563_smba, ALI1563_SMB_IOSIZE,
    ali1563_pci_driver.name)) {
    dev_err(&dev.dev, "Could not allocate I/O space at 0x%04x\n",
    ali1563_smba);
    goto Err;
    }
    dev_info(&dev.dev, "Found ALi1563 SMBus at 0x%04x\n", ali1563_smba);
    return 0;
    Err:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn ali1563_shutdown(dev: *mut pci_dev) {
    static void ali1563_shutdown(struct pci_dev *dev)
    {
    release_region(ali1563_smba, ALI1563_SMB_IOSIZE);
    }
    static const struct i2c_algorithm ali1563_algorithm = {
    .smbus_xfer	= ali1563_access,
    .functionality	= ali1563_func,
    };
    static struct i2c_adapter ali1563_adapter = {
    .owner	= THIS_MODULE,
    .class	= I2C_CLASS_HWMON,
    .algo	= &ali1563_algorithm,
    };
    static int ali1563_probe(struct pci_dev *dev,
    const struct pci_device_id *id_table)
    {
    int error;
    error = ali1563_setup(dev);
    if (error)
    goto exit;
    ali1563_adapter.dev.parent = &dev.dev;
    snprintf(ali1563_adapter.name, sizeof(ali1563_adapter.name),
    "SMBus ALi 1563 Adapter @ %04x", ali1563_smba);
    error = i2c_add_adapter(&ali1563_adapter);
    if (error)
    goto exit_shutdown;
    return 0;
    exit_shutdown:
    ali1563_shutdown(dev);
    exit:
    dev_warn(&dev.dev, "ALi1563 SMBus probe failed (%d)\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ali1563_remove(dev: *mut pci_dev) {
    static void ali1563_remove(struct pci_dev *dev)
    {
    i2c_del_adapter(&ali1563_adapter);
    ali1563_shutdown(dev);
    }
    static const struct pci_device_id ali1563_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M1563) },
    {},
    };
    MODULE_DEVICE_TABLE(pci, ali1563_id_table);
    static struct pci_driver ali1563_pci_driver = {
    .name		= "ali1563_smbus",
    .id_table	= ali1563_id_table,
    .probe		= ali1563_probe,
    .remove		= ali1563_remove,
    };
    module_pci_driver(ali1563_pci_driver);
    MODULE_DESCRIPTION("i2c driver for the ALi 1563 Southbridge");
    MODULE_LICENSE("GPL");
