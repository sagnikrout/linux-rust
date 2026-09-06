//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/perf/ppc970-pmu.c
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
// Performance counter support for PPC970-family processors.
//
// Copyright 2008-2009 Paul Mackerras, IBM Corporation.
//

//
// Bits in event code for PPC970
//

pub const PM_PMC_MSK: c_uint = 0xf;

pub const PM_UNIT_MSK: c_uint = 0xf;
pub const PM_SPCSEL_SH: c_int = 6;
pub const PM_SPCSEL_MSK: c_int = 3;

pub const PM_BYTE_MSK: c_int = 3;
pub const PM_PMCSEL_MSK: c_uint = 0xf;
// Values in PM_UNIT field
pub const PM_NONE: c_int = 0;
pub const PM_FPU: c_int = 1;
pub const PM_VPU: c_int = 2;
pub const PM_ISU: c_int = 3;
pub const PM_IFU: c_int = 4;
pub const PM_IDU: c_int = 5;
pub const PM_STS: c_int = 6;
pub const PM_LSU0: c_int = 7;
pub const PM_LSU1U: c_int = 8;
pub const PM_LSU1L: c_int = 9;
pub const PM_LASTUNIT: c_int = 9;
//
// Bits in MMCR0 for PPC970
//
pub const MMCR0_PMC1SEL_SH: c_int = 8;
pub const MMCR0_PMC2SEL_SH: c_int = 1;
pub const MMCR_PMCSEL_MSK: c_uint = 0x1f;
//
// Bits in MMCR1 for PPC970
//
pub const MMCR1_TTM0SEL_SH: c_int = 62;
pub const MMCR1_TTM1SEL_SH: c_int = 59;
pub const MMCR1_TTM3SEL_SH: c_int = 53;
pub const MMCR1_TTMSEL_MSK: c_int = 3;
pub const MMCR1_TD_CP_DBG0SEL_SH: c_int = 50;
pub const MMCR1_TD_CP_DBG1SEL_SH: c_int = 48;
pub const MMCR1_TD_CP_DBG2SEL_SH: c_int = 46;
pub const MMCR1_TD_CP_DBG3SEL_SH: c_int = 44;
pub const MMCR1_PMC1_ADDER_SEL_SH: c_int = 39;
pub const MMCR1_PMC2_ADDER_SEL_SH: c_int = 38;
pub const MMCR1_PMC6_ADDER_SEL_SH: c_int = 37;
pub const MMCR1_PMC5_ADDER_SEL_SH: c_int = 36;
pub const MMCR1_PMC8_ADDER_SEL_SH: c_int = 35;
pub const MMCR1_PMC7_ADDER_SEL_SH: c_int = 34;
pub const MMCR1_PMC3_ADDER_SEL_SH: c_int = 33;
pub const MMCR1_PMC4_ADDER_SEL_SH: c_int = 32;
pub const MMCR1_PMC3SEL_SH: c_int = 27;
pub const MMCR1_PMC4SEL_SH: c_int = 22;
pub const MMCR1_PMC5SEL_SH: c_int = 17;
pub const MMCR1_PMC6SEL_SH: c_int = 12;
pub const MMCR1_PMC7SEL_SH: c_int = 7;
pub const MMCR1_PMC8SEL_SH: c_int = 2;
    static short mmcr1_adder_bits[8] = {
    MMCR1_PMC1_ADDER_SEL_SH,
    MMCR1_PMC2_ADDER_SEL_SH,
    MMCR1_PMC3_ADDER_SEL_SH,
    MMCR1_PMC4_ADDER_SEL_SH,
    MMCR1_PMC5_ADDER_SEL_SH,
    MMCR1_PMC6_ADDER_SEL_SH,
    MMCR1_PMC7_ADDER_SEL_SH,
    MMCR1_PMC8_ADDER_SEL_SH
    };
