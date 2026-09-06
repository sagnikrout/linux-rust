//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-cti-sysfs.c
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
// Copyright (c) 2019 Linaro Limited, All rights reserved.
// Author: Mike Leach <mike.leach@linaro.org>
//

//
// Declare the number of static declared attribute groups
// Value includes groups + NULL value at end of table.
//
pub const CORESIGHT_CTI_STATIC_GROUPS_MAX: c_int = 5;
//
// List of trigger signal type names. Match the constants declared in
// include\dt-bindings\arm\coresight-cti-dt.h
//
    static const char * const sig_type_names[] = {
    "genio",	/* GEN_IO */
    "intreq",	/* GEN_INTREQ */
    "intack",	/* GEN_INTACK */
    "haltreq",	/* GEN_HALTREQ */
    "restartreq",	/* GEN_RESTARTREQ */
    "pe_edbgreq",	/* PE_EDBGREQ */
    "pe_dbgrestart",/* PE_DBGRESTART */
    "pe_ctiirq",	/* PE_CTIIRQ */
    "pe_pmuirq",	/* PE_PMUIRQ */
    "pe_dbgtrigger",/* PE_DBGTRIGGER */
    "etm_extout",	/* ETM_EXTOUT */
    "etm_extin",	/* ETM_EXTIN */
    "snk_full",	/* SNK_FULL */
    "snk_acqcomp",	/* SNK_ACQCOMP */
    "snk_flushcomp",/* SNK_FLUSHCOMP */
    "snk_flushin",	/* SNK_FLUSHIN */
    "snk_trigin",	/* SNK_TRIGIN */
    "stm_asyncout",	/* STM_ASYNCOUT */
    "stm_tout_spte",/* STM_TOUT_SPTE */
    "stm_tout_sw",	/* STM_TOUT_SW */
    "stm_tout_hete",/* STM_TOUT_HETE */
    "stm_hwevent",	/* STM_HWEVENT */
    "ela_tstart",	/* ELA_TSTART */
    "ela_tstop",	/* ELA_TSTOP */
    "ela_dbgreq",	/* ELA_DBGREQ */
    };
// Show function pointer used in the connections dynamic declared attributes
    typedef ssize_t (*p_show_fn)(struct device *dev, struct device_attribute *attr,
    char *buf);
// Connection attribute types
    enum cti_conn_attr_type {
    CTI_CON_ATTR_NAME,
    CTI_CON_ATTR_TRIGIN_SIG,
    CTI_CON_ATTR_TRIGOUT_SIG,
    CTI_CON_ATTR_TRIGIN_TYPES,
    CTI_CON_ATTR_TRIGOUT_TYPES,
    CTI_CON_ATTR_MAX,
    };
// Names for the connection attributes
    static const char * const con_attr_names[CTI_CON_ATTR_MAX] = {
    "name",
    "in_signals",
    "out_signals",
    "in_types",
    "out_types",
    };
// basic attributes
    static ssize_t enable_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    int enable_req;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock)
    enable_req = cti_is_active(&drvdata.config);
    return sprintf(buf, "%d\n", !!enable_req);
    }
    static ssize_t enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut ret: c_int = 0;
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    ret = kstrtoul(buf, 0, &val);
    if (ret)
    return ret;
    if (val) {
    ret = pm_runtime_resume_and_get(dev.parent);
    if (ret)
    return ret;
    ret = cti_enable(drvdata.csdev, CS_MODE_SYSFS, core::ptr::null_mut());
    if (ret)
    pm_runtime_put(dev.parent);
    } else {
    ret = cti_disable(drvdata.csdev, core::ptr::null_mut());
    if (!ret)
    pm_runtime_put(dev.parent);
    }
    if (ret)
    return ret;
    return size;
    }
    static DEVICE_ATTR_RW(enable);
    static ssize_t powered_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut powered: bool = pm_runtime_active(dev.parent);
    return sprintf(buf, "%d\n", powered);
    }
    static DEVICE_ATTR_RO(powered);
    static ssize_t ctmid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    return sprintf(buf, "%d\n", drvdata.ctidev.ctm_id);
    }
    static DEVICE_ATTR_RO(ctmid);
    static ssize_t nr_trigger_cons_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    return sprintf(buf, "%d\n", drvdata.ctidev.nr_trig_con);
    }
    static DEVICE_ATTR_RO(nr_trigger_cons);
