//! Automatically rewritten from C to Rust
//! Source: drivers/edac/i5400_edac.c
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


//
// Intel 5400 class Memory Controllers kernel module (Seaburg)
//
// This file may be distributed under the terms of the
// GNU General Public License.
//
// Copyright (c) 2008 by:
// Ben Woodard <woodard@redhat.com>
// Mauro Carvalho Chehab
//
// Red Hat Inc. https://www.redhat.com
//
// Forked and adapted from the i5000_edac driver which was
// written by Douglas Thompson Linux Networx <norsk5@xmission.com>
//
// This module is based on the following document:
//
// Intel 5400 Chipset Memory Controller Hub (MCH) - Datasheet
// http://developer.intel.com/design/chipsets/datashts/313070.htm
//
// This Memory Controller manages DDR2 FB-DIMMs. It has 2 branches, each with
// 2 channels operating in lockstep no-mirror mode. Each channel can have up to
// 4 dimm's, each with up to 8GB.
//

//
// Alter this version for the I5400 module when modifications are made
//

    edac_printk(level, "i5400", fmt, ##arg)

    edac_mc_chipset_printk(mci, level, "i5400", fmt, ##arg)
// Limits for i5400
pub const MAX_BRANCHES: c_int = 2;
pub const CHANNELS_PER_BRANCH: c_int = 2;
pub const DIMMS_PER_CHANNEL: c_int = 4;

// Device 16,
// Function 0: System Address
// Function 1: Memory Branch Map, Control, Errors Register
// Function 2: FSB Error Registers
//
// All 3 functions of Device 16 (0,1,2) share the SAME DID and
// uses PCI_DEVICE_ID_INTEL_5400_ERR for device 16 (0,1,2),
// PCI_DEVICE_ID_INTEL_5400_FBD0 and PCI_DEVICE_ID_INTEL_5400_FBD1
// for device 21 (0,1).
//
// OFFSETS for Function 0
pub const AMBASE: c_uint = 0x48 /* AMB Mem Mapped Reg Region Base */;
pub const MAXCH: c_uint = 0x56 /* Max Channel Number */;
pub const MAXDIMMPERCH: c_uint = 0x57 /* Max DIMM PER Channel Number */;
// OFFSETS for Function 1
pub const TOLM: c_uint = 0x6C;
pub const REDMEMB: c_uint = 0x7C;

pub const MIR0: c_uint = 0x80;
pub const MIR1: c_uint = 0x84;
pub const AMIR0: c_uint = 0x8c;
pub const AMIR1: c_uint = 0x90;
// Fatal error registers
pub const FERR_FAT_FBD: c_uint = 0x98	/* also called as FERR_FAT_FB_DIMM at datasheet */;

pub const NERR_FAT_FBD: c_uint = 0x9c;
pub const FERR_NF_FBD: c_uint = 0xa0	/* also called as FERR_NFAT_FB_DIMM at datasheet */;
// Non-fatal error register
pub const NERR_NF_FBD: c_uint = 0xa4;
// Enable error mask
pub const EMASK_FBD: c_uint = 0xa8;
pub const ERR0_FBD: c_uint = 0xac;
pub const ERR1_FBD: c_uint = 0xb0;
pub const ERR2_FBD: c_uint = 0xb4;
pub const MCERR_FBD: c_uint = 0xb8;
// No OFFSETS for Device 16 Function 2
//
// Device 21,
// Function 0: Memory Map Branch 0
//
// Device 22,
// Function 0: Memory Map Branch 1
//
// OFFSETS for Function 0
pub const AMBPRESENT_0: c_uint = 0x64;
pub const AMBPRESENT_1: c_uint = 0x66;
pub const MTR0: c_uint = 0x80;
pub const MTR1: c_uint = 0x82;
pub const MTR2: c_uint = 0x84;
pub const MTR3: c_uint = 0x86;
// OFFSETS for Function 1
pub const NRECFGLOG: c_uint = 0x74;
pub const RECFGLOG: c_uint = 0x78;
pub const NRECMEMA: c_uint = 0xbe;
pub const NRECMEMB: c_uint = 0xc0;
pub const NRECFB_DIMMA: c_uint = 0xc4;
pub const NRECFB_DIMMB: c_uint = 0xc8;
pub const NRECFB_DIMMC: c_uint = 0xcc;
pub const NRECFB_DIMMD: c_uint = 0xd0;
pub const NRECFB_DIMME: c_uint = 0xd4;
pub const NRECFB_DIMMF: c_uint = 0xd8;
pub const REDMEMA: c_uint = 0xdC;
pub const RECMEMA: c_uint = 0xf0;
pub const RECMEMB: c_uint = 0xf4;
pub const RECFB_DIMMA: c_uint = 0xf8;
pub const RECFB_DIMMB: c_uint = 0xec;
pub const RECFB_DIMMC: c_uint = 0xf0;
pub const RECFB_DIMMD: c_uint = 0xf4;
pub const RECFB_DIMME: c_uint = 0xf8;
pub const RECFB_DIMMF: c_uint = 0xfC;
//
// Error indicator bits and masks
// Error masks are according with Table 5-17 of i5400 datasheet
//
    enum error_mask {
    EMASK_M1  = 1<<0,  /* Memory Write error on non-redundant retry */
    EMASK_M2  = 1<<1,  /* Memory or FB-DIMM configuration CRC read error */
    EMASK_M3  = 1<<2,  /* Reserved */
    EMASK_M4  = 1<<3,  /* Uncorrectable Data ECC on Replay */
    EMASK_M5  = 1<<4,  /* Aliased Uncorrectable Non-Mirrored Demand Data ECC */
    EMASK_M6  = 1<<5,  /* Unsupported on i5400 */
    EMASK_M7  = 1<<6,  /* Aliased Uncorrectable Resilver- or Spare-Copy Data ECC */
    EMASK_M8  = 1<<7,  /* Aliased Uncorrectable Patrol Data ECC */
    EMASK_M9  = 1<<8,  /* Non-Aliased Uncorrectable Non-Mirrored Demand Data ECC */
    EMASK_M10 = 1<<9,  /* Unsupported on i5400 */
    EMASK_M11 = 1<<10, /* Non-Aliased Uncorrectable Resilver- or Spare-Copy Data ECC  */
    EMASK_M12 = 1<<11, /* Non-Aliased Uncorrectable Patrol Data ECC */
    EMASK_M13 = 1<<12, /* Memory Write error on first attempt */
    EMASK_M14 = 1<<13, /* FB-DIMM Configuration Write error on first attempt */
    EMASK_M15 = 1<<14, /* Memory or FB-DIMM configuration CRC read error */
    EMASK_M16 = 1<<15, /* Channel Failed-Over Occurred */
    EMASK_M17 = 1<<16, /* Correctable Non-Mirrored Demand Data ECC */
    EMASK_M18 = 1<<17, /* Unsupported on i5400 */
    EMASK_M19 = 1<<18, /* Correctable Resilver- or Spare-Copy Data ECC */
    EMASK_M20 = 1<<19, /* Correctable Patrol Data ECC */
    EMASK_M21 = 1<<20, /* FB-DIMM Northbound parity error on FB-DIMM Sync Status */
    EMASK_M22 = 1<<21, /* SPD protocol Error */
    EMASK_M23 = 1<<22, /* Non-Redundant Fast Reset Timeout */
    EMASK_M24 = 1<<23, /* Refresh error */
    EMASK_M25 = 1<<24, /* Memory Write error on redundant retry */
    EMASK_M26 = 1<<25, /* Redundant Fast Reset Timeout */
    EMASK_M27 = 1<<26, /* Correctable Counter Threshold Exceeded */
    EMASK_M28 = 1<<27, /* DIMM-Spare Copy Completed */
    EMASK_M29 = 1<<28, /* DIMM-Isolation Completed */
    };
//
// Names to translate bit error into something useful
//
    static const char *error_name[] = {
    [0]  = "Memory Write error on non-redundant retry",
    [1]  = "Memory or FB-DIMM configuration CRC read error",
// Reserved
    [3]  = "Uncorrectable Data ECC on Replay",
    [4]  = "Aliased Uncorrectable Non-Mirrored Demand Data ECC",
// M6 Unsupported on i5400
    [6]  = "Aliased Uncorrectable Resilver- or Spare-Copy Data ECC",
    [7]  = "Aliased Uncorrectable Patrol Data ECC",
    [8]  = "Non-Aliased Uncorrectable Non-Mirrored Demand Data ECC",
// M10 Unsupported on i5400
    [10] = "Non-Aliased Uncorrectable Resilver- or Spare-Copy Data ECC",
    [11] = "Non-Aliased Uncorrectable Patrol Data ECC",
    [12] = "Memory Write error on first attempt",
    [13] = "FB-DIMM Configuration Write error on first attempt",
    [14] = "Memory or FB-DIMM configuration CRC read error",
    [15] = "Channel Failed-Over Occurred",
    [16] = "Correctable Non-Mirrored Demand Data ECC",
// M18 Unsupported on i5400
    [18] = "Correctable Resilver- or Spare-Copy Data ECC",
    [19] = "Correctable Patrol Data ECC",
    [20] = "FB-DIMM Northbound parity error on FB-DIMM Sync Status",
    [21] = "SPD protocol Error",
    [22] = "Non-Redundant Fast Reset Timeout",
    [23] = "Refresh error",
    [24] = "Memory Write error on redundant retry",
    [25] = "Redundant Fast Reset Timeout",
    [26] = "Correctable Counter Threshold Exceeded",
    [27] = "DIMM-Spare Copy Completed",
    [28] = "DIMM-Isolation Completed",
    };
// Fatal errors

    EMASK_M2 | \
    EMASK_M23)
// Correctable errors

    EMASK_M20 | \
    EMASK_M19 | \
    EMASK_M18 | \
    EMASK_M17 | \
    EMASK_M16)

    EMASK_M28)

