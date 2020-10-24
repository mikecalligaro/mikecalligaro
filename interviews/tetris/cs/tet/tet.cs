using System;
using System.Drawing;

namespace tet
{
    enum Shape
    {
        I,
        J,
        L,
        O,
        Z,
        T,
        S
    }

    class TetrisPiece
    {
        const int kNumShapeSegments = 4;
        const int kInitialXPos = 4;
        const int kInitialYPos = 0;

        Shape shape;
        Point[] data;

        public TetrisPiece(Shape shape)
        {
            this.shape = shape;
            data = new Point[4];
            PopulateShape();
        }

        private void PopulateShape()
        {
            int x = kInitialXPos;
            int y = kInitialYPos;

            // Shapes
            // I *   J   *   L *     O * *   Z * *     T   *     S   * * 
            //   *       *     *       * *       * *     * * *     * *
            //   *     * *     * *
            //   *

            this.data[0] = new Point(x, y);

            switch (this.shape)
            {
                case Shape.I:
                    this.data[1] = new Point(x, ++y);
                    this.data[2] = new Point(x, ++y);
                    this.data[3] = new Point(x, ++y);
                    break;

                case Shape.J:
                    this.data[1] = new Point(x, ++y);
                    this.data[2] = new Point(x, ++y);
                    this.data[3] = new Point(--x, y);
                    break;

                case Shape.L:
                    this.data[1] = new Point(x, ++y);
                    this.data[2] = new Point(x, ++y);
                    this.data[3] = new Point(++x, y);
                    break;

                case Shape.O:
                    this.data[1] = new Point(++x, y);
                    this.data[2] = new Point(x, ++y);
                    this.data[3] = new Point(--x, y);
                    break;

                case Shape.Z:
                    this.data[1] = new Point(--x, y);
                    this.data[2] = new Point(++x, ++y);
                    this.data[3] = new Point(++x, y);
                    break;

                case Shape.T:
                    this.data[1] = new Point(--x, ++y);
                    this.data[2] = new Point(++x, y);
                    this.data[3] = new Point(++x, y);
                    break;

                case Shape.S:
                    this.data[1] = new Point(++x, y);
                    this.data[2] = new Point(--x, ++y);
                    this.data[3] = new Point(--x, y);
                    break;
            }

        }

        public void Output()
        {
            const int kWellWidth = 10;
            const int kWellHeight = 20;

            for (int y = 0; y < kWellHeight; y++)
            {
                for (int x = 0; x < kWellWidth; x++)
                {
                    bool found = false;
                    for (int i = 0; !found && i < kNumShapeSegments; i++)
                    {
                        if (this.data[i].X == x && this.data[i].Y == y)
                        {
                            Console.Write("@");
                            found = true;
                        }
                    }
                    if (!found)
                    {
                        Console.Write("*");
                    }                    
                }
                Console.Write("\n");
            }
        }

    }

    class tet
    {
        static void Main(string[] args)
        {
            TetrisPiece tetris = new TetrisPiece(Shape.T);
            tetris.Output();
        }
    }
}
