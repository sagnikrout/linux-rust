//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/fam15h_power.c
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
// fam15h_power.c - AMD Family 15h processor power monitoring
//
// Copyright (c) 2011-2016 Advanced Micro Devices, Inc.
// Author: Andreas Herrmann <herrmann.der.user@googlemail.com>
//

    MODULE_DESCRIPTION("AMD Family 15h CPU processor power monitor");
    MODULE_AUTHOR("Andreas Herrmann <herrmann.der.user@googlemail.com>");
    MODULE_LICENSE("GPL");
// D18F3
pub const REG_NORTHBRIDGE_CAP: c_uint = 0xe8;
// D18F4
pub const REG_PROCESSOR_TDP: c_uint = 0x1b8;
// D18F5
pub const REG_TDP_RUNNING_AVERAGE: c_uint = 0xe0;
pub const REG_TDP_LIMIT3: c_uint = 0xe8;
pub const FAM15H_MIN_NUM_ATTRS: c_int = 2;
pub const FAM15H_NUM_GROUPS: c_int = 2;
pub const MAX_CUS: c_int = 8;
// set maximum interval as 1 second
pub const MAX_INTERVAL: c_int = 1000;
pub const PCI_DEVICE_ID_AMD_15H_M70H_NB_F4: c_uint = 0x15b4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fam15h_power_data {
    pub pdev: *mut pci_dev,
    pub tdp_to_watts: c_uint,
    pub base_tdp: c_uint,
    pub processor_pwr_watts: c_uint,
    pub cpu_pwr_sample_ratio: c_uint,
    pub groups: [*const attribute_group; FAM15H_NUM_GROUPS],
    pub group: attribute_group,
// maximum accumulated power of a compute unit
    pub max_cu_acc_power: u64,
// accumulated power of the compute units
    pub cu_acc_power: [u64; MAX_CUS],
// performance timestamp counter
    pub cpu_sw_pwr_ptsc: [u64; MAX_CUS],
// online/offline status of current compute unit
    pub cu_on: [c_int; MAX_CUS],
    pub power_period: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn is_carrizo_or_later() -> bool {
    static bool is_carrizo_or_later(void)
    {
    return boot_cpu_data.x86 == 0x15 && boot_cpu_data.x86_model >= 0x60;
    }
    static ssize_t power1_input_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    u32 val, tdp_limit, running_avg_range;
    s32 running_avg_capture;
    u64 curr_pwr_watts;
    struct fam15h_power_data *data = dev_get_drvdata(dev);
    struct pci_dev *f4 = data.pdev;
    pci_bus_read_config_dword(f4.bus, PCI_DEVFN(PCI_SLOT(f4.devfn), 5),
    REG_TDP_RUNNING_AVERAGE, &val);
//
// On Carrizo and later platforms, TdpRunAvgAccCap bit field
// is extended to 4:31 from 4:25.
//
    if (is_carrizo_or_later()) {
    running_avg_capture = val >> 4;
    running_avg_capture = sign_extend32(running_avg_capture, 27);
    } else {
    running_avg_capture = (val >> 4) & 0x3fffff;
    running_avg_capture = sign_extend32(running_avg_capture, 21);
    }
    running_avg_range = (val & 0xf) + 1;
    pci_bus_read_config_dword(f4.bus, PCI_DEVFN(PCI_SLOT(f4.devfn), 5),
    REG_TDP_LIMIT3, &val);
//
// On Carrizo and later platforms, ApmTdpLimit bit field
// is extended to 16:31 from 16:28.
//
    if (is_carrizo_or_later())
    tdp_limit = val >> 16;
    else
    tdp_limit = (val >> 16) & 0x1fff;
    curr_pwr_watts = ((u64)(tdp_limit +
    data.base_tdp)) << running_avg_range;
    curr_pwr_watts -= running_avg_capture;
    curr_pwr_watts *= data.tdp_to_watts;
//
// Convert to microWatt
//
// power is in Watt provided as fixed point integer with
// scaling factor 1/(2^16).  For conversion we use
// (10^6)/(2^16) = 15625/(2^10)
//
    curr_pwr_watts = (curr_pwr_watts * 15625) >> (10 + running_avg_range);
    return sprintf(buf, "%u\n", (unsigned int) curr_pwr_watts);
    }
    static DEVICE_ATTR_RO(power1_input);
    static ssize_t power1_crit_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct fam15h_power_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", data.processor_pwr_watts);
    }
    static DEVICE_ATTR_RO(power1_crit);