// attribute and group sysfs tables.
    static struct attribute *coresight_cti_attrs[] = {
    &dev_attr_enable.attr,
    &dev_attr_powered.attr,
    &dev_attr_ctmid.attr,
    &dev_attr_nr_trigger_cons.attr,
    core::ptr::null_mut(),
    };
// register based attributes
// Read registers with power check only (no enable check).
    static ssize_t coresight_cti_reg_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cs_off_attribute *cti_attr = container_of(attr, struct cs_off_attribute, attr);
    let mut val: u32 = 0;
    pm_runtime_get_sync(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock)
    val = cti_read_single_reg(drvdata, cti_attr.off);
    pm_runtime_put_sync(dev.parent);
    return sysfs_emit(buf, "0x%x\n", val);
    }
// Write registers with power check only (no enable check).
    static __maybe_unused ssize_t coresight_cti_reg_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cs_off_attribute *cti_attr = container_of(attr, struct cs_off_attribute, attr);
    let mut val: c_ulong = 0;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    pm_runtime_get_sync(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock)
    cti_write_single_reg(drvdata, cti_attr.off, val);
    pm_runtime_put_sync(dev.parent);
    return size;
    }

    (&((struct cs_off_attribute[]) {				\
    {								\
    __ATTR(name, 0444, coresight_cti_reg_show, core::ptr::null_mut()),	\
    offset							\
    }								\
    })[0].attr.attr)

    (&((struct cs_off_attribute[]) {				\
    {								\
    __ATTR(name, 0644, coresight_cti_reg_show,		\
    coresight_cti_reg_store),			\
    offset							\
    }								\
    })[0].attr.attr)

    (&((struct cs_off_attribute[]) {				\
    {								\
    __ATTR(name, 0200, core::ptr::null_mut(), coresight_cti_reg_store),	\
    offset							\
    }								\
    })[0].attr.attr)
// coresight management registers
    static struct attribute *coresight_cti_mgmt_attrs[] = {
    coresight_cti_reg(devaff0, CTIDEVAFF0),
    coresight_cti_reg(devaff1, CTIDEVAFF1),
    coresight_cti_reg(authstatus, CORESIGHT_AUTHSTATUS),
    coresight_cti_reg(devarch, CORESIGHT_DEVARCH),
    coresight_cti_reg(devid, CORESIGHT_DEVID),
    coresight_cti_reg(devtype, CORESIGHT_DEVTYPE),
    coresight_cti_reg(pidr0, CORESIGHT_PERIPHIDR0),
    coresight_cti_reg(pidr1, CORESIGHT_PERIPHIDR1),
    coresight_cti_reg(pidr2, CORESIGHT_PERIPHIDR2),
    coresight_cti_reg(pidr3, CORESIGHT_PERIPHIDR3),
    coresight_cti_reg(pidr4, CORESIGHT_PERIPHIDR4),
    core::ptr::null_mut(),
    };
// CTI low level programming registers
//
// Show a simple 32 bit value if enabled and powered.
// If inaccessible & pcached_val not NULL then show cached value.
//
    static ssize_t cti_reg32_show(struct device *dev, char *buf,
    u32 *pcached_val, int reg_offset)
    {
    let mut val: u32 = 0;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (reg_offset < 0)
    return -EINVAL;
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock) {
    if (cti_is_active(config)) {
    val = cti_read_single_reg(drvdata, reg_offset);
    if (pcached_val)
// pcached_val = val;
    } else if (pcached_val) {
    val = *pcached_val;
    }
    }
    return sprintf(buf, "%#x\n", val);
    }
