//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/port-mapper.c
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
// USB Type-C Connector Class Port Mapping Utility
//
// Copyright (C) 2021, Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn typec_aggregate_bind(dev: *mut device) -> c_int {
    static int typec_aggregate_bind(struct device *dev)
    {
    struct typec_port *port = to_typec_port(dev);
    return component_bind_all(dev, &port.con);
    }
#[no_mangle]
unsafe extern "C" fn typec_aggregate_unbind(dev: *mut device) {
    static void typec_aggregate_unbind(struct device *dev)
    {
    struct typec_port *port = to_typec_port(dev);
    component_unbind_all(dev, &port.con);
    }
    static const struct component_master_ops typec_aggregate_ops = {
    .bind = typec_aggregate_bind,
    .unbind = typec_aggregate_unbind,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct each_port_arg {
    pub port: *mut typec_port,
    pub match: *mut component_match,
}

#[no_mangle]
unsafe extern "C" fn usb4_port_compare(dev: *mut device, fwnode: *mut c_void) -> c_int {
    static int usb4_port_compare(struct device *dev, void *fwnode)
    {
    return usb4_usb3_port_match(dev, fwnode);
    }
#[no_mangle]
unsafe extern "C" fn typec_port_compare(dev: *mut device, fwnode: *mut c_void) -> c_int {
    static int typec_port_compare(struct device *dev, void *fwnode)
    {
    return device_match_fwnode(dev, fwnode);
    }
#[no_mangle]
unsafe extern "C" fn typec_port_match(dev: *mut device, data: *mut c_void) -> c_int {
    static int typec_port_match(struct device *dev, void *data)
    {
    struct acpi_device *adev = to_acpi_device(dev);
    struct each_port_arg *arg = data;
    struct acpi_device *con_adev;
    con_adev = ACPI_COMPANION(&arg.port.dev);
    if (con_adev == adev)
    return 0;
    if (con_adev.pld_crc == adev.pld_crc)	{
    struct fwnode_handle *adev_fwnode = acpi_fwnode_handle(adev);
    component_match_add(&arg.port.dev, &arg.match, typec_port_compare,
    adev_fwnode);
//
// If dev is USB 3.x port, it may have reference to the
// USB4 host interface in which case we can also link the
// Type-C port with the USB4 port.
//
    if (fwnode_property_present(adev_fwnode, "usb4-host-interface"))
    component_match_add(&arg.port.dev, &arg.match,
    usb4_port_compare, adev_fwnode);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn typec_link_ports(con: *mut typec_port) -> c_int {
    int typec_link_ports(struct typec_port *con)
    {
    let mut arg: each_port_arg = { .port = con, .match = core::ptr::null_mut() };
    if (!has_acpi_companion(&con.dev))
    return 0;
    acpi_bus_for_each_dev(typec_port_match, &arg);
    if (!arg.match)
    return 0;
//
// REVISIT: Now each connector can have only a single component master.
// So far only the USB ports connected to the USB Type-C connector share
// the _PLD with it, but if there one day is something else (like maybe
// the DisplayPort ACPI device object) that also shares the _PLD with
// the connector, every one of those needs to have its own component
// master, because each different type of component needs to be bind to
// the connector independently of the other components. That requires
// improvements to the component framework. Right now you can only have
// one master per device.
//
    return component_master_add_with_match(&con.dev, &typec_aggregate_ops, arg.match);
    }
#[no_mangle]
pub unsafe extern "C" fn typec_unlink_ports(con: *mut typec_port) {
    void typec_unlink_ports(struct typec_port *con)
    {
    if (has_acpi_companion(&con.dev))
    component_master_del(&con.dev, &typec_aggregate_ops);
    }
