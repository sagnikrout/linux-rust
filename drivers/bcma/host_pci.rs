//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/host_pci.c
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
// PCI Host
//
// Licensed under the GNU/GPL. See COPYING for details.
//

#[no_mangle]
unsafe extern "C" fn bcma_host_pci_switch_core(core: *mut bcma_device) {
    static void bcma_host_pci_switch_core(struct bcma_device *core)
    {
    int win2 = core.bus.host_is_pcie2 ?
    BCMA_PCIE2_BAR0_WIN2 : BCMA_PCI_BAR0_WIN2;
    pci_write_config_dword(core.bus.host_pci, BCMA_PCI_BAR0_WIN,
    core.addr);
    pci_write_config_dword(core.bus.host_pci, win2, core.wrap);
    core.bus.mapped_core = core;
    bcma_debug(core.bus, "Switched to core: 0x%X\n", core.id.id);
    }
// Provides access to the requested core. Returns base offset that has to be
// used. It makes use of fixed windows when possible.
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_provide_access_to_core(core: *mut bcma_device) -> u16 {
    static u16 bcma_host_pci_provide_access_to_core(struct bcma_device *core)
    {
    switch (core.id.id) {
    case BCMA_CORE_CHIPCOMMON:
    return 3 * BCMA_CORE_SIZE;
    case BCMA_CORE_PCIE:
    return 2 * BCMA_CORE_SIZE;
    }
    if (core.bus.mapped_core != core)
    bcma_host_pci_switch_core(core);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_read8(core: *mut bcma_device, offset: u16) -> u8 {
    static u8 bcma_host_pci_read8(struct bcma_device *core, u16 offset)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    return ioread8(core.bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_read16(core: *mut bcma_device, offset: u16) -> u16 {
    static u16 bcma_host_pci_read16(struct bcma_device *core, u16 offset)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    return ioread16(core.bus.mmio + offset);
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_read32(core: *mut bcma_device, offset: u16) -> u32 {
    static u32 bcma_host_pci_read32(struct bcma_device *core, u16 offset)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    return ioread32(core.bus.mmio + offset);
    }
    static void bcma_host_pci_write8(struct bcma_device *core, u16 offset,
    u8 value)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    iowrite8(value, core.bus.mmio + offset);
    }
    static void bcma_host_pci_write16(struct bcma_device *core, u16 offset,
    u16 value)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    iowrite16(value, core.bus.mmio + offset);
    }
    static void bcma_host_pci_write32(struct bcma_device *core, u16 offset,
    u32 value)
    {
    offset += bcma_host_pci_provide_access_to_core(core);
    iowrite32(value, core.bus.mmio + offset);
    }

    static void bcma_host_pci_block_read(struct bcma_device *core, void *buffer,
    size_t count, u16 offset, u8 reg_width)
    {
    void __iomem *addr = core.bus.mmio + offset;
    if (core.bus.mapped_core != core)
    bcma_host_pci_switch_core(core);
    switch (reg_width) {
    case sizeof(u8):
    ioread8_rep(addr, buffer, count);
    break;
    case sizeof(u16):
    WARN_ON(count & 1);
    ioread16_rep(addr, buffer, count >> 1);
    break;
    case sizeof(u32):
    WARN_ON(count & 3);
    ioread32_rep(addr, buffer, count >> 2);
    break;
    default:
    WARN_ON(1);
    }
    }
    static void bcma_host_pci_block_write(struct bcma_device *core,
    const void *buffer, size_t count,
    u16 offset, u8 reg_width)
    {
    void __iomem *addr = core.bus.mmio + offset;
    if (core.bus.mapped_core != core)
    bcma_host_pci_switch_core(core);
    switch (reg_width) {
    case sizeof(u8):
    iowrite8_rep(addr, buffer, count);
    break;
    case sizeof(u16):
    WARN_ON(count & 1);
    iowrite16_rep(addr, buffer, count >> 1);
    break;
    case sizeof(u32):
    WARN_ON(count & 3);
    iowrite32_rep(addr, buffer, count >> 2);
    break;
    default:
    WARN_ON(1);
    }
    }

#[no_mangle]
unsafe extern "C" fn bcma_host_pci_aread32(core: *mut bcma_device, offset: u16) -> u32 {
    static u32 bcma_host_pci_aread32(struct bcma_device *core, u16 offset)
    {
    if (core.bus.mapped_core != core)
    bcma_host_pci_switch_core(core);
    return ioread32(core.bus.mmio + (1 * BCMA_CORE_SIZE) + offset);
    }
    static void bcma_host_pci_awrite32(struct bcma_device *core, u16 offset,
    u32 value)
    {
    if (core.bus.mapped_core != core)
    bcma_host_pci_switch_core(core);
    iowrite32(value, core.bus.mmio + (1 * BCMA_CORE_SIZE) + offset);
    }
    static const struct bcma_host_ops bcma_host_pci_ops = {
    .read8		= bcma_host_pci_read8,
    .read16		= bcma_host_pci_read16,
    .read32		= bcma_host_pci_read32,
    .write8		= bcma_host_pci_write8,
    .write16	= bcma_host_pci_write16,
    .write32	= bcma_host_pci_write32,

    .block_read	= bcma_host_pci_block_read,
    .block_write	= bcma_host_pci_block_write,

    .aread32	= bcma_host_pci_aread32,
    .awrite32	= bcma_host_pci_awrite32,
    };
    static int bcma_host_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct bcma_bus *bus;
    let mut err: c_int = -ENOMEM;
    u32 val;
// Alloc
    bus = kzalloc_obj(*bus);
    if (!bus)
    goto out;
// Basic PCI configuration
    err = pci_enable_device(dev);
    if (err)
    goto err_kfree_bus;
    err = pci_request_regions(dev, "bcma-pci-bridge");
    if (err)
    goto err_pci_disable;
    pci_set_master(dev);
// Disable the RETRY_TIMEOUT register (0x41) to keep
// PCI Tx retries from interfering with C3 CPU state
    pci_read_config_dword(dev, 0x40, &val);
    if ((val & 0x0000ff00) != 0)
    pci_write_config_dword(dev, 0x40, val & 0xffff00ff);
// SSB needed additional powering up, do we have any AMBA PCI cards?
    if (!pci_is_pcie(dev)) {
    bcma_err(bus, "PCI card detected, they are not supported.\n");
    err = -ENXIO;
    goto err_pci_release_regions;
    }
    bus.dev = &dev.dev;
// Map MMIO
    err = -ENOMEM;
    bus.mmio = pci_iomap(dev, 0, ~0UL);
    if (!bus.mmio)
    goto err_pci_release_regions;
// Host specific
    bus.host_pci = dev;
    bus.hosttype = BCMA_HOSTTYPE_PCI;
    bus.ops = &bcma_host_pci_ops;
    bus.boardinfo.vendor = bus.host_pci.subsystem_vendor;
    bus.boardinfo.type = bus.host_pci.subsystem_device;
// Initialize struct, detect chip
    bcma_init_bus(bus);
// Scan bus to find out generation of PCIe core
    err = bcma_bus_scan(bus);
    if (err)
    goto err_pci_unmap_mmio;
    if (bcma_find_core(bus, BCMA_CORE_PCIE2))
    bus.host_is_pcie2 = true;
// Register
    err = bcma_bus_register(bus);
    if (err)
    goto err_unregister_cores;
    pci_set_drvdata(dev, bus);
    out:
    return err;
    err_unregister_cores:
    bcma_unregister_cores(bus);
    err_pci_unmap_mmio:
    pci_iounmap(dev, bus.mmio);
    err_pci_release_regions:
    pci_release_regions(dev);
    err_pci_disable:
    pci_disable_device(dev);
    err_kfree_bus:
    kfree(bus);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_remove(dev: *mut pci_dev) {
    static void bcma_host_pci_remove(struct pci_dev *dev)
    {
    struct bcma_bus *bus = pci_get_drvdata(dev);
    bcma_bus_unregister(bus);
    pci_iounmap(dev, bus.mmio);
    pci_release_regions(dev);
    pci_disable_device(dev);
    kfree(bus);
    }

#[no_mangle]
unsafe extern "C" fn bcma_host_pci_suspend(dev: *mut device) -> c_int {
    static int bcma_host_pci_suspend(struct device *dev)
    {
    struct bcma_bus *bus = dev_get_drvdata(dev);
    bus.mapped_core = core::ptr::null_mut();
    return bcma_bus_suspend(bus);
    }
#[no_mangle]
unsafe extern "C" fn bcma_host_pci_resume(dev: *mut device) -> c_int {
    static int bcma_host_pci_resume(struct device *dev)
    {
    struct bcma_bus *bus = dev_get_drvdata(dev);
    return bcma_bus_resume(bus);
    }
    static SIMPLE_DEV_PM_OPS(bcma_pm_ops, bcma_host_pci_suspend,
    bcma_host_pci_resume);

    static const struct pci_device_id bcma_pci_bridge_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x0576) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4313) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 43224) },	/* 0xa8d8 */
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4331) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4353) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4357) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4358) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4359) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4360) },
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_DELL, 0x0016) },
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_DELL, 0x0018) },
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_FOXCONN, 0xe092) },
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_BROADCOM, 0x4365, PCI_VENDOR_ID_HP, 0x804a) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x43a0) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x43a9) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x43aa) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x43b1) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4727) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 43227) },	/* 0xa8db, BCM43217 (sic!) */
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 43228) },	/* 0xa8dc */
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, bcma_pci_bridge_tbl);
    static struct pci_driver bcma_pci_bridge_driver = {
    .name = "bcma-pci-bridge",
    .id_table = bcma_pci_bridge_tbl,
    .probe = bcma_host_pci_probe,
    .remove = bcma_host_pci_remove,
    .driver.pm = BCMA_PM_OPS,
    };