//
// Store a simple 32 bit value.
// If pcached_val not NULL, then copy to here too,
// if reg_offset >= 0 then write through if enabled.
//
    static ssize_t cti_reg32_store(struct device *dev, const char *buf,
    size_t size, u32 *pcached_val, int reg_offset)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    if (reg_offset < 0)
    return -EINVAL;
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock) {
// local store
    if (pcached_val)
// pcached_val = (u32)val;
// write through if offset and enabled
    if (cti_is_active(config))
    cti_write_single_reg(drvdata, reg_offset, val);
    }
    return size;
    }
// Standard macro for simple rw cti config registers

    static ssize_t name##_show(struct device *dev,				\
    struct device_attribute *attr,		\
    char *buf)					\
    {									\
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);	\
    return cti_reg32_show(dev, buf,					\
    &drvdata.config.cfgname, offset);	\
    }									\
    \
    static ssize_t name##_store(struct device *dev,				\
    struct device_attribute *attr,		\
    const char *buf, size_t size)		\
    {									\
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);	\
    return cti_reg32_store(dev, buf, size,				\
    &drvdata.config.cfgname, offset);	\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RW(_arg: name) -> static {
    static DEVICE_ATTR_RW(name)
    static ssize_t inout_sel_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u32 val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    val = (u32)drvdata.config.ctiinout_sel;
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t inout_sel_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    if (val >= config.nr_trig_max)
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
    drvdata.config.ctiinout_sel = val;
    return size;
    }
    static DEVICE_ATTR_RW(inout_sel);
    static ssize_t inen_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long val;
    int index;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock) {
    index = drvdata.config.ctiinout_sel;
    val = drvdata.config.ctiinen[index];
    }
    return sprintf(buf, "%#lx\n", val);
    }
    static ssize_t inen_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    int index;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
    index = config.ctiinout_sel;
    config.ctiinen[index] = val;
// write through if enabled
    if (cti_is_active(config))
    cti_write_single_reg(drvdata, CTIINEN(index), val);
    return size;
    }
    static DEVICE_ATTR_RW(inen);
    static ssize_t outen_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long val;
    int index;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock) {
    index = drvdata.config.ctiinout_sel;
    val = drvdata.config.ctiouten[index];
    }
    return sprintf(buf, "%#lx\n", val);
    }
    static ssize_t outen_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    int index;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
    index = config.ctiinout_sel;
    config.ctiouten[index] = val;
// write through if enabled
    if (cti_is_active(config))
    cti_write_single_reg(drvdata, CTIOUTEN(index), val);
    return size;
    }
    static DEVICE_ATTR_RW(outen);
    static ssize_t intack_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    cti_write_intack(dev, val);
    return size;
    }
    static DEVICE_ATTR_WO(intack);
    cti_config_reg32_rw(gate, ctigate, CTIGATE);
    cti_config_reg32_rw(asicctl, asicctl, ASICCTL);
    cti_config_reg32_rw(appset, ctiappset, CTIAPPSET);
    static ssize_t appclear_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
// a 1'b1 in appclr clears down the same bit in appset
    config.ctiappset &= ~val;
// write through if enabled
    if (cti_is_active(config))
    cti_write_single_reg(drvdata, CTIAPPCLEAR, val);
    return size;
    }
    static DEVICE_ATTR_WO(appclear);
    static ssize_t apppulse_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
// write through if enabled
    if (cti_is_active(config))
    cti_write_single_reg(drvdata, CTIAPPPULSE, val);
    return size;
    }
    static DEVICE_ATTR_WO(apppulse);
