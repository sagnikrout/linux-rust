//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/r852.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright © 2009 - Maxim Levitsky
// driver for Ricoh xD readers
//

// nand interface + ecc
//
pub const R852_DATALINE: c_uint = 0x00;
// control register
pub const R852_CTL: c_uint = 0x04;
pub const R852_CTL_COMMAND: c_uint = 0x01	/* send command (#CLE)*/;
pub const R852_CTL_DATA: c_uint = 0x02	/* read/write data (#ALE)*/;
pub const R852_CTL_ON: c_uint = 0x04	/* only seem to controls the hd led, */;
// but has to be set on start...
pub const R852_CTL_RESET: c_uint = 0x08	/* unknown, set only on start once*/;
pub const R852_CTL_CARDENABLE: c_uint = 0x10	/* probably (#CE) - always set*/;
pub const R852_CTL_ECC_ENABLE: c_uint = 0x20	/* enable ecc engine */;
pub const R852_CTL_ECC_ACCESS: c_uint = 0x40	/* read/write ecc via reg #0*/;
pub const R852_CTL_WRITE: c_uint = 0x80	/* set when performing writes (#WP) */;
// card detection status
pub const R852_CARD_STA: c_uint = 0x05;
pub const R852_CARD_STA_CD: c_uint = 0x01	/* state of #CD line, same as 0x04 */;
pub const R852_CARD_STA_RO: c_uint = 0x02	/* card is readonly */;
pub const R852_CARD_STA_PRESENT: c_uint = 0x04	/* card is present (#CD) */;
pub const R852_CARD_STA_ABSENT: c_uint = 0x08	/* card is absent */;
pub const R852_CARD_STA_BUSY: c_uint = 0x80	/* card is busy - (#R/B) */;
// card detection irq status & enable
pub const R852_CARD_IRQ_STA: c_uint = 0x06	/* IRQ status */;
pub const R852_CARD_IRQ_ENABLE: c_uint = 0x07	/* IRQ enable */;
pub const R852_CARD_IRQ_CD: c_uint = 0x01	/* fire when #CD lights, same as 0x04*/;
pub const R852_CARD_IRQ_REMOVE: c_uint = 0x04	/* detect card removal */;
pub const R852_CARD_IRQ_INSERT: c_uint = 0x08	/* detect card insert */;
pub const R852_CARD_IRQ_UNK1: c_uint = 0x10	/* unknown */;
pub const R852_CARD_IRQ_GENABLE: c_uint = 0x80	/* general enable */;
pub const R852_CARD_IRQ_MASK: c_uint = 0x1D;
// hardware enable
pub const R852_HW: c_uint = 0x08;
pub const R852_HW_ENABLED: c_uint = 0x01	/* hw enabled */;
pub const R852_HW_UNKNOWN: c_uint = 0x80;
// dma capabilities
pub const R852_DMA_CAP: c_uint = 0x09;
pub const R852_SMBIT: c_uint = 0x20	/* if set with bit #6 or bit #7, then */;
// hw is smartmedia
pub const R852_DMA1: c_uint = 0x40	/* if set w/bit #7, dma is supported */;
pub const R852_DMA2: c_uint = 0x80	/* if set w/bit #6, dma is supported */;
// physical DMA address - 32 bit value
pub const R852_DMA_ADDR: c_uint = 0x0C;
// dma settings
pub const R852_DMA_SETTINGS: c_uint = 0x10;
pub const R852_DMA_MEMORY: c_uint = 0x01	/* (memory <-> internal hw buffer) */;
pub const R852_DMA_READ: c_uint = 0x02	/* 0 = write, 1 = read */;
pub const R852_DMA_INTERNAL: c_uint = 0x04	/* (internal hw buffer <-> card) */;
// dma IRQ status
pub const R852_DMA_IRQ_STA: c_uint = 0x14;
// dma IRQ enable
pub const R852_DMA_IRQ_ENABLE: c_uint = 0x18;
pub const R852_DMA_IRQ_MEMORY: c_uint = 0x01	/* (memory <-> internal hw buffer) */;
pub const R852_DMA_IRQ_ERROR: c_uint = 0x02	/* error did happen */;
pub const R852_DMA_IRQ_INTERNAL: c_uint = 0x04	/* (internal hw buffer <-> card) */;
pub const R852_DMA_IRQ_MASK: c_uint = 0x07	/* mask of all IRQ bits */;
// ECC syndrome format - read from reg #0 will return two copies of these for
pub const R852_ECC_ERR_BIT_MSK: c_uint = 0x07	/* error bit location */;
pub const R852_ECC_CORRECT: c_uint = 0x10	/* no errors - (guessed) */;
pub const R852_ECC_CORRECTABLE: c_uint = 0x20	/* correctable error exist */;
pub const R852_ECC_FAIL: c_uint = 0x40	/* non correctable error detected */;
pub const R852_DMA_LEN: c_int = 512;
pub const DMA_INTERNAL: c_int = 0;
pub const DMA_MEMORY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r852_device {
    pub controller: nand_controller,
    pub /: *mut *mut *mut void __iomem mmio; / mmio,
    pub /: *mut *mut *mut nand_chip chip; / nand chip backpointer,
    pub /: *mut *mut *mut pci_dev pci_dev; / pci backpointer,
// dma area
    pub buffer*/: *mut *mut dma_addr_t phys_dma_addr; / bus address of,
    pub /: *mut *mut completion dma_done; / data transfer done,
    pub /: *mut *mut dma_addr_t phys_bounce_buffer; / bus address of bounce buffer,
    pub /: *mut *mut *mut uint8_t bounce_buffer; / virtual address of bounce buffer,
    pub /: *mut *mut int dma_dir; / 1 = read, 0 = write,
    pub step,: *mut *mut int dma_stage; / 0 - idle, 1 - first,
    pub /: *mut *mut int dma_state; / 0 = internal, 1 = memory,
    pub /: *mut *mut int dma_error; / dma errors,
    pub /: *mut *mut int dma_usable; / is it possible to use dma,
// card status area
    pub card_detect_work: delayed_work,
    pub card_workqueue: *mut workqueue_struct,
    pub /: *mut *mut int card_registered; / card registered with mtd,
    pub /: *mut *mut int card_detected; / card detected in slot,
    pub inserted,: *mut *mut int card_unstable; / whenever the card is,
    pub /: *mut *mut int readonly; / card is readonly,
    pub /: *mut *mut int sm; / Is card smartmedia,
// interrupt handling
    pub /: *mut *mut spinlock_t irqlock; / IRQ protecting lock,
    pub /: *mut *mut int irq; / irq num,
// misc
    pub /: *mut *mut *mut void tmp_buffer; / temporary buffer,
    pub /: *mut *mut uint8_t ctlreg; / cached contents of control reg,
}

