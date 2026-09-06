//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/cdns3/cdnsp-gadget.h
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
// Cadence CDNSP DRD Driver.
//
// Copyright (C) 2020 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//
// Code based on Linux XHCI driver.
// Origin: Copyright (C) 2008 Intel Corp.
//

// Max number slots - only 1 is allowed.
pub const CDNSP_DEV_MAX_SLOTS: c_int = 1;
pub const CDNSP_EP0_SETUP_SIZE: c_int = 512;
// One control and 15 for in and 15 for out endpoints.
pub const CDNSP_ENDPOINTS_NUM: c_int = 31;
// Best Effort Service Latency.
pub const CDNSP_DEFAULT_BESL: c_int = 0;
// Device Controller command default timeout value in us

// Up to 16 ms to halt an device controller

pub const CDNSP_CTX_SIZE: c_int = 2112;
//
// Controller register interface.
//
// struct cdnsp_cap_regs - CDNSP Registers.
// @hc_capbase:	Length of the capabilities register and controller
// version number
// @hcs_params1: HCSPARAMS1 - Structural Parameters 1
// @hcs_params2: HCSPARAMS2 - Structural Parameters 2
// @hcs_params3: HCSPARAMS3 - Structural Parameters 3
// @hcc_params: HCCPARAMS - Capability Parameters
// @db_off: DBOFF - Doorbell array offset
// @run_regs_off: RTSOFF - Runtime register space offset
// @hcc_params2: HCCPARAMS2 Capability Parameters 2,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_cap_regs {
    pub hc_capbase: __le32,
    pub hcs_params1: __le32,
    pub hcs_params2: __le32,
    pub hcs_params3: __le32,
    pub hcc_params: __le32,
    pub db_off: __le32,
    pub run_regs_off: __le32,
    pub hcc_params2: __le32,
// Reserved up to (CAPLENGTH - 0x1C)
}

// hc_capbase bitmasks.
// bits 7:0 - how long is the Capabilities register.

// bits 31:16

// HCSPARAMS1 - hcs_params1 - bitmasks
// bits 0:7, Max Device Endpoints

// HCCPARAMS offset from PCI base address
pub const HCC_PARAMS_OFFSET: c_uint = 0x10;
// HCCPARAMS - hcc_params - bitmasks
// 1: device controller can use 64-bit address pointers.

// 1: device controller uses 64-byte Device Context structures.

// Max size for Primary Stream Arrays - 2^(n+1), where n is bits 12:15.

// Extended Capabilities pointer from PCI base.

// db_off bitmask - bits 0:1 reserved.

// run_regs_off bitmask - bits 0:4 reserved.

//
// struct cdnsp_op_regs - Device Controller Operational Registers.
// @command: USBCMD - Controller command register.
// @status: USBSTS - Controller status register.
// @page_size: This indicates the page size that the device controller supports.
// If bit n is set, the controller supports a page size of 2^(n+12),
// up to a 128MB page size. 4K is the minimum page size.
// @dnctrl: DNCTRL - Device notification control register.
// @cmd_ring: CRP - 64-bit Command Ring Pointer.
// @dcbaa_ptr: DCBAAP - 64-bit Device Context Base Address Array Pointer.
// @config_reg: CONFIG - Configure Register
// @port_reg_base: PORTSCn - base address for Port Status and Control
// Each port has a Port Status and Control register,
// followed by a Port Power Management Status and Control
// register, a Port Link Info register, and a reserved
// register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_op_regs {
    pub command: __le32,
    pub status: __le32,
    pub page_size: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub dnctrl: __le32,
    pub cmd_ring: __le64,
// rsvd: offset 0x20-2F.
    pub reserved3: [__le32; 4],
    pub dcbaa_ptr: __le64,
    pub config_reg: __le32,
// rsvd: offset 0x3C-3FF.
    pub reserved4: [__le32; 241],
// port 1 registers, which serve as a base address for other ports.
    pub port_reg_base: __le32,
}

// Number of registers per port.
pub const NUM_PORT_REGS: c_int = 4;
//
// struct cdnsp_port_regs - Port Registers.
// @portsc: PORTSC - Port Status and Control Register.
// @portpmsc: PORTPMSC - Port Power Managements Status and Control Register.
// @portli: PORTLI - Port Link Info register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_port_regs {
    pub portsc: __le32,
    pub portpmsc: __le32,
    pub portli: __le32,
    pub reserved: __le32,
}

//
// These bits are Read Only (RO) and should be saved and written to the
// registers: 0 (connect status) and  10:13 (port speed).
// These bits are also sticky - meaning they're in the AUX well and they aren't
// changed by a hot and warm.
//

//
// These bits are RW; writing a 0 clears the bit, writing a 1 sets the bit:
// bits 5:8 (link state), 25:26  ("wake on" enable state)
//

//
// These bits are RW; writing a 1 clears the bit, writing a 0 has no effect:
// bits 1 (port enable/disable), 17  ( connect changed),
// 21 (port reset changed) , 22 (Port Link State Change),
//

// USBCMD - USB command - bitmasks.
// Run/Stop, controller execution - do not write unless controller is halted.

//
// Reset device controller - resets internal controller state machine and all
// registers (except PCI config regs).
//

// Event Interrupt Enable - a '1' allows interrupts from the controller.

//
// Device System Error Interrupt Enable - get out-of-band signal for
// controller errors.
//

// device controller save/restore state.

//
// Enable Wrap Event - '1' means device controller generates an event
// when MFINDEX wraps.
//

// 1: device enabled

// bits 18:31 are reserved (and should be preserved on writes).
// Command register values to disable interrupts.

// USBSTS - USB status - bitmasks
// controller not running - set to 1 when run/stop bit is cleared.

//
// serious error, e.g. PCI parity error. The controller will clear
// the run/stop bit.
//

// event interrupt - clear this prior to clearing any IP flags in IR set.

// port change detect

// save state status - '1' means device controller is saving state.

// restore state status - '1' means controllers is restoring state.

// 1: save or restore error

