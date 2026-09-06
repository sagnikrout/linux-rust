//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/atom/sst/sst.h
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
// sst.h - Intel SST Driver for audio engine
//
// Copyright (C) 2008-14 Intel Corporation
// Authors:	Vinod Koul <vinod.koul@intel.com>
// Harsha Priya <priya.harsha@intel.com>
// Dharageswari R <dharageswari.r@intel.com>
// KP Jeeja <jeeja.kp@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Common private declarations for SST
//

// driver names

pub const SST_SUSPEND_DELAY: c_int = 2000;

pub const SST_ICCM_BOUNDARY: c_int = 4;
pub const SST_CONFIG_SSP_SIGN: c_uint = 0x7ffe8001;
pub const MRFLD_FW_VIRTUAL_BASE: c_uint = 0xC0000000;
pub const MRFLD_FW_DDR_BASE_OFFSET: c_uint = 0x0;
pub const MRFLD_FW_FEATURE_BASE_OFFSET: c_uint = 0x4;
pub const MRFLD_FW_BSS_RESET_BIT: c_int = 0;
// SST Shim register map
pub const SST_CSR: c_uint = 0x00;
pub const SST_ISRX: c_uint = 0x18;
pub const SST_IMRX: c_uint = 0x28;
pub const SST_IPCX: c_uint = 0x38 /* IPC IA -> SST */;
pub const SST_IPCD: c_uint = 0x40 /* IPC SST -> IA */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_states {
    SST_FW_LOADING = 1,
    SST_FW_RUNNING,
    SST_RESET,
    SST_SHUTDOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_algo_ops {
    SST_SET_ALGO = 0,
    SST_GET_ALGO = 1,
}

pub const SST_BLOCK_TIMEOUT: c_int = 1000;
pub const FW_SIGNATURE_SIZE: c_int = 4;
pub const FW_NAME_SIZE: c_int = 32;
// stream states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_stream_states {
    STREAM_UN_INIT	= 0,	/* Freed/Not used stream */
    STREAM_RUNNING	= 1,	/* Running */
    STREAM_PAUSED	= 2,	/* Paused stream */
    STREAM_INIT	= 3,	/* stream init, waiting for data */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ram_type {
    SST_IRAM	= 1,
    SST_DRAM	= 2,
    SST_DDR	= 5,
    SST_CUSTOM_INFO	= 7,	/* consists of FW binary information */
}

// SST shim registers to structure mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub union interrupt_reg {
    pub done_interrupt:1: u64,
    pub busy_interrupt:1: u64,
    pub rsvd:62: u64,
    pub part: },
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_pisr_reg {
    pub pssp0:1: u32,
    pub pssp1:1: u32,
    pub rsvd0:3: u32,
    pub dmac:1: u32,
    pub rsvd1:26: u32,
    pub part: },
    pub full: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_pimr_reg {
    pub ssp0:1: u32,
    pub ssp1:1: u32,
    pub rsvd0:3: u32,
    pub dmac:1: u32,
    pub rsvd1:10: u32,
    pub ssp0_sc:1: u32,
    pub ssp1_sc:1: u32,
    pub rsvd2:3: u32,
    pub dmac_sc:1: u32,
    pub rsvd3:10: u32,
    pub part: },
    pub full: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union config_status_reg_mrfld {
    pub lpe_reset:1: u64,
    pub lpe_reset_vector:1: u64,
    pub runstall:1: u64,
    pub pwaitmode:1: u64,
    pub clk_sel:3: u64,
    pub rsvd2:1: u64,
    pub sst_clk:3: u64,
    pub xt_snoop:1: u64,
    pub rsvd3:4: u64,
    pub clk_sel1:6: u64,
    pub clk_enable:3: u64,
    pub rsvd4:6: u64,
    pub slim0baseclk:1: u64,
    pub rsvd:32: u64,
    pub part: },
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union interrupt_reg_mrfld {
    pub done_interrupt:1: u64,
    pub busy_interrupt:1: u64,
    pub rsvd:62: u64,
    pub part: },
    pub full: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_imr_reg_mrfld {
    pub done_interrupt:1: u64,
    pub busy_interrupt:1: u64,
    pub rsvd:62: u64,
    pub part: },
    pub full: u64,
}

