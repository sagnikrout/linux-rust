//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/qcom/osm-l3.c
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
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const CLK_HW_DIV: c_int = 2;
// OSM Register offsets
pub const REG_ENABLE: c_uint = 0x0;
pub const OSM_LUT_ROW_SIZE: c_int = 32;
pub const OSM_REG_FREQ_LUT: c_uint = 0x110;
pub const OSM_REG_PERF_STATE: c_uint = 0x920;
// EPSS Register offsets
pub const EPSS_LUT_ROW_SIZE: c_int = 4;
pub const EPSS_REG_L3_VOTE: c_uint = 0x90;
pub const EPSS_REG_FREQ_LUT: c_uint = 0x100;
pub const EPSS_REG_PERF_STATE: c_uint = 0x320;

    container_of(_provider, struct qcom_osm_l3_icc_provider, provider)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_osm_l3_icc_provider {
    pub base: *mut void __iomem,
    pub max_state: c_uint,
    pub reg_perf_state: c_uint,
    pub lut_tables: [c_ulong; LUT_MAX_ENTRIES],
    pub provider: icc_provider,
}

//
// struct qcom_osm_l3_node - Qualcomm specific interconnect nodes
// @name: the node name used in debugfs
// @buswidth: width of the interconnect between a node and the bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_osm_l3_node {
    pub name: *const c_char,
    pub buswidth: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_osm_l3_desc {
    pub nodes: *const *const qcom_osm_l3_node,
    pub num_nodes: usize,
    pub lut_row_size: c_uint,
    pub reg_freq_lut: c_uint,
    pub reg_perf_state: c_uint,
    pub lut_max_entries: c_uint,
}

    static const struct qcom_osm_l3_node _name = {			\
    .name = #_name,						\
    .buswidth = _buswidth,					\
    }
    DEFINE_QNODE(osm_l3_slave, 16);
    DEFINE_QNODE(osm_l3_master, 16);
    static const struct qcom_osm_l3_node * const osm_l3_nodes[] = {
    [MASTER_OSM_L3_APPS] = &osm_l3_master,
    [SLAVE_OSM_L3] = &osm_l3_slave,
    };
    DEFINE_QNODE(epss_l3_slave, 32);
    DEFINE_QNODE(epss_l3_master, 32);
    static const struct qcom_osm_l3_node * const epss_l3_nodes[] = {
    [MASTER_EPSS_L3_APPS] = &epss_l3_master,
    [SLAVE_EPSS_L3_SHARED] = &epss_l3_slave,
    };
    static const struct qcom_osm_l3_desc osm_l3 = {
    .nodes = osm_l3_nodes,
    .num_nodes = ARRAY_SIZE(osm_l3_nodes),
    .lut_row_size = OSM_LUT_ROW_SIZE,
    .reg_freq_lut = OSM_REG_FREQ_LUT,
    .reg_perf_state = OSM_REG_PERF_STATE,
    .lut_max_entries = LUT_MAX_ENTRIES,
    };
    static const struct qcom_osm_l3_desc epss_l3_perf_state = {
    .nodes = epss_l3_nodes,
    .num_nodes = ARRAY_SIZE(epss_l3_nodes),
    .lut_row_size = EPSS_LUT_ROW_SIZE,
    .reg_freq_lut = EPSS_REG_FREQ_LUT,
    .reg_perf_state = EPSS_REG_PERF_STATE,
    .lut_max_entries = LUT_MAX_ENTRIES,
    };
    static const struct qcom_osm_l3_desc shikra_epss_l3_perf_state = {
    .nodes = epss_l3_nodes,
    .num_nodes = ARRAY_SIZE(epss_l3_nodes),
    .lut_row_size = EPSS_LUT_ROW_SIZE,
    .reg_freq_lut = EPSS_REG_FREQ_LUT,
    .reg_perf_state = EPSS_REG_PERF_STATE,
    .lut_max_entries = 12,
    };
    static const struct qcom_osm_l3_desc epss_l3_l3_vote = {
    .nodes = epss_l3_nodes,
    .num_nodes = ARRAY_SIZE(epss_l3_nodes),
    .lut_row_size = EPSS_LUT_ROW_SIZE,
    .reg_freq_lut = EPSS_REG_FREQ_LUT,
    .reg_perf_state = EPSS_REG_L3_VOTE,
    .lut_max_entries = LUT_MAX_ENTRIES,
    };
