//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/bwctrl.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// PCIe bandwidth controller
//
// Author: Alexandru Gagniuc <mr.nuke.me@gmail.com>
//
// Copyright (C) 2019 Dell Inc
// Copyright (C) 2023-2024 Intel Corporation
//
// The PCIe bandwidth controller provides a way to alter PCIe Link Speeds
// and notify the operating system when the Link Width or Speed changes. The
// notification capability is required for all Root Ports and Downstream
// Ports supporting Link Width wider than x1 and/or multiple Link Speeds.
//
// This service port driver hooks into the Bandwidth Notification interrupt
// watching for changes or links becoming degraded in operation. It updates
// the cached Current Link Speed that is exposed to user space through sysfs.
//

//
// struct pcie_bwctrl_data - PCIe bandwidth controller
// @set_speed_mutex:	Serializes link speed changes
// @cdev:		Thermal cooling device associated with the port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_bwctrl_data {
    pub set_speed_mutex: mutex,
    pub cdev: *mut thermal_cooling_device,
}

// Prevent port removal during Link Speed changes.
    static DECLARE_RWSEM(pcie_bwctrl_setspeed_rwsem);
#[no_mangle]
pub unsafe extern "C" fn pcie_supported_speeds2target_speed(supported_speeds: u8) -> u16 {
    static inline u16 pcie_supported_speeds2target_speed(u8 supported_speeds)
    {
    return __fls(supported_speeds);
    }
//
// pcie_bwctrl_select_speed - Select Target Link Speed
// @port:	PCIe Port
// @speed_req:	Requested PCIe Link Speed
//
// Select Target Link Speed by take into account Supported Link Speeds of
// both the Root Port and the Endpoint.
//
// Return: Target Link Speed (1=2.5GT/s, 2=5GT/s, 3=8GT/s, etc.)
//
#[no_mangle]
unsafe extern "C" fn pcie_bwctrl_select_speed(port: *mut pci_dev, speed_req: enum pci_bus_speed) -> u16 {
    static u16 pcie_bwctrl_select_speed(struct pci_dev *port, enum pci_bus_speed speed_req)
    {
    struct pci_bus *bus = port.subordinate;
    u8 desired_speeds, supported_speeds;
    struct pci_dev *dev;
    desired_speeds = GENMASK(pci_bus_speed2lnkctl2(speed_req),
    __fls(PCI_EXP_LNKCAP2_SLS_2_5GB));
    supported_speeds = port.supported_speeds;
    if (bus) {
    down_read(&pci_bus_sem);
    dev = list_first_entry_or_null(&bus.devices, struct pci_dev, bus_list);
    if (dev)
    supported_speeds &= dev.supported_speeds;
    up_read(&pci_bus_sem);
    }
    if (!supported_speeds)
    supported_speeds = PCI_EXP_LNKCAP2_SLS_2_5GB;
    return pcie_supported_speeds2target_speed(supported_speeds & desired_speeds);
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwctrl_change_speed(port: *mut pci_dev, target_speed: u16, use_lt: bool) -> c_int {
    static int pcie_bwctrl_change_speed(struct pci_dev *port, u16 target_speed, bool use_lt)
    {
    int ret;
    ret = pcie_capability_clear_and_set_word(port, PCI_EXP_LNKCTL2,
    PCI_EXP_LNKCTL2_TLS, target_speed);
    if (ret != PCIBIOS_SUCCESSFUL)
    return pcibios_err_to_errno(ret);
    return pcie_retrain_link(port, use_lt);
    }
//
// pcie_set_target_speed - Set downstream Link Speed for PCIe Port
// @port:	PCIe Port
// @speed_req:	Requested PCIe Link Speed
// @use_lt:	Wait for the LT or DLLLA bit to detect the end of link training
//
// Attempt to set PCIe Port Link Speed to @speed_req. @speed_req may be
// adjusted downwards to the best speed supported by both the Port and PCIe
// Device underneath it.
//
// Return:
// * 0		- on success
// * -EINVAL	- @speed_req is not a PCIe Link Speed
// * -ENODEV	- @port is not controllable
// * -ETIMEDOUT	- changing Link Speed took too long
// * -EAGAIN	- Link Speed was changed but @speed_req was not achieved
//
    int pcie_set_target_speed(struct pci_dev *port, enum pci_bus_speed speed_req,
    bool use_lt)
    {
    struct pci_bus *bus = port.subordinate;
    u16 target_speed;
    int ret;
    if (WARN_ON_ONCE(!pcie_valid_speed(speed_req)))
    return -EINVAL;
    if (bus && bus.cur_bus_speed == speed_req)
    return 0;
    target_speed = pcie_bwctrl_select_speed(port, speed_req);
    scoped_guard(rwsem_read, &pcie_bwctrl_setspeed_rwsem) {
    struct pcie_bwctrl_data *data = port.link_bwctrl;
//
// port->link_bwctrl is NULL during initial scan when called
// e.g. from the Target Speed quirk.
//
    if (data)
    mutex_lock(&data.set_speed_mutex);
    ret = pcie_bwctrl_change_speed(port, target_speed, use_lt);
    if (data)
    mutex_unlock(&data.set_speed_mutex);
    }
//
// Despite setting higher speed into the Target Link Speed, empty
// bus won't train to 5GT+ speeds.
//
    if (!ret && bus && bus.cur_bus_speed != speed_req &&
    !list_empty(&bus.devices))
    ret = -EAGAIN;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_enable(srv: *mut pcie_device) {
    static void pcie_bwnotif_enable(struct pcie_device *srv)
    {
    struct pci_dev *port = srv.port;
    u16 link_status;
    int ret;
// Note if LBMS has been seen so far
    ret = pcie_capability_read_word(port, PCI_EXP_LNKSTA, &link_status);
    if (ret == PCIBIOS_SUCCESSFUL && link_status & PCI_EXP_LNKSTA_LBMS)
    set_bit(PCI_LINK_LBMS_SEEN, &port.priv_flags);
    pcie_capability_set_word(port, PCI_EXP_LNKCTL,
    PCI_EXP_LNKCTL_LBMIE | PCI_EXP_LNKCTL_LABIE);
    pcie_capability_write_word(port, PCI_EXP_LNKSTA,
    PCI_EXP_LNKSTA_LBMS | PCI_EXP_LNKSTA_LABS);
//
// Update after enabling notifications & clearing status bits ensures
// link speed is up to date.
//
    pcie_update_link_speed(port.subordinate, PCIE_BWCTRL_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_disable(port: *mut pci_dev) {
    static void pcie_bwnotif_disable(struct pci_dev *port)
    {
    pcie_capability_clear_word(port, PCI_EXP_LNKCTL,
    PCI_EXP_LNKCTL_LBMIE | PCI_EXP_LNKCTL_LABIE);
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcie_bwnotif_irq(int irq, void *context)
    {
    struct pcie_device *srv = context;
    struct pci_dev *port = srv.port;
    u16 link_status, events;
    int ret;
    ret = pcie_capability_read_word(port, PCI_EXP_LNKSTA, &link_status);
    if (ret != PCIBIOS_SUCCESSFUL)
    return IRQ_NONE;
    events = link_status & (PCI_EXP_LNKSTA_LBMS | PCI_EXP_LNKSTA_LABS);
    if (!events)
    return IRQ_NONE;
    if (events & PCI_EXP_LNKSTA_LBMS)
    set_bit(PCI_LINK_LBMS_SEEN, &port.priv_flags);
    pcie_capability_write_word(port, PCI_EXP_LNKSTA, events);
//
// Interrupts will not be triggered from any further Link Speed
// change until LBMS is cleared by the write. Therefore, re-read the
// speed (inside pcie_update_link_speed()) after LBMS has been
// cleared to avoid missing link speed changes.
//
    pcie_update_link_speed(port.subordinate, PCIE_BWCTRL_IRQ);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_reset_lbms(port: *mut pci_dev) {
    void pcie_reset_lbms(struct pci_dev *port)
    {
    clear_bit(PCI_LINK_LBMS_SEEN, &port.priv_flags);
    pcie_capability_write_word(port, PCI_EXP_LNKSTA, PCI_EXP_LNKSTA_LBMS);
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_probe(srv: *mut pcie_device) -> c_int {
    static int pcie_bwnotif_probe(struct pcie_device *srv)
    {
    struct pci_dev *port = srv.port;
    int ret;
    if (port.no_bw_notif)
    return -ENODEV;
// Can happen if we run out of bus numbers during enumeration.
    if (!port.subordinate)
    return -ENODEV;
    struct pcie_bwctrl_data *data = devm_kzalloc(&srv.device,
    sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    ret = devm_mutex_init(&srv.device, &data.set_speed_mutex);
    if (ret)
    return ret;
    scoped_guard(rwsem_write, &pcie_bwctrl_setspeed_rwsem) {
    port.link_bwctrl = data;
    ret = request_irq(srv.irq, pcie_bwnotif_irq,
    IRQF_SHARED, "PCIe bwctrl", srv);
    if (ret) {
    port.link_bwctrl = core::ptr::null_mut();
    return ret;
    }
    pcie_bwnotif_enable(srv);
    }
    pci_dbg(port, "enabled with IRQ %d\n", srv.irq);
// Don't fail on errors. Don't leave IS_ERR() "pointer" into ->cdev
    port.link_bwctrl.cdev = pcie_cooling_device_register(port);
    if (IS_ERR(port.link_bwctrl.cdev))
    port.link_bwctrl.cdev = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_remove(srv: *mut pcie_device) {
    static void pcie_bwnotif_remove(struct pcie_device *srv)
    {
    struct pcie_bwctrl_data *data = srv.port.link_bwctrl;
    pcie_cooling_device_unregister(data.cdev);
    scoped_guard(rwsem_write, &pcie_bwctrl_setspeed_rwsem) {
    pcie_bwnotif_disable(srv.port);
    free_irq(srv.irq, srv);
    srv.port.link_bwctrl = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_suspend(srv: *mut pcie_device) -> c_int {
    static int pcie_bwnotif_suspend(struct pcie_device *srv)
    {
    pcie_bwnotif_disable(srv.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_bwnotif_resume(srv: *mut pcie_device) -> c_int {
    static int pcie_bwnotif_resume(struct pcie_device *srv)
    {
    pcie_bwnotif_enable(srv);
    return 0;
    }
    static struct pcie_port_service_driver pcie_bwctrl_driver = {
    .name		= "pcie_bwctrl",
    .port_type	= PCIE_ANY_PORT,
    .service	= PCIE_PORT_SERVICE_BWCTRL,
    .probe		= pcie_bwnotif_probe,
    .suspend	= pcie_bwnotif_suspend,
    .resume		= pcie_bwnotif_resume,
    .remove		= pcie_bwnotif_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn pcie_bwctrl_init() -> int __init {
    int __init pcie_bwctrl_init(void)
    {
    return pcie_port_service_register(&pcie_bwctrl_driver);
    }
