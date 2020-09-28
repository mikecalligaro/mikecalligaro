using System;

namespace ll
{
    class Node
    {
        public int val;
        public Node prev;
        public Node next;

        public Node(int val)
        {
            this.val = val;
            this.prev = null;
            this.next = null;
        }
    }

    class DoublyLinkedList
    {
        Node nodes = null;

        public DoublyLinkedList()
        {
            
        }

        public void RemoveNode(Node node)
        {
            node.next.prev = node.prev;
            node.prev.next = node.next;
        }
//    }

        // Uncomment out the above bracket and end the class here.  
        // Everything below this shouldn't be in the interview.



        public void push(Node node)
        {
            if (this.nodes == null)
            {
                this.nodes = node;
            }
            else
            {
                node.prev = this.nodes;
                node.next = this.nodes.next;
                if (this.nodes.next != null)
                {
                    this.nodes.next.prev = node;
                }
                this.nodes.next = node;
            }
        }

        public void dump()
        {
            if (this.nodes == null)
            {
                Console.WriteLine("Empty List");
                return;
            }

            Node cur = this.nodes;
            do
            {
                Console.Write(cur.val + " ");
                cur = cur.next;
            } while (cur != null);
            Console.Write("\n");
        }

        public Node get(int num)
        {
            Node cur = this.nodes;

            for (int i = 0; i < num; i++)
            {
                cur = cur.next;
            }

            return cur;
        }
    }

    class ll
    {        
        static void Main(string[] args)
        {
            const int num = 5;

            DoublyLinkedList dll = new DoublyLinkedList();
            for (int i = 0; i < num; i++)
            {
                Node newNode = new Node(i);
                dll.push(newNode);
            }

            dll.dump();

            Node rem = dll.get(num / 2);

            dll.RemoveNode(rem);

            dll.dump();

            rem = dll.get(num / 2);

            dll.RemoveNode(rem);

            dll.dump();
        }
    }
}
