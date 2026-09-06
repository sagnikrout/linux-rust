//! Automatically rewritten from C to Rust
//! Source: drivers/platform/mellanox/mlxreg-dpu.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Nvidia Data Processor Unit platform driver
//
// Copyright (C) 2025 Nvidia Technologies Ltd.
//

// I2C bus IO offsets
pub const MLXREG_DPU_REG_FPGA1_VER_OFFSET: c_uint = 0x2400;
pub const MLXREG_DPU_REG_FPGA1_PN_OFFSET: c_uint = 0x2404;
pub const MLXREG_DPU_REG_FPGA1_PN1_OFFSET: c_uint = 0x2405;
pub const MLXREG_DPU_REG_PG_OFFSET: c_uint = 0x2414;
pub const MLXREG_DPU_REG_PG_EVENT_OFFSET: c_uint = 0x2415;
pub const MLXREG_DPU_REG_PG_MASK_OFFSET: c_uint = 0x2416;
pub const MLXREG_DPU_REG_RESET_GP1_OFFSET: c_uint = 0x2417;
pub const MLXREG_DPU_REG_RST_CAUSE1_OFFSET: c_uint = 0x241e;
pub const MLXREG_DPU_REG_GP0_RO_OFFSET: c_uint = 0x242b;
pub const MLXREG_DPU_REG_GP0_OFFSET: c_uint = 0x242e;
pub const MLXREG_DPU_REG_GP1_OFFSET: c_uint = 0x242c;
pub const MLXREG_DPU_REG_GP4_OFFSET: c_uint = 0x2438;
pub const MLXREG_DPU_REG_AGGRCO_OFFSET: c_uint = 0x2442;
pub const MLXREG_DPU_REG_AGGRCO_MASK_OFFSET: c_uint = 0x2443;
pub const MLXREG_DPU_REG_HEALTH_OFFSET: c_uint = 0x244d;
pub const MLXREG_DPU_REG_HEALTH_EVENT_OFFSET: c_uint = 0x244e;
pub const MLXREG_DPU_REG_HEALTH_MASK_OFFSET: c_uint = 0x244f;
pub const MLXREG_DPU_REG_FPGA1_MVER_OFFSET: c_uint = 0x24de;
pub const MLXREG_DPU_REG_CONFIG3_OFFSET: c_uint = 0x24fd;
pub const MLXREG_DPU_REG_MAX: c_uint = 0x3fff;
// Power Good event masks.

    MLXREG_DPU_PG_HVDD_MASK | \
    MLXREG_DPU_PG_VDDQ_MASK | \
    MLXREG_DPU_PG_COMPARATOR_MASK | \
    MLXREG_DPU_PG_1V8_MASK | \
    MLXREG_DPU_PG_VDD_CPU_MASK | \
    MLXREG_DPU_PG_VDD_MASK | \
    MLXREG_DPU_PG_VDDIO_MASK)
// Health event masks.

    MLXREG_DPU_HLTH_VDDQ_HOT_ALERT_MASK | \
    MLXREG_DPU_HLTH_VDD_CPU_HOT_ALERT_MASK | \
    MLXREG_DPU_HLTH_VDDQ_ALERT_MASK | \
    MLXREG_DPU_HLTH_VDD_CPU_ALERT_MASK | \
    MLXREG_DPU_HLTH_THERMAL_TRIP_MASK)
// Hotplug aggregation masks.

    MLXREG_DPU_PG_AGGR_MASK)
// Voltage regulator firmware update status mask.

//
// enum mlxreg_dpu_type - Data Processor Unit types
//
// @MLXREG_DPU_BF3: DPU equipped with BF3 SoC;
//
    enum mlxreg_dpu_type {
    MLXREG_DPU_BF3 = 0x0050,
    };
