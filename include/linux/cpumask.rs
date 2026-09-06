//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpumask.h
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
// Cpumasks provide a bitmap suitable for representing the
// set of CPUs in a system, one bit position per CPU number.  In general,
// only nr_cpu_ids (<= NR_CPUS) bits are valid.
//

//
// cpumask_pr_args - printf args to output a cpumask
// @maskp: cpumask to be printed
//
// Can be used to provide arguments for '%*pb[l]' when printing a cpumask.
//

//
// We have several different "preferred sizes" for the cpumask
// operations, depending on operation.
//
// For example, the bitmap scanning and operating operations have
// optimized routines that work for the single-word case, but only when
// the size is constant. So if NR_CPUS fits in one single word, we are
// better off using that small constant, in order to trigger the
// optimized bit finding. That is 'small_cpumask_size'.
//
// The clearing and copying operations will similarly perform better
// with a constant size, but we limit that size arbitrarily to four
// words. We call this 'large_cpumask_size'.
//
// Finally, some operations just want the exact limit, either because
// they set bits or just don't have any faster fixed-sized versions. We
// call this just 'nr_cpumask_bits'.
//
// Note that these optional constants are always guaranteed to be at
// least as big as 'nr_cpu_ids' itself is, and all our cpumask
// allocations are at least that size (see cpumask_size()). The
// optimization comes from being able to potentially use a compile-time
// constant instead of a run-time generated exact number of CPUs.
//

//
// The following particular system cpumasks and operations manage
// possible, present, active and online cpus.
//
// cpu_possible_mask- has bit 'cpu' set iff cpu is populatable
// cpu_present_mask - has bit 'cpu' set iff cpu is populated
// cpu_enabled_mask - has bit 'cpu' set iff cpu can be brought online
// cpu_online_mask  - has bit 'cpu' set iff cpu available to scheduler
// cpu_active_mask  - has bit 'cpu' set iff cpu available to migration
//
// If !CONFIG_HOTPLUG_CPU, present == possible, and active == online.
//
// The cpu_possible_mask is fixed at boot time, as the set of CPU IDs
// that it is possible might ever be plugged in at anytime during the
// life of that system boot.  The cpu_present_mask is dynamic(*),
// representing which CPUs are currently plugged in.  And
// cpu_online_mask is the dynamic subset of cpu_present_mask,
// indicating those CPUs available for scheduling.
//
// If HOTPLUG is enabled, then cpu_present_mask varies dynamically,
// depending on what ACPI reports as currently plugged in, otherwise
// cpu_present_mask is just a copy of cpu_possible_mask.
//
// (*) Well, cpu_present_mask is dynamic in the hotplug case.  If not
// hotplug, it's a copy of cpu_possible_mask, hence fixed at boot.
//
// Subtleties:
// 1) UP ARCHes (NR_CPUS == 1, CONFIG_SMP not defined) hardcode
// assumption that their single CPU is online.  The UP
// cpu_{online,possible,present}_masks are placebos.  Changing them
// will have no useful affect on the following num_*_cpus()
// and cpu_*() macros in the UP case.  This ugliness is a UP
// optimization - don't waste any instructions or memory references
// asking if you're online or how many CPUs there are if there is
// only one CPU.
//

