//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/energy_model.h
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
// struct em_perf_state - Performance state of a performance domain
// @performance:	CPU performance (capacity) at a given frequency
// @frequency:	The frequency in KHz, for consistency with CPUFreq
// @power:	The power consumed at this level (by 1 CPU or by a registered
// device). It can be a total power: static and dynamic.
// @cost:	The cost coefficient associated with this level, used during
// energy calculation. Equal to: 10 * power * max_frequency / frequency
// @flags:	see "em_perf_state flags" description below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_perf_state {
    pub performance: c_ulong,
    pub frequency: c_ulong,
    pub power: c_ulong,
    pub cost: c_ulong,
    pub flags: c_ulong,
}

//
// em_perf_state flags:
//
// EM_PERF_STATE_INEFFICIENT: The performance state is inefficient. There is
// in this em_perf_domain, another performance state with a higher frequency
// but a lower or equal power cost. Such inefficient states are ignored when
// using em_pd_get_efficient_*() functions.
//

//
// struct em_perf_table - Performance states table
// @rcu:	RCU used for safe access and destruction
// @kref:	Reference counter to track the users
// @state:	List of performance states, in ascending order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_perf_table {
    pub rcu: rcu_head,
    pub kref: kref,
    pub state: [em_perf_state; ],
}

//
// struct em_perf_domain - Performance domain
// @em_table:		Pointer to the runtime modifiable em_perf_table
// @node:		node in	em_pd_list (in energy_model.c)
// @id:			A unique ID number for each performance domain
// @nr_perf_states:	Number of performance states
// @min_perf_state:	Minimum allowed Performance State index
// @max_perf_state:	Maximum allowed Performance State index
// @flags:		See "em_perf_domain flags"
// @cpus:		Cpumask covering the CPUs of the domain. It's here
// for performance reasons to avoid potential cache
// misses during energy calculations in the scheduler
// and simplifies allocating/freeing that memory region.
//
// In case of CPU device, a "performance domain" represents a group of CPUs
// whose performance is scaled together. All CPUs of a performance domain
// must have the same micro-architecture. Performance domains often have
// a 1-to-1 mapping with CPUFreq policies. In case of other devices the @cpus
// field is unused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_perf_domain {
    pub em_table: *mut em_perf_table __rcu,
    pub node: list_head,
    pub id: c_int,
    pub nr_perf_states: c_int,
    pub min_perf_state: c_int,
    pub max_perf_state: c_int,
    pub flags: c_ulong,
    pub cpus: [c_ulong; ],
}

//
// em_perf_domain flags:
//
// EM_PERF_DOMAIN_MICROWATTS: The power values are in micro-Watts or some
// other scale.
//
// EM_PERF_DOMAIN_SKIP_INEFFICIENCIES: Skip inefficient states when estimating
// energy consumption.
//
// EM_PERF_DOMAIN_ARTIFICIAL: The power values are artificial and might be
// created by platform missing real power information
//

//
// The max power value in micro-Watts. The limit of 64 Watts is set as
// a safety net to not overflow multiplications on 32bit platforms. The
// 32bit value limit for total Perf Domain power implies a limit of
// maximum CPUs in such domain to 64.
//

//
// To avoid possible energy estimation overflow on 32bit machines add
// limits to number of CPUs in the Perf. Domain.
// We are safe on 64bit machine, thus some big number.
//

pub const EM_MAX_NUM_CPUS: c_int = 4096;

pub const EM_MAX_NUM_CPUS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_data_callback {
//
// active_power() - Provide power at the next performance state of
// a device
// @dev		: Device for which we do this operation (can be a CPU)
// @power	: Active power at the performance state
// (modified)
// @freq	: Frequency at the performance state in kHz
// (modified)
//
// active_power() must find the lowest performance state of 'dev' above
// 'freq' and update 'power' and 'freq' to the matching active power
// and frequency.
//
// In case of CPUs, the power is the one of a single CPU in the domain,
// expressed in micro-Watts or an abstract scale. It is expected to
// fit in the [0, EM_MAX_POWER] range.
//
// Return 0 on success.
//
    pub freq): *mut c_ulong,
