//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/pi/idreg-override.c
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
// Early cpufeature override framework
//
// Copyright (C) 2020 Google LLC
// Author: Marc Zyngier <maz@kernel.org>
//

pub const FTR_DESC_NAME_LEN: c_int = 20;
pub const FTR_DESC_FIELD_LEN: c_int = 10;
pub const FTR_ALIAS_NAME_LEN: c_int = 30;
pub const FTR_ALIAS_OPTION_LEN: c_int = 116;
    static u64 __boot_status __initdata;
    typedef bool filter_t(u64 val);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftr_set_desc {
    pub name: [c_char; FTR_DESC_NAME_LEN],
    pub override): PREL64(struct arm64_ftr_override,,
    struct {
    pub name: [c_char; FTR_DESC_FIELD_LEN],
    pub shift: u8,
    pub width: u8,
    pub filter): PREL64(filter_t,,
    pub fields: [}; ],
}

    static const struct ftr_set_desc mmfr0 __prel64_initconst = {
    .name		= "id_aa64mmfr0",
    .override	= &id_aa64mmfr0_override,
    .fields		= {
    FIELD("ecv", ID_AA64MMFR0_EL1_ECV_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn mmfr1_vh_filter(val: u64) -> bool __init {
    static bool __init mmfr1_vh_filter(u64 val)
    {
//
// If we ever reach this point while running VHE, we're
// guaranteed to be on one of these funky, VHE-stuck CPUs. If
// the user was trying to force nVHE on us, proceed with
// attitude adjustment.
//
    return !(__boot_status == (BOOT_CPU_FLAG_E2H | BOOT_CPU_MODE_EL2) &&
    val == 0);
    }
    static const struct ftr_set_desc mmfr1 __prel64_initconst = {
    .name		= "id_aa64mmfr1",
    .override	= &id_aa64mmfr1_override,
    .fields		= {
    FIELD("vh", ID_AA64MMFR1_EL1_VH_SHIFT, mmfr1_vh_filter),
    FIELD("hafdbs", ID_AA64MMFR1_EL1_HAFDBS_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn mmfr2_varange_filter(val: u64) -> bool __init {
    static bool __init mmfr2_varange_filter(u64 val)
    {
    int __maybe_unused feat;
    if (val)
    return false;

    feat = cpuid_feature_extract_signed_field(read_sysreg(id_aa64mmfr0_el1),
    ID_AA64MMFR0_EL1_TGRAN_SHIFT);
    if (feat >= ID_AA64MMFR0_EL1_TGRAN_LPA2) {
    id_aa64mmfr0_override.val |=
    (ID_AA64MMFR0_EL1_TGRAN_LPA2 - 1) << ID_AA64MMFR0_EL1_TGRAN_SHIFT;
    id_aa64mmfr0_override.mask |= 0xfU << ID_AA64MMFR0_EL1_TGRAN_SHIFT;
//
// Override PARange to 48 bits - the override will just be
// ignored if the actual PARange is smaller, but this is
// unlikely to be the case for LPA2 capable silicon.
//
    id_aa64mmfr0_override.val |=
    ID_AA64MMFR0_EL1_PARANGE_48 << ID_AA64MMFR0_EL1_PARANGE_SHIFT;
    id_aa64mmfr0_override.mask |= 0xfU << ID_AA64MMFR0_EL1_PARANGE_SHIFT;
    }

    return true;
    }
    static const struct ftr_set_desc mmfr2 __prel64_initconst = {
    .name		= "id_aa64mmfr2",
    .override	= &id_aa64mmfr2_override,
    .fields		= {
    FIELD("varange", ID_AA64MMFR2_EL1_VARange_SHIFT, mmfr2_varange_filter),
    {}
    },
    };
    static const struct ftr_set_desc mmfr4 __prel64_initconst = {
    .name		= "id_aa64mmfr4",
    .override	= &id_aa64mmfr4_override,
    .fields		= {
    FIELD("nv_frac", ID_AA64MMFR4_EL1_NV_frac_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn pfr0_sve_filter(val: u64) -> bool __init {
    static bool __init pfr0_sve_filter(u64 val)
    {
//
// Disabling SVE also means disabling all the features that
// are associated with it. The easiest way to do it is just to
// override id_aa64zfr0_el1 to be 0.
//
    if (!val) {
    id_aa64zfr0_override.val = 0;
    id_aa64zfr0_override.mask = GENMASK(63, 0);
    }
    return true;
    }
    static const struct ftr_set_desc pfr0 __prel64_initconst = {
    .name		= "id_aa64pfr0",
    .override	= &id_aa64pfr0_override,
    .fields		= {
    FIELD("sve", ID_AA64PFR0_EL1_SVE_SHIFT, pfr0_sve_filter),
    FIELD("el0", ID_AA64PFR0_EL1_EL0_SHIFT, core::ptr::null_mut()),
    FIELD("mpam", ID_AA64PFR0_EL1_MPAM_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn pfr1_sme_filter(val: u64) -> bool __init {
    static bool __init pfr1_sme_filter(u64 val)
    {
//
// Similarly to SVE, disabling SME also means disabling all
// the features that are associated with it. Just set
// id_aa64smfr0_el1 to 0 and don't look back.
//
    if (!val) {
    id_aa64smfr0_override.val = 0;
    id_aa64smfr0_override.mask = GENMASK(63, 0);
    }
    return true;
    }
    static const struct ftr_set_desc pfr1 __prel64_initconst = {
    .name		= "id_aa64pfr1",
    .override	= &id_aa64pfr1_override,
    .fields		= {
    FIELD("bt", ID_AA64PFR1_EL1_BT_SHIFT, core::ptr::null_mut() ),
    FIELD("gcs", ID_AA64PFR1_EL1_GCS_SHIFT, core::ptr::null_mut()),
    FIELD("mte", ID_AA64PFR1_EL1_MTE_SHIFT, core::ptr::null_mut()),
    FIELD("sme", ID_AA64PFR1_EL1_SME_SHIFT, pfr1_sme_filter),
    FIELD("mpam_frac", ID_AA64PFR1_EL1_MPAM_frac_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
    static const struct ftr_set_desc isar1 __prel64_initconst = {
    .name		= "id_aa64isar1",
    .override	= &id_aa64isar1_override,
    .fields		= {
    FIELD("gpi", ID_AA64ISAR1_EL1_GPI_SHIFT, core::ptr::null_mut()),
    FIELD("gpa", ID_AA64ISAR1_EL1_GPA_SHIFT, core::ptr::null_mut()),
    FIELD("api", ID_AA64ISAR1_EL1_API_SHIFT, core::ptr::null_mut()),
    FIELD("apa", ID_AA64ISAR1_EL1_APA_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
    static const struct ftr_set_desc isar2 __prel64_initconst = {
    .name		= "id_aa64isar2",
    .override	= &id_aa64isar2_override,
    .fields		= {
    FIELD("gpa3", ID_AA64ISAR2_EL1_GPA3_SHIFT, core::ptr::null_mut()),
    FIELD("apa3", ID_AA64ISAR2_EL1_APA3_SHIFT, core::ptr::null_mut()),
    FIELD("mops", ID_AA64ISAR2_EL1_MOPS_SHIFT, core::ptr::null_mut()),
    {}
    },
    };
    static const struct ftr_set_desc smfr0 __prel64_initconst = {
    .name		= "id_aa64smfr0",
    .override	= &id_aa64smfr0_override,
    .fields		= {
    FIELD("smever", ID_AA64SMFR0_EL1_SMEver_SHIFT, core::ptr::null_mut()),
// FA64 is a one bit field... :-/
    { "fa64", ID_AA64SMFR0_EL1_FA64_SHIFT, 1, },
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn hvhe_filter(val: u64) -> bool __init {
    static bool __init hvhe_filter(u64 val)
    {
    let mut mmfr1: u64 = read_sysreg(id_aa64mmfr1_el1);
    return (val == 1 &&
    lower_32_bits(__boot_status) == BOOT_CPU_MODE_EL2 &&
    cpuid_feature_extract_unsigned_field(mmfr1,
    ID_AA64MMFR1_EL1_VH_SHIFT));
    }
    static const struct ftr_set_desc sw_features __prel64_initconst = {
    .name		= "arm64_sw",
    .override	= &arm64_sw_feature_override,
    .fields		= {
    FIELD("nokaslr", ARM64_SW_FEATURE_OVERRIDE_NOKASLR, core::ptr::null_mut()),
    FIELD("hvhe", ARM64_SW_FEATURE_OVERRIDE_HVHE, hvhe_filter),
    FIELD("rodataoff", ARM64_SW_FEATURE_OVERRIDE_RODATA_OFF, core::ptr::null_mut()),
    {}
    },
    };
    static const
    PREL64(const struct ftr_set_desc, reg) regs[] __prel64_initconst = {
    { &mmfr0	},
    { &mmfr1	},
    { &mmfr2	},
    { &mmfr4	},
    { &pfr0 	},
    { &pfr1 	},
    { &isar1	},
    { &isar2	},
    { &smfr0	},
    { &sw_features	},
    };
    static const struct {
    char	alias[FTR_ALIAS_NAME_LEN];
    char	feature[FTR_ALIAS_OPTION_LEN];
    } aliases[] __initconst = {
    { "kvm_arm.mode=nvhe",		"arm64_sw.hvhe=0 id_aa64mmfr1.vh=0" },
    { "kvm_arm.mode=protected",	"arm64_sw.hvhe=1" },
    { "arm64.nosve",		"id_aa64pfr0.sve=0" },
    { "arm64.nosme",		"id_aa64pfr1.sme=0" },
    { "arm64.nobti",		"id_aa64pfr1.bt=0" },
    { "arm64.nogcs",		"id_aa64pfr1.gcs=0" },
    { "arm64.nopauth",
    "id_aa64isar1.gpi=0 id_aa64isar1.gpa=0 "
    "id_aa64isar1.api=0 id_aa64isar1.apa=0 "
    "id_aa64isar2.gpa3=0 id_aa64isar2.apa3=0"	   },
    { "arm64.nomops",		"id_aa64isar2.mops=0" },
    { "arm64.nomte",		"id_aa64pfr1.mte=0" },
    { "nokaslr",			"arm64_sw.nokaslr=1" },
    { "rodata=off",			"arm64_sw.rodataoff=1" },
    { "arm64.nohaft",		"id_aa64mmfr1.hafdbs=2" },
    { "arm64.nolva",		"id_aa64mmfr2.varange=0" },
    { "arm64.no32bit_el0",		"id_aa64pfr0.el0=1" },
    { "arm64.nompam",		"id_aa64pfr0.mpam=0 id_aa64pfr1.mpam_frac=0" },
    };
#[no_mangle]
unsafe extern "C" fn parse_hexdigit(p: *const c_char, v: *mut u64) -> int __init {
    static int __init parse_hexdigit(const char *p, u64 *v)
    {
// skip "0x" if it comes next
    if (p[0] == '0' && tolower(p[1]) == 'x')
    p += 2;
// check whether the RHS is a single hex digit
    if (!isxdigit(p[0]) || (p[1] && !isspace(p[1])))
    return -EINVAL;
// v = tolower(*p) - (isdigit(*p) ? '0' : 'a' - 10);
    return 0;
    }
    static int __init find_field(const char *cmdline, char *opt, int len,
    const struct ftr_set_desc *reg, int f, u64 *v)
    {
    let mut flen: c_int = strlen(reg.fields[f].name);
// append '<fieldname>=' to obtain '<name>.<fieldname>='
    memcpy(opt + len, reg.fields[f].name, flen);
    len += flen;
    opt[len++] = '=';
    if (memcmp(cmdline, opt, len))
    return -1;
    return parse_hexdigit(cmdline + len, v);
    }
#[no_mangle]
unsafe extern "C" fn match_options(cmdline: *const c_char) -> void __init {
    static void __init match_options(const char *cmdline)
    {
    char opt[FTR_DESC_NAME_LEN + FTR_DESC_FIELD_LEN + 2];
    int i;
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    const struct ftr_set_desc *reg = prel64_pointer(regs[i].reg);
    struct arm64_ftr_override *override;
    let mut len: c_int = strlen(reg.name);
    int f;
    override = prel64_pointer(reg.override);
// set opt[] to '<name>.'
    memcpy(opt, reg.name, len);
    opt[len++] = '.';
    for (f = 0; reg.fields[f].name[0] != '\0'; f++) {
    let mut shift: u64 = reg.fields[f].shift;
    let mut width: u64 = reg.fields[f].width ?: 4;
    let mut mask: u64 = GENMASK_ULL(shift + width - 1, shift);
    bool (*filter)(u64 val);
    u64 v;
    if (find_field(cmdline, opt, len, reg, f, &v))
    continue;
//
// If an override gets filtered out, advertise
// it by setting the value to the all-ones while
// clearing the mask... Yes, this is fragile.
//
    filter = prel64_pointer(reg.fields[f].filter);
    if (filter && !filter(v)) {
    override.val  |= mask;
    override.mask &= ~mask;
    continue;
    }
    override.val  &= ~mask;
    override.val  |= (v << shift) & mask;
    override.mask |= mask;
    return;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __parse_cmdline(cmdline: *const c_char, parse_aliases: bool) -> __init void {
    static __init void __parse_cmdline(const char *cmdline, bool parse_aliases)
    {
    do {
    char buf[256];
    size_t len;
    int i;
    cmdline = skip_spaces(cmdline);
// terminate on "--" appearing on the command line by itself
    if (cmdline[0] == '-' && cmdline[1] == '-' && isspace(cmdline[2]))
    return;
    for (len = 0; cmdline[len] && !isspace(cmdline[len]); len++) {
    if (len >= sizeof(buf) - 1)
    break;
    if (cmdline[len] == '-')
    buf[len] = '_';
    else
    buf[len] = cmdline[len];
    }
    if (!len)
    return;
    buf[len] = 0;
    cmdline += len;
    match_options(buf);
    for (i = 0; parse_aliases && i < ARRAY_SIZE(aliases); i++)
    if (!memcmp(buf, aliases[i].alias, len + 1))
    __parse_cmdline(aliases[i].feature, false);
    } while (1);
    }
    static __init const u8 *get_bootargs_cmdline(const void *fdt, int node)
    {
    static char const bootargs[] __initconst = "bootargs";
    const u8 *prop;
    if (node < 0)
    return core::ptr::null_mut();
    prop = fdt_getprop(fdt, node, bootargs, core::ptr::null_mut());
    if (!prop)
    return core::ptr::null_mut();
    return strlen(prop) ? prop : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn parse_cmdline(fdt: *const c_void, chosen: c_int) -> __init void {
    static __init void parse_cmdline(const void *fdt, int chosen)
    {
    static char const cmdline[] __initconst = CONFIG_CMDLINE;
    const u8 *prop = get_bootargs_cmdline(fdt, chosen);
    if (IS_ENABLED(CONFIG_CMDLINE_FORCE) || !prop)
    __parse_cmdline(cmdline, true);
    if (!IS_ENABLED(CONFIG_CMDLINE_FORCE) && prop)
    __parse_cmdline(prop, true);
    }
    void __init init_feature_override(u64 boot_status, const void *fdt,
    int chosen)
    {
    struct arm64_ftr_override *override;
    const struct ftr_set_desc *reg;
    int i;
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    reg = prel64_pointer(regs[i].reg);
    override = prel64_pointer(reg.override);
    override.val  = 0;
    override.mask = 0;
    }
    __boot_status = boot_status;
    parse_cmdline(fdt, chosen);
    for (i = 0; i < ARRAY_SIZE(regs); i++) {
    reg = prel64_pointer(regs[i].reg);
    override = prel64_pointer(reg.override);
    dcache_clean_inval_poc((unsigned long)override,
    (unsigned long)(override + 1));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn skip_spaces(str: *const c_char) -> *mut char  __init {
    char * __init skip_spaces(const char *str)
    {
    while (isspace(*str))
    ++str;
    return (char *)str;
    }
