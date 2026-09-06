//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/fsl_pamu.h
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
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//

// Bit Field macros
// v = bit field variable; m = mask, m##_SHIFT = shift, x = value to load
//

// PAMU CCSR space
pub const PAMU_PGC: c_uint = 0x00000000     /* Allows all peripheral accesses */;
pub const PAMU_PE: c_uint = 0x40000000      /* enable PAMU                    */;
// PAMU_OFFSET to the next pamu space in ccsr
pub const PAMU_OFFSET: c_uint = 0x1000;
pub const PAMU_MMAP_REGS_BASE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pamu_mmap_regs {
    pub ppbah: u32,
    pub ppbal: u32,
    pub pplah: u32,
    pub pplal: u32,
    pub spbah: u32,
    pub spbal: u32,
    pub splah: u32,
    pub splal: u32,
    pub obah: u32,
    pub obal: u32,
    pub olah: u32,
    pub olal: u32,
}

// PAMU Error Registers
pub const PAMU_POES1: c_uint = 0x0040;
pub const PAMU_POES2: c_uint = 0x0044;
pub const PAMU_POEAH: c_uint = 0x0048;
pub const PAMU_POEAL: c_uint = 0x004C;
pub const PAMU_AVS1: c_uint = 0x0050;
pub const PAMU_AVS1_AV: c_uint = 0x1;
pub const PAMU_AVS1_OTV: c_uint = 0x6;
pub const PAMU_AVS1_APV: c_uint = 0x78;
pub const PAMU_AVS1_WAV: c_uint = 0x380;
pub const PAMU_AVS1_LAV: c_uint = 0x1c00;
pub const PAMU_AVS1_GCV: c_uint = 0x2000;
pub const PAMU_AVS1_PDV: c_uint = 0x4000;

pub const PAMU_AVS1_LIODN_SHIFT: c_int = 16;
pub const PAMU_LAV_LIODN_NOT_IN_PPAACT: c_uint = 0x400;
pub const PAMU_AVS2: c_uint = 0x0054;
pub const PAMU_AVAH: c_uint = 0x0058;
pub const PAMU_AVAL: c_uint = 0x005C;
pub const PAMU_EECTL: c_uint = 0x0060;
pub const PAMU_EEDIS: c_uint = 0x0064;
pub const PAMU_EEINTEN: c_uint = 0x0068;
pub const PAMU_EEDET: c_uint = 0x006C;
pub const PAMU_EEATTR: c_uint = 0x0070;
pub const PAMU_EEAHI: c_uint = 0x0074;
pub const PAMU_EEALO: c_uint = 0x0078;

pub const PAMU_EEDLO: c_uint = 0x0080;
pub const PAMU_EECC: c_uint = 0x0084;
pub const PAMU_UDAD: c_uint = 0x0090;
// PAMU Revision Registers
pub const PAMU_PR1: c_uint = 0x0BF8;
pub const PAMU_PR2: c_uint = 0x0BFC;
// PAMU version mask
pub const PAMU_PR1_MASK: c_uint = 0xffff;
// PAMU Capabilities Registers
pub const PAMU_PC1: c_uint = 0x0C00;
pub const PAMU_PC2: c_uint = 0x0C04;
pub const PAMU_PC3: c_uint = 0x0C08;
pub const PAMU_PC4: c_uint = 0x0C0C;
// PAMU Control Register
pub const PAMU_PC: c_uint = 0x0C10;
// PAMU control defs
pub const PAMU_CONTROL: c_uint = 0x0C10;
pub const PAMU_PC_PGC: c_uint = 0x80000000  /* PAMU gate closed bit */;
pub const PAMU_PC_PE: c_uint = 0x40000000 /* PAMU enable bit */;
pub const PAMU_PC_SPCC: c_uint = 0x00000010 /* sPAACE cache enable */;
pub const PAMU_PC_PPCC: c_uint = 0x00000001 /* pPAACE cache enable */;
pub const PAMU_PC_OCE: c_uint = 0x00001000 /* OMT cache enable */;
pub const PAMU_PFA1: c_uint = 0x0C14;
pub const PAMU_PFA2: c_uint = 0x0C18;

