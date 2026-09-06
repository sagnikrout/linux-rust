//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/qcom_q6v5_mss.c
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
// Qualcomm self-authenticating modem subsystem remoteproc driver
//
// Copyright (C) 2016 Linaro Ltd.
// Copyright (C) 2014 Sony Mobile Communications AB
// Copyright (c) 2012-2013, The Linux Foundation. All rights reserved.
//

pub const MPSS_CRASH_REASON_SMEM: c_int = 421;

pub const MPSS_PAS_ID: c_int = 5;
// RMB Status Register Values
pub const RMB_PBL_SUCCESS: c_uint = 0x1;
pub const RMB_MBA_XPU_UNLOCKED: c_uint = 0x1;
pub const RMB_MBA_XPU_UNLOCKED_SCRIBBLED: c_uint = 0x2;
pub const RMB_MBA_META_DATA_AUTH_SUCCESS: c_uint = 0x3;
pub const RMB_MBA_AUTH_COMPLETE: c_uint = 0x4;
// PBL/MBA interface registers
pub const RMB_MBA_IMAGE_REG: c_uint = 0x00;
pub const RMB_PBL_STATUS_REG: c_uint = 0x04;
pub const RMB_MBA_COMMAND_REG: c_uint = 0x08;
pub const RMB_MBA_STATUS_REG: c_uint = 0x0C;
pub const RMB_PMI_META_DATA_REG: c_uint = 0x10;
pub const RMB_PMI_CODE_START_REG: c_uint = 0x14;
pub const RMB_PMI_CODE_LENGTH_REG: c_uint = 0x18;
pub const RMB_MBA_MSS_STATUS: c_uint = 0x40;
pub const RMB_MBA_ALT_RESET: c_uint = 0x44;
pub const RMB_CMD_META_DATA_READY: c_uint = 0x1;
pub const RMB_CMD_LOAD_READY: c_uint = 0x2;
// QDSP6SS Register Offsets
pub const QDSP6SS_RESET_REG: c_uint = 0x014;
pub const QDSP6SS_GFMUX_CTL_REG: c_uint = 0x020;
pub const QDSP6SS_PWR_CTL_REG: c_uint = 0x030;
pub const QDSP6SS_MEM_PWR_CTL: c_uint = 0x0B0;
pub const QDSP6V6SS_MEM_PWR_CTL: c_uint = 0x034;
pub const QDSP6SS_STRAP_ACC: c_uint = 0x110;
pub const QDSP6V62SS_BHS_STATUS: c_uint = 0x0C4;
// AXI Halt Register Offsets
pub const AXI_HALTREQ_REG: c_uint = 0x0;
pub const AXI_HALTACK_REG: c_uint = 0x4;
pub const AXI_IDLE_REG: c_uint = 0x8;

pub const HALT_ACK_TIMEOUT_US: c_int = 100000;
// QACCEPT Register Offsets
pub const QACCEPT_ACCEPT_REG: c_uint = 0x0;
pub const QACCEPT_ACTIVE_REG: c_uint = 0x4;
pub const QACCEPT_DENY_REG: c_uint = 0x8;
pub const QACCEPT_REQ_REG: c_uint = 0xC;
pub const QACCEPT_TIMEOUT_US: c_int = 50;
// QDSP6SS_RESET

// QDSP6SS CBCR

pub const Q6SS_CBCR_TIMEOUT_US: c_int = 200;
// QDSP6SS_GFMUX_CTL

// QDSP6SS_PWR_CTL

// QDSP6v55 parameters

// QDSP6v56 parameters

pub const QDSP6SS_XO_CBCR: c_uint = 0x0038;
pub const QDSP6SS_ACC_OVERRIDE_VAL: c_uint = 0x20;
pub const QDSP6SS_ACC_OVERRIDE_VAL_9607: c_uint = 0x80800000;

// QDSP6v65 parameters
pub const QDSP6SS_CORE_CBCR: c_uint = 0x20;
pub const QDSP6SS_SLEEP: c_uint = 0x3C;
pub const QDSP6SS_BOOT_CORE_START: c_uint = 0x400;
pub const QDSP6SS_BOOT_CMD: c_uint = 0x404;
pub const BOOT_FSM_TIMEOUT: c_int = 10000;
pub const BHS_CHECK_MAX_LOOPS: c_int = 200;
// External power block headswitch

