//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rapidio/devices/tsi721.h
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
// Tsi721 PCIExpress-to-SRIO bridge definitions
//
// Copyright 2011, Integrated Device Technology, Inc.
//
// Debug output filtering masks

pub const DEFAULT_HOPCOUNT: c_uint = 0xff;
pub const DEFAULT_DESTID: c_uint = 0xff;
// PCI device ID
pub const PCI_DEVICE_ID_TSI721: c_uint = 0x80ab;
pub const BAR_0: c_int = 0;
pub const BAR_1: c_int = 1;
pub const BAR_2: c_int = 2;
pub const BAR_4: c_int = 4;
pub const TSI721_PC2SR_BARS: c_int = 2;
pub const TSI721_PC2SR_WINS: c_int = 8;
pub const TSI721_PC2SR_ZONES: c_int = 8;

// Memory space sizes

pub const RIO_TT_CODE_8: c_uint = 0x00000000;
pub const RIO_TT_CODE_16: c_uint = 0x00000001;
pub const TSI721_DMA_MAXCH: c_int = 8;
pub const TSI721_DMA_MINSTSSZ: c_int = 32;
pub const TSI721_DMA_STSBLKSZ: c_int = 8;
pub const TSI721_SRIO_MAXCH: c_int = 8;

// Register definitions
//
// Registers in PCIe configuration space
//
pub const TSI721_PCIECFG_MSIXTBL: c_uint = 0x0a4;
pub const TSI721_MSIXTBL_OFFSET: c_uint = 0x2c000;
pub const TSI721_PCIECFG_MSIXPBA: c_uint = 0x0a8;
pub const TSI721_MSIXPBA_OFFSET: c_uint = 0x2a000;
pub const TSI721_PCIECFG_EPCTL: c_uint = 0x400;
//
// Event Management Registers
//
pub const TSI721_RIO_EM_INT_STAT: c_uint = 0x10910;
pub const TSI721_RIO_EM_INT_STAT_PW_RX: c_uint = 0x00010000;
pub const TSI721_RIO_EM_INT_ENABLE: c_uint = 0x10914;
pub const TSI721_RIO_EM_INT_ENABLE_PW_RX: c_uint = 0x00010000;
pub const TSI721_RIO_EM_DEV_INT_EN: c_uint = 0x10930;
pub const TSI721_RIO_EM_DEV_INT_EN_INT: c_uint = 0x00000001;
//
// Port-Write Block Registers
//
pub const TSI721_RIO_PW_CTL: c_uint = 0x10a04;
pub const TSI721_RIO_PW_CTL_PW_TIMER: c_uint = 0xf0000000;

pub const TSI721_RIO_PW_CTL_PWC_MODE: c_uint = 0x01000000;
pub const TSI721_RIO_PW_CTL_PWC_CONT: c_uint = 0x00000000;
pub const TSI721_RIO_PW_CTL_PWC_REL: c_uint = 0x01000000;
pub const TSI721_RIO_PW_RX_STAT: c_uint = 0x10a10;
pub const TSI721_RIO_PW_RX_STAT_WR_SIZE: c_uint = 0x0000f000;
pub const TSI_RIO_PW_RX_STAT_WDPTR: c_uint = 0x00000100;
pub const TSI721_RIO_PW_RX_STAT_PW_SHORT: c_uint = 0x00000008;
pub const TSI721_RIO_PW_RX_STAT_PW_TRUNC: c_uint = 0x00000004;
pub const TSI721_RIO_PW_RX_STAT_PW_DISC: c_uint = 0x00000002;
pub const TSI721_RIO_PW_RX_STAT_PW_VAL: c_uint = 0x00000001;

//
// Inbound Doorbells
//
pub const TSI721_IDB_ENTRY_SIZE: c_int = 64;

pub const TSI721_IDQ_SUSPEND: c_uint = 0x00000002;
pub const TSI721_IDQ_INIT: c_uint = 0x00000001;

pub const TSI721_IDQ_RUN: c_uint = 0x00200000;

pub const TSI721_IDQ_MASK_MASK: c_uint = 0xffff0000;
pub const TSI721_IDQ_MASK_PATT: c_uint = 0x0000ffff;

pub const TSI721_IDQ_RP_PTR: c_uint = 0x0007ffff;

