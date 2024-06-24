## Test Case 1:
Ensure that the program greets a user correctly for their first run.

Steps:

- Ensure the user_stats.txt file does not contain the user alice.
- Run the program with command `./advancedHelloWorld alice`.
- Verify that the output is: "Welcome, alice!"
- Check the contents of user_stats.txt to confirm it contains: alice 1
- Run the program again with command `./advancedHelloWorld alice`.
- Verify that the output is: "Hello again(x1), alice"
- Check the contents of user_stats.txt to confirm it contains: alice 2

## Test Case 2
Ensure that the program handles invalid argument counts properly.

Steps:

- Run the program with command `./advancedHelloWorld`.
- Verify that the output is: "Error: Invalid number of arguments"
- Run the program with command `./advancedHelloWorld alice delete extra`.
- Verify that the output is: "Error: Invalid number of arguments"
- Run the program with the correct number of arguments `./advancedHelloWorld alice`
- Verify that the output is appropriate for a known user (based on the current statistics).

## Test Case 3
Ensure that the program can reset statistics for a specific user.

Steps:

- Ensure user_stats.txt contains the entry: bob 5
- Run the program with command `./advancedHelloWorld bob delete`.
- Verify that the output is: Statistics reset for bob
- Check the contents of user_stats.txt to confirm that bob is no longer present.
- Run the program again with command `./advancedHelloWorld bob`.
- Verify that the output is: Welcome, bob!
- Check the contents of user_stats.txt to confirm it contains: bob 1

## Test Case 4
Ensure that the program can clear all user statistics when the secret word is used.

Steps:

- Ensure user_stats.txt contains multiple entries, e.g., alice 2, bob 3, charlie 1.
- Run the program with command `./advancedHelloWorld bread`.
- Verify that the output is: "All history cleared"
- Check the contents of user_stats.txt to confirm it is empty.
- Run the program with command `./advancedHelloWorld alice`.
- Verify that the output is: "Welcome, alice!"
- Check the contents of user_stats.txt to confirm it contains: alice 1

## Test Case 5
Ensure that the program handles invalid commands properly.

Steps:

- Ensure user_stats.txt contains the entry: dave 4
- Run the program with command `./advancedHelloWorld dave invalid_command`.
- Verify that the output is: "Error: Invalid command"
- Check the contents of user_stats.txt to confirm it still contains: dave 4
- Run the program again with command `./advancedHelloWorld dave`.
- Verify that the output is: "Hello again(x5), dave"
- Check the contents of user_stats.txt to confirm it contains: dave 5
