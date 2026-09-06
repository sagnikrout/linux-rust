//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/net2280.h
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
// NetChip 2280 high/full speed USB device controller.
// Unlike many such controllers, this one talks PCI.
//
// Copyright (C) 2002 NetChip Technology, Inc. (http://www.netchip.com)
// Copyright (C) 2003 David Brownell
// Copyright (C) 2014 Ricardo Ribalda - Qtechnology/AS
//

// -------------------------------------------------------------------------

// indexed registers [11.10] are accessed indirectly
// caller must own the device lock.
//
// NOTE:  synchs device/cpu memory views
extern "C" {
    pub fn readl(_arg: &regs->idxdata) -> return;
}
// posted, may not be visible yet

pub const PCI_VENDOR_ID_PLX_LEGACY: c_uint = 0x17cc;

pub const REG_DIAG: c_uint = 0x0;
pub const RETRY_COUNTER: c_int = 16;
pub const FORCE_PCI_SERR: c_int = 11;
pub const FORCE_PCI_INTERRUPT: c_int = 10;
pub const FORCE_USB_INTERRUPT: c_int = 9;
pub const FORCE_CPU_INTERRUPT: c_int = 8;
pub const ILLEGAL_BYTE_ENABLES: c_int = 5;
pub const FAST_TIMES: c_int = 4;
pub const FORCE_RECEIVE_ERROR: c_int = 2;
pub const FORCE_TRANSMIT_CRC_ERROR: c_int = 0;
pub const REG_FRAME: c_uint = 0x02	/* from last sof */;
pub const REG_CHIPREV: c_uint = 0x03	/* in bcd */;
pub const REG_HS_NAK_RATE: c_uint = 0x0a	/* NAK per N uframes */;
pub const CHIPREV_1: c_uint = 0x0100;
pub const CHIPREV_1A: c_uint = 0x0110;
// DEFECT 7374
pub const DEFECT_7374_NUMBEROF_MAX_WAIT_LOOPS: c_int = 200;
pub const DEFECT_7374_PROCESSOR_WAIT_TIME: c_int = 10;
// ep0 max packet size
pub const EP0_SS_MAX_PACKET_SIZE: c_uint = 0x200;
pub const EP0_HS_MAX_PACKET_SIZE: c_uint = 0x40;

// -------------------------------------------------------------------------
// [8.3] for scatter/gather i/o
// use struct net2280_dma_regs bitfields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_dma {
    pub dmacount: __le32,
    pub /: *mut *mut __le32 dmaaddr; / the buffer,
    pub /: *mut *mut __le32 dmadesc; / next dma descriptor,
    pub _reserved: __le32,
    pub __aligned(16): },
// -------------------------------------------------------------------------
// DRIVER DATA STRUCTURES and UTILITIES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_ep {
    pub ep: usb_ep,
    pub cfg: *mut net2280_ep_regs __iomem,
    pub regs: *mut net2280_ep_regs __iomem,
    pub dma: *mut net2280_dma_regs __iomem,
    pub dummy: *mut net2280_dma,
    pub /: *mut *mut dma_addr_t td_dma; / of dummy,
    pub dev: *mut net2280,
    pub irqs: c_ulong,
// analogous to a host-side qh
    pub queue: list_head,
    pub desc: *const usb_endpoint_descriptor,
    pub 1: responded :,
}

// ep0 only
//
// Control Status Phase Handshake was set by the chip when the setup
// packet arrived. While set, the chip automatically NAKs the host's
// Status Phase tokens.
//
// TD 9.9 Halt Endpoint test.  TD 9.22 set feature test.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_request {
    pub req: usb_request,
    pub td: *mut net2280_dma,
    pub td_dma: dma_addr_t,
    pub queue: list_head,
    pub 1: valid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280 {
// each pci device provides one gadget, several endpoints
    pub gadget: usb_gadget,
    pub lock: spinlock_t,
    pub ep: [net2280_ep; 9],
    pub driver: *mut usb_gadget_driver,
    pub chiprev: u16,
    pub enhanced_mode: c_int,
    pub n_ep: c_int,
    pub quirks: kernel_ulong_t,
// pci state used to access those endpoints
    pub pdev: *mut pci_dev,
    pub regs: *mut net2280_regs __iomem,
    pub usb: *mut net2280_usb_regs __iomem,
    pub usb_ext: *mut usb338x_usb_ext_regs __iomem,
    pub pci: *mut net2280_pci_regs __iomem,
    pub dma: *mut net2280_dma_regs __iomem,
    pub dep: *mut net2280_dep_regs __iomem,
    pub epregs: *mut net2280_ep_regs __iomem,
    pub llregs: *mut usb338x_ll_regs __iomem,
    pub plregs: *mut usb338x_pl_regs __iomem,
    pub requests: *mut dma_pool,
// statistics...
}

// ep0 and bulk/intr endpoints
// set NAK_OUT for erratum 0114
// ep0 and bulk/intr endpoints
//
// unless the gadget driver left a short packet in the
// fifo, this reverses the erratum 0114 workaround.
//
// FSM value for Defect 7374 (U1U2 Test) is managed in
// chip's SCRATCH register:
//
pub const DEFECT7374_FSM_FIELD: c_int = 28;
// Waiting for Control Read:
// - A transition to this state indicates a fresh USB connection,
// before the first Setup Packet. The connection speed is not
// known. Firmware is waiting for the first Control Read.
// - Starting state: This state can be thought of as the FSM's typical
// starting state.
// - Tip: Upon the first SS Control Read the FSM never
// returns to this state.
//

// Non-SS Control Read:
// - A transition to this state indicates detection of the first HS
// or FS Control Read.
// - Tip: Upon the first SS Control Read the FSM never
// returns to this state.
//

// SS Control Read:
// - A transition to this state indicates detection of the
// first SS Control Read.
// - This state indicates workaround completion. Workarounds no longer
// need to be applied (as long as the chip remains powered up).
// - Tip: Once in this state the FSM state does not change (until
// the chip's power is lost and restored).
// - This can be thought of as the final state of the FSM;
// the FSM 'locks-up' in this state until the chip loses power.
//

// LED3 (green) is on during USB activity. note erratum 0113.
// indicate speed with bi-color LED 0/1
// indicate power with LED 2
// FIXME this LED never seems to turn on.
// turn off all four GPIO*_DATA bits

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
// NOTE:  hardware races lurk here, and PING protocol issues
// synch with device
