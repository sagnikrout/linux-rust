//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/cdns2/cdns2-gadget.h
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
// USBHS-DEV device controller driver header file
//
// Copyright (C) 2023 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//

//
// USBHS register interface.
// This corresponds to the USBHS Device Controller Interface.
//
// struct cdns2_ep0_regs - endpoint 0 related registers.
// @rxbc: receive (OUT) 0 endpoint byte count register.
// @txbc: transmit (IN) 0 endpoint byte count register.
// @cs: 0 endpoint control and status register.
// @reserved1: reserved.
// @fifo: 0 endpoint fifo register.
// @reserved2: reserved.
// @setupdat: SETUP data register.
// @reserved4: reserved.
// @maxpack: 0 endpoint max packet size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_ep0_regs {
    pub rxbc: __u8,
    pub txbc: __u8,
    pub cs: __u8,
    pub reserved1: [__u8; 4],
    pub fifo: __u8,
    pub reserved2: [__le32; 94],
    pub setupdat: [__u8; 8],
    pub reserved4: [__u8; 88],
    pub maxpack: __u8,
    pub __aligned(4): } __packed,
// EP0CS - bitmasks.
// Endpoint 0 stall bit for status stage.

// HSNAK bit.

// IN 0 endpoint busy bit.

// OUT 0 endpoint busy bit.

// Send STALL in the data stage phase.

// SETUP buffer content was changed.

// EP0FIFO - bitmasks.
// Direction.

// FIFO auto bit.

// FIFO commit bit.

// FIFO access bit.

//
// struct cdns2_epx_base - base endpoint registers.
// @rxbc: OUT endpoint byte count register.
// @rxcon: OUT endpoint control register.
// @rxcs: OUT endpoint control and status register.
// @txbc: IN endpoint byte count register.
// @txcon: IN endpoint control register.
// @txcs: IN endpoint control and status register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_epx_base {
    pub rxbc: __le16,
    pub rxcon: __u8,
    pub rxcs: __u8,
    pub txbc: __le16,
    pub txcon: __u8,
    pub txcs: __u8,
    pub __aligned(4): } __packed,
// rxcon/txcon - endpoint control register bitmasks.
// Endpoint buffering: 0 - single buffering ... 3 - quad buffering.

// Endpoint type.

// Endpoint type: isochronous.
pub const EPX_CON_TYPE_ISOC: c_uint = 0x4;
// Endpoint type: bulk.
pub const EPX_CON_TYPE_BULK: c_uint = 0x8;
// Endpoint type: interrupt.
pub const EPX_CON_TYPE_INT: c_uint = 0xC;
// Number of packets per microframe.

pub const EPX_CON_ISOD_SHIFT: c_uint = 0x4;
// Endpoint stall bit.

// Endpoint enable bit.

// rxcs/txcs - endpoint control and status bitmasks.
// Data sequence error for the ISO endpoint.

//
// struct cdns2_epx_regs - endpoint 1..15 related registers.
// @reserved: reserved.
// @ep: none control endpoints array.
// @reserved2: reserved.
// @endprst: endpoint reset register.
// @reserved3: reserved.
// @isoautoarm: ISO auto-arm register.
// @reserved4: reserved.
// @isodctrl: ISO control register.
// @reserved5: reserved.
// @isoautodump: ISO auto dump enable register.
// @reserved6: reserved.
// @rxmaxpack: receive (OUT) Max packet size register.
// @reserved7: reserved.
// @rxstaddr: receive (OUT) start address endpoint buffer register.
// @reserved8: reserved.
// @txstaddr: transmit (IN) start address endpoint buffer register.
// @reserved9: reserved.
// @txmaxpack: transmit (IN) Max packet size register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_epx_regs {
    pub reserved: [__le32; 2],
    pub ep: [cdns2_epx_base; 15],
    pub reserved2: [__u8; 290],
    pub endprst: __u8,
    pub reserved3: [__u8; 41],
    pub isoautoarm: __le16,
    pub reserved4: [__u8; 10],
    pub isodctrl: __le16,
    pub reserved5: __le16,
    pub isoautodump: __le16,
    pub reserved6: __le32,
    pub rxmaxpack: [__le16; 15],
    pub reserved7: [__le32; 65],
    pub rxstaddr: [__le32; 15],
    pub reserved8: [__u8; 4],
    pub txstaddr: [__le32; 15],
    pub reserved9: [__u8; 98],
    pub txmaxpack: [__le16; 15],
    pub __aligned(4): } __packed,
