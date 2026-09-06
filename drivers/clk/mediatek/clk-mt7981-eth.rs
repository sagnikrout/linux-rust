//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt7981-eth.c
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Sam Shih <sam.shih@mediatek.com>
// Author: Wenzhen Yu <wenzhen.yu@mediatek.com>
// Author: Jianhui Zhao <zhaojh329@gmail.com>
// Author: Daniel Golle <daniel@makrotopia.org>
//

    static const struct mtk_gate_regs sgmii0_cg_regs = {
    .set_ofs = 0xE4,
    .clr_ofs = 0xE4,
    .sta_ofs = 0xE4,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &sgmii0_cg_regs,			\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_no_setclr_inv,	\
    }
    static const struct mtk_gate sgmii0_clks[] = {
    GATE_SGMII0(CLK_SGM0_TX_EN, "sgm0_tx_en", "usb_tx250m", 2),
    GATE_SGMII0(CLK_SGM0_RX_EN, "sgm0_rx_en", "usb_eq_rx250m", 3),
    GATE_SGMII0(CLK_SGM0_CK0_EN, "sgm0_ck0_en", "usb_ln0", 4),
    GATE_SGMII0(CLK_SGM0_CDR_CK0_EN, "sgm0_cdr_ck0_en", "usb_cdr", 5),
    };
    static const struct mtk_gate_regs sgmii1_cg_regs = {
    .set_ofs = 0xE4,
    .clr_ofs = 0xE4,
    .sta_ofs = 0xE4,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &sgmii1_cg_regs,			\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_no_setclr_inv,	\
    }
    static const struct mtk_gate sgmii1_clks[] = {
    GATE_SGMII1(CLK_SGM1_TX_EN, "sgm1_tx_en", "usb_tx250m", 2),
    GATE_SGMII1(CLK_SGM1_RX_EN, "sgm1_rx_en", "usb_eq_rx250m", 3),
    GATE_SGMII1(CLK_SGM1_CK1_EN, "sgm1_ck1_en", "usb_ln0", 4),
    GATE_SGMII1(CLK_SGM1_CDR_CK1_EN, "sgm1_cdr_ck1_en", "usb_cdr", 5),
    };
    static const struct mtk_gate_regs eth_cg_regs = {
    .set_ofs = 0x30,
    .clr_ofs = 0x30,
    .sta_ofs = 0x30,
    };

    .id = _id,				\
    .name = _name,				\
    .parent_name = _parent,			\
    .regs = &eth_cg_regs,			\
    .shift = _shift,			\
    .ops = &mtk_clk_gate_ops_no_setclr_inv,	\
    }
    static const struct mtk_gate eth_clks[] = {
    GATE_ETH(CLK_ETH_FE_EN, "eth_fe_en", "netsys_2x", 6),
    GATE_ETH(CLK_ETH_GP2_EN, "eth_gp2_en", "sgm_325m", 7),
    GATE_ETH(CLK_ETH_GP1_EN, "eth_gp1_en", "sgm_325m", 8),
    GATE_ETH(CLK_ETH_WOCPU0_EN, "eth_wocpu0_en", "netsys_wed_mcu", 15),
    };
    static const struct mtk_clk_desc eth_desc = {
    .clks = eth_clks,
    .num_clks = ARRAY_SIZE(eth_clks),
    };
    static const struct mtk_clk_desc sgmii0_desc = {
    .clks = sgmii0_clks,
    .num_clks = ARRAY_SIZE(sgmii0_clks),
    };
    static const struct mtk_clk_desc sgmii1_desc = {
    .clks = sgmii1_clks,
    .num_clks = ARRAY_SIZE(sgmii1_clks),
    };
    static const struct of_device_id of_match_clk_mt7981_eth[] = {
    { .compatible = "mediatek,mt7981-ethsys", .data = &eth_desc },
    { .compatible = "mediatek,mt7981-sgmiisys_0", .data = &sgmii0_desc },
    { .compatible = "mediatek,mt7981-sgmiisys_1", .data = &sgmii1_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt7981_eth);
    static struct platform_driver clk_mt7981_eth_drv = {
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    .driver = {
    .name = "clk-mt7981-eth",
    .of_match_table = of_match_clk_mt7981_eth,
    },
    };
    module_platform_driver(clk_mt7981_eth_drv);
    MODULE_DESCRIPTION("MediaTek MT7981 Ethernet clocks driver");
    MODULE_LICENSE("GPL");