// PAMU Interrupt control and Status Register
pub const PAMU_PICS: c_uint = 0x0C1C;
pub const PAMU_ACCESS_VIOLATION_STAT: c_uint = 0x8;
pub const PAMU_ACCESS_VIOLATION_ENABLE: c_uint = 0x4;
// PAMU Debug Registers
pub const PAMU_PD1: c_uint = 0x0F00;
pub const PAMU_PD2: c_uint = 0x0F04;
pub const PAMU_PD3: c_uint = 0x0F08;
pub const PAMU_PD4: c_uint = 0x0F0C;
pub const PAACE_AP_PERMS_DENIED: c_uint = 0x0;
pub const PAACE_AP_PERMS_QUERY: c_uint = 0x1;
pub const PAACE_AP_PERMS_UPDATE: c_uint = 0x2;
pub const PAACE_AP_PERMS_ALL: c_uint = 0x3;
pub const PAACE_DD_TO_HOST: c_uint = 0x0;
pub const PAACE_DD_TO_IO: c_uint = 0x1;
pub const PAACE_PT_PRIMARY: c_uint = 0x0;
pub const PAACE_PT_SECONDARY: c_uint = 0x1;
pub const PAACE_V_INVALID: c_uint = 0x0;
pub const PAACE_V_VALID: c_uint = 0x1;
pub const PAACE_MW_SUBWINDOWS: c_uint = 0x1;
pub const PAACE_WSE_4K: c_uint = 0xB;
pub const PAACE_WSE_8K: c_uint = 0xC;
pub const PAACE_WSE_16K: c_uint = 0xD;
pub const PAACE_WSE_32K: c_uint = 0xE;
pub const PAACE_WSE_64K: c_uint = 0xF;
pub const PAACE_WSE_128K: c_uint = 0x10;
pub const PAACE_WSE_256K: c_uint = 0x11;
pub const PAACE_WSE_512K: c_uint = 0x12;
pub const PAACE_WSE_1M: c_uint = 0x13;
pub const PAACE_WSE_2M: c_uint = 0x14;
pub const PAACE_WSE_4M: c_uint = 0x15;
pub const PAACE_WSE_8M: c_uint = 0x16;
pub const PAACE_WSE_16M: c_uint = 0x17;
pub const PAACE_WSE_32M: c_uint = 0x18;
pub const PAACE_WSE_64M: c_uint = 0x19;
pub const PAACE_WSE_128M: c_uint = 0x1A;
pub const PAACE_WSE_256M: c_uint = 0x1B;
pub const PAACE_WSE_512M: c_uint = 0x1C;
pub const PAACE_WSE_1G: c_uint = 0x1D;
pub const PAACE_WSE_2G: c_uint = 0x1E;
pub const PAACE_WSE_4G: c_uint = 0x1F;
pub const PAACE_DID_PCI_EXPRESS_1: c_uint = 0x00;
pub const PAACE_DID_PCI_EXPRESS_2: c_uint = 0x01;
pub const PAACE_DID_PCI_EXPRESS_3: c_uint = 0x02;
pub const PAACE_DID_PCI_EXPRESS_4: c_uint = 0x03;
pub const PAACE_DID_LOCAL_BUS: c_uint = 0x04;
pub const PAACE_DID_SRIO: c_uint = 0x0C;
pub const PAACE_DID_MEM_1: c_uint = 0x10;
pub const PAACE_DID_MEM_2: c_uint = 0x11;
pub const PAACE_DID_MEM_3: c_uint = 0x12;
pub const PAACE_DID_MEM_4: c_uint = 0x13;
pub const PAACE_DID_MEM_1_2: c_uint = 0x14;
pub const PAACE_DID_MEM_3_4: c_uint = 0x15;
pub const PAACE_DID_MEM_1_4: c_uint = 0x16;
pub const PAACE_DID_BM_SW_PORTAL: c_uint = 0x18;
pub const PAACE_DID_PAMU: c_uint = 0x1C;
pub const PAACE_DID_CAAM: c_uint = 0x21;
pub const PAACE_DID_QM_SW_PORTAL: c_uint = 0x3C;
pub const PAACE_DID_CORE0_INST: c_uint = 0x80;
pub const PAACE_DID_CORE0_DATA: c_uint = 0x81;
pub const PAACE_DID_CORE1_INST: c_uint = 0x82;
pub const PAACE_DID_CORE1_DATA: c_uint = 0x83;
pub const PAACE_DID_CORE2_INST: c_uint = 0x84;
pub const PAACE_DID_CORE2_DATA: c_uint = 0x85;
pub const PAACE_DID_CORE3_INST: c_uint = 0x86;
pub const PAACE_DID_CORE3_DATA: c_uint = 0x87;
pub const PAACE_DID_CORE4_INST: c_uint = 0x88;
pub const PAACE_DID_CORE4_DATA: c_uint = 0x89;
pub const PAACE_DID_CORE5_INST: c_uint = 0x8A;
pub const PAACE_DID_CORE5_DATA: c_uint = 0x8B;
pub const PAACE_DID_CORE6_INST: c_uint = 0x8C;
pub const PAACE_DID_CORE6_DATA: c_uint = 0x8D;
pub const PAACE_DID_CORE7_INST: c_uint = 0x8E;
pub const PAACE_DID_CORE7_DATA: c_uint = 0x8F;
pub const PAACE_DID_BROADCAST: c_uint = 0xFF;
pub const PAACE_ATM_NO_XLATE: c_uint = 0x00;
pub const PAACE_ATM_WINDOW_XLATE: c_uint = 0x01;
pub const PAACE_ATM_PAGE_XLATE: c_uint = 0x02;