//
// get_cost() - Provide the cost at the given performance state of
// a device
// @dev		: Device for which we do this operation (can be a CPU)
// @freq	: Frequency at the performance state in kHz
// @cost	: The cost value for the performance state
// (modified)
//
// In case of CPUs, the cost is the one of a single CPU in the domain.
// It is expected to fit in the [0, EM_MAX_POWER] range due to internal
// usage in EAS calculation.
//
// Return 0 on success, or appropriate error value in case of failure.
//
    pub cost): *mut c_ulong,
}

extern "C" {
    pub fn em_dev_unregister_perf_domain(dev: *mut device);
}
extern "C" {
    pub fn em_table_free(table: *mut em_perf_table);
}
extern "C" {
    pub fn em_dev_update_chip_binning(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn em_adjust_cpu_capacity(cpu: c_uint);
}
extern "C" {
    pub fn em_rebuild_sched_domains();
}
//
// em_pd_get_efficient_state() - Get an efficient performance state from the EM
// @table:		List of performance states, in ascending order
// @pd:			performance domain for which this must be done
// @max_util:		Max utilization to map with the EM
//
// It is called from the scheduler code quite frequently and as a consequence
// doesn't implement any check.
//
// Return: An efficient performance state id, high enough to meet @max_util
// requirement.
//
// em_cpu_energy() - Estimates the energy consumed by the CPUs of a
// performance domain
// @pd		: performance domain for which energy has to be estimated
// @max_util	: highest utilization among CPUs of the domain
// @sum_util	: sum of the utilization of all CPUs in the domain
// @allowed_cpu_cap	: maximum allowed CPU capacity for the @pd, which
// might reflect reduced frequency (due to thermal)
//
// This function must be used only for CPU devices. There is no validation,
// i.e. if the EM is a CPU type and has cpumask allocated. It is called from
// the scheduler code quite frequently and that is why there is not checks.
//
// Return: the sum of the energy consumed by the CPUs of the domain assuming
// a capacity state satisfying the max utilization of the domain.
//
// In order to predict the performance state, map the utilization of
// the most utilized CPU of the performance domain to a requested
// performance, like schedutil. Take also into account that the real
// performance might be set lower (due to thermal capping). Thus, clamp
// max utilization to the allowed CPU capacity before calculating
// effective performance.
//
// Find the lowest performance state of the Energy Model above the
// requested performance.
//
// The performance (capacity) of a CPU in the domain at the performance
// state (ps) can be computed as:
//
// ps->freq * scale_cpu
// ps->performance = --------------------                  (1)
// cpu_max_freq
//
// So, ignoring the costs of idle states (which are not available in
// the EM), the energy consumed by this CPU at that performance state
// is estimated as:
//
// ps->power * cpu_util
// cpu_nrg = --------------------                          (2)
// ps->performance
//
// since 'cpu_util / ps->performance' represents its percentage of busy
// time.
//
// NOTE: Although the result of this computation actually is in
// units of power, it can be manipulated as an energy value
// over a scheduling period, since it is assumed to be
// constant during that interval.
//
// By injecting (1) in (2), 'cpu_nrg' can be re-expressed as a product
// of two terms:
//
// ps->power * cpu_max_freq
// cpu_nrg = ------------------------ * cpu_util           (3)
// ps->freq * scale_cpu
//
// The first term is static, and is stored in the em_perf_state struct
// as 'ps->cost'.
//
// Since all CPUs of the domain have the same micro-architecture, they
// share the same 'ps->cost', and the same CPU capacity. Hence, the
// total energy of the domain (which is the simple sum of the energy of
// all of its CPUs) can be factorized as:
//
// pd_nrg = ps->cost * \Sum cpu_util                       (4)
//
// em_pd_nr_perf_states() - Get the number of performance states of a perf.
// domain
// @pd		: performance domain for which this must be done
//
// Return: the number of performance states in the performance domain table
//
// em_perf_state_from_pd() - Get the performance states table of perf.
// domain
// @pd		: performance domain for which this must be done
//
// To use this function the rcu_read_lock() should be hold. After the usage
// of the performance states table is finished, the rcu_read_unlock() should
// be called.
//
// Return: the pointer to performance states table of the performance domain
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_data_callback {

    pub -EINVAL: return,
    pub -EINVAL: return,
    pub NULL: return,
    pub NULL: return,
    pub 0: return,
    pub 0: return,
    pub NULL: return,
    pub -EINVAL: return,
    pub NULL: return,
    pub -EINVAL: return,
    pub -EINVAL: return,
    pub -EINVAL: return,

