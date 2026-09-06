//! Automatically rewritten from C to Rust
//! Source: drivers/platform/mips/rs780e-acpi.c
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

    static unsigned long acpi_iobase;

pub const PM_INDEX: c_uint = 0xCD6;
pub const PM_DATA: c_uint = 0xCD7;
pub const PM2_INDEX: c_uint = 0xCD0;
pub const PM2_DATA: c_uint = 0xCD1;
#[no_mangle]
unsafe extern "C" fn pmio_write_index(index: u16, reg: u8, value: u8) {
    static void pmio_write_index(u16 index, u8 reg, u8 value)
    {
    outb(reg, index);
    outb(value, index + 1);
    }
#[no_mangle]
unsafe extern "C" fn pmio_read_index(index: u16, reg: u8) -> u8 {
    static u8 pmio_read_index(u16 index, u8 reg)
    {
    outb(reg, index);
    return inb(index + 1);
    }
#[no_mangle]
unsafe extern "C" fn pm_iowrite(reg: u8, value: u8) {
    static void pm_iowrite(u8 reg, u8 value)
    {
    pmio_write_index(PM_INDEX, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn pm_ioread(reg: u8) -> u8 {
    static u8 pm_ioread(u8 reg)
    {
    return pmio_read_index(PM_INDEX, reg);
    }
#[no_mangle]
unsafe extern "C" fn pm2_iowrite(reg: u8, value: u8) {
    static void pm2_iowrite(u8 reg, u8 value)
    {
    pmio_write_index(PM2_INDEX, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn pm2_ioread(reg: u8) -> u8 {
    static u8 pm2_ioread(u8 reg)
    {
    return pmio_read_index(PM2_INDEX, reg);
    }
#[no_mangle]
unsafe extern "C" fn acpi_hw_clear_status() {
    static void acpi_hw_clear_status(void)
    {
    u16 value;
// PMStatus: Clear WakeStatus/PwrBtnStatus
    value = inw(ACPI_PM_EVT_BLK);
    value |= (1 << 8 | 1 << 15);
    outw(value, ACPI_PM_EVT_BLK);
// GPEStatus: Clear all generated events
    outl(inl(ACPI_GPE0_BLK), ACPI_GPE0_BLK);
    }
#[no_mangle]
unsafe extern "C" fn acpi_registers_setup() {
    static void acpi_registers_setup(void)
    {
    u32 value;
// PM Status Base
    pm_iowrite(0x20, ACPI_PM_EVT_BLK & 0xff);
    pm_iowrite(0x21, ACPI_PM_EVT_BLK >> 8);
// PM Control Base
    pm_iowrite(0x22, ACPI_PM_CNT_BLK & 0xff);
    pm_iowrite(0x23, ACPI_PM_CNT_BLK >> 8);
// GPM Base
    pm_iowrite(0x28, ACPI_GPE0_BLK & 0xff);
    pm_iowrite(0x29, ACPI_GPE0_BLK >> 8);
// ACPI End
    pm_iowrite(0x2e, ACPI_END & 0xff);
    pm_iowrite(0x2f, ACPI_END >> 8);
// IO Decode: When AcpiDecodeEnable set, South-Bridge uses the contents
// of the PM registers at index 0x20~0x2B to decode ACPI I/O address.
    pm_iowrite(0x0e, 1 << 3);
// SCI_EN set
    outw(1, ACPI_PM_CNT_BLK);
// Enable to generate SCI
    pm_iowrite(0x10, pm_ioread(0x10) | 1);
// GPM3/GPM9 enable
    value = inl(ACPI_GPE0_BLK + 4);
    outl(value | (1 << 14) | (1 << 22), ACPI_GPE0_BLK + 4);
// Set GPM9 as input
    pm_iowrite(0x8d, pm_ioread(0x8d) & (~(1 << 1)));
// Set GPM9 as non-output
    pm_iowrite(0x94, pm_ioread(0x94) | (1 << 3));
// GPM3 config ACPI trigger SCIOUT
    pm_iowrite(0x33, pm_ioread(0x33) & (~(3 << 4)));
// GPM9 config ACPI trigger SCIOUT
    pm_iowrite(0x3d, pm_ioread(0x3d) & (~(3 << 2)));
// GPM3 config falling edge trigger
    pm_iowrite(0x37, pm_ioread(0x37) & (~(1 << 6)));
// No wait for STPGNT# in ACPI Sx state
    pm_iowrite(0x7c, pm_ioread(0x7c) | (1 << 6));
// Set GPM3 pull-down enable
    value = pm2_ioread(0xf6);
    value |= ((1 << 7) | (1 << 3));
    pm2_iowrite(0xf6, value);
// Set GPM9 pull-down enable
    value = pm2_ioread(0xf8);
    value |= ((1 << 5) | (1 << 1));
    pm2_iowrite(0xf8, value);
    }
#[no_mangle]
unsafe extern "C" fn rs780e_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int rs780e_acpi_probe(struct platform_device *pdev)
    {
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    return -ENODEV;
// SCI interrupt need acpi space, allocate here
    if (!request_region(res.start, resource_size(res), "acpi")) {
    pr_err("RS780E-ACPI: Failed to request IO Region\n");
    return -EBUSY;
    }
    acpi_iobase = res.start;
    acpi_registers_setup();
    acpi_hw_clear_status();
    return 0;
    }
    static const struct of_device_id rs780e_acpi_match[] = {
    { .compatible = "loongson,rs780e-acpi" },
    {},
    };
    static struct platform_driver rs780e_acpi_driver = {
    .probe = rs780e_acpi_probe,
    .driver = {
    .name = "RS780E-ACPI",
    .of_match_table = rs780e_acpi_match,
    },
    };
    builtin_platform_driver(rs780e_acpi_driver);
