import click
import numpy as np
from PIL import Image

import expand_rgb

# def random_permutation() -> list:
#     arr = [(1, 0), (-1, 0), (0, 1), (0, -1)]
#     random.shuffle(arr)
#     return arr


# def py_expand_rgb(image: np.ndarray, iterations: int) -> np.ndarray:
#     alpha = image[:, :, 3:]
#     tmp = image.copy()

#     height, width = image.shape[0], image.shape[1]
#     for _ in tqdm(range(iterations)):
#         result = tmp.copy()

#         for x in range(1, width - 1):
#             for y in range(1, height - 1):
#                 color = tmp[y, x]
#                 if color[3] == 0:
#                     order = random_permutation()
#                     for i, j in order:
#                         if tmp[y + j, x + i, 3] != 0:
#                             color = tmp[y + j, x + i]
#                             break
#                 result[y, x] = color
#         tmp = result

#     result[:, :, 3:] = alpha
#     return result


@click.command()
@click.option(
    "--image_path",
    type=click.Path(exists=True),
    required=True,
    help="Path to RGBA image file.",
)
def main(image_path: str) -> None:
    image = Image.open(image_path)
    image = np.array(image)
    alpha_mask = (image[:, :, 3:] < 255).repeat(4, axis=2)
    image[alpha_mask] = 0

    Image.fromarray(image).convert(("RGB")).save("input_rgb.png")

    image = expand_rgb.process(image, 8)
    # image = py_expand_rgb(image, 8)

    image = Image.fromarray(image)
    image.save("output_rgba.png")
    image.convert("RGB").save("output_rgb.png")


if __name__ == "__main__":
    main()
