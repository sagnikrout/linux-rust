//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-thc/intel-thc-hw.h
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
// Copyright (c) 2024 Intel Corporation

// THC registers offset
// Touch Host Controller Control Register
pub const THC_M_PRT_CONTROL_OFFSET: c_uint = 0x1008;
// THC SPI Bus Configuration Register
pub const THC_M_PRT_SPI_CFG_OFFSET: c_uint = 0x1010;
// THC SPI Bus Read Opcode Register
pub const THC_M_PRT_SPI_ICRRD_OPCODE_OFFSET: c_uint = 0x1014;
// THC SPI Bus Read Opcode Register
pub const THC_M_PRT_SPI_DMARD_OPCODE_OFFSET: c_uint = 0x1018;
// THC SPI Bus Write Opcode Register
pub const THC_M_PRT_SPI_WR_OPCODE_OFFSET: c_uint = 0x101C;
// THC Interrupt Enable Register
pub const THC_M_PRT_INT_EN_OFFSET: c_uint = 0x1020;
// THC Interrupt Status Register
pub const THC_M_PRT_INT_STATUS_OFFSET: c_uint = 0x1024;
// THC Error Cause Register
pub const THC_M_PRT_ERR_CAUSE_OFFSET: c_uint = 0x1028;
// THC SW sequencing Control
pub const THC_M_PRT_SW_SEQ_CNTRL_OFFSET: c_uint = 0x1040;
// THC SW sequencing Status
pub const THC_M_PRT_SW_SEQ_STS_OFFSET: c_uint = 0x1044;
// THC SW Sequencing Data DW0 or SPI Address Register
pub const THC_M_PRT_SW_SEQ_DATA0_ADDR_OFFSET: c_uint = 0x1048;
// THC SW sequencing Data DW1
pub const THC_M_PRT_SW_SEQ_DATA1_OFFSET: c_uint = 0x104C;
// THC SW sequencing Data DW2
pub const THC_M_PRT_SW_SEQ_DATA2_OFFSET: c_uint = 0x1050;
// THC SW sequencing Data DW3
pub const THC_M_PRT_SW_SEQ_DATA3_OFFSET: c_uint = 0x1054;
// THC SW sequencing Data DW4
pub const THC_M_PRT_SW_SEQ_DATA4_OFFSET: c_uint = 0x1058;
// THC SW sequencing Data DW5
pub const THC_M_PRT_SW_SEQ_DATA5_OFFSET: c_uint = 0x105C;
// THC SW sequencing Data DW6
pub const THC_M_PRT_SW_SEQ_DATA6_OFFSET: c_uint = 0x1060;
// THC SW sequencing Data DW7
pub const THC_M_PRT_SW_SEQ_DATA7_OFFSET: c_uint = 0x1064;
// THC SW sequencing Data DW8
pub const THC_M_PRT_SW_SEQ_DATA8_OFFSET: c_uint = 0x1068;
// THC SW sequencing Data DW9
pub const THC_M_PRT_SW_SEQ_DATA9_OFFSET: c_uint = 0x106C;
// THC SW sequencing Data DW10
pub const THC_M_PRT_SW_SEQ_DATA10_OFFSET: c_uint = 0x1070;
// THC SW sequencing Data DW11
pub const THC_M_PRT_SW_SEQ_DATA11_OFFSET: c_uint = 0x1074;
// THC SW sequencing Data DW12
pub const THC_M_PRT_SW_SEQ_DATA12_OFFSET: c_uint = 0x1078;
// THC SW sequencing Data DW13
pub const THC_M_PRT_SW_SEQ_DATA13_OFFSET: c_uint = 0x107C;
// THC SW sequencing Data DW14
pub const THC_M_PRT_SW_SEQ_DATA14_OFFSET: c_uint = 0x1080;
// THC SW sequencing Data DW15
pub const THC_M_PRT_SW_SEQ_DATA15_OFFSET: c_uint = 0x1084;
// THC SW sequencing Data DW16
pub const THC_M_PRT_SW_SEQ_DATA16_OFFSET: c_uint = 0x1088;
// THC Write PRD Base Address Register Low
pub const THC_M_PRT_WPRD_BA_LOW_OFFSET: c_uint = 0x1090;
// THC Write PRD Base Address Register High
pub const THC_M_PRT_WPRD_BA_HI_OFFSET: c_uint = 0x1094;
// THC Write DMA Control
pub const THC_M_PRT_WRITE_DMA_CNTRL_OFFSET: c_uint = 0x1098;
// THC Write Interrupt Status
pub const THC_M_PRT_WRITE_INT_STS_OFFSET: c_uint = 0x109C;
// THC Write DMA Error Register
pub const THC_M_PRT_WRITE_DMA_ERR_OFFSET: c_uint = 0x10A0;
// THC device address for the bulk write
pub const THC_M_PRT_WR_BULK_ADDR_OFFSET: c_uint = 0x10B4;
// THC Device Interrupt Cause Register Address
pub const THC_M_PRT_DEV_INT_CAUSE_ADDR_OFFSET: c_uint = 0x10B8;
// THC Device Interrupt Cause Register Value
pub const THC_M_PRT_DEV_INT_CAUSE_REG_VAL_OFFSET: c_uint = 0x10BC;
// THC TXDMA Frame Count
pub const THC_M_PRT_TX_FRM_CNT_OFFSET: c_uint = 0x10E0;
// THC TXDMA Packet Count
pub const THC_M_PRT_TXDMA_PKT_CNT_OFFSET: c_uint = 0x10E4;
// THC Device Interrupt Count on this port
pub const THC_M_PRT_DEVINT_CNT_OFFSET: c_uint = 0x10E8;
// Touch Device Interrupt Cause register Format Configuration Register 1
pub const THC_M_PRT_DEVINT_CFG_1_OFFSET: c_uint = 0x10EC;
// Touch Device Interrupt Cause register Format Configuration Register 2
pub const THC_M_PRT_DEVINT_CFG_2_OFFSET: c_uint = 0x10F0;
// THC Read PRD Base Address Low for the 1st RXDMA
pub const THC_M_PRT_RPRD_BA_LOW_1_OFFSET: c_uint = 0x1100;
// THC Read PRD Base Address High for the 1st RXDMA
pub const THC_M_PRT_RPRD_BA_HI_1_OFFSET: c_uint = 0x1104;
// THC Read PRD Control for the 1st RXDMA
pub const THC_M_PRT_RPRD_CNTRL_1_OFFSET: c_uint = 0x1108;
// THC Read DMA Control for the 1st RXDMA
pub const THC_M_PRT_READ_DMA_CNTRL_1_OFFSET: c_uint = 0x110C;
// THC Read Interrupt Status for the 1st RXDMA
pub const THC_M_PRT_READ_DMA_INT_STS_1_OFFSET: c_uint = 0x1110;
// THC Read DMA Error Register for the 1st RXDMA
pub const THC_M_PRT_READ_DMA_ERR_1_OFFSET: c_uint = 0x1114;
// Touch Sequencer GuC Tail Offset Address Low for the 1st RXDMA
pub const THC_M_PRT_GUC_OFFSET_LOW_1_OFFSET: c_uint = 0x1118;
// Touch Sequencer GuC Tail Offset Address High for the 1st RXDMA
pub const THC_M_PRT_GUC_OFFSET_HI_1_OFFSET: c_uint = 0x111C;
// Touch Host Controller GuC Work Queue Item Size for the 1st RXDMA
pub const THC_M_PRT_GUC_WORKQ_ITEM_SZ_1_OFFSET: c_uint = 0x1120;
// Touch Host Controller GuC Control register for the 1st RXDMA
pub const THC_M_PRT_GUC_WORKQ_SZ_1_OFFSET: c_uint = 0x1124;
// Touch Sequencer Control for the 1st DMA
pub const THC_M_PRT_TSEQ_CNTRL_1_OFFSET: c_uint = 0x1128;
// Touch Sequencer GuC Doorbell Address Low for the 1st RXDMA
pub const THC_M_PRT_GUC_DB_ADDR_LOW_1_OFFSET: c_uint = 0x1130;
// Touch Sequencer GuC Doorbell Address High for the 1st RXDMA
pub const THC_M_PRT_GUC_DB_ADDR_HI_1_OFFSET: c_uint = 0x1134;
// Touch Sequencer GuC Doorbell Data
pub const THC_M_PRT_GUC_DB_DATA_1_OFFSET: c_uint = 0x1138;
// Touch Sequencer GuC Tail Offset Initial Value for the 1st RXDMA
pub const THC_M_PRT_GUC_OFFSET_INITVAL_1_OFFSET: c_uint = 0x1140;
// THC Device Address for the bulk/touch data read for the 1st RXDMA
pub const THC_M_PRT_RD_BULK_ADDR_1_OFFSET: c_uint = 0x1170;
// THC Gfx/SW Doorbell Count from the 1st Stream RXDMA on this port
pub const THC_M_PRT_DB_CNT_1_OFFSET: c_uint = 0x11A0;
// THC Frame Count from the 1st Stream RXDMA on this port
pub const THC_M_PRT_FRM_CNT_1_OFFSET: c_uint = 0x11A4;
// THC Micro Frame Count from the 1st Stream RXDMA on this port
pub const THC_M_PRT_UFRM_CNT_1_OFFSET: c_uint = 0x11A8;
// THC Packet Count from the 1st Stream RXDMA on this port
pub const THC_M_PRT_RXDMA_PKT_CNT_1_OFFSET: c_uint = 0x11AC;
//
// THC Software Interrupt Count from the 1st Stream RXDMA
// on this port
//
pub const THC_M_PRT_SWINT_CNT_1_OFFSET: c_uint = 0x11B0;
// Touch Sequencer Frame Drop Counter for the 1st RXDMA
pub const THC_M_PRT_FRAME_DROP_CNT_1_OFFSET: c_uint = 0x11B4;
// THC Coaescing 1
pub const THC_M_PRT_COALESCE_1_OFFSET: c_uint = 0x11B8;
// THC Read PRD Base Address Low for the 2nd RXDMA
pub const THC_M_PRT_RPRD_BA_LOW_2_OFFSET: c_uint = 0x1200;
// THC Read PRD Base Address High for the 2nd RXDMA
pub const THC_M_PRT_RPRD_BA_HI_2_OFFSET: c_uint = 0x1204;
// THC Read PRD Control for the 2nd RXDMA
pub const THC_M_PRT_RPRD_CNTRL_2_OFFSET: c_uint = 0x1208;
// THC Read DMA Control for the 2nd RXDMA
pub const THC_M_PRT_READ_DMA_CNTRL_2_OFFSET: c_uint = 0x120C;
// THC Read Interrupt Status for the 2nd RXDMA
pub const THC_M_PRT_READ_DMA_INT_STS_2_OFFSET: c_uint = 0x1210;
// THC Read DMA Error Register for the 2nd RXDMA
pub const THC_M_PRT_READ_DMA_ERR_2_OFFSET: c_uint = 0x1214;
// Touch Sequencer GuC Tail Offset Address Low for the 2nd RXDMA
pub const THC_M_PRT_GUC_OFFSET_LOW_2_OFFSET: c_uint = 0x1218;
// Touch Sequencer GuC Tail Offset Address High for the 2nd RXDMA
pub const THC_M_PRT_GUC_OFFSET_HI_2_OFFSET: c_uint = 0x121C;
// Touch Host Controller GuC Work Queue Item Size for the 2nd RXDMA
pub const THC_M_PRT_GUC_WORKQ_ITEM_SZ_2_OFFSET: c_uint = 0x1220;
// Touch Host Controller GuC Control register for the 2nd RXDMA
pub const THC_M_PRT_GUC_WORKQ_SZ_2_OFFSET: c_uint = 0x1224;
// Touch Sequencer Control for the 2nd DMA
pub const THC_M_PRT_TSEQ_CNTRL_2_OFFSET: c_uint = 0x1228;
// Touch Sequencer GuC Doorbell Address Low for the 2nd RXDMA
pub const THC_M_PRT_GUC_DB_ADDR_LOW_2_OFFSET: c_uint = 0x1230;
// Touch Sequencer GuC Doorbell Address High for the 2nd RXDMA
pub const THC_M_PRT_GUC_DB_ADDR_HI_2_OFFSET: c_uint = 0x1234;
// Touch Sequencer GuC Doorbell Data for PRD2
pub const THC_M_PRT_GUC_DB_DATA_2_OFFSET: c_uint = 0x1238;
// Touch Sequencer GuC Tail Offset Initial Value for the 2nd RXDMA
pub const THC_M_PRT_GUC_OFFSET_INITVAL_2_OFFSET: c_uint = 0x1240;
// THC Device Address for the bulk/touch data read for the 2nd RXDMA
pub const THC_M_PRT_RD_BULK_ADDR_2_OFFSET: c_uint = 0x1270;
// THC Gfx/SW Doorbell Count from the 2nd Stream RXDMA on this port
pub const THC_M_PRT_DB_CNT_2_OFFSET: c_uint = 0x12A0;
// THC Frame Count from the 2nd Stream RXDMA on this port
pub const THC_M_PRT_FRM_CNT_2_OFFSET: c_uint = 0x12A4;
// THC Micro Frame Count from the 2nd Stream RXDMA on this port
pub const THC_M_PRT_UFRM_CNT_2_OFFSET: c_uint = 0x12A8;
// THC Packet Count from the 2nd Stream RXDMA on this port
pub const THC_M_PRT_RXDMA_PKT_CNT_2_OFFSET: c_uint = 0x12AC;
//
// THC Software Interrupt Count from the 2nd Stream RXDMA
// on this port
//
pub const THC_M_PRT_SWINT_CNT_2_OFFSET: c_uint = 0x12B0;
// Touch Sequencer Frame Drop Counter for the 2nd RXDMA
pub const THC_M_PRT_FRAME_DROP_CNT_2_OFFSET: c_uint = 0x12B4;
// THC Coaescing 2
pub const THC_M_PRT_COALESCE_2_OFFSET: c_uint = 0x12B8;
// THC SPARE REGISTER
pub const THC_M_PRT_SPARE_REG_OFFSET: c_uint = 0x12BC;
// THC Read PRD Base Address Low for the SW RXDMA
pub const THC_M_PRT_RPRD_BA_LOW_SW_OFFSET: c_uint = 0x12C0;
// THC Read PRD Base Address High for the SW RXDMA
pub const THC_M_PRT_RPRD_BA_HI_SW_OFFSET: c_uint = 0x12C4;
// THC Read PRD Control for the SW RXDMA
pub const THC_M_PRT_RPRD_CNTRL_SW_OFFSET: c_uint = 0x12C8;
// THC Read DMA Control for the SW RXDMA
pub const THC_M_PRT_READ_DMA_CNTRL_SW_OFFSET: c_uint = 0x12CC;
// THC Read Interrupt Status for the SW RXDMA
pub const THC_M_PRT_READ_DMA_INT_STS_SW_OFFSET: c_uint = 0x12D0;
// Touch Sequencer Control for the SW DMA
pub const THC_M_PRT_TSEQ_CNTRL_SW_OFFSET: c_uint = 0x12D4;
// Address for the bulk read for SW DMA engine
pub const THC_M_PRT_RD_BULK_ADDR_SW_OFFSET: c_uint = 0x12D8;
// THC Frame Count from the SW RXDMA on this port
pub const THC_M_PRT_FRM_CNT_SW_OFFSET: c_uint = 0x12DC;
// THC Packet Count from the SW RXDMA on this port
pub const THC_M_PRT_RXDMA_PKT_CNT_SW_OFFSET: c_uint = 0x12E0;
// SW DMA PRD Table Length
pub const THC_M_PRT_SW_DMA_PRD_TABLE_LEN_OFFSET: c_uint = 0x12E4;
// THC timing based Frame/Interrupt caolescing control register for 1st RXDMA
pub const THC_M_PRT_COALESCE_CNTRL_1_OFFSET: c_uint = 0x12E8;
// THC timing based Frame/Interrupt caolescing control register for 2nd RXDMA
pub const THC_M_PRT_COALESCE_CNTRL_2_OFFSET: c_uint = 0x12EC;
// Touch Sequencer PRD Table Empty Counter for the 1st RXDMA
pub const THC_M_PRT_PRD_EMPTY_CNT_1_OFFSET: c_uint = 0x12F0;
// Touch Sequencer PRD Table Empty Counter for the 2nd RXDM
pub const THC_M_PRT_PRD_EMPTY_CNT_2_OFFSET: c_uint = 0x12F4;
// THC coalescing status to reflect the current coalescing FSM state for 1st RXDMA
pub const THC_M_PRT_COALESCE_STS_1_OFFSET: c_uint = 0x12F8;
// THC coalescing status to reflect the current coalescing FSM state for 2nd RXDMA
pub const THC_M_PRT_COALESCE_STS_2_OFFSET: c_uint = 0x12FC;
// THC Register for the SPI Port Duty Cycle Configuration
pub const THC_M_PRT_SPI_DUTYC_CFG_OFFSET: c_uint = 0x1300;
// THC Register for SW I2C Wtite Sequecning control
pub const THC_M_PRT_SW_SEQ_I2C_WR_CNTRL_OFFSET: c_uint = 0x1304;
// THC current Timestamp Register for RXDMA1
pub const THC_M_PRT_TIMESTAMP_1_OFFSET: c_uint = 0x1308;
// THC current Timestamp Register for RXDMA2
pub const THC_M_PRT_TIMESTAMP_2_OFFSET: c_uint = 0x130C;
// Current SYNC Event Timestamp Register
pub const THC_M_PRT_SYNC_TIMESTAMP_OFFSET: c_uint = 0x1310;
// THC Display Sync Register
pub const THC_M_PRT_DISP_SYNC_OFFSET: c_uint = 0x1314;
// THC Display Sync Register
pub const THC_M_PRT_DISP_SYNC_2_OFFSET: c_uint = 0x1318;
// THC Register for SW I2C Wtite Sequecning control
pub const THC_M_PRT_I2C_CFG_OFFSET: c_uint = 0x131C;
// THC register bits definition

