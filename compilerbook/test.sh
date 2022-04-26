assert() {
    expected="$1"
    input="$2"

    cargo run "$input" > tmp.s
    cc tmp.s
    ./a.out
    actual="$?"

    if [ "$actual" = "$expected" ]; then
      echo "$input => $actual"
    else
      echo "$input => $expected expected, but got $actual"
      exit 1
    fi
}

assert 0 0
assert 42 42
assert 18 "10-2+10"
assert 21 "5+20-4"

echo OK