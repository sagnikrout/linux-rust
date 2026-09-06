//! Automatically rewritten from C to Rust
//! Source: drivers/memory/tegra/tegra124-emc.c
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
// Copyright (c) 2014, NVIDIA CORPORATION.  All rights reserved.
//
// Author:
// Mikko Perttunen <mperttunen@nvidia.com>
//

pub const EMC_FBIO_CFG5: c_uint = 0x104;
pub const EMC_FBIO_CFG5_DRAM_TYPE_MASK: c_uint = 0x3;
pub const EMC_FBIO_CFG5_DRAM_TYPE_SHIFT: c_int = 0;

pub const EMC_INTSTATUS: c_uint = 0x0;

pub const EMC_CFG: c_uint = 0xc;

pub const EMC_REFCTRL: c_uint = 0x20;
pub const EMC_REFCTRL_DEV_SEL_SHIFT: c_int = 0;

pub const EMC_TIMING_CONTROL: c_uint = 0x28;
pub const EMC_RC: c_uint = 0x2c;
pub const EMC_RFC: c_uint = 0x30;
pub const EMC_RAS: c_uint = 0x34;
pub const EMC_RP: c_uint = 0x38;
pub const EMC_R2W: c_uint = 0x3c;
pub const EMC_W2R: c_uint = 0x40;
pub const EMC_R2P: c_uint = 0x44;
pub const EMC_W2P: c_uint = 0x48;
pub const EMC_RD_RCD: c_uint = 0x4c;
pub const EMC_WR_RCD: c_uint = 0x50;
pub const EMC_RRD: c_uint = 0x54;
pub const EMC_REXT: c_uint = 0x58;
pub const EMC_WDV: c_uint = 0x5c;
pub const EMC_QUSE: c_uint = 0x60;
pub const EMC_QRST: c_uint = 0x64;
pub const EMC_QSAFE: c_uint = 0x68;
pub const EMC_RDV: c_uint = 0x6c;
pub const EMC_REFRESH: c_uint = 0x70;
pub const EMC_BURST_REFRESH_NUM: c_uint = 0x74;
pub const EMC_PDEX2WR: c_uint = 0x78;
pub const EMC_PDEX2RD: c_uint = 0x7c;
pub const EMC_PCHG2PDEN: c_uint = 0x80;
pub const EMC_ACT2PDEN: c_uint = 0x84;
pub const EMC_AR2PDEN: c_uint = 0x88;
pub const EMC_RW2PDEN: c_uint = 0x8c;
pub const EMC_TXSR: c_uint = 0x90;
pub const EMC_TCKE: c_uint = 0x94;
pub const EMC_TFAW: c_uint = 0x98;
pub const EMC_TRPAB: c_uint = 0x9c;
pub const EMC_TCLKSTABLE: c_uint = 0xa0;
pub const EMC_TCLKSTOP: c_uint = 0xa4;
pub const EMC_TREFBW: c_uint = 0xa8;
pub const EMC_ODT_WRITE: c_uint = 0xb0;
pub const EMC_ODT_READ: c_uint = 0xb4;
pub const EMC_WEXT: c_uint = 0xb8;
pub const EMC_CTT: c_uint = 0xbc;
pub const EMC_RFC_SLR: c_uint = 0xc0;
pub const EMC_MRS_WAIT_CNT2: c_uint = 0xc4;
pub const EMC_MRS_WAIT_CNT: c_uint = 0xc8;
pub const EMC_MRS_WAIT_CNT_SHORT_WAIT_SHIFT: c_int = 0;

    (0x3FF << EMC_MRS_WAIT_CNT_SHORT_WAIT_SHIFT)
pub const EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT: c_int = 16;

    (0x3FF << EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT)
pub const EMC_MRS: c_uint = 0xcc;

pub const EMC_EMRS: c_uint = 0xd0;
pub const EMC_REF: c_uint = 0xd4;
pub const EMC_PRE: c_uint = 0xd8;
pub const EMC_SELF_REF: c_uint = 0xe0;

pub const EMC_SELF_REF_DEV_SEL_SHIFT: c_int = 30;
pub const EMC_MRW: c_uint = 0xe8;
pub const EMC_MRR: c_uint = 0xec;
pub const EMC_MRR_MA_SHIFT: c_int = 16;
pub const LPDDR2_MR4_TEMP_SHIFT: c_int = 0;
pub const EMC_XM2DQSPADCTRL3: c_uint = 0xf8;
pub const EMC_FBIO_SPARE: c_uint = 0x100;
pub const EMC_FBIO_CFG6: c_uint = 0x114;
pub const EMC_EMRS2: c_uint = 0x12c;
pub const EMC_MRW2: c_uint = 0x134;
pub const EMC_MRW4: c_uint = 0x13c;
pub const EMC_EINPUT: c_uint = 0x14c;
pub const EMC_EINPUT_DURATION: c_uint = 0x150;
pub const EMC_PUTERM_EXTRA: c_uint = 0x154;
pub const EMC_TCKESR: c_uint = 0x158;
pub const EMC_TPD: c_uint = 0x15c;
pub const EMC_AUTO_CAL_CONFIG: c_uint = 0x2a4;

pub const EMC_AUTO_CAL_INTERVAL: c_uint = 0x2a8;
pub const EMC_AUTO_CAL_STATUS: c_uint = 0x2ac;

pub const EMC_STATUS: c_uint = 0x2b4;

pub const EMC_CFG_2: c_uint = 0x2b8;
pub const EMC_CFG_2_MODE_SHIFT: c_int = 0;

pub const EMC_CFG_DIG_DLL: c_uint = 0x2bc;
pub const EMC_CFG_DIG_DLL_PERIOD: c_uint = 0x2c0;
pub const EMC_RDV_MASK: c_uint = 0x2cc;
pub const EMC_WDV_MASK: c_uint = 0x2d0;
pub const EMC_CTT_DURATION: c_uint = 0x2d8;
pub const EMC_CTT_TERM_CTRL: c_uint = 0x2dc;
pub const EMC_ZCAL_INTERVAL: c_uint = 0x2e0;
pub const EMC_ZCAL_WAIT_CNT: c_uint = 0x2e4;
pub const EMC_ZQ_CAL: c_uint = 0x2ec;

    (DRAM_DEV_SEL_0 | EMC_ZQ_CAL_LONG | EMC_ZQ_CAL_CMD)

    (DRAM_DEV_SEL_1 | EMC_ZQ_CAL_LONG | EMC_ZQ_CAL_CMD)
pub const EMC_XM2CMDPADCTRL: c_uint = 0x2f0;
pub const EMC_XM2DQSPADCTRL: c_uint = 0x2f8;
pub const EMC_XM2DQSPADCTRL2: c_uint = 0x2fc;

