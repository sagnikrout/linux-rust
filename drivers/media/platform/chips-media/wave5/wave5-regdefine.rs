//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5-regdefine.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - wave5 register definitions
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum W5_VPU_COMMAND {
    W5_INIT_VPU		= 0x0001,
    W5_WAKEUP_VPU		= 0x0002,
    W5_SLEEP_VPU		= 0x0004,
    W5_CREATE_INSTANCE	= 0x0008,       /* queuing command */
    W5_FLUSH_INSTANCE	= 0x0010,
    W5_DESTROY_INSTANCE	= 0x0020,       /* queuing command */
    W5_INIT_SEQ		= 0x0040,       /* queuing command */
    W5_SET_FB		= 0x0080,
    W5_DEC_ENC_PIC		= 0x0100,       /* queuing command */
    W5_ENC_SET_PARAM	= 0x0200,	/* queuing command */
    W5_QUERY		= 0x4000,
    W5_UPDATE_BS		= 0x8000,
    W5_MAX_VPU_COMD		= 0x10000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum query_opt {
    GET_VPU_INFO		= 0,
    SET_WRITE_PROT		= 1,
    GET_RESULT		= 2,
    UPDATE_DISP_FLAG	= 3,
    GET_BW_REPORT		= 4,
    GET_BS_RD_PTR		= 5,		/* for decoder */
    GET_BS_WR_PTR		= 6,		/* for encoder */
    GET_SRC_BUF_FLAG	= 7,		/* for encoder */
    SET_BS_RD_PTR		= 8,		/* for decoder */
    GET_DEBUG_INFO		= 0x61,
}

pub const W5_REG_BASE: c_uint = 0x00000000;
pub const W5_CMD_REG_BASE: c_uint = 0x00000100;
pub const W5_CMD_REG_END: c_uint = 0x00000200;
//
// COMMON
//
// ----
//
// Power on configuration
// PO_DEBUG_MODE    [0]     1 - power on with debug mode
// USE_PO_CONF      [3]     1 - use power-on-configuration
//

// REMAP_CTRL
// PAGE SIZE:   [8:0]   0x001 - 4K
// 0x002 - 8K
// 0x004 - 16K
// ...
// 0x100 - 1M
// REGION ATTR1 [10]    0     - normal
// 1     - make bus error for the region
// REGION ATTR2 [11]    0     - normal
// 1     - bypass region
// REMAP INDEX  [15:12]       - 0 ~ 3
// ENDIAN       [19:16]       - NOTE: Currently not supported in this driver
// AXI-ID       [23:20]       - upper AXI-ID
// BUS_ERROR    [29]    0     - bypass
// 1     - make BUS_ERROR for unmapped region
// BYPASS_ALL   [30]    1     - bypass all
// ENABLE       [31]    1     - update control register[30:16]
//

//
// assign vpu_config0          = {conf_map_converter_reg,      // [31]
// conf_map_converter_sig,         // [30]
// 8'd0,                        // [29:22]
// conf_std_switch_en,          // [21]
// conf_bg_detect,              // [20]
// conf_3dnr_en,                // [19]
// conf_one_axi_en,             // [18]
// conf_sec_axi_en,             // [17]
// conf_bus_info,               // [16]
// conf_afbc_en,                // [15]
// conf_afbc_version_id,        // [14:12]
// conf_fbc_en,                 // [11]
// conf_fbc_version_id,         // [10:08]
// conf_scaler_en,              // [07]
// conf_scaler_version_id,      // [06:04]
// conf_bwb_en,                 // [03]
// 3'd0};                       // [02:00]
//

//
// assign vpu_config1          = {4'd0,                        // [31:28]
// conf_perf_timer_en,          // [27]
// conf_multi_core_en,          // [26]
// conf_gcu_en,                 // [25]
// conf_cu_report,              // [24]
// 4'd0,                        // [23:20]
// conf_vcore_id_3,             // [19]
// conf_vcore_id_2,             // [18]
// conf_vcore_id_1,             // [17]
// conf_vcore_id_0,             // [16]
// conf_bwb_opt,                // [15]
// 7'd0,                        // [14:08]
// conf_cod_std_en_reserved_7,  // [7]
// conf_cod_std_en_reserved_6,  // [6]
// conf_cod_std_en_reserved_5,  // [5]
// conf_cod_std_en_reserved_4,  // [4]
// conf_cod_std_en_reserved_3,  // [3]
// conf_cod_std_en_reserved_2,  // [2]
// conf_cod_std_en_vp9,         // [1]
// conf_cod_std_en_hevc};       // [0]
// }
//

//
// PRODUCT INFORMATION
//

//
// DECODER/ENCODER COMMON
//

// return info when QUERY (GET_RESULT) for en/decoder

// return info when QUERY (GET_RESULT) for en/decoder

// set when SET_FB for en/decoder

//
// INIT_VPU - COMMON
//
// note: W5_ADDR_CODE_BASE should be aligned to 4KB

//
// CREATE_INSTANCE - COMMON
//

//
// DECODER - INIT_SEQ
//

//
// SET_FRAME_BUF
//
// SET_FB_OPTION 0x00       REGISTER FRAMEBUFFERS
// 0x01       UPDATE FRAMEBUFFER, just one framebuffer(linear, fbc and mvcol)
//

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// compression offset table for luma

// compression offset table for chroma

// UPDATE_FB
// CMD_SET_FB_STRIDE [15:0]     - FBC framebuffer stride
// [31:15]    - linear framebuffer stride
//

//
// DECODER - DEC_PIC
//

// sequence change enable mask register
// CMD_SEQ_CHANGE_ENABLE_FLAG [5]   profile_idc
// [16]  pic_width/height_in_luma_sample
// [19]  sps_max_dec_pic_buffering, max_num_reorder, max_latency_increase
//

//
// DECODER - QUERY : GET_VPU_INFO
//

//
// DECODER - QUERY : GET_RESULT
//

//
// USER_DATA_FLAGS for HEVC/H264 only.
// Bits:
// [1] - User data buffer full boolean
// [2] - VUI parameter flag
// [4] - Pic_timing SEI flag
// [5] - 1st user_data_registed_itu_t_t35 prefix SEI flag
// [6] - user_data_unregistered prefix SEI flag
// [7] - 1st user_data_registed_itu_t_t35 suffix SEI flag
// [8] - user_data_unregistered suffix SEI flag
// [10]- mastering_display_color_volume prefix SEI flag
// [11]- chroma_resampling_display_color_volume prefix SEI flag
// [12]- knee_function_info SEI flag
// [13]- tone_mapping_info prefix SEI flag
// [14]- film_grain_characteristics_info prefix SEI flag
// [15]- content_light_level_info prefix SEI flag
// [16]- color_remapping_info prefix SEI flag
// [28]- 2nd user_data_registed_itu_t_t35 prefix SEI flag
// [29]- 3rd user_data_registed_itu_t_t35 prefix SEI flag
// [30]- 2nd user_data_registed_itu_t_t35 suffix SEI flag
// [31]- 3rd user_data_registed_itu_t_t35 suffix SEI flag
//

//
// #define W5_RET_DEC_AU_START_POS             (W5_REG_BASE + 0x0158)
// => Access unit (AU) Bitstream start position
// #define W5_RET_DEC_AU_END_POS               (W5_REG_BASE + 0x015C)
// => Access unit (AU) Bitstream end position
//
// Decoded picture type:
// reg_val & 0x7			=> picture type
// (reg_val >> 4) & 0x3f		=> VCL NAL unit type
// (reg_val >> 31) & 0x1		=> output_flag
// 16 << ((reg_val >> 10) & 0x3)	=> ctu_size
//

//
// #define W5_RET_DEC_RECOVERY_POINT           (W5_REG_BASE + 0x0168)
// => HEVC recovery point
// reg_val & 0xff => number of signed recovery picture order counts
// (reg_val >> 16) & 0x1 => exact match flag
// (reg_val >> 17) & 0x1 => broken link flag
// (reg_val >> 18) & 0x1 => exist flag
//

//
// #define W5_RET_DEC_REALLOC_INDEX            (W5_REG_BASE + 0x0178)
// => display picture index in decoded picture buffer
// reg_val & 0xf => display picture index for FBC buffer (by reordering)
//

//
// #define W5_RET_DEC_ERR_CTB_NUM              (W5_REG_BASE + 0x0180)
// => Number of error CTUs
// reg_val >> 16	=> erroneous CTUs in bitstream
// reg_val & 0xffff	=> total CTUs in bitstream
//
// #define W5_RET_DEC_PIC_PARAM                (W5_REG_BASE + 0x01A0)
// => Bitstream sequence/picture parameter information (AV1 only)
// reg_val & 0x1 => intrabc tool enable
// (reg_val >> 1) & 0x1 => screen content tools enable
//

//
// #define W5_RET_DEC_SEEK_START_TICK          (W5_REG_BASE + 0x01BC)
// #define W5_RET_DEC_SEEK_END_TICK            (W5_REG_BASE + 0x01C0)
// => Start and end ticks for seeking slices of the picture
// #define W5_RET_DEC_PARSING_START_TICK       (W5_REG_BASE + 0x01C4)
// #define W5_RET_DEC_PARSING_END_TICK         (W5_REG_BASE + 0x01C8)
// => Start and end ticks for parsing slices of the picture
// #define W5_RET_DEC_DECODING_START_TICK      (W5_REG_BASE + 0x01CC)
// => Start tick for decoding slices of the picture
//

//
// DECODER - FLUSH_INSTANCE
//

//
// DECODER - QUERY : UPDATE_DISP_FLAG
//

//
// DECODER - QUERY : SET_BS_RD_PTR
//

//
// DECODER - QUERY : GET_BS_RD_PTR
//

//
// QUERY : GET_DEBUG_INFO
//

//
// GDI register for debugging
//
pub const W5_GDI_BASE: c_uint = 0x8800;

pub const W5_BACKBONE_BASE_VCPU: c_uint = 0xFE00;

pub const W5_BACKBONE_BASE_VCORE0: c_uint = 0x8E00;

pub const W5_BACKBONE_BASE_VCORE1: c_uint = 0x9E00  /* for dual-core product */;

pub const W5_COMBINED_BACKBONE_BASE: c_uint = 0xFE00;

//
// for  ENCODER
//

//
// ENCODER - CREATE_INSTANCE
//
// 0x114 ~ 0x124 : defined above (CREATE_INSTANCE COMMON)

//
// ENCODER - SET_FB
//

//
// ENCODER - ENC_SET_PARAM (COMMON & CHANGE_PARAM)
//

//
// ENCODER - ENC_SET_PARAM (CUSTOM_GOP)
//

//
// ENCODER - ENC_PIC
//

//
// ENCODER - QUERY (GET_RESULT)
//

//
// #define W5_RET_ENC_PIC_POC                      (W5_REG_BASE + 0x128)
// => picture order count value of current encoded picture
//

//
// #define W5_RET_ENC_PIC_SLICE_NUM                (W5_REG_BASE + 0x130)
// reg_val & 0xffff = total independent slice segment number (16 bits)
// (reg_val >> 16) & 0xffff = total dependent slice segment number (16 bits)
//
// #define W5_RET_ENC_PIC_SKIP                     (W5_REG_BASE + 0x134)
// reg_val & 0xfe = picture skip flag (7 bits)
//
// #define W5_RET_ENC_PIC_NUM_INTRA                (W5_REG_BASE + 0x138)
// => number of intra blocks in 8x8 (32 bits)
//
// #define W5_RET_ENC_PIC_NUM_MERGE                (W5_REG_BASE + 0x13C)
// => number of merge blocks in 8x8 (32 bits)
//
// #define W5_RET_ENC_PIC_NUM_SKIP                 (W5_REG_BASE + 0x144)
// => number of skip blocks in 8x8 (32 bits)
//
// #define W5_RET_ENC_PIC_AVG_CTU_QP               (W5_REG_BASE + 0x148)
// => Average CTU QP value (32 bits)
//

//
// #define W5_RET_ENC_GOP_PIC_IDX                  (W5_REG_BASE + 0x150)
// => picture index in group of pictures
//

//
// #define W5_RET_ENC_PIC_NUM                      (W5_REG_BASE + 0x158)
// => encoded picture number
//

//
// Only for H264:
// #define W5_RET_ENC_PIC_DIST_LOW                 (W5_REG_BASE + 0x164)
// => lower 32 bits of the sum of squared difference between source Y picture
// and reconstructed Y picture
// #define W5_RET_ENC_PIC_DIST_HIGH                (W5_REG_BASE + 0x168)
// => upper 32 bits of the sum of squared difference between source Y picture
// and reconstructed Y picture
//

//
// #define W5_RET_ENC_PREPARE_START_TICK           (W5_REG_BASE + 0x1BC)
// #define W5_RET_ENC_PREPARE_END_TICK             (W5_REG_BASE + 0x1C0)
// => Start and end ticks for preparing slices of the picture
// #define W5_RET_ENC_PROCESSING_START_TICK        (W5_REG_BASE + 0x1C4)
// #define W5_RET_ENC_PROCESSING_END_TICK          (W5_REG_BASE + 0x1C8)
// => Start and end ticks for processing slices of the picture
// #define W5_RET_ENC_ENCODING_START_TICK          (W5_REG_BASE + 0x1CC)
// => Start tick for encoding slices of the picture
//

//
// ENCODER - QUERY (GET_BS_WR_PTR)
//

//
// ENCODER - QUERY (GET_BW_REPORT)
//

//
// ENCODER - QUERY (GET_SRC_FLAG)
//

