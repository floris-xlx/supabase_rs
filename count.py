import os

def count_lines_in_files(root):
    line_counts = {}
    try:
        for foldername, subfolders, filenames in os.walk(root):
            if 'node_modules' in foldername:
                continue
            elif '.git' in foldername:
                continue

            elif '.vscode' in foldername:
                continue

            elif '__pycache__' in foldername:
                continue
            elif 'venv' in foldername:
                continue
            elif '.next' in foldername:
                continue
                
            elif 'build' in foldername:
                continue

            elif 'target' in foldername:
                continue

            elif 'github' in foldername:
                continue

            for filename in filenames:
                if filename.endswith(('.jsx', '.js', '.css', '.rs')):
                    filepath = os.path.join(foldername, filename)
                    with open(filepath, 'r', encoding='utf-8') as file:
                        file_lines = sum(1 for line in file)
                        file_type = filename.split('.')[-1]
                        if file_type in line_counts:
                            line_counts[file_type] += file_lines
                        else:
                            line_counts[file_type] = file_lines
        for file_type, count in line_counts.items():
            print(f"{file_type}: {count} lines")

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == '__main__':
    root = "./"
    count_lines_in_files(root)
