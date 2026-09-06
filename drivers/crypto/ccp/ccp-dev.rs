//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/ccp-dev.h
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
// AMD Cryptographic Coprocessor (CCP) driver
//
// Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
// Author: Gary R Hook <gary.hook@amd.com>
//

pub const MAX_CCP_NAME_LEN: c_int = 16;
pub const MAX_DMAPOOL_NAME_LEN: c_int = 32;
pub const MAX_HW_QUEUES: c_int = 5;
pub const MAX_CMD_QLEN: c_int = 100;
pub const TRNG_RETRIES: c_int = 10;
pub const CACHE_NONE: c_uint = 0x00;
pub const CACHE_WB_NO_ALLOC: c_uint = 0xb7;
// Register Mappings
pub const Q_MASK_REG: c_uint = 0x000;
pub const TRNG_OUT_REG: c_uint = 0x00c;
pub const IRQ_MASK_REG: c_uint = 0x040;
pub const IRQ_STATUS_REG: c_uint = 0x200;
pub const DEL_CMD_Q_JOB: c_uint = 0x124;
pub const DEL_Q_ACTIVE: c_uint = 0x00000200;
pub const DEL_Q_ID_SHIFT: c_int = 6;
pub const CMD_REQ0: c_uint = 0x180;
pub const CMD_REQ_INCR: c_uint = 0x04;
pub const CMD_Q_STATUS_BASE: c_uint = 0x210;
pub const CMD_Q_INT_STATUS_BASE: c_uint = 0x214;
pub const CMD_Q_STATUS_INCR: c_uint = 0x20;
pub const CMD_Q_CACHE_BASE: c_uint = 0x228;
pub const CMD_Q_CACHE_INC: c_uint = 0x20;

// ------------------------ CCP Version 5 Specifics ------------------------
pub const CMD5_QUEUE_MASK_OFFSET: c_uint = 0x00;
pub const CMD5_QUEUE_PRIO_OFFSET: c_uint = 0x04;
pub const CMD5_REQID_CONFIG_OFFSET: c_uint = 0x08;
pub const CMD5_CMD_TIMEOUT_OFFSET: c_uint = 0x10;
pub const LSB_PUBLIC_MASK_LO_OFFSET: c_uint = 0x18;
pub const LSB_PUBLIC_MASK_HI_OFFSET: c_uint = 0x1C;
pub const LSB_PRIVATE_MASK_LO_OFFSET: c_uint = 0x20;
pub const LSB_PRIVATE_MASK_HI_OFFSET: c_uint = 0x24;
pub const CMD5_PSP_CCP_VERSION: c_uint = 0x100;
pub const CMD5_Q_CONTROL_BASE: c_uint = 0x0000;
pub const CMD5_Q_TAIL_LO_BASE: c_uint = 0x0004;
pub const CMD5_Q_HEAD_LO_BASE: c_uint = 0x0008;
pub const CMD5_Q_INT_ENABLE_BASE: c_uint = 0x000C;
pub const CMD5_Q_INTERRUPT_STATUS_BASE: c_uint = 0x0010;
pub const CMD5_Q_STATUS_BASE: c_uint = 0x0100;
pub const CMD5_Q_INT_STATUS_BASE: c_uint = 0x0104;
pub const CMD5_Q_DMA_STATUS_BASE: c_uint = 0x0108;
pub const CMD5_Q_DMA_READ_STATUS_BASE: c_uint = 0x010C;
pub const CMD5_Q_DMA_WRITE_STATUS_BASE: c_uint = 0x0110;
pub const CMD5_Q_ABORT_BASE: c_uint = 0x0114;
pub const CMD5_Q_AX_CACHE_BASE: c_uint = 0x0118;
pub const CMD5_CONFIG_0_OFFSET: c_uint = 0x6000;
pub const CMD5_TRNG_CTL_OFFSET: c_uint = 0x6008;
pub const CMD5_AES_MASK_OFFSET: c_uint = 0x6010;
pub const CMD5_CLK_GATE_CTL_OFFSET: c_uint = 0x603C;
// Address offset between two virtual queue registers
pub const CMD5_Q_STATUS_INCR: c_uint = 0x1000;
// Bit masks
pub const CMD5_Q_RUN: c_uint = 0x1;
pub const CMD5_Q_HALT: c_uint = 0x2;
pub const CMD5_Q_MEM_LOCATION: c_uint = 0x4;
pub const CMD5_Q_SIZE: c_uint = 0x1F;
pub const CMD5_Q_SHIFT: c_int = 3;
pub const COMMANDS_PER_QUEUE: c_int = 16;

