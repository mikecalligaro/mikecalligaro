#include <iostream>

struct Node
{
  int val;
  Node* next;
  Node* prev;

  Node(int val)
  {
    this->val = val;
    this->next = NULL;
    this->prev = NULL;
  }
};

class DoublyLinkedList
{
public:
  DoublyLinkedList();

  void RemoveNode(Node* node);

private:
  Node* nodes = NULL;
};

DoublyLinkedList::DoublyLinkedList()
{

}

void DoublyLinkedList::RemoveNode(Node* node)
{
  node->next->prev = node->prev;
  node->prev->next = node->next;

  delete node;
}


// Everything from here on down shouldn't be in the interview.
// It's just really quick scaffolding I threw together to make
// sure the code above does what I expect it to.

// Put these three lines in the public section of the class 
// definition for this code to work.
void push(Node* node);
void dump();
Node* get(int num);



void DoublyLinkedList::push(Node* node)
{
  if (this->nodes == NULL)
  {
    this->nodes = node;
  }
  else
  {
    node->prev = this->nodes;
    node->next = this->nodes->next;
    if (this->nodes->next != NULL)
    {
      this->nodes->next->prev = node;
    }
    this->nodes->next = node;
  }
}

void DoublyLinkedList::dump()
{
  if (this->nodes == NULL)
  {
    std::cout << "Empty List\n";
    return;
  }

  Node* cur = this->nodes;
  do
  {
    std::cout << cur->val << " ";
    cur = cur->next;
  } while (cur != NULL);
  std::cout << "\n";
}

Node* DoublyLinkedList::get(int num)
{
  Node* cur = this->nodes;

  for (int i = 0; i < num; i++)
  {
    cur = cur->next;
  }

  return cur;
}



int main()
{
  const int num = 5;

  DoublyLinkedList* dll = new DoublyLinkedList();

  for (int i = 0; i < num; i++)
  {
    Node* newNode = new Node(i);
    dll->push(newNode);
  }

  dll->dump();

  Node* rem = dll->get(num / 2);

  dll->RemoveNode(rem);

  dll->dump();

  rem = dll->get(num / 2);

  dll->RemoveNode(rem);

  dll->dump();
}

