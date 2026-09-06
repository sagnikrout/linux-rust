//! Automatically rewritten from C to Rust
//! Source: drivers/pci/remove.c
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

#[no_mangle]
unsafe extern "C" fn pci_free_resources(dev: *mut pci_dev) {
    static void pci_free_resources(struct pci_dev *dev)
    {
    struct resource *res;
    pci_dev_for_each_resource(dev, res) {
    if (res.parent)
    release_resource(res);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci_stop_dev(dev: *mut pci_dev) {
    static void pci_stop_dev(struct pci_dev *dev)
    {
    pci_pme_active(dev, false);
    if (!pci_dev_test_and_clear_added(dev))
    return;
    device_release_driver(&dev.dev);
    pci_proc_detach_device(dev);
    of_pci_remove_node(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_destroy_dev(dev: *mut pci_dev) {
    static void pci_destroy_dev(struct pci_dev *dev)
    {
    if (pci_dev_test_and_set_removed(dev))
    return;
    platform_pci_remove_wake(dev);
    pci_doe_sysfs_teardown(dev);
    pci_npem_remove(dev);
//
// While device is in D0 drop the device from TSM link operations
// including unbind and disconnect (IDE + SPDM teardown).
//
    pci_tsm_destroy(dev);
    device_del(&dev.dev);
    down_write(&pci_bus_sem);
    list_del(&dev.bus_list);
    up_write(&pci_bus_sem);
    pci_doe_destroy(dev);
    pci_ide_destroy(dev);
    pcie_aspm_exit_link_state(dev);
    pci_bridge_d3_update(dev);
    pci_free_resources(dev);
    put_device(&dev.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_remove_bus(bus: *mut pci_bus) {
    void pci_remove_bus(struct pci_bus *bus)
    {
    pci_proc_detach_bus(bus);
    down_write(&pci_bus_sem);
    list_del(&bus.node);
    pci_bus_release_busn_res(bus);
    up_write(&pci_bus_sem);
    if (bus.ops.remove_bus)
    bus.ops.remove_bus(bus);
    pcibios_remove_bus(bus);
    device_unregister(&bus.dev);
    }
    EXPORT_SYMBOL(pci_remove_bus);
#[no_mangle]
unsafe extern "C" fn pci_stop_bus_device(dev: *mut pci_dev) {
    static void pci_stop_bus_device(struct pci_dev *dev)
    {
    struct pci_bus *bus = dev.subordinate;
    struct pci_dev *child, *tmp;
//
// Stopping an SR-IOV PF device removes all the associated VFs,
// which will update the bus->devices list and confuse the
// iterator.  Therefore, iterate in reverse so we remove the VFs
// first, then the PF.
//
    if (bus) {
    list_for_each_entry_safe_reverse(child, tmp,
    &bus.devices, bus_list)
    pci_stop_bus_device(child);
    }
    pci_stop_dev(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_remove_bus_device(dev: *mut pci_dev) {
    static void pci_remove_bus_device(struct pci_dev *dev)
    {
    struct pci_bus *bus = dev.subordinate;
    struct pci_dev *child, *tmp;
    if (bus) {
    list_for_each_entry_safe(child, tmp,
    &bus.devices, bus_list)
    pci_remove_bus_device(child);
    pci_remove_bus(bus);
    dev.subordinate = core::ptr::null_mut();
    }
    pci_destroy_dev(dev);
    }
//
// pci_stop_and_remove_bus_device - remove a PCI device and any children
// @dev: the device to remove
//
// Remove a PCI device from the device lists, informing the drivers
// that the device has been removed.  We also remove any subordinate
// buses and children in a depth-first manner.
//
// For each device we remove, delete the device structure from the
// device lists, remove the /proc entry, and notify userspace
// (/sbin/hotplug).
//
#[no_mangle]
pub unsafe extern "C" fn pci_stop_and_remove_bus_device(dev: *mut pci_dev) {
    void pci_stop_and_remove_bus_device(struct pci_dev *dev)
    {
    lockdep_assert_held(&pci_rescan_remove_lock);
    pci_stop_bus_device(dev);
    pci_remove_bus_device(dev);
    }
    EXPORT_SYMBOL(pci_stop_and_remove_bus_device);
#[no_mangle]
pub unsafe extern "C" fn pci_stop_and_remove_bus_device_locked(dev: *mut pci_dev) {
    void pci_stop_and_remove_bus_device_locked(struct pci_dev *dev)
    {
    pci_lock_rescan_remove();
    pci_stop_and_remove_bus_device(dev);
    pci_unlock_rescan_remove();
    }
    EXPORT_SYMBOL_GPL(pci_stop_and_remove_bus_device_locked);
#[no_mangle]
pub unsafe extern "C" fn pci_stop_root_bus(bus: *mut pci_bus) {
    void pci_stop_root_bus(struct pci_bus *bus)
    {
    struct pci_dev *child, *tmp;
    struct pci_host_bridge *host_bridge;
    if (!pci_is_root_bus(bus))
    return;
    host_bridge = to_pci_host_bridge(bus.bridge);
    list_for_each_entry_safe_reverse(child, tmp,
    &bus.devices, bus_list)
    pci_stop_bus_device(child);
    of_pci_remove_host_bridge_node(host_bridge);
// stop the host bridge
    device_release_driver(&host_bridge.dev);
    }
    EXPORT_SYMBOL_GPL(pci_stop_root_bus);
#[no_mangle]
pub unsafe extern "C" fn pci_remove_root_bus(bus: *mut pci_bus) {
    void pci_remove_root_bus(struct pci_bus *bus)
    {
    struct pci_dev *child, *tmp;
    struct pci_host_bridge *host_bridge;
    if (!pci_is_root_bus(bus))
    return;
    host_bridge = to_pci_host_bridge(bus.bridge);
    list_for_each_entry_safe(child, tmp,
    &bus.devices, bus_list)
    pci_remove_bus_device(child);

// Release domain_nr if it was dynamically allocated
    if (host_bridge.domain_nr == PCI_DOMAIN_NR_NOT_SET)
    pci_bus_release_domain_nr(host_bridge.dev.parent, bus.domain_nr);

    pci_remove_bus(bus);
    host_bridge.bus = core::ptr::null_mut();
// remove the host bridge
    device_del(&host_bridge.dev);
    }
    EXPORT_SYMBOL_GPL(pci_remove_root_bus);
