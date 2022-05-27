from rust import lib

if __name__ == '__main__':
    lib.say_hello()

    point = lib.point_new(1, 1)

    lib.point_print(point)
    lib.point_print(point)
    lib.point_print(point)

    lib.point_free(point)

    # Super Dangerous, dont do that!
    lib.point_print(point)


