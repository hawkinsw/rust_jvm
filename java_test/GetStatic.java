public class GetStatic {
	public static int hello = 1;

	public static void main(String[] args) {
		int a = -1;
		while (a<hello) {
			a = a + 1;
		}
	}
}