pub const THC_M_CMN_LTR_CTRL_OFFSET: c_uint = 0x14;

// CS Assertion delay default value
pub const THC_CSA_CK_DELAY_VAL_DEFAULT: c_int = 4;
// ARB policy definition
// Arbiter switches on packet boundary
pub const THC_ARB_POLICY_PACKET_BOUNDARY: c_int = 0;
// Arbiter switches on Micro Frame boundary
pub const THC_ARB_POLICY_UFRAME_BOUNDARY: c_int = 1;
// Arbiter switches on Frame boundary
pub const THC_ARB_POLICY_FRAME_BOUNDARY: c_int = 2;

// Default configures for HIDSPI
pub const THC_BIT_OFFSET_INTERRUPT_TYPE: c_int = 4;
// input_report_type is 4 bits for HIDSPI
pub const THC_BIT_LENGTH_INTERRUPT_TYPE: c_int = 4;
// Last fragment indicator is bit 15 for HIDSPI
pub const THC_BIT_OFFSET_LAST_FRAGMENT_FLAG: c_int = 22;
pub const THC_BIT_OFFSET_MICROFRAME_SIZE: c_int = 8;
// input_report_length is 14 bits for HIDSPI
pub const THC_BIT_LENGTH_MICROFRAME_SIZE: c_int = 14;
// MFS unit in power of 2
pub const THC_UNIT_MICROFRAME_SIZE: c_int = 2;
pub const THC_BITMASK_INTERRUPT_TYPE_DATA: c_int = 1;
pub const THC_BITMASK_INVALID_TYPE_DATA: c_int = 2;
// Interrupt Quiesce default timeout value

