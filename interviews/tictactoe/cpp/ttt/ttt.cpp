#include <iostream>

class TicTacToe
{
public:
    TicTacToe();

    bool IsWon(int x, int y, char player);

private:
    void SetMove(int x, int y, char player);
    bool CheckIfWon();

    char board[3][3];
};

TicTacToe::TicTacToe()
{
  
}

bool TicTacToe::IsWon(int x, int y, char player)
{
    SetMove(x, y, player);
    return CheckIfWon();
}

void TicTacToe::SetMove(int x, int y, char player)
{
    this->board[x][y] = player;
}

bool TicTacToe::CheckIfWon()
{
    if (board[1][1] == board[1][2] && board[1][2] == board[1][3])
    {
      return true;
    }

    if (board[2][1] == board[2][2] && board[2][2] == board[2][3])
    {
      return true;
    }

    if (board[3][1] == board[3][2] && board[3][2] == board[3][3])
    {
      return true;
    }

    if (board[1][1] == board[2][1] && board[2][1] == board[3][1])
    {
      return true;
    }

    if (board[1][2] == board[2][2] && board[2][2] == board[3][2])
    {
      return true;
    }

    if (board[1][3] == board[2][3] && board[2][3] == board[3][3])
    {
      return true;
    }

    return false;
}




static void Move(TicTacToe* ttt, int x, int y, char player)
{
    bool won = ttt->IsWon(x, y, player);
    std::cout << "move: x=" << x << " y=" << y << " p=" << player << " won=" << won << "\n";
}

int main()
{
    TicTacToe* ttt = new TicTacToe();

    Move(ttt, 1, 0, 'x');
    Move(ttt, 1, 1, 'x');
    Move(ttt, 1, 2, 'x');
}