pub const EMC_XM2DQPADCTRL: c_uint = 0x300;
pub const EMC_XM2DQPADCTRL2: c_uint = 0x304;
pub const EMC_XM2CLKPADCTRL: c_uint = 0x308;
pub const EMC_XM2COMPPADCTRL: c_uint = 0x30c;
pub const EMC_XM2VTTGENPADCTRL: c_uint = 0x310;
pub const EMC_XM2VTTGENPADCTRL2: c_uint = 0x314;
pub const EMC_XM2VTTGENPADCTRL3: c_uint = 0x318;
pub const EMC_XM2DQSPADCTRL4: c_uint = 0x320;
pub const EMC_DLL_XFORM_DQS0: c_uint = 0x328;
pub const EMC_DLL_XFORM_DQS1: c_uint = 0x32c;
pub const EMC_DLL_XFORM_DQS2: c_uint = 0x330;
pub const EMC_DLL_XFORM_DQS3: c_uint = 0x334;
pub const EMC_DLL_XFORM_DQS4: c_uint = 0x338;
pub const EMC_DLL_XFORM_DQS5: c_uint = 0x33c;
pub const EMC_DLL_XFORM_DQS6: c_uint = 0x340;
pub const EMC_DLL_XFORM_DQS7: c_uint = 0x344;
pub const EMC_DLL_XFORM_QUSE0: c_uint = 0x348;
pub const EMC_DLL_XFORM_QUSE1: c_uint = 0x34c;
pub const EMC_DLL_XFORM_QUSE2: c_uint = 0x350;
pub const EMC_DLL_XFORM_QUSE3: c_uint = 0x354;
pub const EMC_DLL_XFORM_QUSE4: c_uint = 0x358;
pub const EMC_DLL_XFORM_QUSE5: c_uint = 0x35c;
pub const EMC_DLL_XFORM_QUSE6: c_uint = 0x360;
pub const EMC_DLL_XFORM_QUSE7: c_uint = 0x364;
pub const EMC_DLL_XFORM_DQ0: c_uint = 0x368;
pub const EMC_DLL_XFORM_DQ1: c_uint = 0x36c;
pub const EMC_DLL_XFORM_DQ2: c_uint = 0x370;
pub const EMC_DLL_XFORM_DQ3: c_uint = 0x374;
pub const EMC_DLI_TRIM_TXDQS0: c_uint = 0x3a8;
pub const EMC_DLI_TRIM_TXDQS1: c_uint = 0x3ac;
pub const EMC_DLI_TRIM_TXDQS2: c_uint = 0x3b0;
pub const EMC_DLI_TRIM_TXDQS3: c_uint = 0x3b4;
pub const EMC_DLI_TRIM_TXDQS4: c_uint = 0x3b8;
pub const EMC_DLI_TRIM_TXDQS5: c_uint = 0x3bc;
pub const EMC_DLI_TRIM_TXDQS6: c_uint = 0x3c0;
pub const EMC_DLI_TRIM_TXDQS7: c_uint = 0x3c4;
pub const EMC_STALL_THEN_EXE_AFTER_CLKCHANGE: c_uint = 0x3cc;
pub const EMC_SEL_DPD_CTRL: c_uint = 0x3d8;

    ((0xf << 2) | BIT(8))

    ((0x3 << 2) | BIT(5) | BIT(8))
pub const EMC_PRE_REFRESH_REQ_CNT: c_uint = 0x3dc;
pub const EMC_DYN_SELF_REF_CONTROL: c_uint = 0x3e0;
pub const EMC_TXSRDLL: c_uint = 0x3e4;
pub const EMC_CCFIFO_ADDR: c_uint = 0x3e8;
pub const EMC_CCFIFO_DATA: c_uint = 0x3ec;
pub const EMC_CCFIFO_STATUS: c_uint = 0x3f0;
pub const EMC_CDB_CNTL_1: c_uint = 0x3f4;
pub const EMC_CDB_CNTL_2: c_uint = 0x3f8;
pub const EMC_XM2CLKPADCTRL2: c_uint = 0x3fc;
pub const EMC_AUTO_CAL_CONFIG2: c_uint = 0x458;
pub const EMC_AUTO_CAL_CONFIG3: c_uint = 0x45c;
pub const EMC_IBDLY: c_uint = 0x468;
pub const EMC_DLL_XFORM_ADDR0: c_uint = 0x46c;
pub const EMC_DLL_XFORM_ADDR1: c_uint = 0x470;
pub const EMC_DLL_XFORM_ADDR2: c_uint = 0x474;
pub const EMC_DSR_VTTGEN_DRV: c_uint = 0x47c;
pub const EMC_TXDSRVTTGEN: c_uint = 0x480;
pub const EMC_XM2CMDPADCTRL4: c_uint = 0x484;
pub const EMC_XM2CMDPADCTRL5: c_uint = 0x488;
pub const EMC_DLL_XFORM_DQS8: c_uint = 0x4a0;
pub const EMC_DLL_XFORM_DQS9: c_uint = 0x4a4;
pub const EMC_DLL_XFORM_DQS10: c_uint = 0x4a8;
pub const EMC_DLL_XFORM_DQS11: c_uint = 0x4ac;
pub const EMC_DLL_XFORM_DQS12: c_uint = 0x4b0;
pub const EMC_DLL_XFORM_DQS13: c_uint = 0x4b4;
pub const EMC_DLL_XFORM_DQS14: c_uint = 0x4b8;
pub const EMC_DLL_XFORM_DQS15: c_uint = 0x4bc;
pub const EMC_DLL_XFORM_QUSE8: c_uint = 0x4c0;
pub const EMC_DLL_XFORM_QUSE9: c_uint = 0x4c4;
pub const EMC_DLL_XFORM_QUSE10: c_uint = 0x4c8;
pub const EMC_DLL_XFORM_QUSE11: c_uint = 0x4cc;
pub const EMC_DLL_XFORM_QUSE12: c_uint = 0x4d0;
pub const EMC_DLL_XFORM_QUSE13: c_uint = 0x4d4;
pub const EMC_DLL_XFORM_QUSE14: c_uint = 0x4d8;
pub const EMC_DLL_XFORM_QUSE15: c_uint = 0x4dc;
pub const EMC_DLL_XFORM_DQ4: c_uint = 0x4e0;
pub const EMC_DLL_XFORM_DQ5: c_uint = 0x4e4;
pub const EMC_DLL_XFORM_DQ6: c_uint = 0x4e8;
pub const EMC_DLL_XFORM_DQ7: c_uint = 0x4ec;
pub const EMC_DLI_TRIM_TXDQS8: c_uint = 0x520;
pub const EMC_DLI_TRIM_TXDQS9: c_uint = 0x524;
pub const EMC_DLI_TRIM_TXDQS10: c_uint = 0x528;
pub const EMC_DLI_TRIM_TXDQS11: c_uint = 0x52c;
pub const EMC_DLI_TRIM_TXDQS12: c_uint = 0x530;
pub const EMC_DLI_TRIM_TXDQS13: c_uint = 0x534;
pub const EMC_DLI_TRIM_TXDQS14: c_uint = 0x538;
pub const EMC_DLI_TRIM_TXDQS15: c_uint = 0x53c;
pub const EMC_CDB_CNTL_3: c_uint = 0x540;
pub const EMC_XM2DQSPADCTRL5: c_uint = 0x544;
pub const EMC_XM2DQSPADCTRL6: c_uint = 0x548;
pub const EMC_XM2DQPADCTRL3: c_uint = 0x54c;
pub const EMC_DLL_XFORM_ADDR3: c_uint = 0x550;
pub const EMC_DLL_XFORM_ADDR4: c_uint = 0x554;
pub const EMC_DLL_XFORM_ADDR5: c_uint = 0x558;
pub const EMC_CFG_PIPE: c_uint = 0x560;
pub const EMC_QPOP: c_uint = 0x564;
pub const EMC_QUSE_WIDTH: c_uint = 0x568;
pub const EMC_PUTERM_WIDTH: c_uint = 0x56c;
pub const EMC_BGBIAS_CTL0: c_uint = 0x570;

