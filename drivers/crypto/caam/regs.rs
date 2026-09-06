//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/regs.h
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
// CAAM hardware register-level view
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
// Copyright 2018, 2023 NXP
//

//
// Architecture-specific register access methods
//
// CAAM's bus-addressable registers are 64 bits internally.
// They have been wired to be safely accessible on 32-bit
// architectures, however. Registers were organized such
// that (a) they can be contained in 32 bits, (b) if not, then they
// can be treated as two 32-bit entities, or finally (c) if they
// must be treated as a single 64-bit value, then this can safely
// be done with two 32-bit cycles.
//
// For 32-bit operations on 64-bit values, CAAM follows the same
// 64-bit register access conventions as it's predecessors, in that
// writes are "triggered" by a write to the register at the numerically
// higher address, thus, a full 64-bit write cycle requires a write
// to the lower address, followed by a write to the higher address,
// which will latch/execute the write cycle.
//
// For example, let's assume a SW reset of CAAM through the master
// configuration register.
// - SWRST is in bit 31 of MCFG.
// - MCFG begins at base+0x0000.
// - Bits 63-32 are a 32-bit word at base+0x0000 (numerically-lower)
// - Bits 31-0 are a 32-bit word at base+0x0004 (numerically-higher)
//
// (and on Power, the convention is 0-31, 32-63, I know...)
//
// Assuming a 64-bit write to this MCFG to perform a software reset
// would then require a write of 0 to base+0x0000, followed by a
// write of 0x80000000 to base+0x0004, which would "execute" the
// reset.
//
// Of course, since MCFG 63-32 is all zero, we could cheat and simply
// write 0x8000000 to base+0x0004, and the reset would work fine.
// However, since CAAM does contain some write-and-read-intended
// 64-bit registers, this code defines 64-bit access methods for
// the sake of internal consistency and simplicity, and so that a
// clean transition to 64-bit is possible when it becomes necessary.
//
// There are limitations to this that the developer must recognize.
// 32-bit architectures cannot enforce an atomic-64 operation,
// Therefore:
//
// - On writes, since the HW is assumed to latch the cycle on the
// write of the higher-numeric-address word, then ordered
// writes work OK.
//
// - For reads, where a register contains a relevant value of more
// that 32 bits, the hardware employs logic to latch the other
// "half" of the data until read, ensuring an accurate value.
// This is of particular relevance when dealing with CAAM's
// performance counters.
//

extern "C" {
    pub fn ioread32(_arg: reg) -> return;
}
extern "C" {
    pub fn ioread32be(_arg: reg) -> return;
}
//
// The only users of these wr/rd_reg64 functions is the Job Ring (JR).
// The DMA address registers in the JR are handled differently depending on
// platform:
//
// 1. All BE CAAM platforms and i.MX platforms (LE CAAM):
//
// base + 0x0000 : most-significant 32 bits
// base + 0x0004 : least-significant 32 bits
//
// The 32-bit version of this core therefore has to write to base + 0x0004
// to set the 32-bit wide DMA address.
//
// 2. All other LE CAAM platforms (LS1021A etc.)
// base + 0x0000 : least-significant 32 bits
// base + 0x0004 : most-significant 32 bits
//
extern "C" {
    pub fn ioread64(_arg: reg) -> return;
}
extern "C" {
    pub fn ioread64be(_arg: reg) -> return;
}
extern "C" {
    pub fn cpu_to_caam64(_arg: value) -> return;
}
extern "C" {
    pub fn caam64_to_cpu(_arg: value) -> return;
}
extern "C" {
    pub fn cpu_to_caam_dma64(_arg: value) -> return;
}
extern "C" {
    pub fn cpu_to_caam32(_arg: value) -> return;
}
extern "C" {
    pub fn caam_dma64_to_cpu(_arg: value) -> return;
}
extern "C" {
    pub fn caam32_to_cpu(_arg: value) -> return;
}
//
// jr_outentry
// Represents each entry in a JobR output ring
//
// desc = outentry[hw_idx].desc;
// jrstatus = outentry[hw_idx].jrstatus;
// desc = outentry[hw_idx].desc;
// jrstatus = outentry[hw_idx].jrstatus;

// Version registers (Era 10+)	e80-eff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_regs {
    pub /: *mut *mut u32 crca; / CRCA_VERSION,
    pub /: *mut *mut u32 afha; / AFHA_VERSION,
    pub /: *mut *mut u32 kfha; / KFHA_VERSION,
    pub /: *mut *mut u32 pkha; / PKHA_VERSION,
    pub /: *mut *mut u32 aesa; / AESA_VERSION,
    pub /: *mut *mut u32 mdha; / MDHA_VERSION,
    pub /: *mut *mut u32 desa; / DESA_VERSION,
    pub /: *mut *mut u32 snw8a; / SNW8A_VERSION,
    pub /: *mut *mut u32 snw9a; / SNW9A_VERSION,
    pub /: *mut *mut u32 zuce; / ZUCE_VERSION,
    pub /: *mut *mut u32 zuca; / ZUCA_VERSION,
    pub /: *mut *mut u32 ccha; / CCHA_VERSION,
    pub /: *mut *mut u32 ptha; / PTHA_VERSION,
    pub /: *mut *mut u32 rng; / RNG_VERSION,
    pub /: *mut *mut u32 trng; / TRNG_VERSION,
    pub /: *mut *mut u32 aaha; / AAHA_VERSION,
    pub rsvd: [u32; 10],
    pub /: *mut *mut u32 sr; / SR_VERSION,
    pub /: *mut *mut u32 dma; / DMA_VERSION,
    pub /: *mut *mut u32 ai; / AI_VERSION,
    pub /: *mut *mut u32 qi; / QI_VERSION,
    pub /: *mut *mut u32 jr; / JR_VERSION,
    pub /: *mut *mut u32 deco; / DECO_VERSION,
}

