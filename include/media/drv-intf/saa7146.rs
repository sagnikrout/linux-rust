//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/saa7146.h
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

// simple debug messages

// more detailed debug messages

// print enter and exit of functions

// i2c debug messages

// vbi debug messages

// interrupt debug messages

// capture debug messages

// saa7146 page table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_pgtable {
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub dma: dma_addr_t,
// used for offsets for u,v planes for planar capture modes
    pub offset: c_ulong,
// used for custom pagetables (used for example by budget dvb cards)
    pub slist: *mut scatterlist,
    pub nents: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_pci_extension_data {
    pub ext: *mut saa7146_extension,
    pub /: *mut *mut *mut void ext_priv; / most likely a name string,
}

pub const SAA7146_USE_I2C_IRQ: c_uint = 0x1;
pub const SAA7146_I2C_SHORT_DELAY: c_uint = 0x2;
// pairs of subvendor and subdevice ids for
// extension functions
// different device locks
// pci-device & irq stuff
// extension handling
// per device video/vbi information (if available)
// i2c-stuff
// memories
extern "C" {
    pub fn container_of(_arg: v4l2_dev, saa7146_dev: struct, _arg: v4l2_dev) -> return;
}
// from saa7146_i2c.c
extern "C" {
    pub fn saa7146_i2c_adapter_prepare(dev: *mut saa7146_dev, i2c_adapter: *mut i2c_adapter, bitrate: u32) -> c_int;
}
// from saa7146_core.c
extern "C" {
    pub fn saa7146_register_extension(saa7146_extension*: *mut struct) -> c_int;
}
extern "C" {
    pub fn saa7146_unregister_extension(saa7146_extension*: *mut struct) -> c_int;
}
extern "C" {
    pub fn saa7146_format_by_fourcc(dev: *mut saa7146_dev, fourcc: c_int) -> *mut saa7146_format;
}
extern "C" {
    pub fn saa7146_pgtable_alloc(pci: *mut pci_dev, pt: *mut saa7146_pgtable) -> c_int;
}
extern "C" {
    pub fn saa7146_pgtable_free(pci: *mut pci_dev, pt: *mut saa7146_pgtable);
}
extern "C" {
    pub fn saa7146_pgtable_build_single(pci: *mut pci_dev, pt: *mut saa7146_pgtable, list: *mut scatterlist, length: c_int) -> c_int;
}
extern "C" {
    pub fn saa7146_vfree_destroy_pgtable(pci: *mut pci_dev, mem: *mut c_void, pt: *mut saa7146_pgtable);
}
extern "C" {
    pub fn saa7146_setgpio(dev: *mut saa7146_dev, port: c_int, data: u32);
}
extern "C" {
    pub fn saa7146_wait_for_debi_done(dev: *mut saa7146_dev, nobusyloop: c_int) -> c_int;
}
// some memory sizes

// some i2c constants

// unsorted defines
pub const ME1: c_uint = 0x0000000800;
pub const PV1: c_uint = 0x0000000008;
// gpio defines
pub const SAA7146_GPIO_INPUT: c_uint = 0x00;
pub const SAA7146_GPIO_IRQHI: c_uint = 0x10;
pub const SAA7146_GPIO_IRQLO: c_uint = 0x20;
pub const SAA7146_GPIO_IRQHL: c_uint = 0x30;
pub const SAA7146_GPIO_OUTLO: c_uint = 0x40;
pub const SAA7146_GPIO_OUTHI: c_uint = 0x50;
// debi defines
pub const DEBINOSWAP: c_uint = 0x000e0000;
// define for the register programming sequencer (rps)
pub const CMD_NOP: c_uint = 0x00000000  /* No operation */;
pub const CMD_CLR_EVENT: c_uint = 0x00000000  /* Clear event */;
pub const CMD_SET_EVENT: c_uint = 0x10000000  /* Set signal event */;
pub const CMD_PAUSE: c_uint = 0x20000000  /* Pause */;
pub const CMD_CHECK_LATE: c_uint = 0x30000000  /* Check late */;
pub const CMD_UPLOAD: c_uint = 0x40000000  /* Upload */;
pub const CMD_STOP: c_uint = 0x50000000  /* Stop */;
pub const CMD_INTERRUPT: c_uint = 0x60000000  /* Interrupt */;
pub const CMD_JUMP: c_uint = 0x80000000  /* Jump */;
pub const CMD_WR_REG: c_uint = 0x90000000  /* Write (load) register */;
pub const CMD_RD_REG: c_uint = 0xa0000000  /* Read (store) register */;
pub const CMD_WR_REG_MASK: c_uint = 0xc0000000  /* Write register with mask */;

// some events and command modifiers for rps1 squarewave generator

