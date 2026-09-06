//! Automatically rewritten from C to Rust
//! Source: drivers/bus/vexpress-config.c
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
// Copyright (C) 2014 ARM Limited
//

pub const SYS_MISC: c_uint = 0x0;

pub const SYS_PROCID0: c_uint = 0x24;
pub const SYS_PROCID1: c_uint = 0x28;
pub const SYS_HBI_MASK: c_uint = 0xfff;
pub const SYS_PROCIDx_HBI_SHIFT: c_int = 0;
pub const SYS_CFGDATA: c_uint = 0x40;
pub const SYS_CFGCTRL: c_uint = 0x44;

pub const SYS_CFGSTAT: c_uint = 0x48;

pub const VEXPRESS_SITE_MB: c_int = 0;
pub const VEXPRESS_SITE_DB1: c_int = 1;
pub const VEXPRESS_SITE_DB2: c_int = 2;
pub const VEXPRESS_SITE_MASTER: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_syscfg {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub funcs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_syscfg_func {
    pub list: list_head,
    pub syscfg: *mut vexpress_syscfg,
    pub regmap: *mut regmap,
    pub num_templates: c_int,
    pub /: *mut *mut u32 template[] __counted_by(num_templates); / Keep it last!,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_config_bridge_ops {
    pub context): *mut *mut *mut *mut regmap  (regmap_init)(device dev, void,
    pub context): *mut *mut *mut void (regmap_exit)(struct regmap regmap, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_config_bridge {
    pub ops: *mut vexpress_config_bridge_ops,
    pub context: *mut c_void,
}

    static DEFINE_MUTEX(vexpress_config_mutex);
    let mut vexpress_config_site_master: static u32 = VEXPRESS_SITE_MASTER;
#[no_mangle]
unsafe extern "C" fn vexpress_config_set_master(site: u32) {
    static void vexpress_config_set_master(u32 site)
    {
    vexpress_config_site_master = site;
    }
#[no_mangle]
unsafe extern "C" fn vexpress_config_lock(arg: *mut c_void) {
    static void vexpress_config_lock(void *arg)
    {
    mutex_lock(&vexpress_config_mutex);
    }
#[no_mangle]
unsafe extern "C" fn vexpress_config_unlock(arg: *mut c_void) {
    static void vexpress_config_unlock(void *arg)
    {
    mutex_unlock(&vexpress_config_mutex);
    }
    static void vexpress_config_find_prop(struct device_node *node,
    const char *name, u32 *val)
    {
// Default value
// val = 0;
    of_node_get(node);
    while (node) {
    if (of_property_read_u32(node, name, val) == 0) {
    of_node_put(node);
    return;
    }
    node = of_get_next_parent(node);
    }
    }
    static int vexpress_config_get_topo(struct device_node *node, u32 *site,
    u32 *position, u32 *dcc)
    {
    vexpress_config_find_prop(node, "arm,vexpress,site", site);
    if (*site == VEXPRESS_SITE_MASTER)
// site = vexpress_config_site_master;
    if (WARN_ON(vexpress_config_site_master == VEXPRESS_SITE_MASTER))
    return -EINVAL;
    vexpress_config_find_prop(node, "arm,vexpress,position", position);
    vexpress_config_find_prop(node, "arm,vexpress,dcc", dcc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vexpress_config_devres_release(dev: *mut device, res: *mut c_void) {
    static void vexpress_config_devres_release(struct device *dev, void *res)
    {
    struct vexpress_config_bridge *bridge = dev_get_drvdata(dev.parent);
    struct regmap *regmap = res;
    bridge.ops.regmap_exit(regmap, bridge.context);
    }
    struct regmap *devm_regmap_init_vexpress_config(struct device *dev)
    {
    struct vexpress_config_bridge *bridge;
    struct regmap *regmap;
    struct regmap **res;
    bridge = dev_get_drvdata(dev.parent);
    if (WARN_ON(!bridge))
    return ERR_PTR(-EINVAL);
    res = devres_alloc(vexpress_config_devres_release, sizeof(*res),
    GFP_KERNEL);
    if (!res)
    return ERR_PTR(-ENOMEM);
    regmap = (bridge.ops.regmap_init)(dev, bridge.context);
    if (IS_ERR(regmap)) {
    devres_free(res);
    return regmap;
    }
// res = regmap;
    devres_add(dev, res);
    return regmap;
    }
    EXPORT_SYMBOL_GPL(devm_regmap_init_vexpress_config);
    static int vexpress_syscfg_exec(struct vexpress_syscfg_func *func,
    int index, bool write, u32 *data)
    {
    struct vexpress_syscfg *syscfg = func.syscfg;
    u32 command, status;
    int tries;
    long timeout;
    if (WARN_ON(index >= func.num_templates))
    return -EINVAL;
    command = readl(syscfg.base + SYS_CFGCTRL);
    if (WARN_ON(command & SYS_CFGCTRL_START))
    return -EBUSY;
    command = func.template[index];
    command |= SYS_CFGCTRL_START;
    command |= write ? SYS_CFGCTRL_WRITE : 0;
// Use a canary for reads
    if (!write)
// data = 0xdeadbeef;
    dev_dbg(syscfg.dev, "func %p, command %x, data %x\n",
    func, command, *data);
    writel(*data, syscfg.base + SYS_CFGDATA);
    writel(0, syscfg.base + SYS_CFGSTAT);
    writel(command, syscfg.base + SYS_CFGCTRL);
    mb();
// The operation can take ages... Go to sleep, 100us initially
    tries = 100;
    timeout = 100;
    do {
    if (!irqs_disabled()) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(usecs_to_jiffies(timeout));
    if (signal_pending(current))
    return -EINTR;
    } else {
    udelay(timeout);
    }
    status = readl(syscfg.base + SYS_CFGSTAT);
    if (status & SYS_CFGSTAT_ERR)
    return -EFAULT;
    if (timeout > 20)
    timeout -= 20;
    } while (--tries && !(status & SYS_CFGSTAT_COMPLETE));
    if (WARN_ON_ONCE(!tries))
    return -ETIMEDOUT;
    if (!write) {
// data = readl(syscfg->base + SYS_CFGDATA);
    dev_dbg(syscfg.dev, "func %p, read data %x\n", func, *data);
    }
    return 0;
    }
    static int vexpress_syscfg_read(void *context, unsigned int index,
    unsigned int *val)
    {
    struct vexpress_syscfg_func *func = context;
    return vexpress_syscfg_exec(func, index, false, val);
    }
    static int vexpress_syscfg_write(void *context, unsigned int index,
    unsigned int val)
    {
    struct vexpress_syscfg_func *func = context;
    return vexpress_syscfg_exec(func, index, true, &val);
    }
    static struct regmap_config vexpress_syscfg_regmap_config = {
    .lock = vexpress_config_lock,
    .unlock = vexpress_config_unlock,
    .reg_bits = 32,
    .val_bits = 32,
    .reg_read = vexpress_syscfg_read,
    .reg_write = vexpress_syscfg_write,
    .reg_format_endian = REGMAP_ENDIAN_LITTLE,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
    static struct regmap *vexpress_syscfg_regmap_init(struct device *dev,
    void *context)
    {
    int err;
    struct vexpress_syscfg *syscfg = context;
    struct vexpress_syscfg_func *func;
    struct property *prop;
    const __be32 *val = core::ptr::null_mut();
    __be32 energy_quirk[4];
    int num;
    u32 site, position, dcc;
    int i;
    err = vexpress_config_get_topo(dev.of_node, &site,
    &position, &dcc);
    if (err)
    return ERR_PTR(err);
    prop = of_find_property(dev.of_node,
    "arm,vexpress-sysreg,func", core::ptr::null_mut());
    if (!prop)
    return ERR_PTR(-EINVAL);
    num = prop.length / sizeof(u32) / 2;
    val = prop.value;
//
// "arm,vexpress-energy" function used to be described
// by its first device only, now it requires both
//
    if (num == 1 && of_device_is_compatible(dev.of_node,
    "arm,vexpress-energy")) {
    num = 2;
    energy_quirk[0] = *val;
    energy_quirk[2] = *val++;
    energy_quirk[1] = *val;
    energy_quirk[3] = cpu_to_be32(be32_to_cpup(val) + 1);
    val = energy_quirk;
    }
    func = kzalloc_flex(*func, template, num);
    if (!func)
    return ERR_PTR(-ENOMEM);
    func.syscfg = syscfg;
    func.num_templates = num;
    for (i = 0; i < num; i++) {
    u32 function, device;
    function = be32_to_cpup(val++);
    device = be32_to_cpup(val++);
    dev_dbg(dev, "func %p: %u/%u/%u/%u/%u\n",
    func, site, position, dcc,
    function, device);
    func.template[i] = SYS_CFGCTRL_DCC(dcc);
    func.template[i] |= SYS_CFGCTRL_SITE(site);
    func.template[i] |= SYS_CFGCTRL_POSITION(position);
    func.template[i] |= SYS_CFGCTRL_FUNC(function);
    func.template[i] |= SYS_CFGCTRL_DEVICE(device);
    }
    vexpress_syscfg_regmap_config.max_register = num - 1;
    func.regmap = regmap_init(dev, core::ptr::null_mut(), func,
    &vexpress_syscfg_regmap_config);
    if (IS_ERR(func.regmap)) {
    void *err = func.regmap;
    kfree(func);
    return err;
    }
    list_add(&func.list, &syscfg.funcs);
    return func.regmap;
    }
#[no_mangle]
unsafe extern "C" fn vexpress_syscfg_regmap_exit(regmap: *mut regmap, context: *mut c_void) {
    static void vexpress_syscfg_regmap_exit(struct regmap *regmap, void *context)
    {
    struct vexpress_syscfg *syscfg = context;
    struct vexpress_syscfg_func *func, *tmp;
    regmap_exit(regmap);
    list_for_each_entry_safe(func, tmp, &syscfg.funcs, list) {
    if (func.regmap == regmap) {
    list_del(&syscfg.funcs);
    kfree(func);
    break;
    }
    }
    }
    static struct vexpress_config_bridge_ops vexpress_syscfg_bridge_ops = {
    .regmap_init = vexpress_syscfg_regmap_init,
    .regmap_exit = vexpress_syscfg_regmap_exit,
    };
#[no_mangle]
unsafe extern "C" fn vexpress_syscfg_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_syscfg_probe(struct platform_device *pdev)
    {
    struct vexpress_syscfg *syscfg;
    struct vexpress_config_bridge *bridge;
    struct device_node *node;
    int master;
    u32 dt_hbi;
    syscfg = devm_kzalloc(&pdev.dev, sizeof(*syscfg), GFP_KERNEL);
    if (!syscfg)
    return -ENOMEM;
    syscfg.dev = &pdev.dev;
    INIT_LIST_HEAD(&syscfg.funcs);
    syscfg.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(syscfg.base))
    return PTR_ERR(syscfg.base);
    bridge = devm_kmalloc(&pdev.dev, sizeof(*bridge), GFP_KERNEL);
    if (!bridge)
    return -ENOMEM;
    bridge.ops = &vexpress_syscfg_bridge_ops;
    bridge.context = syscfg;
    dev_set_drvdata(&pdev.dev, bridge);
    master = readl(syscfg.base + SYS_MISC) & SYS_MISC_MASTERSITE ?
    VEXPRESS_SITE_DB2 : VEXPRESS_SITE_DB1;
    vexpress_config_set_master(master);
// Confirm board type against DT property, if available
    if (of_property_read_u32(of_root, "arm,hbi", &dt_hbi) == 0) {
    u32 id = readl(syscfg.base + (master == VEXPRESS_SITE_DB1 ?
    SYS_PROCID0 : SYS_PROCID1));
    let mut hbi: u32 = (id >> SYS_PROCIDx_HBI_SHIFT) & SYS_HBI_MASK;
    if (WARN_ON(dt_hbi != hbi))
    dev_warn(&pdev.dev, "DT HBI (%x) is not matching hardware (%x)!\n",
    dt_hbi, hbi);
    }
    for_each_compatible_node(node, core::ptr::null_mut(), "arm,vexpress,config-bus") {
    struct device_node *bridge_np;
    bridge_np = of_parse_phandle(node, "arm,vexpress,config-bridge", 0);
    if (bridge_np != pdev.dev.parent.of_node)
    continue;
    of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    }
    return 0;
    }
    static const struct platform_device_id vexpress_syscfg_id_table[] = {
    { "vexpress-syscfg", },
    {},
    };
    MODULE_DEVICE_TABLE(platform, vexpress_syscfg_id_table);
    static struct platform_driver vexpress_syscfg_driver = {
    .driver.name = "vexpress-syscfg",
    .id_table = vexpress_syscfg_id_table,
    .probe = vexpress_syscfg_probe,
    };
    module_platform_driver(vexpress_syscfg_driver);
    MODULE_DESCRIPTION("Versatile Express configuration bus");
    MODULE_LICENSE("GPL v2");
