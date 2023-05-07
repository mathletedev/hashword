# hashword

> Command-line password generator by generating hashes

Winner of the [2022 Congressional App Challenge](https://www.congressionalappchallenge.us/2022-winners/#Washington)!

## Algorithm

1. Prompt for `seed` and `key`
2. Generate a hash of 46 integers between 0 and 255 with Argon2 using `seed` and `key`
3. Take the first 16 integers and convert them to a character string `res` using modulo indices
4. Using the next 16 integers:
   1. If the index is _even_, store the integer into the variable `pos`
   2. If the index is _odd_, replace index `pos` of `res` with an indexed LOWERCASE character
5. Repeat step 4 for UPPERCASE, NUMERIC, and SPECIAL characters\
   Halve the number of integers taken each iteration\
   It can be proved that this produces at least 1 of each type of character in the final result
6. Print `res` and copy it to the system clipboard

## Argon2 config

Variant: `Argon2id`\
Version: `13`\
Memory cost: `65536`\
Iterations: `4`\
Parallelism factor: `8`\
Hash length: `8`

## Credits

Argon2: https://github.com/P-H-C/phc-winner-argon2
