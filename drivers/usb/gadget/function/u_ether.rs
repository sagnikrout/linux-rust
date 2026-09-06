//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_ether.h
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
// u_ether.h -- interface to USB gadget "ethernet link" utilities
//
// Copyright (C) 2003-2005,2008 David Brownell
// Copyright (C) 2003-2004 Robert Schwebel, Benedikt Spranger
// Copyright (C) 2008 Nokia Corporation
//

pub const QMULT_DEFAULT: c_int = 5;
//
// dev_addr: initial value
// changed by "ifconfig usb0 hw ether xx:xx:xx:xx:xx:xx"
// host_addr: this address is invisible to ifconfig
//

//
// This represents the USB side of an "ethernet" link, managed by a USB
// function which provides control and (maybe) framing.  Two functions
// in different configurations could share the same ethernet link/netdev,
// using different host interaction models.
//
// There is a current limitation that only one instance of this link may
// be present in any given configuration.  When that's a problem, network
// layer facilities can be used to package multiple logical links on this
// single "physical" one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gether {
    pub func: usb_function,
// updated by gether_{connect,disconnect}
    pub ioport: *mut eth_dev,
// endpoints handle full and/or high speeds
    pub in_ep: *mut usb_ep,
    pub out_ep: *mut usb_ep,
    pub is_zlp_ok: bool,
    pub cdc_filter: u16,
// hooks for added framing, as needed for RNDIS and EEM.
    pub header_len: u32,
// NCM requires fixed size bundles
    pub is_fixed: bool,
    pub fixed_out_len: u32,
    pub fixed_in_len: u32,
    pub supports_multi_frame: bool,
    pub skb): *mut sk_buff,
    pub list): *mut sk_buff_head,
// called on network open/close
    pub ): *mut *mut void (open)(struct gether,
    pub ): *mut *mut void (close)(struct gether,
    pub is_suspend: bool,
}

