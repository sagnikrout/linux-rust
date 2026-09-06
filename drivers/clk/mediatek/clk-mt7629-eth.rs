//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt7629-eth.c
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
// Copyright (C) 2018 MediaTek Inc.
// Author: Wenzhen Yu <Wenzhen Yu@mediatek.com>
// Ryder Lee <ryder.lee@mediatek.com>
//

    GATE_MTK(_id, _name, _parent, &eth_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate_regs eth_cg_regs = {
    .set_ofs = 0x30,
    .clr_ofs = 0x30,
    .sta_ofs = 0x30,
    };
    static const struct mtk_gate eth_clks[] = {
    GATE_ETH(CLK_ETH_FE_EN, "eth_fe_en", "eth2pll", 6),
    GATE_ETH(CLK_ETH_GP2_EN, "eth_gp2_en", "txclk_src_pre", 7),
    GATE_ETH(CLK_ETH_GP1_EN, "eth_gp1_en", "txclk_src_pre", 8),
    GATE_ETH(CLK_ETH_GP0_EN, "eth_gp0_en", "txclk_src_pre", 9),
    GATE_ETH(CLK_ETH_ESW_EN, "eth_esw_en", "eth_500m", 16),
    };
    static const struct mtk_gate_regs sgmii_cg_regs = {
    .set_ofs = 0xE4,
    .clr_ofs = 0xE4,
    .sta_ofs = 0xE4,
    };

    GATE_MTK(_id, _name, _parent, &sgmii_cg_regs, _shift, &mtk_clk_gate_ops_no_setclr_inv)
    static const struct mtk_gate sgmii_clks[2][4] = {
    {
    GATE_SGMII(CLK_SGMII_TX_EN, "sgmii_tx_en",
    "ssusb_tx250m", 2),
    GATE_SGMII(CLK_SGMII_RX_EN, "sgmii_rx_en",
    "ssusb_eq_rx250m", 3),
    GATE_SGMII(CLK_SGMII_CDR_REF, "sgmii_cdr_ref",
    "ssusb_cdr_ref", 4),
    GATE_SGMII(CLK_SGMII_CDR_FB, "sgmii_cdr_fb",
    "ssusb_cdr_fb", 5),
    }, {
    GATE_SGMII(CLK_SGMII_TX_EN, "sgmii_tx_en1",
    "ssusb_tx250m", 2),
    GATE_SGMII(CLK_SGMII_RX_EN, "sgmii_rx_en1",
    "ssusb_eq_rx250m", 3),
    GATE_SGMII(CLK_SGMII_CDR_REF, "sgmii_cdr_ref1",
    "ssusb_cdr_ref", 4),
    GATE_SGMII(CLK_SGMII_CDR_FB, "sgmii_cdr_fb1",
    "ssusb_cdr_fb", 5),
    }
    };
    static u16 rst_ofs[] = { 0x34, };
    static const struct mtk_clk_rst_desc clk_rst_desc = {
    .version = MTK_RST_SIMPLE,
    .rst_bank_ofs = rst_ofs,
    .rst_bank_nr = ARRAY_SIZE(rst_ofs),
    };
#[no_mangle]
unsafe extern "C" fn clk_mt7629_ethsys_init(pdev: *mut platform_device) -> c_int {
    static int clk_mt7629_ethsys_init(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    int r;
    clk_data = mtk_alloc_clk_data(CLK_ETH_NR_CLK);
    if (!clk_data)
    return -ENOMEM;
    mtk_clk_register_gates(&pdev.dev, node, eth_clks,
    CLK_ETH_NR_CLK, clk_data);
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    dev_err(&pdev.dev,
    "could not register clock provider: %s: %d\n",
    pdev.name, r);
    mtk_register_reset_controller_with_dev(&pdev.dev, &clk_rst_desc);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt7629_sgmiisys_init(pdev: *mut platform_device) -> c_int {
    static int clk_mt7629_sgmiisys_init(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data;
    struct device_node *node = pdev.dev.of_node;
    static int id;
    int r;
    clk_data = mtk_alloc_clk_data(CLK_SGMII_NR_CLK);
    if (!clk_data)
    return -ENOMEM;
    mtk_clk_register_gates(&pdev.dev, node, sgmii_clks[id++],
    CLK_SGMII_NR_CLK, clk_data);
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    dev_err(&pdev.dev,
    "could not register clock provider: %s: %d\n",
    pdev.name, r);
    return r;
    }
    static const struct of_device_id of_match_clk_mt7629_eth[] = {
    {
    .compatible = "mediatek,mt7629-ethsys",
    .data = clk_mt7629_ethsys_init,
    }, {
    .compatible = "mediatek,mt7629-sgmiisys",
    .data = clk_mt7629_sgmiisys_init,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt7629_eth);
#[no_mangle]
unsafe extern "C" fn clk_mt7629_eth_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt7629_eth_probe(struct platform_device *pdev)
    {
    int (*clk_init)(struct platform_device *);
    int r;
    clk_init = of_device_get_match_data(&pdev.dev);
    if (!clk_init)
    return -EINVAL;
    r = clk_init(pdev);
    if (r)
    dev_err(&pdev.dev,
    "could not register clock provider: %s: %d\n",
    pdev.name, r);
    return r;
    }
    static struct platform_driver clk_mt7629_eth_drv = {
    .probe = clk_mt7629_eth_probe,
    .driver = {
    .name = "clk-mt7629-eth",
    .of_match_table = of_match_clk_mt7629_eth,
    },
    };
    builtin_platform_driver(clk_mt7629_eth_drv);
    MODULE_DESCRIPTION("MediaTek MT7629 Ethernet clocks driver");
    MODULE_LICENSE("GPL");
