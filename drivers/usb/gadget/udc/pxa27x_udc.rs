//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/pxa27x_udc.h
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
// linux/drivers/usb/gadget/pxa27x_udc.h
// Intel PXA27x on-chip full speed USB device controller
//
// Inspired by original driver by Frank Becker, David Brownell, and others.
// Copyright (C) 2008 Robert Jarzmik
//

//
// Register definitions
//
// Offsets
pub const UDCCR: c_uint = 0x0000		/* UDC Control Register */;
pub const UDCICR0: c_uint = 0x0004		/* UDC Interrupt Control Register0 */;
pub const UDCICR1: c_uint = 0x0008		/* UDC Interrupt Control Register1 */;
pub const UDCISR0: c_uint = 0x000C		/* UDC Interrupt Status Register 0 */;
pub const UDCISR1: c_uint = 0x0010		/* UDC Interrupt Status Register 1 */;
pub const UDCFNR: c_uint = 0x0014		/* UDC Frame Number Register */;
pub const UDCOTGICR: c_uint = 0x0018		/* UDC On-The-Go interrupt control */;
pub const UP2OCR: c_uint = 0x0020		/* USB Port 2 Output Control register */;
pub const UP3OCR: c_uint = 0x0024		/* USB Port 3 Output Control register */;

pub const UDCCR_ACN_S: c_int = 11;

pub const UDCCR_AIN_S: c_int = 8;

pub const UDCCR_AAISN_S: c_int = 5;

// Host Port 2 field bits

// Transceiver enablers

pub const UDCCONR_CN_S: c_int = 25;

pub const UDCCONR_IN_S: c_int = 22;

pub const UDCCONR_AISN_S: c_int = 19;

pub const UDCCONR_EN_S: c_int = 15;

pub const UDCCONR_ET_S: c_int = 13;

pub const UDCCONR_MPS_S: c_int = 2;

//
// UDCCR = UDC Endpoint Configuration Registers
// UDCCSR = UDC Control/Status Register for this EP
// UDCBCR = UDC Byte Count Remaining (contents of OUT fifo)
// UDCDR = UDC Endpoint Data Register (the fifo)
//

// Register access macros

pub const UDCCISR1_EP_MASK: c_uint = 0xffff;

//
// Endpoint definitions
//
// Once enabled, pxa endpoint configuration is freezed, and cannot change
// unless a reset happens or the udc is disabled.
// Therefore, we must define all pxa potential endpoint definitions needed for
// all gadget and set them up before the udc is enabled.
//
// As the architecture chosen is fully static, meaning the pxa endpoint
// configurations are set up once and for all, we must provide a way to match
// one usb endpoint (usb_ep) to several pxa endpoints. The reason is that gadget
// layer autoconf doesn't choose the usb_ep endpoint on (config, interface, alt)
// criteria, while the pxa architecture requires that.
//
// The solution is to define several pxa endpoints matching one usb_ep. Ex:
// - "ep1-in" matches pxa endpoint EPA (which is an IN ep at addr 1, when
// the udc talks on (config=3, interface=0, alt=0)
// - "ep1-in" matches pxa endpoint EPB (which is an IN ep at addr 1, when
// the udc talks on (config=3, interface=0, alt=1)
// - "ep1-in" matches pxa endpoint EPC (which is an IN ep at addr 1, when
// the udc talks on (config=2, interface=0, alt=0)
//
// We'll define the pxa endpoint by its index (EPA => idx=1, EPB => idx=2, ...)
//
// Endpoint definition helpers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats {
    pub in_ops: c_ulong,
    pub out_ops: c_ulong,
    pub in_bytes: c_ulong,
    pub out_bytes: c_ulong,
    pub irqs: c_ulong,
}

//
// struct udc_usb_ep - container of each usb_ep structure
// @usb_ep: usb endpoint
// @desc: usb descriptor, especially type and address
// @dev: udc managing this endpoint
// @pxa_ep: matching pxa_ep (cache of find_pxa_ep() call)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_usb_ep {
    pub usb_ep: usb_ep,
    pub desc: usb_endpoint_descriptor,
    pub dev: *mut pxa_udc,
    pub pxa_ep: *mut pxa_ep,
}

