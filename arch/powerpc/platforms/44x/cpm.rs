//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/cpm.c
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
// PowerPC 4xx Clock and Power Management
//
// Copyright (C) 2010, Applied Micro Circuits Corporation
// Victor Gallardo (vgallardo@apm.com)
//
// Based on arch/powerpc/platforms/44x/idle.c:
// Jerone Young <jyoung5@us.ibm.com>
// Copyright 2008 IBM Corp.
//
// Based on arch/powerpc/sysdev/fsl_pmc.c:
// Anton Vorontsov <avorontsov@ru.mvista.com>
// Copyright 2009  MontaVista Software, Inc.
//
// See file CREDITS for list of people who contributed to this
// project.
//

pub const CPM_ER: c_int = 0;
pub const CPM_FR: c_int = 1;
pub const CPM_SR: c_int = 2;
pub const CPM_IDLE_WAIT: c_int = 0;
pub const CPM_IDLE_DOZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm {
    pub dcr_host: dcr_host_t,
    pub dcr_offset: [c_uint; 3],
    pub powersave_off: c_uint,
    pub unused: c_uint,
    pub idle_doze: c_uint,
    pub standby: c_uint,
    pub suspend: c_uint,
}

    static struct cpm cpm;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_idle_mode {
    pub enabled: c_uint,
    pub name: *const c_char,
}

    static struct cpm_idle_mode idle_mode[] = {
    [CPM_IDLE_WAIT] = { 1, "wait" }, /* default */
    [CPM_IDLE_DOZE] = { 0, "doze" },
    };
