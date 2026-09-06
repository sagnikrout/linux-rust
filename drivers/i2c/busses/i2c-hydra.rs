//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-hydra.c
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
    i2c Support for the Apple `Hydra' Mac I/O
    Copyright (c) 1999-2004 Geert Uytterhoeven <geert@linux-m68k.org>
    Based on i2c Support for Via Technologies 82C586B South Bridge
    Copyright (c) 1998, 1999 Kyösti Mälkki <kmalkki@cc.hut.fi>
//

pub const HYDRA_CPD_PD0: c_uint = 0x00000001	/* CachePD lines */;
pub const HYDRA_CPD_PD1: c_uint = 0x00000002;
pub const HYDRA_CPD_PD2: c_uint = 0x00000004;
pub const HYDRA_CPD_PD3: c_uint = 0x00000008;

pub const HYDRA_SCLK_OE: c_uint = 0x00000010;
pub const HYDRA_SDAT_OE: c_uint = 0x00000020;
#[no_mangle]
pub unsafe extern "C" fn pdregw(data: *mut c_void, val: u32) {
    static inline void pdregw(void *data, u32 val)
    {
    struct Hydra *hydra = (struct Hydra *)data;
    writel(val, &hydra.CachePD);
    }
#[no_mangle]
pub unsafe extern "C" fn pdregr(data: *mut c_void) -> u32 {
    static inline u32 pdregr(void *data)
    {
    struct Hydra *hydra = (struct Hydra *)data;
    return readl(&hydra.CachePD);
    }
#[no_mangle]
unsafe extern "C" fn hydra_bit_setscl(data: *mut c_void, state: c_int) {
    static void hydra_bit_setscl(void *data, int state)
    {
    let mut val: u32 = pdregr(data);
    if (state)
    val &= ~HYDRA_SCLK_OE;
    else {
    val &= ~HYDRA_SCLK;
    val |= HYDRA_SCLK_OE;
    }
    pdregw(data, val);
    }
#[no_mangle]
unsafe extern "C" fn hydra_bit_setsda(data: *mut c_void, state: c_int) {
    static void hydra_bit_setsda(void *data, int state)
    {
    let mut val: u32 = pdregr(data);
    if (state)
    val &= ~HYDRA_SDAT_OE;
    else {
    val &= ~HYDRA_SDAT;
    val |= HYDRA_SDAT_OE;
    }
    pdregw(data, val);
    }
#[no_mangle]
unsafe extern "C" fn hydra_bit_getscl(data: *mut c_void) -> c_int {
    static int hydra_bit_getscl(void *data)
    {
    return (pdregr(data) & HYDRA_SCLK) != 0;
    }
#[no_mangle]
unsafe extern "C" fn hydra_bit_getsda(data: *mut c_void) -> c_int {
    static int hydra_bit_getsda(void *data)
    {
    return (pdregr(data) & HYDRA_SDAT) != 0;
    }
// ------------------------------------------------------------------------
    static struct i2c_algo_bit_data hydra_bit_data = {
    .setsda		= hydra_bit_setsda,
    .setscl		= hydra_bit_setscl,
    .getsda		= hydra_bit_getsda,
    .getscl		= hydra_bit_getscl,
    .udelay		= 5,
    .timeout	= HZ
    };
    static struct i2c_adapter hydra_adap = {
    .owner		= THIS_MODULE,
    .name		= "Hydra i2c",
    .algo_data	= &hydra_bit_data,
    };
    static const struct pci_device_id hydra_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_APPLE, PCI_DEVICE_ID_APPLE_HYDRA) },
    { 0, }
    };
    MODULE_DEVICE_TABLE (pci, hydra_ids);
    static int hydra_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    let mut base: c_ulong = pci_resource_start(dev, 0);
    int res;
    if (!request_mem_region(base+offsetof(struct Hydra, CachePD), 4,
    hydra_adap.name))
    return -EBUSY;
    hydra_bit_data.data = pci_ioremap_bar(dev, 0);
    if (hydra_bit_data.data == core::ptr::null_mut()) {
    release_mem_region(base+offsetof(struct Hydra, CachePD), 4);
    return -ENODEV;
    }
    pdregw(hydra_bit_data.data, 0);		/* clear SCLK_OE and SDAT_OE */
    hydra_adap.dev.parent = &dev.dev;
    res = i2c_bit_add_bus(&hydra_adap);
    if (res < 0) {
    iounmap(hydra_bit_data.data);
    release_mem_region(base+offsetof(struct Hydra, CachePD), 4);
    return res;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hydra_remove(dev: *mut pci_dev) {
    static void hydra_remove(struct pci_dev *dev)
    {
    pdregw(hydra_bit_data.data, 0);		/* clear SCLK_OE and SDAT_OE */
    i2c_del_adapter(&hydra_adap);
    iounmap(hydra_bit_data.data);
    release_mem_region(pci_resource_start(dev, 0)+
    offsetof(struct Hydra, CachePD), 4);
    }
    static struct pci_driver hydra_driver = {
    .name		= "hydra_smbus",
    .id_table	= hydra_ids,
    .probe		= hydra_probe,
    .remove		= hydra_remove,
    };
    module_pci_driver(hydra_driver);
    MODULE_AUTHOR("Geert Uytterhoeven <geert@linux-m68k.org>");
    MODULE_DESCRIPTION("i2c for Apple Hydra Mac I/O");
    MODULE_LICENSE("GPL");