// Version registers bitfields
// Number of CHAs instantiated
pub const CHA_VER_NUM_MASK: c_uint = 0xffull;
// CHA Miscellaneous Information
pub const CHA_VER_MISC_SHIFT: c_int = 8;

// CHA Revision Number
pub const CHA_VER_REV_SHIFT: c_int = 16;

// CHA Version ID
pub const CHA_VER_VID_SHIFT: c_int = 24;

// CHA Miscellaneous Information - AESA_MISC specific

// CHA Miscellaneous Information - PKHA_MISC specific

//
// caam_perfmon - Performance Monitor/Secure Memory Status
// CAAM Global Status/Component Version IDs
//
// Spans f00-fff wherever instantiated
//
// Number of DECOs
pub const CHA_NUM_MS_DECONUM_SHIFT: c_int = 24;

//
// CHA version IDs / instantiation bitfields (< Era 10)
// Defined for use with the cha_id fields in perfmon, but the same shift/mask
// selectors can be used to pull out the number of instantiated blocks within
// cha_num fields in perfmon because the locations are the same.
//
pub const CHA_ID_LS_AES_SHIFT: c_int = 0;

pub const CHA_ID_LS_DES_SHIFT: c_int = 4;

pub const CHA_ID_LS_ARC4_SHIFT: c_int = 8;

pub const CHA_ID_LS_MD_SHIFT: c_int = 12;

pub const CHA_ID_LS_RNG_SHIFT: c_int = 16;

pub const CHA_ID_LS_SNW8_SHIFT: c_int = 20;

pub const CHA_ID_LS_KAS_SHIFT: c_int = 24;

pub const CHA_ID_LS_PK_SHIFT: c_int = 28;

pub const CHA_ID_MS_CRC_SHIFT: c_int = 0;

pub const CHA_ID_MS_SNW9_SHIFT: c_int = 4;

pub const CHA_ID_MS_DECO_SHIFT: c_int = 24;

pub const CHA_ID_MS_JR_SHIFT: c_int = 28;

