const rust = import("./parser");

rust.then(
    parser => {
        const data = ["<!-- wp:foo /-->", "hello"];

        for (datum of data) {
            console.log(datum, JSON.parse(parser.root(datum)));
        }
    }
);