// 1: device Not Ready to accept doorbell or op reg writes after reset.

// 1: internal Device Controller Error.

// CRCR - Command Ring Control Register - cmd_ring bitmasks.
// bit 0 is the command ring cycle state.

// stop ring immediately - abort the currently executing command.

//
// Command Ring Busy.
// Set when Doorbell register is written with DB for command and cleared when
// the controller reached end of CR.
//

// 1: command ring is running

// Command Ring pointer - bit mask for the lower 32 bits.

// CONFIG - Configure Register - config_reg bitmasks.
// bits 0:7 - maximum number of device slots enabled.

// bit 8: U3 Entry Enabled, assert PLC when controller enters U3.

// PORTSC - Port Status and Control Register - port_reg_base bitmasks
// 1: device connected.

// 1: port enabled.

// 1: port reset signaling asserted.

//
// Port Link State - bits 5:8
// A read gives the current link PM state of the port,
// a write with Link State Write Strobe sets the link state.
//

// 1: port has power.

//
// bits 10:13 indicate device speed:
// 0 - undefined speed - port hasn't be initialized by a reset yet
// 1 - full speed
// 2 - Reserved (Low Speed not supported
// 3 - high speed
// 4 - super speed
// 5 - super speed
// 6-15 reserved
//

// Port Link State Write Strobe - set this when changing link state

// 1: connect status change

// 1: warm reset for a USB 3.0 device is done.

// 1: reset change - 1 to 0 transition of PORT_RESET

//
// port link status change - set on some port link state transitions:
// Transition			Reason
// ----------------------------------------------------------------------------
// - U3 to Resume		Wakeup signaling from a device
// - Resume to Recovery to U0	USB 3.0 device resume
// - Resume to U0		USB 2.0 device resume
// - U3 to Recovery to U0	Software resume of USB 3.0 device complete
// - U3 to U0			Software resume of USB 2.0 device complete
// - U2 to U0			L1 resume of USB 2.1 device complete
// - U0 to U0			L1 entry rejection by USB 2.1 device
// - U0 to disabled		L1 entry error with USB 2.1 device
// - Any state to inactive	Error on USB 3.0 port
//

// Port configure error change - port failed to configure its link partner.

// Wake on connect (enable).

// Wake on disconnect (enable).

// Indicates if Warm Reset is being received.

// PORTPMSCUSB3 - Port Power Management Status and Control - bitmasks.
// Enables U1 entry.

// Enables U2 entry .

// PORTPMSCUSB2 - Port Power Management Status and Control - bitmasks.

// Remote Wake Enable.

// Best Effort Service Latency (BESL).

// Hardware LPM Enable (HLE).

// Received Best Effort Service Latency (BESL).

// Port Test Control.

//
// struct cdnsp_intr_reg - Interrupt Register Set.
// @irq_pending: IMAN - Interrupt Management Register. Used to enable
// interrupts and check for pending interrupts.
// @irq_control: IMOD - Interrupt Moderation Register.
// Used to throttle interrupts.
// @erst_size: Number of segments in the Event Ring Segment Table (ERST).
// @erst_base: ERST base address.
// @erst_dequeue: Event ring dequeue pointer.
//
// Each interrupter (defined by a MSI-X vector) has an event ring and an Event
// Ring Segment Table (ERST) associated with it. The event ring is comprised of
// multiple segments of the same size. The controller places events on the ring
// and "updates the Cycle bit in the TRBs to indicate to software the current
// position of the Enqueue Pointer." The driver processes those events and
// updates the dequeue pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_intr_reg {
    pub irq_pending: __le32,
    pub irq_control: __le32,
    pub erst_size: __le32,
    pub rsvd: __le32,
    pub erst_base: __le64,
    pub erst_dequeue: __le64,
}

// IMAN - Interrupt Management Register - irq_pending bitmasks l.

// bits 2:31 need to be preserved

// IMOD - Interrupter Moderation Register - irq_control bitmasks.
//
// Minimum interval between interrupts (in 250ns intervals). The interval
// between interrupts will be longer if there are no events on the event ring.
// Default is 4000 (1 ms).
//

// Counter used to count down the time to the next interrupt - HW use only

pub const IMOD_DEFAULT_INTERVAL: c_int = 0;
// erst_size bitmasks.
// Preserve bits 16:31 of erst_size.

// erst_dequeue bitmasks.
//
// Dequeue ERST Segment Index (DESI) - Segment number (or alias)
// where the current dequeue pointer lies. This is an optional HW hint.
//

// Event Handler Busy (EHB) - is the event ring scheduled to be serviced.

//
// struct cdnsp_run_regs
// @microframe_index: MFINDEX - current microframe number.
// @ir_set: Array of Interrupter registers.
//
// Device Controller Runtime Registers:
// "Software should read and write these registers using only Dword (32 bit)
// or larger accesses"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_run_regs {
    pub microframe_index: __le32,
    pub rsvd: [__le32; 7],
    pub ir_set: [cdnsp_intr_reg; 128],
}

//
// USB2.0 Port Peripheral Configuration Registers.
// @ext_cap: Header register for Extended Capability.
// @port_reg1: Timer Configuration Register.
// @port_reg2: Timer Configuration Register.
// @port_reg3: Timer Configuration Register.
// @port_reg4: Timer Configuration Register.
// @port_reg5: Timer Configuration Register.
// @port_reg6: Chicken bits for USB20PPP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_20port_cap {
    pub ext_cap: __le32,
    pub port_reg1: __le32,
    pub port_reg2: __le32,
    pub port_reg3: __le32,
    pub port_reg4: __le32,
    pub port_reg5: __le32,
    pub port_reg6: __le32,
}

// Extended capability register fields

// Extended capability IDs - ID 0 reserved
pub const EXT_CAPS_PROTOCOL: c_int = 2;
// USB 2.0 Port Peripheral Configuration Extended Capability
pub const EXT_CAP_CFG_DEV_20PORT_CAP_ID: c_uint = 0xC1;
//
// Setting this bit to '1' enables automatic wakeup from L1 state on transfer
// TRB prepared when USBSSP operates in USB2.0 mode.
//