//
// Layout of constraint bits:
// 6666555555555544444444443333333333222222222211111111110000000000
// 3210987654321098765432109876543210987654321098765432109876543210
// <><><>[  >[  >[  ><  ><  ><  ><  ><><><><><><><><>
// SPT0T1 UC  PS1 PS2 B0  B1  B2  B3 P1P2P3P4P5P6P7P8
//
// SP - SPCSEL constraint
// 48-49: SPCSEL value 0x3_0000_0000_0000
//
// T0 - TTM0 constraint
// 46-47: TTM0SEL value (0=FPU, 2=IFU, 3=VPU) 0xC000_0000_0000
//
// T1 - TTM1 constraint
// 44-45: TTM1SEL value (0=IDU, 3=STS) 0x3000_0000_0000
//
// UC - unit constraint: can't have all three of FPU|IFU|VPU, ISU, IDU|STS
// 43: UC3 error 0x0800_0000_0000
// 42: FPU|IFU|VPU events needed 0x0400_0000_0000
// 41: ISU events needed 0x0200_0000_0000
// 40: IDU|STS events needed 0x0100_0000_0000
//
// PS1
// 39: PS1 error 0x0080_0000_0000
// 36-38: count of events needing PMC1/2/5/6 0x0070_0000_0000
//
// PS2
// 35: PS2 error 0x0008_0000_0000
// 32-34: count of events needing PMC3/4/7/8 0x0007_0000_0000
//
// B0
// 28-31: Byte 0 event source 0xf000_0000
// Encoding as for the event code
//
// B1, B2, B3
// 24-27, 20-23, 16-19: Byte 1, 2, 3 event sources
//
// P1
// 15: P1 error 0x8000
// 14-15: Count of events needing PMC1
//
// P2..P8
// 0-13: Count of events needing PMC2..PMC8
//
    static unsigned char direct_marked_event[8] = {
    (1<<2) | (1<<3),	/* PMC1: PM_MRK_GRP_DISP, PM_MRK_ST_CMPL */
    (1<<3) | (1<<5),	/* PMC2: PM_THRESH_TIMEO, PM_MRK_BRU_FIN */
    (1<<3) | (1<<5),	/* PMC3: PM_MRK_ST_CMPL_INT, PM_MRK_VMX_FIN */
    (1<<4) | (1<<5),	/* PMC4: PM_MRK_GRP_CMPL, PM_MRK_CRU_FIN */
    (1<<4) | (1<<5),	/* PMC5: PM_GRP_MRK, PM_MRK_GRP_TIMEO */
    (1<<3) | (1<<4) | (1<<5),
// PMC6: PM_MRK_ST_STS, PM_MRK_FXU_FIN, PM_MRK_GRP_ISSUED
    (1<<4) | (1<<5),	/* PMC7: PM_MRK_FPU_FIN, PM_MRK_INST_FIN */
    (1<<4)			/* PMC8: PM_MRK_LSU_FIN */
    };
