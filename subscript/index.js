import("./pkg").then(module => {
    module.main();
    let runner = function() {
    	module.tick();
    	window.requestAnimationFrame(runner);
    };
    window.requestAnimationFrame(runner);
});