// ENDPRST - bitmasks.
// Endpoint number.

// IN direction bit.

// Toggle reset bit.

// FIFO reset bit.

// Toggle status and reset bit.

//
// struct cdns2_interrupt_regs - USB interrupt related registers.
// @reserved: reserved.
// @usbirq: USB interrupt request register.
// @extirq: external interrupt request register.
// @rxpngirq: external interrupt request register.
// @reserved1: reserved.
// @usbien: USB interrupt enable register.
// @extien: external interrupt enable register.
// @reserved2: reserved.
// @usbivect: USB interrupt vector register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_interrupt_regs {
    pub reserved: [__u8; 396],
    pub usbirq: __u8,
    pub extirq: __u8,
    pub rxpngirq: __le16,
    pub reserved1: [__le16; 4],
    pub usbien: __u8,
    pub extien: __u8,
    pub reserved2: [__le16; 3],
    pub usbivect: __u8,
    pub __aligned(4): } __packed,
// EXTIRQ and EXTIEN - bitmasks.
// VBUS fault fall interrupt.

// VBUS fault fall interrupt.

// Wake up interrupt bit.

// USBIEN and USBIRQ - bitmasks.
// SETUP data valid interrupt bit.

// Start-of-frame interrupt bit.

// SETUP token interrupt bit.

// USB suspend interrupt bit.

// USB reset interrupt bit.

// USB high-speed mode interrupt bit.

// Link Power Management interrupt bit.

//
// struct cdns2_usb_regs - USB controller registers.
// @reserved: reserved.
// @lpmctrl: LPM control register.
// @lpmclock: LPM clock register.
// @reserved2: reserved.
// @endprst: endpoint reset register.
// @usbcs: USB control and status register.
// @frmnr: USB frame counter register.
// @fnaddr: function Address register.
// @clkgate: clock gate register.
// @fifoctrl: FIFO control register.
// @speedctrl: speed Control register.
// @sleep_clkgate: sleep Clock Gate register.
// @reserved3: reserved.
// @cpuctrl: microprocessor control register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_usb_regs {
    pub reserved: [__u8; 4],
    pub lpmctrl: __u16,
    pub lpmclock: __u8,
    pub reserved2: [__u8; 411],
    pub endprst: __u8,
    pub usbcs: __u8,
    pub frmnr: __le16,
    pub fnaddr: __u8,
    pub clkgate: __u8,
    pub fifoctrl: __u8,
    pub speedctrl: __u8,
    pub sleep_clkgate: __u8,
    pub reserved3: [__u8; 533],
    pub cpuctrl: __u8,
    pub __aligned(4): } __packed,
// LPMCTRL - bitmasks.
// BESL (Best Effort Service Latency).

// Last received Remote Wakeup field from LPM Extended Token packet.

// Reflects value of the lpmnyet bit located in the usbcs[1] register.

// LPMCLOCK - bitmasks.
//
// If bit is 1 the controller automatically turns off clock
// (utmisleepm goes to low), else the microprocessor should use
// sleep clock gate register to turn off clock.
//

// USBCS - bitmasks.
// Send NYET handshake for the LPM transaction.

// Remote wake-up bit.

// Software disconnect bit.

// Indicates that a wakeup pin resumed the controller.

// FIFOCTRL - bitmasks.
// Endpoint number.

// Direction bit.

// FIFO auto bit.

// FIFO commit bit.

// FIFO access bit.

// SPEEDCTRL - bitmasks.
// Device works in Full Speed.

// Device works in High Speed.

// Force FS mode.

// CPUCTRL- bitmasks.
// UP clock enable

// Controller reset bit.

//
// If the wuen bit is ‘1’, the upclken is automatically set to ‘1’ after
// detecting rising edge of wuintereq interrupt. If the wuen bit is ‘0’,
// the wuintereq interrupt is ignored.
//