pub const PAACE_OTM_NO_XLATE: c_uint = 0x00;
pub const PAACE_OTM_IMMEDIATE: c_uint = 0x01;
pub const PAACE_OTM_INDEXED: c_uint = 0x02;
pub const PAACE_OTM_RESERVED: c_uint = 0x03;
pub const PAACE_M_COHERENCE_REQ: c_uint = 0x01;
pub const PAACE_PID_0: c_uint = 0x0;
pub const PAACE_PID_1: c_uint = 0x1;
pub const PAACE_PID_2: c_uint = 0x2;
pub const PAACE_PID_3: c_uint = 0x3;
pub const PAACE_PID_4: c_uint = 0x4;
pub const PAACE_PID_5: c_uint = 0x5;
pub const PAACE_PID_6: c_uint = 0x6;
pub const PAACE_PID_7: c_uint = 0x7;
pub const PAACE_TCEF_FORMAT0_8B: c_uint = 0x00;
pub const PAACE_TCEF_FORMAT1_RSVD: c_uint = 0x01;
//
// Hard coded value for the PAACT size to accommodate
// maximum LIODN value generated by u-boot.
//
pub const PAACE_NUMBER_ENTRIES: c_uint = 0x500;
// Hard coded value for the SPAACT size
pub const SPAACE_NUMBER_ENTRIES: c_uint = 0x800;
pub const OME_NUMBER_ENTRIES: c_int = 16;
// PAACE Bit Field Defines
pub const PPAACE_AF_WBAL: c_uint = 0xfffff000;
pub const PPAACE_AF_WBAL_SHIFT: c_int = 12;
pub const PPAACE_AF_WSE: c_uint = 0x00000fc0;
pub const PPAACE_AF_WSE_SHIFT: c_int = 6;
pub const PPAACE_AF_MW: c_uint = 0x00000020;
pub const PPAACE_AF_MW_SHIFT: c_int = 5;
pub const SPAACE_AF_LIODN: c_uint = 0xffff0000;
pub const SPAACE_AF_LIODN_SHIFT: c_int = 16;
pub const PAACE_AF_AP: c_uint = 0x00000018;
pub const PAACE_AF_AP_SHIFT: c_int = 3;
pub const PAACE_AF_DD: c_uint = 0x00000004;
pub const PAACE_AF_DD_SHIFT: c_int = 2;
pub const PAACE_AF_PT: c_uint = 0x00000002;
pub const PAACE_AF_PT_SHIFT: c_int = 1;
pub const PAACE_AF_V: c_uint = 0x00000001;
pub const PAACE_AF_V_SHIFT: c_int = 0;
pub const PAACE_DA_HOST_CR: c_uint = 0x80;
pub const PAACE_DA_HOST_CR_SHIFT: c_int = 7;
pub const PAACE_IA_CID: c_uint = 0x00FF0000;
pub const PAACE_IA_CID_SHIFT: c_int = 16;
pub const PAACE_IA_WCE: c_uint = 0x000000F0;
pub const PAACE_IA_WCE_SHIFT: c_int = 4;
pub const PAACE_IA_ATM: c_uint = 0x0000000C;
pub const PAACE_IA_ATM_SHIFT: c_int = 2;
pub const PAACE_IA_OTM: c_uint = 0x00000003;
pub const PAACE_IA_OTM_SHIFT: c_int = 0;
pub const PAACE_WIN_TWBAL: c_uint = 0xfffff000;
pub const PAACE_WIN_TWBAL_SHIFT: c_int = 12;
pub const PAACE_WIN_SWSE: c_uint = 0x00000fc0;
pub const PAACE_WIN_SWSE_SHIFT: c_int = 6;
// PAMU Data Structures
// primary / secondary paact structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct paace {
// PAACE Offset 0x00
    pub /: *mut *mut u32 wbah; / only valid for Primary PAACE,
    pub /: *mut *mut *mut u32 addr_bitfields; / See P/S PAACE_AF_,
