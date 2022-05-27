from cffi import FFI

ffibuilder = FFI()

ffibuilder.cdef("""
    typedef void* point;
    
    void say_hello();
    point point_new(int x, int y);
    void point_print(point p);
    void point_free(point p);
""")

lib = ffibuilder.dlopen("rust/target/release/librust.dylib")
