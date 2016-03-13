public class HelloWorld {
    public static void main(String[] args) {
        new Thread(() -> {
            System.out.println("OJava thread println on STDOUT");
        }).start();

        System.out.println("OJava println on STDOUT\nLine 2 on STDOUT");
        System.err.println("EJava println on STDERR\nLine 2 on STDERR");
    }
}
