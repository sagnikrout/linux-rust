//! Automatically rewritten from C to Rust
//! Source: kernel/irq/matrix.c
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
// Copyright (C) 2017 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpumap {
    pub available: c_uint,
    pub allocated: c_uint,
    pub managed: c_uint,
    pub managed_allocated: c_uint,
    pub initialized: bool,
    pub online: bool,
    pub managed_map: *mut c_ulong,
    pub alloc_map: [c_ulong; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_matrix {
    pub matrix_bits: c_uint,
    pub alloc_start: c_uint,
    pub alloc_end: c_uint,
    pub alloc_size: c_uint,
    pub global_available: c_uint,
    pub global_reserved: c_uint,
    pub systembits_inalloc: c_uint,
    pub total_allocated: c_uint,
    pub online_maps: c_uint,
    pub maps: *mut cpumap __percpu,
    pub system_map: *mut c_ulong,
    pub scratch_map: [c_ulong; ],
}

// Macro flag: #define CREATE_TRACE_POINTS

//
// irq_alloc_matrix - Allocate a irq_matrix structure and initialize it
// @matrix_bits:	Number of matrix bits
// @alloc_start:	From which bit the allocation search starts
// @alloc_end:		At which bit the allocation search ends, i.e first
// invalid bit
//
    __init struct irq_matrix *irq_alloc_matrix(unsigned int matrix_bits,
    unsigned int alloc_start,
    unsigned int alloc_end)
    {
    unsigned int cpu, matrix_size = BITS_TO_LONGS(matrix_bits);
    struct irq_matrix *m;
    m = kzalloc_flex(*m, scratch_map, matrix_size * 2);
    if (!m)
    return core::ptr::null_mut();
    m.system_map = &m.scratch_map[matrix_size];
    m.matrix_bits = matrix_bits;
    m.alloc_start = alloc_start;
    m.alloc_end = alloc_end;
    m.alloc_size = alloc_end - alloc_start;
    m.maps = __alloc_percpu(struct_size(m.maps, alloc_map, matrix_size * 2),
    __alignof__(*m.maps));
    if (!m.maps) {
    kfree(m);
    return core::ptr::null_mut();
    }
    for_each_possible_cpu(cpu) {
    struct cpumap *cm = per_cpu_ptr(m.maps, cpu);
    cm.managed_map = &cm.alloc_map[matrix_size];
    }
    return m;
    }
//
// irq_matrix_online - Bring the local CPU matrix online
// @m:		Matrix pointer
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_online(m: *mut irq_matrix) {
    void irq_matrix_online(struct irq_matrix *m)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
    BUG_ON(cm.online);
    if (!cm.initialized) {
    cm.available = m.alloc_size;
    cm.available -= cm.managed + m.systembits_inalloc;
    cm.initialized = true;
    }
    m.global_available += cm.available;
    cm.online = true;
    m.online_maps++;
    trace_irq_matrix_online(m);
    }
//
// irq_matrix_offline - Bring the local CPU matrix offline
// @m:		Matrix pointer
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_offline(m: *mut irq_matrix) {
    void irq_matrix_offline(struct irq_matrix *m)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
// Update the global available size
    m.global_available -= cm.available;
    cm.online = false;
    m.online_maps--;
    trace_irq_matrix_offline(m);
    }
    static unsigned int matrix_alloc_area(struct irq_matrix *m, struct cpumap *cm,
    unsigned int num, bool managed)
    {
    unsigned int area, start = m.alloc_start;
    let mut end: c_uint = m.alloc_end;
    bitmap_or(m.scratch_map, cm.managed_map, m.system_map, end);
    bitmap_or(m.scratch_map, m.scratch_map, cm.alloc_map, end);
    area = bitmap_find_next_zero_area(m.scratch_map, end, start, num, 0);
    if (area >= end)
    return area;
    if (managed)
    bitmap_set(cm.managed_map, area, num);
    else
    bitmap_set(cm.alloc_map, area, num);
    return area;
    }
// Find the best CPU which has the lowest vector allocation count
    static unsigned int matrix_find_best_cpu(struct irq_matrix *m,
    const struct cpumask *msk)
    {
    unsigned int cpu, best_cpu, maxavl = 0;
    struct cpumap *cm;
    best_cpu = UINT_MAX;
    for_each_cpu(cpu, msk) {
    cm = per_cpu_ptr(m.maps, cpu);
    if (!cm.online || cm.available <= maxavl)
    continue;
    best_cpu = cpu;
    maxavl = cm.available;
    }
    return best_cpu;
    }
// Find the best CPU which has the lowest number of managed IRQs allocated
    static unsigned int matrix_find_best_cpu_managed(struct irq_matrix *m,
    const struct cpumask *msk)
    {
    unsigned int cpu, best_cpu, allocated = UINT_MAX;
    struct cpumap *cm;
    best_cpu = UINT_MAX;
    for_each_cpu(cpu, msk) {
    cm = per_cpu_ptr(m.maps, cpu);
    if (!cm.online || cm.managed_allocated > allocated)
    continue;
    best_cpu = cpu;
    allocated = cm.managed_allocated;
    }
    return best_cpu;
    }
//
// irq_matrix_assign_system - Assign system wide entry in the matrix
// @m:		Matrix pointer
// @bit:	Which bit to reserve
// @replace:	Replace an already allocated vector with a system
// vector at the same bit position.
//
// The BUG_ON()s below are on purpose. If this goes wrong in the
// early boot process, then the chance to survive is about zero.
// If this happens when the system is life, it's not much better.
//
    void irq_matrix_assign_system(struct irq_matrix *m, unsigned int bit,
    bool replace)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
    BUG_ON(bit > m.matrix_bits);
    BUG_ON(m.online_maps > 1 || (m.online_maps && !replace));
    set_bit(bit, m.system_map);
    if (replace) {
    BUG_ON(!test_and_clear_bit(bit, cm.alloc_map));
    cm.allocated--;
    m.total_allocated--;
    }
    if (bit >= m.alloc_start && bit < m.alloc_end)
    m.systembits_inalloc++;
    trace_irq_matrix_assign_system(bit, m);
    }
