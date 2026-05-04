public class App {
    public static void main(String[] args) throws Exception {
        System.out.println("Hello, World!");
        App j = new App();
        j.hello();
        j.doSomething(j.ptr, 4);
        System.out.println(j.ptr);
        j.close(j.ptr);
    }
    static {
        System.loadLibrary("Rust");
    }

    public native void hello();


    public App(){

        ptr = getStructPointer();

    }

    public native void close(long k);

    public native void doSomething(long j, int l);

    public long ptr;

    private native long getStructPointer();
}
