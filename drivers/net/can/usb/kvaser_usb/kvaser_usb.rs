//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/kvaser_usb/kvaser_usb.h
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
// Parts of this driver are based on the following:
// - Kvaser linux leaf driver (version 4.78)
// - CAN driver for esd CAN-USB/2
// - Kvaser linux usbcanII driver (version 5.3)
// - Kvaser linux mhydra driver (version 5.24)
//
// Copyright (C) 2002-2018 KVASER AB, Sweden. All rights reserved.
// Copyright (C) 2010 Matthias Fuchs <matthias.fuchs@esd.eu>, esd gmbh
// Copyright (C) 2012 Olivier Sobrie <olivier@sobrie.be>
// Copyright (C) 2015 Valeo S.A.
//
// Kvaser USB CAN dongles are divided into three major platforms:
// - Hydra: Running firmware labeled as 'mhydra'
// - Leaf: Based on Renesas M32C or Freescale i.MX28, running firmware labeled
// as 'filo'
// - UsbcanII: Based on Renesas M16C, running firmware labeled as 'helios'
//

pub const KVASER_USB_MAX_RX_URBS: c_int = 4;
pub const KVASER_USB_MAX_TX_URBS: c_int = 128;

pub const KVASER_USB_RX_BUFFER_SIZE: c_int = 3072;
pub const KVASER_USB_MAX_NET_DEVICES: c_int = 5;
// Kvaser USB device quirks

