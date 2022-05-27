public class Main {

    public static void main(String[] args) {
        try (Point p = new Point(1, 2)) {
            p.print();
        }
    }

}
