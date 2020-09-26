public class GetStatic {
	public static int hello = 75;

	public static void main(String[] args) {
		int a = 0;
		if (hello > 100) {
			hello = 1;
		} else {
			hello = 2;
		}
	}
}
