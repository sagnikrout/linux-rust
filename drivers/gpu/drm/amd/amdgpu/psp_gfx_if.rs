//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/psp_gfx_if.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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
pub const PSP_GFX_CMD_BUF_VERSION: c_uint = 0x00000001;
pub const GFX_CMD_STATUS_MASK: c_uint = 0x0000FFFF;
pub const GFX_CMD_ID_MASK: c_uint = 0x000F0000;
pub const GFX_CMD_RESERVED_MASK: c_uint = 0x7FF00000;
pub const GFX_CMD_RESPONSE_MASK: c_uint = 0x80000000;
// USBC PD FW version retrieval command
pub const C2PMSG_CMD_GFX_USB_PD_FW_VER: c_uint = 0x2000000;
// TEE Gfx Command IDs for the register interface.
// Command ID must be between 0x00010000 and 0x000F0000.
//
// -----------------------------------------------------------------------------
//
// Control registers of the TEE Gfx interface. These are located in
// SRBM-to-PSP mailbox registers (total 8 registers).
//
// Response flag is set in the command when command is completed by PSP.
// Used in the GFX_CTRL.CmdResp.
// When PSP GFX I/F is initialized, the flag is set.
//
pub const GFX_FLAG_RESPONSE: c_uint = 0x80000000;
// TEE Gfx Command IDs for the ring buffer interface.
// IDs upto 0x1F are reserved for older programs (Raven, Vega 10/12/20)
// IDs of performance monitoring/profiling
// Dynamic memory partitioninig (NPS mode change)
// PSP boot config sub-commands
// PSP boot config bitmask values
// Command to load Trusted Application binary into PSP OS.
// Note: CmdBufLen can be set to 0. In this case no persistent CMD buffer is provided
// for the TA. Each InvokeCommand can have dinamically mapped CMD buffer instead
// of using global persistent buffer.
//
// Command to Unload Trusted Application binary from PSP OS.
// Shared buffers for InvokeCommand.
//
// Max number of descriptors for one shared buffer (in how many different
// physical locations one shared buffer can be stored). If buffer is too much
// fragmented, error will be returned.
//
pub const GFX_BUF_MAX_DESC: c_int = 64;
// total 776 bytes
// Command to execute InvokeCommand entry point of the TA.
// Command to setup TMR region.
// FW types for GFX_CMD_ID_LOAD_IP_FW command. Limit 31.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_gfx_fw_type {
    GFX_FW_TYPE_NONE        = 0,    /* */
    GFX_FW_TYPE_CP_ME       = 1,    /* CP-ME                    VG + RV */
    GFX_FW_TYPE_CP_PFP      = 2,    /* CP-PFP                   VG + RV */
    GFX_FW_TYPE_CP_CE       = 3,    /* CP-CE                    VG + RV */
    GFX_FW_TYPE_CP_MEC      = 4,    /* CP-MEC FW                VG + RV */
    GFX_FW_TYPE_CP_MEC_ME1  = 5,    /* CP-MEC Jump Table 1      VG + RV */
    GFX_FW_TYPE_CP_MEC_ME2  = 6,    /* CP-MEC Jump Table 2      VG      */
    GFX_FW_TYPE_RLC_V       = 7,    /* RLC-V                    VG      */
    GFX_FW_TYPE_RLC_G       = 8,    /* RLC-G                    VG + RV */
    GFX_FW_TYPE_SDMA0       = 9,    /* SDMA0                    VG + RV */
    GFX_FW_TYPE_SDMA1       = 10,   /* SDMA1                    VG      */
    GFX_FW_TYPE_DMCU_ERAM   = 11,   /* DMCU-ERAM                VG + RV */
    GFX_FW_TYPE_DMCU_ISR    = 12,   /* DMCU-ISR                 VG + RV */
    GFX_FW_TYPE_VCN         = 13,   /* VCN                           RV */
    GFX_FW_TYPE_UVD         = 14,   /* UVD                      VG      */
    GFX_FW_TYPE_VCE         = 15,   /* VCE                      VG      */
    GFX_FW_TYPE_ISP         = 16,   /* ISP                           RV */
    GFX_FW_TYPE_ACP         = 17,   /* ACP                           RV */
    GFX_FW_TYPE_SMU         = 18,   /* SMU                      VG      */
    GFX_FW_TYPE_MMSCH       = 19,   /* MMSCH                    VG      */
    GFX_FW_TYPE_RLC_RESTORE_LIST_GPM_MEM        = 20,   /* RLC GPM                  VG + RV */
    GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_MEM        = 21,   /* RLC SRM                  VG + RV */
    GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_CNTL       = 22,   /* RLC CNTL                 VG + RV */
    GFX_FW_TYPE_UVD1        = 23,   /* UVD1                     VG-20   */
    GFX_FW_TYPE_TOC         = 24,   /* TOC                      NV-10   */
    GFX_FW_TYPE_RLC_P                           = 25,   /* RLC P                    NV      */
    GFX_FW_TYPE_RLC_IRAM                        = 26,   /* RLC_IRAM                 NV      */
    GFX_FW_TYPE_GLOBAL_TAP_DELAYS               = 27,   /* GLOBAL TAP DELAYS        NV      */
    GFX_FW_TYPE_SE0_TAP_DELAYS                  = 28,   /* SE0 TAP DELAYS           NV      */
    GFX_FW_TYPE_SE1_TAP_DELAYS                  = 29,   /* SE1 TAP DELAYS           NV      */
    GFX_FW_TYPE_GLOBAL_SE0_SE1_SKEW_DELAYS      = 30,   /* GLOBAL SE0/1 SKEW DELAYS NV      */
    GFX_FW_TYPE_SDMA0_JT                        = 31,   /* SDMA0 JT                 NV      */
    GFX_FW_TYPE_SDMA1_JT                        = 32,   /* SDNA1 JT                 NV      */
    GFX_FW_TYPE_CP_MES                          = 33,   /* CP MES                   NV      */
    GFX_FW_TYPE_MES_STACK                       = 34,   /* MES STACK                NV      */
    GFX_FW_TYPE_RLC_SRM_DRAM_SR                 = 35,   /* RLC SRM DRAM             NV      */
    GFX_FW_TYPE_RLCG_SCRATCH_SR                 = 36,   /* RLCG SCRATCH             NV      */
    GFX_FW_TYPE_RLCP_SCRATCH_SR                 = 37,   /* RLCP SCRATCH             NV      */
    GFX_FW_TYPE_RLCV_SCRATCH_SR                 = 38,   /* RLCV SCRATCH             NV      */
    GFX_FW_TYPE_RLX6_DRAM_SR                    = 39,   /* RLX6 DRAM                NV      */
    GFX_FW_TYPE_SDMA0_PG_CONTEXT                = 40,   /* SDMA0 PG CONTEXT         NV      */
    GFX_FW_TYPE_SDMA1_PG_CONTEXT                = 41,   /* SDMA1 PG CONTEXT         NV      */
    GFX_FW_TYPE_GLOBAL_MUX_SELECT_RAM           = 42,   /* GLOBAL MUX SEL RAM       NV      */
    GFX_FW_TYPE_SE0_MUX_SELECT_RAM              = 43,   /* SE0 MUX SEL RAM          NV      */
    GFX_FW_TYPE_SE1_MUX_SELECT_RAM              = 44,   /* SE1 MUX SEL RAM          NV      */
    GFX_FW_TYPE_ACCUM_CTRL_RAM                  = 45,   /* ACCUM CTRL RAM           NV      */
    GFX_FW_TYPE_RLCP_CAM                        = 46,   /* RLCP CAM                 NV      */
    GFX_FW_TYPE_RLC_SPP_CAM_EXT                 = 47,   /* RLC SPP CAM EXT          NV      */
    GFX_FW_TYPE_RLC_DRAM_BOOT                   = 48,   /* RLC DRAM BOOT            NV      */
    GFX_FW_TYPE_VCN0_RAM                        = 49,   /* VCN_RAM                  NV + RN */
    GFX_FW_TYPE_VCN1_RAM                        = 50,   /* VCN_RAM                  NV + RN */
    GFX_FW_TYPE_DMUB                            = 51,   /* DMUB                          RN */
    GFX_FW_TYPE_SDMA2                           = 52,   /* SDMA2                    MI      */
    GFX_FW_TYPE_SDMA3                           = 53,   /* SDMA3                    MI      */
    GFX_FW_TYPE_SDMA4                           = 54,   /* SDMA4                    MI      */
    GFX_FW_TYPE_SDMA5                           = 55,   /* SDMA5                    MI      */
    GFX_FW_TYPE_SDMA6                           = 56,   /* SDMA6                    MI      */
    GFX_FW_TYPE_SDMA7                           = 57,   /* SDMA7                    MI      */
    GFX_FW_TYPE_VCN1                            = 58,   /* VCN1                     MI      */
    GFX_FW_TYPE_CAP                             = 62,   /* CAP_FW                           */
    GFX_FW_TYPE_SE2_TAP_DELAYS                  = 65,   /* SE2 TAP DELAYS           NV      */
    GFX_FW_TYPE_SE3_TAP_DELAYS                  = 66,   /* SE3 TAP DELAYS           NV      */
    GFX_FW_TYPE_REG_LIST                        = 67,   /* REG_LIST                 MI      */
    GFX_FW_TYPE_IMU_I                           = 68,   /* IMU Instruction FW       SOC21   */
    GFX_FW_TYPE_IMU_D                           = 69,   /* IMU Data FW              SOC21   */
    GFX_FW_TYPE_LSDMA                           = 70,   /* LSDMA FW                 SOC21   */
    GFX_FW_TYPE_SDMA_UCODE_TH0                  = 71,   /* SDMA Thread 0/CTX        SOC21   */
    GFX_FW_TYPE_SDMA_UCODE_TH1                  = 72,   /* SDMA Thread 1/CTL        SOC21   */
    GFX_FW_TYPE_PPTABLE                         = 73,   /* PPTABLE                  SOC21   */
    GFX_FW_TYPE_DISCRETE_USB4                   = 74,   /* dUSB4 FW                 SOC21   */
    GFX_FW_TYPE_TA                              = 75,   /* SRIOV TA FW UUID         SOC21   */
    GFX_FW_TYPE_RS64_MES                        = 76,   /* RS64 MES ucode           SOC21   */
    GFX_FW_TYPE_RS64_MES_STACK                  = 77,   /* RS64 MES stack ucode     SOC21   */
    GFX_FW_TYPE_RS64_KIQ                        = 78,   /* RS64 KIQ ucode           SOC21   */
    GFX_FW_TYPE_RS64_KIQ_STACK                  = 79,   /* RS64 KIQ Heap stack      SOC21   */
    GFX_FW_TYPE_ISP_DATA                        = 80,   /* ISP DATA                 SOC21   */
    GFX_FW_TYPE_CP_MES_KIQ                      = 81,   /* MES KIQ ucode            SOC21   */
    GFX_FW_TYPE_MES_KIQ_STACK                   = 82,   /* MES KIQ stack            SOC21   */
    GFX_FW_TYPE_UMSCH_DATA                      = 83,   /* User Mode Scheduler Data SOC21   */
    GFX_FW_TYPE_UMSCH_UCODE                     = 84,   /* User Mode Scheduler Ucode SOC21  */
    GFX_FW_TYPE_UMSCH_CMD_BUFFER                = 85,   /* User Mode Scheduler Command Buffer SOC21 */
    GFX_FW_TYPE_USB_DP_COMBO_PHY                = 86,   /* USB-Display port Combo   SOC21   */
    GFX_FW_TYPE_RS64_PFP                        = 87,   /* RS64 PFP                 SOC21   */
    GFX_FW_TYPE_RS64_ME                         = 88,   /* RS64 ME                  SOC21   */
    GFX_FW_TYPE_RS64_MEC                        = 89,   /* RS64 MEC                 SOC21   */
    GFX_FW_TYPE_RS64_PFP_P0_STACK               = 90,   /* RS64 PFP stack P0        SOC21   */
    GFX_FW_TYPE_RS64_PFP_P1_STACK               = 91,   /* RS64 PFP stack P1        SOC21   */
    GFX_FW_TYPE_RS64_ME_P0_STACK                = 92,   /* RS64 ME stack P0         SOC21   */
    GFX_FW_TYPE_RS64_ME_P1_STACK                = 93,   /* RS64 ME stack P1         SOC21   */
    GFX_FW_TYPE_RS64_MEC_P0_STACK               = 94,   /* RS64 MEC stack P0        SOC21   */
    GFX_FW_TYPE_RS64_MEC_P1_STACK               = 95,   /* RS64 MEC stack P1        SOC21   */
    GFX_FW_TYPE_RS64_MEC_P2_STACK               = 96,   /* RS64 MEC stack P2        SOC21   */
    GFX_FW_TYPE_RS64_MEC_P3_STACK               = 97,   /* RS64 MEC stack P3        SOC21   */
    GFX_FW_TYPE_RLX6_UCODE_CORE1                = 98,   /* RLCV_IRAM                MI      */
    GFX_FW_TYPE_RLX6_DRAM_BOOT_CORE1            = 99,   /* RLCV DRAM BOOT           MI      */
    GFX_FW_TYPE_VPEC_FW1                        = 100,  /* VPEC FW1 To Save         VPE     */
    GFX_FW_TYPE_VPEC_FW2                        = 101,  /* VPEC FW2 To Save         VPE     */
    GFX_FW_TYPE_VPE                             = 102,
    GFX_FW_TYPE_JPEG_RAM                        = 128,  /**< JPEG Command buffer */
    GFX_FW_TYPE_P2S_TABLE                       = 129,
    GFX_FW_TYPE_MAX
}