// verify cpu argument to cpumask_* operators
//
// cpumask_first - get the first cpu in a cpumask
// @srcp: the cpumask pointer
//
// Return: >= nr_cpu_ids if no cpus set.
//
extern "C" {
    pub fn find_first_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_first_zero - get the first unset cpu in a cpumask
// @srcp: the cpumask pointer
//
// Return: >= nr_cpu_ids if all cpus are set.
//
extern "C" {
    pub fn find_first_zero_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_first_and - return the first cpu from *srcp1 & *srcp2
// @srcp1: the first input
// @srcp2: the second input
//
// Return: >= nr_cpu_ids if no cpus set in both.  See also cpumask_next_and().
//
extern "C" {
    pub fn find_first_and_bit(_arg: cpumask_bits(srcp1), _arg: cpumask_bits(srcp2), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_first_andnot - return the first cpu from *srcp1 & ~*srcp2
// @srcp1: the first input
// @srcp2: the second input
//
// Return: >= nr_cpu_ids if no such cpu found.
//
extern "C" {
    pub fn find_first_andnot_bit(_arg: cpumask_bits(srcp1), _arg: cpumask_bits(srcp2), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_first_and_and - return the first cpu from *srcp1 & *srcp2 & *srcp3
// @srcp1: the first input
// @srcp2: the second input
// @srcp3: the third input
//
// Return: >= nr_cpu_ids if no cpus set in all.
//
// cpumask_last - get the last CPU in a cpumask
// @srcp:	- the cpumask pointer
//
// Return:	>= nr_cpumask_bits if no CPUs set.
//
extern "C" {
    pub fn find_last_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_next - get the next cpu in a cpumask
// @n: the cpu prior to the place to search (i.e. return will be > @n)
// @srcp: the cpumask pointer
//
// Return: >= nr_cpu_ids if no further cpus set.
//
// -1 is a legal arg here.
extern "C" {
    pub fn find_next_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits, 1: n +) -> return;
}
//
// cpumask_next_zero - get the next unset cpu in a cpumask
// @n: the cpu prior to the place to search (i.e. return will be > @n)
// @srcp: the cpumask pointer
//
// Return: >= nr_cpu_ids if no further cpus unset.
//
// -1 is a legal arg here.
extern "C" {
    pub fn find_next_zero_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits, _arg: n+1) -> return;
}

// Uniprocessor: there is only one valid CPU
extern "C" {
    pub fn cpumask_first_and(_arg: src1p, _arg: src2p) -> return;
}
extern "C" {
    pub fn cpumask_first(_arg: srcp) -> return;
}

extern "C" {
    pub fn cpumask_local_spread(i: c_uint, node: c_int) -> c_uint;
}
extern "C" {
    pub fn cpumask_any_distribute(srcp: *const cpumask) -> c_uint;
}

//
// cpumask_next_and - get the next cpu in *src1p & *src2p
// @n: the cpu prior to the place to search (i.e. return will be > @n)
// @src1p: the first cpumask pointer
// @src2p: the second cpumask pointer
//
// Return: >= nr_cpu_ids if no further cpus set in both.
//
// -1 is a legal arg here.
//
// cpumask_next_andnot - get the next cpu in *src1p & ~*src2p
// @n: the cpu prior to the place to search (i.e. return will be > @n)
// @src1p: the first cpumask pointer
// @src2p: the second cpumask pointer
//
// Return: >= nr_cpu_ids if no further cpus set in both.
//
// -1 is a legal arg here.
//
// cpumask_next_and_wrap - get the next cpu in *src1p & *src2p, starting from
// @n+1. If nothing found, wrap around and start from
// the beginning
// @n: the cpu prior to the place to search (i.e. search starts from @n+1)
// @src1p: the first cpumask pointer
// @src2p: the second cpumask pointer
//
// Return: next set bit, wrapped if needed, or >= nr_cpu_ids if @src1p & @src2p is empty.
//
// -1 is a legal arg here.
//
// cpumask_next_wrap - get the next cpu in *src, starting from @n+1. If nothing
// found, wrap around and start from the beginning
// @n: the cpu prior to the place to search (i.e. search starts from @n+1)
// @src: cpumask pointer
//
// Return: next set bit, wrapped if needed, or >= nr_cpu_ids if @src is empty.
//
// -1 is a legal arg here.
extern "C" {
    pub fn find_next_bit_wrap(_arg: cpumask_bits(src), _arg: small_cpumask_bits, 1: n +) -> return;
}
//
// cpumask_random - get random cpu in *src.
// @src: cpumask pointer
//
// Return: random set bit, or >= nr_cpu_ids if @src is empty.
//
extern "C" {
    pub fn find_random_bit(_arg: cpumask_bits(src), _arg: nr_cpu_ids) -> return;
}
//
// for_each_cpu - iterate over every cpu in a mask
// @cpu: the (optionally unsigned) integer iterator
// @mask: the cpumask pointer
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// for_each_cpu_wrap - iterate over every cpu in a mask, starting at a specified location
// @cpu: the (optionally unsigned) integer iterator
// @mask: the cpumask pointer
// @start: the start location
//
// The implementation does not assume any bit in @mask is set (including @start).
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// for_each_cpu_and - iterate over every cpu in both masks
// @cpu: the (optionally unsigned) integer iterator
// @mask1: the first cpumask pointer
// @mask2: the second cpumask pointer
//
// This saves a temporary CPU mask in many places.  It is equivalent to:
// struct cpumask tmp;
// cpumask_and(&tmp, &mask1, &mask2);
// for_each_cpu(cpu, &tmp)
// ...
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// for_each_cpu_andnot - iterate over every cpu present in one mask, excluding
// those present in another.
// @cpu: the (optionally unsigned) integer iterator
// @mask1: the first cpumask pointer
// @mask2: the second cpumask pointer
//
// This saves a temporary CPU mask in many places.  It is equivalent to:
// struct cpumask tmp;
// cpumask_andnot(&tmp, &mask1, &mask2);
// for_each_cpu(cpu, &tmp)
// ...
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// for_each_cpu_or - iterate over every cpu present in either mask
// @cpu: the (optionally unsigned) integer iterator
// @mask1: the first cpumask pointer
// @mask2: the second cpumask pointer
//
// This saves a temporary CPU mask in many places.  It is equivalent to:
// struct cpumask tmp;
// cpumask_or(&tmp, &mask1, &mask2);
// for_each_cpu(cpu, &tmp)
// ...
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// for_each_cpu_from - iterate over CPUs present in @mask, from @cpu to the end of @mask.
// @cpu: the (optionally unsigned) integer iterator
// @mask: the cpumask pointer
//
// After the loop, cpu is >= nr_cpu_ids.
//

//
// cpumask_any_but - return an arbitrary cpu in a cpumask, but not this one.
// @mask: the cpumask to search
// @cpu: the cpu to ignore.
//
// Often used to find any cpu but smp_processor_id() in a mask.
// If @cpu == -1, the function is equivalent to cpumask_any().
// Return: >= nr_cpu_ids if no cpus set.
//
// -1 is a legal arg here.
//
// cpumask_any_and_but - pick an arbitrary cpu from *mask1 & *mask2, but not this one.
// @mask1: the first input cpumask
// @mask2: the second input cpumask
// @cpu: the cpu to ignore
//
// If @cpu == -1, the function is equivalent to cpumask_any_and().
// Returns >= nr_cpu_ids if no cpus set.
//
// -1 is a legal arg here.
extern "C" {
    pub fn cpumask_next_and(_arg: cpu, _arg: mask1, _arg: mask2) -> return;
}
//
// cpumask_any_andnot_but - pick an arbitrary cpu from *mask1 & ~*mask2, but not this one.
// @mask1: the first input cpumask
// @mask2: the second input cpumask
// @cpu: the cpu to ignore
//
// If @cpu == -1, the function returns the first matching cpu.
// Returns >= nr_cpu_ids if no cpus set.
//
// -1 is a legal arg here.
extern "C" {
    pub fn cpumask_next_andnot(_arg: cpu, _arg: mask1, _arg: mask2) -> return;
}
//
// cpumask_nth - get the Nth cpu in a cpumask
// @srcp: the cpumask pointer
// @cpu: the Nth cpu to find, starting from 0
//
// Return: >= nr_cpu_ids if such cpu doesn't exist.
//
extern "C" {
    pub fn find_nth_bit(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits, _arg: cpumask_check(cpu)) -> return;
}
//
// cpumask_nth_and - get the Nth cpu in 2 cpumasks
// @srcp1: the cpumask pointer
// @srcp2: the cpumask pointer
// @cpu: the Nth cpu to find, starting from 0
//
// Return: >= nr_cpu_ids if such cpu doesn't exist.
//
// cpumask_nth_and_andnot - get the Nth cpu set in 1st and 2nd cpumask, and clear in 3rd.
// @srcp1: the cpumask pointer
// @srcp2: the cpumask pointer
// @srcp3: the cpumask pointer
// @cpu: the Nth cpu to find, starting from 0
//
// Return: >= nr_cpu_ids if such cpu doesn't exist.
//

//
// cpumask_set_cpu - set a cpu in a cpumask
// @cpu: cpu number (< nr_cpu_ids)
// @dstp: the cpumask pointer
//
// cpumask_clear_cpus - clear cpus in a cpumask
// @dstp:  the cpumask pointer
// @cpu:   cpu number (< nr_cpu_ids)
// @ncpus: number of cpus to clear (< nr_cpu_ids)
//
// cpumask_clear_cpu - clear a cpu in a cpumask
// @cpu: cpu number (< nr_cpu_ids)
// @dstp: the cpumask pointer
//
// cpumask_test_cpu - test for a cpu in a cpumask
// @cpu: cpu number (< nr_cpu_ids)
// @cpumask: the cpumask pointer
//
// Return: true if @cpu is set in @cpumask, else returns false
//
extern "C" {
    pub fn test_bit(_arg: cpumask_check(cpu), _arg: cpumask_bits((cpumask))) -> return;
}
//
// cpumask_test_and_set_cpu - atomically test and set a cpu in a cpumask
// @cpu: cpu number (< nr_cpu_ids)
// @cpumask: the cpumask pointer
//
// test_and_set_bit wrapper for cpumasks.
//
// Return: true if @cpu is set in old bitmap of @cpumask, else returns false
//
extern "C" {
    pub fn test_and_set_bit(_arg: cpumask_check(cpu), _arg: cpumask_bits(cpumask)) -> return;
}
//
// cpumask_test_and_clear_cpu - atomically test and clear a cpu in a cpumask
// @cpu: cpu number (< nr_cpu_ids)
// @cpumask: the cpumask pointer
//
// test_and_clear_bit wrapper for cpumasks.
//
// Return: true if @cpu is set in old bitmap of @cpumask, else returns false
//
extern "C" {
    pub fn test_and_clear_bit(_arg: cpumask_check(cpu), _arg: cpumask_bits(cpumask)) -> return;
}
//
// cpumask_setall - set all cpus (< nr_cpu_ids) in a cpumask
// @dstp: the cpumask pointer
//
// cpumask_clear - clear all cpus (< nr_cpu_ids) in a cpumask
// @dstp: the cpumask pointer
//
// cpumask_and - *dstp = *src1p & *src2p
// @dstp: the cpumask result
// @src1p: the first input
// @src2p: the second input
//
// Return: false if *@dstp is empty, else returns true
//
// cpumask_or - *dstp = *src1p | *src2p
// @dstp: the cpumask result
// @src1p: the first input
// @src2p: the second input
//
// cpumask_weighted_or - *dstp = *src1p | *src2p and return the weight of the result
// @dstp: the cpumask result
// @src1p: the first input
// @src2p: the second input
//
// Return: The number of bits set in the resulting cpumask @dstp
//
// cpumask_xor - *dstp = *src1p ^ *src2p
// @dstp: the cpumask result
// @src1p: the first input
// @src2p: the second input
//
// cpumask_andnot - *dstp = *src1p & ~*src2p
// @dstp: the cpumask result
// @src1p: the first input
// @src2p: the second input
//
// Return: false if *@dstp is empty, else returns true
//
// cpumask_equal - *src1p == *src2p
// @src1p: the first input
// @src2p: the second input
//
// Return: true if the cpumasks are equal, false if not
//
// cpumask_or_equal - *src1p | *src2p == *src3p
// @src1p: the first input
// @src2p: the second input
// @src3p: the third input
//
// Return: true if first cpumask ORed with second cpumask == third cpumask,
// otherwise false
//
// cpumask_intersects - (*src1p & *src2p) != 0
// @src1p: the first input
// @src2p: the second input
//
// Return: true if first cpumask ANDed with second cpumask is non-empty,
// otherwise false
//
// cpumask_subset - (*src1p & ~*src2p) == 0
// @src1p: the first input
// @src2p: the second input
//
// Return: true if *@src1p is a subset of *@src2p, else returns false
//
// cpumask_empty - *srcp == 0
// @srcp: the cpumask to that all cpus < nr_cpu_ids are clear.
//
// Return: true if srcp is empty (has no bits set), else false
//
extern "C" {
    pub fn bitmap_empty(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_full - *srcp == 0xFFFFFFFF...
// @srcp: the cpumask to that all cpus < nr_cpu_ids are set.
//
// Return: true if srcp is full (has all bits set), else false
//
extern "C" {
    pub fn bitmap_full(_arg: cpumask_bits(srcp), _arg: nr_cpumask_bits) -> return;
}
//
// cpumask_weight - Count of bits in *srcp
// @srcp: the cpumask to count bits (< nr_cpu_ids) in.
//
// Return: count of bits set in *srcp
//
extern "C" {
    pub fn bitmap_weight(_arg: cpumask_bits(srcp), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_weight_and - Count of bits in (*srcp1 & *srcp2)
// @srcp1: the cpumask to count bits (< nr_cpu_ids) in.
// @srcp2: the cpumask to count bits (< nr_cpu_ids) in.
//
// Return: count of bits set in both *srcp1 and *srcp2
//
extern "C" {
    pub fn bitmap_weight_and(_arg: cpumask_bits(srcp1), _arg: cpumask_bits(srcp2), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_weight_andnot - Count of bits in (*srcp1 & ~*srcp2)
// @srcp1: the cpumask to count bits (< nr_cpu_ids) in.
// @srcp2: the cpumask to count bits (< nr_cpu_ids) in.
//
// Return: count of bits set in both *srcp1 and *srcp2
//
extern "C" {
    pub fn bitmap_weight_andnot(_arg: cpumask_bits(srcp1), _arg: cpumask_bits(srcp2), _arg: small_cpumask_bits) -> return;
}
//
// cpumask_shift_right - *dstp = *srcp >> n
// @dstp: the cpumask result
// @srcp: the input to shift
// @n: the number of bits to shift by
//
// cpumask_shift_left - *dstp = *srcp << n
// @dstp: the cpumask result
// @srcp: the input to shift
// @n: the number of bits to shift by
//
// cpumask_copy - *dstp = *srcp
// @dstp: the result
// @srcp: the input cpumask
//
// cpumask_any - pick an arbitrary cpu from *srcp
// @srcp: the input cpumask
//
// Return: >= nr_cpu_ids if no cpus set.
//

//
// cpumask_any_and - pick an arbitrary cpu from *mask1 & *mask2
// @mask1: the first input cpumask
// @mask2: the second input cpumask
//
// Return: >= nr_cpu_ids if no cpus set.
//

//
// cpumask_of - the cpumask containing just a given cpu
// @cpu: the cpu (<= nr_cpu_ids)
//

//
// cpumask_parse_user - extract a cpumask from a user string
// @buf: the buffer to extract from
// @len: the length of the buffer
// @dstp: the cpumask to set.
//
// Return: -errno, or 0 for success.
//
extern "C" {
    pub fn bitmap_parse_user(_arg: buf, _arg: len, _arg: cpumask_bits(dstp), _arg: nr_cpumask_bits) -> return;
}
//
// cpumask_parselist_user - extract a cpumask from a user string
// @buf: the buffer to extract from
// @len: the length of the buffer
// @dstp: the cpumask to set.
//
// Return: -errno, or 0 for success.
//
// cpumask_parse - extract a cpumask from a string
// @buf: the buffer to extract from
// @dstp: the cpumask to set.
//
// Return: -errno, or 0 for success.
//
extern "C" {
    pub fn bitmap_parse(_arg: buf, _arg: UINT_MAX, _arg: cpumask_bits(dstp), _arg: nr_cpumask_bits) -> return;
}
//
// cpulist_parse - extract a cpumask from a user string of ranges
// @buf: the buffer to extract from
// @dstp: the cpumask to set.
//
// Return: -errno, or 0 for success.
//
extern "C" {
    pub fn bitmap_parselist(_arg: buf, _arg: cpumask_bits(dstp), _arg: nr_cpumask_bits) -> return;
}
//
// cpumask_size - calculate size to allocate for a 'struct cpumask' in bytes
//
// Return: size to allocate for a &struct cpumask in bytes
//
extern "C" {
    pub fn bitmap_size(_arg: large_cpumask_bits) -> return;
}

extern "C" {
    pub fn alloc_cpumask_var_node(mask: *mut cpumask_var_t, flags: gfp_t, node: c_int) -> bool;
}
extern "C" {
    pub fn alloc_cpumask_var_node(_arg: mask, __GFP_ZERO: flags |, _arg: node) -> return;
}
//
// alloc_cpumask_var - allocate a struct cpumask
// @mask: pointer to cpumask_var_t where the cpumask is returned
// @flags: GFP_ flags
//
// Only defined when CONFIG_CPUMASK_OFFSTACK=y, otherwise is
// a nop returning a constant 1 (in <linux/cpumask.h>).
//
// See alloc_cpumask_var_node.
//
// Return: %true if allocation succeeded, %false if not
//
extern "C" {
    pub fn alloc_cpumask_var_node(_arg: mask, _arg: flags, _arg: NUMA_NO_NODE) -> return;
}
extern "C" {
    pub fn alloc_cpumask_var(_arg: mask, __GFP_ZERO: flags |) -> return;
}
extern "C" {
    pub fn alloc_bootmem_cpumask_var(mask: *mut cpumask_var_t);
}
extern "C" {
    pub fn free_cpumask_var(mask: cpumask_var_t);
}
extern "C" {
    pub fn free_bootmem_cpumask_var(mask: cpumask_var_t);
}

// Macro flag: #define __cpumask_var_read_mostly

// It's common to want to use cpu_all_mask in struct member initializers,
// so it has to refer to an address rather than a pointer.
extern "C" {
    pub fn DECLARE_BITMAP(_arg: cpu_all_bits, _arg: NR_CPUS) -> const;
}

// First bits of cpu_bit_bitmap are in fact unset.

// Uniprocessor: the possible/online/present masks are always "1"

// Wrappers for arch boot code to manipulate normally-constant masks
extern "C" {
    pub fn init_cpu_present(src: *const cpumask);
}
extern "C" {
    pub fn init_cpu_possible(src: *const cpumask);
}

extern "C" {
    pub fn set_cpu_online(cpu: c_uint, online: bool);
}
extern "C" {
    pub fn set_cpu_possible(cpu: c_uint, possible: bool);
}
//
// to_cpumask - convert a NR_CPUS bitmap to a struct cpumask
// @bitmap: the bitmap
//
// There are a few places where cpumask_var_t isn't appropriate and
// static cpumasks must be used (eg. very early boot), yet we don't
// expose the definition of 'struct cpumask'.
//
// This does the conversion, and can be used as a constant initializer.
//

//
// Special-case data structure for "single bit set only" constant CPU masks.
//
// We pre-generate all the 64 (or 32) possible bit positions, with enough
// padding to the left and the right, and return the constant pointer
// appropriately offset.
//
extern "C" {
    pub fn to_cpumask(_arg: p) -> return;
}

//
// num_online_cpus() - Read the number of online CPUs
//
// Despite the fact that __num_online_cpus is of type atomic_t, this
// interface gives only a momentary snapshot and is not protected against
// concurrent CPU hotplug operations unless invoked from a cpuhp_lock held
// region.
//
// Return: momentary snapshot of the number of online CPUs
//
extern "C" {
    pub fn raw_atomic_read(_arg: &__num_online_cpus) -> return;
}

extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_online_mask) -> return;
}
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_enabled_mask) -> return;
}
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_possible_mask) -> return;
}
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_present_mask) -> return;
}
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_active_mask) -> return;
}
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_dying_mask) -> return;
}