#[no_mangle]
unsafe extern "C" fn do_read_registers_on_cu(_data: *mut c_void) {
    static void do_read_registers_on_cu(void *_data)
    {
    struct fam15h_power_data *data = _data;
    int cu;
//
// With the new x86 topology modelling, cpu core id actually
// is compute unit id.
//
    cu = topology_core_id(smp_processor_id());
    rdmsrq_safe(MSR_F15H_CU_PWR_ACCUMULATOR, &data.cu_acc_power[cu]);
    rdmsrq_safe(MSR_F15H_PTSC, &data.cpu_sw_pwr_ptsc[cu]);
    data.cu_on[cu] = 1;
    }
//
// This function is only able to be called when CPUID
// Fn8000_0007:EDX[12] is set.
//
#[no_mangle]
unsafe extern "C" fn read_registers(data: *mut fam15h_power_data) -> c_int {
    static int read_registers(struct fam15h_power_data *data)
    {
    int core, this_core;
    cpumask_var_t mask;
    int ret, cpu;
    ret = zalloc_cpumask_var(&mask, GFP_KERNEL);
    if (!ret)
    return -ENOMEM;
    memset(data.cu_on, 0, sizeof(int) * MAX_CUS);
    cpus_read_lock();
//
// Choose the first online core of each compute unit, and then
// read their MSR value of power and ptsc in a single IPI,
// because the MSR value of CPU core represent the compute
// unit's.
//
    core = -1;
    for_each_online_cpu(cpu) {
    this_core = topology_core_id(cpu);
    if (this_core == core)
    continue;
    core = this_core;
// get any CPU on this compute unit
    cpumask_set_cpu(cpumask_any(topology_sibling_cpumask(cpu)), mask);
    }
    on_each_cpu_mask(mask, do_read_registers_on_cu, data, true);
    cpus_read_unlock();
    free_cpumask_var(mask);
    return 0;
    }
    static ssize_t power1_average_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct fam15h_power_data *data = dev_get_drvdata(dev);
    u64 prev_cu_acc_power[MAX_CUS], prev_ptsc[MAX_CUS],
    jdelta[MAX_CUS];
    u64 tdelta, avg_acc;
    int cu, cu_num, ret;
    signed long leftover;
//
// With the new x86 topology modelling, x86_max_cores is the
// compute unit number.
//
    cu_num = topology_num_cores_per_package();
    ret = read_registers(data);
    if (ret)
    return 0;
    for (cu = 0; cu < cu_num; cu++) {
    prev_cu_acc_power[cu] = data.cu_acc_power[cu];
    prev_ptsc[cu] = data.cpu_sw_pwr_ptsc[cu];
    }
    leftover = schedule_timeout_interruptible(msecs_to_jiffies(data.power_period));
    if (leftover)
    return 0;
    ret = read_registers(data);
    if (ret)
    return 0;
    for (cu = 0, avg_acc = 0; cu < cu_num; cu++) {
// check if current compute unit is online
    if (data.cu_on[cu] == 0)
    continue;
    if (data.cu_acc_power[cu] < prev_cu_acc_power[cu]) {
    jdelta[cu] = data.max_cu_acc_power + data.cu_acc_power[cu];
    jdelta[cu] -= prev_cu_acc_power[cu];
    } else {
    jdelta[cu] = data.cu_acc_power[cu] - prev_cu_acc_power[cu];
    }
    tdelta = data.cpu_sw_pwr_ptsc[cu] - prev_ptsc[cu];
    jdelta[cu] *= data.cpu_pwr_sample_ratio * 1000;
    do_div(jdelta[cu], tdelta);
// the unit is microWatt
    avg_acc += jdelta[cu];
    }
    return sprintf(buf, "%llu\n", (unsigned long long)avg_acc);
    }
    static DEVICE_ATTR_RO(power1_average);
    static ssize_t power1_average_interval_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct fam15h_power_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%lu\n", data.power_period);
    }
    static ssize_t power1_average_interval_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct fam15h_power_data *data = dev_get_drvdata(dev);
    unsigned long temp;
    int ret;
    ret = kstrtoul(buf, 10, &temp);
    if (ret)
    return ret;
    if (temp > MAX_INTERVAL)
    return -EINVAL;
