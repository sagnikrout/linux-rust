//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/aspeed-vhub/vhub.h
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
// VHUB register definitions
//
pub const AST_VHUB_CTRL: c_uint = 0x00	/* Root Function Control & Status Register */;
pub const AST_VHUB_CONF: c_uint = 0x04	/* Root Configuration Setting Register */;
pub const AST_VHUB_IER: c_uint = 0x08	/* Interrupt Ctrl Register */;
pub const AST_VHUB_ISR: c_uint = 0x0C	/* Interrupt Status Register */;
pub const AST_VHUB_EP_ACK_IER: c_uint = 0x10	/* Programmable Endpoint Pool ACK Interrupt Enable Register */;
pub const AST_VHUB_EP_NACK_IER: c_uint = 0x14	/* Programmable Endpoint Pool NACK Interrupt Enable Register  */;
pub const AST_VHUB_EP_ACK_ISR: c_uint = 0x18	/* Programmable Endpoint Pool ACK Interrupt Status Register  */;
pub const AST_VHUB_EP_NACK_ISR: c_uint = 0x1C	/* Programmable Endpoint Pool NACK Interrupt Status Register  */;
pub const AST_VHUB_SW_RESET: c_uint = 0x20	/* Device Controller Soft Reset Enable Register */;
pub const AST_VHUB_USBSTS: c_uint = 0x24	/* USB Status Register */;
pub const AST_VHUB_EP_TOGGLE: c_uint = 0x28	/* Programmable Endpoint Pool Data Toggle Value Set */;
pub const AST_VHUB_ISO_FAIL_ACC: c_uint = 0x2C	/* Isochronous Transaction Fail Accumulator */;
pub const AST_VHUB_EP0_CTRL: c_uint = 0x30	/* Endpoint 0 Contrl/Status Register */;
pub const AST_VHUB_EP0_DATA: c_uint = 0x34	/* Base Address of Endpoint 0 In/OUT Data Buffer Register */;
pub const AST_VHUB_EP1_CTRL: c_uint = 0x38	/* Endpoint 1 Contrl/Status Register */;
pub const AST_VHUB_EP1_STS_CHG: c_uint = 0x3C	/* Endpoint 1 Status Change Bitmap Data */;
pub const AST_VHUB_SETUP0: c_uint = 0x80	/* Root Device Setup Data Buffer0 */;
pub const AST_VHUB_SETUP1: c_uint = 0x84	/* Root Device Setup Data Buffer1 */;
// Main control reg

// IER & ISR
pub const VHUB_IRQ_DEV1_BIT: c_int = 9;

pub const VHUB_IRQ_ACK_ALL: c_uint = 0x1ff;
// Downstream device IRQ mask.

// SW reset reg

// EP ACK/NACK IRQ masks

// USB status reg

// EP toggle

// HUB EP0 control

// HUB EP1 control

//
// per-device register definitions
//
pub const AST_VHUB_DEV_EN_CTRL: c_uint = 0x00;
pub const AST_VHUB_DEV_ISR: c_uint = 0x04;
pub const AST_VHUB_DEV_EP0_CTRL: c_uint = 0x08;
pub const AST_VHUB_DEV_EP0_DATA: c_uint = 0x0c;
// Device enable control

// Interrupt status

// Control bits.
//
// Note: The driver relies on the bulk of those bits
// matching corresponding vHub EP0 control bits
//

//
// per-endpoint register definitions
//
pub const AST_VHUB_EP_CONFIG: c_uint = 0x00;
pub const AST_VHUB_EP_DMA_CTLSTAT: c_uint = 0x04;
pub const AST_VHUB_EP_DESC_BASE: c_uint = 0x08;
pub const AST_VHUB_EP_DESC_STATUS: c_uint = 0x0C;
// EP config reg

pub const EP_TYPE_OFF: c_int = 0;
pub const EP_TYPE_BULK: c_int = 1;
pub const EP_TYPE_INT: c_int = 2;
pub const EP_TYPE_ISO: c_int = 3;

// EP DMA control

pub const EP_DMA_PROC_RX_IDLE: c_int = 0;
pub const EP_DMA_PROC_TX_IDLE: c_int = 8;

// EP DMA status

//
// DMA descriptors definitions
//
// Desc W1 IN

