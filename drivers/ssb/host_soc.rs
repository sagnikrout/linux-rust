//! Automatically rewritten from C to Rust
//! Source: drivers/ssb/host_soc.c
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


//
// Sonics Silicon Backplane SoC host related functions.
// Subsystem core
//
// Copyright 2005, Broadcom Corporation
// Copyright 2006, 2007, Michael Buesch <m@bues.ch>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

#[no_mangle]
unsafe extern "C" fn ssb_host_soc_read8(dev: *mut ssb_device, offset: u16) -> u8 {
    static u8 ssb_host_soc_read8(struct ssb_device *dev, u16 offset)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    return readb(bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_soc_read16(dev: *mut ssb_device, offset: u16) -> u16 {
    static u16 ssb_host_soc_read16(struct ssb_device *dev, u16 offset)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    return readw(bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_soc_read32(dev: *mut ssb_device, offset: u16) -> u32 {
    static u32 ssb_host_soc_read32(struct ssb_device *dev, u16 offset)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    return readl(bus.mmio + offset);
    }

    static void ssb_host_soc_block_read(struct ssb_device *dev, void *buffer,
    size_t count, u16 offset, u8 reg_width)
    {
    struct ssb_bus *bus = dev.bus;
    void __iomem *addr;
    offset += dev.core_index * SSB_CORE_SIZE;
    addr = bus.mmio + offset;
    switch (reg_width) {
    case sizeof(u8): {
    u8 *buf = buffer;
    while (count) {
// buf = __raw_readb(addr);
    buf++;
    count--;
    }
    break;
    }
    case sizeof(u16): {
    __le16 *buf = buffer;
    WARN_ON(count & 1);
    while (count) {
// buf = ( __le16)__raw_readw(addr);
    buf++;
    count -= 2;
    }
    break;
    }
    case sizeof(u32): {
    __le32 *buf = buffer;
    WARN_ON(count & 3);
    while (count) {
// buf = ( __le32)__raw_readl(addr);
    buf++;
    count -= 4;
    }
    break;
    }
    default:
    WARN_ON(1);
    }
    }

#[no_mangle]
unsafe extern "C" fn ssb_host_soc_write8(dev: *mut ssb_device, offset: u16, value: u8) {
    static void ssb_host_soc_write8(struct ssb_device *dev, u16 offset, u8 value)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    writeb(value, bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_soc_write16(dev: *mut ssb_device, offset: u16, value: u16) {
    static void ssb_host_soc_write16(struct ssb_device *dev, u16 offset, u16 value)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    writew(value, bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_soc_write32(dev: *mut ssb_device, offset: u16, value: u32) {
    static void ssb_host_soc_write32(struct ssb_device *dev, u16 offset, u32 value)
    {
    struct ssb_bus *bus = dev.bus;
    offset += dev.core_index * SSB_CORE_SIZE;
    writel(value, bus.mmio + offset);
    }

    static void ssb_host_soc_block_write(struct ssb_device *dev, const void *buffer,
    size_t count, u16 offset, u8 reg_width)
    {
    struct ssb_bus *bus = dev.bus;
    void __iomem *addr;
    offset += dev.core_index * SSB_CORE_SIZE;
    addr = bus.mmio + offset;
    switch (reg_width) {
    case sizeof(u8): {
    const u8 *buf = buffer;
    while (count) {
    __raw_writeb(*buf, addr);
    buf++;
    count--;
    }
    break;
    }
    case sizeof(u16): {
    const __le16 *buf = buffer;
    WARN_ON(count & 1);
    while (count) {
    __raw_writew(( u16)(*buf), addr);
    buf++;
    count -= 2;
    }
    break;
    }
    case sizeof(u32): {
    const __le32 *buf = buffer;
    WARN_ON(count & 3);
    while (count) {
    __raw_writel(( u32)(*buf), addr);
    buf++;
    count -= 4;
    }
    break;
    }
    default:
    WARN_ON(1);
    }
    }

// Ops for the plain SSB bus without a host-device (no PCI or PCMCIA).
    const struct ssb_bus_ops ssb_host_soc_ops = {
    .read8		= ssb_host_soc_read8,
    .read16		= ssb_host_soc_read16,
    .read32		= ssb_host_soc_read32,
    .write8		= ssb_host_soc_write8,
    .write16	= ssb_host_soc_write16,
    .write32	= ssb_host_soc_write32,

    .block_read	= ssb_host_soc_block_read,
    .block_write	= ssb_host_soc_block_write,

    };
    int ssb_host_soc_get_invariants(struct ssb_bus *bus,
    struct ssb_init_invariants *iv)
    {
    char buf[20];
    int len, err;
// Fill boardinfo structure
    memset(&iv.boardinfo, 0, sizeof(struct ssb_boardinfo));
    len = bcm47xx_nvram_getenv("boardvendor", buf, sizeof(buf));
    if (len > 0) {
    err = kstrtou16(strim(buf), 0, &iv.boardinfo.vendor);
    if (err)
    pr_warn("Couldn't parse nvram board vendor entry with value \"%s\"\n",
    buf);
    }
    if (!iv.boardinfo.vendor)
    iv.boardinfo.vendor = SSB_BOARDVENDOR_BCM;
    len = bcm47xx_nvram_getenv("boardtype", buf, sizeof(buf));
    if (len > 0) {
    err = kstrtou16(strim(buf), 0, &iv.boardinfo.type);
    if (err)
    pr_warn("Couldn't parse nvram board type entry with value \"%s\"\n",
    buf);
    }
    memset(&iv.sprom, 0, sizeof(struct ssb_sprom));
    ssb_fill_sprom_with_fallback(bus, &iv.sprom);
    if (bcm47xx_nvram_getenv("cardbus", buf, sizeof(buf)) >= 0)
    iv.has_cardbus_slot = !!simple_strtoul(buf, core::ptr::null_mut(), 10);
    return 0;
    }
