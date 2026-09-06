//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-imp_iic_wrap.c
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

    static const struct mtk_gate_regs imp_cg_regs = {
    .set_ofs = 0xe08,
    .clr_ofs = 0xe04,
    .sta_ofs = 0xe00,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &imp_cg_regs,			\
    .shift = _shift,			\
    .flags = CLK_OPS_PARENT_ENABLE,		\
    .ops = &mtk_clk_gate_ops_setclr,	\
    }
    static const struct mtk_gate impc_clks[] = {
    GATE_IMP(CLK_IMPC_I2C11, "impc_i2c11", "i2c_p", 0),
    GATE_IMP(CLK_IMPC_I2C12, "impc_i2c12", "i2c_p", 1),
    GATE_IMP(CLK_IMPC_I2C13, "impc_i2c13", "i2c_p", 2),
    GATE_IMP(CLK_IMPC_I2C14, "impc_i2c14", "i2c_p", 3),
    };
    static const struct mtk_clk_desc impc_mcd = {
    .clks = impc_clks,
    .num_clks = ARRAY_SIZE(impc_clks),
    };
    static const struct mtk_gate impe_clks[] = {
    GATE_IMP(CLK_IMPE_I2C5, "impe_i2c5", "i2c_east", 0),
    };
    static const struct mtk_clk_desc impe_mcd = {
    .clks = impe_clks,
    .num_clks = ARRAY_SIZE(impe_clks),
    };
    static const struct mtk_gate_regs impn_hwv_regs = {
    .set_ofs = 0x0000,
    .clr_ofs = 0x0004,
    .sta_ofs = 0x2c00,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &imp_cg_regs,			\
    .hwv_regs = &impn_hwv_regs,		\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_hwv_ops_setclr,	\
    .flags = CLK_OPS_PARENT_ENABLE,		\
    }
    static const struct mtk_gate impn_clks[] = {
    GATE_IMP(CLK_IMPN_I2C1, "impn_i2c1", "i2c_north", 0),
    GATE_IMP(CLK_IMPN_I2C2, "impn_i2c2", "i2c_north", 1),
    GATE_IMP(CLK_IMPN_I2C4, "impn_i2c4", "i2c_north", 2),
    GATE_HWV_IMPN(CLK_IMPN_I2C7, "impn_i2c7", "i2c_north", 3),
    GATE_IMP(CLK_IMPN_I2C8, "impn_i2c8", "i2c_north", 4),
    GATE_IMP(CLK_IMPN_I2C9, "impn_i2c9", "i2c_north", 5),
    };
    static const struct mtk_clk_desc impn_mcd = {
    .clks = impn_clks,
    .num_clks = ARRAY_SIZE(impn_clks),
    };
    static const struct mtk_gate impw_clks[] = {
    GATE_IMP(CLK_IMPW_I2C0, "impw_i2c0", "i2c_west", 0),
    GATE_IMP(CLK_IMPW_I2C3, "impw_i2c3", "i2c_west", 1),
    GATE_IMP(CLK_IMPW_I2C6, "impw_i2c6", "i2c_west", 2),
    GATE_IMP(CLK_IMPW_I2C10, "impw_i2c10", "i2c_west", 3),
    };
    static const struct mtk_clk_desc impw_mcd = {
    .clks = impw_clks,
    .num_clks = ARRAY_SIZE(impw_clks),
    };
    static const struct of_device_id of_match_clk_mt8196_imp_iic_wrap[] = {
    { .compatible = "mediatek,mt8196-imp-iic-wrap-c", .data = &impc_mcd },
    { .compatible = "mediatek,mt8196-imp-iic-wrap-e", .data = &impe_mcd },
    { .compatible = "mediatek,mt8196-imp-iic-wrap-n", .data = &impn_mcd },
    { .compatible = "mediatek,mt8196-imp-iic-wrap-w", .data = &impw_mcd },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_imp_iic_wrap);
    static struct platform_driver clk_mt8196_imp_iic_wrap_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8196-imp_iic_wrap",
    .of_match_table = of_match_clk_mt8196_imp_iic_wrap,
    },
    };
    module_platform_driver(clk_mt8196_imp_iic_wrap_drv);
    MODULE_DESCRIPTION("MediaTek MT8196 I2C Wrapper clocks driver");
    MODULE_LICENSE("GPL");