//
// Define CONFIG_CORESIGHT_CTI_INTEGRATION_REGS to enable the access to the
// integration control registers. Normally only used to investigate connection
// data.
//
    static struct attribute *coresight_cti_regs_attrs[] = {
    &dev_attr_inout_sel.attr,
    &dev_attr_inen.attr,
    &dev_attr_outen.attr,
    &dev_attr_gate.attr,
    &dev_attr_asicctl.attr,
    &dev_attr_intack.attr,
    &dev_attr_appset.attr,
    &dev_attr_appclear.attr,
    &dev_attr_apppulse.attr,
    coresight_cti_reg(triginstatus, CTITRIGINSTATUS),
    coresight_cti_reg(trigoutstatus, CTITRIGOUTSTATUS),
    coresight_cti_reg(chinstatus, CTICHINSTATUS),
    coresight_cti_reg(choutstatus, CTICHOUTSTATUS),

    coresight_cti_reg_rw(itctrl, CORESIGHT_ITCTRL),
    coresight_cti_reg(ittrigin, ITTRIGIN),
    coresight_cti_reg(itchin, ITCHIN),
    coresight_cti_reg_rw(ittrigout, ITTRIGOUT),
    coresight_cti_reg_rw(itchout, ITCHOUT),
    coresight_cti_reg(itchoutack, ITCHOUTACK),
    coresight_cti_reg(ittrigoutack, ITTRIGOUTACK),
    coresight_cti_reg_wo(ittriginack, ITTRIGINACK),
    coresight_cti_reg_wo(itchinack, ITCHINACK),

    core::ptr::null_mut(),
    };
    static umode_t coresight_cti_regs_is_visible(struct kobject *kobj,
    struct attribute *attr, int idx)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    if (attr == &dev_attr_asicctl.attr && !drvdata.config.asicctl_impl)
    return 0;
    return attr.mode;
    }
// CTI channel x-trigger programming
    static int
    cti_trig_op_parse(struct device *dev, enum cti_chan_op op,
    enum cti_trig_dir dir, const char *buf, size_t size)
    {
    u32 chan_idx;
    u32 trig_idx;
    int items, err = -EINVAL;
// extract chan idx and trigger idx
    items = sscanf(buf, "%d %d", &chan_idx, &trig_idx);
    if (items == 2) {
    err = cti_channel_trig_op(dev, op, dir, chan_idx, trig_idx);
    if (!err)
    err = size;
    }
    return err;
    }
    static ssize_t trigin_attach_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    return cti_trig_op_parse(dev, CTI_CHAN_ATTACH, CTI_TRIG_IN,
    buf, size);
    }
    static DEVICE_ATTR_WO(trigin_attach);
    static ssize_t trigin_detach_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    return cti_trig_op_parse(dev, CTI_CHAN_DETACH, CTI_TRIG_IN,
    buf, size);
    }
    static DEVICE_ATTR_WO(trigin_detach);
    static ssize_t trigout_attach_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    return cti_trig_op_parse(dev, CTI_CHAN_ATTACH, CTI_TRIG_OUT,
    buf, size);
    }
    static DEVICE_ATTR_WO(trigout_attach);
    static ssize_t trigout_detach_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    return cti_trig_op_parse(dev, CTI_CHAN_DETACH, CTI_TRIG_OUT,
    buf, size);
    }
    static DEVICE_ATTR_WO(trigout_detach);
    static ssize_t chan_gate_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut err: c_int = 0, channel = 0;
    if (kstrtoint(buf, 0, &channel))
    return -EINVAL;
    err = cti_channel_gate_op(dev, CTI_GATE_CHAN_ENABLE, channel);
    return err ? err : size;
    }
    static ssize_t chan_gate_enable_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut ctigate_bitmask: c_ulong = cfg.ctigate;
    if (cfg.ctigate == 0)
    return sprintf(buf, "\n");
    return sysfs_emit(buf, "%*pbl\n", cfg.nr_ctm_channels, &ctigate_bitmask);
    }
    static DEVICE_ATTR_RW(chan_gate_enable);
    static ssize_t chan_gate_disable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut err: c_int = 0, channel = 0;
    if (kstrtoint(buf, 0, &channel))
    return -EINVAL;
    err = cti_channel_gate_op(dev, CTI_GATE_CHAN_DISABLE, channel);
    return err ? err : size;
    }
    static DEVICE_ATTR_WO(chan_gate_disable);
    static int
    chan_op_parse(struct device *dev, enum cti_chan_set_op op, const char *buf)
    {
    let mut err: c_int = 0, channel = 0;
    if (kstrtoint(buf, 0, &channel))
    return -EINVAL;
    err = cti_channel_setop(dev, op, channel);
    return err;
    }
    static ssize_t chan_set_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut err: c_int = chan_op_parse(dev, CTI_CHAN_SET, buf);
    return err ? err : size;
    }
    static DEVICE_ATTR_WO(chan_set);
    static ssize_t chan_clear_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut err: c_int = chan_op_parse(dev, CTI_CHAN_CLR, buf);
    return err ? err : size;
    }
    static DEVICE_ATTR_WO(chan_clear);
    static ssize_t chan_pulse_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    let mut err: c_int = chan_op_parse(dev, CTI_CHAN_PULSE, buf);
    return err ? err : size;
    }
    static DEVICE_ATTR_WO(chan_pulse);
    static ssize_t trig_filter_enable_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u32 val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock)
    val = drvdata.config.trig_filter_enable;
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t trig_filter_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
    drvdata.config.trig_filter_enable = !!val;
    return size;
    }
    static DEVICE_ATTR_RW(trig_filter_enable);
    static ssize_t trigout_filtered_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut nr_trig_max: c_int = cfg.nr_trig_max;
    let mut mask: c_ulong = cfg.trig_out_filter;
    if (mask == 0)
    return 0;
    return sysfs_emit(buf, "%*pbl\n", nr_trig_max, &mask);
    }
    static DEVICE_ATTR_RO(trigout_filtered);