//
// struct sst_block - This structure is used to block a user/fw data call to another
// fw/user call
//
// @condition: condition for blocking check
// @ret_code: ret code when block is released
// @data: data ptr
// @size: size of data
// @on: block condition
// @msg_id: msg_id = msgid in mfld/ctp, mrfld = NULL
// @drv_id: str_id in mfld/ctp, = drv_id in mrfld
// @node: list head node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_block {
    pub condition: bool,
    pub ret_code: c_int,
    pub data: *mut c_void,
    pub size: u32,
    pub on: bool,
    pub msg_id: u32,
    pub drv_id: u32,
    pub node: list_head,
}

//
// struct stream_info - structure that holds the stream information
//
// @status : stream current state
// @prev : stream prev state
// @resume_status : stream current state to restore on resume
// @resume_prev : stream prev state to restore on resume
// @lock : stream mutex for protecting state
// @alloc_param : parameters used for stream (re-)allocation
// @pcm_substream : PCM substream
// @period_elapsed : PCM period elapsed callback
// @sfreq : stream sampling freq
// @cumm_bytes : cummulative bytes decoded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_info {
    pub status: c_uint,
    pub prev: c_uint,
    pub resume_status: c_uint,
    pub resume_prev: c_uint,
    pub lock: mutex,
    pub alloc_param: snd_sst_alloc_mrfld,
    pub pcm_substream: *mut c_void,
    pub pcm_substream): *mut *mut void (period_elapsed)(void,
    pub sfreq: c_uint,
    pub cumm_bytes: u32,
    pub compr_cb_param: *mut c_void,
    pub compr_cb_param): *mut *mut void (compr_cb)(void,
    pub drain_cb_param: *mut c_void,
    pub drain_cb_param): *mut *mut void (drain_notify)(void,
    pub num_ch: c_uint,
    pub pipe_id: c_uint,
    pub task_id: c_uint,
}

//
// struct sst_fw_header - FW file headers
//
// @signature : FW signature
// @file_size: size of fw image
// @modules : # of modules
// @file_format : version of header format
// @reserved : reserved fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_fw_header {
    pub signature: [c_uchar; FW_SIGNATURE_SIZE],
    pub file_size: u32,
    pub modules: u32,
    pub file_format: u32,
    pub reserved: [u32; 4],
}

//
// struct fw_module_header - module header in FW
//
// @signature: module signature
// @mod_size: size of module
// @blocks: block count
// @type: block type
// @entry_point: module netry point
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_module_header {
    pub signature: [c_uchar; FW_SIGNATURE_SIZE],
    pub mod_size: u32,
    pub blocks: u32,
    pub type: u32,
    pub entry_point: u32,
}

