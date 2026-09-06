//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/hp_82341/hp_82341.h
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
// copyright            : (C) 2002, 2005 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341_hardware_version {
    HW_VERSION_UNKNOWN,
    HW_VERSION_82341C,
    HW_VERSION_82341D,
}

// struct which defines private_data for board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hp_82341_priv {
    pub tms9914_priv: tms9914_priv,
    pub irq: c_uint,
    pub config_control_bits: c_ushort,
    pub mode_control_bits: c_ushort,
    pub event_status_bits: c_ushort,
    pub pnp_dev: *mut pnp_dev,
    pub iobase: [c_ulong; 4],
    pub io_region_offset: c_ulong,
    pub hw_version: hp_82341_hardware_version,
}

// hp 82341 register offsets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341_region_0_registers {
    CONFIG_CONTROL_STATUS_REG = 0x0,
    MODE_CONTROL_STATUS_REG = 0x1,
    MONITOR_REG = 0x2,	// after initialization
    XILINX_DATA_REG = 0x2,	// before initialization, write only
    INTERRUPT_ENABLE_REG = 0x3,
    EVENT_STATUS_REG = 0x4,
    EVENT_ENABLE_REG = 0x5,
    STREAM_STATUS_REG = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341_region_1_registers {
    ID0_REG = 0x2,
    ID1_REG = 0x3,
    TRANSFER_COUNT_LOW_REG = 0x4,
    TRANSFER_COUNT_MID_REG = 0x5,
    TRANSFER_COUNT_HIGH_REG = 0x6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341_region_3_registers {
    BUFFER_PORT_LOW_REG = 0x0,
    BUFFER_PORT_HIGH_REG = 0x1,
    ID2_REG = 0x2,
    ID3_REG = 0x3,
    BUFFER_FLUSH_REG = 0x4,
    BUFFER_CONTROL_REG = 0x7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum config_control_status_bits {
    IRQ_SELECT_MASK = 0x7,
    DMA_CONFIG_MASK = 0x18,
    ENABLE_DMA_CONFIG_BIT = 0x20,
    XILINX_READY_BIT = 0x40,	// read only
    DONE_PGL_BIT = 0x80
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mode_control_status_bits {
    SLOT8_BIT = 0x1,		// read only
    ACTIVE_CONTROLLER_BIT = 0x2,	// read only
    ENABLE_DMA_BIT = 0x4,
    SYSTEM_CONTROLLER_BIT = 0x8,
    MONITOR_BIT = 0x10,
    ENABLE_IRQ_CONFIG_BIT = 0x20,
    ENABLE_TI_STREAM_BIT = 0x40
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum monitor_bits {
    MONITOR_INTERRUPT_PENDING_BIT = 0x1,	// read only
    MONITOR_CLEAR_HOLDOFF_BIT = 0x2,	// write only
    MONITOR_PPOLL_BIT = 0x4,		// write clear
    MONITOR_SRQ_BIT = 0x8,			// write clear
    MONITOR_IFC_BIT = 0x10,			// write clear
    MONITOR_REN_BIT = 0x20,			// write clear
    MONITOR_END_BIT = 0x40,			// write clear
    MONITOR_DAV_BIT = 0x80			// write clear
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum interrupt_enable_bits {
    ENABLE_TI_INTERRUPT_BIT = 0x1,
    ENABLE_POINTERS_EQUAL_INTERRUPT_BIT = 0x4,
    ENABLE_BUFFER_END_INTERRUPT_BIT = 0x10,
    ENABLE_TERMINAL_COUNT_INTERRUPT_BIT = 0x20,
    ENABLE_DMA_TERMINAL_COUNT_INTERRUPT_BIT = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_status_bits {
    TI_INTERRUPT_EVENT_BIT = 0x1,		// write clear
    INTERRUPT_PENDING_EVENT_BIT = 0x2,	// read only
    POINTERS_EQUAL_EVENT_BIT = 0x4,		// write clear
    BUFFER_END_EVENT_BIT = 0x10,		// write clear
    TERMINAL_COUNT_EVENT_BIT = 0x20,	// write clear
    DMA_TERMINAL_COUNT_EVENT_BIT = 0x80,	// write clear
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_enable_bits {
    ENABLE_TI_INTERRUPT_EVENT_BIT = 0x1,		// write clear
    ENABLE_POINTERS_EQUAL_EVENT_BIT = 0x4,		// write clear
    ENABLE_BUFFER_END_EVENT_BIT = 0x10,		// write clear
    ENABLE_TERMINAL_COUNT_EVENT_BIT = 0x20,		// write clear
    ENABLE_DMA_TERMINAL_COUNT_EVENT_BIT = 0x80,	// write clear
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stream_status_bits {
    HALTED_STATUS_BIT = 0x1,	// read
    RESTART_STREAM_BIT = 0x1	// write
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum buffer_control_bits {
    DIRECTION_GPIB_TO_HOST_BIT = 0x20,	// transfer direction (set for gpib to host)
    ENABLE_TI_BUFFER_BIT = 0x40,		// enable fifo
    FAST_WR_EN_BIT = 0x80,			// 350 ns t1 delay?
}

// registers accessible through isapnp chip on 82341d
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341d_pnp_registers {
    PIO_DATA_REG = 0x20,		// read/write pio data lines
    PIO_DIRECTION_REG = 0x21,	// set pio data line directions (set for input)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_82341d_pnp_pio_bits {
    HP_82341D_XILINX_READY_BIT = 0x1,
    HP_82341D_XILINX_DONE_BIT = 0x2,
// use register layout compatible with C and older versions instead of 32 contiguous ioports
    HP_82341D_LEGACY_MODE_BIT = 0x4,
    HP_82341D_NOT_PROG_BIT = 0x8,	// clear to reinitialize xilinx
}