pub const TSI721_IDQ_WP_PTR: c_uint = 0x0007ffff;

pub const TSI721_IDQ_BASEL_ADDR: c_uint = 0xffffffc0;

pub const TSI721_IDQ_SIZE_MIN: c_int = 512;

pub const TSI721_SR_CHINT_ODBOK: c_uint = 0x00000020;
pub const TSI721_SR_CHINT_IDBQRCV: c_uint = 0x00000010;
pub const TSI721_SR_CHINT_SUSP: c_uint = 0x00000008;
pub const TSI721_SR_CHINT_ODBTO: c_uint = 0x00000004;
pub const TSI721_SR_CHINT_ODBRTRY: c_uint = 0x00000002;
pub const TSI721_SR_CHINT_ODBERR: c_uint = 0x00000001;
pub const TSI721_SR_CHINT_ALL: c_uint = 0x0000003f;
pub const TSI721_IBWIN_NUM: c_int = 8;

pub const TSI721_IBWIN_LB_BA: c_uint = 0xfffff000;
pub const TSI721_IBWIN_LB_WEN: c_uint = 0x00000001;

pub const TSI721_IBWIN_SZ_SIZE: c_uint = 0x00001f00;

pub const TSI721_IBWIN_TLA_ADD: c_uint = 0xfffff000;

pub const TSI721_SR2PC_GEN_INTE: c_uint = 0x29800;
pub const TSI721_SR2PC_PWE: c_uint = 0x29804;
pub const TSI721_SR2PC_GEN_INT: c_uint = 0x29808;
pub const TSI721_DEV_INTE: c_uint = 0x29840;
pub const TSI721_DEV_INT: c_uint = 0x29844;
pub const TSI721_DEV_INTSET: c_uint = 0x29848;
pub const TSI721_DEV_INT_BDMA_CH: c_uint = 0x00002000;
pub const TSI721_DEV_INT_BDMA_NCH: c_uint = 0x00001000;
pub const TSI721_DEV_INT_SMSG_CH: c_uint = 0x00000800;
pub const TSI721_DEV_INT_SMSG_NCH: c_uint = 0x00000400;
pub const TSI721_DEV_INT_SR2PC_CH: c_uint = 0x00000200;
pub const TSI721_DEV_INT_SRIO: c_uint = 0x00000020;
pub const TSI721_DEV_CHAN_INTE: c_uint = 0x2984c;
pub const TSI721_DEV_CHAN_INT: c_uint = 0x29850;
pub const TSI721_INT_SR2PC_CHAN_M: c_uint = 0xff000000;

pub const TSI721_INT_IMSG_CHAN_M: c_uint = 0x00ff0000;

pub const TSI721_INT_OMSG_CHAN_M: c_uint = 0x0000ff00;

pub const TSI721_INT_BDMA_CHAN_M: c_uint = 0x000000ff;

//
// PC2SR block registers
//

pub const TSI721_OBWINLB_BA: c_uint = 0xffff8000;
pub const TSI721_OBWINLB_WEN: c_uint = 0x00000001;

pub const TSI721_OBWINSZ_SIZE: c_uint = 0x00001f00;

