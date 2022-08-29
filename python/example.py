import mandelbrot
import os

if not os.path.isdir("output"):
    os.mkdir("output")

centre = mandelbrot.Complex(-0.4605111, 0.56011)
max_iter = 100

n = mandelbrot.sample(centre, max_iter)
print(centre, " -> ", n)

super_samples = 4
m = mandelbrot.multi_sample(centre, max_iter, super_samples, 1e-15)
print(centre, " -> ", m / super_samples**2)

scale = 1e-0
res = [1000, 1000]
# mandelbrot.gpu_render_image(centre, scale, res, super_samples, max_iter, ".")
mandelbrot.gpu_render_video(
    centre, scale, 0.95, res, 1000, super_samples, max_iter, "output"
)
