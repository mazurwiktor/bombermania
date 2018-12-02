import argparse
import json
import enum

from PIL import Image


class BlockType(enum.Enum):
    Void = 0
    Boundary = 1
    Grass = 2
    Concrete = 3


def get_args():
    ap = argparse.ArgumentParser(description="worldgen")
    ap.add_argument("source", nargs=1)
    ap.add_argument("destination", nargs=1)
    return ap.parse_args()


def rgb2block(rgb_tuple):
    if rgb_tuple == (255, 0, 0):
        return BlockType.Boundary
    elif rgb_tuple == (0, 255, 0):
        return BlockType.Grass
    elif rgb_tuple == (0, 0, 0):
        return BlockType.Concrete
    else:
        return BlockType.Void


def image2array(image_path):
    im = Image.open(image_path)
    print("File attrs - extension: {}, size: {}, mode: {}".format(im.format, im.size, im.mode))
    width, height = im.size

    return [[rgb2block(im.getpixel((x, y))).value for x in range(width)] for y in range(height)]


def main():
    args = get_args()
    array = image2array(args.source[0])

    with open(args.destination[0], "w") as f:
        json.dump(array, f)

    for line in array:
        for block in line:
            print(block, end="")
        print("")


if __name__ == "__main__":
    main()
