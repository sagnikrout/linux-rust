//! Automatically rewritten from C to Rust
//! Source: drivers/cxl/core/edac.c
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
// CXL EDAC memory feature driver.
//
// Copyright (c) 2024-2025 HiSilicon Limited.
//
// - Supports functions to configure EDAC features of the
// CXL memory devices.
// - Registers with the EDAC device subsystem driver to expose
// the features sysfs attributes to the user for configuring
// CXL memory RAS feature.
//

pub const CXL_NR_EDAC_DEV_FEATURES: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_patrol_scrub_context {
    pub instance: u8,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub get_version: u8,
    pub set_version: u8,
    pub effects: u16,
    pub cxlmd: *mut cxl_memdev,
    pub cxlr: *mut cxl_region,
}

//
// See CXL spec rev 3.2 @8.2.10.9.11.1 Table 8-222 Device Patrol Scrub Control
// Feature Readable Attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_scrub_rd_attrbs {
    pub scrub_cycle_cap: u8,
    pub scrub_cycle_hours: __le16,
    pub scrub_flags: u8,
    pub __packed: },
//
// See CXL spec rev 3.2 @8.2.10.9.11.1 Table 8-223 Device Patrol Scrub Control
// Feature Writable Attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_scrub_wr_attrbs {
    pub scrub_cycle_hours: u8,
    pub scrub_flags: u8,
    pub __packed: },

    FIELD_GET(CXL_SCRUB_CONTROL_CHANGEABLE, cap)

    FIELD_GET(CXL_SCRUB_CONTROL_CYCLE_MASK, cycle)

    FIELD_GET(CXL_SCRUB_CONTROL_MIN_CYCLE_MASK, cycle)

    FIELD_PREP(CXL_SCRUB_CONTROL_CYCLE_MASK, cycle)

    static int cxl_mem_scrub_get_attrbs(struct cxl_mailbox *cxl_mbox, u8 *cap,
    u16 *cycle, u8 *flags, u8 *min_cycle)
    {
    pub cxl_scrub_rd_attrbs): size_t rd_data_size = sizeof(struct,
    pub data_size: usize,
    struct cxl_scrub_rd_attrbs *rd_attrbs __free(kfree) =
    pub GFP_KERNEL): kzalloc(rd_data_size,,
    if (!rd_attrbs)
    pub -ENOMEM: return,
    data_size = cxl_get_feature(cxl_mbox, &CXL_FEAT_PATROL_SCRUB_UUID,
    CXL_GET_FEAT_SEL_CURRENT_VALUE, rd_attrbs,
    pub NULL): rd_data_size, 0,,
    if (!data_size)
    pub -EIO: return,
// cap = rd_attrbs->scrub_cycle_cap;
// cycle = le16_to_cpu(rd_attrbs->scrub_cycle_hours);
// flags = rd_attrbs->scrub_flags;
    if (min_cycle)
// min_cycle = CXL_GET_SCRUB_MIN_CYCLE(*cycle);
    pub 0: return,
    }
    static int cxl_scrub_get_attrbs(struct cxl_patrol_scrub_context *cxl_ps_ctx,
    u8 *cap, u16 *cycle, u8 *flags, u8 *min_cycle)
    {
    pub cxl_mbox: *mut cxl_mailbox,
    pub p: *mut cxl_region_params,
    pub cxlmd: *mut cxl_memdev,
    pub cxlr: *mut cxl_region,
    pub 0: u8 min_scrub_cycle =,
    pub ret: int i,,
    if (!cxl_ps_ctx.cxlr) {
    pub &cxl_ps_ctx->cxlmd->cxlds->cxl_mbox: cxl_mbox =,
    return cxl_mem_scrub_get_attrbs(cxl_mbox, cap, cycle,
    pub min_cycle): flags,,
    }
    pub rwsem)(&cxl_rwsem.region): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &rwsem)))
    pub ret: return,
    pub cxl_ps_ctx->cxlr: cxlr =,
    pub &cxlr->params: p =,
    pub {: for (i = 0; i < p->nr_targets; i++),
    pub p->targets[i]: *mut *mut cxl_endpoint_decoder cxled =,
    pub cxled_to_memdev(cxled): cxlmd =,
    pub &cxlmd->cxlds->cxl_mbox: cxl_mbox =,
    ret = cxl_mem_scrub_get_attrbs(cxl_mbox, cap, cycle, flags,
    if (ret)
    pub ret: return,
//
// The min_scrub_cycle of a region is the max of minimum scrub
// cycles supported by memdevs that back the region.
//
    if (min_cycle)
    pub min_scrub_cycle): *mut *mut min_scrub_cycle = max(min_cycle,,
    }
    if (min_cycle)
// min_cycle = min_scrub_cycle;
    pub 0: return,
    }
    static int cxl_scrub_set_attrbs_region(struct device *dev,
    struct cxl_patrol_scrub_context *cxl_ps_ctx,
    u8 cycle, u8 flags)
    {
    pub wr_attrbs: cxl_scrub_wr_attrbs,
    pub cxl_mbox: *mut cxl_mailbox,
    pub p: *mut cxl_region_params,
    pub cxlmd: *mut cxl_memdev,
    pub cxlr: *mut cxl_region,
    pub i: int ret,,
    pub rwsem)(&cxl_rwsem.region): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &rwsem)))
    pub ret: return,
    pub cxl_ps_ctx->cxlr: cxlr =,
    pub &cxlr->params: p =,
    pub cycle: wr_attrbs.scrub_cycle_hours =,
    pub flags: wr_attrbs.scrub_flags =,
    pub {: for (i = 0; i < p->nr_targets; i++),
    pub p->targets[i]: *mut *mut cxl_endpoint_decoder cxled =,
    pub cxled_to_memdev(cxled): cxlmd =,
    pub &cxlmd->cxlds->cxl_mbox: cxl_mbox =,
    ret = cxl_set_feature(cxl_mbox, &CXL_FEAT_PATROL_SCRUB_UUID,
    cxl_ps_ctx.set_version, &wr_attrbs,
    sizeof(wr_attrbs),
    CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET,
    pub NULL): 0,,
    if (ret)
    pub ret: return,
    if (cycle != cxlmd.scrub_cycle) {
    if (cxlmd.scrub_region_id != CXL_SCRUB_NO_REGION)
    dev_info(dev,
    "Device scrub rate(%d hours) set by region%d rate overwritten by region%d scrub rate(%d hours)\n",
    cxlmd.scrub_cycle,
    cxlmd.scrub_region_id, cxlr.id,
    pub cycle: cxlmd->scrub_cycle =,
    pub cxlr->id: cxlmd->scrub_region_id =,
    }
    }
    pub 0: return,
    }
    static int cxl_scrub_set_attrbs_device(struct device *dev,
    struct cxl_patrol_scrub_context *cxl_ps_ctx,
    u8 cycle, u8 flags)
    {
    pub wr_attrbs: cxl_scrub_wr_attrbs,
    pub cxl_mbox: *mut cxl_mailbox,
    pub cxlmd: *mut cxl_memdev,
    pub ret: c_int,
    pub cycle: wr_attrbs.scrub_cycle_hours =,
    pub flags: wr_attrbs.scrub_flags =,
    pub cxl_ps_ctx->cxlmd: cxlmd =,
    pub &cxlmd->cxlds->cxl_mbox: cxl_mbox =,
    ret = cxl_set_feature(cxl_mbox, &CXL_FEAT_PATROL_SCRUB_UUID,
    cxl_ps_ctx.set_version, &wr_attrbs,
    sizeof(wr_attrbs),
    CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET, 0,
    if (ret)
    pub ret: return,
    if (cycle != cxlmd.scrub_cycle) {
    if (cxlmd.scrub_region_id != CXL_SCRUB_NO_REGION)
    dev_info(dev,
    "Device scrub rate(%d hours) set by region%d rate overwritten with device local scrub rate(%d hours)\n",
    cxlmd.scrub_cycle, cxlmd.scrub_region_id,
    pub cycle: cxlmd->scrub_cycle =,
    pub CXL_SCRUB_NO_REGION: cxlmd->scrub_region_id =,
    }
    pub 0: return,
    }
    static int cxl_scrub_set_attrbs(struct device *dev,
    struct cxl_patrol_scrub_context *cxl_ps_ctx,
    u8 cycle, u8 flags)
    {
    if (cxl_ps_ctx.cxlr)
    pub flags): return cxl_scrub_set_attrbs_region(dev, cxl_ps_ctx, cycle,,
    pub flags): return cxl_scrub_set_attrbs_device(dev, cxl_ps_ctx, cycle,,
    }
    static int cxl_patrol_scrub_get_enabled_bg(struct device *dev, void *drv_data,
    bool *enabled)
    {
    pub drv_data: *mut *mut cxl_patrol_scrub_context ctx =,
    pub flags: u8 cap,,
    pub cycle: u16,
    pub ret: c_int,
    pub NULL): ret = cxl_scrub_get_attrbs(ctx, &cap, &cycle, &flags,,
    if (ret)
    pub ret: return,
// enabled = CXL_GET_SCRUB_EN_STS(flags);
    pub 0: return,
    }
    static int cxl_patrol_scrub_set_enabled_bg(struct device *dev, void *drv_data,
    bool enable)
    {
    pub drv_data: *mut *mut cxl_patrol_scrub_context ctx =,
    pub wr_cycle: u8 cap, flags,,
    pub rd_cycle: u16,
    pub ret: c_int,
    if (!capable(CAP_SYS_RAWIO))
    pub -EPERM: return,
    pub NULL): ret = cxl_scrub_get_attrbs(ctx, &cap, &rd_cycle, &flags,,
    if (ret)
    pub ret: return,
    pub CXL_GET_SCRUB_CYCLE(rd_cycle): wr_cycle =,
    pub CXL_SET_SCRUB_EN(enable): flags =,
    pub flags): return cxl_scrub_set_attrbs(dev, ctx, wr_cycle,,
    }
    static int cxl_patrol_scrub_get_min_scrub_cycle(struct device *dev,
    void *drv_data, u32 *min)
    {
    pub drv_data: *mut *mut cxl_patrol_scrub_context ctx =,
    pub min_cycle: u8 cap, flags,,
    pub cycle: u16,
    pub ret: c_int,
    pub &min_cycle): ret = cxl_scrub_get_attrbs(ctx, &cap, &cycle, &flags,,
    if (ret)
    pub ret: return,
// min = min_cycle * 3600;
    pub 0: return,
    }
    static int cxl_patrol_scrub_get_max_scrub_cycle(struct device *dev,
    void *drv_data, u32 *max)
    {
// max = U8_MAX * 3600; /* Max set by register size
    pub 0: return,
    }
    static int cxl_patrol_scrub_get_scrub_cycle(struct device *dev, void *drv_data,
    u32 *scrub_cycle_secs)
    {
    pub drv_data: *mut *mut cxl_patrol_scrub_context ctx =,
    pub flags: u8 cap,,
    pub cycle: u16,
    pub ret: c_int,
    pub NULL): ret = cxl_scrub_get_attrbs(ctx, &cap, &cycle, &flags,,
    if (ret)
    pub ret: return,
// scrub_cycle_secs = CXL_GET_SCRUB_CYCLE(cycle) * 3600;
    pub 0: return,
    }
    static int cxl_patrol_scrub_set_scrub_cycle(struct device *dev, void *drv_data,
    u32 scrub_cycle_secs)
    {
    pub drv_data: *mut *mut cxl_patrol_scrub_context ctx =,
    pub 3600: u8 scrub_cycle_hours = scrub_cycle_secs /,
    pub min_cycle: u8 cap, wr_cycle, flags,,
    pub rd_cycle: u16,
    pub ret: c_int,
    if (!capable(CAP_SYS_RAWIO))
    pub -EPERM: return,
    pub &min_cycle): ret = cxl_scrub_get_attrbs(ctx, &cap, &rd_cycle, &flags,,
    if (ret)
    pub ret: return,
    if (!CXL_GET_SCRUB_CYCLE_CHANGEABLE(cap))
    pub -EOPNOTSUPP: return,
    if (scrub_cycle_hours < min_cycle) {
    dev_dbg(dev, "Invalid CXL patrol scrub cycle(%d) to set\n",
    dev_dbg(dev,
    "Minimum supported CXL patrol scrub cycle in hour %d\n",
    pub -EINVAL: return,
    }
    pub CXL_SET_SCRUB_CYCLE(scrub_cycle_hours): wr_cycle =,
    pub flags): return cxl_scrub_set_attrbs(dev, ctx, wr_cycle,,
    }
    static const struct edac_scrub_ops cxl_ps_scrub_ops = {
    .get_enabled_bg = cxl_patrol_scrub_get_enabled_bg,
    .set_enabled_bg = cxl_patrol_scrub_set_enabled_bg,
    .get_min_cycle = cxl_patrol_scrub_get_min_scrub_cycle,
    .get_max_cycle = cxl_patrol_scrub_get_max_scrub_cycle,
    .get_cycle_duration = cxl_patrol_scrub_get_scrub_cycle,
    .set_cycle_duration = cxl_patrol_scrub_set_scrub_cycle,
}

    static int cxl_memdev_scrub_init(struct cxl_memdev *cxlmd,
    struct edac_dev_feature *ras_feature,
    u8 scrub_inst)
    {
    struct cxl_patrol_scrub_context *cxl_ps_ctx;
    struct cxl_feat_entry *feat_entry;
    u8 cap, flags;
    u16 cycle;
    int rc;
    feat_entry = cxl_feature_info(to_cxlfs(cxlmd.cxlds),
    &CXL_FEAT_PATROL_SCRUB_UUID);
    if (IS_ERR(feat_entry))
    return -EOPNOTSUPP;
    if (!(le32_to_cpu(feat_entry.flags) & CXL_FEATURE_F_CHANGEABLE))
    return -EOPNOTSUPP;
    cxl_ps_ctx = devm_kzalloc(&cxlmd.dev, sizeof(*cxl_ps_ctx), GFP_KERNEL);
    if (!cxl_ps_ctx)
    return -ENOMEM;
// cxl_ps_ctx = (struct cxl_patrol_scrub_context){
    .get_feat_size = le16_to_cpu(feat_entry.get_feat_size),
    .set_feat_size = le16_to_cpu(feat_entry.set_feat_size),
    .get_version = feat_entry.get_feat_ver,
    .set_version = feat_entry.set_feat_ver,
    .effects = le16_to_cpu(feat_entry.effects),
    .instance = scrub_inst,
    .cxlmd = cxlmd,
    };
    rc = cxl_mem_scrub_get_attrbs(&cxlmd.cxlds.cxl_mbox, &cap, &cycle,
    &flags, core::ptr::null_mut());
    if (rc)
    return rc;
    cxlmd.scrub_cycle = CXL_GET_SCRUB_CYCLE(cycle);
    cxlmd.scrub_region_id = CXL_SCRUB_NO_REGION;
    ras_feature.ft_type = RAS_FEAT_SCRUB;
    ras_feature.instance = cxl_ps_ctx.instance;
    ras_feature.scrub_ops = &cxl_ps_scrub_ops;
    ras_feature.ctx = cxl_ps_ctx;
    return 0;
    }
    static int cxl_region_scrub_init(struct cxl_region *cxlr,
    struct edac_dev_feature *ras_feature,
    u8 scrub_inst)
    {
    struct cxl_patrol_scrub_context *cxl_ps_ctx;
    struct cxl_region_params *p = &cxlr.params;
    struct cxl_feat_entry *feat_entry = core::ptr::null_mut();
    struct cxl_memdev *cxlmd;
    u8 cap, flags;
    u16 cycle;
    int i, rc;
//
// The cxl_region_rwsem must be held if the code below is used in a context
// other than when the region is in the probe state, as shown here.
//
    for (i = 0; i < p.nr_targets; i++) {
    struct cxl_endpoint_decoder *cxled = p.targets[i];
    cxlmd = cxled_to_memdev(cxled);
    feat_entry = cxl_feature_info(to_cxlfs(cxlmd.cxlds),
    &CXL_FEAT_PATROL_SCRUB_UUID);
    if (IS_ERR(feat_entry))
    return -EOPNOTSUPP;
    if (!(le32_to_cpu(feat_entry.flags) &
    CXL_FEATURE_F_CHANGEABLE))
    return -EOPNOTSUPP;
    rc = cxl_mem_scrub_get_attrbs(&cxlmd.cxlds.cxl_mbox, &cap,
    &cycle, &flags, core::ptr::null_mut());
    if (rc)
    return rc;
    cxlmd.scrub_cycle = CXL_GET_SCRUB_CYCLE(cycle);
    cxlmd.scrub_region_id = CXL_SCRUB_NO_REGION;
    }
    cxl_ps_ctx = devm_kzalloc(&cxlr.dev, sizeof(*cxl_ps_ctx), GFP_KERNEL);
    if (!cxl_ps_ctx)
    return -ENOMEM;
// cxl_ps_ctx = (struct cxl_patrol_scrub_context){
    .get_feat_size = le16_to_cpu(feat_entry.get_feat_size),
    .set_feat_size = le16_to_cpu(feat_entry.set_feat_size),
    .get_version = feat_entry.get_feat_ver,
    .set_version = feat_entry.set_feat_ver,
    .effects = le16_to_cpu(feat_entry.effects),
    .instance = scrub_inst,
    .cxlr = cxlr,
    };
    ras_feature.ft_type = RAS_FEAT_SCRUB;
    ras_feature.instance = cxl_ps_ctx.instance;
    ras_feature.scrub_ops = &cxl_ps_scrub_ops;
    ras_feature.ctx = cxl_ps_ctx;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ecs_context {
    pub num_media_frus: u16,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub get_version: u8,
    pub set_version: u8,
    pub effects: u16,
    pub cxlmd: *mut cxl_memdev,
}

//
// See CXL spec rev 3.2 @8.2.10.9.11.2 Table 8-225 DDR5 ECS Control Feature
// Readable Attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ecs_fru_rd_attrbs {
    pub ecs_cap: u8,
    pub ecs_config: __le16,
    pub ecs_flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ecs_rd_attrbs {
    pub ecs_log_cap: u8,
    pub fru_attrbs: [cxl_ecs_fru_rd_attrbs; ],
    pub __packed: },
//
// See CXL spec rev 3.2 @8.2.10.9.11.2 Table 8-226 DDR5 ECS Control Feature
// Writable Attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ecs_fru_wr_attrbs {
    pub ecs_config: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ecs_wr_attrbs {
    pub ecs_log_cap: u8,
    pub fru_attrbs: [cxl_ecs_fru_wr_attrbs; ],
    pub __packed: },

pub const CXL_ECS_RESET_COUNTER: c_int = 1;
    enum {
    ECS_THRESHOLD_256 = 256,
    ECS_THRESHOLD_1024 = 1024,
    ECS_THRESHOLD_4096 = 4096,
}

    enum {
    ECS_THRESHOLD_IDX_256 = 3,
    ECS_THRESHOLD_IDX_1024 = 4,
    ECS_THRESHOLD_IDX_4096 = 5,
    };
    static const u16 ecs_supp_threshold[] = {
    [ECS_THRESHOLD_IDX_256] = 256,
    [ECS_THRESHOLD_IDX_1024] = 1024,
    [ECS_THRESHOLD_IDX_4096] = 4096,
    };
    enum {
    ECS_LOG_ENTRY_TYPE_DRAM = 0x0,
    ECS_LOG_ENTRY_TYPE_MEM_MEDIA_FRU = 0x1,
    };
    enum cxl_ecs_count_mode {
    ECS_MODE_COUNTS_ROWS = 0,
    ECS_MODE_COUNTS_CODEWORDS = 1,
    };
    static int cxl_mem_ecs_get_attrbs(struct device *dev,
    struct cxl_ecs_context *cxl_ecs_ctx,
    int fru_id, u8 *log_cap, u16 *config)
    {
    struct cxl_memdev *cxlmd = cxl_ecs_ctx.cxlmd;
    struct cxl_mailbox *cxl_mbox = &cxlmd.cxlds.cxl_mbox;
    struct cxl_ecs_fru_rd_attrbs *fru_rd_attrbs;
    size_t rd_data_size;
    size_t data_size;
    rd_data_size = cxl_ecs_ctx.get_feat_size;
    struct cxl_ecs_rd_attrbs *rd_attrbs __free(kvfree) =
    kvzalloc(rd_data_size, GFP_KERNEL);
    if (!rd_attrbs)
    return -ENOMEM;
    data_size = cxl_get_feature(cxl_mbox, &CXL_FEAT_ECS_UUID,
    CXL_GET_FEAT_SEL_CURRENT_VALUE, rd_attrbs,
    rd_data_size, 0, core::ptr::null_mut());
    if (!data_size)
    return -EIO;
    fru_rd_attrbs = rd_attrbs.fru_attrbs;
// log_cap = rd_attrbs->ecs_log_cap;
// config = le16_to_cpu(fru_rd_attrbs[fru_id].ecs_config);
    return 0;
    }
    static int cxl_mem_ecs_set_attrbs(struct device *dev,
    struct cxl_ecs_context *cxl_ecs_ctx,
    int fru_id, u8 log_cap, u16 config)
    {
    struct cxl_memdev *cxlmd = cxl_ecs_ctx.cxlmd;
    struct cxl_mailbox *cxl_mbox = &cxlmd.cxlds.cxl_mbox;
    struct cxl_ecs_fru_rd_attrbs *fru_rd_attrbs;
    struct cxl_ecs_fru_wr_attrbs *fru_wr_attrbs;
    size_t rd_data_size, wr_data_size;
    u16 num_media_frus, count;
    size_t data_size;
    num_media_frus = cxl_ecs_ctx.num_media_frus;
    rd_data_size = cxl_ecs_ctx.get_feat_size;
    wr_data_size = cxl_ecs_ctx.set_feat_size;
    struct cxl_ecs_rd_attrbs *rd_attrbs __free(kvfree) =
    kvzalloc(rd_data_size, GFP_KERNEL);
    if (!rd_attrbs)
    return -ENOMEM;
    data_size = cxl_get_feature(cxl_mbox, &CXL_FEAT_ECS_UUID,
    CXL_GET_FEAT_SEL_CURRENT_VALUE, rd_attrbs,
    rd_data_size, 0, core::ptr::null_mut());
    if (!data_size)
    return -EIO;
    struct cxl_ecs_wr_attrbs *wr_attrbs __free(kvfree) =
    kvzalloc(wr_data_size, GFP_KERNEL);
    if (!wr_attrbs)
    return -ENOMEM;
//
// Fill writable attributes from the current attributes read
// for all the media FRUs.
//
    fru_rd_attrbs = rd_attrbs.fru_attrbs;
    fru_wr_attrbs = wr_attrbs.fru_attrbs;
    wr_attrbs.ecs_log_cap = log_cap;
    for (count = 0; count < num_media_frus; count++)
    fru_wr_attrbs[count].ecs_config =
    fru_rd_attrbs[count].ecs_config;
    fru_wr_attrbs[fru_id].ecs_config = cpu_to_le16(config);
    return cxl_set_feature(cxl_mbox, &CXL_FEAT_ECS_UUID,
    cxl_ecs_ctx.set_version, wr_attrbs,
    wr_data_size,
    CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET,
    0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn cxl_get_ecs_log_entry_type(log_cap: u8, config: u16) -> u8 {
    static u8 cxl_get_ecs_log_entry_type(u8 log_cap, u16 config)
    {
    return FIELD_GET(CXL_ECS_LOG_ENTRY_TYPE_MASK, log_cap);
    }
#[no_mangle]
unsafe extern "C" fn cxl_get_ecs_threshold(log_cap: u8, config: u16) -> u16 {
    static u16 cxl_get_ecs_threshold(u8 log_cap, u16 config)
    {
    let mut index: u8 = FIELD_GET(CXL_ECS_THRESHOLD_COUNT_MASK, config);
    return ecs_supp_threshold[index];
    }
#[no_mangle]
unsafe extern "C" fn cxl_get_ecs_count_mode(log_cap: u8, config: u16) -> u8 {
    static u8 cxl_get_ecs_count_mode(u8 log_cap, u16 config)
    {
    return FIELD_GET(CXL_ECS_COUNT_MODE_MASK, config);
    }

    static int cxl_ecs_get_##attrb(struct device *dev, void *drv_data,  \
    int fru_id, u32 *val)		    \
    {								    \
    struct cxl_ecs_context *ctx = drv_data;			    \
    u8 log_cap;						    \
    u16 config;						    \
    int ret;						    \
    \
    ret = cxl_mem_ecs_get_attrbs(dev, ctx, fru_id, &log_cap,    \
    &config);			    \
    if (ret)						    \
    return ret;					    \
    \
// val = cxl_get_ecs_##attrb(log_cap, config);		    \
    \
    return 0;						    \
    }
    CXL_ECS_GET_ATTR(log_entry_type)
    CXL_ECS_GET_ATTR(count_mode)
    CXL_ECS_GET_ATTR(threshold)
    static int cxl_set_ecs_log_entry_type(struct device *dev, u8 *log_cap,
    u16 *config, u32 val)
    {
    if (val != ECS_LOG_ENTRY_TYPE_DRAM &&
    val != ECS_LOG_ENTRY_TYPE_MEM_MEDIA_FRU)
    return -EINVAL;
// log_cap = FIELD_PREP(CXL_ECS_LOG_ENTRY_TYPE_MASK, val);
    return 0;
    }
    static int cxl_set_ecs_threshold(struct device *dev, u8 *log_cap, u16 *config,
    u32 val)
    {
// config &= ~CXL_ECS_THRESHOLD_COUNT_MASK;
    switch (val) {
    case ECS_THRESHOLD_256:
// config |= FIELD_PREP(CXL_ECS_THRESHOLD_COUNT_MASK,
    ECS_THRESHOLD_IDX_256);
    break;
    case ECS_THRESHOLD_1024:
// config |= FIELD_PREP(CXL_ECS_THRESHOLD_COUNT_MASK,
    ECS_THRESHOLD_IDX_1024);
    break;
    case ECS_THRESHOLD_4096:
// config |= FIELD_PREP(CXL_ECS_THRESHOLD_COUNT_MASK,
    ECS_THRESHOLD_IDX_4096);
    break;
    default:
    dev_dbg(dev, "Invalid CXL ECS threshold count(%u) to set\n",
    val);
    dev_dbg(dev, "Supported ECS threshold counts: %u, %u, %u\n",
    ECS_THRESHOLD_256, ECS_THRESHOLD_1024,
    ECS_THRESHOLD_4096);
    return -EINVAL;
    }
    return 0;
    }
    static int cxl_set_ecs_count_mode(struct device *dev, u8 *log_cap, u16 *config,
    u32 val)
    {
    if (val != ECS_MODE_COUNTS_ROWS && val != ECS_MODE_COUNTS_CODEWORDS) {
    dev_dbg(dev, "Invalid CXL ECS scrub mode(%d) to set\n", val);
    dev_dbg(dev,
    "Supported ECS Modes: 0: ECS counts rows with errors,"
    " 1: ECS counts codewords with errors\n");
    return -EINVAL;
    }
// config &= ~CXL_ECS_COUNT_MODE_MASK;
// config |= FIELD_PREP(CXL_ECS_COUNT_MODE_MASK, val);
    return 0;
    }
    static int cxl_set_ecs_reset_counter(struct device *dev, u8 *log_cap,
    u16 *config, u32 val)
    {
    if (val != CXL_ECS_RESET_COUNTER)
    return -EINVAL;
// config &= ~CXL_ECS_RESET_COUNTER_MASK;
// config |= FIELD_PREP(CXL_ECS_RESET_COUNTER_MASK, val);
    return 0;
    }

    static int cxl_ecs_set_##attrb(struct device *dev, void *drv_data,  \
    int fru_id, u32 val)		    \
    {								    \
    struct cxl_ecs_context *ctx = drv_data;			    \
    u8 log_cap;						    \
    u16 config;						    \
    int ret;						    \
    \
    if (!capable(CAP_SYS_RAWIO))				    \
    return -EPERM;					    \
    \
    ret = cxl_mem_ecs_get_attrbs(dev, ctx, fru_id, &log_cap,    \
    &config);			    \
    if (ret)						    \
    return ret;					    \
    \
    ret = cxl_set_ecs_##attrb(dev, &log_cap, &config, val);     \
    if (ret)						    \
    return ret;					    \
    \
    return cxl_mem_ecs_set_attrbs(dev, ctx, fru_id, log_cap,    \
    config);			    \
    }
    CXL_ECS_SET_ATTR(log_entry_type)
    CXL_ECS_SET_ATTR(count_mode)
    CXL_ECS_SET_ATTR(reset_counter)
    CXL_ECS_SET_ATTR(threshold)
    static const struct edac_ecs_ops cxl_ecs_ops = {
    .get_log_entry_type = cxl_ecs_get_log_entry_type,
    .set_log_entry_type = cxl_ecs_set_log_entry_type,
    .get_mode = cxl_ecs_get_count_mode,
    .set_mode = cxl_ecs_set_count_mode,
    .reset = cxl_ecs_set_reset_counter,
    .get_threshold = cxl_ecs_get_threshold,
    .set_threshold = cxl_ecs_set_threshold,
    };
    static int cxl_memdev_ecs_init(struct cxl_memdev *cxlmd,
    struct edac_dev_feature *ras_feature)
    {
    struct cxl_ecs_context *cxl_ecs_ctx;
    struct cxl_feat_entry *feat_entry;
    int num_media_frus;
    feat_entry =
    cxl_feature_info(to_cxlfs(cxlmd.cxlds), &CXL_FEAT_ECS_UUID);
    if (IS_ERR(feat_entry))
    return -EOPNOTSUPP;
    if (!(le32_to_cpu(feat_entry.flags) & CXL_FEATURE_F_CHANGEABLE))
    return -EOPNOTSUPP;
    num_media_frus = (le16_to_cpu(feat_entry.get_feat_size) -
    sizeof(struct cxl_ecs_rd_attrbs)) /
    sizeof(struct cxl_ecs_fru_rd_attrbs);
    if (!num_media_frus)
    return -EOPNOTSUPP;
    cxl_ecs_ctx =
    devm_kzalloc(&cxlmd.dev, sizeof(*cxl_ecs_ctx), GFP_KERNEL);
    if (!cxl_ecs_ctx)
    return -ENOMEM;
// cxl_ecs_ctx = (struct cxl_ecs_context){
    .get_feat_size = le16_to_cpu(feat_entry.get_feat_size),
    .set_feat_size = le16_to_cpu(feat_entry.set_feat_size),
    .get_version = feat_entry.get_feat_ver,
    .set_version = feat_entry.set_feat_ver,
    .effects = le16_to_cpu(feat_entry.effects),
    .num_media_frus = num_media_frus,
    .cxlmd = cxlmd,
    };
    ras_feature.ft_type = RAS_FEAT_ECS;
    ras_feature.ecs_ops = &cxl_ecs_ops;
    ras_feature.ctx = cxl_ecs_ctx;
    ras_feature.ecs_info.num_media_frus = num_media_frus;
    return 0;
    }
//
// Perform Maintenance CXL 3.2 Spec 8.2.10.7.1
//
// Perform Maintenance input payload
// CXL rev 3.2 section 8.2.10.7.1 Table 8-117
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_maintenance_hdr {
    pub op_class: u8,
    pub op_subclass: u8,
    pub __packed: },
    static int cxl_perform_maintenance(struct cxl_mailbox *cxl_mbox, u8 class,
    u8 subclass, void *data_in,
    size_t data_in_size)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_maintenance_pi {
    pub hdr: cxl_mbox_maintenance_hdr,
    pub data: [u8; ],
    pub __packed: },
    pub mbox_cmd: cxl_mbox_cmd,
    pub hdr_size: usize,
    struct cxl_memdev_maintenance_pi *pi __free(kvfree) =
    pub GFP_KERNEL): kvzalloc(cxl_mbox->payload_size,,
    if (!pi)
    pub -ENOMEM: return,
    pub class: pi->hdr.op_class =,
    pub subclass: pi->hdr.op_subclass =,
    pub sizeof(pi->hdr): hdr_size =,
//
// Check minimum mbox payload size is available for
// the maintenance data transfer.
//
    if (hdr_size + data_in_size > cxl_mbox.payload_size)
    pub -ENOMEM: return,
    pub data_in_size): memcpy(pi->data, data_in,,
    mbox_cmd = (struct cxl_mbox_cmd){
    .opcode = CXL_MBOX_OP_DO_MAINTENANCE,
    .size_in = hdr_size + data_in_size,
    .payload_in = pi,
}

    return cxl_internal_send_cmd(cxl_mbox, &mbox_cmd);
    }
//
// Support for finding a memory operation attributes
// are from the current boot or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mem_err_rec {
    pub rec_gen_media: xarray,
    pub rec_dram: xarray,
}

    enum cxl_mem_repair_type {
    CXL_PPR,
    CXL_CACHELINE_SPARING,
    CXL_ROW_SPARING,
    CXL_BANK_SPARING,
    CXL_RANK_SPARING,
    CXL_REPAIR_MAX,
    };
