import("./pkg").then(module => {
    var start = performance.now();
    module.main();
    var end = performance.now();
    console.log("startup: " + (end - start));
    let runner = function() {
        module.tick();
        window.requestAnimationFrame(runner);
    };
    window.requestAnimationFrame(runner);
});