// Default register access data.
    static struct mlxreg_core_data mlxreg_dpu_io_data[] = {
    {
    .label = "fpga1_version",
    .reg = MLXREG_DPU_REG_FPGA1_VER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "fpga1_pn",
    .reg = MLXREG_DPU_REG_FPGA1_PN_OFFSET,
    .bit = GENMASK(15, 0),
    .mode = 0444,
    .regnum = 2,
    },
    {
    .label = "fpga1_version_min",
    .reg = MLXREG_DPU_REG_FPGA1_MVER_OFFSET,
    .bit = GENMASK(7, 0),
    .mode = 0444,
    },
    {
    .label = "perst_rst",
    .reg = MLXREG_DPU_REG_RESET_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(0),
    .mode = 0644,
    },
    {
    .label = "usbphy_rst",
    .reg = MLXREG_DPU_REG_RESET_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    {
    .label = "phy_rst",
    .reg = MLXREG_DPU_REG_RESET_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0644,
    },
    {
    .label = "tpm_rst",
    .reg = MLXREG_DPU_REG_RESET_GP1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0644,
    },
    {
    .label = "reset_from_main_board",
    .reg = MLXREG_DPU_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0444,
    },
    {
    .label = "reset_aux_pwr_or_reload",
    .reg = MLXREG_DPU_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(2),
    .mode = 0444,
    },
    {
    .label = "reset_comex_pwr_fail",
    .reg = MLXREG_DPU_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(3),
    .mode = 0444,
    },
    {
    .label = "reset_dpu_thermal",
    .reg = MLXREG_DPU_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(6),
    .mode = 0444,
    },
    {
    .label = "reset_pwr_off",
    .reg = MLXREG_DPU_REG_RST_CAUSE1_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(7),
    .mode = 0444,
    },
    {
    .label = "dpu_id",
    .reg = MLXREG_DPU_REG_GP0_RO_OFFSET,
    .bit = GENMASK(3, 0),
    .mode = 0444,
    },
    {
    .label = "voltreg_update_status",
    .reg = MLXREG_DPU_REG_GP0_RO_OFFSET,
    .mask = MLXREG_DPU_VOLTREG_UPD_MASK,
    .bit = 5,
    .mode = 0444,
    },
    {
    .label = "boot_progress",
    .reg = MLXREG_DPU_REG_GP1_OFFSET,
    .mask = GENMASK(3, 0),
    .mode = 0444,
    },
    {
    .label = "ufm_upgrade",
    .reg = MLXREG_DPU_REG_GP4_OFFSET,
    .mask = GENMASK(7, 0) & ~BIT(1),
    .mode = 0644,
    },
    };
    static struct mlxreg_core_platform_data mlxreg_dpu_default_regs_io_data = {
    .data = mlxreg_dpu_io_data,
    .counter = ARRAY_SIZE(mlxreg_dpu_io_data),
    };
// Default hotplug data.
    static struct mlxreg_core_data mlxreg_dpu_power_events_items_data[] = {
    {
    .label = "pg_vddio",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_VDDIO_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_vdd_cpu",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_VDD_CPU_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_vdd",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_VDD_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_1v8",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_1V8_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_comparator",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_COMPARATOR_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_vddq",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_VDDQ_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_hvdd",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_HVDD_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "pg_dvdd",
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_DVDD_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    };
    static struct mlxreg_core_data mlxreg_dpu_health_events_items_data[] = {
    {
    .label = "thermal_trip",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_THERMAL_TRIP_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "ufm_upgrade_done",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_UFM_UPGRADE_DONE_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "vddq_hot_alert",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_VDDQ_HOT_ALERT_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "vdd_cpu_hot_alert",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_VDD_CPU_HOT_ALERT_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "vddq_alert",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_VDDQ_ALERT_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    {
    .label = "vdd_cpu_alert",
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HLTH_VDD_CPU_ALERT_MASK,
    .hpdev.nr = MLXREG_DPU_NR_NONE,
    },
    };
    static struct mlxreg_core_item mlxreg_dpu_hotplug_items[] = {
    {
    .data = mlxreg_dpu_power_events_items_data,
    .aggr_mask = MLXREG_DPU_PG_AGGR_MASK,
    .reg = MLXREG_DPU_REG_PG_OFFSET,
    .mask = MLXREG_DPU_PG_MASK,
    .count = ARRAY_SIZE(mlxreg_dpu_power_events_items_data),
    .health = false,
    .inversed = 0,
    },
    {
    .data = mlxreg_dpu_health_events_items_data,
    .aggr_mask = MLXREG_DPU_HEALTH_AGGR_MASK,
    .reg = MLXREG_DPU_REG_HEALTH_OFFSET,
    .mask = MLXREG_DPU_HEALTH_MASK,
    .count = ARRAY_SIZE(mlxreg_dpu_health_events_items_data),
    .health = false,
    .inversed = 0,
    },
    };
    static
    struct mlxreg_core_hotplug_platform_data mlxreg_dpu_default_hotplug_data = {
    .items = mlxreg_dpu_hotplug_items,
    .count = ARRAY_SIZE(mlxreg_dpu_hotplug_items),
    .cell = MLXREG_DPU_REG_AGGRCO_OFFSET,
    .mask = MLXREG_DPU_AGGR_MASK,
    };
//
// struct mlxreg_dpu - device private data
// @dev: platform device
// @data: platform core data
// @io_data: register access platform data
// @io_regs: register access device
// @hotplug_data: hotplug platform data
// @hotplug: hotplug device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_dpu {
    pub dev: *mut device,
    pub data: *mut mlxreg_core_data,
    pub io_data: *mut mlxreg_core_platform_data,
    pub io_regs: *mut platform_device,
    pub hotplug_data: *mut mlxreg_core_hotplug_platform_data,
    pub hotplug: *mut platform_device,
}

#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxreg_dpu_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXREG_DPU_REG_PG_EVENT_OFFSET:
    case MLXREG_DPU_REG_PG_MASK_OFFSET:
    case MLXREG_DPU_REG_RESET_GP1_OFFSET:
    case MLXREG_DPU_REG_GP0_OFFSET:
    case MLXREG_DPU_REG_GP1_OFFSET:
    case MLXREG_DPU_REG_GP4_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_MASK_OFFSET:
    case MLXREG_DPU_REG_HEALTH_EVENT_OFFSET:
    case MLXREG_DPU_REG_HEALTH_MASK_OFFSET:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxreg_dpu_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXREG_DPU_REG_FPGA1_VER_OFFSET:
    case MLXREG_DPU_REG_FPGA1_PN_OFFSET:
    case MLXREG_DPU_REG_FPGA1_PN1_OFFSET:
    case MLXREG_DPU_REG_PG_OFFSET:
    case MLXREG_DPU_REG_PG_EVENT_OFFSET:
    case MLXREG_DPU_REG_PG_MASK_OFFSET:
    case MLXREG_DPU_REG_RESET_GP1_OFFSET:
    case MLXREG_DPU_REG_RST_CAUSE1_OFFSET:
    case MLXREG_DPU_REG_GP0_RO_OFFSET:
    case MLXREG_DPU_REG_GP0_OFFSET:
    case MLXREG_DPU_REG_GP1_OFFSET:
    case MLXREG_DPU_REG_GP4_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_MASK_OFFSET:
    case MLXREG_DPU_REG_HEALTH_OFFSET:
    case MLXREG_DPU_REG_HEALTH_EVENT_OFFSET:
    case MLXREG_DPU_REG_HEALTH_MASK_OFFSET:
    case MLXREG_DPU_REG_FPGA1_MVER_OFFSET:
    case MLXREG_DPU_REG_CONFIG3_OFFSET:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mlxreg_dpu_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MLXREG_DPU_REG_FPGA1_VER_OFFSET:
    case MLXREG_DPU_REG_FPGA1_PN_OFFSET:
    case MLXREG_DPU_REG_FPGA1_PN1_OFFSET:
    case MLXREG_DPU_REG_PG_OFFSET:
    case MLXREG_DPU_REG_PG_EVENT_OFFSET:
    case MLXREG_DPU_REG_PG_MASK_OFFSET:
    case MLXREG_DPU_REG_RESET_GP1_OFFSET:
    case MLXREG_DPU_REG_RST_CAUSE1_OFFSET:
    case MLXREG_DPU_REG_GP0_RO_OFFSET:
    case MLXREG_DPU_REG_GP0_OFFSET:
    case MLXREG_DPU_REG_GP1_OFFSET:
    case MLXREG_DPU_REG_GP4_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_OFFSET:
    case MLXREG_DPU_REG_AGGRCO_MASK_OFFSET:
    case MLXREG_DPU_REG_HEALTH_OFFSET:
    case MLXREG_DPU_REG_HEALTH_EVENT_OFFSET:
    case MLXREG_DPU_REG_HEALTH_MASK_OFFSET:
    case MLXREG_DPU_REG_FPGA1_MVER_OFFSET:
    case MLXREG_DPU_REG_CONFIG3_OFFSET:
    return true;
    }
    return false;
    }
