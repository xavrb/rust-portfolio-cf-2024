# Open the file for writing
with open("input.txt", "w") as file:
    # Write 5000 lines to the file
    for i in range(1, 50001):
        file.write(f"This is line {i} of the input.\n")
