//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/nx-gzip/include/nxu.h
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
// Hardware interface of the NX-GZIP compression accelerator
//
// Copyright (C) IBM Corporation, 2020
//
// Author: Bulent Abali <abali@us.ibm.com>
//

// deflate
pub const LLSZ: c_int = 286;
pub const DSZ: c_int = 30;
// nx
pub const DHTSZ: c_int = 18;
pub const DHT_MAXSZ: c_int = 288;
pub const MAX_DDE_COUNT: c_int = 256;
// util

// Macro flag: #define NXPRT(X)

// Macro flag: #define NX_CLK(X)

pub const NX_MAX_FAULTS: c_int = 500;
//
// Definitions of acronyms used here. See
// P9 NX Gzip Accelerator User's Manual for details:
// https://github.com/libnxz/power-gzip/blob/develop/doc/power_nx_gzip_um.pdf
//
// adler/crc: 32 bit checksums appended to stream tail
// ce:       completion extension
// cpb:      coprocessor parameter block (metadata)
// crb:      coprocessor request block (command)
// csb:      coprocessor status block (status)
// dht:      dynamic huffman table
// dde:      data descriptor element (address, length)
// ddl:      list of ddes
// dh/fh:    dynamic and fixed huffman types
// fc:       coprocessor function code
// histlen:  history/dictionary length
// history:  sliding window of up to 32KB of data
// lzcount:  Deflate LZ symbol counts
// rembytecnt: remaining byte count
// sfbt:     source final block type; last block's type during decomp
// spbc:     source processed byte count
// subc:     source unprocessed bit count
// tebc:     target ending bit count; valid bits in the last byte
// tpbc:     target processed byte count
// vas:      virtual accelerator switch; the user mode interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nx_qw_t {
    pub word: [u32; 4],
    pub dword: [u64; 2],
    pub __aligned(16): },
//
// Note: NX registers with fewer than 32 bits are declared by
// convention as uint32_t variables in unions. If *_offset and *_mask
// are defined for a variable, then use get_ put_ macros to
// conveniently access the register fields for endian conversions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_dde_t {
// Data Descriptor Element, Section 6.4
    pub dde_count: u32,
// When dde_count == 0 ddead is a pointer to a data buffer;
// ddebc is the buffer length bytes.
// When dde_count > 0 dde is an indirect dde; ddead is a
// pointer to a contiguous list of direct ddes; ddebc is the
// total length of all data pointed to by the list of direct
// ddes. Note that only one level of indirection is permitted.
// See Section 6.4 of the user manual for additional details.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_csb_t {
// Coprocessor Status Block, Section 6.6
    pub csb_v: u32,
// Valid bit. v must be set to 0 by the program
// before submitting the coprocessor command.
// Software can poll for the v bit
//
    pub csb_f: u32,
// 16B CSB size. Written to 0 by DMA when it writes the CPB
    pub csb_cs: u32,
// cs completion sequence; unused
    pub csb_cc: u32,
// cc completion code; cc != 0 exception occurred
    pub csb_ce: u32,
// ce completion extension
}

// target processed byte count TPBC
// Section 6.12.1 CSB NonZero error summary.  FSA Failing storage
// address.  Address where error occurred. When available, written
// to A field of CSB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_ccb_t {
// Coprocessor Completion Block, Section 6.7
    pub reserved: [u32; 3],
// When crb.c==0 (no ccb defined) it is reserved;
// When crb.c==1 (ccb defined) it is cm
//
    pub ccb_cm: u32,
// Signal interrupt of crb.c==1 and cm==1
    pub word: u32,
// generic access to the 32bit word
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_stamped_crb_t {
//
// CRB operand of the paste coprocessor instruction is stamped
// in quadword 4 with the information shown here as its written
// in to the receive FIFO of the coprocessor
//
    pub vas_buf_num: u32,
// Verification only vas buffer number which correlates to
// the low order bits of the atag in the paste command
//
    pub send_wc_id: u32,
// Pointer to Send Window Context that provides for NX address
// translation information, such as MSR and LPCR bits, job
// completion interrupt RA, PSWID, and job utilization counter.
//
}

