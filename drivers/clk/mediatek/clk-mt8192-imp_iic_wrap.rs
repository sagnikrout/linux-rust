//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8192-imp_iic_wrap.c
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

    static const struct mtk_gate_regs imp_iic_wrap_cg_regs = {
    .set_ofs = 0xe08,
    .clr_ofs = 0xe04,
    .sta_ofs = 0xe00,
    };

    GATE_MTK_FLAGS(_id, _name, _parent, &imp_iic_wrap_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr, CLK_OPS_PARENT_ENABLE)
    static const struct mtk_gate imp_iic_wrap_c_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_C_I2C10, "imp_iic_wrap_c_i2c10", "infra_i2c0", 0),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_C_I2C11, "imp_iic_wrap_c_i2c11", "infra_i2c0", 1),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_C_I2C12, "imp_iic_wrap_c_i2c12", "infra_i2c0", 2),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_C_I2C13, "imp_iic_wrap_c_i2c13", "infra_i2c0", 3),
    };
    static const struct mtk_gate imp_iic_wrap_e_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_E_I2C3, "imp_iic_wrap_e_i2c3", "infra_i2c0", 0),
    };
    static const struct mtk_gate imp_iic_wrap_n_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_N_I2C0, "imp_iic_wrap_n_i2c0", "infra_i2c0", 0),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_N_I2C6, "imp_iic_wrap_n_i2c6", "infra_i2c0", 1),
    };
    static const struct mtk_gate imp_iic_wrap_s_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_S_I2C7, "imp_iic_wrap_s_i2c7", "infra_i2c0", 0),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_S_I2C8, "imp_iic_wrap_s_i2c8", "infra_i2c0", 1),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_S_I2C9, "imp_iic_wrap_s_i2c9", "infra_i2c0", 2),
    };
    static const struct mtk_gate imp_iic_wrap_w_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_W_I2C5, "imp_iic_wrap_w_i2c5", "infra_i2c0", 0),
    };
    static const struct mtk_gate imp_iic_wrap_ws_clks[] = {
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_WS_I2C1, "imp_iic_wrap_ws_i2c1", "infra_i2c0", 0),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_WS_I2C2, "imp_iic_wrap_ws_i2c2", "infra_i2c0", 1),
    GATE_IMP_IIC_WRAP(CLK_IMP_IIC_WRAP_WS_I2C4, "imp_iic_wrap_ws_i2c4", "infra_i2c0", 2),
    };
    static const struct mtk_clk_desc imp_iic_wrap_c_desc = {
    .clks = imp_iic_wrap_c_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_c_clks),
    };
    static const struct mtk_clk_desc imp_iic_wrap_e_desc = {
    .clks = imp_iic_wrap_e_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_e_clks),
    };
    static const struct mtk_clk_desc imp_iic_wrap_n_desc = {
    .clks = imp_iic_wrap_n_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_n_clks),
    };
    static const struct mtk_clk_desc imp_iic_wrap_s_desc = {
    .clks = imp_iic_wrap_s_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_s_clks),
    };
    static const struct mtk_clk_desc imp_iic_wrap_w_desc = {
    .clks = imp_iic_wrap_w_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_w_clks),
    };
    static const struct mtk_clk_desc imp_iic_wrap_ws_desc = {
    .clks = imp_iic_wrap_ws_clks,
    .num_clks = ARRAY_SIZE(imp_iic_wrap_ws_clks),
    };
    static const struct of_device_id of_match_clk_mt8192_imp_iic_wrap[] = {
    {
    .compatible = "mediatek,mt8192-imp_iic_wrap_c",
    .data = &imp_iic_wrap_c_desc,
    }, {
    .compatible = "mediatek,mt8192-imp_iic_wrap_e",
    .data = &imp_iic_wrap_e_desc,
    }, {
    .compatible = "mediatek,mt8192-imp_iic_wrap_n",
    .data = &imp_iic_wrap_n_desc,
    }, {
    .compatible = "mediatek,mt8192-imp_iic_wrap_s",
    .data = &imp_iic_wrap_s_desc,
    }, {
    .compatible = "mediatek,mt8192-imp_iic_wrap_w",
    .data = &imp_iic_wrap_w_desc,
    }, {
    .compatible = "mediatek,mt8192-imp_iic_wrap_ws",
    .data = &imp_iic_wrap_ws_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8192_imp_iic_wrap);
    static struct platform_driver clk_mt8192_imp_iic_wrap_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8192-imp_iic_wrap",
    .of_match_table = of_match_clk_mt8192_imp_iic_wrap,
    },
    };
    module_platform_driver(clk_mt8192_imp_iic_wrap_drv);
    MODULE_DESCRIPTION("MediaTek MT8192 I2C Wrapper clocks driver");
    MODULE_LICENSE("GPL");
