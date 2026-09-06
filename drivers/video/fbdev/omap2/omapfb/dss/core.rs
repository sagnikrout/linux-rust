//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/core.c
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
// linux/drivers/video/omap2/dss/core.c
//
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
// Some code and ideas taken from drivers/video/omap/ driver
// by Imre Deak.
//

    static struct {
    struct platform_device *pdev;
    const char *default_display_name;
    } core;
    static char *def_disp_name;
    module_param_named(def_disp, def_disp_name, charp, 0);
    MODULE_PARM_DESC(def_disp, "default display name");
    const char *omapdss_get_default_display_name(void)
    {
    return core.default_display_name;
    }
    EXPORT_SYMBOL(omapdss_get_default_display_name);
#[no_mangle]
pub unsafe extern "C" fn omapdss_get_version() -> enum omapdss_version {
    enum omapdss_version omapdss_get_version(void)
    {
    struct omap_dss_board_info *pdata = core.pdev.dev.platform_data;
    return pdata.version;
    }
    EXPORT_SYMBOL(omapdss_get_version);
    struct platform_device *dss_get_core_pdev(void)
    {
    return core.pdev;
    }
#[no_mangle]
pub unsafe extern "C" fn dss_dsi_enable_pads(dsi_id: c_int, lane_mask: unsigned) -> c_int {
    int dss_dsi_enable_pads(int dsi_id, unsigned lane_mask)
    {
    struct omap_dss_board_info *board_data = core.pdev.dev.platform_data;
    if (!board_data.dsi_enable_pads)
    return -ENOENT;
    return board_data.dsi_enable_pads(dsi_id, lane_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_dsi_disable_pads(dsi_id: c_int, lane_mask: unsigned) {
    void dss_dsi_disable_pads(int dsi_id, unsigned lane_mask)
    {
    struct omap_dss_board_info *board_data = core.pdev.dev.platform_data;
    if (!board_data.dsi_disable_pads)
    return;
    return board_data.dsi_disable_pads(dsi_id, lane_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_set_min_bus_tput(dev: *mut device, tput: c_ulong) -> c_int {
    int dss_set_min_bus_tput(struct device *dev, unsigned long tput)
    {
    struct omap_dss_board_info *pdata = core.pdev.dev.platform_data;
    if (pdata.set_min_bus_tput)
    return pdata.set_min_bus_tput(dev, tput);
    else
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dss_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int dss_show(struct seq_file *s, void *unused)
    {
    void (*func)(struct seq_file *) = s.private;
    func(s);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(dss);
    static struct dentry *dss_debugfs_dir;
#[no_mangle]
unsafe extern "C" fn dss_initialize_debugfs() {
    static void dss_initialize_debugfs(void)
    {
    dss_debugfs_dir = debugfs_create_dir("omapdss", core::ptr::null_mut());
    debugfs_create_file("clk", S_IRUGO, dss_debugfs_dir,
    &dss_debug_dump_clocks, &dss_fops);
    }
#[no_mangle]
unsafe extern "C" fn dss_uninitialize_debugfs() {
    static void dss_uninitialize_debugfs(void)
    {
    debugfs_remove_recursive(dss_debugfs_dir);
    }
#[no_mangle]
pub unsafe extern "C" fn dss_debugfs_create_file(name: *const c_char, ): *mut *mut void (write)(struct seq_file) {
    void dss_debugfs_create_file(const char *name, void (*write)(struct seq_file *))
    {
    debugfs_create_file(name, S_IRUGO, dss_debugfs_dir, write, &dss_fops);
    }

#[no_mangle]
pub unsafe extern "C" fn dss_initialize_debugfs() {
    static inline void dss_initialize_debugfs(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn dss_uninitialize_debugfs() {
    static inline void dss_uninitialize_debugfs(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn dss_debugfs_create_file(name: *const c_char, ): *mut *mut void (write)(struct seq_file) {
    void dss_debugfs_create_file(const char *name, void (*write)(struct seq_file *))
    {
    }

// PLATFORM DEVICE
#[no_mangle]
unsafe extern "C" fn omap_dss_pm_notif(b: *mut notifier_block, v: c_ulong, d: *mut c_void) -> c_int {
    static int omap_dss_pm_notif(struct notifier_block *b, unsigned long v, void *d)
    {
    DSSDBG("pm notif %lu\n", v);
    switch (v) {
    case PM_SUSPEND_PREPARE:
    case PM_HIBERNATION_PREPARE:
    case PM_RESTORE_PREPARE:
    DSSDBG("suspending displays\n");
    return dss_suspend_all_devices();
    case PM_POST_SUSPEND:
    case PM_POST_HIBERNATION:
    case PM_POST_RESTORE:
    DSSDBG("resuming displays\n");
    return dss_resume_all_devices();
    default:
    return 0;
    }
    }
    static struct notifier_block omap_dss_pm_notif_block = {
    .notifier_call = omap_dss_pm_notif,
    };
#[no_mangle]
unsafe extern "C" fn omap_dss_probe(pdev: *mut platform_device) -> int __init {
    static int __init omap_dss_probe(struct platform_device *pdev)
    {
    core.pdev = pdev;
    dss_features_init(omapdss_get_version());
    dss_initialize_debugfs();
    if (def_disp_name)
    core.default_display_name = def_disp_name;
    register_pm_notifier(&omap_dss_pm_notif_block);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_dss_remove(pdev: *mut platform_device) {
    static void omap_dss_remove(struct platform_device *pdev)
    {
    unregister_pm_notifier(&omap_dss_pm_notif_block);
    dss_uninitialize_debugfs();
    }
#[no_mangle]
unsafe extern "C" fn omap_dss_shutdown(pdev: *mut platform_device) {
    static void omap_dss_shutdown(struct platform_device *pdev)
    {
    DSSDBG("shutdown\n");
    dss_disable_all_devices();
    }
    static struct platform_driver omap_dss_driver = {
    .remove		= omap_dss_remove,
    .shutdown	= omap_dss_shutdown,
    .driver		= {
    .name	= "omapdss",
    },
    };
// INIT
    static int (*dss_output_drv_reg_funcs[])(void) __initdata = {
    dss_init_platform_driver,
    dispc_init_platform_driver,

    dsi_init_platform_driver,

    dpi_init_platform_driver,

    sdi_init_platform_driver,

    venc_init_platform_driver,

    hdmi4_init_platform_driver,

    hdmi5_init_platform_driver,

    };
    static void (*dss_output_drv_unreg_funcs[])(void) = {

    hdmi5_uninit_platform_driver,

    hdmi4_uninit_platform_driver,

    venc_uninit_platform_driver,

    sdi_uninit_platform_driver,

    dpi_uninit_platform_driver,

    dsi_uninit_platform_driver,

    dispc_uninit_platform_driver,
    dss_uninit_platform_driver,
    };
#[no_mangle]
unsafe extern "C" fn omap_dss_init() -> int __init {
    static int __init omap_dss_init(void)
    {
    int r;
    int i;
    r = platform_driver_probe(&omap_dss_driver, omap_dss_probe);
    if (r)
    return r;
    for (i = 0; i < ARRAY_SIZE(dss_output_drv_reg_funcs); ++i) {
    r = dss_output_drv_reg_funcs[i]();
    if (r)
    goto err_reg;
    }
    return 0;
    err_reg:
    for (i = ARRAY_SIZE(dss_output_drv_reg_funcs) - i;
    i < ARRAY_SIZE(dss_output_drv_reg_funcs);
    ++i)
    dss_output_drv_unreg_funcs[i]();
    platform_driver_unregister(&omap_dss_driver);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn omap_dss_exit() -> void __exit {
    static void __exit omap_dss_exit(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(dss_output_drv_unreg_funcs); ++i)
    dss_output_drv_unreg_funcs[i]();
    platform_driver_unregister(&omap_dss_driver);
    }
    module_init(omap_dss_init);
    module_exit(omap_dss_exit);
    MODULE_AUTHOR("Tomi Valkeinen <tomi.valkeinen@nokia.com>");
    MODULE_DESCRIPTION("OMAP2/3 Display Subsystem");
    MODULE_LICENSE("GPL v2");
