public class Exceptional {
    public static void exceptionalMethod() throws Exception {
        throw new Exception();
    }
    public static void unExceptionalMethod() {
    }

    public static void main(String args[]) {
        int result = 0;
        try {
            exceptionalMethod();
            //unExceptionalMethod();
            result = 1;
        } catch (Exception e) {
            //System.out.println("This is exceptional!");
            result = -1;
        }
    }
}