// clear all xtrigger / channel programming
    static ssize_t chan_xtrigs_reset_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    int i;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
// clear the CTI trigger / channel programming registers
    for (i = 0; i < config.nr_trig_max; i++) {
    config.ctiinen[i] = 0;
    config.ctiouten[i] = 0;
    }
// clear the other regs
    config.ctigate = GENMASK(config.nr_ctm_channels - 1, 0);
    config.asicctl = 0;
    config.ctiappset = 0;
    config.ctiinout_sel = 0;
    config.xtrig_rchan_sel = 0;
// if enabled then write through
    if (cti_is_active(config))
    cti_write_all_hw_regs(drvdata);
    return size;
    }
    static DEVICE_ATTR_WO(chan_xtrigs_reset);
//
// Write to select a channel to view, read to display the
// cross triggers for the selected channel.
//
    static ssize_t chan_xtrigs_sel_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    if (kstrtoul(buf, 0, &val))
    return -EINVAL;
    if (val > (drvdata.config.nr_ctm_channels - 1))
    return -EINVAL;
    guard(raw_spinlock_irqsave)(&drvdata.spinlock);
    drvdata.config.xtrig_rchan_sel = val;
    return size;
    }
    static ssize_t chan_xtrigs_sel_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long val;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock)
    val = drvdata.config.xtrig_rchan_sel;
    return sprintf(buf, "%ld\n", val);
    }
    static DEVICE_ATTR_RW(chan_xtrigs_sel);
    static ssize_t chan_xtrigs_in_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut used: c_int = 0, reg_idx;
    let mut nr_trig_max: c_int = drvdata.config.nr_trig_max;
    let mut chan_mask: u32 = BIT(cfg.xtrig_rchan_sel);
    for (reg_idx = 0; reg_idx < nr_trig_max; reg_idx++) {
    if (chan_mask & cfg.ctiinen[reg_idx])
    used += sprintf(buf + used, "%d ", reg_idx);
    }
    used += sprintf(buf + used, "\n");
    return used;
    }
    static DEVICE_ATTR_RO(chan_xtrigs_in);
    static ssize_t chan_xtrigs_out_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut used: c_int = 0, reg_idx;
    let mut nr_trig_max: c_int = drvdata.config.nr_trig_max;
    let mut chan_mask: u32 = BIT(cfg.xtrig_rchan_sel);
    for (reg_idx = 0; reg_idx < nr_trig_max; reg_idx++) {
    if (chan_mask & cfg.ctiouten[reg_idx])
    used += sprintf(buf + used, "%d ", reg_idx);
    }
    used += sprintf(buf + used, "\n");
    return used;
    }
    static DEVICE_ATTR_RO(chan_xtrigs_out);
    static ssize_t print_chan_list(struct device *dev,
    char *buf, bool inuse)
    {
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *config = &drvdata.config;
    int i;
    let mut inuse_bits: c_ulong = 0, chan_mask;
// scan regs to get bitmap of channels in use.
    scoped_guard(raw_spinlock_irqsave, &drvdata.spinlock) {
    for (i = 0; i < config.nr_trig_max; i++) {
    inuse_bits |= config.ctiinen[i];
    inuse_bits |= config.ctiouten[i];
    }
    }
// inverse bits if printing free channels
    if (!inuse)
    inuse_bits = ~inuse_bits;
// list of channels, or 'none'
    chan_mask = GENMASK(config.nr_ctm_channels - 1, 0);
    if (inuse_bits & chan_mask)
    return sysfs_emit(buf, "%*pbl\n", config.nr_ctm_channels, &inuse_bits);
    return sprintf(buf, "\n");
    }
    static ssize_t chan_inuse_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return print_chan_list(dev, buf, true);
    }
    static DEVICE_ATTR_RO(chan_inuse);
    static ssize_t chan_free_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return print_chan_list(dev, buf, false);
    }
    static DEVICE_ATTR_RO(chan_free);
    static struct attribute *coresight_cti_channel_attrs[] = {
    &dev_attr_trigin_attach.attr,
    &dev_attr_trigin_detach.attr,
    &dev_attr_trigout_attach.attr,
    &dev_attr_trigout_detach.attr,
    &dev_attr_trig_filter_enable.attr,
    &dev_attr_trigout_filtered.attr,
    &dev_attr_chan_gate_enable.attr,
    &dev_attr_chan_gate_disable.attr,
    &dev_attr_chan_set.attr,
    &dev_attr_chan_clear.attr,
    &dev_attr_chan_pulse.attr,
    &dev_attr_chan_inuse.attr,
    &dev_attr_chan_free.attr,
    &dev_attr_chan_xtrigs_sel.attr,
    &dev_attr_chan_xtrigs_in.attr,
    &dev_attr_chan_xtrigs_out.attr,
    &dev_attr_chan_xtrigs_reset.attr,
    core::ptr::null_mut(),
    };