// Pointer to Receive Window Context. NX uses this to return
// credits to a Receive FIFO as entries are dequeued.
//
// Invalid bit. If this bit is 1 the CRB is discarded by
// NX upon fetching from the receive FIFO. If this bit is 0
// the CRB is processed normally. The bit is stamped to 0
// by VAS and may be written to 1 by hypervisor while
// the CRB is in the receive FIFO (in memory).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_stamped_fault_crb_t {
//
// A CRB that has a translation fault is stamped by NX in quadword 4
// and pasted to the Fault Send Window in VAS.
//
    pub fsa: u64,
    pub nxsf_t: u32,
    pub nxsf_fs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union stamped_crb_t {
    pub vas: vas_stamped_crb_t,
    pub nx: nx_stamped_fault_crb_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_cpb_t {
//
// Coprocessor Parameter Block In/Out are used to pass metadata
// to/from accelerator.  Tables 6.5 and 6.6 of the user manual.
//
// CPBInput
    pub qw0: nx_qw_t,
    pub /: *mut *mut uint32_t in_adler; / bits 0:31,
    pub /: *mut *mut uint32_t in_crc; / bits 32:63,
    pub /: *mut *mut uint32_t in_histlen; / bits 64:75,
    pub /: *mut *mut uint32_t in_subc; / bits 93:95,
}

// bits 108:111
// bits 112:127
// bits 116:127
// CPBOutput
// bits 77:79 qw[24]
// bits 80:95 qw[24]
// bits 108:111 qw[24]
// bits 112:127 qw[24]
// bits 116:127 qw[24]
// qw[25] compress no lzcounts or wrap
// qw[25] compress no lzcounts
// 286 LL and 30 D symbol counts
// qw[43] decompress
// qw[104] compress with lzcounts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_crb_t {
    pub /: *mut *mut uint32_t gzip_fc; / bits[24-31],
}

// c==0 no ccb defined
// at==0 address type is ignored;
// all addrs effective assumed.
//
// byte[64:239] shift csb by 128 bytes out of the crb; csb was
// in crb earlier; JReilly says csb written with partial inject
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gzip_crb_cpb_t {
    pub crb: nx_gzip_crb_t,
    pub cpb: nx_gzip_cpb_t,
    pub __aligned(2048): },
//
// NX hardware convention has the msb bit on the left numbered 0.
// The defines below has *_offset defined as the right most bit
// position of a field.  x of size_mask(x) is the field width in bits.
//

//
// Offsets and Widths within the containing 32 bits of the various NX
// gzip hardware registers.  Use the getnn/putnn macros to access
// these regs
//

pub const dde_count_offset: c_int = 23;
// CSB

pub const csb_v_offset: c_int = 0;

pub const csb_f_offset: c_int = 6;

pub const csb_cs_offset: c_int = 15;

pub const csb_cc_offset: c_int = 23;

pub const csb_ce_offset: c_int = 31;
// CCB

pub const ccb_cm_offset: c_int = 31;
// VAS stamped CRB fields

pub const vas_buf_num_offset: c_int = 5;

pub const send_wc_id_offset: c_int = 31;

pub const recv_wc_id_offset: c_int = 31;

pub const vas_invalid_offset: c_int = 31;
// NX stamped fault CRB fields

pub const nxsf_t_offset: c_int = 23;

pub const nxsf_fs_offset: c_int = 31;
// CPB input

pub const in_histlen_offset: c_int = 11;

pub const in_dhtlen_offset: c_int = 31;

pub const in_subc_offset: c_int = 31;

pub const in_sfbt_offset: c_int = 15;

pub const in_rembytecnt_offset: c_int = 31;
// CPB output

pub const out_tebc_offset: c_int = 15;

pub const out_subc_offset: c_int = 31;

