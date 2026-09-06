//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8188-vdo0.c
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Garmin Chang <garmin.chang@mediatek.com>
//

    static const struct mtk_gate_regs vdo0_0_cg_regs = {
    .set_ofs = 0x104,
    .clr_ofs = 0x108,
    .sta_ofs = 0x100,
    };
    static const struct mtk_gate_regs vdo0_1_cg_regs = {
    .set_ofs = 0x114,
    .clr_ofs = 0x118,
    .sta_ofs = 0x110,
    };
    static const struct mtk_gate_regs vdo0_2_cg_regs = {
    .set_ofs = 0x124,
    .clr_ofs = 0x128,
    .sta_ofs = 0x120,
    };

    GATE_MTK(_id, _name, _parent, &vdo0_0_cg_regs, _shift, &mtk_clk_gate_ops_setclr)

    GATE_MTK(_id, _name, _parent, &vdo0_1_cg_regs, _shift, &mtk_clk_gate_ops_setclr)

    GATE_MTK(_id, _name, _parent, &vdo0_2_cg_regs, _shift, &mtk_clk_gate_ops_setclr)

    GATE_MTK_FLAGS(_id, _name, _parent, &vdo0_2_cg_regs, _shift,	\
    &mtk_clk_gate_ops_setclr, _flags)
    static const struct mtk_gate vdo0_clks[] = {
// VDO0_0
    GATE_VDO0_0(CLK_VDO0_DISP_OVL0, "vdo0_disp_ovl0", "top_vpp", 0),
    GATE_VDO0_0(CLK_VDO0_FAKE_ENG0, "vdo0_fake_eng0", "top_vpp", 2),
    GATE_VDO0_0(CLK_VDO0_DISP_CCORR0, "vdo0_disp_ccorr0", "top_vpp", 4),
    GATE_VDO0_0(CLK_VDO0_DISP_MUTEX0, "vdo0_disp_mutex0", "top_vpp", 6),
    GATE_VDO0_0(CLK_VDO0_DISP_GAMMA0, "vdo0_disp_gamma0", "top_vpp", 8),
    GATE_VDO0_0(CLK_VDO0_DISP_DITHER0, "vdo0_disp_dither0", "top_vpp", 10),
    GATE_VDO0_0(CLK_VDO0_DISP_WDMA0, "vdo0_disp_wdma0", "top_vpp", 17),
    GATE_VDO0_0(CLK_VDO0_DISP_RDMA0, "vdo0_disp_rdma0", "top_vpp", 19),
    GATE_VDO0_0(CLK_VDO0_DSI0, "vdo0_dsi0", "top_vpp", 21),
    GATE_VDO0_0(CLK_VDO0_DSI1, "vdo0_dsi1", "top_vpp", 22),
    GATE_VDO0_0(CLK_VDO0_DSC_WRAP0, "vdo0_dsc_wrap0", "top_vpp", 23),
    GATE_VDO0_0(CLK_VDO0_VPP_MERGE0, "vdo0_vpp_merge0", "top_vpp", 24),
    GATE_VDO0_0(CLK_VDO0_DP_INTF0, "vdo0_dp_intf0", "top_vpp", 25),
    GATE_VDO0_0(CLK_VDO0_DISP_AAL0, "vdo0_disp_aal0", "top_vpp", 26),
    GATE_VDO0_0(CLK_VDO0_INLINEROT0, "vdo0_inlinerot0", "top_vpp", 27),
    GATE_VDO0_0(CLK_VDO0_APB_BUS, "vdo0_apb_bus", "top_vpp", 28),
    GATE_VDO0_0(CLK_VDO0_DISP_COLOR0, "vdo0_disp_color0", "top_vpp", 29),
    GATE_VDO0_0(CLK_VDO0_MDP_WROT0, "vdo0_mdp_wrot0", "top_vpp", 30),
    GATE_VDO0_0(CLK_VDO0_DISP_RSZ0, "vdo0_disp_rsz0", "top_vpp", 31),
// VDO0_1
    GATE_VDO0_1(CLK_VDO0_DISP_POSTMASK0, "vdo0_disp_postmask0", "top_vpp", 0),
    GATE_VDO0_1(CLK_VDO0_FAKE_ENG1, "vdo0_fake_eng1", "top_vpp", 1),
    GATE_VDO0_1(CLK_VDO0_DL_ASYNC2, "vdo0_dl_async2", "top_vpp", 5),
    GATE_VDO0_1(CLK_VDO0_DL_RELAY3, "vdo0_dl_relay3", "top_vpp", 6),
    GATE_VDO0_1(CLK_VDO0_DL_RELAY4, "vdo0_dl_relay4", "top_vpp", 7),
    GATE_VDO0_1(CLK_VDO0_SMI_GALS, "vdo0_smi_gals", "top_vpp", 10),
    GATE_VDO0_1(CLK_VDO0_SMI_COMMON, "vdo0_smi_common", "top_vpp", 11),
    GATE_VDO0_1(CLK_VDO0_SMI_EMI, "vdo0_smi_emi", "top_vpp", 12),
    GATE_VDO0_1(CLK_VDO0_SMI_IOMMU, "vdo0_smi_iommu", "top_vpp", 13),
    GATE_VDO0_1(CLK_VDO0_SMI_LARB, "vdo0_smi_larb", "top_vpp", 14),
    GATE_VDO0_1(CLK_VDO0_SMI_RSI, "vdo0_smi_rsi", "top_vpp", 15),
// VDO0_2
    GATE_VDO0_2(CLK_VDO0_DSI0_DSI, "vdo0_dsi0_dsi", "top_dsi_occ", 0),
    GATE_VDO0_2(CLK_VDO0_DSI1_DSI, "vdo0_dsi1_dsi", "top_dsi_occ", 8),
    GATE_VDO0_2_FLAGS(CLK_VDO0_DP_INTF0_DP_INTF, "vdo0_dp_intf0_dp_intf",
    "top_edp", 16, CLK_SET_RATE_PARENT),
    };
    static const struct mtk_clk_desc vdo0_desc = {
    .clks = vdo0_clks,
    .num_clks = ARRAY_SIZE(vdo0_clks),
    };
    static const struct platform_device_id clk_mt8188_vdo0_id_table[] = {
    { .name = "clk-mt8188-vdo0", .driver_data = (kernel_ulong_t)&vdo0_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, clk_mt8188_vdo0_id_table);
    static struct platform_driver clk_mt8188_vdo0_drv = {
    .probe = mtk_clk_pdev_probe,
    .remove = mtk_clk_pdev_remove,
    .driver = {
    .name = "clk-mt8188-vdo0",
    },
    .id_table = clk_mt8188_vdo0_id_table,
    };
    module_platform_driver(clk_mt8188_vdo0_drv);
    MODULE_DESCRIPTION("MediaTek MT8188 Video Output 0 clocks driver");
    MODULE_LICENSE("GPL");
