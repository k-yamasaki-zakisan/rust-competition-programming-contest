import unittest
import main


class TestMain(unittest.TestCase):
    def test_tashizan(self):
        """test method for tashizan
        """
        value1 = 2
        value2 = 6
        expected = 8
        actual = main.tashizan(value1, value2)
        self.assertEqual(expected, actual)


if __name__ == "__main__":
    unittest.main()