import cv2
from atentry import entry
import argparse
import numpy as np


@entry
def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("-i", "--input", required=True,
                    help="Path to the input image")
    ap.add_argument("-o", "--output", required=True,
                    help="Path to the desired output file location")
    ap.add_argument("--harr-file", default="./datasets/haarcascade_frontalface_alt.xml",
                    help="Path to the desired output file location")
    args = ap.parse_args()

    # Load the input image
    image = cv2.imread(args.input)

    # Load our image classifier
    harr_face = cv2.CascadeClassifier(args.harr_file)

    # Find faces in the image
    img_gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    faces = harr_face.detectMultiScale(
        img_gray, scaleFactor=1.1, minNeighbors=5)
    print(f"Found {len(faces)} faces")

    if len(faces) == 0:
        print("No faces found")
        return 1

    # Get the bounding box of the largest face
    max_area = 0
    max_face = None
    for (x, y, w, h) in faces:
        if w * h > max_area:
            max_area = w * h
            max_face = (x, y, w, h)

    # Get the corners of the largest face
    (x, y, w, h) = max_face
    face_points = np.float32([[x, y], [x + w, y], [x, y + h], [x + w, y + h]])

    # Warp the image
    end_size = (600, 800)
    face_point_ends = np.float32([[end_size[1]/8, end_size[0]/6], [end_size[1], 0], [
                                 end_size[1]/4, end_size[0]], [(end_size[1]/4)*3, end_size[0]]])
    M = cv2.getPerspectiveTransform(face_points, face_point_ends)
    warped = cv2.warpPerspective(image, M, (end_size[1], end_size[0]))

    # Add a blue tint to the image
    blue = np.zeros(warped.shape, dtype=warped.dtype)
    blue[:] = (255, 0, 0)
    warped = cv2.addWeighted(warped, 0.5, blue, 0.5, 0)

    # Add text to the top center
    cv2.putText(warped, "NO BITCHES?", (100, 100),
                cv2.FONT_HERSHEY_SIMPLEX, 3, (255, 255, 255), 15, cv2.LINE_AA)

    # Write the image to the output file
    cv2.imwrite(args.output, warped)

    return 0