//
// struct cdns2_adma_regs - ADMA controller registers.
// @conf: DMA global configuration register.
// @sts: DMA global Status register.
// @reserved1: reserved.
// @ep_sel: DMA endpoint select register.
// @ep_traddr: DMA endpoint transfer ring address register.
// @ep_cfg: DMA endpoint configuration register.
// @ep_cmd: DMA endpoint command register.
// @ep_sts: DMA endpoint status register.
// @reserved2: reserved.
// @ep_sts_en: DMA endpoint status enable register.
// @drbl: DMA doorbell register.
// @ep_ien: DMA endpoint interrupt enable register.
// @ep_ists: DMA endpoint interrupt status register.
// @axim_ctrl: AXI Master Control register.
// @axim_id: AXI Master ID register.
// @reserved3: reserved.
// @axim_cap: AXI Master Wrapper Extended Capability.
// @reserved4: reserved.
// @axim_ctrl0: AXI Master Wrapper Extended Capability Control Register 0.
// @axim_ctrl1: AXI Master Wrapper Extended Capability Control Register 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_adma_regs {
    pub conf: __le32,
    pub sts: __le32,
    pub reserved1: [__le32; 5],
    pub ep_sel: __le32,
    pub ep_traddr: __le32,
    pub ep_cfg: __le32,
    pub ep_cmd: __le32,
    pub ep_sts: __le32,
    pub reserved2: __le32,
    pub ep_sts_en: __le32,
    pub drbl: __le32,
    pub ep_ien: __le32,
    pub ep_ists: __le32,
    pub axim_ctrl: __le32,
    pub axim_id: __le32,
    pub reserved3: __le32,
    pub axim_cap: __le32,
    pub reserved4: __le32,
    pub axim_ctrl0: __le32,
    pub axim_ctrl1: __le32,
}

pub const CDNS2_ADMA_REGS_OFFSET: c_uint = 0x400;
// DMA_CONF - bitmasks.
// Reset USB device configuration.

// Singular DMA transfer mode.

// Multiple DMA transfers mode.

// DMA_EP_CFG - bitmasks.
// Endpoint enable.

// DMA_EP_CMD - bitmasks.
// Endpoint reset.

// Transfer descriptor ready.

// Data flush.

// DMA_EP_STS - bitmasks.
// Interrupt On Complete.

// Interrupt on Short Packet.

// Transfer descriptor missing.

// TRB error.

// DMA busy bit.

// Current Cycle Status.

// OUT size mismatch.

// ISO transmission error.

// DMA_EP_STS_EN - bitmasks.
// OUT transfer missing descriptor enable.

// TRB enable.

// OUT size mismatch enable.

// ISO transmission error enable.

// DMA_EP_IEN - bitmasks.

// DMA_EP_ISTS - bitmasks.

// -------------------------------------------------------------------------
pub const TRBS_PER_SEGMENT: c_int = 600;
pub const ISO_MAX_INTERVAL: c_int = 8;

pub const MAX_ISO_SIZE: c_int = 3076;
//
// To improve performance the TRB buffer pointers can't cross
// 4KB boundaries.
//
pub const TRB_MAX_ISO_BUFF_SHIFT: c_int = 12;

// How much data is left before the 4KB boundary?

//
// struct cdns2_trb - represent Transfer Descriptor block.
// @buffer: pointer to buffer data.
// @length: length of data.
// @control: control flags.
//
// This structure describes transfer block handled by DMA module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_trb {
    pub buffer: __le32,
    pub length: __le32,
    pub control: __le32,
}

//
// These two extra TRBs are reserved for isochronous transfer
// to inject 0 length packet and extra LINK TRB to synchronize the ISO transfer.
//
pub const TRB_ISO_RESERVED: c_int = 2;

// TRB bit mask.

// TRB type IDs.
// Used for Bulk, Interrupt, ISOC, and control data stage.
pub const TRB_NORMAL: c_int = 1;
// TRB for linking ring segments.
pub const TRB_LINK: c_int = 6;
// Cycle bit - indicates TRB ownership by driver or hw.

//
// When set to '1', the device will toggle its interpretation of the Cycle bit.
//

// Interrupt on short packet.

// Chain bit associate this TRB with next one TRB.

// Interrupt on completion.

// Transfer_len bitmasks.

// Data buffer pointer bitmasks.

