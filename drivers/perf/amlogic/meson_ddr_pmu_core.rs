//! Automatically rewritten from C to Rust
//! Source: drivers/perf/amlogic/meson_ddr_pmu_core.c
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
// Copyright (c) 2022 Amlogic, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_pmu {
    pub pmu: pmu,
    pub info: dmc_info,
    pub /: *mut *mut dmc_counter counters; / save counters from hw,
    pub pmu_enabled: bool,
    pub dev: *mut device,
    pub name: *mut c_char,
    pub node: hlist_node,
    pub cpuhp_state: enum cpuhp_state,
    pub /: *mut *mut int cpu; / for cpu hotplug,
}

#[no_mangle]
unsafe extern "C" fn dmc_pmu_enable(pmu: *mut ddr_pmu) {
    static void dmc_pmu_enable(struct ddr_pmu *pmu)
    {
    if (!pmu.pmu_enabled)
    pmu.info.hw_info.enable(&pmu.info);
    pmu.pmu_enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn dmc_pmu_disable(pmu: *mut ddr_pmu) {
    static void dmc_pmu_disable(struct ddr_pmu *pmu)
    {
    if (pmu.pmu_enabled)
    pmu.info.hw_info.disable(&pmu.info);
    pmu.pmu_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_set_axi_filter(event: *mut perf_event, axi_id: u8) {
    static void meson_ddr_set_axi_filter(struct perf_event *event, u8 axi_id)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    int chann;
    if (event.attr.config > ALL_CHAN_COUNTER_ID &&
    event.attr.config < COUNTER_MAX_ID) {
    chann = event.attr.config - CHAN1_COUNTER_ID;
    pmu.info.hw_info.set_axi_filter(&pmu.info, axi_id, chann);
    }
    }
    static void ddr_cnt_addition(struct dmc_counter *sum,
    struct dmc_counter *add1,
    struct dmc_counter *add2,
    int chann_nr)
    {
    int i;
    u64 cnt1, cnt2;
    sum.all_cnt = add1.all_cnt + add2.all_cnt;
    sum.all_req = add1.all_req + add2.all_req;
    for (i = 0; i < chann_nr; i++) {
    cnt1 = add1.channel_cnt[i];
    cnt2 = add2.channel_cnt[i];
    sum.channel_cnt[i] = cnt1 + cnt2;
    }
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_update(event: *mut perf_event) {
    static void meson_ddr_perf_event_update(struct perf_event *event)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    let mut new_raw_count: u64 = 0;
    let mut dc: dmc_counter = {0}, sum_dc = {0};
    int idx;
    let mut chann_nr: c_int = pmu.info.hw_info.chann_nr;
// get the remain counters in register.
    pmu.info.hw_info.get_counters(&pmu.info, &dc);
    ddr_cnt_addition(&sum_dc, &pmu.counters, &dc, chann_nr);
    switch (event.attr.config) {
    case ALL_CHAN_COUNTER_ID:
    new_raw_count = sum_dc.all_cnt;
    break;
    case CHAN1_COUNTER_ID:
    case CHAN2_COUNTER_ID:
    case CHAN3_COUNTER_ID:
    case CHAN4_COUNTER_ID:
    case CHAN5_COUNTER_ID:
    case CHAN6_COUNTER_ID:
    case CHAN7_COUNTER_ID:
    case CHAN8_COUNTER_ID:
    idx = event.attr.config - CHAN1_COUNTER_ID;
    new_raw_count = sum_dc.channel_cnt[idx];
    break;
    }
    local64_set(&event.count, new_raw_count);
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_init(event: *mut perf_event) -> c_int {
    static int meson_ddr_perf_event_init(struct perf_event *event)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    let mut config1: u64 = event.attr.config1;
    let mut config2: u64 = event.attr.config2;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    if (is_sampling_event(event) || event.attach_state & PERF_ATTACH_TASK)
    return -EOPNOTSUPP;
    if (event.cpu < 0)
    return -EOPNOTSUPP;
// check if the number of parameters is too much
    if (event.attr.config != ALL_CHAN_COUNTER_ID &&
    hweight64(config1) + hweight64(config2) > MAX_AXI_PORTS_OF_CHANNEL)
    return -EOPNOTSUPP;
    event.cpu = pmu.cpu;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_start(event: *mut perf_event, flags: c_int) {
    static void meson_ddr_perf_event_start(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    memset(&pmu.counters, 0, sizeof(pmu.counters));
    dmc_pmu_enable(pmu);
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int meson_ddr_perf_event_add(struct perf_event *event, int flags)
    {
    let mut config1: u64 = event.attr.config1;
    let mut config2: u64 = event.attr.config2;
    int i;
    for_each_set_bit(i,
    (const unsigned long *)&config1,
    BITS_PER_TYPE(config1))
    meson_ddr_set_axi_filter(event, i);
    for_each_set_bit(i,
    (const unsigned long *)&config2,
    BITS_PER_TYPE(config2))
    meson_ddr_set_axi_filter(event, i + 64);
    if (flags & PERF_EF_START)
    meson_ddr_perf_event_start(event, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_stop(event: *mut perf_event, flags: c_int) {
    static void meson_ddr_perf_event_stop(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    if (flags & PERF_EF_UPDATE)
    meson_ddr_perf_event_update(event);
    dmc_pmu_disable(pmu);
    }
#[no_mangle]
unsafe extern "C" fn meson_ddr_perf_event_del(event: *mut perf_event, flags: c_int) {
    static void meson_ddr_perf_event_del(struct perf_event *event, int flags)
    {
    meson_ddr_perf_event_stop(event, PERF_EF_UPDATE);
    }
    static ssize_t meson_ddr_perf_cpumask_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask_of(pmu.cpu)));
    }
    static struct device_attribute meson_ddr_perf_cpumask_attr =
    __ATTR(cpumask, 0444, meson_ddr_perf_cpumask_show, core::ptr::null_mut());
    static struct attribute *meson_ddr_perf_cpumask_attrs[] = {
    &meson_ddr_perf_cpumask_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_cpumask_attr_group = {
    .attrs = meson_ddr_perf_cpumask_attrs,
    };
    static ssize_t
    pmu_event_show(struct device *dev, struct device_attribute *attr,
    char *page)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr);
    return sysfs_emit(page, "event=0x%02llx\n", pmu_attr.id);
    }
    static ssize_t
    event_show_unit(struct device *dev, struct device_attribute *attr,
    char *page)
    {
    return sysfs_emit(page, "MB\n");
    }
    static ssize_t
    event_show_scale(struct device *dev, struct device_attribute *attr,
    char *page)
    {
// one count = 16byte = 1.52587890625e-05 MB
    return sysfs_emit(page, "1.52587890625e-05\n");
    }

    {									\
    .attr = __ATTR(_name, 0444, pmu_event_show, core::ptr::null_mut()),		\
    .id = _id,							\
    }

    __ATTR(_name.unit, 0444, event_show_unit, core::ptr::null_mut())

    __ATTR(_name.scale, 0444, event_show_scale, core::ptr::null_mut())
    static struct device_attribute event_unit_attrs[] = {
    AML_DDR_PMU_EVENT_UNIT_ATTR(total_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_1_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_2_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_3_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_4_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_5_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_6_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_7_rw_bytes),
    AML_DDR_PMU_EVENT_UNIT_ATTR(chan_8_rw_bytes),
    };
    static struct device_attribute event_scale_attrs[] = {
    AML_DDR_PMU_EVENT_SCALE_ATTR(total_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_1_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_2_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_3_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_4_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_5_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_6_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_7_rw_bytes),
    AML_DDR_PMU_EVENT_SCALE_ATTR(chan_8_rw_bytes),
    };
    static struct perf_pmu_events_attr event_attrs[] = {
    AML_DDR_PMU_EVENT_ATTR(total_rw_bytes, ALL_CHAN_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_1_rw_bytes, CHAN1_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_2_rw_bytes, CHAN2_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_3_rw_bytes, CHAN3_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_4_rw_bytes, CHAN4_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_5_rw_bytes, CHAN5_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_6_rw_bytes, CHAN6_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_7_rw_bytes, CHAN7_COUNTER_ID),
    AML_DDR_PMU_EVENT_ATTR(chan_8_rw_bytes, CHAN8_COUNTER_ID),
    };
// three attrs are combined an event
    static struct attribute *ddr_perf_events_attrs[COUNTER_MAX_ID * 3];
    static struct attribute_group ddr_perf_events_attr_group = {
    .name = "events",
    .attrs = ddr_perf_events_attrs,
    };
    static umode_t meson_ddr_perf_format_attr_visible(struct kobject *kobj,
    struct attribute *attr,
    int n)
    {
    struct pmu *pmu = dev_get_drvdata(kobj_to_dev(kobj));
    struct ddr_pmu *ddr_pmu = to_ddr_pmu(pmu);
    const u64 *capability = ddr_pmu.info.hw_info.capability;
    struct device_attribute *dev_attr;
    int id;
    char value[20]; // config1:xxx, 20 is enough
    dev_attr = container_of(attr, struct device_attribute, attr);
    dev_attr.show(core::ptr::null_mut(), core::ptr::null_mut(), value);
    if (sscanf(value, "config1:%d", &id) == 1)
    return capability[0] & (1ULL << id) ? attr.mode : 0;
    if (sscanf(value, "config2:%d", &id) == 1)
    return capability[1] & (1ULL << id) ? attr.mode : 0;
    return attr.mode;
    }
    static struct attribute_group ddr_perf_format_attr_group = {
    .name = "format",
    .is_visible = meson_ddr_perf_format_attr_visible,
    };
    static ssize_t meson_ddr_perf_identifier_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    return sysfs_emit(page, "%s\n", pmu.name);
    }
    static struct device_attribute meson_ddr_perf_identifier_attr =
    __ATTR(identifier, 0444, meson_ddr_perf_identifier_show, core::ptr::null_mut());
    static struct attribute *meson_ddr_perf_identifier_attrs[] = {
    &meson_ddr_perf_identifier_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_identifier_attr_group = {
    .attrs = meson_ddr_perf_identifier_attrs,
    };
    static const struct attribute_group *attr_groups[] = {
    &ddr_perf_events_attr_group,
    &ddr_perf_format_attr_group,
    &ddr_perf_cpumask_attr_group,
    &ddr_perf_identifier_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn dmc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dmc_irq_handler(int irq, void *dev_id)
    {
    struct dmc_info *info = dev_id;
    struct ddr_pmu *pmu;
    struct dmc_counter counters, *sum_cnter;
    int i;
    pmu = dmc_info_to_pmu(info);
    if (info.hw_info.irq_handler(info, &counters) != 0)
    goto out;
    sum_cnter = &pmu.counters;
    sum_cnter.all_cnt += counters.all_cnt;
    sum_cnter.all_req += counters.all_req;
    for (i = 0; i < pmu.info.hw_info.chann_nr; i++)
    sum_cnter.channel_cnt[i] += counters.channel_cnt[i];
    if (pmu.pmu_enabled)
//
// the timer interrupt only supprt
// one shot mode, we have to re-enable
// it in ISR to support continue mode.
//
    info.hw_info.enable(info);
    dev_dbg(pmu.dev, "counts: %llu %llu %llu, %llu, %llu, %llu\t\t"
    "sum: %llu %llu %llu, %llu, %llu, %llu\n",
    counters.all_req,
    counters.all_cnt,
    counters.channel_cnt[0],
    counters.channel_cnt[1],
    counters.channel_cnt[2],
    counters.channel_cnt[3],
    pmu.counters.all_req,
    pmu.counters.all_cnt,
    pmu.counters.channel_cnt[0],
    pmu.counters.channel_cnt[1],
    pmu.counters.channel_cnt[2],
    pmu.counters.channel_cnt[3]);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int ddr_perf_offline_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct ddr_pmu *pmu = hlist_entry_safe(node, struct ddr_pmu, node);
    int target;
    if (cpu != pmu.cpu)
    return 0;
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    return 0;
    perf_pmu_migrate_context(&pmu.pmu, cpu, target);
    pmu.cpu = target;
    WARN_ON(irq_set_affinity(pmu.info.irq_num, cpumask_of(pmu.cpu)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fill_event_attr(pmu: *mut ddr_pmu) {
    static void fill_event_attr(struct ddr_pmu *pmu)
    {
    int i, j, k;
    struct attribute **dst = ddr_perf_events_attrs;
    j = 0;
    k = 0;
// fill ALL_CHAN_COUNTER_ID event
    dst[j++] = &event_attrs[k].attr.attr;
    dst[j++] = &event_unit_attrs[k].attr;
    dst[j++] = &event_scale_attrs[k].attr;
    k++;
// fill each channel event
    for (i = 0; i < pmu.info.hw_info.chann_nr; i++, k++) {
    dst[j++] = &event_attrs[k].attr.attr;
    dst[j++] = &event_unit_attrs[k].attr;
    dst[j++] = &event_scale_attrs[k].attr;
    }
    dst[j] = core::ptr::null_mut(); /* mark end */
    }
#[no_mangle]
unsafe extern "C" fn fmt_attr_fill(fmt_attr: *mut attribute) {
    static void fmt_attr_fill(struct attribute **fmt_attr)
    {
    ddr_perf_format_attr_group.attrs = fmt_attr;
    }
    static int ddr_pmu_parse_dt(struct platform_device *pdev,
    struct dmc_info *info)
    {
    void __iomem *base;
    int i, ret;
    info.hw_info = of_device_get_match_data(&pdev.dev);
    for (i = 0; i < info.hw_info.dmc_nr; i++) {
// resource 0 for ddr register base
    base = devm_platform_ioremap_resource(pdev, i);
    if (IS_ERR(base))
    return PTR_ERR(base);
    info.ddr_reg[i] = base;
    }
// resource i for pll register base
    base = devm_platform_ioremap_resource(pdev, i);
    if (IS_ERR(base))
    return PTR_ERR(base);
    info.pll_reg = base;
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    info.irq_num = ret;
    ret = devm_request_irq(&pdev.dev, info.irq_num, dmc_irq_handler,
    IRQF_NOBALANCING, dev_name(&pdev.dev),
    (void *)info);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn meson_ddr_pmu_create(pdev: *mut platform_device) -> c_int {
    int meson_ddr_pmu_create(struct platform_device *pdev)
    {
    int ret;
    char *name;
    struct ddr_pmu *pmu;
    pmu = devm_kzalloc(&pdev.dev, sizeof(struct ddr_pmu), GFP_KERNEL);
    if (!pmu)
    return -ENOMEM;
// pmu = (struct ddr_pmu) {
    .pmu = {
    .module		= THIS_MODULE,
    .parent		= &pdev.dev,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    .task_ctx_nr	= perf_invalid_context,
    .attr_groups	= attr_groups,
    .event_init	= meson_ddr_perf_event_init,
    .add		= meson_ddr_perf_event_add,
    .del		= meson_ddr_perf_event_del,
    .start		= meson_ddr_perf_event_start,
    .stop		= meson_ddr_perf_event_stop,
    .read		= meson_ddr_perf_event_update,
    },
    };
    ret = ddr_pmu_parse_dt(pdev, &pmu.info);
    if (ret < 0)
    return ret;
    fmt_attr_fill(pmu.info.hw_info.fmt_attr);
    pmu.cpu = raw_smp_processor_id();
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, DDR_PERF_DEV_NAME);
    if (!name)
    return -ENOMEM;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN, name, core::ptr::null_mut(),
    ddr_perf_offline_cpu);
    if (ret < 0)
    return ret;
    pmu.cpuhp_state = ret;
// Register the pmu instance for cpu hotplug
    ret = cpuhp_state_add_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    if (ret)
    goto cpuhp_instance_err;
    fill_event_attr(pmu);
    ret = perf_pmu_register(&pmu.pmu, name, -1);
    if (ret)
    goto pmu_register_err;
    pmu.name = name;
    pmu.dev = &pdev.dev;
    pmu.pmu_enabled = false;
    platform_set_drvdata(pdev, pmu);
    return 0;
    pmu_register_err:
    cpuhp_state_remove_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    cpuhp_instance_err:
    cpuhp_remove_state(pmu.cpuhp_state);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn meson_ddr_pmu_remove(pdev: *mut platform_device) -> c_int {
    int meson_ddr_pmu_remove(struct platform_device *pdev)
    {
    struct ddr_pmu *pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&pmu.pmu);
    cpuhp_state_remove_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    cpuhp_remove_state(pmu.cpuhp_state);
    return 0;
    }