//
// Data structures and misc definitions
//
// AST_VHUB_NUM_GEN_EPs and AST_VHUB_NUM_PORTS are kept to avoid breaking
// existing AST2400/AST2500 platforms. AST2600 and future vhub revisions
// should define number of downstream ports and endpoints in device tree.
//

// values are 256 and 32)
//
// DMA descriptor (generic EPs only, currently only used
// for IN endpoints
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_desc {
    pub w0: __le32,
    pub w1: __le32,
}

// A transfer request, either core-originated or internal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_req {
    pub req: usb_request,
    pub queue: list_head,
// Actual count written to descriptors (desc mode only)
    pub act_count: c_uint,
//
// Desc number of the final packet or -1. For non-desc
// mode (or ep0), any >= 0 value means "last packet"
//
    pub last_desc: c_int,
// Request active (pending DMAs)
    pub 1: bool active :,
// Internal request (don't call back core)
    pub 1: bool internal :,
}

// Current state of an EP0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep0_state {
    ep0_state_token,
    ep0_state_data,
    ep0_state_status,
    ep0_state_stall,
}

//
// An endpoint, either generic, ep0, actual gadget EP
// or internal use vhub EP0. vhub EP1 doesn't have an
// associated structure as it's mostly HW managed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_ep {
    pub ep: usb_ep,
// Request queue
    pub queue: list_head,
// EP index in the device, 0 means this is an EP0
    pub d_idx: c_uint,
// Dev pointer or NULL for vHub EP0
    pub dev: *mut ast_vhub_dev,
// vHub itself
    pub vhub: *mut ast_vhub,
//
// DMA buffer for EP0, fallback DMA buffer for misaligned
// OUT transfers for generic EPs
//
    pub buf: *mut c_void,
    pub buf_dma: dma_addr_t,
// The rest depends on the EP type
// EP0 (either device or vhub)
//
// EP0 registers are "similar" for
// vHub and devices but located in
// different places.
//
    pub ctlstat: *mut void __iomem,
    pub setup: *mut void __iomem,
// Current state & direction
    pub state: ep0_state,
    pub dir_in: bool,
// Internal use request
    pub req: ast_vhub_req,
    pub ep0: },
// Generic endpoint (aka EPn)
// Registers
    pub regs: *mut void __iomem,
// Index in global pool (zero-based)
    pub g_idx: c_uint,
// DMA Descriptors
    pub descs: *mut ast_vhub_desc,
    pub descs_dma: dma_addr_t,
    pub d_next: c_uint,
    pub d_last: c_uint,
    pub dma_conf: c_uint,
// Max chunk size for IN EPs
    pub chunk_max: c_uint,
// State flags
    pub 1: bool is_in :,
    pub 1: bool is_iso :,
    pub 1: bool stalled :,
    pub 1: bool wedged :,
    pub 1: bool enabled :,
    pub 1: bool desc_mode :,
    pub epn: },
}

// A device attached to a vHub port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_dev {
    pub vhub: *mut ast_vhub,
    pub regs: *mut void __iomem,
// Device index (zero-based) and name string
    pub index: c_uint,
    pub name: *const c_char,
// sysfs enclosure for the gadget gunk
    pub port_dev: *mut device,
// Link to gadget core
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub 1: bool registered :,
    pub 1: bool wakeup_en :,
    pub 1: bool enabled :,
// Endpoint structures
    pub ep0: ast_vhub_ep,
    pub epns: *mut ast_vhub_ep,
    pub max_epns: u32,
}

// Per vhub port stateinfo structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_port {
// Port status & status change registers
    pub status: u16,
    pub change: u16,
// Associated device slot
    pub dev: ast_vhub_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub_full_cdesc {
    pub cfg: usb_config_descriptor,
    pub intf: usb_interface_descriptor,
    pub ep: usb_endpoint_descriptor,
    pub __packed: },
// Global vhub structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vhub {
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub lock: spinlock_t,
    pub wake_work: work_struct,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
// EP0 DMA buffers allocated in one chunk
    pub ep0_bufs: *mut c_void,
    pub ep0_bufs_dma: dma_addr_t,
// EP0 of the vhub itself
    pub ep0: ast_vhub_ep,
// State of vhub ep1
    pub 1: bool ep1_stalled :,
// Per-port info
    pub ports: *mut ast_vhub_port,
    pub max_ports: u32,
    pub port_irq_mask: u32,
// Generic EP data structures
    pub epns: *mut ast_vhub_ep,
    pub max_epns: u32,