// Recoverable errors

    EMASK_M25 | \
    EMASK_M24 | \
    EMASK_M15 | \
    EMASK_M14 | \
    EMASK_M13 | \
    EMASK_M12 | \
    EMASK_M11 | \
    EMASK_M9  | \
    EMASK_M8  | \
    EMASK_M7  | \
    EMASK_M5)
// uncorrectable errors

// mask to all non-fatal errors

    ERROR_NF_UNCORRECTABLE | \
    ERROR_NF_RECOVERABLE   | \
    ERROR_NF_DIMM_SPARE    | \
    ERROR_NF_SPD_PROTOCOL  | \
    ERROR_NF_NORTH_CRC)
//
// Define error masks for the several registers
//
// Enable all fatal and non fatal errors

// mask for fatal error registers

// masks for non-fatal error register
#[no_mangle]
pub unsafe extern "C" fn to_nf_mask(mask: c_uint) -> c_int {
    static inline int to_nf_mask(unsigned int mask)
    {
    return (mask & EMASK_M29) | (mask >> 3);
    };
#[no_mangle]
pub unsafe extern "C" fn from_nf_ferr(mask: c_uint) -> c_int {
    static inline int from_nf_ferr(unsigned int mask)
    {
    return (mask & EMASK_M29) |		/* Bit 28 */
    (mask & ((1 << 28) - 1) << 3);	/* Bits 0 to 27 */
    };

//
// Defines to extract the various fields from the
// MTRx - Memory Technology Registers
//

// This applies to FERR_NF_FB-DIMM as well as FERR_FAT_FB-DIMM
#[no_mangle]
pub unsafe extern "C" fn extract_fbdchan_indx(x: u32) -> c_int {
    static inline int extract_fbdchan_indx(u32 x)
    {
    return (x>>28) & 0x3;
    }
// Device name and register DID (Device ID)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5400_dev_info {
    pub /: *const *const *const char ctl_name; / name for this device,
    pub /: *mut *mut u16 fsb_mapping_errors; / DID for the branchmap,control,
}

// Table of devices attributes supported by this driver
    static const struct i5400_dev_info i5400_devs[] = {
    {
    .ctl_name = "I5400",
    .fsb_mapping_errors = PCI_DEVICE_ID_INTEL_5400_ERR,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5400_dimm_info {
    pub /: *mut *mut int megabytes; / size, 0 means not present,
}

// driver private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5400_pvt {
    pub /: *mut *mut *mut pci_dev system_address; / 16.0,
    pub /: *mut *mut *mut pci_dev branchmap_werrors; / 16.1,
    pub /: *mut *mut *mut pci_dev fsb_error_regs; / 16.2,
    pub /: *mut *mut *mut pci_dev branch_0; / 21.0,
    pub /: *mut *mut *mut pci_dev branch_1; / 22.0,
    pub /: *mut *mut u16 tolm; / top of low memory,
    union {
    pub /: *mut *mut u64 ambase; / AMB BAR,
    struct {
    pub ambase_bottom: u32,
    pub ambase_top: u32,
    pub __packed: } u,
}

    u16 mir0, mir1;
    u16 b0_mtr[DIMMS_PER_CHANNEL];	/* Memory Technlogy Reg */
    u16 b0_ambpresent0;			/* Branch 0, Channel 0 */
    u16 b0_ambpresent1;			/* Brnach 0, Channel 1 */
    u16 b1_mtr[DIMMS_PER_CHANNEL];	/* Memory Technlogy Reg */
    u16 b1_ambpresent0;			/* Branch 1, Channel 8 */
    u16 b1_ambpresent1;			/* Branch 1, Channel 1 */
// DIMM information matrix, allocating architecture maximums
    struct i5400_dimm_info dimm_info[DIMMS_PER_CHANNEL][MAX_CHANNELS];
// Actual values for this controller
    int maxch;				/* Max channels */
    int maxdimmperch;			/* Max DIMMs per channel */
// Hardware error reporting status
    bool enabled_error_reporting;
    };
// I5400 MCH error information retrieved from Hardware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5400_error_info {
// These registers are always read from the MC
    pub /: *mut *mut u32 ferr_fat_fbd; / First Errors Fatal,
    pub /: *mut *mut u32 nerr_fat_fbd; / Next Errors Fatal,
    pub /: *mut *mut u32 ferr_nf_fbd; / First Errors Non-Fatal,
    pub /: *mut *mut u32 nerr_nf_fbd; / Next Errors Non-Fatal,
// These registers are input ONLY if there was a Recoverable Error
    pub /: *mut *mut u32 redmemb; / Recoverable Mem Data Error log B,
    pub /: *mut *mut u16 recmema; / Recoverable Mem Error log A,
    pub /: *mut *mut u32 recmemb; / Recoverable Mem Error log B,
// These registers are input ONLY if there was a Non-Rec Error
    pub /: *mut *mut u16 nrecmema; / Non-Recoverable Mem log A,
    pub /: *mut *mut u32 nrecmemb; / Non-Recoverable Mem log B,
}

// note that nrec_rdwr changed from NRECMEMA to NRECMEMB between the 5000 and
    5400 better to use an inline function than a macro in this case */
#[no_mangle]
pub unsafe extern "C" fn nrec_bank(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_bank(struct i5400_error_info *info)
    {
    return ((info.nrecmema) >> 12) & 0x7;
    }
#[no_mangle]
pub unsafe extern "C" fn nrec_rank(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_rank(struct i5400_error_info *info)
    {
    return ((info.nrecmema) >> 8) & 0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn nrec_buf_id(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_buf_id(struct i5400_error_info *info)
    {
    return ((info.nrecmema)) & 0xff;
    }
#[no_mangle]
pub unsafe extern "C" fn nrec_rdwr(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_rdwr(struct i5400_error_info *info)
    {
    return (info.nrecmemb) >> 31;
    }
// This applies to both NREC and REC string so it can be used with nrec_rdwr
    and rec_rdwr */
    static inline const char *rdwr_str(int rdwr)
    {
    return rdwr ? "Write" : "Read";
    }
#[no_mangle]
pub unsafe extern "C" fn nrec_cas(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_cas(struct i5400_error_info *info)
    {
    return ((info.nrecmemb) >> 16) & 0x1fff;
    }
#[no_mangle]
pub unsafe extern "C" fn nrec_ras(info: *mut i5400_error_info) -> c_int {
    static inline int nrec_ras(struct i5400_error_info *info)
    {
    return (info.nrecmemb) & 0xffff;
    }
#[no_mangle]
pub unsafe extern "C" fn rec_bank(info: *mut i5400_error_info) -> c_int {
    static inline int rec_bank(struct i5400_error_info *info)
    {
    return ((info.recmema) >> 12) & 0x7;
    }
#[no_mangle]
pub unsafe extern "C" fn rec_rank(info: *mut i5400_error_info) -> c_int {
    static inline int rec_rank(struct i5400_error_info *info)
    {
    return ((info.recmema) >> 8) & 0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn rec_rdwr(info: *mut i5400_error_info) -> c_int {
    static inline int rec_rdwr(struct i5400_error_info *info)
    {
    return (info.recmemb) >> 31;
    }
#[no_mangle]
pub unsafe extern "C" fn rec_cas(info: *mut i5400_error_info) -> c_int {
    static inline int rec_cas(struct i5400_error_info *info)
    {
    return ((info.recmemb) >> 16) & 0x1fff;
    }
#[no_mangle]
pub unsafe extern "C" fn rec_ras(info: *mut i5400_error_info) -> c_int {
    static inline int rec_ras(struct i5400_error_info *info)
    {
    return (info.recmemb) & 0xffff;
    }
    static struct edac_pci_ctl_info *i5400_pci;
//
// i5400_get_error_info	Retrieve the hardware error information from
// the hardware and cache it in the 'info'
// structure
//
    static void i5400_get_error_info(struct mem_ctl_info *mci,
    struct i5400_error_info *info)
    {
    struct i5400_pvt *pvt;
    u32 value;
    pvt = mci.pvt_info;
// read in the 1st FATAL error register
    pci_read_config_dword(pvt.branchmap_werrors, FERR_FAT_FBD, &value);
// Mask only the bits that the doc says are valid
//
    value &= (FERR_FAT_FBDCHAN | FERR_FAT_MASK);
// If there is an error, then read in the
    NEXT FATAL error register and the Memory Error Log Register A
//
    if (value & FERR_FAT_MASK) {
    info.ferr_fat_fbd = value;
// harvest the various error data we need
    pci_read_config_dword(pvt.branchmap_werrors,
    NERR_FAT_FBD, &info.nerr_fat_fbd);
    pci_read_config_word(pvt.branchmap_werrors,
    NRECMEMA, &info.nrecmema);
    pci_read_config_dword(pvt.branchmap_werrors,
    NRECMEMB, &info.nrecmemb);
// Clear the error bits, by writing them back
    pci_write_config_dword(pvt.branchmap_werrors,
    FERR_FAT_FBD, value);
    } else {
    info.ferr_fat_fbd = 0;
    info.nerr_fat_fbd = 0;
    info.nrecmema = 0;
    info.nrecmemb = 0;
    }
// read in the 1st NON-FATAL error register
    pci_read_config_dword(pvt.branchmap_werrors, FERR_NF_FBD, &value);
// If there is an error, then read in the 1st NON-FATAL error
// register as well
    if (value & FERR_NF_MASK) {
    info.ferr_nf_fbd = value;
// harvest the various error data we need
    pci_read_config_dword(pvt.branchmap_werrors,
    NERR_NF_FBD, &info.nerr_nf_fbd);
    pci_read_config_word(pvt.branchmap_werrors,
    RECMEMA, &info.recmema);
    pci_read_config_dword(pvt.branchmap_werrors,
    RECMEMB, &info.recmemb);
    pci_read_config_dword(pvt.branchmap_werrors,
    REDMEMB, &info.redmemb);
// Clear the error bits, by writing them back
    pci_write_config_dword(pvt.branchmap_werrors,
    FERR_NF_FBD, value);
    } else {
    info.ferr_nf_fbd = 0;
    info.nerr_nf_fbd = 0;
    info.recmema = 0;
    info.recmemb = 0;
    info.redmemb = 0;
    }
    }
//
// i5400_proccess_non_recoverable_info(struct mem_ctl_info *mci,
// struct i5400_error_info *info,
// int handle_errors);
//
// handle the Intel FATAL and unrecoverable errors, if any
//
    static void i5400_proccess_non_recoverable_info(struct mem_ctl_info *mci,
    struct i5400_error_info *info,
    unsigned long allErrors)
    {
    char msg[EDAC_MC_LABEL_LEN + 1 + 90 + 80];
    int branch;
    int channel;
    int bank;
    int buf_id;
    int rank;
    int rdwr;
    int ras, cas;
    int errnum;
    char *type = core::ptr::null_mut();
    let mut tp_event: enum hw_event_mc_err_type = HW_EVENT_ERR_UNCORRECTED;
    if (!allErrors)
    return;		/* if no error, return now */
    if (allErrors &  ERROR_FAT_MASK) {
    type = "FATAL";
    tp_event = HW_EVENT_ERR_FATAL;
    } else if (allErrors & FERR_NF_UNCORRECTABLE)
    type = "NON-FATAL uncorrected";
    else
    type = "NON-FATAL recoverable";
// ONLY ONE of the possible error bits will be set, as per the docs
    branch = extract_fbdchan_indx(info.ferr_fat_fbd);
    channel = branch;
// Use the NON-Recoverable macros to extract data
    bank = nrec_bank(info);
    rank = nrec_rank(info);
    buf_id = nrec_buf_id(info);
    rdwr = nrec_rdwr(info);
    ras = nrec_ras(info);
    cas = nrec_cas(info);
    edac_dbg(0, "\t\t%s DIMM= %d  Channels= %d,%d  (Branch= %d DRAM Bank= %d Buffer ID = %d rdwr= %s ras= %d cas= %d)\n",
    type, rank, channel, channel + 1, branch >> 1, bank,
    buf_id, rdwr_str(rdwr), ras, cas);
// Only 1 bit will be on
    errnum = find_first_bit(&allErrors, ARRAY_SIZE(error_name));
// Form out message
    snprintf(msg, sizeof(msg),
    "Bank=%d Buffer ID = %d RAS=%d CAS=%d Err=0x%lx (%s)",
    bank, buf_id, ras, cas, allErrors, error_name[errnum]);
    edac_mc_handle_error(tp_event, mci, 1, 0, 0, 0,
    branch >> 1, -1, rank,
    rdwr ? "Write error" : "Read error",
    msg);
    }
//
// i5400_process_fatal_error_info(struct mem_ctl_info *mci,
// struct i5400_error_info *info,
// int handle_errors);
//
// handle the Intel NON-FATAL errors, if any
//
    static void i5400_process_nonfatal_error_info(struct mem_ctl_info *mci,
    struct i5400_error_info *info)
    {
    char msg[EDAC_MC_LABEL_LEN + 1 + 90 + 80];
    unsigned long allErrors;
    int branch;
    int channel;
    int bank;
    int rank;
    int rdwr;
    int ras, cas;
    int errnum;
// mask off the Error bits that are possible
    allErrors = from_nf_ferr(info.ferr_nf_fbd & FERR_NF_MASK);
    if (!allErrors)
    return;		/* if no error, return now */
// ONLY ONE of the possible error bits will be set, as per the docs
    if (allErrors & (ERROR_NF_UNCORRECTABLE | ERROR_NF_RECOVERABLE)) {
    i5400_proccess_non_recoverable_info(mci, info, allErrors);
    return;
    }
// Correctable errors
    if (allErrors & ERROR_NF_CORRECTABLE) {
    edac_dbg(0, "\tCorrected bits= 0x%lx\n", allErrors);
    branch = extract_fbdchan_indx(info.ferr_nf_fbd);
    channel = 0;
    if (REC_ECC_LOCATOR_ODD(info.redmemb))
    channel = 1;
// Convert channel to be based from zero, instead of
// from branch base of 0
    channel += branch;
    bank = rec_bank(info);
    rank = rec_rank(info);
    rdwr = rec_rdwr(info);
    ras = rec_ras(info);
    cas = rec_cas(info);
// Only 1 bit will be on
    errnum = find_first_bit(&allErrors, ARRAY_SIZE(error_name));
    edac_dbg(0, "\t\tDIMM= %d Channel= %d  (Branch %d DRAM Bank= %d rdwr= %s ras= %d cas= %d)\n",
    rank, channel, branch >> 1, bank,
    rdwr_str(rdwr), ras, cas);
// Form out message
    snprintf(msg, sizeof(msg),
    "Corrected error (Branch=%d DRAM-Bank=%d RDWR=%s "
    "RAS=%d CAS=%d, CE Err=0x%lx (%s))",
    branch >> 1, bank, rdwr_str(rdwr), ras, cas,
    allErrors, error_name[errnum]);
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0,
    branch >> 1, channel % 2, rank,
    rdwr ? "Write error" : "Read error",
    msg);
    return;
    }
// Miscellaneous errors
    errnum = find_first_bit(&allErrors, ARRAY_SIZE(error_name));
    branch = extract_fbdchan_indx(info.ferr_nf_fbd);
    i5400_mc_printk(mci, KERN_EMERG,
    "Non-Fatal misc error (Branch=%d Err=%#lx (%s))",
    branch >> 1, allErrors, error_name[errnum]);
    }
//
// i5400_process_error_info	Process the error info that is
// in the 'info' structure, previously retrieved from hardware
//
    static void i5400_process_error_info(struct mem_ctl_info *mci,
    struct i5400_error_info *info)
    {	u32 allErrors;
// First handle any fatal errors that occurred
    allErrors = (info.ferr_fat_fbd & FERR_FAT_MASK);
    i5400_proccess_non_recoverable_info(mci, info, allErrors);
// now handle any non-fatal errors that occurred
    i5400_process_nonfatal_error_info(mci, info);
    }
//
// i5400_clear_error	Retrieve any error from the hardware
// but do NOT process that error.
// Used for 'clearing' out of previous errors
// Called by the Core module.
//
#[no_mangle]
unsafe extern "C" fn i5400_clear_error(mci: *mut mem_ctl_info) {
    static void i5400_clear_error(struct mem_ctl_info *mci)
    {
    struct i5400_error_info info;
    i5400_get_error_info(mci, &info);
    }
//
// i5400_check_error	Retrieve and process errors reported by the
// hardware. Called by the Core module.
//
#[no_mangle]
unsafe extern "C" fn i5400_check_error(mci: *mut mem_ctl_info) {
    static void i5400_check_error(struct mem_ctl_info *mci)
    {
    struct i5400_error_info info;
    i5400_get_error_info(mci, &info);
    i5400_process_error_info(mci, &info);
    }
//
// i5400_put_devices	'put' all the devices that we have
// reserved via 'get'
//
#[no_mangle]
unsafe extern "C" fn i5400_put_devices(mci: *mut mem_ctl_info) {
    static void i5400_put_devices(struct mem_ctl_info *mci)
    {
    struct i5400_pvt *pvt;
    pvt = mci.pvt_info;
// Decrement usage count for devices
    pci_dev_put(pvt.branch_1);
    pci_dev_put(pvt.branch_0);
    pci_dev_put(pvt.fsb_error_regs);
    pci_dev_put(pvt.branchmap_werrors);
    }
//
// i5400_get_devices	Find and perform 'get' operation on the MCH's
// device/functions we want to reference for this driver
//
// Need to 'get' device 16 func 1 and func 2
//
#[no_mangle]
unsafe extern "C" fn i5400_get_devices(mci: *mut mem_ctl_info, dev_idx: c_int) -> c_int {
    static int i5400_get_devices(struct mem_ctl_info *mci, int dev_idx)
    {
    struct i5400_pvt *pvt;
    struct pci_dev *pdev;
    pvt = mci.pvt_info;
    pvt.branchmap_werrors = core::ptr::null_mut();
    pvt.fsb_error_regs = core::ptr::null_mut();
    pvt.branch_0 = core::ptr::null_mut();
    pvt.branch_1 = core::ptr::null_mut();
// Attempt to 'get' the MCH register we want
    pdev = core::ptr::null_mut();
    while (1) {
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_ERR, pdev);
    if (!pdev) {
// End of list, leave
    i5400_printk(KERN_ERR,
    "'system address,Process Bus' "
    "device not found:"
    "vendor 0x%x device 0x%x ERR func 1 "
    "(broken BIOS?)\n",
    PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_ERR);
    return -ENODEV;
    }
// Store device 16 func 1
    if (PCI_FUNC(pdev.devfn) == 1)
    break;
    }
    pvt.branchmap_werrors = pdev;
    pdev = core::ptr::null_mut();
    while (1) {
    pdev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_ERR, pdev);
    if (!pdev) {
// End of list, leave
    i5400_printk(KERN_ERR,
    "'system address,Process Bus' "
    "device not found:"
    "vendor 0x%x device 0x%x ERR func 2 "
    "(broken BIOS?)\n",
    PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_ERR);
    pci_dev_put(pvt.branchmap_werrors);
    return -ENODEV;
    }
// Store device 16 func 2
    if (PCI_FUNC(pdev.devfn) == 2)
    break;
    }
    pvt.fsb_error_regs = pdev;
    edac_dbg(1, "System Address, processor bus- PCI Bus ID: %s  %x:%x\n",
    pci_name(pvt.system_address),
    pvt.system_address.vendor, pvt.system_address.device);
    edac_dbg(1, "Branchmap, control and errors - PCI Bus ID: %s  %x:%x\n",
    pci_name(pvt.branchmap_werrors),
    pvt.branchmap_werrors.vendor,
    pvt.branchmap_werrors.device);
    edac_dbg(1, "FSB Error Regs - PCI Bus ID: %s  %x:%x\n",
    pci_name(pvt.fsb_error_regs),
    pvt.fsb_error_regs.vendor, pvt.fsb_error_regs.device);
    pvt.branch_0 = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_FBD0, core::ptr::null_mut());
    if (!pvt.branch_0) {
    i5400_printk(KERN_ERR,
    "MC: 'BRANCH 0' device not found:"
    "vendor 0x%x device 0x%x Func 0 (broken BIOS?)\n",
    PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_5400_FBD0);
    pci_dev_put(pvt.fsb_error_regs);
    pci_dev_put(pvt.branchmap_werrors);
    return -ENODEV;
    }
// If this device claims to have more than 2 channels then
// fetch Branch 1's information
//
    if (pvt.maxch < CHANNELS_PER_BRANCH)
    return 0;
    pvt.branch_1 = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_FBD1, core::ptr::null_mut());
    if (!pvt.branch_1) {
    i5400_printk(KERN_ERR,
    "MC: 'BRANCH 1' device not found:"
    "vendor 0x%x device 0x%x Func 0 "
    "(broken BIOS?)\n",
    PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_5400_FBD1);
    pci_dev_put(pvt.branch_0);
    pci_dev_put(pvt.fsb_error_regs);
    pci_dev_put(pvt.branchmap_werrors);
    return -ENODEV;
    }
    return 0;
    }
//
// determine_amb_present
//
// the information is contained in DIMMS_PER_CHANNEL different
// registers determining which of the DIMMS_PER_CHANNEL requires
// knowing which channel is in question
//
// 2 branches, each with 2 channels
// b0_ambpresent0 for channel '0'
// b0_ambpresent1 for channel '1'
// b1_ambpresent0 for channel '2'
// b1_ambpresent1 for channel '3'
//
#[no_mangle]
unsafe extern "C" fn determine_amb_present_reg(pvt: *mut i5400_pvt, channel: c_int) -> c_int {
    static int determine_amb_present_reg(struct i5400_pvt *pvt, int channel)
    {
    int amb_present;
    if (channel < CHANNELS_PER_BRANCH) {
    if (channel & 0x1)
    amb_present = pvt.b0_ambpresent1;
    else
    amb_present = pvt.b0_ambpresent0;
    } else {
    if (channel & 0x1)
    amb_present = pvt.b1_ambpresent1;
    else
    amb_present = pvt.b1_ambpresent0;
    }
    return amb_present;
    }
//
// determine_mtr(pvt, dimm, channel)
//
// return the proper MTR register as determine by the dimm and desired channel
//
#[no_mangle]
unsafe extern "C" fn determine_mtr(pvt: *mut i5400_pvt, dimm: c_int, channel: c_int) -> c_int {
    static int determine_mtr(struct i5400_pvt *pvt, int dimm, int channel)
    {
    int mtr;
    int n;
// There is one MTR for each slot pair of FB-DIMMs,
    Each slot pair may be at branch 0 or branch 1.
//
    n = dimm;
    if (n >= DIMMS_PER_CHANNEL) {
    edac_dbg(0, "ERROR: trying to access an invalid dimm: %d\n",
    dimm);
    return 0;
    }
    if (channel < CHANNELS_PER_BRANCH)
    mtr = pvt.b0_mtr[n];
    else
    mtr = pvt.b1_mtr[n];
    return mtr;
    }
//
#[no_mangle]
unsafe extern "C" fn decode_mtr(slot_row: c_int, mtr: u16) {
    static void decode_mtr(int slot_row, u16 mtr)
    {
    int ans;
    ans = MTR_DIMMS_PRESENT(mtr);
    edac_dbg(2, "\tMTR%d=0x%x:  DIMMs are %sPresent\n",
    slot_row, mtr, ans ? "" : "NOT ");
    if (!ans)
    return;
    edac_dbg(2, "\t\tWIDTH: x%d\n", MTR_DRAM_WIDTH(mtr));
    edac_dbg(2, "\t\tELECTRICAL THROTTLING is %s\n",
    str_enabled_disabled(MTR_DIMMS_ETHROTTLE(mtr)));
    edac_dbg(2, "\t\tNUMBANK: %d bank(s)\n", MTR_DRAM_BANKS(mtr));
    edac_dbg(2, "\t\tNUMRANK: %s\n",
    MTR_DIMM_RANK(mtr) ? "double" : "single");
    edac_dbg(2, "\t\tNUMROW: %s\n",
    MTR_DIMM_ROWS(mtr) == 0 ? "8,192 - 13 rows" :
    MTR_DIMM_ROWS(mtr) == 1 ? "16,384 - 14 rows" :
    MTR_DIMM_ROWS(mtr) == 2 ? "32,768 - 15 rows" :
    "65,536 - 16 rows");
    edac_dbg(2, "\t\tNUMCOL: %s\n",
    MTR_DIMM_COLS(mtr) == 0 ? "1,024 - 10 columns" :
    MTR_DIMM_COLS(mtr) == 1 ? "2,048 - 11 columns" :
    MTR_DIMM_COLS(mtr) == 2 ? "4,096 - 12 columns" :
    "reserved");
    }
    static void handle_channel(struct i5400_pvt *pvt, int dimm, int channel,
    struct i5400_dimm_info *dinfo)
    {
    int mtr;
    int amb_present_reg;
    int addrBits;
    mtr = determine_mtr(pvt, dimm, channel);
    if (MTR_DIMMS_PRESENT(mtr)) {
    amb_present_reg = determine_amb_present_reg(pvt, channel);
// Determine if there is a DIMM present in this DIMM slot
    if (amb_present_reg & (1 << dimm)) {
// Start with the number of bits for a Bank
// on the DRAM
    addrBits = MTR_DRAM_BANKS_ADDR_BITS(mtr);
// Add thenumber of ROW bits
    addrBits += MTR_DIMM_ROWS_ADDR_BITS(mtr);
// add the number of COLUMN bits
    addrBits += MTR_DIMM_COLS_ADDR_BITS(mtr);
// add the number of RANK bits
    addrBits += MTR_DIMM_RANK(mtr);
    addrBits += 6;	/* add 64 bits per DIMM */
    addrBits -= 20;	/* divide by 2^^20 */
    addrBits -= 3;	/* 8 bits per bytes */
    dinfo.megabytes = 1 << addrBits;
    }
    }
    }
//
// calculate_dimm_size
//
// also will output a DIMM matrix map, if debug is enabled, for viewing
// how the DIMMs are populated
//
#[no_mangle]
unsafe extern "C" fn calculate_dimm_size(pvt: *mut i5400_pvt) {
    static void calculate_dimm_size(struct i5400_pvt *pvt)
    {
    struct i5400_dimm_info *dinfo;
    int dimm, max_dimms;
    char *p, *mem_buffer;
    int space, n;
    int channel, branch;
// ================= Generate some debug output =================
    space = PAGE_SIZE;
    mem_buffer = p = kmalloc(space, GFP_KERNEL);
    if (p == core::ptr::null_mut()) {
    i5400_printk(KERN_ERR, "MC: %s:%s() kmalloc() failed\n",
    __FILE__, __func__);
    return;
    }
// Scan all the actual DIMMS
// and calculate the information for each DIMM
// Start with the highest dimm first, to display it first
// and work toward the 0th dimm
//
    max_dimms = pvt.maxdimmperch;
    for (dimm = max_dimms - 1; dimm >= 0; dimm--) {
// on an odd dimm, first output a 'boundary' marker,
// then reset the message buffer
    if (dimm & 0x1) {
    n = snprintf(p, space, "---------------------------"
    "-------------------------------");
    p += n;
    space -= n;
    edac_dbg(2, "%s\n", mem_buffer);
    p = mem_buffer;
    space = PAGE_SIZE;
    }
    n = snprintf(p, space, "dimm %2d    ", dimm);
    p += n;
    space -= n;
    for (channel = 0; channel < pvt.maxch; channel++) {
    dinfo = &pvt.dimm_info[dimm][channel];
    handle_channel(pvt, dimm, channel, dinfo);
    n = snprintf(p, space, "%4d MB   | ", dinfo.megabytes);
    p += n;
    space -= n;
    }
    edac_dbg(2, "%s\n", mem_buffer);
    p = mem_buffer;
    space = PAGE_SIZE;
    }
// Output the last bottom 'boundary' marker
    n = snprintf(p, space, "---------------------------"
    "-------------------------------");
    p += n;
    space -= n;
    edac_dbg(2, "%s\n", mem_buffer);
    p = mem_buffer;
    space = PAGE_SIZE;
// now output the 'channel' labels
    n = snprintf(p, space, "           ");
    p += n;
    space -= n;
    for (channel = 0; channel < pvt.maxch; channel++) {
    n = snprintf(p, space, "channel %d | ", channel);
    p += n;
    space -= n;
    }
    edac_dbg(2, "%s\n", mem_buffer);
    p = mem_buffer;
    space = PAGE_SIZE;
    n = snprintf(p, space, "           ");
    p += n;
    space -= n;
    for (branch = 0; branch < MAX_BRANCHES; branch++) {
    n = snprintf(p, space, "       branch %d       | ", branch);
    p += n;
    space -= n;
    }
// output the last message and free buffer
    edac_dbg(2, "%s\n", mem_buffer);
    kfree(mem_buffer);
    }
//
// i5400_get_mc_regs	read in the necessary registers and
// cache locally
//
// Fills in the private data members
//
#[no_mangle]
unsafe extern "C" fn i5400_get_mc_regs(mci: *mut mem_ctl_info) {
    static void i5400_get_mc_regs(struct mem_ctl_info *mci)
    {
    struct i5400_pvt *pvt;
    u32 actual_tolm;
    u16 limit;
    int slot_row;
    int way0, way1;
    pvt = mci.pvt_info;
    pci_read_config_dword(pvt.system_address, AMBASE,
    &pvt.u.ambase_bottom);
    pci_read_config_dword(pvt.system_address, AMBASE + sizeof(u32),
    &pvt.u.ambase_top);
    edac_dbg(2, "AMBASE= 0x%lx  MAXCH= %d  MAX-DIMM-Per-CH= %d\n",
    (long unsigned int)pvt.ambase, pvt.maxch, pvt.maxdimmperch);
// Get the Branch Map regs
    pci_read_config_word(pvt.branchmap_werrors, TOLM, &pvt.tolm);
    pvt.tolm >>= 12;
    edac_dbg(2, "\nTOLM (number of 256M regions) =%u (0x%x)\n",
    pvt.tolm, pvt.tolm);
    actual_tolm = (u32) ((1000l * pvt.tolm) >> (30 - 28));
    edac_dbg(2, "Actual TOLM byte addr=%u.%03u GB (0x%x)\n",
    actual_tolm/1000, actual_tolm % 1000, pvt.tolm << 28);
    pci_read_config_word(pvt.branchmap_werrors, MIR0, &pvt.mir0);
    pci_read_config_word(pvt.branchmap_werrors, MIR1, &pvt.mir1);
// Get the MIR[0-1] regs
    limit = (pvt.mir0 >> 4) & 0x0fff;
    way0 = pvt.mir0 & 0x1;
    way1 = pvt.mir0 & 0x2;
    edac_dbg(2, "MIR0: limit= 0x%x  WAY1= %u  WAY0= %x\n",
    limit, way1, way0);
    limit = (pvt.mir1 >> 4) & 0xfff;
    way0 = pvt.mir1 & 0x1;
    way1 = pvt.mir1 & 0x2;
    edac_dbg(2, "MIR1: limit= 0x%x  WAY1= %u  WAY0= %x\n",
    limit, way1, way0);
// Get the set of MTR[0-3] regs by each branch
    for (slot_row = 0; slot_row < DIMMS_PER_CHANNEL; slot_row++) {
    let mut where: c_int = MTR0 + (slot_row * sizeof(u16));
// Branch 0 set of MTR registers
    pci_read_config_word(pvt.branch_0, where,
    &pvt.b0_mtr[slot_row]);
    edac_dbg(2, "MTR%d where=0x%x B0 value=0x%x\n",
    slot_row, where, pvt.b0_mtr[slot_row]);
    if (pvt.maxch < CHANNELS_PER_BRANCH) {
    pvt.b1_mtr[slot_row] = 0;
    continue;
    }
// Branch 1 set of MTR registers
    pci_read_config_word(pvt.branch_1, where,
    &pvt.b1_mtr[slot_row]);
    edac_dbg(2, "MTR%d where=0x%x B1 value=0x%x\n",
    slot_row, where, pvt.b1_mtr[slot_row]);
    }
// Read and dump branch 0's MTRs
    edac_dbg(2, "Memory Technology Registers:\n");
    edac_dbg(2, "   Branch 0:\n");
    for (slot_row = 0; slot_row < DIMMS_PER_CHANNEL; slot_row++)
    decode_mtr(slot_row, pvt.b0_mtr[slot_row]);
    pci_read_config_word(pvt.branch_0, AMBPRESENT_0,
    &pvt.b0_ambpresent0);
    edac_dbg(2, "\t\tAMB-Branch 0-present0 0x%x:\n", pvt.b0_ambpresent0);
    pci_read_config_word(pvt.branch_0, AMBPRESENT_1,
    &pvt.b0_ambpresent1);
    edac_dbg(2, "\t\tAMB-Branch 0-present1 0x%x:\n", pvt.b0_ambpresent1);
// Only if we have 2 branchs (4 channels)
    if (pvt.maxch < CHANNELS_PER_BRANCH) {
    pvt.b1_ambpresent0 = 0;
    pvt.b1_ambpresent1 = 0;
    } else {
// Read and dump  branch 1's MTRs
    edac_dbg(2, "   Branch 1:\n");
    for (slot_row = 0; slot_row < DIMMS_PER_CHANNEL; slot_row++)
    decode_mtr(slot_row, pvt.b1_mtr[slot_row]);
    pci_read_config_word(pvt.branch_1, AMBPRESENT_0,
    &pvt.b1_ambpresent0);
    edac_dbg(2, "\t\tAMB-Branch 1-present0 0x%x:\n",
    pvt.b1_ambpresent0);
    pci_read_config_word(pvt.branch_1, AMBPRESENT_1,
    &pvt.b1_ambpresent1);
    edac_dbg(2, "\t\tAMB-Branch 1-present1 0x%x:\n",
    pvt.b1_ambpresent1);
    }
// Go and determine the size of each DIMM and place in an
// orderly matrix
    calculate_dimm_size(pvt);
    }
//
// i5400_init_dimms	Initialize the 'dimms' table within
// the mci control	structure with the
// addressing of memory.
//
// return:
// 0	success
// 1	no actual memory found on this MC
//
#[no_mangle]
unsafe extern "C" fn i5400_init_dimms(mci: *mut mem_ctl_info) -> c_int {
    static int i5400_init_dimms(struct mem_ctl_info *mci)
    {
    struct i5400_pvt *pvt;
    struct dimm_info *dimm;
    int ndimms;
    int mtr;
    int size_mb;
    int  channel, slot;
    pvt = mci.pvt_info;
    ndimms = 0;
//
// FIXME: remove  pvt->dimm_info[slot][channel] and use the 3
// layers here.
//
    for (channel = 0; channel < mci.layers[0].size * mci.layers[1].size;
    channel++) {
    for (slot = 0; slot < mci.layers[2].size; slot++) {
    mtr = determine_mtr(pvt, slot, channel);
// if no DIMMS on this slot, continue
    if (!MTR_DIMMS_PRESENT(mtr))
    continue;
    dimm = edac_get_dimm(mci, channel / 2, channel % 2, slot);
    size_mb =  pvt.dimm_info[slot][channel].megabytes;
    edac_dbg(2, "dimm (branch %d channel %d slot %d): %d.%03d GB\n",
    channel / 2, channel % 2, slot,
    size_mb / 1000, size_mb % 1000);
    dimm.nr_pages = size_mb << 8;
    dimm.grain = 8;
    dimm.dtype = MTR_DRAM_WIDTH(mtr) == 8 ?
    DEV_X8 : DEV_X4;
    dimm.mtype = MEM_FB_DDR2;
//
// The eccc mechanism is SDDC (aka SECC), with
// is similar to Chipkill.
//
    dimm.edac_mode = MTR_DRAM_WIDTH(mtr) == 8 ?
    EDAC_S8ECD8ED : EDAC_S4ECD4ED;
    ndimms++;
    }
    }
//
// When just one memory is provided, it should be at location (0,0,0).
// With such single-DIMM mode, the SDCC algorithm degrades to SECDEC+.
//
    if (ndimms == 1)
    mci.dimms[0].edac_mode = EDAC_SECDED;
    return (ndimms == 0);
    }
//
// i5400_set_error_reporting
// Turn on/off the memory reporting features of the hardware
//
#[no_mangle]
unsafe extern "C" fn i5400_set_error_reporting(mci: *mut mem_ctl_info, enable: bool) {
    static void i5400_set_error_reporting(struct mem_ctl_info *mci, bool enable)
    {
    struct i5400_pvt *pvt;
    u32 fbd_error_mask;
    pvt = mci.pvt_info;
// Read the FBD Error Mask Register
    pci_read_config_dword(pvt.branchmap_werrors, EMASK_FBD,
    &fbd_error_mask);
// Enable with 0, disable with 1
    if (enable)
    fbd_error_mask &= ~(ENABLE_EMASK_ALL);
    else
    fbd_error_mask |= ENABLE_EMASK_ALL;
    pci_write_config_dword(pvt.branchmap_werrors, EMASK_FBD,
    fbd_error_mask);
    }
//
// i5400_probe1	Probe for ONE instance of device to see if it is
// present.
// return:
// 0 for FOUND a device
// < 0 for error code
//
#[no_mangle]
unsafe extern "C" fn i5400_probe1(pdev: *mut pci_dev, dev_idx: c_int) -> c_int {
    static int i5400_probe1(struct pci_dev *pdev, int dev_idx)
    {
    struct mem_ctl_info *mci;
    struct i5400_pvt *pvt;
    struct edac_mc_layer layers[3];
    if (dev_idx >= ARRAY_SIZE(i5400_devs))
    return -EINVAL;
    edac_dbg(0, "MC: pdev bus %u dev=0x%x fn=0x%x\n",
    pdev.bus.number,
    PCI_SLOT(pdev.devfn), PCI_FUNC(pdev.devfn));
// We only are looking for func 0 of the set
    if (PCI_FUNC(pdev.devfn) != 0)
    return -ENODEV;
//
// allocate a new MC control structure
//
// This drivers uses the DIMM slot as "csrow" and the rest as "channel".
//
    layers[0].type = EDAC_MC_LAYER_BRANCH;
    layers[0].size = MAX_BRANCHES;
    layers[0].is_virt_csrow = false;
    layers[1].type = EDAC_MC_LAYER_CHANNEL;
    layers[1].size = CHANNELS_PER_BRANCH;
    layers[1].is_virt_csrow = false;
    layers[2].type = EDAC_MC_LAYER_SLOT;
    layers[2].size = DIMMS_PER_CHANNEL;
    layers[2].is_virt_csrow = true;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers, sizeof(*pvt));
    if (mci == core::ptr::null_mut())
    return -ENOMEM;
    edac_dbg(0, "MC: mci = %p\n", mci);
    mci.pdev = &pdev.dev;	/* record ptr  to the generic device */
    pvt = mci.pvt_info;
    pvt.system_address = pdev;	/* Record this device in our private */
    pvt.maxch = MAX_CHANNELS;
    pvt.maxdimmperch = DIMMS_PER_CHANNEL;
// 'get' the pci devices we want to reserve for our use
    if (i5400_get_devices(mci, dev_idx))
    goto fail0;
// Time to get serious
    i5400_get_mc_regs(mci);	/* retrieve the hardware registers */
    mci.mc_idx = 0;
    mci.mtype_cap = MEM_FLAG_FB_DDR2;
    mci.edac_ctl_cap = EDAC_FLAG_NONE;
    mci.edac_cap = EDAC_FLAG_NONE;
    mci.mod_name = "i5400_edac.c";
    mci.ctl_name = i5400_devs[dev_idx].ctl_name;
    mci.dev_name = pci_name(pdev);
    mci.ctl_page_to_phys = core::ptr::null_mut();
// Set the function pointer to an actual operation function
    mci.edac_check = i5400_check_error;
// initialize the MC control structure 'dimms' table
// with the mapping and control information
    if (i5400_init_dimms(mci)) {
    edac_dbg(0, "MC: Setting mci.edac_cap to EDAC_FLAG_NONE because i5400_init_dimms() returned nonzero value\n");
    mci.edac_cap = EDAC_FLAG_NONE;	/* no dimms found */
    pvt.enabled_error_reporting = false;
    } else {
    edac_dbg(1, "MC: Enable error reporting now\n");
    i5400_set_error_reporting(mci, true);
    pvt.enabled_error_reporting = true;
    }
// add this new MC control structure to EDAC's list of MCs
    if (edac_mc_add_mc(mci)) {
    edac_dbg(0, "MC: failed edac_mc_add_mc()\n");
// Disable error reporting if we just enabled it
    if (pvt.enabled_error_reporting)
    i5400_set_error_reporting(mci, false);
    goto fail1;
    }
    i5400_clear_error(mci);
// allocating generic PCI control info
    i5400_pci = edac_pci_create_generic_ctl(&pdev.dev, EDAC_MOD_STR);
    if (!i5400_pci) {
    printk(KERN_WARNING
    "%s(): Unable to create PCI control\n",
    __func__);
    printk(KERN_WARNING
    "%s(): PCI error report via EDAC not setup\n",
    __func__);
    }
    return 0;
// Error exit unwinding stack
    fail1:
    i5400_put_devices(mci);
    fail0:
    edac_mc_free(mci);
    return -ENODEV;
    }
//
// i5400_init_one	constructor for one instance of device
//
// returns:
// negative on error
// count (>= 0)
//
#[no_mangle]
unsafe extern "C" fn i5400_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int i5400_init_one(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    int rc;
    edac_dbg(0, "MC:\n");
// wake up device
    rc = pci_enable_device(pdev);
    if (rc)
    return rc;
// now probe and enable the device
    return i5400_probe1(pdev, id.driver_data);
    }
//
// i5400_remove_one	destructor for one instance of device
//
#[no_mangle]
unsafe extern "C" fn i5400_remove_one(pdev: *mut pci_dev) {
    static void i5400_remove_one(struct pci_dev *pdev)
    {
    struct mem_ctl_info *mci;
    struct i5400_pvt *pvt;
    edac_dbg(0, "\n");
    if (i5400_pci)
    edac_pci_release_generic_ctl(i5400_pci);
    mci = edac_mc_del_mc(&pdev.dev);
    if (!mci)
    return;
    pvt = mci.pvt_info;
// Disable error reporting on teardown
    if (pvt.enabled_error_reporting)
    i5400_set_error_reporting(mci, false);
// retrieve references to resources, and free those resources
    i5400_put_devices(mci);
    pci_disable_device(pdev);
    edac_mc_free(mci);
    }
//
// pci_device_id	table for which devices we are looking for
//
// The "E500P" device is the first device supported.
//
    static const struct pci_device_id i5400_pci_tbl[] = {
    {PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_5400_ERR)},
    {0,}			/* 0 terminated list. */
    };
    MODULE_DEVICE_TABLE(pci, i5400_pci_tbl);
//
// i5400_driver	pci_driver structure for this module
//
    static struct pci_driver i5400_driver = {
    .name = "i5400_edac",
    .probe = i5400_init_one,
    .remove = i5400_remove_one,
    .id_table = i5400_pci_tbl,
    };
//
// i5400_init		Module entry function
// Try to initialize this module for its devices
//
#[no_mangle]
unsafe extern "C" fn i5400_init() -> int __init {
    static int __init i5400_init(void)
    {
    int pci_rc;
    edac_dbg(2, "MC:\n");
// Ensure that the OPSTATE is set correctly for POLL or NMI
    opstate_init();
    pci_rc = pci_register_driver(&i5400_driver);
    return (pci_rc < 0) ? pci_rc : 0;
    }
//
// i5400_exit()	Module exit function
// Unregister the driver
//
#[no_mangle]
unsafe extern "C" fn i5400_exit() -> void __exit {
    static void __exit i5400_exit(void)
    {
    edac_dbg(2, "MC:\n");
    pci_unregister_driver(&i5400_driver);
    }
    module_init(i5400_init);
    module_exit(i5400_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ben Woodard <woodard@redhat.com>");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_AUTHOR("Red Hat Inc. (https://www.redhat.com)");
    MODULE_DESCRIPTION("MC Driver for Intel I5400 memory controllers - "
    I5400_REVISION);
    module_param(edac_op_state, int, 0444);
    MODULE_PARM_DESC(edac_op_state, "EDAC Error Reporting state: 0=Poll,1=NMI");
