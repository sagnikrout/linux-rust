//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/agilent_82357a/agilent_82357a.h
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
// copyright            : (C) 2004 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_vendor_ids {
    USB_VENDOR_ID_AGILENT = 0x0957
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_device_ids {
    USB_DEVICE_ID_AGILENT_82357A = 0x0107,
    USB_DEVICE_ID_AGILENT_82357A_PREINIT = 0x0007,	// device id before firmware is loaded
    USB_DEVICE_ID_AGILENT_82357B = 0x0718,		// device id before firmware is loaded
    USB_DEVICE_ID_AGILENT_82357B_PREINIT = 0x0518,	// device id before firmware is loaded
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum endpoint_addresses {
    AGILENT_82357_CONTROL_ENDPOINT = 0x0,
    AGILENT_82357_BULK_IN_ENDPOINT = 0x2,
    AGILENT_82357A_BULK_OUT_ENDPOINT = 0x4,
    AGILENT_82357A_INTERRUPT_IN_ENDPOINT = 0x6,
    AGILENT_82357B_BULK_OUT_ENDPOINT = 0x6,
    AGILENT_82357B_INTERRUPT_IN_ENDPOINT = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bulk_commands {
    DATA_PIPE_CMD_WRITE = 0x1,
    DATA_PIPE_CMD_READ = 0x3,
    DATA_PIPE_CMD_WR_REGS = 0x4,
    DATA_PIPE_CMD_RD_REGS = 0x5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357a_read_flags {
    ARF_END_ON_EOI = 0x1,
    ARF_NO_ADDRESS = 0x2,
    ARF_END_ON_EOS_CHAR = 0x4,
    ARF_SPOLL = 0x8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357a_trailing_read_flags {
    ATRF_EOI = 0x1,
    ATRF_ATN = 0x2,
    ATRF_IFC = 0x4,
    ATRF_EOS = 0x8,
    ATRF_ABORT = 0x10,
    ATRF_COUNT = 0x20,
    ATRF_DEAD_BUS = 0x40,
    ATRF_UNADDRESSED = 0x80
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357a_write_flags {
    AWF_SEND_EOI = 0x1,
    AWF_NO_FAST_TALKER_FIRST_BYTE = 0x2,
    AWF_NO_FAST_TALKER = 0x4,
    AWF_NO_ADDRESS = 0x8,
    AWF_ATN = 0x10,
    AWF_SEPARATE_HEADER = 0x80
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357a_interrupt_flag_bit_numbers {
    AIF_SRQ_BN = 0,
    AIF_WRITE_COMPLETE_BN = 1,
    AIF_READ_COMPLETE_BN = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357_error_codes {
    UGP_SUCCESS = 0,
    UGP_ERR_INVALID_CMD = 1,
    UGP_ERR_INVALID_PARAM = 2,
    UGP_ERR_INVALID_REG = 3,
    UGP_ERR_GPIB_READ = 4,
    UGP_ERR_GPIB_WRITE = 5,
    UGP_ERR_FLUSHING = 6,
    UGP_ERR_FLUSHING_ALREADY = 7,
    UGP_ERR_UNSUPPORTED = 8,
    UGP_ERR_OTHER  = 9
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum agilent_82357_control_values {
    XFER_ABORT = 0xa0,
    XFER_STATUS = 0xb0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfer_status_bits {
    XS_COMPLETED = 0x1,
    XS_READ = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfer_status_completion_bits {
    XSC_EOI = 0x1,
    XSC_ATN = 0x2,
    XSC_IFC = 0x4,
    XSC_EOS = 0x8,
    XSC_ABORT = 0x10,
    XSC_COUNT = 0x20,
    XSC_DEAD_BUS = 0x40,
    XSC_BUS_NOT_ADDRESSED = 0x80
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfer_abort_type {
    XA_FLUSH = 0x1
}

pub const STATUS_DATA_LEN: c_int = 8;
pub const INTERRUPT_BUF_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilent_82357a_urb_ctx {
    pub complete: completion,
    pub 1: unsigned timed_out :,
}

// struct which defines local data for each 82357 device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilent_82357a_priv {
    pub bus_interface: *mut usb_interface,
    pub eos_char: c_ushort,
    pub eos_mode: c_ushort,
    pub hw_control_bits: c_ushort,
    pub interrupt_flags: c_ulong,
    pub bulk_urb: *mut urb,
    pub interrupt_urb: *mut urb,
    pub interrupt_buffer: *mut u8,
    pub lock: mutex bulk_transfer_lock; // bulk transfer,
    pub lock: mutex bulk_alloc_lock; // bulk transfer allocation,
    pub lock: mutex interrupt_alloc_lock; // interrupt allocation,
    pub lock: mutex control_alloc_lock; // control message allocation,
    pub bulk_timer: timer_list,
    pub context: agilent_82357a_urb_ctx,
    pub bulk_out_endpoint: c_uint,
    pub interrupt_in_endpoint: c_uint,
    pub 1: unsigned is_cic :,
    pub 1: unsigned ren_state :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilent_82357a_register_pairlet {
    pub address: c_short,
    pub value: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum firmware_registers {
    HW_CONTROL = 0xa,
    LED_CONTROL = 0xb,
    RESET_TO_POWERUP = 0xc,
    PROTOCOL_CONTROL = 0xd,
    FAST_TALKER_T1 = 0xe
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hardware_control_bits {
    NOT_TI_RESET = 0x1,
    SYSTEM_CONTROLLER = 0x2,
    NOT_PARALLEL_POLL = 0x4,
    OSCILLATOR_5V_ON = 0x8,
    OUTPUT_5V_ON = 0x20,
    CPLD_3V_ON = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_control_bits {
    FIRMWARE_LED_CONTROL = 0x1,
    FAIL_LED_ON = 0x20,
    READY_LED_ON = 0x40,
    ACCESS_LED_ON = 0x80
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_to_powerup_bits {
    RESET_SPACEBALL = 0x1,	// wait 2 millisec after sending
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protocol_control_bits {
    WRITE_COMPLETE_INTERRUPT_EN = 0x1,
}