//
// struct cxl_mem_repair_attrbs - CXL memory repair attributes
// @dpa: DPA of memory to repair
// @nibble_mask: nibble mask, identifies one or more nibbles on the memory bus
// @row: row of memory to repair
// @column: column of memory to repair
// @channel: channel of memory to repair
// @sub_channel: sub channel of memory to repair
// @rank: rank of memory to repair
// @bank_group: bank group of memory to repair
// @bank: bank of memory to repair
// @repair_type: repair type. For eg. PPR, memory sparing etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mem_repair_attrbs {
    pub dpa: u64,
    pub nibble_mask: u32,
    pub row: u32,
    pub column: u16,
    pub channel: u8,
    pub sub_channel: u8,
    pub rank: u8,
    pub bank_group: u8,
    pub bank: u8,
    pub repair_type: enum cxl_mem_repair_type,
}

    static struct cxl_event_gen_media *
    cxl_find_rec_gen_media(struct cxl_memdev *cxlmd,
    struct cxl_mem_repair_attrbs *attrbs)
    {
    struct cxl_mem_err_rec *array_rec = cxlmd.err_rec_array;
    struct cxl_event_gen_media *rec;
    if (!array_rec)
    return core::ptr::null_mut();
    rec = xa_load(&array_rec.rec_gen_media, attrbs.dpa);
    if (!rec)
    return core::ptr::null_mut();
    if (attrbs.repair_type == CXL_PPR)
    return rec;
    return core::ptr::null_mut();
    }
    static struct cxl_event_dram *
    cxl_find_rec_dram(struct cxl_memdev *cxlmd,
    struct cxl_mem_repair_attrbs *attrbs)
    {
    struct cxl_mem_err_rec *array_rec = cxlmd.err_rec_array;
    struct cxl_event_dram *rec;
    u16 validity_flags;
    if (!array_rec)
    return core::ptr::null_mut();
    rec = xa_load(&array_rec.rec_dram, attrbs.dpa);
    if (!rec)
    return core::ptr::null_mut();
    validity_flags = get_unaligned_le16(rec.media_hdr.validity_flags);
    if (!(validity_flags & CXL_DER_VALID_CHANNEL) ||
    !(validity_flags & CXL_DER_VALID_RANK))
    return core::ptr::null_mut();
    switch (attrbs.repair_type) {
    case CXL_PPR:
    if (!(validity_flags & CXL_DER_VALID_NIBBLE) ||
    get_unaligned_le24(rec.nibble_mask) == attrbs.nibble_mask)
    return rec;
    break;
    case CXL_CACHELINE_SPARING:
    if (!(validity_flags & CXL_DER_VALID_BANK_GROUP) ||
    !(validity_flags & CXL_DER_VALID_BANK) ||
    !(validity_flags & CXL_DER_VALID_ROW) ||
    !(validity_flags & CXL_DER_VALID_COLUMN))
    return core::ptr::null_mut();
    if (rec.media_hdr.channel == attrbs.channel &&
    rec.media_hdr.rank == attrbs.rank &&
    rec.bank_group == attrbs.bank_group &&
    rec.bank == attrbs.bank &&
    get_unaligned_le24(rec.row) == attrbs.row &&
    get_unaligned_le16(rec.column) == attrbs.column &&
    (!(validity_flags & CXL_DER_VALID_NIBBLE) ||
    get_unaligned_le24(rec.nibble_mask) ==
    attrbs.nibble_mask) &&
    (!(validity_flags & CXL_DER_VALID_SUB_CHANNEL) ||
    rec.sub_channel == attrbs.sub_channel))
    return rec;
    break;
    case CXL_ROW_SPARING:
    if (!(validity_flags & CXL_DER_VALID_BANK_GROUP) ||
    !(validity_flags & CXL_DER_VALID_BANK) ||
    !(validity_flags & CXL_DER_VALID_ROW))
    return core::ptr::null_mut();
    if (rec.media_hdr.channel == attrbs.channel &&
    rec.media_hdr.rank == attrbs.rank &&
    rec.bank_group == attrbs.bank_group &&
    rec.bank == attrbs.bank &&
    get_unaligned_le24(rec.row) == attrbs.row &&
    (!(validity_flags & CXL_DER_VALID_NIBBLE) ||
    get_unaligned_le24(rec.nibble_mask) ==
    attrbs.nibble_mask))
    return rec;
    break;
    case CXL_BANK_SPARING:
    if (!(validity_flags & CXL_DER_VALID_BANK_GROUP) ||
    !(validity_flags & CXL_DER_VALID_BANK))
    return core::ptr::null_mut();
    if (rec.media_hdr.channel == attrbs.channel &&
    rec.media_hdr.rank == attrbs.rank &&
    rec.bank_group == attrbs.bank_group &&
    rec.bank == attrbs.bank &&
    (!(validity_flags & CXL_DER_VALID_NIBBLE) ||
    get_unaligned_le24(rec.nibble_mask) ==
    attrbs.nibble_mask))
    return rec;
    break;
    case CXL_RANK_SPARING:
    if (rec.media_hdr.channel == attrbs.channel &&
    rec.media_hdr.rank == attrbs.rank &&
    (!(validity_flags & CXL_DER_VALID_NIBBLE) ||
    get_unaligned_le24(rec.nibble_mask) ==
    attrbs.nibble_mask))
    return rec;
    break;
    default:
    return core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
pub const CXL_MAX_STORAGE_DAYS: c_int = 10;

    static void cxl_del_expired_gmedia_recs(struct xarray *rec_xarray,
    struct cxl_event_gen_media *cur_rec)
    {
    let mut cur_ts: u64 = le64_to_cpu(cur_rec.media_hdr.hdr.timestamp);
    struct cxl_event_gen_media *rec;
    unsigned long index;
    u64 delta_ts_secs;
    xa_for_each(rec_xarray, index, rec) {
    delta_ts_secs = (cur_ts -
    le64_to_cpu(rec.media_hdr.hdr.timestamp)) / 1000000000ULL;
    if (delta_ts_secs >= CXL_MAX_STORAGE_TIME_SECS) {
    xa_erase(rec_xarray, index);
    kfree(rec);
    }
    }
    }
    static void cxl_del_expired_dram_recs(struct xarray *rec_xarray,
    struct cxl_event_dram *cur_rec)
    {
    let mut cur_ts: u64 = le64_to_cpu(cur_rec.media_hdr.hdr.timestamp);
    struct cxl_event_dram *rec;
    unsigned long index;
    u64 delta_secs;
    xa_for_each(rec_xarray, index, rec) {
    delta_secs = (cur_ts -
    le64_to_cpu(rec.media_hdr.hdr.timestamp)) / 1000000000ULL;
    if (delta_secs >= CXL_MAX_STORAGE_TIME_SECS) {
    xa_erase(rec_xarray, index);
    kfree(rec);
    }
    }
    }
pub const CXL_MAX_REC_STORAGE_COUNT: c_int = 200;
#[no_mangle]
unsafe extern "C" fn cxl_del_overflow_old_recs(rec_xarray: *mut xarray) {
    static void cxl_del_overflow_old_recs(struct xarray *rec_xarray)
    {
    void *err_rec;
    unsigned long index, count = 0;
    xa_for_each(rec_xarray, index, err_rec)
    count++;
    if (count <= CXL_MAX_REC_STORAGE_COUNT)
    return;
    count -= CXL_MAX_REC_STORAGE_COUNT;
    xa_for_each(rec_xarray, index, err_rec) {
    xa_erase(rec_xarray, index);
    kfree(err_rec);
    count--;
    if (!count)
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_store_rec_gen_media(cxlmd: *mut cxl_memdev, evt: *mut union cxl_event) -> c_int {
    int cxl_store_rec_gen_media(struct cxl_memdev *cxlmd, union cxl_event *evt)
    {
    struct cxl_mem_err_rec *array_rec = cxlmd.err_rec_array;
    struct cxl_event_gen_media *rec;
    void *old_rec;
    if (!IS_ENABLED(CONFIG_CXL_EDAC_MEM_REPAIR) || !array_rec)
    return 0;
    rec = kmemdup(&evt.gen_media, sizeof(*rec), GFP_KERNEL);
    if (!rec)
    return -ENOMEM;
    old_rec = xa_store(&array_rec.rec_gen_media,
    le64_to_cpu(rec.media_hdr.phys_addr), rec,
    GFP_KERNEL);
    if (xa_is_err(old_rec)) {
    kfree(rec);
    return xa_err(old_rec);
    }
    kfree(old_rec);
    cxl_del_expired_gmedia_recs(&array_rec.rec_gen_media, rec);
    cxl_del_overflow_old_recs(&array_rec.rec_gen_media);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(cxl_store_rec_gen_media, "CXL");
#[no_mangle]
pub unsafe extern "C" fn cxl_store_rec_dram(cxlmd: *mut cxl_memdev, evt: *mut union cxl_event) -> c_int {
    int cxl_store_rec_dram(struct cxl_memdev *cxlmd, union cxl_event *evt)
    {
    struct cxl_mem_err_rec *array_rec = cxlmd.err_rec_array;
    struct cxl_event_dram *rec;
    void *old_rec;
    if (!IS_ENABLED(CONFIG_CXL_EDAC_MEM_REPAIR) || !array_rec)
    return 0;
    rec = kmemdup(&evt.dram, sizeof(*rec), GFP_KERNEL);
    if (!rec)
    return -ENOMEM;
    old_rec = xa_store(&array_rec.rec_dram,
    le64_to_cpu(rec.media_hdr.phys_addr), rec,
    GFP_KERNEL);
    if (xa_is_err(old_rec)) {
    kfree(rec);
    return xa_err(old_rec);
    }
    kfree(old_rec);
    cxl_del_expired_dram_recs(&array_rec.rec_dram, rec);
    cxl_del_overflow_old_recs(&array_rec.rec_dram);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(cxl_store_rec_dram, "CXL");
#[no_mangle]
unsafe extern "C" fn cxl_is_memdev_memory_online(cxlmd: *const cxl_memdev) -> bool {
    static bool cxl_is_memdev_memory_online(const struct cxl_memdev *cxlmd)
    {
    struct cxl_port *port = cxlmd.endpoint;
    if (port && cxl_num_decoders_committed(port))
    return true;
    return false;
    }
//
// CXL memory sparing control
//
    enum cxl_mem_sparing_granularity {
    CXL_MEM_SPARING_CACHELINE,
    CXL_MEM_SPARING_ROW,
    CXL_MEM_SPARING_BANK,
    CXL_MEM_SPARING_RANK,
    CXL_MEM_SPARING_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mem_sparing_context {
    pub cxlmd: *mut cxl_memdev,
    pub repair_uuid: uuid_t,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub effects: u16,
    pub instance: u8,
    pub get_version: u8,
    pub set_version: u8,
    pub op_class: u8,
    pub op_subclass: u8,
    pub cap_safe_when_in_use: bool,
    pub cap_hard_sparing: bool,
    pub cap_soft_sparing: bool,
    pub channel: u8,
    pub rank: u8,
    pub bank_group: u8,
    pub nibble_mask: u32,
    pub dpa: u64,
    pub row: u32,
    pub column: u16,
    pub bank: u8,
    pub sub_channel: u8,
    pub repair_type: enum edac_mem_repair_type,
    pub persist_mode: bool,
}

    (FIELD_GET(CXL_SPARING_RD_CAP_SAFE_IN_USE_MASK, \
    flags) ^ 1)

    FIELD_GET(CXL_SPARING_RD_CAP_HARD_SPARING_MASK, \
    flags)

    FIELD_GET(CXL_SPARING_RD_CAP_SOFT_SPARING_MASK, \
    flags)

    FIELD_PREP(CXL_SPARING_QUERY_RESOURCE_FLAG, val)

    FIELD_PREP(CXL_SET_HARD_SPARING_FLAG, val)

    FIELD_PREP(CXL_SPARING_SUB_CHNL_VALID_FLAG, val)

    FIELD_PREP(CXL_SPARING_NIB_MASK_VALID_FLAG, val)
//
// See CXL spec rev 3.2 @8.2.10.7.2.3 Table 8-134 Memory Sparing Feature
// Readable Attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_repair_rd_attrbs_hdr {
    pub max_op_latency: u8,
    pub op_cap: __le16,
    pub op_mode: __le16,
    pub op_class: u8,
    pub op_subclass: u8,
    pub rsvd: [u8; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_sparing_rd_attrbs {
    pub hdr: cxl_memdev_repair_rd_attrbs_hdr,
    pub rsvd: u8,
    pub restriction_flags: __le16,
    pub __packed: },
//
// See CXL spec rev 3.2 @8.2.10.7.1.4 Table 8-120 Memory Sparing Input Payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_sparing_in_payload {
    pub flags: u8,
    pub channel: u8,
    pub rank: u8,
    pub nibble_mask: [u8; 3],
    pub bank_group: u8,
    pub bank: u8,
    pub row: [u8; 3],
    pub column: __le16,
    pub sub_channel: u8,
    pub __packed: },
    static int
    cxl_mem_sparing_get_attrbs(struct cxl_mem_sparing_context *cxl_sparing_ctx)
    {
    pub cxl_memdev_sparing_rd_attrbs): size_t rd_data_size = sizeof(struct,
    pub cxl_sparing_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub &cxlmd->cxlds->cxl_mbox: *mut *mut cxl_mailbox cxl_mbox =,
    pub restriction_flags: u16,
    pub data_size: usize,
    pub return_code: u16,
    struct cxl_memdev_sparing_rd_attrbs *rd_attrbs __free(kfree) =
    pub GFP_KERNEL): kzalloc(rd_data_size,,
    if (!rd_attrbs)
    pub -ENOMEM: return,
    data_size = cxl_get_feature(cxl_mbox, &cxl_sparing_ctx.repair_uuid,
    CXL_GET_FEAT_SEL_CURRENT_VALUE, rd_attrbs,
    pub &return_code): rd_data_size, 0,,
    if (!data_size)
    pub -EIO: return,
    pub rd_attrbs->hdr.op_class: cxl_sparing_ctx->op_class =,
    pub rd_attrbs->hdr.op_subclass: cxl_sparing_ctx->op_subclass =,
    pub le16_to_cpu(rd_attrbs->restriction_flags): restriction_flags =,
    cxl_sparing_ctx.cap_safe_when_in_use =
    cxl_sparing_ctx.cap_hard_sparing =
    cxl_sparing_ctx.cap_soft_sparing =
    pub 0: return,
    }
    static struct cxl_event_dram *
    cxl_mem_get_rec_dram(struct cxl_memdev *cxlmd,
    struct cxl_mem_sparing_context *ctx)
    {
    pub }: cxl_mem_repair_attrbs attrbs = { 0,
    pub ctx->dpa: attrbs.dpa =,
    pub ctx->channel: attrbs.channel =,
    pub ctx->rank: attrbs.rank =,
    pub ctx->nibble_mask: attrbs.nibble_mask =,
    switch (ctx.repair_type) {
    case EDAC_REPAIR_CACHELINE_SPARING:
    pub CXL_CACHELINE_SPARING: attrbs.repair_type =,
    pub ctx->bank_group: attrbs.bank_group =,
    pub ctx->bank: attrbs.bank =,
    pub ctx->row: attrbs.row =,
    pub ctx->column: attrbs.column =,
    pub ctx->sub_channel: attrbs.sub_channel =,
    case EDAC_REPAIR_ROW_SPARING:
    pub CXL_ROW_SPARING: attrbs.repair_type =,
    pub ctx->bank_group: attrbs.bank_group =,
    pub ctx->bank: attrbs.bank =,
    pub ctx->row: attrbs.row =,
    case EDAC_REPAIR_BANK_SPARING:
    pub CXL_BANK_SPARING: attrbs.repair_type =,
    pub ctx->bank_group: attrbs.bank_group =,
    pub ctx->bank: attrbs.bank =,
    case EDAC_REPAIR_RANK_SPARING:
    pub CXL_RANK_SPARING: attrbs.repair_type =,
    default:
    pub NULL: return,
    }
    pub &attrbs): return cxl_find_rec_dram(cxlmd,,
    }
    static int
    cxl_mem_perform_sparing(struct device *dev,
    struct cxl_mem_sparing_context *cxl_sparing_ctx)
    {
    pub cxl_sparing_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub sparing_pi: cxl_memdev_sparing_in_payload,
    pub NULL: *mut *mut cxl_event_dram rec =,
    pub 0: u16 validity_flags =,
    pub ret: c_int,
    pub region_rwsem)(&cxl_rwsem.region): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &region_rwsem)))
    pub ret: return,
    pub dpa_rwsem)(&cxl_rwsem.dpa): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &dpa_rwsem)))
    pub ret: return,
    if (!cxl_sparing_ctx.cap_safe_when_in_use) {
// Memory to repair must be offline
    if (cxl_is_memdev_memory_online(cxlmd))
    pub -EBUSY: return,
    } else {
    if (cxl_is_memdev_memory_online(cxlmd)) {
    pub cxl_sparing_ctx): rec = cxl_mem_get_rec_dram(cxlmd,,
    if (!rec)
    pub -EINVAL: return,
    if (!get_unaligned_le16(rec.media_hdr.validity_flags))
    pub -EINVAL: return,
    }
    }
    pub sizeof(sparing_pi)): memset(&sparing_pi, 0,,
    pub CXL_SET_SPARING_QUERY_RESOURCE(0): sparing_pi.flags =,
    if (cxl_sparing_ctx.persist_mode)
    pub CXL_SET_HARD_SPARING(1): sparing_pi.flags |=,
    if (rec)
    pub get_unaligned_le16(rec->media_hdr.validity_flags): validity_flags =,
    switch (cxl_sparing_ctx.repair_type) {
    case EDAC_REPAIR_CACHELINE_SPARING:
    pub cpu_to_le16(cxl_sparing_ctx->column): sparing_pi.column =,
    if (!rec || (validity_flags & CXL_DER_VALID_SUB_CHANNEL)) {
    pub CXL_SET_SPARING_SUB_CHNL_VALID(1): sparing_pi.flags |=,
    pub cxl_sparing_ctx->sub_channel: sparing_pi.sub_channel =,
    }
    case EDAC_REPAIR_ROW_SPARING:
    pub sparing_pi.row): put_unaligned_le24(cxl_sparing_ctx->row,,
    case EDAC_REPAIR_BANK_SPARING:
    pub cxl_sparing_ctx->bank_group: sparing_pi.bank_group =,
    pub cxl_sparing_ctx->bank: sparing_pi.bank =,
    case EDAC_REPAIR_RANK_SPARING:
    pub cxl_sparing_ctx->rank: sparing_pi.rank =,
    default:
    pub cxl_sparing_ctx->channel: sparing_pi.channel =,
    if ((rec && (validity_flags & CXL_DER_VALID_NIBBLE)) ||
    (!rec && (!cxl_sparing_ctx.nibble_mask ||
    (cxl_sparing_ctx.nibble_mask & 0xFFFFFF)))) {
    pub CXL_SET_SPARING_NIB_MASK_VALID(1): sparing_pi.flags |=,
    put_unaligned_le24(cxl_sparing_ctx.nibble_mask,
    }
    }
    return cxl_perform_maintenance(&cxlmd.cxlds.cxl_mbox,
    cxl_sparing_ctx.op_class,
    cxl_sparing_ctx.op_subclass,
    pub sizeof(sparing_pi)): &sparing_pi,,
    }
    static int cxl_mem_sparing_get_repair_type(struct device *dev, void *drv_data,
    const char **repair_type)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    switch (ctx.repair_type) {
    case EDAC_REPAIR_CACHELINE_SPARING:
    case EDAC_REPAIR_ROW_SPARING:
    case EDAC_REPAIR_BANK_SPARING:
    case EDAC_REPAIR_RANK_SPARING:
// repair_type = edac_repair_type[ctx->repair_type];
    default:
    pub -EINVAL: return,
    }
    pub 0: return,
    }

    static int cxl_mem_sparing_get_##attrb(			    \
    struct device *dev, void *drv_data, data_type *val) \
    {							    \
    pub \: *mut *mut cxl_mem_sparing_context ctx = drv_data;,
    \
// val = ctx->attrb;				    \
    \
    pub \: return 0;,
    }
    CXL_SPARING_GET_ATTR(persist_mode, bool)
    CXL_SPARING_GET_ATTR(dpa, u64)
    CXL_SPARING_GET_ATTR(nibble_mask, u32)
    CXL_SPARING_GET_ATTR(bank_group, u32)
    CXL_SPARING_GET_ATTR(bank, u32)
    CXL_SPARING_GET_ATTR(rank, u32)
    CXL_SPARING_GET_ATTR(row, u32)
    CXL_SPARING_GET_ATTR(column, u32)
    CXL_SPARING_GET_ATTR(channel, u32)
    CXL_SPARING_GET_ATTR(sub_channel, u32)

    static int cxl_mem_sparing_set_##attrb(struct device *dev,		\
    void *drv_data, data_type val)	\
    {									\
    pub \: *mut *mut cxl_mem_sparing_context ctx = drv_data;,
    \
    pub \: ctx->attrb = val;,
    \
    pub \: return 0;,
    }
    CXL_SPARING_SET_ATTR(nibble_mask, u32)
    CXL_SPARING_SET_ATTR(bank_group, u32)
    CXL_SPARING_SET_ATTR(bank, u32)
    CXL_SPARING_SET_ATTR(rank, u32)
    CXL_SPARING_SET_ATTR(row, u32)
    CXL_SPARING_SET_ATTR(column, u32)
    CXL_SPARING_SET_ATTR(channel, u32)
    CXL_SPARING_SET_ATTR(sub_channel, u32)
    static int cxl_mem_sparing_set_persist_mode(struct device *dev, void *drv_data,
    bool persist_mode)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    if ((persist_mode && ctx.cap_hard_sparing) ||
    (!persist_mode && ctx.cap_soft_sparing))
    pub persist_mode: ctx->persist_mode =,
    else
    pub -EOPNOTSUPP: return,
    pub 0: return,
    }
    static int cxl_get_mem_sparing_safe_when_in_use(struct device *dev,
    void *drv_data, bool *safe)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
