//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/onboard_usb_dev_pdevs.c
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
// API for creating and destroying USB onboard platform devices
//
// Copyright (c) 2022, Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdev_list_entry {
    pub pdev: *mut platform_device,
    pub node: list_head,
}

#[no_mangle]
unsafe extern "C" fn of_is_onboard_usb_dev(np: *mut device_node) -> bool {
    static bool of_is_onboard_usb_dev(struct device_node *np)
    {
    return !!of_match_node(onboard_dev_match, np);
    }
//
// onboard_dev_create_pdevs -- create platform devices for onboard USB devices
// @parent_hub	: parent hub to scan for connected onboard devices
// @pdev_list	: list of onboard platform devices owned by the parent hub
//
// Creates a platform device for each supported onboard device that is connected
// to the given parent hub. The platform device is in charge of initializing the
// device (enable regulators, take the device out of reset, ...). For onboard
// hubs, it can optionally control whether the device remains powered during
// system suspend or not.
//
// To keep track of the platform devices they are added to a list that is owned
// by the parent hub.
//
// Some background about the logic in this function, which can be a bit hard
// to follow:
//
// Root hubs don't have dedicated device tree nodes, but use the node of their
// HCD. The primary and secondary HCD are usually represented by a single DT
// node. That means the root hubs of the primary and secondary HCD share the
// same device tree node (the HCD node). As a result this function can be called
// twice with the same DT node for root hubs. We only want to create a single
// platform device for each physical onboard device, hence for root hubs the
// loop is only executed for the root hub of the primary HCD. Since the function
// scans through all child nodes it still creates pdevs for onboard devices
// connected to the root hub of the secondary HCD if needed.
//
// Further there must be only one platform device for onboard hubs with a peer
// hub (the hub is a single physical device). To achieve this two measures are
// taken: pdevs for onboard hubs with a peer are only created when the function
// is called on behalf of the parent hub that is connected to the primary HCD
// (directly or through other hubs). For onboard hubs connected to root hubs
// the function processes the nodes of both peers. A platform device is only
// created if the peer hub doesn't have one already.
//
#[no_mangle]
pub unsafe extern "C" fn onboard_dev_create_pdevs(parent_hub: *mut usb_device, pdev_list: *mut list_head) {
    void onboard_dev_create_pdevs(struct usb_device *parent_hub, struct list_head *pdev_list)
    {
    int i;
    struct usb_hcd *hcd = bus_to_hcd(parent_hub.bus);
    struct device_node *np, *npc;
    struct platform_device *pdev;
    struct pdev_list_entry *pdle;
    if (!parent_hub.dev.of_node)
    return;
    if (!parent_hub.parent && !usb_hcd_is_primary_hcd(hcd))
    return;
    for (i = 1; i <= parent_hub.maxchild; i++) {
    np = usb_of_get_device_node(parent_hub, i);
    if (!np)
    continue;
    if (!of_is_onboard_usb_dev(np))
    goto node_put;
    npc = of_parse_phandle(np, "peer-hub", 0);
    if (npc) {
    if (!usb_hcd_is_primary_hcd(hcd)) {
    of_node_put(npc);
    goto node_put;
    }
    pdev = of_find_device_by_node(npc);
    of_node_put(npc);
    if (pdev) {
    put_device(&pdev.dev);
    goto node_put;
    }
    }
    pdev = of_platform_device_create(np, core::ptr::null_mut(), &parent_hub.dev);
    if (!pdev) {
    dev_err(&parent_hub.dev,
    "failed to create platform device for onboard dev '%pOF'\n", np);
    goto node_put;
    }
    pdle = kzalloc_obj(*pdle);
    if (!pdle) {
    of_platform_device_destroy(&pdev.dev, core::ptr::null_mut());
    goto node_put;
    }
    pdle.pdev = pdev;
    list_add(&pdle.node, pdev_list);
    node_put:
    of_node_put(np);
    }
    }
    EXPORT_SYMBOL_GPL(onboard_dev_create_pdevs);
//
// onboard_dev_destroy_pdevs -- free resources of onboard platform devices
// @pdev_list	: list of onboard platform devices
//
// Destroys the platform devices in the given list and frees the memory associated
// with the list entry.
//
#[no_mangle]
pub unsafe extern "C" fn onboard_dev_destroy_pdevs(pdev_list: *mut list_head) {
    void onboard_dev_destroy_pdevs(struct list_head *pdev_list)
    {
    struct pdev_list_entry *pdle, *tmp;
    list_for_each_entry_safe(pdle, tmp, pdev_list, node) {
    list_del(&pdle.node);
    of_platform_device_destroy(&pdle.pdev.dev, core::ptr::null_mut());
    kfree(pdle);
    }
    }
    EXPORT_SYMBOL_GPL(onboard_dev_destroy_pdevs);
