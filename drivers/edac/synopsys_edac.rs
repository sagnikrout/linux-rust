//! Automatically rewritten from C to Rust
//! Source: drivers/edac/synopsys_edac.c
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
// Synopsys DDR ECC Driver
// This driver is based on ppc4xx_edac.c drivers
//
// Copyright (C) 2012 - 2014 Xilinx, Inc.
//

// Number of cs_rows needed per memory controller
pub const SYNPS_EDAC_NR_CSROWS: c_int = 1;
// Number of channels per memory controller
pub const SYNPS_EDAC_NR_CHANS: c_int = 1;
// Granularity of reported error in bytes
pub const SYNPS_EDAC_ERR_GRAIN: c_int = 1;
pub const SYNPS_EDAC_MSG_SIZE: c_int = 256;

// Synopsys DDR memory controller registers that are relevant to ECC
pub const CTRL_OFST: c_uint = 0x0;
pub const T_ZQ_OFST: c_uint = 0xA4;
// ECC control register
pub const ECC_CTRL_OFST: c_uint = 0xC4;
// ECC log register
pub const CE_LOG_OFST: c_uint = 0xC8;
// ECC address register
pub const CE_ADDR_OFST: c_uint = 0xCC;
// ECC data[31:0] register
pub const CE_DATA_31_0_OFST: c_uint = 0xD0;
// Uncorrectable error info registers
pub const UE_LOG_OFST: c_uint = 0xDC;
pub const UE_ADDR_OFST: c_uint = 0xE0;
pub const UE_DATA_31_0_OFST: c_uint = 0xE4;
pub const STAT_OFST: c_uint = 0xF0;
pub const SCRUB_OFST: c_uint = 0xF4;
// Control register bit field definitions
pub const CTRL_BW_MASK: c_uint = 0xC;
pub const CTRL_BW_SHIFT: c_int = 2;
pub const DDRCTL_WDTH_16: c_int = 1;
pub const DDRCTL_WDTH_32: c_int = 0;
// ZQ register bit field definitions
pub const T_ZQ_DDRMODE_MASK: c_uint = 0x2;
// ECC control register bit field definitions
pub const ECC_CTRL_CLR_CE_ERR: c_uint = 0x2;
pub const ECC_CTRL_CLR_UE_ERR: c_uint = 0x1;
// ECC correctable/uncorrectable error log register definitions
pub const LOG_VALID: c_uint = 0x1;
pub const CE_LOG_BITPOS_MASK: c_uint = 0xFE;
pub const CE_LOG_BITPOS_SHIFT: c_int = 1;
// ECC correctable/uncorrectable error address register definitions
pub const ADDR_COL_MASK: c_uint = 0xFFF;
pub const ADDR_ROW_MASK: c_uint = 0xFFFF000;
pub const ADDR_ROW_SHIFT: c_int = 12;
pub const ADDR_BANK_MASK: c_uint = 0x70000000;
pub const ADDR_BANK_SHIFT: c_int = 28;
// ECC statistic register definitions
pub const STAT_UECNT_MASK: c_uint = 0xFF;
pub const STAT_CECNT_MASK: c_uint = 0xFF00;
pub const STAT_CECNT_SHIFT: c_int = 8;
// ECC scrub register definitions
pub const SCRUB_MODE_MASK: c_uint = 0x7;
pub const SCRUB_MODE_SECDED: c_uint = 0x4;
// DDR ECC Quirks

// ZynqMP Enhanced DDR memory controller registers that are relevant to ECC
// ECC Configuration Registers
pub const ECC_CFG0_OFST: c_uint = 0x70;
pub const ECC_CFG1_OFST: c_uint = 0x74;
// ECC Status Register
pub const ECC_STAT_OFST: c_uint = 0x78;
// ECC Clear Register
pub const ECC_CLR_OFST: c_uint = 0x7C;
// ECC Error count Register
pub const ECC_ERRCNT_OFST: c_uint = 0x80;
// ECC Corrected Error Address Register
pub const ECC_CEADDR0_OFST: c_uint = 0x84;
pub const ECC_CEADDR1_OFST: c_uint = 0x88;
// ECC Syndrome Registers
pub const ECC_CSYND0_OFST: c_uint = 0x8C;
pub const ECC_CSYND1_OFST: c_uint = 0x90;
pub const ECC_CSYND2_OFST: c_uint = 0x94;
// ECC Bit Mask0 Address Register
pub const ECC_BITMASK0_OFST: c_uint = 0x98;
pub const ECC_BITMASK1_OFST: c_uint = 0x9C;
pub const ECC_BITMASK2_OFST: c_uint = 0xA0;
// ECC UnCorrected Error Address Register
pub const ECC_UEADDR0_OFST: c_uint = 0xA4;
pub const ECC_UEADDR1_OFST: c_uint = 0xA8;
// ECC Syndrome Registers
pub const ECC_UESYND0_OFST: c_uint = 0xAC;
pub const ECC_UESYND1_OFST: c_uint = 0xB0;
pub const ECC_UESYND2_OFST: c_uint = 0xB4;
// ECC Poison Address Reg
pub const ECC_POISON0_OFST: c_uint = 0xB8;
pub const ECC_POISON1_OFST: c_uint = 0xBC;
pub const ECC_ADDRMAP0_OFFSET: c_uint = 0x200;
// Control register bitfield definitions
pub const ECC_CTRL_BUSWIDTH_MASK: c_uint = 0x3000;
pub const ECC_CTRL_BUSWIDTH_SHIFT: c_int = 12;

// DDR Control Register width definitions
pub const DDRCTL_EWDTH_16: c_int = 2;
pub const DDRCTL_EWDTH_32: c_int = 1;
pub const DDRCTL_EWDTH_64: c_int = 0;
// ECC status register definitions
pub const ECC_STAT_UECNT_MASK: c_uint = 0xF0000;
pub const ECC_STAT_UECNT_SHIFT: c_int = 16;
pub const ECC_STAT_CECNT_MASK: c_uint = 0xF00;
pub const ECC_STAT_CECNT_SHIFT: c_int = 8;
pub const ECC_STAT_BITNUM_MASK: c_uint = 0x7F;
// ECC error count register definitions
pub const ECC_ERRCNT_UECNT_MASK: c_uint = 0xFFFF0000;
pub const ECC_ERRCNT_UECNT_SHIFT: c_int = 16;
pub const ECC_ERRCNT_CECNT_MASK: c_uint = 0xFFFF;
// DDR QOS Interrupt register definitions
pub const DDR_QOS_IRQ_STAT_OFST: c_uint = 0x20200;
pub const DDR_QOSUE_MASK: c_uint = 0x4;
pub const DDR_QOSCE_MASK: c_uint = 0x2;
pub const ECC_CE_UE_INTR_MASK: c_uint = 0x6;
pub const DDR_QOS_IRQ_EN_OFST: c_uint = 0x20208;
pub const DDR_QOS_IRQ_DB_OFST: c_uint = 0x2020C;
// DDR QOS Interrupt register definitions

// ECC Corrected Error Register Mask and Shifts
pub const ECC_CEADDR0_RW_MASK: c_uint = 0x3FFFF;

