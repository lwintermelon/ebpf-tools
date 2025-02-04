use core::mem;

#[inline(always)]
pub unsafe fn map_lookup_elem(map: *mut cty::c_void, key: *const cty::c_void) -> *mut cty::c_void {
    let f: unsafe extern "C" fn(*mut cty::c_void, *const cty::c_void) -> *mut cty::c_void =
        mem::transmute(1usize);

    f(map, key)
}

#[inline(always)]
pub unsafe fn map_update_elem(
    map: *mut cty::c_void,
    key: *const cty::c_void,
    value: *const cty::c_void,
    flags: cty::uint64_t,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        *const cty::c_void,
        *const cty::c_void,
        cty::uint64_t,
    ) -> cty::c_long = mem::transmute(2usize);

    f(map, key, value, flags)
}

#[inline(always)]
pub unsafe fn map_delete_elem(map: *mut cty::c_void, key: *const cty::c_void) -> cty::c_long {
    let f: unsafe extern "C" fn(*mut cty::c_void, *const cty::c_void) -> cty::c_long =
        mem::transmute(3usize);

    f(map, key)
}

#[inline(always)]
pub unsafe fn get_current_pid_tgid() -> cty::uint64_t {
    let f: unsafe extern "C" fn() -> cty::uint64_t = mem::transmute(14usize);

    f()
}

#[inline(always)]
pub unsafe fn get_current_comm(buf: *mut cty::c_void, size_of_buf: cty::uint32_t) -> cty::c_long {
    let f: unsafe extern "C" fn(*mut cty::c_void, cty::uint32_t) -> cty::c_long =
        mem::transmute(16usize);

    f(buf, size_of_buf)
}

#[inline(always)]
pub unsafe fn get_stack(
    ctx: *mut cty::c_void,
    buf: *mut cty::c_void,
    size: cty::uint32_t,
    flags: cty::uint64_t,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        *mut cty::c_void,
        cty::uint32_t,
        cty::uint64_t,
    ) -> cty::c_long = mem::transmute(67usize);

    f(ctx, buf, size, flags)
}

#[inline(always)]
pub unsafe fn probe_read_kernel(
    dst: *mut cty::c_void,
    size: cty::uint32_t,
    unsafe_ptr: *const cty::c_void,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint32_t,
        *const cty::c_void,
    ) -> cty::c_long = mem::transmute(113usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn probe_read_user_str(
    dst: *mut cty::c_void,
    size: cty::uint32_t,
    unsafe_ptr: *const cty::c_void,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint32_t,
        *const cty::c_void,
    ) -> cty::c_long = mem::transmute(114usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn probe_read_kernel_str(
    dst: *mut cty::c_void,
    size: cty::uint32_t,
    unsafe_ptr: *const cty::c_void,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint32_t,
        *const cty::c_void,
    ) -> cty::c_long = mem::transmute(115usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn ringbuf_output(
    ringbuf: *mut cty::c_void,
    data: *mut cty::c_void,
    size: cty::uint64_t,
    flags: cty::uint64_t,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        *mut cty::c_void,
        cty::uint64_t,
        cty::uint64_t,
    ) -> cty::c_long = mem::transmute(130usize);

    f(ringbuf, data, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_reserve(
    ringbuf: *mut cty::c_void,
    size: cty::uint64_t,
    flags: cty::uint64_t,
) -> *mut cty::c_void {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint64_t,
        cty::uint64_t,
    ) -> *mut cty::c_void = mem::transmute(131usize);

    f(ringbuf, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_submit(data: *mut cty::c_void, flags: cty::uint64_t) {
    let f: unsafe extern "C" fn(*mut cty::c_void, cty::uint64_t) = mem::transmute(132usize);

    f(data, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_discard(data: *mut cty::c_void, flags: cty::uint64_t) {
    let f: unsafe extern "C" fn(*mut cty::c_void, cty::uint64_t) = mem::transmute(133usize);

    f(data, flags)
}
