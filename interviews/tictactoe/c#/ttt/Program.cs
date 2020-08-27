using System;

namespace ttt
{
    class TicTacToe
    {
        char[,] board = new char[3, 3];

        public TicTacToe()
        {

        }

        public bool IsWon(int x, int y, char player)
        {
            SetMove(x, y, player);
            return CheckIfWon();
        }

        private void SetMove(int x, int y, char player)
        {
            this.board[x, y] = player;
        }

        private bool CheckIfWon()
        {
            if (board[1, 1] == board[1, 2] && board[1, 2] == board[1, 3])
            {
                return true;
            }

            if (board[2, 1] == board[2, 2] && board[2, 2] == board[2, 3])
            {
                return true;
            }

            if (board[3, 1] == board[3, 2] && board[3, 2] == board[3, 3])
            {
                return true;
            }

            if (board[1, 1] == board[2, 1] && board[2, 1] == board[3, 1])
            {
                return true;
            }

            if (board[1, 2] == board[2, 2] && board[2, 2] == board[3, 2])
            {
                return true;
            }

            if (board[1, 3] == board[2, 3] && board[2, 3] == board[3, 3])
            {
                return true;
            }

            return false;
        }
    }

    class Program
    {
        static void Move(TicTacToe ttt, int x, int y, char player)
        {
            bool won = ttt.IsWon(x, y, player);
            Console.WriteLine("move: x=" + x + " y=" + y + " p=" + player + " won=" + won);
        }

        static void Main(string[] args)
        {
            TicTacToe ttt = new TicTacToe();

            Move(ttt, 1, 0, 'x');
            Move(ttt, 1, 1, 'x');
            Move(ttt, 1, 2, 'x');
        }
    }
}
