function RunCode() {
    Output("Hello World ");
    Output("Second Line");
} 

function Output(out) {
    var element = document.getElementById("output");
    element.innerHTML += out;
}
