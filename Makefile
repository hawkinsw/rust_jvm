all:
	cargo build

test: all
	./target/debug/jvm -c ./java_test/ Hello main

test-debug: all
	./target/debug/jvm -d -c ./java_test/ Float main
	#./target/debug/jvm -d -c ./java_test/:/usr/lib/jvm/java-8-openjdk-amd64/jre/lib/rt.jar Hello main

test-debug-debug: all
	gdb --args ./target/debug/jvm -d -c ./java_test/ Hello main

clean:
	cargo clean