pub const out_sfbt_offset: c_int = 15;

pub const out_rembytecnt_offset: c_int = 31;

pub const out_dhtlen_offset: c_int = 31;
// CRB

pub const gzip_fc_offset: c_int = 31;

pub const crb_c_offset: c_int = 28;

pub const crb_at_offset: c_int = 30;

//
// Access macros for the registers.  Do not access registers directly
// because of the endian conversion.  P9 processor may run either as
// Little or Big endian. However the NX coprocessor regs are always
// big endian.
// Use the 32 and 64b macros to access respective
// register sizes.
// Use nn forms for the register fields shorter than 32 bits.
//

// get 32bits less the REG field

// get 32bits less the REG field

    pub \: (ST)->ddead = 0;,

//
// Completion extension ce(0) ce(1) ce(2).  Bits ce(3-7)
// unused.  Section 6.6 Figure 6.7.
//

pub const CSB_CE_PARTIAL: c_uint = 0x4;
pub const CSB_CE_TERMINATE: c_uint = 0x2;
pub const CSB_CE_TPBC_VALID: c_uint = 0x1;

// termination, output buffers may be modified, SPBC/TPBC invalid Fig.6-7

// if not terminated then check full or partial completion

// TPBC indicates successfully stored data count

// most error CEs have CE(0)=0 and CE(1)=1

// some CC=3 are partially completed, Table 6-8

// Compression: when TPBC>SPBC then CC=64 Table 6-8; target didn't
// compress smaller than source.
//
// Decompress SFBT combinations Tables 5-3, 6-4, 6-6
pub const SFBT_BFINAL: c_uint = 0x1;
pub const SFBT_LIT: c_uint = 0x4;
pub const SFBT_FHT: c_uint = 0x5;
pub const SFBT_DHT: c_uint = 0x6;
pub const SFBT_HDR: c_uint = 0x7;
//
// NX gzip function codes. Table 6.2.
// Bits 0:4 are the FC. Bit 5 is used by the DMA controller to
// select one of the two Byte Count Limits.
//
pub const GZIP_FC_LIMIT_MASK: c_uint = 0x01;
pub const GZIP_FC_COMPRESS_FHT: c_uint = 0x00;
pub const GZIP_FC_COMPRESS_DHT: c_uint = 0x02;
pub const GZIP_FC_COMPRESS_FHT_COUNT: c_uint = 0x04;
pub const GZIP_FC_COMPRESS_DHT_COUNT: c_uint = 0x06;
pub const GZIP_FC_COMPRESS_RESUME_FHT: c_uint = 0x08;
pub const GZIP_FC_COMPRESS_RESUME_DHT: c_uint = 0x0a;
pub const GZIP_FC_COMPRESS_RESUME_FHT_COUNT: c_uint = 0x0c;
pub const GZIP_FC_COMPRESS_RESUME_DHT_COUNT: c_uint = 0x0e;
pub const GZIP_FC_DECOMPRESS: c_uint = 0x10;
pub const GZIP_FC_DECOMPRESS_SINGLE_BLK_N_SUSPEND: c_uint = 0x12;
pub const GZIP_FC_DECOMPRESS_RESUME: c_uint = 0x14;
pub const GZIP_FC_DECOMPRESS_RESUME_SINGLE_BLK_N_SUSPEND: c_uint = 0x16;
pub const GZIP_FC_WRAP: c_uint = 0x1e;

