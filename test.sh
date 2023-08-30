TMP_FILE=tmp.s
TMP=tmp

assert(){
	input=$1
	expected=$2
	./target/release/tng  "$input" > $TMP_FILE
	gcc -o $TMP $TMP_FILE
	./tmp
	actual="$?"
	if [ "$actual" -eq "$expected" ]; then
		echo "$input => $expected"
	else
		echo "fatal: $input => $actual, but expected $expected"
		exit 1
	fi
}
assert "20 + 5 +i 6" 31
echo "OK"