//
// struct fw_block_info - block header for FW
//
// @type: block ram type I/D
// @size: size of block
// @ram_offset: offset in ram
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_block_info {
    pub type: sst_ram_type,
    pub size: u32,
    pub ram_offset: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_runtime_param {
    pub param: snd_sst_runtime_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_sg_list {
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub list_len: c_int,
    pub sg_idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_memcpy_list {
    pub memcpylist: list_head,
    pub dstn: *mut c_void,
    pub src: *const c_void,
    pub size: u32,
    pub is_io: bool,
}

// Firmware Module Information
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_lib_dwnld_status {
    SST_LIB_NOT_FOUND = 0,
    SST_LIB_FOUND,
    SST_LIB_DOWNLOADED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_module_info {
    pub name*/: *const *const *const char name; /Library,
    pub ID*/: *mut *mut u32 id; /Module,
    pub point*/: *mut *mut u32 entry_pt; /Module entry,
    pub status*/: *mut *mut u8 status; /module,
    pub rsvd1: u8,
    pub rsvd2: u16,
}

//
// Structure for managing the Library Region(1.5MB)
// in DDR in Merrifield
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_mem_mgr {
    pub current_base: phys_addr_t,
    pub avail: c_int,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ipc_reg {
    pub ipcx: c_int,
    pub ipcd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_fw_save {
    pub /: *mut *mut *mut void iram; / allocated via kvmalloc(),
    pub /: *mut *mut *mut void dram; / allocated via kvmalloc(),
    pub /: *mut *mut *mut void sram; / allocated via kvmalloc(),
    pub /: *mut *mut *mut void ddr; / allocated via kvmalloc(),
}

//
// struct intel_sst_drv - driver ops
//
// @sst_state : current sst device state
// @dev_id : device identifier, pci_id for pci devices and acpi_id for acpi
// devices
// @shim : SST shim pointer
// @mailbox : SST mailbox pointer
// @iram : SST IRAM pointer
// @dram : SST DRAM pointer
// @pdata : SST info passed as a part of pci platform data
// @shim_phy_add : SST shim phy addr
// @ipc_dispatch_list : ipc messages dispatched
// @rx_list : to copy the process_reply/process_msg from DSP
// @ipc_post_msg_wq : wq to post IPC messages context
// @mad_ops : MAD driver operations registered
// @mad_wq : MAD driver wq
// @post_msg_wq : wq to post IPC messages
// @streams : sst stream contexts
// @list_lock : sst driver list lock (deprecated)
// @ipc_spin_lock : spin lock to handle audio shim access and ipc queue
// @block_lock : spin lock to add block to block_list and assign pvt_id
// @rx_msg_lock : spin lock to handle the rx messages from the DSP
// @scard_ops : sst card ops
// @pci : sst pci device struture
// @dev : pointer to current device struct
// @sst_lock : sst device lock
// @pvt_id : sst private id
// @stream_cnt : total sst active stream count
// @pb_streams : total active pb streams
// @cp_streams : total active cp streams
// @audio_start : audio status
// @qos		: PM Qos struct
// firmware_name : Firmware / Library name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sst_drv {
    pub sst_state: c_int,
    pub irq_num: c_int,
    pub dev_id: c_ushort,
    pub ddr: *mut void __iomem,
    pub shim: *mut void __iomem,
    pub mailbox: *mut void __iomem,
    pub iram: *mut void __iomem,
    pub dram: *mut void __iomem,
    pub mailbox_add: c_uint,
    pub iram_base: c_uint,
    pub dram_base: c_uint,
    pub shim_phy_add: c_uint,
    pub iram_end: c_uint,
    pub dram_end: c_uint,
    pub ddr_end: c_uint,
    pub ddr_base: c_uint,
    pub mailbox_recv_offset: c_uint,
    pub block_list: list_head,
    pub ipc_dispatch_list: list_head,
    pub pdata: *mut sst_platform_info,
    pub rx_list: list_head,
    pub ipc_post_msg_wq: work_struct,
    pub wait_queue: wait_queue_head_t,
    pub post_msg_wq: *mut workqueue_struct,
    pub tstamp: c_uint,
// str_id 0 is not used
    pub streams: [stream_info; MAX_NUM_STREAMS+1],
    pub ipc_spin_lock: spinlock_t,
    pub block_lock: spinlock_t,
    pub rx_msg_lock: spinlock_t,
    pub pci: *mut pci_dev,
    pub dev: *mut device,
    pub pvt_id: volatile long unsigned,
    pub sst_lock: mutex,
    pub stream_cnt: c_uint,
    pub csr_value: c_uint,
    pub fw_in_mem: *mut c_void,
    pub library_list: sst_sg_list fw_sg_list,,
    pub ops: *mut intel_sst_ops,
    pub info: sst_info,
    pub qos: *mut pm_qos_request,
    pub use_dma: c_uint,
    pub use_lli: c_uint,
    pub fw_clear_context: core::sync::atomic::AtomicI32,
    pub lib_dwnld_reqd: bool,
    pub memcpy_list: list_head,
    pub ipc_reg: sst_ipc_reg,
    pub lib_mem_mgr: sst_mem_mgr,
//
// Holder for firmware name. Due to async call it needs to be
// persistent till worker thread gets called
//
    pub firmware_name: [c_char; FW_NAME_SIZE],
    pub fw_version: snd_sst_fw_version,
    pub fw_save: *mut sst_fw_save,
}

// misc definitions
pub const FW_DWNL_ID: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sst_ops {
    pub ): *mut *mut irqreturn_t (interrupt)(int, void,
    pub ): *mut *mut irqreturn_t (irq_thread)(int, void,
    pub ctx): *mut *mut void (clear_interrupt)(struct intel_sst_drv,
    pub ctx): *mut *mut int (start)(struct intel_sst_drv,
    pub ctx): *mut *mut int (reset)(struct intel_sst_drv,
    pub msg): *mut *mut *mut void (process_reply)(struct intel_sst_drv ctx, struct ipc_post,
    pub sync): *mut *mut ipc_post msg, bool,
    pub msg): *mut *mut void (process_message)(struct ipc_post,
    pub set): *mut *mut void (set_bypass)(bool,
    pub sst): *mut *mut int (save_dsp_context)(struct intel_sst_drv,
    pub (*restore_dsp_context)(void): *mut c_void,
    pub params): *mut *mut *mut int (alloc_stream)(struct intel_sst_drv ctx, void,
    pub sst): *mut *mut void (post_download)(struct intel_sst_drv,
}

extern "C" {
    pub fn sst_realloc_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_pause_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_resume_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_drop_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_free_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_start_stream(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_set_stream_param(str_id: c_int, str_param: *mut snd_sst_params) -> c_int;
}
extern "C" {
    pub fn sst_set_metadata(str_id: c_int, params: *mut c_char) -> c_int;
}
extern "C" {
    pub fn sst_process_reply_mrfld(sst_drv_ctx: *mut intel_sst_drv, msg: *mut ipc_post);
}
extern "C" {
    pub fn sst_start_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn intel_sst_reset_dsp_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn intel_sst_clear_intr_mrfld(sst_drv_ctx: *mut intel_sst_drv);
}
extern "C" {
    pub fn sst_load_fw(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_load_library(lib: *mut snd_sst_lib_download, ops: u8) -> c_int;
}
extern "C" {
    pub fn sst_post_download_mrfld(ctx: *mut intel_sst_drv);
}
extern "C" {
    pub fn sst_get_block_stream(sst_drv_ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_memcpy_free_resources(sst_drv_ctx: *mut intel_sst_drv);
}
extern "C" {
    pub fn sst_create_ipc_msg(arg: *mut ipc_post, large: bool) -> c_int;
}
extern "C" {
    pub fn free_stream_context(ctx: *mut intel_sst_drv, str_id: c_uint) -> c_int;
}
extern "C" {
    pub fn sst_clean_stream(stream: *mut stream_info);
}
extern "C" {
    pub fn intel_sst_register_compress(sst: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn intel_sst_remove_compress(sst: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_send_sync_msg(ipc: c_int, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn sst_get_num_channel(str_param: *mut snd_sst_params) -> c_int;
}
extern "C" {
    pub fn sst_get_sfreq(str_param: *mut snd_sst_params) -> c_int;
}
extern "C" {
    pub fn sst_alloc_stream_mrfld(sst_drv_ctx: *mut intel_sst_drv, params: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sst_restore_fw_context();
}
extern "C" {
    pub fn sst_free_block(ctx: *mut intel_sst_drv, freed: *mut sst_block) -> c_int;
}
extern "C" {
    pub fn sst_request_firmware_async(ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_driver_ops(sst: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_firmware_load_cb(fw: *const firmware, context: *mut c_void);
}
extern "C" {
    pub fn sst_process_pending_msg(work: *mut work_struct);
}
extern "C" {
    pub fn sst_assign_pvt_id(drv: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_validate_strid(sst_drv_ctx: *mut intel_sst_drv, str_id: c_int) -> c_int;
}
extern "C" {
    pub fn relocate_imr_addr_mrfld(base_addr: u32) -> u32;
}
extern "C" {
    pub fn sst_pm_runtime_put(sst_drv: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_shim_write(addr: *mut void __iomem, offset: c_int, value: c_int) -> c_int;
}
extern "C" {
    pub fn sst_shim_read(addr: *mut void __iomem, offset: c_int) -> u32;
}
extern "C" {
    pub fn sst_reg_read64(addr: *mut void __iomem, offset: c_int) -> u64;
}
extern "C" {
    pub fn sst_shim_write64(addr: *mut void __iomem, offset: c_int, value: u64) -> c_int;
}
extern "C" {
    pub fn sst_shim_read64(addr: *mut void __iomem, offset: c_int) -> u64;
}
extern "C" {
    pub fn sst_register(: *mut device) -> c_int;
}
extern "C" {
    pub fn sst_unregister(: *mut device) -> c_int;
}
extern "C" {
    pub fn sst_context_init(ctx: *mut intel_sst_drv) -> c_int;
}
extern "C" {
    pub fn sst_context_cleanup(ctx: *mut intel_sst_drv);
}
extern "C" {
    pub fn sst_configure_runtime_pm(ctx: *mut intel_sst_drv);
}
extern "C" {
    pub fn memcpy32_toio(dst: *mut void __iomem, src: *const c_void, count: c_int);
}
extern "C" {
    pub fn memcpy32_fromio(dst: *mut c_void, src: *const void __iomem, count: c_int);
}
