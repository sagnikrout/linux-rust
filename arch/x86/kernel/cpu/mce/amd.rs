//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/mce/amd.c
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
// (c) 2005-2016 Advanced Micro Devices, Inc.
//
// Written by Jacob Shin - AMD, Inc.
// Maintained by: Borislav Petkov <bp@alien8.de>
//

pub const NR_BLOCKS: c_int = 5;
pub const THRESHOLD_MAX: c_uint = 0xFFF;
pub const INT_TYPE_APIC: c_uint = 0x00020000;
pub const MASK_VALID_HI: c_uint = 0x80000000;
pub const MASK_CNTP_HI: c_uint = 0x40000000;
pub const MASK_LOCKED_HI: c_uint = 0x20000000;
pub const MASK_LVTOFF_HI: c_uint = 0x00F00000;
pub const MASK_COUNT_EN_HI: c_uint = 0x00080000;
pub const MASK_INT_TYPE_HI: c_uint = 0x00060000;
pub const MASK_OVERFLOW_HI: c_uint = 0x00010000;
pub const MASK_ERR_COUNT_HI: c_uint = 0x00000FFF;
pub const MASK_BLKPTR_LO: c_uint = 0xFF000000;
pub const MCG_XBLK_ADDR: c_uint = 0xC0000400;
// Deferred error settings
pub const MSR_CU_DEF_ERR: c_uint = 0xC0000410;
pub const MASK_DEF_LVTOFF: c_uint = 0x000000F0;
// Scalable MCA:
// Threshold LVT offset is at MSR0xC0000410[15:12]
pub const SMCA_THR_LVT_OFF: c_uint = 0xF000;
    static bool thresholding_irq_en;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_amd_cpu_data {
    pub thr_intr_banks: mce_banks_t,
    pub dfr_intr_banks: mce_banks_t,
    u32		thr_intr_en: 1,
    dfr_intr_en: 1,
    pub 30: __resv:,
}

    static DEFINE_PER_CPU_READ_MOSTLY(struct mce_amd_cpu_data, mce_amd_data);
    static const char * const th_names[] = {
    "load_store",
    "insn_fetch",
    "combined_unit",
    "decode_unit",
    "northbridge",
    "execution_unit",
    };
    static const char * const smca_umc_block_names[] = {
    "dram_ecc",
    "misc_umc"
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smca_hwid {
    pub /: *mut *mut unsigned int bank_type; / Use with smca_bank_types for easy indexing.,
    pub /: *mut *mut u32 hwid_mcatype; / (hwid,mcatype) tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smca_bank {
    pub hwid: *const smca_hwid,
    pub /: *mut *mut u32 id; / Value of MCA_IPID[InstanceId].,
    pub /: *mut *mut u8 sysfs_id; / Value used for sysfs name.,
    u64 paddrv	:1,	/* Physical Address Valid bit in MCA_CONFIG */
    pub :63: __reserved,
}

    static DEFINE_PER_CPU_READ_MOSTLY(struct smca_bank[MAX_NR_BANKS], smca_banks);
    static DEFINE_PER_CPU_READ_MOSTLY(u8[N_SMCA_BANK_TYPES], smca_bank_counts);
    static const char * const smca_names[] = {
    [SMCA_CS ... SMCA_CS_V2]	= "coherent_station",
    [SMCA_DACC_BE]			= "dacc_be",
    [SMCA_DACC_FE]			= "dacc_fe",
    [SMCA_DE]			= "decode_unit",
    [SMCA_EDDR5CMN]			= "eddr5_cmn",
    [SMCA_EX]			= "execution_unit",
    [SMCA_FP]			= "floating_point",
    [SMCA_GMI_PCS]			= "gmi_pcs",
    [SMCA_GMI_PHY]			= "gmi_phy",
    [SMCA_IF]			= "insn_fetch",
    [SMCA_L2_CACHE]			= "l2_cache",
    [SMCA_L3_CACHE]			= "l3_cache",
    [SMCA_LS ... SMCA_LS_V2]	= "load_store",
    [SMCA_MA_LLC]			= "ma_llc",
    [SMCA_MP5]			= "mp5",
    [SMCA_MPART]			= "mpart",
    [SMCA_MPASP ... SMCA_MPASP_V2]	= "mpasp",
    [SMCA_MPDACC]			= "mpdacc",
    [SMCA_MPDMA]			= "mpdma",
    [SMCA_MPM]			= "mpm",
    [SMCA_MPRAS]			= "mpras",
    [SMCA_NBIF]			= "nbif",
    [SMCA_NBIO]			= "nbio",
    [SMCA_PB]			= "param_block",
    [SMCA_PCIE ... SMCA_PCIE_V2]	= "pcie",
    [SMCA_PCIE_PL]			= "pcie_pl",
    [SMCA_PIE]			= "pie",
    [SMCA_PSP ... SMCA_PSP_V2]	= "psp",
    [SMCA_RESERVED]			= "reserved",
    [SMCA_SATA]			= "sata",
    [SMCA_SHUB]			= "shub",
    [SMCA_SMU ... SMCA_SMU_V2]	= "smu",
    [SMCA_SSBDCI]			= "ssbdci",
// UMC v2 is separate because both of them can exist in a single system.
    [SMCA_UMC]			= "umc",
    [SMCA_UMC_V2]			= "umc_v2",
    [SMCA_USB]			= "usb",
    [SMCA_USR_CP]			= "usr_cp",
    [SMCA_USR_DP]			= "usr_dp",
    [SMCA_WAFL_PHY]			= "wafl_phy",
    [SMCA_XGMI_PCS]			= "xgmi_pcs",
    [SMCA_XGMI_PHY]			= "xgmi_phy",
    };
    static const char *smca_get_name(enum smca_bank_types t)
    {
    if (t >= N_SMCA_BANK_TYPES)
    return core::ptr::null_mut();
    return smca_names[t];
    }
#[no_mangle]
pub unsafe extern "C" fn smca_get_bank_type(cpu: c_uint, bank: c_uint) -> enum smca_bank_types {
    enum smca_bank_types smca_get_bank_type(unsigned int cpu, unsigned int bank)
    {
    struct smca_bank *b;
    if (bank >= MAX_NR_BANKS)
    return N_SMCA_BANK_TYPES;
    b = &per_cpu(smca_banks, cpu)[bank];
    if (!b.hwid)
    return N_SMCA_BANK_TYPES;
    return b.hwid.bank_type;
    }
    EXPORT_SYMBOL_GPL(smca_get_bank_type);
//
// Format:
// { bank_type, hwid_mcatype }
//
// alphanumerically sorted by bank type.
//
    static const struct smca_hwid smca_hwid_mcatypes[] = {
    { SMCA_CS,	 HWID_MCATYPE(0x2E, 0x0)	},
    { SMCA_CS_V2,	 HWID_MCATYPE(0x2E, 0x2)	},
    { SMCA_DACC_BE,	 HWID_MCATYPE(0x164, 0x0)	},
    { SMCA_DACC_FE,	 HWID_MCATYPE(0x157, 0x0)	},
    { SMCA_DE,	 HWID_MCATYPE(0xB0, 0x3)	},
    { SMCA_EDDR5CMN, HWID_MCATYPE(0x1E0, 0x0)	},
    { SMCA_EX,	 HWID_MCATYPE(0xB0, 0x5)	},
    { SMCA_FP,	 HWID_MCATYPE(0xB0, 0x6)	},
    { SMCA_GMI_PCS,  HWID_MCATYPE(0x241, 0x0)	},
    { SMCA_GMI_PHY,	 HWID_MCATYPE(0x269, 0x0)	},
    { SMCA_IF,	 HWID_MCATYPE(0xB0, 0x1)	},
    { SMCA_L2_CACHE, HWID_MCATYPE(0xB0, 0x2)	},
    { SMCA_L3_CACHE, HWID_MCATYPE(0xB0, 0x7)	},
    { SMCA_LS,	 HWID_MCATYPE(0xB0, 0x0)	},
    { SMCA_LS_V2,	 HWID_MCATYPE(0xB0, 0x10)	},
    { SMCA_MA_LLC,	 HWID_MCATYPE(0x2E, 0x4)	},
    { SMCA_MP5,	 HWID_MCATYPE(0x01, 0x2)	},
    { SMCA_MPART,	 HWID_MCATYPE(0xFF, 0x2)	},
    { SMCA_MPASP,	 HWID_MCATYPE(0xFD, 0x0)	},
    { SMCA_MPASP_V2, HWID_MCATYPE(0xFD, 0x1)	},
    { SMCA_MPDACC,	 HWID_MCATYPE(0xBE, 0x0)	},
    { SMCA_MPDMA,	 HWID_MCATYPE(0x01, 0x3)	},
    { SMCA_MPM,	 HWID_MCATYPE(0xF9, 0x0)	},
    { SMCA_MPRAS,	 HWID_MCATYPE(0x12, 0x0)	},
    { SMCA_NBIF,	 HWID_MCATYPE(0x6C, 0x0)	},
    { SMCA_NBIO,	 HWID_MCATYPE(0x18, 0x0)	},
    { SMCA_PB,	 HWID_MCATYPE(0x05, 0x0)	},
    { SMCA_PCIE,	 HWID_MCATYPE(0x46, 0x0)	},
    { SMCA_PCIE_V2,	 HWID_MCATYPE(0x46, 0x1)	},
    { SMCA_PCIE_PL,	 HWID_MCATYPE(0x1E1, 0x0)	},
    { SMCA_PIE,	 HWID_MCATYPE(0x2E, 0x1)	},
    { SMCA_PSP,	 HWID_MCATYPE(0xFF, 0x0)	},
    { SMCA_PSP_V2,	 HWID_MCATYPE(0xFF, 0x1)	},
    { SMCA_RESERVED, HWID_MCATYPE(0x00, 0x0)	},
    { SMCA_SATA,	 HWID_MCATYPE(0xA8, 0x0)	},
    { SMCA_SHUB,	 HWID_MCATYPE(0x80, 0x0)	},
    { SMCA_SMU,	 HWID_MCATYPE(0x01, 0x0)	},
    { SMCA_SMU_V2,	 HWID_MCATYPE(0x01, 0x1)	},
    { SMCA_SSBDCI,	 HWID_MCATYPE(0x5C, 0x0)	},
    { SMCA_UMC,	 HWID_MCATYPE(0x96, 0x0)	},
    { SMCA_UMC_V2,	 HWID_MCATYPE(0x96, 0x1)	},
    { SMCA_USB,	 HWID_MCATYPE(0xAA, 0x0)	},
    { SMCA_USR_CP,	 HWID_MCATYPE(0x180, 0x0)	},
    { SMCA_USR_DP,	 HWID_MCATYPE(0x170, 0x0)	},
    { SMCA_WAFL_PHY, HWID_MCATYPE(0x267, 0x0)	},
    { SMCA_XGMI_PCS, HWID_MCATYPE(0x50, 0x0)	},
    { SMCA_XGMI_PHY, HWID_MCATYPE(0x259, 0x0)	},
    };
//
// In SMCA enabled processors, we can have multiple banks for a given IP type.
// So to define a unique name for each bank, we use a temp c-string to append
// the MCA_IPID[InstanceId] to type's name in get_name().
//
// InstanceId is 32 bits which is 8 characters. Make sure MAX_MCATYPE_NAME_LEN
// is greater than 8 plus 1 (for underscore) plus length of longest type name.
//
pub const MAX_MCATYPE_NAME_LEN: c_int = 30;
    static char buf_mcatype[MAX_MCATYPE_NAME_LEN];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct threshold_block {
// This block's number within its bank.
    pub block: c_uint,
// MCA bank number that contains this block.
    pub bank: c_uint,
// CPU which controls this block's MCA bank.
    pub cpu: c_uint,
// MCA_MISC MSR address for this block.
    pub address: u32,
// Enable/Disable APIC interrupt.
    pub interrupt_enable: bool,
// Bank can generate an interrupt.
    pub interrupt_capable: bool,
// Value upon which threshold interrupt is generated.
    pub threshold_limit: u16,
// sysfs object
    pub kobj: kobject,
// List of threshold blocks within this block's MCA bank.
    pub miscj: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct threshold_bank {
    pub kobj: *mut kobject,
// List of threshold blocks within this MCA bank.
    pub miscj: list_head,
}

    static DEFINE_PER_CPU(struct threshold_bank **, threshold_banks);
//
// A list of the banks enabled on each logical CPU. Controls which respective
// descriptors to initialize later in mce_threshold_create_device().
//
    static DEFINE_PER_CPU(u64, bank_map);
    static void amd_threshold_interrupt(void);
    static void amd_deferred_error_interrupt(void);
#[no_mangle]
unsafe extern "C" fn default_deferred_error_interrupt() {
    static void default_deferred_error_interrupt(void)
    {
    pr_err("Unexpected deferred interrupt at vector %x\n", DEFERRED_ERROR_VECTOR);
    }
    void (*deferred_error_int_vector)(void) = default_deferred_error_interrupt;
#[no_mangle]
unsafe extern "C" fn smca_configure(bank: c_uint, cpu: c_uint) {
    static void smca_configure(unsigned int bank, unsigned int cpu)
    {
    struct mce_amd_cpu_data *data = this_cpu_ptr(&mce_amd_data);
    u8 *bank_counts = this_cpu_ptr(smca_bank_counts);
    const struct smca_hwid *s_hwid;
    unsigned int i, hwid_mcatype;
    struct msr val;
    let mut smca_config: u32 = MSR_AMD64_SMCA_MCx_CONFIG(bank);
// Set appropriate bits in MCA_CONFIG
    if (!rdmsrq_safe(smca_config, &val.q)) {
//
// OS is required to set the MCAX bit to acknowledge that it is
// now using the new MSR ranges and new registers under each
// bank. It also means that the OS will configure deferred
// errors in the new MCx_CONFIG register. If the bit is not set,
// uncorrectable errors will cause a system panic.
//
// MCA_CONFIG[MCAX] is bit 32 (0 in the high portion of the MSR.)
//
    val.h |= BIT(0);
//
// SMCA sets the Deferred Error Interrupt type per bank.
//
// MCA_CONFIG[DeferredIntTypeSupported] is bit 5, and tells us
// if the DeferredIntType bit field is available.
//
// MCA_CONFIG[DeferredIntType] is bits [38:37] ([6:5] in the
// high portion of the MSR). OS should set this to 0x1 to enable
// APIC based interrupt. First, check that no interrupt has been
// set.
//
    if ((val.l & BIT(5)) && !((val.h >> 5) & 0x3) && data.dfr_intr_en) {
    __set_bit(bank, data.dfr_intr_banks);
    val.h |= BIT(5);
    }
//
// SMCA Corrected Error Interrupt
//
// MCA_CONFIG[IntPresent] is bit 10, and tells us if the bank can
// send an MCA Thresholding interrupt without the OS initializing
// this feature. This can be used if the threshold limit is managed
// by the platform.
//
// MCA_CONFIG[IntEn] is bit 40 (8 in the high portion of the MSR).
// The OS should set this to inform the platform that the OS is ready
// to handle the MCA Thresholding interrupt.
//
    if ((val.l & BIT(10)) && data.thr_intr_en) {
    __set_bit(bank, data.thr_intr_banks);
    val.h |= BIT(8);
    }
    this_cpu_ptr(mce_banks_array)[bank].lsb_in_status = !!(val.l & BIT(8));
    if (val.l & MCI_CONFIG_PADDRV)
    this_cpu_ptr(smca_banks)[bank].paddrv = 1;
    wrmsrq(smca_config, val.q);
    }
    if (rdmsrq_safe(MSR_AMD64_SMCA_MCx_IPID(bank), &val.q)) {
    pr_warn("Failed to read MCA_IPID for bank %d\n", bank);
    return;
    }
    hwid_mcatype = HWID_MCATYPE(val.h & MCI_IPID_HWID,
    (val.h & MCI_IPID_MCATYPE) >> 16);
    for (i = 0; i < ARRAY_SIZE(smca_hwid_mcatypes); i++) {
    s_hwid = &smca_hwid_mcatypes[i];
    if (hwid_mcatype == s_hwid.hwid_mcatype) {
    this_cpu_ptr(smca_banks)[bank].hwid = s_hwid;
    this_cpu_ptr(smca_banks)[bank].id = val.l;
    this_cpu_ptr(smca_banks)[bank].sysfs_id = bank_counts[s_hwid.bank_type]++;
    break;
    }
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thresh_restart {
    pub b: *mut threshold_block,
    pub set_lvt_off: c_int,
    pub lvt_off: c_int,
    pub old_limit: u16,
}

    static const char *bank4_names(const struct threshold_block *b)
    {
    switch (b.address) {
// MSR4_MISC0
    case 0x00000413:
    return "dram";
    case 0xc0000408:
    return "ht_links";
    case 0xc0000409:
    return "l3_cache";
    default:
    WARN(1, "Funny MSR: 0x%08x\n", b.address);
    return "";
    }
    };
#[no_mangle]
unsafe extern "C" fn lvt_interrupt_supported(bank: c_uint, msr_high_bits: u32) -> bool {
    static bool lvt_interrupt_supported(unsigned int bank, u32 msr_high_bits)
    {
//
// bank 4 supports APIC LVT interrupts implicitly since forever.
//
    if (bank == 4)
    return true;
//
// IntP: interrupt present; if this bit is set, the thresholding
// bank can generate APIC LVT interrupts
//
    return msr_high_bits & BIT(28);
    }
#[no_mangle]
unsafe extern "C" fn lvt_off_valid(b: *mut threshold_block, apic: c_int, lo: u32, hi: u32) -> bool {
    static bool lvt_off_valid(struct threshold_block *b, int apic, u32 lo, u32 hi)
    {
    let mut msr: c_int = (hi & MASK_LVTOFF_HI) >> 20;
//
// On SMCA CPUs, LVT offset is programmed at a different MSR, and
// the BIOS provides the value. The original field where LVT offset
// was set is reserved. Return early here:
//
    if (mce_flags.smca)
    return false;
    if (apic < 0) {
    pr_err(FW_BUG "cpu %d, failed to setup threshold interrupt "
    "for bank %d, block %d (MSR%08X=0x%x%08x)\n", b.cpu,
    b.bank, b.block, b.address, hi, lo);
    return false;
    }
    if (apic != msr) {
    pr_err(FW_BUG "cpu %d, invalid threshold interrupt offset %d "
    "for bank %d, block %d (MSR%08X=0x%x%08x)\n",
    b.cpu, apic, b.bank, b.block, b.address, hi, lo);
    return false;
    }
    return true;
    };
// Reprogram MCx_MISC MSR behind this threshold block.
#[no_mangle]
unsafe extern "C" fn threshold_restart_block(_tr: *mut c_void) {
    static void threshold_restart_block(void *_tr)
    {
    struct thresh_restart *tr = _tr;
    struct msr val;
// sysfs write might race against an offline operation
    if (!this_cpu_read(threshold_banks) && !tr.set_lvt_off)
    return;
    rdmsrq(tr.b.address, val.q);
//
// Reset error count and overflow bit.
// This is done during init or after handling an interrupt.
//
    if (val.h & MASK_OVERFLOW_HI || tr.set_lvt_off) {
    val.h &= ~(MASK_ERR_COUNT_HI | MASK_OVERFLOW_HI);
    val.h |= THRESHOLD_MAX - tr.b.threshold_limit;
    } else if (tr.old_limit) {	/* change limit w/o reset */
    int new_count = (val.h & THRESHOLD_MAX) +
    (tr.old_limit - tr.b.threshold_limit);
    val.h = (val.h & ~MASK_ERR_COUNT_HI) |
    (new_count & THRESHOLD_MAX);
    }
// clear IntType
    val.h &= ~MASK_INT_TYPE_HI;
    if (!tr.b.interrupt_capable)
    goto done;
    if (tr.set_lvt_off) {
    if (lvt_off_valid(tr.b, tr.lvt_off, val.l, val.h)) {
// set new lvt offset
    val.h &= ~MASK_LVTOFF_HI;
    val.h |= tr.lvt_off << 20;
    }
    }
    if (tr.b.interrupt_enable)
    val.h |= INT_TYPE_APIC;
    done:
    val.h |= MASK_COUNT_EN_HI;
    wrmsrq(tr.b.address, val.q);
    }
#[no_mangle]
unsafe extern "C" fn threshold_restart_bank(bank: c_uint, intr_en: bool) {
    static void threshold_restart_bank(unsigned int bank, bool intr_en)
    {
    struct threshold_bank **thr_banks = this_cpu_read(threshold_banks);
    struct threshold_block *block, *tmp;
    struct thresh_restart tr;
    if (!thr_banks || !thr_banks[bank])
    return;
    memset(&tr, 0, sizeof(tr));
    list_for_each_entry_safe(block, tmp, &thr_banks[bank].miscj, miscj) {
    tr.b = block;
    tr.b.interrupt_enable = intr_en;
    threshold_restart_block(&tr);
    }
    }
// Try to use the threshold limit reported through APEI.
#[no_mangle]
unsafe extern "C" fn get_thr_limit() -> u16 {
    static u16 get_thr_limit(void)
    {
    let mut thr_limit: u32 = mce_get_apei_thr_limit();
// Fallback to old default if APEI limit is not available.
    if (!thr_limit)
    return THRESHOLD_MAX;
    return min(thr_limit, THRESHOLD_MAX);
    }
#[no_mangle]
unsafe extern "C" fn mce_threshold_block_init(b: *mut threshold_block, offset: c_int) {
    static void mce_threshold_block_init(struct threshold_block *b, int offset)
    {
    struct thresh_restart tr = {
    .b			= b,
    .set_lvt_off		= 1,
    .lvt_off		= offset,
    };
    b.threshold_limit		= get_thr_limit();
    threshold_restart_block(&tr);
    };
#[no_mangle]
unsafe extern "C" fn setup_APIC_mce_threshold(reserved: c_int, new: c_int) -> c_int {
    static int setup_APIC_mce_threshold(int reserved, int new)
    {
    if (reserved < 0 && !setup_APIC_eilvt(new, THRESHOLD_APIC_VECTOR,
    APIC_DELIVERY_MODE_FIXED, 0))
    return new;
    return reserved;
    }
    static u32 get_block_address(u32 current_addr, u32 low, u32 high,
    unsigned int bank, unsigned int block,
    unsigned int cpu)
    {
    let mut addr: u32 = 0, offset = 0;
    if ((bank >= per_cpu(mce_num_banks, cpu)) || (block >= NR_BLOCKS))
    return addr;
    if (mce_flags.smca) {
    if (!block)
    return MSR_AMD64_SMCA_MCx_MISC(bank);
    if (!(low & MASK_BLKPTR_LO))
    return 0;
    return MSR_AMD64_SMCA_MCx_MISCy(bank, block - 1);
    }
// Fall back to method we used for older processors:
    switch (block) {
    case 0:
    addr = mca_msr_reg(bank, MCA_MISC);
    break;
    case 1:
    offset = ((low & MASK_BLKPTR_LO) >> 21);
    if (offset)
    addr = MCG_XBLK_ADDR + offset;
    break;
    default:
    addr = ++current_addr;
    }
    return addr;
    }
    static int prepare_threshold_block(unsigned int bank, unsigned int block, u32 addr,
    int offset, u32 misc_high)
    {
    let mut cpu: c_uint = smp_processor_id();
    struct threshold_block b;
    int new;
    if (!block)
    per_cpu(bank_map, cpu) |= BIT_ULL(bank);
    memset(&b, 0, sizeof(b));
    b.cpu			= cpu;
    b.bank			= bank;
    b.block			= block;
    b.address		= addr;
    b.interrupt_capable	= lvt_interrupt_supported(bank, misc_high);
    if (!b.interrupt_capable)
    goto done;
    __set_bit(bank, this_cpu_ptr(&mce_amd_data).thr_intr_banks);
    b.interrupt_enable = 1;
    if (mce_flags.smca)
    goto done;
    new = (misc_high & MASK_LVTOFF_HI) >> 20;
    offset = setup_APIC_mce_threshold(offset, new);
    if (offset == new)
    thresholding_irq_en = true;
    done:
    mce_threshold_block_init(&b, offset);
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_filter_mce(m: *mut mce) -> bool {
    bool amd_filter_mce(struct mce *m)
    {
    let mut bank_type: enum smca_bank_types = smca_get_bank_type(m.extcpu, m.bank);
    struct cpuinfo_x86 *c = &boot_cpu_data;
// Bogus hw errors on Cezanne A0.
    if (c.x86 == 0x19 &&
    c.x86_model == 0x50 &&
    c.x86_stepping == 0x0) {
    if (!(m.status & MCI_STATUS_EN))
    return true;
    }
// See Family 17h Models 10h-2Fh Erratum #1114.
    if (c.x86 == 0x17 &&
    c.x86_model >= 0x10 && c.x86_model <= 0x2F &&
    bank_type == SMCA_IF && XEC(m.status, 0x3f) == 10)
    return true;
// NB GART TLB error reporting is disabled by default.
    if (c.x86 < 0x17) {
    if (m.bank == 4 && XEC(m.status, 0x1f) == 0x5)
    return true;
    }
    return false;
    }
//
// Turn off thresholding banks for the following conditions:
// - MC4_MISC thresholding is not supported on Family 0x15.
// - Prevent possible spurious interrupts from the IF bank on Family 0x17
// Models 0x10-0x2F due to Erratum #1114.
//
#[no_mangle]
unsafe extern "C" fn disable_err_thresholding(c: *mut cpuinfo_x86, bank: c_uint) {
    static void disable_err_thresholding(struct cpuinfo_x86 *c, unsigned int bank)
    {
    int i, num_msrs;
    u64 hwcr;
    bool need_toggle;
    u32 msrs[NR_BLOCKS];
    if (c.x86 == 0x15 && bank == 4) {
    msrs[0] = 0x00000413; /* MC4_MISC0 */
    msrs[1] = 0xc0000408; /* MC4_MISC1 */
    num_msrs = 2;
    } else if (c.x86 == 0x17 &&
    (c.x86_model >= 0x10 && c.x86_model <= 0x2F)) {
    if (smca_get_bank_type(smp_processor_id(), bank) != SMCA_IF)
    return;
    msrs[0] = MSR_AMD64_SMCA_MCx_MISC(bank);
    num_msrs = 1;
    } else {
    return;
    }
    rdmsrq(MSR_K7_HWCR, hwcr);
// McStatusWrEn has to be set
    need_toggle = !(hwcr & BIT(18));
    if (need_toggle)
    wrmsrq(MSR_K7_HWCR, hwcr | BIT(18));
// Clear CntP bit safely
    for (i = 0; i < num_msrs; i++)
    msr_clear_bit(msrs[i], 62);
// restore old settings
    if (need_toggle)
    wrmsrq(MSR_K7_HWCR, hwcr);
    }
#[no_mangle]
unsafe extern "C" fn amd_apply_cpu_quirks(c: *mut cpuinfo_x86) {
    static void amd_apply_cpu_quirks(struct cpuinfo_x86 *c)
    {
    struct mce_bank *mce_banks = this_cpu_ptr(mce_banks_array);
// This should be disabled by the BIOS, but isn't always
    if (c.x86 == 15 && this_cpu_read(mce_num_banks) > 4) {
//
// disable GART TBL walk error reporting, which
// trips off incorrectly with the IOMMU & 3ware
// & Cerberus:
//
    clear_bit(10, (unsigned long *)&mce_banks[4].ctl);
    }
//
// Various K7s with broken bank 0 around. Always disable
// by default.
//
    if (c.x86 == 6 && this_cpu_read(mce_num_banks))
    mce_banks[0].ctl = 0;
    }
//
// Enable the APIC LVT interrupt vectors once per-CPU. This should be done before hardware is
// ready to send interrupts.
//
// Individual error sources are enabled later during per-bank init.
//
#[no_mangle]
unsafe extern "C" fn smca_enable_interrupt_vectors() {
    static void smca_enable_interrupt_vectors(void)
    {
    struct mce_amd_cpu_data *data = this_cpu_ptr(&mce_amd_data);
    u64 mca_intr_cfg, offset;
    if (!mce_flags.smca || !mce_flags.succor)
    return;
    if (rdmsrq_safe(MSR_CU_DEF_ERR, &mca_intr_cfg))
    return;
    offset = (mca_intr_cfg & SMCA_THR_LVT_OFF) >> 12;
    if (!setup_APIC_eilvt(offset, THRESHOLD_APIC_VECTOR, APIC_DELIVERY_MODE_FIXED, 0))
    data.thr_intr_en = 1;
    offset = (mca_intr_cfg & MASK_DEF_LVTOFF) >> 4;
    if (!setup_APIC_eilvt(offset, DEFERRED_ERROR_VECTOR, APIC_DELIVERY_MODE_FIXED, 0))
    data.dfr_intr_en = 1;
    }
// cpu init entry point, called from mce.c with preempt off
#[no_mangle]
pub unsafe extern "C" fn mce_amd_feature_init(c: *mut cpuinfo_x86) {
    void mce_amd_feature_init(struct cpuinfo_x86 *c)
    {
    unsigned int bank, block, cpu = smp_processor_id();
    let mut val: msr = { .q = 0 };
    let mut address: u32 = 0;
    let mut offset: c_int = -1;
    amd_apply_cpu_quirks(c);
    mce_flags.amd_threshold	 = 1;
    smca_enable_interrupt_vectors();
    for (bank = 0; bank < this_cpu_read(mce_num_banks); ++bank) {
    if (mce_flags.smca) {
    smca_configure(bank, cpu);
    if (!this_cpu_ptr(&mce_amd_data).thr_intr_en)
    continue;
    }
    disable_err_thresholding(c, bank);
    for (block = 0; block < NR_BLOCKS; ++block) {
    address = get_block_address(address, val.l, val.h, bank, block, cpu);
    if (!address)
    break;
    if (rdmsrq_safe(address, &val.q))
    break;
    if (!(val.h & MASK_VALID_HI))
    continue;
    if (!(val.h & MASK_CNTP_HI)  ||
    (val.h & MASK_LOCKED_HI))
    continue;
    offset = prepare_threshold_block(bank, block, address, offset, val.h);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn smca_bsp_init() {
    void smca_bsp_init(void)
    {
    mce_threshold_vector	  = amd_threshold_interrupt;
    deferred_error_int_vector = amd_deferred_error_interrupt;
    }
//
// DRAM ECC errors are reported in the Northbridge (bank 4) with
// Extended Error Code 8.
//
#[no_mangle]
unsafe extern "C" fn legacy_mce_is_memory_error(m: *mut mce) -> bool {
    static bool legacy_mce_is_memory_error(struct mce *m)
    {
    return m.bank == 4 && XEC(m.status, 0x1f) == 8;
    }
//
// DRAM ECC errors are reported in Unified Memory Controllers with
// Extended Error Code 0.
//
#[no_mangle]
unsafe extern "C" fn smca_mce_is_memory_error(m: *mut mce) -> bool {
    static bool smca_mce_is_memory_error(struct mce *m)
    {
    enum smca_bank_types bank_type;
    if (XEC(m.status, 0x3f))
    return false;
    bank_type = smca_get_bank_type(m.extcpu, m.bank);
    let mut bank_type: return = = SMCA_UMC || bank_type == SMCA_UMC_V2;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_mce_is_memory_error(m: *mut mce) -> bool {
    bool amd_mce_is_memory_error(struct mce *m)
    {
    if (mce_flags.smca)
    return smca_mce_is_memory_error(m);
    else
    return legacy_mce_is_memory_error(m);
    }
//
// Some AMD systems have an explicit indicator that the value in MCA_ADDR is a
// system physical address. Individual cases though, need to be detected for
// other systems. Future cases will be added as needed.
//
// 1) General case
// a) Assume address is not usable.
// 2) Poison errors
// a) Indicated by MCA_STATUS[43]: poison. Defined for all banks except legacy
// northbridge (bank 4).
// b) Refers to poison consumption in the core. Does not include "no action",
// "action optional", or "deferred" error severities.
// c) Will include a usable address so that immediate action can be taken.
// 3) Northbridge DRAM ECC errors
// a) Reported in legacy bank 4 with extended error code (XEC) 8.
// b) MCA_STATUS[43] is *not* defined as poison in legacy bank 4. Therefore,
// this bit should not be checked.
// 4) MCI_STATUS_PADDRVAL is set
// a) Will provide a valid system physical address.
//
// NOTE: SMCA UMC memory errors fall into case #1.
//
#[no_mangle]
pub unsafe extern "C" fn amd_mce_usable_address(m: *mut mce) -> bool {
    bool amd_mce_usable_address(struct mce *m)
    {
// Check special northbridge case 3) first.
    if (!mce_flags.smca) {
    if (legacy_mce_is_memory_error(m))
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(4: m->bank ==) -> else {
    else if (m.bank == 4)
    return false;
    }
    if (this_cpu_ptr(smca_banks)[m.bank].paddrv)
    return m.status & MCI_STATUS_PADDRV;
// Check poison bit for all other bank types.
    if (m.status & MCI_STATUS_POISON)
    return true;
// Assume address is not usable for all others.
    return false;
    }
    DEFINE_IDTENTRY_SYSVEC(sysvec_deferred_error)
    {
    trace_deferred_error_apic_entry(DEFERRED_ERROR_VECTOR);
    inc_irq_stat(DEFERRED_ERROR);
    deferred_error_int_vector();
    trace_deferred_error_apic_exit(DEFERRED_ERROR_VECTOR);
    apic_eoi();
    }
// APIC interrupt handler for deferred errors
#[no_mangle]
unsafe extern "C" fn amd_deferred_error_interrupt() {
    static void amd_deferred_error_interrupt(void)
    {
    machine_check_poll(MCP_TIMESTAMP, &this_cpu_ptr(&mce_amd_data).dfr_intr_banks);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_amd_handle_storm(bank: c_uint, on: bool) {
    void mce_amd_handle_storm(unsigned int bank, bool on)
    {
    threshold_restart_bank(bank, on);
    }
#[no_mangle]
unsafe extern "C" fn amd_reset_thr_limit(bank: c_uint) {
    static void amd_reset_thr_limit(unsigned int bank)
    {
    threshold_restart_bank(bank, true);
    }
//
// Threshold interrupt handler will service THRESHOLD_APIC_VECTOR. The interrupt
// goes off when error_count reaches threshold_limit.
//
#[no_mangle]
unsafe extern "C" fn amd_threshold_interrupt() {
    static void amd_threshold_interrupt(void)
    {
    machine_check_poll(MCP_TIMESTAMP, &this_cpu_ptr(&mce_amd_data).thr_intr_banks);
    }
#[no_mangle]
pub unsafe extern "C" fn amd_clear_bank(m: *mut mce) {
    void amd_clear_bank(struct mce *m)
    {
    amd_reset_thr_limit(m.bank);
    if (mce_flags.smca) {
//
// Clear MCA_DESTAT for all deferred errors even those
// logged in MCA_STATUS.
//
    if (m.status & MCI_STATUS_DEFERRED)
    mce_wrmsrq(MSR_AMD64_SMCA_MCx_DESTAT(m.bank), 0);
// Don't clear MCA_STATUS if MCA_DESTAT was used exclusively.
    if (m.kflags & MCE_CHECK_DFR_REGS)
    return;
    }
    mce_wrmsrq(mca_msr_reg(m.bank, MCA_STATUS), 0);
    }
//
// Sysfs Interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct threshold_attr {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show) (struct threshold_block , char,
    pub count): *const *const *const *const ssize_t (store) (struct threshold_block , char , size_t,
}

    static ssize_t show_ ## name(struct threshold_block *b, char *buf)	\
    {									\
    return sprintf(buf, "%lu\n", (unsigned long) b.name);		\
    }
    SHOW_FIELDS(interrupt_enable)
    SHOW_FIELDS(threshold_limit)
    static ssize_t
    store_interrupt_enable(struct threshold_block *b, const char *buf, size_t size)
    {
    struct thresh_restart tr;
    unsigned long new;
    if (!b.interrupt_capable)
    return -EINVAL;
    if (kstrtoul(buf, 0, &new) < 0)
    return -EINVAL;
    b.interrupt_enable = !!new;
    memset(&tr, 0, sizeof(tr));
    tr.b		= b;
    if (smp_call_function_single(b.cpu, threshold_restart_block, &tr, 1))
    return -ENODEV;
    return size;
    }
    static ssize_t
    store_threshold_limit(struct threshold_block *b, const char *buf, size_t size)
    {
    struct thresh_restart tr;
    unsigned long new;
    if (kstrtoul(buf, 0, &new) < 0)
    return -EINVAL;
    if (new > THRESHOLD_MAX)
    new = THRESHOLD_MAX;
    if (new < 1)
    new = 1;
    memset(&tr, 0, sizeof(tr));
    tr.old_limit = b.threshold_limit;
    b.threshold_limit = new;
    tr.b = b;
    if (smp_call_function_single(b.cpu, threshold_restart_block, &tr, 1))
    return -ENODEV;
    return size;
    }
#[no_mangle]
unsafe extern "C" fn show_error_count(b: *mut threshold_block, buf: *mut c_char) -> isize {
    static ssize_t show_error_count(struct threshold_block *b, char *buf)
    {
    struct msr val;
// CPU might be offline by now
    if (rdmsrq_on_cpu(b.cpu, b.address, &val.q))
    return -ENODEV;
    return sprintf(buf, "%u\n", ((val.h & THRESHOLD_MAX) -
    (THRESHOLD_MAX - b.threshold_limit)));
    }
    static struct threshold_attr error_count = {
    .attr = {.name = __stringify(error_count), .mode = 0444 },
    .show = show_error_count,
    };

    static struct threshold_attr val = {					\
    .attr	= {.name = __stringify(val), .mode = 0644 },		\
    .show	= show_## val,						\
    .store	= store_## val,						\
    };
    RW_ATTR(interrupt_enable);
    RW_ATTR(threshold_limit);
    static struct attribute *default_attrs[] = {
    &threshold_limit.attr,
    &error_count.attr,
    core::ptr::null_mut(),	/* possibly interrupt_enable if supported, see below */
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(default);

#[no_mangle]
unsafe extern "C" fn show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> isize {
    static ssize_t show(struct kobject *kobj, struct attribute *attr, char *buf)
    {
    struct threshold_block *b = to_block(kobj);
    struct threshold_attr *a = to_attr(attr);
    ssize_t ret;
    ret = a.show ? a.show(b, buf) : -EIO;
    return ret;
    }
    static ssize_t store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t count)
    {
    struct threshold_block *b = to_block(kobj);
    struct threshold_attr *a = to_attr(attr);
    ssize_t ret;
    ret = a.store ? a.store(b, buf, count) : -EIO;
    return ret;
    }
    static const struct sysfs_ops threshold_ops = {
    .show			= show,
    .store			= store,
    };
    static void threshold_block_release(struct kobject *kobj);
    static const struct kobj_type threshold_ktype = {
    .sysfs_ops		= &threshold_ops,
    .default_groups		= default_groups,
    .release		= threshold_block_release,
    };
    static const char *get_name(unsigned int cpu, unsigned int bank, struct threshold_block *b)
    {
    enum smca_bank_types bank_type;
    if (!mce_flags.smca) {
    if (b && bank == 4)
    return bank4_names(b);
    return th_names[bank];
    }
    bank_type = smca_get_bank_type(cpu, bank);
    if (b && (bank_type == SMCA_UMC || bank_type == SMCA_UMC_V2)) {
    if (b.block < ARRAY_SIZE(smca_umc_block_names))
    return smca_umc_block_names[b.block];
    }
    if (b && b.block) {
    snprintf(buf_mcatype, MAX_MCATYPE_NAME_LEN, "th_block_%u", b.block);
    return buf_mcatype;
    }
    if (bank_type >= N_SMCA_BANK_TYPES) {
    snprintf(buf_mcatype, MAX_MCATYPE_NAME_LEN, "th_bank_%u", bank);
    return buf_mcatype;
    }
    if (per_cpu(smca_bank_counts, cpu)[bank_type] == 1)
    return smca_get_name(bank_type);
    snprintf(buf_mcatype, MAX_MCATYPE_NAME_LEN,
    "%s_%u", smca_get_name(bank_type),
    per_cpu(smca_banks, cpu)[bank].sysfs_id);
    return buf_mcatype;
    }
    static int allocate_threshold_blocks(unsigned int cpu, struct threshold_bank *tb,
    unsigned int bank, unsigned int block,
    u32 address)
    {
    struct threshold_block *b = core::ptr::null_mut();
    struct msr val;
    int err;
    if ((bank >= this_cpu_read(mce_num_banks)) || (block >= NR_BLOCKS))
    return 0;
    if (rdmsrq_safe(address, &val.q))
    return 0;
    if (!(val.h & MASK_VALID_HI)) {
    if (block)
    goto recurse;
    else
    return 0;
    }
    if (!(val.h & MASK_CNTP_HI)  ||
    (val.h & MASK_LOCKED_HI))
    goto recurse;
    b = kzalloc_obj(struct threshold_block);
    if (!b)
    return -ENOMEM;
    b.block		= block;
    b.bank			= bank;
    b.cpu			= cpu;
    b.address		= address;
    b.interrupt_enable	= 0;
    b.interrupt_capable	= lvt_interrupt_supported(bank, val.h);
    b.threshold_limit	= get_thr_limit();
    if (b.interrupt_capable) {
    default_attrs[2] = &interrupt_enable.attr;
    b.interrupt_enable = 1;
    } else {
    default_attrs[2] = core::ptr::null_mut();
    }
    list_add(&b.miscj, &tb.miscj);
    mce_threshold_block_init(b, (val.h & MASK_LVTOFF_HI) >> 20);
    err = kobject_init_and_add(&b.kobj, &threshold_ktype, tb.kobj, get_name(cpu, bank, b));
    if (err)
    goto out_free;
    recurse:
    address = get_block_address(address, val.l, val.h, bank, ++block, cpu);
    if (!address)
    return 0;
    err = allocate_threshold_blocks(cpu, tb, bank, block, address);
    if (err)
    goto out_free;
    if (b)
    kobject_uevent(&b.kobj, KOBJ_ADD);
    return 0;
    out_free:
    if (b) {
    list_del(&b.miscj);
    kobject_put(&b.kobj);
    }
    return err;
    }
    static int threshold_create_bank(struct threshold_bank **bp, unsigned int cpu,
    unsigned int bank)
    {
    struct device *dev = this_cpu_read(mce_device);
    struct threshold_bank *b = core::ptr::null_mut();
    const char *name = get_name(cpu, bank, core::ptr::null_mut());
    let mut err: c_int = 0;
    if (!dev)
    return -ENODEV;
    b = kzalloc_obj(struct threshold_bank);
    if (!b) {
    err = -ENOMEM;
    goto out;
    }
// Associate the bank with the per-CPU MCE device
    b.kobj = kobject_create_and_add(name, &dev.kobj);
    if (!b.kobj) {
    err = -EINVAL;
    goto out_free;
    }
    INIT_LIST_HEAD(&b.miscj);
    err = allocate_threshold_blocks(cpu, b, bank, 0, mca_msr_reg(bank, MCA_MISC));
    if (err)
    goto out_kobj;
    bp[bank] = b;
    return 0;
    out_kobj:
    kobject_put(b.kobj);
    out_free:
    kfree(b);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn threshold_block_release(kobj: *mut kobject) {
    static void threshold_block_release(struct kobject *kobj)
    {
    kfree(to_block(kobj));
    }
#[no_mangle]
unsafe extern "C" fn threshold_remove_bank(bank: *mut threshold_bank) {
    static void threshold_remove_bank(struct threshold_bank *bank)
    {
    struct threshold_block *pos, *tmp;
    list_for_each_entry_safe(pos, tmp, &bank.miscj, miscj) {
    list_del(&pos.miscj);
    kobject_put(&pos.kobj);
    }
    kobject_put(bank.kobj);
    kfree(bank);
    }
#[no_mangle]
unsafe extern "C" fn __threshold_remove_device(bp: *mut threshold_bank) {
    static void __threshold_remove_device(struct threshold_bank **bp)
    {
    unsigned int bank, numbanks = this_cpu_read(mce_num_banks);
    for (bank = 0; bank < numbanks; bank++) {
    if (!bp[bank])
    continue;
    threshold_remove_bank(bp[bank]);
    bp[bank] = core::ptr::null_mut();
    }
    kfree(bp);
    }
#[no_mangle]
pub unsafe extern "C" fn mce_threshold_remove_device(cpu: c_uint) {
    void mce_threshold_remove_device(unsigned int cpu)
    {
    struct threshold_bank **bp = this_cpu_read(threshold_banks);
    if (!bp)
    return;
//
// Clear the pointer before cleaning up, so that the interrupt won't
// touch anything of this.
//
    this_cpu_write(threshold_banks, core::ptr::null_mut());
    __threshold_remove_device(bp);
    return;
    }
//
// mce_threshold_create_device - Create the per-CPU MCE threshold device
// @cpu:	The plugged in CPU
//
// Create directories and files for all valid threshold banks.
//
// This is invoked from the CPU hotplug callback which was installed in
// mcheck_init_device(). The invocation happens in context of the hotplug
// thread running on @cpu.  The callback is invoked on all CPUs which are
// online when the callback is installed or during a real hotplug event.
//
#[no_mangle]
pub unsafe extern "C" fn mce_threshold_create_device(cpu: c_uint) {
    void mce_threshold_create_device(unsigned int cpu)
    {
    unsigned int numbanks, bank;
    struct threshold_bank **bp;
    if (!mce_flags.amd_threshold)
    return;
    bp = this_cpu_read(threshold_banks);
    if (bp)
    return;
    numbanks = this_cpu_read(mce_num_banks);
    bp = kzalloc_objs(*bp, numbanks);
    if (!bp)
    return;
    for (bank = 0; bank < numbanks; ++bank) {
    if (!(this_cpu_read(bank_map) & BIT_ULL(bank)))
    continue;
    if (threshold_create_bank(bp, cpu, bank)) {
    __threshold_remove_device(bp);
    return;
    }
    }
    this_cpu_write(threshold_banks, bp);
    if (thresholding_irq_en)
    mce_threshold_vector = amd_threshold_interrupt;
    return;
    }
