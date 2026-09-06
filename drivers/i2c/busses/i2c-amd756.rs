//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-amd756.c
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
    Copyright (c) 1999-2002 Merlin Hughes <merlin@merlin.org>
    Shamelessly ripped from i2c-piix4.c:
    Copyright (c) 1998, 1999  Frodo Looijaard <frodol@dds.nl> and
    Philip Edelbrock <phil@netroedge.com>
//
    2002-04-08: Added nForce support. (Csaba Halasz)
    2002-10-03: Fixed nForce PnP I/O port. (Michael Steil)
    2002-12-28: Rewritten into something that resembles a Linux driver (hch)
    2003-11-29: Added back AMD8111 removed by the previous rewrite.
    (Philip Pokorny)
//
    Supports AMD756, AMD766, AMD768, AMD8111 and nVidia nForce
    Note: we assume there can only be one device, with one SMBus interface.
//

// AMD756 SMBus address offsets
pub const SMB_ADDR_OFFSET: c_uint = 0xE0;
pub const SMB_IOSIZE: c_int = 16;

// PCI Address Constants
// address of I/O space
pub const SMBBA: c_uint = 0x058		/* mh */;
pub const SMBBANFORCE: c_uint = 0x014;
// general configuration
pub const SMBGCFG: c_uint = 0x041		/* mh */;
// silicon revision code
pub const SMBREV: c_uint = 0x008;
// Other settings
pub const MAX_TIMEOUT: c_int = 500;
// AMD756 constants
pub const AMD756_QUICK: c_uint = 0x00;
pub const AMD756_BYTE: c_uint = 0x01;
pub const AMD756_BYTE_DATA: c_uint = 0x02;
pub const AMD756_WORD_DATA: c_uint = 0x03;
pub const AMD756_PROCESS_CALL: c_uint = 0x04;
pub const AMD756_BLOCK_DATA: c_uint = 0x05;
    static struct pci_driver amd756_driver;
    static unsigned short amd756_ioport;
//
    SMBUS event = I/O 28-29 bit 11
    see E0 for the status bits and enabled in E2
//

    GS_HCYC_STS | GS_TO_STS )

