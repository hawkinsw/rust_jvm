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
		int aa = a;
		if (a != 3) {
			aa = 5;
		}
		return aa + b;
	}
	public static int mult(int a, int b) {
		return a * b;
	}

	public static void main(String[] args) {
		int b = 3;
		if (b <= 4) {
			/*
			 * Call a method on Aloha (Aloha.say()) but it
			 * will actually call Hola.say on a because a
			 * is actually an Hola.
			 */
			Hola hola = new Hola();
			Aloha aloha = hola;
			b = mult(something(), aloha.say());
		} else {
			b = sum(1, 4);
		}
		nothing();
		//System.out.println("Hello, world");
	}
}
