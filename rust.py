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

class Point(object):
    def __init__(self, x, y) -> None:
        self.instance = lib.point_new(x, y)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, exc_traceback):
        self.free()

    def print(self):
        if self.instance:
            lib.point_print(self.instance)
        else:
            raise RuntimeError("No instance or already destroyed")

    def free(self):
        """
        If used in a non context-manager way
        :return:
        """
        if self.instance:
            lib.point_free(self.instance)
            self.instance = None
        else:
            raise RuntimeError("No instance or already destroyed")