// -------------------------------------------------------------------------
// Driver numeric constants.
// Maximum address that can be assigned to device.
pub const USB_DEVICE_MAX_ADDRESS: c_int = 127;
// One control and 15 IN and 15 OUT endpoints.
pub const CDNS2_ENDPOINTS_NUM: c_int = 31;
pub const CDNS2_EP_ZLP_BUF_SIZE: c_int = 512;
// -------------------------------------------------------------------------
// Used structures.
//
// struct cdns2_ring - transfer ring representation.
// @trbs: pointer to transfer ring.
// @dma: dma address of transfer ring.
// @free_trbs: number of free TRBs in transfer ring.
// @pcs: producer cycle state.
// @ccs: consumer cycle state.
// @enqueue: enqueue index in transfer ring.
// @dequeue: dequeue index in transfer ring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_ring {
    pub trbs: *mut cdns2_trb,
    pub dma: dma_addr_t,
    pub free_trbs: c_int,
    pub pcs: u8,
    pub ccs: u8,
    pub enqueue: c_int,
    pub dequeue: c_int,
}

//
// struct cdns2_endpoint - extended device side representation of USB endpoint.
// @endpoint: usb endpoint.
// @pending_list: list of requests queuing on transfer ring.
// @deferred_list: list of requests waiting for queuing on transfer ring.
// @pdev: device associated with endpoint.
// @name: a human readable name e.g. ep1out.
// @ring: transfer ring associated with endpoint.
// @ep_state: state of endpoint.
// @idx: index of endpoint in pdev->eps table.
// @dir: endpoint direction.
// @num: endpoint number (1 - 15).
// @type: set to bmAttributes & USB_ENDPOINT_XFERTYPE_MASK.
// @interval: interval between packets used for ISOC and Interrupt endpoint.
// @buffering: on-chip buffers assigned to endpoint.
// @trb_burst_size: number of burst used in TRB.
// @skip: Sometimes the controller cannot process isochronous endpoint ring
// quickly enough and it will miss some isoc tds on the ring and
// generate ISO transmition error.
// Driver sets skip flag when receive a ISO transmition error and
// process the missed TDs on the endpoint ring.
// @wa1_set: use WA1.
// @wa1_trb: TRB assigned to WA1.
// @wa1_trb_index: TRB index for WA1.
// @wa1_cycle_bit: correct cycle bit for WA1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_endpoint {
    pub endpoint: usb_ep,
    pub pending_list: list_head,
    pub deferred_list: list_head,
    pub pdev: *mut cdns2_device,
    pub name: [c_char; 20],
    pub ring: cdns2_ring,

    pub ep_state: u32,
    pub idx: u8,
    pub dir: u8,
    pub num: u8,
    pub type: u8,
    pub interval: c_int,
    pub buffering: u8,
    pub trb_burst_size: u8,
    pub skip: bool,
    pub wa1_set:1: c_uint,
    pub wa1_trb: *mut cdns2_trb,
    pub wa1_trb_index: c_uint,
    pub wa1_cycle_bit:1: c_uint,
}

//
// struct cdns2_request - extended device side representation of usb_request
// object.
// @request: generic usb_request object describing single I/O request.
// @pep: extended representation of usb_ep object.
// @trb: the first TRB association with this request.
// @start_trb: number of the first TRB in transfer ring.
// @end_trb: number of the last TRB in transfer ring.
// @list: used for queuing request in lists.
// @finished_trb: number of trb has already finished per request.
// @num_of_trb: how many trbs are associated with request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_request {
    pub request: usb_request,
    pub pep: *mut cdns2_endpoint,
    pub trb: *mut cdns2_trb,
    pub start_trb: c_int,
    pub end_trb: c_int,
    pub list: list_head,
    pub finished_trb: c_int,
    pub num_of_trb: c_int,
}

