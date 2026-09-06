//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/iris/iris_platform_vpu2.c
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
// Copyright (c) Qualcomm Innovation Center, Inc. All rights reserved.
//

    static const struct iris_firmware_desc iris_milos_desc = {
    .firmware_data = &iris_hfi_milos_data,
    .get_vpu_buffer_size = iris_vpu_buf_size,
    .fwname = "qcom/vpu/vpu20_p2_gen2_s7.mbn",
    };
    static const struct iris_firmware_desc iris_vpu20_p1_gen1_desc = {
    .firmware_data = &iris_hfi_gen1_data,
    .get_vpu_buffer_size = iris_vpu_buf_size,
    .fwname = "qcom/vpu/vpu20_p1.mbn",
    };
    static const struct iris_firmware_desc iris_vpu20_p1_gen2_s6_desc = {
    .firmware_data = &iris_hfi_gen2_data,
    .get_vpu_buffer_size = iris_vpu33_buf_size,
    .fwname = "qcom/vpu/vpu20_p1_gen2_s6.mbn",
    };
    static const struct iris_firmware_desc iris_vpu20_p4_gen1_desc = {
    .firmware_data = &iris_hfi_gen1_data,
    .get_vpu_buffer_size = iris_vpu_buf_size,
    .fwname = "qcom/vpu/vpu20_p4.mbn",
    };
    static const u32 iris_fmts_vpu2_dec[] = {
    [IRIS_FMT_H264] = V4L2_PIX_FMT_H264,
    [IRIS_FMT_HEVC] = V4L2_PIX_FMT_HEVC,
    [IRIS_FMT_VP9] = V4L2_PIX_FMT_VP9,
    };
    static struct platform_inst_caps platform_inst_cap_vpu2 = {
    .min_frame_width = 128,
    .max_frame_width = 8192,
    .min_frame_height = 128,
    .max_frame_height = 8192,
    .max_mbpf = 138240,
    .mb_cycles_vsp = 25,
    .mb_cycles_vpp = 200,
    .max_frame_rate = MAXIMUM_FPS,
    .max_operating_rate = MAXIMUM_FPS,
    };
    static const struct icc_info iris_icc_info_vpu2[] = {
    { "cpu-cfg",    1000, 1000     },
    { "video-mem",  1000, 15000000 },
    };
    static const char * const iris_clk_reset_table_vpu2[] = { "bus", "core" };
    static const char * const iris_pmdomain_table_vpu2[] = { "venus", "vcodec0" };
    static const struct tz_cp_config tz_cp_config_vpu2[] = {
    {
    .cp_start = 0,
    .cp_size = 0x25800000,
    .cp_nonpixel_start = 0x01000000,
    .cp_nonpixel_size = 0x24800000,
    },
    };
    const struct iris_platform_data milos_data = {
    .firmware_desc_gen1 = &iris_milos_desc,
    .vpu_ops = &iris_vpu2_ops,
    .icc_tbl = iris_icc_info_milos,
    .icc_tbl_size = ARRAY_SIZE(iris_icc_info_milos),
    .bw_tbl_dec = sm8250_bw_table_dec,
    .bw_tbl_dec_size = ARRAY_SIZE(sm8250_bw_table_dec),
    .pmdomain_tbl = iris_pmdomain_table_vpu2,
    .pmdomain_tbl_size = ARRAY_SIZE(iris_pmdomain_table_vpu2),
    .opp_pd_tbl = milos_opp_pd_table,
    .opp_pd_tbl_size = ARRAY_SIZE(milos_opp_pd_table),
    .clk_tbl = sm8250_clk_table,
    .clk_tbl_size = ARRAY_SIZE(sm8250_clk_table),
    .opp_clk_tbl = sm8250_opp_clk_table,
    .clk_rst_tbl = iris_clk_reset_table_vpu2,
    .clk_rst_tbl_size = ARRAY_SIZE(iris_clk_reset_table_vpu2),
    .dma_mask = 0xe0000000 - 1,
    .inst_iris_fmts = iris_fmts_vpu2_dec,
    .inst_iris_fmts_size = ARRAY_SIZE(iris_fmts_vpu2_dec),
    .inst_caps = &platform_inst_cap_milos,
    .tz_cp_config_data = tz_cp_config_vpu2,
    .tz_cp_config_data_size = ARRAY_SIZE(tz_cp_config_vpu2),
    .num_vpp_pipe = 2,
    .max_session_count = 16,
    .max_core_mbpf = ((4096 * 2176) / 256) * 2,
    .max_core_mbps = ((3840 * 2176) / 256) * 30 + ((1920 * 1088) / 256) * 30,
    };
    const struct iris_platform_data sc7280_data = {
    .firmware_desc_gen1 = &iris_vpu20_p1_gen1_desc,
    .firmware_desc_gen2 = &iris_vpu20_p1_gen2_s6_desc,
    .vpu_ops = &iris_vpu2_ops,
    .icc_tbl = iris_icc_info_vpu2,
    .icc_tbl_size = ARRAY_SIZE(iris_icc_info_vpu2),
    .bw_tbl_dec = sc7280_bw_table_dec,
    .bw_tbl_dec_size = ARRAY_SIZE(sc7280_bw_table_dec),
    .pmdomain_tbl = iris_pmdomain_table_vpu2,
    .pmdomain_tbl_size = ARRAY_SIZE(iris_pmdomain_table_vpu2),
    .opp_pd_tbl = sc7280_opp_pd_table,
    .opp_pd_tbl_size = ARRAY_SIZE(sc7280_opp_pd_table),
    .clk_tbl = sc7280_clk_table,
    .clk_tbl_size = ARRAY_SIZE(sc7280_clk_table),
    .opp_clk_tbl = sc7280_opp_clk_table,
// Upper bound of DMA address range
    .dma_mask = 0xe0000000 - 1,
    .inst_iris_fmts = iris_fmts_vpu2_dec,
    .inst_iris_fmts_size = ARRAY_SIZE(iris_fmts_vpu2_dec),
    .inst_caps = &platform_inst_cap_vpu2,
    .tz_cp_config_data = tz_cp_config_vpu2,
    .tz_cp_config_data_size = ARRAY_SIZE(tz_cp_config_vpu2),
    .num_vpp_pipe = 1,
    .no_aon = true,
    .max_session_count = 16,
    .max_core_mbpf = 4096 * 2176 / 256 * 2 + 1920 * 1088 / 256,
// max spec for SC7280 is 4096x2176@60fps
    .max_core_mbps = 4096 * 2176 / 256 * 60,
    };
    const struct iris_platform_data sm8250_data = {
    .firmware_desc_gen1 = &iris_vpu20_p4_gen1_desc,
    .vpu_ops = &iris_vpu2_ops,
    .icc_tbl = iris_icc_info_vpu2,
    .icc_tbl_size = ARRAY_SIZE(iris_icc_info_vpu2),
    .clk_rst_tbl = iris_clk_reset_table_vpu2,
    .clk_rst_tbl_size = ARRAY_SIZE(iris_clk_reset_table_vpu2),
    .bw_tbl_dec = sm8250_bw_table_dec,
    .bw_tbl_dec_size = ARRAY_SIZE(sm8250_bw_table_dec),
    .pmdomain_tbl = iris_pmdomain_table_vpu2,
    .pmdomain_tbl_size = ARRAY_SIZE(iris_pmdomain_table_vpu2),
    .opp_pd_tbl = sm8250_opp_pd_table,
    .opp_pd_tbl_size = ARRAY_SIZE(sm8250_opp_pd_table),
    .clk_tbl = sm8250_clk_table,
    .clk_tbl_size = ARRAY_SIZE(sm8250_clk_table),
    .opp_clk_tbl = sm8250_opp_clk_table,
// Upper bound of DMA address range
    .dma_mask = 0xe0000000 - 1,
    .inst_iris_fmts = iris_fmts_vpu2_dec,
    .inst_iris_fmts_size = ARRAY_SIZE(iris_fmts_vpu2_dec),
    .inst_caps = &platform_inst_cap_vpu2,
    .tz_cp_config_data = tz_cp_config_vpu2,
    .tz_cp_config_data_size = ARRAY_SIZE(tz_cp_config_vpu2),
    .num_vpp_pipe = 4,
    .max_session_count = 16,
    .max_core_mbpf = NUM_MBS_8K,
    .max_core_mbps = ((7680 * 4320) / 256) * 60,
    };
