//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/octep_hp.c
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
// Copyright (C) 2024 Marvell.

//
// Type of MSI-X interrupts. OCTEP_HP_INTR_VECTOR() and
// OCTEP_HP_INTR_OFFSET() generate the vector and offset for an interrupt
// type.
//
    enum octep_hp_intr_type {
    OCTEP_HP_INTR_INVALID = -1,
    OCTEP_HP_INTR_ENA = 0,
    OCTEP_HP_INTR_DIS = 1,
    OCTEP_HP_INTR_MAX = 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hp_cmd {
    pub list: list_head,
    pub intr_type: enum octep_hp_intr_type,
    pub intr_val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hp_slot {
    pub list: list_head,
    pub slot: hotplug_slot,
    pub slot_number: u16,
    pub hp_pdev: *mut pci_dev,
    pub hp_devfn: c_uint,
    pub ctrl: *mut octep_hp_controller,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hp_intr_info {
    pub type: enum octep_hp_intr_type,
    pub number: c_int,
    pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hp_controller {
    pub base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub intr: [octep_hp_intr_info; OCTEP_HP_INTR_MAX],
    pub work: work_struct,
    pub slot_list: list_head,
    pub /: *mut *mut mutex slot_lock; / Protects slot_list,
    pub hp_cmd_list: list_head,
    pub /: *mut *mut spinlock_t hp_cmd_lock; / Protects hp_cmd_list,
}

    static void octep_hp_enable_pdev(struct octep_hp_controller *hp_ctrl,
    struct octep_hp_slot *hp_slot)
    {
    guard(mutex)(&hp_ctrl.slot_lock);
    if (hp_slot.hp_pdev) {
    pci_dbg(hp_slot.hp_pdev, "Slot %s is already enabled\n",
    hotplug_slot_name(&hp_slot.slot));
    return;
    }
// Scan the device and add it to the bus
    hp_slot.hp_pdev = pci_scan_single_device(hp_ctrl.pdev.bus,
    hp_slot.hp_devfn);
    pci_bus_assign_resources(hp_ctrl.pdev.bus);
    pci_bus_add_device(hp_slot.hp_pdev);
    dev_dbg(&hp_slot.hp_pdev.dev, "Enabled slot %s\n",
    hotplug_slot_name(&hp_slot.slot));
    }
    static void octep_hp_disable_pdev(struct octep_hp_controller *hp_ctrl,
    struct octep_hp_slot *hp_slot)
    {
    guard(mutex)(&hp_ctrl.slot_lock);
    if (!hp_slot.hp_pdev) {
    pci_dbg(hp_ctrl.pdev, "Slot %s is already disabled\n",
    hotplug_slot_name(&hp_slot.slot));
    return;
    }
    pci_dbg(hp_slot.hp_pdev, "Disabling slot %s\n",
    hotplug_slot_name(&hp_slot.slot));
// Remove the device from the bus
    pci_stop_and_remove_bus_device_locked(hp_slot.hp_pdev);
    hp_slot.hp_pdev = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_enable_slot(slot: *mut hotplug_slot) -> c_int {
    static int octep_hp_enable_slot(struct hotplug_slot *slot)
    {
    struct octep_hp_slot *hp_slot =
    container_of(slot, struct octep_hp_slot, slot);
    octep_hp_enable_pdev(hp_slot.ctrl, hp_slot);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_disable_slot(slot: *mut hotplug_slot) -> c_int {
    static int octep_hp_disable_slot(struct hotplug_slot *slot)
    {
    struct octep_hp_slot *hp_slot =
    container_of(slot, struct octep_hp_slot, slot);
    octep_hp_disable_pdev(hp_slot.ctrl, hp_slot);
    return 0;
    }
    static struct hotplug_slot_ops octep_hp_slot_ops = {
    .enable_slot = octep_hp_enable_slot,
    .disable_slot = octep_hp_disable_slot,
    };
pub const SLOT_NAME_SIZE: c_int = 16;
    static struct octep_hp_slot *
    octep_hp_register_slot(struct octep_hp_controller *hp_ctrl,
    struct pci_dev *pdev, u16 slot_number)
    {
    char slot_name[SLOT_NAME_SIZE];
    struct octep_hp_slot *hp_slot;
    int ret;
    hp_slot = kzalloc_obj(*hp_slot);
    if (!hp_slot)
    return ERR_PTR(-ENOMEM);
    hp_slot.ctrl = hp_ctrl;
    hp_slot.hp_pdev = pdev;
    hp_slot.hp_devfn = pdev.devfn;
    hp_slot.slot_number = slot_number;
    hp_slot.slot.ops = &octep_hp_slot_ops;
    snprintf(slot_name, sizeof(slot_name), "octep_hp_%u", slot_number);
    ret = pci_hp_register(&hp_slot.slot, hp_ctrl.pdev.bus,
    PCI_SLOT(pdev.devfn), slot_name);
    if (ret) {
    kfree(hp_slot);
    return ERR_PTR(ret);
    }
    pci_info(pdev, "Registered slot %s for device %s\n",
    slot_name, pci_name(pdev));
    list_add_tail(&hp_slot.list, &hp_ctrl.slot_list);
    octep_hp_disable_pdev(hp_ctrl, hp_slot);
    return hp_slot;
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_deregister_slot(data: *mut c_void) {
    static void octep_hp_deregister_slot(void *data)
    {
    struct octep_hp_slot *hp_slot = data;
    struct octep_hp_controller *hp_ctrl = hp_slot.ctrl;
    pci_hp_deregister(&hp_slot.slot);
    octep_hp_enable_pdev(hp_ctrl, hp_slot);
    list_del(&hp_slot.list);
    kfree(hp_slot);
    }
    static const char *octep_hp_cmd_name(enum octep_hp_intr_type type)
    {
    switch (type) {
    case OCTEP_HP_INTR_ENA:
    return "hotplug enable";
    case OCTEP_HP_INTR_DIS:
    return "hotplug disable";
    default:
    return "invalid";
    }
    }
    static void octep_hp_cmd_handler(struct octep_hp_controller *hp_ctrl,
    struct octep_hp_cmd *hp_cmd)
    {
    struct octep_hp_slot *hp_slot;
//
// Enable or disable the slots based on the slot mask.
// intr_val is a bit mask where each bit represents a slot.
//
    list_for_each_entry(hp_slot, &hp_ctrl.slot_list, list) {
    if (!(hp_cmd.intr_val & BIT(hp_slot.slot_number)))
    continue;
    pci_info(hp_ctrl.pdev, "Received %s command for slot %s\n",
    octep_hp_cmd_name(hp_cmd.intr_type),
    hotplug_slot_name(&hp_slot.slot));
    switch (hp_cmd.intr_type) {
    case OCTEP_HP_INTR_ENA:
    octep_hp_enable_pdev(hp_ctrl, hp_slot);
    break;
    case OCTEP_HP_INTR_DIS:
    octep_hp_disable_pdev(hp_ctrl, hp_slot);
    break;
    default:
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_work_handler(work: *mut work_struct) {
    static void octep_hp_work_handler(struct work_struct *work)
    {
    struct octep_hp_controller *hp_ctrl;
    struct octep_hp_cmd *hp_cmd;
    unsigned long flags;
    hp_ctrl = container_of(work, struct octep_hp_controller, work);
// Process all the hotplug commands
    spin_lock_irqsave(&hp_ctrl.hp_cmd_lock, flags);
    while (!list_empty(&hp_ctrl.hp_cmd_list)) {
    hp_cmd = list_first_entry(&hp_ctrl.hp_cmd_list,
    struct octep_hp_cmd, list);
    list_del(&hp_cmd.list);
    spin_unlock_irqrestore(&hp_ctrl.hp_cmd_lock, flags);
    octep_hp_cmd_handler(hp_ctrl, hp_cmd);
    kfree(hp_cmd);
    spin_lock_irqsave(&hp_ctrl.hp_cmd_lock, flags);
    }
    spin_unlock_irqrestore(&hp_ctrl.hp_cmd_lock, flags);
    }
    static enum octep_hp_intr_type octep_hp_intr_type(struct octep_hp_intr_info *intr,
    int irq)
    {
    enum octep_hp_intr_type type;
    for (type = OCTEP_HP_INTR_ENA; type < OCTEP_HP_INTR_MAX; type++) {
    if (intr[type].number == irq)
    return type;
    }
    return OCTEP_HP_INTR_INVALID;
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_intr_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t octep_hp_intr_handler(int irq, void *data)
    {
    struct octep_hp_controller *hp_ctrl = data;
    struct pci_dev *pdev = hp_ctrl.pdev;
    enum octep_hp_intr_type type;
    struct octep_hp_cmd *hp_cmd;
    u64 intr_val;
    type = octep_hp_intr_type(hp_ctrl.intr, irq);
    if (type == OCTEP_HP_INTR_INVALID) {
    pci_err(pdev, "Invalid interrupt %d\n", irq);
    return IRQ_HANDLED;
    }
// Read and clear the interrupt
    intr_val = readq(hp_ctrl.base + OCTEP_HP_INTR_OFFSET(type));
    writeq(intr_val, hp_ctrl.base + OCTEP_HP_INTR_OFFSET(type));
    hp_cmd = kzalloc_obj(*hp_cmd, GFP_ATOMIC);
    if (!hp_cmd)
    return IRQ_HANDLED;
    hp_cmd.intr_val = intr_val;
    hp_cmd.intr_type = type;
// Add the command to the list and schedule the work
    spin_lock(&hp_ctrl.hp_cmd_lock);
    list_add_tail(&hp_cmd.list, &hp_ctrl.hp_cmd_list);
    spin_unlock(&hp_ctrl.hp_cmd_lock);
    schedule_work(&hp_ctrl.work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn octep_hp_irq_cleanup(data: *mut c_void) {
    static void octep_hp_irq_cleanup(void *data)
    {
    struct octep_hp_controller *hp_ctrl = data;
    pci_free_irq_vectors(hp_ctrl.pdev);
    flush_work(&hp_ctrl.work);
    }
    static int octep_hp_request_irq(struct octep_hp_controller *hp_ctrl,
    enum octep_hp_intr_type type)
    {
    struct pci_dev *pdev = hp_ctrl.pdev;
    struct octep_hp_intr_info *intr;
    int irq;
    irq = pci_irq_vector(pdev, OCTEP_HP_INTR_VECTOR(type));
    if (irq < 0)
    return irq;
    intr = &hp_ctrl.intr[type];
    intr.number = irq;
    intr.type = type;
    snprintf(intr.name, sizeof(intr.name), "octep_hp_%d", type);
    return devm_request_irq(&pdev.dev, irq, octep_hp_intr_handler,
    IRQF_SHARED, intr.name, hp_ctrl);
    }
    static int octep_hp_controller_setup(struct pci_dev *pdev,
    struct octep_hp_controller *hp_ctrl)
    {
    struct device *dev = &pdev.dev;
    enum octep_hp_intr_type type;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable PCI device\n");
    hp_ctrl.base = pcim_iomap_region(pdev, 0, OCTEP_HP_DRV_NAME);
    if (IS_ERR(hp_ctrl.base))
    return dev_err_probe(dev, PTR_ERR(hp_ctrl.base),
    "Failed to map PCI device region\n");
    pci_set_master(pdev);
    pci_set_drvdata(pdev, hp_ctrl);
    INIT_LIST_HEAD(&hp_ctrl.slot_list);
    INIT_LIST_HEAD(&hp_ctrl.hp_cmd_list);
    mutex_init(&hp_ctrl.slot_lock);
    spin_lock_init(&hp_ctrl.hp_cmd_lock);
    INIT_WORK(&hp_ctrl.work, octep_hp_work_handler);
    hp_ctrl.pdev = pdev;
    ret = pci_alloc_irq_vectors(pdev, 1,
    OCTEP_HP_INTR_VECTOR(OCTEP_HP_INTR_MAX),
    PCI_IRQ_MSIX);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to alloc MSI-X vectors\n");
    ret = devm_add_action(&pdev.dev, octep_hp_irq_cleanup, hp_ctrl);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to add IRQ cleanup action\n");
    for (type = OCTEP_HP_INTR_ENA; type < OCTEP_HP_INTR_MAX; type++) {
    ret = octep_hp_request_irq(hp_ctrl, type);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to request IRQ for vector %d\n",
    OCTEP_HP_INTR_VECTOR(type));
    }
    return 0;
    }
    static int octep_hp_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct octep_hp_controller *hp_ctrl;
    struct pci_dev *tmp_pdev, *next;
    struct octep_hp_slot *hp_slot;
    let mut slot_number: u16 = 0;
    int ret;
    hp_ctrl = devm_kzalloc(&pdev.dev, sizeof(*hp_ctrl), GFP_KERNEL);
    if (!hp_ctrl)
    return -ENOMEM;
    ret = octep_hp_controller_setup(pdev, hp_ctrl);
    if (ret)
    return ret;
//
// Register all hotplug slots. Hotplug controller is the first function
// of the PCI device. The hotplug slots are the remaining functions of
// the PCI device. The hotplug slot functions are logically removed from
// the bus during probing and are re-enabled by the driver when a
// hotplug event is received.
//
    list_for_each_entry_safe(tmp_pdev, next, &pdev.bus.devices, bus_list) {
    if (tmp_pdev == pdev)
    continue;
    hp_slot = octep_hp_register_slot(hp_ctrl, tmp_pdev, slot_number);
    if (IS_ERR(hp_slot))
    return dev_err_probe(&pdev.dev, PTR_ERR(hp_slot),
    "Failed to register hotplug slot %u\n",
    slot_number);
    ret = devm_add_action(&pdev.dev, octep_hp_deregister_slot,
    hp_slot);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to add action for deregistering slot %u\n",
    slot_number);
    slot_number++;
    }
    return 0;
    }
pub const PCI_DEVICE_ID_CAVIUM_OCTEP_HP_CTLR: c_uint = 0xa0e3;
    static struct pci_device_id octep_hp_pci_map[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, PCI_DEVICE_ID_CAVIUM_OCTEP_HP_CTLR) },
    { },
    };
    static struct pci_driver octep_hp = {
    .name = OCTEP_HP_DRV_NAME,
    .id_table = octep_hp_pci_map,
    .probe = octep_hp_pci_probe,
    };
    module_pci_driver(octep_hp);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marvell");
    MODULE_DESCRIPTION("Marvell OCTEON PCI Hotplug driver");
