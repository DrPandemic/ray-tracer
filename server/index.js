const js = import("../pkg/ray.js");
js.then(js => {
    run();
    function run() {
        js.run() || setTimeout(() => run(), 0);
    }
});

