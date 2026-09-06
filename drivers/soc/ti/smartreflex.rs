//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ti/smartreflex.c
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
// OMAP SmartReflex Voltage Control
//
// Author: Thara Gopinath	<thara@ti.com>
//
// Copyright (C) 2012 Texas Instruments, Inc.
// Thara Gopinath <thara@ti.com>
//
// Copyright (C) 2008 Nokia Corporation
// Kalle Jokiniemi
//
// Copyright (C) 2007 Texas Instruments, Inc.
// Lesly A M <x0080970@ti.com>
//

pub const SMARTREFLEX_NAME_LEN: c_int = 32;
pub const NVALUE_NAME_LEN: c_int = 40;
pub const SR_DISABLE_TIMEOUT: c_int = 200;
// sr_list contains all the instances of smartreflex module
    static LIST_HEAD(sr_list);
    static struct omap_sr_class_data *sr_class;
    static struct dentry		*sr_dbg_dir;
#[no_mangle]
pub unsafe extern "C" fn sr_write_reg(sr: *mut omap_sr, offset: unsigned, value: u32) {
    static inline void sr_write_reg(struct omap_sr *sr, unsigned offset, u32 value)
    {
    __raw_writel(value, (sr.base + offset));
    }
    static inline void sr_modify_reg(struct omap_sr *sr, unsigned offset, u32 mask,
    u32 value)
    {
    u32 reg_val;
//
// Smartreflex error config register is special as it contains
// certain status bits which if written a 1 into means a clear
// of those bits. So in order to make sure no accidental write of
// 1 happens to those status bits, do a clear of them in the read
// value. This mean this API doesn't rewrite values in these bits
// if they are currently set, but does allow the caller to write
// those bits.
//
    if (sr.ip_type == SR_TYPE_V1 && offset == ERRCONFIG_V1)
    mask |= ERRCONFIG_STATUS_V1_MASK;
#[no_mangle]
pub unsafe extern "C" fn if(ERRCONFIG_V2: sr->ip_type == SR_TYPE_V2 && offset ==) -> else {
    else if (sr.ip_type == SR_TYPE_V2 && offset == ERRCONFIG_V2)
    mask |= ERRCONFIG_VPBOUNDINTST_V2;
    reg_val = __raw_readl(sr.base + offset);
    reg_val &= ~mask;
    value &= mask;
    reg_val |= value;
    __raw_writel(reg_val, (sr.base + offset));
    }
#[no_mangle]
pub unsafe extern "C" fn sr_read_reg(sr: *mut omap_sr, offset: unsigned) -> u32 {
    static inline u32 sr_read_reg(struct omap_sr *sr, unsigned offset)
    {
    return __raw_readl(sr.base + offset);
    }
    static struct omap_sr *_sr_lookup(struct voltagedomain *voltdm)
    {
    struct omap_sr *sr_info;
    if (!voltdm) {
    pr_err("%s: Null voltage domain passed!\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    list_for_each_entry(sr_info, &sr_list, node) {
    if (voltdm == sr_info.voltdm)
    return sr_info;
    }
    return ERR_PTR(-ENODATA);
    }
#[no_mangle]
unsafe extern "C" fn sr_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sr_interrupt(int irq, void *data)
    {
    struct omap_sr *sr_info = data;
    let mut status: u32 = 0;
    switch (sr_info.ip_type) {
    case SR_TYPE_V1:
// Read the status bits
    status = sr_read_reg(sr_info, ERRCONFIG_V1);
// Clear them by writing back
    sr_write_reg(sr_info, ERRCONFIG_V1, status);
    break;
    case SR_TYPE_V2:
// Read the status bits
    status = sr_read_reg(sr_info, IRQSTATUS);
// Clear them by writing back
    sr_write_reg(sr_info, IRQSTATUS, status);
    break;
    default:
    dev_err(&sr_info.pdev.dev, "UNKNOWN IP type %d\n",
    sr_info.ip_type);
    return IRQ_NONE;
    }
    if (sr_class.notify)
    sr_class.notify(sr_info, status);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sr_set_clk_length(sr: *mut omap_sr) {
    static void sr_set_clk_length(struct omap_sr *sr)
    {
    u32 fclk_speed;
// Try interconnect target module fck first if it already exists
    if (IS_ERR(sr.fck))
    return;
    fclk_speed = clk_get_rate(sr.fck);
    switch (fclk_speed) {
    case 12000000:
    sr.clk_length = SRCLKLENGTH_12MHZ_SYSCLK;
    break;
    case 13000000:
    sr.clk_length = SRCLKLENGTH_13MHZ_SYSCLK;
    break;
    case 19200000:
    sr.clk_length = SRCLKLENGTH_19MHZ_SYSCLK;
    break;
    case 26000000:
    sr.clk_length = SRCLKLENGTH_26MHZ_SYSCLK;
    break;
    case 38400000:
    sr.clk_length = SRCLKLENGTH_38MHZ_SYSCLK;
    break;
    default:
    dev_err(&sr.pdev.dev, "%s: Invalid fclk rate: %d\n",
    __func__, fclk_speed);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn sr_start_vddautocomp(sr: *mut omap_sr) {
    static void sr_start_vddautocomp(struct omap_sr *sr)
    {
    if (!sr_class || !(sr_class.enable) || !(sr_class.configure)) {
    dev_warn(&sr.pdev.dev,
    "%s: smartreflex class driver not registered\n",
    __func__);
    return;
    }
    if (!sr_class.enable(sr))
    sr.autocomp_active = true;
    }
#[no_mangle]
unsafe extern "C" fn sr_stop_vddautocomp(sr: *mut omap_sr) {
    static void sr_stop_vddautocomp(struct omap_sr *sr)
    {
    if (!sr_class || !(sr_class.disable)) {
    dev_warn(&sr.pdev.dev,
    "%s: smartreflex class driver not registered\n",
    __func__);
    return;
    }
    if (sr.autocomp_active) {
    sr_class.disable(sr, 1);
    sr.autocomp_active = false;
    }
    }
//
// This function handles the initializations which have to be done
// only when both sr device and class driver regiter has
// completed. This will be attempted to be called from both sr class
// driver register and sr device intializtion API's. Only one call
// will ultimately succeed.
//
// Currently this function registers interrupt handler for a particular SR
// if smartreflex class driver is already registered and has
// requested for interrupts and the SR interrupt line in present.
//
#[no_mangle]
unsafe extern "C" fn sr_late_init(sr_info: *mut omap_sr) -> c_int {
    static int sr_late_init(struct omap_sr *sr_info)
    {
    let mut ret: c_int = 0;
    if (sr_class.notify && sr_class.notify_flags && sr_info.irq) {
    ret = devm_request_irq(&sr_info.pdev.dev, sr_info.irq,
    sr_interrupt, IRQF_NO_AUTOEN,
    sr_info.name, sr_info);
    if (ret)
    goto error;
    }
    return ret;
    error:
    list_del(&sr_info.node);
    dev_err(&sr_info.pdev.dev, "%s: ERROR in registering interrupt handler. Smartreflex will not function as desired\n",
    __func__);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sr_v1_disable(sr: *mut omap_sr) {
    static void sr_v1_disable(struct omap_sr *sr)
    {
    let mut timeout: c_int = 0;
    int errconf_val = ERRCONFIG_MCUACCUMINTST | ERRCONFIG_MCUVALIDINTST |
    ERRCONFIG_MCUBOUNDINTST;
// Enable MCUDisableAcknowledge interrupt
    sr_modify_reg(sr, ERRCONFIG_V1,
    ERRCONFIG_MCUDISACKINTEN, ERRCONFIG_MCUDISACKINTEN);
// SRCONFIG - disable SR
    sr_modify_reg(sr, SRCONFIG, SRCONFIG_SRENABLE, 0x0);
// Disable all other SR interrupts and clear the status as needed
    if (sr_read_reg(sr, ERRCONFIG_V1) & ERRCONFIG_VPBOUNDINTST_V1)
    errconf_val |= ERRCONFIG_VPBOUNDINTST_V1;
    sr_modify_reg(sr, ERRCONFIG_V1,
    (ERRCONFIG_MCUACCUMINTEN | ERRCONFIG_MCUVALIDINTEN |
    ERRCONFIG_MCUBOUNDINTEN | ERRCONFIG_VPBOUNDINTEN_V1),
    errconf_val);
//
// Wait for SR to be disabled.
// wait until ERRCONFIG.MCUDISACKINTST = 1. Typical latency is 1us.
//
    sr_test_cond_timeout((sr_read_reg(sr, ERRCONFIG_V1) &
    ERRCONFIG_MCUDISACKINTST), SR_DISABLE_TIMEOUT,
    timeout);
    if (timeout >= SR_DISABLE_TIMEOUT)
    dev_warn(&sr.pdev.dev, "%s: Smartreflex disable timedout\n",
    __func__);
// Disable MCUDisableAcknowledge interrupt & clear pending interrupt
    sr_modify_reg(sr, ERRCONFIG_V1, ERRCONFIG_MCUDISACKINTEN,
    ERRCONFIG_MCUDISACKINTST);
    }
#[no_mangle]
unsafe extern "C" fn sr_v2_disable(sr: *mut omap_sr) {
    static void sr_v2_disable(struct omap_sr *sr)
    {
    let mut timeout: c_int = 0;
// Enable MCUDisableAcknowledge interrupt
    sr_write_reg(sr, IRQENABLE_SET, IRQENABLE_MCUDISABLEACKINT);
// SRCONFIG - disable SR
    sr_modify_reg(sr, SRCONFIG, SRCONFIG_SRENABLE, 0x0);
//
// Disable all other SR interrupts and clear the status
// write to status register ONLY on need basis - only if status
// is set.
//
    if (sr_read_reg(sr, ERRCONFIG_V2) & ERRCONFIG_VPBOUNDINTST_V2)
    sr_modify_reg(sr, ERRCONFIG_V2, ERRCONFIG_VPBOUNDINTEN_V2,
    ERRCONFIG_VPBOUNDINTST_V2);
    else
    sr_modify_reg(sr, ERRCONFIG_V2, ERRCONFIG_VPBOUNDINTEN_V2,
    0x0);
    sr_write_reg(sr, IRQENABLE_CLR, (IRQENABLE_MCUACCUMINT |
    IRQENABLE_MCUVALIDINT |
    IRQENABLE_MCUBOUNDSINT));
    sr_write_reg(sr, IRQSTATUS, (IRQSTATUS_MCUACCUMINT |
    IRQSTATUS_MCVALIDINT |
    IRQSTATUS_MCBOUNDSINT));
//
// Wait for SR to be disabled.
// wait until IRQSTATUS.MCUDISACKINTST = 1. Typical latency is 1us.
//
    sr_test_cond_timeout((sr_read_reg(sr, IRQSTATUS) &
    IRQSTATUS_MCUDISABLEACKINT), SR_DISABLE_TIMEOUT,
    timeout);
    if (timeout >= SR_DISABLE_TIMEOUT)
    dev_warn(&sr.pdev.dev, "%s: Smartreflex disable timedout\n",
    __func__);
// Disable MCUDisableAcknowledge interrupt & clear pending interrupt
    sr_write_reg(sr, IRQENABLE_CLR, IRQENABLE_MCUDISABLEACKINT);
    sr_write_reg(sr, IRQSTATUS, IRQSTATUS_MCUDISABLEACKINT);
    }
    static struct omap_sr_nvalue_table *sr_retrieve_nvalue_row(
    struct omap_sr *sr, u32 efuse_offs)
    {
    int i;
    if (!sr.nvalue_table) {
    dev_warn(&sr.pdev.dev, "%s: Missing ntarget value table\n",
    __func__);
    return core::ptr::null_mut();
    }
    for (i = 0; i < sr.nvalue_count; i++) {
    if (sr.nvalue_table[i].efuse_offs == efuse_offs)
    return &sr.nvalue_table[i];
    }
    return core::ptr::null_mut();
    }
// Public Functions
//
// sr_configure_errgen() - Configures the SmartReflex to perform AVS using the
// error generator module.
// @sr:			SR module to be configured.
//
// This API is to be called from the smartreflex class driver to
// configure the error generator module inside the smartreflex module.
// SR settings if using the ERROR module inside Smartreflex.
// SR CLASS 3 by default uses only the ERROR module where as
// SR CLASS 2 can choose between ERROR module and MINMAXAVG
// module. Returns 0 on success and error value in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn sr_configure_errgen(sr: *mut omap_sr) -> c_int {
    int sr_configure_errgen(struct omap_sr *sr)
    {
    u32 sr_config, sr_errconfig, errconfig_offs;
    u32 vpboundint_en, vpboundint_st;
    let mut senp_en: u32 = 0, senn_en = 0;
    u8 senp_shift, senn_shift;
    if (!sr) {
    pr_warn("%s: core::ptr::null_mut() omap_sr from %pS\n",
    __func__, (void *)_RET_IP_);
    return -EINVAL;
    }
    if (!sr.clk_length)
    sr_set_clk_length(sr);
    senp_en = sr.senp_mod;
    senn_en = sr.senn_mod;
    sr_config = (sr.clk_length << SRCONFIG_SRCLKLENGTH_SHIFT) |
    SRCONFIG_SENENABLE | SRCONFIG_ERRGEN_EN;
    switch (sr.ip_type) {
    case SR_TYPE_V1:
    sr_config |= SRCONFIG_DELAYCTRL;
    senn_shift = SRCONFIG_SENNENABLE_V1_SHIFT;
    senp_shift = SRCONFIG_SENPENABLE_V1_SHIFT;
    errconfig_offs = ERRCONFIG_V1;
    vpboundint_en = ERRCONFIG_VPBOUNDINTEN_V1;
    vpboundint_st = ERRCONFIG_VPBOUNDINTST_V1;
    break;
    case SR_TYPE_V2:
    senn_shift = SRCONFIG_SENNENABLE_V2_SHIFT;
    senp_shift = SRCONFIG_SENPENABLE_V2_SHIFT;
    errconfig_offs = ERRCONFIG_V2;
    vpboundint_en = ERRCONFIG_VPBOUNDINTEN_V2;
    vpboundint_st = ERRCONFIG_VPBOUNDINTST_V2;
    break;
    default:
    dev_err(&sr.pdev.dev, "%s: Trying to Configure smartreflex module without specifying the ip\n",
    __func__);
    return -EINVAL;
    }
    sr_config |= ((senn_en << senn_shift) | (senp_en << senp_shift));
    sr_write_reg(sr, SRCONFIG, sr_config);
    sr_errconfig = (sr.err_weight << ERRCONFIG_ERRWEIGHT_SHIFT) |
    (sr.err_maxlimit << ERRCONFIG_ERRMAXLIMIT_SHIFT) |
    (sr.err_minlimit <<  ERRCONFIG_ERRMINLIMIT_SHIFT);
    sr_modify_reg(sr, errconfig_offs, (SR_ERRWEIGHT_MASK |
    SR_ERRMAXLIMIT_MASK | SR_ERRMINLIMIT_MASK),
    sr_errconfig);
// Enabling the interrupts if the ERROR module is used
    sr_modify_reg(sr, errconfig_offs, (vpboundint_en | vpboundint_st),
    vpboundint_en);
    return 0;
    }
//
// sr_disable_errgen() - Disables SmartReflex AVS module's errgen component
// @sr:			SR module to be configured.
//
// This API is to be called from the smartreflex class driver to
// disable the error generator module inside the smartreflex module.
//
// Returns 0 on success and error value in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn sr_disable_errgen(sr: *mut omap_sr) -> c_int {
    int sr_disable_errgen(struct omap_sr *sr)
    {
    u32 errconfig_offs;
    u32 vpboundint_en, vpboundint_st;
    if (!sr) {
    pr_warn("%s: core::ptr::null_mut() omap_sr from %pS\n",
    __func__, (void *)_RET_IP_);
    return -EINVAL;
    }
    switch (sr.ip_type) {
    case SR_TYPE_V1:
    errconfig_offs = ERRCONFIG_V1;
    vpboundint_en = ERRCONFIG_VPBOUNDINTEN_V1;
    vpboundint_st = ERRCONFIG_VPBOUNDINTST_V1;
    break;
    case SR_TYPE_V2:
    errconfig_offs = ERRCONFIG_V2;
    vpboundint_en = ERRCONFIG_VPBOUNDINTEN_V2;
    vpboundint_st = ERRCONFIG_VPBOUNDINTST_V2;
    break;
    default:
    dev_err(&sr.pdev.dev, "%s: Trying to Configure smartreflex module without specifying the ip\n",
    __func__);
    return -EINVAL;
    }
// Disable the Sensor and errorgen
    sr_modify_reg(sr, SRCONFIG, SRCONFIG_SENENABLE | SRCONFIG_ERRGEN_EN, 0);
//
// Disable the interrupts of ERROR module
// NOTE: modify is a read, modify,write - an implicit OCP barrier
// which is required is present here - sequencing is critical
// at this point (after errgen is disabled, vpboundint disable)
//
    sr_modify_reg(sr, errconfig_offs, vpboundint_en | vpboundint_st, 0);
    return 0;
    }
//
// sr_configure_minmax() - Configures the SmartReflex to perform AVS using the
// minmaxavg module.
// @sr:			SR module to be configured.
//
// This API is to be called from the smartreflex class driver to
// configure the minmaxavg module inside the smartreflex module.
// SR settings if using the ERROR module inside Smartreflex.
// SR CLASS 3 by default uses only the ERROR module where as
// SR CLASS 2 can choose between ERROR module and MINMAXAVG
// module. Returns 0 on success and error value in case of failure.
//
#[no_mangle]
pub unsafe extern "C" fn sr_configure_minmax(sr: *mut omap_sr) -> c_int {
    int sr_configure_minmax(struct omap_sr *sr)
    {
    u32 sr_config, sr_avgwt;
    let mut senp_en: u32 = 0, senn_en = 0;
    u8 senp_shift, senn_shift;
    if (!sr) {
    pr_warn("%s: core::ptr::null_mut() omap_sr from %pS\n",
    __func__, (void *)_RET_IP_);
    return -EINVAL;
    }
    if (!sr.clk_length)
    sr_set_clk_length(sr);
    senp_en = sr.senp_mod;
    senn_en = sr.senn_mod;
    sr_config = (sr.clk_length << SRCONFIG_SRCLKLENGTH_SHIFT) |
    SRCONFIG_SENENABLE |
    (sr.accum_data << SRCONFIG_ACCUMDATA_SHIFT);
    switch (sr.ip_type) {
    case SR_TYPE_V1:
    sr_config |= SRCONFIG_DELAYCTRL;
    senn_shift = SRCONFIG_SENNENABLE_V1_SHIFT;
    senp_shift = SRCONFIG_SENPENABLE_V1_SHIFT;
    break;
    case SR_TYPE_V2:
    senn_shift = SRCONFIG_SENNENABLE_V2_SHIFT;
    senp_shift = SRCONFIG_SENPENABLE_V2_SHIFT;
    break;
    default:
    dev_err(&sr.pdev.dev, "%s: Trying to Configure smartreflex module without specifying the ip\n",
    __func__);
    return -EINVAL;
    }
    sr_config |= ((senn_en << senn_shift) | (senp_en << senp_shift));
    sr_write_reg(sr, SRCONFIG, sr_config);
    sr_avgwt = (sr.senp_avgweight << AVGWEIGHT_SENPAVGWEIGHT_SHIFT) |
    (sr.senn_avgweight << AVGWEIGHT_SENNAVGWEIGHT_SHIFT);
    sr_write_reg(sr, AVGWEIGHT, sr_avgwt);
//
// Enabling the interrupts if MINMAXAVG module is used.
// TODO: check if all the interrupts are mandatory
//
    switch (sr.ip_type) {
    case SR_TYPE_V1:
    sr_modify_reg(sr, ERRCONFIG_V1,
    (ERRCONFIG_MCUACCUMINTEN | ERRCONFIG_MCUVALIDINTEN |
    ERRCONFIG_MCUBOUNDINTEN),
    (ERRCONFIG_MCUACCUMINTEN | ERRCONFIG_MCUACCUMINTST |
    ERRCONFIG_MCUVALIDINTEN | ERRCONFIG_MCUVALIDINTST |
    ERRCONFIG_MCUBOUNDINTEN | ERRCONFIG_MCUBOUNDINTST));
    break;
    case SR_TYPE_V2:
    sr_write_reg(sr, IRQSTATUS,
    IRQSTATUS_MCUACCUMINT | IRQSTATUS_MCVALIDINT |
    IRQSTATUS_MCBOUNDSINT | IRQSTATUS_MCUDISABLEACKINT);
    sr_write_reg(sr, IRQENABLE_SET,
    IRQENABLE_MCUACCUMINT | IRQENABLE_MCUVALIDINT |
    IRQENABLE_MCUBOUNDSINT | IRQENABLE_MCUDISABLEACKINT);
    break;
    default:
    dev_err(&sr.pdev.dev, "%s: Trying to Configure smartreflex module without specifying the ip\n",
    __func__);
    return -EINVAL;
    }
    return 0;
    }
//
// sr_enable() - Enables the smartreflex module.
// @sr:		pointer to which the SR module to be configured belongs to.
// @volt:	The voltage at which the Voltage domain associated with
// the smartreflex module is operating at.
// This is required only to program the correct Ntarget value.
//
// This API is to be called from the smartreflex class driver to
// enable a smartreflex module. Returns 0 on success. Returns error
// value if the voltage passed is wrong or if ntarget value is wrong.
//
#[no_mangle]
pub unsafe extern "C" fn sr_enable(sr: *mut omap_sr, volt: c_ulong) -> c_int {
    int sr_enable(struct omap_sr *sr, unsigned long volt)
    {
    struct omap_volt_data *volt_data;
    struct omap_sr_nvalue_table *nvalue_row;
    int ret;
    if (!sr) {
    pr_warn("%s: core::ptr::null_mut() omap_sr from %pS\n",
    __func__, (void *)_RET_IP_);
    return -EINVAL;
    }
    volt_data = omap_voltage_get_voltdata(sr.voltdm, volt);
    if (IS_ERR(volt_data)) {
    dev_warn(&sr.pdev.dev, "%s: Unable to get voltage table for nominal voltage %ld\n",
    __func__, volt);
    return PTR_ERR(volt_data);
    }
    nvalue_row = sr_retrieve_nvalue_row(sr, volt_data.sr_efuse_offs);
    if (!nvalue_row) {
    dev_warn(&sr.pdev.dev, "%s: failure getting SR data for this voltage %ld\n",
    __func__, volt);
    return -ENODATA;
    }
// errminlimit is opp dependent and hence linked to voltage
    sr.err_minlimit = nvalue_row.errminlimit;
    clk_enable(sr.fck);
// Check if SR is already enabled. If yes do nothing
    if (sr_read_reg(sr, SRCONFIG) & SRCONFIG_SRENABLE)
    goto out_enabled;
// Configure SR
    ret = sr_class.configure(sr);
    if (ret)
    goto out_enabled;
    sr_write_reg(sr, NVALUERECIPROCAL, nvalue_row.nvalue);
// SRCONFIG - enable SR
    sr_modify_reg(sr, SRCONFIG, SRCONFIG_SRENABLE, SRCONFIG_SRENABLE);
    out_enabled:
    sr.enabled = 1;
    return 0;
    }
//
// sr_disable() - Disables the smartreflex module.
// @sr:		pointer to which the SR module to be configured belongs to.
//
// This API is to be called from the smartreflex class driver to
// disable a smartreflex module.
//
#[no_mangle]
pub unsafe extern "C" fn sr_disable(sr: *mut omap_sr) {
    void sr_disable(struct omap_sr *sr)
    {
    if (!sr) {
    pr_warn("%s: core::ptr::null_mut() omap_sr from %pS\n",
    __func__, (void *)_RET_IP_);
    return;
    }
// Check if SR clocks are already disabled. If yes do nothing
    if (!sr.enabled)
    return;
//
// Disable SR if only it is indeed enabled. Else just
// disable the clocks.
//
    if (sr_read_reg(sr, SRCONFIG) & SRCONFIG_SRENABLE) {
    switch (sr.ip_type) {
    case SR_TYPE_V1:
    sr_v1_disable(sr);
    break;
    case SR_TYPE_V2:
    sr_v2_disable(sr);
    break;
    default:
    dev_err(&sr.pdev.dev, "UNKNOWN IP type %d\n",
    sr.ip_type);
    }
    }
    clk_disable(sr.fck);
    sr.enabled = 0;
    }
//
// sr_register_class() - API to register a smartreflex class parameters.
// @class_data:	The structure containing various sr class specific data.
//
// This API is to be called by the smartreflex class driver to register itself
// with the smartreflex driver during init. Returns 0 on success else the
// error value.
//
#[no_mangle]
pub unsafe extern "C" fn sr_register_class(class_data: *mut omap_sr_class_data) -> c_int {
    int sr_register_class(struct omap_sr_class_data *class_data)
    {
    struct omap_sr *sr_info;
    if (!class_data) {
    pr_warn("%s:, Smartreflex class data passed is core::ptr::null_mut()\n",
    __func__);
    return -EINVAL;
    }
    if (sr_class) {
    pr_warn("%s: Smartreflex class driver already registered\n",
    __func__);
    return -EBUSY;
    }
    sr_class = class_data;
//
// Call into late init to do initializations that require
// both sr driver and sr class driver to be initiallized.
//
    list_for_each_entry(sr_info, &sr_list, node)
    sr_late_init(sr_info);
    return 0;
    }
//
// omap_sr_enable() -  API to enable SR clocks and to call into the
// registered smartreflex class enable API.
// @voltdm:	VDD pointer to which the SR module to be configured belongs to.
//
// This API is to be called from the kernel in order to enable
// a particular smartreflex module. This API will do the initial
// configurations to turn on the smartreflex module and in turn call
// into the registered smartreflex class enable API.
//
#[no_mangle]
pub unsafe extern "C" fn omap_sr_enable(voltdm: *mut voltagedomain) {
    void omap_sr_enable(struct voltagedomain *voltdm)
    {
    struct omap_sr *sr = _sr_lookup(voltdm);
    if (IS_ERR(sr)) {
    pr_warn("%s: omap_sr struct for voltdm not found\n", __func__);
    return;
    }
    if (!sr.autocomp_active)
    return;
    if (!sr_class || !(sr_class.enable) || !(sr_class.configure)) {
    dev_warn(&sr.pdev.dev, "%s: smartreflex class driver not registered\n",
    __func__);
    return;
    }
    sr_class.enable(sr);
    }
//
// omap_sr_disable() - API to disable SR without resetting the voltage
// processor voltage
// @voltdm:	VDD pointer to which the SR module to be configured belongs to.
//
// This API is to be called from the kernel in order to disable
// a particular smartreflex module. This API will in turn call
// into the registered smartreflex class disable API. This API will tell
// the smartreflex class disable not to reset the VP voltage after
// disabling smartreflex.
//
#[no_mangle]
pub unsafe extern "C" fn omap_sr_disable(voltdm: *mut voltagedomain) {
    void omap_sr_disable(struct voltagedomain *voltdm)
    {
    struct omap_sr *sr = _sr_lookup(voltdm);
    if (IS_ERR(sr)) {
    pr_warn("%s: omap_sr struct for voltdm not found\n", __func__);
    return;
    }
    if (!sr.autocomp_active)
    return;
    if (!sr_class || !(sr_class.disable)) {
    dev_warn(&sr.pdev.dev, "%s: smartreflex class driver not registered\n",
    __func__);
    return;
    }
    sr_class.disable(sr, 0);
    }
//
// omap_sr_disable_reset_volt() - API to disable SR and reset the
// voltage processor voltage
// @voltdm:	VDD pointer to which the SR module to be configured belongs to.
//
// This API is to be called from the kernel in order to disable
// a particular smartreflex module. This API will in turn call
// into the registered smartreflex class disable API. This API will tell
// the smartreflex class disable to reset the VP voltage after
// disabling smartreflex.
//
#[no_mangle]
pub unsafe extern "C" fn omap_sr_disable_reset_volt(voltdm: *mut voltagedomain) {
    void omap_sr_disable_reset_volt(struct voltagedomain *voltdm)
    {
    struct omap_sr *sr = _sr_lookup(voltdm);
    if (IS_ERR(sr)) {
    pr_warn("%s: omap_sr struct for voltdm not found\n", __func__);
    return;
    }
    if (!sr.autocomp_active)
    return;
    if (!sr_class || !(sr_class.disable)) {
    dev_warn(&sr.pdev.dev, "%s: smartreflex class driver not registered\n",
    __func__);
    return;
    }
    sr_class.disable(sr, 1);
    }
// PM Debug FS entries to enable and disable smartreflex.
#[no_mangle]
unsafe extern "C" fn omap_sr_autocomp_show(data: *mut c_void, val: *mut u64) -> c_int {
    static int omap_sr_autocomp_show(void *data, u64 *val)
    {
    struct omap_sr *sr_info = data;
    if (!sr_info) {
    pr_warn("%s: omap_sr struct not found\n", __func__);
    return -EINVAL;
    }
// val = sr_info->autocomp_active;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_sr_autocomp_store(data: *mut c_void, val: u64) -> c_int {
    static int omap_sr_autocomp_store(void *data, u64 val)
    {
    struct omap_sr *sr_info = data;
    if (!sr_info) {
    pr_warn("%s: omap_sr struct not found\n", __func__);
    return -EINVAL;
    }
// Sanity check
    if (val > 1) {
    pr_warn("%s: Invalid argument %lld\n", __func__, val);
    return -EINVAL;
    }
// control enable/disable only if there is a delta in value
    if (sr_info.autocomp_active != val) {
    if (!val)
    sr_stop_vddautocomp(sr_info);
    else
    sr_start_vddautocomp(sr_info);
    }
    return 0;
    }
    DEFINE_SIMPLE_ATTRIBUTE(pm_sr_fops, omap_sr_autocomp_show,
    omap_sr_autocomp_store, "%llu\n");
#[no_mangle]
unsafe extern "C" fn omap_sr_probe(pdev: *mut platform_device) -> c_int {
    static int omap_sr_probe(struct platform_device *pdev)
    {
    struct omap_sr *sr_info;
    struct omap_sr_data *pdata = pdev.dev.platform_data;
    struct dentry *nvalue_dir;
    int i, ret = 0;
    sr_info = devm_kzalloc(&pdev.dev, sizeof(struct omap_sr), GFP_KERNEL);
    if (!sr_info)
    return -ENOMEM;
    sr_info.name = devm_kzalloc(&pdev.dev,
    SMARTREFLEX_NAME_LEN, GFP_KERNEL);
    if (!sr_info.name)
    return -ENOMEM;
    platform_set_drvdata(pdev, sr_info);
    if (!pdata) {
    dev_err(&pdev.dev, "%s: platform data missing\n", __func__);
    return -EINVAL;
    }
    sr_info.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sr_info.base))
    return PTR_ERR(sr_info.base);
    ret = platform_get_irq_optional(pdev, 0);
    if (ret < 0 && ret != -ENXIO)
    return dev_err_probe(&pdev.dev, ret, "failed to get IRQ resource\n");
    if (ret > 0)
    sr_info.irq = ret;
    sr_info.fck = devm_clk_get(pdev.dev.parent, "fck");
    if (IS_ERR(sr_info.fck))
    return PTR_ERR(sr_info.fck);
    clk_prepare(sr_info.fck);
    pm_runtime_enable(&pdev.dev);
    snprintf(sr_info.name, SMARTREFLEX_NAME_LEN, "%s", pdata.name);
    sr_info.pdev = pdev;
    sr_info.srid = pdev.id;
    sr_info.voltdm = pdata.voltdm;
    sr_info.nvalue_table = pdata.nvalue_table;
    sr_info.nvalue_count = pdata.nvalue_count;
    sr_info.senn_mod = pdata.senn_mod;
    sr_info.senp_mod = pdata.senp_mod;
    sr_info.err_weight = pdata.err_weight;
    sr_info.err_maxlimit = pdata.err_maxlimit;
    sr_info.accum_data = pdata.accum_data;
    sr_info.senn_avgweight = pdata.senn_avgweight;
    sr_info.senp_avgweight = pdata.senp_avgweight;
    sr_info.autocomp_active = false;
    sr_info.ip_type = pdata.ip_type;
    sr_set_clk_length(sr_info);
    list_add(&sr_info.node, &sr_list);
//
// Call into late init to do initializations that require
// both sr driver and sr class driver to be initiallized.
//
    if (sr_class) {
    ret = sr_late_init(sr_info);
    if (ret) {
    pr_warn("%s: Error in SR late init\n", __func__);
    goto err_list_del;
    }
    }
    dev_info(&pdev.dev, "%s: SmartReflex driver initialized\n", __func__);
    if (!sr_dbg_dir)
    sr_dbg_dir = debugfs_create_dir("smartreflex", core::ptr::null_mut());
    sr_info.dbg_dir = debugfs_create_dir(sr_info.name, sr_dbg_dir);
    debugfs_create_file("autocomp", S_IRUGO | S_IWUSR, sr_info.dbg_dir,
    sr_info, &pm_sr_fops);
    debugfs_create_x32("errweight", S_IRUGO, sr_info.dbg_dir,
    &sr_info.err_weight);
    debugfs_create_x32("errmaxlimit", S_IRUGO, sr_info.dbg_dir,
    &sr_info.err_maxlimit);
    nvalue_dir = debugfs_create_dir("nvalue", sr_info.dbg_dir);
    if (sr_info.nvalue_count == 0 || !sr_info.nvalue_table) {
    dev_warn(&pdev.dev, "%s: %s: No Voltage table for the corresponding vdd. Cannot create debugfs entries for n-values\n",
    __func__, sr_info.name);
    ret = -ENODATA;
    goto err_debugfs;
    }
    for (i = 0; i < sr_info.nvalue_count; i++) {
    char name[NVALUE_NAME_LEN + 1];
    snprintf(name, sizeof(name), "volt_%lu",
    sr_info.nvalue_table[i].volt_nominal);
    debugfs_create_x32(name, S_IRUGO | S_IWUSR, nvalue_dir,
    &(sr_info.nvalue_table[i].nvalue));
    snprintf(name, sizeof(name), "errminlimit_%lu",
    sr_info.nvalue_table[i].volt_nominal);
    debugfs_create_x32(name, S_IRUGO | S_IWUSR, nvalue_dir,
    &(sr_info.nvalue_table[i].errminlimit));
    }
    return 0;
    err_debugfs:
    debugfs_remove_recursive(sr_info.dbg_dir);
    err_list_del:
    pm_runtime_disable(&pdev.dev);
    list_del(&sr_info.node);
    clk_unprepare(sr_info.fck);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_sr_remove(pdev: *mut platform_device) {
    static void omap_sr_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct omap_sr *sr_info = platform_get_drvdata(pdev);
    if (sr_info.autocomp_active)
    sr_stop_vddautocomp(sr_info);
    debugfs_remove_recursive(sr_info.dbg_dir);
    pm_runtime_disable(dev);
    clk_unprepare(sr_info.fck);
    list_del(&sr_info.node);
    }
#[no_mangle]
unsafe extern "C" fn omap_sr_shutdown(pdev: *mut platform_device) {
    static void omap_sr_shutdown(struct platform_device *pdev)
    {
    struct omap_sr *sr_info = platform_get_drvdata(pdev);
    if (sr_info.autocomp_active)
    sr_stop_vddautocomp(sr_info);
    return;
    }
    static const struct of_device_id omap_sr_match[] = {
    { .compatible = "ti,omap3-smartreflex-core", },
    { .compatible = "ti,omap3-smartreflex-mpu-iva", },
    { .compatible = "ti,omap4-smartreflex-core", },
    { .compatible = "ti,omap4-smartreflex-mpu", },
    { .compatible = "ti,omap4-smartreflex-iva", },
    {  },
    };
    MODULE_DEVICE_TABLE(of, omap_sr_match);
    static struct platform_driver smartreflex_driver = {
    .probe		= omap_sr_probe,
    .remove         = omap_sr_remove,
    .shutdown	= omap_sr_shutdown,
    .driver		= {
    .name	= DRIVER_NAME,
    .of_match_table	= omap_sr_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn sr_init() -> int __init {
    static int __init sr_init(void)
    {
    let mut ret: c_int = 0;
    ret = platform_driver_register(&smartreflex_driver);
    if (ret) {
    pr_err("%s: platform driver register failed for SR\n",
    __func__);
    return ret;
    }
    return 0;
    }
    late_initcall(sr_init);
#[no_mangle]
unsafe extern "C" fn sr_exit() -> void __exit {
    static void __exit sr_exit(void)
    {
    platform_driver_unregister(&smartreflex_driver);
    }
    module_exit(sr_exit);
    MODULE_DESCRIPTION("OMAP Smartreflex Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRIVER_NAME);
    MODULE_AUTHOR("Texas Instruments Inc");
