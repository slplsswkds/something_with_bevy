# This file is needed to create the metallic_roughness texture from separated metallic and roughness

import cv2
import numpy as np

img1 = cv2.imread("metallic.png", cv2.IMREAD_GRAYSCALE)
img2 = cv2.imread("roughness.png", cv2.IMREAD_GRAYSCALE)

if img1.shape != img2.shape:
    raise ValueError("Both images must have the same dimensions")

red_channel = np.zeros_like(img1)

rgb_image = cv2.merge([red_channel, img2, img1])
cv2.imwrite("metallic_roughness.png", rgb_image)
