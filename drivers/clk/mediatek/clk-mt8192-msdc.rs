//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8192-msdc.c
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

    static const struct mtk_gate_regs msdc_top_cg_regs = {
    .set_ofs = 0x0,
    .clr_ofs = 0x0,
    .sta_ofs = 0x0,
    };

    GATE_MTK(_id, _name, _parent, &msdc_top_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate msdc_top_clks[] = {
    GATE_MSDC_TOP(CLK_MSDC_TOP_AES_0P, "msdc_top_aes_0p", "aes_msdcfde_sel", 0),
    GATE_MSDC_TOP(CLK_MSDC_TOP_SRC_0P, "msdc_top_src_0p", "infra_msdc0_src", 1),
    GATE_MSDC_TOP(CLK_MSDC_TOP_SRC_1P, "msdc_top_src_1p", "infra_msdc1_src", 2),
    GATE_MSDC_TOP(CLK_MSDC_TOP_SRC_2P, "msdc_top_src_2p", "infra_msdc2_src", 3),
    GATE_MSDC_TOP(CLK_MSDC_TOP_P_MSDC0, "msdc_top_p_msdc0", "axi_sel", 4),
    GATE_MSDC_TOP(CLK_MSDC_TOP_P_MSDC1, "msdc_top_p_msdc1", "axi_sel", 5),
    GATE_MSDC_TOP(CLK_MSDC_TOP_P_MSDC2, "msdc_top_p_msdc2", "axi_sel", 6),
    GATE_MSDC_TOP(CLK_MSDC_TOP_P_CFG, "msdc_top_p_cfg", "axi_sel", 7),
    GATE_MSDC_TOP(CLK_MSDC_TOP_AXI, "msdc_top_axi", "axi_sel", 8),
    GATE_MSDC_TOP(CLK_MSDC_TOP_H_MST_0P, "msdc_top_h_mst_0p", "infra_msdc0", 9),
    GATE_MSDC_TOP(CLK_MSDC_TOP_H_MST_1P, "msdc_top_h_mst_1p", "infra_msdc1", 10),
    GATE_MSDC_TOP(CLK_MSDC_TOP_H_MST_2P, "msdc_top_h_mst_2p", "infra_msdc2", 11),
    GATE_MSDC_TOP(CLK_MSDC_TOP_MEM_OFF_DLY_26M, "msdc_top_mem_off_dly_26m", "clk26m", 12),
    GATE_MSDC_TOP(CLK_MSDC_TOP_32K, "msdc_top_32k", "clk32k", 13),
    GATE_MSDC_TOP(CLK_MSDC_TOP_AHB2AXI_BRG_AXI, "msdc_top_ahb2axi_brg_axi", "axi_sel", 14),
    };
    static const struct mtk_clk_desc msdc_top_desc = {
    .clks = msdc_top_clks,
    .num_clks = ARRAY_SIZE(msdc_top_clks),
    };
    static const struct of_device_id of_match_clk_mt8192_msdc[] = {
    {
    .compatible = "mediatek,mt8192-msdc_top",
    .data = &msdc_top_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_msdc);
    static struct platform_driver clk_mt8192_msdc_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8192-msdc",
    .of_match_table = of_match_clk_mt8192_msdc,
    },
    };
    module_platform_driver(clk_mt8192_msdc_drv);
    MODULE_DESCRIPTION("MediaTek MT8192 MMC/SD Controller clocks driver");
    MODULE_LICENSE("GPL");