// Command to load HW IP FW.
// Command to save/restore HW IP FW.
// Command to setup register program
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_cmd_reg_prog {
    pub reg_value: u32,
    pub reg_id: u32,
}

// Command to load TOC
// Dynamic boot configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_cmd_sriov_spatial_part {
    pub mode: u32,
    pub override_ips: u32,
    pub override_xcds_avail: u32,
    pub override_this_aid: u32,
}

// Structure for sq performance monitoring/profiling enable/disable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_cmd_config_sq_perfmon {
    pub gfx_xcp_mask: u32,
    pub core_override: u8,
    pub reg_override: u8,
    pub perfmon_override: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_cmd_fb_memory_part {
    pub /: *mut *mut uint32_t mode; / requested NPS mode,
    pub resvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_cmd_req_perf_hw {
    pub req: u32,
    pub ptl_state: u32,
    pub pref_format1: u32,
    pub pref_format2: u32,
}

// All GFX ring buffer commands.
// Command-specific response for Fw Attestation Db
// Command-specific response for boot config.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_uresp_bootcfg {
    pub /: *mut *mut uint32_t boot_cfg; / boot config data,
}

// Command-specific response for fw reserve info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_uresp_fw_reserve_info {
    pub reserve_base_address_hi: u32,
    pub reserve_base_address_lo: u32,
    pub reserve_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_gfx_uresp_perf_hw {
    pub resp: u32,
    pub ptl_state: u32,
    pub pref_format1: u32,
    pub pref_format2: u32,
}

