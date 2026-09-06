//! Automatically rewritten from C to Rust
//! Source: drivers/edac/i7core_edac.c
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
// Intel i7 core/Nehalem Memory Controller kernel module
//
// This driver supports the memory controllers found on the Intel
// processor families i7core, i7core 7xx/8xx, i5core, Xeon 35xx,
// Xeon 55xx and Xeon 56xx also known as Nehalem, Nehalem-EP, Lynnfield
// and Westmere-EP.
//
// Copyright (c) 2009-2010 by:
// Mauro Carvalho Chehab
//
// Red Hat Inc. https://www.redhat.com
//
// Forked and adapted from the i5400_edac driver
//
// Based on the following public Intel datasheets:
// Intel Core i7 Processor Extreme Edition and Intel Core i7 Processor
// Datasheet, Volume 2:
// http://download.intel.com/design/processor/datashts/320835.pdf
// Intel Xeon Processor 5500 Series Datasheet Volume 2
// http://www.intel.com/Assets/PDF/datasheet/321322.pdf
// also available at:
// http://www.arrownac.com/manufacturers/intel/s/nehalem/5500-datasheet-v2.pdf
//

// Static vars
    static LIST_HEAD(i7core_edac_list);
    static DEFINE_MUTEX(i7core_edac_lock);
    static int probed;
    static int use_pci_fixup;
    module_param(use_pci_fixup, int, 0444);
    MODULE_PARM_DESC(use_pci_fixup, "Enable PCI fixup to seek for hidden devices");
//
// This is used for Nehalem-EP and Nehalem-EX devices, where the non-core
// registers start at bus 255, and are not reported by BIOS.
// We currently find devices with only 2 sockets. In order to support more QPI
// Quick Path Interconnect, just increment this number.
//
pub const MAX_SOCKET_BUSES: c_int = 2;
//
// Alter this version for the module when modifications are made
//

