//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-amd8111.c
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
// SMBus 2.0 driver for AMD-8111 IO-Hub.
//
// Copyright (c) 2002 Vojtech Pavlik
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Vojtech Pavlik <vojtech@suse.cz>");
    MODULE_DESCRIPTION("AMD8111 SMBus 2.0 driver");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_smbus {
    pub dev: *mut pci_dev,
    pub adapter: i2c_adapter,
    pub base: c_int,
    pub size: c_int,
}

    static struct pci_driver amd8111_driver;
//
// AMD PCI control registers definitions.
//
pub const AMD_PCI_MISC: c_uint = 0x48;
pub const AMD_PCI_MISC_SCI: c_uint = 0x04	/* deliver SCI */;
pub const AMD_PCI_MISC_INT: c_uint = 0x02	/* deliver PCI IRQ */;
pub const AMD_PCI_MISC_SPEEDUP: c_uint = 0x01	/* 16x clock speedup */;
//
// ACPI 2.0 chapter 13 PCI interface definitions.
//
pub const AMD_EC_DATA: c_uint = 0x00	/* data register */;
pub const AMD_EC_SC: c_uint = 0x04	/* status of controller */;
pub const AMD_EC_CMD: c_uint = 0x04	/* command register */;
pub const AMD_EC_ICR: c_uint = 0x08	/* interrupt control register */;
pub const AMD_EC_SC_SMI: c_uint = 0x04	/* smi event pending */;
pub const AMD_EC_SC_SCI: c_uint = 0x02	/* sci event pending */;
pub const AMD_EC_SC_BURST: c_uint = 0x01	/* burst mode enabled */;
pub const AMD_EC_SC_CMD: c_uint = 0x08	/* byte in data reg is command */;
pub const AMD_EC_SC_IBF: c_uint = 0x02	/* data ready for embedded controller */;
pub const AMD_EC_SC_OBF: c_uint = 0x01	/* data ready for host */;
pub const AMD_EC_CMD_RD: c_uint = 0x80	/* read EC */;
pub const AMD_EC_CMD_WR: c_uint = 0x81	/* write EC */;
pub const AMD_EC_CMD_BE: c_uint = 0x82	/* enable burst mode */;
pub const AMD_EC_CMD_BD: c_uint = 0x83	/* disable burst mode */;
pub const AMD_EC_CMD_QR: c_uint = 0x84	/* query EC */;
//
// ACPI 2.0 chapter 13 access of registers of the EC
//
#[no_mangle]
unsafe extern "C" fn amd_ec_wait_write(smbus: *mut amd_smbus) -> c_int {
    static int amd_ec_wait_write(struct amd_smbus *smbus)
    {
    let mut timeout: c_int = 500;
    while ((inb(smbus.base + AMD_EC_SC) & AMD_EC_SC_IBF) && --timeout)
    udelay(1);
    if (!timeout) {
    dev_warn(&smbus.dev.dev,
    "Timeout while waiting for IBF to clear\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_ec_wait_read(smbus: *mut amd_smbus) -> c_int {
    static int amd_ec_wait_read(struct amd_smbus *smbus)
    {
    let mut timeout: c_int = 500;
    while ((~inb(smbus.base + AMD_EC_SC) & AMD_EC_SC_OBF) && --timeout)
    udelay(1);
    if (!timeout) {
    dev_warn(&smbus.dev.dev,
    "Timeout while waiting for OBF to set\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
    static int amd_ec_read(struct amd_smbus *smbus, unsigned char address,
    unsigned char *data)
    {
    int status;
    status = amd_ec_wait_write(smbus);
    if (status)
    return status;
    outb(AMD_EC_CMD_RD, smbus.base + AMD_EC_CMD);
    status = amd_ec_wait_write(smbus);
    if (status)
    return status;
    outb(address, smbus.base + AMD_EC_DATA);
    status = amd_ec_wait_read(smbus);
    if (status)
    return status;
// data = inb(smbus->base + AMD_EC_DATA);
    return 0;
    }
    static int amd_ec_write(struct amd_smbus *smbus, unsigned char address,
    unsigned char data)
    {
    int status;
    status = amd_ec_wait_write(smbus);
    if (status)
    return status;
    outb(AMD_EC_CMD_WR, smbus.base + AMD_EC_CMD);
    status = amd_ec_wait_write(smbus);
    if (status)
    return status;
    outb(address, smbus.base + AMD_EC_DATA);
    status = amd_ec_wait_write(smbus);
    if (status)
    return status;
    outb(data, smbus.base + AMD_EC_DATA);
    return 0;
    }
//
// ACPI 2.0 chapter 13 SMBus 2.0 EC register model
//
pub const AMD_SMB_PRTCL: c_uint = 0x00	/* protocol, PEC */;
pub const AMD_SMB_STS: c_uint = 0x01	/* status */;
pub const AMD_SMB_ADDR: c_uint = 0x02	/* address */;
pub const AMD_SMB_CMD: c_uint = 0x03	/* command */;
pub const AMD_SMB_DATA: c_uint = 0x04	/* 32 data registers */;
pub const AMD_SMB_BCNT: c_uint = 0x24	/* number of data bytes */;
pub const AMD_SMB_ALRM_A: c_uint = 0x25	/* alarm address */;
pub const AMD_SMB_ALRM_D: c_uint = 0x26	/* 2 bytes alarm data */;
pub const AMD_SMB_STS_DONE: c_uint = 0x80;
pub const AMD_SMB_STS_ALRM: c_uint = 0x40;
pub const AMD_SMB_STS_RES: c_uint = 0x20;
pub const AMD_SMB_STS_STATUS: c_uint = 0x1f;
pub const AMD_SMB_STATUS_OK: c_uint = 0x00;
pub const AMD_SMB_STATUS_FAIL: c_uint = 0x07;
pub const AMD_SMB_STATUS_DNAK: c_uint = 0x10;
pub const AMD_SMB_STATUS_DERR: c_uint = 0x11;
pub const AMD_SMB_STATUS_CMD_DENY: c_uint = 0x12;
pub const AMD_SMB_STATUS_UNKNOWN: c_uint = 0x13;
pub const AMD_SMB_STATUS_ACC_DENY: c_uint = 0x17;
pub const AMD_SMB_STATUS_TIMEOUT: c_uint = 0x18;
pub const AMD_SMB_STATUS_NOTSUP: c_uint = 0x19;
pub const AMD_SMB_STATUS_BUSY: c_uint = 0x1A;
pub const AMD_SMB_STATUS_PEC: c_uint = 0x1F;
pub const AMD_SMB_PRTCL_WRITE: c_uint = 0x00;
pub const AMD_SMB_PRTCL_READ: c_uint = 0x01;
pub const AMD_SMB_PRTCL_QUICK: c_uint = 0x02;
pub const AMD_SMB_PRTCL_BYTE: c_uint = 0x04;
pub const AMD_SMB_PRTCL_BYTE_DATA: c_uint = 0x06;
pub const AMD_SMB_PRTCL_WORD_DATA: c_uint = 0x08;
pub const AMD_SMB_PRTCL_BLOCK_DATA: c_uint = 0x0a;
pub const AMD_SMB_PRTCL_PROC_CALL: c_uint = 0x0c;
pub const AMD_SMB_PRTCL_BLOCK_PROC_CALL: c_uint = 0x0d;
pub const AMD_SMB_PRTCL_I2C_BLOCK_DATA: c_uint = 0x4a;
pub const AMD_SMB_PRTCL_PEC: c_uint = 0x80;
    static s32 amd8111_access(struct i2c_adapter *adap, u16 addr,
    unsigned short flags, char read_write, u8 command, int size,
    union i2c_smbus_data *data)
    {
    struct amd_smbus *smbus = adap.algo_data;
    unsigned char protocol, len, pec, temp[2];
    int i, status;
    protocol = (read_write == I2C_SMBUS_READ) ? AMD_SMB_PRTCL_READ
    : AMD_SMB_PRTCL_WRITE;
    pec = (flags & I2C_CLIENT_PEC) ? AMD_SMB_PRTCL_PEC : 0;
    switch (size) {
    case I2C_SMBUS_QUICK:
    protocol |= AMD_SMB_PRTCL_QUICK;
    read_write = I2C_SMBUS_WRITE;
    break;
    case I2C_SMBUS_BYTE:
    if (read_write == I2C_SMBUS_WRITE) {
    status = amd_ec_write(smbus, AMD_SMB_CMD,
    command);
    if (status)
    return status;
    }
    protocol |= AMD_SMB_PRTCL_BYTE;
    break;
    case I2C_SMBUS_BYTE_DATA:
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    if (read_write == I2C_SMBUS_WRITE) {
    status = amd_ec_write(smbus, AMD_SMB_DATA,
    data.byte);
    if (status)
    return status;
    }
    protocol |= AMD_SMB_PRTCL_BYTE_DATA;
    break;
    case I2C_SMBUS_WORD_DATA:
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    if (read_write == I2C_SMBUS_WRITE) {
    status = amd_ec_write(smbus, AMD_SMB_DATA,
    data.word & 0xff);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_DATA + 1,
    data.word >> 8);
    if (status)
    return status;
    }
    protocol |= AMD_SMB_PRTCL_WORD_DATA | pec;
    break;
    case I2C_SMBUS_BLOCK_DATA:
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    if (read_write == I2C_SMBUS_WRITE) {
    len = min_t(u8, data.block[0],
    I2C_SMBUS_BLOCK_MAX);
    status = amd_ec_write(smbus, AMD_SMB_BCNT, len);
    if (status)
    return status;
    for (i = 0; i < len; i++) {
    status =
    amd_ec_write(smbus, AMD_SMB_DATA + i,
    data.block[i + 1]);
    if (status)
    return status;
    }
    }
    protocol |= AMD_SMB_PRTCL_BLOCK_DATA | pec;
    break;
    case I2C_SMBUS_I2C_BLOCK_DATA:
    len = min_t(u8, data.block[0],
    I2C_SMBUS_BLOCK_MAX);
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_BCNT, len);
    if (status)
    return status;
    if (read_write == I2C_SMBUS_WRITE)
    for (i = 0; i < len; i++) {
    status =
    amd_ec_write(smbus, AMD_SMB_DATA + i,
    data.block[i + 1]);
    if (status)
    return status;
    }
    protocol |= AMD_SMB_PRTCL_I2C_BLOCK_DATA;
    break;
    case I2C_SMBUS_PROC_CALL:
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_DATA,
    data.word & 0xff);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_DATA + 1,
    data.word >> 8);
    if (status)
    return status;
    protocol = AMD_SMB_PRTCL_PROC_CALL | pec;
    read_write = I2C_SMBUS_READ;
    break;
    case I2C_SMBUS_BLOCK_PROC_CALL:
    len = min_t(u8, data.block[0],
    I2C_SMBUS_BLOCK_MAX - 1);
    status = amd_ec_write(smbus, AMD_SMB_CMD, command);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_BCNT, len);
    if (status)
    return status;
    for (i = 0; i < len; i++) {
    status = amd_ec_write(smbus, AMD_SMB_DATA + i,
    data.block[i + 1]);
    if (status)
    return status;
    }
    protocol = AMD_SMB_PRTCL_BLOCK_PROC_CALL | pec;
    read_write = I2C_SMBUS_READ;
    break;
    default:
    dev_warn(&adap.dev, "Unsupported transaction %d\n", size);
    return -EOPNOTSUPP;
    }
    status = amd_ec_write(smbus, AMD_SMB_ADDR, addr << 1);
    if (status)
    return status;
    status = amd_ec_write(smbus, AMD_SMB_PRTCL, protocol);
    if (status)
    return status;
    status = amd_ec_read(smbus, AMD_SMB_STS, temp + 0);
    if (status)
    return status;
    if (~temp[0] & AMD_SMB_STS_DONE) {
    udelay(500);
    status = amd_ec_read(smbus, AMD_SMB_STS, temp + 0);
    if (status)
    return status;
    }
    if (~temp[0] & AMD_SMB_STS_DONE) {
    msleep(1);
    status = amd_ec_read(smbus, AMD_SMB_STS, temp + 0);
    if (status)
    return status;
    }
    if ((~temp[0] & AMD_SMB_STS_DONE) || (temp[0] & AMD_SMB_STS_STATUS))
    return -EIO;
    if (read_write == I2C_SMBUS_WRITE)
    return 0;
    switch (size) {
    case I2C_SMBUS_BYTE:
    case I2C_SMBUS_BYTE_DATA:
    status = amd_ec_read(smbus, AMD_SMB_DATA, &data.byte);
    if (status)
    return status;
    break;
    case I2C_SMBUS_WORD_DATA:
    case I2C_SMBUS_PROC_CALL:
    status = amd_ec_read(smbus, AMD_SMB_DATA, temp + 0);
    if (status)
    return status;
    status = amd_ec_read(smbus, AMD_SMB_DATA + 1, temp + 1);
    if (status)
    return status;
    data.word = (temp[1] << 8) | temp[0];
    break;
    case I2C_SMBUS_BLOCK_DATA:
    case I2C_SMBUS_BLOCK_PROC_CALL:
    status = amd_ec_read(smbus, AMD_SMB_BCNT, &len);
    if (status)
    return status;
    len = min_t(u8, len, I2C_SMBUS_BLOCK_MAX);
    fallthrough;
    case I2C_SMBUS_I2C_BLOCK_DATA:
    for (i = 0; i < len; i++) {
    status = amd_ec_read(smbus, AMD_SMB_DATA + i,
    data.block + i + 1);
    if (status)
    return status;
    }
    data.block[0] = len;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd8111_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 amd8111_func(struct i2c_adapter *adapter)
    {
    return	I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA | I2C_FUNC_SMBUS_BLOCK_DATA |
    I2C_FUNC_SMBUS_PROC_CALL | I2C_FUNC_SMBUS_BLOCK_PROC_CALL |
    I2C_FUNC_SMBUS_I2C_BLOCK | I2C_FUNC_SMBUS_PEC;
    }
    static const struct i2c_algorithm smbus_algorithm = {
    .smbus_xfer = amd8111_access,
    .functionality = amd8111_func,
    };
    static const struct pci_device_id amd8111_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_8111_SMBUS2) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, amd8111_ids);
#[no_mangle]
unsafe extern "C" fn amd8111_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int amd8111_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct amd_smbus *smbus;
    int error;
    if (!(pci_resource_flags(dev, 0) & IORESOURCE_IO))
    return -ENODEV;
    smbus = devm_kzalloc(&dev.dev, sizeof(struct amd_smbus), GFP_KERNEL);
    if (!smbus)
    return -ENOMEM;
    smbus.dev = dev;
    smbus.base = pci_resource_start(dev, 0);
    smbus.size = pci_resource_len(dev, 0);
    error = acpi_check_resource_conflict(&dev.resource[0]);
    if (error)
    return -ENODEV;
    if (!devm_request_region(&dev.dev, smbus.base, smbus.size, amd8111_driver.name))
    return -EBUSY;
    smbus.adapter.owner = THIS_MODULE;
    snprintf(smbus.adapter.name, sizeof(smbus.adapter.name),
    "SMBus2 AMD8111 adapter at %04x", smbus.base);
    smbus.adapter.class = I2C_CLASS_HWMON;
    smbus.adapter.algo = &smbus_algorithm;
    smbus.adapter.algo_data = smbus;
// set up the sysfs linkage to our parent device
    smbus.adapter.dev.parent = &dev.dev;
    pci_write_config_dword(smbus.dev, AMD_PCI_MISC, 0);
    error = i2c_add_adapter(&smbus.adapter);
    if (error)
    return error;
    pci_set_drvdata(dev, smbus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd8111_remove(dev: *mut pci_dev) {
    static void amd8111_remove(struct pci_dev *dev)
    {
    struct amd_smbus *smbus = pci_get_drvdata(dev);
    i2c_del_adapter(&smbus.adapter);
    }
    static struct pci_driver amd8111_driver = {
    .name		= "amd8111_smbus2",
    .id_table	= amd8111_ids,
    .probe		= amd8111_probe,
    .remove		= amd8111_remove,
    };
    module_pci_driver(amd8111_driver);
