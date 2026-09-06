//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pci_regs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// PCI standard defines
// Copyright 1994, Drew Eckhardt
// Copyright 1997--1999 Martin Mares <mj@ucw.cz>
//
// For more information, please consult the following manuals (look at
// http://www.pcisig.com/ for how to get them):
//
// PCI BIOS Specification
// PCI Local Bus Specification
// PCI to PCI Bridge Specification
// PCI System Design Guide
//
// For HyperTransport information, please consult the following manuals
// from http://www.hypertransport.org :
//
// The HyperTransport I/O Link Specification
//
// Conventional PCI and PCI-X Mode 1 devices have 256 bytes of
// configuration space.  PCI-X Mode 2 and PCIe devices have 4096 bytes of
// configuration space.
//
pub const PCI_CFG_SPACE_SIZE: c_int = 256;
pub const PCI_CFG_SPACE_EXP_SIZE: c_int = 4096;
//
// Under PCI, each device has 256 bytes of configuration address space,
// of which the first 64 bytes are standardized as follows:
//
pub const PCI_STD_HEADER_SIZEOF: c_int = 64;

pub const PCI_VENDOR_ID: c_uint = 0x00	/* 16 bits */;
pub const PCI_DEVICE_ID: c_uint = 0x02	/* 16 bits */;
pub const PCI_COMMAND: c_uint = 0x04	/* 16 bits */;
pub const PCI_COMMAND_IO: c_uint = 0x1	/* Enable response in I/O space */;
pub const PCI_COMMAND_MEMORY: c_uint = 0x2	/* Enable response in Memory space */;
pub const PCI_COMMAND_MASTER: c_uint = 0x4	/* Enable bus mastering */;
pub const PCI_COMMAND_SPECIAL: c_uint = 0x8	/* Enable response to special cycles */;
pub const PCI_COMMAND_INVALIDATE: c_uint = 0x10	/* Use memory write and invalidate */;
pub const PCI_COMMAND_VGA_PALETTE: c_uint = 0x20	/* Enable palette snooping */;
pub const PCI_COMMAND_PARITY: c_uint = 0x40	/* Enable parity checking */;
pub const PCI_COMMAND_WAIT: c_uint = 0x80	/* Enable address/data stepping */;
pub const PCI_COMMAND_SERR: c_uint = 0x100	/* Enable SERR */;
pub const PCI_COMMAND_FAST_BACK: c_uint = 0x200	/* Enable back-to-back writes */;
pub const PCI_COMMAND_INTX_DISABLE: c_uint = 0x400 /* INTx Emulation Disable */;
pub const PCI_STATUS: c_uint = 0x06	/* 16 bits */;
pub const PCI_STATUS_IMM_READY: c_uint = 0x01	/* Immediate Readiness */;
pub const PCI_STATUS_INTERRUPT: c_uint = 0x08	/* Interrupt status */;
pub const PCI_STATUS_CAP_LIST: c_uint = 0x10	/* Support Capability List */;
pub const PCI_STATUS_66MHZ: c_uint = 0x20	/* Support 66 MHz PCI 2.1 bus */;
pub const PCI_STATUS_UDF: c_uint = 0x40	/* Support User Definable Features [obsolete] */;
pub const PCI_STATUS_FAST_BACK: c_uint = 0x80	/* Accept fast-back to back */;
pub const PCI_STATUS_PARITY: c_uint = 0x100	/* Detected parity error */;
pub const PCI_STATUS_DEVSEL_MASK: c_uint = 0x600	/* DEVSEL timing */;
pub const PCI_STATUS_DEVSEL_FAST: c_uint = 0x000;
pub const PCI_STATUS_DEVSEL_MEDIUM: c_uint = 0x200;
pub const PCI_STATUS_DEVSEL_SLOW: c_uint = 0x400;
pub const PCI_STATUS_SIG_TARGET_ABORT: c_uint = 0x800 /* Set on target abort */;
pub const PCI_STATUS_REC_TARGET_ABORT: c_uint = 0x1000 /* Master ack of " */;
pub const PCI_STATUS_REC_MASTER_ABORT: c_uint = 0x2000 /* Set on master abort */;
pub const PCI_STATUS_SIG_SYSTEM_ERROR: c_uint = 0x4000 /* Set when we drive SERR */;
pub const PCI_STATUS_DETECTED_PARITY: c_uint = 0x8000 /* Set on parity error */;
pub const PCI_CLASS_REVISION: c_uint = 0x08	/* High 24 bits are class, low 8 revision */;
pub const PCI_REVISION_ID: c_uint = 0x08	/* Revision ID */;
pub const PCI_CLASS_PROG: c_uint = 0x09	/* Reg. Level Programming Interface */;
pub const PCI_CLASS_DEVICE: c_uint = 0x0a	/* Device class */;
pub const PCI_CACHE_LINE_SIZE: c_uint = 0x0c	/* 8 bits */;
pub const PCI_LATENCY_TIMER: c_uint = 0x0d	/* 8 bits */;
pub const PCI_HEADER_TYPE: c_uint = 0x0e	/* 8 bits */;
pub const PCI_HEADER_TYPE_MASK: c_uint = 0x7f;
pub const PCI_HEADER_TYPE_NORMAL: c_int = 0;
pub const PCI_HEADER_TYPE_BRIDGE: c_int = 1;
pub const PCI_HEADER_TYPE_CARDBUS: c_int = 2;
pub const PCI_HEADER_TYPE_MFD: c_uint = 0x80	/* Multi-Function Device (possible) */;
pub const PCI_BIST: c_uint = 0x0f	/* 8 bits */;
pub const PCI_BIST_CODE_MASK: c_uint = 0x0f	/* Return result */;
pub const PCI_BIST_START: c_uint = 0x40	/* 1 to start BIST, 2 secs or less */;
pub const PCI_BIST_CAPABLE: c_uint = 0x80	/* 1 if BIST capable */;
//
// Base addresses specify locations in memory or I/O space.
// Decoded size can be determined by writing a value of
// 0xffffffff to the register, and reading it back.  Only
// 1 bits are decoded.
//
pub const PCI_BASE_ADDRESS_0: c_uint = 0x10	/* 32 bits */;
pub const PCI_BASE_ADDRESS_1: c_uint = 0x14	/* 32 bits [htype 0,1 only] */;
pub const PCI_BASE_ADDRESS_2: c_uint = 0x18	/* 32 bits [htype 0 only] */;
pub const PCI_BASE_ADDRESS_3: c_uint = 0x1c	/* 32 bits */;
pub const PCI_BASE_ADDRESS_4: c_uint = 0x20	/* 32 bits */;
pub const PCI_BASE_ADDRESS_5: c_uint = 0x24	/* 32 bits */;
pub const PCI_BASE_ADDRESS_SPACE: c_uint = 0x01	/* 0 = memory, 1 = I/O */;
pub const PCI_BASE_ADDRESS_SPACE_IO: c_uint = 0x01;
pub const PCI_BASE_ADDRESS_SPACE_MEMORY: c_uint = 0x00;
pub const PCI_BASE_ADDRESS_MEM_TYPE_MASK: c_uint = 0x06;
pub const PCI_BASE_ADDRESS_MEM_TYPE_32: c_uint = 0x00	/* 32 bit address */;
pub const PCI_BASE_ADDRESS_MEM_TYPE_1M: c_uint = 0x02	/* Below 1M [obsolete] */;
pub const PCI_BASE_ADDRESS_MEM_TYPE_64: c_uint = 0x04	/* 64 bit address */;
pub const PCI_BASE_ADDRESS_MEM_PREFETCH: c_uint = 0x08	/* prefetchable? */;

// bit 1 is reserved if address_space = 1
// Header type 0 (normal devices)
pub const PCI_CARDBUS_CIS: c_uint = 0x28;
pub const PCI_SUBSYSTEM_VENDOR_ID: c_uint = 0x2c;
pub const PCI_SUBSYSTEM_ID: c_uint = 0x2e;
pub const PCI_ROM_ADDRESS: c_uint = 0x30	/* Bits 31..11 are address, 10..1 reserved */;
pub const PCI_ROM_ADDRESS_ENABLE: c_uint = 0x01;

pub const PCI_CAPABILITY_LIST: c_uint = 0x34	/* Offset of first capability list entry */;
// 0x35-0x3b are reserved
pub const PCI_INTERRUPT_LINE: c_uint = 0x3c	/* 8 bits */;
pub const PCI_INTERRUPT_PIN: c_uint = 0x3d	/* 8 bits */;
pub const PCI_MIN_GNT: c_uint = 0x3e	/* 8 bits */;
pub const PCI_MAX_LAT: c_uint = 0x3f	/* 8 bits */;
// Header type 1 (PCI-to-PCI bridges)
pub const PCI_PRIMARY_BUS: c_uint = 0x18	/* Primary bus number */;
pub const PCI_SECONDARY_BUS: c_uint = 0x19	/* Secondary bus number */;
pub const PCI_SUBORDINATE_BUS: c_uint = 0x1a	/* Highest bus number behind the bridge */;
pub const PCI_SEC_LATENCY_TIMER: c_uint = 0x1b	/* Latency timer for secondary interface */;
// Masks for dword-sized processing of Bus Number and Sec Latency Timer fields
pub const PCI_PRIMARY_BUS_MASK: c_uint = 0x000000ff;
pub const PCI_SECONDARY_BUS_MASK: c_uint = 0x0000ff00;
pub const PCI_SUBORDINATE_BUS_MASK: c_uint = 0x00ff0000;
pub const PCI_SEC_LATENCY_TIMER_MASK: c_uint = 0xff000000;
pub const PCI_IO_BASE: c_uint = 0x1c	/* I/O range behind the bridge */;
pub const PCI_IO_LIMIT: c_uint = 0x1d;
pub const PCI_IO_RANGE_TYPE_MASK: c_uint = 0x0fUL	/* I/O bridging type */;
pub const PCI_IO_RANGE_TYPE_16: c_uint = 0x00;
pub const PCI_IO_RANGE_TYPE_32: c_uint = 0x01;

pub const PCI_SEC_STATUS: c_uint = 0x1e	/* Secondary status register, only bit 14 used */;
pub const PCI_MEMORY_BASE: c_uint = 0x20	/* Memory range behind */;
pub const PCI_MEMORY_LIMIT: c_uint = 0x22;
pub const PCI_MEMORY_RANGE_TYPE_MASK: c_uint = 0x0fUL;

pub const PCI_PREF_MEMORY_BASE: c_uint = 0x24	/* Prefetchable memory range behind */;
pub const PCI_PREF_MEMORY_LIMIT: c_uint = 0x26;
pub const PCI_PREF_RANGE_TYPE_MASK: c_uint = 0x0fUL;
pub const PCI_PREF_RANGE_TYPE_32: c_uint = 0x00;
pub const PCI_PREF_RANGE_TYPE_64: c_uint = 0x01;

pub const PCI_PREF_BASE_UPPER32: c_uint = 0x28	/* Upper half of prefetchable memory range */;
pub const PCI_PREF_LIMIT_UPPER32: c_uint = 0x2c;
pub const PCI_IO_BASE_UPPER16: c_uint = 0x30	/* Upper half of I/O addresses */;
pub const PCI_IO_LIMIT_UPPER16: c_uint = 0x32;
// 0x34 same as for htype 0
// 0x35-0x3b is reserved
pub const PCI_ROM_ADDRESS1: c_uint = 0x38	/* Same as PCI_ROM_ADDRESS, but for htype 1 */;
// 0x3c-0x3d are same as for htype 0
pub const PCI_BRIDGE_CONTROL: c_uint = 0x3e;
pub const PCI_BRIDGE_CTL_PARITY: c_uint = 0x01	/* Enable parity detection on secondary interface */;
pub const PCI_BRIDGE_CTL_SERR: c_uint = 0x02	/* The same for SERR forwarding */;
pub const PCI_BRIDGE_CTL_ISA: c_uint = 0x04	/* Enable ISA mode */;
pub const PCI_BRIDGE_CTL_VGA: c_uint = 0x08	/* Forward VGA addresses */;
pub const PCI_BRIDGE_CTL_MASTER_ABORT: c_uint = 0x20  /* Report master aborts */;
pub const PCI_BRIDGE_CTL_BUS_RESET: c_uint = 0x40	/* Secondary bus reset */;
pub const PCI_BRIDGE_CTL_FAST_BACK: c_uint = 0x80	/* Fast Back2Back enabled on secondary interface */;
// Header type 2 (CardBus bridges)
pub const PCI_CB_CAPABILITY_LIST: c_uint = 0x14;
// 0x15 reserved
pub const PCI_CB_SEC_STATUS: c_uint = 0x16	/* Secondary status */;
pub const PCI_CB_PRIMARY_BUS: c_uint = 0x18	/* PCI bus number */;
pub const PCI_CB_CARD_BUS: c_uint = 0x19	/* CardBus bus number */;
pub const PCI_CB_SUBORDINATE_BUS: c_uint = 0x1a	/* Subordinate bus number */;
pub const PCI_CB_LATENCY_TIMER: c_uint = 0x1b	/* CardBus latency timer */;
pub const PCI_CB_MEMORY_BASE_0: c_uint = 0x1c;
pub const PCI_CB_MEMORY_LIMIT_0: c_uint = 0x20;
pub const PCI_CB_MEMORY_BASE_1: c_uint = 0x24;
pub const PCI_CB_MEMORY_LIMIT_1: c_uint = 0x28;
pub const PCI_CB_IO_BASE_0: c_uint = 0x2c;
pub const PCI_CB_IO_BASE_0_HI: c_uint = 0x2e;
pub const PCI_CB_IO_LIMIT_0: c_uint = 0x30;
pub const PCI_CB_IO_LIMIT_0_HI: c_uint = 0x32;
pub const PCI_CB_IO_BASE_1: c_uint = 0x34;
pub const PCI_CB_IO_BASE_1_HI: c_uint = 0x36;
pub const PCI_CB_IO_LIMIT_1: c_uint = 0x38;
pub const PCI_CB_IO_LIMIT_1_HI: c_uint = 0x3a;