#[no_mangle]
unsafe extern "C" fn qcom_osm_l3_set(src: *mut icc_node, dst: *mut icc_node) -> c_int {
    static int qcom_osm_l3_set(struct icc_node *src, struct icc_node *dst)
    {
    struct qcom_osm_l3_icc_provider *qp;
    struct icc_provider *provider;
    const struct qcom_osm_l3_node *qn;
    unsigned int index;
    u64 rate;
    qn = src.data;
    provider = src.provider;
    qp = to_osm_l3_provider(provider);
    rate = icc_units_to_bps(dst.peak_bw);
    do_div(rate, qn.buswidth);
    for (index = 0; index < qp.max_state - 1; index++) {
    if (qp.lut_tables[index] >= rate)
    break;
    }
    writel_relaxed(index, qp.base + qp.reg_perf_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_osm_l3_remove(pdev: *mut platform_device) {
    static void qcom_osm_l3_remove(struct platform_device *pdev)
    {
    struct qcom_osm_l3_icc_provider *qp = platform_get_drvdata(pdev);
    icc_provider_deregister(&qp.provider);
    icc_nodes_remove(&qp.provider);
    }
#[no_mangle]
unsafe extern "C" fn qcom_osm_l3_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_osm_l3_probe(struct platform_device *pdev)
    {
    u32 info, src, lval, i, prev_freq = 0, freq;
    static unsigned long hw_rate, xo_rate;
    struct qcom_osm_l3_icc_provider *qp;
    const struct qcom_osm_l3_desc *desc;
    struct icc_onecell_data *data;
    struct icc_provider *provider;
    const struct qcom_osm_l3_node * const *qnodes;
    struct icc_node *node;
    size_t num_nodes;
    struct clk *clk;
    int ret;
    clk = clk_get(&pdev.dev, "xo");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    xo_rate = clk_get_rate(clk);
    clk_put(clk);
    clk = clk_get(&pdev.dev, "alternate");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    hw_rate = clk_get_rate(clk) / CLK_HW_DIV;
    clk_put(clk);
    qp = devm_kzalloc(&pdev.dev, sizeof(*qp), GFP_KERNEL);
    if (!qp)
    return -ENOMEM;
    qp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qp.base))
    return PTR_ERR(qp.base);
// HW should be in enabled state to proceed
    if (!(readl_relaxed(qp.base + REG_ENABLE) & 0x1)) {
    dev_err(&pdev.dev, "error hardware not enabled\n");
    return -ENODEV;
    }
    desc = device_get_match_data(&pdev.dev);
    if (!desc)
    return -EINVAL;
    qp.reg_perf_state = desc.reg_perf_state;
    for (i = 0; i < desc.lut_max_entries; i++) {
    info = readl_relaxed(qp.base + desc.reg_freq_lut +
    i * desc.lut_row_size);
    src = FIELD_GET(LUT_SRC, info);
    lval = FIELD_GET(LUT_L_VAL, info);
    if (src)
    freq = xo_rate * lval;
    else
    freq = hw_rate;
// Two of the same frequencies signify end of table
    if (i > 0 && prev_freq == freq)
    break;
    dev_dbg(&pdev.dev, "index=%d freq=%d\n", i, freq);
    qp.lut_tables[i] = freq;
    prev_freq = freq;
    }
    qp.max_state = i;
    qnodes = desc.nodes;
    num_nodes = desc.num_nodes;
    data = devm_kzalloc(&pdev.dev, struct_size(data, nodes, num_nodes), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.num_nodes = num_nodes;
    provider = &qp.provider;
    provider.dev = &pdev.dev;
    provider.set = qcom_osm_l3_set;
    provider.aggregate = icc_std_aggregate;
    provider.xlate = of_icc_xlate_onecell;
    provider.data = data;
    icc_provider_init(provider);
// Create nodes
    for (i = 0; i < num_nodes; i++) {
    node = icc_node_create_dyn();
    if (IS_ERR(node)) {
    ret = PTR_ERR(node);
    goto err;
    }
    ret = icc_node_set_name(node, provider, qnodes[i].name);
    if (ret) {
    icc_node_destroy(node.id);
    goto err;
    }
// Cast away const and add it back in qcom_osm_l3_set()
    node.data = (void *)qnodes[i];
    icc_node_add(node, provider);
    data.nodes[i] = node;
    }
// Create link
    icc_link_nodes(data.nodes[MASTER_OSM_L3_APPS], &data.nodes[SLAVE_OSM_L3]);
    ret = icc_provider_register(provider);
    if (ret)
    goto err;
    platform_set_drvdata(pdev, qp);
    return 0;
    err:
    icc_nodes_remove(provider);
    return ret;
    }
    static const struct of_device_id osm_l3_of_match[] = {
    { .compatible = "qcom,epss-l3", .data = &epss_l3_l3_vote },
    { .compatible = "qcom,osm-l3", .data = &osm_l3 },
    { .compatible = "qcom,sa8775p-epss-l3", .data = &epss_l3_perf_state },
    { .compatible = "qcom,sc7180-osm-l3", .data = &osm_l3 },
    { .compatible = "qcom,sc7280-epss-l3", .data = &epss_l3_perf_state },
    { .compatible = "qcom,sdm845-osm-l3", .data = &osm_l3 },
    { .compatible = "qcom,shikra-epss-l3", .data = &shikra_epss_l3_perf_state },
    { .compatible = "qcom,sm8150-osm-l3", .data = &osm_l3 },
    { .compatible = "qcom,sc8180x-osm-l3", .data = &osm_l3 },
    { .compatible = "qcom,sm8250-epss-l3", .data = &epss_l3_perf_state },
    { }
    };
    MODULE_DEVICE_TABLE(of, osm_l3_of_match);
    static struct platform_driver osm_l3_driver = {
    .probe = qcom_osm_l3_probe,
    .remove = qcom_osm_l3_remove,
    .driver = {
    .name = "osm-l3",
    .of_match_table = osm_l3_of_match,
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(osm_l3_driver);
    MODULE_DESCRIPTION("Qualcomm OSM L3 interconnect driver");
    MODULE_LICENSE("GPL v2");
