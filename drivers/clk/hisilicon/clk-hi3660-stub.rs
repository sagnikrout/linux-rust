//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clk-hi3660-stub.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Hisilicon clock driver
//
// Copyright (c) 2013-2017 Hisilicon Limited.
// Copyright (c) 2017 Linaro Limited.
//
// Author: Kai Zhao <zhaokai1@hisilicon.com>
// Tao Wang <kevin.wangtao@hisilicon.com>
// Leo Yan <leo.yan@linaro.org>
//

    {							\
    .id = (_id),					\
    .cmd = (_cmd),					\
    .hw.init = &(struct clk_init_data) {		\
    .name = #_name,				\
    .ops = &hi3660_stub_clk_ops,		\
    .num_parents = 0,			\
    .flags = CLK_GET_RATE_NOCACHE,		\
    },						\
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_stub_clk_chan {
    pub cl: mbox_client,
    pub mbox: *mut mbox_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_stub_clk {
    pub id: c_uint,
    pub hw: clk_hw,
    pub cmd: c_uint,
    pub msg: [c_uint; 8],
    pub rate: c_uint,
}

    static void __iomem *freq_reg;
    static struct hi3660_stub_clk_chan stub_clk_chan;
    static unsigned long hi3660_stub_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct hi3660_stub_clk *stub_clk = to_stub_clk(hw);
//
// LPM3 writes back the CPU frequency in shared SRAM so read
// back the frequency.
//
    stub_clk.rate = readl(freq_reg + (stub_clk.id << 2)) * MHZ;
    return stub_clk.rate;
    }
    static int hi3660_stub_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct hi3660_stub_clk *stub_clk = to_stub_clk(hw);
    stub_clk.msg[0] = stub_clk.cmd;
    stub_clk.msg[1] = rate / MHZ;
    dev_dbg(stub_clk_chan.cl.dev, "set rate msg[0]=0x%x msg[1]=0x%x\n",
    stub_clk.msg[0], stub_clk.msg[1]);
    mbox_send_message(stub_clk_chan.mbox, stub_clk.msg);
    mbox_client_txdone(stub_clk_chan.mbox, 0);
    stub_clk.rate = rate;
    return 0;
    }
    static const struct clk_ops hi3660_stub_clk_ops = {
    .recalc_rate    = hi3660_stub_clk_recalc_rate,
    .determine_rate = clk_determine_rate_noop,
    .set_rate       = hi3660_stub_clk_set_rate,
    };
    static struct hi3660_stub_clk hi3660_stub_clks[HI3660_CLK_STUB_NUM] = {
    DEFINE_CLK_STUB(HI3660_CLK_STUB_CLUSTER0, 0x0001030A, "cpu-cluster.0"),
    DEFINE_CLK_STUB(HI3660_CLK_STUB_CLUSTER1, 0x0002030A, "cpu-cluster.1"),
    DEFINE_CLK_STUB(HI3660_CLK_STUB_GPU, 0x0003030A, "clk-g3d"),
    DEFINE_CLK_STUB(HI3660_CLK_STUB_DDR, 0x00040309, "clk-ddrc"),
    };
    static struct clk_hw *hi3660_stub_clk_hw_get(struct of_phandle_args *clkspec,
    void *data)
    {
    let mut idx: c_uint = clkspec.args[0];
    if (idx >= HI3660_CLK_STUB_NUM) {
    pr_err("%s: invalid index %u\n", __func__, idx);
    return ERR_PTR(-EINVAL);
    }
    return &hi3660_stub_clks[idx].hw;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_stub_clk_probe(pdev: *mut platform_device) -> c_int {
    static int hi3660_stub_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    unsigned int i;
    int ret;
// Use mailbox client without blocking
    stub_clk_chan.cl.dev = dev;
    stub_clk_chan.cl.tx_done = core::ptr::null_mut();
    stub_clk_chan.cl.tx_block = false;
    stub_clk_chan.cl.knows_txdone = false;
// Allocate mailbox channel
    stub_clk_chan.mbox = mbox_request_channel(&stub_clk_chan.cl, 0);
    if (IS_ERR(stub_clk_chan.mbox))
    return PTR_ERR(stub_clk_chan.mbox);
    freq_reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(freq_reg))
    return PTR_ERR(freq_reg);
    freq_reg += HI3660_STUB_CLOCK_DATA;
    for (i = 0; i < HI3660_CLK_STUB_NUM; i++) {
    ret = devm_clk_hw_register(&pdev.dev, &hi3660_stub_clks[i].hw);
    if (ret)
    return ret;
    }
    return devm_of_clk_add_hw_provider(&pdev.dev, hi3660_stub_clk_hw_get,
    hi3660_stub_clks);
    }
    static const struct of_device_id hi3660_stub_clk_of_match[] = {
    { .compatible = "hisilicon,hi3660-stub-clk", },
    {}
    };
    static struct platform_driver hi3660_stub_clk_driver = {
    .probe	= hi3660_stub_clk_probe,
    .driver = {
    .name = "hi3660-stub-clk",
    .of_match_table = hi3660_stub_clk_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn hi3660_stub_clk_init() -> int __init {
    static int __init hi3660_stub_clk_init(void)
    {
    return platform_driver_register(&hi3660_stub_clk_driver);
    }
    subsys_initcall(hi3660_stub_clk_init);
