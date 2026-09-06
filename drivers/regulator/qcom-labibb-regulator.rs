//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/qcom-labibb-regulator.c
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
// Copyright (c) 2020, The Linux Foundation. All rights reserved.

pub const REG_PERPH_TYPE: c_uint = 0x04;
pub const QCOM_LAB_TYPE: c_uint = 0x24;
pub const QCOM_IBB_TYPE: c_uint = 0x20;
pub const PMI8998_LAB_REG_BASE: c_uint = 0xde00;
pub const PMI8998_IBB_REG_BASE: c_uint = 0xdc00;
pub const PMI8998_IBB_LAB_REG_OFFSET: c_uint = 0x200;
pub const REG_LABIBB_STATUS1: c_uint = 0x08;

pub const REG_LABIBB_INT_SET_TYPE: c_uint = 0x11;
pub const REG_LABIBB_INT_POLARITY_HIGH: c_uint = 0x12;
pub const REG_LABIBB_INT_POLARITY_LOW: c_uint = 0x13;
pub const REG_LABIBB_INT_LATCHED_CLR: c_uint = 0x14;
pub const REG_LABIBB_INT_EN_SET: c_uint = 0x15;
pub const REG_LABIBB_INT_EN_CLR: c_uint = 0x16;

pub const LABIBB_INT_VREG_TYPE_LEVEL: c_int = 0;
pub const REG_LABIBB_VOLTAGE: c_uint = 0x41;

pub const REG_LABIBB_ENABLE_CTL: c_uint = 0x46;

pub const REG_LABIBB_PD_CTL: c_uint = 0x47;

pub const REG_LABIBB_CURRENT_LIMIT: c_uint = 0x4b;

pub const REG_IBB_PWRUP_PWRDN_CTL_1: c_uint = 0x58;

pub const REG_LABIBB_SOFT_START_CTL: c_uint = 0x5f;
pub const REG_LABIBB_SEC_ACCESS: c_uint = 0xd0;
pub const LABIBB_SEC_UNLOCK_CODE: c_uint = 0xa5;

pub const LABIBB_OFF_ON_DELAY: c_int = 1000;

pub const LABIBB_POLL_ENABLED_TIME: c_int = 1000;
pub const OCP_RECOVERY_INTERVAL_MS: c_int = 500;
pub const SC_RECOVERY_INTERVAL_MS: c_int = 250;
pub const LABIBB_MAX_OCP_COUNT: c_int = 4;
pub const LABIBB_MAX_SC_COUNT: c_int = 3;
pub const LABIBB_MAX_FATAL_COUNT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct labibb_current_limits {
    pub uA_min: u32,
    pub uA_step: u32,
    pub ovr_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct labibb_regulator {
    pub desc: regulator_desc,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub rdev: *mut regulator_dev,
    pub uA_limits: labibb_current_limits,
    pub ocp_recovery_work: delayed_work,
    pub sc_recovery_work: delayed_work,
    pub base: u16,
    pub type: u8,
    pub dischg_sel: u8,
    pub soft_start_sel: u8,
    pub sc_irq: c_int,
    pub sc_count: c_int,
    pub ocp_irq: c_int,
    pub ocp_irq_count: c_int,
    pub fatal_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct labibb_regulator_data {
    pub name: *const c_char,
    pub type: u8,
    pub base: u16,
    pub desc: *const regulator_desc,
}

#[no_mangle]
unsafe extern "C" fn qcom_labibb_ocp_hw_enable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_labibb_ocp_hw_enable(struct regulator_dev *rdev)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    int ret;
// Clear irq latch status to avoid spurious event
    ret = regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_LATCHED_CLR,
    LABIBB_INT_VREG_OK, 1);
    if (ret)
    return ret;
// Enable OCP HW interrupt
    return regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_EN_SET,
    LABIBB_INT_VREG_OK, 1);
    }
#[no_mangle]
unsafe extern "C" fn qcom_labibb_ocp_hw_disable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_labibb_ocp_hw_disable(struct regulator_dev *rdev)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    return regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_EN_CLR,
    LABIBB_INT_VREG_OK, 1);
    }
