use opencv::{
    core::{Scalar, Vector},
    dnn::DNN_BACKEND_CUDA, // I will utilize my GPU to perform faster inference. Your way may vary
    dnn::DNN_TARGET_CUDA,
    highgui,
    imgcodecs::imread,
    imgcodecs::imwrite,
    imgproc::rectangle,
    imgproc::LINE_4,
    prelude::*,
    videoio,
};

use od_opencv::{
    model_format::ModelFormat,
    // I'll use YOLOv8 by Ultralytics.
    // If you prefer traditional YOLO, then import it as:
    // model_classic::ModelYOLOClassic
    model_ultralytics::ModelUltralyticsV8,
};

fn main() {
    println!("Hello, part1!");

    /*     let classes_labels: Vec<&str> = vec!["person", "bicycle", "car", "motorbike", "aeroplane", "bus", "train", "truck", "boat", "traffic light", "fire hydrant", "stop sign", "parking meter", "bench", "bird", "cat", "dog", "horse", "sheep", "cow", "elephant", "bear", "zebra", "giraffe", "backpack", "umbrella", "handbag", "tie", "suitcase", "frisbee", "skis", "snowboard", "sports ball", "kite", "baseball bat", "baseball glove", "skateboard", "surfboard", "tennis racket", "bottle", "wine glass", "cup", "fork", "knife", "spoon", "bowl", "banana", "apple", "sandwich", "orange", "broccoli", "carrot", "hot dog", "pizza", "donut", "cake", "chair", "sofa", "pottedplant", "bed", "diningtable", "toilet", "tvmonitor", "laptop", "mouse", "remote", "keyboard", "cell phone", "microwave", "oven", "toaster", "sink", "refrigerator", "book", "clock", "vase", "scissors", "teddy bear", "hair drier", "toothbrush"];

    // Define format for OpenCV's DNN module
    let mf = ModelFormat::ONNX;

    // Define model's input size
    let net_width = 640;
    let net_height = 640;

    // Initialize model itself
    let mut model = ModelUltralyticsV8::new_from_file("pretrained/yolov8n.onnx", None, (net_width, net_height), mf, DNN_BACKEND_CUDA, DNN_TARGET_CUDA, vec![]).unwrap();

    // Read image into the OpenCV's Mat object
    // Define it as mutable since we are going to put bounding boxes onto it.
    let mut frame = imread("images/dog.jpg", 1).unwrap();

    // Feed forward image through the model
    let (bboxes, class_ids, confidences) = model.forward(&frame, 0.25, 0.4).unwrap();

    // Process results
    for (i, bbox) in bboxes.iter().enumerate() {
        // Place bounding boxes onto the image
        rectangle(&mut frame, *bbox, Scalar::from((0.0, 255.0, 0.0)), 2, LINE_4, 0).unwrap();
        // Debug output to stdin
        println!("Class: {}", classes_labels[class_ids[i]]);
        println!("\tBounding box: {:?}", bbox);
        println!("\tConfidences: {}", confidences[i]);
    }

    // Finally save the updated image to the file system
    imwrite("images/dog_yolov8_n.jpg", &frame, &Vector::new()).unwrap(); */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
