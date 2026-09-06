//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/perf/e6500-pmu.c
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
// Performance counter support for e6500 family processors.
//
// Author: Priyanka Jain, Priyanka.Jain@freescale.com
// Based on e500-pmu.c
// Copyright 2013 Freescale Semiconductor, Inc.
// Copyright 2008-2009 Paul Mackerras, IBM Corporation.
//

//
// Map of generic hardware event types to hardware events
// Zero if unsupported
//
    static int e6500_generic_events[] = {
    [PERF_COUNT_HW_CPU_CYCLES] = 1,
    [PERF_COUNT_HW_INSTRUCTIONS] = 2,
    [PERF_COUNT_HW_CACHE_MISSES] = 221,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = 12,
    [PERF_COUNT_HW_BRANCH_MISSES] = 15,
    };

//
// Table of generalized cache-related events.
// 0 means not supported, -1 means nonsensical, other values
// are event codes.
//
    static int e6500_cache_events[C(MAX)][C(OP_MAX)][C(RESULT_MAX)] = {
    [C(L1D)] = {
// RESULT_ACCESS		RESULT_MISS
    [C(OP_READ)] = {	27,		222	},
    [C(OP_WRITE)] = {	28,		223	},
    [C(OP_PREFETCH)] = {	29,		0	},
    },
    [C(L1I)] = {
// RESULT_ACCESS		RESULT_MISS
    [C(OP_READ)] = {	2,		254	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	37,		0	},
    },
//
// Assuming LL means L2, it's not a good match for this model.
// It does not have separate read/write events (but it does have
// separate instruction/data events).
//
    [C(LL)] = {
// RESULT_ACCESS		RESULT_MISS
    [C(OP_READ)] = {	0,		0	},
    [C(OP_WRITE)] = {	0,		0	},
    [C(OP_PREFETCH)] = {	0,		0	},
    },
//
// There are data/instruction MMU misses, but that's a miss on
// the chip's internal level-one TLB which is probably not
// what the user wants.  Instead, unified level-two TLB misses
// are reported here.
//
    [C(DTLB)] = {
// RESULT_ACCESS		RESULT_MISS
    [C(OP_READ)] = {	26,		66	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    [C(BPU)] = {
// RESULT_ACCESS		RESULT_MISS
    [C(OP_READ)] = {	12,		15	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    [C(NODE)] = {
// RESULT_ACCESS	RESULT_MISS
    [C(OP_READ)] = {	-1,		-1	},
    [C(OP_WRITE)] = {	-1,		-1	},
    [C(OP_PREFETCH)] = {	-1,		-1	},
    },
    };
    let mut num_events: static int = 512;
// Upper half of event id is PMLCb, for threshold events
#[no_mangle]
unsafe extern "C" fn e6500_xlate_event(event_id: u64) -> u64 {
    static u64 e6500_xlate_event(u64 event_id)
    {
    let mut event_low: u32 = (u32)event_id;
    if (event_low >= num_events ||
    (event_id & (FSL_EMB_EVENT_THRESHMUL | FSL_EMB_EVENT_THRESH)))
    return 0;
    return FSL_EMB_EVENT_VALID;
    }
    static struct fsl_emb_pmu e6500_pmu = {
    .name			= "e6500 family",
    .n_counter		= 6,
    .n_restricted		= 0,
    .xlate_event		= e6500_xlate_event,
    .n_generic		= ARRAY_SIZE(e6500_generic_events),
    .generic_events		= e6500_generic_events,
    .cache_events		= &e6500_cache_events,
    };
#[no_mangle]
unsafe extern "C" fn init_e6500_pmu() -> c_int {
    static int init_e6500_pmu(void)
    {
    let mut pvr: c_uint = mfspr(SPRN_PVR);
    if (PVR_VER(pvr) != PVR_VER_E6500)
    return -ENODEV;
    return register_fsl_emb_pmu(&e6500_pmu);
    }
    early_initcall(init_e6500_pmu);
