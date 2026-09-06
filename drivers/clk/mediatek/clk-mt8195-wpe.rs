//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8195-wpe.c
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

    static const struct mtk_gate_regs wpe_cg_regs = {
    .set_ofs = 0x0,
    .clr_ofs = 0x0,
    .sta_ofs = 0x0,
    };
    static const struct mtk_gate_regs wpe_vpp0_cg_regs = {
    .set_ofs = 0x58,
    .clr_ofs = 0x58,
    .sta_ofs = 0x58,
    };
    static const struct mtk_gate_regs wpe_vpp1_cg_regs = {
    .set_ofs = 0x5c,
    .clr_ofs = 0x5c,
    .sta_ofs = 0x5c,
    };

    GATE_MTK(_id, _name, _parent, &wpe_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)

    GATE_MTK(_id, _name, _parent, &wpe_vpp0_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)

    GATE_MTK(_id, _name, _parent, &wpe_vpp1_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate wpe_clks[] = {
    GATE_WPE(CLK_WPE_VPP0, "wpe_vpp0", "top_wpe_vpp", 16),
    GATE_WPE(CLK_WPE_VPP1, "wpe_vpp1", "top_wpe_vpp", 17),
    GATE_WPE(CLK_WPE_SMI_LARB7, "wpe_smi_larb7", "top_wpe_vpp", 18),
    GATE_WPE(CLK_WPE_SMI_LARB8, "wpe_smi_larb8", "top_wpe_vpp", 19),
    GATE_WPE(CLK_WPE_EVENT_TX, "wpe_event_tx", "top_wpe_vpp", 20),
    GATE_WPE(CLK_WPE_SMI_LARB7_P, "wpe_smi_larb7_p", "top_wpe_vpp", 24),
    GATE_WPE(CLK_WPE_SMI_LARB8_P, "wpe_smi_larb8_p", "top_wpe_vpp", 25),
    };
    static const struct mtk_gate wpe_vpp0_clks[] = {
// WPE_VPP0
    GATE_WPE_VPP0(CLK_WPE_VPP0_VGEN, "wpe_vpp0_vgen", "top_img", 0),
    GATE_WPE_VPP0(CLK_WPE_VPP0_EXT, "wpe_vpp0_ext", "top_img", 1),
    GATE_WPE_VPP0(CLK_WPE_VPP0_VFC, "wpe_vpp0_vfc", "top_img", 2),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH0_TOP, "wpe_vpp0_cach0_top", "top_img", 3),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH0_DMA, "wpe_vpp0_cach0_dma", "top_img", 4),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH1_TOP, "wpe_vpp0_cach1_top", "top_img", 5),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH1_DMA, "wpe_vpp0_cach1_dma", "top_img", 6),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH2_TOP, "wpe_vpp0_cach2_top", "top_img", 7),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH2_DMA, "wpe_vpp0_cach2_dma", "top_img", 8),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH3_TOP, "wpe_vpp0_cach3_top", "top_img", 9),
    GATE_WPE_VPP0(CLK_WPE_VPP0_CACH3_DMA, "wpe_vpp0_cach3_dma", "top_img", 10),
    GATE_WPE_VPP0(CLK_WPE_VPP0_PSP, "wpe_vpp0_psp", "top_img", 11),
    GATE_WPE_VPP0(CLK_WPE_VPP0_PSP2, "wpe_vpp0_psp2", "top_img", 12),
    GATE_WPE_VPP0(CLK_WPE_VPP0_SYNC, "wpe_vpp0_sync", "top_img", 13),
    GATE_WPE_VPP0(CLK_WPE_VPP0_C24, "wpe_vpp0_c24", "top_img", 14),
    GATE_WPE_VPP0(CLK_WPE_VPP0_MDP_CROP, "wpe_vpp0_mdp_crop", "top_img", 15),
    GATE_WPE_VPP0(CLK_WPE_VPP0_ISP_CROP, "wpe_vpp0_isp_crop", "top_img", 16),
    GATE_WPE_VPP0(CLK_WPE_VPP0_TOP, "wpe_vpp0_top", "top_img", 17),
