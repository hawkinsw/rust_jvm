class CharArrayLoadStoreTest {
	public static void main(String[] args) {
		char character_array[] = new char[4];
		int result = 0;
		character_array[0] = 'a';
		character_array[1] = 'b';
		character_array[2] = 'c';
		character_array[3] = 'd';
		//character_array[4] = 'e';
		if (character_array[2] == 'c' && character_array[0] != 'a') {
		//if (character_array[2] == 'c' && character_array[4] != 'e') {
			result = 2;
		} else {
			result = 3;
		}
	}
}
