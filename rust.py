from cffi import FFI

ffibuilder = FFI()

ffibuilder.cdef("""
    void say_hello();
    void* point_new(int x, int y);
    void point_print(void* point);
    void point_free(void* point);
""")

lib = ffibuilder.dlopen("rust/target/release/librust.dylib")