// LTR definition
//
// THC uses scale to calculate final LTR value.
// Scale is geometric progression of 2^5 step, starting from 2^0.
// For example, THC_LTR_SCALE_2(2) means 2^(5 * 2) = 1024, unit is ns.
//
pub const THC_LTR_SCALE_0: c_int = 0;
pub const THC_LTR_SCALE_1: c_int = 1;
pub const THC_LTR_SCALE_2: c_int = 2;
pub const THC_LTR_SCALE_3: c_int = 3;
pub const THC_LTR_SCALE_4: c_int = 4;
pub const THC_LTR_SCALE_5: c_int = 5;
pub const THC_LTR_MODE_ACTIVE: c_int = 0;
pub const THC_LTR_MODE_LP: c_int = 1;

//
// THC PIO opcode default value
// @THC_PIO_OP_SPI_TIC_READ: THC opcode for SPI PIO read
// @THC_PIO_OP_SPI_TIC_WRITE: THC opcode for SPI PIO write
// @THC_PIO_OP_I2C_SUBSYSTEM_READ: THC opcode for read I2C subsystem registers
// @THC_PIO_OP_I2C_SUBSYSTEM_WRITE: THC opcode for write I2C subsystem registers
// @THC_PIO_OP_I2C_TIC_READ: THC opcode for read I2C device
// @THC_PIO_OP_I2C_TIC_WRITE: THC opcode for write I2C device
// @THC_PIO_OP_I2C_TIC_WRITE_AND_READ: THC opcode for write followed by read I2C device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_pio_opcode {
    THC_PIO_OP_SPI_TIC_READ = 0x4,
    THC_PIO_OP_SPI_TIC_WRITE = 0x6,
    THC_PIO_OP_I2C_SUBSYSTEM_READ = 0x12,
    THC_PIO_OP_I2C_SUBSYSTEM_WRITE = 0x13,
    THC_PIO_OP_I2C_TIC_READ = 0x14,
    THC_PIO_OP_I2C_TIC_WRITE = 0x18,
    THC_PIO_OP_I2C_TIC_WRITE_AND_READ = 0x1C,
}

