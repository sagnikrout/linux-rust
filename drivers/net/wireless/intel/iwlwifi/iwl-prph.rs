//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-prph.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_prph_h__

//
// Registers in this file are internal, not PCI bus memory mapped.
// Driver accesses these via HBUS_TARG_PRPH_* registers.
//

// APMG (power management) constants

// Device system time
pub const DEVICE_SYSTEM_TIME_REG: c_uint = 0xA0206C;
// Device NMI register and value for 8000 family and lower hw's
pub const DEVICE_SET_NMI_REG: c_uint = 0x00a01c30;

// Device NMI register and value for 9000 family and above hw's
pub const UREG_NIC_SET_NMI_DRIVER: c_uint = 0x00a05c10;

// Shared registers (0x0..0x3ff, via target indirect or periphery
pub const SHR_BASE: c_uint = 0x00a10000;
// Shared GP1 register
pub const SHR_APMG_GP1_REG: c_uint = 0x01dc;

pub const SHR_APMG_GP1_WF_XTAL_LP_EN: c_uint = 0x00000004;
pub const SHR_APMG_GP1_CHICKEN_BIT_SELECT: c_uint = 0x80000000;
// Shared DL_CFG register
pub const SHR_APMG_DL_CFG_REG: c_uint = 0x01c4;

pub const SHR_APMG_DL_CFG_RTCS_CLK_SELECTOR_MSK: c_uint = 0x000000c0;
pub const SHR_APMG_DL_CFG_RTCS_CLK_INTERNAL_XTAL: c_uint = 0x00000080;
pub const SHR_APMG_DL_CFG_DL_CLOCK_POWER_UP: c_uint = 0x00000100;
// Shared APMG_XTAL_CFG register
pub const SHR_APMG_XTAL_CFG_REG: c_uint = 0x1c0;
pub const SHR_APMG_XTAL_CFG_XTAL_ON_REQ: c_uint = 0x80000000;
//
// Device reset for family 8000
// write to bit 24 in order to reset the CPU
//

//
// 7000/3000 series SHR DTS addresses
//

//
// Tx Scheduler
//
// The Tx Scheduler selects the next frame to be transmitted, choosing TFDs
// (Transmit Frame Descriptors) from up to 16 circular Tx queues resident in
// host DRAM.  It steers each frame's Tx command (which contains the frame
// data) into one of up to 7 prioritized Tx DMA FIFO channels within the
// device.  A queue maps to only one (selectable by driver) Tx DMA channel,
// but one DMA channel may take input from several queues.
//
// Tx DMA FIFOs have dedicated purposes.
//
// For 5000 series and up, they are used differently
// (cf. iwl5000_default_queue_to_tx_fifo in iwl-5000.c):
//
// 0 -- EDCA BK (background) frames, lowest priority
// 1 -- EDCA BE (best effort) frames, normal priority
// 2 -- EDCA VI (video) frames, higher priority
// 3 -- EDCA VO (voice) and management frames, highest priority
// 4 -- unused
// 5 -- unused
// 6 -- unused
// 7 -- Commands
//
// Driver should normally map queues 0-6 to Tx DMA/FIFO channels 0-6.
// In addition, driver can map the remaining queues to Tx DMA/FIFO
// channels 0-3 to support 11n aggregation via EDCA DMA channels.
//
// The driver sets up each queue to work in one of two modes:
//
// 1)  Scheduler-Ack, in which the scheduler automatically supports a
// block-ack (BA) window of up to 64 TFDs.  In this mode, each queue
// contains TFDs for a unique combination of Recipient Address (RA)
// and Traffic Identifier (TID), that is, traffic of a given
// Quality-Of-Service (QOS) priority, destined for a single station.
//
// In scheduler-ack mode, the scheduler keeps track of the Tx status of
// each frame within the BA window, including whether it's been transmitted,
// and whether it's been acknowledged by the receiving station.  The device
// automatically processes block-acks received from the receiving STA,
// and reschedules un-acked frames to be retransmitted (successful
// Tx completion may end up being out-of-order).
//
// The driver must maintain the queue's Byte Count table in host DRAM
// for this mode.
// This mode does not support fragmentation.
//
// 2)  FIFO (a.k.a. non-Scheduler-ACK), in which each TFD is processed in order.
// The device may automatically retry Tx, but will retry only one frame
// at a time, until receiving ACK from receiving station, or reaching
// retry limit and giving up.
//
// The command queue (#4/#9) must use this mode!
// This mode does not require use of the Byte Count table in host DRAM.
//
// Driver controls scheduler operation via 3 means:
// 1)  Scheduler registers
// 2)  Shared scheduler data base in internal SRAM
// 3)  Shared data in host DRAM
//
// Initialization:
//
// When loading, driver should allocate memory for:
// 1)  16 TFD circular buffers, each with space for (typically) 256 TFDs.
// 2)  16 Byte Count circular buffers in 16 KBytes contiguous memory
// (1024 bytes for each queue).
//
// After receiving "Alive" response from uCode, driver must initialize
// the scheduler (especially for queue #4/#9, the command queue, otherwise
// the driver can't issue commands!):
//

