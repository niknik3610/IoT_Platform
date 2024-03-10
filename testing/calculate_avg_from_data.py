import sys
import statistics

def calculate_statistics(file_name):
    with open(file_name, 'r') as file:
        numbers = [int(line.strip()) for line in file]
    mean = statistics.mean(numbers)
    median = statistics.median(numbers)
    return mean, median

if __name__ == "__main__":
    file_name = sys.argv[1]
    mean, median = calculate_statistics(file_name)
    print(f"File Name: {file_name}")
    print(f"Mean: {mean}")
    print(f"Median: {median}")