//
// Setting this bit to '1' forces Full Speed when USBSSP operates in USB2.0
// mode (disables High Speed).
//

//
// USB3.x Port Peripheral Configuration Registers.
// @ext_cap: Header register for Extended Capability.
// @mode_addr: Miscellaneous 3xPORT operation mode configuration register.
// @mode_2: 3x Port Control Register 2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_3xport_cap {
    pub ext_cap: __le32,
    pub mode_addr: __le32,
    pub reserved: [__le32; 52],
    pub mode_2: __le32,
}

// Extended Capability Header for 3XPort Configuration Registers.
pub const D_XEC_CFG_3XPORT_CAP: c_uint = 0xC0;

// Revision Extended Capability ID
pub const RTL_REV_CAP: c_uint = 0xC4;

pub const CDNSP_VER_1: c_uint = 0x00000000;
pub const CDNSP_VER_2: c_uint = 0x10000000;

//
// struct cdnsp_rev_cap - controller capabilities.
// @ext_cap: Header for RTL Revision Extended Capability.
// @rtl_revision: RTL revision.
// @rx_buff_size: Rx buffer sizes.
// @tx_buff_size: Tx buffer sizes.
// @ep_supported: Supported endpoints.
// @ctrl_revision: Controller revision ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_rev_cap {
    pub ext_cap: __le32,
    pub rtl_revision: __le32,
    pub rx_buff_size: __le32,
    pub tx_buff_size: __le32,
    pub ep_supported: __le32,
    pub ctrl_revision: __le32,
}

// USB2.0 Port Peripheral Configuration Registers.
pub const D_XEC_PRE_REGS_CAP: c_uint = 0xC8;
pub const REG_CHICKEN_BITS_2_OFFSET: c_uint = 0x48;

pub const REG_CHICKEN_BITS_3_OFFSET: c_uint = 0x4C;

// XBUF Extended Capability ID.
pub const XBUF_CAP_ID: c_uint = 0xCB;
pub const XBUF_RX_TAG_MASK_0_OFFSET: c_uint = 0x1C;
pub const XBUF_RX_TAG_MASK_1_OFFSET: c_uint = 0x24;
pub const XBUF_TX_CMD_OFFSET: c_uint = 0x2C;
//
// struct cdnsp_doorbell_array.
// @cmd_db: Command ring doorbell register.
// @ep_db: Endpoint ring doorbell register.
// Bits 0 - 7: Endpoint target.
// Bits 8 - 15: RsvdZ.
// Bits 16 - 31: Stream ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_doorbell_array {
    pub cmd_db: __le32,
    pub ep_db: __le32,
}

pub const DB_VALUE_CMD: c_uint = 0x00000000;
//
// struct cdnsp_container_ctx.
// @type: Type of context. Used to calculated offsets to contained contexts.
// @size: Size of the context data.
// @ctx_size: context data structure size - 64 or 32 bits.
// @dma: dma address of the bytes.
// @bytes: The raw context data given to HW.
//
// Represents either a Device or Input context. Holds a pointer to the raw
// memory used for the context (bytes) and dma address of it (dma).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_container_ctx {
    pub type: c_uint,
pub const CDNSP_CTX_TYPE_DEVICE: c_uint = 0x1;
pub const CDNSP_CTX_TYPE_INPUT: c_uint = 0x2;
    pub size: c_int,
    pub ctx_size: c_int,
    pub dma: dma_addr_t,
    pub bytes: *mut u8,
}

//
// struct cdnsp_slot_ctx
// @dev_info: Device speed, and last valid endpoint.
// @dev_port: Device port number that is needed to access the USB device.
// @int_target: Interrupter target number.
// @dev_state: Slot state and device address.
//
// Slot Context - This assumes the controller uses 32-byte context
// structures. If the controller uses 64-byte contexts, there is an additional
// 32 bytes reserved at the end of the slot context for controller internal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_slot_ctx {
    pub dev_info: __le32,
    pub dev_port: __le32,
    pub int_target: __le32,
    pub dev_state: __le32,
// offset 0x10 to 0x1f reserved for controller internal use.
    pub reserved: [__le32; 4],
}

// Bits 20:23 in the Slot Context are the speed for the device.

// dev_info bitmasks.
// Device speed - values defined by PORTSC Device Speed field - 20:23.

// Index of the last valid endpoint context in this device context - 27:31.

// dev_port bitmasks
// Device port number that is needed to access the USB device.

// dev_state bitmasks
// USB device address - assigned by the controller.

// Slot state

pub const SLOT_STATE_DISABLED: c_int = 0;

pub const SLOT_STATE_DEFAULT: c_int = 1;
pub const SLOT_STATE_ADDRESSED: c_int = 2;
pub const SLOT_STATE_CONFIGURED: c_int = 3;
//
// struct cdnsp_ep_ctx.
// @ep_info: Endpoint state, streams, mult, and interval information.
// @ep_info2: Information on endpoint type, max packet size, max burst size,
// error count, and whether the controller will force an event for
// all transactions.
// @deq: 64-bit ring dequeue pointer address. If the endpoint only
// defines one stream, this points to the endpoint transfer ring.
// Otherwise, it points to a stream context array, which has a
// ring pointer for each flow.
// @tx_info: Average TRB lengths for the endpoint ring and
// max payload within an Endpoint Service Interval Time (ESIT).
//
// Endpoint Context - This assumes the controller uses 32-byte context
// structures. If the controller uses 64-byte contexts, there is an additional
// 32 bytes reserved at the end of the endpoint context for controller internal
// use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_ep_ctx {
    pub ep_info: __le32,
    pub ep_info2: __le32,
    pub deq: __le64,
    pub tx_info: __le32,
// offset 0x14 - 0x1f reserved for controller internal use.
    pub reserved: [__le32; 3],
}

// ep_info bitmasks.
//
// Endpoint State - bits 0:2:
// 0 - disabled
// 1 - running
// 2 - halted due to halt condition
// 3 - stopped
// 4 - TRB error
// 5-7 - reserved
//

