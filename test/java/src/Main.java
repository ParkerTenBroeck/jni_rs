import java.io.File;

public class Main{

    public static void main(String... args) {
        System.out.println("Java Started!");
        
        try{
            Runtime.getRuntime().load(new File("../../target/release/libtest.so").getCanonicalPath());
        }catch (Exception e){
            throw new RuntimeException(e);
        }
        
        testNew().printHellow();


        var orig = new int[]{1,2,3,4,5,6,7,8,9};
        for(var v : orig){
            System.out.println(v);
        }
        var outpuit = testMethodId(orig);
        for(var v : orig){
            System.out.println(v);
        }
        for(var v : outpuit){
            System.out.println(v);
        }

        // System.out.println(cret().getName());
        // throwMe(new Throwable("woah"));

        // System.out.println(bruh(new int[]{2,3,4,5,6,7,8,9}));
        // new Main().bruh2();
    }

    public native static Main testNew();

    public native static Main test33();

    public native static short[] testMethodId(int[] vals);

    public long printHellow(){
        System.out.println("hellow");
        return -3;
    }

    public native static void throwMe(Throwable throwMe);

    public native static Class<?> cret();

    public native static long bruh(int[] thig);
    public native void bruh2();

}