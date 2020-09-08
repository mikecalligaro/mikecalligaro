
class TicTacToe:
    def __init__(self):
        self.board = [[None for x in range(3)] for y in range(3)]

    def IsWon(self, x, y, player):
        self.SetMove(x, y, player)
        return self.CheckIfWon()

    def SetMove(self, x, y, player):
        self.board[x][y] = player

    def CheckIfWon(self):
        if (self.board[1][1] == self.board[1][2] and self.board[1][2] == self.board[1][3]):
            return True
        
        if (self.board[2][1] == self.board[2][2] and self.board[2][2] == self.board[2][3]):
            return True
        
        if (self.board[3][1] == self.board[3][2] and self.board[3][2] == self.board[3][3]):
            return True
        
        if (self.board[1][1] == self.board[2][1] and self.board[2][1] == self.board[3][1]):
            return True
        
        if (self.board[1][2] == self.board[2][2] and self.board[2][2] == self.board[3][2]):
            return True
        
        if (self.board[1][3] == self.board[2][3] and self.board[2][3] == self.board[3][3]):
            return True
        
        return False


ttt = TicTacToe()
print(ttt.IsWon(1, 1, "x"), ttt.board)
print(ttt.IsWon(1, 0, "x"), ttt.board)
print(ttt.IsWon(1, 2, "x"), ttt.board)
