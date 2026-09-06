//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt2701-img.c
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Shunli Wang <shunli.wang@mediatek.com>
//

    static const struct mtk_gate_regs img_cg_regs = {
    .set_ofs = 0x0004,
    .clr_ofs = 0x0008,
    .sta_ofs = 0x0000,
    };

    GATE_MTK(_id, _name, _parent, &img_cg_regs, _shift, &mtk_clk_gate_ops_setclr)
    static const struct mtk_gate img_clks[] = {
    GATE_DUMMY(CLK_DUMMY, "img_dummy"),
    GATE_IMG(CLK_IMG_SMI_COMM, "img_smi_comm", "mm_sel", 0),
    GATE_IMG(CLK_IMG_RESZ, "img_resz", "mm_sel", 1),
    GATE_IMG(CLK_IMG_JPGDEC_SMI, "img_jpgdec_smi", "mm_sel", 5),
    GATE_IMG(CLK_IMG_JPGDEC, "img_jpgdec", "mm_sel", 6),
    GATE_IMG(CLK_IMG_VENC_LT, "img_venc_lt", "mm_sel", 8),
    GATE_IMG(CLK_IMG_VENC, "img_venc", "mm_sel", 9),
    };
    static const struct mtk_clk_desc img_desc = {
    .clks = img_clks,
    .num_clks = ARRAY_SIZE(img_clks),
    };
    static const struct of_device_id of_match_clk_mt2701_img[] = {
    {
    .compatible = "mediatek,mt2701-imgsys",
    .data = &img_desc,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_img);
    static struct platform_driver clk_mt2701_img_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt2701-img",
    .of_match_table = of_match_clk_mt2701_img,
    },
    };
    module_platform_driver(clk_mt2701_img_drv);
    MODULE_DESCRIPTION("MediaTek MT2701 imgsys clocks driver");
    MODULE_LICENSE("GPL");