// Specific CHA version IDs
pub const CHA_VER_VID_AES_LP: c_uint = 0x3ull;
pub const CHA_VER_VID_AES_HP: c_uint = 0x4ull;
pub const CHA_VER_VID_MD_LP256: c_uint = 0x0ull;
pub const CHA_VER_VID_MD_LP512: c_uint = 0x1ull;
pub const CHA_VER_VID_MD_HP: c_uint = 0x2ull;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_vid {
    pub ip_id: u16,
    pub maj_rev: u8,
    pub min_rev: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_perfmon {
// Performance Monitor Registers			f00-f9f
    pub /: *mut *mut u64 req_dequeued; / PC_REQ_DEQ - Dequeued Requests,
    pub /: *mut *mut u64 ob_enc_req; / PC_OB_ENC_REQ - Outbound Encrypt Requests,
    pub /: *mut *mut u64 ib_dec_req; / PC_IB_DEC_REQ - Inbound Decrypt Requests,
    pub /: *mut *mut u64 ob_enc_bytes; / PC_OB_ENCRYPT - Outbound Bytes Encrypted,
    pub /: *mut *mut u64 ob_prot_bytes; / PC_OB_PROTECT - Outbound Bytes Protected,
    pub /: *mut *mut u64 ib_dec_bytes; / PC_IB_DECRYPT - Inbound Bytes Decrypted,
    pub /: *mut *mut u64 ib_valid_bytes; / PC_IB_VALIDATED Inbound Bytes Validated,
    pub rsvd: [u64; 13],
// CAAM Hardware Instantiation Parameters		fa0-fbf
    pub half*/: *mut *mut u32 cha_rev_ms; / CRNR - CHA Rev No. Most significant,
    pub half*/: *mut *mut u32 cha_rev_ls; / CRNR - CHA Rev No. Least significant,
pub const CTPR_MS_QI_SHIFT: c_int = 25;

pub const CTPR_MS_VIRT_EN_INCL: c_uint = 0x00000001;
pub const CTPR_MS_VIRT_EN_POR: c_uint = 0x00000002;
pub const CTPR_MS_PG_SZ_MASK: c_uint = 0x10;
pub const CTPR_MS_PG_SZ_SHIFT: c_int = 4;
    pub /: *mut *mut u32 comp_parms_ms; / CTPR - Compile Parameters Register,

    pub /: *mut *mut u32 comp_parms_ls; / CTPR - Compile Parameters Register,
    pub rsvd1: [u64; 2],
// CAAM Global Status					fc0-fdf
    pub /: *mut *mut u64 faultaddr; / FAR - Fault Address,
    pub /: *mut *mut u32 faultliodn; / FALR - Fault Address LIODN,
    pub /: *mut *mut u32 faultdetail; / FADR - Fault Addr Detail,
    pub rsvd2: u32,

pub const CSTA_MOO_SECURE: c_int = 1;
pub const CSTA_MOO_TRUSTED: c_int = 2;
    pub /: *mut *mut u32 status; / CSTA - CAAM Status,
    pub rsvd3: u64,
// Component Instantiation Parameters			fe0-fff
    pub /: *mut *mut u32 rtic_id; / RVID - RTIC Version ID,
pub const CCBVID_ERA_MASK: c_uint = 0xff000000;
pub const CCBVID_ERA_SHIFT: c_int = 24;
    pub /: *mut *mut u32 ccb_id; / CCBVID - CCB Version ID,
    pub Significant*/: *mut *mut u32 cha_id_ms; / CHAVID - CHA Version ID Most,
    pub Significant*/: *mut *mut u32 cha_id_ls; / CHAVID - CHA Version ID Least,
    pub /: *mut *mut u32 cha_num_ms; / CHANUM - CHA Number Most Significant,
    pub Significant*/: *mut *mut u32 cha_num_ls; / CHANUM - CHA Number Least,
pub const SECVID_MS_IPID_MASK: c_uint = 0xffff0000;
pub const SECVID_MS_IPID_SHIFT: c_int = 16;
pub const SECVID_MS_MAJ_REV_MASK: c_uint = 0x0000ff00;
pub const SECVID_MS_MAJ_REV_SHIFT: c_int = 8;
    pub /: *mut *mut u32 caam_id_ms; / CAAMVID - CAAM Version ID MS,
    pub /: *mut *mut u32 caam_id_ls; / CAAMVID - CAAM Version ID LS,
}

// LIODN programming for DMA configuration
pub const MSTRID_LOCK_LIODN: c_uint = 0x80000000;
pub const MSTRID_LOCK_MAKETRUSTED: c_uint = 0x00010000	/* only for JR masterid */;
pub const MSTRID_LIODN_MASK: c_uint = 0x0fff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct masterid {
    pub /: *mut *mut u32 liodn_ms; / lock and make-trusted control bits,
    pub /: *mut *mut u32 liodn_ls; / LIODN for non-sequence and seq access,
}

// RNGB test mode (replicated twice in some configurations)
// Padded out to 0x100
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rngtst {
    pub /: *mut *mut u32 mode; / RTSTMODEx - Test mode,
    pub rsvd1: [u32; 3],
    pub /: *mut *mut u32 reset; / RTSTRESETx - Test reset control,
    pub rsvd2: [u32; 3],
    pub /: *mut *mut u32 status; / RTSTSSTATUSx - Test status,
    pub rsvd3: u32,
    pub /: *mut *mut u32 errstat; / RTSTERRSTATx - Test error status,
    pub rsvd4: u32,
    pub /: *mut *mut u32 errctl; / RTSTERRCTLx - Test error control,
    pub rsvd5: u32,
    pub /: *mut *mut u32 entropy; / RTSTENTROPYx - Test entropy,
    pub rsvd6: [u32; 15],
    pub /: *mut *mut u32 verifctl; / RTSTVERIFCTLx - Test verification control,
    pub rsvd7: u32,
    pub /: *mut *mut u32 verifstat; / RTSTVERIFSTATx - Test verification status,
    pub rsvd8: u32,
    pub /: *mut *mut u32 verifdata; / RTSTVERIFDx - Test verification data,
    pub rsvd9: u32,
    pub /: *mut *mut u32 xkey; / RTSTXKEYx - Test XKEY,
    pub rsvd10: u32,
    pub /: *mut *mut u32 oscctctl; / RTSTOSCCTCTLx - Test osc. counter control,
    pub rsvd11: u32,
    pub /: *mut *mut u32 oscct; / RTSTOSCCTx - Test oscillator counter,
    pub rsvd12: u32,
    pub /: *mut *mut u32 oscctstat; / RTSTODCCTSTATx - Test osc counter status,
    pub rsvd13: [u32; 2],
    pub /: *mut *mut u32 ofifo[4]; / RTSTOFIFOx - Test output FIFO,
    pub rsvd14: [u32; 15],
}

// RNG4 TRNG test registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rng4tst {

    pub /: *mut *mut u32 rtmctl; / misc. control register,
    pub /: *mut *mut u32 rtscmisc; / statistical check misc. register,
    pub /: *mut *mut u32 rtpkrrng; / poker range register,
    pub /: *mut *mut u32 rtpkrmax; / PRGM=1: poker max. limit register,
    pub /: *mut *mut u32 rtpkrsq; / PRGM=0: poker square calc. result register,
}

pub const RTSDCTL_ENT_DLY_SHIFT: c_int = 16;

pub const RTSDCTL_ENT_DLY_MIN: c_int = 3200;
pub const RTSDCTL_ENT_DLY_MAX: c_int = 12800;
pub const RTSDCTL_SAMP_SIZE_MASK: c_uint = 0xffff;
pub const RTSDCTL_SAMP_SIZE_VAL: c_int = 512;

pub const RDSTA_SKVT: c_uint = 0x80000000;
pub const RDSTA_SKVN: c_uint = 0x40000000;

pub const RDSTA_IF0: c_uint = 0x00000001;
pub const RDSTA_IF1: c_uint = 0x00000002;

