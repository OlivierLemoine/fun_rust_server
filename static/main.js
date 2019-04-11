(async () => {
    let res = await fetch('/test', {
        method: 'POST',
        body: 'hello\ntest',
    });
    let p = await res.text();
    console.log(p);
})();

// let c = 0;

// setInterval(() => {
//     console.log(c);
//     c = 0;
// }, 1000);

// setInterval(() => {
//     fetch('/').then(() => c++);
// }, 1);