// Device capabilities
pub const KVASER_USB_CAP_BERR_CAP: c_uint = 0x01;
pub const KVASER_USB_CAP_EXT_CAP: c_uint = 0x02;
pub const KVASER_USB_HYDRA_CAP_EXT_CMD: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvaser_usb_leaf_family {
    KVASER_LEAF,
    KVASER_USBCAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvaser_usb_led_state {
    KVASER_USB_LED_ON = 0,
    KVASER_USB_LED_OFF = 1,
}

pub const KVASER_USB_HYDRA_MAX_CMD_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_dev_card_data_hydra {
    pub channel_to_he: [u8; KVASER_USB_MAX_NET_DEVICES],
    pub sysdbg_he: u8,
    pub /: *mut *mut spinlock_t transid_lock; / lock for transid,
    pub transid: u16,
// lock for usb_rx_leftover and usb_rx_leftover_len
    pub usb_rx_leftover_lock: spinlock_t,
    pub usb_rx_leftover: [u8; KVASER_USB_HYDRA_MAX_CMD_LEN],
    pub usb_rx_leftover_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_dev_card_data {
    pub ctrlmode_supported: u32,
    pub capabilities: u32,
    pub hydra: kvaser_usb_dev_card_data_hydra,
    pub usbcan_timestamp_msb: u32,
}

// Context for an outstanding, not yet ACKed, transmission
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_tx_urb_context {
    pub priv: *mut kvaser_usb_net_priv,
    pub echo_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_fw_version {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_busparams {
    pub bitrate: __le32,
    pub tseg1: u8,
    pub tseg2: u8,
    pub sjw: u8,
    pub nsamples: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub nets: [*mut kvaser_usb_net_priv; KVASER_USB_MAX_NET_DEVICES],
    pub driver_info: *const kvaser_usb_driver_info,
    pub cfg: *const kvaser_usb_dev_cfg,
    pub bulk_out: *mut *mut usb_endpoint_descriptor bulk_in,,
    pub rx_submitted: usb_anchor,
    pub ean: [u32; 2],
    pub serial_number: u32,
    pub fw_version: kvaser_usb_fw_version,
    pub hw_revision: u8,
    pub nchannels: c_uint,
// @max_tx_urbs: Firmware-reported maximum number of outstanding,
// not yet ACKed, transmissions on this device. This value is
// also used as a sentinel for marking free tx contexts.
//
    pub max_tx_urbs: c_uint,
    pub card_data: kvaser_usb_dev_card_data,
    pub rxinitdone: bool,
    pub rxbuf: [*mut c_void; KVASER_USB_MAX_RX_URBS],
    pub rxbuf_dma: [dma_addr_t; KVASER_USB_MAX_RX_URBS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_net_priv {
    pub can: can_priv,
    pub devlink_port: devlink_port,
    pub bec: can_berr_counter,
// subdriver-specific data
    pub sub_priv: *mut c_void,
    pub dev: *mut kvaser_usb,
    pub netdev: *mut net_device,
    pub channel: c_int,
    pub tx_submitted: usb_anchor,
    pub busparams_data: kvaser_usb_busparams busparams_nominal,,
    pub /: *mut *mut spinlock_t tx_contexts_lock; / lock for active_tx_contexts,
    pub active_tx_contexts: c_int,
    pub tx_contexts: [kvaser_usb_tx_urb_context; ],
}

//
// struct kvaser_usb_dev_ops - Device specific functions
// @dev_set_mode:		used for can.do_set_mode
// @dev_set_bittiming:		used for can.do_set_bittiming
// @dev_get_busparams:		readback arbitration busparams
// @dev_set_data_bittiming:	used for can.fd.do_set_data_bittiming
// @dev_get_data_busparams:	readback data busparams
// @dev_get_berr_counter:	used for can.do_get_berr_counter
//
// @dev_setup_endpoints:	setup USB in and out endpoints
// @dev_init_card:		initialize card
// @dev_init_channel:		initialize channel
// @dev_remove_channel:		uninitialize channel
// @dev_get_software_info:	get software info
// @dev_get_software_details:	get software details
// @dev_get_card_info:		get card info
// @dev_get_capabilities:	discover device capabilities
// @dev_set_led:		turn on/off device LED
//
// @dev_set_opt_mode:		set ctrlmod
// @dev_start_chip:		start the CAN controller
// @dev_stop_chip:		stop the CAN controller
// @dev_reset_chip:		reset the CAN controller
// @dev_flush_queue:		flush outstanding CAN messages
// @dev_read_bulk_callback:	handle incoming commands
// @dev_frame_to_cmd:		translate struct can_frame into device command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_dev_ops {
    pub mode): *mut *mut *mut int (dev_set_mode)(struct net_device netdev, enum can_mode,
    pub busparams): *const kvaser_usb_busparams,
    pub priv): *mut *mut int (dev_get_busparams)(struct kvaser_usb_net_priv,
    pub busparams): *const kvaser_usb_busparams,
    pub priv): *mut *mut int (dev_get_data_busparams)(struct kvaser_usb_net_priv,
    pub bec): *mut can_berr_counter,
    pub dev): *mut *mut int (dev_setup_endpoints)(struct kvaser_usb,
    pub dev): *mut *mut int (dev_init_card)(struct kvaser_usb,
    pub priv): *mut *mut int (dev_init_channel)(struct kvaser_usb_net_priv,
    pub priv): *mut *mut void (dev_remove_channel)(struct kvaser_usb_net_priv,
    pub dev): *mut *mut int (dev_get_software_info)(struct kvaser_usb,
    pub dev): *mut *mut int (dev_get_software_details)(struct kvaser_usb,
    pub dev): *mut *mut int (dev_get_card_info)(struct kvaser_usb,
    pub dev): *mut *mut int (dev_get_capabilities)(struct kvaser_usb,
    pub duration_ms): u16,
    pub priv): *const *const int (dev_set_opt_mode)(struct kvaser_usb_net_priv,
    pub priv): *mut *mut int (dev_start_chip)(struct kvaser_usb_net_priv,
    pub priv): *mut *mut int (dev_stop_chip)(struct kvaser_usb_net_priv,
    pub channel): *mut *mut *mut int (dev_reset_chip)(struct kvaser_usb dev, int,
    pub priv): *mut *mut int (dev_flush_queue)(struct kvaser_usb_net_priv,
    pub len): c_int,
    pub transid): u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_driver_info {
    pub quirks: u32,
    pub family: kvaser_usb_leaf_family,
    pub ops: *const kvaser_usb_dev_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_usb_dev_cfg {
    pub clock: can_clock,
    pub timestamp_freq: c_uint,
    pub bittiming_const: *const *const can_bittiming_,
    pub data_bittiming_const: *const *const can_bittiming_,
}

extern "C" {
    pub fn kvaser_usb_devlink_port_register(priv: *mut kvaser_usb_net_priv) -> c_int;
}
extern "C" {
    pub fn kvaser_usb_devlink_port_unregister(priv: *mut kvaser_usb_net_priv);
}
extern "C" {
    pub fn kvaser_usb_unlink_tx_urbs(priv: *mut kvaser_usb_net_priv);
}
extern "C" {
    pub fn kvaser_usb_send_cmd(dev: *const kvaser_usb, cmd: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn kvaser_usb_can_rx_over_error(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ns_to_ktime(1000: *mut *mut div_u64(ticks, _arg: cfg->timestamp_freq)) -> return;
}
extern "C" {
    pub fn kvaser_usb_ticks_to_ktime(_arg: cfg, _arg: ticks) -> return;
}
extern "C" {
    pub fn kvaser_usb_ticks_to_ktime(_arg: cfg, _arg: le64_to_cpu(timestamp)) -> return;
}
