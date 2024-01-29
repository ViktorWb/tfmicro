//! Symbol definitions those missing from building tensorflow without a C++
//! (or C) standard library

#[cfg(feature = "interop_cstd")]
mod cstd {
    #[allow(clippy::empty_loop)]
    #[no_mangle]
    // __cxa_pure_virtual is a function, address of which compiler writes
    // in the virtual table when the function is pure virtual. It may be
    // called due to some unnatural pointer abuse or when trying to invoke
    // pure virtual function in the destructor of the abstract base
    // class. The call to this function should never happen in the normal
    // application run. If it happens it means there is a bug.
    pub extern "C" fn __cxa_pure_virtual() {
        loop {}
    }

    #[allow(clippy::empty_loop)]
    #[no_mangle]
    // A cleanup must return control to the unwinding code by tail calling
    // __cxa_end_cleanup. The routine performs whatever housekeeping is
    // required and resumes the exception propagation by calling
    // _Unwind_Resume. This routine does not return.
    pub extern "C" fn __cxa_end_cleanup() {
        loop {}
    }

    #[no_mangle]
    pub extern "C" fn __gxx_personality_v0() {}

    // Simple implementation of errno
    static ERRNO: cty::c_int = 0;
    #[no_mangle]
    pub extern "C" fn __errno() -> *const cty::c_int {
        &ERRNO
    }

    // Despite -fno-rtti, these symbols are still generated. Define them
    // here, in a way that would likely have horrific consequences at
    // runtime
    cpp! {{
        namespace __cxxabiv1 {
            class __class_type_info {
                virtual void dummy();
            };
            void __class_type_info::dummy() { }
        };
        namespace __cxxabiv1 {
            class __si_class_type_info {
                virtual void dummy();
            };
            void __si_class_type_info::dummy() { }
        };
    }}

    // A strcmp implementation, for flatbuffers to use
    cpp! {{
        #include <string.h>

        int strcmp(const char *l, const char *r)
        {
            for (; *l==*r && *l; l++, r++);
            return *(unsigned char *)l - *(unsigned char *)r;
        }
    }}
    cpp! {{
        #include <string.h>

        int strncmp(const char *l, const char *r, size_t n)
        {
            if (!n--) return 0;
            for (; *l && *r && n && *l == *r ; l++, r++, n--);
            return *l - *r;
        }
    }}

    // Underlying assert function for tensorflow to use
    #[no_mangle]
    pub extern "C" fn __assert_func(
        _expr: *const cty::c_char,
        _line: cty::c_int,
        _file: *const cty::c_char,
        _function: *const cty::c_char,
    ) {
        panic!("__assert_func ASSERTED"); // __noreturn__
    }

    // Don't deallocate memory - tensorflow micro should be stack-based
    cpp! {{
        void operator delete(void * p) {}
        void operator delete(void * p, unsigned long l) {}
    }}
}