// safe = ctx->cap_safe_when_in_use;
    pub 0: return,
    }
    static int cxl_mem_sparing_get_min_dpa(struct device *dev, void *drv_data,
    u64 *min_dpa)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    pub ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
// min_dpa = cxlds->dpa_res.start;
    pub 0: return,
    }
    static int cxl_mem_sparing_get_max_dpa(struct device *dev, void *drv_data,
    u64 *max_dpa)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    pub ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
// max_dpa = cxlds->dpa_res.end;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_mem_sparing_set_dpa(dev: *mut device, drv_data: *mut c_void, dpa: u64) -> c_int {
    static int cxl_mem_sparing_set_dpa(struct device *dev, void *drv_data, u64 dpa)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    pub ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
    if (!cxl_resource_contains_addr(&cxlds.dpa_res, dpa))
    pub -EINVAL: return,
    pub dpa: ctx->dpa =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_do_mem_sparing(dev: *mut device, drv_data: *mut c_void, val: u32) -> c_int {
    static int cxl_do_mem_sparing(struct device *dev, void *drv_data, u32 val)
    {
    pub drv_data: *mut *mut cxl_mem_sparing_context ctx =,
    if (val != EDAC_DO_MEM_REPAIR)
    pub -EINVAL: return,
    pub ctx): return cxl_mem_perform_sparing(dev,,
    }

    .get_repair_type = cxl_mem_sparing_get_repair_type,                  \
    .get_persist_mode = cxl_mem_sparing_get_persist_mode,                \
    .set_persist_mode = cxl_mem_sparing_set_persist_mode,                \
    .get_repair_safe_when_in_use = cxl_get_mem_sparing_safe_when_in_use, \
    .get_min_dpa = cxl_mem_sparing_get_min_dpa,                          \
    .get_max_dpa = cxl_mem_sparing_get_max_dpa,                          \
    .get_dpa = cxl_mem_sparing_get_dpa,                                  \
    .set_dpa = cxl_mem_sparing_set_dpa,                                  \
    .get_nibble_mask = cxl_mem_sparing_get_nibble_mask,                  \
    .set_nibble_mask = cxl_mem_sparing_set_nibble_mask,                  \
    .get_rank = cxl_mem_sparing_get_rank,                                \
    .set_rank = cxl_mem_sparing_set_rank,                                \
    .get_channel = cxl_mem_sparing_get_channel,                          \
    .set_channel = cxl_mem_sparing_set_channel,                          \
    .do_repair = cxl_do_mem_sparing

    RANK_OPS, .get_bank_group = cxl_mem_sparing_get_bank_group, \
    .set_bank_group = cxl_mem_sparing_set_bank_group,   \
    .get_bank = cxl_mem_sparing_get_bank,               \
    .set_bank = cxl_mem_sparing_set_bank

    BANK_OPS, .get_row = cxl_mem_sparing_get_row, \
    .set_row = cxl_mem_sparing_set_row

    ROW_OPS, .get_column = cxl_mem_sparing_get_column,          \
    .set_column = cxl_mem_sparing_set_column,           \
    .get_sub_channel = cxl_mem_sparing_get_sub_channel, \
    .set_sub_channel = cxl_mem_sparing_set_sub_channel
    static const struct edac_mem_repair_ops cxl_rank_sparing_ops = {
    RANK_OPS,
}

    static const struct edac_mem_repair_ops cxl_bank_sparing_ops = {
    BANK_OPS,
    };
    static const struct edac_mem_repair_ops cxl_row_sparing_ops = {
    ROW_OPS,
    };
    static const struct edac_mem_repair_ops cxl_cacheline_sparing_ops = {
    CACHELINE_OPS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mem_sparing_desc {
    pub repair_uuid: uuid_t,
    pub repair_type: enum edac_mem_repair_type,
    pub repair_ops: *const edac_mem_repair_ops,
}

    static const struct cxl_mem_sparing_desc mem_sparing_desc[] = {
    {
    .repair_uuid = CXL_FEAT_CACHELINE_SPARING_UUID,
    .repair_type = EDAC_REPAIR_CACHELINE_SPARING,
    .repair_ops = &cxl_cacheline_sparing_ops,
    },
    {
    .repair_uuid = CXL_FEAT_ROW_SPARING_UUID,
    .repair_type = EDAC_REPAIR_ROW_SPARING,
    .repair_ops = &cxl_row_sparing_ops,
    },
    {
    .repair_uuid = CXL_FEAT_BANK_SPARING_UUID,
    .repair_type = EDAC_REPAIR_BANK_SPARING,
    .repair_ops = &cxl_bank_sparing_ops,
    },
    {
    .repair_uuid = CXL_FEAT_RANK_SPARING_UUID,
    .repair_type = EDAC_REPAIR_RANK_SPARING,
    .repair_ops = &cxl_rank_sparing_ops,
    },
    };
    static int cxl_memdev_sparing_init(struct cxl_memdev *cxlmd,
    struct edac_dev_feature *ras_feature,
    const struct cxl_mem_sparing_desc *desc,
    u8 repair_inst)
    {
    struct cxl_mem_sparing_context *cxl_sparing_ctx;
    struct cxl_feat_entry *feat_entry;
    int ret;
    feat_entry = cxl_feature_info(to_cxlfs(cxlmd.cxlds),
    &desc.repair_uuid);
    if (IS_ERR(feat_entry))
    return -EOPNOTSUPP;
    if (!(le32_to_cpu(feat_entry.flags) & CXL_FEATURE_F_CHANGEABLE))
    return -EOPNOTSUPP;
    cxl_sparing_ctx = devm_kzalloc(&cxlmd.dev, sizeof(*cxl_sparing_ctx),
    GFP_KERNEL);
    if (!cxl_sparing_ctx)
    return -ENOMEM;
// cxl_sparing_ctx = (struct cxl_mem_sparing_context){
    .get_feat_size = le16_to_cpu(feat_entry.get_feat_size),
    .set_feat_size = le16_to_cpu(feat_entry.set_feat_size),
    .get_version = feat_entry.get_feat_ver,
    .set_version = feat_entry.set_feat_ver,
    .effects = le16_to_cpu(feat_entry.effects),
    .cxlmd = cxlmd,
    .repair_type = desc.repair_type,
    .instance = repair_inst++,
    };
    uuid_copy(&cxl_sparing_ctx.repair_uuid, &desc.repair_uuid);
    ret = cxl_mem_sparing_get_attrbs(cxl_sparing_ctx);
    if (ret)
    return ret;
    if ((cxl_sparing_ctx.cap_soft_sparing &&
    cxl_sparing_ctx.cap_hard_sparing) ||
    cxl_sparing_ctx.cap_soft_sparing)
    cxl_sparing_ctx.persist_mode = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cxl_sparing_ctx->cap_hard_sparing) -> else {
    else if (cxl_sparing_ctx.cap_hard_sparing)
    cxl_sparing_ctx.persist_mode = 1;
    else
    return -EOPNOTSUPP;
    ras_feature.ft_type = RAS_FEAT_MEM_REPAIR;
    ras_feature.instance = cxl_sparing_ctx.instance;
    ras_feature.mem_repair_ops = desc.repair_ops;
    ras_feature.ctx = cxl_sparing_ctx;
    return 0;
    }
//
// CXL memory soft PPR & hard PPR control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ppr_context {
    pub repair_uuid: uuid_t,
    pub instance: u8,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub get_version: u8,
    pub set_version: u8,
    pub effects: u16,
    pub op_class: u8,
    pub op_subclass: u8,
    pub cap_dpa: bool,
    pub cap_nib_mask: bool,
    pub media_accessible: bool,
    pub data_retained: bool,
    pub cxlmd: *mut cxl_memdev,
    pub repair_type: enum edac_mem_repair_type,
    pub persist_mode: bool,
    pub dpa: u64,
    pub nibble_mask: u32,
}