// PAACE Offset 0x08
// Interpretation of first 32 bits dependent on DD above
// Destination ID, see PAACE_DID_* defines
    pub did: u8,
// Partition ID
    pub pid: u8,
// Snoop ID
    pub snpid: u8,
// coherency_required : 1 reserved : 7
    pub /: *mut *mut *mut u8 coherency_required; / See PAACE_DA_,
    pub to_host: },
// Destination ID, see PAACE_DID_* defines
    pub did: u8,
    pub reserved1: u8,
    pub reserved2: u16,
    pub to_io: },
    pub domain_attr: },
// Implementation attributes + window count + address & operation translation modes
    pub /: *mut *mut *mut u32 impl_attr; / See PAACE_IA_,
// PAACE Offset 0x10
// Translated window base address
    pub twbah: u32,
    pub /: *mut *mut *mut u32 win_bitfields; / See PAACE_WIN_,
// PAACE Offset 0x18
// first secondary paace entry
    pub /: *mut *mut u32 fspi; / only valid for Primary PAACE,
    pub ioea: u8,
    pub moea: u8,
    pub ioeb: u8,
    pub moeb: u8,
    pub immed_ot: },
    pub reserved: u16,
    pub omi: u16,
    pub index_ot: },
    pub op_encode: },
// PAACE Offsets 0x20-0x38
    pub /: *mut *mut u32 reserved[8]; / not currently implemented,
}

// OME : Operation mapping entry
// MOE : Mapped Operation Encodings
// The operation mapping table is table containing operation mapping entries (OME).
// The index of a particular OME is programmed in the PAACE entry for translation
// in bound I/O operations corresponding to an LIODN. The OMT is used for translation
// specifically in case of the indexed translation mode. Each OME contains a 128
// byte mapped operation encoding (MOE), where each byte represents an MOE.
//
pub const NUM_MOE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ome {
    pub moe: [u8; NUM_MOE],
    pub __packed: },

pub const PAMU_PAGE_SHIFT: c_int = 12;

