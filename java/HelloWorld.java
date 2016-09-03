public class HelloWorld {
    public static void main(String[] args) {
        new Thread(() -> {
            System.out.println("Java thread1 l1 STDOUT\nJava thread1 l2 STDOUT");
            System.out.println("Java thread1 l3 STDOUT");
        }).start();

        new Thread(() -> {
            System.out.println("Java thread2 l1 STDOUT\nJava thread2 l2 STDOUT");
            System.out.println("Java thread2 l3 STDOUT");
        }).start();

        System.out.println("Java main l1 STDOUT\nJava main l2 STDOUT");
        System.err.println("Java main l1 STDERR\nJava main l2 STDERR");
    }
}