// 0x3c-0x3d are same as for htype 0
pub const PCI_CB_BRIDGE_CONTROL: c_uint = 0x3e;
pub const PCI_CB_BRIDGE_CTL_PARITY: c_uint = 0x01	/* Similar to standard bridge control register */;
pub const PCI_CB_BRIDGE_CTL_SERR: c_uint = 0x02;
pub const PCI_CB_BRIDGE_CTL_ISA: c_uint = 0x04;
pub const PCI_CB_BRIDGE_CTL_VGA: c_uint = 0x08;
pub const PCI_CB_BRIDGE_CTL_MASTER_ABORT: c_uint = 0x20;
pub const PCI_CB_BRIDGE_CTL_CB_RESET: c_uint = 0x40	/* CardBus reset */;
pub const PCI_CB_BRIDGE_CTL_16BIT_INT: c_uint = 0x80	/* Enable interrupt for 16-bit cards */;
pub const PCI_CB_BRIDGE_CTL_PREFETCH_MEM0: c_uint = 0x100	/* Prefetch enable for both memory regions */;
pub const PCI_CB_BRIDGE_CTL_PREFETCH_MEM1: c_uint = 0x200;
pub const PCI_CB_BRIDGE_CTL_POST_WRITES: c_uint = 0x400;
pub const PCI_CB_SUBSYSTEM_VENDOR_ID: c_uint = 0x40;
pub const PCI_CB_SUBSYSTEM_ID: c_uint = 0x42;
pub const PCI_CB_LEGACY_MODE_BASE: c_uint = 0x44	/* 16-bit PC Card legacy mode base address (ExCa) */;
// 0x48-0x7f reserved
// Capability lists
pub const PCI_CAP_ID_MASK: c_uint = 0x00ff	/* Capability ID mask */;
pub const PCI_CAP_LIST_NEXT_MASK: c_uint = 0xff00	/* Next Capability Pointer mask */;

pub const PCI_CAP_ID_PM: c_uint = 0x01	/* Power Management */;
pub const PCI_CAP_ID_AGP: c_uint = 0x02	/* Accelerated Graphics Port */;
pub const PCI_CAP_ID_VPD: c_uint = 0x03	/* Vital Product Data */;
pub const PCI_CAP_ID_SLOTID: c_uint = 0x04	/* Slot Identification */;
pub const PCI_CAP_ID_MSI: c_uint = 0x05	/* Message Signalled Interrupts */;
pub const PCI_CAP_ID_CHSWP: c_uint = 0x06	/* CompactPCI HotSwap */;
pub const PCI_CAP_ID_PCIX: c_uint = 0x07	/* PCI-X */;
pub const PCI_CAP_ID_HT: c_uint = 0x08	/* HyperTransport */;
pub const PCI_CAP_ID_VNDR: c_uint = 0x09	/* Vendor-Specific */;
pub const PCI_CAP_ID_DBG: c_uint = 0x0A	/* Debug port */;
pub const PCI_CAP_ID_CCRC: c_uint = 0x0B	/* CompactPCI Central Resource Control */;
pub const PCI_CAP_ID_SHPC: c_uint = 0x0C	/* PCI Standard Hot-Plug Controller */;
pub const PCI_CAP_ID_SSVID: c_uint = 0x0D	/* Bridge subsystem vendor/device ID */;
pub const PCI_CAP_ID_AGP3: c_uint = 0x0E	/* AGP Target PCI-PCI bridge */;
pub const PCI_CAP_ID_SECDEV: c_uint = 0x0F	/* Secure Device */;
pub const PCI_CAP_ID_EXP: c_uint = 0x10	/* PCI Express */;
pub const PCI_CAP_ID_MSIX: c_uint = 0x11	/* MSI-X */;
pub const PCI_CAP_ID_SATA: c_uint = 0x12	/* SATA Data/Index Conf. */;
pub const PCI_CAP_ID_AF: c_uint = 0x13	/* PCI Advanced Features */;
pub const PCI_CAP_ID_EA: c_uint = 0x14	/* PCI Enhanced Allocation */;

pub const PCI_CAP_SIZEOF: c_int = 4;
// Power Management Registers

pub const PCI_PM_CAP_VER_MASK: c_uint = 0x0007	/* Version */;
pub const PCI_PM_CAP_PME_CLOCK: c_uint = 0x0008	/* PME clock required */;
pub const PCI_PM_CAP_RESERVED: c_uint = 0x0010  /* Reserved field */;
pub const PCI_PM_CAP_DSI: c_uint = 0x0020	/* Device specific initialization */;
pub const PCI_PM_CAP_AUX_POWER: c_uint = 0x01C0	/* Auxiliary power support mask */;
pub const PCI_PM_CAP_D1: c_uint = 0x0200	/* D1 power state support */;
pub const PCI_PM_CAP_D2: c_uint = 0x0400	/* D2 power state support */;
pub const PCI_PM_CAP_PME: c_uint = 0x0800	/* PME pin supported */;
pub const PCI_PM_CAP_PME_MASK: c_uint = 0xF800	/* PME Mask of all supported states */;
pub const PCI_PM_CAP_PME_D0: c_uint = 0x0800	/* PME# from D0 */;
pub const PCI_PM_CAP_PME_D1: c_uint = 0x1000	/* PME# from D1 */;
pub const PCI_PM_CAP_PME_D2: c_uint = 0x2000	/* PME# from D2 */;
pub const PCI_PM_CAP_PME_D3hot: c_uint = 0x4000	/* PME# from D3 (hot) */;
pub const PCI_PM_CAP_PME_D3cold: c_uint = 0x8000	/* PME# from D3 (cold) */;

pub const PCI_PM_CTRL_STATE_MASK: c_uint = 0x0003	/* Current power state (D0 to D3) */;
pub const PCI_PM_CTRL_NO_SOFT_RESET: c_uint = 0x0008	/* No reset for D3hot->D0 */;
pub const PCI_PM_CTRL_PME_ENABLE: c_uint = 0x0100	/* PME pin enable */;
pub const PCI_PM_CTRL_DATA_SEL_MASK: c_uint = 0x1e00	/* Data select (??) */;
pub const PCI_PM_CTRL_DATA_SCALE_MASK: c_uint = 0x6000	/* Data scale (??) */;
pub const PCI_PM_CTRL_PME_STATUS: c_uint = 0x8000	/* PME pin status */;

pub const PCI_PM_PPB_B2_B3: c_uint = 0x40	/* Stop clock when in D3hot (??) */;
pub const PCI_PM_BPCC_ENABLE: c_uint = 0x80	/* Bus power/clock control enable (??) */;

pub const PCI_PM_SIZEOF: c_int = 8;
// AGP registers

pub const PCI_AGP_STATUS_RQ_MASK: c_uint = 0xff000000	/* Maximum number of requests - 1 */;
pub const PCI_AGP_STATUS_SBA: c_uint = 0x0200	/* Sideband addressing supported */;
pub const PCI_AGP_STATUS_64BIT: c_uint = 0x0020	/* 64-bit addressing supported */;
pub const PCI_AGP_STATUS_FW: c_uint = 0x0010	/* FW transfers supported */;
pub const PCI_AGP_STATUS_RATE4: c_uint = 0x0004	/* 4x transfer rate supported */;
pub const PCI_AGP_STATUS_RATE2: c_uint = 0x0002	/* 2x transfer rate supported */;
pub const PCI_AGP_STATUS_RATE1: c_uint = 0x0001	/* 1x transfer rate supported */;

pub const PCI_AGP_COMMAND_RQ_MASK: c_uint = 0xff000000  /* Master: Maximum number of requests */;
pub const PCI_AGP_COMMAND_SBA: c_uint = 0x0200	/* Sideband addressing enabled */;
pub const PCI_AGP_COMMAND_AGP: c_uint = 0x0100	/* Allow processing of AGP transactions */;
pub const PCI_AGP_COMMAND_64BIT: c_uint = 0x0020	/* Allow processing of 64-bit addresses */;
pub const PCI_AGP_COMMAND_FW: c_uint = 0x0010	/* Force FW transfers */;
pub const PCI_AGP_COMMAND_RATE4: c_uint = 0x0004	/* Use 4x rate */;
pub const PCI_AGP_COMMAND_RATE2: c_uint = 0x0002	/* Use 2x rate */;
pub const PCI_AGP_COMMAND_RATE1: c_uint = 0x0001	/* Use 1x rate */;
pub const PCI_AGP_SIZEOF: c_int = 12;
// Vital Product Data

pub const PCI_VPD_ADDR_MASK: c_uint = 0x7fff	/* Address mask */;
pub const PCI_VPD_ADDR_F: c_uint = 0x8000	/* Write 0, 1 indicates completion */;

pub const PCI_CAP_VPD_SIZEOF: c_int = 8;
// Slot Identification

pub const PCI_SID_ESR_NSLOTS: c_uint = 0x1f	/* Number of expansion slots available */;
pub const PCI_SID_ESR_FIC: c_uint = 0x20	/* First In Chassis Flag */;

// Message Signaled Interrupt registers
pub const PCI_MSI_FLAGS: c_uint = 0x02	/* Message Control */;
pub const PCI_MSI_FLAGS_ENABLE: c_uint = 0x0001	/* MSI feature enabled */;
pub const PCI_MSI_FLAGS_QMASK: c_uint = 0x000e	/* Maximum queue size available */;
pub const PCI_MSI_FLAGS_QSIZE: c_uint = 0x0070	/* Message queue size configured */;
pub const PCI_MSI_FLAGS_64BIT: c_uint = 0x0080	/* 64-bit addresses allowed */;
pub const PCI_MSI_FLAGS_MASKBIT: c_uint = 0x0100	/* Per-vector masking capable */;

pub const PCI_MSI_ADDRESS_LO: c_uint = 0x04	/* Lower 32 bits */;
pub const PCI_MSI_ADDRESS_HI: c_uint = 0x08	/* Upper 32 bits (if PCI_MSI_FLAGS_64BIT set) */;
pub const PCI_MSI_DATA_32: c_uint = 0x08	/* 16 bits of data for 32-bit devices */;
pub const PCI_MSI_MASK_32: c_uint = 0x0c	/* Mask bits register for 32-bit devices */;
pub const PCI_MSI_PENDING_32: c_uint = 0x10	/* Pending intrs for 32-bit devices */;
pub const PCI_MSI_DATA_64: c_uint = 0x0c	/* 16 bits of data for 64-bit devices */;
pub const PCI_MSI_MASK_64: c_uint = 0x10	/* Mask bits register for 64-bit devices */;
pub const PCI_MSI_PENDING_64: c_uint = 0x14	/* Pending intrs for 64-bit devices */;
// MSI-X registers (in MSI-X capability)

pub const PCI_MSIX_FLAGS_QSIZE: c_uint = 0x07FF	/* Table size */;
pub const PCI_MSIX_FLAGS_MASKALL: c_uint = 0x4000	/* Mask all vectors for this function */;
pub const PCI_MSIX_FLAGS_ENABLE: c_uint = 0x8000	/* MSI-X enable */;

pub const PCI_MSIX_TABLE_BIR: c_uint = 0x00000007 /* BAR index */;
pub const PCI_MSIX_TABLE_OFFSET: c_uint = 0xfffffff8 /* Offset into specified BAR */;

pub const PCI_MSIX_PBA_BIR: c_uint = 0x00000007 /* BAR index */;
pub const PCI_MSIX_PBA_OFFSET: c_uint = 0xfffffff8 /* Offset into specified BAR */;

// MSI-X Table entry format (in memory mapped by a BAR)
pub const PCI_MSIX_ENTRY_SIZE: c_int = 16;
pub const PCI_MSIX_ENTRY_LOWER_ADDR: c_uint = 0x0  /* Message Address */;
pub const PCI_MSIX_ENTRY_UPPER_ADDR: c_uint = 0x4  /* Message Upper Address */;
pub const PCI_MSIX_ENTRY_DATA: c_uint = 0x8  /* Message Data */;
pub const PCI_MSIX_ENTRY_VECTOR_CTRL: c_uint = 0xc  /* Vector Control */;
pub const PCI_MSIX_ENTRY_CTRL_MASKBIT: c_uint = 0x00000001  /* Mask Bit */;
pub const PCI_MSIX_ENTRY_CTRL_ST: c_uint = 0xffff0000  /* Steering Tag */;
// CompactPCI Hotswap Register

pub const PCI_CHSWP_DHA: c_uint = 0x01	/* Device Hiding Arm */;
pub const PCI_CHSWP_EIM: c_uint = 0x02	/* ENUM# Signal Mask */;
pub const PCI_CHSWP_PIE: c_uint = 0x04	/* Pending Insert or Extract */;
pub const PCI_CHSWP_LOO: c_uint = 0x08	/* LED On / Off */;
pub const PCI_CHSWP_PI: c_uint = 0x30	/* Programming Interface */;
pub const PCI_CHSWP_EXT: c_uint = 0x40	/* ENUM# status - extraction */;
pub const PCI_CHSWP_INS: c_uint = 0x80	/* ENUM# status - insertion */;
// PCI Advanced Feature registers
pub const PCI_AF_LENGTH: c_int = 2;
pub const PCI_AF_CAP: c_int = 3;
pub const PCI_AF_CAP_TP: c_uint = 0x01;
pub const PCI_AF_CAP_FLR: c_uint = 0x02;
pub const PCI_AF_CTRL: c_int = 4;
pub const PCI_AF_CTRL_FLR: c_uint = 0x01;
pub const PCI_AF_STATUS: c_int = 5;
pub const PCI_AF_STATUS_TP: c_uint = 0x01;

// PCI Enhanced Allocation registers

pub const PCI_EA_NUM_ENT_MASK: c_uint = 0x3f	/* Num Entries Mask */;

pub const PCI_EA_ES: c_uint = 0x00000007 /* Entry Size */;
pub const PCI_EA_BEI: c_uint = 0x000000f0 /* BAR Equivalent Indicator */;
// EA fixed Secondary and Subordinate bus numbers for Bridge
pub const PCI_EA_SEC_BUS_MASK: c_uint = 0xff;
pub const PCI_EA_SUB_BUS_MASK: c_uint = 0xff00;
pub const PCI_EA_SUB_BUS_SHIFT: c_int = 8;
// 0-5 map to BARs 0-5 respectively
pub const PCI_EA_BEI_BAR0: c_int = 0;
pub const PCI_EA_BEI_BAR5: c_int = 5;

