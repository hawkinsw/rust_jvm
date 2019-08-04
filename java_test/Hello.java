public class Hello {
	public static final int hello = 75;
	static {
	}
	public static int integer() throws java.lang.ArithmeticException {
		throw new java.lang.ArithmeticException();
	}
	public static void nothing() {
		return;
	}

	public static int something() {
		return 2;
	}

	public static int sum(int a, int b) {
		return a + b;
	}

	public static void main(String[] args) {
		sum(1, 4);
		Aloha a = new Aloha();
		sum(something(), a.say());
		nothing();
	}
}
