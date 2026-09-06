//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci.h
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
// xHCI host controller driver
//
// Copyright (C) 2008 Intel Corp.
//
// Author: Sarah Sharp
// Some code borrowed from the Linux EHCI driver.
//

// Code sharing between pci-quirks and xhci hcd

// max buffer size for trace and debug messages
pub const XHCI_MSG_MAX: c_int = 500;
// xHCI PCI Configuration Registers

//
// Max number of Devices Slots. xHCI specification section 5.3.3
// Valid values are in the range of 1 to 255.
//
pub const MAX_HC_SLOTS: c_int = 255;
//
// Max Number of Ports. xHCI specification section 5.3.3
// Valid values are in the range of 1 to 255.
//
pub const MAX_HC_PORTS: c_int = 127;
//
// Max number of Interrupter Register Sets. xHCI specification section 5.3.3
// Valid values are in the range of 1 to 1024.
//
pub const MAX_HC_INTRS: c_int = 128;
//
// xHCI register interface.
// This corresponds to the eXtensible Host Controller Interface (xHCI)
// Revision 0.95 specification
//
// struct xhci_cap_regs - xHCI Host Controller Capability Registers.
// @hc_capbase:		length of the capabilities register and HC version number
// @hcs_params1:	HCSPARAMS1 - Structural Parameters 1
// @hcs_params2:	HCSPARAMS2 - Structural Parameters 2
// @hcs_params3:	HCSPARAMS3 - Structural Parameters 3
// @hcc_params:		HCCPARAMS - Capability Parameters
// @db_off:		DBOFF - Doorbell array offset
// @run_regs_off:	RTSOFF - Runtime register space offset
// @hcc_params2:	HCCPARAMS2 Capability Parameters 2, xhci 1.1 only
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_cap_regs {
    pub hc_capbase: __le32,
    pub hcs_params1: __le32,
    pub hcs_params2: __le32,
    pub hcs_params3: __le32,
    pub hcc_params: __le32,
    pub db_off: __le32,
    pub run_regs_off: __le32,
    pub /: *mut *mut __le32 hcc_params2; / xhci 1.1,
// Reserved up to (CAPLENGTH - 0x1C)
}

//
// struct xhci_port_regs - Host Controller USB Port Register Set. xHCI spec 5.4.8
// @portsc:	Port Status and Control
// @portpmsc:	Port Power Management Status and Control
// @portli:	Port Link Info
// @porthlmpc:	Port Hardware LPM Control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_port_regs {
    pub portsc: __le32,
    pub portpmsc: __le32,
    pub portli: __le32,
    pub porthlmpc: __le32,
}

//
// struct xhci_op_regs - xHCI Host Controller Operational Registers.
// @command:		USBCMD - xHC command register
// @status:		USBSTS - xHC status register
// @page_size:		This indicates the page size that the host controller
// supports.  If bit n is set, the HC supports a page size
// of 2^(n+12), up to a 128MB page size.
// 4K is the minimum page size.
// @cmd_ring:		CRP - 64-bit Command Ring Pointer
// @dcbaa_ptr:		DCBAAP - 64-bit Device Context Base Address Array Pointer
// @config_reg:		CONFIG - Configure Register
// @port_regs:		Port Register Sets, from 1 to MaxPorts (defined by HCSPARAMS1).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_op_regs {
    pub command: __le32,
    pub status: __le32,
    pub page_size: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub dev_notification: __le32,
    pub cmd_ring: __le64,
// rsvd: offset 0x20-2F
    pub reserved3: [__le32; 4],
    pub dcbaa_ptr: __le64,
    pub config_reg: __le32,
// rsvd: offset 0x3C-3FF
    pub reserved4: [__le32; 241],
    pub port_regs: [xhci_port_regs; ],
}

// USBCMD - USB command - command bitmasks
// start/stop HC execution - do not write unless HC is halted

// Reset HC - resets internal HC state machine and all registers (except
// PCI config regs).  HC does NOT drive a USB reset on the downstream ports.
// The xHCI driver must reinitialize the xHC after setting this bit.
//

// Event Interrupt Enable - a '1' allows interrupts from the host controller

// Host System Error Interrupt Enable - get out-of-band signal for HC errors

// bits 4:6 are reserved (and should be preserved on writes).
// light reset (port status stays unchanged) - reset completed when this is 0

// host controller save/restore state.

// Enable Wrap Event - '1' means xHC generates an event when MFINDEX wraps.

// MFINDEX power management - '1' means xHC can stop MFINDEX counter if all root
// hubs are in U3 (selective suspend), disconnect, disabled, or powered-off.
// '0' means the xHC can power it off if all ports are in the disconnect,
// disabled, or powered-off state.
//

// bit 14 Extended TBC Enable, changes Isoc TRB fields to support larger TBC

// bits 15:31 are reserved (and should be preserved on writes).

// USBSTS - USB status - status bitmasks
// HC not running - set to 1 when run/stop bit is cleared.

// serious error, e.g. PCI parity error.  The HC will clear the run/stop bit.

// event interrupt - clear this prior to clearing any IP flags in IR set

// port change detect

// bits 5:7 reserved and zeroed
// save state status - '1' means xHC is saving state

// restore state status - '1' means xHC is restoring state

// true: save or restore error

// true: Controller Not Ready to accept doorbell or op reg writes after reset

// true: internal Host Controller Error - SW needs to reset and reinitialize

// bits 13:31 reserved and should be preserved
//
// DNCTRL - Device Notification Control Register - dev_notification bitmasks
// Generate a device notification event when the HC sees a transaction with a
// notification type that matches a bit set in this bit field.
//

// Most of the device notification types should only be used for debug.
// SW does need to pay attention to function wake notifications.
//

// CRCR - Command Ring Control Register - cmd_ring bitmasks
// bit 0 - Cycle bit indicates the ownership of the command ring

// stop ring operation after completion of the currently executing command

// stop ring immediately - abort the currently executing command

// true: command ring is running

// bits 63:6 - Command Ring pointer

// CONFIG - Configure Register - config_reg bitmasks
// bits 0:7 - maximum number of device slots enabled (NumSlotsEn)

// bit 8: U3 Entry Enabled, assert PLC when root port enters U3, xhci 1.1

// bit 9: Configuration Information Enable, xhci 1.1