pub const EXTERNAL_BHS_TIMEOUT_US: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_info {
    pub reg: *mut regulator,
    pub uV: c_int,
    pub uA: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_mss_reg_res {
    pub supply: *const c_char,
    pub uV: c_int,
    pub uA: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_hexagon_res {
    pub hexagon_mba_image: *const c_char,
    pub proxy_supply: *mut qcom_mss_reg_res,
    pub fallback_proxy_supply: *mut qcom_mss_reg_res,
    pub active_supply: *mut qcom_mss_reg_res,
    pub proxy_clk_names: *mut c_char,
    pub reset_clk_names: *mut c_char,
    pub active_clk_names: *mut c_char,
    pub proxy_pd_names: *mut c_char,
    pub version: c_int,
    pub ssctl_id: c_int,
    pub need_mem_protection: bool,
    pub need_pas_mem_setup: bool,
    pub has_alt_reset: bool,
    pub has_mba_logs: bool,
    pub has_spare_reg: bool,
    pub has_qaccept_regs: bool,
    pub has_ext_bhs_reg: bool,
    pub has_ext_cntl_regs: bool,
    pub has_vq6: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6v5 {
    pub dev: *mut device,
    pub rproc: *mut rproc,
    pub reg_base: *mut void __iomem,
    pub rmb_base: *mut void __iomem,
    pub halt_map: *mut regmap,
    pub conn_map: *mut regmap,
    pub halt_q6: u32,
    pub halt_modem: u32,
    pub halt_nc: u32,
    pub halt_vq6: u32,
    pub conn_box: u32,
    pub ext_bhs: u32,
    pub qaccept_mdm: u32,
    pub qaccept_cx: u32,
    pub qaccept_axi: u32,
    pub axim1_clk_off: u32,
    pub crypto_clk_off: u32,
    pub force_clk_on: u32,
    pub rscc_disable: u32,
    pub mss_restart: *mut reset_control,
    pub pdc_reset: *mut reset_control,
    pub q6v5: qcom_q6v5,
    pub active_clks: [*mut clk; 8],
    pub reset_clks: [*mut clk; 4],
    pub proxy_clks: [*mut clk; 4],
    pub proxy_pds: [*mut device; 3],
    pub active_clk_count: c_int,
    pub reset_clk_count: c_int,
    pub proxy_clk_count: c_int,
    pub proxy_pd_count: c_int,
    pub active_regs: [reg_info; 1],
    pub proxy_regs: [reg_info; 1],
    pub fallback_proxy_regs: [reg_info; 2],
    pub active_reg_count: c_int,
    pub proxy_reg_count: c_int,
    pub fallback_proxy_reg_count: c_int,
    pub dump_mba_loaded: bool,
    pub current_dump_size: usize,
    pub total_dump_size: usize,
    pub mba_phys: phys_addr_t,
    pub mba_size: usize,
    pub dp_size: usize,
    pub mdata_phys: phys_addr_t,
    pub mdata_size: usize,
    pub mpss_phys: phys_addr_t,
    pub mpss_reloc: phys_addr_t,
    pub mpss_size: usize,
    pub glink_subdev: qcom_rproc_glink,
    pub smd_subdev: qcom_rproc_subdev,
    pub pdm_subdev: qcom_rproc_pdm,
    pub ssr_subdev: qcom_rproc_ssr,
    pub sysmon: *mut qcom_sysmon,
    pub bam_dmux: *mut platform_device,
    pub need_mem_protection: bool,
    pub need_pas_mem_setup: bool,
    pub has_alt_reset: bool,
    pub has_mba_logs: bool,
    pub has_spare_reg: bool,
    pub has_qaccept_regs: bool,
    pub has_ext_bhs_reg: bool,
    pub has_ext_cntl_regs: bool,
    pub has_vq6: bool,
    pub mpss_perm: u64,
    pub mba_perm: u64,
    pub hexagon_mdt_image: *const c_char,
    pub version: c_int,
}

    enum {
    MSS_MDM9607,
    MSS_MSM8226,
    MSS_MSM8909,
    MSS_MSM8916,
    MSS_MSM8917,
    MSS_MSM8926,
    MSS_MSM8937,
    MSS_MSM8940,
    MSS_MSM8953,
    MSS_MSM8974,
    MSS_MSM8996,
    MSS_MSM8998,
    MSS_SC7180,
    MSS_SC7280,
    MSS_SDM660,
    MSS_SDM845,
    };
    static int q6v5_regulator_init(struct device *dev, struct reg_info *regs,
    const struct qcom_mss_reg_res *reg_res)
    {
    int i;
    if (!reg_res)
    return 0;
    for (i = 0; reg_res[i].supply; i++) {
    regs[i].reg = devm_regulator_get(dev, reg_res[i].supply);
    if (IS_ERR(regs[i].reg))
    return dev_err_probe(dev, PTR_ERR(regs[i].reg),
    "Failed to get %s\n regulator",
    reg_res[i].supply);
    regs[i].uV = reg_res[i].uV;
    regs[i].uA = reg_res[i].uA;
    }
    return i;
    }
    static int q6v5_regulator_enable(struct q6v5 *qproc,
    struct reg_info *regs, int count)
    {
    int ret;
    int i;
    for (i = 0; i < count; i++) {
    if (regs[i].uV > 0) {
    ret = regulator_set_voltage(regs[i].reg,
    regs[i].uV, INT_MAX);
    if (ret) {
    dev_err(qproc.dev,
    "Failed to request voltage for %d.\n",
    i);
    goto err;
    }
    }
    if (regs[i].uA > 0) {
    ret = regulator_set_load(regs[i].reg,
    regs[i].uA);
    if (ret < 0) {
    dev_err(qproc.dev,
    "Failed to set regulator mode\n");
    goto err;
    }
    }
    ret = regulator_enable(regs[i].reg);
    if (ret) {
    dev_err(qproc.dev, "Regulator enable failed\n");
    goto err;
    }
    }
    return 0;
    err:
    for (; i >= 0; i--) {
    if (regs[i].uV > 0)
    regulator_set_voltage(regs[i].reg, 0, INT_MAX);
    if (regs[i].uA > 0)
    regulator_set_load(regs[i].reg, 0);
    regulator_disable(regs[i].reg);
    }
    return ret;
    }
    static void q6v5_regulator_disable(struct q6v5 *qproc,
    struct reg_info *regs, int count)
    {
    int i;
    for (i = 0; i < count; i++) {
    if (regs[i].uV > 0)
    regulator_set_voltage(regs[i].reg, 0, INT_MAX);
    if (regs[i].uA > 0)
    regulator_set_load(regs[i].reg, 0);
    regulator_disable(regs[i].reg);
    }
    }
    static int q6v5_clk_enable(struct device *dev,
    struct clk **clks, int count)
    {
    int rc;
    int i;
    for (i = 0; i < count; i++) {
    rc = clk_prepare_enable(clks[i]);
    if (rc) {
    dev_err(dev, "Clock enable failed\n");
    goto err;
    }
    }
    return 0;
    err:
    for (i--; i >= 0; i--)
    clk_disable_unprepare(clks[i]);
    return rc;
    }
    static void q6v5_clk_disable(struct device *dev,
    struct clk **clks, int count)
    {
    int i;
    for (i = 0; i < count; i++)
    clk_disable_unprepare(clks[i]);
    }
    static int q6v5_pds_enable(struct q6v5 *qproc, struct device **pds,
    size_t pd_count)
    {
    int ret;
    int i;
    for (i = 0; i < pd_count; i++) {
    dev_pm_genpd_set_performance_state(pds[i], INT_MAX);
    ret = pm_runtime_get_sync(pds[i]);
    if (ret < 0) {
    pm_runtime_put_noidle(pds[i]);
    dev_pm_genpd_set_performance_state(pds[i], 0);
    goto unroll_pd_votes;
    }
    }
    return 0;
    unroll_pd_votes:
    for (i--; i >= 0; i--) {
    dev_pm_genpd_set_performance_state(pds[i], 0);
    pm_runtime_put(pds[i]);
    }
    return ret;
    }
    static void q6v5_pds_disable(struct q6v5 *qproc, struct device **pds,
    size_t pd_count)
    {
    int i;
    for (i = 0; i < pd_count; i++) {
    dev_pm_genpd_set_performance_state(pds[i], 0);
    pm_runtime_put(pds[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn q6v5_external_bhs_enable(qproc: *mut q6v5) -> c_int {
    static int q6v5_external_bhs_enable(struct q6v5 *qproc)
    {
    u32 val;
    let mut ret: c_int = 0;
//
// Enable external power block headswitch and wait for it to
// stabilize
//
    regmap_set_bits(qproc.conn_map, qproc.ext_bhs, EXTERNAL_BHS_ON);
    ret = regmap_read_poll_timeout(qproc.conn_map, qproc.ext_bhs,
    val, val & EXTERNAL_BHS_STATUS,
    1, EXTERNAL_BHS_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "External BHS timed out\n");
    ret = -ETIMEDOUT;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_external_bhs_disable(qproc: *mut q6v5) {
    static void q6v5_external_bhs_disable(struct q6v5 *qproc)
    {
    regmap_clear_bits(qproc.conn_map, qproc.ext_bhs, EXTERNAL_BHS_ON);
    }
    static int q6v5_xfer_mem_ownership(struct q6v5 *qproc, u64 *current_perm,
    bool local, bool remote, phys_addr_t addr,
    size_t size)
    {
    struct qcom_scm_vmperm next[2];
    let mut perms: c_int = 0;
    if (!qproc.need_mem_protection)
    return 0;
    if (local == !!(*current_perm & BIT(QCOM_SCM_VMID_HLOS)) &&
    remote == !!(*current_perm & BIT(QCOM_SCM_VMID_MSS_MSA)))
    return 0;
    if (local) {
    next[perms].vmid = QCOM_SCM_VMID_HLOS;
    next[perms].perm = QCOM_SCM_PERM_RWX;
    perms++;
    }
    if (remote) {
    next[perms].vmid = QCOM_SCM_VMID_MSS_MSA;
    next[perms].perm = QCOM_SCM_PERM_RW;
    perms++;
    }
    return qcom_scm_assign_mem(addr, ALIGN(size, SZ_4K),
    current_perm, next, perms);
    }
#[no_mangle]
unsafe extern "C" fn q6v5_debug_policy_load(qproc: *mut q6v5, mba_region: *mut c_void) {
    static void q6v5_debug_policy_load(struct q6v5 *qproc, void *mba_region)
    {
    const struct firmware *dp_fw;
    if (request_firmware_direct(&dp_fw, "msadp", qproc.dev))
    return;
    if (SZ_1M + dp_fw.size <= qproc.mba_size) {
    memcpy(mba_region + SZ_1M, dp_fw.data, dp_fw.size);
    qproc.dp_size = dp_fw.size;
    }
    release_firmware(dp_fw);
    }
pub const MSM8974_B00_OFFSET: c_uint = 0x1000;
#[no_mangle]
unsafe extern "C" fn q6v5_load(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int q6v5_load(struct rproc *rproc, const struct firmware *fw)
    {
    struct q6v5 *qproc = rproc.priv;
    void *mba_region;
// MBA is restricted to a maximum size of 1M
    if (fw.size > qproc.mba_size || fw.size > SZ_1M) {
    dev_err(qproc.dev, "MBA firmware load failed\n");
    return -EINVAL;
    }
    mba_region = memremap(qproc.mba_phys, qproc.mba_size, MEMREMAP_WC);
    if (!mba_region) {
    dev_err(qproc.dev, "unable to map memory region: %pa+%zx\n",
    &qproc.mba_phys, qproc.mba_size);
    return -EBUSY;
    }
    if ((qproc.version == MSS_MSM8974 ||
    qproc.version == MSS_MSM8226 ||
    qproc.version == MSS_MSM8926) &&
    fw.size > MSM8974_B00_OFFSET &&
    !memcmp(fw.data, ELFMAG, SELFMAG))
    memcpy(mba_region, fw.data + MSM8974_B00_OFFSET, fw.size - MSM8974_B00_OFFSET);
    else
    memcpy(mba_region, fw.data, fw.size);
    q6v5_debug_policy_load(qproc, mba_region);
    memunmap(mba_region);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_reset_assert(qproc: *mut q6v5) -> c_int {
    static int q6v5_reset_assert(struct q6v5 *qproc)
    {
    int ret;
    if (qproc.has_alt_reset) {
    reset_control_assert(qproc.pdc_reset);
    ret = reset_control_reset(qproc.mss_restart);
    reset_control_deassert(qproc.pdc_reset);
    } else if (qproc.has_spare_reg) {
//
// When the AXI pipeline is being reset with the Q6 modem partly
// operational there is possibility of AXI valid signal to
// glitch, leading to spurious transactions and Q6 hangs. A work
// around is employed by asserting the AXI_GATING_VALID_OVERRIDE
// BIT before triggering Q6 MSS reset. AXI_GATING_VALID_OVERRIDE
// is withdrawn post MSS assert followed by a MSS deassert,
// while holding the PDC reset.
//
    reset_control_assert(qproc.pdc_reset);
    regmap_update_bits(qproc.conn_map, qproc.conn_box,
    AXI_GATING_VALID_OVERRIDE, 1);
    reset_control_assert(qproc.mss_restart);
    reset_control_deassert(qproc.pdc_reset);
    regmap_update_bits(qproc.conn_map, qproc.conn_box,
    AXI_GATING_VALID_OVERRIDE, 0);
    ret = reset_control_deassert(qproc.mss_restart);
    } else if (qproc.has_ext_cntl_regs) {
    regmap_write(qproc.conn_map, qproc.rscc_disable, 0);
    reset_control_assert(qproc.pdc_reset);
    reset_control_assert(qproc.mss_restart);
    reset_control_deassert(qproc.pdc_reset);
    ret = reset_control_deassert(qproc.mss_restart);
    } else {
    ret = reset_control_assert(qproc.mss_restart);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_reset_deassert(qproc: *mut q6v5) -> c_int {
    static int q6v5_reset_deassert(struct q6v5 *qproc)
    {
    int ret;
    if (qproc.has_alt_reset) {
    reset_control_assert(qproc.pdc_reset);
    writel(1, qproc.rmb_base + RMB_MBA_ALT_RESET);
    ret = reset_control_reset(qproc.mss_restart);
    writel(0, qproc.rmb_base + RMB_MBA_ALT_RESET);
    reset_control_deassert(qproc.pdc_reset);
    } else if (qproc.has_spare_reg || qproc.has_ext_cntl_regs) {
    ret = reset_control_reset(qproc.mss_restart);
    } else {
    ret = reset_control_deassert(qproc.mss_restart);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_rmb_pbl_wait(qproc: *mut q6v5, ms: c_int) -> c_int {
    static int q6v5_rmb_pbl_wait(struct q6v5 *qproc, int ms)
    {
    unsigned long timeout;
    s32 val;
    timeout = jiffies + msecs_to_jiffies(ms);
    for (;;) {
    val = readl(qproc.rmb_base + RMB_PBL_STATUS_REG);
    if (val)
    break;
    if (time_after(jiffies, timeout))
    return -ETIMEDOUT;
    msleep(1);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_rmb_mba_wait(qproc: *mut q6v5, status: u32, ms: c_int) -> c_int {
    static int q6v5_rmb_mba_wait(struct q6v5 *qproc, u32 status, int ms)
    {
    unsigned long timeout;
    s32 val;
    timeout = jiffies + msecs_to_jiffies(ms);
    for (;;) {
    val = readl(qproc.rmb_base + RMB_MBA_STATUS_REG);
    if (val < 0)
    break;
    if (!status && val)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(status: status && val ==) -> else {
    else if (status && val == status)
    break;
    if (time_after(jiffies, timeout))
    return -ETIMEDOUT;
    msleep(1);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_dump_mba_logs(qproc: *mut q6v5) {
    static void q6v5_dump_mba_logs(struct q6v5 *qproc)
    {
    struct rproc *rproc = qproc.rproc;
    void *data;
    void *mba_region;
    if (!qproc.has_mba_logs)
    return;
    if (q6v5_xfer_mem_ownership(qproc, &qproc.mba_perm, true, false, qproc.mba_phys,
    qproc.mba_size))
    return;
    mba_region = memremap(qproc.mba_phys, qproc.mba_size, MEMREMAP_WC);
    if (!mba_region)
    return;
    data = vmalloc(MBA_LOG_SIZE);
    if (data) {
    memcpy(data, mba_region, MBA_LOG_SIZE);
    dev_coredumpv(&rproc.dev, data, MBA_LOG_SIZE, GFP_KERNEL);
    }
    memunmap(mba_region);
    }
#[no_mangle]
unsafe extern "C" fn q6v5proc_reset(qproc: *mut q6v5) -> c_int {
    static int q6v5proc_reset(struct q6v5 *qproc)
    {
    u32 val;
    int ret;
    int i;
    if (qproc.version == MSS_SDM845) {
    val = readl(qproc.reg_base + QDSP6SS_SLEEP);
    val |= Q6SS_CBCR_CLKEN;
    writel(val, qproc.reg_base + QDSP6SS_SLEEP);
    ret = readl_poll_timeout(qproc.reg_base + QDSP6SS_SLEEP,
    val, !(val & Q6SS_CBCR_CLKOFF), 1,
    Q6SS_CBCR_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "QDSP6SS Sleep clock timed out\n");
    return -ETIMEDOUT;
    }
// De-assert QDSP6 stop core
    writel(1, qproc.reg_base + QDSP6SS_BOOT_CORE_START);
// Trigger boot FSM
    writel(1, qproc.reg_base + QDSP6SS_BOOT_CMD);
    ret = readl_poll_timeout(qproc.rmb_base + RMB_MBA_MSS_STATUS,
    val, (val & BIT(0)) != 0, 10, BOOT_FSM_TIMEOUT);
    if (ret) {
    dev_err(qproc.dev, "Boot FSM failed to complete.\n");
// Reset the modem so that boot FSM is in reset state
    q6v5_reset_deassert(qproc);
    return ret;
    }
    goto pbl_wait;
    } else if (qproc.version == MSS_SC7180 || qproc.version == MSS_SC7280) {
    val = readl(qproc.reg_base + QDSP6SS_SLEEP);
    val |= Q6SS_CBCR_CLKEN;
    writel(val, qproc.reg_base + QDSP6SS_SLEEP);
    ret = readl_poll_timeout(qproc.reg_base + QDSP6SS_SLEEP,
    val, !(val & Q6SS_CBCR_CLKOFF), 1,
    Q6SS_CBCR_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "QDSP6SS Sleep clock timed out\n");
    return -ETIMEDOUT;
    }
// Turn on the XO clock needed for PLL setup
    val = readl(qproc.reg_base + QDSP6SS_XO_CBCR);
    val |= Q6SS_CBCR_CLKEN;
    writel(val, qproc.reg_base + QDSP6SS_XO_CBCR);
    ret = readl_poll_timeout(qproc.reg_base + QDSP6SS_XO_CBCR,
    val, !(val & Q6SS_CBCR_CLKOFF), 1,
    Q6SS_CBCR_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "QDSP6SS XO clock timed out\n");
    return -ETIMEDOUT;
    }
// Configure Q6 core CBCR to auto-enable after reset sequence
    val = readl(qproc.reg_base + QDSP6SS_CORE_CBCR);
    val |= Q6SS_CBCR_CLKEN;
    writel(val, qproc.reg_base + QDSP6SS_CORE_CBCR);
// De-assert the Q6 stop core signal
    writel(1, qproc.reg_base + QDSP6SS_BOOT_CORE_START);
// Wait for 10 us for any staggering logic to settle
    usleep_range(10, 20);
// Trigger the boot FSM to start the Q6 out-of-reset sequence
    writel(1, qproc.reg_base + QDSP6SS_BOOT_CMD);
// Poll the MSS_STATUS for FSM completion
    ret = readl_poll_timeout(qproc.rmb_base + RMB_MBA_MSS_STATUS,
    val, (val & BIT(0)) != 0, 10, BOOT_FSM_TIMEOUT);
    if (ret) {
    dev_err(qproc.dev, "Boot FSM failed to complete.\n");
// Reset the modem so that boot FSM is in reset state
    q6v5_reset_deassert(qproc);
    return ret;
    }
    goto pbl_wait;
    } else if (qproc.version == MSS_MDM9607 ||
    qproc.version == MSS_MSM8909 ||
    qproc.version == MSS_MSM8917 ||
    qproc.version == MSS_MSM8937 ||
    qproc.version == MSS_MSM8940 ||
    qproc.version == MSS_MSM8953 ||
    qproc.version == MSS_MSM8996 ||
    qproc.version == MSS_MSM8998 ||
    qproc.version == MSS_SDM660) {
// Override the ACC value if required
    if (qproc.version == MSS_MDM9607 ||
    qproc.version == MSS_MSM8917 ||
    qproc.version == MSS_MSM8937 ||
    qproc.version == MSS_MSM8940)
    writel(QDSP6SS_ACC_OVERRIDE_VAL_9607,
    qproc.reg_base + QDSP6SS_STRAP_ACC);
    else if (qproc.version != MSS_MSM8909 &&
    qproc.version != MSS_MSM8953)
    writel(QDSP6SS_ACC_OVERRIDE_VAL,
    qproc.reg_base + QDSP6SS_STRAP_ACC);
// Assert resets, stop core
    val = readl(qproc.reg_base + QDSP6SS_RESET_REG);
    val |= Q6SS_CORE_ARES | Q6SS_BUS_ARES_ENABLE | Q6SS_STOP_CORE;
    writel(val, qproc.reg_base + QDSP6SS_RESET_REG);
// BHS require xo cbcr to be enabled
    val = readl(qproc.reg_base + QDSP6SS_XO_CBCR);
    val |= Q6SS_CBCR_CLKEN;
    writel(val, qproc.reg_base + QDSP6SS_XO_CBCR);
// Read CLKOFF bit to go low indicating CLK is enabled
    ret = readl_poll_timeout(qproc.reg_base + QDSP6SS_XO_CBCR,
    val, !(val & Q6SS_CBCR_CLKOFF), 1,
    Q6SS_CBCR_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev,
    "xo cbcr enabling timed out (rc:%d)\n", ret);
    return ret;
    }
// Enable power block headswitch and wait for it to stabilize
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= QDSP6v56_BHS_ON;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    udelay(1);
    if (qproc.version == MSS_SDM660) {
    ret = readl_relaxed_poll_timeout(qproc.reg_base + QDSP6V62SS_BHS_STATUS,
    i, (i & QDSP6v55_BHS_EN_REST_ACK),
    1, BHS_CHECK_MAX_LOOPS);
    if (ret == -ETIMEDOUT) {
    dev_err(qproc.dev, "BHS_EN_REST_ACK not set!\n");
    return -ETIMEDOUT;
    }
    }
// Put LDO in bypass mode
    val |= QDSP6v56_LDO_BYP;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    if (qproc.version != MSS_MSM8909) {
    int mem_pwr_ctl;
    int reverse;
// Deassert QDSP6 compiler memory clamp
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val &= ~QDSP6v56_CLAMP_QMC_MEM;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
// Deassert memory peripheral sleep and L2 memory standby
    val |= Q6SS_L2DATA_STBY_N | Q6SS_SLP_RET_N;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
// Turn on L1, L2, ETB and JU memories 1 at a time
    if (qproc.version == MSS_MSM8940 ||
    qproc.version == MSS_MSM8953 ||
    qproc.version == MSS_MSM8996) {
    mem_pwr_ctl = QDSP6SS_MEM_PWR_CTL;
    i = 19;
    reverse = 0;
    } else if (qproc.version == MSS_MDM9607 ||
    qproc.version == MSS_MSM8917 ||
    qproc.version == MSS_MSM8937) {
    mem_pwr_ctl = QDSP6SS_MEM_PWR_CTL;
    i = 19;
//
// Set first 5 bits in reverse to avoid
// "inrush current" issues.
//
    reverse = 6;
    } else {
// MSS_MSM8998, MSS_SDM660
    mem_pwr_ctl = QDSP6V6SS_MEM_PWR_CTL;
    i = 28;
    reverse = 0;
    }
    val = readl(qproc.reg_base + mem_pwr_ctl);
    for (; i >= reverse; i--) {
    val |= BIT(i);
    writel(val, qproc.reg_base + mem_pwr_ctl);
    val = readl(qproc.reg_base + mem_pwr_ctl);
    udelay(1);
    }
    for (i = 0; i < reverse; i++) {
    val |= BIT(i);
    writel(val, qproc.reg_base + mem_pwr_ctl);
//
// Read back value to ensure the write is done then
// wait for 1us for both memory peripheral and data
// array to turn on.
//
    val = readl(qproc.reg_base + mem_pwr_ctl);
    udelay(1);
    }
    } else {
// Turn on memories
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_SLP_RET_N | Q6SS_L2DATA_STBY_N |
    Q6SS_ETB_SLP_NRET_N | QDSP6V55_MEM_BITS;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
// Turn on L2 banks 1 at a time
    for (i = 0; i <= 7; i++) {
    val |= BIT(i);
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    }
    }
// Remove word line clamp
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val &= ~QDSP6v56_CLAMP_WL;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    } else {
// Assert resets, stop core
    val = readl(qproc.reg_base + QDSP6SS_RESET_REG);
    val |= Q6SS_CORE_ARES | Q6SS_BUS_ARES_ENABLE | Q6SS_STOP_CORE;
    writel(val, qproc.reg_base + QDSP6SS_RESET_REG);
// Enable power block headswitch and wait for it to stabilize
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= QDSS_BHS_ON | QDSS_LDO_BYP;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    udelay(1);
//
// Turn on memories. L2 banks should be done individually
// to minimize inrush current.
//
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_SLP_RET_N | Q6SS_L2TAG_SLP_NRET_N |
    Q6SS_ETB_SLP_NRET_N | Q6SS_L2DATA_STBY_N;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_L2DATA_SLP_NRET_N_2;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_L2DATA_SLP_NRET_N_1;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_L2DATA_SLP_NRET_N_0;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    }
// Remove IO clamp
    val &= ~Q6SS_CLAMP_IO;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
// Bring core out of reset
    val = readl(qproc.reg_base + QDSP6SS_RESET_REG);
    val &= ~Q6SS_CORE_ARES;
    writel(val, qproc.reg_base + QDSP6SS_RESET_REG);
// Turn on core clock
    val = readl(qproc.reg_base + QDSP6SS_GFMUX_CTL_REG);
    val |= Q6SS_CLK_ENABLE;
    writel(val, qproc.reg_base + QDSP6SS_GFMUX_CTL_REG);
// Start core execution
    val = readl(qproc.reg_base + QDSP6SS_RESET_REG);
    val &= ~Q6SS_STOP_CORE;
    writel(val, qproc.reg_base + QDSP6SS_RESET_REG);
    pbl_wait:
// Wait for PBL status
    ret = q6v5_rmb_pbl_wait(qproc, 1000);
    if (ret == -ETIMEDOUT) {
    dev_err(qproc.dev, "PBL boot timed out\n");
    } else if (ret != RMB_PBL_SUCCESS) {
    dev_err(qproc.dev, "PBL returned unexpected status %d\n", ret);
    ret = -EINVAL;
    } else {
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5proc_enable_qchannel(qproc: *mut q6v5, map: *mut regmap, offset: u32) -> c_int {
    static int q6v5proc_enable_qchannel(struct q6v5 *qproc, struct regmap *map, u32 offset)
    {
    unsigned int val;
    int ret;
    if (!qproc.has_qaccept_regs)
    return 0;
    if (qproc.has_ext_cntl_regs) {
    regmap_write(qproc.conn_map, qproc.rscc_disable, 0);
    regmap_write(qproc.conn_map, qproc.force_clk_on, 1);
    ret = regmap_read_poll_timeout(qproc.halt_map, qproc.axim1_clk_off, val,
    !val, 1, Q6SS_CBCR_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "failed to enable axim1 clock\n");
    return -ETIMEDOUT;
    }
    }
    regmap_write(map, offset + QACCEPT_REQ_REG, 1);
// Wait for accept
    ret = regmap_read_poll_timeout(map, offset + QACCEPT_ACCEPT_REG, val, val, 5,
    QACCEPT_TIMEOUT_US);
    if (ret) {
    dev_err(qproc.dev, "qchannel enable failed\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5proc_disable_qchannel(qproc: *mut q6v5, map: *mut regmap, offset: u32) {
    static void q6v5proc_disable_qchannel(struct q6v5 *qproc, struct regmap *map, u32 offset)
    {
    int ret;
    unsigned int val, retry;
    let mut nretry: c_uint = 10;
    let mut takedown_complete: bool = false;
    if (!qproc.has_qaccept_regs)
    return;
    while (!takedown_complete && nretry) {
    nretry--;
// Wait for active transactions to complete
    regmap_read_poll_timeout(map, offset + QACCEPT_ACTIVE_REG, val, !val, 5,
    QACCEPT_TIMEOUT_US);
// Request Q-channel transaction takedown
    regmap_write(map, offset + QACCEPT_REQ_REG, 0);
//
// If the request is denied, reset the Q-channel takedown request,
// wait for active transactions to complete and retry takedown.
//
    retry = 10;
    while (retry) {
    usleep_range(5, 10);
    retry--;
    ret = regmap_read(map, offset + QACCEPT_DENY_REG, &val);
    if (!ret && val) {
    regmap_write(map, offset + QACCEPT_REQ_REG, 1);
    break;
    }
    ret = regmap_read(map, offset + QACCEPT_ACCEPT_REG, &val);
    if (!ret && !val) {
    takedown_complete = true;
    break;
    }
    }
    if (!retry)
    break;
    }
// Rely on mss_restart to clear out pending transactions on takedown failure
    if (!takedown_complete)
    dev_err(qproc.dev, "qchannel takedown failed\n");
    }
    static void q6v5proc_halt_axi_port(struct q6v5 *qproc,
    struct regmap *halt_map,
    u32 offset)
    {
    unsigned int val;
    int ret;
// Check if we're already idle
    ret = regmap_read(halt_map, offset + AXI_IDLE_REG, &val);
    if (!ret && val)
    return;
// Assert halt request
    regmap_write(halt_map, offset + AXI_HALTREQ_REG, 1);
// Wait for halt
    regmap_read_poll_timeout(halt_map, offset + AXI_HALTACK_REG, val,
    val, 1000, HALT_ACK_TIMEOUT_US);
    ret = regmap_read(halt_map, offset + AXI_IDLE_REG, &val);
    if (ret || !val)
    dev_err(qproc.dev, "port failed halt\n");
// Clear halt request (port will remain halted until reset)
    regmap_write(halt_map, offset + AXI_HALTREQ_REG, 0);
    }
    static int q6v5_mpss_init_image(struct q6v5 *qproc, const struct firmware *fw,
    const char *fw_name)
    {
    let mut dma_attrs: c_ulong = DMA_ATTR_FORCE_CONTIGUOUS;
    dma_addr_t phys;
    void *metadata;
    u64 mdata_perm;
    int xferop_ret;
    size_t size;
    void *ptr;
    int ret;
    metadata = qcom_mdt_read_metadata(fw, &size, fw_name, qproc.dev);
    if (IS_ERR(metadata))
    return PTR_ERR(metadata);
    if (qproc.mdata_phys) {
    if (size > qproc.mdata_size) {
    ret = -EINVAL;
    dev_err(qproc.dev, "metadata size outside memory range\n");
    goto free_metadata;
    }
    phys = qproc.mdata_phys;
    ptr = memremap(qproc.mdata_phys, size, MEMREMAP_WC);
    if (!ptr) {
    ret = -EBUSY;
    dev_err(qproc.dev, "unable to map memory region: %pa+%zx\n",
    &qproc.mdata_phys, size);
    goto free_metadata;
    }
    } else {
    ptr = dma_alloc_attrs(qproc.dev, size, &phys, GFP_KERNEL, dma_attrs);
    if (!ptr) {
    ret = -ENOMEM;
    dev_err(qproc.dev, "failed to allocate mdt buffer\n");
    goto free_metadata;
    }
    }
    memcpy(ptr, metadata, size);
    if (qproc.mdata_phys)
    memunmap(ptr);
// Hypervisor mapping to access metadata by modem
    mdata_perm = BIT(QCOM_SCM_VMID_HLOS);
    ret = q6v5_xfer_mem_ownership(qproc, &mdata_perm, false, true,
    phys, size);
    if (ret) {
    dev_err(qproc.dev,
    "assigning Q6 access to metadata failed: %d\n", ret);
    ret = -EAGAIN;
    goto free_dma_attrs;
    }
    writel(phys, qproc.rmb_base + RMB_PMI_META_DATA_REG);
    writel(RMB_CMD_META_DATA_READY, qproc.rmb_base + RMB_MBA_COMMAND_REG);
    ret = q6v5_rmb_mba_wait(qproc, RMB_MBA_META_DATA_AUTH_SUCCESS, 1000);
    if (ret == -ETIMEDOUT)
    dev_err(qproc.dev, "MPSS header authentication timed out\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    dev_err(qproc.dev, "MPSS header authentication failed: %d\n", ret);
// Metadata authentication done, remove modem access
    xferop_ret = q6v5_xfer_mem_ownership(qproc, &mdata_perm, true, false,
    phys, size);
    if (xferop_ret)
    dev_warn(qproc.dev,
    "mdt buffer not reclaimed system may become unstable\n");
    free_dma_attrs:
    if (!qproc.mdata_phys)
    dma_free_attrs(qproc.dev, size, ptr, phys, dma_attrs);
    free_metadata:
    kfree(metadata);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_phdr_valid(phdr: *const elf32_phdr) -> bool {
    static bool q6v5_phdr_valid(const struct elf32_phdr *phdr)
    {
    if (phdr.p_type != PT_LOAD)
    return false;
    if ((phdr.p_flags & QCOM_MDT_TYPE_MASK) == QCOM_MDT_TYPE_HASH)
    return false;
    if (!phdr.p_memsz)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_mba_load(qproc: *mut q6v5) -> c_int {
    static int q6v5_mba_load(struct q6v5 *qproc)
    {
    int ret;
    int xfermemop_ret;
    let mut mba_load_err: bool = false;
    ret = qcom_q6v5_prepare(&qproc.q6v5);
    if (ret)
    return ret;
    ret = q6v5_pds_enable(qproc, qproc.proxy_pds, qproc.proxy_pd_count);
    if (ret < 0) {
    dev_err(qproc.dev, "failed to enable proxy power domains\n");
    goto disable_irqs;
    }
    ret = q6v5_regulator_enable(qproc, qproc.fallback_proxy_regs,
    qproc.fallback_proxy_reg_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable fallback proxy supplies\n");
    goto disable_proxy_pds;
    }
    ret = q6v5_regulator_enable(qproc, qproc.proxy_regs,
    qproc.proxy_reg_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable proxy supplies\n");
    goto disable_fallback_proxy_reg;
    }
    ret = q6v5_clk_enable(qproc.dev, qproc.proxy_clks,
    qproc.proxy_clk_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable proxy clocks\n");
    goto disable_proxy_reg;
    }
    ret = q6v5_regulator_enable(qproc, qproc.active_regs,
    qproc.active_reg_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable supplies\n");
    goto disable_proxy_clk;
    }
    if (qproc.has_ext_bhs_reg) {
    ret = q6v5_external_bhs_enable(qproc);
    if (ret < 0)
    goto disable_vdd;
    }
    ret = q6v5_clk_enable(qproc.dev, qproc.reset_clks,
    qproc.reset_clk_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable reset clocks\n");
    goto disable_ext_bhs;
    }
    ret = q6v5_reset_deassert(qproc);
    if (ret) {
    dev_err(qproc.dev, "failed to deassert mss restart\n");
    goto disable_reset_clks;
    }
    ret = q6v5_clk_enable(qproc.dev, qproc.active_clks,
    qproc.active_clk_count);
    if (ret) {
    dev_err(qproc.dev, "failed to enable clocks\n");
    goto assert_reset;
    }
    ret = q6v5proc_enable_qchannel(qproc, qproc.halt_map, qproc.qaccept_axi);
    if (ret) {
    dev_err(qproc.dev, "failed to enable axi bridge\n");
    goto disable_active_clks;
    }
//
// Some versions of the MBA firmware will upon boot wipe the MPSS region as well, so provide
// the Q6 access to this region.
//
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm, false, true,
    qproc.mpss_phys, qproc.mpss_size);
    if (ret) {
    dev_err(qproc.dev, "assigning Q6 access to mpss memory failed: %d\n", ret);
    goto disable_active_clks;
    }
// Assign MBA image access in DDR to q6
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mba_perm, false, true,
    qproc.mba_phys, qproc.mba_size);
    if (ret) {
    dev_err(qproc.dev,
    "assigning Q6 access to mba memory failed: %d\n", ret);
    goto disable_active_clks;
    }
    if (qproc.has_mba_logs)
    qcom_pil_info_store("mba", qproc.mba_phys, MBA_LOG_SIZE);
    writel(qproc.mba_phys, qproc.rmb_base + RMB_MBA_IMAGE_REG);
    if (qproc.dp_size) {
    writel(qproc.mba_phys + SZ_1M, qproc.rmb_base + RMB_PMI_CODE_START_REG);
    writel(qproc.dp_size, qproc.rmb_base + RMB_PMI_CODE_LENGTH_REG);
    }
    ret = q6v5proc_reset(qproc);
    if (ret)
    goto reclaim_mba;
    ret = q6v5_rmb_mba_wait(qproc, 0, 5000);
    if (ret == -ETIMEDOUT) {
    dev_err(qproc.dev, "MBA boot timed out\n");
    goto halt_axi_ports;
    } else if (ret != RMB_MBA_XPU_UNLOCKED &&
    ret != RMB_MBA_XPU_UNLOCKED_SCRIBBLED) {
    dev_err(qproc.dev, "MBA returned unexpected status %d\n", ret);
    ret = -EINVAL;
    goto halt_axi_ports;
    }
    qproc.dump_mba_loaded = true;
    return 0;
    halt_axi_ports:
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_q6);
    if (qproc.has_vq6)
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_vq6);
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_modem);
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_nc);
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_mdm);
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_cx);
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_axi);
    mba_load_err = true;
    reclaim_mba:
    xfermemop_ret = q6v5_xfer_mem_ownership(qproc, &qproc.mba_perm, true,
    false, qproc.mba_phys,
    qproc.mba_size);
    if (xfermemop_ret) {
    dev_err(qproc.dev,
    "Failed to reclaim mba buffer, system may become unstable\n");
    } else if (mba_load_err) {
    q6v5_dump_mba_logs(qproc);
    }
    disable_active_clks:
    q6v5_clk_disable(qproc.dev, qproc.active_clks,
    qproc.active_clk_count);
    assert_reset:
    q6v5_reset_assert(qproc);
    disable_reset_clks:
    q6v5_clk_disable(qproc.dev, qproc.reset_clks,
    qproc.reset_clk_count);
    disable_ext_bhs:
    if (qproc.has_ext_bhs_reg)
    q6v5_external_bhs_disable(qproc);
    disable_vdd:
    q6v5_regulator_disable(qproc, qproc.active_regs,
    qproc.active_reg_count);
    disable_proxy_clk:
    q6v5_clk_disable(qproc.dev, qproc.proxy_clks,
    qproc.proxy_clk_count);
    disable_proxy_reg:
    q6v5_regulator_disable(qproc, qproc.proxy_regs,
    qproc.proxy_reg_count);
    disable_fallback_proxy_reg:
    q6v5_regulator_disable(qproc, qproc.fallback_proxy_regs,
    qproc.fallback_proxy_reg_count);
    disable_proxy_pds:
    q6v5_pds_disable(qproc, qproc.proxy_pds, qproc.proxy_pd_count);
    disable_irqs:
    qcom_q6v5_unprepare(&qproc.q6v5);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_mba_reclaim(qproc: *mut q6v5) {
    static void q6v5_mba_reclaim(struct q6v5 *qproc)
    {
    int ret;
    u32 val;
    qproc.dump_mba_loaded = false;
    qproc.dp_size = 0;
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_q6);
    if (qproc.has_vq6)
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_vq6);
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_modem);
    q6v5proc_halt_axi_port(qproc, qproc.halt_map, qproc.halt_nc);
    if (qproc.version == MSS_MSM8996) {
//
// To avoid high MX current during LPASS/MSS restart.
//
    val = readl(qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    val |= Q6SS_CLAMP_IO | QDSP6v56_CLAMP_WL |
    QDSP6v56_CLAMP_QMC_MEM;
    writel(val, qproc.reg_base + QDSP6SS_PWR_CTL_REG);
    }
    if (qproc.has_ext_cntl_regs) {
    regmap_write(qproc.conn_map, qproc.rscc_disable, 1);
    ret = regmap_read_poll_timeout(qproc.halt_map, qproc.axim1_clk_off, val,
    !val, 1, Q6SS_CBCR_TIMEOUT_US);
    if (ret)
    dev_err(qproc.dev, "failed to enable axim1 clock\n");
    ret = regmap_read_poll_timeout(qproc.halt_map, qproc.crypto_clk_off, val,
    !val, 1, Q6SS_CBCR_TIMEOUT_US);
    if (ret)
    dev_err(qproc.dev, "failed to enable crypto clock\n");
    }
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_mdm);
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_cx);
    q6v5proc_disable_qchannel(qproc, qproc.halt_map, qproc.qaccept_axi);
    q6v5_reset_assert(qproc);
    q6v5_clk_disable(qproc.dev, qproc.reset_clks,
    qproc.reset_clk_count);
    q6v5_clk_disable(qproc.dev, qproc.active_clks,
    qproc.active_clk_count);
    if (qproc.has_ext_bhs_reg)
    q6v5_external_bhs_disable(qproc);
    q6v5_regulator_disable(qproc, qproc.active_regs,
    qproc.active_reg_count);
// In case of failure or coredump scenario where reclaiming MBA memory
// could not happen reclaim it here.
//
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mba_perm, true, false,
    qproc.mba_phys,
    qproc.mba_size);
    WARN_ON(ret);
    ret = qcom_q6v5_unprepare(&qproc.q6v5);
    if (ret) {
    q6v5_pds_disable(qproc, qproc.proxy_pds,
    qproc.proxy_pd_count);
    q6v5_clk_disable(qproc.dev, qproc.proxy_clks,
    qproc.proxy_clk_count);
    q6v5_regulator_disable(qproc, qproc.fallback_proxy_regs,
    qproc.fallback_proxy_reg_count);
    q6v5_regulator_disable(qproc, qproc.proxy_regs,
    qproc.proxy_reg_count);
    }
    }
#[no_mangle]
unsafe extern "C" fn q6v5_reload_mba(rproc: *mut rproc) -> c_int {
    static int q6v5_reload_mba(struct rproc *rproc)
    {
    struct q6v5 *qproc = rproc.priv;
    const struct firmware *fw;
    int ret;
    ret = request_firmware(&fw, rproc.firmware, qproc.dev);
    if (ret < 0)
    return ret;
    q6v5_load(rproc, fw);
    ret = q6v5_mba_load(qproc);
    release_firmware(fw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_mpss_load(qproc: *mut q6v5) -> c_int {
    static int q6v5_mpss_load(struct q6v5 *qproc)
    {
    const struct elf32_phdr *phdrs;
    const struct elf32_phdr *phdr;
    const struct firmware *seg_fw;
    const struct firmware *fw;
    struct elf32_hdr *ehdr;
    phys_addr_t mpss_reloc;
    phys_addr_t boot_addr;
    let mut min_addr: phys_addr_t = PHYS_ADDR_MAX;
    let mut max_addr: phys_addr_t = 0;
    u32 code_length;
    let mut relocate: bool = false;
    char *fw_name;
    size_t fw_name_len;
    ssize_t offset;
    let mut size: usize = 0;
    void *ptr;
    int ret;
    int i;
    fw_name_len = strlen(qproc.hexagon_mdt_image);
    if (fw_name_len <= 4)
    return -EINVAL;
    fw_name = kstrdup(qproc.hexagon_mdt_image, GFP_KERNEL);
    if (!fw_name)
    return -ENOMEM;
    ret = request_firmware(&fw, fw_name, qproc.dev);
    if (ret < 0) {
    dev_err(qproc.dev, "unable to load %s\n", fw_name);
    goto out;
    }
// Initialize the RMB validator
    writel(0, qproc.rmb_base + RMB_PMI_CODE_LENGTH_REG);
    ret = q6v5_mpss_init_image(qproc, fw, qproc.hexagon_mdt_image);
    if (ret)
    goto release_firmware;
    ehdr = (struct elf32_hdr *)fw.data;
    phdrs = (struct elf32_phdr *)(ehdr + 1);
    for (i = 0; i < ehdr.e_phnum; i++) {
    phdr = &phdrs[i];
    if (!q6v5_phdr_valid(phdr))
    continue;
    if (phdr.p_flags & QCOM_MDT_RELOCATABLE)
    relocate = true;
    if (phdr.p_paddr < min_addr)
    min_addr = phdr.p_paddr;
    if (phdr.p_paddr + phdr.p_memsz > max_addr)
    max_addr = ALIGN(phdr.p_paddr + phdr.p_memsz, SZ_4K);
    }
    if (qproc.need_pas_mem_setup) {
    ret = qcom_pas_mem_setup(MPSS_PAS_ID, qproc.mpss_phys, qproc.mpss_size);
    if (ret) {
    dev_err(qproc.dev,
    "setting up mpss memory failed: %d\n", ret);
    goto release_firmware;
    }
    }
//
// In case of a modem subsystem restart on secure devices, the modem
// memory can be reclaimed only after MBA is loaded.
//
    q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm, true, false,
    qproc.mpss_phys, qproc.mpss_size);
// Share ownership between Linux and MSS, during segment loading
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm, true, true,
    qproc.mpss_phys, qproc.mpss_size);
    if (ret) {
    dev_err(qproc.dev,
    "assigning Q6 access to mpss memory failed: %d\n", ret);
    ret = -EAGAIN;
    goto release_firmware;
    }
    mpss_reloc = relocate ? min_addr : qproc.mpss_phys;
    qproc.mpss_reloc = mpss_reloc;
// Load firmware segments
    for (i = 0; i < ehdr.e_phnum; i++) {
    phdr = &phdrs[i];
    if (!q6v5_phdr_valid(phdr))
    continue;
    offset = phdr.p_paddr - mpss_reloc;
    if (offset < 0 || offset + phdr.p_memsz > qproc.mpss_size) {
    dev_err(qproc.dev, "segment outside memory range\n");
    ret = -EINVAL;
    goto release_firmware;
    }
    if (phdr.p_filesz > phdr.p_memsz) {
    dev_err(qproc.dev,
    "refusing to load segment %d with p_filesz > p_memsz\n",
    i);
    ret = -EINVAL;
    goto release_firmware;
    }
    ptr = memremap(qproc.mpss_phys + offset, phdr.p_memsz, MEMREMAP_WC);
    if (!ptr) {
    dev_err(qproc.dev,
    "unable to map memory region: %pa+%zx-%x\n",
    &qproc.mpss_phys, offset, phdr.p_memsz);
    goto release_firmware;
    }
    if (phdr.p_filesz && phdr.p_offset < fw.size) {
// Firmware is large enough to be non-split
    if (phdr.p_offset + phdr.p_filesz > fw.size) {
    dev_err(qproc.dev,
    "failed to load segment %d from truncated file %s\n",
    i, fw_name);
    ret = -EINVAL;
    memunmap(ptr);
    goto release_firmware;
    }
    memcpy(ptr, fw.data + phdr.p_offset, phdr.p_filesz);
    } else if (phdr.p_filesz) {
// Replace "xxx.xxx" with "xxx.bxx"
    sprintf(fw_name + fw_name_len - 3, "b%02d", i);
    ret = request_firmware_into_buf(&seg_fw, fw_name, qproc.dev,
    ptr, phdr.p_filesz);
    if (ret) {
    dev_err(qproc.dev, "failed to load %s\n", fw_name);
    memunmap(ptr);
    goto release_firmware;
    }
    if (seg_fw.size != phdr.p_filesz) {
    dev_err(qproc.dev,
    "failed to load segment %d from truncated file %s\n",
    i, fw_name);
    ret = -EINVAL;
    release_firmware(seg_fw);
    memunmap(ptr);
    goto release_firmware;
    }
    release_firmware(seg_fw);
    }
    if (phdr.p_memsz > phdr.p_filesz) {
    memset(ptr + phdr.p_filesz, 0,
    phdr.p_memsz - phdr.p_filesz);
    }
    memunmap(ptr);
    size += phdr.p_memsz;
    code_length = readl(qproc.rmb_base + RMB_PMI_CODE_LENGTH_REG);
    if (!code_length) {
    boot_addr = relocate ? qproc.mpss_phys : min_addr;
    writel(boot_addr, qproc.rmb_base + RMB_PMI_CODE_START_REG);
    writel(RMB_CMD_LOAD_READY, qproc.rmb_base + RMB_MBA_COMMAND_REG);
    }
    writel(size, qproc.rmb_base + RMB_PMI_CODE_LENGTH_REG);
    ret = readl(qproc.rmb_base + RMB_MBA_STATUS_REG);
    if (ret < 0) {
    dev_err(qproc.dev, "MPSS authentication failed: %d\n",
    ret);
    goto release_firmware;
    }
    }
// Transfer ownership of modem ddr region to q6
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm, false, true,
    qproc.mpss_phys, qproc.mpss_size);
    if (ret) {
    dev_err(qproc.dev,
    "assigning Q6 access to mpss memory failed: %d\n", ret);
    ret = -EAGAIN;
    goto release_firmware;
    }
    ret = q6v5_rmb_mba_wait(qproc, RMB_MBA_AUTH_COMPLETE, 10000);
    if (ret == -ETIMEDOUT)
    dev_err(qproc.dev, "MPSS authentication timed out\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    dev_err(qproc.dev, "MPSS authentication failed: %d\n", ret);
    qcom_pil_info_store("modem", qproc.mpss_phys, qproc.mpss_size);
    release_firmware:
    release_firmware(fw);
    out:
    kfree(fw_name);
    return ret < 0 ? ret : 0;
    }
    static void qcom_q6v5_dump_segment(struct rproc *rproc,
    struct rproc_dump_segment *segment,
    void *dest, size_t cp_offset, size_t size)
    {
    let mut ret: c_int = 0;
    struct q6v5 *qproc = rproc.priv;
    let mut offset: c_int = segment.da - qproc.mpss_reloc;
    void *ptr = core::ptr::null_mut();
// Unlock mba before copying segments
    if (!qproc.dump_mba_loaded) {
    ret = q6v5_reload_mba(rproc);
    if (!ret) {
// Reset ownership back to Linux to copy segments
    ret = q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm,
    true, false,
    qproc.mpss_phys,
    qproc.mpss_size);
    }
    }
    if (!ret)
    ptr = memremap(qproc.mpss_phys + offset + cp_offset, size, MEMREMAP_WC);
    if (ptr) {
    memcpy(dest, ptr, size);
    memunmap(ptr);
    } else {
    memset(dest, 0xff, size);
    }
    qproc.current_dump_size += size;
// Reclaim mba after copying segments
    if (qproc.current_dump_size == qproc.total_dump_size) {
    if (qproc.dump_mba_loaded) {
// Try to reset ownership back to Q6
    q6v5_xfer_mem_ownership(qproc, &qproc.mpss_perm,
    false, true,
    qproc.mpss_phys,
    qproc.mpss_size);
    q6v5_mba_reclaim(qproc);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn q6v5_start(rproc: *mut rproc) -> c_int {
    static int q6v5_start(struct rproc *rproc)
    {
    struct q6v5 *qproc = rproc.priv;
    int xfermemop_ret;
    int ret;
    ret = q6v5_mba_load(qproc);
    if (ret)
    return ret;
    dev_info(qproc.dev, "MBA booted with%s debug policy, loading mpss\n",
    qproc.dp_size ? "" : "out");
    ret = q6v5_mpss_load(qproc);
    if (ret)
    goto reclaim_mpss;
    ret = qcom_q6v5_wait_for_start(&qproc.q6v5, msecs_to_jiffies(5000));
    if (ret == -ETIMEDOUT) {
    dev_err(qproc.dev, "start timed out\n");
    goto reclaim_mpss;
    }
    xfermemop_ret = q6v5_xfer_mem_ownership(qproc, &qproc.mba_perm, true,
    false, qproc.mba_phys,
    qproc.mba_size);
    if (xfermemop_ret)
    dev_err(qproc.dev,
    "Failed to reclaim mba buffer system may become unstable\n");
// Reset Dump Segment Mask
    qproc.current_dump_size = 0;
    return 0;
    reclaim_mpss:
    q6v5_mba_reclaim(qproc);
    q6v5_dump_mba_logs(qproc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_stop(rproc: *mut rproc) -> c_int {
    static int q6v5_stop(struct rproc *rproc)
    {
    struct q6v5 *qproc = rproc.priv;
    int ret;
    ret = qcom_q6v5_request_stop(&qproc.q6v5, qproc.sysmon);
    if (ret == -ETIMEDOUT)
    dev_err(qproc.dev, "timed out on wait\n");
    q6v5_mba_reclaim(qproc);
    return 0;
    }
    static int qcom_q6v5_register_dump_segments(struct rproc *rproc,
    const struct firmware *mba_fw)
    {
    const struct firmware *fw;
    const struct elf32_phdr *phdrs;
    const struct elf32_phdr *phdr;
    const struct elf32_hdr *ehdr;
    struct q6v5 *qproc = rproc.priv;
    unsigned long i;
    int ret;
    ret = request_firmware(&fw, qproc.hexagon_mdt_image, qproc.dev);
    if (ret < 0) {
    dev_err(qproc.dev, "unable to load %s\n",
    qproc.hexagon_mdt_image);
    return ret;
    }
    rproc_coredump_set_elf_info(rproc, ELFCLASS32, EM_NONE);
    ehdr = (struct elf32_hdr *)fw.data;
    phdrs = (struct elf32_phdr *)(ehdr + 1);
    qproc.total_dump_size = 0;
    for (i = 0; i < ehdr.e_phnum; i++) {
    phdr = &phdrs[i];
    if (!q6v5_phdr_valid(phdr))
    continue;
    ret = rproc_coredump_add_custom_segment(rproc, phdr.p_paddr,
    phdr.p_memsz,
    qcom_q6v5_dump_segment,
    core::ptr::null_mut());
    if (ret)
    break;
    qproc.total_dump_size += phdr.p_memsz;
    }
    release_firmware(fw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_panic(rproc: *mut rproc) -> c_ulong {
    static unsigned long q6v5_panic(struct rproc *rproc)
    {
    struct q6v5 *qproc = rproc.priv;
    return qcom_q6v5_panic(&qproc.q6v5);
    }
    static const struct rproc_ops q6v5_ops = {
    .start = q6v5_start,
    .stop = q6v5_stop,
    .parse_fw = qcom_q6v5_register_dump_segments,
    .load = q6v5_load,
    .panic = q6v5_panic,
    };
#[no_mangle]
unsafe extern "C" fn qcom_msa_handover(q6v5: *mut qcom_q6v5) {
    static void qcom_msa_handover(struct qcom_q6v5 *q6v5)
    {
    struct q6v5 *qproc = container_of(q6v5, struct q6v5, q6v5);
    q6v5_clk_disable(qproc.dev, qproc.proxy_clks,
    qproc.proxy_clk_count);
    q6v5_regulator_disable(qproc, qproc.proxy_regs,
    qproc.proxy_reg_count);
    q6v5_regulator_disable(qproc, qproc.fallback_proxy_regs,
    qproc.fallback_proxy_reg_count);
    q6v5_pds_disable(qproc, qproc.proxy_pds, qproc.proxy_pd_count);
    }
#[no_mangle]
unsafe extern "C" fn q6v5_init_mem(qproc: *mut q6v5, pdev: *mut platform_device) -> c_int {
    static int q6v5_init_mem(struct q6v5 *qproc, struct platform_device *pdev)
    {
    struct of_phandle_args args;
    let mut halt_cell_cnt: c_int = 3;
    int ret;
    qproc.reg_base = devm_platform_ioremap_resource_byname(pdev, "qdsp6");
    if (IS_ERR(qproc.reg_base))
    return PTR_ERR(qproc.reg_base);
    qproc.rmb_base = devm_platform_ioremap_resource_byname(pdev, "rmb");
    if (IS_ERR(qproc.rmb_base))
    return PTR_ERR(qproc.rmb_base);
    if (qproc.has_vq6)
    halt_cell_cnt++;
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,halt-regs", halt_cell_cnt, 0, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse qcom,halt-regs\n");
    return -EINVAL;
    }
    qproc.halt_map = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    if (IS_ERR(qproc.halt_map))
    return PTR_ERR(qproc.halt_map);
    qproc.halt_q6 = args.args[0];
    qproc.halt_modem = args.args[1];
    qproc.halt_nc = args.args[2];
    if (qproc.has_vq6)
    qproc.halt_vq6 = args.args[3];
    if (qproc.has_qaccept_regs) {
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,qaccept-regs",
    3, 0, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse qaccept-regs\n");
    return -EINVAL;
    }
    qproc.qaccept_mdm = args.args[0];
    qproc.qaccept_cx = args.args[1];
    qproc.qaccept_axi = args.args[2];
    }
    if (qproc.has_ext_bhs_reg) {
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,ext-bhs-reg",
    1, 0, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse ext-bhs-reg index 0\n");
    return -EINVAL;
    }
    qproc.conn_map = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    if (IS_ERR(qproc.conn_map))
    return PTR_ERR(qproc.conn_map);
    qproc.ext_bhs = args.args[0];
    }
    if (qproc.has_ext_cntl_regs) {
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,ext-regs",
    2, 0, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse ext-regs index 0\n");
    return -EINVAL;
    }
    qproc.conn_map = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    if (IS_ERR(qproc.conn_map))
    return PTR_ERR(qproc.conn_map);
    qproc.force_clk_on = args.args[0];
    qproc.rscc_disable = args.args[1];
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,ext-regs",
    2, 1, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse ext-regs index 1\n");
    return -EINVAL;
    }
    qproc.axim1_clk_off = args.args[0];
    qproc.crypto_clk_off = args.args[1];
    }
    if (qproc.has_spare_reg) {
    ret = of_parse_phandle_with_fixed_args(pdev.dev.of_node,
    "qcom,spare-regs",
    1, 0, &args);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse spare-regs\n");
    return -EINVAL;
    }
    qproc.conn_map = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    if (IS_ERR(qproc.conn_map))
    return PTR_ERR(qproc.conn_map);
    qproc.conn_box = args.args[0];
    }
    return 0;
    }
    static int q6v5_init_clocks(struct device *dev, struct clk **clks,
    char **clk_names)
    {
    int i;
    if (!clk_names)
    return 0;
    for (i = 0; clk_names[i]; i++) {
    clks[i] = devm_clk_get(dev, clk_names[i]);
    if (IS_ERR(clks[i]))
    return dev_err_probe(dev, PTR_ERR(clks[i]),
    "Failed to get %s clock\n",
    clk_names[i]);
    }
    return i;
    }
    static int q6v5_pds_attach(struct device *dev, struct device **devs,
    char **pd_names)
    {
    let mut num_pds: usize = 0;
    int ret;
    int i;
    if (!pd_names)
    return 0;
    while (pd_names[num_pds])
    num_pds++;
// Handle single power domain
    if (num_pds == 1 && dev.pm_domain) {
    devs[0] = dev;
    pm_runtime_enable(dev);
    return 1;
    }
    for (i = 0; i < num_pds; i++) {
    devs[i] = dev_pm_domain_attach_by_name(dev, pd_names[i]);
    if (IS_ERR_OR_NULL(devs[i])) {
    ret = PTR_ERR(devs[i]) ? : -ENODATA;
    goto unroll_attach;
    }
    }
    return num_pds;
    unroll_attach:
    for (i--; i >= 0; i--)
    dev_pm_domain_detach(devs[i], false);
    return ret;
    }
    static void q6v5_pds_detach(struct q6v5 *qproc, struct device **pds,
    size_t pd_count)
    {
    struct device *dev = qproc.dev;
    int i;
// Handle single power domain
    if (pd_count == 1 && dev.pm_domain) {
    pm_runtime_disable(dev);
    return;
    }
    for (i = 0; i < pd_count; i++)
    dev_pm_domain_detach(pds[i], false);
    }
#[no_mangle]
unsafe extern "C" fn q6v5_init_reset(qproc: *mut q6v5) -> c_int {
    static int q6v5_init_reset(struct q6v5 *qproc)
    {
    qproc.mss_restart = devm_reset_control_get_exclusive(qproc.dev,
    "mss_restart");
    if (IS_ERR(qproc.mss_restart)) {
    dev_err(qproc.dev, "failed to acquire mss restart\n");
    return PTR_ERR(qproc.mss_restart);
    }
    if (qproc.has_alt_reset || qproc.has_spare_reg || qproc.has_ext_cntl_regs) {
    qproc.pdc_reset = devm_reset_control_get_exclusive(qproc.dev,
    "pdc_reset");
    if (IS_ERR(qproc.pdc_reset)) {
    dev_err(qproc.dev, "failed to acquire pdc reset\n");
    return PTR_ERR(qproc.pdc_reset);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_alloc_memory_region(qproc: *mut q6v5) -> c_int {
    static int q6v5_alloc_memory_region(struct q6v5 *qproc)
    {
    struct device_node *child;
    struct resource res;
    int ret;
//
// In the absence of mba/mpss sub-child, extract the mba and mpss
// reserved memory regions from device's memory-region property.
//
    child = of_get_child_by_name(qproc.dev.of_node, "mba");
    if (!child) {
    ret = of_reserved_mem_region_to_resource(qproc.dev.of_node, 0, &res);
    } else {
    ret = of_reserved_mem_region_to_resource(child, 0, &res);
    of_node_put(child);
    }
    if (ret) {
    dev_err(qproc.dev, "unable to resolve mba region\n");
    return ret;
    }
    qproc.mba_phys = res.start;
    qproc.mba_size = resource_size(&res);
    if (!child) {
    ret = of_reserved_mem_region_to_resource(qproc.dev.of_node, 1, &res);
    } else {
    child = of_get_child_by_name(qproc.dev.of_node, "mpss");
    ret = of_reserved_mem_region_to_resource(child, 0, &res);
    of_node_put(child);
    }
    if (ret) {
    dev_err(qproc.dev, "unable to resolve mpss region\n");
    return ret;
    }
    qproc.mpss_phys = qproc.mpss_reloc = res.start;
    qproc.mpss_size = resource_size(&res);
    if (!child) {
    ret = of_reserved_mem_region_to_resource(qproc.dev.of_node, 2, &res);
    } else {
    child = of_get_child_by_name(qproc.dev.of_node, "metadata");
    ret = of_reserved_mem_region_to_resource(child, 0, &res);
    of_node_put(child);
    }
    if (ret)
    return 0;
    qproc.mdata_phys = res.start;
    qproc.mdata_size = resource_size(&res);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_probe(pdev: *mut platform_device) -> c_int {
    static int q6v5_probe(struct platform_device *pdev)
    {
    const struct rproc_hexagon_res *desc;
    struct device_node *node;
    struct q6v5 *qproc;
    struct rproc *rproc;
    const char *mba_image;
    int ret;
    desc = of_device_get_match_data(&pdev.dev);
    if (!desc)
    return -EINVAL;
    if (desc.need_mem_protection && !qcom_pas_is_available())
    return -EPROBE_DEFER;
    mba_image = desc.hexagon_mba_image;
    ret = of_property_read_string_index(pdev.dev.of_node, "firmware-name",
    0, &mba_image);
    if (ret < 0 && ret != -EINVAL) {
    dev_err(&pdev.dev, "unable to read mba firmware-name\n");
    return ret;
    }
    rproc = devm_rproc_alloc(&pdev.dev, pdev.name, &q6v5_ops,
    mba_image, sizeof(*qproc));
    if (!rproc) {
    dev_err(&pdev.dev, "failed to allocate rproc\n");
    return -ENOMEM;
    }
    rproc.auto_boot = false;
    rproc_coredump_set_elf_info(rproc, ELFCLASS32, EM_NONE);
    qproc = rproc.priv;
    qproc.dev = &pdev.dev;
    qproc.rproc = rproc;
    qproc.hexagon_mdt_image = "modem.mdt";
    ret = of_property_read_string_index(pdev.dev.of_node, "firmware-name",
    1, &qproc.hexagon_mdt_image);
    if (ret < 0 && ret != -EINVAL) {
    dev_err(&pdev.dev, "unable to read mpss firmware-name\n");
    return ret;
    }
    platform_set_drvdata(pdev, qproc);
    qproc.has_qaccept_regs = desc.has_qaccept_regs;
    qproc.has_ext_bhs_reg = desc.has_ext_bhs_reg;
    qproc.has_ext_cntl_regs = desc.has_ext_cntl_regs;
    qproc.has_vq6 = desc.has_vq6;
    qproc.has_spare_reg = desc.has_spare_reg;
    ret = q6v5_init_mem(qproc, pdev);
    if (ret)
    return ret;
    ret = q6v5_alloc_memory_region(qproc);
    if (ret)
    return ret;
    ret = q6v5_init_clocks(&pdev.dev, qproc.proxy_clks,
    desc.proxy_clk_names);
    if (ret < 0)
    return ret;
    qproc.proxy_clk_count = ret;
    ret = q6v5_init_clocks(&pdev.dev, qproc.reset_clks,
    desc.reset_clk_names);
    if (ret < 0)
    return ret;
    qproc.reset_clk_count = ret;
    ret = q6v5_init_clocks(&pdev.dev, qproc.active_clks,
    desc.active_clk_names);
    if (ret < 0)
    return ret;
    qproc.active_clk_count = ret;
    ret = q6v5_regulator_init(&pdev.dev, qproc.proxy_regs,
    desc.proxy_supply);
    if (ret < 0)
    return ret;
    qproc.proxy_reg_count = ret;
    ret = q6v5_regulator_init(&pdev.dev,  qproc.active_regs,
    desc.active_supply);
    if (ret < 0)
    return ret;
    qproc.active_reg_count = ret;
    ret = q6v5_pds_attach(&pdev.dev, qproc.proxy_pds,
    desc.proxy_pd_names);
// Fallback to regulators for old device trees
    if (ret == -ENODATA && desc.fallback_proxy_supply) {
    ret = q6v5_regulator_init(&pdev.dev,
    qproc.fallback_proxy_regs,
    desc.fallback_proxy_supply);
    if (ret < 0)
    return ret;
    qproc.fallback_proxy_reg_count = ret;
    } else if (ret < 0) {
    dev_err(&pdev.dev, "Failed to init power domains\n");
    return ret;
    } else {
    qproc.proxy_pd_count = ret;
    }
    qproc.has_alt_reset = desc.has_alt_reset;
    ret = q6v5_init_reset(qproc);
    if (ret)
    goto detach_proxy_pds;
    qproc.version = desc.version;
    qproc.need_mem_protection = desc.need_mem_protection;
    qproc.has_mba_logs = desc.has_mba_logs;
    ret = qcom_q6v5_init(&qproc.q6v5, pdev, rproc, MPSS_CRASH_REASON_SMEM, "modem",
    qcom_msa_handover);
    if (ret)
    goto detach_proxy_pds;
    qproc.mpss_perm = BIT(QCOM_SCM_VMID_HLOS);
    qproc.mba_perm = BIT(QCOM_SCM_VMID_HLOS);
    qcom_add_glink_subdev(rproc, &qproc.glink_subdev, "mpss");
    qcom_add_smd_subdev(rproc, &qproc.smd_subdev);
    qcom_add_pdm_subdev(rproc, &qproc.pdm_subdev);
    qcom_add_ssr_subdev(rproc, &qproc.ssr_subdev, "mpss");
    qproc.sysmon = qcom_add_sysmon_subdev(rproc, "modem", desc.ssctl_id);
    if (IS_ERR(qproc.sysmon)) {
    ret = PTR_ERR(qproc.sysmon);
    goto remove_subdevs;
    }
    ret = rproc_add(rproc);
    if (ret)
    goto remove_sysmon_subdev;
    node = of_get_compatible_child(pdev.dev.of_node, "qcom,bam-dmux");
    qproc.bam_dmux = of_platform_device_create(node, core::ptr::null_mut(), &pdev.dev);
    of_node_put(node);
    return 0;
    remove_sysmon_subdev:
    qcom_remove_sysmon_subdev(qproc.sysmon);
    remove_subdevs:
    qcom_remove_ssr_subdev(rproc, &qproc.ssr_subdev);
    qcom_remove_smd_subdev(rproc, &qproc.smd_subdev);
    qcom_remove_glink_subdev(rproc, &qproc.glink_subdev);
    detach_proxy_pds:
    q6v5_pds_detach(qproc, qproc.proxy_pds, qproc.proxy_pd_count);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_remove(pdev: *mut platform_device) {
    static void q6v5_remove(struct platform_device *pdev)
    {
    struct q6v5 *qproc = platform_get_drvdata(pdev);
    struct rproc *rproc = qproc.rproc;
    if (qproc.bam_dmux)
    of_platform_device_destroy(&qproc.bam_dmux.dev, core::ptr::null_mut());
    rproc_del(rproc);
    qcom_q6v5_deinit(&qproc.q6v5);
    qcom_remove_sysmon_subdev(qproc.sysmon);
    qcom_remove_ssr_subdev(rproc, &qproc.ssr_subdev);
    qcom_remove_pdm_subdev(rproc, &qproc.pdm_subdev);
    qcom_remove_smd_subdev(rproc, &qproc.smd_subdev);
    qcom_remove_glink_subdev(rproc, &qproc.glink_subdev);
    q6v5_pds_detach(qproc, qproc.proxy_pds, qproc.proxy_pd_count);
    }
    static const struct rproc_hexagon_res sc7180_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .reset_clk_names = (char*[]){
    "iface",
    "bus",
    "snoc_axi",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "mnoc_axi",
    "nav",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    "mx",
    "mss",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = true,
    .has_spare_reg = true,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_SC7180,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res sc7280_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_clk_names = (char*[]){
    "xo",
    "pka",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "offline",
    "snoc_axi",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    "mss",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = true,
    .has_spare_reg = false,
    .has_qaccept_regs = true,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = true,
    .has_vq6 = true,
    .version = MSS_SC7280,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res sdm660_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_clk_names = (char*[]){
    "xo",
    "qdss",
    "mem",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "gpll0_mss",
    "mnoc_axi",
    "snoc_axi",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    "mx",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_SDM660,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res sdm845_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_clk_names = (char*[]){
    "xo",
    "prng",
    core::ptr::null_mut()
    },
    .reset_clk_names = (char*[]){
    "iface",
    "snoc_axi",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "bus",
    "mem",
    "gpll0_mss",
    "mnoc_axi",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    "mx",
    "mss",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = true,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_SDM845,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8998_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_clk_names = (char*[]){
    "xo",
    "qdss",
    "mem",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "gpll0_mss",
    "mnoc_axi",
    "snoc_axi",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    "mx",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8998,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8996_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    "qdss",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    "gpll0_mss",
    "snoc_axi",
    "mnoc_axi",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "mx",
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = true,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8996,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res mdm9607_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "mx",
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MDM9607,
    .ssctl_id = 0x22,
    };
    static const struct rproc_hexagon_res msm8909_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "mx",
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8909,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8916_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .fallback_proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mx",
    .uV = 1050000,
    },
    {
    .supply = "cx",
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "mx",
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8916,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8917_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .active_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mss",
    .uV = 1050000,
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]) {
    "cx",
    "mx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8917,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8937_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .active_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mss",
    .uV = 1050000,
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]) {
    "cx",
    "mx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = true,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8937,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8940_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .active_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mss",
    .uV = 1050000,
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]) {
    "cx",
    "mx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = true,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8940,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8953_mss = {
    .hexagon_mba_image = "mba.mbn",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]) {
    "cx",
    "mx",
    "mss",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = true,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8953,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8974_mss = {
    .hexagon_mba_image = "mba.b00",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {
    .supply = "mx",
    .uV = 1050000,
    },
    {}
    },
    .fallback_proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "cx",
    .uA = 100000,
    },
    {}
    },
    .active_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mss",
    .uV = 1050000,
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8974,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8226_mss = {
    .hexagon_mba_image = "mba.b00",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {
    .supply = "mx",
    .uV = 1050000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = true,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8226,
    .ssctl_id = 0x12,
    };
    static const struct rproc_hexagon_res msm8926_mss = {
    .hexagon_mba_image = "mba.b00",
    .proxy_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "pll",
    .uA = 100000,
    },
    {
    .supply = "mx",
    .uV = 1050000,
    },
    {}
    },
    .active_supply = (struct qcom_mss_reg_res[]) {
    {
    .supply = "mss",
    .uV = 1050000,
    .uA = 100000,
    },
    {}
    },
    .proxy_clk_names = (char*[]){
    "xo",
    core::ptr::null_mut()
    },
    .active_clk_names = (char*[]){
    "iface",
    "bus",
    "mem",
    core::ptr::null_mut()
    },
    .proxy_pd_names = (char*[]){
    "cx",
    core::ptr::null_mut()
    },
    .need_mem_protection = false,
    .need_pas_mem_setup = false,
    .has_alt_reset = false,
    .has_mba_logs = false,
    .has_spare_reg = false,
    .has_qaccept_regs = false,
    .has_ext_bhs_reg = false,
    .has_ext_cntl_regs = false,
    .has_vq6 = false,
    .version = MSS_MSM8926,
    .ssctl_id = 0x12,
    };
    static const struct of_device_id q6v5_of_match[] = {
    { .compatible = "qcom,q6v5-pil", .data = &msm8916_mss },
    { .compatible = "qcom,mdm9607-mss-pil", .data = &mdm9607_mss },
    { .compatible = "qcom,msm8226-mss-pil", .data = &msm8226_mss },
    { .compatible = "qcom,msm8909-mss-pil", .data = &msm8909_mss },
    { .compatible = "qcom,msm8916-mss-pil", .data = &msm8916_mss },
    { .compatible = "qcom,msm8917-mss-pil", .data = &msm8917_mss },
    { .compatible = "qcom,msm8926-mss-pil", .data = &msm8926_mss },
    { .compatible = "qcom,msm8937-mss-pil", .data = &msm8937_mss },
    { .compatible = "qcom,msm8940-mss-pil", .data = &msm8940_mss },
    { .compatible = "qcom,msm8953-mss-pil", .data = &msm8953_mss },
    { .compatible = "qcom,msm8974-mss-pil", .data = &msm8974_mss },
    { .compatible = "qcom,msm8996-mss-pil", .data = &msm8996_mss },
    { .compatible = "qcom,msm8998-mss-pil", .data = &msm8998_mss },
    { .compatible = "qcom,sc7180-mss-pil", .data = &sc7180_mss },
    { .compatible = "qcom,sc7280-mss-pil", .data = &sc7280_mss },
    { .compatible = "qcom,sdm660-mss-pil", .data = &sdm660_mss },
    { .compatible = "qcom,sdm845-mss-pil", .data = &sdm845_mss },
    { },
    };
    MODULE_DEVICE_TABLE(of, q6v5_of_match);
    static struct platform_driver q6v5_driver = {
    .probe = q6v5_probe,
    .remove = q6v5_remove,
    .driver = {
    .name = "qcom-q6v5-mss",
    .of_match_table = q6v5_of_match,
    },
    };
    module_platform_driver(q6v5_driver);
    MODULE_DESCRIPTION("Qualcomm Self-authenticating modem remoteproc driver");
    MODULE_LICENSE("GPL v2");
