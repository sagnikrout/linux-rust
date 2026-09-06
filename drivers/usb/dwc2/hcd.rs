//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc2/hcd.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// hcd.h - DesignWare HS OTG Controller host-mode declarations
//
// Copyright (C) 2004-2013 Synopsys, Inc.
//
// This file contains the structures, constants, and interfaces for the
// Host Contoller Driver (HCD)
//
// The Host Controller Driver (HCD) is responsible for translating requests
// from the USB Driver into the appropriate actions on the DWC_otg controller.
// It isolates the USBD from the specifics of the controller by providing an
// API to the USBD.
//
// struct dwc2_host_chan - Software host channel descriptor
//
// @hc_num:             Host channel number, used for register address lookup
// @dev_addr:           Address of the device
// @ep_num:             Endpoint of the device
// @ep_is_in:           Endpoint direction
// @speed:              Device speed. One of the following values:
// - USB_SPEED_LOW
// - USB_SPEED_FULL
// - USB_SPEED_HIGH
// @ep_type:            Endpoint type. One of the following values:
// - USB_ENDPOINT_XFER_CONTROL: 0
// - USB_ENDPOINT_XFER_ISOC:    1
// - USB_ENDPOINT_XFER_BULK:    2
// - USB_ENDPOINT_XFER_INTR:    3
// @max_packet:         Max packet size in bytes
// @data_pid_start:     PID for initial transaction.
// 0: DATA0
// 1: DATA2
// 2: DATA1
// 3: MDATA (non-Control EP),
// SETUP (Control EP)
// @multi_count:        Number of additional periodic transactions per
// (micro)frame
// @xfer_buf:           Pointer to current transfer buffer position
// @xfer_dma:           DMA address of xfer_buf
// @align_buf:          In Buffer DMA mode this will be used if xfer_buf is not
// DWORD aligned
// @xfer_len:           Total number of bytes to transfer
// @xfer_count:         Number of bytes transferred so far
// @start_pkt_count:    Packet count at start of transfer
// @xfer_started:       True if the transfer has been started
// @do_ping:            True if a PING request should be issued on this channel
// @error_state:        True if the error count for this transaction is non-zero
// @halt_on_queue:      True if this channel should be halted the next time a
// request is queued for the channel. This is necessary in
// slave mode if no request queue space is available when
// an attempt is made to halt the channel.
// @halt_pending:       True if the host channel has been halted, but the core
// is not finished flushing queued requests
// @do_split:           Enable split for the channel
// @complete_split:     Enable complete split
// @hub_addr:           Address of high speed hub for the split
// @hub_port:           Port of the low/full speed device for the split
// @xact_pos:           Split transaction position. One of the following values:
// - DWC2_HCSPLT_XACTPOS_MID
// - DWC2_HCSPLT_XACTPOS_BEGIN
// - DWC2_HCSPLT_XACTPOS_END
// - DWC2_HCSPLT_XACTPOS_ALL
// @requests:           Number of requests issued for this channel since it was
// assigned to the current transfer (not counting PINGs)
// @schinfo:            Scheduling micro-frame bitmap
// @ntd:                Number of transfer descriptors for the transfer
// @halt_status:        Reason for halting the host channel
// @hcint:               Contents of the HCINT register when the interrupt came
// @qh:                 QH for the transfer being processed by this channel
// @hc_list_entry:      For linking to list of host channels
// @desc_list_addr:     Current QH's descriptor list DMA address
// @desc_list_sz:       Current QH's descriptor list size
// @split_order_list_entry: List entry for keeping track of the order of splits
//
// This structure represents the state of a single host channel when acting in
// host mode. It contains the data items needed to transfer packets to an
// endpoint via a host channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_host_chan {
    pub hc_num: u8,
    pub dev_addr:7: unsigned,
    pub ep_num:4: unsigned,
    pub ep_is_in:1: unsigned,
    pub speed:4: unsigned,
    pub ep_type:2: unsigned,
    pub max_packet:11: unsigned,
    pub data_pid_start:2: unsigned,

    pub multi_count:2: unsigned,
    pub xfer_buf: *mut u8,
    pub xfer_dma: dma_addr_t,
    pub align_buf: dma_addr_t,
    pub xfer_len: u32,
    pub xfer_count: u32,
    pub start_pkt_count: u16,
    pub xfer_started: u8,
    pub do_ping: u8,
    pub error_state: u8,
    pub halt_on_queue: u8,
    pub halt_pending: u8,
    pub do_split: u8,
    pub complete_split: u8,
    pub hub_addr: u8,
    pub hub_port: u8,
    pub xact_pos: u8,

    pub requests: u8,
    pub schinfo: u8,
    pub ntd: u16,
    pub halt_status: dwc2_halt_status,
    pub hcint: u32,
    pub qh: *mut dwc2_qh,
    pub hc_list_entry: list_head,
    pub desc_list_addr: dma_addr_t,
    pub desc_list_sz: u32,
    pub split_order_list_entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hcd_pipe_info {
    pub dev_addr: u8,
    pub ep_num: u8,
    pub pipe_type: u8,
    pub pipe_dir: u8,
    pub maxp: u16,
    pub maxp_mult: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hcd_iso_packet_desc {
    pub offset: u32,
    pub length: u32,
    pub actual_length: u32,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hcd_urb {
    pub priv: *mut c_void,
    pub qtd: *mut dwc2_qtd,
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub setup_packet: *mut c_void,
    pub setup_dma: dma_addr_t,
    pub length: u32,
    pub actual_length: u32,
    pub status: u32,
    pub error_count: u32,
    pub packet_count: u32,
    pub flags: u32,
    pub interval: u16,
    pub pipe_info: dwc2_hcd_pipe_info,
    pub iso_descs: [dwc2_hcd_iso_packet_desc; ],
}

// Phases for control transfers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_control_phase {
    DWC2_CONTROL_SETUP,
    DWC2_CONTROL_DATA,
    DWC2_CONTROL_STATUS,
}

// Transaction types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_transaction_type {
    DWC2_TRANSACTION_NONE,
    DWC2_TRANSACTION_PERIODIC,
    DWC2_TRANSACTION_NON_PERIODIC,
    DWC2_TRANSACTION_ALL,
}

