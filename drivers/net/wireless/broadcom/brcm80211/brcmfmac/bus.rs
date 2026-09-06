//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/bus.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

// IDs of the 6 default common rings of msgbuf protocol
pub const BRCMF_H2D_MSGRING_CONTROL_SUBMIT: c_int = 0;
pub const BRCMF_H2D_MSGRING_RXPOST_SUBMIT: c_int = 1;
pub const BRCMF_H2D_MSGRING_FLOWRING_IDSTART: c_int = 2;
pub const BRCMF_D2H_MSGRING_CONTROL_COMPLETE: c_int = 2;
pub const BRCMF_D2H_MSGRING_TX_COMPLETE: c_int = 3;
pub const BRCMF_D2H_MSGRING_RX_COMPLETE: c_int = 4;
pub const BRCMF_NROF_H2D_COMMON_MSGRINGS: c_int = 2;
pub const BRCMF_NROF_D2H_COMMON_MSGRINGS: c_int = 3;

// The interval to poll console
pub const BRCMF_CONSOLE: c_int = 10;
// The maximum console interval value (5 mins)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_fwvendor {
    BRCMF_FWVENDOR_WCC,
    BRCMF_FWVENDOR_CYW,
    BRCMF_FWVENDOR_BCA,
// keep last
    BRCMF_FWVENDOR_NUM,
    BRCMF_FWVENDOR_INVALID
}

// The level of bus communication with the dongle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_bus_state {
    BRCMF_BUS_DOWN,		/* Not ready for frame transfers */
    BRCMF_BUS_UP		/* Ready for frame transfers */
}

// The level of bus communication with the dongle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_bus_protocol_type {
    BRCMF_PROTO_BCDC,
    BRCMF_PROTO_MSGBUF
}

// Firmware blobs that may be available
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_blob_type {
    BRCMF_BLOB_CLM,
    BRCMF_BLOB_TXCAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bus_dcmd {
    pub name: *mut c_char,
    pub param: *mut c_char,
    pub param_len: c_int,
    pub list: list_head,
}

//
// struct brcmf_bus_ops - bus callback operations.
//
// @preinit: execute bus/device specific dongle init commands (optional).
// @init: prepare for communication with dongle.
// @stop: clear pending frames, disable data flow.
// @txdata: send a data frame to the dongle. When the data
// has been transferred, the common driver must be
// notified using brcmf_txcomplete(). The common
// driver calls this function with interrupts
// disabled.
// @txctl: transmit a control request message to dongle.
// @rxctl: receive a control response message from dongle.
// @gettxq: obtain a reference of bus transmit queue (optional).
// @wowl_config: specify if dongle is configured for wowl when going to suspend
// @get_ramsize: obtain size of device memory.
// @get_memdump: obtain device memory dump in provided buffer.
// @get_blob: obtain a firmware blob.
// @remove: initiate unbind of the device.
//
// This structure provides an abstract interface towards the
// bus specific driver. For control messages to common driver
// will assure there is only one active transaction. Unless
// indicated otherwise these callbacks are mandatory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bus_ops {
    pub dev): *mut *mut int (preinit)(struct device,
    pub dev): *mut *mut void (stop)(struct device,
    pub skb): *mut *mut *mut int (txdata)(struct device dev, struct sk_buff,
    pub len): *mut *mut *mut *mut int (txctl)(struct device dev, unsigned char msg, uint,
    pub len): *mut *mut *mut *mut int (rxctl)(struct device dev, unsigned char msg, uint,
    pub dev): *mut *mut *mut pktq  (gettxq)(device,
    pub enabled): *mut *mut *mut void (wowl_config)(struct device dev, bool,
    pub dev): *mut *mut size_t (get_ramsize)(struct device,
    pub len): *mut *mut *mut *mut int (get_memdump)(struct device dev, void data, size_t,
    pub type): brcmf_blob_type,
    pub dev): *mut *mut void (debugfs_create)(struct device,
    pub dev): *mut *mut int (reset)(struct device,
    pub dev): *mut *mut void (remove)(struct device,
}

//
// struct brcmf_bus_msgbuf - bus ringbuf if in case of msgbuf.
//
// @commonrings: commonrings which are always there.
// @flowrings: commonrings which are dynamically created and destroyed for data.
// @rx_dataoffset: if set then all rx data has this offset.
// @max_rxbufpost: maximum number of buffers to post for rx.
// @max_flowrings: maximum number of tx flow rings supported.
// @max_submissionrings: maximum number of submission rings(h2d) supported.
// @max_completionrings: maximum number of completion rings(d2h) supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bus_msgbuf {
    pub commonrings: [*mut brcmf_commonring; BRCMF_NROF_COMMON_MSGRINGS],
    pub flowrings: *mut brcmf_commonring,
    pub rx_dataoffset: u32,
    pub max_rxbufpost: u32,
    pub max_flowrings: u16,
    pub max_submissionrings: u16,
    pub max_completionrings: u16,
}

