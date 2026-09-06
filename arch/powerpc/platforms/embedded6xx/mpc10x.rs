//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/embedded6xx/mpc10x.h
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


//
// Common routines for the Motorola SPS MPC106/8240/107 Host bridge/Mem
// ctlr/EPIC/etc.
//
// Author: Mark A. Greer
// mgreer@mvista.com
//
// 2001 (c) MontaVista, Software, Inc.  This file is licensed under
// the terms of the GNU General Public License version 2.  This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

//
// The values here don't completely map everything but should work in most
// cases.
//
// MAP A (PReP Map)
// Processor: 0x80000000 - 0x807fffff -> PCI I/O: 0x00000000 - 0x007fffff
// Processor: 0xc0000000 - 0xdfffffff -> PCI MEM: 0x00000000 - 0x1fffffff
// PCI MEM:   0x80000000 -> Processor System Memory: 0x00000000
//
// MAP B (CHRP Map)
// Processor: 0xfe000000 - 0xfebfffff -> PCI I/O: 0x00000000 - 0x00bfffff
// Processor: 0x80000000 - 0xbfffffff -> PCI MEM: 0x80000000 - 0xbfffffff
// PCI MEM:   0x00000000 -> Processor System Memory: 0x00000000
//
// Define the vendor/device IDs for the various bridges--should be added to
// <linux/pci_ids.h>
//

// Define the type of map to use
pub const MPC10X_MEM_MAP_A: c_int = 1;
pub const MPC10X_MEM_MAP_B: c_int = 2;
// Map A (PReP Map) Defines
pub const MPC10X_MAPA_CNFG_ADDR: c_uint = 0x80000cf8;
pub const MPC10X_MAPA_CNFG_DATA: c_uint = 0x80000cfc;
pub const MPC10X_MAPA_ISA_IO_BASE: c_uint = 0x80000000;
pub const MPC10X_MAPA_ISA_MEM_BASE: c_uint = 0xc0000000;
pub const MPC10X_MAPA_DRAM_OFFSET: c_uint = 0x80000000;
pub const MPC10X_MAPA_PCI_INTACK_ADDR: c_uint = 0xbffffff0;
pub const MPC10X_MAPA_PCI_IO_START: c_uint = 0x00000000;

pub const MPC10X_MAPA_PCI_MEM_START: c_uint = 0x00000000;

// Map B (CHRP Map) Defines
pub const MPC10X_MAPB_CNFG_ADDR: c_uint = 0xfec00000;
pub const MPC10X_MAPB_CNFG_DATA: c_uint = 0xfee00000;
pub const MPC10X_MAPB_ISA_IO_BASE: c_uint = 0xfe000000;
pub const MPC10X_MAPB_ISA_MEM_BASE: c_uint = 0x80000000;
pub const MPC10X_MAPB_DRAM_OFFSET: c_uint = 0x00000000;
pub const MPC10X_MAPB_PCI_INTACK_ADDR: c_uint = 0xfef00000;
pub const MPC10X_MAPB_PCI_IO_START: c_uint = 0x00000000;

pub const MPC10X_MAPB_PCI_MEM_START: c_uint = 0x80000000;