pub const TSI721_ZONE_SEL: c_uint = 0x41300;
pub const TSI721_ZONE_SEL_RD_WRB: c_uint = 0x00020000;
pub const TSI721_ZONE_SEL_GO: c_uint = 0x00010000;
pub const TSI721_ZONE_SEL_WIN: c_uint = 0x00000038;
pub const TSI721_ZONE_SEL_ZONE: c_uint = 0x00000007;
pub const TSI721_LUT_DATA0: c_uint = 0x41304;
pub const TSI721_LUT_DATA0_ADD: c_uint = 0xfffff000;
pub const TSI721_LUT_DATA0_RDTYPE: c_uint = 0x00000f00;
pub const TSI721_LUT_DATA0_NREAD: c_uint = 0x00000100;
pub const TSI721_LUT_DATA0_MNTRD: c_uint = 0x00000200;
pub const TSI721_LUT_DATA0_RDCRF: c_uint = 0x00000020;
pub const TSI721_LUT_DATA0_WRCRF: c_uint = 0x00000010;
pub const TSI721_LUT_DATA0_WRTYPE: c_uint = 0x0000000f;
pub const TSI721_LUT_DATA0_NWR: c_uint = 0x00000001;
pub const TSI721_LUT_DATA0_MNTWR: c_uint = 0x00000002;
pub const TSI721_LUT_DATA0_NWR_R: c_uint = 0x00000004;
pub const TSI721_LUT_DATA1: c_uint = 0x41308;
pub const TSI721_LUT_DATA2: c_uint = 0x4130c;
pub const TSI721_LUT_DATA2_HC: c_uint = 0xff000000;
pub const TSI721_LUT_DATA2_ADD65: c_uint = 0x000c0000;
pub const TSI721_LUT_DATA2_TT: c_uint = 0x00030000;
pub const TSI721_LUT_DATA2_DSTID: c_uint = 0x0000ffff;
pub const TSI721_PC2SR_INTE: c_uint = 0x41310;
pub const TSI721_DEVCTL: c_uint = 0x48004;
pub const TSI721_DEVCTL_SRBOOT_CMPL: c_uint = 0x00000004;
pub const TSI721_I2C_INT_ENABLE: c_uint = 0x49120;
//
// Block DMA Engine Registers
// x = 0..7
//

pub const TSI721_DMAC_DWRCNT: c_uint = 0x000;
pub const TSI721_DMAC_DRDCNT: c_uint = 0x004;
pub const TSI721_DMAC_CTL: c_uint = 0x008;
pub const TSI721_DMAC_CTL_SUSP: c_uint = 0x00000002;
pub const TSI721_DMAC_CTL_INIT: c_uint = 0x00000001;
pub const TSI721_DMAC_INT: c_uint = 0x00c;
pub const TSI721_DMAC_INT_STFULL: c_uint = 0x00000010;
pub const TSI721_DMAC_INT_DONE: c_uint = 0x00000008;
pub const TSI721_DMAC_INT_SUSP: c_uint = 0x00000004;
pub const TSI721_DMAC_INT_ERR: c_uint = 0x00000002;
pub const TSI721_DMAC_INT_IOFDONE: c_uint = 0x00000001;
pub const TSI721_DMAC_INT_ALL: c_uint = 0x0000001f;
pub const TSI721_DMAC_INTSET: c_uint = 0x010;
pub const TSI721_DMAC_STS: c_uint = 0x014;
pub const TSI721_DMAC_STS_ABORT: c_uint = 0x00400000;
pub const TSI721_DMAC_STS_RUN: c_uint = 0x00200000;
pub const TSI721_DMAC_STS_CS: c_uint = 0x001f0000;
pub const TSI721_DMAC_INTE: c_uint = 0x018;
pub const TSI721_DMAC_DPTRL: c_uint = 0x024;
pub const TSI721_DMAC_DPTRL_MASK: c_uint = 0xffffffe0;
pub const TSI721_DMAC_DPTRH: c_uint = 0x028;
pub const TSI721_DMAC_DSBL: c_uint = 0x02c;
pub const TSI721_DMAC_DSBL_MASK: c_uint = 0xffffffc0;
pub const TSI721_DMAC_DSBH: c_uint = 0x030;
pub const TSI721_DMAC_DSSZ: c_uint = 0x034;
pub const TSI721_DMAC_DSSZ_SIZE_M: c_uint = 0x0000000f;

pub const TSI721_DMAC_DSRP: c_uint = 0x038;
pub const TSI721_DMAC_DSRP_MASK: c_uint = 0x0007ffff;
pub const TSI721_DMAC_DSWP: c_uint = 0x03c;
pub const TSI721_DMAC_DSWP_MASK: c_uint = 0x0007ffff;
pub const TSI721_BDMA_INTE: c_uint = 0x5f000;
//
// Messaging definitions
//

pub const TSI721_IMSG_MAXCH: c_int = 8;

pub const TSI721_IMSGD_MIN_RING_SIZE: c_int = 32;
pub const TSI721_IMSGD_RING_SIZE: c_int = 512;

pub const TSI721_OMSGD_MIN_RING_SIZE: c_int = 32;
pub const TSI721_OMSGD_RING_SIZE: c_int = 512;
//
// Outbound Messaging Engine Registers
// x = 0..7
//

