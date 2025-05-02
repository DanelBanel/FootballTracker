from ultralytics import YOLO
import pathlib

# Get current path to file

folder_path = pathlib.Path(__file__).parent.resolve()

# Get path to .pt file from user input
pt_file = input("Enter the name of .pt file in 'assets/models/': ")
if not pt_file.endswith('.pt'):
    print("Please enter a valid .pt file name.")
    exit()
pt_file_path = pathlib.Path(f"{folder_path}/assets/models/{pt_file}")
print(f"Path to .pt file: {pt_file_path}")
if not pt_file_path.exists():
    print("File does not exist.")
    exit()

model = YOLO(pt_file_path)
model.export(format="onnx", opset=12)
print("Model exported to ONNX format successfully.")