// Create the connections trigger groups and attrs dynamically
//
// Each connection has dynamic group triggers<N> + name, trigin/out sigs/types
// attributes, + each device has static nr_trigger_cons giving the number
// of groups. e.g. in sysfs:-
// /cti_<name>/triggers0
// /cti_<name>/triggers1
// /cti_<name>/nr_trigger_cons
// where nr_trigger_cons = 2
//
    static ssize_t con_name_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *ext_attr =
    container_of(attr, struct dev_ext_attribute, attr);
    struct cti_trig_con *con = (struct cti_trig_con *)ext_attr.var;
    return sprintf(buf, "%s\n", con.con_dev_name);
    }
    static ssize_t trigin_sig_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *ext_attr =
    container_of(attr, struct dev_ext_attribute, attr);
    struct cti_trig_con *con = (struct cti_trig_con *)ext_attr.var;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut mask: c_ulong = con.con_in.used_mask;
    return sysfs_emit(buf, "%*pbl\n", cfg.nr_trig_max, &mask);
    }
    static ssize_t trigout_sig_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *ext_attr =
    container_of(attr, struct dev_ext_attribute, attr);
    struct cti_trig_con *con = (struct cti_trig_con *)ext_attr.var;
    struct cti_drvdata *drvdata = dev_get_drvdata(dev.parent);
    struct cti_config *cfg = &drvdata.config;
    let mut mask: c_ulong = con.con_out.used_mask;
    return sysfs_emit(buf, "%*pbl\n", cfg.nr_trig_max, &mask);
    }
