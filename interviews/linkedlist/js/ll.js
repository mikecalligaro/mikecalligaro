
class Node {
    constructor(val) {
        this.val = val;
        this.prev = null;
        this.next = null;
    }
}

class DoublyLinkedList {
    constructor() {
        this.nodes = null;
    }

    RemoveNode(node) {
        node.next.prev = node.prev;
        node.prev.next = node.next;
    }
}

// Everything from here on down shouldn't be in the interview.
// It's just really quick scaffolding I threw together to make
// sure the code above does what I expect it to.
// To get this to work again, remove the closing bracket right
// above this comment.

    push(node) {
        if (this.nodes == null) {
            this.nodes = node;
        }
        else {
            node.prev = this.nodes;
            node.next = this.nodes.next;
            if (this.nodes.next != null) {
                this.nodes.next.prev = node;
            }
            this.nodes.next = node;
        }
    }

    dump() {
        if (this.nodes == null) {
            Console.WriteLine("Empty List");
            return;
        }

        var cur = this.nodes;
        do {
            Output(cur.val + " ");
            cur = cur.next;
        } while (cur != null);        
    }

    get(num) {
        var cur = this.nodes;

        for (var i = 0; i < num; i++) {
            cur = cur.next;
        }

        return cur;
    }
}


function RunCode() {
    const num = 6;

    var dll = new DoublyLinkedList();

    for (var i = 0; i < num; i++) {
        var newNode = new Node(i);
        dll.push(newNode);
    }

    dll.dump();

    var rem = dll.get(num / 2);

    dll.RemoveNode(rem);

    dll.dump();

    rem = dll.get(num / 2);

    dll.RemoveNode(rem);

    dll.dump();
} 

function Output(out) {
    var element = document.getElementById("output");
    element.innerHTML += out;
}

