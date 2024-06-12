import os

def count_lines_in_rs_files(root_dir):
    total_lines = 0
    file_line_counts = {}

    def walk_directory(directory):
        nonlocal total_lines
        for root, dirs, files in os.walk(directory):
            # Skip the /target directory
            if 'target' in dirs:
                dirs.remove('target')
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    try:
                        with open(file_path, 'r', encoding='utf-8') as f:
                            line_count = sum(1 for _ in f)
                            file_line_counts[file_path] = line_count
                            total_lines += line_count
                    except UnicodeDecodeError:
                        print(f"Could not decode file: {file_path}")

    walk_directory(root_dir)
    return file_line_counts, total_lines

# Example usage:
file_counts, total = count_lines_in_rs_files('./')
complete_total = sum(file_counts.values())
print(f"Total lines: {total}")
print(f"Complete total lines: {complete_total}")
for file, count in file_counts.items():
    print(f"{file}: {count} lines")
