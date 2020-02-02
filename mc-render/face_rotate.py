
class Cubic:

    def __init__(self):
        self.west = 'Face::West'
        self.down = 'Face::Down'
        self.north = 'Face::North'
        self.south = 'Face::South'
        self.up = 'Face::Up'
        self.east = 'Face::East'

    def rotate_y(self):
        tmp = self.south
        self.south = self.east
        self.east = self.north
        self.north = self.west
        self.west = tmp

    def rotate_x(self):
        tmp = self.south
        self.south = self.up
        self.up = self.north
        self.north = self.down
        self.down = tmp

    def rotate_xn(self, n):
        for i in range(0, n):
            self.rotate_x()

    def rotate_yn(self, n):
        for i in range(0, n):
            self.rotate_y()

    def display(self):
        return [
            self.west, self.down, self.north, self.south, self.up, self.east
        ]


if __name__ == '__main__':

    c = Cubic()
    c.rotate_yn(8)
    print(c.display())
    c.rotate_xn(4)
    print(c.display())
    print()

    ax = list()
    for rx in range(0, 4):
        ay = list()
        for ry in range(0, 4):
            c = Cubic()
            c.rotate_xn(rx)
            c.rotate_yn(ry)
            ay.append('[' + ','.join(c.display()) + ']')
        ax.append('[' + ','.join(ay) + ']')
    print('[' + ','.join(ax) + ']')
    print()
    ax = list()
    for rx in range(0, 4):
        ay = list()
        for ry in range(0, 4):
            c = Cubic()
            c.rotate_yn(4 - ry)
            c.rotate_xn(4 - rx)
            ay.append('[' + ','.join(c.display()) + ']')
        ax.append('[' + ','.join(ay) + ']')
    print('[' + ','.join(ax) + ']')
    print()
