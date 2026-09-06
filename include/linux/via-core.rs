//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/via-core.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
// Copyright 2009-2010 Jonathan Corbet <corbet@lwn.net>
// Copyright 2010 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
//

//
// A description of each known serial I2C/GPIO port.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum via_port_type {
    VIA_PORT_NONE = 0,
    VIA_PORT_I2C,
    VIA_PORT_GPIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum via_port_mode {
    VIA_MODE_OFF = 0,
    VIA_MODE_I2C,		/* Used as I2C port */
    VIA_MODE_GPIO,	/* Two GPIO ports */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viafb_i2c_adap {
    VIA_PORT_26 = 0,
    VIA_PORT_31,
    VIA_PORT_25,
    VIA_PORT_2C,
    VIA_PORT_3D,
}

pub const VIAFB_NUM_PORTS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_port_cfg {
    pub type: via_port_type,
    pub mode: via_port_mode,
    pub io_port: u16,
    pub ioport_index: u8,
}

//
// Allow subdevs to register suspend/resume hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_pm_hooks {
    pub list: list_head,
    pub private): *mut *mut int (suspend)(void,
    pub private): *mut *mut int (resume)(void,
    pub private: *mut c_void,
}

extern "C" {
    pub fn viafb_pm_register(hooks: *mut viafb_pm_hooks);
}
extern "C" {
    pub fn viafb_pm_unregister(hooks: *mut viafb_pm_hooks);
}
//
// This is the global viafb "device" containing stuff needed by
// all subdevs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_dev {
    pub pdev: *mut pci_dev,
    pub chip_type: c_int,
    pub port_cfg: *mut via_port_cfg,
//
// Spinlock for access to device registers.  Not yet
// globally used.
//
    pub reg_lock: spinlock_t,
//
// The framebuffer MMIO region.  Little, if anything, touches
// this memory directly, and certainly nothing outside of the
// framebuffer device itself.  We *do* have to be able to allocate
// chunks of this memory for other devices, though.
//
    pub fbmem_start: c_ulong,
    pub fbmem_len: c_long,
    pub fbmem: *mut void __iomem,

    pub camera_fbmem_offset: c_long,
    pub camera_fbmem_size: c_long,

//
// The MMIO region for device registers.
//
    pub engine_start: c_ulong,
    pub engine_len: c_ulong,
    pub engine_mmio: *mut void __iomem,
}

//
// Interrupt management.
//
extern "C" {
    pub fn viafb_irq_enable(mask: u32);
}
extern "C" {
    pub fn viafb_irq_disable(mask: u32);
}
//
// The global interrupt control register and its bits.
//
pub const VDE_INTERRUPT: c_uint = 0x200	/* Video interrupt flags/masks */;
pub const VDE_I_DVISENSE: c_uint = 0x00000001  /* DVI sense int status */;
pub const VDE_I_VBLANK: c_uint = 0x00000002  /* Vertical blank status */;
pub const VDE_I_MCCFI: c_uint = 0x00000004  /* MCE compl. frame int status */;
pub const VDE_I_VSYNC: c_uint = 0x00000008  /* VGA VSYNC int status */;
pub const VDE_I_DMA0DDONE: c_uint = 0x00000010  /* DMA 0 descr done */;
pub const VDE_I_DMA0TDONE: c_uint = 0x00000020  /* DMA 0 transfer done */;
pub const VDE_I_DMA1DDONE: c_uint = 0x00000040  /* DMA 1 descr done */;
pub const VDE_I_DMA1TDONE: c_uint = 0x00000080  /* DMA 1 transfer done */;
pub const VDE_I_C1AV: c_uint = 0x00000100  /* Cap Eng 1 act vid end */;
pub const VDE_I_HQV0: c_uint = 0x00000200  /* First HQV engine */;
pub const VDE_I_HQV1: c_uint = 0x00000400  /* Second HQV engine */;
pub const VDE_I_HQV1EN: c_uint = 0x00000800  /* Second HQV engine enable */;
pub const VDE_I_C0AV: c_uint = 0x00001000  /* Cap Eng 0 act vid end */;
pub const VDE_I_C0VBI: c_uint = 0x00002000  /* Cap Eng 0 VBI end */;
pub const VDE_I_C1VBI: c_uint = 0x00004000  /* Cap Eng 1 VBI end */;
pub const VDE_I_VSYNC2: c_uint = 0x00008000  /* Sec. Disp. VSYNC */;
pub const VDE_I_DVISNSEN: c_uint = 0x00010000  /* DVI sense enable */;
pub const VDE_I_VSYNC2EN: c_uint = 0x00020000  /* Sec Disp VSYNC enable */;
pub const VDE_I_MCCFIEN: c_uint = 0x00040000  /* MC comp frame int mask enable */;
pub const VDE_I_VSYNCEN: c_uint = 0x00080000  /* VSYNC enable */;
pub const VDE_I_DMA0DDEN: c_uint = 0x00100000  /* DMA 0 descr done enable */;
pub const VDE_I_DMA0TDEN: c_uint = 0x00200000  /* DMA 0 trans done enable */;
pub const VDE_I_DMA1DDEN: c_uint = 0x00400000  /* DMA 1 descr done enable */;
pub const VDE_I_DMA1TDEN: c_uint = 0x00800000  /* DMA 1 trans done enable */;
pub const VDE_I_C1AVEN: c_uint = 0x01000000  /* cap 1 act vid end enable */;
pub const VDE_I_HQV0EN: c_uint = 0x02000000  /* First hqv engine enable */;
pub const VDE_I_C1VBIEN: c_uint = 0x04000000  /* Cap 1 VBI end enable */;
pub const VDE_I_LVDSSI: c_uint = 0x08000000  /* LVDS sense interrupt */;
pub const VDE_I_C0AVEN: c_uint = 0x10000000  /* Cap 0 act vid end enable */;
pub const VDE_I_C0VBIEN: c_uint = 0x20000000  /* Cap 0 VBI end enable */;
pub const VDE_I_LVDSSIEN: c_uint = 0x40000000  /* LVDS Sense enable */;
pub const VDE_I_ENABLE: c_uint = 0x80000000  /* Global interrupt enable */;