// convert a sig type id to a name
    static const char *
    cti_sig_type_name(struct cti_trig_con *con, int used_count, bool in)
    {
    let mut idx: c_int = 0;
    struct cti_trig_grp *grp = in ? con.con_in : con.con_out;
    if (used_count < grp.nr_sigs)
    idx = grp.sig_types[used_count];
    return sig_type_names[idx];
    }
    static ssize_t trigin_type_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *ext_attr =
    container_of(attr, struct dev_ext_attribute, attr);
    struct cti_trig_con *con = (struct cti_trig_con *)ext_attr.var;
    int sig_idx, used = 0;
    const char *name;
    for (sig_idx = 0; sig_idx < con.con_in.nr_sigs; sig_idx++) {
    name = cti_sig_type_name(con, sig_idx, true);
    used += sprintf(buf + used, "%s ", name);
    }
    used += sprintf(buf + used, "\n");
    return used;
    }
    static ssize_t trigout_type_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *ext_attr =
    container_of(attr, struct dev_ext_attribute, attr);
    struct cti_trig_con *con = (struct cti_trig_con *)ext_attr.var;
    int sig_idx, used = 0;
    const char *name;
    for (sig_idx = 0; sig_idx < con.con_out.nr_sigs; sig_idx++) {
    name = cti_sig_type_name(con, sig_idx, false);
    used += sprintf(buf + used, "%s ", name);
    }
    used += sprintf(buf + used, "\n");
    return used;
    }
//
// Array of show function names declared above to allow selection
// for the connection attributes
//
    static p_show_fn show_fns[CTI_CON_ATTR_MAX] = {
    con_name_show,
    trigin_sig_show,
    trigout_sig_show,
    trigin_type_show,
    trigout_type_show,
    };
    static int cti_create_con_sysfs_attr(struct device *dev,
    struct cti_trig_con *con,
    enum cti_conn_attr_type attr_type,
    int attr_idx)
    {
    struct dev_ext_attribute *eattr;
    char *name;
    eattr = devm_kzalloc(dev, sizeof(struct dev_ext_attribute),
    GFP_KERNEL);
    if (eattr) {
    name = devm_kstrdup(dev, con_attr_names[attr_type],
    GFP_KERNEL);
    if (name) {
// fill out the underlying attribute struct
    eattr.attr.attr.name = name;
    eattr.attr.attr.mode = 0444;
// now the device_attribute struct
    eattr.attr.show = show_fns[attr_type];
    } else {
    return -ENOMEM;
    }
    } else {
    return -ENOMEM;
    }
    eattr.var = con;
    con.con_attrs[attr_idx] = &eattr.attr.attr;
//
// Initialize the dynamically allocated attribute
// to avoid LOCKDEP splat. See include/linux/sysfs.h
// for more details.
//
    sysfs_attr_init(con.con_attrs[attr_idx]);
    return 0;
    }
    static struct attribute_group *
    cti_create_con_sysfs_group(struct device *dev, struct cti_device *ctidev,
    int con_idx, struct cti_trig_con *tc)
    {
    struct attribute_group *group = core::ptr::null_mut();
    int grp_idx;
    group = devm_kzalloc(dev, sizeof(struct attribute_group), GFP_KERNEL);
    if (!group)
    return core::ptr::null_mut();
    group.name = devm_kasprintf(dev, GFP_KERNEL, "triggers%d", con_idx);
    if (!group.name)
    return core::ptr::null_mut();
    grp_idx = con_idx + CORESIGHT_CTI_STATIC_GROUPS_MAX - 1;
    ctidev.con_groups[grp_idx] = group;
    tc.attr_group = group;
    return group;
    }