//
// Max Tx window size is the max number of contiguous TFDs that the scheduler
// can keep track of at one time when creating block-ack chains of frames.
// Note that "64" matches the number of ack bits in a block-ack packet.
//
pub const SCD_WIN_SIZE: c_int = 64;
pub const SCD_FRAME_LIMIT: c_int = 64;

// agn SCD

// Context Data

// Tx status

// Translation Data

// Macro flag: #define SCD_CONTEXT_QUEUE_OFFSET(x)\
// Macro flag: #define SCD_TX_STTS_QUEUE_OFFSET(x)\

// END TX SCHEDULER
// Oscillator clock

//
// Replacing FH_UCODE_LOAD_STATUS
// This register is writen by driver and is read by uCode during boot flow.
// Note this address is cleared after MAC reset.
//

// Rx FIFO

// Tx FIFO

// UMAC Internal Tx Fifo

// Radio registers access

// LTR control (Qu only)
pub const HPM_MAC_LTR_CSR: c_uint = 0xa0348c;
pub const HPM_MAC_LRT_ENABLE_ALL: c_uint = 0xf;
// also uses CSR_LTR_* for values
pub const HPM_UMAC_LTR: c_uint = 0xa03480;
// FW monitor

// FW monitor family 8000 and on

// FW monitor familiy AX210 and on

// M2S registers

// enable the ID buf for read
pub const WFPM_PS_CTL_CLR: c_uint = 0xA0300C;
pub const WFMP_MAC_ADDR_0: c_uint = 0xA03080;
pub const WFMP_MAC_ADDR_1: c_uint = 0xA03084;
pub const LMPM_PMG_EN: c_uint = 0xA01CEC;
pub const RADIO_REG_SYS_MANUAL_DFT_0: c_uint = 0xAD4078;
pub const RFIC_REG_RD: c_uint = 0xAD0470;
pub const WFPM_CTRL_REG: c_uint = 0xA03030;
pub const WFPM_OTP_CFG1_ADDR: c_uint = 0x00a03098;

pub const WFPM_OTP_BZ_BNJ_JACKET_BIT: c_int = 5;
pub const WFPM_OTP_BZ_BNJ_CDB_BIT: c_int = 4;

pub const WFPM_GP2: c_uint = 0xA030B4;
// DBGI SRAM Register details
pub const DBGI_SRAM_TARGET_ACCESS_RDATA_LSB: c_uint = 0x00A2E154;
pub const DBGI_SRAM_TARGET_ACCESS_RDATA_MSB: c_uint = 0x00A2E158;
pub const DBGI_SRAM_FIFO_POINTERS: c_uint = 0x00A2E148;
pub const DBGI_SRAM_FIFO_POINTERS_WR_PTR_MSK: c_uint = 0x00000FFF;
pub const CNVI_AUX_MISC_CHIP: c_uint = 0xA200B0;