// 9-14 map to VF BARs 0-5 respectively
pub const PCI_EA_BEI_VF_BAR0: c_int = 9;
pub const PCI_EA_BEI_VF_BAR5: c_int = 14;

pub const PCI_EA_PP: c_uint = 0x0000ff00	/* Primary Properties */;
pub const PCI_EA_SP: c_uint = 0x00ff0000	/* Secondary Properties */;
pub const PCI_EA_P_MEM: c_uint = 0x00	/* Non-Prefetch Memory */;
pub const PCI_EA_P_MEM_PREFETCH: c_uint = 0x01	/* Prefetchable Memory */;
pub const PCI_EA_P_IO: c_uint = 0x02	/* I/O Space */;
pub const PCI_EA_P_VF_MEM_PREFETCH: c_uint = 0x03	/* VF Prefetchable Memory */;
pub const PCI_EA_P_VF_MEM: c_uint = 0x04	/* VF Non-Prefetch Memory */;
pub const PCI_EA_P_BRIDGE_MEM: c_uint = 0x05	/* Bridge Non-Prefetch Memory */;
pub const PCI_EA_P_BRIDGE_MEM_PREFETCH: c_uint = 0x06	/* Bridge Prefetchable Memory */;
pub const PCI_EA_P_BRIDGE_IO: c_uint = 0x07	/* Bridge I/O Space */;
// 0x08-0xfc reserved
pub const PCI_EA_P_MEM_RESERVED: c_uint = 0xfd	/* Reserved Memory */;
pub const PCI_EA_P_IO_RESERVED: c_uint = 0xfe	/* Reserved I/O Space */;
pub const PCI_EA_P_UNAVAILABLE: c_uint = 0xff	/* Entry Unavailable */;
pub const PCI_EA_WRITABLE: c_uint = 0x40000000	/* Writable: 1 = RW, 0 = HwInit */;
pub const PCI_EA_ENABLE: c_uint = 0x80000000	/* Enable for this entry */;

// bit 0 is reserved
pub const PCI_EA_IS_64: c_uint = 0x00000002	/* 64-bit field flag */;
pub const PCI_EA_FIELD_MASK: c_uint = 0xfffffffc	/* For Base & Max Offset */;
// PCI-X registers (Type 0 (non-bridge) devices)

pub const PCI_X_CMD_DPERR_E: c_uint = 0x0001	/* Data Parity Error Recovery Enable */;
pub const PCI_X_CMD_ERO: c_uint = 0x0002	/* Enable Relaxed Ordering */;
pub const PCI_X_CMD_READ_512: c_uint = 0x0000	/* 512 byte maximum read byte count */;
pub const PCI_X_CMD_READ_1K: c_uint = 0x0004	/* 1Kbyte maximum read byte count */;
pub const PCI_X_CMD_READ_2K: c_uint = 0x0008	/* 2Kbyte maximum read byte count */;
pub const PCI_X_CMD_READ_4K: c_uint = 0x000c	/* 4Kbyte maximum read byte count */;
pub const PCI_X_CMD_MAX_READ: c_uint = 0x000c	/* Max Memory Read Byte Count */;
// Max # of outstanding split transactions
pub const PCI_X_CMD_SPLIT_1: c_uint = 0x0000	/* Max 1 */;
pub const PCI_X_CMD_SPLIT_2: c_uint = 0x0010	/* Max 2 */;
pub const PCI_X_CMD_SPLIT_3: c_uint = 0x0020	/* Max 3 */;
pub const PCI_X_CMD_SPLIT_4: c_uint = 0x0030	/* Max 4 */;
pub const PCI_X_CMD_SPLIT_8: c_uint = 0x0040	/* Max 8 */;
pub const PCI_X_CMD_SPLIT_12: c_uint = 0x0050	/* Max 12 */;
pub const PCI_X_CMD_SPLIT_16: c_uint = 0x0060	/* Max 16 */;
pub const PCI_X_CMD_SPLIT_32: c_uint = 0x0070	/* Max 32 */;
pub const PCI_X_CMD_MAX_SPLIT: c_uint = 0x0070	/* Max Outstanding Split Transactions */;

pub const PCI_X_STATUS_DEVFN: c_uint = 0x000000ff	/* A copy of devfn */;
pub const PCI_X_STATUS_BUS: c_uint = 0x0000ff00	/* A copy of bus nr */;
pub const PCI_X_STATUS_64BIT: c_uint = 0x00010000	/* 64-bit device */;
pub const PCI_X_STATUS_133MHZ: c_uint = 0x00020000	/* 133 MHz capable */;
pub const PCI_X_STATUS_SPL_DISC: c_uint = 0x00040000	/* Split Completion Discarded */;
pub const PCI_X_STATUS_UNX_SPL: c_uint = 0x00080000	/* Unexpected Split Completion */;
pub const PCI_X_STATUS_COMPLEX: c_uint = 0x00100000	/* Device Complexity */;
pub const PCI_X_STATUS_MAX_READ: c_uint = 0x00600000	/* Designed Max Memory Read Count */;
pub const PCI_X_STATUS_MAX_SPLIT: c_uint = 0x03800000	/* Designed Max Outstanding Split Transactions */;
pub const PCI_X_STATUS_MAX_CUM: c_uint = 0x1c000000	/* Designed Max Cumulative Read Size */;
pub const PCI_X_STATUS_SPL_ERR: c_uint = 0x20000000	/* Rcvd Split Completion Error Msg */;
pub const PCI_X_STATUS_266MHZ: c_uint = 0x40000000	/* 266 MHz capable */;
pub const PCI_X_STATUS_533MHZ: c_uint = 0x80000000	/* 533 MHz capable */;

// PCI-X registers (Type 1 (bridge) devices)

pub const PCI_X_SSTATUS_64BIT: c_uint = 0x0001	/* Secondary AD interface is 64 bits */;
pub const PCI_X_SSTATUS_133MHZ: c_uint = 0x0002	/* 133 MHz capable */;
pub const PCI_X_SSTATUS_FREQ: c_uint = 0x03c0	/* Secondary Bus Mode and Frequency */;
pub const PCI_X_SSTATUS_VERS: c_uint = 0x3000	/* PCI-X Capability Version */;
pub const PCI_X_SSTATUS_V1: c_uint = 0x1000	/* Mode 2, not Mode 1 */;
pub const PCI_X_SSTATUS_V2: c_uint = 0x2000	/* Mode 1 or Modes 1 and 2 */;
pub const PCI_X_SSTATUS_266MHZ: c_uint = 0x4000	/* 266 MHz capable */;
pub const PCI_X_SSTATUS_533MHZ: c_uint = 0x8000	/* 533 MHz capable */;

// PCI Bridge Subsystem ID registers

// PCI Express capability registers
pub const PCI_EXP_FLAGS: c_uint = 0x02	/* Capabilities register */;
pub const PCI_EXP_FLAGS_VERS: c_uint = 0x000f	/* Capability version */;
pub const PCI_EXP_FLAGS_TYPE: c_uint = 0x00f0	/* Device/Port type */;
pub const PCI_EXP_TYPE_ENDPOINT: c_uint = 0x0	/* Express Endpoint */;
pub const PCI_EXP_TYPE_LEG_END: c_uint = 0x1	/* Legacy Endpoint */;
pub const PCI_EXP_TYPE_ROOT_PORT: c_uint = 0x4	/* Root Port */;
pub const PCI_EXP_TYPE_UPSTREAM: c_uint = 0x5	/* Upstream Port */;
pub const PCI_EXP_TYPE_DOWNSTREAM: c_uint = 0x6	/* Downstream Port */;
pub const PCI_EXP_TYPE_PCI_BRIDGE: c_uint = 0x7	/* PCIe to PCI/PCI-X Bridge */;
pub const PCI_EXP_TYPE_PCIE_BRIDGE: c_uint = 0x8	/* PCI/PCI-X to PCIe Bridge */;
pub const PCI_EXP_TYPE_RC_END: c_uint = 0x9	/* Root Complex Integrated Endpoint */;
pub const PCI_EXP_TYPE_RC_EC: c_uint = 0xa	/* Root Complex Event Collector */;
pub const PCI_EXP_FLAGS_SLOT: c_uint = 0x0100	/* Slot implemented */;
pub const PCI_EXP_FLAGS_IRQ: c_uint = 0x3e00	/* Interrupt message number */;
pub const PCI_EXP_FLAGS_FLIT: c_uint = 0x8000	/* Flit Mode Supported */;
pub const PCI_EXP_DEVCAP: c_uint = 0x04	/* Device capabilities */;
pub const PCI_EXP_DEVCAP_PAYLOAD: c_uint = 0x00000007 /* Max_Payload_Size */;
pub const PCI_EXP_DEVCAP_PHANTOM: c_uint = 0x00000018 /* Phantom functions */;
pub const PCI_EXP_DEVCAP_EXT_TAG: c_uint = 0x00000020 /* Extended tags */;
pub const PCI_EXP_DEVCAP_L0S: c_uint = 0x000001c0 /* L0s Acceptable Latency */;
pub const PCI_EXP_DEVCAP_L1: c_uint = 0x00000e00 /* L1 Acceptable Latency */;
pub const PCI_EXP_DEVCAP_ATN_BUT: c_uint = 0x00001000 /* Attention Button Present */;
pub const PCI_EXP_DEVCAP_ATN_IND: c_uint = 0x00002000 /* Attention Indicator Present */;
pub const PCI_EXP_DEVCAP_PWR_IND: c_uint = 0x00004000 /* Power Indicator Present */;
pub const PCI_EXP_DEVCAP_RBER: c_uint = 0x00008000 /* Role-Based Error Reporting */;
pub const PCI_EXP_DEVCAP_PWR_VAL: c_uint = 0x03fc0000 /* Slot Power Limit Value */;
pub const PCI_EXP_DEVCAP_PWR_SCL: c_uint = 0x0c000000 /* Slot Power Limit Scale */;
pub const PCI_EXP_DEVCAP_FLR: c_uint = 0x10000000 /* Function Level Reset */;
pub const PCI_EXP_DEVCAP_TEE: c_uint = 0x40000000 /* TEE I/O (TDISP) Support */;
pub const PCI_EXP_DEVCTL: c_uint = 0x08	/* Device Control */;
pub const PCI_EXP_DEVCTL_CERE: c_uint = 0x0001	/* Correctable Error Reporting En. */;
pub const PCI_EXP_DEVCTL_NFERE: c_uint = 0x0002	/* Non-Fatal Error Reporting Enable */;
pub const PCI_EXP_DEVCTL_FERE: c_uint = 0x0004	/* Fatal Error Reporting Enable */;
pub const PCI_EXP_DEVCTL_URRE: c_uint = 0x0008	/* Unsupported Request Reporting En. */;
pub const PCI_EXP_DEVCTL_RELAX_EN: c_uint = 0x0010 /* Enable relaxed ordering */;
pub const PCI_EXP_DEVCTL_PAYLOAD: c_uint = 0x00e0	/* Max_Payload_Size */;
pub const PCI_EXP_DEVCTL_PAYLOAD_128B: c_uint = 0x0000 /* 128 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_256B: c_uint = 0x0020 /* 256 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_512B: c_uint = 0x0040 /* 512 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_1024B: c_uint = 0x0060 /* 1024 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_2048B: c_uint = 0x0080 /* 2048 Bytes */;
pub const PCI_EXP_DEVCTL_PAYLOAD_4096B: c_uint = 0x00a0 /* 4096 Bytes */;
pub const PCI_EXP_DEVCTL_EXT_TAG: c_uint = 0x0100	/* Extended Tag Field Enable */;
pub const PCI_EXP_DEVCTL_PHANTOM: c_uint = 0x0200	/* Phantom Functions Enable */;
pub const PCI_EXP_DEVCTL_AUX_PME: c_uint = 0x0400	/* Auxiliary Power PM Enable */;
pub const PCI_EXP_DEVCTL_NOSNOOP_EN: c_uint = 0x0800  /* Enable No Snoop */;
pub const PCI_EXP_DEVCTL_READRQ: c_uint = 0x7000	/* Max_Read_Request_Size */;
pub const PCI_EXP_DEVCTL_READRQ_128B: c_uint = 0x0000 /* 128 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_256B: c_uint = 0x1000 /* 256 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_512B: c_uint = 0x2000 /* 512 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_1024B: c_uint = 0x3000 /* 1024 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_2048B: c_uint = 0x4000 /* 2048 Bytes */;
pub const PCI_EXP_DEVCTL_READRQ_4096B: c_uint = 0x5000 /* 4096 Bytes */;
pub const PCI_EXP_DEVCTL_BCR_FLR: c_uint = 0x8000  /* Bridge Configuration Retry / FLR */;
pub const PCI_EXP_DEVSTA: c_uint = 0x0a	/* Device Status */;
pub const PCI_EXP_DEVSTA_CED: c_uint = 0x0001	/* Correctable Error Detected */;
pub const PCI_EXP_DEVSTA_NFED: c_uint = 0x0002	/* Non-Fatal Error Detected */;
pub const PCI_EXP_DEVSTA_FED: c_uint = 0x0004	/* Fatal Error Detected */;
pub const PCI_EXP_DEVSTA_URD: c_uint = 0x0008	/* Unsupported Request Detected */;
pub const PCI_EXP_DEVSTA_AUXPD: c_uint = 0x0010	/* AUX Power Detected */;
pub const PCI_EXP_DEVSTA_TRPND: c_uint = 0x0020	/* Transactions Pending */;

