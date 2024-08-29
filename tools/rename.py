import os
import glob
import argparse
import shutil

# Define the argument parser
parser = argparse.ArgumentParser(description='Rename files by adding a padded index to their names.')
parser.add_argument('input_folder', type=str, help='Path to the input folder containing files to rename.')
parser.add_argument('output_folder', type=str, help='Path to the output folder where renamed files will be saved.')

# Parse the arguments
args = parser.parse_args()

# Get the input and output folder paths
input_folder = args.input_folder
output_folder = args.output_folder

# Ensure the output folder exists
os.makedirs(output_folder, exist_ok=True)

# Get a list of all files in the input folder
file_list = glob.glob(os.path.join(input_folder, '*'))

# Sort files if necessary
file_list.sort()

# Loop through files and rename them
for index, file_path in enumerate(file_list):
    # Get the base name and extension of the file
    base_name = os.path.basename(file_path)
    name, ext = os.path.splitext(base_name)

    # Create a new name with padded index
    new_name = f"image_{str(index + 1).zfill(4)}{ext}"
    #new_name = "{}.png".format(name[0:-4])

    # Construct the full path for the new file name in the output folder
    new_file_path = os.path.join(output_folder, new_name)

    # Copy the file to the new location with the new name
    shutil.copy2(file_path, new_file_path)

    print(f'Renamed and moved: {file_path} -> {new_file_path}')
