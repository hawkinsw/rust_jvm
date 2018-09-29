
public class Hello {
	public final int hello = 75;
	public static int integer() throws java.lang.ArithmeticException {
		throw new java.lang.ArithmeticException();
	}
	public static void main(String[] args) {
		try {
			integer();
		} catch (java.lang.ArithmeticException e) {
			System.out.println("Not Hello, World.");
		}
		System.out.println("Hello, World.");
	}
}
