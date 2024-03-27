import os

# Function to find all files with a specific extension recursively in a directory
def find_files_with_extension(directory, extension):
    found_files = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(extension):
                found_files.append(os.path.join(root, file))
    return found_files

# Function to append file name and content to a target file with encoding specified
def append_file_content_to_target(target_file, file_path, encoding='utf-8'):
    with open(target_file, 'a', encoding=encoding) as op_file:
        op_file.write(f"// src: {file_path}\n")
        with open(file_path, 'r', encoding=encoding) as source_file:
            op_file.write(source_file.read())

# Define the target output file
output_file = "op.txt"

# Check if the output file exists, and if it does, delete it
if os.path.exists(output_file):
    os.remove(output_file)

# Get the current working directory as the base folder for the search
folder_to_search = os.getcwd()
extension_to_search = ".rs"
found_files = find_files_with_extension(folder_to_search, extension_to_search)

# Iterate through the found files and append their content to the output file
for file_path in found_files:
    append_file_content_to_target(output_file, file_path)

print(f"All .rs files found in '{folder_to_search}' have been appended to '{output_file}'.")
