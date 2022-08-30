import mandelbrot
import os

# Create output directory if it doesn't exist.
if not os.path.isdir("output"):
    os.mkdir("output")

# Center of view, and the maximum number of iterations to perform at each sampling point.
centre = mandelbrot.Complex(-0.5555111, 0.56011)
max_iter = 500

# Determine the number of iterations required to "escape" the given point on the complex plain.
# "Escape": the number of iterations required to exceed a magnitude of 2.0
n = mandelbrot.sample(centre, max_iter)
print(centre, " -> ", n)


# Resolution of output image(s), sqrt of the number of super_samples per pixel, scale: width of the image.
res = [4000, 4000]
super_samples = 10
scale = 2e-1
mandelbrot.gpu_render_image(centre, scale, res, super_samples, max_iter, "output")
# mandelbrot.gpu_render_video(
#     centre, scale, 0.99, res, 1000, super_samples, max_iter, "output"
# )