#[no_mangle]
unsafe extern "C" fn amd756_transaction(adap: *mut i2c_adapter) -> c_int {
    static int amd756_transaction(struct i2c_adapter *adap)
    {
    int temp;
    let mut result: c_int = 0;
    let mut timeout: c_int = 0;
    dev_dbg(&adap.dev, "Transaction (pre): GS=%04x, GE=%04x, ADD=%04x, "
    "DAT=%04x\n", inw_p(SMB_GLOBAL_STATUS),
    inw_p(SMB_GLOBAL_ENABLE), inw_p(SMB_HOST_ADDRESS),
    inb_p(SMB_HOST_DATA));
// Make sure the SMBus host is ready to start transmitting
    if ((temp = inw_p(SMB_GLOBAL_STATUS)) & (GS_HST_STS | GS_SMB_STS)) {
    dev_dbg(&adap.dev, "SMBus busy (%04x). Waiting...\n", temp);
    do {
    msleep(1);
    temp = inw_p(SMB_GLOBAL_STATUS);
    } while ((temp & (GS_HST_STS | GS_SMB_STS)) &&
    (timeout++ < MAX_TIMEOUT));
// If the SMBus is still busy, we give up
    if (timeout > MAX_TIMEOUT) {
    dev_dbg(&adap.dev, "Busy wait timeout (%04x)\n", temp);
    goto abort;
    }
    timeout = 0;
    }
// start the transaction by setting the start bit
    outw_p(inw(SMB_GLOBAL_ENABLE) | GE_HOST_STC, SMB_GLOBAL_ENABLE);
// We will always wait for a fraction of a second!
    do {
    msleep(1);
    temp = inw_p(SMB_GLOBAL_STATUS);
    } while ((temp & GS_HST_STS) && (timeout++ < MAX_TIMEOUT));
// If the SMBus is still busy, we give up
    if (timeout > MAX_TIMEOUT) {
    dev_dbg(&adap.dev, "Completion timeout!\n");
    goto abort;
    }
    if (temp & GS_PRERR_STS) {
    result = -ENXIO;
    dev_dbg(&adap.dev, "SMBus Protocol error (no response)!\n");
    }
    if (temp & GS_COL_STS) {
    result = -EIO;
    dev_warn(&adap.dev, "SMBus collision!\n");
    }
    if (temp & GS_TO_STS) {
    result = -ETIMEDOUT;
    dev_dbg(&adap.dev, "SMBus protocol timeout!\n");
    }
    if (temp & GS_HCYC_STS)
    dev_dbg(&adap.dev, "SMBus protocol success!\n");
    outw_p(GS_CLEAR_STS, SMB_GLOBAL_STATUS);

    if (((temp = inw_p(SMB_GLOBAL_STATUS)) & GS_CLEAR_STS) != 0x00) {
    dev_dbg(&adap.dev,
    "Failed reset at end of transaction (%04x)\n", temp);
    }

    dev_dbg(&adap.dev,
    "Transaction (post): GS=%04x, GE=%04x, ADD=%04x, DAT=%04x\n",
    inw_p(SMB_GLOBAL_STATUS), inw_p(SMB_GLOBAL_ENABLE),
    inw_p(SMB_HOST_ADDRESS), inb_p(SMB_HOST_DATA));
    return result;
    abort:
    dev_warn(&adap.dev, "Sending abort\n");
    outw_p(inw(SMB_GLOBAL_ENABLE) | GE_ABORT, SMB_GLOBAL_ENABLE);
    msleep(100);
    outw_p(GS_CLEAR_STS, SMB_GLOBAL_STATUS);
    return -EIO;
    }
// Return negative errno on error.
    static s32 amd756_access(struct i2c_adapter * adap, u16 addr,
    unsigned short flags, char read_write,
    u8 command, int size, union i2c_smbus_data * data)
    {
    int i, len;
    int status;
    switch (size) {
    case I2C_SMBUS_QUICK:
    outw_p(((addr & 0x7f) << 1) | (read_write & 0x01),
    SMB_HOST_ADDRESS);
    size = AMD756_QUICK;
    break;
    case I2C_SMBUS_BYTE:
    outw_p(((addr & 0x7f) << 1) | (read_write & 0x01),
    SMB_HOST_ADDRESS);
    if (read_write == I2C_SMBUS_WRITE)
    outb_p(command, SMB_HOST_DATA);
    size = AMD756_BYTE;
    break;
    case I2C_SMBUS_BYTE_DATA:
    outw_p(((addr & 0x7f) << 1) | (read_write & 0x01),
    SMB_HOST_ADDRESS);
    outb_p(command, SMB_HOST_COMMAND);
    if (read_write == I2C_SMBUS_WRITE)
    outw_p(data.byte, SMB_HOST_DATA);
    size = AMD756_BYTE_DATA;
    break;
    case I2C_SMBUS_WORD_DATA:
    outw_p(((addr & 0x7f) << 1) | (read_write & 0x01),
    SMB_HOST_ADDRESS);
    outb_p(command, SMB_HOST_COMMAND);
    if (read_write == I2C_SMBUS_WRITE)
    outw_p(data.word, SMB_HOST_DATA);
    size = AMD756_WORD_DATA;
    break;
    case I2C_SMBUS_BLOCK_DATA:
    outw_p(((addr & 0x7f) << 1) | (read_write & 0x01),
    SMB_HOST_ADDRESS);
    outb_p(command, SMB_HOST_COMMAND);
    if (read_write == I2C_SMBUS_WRITE) {
    len = data.block[0];
    if (len < 0)
    len = 0;
    if (len > 32)
    len = 32;
    outw_p(len, SMB_HOST_DATA);
// i = inw_p(SMBHSTCNT); Reset SMBBLKDAT
    for (i = 1; i <= len; i++)
    outb_p(data.block[i],
    SMB_HOST_BLOCK_DATA);
    }
    size = AMD756_BLOCK_DATA;
    break;
    default:
    dev_warn(&adap.dev, "Unsupported transaction %d\n", size);
    return -EOPNOTSUPP;
    }
// How about enabling interrupts...
    outw_p(size & GE_CYC_TYPE_MASK, SMB_GLOBAL_ENABLE);
    status = amd756_transaction(adap);
    if (status)
    return status;
    if ((read_write == I2C_SMBUS_WRITE) || (size == AMD756_QUICK))
    return 0;
    switch (size) {
    case AMD756_BYTE:
    data.byte = inw_p(SMB_HOST_DATA);
    break;
    case AMD756_BYTE_DATA:
    data.byte = inw_p(SMB_HOST_DATA);
    break;
    case AMD756_WORD_DATA:
    data.word = inw_p(SMB_HOST_DATA);
    break;
    case AMD756_BLOCK_DATA:
    data.block[0] = inw_p(SMB_HOST_DATA) & 0x3f;
    if(data.block[0] > 32)
    data.block[0] = 32;
// i = inw_p(SMBHSTCNT); Reset SMBBLKDAT
    for (i = 1; i <= data.block[0]; i++)
    data.block[i] = inb_p(SMB_HOST_BLOCK_DATA);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd756_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 amd756_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA;
    }
    static const struct i2c_algorithm smbus_algorithm = {
    .smbus_xfer	= amd756_access,
    .functionality	= amd756_func,
    };
    static struct i2c_adapter amd756_smbus = {
    .owner		= THIS_MODULE,
    .class          = I2C_CLASS_HWMON,
    .algo		= &smbus_algorithm,
    };
    enum chiptype { AMD756, AMD766, AMD768, NFORCE, AMD8111 };
    static const char* chipname[] = {
    "AMD756", "AMD766", "AMD768",
    "nVidia nForce", "AMD8111",
    };
    static const struct pci_device_id amd756_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_VIPER_740B),
    .driver_data = AMD756 },
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_VIPER_7413),
    .driver_data = AMD766 },
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_OPUS_7443),
    .driver_data = AMD768 },
    { PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_8111_SMBUS),
    .driver_data = AMD8111 },
    { PCI_DEVICE(PCI_VENDOR_ID_NVIDIA, PCI_DEVICE_ID_NVIDIA_NFORCE_SMBUS),
    .driver_data = NFORCE },
    { 0, }
    };
    MODULE_DEVICE_TABLE (pci, amd756_ids);