// bits 10:31 - reserved and should be preserved
// bits 15:0 - HCD page shift bit
pub const XHCI_PAGE_SIZE_MASK: c_uint = 0xffff;
//
// struct xhci_intr_reg - Interrupt Register Set, v1.2 section 5.5.2.
// @iman:		IMAN - Interrupt Management Register. Used to enable
// interrupts and check for pending interrupts.
// @imod:		IMOD - Interrupt Moderation Register. Used to throttle interrupts.
// @erst_size:		ERSTSZ - Number of segments in the Event Ring Segment Table (ERST).
// @erst_base:		ERSTBA - Event ring segment table base address.
// @erst_dequeue:	ERDP - Event ring dequeue pointer.
//
// Each interrupter (defined by a MSI-X vector) has an event ring and an Event
// Ring Segment Table (ERST) associated with it.  The event ring is comprised of
// multiple segments of the same size.  The HC places events on the ring and
// "updates the Cycle bit in the TRBs to indicate to software the current
// position of the Enqueue Pointer." The HCD (Linux) processes those events and
// updates the dequeue pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_intr_reg {
    pub iman: __le32,
    pub imod: __le32,
    pub erst_size: __le32,
    pub rsvd: __le32,
    pub erst_base: __le64,
    pub erst_dequeue: __le64,
}

// iman bitmasks
// bit 0 - Interrupt Pending (IP), whether there is an interrupt pending. Write-1-to-clear.

// bit 1 - Interrupt Enable (IE), whether the interrupter is capable of generating an interrupt

// imod bitmasks
//
// bits 15:0 - Interrupt Moderation Interval, the minimum interval between interrupts
// (in 250ns intervals). The interval between interrupts will be longer if there are no
// events on the event ring. Default is 4000 (1 ms).
//

// bits 31:16 - Interrupt Moderation Counter, used to count down the time to the next interrupt

// erst_size bitmasks
// bits 15:0 - Event Ring Segment Table Size, number of ERST entries

// erst_base bitmasks
// bits 63:6 - Event Ring Segment Table Base Address Register

// erst_dequeue bitmasks
//
// bits 2:0 - Dequeue ERST Segment Index (DESI), is the segment number (or alias) where the
// current dequeue pointer lies. This is an optional HW hint.
//

//
// bit 3 - Event Handler Busy (EHB), whether the event ring is scheduled to be serviced by
// a work queue (or delayed service routine)?
//

// bits 63:4 - Event Ring Dequeue Pointer

//
// struct xhci_run_regs
// @microframe_index:
// MFINDEX - current microframe number
//
// Section 5.5 Host Controller Runtime Registers:
// "Software should read and write these registers using only Dword (32 bit)
// or larger accesses"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_run_regs {
    pub microframe_index: __le32,
    pub rsvd: [__le32; 7],
    pub ir_set: [xhci_intr_reg; 1024],
}

// Bits [13:3] of the microframe index equals the 1ms frame index

pub const MAX_FRAMES: c_int = 2048;

//
// struct doorbell_array
//
// Bits  0 -  7: Endpoint target
// Bits  8 - 15: RsvdZ
// Bits 16 - 31: Stream ID
//
// Section 5.6
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_doorbell_array {
    pub doorbell: [__le32; 256],
}

pub const DB_VALUE_HOST: c_uint = 0x00000000;

//
// struct xhci_container_ctx
// @type: Type of context.  Used to calculated offsets to contained contexts.
// @size: Size of the context data
// @bytes: The raw context data given to HW
// @dma: dma address of the bytes
//
// Represents either a Device or Input context.  Holds a pointer to the raw
// memory used for the context (bytes) and dma address of it (dma).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_container_ctx {
    pub type: unsigned,
pub const XHCI_CTX_TYPE_DEVICE: c_uint = 0x1;
pub const XHCI_CTX_TYPE_INPUT: c_uint = 0x2;
    pub size: c_int,
    pub bytes: *mut u8,
    pub dma: dma_addr_t,
}

//
// struct xhci_slot_ctx
// @dev_info:	Route string, device speed, hub info, and last valid endpoint
// @dev_info2:	Max exit latency for device number, root hub port number
// @tt_info:	tt_info is used to construct split transaction tokens
// @dev_state:	slot state and device address
//
// Slot Context - section 6.2.1.1.  This assumes the HC uses 32-byte context
// structures.  If the HC uses 64-byte contexts, there is an additional 32 bytes
// reserved at the end of the slot context for HC internal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_slot_ctx {
    pub dev_info: __le32,
    pub dev_info2: __le32,
    pub tt_info: __le32,
    pub dev_state: __le32,
// offset 0x10 to 0x1f reserved for HC internal use
    pub reserved: [__le32; 4],
}

// dev_info bitmasks
// Route String - 0:19

// Device speed - values defined by PORTSC Device Speed field - 20:23

// bit 24 reserved
// Is this LS/FS device connected through a HS hub? - bit 25

// Set if the device is a hub - bit 26

// Index of the last valid endpoint context in this device context - 27:31

// dev_info2 bitmasks
// Max Exit Latency (ms) - worst case time to wake up all links in dev path

// Root hub port number that is needed to access the USB device

// Maximum number of ports under a hub device

// tt_info bitmasks
//
// TT Hub Slot ID - for low or full speed devices attached to a high-speed hub
// The Slot ID of the hub that isolates the high speed signaling from
// this low or full-speed device.  '0' if attached to root hub port.
//

//
// The number of the downstream facing port of the high-speed hub
// '0' if the device is not low or full speed.
//

// dev_state bitmasks
// USB device address - assigned by the HC

// bits 8:26 reserved
// Slot state

pub const SLOT_STATE_DISABLED: c_int = 0;

pub const SLOT_STATE_DEFAULT: c_int = 1;
pub const SLOT_STATE_ADDRESSED: c_int = 2;
pub const SLOT_STATE_CONFIGURED: c_int = 3;
//
// struct xhci_ep_ctx
// @ep_info:	endpoint state, streams, mult, and interval information.
// @ep_info2:	information on endpoint type, max packet size, max burst size,
// error count, and whether the HC will force an event for all
// transactions.
// @deq:	64-bit ring dequeue pointer address.  If the endpoint only
// defines one stream, this points to the endpoint transfer ring.
// Otherwise, it points to a stream context array, which has a
// ring pointer for each flow.
// @tx_info:
// Average TRB lengths for the endpoint ring and
// max payload within an Endpoint Service Interval Time (ESIT).
//
// Endpoint Context - section 6.2.1.2.  This assumes the HC uses 32-byte context
// structures.  If the HC uses 64-byte contexts, there is an additional 32 bytes
// reserved at the end of the endpoint context for HC internal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_ep_ctx {
    pub ep_info: __le32,
    pub ep_info2: __le32,
    pub deq: __le64,
    pub tx_info: __le32,
// offset 0x14 - 0x1f reserved for HC internal use
    pub reserved: [__le32; 3],
}

// ep_info bitmasks
//
// Endpoint State - bits 0:2
// 0 - disabled
// 1 - running
// 2 - halted due to halt condition - ok to manipulate endpoint ring
// 3 - stopped
// 4 - TRB error
// 5-7 - reserved
//