//
// Returns 1 if event counts things relating to marked instructions
// and thus needs the MMCRA_SAMPLE_ENABLE bit set, or 0 if not.
//
#[no_mangle]
unsafe extern "C" fn p970_marked_instr_event(event: u64) -> c_int {
    static int p970_marked_instr_event(u64 event)
    {
    int pmc, psel, unit, byte, bit;
    unsigned int mask;
    pmc = (event >> PM_PMC_SH) & PM_PMC_MSK;
    psel = event & PM_PMCSEL_MSK;
    if (pmc) {
    if (direct_marked_event[pmc - 1] & (1 << psel))
    return 1;
    if (psel == 0)		/* add events */
    bit = (pmc <= 4)? pmc - 1: 8 - pmc;
    else if (psel == 7 || psel == 13)	/* decode events */
    bit = 4;
    else
    return 0;
    } else
    bit = psel;
    byte = (event >> PM_BYTE_SH) & PM_BYTE_MSK;
    unit = (event >> PM_UNIT_SH) & PM_UNIT_MSK;
    mask = 0;
    switch (unit) {
    case PM_VPU:
    mask = 0x4c;		/* byte 0 bits 2,3,6 */
    break;
    case PM_LSU0:
// byte 2 bits 0,2,3,4,6; all of byte 1
    mask = 0x085dff00;
    break;
    case PM_LSU1L:
    mask = 0x50 << 24;	/* byte 3 bits 4,6 */
    break;
    }
    return (mask >> (byte * 8 + bit)) & 1;
    }
// Masks and values for using events from the various units
    static unsigned long unit_cons[PM_LASTUNIT+1][2] = {
    [PM_FPU] =   { 0xc80000000000ull, 0x040000000000ull },
    [PM_VPU] =   { 0xc80000000000ull, 0xc40000000000ull },
    [PM_ISU] =   { 0x080000000000ull, 0x020000000000ull },
    [PM_IFU] =   { 0xc80000000000ull, 0x840000000000ull },
    [PM_IDU] =   { 0x380000000000ull, 0x010000000000ull },
    [PM_STS] =   { 0x380000000000ull, 0x310000000000ull },
    };
    static int p970_get_constraint(u64 event, unsigned long *maskp,
    unsigned long *valp, u64 event_config1 __maybe_unused)
    {
    int pmc, byte, unit, sh, spcsel;
    let mut mask: c_ulong = 0, value = 0;
    let mut grp: c_int = -1;
    pmc = (event >> PM_PMC_SH) & PM_PMC_MSK;
    if (pmc) {
    if (pmc > 8)
    return -1;
    sh = (pmc - 1) * 2;
    mask |= 2 << sh;
    value |= 1 << sh;
    grp = ((pmc - 1) >> 1) & 1;
    }
    unit = (event >> PM_UNIT_SH) & PM_UNIT_MSK;
    if (unit) {
    if (unit > PM_LASTUNIT)
    return -1;
    mask |= unit_cons[unit][0];
    value |= unit_cons[unit][1];
    byte = (event >> PM_BYTE_SH) & PM_BYTE_MSK;
//
// Bus events on bytes 0 and 2 can be counted
// on PMC1/2/5/6; bytes 1 and 3 on PMC3/4/7/8.
//
    if (!pmc)
    grp = byte & 1;
// Set byte lane select field
    mask  |= 0xfULL << (28 - 4 * byte);
    value |= (unsigned long)unit << (28 - 4 * byte);
    }
    if (grp == 0) {
// increment PMC1/2/5/6 field
    mask  |= 0x8000000000ull;
    value |= 0x1000000000ull;
    } else if (grp == 1) {
// increment PMC3/4/7/8 field
    mask  |= 0x800000000ull;
    value |= 0x100000000ull;
    }
    spcsel = (event >> PM_SPCSEL_SH) & PM_SPCSEL_MSK;
    if (spcsel) {
    mask  |= 3ull << 48;
    value |= (unsigned long)spcsel << 48;
    }
// maskp = mask;
// valp = value;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p970_get_alternatives(event: u64, flags: c_uint, alt[]: u64) -> c_int {
    static int p970_get_alternatives(u64 event, unsigned int flags, u64 alt[])
    {
    alt[0] = event;
// 2 alternatives for LSU empty
    if (event == 0x2002 || event == 0x3002) {
    alt[1] = event ^ 0x1000;
    return 2;
    }
    return 1;
    }
    static int p970_compute_mmcr(u64 event[], int n_ev,
    unsigned int hwc[], struct mmcr_regs *mmcr,
    struct perf_event *pevents[],
    u32 flags __maybe_unused)
    {
    let mut mmcr0: c_ulong = 0, mmcr1 = 0, mmcra = 0;
    unsigned int pmc, unit, byte, psel;
    unsigned int ttm, grp;
    let mut pmc_inuse: c_uint = 0;
    unsigned int pmc_grp_use[2];
    unsigned char busbyte[4];
    unsigned char unituse[16];
    unsigned char unitmap[] = { 0, 0<<3, 3<<3, 1<<3, 2<<3, 0|4, 3|4 };
    unsigned char ttmuse[2];
    unsigned char pmcsel[8];
    int i;
    int spcsel;
    if (n_ev > 8)
    return -1;
// First pass to count resource use
    pmc_grp_use[0] = pmc_grp_use[1] = 0;
    memset(busbyte, 0, sizeof(busbyte));
    memset(unituse, 0, sizeof(unituse));
    for (i = 0; i < n_ev; ++i) {
    pmc = (event[i] >> PM_PMC_SH) & PM_PMC_MSK;
    if (pmc) {
    if (pmc_inuse & (1 << (pmc - 1)))
    return -1;
    pmc_inuse |= 1 << (pmc - 1);
// count 1/2/5/6 vs 3/4/7/8 use
    ++pmc_grp_use[((pmc - 1) >> 1) & 1];
    }
    unit = (event[i] >> PM_UNIT_SH) & PM_UNIT_MSK;
    byte = (event[i] >> PM_BYTE_SH) & PM_BYTE_MSK;
    if (unit) {
    if (unit > PM_LASTUNIT)
    return -1;
    if (!pmc)
    ++pmc_grp_use[byte & 1];
    if (busbyte[byte] && busbyte[byte] != unit)
    return -1;
    busbyte[byte] = unit;
    unituse[unit] = 1;
    }
    }
    if (pmc_grp_use[0] > 4 || pmc_grp_use[1] > 4)
    return -1;
//
// Assign resources and set multiplexer selects.
//
// PM_ISU can go either on TTM0 or TTM1, but that's the only
// choice we have to deal with.
//
    if (unituse[PM_ISU] &
    (unituse[PM_FPU] | unituse[PM_IFU] | unituse[PM_VPU]))
    unitmap[PM_ISU] = 2 | 4;	/* move ISU to TTM1 */
// Set TTM[01]SEL fields.
    ttmuse[0] = ttmuse[1] = 0;
    for (i = PM_FPU; i <= PM_STS; ++i) {
    if (!unituse[i])
    continue;
    ttm = unitmap[i];
    ++ttmuse[(ttm >> 2) & 1];
    mmcr1 |= (unsigned long)(ttm & ~4) << MMCR1_TTM1SEL_SH;
    }
// Check only one unit per TTMx
    if (ttmuse[0] > 1 || ttmuse[1] > 1)
    return -1;
// Set byte lane select fields and TTM3SEL.
    for (byte = 0; byte < 4; ++byte) {
    unit = busbyte[byte];
    if (!unit)
    continue;
    if (unit <= PM_STS)
    ttm = (unitmap[unit] >> 2) & 1;
#[no_mangle]
pub unsafe extern "C" fn if(PM_LSU0: unit ==) -> else {
    else if (unit == PM_LSU0)
    ttm = 2;
    else {
    ttm = 3;
    if (unit == PM_LSU1L && byte >= 2)
    mmcr1 |= 1ull << (MMCR1_TTM3SEL_SH + 3 - byte);
    }
    mmcr1 |= (unsigned long)ttm
    << (MMCR1_TD_CP_DBG0SEL_SH - 2 * byte);
    }
// Second pass: assign PMCs, set PMCxSEL and PMCx_ADDER_SEL fields
    memset(pmcsel, 0x8, sizeof(pmcsel));	/* 8 means don't count */
    for (i = 0; i < n_ev; ++i) {
    pmc = (event[i] >> PM_PMC_SH) & PM_PMC_MSK;
    unit = (event[i] >> PM_UNIT_SH) & PM_UNIT_MSK;
    byte = (event[i] >> PM_BYTE_SH) & PM_BYTE_MSK;
    psel = event[i] & PM_PMCSEL_MSK;
    if (!pmc) {
// Bus event or any-PMC direct event
    if (unit)
    psel |= 0x10 | ((byte & 2) << 2);
    else
    psel |= 8;
    for (pmc = 0; pmc < 8; ++pmc) {
    if (pmc_inuse & (1 << pmc))
    continue;
    grp = (pmc >> 1) & 1;
    if (unit) {
    if (grp == (byte & 1))
    break;
    } else if (pmc_grp_use[grp] < 4) {
    ++pmc_grp_use[grp];
    break;
    }
    }
    pmc_inuse |= 1 << pmc;
    } else {
// Direct event
    --pmc;
    if (psel == 0 && (byte & 2))
// add events on higher-numbered bus
    mmcr1 |= 1ull << mmcr1_adder_bits[pmc];
    }
    pmcsel[pmc] = psel;
    hwc[i] = pmc;
    spcsel = (event[i] >> PM_SPCSEL_SH) & PM_SPCSEL_MSK;
    mmcr1 |= spcsel;
    if (p970_marked_instr_event(event[i]))
    mmcra |= MMCRA_SAMPLE_ENABLE;
    }
    for (pmc = 0; pmc < 2; ++pmc)
    mmcr0 |= pmcsel[pmc] << (MMCR0_PMC1SEL_SH - 7 * pmc);
    for (; pmc < 8; ++pmc)
    mmcr1 |= (unsigned long)pmcsel[pmc]
    << (MMCR1_PMC3SEL_SH - 5 * (pmc - 2));
    if (pmc_inuse & 1)
    mmcr0 |= MMCR0_PMC1CE;
    if (pmc_inuse & 0xfe)
    mmcr0 |= MMCR0_PMCjCE;
    mmcra |= 0x2000;	/* mark only one IOP per PPC instruction */
// Return MMCRx values
    mmcr.mmcr0 = mmcr0;
    mmcr.mmcr1 = mmcr1;
    mmcr.mmcra = mmcra;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p970_disable_pmc(pmc: c_uint, mmcr: *mut mmcr_regs) {
    static void p970_disable_pmc(unsigned int pmc, struct mmcr_regs *mmcr)
    {
    int shift;
//
// Setting the PMCxSEL field to 0x08 disables PMC x.
//
    if (pmc <= 1) {
    shift = MMCR0_PMC1SEL_SH - 7 * pmc;
    mmcr.mmcr0 = (mmcr.mmcr0 & ~(0x1fUL << shift)) | (0x08UL << shift);
    } else {
    shift = MMCR1_PMC3SEL_SH - 5 * (pmc - 2);
    mmcr.mmcr1 = (mmcr.mmcr1 & ~(0x1fUL << shift)) | (0x08UL << shift);
    }
    }
    static int ppc970_generic_events[] = {
    [PERF_COUNT_HW_CPU_CYCLES]		= 7,
    [PERF_COUNT_HW_INSTRUCTIONS]		= 1,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= 0x8810, /* PM_LD_REF_L1 */
    [PERF_COUNT_HW_CACHE_MISSES]		= 0x3810, /* PM_LD_MISS_L1 */
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= 0x431,  /* PM_BR_ISSUED */
    [PERF_COUNT_HW_BRANCH_MISSES] 		= 0x327,  /* PM_GRP_BR_MPRED */
    };

//
// Table of generalized cache-related events.
// 0 means not supported, -1 means nonsensical, other values
// are event codes.
//
    static u64 ppc970_cache_events[C(MAX)][C(OP_MAX)][C(RESULT_MAX)] = {
    [C(L1D)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0x8810,		0x3810	},
    [C(OP_WRITE)] = {	0x7810,		0x813	},
    [C(OP_PREFETCH)] = {	0x731,		0	},
    },
    [C(L1I)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0,		0	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	0,		0	},
    },
    [C(LL)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0,		0	},
    [C(OP_WRITE)] = {	0,		0	},
    [C(OP_PREFETCH)] = {	0x733,		0	},
    },
    [C(DTLB)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0,		0x704	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    [C(ITLB)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0,		0x700	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    [C(BPU)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	0x431,		0x327	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    [C(NODE)] = {		/* 	RESULT_ACCESS	RESULT_MISS */
    [C(OP_READ)] = {	-1,		-1	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    };
    static struct power_pmu ppc970_pmu = {
    .name			= "PPC970/FX/MP",
    .n_counter		= 8,
    .max_alternatives	= 2,
    .add_fields		= 0x001100005555ull,
    .test_adder		= 0x013300000000ull,
    .compute_mmcr		= p970_compute_mmcr,
    .get_constraint		= p970_get_constraint,
    .get_alternatives	= p970_get_alternatives,
    .disable_pmc		= p970_disable_pmc,
    .n_generic		= ARRAY_SIZE(ppc970_generic_events),
    .generic_events		= ppc970_generic_events,
    .cache_events		= &ppc970_cache_events,
    .flags			= PPMU_NO_SIPR | PPMU_NO_CONT_SAMPLING,
    };
#[no_mangle]
pub unsafe extern "C" fn init_ppc970_pmu() -> int __init {
    int __init init_ppc970_pmu(void)
    {
    let mut pvr: c_uint = mfspr(SPRN_PVR);
    if (PVR_VER(pvr) != PVR_970 && PVR_VER(pvr) != PVR_970MP &&
    PVR_VER(pvr) != PVR_970FX && PVR_VER(pvr) != PVR_970GX)
    return -ENODEV;
    return register_power_pmu(&ppc970_pmu);
    }