//
// qcom_labibb_check_ocp_status - Check the Over-Current Protection status
// @vreg: Main driver structure
//
// This function checks the STATUS1 register for the VREG_OK bit: if it is
// set, then there is no Over-Current event.
//
// Returns: Zero if there is no over-current, 1 if in over-current or
// negative number for error
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_check_ocp_status(vreg: *mut labibb_regulator) -> c_int {
    static int qcom_labibb_check_ocp_status(struct labibb_regulator *vreg)
    {
    u32 cur_status;
    int ret;
    ret = regmap_read(vreg.rdev.regmap, vreg.base + REG_LABIBB_STATUS1,
    &cur_status);
    if (ret)
    return ret;
    return !(cur_status & LABIBB_STATUS1_VREG_OK_BIT);
    }
//
// qcom_labibb_ocp_recovery_worker - Handle OCP event
// @work: OCP work structure
//
// This is the worker function to handle the Over Current Protection
// hardware event; This will check if the hardware is still
// signaling an over-current condition and will eventually stop
// the regulator if such condition is still signaled after
// LABIBB_MAX_OCP_COUNT times.
//
// If the driver that is consuming the regulator did not take action
// for the OCP condition, or the hardware did not stabilize, a cut
// of the LAB and IBB regulators will be forced (regulators will be
// disabled).
//
// As last, if the writes to shut down the LAB/IBB regulators fail
// for more than LABIBB_MAX_FATAL_COUNT, then a kernel panic will be
// triggered, as a last resort to protect the hardware from burning;
// this, however, is expected to never happen, but this is kept to
// try to further ensure that we protect the hardware at all costs.
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_ocp_recovery_worker(work: *mut work_struct) {
    static void qcom_labibb_ocp_recovery_worker(struct work_struct *work)
    {
    struct labibb_regulator *vreg;
    const struct regulator_ops *ops;
    int ret;
    vreg = container_of(work, struct labibb_regulator,
    ocp_recovery_work.work);
    ops = vreg.rdev.desc.ops;
    if (vreg.ocp_irq_count >= LABIBB_MAX_OCP_COUNT) {
//
// If we tried to disable the regulator multiple times but
// we kept failing, there's only one last hope to save our
// hardware from the death: raise a kernel bug, reboot and
// hope that the bootloader kindly saves us. This, though
// is done only as paranoid checking, because failing the
// regmap write to disable the vreg is almost impossible,
// since we got here after multiple regmap R/W.
//
    BUG_ON(vreg.fatal_count > LABIBB_MAX_FATAL_COUNT);
    dev_err(&vreg.rdev.dev, "LABIBB: CRITICAL: Disabling regulator\n");
// Disable the regulator immediately to avoid damage
    ret = ops.disable(vreg.rdev);
    if (ret) {
    vreg.fatal_count++;
    goto reschedule;
    }
    enable_irq(vreg.ocp_irq);
    vreg.fatal_count = 0;
    return;
    }
    ret = qcom_labibb_check_ocp_status(vreg);
    if (ret != 0) {
    vreg.ocp_irq_count++;
    goto reschedule;
    }
    ret = qcom_labibb_ocp_hw_enable(vreg.rdev);
    if (ret) {
// We cannot trust it without OCP enabled.
    dev_err(vreg.dev, "Cannot enable OCP IRQ\n");
    vreg.ocp_irq_count++;
    goto reschedule;
    }
    enable_irq(vreg.ocp_irq);
// Everything went fine: reset the OCP count!
    vreg.ocp_irq_count = 0;
    return;
    reschedule:
    mod_delayed_work(system_dfl_wq, &vreg.ocp_recovery_work,
    msecs_to_jiffies(OCP_RECOVERY_INTERVAL_MS));
    }
//
// qcom_labibb_ocp_isr - Interrupt routine for OverCurrent Protection
// @irq:  Interrupt number
// @chip: Main driver structure
//
// Over Current Protection (OCP) will signal to the client driver
// that an over-current event has happened and then will schedule
// a recovery worker.
//
// Disabling and eventually re-enabling the regulator is expected
// to be done by the driver, as some hardware may be triggering an
// over-current condition only at first initialization or it may
// be expected only for a very brief amount of time, after which
// the attached hardware may be expected to stabilize its current
// draw.
//
// Returns: IRQ_HANDLED for success or IRQ_NONE for failure.
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_ocp_isr(irq: c_int, chip: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_labibb_ocp_isr(int irq, void *chip)
    {
    struct labibb_regulator *vreg = chip;
    const struct regulator_ops *ops = vreg.rdev.desc.ops;
    int ret;
// If the regulator is not enabled, this is a fake event
    if (!ops.is_enabled(vreg.rdev))
    return IRQ_HANDLED;
// If we tried to recover for too many times it's not getting better
    if (vreg.ocp_irq_count > LABIBB_MAX_OCP_COUNT)
    return IRQ_NONE;
//
// If we (unlikely) can't read this register, to prevent hardware
// damage at all costs, we assume that the overcurrent event was
// real; Moreover, if the status register is not signaling OCP,
// it was a spurious event, so it's all ok.
//
    ret = qcom_labibb_check_ocp_status(vreg);
    if (ret == 0) {
    vreg.ocp_irq_count = 0;
    goto end;
    }
    vreg.ocp_irq_count++;
//
// Disable the interrupt temporarily, or it will fire continuously;
// we will re-enable it in the recovery worker function.
//
    disable_irq_nosync(irq);
// Warn the user for overcurrent
    dev_warn(vreg.dev, "Over-Current interrupt fired!\n");
// Disable the interrupt to avoid hogging
    ret = qcom_labibb_ocp_hw_disable(vreg.rdev);
    if (ret)
    goto end;
// Signal overcurrent event to drivers
    regulator_notifier_call_chain(vreg.rdev,
    REGULATOR_EVENT_OVER_CURRENT, core::ptr::null_mut());
    end:
// Schedule the recovery work
    schedule_delayed_work(&vreg.ocp_recovery_work,
    msecs_to_jiffies(OCP_RECOVERY_INTERVAL_MS));
    if (ret)
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
    static int qcom_labibb_set_ocp(struct regulator_dev *rdev, int lim,
    int severity, bool enable)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    char *ocp_irq_name;
    let mut irq_flags: u32 = IRQF_ONESHOT;
    int irq_trig_low, ret;
//
// labibb supports only protection - and does not support setting
// limit. Furthermore, we don't support disabling protection.
//
    if (lim || severity != REGULATOR_SEVERITY_PROT || !enable)
    return -EINVAL;
// If there is no OCP interrupt, there's nothing to set
    if (vreg.ocp_irq <= 0)
    return -EINVAL;
    ocp_irq_name = devm_kasprintf(vreg.dev, GFP_KERNEL, "%s-over-current",
    vreg.desc.name);
    if (!ocp_irq_name)
    return -ENOMEM;
// IRQ polarities - LAB: trigger-low, IBB: trigger-high
    switch (vreg.type) {
    case QCOM_LAB_TYPE:
    irq_flags |= IRQF_TRIGGER_LOW;
    irq_trig_low = 1;
    break;
    case QCOM_IBB_TYPE:
    irq_flags |= IRQF_TRIGGER_HIGH;
    irq_trig_low = 0;
    break;
    default:
    return -EINVAL;
    }
// Activate OCP HW level interrupt
    ret = regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_SET_TYPE,
    LABIBB_INT_VREG_OK,
    LABIBB_INT_VREG_TYPE_LEVEL);
    if (ret)
    return ret;
// Set OCP interrupt polarity
    ret = regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_POLARITY_HIGH,
    LABIBB_INT_VREG_OK, !irq_trig_low);
    if (ret)
    return ret;
    ret = regmap_update_bits(rdev.regmap,
    vreg.base + REG_LABIBB_INT_POLARITY_LOW,
    LABIBB_INT_VREG_OK, irq_trig_low);
    if (ret)
    return ret;
    ret = qcom_labibb_ocp_hw_enable(rdev);
    if (ret)
    return ret;
    return devm_request_threaded_irq(vreg.dev, vreg.ocp_irq, core::ptr::null_mut(),
    qcom_labibb_ocp_isr, irq_flags,
    ocp_irq_name, vreg);
    }
//
// qcom_labibb_check_sc_status - Check the Short Circuit Protection status
// @vreg: Main driver structure
//
// This function checks the STATUS1 register on both LAB and IBB regulators
// for the ShortCircuit bit: if it is set on *any* of them, then we have
// experienced a short-circuit event.
//
// Returns: Zero if there is no short-circuit, 1 if in short-circuit or
// negative number for error
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_check_sc_status(vreg: *mut labibb_regulator) -> c_int {
    static int qcom_labibb_check_sc_status(struct labibb_regulator *vreg)
    {
    u32 ibb_status, ibb_reg, lab_status, lab_reg;
    int ret;
// We have to work on both regulators due to PBS...
    lab_reg = ibb_reg = vreg.base + REG_LABIBB_STATUS1;
    if (vreg.type == QCOM_LAB_TYPE)
    ibb_reg -= PMI8998_IBB_LAB_REG_OFFSET;
    else
    lab_reg += PMI8998_IBB_LAB_REG_OFFSET;
    ret = regmap_read(vreg.rdev.regmap, lab_reg, &lab_status);
    if (ret)
    return ret;
    ret = regmap_read(vreg.rdev.regmap, ibb_reg, &ibb_status);
    if (ret)
    return ret;
    return !!(lab_status & LABIBB_STATUS1_SC_BIT) ||
    !!(ibb_status & LABIBB_STATUS1_SC_BIT);
    }
//
// qcom_labibb_sc_recovery_worker - Handle Short Circuit event
// @work: SC work structure
//
// This is the worker function to handle the Short Circuit Protection
// hardware event; This will check if the hardware is still
// signaling a short-circuit condition and will eventually never
// re-enable the regulator if such condition is still signaled after
// LABIBB_MAX_SC_COUNT times.
//
// If the driver that is consuming the regulator did not take action
// for the SC condition, or the hardware did not stabilize, this
// worker will stop rescheduling, leaving the regulators disabled
// as already done by the Portable Batch System (PBS).
//
// Returns: IRQ_HANDLED for success or IRQ_NONE for failure.
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_sc_recovery_worker(work: *mut work_struct) {
    static void qcom_labibb_sc_recovery_worker(struct work_struct *work)
    {
    struct labibb_regulator *vreg;
    const struct regulator_ops *ops;
    u32 lab_reg, ibb_reg, lab_val, ibb_val, val;
    let mut pbs_cut: bool = false;
    int i, sc, ret;
    vreg = container_of(work, struct labibb_regulator,
    sc_recovery_work.work);
    ops = vreg.rdev.desc.ops;
//
// If we tried to check the regulator status multiple times but we
// kept failing, then just bail out, as the Portable Batch System
// (PBS) will disable the vregs for us, preventing hardware damage.
//
    if (vreg.fatal_count > LABIBB_MAX_FATAL_COUNT)
    return;
// Too many short-circuit events. Throw in the towel.
    if (vreg.sc_count > LABIBB_MAX_SC_COUNT)
    return;
//
// The Portable Batch System (PBS) automatically disables LAB
// and IBB when a short-circuit event is detected, so we have to
// check and work on both of them at the same time.
//
    lab_reg = ibb_reg = vreg.base + REG_LABIBB_ENABLE_CTL;
    if (vreg.type == QCOM_LAB_TYPE)
    ibb_reg -= PMI8998_IBB_LAB_REG_OFFSET;
    else
    lab_reg += PMI8998_IBB_LAB_REG_OFFSET;
    sc = qcom_labibb_check_sc_status(vreg);
    if (sc)
    goto reschedule;
    for (i = 0; i < LABIBB_MAX_SC_COUNT; i++) {
    ret = regmap_read(vreg.regmap, lab_reg, &lab_val);
    if (ret) {
    vreg.fatal_count++;
    goto reschedule;
    }
    ret = regmap_read(vreg.regmap, ibb_reg, &ibb_val);
    if (ret) {
    vreg.fatal_count++;
    goto reschedule;
    }
    val = lab_val & ibb_val;
    if (!(val & LABIBB_CONTROL_ENABLE)) {
    pbs_cut = true;
    break;
    }
    usleep_range(5000, 6000);
    }
    if (pbs_cut)
    goto reschedule;
//
// If we have reached this point, we either have successfully
// recovered from the SC condition or we had a spurious SC IRQ,
// which means that we can re-enable the regulators, if they
// have ever been disabled by the PBS.
//
    ret = ops.enable(vreg.rdev);
    if (ret)
    goto reschedule;
// Everything went fine: reset the OCP count!
    vreg.sc_count = 0;
    enable_irq(vreg.sc_irq);
    return;
    reschedule:
//
// Now that we have done basic handling of the short-circuit,
// reschedule this worker in the regular system workqueue, as
// taking action is not truly urgent anymore.
//
    vreg.sc_count++;
    mod_delayed_work(system_dfl_wq, &vreg.sc_recovery_work,
    msecs_to_jiffies(SC_RECOVERY_INTERVAL_MS));
    }
//
// qcom_labibb_sc_isr - Interrupt routine for Short Circuit Protection
// @irq:  Interrupt number
// @chip: Main driver structure
//
// Short Circuit Protection (SCP) will signal to the client driver
// that a regulation-out event has happened and then will schedule
// a recovery worker.
//
// The LAB and IBB regulators will be automatically disabled by the
// Portable Batch System (PBS) and they will be enabled again by
// the worker function if the hardware stops signaling the short
// circuit event.
//
// Returns: IRQ_HANDLED for success or IRQ_NONE for failure.
//
#[no_mangle]
unsafe extern "C" fn qcom_labibb_sc_isr(irq: c_int, chip: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_labibb_sc_isr(int irq, void *chip)
    {
    struct labibb_regulator *vreg = chip;
    if (vreg.sc_count > LABIBB_MAX_SC_COUNT)
    return IRQ_NONE;
// Warn the user for short circuit
    dev_warn(vreg.dev, "Short-Circuit interrupt fired!\n");
//
// Disable the interrupt temporarily, or it will fire continuously;
// we will re-enable it in the recovery worker function.
//
    disable_irq_nosync(irq);
// Signal out of regulation event to drivers
    regulator_notifier_call_chain(vreg.rdev,
    REGULATOR_EVENT_REGULATION_OUT, core::ptr::null_mut());
// Schedule the short-circuit handling as high-priority work
    mod_delayed_work(system_highpri_wq, &vreg.sc_recovery_work,
    msecs_to_jiffies(SC_RECOVERY_INTERVAL_MS));
    return IRQ_HANDLED;
    }
    static int qcom_labibb_set_current_limit(struct regulator_dev *rdev,
    int min_uA, int max_uA)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    struct regulator_desc *desc = &vreg.desc;
    struct labibb_current_limits *lim = &vreg.uA_limits;
    u32 mask, val;
    int i, ret, sel = -1;
    if (min_uA < lim.uA_min || max_uA < lim.uA_min)
    return -EINVAL;
    for (i = 0; i < desc.n_current_limits; i++) {
    let mut uA_limit: c_int = (lim.uA_step * i) + lim.uA_min;
    if (max_uA >= uA_limit && min_uA <= uA_limit)
    sel = i;
    }
    if (sel < 0)
    return -EINVAL;
// Current limit setting needs secure access
    ret = regmap_write(vreg.regmap, vreg.base + REG_LABIBB_SEC_ACCESS,
    LABIBB_SEC_UNLOCK_CODE);
    if (ret)
    return ret;
    mask = desc.csel_mask | lim.ovr_val;
    mask |= LABIBB_CURRENT_LIMIT_EN;
    val = (u32)sel | lim.ovr_val;
    val |= LABIBB_CURRENT_LIMIT_EN;
    return regmap_update_bits(vreg.regmap, desc.csel_reg, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn qcom_labibb_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int qcom_labibb_get_current_limit(struct regulator_dev *rdev)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    struct regulator_desc *desc = &vreg.desc;
    struct labibb_current_limits *lim = &vreg.uA_limits;
    unsigned int cur_step;
    int ret;
    ret = regmap_read(vreg.regmap, desc.csel_reg, &cur_step);
    if (ret)
    return ret;
    cur_step &= desc.csel_mask;
    return (cur_step * lim.uA_step) + lim.uA_min;
    }
#[no_mangle]
unsafe extern "C" fn qcom_labibb_set_soft_start(rdev: *mut regulator_dev) -> c_int {
    static int qcom_labibb_set_soft_start(struct regulator_dev *rdev)
    {
    struct labibb_regulator *vreg = rdev_get_drvdata(rdev);
    let mut val: u32 = 0;
    if (vreg.type == QCOM_IBB_TYPE)
    val = vreg.dischg_sel;
    else
    val = vreg.soft_start_sel;
    return regmap_write(rdev.regmap, rdev.desc.soft_start_reg, val);
    }
#[no_mangle]
unsafe extern "C" fn qcom_labibb_get_table_sel(table: *const c_int, sz: c_int, value: u32) -> c_int {
    static int qcom_labibb_get_table_sel(const int *table, int sz, u32 value)
    {
    int i;
    for (i = 0; i < sz; i++)
    if (table[i] == value)
    return i;
    return -EINVAL;
    }
// IBB discharge resistor values in KOhms
    static const int dischg_resistor_values[] = { 300, 64, 32, 16 };
// Soft start time in microseconds
    static const int soft_start_values[] = { 200, 400, 600, 800 };
    static int qcom_labibb_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *config)
    {
    struct labibb_regulator *vreg = config.driver_data;
    u32 dischg_kohms, soft_start_time;
    int ret;
    ret = of_property_read_u32(np, "qcom,discharge-resistor-kohms",
    &dischg_kohms);
    if (ret)
    dischg_kohms = 300;
    ret = qcom_labibb_get_table_sel(dischg_resistor_values,
    ARRAY_SIZE(dischg_resistor_values),
    dischg_kohms);
    if (ret < 0)
    return ret;
    vreg.dischg_sel = (u8)ret;
    ret = of_property_read_u32(np, "qcom,soft-start-us",
    &soft_start_time);
    if (ret)
    soft_start_time = 200;
    ret = qcom_labibb_get_table_sel(soft_start_values,
    ARRAY_SIZE(soft_start_values),
    soft_start_time);
    if (ret < 0)
    return ret;
    vreg.soft_start_sel = (u8)ret;
    return 0;
    }
    static const struct regulator_ops qcom_labibb_ops = {
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    .map_voltage		= regulator_map_voltage_linear,
    .set_active_discharge	= regulator_set_active_discharge_regmap,
    .set_pull_down		= regulator_set_pull_down_regmap,
    .set_current_limit	= qcom_labibb_set_current_limit,
    .get_current_limit	= qcom_labibb_get_current_limit,
    .set_soft_start		= qcom_labibb_set_soft_start,
    .set_over_current_protection = qcom_labibb_set_ocp,
    };
    static const struct regulator_desc pmi8998_lab_desc = {
    .enable_mask		= LAB_ENABLE_CTL_MASK,
    .enable_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_ENABLE_CTL),
    .enable_val		= LABIBB_CONTROL_ENABLE,
    .enable_time		= LAB_ENABLE_TIME,
    .poll_enabled_time	= LABIBB_POLL_ENABLED_TIME,
    .soft_start_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_SOFT_START_CTL),
    .pull_down_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_PD_CTL),
    .pull_down_mask		= LAB_PD_CTL_MASK,
    .pull_down_val_on	= LAB_PD_CTL_STRONG_PULL,
    .vsel_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_VOLTAGE),
    .vsel_mask		= LAB_VOLTAGE_SET_MASK,
    .apply_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_VOLTAGE),
    .apply_bit		= LABIBB_VOLTAGE_OVERRIDE_EN,
    .csel_reg		= (PMI8998_LAB_REG_BASE + REG_LABIBB_CURRENT_LIMIT),
    .csel_mask		= LAB_CURRENT_LIMIT_MASK,
    .n_current_limits	= 8,
    .off_on_delay		= LABIBB_OFF_ON_DELAY,
    .owner			= THIS_MODULE,
    .type			= REGULATOR_VOLTAGE,
    .min_uV			= 4600000,
    .uV_step		= 100000,
    .n_voltages		= 16,
    .ops			= &qcom_labibb_ops,
    .of_parse_cb		= qcom_labibb_of_parse_cb,
    };
    static const struct regulator_desc pmi8998_ibb_desc = {
    .enable_mask		= IBB_ENABLE_CTL_MASK,
    .enable_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_ENABLE_CTL),
    .enable_val		= LABIBB_CONTROL_ENABLE,
    .enable_time		= IBB_ENABLE_TIME,
    .poll_enabled_time	= LABIBB_POLL_ENABLED_TIME,
    .soft_start_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_SOFT_START_CTL),
    .active_discharge_off	= 0,
    .active_discharge_on	= IBB_CTL_1_DISCHARGE_EN,
    .active_discharge_mask	= IBB_CTL_1_DISCHARGE_EN,
    .active_discharge_reg	= (PMI8998_IBB_REG_BASE + REG_IBB_PWRUP_PWRDN_CTL_1),
    .pull_down_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_PD_CTL),
    .pull_down_mask		= IBB_PD_CTL_MASK,
    .pull_down_val_on	= IBB_PD_CTL_HALF_STRENGTH | IBB_PD_CTL_EN,
    .vsel_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_VOLTAGE),
    .vsel_mask		= IBB_VOLTAGE_SET_MASK,
    .apply_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_VOLTAGE),
    .apply_bit		= LABIBB_VOLTAGE_OVERRIDE_EN,
    .csel_reg		= (PMI8998_IBB_REG_BASE + REG_LABIBB_CURRENT_LIMIT),
    .csel_mask		= IBB_CURRENT_LIMIT_MASK,
    .n_current_limits	= 32,
    .off_on_delay		= LABIBB_OFF_ON_DELAY,
    .owner			= THIS_MODULE,
    .type			= REGULATOR_VOLTAGE,
    .min_uV			= 1400000,
    .uV_step		= 100000,
    .n_voltages		= 64,
    .ops			= &qcom_labibb_ops,
    .of_parse_cb		= qcom_labibb_of_parse_cb,
    };
    static const struct labibb_regulator_data pmi8998_labibb_data[] = {
    {"lab", QCOM_LAB_TYPE, PMI8998_LAB_REG_BASE, &pmi8998_lab_desc},
    {"ibb", QCOM_IBB_TYPE, PMI8998_IBB_REG_BASE, &pmi8998_ibb_desc},
    { },
    };
    static const struct of_device_id qcom_labibb_match[] = {
    { .compatible = "qcom,pmi8998-lab-ibb", .data = &pmi8998_labibb_data},
    { },
    };
    MODULE_DEVICE_TABLE(of, qcom_labibb_match);