pub const TSI721_OBDMAC_CTL_MASK: c_uint = 0x00000007;
pub const TSI721_OBDMAC_CTL_RETRY_THR: c_uint = 0x00000004;
pub const TSI721_OBDMAC_CTL_SUSPEND: c_uint = 0x00000002;
pub const TSI721_OBDMAC_CTL_INIT: c_uint = 0x00000001;

pub const TSI721_OBDMAC_INT_MASK: c_uint = 0x0000001F;
pub const TSI721_OBDMAC_INT_ST_FULL: c_uint = 0x00000010;
pub const TSI721_OBDMAC_INT_DONE: c_uint = 0x00000008;
pub const TSI721_OBDMAC_INT_SUSPENDED: c_uint = 0x00000004;
pub const TSI721_OBDMAC_INT_ERROR: c_uint = 0x00000002;
pub const TSI721_OBDMAC_INT_IOF_DONE: c_uint = 0x00000001;

pub const TSI721_OBDMAC_STS_MASK: c_uint = 0x007f0000;
pub const TSI721_OBDMAC_STS_ABORT: c_uint = 0x00400000;
pub const TSI721_OBDMAC_STS_RUN: c_uint = 0x00200000;
pub const TSI721_OBDMAC_STS_CS: c_uint = 0x001f0000;

pub const TSI721_OBDMAC_PWE_MASK: c_uint = 0x00000002;
pub const TSI721_OBDMAC_PWE_ERROR_EN: c_uint = 0x00000002;

pub const TSI721_OBDMAC_DPTRL_MASK: c_uint = 0xfffffff0;

pub const TSI721_OBDMAC_DPTRH_MASK: c_uint = 0xffffffff;

pub const TSI721_OBDMAC_DSBL_MASK: c_uint = 0xffffffc0;

pub const TSI721_OBDMAC_DSBH_MASK: c_uint = 0xffffffff;

pub const TSI721_OBDMAC_DSSZ_MASK: c_uint = 0x0000000f;

pub const TSI721_OBDMAC_DSRP_MASK: c_uint = 0x0007ffff;

pub const TSI721_OBDMAC_DSWP_MASK: c_uint = 0x0007ffff;
pub const TSI721_RQRPTO: c_uint = 0x60010;
pub const TSI721_RQRPTO_MASK: c_uint = 0x00ffffff;

//
// Inbound Messaging Engine Registers
// x = 0..7
//
pub const TSI721_IB_DEVID_GLOBAL: c_uint = 0xffff;

pub const TSI721_IBDMAC_FQBL_MASK: c_uint = 0xffffffc0;

pub const TSI721_IBDMAC_FQBH_MASK: c_uint = 0xffffffff;

pub const TSI721_IBDMAC_FQSZ_MASK: c_uint = 0x0000000f;

pub const TSI721_IBDMAC_FQRP_MASK: c_uint = 0x0007ffff;

pub const TSI721_IBDMAC_FQWP_MASK: c_uint = 0x0007ffff;

pub const TSI721_IBDMAC_FQTH_MASK: c_uint = 0x0007ffff;
pub const TSI721_IB_DEVID: c_uint = 0x60020;
pub const TSI721_IB_DEVID_MASK: c_uint = 0x0000ffff;

pub const TSI721_IBDMAC_CTL_MASK: c_uint = 0x00000003;
pub const TSI721_IBDMAC_CTL_SUSPEND: c_uint = 0x00000002;
pub const TSI721_IBDMAC_CTL_INIT: c_uint = 0x00000001;

pub const TSI721_IBDMAC_STS_MASK: c_uint = 0x007f0000;
pub const TSI721_IBSMAC_STS_ABORT: c_uint = 0x00400000;
pub const TSI721_IBSMAC_STS_RUN: c_uint = 0x00200000;
pub const TSI721_IBSMAC_STS_CS: c_uint = 0x001f0000;

pub const TSI721_IBDMAC_INT_MASK: c_uint = 0x0000100f;
pub const TSI721_IBDMAC_INT_SRTO: c_uint = 0x00001000;
pub const TSI721_IBDMAC_INT_SUSPENDED: c_uint = 0x00000008;
pub const TSI721_IBDMAC_INT_PC_ERROR: c_uint = 0x00000004;
pub const TSI721_IBDMAC_INT_FQ_LOW: c_uint = 0x00000002;
pub const TSI721_IBDMAC_INT_DQ_RCV: c_uint = 0x00000001;

