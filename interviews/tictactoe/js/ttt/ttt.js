function RunCode() {
    var ttt = new TicTacToe();

    Move(ttt, 0, 0, 'x');
    Move(ttt, 0, 1, 'x');
    Move(ttt, 0, 2, 'x');
} 

function Move(ttt, x, y, player) {
    var won;

    won = ttt.IsWon(x, y, player);
    Output(" Won " + won);

}

function Output(out) {
    var element = document.getElementById("output");
    element.innerHTML += out;
}

class TicTacToe {
    constructor() {
        this.board = new Array(3);
        for (var i = 0; i < this.board.length; i++) {
            this.board[i] = new Array(3);
        }
    }

    IsWon(x, y, player) {
        this.SetMove(x, y, player);
        return this.CheckIfWon();
    }

    SetMove(x, y, player) {
        this.board[x][y] = player;
    }

    CheckIfWon() {
        if (this.board[1][1] == this.board[1][2] && this.board[1][2] == this.board[1][3]) {
            return true;
        }

        if (this.board[2][1] == this.board[2][2] && this.board[2][2] == this.board[2][3]) {
            return true;
        }

        if (this.board[3][1] == this.board[3][2] && this.board[3][2] == this.board[3][3]) {
            return true;
        }

        if (this.board[1][1] == this.board[2][1] && this.board[2][1] == this.board[3][1]) {
            return true;
        }

        if (this.board[1][2] == this.board[2][2] && this.board[2][2] == this.board[3][2]) {
            return true;
        }

        if (this.board[1][3] == this.board[2][3] && this.board[2][3] == this.board[3][3]) {
            return true;
        }
    
        return false;
    }
}