//
// THC SPI IO mode
// @THC_SINGLE_IO: single IO mode, 1(opcode) - 1(address) - 1(data)
// @THC_DUAL_IO: dual IO mode, 1(opcode) - 2(address) - 2(data)
// @THC_QUAD_IO: quad IO mode, 1(opcode) - 4(address) - 4(data)
// @THC_QUAD_PARALLEL_IO: parallel quad IO mode, 4(opcode) - 4(address) - 4(data)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_spi_iomode {
    THC_SINGLE_IO = 0,
    THC_DUAL_IO = 1,
    THC_QUAD_IO = 2,
    THC_QUAD_PARALLEL_IO = 3,
}

//
// THC SPI frequency divider
//
// This DIV final value is determined by THC_M_PRT_SPI_CFG_SPI_LOW_FREQ_EN bit.
// If THC_M_PRT_SPI_CFG_SPI_LOW_FREQ_EN isn't be set, THC takes the DIV value directly;
// If THC_M_PRT_SPI_CFG_SPI_LOW_FREQ_EN is set, THC takes the DIV value multiply by 8.
//
// For example, if THC input clock is 125MHz:
// When THC_M_PRT_SPI_CFG_SPI_LOW_FREQ_EN isn't set, THC_SPI_FRQ_DIV_3 means DIV is 3,
// THC final clock is 125 / 3 = 41.667MHz;
// When THC_M_PRT_SPI_CFG_SPI_LOW_FREQ_EN is set, THC_SPI_FRQ_DIV_3 means DIV is 3 * 8,
// THC final clock is 125 / (3 * 8) = 5.208MHz;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thc_spi_frq_div {
    THC_SPI_FRQ_RESERVED = 0,
    THC_SPI_FRQ_DIV_1 = 1,
    THC_SPI_FRQ_DIV_2 = 2,
    THC_SPI_FRQ_DIV_3 = 3,
    THC_SPI_FRQ_DIV_4 = 4,
    THC_SPI_FRQ_DIV_5 = 5,
    THC_SPI_FRQ_DIV_6 = 6,
    THC_SPI_FRQ_DIV_7 = 7,
}