// the interval value should be greater than 0
    if (temp <= 0)
    return -EINVAL;
    data.power_period = temp;
    return count;
    }
    static DEVICE_ATTR_RW(power1_average_interval);
    static int fam15h_power_init_attrs(struct pci_dev *pdev,
    struct fam15h_power_data *data)
    {
    let mut n: c_int = FAM15H_MIN_NUM_ATTRS;
    struct attribute **fam15h_power_attrs;
    struct cpuinfo_x86 *c = &boot_cpu_data;
    if (c.x86 == 0x15 &&
    (c.x86_model <= 0xf ||
    (c.x86_model >= 0x60 && c.x86_model <= 0x7f)))
    n += 1;
// check if processor supports accumulated power
    if (boot_cpu_has(X86_FEATURE_ACC_POWER))
    n += 2;
    fam15h_power_attrs = devm_kcalloc(&pdev.dev, n,
    sizeof(*fam15h_power_attrs),
    GFP_KERNEL);
    if (!fam15h_power_attrs)
    return -ENOMEM;
    n = 0;
    fam15h_power_attrs[n++] = &dev_attr_power1_crit.attr;
    if (c.x86 == 0x15 &&
    (c.x86_model <= 0xf ||
    (c.x86_model >= 0x60 && c.x86_model <= 0x7f)))
    fam15h_power_attrs[n++] = &dev_attr_power1_input.attr;
    if (boot_cpu_has(X86_FEATURE_ACC_POWER)) {
    fam15h_power_attrs[n++] = &dev_attr_power1_average.attr;
    fam15h_power_attrs[n++] = &dev_attr_power1_average_interval.attr;
    }
    data.group.attrs = fam15h_power_attrs;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn should_load_on_this_node(f4: *mut pci_dev) -> bool {
    static bool should_load_on_this_node(struct pci_dev *f4)
    {
    u32 val;
    pci_bus_read_config_dword(f4.bus, PCI_DEVFN(PCI_SLOT(f4.devfn), 3),
    REG_NORTHBRIDGE_CAP, &val);
    if ((val & BIT(29)) && ((val >> 30) & 3))
    return false;
    return true;
    }
//
// Newer BKDG versions have an updated recommendation on how to properly
// initialize the running average range (was: 0xE, now: 0x9). This avoids
// counter saturations resulting in bogus power readings.
// We correct this value ourselves to cope with older BIOSes.
//
    static const struct pci_device_id affected_device[] = {
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_15H_NB_F4) },
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn tweak_runavg_range(pdev: *mut pci_dev) {
    static void tweak_runavg_range(struct pci_dev *pdev)
    {
    u32 val;
//
// let this quirk apply only to the current version of the
// northbridge, since future versions may change the behavior
//
    if (!pci_match_id(affected_device, pdev))
    return;
    pci_bus_read_config_dword(pdev.bus,
    PCI_DEVFN(PCI_SLOT(pdev.devfn), 5),
    REG_TDP_RUNNING_AVERAGE, &val);
    if ((val & 0xf) != 0xe)
    return;
    val &= ~0xf;
    val |=  0x9;
    pci_bus_write_config_dword(pdev.bus,
    PCI_DEVFN(PCI_SLOT(pdev.devfn), 5),
    REG_TDP_RUNNING_AVERAGE, val);
    }
#[no_mangle]
unsafe extern "C" fn fam15h_power_resume(dev: *mut device) -> c_int {
    static int fam15h_power_resume(struct device *dev)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    tweak_runavg_range(pdev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(fam15h_power_ops, core::ptr::null_mut(), fam15h_power_resume);
    static int fam15h_power_init_data(struct pci_dev *f4,
    struct fam15h_power_data *data)
    {
    u32 val;
    u64 tmp;
    int ret;
    pci_read_config_dword(f4, REG_PROCESSOR_TDP, &val);
    data.base_tdp = val >> 16;
    tmp = val & 0xffff;
    pci_bus_read_config_dword(f4.bus, PCI_DEVFN(PCI_SLOT(f4.devfn), 5),
    REG_TDP_LIMIT3, &val);
    data.tdp_to_watts = ((val & 0x3ff) << 6) | ((val >> 10) & 0x3f);
    tmp *= data.tdp_to_watts;
// result not allowed to be >= 256W
    if ((tmp >> 16) >= 256)
    dev_warn(&f4.dev,
    "Bogus value for ProcessorPwrWatts (processor_pwr_watts>=%u)\n",
    (unsigned int) (tmp >> 16));
// convert to microWatt
    data.processor_pwr_watts = (tmp * 15625) >> 10;
    ret = fam15h_power_init_attrs(f4, data);
    if (ret)
    return ret;
// CPUID Fn8000_0007:EDX[12] indicates to support accumulated power
    if (!boot_cpu_has(X86_FEATURE_ACC_POWER))
    return 0;
//
// determine the ratio of the compute unit power accumulator
// sample period to the PTSC counter period by executing CPUID
// Fn8000_0007:ECX
//
    data.cpu_pwr_sample_ratio = cpuid_ecx(0x80000007);
    if (rdmsrq_safe(MSR_F15H_CU_MAX_PWR_ACCUMULATOR, &tmp)) {
    pr_err("Failed to read max compute unit power accumulator MSR\n");
    return -ENODEV;
    }
    data.max_cu_acc_power = tmp;
//
// Milliseconds are a reasonable interval for the measurement.
// But it shouldn't set too long here, because several seconds
// would cause the read function to hang. So set default
// interval as 10 ms.
//
    data.power_period = 10;
    return read_registers(data);
    }
    static int fam15h_power_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct fam15h_power_data *data;
    struct device *dev = &pdev.dev;
    struct device *hwmon_dev;
    int ret;
//
// though we ignore every other northbridge, we still have to
// do the tweaking on _each_ node in MCM processors as the counters
// are working hand-in-hand
//
    tweak_runavg_range(pdev);
    if (!should_load_on_this_node(pdev))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(struct fam15h_power_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    ret = fam15h_power_init_data(pdev, data);
    if (ret)
    return ret;
    data.pdev = pdev;
    data.groups[0] = &data.group;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, "fam15h_power",
    data,
    &data.groups[0]);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct pci_device_id fam15h_power_id_table[] = {
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_15H_NB_F4) },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_15H_M30H_NB_F4) },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_15H_M60H_NB_F4) },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_15H_M70H_NB_F4) },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_16H_NB_F4) },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_16H_M30H_NB_F4) },
    {}
    };
    MODULE_DEVICE_TABLE(pci, fam15h_power_id_table);
    static struct pci_driver fam15h_power_driver = {
    .name = "fam15h_power",
    .id_table = fam15h_power_id_table,
    .probe = fam15h_power_probe,
    .driver.pm = &fam15h_power_ops,
    };
    module_pci_driver(fam15h_power_driver);