//
// cpumap_print_bitmask_to_buf  - copies the cpumask into the buffer as
// hex values of cpumask
//
// @buf: the buffer to copy into
// @mask: the cpumask to copy
// @off: in the string from which we are copying, we copy to @buf
// @count: the maximum number of bytes to print
//
// The function prints the cpumask into the buffer as hex values of
// cpumask; Typically used by bin_attribute to export cpumask bitmask
// ABI.
//
// Return: the length of how many bytes have been copied, excluding
// terminating '\0'.
//
// cpumap_print_list_to_buf  - copies the cpumask into the buffer as
// comma-separated list of cpus
// @buf: the buffer to copy into
// @mask: the cpumask to copy
// @off: in the string from which we are copying, we copy to @buf
// @count: the maximum number of bytes to print
//
// Everything is same with the above cpumap_print_bitmask_to_buf()
// except the print format.
//
// Return: the length of how many bytes have been copied, excluding
// terminating '\0'.
//

//
// Provide a valid theoretical max size for cpumap and cpulist sysfs files
// to avoid breaking userspace which may allocate a buffer based on the size
// reported by e.g. fstat.
//
// for cpumap NR_CPUS * 9/32 - 1 should be an exact length.
//
// For cpulist 7 is (ceil(log10(NR_CPUS)) + 1) allowing for NR_CPUS to be up
// to 2 orders of magnitude larger than 8192. And then we divide by 2 to
// cover a worst-case of every other cpu being on one of two nodes for a
// very large NR_CPUS.
//
// Use PAGE_SIZE as a minimum for smaller configurations while avoiding
// unsigned comparison to -1.
//