pub const EP_STATE_DISABLED: c_int = 0;
pub const EP_STATE_RUNNING: c_int = 1;
pub const EP_STATE_HALTED: c_int = 2;
pub const EP_STATE_STOPPED: c_int = 3;
pub const EP_STATE_ERROR: c_int = 4;

// Mult - Max number of burst within an interval, in EP companion desc.

// bits 10:14 are Max Primary Streams.
// bit 15 is Linear Stream Array.
// Interval - period between requests to an endpoint - 125u increments.

// Endpoint is set up with a Linear Stream Array (vs. Secondary Stream Array)

// ep_info2 bitmasks

pub const ISOC_OUT_EP: c_int = 1;
pub const BULK_OUT_EP: c_int = 2;
pub const INT_OUT_EP: c_int = 3;
pub const CTRL_EP: c_int = 4;
pub const ISOC_IN_EP: c_int = 5;
pub const BULK_IN_EP: c_int = 6;
pub const INT_IN_EP: c_int = 7;
// bit 6 reserved.
// bit 7 is Device Initiate Disable - for disabling stream selection.

// tx_info bitmasks.

// deq bitmasks.

//
// struct cdnsp_input_control_context
// Input control context;
//
// @drop_context: Set the bit of the endpoint context you want to disable.
// @add_context: Set the bit of the endpoint context you want to enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_input_control_ctx {
    pub drop_flags: __le32,
    pub add_flags: __le32,
    pub rsvd2: [__le32; 6],
}

//
// Represents everything that is needed to issue a command on the command ring.
//
// @in_ctx: Pointer to input context structure.
// @status: Command Completion Code for last command.
// @command_trb: Pointer to command TRB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_command {
// Input context for changing device state.
    pub in_ctx: *mut cdnsp_container_ctx,
    pub status: u32,
    pub command_trb: *mut cdnsp_trb,
}

//
// Stream context structure.
//
// @stream_ring: 64-bit stream ring address, cycle state, and stream type.
// @reserved: offset 0x14 - 0x1f reserved for controller internal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_stream_ctx {
    pub stream_ring: __le64,
    pub reserved: [__le32; 2],
}

// Stream Context Types - bits 3:1 of stream ctx deq ptr.

// Secondary stream array type, dequeue pointer is to a transfer ring.
pub const SCT_SEC_TR: c_int = 0;
// Primary stream array type, dequeue pointer is to a transfer ring.
pub const SCT_PRI_TR: c_int = 1;
//
// struct cdnsp_stream_info: Representing everything that is needed to
// supports stream capable endpoints.
// @stream_rings: Array of pointers containing Transfer rings for all
// supported streams.
// @num_streams: Number of streams, including stream 0.
// @stream_ctx_array: The stream context array may be bigger than the number
// of streams the driver asked for.
// @num_stream_ctxs: Number of streams.
// @ctx_array_dma: Dma address of Context Stream Array.
// @trb_address_map: For mapping physical TRB addresses to segments in
// stream rings.
// @td_count: Number of TDs associated with endpoint.
// @first_prime_det: First PRIME packet detected.
// @drbls_count: Number of allowed doorbells.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_stream_info {
    pub stream_rings: *mut cdnsp_ring,
    pub num_streams: c_uint,
    pub stream_ctx_array: *mut cdnsp_stream_ctx,
    pub num_stream_ctxs: c_uint,
    pub ctx_array_dma: dma_addr_t,
    pub trb_address_map: radix_tree_root,
    pub td_count: c_int,
    pub first_prime_det: u8,
pub const STREAM_DRBL_FIFO_DEPTH: c_int = 2;
    pub drbls_count: u8,
}

pub const STREAM_LOG_STREAMS: c_int = 4;

//
// struct cdnsp_ep - extended device side representation of USB endpoint.
// @endpoint: usb endpoint
// @pending_req_list: List of requests queuing on transfer ring.
// @pdev: Device associated with this endpoint.
// @number: Endpoint number (1 - 15).
// idx: The device context index (DCI).
// interval: Interval between packets used for ISOC endpoint.
// @name: A human readable name e.g. ep1out.
// @direction: Endpoint direction.
// @buffering: Number of on-chip buffers related to endpoint.
// @buffering_period; Number of on-chip buffers related to periodic endpoint.
// @in_ctx: Pointer to input endpoint context structure.
// @out_ctx: Pointer to output endpoint context structure.
// @ring: Pointer to transfer ring.
// @stream_info: Hold stream information.
// @ep_state: Current state of endpoint.
// @skip: Sometimes the controller can not process isochronous endpoint ring
// quickly enough, and it will miss some isoc tds on the ring and
// generate Missed Service Error Event.
// Set skip flag when receive a Missed Service Error Event and
// process the missed tds on the endpoint ring.
// @wa1_nop_trb: hold pointer to NOP trb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_ep {
    pub endpoint: usb_ep,
    pub pending_list: list_head,
    pub pdev: *mut cdnsp_device,
    pub number: u8,
    pub idx: u8,
    pub interval: u32,
    pub name: [c_char; 20],
    pub direction: u8,
    pub buffering: u8,
    pub buffering_period: u8,
    pub in_ctx: *mut cdnsp_ep_ctx,
    pub out_ctx: *mut cdnsp_ep_ctx,
    pub ring: *mut cdnsp_ring,
    pub stream_info: cdnsp_stream_info,
    pub ep_state: c_uint,

    pub skip: bool,
    pub wa1_nop_trb: *mut cdnsp_trb,
}

//
// struct cdnsp_device_context_array
// @dev_context_ptr: Array of 64-bit DMA addresses for device contexts.
// @dma: DMA address for device contexts structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_device_context_array {
    pub 1]: __le64 dev_context_ptrs[CDNSP_DEV_MAX_SLOTS +,
    pub dma: dma_addr_t,
}