// Stages used during enumeration process.
pub const CDNS2_SETUP_STAGE: c_uint = 0x0;
pub const CDNS2_DATA_STAGE: c_uint = 0x1;
pub const CDNS2_STATUS_STAGE: c_uint = 0x2;
//
// struct cdns2_device - represent USB device.
// @dev: pointer to device structure associated whit this controller.
// @gadget: device side representation of the peripheral controller.
// @gadget_driver: pointer to the gadget driver.
// @lock: for synchronizing.
// @irq: interrupt line number.
// @regs: base address for registers
// @usb_regs: base address for common USB registers.
// @ep0_regs: base address for endpoint 0 related registers.
// @epx_regs: base address for all none control endpoint registers.
// @interrupt_regs: base address for interrupt handling related registers.
// @adma_regs: base address for ADMA registers.
// @eps_dma_pool: endpoint Transfer Ring pool.
// @setup: used while processing usb control requests.
// @ep0_preq: private request used while handling EP0.
// @ep0_stage: ep0 stage during enumeration process.
// @zlp_buf: zlp buffer.
// @dev_address: device address assigned by host.
// @eps: array of objects describing endpoints.
// @selected_ep: actually selected endpoint. It's used only to improve
// performance by limiting access to dma_ep_sel register.
// @is_selfpowered: device is self powered.
// @may_wakeup: allows device to remote wakeup the host.
// @status_completion_no_call: indicate that driver is waiting for status
// stage completion. It's used in deferred SET_CONFIGURATION request.
// @in_lpm: indicate the controller is in low power mode.
// @pending_status_wq: workqueue handling status stage for deferred requests.
// @pending_status_request: request for which status stage was deferred.
// @eps_supported: endpoints supported by controller in form:
// bit: 0 - ep0, 1 - epOut1, 2 - epIn1, 3 - epOut2 ...
// @burst_opt: array with the best burst size value for different TRB size.
// @onchip_tx_buf: size of transmit on-chip buffer in KB.
// @onchip_rx_buf: size of receive on-chip buffer in KB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns2_device {
    pub dev: *mut device,
    pub gadget: usb_gadget,
    pub gadget_driver: *mut usb_gadget_driver,
// generic spin-lock for drivers
    pub lock: spinlock_t,
    pub irq: c_int,
    pub regs: *mut void __iomem,
    pub usb_regs: *mut cdns2_usb_regs __iomem,
    pub ep0_regs: *mut cdns2_ep0_regs __iomem,
    pub epx_regs: *mut cdns2_epx_regs __iomem,
    pub interrupt_regs: *mut cdns2_interrupt_regs __iomem,
    pub adma_regs: *mut cdns2_adma_regs __iomem,
    pub eps_dma_pool: *mut dma_pool,
    pub setup: usb_ctrlrequest,
    pub ep0_preq: cdns2_request,
    pub ep0_stage: u8,
    pub zlp_buf: *mut c_void,
    pub dev_address: u8,
    pub eps: [cdns2_endpoint; CDNS2_ENDPOINTS_NUM],
    pub selected_ep: u32,
    pub is_selfpowered: bool,
    pub may_wakeup: bool,
    pub status_completion_no_call: bool,
    pub in_lpm: bool,
    pub pending_status_wq: work_struct,
    pub pending_status_request: *mut usb_request,
    pub eps_supported: u32,
    pub 1]: u8 burst_opt[MAX_ISO_SIZE +,
// in KB
    pub onchip_tx_buf: u16,
    pub onchip_rx_buf: u16,
}

extern "C" {
    pub fn cdns2_pending_setup_status_handler(work: *mut work_struct);
}
extern "C" {
    pub fn cdns2_select_ep(pdev: *mut cdns2_device, ep: u32);
}
extern "C" {
    pub fn cdns2_gadget_ep_dequeue(ep: *mut usb_ep, request: *mut usb_request) -> c_int;
}
extern "C" {
    pub fn cdns2_init_ep0(pdev: *mut cdns2_device, pep: *mut cdns2_endpoint);
}
extern "C" {
    pub fn cdns2_ep0_config(pdev: *mut cdns2_device);
}
extern "C" {
    pub fn cdns2_handle_ep0_interrupt(pdev: *mut cdns2_device, dir: c_int);
}
extern "C" {
    pub fn cdns2_handle_setup_packet(pdev: *mut cdns2_device);
}
extern "C" {
    pub fn cdns2_gadget_resume(pdev: *mut cdns2_device, hibernated: bool) -> c_int;
}
extern "C" {
    pub fn cdns2_gadget_suspend(pdev: *mut cdns2_device) -> c_int;
}
extern "C" {
    pub fn cdns2_gadget_remove(pdev: *mut cdns2_device);
}
extern "C" {
    pub fn cdns2_gadget_init(pdev: *mut cdns2_device) -> c_int;
}
extern "C" {
    pub fn set_reg_bit_8(ptr: *mut void __iomem, mask: u8);
}
