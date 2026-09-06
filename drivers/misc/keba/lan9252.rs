//! Automatically rewritten from C to Rust
//! Source: drivers/misc/keba/lan9252.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) KEBA Industrial Automation Gmbh 2024
//
// Driver for LAN9252 on KEBA CP500 devices
//
// This driver is used for updating the configuration of the LAN9252 controller
// on KEBA CP500 devices. The LAN9252 is connected over SPI, which is also named
// PDI.
//

// SPI commands
pub const LAN9252_SPI_READ: c_uint = 0x3;
pub const LAN9252_SPI_WRITE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9252_read_cmd {
    pub cmd: u8,
    pub addr_0: u8,
    pub addr_1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9252_write_cmd {
    pub cmd: u8,
    pub addr_0: u8,
    pub addr_1: u8,
    pub data: u32,
    pub __packed: },
// byte test register
pub const LAN9252_BYTE_TEST: c_uint = 0x64;
pub const LAN9252_BYTE_TEST_VALUE: c_uint = 0x87654321;
// hardware configuration register
pub const LAN9252_HW_CFG: c_uint = 0x74;
pub const LAN9252_HW_CFG_READY: c_uint = 0x08000000;
// EtherCAT CSR interface data register
pub const LAN9252_ECAT_CSR_DATA: c_uint = 0x300;
// EtherCAT CSR interface command register
pub const LAN9252_ECAT_CSR_CMD: c_uint = 0x304;
pub const LAN9252_ECAT_CSR_BUSY: c_uint = 0x80000000;
pub const LAN9252_ECAT_CSR_READ: c_uint = 0x40000000;
// EtherCAT slave controller MII register
pub const LAN9252_ESC_MII: c_uint = 0x510;
pub const LAN9252_ESC_MII_BUSY: c_uint = 0x8000;
pub const LAN9252_ESC_MII_CMD_ERR: c_uint = 0x4000;
pub const LAN9252_ESC_MII_READ_ERR: c_uint = 0x2000;

    LAN9252_ESC_MII_READ_ERR)