//
// caam_ctrl - basic core configuration
// starts base + 0x0000 padded out to 0x1000
//
pub const KEK_KEY_SIZE: c_int = 8;
pub const TKEK_KEY_SIZE: c_int = 8;
pub const TDSK_KEY_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_ctrl {
// Basic Configuration Section				000-01f
// Read/Writable
    pub rsvd1: u32,
    pub /: *mut *mut u32 mcr; / MCFG Master Config Register,
    pub rsvd2: u32,
    pub /: *mut *mut u32 scfgr; / SCFGR, Security Config Register,
// Bus Access Configuration Section			010-11f
// Read/Writable
    pub /: *mut *mut masterid jr_mid[4]; / JRxLIODNR - JobR LIODN setup,
    pub rsvd3: [u32; 11],
    pub /: *mut *mut u32 jrstart; / JRSTART - Job Ring Start Register,
    pub /: *mut *mut masterid rtic_mid[4]; / RTICxLIODNR - RTIC LIODN setup,
    pub rsvd4: [u32; 5],
    pub /: *mut *mut u32 deco_rsr; / DECORSR - Deco Request Source,
    pub rsvd11: u32,
    pub /: *mut *mut u32 deco_rq; / DECORR - DECO Request,
    pub /: *mut *mut masterid deco_mid[16]; / DECOxLIODNR - 1 per DECO,
// DECO Availability/Reset Section			120-3ff
    pub /: *mut *mut u32 deco_avail; / DAR - DECO availability,
    pub /: *mut *mut u32 deco_reset; / DRR - DECO reset,
    pub rsvd6: [u32; 182],
// Key Encryption/Decryption Configuration              400-5ff
// Read/Writable only while in Non-secure mode
    pub /: *mut *mut u32 kek[KEK_KEY_SIZE]; / JDKEKR - Key Encryption Key,
    pub /: *mut *mut u32 tkek[TKEK_KEY_SIZE]; / TDKEKR - Trusted Desc KEK,
    pub /: *mut *mut u32 tdsk[TDSK_KEY_SIZE]; / TDSKR - Trusted Desc Signing Key,
    pub rsvd7: [u32; 32],
    pub /: *mut *mut u64 sknonce; / SKNR - Secure Key Nonce,
    pub rsvd8: [u32; 70],
// RNG Test/Verification/Debug Access                   600-7ff
// (Useful in Test/Debug modes only...)
    pub rtst: [rngtst; 2],
    pub r4tst: [rng4tst; 2],
}

// Version registers - introduced with era 10		e80-eff
// Performance Monitor                                  f00-fff
//
// Controller master config register defs
//
pub const MCFGR_SWRESET: c_uint = 0x80000000 /* software reset */;
pub const MCFGR_WDENABLE: c_uint = 0x40000000 /* DECO watchdog enable */;
pub const MCFGR_WDFAIL: c_uint = 0x20000000 /* DECO watchdog force-fail */;
pub const MCFGR_DMA_RESET: c_uint = 0x10000000;
pub const MCFGR_LONG_PTR: c_uint = 0x00010000 /* Use >32-bit desc addressing */;
pub const SCFGR_RDBENABLE: c_uint = 0x00000400;
pub const SCFGR_VIRT_EN: c_uint = 0x00008000;
pub const DECORR_RQD0ENABLE: c_uint = 0x00000001 /* Enable DECO0 for direct access */;
pub const DECORSR_JR0: c_uint = 0x00000001 /* JR to supply TZ, SDID, ICID */;
pub const DECORSR_VALID: c_uint = 0x80000000;
pub const DECORR_DEN0: c_uint = 0x00010000 /* DECO0 available for access*/;
// AXI read cache control
pub const MCFGR_ARCACHE_SHIFT: c_int = 12;

// AXI write cache control
pub const MCFGR_AWCACHE_SHIFT: c_int = 8;

// AXI pipeline depth
pub const MCFGR_AXIPIPE_SHIFT: c_int = 4;