pub const EP_STATE_DISABLED: c_int = 0;
pub const EP_STATE_RUNNING: c_int = 1;
pub const EP_STATE_HALTED: c_int = 2;
pub const EP_STATE_STOPPED: c_int = 3;
pub const EP_STATE_ERROR: c_int = 4;

// Mult - Max number of burtst within an interval, in EP companion desc.

// bits 10:14 are Max Primary Streams
// bit 15 is Linear Stream Array
// Interval - period between requests to an endpoint - 125u increments.

// Endpoint is set up with a Linear Stream Array (vs. Secondary Stream Array)

// hosts with LEC=1 use bits 31:24 as ESIT high bits.

// ep_info2 bitmasks
//
// Force Event - generate transfer events for all TRBs for this endpoint
// This will tell the HC to ignore the IOC and ISP flags (for debugging only).
//

pub const ISOC_OUT_EP: c_int = 1;
pub const BULK_OUT_EP: c_int = 2;
pub const INT_OUT_EP: c_int = 3;
pub const CTRL_EP: c_int = 4;
pub const ISOC_IN_EP: c_int = 5;
pub const BULK_IN_EP: c_int = 6;
pub const INT_IN_EP: c_int = 7;
// bit 6 reserved
// bit 7 is Host Initiate Disable - for disabling stream selection

// tx_info bitmasks

// deq bitmasks

// bits 63:4 - TR Dequeue Pointer

//
// struct xhci_input_control_context
// Input control context; see section 6.2.5.
//
// @drop_context:	set the bit of the endpoint context you want to disable
// @add_context:	set the bit of the endpoint context you want to enable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_input_control_ctx {
    pub drop_flags: __le32,
    pub add_flags: __le32,
    pub rsvd2: [__le32; 6],
}

// Represents everything that is needed to issue a command on the command ring.
// It's useful to pre-allocate these for commands that cannot fail due to
// out-of-memory errors, like freeing streams.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_command {
// Input context for changing device state
    pub in_ctx: *mut xhci_container_ctx,
    pub status: u32,
    pub comp_param: u32,
    pub slot_id: c_int,
// If completion is null, no one is waiting on this command
// and the structure can be freed after the command completes.
//
    pub completion: *mut completion,
    pub command_trb: *mut xhci_trb,
    pub cmd_list: list_head,
// xHCI command response timeout in milliseconds
    pub timeout_ms: c_uint,
}

// drop context bitmasks

// add context bitmasks

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_stream_ctx {
// 64-bit stream ring address, cycle state, and stream type
    pub stream_ring: __le64,
// offset 0x14 - 0x1f reserved for HC internal use
    pub reserved: [__le32; 2],
}

// Stream Context Types (section 6.4.1) - bits 3:1 of stream ctx deq ptr

// Secondary stream array type, dequeue pointer is to a transfer ring
pub const SCT_SEC_TR: c_int = 0;
// Primary stream array type, dequeue pointer is to a transfer ring
pub const SCT_PRI_TR: c_int = 1;
// Dequeue pointer is for a secondary stream array (SSA) with 8 entries
pub const SCT_SSA_8: c_int = 2;
pub const SCT_SSA_16: c_int = 3;
pub const SCT_SSA_32: c_int = 4;
pub const SCT_SSA_64: c_int = 5;
pub const SCT_SSA_128: c_int = 6;
pub const SCT_SSA_256: c_int = 7;
// Assume no secondary streams for now
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_stream_info {
    pub stream_rings: *mut xhci_ring,
// Number of streams, including stream 0 (which drivers can't use)
    pub num_streams: c_uint,
// The stream context array may be bigger than
// the number of streams the driver asked for
//
    pub stream_ctx_array: *mut xhci_stream_ctx,
    pub num_stream_ctxs: c_uint,
    pub ctx_array_dma: dma_addr_t,
// For mapping physical TRB addresses to segments in stream rings
    pub trb_address_map: radix_tree_root,
    pub free_streams_command: *mut xhci_command,
}

pub const SMALL_STREAM_ARRAY_SIZE: c_int = 256;
pub const MEDIUM_STREAM_ARRAY_SIZE: c_int = 1024;
pub const GET_PORT_BW_ARRAY_SIZE: c_int = 256;
// Some Intel xHCI host controllers need software to keep track of the bus
// bandwidth.  Keep track of endpoint info here.  Each root port is allocated
// the full bus bandwidth.  We must also treat TTs (including each port under a
// multi-TT hub) as a separate bandwidth domain.  The direct memory interface
// (DMI) also limits the total bandwidth (across all domains) that can be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_bw_info {
// ep_interval is zero-based
    pub ep_interval: c_uint,
// mult and num_packets are one-based
    pub mult: c_uint,
    pub num_packets: c_uint,
    pub max_packet_size: c_uint,
    pub max_esit_payload: c_uint,
    pub type: c_uint,
}

// "Block" sizes in bytes the hardware uses for different device speeds.
// The logic in this part of the hardware limits the number of bits the hardware
// can use, so must represent bandwidth in a less precise manner to mimic what
// the scheduler hardware computes.
//
pub const FS_BLOCK: c_int = 1;
pub const HS_BLOCK: c_int = 4;
pub const SS_BLOCK: c_int = 16;
pub const DMI_BLOCK: c_int = 32;
// Each device speed has a protocol overhead (CRC, bit stuffing, etc) associated
// with each byte transferred.  SuperSpeed devices have an initial overhead to
// set up bursts.  These are in blocks, see above.  LS overhead has already been
// translated into FS blocks.
//
pub const DMI_OVERHEAD: c_int = 8;
pub const DMI_OVERHEAD_BURST: c_int = 4;
pub const SS_OVERHEAD: c_int = 8;
pub const SS_OVERHEAD_BURST: c_int = 32;
pub const HS_OVERHEAD: c_int = 26;
pub const FS_OVERHEAD: c_int = 20;
pub const LS_OVERHEAD: c_int = 128;
// The TTs need to claim roughly twice as much bandwidth (94 bytes per
// microframe ~= 24Mbps) of the HS bus as the devices can actually use because
// of overhead associated with split transfers crossing microframe boundaries.
// 31 blocks is pure protocol overhead.
//