// create a triggers connection group and the attributes for that group
    static int cti_create_con_attr_set(struct device *dev, int con_idx,
    struct cti_device *ctidev,
    struct cti_trig_con *tc)
    {
    struct attribute_group *attr_group = core::ptr::null_mut();
    let mut attr_idx: c_int = 0;
    let mut err: c_int = -ENOMEM;
    attr_group = cti_create_con_sysfs_group(dev, ctidev, con_idx, tc);
    if (!attr_group)
    return -ENOMEM;
// allocate NULL terminated array of attributes
    tc.con_attrs = devm_kcalloc(dev, CTI_CON_ATTR_MAX + 1,
    sizeof(struct attribute *), GFP_KERNEL);
    if (!tc.con_attrs)
    return -ENOMEM;
    err = cti_create_con_sysfs_attr(dev, tc, CTI_CON_ATTR_NAME,
    attr_idx++);
    if (err)
    return err;
    if (tc.con_in.nr_sigs > 0) {
    err = cti_create_con_sysfs_attr(dev, tc,
    CTI_CON_ATTR_TRIGIN_SIG,
    attr_idx++);
    if (err)
    return err;
    err = cti_create_con_sysfs_attr(dev, tc,
    CTI_CON_ATTR_TRIGIN_TYPES,
    attr_idx++);
    if (err)
    return err;
    }
    if (tc.con_out.nr_sigs > 0) {
    err = cti_create_con_sysfs_attr(dev, tc,
    CTI_CON_ATTR_TRIGOUT_SIG,
    attr_idx++);
    if (err)
    return err;
    err = cti_create_con_sysfs_attr(dev, tc,
    CTI_CON_ATTR_TRIGOUT_TYPES,
    attr_idx++);
    if (err)
    return err;
    }
    attr_group.attrs = tc.con_attrs;
    return 0;
    }
// create the array of group pointers for the CTI sysfs groups
#[no_mangle]
unsafe extern "C" fn cti_create_cons_groups(dev: *mut device, ctidev: *mut cti_device) -> c_int {
    static int cti_create_cons_groups(struct device *dev, struct cti_device *ctidev)
    {
    int nr_groups;
// nr groups = dynamic + static + NULL terminator
    nr_groups = ctidev.nr_trig_con + CORESIGHT_CTI_STATIC_GROUPS_MAX;
    ctidev.con_groups = devm_kcalloc(dev, nr_groups,
    sizeof(struct attribute_group *),
    GFP_KERNEL);
    if (!ctidev.con_groups)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cti_create_cons_sysfs(dev: *mut device, drvdata: *mut cti_drvdata) -> c_int {
    int cti_create_cons_sysfs(struct device *dev, struct cti_drvdata *drvdata)
    {
    struct cti_device *ctidev = &drvdata.ctidev;
    int err, con_idx = 0, i;
    struct cti_trig_con *tc;
    err = cti_create_cons_groups(dev, ctidev);
    if (err)
    return err;
// populate first locations with the static set of groups
    for (i = 0; i < (CORESIGHT_CTI_STATIC_GROUPS_MAX - 1); i++)
    ctidev.con_groups[i] = coresight_cti_groups[i];
// add dynamic set for each connection
    list_for_each_entry(tc, &ctidev.trig_cons, node) {
    err = cti_create_con_attr_set(dev, con_idx++, ctidev, tc);
    if (err)
    break;
    }
    return err;
    }
// attribute and group sysfs tables.
    static const struct attribute_group coresight_cti_group = {
    .attrs = coresight_cti_attrs,
    };
    static const struct attribute_group coresight_cti_mgmt_group = {
    .attrs = coresight_cti_mgmt_attrs,
    .name = "mgmt",
    };
    static const struct attribute_group coresight_cti_regs_group = {
    .attrs = coresight_cti_regs_attrs,
    .is_visible = coresight_cti_regs_is_visible,
    .name = "regs",
    };
    static const struct attribute_group coresight_cti_channels_group = {
    .attrs = coresight_cti_channel_attrs,
    .name = "channels",
    };
    const struct attribute_group *
    coresight_cti_groups[CORESIGHT_CTI_STATIC_GROUPS_MAX] = {
    &coresight_cti_group,
    &coresight_cti_mgmt_group,
    &coresight_cti_regs_group,
    &coresight_cti_channels_group,
    core::ptr::null_mut(),
    };