pub const MCFGR_AXIPRI: c_uint = 0x00000008 /* Assert AXI priority sideband */;
pub const MCFGR_LARGE_BURST: c_uint = 0x00000004 /* 128/256-byte burst size */;
pub const MCFGR_BURST_64: c_uint = 0x00000001 /* 64-byte burst size */;
// JRSTART register offsets
pub const JRSTART_JR0_START: c_uint = 0x00000001 /* Start Job ring 0 */;
pub const JRSTART_JR1_START: c_uint = 0x00000002 /* Start Job ring 1 */;
pub const JRSTART_JR2_START: c_uint = 0x00000004 /* Start Job ring 2 */;
pub const JRSTART_JR3_START: c_uint = 0x00000008 /* Start Job ring 3 */;
//
// caam_job_ring - direct job ring setup
// 1-4 possible per instantiation, base + 1000/2000/3000/4000
// Padded out to 0x1000
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_job_ring {
// Input ring
    pub /: *mut *mut u64 inpring_base; / IRBAx - Input desc ring baseaddr,
    pub rsvd1: u32,
    pub /: *mut *mut u32 inpring_size; / IRSx - Input ring size,
    pub rsvd2: u32,
    pub /: *mut *mut u32 inpring_avail; / IRSAx - Input ring room remaining,
    pub rsvd3: u32,
    pub /: *mut *mut u32 inpring_jobadd; / IRJAx - Input ring jobs added,
// Output Ring
    pub /: *mut *mut u64 outring_base; / ORBAx - Output status ring base addr,
    pub rsvd4: u32,
    pub /: *mut *mut u32 outring_size; / ORSx - Output ring size,
    pub rsvd5: u32,
    pub /: *mut *mut u32 outring_rmvd; / ORJRx - Output ring jobs removed,
    pub rsvd6: u32,
    pub /: *mut *mut u32 outring_used; / ORSFx - Output ring slots full,
// Status/Configuration
    pub rsvd7: u32,
    pub /: *mut *mut u32 jroutstatus; / JRSTAx - JobR output status,
    pub rsvd8: u32,
    pub /: *mut *mut u32 jrintstatus; / JRINTx - JobR interrupt status,
    pub /: *mut *mut u32 rconfig_hi; / JRxCFG - Ring configuration,
    pub rconfig_lo: u32,
// Indices. CAAM maintains as "heads" of each queue
    pub rsvd9: u32,
    pub /: *mut *mut u32 inp_rdidx; / IRRIx - Input ring read index,
    pub rsvd10: u32,
    pub /: *mut *mut u32 out_wtidx; / ORWIx - Output ring write index,
// Command/control
    pub rsvd11: u32,
    pub /: *mut *mut u32 jrcommand; / JRCRx - JobR command,
    pub rsvd12: [u32; 900],
// Version registers - introduced with era 10           e80-eff
    pub vreg: version_regs,
// Performance Monitor                                  f00-fff
    pub perfmon: caam_perfmon,
}

pub const JR_RINGSIZE_MASK: c_uint = 0x03ff;
//
// jrstatus - Job Ring Output Status
// All values in lo word
// Also note, same values written out as status through QI
// in the command/status field of a frame descriptor
//
pub const JRSTA_SSRC_SHIFT: c_int = 28;
pub const JRSTA_SSRC_MASK: c_uint = 0xf0000000;
pub const JRSTA_SSRC_NONE: c_uint = 0x00000000;
pub const JRSTA_SSRC_CCB_ERROR: c_uint = 0x20000000;
pub const JRSTA_SSRC_JUMP_HALT_USER: c_uint = 0x30000000;
pub const JRSTA_SSRC_DECO: c_uint = 0x40000000;
pub const JRSTA_SSRC_QI: c_uint = 0x50000000;
pub const JRSTA_SSRC_JRERROR: c_uint = 0x60000000;
pub const JRSTA_SSRC_JUMP_HALT_CC: c_uint = 0x70000000;
pub const JRSTA_DECOERR_JUMP: c_uint = 0x08000000;
pub const JRSTA_DECOERR_INDEX_SHIFT: c_int = 8;
pub const JRSTA_DECOERR_INDEX_MASK: c_uint = 0xff00;
pub const JRSTA_DECOERR_ERROR_MASK: c_uint = 0x00ff;
pub const JRSTA_DECOERR_NONE: c_uint = 0x00;
pub const JRSTA_DECOERR_LINKLEN: c_uint = 0x01;
pub const JRSTA_DECOERR_LINKPTR: c_uint = 0x02;
pub const JRSTA_DECOERR_JRCTRL: c_uint = 0x03;
pub const JRSTA_DECOERR_DESCCMD: c_uint = 0x04;
pub const JRSTA_DECOERR_ORDER: c_uint = 0x05;
pub const JRSTA_DECOERR_KEYCMD: c_uint = 0x06;
pub const JRSTA_DECOERR_LOADCMD: c_uint = 0x07;
pub const JRSTA_DECOERR_STORECMD: c_uint = 0x08;
pub const JRSTA_DECOERR_OPCMD: c_uint = 0x09;
pub const JRSTA_DECOERR_FIFOLDCMD: c_uint = 0x0a;
pub const JRSTA_DECOERR_FIFOSTCMD: c_uint = 0x0b;
pub const JRSTA_DECOERR_MOVECMD: c_uint = 0x0c;
pub const JRSTA_DECOERR_JUMPCMD: c_uint = 0x0d;
pub const JRSTA_DECOERR_MATHCMD: c_uint = 0x0e;
pub const JRSTA_DECOERR_SHASHCMD: c_uint = 0x0f;
pub const JRSTA_DECOERR_SEQCMD: c_uint = 0x10;
pub const JRSTA_DECOERR_DECOINTERNAL: c_uint = 0x11;
pub const JRSTA_DECOERR_SHDESCHDR: c_uint = 0x12;
pub const JRSTA_DECOERR_HDRLEN: c_uint = 0x13;
pub const JRSTA_DECOERR_BURSTER: c_uint = 0x14;
pub const JRSTA_DECOERR_DESCSIGNATURE: c_uint = 0x15;
pub const JRSTA_DECOERR_DMA: c_uint = 0x16;
pub const JRSTA_DECOERR_BURSTFIFO: c_uint = 0x17;
pub const JRSTA_DECOERR_JRRESET: c_uint = 0x1a;
pub const JRSTA_DECOERR_JOBFAIL: c_uint = 0x1b;
pub const JRSTA_DECOERR_DNRERR: c_uint = 0x80;
pub const JRSTA_DECOERR_UNDEFPCL: c_uint = 0x81;
pub const JRSTA_DECOERR_PDBERR: c_uint = 0x82;
pub const JRSTA_DECOERR_ANRPLY_LATE: c_uint = 0x83;
pub const JRSTA_DECOERR_ANRPLY_REPLAY: c_uint = 0x84;
pub const JRSTA_DECOERR_SEQOVF: c_uint = 0x85;
pub const JRSTA_DECOERR_INVSIGN: c_uint = 0x86;
pub const JRSTA_DECOERR_DSASIGN: c_uint = 0x87;
pub const JRSTA_QIERR_ERROR_MASK: c_uint = 0x00ff;
pub const JRSTA_CCBERR_JUMP: c_uint = 0x08000000;
pub const JRSTA_CCBERR_INDEX_MASK: c_uint = 0xff00;
pub const JRSTA_CCBERR_INDEX_SHIFT: c_int = 8;
pub const JRSTA_CCBERR_CHAID_MASK: c_uint = 0x00f0;
pub const JRSTA_CCBERR_CHAID_SHIFT: c_int = 4;
pub const JRSTA_CCBERR_ERRID_MASK: c_uint = 0x000f;