// The number of elements per LS bitmap (per port on multi_tt)

//
// struct dwc2_tt - dwc2 data associated with a usb_tt
//
// @refcount:           Number of Queue Heads (QHs) holding a reference.
// @usb_tt:             Pointer back to the official usb_tt.
// @periodic_bitmaps:   Bitmap for which parts of the 1ms frame are accounted
// for already.  Each is DWC2_ELEMENTS_PER_LS_BITMAP
// elements (so sizeof(long) times that in bytes).
//
// This structure is stored in the hcpriv of the official usb_tt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_tt {
    pub refcount: c_int,
    pub usb_tt: *mut usb_tt,
    pub periodic_bitmaps: [c_ulong; ],
}

//
// struct dwc2_hs_transfer_time - Info about a transfer on the high speed bus.
//
// @start_schedule_us:  The start time on the main bus schedule.  Note that
// the main bus schedule is tightly packed and this
// time should be interpreted as tightly packed (so
// uFrame 0 starts at 0 us, uFrame 1 starts at 100 us
// instead of 125 us).
// @duration_us:           How long this transfer goes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hs_transfer_time {
    pub start_schedule_us: u32,
    pub duration_us: u16,
}

//
// struct dwc2_qh - Software queue head structure
//
// @hsotg:              The HCD state structure for the DWC OTG controller
// @ep_type:            Endpoint type. One of the following values:
// - USB_ENDPOINT_XFER_CONTROL
// - USB_ENDPOINT_XFER_BULK
// - USB_ENDPOINT_XFER_INT
// - USB_ENDPOINT_XFER_ISOC
// @ep_is_in:           Endpoint direction
// @maxp:               Value from wMaxPacketSize field of Endpoint Descriptor
// @maxp_mult:          Multiplier for maxp
// @dev_speed:          Device speed. One of the following values:
// - USB_SPEED_LOW
// - USB_SPEED_FULL
// - USB_SPEED_HIGH
// @data_toggle:        Determines the PID of the next data packet for
// non-controltransfers. Ignored for control transfers.
// One of the following values:
// - DWC2_HC_PID_DATA0
// - DWC2_HC_PID_DATA1
// @ping_state:         Ping state
// @do_split:           Full/low speed endpoint on high-speed hub requires split
// @td_first:           Index of first activated isochronous transfer descriptor
// @td_last:            Index of last activated isochronous transfer descriptor
// @host_us:            Bandwidth in microseconds per transfer as seen by host
// @device_us:          Bandwidth in microseconds per transfer as seen by device
// @host_interval:      Interval between transfers as seen by the host.  If
// the host is high speed and the device is low speed this
// will be 8 times device interval.
// @device_interval:    Interval between transfers as seen by the device.
// interval.
// @next_active_frame:  (Micro)frame _before_ we next need to put something on
// the bus.  We'll move the qh to active here.  If the
// host is in high speed mode this will be a uframe.  If
// the host is in low speed mode this will be a full frame.
// @start_active_frame: If we are partway through a split transfer, this will be
// what next_active_frame was when we started.  Otherwise
// it should always be the same as next_active_frame.
// @num_hs_transfers:   Number of transfers in hs_transfers.
// Normally this is 1 but can be more than one for splits.
// Always >= 1 unless the host is in low/full speed mode.
// @hs_transfers:       Transfers that are scheduled as seen by the high speed
// bus.  Not used if host is in low or full speed mode (but
// note that it IS USED if the device is low or full speed
// as long as the HOST is in high speed mode).
// @ls_start_schedule_slice: Start time (in slices) on the low speed bus
// schedule that's being used by this device.  This
// will be on the periodic_bitmap in a
// "struct dwc2_tt".  Not used if this device is high
// speed.  Note that this is in "schedule slice" which
// is tightly packed.
// @ntd:                Actual number of transfer descriptors in a list
// @dw_align_buf:       Used instead of original buffer if its physical address
// is not dword-aligned
// @dw_align_buf_dma:   DMA address for dw_align_buf
// @qtd_list:           List of QTDs for this QH
// @channel:            Host channel currently processing transfers for this QH
// @qh_list_entry:      Entry for QH in either the periodic or non-periodic
// schedule
// @desc_list:          List of transfer descriptors
// @desc_list_dma:      Physical address of desc_list
// @desc_list_sz:       Size of descriptors list
// @n_bytes:            Xfer Bytes array. Each element corresponds to a transfer
// descriptor and indicates original XferSize value for the
// descriptor
// @unreserve_timer:    Timer for releasing periodic reservation.
// @wait_timer:         Timer used to wait before re-queuing.
// @dwc_tt:            Pointer to our tt info (or NULL if no tt).
// @ttport:             Port number within our tt.
// @tt_buffer_dirty     True if clear_tt_buffer_complete is pending
// @unreserve_pending:  True if we planned to unreserve but haven't yet.
// @schedule_low_speed: True if we have a low/full speed component (either the
// host is in low/full speed mode or do_split).
// @want_wait:          We should wait before re-queuing; only matters for non-
// periodic transfers and is ignored for periodic ones.
// @wait_timer_cancel:  Set to true to cancel the wait_timer.
//
// @tt_buffer_dirty:	True if EP's TT buffer is not clean.
// A Queue Head (QH) holds the static characteristics of an endpoint and
// maintains a list of transfers (QTDs) for that endpoint. A QH structure may
// be entered in either the non-periodic or periodic schedule.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_qh {
    pub hsotg: *mut dwc2_hsotg,
    pub ep_type: u8,
    pub ep_is_in: u8,
    pub maxp: u16,
    pub maxp_mult: u16,
    pub dev_speed: u8,
    pub data_toggle: u8,
    pub ping_state: u8,
    pub do_split: u8,
    pub td_first: u8,
    pub td_last: u8,
    pub host_us: u16,
    pub device_us: u16,
    pub host_interval: u16,
    pub device_interval: u16,
    pub next_active_frame: u16,
    pub start_active_frame: u16,
    pub num_hs_transfers: i16,
    pub hs_transfers: [dwc2_hs_transfer_time; DWC2_HS_SCHEDULE_UFRAMES],
    pub ls_start_schedule_slice: u32,
    pub ntd: u16,
    pub dw_align_buf: *mut u8,
    pub dw_align_buf_dma: dma_addr_t,
    pub qtd_list: list_head,
    pub channel: *mut dwc2_host_chan,
    pub qh_list_entry: list_head,
    pub desc_list: *mut dwc2_dma_desc,
    pub desc_list_dma: dma_addr_t,
    pub desc_list_sz: u32,
    pub n_bytes: *mut u32,
    pub unreserve_timer: timer_list,
    pub wait_timer: hrtimer,
    pub dwc_tt: *mut dwc2_tt,
    pub ttport: c_int,
    pub tt_buffer_dirty:1: unsigned,
    pub unreserve_pending:1: unsigned,
    pub schedule_low_speed:1: unsigned,
    pub want_wait:1: unsigned,
    pub wait_timer_cancel:1: unsigned,
}