#[no_mangle]
unsafe extern "C" fn cpm_set(cpm_reg: c_uint, mask: c_uint) -> c_uint {
    static unsigned int cpm_set(unsigned int cpm_reg, unsigned int mask)
    {
    unsigned int value;
// CPM controller supports 3 different types of sleep interface
// known as class 1, 2 and 3. For class 1 units, they are
// unconditionally put to sleep when the corresponding CPM bit is
// set. For class 2 and 3 units this is not case; if they can be
// put to sleep, they will. Here we do not verify, we just
// set them and expect them to eventually go off when they can.
//
    value = dcr_read(cpm.dcr_host, cpm.dcr_offset[cpm_reg]);
    dcr_write(cpm.dcr_host, cpm.dcr_offset[cpm_reg], value | mask);
// return old state, to restore later if needed
    return value;
    }
#[no_mangle]
unsafe extern "C" fn cpm_idle_wait() {
    static void cpm_idle_wait(void)
    {
    unsigned long msr_save;
// save off initial state
    msr_save = mfmsr();
// sync required when CPM0_ER[CPU] is set
    mb();
// set wait state MSR
    mtmsr(msr_save|MSR_WE|MSR_EE|MSR_CE|MSR_DE);
    isync();
// return to initial state
    mtmsr(msr_save);
    isync();
    }
#[no_mangle]
unsafe extern "C" fn cpm_idle_sleep(mask: c_uint) {
    static void cpm_idle_sleep(unsigned int mask)
    {
    unsigned int er_save;
// update CPM_ER state
    er_save = cpm_set(CPM_ER, mask);
// go to wait state so that CPM0_ER[CPU] can take effect
    cpm_idle_wait();
// restore CPM_ER state
    dcr_write(cpm.dcr_host, cpm.dcr_offset[CPM_ER], er_save);
    }
#[no_mangle]
unsafe extern "C" fn cpm_idle_doze() {
    static void cpm_idle_doze(void)
    {
    cpm_idle_sleep(cpm.idle_doze);
    }
#[no_mangle]
unsafe extern "C" fn cpm_idle_config(mode: c_int) {
    static void cpm_idle_config(int mode)
    {
    int i;
    if (idle_mode[mode].enabled)
    return;
    for (i = 0; i < ARRAY_SIZE(idle_mode); i++)
    idle_mode[i].enabled = 0;
    idle_mode[mode].enabled = 1;
    }
    static ssize_t cpm_idle_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    char *s = buf;
    int i;
    for (i = 0; i < ARRAY_SIZE(idle_mode); i++) {
    if (idle_mode[i].enabled)
    s += sprintf(s, "[%s] ", idle_mode[i].name);
    else
    s += sprintf(s, "%s ", idle_mode[i].name);
    }
// (s-1) = '\n'; /* convert the last space to a newline
    return s - buf;
    }
    static ssize_t cpm_idle_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t n)
    {
    int i;
    char *p;
    int len;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    for (i = 0; i < ARRAY_SIZE(idle_mode); i++) {
    if (strncmp(buf, idle_mode[i].name, len) == 0) {
    cpm_idle_config(i);
    return n;
    }
    }
    return -EINVAL;
    }
    static struct kobj_attribute cpm_idle_attr =
    __ATTR(idle, 0644, cpm_idle_show, cpm_idle_store);
#[no_mangle]
unsafe extern "C" fn cpm_idle_config_sysfs() -> void __init {
    static void __init cpm_idle_config_sysfs(void)
    {
    struct device *dev;
    unsigned long ret;
    dev = get_cpu_device(0);
    ret = sysfs_create_file(&dev.kobj,
    &cpm_idle_attr.attr);
    if (ret)
    printk(KERN_WARNING
    "cpm: failed to create idle sysfs entry\n");
    }
#[no_mangle]
unsafe extern "C" fn cpm_idle() {
    static void cpm_idle(void)
    {
    if (idle_mode[CPM_IDLE_DOZE].enabled)
    cpm_idle_doze();
    else
    cpm_idle_wait();
    }
#[no_mangle]
unsafe extern "C" fn cpm_suspend_valid(state: suspend_state_t) -> c_int {
    static int cpm_suspend_valid(suspend_state_t state)
    {
    switch (state) {
    case PM_SUSPEND_STANDBY:
    return !!cpm.standby;
    case PM_SUSPEND_MEM:
    return !!cpm.suspend;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn cpm_suspend_standby(mask: c_uint) {
    static void cpm_suspend_standby(unsigned int mask)
    {
    unsigned long tcr_save;
// disable decrement interrupt
    tcr_save = mfspr(SPRN_TCR);
    mtspr(SPRN_TCR, tcr_save & ~TCR_DIE);
// go to sleep state
    cpm_idle_sleep(mask);
// restore decrement interrupt
    mtspr(SPRN_TCR, tcr_save);
    }
#[no_mangle]
unsafe extern "C" fn cpm_suspend_enter(state: suspend_state_t) -> c_int {
    static int cpm_suspend_enter(suspend_state_t state)
    {
    switch (state) {
    case PM_SUSPEND_STANDBY:
    cpm_suspend_standby(cpm.standby);
    break;
    case PM_SUSPEND_MEM:
    cpm_suspend_standby(cpm.suspend);
    break;
    }
    return 0;
    }
    static const struct platform_suspend_ops cpm_suspend_ops = {
    .valid		= cpm_suspend_valid,
    .enter		= cpm_suspend_enter,
    };
    static int __init cpm_get_uint_property(struct device_node *np,
    const char *name)
    {
    int len;
    const unsigned int *prop = of_get_property(np, name, &len);
    if (prop == core::ptr::null_mut() || len < sizeof(u32))
    return 0;
    return *prop;
    }
#[no_mangle]
unsafe extern "C" fn cpm_init() -> int __init {
    static int __init cpm_init(void)
    {
    struct device_node *np;
    int dcr_base, dcr_len;
    let mut ret: c_int = 0;
    if (!cpm.powersave_off) {
    cpm_idle_config(CPM_IDLE_WAIT);
    ppc_md.power_save = &cpm_idle;
    }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,cpm");
    if (!np) {
    ret = -EINVAL;
    goto out;
    }
    dcr_base = dcr_resource_start(np, 0);
    dcr_len = dcr_resource_len(np, 0);
    if (dcr_base == 0 || dcr_len == 0) {
    printk(KERN_ERR "cpm: could not parse dcr property for %pOF\n",
    np);
    ret = -EINVAL;
    goto node_put;
    }
    cpm.dcr_host = dcr_map(np, dcr_base, dcr_len);
    if (!DCR_MAP_OK(cpm.dcr_host)) {
    printk(KERN_ERR "cpm: failed to map dcr property for %pOF\n",
    np);
    ret = -EINVAL;
    goto node_put;
    }
// All 4xx SoCs with a CPM controller have one of two
// different order for the CPM registers. Some have the
// CPM registers in the following order (ER,FR,SR). The
// others have them in the following order (SR,ER,FR).
//
    if (cpm_get_uint_property(np, "er-offset") == 0) {
    cpm.dcr_offset[CPM_ER] = 0;
    cpm.dcr_offset[CPM_FR] = 1;
    cpm.dcr_offset[CPM_SR] = 2;
    } else {
    cpm.dcr_offset[CPM_ER] = 1;
    cpm.dcr_offset[CPM_FR] = 2;
    cpm.dcr_offset[CPM_SR] = 0;
    }
// Now let's see what IPs to turn off for the following modes
    cpm.unused = cpm_get_uint_property(np, "unused-units");
    cpm.idle_doze = cpm_get_uint_property(np, "idle-doze");
    cpm.standby = cpm_get_uint_property(np, "standby");
    cpm.suspend = cpm_get_uint_property(np, "suspend");
// If some IPs are unused let's turn them off now
    if (cpm.unused) {
    cpm_set(CPM_ER, cpm.unused);
    cpm_set(CPM_FR, cpm.unused);
    }
// Now let's export interfaces
    if (!cpm.powersave_off && cpm.idle_doze)
    cpm_idle_config_sysfs();
    if (cpm.standby || cpm.suspend)
    suspend_set_ops(&cpm_suspend_ops);
    node_put:
    of_node_put(np);
    out:
    return ret;
    }
    late_initcall(cpm_init);
#[no_mangle]
unsafe extern "C" fn cpm_powersave_off(arg: *mut c_char) -> int __init {
    static int __init cpm_powersave_off(char *arg)
    {
    cpm.powersave_off = 1;
    return 1;
    }
    __setup("powersave=off", cpm_powersave_off);