//
// See CXL rev 3.2 @8.2.10.7.2.1 Table 8-128 sPPR Feature Readable Attributes
//
// See CXL rev 3.2 @8.2.10.7.2.2 Table 8-131 hPPR Feature Readable Attributes
//

    FIELD_GET(CXL_PPR_FLAG_DPA_SUPPORT_MASK, flags)

    FIELD_GET(CXL_PPR_FLAG_NIB_SUPPORT_MASK, flags)

    (FIELD_GET(CXL_PPR_RESTRICTION_FLAG_MEDIA_ACCESSIBLE_MASK, \
    restriction_flags) ^ 1)

    (FIELD_GET(CXL_PPR_RESTRICTION_FLAG_DATA_RETAINED_MASK, \
    restriction_flags) ^ 1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_ppr_rd_attrbs {
    pub hdr: cxl_memdev_repair_rd_attrbs_hdr,
    pub ppr_flags: u8,
    pub restriction_flags: __le16,
    pub ppr_op_mode: u8,
    pub __packed: },
//
// See CXL rev 3.2 @8.2.10.7.1.2 Table 8-118 sPPR Maintenance Input Payload
//
// See CXL rev 3.2 @8.2.10.7.1.3 Table 8-119 hPPR Maintenance Input Payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_ppr_maintenance_attrbs {
    pub flags: u8,
    pub dpa: __le64,
    pub nibble_mask: [u8; 3],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn cxl_mem_ppr_get_attrbs(cxl_ppr_ctx: *mut cxl_ppr_context) -> c_int {
    static int cxl_mem_ppr_get_attrbs(struct cxl_ppr_context *cxl_ppr_ctx)
    {
    pub cxl_memdev_ppr_rd_attrbs): size_t rd_data_size = sizeof(struct,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub &cxlmd->cxlds->cxl_mbox: *mut *mut cxl_mailbox cxl_mbox =,
    pub restriction_flags: u16,
    pub data_size: usize,
    pub return_code: u16,
    struct cxl_memdev_ppr_rd_attrbs *rd_attrbs __free(kfree) =
    pub GFP_KERNEL): kmalloc(rd_data_size,,
    if (!rd_attrbs)
    pub -ENOMEM: return,
    data_size = cxl_get_feature(cxl_mbox, &cxl_ppr_ctx.repair_uuid,
    CXL_GET_FEAT_SEL_CURRENT_VALUE, rd_attrbs,
    pub &return_code): rd_data_size, 0,,
    if (!data_size)
    pub -EIO: return,
    pub rd_attrbs->hdr.op_class: cxl_ppr_ctx->op_class =,
    pub rd_attrbs->hdr.op_subclass: cxl_ppr_ctx->op_subclass =,
    pub CXL_PPR_GET_CAP_DPA(rd_attrbs->ppr_flags): cxl_ppr_ctx->cap_dpa =,
    cxl_ppr_ctx.cap_nib_mask =
    pub le16_to_cpu(rd_attrbs->restriction_flags): restriction_flags =,
    cxl_ppr_ctx.media_accessible =
    cxl_ppr_ctx.data_retained =
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_mem_perform_ppr(cxl_ppr_ctx: *mut cxl_ppr_context) -> c_int {
    static int cxl_mem_perform_ppr(struct cxl_ppr_context *cxl_ppr_ctx)
    {
    pub maintenance_attrbs: cxl_memdev_ppr_maintenance_attrbs,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub }: cxl_mem_repair_attrbs attrbs = { 0,
    pub ret: c_int,
    pub region_rwsem)(&cxl_rwsem.region): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &region_rwsem)))
    pub ret: return,
    pub dpa_rwsem)(&cxl_rwsem.dpa): ACQUIRE(rwsem_read_intr,,
    if ((ret = ACQUIRE_ERR(rwsem_read_intr, &dpa_rwsem)))
    pub ret: return,
    if (!cxl_ppr_ctx.media_accessible || !cxl_ppr_ctx.data_retained) {
// Memory to repair must be offline
    if (cxl_is_memdev_memory_online(cxlmd))
    pub -EBUSY: return,
    } else {
    if (cxl_is_memdev_memory_online(cxlmd)) {
// Check memory to repair is from the current boot
    pub CXL_PPR: attrbs.repair_type =,
    pub cxl_ppr_ctx->dpa: attrbs.dpa =,
    pub cxl_ppr_ctx->nibble_mask: attrbs.nibble_mask =,
    if (!cxl_find_rec_dram(cxlmd, &attrbs) &&
    !cxl_find_rec_gen_media(cxlmd, &attrbs))
    pub -EINVAL: return,
    }
    }
    pub sizeof(maintenance_attrbs)): memset(&maintenance_attrbs, 0,,
    pub 0: maintenance_attrbs.flags =,
    pub cpu_to_le64(cxl_ppr_ctx->dpa): maintenance_attrbs.dpa =,
    put_unaligned_le24(cxl_ppr_ctx.nibble_mask,
    return cxl_perform_maintenance(&cxlmd.cxlds.cxl_mbox,
    cxl_ppr_ctx.op_class,
    cxl_ppr_ctx.op_subclass,
    &maintenance_attrbs,
    }
    static int cxl_ppr_get_repair_type(struct device *dev, void *drv_data,
    const char **repair_type)
    {
// repair_type = edac_repair_type[EDAC_REPAIR_PPR];
    pub 0: return,
    }
    static int cxl_ppr_get_persist_mode(struct device *dev, void *drv_data,
    bool *persist_mode)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
// persist_mode = cxl_ppr_ctx->persist_mode;
    pub 0: return,
    }
    static int cxl_get_ppr_safe_when_in_use(struct device *dev, void *drv_data,
    bool *safe)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
// safe = cxl_ppr_ctx->media_accessible & cxl_ppr_ctx->data_retained;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_ppr_get_min_dpa(dev: *mut device, drv_data: *mut c_void, min_dpa: *mut u64) -> c_int {
    static int cxl_ppr_get_min_dpa(struct device *dev, void *drv_data, u64 *min_dpa)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
// min_dpa = cxlds->dpa_res.start;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_ppr_get_max_dpa(dev: *mut device, drv_data: *mut c_void, max_dpa: *mut u64) -> c_int {
    static int cxl_ppr_get_max_dpa(struct device *dev, void *drv_data, u64 *max_dpa)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
// max_dpa = cxlds->dpa_res.end;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_ppr_get_dpa(dev: *mut device, drv_data: *mut c_void, dpa: *mut u64) -> c_int {
    static int cxl_ppr_get_dpa(struct device *dev, void *drv_data, u64 *dpa)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
// dpa = cxl_ppr_ctx->dpa;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_ppr_set_dpa(dev: *mut device, drv_data: *mut c_void, dpa: u64) -> c_int {
    static int cxl_ppr_set_dpa(struct device *dev, void *drv_data, u64 dpa)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
    if (!cxl_resource_contains_addr(&cxlds.dpa_res, dpa))
    pub -EINVAL: return,
    pub dpa: cxl_ppr_ctx->dpa =,
    pub 0: return,
    }
    static int cxl_ppr_get_nibble_mask(struct device *dev, void *drv_data,
    u32 *nibble_mask)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
