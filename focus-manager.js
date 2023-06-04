setTimeout(function () {
    let canvas = document.getElementById("main");
    canvas?.focus()
}, 300);

setTimeout(function () {
    let canvas = document.getElementById("main");
    canvas?.focus()

    canvas.addEventListener('contextmenu', function(ev) {
        ev.preventDefault();
        return false;
    }, false);
}, 1000);