//
// struct dwc2_qtd - Software queue transfer descriptor (QTD)
//
// @control_phase:      Current phase for control transfers (Setup, Data, or
// Status)
// @in_process:         Indicates if this QTD is currently processed by HW
// @data_toggle:        Determines the PID of the next data packet for the
// data phase of control transfers. Ignored for other
// transfer types. One of the following values:
// - DWC2_HC_PID_DATA0
// - DWC2_HC_PID_DATA1
// @complete_split:     Keeps track of the current split type for FS/LS
// endpoints on a HS Hub
// @isoc_split_pos:     Position of the ISOC split in full/low speed
// @isoc_frame_index:   Index of the next frame descriptor for an isochronous
// transfer. A frame descriptor describes the buffer
// position and length of the data to be transferred in the
// next scheduled (micro)frame of an isochronous transfer.
// It also holds status for that transaction. The frame
// index starts at 0.
// @isoc_split_offset:  Position of the ISOC split in the buffer for the
// current frame
// @ssplit_out_xfer_count: How many bytes transferred during SSPLIT OUT
// @error_count:        Holds the number of bus errors that have occurred for
// a transaction within this transfer
// @n_desc:             Number of DMA descriptors for this QTD
// @isoc_frame_index_last: Last activated frame (packet) index, used in
// descriptor DMA mode only
// @num_naks:           Number of NAKs received on this QTD.
// @urb:                URB for this transfer
// @qh:                 Queue head for this QTD
// @qtd_list_entry:     For linking to the QH's list of QTDs
// @isoc_td_first:	Index of first activated isochronous transfer
// descriptor in Descriptor DMA mode
// @isoc_td_last:	Index of last activated isochronous transfer
// descriptor in Descriptor DMA mode
//
// A Queue Transfer Descriptor (QTD) holds the state of a bulk, control,
// interrupt, or isochronous transfer. A single QTD is created for each URB
// (of one of these types) submitted to the HCD. The transfer associated with
// a QTD may require one or multiple transactions.
//
// A QTD is linked to a Queue Head, which is entered in either the
// non-periodic or periodic schedule for execution. When a QTD is chosen for
// execution, some or all of its transactions may be executed. After
// execution, the state of the QTD is updated. The QTD may be retired if all
// its transactions are complete or if an error occurred. Otherwise, it
// remains in the schedule so more transactions can be executed later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_qtd {
    pub control_phase: dwc2_control_phase,
    pub in_process: u8,
    pub data_toggle: u8,
    pub complete_split: u8,
    pub isoc_split_pos: u8,
    pub isoc_frame_index: u16,
    pub isoc_split_offset: u16,
    pub isoc_td_last: u16,
    pub isoc_td_first: u16,
    pub ssplit_out_xfer_count: u32,
    pub error_count: u8,
    pub n_desc: u8,
    pub isoc_frame_index_last: u16,
    pub num_naks: u16,
    pub urb: *mut dwc2_hcd_urb,
    pub qh: *mut dwc2_qh,
    pub qtd_list_entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_xfer_info {
    pub hsotg: *mut dwc2_hsotg,
    pub chan: *mut dwc2_host_chan,
}