//
// struct pxa_ep - pxa endpoint
// @dev: udc device
// @queue: requests queue
// @lock: lock to pxa_ep data (queues and stats)
// @enabled: true when endpoint enabled (not stopped by gadget layer)
// @in_handle_ep: number of recursions of handle_ep() function
// Prevents deadlocks or infinite recursions of types :
// irq->handle_ep()->req_done()->req.complete()->pxa_ep_queue()->handle_ep()
// or
// pxa_ep_queue()->handle_ep()->req_done()->req.complete()->pxa_ep_queue()
// @idx: endpoint index (1 => epA, 2 => epB, ..., 24 => epX)
// @name: endpoint name (for trace/debug purpose)
// @dir_in: 1 if IN endpoint, 0 if OUT endpoint
// @addr: usb endpoint number
// @config: configuration in which this endpoint is active
// @interface: interface in which this endpoint is active
// @alternate: altsetting in which this endpoint is active
// @fifo_size: max packet size in the endpoint fifo
// @type: endpoint type (bulk, iso, int, ...)
// @udccsr_value: save register of UDCCSR0 for suspend/resume
// @udccr_value: save register of UDCCR for suspend/resume
// @stats: endpoint statistics
//
// The *PROBLEM* is that pxa's endpoint configuration scheme is both misdesigned
// (cares about config/interface/altsetting, thus placing needless limits on
// device capability) and full of implementation bugs forcing it to be set up
// for use more or less like a pxa255.
//
// As we define the pxa_ep statically, we must guess all needed pxa_ep for all
// gadget which may work with this udc driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_ep {
    pub dev: *mut pxa_udc,
    pub queue: list_head,
    pub /: *mut *mut spinlock_t lock; / Protects this structure,
// (queues, stats)
    pub enabled:1: unsigned,
    pub in_handle_ep:1: unsigned,
    pub idx:5: unsigned,
    pub name: *mut c_char,
//
// Specific pxa endpoint data, needed for hardware initialization
//
    pub dir_in:1: unsigned,
    pub addr:4: unsigned,
    pub config:2: unsigned,
    pub interface:3: unsigned,
    pub alternate:3: unsigned,
    pub fifo_size: unsigned,
    pub type: unsigned,

    pub udccsr_value: u32,
    pub udccr_value: u32,

    pub stats: stats,
}

//
// struct pxa27x_request - container of each usb_request structure
// @req: usb request
// @udc_usb_ep: usb endpoint the request was submitted on
// @in_use: sanity check if request already queued on an pxa_ep
// @queue: linked list of requests, linked on pxa_ep->queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa27x_request {
    pub req: usb_request,
    pub udc_usb_ep: *mut udc_usb_ep,
    pub in_use:1: unsigned,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep0_state {
    WAIT_FOR_SETUP,
    SETUP_STAGE,
    IN_DATA_STAGE,
    OUT_DATA_STAGE,
    IN_STATUS_STAGE,
    OUT_STATUS_STAGE,
    STALL,
    WAIT_ACK_SET_CONF_INTERF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udc_stats {
    pub irqs_reset: c_ulong,
    pub irqs_suspend: c_ulong,
    pub irqs_resume: c_ulong,
    pub irqs_reconfig: c_ulong,
}

//
// struct pxa_udc - udc structure
// @regs: mapped IO space
// @irq: udc irq
// @clk: udc clock
// @usb_gadget: udc gadget structure
// @driver: bound gadget (zero, g_ether, g_mass_storage, ...)
// @dev: device
// @gpiod: gpio descriptor of gpio for D+ pullup (or NULL if none)
// @transceiver: external transceiver to handle vbus sense and D+ pullup
// @ep0state: control endpoint state machine state
// @stats: statistics on udc usage
// @udc_usb_ep: array of usb endpoints offered by the gadget
// @pxa_ep: array of pxa available endpoints
// @enabled: UDC was enabled by a previous udc_enable()
// @pullup_on: if pullup resistor connected to D+ pin
// @pullup_resume: if pullup resistor should be connected to D+ pin on resume
// @config: UDC active configuration
// @last_interface: UDC interface of the last SET_INTERFACE host request
// @last_alternate: UDC altsetting of the last SET_INTERFACE host request
// @udccsr0: save of udccsr0 in case of suspend
// @debugfs_state: debugfs entry for "udcstate"
// @debugfs_queues: debugfs entry for "queues"
// @debugfs_eps: debugfs entry for "epstate"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_udc {
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub dev: *mut device,
    pub gpiod: *mut gpio_desc,
    pub transceiver: *mut usb_phy,
    pub ep0state: ep0_state,
    pub stats: udc_stats,
    pub udc_usb_ep: [udc_usb_ep; NR_USB_ENDPOINTS],
    pub pxa_ep: [pxa_ep; NR_PXA_ENDPOINTS],
    pub enabled:1: unsigned,
    pub pullup_on:1: unsigned,
    pub pullup_resume:1: unsigned,
    pub vbus_sensed:1: unsigned,
    pub config:2: unsigned,
    pub last_interface:3: unsigned,
    pub last_alternate:3: unsigned,

    pub udccsr0: unsigned,

}

extern "C" {
    pub fn container_of(_arg: gadget, pxa_udc: struct, _arg: gadget) -> return;
}
//
// Debugging/message support
//