// THC I2C sub-system registers
pub const THC_I2C_IC_CON_OFFSET: c_uint = 0x0;
pub const THC_I2C_IC_TAR_OFFSET: c_uint = 0x4;
pub const THC_I2C_IC_SAR_OFFSET: c_uint = 0x8;
pub const THC_I2C_IC_HS_MADDR_OFFSET: c_uint = 0xC;
pub const THC_I2C_IC_DATA_CMD_OFFSET: c_uint = 0x10;
pub const THC_I2C_IC_SS_SCL_HCNT_OFFSET: c_uint = 0x14;
pub const THC_I2C_IC_UFM_SCL_HCNT_OFFSET: c_uint = 0x14;
pub const THC_I2C_IC_SS_SCL_LCNT_OFFSET: c_uint = 0x18;
pub const THC_I2C_IC_UFM_SCL_LCNT_OFFSET: c_uint = 0x18;
pub const THC_I2C_IC_FS_SCL_HCNT_OFFSET: c_uint = 0x1C;
pub const THC_I2C_IC_UFM_TBUF_CNT_OFFSET: c_uint = 0x1C;
pub const THC_I2C_IC_FS_SCL_LCNT_OFFSET: c_uint = 0x20;
pub const THC_I2C_IC_HS_SCL_HCNT_OFFSET: c_uint = 0x24;
pub const THC_I2C_IC_HS_SCL_LCNT_OFFSET: c_uint = 0x28;
pub const THC_I2C_IC_INTR_STAT_OFFSET: c_uint = 0x2C;
pub const THC_I2C_IC_INTR_MASK_OFFSET: c_uint = 0x30;
pub const THC_I2C_IC_RAW_INTR_STAT_OFFSET: c_uint = 0x34;
pub const THC_I2C_IC_RX_TL_OFFSET: c_uint = 0x38;
pub const THC_I2C_IC_TX_TL_OFFSET: c_uint = 0x3C;
pub const THC_I2C_IC_CLR_INTR_OFFSET: c_uint = 0x40;
pub const THC_I2C_IC_CLR_RX_UNDER_OFFSET: c_uint = 0x44;
pub const THC_I2C_IC_CLR_RX_OVER_OFFSET: c_uint = 0x48;
pub const THC_I2C_IC_CLR_TX_OVER_OFFSET: c_uint = 0x4C;
pub const THC_I2C_IC_CLR_RD_REQ_OFFSET: c_uint = 0x50;
pub const THC_I2C_IC_CLR_TX_ABRT_OFFSET: c_uint = 0x54;
pub const THC_I2C_IC_CLR_RX_DONE_OFFSET: c_uint = 0x58;
pub const THC_I2C_IC_CLR_ACTIVITY_OFFSET: c_uint = 0x5C;
pub const THC_I2C_IC_CLR_STOP_DET_OFFSET: c_uint = 0x60;
pub const THC_I2C_IC_CLR_START_DET_OFFSET: c_uint = 0x64;
pub const THC_I2C_IC_CLR_GEN_CALL_OFFSET: c_uint = 0x68;
pub const THC_I2C_IC_ENABLE_OFFSET: c_uint = 0x6C;
pub const THC_I2C_IC_STATUS_OFFSET: c_uint = 0x70;
pub const THC_I2C_IC_TXFLR_OFFSET: c_uint = 0x74;
pub const THC_I2C_IC_RXFLR_OFFSET: c_uint = 0x78;
pub const THC_I2C_IC_SDA_HOLD_OFFSET: c_uint = 0x7C;
pub const THC_I2C_IC_TX_ABRT_SOURCE_OFFSET: c_uint = 0x80;
pub const THC_I2C_IC_SLV_DATA_NACK_ONLY_OFFSET: c_uint = 0x84;
pub const THC_I2C_IC_DMA_CR_OFFSET: c_uint = 0x88;
pub const THC_I2C_IC_DMA_TDLR_OFFSET: c_uint = 0x8C;
pub const THC_I2C_IC_DMA_RDLR_OFFSET: c_uint = 0x90;
pub const THC_I2C_IC_SDA_SETUP_OFFSET: c_uint = 0x94;
pub const THC_I2C_IC_ACK_GENERAL_CALL_OFFSET: c_uint = 0x98;
pub const THC_I2C_IC_ENABLE_STATUS_OFFSET: c_uint = 0x9C;
pub const THC_I2C_IC_FS_SPKLEN_OFFSET: c_uint = 0xA0;
pub const THC_I2C_IC_UFM_SPKLEN_OFFSET: c_uint = 0xA0;
pub const THC_I2C_IC_HS_SPKLEN_OFFSET: c_uint = 0xA4;
pub const THC_I2C_IC_CLR_RESTART_DET_OFFSET: c_uint = 0xA8;
pub const THC_I2C_IC_SCL_STUCK_AT_LOW_TIMEOUT_OFFSET: c_uint = 0xAC;
pub const THC_I2C_IC_SDA_STUCK_AT_LOW_TIMEOUT_OFFSET: c_uint = 0xB0;
pub const THC_I2C_IC_CLR_SCL_STUCK_DET_OFFSET: c_uint = 0xB4;
pub const THC_I2C_IC_DEVICE_ID_OFFSET: c_uint = 0xB8;
pub const THC_I2C_IC_SMBUS_CLK_LOW_SEXT_OFFSET: c_uint = 0xBC;
pub const THC_I2C_IC_SMBUS_CLK_LOW_MEXT_OFFSET: c_uint = 0xC0;
pub const THC_I2C_IC_SMBUS_THIGH_MAX_IDLE_COUNT_OFFSET: c_uint = 0xC4;
pub const THC_I2C_IC_SMBUS_INTR_STAT_OFFSET: c_uint = 0xC8;
pub const THC_I2C_IC_SMBUS_INTR_MASK_OFFSET: c_uint = 0xCC;
pub const THC_I2C_IC_SMBUS_RAW_INTR_STAT_OFFSET: c_uint = 0xD0;
pub const THC_I2C_IC_CLR_SMBUS_INTR_OFFSET: c_uint = 0xD4;
pub const THC_I2C_IC_OPTIONAL_SAR_OFFSET: c_uint = 0xD8;
pub const THC_I2C_IC_SMBUS_UDID_LSB_OFFSET: c_uint = 0xDC;
pub const THC_I2C_IC_SMBUS_UDID_WORD0_OFFSET: c_uint = 0xDC;
pub const THC_I2C_IC_SMBUS_UDID_WORD1_OFFSET: c_uint = 0xE0;
pub const THC_I2C_IC_SMBUS_UDID_WORD2_OFFSET: c_uint = 0xE4;
pub const THC_I2C_IC_SMBUS_UDID_WORD3_OFFSET: c_uint = 0xE8;
pub const THC_I2C_IC_COMP_PARAM_1_OFFSET: c_uint = 0xF4;
pub const THC_I2C_IC_COMP_VERSION_OFFSET: c_uint = 0xF8;
pub const THC_I2C_IC_COMP_TYPE_OFFSET: c_uint = 0xFC;
//
// THC I2C sub-system supported speed mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum THC_I2C_SPEED_MODE {
    THC_I2C_STANDARD = 1,
    THC_I2C_FAST_AND_PLUS = 2,
    THC_I2C_HIGH_SPEED = 3,
}

// THC I2C sub-system register bits definition

