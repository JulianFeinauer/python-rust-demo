public class Point implements AutoCloseable {
    static {
        System.loadLibrary("rust");
    }

    private static native long java_point_create(int x, int y);
    private static native long java_point_print(long point);
    private static native long java_point_free(long point);

    public static void main(String[] args) {
        try (Point p = new Point(1, 2)) {
            p.print();
        }
    }

    private long ptr;

    public Point(int x, int y) {
        this.ptr = java_point_create(x, y);
    }

    public void print() {
        java_point_print(this.ptr);
    }

    @Override
    public void close() {
        java_point_free(this.ptr);
    }
}
