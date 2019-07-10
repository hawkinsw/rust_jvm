all:
	cargo build

test: all
	./target/debug/jvm java_test/Hello Hello

test-debug: all
	./target/debug/jvm -d java_test/Hello Hello