//
// Debug macros
//

    edac_printk(level, "i7core", fmt, ##arg)

    edac_mc_chipset_printk(mci, level, "i7core", fmt, ##arg)
//
// i7core Memory Controller Registers
//
// OFFSETS for Device 0 Function 0
pub const MC_CFG_CONTROL: c_uint = 0x90;
pub const MC_CFG_UNLOCK: c_uint = 0x02;
pub const MC_CFG_LOCK: c_uint = 0x00;
// OFFSETS for Device 3 Function 0
pub const MC_CONTROL: c_uint = 0x48;
pub const MC_STATUS: c_uint = 0x4c;
pub const MC_MAX_DOD: c_uint = 0x64;
//
// OFFSETS for Device 3 Function 4, as indicated on Xeon 5500 datasheet:
// http://www.arrownac.com/manufacturers/intel/s/nehalem/5500-datasheet-v2.pdf
//
pub const MC_TEST_ERR_RCV1: c_uint = 0x60;

pub const MC_TEST_ERR_RCV0: c_uint = 0x64;

// OFFSETS for Device 3 Function 2, as indicated on Xeon 5500 datasheet
pub const MC_SSRCONTROL: c_uint = 0x48;
pub const SSR_MODE_DISABLE: c_uint = 0x00;
pub const SSR_MODE_ENABLE: c_uint = 0x01;
pub const SSR_MODE_MASK: c_uint = 0x03;
pub const MC_SCRUB_CONTROL: c_uint = 0x4c;

pub const SCRUBINTERVAL_MASK: c_uint = 0xffffff;
pub const MC_COR_ECC_CNT_0: c_uint = 0x80;
pub const MC_COR_ECC_CNT_1: c_uint = 0x84;
pub const MC_COR_ECC_CNT_2: c_uint = 0x88;
pub const MC_COR_ECC_CNT_3: c_uint = 0x8c;
pub const MC_COR_ECC_CNT_4: c_uint = 0x90;
pub const MC_COR_ECC_CNT_5: c_uint = 0x94;

// OFFSETS for Devices 4,5 and 6 Function 0
pub const MC_CHANNEL_DIMM_INIT_PARAMS: c_uint = 0x58;

pub const MC_CHANNEL_MAPPER: c_uint = 0x60;

pub const MC_CHANNEL_RANK_PRESENT: c_uint = 0x7c;
pub const RANK_PRESENT_MASK: c_uint = 0xffff;
pub const MC_CHANNEL_ADDR_MATCH: c_uint = 0xf0;
pub const MC_CHANNEL_ERROR_MASK: c_uint = 0xf8;
pub const MC_CHANNEL_ERROR_INJECT: c_uint = 0xfc;
pub const INJECT_ADDR_PARITY: c_uint = 0x10;
pub const INJECT_ECC: c_uint = 0x08;
pub const MASK_CACHELINE: c_uint = 0x06;
pub const MASK_FULL_CACHELINE: c_uint = 0x06;
pub const MASK_MSB32_CACHELINE: c_uint = 0x04;
pub const MASK_LSB32_CACHELINE: c_uint = 0x02;
pub const NO_MASK_CACHELINE: c_uint = 0x00;
pub const REPEAT_EN: c_uint = 0x01;
// OFFSETS for Devices 4,5 and 6 Function 1
pub const MC_DOD_CH_DIMM0: c_uint = 0x48;
pub const MC_DOD_CH_DIMM1: c_uint = 0x4c;
pub const MC_DOD_CH_DIMM2: c_uint = 0x50;

pub const MC_DOD_NUMCOL_MASK: c_int = 3;

pub const MC_RANK_PRESENT: c_uint = 0x7c;
pub const MC_SAG_CH_0: c_uint = 0x80;
pub const MC_SAG_CH_1: c_uint = 0x84;
pub const MC_SAG_CH_2: c_uint = 0x88;
pub const MC_SAG_CH_3: c_uint = 0x8c;
pub const MC_SAG_CH_4: c_uint = 0x90;
pub const MC_SAG_CH_5: c_uint = 0x94;
pub const MC_SAG_CH_6: c_uint = 0x98;
pub const MC_SAG_CH_7: c_uint = 0x9c;
pub const MC_RIR_LIMIT_CH_0: c_uint = 0x40;
pub const MC_RIR_LIMIT_CH_1: c_uint = 0x44;
pub const MC_RIR_LIMIT_CH_2: c_uint = 0x48;
pub const MC_RIR_LIMIT_CH_3: c_uint = 0x4C;
pub const MC_RIR_LIMIT_CH_4: c_uint = 0x50;
pub const MC_RIR_LIMIT_CH_5: c_uint = 0x54;
pub const MC_RIR_LIMIT_CH_6: c_uint = 0x58;
pub const MC_RIR_LIMIT_CH_7: c_uint = 0x5C;

pub const MC_RIR_WAY_CH: c_uint = 0x80;

pub const MC_RIR_WAY_RANK_MASK: c_uint = 0x7;
//
// i7core structs
//
pub const NUM_CHANS: c_int = 3;

pub const MAX_MCR_FUNC: c_int = 4;
pub const MAX_CHAN_FUNC: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i7core_info {
    pub mc_control: u32,
    pub mc_status: u32,
    pub max_dod: u32,
    pub ch_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i7core_inject {
    pub enable: c_int,
    pub section: u32,
    pub type: u32,
    pub eccmask: u32,
// Error address mask
    pub col: int channel, dimm, rank, bank, page,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i7core_channel {
    pub is_3dimms_present: bool,
    pub is_single_4rank: bool,
    pub has_4rank: bool,
    pub dimms: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_id_descr {
    pub dev: c_int,
    pub func: c_int,
    pub dev_id: c_int,
    pub optional: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_id_table {
    pub descr: *const pci_id_descr,
    pub n_devs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i7core_dev {
    pub list: list_head,
    pub socket: u8,
    pub mci: *mut mem_ctl_info,
    pub n_devs: c_int,
    pub __counted_by(n_devs): *mut *mut pci_dev pdev[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i7core_pvt {
    pub chancounts_dev: *mut *mut device addrmatch_dev,,
    pub pci_noncore: *mut pci_dev,
    pub 1]: *mut *mut pci_dev pci_mcr[MAX_MCR_FUNC +,
    pub 1]: *mut *mut pci_dev pci_ch[NUM_CHANS][MAX_CHAN_FUNC +,
    pub i7core_dev: *mut i7core_dev,
    pub info: i7core_info,
    pub inject: i7core_inject,
    pub channel: [i7core_channel; NUM_CHANS],
    pub ce_count_available: c_int,
// ECC corrected errors counts per udimm
    pub udimm_ce_count: [c_ulong; MAX_DIMMS],
    pub udimm_last_ce_count: [c_int; MAX_DIMMS],
// ECC corrected errors counts per rdimm
    pub rdimm_ce_count: [c_ulong; NUM_CHANS][MAX_DIMMS],
    pub rdimm_last_ce_count: [c_int; NUM_CHANS][MAX_DIMMS],
    pub enable_scrub: bool is_registered,,
// DCLK Frequency used for computing scrub rate
    pub dclk_freq: c_int,
// Struct to control EDAC polling
    pub i7core_pci: *mut edac_pci_ctl_info,
}

    .dev = (device),			\
    .func = (function),			\
    .dev_id = (device_id)
    static const struct pci_id_descr pci_dev_descr_i7core_nehalem[] = {
// Memory controller
    { PCI_DESCR(3, 0, PCI_DEVICE_ID_INTEL_I7_MCR)     },
    { PCI_DESCR(3, 1, PCI_DEVICE_ID_INTEL_I7_MC_TAD)  },
// Exists only for RDIMM
    { PCI_DESCR(3, 2, PCI_DEVICE_ID_INTEL_I7_MC_RAS), .optional = 1  },
    { PCI_DESCR(3, 4, PCI_DEVICE_ID_INTEL_I7_MC_TEST) },
// Channel 0
    { PCI_DESCR(4, 0, PCI_DEVICE_ID_INTEL_I7_MC_CH0_CTRL) },
    { PCI_DESCR(4, 1, PCI_DEVICE_ID_INTEL_I7_MC_CH0_ADDR) },
    { PCI_DESCR(4, 2, PCI_DEVICE_ID_INTEL_I7_MC_CH0_RANK) },
    { PCI_DESCR(4, 3, PCI_DEVICE_ID_INTEL_I7_MC_CH0_TC)   },
// Channel 1
    { PCI_DESCR(5, 0, PCI_DEVICE_ID_INTEL_I7_MC_CH1_CTRL) },
    { PCI_DESCR(5, 1, PCI_DEVICE_ID_INTEL_I7_MC_CH1_ADDR) },
    { PCI_DESCR(5, 2, PCI_DEVICE_ID_INTEL_I7_MC_CH1_RANK) },
    { PCI_DESCR(5, 3, PCI_DEVICE_ID_INTEL_I7_MC_CH1_TC)   },
// Channel 2
    { PCI_DESCR(6, 0, PCI_DEVICE_ID_INTEL_I7_MC_CH2_CTRL) },
    { PCI_DESCR(6, 1, PCI_DEVICE_ID_INTEL_I7_MC_CH2_ADDR) },
    { PCI_DESCR(6, 2, PCI_DEVICE_ID_INTEL_I7_MC_CH2_RANK) },
    { PCI_DESCR(6, 3, PCI_DEVICE_ID_INTEL_I7_MC_CH2_TC)   },
// Generic Non-core registers
//
// This is the PCI device on i7core and on Xeon 35xx (8086:2c41)
// On Xeon 55xx, however, it has a different id (8086:2c40). So,
// the probing code needs to test for the other address in case of
// failure of this one
//
    { PCI_DESCR(0, 0, PCI_DEVICE_ID_INTEL_I7_NONCORE)  },
    };
    static const struct pci_id_descr pci_dev_descr_lynnfield[] = {
    { PCI_DESCR( 3, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MCR)         },
    { PCI_DESCR( 3, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_TAD)      },
    { PCI_DESCR( 3, 4, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_TEST)     },
    { PCI_DESCR( 4, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_CTRL) },
    { PCI_DESCR( 4, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_ADDR) },
    { PCI_DESCR( 4, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_RANK) },
    { PCI_DESCR( 4, 3, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_TC)   },
    { PCI_DESCR( 5, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_CTRL) },
    { PCI_DESCR( 5, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_ADDR) },
    { PCI_DESCR( 5, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_RANK) },
    { PCI_DESCR( 5, 3, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_TC)   },
//
// This is the PCI device has an alternate address on some
// processors like Core i7 860
//
    { PCI_DESCR( 0, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE)     },
    };
    static const struct pci_id_descr pci_dev_descr_i7core_westmere[] = {
// Memory controller
    { PCI_DESCR(3, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MCR_REV2)     },
    { PCI_DESCR(3, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_TAD_REV2)  },
// Exists only for RDIMM
    { PCI_DESCR(3, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_RAS_REV2), .optional = 1  },
    { PCI_DESCR(3, 4, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_TEST_REV2) },
// Channel 0
    { PCI_DESCR(4, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_CTRL_REV2) },
    { PCI_DESCR(4, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_ADDR_REV2) },
    { PCI_DESCR(4, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_RANK_REV2) },
    { PCI_DESCR(4, 3, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH0_TC_REV2)   },
// Channel 1
    { PCI_DESCR(5, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_CTRL_REV2) },
    { PCI_DESCR(5, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_ADDR_REV2) },
    { PCI_DESCR(5, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_RANK_REV2) },
    { PCI_DESCR(5, 3, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH1_TC_REV2)   },
// Channel 2
    { PCI_DESCR(6, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH2_CTRL_REV2) },
    { PCI_DESCR(6, 1, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH2_ADDR_REV2) },
    { PCI_DESCR(6, 2, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH2_RANK_REV2) },
    { PCI_DESCR(6, 3, PCI_DEVICE_ID_INTEL_LYNNFIELD_MC_CH2_TC_REV2)   },
// Generic Non-core registers
    { PCI_DESCR(0, 0, PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE_REV2)  },
    };

    static const struct pci_id_table pci_dev_table[] = {
    PCI_ID_TABLE_ENTRY(pci_dev_descr_i7core_nehalem),
    PCI_ID_TABLE_ENTRY(pci_dev_descr_lynnfield),
    PCI_ID_TABLE_ENTRY(pci_dev_descr_i7core_westmere),
    { core::ptr::null_mut(), }
    };
//
// pci_device_id	table for which devices we are looking for
//
    static const struct pci_device_id i7core_pci_tbl[] = {
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_X58_HUB_MGMT)},
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_LYNNFIELD_QPI_LINK0)},
    { 0, }
    };
//
    Ancillary status routines
//
// MC_CONTROL bits

// MC_STATUS bits

// MC_MAX_DOD read functions
#[no_mangle]
pub unsafe extern "C" fn numdimms(dimms: u32) -> c_int {
    static inline int numdimms(u32 dimms)
    {
    return (dimms & 0x3) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn numrank(rank: u32) -> c_int {
    static inline int numrank(u32 rank)
    {
    static const int ranks[] = { 1, 2, 4, -EINVAL };
    return ranks[rank & 0x3];
    }
#[no_mangle]
pub unsafe extern "C" fn numbank(bank: u32) -> c_int {
    static inline int numbank(u32 bank)
    {
    static const int banks[] = { 4, 8, 16, -EINVAL };
    return banks[bank & 0x3];
    }
#[no_mangle]
pub unsafe extern "C" fn numrow(row: u32) -> c_int {
    static inline int numrow(u32 row)
    {
    static const int rows[] = {
    1 << 12, 1 << 13, 1 << 14, 1 << 15,
    1 << 16, -EINVAL, -EINVAL, -EINVAL,
    };
    return rows[row & 0x7];
    }
#[no_mangle]
pub unsafe extern "C" fn numcol(col: u32) -> c_int {
    static inline int numcol(u32 col)
    {
    static const int cols[] = {
    1 << 10, 1 << 11, 1 << 12, -EINVAL,
    };
    return cols[col & 0x3];
    }
    static struct i7core_dev *get_i7core_dev(u8 socket)
    {
    struct i7core_dev *i7core_dev;
    list_for_each_entry(i7core_dev, &i7core_edac_list, list) {
    if (i7core_dev.socket == socket)
    return i7core_dev;
    }
    return core::ptr::null_mut();
    }
    static struct i7core_dev *alloc_i7core_dev(u8 socket,
    const struct pci_id_table *table)
    {
    struct i7core_dev *i7core_dev;
    i7core_dev = kzalloc_flex(*i7core_dev, pdev, table.n_devs);
    if (!i7core_dev)
    return core::ptr::null_mut();
    i7core_dev.n_devs = table.n_devs;
    i7core_dev.socket = socket;
    list_add_tail(&i7core_dev.list, &i7core_edac_list);
    return i7core_dev;
    }
#[no_mangle]
unsafe extern "C" fn free_i7core_dev(i7core_dev: *mut i7core_dev) {
    static void free_i7core_dev(struct i7core_dev *i7core_dev)
    {
    list_del(&i7core_dev.list);
    kfree(i7core_dev);
    }
//
    Memory check routines
//
#[no_mangle]
unsafe extern "C" fn get_dimm_config(mci: *mut mem_ctl_info) -> c_int {
    static int get_dimm_config(struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    struct pci_dev *pdev;
    int i, j;
    enum edac_type mode;
    enum mem_type mtype;
    struct dimm_info *dimm;
// Get data from the MC register, function 0
    pdev = pvt.pci_mcr[0];
    if (!pdev)
    return -ENODEV;
// Device 3 function 0 reads
    pci_read_config_dword(pdev, MC_CONTROL, &pvt.info.mc_control);
    pci_read_config_dword(pdev, MC_STATUS, &pvt.info.mc_status);
    pci_read_config_dword(pdev, MC_MAX_DOD, &pvt.info.max_dod);
    pci_read_config_dword(pdev, MC_CHANNEL_MAPPER, &pvt.info.ch_map);
    edac_dbg(0, "QPI %d control=0x%08x status=0x%08x dod=0x%08x map=0x%08x\n",
    pvt.i7core_dev.socket, pvt.info.mc_control,
    pvt.info.mc_status, pvt.info.max_dod, pvt.info.ch_map);
    if (ECC_ENABLED(pvt)) {
    edac_dbg(0, "ECC enabled with x%d SDCC\n", ECCx8(pvt) ? 8 : 4);
    if (ECCx8(pvt))
    mode = EDAC_S8ECD8ED;
    else
    mode = EDAC_S4ECD4ED;
    } else {
    edac_dbg(0, "ECC disabled\n");
    mode = EDAC_NONE;
    }
// FIXME: need to handle the error codes
    edac_dbg(0, "DOD Max limits: DIMMS: %d, %d-ranked, %d-banked x%x x 0x%x\n",
    numdimms(pvt.info.max_dod),
    numrank(pvt.info.max_dod >> 2),
    numbank(pvt.info.max_dod >> 4),
    numrow(pvt.info.max_dod >> 6),
    numcol(pvt.info.max_dod >> 9));
    for (i = 0; i < NUM_CHANS; i++) {
    u32 data, dimm_dod[3], value[8];
    if (!pvt.pci_ch[i][0])
    continue;
    if (!CH_ACTIVE(pvt, i)) {
    edac_dbg(0, "Channel %i is not active\n", i);
    continue;
    }
    if (CH_DISABLED(pvt, i)) {
    edac_dbg(0, "Channel %i is disabled\n", i);
    continue;
    }
// Devices 4-6 function 0
    pci_read_config_dword(pvt.pci_ch[i][0],
    MC_CHANNEL_DIMM_INIT_PARAMS, &data);
    if (data & THREE_DIMMS_PRESENT)
    pvt.channel[i].is_3dimms_present = true;
    if (data & SINGLE_QUAD_RANK_PRESENT)
    pvt.channel[i].is_single_4rank = true;
    if (data & QUAD_RANK_PRESENT)
    pvt.channel[i].has_4rank = true;
    if (data & REGISTERED_DIMM)
    mtype = MEM_RDDR3;
    else
    mtype = MEM_DDR3;
// Devices 4-6 function 1
    pci_read_config_dword(pvt.pci_ch[i][1],
    MC_DOD_CH_DIMM0, &dimm_dod[0]);
    pci_read_config_dword(pvt.pci_ch[i][1],
    MC_DOD_CH_DIMM1, &dimm_dod[1]);
    pci_read_config_dword(pvt.pci_ch[i][1],
    MC_DOD_CH_DIMM2, &dimm_dod[2]);
    edac_dbg(0, "Ch%d phy rd%d, wr%d (0x%08x): %s%s%s%cDIMMs\n",
    i,
    RDLCH(pvt.info.ch_map, i), WRLCH(pvt.info.ch_map, i),
    data,
    pvt.channel[i].is_3dimms_present ? "3DIMMS " : "",
    pvt.channel[i].is_3dimms_present ? "SINGLE_4R " : "",
    pvt.channel[i].has_4rank ? "HAS_4R " : "",
    (data & REGISTERED_DIMM) ? 'R' : 'U');
    for (j = 0; j < 3; j++) {
    u32 banks, ranks, rows, cols;
    u32 size, npages;
    if (!DIMM_PRESENT(dimm_dod[j]))
    continue;
    dimm = edac_get_dimm(mci, i, j, 0);
    banks = numbank(MC_DOD_NUMBANK(dimm_dod[j]));
    ranks = numrank(MC_DOD_NUMRANK(dimm_dod[j]));
    rows = numrow(MC_DOD_NUMROW(dimm_dod[j]));
    cols = numcol(MC_DOD_NUMCOL(dimm_dod[j]));
// DDR3 has 8 I/O banks
    size = (rows * cols * banks * ranks) >> (20 - 3);
    edac_dbg(0, "\tdimm %d %d MiB offset: %x, bank: %d, rank: %d, row: %#x, col: %#x\n",
    j, size,
    RANKOFFSET(dimm_dod[j]),
    banks, ranks, rows, cols);
    npages = MiB_TO_PAGES(size);
    dimm.nr_pages = npages;
    switch (banks) {
    case 4:
    dimm.dtype = DEV_X4;
    break;
    case 8:
    dimm.dtype = DEV_X8;
    break;
    case 16:
    dimm.dtype = DEV_X16;
    break;
    default:
    dimm.dtype = DEV_UNKNOWN;
    }
    snprintf(dimm.label, sizeof(dimm.label),
    "CPU#%uChannel#%u_DIMM#%u",
    pvt.i7core_dev.socket, i, j);
    dimm.grain = 8;
    dimm.edac_mode = mode;
    dimm.mtype = mtype;
    }
    pci_read_config_dword(pdev, MC_SAG_CH_0, &value[0]);
    pci_read_config_dword(pdev, MC_SAG_CH_1, &value[1]);
    pci_read_config_dword(pdev, MC_SAG_CH_2, &value[2]);
    pci_read_config_dword(pdev, MC_SAG_CH_3, &value[3]);
    pci_read_config_dword(pdev, MC_SAG_CH_4, &value[4]);
    pci_read_config_dword(pdev, MC_SAG_CH_5, &value[5]);
    pci_read_config_dword(pdev, MC_SAG_CH_6, &value[6]);
    pci_read_config_dword(pdev, MC_SAG_CH_7, &value[7]);
    edac_dbg(1, "\t[%i] DIVBY3\tREMOVED\tOFFSET\n", i);
    for (j = 0; j < 8; j++)
    edac_dbg(1, "\t\t%#x\t%#x\t%#x\n",
    (value[j] >> 27) & 0x1,
    (value[j] >> 24) & 0x7,
    (value[j] & ((1 << 24) - 1)));
    }
    return 0;
    }
//
    Error insertion routines
//

// The i7core has independent error injection features per channel.
    However, to have a simpler code, we don't allow enabling error injection
    on more than one channel.
    Also, since a change at an inject parameter will be applied only at enable,
    we're disabling error injection on all write calls to the sysfs nodes that
    controls the error code injection.
//
#[no_mangle]
unsafe extern "C" fn disable_inject(mci: *const mem_ctl_info) -> c_int {
    static int disable_inject(const struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    pvt.inject.enable = 0;
    if (!pvt.pci_ch[pvt.inject.channel][0])
    return -ENODEV;
    pci_write_config_dword(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ERROR_INJECT, 0);
    return 0;
    }
//
// i7core inject inject.section
//
// accept and store error injection inject.section value
// bit 0 - refers to the lower 32-byte half cacheline
// bit 1 - refers to the upper 32-byte half cacheline
//
    static ssize_t i7core_inject_section_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    unsigned long value;
    int rc;
    if (pvt.inject.enable)
    disable_inject(mci);
    rc = kstrtoul(data, 10, &value);
    if ((rc < 0) || (value > 3))
    return -EIO;
    pvt.inject.section = (u32) value;
    return count;
    }
    static ssize_t i7core_inject_section_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    return sprintf(data, "0x%08x\n", pvt.inject.section);
    }
//
// i7core inject.type
//
// accept and store error injection inject.section value
// bit 0 - repeat enable - Enable error repetition
// bit 1 - inject ECC error
// bit 2 - inject parity error
//
    static ssize_t i7core_inject_type_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    unsigned long value;
    int rc;
    if (pvt.inject.enable)
    disable_inject(mci);
    rc = kstrtoul(data, 10, &value);
    if ((rc < 0) || (value > 7))
    return -EIO;
    pvt.inject.type = (u32) value;
    return count;
    }
    static ssize_t i7core_inject_type_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    return sprintf(data, "0x%08x\n", pvt.inject.type);
    }
//
// i7core_inject_inject.eccmask_store
//
// The type of error (UE/CE) will depend on the inject.eccmask value:
// Any bits set to a 1 will flip the corresponding ECC bit
// Correctable errors can be injected by flipping 1 bit or the bits within
// a symbol pair (2 consecutive aligned 8-bit pairs - i.e. 7:0 and 15:8 or
// 23:16 and 31:24). Flipping bits in two symbol pairs will cause an
// uncorrectable error to be injected.
//
    static ssize_t i7core_inject_eccmask_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    unsigned long value;
    int rc;
    if (pvt.inject.enable)
    disable_inject(mci);
    rc = kstrtoul(data, 10, &value);
    if (rc < 0)
    return -EIO;
    pvt.inject.eccmask = (u32) value;
    return count;
    }
    static ssize_t i7core_inject_eccmask_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    return sprintf(data, "0x%08x\n", pvt.inject.eccmask);
    }
//
// i7core_addrmatch
//
// The type of error (UE/CE) will depend on the inject.eccmask value:
// Any bits set to a 1 will flip the corresponding ECC bit
// Correctable errors can be injected by flipping 1 bit or the bits within
// a symbol pair (2 consecutive aligned 8-bit pairs - i.e. 7:0 and 15:8 or
// 23:16 and 31:24). Flipping bits in two symbol pairs will cause an
// uncorrectable error to be injected.
//

    static ssize_t i7core_inject_store_##param(			\
    struct device *dev,					\
    struct device_attribute *mattr,				\
    const char *data, size_t count)				\
    {								\
    struct mem_ctl_info *mci = dev_get_drvdata(dev);	\
    struct i7core_pvt *pvt;					\
    long value;						\
    int rc;							\
    \
    edac_dbg(1, "\n");					\
    pvt = mci.pvt_info;					\
    \
    if (pvt.inject.enable)					\
    disable_inject(mci);				\
    \
    if (!strcasecmp(data, "any") || !strcasecmp(data, "any\n"))\
    value = -1;					\
    else {							\
    rc = kstrtoul(data, 10, &value);		\
    if ((rc < 0) || (value >= limit))		\
    return -EIO;				\
    }							\
    \
    pvt.inject.param = value;				\
    \
    return count;						\
    }								\
    \
    static ssize_t i7core_inject_show_##param(			\
    struct device *dev,					\
    struct device_attribute *mattr,				\
    char *data)						\
    {								\
    struct mem_ctl_info *mci = dev_get_drvdata(dev);	\
    struct i7core_pvt *pvt;					\
    \
    pvt = mci.pvt_info;					\
    edac_dbg(1, "pvt=%p\n", pvt);				\
    if (pvt.inject.param < 0)				\
    return sprintf(data, "any\n");			\
    else							\
    return sprintf(data, "%d\n", pvt.inject.param);\
    }

    static DEVICE_ATTR(param, S_IRUGO | S_IWUSR,		\
    i7core_inject_show_##param,			\
    i7core_inject_store_##param)
    DECLARE_ADDR_MATCH(channel, 3);
    DECLARE_ADDR_MATCH(dimm, 3);
    DECLARE_ADDR_MATCH(rank, 4);
    DECLARE_ADDR_MATCH(bank, 32);
    DECLARE_ADDR_MATCH(page, 0x10000);
    DECLARE_ADDR_MATCH(col, 0x4000);
    ATTR_ADDR_MATCH(channel);
    ATTR_ADDR_MATCH(dimm);
    ATTR_ADDR_MATCH(rank);
    ATTR_ADDR_MATCH(bank);
    ATTR_ADDR_MATCH(page);
    ATTR_ADDR_MATCH(col);
#[no_mangle]
unsafe extern "C" fn write_and_test(dev: *mut pci_dev, where: c_int, val: u32) -> c_int {
    static int write_and_test(struct pci_dev *dev, const int where, const u32 val)
    {
    u32 read;
    int count;
    edac_dbg(0, "setting pci %02x:%02x.%x reg=%02x value=%08x\n",
    dev.bus.number, PCI_SLOT(dev.devfn), PCI_FUNC(dev.devfn),
    where, val);
    for (count = 0; count < 10; count++) {
    if (count)
    msleep(100);
    pci_write_config_dword(dev, where, val);
    pci_read_config_dword(dev, where, &read);
    if (read == val)
    return 0;
    }
    i7core_printk(KERN_ERR, "Error during set pci %02x:%02x.%x reg=%02x "
    "write=%08x. Read=%08x\n",
    dev.bus.number, PCI_SLOT(dev.devfn), PCI_FUNC(dev.devfn),
    where, val, read);
    return -EINVAL;
    }
//
// This routine prepares the Memory Controller for error injection.
// The error will be injected when some process tries to write to the
// memory that matches the given criteria.
// The criteria can be set in terms of a mask where dimm, rank, bank, page
// and col can be specified.
// A -1 value for any of the mask items will make the MCU to ignore
// that matching criteria for error injection.
//
// It should be noticed that the error will only happen after a write operation
// on a memory that matches the condition. if REPEAT_EN is not enabled at
// inject mask, then it will produce just one error. Otherwise, it will repeat
// until the injectmask would be cleaned.
//
// FIXME: This routine assumes that MAXNUMDIMMS value of MC_MAX_DOD
// is reliable enough to check if the MC is using the
// three channels. However, this is not clear at the datasheet.
//
    static ssize_t i7core_inject_enable_store(struct device *dev,
    struct device_attribute *mattr,
    const char *data, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    u32 injectmask;
    let mut mask: u64 = 0;
    int  rc;
    long enable;
    if (!pvt.pci_ch[pvt.inject.channel][0])
    return 0;
    rc = kstrtoul(data, 10, &enable);
    if ((rc < 0))
    return 0;
    if (enable) {
    pvt.inject.enable = 1;
    } else {
    disable_inject(mci);
    return count;
    }
// Sets pvt->inject.dimm mask
    if (pvt.inject.dimm < 0)
    mask |= 1LL << 41;
    else {
    if (pvt.channel[pvt.inject.channel].dimms > 2)
    mask |= (pvt.inject.dimm & 0x3LL) << 35;
    else
    mask |= (pvt.inject.dimm & 0x1LL) << 36;
    }
// Sets pvt->inject.rank mask
    if (pvt.inject.rank < 0)
    mask |= 1LL << 40;
    else {
    if (pvt.channel[pvt.inject.channel].dimms > 2)
    mask |= (pvt.inject.rank & 0x1LL) << 34;
    else
    mask |= (pvt.inject.rank & 0x3LL) << 34;
    }
// Sets pvt->inject.bank mask
    if (pvt.inject.bank < 0)
    mask |= 1LL << 39;
    else
    mask |= (pvt.inject.bank & 0x15LL) << 30;
// Sets pvt->inject.page mask
    if (pvt.inject.page < 0)
    mask |= 1LL << 38;
    else
    mask |= (pvt.inject.page & 0xffff) << 14;
// Sets pvt->inject.column mask
    if (pvt.inject.col < 0)
    mask |= 1LL << 37;
    else
    mask |= (pvt.inject.col & 0x3fff);
//
// bit    0: REPEAT_EN
// bits 1-2: MASK_HALF_CACHELINE
// bit    3: INJECT_ECC
// bit    4: INJECT_ADDR_PARITY
//
    injectmask = (pvt.inject.type & 1) |
    (pvt.inject.section & 0x3) << 1 |
    (pvt.inject.type & 0x6) << (3 - 1);
// Unlock writes to registers - this register is write only
    pci_write_config_dword(pvt.pci_noncore,
    MC_CFG_CONTROL, 0x2);
    write_and_test(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ADDR_MATCH, mask);
    write_and_test(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ADDR_MATCH + 4, mask >> 32L);
    write_and_test(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ERROR_MASK, pvt.inject.eccmask);
    write_and_test(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ERROR_INJECT, injectmask);
//
// This is something undocumented, based on my tests
// Without writing 8 to this register, errors aren't injected. Not sure
// why.
//
    pci_write_config_dword(pvt.pci_noncore,
    MC_CFG_CONTROL, 8);
    edac_dbg(0, "Error inject addr match 0x%016llx, ecc 0x%08x, inject 0x%08x\n",
    mask, pvt.inject.eccmask, injectmask);
    return count;
    }
    static ssize_t i7core_inject_enable_show(struct device *dev,
    struct device_attribute *mattr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct i7core_pvt *pvt = mci.pvt_info;
    u32 injectmask;
    if (!pvt.pci_ch[pvt.inject.channel][0])
    return 0;
    pci_read_config_dword(pvt.pci_ch[pvt.inject.channel][0],
    MC_CHANNEL_ERROR_INJECT, &injectmask);
    edac_dbg(0, "Inject error read: 0x%018x\n", injectmask);
    if (injectmask & 0x0c)
    pvt.inject.enable = 1;
    return sprintf(data, "%d\n", pvt.inject.enable);
    }

    static ssize_t i7core_show_counter_##param(			\
    struct device *dev,					\
    struct device_attribute *mattr,				\
    char *data)						\
    {								\
    struct mem_ctl_info *mci = dev_get_drvdata(dev);	\
    struct i7core_pvt *pvt = mci.pvt_info;			\
    \
    edac_dbg(1, "\n");					\
    if (!pvt.ce_count_available || (pvt.is_registered))	\
    return sprintf(data, "data unavailable\n");	\
    return sprintf(data, "%lu\n",				\
    pvt.udimm_ce_count[param]);		\
    }

    static DEVICE_ATTR(udimm##param, S_IRUGO | S_IWUSR,	\
    i7core_show_counter_##param,		\
    core::ptr::null_mut())
    DECLARE_COUNTER(0);
    DECLARE_COUNTER(1);
    DECLARE_COUNTER(2);
    ATTR_COUNTER(0);
    ATTR_COUNTER(1);
    ATTR_COUNTER(2);
//
// inject_addrmatch device sysfs struct
//
    static struct attribute *i7core_addrmatch_attrs[] = {
    &dev_attr_channel.attr,
    &dev_attr_dimm.attr,
    &dev_attr_rank.attr,
    &dev_attr_bank.attr,
    &dev_attr_page.attr,
    &dev_attr_col.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group addrmatch_grp = {
    .attrs	= i7core_addrmatch_attrs,
    };
    static const struct attribute_group *addrmatch_groups[] = {
    &addrmatch_grp,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn addrmatch_release(device: *mut device) {
    static void addrmatch_release(struct device *device)
    {
    edac_dbg(1, "Releasing device %s\n", dev_name(device));
    kfree(device);
    }
    static const struct device_type addrmatch_type = {
    .groups		= addrmatch_groups,
    .release	= addrmatch_release,
    };
//
// all_channel_counts sysfs struct
//
    static struct attribute *i7core_udimm_counters_attrs[] = {
    &dev_attr_udimm0.attr,
    &dev_attr_udimm1.attr,
    &dev_attr_udimm2.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group all_channel_counts_grp = {
    .attrs	= i7core_udimm_counters_attrs,
    };
    static const struct attribute_group *all_channel_counts_groups[] = {
    &all_channel_counts_grp,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn all_channel_counts_release(device: *mut device) {
    static void all_channel_counts_release(struct device *device)
    {
    edac_dbg(1, "Releasing device %s\n", dev_name(device));
    kfree(device);
    }
    static const struct device_type all_channel_counts_type = {
    .groups		= all_channel_counts_groups,
    .release	= all_channel_counts_release,
    };
//
// inject sysfs attributes
//
    static DEVICE_ATTR(inject_section, S_IRUGO | S_IWUSR,
    i7core_inject_section_show, i7core_inject_section_store);
    static DEVICE_ATTR(inject_type, S_IRUGO | S_IWUSR,
    i7core_inject_type_show, i7core_inject_type_store);
    static DEVICE_ATTR(inject_eccmask, S_IRUGO | S_IWUSR,
    i7core_inject_eccmask_show, i7core_inject_eccmask_store);
    static DEVICE_ATTR(inject_enable, S_IRUGO | S_IWUSR,
    i7core_inject_enable_show, i7core_inject_enable_store);
    static struct attribute *i7core_dev_attrs[] = {
    &dev_attr_inject_section.attr,
    &dev_attr_inject_type.attr,
    &dev_attr_inject_eccmask.attr,
    &dev_attr_inject_enable.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(i7core_dev);
#[no_mangle]
unsafe extern "C" fn i7core_create_sysfs_devices(mci: *mut mem_ctl_info) -> c_int {
    static int i7core_create_sysfs_devices(struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    int rc;
    pvt.addrmatch_dev = kzalloc_obj(*pvt.addrmatch_dev);
    if (!pvt.addrmatch_dev)
    return -ENOMEM;
    pvt.addrmatch_dev.type = &addrmatch_type;
    pvt.addrmatch_dev.bus = mci.dev.bus;
    device_initialize(pvt.addrmatch_dev);
    pvt.addrmatch_dev.parent = &mci.dev;
    dev_set_name(pvt.addrmatch_dev, "inject_addrmatch");
    dev_set_drvdata(pvt.addrmatch_dev, mci);
    edac_dbg(1, "creating %s\n", dev_name(pvt.addrmatch_dev));
    rc = device_add(pvt.addrmatch_dev);
    if (rc < 0)
    goto err_put_addrmatch;
    if (!pvt.is_registered) {
    pvt.chancounts_dev = kzalloc_obj(*pvt.chancounts_dev);
    if (!pvt.chancounts_dev) {
    rc = -ENOMEM;
    goto err_del_addrmatch;
    }
    pvt.chancounts_dev.type = &all_channel_counts_type;
    pvt.chancounts_dev.bus = mci.dev.bus;
    device_initialize(pvt.chancounts_dev);
    pvt.chancounts_dev.parent = &mci.dev;
    dev_set_name(pvt.chancounts_dev, "all_channel_counts");
    dev_set_drvdata(pvt.chancounts_dev, mci);
    edac_dbg(1, "creating %s\n", dev_name(pvt.chancounts_dev));
    rc = device_add(pvt.chancounts_dev);
    if (rc < 0)
    goto err_put_chancounts;
    }
    return 0;
    err_put_chancounts:
    put_device(pvt.chancounts_dev);
    err_del_addrmatch:
    device_del(pvt.addrmatch_dev);
    err_put_addrmatch:
    put_device(pvt.addrmatch_dev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn i7core_delete_sysfs_devices(mci: *mut mem_ctl_info) {
    static void i7core_delete_sysfs_devices(struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    edac_dbg(1, "\n");
    if (!pvt.is_registered) {
    device_del(pvt.chancounts_dev);
    put_device(pvt.chancounts_dev);
    }
    device_del(pvt.addrmatch_dev);
    put_device(pvt.addrmatch_dev);
    }
//
    Device initialization routines: put/get, init/exit
//
// i7core_put_all_devices	'put' all the devices that we have
// reserved via 'get'
//
#[no_mangle]
unsafe extern "C" fn i7core_put_devices(i7core_dev: *mut i7core_dev) {
    static void i7core_put_devices(struct i7core_dev *i7core_dev)
    {
    int i;
    edac_dbg(0, "\n");
    for (i = 0; i < i7core_dev.n_devs; i++) {
    struct pci_dev *pdev = i7core_dev.pdev[i];
    if (!pdev)
    continue;
    edac_dbg(0, "Removing dev %02x:%02x.%d\n",
    pdev.bus.number,
    PCI_SLOT(pdev.devfn), PCI_FUNC(pdev.devfn));
    pci_dev_put(pdev);
    }
    }
#[no_mangle]
unsafe extern "C" fn i7core_put_all_devices() {
    static void i7core_put_all_devices(void)
    {
    struct i7core_dev *i7core_dev, *tmp;
    list_for_each_entry_safe(i7core_dev, tmp, &i7core_edac_list, list) {
    i7core_put_devices(i7core_dev);
    free_i7core_dev(i7core_dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn i7core_xeon_pci_fixup(table: *const pci_id_table) -> void __init {
    static void __init i7core_xeon_pci_fixup(const struct pci_id_table *table)
    {
    struct pci_dev *pdev = core::ptr::null_mut();
    int i;
//
// On Xeon 55xx, the Intel Quick Path Arch Generic Non-core pci buses
// aren't announced by acpi. So, we need to use a legacy scan probing
// to detect them
//
    while (table && table.descr) {
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL, table.descr[0].dev_id, core::ptr::null_mut());
    if (unlikely(!pdev)) {
    for (i = 0; i < MAX_SOCKET_BUSES; i++)
    pcibios_scan_specific_bus(255-i);
    }
    pci_dev_put(pdev);
    table++;
    }
    }
#[no_mangle]
unsafe extern "C" fn i7core_pci_lastbus() -> unsigned {
    static unsigned i7core_pci_lastbus(void)
    {
    let mut last_bus: c_int = 0, bus;
    struct pci_bus *b = core::ptr::null_mut();
    while ((b = pci_find_next_bus(b)) != core::ptr::null_mut()) {
    bus = b.number;
    edac_dbg(0, "Found bus %d\n", bus);
    if (bus > last_bus)
    last_bus = bus;
    }
    edac_dbg(0, "Last bus %d\n", last_bus);
    return last_bus;
    }
//
// i7core_get_all_devices	Find and perform 'get' operation on the MCH's
// device/functions we want to reference for this driver
//
// Need to 'get' device 16 func 1 and func 2
//
    static int i7core_get_onedevice(struct pci_dev **prev,
    const struct pci_id_table *table,
    const unsigned devno,
    const unsigned last_bus)
    {
    struct i7core_dev *i7core_dev;
    const struct pci_id_descr *dev_descr = &table.descr[devno];
    struct pci_dev *pdev = core::ptr::null_mut();
    let mut bus: u8 = 0;
    let mut socket: u8 = 0;
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    dev_descr.dev_id, *prev);
//
// On Xeon 55xx, the Intel QuickPath Arch Generic Non-core regs
// is at addr 8086:2c40, instead of 8086:2c41. So, we need
// to probe for the alternate address in case of failure
//
    if (dev_descr.dev_id == PCI_DEVICE_ID_INTEL_I7_NONCORE && !pdev) {
    pci_dev_get(*prev);	/* pci_get_device will put it */
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_I7_NONCORE_ALT, *prev);
    }
    if (dev_descr.dev_id == PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE &&
    !pdev) {
    pci_dev_get(*prev);	/* pci_get_device will put it */
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE_ALT,
// prev);
    }
    if (!pdev) {
    if (*prev) {
// prev = pdev;
    return 0;
    }
    if (dev_descr.optional)
    return 0;
    if (devno == 0)
    return -ENODEV;
    i7core_printk(KERN_INFO,
    "Device not found: dev %02x.%d PCI ID %04x:%04x\n",
    dev_descr.dev, dev_descr.func,
    PCI_VENDOR_ID_INTEL, dev_descr.dev_id);
// End of list, leave
    return -ENODEV;
    }
    bus = pdev.bus.number;
    socket = last_bus - bus;
    i7core_dev = get_i7core_dev(socket);
    if (!i7core_dev) {
    i7core_dev = alloc_i7core_dev(socket, table);
    if (!i7core_dev) {
    pci_dev_put(pdev);
    return -ENOMEM;
    }
    }
    if (i7core_dev.pdev[devno]) {
    i7core_printk(KERN_ERR,
    "Duplicated device for "
    "dev %02x:%02x.%d PCI ID %04x:%04x\n",
    bus, dev_descr.dev, dev_descr.func,
    PCI_VENDOR_ID_INTEL, dev_descr.dev_id);
    pci_dev_put(pdev);
    return -ENODEV;
    }
    i7core_dev.pdev[devno] = pdev;
// Sanity check
    if (unlikely(PCI_SLOT(pdev.devfn) != dev_descr.dev ||
    PCI_FUNC(pdev.devfn) != dev_descr.func)) {
    i7core_printk(KERN_ERR,
    "Device PCI ID %04x:%04x "
    "has dev %02x:%02x.%d instead of dev %02x:%02x.%d\n",
    PCI_VENDOR_ID_INTEL, dev_descr.dev_id,
    bus, PCI_SLOT(pdev.devfn), PCI_FUNC(pdev.devfn),
    bus, dev_descr.dev, dev_descr.func);
    return -ENODEV;
    }
// Be sure that the device is enabled
    if (unlikely(pci_enable_device(pdev) < 0)) {
    i7core_printk(KERN_ERR,
    "Couldn't enable "
    "dev %02x:%02x.%d PCI ID %04x:%04x\n",
    bus, dev_descr.dev, dev_descr.func,
    PCI_VENDOR_ID_INTEL, dev_descr.dev_id);
    return -ENODEV;
    }
    edac_dbg(0, "Detected socket %d dev %02x:%02x.%d PCI ID %04x:%04x\n",
    socket, bus, dev_descr.dev,
    dev_descr.func,
    PCI_VENDOR_ID_INTEL, dev_descr.dev_id);
//
// As stated on drivers/pci/search.c, the reference count for
// @from is always decremented if it is not %NULL. So, as we need
// to get all devices up to null, we need to do a get for the device
//
    pci_dev_get(pdev);
// prev = pdev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i7core_get_all_devices() -> c_int {
    static int i7core_get_all_devices(void)
    {
    int i, rc, last_bus;
    struct pci_dev *pdev = core::ptr::null_mut();
    const struct pci_id_table *table = pci_dev_table;
    last_bus = i7core_pci_lastbus();
    while (table && table.descr) {
    for (i = 0; i < table.n_devs; i++) {
    pdev = core::ptr::null_mut();
    do {
    rc = i7core_get_onedevice(&pdev, table, i,
    last_bus);
    if (rc < 0) {
    if (i == 0) {
    i = table.n_devs;
    break;
    }
    i7core_put_all_devices();
    return -ENODEV;
    }
    } while (pdev);
    }
    table++;
    }
    return 0;
    }
    static int mci_bind_devs(struct mem_ctl_info *mci,
    struct i7core_dev *i7core_dev)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    struct pci_dev *pdev;
    int i, func, slot;
    char *family;
    pvt.is_registered = false;
    pvt.enable_scrub  = false;
    for (i = 0; i < i7core_dev.n_devs; i++) {
    pdev = i7core_dev.pdev[i];
    if (!pdev)
    continue;
    func = PCI_FUNC(pdev.devfn);
    slot = PCI_SLOT(pdev.devfn);
    if (slot == 3) {
    if (unlikely(func > MAX_MCR_FUNC))
    goto error;
    pvt.pci_mcr[func] = pdev;
    } else if (likely(slot >= 4 && slot < 4 + NUM_CHANS)) {
    if (unlikely(func > MAX_CHAN_FUNC))
    goto error;
    pvt.pci_ch[slot - 4][func] = pdev;
    } else if (!slot && !func) {
    pvt.pci_noncore = pdev;
// Detect the processor family
    switch (pdev.device) {
    case PCI_DEVICE_ID_INTEL_I7_NONCORE:
    family = "Xeon 35xx/ i7core";
    pvt.enable_scrub = false;
    break;
    case PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE_ALT:
    family = "i7-800/i5-700";
    pvt.enable_scrub = false;
    break;
    case PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE:
    family = "Xeon 34xx";
    pvt.enable_scrub = false;
    break;
    case PCI_DEVICE_ID_INTEL_I7_NONCORE_ALT:
    family = "Xeon 55xx";
    pvt.enable_scrub = true;
    break;
    case PCI_DEVICE_ID_INTEL_LYNNFIELD_NONCORE_REV2:
    family = "Xeon 56xx / i7-900";
    pvt.enable_scrub = true;
    break;
    default:
    family = "unknown";
    pvt.enable_scrub = false;
    }
    edac_dbg(0, "Detected a processor type %s\n", family);
    } else
    goto error;
    edac_dbg(0, "Associated fn %d.%d, dev = %p, socket %d\n",
    PCI_SLOT(pdev.devfn), PCI_FUNC(pdev.devfn),
    pdev, i7core_dev.socket);
    if (PCI_SLOT(pdev.devfn) == 3 &&
    PCI_FUNC(pdev.devfn) == 2)
    pvt.is_registered = true;
    }
    return 0;
    error:
    i7core_printk(KERN_ERR, "Device %d, function %d "
    "is out of the expected range\n",
    slot, func);
    return -EINVAL;
    }
//
    Error check routines
//
    static void i7core_rdimm_update_ce_count(struct mem_ctl_info *mci,
    const int chan,
    const int new0,
    const int new1,
    const int new2)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    let mut add0: c_int = 0, add1 = 0, add2 = 0;
// Updates CE counters if it is not the first time here
    if (pvt.ce_count_available) {
// Updates CE counters
    add2 = new2 - pvt.rdimm_last_ce_count[chan][2];
    add1 = new1 - pvt.rdimm_last_ce_count[chan][1];
    add0 = new0 - pvt.rdimm_last_ce_count[chan][0];
    if (add2 < 0)
    add2 += 0x7fff;
    pvt.rdimm_ce_count[chan][2] += add2;
    if (add1 < 0)
    add1 += 0x7fff;
    pvt.rdimm_ce_count[chan][1] += add1;
    if (add0 < 0)
    add0 += 0x7fff;
    pvt.rdimm_ce_count[chan][0] += add0;
    } else
    pvt.ce_count_available = 1;
// Store the new values
    pvt.rdimm_last_ce_count[chan][2] = new2;
    pvt.rdimm_last_ce_count[chan][1] = new1;
    pvt.rdimm_last_ce_count[chan][0] = new0;
// updated the edac core
    if (add0 != 0)
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, add0,
    0, 0, 0,
    chan, 0, -1, "error", "");
    if (add1 != 0)
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, add1,
    0, 0, 0,
    chan, 1, -1, "error", "");
    if (add2 != 0)
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, add2,
    0, 0, 0,
    chan, 2, -1, "error", "");
    }
#[no_mangle]
unsafe extern "C" fn i7core_rdimm_check_mc_ecc_err(mci: *mut mem_ctl_info) {
    static void i7core_rdimm_check_mc_ecc_err(struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    u32 rcv[3][2];
    int i, new0, new1, new2;
// Read DEV 3: FUN 2:  MC_COR_ECC_CNT regs directly
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_0,
    &rcv[0][0]);
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_1,
    &rcv[0][1]);
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_2,
    &rcv[1][0]);
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_3,
    &rcv[1][1]);
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_4,
    &rcv[2][0]);
    pci_read_config_dword(pvt.pci_mcr[2], MC_COR_ECC_CNT_5,
    &rcv[2][1]);
    for (i = 0 ; i < 3; i++) {
    edac_dbg(3, "MC_COR_ECC_CNT%d = 0x%x; MC_COR_ECC_CNT%d = 0x%x\n",
    (i * 2), rcv[i][0], (i * 2) + 1, rcv[i][1]);
// if the channel has 3 dimms
    if (pvt.channel[i].dimms > 2) {
    new0 = DIMM_BOT_COR_ERR(rcv[i][0]);
    new1 = DIMM_TOP_COR_ERR(rcv[i][0]);
    new2 = DIMM_BOT_COR_ERR(rcv[i][1]);
    } else {
    new0 = DIMM_TOP_COR_ERR(rcv[i][0]) +
    DIMM_BOT_COR_ERR(rcv[i][0]);
    new1 = DIMM_TOP_COR_ERR(rcv[i][1]) +
    DIMM_BOT_COR_ERR(rcv[i][1]);
    new2 = 0;
    }
    i7core_rdimm_update_ce_count(mci, i, new0, new1, new2);
    }
    }
// This function is based on the device 3 function 4 registers as described on:
// Intel Xeon Processor 5500 Series Datasheet Volume 2
// http://www.intel.com/Assets/PDF/datasheet/321322.pdf
// also available at:
// http://www.arrownac.com/manufacturers/intel/s/nehalem/5500-datasheet-v2.pdf
//
#[no_mangle]
unsafe extern "C" fn i7core_udimm_check_mc_ecc_err(mci: *mut mem_ctl_info) {
    static void i7core_udimm_check_mc_ecc_err(struct mem_ctl_info *mci)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    u32 rcv1, rcv0;
    int new0, new1, new2;
    if (!pvt.pci_mcr[4]) {
    edac_dbg(0, "MCR registers not found\n");
    return;
    }
// Corrected test errors
    pci_read_config_dword(pvt.pci_mcr[4], MC_TEST_ERR_RCV1, &rcv1);
    pci_read_config_dword(pvt.pci_mcr[4], MC_TEST_ERR_RCV0, &rcv0);
// Store the new values
    new2 = DIMM2_COR_ERR(rcv1);
    new1 = DIMM1_COR_ERR(rcv0);
    new0 = DIMM0_COR_ERR(rcv0);
// Updates CE counters if it is not the first time here
    if (pvt.ce_count_available) {
// Updates CE counters
    int add0, add1, add2;
    add2 = new2 - pvt.udimm_last_ce_count[2];
    add1 = new1 - pvt.udimm_last_ce_count[1];
    add0 = new0 - pvt.udimm_last_ce_count[0];
    if (add2 < 0)
    add2 += 0x7fff;
    pvt.udimm_ce_count[2] += add2;
    if (add1 < 0)
    add1 += 0x7fff;
    pvt.udimm_ce_count[1] += add1;
    if (add0 < 0)
    add0 += 0x7fff;
    pvt.udimm_ce_count[0] += add0;
    if (add0 | add1 | add2)
    i7core_printk(KERN_ERR, "New Corrected error(s): "
    "dimm0: +%d, dimm1: +%d, dimm2 +%d\n",
    add0, add1, add2);
    } else
    pvt.ce_count_available = 1;
// Store the new values
    pvt.udimm_last_ce_count[2] = new2;
    pvt.udimm_last_ce_count[1] = new1;
    pvt.udimm_last_ce_count[0] = new0;
    }
//
// According with tables E-11 and E-12 of chapter E.3.3 of Intel 64 and IA-32
// Architectures Software Developer’s Manual Volume 3B.
// Nehalem are defined as family 0x06, model 0x1a
//
// The MCA registers used here are the following ones:
// struct mce field	MCA Register
// m->status	MSR_IA32_MC8_STATUS
// m->addr		MSR_IA32_MC8_ADDR
// m->misc		MSR_IA32_MC8_MISC
// In the case of Nehalem, the error information is masked at .status and .misc
// fields
//
    static void i7core_mce_output_error(struct mem_ctl_info *mci,
    const struct mce *m)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    char *optype, *err;
    enum hw_event_mc_err_type tp_event;
    let mut error: c_ulong = m.status & 0x1ff0000l;
    let mut uncorrected_error: bool = m.mcgstatus & 1ll << 61;
    let mut ripv: bool = m.mcgstatus & 1;
    let mut optypenum: u32 = (m.status >> 4) & 0x07;
    let mut core_err_cnt: u32 = (m.status >> 38) & 0x7fff;
    let mut dimm: u32 = (m.misc >> 16) & 0x3;
    let mut channel: u32 = (m.misc >> 18) & 0x3;
    let mut syndrome: u32 = m.misc >> 32;
    let mut errnum: u32 = find_first_bit(&error, 32);
    if (uncorrected_error) {
    core_err_cnt = 1;
    if (ripv)
    tp_event = HW_EVENT_ERR_UNCORRECTED;
    else
    tp_event = HW_EVENT_ERR_FATAL;
    } else {
    tp_event = HW_EVENT_ERR_CORRECTED;
    }
    switch (optypenum) {
    case 0:
    optype = "generic undef request";
    break;
    case 1:
    optype = "read error";
    break;
    case 2:
    optype = "write error";
    break;
    case 3:
    optype = "addr/cmd error";
    break;
    case 4:
    optype = "scrubbing error";
    break;
    default:
    optype = "reserved";
    break;
    }
    switch (errnum) {
    case 16:
    err = "read ECC error";
    break;
    case 17:
    err = "RAS ECC error";
    break;
    case 18:
    err = "write parity error";
    break;
    case 19:
    err = "redundancy loss";
    break;
    case 20:
    err = "reserved";
    break;
    case 21:
    err = "memory range error";
    break;
    case 22:
    err = "RTID out of range";
    break;
    case 23:
    err = "address parity error";
    break;
    case 24:
    err = "byte enable parity error";
    break;
    default:
    err = "unknown";
    }
//
// Call the helper to output message
// FIXME: what to do if core_err_cnt > 1? Currently, it generates
// only one event
//
    if (uncorrected_error || !pvt.is_registered)
    edac_mc_handle_error(tp_event, mci, core_err_cnt,
    m.addr >> PAGE_SHIFT,
    m.addr & ~PAGE_MASK,
    syndrome,
    channel, dimm, -1,
    err, optype);
    }
//
// i7core_check_error	Retrieve and process errors reported by the
// hardware. Called by the Core module.
//
#[no_mangle]
unsafe extern "C" fn i7core_check_error(mci: *mut mem_ctl_info, m: *mut mce) {
    static void i7core_check_error(struct mem_ctl_info *mci, struct mce *m)
    {
    struct i7core_pvt *pvt = mci.pvt_info;
    i7core_mce_output_error(mci, m);
//
// Now, let's increment CE error counts
//
    if (!pvt.is_registered)
    i7core_udimm_check_mc_ecc_err(mci);
    else
    i7core_rdimm_check_mc_ecc_err(mci);
    }
//
// Check that logging is enabled and that this is the right type
// of error for us to handle.
//
    static int i7core_mce_check_error(struct notifier_block *nb, unsigned long val,
    void *data)
    {
    struct mce *mce = (struct mce *)data;
    struct i7core_dev *i7_dev;
    struct mem_ctl_info *mci;
    i7_dev = get_i7core_dev(mce.socketid);
    if (!i7_dev || (mce.kflags & MCE_HANDLED_CEC))
    return NOTIFY_DONE;
    mci = i7_dev.mci;
//
// Just let mcelog handle it if the error is
// outside the memory controller
//
    if (((mce.status & 0xffff) >> 7) != 1)
    return NOTIFY_DONE;
// Bank 8 registers are the only ones that we know how to handle
    if (mce.bank != 8)
    return NOTIFY_DONE;
    i7core_check_error(mci, mce);
// Advise mcelog that the errors were handled
    mce.kflags |= MCE_HANDLED_EDAC;
    return NOTIFY_OK;
    }
    static struct notifier_block i7_mce_dec = {
    .notifier_call	= i7core_mce_check_error,
    .priority	= MCE_PRIO_EDAC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memdev_dmi_entry {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
    pub phys_mem_array_handle: u16,
    pub mem_err_info_handle: u16,
    pub total_width: u16,
    pub data_width: u16,
    pub size: u16,
    pub form: u8,
    pub device_set: u8,
    pub device_locator: u8,
    pub bank_locator: u8,
    pub memory_type: u8,
    pub type_detail: u16,
    pub speed: u16,
    pub manufacturer: u8,
    pub serial_number: u8,
    pub asset_tag: u8,
    pub part_number: u8,
    pub attributes: u8,
    pub extended_size: u32,
    pub conf_mem_clk_speed: u16,
    pub __attribute__((__packed__)): },
//
// Decode the DRAM Clock Frequency, be paranoid, make sure that all
// memory devices show the same speed, and if they don't then consider
// all speeds to be invalid.
//
#[no_mangle]
unsafe extern "C" fn decode_dclk(dh: *const dmi_header, _dclk_freq: *mut c_void) {
    static void decode_dclk(const struct dmi_header *dh, void *_dclk_freq)
    {
    pub _dclk_freq: *mut *mut int dclk_freq =,
    pub dmi_mem_clk_speed: u16,
    if (*dclk_freq == -1)
    if (dh.type == DMI_ENTRY_MEM_DEVICE) {
    struct memdev_dmi_entry *memdev_dmi_entry =
    pub )dh: *mut (struct memdev_dmi_entry,
    unsigned long conf_mem_clk_speed_offset =
    (unsigned long)&memdev_dmi_entry.conf_mem_clk_speed -
    pub long)&memdev_dmi_entry->type: (unsigned,
    unsigned long speed_offset =
    (unsigned long)&memdev_dmi_entry.speed -
    pub long)&memdev_dmi_entry->type: (unsigned,
// Check that a DIMM is present
    if (memdev_dmi_entry.size == 0)
//
// Pick the configured speed if it's available, otherwise
// pick the DIMM speed, or we don't have a speed.
//
    if (memdev_dmi_entry.length > conf_mem_clk_speed_offset) {
    dmi_mem_clk_speed =
    } else if (memdev_dmi_entry.length > speed_offset) {
    pub memdev_dmi_entry->speed: dmi_mem_clk_speed =,
    } else {
// dclk_freq = -1;
    }
    if (*dclk_freq == 0) {
// First pass, speed was 0
    if (dmi_mem_clk_speed > 0) {
// Set speed if a valid speed is read
// dclk_freq = dmi_mem_clk_speed;
    } else {
// Otherwise we don't have a valid speed
// dclk_freq = -1;
    }
    } else if (*dclk_freq > 0 &&
// dclk_freq != dmi_mem_clk_speed) {
//
// If we have a speed, check that all DIMMS are the same
// speed, otherwise set the speed as invalid.
//
// dclk_freq = -1;
    }
    }
    }
//
// The default DCLK frequency is used as a fallback if we
// fail to find anything reliable in the DMI. The value
// is taken straight from the datasheet.
//
pub const DEFAULT_DCLK_FREQ: c_int = 800;
#[no_mangle]
unsafe extern "C" fn get_dclk_freq() -> c_int {
    static int get_dclk_freq(void)
    {
    pub 0: int dclk_freq =,
    pub )&dclk_freq): *mut dmi_walk(decode_dclk, (void,
    if (dclk_freq < 1)
    pub DEFAULT_DCLK_FREQ: return,
    pub dclk_freq: return,
    }
//
// set_sdram_scrub_rate		This routine sets byte/sec bandwidth scrub rate
// to hardware according to SCRUBINTERVAL formula
// found in datasheet.
//
#[no_mangle]
unsafe extern "C" fn set_sdram_scrub_rate(mci: *mut mem_ctl_info, new_bw: u32) -> c_int {
    static int set_sdram_scrub_rate(struct mem_ctl_info *mci, u32 new_bw)
    {
    pub mci->pvt_info: *mut *mut i7core_pvt pvt =,
    pub pdev: *mut pci_dev,
    pub dw_scrub: u32,
    pub dw_ssr: u32,
// Get data from the MC register, function 2
    pub pvt->pci_mcr[2]: pdev =,
    if (!pdev)
    pub -ENODEV: return,
    pub &dw_scrub): pci_read_config_dword(pdev, MC_SCRUB_CONTROL,,
    if (new_bw == 0) {
// Prepare to disable petrol scrub
    pub ~STARTSCRUB: dw_scrub &=,
// Stop the patrol scrub engine
    write_and_test(pdev, MC_SCRUB_CONTROL,
    pub ~SCRUBINTERVAL_MASK): dw_scrub &,
// Get current status of scrub rate and set bit to disable
    pub &dw_ssr): pci_read_config_dword(pdev, MC_SSRCONTROL,,
    pub ~SSR_MODE_MASK: dw_ssr &=,
    pub SSR_MODE_DISABLE: dw_ssr |=,
    } else {
    pub 64: int cache_line_size =,
    pub pvt->dclk_freq: u32 freq_dclk_mhz =,
    pub scrub_interval: c_ulonglong,
//
// Translate the desired scrub rate to a register value and
// program the corresponding register value.
//
    scrub_interval = (unsigned long long)freq_dclk_mhz *
    pub 1000000: *mut *mut cache_line_size,
    pub new_bw): do_div(scrub_interval,,
    if (!scrub_interval || scrub_interval > SCRUBINTERVAL_MASK)
    pub -EINVAL: return,
    pub scrub_interval: dw_scrub = SCRUBINTERVAL_MASK &,
// Start the patrol scrub engine
    pci_write_config_dword(pdev, MC_SCRUB_CONTROL,
    pub dw_scrub): STARTSCRUB |,
// Get current status of scrub rate and set bit to enable
    pub &dw_ssr): pci_read_config_dword(pdev, MC_SSRCONTROL,,
    pub ~SSR_MODE_MASK: dw_ssr &=,
    pub SSR_MODE_ENABLE: dw_ssr |=,
    }
// Disable or enable scrubbing
    pub dw_ssr): pci_write_config_dword(pdev, MC_SSRCONTROL,,
    pub new_bw: return,
    }
//
// get_sdram_scrub_rate		This routine convert current scrub rate value
// into byte/sec bandwidth according to
// SCRUBINTERVAL formula found in datasheet.
//
#[no_mangle]
unsafe extern "C" fn get_sdram_scrub_rate(mci: *mut mem_ctl_info) -> c_int {
    static int get_sdram_scrub_rate(struct mem_ctl_info *mci)
    {
    pub mci->pvt_info: *mut *mut i7core_pvt pvt =,
    pub pdev: *mut pci_dev,
    pub 64: u32 cache_line_size =,
    pub pvt->dclk_freq: u32 freq_dclk_mhz =,
    pub scrub_rate: c_ulonglong,
    pub scrubval: u32,
// Get data from the MC register, function 2
    pub pvt->pci_mcr[2]: pdev =,
    if (!pdev)
    pub -ENODEV: return,
// Get current scrub control data
    pub &scrubval): pci_read_config_dword(pdev, MC_SCRUB_CONTROL,,
// Mask highest 8-bits to 0
    pub SCRUBINTERVAL_MASK: scrubval &=,
    if (!scrubval)
    pub 0: return,
// Calculate scrub rate value into byte/sec bandwidth
    scrub_rate =  (unsigned long long)freq_dclk_mhz *
    pub cache_line_size: *mut *mut 1000000,
    pub scrubval): do_div(scrub_rate,,
    pub (int)scrub_rate: return,
    }
#[no_mangle]
unsafe extern "C" fn enable_sdram_scrub_setting(mci: *mut mem_ctl_info) {
    static void enable_sdram_scrub_setting(struct mem_ctl_info *mci)
    {
    pub mci->pvt_info: *mut *mut i7core_pvt pvt =,
    pub pci_lock: u32,
// Unlock writes to pci registers
    pub &pci_lock): pci_read_config_dword(pvt->pci_noncore, MC_CFG_CONTROL,,
    pub ~0x3: pci_lock &=,
    pci_write_config_dword(pvt.pci_noncore, MC_CFG_CONTROL,
    pub MC_CFG_UNLOCK): pci_lock |,
    pub set_sdram_scrub_rate: mci->set_sdram_scrub_rate =,
    pub get_sdram_scrub_rate: mci->get_sdram_scrub_rate =,
    }
#[no_mangle]
unsafe extern "C" fn disable_sdram_scrub_setting(mci: *mut mem_ctl_info) {
    static void disable_sdram_scrub_setting(struct mem_ctl_info *mci)
    {
    pub mci->pvt_info: *mut *mut i7core_pvt pvt =,
    pub pci_lock: u32,
// Lock writes to pci registers
    pub &pci_lock): pci_read_config_dword(pvt->pci_noncore, MC_CFG_CONTROL,,
    pub ~0x3: pci_lock &=,
    pci_write_config_dword(pvt.pci_noncore, MC_CFG_CONTROL,
    pub MC_CFG_LOCK): pci_lock |,
    }
#[no_mangle]
unsafe extern "C" fn i7core_pci_ctl_create(pvt: *mut i7core_pvt) {
    static void i7core_pci_ctl_create(struct i7core_pvt *pvt)
    {
    pvt.i7core_pci = edac_pci_create_generic_ctl(
    &pvt.i7core_dev.pdev[0].dev,
    if (unlikely(!pvt.i7core_pci))
    i7core_printk(KERN_WARNING,
    pub EDAC\n"): "Unable to setup PCI error report via,
    }
#[no_mangle]
unsafe extern "C" fn i7core_pci_ctl_release(pvt: *mut i7core_pvt) {
    static void i7core_pci_ctl_release(struct i7core_pvt *pvt)
    {
    if (likely(pvt.i7core_pci))
    else
    i7core_printk(KERN_ERR,
    "Couldn't find mem_ctl_info for socket %d\n",
    pub NULL: pvt->i7core_pci =,
    }
#[no_mangle]
unsafe extern "C" fn i7core_unregister_mci(i7core_dev: *mut i7core_dev) {
    static void i7core_unregister_mci(struct i7core_dev *i7core_dev)
    {
    pub i7core_dev->mci: *mut *mut mem_ctl_info mci =,
    pub pvt: *mut i7core_pvt,
    if (unlikely(!mci || !mci.pvt_info)) {
    pub &i7core_dev->pdev[0]->dev): edac_dbg(0, "MC: dev = %p\n",,
    pub handler\n"): i7core_printk(KERN_ERR, "Couldn't find mci,
    }
    pub mci->pvt_info: pvt =,
    pub &i7core_dev->pdev[0]->dev): edac_dbg(0, "MC: mci = %p, dev = %p\n", mci,,
// Disable scrubrate setting
    if (pvt.enable_scrub)
// Disable EDAC polling
// Remove MC sysfs nodes
    pub mci->ctl_name): edac_dbg(1, "%s: free mci struct\n",,
    pub NULL: i7core_dev->mci =,
    }
#[no_mangle]
unsafe extern "C" fn i7core_register_mci(i7core_dev: *mut i7core_dev) -> c_int {
    static int i7core_register_mci(struct i7core_dev *i7core_dev)
    {
    pub mci: *mut mem_ctl_info,
    pub pvt: *mut i7core_pvt,
    pub rc: c_int,
    pub layers: [edac_mc_layer; 2],
// allocate a new MC control structure
    pub EDAC_MC_LAYER_CHANNEL: layers[0].type =,
    pub NUM_CHANS: layers[0].size =,
    pub false: layers[0].is_virt_csrow =,
    pub EDAC_MC_LAYER_SLOT: layers[1].type =,
    pub MAX_DIMMS: layers[1].size =,
    pub true: layers[1].is_virt_csrow =,
    mci = edac_mc_alloc(i7core_dev.socket, ARRAY_SIZE(layers), layers,
    if (unlikely(!mci))
    pub -ENOMEM: return,
    pub &i7core_dev->pdev[0]->dev): edac_dbg(0, "MC: mci = %p, dev = %p\n", mci,,
    pub mci->pvt_info: pvt =,
    pub sizeof(*pvt)): *mut memset(pvt, 0,,
// Associates i7core_dev and mci for future usage
    pub i7core_dev: pvt->i7core_dev =,
    pub mci: i7core_dev->mci =,
//
// FIXME: how to handle RDDR3 at MCI level? It is possible to have
// Mixed RDDR3/UDDR3 with Nehalem, provided that they are on different
// memory channels
//
    pub MEM_FLAG_DDR3: mci->mtype_cap =,
    pub EDAC_FLAG_NONE: mci->edac_ctl_cap =,
    pub EDAC_FLAG_NONE: mci->edac_cap =,
    pub "i7core_edac.c": mci->mod_name =,
    pub i7core_dev->socket): mci->ctl_name = kasprintf(GFP_KERNEL, "i7 core #%d",,
    if (!mci.ctl_name) {
    pub -ENOMEM: rc =,
    pub fail1: goto,
    }
    pub pci_name(i7core_dev->pdev[0]): mci->dev_name =,
    pub NULL: mci->ctl_page_to_phys =,
// Store pci devices at mci for faster access
    pub i7core_dev): rc = mci_bind_devs(mci,,
    if (unlikely(rc < 0))
    pub fail0: goto,
// Get dimm basic config
// record ptr to the generic device
    pub &i7core_dev->pdev[0]->dev: mci->pdev =,
// Enable scrubrate setting
    if (pvt.enable_scrub)
// add this new MC control structure to EDAC's list of MCs
    if (unlikely(edac_mc_add_mc_with_groups(mci, i7core_dev_groups))) {
    pub edac_mc_add_mc()\n"): edac_dbg(0, "MC: failed,
// FIXME: perhaps some code should go here that disables error
// reporting if we just enabled it
//
    pub -EINVAL: rc =,
    pub fail0: goto,
    }
    if (i7core_create_sysfs_devices(mci)) {
    pub nodes\n"): edac_dbg(0, "MC: failed to create sysfs,
    pub -EINVAL: rc =,
    pub fail0: goto,
    }
// Default error mask is any memory
    pub 0: pvt->inject.channel =,
    pub -1: pvt->inject.dimm =,
    pub -1: pvt->inject.rank =,
    pub -1: pvt->inject.bank =,
    pub -1: pvt->inject.page =,
    pub -1: pvt->inject.col =,
// allocating generic PCI control info
// DCLK for scrub rate setting
    pub get_dclk_freq(): pvt->dclk_freq =,
    pub 0: return,
    fail0:
    fail1:
    pub NULL: i7core_dev->mci =,
    pub rc: return,
    }
//
// i7core_probe	Probe for ONE instance of device to see if it is
// present.
// return:
// 0 for FOUND a device
// < 0 for error code
//
#[no_mangle]
unsafe extern "C" fn i7core_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int i7core_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    pub 0: int rc, count =,
    pub i7core_dev: *mut i7core_dev,
// get the pci devices we want to reserve for our use
//
// All memory controllers are allocated at the first pass.
//
    if (unlikely(probed >= 1)) {
    pub -ENODEV: return,
    }
    pub i7core_get_all_devices(): rc =,
    if (unlikely(rc < 0))
    pub fail0: goto,
    list_for_each_entry(i7core_dev, &i7core_edac_list, list) {
    pub i7core_register_mci(i7core_dev): rc =,
    if (unlikely(rc < 0))
    pub fail1: goto,
    }
//
// Nehalem-EX uses a different memory controller. However, as the
// memory controller is not visible on some Nehalem/Nehalem-EP, we
// need to indirectly probe via a X58 PCI device. The same devices
// are found on (some) Nehalem-EX. So, on those machines, the
// probe routine needs to return -ENODEV, as the actual Memory
// Controller registers won't be detected.
//
    if (!count) {
    pub -ENODEV: rc =,
    pub fail1: goto,
    }
    i7core_printk(KERN_INFO,
    "Driver loaded, %d memory controller(s) found.\n",
    pub 0: return,
    fail1:
    list_for_each_entry(i7core_dev, &i7core_edac_list, list)
    fail0:
    pub rc: return,
    }
//
// i7core_remove	destructor for one instance of device
//
#[no_mangle]
unsafe extern "C" fn i7core_remove(pdev: *mut pci_dev) {
    static void i7core_remove(struct pci_dev *pdev)
    {
    pub i7core_dev: *mut i7core_dev,
    pub "\n"): edac_dbg(0,,
//
// we have a trouble here: pdev value for removal will be wrong, since
// it will point to the X58 register used to detect that the machine
// is a Nehalem or upper design. However, due to the way several PCI
// devices are grouped together to provide MC functionality, we need
// to use a different method for releasing the devices
//
    if (unlikely(!probed)) {
    }
    list_for_each_entry(i7core_dev, &i7core_edac_list, list)
// Release PCI resources
    }
    pub i7core_pci_tbl): MODULE_DEVICE_TABLE(pci,,
//
// i7core_driver	pci_driver structure for this module
//
    static struct pci_driver i7core_driver = {
    .name     = "i7core_edac",
    .probe    = i7core_probe,
    .remove   = i7core_remove,
    .id_table = i7core_pci_tbl,
}

//
// i7core_init		Module entry function
// Try to initialize this module for its devices
//
#[no_mangle]
unsafe extern "C" fn i7core_init() -> int __init {
    static int __init i7core_init(void)
    {
    int pci_rc;
    edac_dbg(2, "\n");
// Ensure that the OPSTATE is set correctly for POLL or NMI
    opstate_init();
    if (use_pci_fixup)
    i7core_xeon_pci_fixup(pci_dev_table);
    pci_rc = pci_register_driver(&i7core_driver);
    if (pci_rc >= 0) {
    mce_register_decode_chain(&i7_mce_dec);
    return 0;
    }
    i7core_printk(KERN_ERR, "Failed to register device with error %d.\n",
    pci_rc);
    return pci_rc;
    }
//
// i7core_exit()	Module exit function
// Unregister the driver
//
#[no_mangle]
unsafe extern "C" fn i7core_exit() -> void __exit {
    static void __exit i7core_exit(void)
    {
    edac_dbg(2, "\n");
    pci_unregister_driver(&i7core_driver);
    mce_unregister_decode_chain(&i7_mce_dec);
    }
    module_init(i7core_init);
    module_exit(i7core_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_AUTHOR("Red Hat Inc. (https://www.redhat.com)");
    MODULE_DESCRIPTION("MC Driver for Intel i7 Core memory controllers - "
    I7CORE_REVISION);
    module_param(edac_op_state, int, 0444);
    MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");