pub const GPIO3_MSK: c_uint = 0xFF000000  // GPIO #3 control bits;
// Bit mask constants
pub const MASK_00: c_uint = 0x00000001    /* Mask value for bit 0 */;
pub const MASK_01: c_uint = 0x00000002    /* Mask value for bit 1 */;
pub const MASK_02: c_uint = 0x00000004    /* Mask value for bit 2 */;
pub const MASK_03: c_uint = 0x00000008    /* Mask value for bit 3 */;
pub const MASK_04: c_uint = 0x00000010    /* Mask value for bit 4 */;
pub const MASK_05: c_uint = 0x00000020    /* Mask value for bit 5 */;
pub const MASK_06: c_uint = 0x00000040    /* Mask value for bit 6 */;
pub const MASK_07: c_uint = 0x00000080    /* Mask value for bit 7 */;
pub const MASK_08: c_uint = 0x00000100    /* Mask value for bit 8 */;
pub const MASK_09: c_uint = 0x00000200    /* Mask value for bit 9 */;
pub const MASK_10: c_uint = 0x00000400    /* Mask value for bit 10 */;
pub const MASK_11: c_uint = 0x00000800    /* Mask value for bit 11 */;
pub const MASK_12: c_uint = 0x00001000    /* Mask value for bit 12 */;
pub const MASK_13: c_uint = 0x00002000    /* Mask value for bit 13 */;
pub const MASK_14: c_uint = 0x00004000    /* Mask value for bit 14 */;
pub const MASK_15: c_uint = 0x00008000    /* Mask value for bit 15 */;
pub const MASK_16: c_uint = 0x00010000    /* Mask value for bit 16 */;
pub const MASK_17: c_uint = 0x00020000    /* Mask value for bit 17 */;
pub const MASK_18: c_uint = 0x00040000    /* Mask value for bit 18 */;
pub const MASK_19: c_uint = 0x00080000    /* Mask value for bit 19 */;
pub const MASK_20: c_uint = 0x00100000    /* Mask value for bit 20 */;
pub const MASK_21: c_uint = 0x00200000    /* Mask value for bit 21 */;
pub const MASK_22: c_uint = 0x00400000    /* Mask value for bit 22 */;
pub const MASK_23: c_uint = 0x00800000    /* Mask value for bit 23 */;
pub const MASK_24: c_uint = 0x01000000    /* Mask value for bit 24 */;
pub const MASK_25: c_uint = 0x02000000    /* Mask value for bit 25 */;
pub const MASK_26: c_uint = 0x04000000    /* Mask value for bit 26 */;
pub const MASK_27: c_uint = 0x08000000    /* Mask value for bit 27 */;
pub const MASK_28: c_uint = 0x10000000    /* Mask value for bit 28 */;
pub const MASK_29: c_uint = 0x20000000    /* Mask value for bit 29 */;
pub const MASK_30: c_uint = 0x40000000    /* Mask value for bit 30 */;
pub const MASK_31: c_uint = 0x80000000    /* Mask value for bit 31 */;
pub const MASK_B0: c_uint = 0x000000ff    /* Mask value for byte 0 */;
pub const MASK_B1: c_uint = 0x0000ff00    /* Mask value for byte 1 */;
pub const MASK_B2: c_uint = 0x00ff0000    /* Mask value for byte 2 */;
pub const MASK_B3: c_uint = 0xff000000    /* Mask value for byte 3 */;
pub const MASK_W0: c_uint = 0x0000ffff    /* Mask value for word 0 */;
pub const MASK_W1: c_uint = 0xffff0000    /* Mask value for word 1 */;
pub const MASK_PA: c_uint = 0xfffffffc    /* Mask value for physical address */;
pub const MASK_PR: c_uint = 0xfffffffe	/* Mask value for protection register */;
pub const MASK_ER: c_uint = 0xffffffff    /* Mask value for the entire register */;
pub const MASK_NONE: c_uint = 0x00000000    /* No mask */;
// register aliases
pub const BASE_ODD1: c_uint = 0x00  /* Video DMA 1 registers  */;
pub const BASE_EVEN1: c_uint = 0x04;
pub const PROT_ADDR1: c_uint = 0x08;
pub const PITCH1: c_uint = 0x0C;
pub const BASE_PAGE1: c_uint = 0x10  /* Video DMA 1 base page */;
pub const NUM_LINE_BYTE1: c_uint = 0x14;
pub const BASE_ODD2: c_uint = 0x18  /* Video DMA 2 registers */;
pub const BASE_EVEN2: c_uint = 0x1C;
pub const PROT_ADDR2: c_uint = 0x20;
pub const PITCH2: c_uint = 0x24;
pub const BASE_PAGE2: c_uint = 0x28  /* Video DMA 2 base page */;
pub const NUM_LINE_BYTE2: c_uint = 0x2C;
pub const BASE_ODD3: c_uint = 0x30  /* Video DMA 3 registers */;
pub const BASE_EVEN3: c_uint = 0x34;
pub const PROT_ADDR3: c_uint = 0x38;
pub const PITCH3: c_uint = 0x3C;
pub const BASE_PAGE3: c_uint = 0x40  /* Video DMA 3 base page */;
pub const NUM_LINE_BYTE3: c_uint = 0x44;
pub const PCI_BT_V1: c_uint = 0x48  /* Video/FIFO 1 */;
pub const PCI_BT_V2: c_uint = 0x49  /* Video/FIFO 2 */;
pub const PCI_BT_V3: c_uint = 0x4A  /* Video/FIFO 3 */;
pub const PCI_BT_DEBI: c_uint = 0x4B  /* DEBI */;
pub const PCI_BT_A: c_uint = 0x4C  /* Audio */;
pub const DD1_INIT: c_uint = 0x50  /* Init setting of DD1 interface */;
pub const DD1_STREAM_B: c_uint = 0x54  /* DD1 B video data stream handling */;
pub const DD1_STREAM_A: c_uint = 0x56  /* DD1 A video data stream handling */;
pub const BRS_CTRL: c_uint = 0x58  /* BRS control register */;
pub const HPS_CTRL: c_uint = 0x5C  /* HPS control register */;
pub const HPS_V_SCALE: c_uint = 0x60  /* HPS vertical scale */;
pub const HPS_V_GAIN: c_uint = 0x64  /* HPS vertical ACL and gain */;
pub const HPS_H_PRESCALE: c_uint = 0x68  /* HPS horizontal prescale   */;
pub const HPS_H_SCALE: c_uint = 0x6C  /* HPS horizontal scale */;
pub const BCS_CTRL: c_uint = 0x70  /* BCS control */;
pub const CHROMA_KEY_RANGE: c_uint = 0x74;
pub const CLIP_FORMAT_CTRL: c_uint = 0x78  /* HPS outputs formats & clipping */;
pub const DEBI_CONFIG: c_uint = 0x7C;
pub const DEBI_COMMAND: c_uint = 0x80;
pub const DEBI_PAGE: c_uint = 0x84;
pub const DEBI_AD: c_uint = 0x88;
pub const I2C_TRANSFER: c_uint = 0x8C;
pub const I2C_STATUS: c_uint = 0x90;
pub const BASE_A1_IN: c_uint = 0x94	/* Audio 1 input DMA */;
pub const PROT_A1_IN: c_uint = 0x98;
pub const PAGE_A1_IN: c_uint = 0x9C;
pub const BASE_A1_OUT: c_uint = 0xA0  /* Audio 1 output DMA */;
pub const PROT_A1_OUT: c_uint = 0xA4;
pub const PAGE_A1_OUT: c_uint = 0xA8;
pub const BASE_A2_IN: c_uint = 0xAC  /* Audio 2 input DMA */;
pub const PROT_A2_IN: c_uint = 0xB0;
pub const PAGE_A2_IN: c_uint = 0xB4;
pub const BASE_A2_OUT: c_uint = 0xB8  /* Audio 2 output DMA */;
pub const PROT_A2_OUT: c_uint = 0xBC;
pub const PAGE_A2_OUT: c_uint = 0xC0;
pub const RPS_PAGE0: c_uint = 0xC4  /* RPS task 0 page register */;
pub const RPS_PAGE1: c_uint = 0xC8  /* RPS task 1 page register */;
pub const RPS_THRESH0: c_uint = 0xCC  /* HBI threshold for task 0 */;
pub const RPS_THRESH1: c_uint = 0xD0  /* HBI threshold for task 1 */;
pub const RPS_TOV0: c_uint = 0xD4  /* RPS timeout for task 0 */;
pub const RPS_TOV1: c_uint = 0xD8  /* RPS timeout for task 1 */;
pub const IER: c_uint = 0xDC  /* Interrupt enable register */;
pub const GPIO_CTRL: c_uint = 0xE0  /* GPIO 0-3 register */;
pub const EC1SSR: c_uint = 0xE4  /* Event cnt set 1 source select */;
pub const EC2SSR: c_uint = 0xE8  /* Event cnt set 2 source select */;
pub const ECT1R: c_uint = 0xEC  /* Event cnt set 1 thresholds */;
pub const ECT2R: c_uint = 0xF0  /* Event cnt set 2 thresholds */;
pub const ACON1: c_uint = 0xF4;
pub const ACON2: c_uint = 0xF8;
pub const MC1: c_uint = 0xFC   /* Main control register 1 */;
pub const MC2: c_uint = 0x100  /* Main control register 2  */;
pub const RPS_ADDR0: c_uint = 0x104  /* RPS task 0 address register */;
pub const RPS_ADDR1: c_uint = 0x108  /* RPS task 1 address register */;
pub const ISR: c_uint = 0x10C  /* Interrupt status register */;
pub const PSR: c_uint = 0x110  /* Primary status register */;
pub const SSR: c_uint = 0x114  /* Secondary status register */;
pub const EC1R: c_uint = 0x118  /* Event counter set 1 register */;
pub const EC2R: c_uint = 0x11C  /* Event counter set 2 register */;
pub const PCI_VDP1: c_uint = 0x120  /* Video DMA pointer of FIFO 1 */;
pub const PCI_VDP2: c_uint = 0x124  /* Video DMA pointer of FIFO 2 */;
pub const PCI_VDP3: c_uint = 0x128  /* Video DMA pointer of FIFO 3 */;
pub const PCI_ADP1: c_uint = 0x12C  /* Audio DMA pointer of audio out 1 */;
pub const PCI_ADP2: c_uint = 0x130  /* Audio DMA pointer of audio in 1 */;
pub const PCI_ADP3: c_uint = 0x134  /* Audio DMA pointer of audio out 2 */;
pub const PCI_ADP4: c_uint = 0x138  /* Audio DMA pointer of audio in 2 */;
pub const PCI_DMA_DDP: c_uint = 0x13C  /* DEBI DMA pointer */;
pub const LEVEL_REP: c_uint = 0x140,;
pub const A_TIME_SLOT1: c_uint = 0x180,  /* from 180 - 1BC */;
pub const A_TIME_SLOT2: c_uint = 0x1C0,  /* from 1C0 - 1FC */;
// isr masks
pub const SPCI_PPEF: c_uint = 0x80000000  /* PCI parity error */;
pub const SPCI_PABO: c_uint = 0x40000000  /* PCI access error (target or master abort) */;
pub const SPCI_PPED: c_uint = 0x20000000  /* PCI parity error on 'real time data' */;
pub const SPCI_RPS_I1: c_uint = 0x10000000  /* Interrupt issued by RPS1 */;
pub const SPCI_RPS_I0: c_uint = 0x08000000  /* Interrupt issued by RPS0 */;
pub const SPCI_RPS_LATE1: c_uint = 0x04000000  /* RPS task 1 is late */;
pub const SPCI_RPS_LATE0: c_uint = 0x02000000  /* RPS task 0 is late */;
pub const SPCI_RPS_E1: c_uint = 0x01000000  /* RPS error from task 1 */;
pub const SPCI_RPS_E0: c_uint = 0x00800000  /* RPS error from task 0 */;
pub const SPCI_RPS_TO1: c_uint = 0x00400000  /* RPS timeout task 1 */;
pub const SPCI_RPS_TO0: c_uint = 0x00200000  /* RPS timeout task 0 */;
pub const SPCI_UPLD: c_uint = 0x00100000  /* RPS in upload */;
pub const SPCI_DEBI_S: c_uint = 0x00080000  /* DEBI status */;
pub const SPCI_DEBI_E: c_uint = 0x00040000  /* DEBI error */;
pub const SPCI_IIC_S: c_uint = 0x00020000  /* I2C status */;
pub const SPCI_IIC_E: c_uint = 0x00010000  /* I2C error */;
pub const SPCI_A2_IN: c_uint = 0x00008000  /* Audio 2 input DMA protection / limit */;
pub const SPCI_A2_OUT: c_uint = 0x00004000  /* Audio 2 output DMA protection / limit */;
pub const SPCI_A1_IN: c_uint = 0x00002000  /* Audio 1 input DMA protection / limit */;
pub const SPCI_A1_OUT: c_uint = 0x00001000  /* Audio 1 output DMA protection / limit */;
pub const SPCI_AFOU: c_uint = 0x00000800  /* Audio FIFO over- / underflow */;
pub const SPCI_V_PE: c_uint = 0x00000400  /* Video protection address */;
pub const SPCI_VFOU: c_uint = 0x00000200  /* Video FIFO over- / underflow */;
pub const SPCI_FIDA: c_uint = 0x00000100  /* Field ID video port A */;
pub const SPCI_FIDB: c_uint = 0x00000080  /* Field ID video port B */;
pub const SPCI_PIN3: c_uint = 0x00000040  /* GPIO pin 3 */;
pub const SPCI_PIN2: c_uint = 0x00000020  /* GPIO pin 2 */;
pub const SPCI_PIN1: c_uint = 0x00000010  /* GPIO pin 1 */;
pub const SPCI_PIN0: c_uint = 0x00000008  /* GPIO pin 0 */;
pub const SPCI_ECS: c_uint = 0x00000004  /* Event counter 1, 2, 4, 5 */;
pub const SPCI_EC3S: c_uint = 0x00000002  /* Event counter 3 */;
pub const SPCI_EC0S: c_uint = 0x00000001  /* Event counter 0 */;
// i2c

