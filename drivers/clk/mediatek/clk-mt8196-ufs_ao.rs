//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-ufs_ao.c
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

pub const MT8196_UFSAO_RST0_SET_OFFSET: c_uint = 0x48;
pub const MT8196_UFSAO_RST1_SET_OFFSET: c_uint = 0x148;
    static const struct mtk_gate_regs ufsao0_cg_regs = {
    .set_ofs = 0x108,
    .clr_ofs = 0x10c,
    .sta_ofs = 0x104,
    };
    static const struct mtk_gate_regs ufsao1_cg_regs = {
    .set_ofs = 0x8,
    .clr_ofs = 0xc,
    .sta_ofs = 0x4,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &ufsao0_cg_regs,		\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_setclr,	\
    }

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &ufsao1_cg_regs,		\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_setclr,	\
    }
    static const struct mtk_gate ufsao_clks[] = {
// UFSAO0
    GATE_UFSAO0(CLK_UFSAO_UFSHCI_UFS, "ufsao_ufshci_ufs", "ufs", 0),
    GATE_UFSAO0(CLK_UFSAO_UFSHCI_AES, "ufsao_ufshci_aes", "aes_ufsfde", 1),
// UFSAO1
    GATE_UFSAO1(CLK_UFSAO_UNIPRO_TX_SYM, "ufsao_unipro_tx_sym", "clk26m", 0),
    GATE_UFSAO1(CLK_UFSAO_UNIPRO_RX_SYM0, "ufsao_unipro_rx_sym0", "clk26m", 1),
    GATE_UFSAO1(CLK_UFSAO_UNIPRO_RX_SYM1, "ufsao_unipro_rx_sym1", "clk26m", 2),
    GATE_UFSAO1(CLK_UFSAO_UNIPRO_SYS, "ufsao_unipro_sys", "ufs", 3),
    GATE_UFSAO1(CLK_UFSAO_UNIPRO_SAP, "ufsao_unipro_sap", "clk26m", 4),
    GATE_UFSAO1(CLK_UFSAO_PHY_SAP, "ufsao_phy_sap", "clk26m", 8),
    };
    static u16 ufsao_rst_ofs[] = {
    MT8196_UFSAO_RST0_SET_OFFSET,
    MT8196_UFSAO_RST1_SET_OFFSET
    };
    static u16 ufsao_rst_idx_map[] = {
    [MT8196_UFSAO_RST0_UFS_MPHY] = 8,
    [MT8196_UFSAO_RST1_UFS_UNIPRO] = 1 * RST_NR_PER_BANK + 0,
    [MT8196_UFSAO_RST1_UFS_CRYPTO] = 1 * RST_NR_PER_BANK + 1,
    [MT8196_UFSAO_RST1_UFSHCI] = 1 * RST_NR_PER_BANK + 2,
    };
    static const struct mtk_clk_rst_desc ufsao_rst_desc = {
    .version = MTK_RST_SET_CLR,
    .rst_bank_ofs = ufsao_rst_ofs,
    .rst_bank_nr = ARRAY_SIZE(ufsao_rst_ofs),
    .rst_idx_map = ufsao_rst_idx_map,
    .rst_idx_map_nr = ARRAY_SIZE(ufsao_rst_idx_map),
    };
    static const struct mtk_clk_desc ufsao_mcd = {
    .clks = ufsao_clks,
    .num_clks = ARRAY_SIZE(ufsao_clks),
    .rst_desc = &ufsao_rst_desc,
    };
    static const struct of_device_id of_match_clk_mt8196_ufs_ao[] = {
    { .compatible = "mediatek,mt8196-ufscfg-ao", .data = &ufsao_mcd },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_ufs_ao);
    static struct platform_driver clk_mt8196_ufs_ao_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8196-ufs-ao",
    .of_match_table = of_match_clk_mt8196_ufs_ao,
    },
    };
    module_platform_driver(clk_mt8196_ufs_ao_drv);
    MODULE_DESCRIPTION("MediaTek MT8196 ufs_ao clocks driver");
    MODULE_LICENSE("GPL");
