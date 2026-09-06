//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_psp.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Author: Huang Rui
//

pub const PSP_FENCE_BUFFER_SIZE: c_uint = 0x1000;
pub const PSP_CMD_BUFFER_SIZE: c_uint = 0x1000;
pub const PSP_1_MEG: c_uint = 0x100000;

pub const PSP_TMR_ALIGNMENT: c_uint = 0x100000;
pub const PSP_FW_NAME_LEN: c_uint = 0x24;
// VBIOS gfl defines
pub const MBOX_READY_MASK: c_uint = 0x80000000;
pub const MBOX_STATUS_MASK: c_uint = 0x0000FFFF;
pub const MBOX_COMMAND_MASK: c_uint = 0x00FF0000;
pub const MBOX_READY_FLAG: c_uint = 0x80000000;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_LO: c_uint = 0x2;
pub const C2PMSG_CMD_SPI_UPDATE_ROM_IMAGE_ADDR_HI: c_uint = 0x3;
pub const C2PMSG_CMD_SPI_UPDATE_FLASH_IMAGE: c_uint = 0x4;
pub const C2PMSG_CMD_SPI_GET_ROM_IMAGE_ADDR_LO: c_uint = 0xf;
pub const C2PMSG_CMD_SPI_GET_ROM_IMAGE_ADDR_HI: c_uint = 0x10;
pub const C2PMSG_CMD_SPI_GET_FLASH_IMAGE: c_uint = 0x11;
// Command register bit 31 set to indicate readiness