pub const PCI_EXP_LNKCAP: c_uint = 0x0c	/* Link Capabilities */;
pub const PCI_EXP_LNKCAP_SLS: c_uint = 0x0000000f /* Max Link Speed (prior to PCIe r3.0: Supported Link Speeds) */;
pub const PCI_EXP_LNKCAP_SLS_2_5GB: c_uint = 0x00000001 /* LNKCAP2 SLS Vector bit 0 */;
pub const PCI_EXP_LNKCAP_SLS_5_0GB: c_uint = 0x00000002 /* LNKCAP2 SLS Vector bit 1 */;
pub const PCI_EXP_LNKCAP_SLS_8_0GB: c_uint = 0x00000003 /* LNKCAP2 SLS Vector bit 2 */;
pub const PCI_EXP_LNKCAP_SLS_16_0GB: c_uint = 0x00000004 /* LNKCAP2 SLS Vector bit 3 */;
pub const PCI_EXP_LNKCAP_SLS_32_0GB: c_uint = 0x00000005 /* LNKCAP2 SLS Vector bit 4 */;
pub const PCI_EXP_LNKCAP_SLS_64_0GB: c_uint = 0x00000006 /* LNKCAP2 SLS Vector bit 5 */;
pub const PCI_EXP_LNKCAP_MLW: c_uint = 0x000003f0 /* Maximum Link Width */;
pub const PCI_EXP_LNKCAP_ASPMS: c_uint = 0x00000c00 /* ASPM Support */;
pub const PCI_EXP_LNKCAP_ASPM_L0S: c_uint = 0x00000400 /* ASPM L0s Support */;
pub const PCI_EXP_LNKCAP_ASPM_L1: c_uint = 0x00000800 /* ASPM L1 Support */;
pub const PCI_EXP_LNKCAP_L0SEL: c_uint = 0x00007000 /* L0s Exit Latency */;
pub const PCI_EXP_LNKCAP_L1EL: c_uint = 0x00038000 /* L1 Exit Latency */;
pub const PCI_EXP_LNKCAP_CLKPM: c_uint = 0x00040000 /* Clock Power Management */;
pub const PCI_EXP_LNKCAP_SDERC: c_uint = 0x00080000 /* Surprise Down Error Reporting Capable */;
pub const PCI_EXP_LNKCAP_DLLLARC: c_uint = 0x00100000 /* Data Link Layer Link Active Reporting Capable */;
pub const PCI_EXP_LNKCAP_LBNC: c_uint = 0x00200000 /* Link Bandwidth Notification Capability */;
pub const PCI_EXP_LNKCAP_PN: c_uint = 0xff000000 /* Port Number */;
pub const PCI_EXP_LNKCTL: c_uint = 0x10	/* Link Control */;
pub const PCI_EXP_LNKCTL_ASPMC: c_uint = 0x0003	/* ASPM Control */;
pub const PCI_EXP_LNKCTL_ASPM_L0S: c_uint = 0x0001	/* L0s Enable */;
pub const PCI_EXP_LNKCTL_ASPM_L1: c_uint = 0x0002	/* L1 Enable */;
pub const PCI_EXP_LNKCTL_RCB: c_uint = 0x0008	/* Read Completion Boundary */;
pub const PCI_EXP_LNKCTL_LD: c_uint = 0x0010	/* Link Disable */;
pub const PCI_EXP_LNKCTL_RL: c_uint = 0x0020	/* Retrain Link */;
pub const PCI_EXP_LNKCTL_CCC: c_uint = 0x0040	/* Common Clock Configuration */;
pub const PCI_EXP_LNKCTL_ES: c_uint = 0x0080	/* Extended Synch */;
pub const PCI_EXP_LNKCTL_CLKREQ_EN: c_uint = 0x0100 /* Enable clkreq */;
pub const PCI_EXP_LNKCTL_HAWD: c_uint = 0x0200	/* Hardware Autonomous Width Disable */;
pub const PCI_EXP_LNKCTL_LBMIE: c_uint = 0x0400	/* Link Bandwidth Management Interrupt Enable */;
pub const PCI_EXP_LNKCTL_LABIE: c_uint = 0x0800	/* Link Autonomous Bandwidth Interrupt Enable */;
pub const PCI_EXP_LNKSTA: c_uint = 0x12	/* Link Status */;
pub const PCI_EXP_LNKSTA_CLS: c_uint = 0x000f	/* Current Link Speed */;
pub const PCI_EXP_LNKSTA_CLS_2_5GB: c_uint = 0x0001 /* Current Link Speed 2.5GT/s */;
pub const PCI_EXP_LNKSTA_CLS_5_0GB: c_uint = 0x0002 /* Current Link Speed 5.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_8_0GB: c_uint = 0x0003 /* Current Link Speed 8.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_16_0GB: c_uint = 0x0004 /* Current Link Speed 16.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_32_0GB: c_uint = 0x0005 /* Current Link Speed 32.0GT/s */;
pub const PCI_EXP_LNKSTA_CLS_64_0GB: c_uint = 0x0006 /* Current Link Speed 64.0GT/s */;
pub const PCI_EXP_LNKSTA_NLW: c_uint = 0x03f0	/* Negotiated Link Width */;
pub const PCI_EXP_LNKSTA_NLW_X1: c_uint = 0x0010	/* Current Link Width x1 */;
pub const PCI_EXP_LNKSTA_NLW_X2: c_uint = 0x0020	/* Current Link Width x2 */;
pub const PCI_EXP_LNKSTA_NLW_X4: c_uint = 0x0040	/* Current Link Width x4 */;
pub const PCI_EXP_LNKSTA_NLW_X8: c_uint = 0x0080	/* Current Link Width x8 */;

pub const PCI_EXP_LNKSTA_LT: c_uint = 0x0800	/* Link Training */;
pub const PCI_EXP_LNKSTA_SLC: c_uint = 0x1000	/* Slot Clock Configuration */;
pub const PCI_EXP_LNKSTA_DLLLA: c_uint = 0x2000	/* Data Link Layer Link Active */;
pub const PCI_EXP_LNKSTA_LBMS: c_uint = 0x4000	/* Link Bandwidth Management Status */;
pub const PCI_EXP_LNKSTA_LABS: c_uint = 0x8000	/* Link Autonomous Bandwidth Status */;

pub const PCI_EXP_SLTCAP: c_uint = 0x14	/* Slot Capabilities */;
pub const PCI_EXP_SLTCAP_ABP: c_uint = 0x00000001 /* Attention Button Present */;
pub const PCI_EXP_SLTCAP_PCP: c_uint = 0x00000002 /* Power Controller Present */;
pub const PCI_EXP_SLTCAP_MRLSP: c_uint = 0x00000004 /* MRL Sensor Present */;
pub const PCI_EXP_SLTCAP_AIP: c_uint = 0x00000008 /* Attention Indicator Present */;
pub const PCI_EXP_SLTCAP_PIP: c_uint = 0x00000010 /* Power Indicator Present */;
pub const PCI_EXP_SLTCAP_HPS: c_uint = 0x00000020 /* Hot-Plug Surprise */;
pub const PCI_EXP_SLTCAP_HPC: c_uint = 0x00000040 /* Hot-Plug Capable */;
pub const PCI_EXP_SLTCAP_SPLV: c_uint = 0x00007f80 /* Slot Power Limit Value */;
pub const PCI_EXP_SLTCAP_SPLS: c_uint = 0x00018000 /* Slot Power Limit Scale */;
pub const PCI_EXP_SLTCAP_EIP: c_uint = 0x00020000 /* Electromechanical Interlock Present */;
pub const PCI_EXP_SLTCAP_NCCS: c_uint = 0x00040000 /* No Command Completed Support */;
pub const PCI_EXP_SLTCAP_PSN: c_uint = 0xfff80000 /* Physical Slot Number */;
pub const PCI_EXP_SLTCTL: c_uint = 0x18	/* Slot Control */;
pub const PCI_EXP_SLTCTL_ABPE: c_uint = 0x0001	/* Attention Button Pressed Enable */;
pub const PCI_EXP_SLTCTL_PFDE: c_uint = 0x0002	/* Power Fault Detected Enable */;
pub const PCI_EXP_SLTCTL_MRLSCE: c_uint = 0x0004	/* MRL Sensor Changed Enable */;
pub const PCI_EXP_SLTCTL_PDCE: c_uint = 0x0008	/* Presence Detect Changed Enable */;
pub const PCI_EXP_SLTCTL_CCIE: c_uint = 0x0010	/* Command Completed Interrupt Enable */;
pub const PCI_EXP_SLTCTL_HPIE: c_uint = 0x0020	/* Hot-Plug Interrupt Enable */;
pub const PCI_EXP_SLTCTL_AIC: c_uint = 0x00c0	/* Attention Indicator Control */;

pub const PCI_EXP_SLTCTL_ATTN_IND_ON: c_uint = 0x0040 /* Attention Indicator on */;
pub const PCI_EXP_SLTCTL_ATTN_IND_BLINK: c_uint = 0x0080 /* Attention Indicator blinking */;
pub const PCI_EXP_SLTCTL_ATTN_IND_OFF: c_uint = 0x00c0 /* Attention Indicator off */;
pub const PCI_EXP_SLTCTL_PIC: c_uint = 0x0300	/* Power Indicator Control */;
pub const PCI_EXP_SLTCTL_PWR_IND_ON: c_uint = 0x0100 /* Power Indicator on */;
pub const PCI_EXP_SLTCTL_PWR_IND_BLINK: c_uint = 0x0200 /* Power Indicator blinking */;
pub const PCI_EXP_SLTCTL_PWR_IND_OFF: c_uint = 0x0300 /* Power Indicator off */;
pub const PCI_EXP_SLTCTL_PCC: c_uint = 0x0400	/* Power Controller Control */;
pub const PCI_EXP_SLTCTL_PWR_ON: c_uint = 0x0000 /* Power On */;
pub const PCI_EXP_SLTCTL_PWR_OFF: c_uint = 0x0400 /* Power Off */;
pub const PCI_EXP_SLTCTL_EIC: c_uint = 0x0800	/* Electromechanical Interlock Control */;
pub const PCI_EXP_SLTCTL_DLLSCE: c_uint = 0x1000	/* Data Link Layer State Changed Enable */;
pub const PCI_EXP_SLTCTL_ASPL_DISABLE: c_uint = 0x2000 /* Auto Slot Power Limit Disable */;
pub const PCI_EXP_SLTCTL_IBPD_DISABLE: c_uint = 0x4000 /* In-band PD disable */;
pub const PCI_EXP_SLTSTA: c_uint = 0x1a	/* Slot Status */;
pub const PCI_EXP_SLTSTA_ABP: c_uint = 0x0001	/* Attention Button Pressed */;
pub const PCI_EXP_SLTSTA_PFD: c_uint = 0x0002	/* Power Fault Detected */;
pub const PCI_EXP_SLTSTA_MRLSC: c_uint = 0x0004	/* MRL Sensor Changed */;
pub const PCI_EXP_SLTSTA_PDC: c_uint = 0x0008	/* Presence Detect Changed */;
pub const PCI_EXP_SLTSTA_CC: c_uint = 0x0010	/* Command Completed */;
pub const PCI_EXP_SLTSTA_MRLSS: c_uint = 0x0020	/* MRL Sensor State */;
pub const PCI_EXP_SLTSTA_PDS: c_uint = 0x0040	/* Presence Detect State */;
pub const PCI_EXP_SLTSTA_EIS: c_uint = 0x0080	/* Electromechanical Interlock Status */;
pub const PCI_EXP_SLTSTA_DLLSC: c_uint = 0x0100	/* Data Link Layer State Changed */;
pub const PCI_EXP_RTCTL: c_uint = 0x1c	/* Root Control */;
pub const PCI_EXP_RTCTL_SECEE: c_uint = 0x0001	/* System Error on Correctable Error */;
pub const PCI_EXP_RTCTL_SENFEE: c_uint = 0x0002	/* System Error on Non-Fatal Error */;
pub const PCI_EXP_RTCTL_SEFEE: c_uint = 0x0004	/* System Error on Fatal Error */;
pub const PCI_EXP_RTCTL_PMEIE: c_uint = 0x0008	/* PME Interrupt Enable */;
pub const PCI_EXP_RTCTL_RRS_SVE: c_uint = 0x0010	/* Config RRS Software Visibility Enable */;

pub const PCI_EXP_RTCAP: c_uint = 0x1e	/* Root Capabilities */;
pub const PCI_EXP_RTCAP_RRS_SV: c_uint = 0x0001	/* Config RRS Software Visibility */;