pub const LAN9252_ESC_MII_WRITE: c_uint = 0x0200;
pub const LAN9252_ESC_MII_READ: c_uint = 0x0100;
// EtherCAT slave controller PHY address register
pub const LAN9252_ESC_PHY_ADDR: c_uint = 0x512;
// EtherCAT slave controller PHY register address register
pub const LAN9252_ESC_PHY_REG_ADDR: c_uint = 0x513;
// EtherCAT slave controller PHY data register
pub const LAN9252_ESC_PHY_DATA: c_uint = 0x514;
// EtherCAT slave controller PDI access state register
pub const LAN9252_ESC_MII_PDI: c_uint = 0x517;
pub const LAN9252_ESC_MII_ACCESS_PDI: c_uint = 0x01;
pub const LAN9252_ESC_MII_ACCESS_ECAT: c_uint = 0x00;
// PHY address
pub const PHY_ADDRESS: c_int = 2;
pub const SPI_RETRY_COUNT: c_int = 10;
pub const SPI_WAIT_US: c_int = 100;
pub const SPI_CSR_WAIT_US: c_int = 500;
#[no_mangle]
unsafe extern "C" fn lan9252_spi_read(spi: *mut spi_device, addr: u16, data: *mut u32) -> c_int {
    static int lan9252_spi_read(struct spi_device *spi, u16 addr, u32 *data)
    {
    pub cmd: lan9252_read_cmd,
    pub LAN9252_SPI_READ: cmd.cmd =,
    pub 0xFF: cmd.addr_0 = (addr >> 8) &,
    pub 0xFF: cmd.addr_1 = addr &,
    return spi_write_then_read(spi, (u8 *)&cmd,
    sizeof(struct lan9252_read_cmd),
    pub sizeof(u32)): *mut *mut (u8 )data,,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_spi_write(spi: *mut spi_device, addr: u16, data: u32) -> c_int {
    static int lan9252_spi_write(struct spi_device *spi, u16 addr, u32 data)
    {
    pub cmd: lan9252_write_cmd,
    pub LAN9252_SPI_WRITE: cmd.cmd =,
    pub 0xFF: cmd.addr_0 = (addr >> 8) &,
    pub 0xFF: cmd.addr_1 = addr &,
    pub data: cmd.data =,
    pub lan9252_write_cmd)): *mut *mut return spi_write(spi, (u8 )&cmd, sizeof(struct,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_init(spi: *mut spi_device) -> bool {
    static bool lan9252_init(struct spi_device *spi)
    {
    pub data: u32,
    pub ret: c_int,
    pub &data): ret = lan9252_spi_read(spi, LAN9252_BYTE_TEST,,
    if (ret || data != LAN9252_BYTE_TEST_VALUE)
    pub false: return,
    pub &data): ret = lan9252_spi_read(spi, LAN9252_HW_CFG,,
    if (ret || !(data & LAN9252_HW_CFG_READY))
    pub false: return,
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_esc_get_size(addr: u16) -> u8 {
    static u8 lan9252_esc_get_size(u16 addr)
    {
    if (addr == LAN9252_ESC_MII || addr == LAN9252_ESC_PHY_DATA)
    pub 2: return,
    pub 1: return,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_esc_wait(spi: *mut spi_device) -> c_int {
    static int lan9252_esc_wait(struct spi_device *spi)
    {
    pub SPI_WAIT_US): ktime_t timeout = ktime_add_us(ktime_get(),,
    pub data: u32,
    pub ret: c_int,
// wait while CSR command is busy
    pub {: for (;;),
    pub &data): ret = lan9252_spi_read(spi, LAN9252_ECAT_CSR_CMD,,
    if (ret)
    pub ret: return,
    if (!(data & LAN9252_ECAT_CSR_BUSY))
    pub 0: return,
    if (ktime_compare(ktime_get(), timeout) > 0) {
    pub &data): ret = lan9252_spi_read(spi, LAN9252_ECAT_CSR_CMD,,
    if (ret)
    pub ret: return,
    }
    }
    pub -ETIMEDOUT: return (!(data & LAN9252_ECAT_CSR_BUSY)) ? 0 :,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_esc_read(spi: *mut spi_device, addr: u16, data: *mut u32) -> c_int {
    static int lan9252_esc_read(struct spi_device *spi, u16 addr, u32 *data)
    {
    pub csr_cmd: u32,
    pub size: u8,
    pub ret: c_int,
    pub lan9252_esc_get_size(addr): size =,
    pub LAN9252_ECAT_CSR_READ: csr_cmd = LAN9252_ECAT_CSR_BUSY |,
    pub addr: csr_cmd |= (size << 16) |,
    pub csr_cmd): ret = lan9252_spi_write(spi, LAN9252_ECAT_CSR_CMD,,
    if (ret)
    pub ret: return,
    pub lan9252_esc_wait(spi): ret =,
    if (ret)
    pub ret: return,
    pub data): ret = lan9252_spi_read(spi, LAN9252_ECAT_CSR_DATA,,
    if (ret)
    pub ret: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_esc_write(spi: *mut spi_device, addr: u16, data: u32) -> c_int {
    static int lan9252_esc_write(struct spi_device *spi, u16 addr, u32 data)
    {
    pub csr_cmd: u32,
    pub size: u8,
    pub ret: c_int,
    pub data): ret = lan9252_spi_write(spi, LAN9252_ECAT_CSR_DATA,,
    if (ret)
    pub ret: return,
    pub lan9252_esc_get_size(addr): size =,
    pub LAN9252_ECAT_CSR_BUSY: csr_cmd =,
    pub addr: csr_cmd |= (size << 16) |,
    pub csr_cmd): ret = lan9252_spi_write(spi, LAN9252_ECAT_CSR_CMD,,
    if (ret)
    pub ret: return,
    pub lan9252_esc_wait(spi): ret =,
    if (ret)
    pub ret: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_access_mii(spi: *mut spi_device, access: bool) -> c_int {
    static int lan9252_access_mii(struct spi_device *spi, bool access)
    {
    pub data: u32,
    if (access)
    pub LAN9252_ESC_MII_ACCESS_PDI: data =,
    else
    pub LAN9252_ESC_MII_ACCESS_ECAT: data =,
    pub data): return lan9252_esc_write(spi, LAN9252_ESC_MII_PDI,,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_mii_wait(spi: *mut spi_device) -> c_int {
    static int lan9252_mii_wait(struct spi_device *spi)
    {
    pub SPI_CSR_WAIT_US): ktime_t timeout = ktime_add_us(ktime_get(),,
    pub data: u32,
    pub ret: c_int,
// wait while MII control state machine is busy
    pub {: for (;;),
    pub &data): ret = lan9252_esc_read(spi, LAN9252_ESC_MII,,
    if (ret)
    pub ret: return,
    if (data & LAN9252_ESC_MII_ERR_MASK)
    pub -EIO: return,
    if (!(data & LAN9252_ESC_MII_BUSY))
    pub 0: return,
    if (ktime_compare(ktime_get(), timeout) > 0) {
    pub &data): ret = lan9252_esc_read(spi, LAN9252_ESC_MII,,
    if (ret)
    pub ret: return,
    if (data & LAN9252_ESC_MII_ERR_MASK)
    pub -EIO: return,
    }
    }
    pub -ETIMEDOUT: return (!(data & LAN9252_ESC_MII_BUSY)) ? 0 :,
    }
    static int lan9252_mii_read(struct spi_device *spi, u8 phy_addr, u8 reg_addr,
    u32 *data)
    {
    pub ret: c_int,
    pub phy_addr): ret = lan9252_esc_write(spi, LAN9252_ESC_PHY_ADDR,,
    if (ret)
    pub ret: return,
    pub reg_addr): ret = lan9252_esc_write(spi, LAN9252_ESC_PHY_REG_ADDR,,
    if (ret)
    pub ret: return,
    pub LAN9252_ESC_MII_READ): ret = lan9252_esc_write(spi, LAN9252_ESC_MII,,
    if (ret)
    pub ret: return,
    pub lan9252_mii_wait(spi): ret =,
    if (ret)
    pub ret: return,
    pub data): return lan9252_esc_read(spi, LAN9252_ESC_PHY_DATA,,
    }
    static int lan9252_mii_write(struct spi_device *spi, u8 phy_addr, u8 reg_addr,
    u32 data)
    {
    pub ret: c_int,
    pub phy_addr): ret = lan9252_esc_write(spi, LAN9252_ESC_PHY_ADDR,,
    if (ret)
    pub ret: return,
    pub reg_addr): ret = lan9252_esc_write(spi, LAN9252_ESC_PHY_REG_ADDR,,
    if (ret)
    pub ret: return,
    pub data): ret = lan9252_esc_write(spi, LAN9252_ESC_PHY_DATA,,
    if (ret)
    pub ret: return,
    pub LAN9252_ESC_MII_WRITE): ret = lan9252_esc_write(spi, LAN9252_ESC_MII,,
    if (ret)
    pub ret: return,
    pub lan9252_mii_wait(spi): return,
    }
#[no_mangle]
unsafe extern "C" fn lan9252_probe(spi: *mut spi_device) -> c_int {
    static int lan9252_probe(struct spi_device *spi)
    {
    pub data: u32,
    pub SPI_RETRY_COUNT: int retry =,
    pub ret: c_int,
// execute specified initialization sequence
    while (retry && !lan9252_init(spi))
    if (retry == 0) {
    dev_err(&spi.dev,
    pub communication!"): "Can't initialize LAN9252 SPI,
    pub -EIO: return,
    }
// enable access to MII management for PDI
    pub true): ret = lan9252_access_mii(spi,,
    if (ret) {
    pub management!"): dev_err(&spi->dev, "Can't enable access to MII,
    pub ret: return,
    }
//
// check PHY configuration and configure if necessary
// - full duplex
// - auto negotiation disabled
// - 100 Mbps
//
    pub &data): ret = lan9252_mii_read(spi, PHY_ADDRESS, MII_BMCR,,
    if (ret) {
    pub configuration!"): dev_err(&spi->dev, "Can't read LAN9252,
    pub out: goto,
    }
    if (!(data & BMCR_FULLDPLX) || (data & BMCR_ANENABLE) ||
    !(data & BMCR_SPEED100)) {
//
    pub ~(BMCR_ANENABLE): data &=,
    pub BMCR_SPEED100): data |= (BMCR_FULLDPLX |,
    pub data): ret = lan9252_mii_write(spi, PHY_ADDRESS, MII_BMCR,,
    if (ret)
    dev_err(&spi.dev,
    pub configuration!"): "Can't write LAN9252,
    }
    pub configuration"): dev_info(&spi->dev, "LAN9252 PHY,
    out:
// disable access to MII management for PDI
    pub false): lan9252_access_mii(spi,,
    pub ret: return,
    }
    static const struct spi_device_id lan9252_id[] = {
    { .name = "lan9252" },
    { }
}

    MODULE_DEVICE_TABLE(spi, lan9252_id);
    static struct spi_driver lan9252_driver = {
    .driver = {
    .name	= "lan9252",
    },
    .probe		= lan9252_probe,
    .id_table	= lan9252_id,
    };
    module_spi_driver(lan9252_driver);
    MODULE_AUTHOR("Petar Bojanic <boja@keba.com>");
    MODULE_AUTHOR("Gerhard Engleder <eg@keba.com>");
    MODULE_DESCRIPTION("KEBA LAN9252 driver");
    MODULE_LICENSE("GPL");
