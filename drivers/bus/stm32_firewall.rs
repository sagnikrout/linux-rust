//! Automatically rewritten from C to Rust
//! Source: drivers/bus/stm32_firewall.c
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
// Copyright (C) 2023, STMicroelectronics - All Rights Reserved
//

// Corresponds to STM32_FIREWALL_MAX_EXTRA_ARGS + firewall ID

    static LIST_HEAD(firewall_controller_list);
    static DEFINE_MUTEX(firewall_controller_list_lock);
// Firewall device API
    int stm32_firewall_get_firewall(struct device_node *np, struct stm32_firewall *firewall,
    unsigned int nb_firewall)
    {
    struct stm32_firewall_controller *ctrl;
    struct of_phandle_iterator it;
    unsigned int i, j = 0;
    int err;
    if (!firewall || !nb_firewall)
    return -EINVAL;
// Parse property with phandle parsed out
    of_for_each_phandle(&it, err, np, "access-controllers", "#access-controller-cells", 0) {
    struct of_phandle_args provider_args;
    struct device_node *provider = it.node;
    const char *fw_entry;
    let mut match: bool = false;
    if (err) {
    pr_err("Unable to get access-controllers property for node %s\n, err: %d",
    np.full_name, err);
    of_node_put(provider);
    return err;
    }
    if (j >= nb_firewall) {
    pr_err("Too many firewall controllers");
    of_node_put(provider);
    return -EINVAL;
    }
    provider_args.args_count = of_phandle_iterator_args(&it, provider_args.args,
    STM32_FIREWALL_MAX_ARGS);
// Check if the parsed phandle corresponds to a registered firewall controller
    mutex_lock(&firewall_controller_list_lock);
    list_for_each_entry(ctrl, &firewall_controller_list, entry) {
    if (ctrl.dev.of_node.phandle == it.phandle) {
    match = true;
    firewall[j].firewall_ctrl = ctrl;
    break;
    }
    }
    mutex_unlock(&firewall_controller_list_lock);
    if (!match) {
    firewall[j].firewall_ctrl = core::ptr::null_mut();
    pr_err("No firewall controller registered for %s\n", np.full_name);
    of_node_put(provider);
    return -ENODEV;
    }
    err = of_property_read_string_index(np, "access-controller-names", j, &fw_entry);
    if (err == 0)
    firewall[j].entry = fw_entry;
// Handle the case when there are no arguments given along with the phandle
    if (provider_args.args_count < 0 ||
    provider_args.args_count > STM32_FIREWALL_MAX_ARGS) {
    of_node_put(provider);
    return -EINVAL;
    } else if (provider_args.args_count == 0) {
    firewall[j].extra_args_size = 0;
    firewall[j].firewall_id = U32_MAX;
    j++;
    continue;
    }
// The firewall ID is always the first argument
    firewall[j].firewall_id = provider_args.args[0];
// Extra args start at the second argument
    for (i = 0; i < provider_args.args_count - 1; i++)
    firewall[j].extra_args[i] = provider_args.args[i + 1];
// Remove the firewall ID arg that is not an extra argument
    firewall[j].extra_args_size = provider_args.args_count - 1;
    j++;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_get_firewall);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_grant_access(firewall: *mut stm32_firewall) -> c_int {
    int stm32_firewall_grant_access(struct stm32_firewall *firewall)
    {
    struct stm32_firewall_controller *firewall_controller;
    if (!firewall || firewall.firewall_id == U32_MAX)
    return -EINVAL;
    firewall_controller = firewall.firewall_ctrl;
    if (!firewall_controller)
    return -ENODEV;
    return firewall_controller.grant_access(firewall_controller, firewall.firewall_id);
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_grant_access);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_grant_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32) -> c_int {
    int stm32_firewall_grant_access_by_id(struct stm32_firewall *firewall, u32 subsystem_id)
    {
    struct stm32_firewall_controller *firewall_controller;
    if (!firewall || subsystem_id == U32_MAX || firewall.firewall_id == U32_MAX)
    return -EINVAL;
    firewall_controller = firewall.firewall_ctrl;
    if (!firewall_controller)
    return -ENODEV;
    return firewall_controller.grant_access(firewall_controller, subsystem_id);
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_grant_access_by_id);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_release_access(firewall: *mut stm32_firewall) {
    void stm32_firewall_release_access(struct stm32_firewall *firewall)
    {
    struct stm32_firewall_controller *firewall_controller;
    if (!firewall || firewall.firewall_id == U32_MAX) {
    pr_debug("Incorrect arguments when releasing a firewall access\n");
    return;
    }
    firewall_controller = firewall.firewall_ctrl;
    if (!firewall_controller) {
    pr_debug("No firewall controller to release\n");
    return;
    }
    firewall_controller.release_access(firewall_controller, firewall.firewall_id);
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_release_access);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_release_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32) {
    void stm32_firewall_release_access_by_id(struct stm32_firewall *firewall, u32 subsystem_id)
    {
    struct stm32_firewall_controller *firewall_controller;
    if (!firewall || subsystem_id == U32_MAX || firewall.firewall_id == U32_MAX) {
    pr_debug("Incorrect arguments when releasing a firewall access");
    return;
    }
    firewall_controller = firewall.firewall_ctrl;
    if (!firewall_controller) {
    pr_debug("No firewall controller to release");
    return;
    }
    firewall_controller.release_access(firewall_controller, subsystem_id);
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_release_access_by_id);
    int stm32_firewall_get_grant_all_access(struct device *dev, struct stm32_firewall **firewall,
    int *nb_firewall)
    {
    struct stm32_firewall *loc_firewall;
    int err;
    int i;
// nb_firewall = of_count_phandle_with_args(dev->of_node, "access-controllers",
    "#access-controller-cells");
    if (*nb_firewall < 0)
    return *nb_firewall;
    if (!*nb_firewall) {
// firewall = NULL;
    return 0;
    }
    loc_firewall = devm_kcalloc(dev, *nb_firewall, sizeof(*loc_firewall), GFP_KERNEL);
    if (!loc_firewall)
    return -ENOMEM;
// Get stm32 firewall information
    err = stm32_firewall_get_firewall(dev.of_node, loc_firewall, *nb_firewall);
    if (err)
    return err;
    for (i = 0; i < *nb_firewall; i++) {
    err = stm32_firewall_grant_access(&loc_firewall[i]);
    if (err) {
    while (i--)
    stm32_firewall_release_access(&loc_firewall[i]);
    return err;
    }
    }
// firewall = loc_firewall;
    return 0;
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_get_grant_all_access);
// Firewall controller API
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_controller_register(firewall_controller: *mut stm32_firewall_controller) -> c_int {
    int stm32_firewall_controller_register(struct stm32_firewall_controller *firewall_controller)
    {
    struct stm32_firewall_controller *ctrl;
    if (!firewall_controller)
    return -ENODEV;
    pr_info("Registering %s firewall controller\n", firewall_controller.name);
    mutex_lock(&firewall_controller_list_lock);
    list_for_each_entry(ctrl, &firewall_controller_list, entry) {
    if (ctrl == firewall_controller) {
    pr_debug("%s firewall controller already registered\n",
    firewall_controller.name);
    mutex_unlock(&firewall_controller_list_lock);
    return 0;
    }
    }
    list_add_tail(&firewall_controller.entry, &firewall_controller_list);
    mutex_unlock(&firewall_controller_list_lock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_controller_register);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_controller_unregister(firewall_controller: *mut stm32_firewall_controller) {
    void stm32_firewall_controller_unregister(struct stm32_firewall_controller *firewall_controller)
    {
    struct stm32_firewall_controller *ctrl;
    let mut controller_removed: bool = false;
    if (!firewall_controller) {
    pr_debug("Null reference while unregistering firewall controller\n");
    return;
    }
    mutex_lock(&firewall_controller_list_lock);
    list_for_each_entry(ctrl, &firewall_controller_list, entry) {
    if (ctrl == firewall_controller) {
    controller_removed = true;
    list_del_init(&ctrl.entry);
    break;
    }
    }
    mutex_unlock(&firewall_controller_list_lock);
    if (!controller_removed)
    pr_debug("There was no firewall controller named %s to unregister\n",
    firewall_controller.name);
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_controller_unregister);
#[no_mangle]
pub unsafe extern "C" fn stm32_firewall_populate_bus(firewall_controller: *mut stm32_firewall_controller) -> c_int {
    int stm32_firewall_populate_bus(struct stm32_firewall_controller *firewall_controller)
    {
    struct stm32_firewall *firewalls;
    struct device *parent;
    unsigned int i;
    int len;
    int err;
    parent = firewall_controller.dev;
    dev_dbg(parent, "Populating %s system bus\n", dev_name(firewall_controller.dev));
    for_each_available_child_of_node_scoped(dev_of_node(parent), child) {
// The access-controllers property is mandatory for firewall bus devices
    len = of_count_phandle_with_args(child, "access-controllers",
    "#access-controller-cells");
    if (len <= 0)
    return -EINVAL;
    firewalls = kzalloc_objs(*firewalls, len);
    if (!firewalls)
    return -ENOMEM;
    err = stm32_firewall_get_firewall(child, firewalls, (unsigned int)len);
    if (err) {
    kfree(firewalls);
    return err;
    }
    for (i = 0; i < len; i++) {
    if (firewall_controller.grant_access(firewalls[i].firewall_ctrl,
    firewalls[i].firewall_id)) {
//
// Peripheral access not allowed or not defined.
// Mark the node as populated so platform bus won't probe it
//
    of_detach_node(child);
    dev_err(parent, "%s: Device driver will not be probed\n",
    child.full_name);
    }
    }
    kfree(firewalls);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(stm32_firewall_populate_bus);