//
// struct cdnsp_transfer_event.
// @buffer: 64-bit buffer address, or immediate data.
// @transfer_len: Data length transferred.
// @flags: Field is interpreted differently based on the type of TRB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_transfer_event {
    pub buffer: __le64,
    pub transfer_len: __le32,
    pub flags: __le32,
}

// Invalidate event after disabling endpoint.
pub const TRB_EVENT_INVALIDATE: c_int = 8;
// Transfer event TRB length bit mask.
// bits 0:23

// Completion Code - only applicable for some types of TRBs

pub const COMP_INVALID: c_int = 0;
pub const COMP_SUCCESS: c_int = 1;
pub const COMP_DATA_BUFFER_ERROR: c_int = 2;
pub const COMP_BABBLE_DETECTED_ERROR: c_int = 3;
pub const COMP_TRB_ERROR: c_int = 5;
pub const COMP_RESOURCE_ERROR: c_int = 7;
pub const COMP_NO_SLOTS_AVAILABLE_ERROR: c_int = 9;
pub const COMP_INVALID_STREAM_TYPE_ERROR: c_int = 10;
pub const COMP_SLOT_NOT_ENABLED_ERROR: c_int = 11;
pub const COMP_ENDPOINT_NOT_ENABLED_ERROR: c_int = 12;
pub const COMP_SHORT_PACKET: c_int = 13;
pub const COMP_RING_UNDERRUN: c_int = 14;
pub const COMP_RING_OVERRUN: c_int = 15;
pub const COMP_VF_EVENT_RING_FULL_ERROR: c_int = 16;
pub const COMP_PARAMETER_ERROR: c_int = 17;
pub const COMP_CONTEXT_STATE_ERROR: c_int = 19;
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
// Transfer Event NRDY bit fields

pub const STREAM_PRIME_ACK: c_uint = 0xFFFE;
pub const STREAM_REJECTED: c_uint = 0xFFFF;
// Transfer Event bit fields

//
// struct cdnsp_link_trb
// @segment_ptr: 64-bit segment pointer.
// @intr_target: Interrupter target.
// @control: Flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_link_trb {
    pub segment_ptr: __le64,
    pub intr_target: __le32,
    pub control: __le32,
}

// control bitfields

//
// struct cdnsp_event_cmd - Command completion event TRB.
// cmd_trb: Pointer to command TRB, or the value passed by the event data trb
// status: Command completion parameters and error code.
// flags: Flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_event_cmd {
    pub cmd_trb: __le64,
    pub status: __le32,
    pub flags: __le32,
}

// flags bitmasks
// Address device - disable SetAddress.

// Configure Endpoint - Deconfigure.

// Force Header

pub const TRB_FH_TR_PACKET: c_uint = 0x4;

pub const TRB_FH_TR_PACKET_DEV_NOT: c_uint = 0x6;

pub const TRB_FH_TR_PACKET_FUNCTION_WAKE: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdnsp_setup_dev {
    SETUP_CONTEXT_ONLY,
    SETUP_CONTEXT_ADDRESS,
}

// bits 24:31 are the slot ID.

// Stop Endpoint TRB - ep_index to endpoint ID for this TRB.

pub const LAST_EP_INDEX: c_int = 30;
// Set TR Dequeue Pointer command TRB fields.

//
// Halt Endpoint Command TRB field.
// The ESP bit only exists in the SSP2 controller.
//

// Link TRB specific fields.

// Port Status Change Event TRB fields.
// Port ID - bits 31:24.

// Normal TRB fields.
// transfer_len bitmasks - bits 0:16.

// TD Size, packets remaining in this TD, bits 21:17 (5 bits, so max 31).

//
// Controller uses the TD_SIZE field for TBC if Extended TBC
// is enabled (ETE).
//

// Interrupter Target - which MSI-X vector to target the completion event at.

//
// Total burst count field, Rsvdz on controller with Extended TBC
// enabled (ETE).
//

// Cycle bit - indicates TRB ownership by driver or driver.

//
// Force next event data TRB to be evaluated before task switch.
// Used to pass OS data back after a TD completes.
//

// Interrupt on short packet.

// Set PCIe no snoop attribute.

// Chain multiple TRBs into a TD.

// Interrupt on completion.

// The buffer pointer contains immediate data.

// 0 - NRDY during data stage, 1 - NRDY during status stage (only control).

// Block Event Interrupt.

// Control transfer TRB specific fields.

// TRB bit mask in Data Stage TRB

pub const TRB_SETUP_SPEEDID_USB3: c_uint = 0x1;
pub const TRB_SETUP_SPEEDID_USB2: c_uint = 0x0;

pub const TRB_SETUPSTAT_ACK: c_uint = 0x1;
pub const TRB_SETUPSTAT_STALL: c_uint = 0x0;

