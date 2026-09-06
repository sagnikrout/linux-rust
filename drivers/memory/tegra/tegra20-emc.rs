//! Automatically rewritten from C to Rust
//! Source: drivers/memory/tegra/tegra20-emc.c
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
// Tegra20 External Memory Controller driver
//
// Author: Dmitry Osipenko <digetx@gmail.com>
//

pub const EMC_INTSTATUS: c_uint = 0x000;
pub const EMC_INTMASK: c_uint = 0x004;
pub const EMC_DBG: c_uint = 0x008;
pub const EMC_ADR_CFG_0: c_uint = 0x010;
pub const EMC_TIMING_CONTROL: c_uint = 0x028;
pub const EMC_RC: c_uint = 0x02c;
pub const EMC_RFC: c_uint = 0x030;
pub const EMC_RAS: c_uint = 0x034;
pub const EMC_RP: c_uint = 0x038;
pub const EMC_R2W: c_uint = 0x03c;
pub const EMC_W2R: c_uint = 0x040;
pub const EMC_R2P: c_uint = 0x044;
pub const EMC_W2P: c_uint = 0x048;
pub const EMC_RD_RCD: c_uint = 0x04c;
pub const EMC_WR_RCD: c_uint = 0x050;
pub const EMC_RRD: c_uint = 0x054;
pub const EMC_REXT: c_uint = 0x058;
pub const EMC_WDV: c_uint = 0x05c;
pub const EMC_QUSE: c_uint = 0x060;
pub const EMC_QRST: c_uint = 0x064;
pub const EMC_QSAFE: c_uint = 0x068;
pub const EMC_RDV: c_uint = 0x06c;
pub const EMC_REFRESH: c_uint = 0x070;
pub const EMC_BURST_REFRESH_NUM: c_uint = 0x074;
pub const EMC_PDEX2WR: c_uint = 0x078;
pub const EMC_PDEX2RD: c_uint = 0x07c;
pub const EMC_PCHG2PDEN: c_uint = 0x080;
pub const EMC_ACT2PDEN: c_uint = 0x084;
pub const EMC_AR2PDEN: c_uint = 0x088;
pub const EMC_RW2PDEN: c_uint = 0x08c;
pub const EMC_TXSR: c_uint = 0x090;
pub const EMC_TCKE: c_uint = 0x094;
pub const EMC_TFAW: c_uint = 0x098;
pub const EMC_TRPAB: c_uint = 0x09c;
pub const EMC_TCLKSTABLE: c_uint = 0x0a0;
pub const EMC_TCLKSTOP: c_uint = 0x0a4;
pub const EMC_TREFBW: c_uint = 0x0a8;
pub const EMC_QUSE_EXTRA: c_uint = 0x0ac;
pub const EMC_ODT_WRITE: c_uint = 0x0b0;
pub const EMC_ODT_READ: c_uint = 0x0b4;
pub const EMC_MRR: c_uint = 0x0ec;
pub const EMC_FBIO_CFG5: c_uint = 0x104;
pub const EMC_FBIO_CFG6: c_uint = 0x114;
pub const EMC_STAT_CONTROL: c_uint = 0x160;
pub const EMC_STAT_LLMC_CONTROL: c_uint = 0x178;
pub const EMC_STAT_PWR_CLOCK_LIMIT: c_uint = 0x198;
pub const EMC_STAT_PWR_CLOCKS: c_uint = 0x19c;
pub const EMC_STAT_PWR_COUNT: c_uint = 0x1a0;
pub const EMC_AUTO_CAL_INTERVAL: c_uint = 0x2a8;
pub const EMC_CFG_2: c_uint = 0x2b8;
pub const EMC_CFG_DIG_DLL: c_uint = 0x2bc;
pub const EMC_DLL_XFORM_DQS: c_uint = 0x2c0;
pub const EMC_DLL_XFORM_QUSE: c_uint = 0x2c4;
pub const EMC_ZCAL_REF_CNT: c_uint = 0x2e0;
pub const EMC_ZCAL_WAIT_CNT: c_uint = 0x2e4;
pub const EMC_CFG_CLKTRIM_0: c_uint = 0x2d0;
pub const EMC_CFG_CLKTRIM_1: c_uint = 0x2d4;
pub const EMC_CFG_CLKTRIM_2: c_uint = 0x2d8;

    enum emc_dram_type {
    DRAM_TYPE_RESERVED,
    DRAM_TYPE_DDR1,
    DRAM_TYPE_LPDDR2,
    DRAM_TYPE_DDR2,
    };
    static const u16 emc_timing_registers[] = {
    EMC_RC,
    EMC_RFC,
    EMC_RAS,
    EMC_RP,
    EMC_R2W,
    EMC_W2R,
    EMC_R2P,
    EMC_W2P,
    EMC_RD_RCD,
    EMC_WR_RCD,
    EMC_RRD,
    EMC_REXT,
    EMC_WDV,
    EMC_QUSE,
    EMC_QRST,
    EMC_QSAFE,
    EMC_RDV,
    EMC_REFRESH,
    EMC_BURST_REFRESH_NUM,
    EMC_PDEX2WR,
    EMC_PDEX2RD,
    EMC_PCHG2PDEN,
    EMC_ACT2PDEN,
    EMC_AR2PDEN,
    EMC_RW2PDEN,
    EMC_TXSR,
    EMC_TCKE,
    EMC_TFAW,
    EMC_TRPAB,
    EMC_TCLKSTABLE,
    EMC_TCLKSTOP,
    EMC_TREFBW,
    EMC_QUSE_EXTRA,
    EMC_FBIO_CFG6,
    EMC_ODT_WRITE,
    EMC_ODT_READ,
    EMC_FBIO_CFG5,
    EMC_CFG_DIG_DLL,
    EMC_DLL_XFORM_DQS,
    EMC_DLL_XFORM_QUSE,
    EMC_ZCAL_REF_CNT,
    EMC_ZCAL_WAIT_CNT,
    EMC_AUTO_CAL_INTERVAL,
    EMC_CFG_CLKTRIM_0,
    EMC_CFG_CLKTRIM_1,
    EMC_CFG_CLKTRIM_2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emc_timing {
    pub rate: c_ulong,
    pub data: [u32; ARRAY_SIZE(emc_timing_registers)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_emc {
    pub dev: *mut device,
    pub mc: *mut tegra_mc,
    pub provider: icc_provider,
    pub clk_nb: notifier_block,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub dram_bus_width: c_uint,
    pub timings: *mut emc_timing,
    pub num_timings: c_uint,
    struct {
    pub root: *mut dentry,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub debugfs: },
    pub reqs: tegra_emc_rate_requests,
    pub ondemand_data: devfreq_simple_ondemand_data,
// memory chip identity information
    pub basic_conf4: union lpddr2_basic_config4,
    pub manufacturer_id: c_uint,
    pub revision_id1: c_uint,
    pub revision_id2: c_uint,
    pub mrr_error: bool,
}

#[no_mangle]
unsafe extern "C" fn tegra20_emc_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra20_emc_isr(int irq, void *data)
    {
    struct tegra_emc *emc = data;
    let mut intmask: u32 = EMC_REFRESH_OVERFLOW_INT;
    u32 status;
    status = readl_relaxed(emc.regs + EMC_INTSTATUS) & intmask;
    if (!status)
    return IRQ_NONE;
// notify about HW problem
    if (status & EMC_REFRESH_OVERFLOW_INT)
    dev_err_ratelimited(emc.dev,
    "refresh request overflow timeout\n");
// clear interrupts
    writel_relaxed(status, emc.regs + EMC_INTSTATUS);
    return IRQ_HANDLED;
    }
    static struct emc_timing *tegra20_emc_find_timing(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = core::ptr::null_mut();
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate >= rate) {
    timing = &emc.timings[i];
    break;
    }
    }
    if (!timing) {
    dev_err(emc.dev, "no timing for rate %lu\n", rate);
    return core::ptr::null_mut();
    }
    return timing;
    }
#[no_mangle]
unsafe extern "C" fn emc_prepare_timing_change(emc: *mut tegra_emc, rate: c_ulong) -> c_int {
    static int emc_prepare_timing_change(struct tegra_emc *emc, unsigned long rate)
    {
    struct emc_timing *timing = tegra20_emc_find_timing(emc, rate);
    unsigned int i;
    if (!timing)
    return -EINVAL;
    dev_dbg(emc.dev, "%s: using timing rate %lu for requested rate %lu\n",
    __func__, timing.rate, rate);
// program shadow registers
    for (i = 0; i < ARRAY_SIZE(timing.data); i++)
    writel_relaxed(timing.data[i],
    emc.regs + emc_timing_registers[i]);
// wait until programming has settled
    readl_relaxed(emc.regs + emc_timing_registers[i - 1]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn emc_complete_timing_change(emc: *mut tegra_emc, flush: bool) -> c_int {
    static int emc_complete_timing_change(struct tegra_emc *emc, bool flush)
    {
    int err;
    u32 v;
    dev_dbg(emc.dev, "%s: flush %d\n", __func__, flush);
    if (flush) {
// manually initiate memory timing update
    writel_relaxed(EMC_TIMING_UPDATE,
    emc.regs + EMC_TIMING_CONTROL);
    return 0;
    }
    err = readl_relaxed_poll_timeout_atomic(emc.regs + EMC_INTSTATUS, v,
    v & EMC_CLKCHANGE_COMPLETE_INT,
    1, 100);
    if (err) {
    dev_err(emc.dev, "emc-car handshake timeout: %d\n", err);
    return err;
    }
    return 0;
    }
    static int tegra20_emc_clk_change_notify(struct notifier_block *nb,
    unsigned long msg, void *data)
    {
    struct tegra_emc *emc = container_of(nb, struct tegra_emc, clk_nb);
    struct clk_notifier_data *cnd = data;
    int err;
    switch (msg) {
    case PRE_RATE_CHANGE:
    err = emc_prepare_timing_change(emc, cnd.new_rate);
    break;
    case ABORT_RATE_CHANGE:
    err = emc_prepare_timing_change(emc, cnd.old_rate);
    if (err)
    break;
    err = emc_complete_timing_change(emc, true);
    break;
    case POST_RATE_CHANGE:
    err = emc_complete_timing_change(emc, false);
    break;
    default:
    return NOTIFY_DONE;
    }
    return notifier_from_errno(err);
    }
    static int load_one_timing_from_dt(struct tegra_emc *emc,
    struct emc_timing *timing,
    struct device_node *node)
    {
    u32 rate;
    int err;
    if (!of_device_is_compatible(node, "nvidia,tegra20-emc-table")) {
    dev_err(emc.dev, "incompatible DT node: %pOF\n", node);
    return -EINVAL;
    }
    err = of_property_read_u32(node, "clock-frequency", &rate);
    if (err) {
    dev_err(emc.dev, "timing %pOF: failed to read rate: %d\n",
    node, err);
    return err;
    }
    err = of_property_read_u32_array(node, "nvidia,emc-registers",
    timing.data,
    ARRAY_SIZE(emc_timing_registers));
    if (err) {
    dev_err(emc.dev,
    "timing %pOF: failed to read emc timing data: %d\n",
    node, err);
    return err;
    }
//
// The EMC clock rate is twice the bus rate, and the bus rate is
// measured in kHz.
//
    timing.rate = rate * 2 * 1000;
    dev_dbg(emc.dev, "%s: %pOF: EMC rate %lu\n",
    __func__, node, timing.rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_timings(_a: *const c_void, _b: *const c_void) -> c_int {
    static int cmp_timings(const void *_a, const void *_b)
    {
    const struct emc_timing *a = _a;
    const struct emc_timing *b = _b;
    if (a.rate < b.rate)
    return -1;
    if (a.rate > b.rate)
    return 1;
    return 0;
    }
    static int tegra20_emc_load_timings_from_dt(struct tegra_emc *emc,
    struct device_node *node)
    {
    struct emc_timing *timing;
    int child_count;
    int err;
    child_count = of_get_child_count(node);
    if (!child_count) {
    dev_err(emc.dev, "no memory timings in DT node: %pOF\n", node);
    return -EINVAL;
    }
    emc.timings = devm_kcalloc(emc.dev, child_count, sizeof(*timing),
    GFP_KERNEL);
    if (!emc.timings)
    return -ENOMEM;
    timing = emc.timings;
    for_each_child_of_node_scoped(node, child) {
    if (of_node_name_eq(child, "lpddr2"))
    continue;
    err = load_one_timing_from_dt(emc, timing++, child);
    if (err)
    return err;
    emc.num_timings++;
    }
    sort(emc.timings, emc.num_timings, sizeof(*timing), cmp_timings,
    core::ptr::null_mut());
    dev_info_once(emc.dev,
    "got %u timings for RAM code %u (min %luMHz max %luMHz)\n",
    emc.num_timings,
    tegra_read_ram_code(),
    emc.timings[0].rate / 1000000,
    emc.timings[emc.num_timings - 1].rate / 1000000);
    return 0;
    }
    static struct device_node *
    tegra20_emc_find_node_by_ram_code(struct tegra_emc *emc)
    {
    struct device *dev = emc.dev;
    struct device_node *np;
    u32 value, ram_code;
    int err;
    if (emc.mrr_error) {
    dev_warn(dev, "memory timings skipped due to MRR error\n");
    return core::ptr::null_mut();
    }
    if (of_get_child_count(dev.of_node) == 0) {
    dev_info_once(dev, "device-tree doesn't have memory timings\n");
    return core::ptr::null_mut();
    }
    if (!of_property_read_bool(dev.of_node, "nvidia,use-ram-code"))
    return of_node_get(dev.of_node);
    ram_code = tegra_read_ram_code();
    for_each_child_of_node(dev.of_node, np) {
    if (!of_node_name_eq(np, "emc-tables"))
    continue;
    err = of_property_read_u32(np, "nvidia,ram-code", &value);
    if (err || value != ram_code) {
    struct device_node *lpddr2_np;
    let mut cfg_mismatches: bool = false;
    lpddr2_np = of_get_child_by_name(np, "lpddr2");
    if (lpddr2_np) {
    const struct lpddr2_info *info;
    info = of_lpddr2_get_info(lpddr2_np, dev);
    if (info) {
    if (info.manufacturer_id >= 0 &&
    info.manufacturer_id != emc.manufacturer_id)
    cfg_mismatches = true;
    if (info.revision_id1 >= 0 &&
    info.revision_id1 != emc.revision_id1)
    cfg_mismatches = true;
    if (info.revision_id2 >= 0 &&
    info.revision_id2 != emc.revision_id2)
    cfg_mismatches = true;
    if (info.density != emc.basic_conf4.density)
    cfg_mismatches = true;
    if (info.io_width != emc.basic_conf4.io_width)
    cfg_mismatches = true;
    if (info.arch_type != emc.basic_conf4.arch_type)
    cfg_mismatches = true;
    } else {
    dev_err(dev, "failed to parse %pOF\n", lpddr2_np);
    cfg_mismatches = true;
    }
    of_node_put(lpddr2_np);
    } else {
    cfg_mismatches = true;
    }
    if (cfg_mismatches) {
    continue;
    }
    }
    return np;
    }
    dev_err(dev, "no memory timings for RAM code %u found in device tree\n",
    ram_code);
    return core::ptr::null_mut();
    }
    static int emc_read_lpddr_mode_register(struct tegra_emc *emc,
    unsigned int emem_dev,
    unsigned int register_addr,
    unsigned int *register_data)
    {
    let mut memory_dev: u32 = emem_dev ? 1 : 2;
    u32 val, mr_mask = 0xff;
    int err;
// clear data-valid interrupt status
    writel_relaxed(EMC_MRR_DIVLD_INT, emc.regs + EMC_INTSTATUS);
// issue mode register read request
    val  = FIELD_PREP(EMC_MRR_DEV_SELECTN, memory_dev);
    val |= FIELD_PREP(EMC_MRR_MRR_MA, register_addr);
    writel_relaxed(val, emc.regs + EMC_MRR);
// wait for the LPDDR2 data-valid interrupt
    err = readl_relaxed_poll_timeout_atomic(emc.regs + EMC_INTSTATUS, val,
    val & EMC_MRR_DIVLD_INT,
    1, 100);
    if (err) {
    dev_err(emc.dev, "mode register %u read failed: %d\n",
    register_addr, err);
    emc.mrr_error = true;
    return err;
    }
// read out mode register data
    val = readl_relaxed(emc.regs + EMC_MRR);
// register_data = FIELD_GET(EMC_MRR_MRR_DATA, val) & mr_mask;
    return 0;
    }
    static void emc_read_lpddr_sdram_info(struct tegra_emc *emc,
    unsigned int emem_dev,
    bool print_out)
    {
// these registers are standard for all LPDDR JEDEC memory chips
    emc_read_lpddr_mode_register(emc, emem_dev, 5, &emc.manufacturer_id);
    emc_read_lpddr_mode_register(emc, emem_dev, 6, &emc.revision_id1);
    emc_read_lpddr_mode_register(emc, emem_dev, 7, &emc.revision_id2);
    emc_read_lpddr_mode_register(emc, emem_dev, 8, &emc.basic_conf4.value);
    if (!print_out)
    return;
    dev_info(emc.dev, "SDRAM[dev%u]: manufacturer: 0x%x (%s) rev1: 0x%x rev2: 0x%x prefetch: S%u density: %uMbit iowidth: %ubit\n",
    emem_dev, emc.manufacturer_id,
    lpddr2_jedec_manufacturer(emc.manufacturer_id),
    emc.revision_id1, emc.revision_id2,
    4 >> emc.basic_conf4.arch_type,
    64 << emc.basic_conf4.density,
    32 >> emc.basic_conf4.io_width);
    }
#[no_mangle]
unsafe extern "C" fn emc_setup_hw(emc: *mut tegra_emc) -> c_int {
    static int emc_setup_hw(struct tegra_emc *emc)
    {
    u32 emc_cfg, emc_dbg, emc_fbio, emc_adr_cfg;
    let mut intmask: u32 = EMC_REFRESH_OVERFLOW_INT;
    static bool print_sdram_info_once;
    enum emc_dram_type dram_type;
    const char *dram_type_str;
    unsigned int emem_numdev;
    emc_cfg = readl_relaxed(emc.regs + EMC_CFG_2);
//
// Depending on a memory type, DRAM should enter either self-refresh
// or power-down state on EMC clock change.
//
    if (!(emc_cfg & EMC_CLKCHANGE_PD_ENABLE) &&
    !(emc_cfg & EMC_CLKCHANGE_SR_ENABLE)) {
    dev_err(emc.dev,
    "bootloader didn't specify DRAM auto-suspend mode\n");
    return -EINVAL;
    }
// enable EMC and CAR to handshake on PLL divider/source changes
    emc_cfg |= EMC_CLKCHANGE_REQ_ENABLE;
    writel_relaxed(emc_cfg, emc.regs + EMC_CFG_2);
// initialize interrupt
    writel_relaxed(intmask, emc.regs + EMC_INTMASK);
    writel_relaxed(intmask, emc.regs + EMC_INTSTATUS);
// ensure that unwanted debug features are disabled
    emc_dbg = readl_relaxed(emc.regs + EMC_DBG);
    emc_dbg |= EMC_DBG_CFG_PRIORITY;
    emc_dbg &= ~EMC_DBG_READ_MUX_ASSEMBLY;
    emc_dbg &= ~EMC_DBG_WRITE_MUX_ACTIVE;
    emc_dbg &= ~EMC_DBG_FORCE_UPDATE;
    writel_relaxed(emc_dbg, emc.regs + EMC_DBG);
    emc_fbio = readl_relaxed(emc.regs + EMC_FBIO_CFG5);
    if (emc_fbio & EMC_FBIO_CFG5_DRAM_WIDTH_X16)
    emc.dram_bus_width = 16;
    else
    emc.dram_bus_width = 32;
    dram_type = FIELD_GET(EMC_FBIO_CFG5_DRAM_TYPE, emc_fbio);
    switch (dram_type) {
    case DRAM_TYPE_RESERVED:
    dram_type_str = "INVALID";
    break;
    case DRAM_TYPE_DDR1:
    dram_type_str = "DDR1";
    break;
    case DRAM_TYPE_LPDDR2:
    dram_type_str = "LPDDR2";
    break;
    case DRAM_TYPE_DDR2:
    dram_type_str = "DDR2";
    break;
    }
    emc_adr_cfg = readl_relaxed(emc.regs + EMC_ADR_CFG_0);
    emem_numdev = FIELD_GET(EMC_ADR_CFG_0_EMEM_NUMDEV, emc_adr_cfg) + 1;
    dev_info_once(emc.dev, "%ubit DRAM bus, %u %s %s attached\n",
    emc.dram_bus_width, emem_numdev, dram_type_str,
    emem_numdev == 2 ? "devices" : "device");
    if (dram_type == DRAM_TYPE_LPDDR2) {
    while (emem_numdev--)
    emc_read_lpddr_sdram_info(emc, emem_numdev,
    !print_sdram_info_once);
    print_sdram_info_once = true;
    }
    return 0;
    }
    static long emc_round_rate(unsigned long rate,
    unsigned long min_rate,
    unsigned long max_rate,
    void *arg)
    {
    struct emc_timing *timing = core::ptr::null_mut();
    struct tegra_emc *emc = arg;
    unsigned int i;
    if (!emc.num_timings)
    return clk_get_rate(emc.clk);
    min_rate = min(min_rate, emc.timings[emc.num_timings - 1].rate);
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate < rate && i != emc.num_timings - 1)
    continue;
    if (emc.timings[i].rate > max_rate) {
    i = max(i, 1u) - 1;
    if (emc.timings[i].rate < min_rate)
    break;
    }
    if (emc.timings[i].rate < min_rate)
    continue;
    timing = &emc.timings[i];
    break;
    }
    if (!timing) {
    dev_err(emc.dev, "no timing for rate %lu min %lu max %lu\n",
    rate, min_rate, max_rate);
    return -EINVAL;
    }
    return timing.rate;
    }
//
// debugfs interface
//
// The memory controller driver exposes some files in debugfs that can be used
// to control the EMC frequency. The top-level directory can be found here:
//
// /sys/kernel/debug/emc
//
// It contains the following files:
//
// - available_rates: This file contains a list of valid, space-separated
// EMC frequencies.
//
// - min_rate: Writing a value to this file sets the given frequency as the
// floor of the permitted range. If this is higher than the currently
// configured EMC frequency, this will cause the frequency to be
// increased so that it stays within the valid range.
//
// - max_rate: Similarily to the min_rate file, writing a value to this file
// sets the given frequency as the ceiling of the permitted range. If
// the value is lower than the currently configured EMC frequency, this
// will cause the frequency to be decreased so that it stays within the
// valid range.
//
#[no_mangle]
unsafe extern "C" fn tegra20_emc_validate_rate(emc: *mut tegra_emc, rate: c_ulong) -> bool {
    static bool tegra20_emc_validate_rate(struct tegra_emc *emc, unsigned long rate)
    {
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++)
    if (rate == emc.timings[i].rate)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debug_available_rates_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tegra20_emc_debug_available_rates_show(struct seq_file *s, void *data)
    {
    struct tegra_emc *emc = s.private;
    const char *prefix = "";
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    seq_printf(s, "%s%lu", prefix, emc.timings[i].rate);
    prefix = " ";
    }
    seq_puts(s, "\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra20_emc_debug_available_rates);
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debug_min_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra20_emc_debug_min_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.min_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debug_min_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra20_emc_debug_min_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra20_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.min_rate = rate;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(tegra20_emc_debug_min_rate_fops,
    tegra20_emc_debug_min_rate_get,
    tegra20_emc_debug_min_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debug_max_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra20_emc_debug_max_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.max_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debug_max_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra20_emc_debug_max_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra20_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_max_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.max_rate = rate;
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(tegra20_emc_debug_max_rate_fops,
    tegra20_emc_debug_max_rate_get,
    tegra20_emc_debug_max_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra20_emc_debugfs_init(emc: *mut tegra_emc) {
    static void tegra20_emc_debugfs_init(struct tegra_emc *emc)
    {
    struct device *dev = emc.dev;
    unsigned int i;
    int err;
    emc.debugfs.min_rate = ULONG_MAX;
    emc.debugfs.max_rate = 0;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate < emc.debugfs.min_rate)
    emc.debugfs.min_rate = emc.timings[i].rate;
    if (emc.timings[i].rate > emc.debugfs.max_rate)
    emc.debugfs.max_rate = emc.timings[i].rate;
    }
    if (!emc.num_timings) {
    emc.debugfs.min_rate = clk_get_rate(emc.clk);
    emc.debugfs.max_rate = emc.debugfs.min_rate;
    }
    err = clk_set_rate_range(emc.clk, emc.debugfs.min_rate,
    emc.debugfs.max_rate);
    if (err < 0) {
    dev_err(dev, "failed to set rate range [%lu-%lu] for %pC\n",
    emc.debugfs.min_rate, emc.debugfs.max_rate,
    emc.clk);
    }
    emc.debugfs.root = debugfs_create_dir("emc", core::ptr::null_mut());
    debugfs_create_file("available_rates", 0444, emc.debugfs.root,
    emc, &tegra20_emc_debug_available_rates_fops);
    debugfs_create_file("min_rate", 0644, emc.debugfs.root,
    emc, &tegra20_emc_debug_min_rate_fops);
    debugfs_create_file("max_rate", 0644, emc.debugfs.root,
    emc, &tegra20_emc_debug_max_rate_fops);
    }
    static inline struct tegra_emc *
    to_tegra_emc_provider(struct icc_provider *provider)
    {
    return container_of(provider, struct tegra_emc, provider);
    }
    static struct icc_node_data *
    emc_of_icc_xlate_extended(const struct of_phandle_args *spec, void *data)
    {
    struct icc_provider *provider = data;
    struct icc_node_data *ndata;
    struct icc_node *node;
// External Memory is the only possible ICC route
    list_for_each_entry(node, &provider.nodes, node_list) {
    if (node.id != TEGRA_ICC_EMEM)
    continue;
    ndata = kzalloc_obj(*ndata);
    if (!ndata)
    return ERR_PTR(-ENOMEM);
//
// SRC and DST nodes should have matching TAG in order to have
// it set by default for a requested path.
//
    ndata.tag = TEGRA_MC_ICC_TAG_ISO;
    ndata.node = node;
    return ndata;
    }
    return ERR_PTR(-EPROBE_DEFER);
    }
#[no_mangle]
unsafe extern "C" fn emc_icc_set(src: *mut icc_node, dst: *mut icc_node) -> c_int {
    static int emc_icc_set(struct icc_node *src, struct icc_node *dst)
    {
    struct tegra_emc *emc = to_tegra_emc_provider(dst.provider);
    let mut peak_bw: c_ulonglong = icc_units_to_bps(dst.peak_bw);
    let mut avg_bw: c_ulonglong = icc_units_to_bps(dst.avg_bw);
    let mut rate: c_ulonglong = max(avg_bw, peak_bw);
    unsigned int dram_data_bus_width_bytes;
    int err;
//
// Tegra20 EMC runs on x2 clock rate of SDRAM bus because DDR data
// is sampled on both clock edges.  This means that EMC clock rate
// equals to the peak data-rate.
//
    dram_data_bus_width_bytes = emc.dram_bus_width / 8;
    do_div(rate, dram_data_bus_width_bytes);
    rate = min_t(u64, rate, U32_MAX);
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_ICC);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_interconnect_init(emc: *mut tegra_emc) -> c_int {
    static int tegra20_emc_interconnect_init(struct tegra_emc *emc)
    {
    const struct tegra_mc_soc *soc;
    struct icc_node *node;
    int err;
    emc.mc = devm_tegra_memory_controller_get(emc.dev);
    if (IS_ERR(emc.mc))
    return PTR_ERR(emc.mc);
    soc = emc.mc.soc;
    emc.provider.dev = emc.dev;
    emc.provider.set = emc_icc_set;
    emc.provider.data = &emc.provider;
    emc.provider.aggregate = soc.icc_ops.aggregate;
    emc.provider.xlate_extended = emc_of_icc_xlate_extended;
    icc_provider_init(&emc.provider);
// create External Memory Controller node
    node = icc_node_create(TEGRA_ICC_EMC);
    if (IS_ERR(node))
    return PTR_ERR(node);
    node.name = "External Memory Controller";
    icc_node_add(node, &emc.provider);
// link External Memory Controller to External Memory (DRAM)
    err = icc_link_create(node, TEGRA_ICC_EMEM);
    if (err)
    goto remove_nodes;
// create External Memory node
    node = icc_node_create(TEGRA_ICC_EMEM);
    if (IS_ERR(node)) {
    err = PTR_ERR(node);
    goto remove_nodes;
    }
    node.name = "External Memory (DRAM)";
    icc_node_add(node, &emc.provider);
    err = icc_provider_register(&emc.provider);
    if (err)
    goto remove_nodes;
    return 0;
    remove_nodes:
    icc_nodes_remove(&emc.provider);
    return dev_err_probe(emc.dev, err, "failed to initialize ICC\n");
    }
#[no_mangle]
unsafe extern "C" fn devm_tegra20_emc_unset_callback(data: *mut c_void) {
    static void devm_tegra20_emc_unset_callback(void *data)
    {
    tegra20_clk_set_emc_round_callback(core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn devm_tegra20_emc_unreg_clk_notifier(data: *mut c_void) {
    static void devm_tegra20_emc_unreg_clk_notifier(void *data)
    {
    struct tegra_emc *emc = data;
    clk_notifier_unregister(emc.clk, &emc.clk_nb);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_init_clk(emc: *mut tegra_emc) -> c_int {
    static int tegra20_emc_init_clk(struct tegra_emc *emc)
    {
    int err;
    tegra20_clk_set_emc_round_callback(emc_round_rate, emc);
    err = devm_add_action_or_reset(emc.dev, devm_tegra20_emc_unset_callback,
    core::ptr::null_mut());
    if (err)
    return err;
    emc.clk = devm_clk_get(emc.dev, core::ptr::null_mut());
    if (IS_ERR(emc.clk))
    return dev_err_probe(emc.dev, PTR_ERR(emc.clk),
    "failed to get EMC clock\n");
    err = clk_notifier_register(emc.clk, &emc.clk_nb);
    if (err)
    return dev_err_probe(emc.dev, err, "failed to register clk notifier\n");
    err = devm_add_action_or_reset(emc.dev,
    devm_tegra20_emc_unreg_clk_notifier, emc);
    if (err)
    return err;
    return 0;
    }
    static int tegra20_emc_devfreq_target(struct device *dev, unsigned long *freq,
    u32 flags)
    {
    struct tegra_emc *emc = dev_get_drvdata(dev);
    struct dev_pm_opp *opp;
    unsigned long rate;
    opp = devfreq_recommended_opp(dev, freq, flags);
    if (IS_ERR(opp)) {
    dev_err(dev, "failed to find opp for %lu Hz\n", *freq);
    return PTR_ERR(opp);
    }
    rate = dev_pm_opp_get_freq(opp);
    dev_pm_opp_put(opp);
    return tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEVFREQ);
    }
    static int tegra20_emc_devfreq_get_dev_status(struct device *dev,
    struct devfreq_dev_status *stat)
    {
    struct tegra_emc *emc = dev_get_drvdata(dev);
// freeze counters
    writel_relaxed(EMC_PWR_GATHER_DISABLE, emc.regs + EMC_STAT_CONTROL);
//
// busy_time: number of clocks EMC request was accepted
// total_time: number of clocks PWR_GATHER control was set to ENABLE
//
    stat.busy_time = readl_relaxed(emc.regs + EMC_STAT_PWR_COUNT);
    stat.total_time = readl_relaxed(emc.regs + EMC_STAT_PWR_CLOCKS);
    stat.current_frequency = clk_get_rate(emc.clk);
// clear counters and restart
    writel_relaxed(EMC_PWR_GATHER_CLEAR, emc.regs + EMC_STAT_CONTROL);
    writel_relaxed(EMC_PWR_GATHER_ENABLE, emc.regs + EMC_STAT_CONTROL);
    return 0;
    }
    static struct devfreq_dev_profile tegra20_emc_devfreq_profile = {
    .polling_ms = 30,
    .target = tegra20_emc_devfreq_target,
    .get_dev_status = tegra20_emc_devfreq_get_dev_status,
    };
#[no_mangle]
unsafe extern "C" fn tegra20_emc_devfreq_init(emc: *mut tegra_emc) -> c_int {
    static int tegra20_emc_devfreq_init(struct tegra_emc *emc)
    {
    struct devfreq *devfreq;
//
// PWR_COUNT is 1/2 of PWR_CLOCKS at max, and thus, the up-threshold
// should be less than 50.  Secondly, multiple active memory clients
// may cause over 20% of lost clock cycles due to stalls caused by
// competing memory accesses.  This means that threshold should be
// set to a less than 30 in order to have a properly working governor.
//
    emc.ondemand_data.upthreshold = 20;
//
// Reset statistic gathers state, select global bandwidth for the
// statistics collection mode and set clocks counter saturation
// limit to maximum.
//
    writel_relaxed(0x00000000, emc.regs + EMC_STAT_CONTROL);
    writel_relaxed(0x00000000, emc.regs + EMC_STAT_LLMC_CONTROL);
    writel_relaxed(0xffffffff, emc.regs + EMC_STAT_PWR_CLOCK_LIMIT);
    devfreq = devm_devfreq_add_device(emc.dev, &tegra20_emc_devfreq_profile,
    DEVFREQ_GOV_SIMPLE_ONDEMAND,
    &emc.ondemand_data);
    if (IS_ERR(devfreq))
    return dev_err_probe(emc.dev, PTR_ERR(devfreq),
    "failed to initialize devfreq\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_emc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra20_emc_probe(struct platform_device *pdev)
    {
    let mut opp_params: tegra_core_opp_params = {};
    struct device_node *np;
    struct tegra_emc *emc;
    int irq, err;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    emc = devm_kzalloc(&pdev.dev, sizeof(*emc), GFP_KERNEL);
    if (!emc)
    return -ENOMEM;
    emc.clk_nb.notifier_call = tegra20_emc_clk_change_notify;
    emc.dev = &pdev.dev;
    emc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(emc.regs))
    return PTR_ERR(emc.regs);
    err = emc_setup_hw(emc);
    if (err)
    return err;
    np = tegra20_emc_find_node_by_ram_code(emc);
    if (np) {
    err = tegra20_emc_load_timings_from_dt(emc, np);
    of_node_put(np);
    if (err)
    return err;
    }
    err = devm_request_irq(&pdev.dev, irq, tegra20_emc_isr, 0,
    dev_name(&pdev.dev), emc);
    if (err) {
    dev_err(&pdev.dev, "failed to request IRQ: %d\n", err);
    return err;
    }
    err = tegra20_emc_init_clk(emc);
    if (err)
    return err;
    opp_params.init_state = true;
    err = devm_tegra_core_dev_init_opp_table(&pdev.dev, &opp_params);
    if (err)
    return err;
    platform_set_drvdata(pdev, emc);
    tegra_emc_rate_requests_init(&emc.reqs, &pdev.dev);
    tegra20_emc_debugfs_init(emc);
    tegra20_emc_interconnect_init(emc);
    tegra20_emc_devfreq_init(emc);
//
// Don't allow the kernel module to be unloaded. Unloading adds some
// extra complexity which doesn't really worth the effort in a case of
// this driver.
//
    try_module_get(THIS_MODULE);
    return 0;
    }
    static const struct of_device_id tegra20_emc_of_match[] = {
    { .compatible = "nvidia,tegra20-emc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tegra20_emc_of_match);
    static struct platform_driver tegra20_emc_driver = {
    .probe = tegra20_emc_probe,
    .driver = {
    .name = "tegra20-emc",
    .of_match_table = tegra20_emc_of_match,
    .suppress_bind_attrs = true,
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(tegra20_emc_driver);
    MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra20 EMC driver");
    MODULE_SOFTDEP("pre: governor_simpleondemand");
    MODULE_LICENSE("GPL v2");