extern "C" {
    pub fn dwc2_calc_frame_interval(hsotg: *mut dwc2_hsotg) -> u32;
}
// Gets the struct usb_hcd that contains a struct dwc2_hsotg
//
// Inline used to disable one channel interrupt. Channel interrupts are
// disabled when the channel is halted or released by the interrupt handler.
// There is no need to handle further interrupts of that type until the
// channel is re-assigned. In fact, subsequent handling may cause crashes
// because the channel structures are cleaned up when the channel is released.
//
extern "C" {
    pub fn dwc2_hc_cleanup(hsotg: *mut dwc2_hsotg, chan: *mut dwc2_host_chan);
}
//
// Reads HPRT0 in preparation to modify. It keeps the WC bits 0 so that if they
// are read as 1, they won't clear when written back.
//
extern "C" {
    pub fn dwc2_hcd_init(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hcd_remove(hsotg: *mut dwc2_hsotg);
}
// Transaction Execution Functions
// Schedule Queue Functions
// Implemented in hcd_queue.c
extern "C" {
    pub fn dwc2_hcd_qh_free(hsotg: *mut dwc2_hsotg, qh: *mut dwc2_qh);
}
extern "C" {
    pub fn dwc2_hcd_qh_add(hsotg: *mut dwc2_hsotg, qh: *mut dwc2_qh) -> c_int;
}
extern "C" {
    pub fn dwc2_hcd_qh_unlink(hsotg: *mut dwc2_hsotg, qh: *mut dwc2_qh);
}
extern "C" {
    pub fn dwc2_hcd_qtd_init(qtd: *mut dwc2_qtd, urb: *mut dwc2_hcd_urb);
}
// Unlinks and frees a QTD
// Descriptor DMA support functions
extern "C" {
    pub fn dwc2_hcd_qh_free_ddma(hsotg: *mut dwc2_hsotg, qh: *mut dwc2_qh);
}
// Check if QH is non-periodic

//
// Returns true if frame1 index is greater than frame2 index. The comparison
// is done modulo FRLISTEN_64_SIZE. This accounts for the rollover of the
// frame number when the max index frame number is reached.
//
// Returns true if frame1 is less than or equal to frame2. The comparison is
// done modulo HFNUM_MAX_FRNUM. This accounts for the rollover of the
// frame number when the max frame number is reached.
//
// Returns true if frame1 is greater than frame2. The comparison is done
// modulo HFNUM_MAX_FRNUM. This accounts for the rollover of the frame
// number when the max frame number is reached.
//
// Increments frame by the amount specified by inc. The addition is done
// modulo HFNUM_MAX_FRNUM. Returns the incremented value.
//
// Returns the Core Interrupt Status register contents, ANDed with the Core
// Interrupt Mask register contents
//
// HCD Core API
//
// dwc2_handle_hcd_intr() - Called on every hardware interrupt
//
// @hsotg: The DWC2 HCD
//
// Returns IRQ_HANDLED if interrupt is handled
// Return IRQ_NONE if interrupt is not handled
//
extern "C" {
    pub fn dwc2_handle_hcd_intr(hsotg: *mut dwc2_hsotg) -> irqreturn_t;
}
//
// dwc2_hcd_stop() - Halts the DWC_otg host mode operation
//
// @hsotg: The DWC2 HCD
//
extern "C" {
    pub fn dwc2_hcd_stop(hsotg: *mut dwc2_hsotg);
}
//
// dwc2_hcd_is_b_host() - Returns 1 if core currently is acting as B host,
// and 0 otherwise
//
// @hsotg: The DWC2 HCD
//
extern "C" {
    pub fn dwc2_hcd_is_b_host(hsotg: *mut dwc2_hsotg) -> c_int;
}
//
// dwc2_hcd_dump_state() - Dumps hsotg state
//
// @hsotg: The DWC2 HCD
//
// NOTE: This function will be removed once the peripheral controller code
// is integrated and the driver is stable
//
extern "C" {
    pub fn dwc2_hcd_dump_state(hsotg: *mut dwc2_hsotg);
}
// URB interface
// Transfer flags
pub const URB_GIVEBACK_ASAP: c_uint = 0x1;
pub const URB_SEND_ZERO_PACKET: c_uint = 0x2;
// Host driver callbacks
extern "C" {
    pub fn dwc2_host_get_speed(hsotg: *mut dwc2_hsotg, context: *mut c_void) -> c_int;
}