pub const ECC_CEADDR1_BNKGRP_MASK: c_uint = 0x3000000;
pub const ECC_CEADDR1_BNKNR_MASK: c_uint = 0x70000;
pub const ECC_CEADDR1_BLKNR_MASK: c_uint = 0xFFF;
pub const ECC_CEADDR1_BNKGRP_SHIFT: c_int = 24;
pub const ECC_CEADDR1_BNKNR_SHIFT: c_int = 16;
// ECC Poison register shifts
pub const ECC_POISON0_RANK_SHIFT: c_int = 24;

pub const ECC_POISON0_COLUMN_SHIFT: c_int = 0;
pub const ECC_POISON0_COLUMN_MASK: c_uint = 0xFFF;
pub const ECC_POISON1_BG_SHIFT: c_int = 28;
pub const ECC_POISON1_BG_MASK: c_uint = 0x30000000;
pub const ECC_POISON1_BANKNR_SHIFT: c_int = 24;
pub const ECC_POISON1_BANKNR_MASK: c_uint = 0x7000000;
pub const ECC_POISON1_ROW_SHIFT: c_int = 0;
pub const ECC_POISON1_ROW_MASK: c_uint = 0x3FFFF;
// DDR Memory type defines
pub const MEM_TYPE_DDR3: c_uint = 0x1;
pub const MEM_TYPE_LPDDR3: c_uint = 0x8;
pub const MEM_TYPE_DDR2: c_uint = 0x4;
pub const MEM_TYPE_DDR4: c_uint = 0x10;
pub const MEM_TYPE_LPDDR4: c_uint = 0x20;
// DDRC Software control register
pub const DDRC_SWCTL: c_uint = 0x320;
// DDRC ECC CE & UE poison mask
pub const ECC_CEPOISON_MASK: c_uint = 0x3;
pub const ECC_UEPOISON_MASK: c_uint = 0x1;
// DDRC Device config masks
pub const DDRC_MSTR_CFG_MASK: c_uint = 0xC0000000;
pub const DDRC_MSTR_CFG_SHIFT: c_int = 30;
pub const DDRC_MSTR_CFG_X4_MASK: c_uint = 0x0;
pub const DDRC_MSTR_CFG_X8_MASK: c_uint = 0x1;
pub const DDRC_MSTR_CFG_X16_MASK: c_uint = 0x2;
pub const DDRC_MSTR_CFG_X32_MASK: c_uint = 0x3;
pub const DDR_MAX_ROW_SHIFT: c_int = 18;
pub const DDR_MAX_COL_SHIFT: c_int = 14;
pub const DDR_MAX_BANK_SHIFT: c_int = 3;
pub const DDR_MAX_BANKGRP_SHIFT: c_int = 2;
pub const ROW_MAX_VAL_MASK: c_uint = 0xF;
pub const COL_MAX_VAL_MASK: c_uint = 0xF;
pub const BANK_MAX_VAL_MASK: c_uint = 0x1F;
pub const BANKGRP_MAX_VAL_MASK: c_uint = 0x1F;
pub const RANK_MAX_VAL_MASK: c_uint = 0x1F;
pub const ROW_B0_BASE: c_int = 6;
pub const ROW_B1_BASE: c_int = 7;
pub const ROW_B2_BASE: c_int = 8;
pub const ROW_B3_BASE: c_int = 9;
pub const ROW_B4_BASE: c_int = 10;
pub const ROW_B5_BASE: c_int = 11;
pub const ROW_B6_BASE: c_int = 12;
pub const ROW_B7_BASE: c_int = 13;
pub const ROW_B8_BASE: c_int = 14;
pub const ROW_B9_BASE: c_int = 15;
pub const ROW_B10_BASE: c_int = 16;
pub const ROW_B11_BASE: c_int = 17;
pub const ROW_B12_BASE: c_int = 18;
pub const ROW_B13_BASE: c_int = 19;
pub const ROW_B14_BASE: c_int = 20;
pub const ROW_B15_BASE: c_int = 21;
pub const ROW_B16_BASE: c_int = 22;
pub const ROW_B17_BASE: c_int = 23;
pub const COL_B2_BASE: c_int = 2;
pub const COL_B3_BASE: c_int = 3;
pub const COL_B4_BASE: c_int = 4;
pub const COL_B5_BASE: c_int = 5;
pub const COL_B6_BASE: c_int = 6;
pub const COL_B7_BASE: c_int = 7;
pub const COL_B8_BASE: c_int = 8;
pub const COL_B9_BASE: c_int = 9;
pub const COL_B10_BASE: c_int = 10;
pub const COL_B11_BASE: c_int = 11;
pub const COL_B12_BASE: c_int = 12;
pub const COL_B13_BASE: c_int = 13;
pub const BANK_B0_BASE: c_int = 2;
pub const BANK_B1_BASE: c_int = 3;
pub const BANK_B2_BASE: c_int = 4;
pub const BANKGRP_B0_BASE: c_int = 2;
pub const BANKGRP_B1_BASE: c_int = 3;
pub const RANK_B0_BASE: c_int = 6;
//
// struct ecc_error_info - ECC error log information.
// @row:	Row number.
// @col:	Column number.
// @bank:	Bank number.
// @bitpos:	Bit position.
// @data:	Data causing the error.
// @bankgrpnr:	Bank group number.
// @blknr:	Block number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_error_info {
    pub row: u32,
    pub col: u32,
    pub bank: u32,
    pub bitpos: u32,
    pub data: u32,
    pub bankgrpnr: u32,
    pub blknr: u32,
}

//
// struct synps_ecc_status - ECC status information to report.
// @ce_cnt:	Correctable error count.
// @ue_cnt:	Uncorrectable error count.
// @ceinfo:	Correctable error log information.
// @ueinfo:	Uncorrectable error log information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synps_ecc_status {
    pub ce_cnt: u32,
    pub ue_cnt: u32,
    pub ceinfo: ecc_error_info,
    pub ueinfo: ecc_error_info,
}

//
// struct synps_edac_priv - DDR memory controller private instance data.
// @baseaddr:		Base address of the DDR controller.
// @reglock:		Concurrent CSRs access lock.
// @message:		Buffer for framing the event specific info.
// @stat:		ECC status information.
// @p_data:		Platform data.
// @ce_cnt:		Correctable Error count.
// @ue_cnt:		Uncorrectable Error count.
// @poison_addr:	Data poison address.
// @row_shift:		Bit shifts for row bit.
// @col_shift:		Bit shifts for column bit.
// @bank_shift:		Bit shifts for bank bit.
// @bankgrp_shift:	Bit shifts for bank group bit.
// @rank_shift:		Bit shifts for rank bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synps_edac_priv {
    pub baseaddr: *mut void __iomem,
    pub reglock: spinlock_t,
    pub message: [c_char; SYNPS_EDAC_MSG_SIZE],
    pub stat: synps_ecc_status,
    pub p_data: *const synps_platform_data,
    pub ce_cnt: u32,
    pub ue_cnt: u32,

    pub poison_addr: c_ulong,
    pub row_shift: [u32; 18],
    pub col_shift: [u32; 14],
    pub bank_shift: [u32; 3],
    pub bankgrp_shift: [u32; 2],
    pub rank_shift: [u32; 1],
}

    enum synps_platform_type {
    ZYNQ,
    ZYNQMP,
    SYNPS,
    };