// variant of gether_setup that allows customizing network device name
// netdev setup/teardown as directed by the gadget driver
// gether_setup - initialize one ethernet-over-usb link
// @g: gadget to associated with these links
// @ethaddr: NULL, or a buffer in which the ethernet address of the
// host side of the link is recorded
// Context: may sleep
//
// This sets up the single network link that may be exported by a
// gadget driver using this framework.  The link layer addresses are
// set up using module parameters.
//
// Returns a eth_dev pointer on success, or an ERR_PTR on failure
//
extern "C" {
    pub fn gether_setup_name(_arg: g, _arg: dev_addr, _arg: host_addr, _arg: ethaddr, _arg: qmult, _arg: "usb") -> return;
}
//
// variant of gether_setup_default that allows customizing
// network device name
//
// gether_register_netdev - register the net device
// @net: net device to register
//
// Registers the net device associated with this ethernet-over-usb link
//
extern "C" {
    pub fn gether_register_netdev(net: *mut net_device) -> c_int;
}
// gether_setup_default - initialize one ethernet-over-usb link
// Context: may sleep
//
// This sets up the single network link that may be exported by a
// gadget driver using this framework.  The link layer addresses
// are set to random values.
//
// Returns negative errno, or zero on success
//
extern "C" {
    pub fn gether_setup_name_default(_arg: "usb") -> return;
}
//
// gether_set_gadget - initialize one ethernet-over-usb link with a gadget
// @net: device representing this link
// @g: the gadget to initialize with
//
// This associates one ethernet-over-usb link with a gadget.
//
extern "C" {
    pub fn gether_set_gadget(net: *mut net_device, g: *mut usb_gadget);
}
//
// gether_attach_gadget - Reparent net_device to the gadget device.
// @net: The network device to reparent.
// @g: The target USB gadget device to parent to.
//
// This function moves the network device to be a child of the USB gadget
// device in the device hierarchy. This is typically done when the function
// is bound to a configuration.
//
// Returns 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn gether_attach_gadget(net: *mut net_device, g: *mut usb_gadget) -> c_int;
}
//
// gether_detach_gadget - Detach net_device from its gadget parent.
// @net: The network device to detach.
//
// This function moves the network device to be a child of the virtual
// devices parent, effectively detaching it from the USB gadget device
// hierarchy. This is typically done when the function is unbound
// from a configuration but the instance is not yet freed.
//
extern "C" {
    pub fn gether_detach_gadget(net: *mut net_device);
}
//
// gether_set_dev_addr - initialize an ethernet-over-usb link with eth address
// @net: device representing this link
// @dev_addr: eth address of this device
//
// This sets the device-side Ethernet address of this ethernet-over-usb link
// if dev_addr is correct.
// Returns negative errno if the new address is incorrect.
//
extern "C" {
    pub fn gether_set_dev_addr(net: *mut net_device, dev_addr: *const c_char) -> c_int;
}
//
// gether_get_dev_addr - get an ethernet-over-usb link eth address
// @net: device representing this link
// @dev_addr: place to store device's eth address
// @len: length of the @dev_addr buffer
//
// This gets the device-side Ethernet address of this ethernet-over-usb link.
// Returns zero on success, else negative errno.
//
extern "C" {
    pub fn gether_get_dev_addr(net: *mut net_device, dev_addr: *mut c_char, len: c_int) -> c_int;
}
//
// gether_set_host_addr - initialize an ethernet-over-usb link with host address
// @net: device representing this link
// @host_addr: eth address of the host
//
// This sets the host-side Ethernet address of this ethernet-over-usb link
// if host_addr is correct.
// Returns negative errno if the new address is incorrect.
//
extern "C" {
    pub fn gether_set_host_addr(net: *mut net_device, host_addr: *const c_char) -> c_int;
}
//
// gether_get_host_addr - get an ethernet-over-usb link host address
// @net: device representing this link
// @host_addr: place to store eth address of the host
// @len: length of the @host_addr buffer
//
// This gets the host-side Ethernet address of this ethernet-over-usb link.
// Returns zero on success, else negative errno.
//
extern "C" {
    pub fn gether_get_host_addr(net: *mut net_device, host_addr: *mut c_char, len: c_int) -> c_int;
}
//
// gether_get_host_addr_cdc - get an ethernet-over-usb link host address
// @net: device representing this link
// @host_addr: place to store eth address of the host
// @len: length of the @host_addr buffer
//
// This gets the CDC formatted host-side Ethernet address of this
// ethernet-over-usb link.
// Returns zero on success, else negative errno.
//
extern "C" {
    pub fn gether_get_host_addr_cdc(net: *mut net_device, host_addr: *mut c_char, len: c_int) -> c_int;
}
//
// gether_get_host_addr_u8 - get an ethernet-over-usb link host address
// @net: device representing this link
// @host_mac: place to store the eth address of the host
//
// This gets the binary formatted host-side Ethernet address of this
// ethernet-over-usb link.
//
extern "C" {
    pub fn gether_get_host_addr_u8(net: *mut net_device, host_mac[ETH_ALEN]: u8);
}
//
// gether_set_qmult - initialize an ethernet-over-usb link with a multiplier
// @net: device representing this link
// @qmult: queue multiplier
//
// This sets the queue length multiplier of this ethernet-over-usb link.
// For higher speeds use longer queues.
//
extern "C" {
    pub fn gether_set_qmult(net: *mut net_device, qmult: unsigned);
}
//
// gether_get_qmult - get an ethernet-over-usb link multiplier
// @net: device representing this link
//
// This gets the queue length multiplier of this ethernet-over-usb link.
//
extern "C" {
    pub fn gether_get_qmult(net: *mut net_device) -> unsigned;
}
//
// gether_get_ifname - get an ethernet-over-usb link interface name
// @net: device representing this link
// @name: place to store the interface name
// @len: length of the @name buffer
//
// This gets the interface name of this ethernet-over-usb link.
// Returns zero on success, else negative errno.
//
extern "C" {
    pub fn gether_get_ifname(net: *mut net_device, name: *mut c_char, len: c_int) -> c_int;
}
//
// gether_set_ifname - set an ethernet-over-usb link interface name
// @net: device representing this link
// @name: new interface name
// @len: length of @name
//
// This sets the interface name of this ethernet-over-usb link.
// A single terminating newline, if any, is ignored.
// Returns zero on success, else negative errno.
//
extern "C" {
    pub fn gether_set_ifname(net: *mut net_device, name: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn gether_cleanup(dev: *mut eth_dev);
}
extern "C" {
    pub fn gether_suspend(link: *mut gether);
}
extern "C" {
    pub fn gether_resume(link: *mut gether);
}
// connect/disconnect is handled by individual functions
extern "C" {
    pub fn gether_disconnect(: *mut gether);
}
// Some controllers can't support CDC Ethernet (ECM) ...
// Everything else is *presumably* fine ... but this is a bit
// chancy, so be **CERTAIN** there are no hardware issues with
// your controller.  Add it above if it can't handle CDC.
//
// peak (theoretical) bulk transfer rate in bits-per-second