// Union of command-specific responses for GPCOM ring.
#[repr(C)]
#[derive(Copy, Clone)]
pub union psp_gfx_uresp {
    pub reserved: psp_gfx_uresp_reserved,
    pub boot_cfg: psp_gfx_uresp_bootcfg,
    pub fwar_db_info: psp_gfx_uresp_fwar_db_info,
    pub fw_reserve_info: psp_gfx_uresp_fw_reserve_info,
    pub perf_hw_info: psp_gfx_uresp_perf_hw,
}

// Structure of GFX Response buffer.
// For GPCOM I/F it is part of GFX_CMD_RESP buffer, for RBI
// it is separate buffer.
//
// total 96 bytes
// Structure of Command buffer pointed by psp_gfx_rb_frame.cmd_buf_addr_hi
// and psp_gfx_rb_frame.cmd_buf_addr_lo.
//
// These fields are used for RBI only. They are all 0 in GPCOM commands
//
// Note: Resp is part of this buffer for GPCOM ring. For RBI ring the response
// is separate buffer pointed by resp_buf_addr_hi and resp_buf_addr_lo.
//
// total size 1024 bytes

// Structure of the Ring Buffer Frame
// total 64 bytes
pub const PSP_ERR_UNKNOWN_COMMAND: c_uint = 0x00000100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tee_error_code {
    TEE_SUCCESS			= 0x00000000,
    TEE_ERROR_CANCEL		= 0xFFFF0002,
    TEE_ERROR_NOT_SUPPORTED		= 0xFFFF000A,
}
