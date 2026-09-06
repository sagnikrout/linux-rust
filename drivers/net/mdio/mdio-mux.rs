//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux.c
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
// Copyright (C) 2011, 2012 Cavium, Inc.
//

    struct mdio_mux_child_bus;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_mux_parent_bus {
    pub mii_bus: *mut mii_bus,
    pub current_child: c_int,
    pub parent_id: c_int,
    pub switch_data: *mut c_void,
    pub data): *mut *mut int (switch_fn)(int current_child, int desired_child, void,
// List of our children linked through their next fields.
    pub children: *mut mdio_mux_child_bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_mux_child_bus {
    pub mii_bus: *mut mii_bus,
    pub parent: *mut mdio_mux_parent_bus,
    pub next: *mut mdio_mux_child_bus,
    pub bus_number: c_int,
}

//
// The parent bus' lock is used to order access to the switch_fn.
//
#[no_mangle]
unsafe extern "C" fn mdio_mux_read(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int {
    static int mdio_mux_read(struct mii_bus *bus, int phy_id, int regnum)
    {
    struct mdio_mux_child_bus *cb = bus.priv;
    struct mdio_mux_parent_bus *pb = cb.parent;
    int r;
    mutex_lock_nested(&pb.mii_bus.mdio_lock, MDIO_MUTEX_MUX);
    r = pb.switch_fn(pb.current_child, cb.bus_number, pb.switch_data);
    if (r)
    goto out;
    pb.current_child = cb.bus_number;
    r = pb.mii_bus.read(pb.mii_bus, phy_id, regnum);
    out:
    mutex_unlock(&pb.mii_bus.mdio_lock);
    return r;
    }
    static int mdio_mux_read_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum)
    {
    struct mdio_mux_child_bus *cb = bus.priv;
    struct mdio_mux_parent_bus *pb = cb.parent;
    int r;
    mutex_lock_nested(&pb.mii_bus.mdio_lock, MDIO_MUTEX_MUX);
    r = pb.switch_fn(pb.current_child, cb.bus_number, pb.switch_data);
    if (r)
    goto out;
    pb.current_child = cb.bus_number;
    r = pb.mii_bus.read_c45(pb.mii_bus, phy_id, dev_addr, regnum);
    out:
    mutex_unlock(&pb.mii_bus.mdio_lock);
    return r;
    }
//
// The parent bus' lock is used to order access to the switch_fn.
//
    static int mdio_mux_write(struct mii_bus *bus, int phy_id,
    int regnum, u16 val)
    {
    struct mdio_mux_child_bus *cb = bus.priv;
    struct mdio_mux_parent_bus *pb = cb.parent;
    int r;
    mutex_lock_nested(&pb.mii_bus.mdio_lock, MDIO_MUTEX_MUX);
    r = pb.switch_fn(pb.current_child, cb.bus_number, pb.switch_data);
    if (r)
    goto out;
    pb.current_child = cb.bus_number;
    r = pb.mii_bus.write(pb.mii_bus, phy_id, regnum, val);
    out:
    mutex_unlock(&pb.mii_bus.mdio_lock);
    return r;
    }
    static int mdio_mux_write_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum, u16 val)
    {
    struct mdio_mux_child_bus *cb = bus.priv;
    struct mdio_mux_parent_bus *pb = cb.parent;
    int r;
    mutex_lock_nested(&pb.mii_bus.mdio_lock, MDIO_MUTEX_MUX);
    r = pb.switch_fn(pb.current_child, cb.bus_number, pb.switch_data);
    if (r)
    goto out;
    pb.current_child = cb.bus_number;
    r = pb.mii_bus.write_c45(pb.mii_bus, phy_id, dev_addr, regnum, val);
    out:
    mutex_unlock(&pb.mii_bus.mdio_lock);
    return r;
    }
    static int parent_count;
#[no_mangle]
unsafe extern "C" fn mdio_mux_uninit_children(pb: *mut mdio_mux_parent_bus) {
    static void mdio_mux_uninit_children(struct mdio_mux_parent_bus *pb)
    {
    struct mdio_mux_child_bus *cb = pb.children;
    while (cb) {
    mdiobus_unregister(cb.mii_bus);
    mdiobus_free(cb.mii_bus);
    cb = cb.next;
    }
    }
    int mdio_mux_init(struct device *dev,
    struct device_node *mux_node,
    int (*switch_fn)(int cur, int desired, void *data),
    void **mux_handle,
    void *data,
    struct mii_bus *mux_bus)
    {
    struct device_node *parent_bus_node;
    struct device_node *child_bus_node;
    int r, ret_val;
    struct mii_bus *parent_bus;
    struct mdio_mux_parent_bus *pb;
    struct mdio_mux_child_bus *cb;
    if (!mux_node)
    return -ENODEV;
    if (!mux_bus) {
    parent_bus_node = of_parse_phandle(mux_node,
    "mdio-parent-bus", 0);
    if (!parent_bus_node)
    return -ENODEV;
    parent_bus = of_mdio_find_bus(parent_bus_node);
    if (!parent_bus) {
    ret_val = -EPROBE_DEFER;
    goto err_parent_bus;
    }
    } else {
    parent_bus_node = core::ptr::null_mut();
    parent_bus = mux_bus;
    get_device(&parent_bus.dev);
    }
    pb = devm_kzalloc(dev, sizeof(*pb), GFP_KERNEL);
    if (!pb) {
    ret_val = -ENOMEM;
    goto err_pb_kz;
    }
    pb.switch_data = data;
    pb.switch_fn = switch_fn;
    pb.current_child = -1;
    pb.parent_id = parent_count++;
    pb.mii_bus = parent_bus;
    ret_val = -ENODEV;
    for_each_available_child_of_node(mux_node, child_bus_node) {
    int v;
    r = of_property_read_u32(child_bus_node, "reg", &v);
    if (r) {
    dev_err(dev,
    "Error: Failed to find reg for child %pOF: %pe\n",
    child_bus_node, ERR_PTR(r));
    continue;
    }
    cb = devm_kzalloc(dev, sizeof(*cb), GFP_KERNEL);
    if (!cb) {
    ret_val = -ENOMEM;
    goto err_loop;
    }
    cb.bus_number = v;
    cb.parent = pb;
    cb.mii_bus = mdiobus_alloc();
    if (!cb.mii_bus) {
    ret_val = -ENOMEM;
    goto err_loop;
    }
    cb.mii_bus.priv = cb;
    cb.mii_bus.name = "mdio_mux";
    snprintf(cb.mii_bus.id, MII_BUS_ID_SIZE, "%s-%x.%x",
    cb.mii_bus.name, pb.parent_id, v);
    cb.mii_bus.parent = dev;
    if (parent_bus.read)
    cb.mii_bus.read = mdio_mux_read;
    if (parent_bus.write)
    cb.mii_bus.write = mdio_mux_write;
    if (parent_bus.read_c45)
    cb.mii_bus.read_c45 = mdio_mux_read_c45;
    if (parent_bus.write_c45)
    cb.mii_bus.write_c45 = mdio_mux_write_c45;
    r = of_mdiobus_register(cb.mii_bus, child_bus_node);
    if (r) {
    mdiobus_free(cb.mii_bus);
    if (r == -EPROBE_DEFER) {
    ret_val = r;
    goto err_loop;
    }
    devm_kfree(dev, cb);
    dev_err(dev,
    "Error: Failed to register MDIO bus for child %pOF: %pe\n",
    child_bus_node, ERR_PTR(r));
    } else {
    cb.next = pb.children;
    pb.children = cb;
    }
    }
    if (pb.children) {
// mux_handle = pb;
    return 0;
    }
    dev_err(dev, "Error: No acceptable child buses found\n");
    err_loop:
    mdio_mux_uninit_children(pb);
    of_node_put(child_bus_node);
    err_pb_kz:
    put_device(&parent_bus.dev);
    err_parent_bus:
    of_node_put(parent_bus_node);
    return ret_val;
    }
    EXPORT_SYMBOL_GPL(mdio_mux_init);
#[no_mangle]
pub unsafe extern "C" fn mdio_mux_uninit(mux_handle: *mut c_void) {
    void mdio_mux_uninit(void *mux_handle)
    {
    struct mdio_mux_parent_bus *pb = mux_handle;
    mdio_mux_uninit_children(pb);
    put_device(&pb.mii_bus.dev);
    }
    EXPORT_SYMBOL_GPL(mdio_mux_uninit);
    MODULE_DESCRIPTION(DRV_DESCRIPTION);
    MODULE_AUTHOR("David Daney");
    MODULE_LICENSE("GPL v2");