// Values to check for a successful GFX_CMD response wait. Check against
// both status bits and response state - helps to detect a command failure
// or other unexpected cases like a device drop reading all 0xFFs
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_shared_mem_size {
    PSP_ASD_SHARED_MEM_SIZE				= 0x0,
    PSP_XGMI_SHARED_MEM_SIZE			= 0x4000,
    PSP_RAS_SHARED_MEM_SIZE				= 0x4000,
    PSP_HDCP_SHARED_MEM_SIZE			= 0x4000,
    PSP_DTM_SHARED_MEM_SIZE				= 0x4000,
    PSP_RAP_SHARED_MEM_SIZE				= 0x4000,
    PSP_SECUREDISPLAY_SHARED_MEM_SIZE	= 0x4000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_type_id {
    TA_TYPE_XGMI = 1,
    TA_TYPE_RAS,
    TA_TYPE_HDCP,
    TA_TYPE_DTM,
    TA_TYPE_RAP,
    TA_TYPE_SECUREDISPLAY,

    TA_TYPE_MAX_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_bootloader_cmd {
    PSP_BL__LOAD_SYSDRV		= 0x10000,
    PSP_BL__LOAD_SOSDRV		= 0x20000,
    PSP_BL__LOAD_KEY_DATABASE	= 0x80000,
    PSP_BL__LOAD_SOCDRV             = 0xB0000,
    PSP_BL__LOAD_DBGDRV             = 0xC0000,
    PSP_BL__LOAD_HADDRV		= PSP_BL__LOAD_DBGDRV,
    PSP_BL__LOAD_INTFDRV		= 0xD0000,
    PSP_BL__LOAD_RASDRV		= 0xE0000,
    PSP_BL__LOAD_IPKEYMGRDRV	= 0xF0000,
    PSP_BL__DRAM_LONG_TRAIN		= 0x100000,
    PSP_BL__DRAM_SHORT_TRAIN	= 0x200000,
    PSP_BL__LOAD_TOS_SPL_TABLE	= 0x10000000,
    PSP_BL__LOAD_SPDMDRV		= 0x20000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_ring_type {
    PSP_RING_TYPE__INVALID = 0,
//
// These values map to the way the PSP kernel identifies the
// rings.
//
    PSP_RING_TYPE__UM = 1, /* User mode ring (formerly called RBI) */
    PSP_RING_TYPE__KM = 2  /* Kernel mode ring (formerly called GPCOM) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_ring {
    pub ring_type: psp_ring_type,
    pub ring_mem: *mut psp_gfx_rb_frame,
    pub ring_mem_mc_addr: u64,
    pub ring_mem_handle: *mut c_void,
    pub ring_size: u32,
    pub ring_wptr: u32,
}

// More registers may will be supported
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_reg_prog_id {
    PSP_REG_IH_RB_CNTL        = 0,  /* register IH_RB_CNTL */
    PSP_REG_IH_RB_CNTL_RING1  = 1,  /* register IH_RB_CNTL_RING1 */
    PSP_REG_IH_RB_CNTL_RING2  = 2,  /* register IH_RB_CNTL_RING2 */
    PSP_REG_MMHUB_L1_TLB_CNTL = 25,
    PSP_REG_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_funcs {
    pub psp): *mut *mut int (init_microcode)(struct psp_context,
    pub psp): *mut *mut int (wait_for_bootloader)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_kdb)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_spl)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_sysdrv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_soc_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_intf_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_dbg_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_ras_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_ipkeymgr_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_spdm_drv)(struct psp_context,
    pub psp): *mut *mut int (bootloader_load_sos)(struct psp_context,
    pub ring_type): psp_ring_type,
    pub ring_type): psp_ring_type,
    pub ring_type): psp_ring_type,
    pub psp): *mut *mut bool (smu_reload_quirk)(struct psp_context,
    pub psp): *mut *mut int (mode1_reset)(struct psp_context,
    pub ops): *mut *mut *mut int (mem_training)(struct psp_context psp, uint32_t,
    pub psp): *mut *mut uint32_t (ring_get_wptr)(struct psp_context,
    pub value): *mut *mut *mut void (ring_set_wptr)(struct psp_context psp, uint32_t,
    pub fw_pri_mc_addr): *mut *mut *mut int (load_usbc_pd_fw)(struct psp_context psp, uint64_t,
    pub fw_ver): *mut *mut *mut int (read_usbc_pd_fw)(struct psp_context psp, uint32_t,
    pub fw_pri_mc_addr): *mut *mut *mut int (update_spirom)(struct psp_context psp, uint64_t,
    pub fw_pri_mc_addr): *mut *mut *mut int (dump_spirom)(struct psp_context psp, uint64_t,
    pub psp): *mut *mut int (vbflash_stat)(struct psp_context,
    pub psp): *mut *mut int (fatal_error_recovery_quirk)(struct psp_context,
    pub psp): *mut *mut bool (get_ras_capability)(struct psp_context,
    pub psp): *mut *mut bool (is_aux_sos_load_required)(struct psp_context,
    pub psp): *mut *mut bool (is_reload_needed)(struct psp_context,
    pub id): psp_reg_prog_id,
    pub type): *mut psp_gfx_fw_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_funcs {
    pub psp): *mut *mut int (fn_ta_initialize)(struct psp_context,
    pub ta_cmd_id): *mut *mut *mut int (fn_ta_invoke)(struct psp_context psp, uint32_t,
    pub psp): *mut *mut int (fn_ta_terminate)(struct psp_context,
}

