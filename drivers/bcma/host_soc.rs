//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/host_soc.c
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
// Broadcom specific AMBA
// System on Chip (SoC) Host
//
// Licensed under the GNU/GPL. See COPYING for details.
//

#[no_mangle]
unsafe extern "C" fn bcma_host_soc_read8(core: *mut bcma_device, offset: u16) -> u8 {
    static u8 bcma_host_soc_read8(struct bcma_device *core, u16 offset)
    {
    return readb(core.io_addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_soc_read16(core: *mut bcma_device, offset: u16) -> u16 {
    static u16 bcma_host_soc_read16(struct bcma_device *core, u16 offset)
    {
    return readw(core.io_addr + offset);
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_soc_read32(core: *mut bcma_device, offset: u16) -> u32 {
    static u32 bcma_host_soc_read32(struct bcma_device *core, u16 offset)
    {
    return readl(core.io_addr + offset);
    }
    static void bcma_host_soc_write8(struct bcma_device *core, u16 offset,
    u8 value)
    {
    writeb(value, core.io_addr + offset);
    }
    static void bcma_host_soc_write16(struct bcma_device *core, u16 offset,
    u16 value)
    {
    writew(value, core.io_addr + offset);
    }
    static void bcma_host_soc_write32(struct bcma_device *core, u16 offset,
    u32 value)
    {
    writel(value, core.io_addr + offset);
    }

    static void bcma_host_soc_block_read(struct bcma_device *core, void *buffer,
    size_t count, u16 offset, u8 reg_width)
    {
    void __iomem *addr = core.io_addr + offset;
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
    static void bcma_host_soc_block_write(struct bcma_device *core,
    const void *buffer,
    size_t count, u16 offset, u8 reg_width)
    {
    void __iomem *addr = core.io_addr + offset;
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

#[no_mangle]
unsafe extern "C" fn bcma_host_soc_aread32(core: *mut bcma_device, offset: u16) -> u32 {
    static u32 bcma_host_soc_aread32(struct bcma_device *core, u16 offset)
    {
    if (WARN_ONCE(!core.io_wrap, "Accessed core has no wrapper/agent\n"))
    return ~0;
    return readl(core.io_wrap + offset);
    }
    static void bcma_host_soc_awrite32(struct bcma_device *core, u16 offset,
    u32 value)
    {
    if (WARN_ONCE(!core.io_wrap, "Accessed core has no wrapper/agent\n"))
    return;
    writel(value, core.io_wrap + offset);
    }
    static const struct bcma_host_ops bcma_host_soc_ops = {
    .read8		= bcma_host_soc_read8,
    .read16		= bcma_host_soc_read16,
    .read32		= bcma_host_soc_read32,
    .write8		= bcma_host_soc_write8,
    .write16	= bcma_host_soc_write16,
    .write32	= bcma_host_soc_write32,

    .block_read	= bcma_host_soc_block_read,
    .block_write	= bcma_host_soc_block_write,

    .aread32	= bcma_host_soc_aread32,
    .awrite32	= bcma_host_soc_awrite32,
    };
#[no_mangle]
pub unsafe extern "C" fn bcma_host_soc_register(soc: *mut bcma_soc) -> int __init {
    int __init bcma_host_soc_register(struct bcma_soc *soc)
    {
    struct bcma_bus *bus = &soc.bus;
// iomap only first core. We have to read some register on this core
// to scan the bus.
//
    bus.mmio = ioremap(BCMA_ADDR_BASE, BCMA_CORE_SIZE * 1);
    if (!bus.mmio)
    return -ENOMEM;
// Host specific
    bus.hosttype = BCMA_HOSTTYPE_SOC;
    bus.ops = &bcma_host_soc_ops;
// Initialize struct, detect chip
    bcma_init_bus(bus);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_host_soc_init(soc: *mut bcma_soc) -> int __init {
    int __init bcma_host_soc_init(struct bcma_soc *soc)
    {
    struct bcma_bus *bus = &soc.bus;
    int err;
// Scan bus and initialize it
    err = bcma_bus_early_register(bus);
    if (err)
    iounmap(bus.mmio);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn bcma_host_soc_probe(pdev: *mut platform_device) -> c_int {
    static int bcma_host_soc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct bcma_bus *bus;
    int err;
// Alloc
    bus = devm_kzalloc(dev, sizeof(*bus), GFP_KERNEL);
    if (!bus)
    return -ENOMEM;
    bus.dev = dev;
// Map MMIO
    bus.mmio = of_iomap(np, 0);
    if (!bus.mmio)
    return -ENOMEM;
// Host specific
    bus.hosttype = BCMA_HOSTTYPE_SOC;
    bus.ops = &bcma_host_soc_ops;
// Initialize struct, detect chip
    bcma_init_bus(bus);
// Register
    err = bcma_bus_register(bus);
    if (err)
    goto err_unmap_mmio;
    platform_set_drvdata(pdev, bus);
    return err;
    err_unmap_mmio:
    iounmap(bus.mmio);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_soc_remove(pdev: *mut platform_device) {
    static void bcma_host_soc_remove(struct platform_device *pdev)
    {
    struct bcma_bus *bus = platform_get_drvdata(pdev);
    bcma_bus_unregister(bus);
    iounmap(bus.mmio);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
    static const struct of_device_id bcma_host_soc_of_match[] = {
    { .compatible = "brcm,bus-axi", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcma_host_soc_of_match);
    static struct platform_driver bcma_host_soc_driver = {
    .driver = {
    .name = "bcma-host-soc",
    .of_match_table = bcma_host_soc_of_match,
    },
    .probe		= bcma_host_soc_probe,
    .remove		= bcma_host_soc_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn bcma_host_soc_register_driver() -> int __init {
    int __init bcma_host_soc_register_driver(void)
    {
    return platform_driver_register(&bcma_host_soc_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_host_soc_unregister_driver() -> void __exit {
    void __exit bcma_host_soc_unregister_driver(void)
    {
    platform_driver_unregister(&bcma_host_soc_driver);
    }
