//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/altera_edac.h
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
// Copyright (C) 2017-2018, Intel Corporation
// Copyright (C) 2015 Altera Corporation
//

// SDRAM Controller CtrlCfg Register
pub const CV_CTLCFG_OFST: c_uint = 0x00;
// SDRAM Controller CtrlCfg Register Bit Masks
pub const CV_CTLCFG_ECC_EN: c_uint = 0x400;
pub const CV_CTLCFG_ECC_CORR_EN: c_uint = 0x800;
pub const CV_CTLCFG_GEN_SB_ERR: c_uint = 0x2000;
pub const CV_CTLCFG_GEN_DB_ERR: c_uint = 0x4000;

// SDRAM Controller Address Width Register
pub const CV_DRAMADDRW_OFST: c_uint = 0x2C;
// SDRAM Controller Address Widths Field Register
pub const DRAMADDRW_COLBIT_MASK: c_uint = 0x001F;
pub const DRAMADDRW_COLBIT_SHIFT: c_int = 0;
pub const DRAMADDRW_ROWBIT_MASK: c_uint = 0x03E0;
pub const DRAMADDRW_ROWBIT_SHIFT: c_int = 5;
pub const CV_DRAMADDRW_BANKBIT_MASK: c_uint = 0x1C00;
pub const CV_DRAMADDRW_BANKBIT_SHIFT: c_int = 10;
pub const CV_DRAMADDRW_CSBIT_MASK: c_uint = 0xE000;
pub const CV_DRAMADDRW_CSBIT_SHIFT: c_int = 13;
// SDRAM Controller Interface Data Width Register
pub const CV_DRAMIFWIDTH_OFST: c_uint = 0x30;
// SDRAM Controller Interface Data Width Defines
pub const CV_DRAMIFWIDTH_16B_ECC: c_int = 24;
pub const CV_DRAMIFWIDTH_32B_ECC: c_int = 40;
// SDRAM Controller DRAM Status Register
pub const CV_DRAMSTS_OFST: c_uint = 0x38;
// SDRAM Controller DRAM Status Register Bit Masks
pub const CV_DRAMSTS_SBEERR: c_uint = 0x04;
pub const CV_DRAMSTS_DBEERR: c_uint = 0x08;
pub const CV_DRAMSTS_CORR_DROP: c_uint = 0x10;
// SDRAM Controller DRAM IRQ Register
pub const CV_DRAMINTR_OFST: c_uint = 0x3C;
// SDRAM Controller DRAM IRQ Register Bit Masks
pub const CV_DRAMINTR_INTREN: c_uint = 0x01;
pub const CV_DRAMINTR_SBEMASK: c_uint = 0x02;
pub const CV_DRAMINTR_DBEMASK: c_uint = 0x04;
pub const CV_DRAMINTR_CORRDROPMASK: c_uint = 0x08;
pub const CV_DRAMINTR_INTRCLR: c_uint = 0x10;
// SDRAM Controller Single Bit Error Count Register
pub const CV_SBECOUNT_OFST: c_uint = 0x40;
// SDRAM Controller Double Bit Error Count Register
pub const CV_DBECOUNT_OFST: c_uint = 0x44;
// SDRAM Controller ECC Error Address Register
pub const CV_ERRADDR_OFST: c_uint = 0x48;
// -----------------------------------------
// SDRAM Controller EccCtrl Register
pub const A10_ECCCTRL1_OFST: c_uint = 0x00;
// SDRAM Controller EccCtrl Register Bit Masks
pub const A10_ECCCTRL1_ECC_EN: c_uint = 0x001;
pub const A10_ECCCTRL1_CNT_RST: c_uint = 0x010;
pub const A10_ECCCTRL1_AWB_CNT_RST: c_uint = 0x100;