pub const JRSTA_CCBERR_ERRID_NONE: c_uint = 0x00;
pub const JRSTA_CCBERR_ERRID_MODE: c_uint = 0x01;
pub const JRSTA_CCBERR_ERRID_DATASIZ: c_uint = 0x02;
pub const JRSTA_CCBERR_ERRID_KEYSIZ: c_uint = 0x03;
pub const JRSTA_CCBERR_ERRID_PKAMEMSZ: c_uint = 0x04;
pub const JRSTA_CCBERR_ERRID_PKBMEMSZ: c_uint = 0x05;
pub const JRSTA_CCBERR_ERRID_SEQUENCE: c_uint = 0x06;
pub const JRSTA_CCBERR_ERRID_PKDIVZRO: c_uint = 0x07;
pub const JRSTA_CCBERR_ERRID_PKMODEVN: c_uint = 0x08;
pub const JRSTA_CCBERR_ERRID_KEYPARIT: c_uint = 0x09;
pub const JRSTA_CCBERR_ERRID_ICVCHK: c_uint = 0x0a;
pub const JRSTA_CCBERR_ERRID_HARDWARE: c_uint = 0x0b;
pub const JRSTA_CCBERR_ERRID_CCMAAD: c_uint = 0x0c;
pub const JRSTA_CCBERR_ERRID_INVCHA: c_uint = 0x0f;
pub const JRINT_ERR_INDEX_MASK: c_uint = 0x3fff0000;
pub const JRINT_ERR_INDEX_SHIFT: c_int = 16;
pub const JRINT_ERR_TYPE_MASK: c_uint = 0xf00;
pub const JRINT_ERR_TYPE_SHIFT: c_int = 8;
pub const JRINT_ERR_HALT_MASK: c_uint = 0xc;
pub const JRINT_ERR_HALT_SHIFT: c_int = 2;
pub const JRINT_ERR_HALT_INPROGRESS: c_uint = 0x4;
pub const JRINT_ERR_HALT_COMPLETE: c_uint = 0x8;
pub const JRINT_JR_ERROR: c_uint = 0x02;
pub const JRINT_JR_INT: c_uint = 0x01;
pub const JRINT_ERR_TYPE_WRITE: c_int = 1;
pub const JRINT_ERR_TYPE_BAD_INPADDR: c_int = 3;
pub const JRINT_ERR_TYPE_BAD_OUTADDR: c_int = 4;
pub const JRINT_ERR_TYPE_INV_INPWRT: c_int = 5;
pub const JRINT_ERR_TYPE_INV_OUTWRT: c_int = 6;
pub const JRINT_ERR_TYPE_RESET: c_int = 7;
pub const JRINT_ERR_TYPE_REMOVE_OFL: c_int = 8;
pub const JRINT_ERR_TYPE_ADD_OFL: c_int = 9;
pub const JRCFG_SOE: c_uint = 0x04;
pub const JRCFG_ICEN: c_uint = 0x02;
pub const JRCFG_IMSK: c_uint = 0x01;
pub const JRCFG_ICDCT_SHIFT: c_int = 8;
pub const JRCFG_ICTT_SHIFT: c_int = 16;
pub const JRCR_RESET: c_uint = 0x01;
//
// caam_assurance - Assurance Controller View
// base + 0x6000 padded out to 0x1000
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtic_element {
    pub address: u64,
    pub rsvd: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtic_block {
    pub element: [rtic_element; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtic_memhash {
    pub memhash_be: [u32; 32],
    pub memhash_le: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_assurance {
// Status/Command/Watchdog
    pub rsvd1: u32,
    pub /: *mut *mut u32 status; / RSTA - Status,
    pub rsvd2: u32,
    pub /: *mut *mut u32 cmd; / RCMD - Command,
    pub rsvd3: u32,
    pub /: *mut *mut u32 ctrl; / RCTL - Control,
    pub rsvd4: u32,
    pub /: *mut *mut u32 throttle; / RTHR - Throttle,
    pub rsvd5: [u32; 2],
    pub /: *mut *mut u64 watchdog; / RWDOG - Watchdog Timer,
    pub rsvd6: u32,
    pub /: *mut *mut u32 rend; / REND - Endian corrections,
    pub rsvd7: [u32; 50],
// Block access/configuration @ 100/110/120/130
    pub /: *mut *mut rtic_block memblk[4]; / Memory Blocks A-D,
    pub rsvd8: [u32; 32],
// Block hashes @ 200/300/400/500
    pub /: *mut *mut rtic_memhash hash[4]; / Block hash values A-D,
    pub rsvd_3: [u32; 640],
}

//
// caam_queue_if - QI configuration and control
// starts base + 0x7000, padded out to 0x1000 long
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_queue_if {
    pub /: *mut *mut u32 qi_control_hi; / QICTL - QI Control,
    pub qi_control_lo: u32,
    pub rsvd1: u32,
    pub /: *mut *mut u32 qi_status; / QISTA - QI Status,
    pub /: *mut *mut u32 qi_deq_cfg_hi; / QIDQC - QI Dequeue Configuration,
    pub qi_deq_cfg_lo: u32,
    pub /: *mut *mut u32 qi_enq_cfg_hi; / QISEQC - QI Enqueue Command,
    pub qi_enq_cfg_lo: u32,
    pub rsvd2: [u32; 1016],
}

// QI control bits - low word
pub const QICTL_DQEN: c_uint = 0x01              /* Enable frame pop          */;
pub const QICTL_STOP: c_uint = 0x02              /* Stop dequeue/enqueue      */;
pub const QICTL_SOE: c_uint = 0x04              /* Stop on error             */;
// QI control bits - high word
pub const QICTL_MBSI: c_uint = 0x01;
pub const QICTL_MHWSI: c_uint = 0x02;
pub const QICTL_MWSI: c_uint = 0x04;
pub const QICTL_MDWSI: c_uint = 0x08;
pub const QICTL_CBSI: c_uint = 0x10		/* CtrlDataByteSwapInput     */;
pub const QICTL_CHWSI: c_uint = 0x20		/* CtrlDataHalfSwapInput     */;
pub const QICTL_CWSI: c_uint = 0x40		/* CtrlDataWordSwapInput     */;
pub const QICTL_CDWSI: c_uint = 0x80		/* CtrlDataDWordSwapInput    */;
pub const QICTL_MBSO: c_uint = 0x0100;
pub const QICTL_MHWSO: c_uint = 0x0200;
pub const QICTL_MWSO: c_uint = 0x0400;
pub const QICTL_MDWSO: c_uint = 0x0800;
pub const QICTL_CBSO: c_uint = 0x1000		/* CtrlDataByteSwapOutput    */;
pub const QICTL_CHWSO: c_uint = 0x2000		/* CtrlDataHalfSwapOutput    */;
pub const QICTL_CWSO: c_uint = 0x4000		/* CtrlDataWordSwapOutput    */;
pub const QICTL_CDWSO: c_uint = 0x8000		/* CtrlDataDWordSwapOutput   */;
pub const QICTL_DMBS: c_uint = 0x010000;
pub const QICTL_EPO: c_uint = 0x020000;
// QI status bits
pub const QISTA_PHRDERR: c_uint = 0x01              /* PreHeader Read Error      */;
pub const QISTA_CFRDERR: c_uint = 0x02              /* Compound Frame Read Error */;
pub const QISTA_OFWRERR: c_uint = 0x04              /* Output Frame Read Error   */;
pub const QISTA_BPDERR: c_uint = 0x08              /* Buffer Pool Depleted      */;
pub const QISTA_BTSERR: c_uint = 0x10              /* Buffer Undersize          */;
pub const QISTA_CFWRERR: c_uint = 0x20              /* Compound Frame Write Err  */;
pub const QISTA_STOPD: c_uint = 0x80000000        /* QI Stopped (see QICTL)    */;
// deco_sg_table - DECO view of scatter/gather table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deco_sg_table {
    pub /: *mut *mut u64 addr; / Segment Address,
    pub /: *mut *mut u32 elen; / E, F bits + 30-bit length,
    pub /: *mut *mut u32 bpid_offset; / Buffer Pool ID + 16-bit length,
}

//
// caam_deco - descriptor controller - CHA cluster block
//
// Only accessible when direct DECO access is turned on
// (done in DECORR, via MID programmed in DECOxMID
//
// 5 typical, base + 0x8000/9000/a000/b000
// Padded out to 0x1000 long
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_deco {
    pub rsvd1: u32,
    pub /: *mut *mut u32 cls1_mode; / CxC1MR - Class 1 Mode,
    pub rsvd2: u32,
    pub /: *mut *mut u32 cls1_keysize; / CxC1KSR - Class 1 Key Size,
    pub /: *mut *mut u32 cls1_datasize_hi; / CxC1DSR - Class 1 Data Size,
    pub cls1_datasize_lo: u32,
    pub rsvd3: u32,
    pub /: *mut *mut u32 cls1_icvsize; / CxC1ICVSR - Class 1 ICV size,
    pub rsvd4: [u32; 5],
    pub /: *mut *mut u32 cha_ctrl; / CCTLR - CHA control,
    pub rsvd5: u32,
    pub /: *mut *mut u32 irq_crtl; / CxCIRQ - CCB interrupt done/error/clear,
    pub rsvd6: u32,
    pub /: *mut *mut u32 clr_written; / CxCWR - Clear-Written,
    pub /: *mut *mut u32 ccb_status_hi; / CxCSTA - CCB Status/Error,
    pub ccb_status_lo: u32,
    pub rsvd7: [u32; 3],
    pub /: *mut *mut u32 aad_size; / CxAADSZR - Current AAD Size,
    pub rsvd8: u32,
    pub /: *mut *mut u32 cls1_iv_size; / CxC1IVSZR - Current Class 1 IV Size,
    pub rsvd9: [u32; 7],
    pub /: *mut *mut u32 pkha_a_size; / PKASZRx - Size of PKHA A,
    pub rsvd10: u32,
    pub /: *mut *mut u32 pkha_b_size; / PKBSZRx - Size of PKHA B,
    pub rsvd11: u32,
    pub /: *mut *mut u32 pkha_n_size; / PKNSZRx - Size of PKHA N,
    pub rsvd12: u32,
    pub /: *mut *mut u32 pkha_e_size; / PKESZRx - Size of PKHA E,
    pub rsvd13: [u32; 24],
    pub /: *mut *mut u32 cls1_ctx[16]; / CxC1CTXR - Class 1 Context @100,
    pub rsvd14: [u32; 48],
    pub /: *mut *mut u32 cls1_key[8]; / CxC1KEYR - Class 1 Key @200,
    pub rsvd15: [u32; 121],
    pub /: *mut *mut u32 cls2_mode; / CxC2MR - Class 2 Mode,
    pub rsvd16: u32,
    pub /: *mut *mut u32 cls2_keysize; / CxX2KSR - Class 2 Key Size,
    pub /: *mut *mut u32 cls2_datasize_hi; / CxC2DSR - Class 2 Data Size,
    pub cls2_datasize_lo: u32,
    pub rsvd17: u32,
    pub /: *mut *mut u32 cls2_icvsize; / CxC2ICVSZR - Class 2 ICV Size,
    pub rsvd18: [u32; 56],
    pub /: *mut *mut u32 cls2_ctx[18]; / CxC2CTXR - Class 2 Context @500,
    pub rsvd19: [u32; 46],
    pub /: *mut *mut u32 cls2_key[32]; / CxC2KEYR - Class2 Key @600,
    pub rsvd20: [u32; 84],
    pub /: *mut *mut u32 inp_infofifo_hi; / CxIFIFO - Input Info FIFO @7d0,
    pub inp_infofifo_lo: u32,
    pub rsvd21: [u32; 2],
    pub /: *mut *mut u64 inp_datafifo; / CxDFIFO - Input Data FIFO,
    pub rsvd22: [u32; 2],
    pub /: *mut *mut u64 out_datafifo; / CxOFIFO - Output Data FIFO,
    pub rsvd23: [u32; 2],
    pub /: *mut *mut u32 jr_ctl_hi; / CxJRR - JobR Control Register @800,
    pub jr_ctl_lo: u32,
    pub /: *mut *mut u64 jr_descaddr; / CxDADR - JobR Descriptor Address,
pub const DECO_OP_STATUS_HI_ERR_MASK: c_uint = 0xF00000FF;
    pub /: *mut *mut u32 op_status_hi; / DxOPSTA - DECO Operation Status,
    pub op_status_lo: u32,
    pub rsvd24: [u32; 2],
    pub /: *mut *mut u32 liodn; / DxLSR - DECO LIODN Status - non-seq,
    pub /: *mut *mut u32 td_liodn; / DxLSR - DECO LIODN Status - trustdesc,
    pub rsvd26: [u32; 6],
    pub /: *mut *mut u64 math[4]; / DxMTH - Math register,
    pub rsvd27: [u32; 8],
    pub /: *mut *mut deco_sg_table gthr_tbl[4]; / DxGTR - Gather Tables,
    pub rsvd28: [u32; 16],
    pub /: *mut *mut deco_sg_table sctr_tbl[4]; / DxSTR - Scatter Tables,
    pub rsvd29: [u32; 48],
    pub /: *mut *mut u32 descbuf[64]; / DxDESB - Descriptor buffer,
    pub rscvd30: [u32; 193],
pub const DESC_DBG_DECO_STAT_VALID: c_uint = 0x80000000;
pub const DESC_DBG_DECO_STAT_MASK: c_uint = 0x00F00000;
pub const DESC_DBG_DECO_STAT_SHIFT: c_int = 20;
    pub /: *mut *mut u32 desc_dbg; / DxDDR - DECO Debug Register,
    pub rsvd31: [u32; 13],
pub const DESC_DER_DECO_STAT_MASK: c_uint = 0x000F0000;
pub const DESC_DER_DECO_STAT_SHIFT: c_int = 16;
    pub /: *mut *mut u32 dbg_exec; / DxDER - DECO Debug Exec Register,
    pub rsvd32: [u32; 112],
}

pub const DECO_STAT_HOST_ERR: c_uint = 0xD;
pub const DECO_JQCR_WHL: c_uint = 0x20000000;
pub const DECO_JQCR_FOUR: c_uint = 0x10000000;
pub const JR_BLOCK_NUMBER: c_int = 1;
pub const ASSURE_BLOCK_NUMBER: c_int = 6;
pub const QI_BLOCK_NUMBER: c_int = 7;
pub const DECO_BLOCK_NUMBER: c_int = 8;
pub const PG_SIZE_4K: c_uint = 0x1000;
pub const PG_SIZE_64K: c_uint = 0x10000;
