//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/dma.h
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
// linux/include/asm/dma.h: Defines for using and allocating dma channels.
// Written by Hennus Bergman, 1992.
// High DMA channel support & info by Hannu Savolainen
// and John Boyd, Nov. 1992.
//

//
// NOTES about DMA transfers:
//
// controller 1: channels 0-3, byte operations, ports 00-1F
// controller 2: channels 4-7, word operations, ports C0-DF
//
// - ALL registers are 8 bits only, regardless of transfer size
// - channel 4 is not used - cascades 1 into 2.
// - channels 0-3 are byte - addresses/counts are for physical bytes
// - channels 5-7 are word - addresses/counts are for physical words
// - transfers must not cross physical 64K (0-3) or 128K (5-7) boundaries
// - transfer count loaded to registers is 1 less than actual count
// - controller 2 offsets are all even (2x offsets for controller 1)
// - page registers for 5-7 don't use data bit 0, represent 128K pages
// - page registers for 0-3 use bit 0, represent 64K pages
//
// DMA transfers are limited to the lower 16MB of _physical_ memory.
// Note that addresses loaded into registers must be _physical_ addresses,
// not logical addresses (which may differ if paging is active).
//
// Address mapping for channels 0-3:
//
// A23 ... A16 A15 ... A8  A7 ... A0    (Physical addresses)
// |  ...  |   |  ... |   |  ... |
// P7  ...  P0  A7 ... A0  A7 ... A0
// |    Page    | Addr MSB | Addr LSB |   (DMA registers)
//
// Address mapping for channels 5-7:
//
// A23 ... A17 A16 A15 ... A9 A8 A7 ... A1 A0    (Physical addresses)
// |  ...  |   \   \   ... \  \  \  ... \  \
// |  ...  |    \   \   ... \  \  \  ... \  (not used)
// |  ...  |     \   \   ... \  \  \  ... \
// P7  ...  P1 (0) A7 A6  ... A0 A7 A6 ... A0
// |      Page      |  Addr MSB   |  Addr LSB  |   (DMA registers)
//
// Again, channels 5-7 transfer _physical_ words (16 bits), so addresses
// and counts _must_ be word-aligned (the lowest address bit is _ignored_ at
// the hardware level, so odd-byte transfers aren't possible).
//
// Transfer count (_not # bytes_) is limited to 64K, represented as actual
// count - 1 : 64K => 0xFFFF, 1 => 0x0000.  Thus, count is always 1 or more,
// and up to 128K bytes may be transferred on channels 5-7 in one operation.
//
pub const MAX_DMA_CHANNELS: c_int = 8;
// 16MB ISA DMA zone

// 4GB broken PCI/AGP hardware bus master zone

// The maximum address that we can perform a DMA transfer to on this platform

// Compat define for old dma zone

