//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wwan.h
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
// Copyright (c) 2021, Linaro Ltd <loic.poulain@linaro.org>

//
// enum wwan_port_type - WWAN port types
// @WWAN_PORT_AT: AT commands
// @WWAN_PORT_MBIM: Mobile Broadband Interface Model control
// @WWAN_PORT_QMI: Qcom modem/MSM interface for modem control
// @WWAN_PORT_QCDM: Qcom Modem diagnostic interface
// @WWAN_PORT_FIREHOSE: XML based command protocol
// @WWAN_PORT_XMMRPC: Control protocol for Intel XMM modems
// @WWAN_PORT_FASTBOOT: Fastboot protocol control
// @WWAN_PORT_ADB: ADB protocol control
// @WWAN_PORT_MIPC: MTK MIPC diagnostic interface
// @WWAN_PORT_NMEA: embedded GNSS receiver with NMEA output
//
// @WWAN_PORT_MAX: Highest supported port types
// @WWAN_PORT_UNKNOWN: Special value to indicate an unknown port type
// @__WWAN_PORT_MAX: Internal use
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wwan_port_type {
    WWAN_PORT_AT,
    WWAN_PORT_MBIM,
    WWAN_PORT_QMI,
    WWAN_PORT_QCDM,
    WWAN_PORT_FIREHOSE,
    WWAN_PORT_XMMRPC,
    WWAN_PORT_FASTBOOT,
    WWAN_PORT_ADB,
    WWAN_PORT_MIPC,
    WWAN_PORT_NMEA,

// Add new port types above this line

    __WWAN_PORT_MAX,
    WWAN_PORT_MAX = __WWAN_PORT_MAX - 1,
    WWAN_PORT_UNKNOWN,
}

// struct wwan_port_ops - The WWAN port operations
// @start: The routine for starting the WWAN port device.
// @stop: The routine for stopping the WWAN port device.
// @tx: Non-blocking routine that sends WWAN port protocol data to the device.
// @tx_blocking: Optional blocking routine that sends WWAN port protocol data
// to the device.
// @tx_poll: Optional routine that sets additional TX poll flags.
//
// The wwan_port_ops structure contains a list of low-level operations
// that control a WWAN port device. All functions are mandatory unless specified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_port_ops {
    pub port): *mut *mut int (start)(struct wwan_port,
    pub port): *mut *mut void (stop)(struct wwan_port,
    pub skb): *mut *mut *mut int (tx)(struct wwan_port port, struct sk_buff,
// Optional operations
    pub skb): *mut *mut *mut int (tx_blocking)(struct wwan_port port, struct sk_buff,
    pub wait): *mut poll_table,
}

// struct wwan_port_caps - The WWAN port capbilities
// @frag_len: WWAN port TX fragments length
// @headroom_len: WWAN port TX fragments reserved headroom length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_port_caps {
    pub frag_len: usize,
    pub headroom_len: c_uint,
}

//
// wwan_create_port - Add a new WWAN port
// @parent: Device to use as parent and shared by all WWAN ports
// @type: WWAN port type
// @ops: WWAN port operations
// @caps: WWAN port capabilities
// @drvdata: Pointer to caller driver data
//
// Allocate and register a new WWAN port. The port will be automatically exposed
// to user as a character device and attached to the right virtual WWAN device,
// based on the parent pointer. The parent pointer is the device shared by all
// components of a same WWAN modem (e.g. USB dev, PCI dev, MHI controller...).
//
// drvdata will be placed in the WWAN port device driver data and can be
// retrieved with wwan_port_get_drvdata().
//
// This function must be balanced with a call to wwan_remove_port().
//
// Returns: a valid pointer to wwan_port on success or PTR_ERR on failure
//
// wwan_remove_port - Remove a WWAN port
// @port: WWAN port to remove
//
// Remove a previously created port.
//
extern "C" {
    pub fn wwan_remove_port(port: *mut wwan_port);
}
//
// wwan_port_rx - Receive data from the WWAN port
// @port: WWAN port for which data is received
// @skb: Pointer to the rx buffer
//
// A port driver calls this function upon data reception (MBIM, AT...).
//
extern "C" {
    pub fn wwan_port_rx(port: *mut wwan_port, skb: *mut sk_buff);
}
//
// wwan_port_txoff - Stop TX on WWAN port
// @port: WWAN port for which TX must be stopped
//
// Used for TX flow control, a port driver calls this function to indicate TX
// is temporary unavailable (e.g. due to ring buffer fullness).
//
extern "C" {
    pub fn wwan_port_txoff(port: *mut wwan_port);
}
//
// wwan_port_txon - Restart TX on WWAN port
// @port: WWAN port for which TX must be restarted
//
// Used for TX flow control, a port driver calls this function to indicate TX
// is available again.
//
extern "C" {
    pub fn wwan_port_txon(port: *mut wwan_port);
}
//
// wwan_port_get_drvdata - Retrieve driver data from a WWAN port
// @port: Related WWAN port
//
// struct wwan_netdev_priv - WWAN core network device private data
// @link_id: WWAN device data link id
// @drv_priv: driver private data area, size is determined in &wwan_ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_netdev_priv {
    pub link_id: u32,
// must be last
    pub )): *mut u8 drv_priv[] __aligned(sizeof(void,
}

//
// Used to indicate that the WWAN core should not create a default network
// link.
//

//
// struct wwan_ops - WWAN device ops
// @priv_size: size of private netdev data area
// @setup: set up a new netdev
// @newlink: register the new netdev
// @dellink: remove the given netdev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwan_ops {
    pub priv_size: c_uint,
    pub dev): *mut *mut void (setup)(struct net_device,
    pub extack): *mut u32 if_id, struct netlink_ext_ack,
    pub head): *mut list_head,
}

extern "C" {
    pub fn wwan_unregister_ops(parent: *mut device);
}

extern "C" {
    pub fn wwan_put_debugfs_dir(dir: *mut dentry);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

