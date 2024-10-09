#!/bin/bash
# create a folder for outputs_actual if not exists
mkdir -p tests/outputs_actual

# for all inputs file in tests/inputs folder, and generate the actual output file using cargo run < in0.txt > tests/outputs_actual/out0.txt replace the in in the file name with out
for input_file in tests/inputs/*.txt; do
    # Extract the number from the file name
    number=$(basename "$input_file" | sed 's/in//; s/.txt//')
    # Generate the actual output file name
    actual_output_file="tests/outputs_actual/out$number.txt"
    # Run the program with the input file and redirect the output to the actual output file
    cargo run < "$input_file" > "$actual_output_file"
done

# create a file for the result if not exists, recreate the file if exists
rm -f tests/result.txt
touch tests/result.txt

# for all output files in tests/outputs_actual folder, compare with the expected output file in tests/outputs_expected folder, count the number of differences and print the number of differences
for actual_output_file in tests/outputs_actual/*.txt; do
    # Extract the number from the file name
    number=$(basename "$actual_output_file" | sed 's/out//; s/.txt//')
    # Generate the expected output file name
    expected_output_file="tests/outputs/out$number.txt"

    # Count the number of differences
    diff_count=$(diff "$actual_output_file" "$expected_output_file" | wc -l)
    # append the number of differences to tests/result.txt
    echo "Number of differences for $actual_output_file: $diff_count" >> "tests/result.txt"
    # Compare the actual output file with the expected output file and append the result to tests/result.txt
    diff "$actual_output_file" "$expected_output_file" >> "tests/result.txt"
done