// Isochronous TRB specific fields

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_generic_trb {
    pub field: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cdnsp_trb {
    pub link: cdnsp_link_trb,
    pub trans_event: cdnsp_transfer_event,
    pub event_cmd: cdnsp_event_cmd,
    pub generic: cdnsp_generic_trb,
}

// TRB bit mask.

// TRB type IDs.
// bulk, interrupt, isoc scatter/gather, and control data stage.
pub const TRB_NORMAL: c_int = 1;
// Setup Stage for control transfers.
pub const TRB_SETUP: c_int = 2;
// Data Stage for control transfers.
pub const TRB_DATA: c_int = 3;
// Status Stage for control transfers.
pub const TRB_STATUS: c_int = 4;
// ISOC transfers.
pub const TRB_ISOC: c_int = 5;
// TRB for linking ring segments.
pub const TRB_LINK: c_int = 6;
pub const TRB_EVENT_DATA: c_int = 7;
// Transfer Ring No-op (not for the command ring).
pub const TRB_TR_NOOP: c_int = 8;
// Command TRBs
// Enable Slot Command.
pub const TRB_ENABLE_SLOT: c_int = 9;
// Disable Slot Command.
pub const TRB_DISABLE_SLOT: c_int = 10;
// Address Device Command.
pub const TRB_ADDR_DEV: c_int = 11;
// Configure Endpoint Command.
pub const TRB_CONFIG_EP: c_int = 12;
// Evaluate Context Command.
pub const TRB_EVAL_CONTEXT: c_int = 13;
// Reset Endpoint Command.
pub const TRB_RESET_EP: c_int = 14;
// Stop Transfer Ring Command.
pub const TRB_STOP_RING: c_int = 15;
// Set Transfer Ring Dequeue Pointer Command.
pub const TRB_SET_DEQ: c_int = 16;
// Reset Device Command.
pub const TRB_RESET_DEV: c_int = 17;
// Force Event Command (opt).
pub const TRB_FORCE_EVENT: c_int = 18;
// Force Header Command - generate a transaction or link management packet.
pub const TRB_FORCE_HEADER: c_int = 22;
// No-op Command - not for transfer rings.
pub const TRB_CMD_NOOP: c_int = 23;
// TRB IDs 24-31 reserved.
// Event TRBS.
// Transfer Event.
pub const TRB_TRANSFER: c_int = 32;
// Command Completion Event.
pub const TRB_COMPLETION: c_int = 33;
// Port Status Change Event.
pub const TRB_PORT_STATUS: c_int = 34;
// Device Controller Event.
pub const TRB_HC_EVENT: c_int = 37;
// MFINDEX Wrap Event - microframe counter wrapped.
pub const TRB_MFINDEX_WRAP: c_int = 39;
// TRB IDs 40-47 reserved.
// Endpoint Not Ready Event.
pub const TRB_ENDPOINT_NRDY: c_int = 48;
// TRB IDs 49-53 reserved.
// Halt Endpoint Command.
pub const TRB_HALT_ENDPOINT: c_int = 54;
// Doorbell Overflow Event.
pub const TRB_DRB_OVERFLOW: c_int = 57;

//
// TRBS_PER_SEGMENT must be a multiple of 4.
// The command ring is 64-byte aligned, so it must also be greater than 16.
//
pub const TRBS_PER_SEGMENT: c_int = 256;
pub const TRBS_PER_EVENT_SEGMENT: c_int = 256;
pub const TRBS_PER_EV_DEQ_UPDATE: c_int = 100;

// TRB buffer pointers can't cross 64KB boundaries.
pub const TRB_MAX_BUFF_SHIFT: c_int = 16;

// How much data is left before the 64KB boundary?

//
// struct cdnsp_segment - segment related data.
// @trbs: Array of Transfer Request Blocks.
// @next: Pointer to the next segment.
// @dma: DMA address of current segment.
// @bounce_dma: Bounce  buffer DMA address .
// @bounce_buf: Bounce buffer virtual address.
// bounce_offs: Bounce buffer offset.
// bounce_len: Bounce buffer length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_segment {
    pub trbs: *mut cdnsp_trb,
    pub next: *mut cdnsp_segment,
    pub dma: dma_addr_t,
// Max packet sized bounce buffer for td-fragmant alignment
    pub bounce_dma: dma_addr_t,
    pub bounce_buf: *mut c_void,
    pub bounce_offs: c_uint,
    pub bounce_len: c_uint,
}

//
// struct cdnsp_td - Transfer Descriptor object.
// @td_list: Used for binding TD with ep_ring->td_list.
// @preq: Request associated with this TD
// @start_seg: Segment containing the first_trb in TD.
// @first_trb: First TRB for this TD.
// @last_trb: Last TRB related with TD.
// @bounce_seg: Bounce segment for this TD.
// @request_length_set: actual_length of the request has already been set.
// @drbl - TD has been added to HW scheduler - only for stream capable
// endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_td {
    pub td_list: list_head,
    pub preq: *mut cdnsp_request,
    pub start_seg: *mut cdnsp_segment,
    pub first_trb: *mut cdnsp_trb,
    pub last_trb: *mut cdnsp_trb,
    pub bounce_seg: *mut cdnsp_segment,
    pub request_length_set: bool,
    pub drbl: bool,
}

//
// struct cdnsp_dequeue_state - New dequeue pointer for Transfer Ring.
// @new_deq_seg: New dequeue segment.
// @new_deq_ptr: New dequeue pointer.
// @new_cycle_state: New cycle state.
// @stream_id: stream id for which new dequeue pointer has been selected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_dequeue_state {
    pub new_deq_seg: *mut cdnsp_segment,
    pub new_deq_ptr: *mut cdnsp_trb,
    pub new_cycle_state: c_int,
    pub stream_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdnsp_ring_type {
    TYPE_CTRL = 0,
    TYPE_ISOC,
    TYPE_BULK,
    TYPE_INTR,
    TYPE_STREAM,
    TYPE_COMMAND,
    TYPE_EVENT,
}

//
// struct cdnsp_ring - information describing transfer, command or event ring.
// @first_seg: First segment on transfer ring.
// @last_seg: Last segment on transfer ring.
// @enqueue: SW enqueue pointer address.
// @enq_seg: SW enqueue segment address.
// @dequeue: SW dequeue pointer address.
// @deq_seg: SW dequeue segment address.
// @td_list: transfer descriptor list associated with this ring.
// @cycle_state: Current cycle bit. Write the cycle state into the TRB cycle
// field to give ownership of the TRB to the device controller
// (if we are the producer) or to check if we own the TRB
// (if we are the consumer).
// @stream_id: Stream id
// @stream_active: Stream is active - PRIME packet has been detected.
// @stream_rejected: This ring has been rejected by host.
// @num_tds: Number of TDs associated with ring.
// @num_segs: Number of segments.
// @num_trbs_free: Number of free TRBs on the ring.
// @bounce_buf_len: Length of bounce buffer.
// @type: Ring type - event, transfer, or command ring.
// @last_td_was_short - TD is short TD.
// @trb_address_map: For mapping physical TRB addresses to segments in
// stream rings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_ring {
    pub first_seg: *mut cdnsp_segment,
    pub last_seg: *mut cdnsp_segment,
    pub enqueue: *mut cdnsp_trb,
    pub enq_seg: *mut cdnsp_segment,
    pub dequeue: *mut cdnsp_trb,
    pub deq_seg: *mut cdnsp_segment,
    pub td_list: list_head,
    pub cycle_state: u32,
    pub stream_id: c_uint,
    pub stream_active: c_uint,
    pub stream_rejected: c_uint,
    pub num_tds: c_int,
    pub num_segs: c_uint,
    pub num_trbs_free: c_uint,
    pub bounce_buf_len: c_uint,
    pub type: cdnsp_ring_type,
    pub last_td_was_short: bool,
    pub trb_address_map: *mut radix_tree_root,
}