// Configuration for the register map of a device with 2 bytes address space.
    static const struct regmap_config mlxreg_dpu_regmap_conf = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = MLXREG_DPU_REG_MAX,
    .cache_type = REGCACHE_FLAT,
    .writeable_reg = mlxreg_dpu_writeable_reg,
    .readable_reg = mlxreg_dpu_readable_reg,
    .volatile_reg = mlxreg_dpu_volatile_reg,
    };
    static int
    mlxreg_dpu_copy_hotplug_data(struct device *dev, struct mlxreg_dpu *mlxreg_dpu,
    const struct mlxreg_core_hotplug_platform_data *hotplug_data)
    {
    struct mlxreg_core_item *item;
    int i;
    mlxreg_dpu.hotplug_data = devm_kmemdup(dev, hotplug_data,
    sizeof(*mlxreg_dpu.hotplug_data), GFP_KERNEL);
    if (!mlxreg_dpu.hotplug_data)
    return -ENOMEM;
    mlxreg_dpu.hotplug_data.items = devm_kmemdup(dev, hotplug_data.items,
    mlxreg_dpu.hotplug_data.count *
    sizeof(*mlxreg_dpu.hotplug_data.items),
    GFP_KERNEL);
    if (!mlxreg_dpu.hotplug_data.items)
    return -ENOMEM;
    item = mlxreg_dpu.hotplug_data.items;
    for (i = 0; i < hotplug_data.count; i++, item++) {
    item.data = devm_kmemdup(dev, hotplug_data.items[i].data,
    hotplug_data.items[i].count * sizeof(*item.data),
    GFP_KERNEL);
    if (!item.data)
    return -ENOMEM;
    }
    return 0;
    }
    static int mlxreg_dpu_config_init(struct mlxreg_dpu *mlxreg_dpu, void *regmap,
    struct mlxreg_core_data *data, int irq)
    {
    struct device *dev = &data.hpdev.client.dev;
    u32 regval;
    int err;
// Validate DPU type.
    err = regmap_read(regmap, MLXREG_DPU_REG_CONFIG3_OFFSET, &regval);
    if (err)
    return err;
    switch (regval) {
    case MLXREG_DPU_BF3:
// Copy platform specific hotplug data.
    err = mlxreg_dpu_copy_hotplug_data(dev, mlxreg_dpu,
    &mlxreg_dpu_default_hotplug_data);
    if (err)
    return err;
    mlxreg_dpu.io_data = &mlxreg_dpu_default_regs_io_data;
    break;
    default:
    return -ENODEV;
    }
// Register IO access driver.
    if (mlxreg_dpu.io_data) {
    mlxreg_dpu.io_data.regmap = regmap;
    mlxreg_dpu.io_regs =
    platform_device_register_resndata(dev, "mlxreg-io",
    data.slot, core::ptr::null_mut(), 0,
    mlxreg_dpu.io_data,
    sizeof(*mlxreg_dpu.io_data));
    if (IS_ERR(mlxreg_dpu.io_regs)) {
    dev_err(dev, "Failed to create region for client %s at bus %d at addr 0x%02x\n",
    data.hpdev.brdinfo.type, data.hpdev.nr,
    data.hpdev.brdinfo.addr);
    return PTR_ERR(mlxreg_dpu.io_regs);
    }
    }
// Register hotplug driver.
    if (mlxreg_dpu.hotplug_data && irq) {
    mlxreg_dpu.hotplug_data.regmap = regmap;
    mlxreg_dpu.hotplug_data.irq = irq;
    mlxreg_dpu.hotplug =
    platform_device_register_resndata(dev, "mlxreg-hotplug",
    data.slot, core::ptr::null_mut(), 0,
    mlxreg_dpu.hotplug_data,
    sizeof(*mlxreg_dpu.hotplug_data));
    if (IS_ERR(mlxreg_dpu.hotplug)) {
    err = PTR_ERR(mlxreg_dpu.hotplug);
    goto fail_register_hotplug;
    }
    }
    return 0;
    fail_register_hotplug:
    platform_device_unregister(mlxreg_dpu.io_regs);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_config_exit(mlxreg_dpu: *mut mlxreg_dpu) {
    static void mlxreg_dpu_config_exit(struct mlxreg_dpu *mlxreg_dpu)
    {
    platform_device_unregister(mlxreg_dpu.hotplug);
    platform_device_unregister(mlxreg_dpu.io_regs);
    }
#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_probe(pdev: *mut platform_device) -> c_int {
    static int mlxreg_dpu_probe(struct platform_device *pdev)
    {
    struct mlxreg_core_data *data;
    struct mlxreg_dpu *mlxreg_dpu;
    void *regmap;
    int err;
    data = dev_get_platdata(&pdev.dev);
    if (!data || !data.hpdev.brdinfo)
    return -EINVAL;
    data.hpdev.adapter = i2c_get_adapter(data.hpdev.nr);
    if (!data.hpdev.adapter)
    return -EPROBE_DEFER;
    mlxreg_dpu = devm_kzalloc(&pdev.dev, sizeof(*mlxreg_dpu), GFP_KERNEL);
    if (!mlxreg_dpu) {
    err = -ENOMEM;
    goto alloc_fail;
    }
// Create device at the top of DPU I2C tree.
    data.hpdev.client = i2c_new_client_device(data.hpdev.adapter,
    data.hpdev.brdinfo);
    if (IS_ERR(data.hpdev.client)) {
    dev_err(&pdev.dev, "Failed to create client %s at bus %d at addr 0x%02x\n",
    data.hpdev.brdinfo.type, data.hpdev.nr, data.hpdev.brdinfo.addr);
    err = PTR_ERR(data.hpdev.client);
    goto i2c_new_device_fail;
    }
    regmap = devm_regmap_init_i2c(data.hpdev.client, &mlxreg_dpu_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&pdev.dev, "Failed to create regmap for client %s at bus %d at addr 0x%02x\n",
    data.hpdev.brdinfo.type, data.hpdev.nr, data.hpdev.brdinfo.addr);
    err = PTR_ERR(regmap);
    goto devm_regmap_init_i2c_fail;
    }
// Sync registers with hardware.
    regcache_mark_dirty(regmap);
    err = regcache_sync(regmap);
    if (err) {
    dev_err(&pdev.dev, "Failed to sync regmap for client %s at bus %d at addr 0x%02x\n",
    data.hpdev.brdinfo.type, data.hpdev.nr, data.hpdev.brdinfo.addr);
    goto regcache_sync_fail;
    }
    mlxreg_dpu.data = data;
    mlxreg_dpu.dev = &pdev.dev;
    platform_set_drvdata(pdev, mlxreg_dpu);
    err = mlxreg_dpu_config_init(mlxreg_dpu, regmap, data, data.hpdev.brdinfo.irq);
    if (err)
    goto mlxreg_dpu_config_init_fail;
    return err;
    mlxreg_dpu_config_init_fail:
    regcache_sync_fail:
    devm_regmap_init_i2c_fail:
    i2c_unregister_device(data.hpdev.client);
    i2c_new_device_fail:
    alloc_fail:
    i2c_put_adapter(data.hpdev.adapter);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxreg_dpu_remove(pdev: *mut platform_device) {
    static void mlxreg_dpu_remove(struct platform_device *pdev)
    {
    struct mlxreg_core_data *data = dev_get_platdata(&pdev.dev);
    struct mlxreg_dpu *mlxreg_dpu = platform_get_drvdata(pdev);
    mlxreg_dpu_config_exit(mlxreg_dpu);
    i2c_unregister_device(data.hpdev.client);
    i2c_put_adapter(data.hpdev.adapter);
    }
    static struct platform_driver mlxreg_dpu_driver = {
    .probe = mlxreg_dpu_probe,
    .remove = mlxreg_dpu_remove,
    .driver = {
    .name = "mlxreg-dpu",
    },
    };
    module_platform_driver(mlxreg_dpu_driver);
    MODULE_AUTHOR("Vadim Pasternak <vadimp@nvidia.com>");
    MODULE_DESCRIPTION("Nvidia Data Processor Unit platform driver");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_ALIAS("platform:mlxreg-dpu");