pub const TSI721_IBDMAC_PWE_MASK: c_uint = 0x00001700;
pub const TSI721_IBDMAC_PWE_SRTO: c_uint = 0x00001000;
pub const TSI721_IBDMAC_PWE_ILL_FMT: c_uint = 0x00000400;
pub const TSI721_IBDMAC_PWE_ILL_DEC: c_uint = 0x00000200;
pub const TSI721_IBDMAC_PWE_IMP_SP: c_uint = 0x00000100;

pub const TSI721_IBDMAC_DQBL_MASK: c_uint = 0xffffffc0;
pub const TSI721_IBDMAC_DQBL_ADDR: c_uint = 0xffffffc0;

pub const TSI721_IBDMAC_DQBH_MASK: c_uint = 0xffffffff;

pub const TSI721_IBDMAC_DQRP_MASK: c_uint = 0x0007ffff;

pub const TSI721_IBDMAC_DQWR_MASK: c_uint = 0x0007ffff;

pub const TSI721_IBDMAC_DQSZ_MASK: c_uint = 0x0000000f;
//
// Messaging Engine Interrupts
//
pub const TSI721_SMSG_PWE: c_uint = 0x6a004;
pub const TSI721_SMSG_INTE: c_uint = 0x6a000;
pub const TSI721_SMSG_INT: c_uint = 0x6a008;
pub const TSI721_SMSG_INTSET: c_uint = 0x6a010;
pub const TSI721_SMSG_INT_MASK: c_uint = 0x0086ffff;
pub const TSI721_SMSG_INT_UNS_RSP: c_uint = 0x00800000;
pub const TSI721_SMSG_INT_ECC_NCOR: c_uint = 0x00040000;
pub const TSI721_SMSG_INT_ECC_COR: c_uint = 0x00020000;
pub const TSI721_SMSG_INT_ECC_NCOR_CH: c_uint = 0x0000ff00;
pub const TSI721_SMSG_INT_ECC_COR_CH: c_uint = 0x000000ff;
pub const TSI721_SMSG_ECC_LOG: c_uint = 0x6a014;
pub const TSI721_SMSG_ECC_LOG_MASK: c_uint = 0x00070007;
pub const TSI721_SMSG_ECC_LOG_ECC_NCOR_M: c_uint = 0x00070000;
pub const TSI721_SMSG_ECC_LOG_ECC_COR_M: c_uint = 0x00000007;
pub const TSI721_RETRY_GEN_CNT: c_uint = 0x6a100;
pub const TSI721_RETRY_GEN_CNT_MASK: c_uint = 0xffffffff;
pub const TSI721_RETRY_RX_CNT: c_uint = 0x6a104;
pub const TSI721_RETRY_RX_CNT_MASK: c_uint = 0xffffffff;

pub const TSI721_SMSG_ECC_COR_LOG_MASK: c_uint = 0x000000ff;

pub const TSI721_SMSG_ECC_NCOR_MASK: c_uint = 0x000000ff;
//
// Block DMA Descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_dma_desc {
    pub type_id: __le32,
pub const TSI721_DMAD_DEVID: c_uint = 0x0000ffff;
pub const TSI721_DMAD_CRF: c_uint = 0x00010000;
pub const TSI721_DMAD_PRIO: c_uint = 0x00060000;
pub const TSI721_DMAD_RTYPE: c_uint = 0x00780000;
pub const TSI721_DMAD_IOF: c_uint = 0x08000000;
pub const TSI721_DMAD_DTYPE: c_uint = 0xe0000000;
    pub bcount: __le32,
pub const TSI721_DMAD_BCOUNT1: c_uint = 0x03ffffff /* if DTYPE == 1 */;
pub const TSI721_DMAD_BCOUNT2: c_uint = 0x0000000f /* if DTYPE == 2 */;
pub const TSI721_DMAD_TT: c_uint = 0x0c000000;
pub const TSI721_DMAD_RADDR0: c_uint = 0xc0000000;
    pub /: *mut *mut __le32 raddr_lo; / if DTYPE == (1 || 2),
    pub /: *mut *mut __le32 next_lo; / if DTYPE == 3,
}