//
// struct synps_platform_data -  synps platform data structure.
// @platform:		Identifies the target hardware platform
// @get_error_info:	Get EDAC error info.
// @get_mtype:		Get mtype.
// @get_dtype:		Get dtype.
// @get_mem_info:	Get EDAC memory info
// @quirks:		To differentiate IPs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synps_platform_data {
    pub platform: enum synps_platform_type,
    pub priv): *mut *mut int (get_error_info)(struct synps_edac_priv,
    pub base): *const *const enum mem_type (get_mtype)(void __iomem,
    pub base): *const *const enum dev_type (get_dtype)(void __iomem,

    pub priv): *mut *mut u64 (get_mem_info)(struct synps_edac_priv,

    pub quirks: c_int,
}

//
// zynq_get_error_info - Get the current ECC error info.
// @priv:	DDR memory controller private instance data.
//
// Return: one if there is no error, otherwise zero.
//
#[no_mangle]
unsafe extern "C" fn zynq_get_error_info(priv: *mut synps_edac_priv) -> c_int {
    static int zynq_get_error_info(struct synps_edac_priv *priv)
    {
    struct synps_ecc_status *p;
    u32 regval, clearval = 0;
    void __iomem *base;
    base = priv.baseaddr;
    p = &priv.stat;
    regval = readl(base + STAT_OFST);
    if (!regval)
    return 1;
    p.ce_cnt = (regval & STAT_CECNT_MASK) >> STAT_CECNT_SHIFT;
    p.ue_cnt = regval & STAT_UECNT_MASK;
    regval = readl(base + CE_LOG_OFST);
    if (!(p.ce_cnt && (regval & LOG_VALID)))
    goto ue_err;
    p.ceinfo.bitpos = (regval & CE_LOG_BITPOS_MASK) >> CE_LOG_BITPOS_SHIFT;
    regval = readl(base + CE_ADDR_OFST);
    p.ceinfo.row = (regval & ADDR_ROW_MASK) >> ADDR_ROW_SHIFT;
    p.ceinfo.col = regval & ADDR_COL_MASK;
    p.ceinfo.bank = (regval & ADDR_BANK_MASK) >> ADDR_BANK_SHIFT;
    p.ceinfo.data = readl(base + CE_DATA_31_0_OFST);
    edac_dbg(3, "CE bit position: %d data: %d\n", p.ceinfo.bitpos,
    p.ceinfo.data);
    clearval = ECC_CTRL_CLR_CE_ERR;
    ue_err:
    regval = readl(base + UE_LOG_OFST);
    if (!(p.ue_cnt && (regval & LOG_VALID)))
    goto out;
    regval = readl(base + UE_ADDR_OFST);
    p.ueinfo.row = (regval & ADDR_ROW_MASK) >> ADDR_ROW_SHIFT;
    p.ueinfo.col = regval & ADDR_COL_MASK;
    p.ueinfo.bank = (regval & ADDR_BANK_MASK) >> ADDR_BANK_SHIFT;
    p.ueinfo.data = readl(base + UE_DATA_31_0_OFST);
    clearval |= ECC_CTRL_CLR_UE_ERR;
    out:
    writel(clearval, base + ECC_CTRL_OFST);
    writel(0x0, base + ECC_CTRL_OFST);
    return 0;
    }

//
// zynqmp_get_mem_info - Get the current memory info.
// @priv:	DDR memory controller private instance data.
//
// Return: host interface address.
//
#[no_mangle]
unsafe extern "C" fn zynqmp_get_mem_info(priv: *mut synps_edac_priv) -> u64 {
    static u64 zynqmp_get_mem_info(struct synps_edac_priv *priv)
    {
    let mut hif_addr: u64 = 0, linear_addr;
    linear_addr = priv.poison_addr;
    if (linear_addr >= SZ_32G)
    linear_addr = linear_addr - SZ_32G + SZ_2G;
    hif_addr = linear_addr >> 3;
    return hif_addr;
    }

//
// zynqmp_get_error_info - Get the current ECC error info.
// @priv:	DDR memory controller private instance data.
//
// Return: one if there is no error otherwise returns zero.
//
#[no_mangle]
unsafe extern "C" fn zynqmp_get_error_info(priv: *mut synps_edac_priv) -> c_int {
    static int zynqmp_get_error_info(struct synps_edac_priv *priv)
    {
    struct synps_ecc_status *p;
    u32 regval, clearval;
    unsigned long flags;
    void __iomem *base;
    base = priv.baseaddr;
    p = &priv.stat;
    regval = readl(base + ECC_ERRCNT_OFST);
    p.ce_cnt = regval & ECC_ERRCNT_CECNT_MASK;
    p.ue_cnt = (regval & ECC_ERRCNT_UECNT_MASK) >> ECC_ERRCNT_UECNT_SHIFT;
    if (!p.ce_cnt)
    goto ue_err;
    regval = readl(base + ECC_STAT_OFST);
    if (!regval)
    return 1;
    p.ceinfo.bitpos = (regval & ECC_STAT_BITNUM_MASK);
    regval = readl(base + ECC_CEADDR0_OFST);
    p.ceinfo.row = (regval & ECC_CEADDR0_RW_MASK);
    regval = readl(base + ECC_CEADDR1_OFST);
    p.ceinfo.bank = (regval & ECC_CEADDR1_BNKNR_MASK) >>
    ECC_CEADDR1_BNKNR_SHIFT;
    p.ceinfo.bankgrpnr = (regval &	ECC_CEADDR1_BNKGRP_MASK) >>
    ECC_CEADDR1_BNKGRP_SHIFT;
    p.ceinfo.blknr = (regval & ECC_CEADDR1_BLKNR_MASK);
    p.ceinfo.data = readl(base + ECC_CSYND0_OFST);
    edac_dbg(2, "ECCCSYN0: 0x%08X ECCCSYN1: 0x%08X ECCCSYN2: 0x%08X\n",
    readl(base + ECC_CSYND0_OFST), readl(base + ECC_CSYND1_OFST),
    readl(base + ECC_CSYND2_OFST));
    ue_err:
    if (!p.ue_cnt)
    goto out;
    regval = readl(base + ECC_UEADDR0_OFST);
    p.ueinfo.row = (regval & ECC_CEADDR0_RW_MASK);
    regval = readl(base + ECC_UEADDR1_OFST);
    p.ueinfo.bankgrpnr = (regval & ECC_CEADDR1_BNKGRP_MASK) >>
    ECC_CEADDR1_BNKGRP_SHIFT;
    p.ueinfo.bank = (regval & ECC_CEADDR1_BNKNR_MASK) >>
    ECC_CEADDR1_BNKNR_SHIFT;
    p.ueinfo.blknr = (regval & ECC_CEADDR1_BLKNR_MASK);
    p.ueinfo.data = readl(base + ECC_UESYND0_OFST);
    out:
    spin_lock_irqsave(&priv.reglock, flags);
    clearval = readl(base + ECC_CLR_OFST) |
    ECC_CTRL_CLR_CE_ERR | ECC_CTRL_CLR_CE_ERRCNT |
    ECC_CTRL_CLR_UE_ERR | ECC_CTRL_CLR_UE_ERRCNT;
    writel(clearval, base + ECC_CLR_OFST);
    spin_unlock_irqrestore(&priv.reglock, flags);
    return 0;
    }
//
// handle_error - Handle Correctable and Uncorrectable errors.
// @mci:	EDAC memory controller instance.
// @p:		Synopsys ECC status structure.
//
// Handles ECC correctable and uncorrectable errors.
//
#[no_mangle]
unsafe extern "C" fn handle_error(mci: *mut mem_ctl_info, p: *mut synps_ecc_status) {
    static void handle_error(struct mem_ctl_info *mci, struct synps_ecc_status *p)
    {
    struct synps_edac_priv *priv = mci.pvt_info;
    struct ecc_error_info *pinf;
    if (p.ce_cnt) {
    pinf = &p.ceinfo;
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT) {
    snprintf(priv.message, SYNPS_EDAC_MSG_SIZE,
    "DDR ECC error type:%s Row %d Bank %d BankGroup Number %d Block Number %d Bit Position: %d Data: 0x%08x",
    "CE", pinf.row, pinf.bank,
    pinf.bankgrpnr, pinf.blknr,
    pinf.bitpos, pinf.data);
    } else {
    snprintf(priv.message, SYNPS_EDAC_MSG_SIZE,
    "DDR ECC error type:%s Row %d Bank %d Col %d Bit Position: %d Data: 0x%08x",
    "CE", pinf.row, pinf.bank, pinf.col,
    pinf.bitpos, pinf.data);
    }
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci,
    p.ce_cnt, 0, 0, 0, 0, 0, -1,
    priv.message, "");
    }
    if (p.ue_cnt) {
    pinf = &p.ueinfo;
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT) {
    snprintf(priv.message, SYNPS_EDAC_MSG_SIZE,
    "DDR ECC error type :%s Row %d Bank %d BankGroup Number %d Block Number %d",
    "UE", pinf.row, pinf.bank,
    pinf.bankgrpnr, pinf.blknr);
    } else {
    snprintf(priv.message, SYNPS_EDAC_MSG_SIZE,
    "DDR ECC error type :%s Row %d Bank %d Col %d ",
    "UE", pinf.row, pinf.bank, pinf.col);
    }
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci,
    p.ue_cnt, 0, 0, 0, 0, 0, -1,
    priv.message, "");
    }
    memset(p, 0, sizeof(*p));
    }