// SDRAM Controller Address Width Register
pub const CV_DRAMADDRW: c_uint = 0xFFC2502C;
pub const A10_DRAMADDRW: c_uint = 0xFFCFA0A8;
pub const S10_DRAMADDRW: c_uint = 0xF80110E0;
// SDRAM Controller Address Widths Field Register
pub const DRAMADDRW_COLBIT_MASK: c_uint = 0x001F;
pub const DRAMADDRW_COLBIT_SHIFT: c_int = 0;
pub const DRAMADDRW_ROWBIT_MASK: c_uint = 0x03E0;
pub const DRAMADDRW_ROWBIT_SHIFT: c_int = 5;
pub const CV_DRAMADDRW_BANKBIT_MASK: c_uint = 0x1C00;
pub const CV_DRAMADDRW_BANKBIT_SHIFT: c_int = 10;
pub const CV_DRAMADDRW_CSBIT_MASK: c_uint = 0xE000;
pub const CV_DRAMADDRW_CSBIT_SHIFT: c_int = 13;
pub const A10_DRAMADDRW_BANKBIT_MASK: c_uint = 0x3C00;
pub const A10_DRAMADDRW_BANKBIT_SHIFT: c_int = 10;
pub const A10_DRAMADDRW_GRPBIT_MASK: c_uint = 0xC000;
pub const A10_DRAMADDRW_GRPBIT_SHIFT: c_int = 14;
pub const A10_DRAMADDRW_CSBIT_MASK: c_uint = 0x70000;
pub const A10_DRAMADDRW_CSBIT_SHIFT: c_int = 16;
// SDRAM Controller Interface Data Width Register
pub const CV_DRAMIFWIDTH: c_uint = 0xFFC25030;
pub const A10_DRAMIFWIDTH: c_uint = 0xFFCFB008;
pub const S10_DRAMIFWIDTH: c_uint = 0xF8011008;
// SDRAM Controller Interface Data Width Defines
pub const CV_DRAMIFWIDTH_16B_ECC: c_int = 24;
pub const CV_DRAMIFWIDTH_32B_ECC: c_int = 40;
pub const A10_DRAMIFWIDTH_16B: c_uint = 0x0;
pub const A10_DRAMIFWIDTH_32B: c_uint = 0x1;
pub const A10_DRAMIFWIDTH_64B: c_uint = 0x2;
// SDRAM Controller DRAM IRQ Register
pub const A10_ERRINTEN_OFST: c_uint = 0x10;
// SDRAM Controller DRAM IRQ Register Bit Masks
pub const A10_ERRINTEN_SERRINTEN: c_uint = 0x01;
pub const A10_ERRINTEN_DERRINTEN: c_uint = 0x02;

// SDRAM Interrupt Mode Register
pub const A10_INTMODE_OFST: c_uint = 0x1C;
pub const A10_INTMODE_SB_INT: c_int = 1;
// SDRAM Controller Error Status Register
pub const A10_INTSTAT_OFST: c_uint = 0x20;
// SDRAM Controller Error Status Register Bit Masks
pub const A10_INTSTAT_SBEERR: c_uint = 0x01;
pub const A10_INTSTAT_DBEERR: c_uint = 0x02;
// SDRAM Controller ECC Error Address Register
pub const A10_DERRADDR_OFST: c_uint = 0x2C;
pub const A10_SERRADDR_OFST: c_uint = 0x30;
// SDRAM Controller ECC Diagnostic Register
pub const A10_DIAGINTTEST_OFST: c_uint = 0x24;
pub const A10_DIAGINT_TSERRA_MASK: c_uint = 0x0001;
pub const A10_DIAGINT_TDERRA_MASK: c_uint = 0x0100;
pub const A10_SBERR_IRQ: c_int = 34;
pub const A10_DBERR_IRQ: c_int = 32;
// SDRAM Single Bit Error Count Compare Set Register
pub const A10_SERRCNTREG_OFST: c_uint = 0x3C;
pub const A10_SYMAN_INTMASK_CLR: c_uint = 0xFFD06098;
pub const A10_INTMASK_CLR_OFST: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_sdram_prv_data {
    pub ecc_ctrl_offset: c_int,
    pub ecc_ctl_en_mask: c_int,
    pub ecc_cecnt_offset: c_int,
    pub ecc_uecnt_offset: c_int,
    pub ecc_stat_offset: c_int,
    pub ecc_stat_ce_mask: c_int,
    pub ecc_stat_ue_mask: c_int,
    pub ecc_saddr_offset: c_int,
    pub ecc_daddr_offset: c_int,
    pub ecc_irq_en_offset: c_int,
    pub ecc_irq_en_mask: c_int,
    pub ecc_irq_clr_offset: c_int,
    pub ecc_irq_clr_mask: c_int,
    pub ecc_cnt_rst_offset: c_int,
    pub ecc_cnt_rst_mask: c_int,
    pub ecc_enable_mask: c_int,
    pub ce_set_mask: c_int,
    pub ue_set_mask: c_int,
    pub ce_ue_trgr_offset: c_int,
}

