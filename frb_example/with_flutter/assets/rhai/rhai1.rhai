#!/home/to/me/bin/rhai-run

// This is a Rhai script

//! This script defines a function and calls it.

fn call_me() {
    return 3;
}

let result = call_me();

print(`call_me() should be 3: ${result}`);

let x = [1, 2, 3];

print("x[1] should be 2:");
print(x[1]);

x[1] = 5;

print(`x[1] should be 5: ${x[1]}`);

let answer = 42;
print(`The answer is: ${answer}`);
return answer;