//
// struct brcmf_bus_stats - bus statistic counters.
//
// @pktcowed: packets cowed for extra headroom/unorphan.
// @pktcow_failed: packets dropped due to failed cow-ing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bus_stats {
    pub pktcowed: core::sync::atomic::AtomicI32,
    pub pktcow_failed: core::sync::atomic::AtomicI32,
}

//
// struct brcmf_bus - interface structure between common and bus layer
//
// @bus_priv: pointer to private bus device.
// @proto_type: protocol type, bcdc or msgbuf
// @dev: device pointer of bus device.
// @drvr: public driver information.
// @state: operational state of the bus interface.
// @stats: statistics shared between common and bus layer.
// @maxctl: maximum size for rxctl request message.
// @chip: device identifier of the dongle chip.
// @chiprev: revision of the dongle chip.
// @fwvid: firmware vendor-support identifier of the device.
// @always_use_fws_queue: bus wants use queue also when fwsignal is inactive.
// @wowl_supported: is wowl supported by bus driver.
// @ops: callbacks for this bus instance.
// @msgbuf: msgbuf protocol parameters provided by bus layer.
// @list: member used to add this bus instance to linked list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_bus {
    pub sdio: *mut brcmf_sdio_dev,
    pub usb: *mut brcmf_usbdev,
    pub pcie: *mut brcmf_pciedev,
    pub bus_priv: },
    pub proto_type: brcmf_bus_protocol_type,
    pub dev: *mut device,
    pub drvr: *mut brcmf_pub,
    pub state: brcmf_bus_state,
    pub stats: brcmf_bus_stats,
    pub maxctl: c_uint,
    pub chip: u32,
    pub chiprev: u32,
    pub fwvid: brcmf_fwvendor,
    pub always_use_fws_queue: bool,
    pub wowl_supported: bool,
    pub /: *mut *mut bool removing; / device removal in progress; quiesce async work,
    pub bus_reset_lock: mutex,
    pub ops: *const brcmf_bus_ops,
    pub msgbuf: *mut brcmf_bus_msgbuf,
    pub list: list_head,
}

extern "C" {
    pub fn brcmf_bus_cancel_reset_work(bus_if: *mut brcmf_bus);
}
extern "C" {
    pub fn brcmf_bus_allow_reset_work(bus_if: *mut brcmf_bus);
}
//
// callback wrappers
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
//
// interface functions from common layer
//
// Receive frame for delivery to OS.  Callee disposes of rxp.
// Receive async event packet from firmware. Callee disposes of rxp.
extern "C" {
    pub fn brcmf_rx_event(dev: *mut device, rxp: *mut sk_buff);
}
extern "C" {
    pub fn brcmf_alloc(dev: *mut device, settings: *mut brcmf_mp_device) -> c_int;
}
// Indication from bus module regarding presence/insertion of dongle.
extern "C" {
    pub fn brcmf_attach(dev: *mut device) -> c_int;
}
// Indication from bus module regarding removal/absence of dongle
extern "C" {
    pub fn brcmf_detach(dev: *mut device);
}
extern "C" {
    pub fn brcmf_free(dev: *mut device);
}
// Indication from bus module that dongle should be reset
extern "C" {
    pub fn brcmf_dev_reset(dev: *mut device);
}
// Request from bus module to initiate a coredump
extern "C" {
    pub fn brcmf_dev_coredump(dev: *mut device);
}
// Indication that firmware has halted or crashed
extern "C" {
    pub fn brcmf_fw_crashed(dev: *mut device);
}
// Configure the "global" bus state used by upper layers
extern "C" {
    pub fn brcmf_bus_change_state(bus: *mut brcmf_bus, state: brcmf_bus_state);
}
extern "C" {
    pub fn brcmf_iovar_data_set(dev: *mut device, name: *mut c_char, data: *mut c_void, len: u32) -> i32;
}
extern "C" {
    pub fn brcmf_bus_add_txhdrlen(dev: *mut device, len: c_uint);
}

extern "C" {
    pub fn brcmf_sdio_exit();
}
extern "C" {
    pub fn brcmf_sdio_register() -> c_int;
}

extern "C" {
    pub fn brcmf_usb_exit();
}
extern "C" {
    pub fn brcmf_usb_register() -> c_int;
}

extern "C" {
    pub fn brcmf_pcie_exit();
}
extern "C" {
    pub fn brcmf_pcie_register() -> c_int;
}