#[no_mangle]
pub unsafe extern "C" fn bcma_host_pci_init() -> int __init {
    int __init bcma_host_pci_init(void)
    {
    return pci_register_driver(&bcma_pci_bridge_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_host_pci_exit() -> void __exit {
    void __exit bcma_host_pci_exit(void)
    {
    pci_unregister_driver(&bcma_pci_bridge_driver);
    }
//
// Runtime ops for drivers.
//
// See also pcicore_up
#[no_mangle]
pub unsafe extern "C" fn bcma_host_pci_up(bus: *mut bcma_bus) {
    void bcma_host_pci_up(struct bcma_bus *bus)
    {
    if (bus.hosttype != BCMA_HOSTTYPE_PCI)
    return;
    if (bus.host_is_pcie2)
    bcma_core_pcie2_up(&bus.drv_pcie2);
    else
    bcma_core_pci_up(&bus.drv_pci[0]);
    }
    EXPORT_SYMBOL_GPL(bcma_host_pci_up);
// See also pcicore_down
#[no_mangle]
pub unsafe extern "C" fn bcma_host_pci_down(bus: *mut bcma_bus) {
    void bcma_host_pci_down(struct bcma_bus *bus)
    {
    if (bus.hosttype != BCMA_HOSTTYPE_PCI)
    return;
    if (!bus.host_is_pcie2)
    bcma_core_pci_down(&bus.drv_pci[0]);
    }
    EXPORT_SYMBOL_GPL(bcma_host_pci_down);
// See also si_pci_setup
    int bcma_host_pci_irq_ctl(struct bcma_bus *bus, struct bcma_device *core,
    bool enable)
    {
    struct pci_dev *pdev;
    u32 coremask, tmp;
    let mut err: c_int = 0;
    if (bus.hosttype != BCMA_HOSTTYPE_PCI) {
// This bcma device is not on a PCI host-bus. So the IRQs are
// not routed through the PCI core.
// So we must not enable routing through the PCI core.
    goto out;
    }
    pdev = bus.host_pci;
    err = pci_read_config_dword(pdev, BCMA_PCI_IRQMASK, &tmp);
    if (err)
    goto out;
    coremask = BIT(core.core_index) << 8;
    if (enable)
    tmp |= coremask;
    else
    tmp &= ~coremask;
    err = pci_write_config_dword(pdev, BCMA_PCI_IRQMASK, tmp);
    out:
    return err;
    }
    EXPORT_SYMBOL_GPL(bcma_host_pci_irq_ctl);
