//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_core.h
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
// MUSB OTG driver defines
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//

// Helper defines for struct musb->hwvers

pub const MUSB_HWVERS_RC: c_uint = 0x8000;
pub const MUSB_HWVERS_1300: c_uint = 0x52C;
pub const MUSB_HWVERS_1400: c_uint = 0x590;
pub const MUSB_HWVERS_1800: c_uint = 0x720;
pub const MUSB_HWVERS_1900: c_uint = 0x784;
pub const MUSB_HWVERS_2000: c_uint = 0x800;

// NOTE:  otg and peripheral-only state machines start at B_IDLE.
// OTG or host-only go to A_IDLE when ID is sensed.
//

// CONSTANTS

// host side ep0 states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_h_ep0_state {
    MUSB_EP0_IDLE,
    MUSB_EP0_START,			/* expect ack of setup */
    MUSB_EP0_IN,			/* expect IN DATA */
    MUSB_EP0_OUT,			/* expect ack of OUT DATA */
    MUSB_EP0_STATUS,		/* expect ack of STATUS */
    } __attribute__ ((packed));

// peripheral side ep0 states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_g_ep0_state {
    MUSB_EP0_STAGE_IDLE,		/* idle, waiting for SETUP */
    MUSB_EP0_STAGE_SETUP,		/* received SETUP */
    MUSB_EP0_STAGE_TX,		/* IN data */
    MUSB_EP0_STAGE_RX,		/* OUT data */
    MUSB_EP0_STAGE_STATUSIN,	/* (after OUT data) */
    MUSB_EP0_STAGE_STATUSOUT,	/* (after IN data) */
    MUSB_EP0_STAGE_ACKWAIT,		/* after zlp, before statusin */
    } __attribute__ ((packed));

//
// OTG protocol constants.  See USB OTG 1.3 spec,
// sections 5.5 "Device Timings" and 6.6.5 "Timers".
//

// FUNCTIONS

// Macro flag: #define MUSB_HST_MODE(_musb)\
    { (_musb)->is_host = true; }

    { (_musb)->is_host = false; }

    (musb_readb((_x)->mregs, MUSB_DEVCTL)&MUSB_DEVCTL_HM)

// TYPES

    struct musb_io;

//
// struct musb_platform_ops - Operations passed to musb_core by HW glue layer
// @quirks:	flags for platform specific quirks
// @enable:	enable device
// @disable:	disable device
// @ep_offset:	returns the end point offset
// @ep_select:	selects the specified end point
// @fifo_mode:	sets the fifo mode
// @fifo_offset: returns the fifo offset
// @readb:	read 8 bits
// @writeb:	write 8 bits
// @clearb:	could be clear-on-readb or W1C
// @readw:	read 16 bits
// @writew:	write 16 bits
// @clearw:	could be clear-on-readw or W1C
// @read_fifo:	reads the fifo
// @write_fifo:	writes to fifo
// @get_toggle:	platform specific get toggle function
// @set_toggle:	platform specific set toggle function
// @dma_init:	platform specific dma init function
// @dma_exit:	platform specific dma exit function
// @init:	turns on clocks, sets up platform-specific registers, etc
// @exit:	undoes @init
// @set_mode:	forcefully changes operating mode
// @try_idle:	tries to idle the IP
// @recover:	platform-specific babble recovery
// @vbus_status: returns vbus status if possible
// @set_vbus:	forces vbus status
// @pre_root_reset_end: called before the root usb port reset flag gets cleared
// @post_root_reset_end: called after the root usb port reset flag gets cleared
// @phy_callback: optional callback function for the phy to call
//
    struct musb_platform_ops {

    u32	quirks;

    int	(*init)(struct musb *musb);
    int	(*exit)(struct musb *musb);

    void	(*enable)(struct musb *musb);
    void	(*disable)(struct musb *musb);

    u32	(*ep_offset)(u8 epnum, u16 offset);
    void	(*ep_select)(void __iomem *mbase, u8 epnum);
    u16	fifo_mode;
    u32	(*fifo_offset)(u8 epnum);
    u32	(*busctl_offset)(u8 epnum, u16 offset);
    u8	(*readb)(void __iomem *addr, u32 offset);
    void	(*writeb)(void __iomem *addr, u32 offset, u8 data);
    u8	(*clearb)(void __iomem *addr, u32 offset);
    u16	(*readw)(void __iomem *addr, u32 offset);
    void	(*writew)(void __iomem *addr, u32 offset, u16 data);
    u16	(*clearw)(void __iomem *addr, u32 offset);
    void	(*read_fifo)(struct musb_hw_ep *hw_ep, u16 len, u8 *buf);
    void	(*write_fifo)(struct musb_hw_ep *hw_ep, u16 len, const u8 *buf);
    u16	(*get_toggle)(struct musb_qh *qh, int is_out);
    u16	(*set_toggle)(struct musb_qh *qh, int is_out, struct urb *urb);
    struct dma_controller *
    (*dma_init) (struct musb *musb, void __iomem *base);
    void	(*dma_exit)(struct dma_controller *c);
    int	(*set_mode)(struct musb *musb, u8 mode);
    void	(*try_idle)(struct musb *musb, unsigned long timeout);
    int	(*recover)(struct musb *musb);

    int	(*vbus_status)(struct musb *musb);
    void	(*set_vbus)(struct musb *musb, int on);

    void	(*pre_root_reset_end)(struct musb *musb);
    void	(*post_root_reset_end)(struct musb *musb);
    int	(*phy_callback)(enum musb_vbus_id_status status);
    void	(*clear_ep_rxintr)(struct musb *musb, int epnum);
}