//
// DMA management.
//
extern "C" {
    pub fn viafb_request_dma() -> c_int;
}
extern "C" {
    pub fn viafb_release_dma();
}
// void viafb_dma_copy_out(unsigned int offset, dma_addr_t paddr, int len);
extern "C" {
    pub fn viafb_dma_copy_out_sg(offset: c_uint, sg: *mut scatterlist, nsg: c_int) -> c_int;
}
//
// DMA Controller registers.
//
pub const VDMA_MR0: c_uint = 0xe00		/* Mod reg 0 */;
pub const VDMA_MR_CHAIN: c_uint = 0x01		/* Chaining mode */;
pub const VDMA_MR_TDIE: c_uint = 0x02		/* Transfer done int enable */;
pub const VDMA_CSR0: c_uint = 0xe04		/* Control/status */;
pub const VDMA_C_ENABLE: c_uint = 0x01		  /* DMA Enable */;
pub const VDMA_C_START: c_uint = 0x02		  /* Start a transfer */;
pub const VDMA_C_ABORT: c_uint = 0x04		  /* Abort a transfer */;
pub const VDMA_C_DONE: c_uint = 0x08		  /* Transfer is done */;
pub const VDMA_MARL0: c_uint = 0xe20		/* Mem addr low */;
pub const VDMA_MARH0: c_uint = 0xe24		/* Mem addr high */;
pub const VDMA_DAR0: c_uint = 0xe28		/* Device address */;
pub const VDMA_DQWCR0: c_uint = 0xe2c		/* Count (16-byte) */;
pub const VDMA_TMR0: c_uint = 0xe30		/* Tile mode reg */;
pub const VDMA_DPRL0: c_uint = 0xe34		/* Not sure */;
pub const VDMA_DPR_IN: c_uint = 0x08		/* Inbound transfer to FB */;
pub const VDMA_DPRH0: c_uint = 0xe38;

//
// Useful stuff that probably belongs somewhere global.
//
pub const VGA_WIDTH: c_int = 640;
pub const VGA_HEIGHT: c_int = 480;

//
// Indexed port operations.  Note that these are all multi-op
// functions; every invocation will be racy if you're not holding
// reg_lock.
//
pub const VIAStatus: c_uint = 0x3DA  /* Non-indexed port */;
pub const VIACR: c_uint = 0x3D4;
pub const VIASR: c_uint = 0x3C4;
pub const VIAGR: c_uint = 0x3CE;
pub const VIAAR: c_uint = 0x3C0;
extern "C" {
    pub fn inb(1: port +) -> return;
}
pub const VIA_MISC_REG_READ: c_uint = 0x03CC;
pub const VIA_MISC_REG_WRITE: c_uint = 0x03C2;