// 8237 DMA controllers
pub const IO_DMA1_BASE: c_uint = 0x00	/* 8 bit slave DMA, channels 0..3 */;
pub const IO_DMA2_BASE: c_uint = 0xC0	/* 16 bit master DMA, ch 4(=slave input)..7 */;
// DMA controller registers
pub const DMA1_CMD_REG: c_uint = 0x08	/* command register (w) */;
pub const DMA1_STAT_REG: c_uint = 0x08	/* status register (r) */;
pub const DMA1_REQ_REG: c_uint = 0x09    /* request register (w) */;
pub const DMA1_MASK_REG: c_uint = 0x0A	/* single-channel mask (w) */;
pub const DMA1_MODE_REG: c_uint = 0x0B	/* mode register (w) */;
pub const DMA1_CLEAR_FF_REG: c_uint = 0x0C	/* clear pointer flip-flop (w) */;
pub const DMA1_TEMP_REG: c_uint = 0x0D    /* Temporary Register (r) */;
pub const DMA1_RESET_REG: c_uint = 0x0D	/* Master Clear (w) */;
pub const DMA1_CLR_MASK_REG: c_uint = 0x0E    /* Clear Mask */;
pub const DMA1_MASK_ALL_REG: c_uint = 0x0F    /* all-channels mask (w) */;
pub const DMA2_CMD_REG: c_uint = 0xD0	/* command register (w) */;
pub const DMA2_STAT_REG: c_uint = 0xD0	/* status register (r) */;
pub const DMA2_REQ_REG: c_uint = 0xD2    /* request register (w) */;
pub const DMA2_MASK_REG: c_uint = 0xD4	/* single-channel mask (w) */;
pub const DMA2_MODE_REG: c_uint = 0xD6	/* mode register (w) */;
pub const DMA2_CLEAR_FF_REG: c_uint = 0xD8	/* clear pointer flip-flop (w) */;
pub const DMA2_TEMP_REG: c_uint = 0xDA    /* Temporary Register (r) */;
pub const DMA2_RESET_REG: c_uint = 0xDA	/* Master Clear (w) */;
pub const DMA2_CLR_MASK_REG: c_uint = 0xDC    /* Clear Mask */;
pub const DMA2_MASK_ALL_REG: c_uint = 0xDE    /* all-channels mask (w) */;
pub const DMA_ADDR_0: c_uint = 0x00    /* DMA address registers */;
pub const DMA_ADDR_1: c_uint = 0x02;
pub const DMA_ADDR_2: c_uint = 0x04;
pub const DMA_ADDR_3: c_uint = 0x06;
pub const DMA_ADDR_4: c_uint = 0xC0;
pub const DMA_ADDR_5: c_uint = 0xC4;
pub const DMA_ADDR_6: c_uint = 0xC8;
pub const DMA_ADDR_7: c_uint = 0xCC;
pub const DMA_CNT_0: c_uint = 0x01    /* DMA count registers */;
pub const DMA_CNT_1: c_uint = 0x03;
pub const DMA_CNT_2: c_uint = 0x05;
pub const DMA_CNT_3: c_uint = 0x07;
pub const DMA_CNT_4: c_uint = 0xC2;
pub const DMA_CNT_5: c_uint = 0xC6;
pub const DMA_CNT_6: c_uint = 0xCA;
pub const DMA_CNT_7: c_uint = 0xCE;
pub const DMA_PAGE_0: c_uint = 0x87    /* DMA page registers */;
pub const DMA_PAGE_1: c_uint = 0x83;
pub const DMA_PAGE_2: c_uint = 0x81;
pub const DMA_PAGE_3: c_uint = 0x82;
pub const DMA_PAGE_5: c_uint = 0x8B;
pub const DMA_PAGE_6: c_uint = 0x89;
pub const DMA_PAGE_7: c_uint = 0x8A;
// I/O to memory, no autoinit, increment, single mode
pub const DMA_MODE_READ: c_uint = 0x44;
// memory to I/O, no autoinit, increment, single mode
pub const DMA_MODE_WRITE: c_uint = 0x48;
// pass thru DREQ->HRQ, DACK<-HLDA only
pub const DMA_MODE_CASCADE: c_uint = 0xC0;
pub const DMA_AUTOINIT: c_uint = 0x10;

// enable/disable a specific DMA channel
// Clear the 'DMA Pointer Flip Flop'.
// Write 0 for LSB/MSB, 1 for MSB/LSB access.
// Use this once to initialize the FF to a known state.
// After that, keep track of it. :-)
// --- In order to do that, the DMA routines below should ---
// --- only be used while holding the DMA lock ! ---
//
// set mode (above) for a specific DMA channel
// Set only the page register bits of the transfer address.
// This is used for successive transfers when we know the contents of
// the lower 16 bits of the DMA current address register, but a 64k boundary
// may have been crossed.
//
// Set transfer address & page bits for specific DMA channel.
// Assumes dma flipflop is clear.
//
// Set transfer size (max 64k for DMA0..3, 128k for DMA5..7) for
// a specific DMA channel.
// You must ensure the parameters are valid.
// NOTE: from a manual: "the number of transfers is one more
// than the initial word count"! This is taken into account.
// Assumes dma flip-flop is clear.
// NOTE 2: "count" represents _bytes_ and must be even for channels 5-7.
//
// Get DMA residue count. After a DMA transfer, this
// should return zero. Reading this while a DMA transfer is
// still in progress will return unpredictable results.
// If called before the channel has been used, it may return 1.
// Otherwise, it returns the number of _bytes_ left to transfer.
//
// Assumes DMA flip-flop is clear.
//
// using short to get 16-bit wrap around
// These are in kernel/dma.c because x86 uses CONFIG_GENERIC_ISA_DMA

extern "C" {
    pub fn request_dma(dmanr: c_uint, device_id: *const c_char) -> c_int;
}
extern "C" {
    pub fn free_dma(dmanr: c_uint);
}