//
// struct musb_hw_ep - endpoint hardware (bidirectional)
//
// Ordered slightly for better cacheline locality.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_hw_ep {
    pub musb: *mut musb,
    pub fifo: *mut void __iomem,
    pub regs: *mut void __iomem,

    pub conf: *mut void __iomem,

// index in musb->endpoints[]
    pub epnum: u8,
// hardware configuration, possibly dynamic
    pub is_shared_fifo: bool,
    pub tx_double_buffered: bool,
    pub rx_double_buffered: bool,
    pub max_packet_sz_tx: u16,
    pub max_packet_sz_rx: u16,
    pub tx_channel: *mut dma_channel,
    pub rx_channel: *mut dma_channel,

// TUSB has "asynchronous" and "synchronous" dma modes
    pub fifo_async: dma_addr_t,
    pub fifo_sync: dma_addr_t,
    pub fifo_sync_va: *mut void __iomem,

// currently scheduled peripheral endpoint
    pub in_qh: *mut musb_qh,
    pub out_qh: *mut musb_qh,
    pub rx_reinit: u8,
    pub tx_reinit: u8,
// peripheral side
    pub /: *mut *mut musb_ep ep_in; / TX,
    pub /: *mut *mut musb_ep ep_out; / RX,
}

extern "C" {
    pub fn next_request(_arg: &hw_ep->ep_in) -> return;
}
extern "C" {
    pub fn next_request(_arg: &hw_ep->ep_out) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_csr_regs {
// FIFO registers
    pub rxcsr: u16 txmaxp, txcsr, rxmaxp,,
    pub txfifoadd: u16 rxfifoadd,,
    pub rxinterval: u8 txtype, txinterval, rxtype,,
    pub txfifosz: u8 rxfifosz,,
    pub txhubport: u8 txfunaddr, txhubaddr,,
    pub rxhubport: u8 rxfunaddr, rxhubaddr,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_context_registers {
    pub power: u8,
    pub intrusbe: u8,
    pub frame: u16,
    pub testmode: u8 index,,
    pub misc: u8 devctl, busctl,,
    pub otg_interfsel: u32,
    pub index_regs: [musb_csr_regs; MUSB_C_NUM_EPS],
}

//
// struct musb - Driver instance data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb {
// device lock
    pub lock: spinlock_t,
    pub /: *mut *mut spinlock_t list_lock; / resume work list lock,
    pub io: musb_io,
    pub ops: *const musb_platform_ops,
    pub context: musb_context_registers,
    pub ): *mut *mut irqreturn_t (isr)(int, void,
    pub irq_work: delayed_work,
    pub deassert_reset_work: delayed_work,
    pub finish_resume_work: delayed_work,
    pub gadget_work: delayed_work,
    pub hwvers: u16,
    pub intrrxe: u16,
    pub intrtxe: u16,
// this hub status bit is reserved by USB 2.0 and not seen by usbcore

    pub port1_status: u32,
    pub rh_timer: c_ulong,
    pub ep0_stage: musb_h_ep0_state,
// bulk traffic normally dedicates endpoint hardware, and each
// direction has its own ring of host side endpoints.
// we try to progress the transfer at the head of each endpoint's
// queue until it completes or NAKs too much; then we try the next
// endpoint.
//
    pub bulk_ep: *mut musb_hw_ep,
    pub /: *mut *mut list_head control; / of musb_qh,
    pub /: *mut *mut list_head in_bulk; / of musb_qh,
    pub /: *mut *mut list_head out_bulk; / of musb_qh,
    pub /: *mut *mut list_head pending_list; / pending work list,
    pub otg_timer: timer_list,
    pub dev_timer: timer_list,
    pub nb: notifier_block,
    pub dma_controller: *mut dma_controller,
    pub controller: *mut device,
    pub ctrl_base: *mut void __iomem,
    pub mregs: *mut void __iomem,

    pub async: dma_addr_t,
    pub sync: dma_addr_t,
    pub sync_va: *mut void __iomem,
    pub tusb_revision: u8,

// passed down from chip/board specific irq handlers
    pub int_usb: u8,
    pub int_rx: u16,
    pub int_tx: u16,
    pub xceiv: *mut usb_phy,
    pub phy: *mut phy,
    pub otg_state: usb_otg_state,
    pub nIrq: c_int,
    pub irq_wake:1: unsigned,
    pub endpoints: [musb_hw_ep; MUSB_C_NUM_EPS],
pub const VBUSERR_RETRY_COUNT: c_int = 3;
    pub vbuserr_retry: u16,
    pub epmask: u16,
    pub nr_endpoints: u8,
    pub /: *mut *mut u8 min_power; / vbus for periph, in mA/2,
    pub port_mode: musb_mode,
    pub session: bool,
    pub quirk_retries: c_ulong,
    pub is_host: bool,
    pub /: *mut *mut int a_wait_bcon; / VBUS timeout in msecs,
    pub /: *mut *mut unsigned long idle_timeout; / Next timeout in jiffies,
    pub is_initialized:1: unsigned,
    pub is_runtime_suspended:1: unsigned,
// active means connected and not suspended
    pub is_active:1: unsigned,
    pub is_multipoint:1: unsigned,
    pub /: *mut *mut unsigned hb_iso_rx:1; / high bandwidth iso rx?,
    pub /: *mut *mut unsigned hb_iso_tx:1; / high bandwidth iso tx?,
    pub /: *mut *mut unsigned dyn_fifo:1; / dynamic FIFO supported?,
    pub bulk_split:1: unsigned,

    pub bulk_combine:1: unsigned,

// is_suspended means USB B_PERIPHERAL suspend
    pub is_suspended:1: unsigned,
// may_wakeup means remote wakeup is enabled
    pub may_wakeup:1: unsigned,
// is_self_powered is reported in device status and the
// config descriptor.  is_bus_powered means B_PERIPHERAL
// draws some VBUS current; both can be true.
//
    pub is_self_powered:1: unsigned,
    pub is_bus_powered:1: unsigned,
    pub set_address:1: unsigned,
    pub test_mode:1: unsigned,
    pub softconnect:1: unsigned,
    pub flush_irq_work:1: unsigned,
    pub address: u8,
    pub test_mode_nr: u8,
    pub /: *mut *mut u16 ackpend; / ep0,
    pub ep0_state: musb_g_ep0_state,
    pub /: *mut *mut usb_gadget g; / the gadget,
    pub /: *mut *mut *mut usb_gadget_driver gadget_driver; / its driver,
    pub /: *mut *mut *mut usb_hcd hcd; / the usb hcd,
    pub config: *const musb_hdrc_config,
    pub xceiv_old_state: c_int,

    pub debugfs_root: *mut dentry,

}

// This must be included after struct musb is defined

extern "C" {
    pub fn container_of(_arg: g, musb: struct, _arg: g) -> return;
}
// read from core using indexed model
// 0's returned when no more endpoints
// shared TX/RX FIFO?
// Glue it together
extern "C" {
    pub fn musb_stop(musb: *mut musb);
}
extern "C" {
    pub fn musb_start(musb: *mut musb);
}
extern "C" {
    pub fn musb_write_fifo(ep: *mut musb_hw_ep, len: u16, src: *const u8);
}
extern "C" {
    pub fn musb_read_fifo(ep: *mut musb_hw_ep, len: u16, dst: *mut u8);
}
extern "C" {
    pub fn musb_set_host(musb: *mut musb) -> c_int;
}
extern "C" {
    pub fn musb_set_peripheral(musb: *mut musb) -> c_int;
}
extern "C" {
    pub fn musb_load_testpacket(: *mut musb);
}
extern "C" {
    pub fn musb_interrupt(: *mut musb) -> irqreturn_t;
}
extern "C" {
    pub fn musb_hnp_stop(musb: *mut musb);
}
extern "C" {
    pub fn usb_otg_state_string(_arg: musb_get_state(musb)) -> return;
}
//
// gets the "dr_mode" property from DT and converts it into musb_mode
// if the property is not found or not recognized returns MUSB_OTG
//
extern "C" {
    pub fn musb_get_mode(dev: *mut device) -> musb_mode;
}
