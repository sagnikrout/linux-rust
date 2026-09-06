//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/smem_dramc.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const SMEM_DDR_INFO_ID: c_int = 603;
pub const MAX_DDR_FREQ_NUM_V3: c_int = 13;
pub const MAX_DDR_FREQ_NUM_V5: c_int = 14;
pub const MAX_CHAN_NUM: c_int = 8;
pub const MAX_RANK_NUM: c_int = 2;
pub const DDR_HBB_MIN: c_int = 13;
pub const DDR_HBB_MAX: c_int = 19;
pub const MAX_SHUB_ENTRIES: c_int = 8;
    static struct smem_dram *__dram;
    enum ddr_info_version {
    INFO_UNKNOWN,
    INFO_V3,
    INFO_V3_WITH_14_FREQS,
    INFO_V4,
    INFO_V5,
    INFO_V5_WITH_6_REGIONS,
    INFO_V6, /* INFO_V6 seems to only have shipped with 6 DDR regions, unlike V7 */
    INFO_V7,
    INFO_V7_WITH_6_REGIONS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smem_dram {
    pub frequencies: [c_ulong; MAX_DDR_FREQ_NUM_V5],
    pub num_frequencies: u32,
    pub hbb: u8,
}

    enum ddr_type {
    DDR_TYPE_NODDR = 0,
    DDR_TYPE_LPDDR1 = 1,
    DDR_TYPE_LPDDR2 = 2,
    DDR_TYPE_PCDDR2 = 3,
    DDR_TYPE_PCDDR3 = 4,
    DDR_TYPE_LPDDR3 = 5,
    DDR_TYPE_LPDDR4 = 6,
    DDR_TYPE_LPDDR4X = 7,
    DDR_TYPE_LPDDR5 = 8,
    DDR_TYPE_LPDDR5X = 9,
    };
// The data structures below are NOT __packed on purpose!
// Structs used across multiple versions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_part_details {
    pub revision_id1: __le16,
    pub revision_id2: __le16,
    pub width: __le16,
    pub density: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_freq_table {
    pub freq_khz: __le32,
    pub enabled: u8,
}

// V3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_freq_plan_v3 {
    pub ddr_freq: [ddr_freq_table; MAX_DDR_FREQ_NUM_V3],
    pub num_ddr_freqs: u8,
    pub clk_period_address: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_details_v3 {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub ddr_params: [ddr_part_details; MAX_CHAN_NUM],
    pub ddr_freq_tbl: ddr_freq_plan_v3,
    pub num_channels: u8,
}

// Some V3 structs have an additional frequency level
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_freq_plan_v3_14freqs {
    pub 1]: ddr_freq_table ddr_freq[MAX_DDR_FREQ_NUM_V3 +,
    pub num_ddr_freqs: u8,
    pub clk_period_address: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_details_v3_14freqs {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub ddr_params: [ddr_part_details; MAX_CHAN_NUM],
    pub ddr_freq_tbl: ddr_freq_plan_v3_14freqs,
    pub num_channels: u8,
}

// V4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_details_v4 {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub ddr_params: [ddr_part_details; MAX_CHAN_NUM],
    pub ddr_freq_tbl: ddr_freq_plan_v3,
    pub num_channels: u8,
    pub num_ranks: [u8; MAX_CHAN_NUM],
    pub highest_bank_addr_bit: [u8; MAX_CHAN_NUM][MAX_RANK_NUM],
}

// V5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shub_freq_table {
    pub enable: u8,
    pub freq_khz: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shub_freq_plan_entry {
    pub num_shub_freqs: u8,
    pub shub_freq: [shub_freq_table; MAX_SHUB_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_xbl2quantum_smem_data {
    pub ssr_cookie_addr: phys_addr_t,
    pub reserved: [__le32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_freq_plan_v5 {
    pub ddr_freq: [ddr_freq_table; MAX_DDR_FREQ_NUM_V5],
    pub num_ddr_freqs: u8,
    pub clk_period_address: phys_addr_t,
    pub max_nom_ddr_freq: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_region_v5 {
    pub start_address: __le64,
    pub size: __le64,
    pub mem_controller_address: __le64,
    pub /: *mut *mut __le32 granule_size; / MiB,
    pub ddr_rank: u8,

    pub segments_start_index: u8,
    pub segments_start_offset: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_regions_v5 {
    pub /: *mut *mut __le32 ddr_region_num; / We expect this to always be 4 or 6,
    pub ddr_rank0_size: __le64,
    pub ddr_rank1_size: __le64,
    pub ddr_cs0_start_addr: __le64,
    pub ddr_cs1_start_addr: __le64,
    pub highest_bank_addr_bit: __le32,
    pub __counted_by_le(ddr_region_num): ddr_region_v5 ddr_region[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_details_v5 {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub ddr_params: [ddr_part_details; MAX_CHAN_NUM],
    pub ddr_freq_tbl: ddr_freq_plan_v5,
    pub num_channels: u8,
    pub _padding: u8,
    pub ddr_regions: ddr_regions_v5,
}

// V6
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_misc_info_v6 {
    pub dsf_version: __le32,
    pub reserved: [__le32; 10],
}

// V7
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_details_v7 {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub ddr_params: [ddr_part_details; MAX_CHAN_NUM],
    pub ddr_freq_tbl: ddr_freq_plan_v5,
    pub num_channels: u8,
    pub sct_config: u8,
    pub ddr_regions: ddr_regions_v5,
}

//
// qcom_smem_dram_get_hbb(): Get the Highest bank address bit
//
// Context: Check qcom_smem_is_available() before calling this function.
// Because __dram * is initialized by smem_dram_parse(), which is in turn
// called from * qcom_smem_probe(), __dram will only be NULL if the data
// couldn't have been found/interpreted correctly.
//
// Return: highest bank bit on success, -ENODATA on failure.
//
#[no_mangle]
pub unsafe extern "C" fn qcom_smem_dram_get_hbb() -> c_int {
    int qcom_smem_dram_get_hbb(void)
    {
    if (!__dram || !__dram.hbb)
    return -ENODATA;
    if (__dram.hbb < DDR_HBB_MIN || __dram.hbb > DDR_HBB_MAX)
    return -ENODATA;
    return __dram.hbb;
    }
    EXPORT_SYMBOL_GPL(qcom_smem_dram_get_hbb);
#[no_mangle]
unsafe extern "C" fn smem_dram_parse_v3_data(dram: *mut smem_dram, data: *mut c_void) {
    static void smem_dram_parse_v3_data(struct smem_dram *dram, void *data)
    {
    struct ddr_details_v3 *details = data;
    for (int i = 0; i < MAX_DDR_FREQ_NUM_V3; i++) {
    struct ddr_freq_table *freq_entry = &details.ddr_freq_tbl.ddr_freq[i];
    if (freq_entry.freq_khz && freq_entry.enabled) {
    let mut freq_khz: u32 = le32_to_cpu(freq_entry.freq_khz);
    dram.frequencies[dram.num_frequencies++] = 1000 * freq_khz;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn smem_dram_parse_v3_14freqs_data(dram: *mut smem_dram, data: *mut c_void) {
    static void smem_dram_parse_v3_14freqs_data(struct smem_dram *dram, void *data)
    {
    struct ddr_details_v3_14freqs *details = data;
    for (int i = 0; i < MAX_DDR_FREQ_NUM_V3 + 1; i++) {
    struct ddr_freq_table *freq_entry = &details.ddr_freq_tbl.ddr_freq[i];
    if (freq_entry.freq_khz && freq_entry.enabled)
    dram.frequencies[dram.num_frequencies++] = 1000 * freq_entry.freq_khz;
    }
    }
#[no_mangle]
unsafe extern "C" fn smem_dram_parse_v4_data(dram: *mut smem_dram, data: *mut c_void) {
    static void smem_dram_parse_v4_data(struct smem_dram *dram, void *data)
    {
    struct ddr_details_v4 *details = data;
// Rank 0 channel 0 entry holds the correct value
    dram.hbb = details.highest_bank_addr_bit[0][0];
    for (int i = 0; i < MAX_DDR_FREQ_NUM_V3; i++) {
    struct ddr_freq_table *freq_entry = &details.ddr_freq_tbl.ddr_freq[i];
    if (freq_entry.freq_khz && freq_entry.enabled) {
    let mut freq_khz: u32 = le32_to_cpu(freq_entry.freq_khz);
    dram.frequencies[dram.num_frequencies++] = 1000 * freq_khz;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn smem_dram_parse_v5_data(dram: *mut smem_dram, data: *mut c_void) {
    static void smem_dram_parse_v5_data(struct smem_dram *dram, void *data)
    {
    struct ddr_details_v5 *details = data;
    struct ddr_regions_v5 *region = &details.ddr_regions;
    dram.hbb = le32_to_cpu(region[0].highest_bank_addr_bit);
    for (int i = 0; i < MAX_DDR_FREQ_NUM_V5; i++) {
    struct ddr_freq_table *freq_entry = &details.ddr_freq_tbl.ddr_freq[i];
    if (freq_entry.freq_khz && freq_entry.enabled) {
    let mut freq_khz: u32 = le32_to_cpu(freq_entry.freq_khz);
    dram.frequencies[dram.num_frequencies++] = 1000 * freq_khz;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn smem_dram_parse_v7_data(dram: *mut smem_dram, data: *mut c_void) {
    static void smem_dram_parse_v7_data(struct smem_dram *dram, void *data)
    {
    struct ddr_details_v7 *details = data;
    struct ddr_regions_v5 *region = &details.ddr_regions;
    dram.hbb = le32_to_cpu(region[0].highest_bank_addr_bit);
    for (int i = 0; i < MAX_DDR_FREQ_NUM_V5; i++) {
    struct ddr_freq_table *freq_entry = &details.ddr_freq_tbl.ddr_freq[i];
    if (freq_entry.freq_khz && freq_entry.enabled) {
    let mut freq_khz: u32 = le32_to_cpu(freq_entry.freq_khz);
    dram.frequencies[dram.num_frequencies++] = 1000 * freq_khz;
    }
    }
    }
// The structure contains no version field, so we have to perform some guesswork..
#[no_mangle]
unsafe extern "C" fn smem_dram_infer_struct_version(size: usize) -> c_int {
    static int smem_dram_infer_struct_version(size_t size)
    {
// Some early versions provided less bytes of less useful data
    if (size < sizeof(struct ddr_details_v3))
    return -EINVAL;
    if (size == sizeof(struct ddr_details_v3))
    return INFO_V3;
    if (size == sizeof(struct ddr_details_v3_14freqs))
    return INFO_V3_WITH_14_FREQS;
    if (size == sizeof(struct ddr_details_v4))
    return INFO_V4;
    if (size == sizeof(struct ddr_details_v5) +
#[no_mangle]
pub unsafe extern "C" fn sizeof(ddr_region_v5): struct) -> *mut 4 {
    4 * sizeof(struct ddr_region_v5))
    return INFO_V5;
    if (size == sizeof(struct ddr_details_v5) +
    4 * sizeof(struct ddr_region_v5) +
    sizeof(struct ddr_xbl2quantum_smem_data) +
    sizeof(struct shub_freq_plan_entry))
    return INFO_V5;
    if (size == sizeof(struct ddr_details_v5) +
#[no_mangle]
pub unsafe extern "C" fn sizeof(ddr_region_v5): struct) -> *mut 6 {
    6 * sizeof(struct ddr_region_v5))
    return INFO_V5_WITH_6_REGIONS;
    if (size == sizeof(struct ddr_details_v5) +
    6 * sizeof(struct ddr_region_v5) +
    sizeof(struct ddr_xbl2quantum_smem_data) +
    sizeof(struct shub_freq_plan_entry))
    return INFO_V5_WITH_6_REGIONS;
    if (size == sizeof(struct ddr_details_v5) +
    6 * sizeof(struct ddr_region_v5) +
    sizeof(struct ddr_misc_info_v6) +
    sizeof(struct shub_freq_plan_entry))
    return INFO_V6;
    if (size == sizeof(struct ddr_details_v7) +
    4 * sizeof(struct ddr_region_v5) +
    sizeof(struct ddr_misc_info_v6) +
    sizeof(struct shub_freq_plan_entry))
    return INFO_V7;
    if (size == sizeof(struct ddr_details_v7) +
    6 * sizeof(struct ddr_region_v5) +
    sizeof(struct ddr_misc_info_v6) +
    sizeof(struct shub_freq_plan_entry))
    return INFO_V7_WITH_6_REGIONS;
    return INFO_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn smem_dram_frequencies_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int smem_dram_frequencies_show(struct seq_file *s, void *unused)
    {
    struct smem_dram *dram = s.private;
    for (int i = 0; i < dram.num_frequencies; i++)
    seq_printf(s, "%lu\n", dram.frequencies[i]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(smem_dram_frequencies);
#[no_mangle]
unsafe extern "C" fn smem_hbb_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int smem_hbb_show(struct seq_file *s, void *unused)
    {
    struct smem_dram *dram = s.private;
    if (!dram.hbb)
    return -EINVAL;
    seq_printf(s, "%d\n", dram.hbb);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(smem_hbb);
    struct dentry *smem_dram_parse(struct qcom_smem *smem, struct device *dev)
    {
    struct dentry *debugfs_dir;
    enum ddr_info_version ver;
    struct smem_dram *dram;
    size_t actual_size;
    void *data;
// No need to check qcom_smem_is_available(), this func is called by the SMEM driver
    data = __qcom_smem_get(smem, QCOM_SMEM_HOST_ANY, SMEM_DDR_INFO_ID, &actual_size);
    if (IS_ERR_OR_NULL(data))
    return ERR_PTR(-ENODATA);
    ver = smem_dram_infer_struct_version(actual_size);
    if (ver < 0) {
// Some SoCs don't provide data that's useful for us
    return ERR_PTR(-ENODATA);
    } else if (ver == INFO_UNKNOWN) {
// In other cases, we may not have added support for a newer struct revision
    dev_err(dev, "Found an unknown type of DRAM info struct (size = %zu)\n",
    actual_size);
    return ERR_PTR(-EINVAL);
    }
    dram = devm_kzalloc(dev, sizeof(*dram), GFP_KERNEL);
    if (!dram)
    return ERR_PTR(-ENOMEM);
    switch (ver) {
    case INFO_V3:
    smem_dram_parse_v3_data(dram, data);
    break;
    case INFO_V3_WITH_14_FREQS:
    smem_dram_parse_v3_14freqs_data(dram, data);
    break;
    case INFO_V4:
    smem_dram_parse_v4_data(dram, data);
    break;
    case INFO_V5:
    case INFO_V5_WITH_6_REGIONS:
    case INFO_V6:
    smem_dram_parse_v5_data(dram, data);
    break;
    case INFO_V7:
    case INFO_V7_WITH_6_REGIONS:
    smem_dram_parse_v7_data(dram, data);
    break;
    default:
    return ERR_PTR(-EINVAL);
    }
    debugfs_dir = debugfs_create_dir("qcom_smem", core::ptr::null_mut());
    debugfs_create_file("dram_frequencies", 0444, debugfs_dir, dram,
    &smem_dram_frequencies_fops);
    debugfs_create_file("hbb", 0444, debugfs_dir, dram, &smem_hbb_fops);
    __dram = dram;
    return debugfs_dir;
    }
