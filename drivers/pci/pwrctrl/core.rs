//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pwrctrl/core.c
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
// Copyright (C) 2024 Linaro Ltd.
//

    static int pci_pwrctrl_notify(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct pci_pwrctrl *pwrctrl = container_of(nb, struct pci_pwrctrl, nb);
    struct device *dev = data;
    if (dev_fwnode(dev) != dev_fwnode(pwrctrl.dev))
    return NOTIFY_DONE;
    switch (action) {
    case BUS_NOTIFY_ADD_DEVICE:
//
// We will have two struct device objects bound to two different
// drivers on different buses but consuming the same DT node. We
// must not bind the pins twice in this case but only once for
// the first device to be added.
//
// If we got here then the PCI device is the second after the
// power control platform device. Mark its OF node as reused.
//
    dev_set_of_node_reused(dev);
    break;
    }
    return NOTIFY_DONE;
    }
//
// pci_pwrctrl_init() - Initialize the PCI power control context struct
//
// @pwrctrl: PCI power control data
// @dev: Parent device
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_init(pwrctrl: *mut pci_pwrctrl, dev: *mut device) {
    void pci_pwrctrl_init(struct pci_pwrctrl *pwrctrl, struct device *dev)
    {
    pwrctrl.dev = dev;
    dev_set_drvdata(dev, pwrctrl);
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_init);
//
// pci_pwrctrl_device_set_ready() - Notify the pwrctrl subsystem that the PCI
// device is powered-up and ready to be detected.
//
// @pwrctrl: PCI power control data.
//
// Returns:
// 0 on success, negative error number on error.
//
// Note:
// This function returning 0 doesn't mean the device was detected. It means,
// that the bus rescan was successfully started. The device will get bound to
// its PCI driver asynchronously.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_device_set_ready(pwrctrl: *mut pci_pwrctrl) -> c_int {
    int pci_pwrctrl_device_set_ready(struct pci_pwrctrl *pwrctrl)
    {
    int ret;
    if (!pwrctrl.dev)
    return -ENODEV;
    pwrctrl.nb.notifier_call = pci_pwrctrl_notify;
    ret = bus_register_notifier(&pci_bus_type, &pwrctrl.nb);
    if (ret)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_device_set_ready);
//
// pci_pwrctrl_device_unset_ready() - Notify the pwrctrl subsystem that the PCI
// device is about to be powered-down.
//
// @pwrctrl: PCI power control data.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_device_unset_ready(pwrctrl: *mut pci_pwrctrl) {
    void pci_pwrctrl_device_unset_ready(struct pci_pwrctrl *pwrctrl)
    {
//
// We don't have to delete the link here. Typically, this function
// is only called when the power control device is being detached. If
// it is being detached then the child PCI device must have already
// been unbound too or the device core wouldn't let us unbind.
//
    bus_unregister_notifier(&pci_bus_type, &pwrctrl.nb);
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_device_unset_ready);
#[no_mangle]
unsafe extern "C" fn devm_pci_pwrctrl_device_unset_ready(data: *mut c_void) {
    static void devm_pci_pwrctrl_device_unset_ready(void *data)
    {
    struct pci_pwrctrl *pwrctrl = data;
    pci_pwrctrl_device_unset_ready(pwrctrl);
    }
//
// devm_pci_pwrctrl_device_set_ready - Managed variant of
// pci_pwrctrl_device_set_ready().
//
// @dev: Device managing this pwrctrl provider.
// @pwrctrl: PCI power control data.
//
// Returns:
// 0 on success, negative error number on error.
//
    int devm_pci_pwrctrl_device_set_ready(struct device *dev,
    struct pci_pwrctrl *pwrctrl)
    {
    int ret;
    ret = pci_pwrctrl_device_set_ready(pwrctrl);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev,
    devm_pci_pwrctrl_device_unset_ready,
    pwrctrl);
    }
    EXPORT_SYMBOL_GPL(devm_pci_pwrctrl_device_set_ready);
//
// Check whether the pwrctrl device really needs to be created or not. The
// pwrctrl device will only be created if the node satisfies below requirements:
//
// 1. Presence of compatible property with "pci" prefix to match against the
// pwrctrl driver (AND)
// 2. At least one of the power supplies defined in the devicetree node of the
// device (OR) in the remote endpoint parent node to indicate pwrctrl
// requirement.
//
#[no_mangle]
unsafe extern "C" fn pci_pwrctrl_is_required(np: *mut device_node) -> bool {
    static bool pci_pwrctrl_is_required(struct device_node *np)
    {
    struct device_node *endpoint;
    const char *compat;
    int ret;
    ret = of_property_read_string(np, "compatible", &compat);
    if (ret < 0)
    return false;
    if (!strstarts(compat, "pci"))
    return false;
    if (of_pci_supply_present(np))
    return true;
    if (of_graph_is_present(np)) {
    for_each_endpoint_of_node(np, endpoint) {
    struct device_node *remote __free(device_node) =
    of_graph_get_remote_port_parent(endpoint);
    if (remote) {
    if (of_pci_supply_present(remote)) {
    of_node_put(endpoint);
    return true;
    }
    }
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn __pci_pwrctrl_power_off_device(dev: *mut device) -> c_int {
    static int __pci_pwrctrl_power_off_device(struct device *dev)
    {
    struct pci_pwrctrl *pwrctrl = dev_get_drvdata(dev);
    if (!pwrctrl)
    return 0;
    return pwrctrl.power_off(pwrctrl);
    }
#[no_mangle]
unsafe extern "C" fn pci_pwrctrl_power_off_device(np: *mut device_node) {
    static void pci_pwrctrl_power_off_device(struct device_node *np)
    {
    struct platform_device *pdev;
    int ret;
    for_each_available_child_of_node_scoped(np, child)
    pci_pwrctrl_power_off_device(child);
    if (!pci_pwrctrl_is_required(np))
    return;
    pdev = of_find_device_by_node(np);
    if (!pdev)
    return;
    scoped_guard(device, &pdev.dev) {
    if (device_is_bound(&pdev.dev)) {
    ret = __pci_pwrctrl_power_off_device(&pdev.dev);
    if (ret)
    dev_err(&pdev.dev, "Failed to power off device: %d", ret);
    }
    }
    platform_device_put(pdev);
    }
//
// pci_pwrctrl_power_off_devices - Power off pwrctrl devices
//
// @parent: PCI host controller device
//
// Recursively traverse all pwrctrl devices for the devicetree hierarchy
// below the specified PCI host controller and power them off in a depth
// first manner.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_power_off_devices(parent: *mut device) {
    void pci_pwrctrl_power_off_devices(struct device *parent)
    {
    struct device_node *np = parent.of_node;
    for_each_available_child_of_node_scoped(np, child)
    pci_pwrctrl_power_off_device(child);
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_power_off_devices);
#[no_mangle]
unsafe extern "C" fn __pci_pwrctrl_power_on_device(dev: *mut device) -> c_int {
    static int __pci_pwrctrl_power_on_device(struct device *dev)
    {
    struct pci_pwrctrl *pwrctrl = dev_get_drvdata(dev);
    if (!pwrctrl)
    return 0;
    return pwrctrl.power_on(pwrctrl);
    }
//
// Power on the devices in a depth first manner. Before powering on the device,
// make sure its driver is bound.
//
#[no_mangle]
unsafe extern "C" fn pci_pwrctrl_power_on_device(np: *mut device_node) -> c_int {
    static int pci_pwrctrl_power_on_device(struct device_node *np)
    {
    struct platform_device *pdev;
    let mut ret: c_int = 0;
    for_each_available_child_of_node_scoped(np, child) {
    ret = pci_pwrctrl_power_on_device(child);
    if (ret)
    return ret;
    }
    if (!pci_pwrctrl_is_required(np))
    return 0;
    pdev = of_find_device_by_node(np);
    if (!pdev)
    return 0;
    scoped_guard(device, &pdev.dev) {
    if (device_is_bound(&pdev.dev)) {
    ret = __pci_pwrctrl_power_on_device(&pdev.dev);
    } else {
// FIXME: Use blocking wait instead of probe deferral
    dev_dbg(&pdev.dev, "driver is not bound\n");
    ret = -EPROBE_DEFER;
    }
    }
    platform_device_put(pdev);
    return ret;
    }
//
// pci_pwrctrl_power_on_devices - Power on pwrctrl devices
//
// @parent: PCI host controller device
//
// Recursively traverse all pwrctrl devices for the devicetree hierarchy
// below the specified PCI host controller and power them on in a depth
// first manner. On error, all powered on devices will be powered off.
//
// Return: 0 on success, -EPROBE_DEFER if any pwrctrl driver is not bound, an
// appropriate error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_power_on_devices(parent: *mut device) -> c_int {
    int pci_pwrctrl_power_on_devices(struct device *parent)
    {
    struct device_node *np = parent.of_node;
    struct device_node *child = core::ptr::null_mut();
    int ret;
    for_each_available_child_of_node(np, child) {
    ret = pci_pwrctrl_power_on_device(child);
    if (ret)
    goto err_power_off;
    }
    return 0;
    err_power_off:
    for_each_available_child_of_node_scoped(np, tmp) {
    if (tmp == child)
    break;
    pci_pwrctrl_power_off_device(tmp);
    }
    of_node_put(child);
    return ret;
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_power_on_devices);
    static int pci_pwrctrl_create_device(struct device_node *np,
    struct device *parent)
    {
    struct platform_device *pdev;
    int ret;
    for_each_available_child_of_node_scoped(np, child) {
    ret = pci_pwrctrl_create_device(child, parent);
    if (ret)
    return ret;
    }
// Bail out if the platform device is already available for the node
    pdev = of_find_device_by_node(np);
    if (pdev) {
    platform_device_put(pdev);
    return 0;
    }
    if (!pci_pwrctrl_is_required(np)) {
    dev_dbg(parent, "Skipping OF node: %s\n", np.name);
    return 0;
    }
// Now create the pwrctrl device
    pdev = of_platform_device_create(np, core::ptr::null_mut(), parent);
    if (!pdev) {
    dev_err(parent, "Failed to create pwrctrl device for node: %s\n", np.name);
    return -EINVAL;
    }
    return 0;
    }
//
// pci_pwrctrl_create_devices - Create pwrctrl devices
//
// @parent: PCI host controller device
//
// Recursively create pwrctrl devices for the devicetree hierarchy below
// the specified PCI host controller in a depth first manner. On error, all
// created devices will be destroyed.
//
// Return: 0 on success, negative error number on error.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_create_devices(parent: *mut device) -> c_int {
    int pci_pwrctrl_create_devices(struct device *parent)
    {
    int ret;
    for_each_available_child_of_node_scoped(parent.of_node, child) {
    ret = pci_pwrctrl_create_device(child, parent);
    if (ret) {
    pci_pwrctrl_destroy_devices(parent);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_create_devices);
#[no_mangle]
unsafe extern "C" fn pci_pwrctrl_destroy_device(np: *mut device_node) {
    static void pci_pwrctrl_destroy_device(struct device_node *np)
    {
    struct platform_device *pdev;
    for_each_available_child_of_node_scoped(np, child)
    pci_pwrctrl_destroy_device(child);
    pdev = of_find_device_by_node(np);
    if (!pdev)
    return;
    of_device_unregister(pdev);
    platform_device_put(pdev);
    of_node_clear_flag(np, OF_POPULATED);
    }
//
// pci_pwrctrl_destroy_devices - Destroy pwrctrl devices
//
// @parent: PCI host controller device
//
// Recursively destroy pwrctrl devices for the devicetree hierarchy below
// the specified PCI host controller in a depth first manner.
//
#[no_mangle]
pub unsafe extern "C" fn pci_pwrctrl_destroy_devices(parent: *mut device) {
    void pci_pwrctrl_destroy_devices(struct device *parent)
    {
    struct device_node *np = parent.of_node;
    for_each_available_child_of_node_scoped(np, child)
    pci_pwrctrl_destroy_device(child);
    }
    EXPORT_SYMBOL_GPL(pci_pwrctrl_destroy_devices);
    MODULE_AUTHOR("Bartosz Golaszewski <bartosz.golaszewski@linaro.org>");
    MODULE_DESCRIPTION("PCI Device Power Control core driver");
    MODULE_LICENSE("GPL");
