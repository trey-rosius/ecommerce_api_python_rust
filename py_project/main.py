from pprint import pprint

import ecommerce_api


def main():
    result = ecommerce_api.sum_as_string(5, 80)


    pprint("result is {}".format(result))


if __name__ == "__main__":
    main()