pub const EMC_PUTERM_ADJ: c_uint = 0x574;
pub const DRAM_DEV_SEL_ALL: c_int = 0;

    (EMC_CFG_DYN_SREF | EMC_CFG_DRAM_ACPD | EMC_CFG_DRAM_CLKSTOP_SR | \
    EMC_CFG_DRAM_CLKSTOP_PD | EMC_CFG_DSR_VTTGEN_DRV_EN)

// Maximum amount of time in us. to wait for changes to become effective
pub const EMC_STATUS_UPDATE_TIMEOUT: c_int = 1000;
    enum emc_dram_type {
    DRAM_TYPE_DDR3 = 0,
    DRAM_TYPE_DDR1 = 1,
    DRAM_TYPE_LPDDR3 = 2,
    DRAM_TYPE_DDR2 = 3
    };
    enum emc_dll_change {
    DLL_CHANGE_NONE,
    DLL_CHANGE_ON,
    DLL_CHANGE_OFF
    };
    static const unsigned long emc_burst_regs[] = {
    EMC_RC,
    EMC_RFC,
    EMC_RFC_SLR,
    EMC_RAS,
    EMC_RP,
    EMC_R2W,
    EMC_W2R,
    EMC_R2P,
    EMC_W2P,
    EMC_RD_RCD,
    EMC_WR_RCD,
    EMC_RRD,
    EMC_REXT,
    EMC_WEXT,
    EMC_WDV,
    EMC_WDV_MASK,
    EMC_QUSE,
    EMC_QUSE_WIDTH,
    EMC_IBDLY,
    EMC_EINPUT,
    EMC_EINPUT_DURATION,
    EMC_PUTERM_EXTRA,
    EMC_PUTERM_WIDTH,
    EMC_PUTERM_ADJ,
    EMC_CDB_CNTL_1,
    EMC_CDB_CNTL_2,
    EMC_CDB_CNTL_3,
    EMC_QRST,
    EMC_QSAFE,
    EMC_RDV,
    EMC_RDV_MASK,
    EMC_REFRESH,
    EMC_BURST_REFRESH_NUM,
    EMC_PRE_REFRESH_REQ_CNT,
    EMC_PDEX2WR,
    EMC_PDEX2RD,
    EMC_PCHG2PDEN,
    EMC_ACT2PDEN,
    EMC_AR2PDEN,
    EMC_RW2PDEN,
    EMC_TXSR,
    EMC_TXSRDLL,
    EMC_TCKE,
    EMC_TCKESR,
    EMC_TPD,
    EMC_TFAW,
    EMC_TRPAB,
    EMC_TCLKSTABLE,
    EMC_TCLKSTOP,
    EMC_TREFBW,
    EMC_FBIO_CFG6,
    EMC_ODT_WRITE,
    EMC_ODT_READ,
    EMC_FBIO_CFG5,
    EMC_CFG_DIG_DLL,
    EMC_CFG_DIG_DLL_PERIOD,
    EMC_DLL_XFORM_DQS0,
    EMC_DLL_XFORM_DQS1,
    EMC_DLL_XFORM_DQS2,
    EMC_DLL_XFORM_DQS3,
    EMC_DLL_XFORM_DQS4,
    EMC_DLL_XFORM_DQS5,
    EMC_DLL_XFORM_DQS6,
    EMC_DLL_XFORM_DQS7,
    EMC_DLL_XFORM_DQS8,
    EMC_DLL_XFORM_DQS9,
    EMC_DLL_XFORM_DQS10,
    EMC_DLL_XFORM_DQS11,
    EMC_DLL_XFORM_DQS12,
    EMC_DLL_XFORM_DQS13,
    EMC_DLL_XFORM_DQS14,
    EMC_DLL_XFORM_DQS15,
    EMC_DLL_XFORM_QUSE0,
    EMC_DLL_XFORM_QUSE1,
    EMC_DLL_XFORM_QUSE2,
    EMC_DLL_XFORM_QUSE3,
    EMC_DLL_XFORM_QUSE4,
    EMC_DLL_XFORM_QUSE5,
    EMC_DLL_XFORM_QUSE6,
    EMC_DLL_XFORM_QUSE7,
    EMC_DLL_XFORM_ADDR0,
    EMC_DLL_XFORM_ADDR1,
    EMC_DLL_XFORM_ADDR2,
    EMC_DLL_XFORM_ADDR3,
    EMC_DLL_XFORM_ADDR4,
    EMC_DLL_XFORM_ADDR5,
    EMC_DLL_XFORM_QUSE8,
    EMC_DLL_XFORM_QUSE9,
    EMC_DLL_XFORM_QUSE10,
    EMC_DLL_XFORM_QUSE11,
    EMC_DLL_XFORM_QUSE12,
    EMC_DLL_XFORM_QUSE13,
    EMC_DLL_XFORM_QUSE14,
    EMC_DLL_XFORM_QUSE15,
    EMC_DLI_TRIM_TXDQS0,
    EMC_DLI_TRIM_TXDQS1,
    EMC_DLI_TRIM_TXDQS2,
    EMC_DLI_TRIM_TXDQS3,
    EMC_DLI_TRIM_TXDQS4,
    EMC_DLI_TRIM_TXDQS5,
    EMC_DLI_TRIM_TXDQS6,
    EMC_DLI_TRIM_TXDQS7,
    EMC_DLI_TRIM_TXDQS8,
    EMC_DLI_TRIM_TXDQS9,
    EMC_DLI_TRIM_TXDQS10,
    EMC_DLI_TRIM_TXDQS11,
    EMC_DLI_TRIM_TXDQS12,
    EMC_DLI_TRIM_TXDQS13,
    EMC_DLI_TRIM_TXDQS14,
    EMC_DLI_TRIM_TXDQS15,
    EMC_DLL_XFORM_DQ0,
    EMC_DLL_XFORM_DQ1,
    EMC_DLL_XFORM_DQ2,
    EMC_DLL_XFORM_DQ3,
    EMC_DLL_XFORM_DQ4,
    EMC_DLL_XFORM_DQ5,
    EMC_DLL_XFORM_DQ6,
    EMC_DLL_XFORM_DQ7,
    EMC_XM2CMDPADCTRL,
    EMC_XM2CMDPADCTRL4,
    EMC_XM2CMDPADCTRL5,
    EMC_XM2DQPADCTRL2,
    EMC_XM2DQPADCTRL3,
    EMC_XM2CLKPADCTRL,
    EMC_XM2CLKPADCTRL2,
    EMC_XM2COMPPADCTRL,
    EMC_XM2VTTGENPADCTRL,
    EMC_XM2VTTGENPADCTRL2,
    EMC_XM2VTTGENPADCTRL3,
    EMC_XM2DQSPADCTRL3,
    EMC_XM2DQSPADCTRL4,
    EMC_XM2DQSPADCTRL5,
    EMC_XM2DQSPADCTRL6,
    EMC_DSR_VTTGEN_DRV,
    EMC_TXDSRVTTGEN,
    EMC_FBIO_SPARE,
    EMC_ZCAL_WAIT_CNT,
    EMC_MRS_WAIT_CNT2,
    EMC_CTT,
    EMC_CTT_DURATION,
    EMC_CFG_PIPE,
    EMC_DYN_SELF_REF_CONTROL,
    EMC_QPOP
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emc_timing {
    pub rate: c_ulong,
    pub emc_burst_data: [u32; ARRAY_SIZE(emc_burst_regs)],
    pub emc_auto_cal_config: u32,
    pub emc_auto_cal_config2: u32,
    pub emc_auto_cal_config3: u32,
    pub emc_auto_cal_interval: u32,
    pub emc_bgbias_ctl0: u32,
    pub emc_cfg: u32,
    pub emc_cfg_2: u32,
    pub emc_ctt_term_ctrl: u32,
    pub emc_mode_1: u32,
    pub emc_mode_2: u32,
    pub emc_mode_4: u32,
    pub emc_mode_reset: u32,
    pub emc_mrs_wait_cnt: u32,
    pub emc_sel_dpd_ctrl: u32,
    pub emc_xm2dqspadctrl2: u32,
    pub emc_zcal_cnt_long: u32,
    pub emc_zcal_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_emc {
    pub dev: *mut device,
    pub mc: *mut tegra_mc,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub dram_type: enum emc_dram_type,
    pub dram_bus_width: c_uint,
    pub dram_num: c_uint,
    pub last_timing: emc_timing,
    pub timings: *mut emc_timing,
    pub num_timings: c_uint,
    struct {
    pub root: *mut dentry,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub debugfs: },
    pub provider: icc_provider,
    pub reqs: tegra_emc_rate_requests,
}

// Timing change sequence functions
    static void emc_ccfifo_writel(struct tegra_emc *emc, u32 value,
    unsigned long offset)
    {
    writel(value, emc.regs + EMC_CCFIFO_DATA);
    writel(offset, emc.regs + EMC_CCFIFO_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn emc_seq_update_timing(emc: *mut tegra_emc) {
    static void emc_seq_update_timing(struct tegra_emc *emc)
    {
    unsigned int i;
    u32 value;
    writel(1, emc.regs + EMC_TIMING_CONTROL);
    for (i = 0; i < EMC_STATUS_UPDATE_TIMEOUT; ++i) {
    value = readl(emc.regs + EMC_STATUS);
    if ((value & EMC_STATUS_TIMING_UPDATE_STALLED) == 0)
    return;
    udelay(1);
    }
    dev_err(emc.dev, "timing update timed out\n");
    }
#[no_mangle]
unsafe extern "C" fn emc_seq_disable_auto_cal(emc: *mut tegra_emc) {
    static void emc_seq_disable_auto_cal(struct tegra_emc *emc)
    {
    unsigned int i;
    u32 value;
    writel(0, emc.regs + EMC_AUTO_CAL_INTERVAL);
    for (i = 0; i < EMC_STATUS_UPDATE_TIMEOUT; ++i) {
    value = readl(emc.regs + EMC_AUTO_CAL_STATUS);
    if ((value & EMC_AUTO_CAL_STATUS_ACTIVE) == 0)
    return;
    udelay(1);
    }
    dev_err(emc.dev, "auto cal disable timed out\n");
    }
#[no_mangle]
unsafe extern "C" fn emc_seq_wait_clkchange(emc: *mut tegra_emc) {
    static void emc_seq_wait_clkchange(struct tegra_emc *emc)
    {
    unsigned int i;
    u32 value;
    for (i = 0; i < EMC_STATUS_UPDATE_TIMEOUT; ++i) {
    value = readl(emc.regs + EMC_INTSTATUS);
    if (value & EMC_INTSTATUS_CLKCHANGE_COMPLETE)
    return;
    udelay(1);
    }
    dev_err(emc.dev, "clock change timed out\n");
    }
    static struct emc_timing *tegra124_emc_find_timing(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = core::ptr::null_mut();
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate == rate) {
    timing = &emc.timings[i];
    break;
    }
    }
    if (!timing) {
    dev_err(emc.dev, "no timing for rate %lu\n", rate);
    return core::ptr::null_mut();
    }
    return timing;
    }
    static int tegra124_emc_prepare_timing_change(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = tegra124_emc_find_timing(emc, rate);
    struct emc_timing *last = &emc.last_timing;
    enum emc_dll_change dll_change;
    let mut pre_wait: c_uint = 0;
    u32 val, val2, mask;
    let mut update: bool = false;
    unsigned int i;
    if (!timing)
    return -ENOENT;
    if ((last.emc_mode_1 & 0x1) == (timing.emc_mode_1 & 0x1))
    dll_change = DLL_CHANGE_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(0x1): !(timing->emc_mode_1 &) -> else {
    else if (!(timing.emc_mode_1 & 0x1))
    dll_change = DLL_CHANGE_ON;
    else
    dll_change = DLL_CHANGE_OFF;
// Clear CLKCHANGE_COMPLETE interrupts
    writel(EMC_INTSTATUS_CLKCHANGE_COMPLETE, emc.regs + EMC_INTSTATUS);
// Disable dynamic self-refresh
    val = readl(emc.regs + EMC_CFG);
    if (val & EMC_CFG_PWR_MASK) {
    val &= ~EMC_CFG_POWER_FEATURES_MASK;
    writel(val, emc.regs + EMC_CFG);
    pre_wait = 5;
    }
// Disable SEL_DPD_CTRL for clock change
    if (emc.dram_type == DRAM_TYPE_DDR3)
    mask = EMC_SEL_DPD_CTRL_DDR3_MASK;
    else
    mask = EMC_SEL_DPD_CTRL_MASK;
    val = readl(emc.regs + EMC_SEL_DPD_CTRL);
    if (val & mask) {
    val &= ~mask;
    writel(val, emc.regs + EMC_SEL_DPD_CTRL);
    }
// Prepare DQ/DQS for clock change
    val = readl(emc.regs + EMC_BGBIAS_CTL0);
    val2 = last.emc_bgbias_ctl0;
    if (!(timing.emc_bgbias_ctl0 &
    EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_RX) &&
    (val & EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_RX)) {
    val2 &= ~EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_RX;
    update = true;
    }
    if ((val & EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD) ||
    (val & EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_VTTGEN)) {
    update = true;
    }
    if (update) {
    writel(val2, emc.regs + EMC_BGBIAS_CTL0);
    if (pre_wait < 5)
    pre_wait = 5;
    }
    update = false;
    val = readl(emc.regs + EMC_XM2DQSPADCTRL2);
    if (timing.emc_xm2dqspadctrl2 & EMC_XM2DQSPADCTRL2_VREF_ENABLE &&
    !(val & EMC_XM2DQSPADCTRL2_VREF_ENABLE)) {
    val |= EMC_XM2DQSPADCTRL2_VREF_ENABLE;
    update = true;
    }
    if (timing.emc_xm2dqspadctrl2 & EMC_XM2DQSPADCTRL2_RX_FT_REC_ENABLE &&
    !(val & EMC_XM2DQSPADCTRL2_RX_FT_REC_ENABLE)) {
    val |= EMC_XM2DQSPADCTRL2_RX_FT_REC_ENABLE;
    update = true;
    }
    if (update) {
    writel(val, emc.regs + EMC_XM2DQSPADCTRL2);
    if (pre_wait < 30)
    pre_wait = 30;
    }
// Wait to settle
    if (pre_wait) {
    emc_seq_update_timing(emc);
    udelay(pre_wait);
    }
// Program CTT_TERM control
    if (last.emc_ctt_term_ctrl != timing.emc_ctt_term_ctrl) {
    emc_seq_disable_auto_cal(emc);
    writel(timing.emc_ctt_term_ctrl,
    emc.regs + EMC_CTT_TERM_CTRL);
    emc_seq_update_timing(emc);
    }
// Program burst shadow registers
    for (i = 0; i < ARRAY_SIZE(timing.emc_burst_data); ++i)
    writel(timing.emc_burst_data[i],
    emc.regs + emc_burst_regs[i]);
    writel(timing.emc_xm2dqspadctrl2, emc.regs + EMC_XM2DQSPADCTRL2);
    writel(timing.emc_zcal_interval, emc.regs + EMC_ZCAL_INTERVAL);
    tegra_mc_write_emem_configuration(emc.mc, timing.rate);
    val = timing.emc_cfg & ~EMC_CFG_POWER_FEATURES_MASK;
    emc_ccfifo_writel(emc, val, EMC_CFG);
// Program AUTO_CAL_CONFIG
    if (timing.emc_auto_cal_config2 != last.emc_auto_cal_config2)
    emc_ccfifo_writel(emc, timing.emc_auto_cal_config2,
    EMC_AUTO_CAL_CONFIG2);
    if (timing.emc_auto_cal_config3 != last.emc_auto_cal_config3)
    emc_ccfifo_writel(emc, timing.emc_auto_cal_config3,
    EMC_AUTO_CAL_CONFIG3);
    if (timing.emc_auto_cal_config != last.emc_auto_cal_config) {
    val = timing.emc_auto_cal_config;
    val &= EMC_AUTO_CAL_CONFIG_AUTO_CAL_START;
    emc_ccfifo_writel(emc, val, EMC_AUTO_CAL_CONFIG);
    }
// DDR3: predict MRS long wait count
    if (emc.dram_type == DRAM_TYPE_DDR3 &&
    dll_change == DLL_CHANGE_ON) {
    let mut cnt: u32 = 512;
    if (timing.emc_zcal_interval != 0 &&
    last.emc_zcal_interval == 0)
    cnt -= emc.dram_num * 256;
    val = (timing.emc_mrs_wait_cnt
    & EMC_MRS_WAIT_CNT_SHORT_WAIT_MASK)
    >> EMC_MRS_WAIT_CNT_SHORT_WAIT_SHIFT;
    if (cnt < val)
    cnt = val;
    val = timing.emc_mrs_wait_cnt
    & ~EMC_MRS_WAIT_CNT_LONG_WAIT_MASK;
    val |= (cnt << EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT)
    & EMC_MRS_WAIT_CNT_LONG_WAIT_MASK;
    writel(val, emc.regs + EMC_MRS_WAIT_CNT);
    }
    val = timing.emc_cfg_2;
    val &= ~EMC_CFG_2_DIS_STP_OB_CLK_DURING_NON_WR;
    emc_ccfifo_writel(emc, val, EMC_CFG_2);
// DDR3: Turn off DLL and enter self-refresh
    if (emc.dram_type == DRAM_TYPE_DDR3 && dll_change == DLL_CHANGE_OFF)
    emc_ccfifo_writel(emc, timing.emc_mode_1, EMC_EMRS);
// Disable refresh controller
    emc_ccfifo_writel(emc, EMC_REFCTRL_DEV_SEL(emc.dram_num),
    EMC_REFCTRL);
    if (emc.dram_type == DRAM_TYPE_DDR3)
    emc_ccfifo_writel(emc, EMC_DRAM_DEV_SEL(emc.dram_num) |
    EMC_SELF_REF_CMD_ENABLED,
    EMC_SELF_REF);
// Flow control marker
    emc_ccfifo_writel(emc, 1, EMC_STALL_THEN_EXE_AFTER_CLKCHANGE);
// DDR3: Exit self-refresh
    if (emc.dram_type == DRAM_TYPE_DDR3)
    emc_ccfifo_writel(emc, EMC_DRAM_DEV_SEL(emc.dram_num),
    EMC_SELF_REF);
    emc_ccfifo_writel(emc, EMC_REFCTRL_DEV_SEL(emc.dram_num) |
    EMC_REFCTRL_ENABLE,
    EMC_REFCTRL);
// Set DRAM mode registers
    if (emc.dram_type == DRAM_TYPE_DDR3) {
    if (timing.emc_mode_1 != last.emc_mode_1)
    emc_ccfifo_writel(emc, timing.emc_mode_1, EMC_EMRS);
    if (timing.emc_mode_2 != last.emc_mode_2)
    emc_ccfifo_writel(emc, timing.emc_mode_2, EMC_EMRS2);
    if ((timing.emc_mode_reset != last.emc_mode_reset) ||
    dll_change == DLL_CHANGE_ON) {
    val = timing.emc_mode_reset;
    if (dll_change == DLL_CHANGE_ON) {
    val |= EMC_MODE_SET_DLL_RESET;
    val |= EMC_MODE_SET_LONG_CNT;
    } else {
    val &= ~EMC_MODE_SET_DLL_RESET;
    }
    emc_ccfifo_writel(emc, val, EMC_MRS);
    }
    } else {
    if (timing.emc_mode_2 != last.emc_mode_2)
    emc_ccfifo_writel(emc, timing.emc_mode_2, EMC_MRW2);
    if (timing.emc_mode_1 != last.emc_mode_1)
    emc_ccfifo_writel(emc, timing.emc_mode_1, EMC_MRW);
    if (timing.emc_mode_4 != last.emc_mode_4)
    emc_ccfifo_writel(emc, timing.emc_mode_4, EMC_MRW4);
    }
// Issue ZCAL command if turning ZCAL on
    if (timing.emc_zcal_interval != 0 && last.emc_zcal_interval == 0) {
    emc_ccfifo_writel(emc, EMC_ZQ_CAL_LONG_CMD_DEV0, EMC_ZQ_CAL);
    if (emc.dram_num > 1)
    emc_ccfifo_writel(emc, EMC_ZQ_CAL_LONG_CMD_DEV1,
    EMC_ZQ_CAL);
    }
// Write to RO register to remove stall after change
    emc_ccfifo_writel(emc, 0, EMC_CCFIFO_STATUS);
    if (timing.emc_cfg_2 & EMC_CFG_2_DIS_STP_OB_CLK_DURING_NON_WR)
    emc_ccfifo_writel(emc, timing.emc_cfg_2, EMC_CFG_2);
// Disable AUTO_CAL for clock change
    emc_seq_disable_auto_cal(emc);
// Read register to wait until programming has settled
    readl(emc.regs + EMC_INTSTATUS);
    return 0;
    }
    static void tegra124_emc_complete_timing_change(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = tegra124_emc_find_timing(emc, rate);
    struct emc_timing *last = &emc.last_timing;
    u32 val;
    if (!timing)
    return;
// Wait until the state machine has settled
    emc_seq_wait_clkchange(emc);
// Restore AUTO_CAL
    if (timing.emc_ctt_term_ctrl != last.emc_ctt_term_ctrl)
    writel(timing.emc_auto_cal_interval,
    emc.regs + EMC_AUTO_CAL_INTERVAL);
// Restore dynamic self-refresh
    if (timing.emc_cfg & EMC_CFG_PWR_MASK)
    writel(timing.emc_cfg, emc.regs + EMC_CFG);
// Set ZCAL wait count
    writel(timing.emc_zcal_cnt_long, emc.regs + EMC_ZCAL_WAIT_CNT);
// LPDDR3: Turn off BGBIAS if low frequency
    if (emc.dram_type == DRAM_TYPE_LPDDR3 &&
    timing.emc_bgbias_ctl0 &
    EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_RX) {
    val = timing.emc_bgbias_ctl0;
    val |= EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD_IBIAS_VTTGEN;
    val |= EMC_BGBIAS_CTL0_BIAS0_DSC_E_PWRD;
    writel(val, emc.regs + EMC_BGBIAS_CTL0);
    } else {
    if (emc.dram_type == DRAM_TYPE_DDR3 &&
    readl(emc.regs + EMC_BGBIAS_CTL0) !=
    timing.emc_bgbias_ctl0) {
    writel(timing.emc_bgbias_ctl0,
    emc.regs + EMC_BGBIAS_CTL0);
    }
    writel(timing.emc_auto_cal_interval,
    emc.regs + EMC_AUTO_CAL_INTERVAL);
    }
// Wait for timing to settle
    udelay(2);
// Reprogram SEL_DPD_CTRL
    writel(timing.emc_sel_dpd_ctrl, emc.regs + EMC_SEL_DPD_CTRL);
    emc_seq_update_timing(emc);
    emc.last_timing = *timing;
    }
// Initialization and deinitialization
    static void emc_read_current_timing(struct tegra_emc *emc,
    struct emc_timing *timing)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(emc_burst_regs); ++i)
    timing.emc_burst_data[i] =
    readl(emc.regs + emc_burst_regs[i]);
    timing.emc_cfg = readl(emc.regs + EMC_CFG);
    timing.emc_auto_cal_interval = 0;
    timing.emc_zcal_cnt_long = 0;
    timing.emc_mode_1 = 0;
    timing.emc_mode_2 = 0;
    timing.emc_mode_4 = 0;
    timing.emc_mode_reset = 0;
    }
#[no_mangle]
unsafe extern "C" fn emc_init(emc: *mut tegra_emc) {
    static void emc_init(struct tegra_emc *emc)
    {
    emc.dram_type = readl(emc.regs + EMC_FBIO_CFG5);
    if (emc.dram_type & EMC_FBIO_CFG5_DRAM_WIDTH_X64)
    emc.dram_bus_width = 64;
    else
    emc.dram_bus_width = 32;
    dev_info_once(emc.dev, "%ubit DRAM bus\n", emc.dram_bus_width);
    emc.dram_type &= EMC_FBIO_CFG5_DRAM_TYPE_MASK;
    emc.dram_type >>= EMC_FBIO_CFG5_DRAM_TYPE_SHIFT;
    emc.dram_num = tegra_mc_get_emem_device_count(emc.mc);
    emc_read_current_timing(emc, &emc.last_timing);
    }
    static int load_one_timing_from_dt(struct tegra_emc *emc,
    struct emc_timing *timing,
    struct device_node *node)
    {
    u32 value;
    int err;
    err = of_property_read_u32(node, "clock-frequency", &value);
    if (err) {
    dev_err(emc.dev, "timing %pOFn: failed to read rate: %d\n",
    node, err);
    return err;
    }
    timing.rate = value;
    err = of_property_read_u32_array(node, "nvidia,emc-configuration",
    timing.emc_burst_data,
    ARRAY_SIZE(timing.emc_burst_data));
    if (err) {
    dev_err(emc.dev,
    "timing %pOFn: failed to read emc burst data: %d\n",
    node, err);
    return err;
    }

    err = of_property_read_u32(node, dtprop, &timing.prop); \
    if (err) { \
    dev_err(emc.dev, "timing %pOFn: failed to read " #prop ": %d\n", \
    node, err); \
    return err; \
    } \
    }
    EMC_READ_PROP(emc_auto_cal_config, "nvidia,emc-auto-cal-config")
    EMC_READ_PROP(emc_auto_cal_config2, "nvidia,emc-auto-cal-config2")
    EMC_READ_PROP(emc_auto_cal_config3, "nvidia,emc-auto-cal-config3")
    EMC_READ_PROP(emc_auto_cal_interval, "nvidia,emc-auto-cal-interval")
    EMC_READ_PROP(emc_bgbias_ctl0, "nvidia,emc-bgbias-ctl0")
    EMC_READ_PROP(emc_cfg, "nvidia,emc-cfg")
    EMC_READ_PROP(emc_cfg_2, "nvidia,emc-cfg-2")
    EMC_READ_PROP(emc_ctt_term_ctrl, "nvidia,emc-ctt-term-ctrl")
    EMC_READ_PROP(emc_mode_1, "nvidia,emc-mode-1")
    EMC_READ_PROP(emc_mode_2, "nvidia,emc-mode-2")
    EMC_READ_PROP(emc_mode_4, "nvidia,emc-mode-4")
    EMC_READ_PROP(emc_mode_reset, "nvidia,emc-mode-reset")
    EMC_READ_PROP(emc_mrs_wait_cnt, "nvidia,emc-mrs-wait-cnt")
    EMC_READ_PROP(emc_sel_dpd_ctrl, "nvidia,emc-sel-dpd-ctrl")
    EMC_READ_PROP(emc_xm2dqspadctrl2, "nvidia,emc-xm2dqspadctrl2")
    EMC_READ_PROP(emc_zcal_cnt_long, "nvidia,emc-zcal-cnt-long")
    EMC_READ_PROP(emc_zcal_interval, "nvidia,emc-zcal-interval")

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_timings(_a: *const c_void, _b: *const c_void) -> c_int {
    static int cmp_timings(const void *_a, const void *_b)
    {
    const struct emc_timing *a = _a;
    const struct emc_timing *b = _b;
    if (a.rate < b.rate)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(b->rate: a->rate ==) -> else {
    else if (a.rate == b.rate)
    return 0;
    else
    return 1;
    }
    static int tegra124_emc_load_timings_from_dt(struct tegra_emc *emc,
    struct device_node *node)
    {
    let mut child_count: c_int = of_get_child_count(node);
    struct emc_timing *timing;
    let mut i: c_uint = 0;
    int err;
    emc.timings = devm_kcalloc(emc.dev, child_count, sizeof(*timing),
    GFP_KERNEL);
    if (!emc.timings)
    return -ENOMEM;
    emc.num_timings = child_count;
    for_each_child_of_node_scoped(node, child) {
    timing = &emc.timings[i++];
    err = load_one_timing_from_dt(emc, timing, child);
    if (err)
    return err;
    }
    sort(emc.timings, emc.num_timings, sizeof(*timing), cmp_timings,
    core::ptr::null_mut());
    return 0;
    }
    static const struct of_device_id tegra124_emc_of_match[] = {
    { .compatible = "nvidia,tegra124-emc" },
    { .compatible = "nvidia,tegra132-emc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra124_emc_of_match);
    static struct device_node *
    tegra124_emc_find_node_by_ram_code(struct device_node *node, u32 ram_code)
    {
    struct device_node *np;
    int err;
    for_each_child_of_node(node, np) {
    u32 value;
    err = of_property_read_u32(np, "nvidia,ram-code", &value);
    if (err || (value != ram_code))
    continue;
    return np;
    }
    return core::ptr::null_mut();
    }
//
// debugfs interface
//
// The memory controller driver exposes some files in debugfs that can be used
// to control the EMC frequency. The top-level directory can be found here:
//
// /sys/kernel/debug/emc
//
// It contains the following files:
//
// - available_rates: This file contains a list of valid, space-separated
// EMC frequencies.
//
// - min_rate: Writing a value to this file sets the given frequency as the
// floor of the permitted range. If this is higher than the currently
// configured EMC frequency, this will cause the frequency to be
// increased so that it stays within the valid range.
//
// - max_rate: Similarily to the min_rate file, writing a value to this file
// sets the given frequency as the ceiling of the permitted range. If
// the value is lower than the currently configured EMC frequency, this
// will cause the frequency to be decreased so that it stays within the
// valid range.
//
#[no_mangle]
unsafe extern "C" fn tegra124_emc_validate_rate(emc: *mut tegra_emc, rate: c_ulong) -> bool {
    static bool tegra124_emc_validate_rate(struct tegra_emc *emc, unsigned long rate)
    {
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++)
    if (rate == emc.timings[i].rate)
    return true;
    return false;
    }
    static int tegra124_emc_debug_available_rates_show(struct seq_file *s,
    void *data)
    {
    struct tegra_emc *emc = s.private;
    const char *prefix = "";
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    seq_printf(s, "%s%lu", prefix, emc.timings[i].rate);
    prefix = " ";
    }
    seq_puts(s, "\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra124_emc_debug_available_rates);
#[no_mangle]
unsafe extern "C" fn tegra124_emc_debug_min_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra124_emc_debug_min_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.min_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra124_emc_debug_min_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra124_emc_debug_min_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra124_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.min_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra124_emc_debug_min_rate_fops,
    tegra124_emc_debug_min_rate_get,
    tegra124_emc_debug_min_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra124_emc_debug_max_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra124_emc_debug_max_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.max_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra124_emc_debug_max_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra124_emc_debug_max_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra124_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_max_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.max_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra124_emc_debug_max_rate_fops,
    tegra124_emc_debug_max_rate_get,
    tegra124_emc_debug_max_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn emc_debugfs_init(dev: *mut device, emc: *mut tegra_emc) {
    static void emc_debugfs_init(struct device *dev, struct tegra_emc *emc)
    {
    unsigned int i;
    int err;
    emc.debugfs.min_rate = ULONG_MAX;
    emc.debugfs.max_rate = 0;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate < emc.debugfs.min_rate)
    emc.debugfs.min_rate = emc.timings[i].rate;
    if (emc.timings[i].rate > emc.debugfs.max_rate)
    emc.debugfs.max_rate = emc.timings[i].rate;
    }
    if (!emc.num_timings) {
    emc.debugfs.min_rate = clk_get_rate(emc.clk);
    emc.debugfs.max_rate = emc.debugfs.min_rate;
    }
    err = clk_set_rate_range(emc.clk, emc.debugfs.min_rate,
    emc.debugfs.max_rate);
    if (err < 0) {
    dev_err(dev, "failed to set rate range [%lu-%lu] for %pC\n",
    emc.debugfs.min_rate, emc.debugfs.max_rate,
    emc.clk);
    return;
    }
    emc.debugfs.root = debugfs_create_dir("emc", core::ptr::null_mut());
    debugfs_create_file("available_rates", 0444, emc.debugfs.root, emc,
    &tegra124_emc_debug_available_rates_fops);
    debugfs_create_file("min_rate", 0644, emc.debugfs.root,
    emc, &tegra124_emc_debug_min_rate_fops);
    debugfs_create_file("max_rate", 0644, emc.debugfs.root,
    emc, &tegra124_emc_debug_max_rate_fops);
    }
    static inline struct tegra_emc *
    to_tegra_emc_provider(struct icc_provider *provider)
    {
    return container_of(provider, struct tegra_emc, provider);
    }
    static struct icc_node_data *
    emc_of_icc_xlate_extended(const struct of_phandle_args *spec, void *data)
    {
    struct icc_provider *provider = data;
    struct icc_node_data *ndata;
    struct icc_node *node;
// External Memory is the only possible ICC route
    list_for_each_entry(node, &provider.nodes, node_list) {
    if (node.id != TEGRA_ICC_EMEM)
    continue;
    ndata = kzalloc_obj(*ndata);
    if (!ndata)
    return ERR_PTR(-ENOMEM);
//
// SRC and DST nodes should have matching TAG in order to have
// it set by default for a requested path.
//
    ndata.tag = TEGRA_MC_ICC_TAG_ISO;
    ndata.node = node;
    return ndata;
    }
    return ERR_PTR(-EPROBE_DEFER);
    }
#[no_mangle]
unsafe extern "C" fn emc_icc_set(src: *mut icc_node, dst: *mut icc_node) -> c_int {
    static int emc_icc_set(struct icc_node *src, struct icc_node *dst)
    {
    struct tegra_emc *emc = to_tegra_emc_provider(dst.provider);
    let mut peak_bw: c_ulonglong = icc_units_to_bps(dst.peak_bw);
    let mut avg_bw: c_ulonglong = icc_units_to_bps(dst.avg_bw);
    let mut rate: c_ulonglong = max(avg_bw, peak_bw);
    unsigned int dram_data_bus_width_bytes;
    let mut ddr: c_uint = 2;
    int err;
//
// Tegra124 EMC runs on a clock rate of SDRAM bus. This means that
// EMC clock rate is twice smaller than the peak data rate because
// data is sampled on both EMC clock edges.
//
    dram_data_bus_width_bytes = emc.dram_bus_width / 8;
    do_div(rate, ddr * dram_data_bus_width_bytes);
    rate = min_t(u64, rate, U32_MAX);
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_ICC);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra124_emc_interconnect_init(emc: *mut tegra_emc) -> c_int {
    static int tegra124_emc_interconnect_init(struct tegra_emc *emc)
    {
    const struct tegra_mc_soc *soc = emc.mc.soc;
    struct icc_node *node;
    int err;
    emc.provider.dev = emc.dev;
    emc.provider.set = emc_icc_set;
    emc.provider.data = &emc.provider;
    emc.provider.aggregate = soc.icc_ops.aggregate;
    emc.provider.xlate_extended = emc_of_icc_xlate_extended;
    icc_provider_init(&emc.provider);
// create External Memory Controller node
    node = icc_node_create(TEGRA_ICC_EMC);
    if (IS_ERR(node))
    return PTR_ERR(node);
    node.name = "External Memory Controller";
    icc_node_add(node, &emc.provider);
// link External Memory Controller to External Memory (DRAM)
    err = icc_link_create(node, TEGRA_ICC_EMEM);
    if (err)
    goto remove_nodes;
// create External Memory node
    node = icc_node_create(TEGRA_ICC_EMEM);
    if (IS_ERR(node)) {
    err = PTR_ERR(node);
    goto remove_nodes;
    }
    node.name = "External Memory (DRAM)";
    icc_node_add(node, &emc.provider);
    err = icc_provider_register(&emc.provider);
    if (err)
    goto remove_nodes;
    return 0;
    remove_nodes:
    icc_nodes_remove(&emc.provider);
    return dev_err_probe(emc.dev, err, "failed to initialize ICC\n");
    }
#[no_mangle]
unsafe extern "C" fn tegra124_emc_opp_table_init(emc: *mut tegra_emc) -> c_int {
    static int tegra124_emc_opp_table_init(struct tegra_emc *emc)
    {
    let mut hw_version: u32 = BIT(tegra_sku_info.soc_speedo_id);
    int opp_token, err;
    err = dev_pm_opp_set_supported_hw(emc.dev, &hw_version, 1);
    if (err < 0)
    return dev_err_probe(emc.dev, err, "failed to set OPP supported HW\n");
    opp_token = err;
    err = dev_pm_opp_of_add_table(emc.dev);
    if (err) {
    if (err == -ENODEV)
    dev_err_probe(emc.dev, err,
    "OPP table not found, please update your device tree\n");
    else
    dev_err_probe(emc.dev, err, "failed to add OPP table\n");
    goto put_hw_table;
    }
    dev_info_once(emc.dev, "OPP HW ver. 0x%x, current clock rate %lu MHz\n",
    hw_version, clk_get_rate(emc.clk) / 1000000);
// first dummy rate-set initializes voltage state
    err = dev_pm_opp_set_rate(emc.dev, clk_get_rate(emc.clk));
    if (err) {
    dev_err_probe(emc.dev, err, "failed to initialize OPP clock\n");
    goto remove_table;
    }
    return 0;
    remove_table:
    dev_pm_opp_of_remove_table(emc.dev);
    put_hw_table:
    dev_pm_opp_put_supported_hw(opp_token);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn devm_tegra124_emc_unset_callback(data: *mut c_void) {
    static void devm_tegra124_emc_unset_callback(void *data)
    {
    tegra124_clk_set_emc_callbacks(core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn tegra124_emc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra124_emc_probe(struct platform_device *pdev)
    {
    struct device_node *np;
    struct tegra_emc *emc;
    u32 ram_code;
    int err;
    emc = devm_kzalloc(&pdev.dev, sizeof(*emc), GFP_KERNEL);
    if (!emc)
    return -ENOMEM;
    emc.dev = &pdev.dev;
    emc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(emc.regs))
    return PTR_ERR(emc.regs);
    emc.mc = devm_tegra_memory_controller_get(&pdev.dev);
    if (IS_ERR(emc.mc))
    return PTR_ERR(emc.mc);
    ram_code = tegra_read_ram_code();
    np = tegra124_emc_find_node_by_ram_code(pdev.dev.of_node, ram_code);
    if (np) {
    err = tegra124_emc_load_timings_from_dt(emc, np);
    of_node_put(np);
    if (err)
    return err;
    } else {
    dev_info_once(&pdev.dev,
    "no memory timings for RAM code %u found in DT\n",
    ram_code);
    }
    emc_init(emc);
    platform_set_drvdata(pdev, emc);
    tegra124_clk_set_emc_callbacks(tegra124_emc_prepare_timing_change,
    tegra124_emc_complete_timing_change);
    err = devm_add_action_or_reset(&pdev.dev, devm_tegra124_emc_unset_callback,
    core::ptr::null_mut());
    if (err)
    return err;
    emc.clk = devm_clk_get(&pdev.dev, "emc");
    if (IS_ERR(emc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(emc.clk),
    "failed to get EMC clock\n");
    err = tegra124_emc_opp_table_init(emc);
    if (err)
    return err;
    tegra_emc_rate_requests_init(&emc.reqs, &pdev.dev);
    if (IS_ENABLED(CONFIG_DEBUG_FS))
    emc_debugfs_init(&pdev.dev, emc);
    tegra124_emc_interconnect_init(emc);
//
// Don't allow the kernel module to be unloaded. Unloading adds some
// extra complexity which doesn't really worth the effort in a case of
// this driver.
//
    try_module_get(THIS_MODULE);
    return 0;
    };
    static struct platform_driver tegra124_emc_driver = {
    .probe = tegra124_emc_probe,
    .driver = {
    .name = "tegra-emc",
    .of_match_table = tegra124_emc_of_match,
    .suppress_bind_attrs = true,
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(tegra124_emc_driver);
    MODULE_AUTHOR("Mikko Perttunen <mperttunen@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra124 EMC driver");
    MODULE_LICENSE("GPL v2");