// nibble_mask = cxl_ppr_ctx->nibble_mask;
    pub 0: return,
    }
    static int cxl_ppr_set_nibble_mask(struct device *dev, void *drv_data,
    u32 nibble_mask)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
    pub nibble_mask: cxl_ppr_ctx->nibble_mask =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cxl_do_ppr(dev: *mut device, drv_data: *mut c_void, val: u32) -> c_int {
    static int cxl_do_ppr(struct device *dev, void *drv_data, u32 val)
    {
    pub drv_data: *mut *mut cxl_ppr_context cxl_ppr_ctx =,
    pub cxl_ppr_ctx->cxlmd: *mut *mut cxl_memdev cxlmd =,
    pub cxlmd->cxlds: *mut *mut cxl_dev_state cxlds =,
    if (val != EDAC_DO_MEM_REPAIR ||
    !cxl_resource_contains_addr(&cxlds.dpa_res, cxl_ppr_ctx.dpa))
    pub -EINVAL: return,
    pub cxl_mem_perform_ppr(cxl_ppr_ctx): return,
    }
    static const struct edac_mem_repair_ops cxl_sppr_ops = {
    .get_repair_type = cxl_ppr_get_repair_type,
    .get_persist_mode = cxl_ppr_get_persist_mode,
    .get_repair_safe_when_in_use = cxl_get_ppr_safe_when_in_use,
    .get_min_dpa = cxl_ppr_get_min_dpa,
    .get_max_dpa = cxl_ppr_get_max_dpa,
    .get_dpa = cxl_ppr_get_dpa,
    .set_dpa = cxl_ppr_set_dpa,
    .get_nibble_mask = cxl_ppr_get_nibble_mask,
    .set_nibble_mask = cxl_ppr_set_nibble_mask,
    .do_repair = cxl_do_ppr,
}

    static int cxl_memdev_soft_ppr_init(struct cxl_memdev *cxlmd,
    struct edac_dev_feature *ras_feature,
    u8 repair_inst)
    {
    struct cxl_ppr_context *cxl_sppr_ctx;
    struct cxl_feat_entry *feat_entry;
    int ret;
    feat_entry = cxl_feature_info(to_cxlfs(cxlmd.cxlds),
    &CXL_FEAT_SPPR_UUID);
    if (IS_ERR(feat_entry))
    return -EOPNOTSUPP;
    if (!(le32_to_cpu(feat_entry.flags) & CXL_FEATURE_F_CHANGEABLE))
    return -EOPNOTSUPP;
    cxl_sppr_ctx =
    devm_kzalloc(&cxlmd.dev, sizeof(*cxl_sppr_ctx), GFP_KERNEL);
    if (!cxl_sppr_ctx)
    return -ENOMEM;
// cxl_sppr_ctx = (struct cxl_ppr_context){
    .get_feat_size = le16_to_cpu(feat_entry.get_feat_size),
    .set_feat_size = le16_to_cpu(feat_entry.set_feat_size),
    .get_version = feat_entry.get_feat_ver,
    .set_version = feat_entry.set_feat_ver,
    .effects = le16_to_cpu(feat_entry.effects),
    .cxlmd = cxlmd,
    .repair_type = EDAC_REPAIR_PPR,
    .persist_mode = 0,
    .instance = repair_inst,
    };
    uuid_copy(&cxl_sppr_ctx.repair_uuid, &CXL_FEAT_SPPR_UUID);
    ret = cxl_mem_ppr_get_attrbs(cxl_sppr_ctx);
    if (ret)
    return ret;
    ras_feature.ft_type = RAS_FEAT_MEM_REPAIR;
    ras_feature.instance = cxl_sppr_ctx.instance;
    ras_feature.mem_repair_ops = &cxl_sppr_ops;
    ras_feature.ctx = cxl_sppr_ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn err_rec_free(_cxlmd: *mut c_void) {
    static void err_rec_free(void *_cxlmd)
    {
    struct cxl_memdev *cxlmd = _cxlmd;
    struct cxl_mem_err_rec *array_rec = cxlmd.err_rec_array;
    struct cxl_event_gen_media *rec_gen_media;
    struct cxl_event_dram *rec_dram;
    unsigned long index;
    cxlmd.err_rec_array = core::ptr::null_mut();
    xa_for_each(&array_rec.rec_dram, index, rec_dram)
    kfree(rec_dram);
    xa_destroy(&array_rec.rec_dram);
    xa_for_each(&array_rec.rec_gen_media, index, rec_gen_media)
    kfree(rec_gen_media);
    xa_destroy(&array_rec.rec_gen_media);
    kfree(array_rec);
    }
#[no_mangle]
unsafe extern "C" fn devm_cxl_memdev_setup_err_rec(cxlmd: *mut cxl_memdev) -> c_int {
    static int devm_cxl_memdev_setup_err_rec(struct cxl_memdev *cxlmd)
    {
    struct cxl_mem_err_rec *array_rec = kzalloc_obj(*array_rec);
    if (!array_rec)
    return -ENOMEM;
    xa_init(&array_rec.rec_gen_media);
    xa_init(&array_rec.rec_dram);
    cxlmd.err_rec_array = array_rec;
    return devm_add_action_or_reset(&cxlmd.dev, err_rec_free, cxlmd);
    }
#[no_mangle]
pub unsafe extern "C" fn devm_cxl_memdev_edac_register(cxlmd: *mut cxl_memdev) -> c_int {
    int devm_cxl_memdev_edac_register(struct cxl_memdev *cxlmd)
    {
    struct edac_dev_feature ras_features[CXL_NR_EDAC_DEV_FEATURES];
    let mut num_ras_features: c_int = 0;
    let mut repair_inst: u8 = 0;
    int rc;
    if (IS_ENABLED(CONFIG_CXL_EDAC_SCRUB)) {
    rc = cxl_memdev_scrub_init(cxlmd, &ras_features[num_ras_features], 0);
    if (rc < 0 && rc != -EOPNOTSUPP)
    return rc;
    if (rc != -EOPNOTSUPP)
    num_ras_features++;
    }
    if (IS_ENABLED(CONFIG_CXL_EDAC_ECS)) {
    rc = cxl_memdev_ecs_init(cxlmd, &ras_features[num_ras_features]);
    if (rc < 0 && rc != -EOPNOTSUPP)
    return rc;
    if (rc != -EOPNOTSUPP)
    num_ras_features++;
    }
    if (IS_ENABLED(CONFIG_CXL_EDAC_MEM_REPAIR)) {
    for (int i = 0; i < CXL_MEM_SPARING_MAX; i++) {
    rc = cxl_memdev_sparing_init(cxlmd,
    &ras_features[num_ras_features],
    &mem_sparing_desc[i], repair_inst);
    if (rc == -EOPNOTSUPP)
    continue;
    if (rc < 0)
    return rc;
    repair_inst++;
    num_ras_features++;
    }
    rc = cxl_memdev_soft_ppr_init(cxlmd, &ras_features[num_ras_features],
    repair_inst);
    if (rc < 0 && rc != -EOPNOTSUPP)
    return rc;
    if (rc != -EOPNOTSUPP) {
    repair_inst++;
    num_ras_features++;
    }
    if (repair_inst) {
    rc = devm_cxl_memdev_setup_err_rec(cxlmd);
    if (rc)
    return rc;
    }
    }
    if (!num_ras_features)
    return -EINVAL;
    char *cxl_dev_name __free(kfree) =
    kasprintf(GFP_KERNEL, "cxl_%s", dev_name(&cxlmd.dev));
    if (!cxl_dev_name)
    return -ENOMEM;
    return edac_dev_register(&cxlmd.dev, cxl_dev_name, core::ptr::null_mut(),
    num_ras_features, ras_features);
    }
    EXPORT_SYMBOL_NS_GPL(devm_cxl_memdev_edac_register, "CXL");
#[no_mangle]
pub unsafe extern "C" fn devm_cxl_region_edac_register(cxlr: *mut cxl_region) -> c_int {
    int devm_cxl_region_edac_register(struct cxl_region *cxlr)
    {
    struct edac_dev_feature ras_features[CXL_NR_EDAC_DEV_FEATURES];
    let mut num_ras_features: c_int = 0;
    int rc;
    if (!IS_ENABLED(CONFIG_CXL_EDAC_SCRUB))
    return 0;
    rc = cxl_region_scrub_init(cxlr, &ras_features[num_ras_features], 0);
    if (rc < 0)
    return rc;
    num_ras_features++;
    char *cxl_dev_name __free(kfree) =
    kasprintf(GFP_KERNEL, "cxl_%s", dev_name(&cxlr.dev));
    if (!cxl_dev_name)
    return -ENOMEM;
    return edac_dev_register(&cxlr.dev, cxl_dev_name, core::ptr::null_mut(),
    num_ras_features, ras_features);
    }
    EXPORT_SYMBOL_NS_GPL(devm_cxl_region_edac_register, "CXL");