pub const TSI721_DMAD_CFGOFF: c_uint = 0x00ffffff;
pub const TSI721_DMAD_HOPCNT: c_uint = 0xff000000;
//
// Inbound Messaging Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_imsg_desc {
    pub type_id: __le32,
pub const TSI721_IMD_DEVID: c_uint = 0x0000ffff;
pub const TSI721_IMD_CRF: c_uint = 0x00010000;
pub const TSI721_IMD_PRIO: c_uint = 0x00060000;
pub const TSI721_IMD_TT: c_uint = 0x00180000;
pub const TSI721_IMD_DTYPE: c_uint = 0xe0000000;
    pub msg_info: __le32,
pub const TSI721_IMD_BCOUNT: c_uint = 0x00000ff8;
pub const TSI721_IMD_SSIZE: c_uint = 0x0000f000;
pub const TSI721_IMD_LETER: c_uint = 0x00030000;
pub const TSI721_IMD_XMBOX: c_uint = 0x003c0000;
pub const TSI721_IMD_MBOX: c_uint = 0x00c00000;
pub const TSI721_IMD_CS: c_uint = 0x78000000;
pub const TSI721_IMD_HO: c_uint = 0x80000000;
    pub bufptr_lo: __le32,
    pub bufptr_hi: __le32,
    pub reserved: [u32; 12],
    pub __aligned(64): },
//
// Outbound Messaging Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_omsg_desc {
    pub type_id: __le32,
pub const TSI721_OMD_DEVID: c_uint = 0x0000ffff;
pub const TSI721_OMD_CRF: c_uint = 0x00010000;
pub const TSI721_OMD_PRIO: c_uint = 0x00060000;
pub const TSI721_OMD_IOF: c_uint = 0x08000000;
pub const TSI721_OMD_DTYPE: c_uint = 0xe0000000;
pub const TSI721_OMD_RSRVD: c_uint = 0x17f80000;
    pub msg_info: __le32,
pub const TSI721_OMD_BCOUNT: c_uint = 0x00000ff8;
pub const TSI721_OMD_SSIZE: c_uint = 0x0000f000;
pub const TSI721_OMD_LETER: c_uint = 0x00030000;
pub const TSI721_OMD_XMBOX: c_uint = 0x003c0000;
pub const TSI721_OMD_MBOX: c_uint = 0x00c00000;
pub const TSI721_OMD_TT: c_uint = 0x0c000000;
    pub /: *mut *mut __le32 bufptr_lo; / if DTYPE == 4,
    pub /: *mut *mut __le32 next_lo; / if DTYPE == 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_dma_sts {
    pub desc_sts: [__le64; 8],
    pub __aligned(64): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_desc_sts_fifo {
    pub da64: __le64,
    pub lo: __le32,
    pub hi: __le32,
    pub da32: },
    pub stat: [}; 8],
    pub __aligned(64): },
// Descriptor types for BDMA and Messaging blocks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_dtype {
    DTYPE1 = 1, /* Data Transfer DMA Descriptor */
    DTYPE2 = 2, /* Immediate Data Transfer DMA Descriptor */
    DTYPE3 = 3, /* Block Pointer DMA Descriptor */
    DTYPE4 = 4, /* Outbound Msg DMA Descriptor */
    DTYPE5 = 5, /* OB Messaging Block Pointer Descriptor */
    DTYPE6 = 6  /* Inbound Messaging Descriptor */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_rtype {
    NREAD = 0,
    LAST_NWRITE_R = 1,
    ALL_NWRITE = 2,
    ALL_NWRITE_R = 3,
    MAINT_RD = 4,
    MAINT_WR = 5
}

//
// mport Driver Definitions
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsi721_smsg_int_flag {
    SMSG_INT_NONE		= 0x00000000,
    SMSG_INT_ECC_COR_CH	= 0x000000ff,
    SMSG_INT_ECC_NCOR_CH	= 0x0000ff00,
    SMSG_INT_ECC_COR	= 0x00020000,
    SMSG_INT_ECC_NCOR	= 0x00040000,
    SMSG_INT_UNS_RSP	= 0x00800000,
    SMSG_INT_ALL		= 0x0006ffff
}

