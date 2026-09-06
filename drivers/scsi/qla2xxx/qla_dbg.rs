//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_dbg.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//

//
// Firmware Dump structure definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2300_fw_dump {
    pub hccr: __be16,
    pub pbiu_reg: [__be16; 8],
    pub risc_host_reg: [__be16; 8],
    pub mailbox_reg: [__be16; 32],
    pub resp_dma_reg: [__be16; 32],
    pub dma_reg: [__be16; 48],
    pub risc_hdw_reg: [__be16; 16],
    pub risc_gp0_reg: [__be16; 16],
    pub risc_gp1_reg: [__be16; 16],
    pub risc_gp2_reg: [__be16; 16],
    pub risc_gp3_reg: [__be16; 16],
    pub risc_gp4_reg: [__be16; 16],
    pub risc_gp5_reg: [__be16; 16],
    pub risc_gp6_reg: [__be16; 16],
    pub risc_gp7_reg: [__be16; 16],
    pub frame_buf_hdw_reg: [__be16; 64],
    pub fpm_b0_reg: [__be16; 64],
    pub fpm_b1_reg: [__be16; 64],
    pub risc_ram: [__be16; 0xf800],
    pub stack_ram: [__be16; 0x1000],
    pub data_ram: [__be16; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2100_fw_dump {
    pub hccr: __be16,
    pub pbiu_reg: [__be16; 8],
    pub mailbox_reg: [__be16; 32],
    pub dma_reg: [__be16; 48],
    pub risc_hdw_reg: [__be16; 16],
    pub risc_gp0_reg: [__be16; 16],
    pub risc_gp1_reg: [__be16; 16],
    pub risc_gp2_reg: [__be16; 16],
    pub risc_gp3_reg: [__be16; 16],
    pub risc_gp4_reg: [__be16; 16],
    pub risc_gp5_reg: [__be16; 16],
    pub risc_gp6_reg: [__be16; 16],
    pub risc_gp7_reg: [__be16; 16],
    pub frame_buf_hdw_reg: [__be16; 16],
    pub fpm_b0_reg: [__be16; 64],
    pub fpm_b1_reg: [__be16; 64],
    pub risc_ram: [__be16; 0xf000],
    pub queue_dump: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla24xx_fw_dump {
    pub host_status: __be32,
    pub host_reg: [__be32; 32],
    pub shadow_reg: [__be32; 7],
    pub mailbox_reg: [__be16; 32],
    pub xseq_gp_reg: [__be32; 128],
    pub xseq_0_reg: [__be32; 16],
    pub xseq_1_reg: [__be32; 16],
    pub rseq_gp_reg: [__be32; 128],
    pub rseq_0_reg: [__be32; 16],
    pub rseq_1_reg: [__be32; 16],
    pub rseq_2_reg: [__be32; 16],
    pub cmd_dma_reg: [__be32; 16],
    pub req0_dma_reg: [__be32; 15],
    pub resp0_dma_reg: [__be32; 15],
    pub req1_dma_reg: [__be32; 15],
    pub xmt0_dma_reg: [__be32; 32],
    pub xmt1_dma_reg: [__be32; 32],
    pub xmt2_dma_reg: [__be32; 32],
    pub xmt3_dma_reg: [__be32; 32],
    pub xmt4_dma_reg: [__be32; 32],
    pub xmt_data_dma_reg: [__be32; 16],
    pub rcvt0_data_dma_reg: [__be32; 32],
    pub rcvt1_data_dma_reg: [__be32; 32],
    pub risc_gp_reg: [__be32; 128],
    pub lmc_reg: [__be32; 112],
    pub fpm_hdw_reg: [__be32; 192],
    pub fb_hdw_reg: [__be32; 176],
    pub code_ram: [__be32; 0x2000],
    pub ext_mem: [__be32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla25xx_fw_dump {
    pub host_status: __be32,
    pub host_risc_reg: [__be32; 32],
    pub pcie_regs: [__be32; 4],
    pub host_reg: [__be32; 32],
    pub shadow_reg: [__be32; 11],
    pub risc_io_reg: __be32,
    pub mailbox_reg: [__be16; 32],
    pub xseq_gp_reg: [__be32; 128],
    pub xseq_0_reg: [__be32; 48],
    pub xseq_1_reg: [__be32; 16],
    pub rseq_gp_reg: [__be32; 128],
    pub rseq_0_reg: [__be32; 32],
    pub rseq_1_reg: [__be32; 16],
    pub rseq_2_reg: [__be32; 16],
    pub aseq_gp_reg: [__be32; 128],
    pub aseq_0_reg: [__be32; 32],
    pub aseq_1_reg: [__be32; 16],
    pub aseq_2_reg: [__be32; 16],
    pub cmd_dma_reg: [__be32; 16],
    pub req0_dma_reg: [__be32; 15],
    pub resp0_dma_reg: [__be32; 15],
    pub req1_dma_reg: [__be32; 15],
    pub xmt0_dma_reg: [__be32; 32],
    pub xmt1_dma_reg: [__be32; 32],
    pub xmt2_dma_reg: [__be32; 32],
    pub xmt3_dma_reg: [__be32; 32],
    pub xmt4_dma_reg: [__be32; 32],
    pub xmt_data_dma_reg: [__be32; 16],
    pub rcvt0_data_dma_reg: [__be32; 32],
    pub rcvt1_data_dma_reg: [__be32; 32],
    pub risc_gp_reg: [__be32; 128],
    pub lmc_reg: [__be32; 128],
    pub fpm_hdw_reg: [__be32; 192],
    pub fb_hdw_reg: [__be32; 192],
    pub code_ram: [__be32; 0x2000],
    pub ext_mem: [__be32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla81xx_fw_dump {
    pub host_status: __be32,
    pub host_risc_reg: [__be32; 32],
    pub pcie_regs: [__be32; 4],
    pub host_reg: [__be32; 32],
    pub shadow_reg: [__be32; 11],
    pub risc_io_reg: __be32,
    pub mailbox_reg: [__be16; 32],
    pub xseq_gp_reg: [__be32; 128],
    pub xseq_0_reg: [__be32; 48],
    pub xseq_1_reg: [__be32; 16],
    pub rseq_gp_reg: [__be32; 128],
    pub rseq_0_reg: [__be32; 32],
    pub rseq_1_reg: [__be32; 16],
    pub rseq_2_reg: [__be32; 16],
    pub aseq_gp_reg: [__be32; 128],
    pub aseq_0_reg: [__be32; 32],
    pub aseq_1_reg: [__be32; 16],
    pub aseq_2_reg: [__be32; 16],
    pub cmd_dma_reg: [__be32; 16],
    pub req0_dma_reg: [__be32; 15],
    pub resp0_dma_reg: [__be32; 15],
    pub req1_dma_reg: [__be32; 15],
    pub xmt0_dma_reg: [__be32; 32],
    pub xmt1_dma_reg: [__be32; 32],
    pub xmt2_dma_reg: [__be32; 32],
    pub xmt3_dma_reg: [__be32; 32],
    pub xmt4_dma_reg: [__be32; 32],
    pub xmt_data_dma_reg: [__be32; 16],
    pub rcvt0_data_dma_reg: [__be32; 32],
    pub rcvt1_data_dma_reg: [__be32; 32],
    pub risc_gp_reg: [__be32; 128],
    pub lmc_reg: [__be32; 128],
    pub fpm_hdw_reg: [__be32; 224],
    pub fb_hdw_reg: [__be32; 208],
    pub code_ram: [__be32; 0x2000],
    pub ext_mem: [__be32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla83xx_fw_dump {
    pub host_status: __be32,
    pub host_risc_reg: [__be32; 48],
    pub pcie_regs: [__be32; 4],
    pub host_reg: [__be32; 32],
    pub shadow_reg: [__be32; 11],
    pub risc_io_reg: __be32,
    pub mailbox_reg: [__be16; 32],
    pub xseq_gp_reg: [__be32; 256],
    pub xseq_0_reg: [__be32; 48],
    pub xseq_1_reg: [__be32; 16],
    pub xseq_2_reg: [__be32; 16],
    pub rseq_gp_reg: [__be32; 256],
    pub rseq_0_reg: [__be32; 32],
    pub rseq_1_reg: [__be32; 16],
    pub rseq_2_reg: [__be32; 16],
    pub rseq_3_reg: [__be32; 16],
    pub aseq_gp_reg: [__be32; 256],
    pub aseq_0_reg: [__be32; 32],
    pub aseq_1_reg: [__be32; 16],
    pub aseq_2_reg: [__be32; 16],
    pub aseq_3_reg: [__be32; 16],
    pub cmd_dma_reg: [__be32; 64],
    pub req0_dma_reg: [__be32; 15],
    pub resp0_dma_reg: [__be32; 15],
    pub req1_dma_reg: [__be32; 15],
    pub xmt0_dma_reg: [__be32; 32],
    pub xmt1_dma_reg: [__be32; 32],
    pub xmt2_dma_reg: [__be32; 32],
    pub xmt3_dma_reg: [__be32; 32],
    pub xmt4_dma_reg: [__be32; 32],
    pub xmt_data_dma_reg: [__be32; 16],
    pub rcvt0_data_dma_reg: [__be32; 32],
    pub rcvt1_data_dma_reg: [__be32; 32],
    pub risc_gp_reg: [__be32; 128],
    pub lmc_reg: [__be32; 128],
    pub fpm_hdw_reg: [__be32; 256],
    pub rq0_array_reg: [__be32; 256],
    pub rq1_array_reg: [__be32; 256],
    pub rp0_array_reg: [__be32; 256],
    pub rp1_array_reg: [__be32; 256],
    pub queue_control_reg: [__be32; 16],
    pub fb_hdw_reg: [__be32; 432],
    pub at0_array_reg: [__be32; 128],
    pub code_ram: [__be32; 0x2400],
    pub ext_mem: [__be32; 1],
}

pub const EFT_NUM_BUFFERS: c_int = 4;
pub const EFT_BYTES_PER_BUFFER: c_uint = 0x4000;

pub const FCE_NUM_BUFFERS: c_int = 64;
pub const FCE_BYTES_PER_BUFFER: c_uint = 0x400;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_fce_chain {
    pub type: __be32,
    pub chain_size: __be32,
    pub size: __be32,
    pub addr_l: __be32,
    pub addr_h: __be32,
    pub eregs: [__be32; 8],
}

// used by exchange off load and extended login offload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_offld_chain {
    pub type: __be32,
    pub chain_size: __be32,
    pub size: __be32,
    pub reserved: __be32,
    pub addr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_mq_chain {
    pub type: __be32,
    pub chain_size: __be32,
    pub count: __be32,
    pub QLA_MQ_SIZE]: *mut *mut __be32 qregs[4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_mqueue_header {
    pub queue: __be32,
pub const TYPE_REQUEST_QUEUE: c_uint = 0x1;
pub const TYPE_RESPONSE_QUEUE: c_uint = 0x2;
pub const TYPE_ATIO_QUEUE: c_uint = 0x3;
    pub number: __be32,
    pub size: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_mqueue_chain {
    pub type: __be32,
    pub chain_size: __be32,
}

pub const DUMP_CHAIN_VARIANT: c_uint = 0x80000000;
pub const DUMP_CHAIN_FCE: c_uint = 0x7FFFFAF0;
pub const DUMP_CHAIN_MQ: c_uint = 0x7FFFFAF1;
pub const DUMP_CHAIN_QUEUE: c_uint = 0x7FFFFAF2;
pub const DUMP_CHAIN_EXLOGIN: c_uint = 0x7FFFFAF3;
pub const DUMP_CHAIN_EXCHG: c_uint = 0x7FFFFAF4;
pub const DUMP_CHAIN_LAST: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2xxx_fw_dump {
    pub signature: [u8; 4],
    pub version: __be32,
    pub fw_major_version: __be32,
    pub fw_minor_version: __be32,
    pub fw_subminor_version: __be32,
    pub fw_attributes: __be32,
    pub vendor: __be32,
    pub device: __be32,
    pub subsystem_vendor: __be32,
    pub subsystem_device: __be32,
    pub fixed_size: __be32,
    pub mem_size: __be32,
    pub req_q_size: __be32,
    pub rsp_q_size: __be32,
    pub eft_size: __be32,
    pub eft_addr_l: __be32,
    pub eft_addr_h: __be32,
    pub header_size: __be32,
    pub isp21: qla2100_fw_dump,
    pub isp23: qla2300_fw_dump,
    pub isp24: qla24xx_fw_dump,
    pub isp25: qla25xx_fw_dump,
    pub isp81: qla81xx_fw_dump,
    pub isp83: qla83xx_fw_dump,
    pub isp: },
}

pub const QL_DBG_DEFAULT1_MASK: c_uint = 0x1e600000;

// No messages will use this value.
// This should be always highest value
// as compared to other log levels.
//
// Debug Levels
// The 0x40000000 is the max value any debug level can have
// as ql2xextended_error_logging is of type signed int
//
pub const ql_dbg_init: c_uint = 0x40000000 /* Init Debug */;
pub const ql_dbg_mbx: c_uint = 0x20000000 /* MBX Debug */;
pub const ql_dbg_disc: c_uint = 0x10000000 /* Device Discovery Debug */;
pub const ql_dbg_io: c_uint = 0x08000000 /* IO Tracing Debug */;
pub const ql_dbg_dpc: c_uint = 0x04000000 /* DPC Thead Debug */;
pub const ql_dbg_async: c_uint = 0x02000000 /* Async events Debug */;
pub const ql_dbg_timer: c_uint = 0x01000000 /* Timer Debug */;
pub const ql_dbg_user: c_uint = 0x00800000 /* User Space Interations Debug */;
pub const ql_dbg_taskm: c_uint = 0x00400000 /* Task Management Debug */;
pub const ql_dbg_aer: c_uint = 0x00200000 /* AER/EEH Debug */;
pub const ql_dbg_multiq: c_uint = 0x00100000 /* MultiQ Debug */;
pub const ql_dbg_p3p: c_uint = 0x00080000 /* P3P specific Debug */;
pub const ql_dbg_vport: c_uint = 0x00040000 /* Virtual Port Debug */;
pub const ql_dbg_buffer: c_uint = 0x00020000 /* For dumping the buffer/regs */;
pub const ql_dbg_misc: c_uint = 0x00010000 /* For dumping everything that is not;
// not covered by upper categories
//
pub const ql_dbg_verbose: c_uint = 0x00008000 /* More verbosity for each level;
// This is to be used with other levels where
// more verbosity is required. It might not
// be applicable to all the levels.
//
pub const ql_dbg_tgt: c_uint = 0x00004000 /* Target mode */;
pub const ql_dbg_tgt_mgt: c_uint = 0x00002000 /* Target mode management */;
pub const ql_dbg_tgt_tmr: c_uint = 0x00001000 /* Target mode task management */;
pub const ql_dbg_tgt_dif: c_uint = 0x00000800 /* Target mode dif */;
pub const ql_dbg_edif: c_uint = 0x00000400 /* edif and purex debug */;
pub const ql_dbg_unsol: c_uint = 0x00000100 /* Unsolicited path debug */;
extern "C" {
    pub fn qla24xx_soft_reset(: *mut qla_hw_data) -> c_int;
}
// log_tunable = QL_DBG_DEFAULT1_MASK;
// Assumes local variable pbuf and pbuf_ready present.

// Macro flag: #define QLA_ENABLE_KERNEL_TRACING

// Macro flag: #define QLA_TRACE_ENABLE(_tr)
