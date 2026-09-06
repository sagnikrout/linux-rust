//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/gr_udc.h
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
// USB Peripheral Controller driver for Aeroflex Gaisler GRUSBDC.
//
// 2013 (c) Aeroflex Gaisler AB
//
// This driver supports GRUSBDC USB Device Controller cores available in the
// GRLIB VHDL IP core library.
//
// Full documentation of the GRUSBDC core can be found here:
// https://www.gaisler.com/products/grlib/grip.pdf
//
// Contributors:
// - Andreas Larsson <andreas@gaisler.com>
// - Marko Isomaki
//
// Control registers on the AMBA bus

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_epregs {
    pub epctrl: u32,
    pub slvctrl: u32,
    pub slvdata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_regs {
    pub /: *mut *mut gr_epregs epo[GR_MAXEP]; / 0x000 - 0x0fc,
    pub /: *mut *mut gr_epregs epi[GR_MAXEP]; / 0x100 - 0x1fc,
    pub /: *mut *mut u32 control; / 0x200,
    pub /: *mut *mut u32 status; / 0x204,
}

pub const GR_EPCTRL_BUFSZ_SCALER: c_int = 8;
pub const GR_EPCTRL_BUFSZ_MASK: c_uint = 0xffe00000;
pub const GR_EPCTRL_BUFSZ_POS: c_int = 21;

pub const GR_EPCTRL_MAXPL_MASK: c_uint = 0x0003ff80;
pub const GR_EPCTRL_MAXPL_POS: c_int = 7;
pub const GR_EPCTRL_NT_MASK: c_uint = 0x00000060;
pub const GR_EPCTRL_NT_POS: c_int = 5;
pub const GR_EPCTRL_TT_MASK: c_uint = 0x00000018;
pub const GR_EPCTRL_TT_POS: c_int = 3;

pub const GR_EPSTAT_B1CNT_MASK: c_uint = 0x1fff0000;
pub const GR_EPSTAT_B1CNT_POS: c_int = 16;
pub const GR_EPSTAT_B0CNT_MASK: c_uint = 0x0000fff8;
pub const GR_EPSTAT_B0CNT_POS: c_int = 3;

pub const GR_CONTROL_TS_MASK: c_uint = 0x00000e00;
pub const GR_CONTROL_TS_POS: c_int = 9;

pub const GR_CONTROL_UA_MASK: c_uint = 0x000000fe;
pub const GR_CONTROL_UA_POS: c_int = 1;

pub const GR_STATUS_NEPI_MASK: c_uint = 0xf0000000;
pub const GR_STATUS_NEPI_POS: c_int = 28;
pub const GR_STATUS_NEPO_MASK: c_uint = 0x0f000000;
pub const GR_STATUS_NEPO_POS: c_int = 24;

pub const GR_STATUS_AF_MASK: c_uint = 0x00003800;
pub const GR_STATUS_AF_POS: c_int = 11;
pub const GR_STATUS_FN_MASK: c_uint = 0x000007ff;
pub const GR_STATUS_FN_POS: c_int = 0;

// -------------------------------------------------------------------------
// Driver data structures and utilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_dma_desc {
    pub ctrl: u32,
    pub data: u32,
    pub next: u32,
// These must be last because hw uses the previous three
    pub paddr: u32,
    pub next_desc: *mut gr_dma_desc,
}

pub const GR_DESC_OUT_CTRL_LEN_MASK: c_uint = 0x00001fff;

pub const GR_DESC_IN_CTRL_LEN_MASK: c_uint = 0x00001fff;
pub const GR_DESC_DMAADDR_MASK: c_uint = 0xfffffffc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_ep {
    pub ep: usb_ep,
    pub dev: *mut gr_udc,
    pub bytes_per_buffer: u16,
    pub dma_start: c_uint,
    pub regs: *mut gr_epregs __iomem,
    pub num:8: unsigned,
    pub is_in:1: unsigned,
    pub stopped:1: unsigned,
    pub wedged:1: unsigned,
    pub callback:1: unsigned,
// analogous to a host-side qh
    pub queue: list_head,
    pub ep_list: list_head,
// Bounce buffer for end of "odd" sized OUT requests
    pub tailbuf: *mut c_void,
    pub tailbuf_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_request {
    pub req: usb_request,
    pub queue: list_head,
// Chain of dma descriptors
    pub /: *mut *mut *mut gr_dma_desc first_desc; / First in the chain,
    pub /: *mut *mut *mut gr_dma_desc curr_desc; / Current descriptor,
    pub /: *mut *mut *mut gr_dma_desc last_desc; / Last in the chain,
    pub /: *mut *mut u16 evenlen; / Size of even length head (if oddlen != 0),
    pub /: *mut *mut u16 oddlen; / Size of odd length tail if buffer length is "odd",
    pub /: *mut *mut u8 setup; / Setup packet,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gr_ep0state {
    GR_EP0_DISCONNECT = 0,	/* No host */
    GR_EP0_SETUP,		/* Between STATUS ack and SETUP report */
    GR_EP0_IDATA,		/* IN data stage */
    GR_EP0_ODATA,		/* OUT data stage */
    GR_EP0_ISTATUS,		/* Status stage after IN data stage */
    GR_EP0_OSTATUS,		/* Status stage after OUT data stage */
    GR_EP0_STALL,		/* Data or status stages */
    GR_EP0_SUSPEND,		/* USB suspend */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gr_udc {
    pub gadget: usb_gadget,
    pub epi: [gr_ep; GR_MAXEP],
    pub epo: [gr_ep; GR_MAXEP],
    pub driver: *mut usb_gadget_driver,
    pub desc_pool: *mut dma_pool,
    pub dev: *mut device,
    pub ep0state: gr_ep0state,
    pub ep0reqo: *mut gr_request,
    pub ep0reqi: *mut gr_request,
    pub regs: *mut gr_regs __iomem,
    pub irq: c_int,
    pub irqi: c_int,
    pub irqo: c_int,
    pub added:1: unsigned,
    pub irq_enabled:1: unsigned,
    pub remote_wakeup:1: unsigned,
    pub test_mode: u8,
    pub suspended_from: usb_device_state,
    pub nepi: c_uint,
    pub nepo: c_uint,
    pub ep_list: list_head,
    pub /: *mut *mut spinlock_t lock; / General lock, a.k.a. "dev->lock" in comments,
}