pub const INT_COMPLETION: c_uint = 0x1;
pub const INT_ERROR: c_uint = 0x2;
pub const INT_QUEUE_STOPPED: c_uint = 0x4;
pub const INT_EMPTY_QUEUE: c_uint = 0x8;

pub const LSB_REGION_WIDTH: c_int = 5;
pub const MAX_LSB_CNT: c_int = 8;
pub const LSB_SIZE: c_int = 16;
pub const LSB_ITEM_SIZE: c_int = 32;

// ------------------------ CCP Version 3 Specifics ------------------------
pub const REQ0_WAIT_FOR_WRITE: c_uint = 0x00000004;
pub const REQ0_INT_ON_COMPLETE: c_uint = 0x00000002;
pub const REQ0_STOP_ON_COMPLETE: c_uint = 0x00000001;
pub const REQ0_CMD_Q_SHIFT: c_int = 9;
pub const REQ0_JOBID_SHIFT: c_int = 3;
// REQ1 Related Values
pub const REQ1_PROTECT_SHIFT: c_int = 27;
pub const REQ1_ENGINE_SHIFT: c_int = 23;
pub const REQ1_KEY_KSB_SHIFT: c_int = 2;
pub const REQ1_EOM: c_uint = 0x00000002;
pub const REQ1_INIT: c_uint = 0x00000001;
// AES Related Values
pub const REQ1_AES_TYPE_SHIFT: c_int = 21;
pub const REQ1_AES_MODE_SHIFT: c_int = 18;
pub const REQ1_AES_ACTION_SHIFT: c_int = 17;
pub const REQ1_AES_CFB_SIZE_SHIFT: c_int = 10;
// XTS-AES Related Values
pub const REQ1_XTS_AES_SIZE_SHIFT: c_int = 10;
// SHA Related Values
pub const REQ1_SHA_TYPE_SHIFT: c_int = 21;
// RSA Related Values
pub const REQ1_RSA_MOD_SIZE_SHIFT: c_int = 10;
// Pass-Through Related Values
pub const REQ1_PT_BW_SHIFT: c_int = 12;
pub const REQ1_PT_BS_SHIFT: c_int = 10;
// ECC Related Values
pub const REQ1_ECC_AFFINE_CONVERT: c_uint = 0x00200000;
pub const REQ1_ECC_FUNCTION_SHIFT: c_int = 18;
// REQ4 Related Values
pub const REQ4_KSB_SHIFT: c_int = 18;
pub const REQ4_MEMTYPE_SHIFT: c_int = 16;
// REQ6 Related Values
pub const REQ6_MEMTYPE_SHIFT: c_int = 16;
// Key Storage Block
pub const KSB_START: c_int = 77;
pub const KSB_END: c_int = 127;

pub const CCP_SB_BITS: c_int = 256;
pub const CCP_JOBID_MASK: c_uint = 0x0000003f;
// ------------------------ General CCP Defines ------------------------
pub const CCP_DMA_DFLT: c_uint = 0x0;
pub const CCP_DMA_PRIV: c_uint = 0x1;
pub const CCP_DMA_PUB: c_uint = 0x2;
pub const CCP_DMAPOOL_MAX_SIZE: c_int = 64;

pub const CCP_REVERSE_BUF_SIZE: c_int = 64;
pub const CCP_AES_KEY_SB_COUNT: c_int = 1;
pub const CCP_AES_CTX_SB_COUNT: c_int = 1;
pub const CCP_XTS_AES_KEY_SB_COUNT: c_int = 1;
pub const CCP5_XTS_AES_KEY_SB_COUNT: c_int = 2;
pub const CCP_XTS_AES_CTX_SB_COUNT: c_int = 1;
pub const CCP_DES3_KEY_SB_COUNT: c_int = 1;
pub const CCP_DES3_CTX_SB_COUNT: c_int = 1;
pub const CCP_SHA_SB_COUNT: c_int = 1;
pub const CCP_RSA_MAX_WIDTH: c_int = 4096;
pub const CCP5_RSA_MAX_WIDTH: c_int = 16384;
pub const CCP_PASSTHRU_BLOCKSIZE: c_int = 256;
pub const CCP_PASSTHRU_MASKSIZE: c_int = 32;
pub const CCP_PASSTHRU_SB_COUNT: c_int = 1;