#[no_mangle]
unsafe extern "C" fn qcom_labibb_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_labibb_regulator_probe(struct platform_device *pdev)
    {
    struct labibb_regulator *vreg;
    struct device *dev = &pdev.dev;
    let mut cfg: regulator_config = {};
    struct device_node *reg_node;
    const struct labibb_regulator_data *reg_data;
    struct regmap *reg_regmap;
    unsigned int type;
    int ret;
    reg_regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!reg_regmap) {
    dev_err(&pdev.dev, "Couldn't get parent's regmap\n");
    return -ENODEV;
    }
    reg_data = device_get_match_data(&pdev.dev);
    if (!reg_data)
    return -ENODEV;
    for (; reg_data.name; reg_data++) {
    char *sc_irq_name;
    let mut irq: c_int = 0;
// Validate if the type of regulator is indeed
// what's mentioned in DT.
//
    ret = regmap_read(reg_regmap, reg_data.base + REG_PERPH_TYPE,
    &type);
    if (ret < 0) {
    dev_err(dev,
    "Peripheral type read failed ret=%d\n",
    ret);
    return -EINVAL;
    }
    if (WARN_ON((type != QCOM_LAB_TYPE) && (type != QCOM_IBB_TYPE)) ||
    WARN_ON(type != reg_data.type))
    return -EINVAL;
    vreg  = devm_kzalloc(&pdev.dev, sizeof(*vreg),
    GFP_KERNEL);
    if (!vreg)
    return -ENOMEM;
    sc_irq_name = devm_kasprintf(dev, GFP_KERNEL,
    "%s-short-circuit",
    reg_data.name);
    if (!sc_irq_name)
    return -ENOMEM;
    reg_node = of_get_child_by_name(pdev.dev.of_node,
    reg_data.name);
    if (!reg_node)
    return -EINVAL;
// The Short Circuit interrupt is critical
    irq = of_irq_get_byname(reg_node, "sc-err");
    if (irq <= 0) {
    if (irq == 0)
    irq = -EINVAL;
    of_node_put(reg_node);
    return dev_err_probe(vreg.dev, irq,
    "Short-circuit irq not found.\n");
    }
    vreg.sc_irq = irq;
// OverCurrent Protection IRQ is optional
    irq = of_irq_get_byname(reg_node, "ocp");
    vreg.ocp_irq = irq;
    vreg.ocp_irq_count = 0;
    of_node_put(reg_node);
    vreg.regmap = reg_regmap;
    vreg.dev = dev;
    vreg.base = reg_data.base;
    vreg.type = reg_data.type;
    INIT_DELAYED_WORK(&vreg.sc_recovery_work,
    qcom_labibb_sc_recovery_worker);
    if (vreg.ocp_irq > 0)
    INIT_DELAYED_WORK(&vreg.ocp_recovery_work,
    qcom_labibb_ocp_recovery_worker);
    switch (vreg.type) {
    case QCOM_LAB_TYPE:
// LAB Limits: 200-1600mA
    vreg.uA_limits.uA_min  = 200000;
    vreg.uA_limits.uA_step = 200000;
    vreg.uA_limits.ovr_val = LAB_CURRENT_LIMIT_OVERRIDE_EN;
    break;
    case QCOM_IBB_TYPE:
// IBB Limits: 0-1550mA
    vreg.uA_limits.uA_min  = 0;
    vreg.uA_limits.uA_step = 50000;
    vreg.uA_limits.ovr_val = 0; /* No override bit */
    break;
    default:
    return -EINVAL;
    }
    memcpy(&vreg.desc, reg_data.desc, sizeof(vreg.desc));
    vreg.desc.of_match = reg_data.name;
    vreg.desc.name = reg_data.name;
    cfg.dev = vreg.dev;
    cfg.driver_data = vreg;
    cfg.regmap = vreg.regmap;
    vreg.rdev = devm_regulator_register(vreg.dev, &vreg.desc,
    &cfg);
    if (IS_ERR(vreg.rdev)) {
    dev_err(dev, "qcom_labibb: error registering %s : %d\n",
    reg_data.name, ret);
    return PTR_ERR(vreg.rdev);
    }
    ret = devm_request_threaded_irq(vreg.dev, vreg.sc_irq, core::ptr::null_mut(),
    qcom_labibb_sc_isr,
    IRQF_ONESHOT |
    IRQF_TRIGGER_RISING,
    sc_irq_name, vreg);
    if (ret)
    return ret;
    }
    return 0;
    }
    static struct platform_driver qcom_labibb_regulator_driver = {
    .driver	= {
    .name = "qcom-lab-ibb-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= qcom_labibb_match,
    },
    .probe = qcom_labibb_regulator_probe,
    };
    module_platform_driver(qcom_labibb_regulator_driver);
    MODULE_DESCRIPTION("Qualcomm labibb driver");
    MODULE_AUTHOR("Nisha Kumari <nishakumari@codeaurora.org>");
    MODULE_AUTHOR("Sumit Semwal <sumit.semwal@linaro.org>");
    MODULE_LICENSE("GPL v2");
