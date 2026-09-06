//! Automatically rewritten from C to Rust
//! Source: fs/xfs/xfs_globals.c
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

//
// Tunable XFS parameters.  xfs_params is required even when CONFIG_SYSCTL=n,
// other XFS code uses these values.  Times are measured in centisecs (i.e.
// 100ths of a second) with the exception of blockgc_timer, which is measured
// in seconds.
//
    xfs_param_t xfs_params = {
// MIN		DFLT		MAX
    .panic_mask	= {	0,		0,		XFS_PTAG_MASK},
    .error_level	= {	0,		3,		11	},
    .syncd_timer	= {	1*100,		30*100,		7200*100},
    .stats_clear	= {	0,		0,		1	},
    .inherit_sync	= {	0,		1,		1	},
    .inherit_nodump	= {	0,		1,		1	},
    .inherit_noatim = {	0,		1,		1	},
    .inherit_nosym	= {	0,		0,		1	},
    .rotorstep	= {	1,		1,		255	},
    .inherit_nodfrg	= {	0,		1,		1	},
    .fstrm_timer	= {	1,		30*100,		3600*100},
    .blockgc_timer	= {	1,		300,		3600*24},
    };
    struct xfs_globals xfs_globals = {
    .log_recovery_delay	=	0,	/* no delay by default */
    .mount_delay		=	0,	/* no delay by default */

    .bug_on_assert		=	true,	/* assert failures BUG() */

    .bug_on_assert		=	false,	/* assert failures WARN() */

    .pwork_threads		=	-1,	/* automatic thread detection */
    .larp			=	false,	/* log attribute replay */

//
// Leave this many record slots empty when bulk loading btrees.  By
// default we load new btree leaf blocks 75% full.
//
    .bload_leaf_slack	=	-1,
//
// Leave this many key/ptr slots empty when bulk loading btrees.  By
// default we load new btree node blocks 75% full.
//
    .bload_node_slack	=	-1,
    };