// Miscellaneous Configuration register offsets
pub const MPC10X_CFG_PIR_REG: c_uint = 0x09;
pub const MPC10X_CFG_PIR_HOST_BRIDGE: c_uint = 0x00;
pub const MPC10X_CFG_PIR_AGENT: c_uint = 0x01;
pub const MPC10X_CFG_EUMBBAR: c_uint = 0x78;
pub const MPC10X_CFG_PICR1_REG: c_uint = 0xa8;
pub const MPC10X_CFG_PICR1_ADDR_MAP_MASK: c_uint = 0x00010000;
pub const MPC10X_CFG_PICR1_ADDR_MAP_A: c_uint = 0x00010000;
pub const MPC10X_CFG_PICR1_ADDR_MAP_B: c_uint = 0x00000000;
pub const MPC10X_CFG_PICR1_SPEC_PCI_RD: c_uint = 0x00000004;
pub const MPC10X_CFG_PICR1_ST_GATH_EN: c_uint = 0x00000040;
pub const MPC10X_CFG_PICR2_REG: c_uint = 0xac;
pub const MPC10X_CFG_PICR2_COPYBACK_OPT: c_uint = 0x00000001;
pub const MPC10X_CFG_MAPB_OPTIONS_REG: c_uint = 0xe0;
pub const MPC10X_CFG_MAPB_OPTIONS_CFAE: c_uint = 0x80	/* CPU_FD_ALIAS_EN */;
pub const MPC10X_CFG_MAPB_OPTIONS_PFAE: c_uint = 0x40	/* PCI_FD_ALIAS_EN */;
pub const MPC10X_CFG_MAPB_OPTIONS_DR: c_uint = 0x20	/* DLL_RESET */;
pub const MPC10X_CFG_MAPB_OPTIONS_PCICH: c_uint = 0x08	/* PCI_COMPATIBILITY_HOLE */;
pub const MPC10X_CFG_MAPB_OPTIONS_PROCCH: c_uint = 0x04	/* PROC_COMPATIBILITY_HOLE */;
// Define offsets for the memory controller registers in the config space
pub const MPC10X_MCTLR_MEM_START_1: c_uint = 0x80	/* Banks 0-3 */;
pub const MPC10X_MCTLR_MEM_START_2: c_uint = 0x84	/* Banks 4-7 */;
pub const MPC10X_MCTLR_EXT_MEM_START_1: c_uint = 0x88	/* Banks 0-3 */;
pub const MPC10X_MCTLR_EXT_MEM_START_2: c_uint = 0x8c	/* Banks 4-7 */;
pub const MPC10X_MCTLR_MEM_END_1: c_uint = 0x90	/* Banks 0-3 */;
pub const MPC10X_MCTLR_MEM_END_2: c_uint = 0x94	/* Banks 4-7 */;
pub const MPC10X_MCTLR_EXT_MEM_END_1: c_uint = 0x98	/* Banks 0-3 */;
pub const MPC10X_MCTLR_EXT_MEM_END_2: c_uint = 0x9c	/* Banks 4-7 */;
pub const MPC10X_MCTLR_MEM_BANK_ENABLES: c_uint = 0xa0;
// Define some offset in the EUMB
pub const MPC10X_EUMB_SIZE: c_uint = 0x00100000 /* Total EUMB size (1MB) */;
pub const MPC10X_EUMB_MU_OFFSET: c_uint = 0x00000000 /* Msg Unit reg offset */;
pub const MPC10X_EUMB_MU_SIZE: c_uint = 0x00001000 /* Msg Unit reg size */;
pub const MPC10X_EUMB_DMA_OFFSET: c_uint = 0x00001000 /* DMA Unit reg offset */;
pub const MPC10X_EUMB_DMA_SIZE: c_uint = 0x00001000 /* DMA Unit reg size  */;
pub const MPC10X_EUMB_ATU_OFFSET: c_uint = 0x00002000 /* Addr xlate reg offset */;
pub const MPC10X_EUMB_ATU_SIZE: c_uint = 0x00001000 /* Addr xlate reg size  */;
pub const MPC10X_EUMB_I2C_OFFSET: c_uint = 0x00003000 /* I2C Unit reg offset */;
pub const MPC10X_EUMB_I2C_SIZE: c_uint = 0x00001000 /* I2C Unit reg size  */;
pub const MPC10X_EUMB_DUART_OFFSET: c_uint = 0x00004000 /* DUART Unit reg offset (8245) */;
pub const MPC10X_EUMB_DUART_SIZE: c_uint = 0x00001000 /* DUART Unit reg size (8245) */;
pub const MPC10X_EUMB_EPIC_OFFSET: c_uint = 0x00040000 /* EPIC offset in EUMB */;
pub const MPC10X_EUMB_EPIC_SIZE: c_uint = 0x00030000 /* EPIC size */;
pub const MPC10X_EUMB_PM_OFFSET: c_uint = 0x000fe000 /* Performance Monitor reg offset (8245) */;
pub const MPC10X_EUMB_PM_SIZE: c_uint = 0x00001000 /* Performance Monitor reg size (8245) */;
pub const MPC10X_EUMB_WP_OFFSET: c_uint = 0x000ff000 /* Data path diagnostic, watchpoint reg offset */;
pub const MPC10X_EUMB_WP_SIZE: c_uint = 0x00001000 /* Data path diagnostic, watchpoint reg size */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppc_sys_devices {
    MPC10X_IIC1,
    MPC10X_DMA0,
    MPC10X_DMA1,
    MPC10X_UART0,
    MPC10X_UART1,
    NUM_PPC_SYS_DEVS,
}

extern "C" {
    pub fn mpc10x_get_mem_size(mem_map: c_uint) -> c_ulong;
}
extern "C" {
    pub fn mpc10x_enable_store_gathering(hose: *mut pci_controller) -> c_int;
}
extern "C" {
    pub fn mpc10x_disable_store_gathering(hose: *mut pci_controller) -> c_int;
}
// For MPC107 boards that use the built-in openpic
extern "C" {
    pub fn mpc10x_set_openpic();
}
extern "C" {
    pub fn avr_uart_configure();
}
extern "C" {
    pub fn avr_uart_send(c: c_char);
}
