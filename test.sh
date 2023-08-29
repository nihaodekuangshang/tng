TMP_FILE=tmp.s
TMP=tmp

assert(){
	input=$1
	expected=$2
	./target/release/tng  "$input" > $TMP_FILE
	gcc -o $TMP $TMP_FILE
	./tmp
	actual="$?"
	if [ "$actual" -ne "$expected" ]; then
		echo "fatal: $input => $actual, but expected $expected"
		exit 1
	fi
}
for i in $(seq 0 255); do
	assert $i $i
done
echo "OK"