pub const AMDGPU_XGMI_MAX_CONNECTED_NODES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_xgmi_node_info {
    pub node_id: u64,
    pub num_hops: u8,
    pub is_sharing_enabled: u8,
    pub sdma_engine: ta_xgmi_assigned_sdma_engine,
    pub num_links: u8,
    pub port_num: [xgmi_connected_port_num; TA_XGMI__MAX_PORT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_xgmi_topology_info {
    pub num_nodes: u32,
    pub nodes: [psp_xgmi_node_info; AMDGPU_XGMI_MAX_CONNECTED_NODES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_bin_desc {
    pub fw_version: u32,
    pub feature_version: u32,
    pub size_bytes: u32,
    pub start_addr: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_mem_context {
    pub shared_bo: *mut amdgpu_bo,
    pub shared_mc_addr: u64,
    pub shared_buf: *mut c_void,
    pub shared_mem_size: psp_shared_mem_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_context {
    pub initialized: bool,
    pub session_id: u32,
    pub resp_status: u32,
    pub mem_context: ta_mem_context,
    pub bin_desc: psp_bin_desc,
    pub ta_load_type: psp_gfx_cmd_id,
    pub ta_type: ta_type_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_cp_context {
    pub context: ta_context,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_xgmi_context {
    pub context: ta_context,
    pub top_info: psp_xgmi_topology_info,
    pub supports_extended_data: bool,
    pub supports_ext_link_info: bool,
    pub xgmi_ta_caps: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_ras_context {
    pub context: ta_context,
    pub ras: *mut amdgpu_ras,
    pub mutex: mutex,
}

pub const MEM_TRAIN_SYSTEM_SIGNATURE: c_uint = 0x54534942;
pub const GDDR6_MEM_TRAINING_DATA_SIZE_IN_BYTES: c_uint = 0x1000;
pub const GDDR6_MEM_TRAINING_OFFSET: c_uint = 0x8000;
// Define the VRAM size that will be encroached by BIST training.
pub const BIST_MEM_TRAINING_ENCROACHED_SIZE: c_uint = 0x2000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_memory_training_init_flag {
    PSP_MEM_TRAIN_NOT_SUPPORT	= 0x0,
    PSP_MEM_TRAIN_SUPPORT		= 0x1,
    PSP_MEM_TRAIN_INIT_FAILED	= 0x2,
    PSP_MEM_TRAIN_RESERVE_SUCCESS	= 0x4,
    PSP_MEM_TRAIN_INIT_SUCCESS	= 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_memory_training_ops {
    PSP_MEM_TRAIN_SEND_LONG_MSG	= 0x1,
    PSP_MEM_TRAIN_SAVE		= 0x2,
    PSP_MEM_TRAIN_RESTORE		= 0x4,
    PSP_MEM_TRAIN_SEND_SHORT_MSG	= 0x8,
    PSP_MEM_TRAIN_COLD_BOOT		= PSP_MEM_TRAIN_SEND_LONG_MSG,
    PSP_MEM_TRAIN_RESUME		= PSP_MEM_TRAIN_SEND_SHORT_MSG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_memory_training_context {
// training data size
    pub train_data_size: u64,
//
// sys_cache
// cpu virtual address
// system memory buffer that used to store the training data.
//
    pub sys_cache: *mut c_void,
// vram offset of the p2c training data
    pub p2c_train_data_offset: u64,
// vram offset of the c2p training data
    pub c2p_train_data_offset: u64,
    pub init: psp_memory_training_init_flag,
    pub training_cnt: u32,
    pub enable_mem_training: bool,
}

// PSP runtime DB
pub const PSP_RUNTIME_DB_SIZE_IN_BYTES: c_uint = 0x10000;
pub const PSP_RUNTIME_DB_OFFSET: c_uint = 0x100000;
pub const PSP_RUNTIME_DB_COOKIE_ID: c_uint = 0x0ed5;
pub const PSP_RUNTIME_DB_VER_1: c_uint = 0x0100;
pub const PSP_RUNTIME_DB_DIAG_ENTRY_MAX_COUNT: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_runtime_entry_type {
    PSP_RUNTIME_ENTRY_TYPE_INVALID		= 0x0,
    PSP_RUNTIME_ENTRY_TYPE_TEST		= 0x1,
    PSP_RUNTIME_ENTRY_TYPE_MGPU_COMMON	= 0x2,  /* Common mGPU runtime data */
    PSP_RUNTIME_ENTRY_TYPE_MGPU_WAFL	= 0x3,  /* WAFL runtime data */
    PSP_RUNTIME_ENTRY_TYPE_MGPU_XGMI	= 0x4,  /* XGMI runtime data */
    PSP_RUNTIME_ENTRY_TYPE_BOOT_CONFIG	= 0x5,  /* Boot Config runtime data */
    PSP_RUNTIME_ENTRY_TYPE_PPTABLE_ERR_STATUS = 0x6, /* SCPM validation data */
}

// PSP runtime DB header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_runtime_data_header {
// determine the existence of runtime db
    pub cookie: u16,
// version of runtime db
    pub version: u16,
}

// PSP runtime DB entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_runtime_entry {
// type of runtime db entry
    pub entry_type: u32,
// offset of entry in bytes
    pub offset: u16,
// size of entry in bytes
    pub size: u16,
}

// PSP runtime DB directory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_runtime_data_directory {
// number of valid entries
    pub entry_count: u16,
// db entries
    pub entry_list: [psp_runtime_entry; PSP_RUNTIME_DB_DIAG_ENTRY_MAX_COUNT],
}

// PSP runtime DB boot config feature bitmask
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_runtime_boot_cfg_feature {
    BOOT_CFG_FEATURE_GECC                       = 0x1,
    BOOT_CFG_FEATURE_TWO_STAGE_DRAM_TRAINING    = 0x2,
}

// PSP run time DB SCPM authentication defines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_runtime_scpm_authentication {
    SCPM_DISABLE                     = 0x0,
    SCPM_ENABLE                      = 0x1,
    SCPM_ENABLE_WITH_SCPM_ERR        = 0x2,
}

// PSP runtime DB boot config entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_runtime_boot_cfg_entry {
    pub boot_cfg_bitmask: u32,
    pub reserved: u32,
}

// PSP runtime DB SCPM entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_runtime_scpm_entry {
    pub scpm_status: psp_runtime_scpm_authentication,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spirom_bo {
    pub bo: *mut amdgpu_bo,
    pub mc_addr: u64,
    pub cpu_addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_ptl_cmd {
    PSP_PTL_PERF_MON_QUERY = 0xA0000000,
    PSP_PTL_PERF_MON_SET = 0xA0000001,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_ptl_format_type {
    GFX_FTYPE_I8          = 0x00000000,
    GFX_FTYPE_F16         = 0x00000001,
    GFX_FTYPE_BF16        = 0x00000002,
    GFX_FTYPE_F32         = 0x00000003,
    GFX_FTYPE_F64         = 0x00000004,
    GFX_FTYPE_F8          = 0x00000005,
    GFX_FTYPE_VECTOR      = 0x00000006,
    GFX_FTYPE_INVALID     = 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_ptl_perf_req {
    pub req: psp_ptl_cmd,
    pub ptl_state: u32,
    pub pref_format1: u32,
    pub pref_format2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_context {
    pub adev: *mut amdgpu_device,
    pub km_ring: psp_ring,
    pub cmd: *mut psp_gfx_cmd_resp,
    pub funcs: *const psp_funcs,
    pub ta_funcs: *const ta_funcs,
// firmware buffer
    pub fw_pri_bo: *mut amdgpu_bo,
    pub fw_pri_mc_addr: u64,
    pub fw_pri_buf: *mut c_void,
// sos firmware
    pub sos_fw: *const firmware,
    pub sys: psp_bin_desc,
    pub sos: psp_bin_desc,
    pub toc: psp_bin_desc,
    pub kdb: psp_bin_desc,
    pub spl: psp_bin_desc,
    pub rl: psp_bin_desc,
    pub soc_drv: psp_bin_desc,
    pub intf_drv: psp_bin_desc,
    pub dbg_drv: psp_bin_desc,
    pub ras_drv: psp_bin_desc,
    pub ipkeymgr_drv: psp_bin_desc,
    pub spdm_drv: psp_bin_desc,
// tmr buffer
    pub tmr_bo: *mut amdgpu_bo,
    pub tmr_mc_addr: u64,
// asd firmware
    pub asd_fw: *const firmware,
// toc firmware
    pub toc_fw: *const firmware,
// cap firmware
    pub cap_fw: *const firmware,
// fence buffer
    pub fence_buf_bo: *mut amdgpu_bo,
    pub fence_buf_mc_addr: u64,
    pub fence_buf: *mut c_void,
// cmd buffer
    pub cmd_buf_bo: *mut amdgpu_bo,
    pub cmd_buf_mc_addr: u64,
    pub cmd_buf_mem: *mut psp_gfx_cmd_resp,
// fence value associated with cmd buffer
    pub fence_value: core::sync::atomic::AtomicI32,
// flag to mark whether gfx fw autoload is supported or not
    pub autoload_supported: bool,
// flag to mark whether psp use runtime TMR or boottime TMR
    pub boot_time_tmr: bool,
// flag to mark whether df cstate management centralized to PMFW
    pub pmfw_centralized_cstate_management: bool,
// xgmi ta firmware and buffer
    pub ta_fw: *const firmware,
    pub ta_fw_version: u32,
    pub cap_fw_version: u32,
    pub cap_feature_version: u32,
    pub cap_ucode_size: u32,
    pub asd_context: ta_context,
    pub xgmi_context: psp_xgmi_context,
    pub ras_context: psp_ras_context,
    pub hdcp_context: ta_cp_context,
    pub dtm_context: ta_cp_context,
    pub rap_context: ta_cp_context,
    pub securedisplay_context: ta_cp_context,
    pub mutex: mutex,
    pub mem_train_ctx: psp_memory_training_context,
    pub boot_cfg_bitmask: u32,
// firmware upgrades supported
    pub sup_pd_fw_up: bool,
    pub sup_ifwi_up: bool,
    pub vbflash_tmp_buf: *mut c_char,
    pub vbflash_image_size: usize,
    pub vbflash_done: bool,

    pub spirom_dump_trip: *mut spirom_bo,

    pub ptl: amdgpu_ptl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_psp_funcs {
    pub AMDGPU_UCODE_ID): enum,
}

extern "C" {
    pub fn psp_gpu_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn psp_ta_free_shared_buf(mem_ctx: *mut ta_mem_context);
}
extern "C" {
    pub fn psp_ta_unload(psp: *mut psp_context, context: *mut ta_context) -> c_int;
}
extern "C" {
    pub fn psp_ta_load(psp: *mut psp_context, context: *mut ta_context) -> c_int;
}
extern "C" {
    pub fn psp_xgmi_initialize(psp: *mut psp_context, set_extended_data: bool, load_ta: bool) -> c_int;
}
extern "C" {
    pub fn psp_xgmi_terminate(psp: *mut psp_context) -> c_int;
}
extern "C" {
    pub fn psp_xgmi_invoke(psp: *mut psp_context, ta_cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn psp_xgmi_get_hive_id(psp: *mut psp_context, hive_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn psp_xgmi_get_node_id(psp: *mut psp_context, node_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn psp_ras_initialize(psp: *mut psp_context) -> c_int;
}
extern "C" {
    pub fn psp_ras_invoke(psp: *mut psp_context, ta_cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn psp_ras_terminate(psp: *mut psp_context) -> c_int;
}
extern "C" {
    pub fn psp_hdcp_invoke(psp: *mut psp_context, ta_cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn psp_dtm_invoke(psp: *mut psp_context, ta_cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn psp_rap_invoke(psp: *mut psp_context, ta_cmd_id: u32, status: *mut ta_rap_status) -> c_int;
}
extern "C" {
    pub fn psp_securedisplay_invoke(psp: *mut psp_context, ta_cmd_id: u32) -> c_int;
}
extern "C" {
    pub fn psp_rlc_autoload_start(psp: *mut psp_context) -> c_int;
}
extern "C" {
    pub fn psp_update_fw_reservation(psp: *mut psp_context) -> c_int;
}
extern "C" {
    pub fn psp_copy_fw(psp: *mut psp_context, start_addr: *mut u8, bin_size: u32) -> c_int;
}
extern "C" {
    pub fn psp_spatial_partition(psp: *mut psp_context, mode: c_int) -> c_int;
}
extern "C" {
    pub fn psp_memory_partition(psp: *mut psp_context, mode: c_int) -> c_int;
}
extern "C" {
    pub fn is_psp_fw_valid(bin: psp_bin_desc) -> c_int;
}
extern "C" {
    pub fn amdgpu_psp_wait_for_bootloader(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_psp_get_ras_capability(psp: *mut psp_context) -> bool;
}
extern "C" {
    pub fn amdgpu_psp_tos_reload_needed(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_psp_debugfs_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn psp_set_mmhub_eco_sec_level(adev: *mut amdgpu_device) -> c_int;
}