pub const CNVI_AUX_MISC_CHIP_PROD_TYPE_GL: c_uint = 0x910;
pub const CNVI_AUX_MISC_CHIP_PROD_TYPE_BZ_U: c_uint = 0x930;
pub const CNVI_AUX_MISC_CHIP_PROD_TYPE_BZ_I: c_uint = 0x900;
pub const CNVI_AUX_MISC_CHIP_PROD_TYPE_BZ_W: c_uint = 0x901;
pub const CNVR_AUX_MISC_CHIP: c_uint = 0xA2B800;
pub const CNVR_SCU_SD_REGS_SD_REG_DIG_DCDC_VTRIM: c_uint = 0xA29890;
pub const CNVR_SCU_SD_REGS_SD_REG_ACTIVE_VDIG_MIRROR: c_uint = 0xA29938;
pub const CNVI_SCU_SEQ_DATA_DW9: c_uint = 0xA27488;
pub const CNVI_SCU_REG_FOR_ECO_1: c_uint = 0xA26EF8;

pub const CNVI_PMU_STEP_FLOW_BZ: c_uint = 0xA2D588;
pub const CNVI_PMU_STEP_FLOW_SC: c_uint = 0xA2D688;

pub const PREG_AUX_BUS_WPROT_0: c_uint = 0xA04CC0;
// device family 9000 WPROT register
pub const PREG_PRPH_WPROT_9000: c_uint = 0xA04CE0;
// device family 22000 WPROT register
pub const PREG_PRPH_WPROT_22000: c_uint = 0xA04D00;
pub const SB_MODIFY_CFG_FLAG: c_uint = 0xA03088;
pub const SB_CFG_RESIDES_IN_ROM: c_uint = 0x80;
pub const SB_CPU_1_STATUS: c_uint = 0xA01E30;
pub const SB_CPU_2_STATUS: c_uint = 0xA01E34;
pub const UMAG_SB_CPU_1_STATUS: c_uint = 0xA038C0;
pub const UMAG_SB_CPU_2_STATUS: c_uint = 0xA038C4;
pub const UMAG_GEN_HW_STATUS: c_uint = 0xA038C8;
pub const UREG_UMAC_CURRENT_PC: c_uint = 0xa05c18;
pub const UREG_LMAC1_CURRENT_PC: c_uint = 0xa05c1c;
pub const UREG_LMAC2_CURRENT_PC: c_uint = 0xa05c20;
pub const WFPM_LMAC1_PD_NOTIFICATION: c_uint = 0xa0338c;
pub const WFPM_ARC1_PD_NOTIFICATION: c_uint = 0xa03044;
pub const HPM_SECONDARY_DEVICE_STATE: c_uint = 0xa03404;
pub const WFPM_MAC_OTP_CFG7_ADDR: c_uint = 0xa03338;
pub const WFPM_MAC_OTP_CFG7_DATA: c_uint = 0xa0333c;
pub const WFPM_RSRCS_4PHS_REQ_STTS: c_uint = 0xa033f8;
pub const WFPM_RSRCS_4PHS_ACK_STTS: c_uint = 0xa033fc;

// For UMAG_GEN_HW_STATUS reg check
// FW chicken bits
pub const LMPM_CHICK: c_uint = 0xA01FF8;
// FW chicken bits
pub const LMPM_PAGE_PASS_NOTIF: c_uint = 0xA03824;
//
// CRF ID register
//
// type: bits 0-11
// reserved: bits 12-18
// slave_exist: bit 19
// dash: bits 20-23
// step: bits 24-27
// flavor: bits 28-31
//

pub const SD_REG_VER: c_uint = 0xa29600;
pub const SD_REG_VER_GEN2: c_uint = 0x00a2b800;
pub const REG_CRF_ID_TYPE_JF_1: c_uint = 0x201;
pub const REG_CRF_ID_TYPE_JF_2: c_uint = 0x202;
pub const REG_CRF_ID_TYPE_HR_CDB: c_uint = 0x503;
pub const REG_CRF_ID_TYPE_HR_NONE_CDB: c_uint = 0x504;
pub const REG_CRF_ID_TYPE_HR_NONE_CDB_1X1: c_uint = 0x501;
pub const REG_CRF_ID_TYPE_HR_NONE_CDB_CCP: c_uint = 0x532;
pub const REG_CRF_ID_TYPE_GF: c_uint = 0x410;
pub const REG_CRF_ID_TYPE_FM: c_uint = 0x910;
pub const REG_CRF_ID_TYPE_WHP: c_uint = 0xA10;
pub const REG_CRF_ID_TYPE_PE: c_uint = 0xA30;
pub const HPM_DEBUG: c_uint = 0xA03440;