// Altera SDRAM Memory Controller data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_sdram_mc_data {
    pub mc_vbase: *mut regmap,
    pub sb_irq: c_int,
    pub db_irq: c_int,
    pub data: *const altr_sdram_prv_data,
}

// EDAC Device Defines
// General Device Trigger Defines

// Cyclone5 and Arria5 Defines
// OCRAM ECC Management Group Defines
pub const ALTR_MAN_GRP_OCRAM_ECC_OFFSET: c_uint = 0x04;
pub const ALTR_OCR_ECC_REG_OFFSET: c_uint = 0x00;

// L2 ECC Management Group Defines
pub const ALTR_MAN_GRP_L2_ECC_OFFSET: c_uint = 0x00;
pub const ALTR_L2_ECC_REG_OFFSET: c_uint = 0x00;

// Arria10 General ECC Block Module Defines
pub const ALTR_A10_ECC_CTRL_OFST: c_uint = 0x08;

pub const ALTR_A10_ECC_INITSTAT_OFST: c_uint = 0x0C;

pub const ALTR_A10_ECC_ERRINTEN_OFST: c_uint = 0x10;
pub const ALTR_A10_ECC_ERRINTENS_OFST: c_uint = 0x14;
pub const ALTR_A10_ECC_ERRINTENR_OFST: c_uint = 0x18;

pub const ALTR_A10_ECC_INTMODE_OFST: c_uint = 0x1C;

pub const ALTR_A10_ECC_INTSTAT_OFST: c_uint = 0x20;

pub const ALTR_A10_ECC_INTTEST_OFST: c_uint = 0x24;

// ECC Manager Defines
pub const A10_SYSMGR_ECC_INTMASK_SET_OFST: c_uint = 0x94;
pub const A10_SYSMGR_ECC_INTMASK_CLR_OFST: c_uint = 0x98;

pub const A10_SYSMGR_ECC_INTSTAT_SERR_OFST: c_uint = 0x9C;
pub const A10_SYSMGR_ECC_INTSTAT_DERR_OFST: c_uint = 0xA0;

pub const A10_SYSGMR_MPU_CLEAR_L2_ECC_OFST: c_uint = 0xA8;

// Arria 10 L2 ECC Management Group Defines
pub const ALTR_A10_L2_ECC_CTL_OFST: c_uint = 0x0;

pub const ALTR_A10_L2_ECC_STATUS: c_uint = 0xFFD060A4;
pub const ALTR_A10_L2_ECC_STAT_OFST: c_uint = 0xA4;

pub const ALTR_A10_L2_ECC_CLR_OFST: c_uint = 0x4;

pub const ALTR_A10_L2_ECC_CE_INJ_MASK: c_uint = 0x00000101;
pub const ALTR_A10_L2_ECC_UE_INJ_MASK: c_uint = 0x00010101;
// Arria 10 OCRAM ECC Management Group Defines

// Arria 10 Ethernet ECC Management Group Defines

// Arria 10 SDMMC ECC Management Group Defines

// A10 ECC Controller memory initialization timeout
pub const ALTR_A10_ECC_INIT_WATCHDOG_10US: c_int = 10000;
// Stratix10 Defines
pub const ALTR_S10_ECC_CTRL_SDRAM_OFST: c_uint = 0x00;

pub const ALTR_S10_ECC_ERRINTEN_OFST: c_uint = 0x10;
pub const ALTR_S10_ECC_ERRINTENS_OFST: c_uint = 0x14;
pub const ALTR_S10_ECC_ERRINTENR_OFST: c_uint = 0x18;

pub const ALTR_S10_ECC_INTMODE_OFST: c_uint = 0x1C;

pub const ALTR_S10_ECC_INTSTAT_OFST: c_uint = 0x20;