pub const CCP_ECC_MAX_OPERANDS: c_int = 6;
pub const CCP_ECC_MAX_OUTPUTS: c_int = 3;
pub const CCP_ECC_SRC_BUF_SIZE: c_int = 448;
pub const CCP_ECC_DST_BUF_SIZE: c_int = 192;
pub const CCP_ECC_OPERAND_SIZE: c_int = 64;
pub const CCP_ECC_OUTPUT_SIZE: c_int = 64;
pub const CCP_ECC_RESULT_OFFSET: c_int = 60;
pub const CCP_ECC_RESULT_SUCCESS: c_uint = 0x0001;
pub const CCP_SB_BYTES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_dma_cmd {
    pub entry: list_head,
    pub ccp_cmd: ccp_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_dma_desc {
    pub entry: list_head,
    pub ccp: *mut ccp_device,
    pub pending: list_head,
    pub active: list_head,
    pub status: dma_status,
    pub tx_desc: dma_async_tx_descriptor,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_dma_chan {
    pub ccp: *mut ccp_device,
    pub lock: spinlock_t,
    pub created: list_head,
    pub pending: list_head,
    pub active: list_head,
    pub complete: list_head,
    pub cleanup_tasklet: tasklet_struct,
    pub status: dma_status,
    pub dma_chan: dma_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_cmd_queue {
    pub ccp: *mut ccp_device,
// Queue identifier
    pub id: u32,
// Queue dma pool
    pub dma_pool: *mut dma_pool,
// Queue base address (not neccessarily aligned)
    pub qbase: *mut ccp5_desc,
// Aligned queue start address (per requirement)
    pub ____cacheline_aligned: mutex q_mutex,
    pub qidx: c_uint,
// Version 5 has different requirements for queue memory
    pub qsize: c_uint,
    pub qbase_dma: dma_addr_t,
    pub qdma_tail: dma_addr_t,
// Per-queue reserved storage block(s)
    pub sb_key: u32,
    pub sb_ctx: u32,
// Bitmap of LSBs that can be accessed by this queue
    pub MAX_LSB_CNT): DECLARE_BITMAP(lsbmask,,
// Private LSB that is assigned to this queue, or -1 if none.
// Bitmap for my private LSB, unused otherwise
//
    pub lsb: c_int,
    pub PLSB_MAP_SIZE): DECLARE_BITMAP(lsbmap,,
// Queue processing thread
    pub kthread: *mut task_struct,
    pub active: c_uint,
    pub suspended: c_uint,
// Number of free command slots available
    pub free_slots: c_uint,
// Interrupt masks
    pub int_ok: u32,
    pub int_err: u32,
// Register addresses for queue
    pub reg_control: *mut void __iomem,
    pub reg_tail_lo: *mut void __iomem,
    pub reg_head_lo: *mut void __iomem,
    pub reg_int_enable: *mut void __iomem,
    pub reg_interrupt_status: *mut void __iomem,
    pub reg_status: *mut void __iomem,
    pub reg_int_status: *mut void __iomem,
    pub reg_dma_status: *mut void __iomem,
    pub reg_dma_read_status: *mut void __iomem,
    pub reg_dma_write_status: *mut void __iomem,
    pub /: *mut *mut u32 qcontrol; / Cached control register,
// Status values from job
    pub int_status: u32,
    pub q_status: u32,
    pub q_int_status: u32,
    pub cmd_error: u32,
// Interrupt wait queue
    pub int_queue: wait_queue_head_t,
    pub int_rcvd: c_uint,
// Per-queue Statistics
    pub total_ops: c_ulong,
    pub total_aes_ops: c_ulong,
    pub total_xts_aes_ops: c_ulong,
    pub total_3des_ops: c_ulong,
    pub total_sha_ops: c_ulong,
    pub total_rsa_ops: c_ulong,
    pub total_pt_ops: c_ulong,
    pub total_ecc_ops: c_ulong,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_device {
    pub entry: list_head,
    pub vdata: *mut ccp_vdata,
    pub ord: c_uint,
    pub name: [c_char; MAX_CCP_NAME_LEN],
    pub rngname: [c_char; MAX_CCP_NAME_LEN],
    pub dev: *mut device,
    pub sp: *mut sp_device,
// Bus specific device information
//
    pub dev_specific: *mut c_void,
    pub qim: c_uint,
    pub irq: c_uint,
    pub use_tasklet: bool,
    pub irq_tasklet: tasklet_struct,
// I/O area used for device communication. The register mapping
// starts at an offset into the mapped bar.
// The CMD_REQx registers and the Delete_Cmd_Queue_Job register
// need to be protected while a command queue thread is accessing
// them.
//
    pub ____cacheline_aligned: mutex req_mutex,
    pub io_regs: *mut void __iomem,
// Master lists that all cmds are queued on. Because there can be
// more than one CCP command queue that can process a cmd a separate
// backlog list is needed so that the backlog completion call
// completes before the cmd is available for execution.
//
    pub ____cacheline_aligned: spinlock_t cmd_lock,
    pub cmd_count: c_uint,
    pub cmd: list_head,
    pub backlog: list_head,
// The command queues. These represent the queues available on the
// CCP that are available for processing cmds
//
    pub cmd_q: [ccp_cmd_queue; MAX_HW_QUEUES],
    pub cmd_q_count: c_uint,
    pub max_q_count: c_uint,
// Support for the CCP True RNG
//
    pub hwrng: hwrng,
    pub hwrng_retries: c_uint,
// Support for the CCP DMA capabilities
//
    pub dma_dev: dma_device,
    pub ccp_dma_chan: *mut ccp_dma_chan,
    pub dma_cmd_cache: *mut kmem_cache,
    pub dma_desc_cache: *mut kmem_cache,
// A counter used to generate job-ids for cmds submitted to the CCP
//
    pub ____cacheline_aligned: atomic_t current_id,
// The v3 CCP uses key storage blocks (SB) to maintain context for
// certain operations. To prevent multiple cmds from using the same
// SB range a command queue reserves an SB range for the duration of
// the cmd. Each queue, will however, reserve 2 SB blocks for
// operations that only require single SB entries (eg. AES context/iv
// and key) in order to avoid allocation contention.  This will reserve
// at most 10 SB entries, leaving 40 SB entries available for dynamic
// allocation.
//
// The v5 CCP Local Storage Block (LSB) is broken up into 8
// memrory ranges, each of which can be enabled for access by one
// or more queues. Device initialization takes this into account,
// and attempts to assign one region for exclusive use by each
// available queue; the rest are then aggregated as "public" use.
// If there are fewer regions than queues, all regions are shared
// amongst all queues.
//
    pub ____cacheline_aligned: mutex sb_mutex,
    pub KSB_COUNT): DECLARE_BITMAP(sb,,
    pub sb_queue: wait_queue_head_t,
    pub sb_avail: c_uint,
    pub sb_count: c_uint,
    pub sb_start: u32,
// Bitmap of shared LSBs, if any
    pub SLSB_MAP_SIZE): DECLARE_BITMAP(lsbmap,,
// Suspend support
    pub suspending: c_uint,
    pub suspend_queue: wait_queue_head_t,
// DMA caching attribute support
    pub axcache: c_uint,
// Device Statistics
    pub total_interrupts: c_ulong,
// DebugFS info
    pub debugfs_instance: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccp_memtype {
    CCP_MEMTYPE_SYSTEM = 0,
    CCP_MEMTYPE_SB,
    CCP_MEMTYPE_LOCAL,
    CCP_MEMTYPE__LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_dma_info {
    pub address: dma_addr_t,
    pub offset: c_uint,
    pub length: c_uint,
    pub dir: dma_data_direction,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_dm_workarea {
    pub dev: *mut device,
    pub dma_pool: *mut dma_pool,
    pub address: *mut u8,
    pub dma: ccp_dma_info,
    pub length: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_sg_workarea {
    pub sg: *mut scatterlist,
    pub nents: c_int,
    pub sg_used: c_uint,
    pub dma_sg: *mut scatterlist,
    pub dma_sg_head: *mut scatterlist,
    pub dma_dev: *mut device,
    pub dma_count: c_uint,
    pub dma_dir: dma_data_direction,
    pub bytes_left: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_data {
    pub sg_wa: ccp_sg_workarea,
    pub dm_wa: ccp_dm_workarea,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_mem {
    pub type: ccp_memtype,
    pub dma: ccp_dma_info,
    pub sb: u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_op {
    pub type: ccp_aes_type,
    pub mode: ccp_aes_mode,
    pub action: ccp_aes_action,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_xts_aes_op {
    pub type: ccp_aes_type,
    pub action: ccp_aes_action,
    pub unit_size: ccp_xts_aes_unit_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_des3_op {
    pub type: ccp_des3_type,
    pub mode: ccp_des3_mode,
    pub action: ccp_des3_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_sha_op {
    pub type: ccp_sha_type,
    pub msg_bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_rsa_op {
    pub mod_size: u32,
    pub input_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_passthru_op {
    pub bit_mod: ccp_passthru_bitwise,
    pub byte_swap: ccp_passthru_byteswap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_ecc_op {
    pub function: ccp_ecc_function,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_op {
    pub cmd_q: *mut ccp_cmd_queue,
    pub jobid: u32,
    pub ioc: u32,
    pub soc: u32,
    pub sb_key: u32,
    pub sb_ctx: u32,
    pub init: u32,
    pub eom: u32,
    pub src: ccp_mem,
    pub dst: ccp_mem,
    pub exp: ccp_mem,
    pub aes: ccp_aes_op,
    pub xts: ccp_xts_aes_op,
    pub des3: ccp_des3_op,
    pub sha: ccp_sha_op,
    pub rsa: ccp_rsa_op,
    pub passthru: ccp_passthru_op,
    pub ecc: ccp_ecc_op,
    pub u: },
}

extern "C" {
    pub fn lower_32_bits(info->offset: info->address +) -> return;
}
//
// descriptor for version 5 CPP commands
// 8 32-bit words:
// word 0: function; engine; control bits
// word 1: length of source data
// word 2: low 32 bits of source pointer
// word 3: upper 16 bits of source pointer; source memory type
// word 4: low 32 bits of destination pointer
// word 5: upper 16 bits of destination pointer; destination memory type
// word 6: low 32 bits of key pointer
// word 7: upper 16 bits of key pointer; key memory type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword0 {
    pub soc:1: c_uint,
    pub ioc:1: c_uint,
    pub rsvd1:1: c_uint,
    pub init:1: c_uint,
    pub /: *mut *mut unsigned int eom:1; / AES/SHA only,
    pub function:15: c_uint,
    pub engine:4: c_uint,
    pub prot:1: c_uint,
    pub rsvd2:7: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword3 {
    pub src_hi:16: c_uint,
    pub src_mem:2: c_uint,
    pub lsb_cxt_id:8: c_uint,
    pub rsvd1:5: c_uint,
    pub fixed:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dword4 {
    pub /: *mut *mut u32 dst_lo; / NON-SHA,
    pub /: *mut *mut u32 sha_len_lo; / SHA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dword5 {
    pub dst_hi:16: c_uint,
    pub dst_mem:2: c_uint,
    pub rsvd1:13: c_uint,
    pub fixed:1: c_uint,
    pub fields: },
    pub sha_len_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword7 {
    pub key_hi:16: c_uint,
    pub key_mem:2: c_uint,
    pub rsvd1:14: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp5_desc {
    pub dw0: dword0,
    pub length: u32,
    pub src_lo: u32,
    pub dw3: dword3,
    pub dw4: dword4,
    pub dw5: dword5,
    pub key_lo: u32,
    pub dw7: dword7,
}

extern "C" {
    pub fn ccp_add_device(ccp: *mut ccp_device);
}
extern "C" {
    pub fn ccp_del_device(ccp: *mut ccp_device);
}
extern "C" {
    pub fn ccp_log_error(: *mut ccp_device, int: unsigned);
}
extern "C" {
    pub fn ccp_queues_suspended(ccp: *mut ccp_device) -> bool;
}
extern "C" {
    pub fn ccp_cmd_queue_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ccp_trng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int;
}
extern "C" {
    pub fn ccp_run_cmd(cmd_q: *mut ccp_cmd_queue, cmd: *mut ccp_cmd) -> c_int;
}
extern "C" {
    pub fn ccp_register_rng(ccp: *mut ccp_device) -> c_int;
}
extern "C" {
    pub fn ccp_unregister_rng(ccp: *mut ccp_device);
}
extern "C" {
    pub fn ccp_dmaengine_register(ccp: *mut ccp_device) -> c_int;
}
extern "C" {
    pub fn ccp_dmaengine_unregister(ccp: *mut ccp_device);
}
extern "C" {
    pub fn ccp5_debugfs_setup(ccp: *mut ccp_device);
}
extern "C" {
    pub fn ccp5_debugfs_destroy();
}
// Structure for computation functions that are device-specific
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_actions {
    pub ): *mut *mut int (aes)(struct ccp_op,
    pub ): *mut *mut int (xts_aes)(struct ccp_op,
    pub ): *mut *mut int (des3)(struct ccp_op,
    pub ): *mut *mut int (sha)(struct ccp_op,
    pub ): *mut *mut int (rsa)(struct ccp_op,
    pub ): *mut *mut int (passthru)(struct ccp_op,
    pub ): *mut *mut int (ecc)(struct ccp_op,
    pub int): *mut *mut *mut u32 (sballoc)(struct ccp_cmd_queue , unsigned,
    pub int): *mut *mut *mut void (sbfree)(struct ccp_cmd_queue , unsigned int, unsigned,
    pub ): *mut *mut unsigned int (get_free_slots)(struct ccp_cmd_queue,
    pub ): *mut *mut int (init)(struct ccp_device,
    pub ): *mut *mut void (destroy)(struct ccp_device,
    pub ): *mut *mut irqreturn_t (irqhandler)(int, void,
}