pub const PCI_EXP_RTSTA: c_uint = 0x20	/* Root Status */;
pub const PCI_EXP_RTSTA_PME_RQ_ID: c_uint = 0x0000ffff /* PME Requester ID */;
pub const PCI_EXP_RTSTA_PME: c_uint = 0x00010000 /* PME status */;
pub const PCI_EXP_RTSTA_PENDING: c_uint = 0x00020000 /* PME pending */;
//
// The Device Capabilities 2, Device Status 2, Device Control 2,
// Link Capabilities 2, Link Status 2, Link Control 2,
// Slot Capabilities 2, Slot Status 2, and Slot Control 2 registers
// are only present on devices with PCIe Capability version 2.
// Use pcie_capability_read_word() and similar interfaces to use them
// safely.
//
pub const PCI_EXP_DEVCAP2: c_uint = 0x24	/* Device Capabilities 2 */;
pub const PCI_EXP_DEVCAP2_COMP_TMOUT_DIS: c_uint = 0x00000010 /* Completion Timeout Disable supported */;
pub const PCI_EXP_DEVCAP2_ARI: c_uint = 0x00000020 /* Alternative Routing-ID */;
pub const PCI_EXP_DEVCAP2_ATOMIC_ROUTE: c_uint = 0x00000040 /* Atomic Op routing */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP32: c_uint = 0x00000080 /* 32b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP64: c_uint = 0x00000100 /* 64b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_ATOMIC_COMP128: c_uint = 0x00000200 /* 128b AtomicOp completion */;
pub const PCI_EXP_DEVCAP2_LTR: c_uint = 0x00000800 /* Latency tolerance reporting */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_MASK: c_uint = 0x00003000 /* TPH completer support */;
pub const PCI_EXP_DEVCAP2_OBFF_MASK: c_uint = 0x000c0000 /* OBFF support mechanism */;
pub const PCI_EXP_DEVCAP2_OBFF_MSG: c_uint = 0x00040000 /* New message signaling */;
pub const PCI_EXP_DEVCAP2_OBFF_WAKE: c_uint = 0x00080000 /* Re-use WAKE# for OBFF */;
pub const PCI_EXP_DEVCAP2_EE_PREFIX: c_uint = 0x00200000 /* End-End TLP Prefix */;
pub const PCI_EXP_DEVCAP2_EE_PREFIX_MAX: c_uint = 0x00c00000 /* Max End-End TLP Prefixes */;
pub const PCI_EXP_DEVCTL2: c_uint = 0x28	/* Device Control 2 */;
pub const PCI_EXP_DEVCTL2_COMP_TIMEOUT: c_uint = 0x000f	/* Completion Timeout Value */;
pub const PCI_EXP_DEVCTL2_COMP_TMOUT_DIS: c_uint = 0x0010	/* Completion Timeout Disable */;
pub const PCI_EXP_DEVCTL2_ARI: c_uint = 0x0020	/* Alternative Routing-ID */;
pub const PCI_EXP_DEVCTL2_ATOMIC_REQ: c_uint = 0x0040	/* Set Atomic requests */;
pub const PCI_EXP_DEVCTL2_ATOMIC_EGRESS_BLOCK: c_uint = 0x0080 /* Block atomic egress */;
pub const PCI_EXP_DEVCTL2_IDO_REQ_EN: c_uint = 0x0100	/* Allow IDO for requests */;
pub const PCI_EXP_DEVCTL2_IDO_CMP_EN: c_uint = 0x0200	/* Allow IDO for completions */;
pub const PCI_EXP_DEVCTL2_LTR_EN: c_uint = 0x0400	/* Enable LTR mechanism */;
pub const PCI_EXP_DEVCTL2_OBFF_MSGA_EN: c_uint = 0x2000	/* Enable OBFF Message type A */;
pub const PCI_EXP_DEVCTL2_OBFF_MSGB_EN: c_uint = 0x4000	/* Enable OBFF Message type B */;
pub const PCI_EXP_DEVCTL2_OBFF_WAKE_EN: c_uint = 0x6000	/* OBFF using WAKE# signaling */;
pub const PCI_EXP_DEVSTA2: c_uint = 0x2a	/* Device Status 2 */;
pub const PCI_CAP_EXP_RC_ENDPOINT_SIZEOF_V2: c_uint = 0x2c	/* end of v2 EPs w/o link */;
pub const PCI_EXP_LNKCAP2: c_uint = 0x2c	/* Link Capabilities 2 */;
pub const PCI_EXP_LNKCAP2_SLS: c_uint = 0x000000fe /* Supported Link Speeds Vector */;
pub const PCI_EXP_LNKCAP2_SLS_2_5GB: c_uint = 0x00000002 /* Supported Speed 2.5GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_5_0GB: c_uint = 0x00000004 /* Supported Speed 5GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_8_0GB: c_uint = 0x00000008 /* Supported Speed 8GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_16_0GB: c_uint = 0x00000010 /* Supported Speed 16GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_32_0GB: c_uint = 0x00000020 /* Supported Speed 32GT/s */;
pub const PCI_EXP_LNKCAP2_SLS_64_0GB: c_uint = 0x00000040 /* Supported Speed 64GT/s */;
pub const PCI_EXP_LNKCAP2_CROSSLINK: c_uint = 0x00000100 /* Crosslink supported */;
pub const PCI_EXP_LNKCTL2: c_uint = 0x30	/* Link Control 2 */;
pub const PCI_EXP_LNKCTL2_TLS: c_uint = 0x000f;
pub const PCI_EXP_LNKCTL2_TLS_2_5GT: c_uint = 0x0001 /* Supported Speed 2.5GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_5_0GT: c_uint = 0x0002 /* Supported Speed 5GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_8_0GT: c_uint = 0x0003 /* Supported Speed 8GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_16_0GT: c_uint = 0x0004 /* Supported Speed 16GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_32_0GT: c_uint = 0x0005 /* Supported Speed 32GT/s */;
pub const PCI_EXP_LNKCTL2_TLS_64_0GT: c_uint = 0x0006 /* Supported Speed 64GT/s */;
pub const PCI_EXP_LNKCTL2_ENTER_COMP: c_uint = 0x0010 /* Enter Compliance */;
pub const PCI_EXP_LNKCTL2_TX_MARGIN: c_uint = 0x0380 /* Transmit Margin */;
pub const PCI_EXP_LNKCTL2_HASD: c_uint = 0x0020 /* HW Autonomous Speed Disable */;
pub const PCI_EXP_LNKSTA2: c_uint = 0x32	/* Link Status 2 */;
pub const PCI_EXP_LNKSTA2_FLIT: c_uint = 0x0400 /* Flit Mode Status */;
pub const PCI_CAP_EXP_ENDPOINT_SIZEOF_V2: c_uint = 0x34	/* end of v2 EPs w/ link */;
pub const PCI_EXP_SLTCAP2: c_uint = 0x34	/* Slot Capabilities 2 */;
pub const PCI_EXP_SLTCAP2_IBPD: c_uint = 0x00000001 /* In-band PD Disable Supported */;
pub const PCI_EXP_SLTCTL2: c_uint = 0x38	/* Slot Control 2 */;
pub const PCI_EXP_SLTSTA2: c_uint = 0x3a	/* Slot Status 2 */;
// Extended Capabilities (PCI-X 2.0 and Express)

pub const PCI_EXT_CAP_ID_ERR: c_uint = 0x01	/* Advanced Error Reporting */;
pub const PCI_EXT_CAP_ID_VC: c_uint = 0x02	/* Virtual Channel Capability */;
pub const PCI_EXT_CAP_ID_DSN: c_uint = 0x03	/* Device Serial Number */;
pub const PCI_EXT_CAP_ID_PWR: c_uint = 0x04	/* Power Budgeting */;
pub const PCI_EXT_CAP_ID_RCLD: c_uint = 0x05	/* Root Complex Link Declaration */;
pub const PCI_EXT_CAP_ID_RCILC: c_uint = 0x06	/* Root Complex Internal Link Control */;
pub const PCI_EXT_CAP_ID_RCEC: c_uint = 0x07	/* Root Complex Event Collector */;
pub const PCI_EXT_CAP_ID_MFVC: c_uint = 0x08	/* Multi-Function VC Capability */;
pub const PCI_EXT_CAP_ID_VC9: c_uint = 0x09	/* same as _VC */;
pub const PCI_EXT_CAP_ID_RCRB: c_uint = 0x0A	/* Root Complex RB? */;
pub const PCI_EXT_CAP_ID_VNDR: c_uint = 0x0B	/* Vendor-Specific */;
pub const PCI_EXT_CAP_ID_CAC: c_uint = 0x0C	/* Config Access - obsolete */;
pub const PCI_EXT_CAP_ID_ACS: c_uint = 0x0D	/* Access Control Services */;
pub const PCI_EXT_CAP_ID_ARI: c_uint = 0x0E	/* Alternate Routing ID */;
pub const PCI_EXT_CAP_ID_ATS: c_uint = 0x0F	/* Address Translation Services */;
pub const PCI_EXT_CAP_ID_SRIOV: c_uint = 0x10	/* Single Root I/O Virtualization */;
pub const PCI_EXT_CAP_ID_MRIOV: c_uint = 0x11	/* Multi Root I/O Virtualization */;
pub const PCI_EXT_CAP_ID_MCAST: c_uint = 0x12	/* Multicast */;
pub const PCI_EXT_CAP_ID_PRI: c_uint = 0x13	/* Page Request Interface */;
pub const PCI_EXT_CAP_ID_AMD_XXX: c_uint = 0x14	/* Reserved for AMD */;
pub const PCI_EXT_CAP_ID_REBAR: c_uint = 0x15	/* Resizable BAR */;
pub const PCI_EXT_CAP_ID_DPA: c_uint = 0x16	/* Dynamic Power Allocation */;
pub const PCI_EXT_CAP_ID_TPH: c_uint = 0x17	/* TPH Requester */;
pub const PCI_EXT_CAP_ID_LTR: c_uint = 0x18	/* Latency Tolerance Reporting */;
pub const PCI_EXT_CAP_ID_SECPCI: c_uint = 0x19	/* Secondary PCIe Capability */;
pub const PCI_EXT_CAP_ID_PMUX: c_uint = 0x1A	/* Protocol Multiplexing */;
pub const PCI_EXT_CAP_ID_PASID: c_uint = 0x1B	/* Process Address Space ID */;
pub const PCI_EXT_CAP_ID_DPC: c_uint = 0x1D	/* Downstream Port Containment */;
pub const PCI_EXT_CAP_ID_L1SS: c_uint = 0x1E	/* L1 PM Substates */;
pub const PCI_EXT_CAP_ID_PTM: c_uint = 0x1F	/* Precision Time Measurement */;
pub const PCI_EXT_CAP_ID_DVSEC: c_uint = 0x23	/* Designated Vendor-Specific */;
pub const PCI_EXT_CAP_ID_VF_REBAR: c_uint = 0x24	/* VF Resizable BAR */;
pub const PCI_EXT_CAP_ID_DLF: c_uint = 0x25	/* Data Link Feature */;
pub const PCI_EXT_CAP_ID_PL_16GT: c_uint = 0x26	/* Physical Layer 16.0 GT/s */;
pub const PCI_EXT_CAP_ID_NPEM: c_uint = 0x29	/* Native PCIe Enclosure Management */;
pub const PCI_EXT_CAP_ID_PL_32GT: c_uint = 0x2A    /* Physical Layer 32.0 GT/s */;
pub const PCI_EXT_CAP_ID_DOE: c_uint = 0x2E	/* Data Object Exchange */;
pub const PCI_EXT_CAP_ID_DEV3: c_uint = 0x2F	/* Device 3 Capability/Control/Status */;
pub const PCI_EXT_CAP_ID_IDE: c_uint = 0x30    /* Integrity and Data Encryption */;
pub const PCI_EXT_CAP_ID_PL_64GT: c_uint = 0x31	/* Physical Layer 64.0 GT/s */;

pub const PCI_EXT_CAP_DSN_SIZEOF: c_int = 12;
pub const PCI_EXT_CAP_MCAST_ENDPOINT_SIZEOF: c_int = 40;
// Advanced Error Reporting
pub const PCI_ERR_UNCOR_STATUS: c_uint = 0x04	/* Uncorrectable Error Status */;
pub const PCI_ERR_UNC_UND: c_uint = 0x00000001	/* Undefined */;
pub const PCI_ERR_UNC_DLP: c_uint = 0x00000010	/* Data Link Protocol */;
pub const PCI_ERR_UNC_SURPDN: c_uint = 0x00000020	/* Surprise Down */;
pub const PCI_ERR_UNC_POISON_TLP: c_uint = 0x00001000	/* Poisoned TLP */;
pub const PCI_ERR_UNC_FCP: c_uint = 0x00002000	/* Flow Control Protocol */;
pub const PCI_ERR_UNC_COMP_TIME: c_uint = 0x00004000	/* Completion Timeout */;
pub const PCI_ERR_UNC_COMP_ABORT: c_uint = 0x00008000	/* Completer Abort */;
pub const PCI_ERR_UNC_UNX_COMP: c_uint = 0x00010000	/* Unexpected Completion */;
pub const PCI_ERR_UNC_RX_OVER: c_uint = 0x00020000	/* Receiver Overflow */;
pub const PCI_ERR_UNC_MALF_TLP: c_uint = 0x00040000	/* Malformed TLP */;
pub const PCI_ERR_UNC_ECRC: c_uint = 0x00080000	/* ECRC Error Status */;
pub const PCI_ERR_UNC_UNSUP: c_uint = 0x00100000	/* Unsupported Request */;
pub const PCI_ERR_UNC_ACSV: c_uint = 0x00200000	/* ACS Violation */;
pub const PCI_ERR_UNC_INTN: c_uint = 0x00400000	/* internal error */;
pub const PCI_ERR_UNC_MCBTLP: c_uint = 0x00800000	/* MC blocked TLP */;
pub const PCI_ERR_UNC_ATOMEG: c_uint = 0x01000000	/* Atomic egress blocked */;
pub const PCI_ERR_UNC_TLPPRE: c_uint = 0x02000000	/* TLP prefix blocked */;
pub const PCI_ERR_UNC_POISON_BLK: c_uint = 0x04000000	/* Poisoned TLP Egress Blocked */;
pub const PCI_ERR_UNC_DMWR_BLK: c_uint = 0x08000000	/* DMWr Request Egress Blocked */;
pub const PCI_ERR_UNC_IDE_CHECK: c_uint = 0x10000000	/* IDE Check Failed */;
pub const PCI_ERR_UNC_MISR_IDE: c_uint = 0x20000000	/* Misrouted IDE TLP */;
pub const PCI_ERR_UNC_PCRC_CHECK: c_uint = 0x40000000	/* PCRC Check Failed */;
pub const PCI_ERR_UNC_XLAT_BLK: c_uint = 0x80000000	/* TLP Translation Egress Blocked */;
pub const PCI_ERR_UNCOR_MASK: c_uint = 0x08	/* Uncorrectable Error Mask */;
// Same bits as above
pub const PCI_ERR_UNCOR_SEVER: c_uint = 0x0c	/* Uncorrectable Error Severity */;
// Same bits as above
pub const PCI_ERR_COR_STATUS: c_uint = 0x10	/* Correctable Error Status */;
pub const PCI_ERR_COR_RCVR: c_uint = 0x00000001	/* Receiver Error Status */;
pub const PCI_ERR_COR_BAD_TLP: c_uint = 0x00000040	/* Bad TLP Status */;
pub const PCI_ERR_COR_BAD_DLLP: c_uint = 0x00000080	/* Bad DLLP Status */;
pub const PCI_ERR_COR_REP_ROLL: c_uint = 0x00000100	/* REPLAY_NUM Rollover */;
pub const PCI_ERR_COR_REP_TIMER: c_uint = 0x00001000	/* Replay Timer Timeout */;
pub const PCI_ERR_COR_ADV_NFAT: c_uint = 0x00002000	/* Advisory Non-Fatal */;
pub const PCI_ERR_COR_INTERNAL: c_uint = 0x00004000	/* Corrected Internal */;
pub const PCI_ERR_COR_LOG_OVER: c_uint = 0x00008000	/* Header Log Overflow */;
pub const PCI_ERR_COR_MASK: c_uint = 0x14	/* Correctable Error Mask */;
// Same bits as above
pub const PCI_ERR_CAP: c_uint = 0x18	/* Advanced Error Capabilities & Ctrl*/;