// WPE_VPP1
    GATE_WPE_VPP1(CLK_WPE_VPP0_VECI, "wpe_vpp0_veci", "top_img", 0),
    GATE_WPE_VPP1(CLK_WPE_VPP0_VEC2I, "wpe_vpp0_vec2i", "top_img", 1),
    GATE_WPE_VPP1(CLK_WPE_VPP0_VEC3I, "wpe_vpp0_vec3i", "top_img", 2),
    GATE_WPE_VPP1(CLK_WPE_VPP0_WPEO, "wpe_vpp0_wpeo", "top_img", 3),
    GATE_WPE_VPP1(CLK_WPE_VPP0_MSKO, "wpe_vpp0_msko", "top_img", 4),
    };
    static const struct mtk_gate wpe_vpp1_clks[] = {
// WPE_VPP0
    GATE_WPE_VPP0(CLK_WPE_VPP1_VGEN, "wpe_vpp1_vgen", "top_img", 0),
    GATE_WPE_VPP0(CLK_WPE_VPP1_EXT, "wpe_vpp1_ext", "top_img", 1),
    GATE_WPE_VPP0(CLK_WPE_VPP1_VFC, "wpe_vpp1_vfc", "top_img", 2),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH0_TOP, "wpe_vpp1_cach0_top", "top_img", 3),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH0_DMA, "wpe_vpp1_cach0_dma", "top_img", 4),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH1_TOP, "wpe_vpp1_cach1_top", "top_img", 5),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH1_DMA, "wpe_vpp1_cach1_dma", "top_img", 6),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH2_TOP, "wpe_vpp1_cach2_top", "top_img", 7),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH2_DMA, "wpe_vpp1_cach2_dma", "top_img", 8),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH3_TOP, "wpe_vpp1_cach3_top", "top_img", 9),
    GATE_WPE_VPP0(CLK_WPE_VPP1_CACH3_DMA, "wpe_vpp1_cach3_dma", "top_img", 10),
    GATE_WPE_VPP0(CLK_WPE_VPP1_PSP, "wpe_vpp1_psp", "top_img", 11),
    GATE_WPE_VPP0(CLK_WPE_VPP1_PSP2, "wpe_vpp1_psp2", "top_img", 12),
    GATE_WPE_VPP0(CLK_WPE_VPP1_SYNC, "wpe_vpp1_sync", "top_img", 13),
    GATE_WPE_VPP0(CLK_WPE_VPP1_C24, "wpe_vpp1_c24", "top_img", 14),
    GATE_WPE_VPP0(CLK_WPE_VPP1_MDP_CROP, "wpe_vpp1_mdp_crop", "top_img", 15),
    GATE_WPE_VPP0(CLK_WPE_VPP1_ISP_CROP, "wpe_vpp1_isp_crop", "top_img", 16),
    GATE_WPE_VPP0(CLK_WPE_VPP1_TOP, "wpe_vpp1_top", "top_img", 17),
// WPE_VPP1
    GATE_WPE_VPP1(CLK_WPE_VPP1_VECI, "wpe_vpp1_veci", "top_img", 0),
    GATE_WPE_VPP1(CLK_WPE_VPP1_VEC2I, "wpe_vpp1_vec2i", "top_img", 1),
    GATE_WPE_VPP1(CLK_WPE_VPP1_VEC3I, "wpe_vpp1_vec3i", "top_img", 2),
    GATE_WPE_VPP1(CLK_WPE_VPP1_WPEO, "wpe_vpp1_wpeo", "top_img", 3),
    GATE_WPE_VPP1(CLK_WPE_VPP1_MSKO, "wpe_vpp1_msko", "top_img", 4),
    };
    static const struct mtk_clk_desc wpe_desc = {
    .clks = wpe_clks,
    .num_clks = ARRAY_SIZE(wpe_clks),
    };
    static const struct mtk_clk_desc wpe_vpp0_desc = {
    .clks = wpe_vpp0_clks,
    .num_clks = ARRAY_SIZE(wpe_vpp0_clks),
    };
    static const struct mtk_clk_desc wpe_vpp1_desc = {
    .clks = wpe_vpp1_clks,
    .num_clks = ARRAY_SIZE(wpe_vpp1_clks),
    };
    static const struct of_device_id of_match_clk_mt8195_wpe[] = {
    {
    .compatible = "mediatek,mt8195-wpesys",
    .data = &wpe_desc,
    }, {
    .compatible = "mediatek,mt8195-wpesys_vpp0",
    .data = &wpe_vpp0_desc,
    }, {
    .compatible = "mediatek,mt8195-wpesys_vpp1",
    .data = &wpe_vpp1_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_wpe);
    static struct platform_driver clk_mt8195_wpe_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8195-wpe",
    .of_match_table = of_match_clk_mt8195_wpe,
    },
    };
    module_platform_driver(clk_mt8195_wpe_drv);
    MODULE_DESCRIPTION("MediaTek MT8195 Warp Engine clocks driver");
    MODULE_LICENSE("GPL");
