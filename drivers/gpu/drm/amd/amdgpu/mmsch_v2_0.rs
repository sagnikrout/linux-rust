//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/mmsch_v2_0.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
// addressBlock: uvd0_mmsch_dec
// base address: 0x1e000
pub const mmMMSCH_UCODE_ADDR: c_uint = 0x0000;
pub const mmMMSCH_UCODE_ADDR_BASE_IDX: c_int = 0;
pub const mmMMSCH_UCODE_DATA: c_uint = 0x0001;
pub const mmMMSCH_UCODE_DATA_BASE_IDX: c_int = 0;
pub const mmMMSCH_SRAM_ADDR: c_uint = 0x0002;
pub const mmMMSCH_SRAM_ADDR_BASE_IDX: c_int = 0;
pub const mmMMSCH_SRAM_DATA: c_uint = 0x0003;
pub const mmMMSCH_SRAM_DATA_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_SRAM_OFFSET: c_uint = 0x0004;
pub const mmMMSCH_VF_SRAM_OFFSET_BASE_IDX: c_int = 0;
pub const mmMMSCH_DB_SRAM_OFFSET: c_uint = 0x0005;
pub const mmMMSCH_DB_SRAM_OFFSET_BASE_IDX: c_int = 0;
pub const mmMMSCH_CTX_SRAM_OFFSET: c_uint = 0x0006;
pub const mmMMSCH_CTX_SRAM_OFFSET_BASE_IDX: c_int = 0;
pub const mmMMSCH_CTL: c_uint = 0x0007;
pub const mmMMSCH_CTL_BASE_IDX: c_int = 0;
pub const mmMMSCH_INTR: c_uint = 0x0008;
pub const mmMMSCH_INTR_BASE_IDX: c_int = 0;
pub const mmMMSCH_INTR_ACK: c_uint = 0x0009;
pub const mmMMSCH_INTR_ACK_BASE_IDX: c_int = 0;
pub const mmMMSCH_INTR_STATUS: c_uint = 0x000a;
pub const mmMMSCH_INTR_STATUS_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_VMID: c_uint = 0x000b;
pub const mmMMSCH_VF_VMID_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_CTX_ADDR_LO: c_uint = 0x000c;
pub const mmMMSCH_VF_CTX_ADDR_LO_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_CTX_ADDR_HI: c_uint = 0x000d;
pub const mmMMSCH_VF_CTX_ADDR_HI_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_CTX_SIZE: c_uint = 0x000e;
pub const mmMMSCH_VF_CTX_SIZE_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_GPCOM_ADDR_LO: c_uint = 0x000f;
pub const mmMMSCH_VF_GPCOM_ADDR_LO_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_GPCOM_ADDR_HI: c_uint = 0x0010;
pub const mmMMSCH_VF_GPCOM_ADDR_HI_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_GPCOM_SIZE: c_uint = 0x0011;
pub const mmMMSCH_VF_GPCOM_SIZE_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_HOST: c_uint = 0x0012;
pub const mmMMSCH_VF_MAILBOX_HOST_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_RESP: c_uint = 0x0013;
pub const mmMMSCH_VF_MAILBOX_RESP_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_0: c_uint = 0x0014;
pub const mmMMSCH_VF_MAILBOX_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_0_RESP: c_uint = 0x0015;
pub const mmMMSCH_VF_MAILBOX_0_RESP_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_1: c_uint = 0x0016;
pub const mmMMSCH_VF_MAILBOX_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX_1_RESP: c_uint = 0x0017;
pub const mmMMSCH_VF_MAILBOX_1_RESP_BASE_IDX: c_int = 0;
pub const mmMMSCH_CNTL: c_uint = 0x001c;
pub const mmMMSCH_CNTL_BASE_IDX: c_int = 0;
pub const mmMMSCH_NONCACHE_OFFSET0: c_uint = 0x001d;
pub const mmMMSCH_NONCACHE_OFFSET0_BASE_IDX: c_int = 0;
pub const mmMMSCH_NONCACHE_SIZE0: c_uint = 0x001e;
pub const mmMMSCH_NONCACHE_SIZE0_BASE_IDX: c_int = 0;
pub const mmMMSCH_NONCACHE_OFFSET1: c_uint = 0x001f;
pub const mmMMSCH_NONCACHE_OFFSET1_BASE_IDX: c_int = 0;
pub const mmMMSCH_NONCACHE_SIZE1: c_uint = 0x0020;
pub const mmMMSCH_NONCACHE_SIZE1_BASE_IDX: c_int = 0;
pub const mmMMSCH_PDEBUG_STATUS: c_uint = 0x0021;
pub const mmMMSCH_PDEBUG_STATUS_BASE_IDX: c_int = 0;
pub const mmMMSCH_PDEBUG_DATA_32UPPERBITS: c_uint = 0x0022;
pub const mmMMSCH_PDEBUG_DATA_32UPPERBITS_BASE_IDX: c_int = 0;
pub const mmMMSCH_PDEBUG_DATA_32LOWERBITS: c_uint = 0x0023;
pub const mmMMSCH_PDEBUG_DATA_32LOWERBITS_BASE_IDX: c_int = 0;
pub const mmMMSCH_PDEBUG_EPC: c_uint = 0x0024;
pub const mmMMSCH_PDEBUG_EPC_BASE_IDX: c_int = 0;
pub const mmMMSCH_PDEBUG_EXCCAUSE: c_uint = 0x0025;
pub const mmMMSCH_PDEBUG_EXCCAUSE_BASE_IDX: c_int = 0;
pub const mmMMSCH_PROC_STATE1: c_uint = 0x0026;
pub const mmMMSCH_PROC_STATE1_BASE_IDX: c_int = 0;
pub const mmMMSCH_LAST_MC_ADDR: c_uint = 0x0027;
pub const mmMMSCH_LAST_MC_ADDR_BASE_IDX: c_int = 0;
pub const mmMMSCH_LAST_MEM_ACCESS_HI: c_uint = 0x0028;
pub const mmMMSCH_LAST_MEM_ACCESS_HI_BASE_IDX: c_int = 0;
pub const mmMMSCH_LAST_MEM_ACCESS_LO: c_uint = 0x0029;
pub const mmMMSCH_LAST_MEM_ACCESS_LO_BASE_IDX: c_int = 0;
pub const mmMMSCH_IOV_ACTIVE_FCN_ID: c_uint = 0x002a;
pub const mmMMSCH_IOV_ACTIVE_FCN_ID_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_0: c_uint = 0x002b;
pub const mmMMSCH_SCRATCH_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_1: c_uint = 0x002c;
pub const mmMMSCH_SCRATCH_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_0: c_uint = 0x002d;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_0: c_uint = 0x002e;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_0: c_uint = 0x002f;
pub const mmMMSCH_GPUIOV_CMD_STATUS_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_0: c_uint = 0x0030;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_0: c_uint = 0x0031;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_0: c_uint = 0x0032;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW6_0: c_uint = 0x0033;
pub const mmMMSCH_GPUIOV_DW6_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW7_0: c_uint = 0x0034;
pub const mmMMSCH_GPUIOV_DW7_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW8_0: c_uint = 0x0035;
pub const mmMMSCH_GPUIOV_DW8_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_1: c_uint = 0x0036;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_1: c_uint = 0x0037;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_1: c_uint = 0x0038;
pub const mmMMSCH_GPUIOV_CMD_STATUS_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_1: c_uint = 0x0039;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_1: c_uint = 0x003a;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_1: c_uint = 0x003b;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW6_1: c_uint = 0x003c;
pub const mmMMSCH_GPUIOV_DW6_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW7_1: c_uint = 0x003d;
pub const mmMMSCH_GPUIOV_DW7_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW8_1: c_uint = 0x003e;
pub const mmMMSCH_GPUIOV_DW8_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CNTXT: c_uint = 0x003f;
pub const mmMMSCH_GPUIOV_CNTXT_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_2: c_uint = 0x0040;
pub const mmMMSCH_SCRATCH_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_3: c_uint = 0x0041;
pub const mmMMSCH_SCRATCH_3_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_4: c_uint = 0x0042;
pub const mmMMSCH_SCRATCH_4_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_5: c_uint = 0x0043;
pub const mmMMSCH_SCRATCH_5_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_6: c_uint = 0x0044;
pub const mmMMSCH_SCRATCH_6_BASE_IDX: c_int = 0;
pub const mmMMSCH_SCRATCH_7: c_uint = 0x0045;
pub const mmMMSCH_SCRATCH_7_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_HEAD_0: c_uint = 0x0046;
pub const mmMMSCH_VFID_FIFO_HEAD_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_TAIL_0: c_uint = 0x0047;
pub const mmMMSCH_VFID_FIFO_TAIL_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_HEAD_1: c_uint = 0x0048;
pub const mmMMSCH_VFID_FIFO_HEAD_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_TAIL_1: c_uint = 0x0049;
pub const mmMMSCH_VFID_FIFO_TAIL_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_NACK_STATUS: c_uint = 0x004a;
pub const mmMMSCH_NACK_STATUS_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX0_DATA: c_uint = 0x004b;
pub const mmMMSCH_VF_MAILBOX0_DATA_BASE_IDX: c_int = 0;
pub const mmMMSCH_VF_MAILBOX1_DATA: c_uint = 0x004c;
pub const mmMMSCH_VF_MAILBOX1_DATA_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_0: c_uint = 0x004d;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_0: c_uint = 0x004e;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_0: c_uint = 0x004f;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_1: c_uint = 0x0050;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_1: c_uint = 0x0051;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_1: c_uint = 0x0052;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CNTXT_IP: c_uint = 0x0053;
pub const mmMMSCH_GPUIOV_CNTXT_IP_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_2: c_uint = 0x0054;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_2: c_uint = 0x0055;
pub const mmMMSCH_GPUIOV_CMD_CONTROL_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_2: c_uint = 0x0056;
pub const mmMMSCH_GPUIOV_CMD_STATUS_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_2: c_uint = 0x0057;
pub const mmMMSCH_GPUIOV_VM_BUSY_STATUS_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_2: c_uint = 0x0058;
pub const mmMMSCH_GPUIOV_ACTIVE_FCNS_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_2: c_uint = 0x0059;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW6_2: c_uint = 0x005a;
pub const mmMMSCH_GPUIOV_DW6_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW7_2: c_uint = 0x005b;
pub const mmMMSCH_GPUIOV_DW7_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_DW8_2: c_uint = 0x005c;
pub const mmMMSCH_GPUIOV_DW8_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_2: c_uint = 0x005d;
pub const mmMMSCH_GPUIOV_SCH_BLOCK_IP_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_2: c_uint = 0x005e;
pub const mmMMSCH_GPUIOV_CMD_STATUS_IP_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_2: c_uint = 0x005f;
pub const mmMMSCH_GPUIOV_ACTIVE_FCN_ID_IP_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_HEAD_2: c_uint = 0x0060;
pub const mmMMSCH_VFID_FIFO_HEAD_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_VFID_FIFO_TAIL_2: c_uint = 0x0061;
pub const mmMMSCH_VFID_FIFO_TAIL_2_BASE_IDX: c_int = 0;
pub const mmMMSCH_VM_BUSY_STATUS_0: c_uint = 0x0062;
pub const mmMMSCH_VM_BUSY_STATUS_0_BASE_IDX: c_int = 0;
pub const mmMMSCH_VM_BUSY_STATUS_1: c_uint = 0x0063;
pub const mmMMSCH_VM_BUSY_STATUS_1_BASE_IDX: c_int = 0;
pub const mmMMSCH_VM_BUSY_STATUS_2: c_uint = 0x0064;
pub const mmMMSCH_VM_BUSY_STATUS_2_BASE_IDX: c_int = 0;
pub const MMSCH_VERSION_MAJOR: c_int = 2;
pub const MMSCH_VERSION_MINOR: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmsch_v2_0_command_type {
    MMSCH_COMMAND__DIRECT_REG_WRITE = 0,
    MMSCH_COMMAND__DIRECT_REG_POLLING = 2,
    MMSCH_COMMAND__DIRECT_REG_READ_MODIFY_WRITE = 3,
    MMSCH_COMMAND__INDIRECT_REG_WRITE = 8,
    MMSCH_COMMAND__END = 0xf
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_init_header {
    pub version: u32,
    pub header_size: u32,
    pub vcn_init_status: u32,
    pub vcn_table_offset: u32,
    pub vcn_table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_direct_reg_header {
    pub 28: uint32_t reg_offset :,
    pub 4: uint32_t command_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_indirect_reg_header {
    pub 20: uint32_t reg_offset :,
    pub 8: uint32_t reg_idx_space :,
    pub 4: uint32_t command_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_direct_write {
    pub cmd_header: mmsch_v2_0_cmd_direct_reg_header,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_direct_read_modify_write {
    pub cmd_header: mmsch_v2_0_cmd_direct_reg_header,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_direct_polling {
    pub cmd_header: mmsch_v2_0_cmd_direct_reg_header,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_end {
    pub cmd_header: mmsch_v2_0_cmd_direct_reg_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v2_0_cmd_indirect_write {
    pub cmd_header: mmsch_v2_0_cmd_indirect_reg_header,
    pub reg_value: u32,
}

