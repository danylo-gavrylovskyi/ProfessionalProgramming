#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <path-to-program>"
    exit 1
fi

PROGRAM="$1"
USER_STATS="user_stats.txt"

function run_test_case_1 {
    echo "Running Test Case 1..."
    rm -f $USER_STATS

    $PROGRAM alice > output.txt
    EXPECTED_OUTPUT="Welcome, alice!"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 1.1 Passed" || echo "Test Case 1.1 Failed"
    grep -q "alice 1" $USER_STATS && echo "Test Case 1.2 Passed" || echo "Test Case 1.2 Failed"

    $PROGRAM alice > output.txt
    EXPECTED_OUTPUT="Hello again(x2), alice"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 1.3 Passed" || echo "Test Case 1.3 Failed"
    grep -q "alice 2" $USER_STATS && echo "Test Case 1.4 Passed" || echo "Test Case 1.4 Failed"
}

function run_test_case_2 {
    echo "Running Test Case 2..."

    $PROGRAM > output.txt 2>&1
    EXPECTED_OUTPUT="Error: Invalid number of arguments"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 2.1 Passed" || echo "Test Case 2.1 Failed"

    $PROGRAM alice delete extra > output.txt 2>&1
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 2.2 Passed" || echo "Test Case 2.2 Failed"

    $PROGRAM alice > output.txt
    grep -q "Hello again(x3), alice" output.txt && echo "Test Case 2.3 Passed" || echo "Test Case 2.3 Failed"
}

function run_test_case_3 {
    echo "Running Test Case 3..."
    echo "bob 5" > $USER_STATS

    $PROGRAM bob delete > output.txt
    EXPECTED_OUTPUT="Statistics reset for bob"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 3.1 Passed" || echo "Test Case 3.1 Failed"
    grep -q "bob" $USER_STATS || echo "Test Case 3.2 Passed"

    $PROGRAM bob > output.txt
    EXPECTED_OUTPUT="Welcome, bob!"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 3.3 Passed" || echo "Test Case 3.3 Failed"
    grep -q "bob 1" $USER_STATS && echo "Test Case 3.4 Passed" || echo "Test Case 3.4 Failed"
}

function run_test_case_4 {
    echo "Running Test Case 4..."
    echo -e "alice 2\nbob 3\ncharlie 1" > $USER_STATS

    $PROGRAM bread > output.txt
    EXPECTED_OUTPUT="All history cleared"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 4.1 Passed" || echo "Test Case 4.1 Failed"
    [ ! -s $USER_STATS ] && echo "Test Case 4.2 Passed" || echo "Test Case 4.2 Failed"

    $PROGRAM alice > output.txt
    EXPECTED_OUTPUT="Welcome, alice!"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 4.3 Passed" || echo "Test Case 4.3 Failed"
    grep -q "alice 1" $USER_STATS && echo "Test Case 4.4 Passed" || echo "Test Case 4.4 Failed"
}

function run_test_case_5 {
    echo "Running Test Case 5..."
    echo "dave 4" > $USER_STATS

    $PROGRAM dave invalid_command > output.txt 2>&1
    EXPECTED_OUTPUT="Error: Invalid command"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 5.1 Passed" || echo "Test Case 5.1 Failed"
    grep -q "dave 4" $USER_STATS && echo "Test Case 5.2 Passed" || echo "Test Case 5.2 Failed"

    $PROGRAM dave > output.txt
    EXPECTED_OUTPUT="Hello again(x5), dave"
    grep -q "$EXPECTED_OUTPUT" output.txt && echo "Test Case 5.3 Passed" || echo "Test Case 5.3 Failed"
    grep -q "dave 5" $USER_STATS && echo "Test Case 5.4 Passed" || echo "Test Case 5.4 Failed"
}

run_test_case_1
run_test_case_2
run_test_case_3
run_test_case_4
run_test_case_5

rm -f output.txt