#[no_mangle]
unsafe extern "C" fn enable_intr(priv: *mut synps_edac_priv) {
    static void enable_intr(struct synps_edac_priv *priv)
    {
    unsigned long flags;
// Enable UE/CE Interrupts
    if (!(priv.p_data.quirks & DDR_ECC_INTR_SELF_CLEAR)) {
    writel(DDR_QOSUE_MASK | DDR_QOSCE_MASK,
    priv.baseaddr + DDR_QOS_IRQ_EN_OFST);
    return;
    }
    spin_lock_irqsave(&priv.reglock, flags);
    writel(DDR_UE_MASK | DDR_CE_MASK,
    priv.baseaddr + ECC_CLR_OFST);
    spin_unlock_irqrestore(&priv.reglock, flags);
    }
#[no_mangle]
unsafe extern "C" fn disable_intr(priv: *mut synps_edac_priv) {
    static void disable_intr(struct synps_edac_priv *priv)
    {
    unsigned long flags;
// Disable UE/CE Interrupts
    if (!(priv.p_data.quirks & DDR_ECC_INTR_SELF_CLEAR)) {
    writel(DDR_QOSUE_MASK | DDR_QOSCE_MASK,
    priv.baseaddr + DDR_QOS_IRQ_DB_OFST);
    return;
    }
    spin_lock_irqsave(&priv.reglock, flags);
    writel(0, priv.baseaddr + ECC_CLR_OFST);
    spin_unlock_irqrestore(&priv.reglock, flags);
    }
//
// intr_handler - Interrupt Handler for ECC interrupts.
// @irq:        IRQ number.
// @dev_id:     Device ID.
//
// Return: IRQ_NONE, if interrupt not set or IRQ_HANDLED otherwise.
//
#[no_mangle]
unsafe extern "C" fn intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t intr_handler(int irq, void *dev_id)
    {
    const struct synps_platform_data *p_data;
    struct mem_ctl_info *mci = dev_id;
    struct synps_edac_priv *priv;
    int status, regval;
    priv = mci.pvt_info;
    p_data = priv.p_data;
//
// v3.0 of the controller has the ce/ue bits cleared automatically,
// so this condition does not apply.
//
    if (!(priv.p_data.quirks & DDR_ECC_INTR_SELF_CLEAR)) {
    regval = readl(priv.baseaddr + DDR_QOS_IRQ_STAT_OFST);
    regval &= (DDR_QOSCE_MASK | DDR_QOSUE_MASK);
    if (!(regval & ECC_CE_UE_INTR_MASK))
    return IRQ_NONE;
    }
    status = p_data.get_error_info(priv);
    if (status)
    return IRQ_NONE;
    priv.ce_cnt += priv.stat.ce_cnt;
    priv.ue_cnt += priv.stat.ue_cnt;
    handle_error(mci, &priv.stat);
    edac_dbg(3, "Total error count CE %d UE %d\n",
    priv.ce_cnt, priv.ue_cnt);
// v3.0 of the controller does not have this register
    if (!(priv.p_data.quirks & DDR_ECC_INTR_SELF_CLEAR))
    writel(regval, priv.baseaddr + DDR_QOS_IRQ_STAT_OFST);
    return IRQ_HANDLED;
    }
//
// check_errors - Check controller for ECC errors.
// @mci:	EDAC memory controller instance.
//
// Check and post ECC errors. Called by the polling thread.
//
#[no_mangle]
unsafe extern "C" fn check_errors(mci: *mut mem_ctl_info) {
    static void check_errors(struct mem_ctl_info *mci)
    {
    const struct synps_platform_data *p_data;
    struct synps_edac_priv *priv;
    int status;
    priv = mci.pvt_info;
    p_data = priv.p_data;
    status = p_data.get_error_info(priv);
    if (status)
    return;
    priv.ce_cnt += priv.stat.ce_cnt;
    priv.ue_cnt += priv.stat.ue_cnt;
    handle_error(mci, &priv.stat);
    edac_dbg(3, "Total error count CE %d UE %d\n",
    priv.ce_cnt, priv.ue_cnt);
    }
//
// zynq_get_dtype - Return the controller memory width.
// @base:	DDR memory controller base address.
//
// Get the EDAC device type width appropriate for the current controller
// configuration.
//
// Return: a device type width enumeration.
//
#[no_mangle]
unsafe extern "C" fn zynq_get_dtype(base: *const void __iomem) -> enum dev_type {
    static enum dev_type zynq_get_dtype(const void __iomem *base)
    {
    enum dev_type dt;
    u32 width;
    width = readl(base + CTRL_OFST);
    width = (width & CTRL_BW_MASK) >> CTRL_BW_SHIFT;
    switch (width) {
    case DDRCTL_WDTH_16:
    dt = DEV_X2;
    break;
    case DDRCTL_WDTH_32:
    dt = DEV_X4;
    break;
    default:
    dt = DEV_UNKNOWN;
    }
    return dt;
    }
//
// zynqmp_get_dtype - Return the controller memory width.
// @base:	DDR memory controller base address.
//
// Get the EDAC device type width appropriate for the current controller
// configuration.
//
// Return: a device type width enumeration.
//
#[no_mangle]
unsafe extern "C" fn zynqmp_get_dtype(base: *const void __iomem) -> enum dev_type {
    static enum dev_type zynqmp_get_dtype(const void __iomem *base)
    {
    enum dev_type dt;
    u32 width;
    width = readl(base + CTRL_OFST);
    width = (width & ECC_CTRL_BUSWIDTH_MASK) >> ECC_CTRL_BUSWIDTH_SHIFT;
    switch (width) {
    case DDRCTL_EWDTH_16:
    dt = DEV_X2;
    break;
    case DDRCTL_EWDTH_32:
    dt = DEV_X4;
    break;
    case DDRCTL_EWDTH_64:
    dt = DEV_X8;
    break;
    default:
    dt = DEV_UNKNOWN;
    }
    return dt;
    }
#[no_mangle]
unsafe extern "C" fn get_ecc_state(priv: *mut synps_edac_priv) -> bool {
    static bool get_ecc_state(struct synps_edac_priv *priv)
    {
    u32 ecctype, clearval;
    enum dev_type dt;
    if (priv.p_data.platform == ZYNQ) {
    dt = zynq_get_dtype(priv.baseaddr);
    if (dt == DEV_UNKNOWN)
    return false;
    ecctype = readl(priv.baseaddr + SCRUB_OFST) & SCRUB_MODE_MASK;
    if (ecctype == SCRUB_MODE_SECDED && dt == DEV_X2) {
    clearval = ECC_CTRL_CLR_CE_ERR | ECC_CTRL_CLR_UE_ERR;
    writel(clearval, priv.baseaddr + ECC_CTRL_OFST);
    writel(0x0, priv.baseaddr + ECC_CTRL_OFST);
    return true;
    }
    } else {
    dt = zynqmp_get_dtype(priv.baseaddr);
    if (dt == DEV_UNKNOWN)
    return false;
    ecctype = readl(priv.baseaddr + ECC_CFG0_OFST) & SCRUB_MODE_MASK;
    if (ecctype == SCRUB_MODE_SECDED &&
    (dt == DEV_X2 || dt == DEV_X4 || dt == DEV_X8)) {
    clearval = readl(priv.baseaddr + ECC_CLR_OFST) |
    ECC_CTRL_CLR_CE_ERR | ECC_CTRL_CLR_CE_ERRCNT |
    ECC_CTRL_CLR_UE_ERR | ECC_CTRL_CLR_UE_ERRCNT;
    writel(clearval, priv.baseaddr + ECC_CLR_OFST);
    return true;
    }
    }
    return false;
    }
//
// get_memsize - Read the size of the attached memory device.
//
// Return: the memory size in bytes.
//
#[no_mangle]
unsafe extern "C" fn get_memsize() -> u32 {
    static u32 get_memsize(void)
    {
    struct sysinfo inf;
    si_meminfo(&inf);
    return inf.totalram * inf.mem_unit;
    }
//
// zynq_get_mtype - Return the controller memory type.
// @base:	Synopsys ECC status structure.
//
// Get the EDAC memory type appropriate for the current controller
// configuration.
//
// Return: a memory type enumeration.
//
#[no_mangle]
unsafe extern "C" fn zynq_get_mtype(base: *const void __iomem) -> enum mem_type {
    static enum mem_type zynq_get_mtype(const void __iomem *base)
    {
    enum mem_type mt;
    u32 memtype;
    memtype = readl(base + T_ZQ_OFST);
    if (memtype & T_ZQ_DDRMODE_MASK)
    mt = MEM_DDR3;
    else
    mt = MEM_DDR2;
    return mt;
    }
//
// zynqmp_get_mtype - Returns controller memory type.
// @base:	Synopsys ECC status structure.
//
// Get the EDAC memory type appropriate for the current controller
// configuration.
//
// Return: a memory type enumeration.
//
#[no_mangle]
unsafe extern "C" fn zynqmp_get_mtype(base: *const void __iomem) -> enum mem_type {
    static enum mem_type zynqmp_get_mtype(const void __iomem *base)
    {
    enum mem_type mt;
    u32 memtype;
    memtype = readl(base + CTRL_OFST);
    if ((memtype & MEM_TYPE_DDR3) || (memtype & MEM_TYPE_LPDDR3))
    mt = MEM_DDR3;
#[no_mangle]
pub unsafe extern "C" fn if(MEM_TYPE_DDR2: memtype &) -> else {
    else if (memtype & MEM_TYPE_DDR2)
    mt = MEM_RDDR2;
#[no_mangle]
pub unsafe extern "C" fn if(MEM_TYPE_DDR4): (memtype & MEM_TYPE_LPDDR4) || (memtype &) -> else {
    else if ((memtype & MEM_TYPE_LPDDR4) || (memtype & MEM_TYPE_DDR4))
    mt = MEM_DDR4;
    else
    mt = MEM_EMPTY;
    return mt;
    }
//
// init_csrows - Initialize the csrow data.
// @mci:	EDAC memory controller instance.
//
// Initialize the chip select rows associated with the EDAC memory
// controller instance.
//
#[no_mangle]
unsafe extern "C" fn init_csrows(mci: *mut mem_ctl_info) {
    static void init_csrows(struct mem_ctl_info *mci)
    {
    struct synps_edac_priv *priv = mci.pvt_info;
    const struct synps_platform_data *p_data;
    struct csrow_info *csi;
    struct dimm_info *dimm;
    u32 size, row;
    int j;
    p_data = priv.p_data;
    for (row = 0; row < mci.nr_csrows; row++) {
    csi = mci.csrows[row];
    size = get_memsize();
    for (j = 0; j < csi.nr_channels; j++) {
    dimm		= csi.channels[j].dimm;
    dimm.edac_mode	= EDAC_SECDED;
    dimm.mtype	= p_data.get_mtype(priv.baseaddr);
    dimm.nr_pages	= (size >> PAGE_SHIFT) / csi.nr_channels;
    dimm.grain	= SYNPS_EDAC_ERR_GRAIN;
    dimm.dtype	= p_data.get_dtype(priv.baseaddr);
    }
    }
    }
//
// mc_init - Initialize one driver instance.
// @mci:	EDAC memory controller instance.
// @pdev:	platform device.
//
// Perform initialization of the EDAC memory controller instance and
// related driver-private data associated with the memory controller the
// instance is bound to.
//
#[no_mangle]
unsafe extern "C" fn mc_init(mci: *mut mem_ctl_info, pdev: *mut platform_device) {
    static void mc_init(struct mem_ctl_info *mci, struct platform_device *pdev)
    {
    struct synps_edac_priv *priv;
    mci.pdev = &pdev.dev;
    priv = mci.pvt_info;
    platform_set_drvdata(pdev, mci);
// Initialize controller capabilities and configuration
    mci.mtype_cap = MEM_FLAG_DDR3 | MEM_FLAG_DDR2;
    mci.edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED;
    mci.scrub_cap = SCRUB_HW_SRC;
    mci.scrub_mode = SCRUB_NONE;
    mci.edac_cap = EDAC_FLAG_SECDED;
    mci.ctl_name = "synps_ddr_controller";
    mci.dev_name = SYNPS_EDAC_MOD_STRING;
    mci.mod_name = SYNPS_EDAC_MOD_VER;
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT) {
    edac_op_state = EDAC_OPSTATE_INT;
    } else {
    edac_op_state = EDAC_OPSTATE_POLL;
    mci.edac_check = check_errors;
    }
    mci.ctl_page_to_phys = core::ptr::null_mut();
    init_csrows(mci);
    }
    static int setup_irq(struct mem_ctl_info *mci,
    struct platform_device *pdev)
    {
    struct synps_edac_priv *priv = mci.pvt_info;
    int ret, irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    edac_printk(KERN_ERR, EDAC_MC,
    "No IRQ %d in DT\n", irq);
    return irq;
    }
    ret = devm_request_irq(&pdev.dev, irq, intr_handler,
    0, dev_name(&pdev.dev), mci);
    if (ret < 0) {
    edac_printk(KERN_ERR, EDAC_MC, "Failed to request IRQ\n");
    return ret;
    }
    enable_intr(priv);
    return 0;
    }
    static const struct synps_platform_data zynq_edac_def = {
    .platform = ZYNQ,
    .get_error_info	= zynq_get_error_info,
    .get_mtype	= zynq_get_mtype,
    .get_dtype	= zynq_get_dtype,
    .quirks		= 0,
    };
    static const struct synps_platform_data zynqmp_edac_def = {
    .platform = ZYNQMP,
    .get_error_info	= zynqmp_get_error_info,
    .get_mtype	= zynqmp_get_mtype,
    .get_dtype	= zynqmp_get_dtype,

    .get_mem_info	= zynqmp_get_mem_info,

    .quirks         = (DDR_ECC_INTR_SUPPORT

    | DDR_ECC_DATA_POISON_SUPPORT

    ),
    };
    static const struct synps_platform_data synopsys_edac_def = {
    .platform = SYNPS,
    .get_error_info	= zynqmp_get_error_info,
    .get_mtype	= zynqmp_get_mtype,
    .get_dtype	= zynqmp_get_dtype,
    .quirks         = (DDR_ECC_INTR_SUPPORT | DDR_ECC_INTR_SELF_CLEAR

    | DDR_ECC_DATA_POISON_SUPPORT

    ),
    };
    static const struct of_device_id synps_edac_match[] = {
    {
    .compatible = "xlnx,zynq-ddrc-a05",
    .data = (void *)&zynq_edac_def
    },
    {
    .compatible = "xlnx,zynqmp-ddrc-2.40a",
    .data = (void *)&zynqmp_edac_def
    },
    {
    .compatible = "snps,ddrc-3.80a",
    .data = (void *)&synopsys_edac_def
    },
    {
// end of table
    }
    };
    MODULE_DEVICE_TABLE(of, synps_edac_match);

//
// ddr_poison_setup -	Update poison registers.
// @priv:		DDR memory controller private instance data.
//
// Update poison registers as per DDR mapping.
// Return: none.
//
#[no_mangle]
unsafe extern "C" fn ddr_poison_setup(priv: *mut synps_edac_priv) {
    static void ddr_poison_setup(struct synps_edac_priv *priv)
    {
    let mut col: c_int = 0, row = 0, bank = 0, bankgrp = 0, rank = 0, regval;
    const struct synps_platform_data *p_data;
    int index;
    let mut hif_addr: c_ulong = 0;
    p_data = priv.p_data;
    if (p_data.get_mem_info)
    hif_addr = p_data.get_mem_info(priv);
    else
    hif_addr = priv.poison_addr >> 3;
    for (index = 0; index < DDR_MAX_ROW_SHIFT; index++) {
    if (priv.row_shift[index])
    row |= (((hif_addr >> priv.row_shift[index]) &
    BIT(0)) << index);
    else
    break;
    }
    for (index = 0; index < DDR_MAX_COL_SHIFT; index++) {
    if (priv.col_shift[index] || index < 3)
    col |= (((hif_addr >> priv.col_shift[index]) &
    BIT(0)) << index);
    else
    break;
    }
    for (index = 0; index < DDR_MAX_BANK_SHIFT; index++) {
    if (priv.bank_shift[index])
    bank |= (((hif_addr >> priv.bank_shift[index]) &
    BIT(0)) << index);
    else
    break;
    }
    for (index = 0; index < DDR_MAX_BANKGRP_SHIFT; index++) {
    if (priv.bankgrp_shift[index])
    bankgrp |= (((hif_addr >> priv.bankgrp_shift[index])
    & BIT(0)) << index);
    else
    break;
    }
    if (priv.rank_shift[0])
    rank = (hif_addr >> priv.rank_shift[0]) & BIT(0);
    regval = (rank << ECC_POISON0_RANK_SHIFT) & ECC_POISON0_RANK_MASK;
    regval |= (col << ECC_POISON0_COLUMN_SHIFT) & ECC_POISON0_COLUMN_MASK;
    writel(regval, priv.baseaddr + ECC_POISON0_OFST);
    regval = (bankgrp << ECC_POISON1_BG_SHIFT) & ECC_POISON1_BG_MASK;
    regval |= (bank << ECC_POISON1_BANKNR_SHIFT) & ECC_POISON1_BANKNR_MASK;
    regval |= (row << ECC_POISON1_ROW_SHIFT) & ECC_POISON1_ROW_MASK;
    writel(regval, priv.baseaddr + ECC_POISON1_OFST);
    }
    static ssize_t inject_data_error_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct synps_edac_priv *priv = mci.pvt_info;
    return sprintf(data, "Poison0 Addr: 0x%08x\n\rPoison1 Addr: 0x%08x\n\r"
    "Error injection Address: 0x%lx\n\r",
    readl(priv.baseaddr + ECC_POISON0_OFST),
    readl(priv.baseaddr + ECC_POISON1_OFST),
    priv.poison_addr);
    }
    static ssize_t inject_data_error_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct synps_edac_priv *priv = mci.pvt_info;
    if (kstrtoul(data, 0, &priv.poison_addr))
    return -EINVAL;
    ddr_poison_setup(priv);
    return count;
    }
    static ssize_t inject_data_poison_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct synps_edac_priv *priv = mci.pvt_info;
    return sprintf(data, "Data Poisoning: %s\n\r",
    (((readl(priv.baseaddr + ECC_CFG1_OFST)) & 0x3) == 0x3)
    ? ("Correctable Error") : ("UnCorrectable Error"));
    }
    static ssize_t inject_data_poison_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct synps_edac_priv *priv = mci.pvt_info;
    writel(0, priv.baseaddr + DDRC_SWCTL);
    if (strncmp(data, "CE", 2) == 0)
    writel(ECC_CEPOISON_MASK, priv.baseaddr + ECC_CFG1_OFST);
    else
    writel(ECC_UEPOISON_MASK, priv.baseaddr + ECC_CFG1_OFST);
    writel(1, priv.baseaddr + DDRC_SWCTL);
    return count;
    }
    static DEVICE_ATTR_RW(inject_data_error);
    static DEVICE_ATTR_RW(inject_data_poison);
#[no_mangle]
unsafe extern "C" fn edac_create_sysfs_attributes(mci: *mut mem_ctl_info) -> c_int {
    static int edac_create_sysfs_attributes(struct mem_ctl_info *mci)
    {
    int rc;
    rc = device_create_file(&mci.dev, &dev_attr_inject_data_error);
    if (rc < 0)
    return rc;
    rc = device_create_file(&mci.dev, &dev_attr_inject_data_poison);
    if (rc < 0)
    return rc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn edac_remove_sysfs_attributes(mci: *mut mem_ctl_info) {
    static void edac_remove_sysfs_attributes(struct mem_ctl_info *mci)
    {
    device_remove_file(&mci.dev, &dev_attr_inject_data_error);
    device_remove_file(&mci.dev, &dev_attr_inject_data_poison);
    }
#[no_mangle]
unsafe extern "C" fn setup_row_address_map(priv: *mut synps_edac_priv, addrmap: *mut u32) {
    static void setup_row_address_map(struct synps_edac_priv *priv, u32 *addrmap)
    {
    u32 addrmap_row_b2_10;
    int index;
    priv.row_shift[0] = (addrmap[5] & ROW_MAX_VAL_MASK) + ROW_B0_BASE;
    priv.row_shift[1] = ((addrmap[5] >> 8) &
    ROW_MAX_VAL_MASK) + ROW_B1_BASE;
    addrmap_row_b2_10 = (addrmap[5] >> 16) & ROW_MAX_VAL_MASK;
    if (addrmap_row_b2_10 != ROW_MAX_VAL_MASK) {
    for (index = 2; index < 11; index++)
    priv.row_shift[index] = addrmap_row_b2_10 +
    index + ROW_B0_BASE;
    } else {
    priv.row_shift[2] = (addrmap[9] &
    ROW_MAX_VAL_MASK) + ROW_B2_BASE;
    priv.row_shift[3] = ((addrmap[9] >> 8) &
    ROW_MAX_VAL_MASK) + ROW_B3_BASE;
    priv.row_shift[4] = ((addrmap[9] >> 16) &
    ROW_MAX_VAL_MASK) + ROW_B4_BASE;
    priv.row_shift[5] = ((addrmap[9] >> 24) &
    ROW_MAX_VAL_MASK) + ROW_B5_BASE;
    priv.row_shift[6] = (addrmap[10] &
    ROW_MAX_VAL_MASK) + ROW_B6_BASE;
    priv.row_shift[7] = ((addrmap[10] >> 8) &
    ROW_MAX_VAL_MASK) + ROW_B7_BASE;
    priv.row_shift[8] = ((addrmap[10] >> 16) &
    ROW_MAX_VAL_MASK) + ROW_B8_BASE;
    priv.row_shift[9] = ((addrmap[10] >> 24) &
    ROW_MAX_VAL_MASK) + ROW_B9_BASE;
    priv.row_shift[10] = (addrmap[11] &
    ROW_MAX_VAL_MASK) + ROW_B10_BASE;
    }
    priv.row_shift[11] = (((addrmap[5] >> 24) & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : (((addrmap[5] >> 24) &
    ROW_MAX_VAL_MASK) + ROW_B11_BASE);
    priv.row_shift[12] = ((addrmap[6] & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : ((addrmap[6] &
    ROW_MAX_VAL_MASK) + ROW_B12_BASE);
    priv.row_shift[13] = (((addrmap[6] >> 8) & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : (((addrmap[6] >> 8) &
    ROW_MAX_VAL_MASK) + ROW_B13_BASE);
    priv.row_shift[14] = (((addrmap[6] >> 16) & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : (((addrmap[6] >> 16) &
    ROW_MAX_VAL_MASK) + ROW_B14_BASE);
    priv.row_shift[15] = (((addrmap[6] >> 24) & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : (((addrmap[6] >> 24) &
    ROW_MAX_VAL_MASK) + ROW_B15_BASE);
    priv.row_shift[16] = ((addrmap[7] & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : ((addrmap[7] &
    ROW_MAX_VAL_MASK) + ROW_B16_BASE);
    priv.row_shift[17] = (((addrmap[7] >> 8) & ROW_MAX_VAL_MASK) ==
    ROW_MAX_VAL_MASK) ? 0 : (((addrmap[7] >> 8) &
    ROW_MAX_VAL_MASK) + ROW_B17_BASE);
    }
#[no_mangle]
unsafe extern "C" fn setup_column_address_map(priv: *mut synps_edac_priv, addrmap: *mut u32) {
    static void setup_column_address_map(struct synps_edac_priv *priv, u32 *addrmap)
    {
    u32 width, memtype;
    int index;
    memtype = readl(priv.baseaddr + CTRL_OFST);
    width = (memtype & ECC_CTRL_BUSWIDTH_MASK) >> ECC_CTRL_BUSWIDTH_SHIFT;
    priv.col_shift[0] = 0;
    priv.col_shift[1] = 1;
    priv.col_shift[2] = (addrmap[2] & COL_MAX_VAL_MASK) + COL_B2_BASE;
    priv.col_shift[3] = ((addrmap[2] >> 8) &
    COL_MAX_VAL_MASK) + COL_B3_BASE;
    priv.col_shift[4] = (((addrmap[2] >> 16) & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : (((addrmap[2] >> 16) &
    COL_MAX_VAL_MASK) + COL_B4_BASE);
    priv.col_shift[5] = (((addrmap[2] >> 24) & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : (((addrmap[2] >> 24) &
    COL_MAX_VAL_MASK) + COL_B5_BASE);
    priv.col_shift[6] = ((addrmap[3] & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : ((addrmap[3] &
    COL_MAX_VAL_MASK) + COL_B6_BASE);
    priv.col_shift[7] = (((addrmap[3] >> 8) & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : (((addrmap[3] >> 8) &
    COL_MAX_VAL_MASK) + COL_B7_BASE);
    priv.col_shift[8] = (((addrmap[3] >> 16) & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : (((addrmap[3] >> 16) &
    COL_MAX_VAL_MASK) + COL_B8_BASE);
    priv.col_shift[9] = (((addrmap[3] >> 24) & COL_MAX_VAL_MASK) ==
    COL_MAX_VAL_MASK) ? 0 : (((addrmap[3] >> 24) &
    COL_MAX_VAL_MASK) + COL_B9_BASE);
    if (width == DDRCTL_EWDTH_64) {
    if (memtype & MEM_TYPE_LPDDR3) {
    priv.col_shift[10] = ((addrmap[4] &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    ((addrmap[4] & COL_MAX_VAL_MASK) +
    COL_B10_BASE);
    priv.col_shift[11] = (((addrmap[4] >> 8) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[4] >> 8) & COL_MAX_VAL_MASK) +
    COL_B11_BASE);
    } else {
    priv.col_shift[11] = ((addrmap[4] &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    ((addrmap[4] & COL_MAX_VAL_MASK) +
    COL_B10_BASE);
    priv.col_shift[13] = (((addrmap[4] >> 8) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[4] >> 8) & COL_MAX_VAL_MASK) +
    COL_B11_BASE);
    }
    } else if (width == DDRCTL_EWDTH_32) {
    if (memtype & MEM_TYPE_LPDDR3) {
    priv.col_shift[10] = (((addrmap[3] >> 24) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 24) & COL_MAX_VAL_MASK) +
    COL_B9_BASE);
    priv.col_shift[11] = ((addrmap[4] &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    ((addrmap[4] & COL_MAX_VAL_MASK) +
    COL_B10_BASE);
    } else {
    priv.col_shift[11] = (((addrmap[3] >> 24) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 24) & COL_MAX_VAL_MASK) +
    COL_B9_BASE);
    priv.col_shift[13] = ((addrmap[4] &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    ((addrmap[4] & COL_MAX_VAL_MASK) +
    COL_B10_BASE);
    }
    } else {
    if (memtype & MEM_TYPE_LPDDR3) {
    priv.col_shift[10] = (((addrmap[3] >> 16) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 16) & COL_MAX_VAL_MASK) +
    COL_B8_BASE);
    priv.col_shift[11] = (((addrmap[3] >> 24) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 24) & COL_MAX_VAL_MASK) +
    COL_B9_BASE);
    priv.col_shift[13] = ((addrmap[4] &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    ((addrmap[4] & COL_MAX_VAL_MASK) +
    COL_B10_BASE);
    } else {
    priv.col_shift[11] = (((addrmap[3] >> 16) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 16) & COL_MAX_VAL_MASK) +
    COL_B8_BASE);
    priv.col_shift[13] = (((addrmap[3] >> 24) &
    COL_MAX_VAL_MASK) == COL_MAX_VAL_MASK) ? 0 :
    (((addrmap[3] >> 24) & COL_MAX_VAL_MASK) +
    COL_B9_BASE);
    }
    }
    if (width) {
    for (index = 9; index > width; index--) {
    priv.col_shift[index] = priv.col_shift[index - width];
    priv.col_shift[index - width] = 0;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_bank_address_map(priv: *mut synps_edac_priv, addrmap: *mut u32) {
    static void setup_bank_address_map(struct synps_edac_priv *priv, u32 *addrmap)
    {
    priv.bank_shift[0] = (addrmap[1] & BANK_MAX_VAL_MASK) + BANK_B0_BASE;
    priv.bank_shift[1] = ((addrmap[1] >> 8) &
    BANK_MAX_VAL_MASK) + BANK_B1_BASE;
    priv.bank_shift[2] = (((addrmap[1] >> 16) &
    BANK_MAX_VAL_MASK) == BANK_MAX_VAL_MASK) ? 0 :
    (((addrmap[1] >> 16) & BANK_MAX_VAL_MASK) +
    BANK_B2_BASE);
    }
#[no_mangle]
unsafe extern "C" fn setup_bg_address_map(priv: *mut synps_edac_priv, addrmap: *mut u32) {
    static void setup_bg_address_map(struct synps_edac_priv *priv, u32 *addrmap)
    {
    priv.bankgrp_shift[0] = (addrmap[8] &
    BANKGRP_MAX_VAL_MASK) + BANKGRP_B0_BASE;
    priv.bankgrp_shift[1] = (((addrmap[8] >> 8) & BANKGRP_MAX_VAL_MASK) ==
    BANKGRP_MAX_VAL_MASK) ? 0 : (((addrmap[8] >> 8)
    & BANKGRP_MAX_VAL_MASK) + BANKGRP_B1_BASE);
    }
#[no_mangle]
unsafe extern "C" fn setup_rank_address_map(priv: *mut synps_edac_priv, addrmap: *mut u32) {
    static void setup_rank_address_map(struct synps_edac_priv *priv, u32 *addrmap)
    {
    priv.rank_shift[0] = ((addrmap[0] & RANK_MAX_VAL_MASK) ==
    RANK_MAX_VAL_MASK) ? 0 : ((addrmap[0] &
    RANK_MAX_VAL_MASK) + RANK_B0_BASE);
    }
//
// setup_address_map -	Set Address Map by querying ADDRMAP registers.
// @priv:		DDR memory controller private instance data.
//
// Set Address Map by querying ADDRMAP registers.
//
// Return: none.
//
#[no_mangle]
unsafe extern "C" fn setup_address_map(priv: *mut synps_edac_priv) {
    static void setup_address_map(struct synps_edac_priv *priv)
    {
    u32 addrmap[12];
    int index;
    for (index = 0; index < 12; index++) {
    u32 addrmap_offset;
    addrmap_offset = ECC_ADDRMAP0_OFFSET + (index * 4);
    addrmap[index] = readl(priv.baseaddr + addrmap_offset);
    }
    setup_row_address_map(priv, addrmap);
    setup_column_address_map(priv, addrmap);
    setup_bank_address_map(priv, addrmap);
    setup_bg_address_map(priv, addrmap);
    setup_rank_address_map(priv, addrmap);
    }

//
// mc_probe - Check controller and bind driver.
// @pdev:	platform device.
//
// Probe a specific controller instance for binding with the driver.
//
// Return: 0 if the controller instance was successfully bound to the
// driver; otherwise, < 0 on error.
//
#[no_mangle]
unsafe extern "C" fn mc_probe(pdev: *mut platform_device) -> c_int {
    static int mc_probe(struct platform_device *pdev)
    {
    const struct synps_platform_data *p_data;
    struct edac_mc_layer layers[2];
    struct synps_edac_priv *priv;
    struct mem_ctl_info *mci;
    void __iomem *baseaddr;
    int rc;
    baseaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(baseaddr))
    return PTR_ERR(baseaddr);
    p_data = of_device_get_match_data(&pdev.dev);
    if (!p_data)
    return -ENODEV;
    layers[0].type = EDAC_MC_LAYER_CHIP_SELECT;
    layers[0].size = SYNPS_EDAC_NR_CSROWS;
    layers[0].is_virt_csrow = true;
    layers[1].type = EDAC_MC_LAYER_CHANNEL;
    layers[1].size = SYNPS_EDAC_NR_CHANS;
    layers[1].is_virt_csrow = false;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers,
    sizeof(struct synps_edac_priv));
    if (!mci) {
    edac_printk(KERN_ERR, EDAC_MC,
    "Failed memory allocation for mc instance\n");
    return -ENOMEM;
    }
    priv = mci.pvt_info;
    priv.baseaddr = baseaddr;
    priv.p_data = p_data;
    if (!get_ecc_state(priv)) {
    edac_printk(KERN_INFO, EDAC_MC, "ECC not enabled\n");
    rc = -ENODEV;
    goto free_edac_mc;
    }
    spin_lock_init(&priv.reglock);
    mc_init(mci, pdev);
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT) {
    rc = setup_irq(mci, pdev);
    if (rc)
    goto free_edac_mc;
    }
    rc = edac_mc_add_mc(mci);
    if (rc) {
    edac_printk(KERN_ERR, EDAC_MC,
    "Failed to register with EDAC core\n");
    goto free_edac_mc;
    }

    if (priv.p_data.quirks & DDR_ECC_DATA_POISON_SUPPORT) {
    rc = edac_create_sysfs_attributes(mci);
    if (rc) {
    edac_printk(KERN_ERR, EDAC_MC,
    "Failed to create sysfs entries\n");
    goto free_edac_mc;
    }
    }
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT)
    setup_address_map(priv);

//
// Start capturing the correctable and uncorrectable errors. A write of
// 0 starts the counters.
//
    if (!(priv.p_data.quirks & DDR_ECC_INTR_SUPPORT))
    writel(0x0, baseaddr + ECC_CTRL_OFST);
    return rc;
    free_edac_mc:
    edac_mc_free(mci);
    return rc;
    }
//
// mc_remove - Unbind driver from controller.
// @pdev:	Platform device.
//
// Return: Unconditionally 0
//
#[no_mangle]
unsafe extern "C" fn mc_remove(pdev: *mut platform_device) {
    static void mc_remove(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci = platform_get_drvdata(pdev);
    struct synps_edac_priv *priv = mci.pvt_info;
    if (priv.p_data.quirks & DDR_ECC_INTR_SUPPORT)
    disable_intr(priv);

    if (priv.p_data.quirks & DDR_ECC_DATA_POISON_SUPPORT)
    edac_remove_sysfs_attributes(mci);

    edac_mc_del_mc(&pdev.dev);
    edac_mc_free(mci);
    }
    static struct platform_driver synps_edac_mc_driver = {
    .driver = {
    .name = "synopsys-edac",
    .of_match_table = synps_edac_match,
    },
    .probe = mc_probe,
    .remove = mc_remove,
    };
    module_platform_driver(synps_edac_mc_driver);
    MODULE_AUTHOR("Xilinx Inc");
    MODULE_DESCRIPTION("Synopsys DDR ECC driver");
    MODULE_LICENSE("GPL v2");