// Upstream bus is suspended ?
    pub 1: bool suspended :,
// Hub itself can signal remote wakeup
    pub 1: bool wakeup_en :,
// Force full speed only
    pub 1: bool force_usb1 :,
// Upstream bus speed captured at bus reset
    pub speed: c_uint,
// Standard USB Descriptors of the vhub.
    pub vhub_dev_desc: usb_device_descriptor,
    pub vhub_conf_desc: ast_vhub_full_cdesc,
    pub vhub_hub_desc: usb_hub_descriptor,
    pub vhub_str_desc: list_head,
    pub vhub_qual_desc: usb_qualifier_descriptor,
}

// Standard request handlers result codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum std_req_rc {
    std_req_stall = -1,	/* Stall requested */
    std_req_complete = 0,	/* Request completed with no data */
    std_req_data = 1,	/* Request completed with data */
    std_req_driver = 2,	/* Pass to driver pls */
}

//
// This works around a confirmed HW issue with the Aspeed chip.
//
// The core uses a different bus to memory than the AHB going to
// the USB device controller. Due to the latter having a higher
// priority than the core for arbitration on that bus, it's
// possible for an MMIO to the device, followed by a DMA by the
// device from memory to all be performed and services before
// a previous store to memory gets completed.
//
// This the following scenario can happen:
//
// - Driver writes to a DMA descriptor (Mbus)
// - Driver writes to the MMIO register to start the DMA (AHB)
// - The gadget sees the second write and sends a read of the
// descriptor to the memory controller (Mbus)
// - The gadget hits memory before the descriptor write
// causing it to read an obsolete value.
//
// Thankfully the problem is limited to the USB gadget device, other
// masters in the SoC all have a lower priority than the core, thus
// ensuring that the store by the core arrives first.
//
// The workaround consists of using a dummy read of the memory before
// doing the MMIO writes. This will ensure that the previous writes
// have been "pushed out".
//
// core.c
extern "C" {
    pub fn ast_vhub_nuke(ep: *mut ast_vhub_ep, status: c_int);
}
extern "C" {
    pub fn ast_vhub_free_request(u_ep: *mut usb_ep, u_req: *mut usb_request);
}
extern "C" {
    pub fn ast_vhub_init_hw(vhub: *mut ast_vhub);
}
// ep0.c
extern "C" {
    pub fn ast_vhub_ep0_handle_ack(ep: *mut ast_vhub_ep, in_ack: bool);
}
extern "C" {
    pub fn ast_vhub_ep0_handle_setup(ep: *mut ast_vhub_ep);
}
extern "C" {
    pub fn ast_vhub_reset_ep0(dev: *mut ast_vhub_dev);
}
extern "C" {
    pub fn ast_vhub_reply(ep: *mut ast_vhub_ep, ptr: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn __ast_vhub_simple_reply(ep: *mut ast_vhub_ep, len: c_int, ...) -> c_int;
}

// hub.c
extern "C" {
    pub fn ast_vhub_init_hub(vhub: *mut ast_vhub) -> c_int;
}
extern "C" {
    pub fn ast_vhub_hub_suspend(vhub: *mut ast_vhub);
}
extern "C" {
    pub fn ast_vhub_hub_resume(vhub: *mut ast_vhub);
}
extern "C" {
    pub fn ast_vhub_hub_reset(vhub: *mut ast_vhub);
}
extern "C" {
    pub fn ast_vhub_hub_wake_all(vhub: *mut ast_vhub);
}
// dev.c
extern "C" {
    pub fn ast_vhub_init_dev(vhub: *mut ast_vhub, idx: c_uint) -> c_int;
}
extern "C" {
    pub fn ast_vhub_del_dev(d: *mut ast_vhub_dev);
}
extern "C" {
    pub fn ast_vhub_dev_irq(d: *mut ast_vhub_dev);
}
// epn.c
extern "C" {
    pub fn ast_vhub_epn_ack_irq(ep: *mut ast_vhub_ep);
}
extern "C" {
    pub fn ast_vhub_update_epn_stall(ep: *mut ast_vhub_ep);
}
extern "C" {
    pub fn ast_vhub_dev_suspend(d: *mut ast_vhub_dev);
}
extern "C" {
    pub fn ast_vhub_dev_resume(d: *mut ast_vhub_dev);
}
extern "C" {
    pub fn ast_vhub_dev_reset(d: *mut ast_vhub_dev);
}