pub const IOE_READ: c_uint = 0x00;
pub const IOE_READ_IDX: c_uint = 0x00;
pub const IOE_WRITE: c_uint = 0x81;
pub const IOE_WRITE_IDX: c_uint = 0x01;
pub const IOE_EREAD0: c_uint = 0x82    /* Enhanced read type 0 */;
pub const IOE_EREAD0_IDX: c_uint = 0x02    /* Enhanced read type 0 */;
pub const IOE_EWRITE0: c_uint = 0x83    /* Enhanced write type 0 */;
pub const IOE_EWRITE0_IDX: c_uint = 0x03    /* Enhanced write type 0 */;
pub const IOE_DIRECT0: c_uint = 0x84    /* Directive type 0 */;
pub const IOE_DIRECT0_IDX: c_uint = 0x04    /* Directive type 0 */;
pub const IOE_EREAD1: c_uint = 0x85    /* Enhanced read type 1 */;
pub const IOE_EREAD1_IDX: c_uint = 0x05    /* Enhanced read type 1 */;
pub const IOE_EWRITE1: c_uint = 0x86    /* Enhanced write type 1 */;
pub const IOE_EWRITE1_IDX: c_uint = 0x06    /* Enhanced write type 1 */;
pub const IOE_DIRECT1: c_uint = 0x87    /* Directive type 1 */;
pub const IOE_DIRECT1_IDX: c_uint = 0x07    /* Directive type 1 */;
pub const IOE_RAC: c_uint = 0x8c    /* Read with Atomic clear */;
pub const IOE_RAC_IDX: c_uint = 0x0c    /* Read with Atomic clear */;
pub const IOE_RAS: c_uint = 0x8d    /* Read with Atomic set */;
pub const IOE_RAS_IDX: c_uint = 0x0d    /* Read with Atomic set */;
pub const IOE_RAD: c_uint = 0x8e    /* Read with Atomic decrement */;
pub const IOE_RAD_IDX: c_uint = 0x0e    /* Read with Atomic decrement */;
pub const IOE_RAI: c_uint = 0x8f    /* Read with Atomic increment */;
pub const IOE_RAI_IDX: c_uint = 0x0f    /* Read with Atomic increment */;
pub const EOE_READ: c_uint = 0x00;
pub const EOE_WRITE: c_uint = 0x01;
pub const EOE_RAC: c_uint = 0x0c    /* Read with Atomic clear */;
pub const EOE_RAS: c_uint = 0x0d    /* Read with Atomic set */;
pub const EOE_RAD: c_uint = 0x0e    /* Read with Atomic decrement */;
pub const EOE_RAI: c_uint = 0x0f    /* Read with Atomic increment */;
pub const EOE_LDEC: c_uint = 0x10    /* Load external cache */;
pub const EOE_LDECL: c_uint = 0x11    /* Load external cache with stash lock */;
pub const EOE_LDECPE: c_uint = 0x12    /* Load external cache with preferred exclusive */;
pub const EOE_LDECPEL: c_uint = 0x13    /* Load external cache with preferred exclusive and lock */;
pub const EOE_LDECFE: c_uint = 0x14    /* Load external cache with forced exclusive */;
pub const EOE_LDECFEL: c_uint = 0x15    /* Load external cache with forced exclusive and lock */;
pub const EOE_RSA: c_uint = 0x16    /* Read with stash allocate */;
pub const EOE_RSAU: c_uint = 0x17    /* Read with stash allocate and unlock */;
pub const EOE_READI: c_uint = 0x18    /* Read with invalidate */;
pub const EOE_RWNITC: c_uint = 0x19    /* Read with no intention to cache */;
pub const EOE_WCI: c_uint = 0x1a    /* Write cache inhibited */;
pub const EOE_WWSA: c_uint = 0x1b    /* Write with stash allocate */;
pub const EOE_WWSAL: c_uint = 0x1c    /* Write with stash allocate and lock */;
pub const EOE_WWSAO: c_uint = 0x1d    /* Write with stash allocate only */;
pub const EOE_WWSAOL: c_uint = 0x1e    /* Write with stash allocate only and lock */;
pub const EOE_VALID: c_uint = 0x80;
// Function prototypes
    pub pamu_domain_init(void): c_int,
    pub liodn): int pamu_enable_liodn(int,
    pub liodn): int pamu_disable_liodn(int,
    pub prot): int pamu_config_ppaace(int liodn, u32 omi, uint32_t stashid, int,
    pub vcpu): u32 get_stash_id(u32 stash_dest_hint, u32,
    pub dev): *mut *mut void get_ome_index(u32 omi_index, struct device,
    pub value): int pamu_update_paace_stash(int liodn, u32,
