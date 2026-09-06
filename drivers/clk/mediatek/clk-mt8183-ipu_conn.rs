//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8183-ipu_conn.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

    static const struct mtk_gate_regs ipu_conn_cg_regs = {
    .set_ofs = 0x4,
    .clr_ofs = 0x8,
    .sta_ofs = 0x0,
    };
    static const struct mtk_gate_regs ipu_conn_apb_cg_regs = {
    .set_ofs = 0x10,
    .clr_ofs = 0x10,
    .sta_ofs = 0x10,
    };
    static const struct mtk_gate_regs ipu_conn_axi_cg_regs = {
    .set_ofs = 0x18,
    .clr_ofs = 0x18,
    .sta_ofs = 0x18,
    };
    static const struct mtk_gate_regs ipu_conn_axi1_cg_regs = {
    .set_ofs = 0x1c,
    .clr_ofs = 0x1c,
    .sta_ofs = 0x1c,
    };
    static const struct mtk_gate_regs ipu_conn_axi2_cg_regs = {
    .set_ofs = 0x20,
    .clr_ofs = 0x20,
    .sta_ofs = 0x20,
    };

    GATE_MTK(_id, _name, _parent, &ipu_conn_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr)

    GATE_MTK(_id, _name, _parent, &ipu_conn_apb_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr)

    GATE_MTK(_id, _name, _parent, &ipu_conn_axi_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr_inv)

    GATE_MTK(_id, _name, _parent, &ipu_conn_axi1_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr_inv)

    GATE_MTK(_id, _name, _parent, &ipu_conn_axi2_cg_regs, _shift,	\
    &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate ipu_conn_clks[] = {
    GATE_IPU_CONN(CLK_IPU_CONN_IPU,
    "ipu_conn_ipu", "dsp_sel", 0),
    GATE_IPU_CONN(CLK_IPU_CONN_AHB,
    "ipu_conn_ahb", "dsp_sel", 1),
    GATE_IPU_CONN(CLK_IPU_CONN_AXI,
    "ipu_conn_axi", "dsp_sel", 2),
    GATE_IPU_CONN(CLK_IPU_CONN_ISP,
    "ipu_conn_isp", "dsp_sel", 3),
    GATE_IPU_CONN(CLK_IPU_CONN_CAM_ADL,
    "ipu_conn_cam_adl", "dsp_sel", 4),
    GATE_IPU_CONN(CLK_IPU_CONN_IMG_ADL,
    "ipu_conn_img_adl", "dsp_sel", 5),
    GATE_IPU_CONN_APB(CLK_IPU_CONN_DAP_RX,
    "ipu_conn_dap_rx", "dsp1_sel", 0),
    GATE_IPU_CONN_APB(CLK_IPU_CONN_APB2AXI,
    "ipu_conn_apb2axi", "dsp1_sel", 3),
    GATE_IPU_CONN_APB(CLK_IPU_CONN_APB2AHB,
    "ipu_conn_apb2ahb", "dsp1_sel", 20),
    GATE_IPU_CONN_AXI_I(CLK_IPU_CONN_IPU_CAB1TO2,
    "ipu_conn_ipu_cab1to2", "dsp1_sel", 6),
    GATE_IPU_CONN_AXI_I(CLK_IPU_CONN_IPU1_CAB1TO2,
    "ipu_conn_ipu1_cab1to2", "dsp1_sel", 13),
    GATE_IPU_CONN_AXI_I(CLK_IPU_CONN_IPU2_CAB1TO2,
    "ipu_conn_ipu2_cab1to2", "dsp1_sel", 20),
    GATE_IPU_CONN_AXI1_I(CLK_IPU_CONN_CAB3TO3,
    "ipu_conn_cab3to3", "dsp1_sel", 0),
    GATE_IPU_CONN_AXI2_I(CLK_IPU_CONN_CAB2TO1,
    "ipu_conn_cab2to1", "dsp1_sel", 14),
    GATE_IPU_CONN_AXI2_I(CLK_IPU_CONN_CAB3TO1_SLICE,
    "ipu_conn_cab3to1_slice", "dsp1_sel", 17),
    };
    static const struct mtk_clk_desc ipu_conn_desc = {
    .clks = ipu_conn_clks,
    .num_clks = ARRAY_SIZE(ipu_conn_clks),
    };
    static const struct of_device_id of_match_clk_mt8183_ipu_conn[] = {
    {
    .compatible = "mediatek,mt8183-ipu_conn",
    .data = &ipu_conn_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_conn);
    static struct platform_driver clk_mt8183_ipu_conn_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8183-ipu_conn",
    .of_match_table = of_match_clk_mt8183_ipu_conn,
    },
    };
    module_platform_driver(clk_mt8183_ipu_conn_drv);
    MODULE_DESCRIPTION("MediaTek MT8183 Image Processing Unit Bus clocks driver");
    MODULE_LICENSE("GPL");