// CSB.CC Error codes
pub const ERR_NX_OK: c_int = 0;
pub const ERR_NX_ALIGNMENT: c_int = 1;
pub const ERR_NX_OPOVERLAP: c_int = 2;
pub const ERR_NX_DATA_LENGTH: c_int = 3;
pub const ERR_NX_TRANSLATION: c_int = 5;
pub const ERR_NX_PROTECTION: c_int = 6;
pub const ERR_NX_EXTERNAL_UE7: c_int = 7;
pub const ERR_NX_INVALID_OP: c_int = 8;
pub const ERR_NX_PRIVILEGE: c_int = 9;
pub const ERR_NX_INTERNAL_UE: c_int = 10;
pub const ERR_NX_EXTERN_UE_WR: c_int = 12;
pub const ERR_NX_TARGET_SPACE: c_int = 13;
pub const ERR_NX_EXCESSIVE_DDE: c_int = 14;
pub const ERR_NX_TRANSL_WR: c_int = 15;
pub const ERR_NX_PROTECT_WR: c_int = 16;
pub const ERR_NX_SUBFUNCTION: c_int = 17;
pub const ERR_NX_FUNC_ABORT: c_int = 18;
pub const ERR_NX_BYTE_MAX: c_int = 19;
pub const ERR_NX_CORRUPT_CRB: c_int = 20;
pub const ERR_NX_INVALID_CRB: c_int = 21;
pub const ERR_NX_INVALID_DDE: c_int = 30;
pub const ERR_NX_SEGMENTED_DDL: c_int = 31;
pub const ERR_NX_DDE_OVERFLOW: c_int = 33;
pub const ERR_NX_TPBC_GT_SPBC: c_int = 64;
pub const ERR_NX_MISSING_CODE: c_int = 66;
pub const ERR_NX_INVALID_DIST: c_int = 67;
pub const ERR_NX_INVALID_DHT: c_int = 68;
pub const ERR_NX_EXTERNAL_UE90: c_int = 90;
pub const ERR_NX_WDOG_TIMER: c_int = 224;
pub const ERR_NX_AT_FAULT: c_int = 250;
pub const ERR_NX_INTR_SERVER: c_int = 252;
pub const ERR_NX_UE253: c_int = 253;
pub const ERR_NX_NO_HW: c_int = 254;
pub const ERR_NX_HUNG_OP: c_int = 255;
pub const ERR_NX_END: c_int = 256;
// initial values for non-resume operations

// prototypes
    pub handle): *mut *mut int nxu_submit_job(struct nx_gzip_crb_cpb_t c, void,
    pub ctx): *mut *mut extern void nxu_sigsegv_handler(int sig, siginfo_t info, void,
    pub wr): *mut *mut extern int nxu_touch_pages(void buf, long buf_len, long page_len, int,
// caller supplies a print buffer 4*sizeof(crb)
    pub prbuf): *mut *mut *mut char nx_crb_str(struct nx_gzip_crb_t crb, char,
    pub prbuf): *mut *mut *mut char nx_cpb_str(struct nx_gzip_cpb_t cpb, char,
    pub prbuf): *mut *mut *mut char nx_prt_hex(void cp, int sz, char,
    pub prbuf): *mut *mut *mut char nx_lzcount_str(struct nx_gzip_cpb_t cpb, char,
    pub e): *mut *mut char nx_strerror(int,

    pub ctx): *mut int nx_sim_init(void,
    pub ctx): *mut int nx_sim_end(void,
    pub ctx): *mut *mut int nxu_run_sim_job(struct nx_gzip_crb_cpb_t c, void,

// Deflate stream manipulation

// append 10 bits 0000001b 00...... ;
// assumes appending starts on a byte boundary; b is the final bit.
//

// 842 Engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_eft_crb_t {
    pub /: *mut *mut uint32_t eft_fc; / bits[29-31],
}

// c==0 no ccb defined
// at==0 address type is ignored;
// all addrs effective assumed.
//
// 842 CRB

pub const EFT_FC_OFFSET: c_int = 31;
pub const EFT_FC_COMPRESS: c_uint = 0x0;
pub const EFT_FC_COMPRESS_WITH_CRC: c_uint = 0x1;
pub const EFT_FC_DECOMPRESS: c_uint = 0x2;
pub const EFT_FC_DECOMPRESS_WITH_CRC: c_uint = 0x3;
pub const EFT_FC_BLK_DATA_MOVE: c_uint = 0x4;