//
// struct cdnsp_erst_entry - even ring segment table entry object.
// @seg_addr: 64-bit event ring segment address.
// seg_size: Number of TRBs in segment.;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_erst_entry {
    pub seg_addr: __le64,
    pub seg_size: __le32,
// Set to zero
    pub rsvd: __le32,
}

//
// struct cdnsp_erst - even ring segment table for event ring.
// @entries: Array of event ring segments
// @num_entries: Number of segments in entries array.
// @erst_dma_addr: DMA address for entries array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_erst {
    pub entries: *mut cdnsp_erst_entry,
    pub num_entries: c_uint,
    pub erst_dma_addr: dma_addr_t,
}

//
// struct cdnsp_request - extended device side representation of usb_request
// object .
// @td: Transfer descriptor associated with this request.
// @request: Generic usb_request object describing single I/O request.
// @list: Used to adding request to endpoint pending_list.
// @pep: Extended representation of usb_ep object
// @epnum: Endpoint number associated with usb request.
// @direction: Endpoint direction for usb request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_request {
    pub td: cdnsp_td,
    pub request: usb_request,
    pub list: list_head,
    pub pep: *mut cdnsp_ep,
    pub epnum: u8,
    pub direction:1: unsigned,
}

pub const ERST_NUM_SEGS: c_int = 1;
// Stages used during enumeration process.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdnsp_ep0_stage {
    CDNSP_SETUP_STAGE,
    CDNSP_DATA_STAGE,
    CDNSP_STATUS_STAGE,
}

//
// struct cdnsp_port - holds information about detected ports.
// @port_num: Port number.
// @exist: Indicate if port exist.
// maj_rev: Major revision.
// min_rev: Minor revision.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_port {
    pub regs: *mut cdnsp_port_regs __iomem,
    pub port_num: u8,
    pub exist: u8,
    pub maj_rev: u8,
    pub min_rev: u8,
}

//
// struct cdnsp_device - represent USB device.
// @dev: Pointer to device structure associated whit this controller.
// @gadget: Device side representation of the peripheral controller.
// @gadget_driver: Pointer to the gadget driver.
// @irq: IRQ line number used by device side.
// @regs:IO device memory.
// @cap_regs: Capability registers.
// @op_regs: Operational registers.
// @run_regs: Runtime registers.
// @dba: Device base address register.
// @ir_set: Current interrupter register set.
// @port20_regs: Port 2.0 Peripheral Configuration Registers.
// @port3x_regs: USB3.x Port Peripheral Configuration Registers.
// @rev_cap: Controller Capabilities Registers.
// @hcs_params1: Cached register copies of read-only HCSPARAMS1
// @hcc_params: Cached register copies of read-only HCCPARAMS1
// @rtl_revision: Cached controller rtl revision.
// @setup: Temporary buffer for setup packet.
// @ep0_preq: Internal allocated request used during enumeration.
// @ep0_stage: ep0 stage during enumeration process.
// @three_stage_setup: Three state or two state setup.
// @ep0_expect_in: Data IN expected for control transfer.
// @setup_id: Setup identifier.
// @setup_speed - Speed detected for current SETUP packet.
// @setup_buf: Buffer for SETUP packet.
// @device_address: Current device address.
// @may_wakeup: remote wakeup enabled/disabled.
// @lock: Lock used in interrupt thread context.
// @hci_version: device controller version.
// @dcbaa: Device context base address array.
// @cmd_ring: Command ring.
// @cmd: Represent all what is needed to issue command on Command Ring.
// @event_ring: Event ring.
// @erst: Event Ring Segment table
// @slot_id: Current Slot ID. Should be 0 or 1.
// @out_ctx: Output context.
// @in_ctx: Input context.
// @eps: array of endpoints object associated with device.
// @usb2_hw_lpm_capable: hardware lpm is enabled;
// @u1_allowed: Allow device transition to U1 state.
// @u2_allowed: Allow device transition to U2 state
// @device_pool: DMA pool for allocating input and output context.
// @segment_pool: DMA pool for allocating new segments.
// @cdnsp_state: Current state of controller.
// @link_state: Current link state.
// @usb2_port - Port USB 2.0.
// @usb3_port - Port USB 3.0.
// @active_port - Current selected Port.
// @test_mode: selected Test Mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_device {
    pub dev: *mut device,
    pub gadget: usb_gadget,
    pub gadget_driver: *mut usb_gadget_driver,
    pub irq: c_uint,
    pub regs: *mut void __iomem,
// Registers map
    pub cap_regs: *mut cdnsp_cap_regs __iomem,
    pub op_regs: *mut cdnsp_op_regs __iomem,
    pub run_regs: *mut cdnsp_run_regs __iomem,
    pub dba: *mut cdnsp_doorbell_array __iomem,
    pub ir_set: *mut cdnsp_intr_reg __iomem,
    pub port20_regs: *mut cdnsp_20port_cap __iomem,
    pub port3x_regs: *mut cdnsp_3xport_cap __iomem,
    pub rev_cap: *mut cdnsp_rev_cap __iomem,
// Cached register copies of read-only CDNSP data
    pub hcs_params1: __u32,
    pub hcs_params3: __u32,
    pub hcc_params: __u32,
pub const RTL_REVISION_NEW_LPM: c_uint = 0x2700;
    pub rtl_revision: __u32,
