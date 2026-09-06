//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ras.h
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


//
// Copyright 2018 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const AMDGPU_RAS_BOOT_STATUS_POLLING_LIMIT: c_int = 100;
pub const AMDGPU_RAS_BOOT_STEADY_STATUS: c_uint = 0xBA;
pub const AMDGPU_RAS_BOOT_STATUS_MASK: c_uint = 0xFF;

// position of instance value in sub_block_index of
// ta_ras_trigger_error_input, the sub block uses lower 12 bits
//
pub const AMDGPU_RAS_INST_MASK: c_uint = 0xfffff000;
pub const AMDGPU_RAS_INST_SHIFT: c_uint = 0xc;
pub const AMDGPU_RAS_FEATURES_SOCKETID_SHIFT: c_int = 29;
pub const AMDGPU_RAS_FEATURES_SOCKETID_MASK: c_uint = 0xe0000000;
// Reserve 8 physical dram row for possible retirement.
// In worst cases, it will lose 8 * 2MB memory in vram domain

// The high three bits indicates socketid

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_block {
    AMDGPU_RAS_BLOCK__UMC = 0,
    AMDGPU_RAS_BLOCK__SDMA,
    AMDGPU_RAS_BLOCK__GFX,
    AMDGPU_RAS_BLOCK__MMHUB,
    AMDGPU_RAS_BLOCK__ATHUB,
    AMDGPU_RAS_BLOCK__PCIE_BIF,
    AMDGPU_RAS_BLOCK__HDP,
    AMDGPU_RAS_BLOCK__XGMI_WAFL,
    AMDGPU_RAS_BLOCK__DF,
    AMDGPU_RAS_BLOCK__SMN,
    AMDGPU_RAS_BLOCK__SEM,
    AMDGPU_RAS_BLOCK__MP0,
    AMDGPU_RAS_BLOCK__MP1,
    AMDGPU_RAS_BLOCK__FUSE,
    AMDGPU_RAS_BLOCK__MCA,
    AMDGPU_RAS_BLOCK__VCN,
    AMDGPU_RAS_BLOCK__JPEG,
    AMDGPU_RAS_BLOCK__IH,
    AMDGPU_RAS_BLOCK__MPIO,
    AMDGPU_RAS_BLOCK__MMSCH,

    AMDGPU_RAS_BLOCK__LAST,
    AMDGPU_RAS_BLOCK__ANY = -1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_mca_block {
    AMDGPU_RAS_MCA_BLOCK__MP0 = 0,
    AMDGPU_RAS_MCA_BLOCK__MP1,
    AMDGPU_RAS_MCA_BLOCK__MPIO,
    AMDGPU_RAS_MCA_BLOCK__IOHC,

    AMDGPU_RAS_MCA_BLOCK__LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_gfx_subblock {
// CPC
    AMDGPU_RAS_BLOCK__GFX_CPC_INDEX_START = 0,
    AMDGPU_RAS_BLOCK__GFX_CPC_SCRATCH =
    AMDGPU_RAS_BLOCK__GFX_CPC_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_CPC_UCODE,
    AMDGPU_RAS_BLOCK__GFX_DC_STATE_ME1,
    AMDGPU_RAS_BLOCK__GFX_DC_CSINVOC_ME1,
    AMDGPU_RAS_BLOCK__GFX_DC_RESTORE_ME1,
    AMDGPU_RAS_BLOCK__GFX_DC_STATE_ME2,
    AMDGPU_RAS_BLOCK__GFX_DC_CSINVOC_ME2,
    AMDGPU_RAS_BLOCK__GFX_DC_RESTORE_ME2,
    AMDGPU_RAS_BLOCK__GFX_CPC_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_DC_RESTORE_ME2,
// CPF
    AMDGPU_RAS_BLOCK__GFX_CPF_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_CPF_ROQ_ME2 =
    AMDGPU_RAS_BLOCK__GFX_CPF_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_CPF_ROQ_ME1,
    AMDGPU_RAS_BLOCK__GFX_CPF_TAG,
    AMDGPU_RAS_BLOCK__GFX_CPF_INDEX_END = AMDGPU_RAS_BLOCK__GFX_CPF_TAG,
// CPG
    AMDGPU_RAS_BLOCK__GFX_CPG_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_CPG_DMA_ROQ =
    AMDGPU_RAS_BLOCK__GFX_CPG_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_CPG_DMA_TAG,
    AMDGPU_RAS_BLOCK__GFX_CPG_TAG,
    AMDGPU_RAS_BLOCK__GFX_CPG_INDEX_END = AMDGPU_RAS_BLOCK__GFX_CPG_TAG,
// GDS
    AMDGPU_RAS_BLOCK__GFX_GDS_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_GDS_MEM = AMDGPU_RAS_BLOCK__GFX_GDS_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_GDS_INPUT_QUEUE,
    AMDGPU_RAS_BLOCK__GFX_GDS_OA_PHY_CMD_RAM_MEM,
    AMDGPU_RAS_BLOCK__GFX_GDS_OA_PHY_DATA_RAM_MEM,
    AMDGPU_RAS_BLOCK__GFX_GDS_OA_PIPE_MEM,
    AMDGPU_RAS_BLOCK__GFX_GDS_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_GDS_OA_PIPE_MEM,
// SPI
    AMDGPU_RAS_BLOCK__GFX_SPI_SR_MEM,
// SQ
    AMDGPU_RAS_BLOCK__GFX_SQ_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_SQ_SGPR = AMDGPU_RAS_BLOCK__GFX_SQ_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_SQ_LDS_D,
    AMDGPU_RAS_BLOCK__GFX_SQ_LDS_I,
    AMDGPU_RAS_BLOCK__GFX_SQ_VGPR,
    AMDGPU_RAS_BLOCK__GFX_SQ_INDEX_END = AMDGPU_RAS_BLOCK__GFX_SQ_VGPR,
// SQC (3 ranges)
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX_START,
// SQC range 0
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX0_START =
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_UTCL1_LFIFO =
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX0_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU0_WRITE_DATA_BUF,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU0_UTCL1_LFIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU1_WRITE_DATA_BUF,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU1_UTCL1_LFIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU2_WRITE_DATA_BUF,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU2_UTCL1_LFIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX0_END =
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_CU2_UTCL1_LFIFO,
// SQC range 1
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKA_TAG_RAM =
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKA_UTCL1_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKA_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKA_BANK_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_TAG_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_HIT_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_DIRTY_BIT_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_BANK_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX1_END =
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKA_BANK_RAM,
// SQC range 2
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKB_TAG_RAM =
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKB_UTCL1_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKB_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_INST_BANKB_BANK_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_TAG_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_HIT_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_MISS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_DIRTY_BIT_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_BANK_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX2_END =
    AMDGPU_RAS_BLOCK__GFX_SQC_DATA_BANKB_BANK_RAM,
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_SQC_INDEX2_END,
// TA
    AMDGPU_RAS_BLOCK__GFX_TA_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TA_FS_DFIFO =
    AMDGPU_RAS_BLOCK__GFX_TA_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TA_FS_AFIFO,
    AMDGPU_RAS_BLOCK__GFX_TA_FL_LFIFO,
    AMDGPU_RAS_BLOCK__GFX_TA_FX_LFIFO,
    AMDGPU_RAS_BLOCK__GFX_TA_FS_CFIFO,
    AMDGPU_RAS_BLOCK__GFX_TA_INDEX_END = AMDGPU_RAS_BLOCK__GFX_TA_FS_CFIFO,
// TCA
    AMDGPU_RAS_BLOCK__GFX_TCA_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TCA_HOLE_FIFO =
    AMDGPU_RAS_BLOCK__GFX_TCA_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TCA_REQ_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCA_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_TCA_REQ_FIFO,
// TCC (5 sub-ranges)
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX_START,
// TCC range 0
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX0_START =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DATA =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX0_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DATA_BANK_0_1,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DATA_BANK_1_0,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DATA_BANK_1_1,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DIRTY_BANK_0,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_DIRTY_BANK_1,
    AMDGPU_RAS_BLOCK__GFX_TCC_HIGH_RATE_TAG,
    AMDGPU_RAS_BLOCK__GFX_TCC_LOW_RATE_TAG,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX0_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_LOW_RATE_TAG,
// TCC range 1
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_IN_USE_DEC =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_IN_USE_TRANSFER,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX1_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_IN_USE_TRANSFER,
// TCC range 2
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_RETURN_DATA =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_RETURN_CONTROL,
    AMDGPU_RAS_BLOCK__GFX_TCC_UC_ATOMIC_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCC_WRITE_RETURN,
    AMDGPU_RAS_BLOCK__GFX_TCC_WRITE_CACHE_READ,
    AMDGPU_RAS_BLOCK__GFX_TCC_SRC_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCC_SRC_FIFO_NEXT_RAM,
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_TAG_PROBE_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX2_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_CACHE_TAG_PROBE_FIFO,
// TCC range 3
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX3_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_LATENCY_FIFO =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX3_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_LATENCY_FIFO_NEXT_RAM,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX3_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_LATENCY_FIFO_NEXT_RAM,
// TCC range 4
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX4_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_WRRET_TAG_WRITE_RETURN =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX4_START,
    AMDGPU_RAS_BLOCK__GFX_TCC_ATOMIC_RETURN_BUFFER,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX4_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_ATOMIC_RETURN_BUFFER,
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_TCC_INDEX4_END,
// TCI
    AMDGPU_RAS_BLOCK__GFX_TCI_WRITE_RAM,
// TCP
    AMDGPU_RAS_BLOCK__GFX_TCP_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TCP_CACHE_RAM =
    AMDGPU_RAS_BLOCK__GFX_TCP_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TCP_LFIFO_RAM,
    AMDGPU_RAS_BLOCK__GFX_TCP_CMD_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCP_VM_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TCP_DB_RAM,
    AMDGPU_RAS_BLOCK__GFX_TCP_UTCL1_LFIFO0,
    AMDGPU_RAS_BLOCK__GFX_TCP_UTCL1_LFIFO1,
    AMDGPU_RAS_BLOCK__GFX_TCP_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_TCP_UTCL1_LFIFO1,
// TD
    AMDGPU_RAS_BLOCK__GFX_TD_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TD_SS_FIFO_LO =
    AMDGPU_RAS_BLOCK__GFX_TD_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_TD_SS_FIFO_HI,
    AMDGPU_RAS_BLOCK__GFX_TD_CS_FIFO,
    AMDGPU_RAS_BLOCK__GFX_TD_INDEX_END = AMDGPU_RAS_BLOCK__GFX_TD_CS_FIFO,
// EA (3 sub-ranges)
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX_START,
// EA range 0
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX0_START =
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX_START,
    AMDGPU_RAS_BLOCK__GFX_EA_DRAMRD_CMDMEM =
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX0_START,
    AMDGPU_RAS_BLOCK__GFX_EA_DRAMWR_CMDMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_DRAMWR_DATAMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_RRET_TAGMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_WRET_TAGMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_GMIRD_CMDMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_GMIWR_CMDMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_GMIWR_DATAMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX0_END =
    AMDGPU_RAS_BLOCK__GFX_EA_GMIWR_DATAMEM,
// EA range 1
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_EA_DRAMRD_PAGEMEM =
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX1_START,
    AMDGPU_RAS_BLOCK__GFX_EA_DRAMWR_PAGEMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_IORD_CMDMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_IOWR_CMDMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_IOWR_DATAMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_GMIRD_PAGEMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_GMIWR_PAGEMEM,
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX1_END =
    AMDGPU_RAS_BLOCK__GFX_EA_GMIWR_PAGEMEM,
// EA range 2
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_EA_MAM_D0MEM =
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX2_START,
    AMDGPU_RAS_BLOCK__GFX_EA_MAM_D1MEM,
    AMDGPU_RAS_BLOCK__GFX_EA_MAM_D2MEM,
    AMDGPU_RAS_BLOCK__GFX_EA_MAM_D3MEM,
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX2_END =
    AMDGPU_RAS_BLOCK__GFX_EA_MAM_D3MEM,
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX_END =
    AMDGPU_RAS_BLOCK__GFX_EA_INDEX2_END,
// UTC VM L2 bank
    AMDGPU_RAS_BLOCK__UTC_VML2_BANK_CACHE,
// UTC VM walker
    AMDGPU_RAS_BLOCK__UTC_VML2_WALKER,
// UTC ATC L2 2MB cache
    AMDGPU_RAS_BLOCK__UTC_ATCL2_CACHE_2M_BANK,
// UTC ATC L2 4KB cache
    AMDGPU_RAS_BLOCK__UTC_ATCL2_CACHE_4K_BANK,
    AMDGPU_RAS_BLOCK__GFX_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_error_type {
    AMDGPU_RAS_ERROR__NONE							= 0,
    AMDGPU_RAS_ERROR__PARITY						= 1,
    AMDGPU_RAS_ERROR__SINGLE_CORRECTABLE					= 2,
    AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE					= 4,
    AMDGPU_RAS_ERROR__POISON						= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_ret {
    AMDGPU_RAS_SUCCESS = 0,
    AMDGPU_RAS_FAIL,
    AMDGPU_RAS_UE,
    AMDGPU_RAS_CE,
    AMDGPU_RAS_PT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_error_query_mode {
    AMDGPU_RAS_INVALID_ERROR_QUERY		= 0,
    AMDGPU_RAS_DIRECT_ERROR_QUERY		= 1,
    AMDGPU_RAS_FIRMWARE_ERROR_QUERY		= 2,
    AMDGPU_RAS_VIRT_ERROR_COUNT_QUERY	= 3,
}

// ras error status reisger fields
pub const ERR_STATUS_LO__ERR_STATUS_VALID_FLAG__SHIFT: c_uint = 0x0;
pub const ERR_STATUS_LO__ERR_STATUS_VALID_FLAG_MASK: c_uint = 0x00000001L;
pub const ERR_STATUS_LO__MEMORY_ID__SHIFT: c_uint = 0x18;
pub const ERR_STATUS_LO__MEMORY_ID_MASK: c_uint = 0xFF000000L;
pub const ERR_STATUS_HI__ERR_INFO_VALID_FLAG__SHIFT: c_uint = 0x2;
pub const ERR_STATUS_HI__ERR_INFO_VALID_FLAG_MASK: c_uint = 0x00000004L;
pub const ERR_STATUS__ERR_CNT__SHIFT: c_uint = 0x17;
pub const ERR_STATUS__ERR_CNT_MASK: c_uint = 0x03800000L;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_err_status_reg_entry {
    pub hwip: u32,
    pub ip_inst: u32,
    pub seg_lo: u32,
    pub reg_lo: u32,
    pub seg_hi: u32,
    pub reg_hi: u32,
    pub reg_inst: u32,
    pub flags: u32,
    pub block_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_memory_id_entry {
    pub memory_id: u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_common_if {
    pub block: amdgpu_ras_block,
    pub type: amdgpu_ras_error_type,
    pub sub_block_index: u32,
    pub name: [c_char; 32],
}

pub const MAX_UMC_CHANNEL_NUM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_info_per_ch {
    pub ce_count_lo_chip: u16,
    pub ce_count_hi_chip: u16,
    pub mca_umc_status: u64,
    pub mca_umc_addr: u64,
    pub mca_ceumc_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umc_ecc_info {
    pub ecc: [ecc_info_per_ch; MAX_UMC_CHANNEL_NUM],
// Determine smu ecctable whether support
// record correctable error address
//
    pub record_ce_addr_supported: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_event_type {
    RAS_EVENT_TYPE_INVALID = 0,
    RAS_EVENT_TYPE_FATAL,
    RAS_EVENT_TYPE_POISON_CREATION,
    RAS_EVENT_TYPE_POISON_CONSUMPTION,
    RAS_EVENT_TYPE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_event_state {
    pub last_seqno: u64,
    pub count: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_event_manager {
    pub seqno: core::sync::atomic::AtomicI64,
    pub event_state: [ras_event_state; RAS_EVENT_TYPE_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_event_id {
    pub type: ras_event_type,
    pub event_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_query_context {
    pub evid: ras_event_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_err_pages {
    pub count: u32,
    pub pfn: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ecc_err {
    pub status: u64,
    pub ipid: u64,
    pub addr: u64,
    pub pa_pfn: u64,
// save global channel index across all UMC instances
    pub channel_idx: u32,
    pub err_pages: ras_err_pages,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ecc_log_info {
    pub lock: mutex,
    pub de_page_tree: radix_tree_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_critical_region {
    pub node: list_head,
    pub bo: *mut amdgpu_bo,
    pub start: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras {
    pub ras_mgr: *mut c_void,
// ras infrastructure
// for ras itself.
    pub features: u32,
    pub schema: u32,
    pub head: list_head,
// sysfs
    pub features_attr: device_attribute,
    pub version_attr: device_attribute,
    pub schema_attr: device_attribute,
    pub event_state_attr: device_attribute,
    pub badpages_attr: bin_attribute,
    pub de_ras_eeprom_table: *mut dentry,
// block array
    pub objs: *mut ras_manager,
// gpu recovery
    pub recovery_work: work_struct,
    pub in_recovery: core::sync::atomic::AtomicI32,
    pub adev: *mut amdgpu_device,
// error handler data
    pub eh_data: *mut ras_err_handler_data,
    pub recovery_lock: mutex,
    pub flags: u32,
    pub reboot: bool,
    pub eeprom_control: amdgpu_ras_eeprom_control,
    pub error_query_ready: bool,
// bad page count threshold
    pub bad_page_cnt_threshold: u32,
// disable ras error count harvest in recovery
    pub disable_ras_err_cnt_harvest: bool,
// is poison mode supported
    pub poison_supported: bool,
// RAS count errors delayed work
    pub ras_counte_delay_work: delayed_work,
    pub ras_ue_count: core::sync::atomic::AtomicI32,
    pub ras_ce_count: core::sync::atomic::AtomicI32,
// record umc error info queried from smu
    pub umc_ecc: umc_ecc_info,
// Indicates smu whether need update bad channel info
    pub update_channel_flag: bool,
// Record status of smu mca debug mode
    pub is_mca_debug_mode: bool,
    pub is_rma: bool,
// Record special requirements of gpu reset caller
    pub gpu_reset_flags: u32,
    pub page_retirement_lock: mutex,
    pub page_rsv_lock: mutex,
    pub umc_ecc_log: ras_ecc_log_info,
// ras errors detected
    pub ras_err_state: c_ulong,
// RAS event manager
    pub __event_mgr: ras_event_manager,
    pub event_mgr: *mut ras_event_manager,
    pub reserved_pages_in_bytes: u64,
    pub init_task_pid: pid_t,
    pub init_task_comm: [c_char; TASK_COMM_LEN],
    pub bad_page_num: c_int,
    pub critical_region_head: list_head,
    pub critical_region_lock: mutex,
// Disable/Enable uniras switch
    pub uniras_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_fs_data {
    pub sysfs_name: [c_char; 48],
    pub debugfs_name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_err_info {
    pub mcm_info: amdgpu_smuio_mcm_config_info,
    pub ce_count: u64,
    pub ue_count: u64,
    pub de_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_err_node {
    pub node: list_head,
    pub err_info: ras_err_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_err_data {
    pub ue_count: c_ulong,
    pub ce_count: c_ulong,
    pub de_count: c_ulong,
    pub err_addr_cnt: c_ulong,
    pub err_addr: *mut eeprom_table_record,
    pub err_addr_len: c_ulong,
    pub err_list_count: u32,
    pub err_node_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_err_handler_data {
// point to bad page records array
    pub bps: *mut eeprom_table_record,
// the count of entries
    pub count: c_int,
    pub count_saved: c_int,
// the space can place new entries
    pub space_left: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ih_data {
// interrupt bottom half
    pub ih_work: work_struct,
    pub inuse: c_int,
// IP callback
    pub cb: ras_ih_cb,
// full of entries
    pub ring: *mut c_uchar,
    pub ring_size: c_uint,
    pub element_size: c_uint,
    pub aligned_element_size: c_uint,
    pub rptr: c_uint,
    pub wptr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_manager {
    pub head: ras_common_if,
// reference count
    pub use: c_int,
// ras block link
    pub node: list_head,
// the device
    pub adev: *mut amdgpu_device,
// sysfs
    pub sysfs_attr: device_attribute,
    pub attr_inuse: c_int,
// fs node name
    pub fs_data: ras_fs_data,
// IH data
    pub ih_data: ras_ih_data,
    pub err_data: ras_err_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_badpage {
    pub bp: c_uint,
    pub size: c_uint,
    pub flags: c_uint,
}

// interfaces for IP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_fs_if {
    pub head: ras_common_if,
    pub sysfs_name: *const *const c_char,
    pub debugfs_name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_query_if {
    pub head: ras_common_if,
    pub ue_count: c_ulong,
    pub ce_count: c_ulong,
    pub de_count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_inject_if {
    pub head: ras_common_if,
    pub address: u64,
    pub value: u64,
    pub instance_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cure_if {
    pub head: ras_common_if,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ih_if {
    pub head: ras_common_if,
    pub cb: ras_ih_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_dispatch_if {
    pub head: ras_common_if,
    pub entry: *mut amdgpu_iv_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_debug_if {
    pub head: ras_common_if,
    pub inject: ras_inject_if,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_block_object {
    pub ras_comm: ras_common_if,
    pub sub_block_index): amdgpu_ras_block block, uint32_t,
    pub ras_block): *mut *mut *mut int (ras_late_init)(struct amdgpu_device adev, struct ras_common_if,
    pub ras_block): *mut *mut *mut void (ras_suspend)(struct amdgpu_device adev, struct ras_common_if,
    pub ras_block): *mut *mut *mut void (ras_fini)(struct amdgpu_device adev, struct ras_common_if,
    pub ras_cb: ras_ih_cb,
    pub hw_ops: *const amdgpu_ras_block_hw_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_block_hw_ops {
    pub instance_mask): *mut *mut void inject_if, uint32_t,
    pub ras_error_status): *mut *mut *mut void (query_ras_error_count)(struct amdgpu_device adev, void,
    pub adev): *mut *mut void (query_ras_error_status)(struct amdgpu_device,
    pub ras_error_status): *mut *mut *mut void (query_ras_error_address)(struct amdgpu_device adev, void,
    pub adev): *mut *mut void (reset_ras_error_count)(struct amdgpu_device,
    pub adev): *mut *mut void (reset_ras_error_status)(struct amdgpu_device,
    pub adev): *mut *mut bool (query_poison_status)(struct amdgpu_device,
    pub adev): *mut *mut bool (handle_poison_consumption)(struct amdgpu_device,
}

// work flow
// vbios
// 1: ras feature enable (enabled by default)
// psp
// 2: ras framework init (in ip_init)
// IP
// 3: IH add
// 4: debugfs/sysfs create
// 5: query/inject
// 6: debugfs/sysfs remove
// 7: IH remove
// 8: feature disable
//
extern "C" {
    pub fn amdgpu_ras_init_badpage_info(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_recovery_init(adev: *mut amdgpu_device, init_bp_info: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_resume(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ras_suspend(adev: *mut amdgpu_device);
}
// error handling functions
// called in ip_init and ip_fini
extern "C" {
    pub fn amdgpu_ras_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_late_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_pre_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_debugfs_create_all(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ras_global_ras_isr(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_set_error_query_ready(adev: *mut amdgpu_device, ready: bool);
}
extern "C" {
    pub fn amdgpu_ras_need_emergency_restart(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_release_ras_context(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_persistent_edc_harvesting_supported(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_is_poison_mode_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_is_supported(adev: *mut amdgpu_device, block: c_uint) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_reset_gpu(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_get_context(adev: *mut amdgpu_device) -> *mut amdgpu_ras;
}
extern "C" {
    pub fn amdgpu_ras_set_context(adev: *mut amdgpu_device, ras_con: *mut amdgpu_ras) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_set_mca_debug_mode(adev: *mut amdgpu_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_get_mca_debug_mode(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_interrupt_fatal_error_handler(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ras_get_error_type_name(err_type: u32, err_type_name: *mut c_char);
}
extern "C" {
    pub fn amdgpu_ras_error_data_init(err_data: *mut ras_err_data) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_error_data_fini(err_data: *mut ras_err_data);
}
extern "C" {
    pub fn amdgpu_ras_query_boot_status(adev: *mut amdgpu_device, num_instances: u32);
}
extern "C" {
    pub fn amdgpu_ras_set_fed(adev: *mut amdgpu_device, status: bool);
}
extern "C" {
    pub fn amdgpu_ras_get_fed_status(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_clear_err_state(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ras_is_err_state(adev: *mut amdgpu_device, block: c_int) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_acquire_event_id(adev: *mut amdgpu_device, type: ras_event_type) -> u64;
}
extern "C" {
    pub fn amdgpu_ras_reserve_page(adev: *mut amdgpu_device, pfn: u64) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_add_critical_region(adev: *mut amdgpu_device, bo: *mut amdgpu_bo) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_check_critical_address(adev: *mut amdgpu_device, addr: u64) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_in_recovery(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_is_rma(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_resume_after_reset(adev: *mut amdgpu_device) -> c_int;
}