pub const PCI_ERR_CAP_ECRC_GENC: c_uint = 0x00000020 /* ECRC Generation Capable */;
pub const PCI_ERR_CAP_ECRC_GENE: c_uint = 0x00000040 /* ECRC Generation Enable */;
pub const PCI_ERR_CAP_ECRC_CHKC: c_uint = 0x00000080 /* ECRC Check Capable */;
pub const PCI_ERR_CAP_ECRC_CHKE: c_uint = 0x00000100 /* ECRC Check Enable */;
pub const PCI_ERR_CAP_PREFIX_LOG_PRESENT: c_uint = 0x00000800 /* TLP Prefix Log Present */;
pub const PCI_ERR_CAP_COMP_TIME_LOG: c_uint = 0x00001000 /* Completion Timeout Prefix/Header Log Capable */;
pub const PCI_ERR_CAP_TLP_LOG_FLIT: c_uint = 0x00040000 /* TLP was logged in Flit Mode */;
pub const PCI_ERR_CAP_TLP_LOG_SIZE: c_uint = 0x00f80000 /* Logged TLP Size (only in Flit mode) */;
pub const PCI_ERR_HEADER_LOG: c_uint = 0x1c	/* Header Log Register (16 bytes) */;
pub const PCI_ERR_ROOT_COMMAND: c_uint = 0x2c	/* Root Error Command */;
pub const PCI_ERR_ROOT_CMD_COR_EN: c_uint = 0x00000001 /* Correctable Err Reporting Enable */;
pub const PCI_ERR_ROOT_CMD_NONFATAL_EN: c_uint = 0x00000002 /* Non-Fatal Err Reporting Enable */;
pub const PCI_ERR_ROOT_CMD_FATAL_EN: c_uint = 0x00000004 /* Fatal Err Reporting Enable */;
pub const PCI_ERR_ROOT_STATUS: c_uint = 0x30;
pub const PCI_ERR_ROOT_COR_RCV: c_uint = 0x00000001 /* ERR_COR Received */;
pub const PCI_ERR_ROOT_MULTI_COR_RCV: c_uint = 0x00000002 /* Multiple ERR_COR */;
pub const PCI_ERR_ROOT_UNCOR_RCV: c_uint = 0x00000004 /* ERR_FATAL/NONFATAL */;
pub const PCI_ERR_ROOT_MULTI_UNCOR_RCV: c_uint = 0x00000008 /* Multiple FATAL/NONFATAL */;
pub const PCI_ERR_ROOT_FIRST_FATAL: c_uint = 0x00000010 /* First UNC is Fatal */;
pub const PCI_ERR_ROOT_NONFATAL_RCV: c_uint = 0x00000020 /* Non-Fatal Received */;
pub const PCI_ERR_ROOT_FATAL_RCV: c_uint = 0x00000040 /* Fatal Received */;
pub const PCI_ERR_ROOT_AER_IRQ: c_uint = 0xf8000000 /* Advanced Error Interrupt Message Number */;
pub const PCI_ERR_ROOT_ERR_SRC: c_uint = 0x34	/* Error Source Identification */;
pub const PCI_ERR_PREFIX_LOG: c_uint = 0x38	/* TLP Prefix LOG Register (up to 16 bytes) */;
// Virtual Channel
pub const PCI_VC_PORT_CAP1: c_uint = 0x04;
pub const PCI_VC_CAP1_EVCC: c_uint = 0x00000007	/* extended VC count */;
pub const PCI_VC_CAP1_LPEVCC: c_uint = 0x00000070	/* low prio extended VC count */;
pub const PCI_VC_CAP1_ARB_SIZE: c_uint = 0x00000c00;
pub const PCI_VC_PORT_CAP2: c_uint = 0x08;
pub const PCI_VC_CAP2_32_PHASE: c_uint = 0x00000002;
pub const PCI_VC_CAP2_64_PHASE: c_uint = 0x00000004;
pub const PCI_VC_CAP2_128_PHASE: c_uint = 0x00000008;
pub const PCI_VC_CAP2_ARB_OFF: c_uint = 0xff000000;
pub const PCI_VC_PORT_CTRL: c_uint = 0x0c;
pub const PCI_VC_PORT_CTRL_LOAD_TABLE: c_uint = 0x00000001;
pub const PCI_VC_PORT_STATUS: c_uint = 0x0e;
pub const PCI_VC_PORT_STATUS_TABLE: c_uint = 0x00000001;
pub const PCI_VC_RES_CAP: c_uint = 0x10;
pub const PCI_VC_RES_CAP_32_PHASE: c_uint = 0x00000002;
pub const PCI_VC_RES_CAP_64_PHASE: c_uint = 0x00000004;
pub const PCI_VC_RES_CAP_128_PHASE: c_uint = 0x00000008;
pub const PCI_VC_RES_CAP_128_PHASE_TB: c_uint = 0x00000010;
pub const PCI_VC_RES_CAP_256_PHASE: c_uint = 0x00000020;
pub const PCI_VC_RES_CAP_ARB_OFF: c_uint = 0xff000000;
pub const PCI_VC_RES_CTRL: c_uint = 0x14;
pub const PCI_VC_RES_CTRL_LOAD_TABLE: c_uint = 0x00010000;
pub const PCI_VC_RES_CTRL_ARB_SELECT: c_uint = 0x000e0000;
pub const PCI_VC_RES_CTRL_ID: c_uint = 0x07000000;
pub const PCI_VC_RES_CTRL_ENABLE: c_uint = 0x80000000;
pub const PCI_VC_RES_STATUS: c_uint = 0x1a;
pub const PCI_VC_RES_STATUS_TABLE: c_uint = 0x00000001;
pub const PCI_VC_RES_STATUS_NEGO: c_uint = 0x00000002;
pub const PCI_CAP_VC_BASE_SIZEOF: c_uint = 0x10;
pub const PCI_CAP_VC_PER_VC_SIZEOF: c_uint = 0x0c;
// Power Budgeting
pub const PCI_PWR_DSR: c_uint = 0x04	/* Data Select Register */;
pub const PCI_PWR_DATA: c_uint = 0x08	/* Data Register */;

pub const PCI_PWR_CAP: c_uint = 0x0c	/* Capability */;

pub const PCI_EXT_CAP_PWR_SIZEOF: c_uint = 0x10;
// Root Complex Event Collector Endpoint Association

pub const PCI_RCEC_BUSN_REG_VER: c_uint = 0x02	/* Least version with BUSN present */;

// Vendor-Specific (VSEC, PCI_EXT_CAP_ID_VNDR)

//
// HyperTransport sub capability types
//
// Unfortunately there are both 3 bit and 5 bit capability types defined
// in the HT spec, catering for that is a little messy. You probably don't
// want to use these directly, just use pci_find_ht_capability() and it
// will do the right thing for you.
//
pub const HT_3BIT_CAP_MASK: c_uint = 0xE0;
pub const HT_CAPTYPE_SLAVE: c_uint = 0x00	/* Slave/Primary link configuration */;
pub const HT_CAPTYPE_HOST: c_uint = 0x20	/* Host/Secondary link configuration */;
pub const HT_5BIT_CAP_MASK: c_uint = 0xF8;
pub const HT_CAPTYPE_IRQ: c_uint = 0x80	/* IRQ Configuration */;
pub const HT_CAPTYPE_REMAPPING_40: c_uint = 0xA0	/* 40 bit address remapping */;
pub const HT_CAPTYPE_REMAPPING_64: c_uint = 0xA2	/* 64 bit address remapping */;
pub const HT_CAPTYPE_UNITID_CLUMP: c_uint = 0x90	/* Unit ID clumping */;
pub const HT_CAPTYPE_EXTCONF: c_uint = 0x98	/* Extended Configuration Space Access */;
pub const HT_CAPTYPE_MSI_MAPPING: c_uint = 0xA8	/* MSI Mapping Capability */;
pub const HT_MSI_FLAGS: c_uint = 0x02		/* Offset to flags */;
pub const HT_MSI_FLAGS_ENABLE: c_uint = 0x1		/* Mapping enable */;
pub const HT_MSI_FLAGS_FIXED: c_uint = 0x2		/* Fixed mapping only */;
pub const HT_MSI_FIXED_ADDR: c_uint = 0x00000000FEE00000ULL	/* Fixed addr */;
pub const HT_MSI_ADDR_LO: c_uint = 0x04		/* Offset to low addr bits */;
pub const HT_MSI_ADDR_LO_MASK: c_uint = 0xFFF00000	/* Low address bit mask */;
pub const HT_MSI_ADDR_HI: c_uint = 0x08		/* Offset to high addr bits */;
pub const HT_CAPTYPE_DIRECT_ROUTE: c_uint = 0xB0	/* Direct routing configuration */;
pub const HT_CAPTYPE_VCSET: c_uint = 0xB8	/* Virtual Channel configuration */;
pub const HT_CAPTYPE_ERROR_RETRY: c_uint = 0xC0	/* Retry on error configuration */;
pub const HT_CAPTYPE_GEN3: c_uint = 0xD0	/* Generation 3 HyperTransport configuration */;
pub const HT_CAPTYPE_PM: c_uint = 0xE0	/* HyperTransport power management configuration */;

// Alternative Routing-ID Interpretation
pub const PCI_ARI_CAP: c_uint = 0x04	/* ARI Capability Register */;
pub const PCI_ARI_CAP_MFVC: c_uint = 0x0001	/* MFVC Function Groups Capability */;
pub const PCI_ARI_CAP_ACS: c_uint = 0x0002	/* ACS Function Groups Capability */;

pub const PCI_ARI_CTRL: c_uint = 0x06	/* ARI Control Register */;
pub const PCI_ARI_CTRL_MFVC: c_uint = 0x0001	/* MFVC Function Groups Enable */;
pub const PCI_ARI_CTRL_ACS: c_uint = 0x0002	/* ACS Function Groups Enable */;

pub const PCI_EXT_CAP_ARI_SIZEOF: c_int = 8;
// Address Translation Service
pub const PCI_ATS_CAP: c_uint = 0x04	/* ATS Capability Register */;

pub const PCI_ATS_CAP_PAGE_ALIGNED: c_uint = 0x0020 /* Page Aligned Request */;
pub const PCI_ATS_CTRL: c_uint = 0x06	/* ATS Control Register */;
pub const PCI_ATS_CTRL_ENABLE: c_uint = 0x8000	/* ATS Enable */;

pub const PCI_EXT_CAP_ATS_SIZEOF: c_int = 8;
// Page Request Interface
pub const PCI_PRI_CTRL: c_uint = 0x04	/* PRI control register */;
pub const PCI_PRI_CTRL_ENABLE: c_uint = 0x0001	/* Enable */;
pub const PCI_PRI_CTRL_RESET: c_uint = 0x0002	/* Reset */;
pub const PCI_PRI_STATUS: c_uint = 0x06	/* PRI status register */;
pub const PCI_PRI_STATUS_RF: c_uint = 0x0001	/* Response Failure */;
pub const PCI_PRI_STATUS_UPRGI: c_uint = 0x0002	/* Unexpected PRG index */;
pub const PCI_PRI_STATUS_STOPPED: c_uint = 0x0100	/* PRI Stopped */;
pub const PCI_PRI_STATUS_PASID: c_uint = 0x8000	/* PRG Response PASID Required */;
pub const PCI_PRI_MAX_REQ: c_uint = 0x08	/* PRI max reqs supported */;
pub const PCI_PRI_ALLOC_REQ: c_uint = 0x0c	/* PRI max reqs allowed */;
pub const PCI_EXT_CAP_PRI_SIZEOF: c_int = 16;
// Process Address Space ID
pub const PCI_PASID_CAP: c_uint = 0x04    /* PASID feature register */;
pub const PCI_PASID_CAP_EXEC: c_uint = 0x0002	/* Exec permissions Supported */;
pub const PCI_PASID_CAP_PRIV: c_uint = 0x0004	/* Privilege Mode Supported */;
pub const PCI_PASID_CAP_WIDTH: c_uint = 0x1f00;
pub const PCI_PASID_CTRL: c_uint = 0x06    /* PASID control register */;
pub const PCI_PASID_CTRL_ENABLE: c_uint = 0x0001	/* Enable bit */;
pub const PCI_PASID_CTRL_EXEC: c_uint = 0x0002	/* Exec permissions Enable */;
pub const PCI_PASID_CTRL_PRIV: c_uint = 0x0004	/* Privilege Mode Enable */;
pub const PCI_EXT_CAP_PASID_SIZEOF: c_int = 8;
// Single Root I/O Virtualization
pub const PCI_SRIOV_CAP: c_uint = 0x04	/* SR-IOV Capabilities */;
pub const PCI_SRIOV_CAP_VFM: c_uint = 0x00000001  /* VF Migration Capable */;

pub const PCI_SRIOV_CTRL: c_uint = 0x08	/* SR-IOV Control */;
pub const PCI_SRIOV_CTRL_VFE: c_uint = 0x0001	/* VF Enable */;
pub const PCI_SRIOV_CTRL_VFM: c_uint = 0x0002	/* VF Migration Enable */;
pub const PCI_SRIOV_CTRL_INTR: c_uint = 0x0004	/* VF Migration Interrupt Enable */;
pub const PCI_SRIOV_CTRL_MSE: c_uint = 0x0008	/* VF Memory Space Enable */;
pub const PCI_SRIOV_CTRL_ARI: c_uint = 0x0010	/* ARI Capable Hierarchy */;
pub const PCI_SRIOV_STATUS: c_uint = 0x0a	/* SR-IOV Status */;
pub const PCI_SRIOV_STATUS_VFM: c_uint = 0x0001	/* VF Migration Status */;
pub const PCI_SRIOV_INITIAL_VF: c_uint = 0x0c	/* Initial VFs */;
pub const PCI_SRIOV_TOTAL_VF: c_uint = 0x0e	/* Total VFs */;
pub const PCI_SRIOV_NUM_VF: c_uint = 0x10	/* Number of VFs */;
pub const PCI_SRIOV_FUNC_LINK: c_uint = 0x12	/* Function Dependency Link */;
pub const PCI_SRIOV_VF_OFFSET: c_uint = 0x14	/* First VF Offset */;
pub const PCI_SRIOV_VF_STRIDE: c_uint = 0x16	/* Following VF Stride */;
pub const PCI_SRIOV_VF_DID: c_uint = 0x1a	/* VF Device ID */;
pub const PCI_SRIOV_SUP_PGSIZE: c_uint = 0x1c	/* Supported Page Sizes */;
pub const PCI_SRIOV_SYS_PGSIZE: c_uint = 0x20	/* System Page Size */;
pub const PCI_SRIOV_BAR: c_uint = 0x24	/* VF BAR0 */;

