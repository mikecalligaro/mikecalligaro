class Node:
    def __init__(self, val):    
        self.val = val
        self.prev = None
        self.next = None

class DoublyLinkedList:
    def __init__(self):
        self.nodes = None

    def RemoveNode(self, node):
        node.next.prev = node.prev
        node.prev.next = node.next

# The interview code ends here. The rest of this is just scaffolding 
# I threw together quickly to make sure the above code does what
# I expect it to.

    def push(self, node):
        if (self.nodes == None):
            self.nodes = node        
        else:
            node.prev = self.nodes
            node.next = self.nodes.next
            if (self.nodes.next != None):
                self.nodes.next.prev = node            
            self.nodes.next = node

    def dump(self):
        if (self.nodes == None):
            print("Empty List")
            return

        cur = self.nodes
        while cur != None:  
            print(cur.val)
            cur = cur.next

    def get(self, num):
        cur = self.nodes

        for i in range(num):
            cur = cur.next        

        return cur



dll = DoublyLinkedList()
num = 6
half = 3

for i in range(num):
    newNode = Node(i)
    dll.push(newNode)

dll.dump()

rem = dll.get(half)

dll.RemoveNode(rem)

dll.dump()

rem = dll.get(half)

dll.RemoveNode(rem)

dll.dump()



# class TicTacToe:
#     def __init__(self):
#         self.board = [[None for x in range(3)] for y in range(3)]

#     def IsWon(self, x, y, player):
#         self.SetMove(x, y, player)
#         return self.CheckIfWon()

#     def SetMove(self, x, y, player):
#         self.board[x][y] = player

#     def CheckIfWon(self):
#         if (self.board[1][1] == self.board[1][2] and self.board[1][2] == self.board[1][3]):
#             return True
        
#         if (self.board[2][1] == self.board[2][2] and self.board[2][2] == self.board[2][3]):
#             return True
        
#         if (self.board[3][1] == self.board[3][2] and self.board[3][2] == self.board[3][3]):
#             return True
        
#         if (self.board[1][1] == self.board[2][1] and self.board[2][1] == self.board[3][1]):
#             return True
        
#         if (self.board[1][2] == self.board[2][2] and self.board[2][2] == self.board[3][2]):
#             return True
        
#         if (self.board[1][3] == self.board[2][3] and self.board[2][3] == self.board[3][3]):
#             return True
        
#         return False


# ttt = TicTacToe()
# print(ttt.IsWon(1, 1, "x"), ttt.board)
# print(ttt.IsWon(1, 0, "x"), ttt.board)
# print(ttt.IsWon(1, 2, "x"), ttt.board)