// Bandwidth limits in blocks
pub const FS_BW_LIMIT: c_int = 1285;
pub const TT_BW_LIMIT: c_int = 1320;
pub const HS_BW_LIMIT: c_int = 1607;
pub const SS_BW_LIMIT_IN: c_int = 3906;
pub const DMI_BW_LIMIT_IN: c_int = 3906;
pub const SS_BW_LIMIT_OUT: c_int = 3906;
pub const DMI_BW_LIMIT_OUT: c_int = 3906;
// Percentage of bus bandwidth reserved for non-periodic transfers
pub const FS_BW_RESERVED: c_int = 10;
pub const HS_BW_RESERVED: c_int = 20;
pub const SS_BW_RESERVED: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_virt_ep {
    pub /: *mut *mut *mut xhci_virt_device vdev; / parent,
    pub ep_index: c_uint,
    pub ring: *mut xhci_ring,
// Related to endpoints that are configured to use stream IDs only
    pub stream_info: *mut xhci_stream_info,
// Temporary storage in case the configure endpoint command fails and we
// have to restore the device state to the previous state
//
    pub new_ring: *mut xhci_ring,
    pub err_count: c_uint,
    pub ep_state: c_uint,

// Transitioning the endpoint to using streams, don't enqueue URBs

// Transitioning the endpoint to not using streams, don't enqueue URBs

// usb_hub_clear_tt_buffer is in progress

// ----  Related to URB cancellation ----
    pub cancelled_td_list: list_head,
// Dequeue pointer and dequeue segment for a submitted Set TR Dequeue
// command.  We'll need to update the ring's dequeue segment and dequeue
// pointer after the command completes.
//
    pub queued_deq_seg: *mut xhci_segment,
    pub queued_deq_ptr: *mut xhci_trb,
//
// Sometimes the xHC can not process isochronous endpoint ring quickly
// enough, and it will miss some isoc tds on the ring and generate
// a Missed Service Error Event.
// Set skip flag when receive a Missed Service Error Event and
// process the missed tds on the endpoint ring.
//
    pub skip: bool,
// Bandwidth checking storage
    pub bw_info: xhci_bw_info,
    pub bw_endpoint_list: list_head,
    pub stop_time: c_ulong,
// Isoch Frame ID checking storage
    pub next_uframe: c_int,
// Use new Isoch TRB layout needed for extended TBC support
    pub use_extended_tbc: bool,
// set if this endpoint is controlled via sideband access
    pub sideband: *mut xhci_sideband,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_overhead_type {
    LS_OVERHEAD_TYPE = 0,
    FS_OVERHEAD_TYPE,
    HS_OVERHEAD_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_interval_bw {
    pub num_packets: c_uint,
// Sorted by max packet size.
// Head of the list is the greatest max packet size.
//
    pub endpoints: list_head,
// How many endpoints of each speed are present.
    pub overhead: [c_uint; 3],
}

pub const XHCI_MAX_INTERVAL: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_interval_bw_table {
    pub interval0_esit_payload: c_uint,
    pub interval_bw: [xhci_interval_bw; XHCI_MAX_INTERVAL],
// Includes reserved bandwidth for async endpoints
    pub bw_used: c_uint,
    pub ss_bw_in: c_uint,
    pub ss_bw_out: c_uint,
}

pub const EP_CTX_PER_DEV: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_virt_device {
    pub slot_id: c_int,
    pub udev: *mut usb_device,
//
// Commands to the hardware are passed an "input context" that
// tells the hardware what to change in its data structures.
// The hardware will return changes in an "output context" that
// software must allocate for the hardware.  We need to keep
// track of input and output contexts separately because
// these commands might fail and we don't trust the hardware.
//
    pub out_ctx: *mut xhci_container_ctx,
// Used for addressing devices and configuration changes
    pub in_ctx: *mut xhci_container_ctx,
    pub eps: [xhci_virt_ep; EP_CTX_PER_DEV],
    pub rhub_port: *mut xhci_port,
    pub bw_table: *mut xhci_interval_bw_table,
    pub tt_info: *mut xhci_tt_bw_info,
// The current max exit latency for the enabled USB3 link states.
    pub current_mel: u16,
// Used for the debugfs interfaces.
    pub debugfs_private: *mut c_void,
// set if this endpoint is controlled via sideband access
    pub sideband: *mut xhci_sideband,
}

//
// For each roothub, keep track of the bandwidth information for each periodic
// interval.
//
// If a high speed hub is attached to the roothub, each TT associated with that
// hub is a separate bandwidth domain.  The interval information for the
// endpoints on the devices under that TT will appear in the TT structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_root_port_bw_info {
    pub tts: list_head,
    pub num_active_tts: c_uint,
    pub bw_table: xhci_interval_bw_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_tt_bw_info {
    pub tt_list: list_head,
    pub slot_id: c_int,
    pub ttport: c_int,
    pub bw_table: xhci_interval_bw_table,
    pub active_eps: c_int,
}

//
// struct xhci_device_context_array
// @ctx_array:	Pointer to an array of addresses. The array size depends on Max
// Slots read from HCSPARAMS1.
// @dma:	DMA address to @ctx_array
//
// Device Context Base Address Array (DCBAA) - Section 6.1.
// ctx_array[0]:		Scratchpad Buffer Array Base Address
// ctx_array[1-MaxSlots]:	Device Context Base Address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_device_context_array {
    pub ctx_array: *mut __le64,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_transfer_event {
// 64-bit buffer address, or immediate data
    pub buffer: __le64,
    pub transfer_len: __le32,
// This field is interpreted differently based on the type of TRB
    pub flags: __le32,
}

// Transfer event flags bitfield, also for select command completion events

// Transfer event TRB length bit mask

// Completion Code - only applicable for some types of TRBs

pub const COMP_INVALID: c_int = 0;
pub const COMP_SUCCESS: c_int = 1;
pub const COMP_DATA_BUFFER_ERROR: c_int = 2;
pub const COMP_BABBLE_DETECTED_ERROR: c_int = 3;
pub const COMP_USB_TRANSACTION_ERROR: c_int = 4;
pub const COMP_TRB_ERROR: c_int = 5;
pub const COMP_STALL_ERROR: c_int = 6;
pub const COMP_RESOURCE_ERROR: c_int = 7;
pub const COMP_BANDWIDTH_ERROR: c_int = 8;
pub const COMP_NO_SLOTS_AVAILABLE_ERROR: c_int = 9;
pub const COMP_INVALID_STREAM_TYPE_ERROR: c_int = 10;
pub const COMP_SLOT_NOT_ENABLED_ERROR: c_int = 11;
pub const COMP_ENDPOINT_NOT_ENABLED_ERROR: c_int = 12;
pub const COMP_SHORT_PACKET: c_int = 13;
pub const COMP_RING_UNDERRUN: c_int = 14;
pub const COMP_RING_OVERRUN: c_int = 15;
pub const COMP_VF_EVENT_RING_FULL_ERROR: c_int = 16;
pub const COMP_PARAMETER_ERROR: c_int = 17;
pub const COMP_BANDWIDTH_OVERRUN_ERROR: c_int = 18;
pub const COMP_CONTEXT_STATE_ERROR: c_int = 19;
pub const COMP_NO_PING_RESPONSE_ERROR: c_int = 20;
pub const COMP_EVENT_RING_FULL_ERROR: c_int = 21;
pub const COMP_INCOMPATIBLE_DEVICE_ERROR: c_int = 22;
pub const COMP_MISSED_SERVICE_ERROR: c_int = 23;
pub const COMP_COMMAND_RING_STOPPED: c_int = 24;
pub const COMP_COMMAND_ABORTED: c_int = 25;
pub const COMP_STOPPED: c_int = 26;
pub const COMP_STOPPED_LENGTH_INVALID: c_int = 27;
pub const COMP_STOPPED_SHORT_PACKET: c_int = 28;
pub const COMP_MAX_EXIT_LATENCY_TOO_LARGE_ERROR: c_int = 29;
pub const COMP_ISOCH_BUFFER_OVERRUN: c_int = 31;
pub const COMP_EVENT_LOST_ERROR: c_int = 32;
pub const COMP_UNDEFINED_ERROR: c_int = 33;
pub const COMP_INVALID_STREAM_ID_ERROR: c_int = 34;
pub const COMP_SECONDARY_BANDWIDTH_ERROR: c_int = 35;
pub const COMP_SPLIT_TRANSACTION_ERROR: c_int = 36;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_link_trb {
// 64-bit segment pointer
    pub segment_ptr: __le64,
    pub intr_target: __le32,
    pub control: __le32,
}

// control bitfields

// Command completion event TRB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_event_cmd {
// Pointer to command TRB, or the value passed by the event data trb
    pub cmd_trb: __le64,
    pub status: __le32,
    pub flags: __le32,
}

// status bitmasks

// Address device - disable SetAddress

// Configure Endpoint - Deconfigure

// Stop Ring - Transfer State Preserve

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_ep_reset_type {
    EP_HARD_RESET,
    EP_SOFT_RESET,
}

// Force Event

// Set Latency Tolerance Value

// Get Port Bandwidth

// Force Header

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_setup_dev {
    SETUP_CONTEXT_ONLY,
    SETUP_CONTEXT_ADDRESS,
}

// bits 16:23 are the virtual function ID
// bits 24:31 are the slot ID
// bits 19:16 are the dev speed

// Stop Endpoint TRB - ep_index to endpoint ID for this TRB

pub const LAST_EP_INDEX: c_int = 30;
// Set TR Dequeue Pointer command TRB fields, 6.4.3.9

// Link TRB specific fields

// Port Status Change Event TRB fields
// Port ID - bits 31:24

// Normal TRB fields
// transfer_len bitmasks - bits 0:16

// TD Size, packets remaining in this TD, bits 21:17 (5 bits, so max 31)

// xhci 1.1 uses the TD_SIZE field for TBC if Extended TBC is enabled (ETE)

// Interrupter Target - which MSI-X vector to target the completion event at

// Cycle bit - indicates TRB ownership by HC or HCD

//
// Force next event data TRB to be evaluated before task switch.
// Used to pass OS data back after a TD completes.
//

// Interrupt on short packet

// Set PCIe no snoop attribute

// Chain multiple TRBs into a TD

// Interrupt on completion

// The buffer pointer contains immediate data

// TDs smaller than this might use IDT
pub const TRB_IDT_MAX_SIZE: c_int = 8;
// Block Event Interrupt

// Control transfer TRB specific fields

pub const TRB_DATA_OUT: c_int = 2;
pub const TRB_DATA_IN: c_int = 3;
// Isochronous TRB specific fields

// Total burst count field, Rsvdz on xhci 1.1 with Extended TBC enabled (ETE)

// TRB cache size for xHC with TRB cache
pub const TRB_CACHE_SIZE_HS: c_int = 8;
pub const TRB_CACHE_SIZE_SS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_generic_trb {
    pub field: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xhci_trb {
    pub link: xhci_link_trb,
    pub trans_event: xhci_transfer_event,
    pub event_cmd: xhci_event_cmd,
    pub generic: xhci_generic_trb,
}

// TRB bit mask

// TRB type IDs
// bulk, interrupt, isoc scatter/gather, and control data stage
pub const TRB_NORMAL: c_int = 1;
// setup stage for control transfers
pub const TRB_SETUP: c_int = 2;
// data stage for control transfers
pub const TRB_DATA: c_int = 3;
// status stage for control transfers
pub const TRB_STATUS: c_int = 4;
// isoc transfers
pub const TRB_ISOC: c_int = 5;
// TRB for linking ring segments
pub const TRB_LINK: c_int = 6;
pub const TRB_EVENT_DATA: c_int = 7;
// Transfer Ring No-op (not for the command ring)
pub const TRB_TR_NOOP: c_int = 8;
// Command TRBs
// Enable Slot Command
pub const TRB_ENABLE_SLOT: c_int = 9;
// Disable Slot Command
pub const TRB_DISABLE_SLOT: c_int = 10;
// Address Device Command
pub const TRB_ADDR_DEV: c_int = 11;
// Configure Endpoint Command
pub const TRB_CONFIG_EP: c_int = 12;
// Evaluate Context Command
pub const TRB_EVAL_CONTEXT: c_int = 13;
// Reset Endpoint Command
pub const TRB_RESET_EP: c_int = 14;
// Stop Transfer Ring Command
pub const TRB_STOP_RING: c_int = 15;
// Set Transfer Ring Dequeue Pointer Command
pub const TRB_SET_DEQ: c_int = 16;
// Reset Device Command
pub const TRB_RESET_DEV: c_int = 17;
// Force Event Command (opt)
pub const TRB_FORCE_EVENT: c_int = 18;
// Negotiate Bandwidth Command (opt)
pub const TRB_NEG_BANDWIDTH: c_int = 19;
// Set Latency Tolerance Value Command (opt)
pub const TRB_SET_LT: c_int = 20;
// Get port bandwidth Command
pub const TRB_GET_BW: c_int = 21;
// Force Header Command - generate a transaction or link management packet
pub const TRB_FORCE_HEADER: c_int = 22;
// No-op Command - not for transfer rings
pub const TRB_CMD_NOOP: c_int = 23;
// TRB IDs 24-31 reserved
// Event TRBS
// Transfer Event
pub const TRB_TRANSFER: c_int = 32;
// Command Completion Event
pub const TRB_COMPLETION: c_int = 33;
// Port Status Change Event
pub const TRB_PORT_STATUS: c_int = 34;
// Bandwidth Request Event (opt)
pub const TRB_BANDWIDTH_EVENT: c_int = 35;
// Doorbell Event (opt)
pub const TRB_DOORBELL: c_int = 36;
// Host Controller Event
pub const TRB_HC_EVENT: c_int = 37;
// Device Notification Event - device sent function wake notification
pub const TRB_DEV_NOTE: c_int = 38;
// MFINDEX Wrap Event - microframe counter wrapped
pub const TRB_MFINDEX_WRAP: c_int = 39;
// TRB IDs 40-47 reserved, 48-63 is vendor-defined
pub const TRB_VENDOR_DEFINED_LOW: c_int = 48;
// Nec vendor-specific command completion event.
pub const TRB_NEC_CMD_COMP: c_int = 48;
// Get NEC firmware revision.
pub const TRB_NEC_GET_FW: c_int = 49;

// Above, but for __le32 types -- can avoid work by swapping constants:

//
// TRBS_PER_SEGMENT must be a multiple of 4,
// since the command ring is 64-byte aligned.
// It must also be greater than 16.
//
pub const TRBS_PER_SEGMENT: c_int = 256;
// Allow two commands + a link TRB, along with any reserved command TRBs

// TRB buffer pointers can't cross 64KB boundaries
pub const TRB_MAX_BUFF_SHIFT: c_int = 16;

// How much data is left before the 64KB boundary?

pub const MAX_SOFT_RETRY: c_int = 3;
//
// Limits of consecutive isoc trbs that can Block Event Interrupt (BEI) if
// XHCI_AVOID_BEI quirk is in use.
//
pub const AVOID_BEI_INTERVAL_MIN: c_int = 8;
pub const AVOID_BEI_INTERVAL_MAX: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_segment {
    pub trbs: *mut xhci_trb,
// private to HCD
    pub next: *mut xhci_segment,
    pub num: c_uint,
    pub dma: dma_addr_t,
// Max packet sized bounce buffer for td-fragmant alignment
    pub bounce_dma: dma_addr_t,
    pub bounce_buf: *mut c_void,
    pub bounce_offs: c_uint,
    pub bounce_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_cancelled_td_status {
    TD_DIRTY = 0,
    TD_HALTED,
    TD_CLEARING_CACHE,
    TD_CLEARING_CACHE_DEFERRED,
    TD_CLEARED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_td {
    pub td_list: list_head,
    pub cancelled_td_list: list_head,
    pub status: c_int,
    pub cancel_status: xhci_cancelled_td_status,
    pub urb: *mut urb,
    pub start_seg: *mut xhci_segment,
    pub start_trb: *mut xhci_trb,
    pub end_seg: *mut xhci_segment,
    pub end_trb: *mut xhci_trb,
    pub bounce_seg: *mut xhci_segment,
// actual_length of the URB has already been set
    pub urb_length_set: bool,
    pub error_mid_td: bool,
}

//
// xHCI command default timeout value in milliseconds.
// USB 3.2 spec, section 9.2.6.1
//
pub const XHCI_CMD_DEFAULT_TIMEOUT: c_int = 5000;
// command descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_cd {
    pub command: *mut xhci_command,
    pub cmd_trb: *mut xhci_trb,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_ring_type {
    TYPE_CTRL = 0,
    TYPE_ISOC,
    TYPE_BULK,
    TYPE_INTR,
    TYPE_STREAM,
    TYPE_COMMAND,
    TYPE_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_ring {
    pub first_seg: *mut xhci_segment,
    pub last_seg: *mut xhci_segment,
    pub enqueue: *mut xhci_trb,
    pub enq_seg: *mut xhci_segment,
    pub dequeue: *mut xhci_trb,
    pub deq_seg: *mut xhci_segment,
    pub td_list: list_head,
//
// Write the cycle state into the TRB cycle field to give ownership of
// the TRB to the host controller (if we are the producer), or to check
// if we own the TRB (if we are the consumer).  See section 4.9.1.
//
    pub cycle_state: u32,
    pub stream_id: c_uint,
    pub num_segs: c_uint,
    pub bounce_buf_len: c_uint,
    pub type: xhci_ring_type,
    pub old_trb_comp_code: u32,
    pub trb_address_map: *mut radix_tree_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_erst_entry {
// 64-bit event ring segment address
    pub seg_addr: __le64,
    pub seg_size: __le32,
// Set to zero
    pub rsvd: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_erst {
    pub entries: *mut xhci_erst_entry,
    pub num_entries: c_uint,
// xhci->event_ring keeps track of segment dma addresses
    pub erst_dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_scratchpad {
    pub sp_array: *mut u64,
    pub sp_dma: dma_addr_t,
    pub sp_buffers: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_priv {
    pub num_tds: c_int,
    pub num_tds_done: c_int,
    pub __counted_by(num_tds): xhci_td td[],
}

// Number of Event Ring segments to allocate, when amount is not specified. (spec allows 32k)
pub const ERST_DEFAULT_SEGS: c_int = 2;
// Poll every 60 seconds
pub const POLL_TIMEOUT: c_int = 60;
// Stop endpoint command timeout (secs) for URB cancellation watchdog timer
pub const XHCI_STOP_EP_CMD_TIMEOUT: c_int = 5;
// XXX: Make these module parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3_save {
    pub command: u32,
    pub dev_nt: u32,
    pub dcbaa_ptr: u64,
    pub config_reg: u32,
}

// Use for lpm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_info {
    pub dev_id: u32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_bus_state {
    pub bus_suspended: c_ulong,
    pub next_statechange: c_ulong,
// Port suspend arrays are indexed by the portnum of the fake roothub
// ports suspend status arrays - max 31 ports for USB2, 15 for USB3
    pub port_c_suspend: u32,
    pub suspended_ports: u32,
    pub port_remote_wakeup: u32,
// which ports have started to resume
    pub resuming_ports: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_interrupter {
    pub event_ring: *mut xhci_ring,
    pub erst: xhci_erst,
    pub ir_set: *mut xhci_intr_reg __iomem,
    pub intr_num: c_uint,
    pub ip_autoclear: bool,
    pub isoc_bei_interval: u32,
// For interrupter registers save and restore over suspend/resume
    pub s3_iman: u32,
    pub s3_imod: u32,
    pub s3_erst_size: u32,
    pub s3_erst_base: u64,
    pub s3_erst_dequeue: u64,
}

//
// It can take up to 20 ms to transition from RExit to U0 on the
// Intel Lynx Point LP xHCI host.
//
pub const XHCI_MAX_REXIT_TIMEOUT_MS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_port_cap {
    pub /: *mut *mut *mut u32 psi; / array of protocol speed ID entries,
    pub psi_count: u8,
    pub psi_uid_count: u8,
    pub maj_rev: u8,
    pub min_rev: u8,
    pub protocol_caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_port {
    pub port_reg: *mut xhci_port_regs __iomem,
    pub hw_portnum: c_int,
    pub hcd_portnum: c_int,
    pub rhub: *mut xhci_hub,
    pub port_cap: *mut xhci_port_cap,
    pub link_inactive:1: c_uint,
    pub connected:1: c_uint,
    pub lpm_incapable:1: c_uint,
    pub resume_timestamp: c_ulong,
    pub rexit_active: bool,
// Slot ID is the index of the device directly connected to the port
    pub slot_id: c_int,
    pub rexit_done: completion,
    pub u3exit_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_hub {
    pub ports: *mut xhci_port,
    pub num_ports: c_uint,
    pub hcd: *mut usb_hcd,
// keep track of bus suspend info
    pub bus_state: xhci_bus_state,
// supported prococol extended capabiliy values
    pub maj_rev: u8,
    pub min_rev: u8,
}

// There is one xhci_hcd structure per controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_hcd {
    pub main_hcd: *mut usb_hcd,
    pub shared_hcd: *mut usb_hcd,
// glue to PCI and HCD framework
    pub cap_regs: *mut xhci_cap_regs __iomem,
    pub op_regs: *mut xhci_op_regs __iomem,
    pub run_regs: *mut xhci_run_regs __iomem,
    pub dba: *mut xhci_doorbell_array __iomem,
// Cached register copies of read-only HC data
    pub hcs_params2: __u32,
    pub hcs_params3: __u32,
    pub hcc_params: __u32,
    pub hcc_params2: __u32,
    pub lock: spinlock_t,
// packed release number
    pub hci_version: u16,
    pub max_interrupters: u16,
    pub max_slots: u8,
    pub max_ports: u8,
// imod_interval in ns (I * 250ns)
    pub imod_interval: u32,
    pub page_size: u32,
    pub dma_mask_bits: c_uint,
// MSI-X/MSI vectors
    pub nvecs: c_int,
// optional clocks
    pub clk: *mut clk,
    pub reg_clk: *mut clk,
// optional reset controller
    pub reset: *mut reset_control,
// data structures
    pub dcbaa: xhci_device_context_array,
    pub interrupters: *mut xhci_interrupter,
    pub cmd_ring: *mut xhci_ring,
    pub cmd_ring_state: c_uint,

    pub cmd_list: list_head,
    pub cmd_ring_reserved_trbs: c_uint,
    pub cmd_timer: delayed_work,
    pub cmd_ring_stop_completion: completion,
    pub current_cmd: *mut xhci_command,
// Scratchpad
    pub scratchpad: *mut xhci_scratchpad,
// slot enabling and address device helpers
// these are not thread safe so use mutex
    pub mutex: mutex,
// Internal mirror of the HW's dcbaa
    pub devs: *mut xhci_virt_device,
// For keeping track of bandwidth domains per roothub.
    pub rh_bw: *mut xhci_root_port_bw_info,
// DMA pools
    pub device_pool: *mut dma_pool,
    pub segment_pool: *mut dma_pool,
    pub small_streams_pool: *mut dma_pool,
    pub port_bw_pool: *mut dma_pool,
    pub medium_streams_pool: *mut dma_pool,
// Host controller watchdog timer structures
    pub xhc_state: c_uint,
    pub run_graceperiod: c_ulong,
    pub s3: s3_save,
// Host controller is dying - not responding to commands. "I'm not dead yet!"
//
// xHC interrupts have been disabled and a watchdog timer will (or has already)
// halt the xHCI host, and complete all URBs with an -ESHUTDOWN code.  Any code
// that sees this status (other than the timer that set it) should stop touching
// hardware immediately.  Interrupt handlers should return immediately when
// they see this status (any time they drop and re-acquire xhci->lock).
// xhci_urb_dequeue() should call usb_hcd_check_unlink_urb() and return without
// putting the TD on the canceled list, etc.
//
// There are no reports of xHCI host controllers that display this issue.
//

    pub quirks: c_ulonglong,

//
// Certain Intel host controllers have a limit to the number of endpoint
// contexts they can handle.  Ideally, they would signal that they can't handle
// anymore endpoint contexts by returning a Resource Error for the Configure
// Endpoint command, but they don't.  Instead they expect software to keep track
// of the number of active endpoints for them, across configure endpoint
// commands, reset device commands, disable slot commands, and address device
// commands.
//

// For controllers with a broken beyond repair streams implementation

// For controller with a broken Port Disable implementation

// Reserved. It was XHCI_RENESAS_FW_QUIRK

    pub num_active_eps: c_uint,
    pub limit_active_eps: c_uint,
    pub hw_ports: *mut xhci_port,
    pub usb2_rhub: xhci_hub,
    pub usb3_rhub: xhci_hub,
// support xHCI 1.0 spec USB2 hardware LPM
    pub hw_lpm_support:1: unsigned,
// Broken Suspend flag for SNPS Suspend resume issue
    pub broken_suspend:1: unsigned,
// Indicates that omitting hcd is supported if root hub has no ports
    pub allow_single_roothub:1: unsigned,
// cached extended protocol port capabilities
    pub port_caps: *mut xhci_port_cap,
    pub num_port_caps: c_uint,
// Compliance Mode Recovery Data
    pub comp_mode_recovery_timer: timer_list,
    pub port_status_u0: u32,
    pub test_mode: u16,
// Compliance Mode Timer Triggered every 2 seconds
pub const COMP_MODE_RCVRY_MSECS: c_int = 2000;
    pub debugfs_root: *mut dentry,
    pub debugfs_slots: *mut dentry,
    pub regset_list: list_head,
    pub dbc: *mut c_void,
// platform-specific data -- must come last
    pub __aligned(sizeof(s64)): unsigned long priv[],
}

// Platform specific overrides to generic XHCI hc_driver ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_driver_overrides {
    pub extra_priv_size: usize,
    pub hcd): *mut *mut int (reset)(struct usb_hcd,
    pub hcd): *mut *mut int (start)(struct usb_hcd,
    pub ep): *mut usb_host_endpoint,
    pub ep): *mut usb_host_endpoint,
    pub ): *mut *mut *mut int (check_bandwidth)(struct usb_hcd , struct usb_device,
    pub ): *mut *mut *mut void (reset_bandwidth)(struct usb_hcd , struct usb_device,
    pub mem_flags): *mut *mut usb_tt tt, gfp_t,
    pub wLength): *mut *mut u16 wIndex, char buf, u16,
}

pub const XHCI_CFC_DELAY: c_int = 10;
// convert between an HCD pointer and the corresponding EHCI_HCD

//
// Registers should always be accessed with double word or quad word accesses.
//
// Some xHCI implementations may support 64-bit address pointers.  Registers
// with 64-bit address pointers should be written to with dword accesses by
// writing the low dword first (ptr[0]), then the high dword (ptr[1]) second.
// xHCI implementations that do not support 64-bit address pointers will ignore
// the high dword, and write order is irrelevant.
//
extern "C" {
    pub fn lo_hi_readq(_arg: regs) -> return;
}
//
// Reportedly, some chapters of v0.95 spec said that Link TRB always has its chain bit set.
// Other chapters and later specs say that it should only be set if the link is inside a TD
// which continues from the end of one segment to the next segment.
//
// Some 0.95 hardware was found to misbehave if any link TRB doesn't have the chain bit set.
//
// 0.96 hardware from AMD and NEC was found to ignore unchained isochronous link TRBs when
// "resynchronizing the pipe" after a Missed Service Error.
//
// xHCI debugging
// xHCI memory management
extern "C" {
    pub fn xhci_mem_cleanup(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_mem_init(xhci: *mut xhci_hcd, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn xhci_free_virt_device(xhci: *mut xhci_hcd, dev: *mut xhci_virt_device, slot_id: c_int);
}
extern "C" {
    pub fn xhci_free_virt_devices_depth_first(xhci: *mut xhci_hcd, slot_id: c_int);
}
extern "C" {
    pub fn xhci_alloc_virt_device(xhci: *mut xhci_hcd, slot_id: c_int, udev: *mut usb_device, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn xhci_setup_addressable_virt_dev(xhci: *mut xhci_hcd, udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn xhci_get_endpoint_index(desc: *mut usb_endpoint_descriptor) -> c_uint;
}
extern "C" {
    pub fn xhci_last_valid_endpoint(added_ctxs: u32) -> c_uint;
}
extern "C" {
    pub fn xhci_endpoint_zero(xhci: *mut xhci_hcd, virt_dev: *mut xhci_virt_device, ep: *mut usb_host_endpoint);
}
extern "C" {
    pub fn xhci_clear_endpoint_bw_info(bw_info: *mut xhci_bw_info);
}
extern "C" {
    pub fn xhci_rh_bw_cleanup(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_ring_free(xhci: *mut xhci_hcd, ring: *mut xhci_ring);
}
extern "C" {
    pub fn xhci_initialize_ring_info(ring: *mut xhci_ring);
}
extern "C" {
    pub fn xhci_ring_init(xhci: *mut xhci_hcd, ring: *mut xhci_ring);
}
extern "C" {
    pub fn xhci_urb_free_priv(urb_priv: *mut urb_priv);
}
// hcd, struct xhci_interrupter *ir);
// xHCI host controller glue
extern "C" {
    pub fn void(: *mut *mut xhci_get_quirks_t)(struct device, : *mut xhci_hcd) -> typedef;
}
extern "C" {
    pub fn xhci_handshake(ptr: *mut void __iomem, mask: u32, done: u32, timeout_us: u64) -> c_int;
}
extern "C" {
    pub fn xhci_quiesce(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_halt(xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_start(xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_reset(xhci: *mut xhci_hcd, timeout_us: u64) -> c_int;
}
extern "C" {
    pub fn xhci_run(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_gen_setup(hcd: *mut usb_hcd, get_quirks: xhci_get_quirks_t) -> c_int;
}
extern "C" {
    pub fn xhci_shutdown(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn xhci_stop(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn xhci_check_bandwidth(hcd: *mut usb_hcd, udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn xhci_reset_bandwidth(hcd: *mut usb_hcd, udev: *mut usb_device);
}
extern "C" {
    pub fn xhci_disable_slot(xhci: *mut xhci_hcd, slot_id: u32) -> c_int;
}
extern "C" {
    pub fn xhci_disable_and_free_slot(xhci: *mut xhci_hcd, slot_id: u32) -> c_int;
}
extern "C" {
    pub fn xhci_ext_cap_init(xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_suspend(xhci: *mut xhci_hcd, do_wakeup: bool) -> c_int;
}
extern "C" {
    pub fn xhci_resume(xhci: *mut xhci_hcd, power_lost: bool, is_auto_resume: bool) -> c_int;
}
extern "C" {
    pub fn xhci_irq(hcd: *mut usb_hcd) -> irqreturn_t;
}
extern "C" {
    pub fn xhci_msi_irq(irq: c_int, hcd: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn xhci_alloc_dev(hcd: *mut usb_hcd, udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn xhci_enable_interrupter(ir: *mut xhci_interrupter) -> c_int;
}
extern "C" {
    pub fn xhci_disable_interrupter(xhci: *mut xhci_hcd, ir: *mut xhci_interrupter) -> c_int;
}
// xHCI ring, segment, TRB, and TD functions
extern "C" {
    pub fn xhci_trb_virt_to_dma(seg: *mut xhci_segment, trb: *mut xhci_trb) -> dma_addr_t;
}
extern "C" {
    pub fn xhci_is_vendor_info_code(xhci: *mut xhci_hcd, trb_comp_code: c_uint) -> c_int;
}
extern "C" {
    pub fn xhci_ring_cmd_db(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_handle_command_timeout(work: *mut work_struct);
}
extern "C" {
    pub fn xhci_cleanup_command_queue(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn inc_deq(xhci: *mut xhci_hcd, ring: *mut xhci_ring);
}
extern "C" {
    pub fn count_trbs(addr: u64, len: u64) -> c_uint;
}
extern "C" {
    pub fn xhci_num_trbs_free(ring: *mut xhci_ring) -> c_uint;
}
extern "C" {
    pub fn xhci_process_cancelled_tds(xhci: *mut xhci_hcd, ep: *mut xhci_virt_ep);
}
extern "C" {
    pub fn xhci_add_interrupter(xhci: *mut xhci_hcd, intr_num: c_uint);
}
extern "C" {
    pub fn xhci_portsc_writel(port: *mut xhci_port, val: u32);
}
extern "C" {
    pub fn xhci_portsc_readl(port: *mut xhci_port) -> u32;
}
// xHCI roothub code
extern "C" {
    pub fn xhci_hub_status_data(hcd: *mut usb_hcd, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn xhci_find_raw_port_number(hcd: *mut usb_hcd, port1: c_int) -> c_int;
}
extern "C" {
    pub fn xhci_hc_died(xhci: *mut xhci_hcd);
}

extern "C" {
    pub fn xhci_bus_suspend(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_bus_resume(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_get_resuming_ports(hcd: *mut usb_hcd) -> c_ulong;
}

extern "C" {
    pub fn xhci_port_state_to_neutral(state: u32) -> u32;
}
extern "C" {
    pub fn xhci_ring_device(xhci: *mut xhci_hcd, slot_id: c_int);
}
// xHCI contexts
//
// TODO: As per spec Isochronous IDT transmissions are supported. We bypass
// them anyways as we where unable to find a device that matches the
// constraints.
//
// RO/ROS: Read-only
// RWS; writing 1 sets the bit, writing 0 clears the bit.
// RW; writing 1 sets the bit, writing 0 clears the bit
// RW1S; writing 1 sets the bit, writing 0 has no effect
// RW1CS; writing 1 clears the bit, writing 0 has no effect.
