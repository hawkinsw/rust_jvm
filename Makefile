all:
	cargo build

test: all
	./target/debug/jvm -c ./java_test/ Hello main

test-debug: all
	./target/debug/jvm -d -c ./java_test/ Hello main

test-debug-debug: all
	gdb --args ./target/debug/jvm -d -c ./java_test/ Hello main
