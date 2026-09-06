//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/net2280.h
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
//
// -------------------------------------------------------------------------
// NET2280 MEMORY MAPPED REGISTERS
//
// The register layout came from the chip documentation, and the bit
// number definitions were extracted from chip specification.
//
// Use the shift operator ('<<') to build bit masks, with readl/writel
// to access the registers through PCI.
//
// main registers, BAR0 + 0x0000
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_regs {
// offset 0x0000
    pub devinit: u32,
pub const LOCAL_CLOCK_FREQUENCY: c_int = 8;
pub const FORCE_PCI_RESET: c_int = 7;
pub const PCI_ID: c_int = 6;
pub const PCI_ENABLE: c_int = 5;
pub const FIFO_SOFT_RESET: c_int = 4;
pub const CFG_SOFT_RESET: c_int = 3;
pub const PCI_SOFT_RESET: c_int = 2;
pub const USB_SOFT_RESET: c_int = 1;
pub const M8051_RESET: c_int = 0;
    pub eectl: u32,
pub const EEPROM_ADDRESS_WIDTH: c_int = 23;
pub const EEPROM_CHIP_SELECT_ACTIVE: c_int = 22;
pub const EEPROM_PRESENT: c_int = 21;
pub const EEPROM_VALID: c_int = 20;
pub const EEPROM_BUSY: c_int = 19;
pub const EEPROM_CHIP_SELECT_ENABLE: c_int = 18;
pub const EEPROM_BYTE_READ_START: c_int = 17;
pub const EEPROM_BYTE_WRITE_START: c_int = 16;
pub const EEPROM_READ_DATA: c_int = 8;
pub const EEPROM_WRITE_DATA: c_int = 0;
    pub eeclkfreq: u32,
    pub _unused0: u32,
// offset 0x0010
    pub /: *mut *mut u32 pciirqenb0; / interrupt PCI master ...,
pub const SETUP_PACKET_INTERRUPT_ENABLE: c_int = 7;
pub const ENDPOINT_F_INTERRUPT_ENABLE: c_int = 6;
pub const ENDPOINT_E_INTERRUPT_ENABLE: c_int = 5;
pub const ENDPOINT_D_INTERRUPT_ENABLE: c_int = 4;
pub const ENDPOINT_C_INTERRUPT_ENABLE: c_int = 3;
pub const ENDPOINT_B_INTERRUPT_ENABLE: c_int = 2;
pub const ENDPOINT_A_INTERRUPT_ENABLE: c_int = 1;
pub const ENDPOINT_0_INTERRUPT_ENABLE: c_int = 0;
    pub pciirqenb1: u32,
pub const PCI_INTERRUPT_ENABLE: c_int = 31;
pub const POWER_STATE_CHANGE_INTERRUPT_ENABLE: c_int = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT_ENABLE: c_int = 26;
pub const PCI_PARITY_ERROR_INTERRUPT_ENABLE: c_int = 25;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 19;
pub const PCI_TARGET_ABORT_ASSERTED_INTERRUPT_ENABLE: c_int = 18;
pub const PCI_RETRY_ABORT_INTERRUPT_ENABLE: c_int = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT_ENABLE: c_int = 16;
pub const GPIO_INTERRUPT_ENABLE: c_int = 13;
pub const DMA_D_INTERRUPT_ENABLE: c_int = 12;
pub const DMA_C_INTERRUPT_ENABLE: c_int = 11;
pub const DMA_B_INTERRUPT_ENABLE: c_int = 10;
pub const DMA_A_INTERRUPT_ENABLE: c_int = 9;
pub const EEPROM_DONE_INTERRUPT_ENABLE: c_int = 8;
pub const VBUS_INTERRUPT_ENABLE: c_int = 7;
pub const CONTROL_STATUS_INTERRUPT_ENABLE: c_int = 6;
pub const ROOT_PORT_RESET_INTERRUPT_ENABLE: c_int = 4;
pub const SUSPEND_REQUEST_INTERRUPT_ENABLE: c_int = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT_ENABLE: c_int = 2;
pub const RESUME_INTERRUPT_ENABLE: c_int = 1;
pub const SOF_INTERRUPT_ENABLE: c_int = 0;
    pub /: *mut *mut u32 cpu_irqenb0; / ... or onboard 8051,
pub const SETUP_PACKET_INTERRUPT_ENABLE: c_int = 7;
pub const ENDPOINT_F_INTERRUPT_ENABLE: c_int = 6;
pub const ENDPOINT_E_INTERRUPT_ENABLE: c_int = 5;
pub const ENDPOINT_D_INTERRUPT_ENABLE: c_int = 4;
pub const ENDPOINT_C_INTERRUPT_ENABLE: c_int = 3;
pub const ENDPOINT_B_INTERRUPT_ENABLE: c_int = 2;
pub const ENDPOINT_A_INTERRUPT_ENABLE: c_int = 1;
pub const ENDPOINT_0_INTERRUPT_ENABLE: c_int = 0;
    pub cpu_irqenb1: u32,
pub const CPU_INTERRUPT_ENABLE: c_int = 31;
pub const POWER_STATE_CHANGE_INTERRUPT_ENABLE: c_int = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT_ENABLE: c_int = 26;
pub const PCI_PARITY_ERROR_INTERRUPT_ENABLE: c_int = 25;
pub const PCI_INTA_INTERRUPT_ENABLE: c_int = 24;
pub const PCI_PME_INTERRUPT_ENABLE: c_int = 23;
pub const PCI_SERR_INTERRUPT_ENABLE: c_int = 22;
pub const PCI_PERR_INTERRUPT_ENABLE: c_int = 21;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 19;
pub const PCI_RETRY_ABORT_INTERRUPT_ENABLE: c_int = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT_ENABLE: c_int = 16;
pub const GPIO_INTERRUPT_ENABLE: c_int = 13;
pub const DMA_D_INTERRUPT_ENABLE: c_int = 12;
pub const DMA_C_INTERRUPT_ENABLE: c_int = 11;
pub const DMA_B_INTERRUPT_ENABLE: c_int = 10;
pub const DMA_A_INTERRUPT_ENABLE: c_int = 9;
pub const EEPROM_DONE_INTERRUPT_ENABLE: c_int = 8;
pub const VBUS_INTERRUPT_ENABLE: c_int = 7;
pub const CONTROL_STATUS_INTERRUPT_ENABLE: c_int = 6;
pub const ROOT_PORT_RESET_INTERRUPT_ENABLE: c_int = 4;
pub const SUSPEND_REQUEST_INTERRUPT_ENABLE: c_int = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT_ENABLE: c_int = 2;
pub const RESUME_INTERRUPT_ENABLE: c_int = 1;
pub const SOF_INTERRUPT_ENABLE: c_int = 0;
// offset 0x0020
    pub _unused1: u32,
    pub usbirqenb1: u32,
pub const USB_INTERRUPT_ENABLE: c_int = 31;
pub const POWER_STATE_CHANGE_INTERRUPT_ENABLE: c_int = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT_ENABLE: c_int = 26;
pub const PCI_PARITY_ERROR_INTERRUPT_ENABLE: c_int = 25;
pub const PCI_INTA_INTERRUPT_ENABLE: c_int = 24;
pub const PCI_PME_INTERRUPT_ENABLE: c_int = 23;
pub const PCI_SERR_INTERRUPT_ENABLE: c_int = 22;
pub const PCI_PERR_INTERRUPT_ENABLE: c_int = 21;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT_ENABLE: c_int = 19;
pub const PCI_RETRY_ABORT_INTERRUPT_ENABLE: c_int = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT_ENABLE: c_int = 16;
pub const GPIO_INTERRUPT_ENABLE: c_int = 13;
pub const DMA_D_INTERRUPT_ENABLE: c_int = 12;
pub const DMA_C_INTERRUPT_ENABLE: c_int = 11;
pub const DMA_B_INTERRUPT_ENABLE: c_int = 10;
pub const DMA_A_INTERRUPT_ENABLE: c_int = 9;
pub const EEPROM_DONE_INTERRUPT_ENABLE: c_int = 8;
pub const VBUS_INTERRUPT_ENABLE: c_int = 7;
pub const CONTROL_STATUS_INTERRUPT_ENABLE: c_int = 6;
pub const ROOT_PORT_RESET_INTERRUPT_ENABLE: c_int = 4;
pub const SUSPEND_REQUEST_INTERRUPT_ENABLE: c_int = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT_ENABLE: c_int = 2;
pub const RESUME_INTERRUPT_ENABLE: c_int = 1;
pub const SOF_INTERRUPT_ENABLE: c_int = 0;
    pub irqstat0: u32,
pub const INTA_ASSERTED: c_int = 12;
pub const SETUP_PACKET_INTERRUPT: c_int = 7;
pub const ENDPOINT_F_INTERRUPT: c_int = 6;
pub const ENDPOINT_E_INTERRUPT: c_int = 5;
pub const ENDPOINT_D_INTERRUPT: c_int = 4;
pub const ENDPOINT_C_INTERRUPT: c_int = 3;
pub const ENDPOINT_B_INTERRUPT: c_int = 2;
pub const ENDPOINT_A_INTERRUPT: c_int = 1;
pub const ENDPOINT_0_INTERRUPT: c_int = 0;

    pub irqstat1: u32,
pub const POWER_STATE_CHANGE_INTERRUPT: c_int = 27;
pub const PCI_ARBITER_TIMEOUT_INTERRUPT: c_int = 26;
pub const PCI_PARITY_ERROR_INTERRUPT: c_int = 25;
pub const PCI_INTA_INTERRUPT: c_int = 24;
pub const PCI_PME_INTERRUPT: c_int = 23;
pub const PCI_SERR_INTERRUPT: c_int = 22;
pub const PCI_PERR_INTERRUPT: c_int = 21;
pub const PCI_MASTER_ABORT_RECEIVED_INTERRUPT: c_int = 20;
pub const PCI_TARGET_ABORT_RECEIVED_INTERRUPT: c_int = 19;
pub const PCI_RETRY_ABORT_INTERRUPT: c_int = 17;
pub const PCI_MASTER_CYCLE_DONE_INTERRUPT: c_int = 16;
pub const SOF_DOWN_INTERRUPT: c_int = 14;
pub const GPIO_INTERRUPT: c_int = 13;
pub const DMA_D_INTERRUPT: c_int = 12;
pub const DMA_C_INTERRUPT: c_int = 11;
pub const DMA_B_INTERRUPT: c_int = 10;
pub const DMA_A_INTERRUPT: c_int = 9;
pub const EEPROM_DONE_INTERRUPT: c_int = 8;
pub const VBUS_INTERRUPT: c_int = 7;
pub const CONTROL_STATUS_INTERRUPT: c_int = 6;
pub const ROOT_PORT_RESET_INTERRUPT: c_int = 4;
pub const SUSPEND_REQUEST_INTERRUPT: c_int = 3;
pub const SUSPEND_REQUEST_CHANGE_INTERRUPT: c_int = 2;
pub const RESUME_INTERRUPT: c_int = 1;
pub const SOF_INTERRUPT: c_int = 0;
// offset 0x0030
    pub idxaddr: u32,
    pub idxdata: u32,
    pub fifoctl: u32,
pub const PCI_BASE2_RANGE: c_int = 16;
pub const IGNORE_FIFO_AVAILABILITY: c_int = 3;
pub const PCI_BASE2_SELECT: c_int = 2;
pub const FIFO_CONFIGURATION_SELECT: c_int = 0;
    pub _unused2: u32,
// offset 0x0040
    pub memaddr: u32,
pub const START: c_int = 28;
pub const DIRECTION: c_int = 27;
pub const FIFO_DIAGNOSTIC_SELECT: c_int = 24;
pub const MEMORY_ADDRESS: c_int = 0;
    pub memdata0: u32,
    pub memdata1: u32,
    pub _unused3: u32,
// offset 0x0050
    pub gpioctl: u32,
pub const GPIO3_LED_SELECT: c_int = 12;
pub const GPIO3_INTERRUPT_ENABLE: c_int = 11;
pub const GPIO2_INTERRUPT_ENABLE: c_int = 10;
pub const GPIO1_INTERRUPT_ENABLE: c_int = 9;
pub const GPIO0_INTERRUPT_ENABLE: c_int = 8;
pub const GPIO3_OUTPUT_ENABLE: c_int = 7;
pub const GPIO2_OUTPUT_ENABLE: c_int = 6;
pub const GPIO1_OUTPUT_ENABLE: c_int = 5;
pub const GPIO0_OUTPUT_ENABLE: c_int = 4;
pub const GPIO3_DATA: c_int = 3;
pub const GPIO2_DATA: c_int = 2;
pub const GPIO1_DATA: c_int = 1;
pub const GPIO0_DATA: c_int = 0;
    pub gpiostat: u32,
pub const GPIO3_INTERRUPT: c_int = 3;
pub const GPIO2_INTERRUPT: c_int = 2;
pub const GPIO1_INTERRUPT: c_int = 1;
pub const GPIO0_INTERRUPT: c_int = 0;
// C attribute field omitted
// usb control, BAR0 + 0x0080
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_usb_regs {
// offset 0x0080
    pub stdrsp: u32,
pub const STALL_UNSUPPORTED_REQUESTS: c_int = 31;
pub const SET_TEST_MODE: c_int = 16;
pub const GET_OTHER_SPEED_CONFIGURATION: c_int = 15;
pub const GET_DEVICE_QUALIFIER: c_int = 14;
pub const SET_ADDRESS: c_int = 13;
pub const ENDPOINT_SET_CLEAR_HALT: c_int = 12;
pub const DEVICE_SET_CLEAR_DEVICE_REMOTE_WAKEUP: c_int = 11;
pub const GET_STRING_DESCRIPTOR_2: c_int = 10;
pub const GET_STRING_DESCRIPTOR_1: c_int = 9;
pub const GET_STRING_DESCRIPTOR_0: c_int = 8;
pub const GET_SET_INTERFACE: c_int = 6;
pub const GET_SET_CONFIGURATION: c_int = 5;
pub const GET_CONFIGURATION_DESCRIPTOR: c_int = 4;
pub const GET_DEVICE_DESCRIPTOR: c_int = 3;
pub const GET_ENDPOINT_STATUS: c_int = 2;
pub const GET_INTERFACE_STATUS: c_int = 1;
pub const GET_DEVICE_STATUS: c_int = 0;
    pub prodvendid: u32,
pub const PRODUCT_ID: c_int = 16;
pub const VENDOR_ID: c_int = 0;
    pub relnum: u32,
    pub usbctl: u32,
pub const SERIAL_NUMBER_INDEX: c_int = 16;
pub const PRODUCT_ID_STRING_ENABLE: c_int = 13;
pub const VENDOR_ID_STRING_ENABLE: c_int = 12;
pub const USB_ROOT_PORT_WAKEUP_ENABLE: c_int = 11;
pub const VBUS_PIN: c_int = 10;
pub const TIMED_DISCONNECT: c_int = 9;
pub const SUSPEND_IMMEDIATELY: c_int = 7;
pub const SELF_POWERED_USB_DEVICE: c_int = 6;
pub const REMOTE_WAKEUP_SUPPORT: c_int = 5;
pub const PME_POLARITY: c_int = 4;
pub const USB_DETECT_ENABLE: c_int = 3;
pub const PME_WAKEUP_ENABLE: c_int = 2;
pub const DEVICE_REMOTE_WAKEUP_ENABLE: c_int = 1;
pub const SELF_POWERED_STATUS: c_int = 0;
// offset 0x0090
    pub usbstat: u32,
pub const HIGH_SPEED: c_int = 7;
pub const FULL_SPEED: c_int = 6;
pub const GENERATE_RESUME: c_int = 5;
pub const GENERATE_DEVICE_REMOTE_WAKEUP: c_int = 4;
    pub xcvrdiag: u32,
pub const FORCE_HIGH_SPEED_MODE: c_int = 31;
pub const FORCE_FULL_SPEED_MODE: c_int = 30;
pub const USB_TEST_MODE: c_int = 24;
pub const LINE_STATE: c_int = 16;
pub const TRANSCEIVER_OPERATION_MODE: c_int = 2;
pub const TRANSCEIVER_SELECT: c_int = 1;
pub const TERMINATION_SELECT: c_int = 0;
    pub setup0123: u32,
    pub setup4567: u32,
// offset 0x0090
    pub _unused0: u32,
    pub ouraddr: u32,
pub const FORCE_IMMEDIATE: c_int = 7;
pub const OUR_USB_ADDRESS: c_int = 0;
    pub ourconfig: u32,
// C attribute field omitted
// pci control, BAR0 + 0x0100
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_pci_regs {
// offset 0x0100
    pub pcimstctl: u32,
pub const PCI_ARBITER_PARK_SELECT: c_int = 13;

pub const PCI_RETRY_ABORT_ENABLE: c_int = 11;
pub const DMA_MEMORY_WRITE_AND_INVALIDATE_ENABLE: c_int = 10;
pub const DMA_READ_MULTIPLE_ENABLE: c_int = 9;
pub const DMA_READ_LINE_ENABLE: c_int = 8;
pub const PCI_MASTER_COMMAND_SELECT: c_int = 6;
pub const MEM_READ_OR_WRITE: c_int = 0;
pub const IO_READ_OR_WRITE: c_int = 1;
pub const CFG_READ_OR_WRITE: c_int = 2;
pub const PCI_MASTER_START: c_int = 5;
pub const PCI_MASTER_READ_WRITE: c_int = 4;
pub const PCI_MASTER_WRITE: c_int = 0;
pub const PCI_MASTER_READ: c_int = 1;
pub const PCI_MASTER_BYTE_WRITE_ENABLES: c_int = 0;
    pub pcimstaddr: u32,
    pub pcimstdata: u32,
    pub pcimststat: u32,
pub const PCI_ARBITER_CLEAR: c_int = 2;
pub const PCI_EXTERNAL_ARBITER: c_int = 1;
pub const PCI_HOST_MODE: c_int = 0;
// C attribute field omitted
// dma control, BAR0 + 0x0180 ... array of four structs like this,
// for channels 0..3.  see also struct net2280_dma:  descriptor
// that can be loaded into some of these registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_dma_regs {
// offset 0x0180, 0x01a0, 0x01c0, 0x01e0,
    pub dmactl: u32,
pub const DMA_SCATTER_GATHER_DONE_INTERRUPT_ENABLE: c_int = 25;
pub const DMA_CLEAR_COUNT_ENABLE: c_int = 21;
pub const DESCRIPTOR_POLLING_RATE: c_int = 19;
pub const POLL_CONTINUOUS: c_int = 0;
pub const POLL_1_USEC: c_int = 1;
pub const POLL_100_USEC: c_int = 2;
pub const POLL_1_MSEC: c_int = 3;
pub const DMA_VALID_BIT_POLLING_ENABLE: c_int = 18;
pub const DMA_VALID_BIT_ENABLE: c_int = 17;
pub const DMA_SCATTER_GATHER_ENABLE: c_int = 16;
pub const DMA_OUT_AUTO_START_ENABLE: c_int = 4;
pub const DMA_PREEMPT_ENABLE: c_int = 3;
pub const DMA_FIFO_VALIDATE: c_int = 2;
pub const DMA_ENABLE: c_int = 1;
pub const DMA_ADDRESS_HOLD: c_int = 0;
    pub dmastat: u32,
pub const DMA_ABORT_DONE_INTERRUPT: c_int = 27;
pub const DMA_SCATTER_GATHER_DONE_INTERRUPT: c_int = 25;
pub const DMA_TRANSACTION_DONE_INTERRUPT: c_int = 24;
pub const DMA_ABORT: c_int = 1;
pub const DMA_START: c_int = 0;
    pub _unused0: [u32; 2],
// offset 0x0190, 0x01b0, 0x01d0, 0x01f0,
    pub dmacount: u32,
pub const VALID_BIT: c_int = 31;
pub const DMA_DIRECTION: c_int = 30;
pub const DMA_DONE_INTERRUPT_ENABLE: c_int = 29;
pub const END_OF_CHAIN: c_int = 28;

pub const DMA_BYTE_COUNT: c_int = 0;
    pub dmaaddr: u32,
    pub dmadesc: u32,
    pub _unused1: u32,
// C attribute field omitted
// dedicated endpoint registers, BAR0 + 0x0200
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_dep_regs {
// offset 0x0200, 0x0210, 0x220, 0x230, 0x240
    pub dep_cfg: u32,
// offset 0x0204, 0x0214, 0x224, 0x234, 0x244
    pub dep_rsp: u32,
    pub _unused: [u32; 2],
// C attribute field omitted
// configurable endpoint registers, BAR0 + 0x0300 ... array of seven structs
// like this, for ep0 then the configurable endpoints A..F
// ep0 reserved for control; E and F have only 64 bytes of fifo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_ep_regs {
// offset 0x0300, 0x0320, 0x0340, 0x0360, 0x0380, 0x03a0, 0x03c0
    pub ep_cfg: u32,
pub const ENDPOINT_BYTE_COUNT: c_int = 16;
pub const ENDPOINT_ENABLE: c_int = 10;
pub const ENDPOINT_TYPE: c_int = 8;
pub const ENDPOINT_DIRECTION: c_int = 7;
pub const ENDPOINT_NUMBER: c_int = 0;
    pub ep_rsp: u32,
pub const SET_NAK_OUT_PACKETS: c_int = 15;
pub const SET_EP_HIDE_STATUS_PHASE: c_int = 14;
pub const SET_EP_FORCE_CRC_ERROR: c_int = 13;
pub const SET_INTERRUPT_MODE: c_int = 12;
pub const SET_CONTROL_STATUS_PHASE_HANDSHAKE: c_int = 11;
pub const SET_NAK_OUT_PACKETS_MODE: c_int = 10;
pub const SET_ENDPOINT_TOGGLE: c_int = 9;
pub const SET_ENDPOINT_HALT: c_int = 8;
pub const CLEAR_NAK_OUT_PACKETS: c_int = 7;
pub const CLEAR_EP_HIDE_STATUS_PHASE: c_int = 6;
pub const CLEAR_EP_FORCE_CRC_ERROR: c_int = 5;
pub const CLEAR_INTERRUPT_MODE: c_int = 4;
pub const CLEAR_CONTROL_STATUS_PHASE_HANDSHAKE: c_int = 3;
pub const CLEAR_NAK_OUT_PACKETS_MODE: c_int = 2;
pub const CLEAR_ENDPOINT_TOGGLE: c_int = 1;
pub const CLEAR_ENDPOINT_HALT: c_int = 0;
    pub ep_irqenb: u32,
pub const SHORT_PACKET_OUT_DONE_INTERRUPT_ENABLE: c_int = 6;
pub const SHORT_PACKET_TRANSFERRED_INTERRUPT_ENABLE: c_int = 5;
pub const DATA_PACKET_RECEIVED_INTERRUPT_ENABLE: c_int = 3;
pub const DATA_PACKET_TRANSMITTED_INTERRUPT_ENABLE: c_int = 2;
pub const DATA_OUT_PING_TOKEN_INTERRUPT_ENABLE: c_int = 1;
pub const DATA_IN_TOKEN_INTERRUPT_ENABLE: c_int = 0;
    pub ep_stat: u32,
pub const FIFO_VALID_COUNT: c_int = 24;
pub const HIGH_BANDWIDTH_OUT_TRANSACTION_PID: c_int = 22;
pub const TIMEOUT: c_int = 21;
pub const USB_STALL_SENT: c_int = 20;
pub const USB_IN_NAK_SENT: c_int = 19;
pub const USB_IN_ACK_RCVD: c_int = 18;
pub const USB_OUT_PING_NAK_SENT: c_int = 17;
pub const USB_OUT_ACK_SENT: c_int = 16;
pub const FIFO_OVERFLOW: c_int = 13;
pub const FIFO_UNDERFLOW: c_int = 12;
pub const FIFO_FULL: c_int = 11;
pub const FIFO_EMPTY: c_int = 10;
pub const FIFO_FLUSH: c_int = 9;
pub const SHORT_PACKET_OUT_DONE_INTERRUPT: c_int = 6;
pub const SHORT_PACKET_TRANSFERRED_INTERRUPT: c_int = 5;
pub const NAK_OUT_PACKETS: c_int = 4;
pub const DATA_PACKET_RECEIVED_INTERRUPT: c_int = 3;
pub const DATA_PACKET_TRANSMITTED_INTERRUPT: c_int = 2;
pub const DATA_OUT_PING_TOKEN_INTERRUPT: c_int = 1;
pub const DATA_IN_TOKEN_INTERRUPT: c_int = 0;
// offset 0x0310, 0x0330, 0x0350, 0x0370, 0x0390, 0x03b0, 0x03d0
    pub ep_avail: u32,
    pub ep_data: u32,
    pub _unused0: [u32; 2],
// C attribute field omitted