pub const PCI_SRIOV_VFM: c_uint = 0x3c	/* VF Migration State Array Offset*/;

pub const PCI_SRIOV_VFM_UA: c_uint = 0x0	/* Inactive.Unavailable */;
pub const PCI_SRIOV_VFM_MI: c_uint = 0x1	/* Dormant.MigrateIn */;
pub const PCI_SRIOV_VFM_MO: c_uint = 0x2	/* Active.MigrateOut */;
pub const PCI_SRIOV_VFM_AV: c_uint = 0x3	/* Active.Available */;
pub const PCI_EXT_CAP_SRIOV_SIZEOF: c_uint = 0x40;
pub const PCI_LTR_MAX_SNOOP_LAT: c_uint = 0x4;
pub const PCI_LTR_MAX_NOSNOOP_LAT: c_uint = 0x6;
pub const PCI_LTR_VALUE_MASK: c_uint = 0x000003ff;
pub const PCI_LTR_SCALE_MASK: c_uint = 0x00001c00;
pub const PCI_LTR_SCALE_SHIFT: c_int = 10;
pub const PCI_LTR_NOSNOOP_VALUE: c_uint = 0x03ff0000 /* Max No-Snoop Latency Value */;
pub const PCI_LTR_NOSNOOP_SCALE: c_uint = 0x1c000000 /* Scale for Max Value */;
pub const PCI_EXT_CAP_LTR_SIZEOF: c_int = 8;
// Access Control Service
pub const PCI_ACS_CAP: c_uint = 0x04	/* ACS Capability Register */;
pub const PCI_ACS_SV: c_uint = 0x0001	/* Source Validation */;
pub const PCI_ACS_TB: c_uint = 0x0002	/* Translation Blocking */;
pub const PCI_ACS_RR: c_uint = 0x0004	/* P2P Request Redirect */;
pub const PCI_ACS_CR: c_uint = 0x0008	/* P2P Completion Redirect */;
pub const PCI_ACS_UF: c_uint = 0x0010	/* Upstream Forwarding */;
pub const PCI_ACS_EC: c_uint = 0x0020	/* P2P Egress Control */;
pub const PCI_ACS_DT: c_uint = 0x0040	/* Direct Translated P2P */;
pub const PCI_ACS_EGRESS_BITS: c_uint = 0x05	/* ACS Egress Control Vector Size */;
pub const PCI_ACS_CTRL: c_uint = 0x06	/* ACS Control Register */;
pub const PCI_ACS_EGRESS_CTL_V: c_uint = 0x08	/* ACS Egress Control Vector */;
// SATA capability

pub const PCI_SATA_REGS_MASK: c_uint = 0xF	/* location - BAR#/inline */;
pub const PCI_SATA_REGS_INLINE: c_uint = 0xF	/* REGS in config space */;
pub const PCI_SATA_SIZEOF_SHORT: c_int = 8;
pub const PCI_SATA_SIZEOF_LONG: c_int = 16;
// Resizable BARs

pub const PCI_REBAR_CAP_SIZES: c_uint = 0xFFFFFFF0  /* supported BAR sizes */;

pub const PCI_REBAR_CTRL_BAR_IDX: c_uint = 0x00000007  /* BAR index */;
pub const PCI_REBAR_CTRL_NBAR_MASK: c_uint = 0x000000E0  /* # of resizable BARs */;

pub const PCI_REBAR_CTRL_BAR_SIZE: c_uint = 0x00001F00  /* BAR size */;

// Dynamic Power Allocation

pub const PCI_DPA_CAP_SUBSTATE_MASK: c_uint = 0x1F	/* # substates - 1 */;

// TPH Completer Support
pub const PCI_EXP_DEVCAP2_TPH_COMP_NONE: c_uint = 0x0 /* None */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_TPH_ONLY: c_uint = 0x1 /* TPH only */;
pub const PCI_EXP_DEVCAP2_TPH_COMP_EXT_TPH: c_uint = 0x3 /* TPH and Extended TPH */;
// TPH Requester

pub const PCI_TPH_CAP_ST_NS: c_uint = 0x00000001 /* No ST Mode Supported */;
pub const PCI_TPH_CAP_ST_IV: c_uint = 0x00000002 /* Interrupt Vector Mode Supported */;
pub const PCI_TPH_CAP_ST_DS: c_uint = 0x00000004 /* Device Specific Mode Supported */;
pub const PCI_TPH_CAP_EXT_TPH: c_uint = 0x00000100 /* Ext TPH Requester Supported */;
pub const PCI_TPH_CAP_LOC_MASK: c_uint = 0x00000600 /* ST Table Location */;
pub const PCI_TPH_LOC_NONE: c_uint = 0x00000000 /* Not present */;
pub const PCI_TPH_LOC_CAP: c_uint = 0x00000200 /* In capability */;
pub const PCI_TPH_LOC_MSIX: c_uint = 0x00000400 /* In MSI-X */;
pub const PCI_TPH_CAP_ST_MASK: c_uint = 0x07FF0000 /* ST Table Size */;

pub const PCI_TPH_BASE_SIZEOF: c_uint = 0xc	/* Size with no ST table */;

pub const PCI_TPH_CTRL_MODE_SEL_MASK: c_uint = 0x00000007 /* ST Mode Select */;
pub const PCI_TPH_ST_NS_MODE: c_uint = 0x0 /* No ST Mode */;
pub const PCI_TPH_ST_IV_MODE: c_uint = 0x1 /* Interrupt Vector Mode */;
pub const PCI_TPH_ST_DS_MODE: c_uint = 0x2 /* Device Specific Mode */;
pub const PCI_TPH_CTRL_REQ_EN_MASK: c_uint = 0x00000300 /* TPH Requester Enable */;
pub const PCI_TPH_REQ_DISABLE: c_uint = 0x0 /* No TPH requests allowed */;
pub const PCI_TPH_REQ_TPH_ONLY: c_uint = 0x1 /* TPH only requests allowed */;
pub const PCI_TPH_REQ_EXT_TPH: c_uint = 0x3 /* Extended TPH requests allowed */;
// Downstream Port Containment
pub const PCI_EXP_DPC_CAP: c_uint = 0x04	/* DPC Capability */;
pub const PCI_EXP_DPC_IRQ: c_uint = 0x001F	/* Interrupt Message Number */;
pub const PCI_EXP_DPC_CAP_RP_EXT: c_uint = 0x0020	/* Root Port Extensions */;
pub const PCI_EXP_DPC_CAP_POISONED_TLP: c_uint = 0x0040	/* Poisoned TLP Egress Blocking Supported */;
pub const PCI_EXP_DPC_CAP_SW_TRIGGER: c_uint = 0x0080	/* Software Triggering Supported */;
pub const PCI_EXP_DPC_RP_PIO_LOG_SIZE: c_uint = 0x0F00	/* RP PIO Log Size [3:0] */;
pub const PCI_EXP_DPC_CAP_DL_ACTIVE: c_uint = 0x1000	/* ERR_COR signal on DL_Active supported */;
pub const PCI_EXP_DPC_RP_PIO_LOG_SIZE4: c_uint = 0x2000	/* RP PIO Log Size [4] */;
pub const PCI_EXP_DPC_CTL: c_uint = 0x06	/* DPC control */;
pub const PCI_EXP_DPC_CTL_EN_FATAL: c_uint = 0x0001	/* Enable trigger on ERR_FATAL message */;
pub const PCI_EXP_DPC_CTL_EN_NONFATAL: c_uint = 0x0002	/* Enable trigger on ERR_NONFATAL message */;
pub const PCI_EXP_DPC_CTL_INT_EN: c_uint = 0x0008	/* DPC Interrupt Enable */;
pub const PCI_EXP_DPC_STATUS: c_uint = 0x08	/* DPC Status */;
pub const PCI_EXP_DPC_STATUS_TRIGGER: c_uint = 0x0001 /* Trigger Status */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN: c_uint = 0x0006 /* Trigger Reason */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_UNCOR: c_uint = 0x0000 /* Uncorrectable error */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_NFE: c_uint = 0x0002 /* Rcvd ERR_NONFATAL */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_FE: c_uint = 0x0004 /* Rcvd ERR_FATAL */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_IN_EXT: c_uint = 0x0006 /* Reason in Trig Reason Extension field */;
pub const PCI_EXP_DPC_STATUS_INTERRUPT: c_uint = 0x0008 /* Interrupt Status */;
pub const PCI_EXP_DPC_RP_BUSY: c_uint = 0x0010 /* Root Port Busy */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_EXT: c_uint = 0x0060 /* Trig Reason Extension */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_RP_PIO: c_uint = 0x0000	/* RP PIO error */;
pub const PCI_EXP_DPC_STATUS_TRIGGER_RSN_SW_TRIGGER: c_uint = 0x0020	/* DPC SW Trigger bit */;
pub const PCI_EXP_DPC_RP_PIO_FEP: c_uint = 0x1f00 /* RP PIO First Err Ptr */;
pub const PCI_EXP_DPC_SOURCE_ID: c_uint = 0x0A	/* DPC Source Identifier */;
pub const PCI_EXP_DPC_RP_PIO_STATUS: c_uint = 0x0C	/* RP PIO Status */;
pub const PCI_EXP_DPC_RP_PIO_MASK: c_uint = 0x10	/* RP PIO Mask */;
pub const PCI_EXP_DPC_RP_PIO_SEVERITY: c_uint = 0x14	/* RP PIO Severity */;
pub const PCI_EXP_DPC_RP_PIO_SYSERROR: c_uint = 0x18	/* RP PIO SysError */;
pub const PCI_EXP_DPC_RP_PIO_EXCEPTION: c_uint = 0x1C	/* RP PIO Exception */;
pub const PCI_EXP_DPC_RP_PIO_HEADER_LOG: c_uint = 0x20	/* RP PIO Header Log */;
pub const PCI_EXP_DPC_RP_PIO_IMPSPEC_LOG: c_uint = 0x30	/* RP PIO ImpSpec Log */;
pub const PCI_EXP_DPC_RP_PIO_TLPPREFIX_LOG: c_uint = 0x34	/* RP PIO TLP Prefix Log */;
// Precision Time Measurement
pub const PCI_PTM_CAP: c_uint = 0x04	    /* PTM Capability */;
pub const PCI_PTM_CAP_REQ: c_uint = 0x00000001  /* Requester capable */;
pub const PCI_PTM_CAP_RES: c_uint = 0x00000002  /* Responder capable */;
pub const PCI_PTM_CAP_ROOT: c_uint = 0x00000004  /* Root capable */;
pub const PCI_PTM_GRANULARITY_MASK: c_uint = 0x0000FF00  /* Clock granularity */;
pub const PCI_PTM_CTRL: c_uint = 0x08	    /* PTM Control */;
pub const PCI_PTM_CTRL_ENABLE: c_uint = 0x00000001  /* PTM enable */;
pub const PCI_PTM_CTRL_ROOT: c_uint = 0x00000002  /* Root select */;
// ASPM L1 PM Substates
pub const PCI_L1SS_CAP: c_uint = 0x04	/* Capabilities Register */;
pub const PCI_L1SS_CAP_PCIPM_L1_2: c_uint = 0x00000001  /* PCI-PM L1.2 Supported */;
pub const PCI_L1SS_CAP_PCIPM_L1_1: c_uint = 0x00000002  /* PCI-PM L1.1 Supported */;
pub const PCI_L1SS_CAP_ASPM_L1_2: c_uint = 0x00000004  /* ASPM L1.2 Supported */;
pub const PCI_L1SS_CAP_ASPM_L1_1: c_uint = 0x00000008  /* ASPM L1.1 Supported */;
pub const PCI_L1SS_CAP_L1_PM_SS: c_uint = 0x00000010  /* L1 PM Substates Supported */;
pub const PCI_L1SS_CAP_CM_RESTORE_TIME: c_uint = 0x0000ff00  /* Port Common_Mode_Restore_Time */;
pub const PCI_L1SS_CAP_P_PWR_ON_SCALE: c_uint = 0x00030000  /* Port T_POWER_ON scale */;
pub const PCI_L1SS_CAP_P_PWR_ON_VALUE: c_uint = 0x00f80000  /* Port T_POWER_ON value */;
pub const PCI_L1SS_CTL1: c_uint = 0x08	/* Control 1 Register */;
pub const PCI_L1SS_CTL1_PCIPM_L1_2: c_uint = 0x00000001  /* PCI-PM L1.2 Enable */;
pub const PCI_L1SS_CTL1_PCIPM_L1_1: c_uint = 0x00000002  /* PCI-PM L1.1 Enable */;
pub const PCI_L1SS_CTL1_ASPM_L1_2: c_uint = 0x00000004  /* ASPM L1.2 Enable */;
pub const PCI_L1SS_CTL1_ASPM_L1_1: c_uint = 0x00000008  /* ASPM L1.1 Enable */;
pub const PCI_L1SS_CTL1_L1_2_MASK: c_uint = 0x00000005;
pub const PCI_L1SS_CTL1_L1SS_MASK: c_uint = 0x0000000f;
pub const PCI_L1SS_CTL1_CM_RESTORE_TIME: c_uint = 0x0000ff00  /* Common_Mode_Restore_Time */;
pub const PCI_L1SS_CTL1_LTR_L12_TH_VALUE: c_uint = 0x03ff0000  /* LTR_L1.2_THRESHOLD_Value */;
pub const PCI_L1SS_CTL1_LTR_L12_TH_SCALE: c_uint = 0xe0000000  /* LTR_L1.2_THRESHOLD_Scale */;
pub const PCI_L1SS_CTL2: c_uint = 0x0c	/* Control 2 Register */;
pub const PCI_L1SS_CTL2_T_PWR_ON_SCALE: c_uint = 0x00000003  /* T_POWER_ON Scale */;
pub const PCI_L1SS_CTL2_T_PWR_ON_VALUE: c_uint = 0x000000f8  /* T_POWER_ON Value */;
// Designated Vendor-Specific (DVSEC, PCI_EXT_CAP_ID_DVSEC)
pub const PCI_DVSEC_HEADER1: c_uint = 0x4 /* Designated Vendor-Specific Header1 */;

