class ArrayPlay {
	public static void main(String args[]) {
		Hola holas[] = new Hola[4];
		Hola hola = new Hola();
		holas[0] = hola;

		hola.what_to_say = 73;

		int sum = holas[0].say();
		sum = hola.say();
	}
}