//
// irq_matrix_reserve_managed - Reserve a managed interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	On which CPUs the bits should be reserved.
//
// Can be called for offline CPUs. Note, this will only reserve one bit
// on all CPUs in @msk, but it's not guaranteed that the bits are at the
// same offset on all CPUs
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserve_managed(m: *mut irq_matrix, msk: *const cpumask) -> c_int {
    int irq_matrix_reserve_managed(struct irq_matrix *m, const struct cpumask *msk)
    {
    unsigned int cpu, failed_cpu;
    for_each_cpu(cpu, msk) {
    struct cpumap *cm = per_cpu_ptr(m.maps, cpu);
    unsigned int bit;
    bit = matrix_alloc_area(m, cm, 1, true);
    if (bit >= m.alloc_end)
    goto cleanup;
    cm.managed++;
    if (cm.online) {
    cm.available--;
    m.global_available--;
    }
    trace_irq_matrix_reserve_managed(bit, cpu, m, cm);
    }
    return 0;
    cleanup:
    failed_cpu = cpu;
    for_each_cpu(cpu, msk) {
    if (cpu == failed_cpu)
    break;
    irq_matrix_remove_managed(m, cpumask_of(cpu));
    }
    return -ENOSPC;
    }
//
// irq_matrix_remove_managed - Remove managed interrupts in a CPU map
// @m:		Matrix pointer
// @msk:	On which CPUs the bits should be removed
//
// Can be called for offline CPUs
//
// This removes not allocated managed interrupts from the map. It does
// not matter which one because the managed interrupts free their
// allocation when they shut down. If not, the accounting is screwed,
// but all what can be done at this point is warn about it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_remove_managed(m: *mut irq_matrix, msk: *const cpumask) {
    void irq_matrix_remove_managed(struct irq_matrix *m, const struct cpumask *msk)
    {
    unsigned int cpu;
    for_each_cpu(cpu, msk) {
    struct cpumap *cm = per_cpu_ptr(m.maps, cpu);
    unsigned int bit, end = m.alloc_end;
    if (WARN_ON_ONCE(!cm.managed))
    continue;
// Get managed bit which are not allocated
    bitmap_andnot(m.scratch_map, cm.managed_map, cm.alloc_map, end);
    bit = find_first_bit(m.scratch_map, end);
    if (WARN_ON_ONCE(bit >= end))
    continue;
    clear_bit(bit, cm.managed_map);
    cm.managed--;
    if (cm.online) {
    cm.available++;
    m.global_available++;
    }
    trace_irq_matrix_remove_managed(bit, cpu, m, cm);
    }
    }
//
// irq_matrix_alloc_managed - Allocate a managed interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	Which CPUs to search in
// @mapped_cpu:	Pointer to store the CPU for which the irq was allocated
//
    int irq_matrix_alloc_managed(struct irq_matrix *m, const struct cpumask *msk,
    unsigned int *mapped_cpu)
    {
    unsigned int bit, cpu, end;
    struct cpumap *cm;
    if (cpumask_empty(msk))
    return -EINVAL;
    cpu = matrix_find_best_cpu_managed(m, msk);
    if (cpu == UINT_MAX)
    return -ENOSPC;
    cm = per_cpu_ptr(m.maps, cpu);
    end = m.alloc_end;
// Get managed bit which are not allocated
    bitmap_andnot(m.scratch_map, cm.managed_map, cm.alloc_map, end);
    bit = find_first_bit(m.scratch_map, end);
    if (bit >= end)
    return -ENOSPC;
    set_bit(bit, cm.alloc_map);
    cm.allocated++;
    cm.managed_allocated++;
    m.total_allocated++;
// mapped_cpu = cpu;
    trace_irq_matrix_alloc_managed(bit, cpu, m, cm);
    return bit;
    }
//
// irq_matrix_assign - Assign a preallocated interrupt in the local CPU map
// @m:		Matrix pointer
// @bit:	Which bit to mark
//
// This should only be used to mark preallocated vectors
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_assign(m: *mut irq_matrix, bit: c_uint) {
    void irq_matrix_assign(struct irq_matrix *m, unsigned int bit)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
    if (WARN_ON_ONCE(bit < m.alloc_start || bit >= m.alloc_end))
    return;
    if (WARN_ON_ONCE(test_and_set_bit(bit, cm.alloc_map)))
    return;
    cm.allocated++;
    m.total_allocated++;
    cm.available--;
    m.global_available--;
    trace_irq_matrix_assign(bit, smp_processor_id(), m, cm);
    }
//
// irq_matrix_reserve - Reserve interrupts
// @m:		Matrix pointer
//
// This is merely a book keeping call. It increments the number of globally
// reserved interrupt bits w/o actually allocating them. This allows to
// setup interrupt descriptors w/o assigning low level resources to it.
// The actual allocation happens when the interrupt gets activated.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserve(m: *mut irq_matrix) {
    void irq_matrix_reserve(struct irq_matrix *m)
    {
    if (m.global_reserved == m.global_available)
    pr_warn("Interrupt reservation exceeds available resources\n");
    m.global_reserved++;
    trace_irq_matrix_reserve(m);
    }
//
// irq_matrix_remove_reserved - Remove interrupt reservation
// @m:		Matrix pointer
//
// This is merely a book keeping call. It decrements the number of globally
// reserved interrupt bits. This is used to undo irq_matrix_reserve() when the
// interrupt was never in use and a real vector allocated, which undid the
// reservation.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_remove_reserved(m: *mut irq_matrix) {
    void irq_matrix_remove_reserved(struct irq_matrix *m)
    {
    m.global_reserved--;
    trace_irq_matrix_remove_reserved(m);
    }
//
// irq_matrix_alloc - Allocate a regular interrupt in a CPU map
// @m:		Matrix pointer
// @msk:	Which CPUs to search in
// @reserved:	Allocate previously reserved interrupts
// @mapped_cpu: Pointer to store the CPU for which the irq was allocated
//
    int irq_matrix_alloc(struct irq_matrix *m, const struct cpumask *msk,
    bool reserved, unsigned int *mapped_cpu)
    {
    unsigned int cpu, bit;
    struct cpumap *cm;
//
// Not required in theory, but matrix_find_best_cpu() uses
// for_each_cpu() which ignores the cpumask on UP .
//
    if (cpumask_empty(msk))
    return -EINVAL;
    cpu = matrix_find_best_cpu(m, msk);
    if (cpu == UINT_MAX)
    return -ENOSPC;
    cm = per_cpu_ptr(m.maps, cpu);
    bit = matrix_alloc_area(m, cm, 1, false);
    if (bit >= m.alloc_end)
    return -ENOSPC;
    cm.allocated++;
    cm.available--;
    m.total_allocated++;
    m.global_available--;
    if (reserved)
    m.global_reserved--;
// mapped_cpu = cpu;
    trace_irq_matrix_alloc(bit, cpu, m, cm);
    return bit;
    }
//
// irq_matrix_free - Free allocated interrupt in the matrix
// @m:		Matrix pointer
// @cpu:	Which CPU map needs be updated
// @bit:	The bit to remove
// @managed:	If true, the interrupt is managed and not accounted
// as available.
//
    void irq_matrix_free(struct irq_matrix *m, unsigned int cpu,
    unsigned int bit, bool managed)
    {
    struct cpumap *cm = per_cpu_ptr(m.maps, cpu);
    if (WARN_ON_ONCE(bit < m.alloc_start || bit >= m.alloc_end))
    return;
    if (WARN_ON_ONCE(!test_and_clear_bit(bit, cm.alloc_map)))
    return;
    cm.allocated--;
    if(managed)
    cm.managed_allocated--;
    if (cm.online)
    m.total_allocated--;
    if (!managed) {
    cm.available++;
    if (cm.online)
    m.global_available++;
    }
    trace_irq_matrix_free(bit, cpu, m, cm);
    }
//
// irq_matrix_available - Get the number of globally available irqs
// @m:		Pointer to the matrix to query
// @cpudown:	If true, the local CPU is about to go down, adjust
// the number of available irqs accordingly
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_available(m: *mut irq_matrix, cpudown: bool) -> c_uint {
    unsigned int irq_matrix_available(struct irq_matrix *m, bool cpudown)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
    if (!cpudown)
    return m.global_available;
    return m.global_available - cm.available;
    }
//
// irq_matrix_reserved - Get the number of globally reserved irqs
// @m:		Pointer to the matrix to query
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_reserved(m: *mut irq_matrix) -> c_uint {
    unsigned int irq_matrix_reserved(struct irq_matrix *m)
    {
    return m.global_reserved;
    }
//
// irq_matrix_allocated - Get the number of allocated non-managed irqs on the local CPU
// @m:		Pointer to the matrix to search
//
// This returns number of allocated non-managed interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_allocated(m: *mut irq_matrix) -> c_uint {
    unsigned int irq_matrix_allocated(struct irq_matrix *m)
    {
    struct cpumap *cm = this_cpu_ptr(m.maps);
    return cm.allocated - cm.managed_allocated;
    }

//
// irq_matrix_debug_show - Show detailed allocation information
// @sf:		Pointer to the seq_file to print to
// @m:		Pointer to the matrix allocator
// @ind:	Indentation for the print format
//
// Note, this is a lockless snapshot.
//
#[no_mangle]
pub unsafe extern "C" fn irq_matrix_debug_show(sf: *mut seq_file, m: *mut irq_matrix, ind: c_int) {
    void irq_matrix_debug_show(struct seq_file *sf, struct irq_matrix *m, int ind)
    {
    let mut nsys: c_uint = bitmap_weight(m.system_map, m.matrix_bits);
    int cpu;
    seq_printf(sf, "Online bitmaps:   %6u\n", m.online_maps);
    seq_printf(sf, "Global available: %6u\n", m.global_available);
    seq_printf(sf, "Global reserved:  %6u\n", m.global_reserved);
    seq_printf(sf, "Total allocated:  %6u\n", m.total_allocated);
    seq_printf(sf, "System: %u: %*pbl\n", nsys, m.matrix_bits,
    m.system_map);
    seq_printf(sf, "%*s| CPU | avl | man | mac | act | vectors\n", ind, " ");
    cpus_read_lock();
    for_each_online_cpu(cpu) {
    struct cpumap *cm = per_cpu_ptr(m.maps, cpu);
    seq_printf(sf, "%*s %4d  %4u  %4u  %4u %4u  %*pbl\n", ind, " ",
    cpu, cm.available, cm.managed,
    cm.managed_allocated, cm.allocated,
    m.matrix_bits, cm.alloc_map);
    }
    cpus_read_unlock();
    }
