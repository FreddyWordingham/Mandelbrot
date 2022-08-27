import mandelbrot

centre = mandelbrot.Complex(-0.4605111, 0.56011)
max_iter = 1000

n = mandelbrot.sample(centre, max_iter)
print(centre, " -> ", n)

k = 4
m = mandelbrot.multi_sample(centre, max_iter, k, 1e-15)
print(centre, " -> ", m / k**2)