pub const ALTR_S10_ECC_INTTEST_OFST: c_uint = 0x24;

pub const ALTR_S10_DERR_ADDRA_OFST: c_uint = 0x2C;
// Stratix10 ECC Manager Defines
pub const S10_SYSMGR_ECC_INTMASK_CLR_OFST: c_uint = 0x98;
pub const S10_SYSMGR_ECC_INTSTAT_DERR_OFST: c_uint = 0xA0;
// Sticky registers for Uncorrected Errors
pub const S10_SYSMGR_UE_VAL_OFST: c_uint = 0x220;
pub const S10_SYSMGR_UE_ADDR_OFST: c_uint = 0x224;

pub const S10_DBE_IRQ_MASK: c_uint = 0x3FFFE;
// Define ECC Block Offsets for peripherals
pub const ECC_BLK_ADDRESS_OFST: c_uint = 0x40;
pub const ECC_BLK_RDATA0_OFST: c_uint = 0x44;
pub const ECC_BLK_RDATA1_OFST: c_uint = 0x48;
pub const ECC_BLK_RDATA2_OFST: c_uint = 0x4C;
pub const ECC_BLK_RDATA3_OFST: c_uint = 0x50;
pub const ECC_BLK_WDATA0_OFST: c_uint = 0x54;
pub const ECC_BLK_WDATA1_OFST: c_uint = 0x58;
pub const ECC_BLK_WDATA2_OFST: c_uint = 0x5C;
pub const ECC_BLK_WDATA3_OFST: c_uint = 0x60;
pub const ECC_BLK_RECC0_OFST: c_uint = 0x64;
pub const ECC_BLK_RECC1_OFST: c_uint = 0x68;
pub const ECC_BLK_WECC0_OFST: c_uint = 0x6C;
pub const ECC_BLK_WECC1_OFST: c_uint = 0x70;
pub const ECC_BLK_DBYTECTRL_OFST: c_uint = 0x74;
pub const ECC_BLK_ACCCTRL_OFST: c_uint = 0x78;
pub const ECC_BLK_STARTACC_OFST: c_uint = 0x7C;
pub const ECC_XACT_KICK: c_uint = 0x10000;
pub const ECC_WORD_WRITE: c_uint = 0xFF;
pub const ECC_WRITE_DOVR: c_uint = 0x101;
pub const ECC_WRITE_EDOVR: c_uint = 0x103;
pub const ECC_READ_EOVR: c_uint = 0x2;
pub const ECC_READ_EDOVR: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_device_prv_data {
    pub device): *mut *mut int (setup)(struct altr_edac_device_dev,
    pub ce_clear_mask: c_int,
    pub ue_clear_mask: c_int,
    pub irq_status_mask: c_int,
    pub other): *mut *mut *mut void  (alloc_mem)(size_t size, void,
    pub other): *mut *mut *mut void (free_mem)(void p, size_t size, void,
    pub ecc_enable_mask: c_int,
    pub ecc_en_ofst: c_int,
    pub ce_set_mask: c_int,
    pub ue_set_mask: c_int,
    pub set_err_ofst: c_int,
    pub dev_id): *mut *mut irqreturn_t (ecc_irq_handler)(int irq, void,
    pub trig_alloc_sz: c_int,
    pub inject_fops: *const file_operations,
    pub panic: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_edac_device_dev {
    pub next: list_head,
    pub base: *mut void __iomem,
    pub sb_irq: c_int,
    pub db_irq: c_int,
    pub data: *const edac_device_prv_data,
    pub debugfs_dir: *mut dentry,
    pub edac_dev_name: *mut c_char,
    pub edac: *mut altr_arria10_edac,
    pub edac_dev: *mut edac_device_ctl_info,
    pub ddev: device,
    pub edac_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altr_arria10_edac {
    pub dev: *mut device,
    pub ecc_mgr_map: *mut regmap,
    pub sb_irq: c_int,
    pub db_irq: c_int,
    pub domain: *mut irq_domain,
    pub irq_chip: irq_chip,
    pub a10_ecc_devices: list_head,
    pub panic_notifier: notifier_block,
    pub is_s10: bool,
}