#[no_mangle]
unsafe extern "C" fn amd756_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int amd756_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    let mut nforce: c_int = (id.driver_data == NFORCE);
    int error;
    u8 temp;
    if (amd756_ioport) {
    dev_err(&pdev.dev, "Only one device supported "
    "(you have a strange motherboard, btw)\n");
    return -ENODEV;
    }
    if (nforce) {
    if (PCI_FUNC(pdev.devfn) != 1)
    return -ENODEV;
    pci_read_config_word(pdev, SMBBANFORCE, &amd756_ioport);
    amd756_ioport &= 0xfffc;
    } else { /* amd */
    if (PCI_FUNC(pdev.devfn) != 3)
    return -ENODEV;
    pci_read_config_byte(pdev, SMBGCFG, &temp);
    if ((temp & 128) == 0) {
    dev_err(&pdev.dev,
    "Error: SMBus controller I/O not enabled!\n");
    return -ENODEV;
    }
// Determine the address of the SMBus areas
// Technically it is a dword but...
    pci_read_config_word(pdev, SMBBA, &amd756_ioport);
    amd756_ioport &= 0xff00;
    amd756_ioport += SMB_ADDR_OFFSET;
    }
    error = acpi_check_region(amd756_ioport, SMB_IOSIZE,
    amd756_driver.name);
    if (error)
    return -ENODEV;
    if (!request_region(amd756_ioport, SMB_IOSIZE, amd756_driver.name)) {
    dev_err(&pdev.dev, "SMB region 0x%x already in use!\n",
    amd756_ioport);
    return -ENODEV;
    }
    pci_read_config_byte(pdev, SMBREV, &temp);
    dev_dbg(&pdev.dev, "SMBREV = 0x%X\n", temp);
    dev_dbg(&pdev.dev, "AMD756_smba = 0x%X\n", amd756_ioport);
// set up the sysfs linkage to our parent device
    amd756_smbus.dev.parent = &pdev.dev;
    snprintf(amd756_smbus.name, sizeof(amd756_smbus.name),
    "SMBus %s adapter at %04x", chipname[id.driver_data],
    amd756_ioport);
    error = i2c_add_adapter(&amd756_smbus);
    if (error)
    goto out_err;
    return 0;
    out_err:
    release_region(amd756_ioport, SMB_IOSIZE);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn amd756_remove(dev: *mut pci_dev) {
    static void amd756_remove(struct pci_dev *dev)
    {
    i2c_del_adapter(&amd756_smbus);
    release_region(amd756_ioport, SMB_IOSIZE);
    }
    static struct pci_driver amd756_driver = {
    .name		= "amd756_smbus",
    .id_table	= amd756_ids,
    .probe		= amd756_probe,
    .remove		= amd756_remove,
    };
    module_pci_driver(amd756_driver);
    MODULE_AUTHOR("Merlin Hughes <merlin@merlin.org>");
    MODULE_DESCRIPTION("AMD756/766/768/8111 and nVidia nForce SMBus driver");
    MODULE_LICENSE("GPL");
