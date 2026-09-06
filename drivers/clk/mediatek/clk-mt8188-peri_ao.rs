//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8188-peri_ao.c
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

    static const struct mtk_gate_regs peri_ao_cg_regs = {
    .set_ofs = 0x10,
    .clr_ofs = 0x14,
    .sta_ofs = 0x18,
    };

    GATE_MTK(_id, _name, _parent, &peri_ao_cg_regs, _shift, &mtk_clk_gate_ops_setclr)
    static const struct mtk_gate peri_ao_clks[] = {
    GATE_PERI_AO(CLK_PERI_AO_ETHERNET, "peri_ao_ethernet", "top_axi", 0),
    GATE_PERI_AO(CLK_PERI_AO_ETHERNET_BUS, "peri_ao_ethernet_bus", "top_axi", 1),
    GATE_PERI_AO(CLK_PERI_AO_FLASHIF_BUS, "peri_ao_flashif_bus", "top_axi", 3),
    GATE_PERI_AO(CLK_PERI_AO_FLASHIF_26M, "peri_ao_flashif_26m", "clk26m", 4),
    GATE_PERI_AO(CLK_PERI_AO_FLASHIFLASHCK, "peri_ao_flashiflashck", "top_spinor", 5),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_2P_BUS, "peri_ao_ssusb_2p_bus", "top_usb_top_2p", 9),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_2P_XHCI, "peri_ao_ssusb_2p_xhci", "top_ssusb_xhci_2p", 10),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_3P_BUS, "peri_ao_ssusb_3p_bus", "top_usb_top_3p", 11),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_3P_XHCI, "peri_ao_ssusb_3p_xhci", "top_ssusb_xhci_3p", 12),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_BUS, "peri_ao_ssusb_bus", "top_usb_top", 13),
    GATE_PERI_AO(CLK_PERI_AO_SSUSB_XHCI, "peri_ao_ssusb_xhci", "top_ssusb_xhci", 14),
    GATE_PERI_AO(CLK_PERI_AO_ETHERNET_MAC, "peri_ao_ethernet_mac_clk", "top_snps_eth_250m", 16),
    GATE_PERI_AO(CLK_PERI_AO_PCIE_P0_FMEM, "peri_ao_pcie_p0_fmem", "hd_466m_fmem_ck", 24),
    };
    static const struct mtk_clk_desc peri_ao_desc = {
    .clks = peri_ao_clks,
    .num_clks = ARRAY_SIZE(peri_ao_clks),
    };
    static const struct of_device_id of_match_clk_mt8188_peri_ao[] = {
    { .compatible = "mediatek,mt8188-pericfg-ao", .data = &peri_ao_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_peri_ao);
    static struct platform_driver clk_mt8188_peri_ao_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt8188-peri_ao",
    .of_match_table = of_match_clk_mt8188_peri_ao,
    },
    };
    module_platform_driver(clk_mt8188_peri_ao_drv);
    MODULE_DESCRIPTION("MediaTek MT8188 pericfg clocks driver");
    MODULE_LICENSE("GPL");