// Lock used in interrupt thread context.
    pub lock: spinlock_t,
    pub setup: usb_ctrlrequest,
    pub ep0_preq: cdnsp_request,
    pub ep0_stage: cdnsp_ep0_stage,
    pub three_stage_setup: u8,
    pub ep0_expect_in: u8,
    pub setup_id: u8,
    pub setup_speed: u8,
    pub setup_buf: *mut c_void,
    pub device_address: u8,
    pub may_wakeup: c_int,
    pub hci_version: u16,
// data structures
    pub dcbaa: *mut cdnsp_device_context_array,
    pub cmd_ring: *mut cdnsp_ring,
    pub cmd: cdnsp_command,
    pub event_ring: *mut cdnsp_ring,
    pub erst: cdnsp_erst,
    pub slot_id: c_int,
//
// Commands to the hardware are passed an "input context" that
// tells the hardware what to change in its data structures.
// The hardware will return changes in an "output context" that
// software must allocate for the hardware. .
//
    pub out_ctx: cdnsp_container_ctx,
    pub in_ctx: cdnsp_container_ctx,
    pub eps: [cdnsp_ep; CDNSP_ENDPOINTS_NUM],
    pub usb2_hw_lpm_capable:1: u8,
    pub u1_allowed:1: u8,
    pub u2_allowed:1: u8,
// DMA pools
    pub device_pool: *mut dma_pool,
    pub segment_pool: *mut dma_pool,

    pub cdnsp_state: c_uint,
    pub link_state: c_uint,
    pub usb2_port: cdnsp_port,
    pub eusb_port: cdnsp_port,
    pub usb3_port: cdnsp_port,
    pub active_port: *mut cdnsp_port,
    pub test_mode: u16,
}

//
// Registers should always be accessed with double word or quad word accesses.
//
// Registers with 64-bit address pointers should be written to with
// dword accesses by writing the low dword first (ptr[0]), then the high dword
// (ptr[1]) second. controller implementations that do not support 64-bit
// address pointers will ignore the high dword, and write order is irrelevant.
//
extern "C" {
    pub fn lo_hi_readq(_arg: regs) -> return;
}
// CDNSP memory management functions.
extern "C" {
    pub fn cdnsp_mem_cleanup(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_mem_init(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_setup_addressable_priv_dev(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_copy_ep0_dequeue_into_input_ctx(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_endpoint_zero(pdev: *mut cdnsp_device, ep: *mut cdnsp_ep);
}
extern "C" {
    pub fn cdnsp_alloc_streams(pdev: *mut cdnsp_device, pep: *mut cdnsp_ep) -> c_int;
}
extern "C" {
    pub fn cdnsp_free_endpoint_rings(pdev: *mut cdnsp_device, pep: *mut cdnsp_ep);
}
// Device controller glue.
extern "C" {
    pub fn cdnsp_find_next_ext_cap(base: *mut void __iomem, start: u32, id: c_int) -> c_int;
}
extern "C" {
    pub fn cdnsp_halt(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_died(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_reset(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_irq_handler(irq: c_int, priv: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cdnsp_setup_device(pdev: *mut cdnsp_device, setup: cdnsp_setup_dev) -> c_int;
}
extern "C" {
    pub fn cdnsp_thread_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
// Ring, segment, TRB, and TD functions.
extern "C" {
    pub fn cdnsp_last_trb_on_seg(seg: *mut cdnsp_segment, trb: *mut cdnsp_trb) -> bool;
}
extern "C" {
    pub fn cdnsp_wait_for_cmd_compl(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_initialize_ring_info(ring: *mut cdnsp_ring);
}
extern "C" {
    pub fn cdnsp_ring_cmd_db(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_queue_slot_control(pdev: *mut cdnsp_device, trb_type: u32);
}
extern "C" {
    pub fn cdnsp_queue_ctrl_tx(pdev: *mut cdnsp_device, preq: *mut cdnsp_request) -> c_int;
}
extern "C" {
    pub fn cdnsp_queue_bulk_tx(pdev: *mut cdnsp_device, preq: *mut cdnsp_request) -> c_int;
}
extern "C" {
    pub fn cdnsp_queue_reset_ep(pdev: *mut cdnsp_device, ep_index: c_uint);
}
extern "C" {
    pub fn cdnsp_force_header_wakeup(pdev: *mut cdnsp_device, intf_num: c_int);
}
extern "C" {
    pub fn cdnsp_queue_reset_device(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_inc_deq(pdev: *mut cdnsp_device, ring: *mut cdnsp_ring);
}
extern "C" {
    pub fn cdnsp_port_state_to_neutral(state: u32) -> u32;
}
// CDNSP device controller contexts.
extern "C" {
    pub fn cdnsp_enable_slot(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_disable_slot(pdev: *mut cdnsp_device) -> c_int;
}
// cdnsp_get_input_control_ctx(struct cdnsp_container_ctx *ctx);
// CDNSP gadget interface.
extern "C" {
    pub fn cdnsp_suspend_gadget(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_resume_gadget(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_disconnect_gadget(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_ep_enqueue(pep: *mut cdnsp_ep, preq: *mut cdnsp_request) -> c_int;
}
extern "C" {
    pub fn cdnsp_ep_dequeue(pep: *mut cdnsp_ep, preq: *mut cdnsp_request) -> c_int;
}
extern "C" {
    pub fn cdnsp_port_speed(port_status: c_uint) -> c_uint;
}
extern "C" {
    pub fn cdnsp_irq_reset(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_cmd_stop_ep(pdev: *mut cdnsp_device, pep: *mut cdnsp_ep) -> c_int;
}
extern "C" {
    pub fn cdnsp_setup_analyze(pdev: *mut cdnsp_device);
}
extern "C" {
    pub fn cdnsp_status_stage(pdev: *mut cdnsp_device) -> c_int;
}
extern "C" {
    pub fn cdnsp_reset_device(pdev: *mut cdnsp_device) -> c_int;
}
//
// next_request - gets the next request on the given list
// @list: the request list to operate on
//
// Caller should take care of locking. This function return NULL or the first
// request available on list.
//
extern "C" {
    pub fn list_first_entry_or_null(_arg: list, cdnsp_request: struct, _arg: list) -> return;
}