pub const HPM_HIPM_GEN_CFG: c_uint = 0xA03458;

pub const UREG_DOORBELL_TO_ISR6: c_uint = 0xA05C04;

//
// From BZ family driver triggers this bit for suspend and resume
// The driver should update CSR_IPC_SLEEP_CONTROL before triggering
// this interrupt with suspend/resume value
//

pub const CNVI_MBOX_C: c_uint = 0xA3400C;
pub const FSEQ_ERROR_CODE: c_uint = 0xA340C8;
pub const FSEQ_TOP_INIT_VERSION: c_uint = 0xA34038;
pub const FSEQ_CNVIO_INIT_VERSION: c_uint = 0xA3403C;
pub const FSEQ_OTP_VERSION: c_uint = 0xA340FC;
pub const FSEQ_TOP_CONTENT_VERSION: c_uint = 0xA340F4;
pub const FSEQ_ALIVE_TOKEN: c_uint = 0xA340F0;
pub const FSEQ_CNVI_ID: c_uint = 0xA3408C;
pub const FSEQ_CNVR_ID: c_uint = 0xA34090;
pub const FSEQ_PREV_CNVIO_INIT_VERSION: c_uint = 0xA34084;
pub const FSEQ_WIFI_FSEQ_VERSION: c_uint = 0xA34040;
pub const FSEQ_BT_FSEQ_VERSION: c_uint = 0xA34044;
pub const FSEQ_CLASS_TP_VERSION: c_uint = 0xA34078;
pub const IWL_D3_SLEEP_STATUS_SUSPEND: c_uint = 0xD3;
pub const IWL_D3_SLEEP_STATUS_RESUME: c_uint = 0xD0;
pub const WMAL_INDRCT_RD_CMD1_OPMOD_POS: c_int = 28;
pub const WMAL_INDRCT_RD_CMD1_BYTE_ADDRESS_MSK: c_uint = 0xFFFFF;
pub const WMAL_CMD_READ_BURST_ACCESS: c_int = 2;
pub const WMAL_MRSPF_1: c_uint = 0xADFC20;
pub const WMAL_INDRCT_RD_CMD1: c_uint = 0xADFD44;
pub const WMAL_INDRCT_CMD1: c_uint = 0xADFC14;

pub const WMAL_MRSPF_STTS: c_uint = 0xADFC24;
pub const WMAL_MRSPF_STTS_FIFO1_NOT_EMPTY_POS: c_int = 15;
pub const WMAL_MRSPF_STTS_FIFO1_NOT_EMPTY_MSK: c_uint = 0x8000;
pub const WMAL_TIMEOUT_VAL: c_uint = 0xA5A5A5A2;

pub const WFPM_LMAC1_PS_CTL_RW: c_uint = 0xA03380;
pub const WFPM_LMAC2_PS_CTL_RW: c_uint = 0xA033C0;
pub const WFPM_PS_CTL_RW_PHYRF_PD_FSM_CURSTATE_MSK: c_uint = 0x0000000F;
pub const WFPM_PHYRF_STATE_ON: c_int = 5;
pub const HBUS_TIMEOUT: c_uint = 0xA5A5A5A1;
pub const WFPM_DPHY_OFF: c_uint = 0xDF10FF;
pub const REG_OTP_MINOR: c_uint = 0xA0333C;
pub const WFPM_LMAC2_PD_NOTIFICATION: c_uint = 0xA033CC;

pub const DPHYIP_INDIRECT: c_uint = 0xA2D800;
pub const DPHYIP_INDIRECT_RD_MSK: c_uint = 0xFF000000;
pub const DPHYIP_INDIRECT_RD_SHIFT: c_int = 24;
