//! Automatically rewritten from C to Rust
//! Source: drivers/misc/cb710/core.c
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
// cb710/core.c
//
// Copyright by Michał Mirosław, 2008-2009
//

    static DEFINE_IDA(cb710_ida);
    void cb710_pci_update_config_reg(struct pci_dev *pdev,
    int reg, uint32_t mask, uint32_t xor)
    {
    u32 rval;
    pci_read_config_dword(pdev, reg, &rval);
    rval = (rval & mask) ^ xor;
    pci_write_config_dword(pdev, reg, rval);
    }
    EXPORT_SYMBOL_GPL(cb710_pci_update_config_reg);
// Some magic writes based on Windows driver init code
#[no_mangle]
unsafe extern "C" fn cb710_pci_configure(pdev: *mut pci_dev) -> c_int {
    static int cb710_pci_configure(struct pci_dev *pdev)
    {
    let mut devfn: c_uint = PCI_DEVFN(PCI_SLOT(pdev.devfn), 0);
    struct pci_dev *pdev0;
    u32 val;
    cb710_pci_update_config_reg(pdev, 0x48,
    ~0x000000FF, 0x0000003F);
    pci_read_config_dword(pdev, 0x48, &val);
    if (val & 0x80000000)
    return 0;
    pdev0 = pci_get_slot(pdev.bus, devfn);
    if (!pdev0)
    return -ENODEV;
    if (pdev0.vendor == PCI_VENDOR_ID_ENE
    && pdev0.device == PCI_DEVICE_ID_ENE_720) {
    cb710_pci_update_config_reg(pdev0, 0x8C,
    ~0x00F00000, 0x00100000);
    cb710_pci_update_config_reg(pdev0, 0xB0,
    ~0x08000000, 0x08000000);
    }
    cb710_pci_update_config_reg(pdev0, 0x8C,
    ~0x00000F00, 0x00000200);
    cb710_pci_update_config_reg(pdev0, 0x90,
    ~0x00060000, 0x00040000);
    pci_dev_put(pdev0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cb710_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cb710_irq_handler(int irq, void *data)
    {
    struct cb710_chip *chip = data;
    struct cb710_slot *slot = &chip.slot[0];
    let mut handled: irqreturn_t = IRQ_NONE;
    unsigned nr;
    spin_lock(&chip.irq_lock); /* incl. smp_rmb() */
    for (nr = chip.slots; nr; ++slot, --nr) {
    let mut handler_func: cb710_irq_handler_t = slot.irq_handler;
    if (handler_func && handler_func(slot))
    handled = IRQ_HANDLED;
    }
    spin_unlock(&chip.irq_lock);
    return handled;
    }
#[no_mangle]
unsafe extern "C" fn cb710_release_slot(dev: *mut device) {
    static void cb710_release_slot(struct device *dev)
    {

    struct cb710_slot *slot = cb710_pdev_to_slot(to_platform_device(dev));
    struct cb710_chip *chip = cb710_slot_to_chip(slot);
// slot struct can be freed now
    atomic_dec(&chip.slot_refs_count);

    }
    static int cb710_register_slot(struct cb710_chip *chip,
    unsigned slot_mask, unsigned io_offset, const char *name)
    {
    let mut nr: c_int = chip.slots;
    struct cb710_slot *slot = &chip.slot[nr];
    int err;
    dev_dbg(cb710_chip_dev(chip),
    "register: %s.%d; slot %d; mask %d; IO offset: 0x%02X\n",
    name, chip.platform_id, nr, slot_mask, io_offset);
// slot->irq_handler == NULL here; this needs to be
// seen before platform_device_register()
    ++chip.slots;
    smp_wmb();
    slot.iobase = chip.iobase + io_offset;
    slot.pdev.name = name;
    slot.pdev.id = chip.platform_id;
    slot.pdev.dev.parent = &chip.pdev.dev;
    slot.pdev.dev.release = cb710_release_slot;
    err = platform_device_register(&slot.pdev);

    atomic_inc(&chip.slot_refs_count);

    if (err) {
// device_initialize() called from platform_device_register()
// wants this on error path
    platform_device_put(&slot.pdev);
// slot->irq_handler == NULL here anyway, so no lock needed
    --chip.slots;
    return err;
    }
    chip.slot_mask |= slot_mask;
    return 0;
    }
    static void cb710_unregister_slot(struct cb710_chip *chip,
    unsigned slot_mask)
    {
    let mut nr: c_int = chip.slots - 1;
    if (!(chip.slot_mask & slot_mask))
    return;
    platform_device_unregister(&chip.slot[nr].pdev);
// complementary to spin_unlock() in cb710_set_irq_handler()
    smp_rmb();
    BUG_ON(chip.slot[nr].irq_handler != core::ptr::null_mut());
// slot->irq_handler == NULL here, so no lock needed
    --chip.slots;
    chip.slot_mask &= ~slot_mask;
    }
    void cb710_set_irq_handler(struct cb710_slot *slot,
    cb710_irq_handler_t handler)
    {
    struct cb710_chip *chip = cb710_slot_to_chip(slot);
    unsigned long flags;
    spin_lock_irqsave(&chip.irq_lock, flags);
    slot.irq_handler = handler;
    spin_unlock_irqrestore(&chip.irq_lock, flags);
    }
    EXPORT_SYMBOL_GPL(cb710_set_irq_handler);
#[no_mangle]
unsafe extern "C" fn cb710_suspend(dev_d: *mut device) -> int __maybe_unused {
    static int __maybe_unused cb710_suspend(struct device *dev_d)
    {
    struct pci_dev *pdev = to_pci_dev(dev_d);
    struct cb710_chip *chip = pci_get_drvdata(pdev);
    devm_free_irq(&pdev.dev, pdev.irq, chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cb710_resume(dev_d: *mut device) -> int __maybe_unused {
    static int __maybe_unused cb710_resume(struct device *dev_d)
    {
    struct pci_dev *pdev = to_pci_dev(dev_d);
    struct cb710_chip *chip = pci_get_drvdata(pdev);
    return devm_request_irq(&pdev.dev, pdev.irq,
    cb710_irq_handler, IRQF_SHARED, KBUILD_MODNAME, chip);
    }
    static int cb710_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct cb710_chip *chip;
    u32 val;
    int err;
    let mut n: c_int = 0;
    err = cb710_pci_configure(pdev);
    if (err)
    return err;
// this is actually magic...
    pci_read_config_dword(pdev, 0x48, &val);
    if (!(val & 0x80000000)) {
    pci_write_config_dword(pdev, 0x48, val|0x71000000);
    pci_read_config_dword(pdev, 0x48, &val);
    }
    dev_dbg(&pdev.dev, "PCI config[0x48] = 0x%08X\n", val);
    if (!(val & 0x70000000))
    return -ENODEV;
    val = (val >> 28) & 7;
    if (val & CB710_SLOT_MMC)
    ++n;
    if (val & CB710_SLOT_MS)
    ++n;
    if (val & CB710_SLOT_SM)
    ++n;
    chip = devm_kzalloc(&pdev.dev, struct_size(chip, slot, n),
    GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    err = pcim_enable_device(pdev);
    if (err)
    return err;
    spin_lock_init(&chip.irq_lock);
    chip.pdev = pdev;
    chip.iobase = pcim_iomap_region(pdev, 0, KBUILD_MODNAME);
    if (IS_ERR(chip.iobase))
    return PTR_ERR(chip.iobase);
    pci_set_drvdata(pdev, chip);
    err = devm_request_irq(&pdev.dev, pdev.irq,
    cb710_irq_handler, IRQF_SHARED, KBUILD_MODNAME, chip);
    if (err)
    return err;
    err = ida_alloc(&cb710_ida, GFP_KERNEL);
    if (err < 0)
    return err;
    chip.platform_id = err;
    dev_info(&pdev.dev, "id %d, IO 0x%p, IRQ %d\n",
    chip.platform_id, chip.iobase, pdev.irq);
    if (val & CB710_SLOT_MMC) {	/* MMC/SD slot */
    err = cb710_register_slot(chip,
    CB710_SLOT_MMC, 0x00, "cb710-mmc");
    if (err)
    return err;
    }
    if (val & CB710_SLOT_MS) {	/* MemoryStick slot */
    err = cb710_register_slot(chip,
    CB710_SLOT_MS, 0x40, "cb710-ms");
    if (err)
    goto unreg_mmc;
    }
    if (val & CB710_SLOT_SM) {	/* SmartMedia slot */
    err = cb710_register_slot(chip,
    CB710_SLOT_SM, 0x60, "cb710-sm");
    if (err)
    goto unreg_ms;
    }
    return 0;
    unreg_ms:
    cb710_unregister_slot(chip, CB710_SLOT_MS);
    unreg_mmc:
    cb710_unregister_slot(chip, CB710_SLOT_MMC);

    BUG_ON(atomic_read(&chip.slot_refs_count) != 0);

    return err;
    }
#[no_mangle]
unsafe extern "C" fn cb710_remove_one(pdev: *mut pci_dev) {
    static void cb710_remove_one(struct pci_dev *pdev)
    {
    struct cb710_chip *chip = pci_get_drvdata(pdev);
    cb710_unregister_slot(chip, CB710_SLOT_SM);
    cb710_unregister_slot(chip, CB710_SLOT_MS);
    cb710_unregister_slot(chip, CB710_SLOT_MMC);

    BUG_ON(atomic_read(&chip.slot_refs_count) != 0);

    ida_free(&cb710_ida, chip.platform_id);
    }
    static const struct pci_device_id cb710_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_ENE, PCI_DEVICE_ID_ENE_CB710_FLASH) },
    { }
    };
    static SIMPLE_DEV_PM_OPS(cb710_pm_ops, cb710_suspend, cb710_resume);
    static struct pci_driver cb710_driver = {
    .name = KBUILD_MODNAME,
    .id_table = cb710_pci_tbl,
    .probe = cb710_probe,
    .remove = cb710_remove_one,
    .driver.pm = &cb710_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn cb710_init_module() -> int __init {
    static int __init cb710_init_module(void)
    {
    return pci_register_driver(&cb710_driver);
    }
#[no_mangle]
unsafe extern "C" fn cb710_cleanup_module() -> void __exit {
    static void __exit cb710_cleanup_module(void)
    {
    pci_unregister_driver(&cb710_driver);
    ida_destroy(&cb710_ida);
    }
    module_init(cb710_init_module);
    module_exit(cb710_cleanup_module);
    MODULE_AUTHOR("Michał Mirosław <mirq-linux@rere.qmqm.pl>");
    MODULE_DESCRIPTION("ENE CB710 memory card reader driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, cb710_pci_tbl);
