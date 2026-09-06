//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/pme.c
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
// PCIe Native PME support
//
// Copyright (C) 2007 - 2009 Intel Corp
// Copyright (C) 2007 - 2009 Shaohua Li <shaohua.li@intel.com>
// Copyright (C) 2009 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

//
// If this switch is set, MSI will not be used for PCIe PME signaling.  This
// causes the PCIe port driver to use INTx interrupts only, but it turns out
// that using MSI for PCIe PME signaling doesn't play well with PCIe PME-based
// wake-up from system sleep states.
//
    bool pcie_pme_msi_disabled;
#[no_mangle]
unsafe extern "C" fn pcie_pme_setup(str: *mut c_char) -> int __init {
    static int __init pcie_pme_setup(char *str)
    {
    if (!strncmp(str, "nomsi", 5))
    pcie_pme_msi_disabled = true;
    return 1;
    }
    __setup("pcie_pme=", pcie_pme_setup);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_pme_service_data {
    pub lock: spinlock_t,
    pub srv: *mut pcie_device,
    pub work: work_struct,
    pub /: *mut *mut bool noirq; / If set, keep the PME interrupt disabled.,
}

//
// pcie_pme_interrupt_enable - Enable/disable PCIe PME interrupt generation.
// @dev: PCIe root port or event collector.
// @enable: Enable or disable the interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn pcie_pme_interrupt_enable(dev: *mut pci_dev, enable: bool) {
    void pcie_pme_interrupt_enable(struct pci_dev *dev, bool enable)
    {
    if (enable)
    pcie_capability_set_word(dev, PCI_EXP_RTCTL,
    PCI_EXP_RTCTL_PMEIE);
    else
    pcie_capability_clear_word(dev, PCI_EXP_RTCTL,
    PCI_EXP_RTCTL_PMEIE);
    }
//
// pcie_pme_walk_bus - Scan a PCI bus for devices asserting PME#.
// @bus: PCI bus to scan.
//
// Scan given PCI bus and all buses under it for devices asserting PME#.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_walk_bus(bus: *mut pci_bus) -> bool {
    static bool pcie_pme_walk_bus(struct pci_bus *bus)
    {
    struct pci_dev *dev;
    let mut ret: bool = false;
    list_for_each_entry(dev, &bus.devices, bus_list) {
// Skip PCIe devices in case we started from a root port.
    if (!pci_is_pcie(dev) && pci_check_pme_status(dev)) {
    if (dev.pme_poll)
    dev.pme_poll = false;
    pci_wakeup_event(dev);
    pm_request_resume(&dev.dev);
    ret = true;
    }
    if (dev.subordinate && pcie_pme_walk_bus(dev.subordinate))
    ret = true;
    }
    return ret;
    }
//
// pcie_pme_from_pci_bridge - Check if PCIe-PCI bridge generated a PME.
// @bus: Secondary bus of the bridge.
// @devfn: Device/function number to check.
//
// PME from PCI devices under a PCIe-PCI bridge may be converted to an in-band
// PCIe PME message.  In such that case the bridge should use the Requester ID
// of device/function number 0 on its secondary bus.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_from_pci_bridge(bus: *mut pci_bus, devfn: u8) -> bool {
    static bool pcie_pme_from_pci_bridge(struct pci_bus *bus, u8 devfn)
    {
    struct pci_dev *dev;
    let mut found: bool = false;
    if (devfn)
    return false;
    dev = pci_dev_get(bus.self);
    if (!dev)
    return false;
    if (pci_is_pcie(dev) && pci_pcie_type(dev) == PCI_EXP_TYPE_PCI_BRIDGE) {
    down_read(&pci_bus_sem);
    if (pcie_pme_walk_bus(bus))
    found = true;
    up_read(&pci_bus_sem);
    }
    pci_dev_put(dev);
    return found;
    }
//
// pcie_pme_handle_request - Find device that generated PME and handle it.
// @port: Root port or event collector that generated the PME interrupt.
// @req_id: PCIe Requester ID of the device that generated the PME.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_handle_request(port: *mut pci_dev, req_id: u16) {
    static void pcie_pme_handle_request(struct pci_dev *port, u16 req_id)
    {
    let mut busnr: u8 = req_id >> 8, devfn = req_id & 0xff;
    struct pci_bus *bus;
    struct pci_dev *dev;
    let mut found: bool = false;
// First, check if the PME is from the root port itself.
    if (port.devfn == devfn && port.bus.number == busnr) {
    if (port.pme_poll)
    port.pme_poll = false;
    if (pci_check_pme_status(port)) {
    pm_request_resume(&port.dev);
    found = true;
    } else {
//
// Apparently, the root port generated the PME on behalf
// of a non-PCIe device downstream.  If this is done by
// a root port, the Requester ID field in its status
// register may contain either the root port's, or the
// source device's information (PCI Express Base
// Specification, Rev. 2.0, Section 6.1.9).
//
    down_read(&pci_bus_sem);
    found = pcie_pme_walk_bus(port.subordinate);
    up_read(&pci_bus_sem);
    }
    goto out;
    }
// Second, find the bus the source device is on.
    bus = pci_find_bus(pci_domain_nr(port.bus), busnr);
    if (!bus)
    goto out;
// Next, check if the PME is from a PCIe-PCI bridge.
    found = pcie_pme_from_pci_bridge(bus, devfn);
    if (found)
    goto out;
// Finally, try to find the PME source on the bus.
    down_read(&pci_bus_sem);
    list_for_each_entry(dev, &bus.devices, bus_list) {
    pci_dev_get(dev);
    if (dev.devfn == devfn) {
    found = true;
    break;
    }
    pci_dev_put(dev);
    }
    up_read(&pci_bus_sem);
    if (found) {
// The device is there, but we have to check its PME status.
    found = pci_check_pme_status(dev);
    if (found) {
    if (dev.pme_poll)
    dev.pme_poll = false;
    pci_wakeup_event(dev);
    pm_request_resume(&dev.dev);
    }
    pci_dev_put(dev);
    } else if (devfn) {
//
// The device is not there, but we can still try to recover by
// assuming that the PME was reported by a PCIe-PCI bridge that
// used devfn different from zero.
//
    pci_info(port, "interrupt generated for non-existent device %02x:%02x.%d\n",
    busnr, PCI_SLOT(devfn), PCI_FUNC(devfn));
    found = pcie_pme_from_pci_bridge(bus, 0);
    }
    out:
    if (!found)
    pci_info(port, "Spurious native interrupt!\n");
    }
//
// pcie_pme_work_fn - Work handler for PCIe PME interrupt.
// @work: Work structure giving access to service data.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_work_fn(work: *mut work_struct) {
    static void pcie_pme_work_fn(struct work_struct *work)
    {
    struct pcie_pme_service_data *data =
    container_of(work, struct pcie_pme_service_data, work);
    struct pci_dev *port = data.srv.port;
    u32 rtsta;
    spin_lock_irq(&data.lock);
    for (;;) {
    if (data.noirq)
    break;
    pcie_capability_read_dword(port, PCI_EXP_RTSTA, &rtsta);
    if (PCI_POSSIBLE_ERROR(rtsta))
    break;
    if (rtsta & PCI_EXP_RTSTA_PME) {
//
// Clear PME status of the port.  If there are other
// pending PMEs, the status will be set again.
//
    pcie_clear_root_pme_status(port);
    spin_unlock_irq(&data.lock);
    pcie_pme_handle_request(port,
    FIELD_GET(PCI_EXP_RTSTA_PME_RQ_ID, rtsta));
    spin_lock_irq(&data.lock);
    continue;
    }
// No need to loop if there are no more PMEs pending.
    if (!(rtsta & PCI_EXP_RTSTA_PENDING))
    break;
    spin_unlock_irq(&data.lock);
    cpu_relax();
    spin_lock_irq(&data.lock);
    }
    if (!data.noirq)
    pcie_pme_interrupt_enable(port, true);
    spin_unlock_irq(&data.lock);
    }
//
// pcie_pme_irq - Interrupt handler for PCIe root port PME interrupt.
// @irq: Interrupt vector.
// @context: Interrupt context pointer.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcie_pme_irq(int irq, void *context)
    {
    struct pci_dev *port;
    struct pcie_pme_service_data *data;
    u32 rtsta;
    unsigned long flags;
    port = ((struct pcie_device *)context).port;
    data = get_service_data((struct pcie_device *)context);
    spin_lock_irqsave(&data.lock, flags);
    pcie_capability_read_dword(port, PCI_EXP_RTSTA, &rtsta);
    if (PCI_POSSIBLE_ERROR(rtsta) || !(rtsta & PCI_EXP_RTSTA_PME)) {
    spin_unlock_irqrestore(&data.lock, flags);
    return IRQ_NONE;
    }
    pcie_pme_interrupt_enable(port, false);
    spin_unlock_irqrestore(&data.lock, flags);
// We don't use pm_wq, because it's freezable.
    schedule_work(&data.work);
    return IRQ_HANDLED;
    }
//
// pcie_pme_can_wakeup - Set the wakeup capability flag.
// @dev: PCI device to handle.
// @ign: Ignored.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_can_wakeup(dev: *mut pci_dev, ign: *mut c_void) -> c_int {
    static int pcie_pme_can_wakeup(struct pci_dev *dev, void *ign)
    {
    device_set_wakeup_capable(&dev.dev, true);
    return 0;
    }
//
// pcie_pme_mark_devices - Set the wakeup flag for devices below a port.
// @port: PCIe root port or event collector to handle.
//
// For each device below given root port, including the port itself (or for each
// root complex integrated endpoint if @port is a root complex event collector)
// set the flag indicating that it can signal run-time wake-up events.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_mark_devices(port: *mut pci_dev) {
    static void pcie_pme_mark_devices(struct pci_dev *port)
    {
    pcie_pme_can_wakeup(port, core::ptr::null_mut());
    if (pci_pcie_type(port) == PCI_EXP_TYPE_RC_EC)
    pcie_walk_rcec(port, pcie_pme_can_wakeup, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if(_arg: port->subordinate) -> else {
    else if (port.subordinate)
    pci_walk_bus(port.subordinate, pcie_pme_can_wakeup, core::ptr::null_mut());
    }
//
// pcie_pme_probe - Initialize PCIe PME service for given root port.
// @srv: PCIe service to initialize.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_probe(srv: *mut pcie_device) -> c_int {
    static int pcie_pme_probe(struct pcie_device *srv)
    {
    struct pci_dev *port = srv.port;
    struct pcie_pme_service_data *data;
    let mut type: c_int = pci_pcie_type(port);
    int ret;
// Limit to Root Ports or Root Complex Event Collectors
    if (type != PCI_EXP_TYPE_RC_EC &&
    type != PCI_EXP_TYPE_ROOT_PORT)
    return -ENODEV;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    spin_lock_init(&data.lock);
    INIT_WORK(&data.work, pcie_pme_work_fn);
    data.srv = srv;
    set_service_data(srv, data);
    pcie_pme_interrupt_enable(port, false);
    pcie_clear_root_pme_status(port);
    ret = request_irq(srv.irq, pcie_pme_irq, IRQF_SHARED, "PCIe PME", srv);
    if (ret) {
    kfree(data);
    return ret;
    }
    pci_info(port, "Signaling with IRQ %d\n", srv.irq);
    pcie_pme_mark_devices(port);
    pcie_pme_interrupt_enable(port, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_pme_check_wakeup(bus: *mut pci_bus) -> bool {
    static bool pcie_pme_check_wakeup(struct pci_bus *bus)
    {
    struct pci_dev *dev;
    if (!bus)
    return false;
    list_for_each_entry(dev, &bus.devices, bus_list)
    if (device_may_wakeup(&dev.dev)
    || pcie_pme_check_wakeup(dev.subordinate))
    return true;
    return false;
    }
    static void pcie_pme_disable_interrupt(struct pci_dev *port,
    struct pcie_pme_service_data *data)
    {
    spin_lock_irq(&data.lock);
    pcie_pme_interrupt_enable(port, false);
    pcie_clear_root_pme_status(port);
    data.noirq = true;
    spin_unlock_irq(&data.lock);
    }
//
// pcie_pme_suspend - Suspend PCIe PME service device.
// @srv: PCIe service device to suspend.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_suspend(srv: *mut pcie_device) -> c_int {
    static int pcie_pme_suspend(struct pcie_device *srv)
    {
    struct pcie_pme_service_data *data = get_service_data(srv);
    struct pci_dev *port = srv.port;
    bool wakeup;
    int ret;
    if (device_may_wakeup(&port.dev)) {
    wakeup = true;
    } else {
    down_read(&pci_bus_sem);
    wakeup = pcie_pme_check_wakeup(port.subordinate);
    up_read(&pci_bus_sem);
    }
    if (wakeup) {
    ret = enable_irq_wake(srv.irq);
    if (!ret)
    return 0;
    }
    pcie_pme_disable_interrupt(port, data);
    synchronize_irq(srv.irq);
    return 0;
    }
//
// pcie_pme_resume - Resume PCIe PME service device.
// @srv: PCIe service device to resume.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_resume(srv: *mut pcie_device) -> c_int {
    static int pcie_pme_resume(struct pcie_device *srv)
    {
    struct pcie_pme_service_data *data = get_service_data(srv);
    spin_lock_irq(&data.lock);
    if (data.noirq) {
    struct pci_dev *port = srv.port;
    pcie_clear_root_pme_status(port);
    pcie_pme_interrupt_enable(port, true);
    data.noirq = false;
    } else {
    disable_irq_wake(srv.irq);
    }
    spin_unlock_irq(&data.lock);
    return 0;
    }
//
// pcie_pme_remove - Prepare PCIe PME service device for removal.
// @srv: PCIe service device to remove.
//
#[no_mangle]
unsafe extern "C" fn pcie_pme_remove(srv: *mut pcie_device) {
    static void pcie_pme_remove(struct pcie_device *srv)
    {
    struct pcie_pme_service_data *data = get_service_data(srv);
    pcie_pme_disable_interrupt(srv.port, data);
    free_irq(srv.irq, srv);
    cancel_work_sync(&data.work);
    kfree(data);
    }
    static struct pcie_port_service_driver pcie_pme_driver = {
    .name		= "pcie_pme",
    .port_type	= PCIE_ANY_PORT,
    .service	= PCIE_PORT_SERVICE_PME,
    .probe		= pcie_pme_probe,
    .suspend	= pcie_pme_suspend,
    .resume		= pcie_pme_resume,
    .remove		= pcie_pme_remove,
    };
//
// pcie_pme_init - Register the PCIe PME service driver.
//
#[no_mangle]
pub unsafe extern "C" fn pcie_pme_init() -> int __init {
    int __init pcie_pme_init(void)
    {
    return pcie_port_service_register(&pcie_pme_driver);
    }