// Structures

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_tx_desc {
    pub txd: dma_async_tx_descriptor,
    pub destid: u16,
// low 64-bits of 66-bit RIO address
    pub rio_addr: u64,
// upper 2-bits of 66-bit RIO address
    pub rio_addr_u: u8,
    pub rtype: dma_rtype,
    pub desc_node: list_head,
    pub sg: *mut scatterlist,
    pub sg_len: c_uint,
    pub status: dma_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_bdma_chan {
    pub id: c_int,
    pub regs: *mut void __iomem,
    pub /: *mut *mut int bd_num; / number of HW buffer descriptors,
    pub /: *mut *mut *mut void bd_base; / start of DMA descriptors,
    pub bd_phys: dma_addr_t,
    pub /: *mut *mut *mut void sts_base; / start of DMA BD status FIFO,
    pub sts_phys: dma_addr_t,
    pub sts_size: c_int,
    pub sts_rdptr: u32,
    pub wr_count: u32,
    pub wr_count_next: u32,
    pub dchan: dma_chan,
    pub tx_desc: *mut tsi721_tx_desc,
    pub lock: spinlock_t,
    pub active_tx: *mut tsi721_tx_desc,
    pub queue: list_head,
    pub free_list: list_head,
    pub tasklet: tasklet_struct,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_bdma_maint {
    pub /: *mut *mut int ch_id; / BDMA channel number,
    pub /: *mut *mut int bd_num; / number of buffer descriptors,
    pub /: *mut *mut *mut void bd_base; / start of DMA descriptors,
    pub bd_phys: dma_addr_t,
    pub /: *mut *mut *mut void sts_base; / start of DMA BD status FIFO,
    pub sts_phys: dma_addr_t,
    pub sts_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_imsg_ring {
    pub size: u32,
// VA/PA of data buffers for incoming messages
    pub buf_base: *mut c_void,
    pub buf_phys: dma_addr_t,
// VA/PA of circular free buffer list
    pub imfq_base: *mut c_void,
    pub imfq_phys: dma_addr_t,
// VA/PA of Inbound message descriptors
    pub imd_base: *mut c_void,
    pub imd_phys: dma_addr_t,
// Inbound Queue buffer pointers
    pub imq_base: [*mut c_void; TSI721_IMSGD_RING_SIZE],
    pub rx_slot: u32,
    pub dev_id: *mut c_void,
    pub fq_wrptr: u32,
    pub desc_rdptr: u32,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_omsg_ring {
    pub size: u32,
// VA/PA of OB Msg descriptors
    pub omd_base: *mut c_void,
    pub omd_phys: dma_addr_t,
// VA/PA of OB Msg data buffers
    pub omq_base: [*mut c_void; TSI721_OMSGD_RING_SIZE],
    pub omq_phys: [dma_addr_t; TSI721_OMSGD_RING_SIZE],
// VA/PA of OB Msg descriptor status FIFO
    pub sts_base: *mut c_void,
    pub sts_phys: dma_addr_t,
    pub /: *mut *mut u32 sts_size; / # of allocated status entries,
    pub sts_rdptr: u32,
    pub tx_slot: u32,
    pub dev_id: *mut c_void,
    pub wr_count: u32,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsi721_flags {
    TSI721_USING_MSI	= (1 << 0),
    TSI721_USING_MSIX	= (1 << 1),
    TSI721_IMSGID_SET	= (1 << 2),
}

//
// MSI-X Table Entries (0 ... 69)
//

pub const TSI721_MSIX_BDMA_INT: c_int = 16;

pub const TSI721_MSIX_MSG_INT: c_int = 49;

pub const TSI721_MSIX_SR2PC_INT: c_int = 66;
pub const TSI721_MSIX_PC2SR_INT: c_int = 67;
pub const TSI721_MSIX_SRIO_MAC_INT: c_int = 68;
pub const TSI721_MSIX_I2C_INT: c_int = 69;
// MSI-X vector and init table entry indexes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsi721_msix_vect {
    TSI721_VECT_IDB,
    TSI721_VECT_PWRX, /* PW_RX is part of SRIO MAC Interrupt reporting */
    TSI721_VECT_OMB0_DONE,
    TSI721_VECT_OMB1_DONE,
    TSI721_VECT_OMB2_DONE,
    TSI721_VECT_OMB3_DONE,
    TSI721_VECT_OMB0_INT,
    TSI721_VECT_OMB1_INT,
    TSI721_VECT_OMB2_INT,
    TSI721_VECT_OMB3_INT,
    TSI721_VECT_IMB0_RCV,
    TSI721_VECT_IMB1_RCV,
    TSI721_VECT_IMB2_RCV,
    TSI721_VECT_IMB3_RCV,
    TSI721_VECT_IMB0_INT,
    TSI721_VECT_IMB1_INT,
    TSI721_VECT_IMB2_INT,
    TSI721_VECT_IMB3_INT,

    TSI721_VECT_DMA0_DONE,
    TSI721_VECT_DMA1_DONE,
    TSI721_VECT_DMA2_DONE,
    TSI721_VECT_DMA3_DONE,
    TSI721_VECT_DMA4_DONE,
    TSI721_VECT_DMA5_DONE,
    TSI721_VECT_DMA6_DONE,
    TSI721_VECT_DMA7_DONE,
    TSI721_VECT_DMA0_INT,
    TSI721_VECT_DMA1_INT,
    TSI721_VECT_DMA2_INT,
    TSI721_VECT_DMA3_INT,
    TSI721_VECT_DMA4_INT,
    TSI721_VECT_DMA5_INT,
    TSI721_VECT_DMA6_INT,
    TSI721_VECT_DMA7_INT,

    TSI721_VECT_MAX
}

pub const IRQ_DEVICE_NAME_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msix_irq {
    pub vector: u16,
    pub irq_name: [c_char; IRQ_DEVICE_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_ib_win_mapping {
    pub node: list_head,
    pub lstart: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_ib_win {
    pub rstart: u64,
    pub size: u32,
    pub lstart: dma_addr_t,
    pub active: bool,
    pub xlat: bool,
    pub mappings: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_obw_bar {
    pub base: u64,
    pub size: u64,
    pub free: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_ob_win {
    pub base: u64,
    pub size: u32,
    pub destid: u16,
    pub rstart: u64,
    pub active: bool,
    pub pbar: *mut tsi721_obw_bar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsi721_device {
    pub pdev: *mut pci_dev,
    pub mport: rio_mport,
    pub flags: u32,
    pub regs: *mut void __iomem,
    pub msix: [msix_irq; TSI721_VECT_MAX],
// Doorbells
    pub odb_base: *mut void __iomem,
    pub idb_base: *mut c_void,
    pub idb_dma: dma_addr_t,
    pub idb_work: work_struct,
    pub db_discard_count: u32,
// Inbound Port-Write
    pub pw_work: work_struct,
    pub pw_fifo: kfifo,
    pub pw_fifo_lock: spinlock_t,
    pub pw_discard_count: u32,
// BDMA Engine
    pub /: *mut *mut tsi721_bdma_maint mdma; / Maintenance rd/wr request channel,
    pub bdma: [tsi721_bdma_chan; TSI721_DMA_CHNUM],
// Inbound Messaging
    pub imsg_init: [c_int; TSI721_IMSG_CHNUM],
    pub imsg_ring: [tsi721_imsg_ring; TSI721_IMSG_CHNUM],
// Outbound Messaging
    pub omsg_init: [c_int; TSI721_OMSG_CHNUM],
    pub omsg_ring: [tsi721_omsg_ring; TSI721_OMSG_CHNUM],
// Inbound Mapping Windows
    pub ib_win: [tsi721_ib_win; TSI721_IBWIN_NUM],
    pub ibwin_cnt: c_int,
// Outbound Mapping Windows
    pub p2r_bar: [tsi721_obw_bar; 2],
    pub ob_win: [tsi721_ob_win; TSI721_OBWIN_NUM],
    pub obwin_cnt: c_int,
}

extern "C" {
    pub fn tsi721_bdma_handler(bdma_chan: *mut tsi721_bdma_chan);
}
extern "C" {
    pub fn tsi721_register_dma(priv: *mut tsi721_device) -> c_int;
}
extern "C" {
    pub fn tsi721_unregister_dma(priv: *mut tsi721_device);
}
extern "C" {
    pub fn tsi721_dma_stop_all(priv: *mut tsi721_device);
}