pub const PCI_DVSEC_HEADER2: c_uint = 0x8 /* Designated Vendor-Specific Header2 */;

// VF Resizable BARs, same layout as PCI_REBAR

// Data Link Feature
pub const PCI_DLF_CAP: c_uint = 0x04	/* Capabilities Register */;
pub const PCI_DLF_EXCHANGE_ENABLE: c_uint = 0x80000000  /* Data Link Feature Exchange Enable */;
// Secondary PCIe Capability 8.0 GT/s
pub const PCI_SECPCI_LE_CTRL: c_uint = 0x0c /* Lane Equalization Control Register */;
// Physical Layer 16.0 GT/s
pub const PCI_PL_16GT_LE_CTRL: c_uint = 0x20	/* Lane Equalization Control Register */;
pub const PCI_PL_16GT_LE_CTRL_DSP_TX_PRESET_MASK: c_uint = 0x0000000F;
pub const PCI_PL_16GT_LE_CTRL_USP_TX_PRESET_MASK: c_uint = 0x000000F0;
pub const PCI_PL_16GT_LE_CTRL_USP_TX_PRESET_SHIFT: c_int = 4;
// Physical Layer 32.0 GT/s
pub const PCI_PL_32GT_LE_CTRL: c_uint = 0x20	/* Lane Equalization Control Register */;
// Physical Layer 64.0 GT/s
pub const PCI_PL_64GT_LE_CTRL: c_uint = 0x20	/* Lane Equalization Control Register */;
// Native PCIe Enclosure Management
pub const PCI_NPEM_CAP: c_uint = 0x04 /* NPEM capability register */;
pub const PCI_NPEM_CAP_CAPABLE: c_uint = 0x00000001 /* NPEM Capable */;
pub const PCI_NPEM_CTRL: c_uint = 0x08 /* NPEM control register */;
pub const PCI_NPEM_CTRL_ENABLE: c_uint = 0x00000001 /* NPEM Enable */;
//
// Native PCIe Enclosure Management indication bits and Reset command bit
// are corresponding for capability and control registers.
//
pub const PCI_NPEM_CMD_RESET: c_uint = 0x00000002 /* Reset Command */;
pub const PCI_NPEM_IND_OK: c_uint = 0x00000004 /* OK */;
pub const PCI_NPEM_IND_LOCATE: c_uint = 0x00000008 /* Locate */;
pub const PCI_NPEM_IND_FAIL: c_uint = 0x00000010 /* Fail */;
pub const PCI_NPEM_IND_REBUILD: c_uint = 0x00000020 /* Rebuild */;
pub const PCI_NPEM_IND_PFA: c_uint = 0x00000040 /* Predicted Failure Analysis */;
pub const PCI_NPEM_IND_HOTSPARE: c_uint = 0x00000080 /* Hot Spare */;
pub const PCI_NPEM_IND_ICA: c_uint = 0x00000100 /* In Critical Array */;
pub const PCI_NPEM_IND_IFA: c_uint = 0x00000200 /* In Failed Array */;
pub const PCI_NPEM_IND_IDT: c_uint = 0x00000400 /* Device Type */;
pub const PCI_NPEM_IND_DISABLED: c_uint = 0x00000800 /* Disabled */;
pub const PCI_NPEM_IND_SPEC_0: c_uint = 0x01000000;
pub const PCI_NPEM_IND_SPEC_1: c_uint = 0x02000000;
pub const PCI_NPEM_IND_SPEC_2: c_uint = 0x04000000;
pub const PCI_NPEM_IND_SPEC_3: c_uint = 0x08000000;
pub const PCI_NPEM_IND_SPEC_4: c_uint = 0x10000000;
pub const PCI_NPEM_IND_SPEC_5: c_uint = 0x20000000;
pub const PCI_NPEM_IND_SPEC_6: c_uint = 0x40000000;
pub const PCI_NPEM_IND_SPEC_7: c_uint = 0x80000000;
pub const PCI_NPEM_STATUS: c_uint = 0x0c /* NPEM status register */;
pub const PCI_NPEM_STATUS_CC: c_uint = 0x00000001 /* Command Completed */;
// Data Object Exchange
pub const PCI_DOE_CAP: c_uint = 0x04    /* DOE Capabilities Register */;
pub const PCI_DOE_CAP_INT_SUP: c_uint = 0x00000001  /* Interrupt Support */;
pub const PCI_DOE_CAP_INT_MSG_NUM: c_uint = 0x00000ffe  /* Interrupt Message Number */;
pub const PCI_DOE_CTRL: c_uint = 0x08    /* DOE Control Register */;
pub const PCI_DOE_CTRL_ABORT: c_uint = 0x00000001  /* DOE Abort */;
pub const PCI_DOE_CTRL_INT_EN: c_uint = 0x00000002  /* DOE Interrupt Enable */;
pub const PCI_DOE_CTRL_GO: c_uint = 0x80000000  /* DOE Go */;
pub const PCI_DOE_STATUS: c_uint = 0x0c    /* DOE Status Register */;
pub const PCI_DOE_STATUS_BUSY: c_uint = 0x00000001  /* DOE Busy */;
pub const PCI_DOE_STATUS_INT_STATUS: c_uint = 0x00000002  /* DOE Interrupt Status */;
pub const PCI_DOE_STATUS_ERROR: c_uint = 0x00000004  /* DOE Error */;
pub const PCI_DOE_STATUS_DATA_OBJECT_READY: c_uint = 0x80000000  /* Data Object Ready */;
pub const PCI_DOE_WRITE: c_uint = 0x10    /* DOE Write Data Mailbox Register */;
pub const PCI_DOE_READ: c_uint = 0x14    /* DOE Read Data Mailbox Register */;
pub const PCI_DOE_CAP_SIZEOF: c_uint = 0x18	/* Size of DOE register block */;
// DOE Data Object - note not actually registers
pub const PCI_DOE_DATA_OBJECT_HEADER_1_VID: c_uint = 0x0000ffff;
pub const PCI_DOE_DATA_OBJECT_HEADER_1_TYPE: c_uint = 0x00ff0000;
pub const PCI_DOE_DATA_OBJECT_HEADER_2_LENGTH: c_uint = 0x0003ffff;
pub const PCI_DOE_DATA_OBJECT_DISC_REQ_3_INDEX: c_uint = 0x000000ff;
pub const PCI_DOE_DATA_OBJECT_DISC_REQ_3_VER: c_uint = 0x0000ff00;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_VID: c_uint = 0x0000ffff;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE: c_uint = 0x00ff0000;
pub const PCI_DOE_DATA_OBJECT_DISC_RSP_3_NEXT_INDEX: c_uint = 0xff000000;
// Deprecated old name, replaced with PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE

// Device 3 Extended Capability
pub const PCI_DEV3_CAP: c_uint = 0x04	/* Device 3 Capabilities Register */;
pub const PCI_DEV3_CTL: c_uint = 0x08	/* Device 3 Control Register */;
pub const PCI_DEV3_STA: c_uint = 0x0c	/* Device 3 Status Register */;
pub const PCI_DEV3_STA_SEGMENT: c_uint = 0x8	/* Segment Captured (end-to-end flit-mode detected) */;
// Integrity and Data Encryption Extended Capability
pub const PCI_IDE_CAP: c_uint = 0x04;
pub const PCI_IDE_CAP_LINK: c_uint = 0x1  /* Link IDE Stream Supported */;
pub const PCI_IDE_CAP_SELECTIVE: c_uint = 0x2  /* Selective IDE Streams Supported */;
pub const PCI_IDE_CAP_FLOWTHROUGH: c_uint = 0x4  /* Flow-Through IDE Stream Supported */;
pub const PCI_IDE_CAP_PARTIAL_HEADER_ENC: c_uint = 0x8  /* Partial Header Encryption Supported */;
pub const PCI_IDE_CAP_AGGREGATION: c_uint = 0x10 /* Aggregation Supported */;
pub const PCI_IDE_CAP_PCRC: c_uint = 0x20 /* PCRC Supported */;
pub const PCI_IDE_CAP_IDE_KM: c_uint = 0x40 /* IDE_KM Protocol Supported */;
pub const PCI_IDE_CAP_SEL_CFG: c_uint = 0x80 /* Selective IDE for Config Request Support */;

pub const PCI_IDE_CAP_TEE_LIMITED: c_uint = 0x1000000 /* TEE-Limited Stream Supported */;
pub const PCI_IDE_CTL: c_uint = 0x08;
pub const PCI_IDE_CTL_FLOWTHROUGH_IDE: c_uint = 0x4  /* Flow-Through IDE Stream Enabled */;
pub const PCI_IDE_LINK_STREAM_0: c_uint = 0xc  /* First Link Stream Register Block */;
pub const PCI_IDE_LINK_BLOCK_SIZE: c_int = 8;
// Link IDE Stream block, up to PCI_IDE_CAP_LINK_TC_NUM
pub const PCI_IDE_LINK_CTL_0: c_uint = 0x00		  /* First Link Control Register Offset in block */;
pub const PCI_IDE_LINK_CTL_EN: c_uint = 0x1		  /* Link IDE Stream Enable */;

pub const PCI_IDE_LINK_CTL_PCRC_EN: c_uint = 0x100		  /* PCRC Enable */;

pub const PCI_IDE_LINK_STS_0: c_uint = 0x4               /* First Link Status Register Offset in block */;

pub const PCI_IDE_LINK_STS_IDE_FAIL: c_uint = 0x80000000	  /* IDE fail message received */;
// Selective IDE Stream block, up to PCI_IDE_CAP_SELECTIVE_STREAMS_NUM
// Selective IDE Stream Capability Register
pub const PCI_IDE_SEL_CAP: c_uint = 0x00;

// Selective IDE Stream Control Register
pub const PCI_IDE_SEL_CTL: c_uint = 0x04;
pub const PCI_IDE_SEL_CTL_EN: c_uint = 0x1		  /* Selective IDE Stream Enable */;

pub const PCI_IDE_SEL_CTL_PCRC_EN: c_uint = 0x100		  /* PCRC Enable */;
pub const PCI_IDE_SEL_CTL_CFG_EN: c_uint = 0x200		  /* Selective IDE for Configuration Requests */;

pub const PCI_IDE_SEL_CTL_DEFAULT: c_uint = 0x400000	  /* Default Stream */;
pub const PCI_IDE_SEL_CTL_TEE_LIMITED: c_uint = 0x800000	  /* TEE-Limited Stream */;

pub const PCI_IDE_SEL_CTL_ID_MAX: c_int = 255;
// Selective IDE Stream Status Register
pub const PCI_IDE_SEL_STS: c_uint = 0x08;

pub const PCI_IDE_SEL_STS_STATE_INSECURE: c_int = 0;
pub const PCI_IDE_SEL_STS_STATE_SECURE: c_int = 2;
pub const PCI_IDE_SEL_STS_IDE_FAIL: c_uint = 0x80000000	 /* IDE fail message received */;
// IDE RID Association Register 1
pub const PCI_IDE_SEL_RID_1: c_uint = 0x0c;

// IDE RID Association Register 2
pub const PCI_IDE_SEL_RID_2: c_uint = 0x10;
pub const PCI_IDE_SEL_RID_2_VALID: c_uint = 0x1;

// Selective IDE Address Association Register Block, up to PCI_IDE_SEL_CAP_ASSOC_NUM
pub const PCI_IDE_SEL_ADDR_BLOCK_SIZE: c_int = 12;

pub const PCI_IDE_SEL_ADDR_1_VALID: c_uint = 0x1;

// IDE Address Association Register 2 is "Memory Limit Upper"

// IDE Address Association Register 3 is "Memory Base Upper"

//
// Compute Express Link (CXL r4.0, sec 8.1)
//
// Note that CXL DVSEC id 3 and 7 to be ignored when the CXL link state
// is "disconnected" (CXL r4.0, sec 9.12.3). Re-enumerate these
// registers on downstream link-up events.
//
// CXL r4.0, 8.1.3: PCIe DVSEC for CXL Device
pub const PCI_DVSEC_CXL_DEVICE: c_int = 0;
pub const PCI_DVSEC_CXL_CAP: c_uint = 0xA;

pub const PCI_DVSEC_CXL_CTRL: c_uint = 0xC;

pub const CXL_DVSEC_RANGE_MAX: c_int = 2;
// CXL r4.0, 8.1.4: Non-CXL Function Map DVSEC
pub const PCI_DVSEC_CXL_FUNCTION_MAP: c_int = 2;
// CXL r4.0, 8.1.5: Extensions DVSEC for Ports
pub const PCI_DVSEC_CXL_PORT: c_int = 3;
pub const PCI_DVSEC_CXL_PORT_CTL: c_uint = 0x0c;
pub const PCI_DVSEC_CXL_PORT_CTL_UNMASK_SBR: c_uint = 0x00000001;
// CXL r4.0, 8.1.6: GPF DVSEC for CXL Port
pub const PCI_DVSEC_CXL_PORT_GPF: c_int = 4;
pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_1_CONTROL: c_uint = 0x0C;

pub const PCI_DVSEC_CXL_PORT_GPF_PHASE_2_CONTROL: c_uint = 0xE;

// CXL r4.0, 8.1.7: GPF DVSEC for CXL Device
pub const PCI_DVSEC_CXL_DEVICE_GPF: c_int = 5;
// CXL r4.0, 8.1.8: Flex Bus DVSEC
pub const PCI_DVSEC_CXL_FLEXBUS_PORT: c_int = 7;
pub const PCI_DVSEC_CXL_FLEXBUS_PORT_STATUS: c_uint = 0xE;

// CXL r4.0, 8.1.9: Register Locator DVSEC
pub const PCI_DVSEC_CXL_REG_LOCATOR: c_int = 8;
pub const PCI_DVSEC_CXL_REG_LOCATOR_BLOCK1: c_uint = 0xC;

