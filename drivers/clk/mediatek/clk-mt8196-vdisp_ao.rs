//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-vdisp_ao.c
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
// Copyright (c) 2025 MediaTek Inc.
// Guangjie Song <guangjie.song@mediatek.com>
// Copyright (c) 2025 Collabora Ltd.
// Laura Nao <laura.nao@collabora.com>
//

    static const struct mtk_gate_regs mm_v_cg_regs = {
    .set_ofs = 0x104,
    .clr_ofs = 0x108,
    .sta_ofs = 0x100,
    };
    static const struct mtk_gate_regs mm_v_hwv_regs = {
    .set_ofs = 0x0030,
    .clr_ofs = 0x0034,
    .sta_ofs = 0x2c18,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &mm_v_cg_regs,			\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_setclr,	\
    .flags = CLK_OPS_PARENT_ENABLE |	\
    CLK_IS_CRITICAL,		\
    }

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &mm_v_cg_regs,			\
    .hwv_regs = &mm_v_hwv_regs,		\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_hwv_ops_setclr,	\
    .flags = CLK_OPS_PARENT_ENABLE,		\
    }
    static const struct mtk_gate mm_v_clks[] = {
    GATE_HWV_MM_V(CLK_MM_V_DISP_VDISP_AO_CONFIG, "mm_v_disp_vdisp_ao_config", "disp", 0),
    GATE_HWV_MM_V(CLK_MM_V_DISP_DPC, "mm_v_disp_dpc", "disp", 16),
    GATE_MM_AO_V(CLK_MM_V_SMI_SUB_SOMM0, "mm_v_smi_sub_somm0", "disp", 2),
    };
    static const struct mtk_clk_desc mm_v_mcd = {
    .clks = mm_v_clks,
    .num_clks = ARRAY_SIZE(mm_v_clks),
    };
    static const struct of_device_id of_match_clk_mt8196_vdisp_ao[] = {
    { .compatible = "mediatek,mt8196-vdisp-ao", .data = &mm_v_mcd },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_vdisp_ao);
    static struct platform_driver clk_mt8196_vdisp_ao_drv = {
    .probe = mtk_clk_pdev_probe,
    .remove = mtk_clk_pdev_remove,
    .driver = {
    .name = "clk-mt8196-vdisp-ao",
    .of_match_table = of_match_clk_mt8196_vdisp_ao,
    },
    };
    module_platform_driver(clk_mt8196_vdisp_ao_drv);
    MODULE_DESCRIPTION("MediaTek MT8196 vdisp_ao clocks driver");
    MODULE_LICENSE("GPL");
